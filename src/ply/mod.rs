use serde::{Deserialize, Serialize};
use super::db;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct System {
    pub name: String
}

impl db::Element for System {
    fn merge(&mut self,rhs:&Self) {

    }
}