// Block represents a data structure containing special fields explained below.
pub struct Block {
    timestamp: u128, // current block timestamp in seconds.
    transactions: String, //
    previous_block_hash: String, // 256-bit has of the previous block header.
    hash_merkle_root: String, // 256-bit has of the transactions in the current block.
    height: usize,
    nonce: i32, // 32-bit number that starts at 0.
}

pub struct Blockchain {
    blocks: Vec<Block>
}

fn main() {}