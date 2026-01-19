/// error returned when creating a project fails
#[derive(Debug, thiserror::Error)]
pub enum ProjectCreateError {}

/// error returned when updating user's project fails
#[derive(Debug, thiserror::Error)]
pub enum ProjectUpdateError {
    /// the project does not exist
    #[error("project_not_found")]
    ProjectNotFound
}

/// error returned when deleting user's project fails
#[derive(Debug, thiserror::Error)]
pub enum ProjectDeleteError {
    /// the project does not exist
    #[error("project_not_found")]
    ProjectNotFound
}