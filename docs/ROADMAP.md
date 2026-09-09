# Product roadmap

Kemdara's defensible identity is not another algorithm leaderboard. It is an open-source cryptography tradeoff laboratory that connects reproducible local measurements, protocol behavior, explicit security properties, safe failures, and upstream evidence in one learner-facing interface.

## Delivery order

### P0 — trustworthy observation model

1. Record protocol message flights, directions, phases, measured wire bytes, and the security state reached after each flight.
2. Separate untimed observations from timed cryptographic work.
3. Track peak live heap delta, allocated bytes, and allocation count on the benchmark thread. Label the scope precisely; do not call it total process memory.
4. Version every JSON schema change and add serialization tests.

Acceptance: a learner can explain both the latency and byte cost of a scenario, and measurement overhead is not included in the reported crypto timing.

### P0.5 — distribution and first-run experience

1. Publish reproducible Windows and macOS application artifacts from tagged GitHub releases.
2. Offer three starting paths: **Learn** for a guided scenario, **Compare** for local/imported reports, and **Contribute** for adapters, evidence, or result bundles.
3. Ship a tiny deterministic sample report so the interface is useful before a long benchmark completes.
4. Explain local execution, data collection, and experimental labels before the first run.

Acceptance: a new user can reach one explained visualization without installing Rust or reading repository documentation.

### P1 — result lifecycle

1. Import two or more Kemdara JSON reports and align results by stable algorithm ID, schema, workload fingerprint, and build metadata.
2. Show absolute values and ratios without declaring a winner; flag incomparable workloads and material noise.
3. Export a portable result bundle containing the report, source revision, dependency lock digest, optional notes, and a human-readable summary.
4. Preview the exact machine metadata before sharing and make all publishing opt-in.
5. Store annotations separately from measurements. Require author/provenance, distinguish observation from hypothesis, and retain disagreements rather than overwriting them.

Acceptance: Windows and macOS reports can be compared safely, a shared bundle reproduces its context, and commentary cannot silently mutate raw results.

### P1 — interactive protocol laboratory

1. Add a scenario composer for supported combinations of protocol pattern, payload size, authentication goal, and simulated network conditions.
2. Add a safe failure laboratory with fixed demonstrations for corruption, replay, nonce reuse, truncation, reordering, wrong identity, and downgrade attempts.
3. Show the expected defense, observed result, and limit of the demonstration. Do not expose arbitrary packet injection or imply that rejection proves security.

Acceptance: changing one input creates a reproducible scenario record, and every failure demonstration explains what property was tested.

### P2 — external evidence

1. Import upstream evidence by pinned source/version: Wycheproof cases, Noise Explorer properties, liboqs results/metadata, and SUPERCOP data.
2. Keep external results visually distinct from local measurements and preserve provenance.

Acceptance: every external claim links to its exact upstream source and version.

### P3 — broader protocols and contributor surface

Add Noise NK/IK, HPKE, TLS 1.3, and then narrowly scoped MLS scenarios. Stabilize the custom adapter template only after the observation and evidence contracts settle.

## Non-goals

- A universal score or “best crypto” badge.
- Reimplementing primitives already maintained by specialist projects.
- Treating successful tests, benchmarks, or imported evidence as an audit.
- Shipping a generic production protocol or key-management system.
