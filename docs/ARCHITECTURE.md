# Kemdara architecture

Kemdara is a local measurement harness, not a new cryptographic protocol. Its first job is to make experiments repeatable and falsifiable on ordinary Windows and macOS computers.

## Boundaries

```text
Native GUI / JSON CLI
          |
Benchmark runner + machine metadata
          |
CryptoExperiment adapter
          |
Auditable third-party implementation
```

The UI never calls a cryptographic crate directly. Every runnable entry implements one small adapter with category, maturity, workload metadata, and a complete self-checking operation. This keeps presentation, timing, and primitive-specific code separate. An adapter may also return an untimed `ExperimentObservation`; protocol adapters use it for actual message lengths and transcript events so visualization bookkeeping cannot distort latency samples.

## What one sample measures

- X25519, X448, and P-256: generate two ephemeral keypairs, calculate both shared secrets, and compare them.
- ML-KEM-512/768/1024: generate a keypair, encapsulate, decapsulate, and compare the shared secrets.
- AEAD: generate a key and nonce, encrypt and decrypt 64 KiB, and compare the recovered plaintext.
- Hash: process a fixed 1 MiB payload; known-answer checks remain separate tests.
- HKDF: perform RFC 5869 extract/expand and compare the output with the official test case.
- Password KDFs: derive a key with fixed, recorded memory/time parameters and validate official vectors in tests.
- Signatures: generate a keypair, sign 1 KiB, and verify the signature.
- Experimental hybrids: complete both key-establishment components, combine them with domain-separated HKDF, and compare both parties' output.
- Protocol handshakes: complete every handshake flight, enter transport mode, exchange encrypted 1 KiB payloads in both directions, and compare both plaintexts.

Operations from different categories are not comparable. Results only describe this machine, build, dependency set, and measurement definition. Every JSON result records the category, maturity, workload, actual sample count, and a learner-facing tradeoff profile.

## Evidence ladder

A candidate moves upward only when the earlier evidence exists:

1. It compiles behind an isolated adapter.
2. The category-specific correctness check passes in repeated runs.
3. Official known-answer vectors pass.
4. A second independent implementation agrees.
5. Adversarial and malformed-input vectors pass.
6. Property tests and fuzzing find no invariant violations.
7. Side-channel experiments show no obvious leakage.
8. Microbenchmarks are stable across runs and machines.
9. Protocol-level experiments include bandwidth, latency, failure, and downgrade behavior.

Passing the ladder is research evidence, not a claim that a custom construction is production-safe.

## Adding a candidate

1. Prefer a maintained implementation with a clear license and published specification.
2. Add a zero-sized adapter in the relevant `src/algorithms/` module.
3. Return static metadata from `info()`.
4. Make `run_once()` execute a complete, documented workload and verify its result.
5. Add at least one official known-answer test when vectors exist.
6. Register the adapter in `registry()`.
7. Record the workload, maturity, and any iteration divisor in metadata.

Catalog-only candidates live in `exploration/candidates.json`; the executable never loads this file. Promotion into `registry()` requires the admission gate in `docs/EXPLORATION.md`.

Do not mix experimental outputs into production traffic, do not invent performance claims from one machine, and do not call a construction secure because it passes correctness tests.

The separate production-track boundary and admission gates are described in [PRODUCTION_PROTOCOL.md](PRODUCTION_PROTOCOL.md).
