# Exploration queue

Kemdara separates three maturity levels in code and JSON: `standardized`, `interoperable`, and `experimental`. A candidate in this document is **not executable** until it passes the admission gate below.

## Runnable experiments

The X25519 + ML-KEM-768 and P-256 + ML-KEM-768 adapters measure a local research combiner. Both complete the classical and post-quantum exchanges, concatenate those shared secrets in a fixed order, and feed them to domain-separated HKDF-SHA-256.

These adapters are not X-Wing, TLS, SSH, HPKE, or another interoperable protocol. Their output must not protect production traffic.

## Candidates worth evaluating next

| Candidate | Why it fits Kemdara | Current boundary | Next evidence needed |
| --- | --- | --- | --- |
| X-Wing | A specified X25519 + ML-KEM-768 hybrid KEM and a natural successor to the local combiner | CFRG Internet-Draft; not implemented here | Track the draft, import official vectors, and cross-check a maintained implementation |
| HQC | Different code-based assumption from ML-KEM and selected by NIST as a backup KEM | Selected for standardization; final standard pending | Wait for the final NIST specification and stable reviewed implementations |
| HAWK | Compact lattice signature with different tradeoffs from ML-DSA | NIST additional-signature candidate | Follow NIST evaluation, implementation maturity, and constant-time review |
| FAEST | Signature based on symmetric primitives, useful for assumption diversity | NIST additional-signature candidate | Follow NIST evaluation and obtain cross-implementation vectors |
| MAYO / UOV / QR-UOV | Multivariate signature families provide assumption diversity | NIST additional-signature candidates | Require cryptanalytic stability and maintained constant-time implementations |
| SQIsign | Very compact isogeny-based signatures make an interesting size/CPU tradeoff | NIST additional-signature candidate | Require evaluation progress, robust implementations, and adversarial vectors |
| SDitH / SNOVA / MQOM | Broaden the signature design space for comparative research | NIST additional-signature candidates | Require stable specs, vectors, and independent implementations |

Primary status references:

- NIST post-quantum project: <https://csrc.nist.gov/projects/post-quantum-cryptography>
- NIST HQC selection: <https://www.nist.gov/news-events/news/2025/03/nist-selects-hqc-fifth-algorithm-post-quantum-encryption>
- NIST additional signature candidates: <https://www.nist.gov/news-events/news/2026/05/nist-advances-additional-digital-signature-algorithms-next-round>
- CFRG X-Wing draft: <https://datatracker.ietf.org/doc/draft-connolly-cfrg-xwing-kem/>

## Admission gate

A new executable candidate needs all of the following:

1. A version-pinned public specification and unambiguous parameter set.
2. Official or project-authorized known-answer vectors.
3. A maintained implementation with an acceptable license and no unresolved critical advisory.
4. At least one independent implementation for differential testing.
5. A category-specific correctness check and negative test.
6. Explicit `experimental` metadata and a non-production warning.
7. Reproducible CI on Windows, macOS, and Linux before benchmark numbers are published.

Kemdara composes reviewed primitives for experiments; it does not invent new curves, permutation functions, or encryption modes.
