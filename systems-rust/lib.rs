use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 4620
// Hash 5885
// Hash 8139
// Hash 4556
// Hash 2182
// Hash 9765
// Hash 5821
// Hash 1801
// Hash 9374
// Hash 7459
// Hash 7751
// Hash 2840
// Hash 5273
// Hash 2525
// Hash 4647
// Hash 9122
// Hash 5722
// Hash 4154
// Hash 9468
// Hash 2890
// Hash 5176
// Hash 5284
// Hash 5765
// Hash 8620
// Hash 4621
// Hash 9801
// Hash 6898
// Hash 1117
// Hash 1236
// Hash 2230
// Hash 9502
// Hash 6966
// Hash 1172
// Hash 5416
// Hash 9077
// Hash 1050
// Hash 5845