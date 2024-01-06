use serde::{Deserialize, Serialize};

impl HomeResponse {
    #[must_use]
    pub fn new(welcome_message : &str) -> Self {
        Self {
            welcome_message: welcome_message.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HomeResponse {
    pub welcome_message: String,
}
