use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub jwt_expiry_days: i64,
    pub move_deadline_hours: i64,
}

impl Config {
    pub fn from_env() -> Self {
        // Load from environment variables
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://chess_user:chess_password@localhost:5432/rusty_chess".to_string());

        let server_host = std::env::var("SERVER_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let server_port = std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "dev-secret-key".to_string());

        let jwt_expiry_days = std::env::var("JWT_EXPIRY_DAYS")
            .unwrap_or_else(|_| "7".to_string())
            .parse()
            .unwrap_or(7);

        let move_deadline_hours = std::env::var("MOVE_DEADLINE_HOURS")
            .unwrap_or_else(|_| "72".to_string())
            .parse()
            .unwrap_or(72);

        Config {
            database_url,
            server_host,
            server_port,
            jwt_secret,
            jwt_expiry_days,
            move_deadline_hours,
        }
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
