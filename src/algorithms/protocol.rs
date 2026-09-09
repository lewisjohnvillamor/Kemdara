use snow::{Builder, params::NoiseParams};

use super::{AlgorithmInfo, CryptoExperiment, ExperimentCategory, Maturity};

const TRANSPORT_PAYLOAD: [u8; 1024] = [0x4b; 1024];

pub(super) struct NoiseNnExperiment;
pub(super) struct NoiseXxExperiment;

pub(super) static NOISE_NN: NoiseNnExperiment = NoiseNnExperiment;
pub(super) static NOISE_XX: NoiseXxExperiment = NoiseXxExperiment;

impl CryptoExperiment for NoiseNnExperiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "noise-nn-25519-chachapoly-blake2s",
            name: "Noise NN",
            family: "Unauthenticated secure-channel handshake",
            standard: "Noise Protocol Framework revision 34",
            quantum_resistant: false,
            category: ExperimentCategory::ProtocolHandshake,
            maturity: Maturity::Experimental,
            workload: "2-message NN handshake + encrypted 1 KiB transport round-trip",
            iteration_divisor: 1,
            summary: "Minimal forward-secret channel with no peer authentication.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let params: NoiseParams = "Noise_NN_25519_ChaChaPoly_BLAKE2s"
            .parse()
            .map_err(|error: snow::Error| error.to_string())?;
        let mut initiator = Builder::new(params.clone())
            .build_initiator()
            .map_err(|error| error.to_string())?;
        let mut responder = Builder::new(params)
            .build_responder()
            .map_err(|error| error.to_string())?;
        let mut wire = [0_u8; 65535];
        let mut plaintext = [0_u8; 65535];

        transfer_handshake(&mut initiator, &mut responder, &mut wire, &mut plaintext)?;
        transfer_handshake(&mut responder, &mut initiator, &mut wire, &mut plaintext)?;
        verify_transport(initiator, responder, &mut wire, &mut plaintext)
    }
}

impl CryptoExperiment for NoiseXxExperiment {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            id: "noise-xx-25519-chachapoly-blake2s",
            name: "Noise XX",
            family: "Mutually authenticated secure-channel handshake",
            standard: "Noise Protocol Framework revision 34",
            quantum_resistant: false,
            category: ExperimentCategory::ProtocolHandshake,
            maturity: Maturity::Experimental,
            workload: "3-message XX handshake + encrypted 1 KiB transport round-trip",
            iteration_divisor: 1,
            summary: "Generic mutual authentication when static keys are exchanged in-handshake.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        let params: NoiseParams = "Noise_XX_25519_ChaChaPoly_BLAKE2s"
            .parse()
            .map_err(|error: snow::Error| error.to_string())?;
        let initiator_builder = Builder::new(params.clone());
        let responder_builder = Builder::new(params.clone());
        let initiator_keypair = initiator_builder
            .generate_keypair()
            .map_err(|error| error.to_string())?;
        let responder_keypair = responder_builder
            .generate_keypair()
            .map_err(|error| error.to_string())?;
        let mut initiator = initiator_builder
            .local_private_key(&initiator_keypair.private)
            .map_err(|error| error.to_string())?
            .build_initiator()
            .map_err(|error| error.to_string())?;
        let mut responder = responder_builder
            .local_private_key(&responder_keypair.private)
            .map_err(|error| error.to_string())?
            .build_responder()
            .map_err(|error| error.to_string())?;
        let mut wire = [0_u8; 65535];
        let mut plaintext = [0_u8; 65535];

        transfer_handshake(&mut initiator, &mut responder, &mut wire, &mut plaintext)?;
        transfer_handshake(&mut responder, &mut initiator, &mut wire, &mut plaintext)?;
        transfer_handshake(&mut initiator, &mut responder, &mut wire, &mut plaintext)?;

        if initiator.get_remote_static() != Some(responder_keypair.public.as_slice())
            || responder.get_remote_static() != Some(initiator_keypair.public.as_slice())
        {
            return Err("Noise XX peers did not authenticate the expected static keys".into());
        }

        verify_transport(initiator, responder, &mut wire, &mut plaintext)
    }
}

fn transfer_handshake(
    sender: &mut snow::HandshakeState,
    receiver: &mut snow::HandshakeState,
    wire: &mut [u8],
    plaintext: &mut [u8],
) -> Result<(), String> {
    let length = sender
        .write_message(&[], wire)
        .map_err(|error| error.to_string())?;
    receiver
        .read_message(&wire[..length], plaintext)
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn verify_transport(
    initiator: snow::HandshakeState,
    responder: snow::HandshakeState,
    wire: &mut [u8],
    plaintext: &mut [u8],
) -> Result<(), String> {
    if !initiator.is_handshake_finished() || !responder.is_handshake_finished() {
        return Err("Noise handshake did not reach transport state".into());
    }

    let mut initiator = initiator
        .into_transport_mode()
        .map_err(|error| error.to_string())?;
    let mut responder = responder
        .into_transport_mode()
        .map_err(|error| error.to_string())?;
    let ciphertext_length = initiator
        .write_message(&TRANSPORT_PAYLOAD, wire)
        .map_err(|error| error.to_string())?;
    let plaintext_length = responder
        .read_message(&wire[..ciphertext_length], plaintext)
        .map_err(|error| error.to_string())?;

    if plaintext[..plaintext_length] == TRANSPORT_PAYLOAD {
        Ok(())
    } else {
        Err("Noise transport plaintext did not round-trip".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_protocol_scenarios_reach_transport_mode() {
        NOISE_NN.run_once().unwrap();
        NOISE_XX.run_once().unwrap();
    }

    #[test]
    fn noise_nn_rejects_a_corrupted_authenticated_response() {
        let params: NoiseParams = "Noise_NN_25519_ChaChaPoly_BLAKE2s".parse().unwrap();
        let mut initiator = Builder::new(params.clone()).build_initiator().unwrap();
        let mut responder = Builder::new(params).build_responder().unwrap();
        let mut wire = [0_u8; 65535];
        let mut plaintext = [0_u8; 65535];

        transfer_handshake(&mut initiator, &mut responder, &mut wire, &mut plaintext).unwrap();
        let length = responder.write_message(&[], &mut wire).unwrap();
        wire[length - 1] ^= 1;
        assert!(
            initiator
                .read_message(&wire[..length], &mut plaintext)
                .is_err()
        );
    }
}
