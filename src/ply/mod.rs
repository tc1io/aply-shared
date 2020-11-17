use serde::{Deserialize, Serialize};
use super::db;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Assembly {
    pub org_id: String,
    pub domain: String,
    pub org_name: String,

}
impl From<db::DbEntity> for Assembly {
    fn from(d: db::DbEntity) -> Self {
        Assembly{
            org_id: d.0.to_string(),
            domain: d.0.to_string(),
            org_name: d.0.to_string(),
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