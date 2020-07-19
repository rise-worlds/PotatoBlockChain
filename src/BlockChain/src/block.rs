use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::str;

mod transaction;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub number: i64,
    pub pre_hash: String,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SignedBlock {
    pub block: Block,
    pub signatures: Vec<String>
}