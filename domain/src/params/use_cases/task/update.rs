pub struct Params {
    pub user_token: String,
    pub id: i32,
    pub new: NewData
}

pub struct NewData {
    pub title: String,
    pub description: String
}
