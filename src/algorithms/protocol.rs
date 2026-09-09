use snow::{Builder, params::NoiseParams};

use super::{
    AlgorithmInfo, CryptoExperiment, ExperimentCategory, ExperimentObservation, Maturity,
    ProtocolTrace, TranscriptEvent,
};

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
            workload: "2-message NN handshake + two-way encrypted 1 KiB transport exchange",
            iteration_divisor: 1,
            summary: "Minimal forward-secret channel with no peer authentication.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        run_noise_nn().map(|_| ())
    }

    fn observe(&self) -> Result<ExperimentObservation, String> {
        let [first, second, forward, return_flight] = run_noise_nn()?;
        Ok(observation(protocol_trace(
            "Noise_NN_25519_ChaChaPoly_BLAKE2s",
            "None: NN does not exchange or authenticate static identity keys.",
            "Ephemeral DH provides forward secrecy for the completed session.",
            "No static identities are sent because neither peer is authenticated.",
            [
                event(
                    1,
                    "initiator → responder",
                    "handshake",
                    "e",
                    first,
                    "Responder has the initiator's ephemeral key; no identity is authenticated.",
                ),
                event(
                    2,
                    "responder → initiator",
                    "handshake",
                    "e, ee",
                    second,
                    "Both peers have handshake keys; neither peer identity is authenticated.",
                ),
                event(
                    3,
                    "initiator → responder",
                    "transport",
                    "AEAD",
                    forward,
                    "Encrypted transport; peer identity remains unauthenticated.",
                ),
                event(
                    4,
                    "responder → initiator",
                    "transport",
                    "AEAD",
                    return_flight,
                    "Two-way encrypted transport verified; peer identities remain unauthenticated.",
                ),
            ],
        )))
    }
}

fn run_noise_nn() -> Result<[usize; 4], String> {
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

    let first = transfer_handshake(&mut initiator, &mut responder, &mut wire, &mut plaintext)?;
    let second = transfer_handshake(&mut responder, &mut initiator, &mut wire, &mut plaintext)?;
    let (forward, return_flight) =
        verify_transport(initiator, responder, &mut wire, &mut plaintext)?;

    Ok([first, second, forward, return_flight])
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
            workload: "3-message XX handshake + two-way encrypted 1 KiB transport exchange",
            iteration_divisor: 1,
            summary: "Generic mutual authentication when static keys are exchanged in-handshake.",
        }
    }

    fn run_once(&self) -> Result<(), String> {
        run_noise_xx().map(|_| ())
    }

    fn observe(&self) -> Result<ExperimentObservation, String> {
        let [first, second, third, forward, return_flight] = run_noise_xx()?;
        Ok(observation(protocol_trace(
            "Noise_XX_25519_ChaChaPoly_BLAKE2s",
            "Mutual static-key possession is verified; applications must still bind keys to trusted identities.",
            "Ephemeral DH provides forward secrecy after the handshake completes.",
            "Static keys are encrypted in flights 2 and 3, hiding them from passive observers.",
            [
                event(
                    1,
                    "initiator → responder",
                    "handshake",
                    "e",
                    first,
                    "Responder has the initiator's ephemeral key; no identity is authenticated yet.",
                ),
                event(
                    2,
                    "responder → initiator",
                    "handshake",
                    "e, ee, s, es",
                    second,
                    "Initiator verifies responder static-key possession; responder has not authenticated the initiator.",
                ),
                event(
                    3,
                    "initiator → responder",
                    "handshake",
                    "s, se",
                    third,
                    "Both peers verify static-key possession; external identity trust is still required.",
                ),
                event(
                    4,
                    "initiator → responder",
                    "transport",
                    "AEAD",
                    forward,
                    "Encrypted transport with mutually authenticated key possession.",
                ),
                event(
                    5,
                    "responder → initiator",
                    "transport",
                    "AEAD",
                    return_flight,
                    "Two-way authenticated encrypted transport verified.",
                ),
            ],
        )))
    }
}

fn run_noise_xx() -> Result<[usize; 5], String> {
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

    let first = transfer_handshake(&mut initiator, &mut responder, &mut wire, &mut plaintext)?;
    let second = transfer_handshake(&mut responder, &mut initiator, &mut wire, &mut plaintext)?;
    let third = transfer_handshake(&mut initiator, &mut responder, &mut wire, &mut plaintext)?;

    if initiator.get_remote_static() != Some(responder_keypair.public.as_slice())
        || responder.get_remote_static() != Some(initiator_keypair.public.as_slice())
    {
        return Err("Noise XX peers did not authenticate the expected static keys".into());
    }

    let (forward, return_flight) =
        verify_transport(initiator, responder, &mut wire, &mut plaintext)?;

    Ok([first, second, third, forward, return_flight])
}

fn event(
    flight: usize,
    direction: &'static str,
    phase: &'static str,
    tokens: &'static str,
    wire_bytes: usize,
    security_state: &'static str,
) -> TranscriptEvent {
    TranscriptEvent {
        flight,
        direction,
        phase,
        tokens,
        wire_bytes,
        security_state,
    }
}

fn protocol_trace<const N: usize>(
    pattern: &'static str,
    authentication: &'static str,
    forward_secrecy: &'static str,
    identity_exposure: &'static str,
    events: [TranscriptEvent; N],
) -> ProtocolTrace {
    let handshake_messages = events.iter().filter(|event| event.phase == "handshake").count();
    let transport_messages = events.iter().filter(|event| event.phase == "transport").count();
    let handshake_wire_bytes = events
        .iter()
        .filter(|event| event.phase == "handshake")
        .map(|event| event.wire_bytes)
        .sum();
    let transport_wire_bytes = events
        .iter()
        .filter(|event| event.phase == "transport")
        .map(|event| event.wire_bytes)
        .sum();
    let application_payload_bytes = TRANSPORT_PAYLOAD.len() * transport_messages;
    let total_wire_bytes = handshake_wire_bytes + transport_wire_bytes;

    ProtocolTrace {
        pattern,
        handshake_messages,
        transport_messages,
        handshake_wire_bytes,
        transport_wire_bytes,
        application_payload_bytes,
        total_wire_bytes,
        expansion_bytes: total_wire_bytes.saturating_sub(application_payload_bytes),
        authentication,
        forward_secrecy,
        identity_exposure,
        events: events.into(),
    }
}

fn observation(protocol: ProtocolTrace) -> ExperimentObservation {
    ExperimentObservation {
        total_wire_bytes: Some(protocol.total_wire_bytes),
        application_payload_bytes: Some(protocol.application_payload_bytes),
        protocol: Some(protocol),
    }
}

fn transfer_handshake(
    sender: &mut snow::HandshakeState,
    receiver: &mut snow::HandshakeState,
    wire: &mut [u8],
    plaintext: &mut [u8],
) -> Result<usize, String> {
    let length = sender
        .write_message(&[], wire)
        .map_err(|error| error.to_string())?;
    receiver
        .read_message(&wire[..length], plaintext)
        .map_err(|error| error.to_string())?;
    Ok(length)
}

fn verify_transport(
    initiator: snow::HandshakeState,
    responder: snow::HandshakeState,
    wire: &mut [u8],
    plaintext: &mut [u8],
) -> Result<(usize, usize), String> {
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

    if plaintext[..plaintext_length] != TRANSPORT_PAYLOAD {
        return Err("Noise initiator-to-responder transport plaintext did not round-trip".into());
    }

    let return_ciphertext_length = responder
        .write_message(&TRANSPORT_PAYLOAD, wire)
        .map_err(|error| error.to_string())?;
    let return_plaintext_length = initiator
        .read_message(&wire[..return_ciphertext_length], plaintext)
        .map_err(|error| error.to_string())?;
    if plaintext[..return_plaintext_length] != TRANSPORT_PAYLOAD {
        return Err("Noise responder-to-initiator transport plaintext did not round-trip".into());
    }

    Ok((ciphertext_length, return_ciphertext_length))
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
    fn protocol_observations_account_for_every_wire_byte() {
        for experiment in [&NOISE_NN as &dyn CryptoExperiment, &NOISE_XX] {
            let observation = experiment.observe().unwrap();
            let trace = observation.protocol.unwrap();
            assert_eq!(trace.transport_messages, 2);
            assert_eq!(trace.application_payload_bytes, 2 * TRANSPORT_PAYLOAD.len());
            assert_eq!(
                trace.total_wire_bytes,
                trace.events.iter().map(|event| event.wire_bytes).sum()
            );
            assert_eq!(observation.total_wire_bytes, Some(trace.total_wire_bytes));
        }
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
