use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.nonce
        ));
        hex::encode(hasher.finalize())
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending: Vec<Transaction>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, vec![], String::from("0"));
        Self {
            chain: vec![genesis_block],
            pending: vec![],
            difficulty: 3,
        }
    }

    pub fn add_transaction(&mut self, txn: Transaction) {
        self.pending.push(txn);
    }

    pub fn mine_block(&mut self) -> Block {
        let last_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            last_block.index + 1,
            self.pending.clone(),
            last_block.hash.clone()
        );
        new_block.mine(self.difficulty);
        self.chain.push(new_block.clone());
        self.pending.clear();
        new_block
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let prev = &self.chain[i-1];
            let curr = &self.chain[i];

            if curr.hash != curr.calculate_hash() || curr.previous_hash != prev.hash {
                return false;
            }
        }
        true
    }
}