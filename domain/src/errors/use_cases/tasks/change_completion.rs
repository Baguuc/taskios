#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("PROJECT_NOT_EXIST")]
    ProjectNotExist,
    
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    
    #[error("UNAUTHORIZED")]
    Unauthorized
}
