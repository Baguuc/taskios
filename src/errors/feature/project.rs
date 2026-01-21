/// Represents errors that can occur when creating a project.
#[derive(thiserror::Error, Debug)]
pub enum ProjectCreateError {
    /// The user is not authorized to create projects.
    #[error("unauthorized")]
    Unauthorized,
    /// The provided token is invalid.
    #[error("invalid_token")]
    InvalidToken,
}

/// Represents errors that can occur when listing projects.
#[derive(thiserror::Error, Debug)]
pub enum ProjectListError {
    /// The user is not authorized to list projects.
    #[error("unauthorized")]
    Unauthorized,
    /// The provided token is invalid.
    #[error("invalid_token")]
    InvalidToken,
}

/// Represents errors that can occur when updating a project.
#[derive(thiserror::Error, Debug)]
pub enum ProjectUpdateError {
    /// The user is not authorized to update projects.
    #[error("unauthorized")]
    Unauthorized,
    /// The provided token is invalid.
    #[error("invalid_token")]
    InvalidToken,
    /// The project was not found.
    #[error("project_not_found")]
    ProjectNotFound,
}

/// Represents errors that can occur when deleting a project.
#[derive(thiserror::Error, Debug)]
pub enum ProjectDeleteError {
    /// The user is not authorized to delete projects.
    #[error("unauthorized")]
    Unauthorized,
    /// The provided token is invalid.
    #[error("invalid_token")]
    InvalidToken,
    /// The project was not found.
    #[error("project_not_found")]
    ProjectNotFound,
}