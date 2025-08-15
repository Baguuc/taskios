#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    
    #[error("UNAUTHORIZED")]
    Unauthorized
}
