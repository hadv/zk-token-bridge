# zk Token Bridge Example

This repository provides a minimal example of a token bridge that uses the
[RISC Zero](https://www.risczero.com) zkVM. It contains two Rust crates:

- `bridge-guest` – the zkVM guest code that verifies a token transfer.
- `bridge-host` – the host application that runs the guest and verifies the proof.

The guest verifies an Ethereum beacon header and an Ed25519 signature on a
`Transfer` structure. If both checks pass, it commits the transfer back to the
host. The host uses `default_prover` from the `risc0-zkvm` crate to produce and
verify the proof.

## Building

RISC Zero and its custom target are required to compile the guest. In an offline
environment, the dependencies may not be available and compilation will fail.

To build (with network access):

```bash
cargo build --release --workspace
```

If the environment lacks network access after setup, the build will fail when
`cargo` attempts to download dependencies.

Run the example host binary with:

```bash
cargo run -p bridge-host
```
