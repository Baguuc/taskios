#[derive(serde::Serialize, serde::Deserialize)]
pub struct Project {
    id: i32,
    name: String,
    tasks: Vec<crate::Task>
}
