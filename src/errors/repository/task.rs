/// error returned when creating a task fails
#[derive(Debug, thiserror::Error)]
pub enum TaskCreateError {
    /// project not found
    #[error("project_not_found")]
    ProjectNotFound,
}

/// error returned when updating user's task fails
#[derive(Debug, thiserror::Error)]
pub enum TaskUpdateError {
    /// the task does not exist
    #[error("task_not_found")]
    TaskNotFound,
}

/// error returned when creating a task fails
#[derive(Debug, thiserror::Error)]
pub enum TaskDeleteError {
    /// task not found
    #[error("task_not_found")]
    TaskNotFound,
}
