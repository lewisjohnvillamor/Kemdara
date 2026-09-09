use std::hint::black_box;

use argon2::{Algorithm, Argon2, Params as Argon2Params, Version};
use scrypt::Params as ScryptParams;

use super::{AlgorithmInfo, CryptoExperiment, ExperimentCategory, Maturity};

const PASSWORD: &[u8] = b"Kemdara benchmark password";
const SALT: &[u8] = b"Kemdara fixed salt";

pub(super) struct Argon2idExperiment;
pub(super) struct ScryptExperiment;

pub(super) static ARGON2ID: Argon2idExperiment = Argon2idExperiment;
pub(super) static SCRYPT: ScryptExperiment = ScryptExperiment;

impl CryptoExperiment for Argon2idExperiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "argon2id-19mib-t2-p1",
            name: "Argon2id",
            family: "Memory-hard password KDF",
            standard: "RFC 9106",
            quantum_resistant: false,
            category: ExperimentCategory::PasswordDerivation,
            maturity: Maturity::Standardized,
            workload: "derive 32 bytes · 19 MiB · t=2 · p=1",
            iteration_divisor: 200,
            summary: "Hybrid data-independent/data-dependent password hashing resistant to GPU attacks.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let params = Argon2Params::new(19 * 1024, 2, 1, Some(32))
            .map_err(|error| format!("Argon2id parameters failed: {error}"))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut output = [0_u8; 32];
        argon2
            .hash_password_into(PASSWORD, SALT, &mut output)
            .map_err(|error| format!("Argon2id derivation failed: {error}"))?;
        if output.iter().all(|byte| *byte == 0) {
            Err("Argon2id returned an all-zero output".into())
        } else {
            black_box(output);
            Ok(())
        }
    }
}

impl CryptoExperiment for ScryptExperiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "scrypt-ln15-r8-p1",
            name: "scrypt",
            family: "Memory-hard password KDF",
            standard: "RFC 7914",
            quantum_resistant: false,
            category: ExperimentCategory::PasswordDerivation,
            maturity: Maturity::Standardized,
            workload: "derive 32 bytes · N=32768 · r=8 · p=1",
            iteration_divisor: 200,
            summary: "Older memory-hard password KDF that remains useful as a comparative baseline.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let params = ScryptParams::new(15, 8, 1)
            .map_err(|error| format!("scrypt parameters failed: {error}"))?;
        let mut output = [0_u8; 32];
        scrypt::scrypt(PASSWORD, SALT, &params, &mut output)
            .map_err(|error| format!("scrypt derivation failed: {error}"))?;
        if output.iter().all(|byte| *byte == 0) {
            Err("scrypt returned an all-zero output".into())
        } else {
            black_box(output);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argon2id_matches_reference_vector() {
        let params = Argon2Params::new(256, 2, 1, Some(32)).unwrap();
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut output = [0_u8; 32];
        argon2
            .hash_password_into(b"password", b"somesalt", &mut output)
            .unwrap();
        assert_eq!(
            output,
            [
                0x9d, 0xfe, 0xb9, 0x10, 0xe8, 0x0b, 0xad, 0x03, 0x11, 0xfe, 0xe2, 0x0f, 0x9c, 0x0e,
                0x2b, 0x12, 0xc1, 0x79, 0x87, 0xb4, 0xca, 0xc9, 0x0c, 0x2e, 0xf5, 0x4d, 0x5b, 0x30,
                0x21, 0xc6, 0x8b, 0xfe,
            ]
        );
    }

    #[test]
    fn scrypt_matches_rfc_7914_vector() {
        let params = ScryptParams::new(4, 1, 1).unwrap();
        let mut output = [0_u8; 64];
        scrypt::scrypt(b"", b"", &params, &mut output).unwrap();
        assert_eq!(
            output,
            [
                0x77, 0xd6, 0x57, 0x62, 0x38, 0x65, 0x7b, 0x20, 0x3b, 0x19, 0xca, 0x42, 0xc1, 0x8a,
                0x04, 0x97, 0xf1, 0x6b, 0x48, 0x44, 0xe3, 0x07, 0x4a, 0xe8, 0xdf, 0xdf, 0xfa, 0x3f,
                0xed, 0xe2, 0x14, 0x42, 0xfc, 0xd0, 0x06, 0x9d, 0xed, 0x09, 0x48, 0xf8, 0x32, 0x6a,
                0x75, 0x3a, 0x0f, 0xc8, 0x1f, 0x17, 0xe8, 0xd3, 0xe0, 0xfb, 0x2e, 0x0d, 0x36, 0x28,
                0xcf, 0x35, 0xe2, 0x0c, 0x38, 0xd1, 0x89, 0x06,
            ]
        );
    }
}
