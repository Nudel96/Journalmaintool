use crate::{
    error::{AppError, Result},
    middleware::AuthUser,
    models::{CheckoutSessionResponse, CreateCheckoutRequest, SubscriptionInterval, SubscriptionStatus, SubscriptionTier},
    repositories::UserRepository,
    services::{StripeService, WebhookAction},
    AppState,
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    Json,
};

/// Create Stripe checkout session
pub async fn create_checkout_session(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateCheckoutRequest>,
) -> Result<Json<CheckoutSessionResponse>> {
    let user_repo = UserRepository::new(state.db.clone());

    // Get user
    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or(AppError::UserNotFound)?;

    // Parse interval
    let interval = SubscriptionInterval::from(payload.interval);

    // Create Stripe service
    let stripe_service = StripeService::new(
        state.config.stripe_secret_key.clone(),
        state.config.stripe_webhook_secret.clone(),
    );

    // Create or get Stripe customer
    let customer_id = if let Some(cid) = user.stripe_customer_id {
        cid
    } else {
        let customer = stripe_service
            .create_customer(&user.email, &user.name)
            .await?;
        
        // Update user with customer ID
        user_repo
            .update_stripe_customer(user_id, &customer.id.to_string())
            .await?;

        customer.id.to_string()
    };

    // Create checkout session
    let success_url = format!("{}/dashboard?session_id={{CHECKOUT_SESSION_ID}}", 
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string()));
    let cancel_url = format!("{}/pricing", 
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string()));

    let session = stripe_service
        .create_checkout_session(&customer_id, interval, &success_url, &cancel_url)
        .await?;

    Ok(Json(CheckoutSessionResponse {
        session_id: session.id.to_string(),
        url: session.url.unwrap_or_default(),
    }))
}

/// Handle Stripe webhook
pub async fn handle_stripe_webhook(
    State(state): State<AppState>,
    request: Request,
) -> Result<StatusCode> {
    // Get signature from header
    let signature = request
        .headers()
        .get("stripe-signature")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::ValidationError("Missing stripe-signature header".to_string()))?;

    // Get body
    let body = axum::body::to_bytes(request.into_body(), usize::MAX)
        .await
        .map_err(|_| AppError::InternalServerError("Failed to read request body".to_string()))?;

    let payload = String::from_utf8(body.to_vec())
        .map_err(|_| AppError::InternalServerError("Invalid UTF-8 in request body".to_string()))?;

    // Create Stripe service
    let stripe_service = StripeService::new(
        state.config.stripe_secret_key.clone(),
        state.config.stripe_webhook_secret.clone(),
    );

    // Verify webhook
    let event = stripe_service.verify_webhook(&payload, signature)?;

    // Handle event
    let action = stripe_service.handle_webhook_event(event).await?;

    let user_repo = UserRepository::new(state.db.clone());

    match action {
        WebhookAction::SubscriptionCreated { customer_id, .. } => {
            if let Some(cid) = customer_id {
                user_repo
                    .update_subscription(
                        &cid,
                        SubscriptionStatus::Active.as_str(),
                        SubscriptionTier::Paid.as_str(),
                        Some("month"), // Default to month, will be updated on next event
                    )
                    .await?;
                tracing::info!("Subscription created for customer: {}", cid);
            }
        }
        WebhookAction::SubscriptionUpdated { customer_id, status } => {
            user_repo
                .update_subscription(
                    &customer_id,
                    &status,
                    SubscriptionTier::Paid.as_str(),
                    None,
                )
                .await?;
            tracing::info!("Subscription updated for customer: {}", customer_id);
        }
        WebhookAction::SubscriptionCanceled { customer_id } => {
            user_repo
                .update_subscription(
                    &customer_id,
                    SubscriptionStatus::Canceled.as_str(),
                    SubscriptionTier::None.as_str(),
                    None,
                )
                .await?;
            tracing::info!("Subscription canceled for customer: {}", customer_id);
        }
        WebhookAction::Ignored => {
            tracing::debug!("Webhook event ignored");
        }
    }

    Ok(StatusCode::OK)
}

