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
| Classic McEliece | Extremely large public keys but a long-studied code-based security assumption | Research candidate; not a Kemdara executable | Require a maintained safe Rust implementation and independent vectors |
| FrodoKEM | Conservative plain-LWE design without structured lattices | Research candidate; not standardized by NIST | Require stable implementation, vectors, and a clear interoperability target |
| NTRU Prime / sntrup761 | Different lattice structure and real-world hybrid deployment history | Protocol-specific deployments; not a Kemdara executable | Import authoritative vectors and benchmark the exact protocol construction |
| BIKE | Compact quasi-cyclic code-based KEM with a very different failure/performance profile | Research candidate | Require current cryptanalytic review and stable constant-time implementation |
| FROST | Threshold Schnorr signatures expose coordination and multi-party latency tradeoffs | Standardized protocol, not a single primitive timing | Add a protocol runner with network/round metrics rather than a microbenchmark adapter |
| OPAQUE | Password-authenticated key exchange can be more useful than comparing password hashes alone | Standardized protocol, not a single primitive timing | Add registration/login transcripts and active-attack negative tests |
| Verifiable delay functions | Deliberately sequential work would invert Kemdara's normal “faster is better” view | Active research family | Select a precise construction, threat model, and independently verifiable implementation |

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

## Unconventional executable baselines

- Ascon-AEAD128, Ascon-Hash256, and Ascon-XOF128 expose the constrained-device design space standardized in NIST SP 800-232.
- AES-256-GCM-SIV measures the cost of limiting damage from accidental nonce reuse.
- KangarooTwelve explores a parallel tree XOF built from reduced-round Keccak.
- Argon2id and scrypt expose memory cost as a first-class benchmark dimension. Their Kemdara parameters are fixed research profiles, not password-policy advice.
