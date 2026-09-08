use std::hint::black_box;

use blake2::Blake2s256;
use hkdf::Hkdf;
use sha2::{Digest, Sha256, Sha384};
use sha3::Sha3_256;

use super::{AlgorithmInfo, CryptoExperiment, ExperimentCategory, Maturity};

static HASH_PAYLOAD: [u8; 1024 * 1024] = [0x5C; 1024 * 1024];
const HASH_WORKLOAD: &str = "hash 1 MiB fixed payload";

macro_rules! hash_experiment {
    ($type:ident, $static:ident, $id:literal, $name:literal, $family:literal, $standard:literal, $digest:ty, $summary:literal) => {
        pub(super) struct $type;
        pub(super) static $static: $type = $type;

        impl CryptoExperiment for $type {
            fn info(&self) -> AlgorithmInfo {
                AlgorithmInfo {
                    id: $id,
                    name: $name,
                    family: $family,
                    standard: $standard,
                    quantum_resistant: true,
                    category: ExperimentCategory::Hash,
                    maturity: Maturity::Standardized,
                    workload: HASH_WORKLOAD,
                    iteration_divisor: 1,
                    summary: $summary,
                }
            }

            fn run_once(&self) -> Result<(), String> {
                black_box(<$digest>::digest(HASH_PAYLOAD));
                Ok(())
            }
        }
    };
}

hash_experiment!(
    Sha256Experiment,
    SHA256,
    "sha-256",
    "SHA-256",
    "SHA-2",
    "FIPS 180-4",
    Sha256,
    "Common 256-bit standardized hash baseline."
);
hash_experiment!(
    Sha384Experiment,
    SHA384,
    "sha-384",
    "SHA-384",
    "SHA-2",
    "FIPS 180-4",
    Sha384,
    "SHA-2 with a larger output and security margin."
);
hash_experiment!(
    Sha3Experiment,
    SHA3_256,
    "sha3-256",
    "SHA3-256",
    "SHA-3",
    "FIPS 202",
    Sha3_256,
    "Keccak-based standardized alternative to SHA-2."
);
hash_experiment!(
    Blake2sExperiment,
    BLAKE2S,
    "blake2s-256",
    "BLAKE2s-256",
    "BLAKE2",
    "RFC 7693",
    Blake2s256,
    "Compact hash optimized for 8- to 32-bit platforms."
);

pub(super) struct Blake3Experiment;
pub(super) static BLAKE3: Blake3Experiment = Blake3Experiment;

impl CryptoExperiment for Blake3Experiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "blake3",
            name: "BLAKE3",
            family: "BLAKE",
            standard: "BLAKE3 specification",
            quantum_resistant: true,
            category: ExperimentCategory::Hash,
            maturity: Maturity::Interoperable,
            workload: HASH_WORKLOAD,
            iteration_divisor: 1,
            summary: "Parallel tree hash intended for high throughput and extensibility.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        black_box(blake3::hash(&HASH_PAYLOAD));
        Ok(())
    }
}

pub(super) struct HkdfSha256Experiment;
pub(super) static HKDF_SHA256: HkdfSha256Experiment = HkdfSha256Experiment;

impl CryptoExperiment for HkdfSha256Experiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "hkdf-sha-256",
            name: "HKDF-SHA-256",
            family: "Extract-and-expand KDF",
            standard: "RFC 5869",
            quantum_resistant: true,
            category: ExperimentCategory::KeyDerivation,
            maturity: Maturity::Standardized,
            workload: "RFC 5869 extract + 42-byte expand + KAT check",
            iteration_divisor: 1,
            summary: "Domain-separable key derivation built on HMAC-SHA-256.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        const IKM: [u8; 22] = [0x0b; 22];
        const SALT: [u8; 13] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
        ];
        const INFO: [u8; 10] = [0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9];
        const EXPECTED: [u8; 42] = [
            0x3c, 0xb2, 0x5f, 0x25, 0xfa, 0xac, 0xd5, 0x7a, 0x90, 0x43, 0x4f, 0x64, 0xd0, 0x36,
            0x2f, 0x2a, 0x2d, 0x2d, 0x0a, 0x90, 0xcf, 0x1a, 0x5a, 0x4c, 0x5d, 0xb0, 0x2d, 0x56,
            0xec, 0xc4, 0xc5, 0xbf, 0x34, 0x00, 0x72, 0x08, 0xd5, 0xb8, 0x87, 0x18, 0x58, 0x65,
        ];
        let hkdf = Hkdf::<Sha256>::new(Some(&SALT), &IKM);
        let mut output = [0_u8; 42];
        hkdf.expand(&INFO, &mut output)
            .map_err(|_| "HKDF expansion failed")?;
        if output == EXPECTED {
            Ok(())
        } else {
            Err("HKDF-SHA-256 failed RFC 5869 test case 1".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hkdf_matches_rfc_5869() {
        HKDF_SHA256.run_once().unwrap();
    }
}
