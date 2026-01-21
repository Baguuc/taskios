pub struct ProjectCreateParams<'p> {
    pub project: &'p crate::models::ProjectWithoutId,
    pub token: &'p String,
}

pub struct ProjectListParams<'p> {
    pub token: &'p String,
    // the page size is 5
    pub page_number: &'p u32,
}

pub struct ProjectUpdateParams<'p> {
    pub id: &'p i32,
    pub new_data: &'p crate::models::PartialProject,
    pub token: &'p String,
}

pub struct ProjectDeleteParams<'p> {
    pub id: &'p i32,
    pub token: &'p String,
}
