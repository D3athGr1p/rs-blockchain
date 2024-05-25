use sha2::{Digest, Sha256};

use crate::{block::Block, transaction::Transaction};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u64,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 4,
            pending_transactions: Vec::new(),
        };
        blockchain.create_genesis_block();
        blockchain
    }

    pub fn create_genesis_block(&mut self) {
        let genesis = Block::new(
            0,
            0,
            Self::current_timestamp(),
            Vec::new(),
            String::from("0"),
        );
        self.chain.push(genesis);
    }

    pub fn create_transaction(&mut self, sender: String, recipient: String, amount: u64) {
        let transaction = Transaction::new(sender, recipient, amount);
        self.pending_transactions.push(transaction);
    }

    fn current_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    pub fn create_new_block(&mut self) -> Block {
        let previous_hash = self.last_block().calculate_hash();
        let mut nonce = 0;
        let mut block = Block::new(
            nonce,
            self.chain.len() as u64,
            Self::current_timestamp(),
            self.pending_transactions.clone(),
            previous_hash,
        );

        while !self.valid_nonce(nonce, &block, self.difficulty) {
            nonce += 1;
        }

        block.nonce = nonce;

        self.pending_transactions.clear();
        self.chain.push(block.clone());
        block
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn valid_nonce(&self, nonce: u64, block: &Block, difficulty: u64) -> bool {
        let mut block = block.clone();
        block.nonce = nonce;

        let block_string = serde_json::to_string(&block).expect("Failed to serialize block");

        let mut hasher = Sha256::new();
        hasher.update(block_string);
        let guess_hash = format!("{:x}", hasher.finalize());
        guess_hash.starts_with(&"0".repeat(difficulty as usize))
    }
}
