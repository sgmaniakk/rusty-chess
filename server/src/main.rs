use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rusty_chess_server::{
    config::Config,
    handlers,
    middleware::auth_middleware,
    services::{AuthService, GameService},
    AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusty_chess_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env();
    tracing::info!("Starting Rusty Chess Server");
    tracing::info!("Server address: {}", config.server_address());

    // Connect to database
    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    tracing::info!("Database connected successfully");

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("Migrations complete");

    // Create services
    let auth_service = AuthService::new(config.jwt_secret.clone(), config.jwt_expiry_days);
    let game_service = GameService::new(config.move_deadline_hours);

    // Create app state
    let state = AppState {
        db: pool,
        auth_service,
        game_service,
    };

    // Build router
    let protected_routes = Router::new()
        .route("/api/games", get(handlers::list_games))
        .route("/api/games", post(handlers::create_game))
        .route("/api/games/:id", get(handlers::get_game))
        .route("/api/games/:id/moves", post(handlers::submit_move))
        .route("/api/games/:id/moves", get(handlers::get_moves))
        .route("/api/games/:id/pgn", get(handlers::export_pgn))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let app = Router::new()
        // Public routes
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        // Merge protected routes
        .merge(protected_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Start server
    let listener = tokio::net::TcpListener::bind(&config.server_address()).await?;
    tracing::info!("Server listening on {}", config.server_address());

    axum::serve(listener, app).await?;

    Ok(())
}
