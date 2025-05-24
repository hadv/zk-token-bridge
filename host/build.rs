fn main() {
    // Build the zkVM guest ELF
    risc0_build::embed_methods!("../guest");
}
