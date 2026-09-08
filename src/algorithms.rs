//! Small adapters around established cryptographic implementations.
//!
//! Kemdara deliberately keeps the adapter boundary narrow: one complete key
//! establishment per call, with both parties' results compared before success.

use ml_kem::{
    MlKem768,
    kem::{Decapsulate, Encapsulate, Kem},
};
use p256::{ecdh::EphemeralSecret as P256Secret, elliptic_curve::Generate};
use x25519_dalek::{EphemeralSecret as X25519Secret, PublicKey as X25519PublicKey};

#[derive(Clone, Copy, Debug)]
pub struct AlgorithmInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub family: &'static str,
    pub standard: &'static str,
    pub quantum_resistant: bool,
    pub summary: &'static str,
}

pub trait EstablishmentAlgorithm: Sync {
    fn info(&self) -> AlgorithmInfo;

    /// Run a complete two-party establishment and verify that both sides agree.
    fn run_once(&self) -> Result<(), String>;
}

struct X25519Algorithm;
struct X448Algorithm;
struct P256Algorithm;
struct MlKem768Algorithm;

static X25519: X25519Algorithm = X25519Algorithm;
static X448: X448Algorithm = X448Algorithm;
static P256: P256Algorithm = P256Algorithm;
static ML_KEM_768: MlKem768Algorithm = MlKem768Algorithm;

pub fn registry() -> [&'static dyn EstablishmentAlgorithm; 4] {
    [&X25519, &X448, &P256, &ML_KEM_768]
}

impl EstablishmentAlgorithm for X25519Algorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "x25519",
            name: "X25519",
            family: "Elliptic-curve Diffie-Hellman",
            standard: "RFC 7748",
            quantum_resistant: false,
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

impl EstablishmentAlgorithm for X448Algorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "x448",
            name: "X448",
            family: "Elliptic-curve Diffie-Hellman",
            standard: "RFC 7748",
            quantum_resistant: false,
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

impl EstablishmentAlgorithm for P256Algorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "p256",
            name: "NIST P-256",
            family: "Elliptic-curve Diffie-Hellman",
            standard: "NIST SP 800-186",
            quantum_resistant: false,
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

impl EstablishmentAlgorithm for MlKem768Algorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "ml-kem-768",
            name: "ML-KEM-768",
            family: "Module-lattice key encapsulation",
            standard: "FIPS 203",
            quantum_resistant: true,
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
