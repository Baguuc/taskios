/// Error returned when creating a project fails
#[derive(Debug, thiserror::Error)]
pub enum ProjectCreateError {}

/// Error returned when updating user's project fails
#[derive(Debug, thiserror::Error)]
pub enum ProjectUpdateError {
    /// The project does not exist
    #[error("project_not_found")]
    ProjectNotFound,
}

/// Error returned when deleting user's project fails
#[derive(Debug, thiserror::Error)]
pub enum ProjectDeleteError {
    /// The project does not exist
    #[error("project_not_found")]
    ProjectNotFound,
}
