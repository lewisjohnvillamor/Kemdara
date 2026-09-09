use aes_gcm::{
    Aes128Gcm, Aes256Gcm, Nonce as AesNonce,
    aead::{Aead, Generate, Key, KeyInit},
};
use aes_gcm_siv::{Aes256GcmSiv, Nonce as SivNonce};
use ascon_aead128::{AsconAead128, AsconAead128Key, AsconAead128Nonce};
use chacha20poly1305::{ChaCha20Poly1305, Nonce as ChaChaNonce, XChaCha20Poly1305, XNonce};

use super::{AlgorithmInfo, CryptoExperiment, ExperimentCategory, Maturity};

static PAYLOAD: [u8; 64 * 1024] = [0xA5; 64 * 1024];
const WORKLOAD: &str = "64 KiB encrypt + decrypt + plaintext check";

pub(super) struct Aes128GcmExperiment;
pub(super) struct Aes256GcmExperiment;
pub(super) struct ChaCha20Poly1305Experiment;
pub(super) struct XChaCha20Poly1305Experiment;
pub(super) struct Aes256GcmSivExperiment;
pub(super) struct AsconAead128Experiment;

pub(super) static AES128_GCM: Aes128GcmExperiment = Aes128GcmExperiment;
pub(super) static AES256_GCM: Aes256GcmExperiment = Aes256GcmExperiment;
pub(super) static CHACHA20_POLY1305: ChaCha20Poly1305Experiment = ChaCha20Poly1305Experiment;
pub(super) static XCHACHA20_POLY1305: XChaCha20Poly1305Experiment = XChaCha20Poly1305Experiment;
pub(super) static AES256_GCM_SIV: Aes256GcmSivExperiment = Aes256GcmSivExperiment;
pub(super) static ASCON_AEAD128: AsconAead128Experiment = AsconAead128Experiment;

fn check_plaintext(plaintext: &[u8]) -> Result<(), String> {
    if plaintext == PAYLOAD {
        Ok(())
    } else {
        Err("authenticated decryption did not recover the payload".into())
    }
}

impl CryptoExperiment for Aes128GcmExperiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "aes-128-gcm",
            name: "AES-128-GCM",
            family: "Authenticated encryption",
            standard: "NIST SP 800-38D",
            quantum_resistant: false,
            category: ExperimentCategory::PayloadEncryption,
            maturity: Maturity::Standardized,
            workload: WORKLOAD,
            iteration_divisor: 1,
            summary: "Widely deployed AEAD with hardware acceleration on many CPUs.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let key = Key::<Aes128Gcm>::generate();
        let cipher = Aes128Gcm::new(&key);
        let nonce = AesNonce::generate();
        let ciphertext = cipher
            .encrypt(&nonce, PAYLOAD.as_ref())
            .map_err(|_| "AES-128-GCM encryption failed")?;
        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|_| "AES-128-GCM authentication failed")?;
        check_plaintext(&plaintext)
    }
}

impl CryptoExperiment for Aes256GcmExperiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "aes-256-gcm",
            name: "AES-256-GCM",
            family: "Authenticated encryption",
            standard: "NIST SP 800-38D",
            quantum_resistant: false,
            category: ExperimentCategory::PayloadEncryption,
            maturity: Maturity::Standardized,
            workload: WORKLOAD,
            iteration_divisor: 1,
            summary: "Larger AES key with the same standardized GCM construction.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let key = Key::<Aes256Gcm>::generate();
        let cipher = Aes256Gcm::new(&key);
        let nonce = AesNonce::generate();
        let ciphertext = cipher
            .encrypt(&nonce, PAYLOAD.as_ref())
            .map_err(|_| "AES-256-GCM encryption failed")?;
        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|_| "AES-256-GCM authentication failed")?;
        check_plaintext(&plaintext)
    }
}

impl CryptoExperiment for ChaCha20Poly1305Experiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "chacha20-poly1305",
            name: "ChaCha20-Poly1305",
            family: "Authenticated encryption",
            standard: "RFC 8439",
            quantum_resistant: false,
            category: ExperimentCategory::PayloadEncryption,
            maturity: Maturity::Standardized,
            workload: WORKLOAD,
            iteration_divisor: 1,
            summary: "Software-friendly AEAD with consistent performance across CPUs.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let key = chacha20poly1305::Key::generate();
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaChaNonce::generate();
        let ciphertext = cipher
            .encrypt(&nonce, PAYLOAD.as_ref())
            .map_err(|_| "ChaCha20-Poly1305 encryption failed")?;
        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|_| "ChaCha20-Poly1305 authentication failed")?;
        check_plaintext(&plaintext)
    }
}

impl CryptoExperiment for XChaCha20Poly1305Experiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "xchacha20-poly1305",
            name: "XChaCha20-Poly1305",
            family: "Authenticated encryption",
            standard: "CFRG draft / deployed construction",
            quantum_resistant: false,
            category: ExperimentCategory::PayloadEncryption,
            maturity: Maturity::Interoperable,
            workload: WORKLOAD,
            iteration_divisor: 1,
            summary: "Extended-nonce ChaCha construction; interoperable but not an RFC standard.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let key = chacha20poly1305::Key::generate();
        let cipher = XChaCha20Poly1305::new(&key);
        let nonce = XNonce::generate();
        let ciphertext = cipher
            .encrypt(&nonce, PAYLOAD.as_ref())
            .map_err(|_| "XChaCha20-Poly1305 encryption failed")?;
        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|_| "XChaCha20-Poly1305 authentication failed")?;
        check_plaintext(&plaintext)
    }
}

impl CryptoExperiment for Aes256GcmSivExperiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "aes-256-gcm-siv",
            name: "AES-256-GCM-SIV",
            family: "Nonce-misuse-resistant authenticated encryption",
            standard: "RFC 8452",
            quantum_resistant: false,
            category: ExperimentCategory::PayloadEncryption,
            maturity: Maturity::Standardized,
            workload: WORKLOAD,
            iteration_divisor: 1,
            summary: "Limits damage from accidental nonce reuse, unlike ordinary AES-GCM.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let key = aes_gcm_siv::Key::<Aes256GcmSiv>::generate();
        let cipher = Aes256GcmSiv::new(&key);
        let nonce = SivNonce::generate();
        let ciphertext = cipher
            .encrypt(&nonce, PAYLOAD.as_ref())
            .map_err(|_| "AES-256-GCM-SIV encryption failed")?;
        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|_| "AES-256-GCM-SIV authentication failed")?;
        check_plaintext(&plaintext)
    }
}

impl CryptoExperiment for AsconAead128Experiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "ascon-aead128",
            name: "Ascon-AEAD128",
            family: "Lightweight authenticated encryption",
            standard: "NIST SP 800-232",
            quantum_resistant: false,
            category: ExperimentCategory::PayloadEncryption,
            maturity: Maturity::Standardized,
            workload: WORKLOAD,
            iteration_divisor: 1,
            summary: "Permutation-based AEAD standardized for constrained devices.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let key = AsconAead128Key::generate();
        let cipher = AsconAead128::new(&key);
        let nonce = AsconAead128Nonce::generate();
        let ciphertext = cipher
            .encrypt(&nonce, PAYLOAD.as_ref())
            .map_err(|_| "Ascon-AEAD128 encryption failed")?;
        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|_| "Ascon-AEAD128 authentication failed")?;
        check_plaintext(&plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_payload_adapter_rejects_corruption() {
        let key = Key::<Aes128Gcm>::generate();
        let cipher = Aes128Gcm::new(&key);
        let nonce = AesNonce::generate();
        let mut ciphertext = cipher.encrypt(&nonce, PAYLOAD.as_ref()).unwrap();
        ciphertext[0] ^= 1;
        assert!(cipher.decrypt(&nonce, ciphertext.as_ref()).is_err());

        AES256_GCM_SIV.run_once().unwrap();
        ASCON_AEAD128.run_once().unwrap();
    }
}
