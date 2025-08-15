#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("NOT_EXIST")]
    NotExist,
    
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    
    #[error("UNAUTHORIZED")]
    Unauthorized
}
