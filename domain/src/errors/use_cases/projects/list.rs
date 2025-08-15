#[derive(thiserror::Error, Debug)]
pub enum Error { 
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    
    #[error("UNAUTHORIZED")]
    Unauthorized
}
