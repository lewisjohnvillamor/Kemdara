use ed25519_dalek::{Signer as _, Verifier as _};
use ml_dsa::{Generate as _, Keypair as _};
use p256::ecdsa::{Signature as P256Signature, SigningKey as P256SigningKey};

use super::{AlgorithmInfo, CryptoExperiment, ExperimentCategory, Maturity};

const MESSAGE: [u8; 1024] = [0x36; 1024];
const WORKLOAD: &str = "keygen + sign 1 KiB + verify";

pub(super) struct Ed25519Experiment;
pub(super) struct P256EcdsaExperiment;
pub(super) struct MlDsa44Experiment;
pub(super) struct MlDsa65Experiment;
pub(super) struct MlDsa87Experiment;
pub(super) struct SlhDsaShake128fExperiment;

pub(super) static ED25519: Ed25519Experiment = Ed25519Experiment;
pub(super) static P256_ECDSA: P256EcdsaExperiment = P256EcdsaExperiment;
pub(super) static MLDSA44: MlDsa44Experiment = MlDsa44Experiment;
pub(super) static MLDSA65: MlDsa65Experiment = MlDsa65Experiment;
pub(super) static MLDSA87: MlDsa87Experiment = MlDsa87Experiment;
pub(super) static SLHDSA_SHAKE128F: SlhDsaShake128fExperiment = SlhDsaShake128fExperiment;

impl CryptoExperiment for Ed25519Experiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "ed25519",
            name: "Ed25519",
            family: "EdDSA signature",
            standard: "RFC 8032",
            quantum_resistant: false,
            category: ExperimentCategory::DigitalSignature,
            maturity: Maturity::Standardized,
            workload: WORKLOAD,
            iteration_divisor: 1,
            summary: "Deterministic Edwards-curve signature with compact keys and signatures.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let mut seed = [0_u8; 32];
        getrandom::fill(&mut seed).map_err(|error| error.to_string())?;
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();
        let signature = signing_key.sign(&MESSAGE);
        verifying_key
            .verify(&MESSAGE, &signature)
            .map_err(|error| format!("Ed25519 verification failed: {error}"))
    }
}

impl CryptoExperiment for P256EcdsaExperiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "ecdsa-p256-sha256",
            name: "ECDSA P-256",
            family: "ECDSA signature",
            standard: "FIPS 186-5",
            quantum_resistant: false,
            category: ExperimentCategory::DigitalSignature,
            maturity: Maturity::Standardized,
            workload: WORKLOAD,
            iteration_divisor: 1,
            summary: "Widely interoperable ECDSA over NIST P-256 with SHA-256.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let signing_key = loop {
            let mut scalar = [0_u8; 32];
            getrandom::fill(&mut scalar).map_err(|error| error.to_string())?;
            if let Ok(key) = P256SigningKey::from_slice(&scalar) {
                break key;
            }
        };
        let verifying_key = signing_key.verifying_key();
        let signature: P256Signature = signing_key.sign(&MESSAGE);
        verifying_key
            .verify(&MESSAGE, &signature)
            .map_err(|error| format!("P-256 ECDSA verification failed: {error}"))
    }
}

macro_rules! ml_dsa_experiment {
    ($type:ident, $params:ty, $id:literal, $name:literal, $summary:literal, $divisor:literal) => {
        impl CryptoExperiment for $type {
            fn info(&self) -> AlgorithmInfo {
                AlgorithmInfo {
                    id: $id,
                    name: $name,
                    family: "Module-lattice signature",
                    standard: "FIPS 204",
                    quantum_resistant: true,
                    category: ExperimentCategory::DigitalSignature,
                    maturity: Maturity::Standardized,
                    workload: WORKLOAD,
                    iteration_divisor: $divisor,
                    summary: $summary,
                }
            }

            fn run_once(&self) -> Result<(), String> {
                let signing_key = ml_dsa::SigningKey::<$params>::generate();
                let signature: ml_dsa::Signature<$params> = signing_key.sign(&MESSAGE);
                signing_key
                    .verifying_key()
                    .verify(&MESSAGE, &signature)
                    .map_err(|error| format!("{} verification failed: {error}", $name))
            }
        }
    };
}

ml_dsa_experiment!(
    MlDsa44Experiment,
    ml_dsa::MlDsa44,
    "ml-dsa-44",
    "ML-DSA-44",
    "Smallest standardized ML-DSA parameter set.",
    1
);
ml_dsa_experiment!(
    MlDsa65Experiment,
    ml_dsa::MlDsa65,
    "ml-dsa-65",
    "ML-DSA-65",
    "Balanced standardized ML-DSA parameter set.",
    1
);
ml_dsa_experiment!(
    MlDsa87Experiment,
    ml_dsa::MlDsa87,
    "ml-dsa-87",
    "ML-DSA-87",
    "Largest standardized ML-DSA parameter set.",
    1
);

impl CryptoExperiment for SlhDsaShake128fExperiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "slh-dsa-shake-128f",
            name: "SLH-DSA-SHAKE-128f",
            family: "Stateless hash-based signature",
            standard: "FIPS 205",
            quantum_resistant: true,
            category: ExperimentCategory::DigitalSignature,
            maturity: Maturity::Standardized,
            workload: WORKLOAD,
            iteration_divisor: 20,
            summary: "Fast-signing SLH-DSA parameter set with conservative hash-based assumptions.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let mut rng = rand::rng();
        let signing_key = slh_dsa::SigningKey::<slh_dsa::Shake128f>::new(&mut rng);
        let verifying_key = signing_key.verifying_key();
        let signature = signing_key
            .try_sign(&MESSAGE)
            .map_err(|error| format!("SLH-DSA signing failed: {error}"))?;
        verifying_key
            .verify(&MESSAGE, &signature)
            .map_err(|error| format!("SLH-DSA verification failed: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classical_signature_adapters_verify() {
        ED25519.run_once().unwrap();
        P256_ECDSA.run_once().unwrap();
    }

    #[test]
    fn post_quantum_signature_adapters_verify() {
        MLDSA44.run_once().unwrap();
        MLDSA65.run_once().unwrap();
        MLDSA87.run_once().unwrap();
        SLHDSA_SHAKE128F.run_once().unwrap();
    }
}
