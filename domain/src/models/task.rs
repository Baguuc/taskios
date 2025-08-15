#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum TaskCompletion {
    Completed,
    Uncompleted
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completion: TaskCompletion
}
