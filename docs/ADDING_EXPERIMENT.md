# Add an experiment

Kemdara is deliberately open to new implementations, including a learner's own construction. The common harness makes measurements comparable and failures visible; it does not certify that a new construction is secure.

## Decide what is being measured

Choose exactly one category and a complete operation. Examples include keygen plus encapsulate plus decapsulate for a KEM, or encrypt plus decrypt plus plaintext verification for an AEAD. Do not compare a single primitive call against another adapter's complete protocol operation.

Record:

- exact algorithm and parameter-set name;
- specification/version, or `original experiment` when none exists;
- input/output sizes and workload steps;
- security property being explored;
- expected strength, cost, best fit, and caveat;
- implementation source, version, license, and evidence level.

## Adapter template

Create a module under `src/algorithms/` and implement the shared trait:

```rust
use crate::algorithms::{
    AlgorithmInfo, CryptoExperiment, ExperimentCategory, Maturity,
};

pub struct MyExperiment;

impl CryptoExperiment for MyExperiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "my-experiment-v1",
            name: "My experiment v1",
            family: "Original experiment",
            standard: "No public standard",
            quantum_resistant: false,
            category: ExperimentCategory::Hash,
            maturity: Maturity::Experimental,
            workload: "process a fixed 1 MiB payload",
            iteration_divisor: 1,
            summary: "The exact hypothesis this construction explores.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        // Execute the same complete, deterministic workload on every sample.
        // Verify all outputs or invariants before returning Ok(()).
        todo!()
    }
}
```

Add a precise entry to `tradeoff_for()` in `src/algorithms/tradeoffs.rs`, register the static adapter in `registry()`, and add tests.

## Evidence labels

- `standardized`: the named construction/parameter set has a final public standard.
- `interoperable`: a stable public specification and multiple compatible implementations exist.
- `experimental`: original, draft, non-interoperable, or implementation-assurance-limited work.

Original algorithms must remain `experimental`, even when every correctness test passes.

## Minimum tests

1. A successful round-trip or invariant test.
2. A changed-input or corrupted-output negative test.
3. Official known-answer vectors when they exist.
4. Deterministic test fixtures where the specification permits them.
5. At least two samples in noise-sensitive local runs.
6. `cargo fmt`, Clippy with warnings denied, unit tests, and a release build on CI.

When a candidate becomes important, add differential testing against an independent implementation and import relevant Wycheproof-style adversarial vectors. These strengthen evidence but still do not turn Kemdara into a certification authority.
