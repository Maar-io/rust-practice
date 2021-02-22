use std::fmt::{ self, Debug, Formatter };

use super::traits::{Hashable};
use super::*;


pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,

}

impl Block {
    pub fn new(index: u32, timestamp: u128, prev_block_hash: BlockHash, 
        nonce: u64, payload: String) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

impl Hashable for Block{
    fn bytes(&self) -> Vec<u8>{
        let mut b = vec![];
        b.extend(&u32_bytes(&self.index));
        b.extend(&u128_bytes(&self.timestamp));
        b.extend(&self.prev_block_hash);
        b.extend(&u64_bytes(&self.nonce));
        b.extend(self.payload.as_bytes());

        b
    }
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with {}",
        &self.index,
        &hex::encode(&self.hash),
        &self.timestamp,
        &self.payload,
        )
    }
}
