#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("NOT_ADDED_YET")]
    NotAddedYet,
    
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    
    #[error("UNAUTHORIZED")]
    Unauthorized
}
