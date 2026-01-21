/// Parameters required to list tasks of a project.
pub struct TasksListParams<'p> {
    /// ID of the project.
    pub project_id: &'p i32,
    /// User's token.
    pub token: &'p String,
}

/// Parameters required to create a task.
pub struct TaskCreateParams<'p> {
    /// Task to create.
    pub task: &'p crate::models::TaskWithoutId,
    /// User's token.
    pub token: &'p String,
}

/// Parameters required to delete a task.
pub struct TaskDeleteParams<'p> {
    /// ID of the task to delete.
    pub task_id: &'p i32,
    /// User's token.
    pub token: &'p String,
}

/// Parameters required to update a task.
pub struct TasksUpdateParams<'p> {
    /// ID of the task to update.
    pub task_id: &'p i32,
    /// New data of the task (received as a partial).
    pub new_data: &'p crate::models::PartialTask,
    /// User's token.
    pub token: &'p String,
}
