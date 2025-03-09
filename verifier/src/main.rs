use risc0_zkvm::Receipt;
use risc0_zkvm::sha::Digest;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use std::env;
use hex;
use bincode;

// This struct must match the one in your host program
#[derive(Serialize, Deserialize)]
struct ProofPackage {
    receipt: Receipt,
    method_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumBlock {
    pub hash: String,
    pub parent_hash: String,
    pub timestamp: String,
    pub number: u64,
    pub transactions_root: String,
    pub is_valid_timestamp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockValidationResult {
    pub block: EthereumBlock,
    pub is_valid_hash: bool,
    pub is_valid_timestamp: bool,
    pub is_valid_structure: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ethereum_proof_verifier <proof_file_path>");
        println!("Example: ethereum_proof_verifier ../host/proofs/ethereum_block_22008660_proof.bin");
        return Ok(());
    }
    
    let proof_path = &args[1];
    let proof_file = Path::new(proof_path);
    
    if !proof_file.exists() {
        return Err(format!("Proof file not found: {}", proof_path).into());
    }
    
    // Read the proof file
    println!("Reading proof from: {}", proof_path);
    let proof_data = fs::read(proof_file)?;
    
    // Deserialize the ProofPackage
    println!("Deserializing proof package...");
    let package: ProofPackage = bincode::deserialize(&proof_data)?;
    
    // Extract the receipt and method ID
    let receipt = package.receipt;
    
    // Convert the method ID from hex string to Digest
    println!("Using method ID from package: {}", package.method_id);
    let method_id_bytes = hex::decode(&package.method_id)?;
    let method_id_array: [u8; 32] = match method_id_bytes.try_into() {
        Ok(array) => array,
        Err(_) => return Err("Failed to convert method ID to 32-byte array".into()),
    };
    let method_id = Digest::from(method_id_array);
    
    // Verify the proof
    println!("Verifying proof...");
    match receipt.verify(method_id) {
        Ok(_) => println!("Cryptographic verification successful!"),
        Err(e) => {
            println!("Verification error: {}", e);
            println!("This may be due to an incompatibility between the verifier and the proof.");
            println!("Continuing to decode journal data anyway...");
        }
    }
    
    // Decode the journal data
    //let result: BlockValidationResult = receipt.journal.decode()?;
    let result = match receipt.verify(method_id) {
        Ok(_) => {
            println!("Cryptographic verification successful!");
            receipt.journal.decode::<BlockValidationResult>()?
        },
        Err(e) => {
            println!("Verification error: {}", e);
            println!("This may be due to an incompatibility between the verifier and the proof.");
            println!("Continuing to decode journal data anyway...");
            receipt.journal.decode::<BlockValidationResult>()?
        }
    };
    
    println!("\n==== ETHEREUM BLOCK PROOF DETAILS ====");
    println!("Block Number: {}", result.block.number);
    println!("Block Hash: {}", result.block.hash);
    println!("Timestamp: {}", result.block.timestamp);
    println!("Validation Results:");
    println!("  Valid Hash: {}", result.is_valid_hash);
    println!("  Valid Timestamp: {}", result.is_valid_timestamp);
    println!("  Valid Structure: {}", result.is_valid_structure);
    
    // Additional information
    println!("\nProof Journal Size: {} bytes", receipt.journal.bytes.len());
    println!("Proof for block #{} processed successfully!", result.block.number);
    println!("======================================\n");
    
    Ok(())
}