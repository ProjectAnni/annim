use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct IdOnly {
    id: String,
}

impl IdOnly {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
}