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
#[serde(tag = "completion")]
pub enum Task {
    #[serde(rename = "completed")]
    Uncompleted(UncompletedTask),

    #[serde(rename = "uncompleted")]
    Completed(CompletedTask),
}
