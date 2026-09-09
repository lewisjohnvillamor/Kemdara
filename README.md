# Kemdara

**Run it. Break it. Measure it. Understand what every cryptographic choice buys you.**

A local, cross-platform cryptography experimentation workbench for **learning, verification, and benchmarking**. Kemdara runs real implementations and production-shaped protocol scenarios, but it is an educational lab rather than a production security claim. The protocol roadmap is defined in [docs/PRODUCTION_PROTOCOL.md](docs/PRODUCTION_PROTOCOL.md).

Kemdara does not answer “which algorithm is best?” Every choice has a workload, security goal, compatibility boundary, and cost. Results are grouped only where the operations are meaningfully comparable, and each result includes an explicit strength, cost, best fit, and caveat.

Its intended niche is a **cryptography tradeoff laboratory**: the breadth of specialist suites, the approachability of an interactive teaching tool, and enough protocol context to show what a primitive actually buys when it becomes part of a system. Kemdara reuses established implementations and evidence; it does not compete with them or invent a new production wire protocol.

## Why use Kemdara?

- **Students** can move from “this algorithm is fast” to seeing the operation, message flow, byte cost, security properties, and failure behavior that make the number meaningful.
- **Engineers** can reproduce the same workload on their own hardware, inspect timing noise, and compare deployment-shaped tradeoffs without relying on a universal ranking.
- **Researchers and implementers** can add an isolated adapter, attach evidence and caveats, and let other people reproduce an observation without merging a new cryptographic primitive into Kemdara itself.

The desired first-run experience is: download a packaged app, choose **Learn**, **Compare**, or **Contribute**, run a guided sample, and export a self-describing result bundle. The repository is not there yet—source builds and JSON export work today; packaged releases, import, annotations, and guided modes are roadmap items. That limitation is explicit so early contributors know where help is valuable.

Executable adapters are grouped by workload:

- Key establishment: X25519, X448, P-256 ECDH, and ML-KEM-512/768/1024
- Payload encryption: AES-GCM, ChaCha20-Poly1305, XChaCha20-Poly1305, AES-256-GCM-SIV, and Ascon-AEAD128
- Hash/XOF/KDF: SHA-2, SHA-3, BLAKE2s, BLAKE3, Ascon-Hash256/XOF128, KangarooTwelve, and HKDF-SHA-256
- Password derivation: Argon2id and scrypt with explicit memory/work parameters
- Signatures: Ed25519, ECDSA P-256, ML-DSA-44/65/87, and SLH-DSA-SHAKE-128f
- Experimental hybrids: X25519 + ML-KEM-768 and P-256 + ML-KEM-768
- Protocol scenarios: Noise NN and Noise XX with a complete handshake and two-way encrypted transport exchange

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

Kemdara currently has a native desktop GUI, not a hosted web interface. A browser build is possible later, but native execution is the trustworthy baseline for local CPU measurements because it avoids browser scheduling and WebAssembly differences.

The first usable baseline includes:

- a native `egui`/`eframe` desktop interface (no browser or server required)
- background benchmark execution so the window remains responsive
- mean, median, P95, min/max, standard deviation, coefficient-of-variation noise, and operations-per-second measurements
- category-normalized latency bars with median/P95 markers and detailed hover readouts
- expandable learner-facing tradeoff cards for every runnable experiment
- expandable Noise flight timelines with measured bytes and security state after each message
- measured handshake, transport, payload, expansion, and total wire bytes in JSON
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

The contributor walkthrough and adapter template are in [docs/ADDING_EXPERIMENT.md](docs/ADDING_EXPERIMENT.md). A custom algorithm is always labeled experimental; passing the common runner shows correctness and measurement behavior, not security.

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

## Reuse rather than recreate

Kemdara is a presentation and integration workbench, not a replacement for specialist projects. Planned importers and adapters should reuse:

- SUPERCOP/eBACS for broad implementation-performance data;
- liboqs and pqm4 for post-quantum implementations and constrained targets;
- Project Wycheproof and official standards vectors for negative/correctness testing;
- Noise Explorer for formal handshake-pattern results;
- Criterion-style statistical methods for stable local measurements.

See [docs/ECOSYSTEM_GAPS.md](docs/ECOSYSTEM_GAPS.md) for the gap Kemdara is intended to fill.

## Roadmap and priority

The ordering is deliberate: trustworthy measurements come before controls and comparisons that depend on them.

| Priority | Module | Why it comes here | Status |
| --- | --- | --- | --- |
| P0 | Observation contract: transcript and wire size | Makes protocol costs inspectable and gives later modules one versioned data model | First Noise slice implemented |
| P0 | Peak tracked heap and allocation counts | Adds a reproducible memory view without pretending noisy whole-process RSS is algorithm memory | Next |
| P0.5 | Packaged apps and guided first run | Removes the Rust-toolchain barrier and gives each audience an obvious starting path | Planned |
| P1 | Result import, comparison, sharing, and annotations | Turns isolated runs into reproducible conversations while preserving provenance and caveats | Planned |
| P1 | Scenario composer and safe failure lab | Lets learners vary supported inputs and observe fixed corruption, replay, nonce, and downgrade demonstrations | Planned |
| P2 | Upstream evidence importers | Links local observations to Wycheproof, Noise Explorer, liboqs, and SUPERCOP without duplicating their work | Planned |
| P3 | More protocol scenarios | Noise NK/IK, HPKE, TLS 1.3, then carefully scoped MLS scenarios | Planned |

Two cross-cutting requirements apply to every phase: the GUI and JSON must show the exact workload, and failure/correctness evidence must never be presented as a security proof. See [docs/ROADMAP.md](docs/ROADMAP.md) for acceptance criteria.

## Contributing results

Today, contributors can generate a versioned JSON report with the headless command and attach it to a GitHub issue or research note. Kemdara does not yet maintain a canonical public result corpus, because accepting numbers without workload compatibility, provenance, build metadata, and noise checks would create a misleading leaderboard.

The planned contribution flow validates a report locally, previews exactly what machine metadata will be shared, compares only compatible workload fingerprints, and stores commentary as a separate attributed annotation rather than changing the measurement. See [docs/RESULTS_AND_ANNOTATIONS.md](docs/RESULTS_AND_ANNOTATIONS.md).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Kemdara by you will be dual-licensed as above, without additional terms or conditions.
