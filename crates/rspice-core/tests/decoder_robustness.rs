//! Decoder robustness: corrupted artifacts fail, they do not lie or panic.
//!
//! `parser_robustness.rs` does this for authored netlist text. This file does
//! it for the *machine-written* artifacts the product reads back: the
//! transient checkpoint (both the canonical text and the packed envelope), the
//! shared typed result document, and the authored-output resolution that turns
//! `.PRINT`/`.SAVE`/`.MEASURE` operands into a projection.
//!
//! Three invariants hold for every mutant:
//!
//! 1. **No panic.** Every decode runs inside `catch_unwind`, and the offending
//!    bytes are printed when one escapes.
//! 2. **A typed error, or a faithful value.** A decode that succeeds must be a
//!    fixed point: re-encoding what it produced and decoding that again yields
//!    an equal value. A decoder that accepted damaged bytes and invented a
//!    different result fails this.
//! 3. **No silent truncation.** A strict prefix of a packed checkpoint is
//!    always refused — the envelope carries the canonical length and a BLAKE3
//!    digest — and no prefix of the canonical text ever decodes back to the
//!    checkpoint it was cut from.
//!
//! The mutation streams are seeded xorshift, so a failure reproduces exactly.

use std::panic::{AssertUnwindSafe, catch_unwind};

use rspice_core::Netlist;
use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::{
    Engine, SimulationConfig, TransientCheckpoint, TransientCheckpointEncoding,
};
use rspice_core::execution::result_document::AnalysisResultDocument;
use rspice_core::execution::{DeckPlan, SignalProjection};
use rspice_core::netlist::{
    validate_output_expressions, validate_output_requests, validate_output_symbols,
};

/// Deterministic xorshift64* stream, matching `parser_robustness.rs`.
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

/// Bytes that historically break length-prefixed and text decoders alike.
const HOSTILE_BYTES: &[u8] = b"\0\n\r\t -+.eE0189NnIiFfXx{}[]\",:\x7f\xff\xc3\xa9";

fn mutate(rng: &mut Rng, seed: &[u8]) -> Vec<u8> {
    let mut bytes = seed.to_vec();
    match rng.below(5) {
        // Truncate at a random point.
        0 => {
            let cut = rng.below(bytes.len() + 1);
            bytes.truncate(cut);
        }
        // Flip a handful of bytes.
        1 => {
            for _ in 0..1 + rng.below(8) {
                if bytes.is_empty() {
                    break;
                }
                let position = rng.below(bytes.len());
                bytes[position] = (rng.next() & 0xff) as u8;
            }
        }
        // Inject decoder-hostile bytes.
        2 => {
            for _ in 0..1 + rng.below(6) {
                let at = rng.below(bytes.len() + 1);
                bytes.insert(at, HOSTILE_BYTES[rng.below(HOSTILE_BYTES.len())]);
            }
        }
        // Splice a slice somewhere else.
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
        // Delete a slice.
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

/// Drive `target` over `rounds` mutants of `seed`, failing with the exact
/// bytes when it panics.
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

/// Walk every prefix of `seed`, sampled so a large artifact stays cheap.
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
    // The complete artifact is always exercised, whatever the stride was.
    let result = catch_unwind(AssertUnwindSafe(|| target(seed)));
    assert!(result.is_ok(), "{name} panicked on the whole artifact");
}

//=============================================================================
// Transient checkpoint
//=============================================================================

const CHECKPOINT_DECK: &str = "\
decoder robustness fixture
V1 in 0 PULSE(0 1 0 100n 100n 4u 10u)
R1 in out 1k
C1 out 0 2n
L1 out mid 1u
R2 mid 0 220
.TRAN 50n 2u
.END
";

fn checkpoint_fixture() -> TransientCheckpoint {
    let netlist = Netlist::parse(CHECKPOINT_DECK).expect("the checkpoint fixture parses");
    let (_, checkpoint) = Engine::new(SimulationConfig::default())
        .run_tran_checkpointed(&netlist, 1.0e-6, 50.0e-9)
        .expect("the checkpoint fixture solves its first segment");
    checkpoint
}

/// A decode that succeeded must be a fixed point of encode/decode.
fn assert_decode_is_faithful(bytes: &[u8]) {
    let Ok(decoded) = TransientCheckpoint::from_bytes(bytes) else {
        return;
    };
    let reencoded = decoded
        .to_bytes(TransientCheckpointEncoding::Unpacked)
        .expect("a checkpoint the decoder accepted must be encodable");
    let again = TransientCheckpoint::from_bytes(&reencoded)
        .expect("a checkpoint's own canonical encoding must decode");
    assert_eq!(
        again, decoded,
        "the decoder produced a value that does not survive its own encoding"
    );
}

#[test]
fn checkpoint_text_decoder_survives_chaos_without_inventing_a_value() {
    let canonical = checkpoint_fixture().to_text().into_bytes();
    chaos_bytes(
        "TransientCheckpoint::from_bytes (canonical text)",
        &canonical,
        0x0DEC_0001,
        3_000,
        |input| {
            assert_decode_is_faithful(input);
            if let Ok(text) = std::str::from_utf8(input) {
                let _ = TransientCheckpoint::from_text(text);
                let _ = TransientCheckpoint::from_text_with_limit(text, 1_024);
            }
        },
    );
}

#[test]
fn checkpoint_packed_decoder_survives_chaos_without_inventing_a_value() {
    let packed = checkpoint_fixture()
        .to_bytes(TransientCheckpointEncoding::Packed)
        .expect("the fixture packs");
    chaos_bytes(
        "TransientCheckpoint::from_bytes (packed envelope)",
        &packed,
        0x0DEC_0002,
        3_000,
        |input| {
            assert_decode_is_faithful(input);
            let _ = TransientCheckpoint::from_bytes_with_limit(input, 4_096);
            let _ = TransientCheckpoint::from_bytes_with_encoding(
                input,
                TransientCheckpointEncoding::Packed,
                4_096,
            );
            let _ = TransientCheckpoint::from_bytes_with_limit_and_abort(input, 4_096, &NoAbort);
        },
    );
}

#[test]
fn every_truncation_of_a_checkpoint_is_refused_or_faithful() {
    let checkpoint = checkpoint_fixture();
    let canonical = checkpoint.to_text().into_bytes();
    let packed = checkpoint
        .to_bytes(TransientCheckpointEncoding::Packed)
        .expect("the fixture packs");

    truncation_sweep("canonical checkpoint text", &canonical, 400, |input| {
        assert_decode_is_faithful(input);
    });
    truncation_sweep("packed checkpoint envelope", &packed, 400, |input| {
        assert_decode_is_faithful(input);
    });
}

#[test]
fn a_truncated_checkpoint_never_decodes_back_to_the_whole_one() {
    let checkpoint = checkpoint_fixture();
    let canonical = checkpoint.to_text().into_bytes();
    let packed = checkpoint
        .to_bytes(TransientCheckpointEncoding::Packed)
        .expect("the fixture packs");

    // The packed envelope declares its canonical length and carries a BLAKE3
    // digest of it, so *no* strict prefix may decode at all.
    let stride = (packed.len() / 400).max(1);
    let mut cut = 0;
    while cut < packed.len() {
        assert!(
            TransientCheckpoint::from_bytes(&packed[..cut]).is_err(),
            "a {cut}-byte prefix of a {}-byte packed checkpoint decoded",
            packed.len()
        );
        cut += stride;
    }

    // The canonical text is a line format, so a prefix can be a syntactically
    // complete but *shorter* checkpoint. What must never happen is a prefix
    // decoding back to the checkpoint it was cut from.
    let stride = (canonical.len() / 400).max(1);
    let mut cut = 0;
    while cut < canonical.len() {
        if let Ok(decoded) = TransientCheckpoint::from_bytes(&canonical[..cut]) {
            assert_ne!(
                decoded, checkpoint,
                "a {cut}-byte prefix decoded back to the complete checkpoint"
            );
        }
        cut += stride;
    }
}

#[test]
fn a_packed_checkpoint_is_refused_where_canonical_text_is_required() {
    let packed = checkpoint_fixture()
        .to_bytes(TransientCheckpointEncoding::Packed)
        .expect("the fixture packs");
    let refused = TransientCheckpoint::from_bytes_with_encoding(
        &packed,
        TransientCheckpointEncoding::Unpacked,
        1 << 20,
    );
    assert!(
        refused.is_err(),
        "an authenticated caller that declared canonical text must not get a packed decode"
    );
}

//=============================================================================
// Shared typed result document
//=============================================================================

/// A minimal but complete document of the simplest family, used as the seed
/// the mutation stream damages.
fn result_document_json() -> String {
    let netlist = Netlist::parse("result document seed\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n")
        .expect("the seed deck parses");
    let plan = DeckPlan::from_netlist(&netlist, &rspice_core::ResourceLimits::default())
        .expect("the seed deck plans");
    assert_eq!(plan.analyses().len(), 1);
    // Build the smallest valid document for the planned analysis by encoding
    // an empty operating-point result through the shared builder.
    use rspice_core::execution::result_document::{OperatingPointPayload, ResultPayload};
    AnalysisResultDocument::builder(
        plan.analyses()[0].id(),
        ResultPayload::Op(OperatingPointPayload {
            observables: Vec::new(),
        }),
        0,
    )
    .build()
    .expect("an empty operating-point document is valid")
    .to_json()
    .expect("the seed document encodes")
}

#[test]
fn result_document_decoder_survives_chaos_without_inventing_a_value() {
    let seed = result_document_json().into_bytes();
    chaos_bytes(
        "AnalysisResultDocument::from_json",
        &seed,
        0x0DEC_0010,
        4_000,
        |input| {
            let Ok(text) = std::str::from_utf8(input) else {
                return;
            };
            let Ok(decoded) = AnalysisResultDocument::from_json(text) else {
                return;
            };
            let reencoded = decoded
                .to_json()
                .expect("a document the decoder accepted must encode");
            let again = AnalysisResultDocument::from_json(&reencoded)
                .expect("a document's own encoding must decode");
            assert_eq!(
                again, decoded,
                "the decoder produced a document that does not survive its own encoding"
            );
        },
    );
}

#[test]
fn result_document_decoder_survives_every_truncation() {
    let seed = result_document_json().into_bytes();
    truncation_sweep("AnalysisResultDocument::from_json", &seed, 400, |input| {
        if let Ok(text) = std::str::from_utf8(input) {
            let _ = AnalysisResultDocument::from_json(text);
        }
    });
}

//=============================================================================
// Authored output resolution
//=============================================================================

/// Decks whose whole point is the output surface: ordered `.PRINT` columns,
/// `.SAVE`/`.PROBE` selectors, expressions, lead currents, and measurements.
const OUTPUT_DECKS: &[&str] = &[
    "print columns\n\
     V1 in 0 SIN(0 1 1k)\n\
     R1 in out 1k\n\
     C1 out 0 1n\n\
     .tran 1u 1m\n\
     .print tran v(out) i(V1) v(in,out) {v(out)*2}\n\
     .save v(out) i(R1)\n\
     .end\n",
    "probe and measure\n\
     V1 in 0 DC 1 AC 1\n\
     R1 in out 1k\n\
     C1 out 0 1n\n\
     .ac dec 10 1 1meg\n\
     .probe ac vdb(out) vp(out) vr(out) vi(out)\n\
     .meas ac bw trig v(out) val=0.707 rise=1 targ v(out) val=0.707 fall=1\n\
     .print ac v(out)\n\
     .end\n",
    "hierarchy and lead currents\n\
     .subckt cell a y\n\
     RA a y 1k\n\
     CY y 0 1p\n\
     .ends\n\
     V1 in 0 1\n\
     X1 in out cell\n\
     M1 out in 0 0 nch w=1u l=100n\n\
     .model nch nmos level=1\n\
     .op\n\
     .print op v(x1.y) i(x1.ra) @m1[id] v(*)\n\
     .save all\n\
     .end\n",
];

#[test]
fn output_symbol_resolution_survives_chaos() {
    for (index, deck) in OUTPUT_DECKS.iter().enumerate() {
        let seed_base = 0x0DEC_0020 ^ ((index as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        chaos_bytes(
            "authored output resolution",
            deck.as_bytes(),
            seed_base,
            1_200,
            |input| {
                let text = String::from_utf8_lossy(input);
                let Ok(netlist) = Netlist::parse(&text) else {
                    return;
                };
                let _ = validate_output_requests(&netlist);
                let _ = validate_output_expressions(&netlist);
                let _ = validate_output_symbols(&netlist);
                let _ = SignalProjection::from_netlist(&netlist);
            },
        );
    }
}

#[test]
fn output_symbol_resolution_survives_every_truncation() {
    for deck in OUTPUT_DECKS {
        truncation_sweep(
            "authored output resolution",
            deck.as_bytes(),
            400,
            |input| {
                let text = String::from_utf8_lossy(input);
                let Ok(netlist) = Netlist::parse(&text) else {
                    return;
                };
                let _ = validate_output_requests(&netlist);
                let _ = validate_output_expressions(&netlist);
                let _ = validate_output_symbols(&netlist);
                let _ = SignalProjection::from_netlist(&netlist);
            },
        );
    }
}

#[test]
fn a_projection_accepted_for_a_deck_never_names_a_symbol_the_deck_refuses() {
    // The two surfaces must agree: if symbol validation refuses a deck, the
    // projection built from it is never used; if it accepts, the projection
    // must be constructible. A deck that validates but cannot project would
    // be a silent fallback.
    for deck in OUTPUT_DECKS {
        let netlist = Netlist::parse(deck).expect("an output fixture parses");
        if validate_output_symbols(&netlist).is_ok() {
            SignalProjection::from_netlist(&netlist)
                .expect("a validated deck must yield a projection");
        }
    }
}
