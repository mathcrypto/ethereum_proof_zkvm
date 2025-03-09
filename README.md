# Ethereum Block Proof zkVM

## Quick Start

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

### Build and Run
1. Build the Guest Code
```
cd methods/guest
cargo build --release
```
2. Build the Host Code
```
cd host
cargo build --release
```
3. Run the Ethereum Proof Generator
```
# Optional: export custom Ethereum provider URL
# export ETH_PROVIDER_URL="your_ethereum_node_url"

# Generate proof
cargo run --release
```
4. Verify the Proof
```
# Use the verifier to check the generated proof
cd verifier
cargo run --release ../host/proofs/ethereum_block_XXXXX_proof.bin
```
### What Happens

1. Connects to Ethereum and fetches the latest block
2. Creates a zero-knowledge proof validating the block
3. Generates a method ID for proof verification
4. Saves the proof to proofs/ directory
5. Provides a separate verification tool to validate the proof

## zkVM Proof Generation and Verification Flow

The project demonstrates a comprehensive zero-knowledge proof workflow:

1. Compile Guest Program

   * Guest program compiled to an ELF binary
   * ELF binary contains Ethereum block validation logic


2. Generate Method ID

    * Unique cryptographic identifier for the specific computation
    * Used to ensure proof integrity during verification


3. Execute and Prove

   * zkVM executes the ELF binary
   * Records and validates the execution session
   * Generates a cryptographic proof (receipt)


4. Verify Proof

   * Separate verification tool checks proof validity
   * Uses method ID to validate the specific computation

## Directory Structure

```text
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs         <-- [Proof Generation]
├── verifier
│   ├── Cargo.toml
│   └── src
│       └── main.rs     <-- [Proof Verification]
├── methods
    ├── Cargo.toml
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs  <-- [Guest program logic goes here]
    