use bridge_guest::{Transfer};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() -> anyhow::Result<()> {
    // Sample transfer to prove
    let transfer = Transfer { from: 1, to: 2, amount: 10 };

    // Set up the zkVM execution environment
    let env = ExecutorEnv::builder()
        .write(&transfer)
        .build()?;

    // Run the guest to generate a receipt
    let receipt = default_prover().prove(env, bridge_guest::ENTRY)?;

    // Verify that the guest committed to the same transfer
    let verified: Transfer = receipt.journal.decode()?;
    println!(
        "Proof verified transfer: {} -> {} amount {}",
        verified.from, verified.to, verified.amount
    );

    Ok(())
}
