use serde::{Deserialize, Serialize};
use super::db;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Assembly {
    pub name: String
}

impl From<db::DbEntity> for Assembly {
    fn from(d: db::DbEntity) -> Self {
        Assembly{
            name:d.0
        }
    }
}

impl db::Element for Assembly {
    fn merge(&mut self,_rhs:&Self) {

    }
    fn fmerge(self, _rhs:&Assembly) -> Assembly {
        self.clone()

    }
}