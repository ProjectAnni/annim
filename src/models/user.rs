use serde::{Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct UserInfo {
    id: String,
    username: String,
    email: String,
    nickname: String,
    avatar: String,
}

impl UserInfo {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
}