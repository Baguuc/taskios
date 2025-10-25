#[derive(thiserror::Error, Debug)]
pub enum ProjectCreateError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("invalid_token")]
    InvalidToken
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectUpdateError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("invalid_token")]
    InvalidToken,
    #[error("project_not_found")]
    ProjectNotFound
}
