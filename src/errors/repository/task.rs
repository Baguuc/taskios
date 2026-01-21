/// Error returned when creating a task fails
#[derive(Debug, thiserror::Error)]
pub enum TaskCreateError {
    /// Project not found
    #[error("project_not_found")]
    ProjectNotFound,
}

/// Error returned when updating user's task fails
#[derive(Debug, thiserror::Error)]
pub enum TaskUpdateError {
    /// The task does not exist
    #[error("task_not_found")]
    TaskNotFound,
}

/// Error returned when creating a task fails
#[derive(Debug, thiserror::Error)]
pub enum TaskDeleteError {
    /// Task not found
    #[error("task_not_found")]
    TaskNotFound,
}
