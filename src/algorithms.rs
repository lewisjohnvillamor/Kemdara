//! Registry metadata and adapters for complete, verified crypto workloads.

use ml_kem::{
    MlKem512, MlKem768, MlKem1024,
    kem::{Decapsulate, Encapsulate, Kem},
};
use p256::{ecdh::EphemeralSecret as P256Secret, elliptic_curve::Generate};
use x25519_dalek::{EphemeralSecret as X25519Secret, PublicKey as X25519PublicKey};

mod digest;
mod hybrid;
mod password;
mod payload;
mod protocol;
mod signatures;
mod tradeoffs;

use self::{
    digest::{
        ASCON_HASH256, ASCON_XOF128, BLAKE2S, BLAKE3, HKDF_SHA256, KANGAROO_TWELVE, SHA3_256,
        SHA256, SHA384,
    },
    hybrid::{P256_MLKEM768, X25519_MLKEM768},
    password::{ARGON2ID, SCRYPT},
    payload::{
        AES128_GCM, AES256_GCM, AES256_GCM_SIV, ASCON_AEAD128, CHACHA20_POLY1305,
        XCHACHA20_POLY1305,
    },
    protocol::{NOISE_NN, NOISE_XX},
    signatures::{ED25519, MLDSA44, MLDSA65, MLDSA87, P256_ECDSA, SLHDSA_SHAKE128F},
};

pub use tradeoffs::{TradeoffProfile, tradeoff_for};

use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ExperimentObservation {
    /// Bytes emitted by the complete measured workload, when the adapter can observe them.
    pub total_wire_bytes: Option<usize>,
    /// Application bytes carried inside the measured workload.
    pub application_payload_bytes: Option<usize>,
    /// A protocol-level trace. Primitive adapters leave this empty.
    pub protocol: Option<ProtocolTrace>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProtocolTrace {
    pub pattern: &'static str,
    pub handshake_messages: usize,
    pub transport_messages: usize,
    pub handshake_wire_bytes: usize,
    pub transport_wire_bytes: usize,
    pub application_payload_bytes: usize,
    pub total_wire_bytes: usize,
    pub expansion_bytes: usize,
    pub authentication: &'static str,
    pub forward_secrecy: &'static str,
    pub identity_exposure: &'static str,
    pub events: Vec<TranscriptEvent>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TranscriptEvent {
    pub flight: usize,
    pub direction: &'static str,
    pub phase: &'static str,
    pub tokens: &'static str,
    pub wire_bytes: usize,
    pub security_state: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExperimentCategory {
    KeyEstablishment,
    PayloadEncryption,
    Hash,
    KeyDerivation,
    PasswordDerivation,
    DigitalSignature,
    HybridKeyEstablishment,
    ProtocolHandshake,
}

impl ExperimentCategory {
    pub const fn label(self) -> &'static str {
        match self {
            Self::KeyEstablishment => "Key establishment",
            Self::PayloadEncryption => "Payload encryption",
            Self::Hash => "Hash",
            Self::KeyDerivation => "Key derivation",
            Self::PasswordDerivation => "Password derivation",
            Self::DigitalSignature => "Digital signature",
            Self::HybridKeyEstablishment => "Hybrid key establishment",
            Self::ProtocolHandshake => "Protocol handshake",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Maturity {
    Standardized,
    Interoperable,
    Experimental,
}

impl Maturity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standardized => "Standardized",
            Self::Interoperable => "Interoperable",
            Self::Experimental => "Experimental",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AlgorithmInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub family: &'static str,
    pub standard: &'static str,
    pub quantum_resistant: bool,
    pub category: ExperimentCategory,
    pub maturity: Maturity,
    pub workload: &'static str,
    /// Reduces iterations for unusually expensive full operations.
    pub iteration_divisor: usize,
    pub summary: &'static str,
}

pub trait CryptoExperiment: Sync {
    fn info(&self) -> AlgorithmInfo;

    /// Run the complete workload and perform its category-specific correctness check.
    fn run_once(&self) -> Result<(), String>;

    /// Collect deterministic, untimed metadata for a successful workload.
    ///
    /// Keeping observation outside the timing loop prevents transcript allocation and
    /// presentation bookkeeping from distorting the cryptographic measurement.
    fn observe(&self) -> Result<ExperimentObservation, String> {
        Ok(ExperimentObservation::default())
    }
}

/// Backward-compatible name for the original adapter boundary.
pub use CryptoExperiment as EstablishmentAlgorithm;

struct X25519Algorithm;
struct X448Algorithm;
struct P256Algorithm;
struct MlKem512Algorithm;
struct MlKem768Algorithm;
struct MlKem1024Algorithm;

static X25519: X25519Algorithm = X25519Algorithm;
static X448: X448Algorithm = X448Algorithm;
static P256: P256Algorithm = P256Algorithm;
static ML_KEM_512: MlKem512Algorithm = MlKem512Algorithm;
static ML_KEM_768: MlKem768Algorithm = MlKem768Algorithm;
static ML_KEM_1024: MlKem1024Algorithm = MlKem1024Algorithm;

pub fn registry() -> Vec<&'static dyn CryptoExperiment> {
    vec![
        &X25519,
        &X448,
        &P256,
        &ML_KEM_512,
        &ML_KEM_768,
        &ML_KEM_1024,
        &AES128_GCM,
        &AES256_GCM,
        &CHACHA20_POLY1305,
        &XCHACHA20_POLY1305,
        &AES256_GCM_SIV,
        &ASCON_AEAD128,
        &SHA256,
        &SHA384,
        &SHA3_256,
        &BLAKE2S,
        &BLAKE3,
        &ASCON_HASH256,
        &ASCON_XOF128,
        &KANGAROO_TWELVE,
        &HKDF_SHA256,
        &ARGON2ID,
        &SCRYPT,
        &ED25519,
        &P256_ECDSA,
        &MLDSA44,
        &MLDSA65,
        &MLDSA87,
        &SLHDSA_SHAKE128F,
        &X25519_MLKEM768,
        &P256_MLKEM768,
        &NOISE_NN,
        &NOISE_XX,
    ]
}

impl CryptoExperiment for X25519Algorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "x25519",
            name: "X25519",
            family: "Elliptic-curve Diffie-Hellman",
            standard: "RFC 7748",
            quantum_resistant: false,
            category: ExperimentCategory::KeyEstablishment,
            maturity: Maturity::Standardized,
            workload: "2 keypairs + 2 shared-secret derivations",
            iteration_divisor: 1,
            summary: "Fast ~128-bit classical security on Curve25519.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let alice_secret = X25519Secret::random();
        let alice_public = X25519PublicKey::from(&alice_secret);
        let bob_secret = X25519Secret::random();
        let bob_public = X25519PublicKey::from(&bob_secret);

        let alice_shared = alice_secret.diffie_hellman(&bob_public);
        let bob_shared = bob_secret.diffie_hellman(&alice_public);
        let shared = alice_shared.as_bytes();

        if shared != bob_shared.as_bytes() {
            return Err("peers derived different X25519 secrets".into());
        }
        if shared.iter().all(|byte| *byte == 0) {
            return Err("X25519 produced an all-zero shared secret".into());
        }
        Ok(())
    }
}

impl CryptoExperiment for X448Algorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "x448",
            name: "X448",
            family: "Elliptic-curve Diffie-Hellman",
            standard: "RFC 7748",
            quantum_resistant: false,
            category: ExperimentCategory::KeyEstablishment,
            maturity: Maturity::Standardized,
            workload: "2 keypairs + 2 shared-secret derivations",
            iteration_divisor: 1,
            summary: "Higher classical security margin at a larger performance cost.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let mut alice_secret = [0_u8; 56];
        let mut bob_secret = [0_u8; 56];
        getrandom::fill(&mut alice_secret).map_err(|error| error.to_string())?;
        getrandom::fill(&mut bob_secret).map_err(|error| error.to_string())?;

        let alice_public = crrl::x448::x448_base(&alice_secret);
        let bob_public = crrl::x448::x448_base(&bob_secret);
        let alice_shared = crrl::x448::x448(&bob_public, &alice_secret);
        let bob_shared = crrl::x448::x448(&alice_public, &bob_secret);

        if alice_shared != bob_shared {
            return Err("peers derived different X448 secrets".into());
        }
        if alice_shared.iter().all(|byte| *byte == 0) {
            return Err("X448 produced an all-zero shared secret".into());
        }
        Ok(())
    }
}

impl CryptoExperiment for P256Algorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "p256",
            name: "NIST P-256",
            family: "Elliptic-curve Diffie-Hellman",
            standard: "NIST SP 800-186",
            quantum_resistant: false,
            category: ExperimentCategory::KeyEstablishment,
            maturity: Maturity::Standardized,
            workload: "2 keypairs + 2 shared-secret derivations",
            iteration_divisor: 1,
            summary: "Widely interoperable standardized Weierstrass curve.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let alice_secret = P256Secret::generate();
        let alice_public = alice_secret.public_key();
        let bob_secret = P256Secret::generate();
        let bob_public = bob_secret.public_key();

        let alice_shared = alice_secret.diffie_hellman(&bob_public);
        let bob_shared = bob_secret.diffie_hellman(&alice_public);

        if alice_shared.raw_secret_bytes() != bob_shared.raw_secret_bytes() {
            return Err("peers derived different P-256 secrets".into());
        }
        Ok(())
    }
}

impl CryptoExperiment for MlKem768Algorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "ml-kem-768",
            name: "ML-KEM-768",
            family: "Module-lattice key encapsulation",
            standard: "FIPS 203",
            quantum_resistant: true,
            category: ExperimentCategory::KeyEstablishment,
            maturity: Maturity::Standardized,
            workload: "keygen + encapsulate + decapsulate",
            iteration_divisor: 1,
            summary: "NIST-standardized post-quantum key encapsulation.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let (decapsulation_key, encapsulation_key) = MlKem768::generate_keypair();
        let (ciphertext, sender_shared) = encapsulation_key.encapsulate();
        let receiver_shared = decapsulation_key.decapsulate(&ciphertext);

        if sender_shared != receiver_shared {
            return Err("ML-KEM encapsulation and decapsulation disagreed".into());
        }
        Ok(())
    }
}

impl CryptoExperiment for MlKem512Algorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "ml-kem-512",
            name: "ML-KEM-512",
            family: "Module-lattice key encapsulation",
            standard: "FIPS 203",
            quantum_resistant: true,
            category: ExperimentCategory::KeyEstablishment,
            maturity: Maturity::Standardized,
            workload: "keygen + encapsulate + decapsulate",
            iteration_divisor: 1,
            summary: "Smallest and fastest standardized ML-KEM parameter set.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let (decapsulation_key, encapsulation_key) = MlKem512::generate_keypair();
        let (ciphertext, sender_shared) = encapsulation_key.encapsulate();
        let receiver_shared = decapsulation_key.decapsulate(&ciphertext);

        if sender_shared != receiver_shared {
            return Err("ML-KEM-512 encapsulation and decapsulation disagreed".into());
        }
        Ok(())
    }
}

impl CryptoExperiment for MlKem1024Algorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "ml-kem-1024",
            name: "ML-KEM-1024",
            family: "Module-lattice key encapsulation",
            standard: "FIPS 203",
            quantum_resistant: true,
            category: ExperimentCategory::KeyEstablishment,
            maturity: Maturity::Standardized,
            workload: "keygen + encapsulate + decapsulate",
            iteration_divisor: 1,
            summary: "Largest standardized ML-KEM parameter set and security margin.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let (decapsulation_key, encapsulation_key) = MlKem1024::generate_keypair();
        let (ciphertext, sender_shared) = encapsulation_key.encapsulate();
        let receiver_shared = decapsulation_key.decapsulate(&ciphertext);

        if sender_shared != receiver_shared {
            return Err("ML-KEM-1024 encapsulation and decapsulation disagreed".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode<const N: usize>(value: &str) -> [u8; N] {
        let mut bytes = [0_u8; N];
        hex::decode_to_slice(value, &mut bytes).expect("valid test-vector hex");
        bytes
    }

    #[test]
    fn all_adapters_complete_a_verified_exchange() {
        for algorithm in registry() {
            algorithm
                .run_once()
                .unwrap_or_else(|error| panic!("{}: {error}", algorithm.info().name));
        }
    }

    #[test]
    fn x25519_matches_rfc_7748_vector() {
        let scalar = decode("a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4");
        let point = decode("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c");
        let expected = decode("c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552");

        assert_eq!(x25519_dalek::x25519(scalar, point), expected);
    }

    #[test]
    fn x448_matches_rfc_7748_vector() {
        let scalar = decode(concat!(
            "3d262fddf9ec8e88495266fea19a34d28882acef045104d0d1aae121",
            "700a779c984c24f8cdd78fbff44943eba368f54b29259a4f1c600ad3"
        ));
        let point = decode(concat!(
            "06fce640fa3487bfda5f6cf2d5263f8aad88334cbd07437f020f08f9",
            "814dc031ddbdc38c19c6da2583fa5429db94ada18aa7a7fb4ef8a086"
        ));
        let expected = decode(concat!(
            "ce3e4ff95a60dc6697da1db1d85e6afbdf79b50a2412d7546d5f239f",
            "e14fbaadeb445fc66a01b0779d98223961111e21766282f73dd96b6f"
        ));

        assert_eq!(crrl::x448::x448(&point, &scalar), expected);
    }
}
