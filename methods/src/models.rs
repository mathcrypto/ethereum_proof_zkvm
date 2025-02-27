/* // methods/src/models.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumBlock {
    pub hash: String,
    pub parent_hash: String,
    pub timestamp: u64,
    pub number: u64,
    pub transactions_root: String,
}
*/
