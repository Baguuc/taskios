/// struct representing a task
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub project_id: i32,
}

/// struct representing a task without id
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct TaskWithoutId {
    pub title: String,
    pub description: String,
    pub done: bool,
    pub project_id: i32,
}

/// struct representing a partial task
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct PartialTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
}
