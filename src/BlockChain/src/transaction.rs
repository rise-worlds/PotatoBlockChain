use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Action {
    pub contract: String,
    pub name: String,
    pub actor: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub time: DateTime<Utc>,
    pub hash: String,
    pub actions: Vec<Action>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SignedTransaction {
    pub unpacked_trx: Transaction,
    pub signatures: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PackedTransaction {
    
}