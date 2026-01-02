pub mod chess;
pub mod models;
pub mod db;
pub mod config;
pub mod error;
pub mod services;
pub mod middleware;
pub mod handlers;

use sqlx::PgPool;
use services::{AuthService, GameService};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub auth_service: AuthService,
    pub game_service: GameService,
}
