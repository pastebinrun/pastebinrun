use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Form {
    pub nickname: String,
    pub password: String,
    pub confirm_password: String,
}
