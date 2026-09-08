# Kemdara architecture

Kemdara is a local measurement harness, not a new cryptographic protocol. Its first job is to make experiments repeatable and falsifiable on ordinary Windows and macOS computers.

## Boundaries

```text
Native GUI / JSON CLI
          |
Benchmark runner + machine metadata
          |
EstablishmentAlgorithm adapter
          |
Auditable third-party implementation
```

The UI never calls a cryptographic crate directly. Every candidate implements one small adapter with metadata and a complete, self-checking key-establishment operation. This keeps presentation, timing, and primitive-specific code separate.

## What one sample measures

- X25519, X448, and P-256: generate two ephemeral keypairs, calculate both shared secrets, and compare them.
- ML-KEM-768: generate a keypair, encapsulate, decapsulate, and compare the shared secrets.

These operations solve similar key-establishment problems but are not interchangeable. Results only describe this machine, build, dependency set, and measurement definition.

## Evidence ladder

A candidate moves upward only when the earlier evidence exists:

1. It compiles behind an isolated adapter.
2. Both peers agree in repeated randomized runs.
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
2. Add a zero-sized adapter in `src/algorithms.rs`.
3. Return static metadata from `info()`.
4. Make `run_once()` generate fresh inputs and verify both outputs.
5. Add at least one official known-answer test when vectors exist.
6. Register the adapter in `registry()`.
7. Record any non-equivalent work included in its benchmark.

Do not mix experimental outputs into production traffic, do not invent performance claims from one machine, and do not call a construction secure because it passes correctness tests.
