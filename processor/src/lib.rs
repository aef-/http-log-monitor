use serde::{Deserialize, Serialize};

pub type Date = i64;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub remotehost: String,
    pub rfc931: String,
    pub authuser: String,
    pub date: Date,
    pub request: String,
    pub status: u16,
    pub bytes: u64,
}
