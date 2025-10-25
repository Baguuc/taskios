pub struct ProjectCreateParams<'p> {
    pub name: &'p String
}

pub struct ProjectUpdateParams<'p> {
    pub id: &'p i32,
    pub new_name: &'p Option<String>
}
