use risc0_zkvm::guest::env;
use sha2::{Sha256, Digest as ShaDigest};
use risc0_zkvm::sha::Digest;
use serde::{Serialize, Deserialize};

//use methods::models::EthereumBlock; // Import the EthereumBlock from models in methods/src

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumBlock {
    pub hash: String,
    pub parent_hash: String,
    pub timestamp: u64,
    pub number: u64,
    pub transactions_root: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockValidationResult {
    pub block_number: u64,
    pub is_valid_hash: bool,
    pub is_valid_timestamp: bool,
    pub is_valid_structure: bool,
}

fn main() {
    // Step 1: read the Ethereum Block input
    
    let block: EthereumBlock = env::read();

    // Step 2: Validate the block hash and convert it to Digest
    let is_valid_hash = validate_block_hash(&block);
    let valid_hash_digest = hash_to_digest(is_valid_hash.to_string());

    // Step 3: Validate the block timestamp and convert it to Digest
    let is_valid_timestamp = block.timestamp > 0;
    let valid_timestamp_digest = hash_to_digest(is_valid_timestamp.to_string());
    
    // Structure validation (check if all required fields are present)
    let is_valid_structure = !block.hash.is_empty() &&
                             !block.parent_hash.is_empty() &&
                             !block.transactions_root.is_empty();
    
    let validation_result = BlockValidationResult {
        block_number: block.number,
        is_valid_hash,
        is_valid_timestamp,
        is_valid_structure,
    };

    // write public output to the journal
    env::commit(&validation_result);
}

// Function to validate the block's hash 
fn validate_block_hash(block: &EthereumBlock) -> bool {
    !block.hash.is_empty() && block.hash.starts_with("0x")
}
// Helper function to convert a string into a Digest using SHA256

fn hash_to_digest(input: String) -> Digest {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let fixed_size_array: [u8; 32] = result.into();
    Digest::from(fixed_size_array)
}



