use hkdf::Hkdf;
use ml_kem::{
    MlKem768,
    kem::{Decapsulate, Encapsulate, Kem},
};
use p256::{ecdh::EphemeralSecret as P256Secret, elliptic_curve::Generate};
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret as X25519Secret, PublicKey as X25519PublicKey};

use super::{AlgorithmInfo, CryptoExperiment, ExperimentCategory, Maturity};

const LABEL: &[u8] = b"Kemdara experimental hybrid combiner v1";
const WORKLOAD: &str = "classical exchange + ML-KEM-768 + HKDF-SHA-256 combiner";

pub(super) struct X25519MlKem768Experiment;
pub(super) struct P256MlKem768Experiment;

pub(super) static X25519_MLKEM768: X25519MlKem768Experiment = X25519MlKem768Experiment;
pub(super) static P256_MLKEM768: P256MlKem768Experiment = P256MlKem768Experiment;

fn combine(classical: &[u8], post_quantum: &[u8]) -> Result<[u8; 32], String> {
    let mut input = Vec::with_capacity(classical.len() + post_quantum.len());
    input.extend_from_slice(classical);
    input.extend_from_slice(post_quantum);
    let hkdf = Hkdf::<Sha256>::new(None, &input);
    let mut output = [0_u8; 32];
    hkdf.expand(LABEL, &mut output).map_err(|_| "hybrid HKDF expansion failed")?;
    Ok(output)
}

fn ml_kem_exchange() -> (Vec<u8>, Vec<u8>) {
    let (decapsulation_key, encapsulation_key) = MlKem768::generate_keypair();
    let (ciphertext, sender_shared) = encapsulation_key.encapsulate();
    let receiver_shared = decapsulation_key.decapsulate(&ciphertext);
    (sender_shared.as_slice().to_vec(), receiver_shared.as_slice().to_vec())
}

impl CryptoExperiment for X25519MlKem768Experiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "hybrid-x25519-ml-kem-768",
            name: "X25519 + ML-KEM-768",
            family: "Hybrid key establishment",
            standard: "Kemdara experiment (not protocol compatible)",
            quantum_resistant: true,
            category: ExperimentCategory::HybridKeyEstablishment,
            maturity: Maturity::Experimental,
            workload: WORKLOAD,
            iteration_divisor: 1,
            summary: "Research combiner retaining a classical and a post-quantum component.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let alice_secret = X25519Secret::random();
        let alice_public = X25519PublicKey::from(&alice_secret);
        let bob_secret = X25519Secret::random();
        let bob_public = X25519PublicKey::from(&bob_secret);
        let alice_classical = alice_secret.diffie_hellman(&bob_public);
        let bob_classical = bob_secret.diffie_hellman(&alice_public);
        let (sender_pq, receiver_pq) = ml_kem_exchange();
        let alice = combine(alice_classical.as_bytes(), &sender_pq)?;
        let bob = combine(bob_classical.as_bytes(), &receiver_pq)?;
        if alice == bob {
            Ok(())
        } else {
            Err("X25519 + ML-KEM-768 hybrid secrets disagreed".into())
        }
    }
}

impl CryptoExperiment for P256MlKem768Experiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "hybrid-p256-ml-kem-768",
            name: "P-256 + ML-KEM-768",
            family: "Hybrid key establishment",
            standard: "Kemdara experiment (not protocol compatible)",
            quantum_resistant: true,
            category: ExperimentCategory::HybridKeyEstablishment,
            maturity: Maturity::Experimental,
            workload: WORKLOAD,
            iteration_divisor: 1,
            summary: "Research combiner pairing a widely deployed curve with ML-KEM-768.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let alice_secret = P256Secret::generate();
        let alice_public = alice_secret.public_key();
        let bob_secret = P256Secret::generate();
        let bob_public = bob_secret.public_key();
        let alice_classical = alice_secret.diffie_hellman(&bob_public);
        let bob_classical = bob_secret.diffie_hellman(&alice_public);
        let (sender_pq, receiver_pq) = ml_kem_exchange();
        let alice = combine(alice_classical.raw_secret_bytes(), &sender_pq)?;
        let bob = combine(bob_classical.raw_secret_bytes(), &receiver_pq)?;
        if alice == bob {
            Ok(())
        } else {
            Err("P-256 + ML-KEM-768 hybrid secrets disagreed".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_experimental_hybrids_agree() {
        X25519_MLKEM768.run_once().unwrap();
        P256_MLKEM768.run_once().unwrap();
    }
}
