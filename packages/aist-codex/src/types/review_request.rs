use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReviewRequest {
    pub id: String,
    pub question: String,
}
