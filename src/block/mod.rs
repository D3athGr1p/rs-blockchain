use std::error::Error;

use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub nonce: u64,
    pub height: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
}

impl Block {
    pub fn new(
        nonce: u64,
        height: u64,
        timestamp: u128,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> Self {
        Block {
            nonce,
            height,
            timestamp,
            transactions,
            previous_hash,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let block_string = serde_json::to_string(self).expect("Failed to serialize block");
        let mut hasher = Sha256::new();
        hasher.update(block_string);
        format!("{:x}", hasher.finalize())
    }

    pub fn validate(&self, difficulty: u64) -> Result<bool, Box<dyn Error>> {
        Ok(self
            .calculate_hash()
            .starts_with(&"0".repeat(difficulty as usize)))
    }

    pub fn print_block_json(&self) {
        let block_hash = self.calculate_hash();
        let mut block_value = serde_json::to_value(self).unwrap();
        block_value
            .as_object_mut()
            .unwrap()
            .insert("hash".to_string(), json!(block_hash));

        let json_string = serde_json::to_string_pretty(&block_value).unwrap();
        println!("{}", json_string);
    }
}
