# Kemdara

A local, cross-platform cryptography experimentation workbench for **learning, verification, and benchmarking**. It is intentionally **not** a production protocol implementation.

Executable adapters are grouped by workload:

- Key establishment: X25519, X448, P-256 ECDH, and ML-KEM-512/768/1024
- Payload encryption: AES-GCM, ChaCha20-Poly1305, XChaCha20-Poly1305, AES-256-GCM-SIV, and Ascon-AEAD128
- Hash/XOF/KDF: SHA-2, SHA-3, BLAKE2s, BLAKE3, Ascon-Hash256/XOF128, KangarooTwelve, and HKDF-SHA-256
- Password derivation: Argon2id and scrypt with explicit memory/work parameters
- Signatures: Ed25519, ECDSA P-256, ML-DSA-44/65/87, and SLH-DSA-SHAKE-128f
- Experimental hybrids: X25519 + ML-KEM-768 and P-256 + ML-KEM-768

The workbench runs the same code on Windows x86-64 and macOS (Intel or Apple Silicon), records machine metadata, verifies each complete workload, runs known-answer tests where available, and displays comparisons within each workload category.

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

The first usable baseline includes:

- a native `egui`/`eframe` desktop interface (no browser or server required)
- background benchmark execution so the window remains responsive
- mean, median, P95, and operations-per-second measurements
- clear classical vs post-quantum labeling
- a versioned JSON format for comparing machines later

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

Each timing is a deliberately complete operation:

- X25519/X448/P-256: generate two ephemeral keypairs and compute both peers' shared secret.
- ML-KEM: keygen + encapsulate + decapsulate.
- AEAD: generate key and nonce, encrypt 64 KiB, decrypt it, and compare the plaintext.
- Hash: hash a fixed 1 MiB payload.
- HKDF: extract and expand the RFC 5869 test input and verify the result.
- Password KDF: derive 32 bytes using the recorded memory, time, and parallelism parameters.
- Signatures: keygen, sign a 1 KiB message, and verify it.
- Hybrids: complete the classical exchange and ML-KEM-768, then combine both secrets with a domain-separated HKDF.

Only compare results inside the same category. The global suite is not a ranking of unlike primitives. Password KDF parameters are research profiles, not deployment recommendations. Particularly expensive adapters may run fewer samples; every JSON result records its actual sample count and workload.

## Add an experimental algorithm

Implement `CryptoExperiment`, give it category and maturity metadata, define an honest complete workload, add a correctness path and preferably official vectors, then add it to `registry()`.

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the adapter boundary, evidence ladder, and rules that keep experiments separate from production-safe claims.

The intended future order is:

1. Official known-answer vectors
2. Cross-implementation checks
3. Wycheproof/adversarial vectors
4. Fuzz/property testing
5. Side-channel experiments
6. Microbenchmarks
7. Protocol/network benchmarks
8. Only then experimental curves/KEMs/hybrid logic

Draft and not-yet-implemented candidates are tracked in [docs/EXPLORATION.md](docs/EXPLORATION.md) and [exploration/candidates.json](exploration/candidates.json). Being listed is not an endorsement and does not make a candidate runnable.

## Next planned modules

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
