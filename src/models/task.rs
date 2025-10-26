#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool
}
