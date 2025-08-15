#[derive(serde::Serialize, serde::Deserialize)]
pub struct UncompletedTask {
    pub id: i32,
    pub title: String,
    pub description: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CompletedTask {
    pub id: i32,
    pub title: String
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "completion")]
pub enum Task {
    #[serde(rename = "completed")]
    Uncompleted(UncompletedTask),

    #[serde(rename = "uncompleted")]
    Completed(CompletedTask),
}
