use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Ping error: {0}")]
    Ping(#[from] surge_ping::SurgeError),

    #[error("Notification error: {0}")]
    #[allow(dead_code)]
    Notification(String),

    #[error("{0}")]
    Custom(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn to_user_message(&self) -> String {
        match self {
            AppError::Database(_) => "Failed to access database".to_string(),
            AppError::Migration(_) => "Failed to run database migration".to_string(),
            AppError::Http(e) => format!("Network error: {}", e),
            AppError::Serde(_) => "Failed to process data format".to_string(),
            AppError::Io(e) => format!("File system error: {}", e),
            AppError::Ping(e) => format!("Network ping failed: {}", e),
            AppError::Notification(msg) => format!("Notification error: {}", msg),
            AppError::Custom(msg) => msg.clone(),
        }
    }
}
