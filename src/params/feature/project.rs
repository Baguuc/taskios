pub struct ProjectCreateParams<'p> {
    pub name: &'p String,
    pub token: &'p String
}

pub struct ProjectUpdateParams<'p> {
    pub id: &'p i32,
    pub new_name: &'p Option<String>,
    pub token: &'p String
}

pub struct ProjectDeleteParams<'p> {
    pub id: &'p i32,
    pub token: &'p String
}
