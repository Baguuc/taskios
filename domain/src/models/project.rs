#[derive(serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub tasks: Vec<crate::Task>
}
