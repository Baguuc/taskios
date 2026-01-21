/// struct representing a project
#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow, Debug, Clone)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

/// struct representing a project without id
#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow, Debug, Clone)]
pub struct ProjectWithoutId {
    pub name: String,
}

/// struct representing a project with all fields optional
#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow, Debug, Clone)]
pub struct PartialProject {
    pub name: Option<String>,
}

/// struct representing a project belonging to a user and user's permissions to it
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct UserProject {
    pub id: i32,
    pub name: String,
    pub permissions: Vec<String>,
}
