pub struct ProjectCreateParams<'p> {
    pub name: &'p String,
    pub token: &'p String
}

pub struct ProjectListParams<'p> {
    pub token: &'p String,
    // the page size is 5
    pub page_number: &'p u32
}

pub struct ProjectUpdateParams<'p> {
    pub id: &'p i32,
    pub new_name: &'p Option<String>,
    pub token: &'p String
}

pub struct ProjectDeleteParams<'p> {
    pub id: &'p i32,
    pub token: &'p String
}

pub struct ProjectListTasksParams<'p> {
    pub id: &'p i32,
    pub token: &'p String
}

pub struct ProjectCreateTaskParams<'p> {
    pub project_id: &'p i32,
    pub title: &'p String,
    pub description: &'p String,
    pub token: &'p String
}

pub struct ProjectDeleteTaskParams<'p> {
    pub task_id: &'p i32,
    pub token: &'p String
}

pub struct ProjectUpdateTaskParams<'p> {
    pub task_id: &'p i32,
    pub new_title: Option<String>,
    pub new_description: Option<String>,
    pub new_done: Option<bool>,
    pub token: &'p String
}
