use std::u64;
use crate::store::{Request, Response};
use std::io::Cursor;

mod store;
pub type NodeId = u64;

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Request,
        R = Response,
);