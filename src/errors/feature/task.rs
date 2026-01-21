/// Represents one of the errors that can occur when listing tasks
/// belonging to a project.
#[derive(thiserror::Error, Debug)]
pub enum TasksListError {
    /// The user is not authorized to access the project.
    #[error("unauthorized")]
    Unauthorized,
    /// The user's token is invalid.
    #[error("invalid_token")]
    InvalidToken,
    /// The project does not exist.
    #[error("project_not_found")]
    ProjectNotFound,
}

/// Represents one of the errors that can occur when creating a task.
#[derive(thiserror::Error, Debug)]
pub enum TaskCreateError {
    /// The user is not authorized to access the project.
    #[error("unauthorized")]
    Unauthorized,
    /// The user's token is invalid.
    #[error("invalid_token")]
    InvalidToken,
    /// The project does not exist.
    #[error("project_not_found")]
    ProjectNotFound,
}

/// Represents one of the errors that can occur when deleting a task.
#[derive(thiserror::Error, Debug)]
pub enum TaskDeleteError {
    /// The user is not authorized to access the project.
    #[error("unauthorized")]
    Unauthorized,
    /// The user's token is invalid.
    #[error("invalid_token")]
    InvalidToken,
    /// The task does not exist.
    #[error("task_not_found")]
    TaskNotFound,
}

/// Represents one of the errors that can occur when updating a task.
#[derive(thiserror::Error, Debug)]
pub enum TaskUpdateError {
    /// The user is not authorized to access the project.
    #[error("unauthorized")]
    Unauthorized,
    /// The user's token is invalid.
    #[error("invalid_token")]
    InvalidToken,
    /// The task does not exist.
    #[error("task_not_found")]
    TaskNotFound,
}
