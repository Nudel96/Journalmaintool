use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use trading_journal_backend::{
    db::{create_pool, run_migrations},
    handlers,
    middleware::auth_middleware,
    AppState, Config,
};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trading_journal_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    config.validate().expect("Invalid configuration");

    // Create database connection pool
    let db = create_pool(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    run_migrations(&db)
        .await
        .expect("Failed to run migrations");

    // Create application state
    let state = AppState {
        db: db.clone(),
        config: config.clone(),
    };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/auth/register", post(handlers::register))
        .route("/auth/login", post(handlers::login));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/auth/me", get(handlers::me))
        .route("/subscriptions/checkout", post(handlers::create_checkout_session))
        .route("/trades", post(handlers::create_trade))
        .route("/trades", get(handlers::list_trades))
        .route("/trades/:id", get(handlers::get_trade))
        .route("/trades/:id", put(handlers::update_trade))
        .route("/trades/:id", delete(handlers::delete_trade))
        .route("/analytics/overview", get(handlers::get_overview))
        .route("/analytics/symbols", get(handlers::get_by_symbol))
        .route("/analytics/setups", get(handlers::get_by_setup))
        .route("/analytics/mistakes", get(handlers::get_mistakes))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // Combine all API routes under /api prefix
    let api_routes = Router::new()
        .merge(public_routes)
        .merge(protected_routes);

    // Build application router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/webhooks/stripe", post(handlers::handle_stripe_webhook))
        .nest("/api", api_routes)
        .layer(cors)
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    tracing::info!("🚀 Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
