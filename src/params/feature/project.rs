/// Parameters required to create a project.
pub struct ProjectCreateParams<'p> {
    /// Data of the project to create.
    pub project: &'p crate::models::ProjectWithoutId,
    /// User's token.
    pub token: &'p String,
}

/// Parameters required to list user's projects.
pub struct ProjectListParams<'p> {
    /// User's token.
    pub token: &'p String,
    /// Number of the page to retrieve (for pagination).
    /// The page size is 5.
    pub page_number: &'p u32,
}

/// Parameters required to update a project.
pub struct ProjectUpdateParams<'p> {
    /// The ID of the project to update.
    pub id: &'p i32,
    /// The new data of the project (received as a partial).
    pub new_data: &'p crate::models::PartialProject,
    /// User's token.
    pub token: &'p String,
}

/// Parameters required to delete a project.
pub struct ProjectDeleteParams<'p> {
    /// The ID of the project to delete.
    pub id: &'p i32,
    /// User's token.
    pub token: &'p String,
}
