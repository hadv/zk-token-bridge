use risc0_zkvm::guest::env;
use risc0_zkvm::guest::entry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    pub from: u64,
    pub to: u64,
    pub amount: u64,
}

#[entry]
fn main() {
    // Read the transfer from the host
    let transfer: Transfer = env::read();

    // In a real bridge we would verify signatures or Merkle proofs here.
    // For this minimal example we simply echo the transfer back.
    env::commit(&transfer);
}
