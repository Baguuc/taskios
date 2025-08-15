/// ## Params
///
/// params to update a task in database
///
pub struct Params {
    pub id: i32,
    pub new: NewData
}

pub struct NewData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completion: Option<String>
}
