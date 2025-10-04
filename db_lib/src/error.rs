use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Pool error: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),

    #[error("Pool creation error: {0}")]
    Pool_creation(#[from] deadpool_postgres::CreatePoolError),

    #[error("Conversion error: {0}")]
    Conversion(String),

    #[error("Database query error: {0}")]
    Query(#[from] tokio_postgres::Error),
}