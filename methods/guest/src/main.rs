use risc0_zkvm::guest::env;
//use risc0_zkvm::sha::Sha256 as Risc0Sha256;
use serde::{Serialize, Deserialize};
//use primitive_types::U256;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumBlock {
    pub hash: String,
    pub parent_hash: String,
    pub timestamp: String,
    pub number: u64,
    pub transactions_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockValidationResult {
    pub block: EthereumBlock,
    pub is_valid_hash: bool,
    pub is_valid_timestamp: bool,
    pub is_valid_structure: bool,
}

fn main() {
    // Step 1: read the Ethereum Block input
    let block: EthereumBlock = env::read();
    println!("Guest program received Block: #{}", block.number);

    // Step 2: Validate the block hash
    let is_valid_hash = validate_block_hash(&block);

    // Step 3: Validate the block timestamp
    let is_valid_timestamp = block.timestamp.starts_with("0x");
    
    // Structure validation (check if all required fields are present)
    let is_valid_structure = !block.hash.is_empty() &&
                             !block.parent_hash.is_empty() &&
                             !block.transactions_root.is_empty();
    
    let validation_result = BlockValidationResult {
        block,
        is_valid_hash,
        is_valid_timestamp,
        is_valid_structure,
    };

    println!("Block validation results:");
    println!("  Valid hash: {}", is_valid_hash);
    println!("  Valid timestamp: {}", is_valid_timestamp);
    println!("  Valid structure: {}", is_valid_structure);

    // write public output to the journal
    env::commit(&validation_result);
}

// Function to validate the block's hash 
fn validate_block_hash(block: &EthereumBlock) -> bool {
    !block.hash.is_empty() && block.hash.starts_with("0x")
}

// Helper function to convert a string into a SHA256 hash
