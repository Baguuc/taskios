#[derive(thiserror::Error, Debug)]
pub enum ProjectCreateError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("invalid_token")]
    InvalidToken
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectListError {
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

#[derive(thiserror::Error, Debug)]
pub enum ProjectDeleteError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("invalid_token")]
    InvalidToken,
    #[error("project_not_found")]
    ProjectNotFound
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectListTasksError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("invalid_token")]
    InvalidToken,
    #[error("project_not_found")]
    ProjectNotFound
}
