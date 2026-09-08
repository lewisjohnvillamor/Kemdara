# Kemdara

A local, cross-platform cryptography experimentation workbench for **learning, verification, and benchmarking**. It is intentionally **not** a production protocol implementation.

Initial adapters:

- X25519 (`x25519-dalek`) — RFC 7748
- X448 (`crrl`) — RFC 7748
- NIST P-256 ECDH (`p256`) — SP 800-186 / SP 800-56A
- ML-KEM-768 (`ml-kem`) — FIPS 203

The workbench runs the same code on Windows x86-64 and macOS (Intel or Apple Silicon), records machine metadata, verifies exchanges, runs official RFC 7748 known-answer tests for X25519/X448, and displays local benchmark comparisons.

> **Safety:** experimental/research software. Do not use custom algorithms or these benchmark wrappers to protect production traffic.

## Requirements

Install the current stable Rust toolchain. The project requires Rust 1.85 or newer.

Windows (PowerShell):

```powershell
winget install Rustlang.Rustup
rustup default stable
```

macOS:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

## Run the GUI

```bash
cargo run --release
```

The first build downloads and compiles the dependencies.

## Headless benchmark

Useful when comparing your Windows PC with the spare MacBook:

```bash
cargo run --release -- --cli --iterations 500 > windows.json
```

On the MacBook:

```bash
cargo run --release -- --cli --iterations 500 > macbook.json
```

Keep the machine idle and plugged in when comparing microbenchmarks. The GUI records `x86_64` vs `aarch64` plus useful CPU features such as AVX2/AES-NI or NEON/AES where available.

For machine-specific peak measurements, compile with native CPU tuning:

Windows PowerShell:

```powershell
$env:RUSTFLAGS="-C target-cpu=native"
cargo run --release -- --cli --iterations 500 > windows-native.json
```

macOS:

```bash
RUSTFLAGS="-C target-cpu=native" cargo run --release -- --cli --iterations 500 > mac-native.json
```

Do not distribute a `target-cpu=native` binary to unrelated machines; use it only for local benchmarking.

## What a benchmark means

The current timing is a deliberately simple **full key-establishment round trip**:

- X25519/X448/P-256: generate two ephemeral keypairs and compute both peers' shared secret.
- ML-KEM-768: keygen + encapsulate + decapsulate.

These are not identical protocols, so the number is a local research comparison, not a universal claim that one primitive is "better".

## Add an experimental algorithm

Implement `EstablishmentAlgorithm` in `src/algorithms.rs`, give it metadata, a `run_once()` correctness path, and ideally a known-answer test. Then add it to `registry()`.

The intended future order is:

1. Official known-answer vectors
2. Cross-implementation checks
3. Wycheproof/adversarial vectors
4. Fuzz/property testing
5. Side-channel experiments
6. Microbenchmarks
7. Protocol/network benchmarks
8. Only then experimental curves/KEMs/hybrid logic

## Next planned modules

- ML-KEM-512 / ML-KEM-1024
- AES-GCM vs ChaCha20-Poly1305 payload benchmark
- HKDF/BLAKE2s/SHA-2/SHA-3 comparison
- Wycheproof importer
- Noise-style handshake composer
- toy elliptic-curve visualizer
- result comparison/import between machines
- custom algorithm plugin folder

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Kemdara by you will be dual-licensed as above, without additional terms or conditions.
