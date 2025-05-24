# zk Token Bridge Example

This repository provides a minimal example of a token bridge that uses the
[RISC Zero](https://www.risczero.com) zkVM. It contains two Rust crates:

- `bridge-guest` – the zkVM guest code that verifies a token transfer.
- `bridge-host` – the host application that runs the guest and verifies the proof.

The guest reads a `Transfer` structure from the host, performs any desired
checks, and commits the transfer back to the host. The host uses `default_prover`
from the `risc0-zkvm` crate to produce and verify the proof.

## Building

RISC Zero and its custom target are required to compile the guest. In an offline
environment, the dependencies may not be available and compilation will fail.

To build (with network access):

```bash
cargo build --release --workspace
```

Run the example host binary with:

```bash
cargo run -p bridge-host
```
