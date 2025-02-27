# Ethereum Block Proof zkVM


## Quick Start

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following
command:

```bash
cargo run
```
cargo risczero build --manifest-path methods/guest/Cargo.toml
which generates ELF files
cargo build --manifest-path host/Cargo.toml 
cargo run --bin host
cargo run --manifest-path host/Cargo.toml

 
## zkVM Proof Generation and Verification Flow

Proving the correct execution of a guest program using zkVM involves compiling the guest program into an ELF binary, running it on the zkVM, generating a cryptographic proof (receipt), and verifying the proof.

### Step-by-Step Overview:

1. **Compile the Guest Program to an ELF Binary**

The guest program (in this case, the Ethereum block proof program) is compiled into an ELF binary. This ELF file is what the zkVM will execute.
```rust
const ETHEREUM_PROOF_GUEST_ELF: &str = "../methods/guest/target/riscv32im-risc0-zkvm-elf/docker/ethereum_proof_guest";
```
The ELF binary contains the logic for generating a proof based on the Ethereum block.


2. **Executor Runs the ELF Binary and Records the Session**

The zkVM executes the compiled ELF binary and records the session. This session captures the execution state of the program, which is needed for later verification.
In the code:

```rust
let env = ExecutorEnv::builder()
    .write(&ethereum_block)  // Input data (Ethereum block) to the zkVM
    .unwrap()
    .build()
    .unwrap();

let prover = default_prover();  // Default prover to handle the execution
let elf_path = std::fs::read(ETHEREUM_PROOF_GUEST_ELF).expect("Failed to read ELF file");  // Read the ELF binary
let prove_info = prover.prove(env, &elf_path).unwrap(); 
``` 
3. **Prover Validates the Execution and Generates a Receipt**

After executing the program, the prover checks the validity of the session and generates a receipt. This receipt serves as the proof of correct execution of the guest program.
```rust
let receipt = prove_info.receipt; 
```

4. **Verification of Execution**

To verify the correctness of the program execution, the receipt is validated using the ImageID. The ImageID is a cryptographic identifier of the expected ELF binary. It ensures that the exact program was executed.

```rust
receipt.verify(guest_id_digest).unwrap(); 
```



## Directory Structure


```text
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs                    <-- [Host code goes here]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── method_name.rs         <-- [Guest code goes here]
    └── src
        └── lib.rs

