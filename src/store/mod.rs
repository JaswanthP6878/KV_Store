use std::collections::BTreeMap;

use openraft::BasicNode;
use openraft::LogId;
use openraft::Node;
use openraft::StoredMembership;
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


// stored SnapShot is used to send the copy to other raft instances
// or restore the state
#[derive(Debug)]
pub struct StoredSnapshot {
    pub meta: SnapshotMeta<NodeId, BasicNode>,
    // the data at a given snapshot moment
    pub data: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug,Default, Clone)]
pub struct StateMachineData {
    pub last_applied_log: Option<LogId<NodeId>>,
 
    pub last_membership_log: StoredMembership<NodeId, BasicNode>,

    pub data: BTreeMap<String, String>,
}




