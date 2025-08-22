use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use risc0_zkvm::guest::{env, entry};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct BeaconHeader {
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: [u8; 32],
    pub state_root: [u8; 32],
    pub body_root: [u8; 32],
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transfer {
    pub from: u64,
    pub to: u64,
    pub amount: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub beacon: BeaconHeader,
    pub transfer: Transfer,
    pub signature: [u8; 64],
    pub pubkey: [u8; 32],
    pub expected_root: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct VerifiedTransfer {
    pub root: [u8; 32],
    pub transfer: Transfer,
}

#[entry]
fn main() {
    let input: Input = env::read();

    // Hash beacon header fields to obtain the root
    let mut hasher = Sha256::new();
    hasher.update(&input.beacon.slot.to_le_bytes());
    hasher.update(&input.beacon.proposer_index.to_le_bytes());
    hasher.update(&input.beacon.parent_root);
    hasher.update(&input.beacon.state_root);
    hasher.update(&input.beacon.body_root);
    let root: [u8; 32] = hasher.finalize().into();

    if root != input.expected_root {
        panic!("Beacon header root mismatch");
    }

    // Verify the transfer signature
    let vk = VerifyingKey::from_bytes(&input.pubkey).expect("invalid pubkey");
    let sig = Signature::from_bytes(&input.signature).expect("invalid signature");
    let transfer_bytes = serde_json::to_vec(&input.transfer).expect("serialize");
    vk.verify(&transfer_bytes, &sig).expect("signature check failed");

    let verified = VerifiedTransfer {
        root,
        transfer: input.transfer,
    };
    env::commit(&verified);
}
