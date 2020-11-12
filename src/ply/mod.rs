use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct System {
    pub name: String
}
