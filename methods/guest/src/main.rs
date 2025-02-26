use risc0_zkvm::guest::env;

fn main() {
    // TODO: Implement guest code here

    // read the Ethereum Block input
    let block: EthereumBlock = env::read();

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
