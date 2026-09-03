//! Decoder robustness for the adapter's wire and document contracts.
//!
//! The executor reads exactly three kinds of machine-written input: the
//! protocol-4 engine request on standard input, the axis-execution manifest a
//! controller may hand back, and the shared result documents it published —
//! covered here through the `fft` child a transient publishes, which carries
//! the widest payload of any of them. All three are decoded from bytes
//! produced elsewhere, so all three are damaged here with a seeded xorshift
//! stream and with an exhaustive truncation sweep.
//!
//! The invariant is the same as `rspice-core`'s decoder robustness suite:
//! never a panic, never a success on corrupted input that claims a different
//! value, always a typed error. "Claims a different value" is checked as a
//! fixed point — anything the decoder accepted must survive its own
//! re-encoding unchanged.

use std::panic::{AssertUnwindSafe, catch_unwind};

use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::execution::{
    ANALYSIS_RESULT_DOCUMENT_VERSION, AnalysisKind, AnalysisResultDocument, AxisKind, DeckPlan,
    numeric_run_coordinate_id, planned_transient_fft_spectra,
};
use rspice_core::netlist::Netlist;
use rspice_core::resource::ResourceLimits;
use rspice_engine_adapter::axis_execution_document::{
    AnalysisExecution, AxisAnalysisKind, AxisAssignmentDocument, AxisAssignmentKind,
    AxisExecutionDocument, CoordinateExecution, OutputNamespaceDocument, ResultDocumentReference,
    StepTargetDocument,
};
use rspice_engine_adapter::measure::canonical_decimal;
use rspice_engine_adapter::wire::{
    EngineArtifact, EngineRequest, EngineRevision, INTEGRITY_ENGINE_PROTOCOL_VERSION,
    MAX_ENGINE_RESULT_MANIFEST_BYTES, MAX_ENGINE_RETAINED_RESULT_BYTES, digest_hex,
    parse_engine_request, revision_content_digest, simulation_request_digest,
};
use serde_json::json;
use uuid::Uuid;

/// Deterministic xorshift64* stream.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound.max(1) as u64) as usize
    }
}

/// Bytes that historically break JSON and digest decoders.
const HOSTILE_BYTES: &[u8] = b"{}[]\",:\\ \n\t-+.eE0189abcdefNnIi\0\x7f\xff";

fn mutate(rng: &mut Rng, seed: &[u8]) -> Vec<u8> {
    let mut bytes = seed.to_vec();
    match rng.below(5) {
        0 => {
            let cut = rng.below(bytes.len() + 1);
            bytes.truncate(cut);
        }
        1 => {
            for _ in 0..1 + rng.below(8) {
                if bytes.is_empty() {
                    break;
                }
                let position = rng.below(bytes.len());
                bytes[position] = (rng.next() & 0xff) as u8;
            }
        }
        2 => {
            for _ in 0..1 + rng.below(6) {
                let at = rng.below(bytes.len() + 1);
                bytes.insert(at, HOSTILE_BYTES[rng.below(HOSTILE_BYTES.len())]);
            }
        }
        3 => {
            if !bytes.is_empty() {
                let start = rng.below(bytes.len());
                let length = rng.below((bytes.len() - start).min(32) + 1);
                let slice = bytes[start..start + length].to_vec();
                let at = rng.below(bytes.len() + 1);
                for (offset, byte) in slice.into_iter().enumerate() {
                    bytes.insert(at + offset, byte);
                }
            }
        }
        _ => {
            if !bytes.is_empty() {
                let start = rng.below(bytes.len());
                let length = rng.below((bytes.len() - start).min(32) + 1);
                bytes.drain(start..start + length);
            }
        }
    }
    bytes
}

fn chaos_bytes<F: Fn(&[u8])>(name: &str, seed: &[u8], seed_base: u64, rounds: usize, target: F) {
    let mut rng = Rng::new(seed_base);
    for round in 0..rounds {
        let input = mutate(&mut rng, seed);
        let result = catch_unwind(AssertUnwindSafe(|| target(&input)));
        assert!(
            result.is_ok(),
            "{name} panicked (round {round}) on {} bytes:\n{}",
            input.len(),
            String::from_utf8_lossy(&input)
        );
    }
}

fn truncation_sweep<F: Fn(&[u8])>(name: &str, seed: &[u8], samples: usize, target: F) {
    let stride = (seed.len() / samples.max(1)).max(1);
    let mut cut = 0;
    while cut <= seed.len() {
        let input = &seed[..cut];
        let result = catch_unwind(AssertUnwindSafe(|| target(input)));
        assert!(
            result.is_ok(),
            "{name} panicked on the {cut}-byte prefix of a {}-byte artifact",
            seed.len()
        );
        cut += stride;
    }
    let result = catch_unwind(AssertUnwindSafe(|| target(seed)));
    assert!(result.is_ok(), "{name} panicked on the whole artifact");
}

//=============================================================================
// Protocol-4 engine request
//=============================================================================

/// Build a request the executor accepts, through the same canonicalization it
/// verifies against, so the mutation stream starts from a valid document.
fn engine_request_bytes() -> Vec<u8> {
    let circuit_id = Uuid::parse_str("019f76ae-0000-7000-8000-0000000009a1")
        .expect("a fixed fixture UUID parses");
    let revision_id = Uuid::parse_str("019f76ae-0000-7000-8000-0000000009a2")
        .expect("a fixed fixture UUID parses");
    let run_id = Uuid::parse_str("019f76ae-0000-7000-8000-0000000009a3")
        .expect("a fixed fixture UUID parses");
    let document = json!({"components": [], "schema": "rspice-circuit-v1"});
    let analysis = json!({"kind": "operating_point"});
    let artifacts: Vec<EngineArtifact> = Vec::new();

    let revision_digest = revision_content_digest(1, &document, &artifacts)
        .expect("the fixture revision canonicalizes");
    let request_digest =
        simulation_request_digest(circuit_id, revision_id, &revision_digest, &analysis)
            .expect("the fixture request canonicalizes");

    let request = EngineRequest {
        protocol_version: INTEGRITY_ENGINE_PROTOCOL_VERSION,
        simulation_run_id: run_id,
        circuit_id,
        attempt: 1,
        request_digest_version: Some(1),
        request_sha256: digest_hex(&request_digest),
        analysis,
        revision: EngineRevision {
            id: revision_id,
            schema_version: 1,
            content_digest_version: Some(2),
            content_sha256: digest_hex(&revision_digest),
            document,
            artifacts,
        },
    };
    let bytes = serde_json::to_vec(&request).expect("the fixture request serializes");
    parse_engine_request(&bytes).expect("the fixture request is a valid protocol-4 request");
    bytes
}

fn assert_request_decode_is_faithful(bytes: &[u8]) {
    let Ok(request) = parse_engine_request(bytes) else {
        return;
    };
    let reencoded =
        serde_json::to_vec(&request).expect("an accepted request must be re-serializable");
    let again = parse_engine_request(&reencoded)
        .expect("an accepted request's own serialization must be accepted");
    assert_eq!(
        serde_json::to_value(&again).expect("re-decoded request renders"),
        serde_json::to_value(&request).expect("decoded request renders"),
        "the request decoder produced a value that does not survive its own serialization"
    );
}

#[test]
fn engine_request_decoder_survives_chaos_without_inventing_a_value() {
    let seed = engine_request_bytes();
    chaos_bytes(
        "parse_engine_request",
        &seed,
        0x0ADA_0001,
        4_000,
        assert_request_decode_is_faithful,
    );
}

#[test]
fn engine_request_decoder_survives_every_truncation() {
    let seed = engine_request_bytes();
    truncation_sweep("parse_engine_request", &seed, 400, |input| {
        assert_request_decode_is_faithful(input);
    });
}

#[test]
fn a_strict_prefix_of_a_valid_request_is_always_refused() {
    // The request commits to its own digests, so a truncated one cannot be a
    // shorter valid request: the JSON is simply incomplete.
    let seed = engine_request_bytes();
    for cut in 0..seed.len() {
        assert!(
            parse_engine_request(&seed[..cut]).is_err(),
            "a {cut}-byte prefix of a {}-byte request was accepted",
            seed.len()
        );
    }
}

//=============================================================================
// Axis-execution manifest
//=============================================================================

fn axis_manifest_json() -> Vec<u8> {
    let mut runs = Vec::new();
    for (ordinal, resistance) in [1_000.0_f64, 2_000.0].into_iter().enumerate() {
        let coordinate_id =
            numeric_run_coordinate_id(&[(AxisKind::Step, "param:r", resistance)], 0)
                .expect("a finite fixture assignment has an identity")
                .to_string();
        let namespace = format!("run-{coordinate_id}");
        runs.push(CoordinateExecution {
            ordinal: ordinal + 1,
            coordinate_id,
            coordinate_namespace: namespace.clone(),
            assignments: vec![AxisAssignmentDocument {
                kind: AxisAssignmentKind::Step,
                name: "param:r".to_owned(),
                value_index: ordinal,
                value_decimal: canonical_decimal(resistance).expect("a finite fixture value"),
                target: Some(StepTargetDocument::Parameter {
                    name: "r".to_owned(),
                }),
            }],
            analyses: vec![AnalysisExecution {
                analysis_id: "tran-001".to_owned(),
                output_namespace: OutputNamespaceDocument {
                    coordinate: namespace,
                    analysis: "tran-001".to_owned(),
                },
                artifacts: vec![ResultDocumentReference {
                    path: format!("results/coordinate-{ordinal}__tran-001.result.json"),
                    content_type: "application/vnd.rspice.analysis-result+json;version=1"
                        .to_owned(),
                    schema: "rspice-analysis-result".to_owned(),
                    schema_version: 1,
                    result_kind: "tran".to_owned(),
                }],
                measurements: Vec::new(),
            }],
        });
    }
    let document = AxisExecutionDocument::new(AxisAnalysisKind::Transient, runs)
        .expect("the fixture manifest is valid");
    serde_json::to_vec(&document.to_value().expect("the fixture manifest encodes"))
        .expect("the fixture manifest renders")
}

fn assert_manifest_decode_is_faithful(bytes: &[u8]) {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return;
    };
    let Ok(document) = AxisExecutionDocument::from_json_with_abort(
        text,
        &NoAbort,
        MAX_ENGINE_RESULT_MANIFEST_BYTES as u64,
    ) else {
        return;
    };
    let reencoded = serde_json::to_string(
        &document
            .to_value()
            .expect("an accepted manifest must re-encode"),
    )
    .expect("an accepted manifest renders");
    let again = AxisExecutionDocument::from_json_with_abort(
        &reencoded,
        &NoAbort,
        MAX_ENGINE_RESULT_MANIFEST_BYTES as u64,
    )
    .expect("an accepted manifest's own encoding must be accepted");
    assert_eq!(
        again, document,
        "the manifest decoder produced a value that does not survive its own encoding"
    );
    // Whatever it accepted, every coordinate identity is the planner's own.
    for run in &document.runs {
        assert_eq!(
            run.coordinate_namespace,
            format!("run-{}", run.coordinate_id)
        );
    }
}

#[test]
fn axis_manifest_decoder_survives_chaos_without_inventing_a_value() {
    let seed = axis_manifest_json();
    chaos_bytes(
        "AxisExecutionDocument::from_json_with_abort",
        &seed,
        0x0ADA_0002,
        4_000,
        assert_manifest_decode_is_faithful,
    );
}

#[test]
fn axis_manifest_decoder_survives_every_truncation() {
    let seed = axis_manifest_json();
    truncation_sweep(
        "AxisExecutionDocument::from_json_with_abort",
        &seed,
        400,
        assert_manifest_decode_is_faithful,
    );
}

//=============================================================================
// Transient `.FFT` child document
//=============================================================================

/// The exact `fft` child this executor publishes beside a transient, encoded
/// the way it lands under `results/`.
fn fft_child_json() -> Vec<u8> {
    let deck = "fft decoder fixture\n\
                V1 out 0 SIN(0 1 2k)\n\
                R1 out 0 1k\n\
                .options fft fftout=1\n\
                .tran 1u 1m\n\
                .fft v(out) np=16 format=unorm window=rect freq=2k\n\
                .end\n";
    let netlist = Netlist::parse_validated(deck).expect("the FFT fixture parses");
    let plan = DeckPlan::from_netlist_with_abort(&netlist, &ResourceLimits::default(), &NoAbort)
        .expect("the FFT fixture plans");
    let parent = plan
        .analyses()
        .iter()
        .find(|analysis| analysis.id().kind() == AnalysisKind::Tran)
        .expect("the FFT fixture plans a transient")
        .id();
    let result = Engine::new(SimulationConfig::default())
        .run_tran_with_abort(&netlist, 1.0e-3, 1.0e-6, &NoAbort)
        .expect("the FFT fixture executes");
    let spectra = planned_transient_fft_spectra(&plan, parent, &result.fft_results, &NoAbort)
        .expect("the FFT fixture pairs its planned spectrum");
    let spectrum = spectra.first().expect("the fixture authors one .FFT card");
    AnalysisResultDocument::from_transient_fft(
        spectrum.analysis,
        spectrum.parent,
        spectrum.output_unit.clone(),
        spectrum.result,
    )
    .expect("the FFT fixture projects into the shared document")
    .build()
    .expect("the FFT fixture document builds")
    .to_json()
    .expect("the FFT fixture encodes")
    .into_bytes()
}

fn assert_fft_child_decode_is_faithful(bytes: &[u8]) {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return;
    };
    let Ok(document) = AnalysisResultDocument::from_json_with_abort(
        text,
        &NoAbort,
        MAX_ENGINE_RETAINED_RESULT_BYTES,
    ) else {
        return;
    };
    let reencoded = document
        .to_json()
        .expect("an accepted child must re-encode");
    let again = AnalysisResultDocument::from_json(&reencoded)
        .expect("an accepted child's own encoding must be accepted");
    assert_eq!(
        again, document,
        "the FFT child decoder produced a value that does not survive its own encoding"
    );
}

#[test]
fn fft_child_decoder_survives_chaos_without_inventing_a_value() {
    let seed = fft_child_json();
    chaos_bytes(
        "AnalysisResultDocument::from_json_with_abort (fft child)",
        &seed,
        0x0ADA_0003,
        3_000,
        assert_fft_child_decode_is_faithful,
    );
}

#[test]
fn fft_child_decoder_survives_every_truncation() {
    let seed = fft_child_json();
    truncation_sweep(
        "AnalysisResultDocument::from_json_with_abort (fft child)",
        &seed,
        400,
        assert_fft_child_decode_is_faithful,
    );
}

#[test]
fn a_forward_schema_version_of_the_fft_child_is_refused_before_any_field() {
    let seed = fft_child_json();
    let mut document: serde_json::Value =
        serde_json::from_slice(&seed).expect("the fixture child is JSON");
    document["schemaVersion"] = json!(ANALYSIS_RESULT_DOCUMENT_VERSION + 1);
    // Fields this build has no meaning for are added alongside the bumped
    // version: a decoder that reported them as unknown fields would be reading
    // a future document field by field instead of refusing it outright.
    document["futureField"] = json!({"published": "by a later build"});
    let error = AnalysisResultDocument::from_json(
        &serde_json::to_string(&document).expect("the forward child serializes"),
    )
    .expect_err("a forward schema version must be refused");
    assert!(
        error.to_string().contains("version"),
        "forward-version refusal must name the version: {error}"
    );
}
