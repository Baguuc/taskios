pub struct TasksListParams<'p> {
    pub project_id: &'p i32,
    pub token: &'p String
}

pub struct TaskCreateParams<'p> {
    pub task: &'p crate::models::TaskWithoutId,
    pub token: &'p String
}

pub struct TaskDeleteParams<'p> {
    pub task_id: &'p i32,
    pub token: &'p String
}

pub struct TasksUpdateParams<'p> {
    pub task_id: &'p i32,
    pub new_data: &'p crate::models::PartialTask,
    pub token: &'p String
}
