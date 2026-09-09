use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct TradeoffProfile {
    pub strength: &'static str,
    pub cost: &'static str,
    pub best_fit: &'static str,
    pub caveat: &'static str,
}

pub fn tradeoff_for(id: &str) -> TradeoffProfile {
    match id {
        "x25519" => profile(
            "Compact keys and strong software performance",
            "Provides classical rather than post-quantum security",
            "General-purpose ephemeral key agreement",
            "Public keys still require authentication by a protocol",
        ),
        "x448" => profile(
            "Higher classical security margin than X25519",
            "Larger keys and substantially more CPU time",
            "Systems deliberately targeting a larger classical margin",
            "Extra cost is often unnecessary for ordinary 128-bit targets",
        ),
        "p256" => profile(
            "Very broad standards, hardware, certificate, and protocol support",
            "More complex curve encoding and generally slower software operations",
            "Compatibility-heavy PKI and regulated environments",
            "Use strict point validation and a maintained implementation",
        ),
        "ml-kem-512" | "ml-kem-768" | "ml-kem-1024" => profile(
            "Standardized post-quantum key establishment",
            "Larger public keys and ciphertexts than classical elliptic curves",
            "Post-quantum migration experiments",
            "Parameter levels trade bandwidth and CPU for security margin",
        ),
        "aes-128-gcm" | "aes-256-gcm" => profile(
            "Extremely fast on CPUs with AES and carry-less-multiply acceleration",
            "Catastrophic security loss is possible if a nonce repeats",
            "High-throughput standardized transport encryption",
            "Nonce allocation is a protocol responsibility, not an optional detail",
        ),
        "chacha20-poly1305" => profile(
            "Consistent fast software performance without AES hardware",
            "Can lose to hardware-accelerated AES on desktop and server CPUs",
            "Mobile, embedded, and portable software implementations",
            "The standard nonce must still be unique for each key",
        ),
        "xchacha20-poly1305" => profile(
            "Large nonces make safe randomized nonce generation practical",
            "Less universal protocol and compliance support than IETF ChaCha20-Poly1305",
            "File, storage, and application-level encryption",
            "Interoperable in major libraries but not itself an RFC cipher suite",
        ),
        "aes-256-gcm-siv" => profile(
            "Limits damage from accidental nonce reuse",
            "Different construction and interoperability footprint from ordinary GCM",
            "Systems where perfect nonce discipline is difficult",
            "Misuse resistance reduces risk; it does not make unlimited reuse safe",
        ),
        "ascon-aead128" => profile(
            "Small permutation-based design standardized for constrained devices",
            "Often slower than AES-NI or ChaCha on powerful desktop CPUs",
            "Constrained hardware and implementation-diversity studies",
            "Desktop throughput is not its primary optimization target",
        ),
        "sha-256" | "sha-384" => profile(
            "Mature, standardized, and widely accelerated",
            "Sequential design offers less parallel scaling than tree hashes",
            "Interoperable protocols and general hashing",
            "SHA-384 changes output and internal structure, not merely iteration count",
        ),
        "sha3-256" => profile(
            "Different sponge construction from SHA-2",
            "Common software implementations are slower than SHA-2 or BLAKE3",
            "Standards requiring SHA-3 and design-diversity experiments",
            "A different assumption base may matter more than raw throughput",
        ),
        "blake2s-256" => profile(
            "Compact and efficient on smaller-word platforms",
            "Less protocol and compliance adoption than SHA-2",
            "Embedded software and BLAKE-family comparisons",
            "Choose BLAKE2b instead when the target and protocol call for it",
        ),
        "blake3" => profile(
            "High throughput, tree parallelism, streaming, and XOF support",
            "Not a NIST or IETF general-purpose hash standard",
            "Content hashing and high-throughput application workloads",
            "Protocol compatibility may outweigh its speed advantage",
        ),
        "ascon-hash256" | "ascon-xof128" => profile(
            "Compact standardized permutation family for constrained targets",
            "Low desktop throughput in the current pure-Rust implementation",
            "Small-device and shared-permutation design studies",
            "Compare code size and energy before judging it by desktop latency",
        ),
        "kangaroo-twelve-128" => profile(
            "Tree hashing and parallelism for long inputs",
            "Smaller interoperability footprint than SHA-2/SHA-3",
            "Large-content hashing and Keccak-family exploration",
            "Short-message results may not reveal its tree-mode advantage",
        ),
        "hkdf-sha-256" => profile(
            "Simple domain-separated extraction and expansion",
            "Not memory-hard and therefore unsuitable for password storage",
            "Deriving protocol keys from high-entropy secret material",
            "Context strings and output separation must be designed deliberately",
        ),
        "argon2id-19mib-t2-p1" => profile(
            "Memory-hard password derivation with balanced side-channel resistance",
            "Intentionally consumes noticeable memory and time",
            "Password hashing and password-derived encryption keys",
            "Parameters must be tuned for the actual deployment and threat model",
        ),
        "scrypt-ln15-r8-p1" => profile(
            "Mature memory-hard baseline with wide implementation availability",
            "Can be slower or less flexible than modern Argon2id profiles",
            "Compatibility and comparative password-KDF studies",
            "Kemdara's parameters are measurements, not password-policy advice",
        ),
        "ed25519" => profile(
            "Compact deterministic signatures with strong software ergonomics",
            "No post-quantum security",
            "Modern application signatures and identity keys",
            "Protocol encoding and key validation still matter",
        ),
        "ecdsa-p256-sha256" => profile(
            "Deep PKI, hardware-token, and standards interoperability",
            "Signing nonce failures can expose the private key",
            "Existing certificate and hardware ecosystems",
            "Never implement nonce generation or curve arithmetic ad hoc",
        ),
        "ml-dsa-44" | "ml-dsa-65" | "ml-dsa-87" => profile(
            "Standardized post-quantum signatures with multiple security levels",
            "Larger keys/signatures and naturally variable signing work",
            "Post-quantum authentication and migration experiments",
            "Report distributions; a mean hides rejection-sampling variation",
        ),
        "slh-dsa-shake-128f" => profile(
            "Conservative hash-based post-quantum security assumptions",
            "Very large signatures and expensive signing",
            "Assumption-diverse fallback and long-lived verification contexts",
            "The fast parameter set is still far slower than lattice signatures",
        ),
        "hybrid-x25519-ml-kem-768" | "hybrid-p256-ml-kem-768" => profile(
            "Hedges across classical and post-quantum components",
            "Adds CPU, bandwidth, implementation, and negotiation complexity",
            "Researching migration and failure behavior",
            "Kemdara's combiner is not an interoperable protocol construction",
        ),
        "noise-nn-25519-chachapoly-blake2s" => profile(
            "Two-message forward-secret channel with minimal setup",
            "Does not authenticate either peer",
            "Ephemeral sessions where authentication exists in another layer",
            "An active attacker can impersonate a peer unless the channel is bound externally",
        ),
        "noise-xx-25519-chachapoly-blake2s" => profile(
            "Generic mutual authentication without pre-known static public keys",
            "Requires a third handshake message and an application trust decision",
            "Two-party channels that exchange static identities during setup",
            "This adapter uses an unaudited library and is educational only",
        ),
        _ => profile(
            "Provides a distinct cryptographic design point",
            "Its costs depend on the exact workload and target machine",
            "Controlled comparison within the same workload category",
            "A benchmark result is neither a security proof nor a deployment recommendation",
        ),
    }
}

const fn profile(
    strength: &'static str,
    cost: &'static str,
    best_fit: &'static str,
    caveat: &'static str,
) -> TradeoffProfile {
    TradeoffProfile {
        strength,
        cost,
        best_fit,
        caveat,
    }
}
