# Ecosystem map and Kemdara's gap

Kemdara should integrate existing work rather than recreate mature cryptographic code or formal-analysis engines.

| Existing project | What it already does well | What Kemdara should reuse | Learner-facing gap Kemdara can fill |
| --- | --- | --- | --- |
| SUPERCOP / eBACS | Broad unified primitive benchmarking across implementations and machines | Workload definitions and optional result import | Dense expert-oriented data does not explain product-level tradeoffs or protocol composition |
| Open Quantum Safe / liboqs | Common APIs, tests, benchmarks, and protocol prototypes for post-quantum algorithms | PQ implementations, metadata, and comparison baselines | Primarily PQ-focused rather than a guided classical/PQ/password/protocol map |
| pqm4 | Reproducible PQ benchmarking on Cortex-M4 microcontrollers | Constrained-device reference results | Specialized hardware setup is a barrier for ordinary desktop learners |
| Project Wycheproof | JSON vectors covering known attacks and tricky implementation behavior | Negative-vector importer | It validates behavior but is not a performance or tradeoff explorer |
| Noise Explorer | Formal models and security results for Noise handshake patterns | Link/import pattern properties rather than reimplement formal verification | It does not compare local implementation latency, variance, wire size, and machine effects |
| RustCrypto and specialist libraries | Maintained Rust implementations and per-crate benchmarks | Pin and wrap implementations instead of rewriting primitives | Results are spread across repositories and use different workload definitions |
| Criterion.rs | Statistical benchmarking and regression detection | Confidence intervals, warm-up, outlier analysis, and saved baselines | It is a benchmark engine rather than a cryptography curriculum |

## Kemdara's opportunity

Kemdara is the integration and explanation layer:

1. One cross-platform desktop application and JSON schema.
2. Complete, correctness-checked workloads grouped by comparable purpose.
3. A visible evidence label separating standards, interoperability, and experiments.
4. Per-result strength, cost, best-fit, and caveat cards.
5. CPU latency and throughput alongside variance, memory, artifact size, and wire bytes.
6. Primitive experiments and whole-protocol scenarios without pretending they are the same measurement.
7. A documented adapter path for learners to add a construction or implementation.
8. Links/importers to upstream vectors, formal analyses, and benchmark datasets.

## Weak points to avoid becoming

- A giant checklist of algorithms with dishonest apples-to-oranges rankings.
- A new cryptographic implementation library competing with reviewed specialists.
- A single-machine leaderboard presented as universal performance.
- A GUI that hides workload definitions, parameter sets, build flags, or sample counts.
- A “secure” badge inferred from round-trip correctness.
- A plugin system that downloads and executes arbitrary third-party native code without an explicit trust boundary.

The curated core should stay small enough that every executable entry has a clear hypothesis, trustworthy source, repeatable workload, and educational explanation.
