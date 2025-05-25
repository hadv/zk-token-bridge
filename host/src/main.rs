use bridge_guest::{BeaconHeader, Input, Transfer, VerifiedTransfer};
use ed25519_dalek::{SigningKey, Signer};
use rand::rngs::OsRng;
use risc0_zkvm::{default_prover, ExecutorEnv};
use sha2::{Digest, Sha256};

fn main() -> anyhow::Result<()> {
    // Sample beacon header
    let beacon = BeaconHeader {
        slot: 1,
        proposer_index: 42,
        parent_root: [0u8; 32],
        state_root: [1u8; 32],
        body_root: [2u8; 32],
    };

    // Compute expected root
    let mut hasher = Sha256::new();
    hasher.update(&beacon.slot.to_le_bytes());
    hasher.update(&beacon.proposer_index.to_le_bytes());
    hasher.update(&beacon.parent_root);
    hasher.update(&beacon.state_root);
    hasher.update(&beacon.body_root);
    let expected_root: [u8; 32] = hasher.finalize().into();

    // Transfer to verify
    let transfer = Transfer { from: 1, to: 2, amount: 10 };

    // Generate signing key and sign the transfer
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let pubkey_bytes: [u8; 32] = signing_key.verifying_key().to_bytes();
    let transfer_bytes = serde_json::to_vec(&transfer)?;
    let signature = signing_key.sign(&transfer_bytes).to_bytes();

    let input = Input {
        beacon,
        transfer,
        signature,
        pubkey: pubkey_bytes,
        expected_root,
    };

    let env = ExecutorEnv::builder()
        .write(&input)
        .build()?;

    let receipt = default_prover().prove(env, bridge_guest::ENTRY)?;
    let verified: VerifiedTransfer = receipt.journal.decode()?;
    println!(
        "Verified root: {:?}, transfer {} -> {} amount {}",
        verified.root, verified.transfer.from, verified.transfer.to, verified.transfer.amount
    );

    Ok(())
}
