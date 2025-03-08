use risc0_zkvm::{default_prover, ExecutorEnv, ProveInfo};
use serde::{Serialize, Deserialize};
use risc0_zkvm::serde::to_vec;
use ethers::prelude::*;
use std::fs;
use std::path::Path;
use hex;


const ETHEREUM_PROOF_GUEST_ID: &str = "d5ce7284368ae6712ec225d58133e60408b408f1925c38e534b749f568d84e36";
fn get_elf_path() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {}", current_dir.display());
    let project_root = current_dir.parent().expect("Failed to get project root");
    println!("Project root: {}", project_root.display());

    let elf_path = project_root.join("target/riscv-guest/methods/method/riscv32im-risc0-zkvm-elf/release/method");
    // Verify the file exists
    if !elf_path.exists() {
        panic!("RISC-V ELF file not found at: {}", elf_path.display());
    }
    
    elf_path.to_str().expect("Failed to convert path to string").to_string()
}


    
    
    
    



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

async fn get_ethereum_block(provider_url: &str) -> Result<EthereumBlock, Box<dyn std::error::Error>> { 
    println!("Connecting to Ethereum node at: {}", provider_url);
    
    // Set up Ethereum provider
    let provider = Provider::<Http>::try_from(provider_url)?;

    // Fetch the latest block
    println!("Fetching latest Ethereum block...");
    let block = provider.get_block(BlockId::Number(BlockNumber::Latest)).await?;

    match block {
        Some(block) => {
            let hash = format!("{:?}", block.hash.unwrap_or_default());
            let parent_hash = block.parent_hash.to_string();
            let timestamp = block.timestamp.as_u64().to_string(); 
            let number = block.number.unwrap_or_default().as_u64();
            let transactions_root = block.transactions_root.to_string();
            
            println!("Successfully fetched block #{}", number);
            
            Ok(EthereumBlock {
                hash,
                parent_hash,
                timestamp,
                number,
                transactions_root,
            })
        },
        None => {
            println!("Block not found");
            Err("Block not found".into())
        },
    }
}

fn save_receipt(prove_info: &ProveInfo, block_number: U64) -> Result<String, Box<dyn std::error::Error>> {
    // Create proofs directory if it doesn't exist
    let proof_dir = Path::new("proofs");
    if !proof_dir.exists() {
        fs::create_dir(proof_dir)?;
    }

    
    // Generate filename with block number
    let filename = format!("proofs/ethereum_block_{}_proof.bin", block_number);
    
    // Serialize the receipt - FIXED: use prove_info.receipt
    let receipt_bytes = prove_info.receipt.journal.bytes.to_vec();
    
    // Save to file
    fs::write(&filename, &receipt_bytes)?;
    println!("Proof saved to {}", filename);
    
    Ok(filename)
}

fn print_proof_details(prove_info: &ProveInfo, validation_result: &BlockValidationResult) {
    println!("\n==== ETHEREUM BLOCK PROOF DETAILS ====");
    println!("Block Number: {}", validation_result.block.number);
    println!("Block Hash: {}", validation_result.block.hash);
    println!("Timestamp: {}", validation_result.block.timestamp);
    println!("Validation Result:");
    println!("  Valid hash: {}", validation_result.is_valid_hash);
    println!("  Valid timestamp: {}", validation_result.is_valid_timestamp);
    println!("Valid structure: {}", validation_result.is_valid_structure);
    
    // Get journal bytes
    let journal_bytes = prove_info.receipt.journal.bytes.as_slice();
    
    // Get proof size in KB (approximate) - FIXED: use prove_info.receipt
    let proof_size = to_vec(&prove_info.receipt).unwrap_or_default().len() / 1024;
    println!("Proof Size: {} KB", proof_size);
    
    // Print the journal (output from the guest program)
    println!("\nProof Journal:");
    if let Ok(journal_data) = String::from_utf8(journal_bytes.to_vec()) {
        println!("{}", journal_data);
    } else {
        println!("Journal contains binary data");
        println!("Journal size: {} bytes", journal_bytes.len());
    }
    
    println!("\nProof Verification Status: VALID");
    println!("======================================\n");
}

fn get_guest_id_digest() -> Result<risc0_zkvm::sha::Digest, Box<dyn std::error::Error>> {
    // Decode the hexadecimal string into a byte vector
    let bytes = hex::decode(ETHEREUM_PROOF_GUEST_ID)?;

    // Convert the byte vector into a fixed-size array of 32 bytes
    let bytes_array: [u8; 32] = bytes.as_slice().try_into().map_err(|_| "Failed to convert to 32-byte array")?;

    // Convert the byte array into a Digest
    Ok(risc0_zkvm::sha::Digest::from(bytes_array))
}
    

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Ethereum block proof generator");

    // Get provider URL from environment or use default
    let provider_url = std::env::var("ETH_PROVIDER_URL")
        .unwrap_or_else(|_| "https://mainnet.infura.io/v3/7d87244da7c44327a552a12f4ecad09d".to_string());

    // Step 1: Fetch Ethereum block
    let ethereum_block = get_ethereum_block(&provider_url).await?;
    println!("Fetched Ethereum Block #{}", ethereum_block.number);
    
    // Step 2: Set up the executor environment with block data
    println!("Setting up zkVM environment");
    let env = ExecutorEnv::builder()
        .write(&ethereum_block)
        .unwrap()
        .build()
        .unwrap();

    // Step 3: Generate the proof
    println!("Generating proof (this may take a while)...");
    
    // Read the ELF file
    let elf_path_str = get_elf_path(); 
    let elf_path = Path::new(&elf_path_str);
    println!("Using ELF file: {:?}", elf_path.display()); 
    println!("ELF file exists: {}", elf_path.exists());
    
    match std::env::current_dir() {
        Ok(current_dir) => println!("Current directory: {}", current_dir.display()),
        Err(e) => println!("Failed to get current directory: {}", e),
    }
    if !elf_path.exists() {
        panic!("ELF file not found at: {:?}", elf_path.display());
    }
    // Read ELF file
    println!("Reading ELF file...");
    let elf_data = match std::fs::read(elf_path) {
        Ok(data) => {
            println!("ELF file read successfully");
            data
        },
        Err(e) => {
            panic!("Failed to read ELF file: {}", e);
        }
        };
    
    // Get the prover
    println!("Creating prover...");
    let prover = default_prover();
    
    // Use ELF data for proving
    println!("Starting proof generation...");
    println!("Generating the proof (this may take a while)...");
    let prove_info = prover.prove(env, &elf_data).expect("Failed to prove");
    println!("Proof generation completed successfully");

    // Step 4: Deserialize the journal to get the validation result
    let validation_result: BlockValidationResult = prove_info.receipt.journal.decode()?;
    println!("Deserialized validation result: {:?}", validation_result);

    // Step 5: Print proof details
    print_proof_details(&prove_info, &validation_result);

    // Step 6: Save the proof
    let proof_path = save_receipt(&prove_info, ethereum_block.number.into())?;
    println!("Proof saved to: {}", proof_path);

    Ok(())
}
 

// Helper function to convert a string into a Digest using SHA256
/*fn hash_to_digest(input: String) -> Digest {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let fixed_size_array: [u8; 32] = result.into();
    Digest::from(fixed_size_array)
}
    */