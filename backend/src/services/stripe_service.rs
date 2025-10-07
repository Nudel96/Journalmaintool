use crate::{
    error::{AppError, Result},
    models::{SubscriptionInterval, STRIPE_PRICE_IDS},
};
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCustomer, Customer, EventObject, EventType,
    Webhook,
};

pub struct StripeService {
    client: Client,
    webhook_secret: String,
}

impl StripeService {
    pub fn new(secret_key: String, webhook_secret: String) -> Self {
        let client = Client::new(secret_key);
        Self {
            client,
            webhook_secret,
        }
    }

    /// Create a Stripe customer
    pub async fn create_customer(&self, email: &str, name: &str) -> Result<Customer> {
        let mut params = CreateCustomer::new();
        params.email = Some(email);
        params.name = Some(name);

        Customer::create(&self.client, params)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Stripe error: {}", e)))
    }

    /// Create a checkout session
    pub async fn create_checkout_session(
        &self,
        customer_id: &str,
        interval: SubscriptionInterval,
        success_url: &str,
        cancel_url: &str,
    ) -> Result<CheckoutSession> {
        let price_id = STRIPE_PRICE_IDS.get_price_id(&interval);

        let mut params = CreateCheckoutSession::new();
        params.mode = Some(CheckoutSessionMode::Subscription);
        params.customer = Some(customer_id.to_string());
        params.success_url = Some(success_url);
        params.cancel_url = Some(cancel_url);

        // Add line item
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            price: Some(price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);

        CheckoutSession::create(&self.client, params)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Stripe error: {}", e)))
    }

    /// Verify and parse webhook event
    pub fn verify_webhook(&self, payload: &str, signature: &str) -> Result<stripe::Event> {
        Webhook::construct_event(payload, signature, &self.webhook_secret)
            .map_err(|e| AppError::InternalServerError(format!("Webhook verification failed: {}", e)))
    }

    /// Handle webhook event
    pub async fn handle_webhook_event(&self, event: stripe::Event) -> Result<WebhookAction> {
        match event.type_ {
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    return Ok(WebhookAction::SubscriptionCreated {
                        customer_id: session.customer.map(|c| c.id().to_string()),
                        subscription_id: session.subscription.map(|s| s.id().to_string()),
                    });
                }
            }
            EventType::CustomerSubscriptionUpdated => {
                if let EventObject::Subscription(subscription) = event.data.object {
                    return Ok(WebhookAction::SubscriptionUpdated {
                        customer_id: subscription.customer.id().to_string(),
                        status: subscription.status.to_string(),
                    });
                }
            }
            EventType::CustomerSubscriptionDeleted => {
                if let EventObject::Subscription(subscription) = event.data.object {
                    return Ok(WebhookAction::SubscriptionCanceled {
                        customer_id: subscription.customer.id().to_string(),
                    });
                }
            }
            _ => {
                tracing::info!("Unhandled webhook event type: {:?}", event.type_);
            }
        }

        Ok(WebhookAction::Ignored)
    }
}

/// Actions to take based on webhook events
#[derive(Debug)]
pub enum WebhookAction {
    SubscriptionCreated {
        customer_id: Option<String>,
        subscription_id: Option<String>,
    },
    SubscriptionUpdated {
        customer_id: String,
        status: String,
    },
    SubscriptionCanceled {
        customer_id: String,
    },
    Ignored,
}

