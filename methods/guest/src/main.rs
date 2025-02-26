mod models;
use risc0_zkvm::guest::env;
use sha2::{Sha256, Digest as ShaDigest};
use risc0_zkvm::sha::Digest;
use crate::models::EthereumBlock;
fn main() {
    // TODO: Implement guest code here

    // read the Ethereum Block input
    let block: EthereumBlock = env::read();

    // Step 2: Validate the block hash and convert it to Digest
    let is_valid_hash = block.hash == compute_block_hash(&block);
    let valid_hash_digest = hash_to_digest(is_valid_hash.to_string());

    // Step 3: Validate the block timestamp and convert it to Digest
    let is_valid_timestamp = block.timestamp > 0;
    let valid_timestamp_digest = hash_to_digest(is_valid_timestamp.to_string());


    // write public output to the journal
    env::commit(&(valid_hash_digest, valid_timestamp_digest));
}

// Function to validate the block's hash (simple check for illustration)
fn validate_block_hash(block: &EthereumBlock) -> String {
    block.hash.clone()
}

// Example of computing the block hash (assuming it's already a valid hash)
fn compute_block_hash(block: &EthereumBlock) -> String {
    block.hash.clone()
}

// Helper function to convert a string into a Digest using SHA256
fn hash_to_digest(input: String) -> Digest {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let fixed_size_array: [u8; 32] = result.into();
    Digest::from(fixed_size_array)
}
