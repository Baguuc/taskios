#[derive(serde::Serialize, serde::Deserialize)]
pub struct UncompletedTask {
    id: i32,
    title: String,
    description: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CompletedTask {
    id: i32,
    title: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Task {
    Uncompleted(UncompletedTask),
    Completed(CompletedTask),
}
