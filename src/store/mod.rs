use openraft::BasicNode;
use serde::Deserialize;
use serde::Serialize;

use crate::TypeConfig;
use openraft::SnapshotMeta;
use crate::NodeId;


mod memlog;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Request {
    Set {key : String, value: String},
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    result: Option<String>
}

// we have to define the LogStore, which has info regarding what is stored in logs, 
// how are logs stored and how can the data be retrived
pub type LogStore = memlog::LogStore<TypeConfig>;

pub struct StoredSnapshot {
    pub meta: SnapshotMeta<NodeId, BasicNode>,
    pub data: Vec<u8>
}