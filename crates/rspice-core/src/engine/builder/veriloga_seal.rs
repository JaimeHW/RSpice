//! A compiled Verilog-A artifact sealed to cross into another process.
//!
//! A sealed runtime carries its compiled model and canonical IR to the browser
//! worker, which installs it without compiling anything. The pair is written as
//! one record in the cache's compact encoding ([`super::veriloga_record`]),
//! the same bytes on every architecture, with what the artifact holds twice
//! written once:
//!
//! - A noise pass that differs from the ordinary pass is the ordinary pass
//!   with the noise shadows' steps inserted: a few dozen steps among the
//!   226,000 of BSIM-CMG 112. It is written as those insertions.
//! - The MIR's expression arena is a copy of the HIR's, and is written once.
//!
//! The reader restores both, so an opened artifact is exactly the one sealed.

use super::veriloga_record;
use rspice_veriloga::CompiledModel;
use rspice_veriloga::canonical_ir::CanonicalIrArtifact;
use rspice_veriloga::codegen::AssignmentStep;

/// A compiled model and its canonical IR sealed as one record, with the
/// fingerprint the engine knows the pair by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SealedVerilogAArtifact {
    pub bytes: Vec<u8>,
    /// The fingerprint an install compares installed artifacts by: the one
    /// the engine computes for the opened pair.
    pub fingerprint: [u8; 32],
}

/// The model's noise pass, at this path in [`SealedRecordRef`], is written
/// empty when it is written as insertions.
const NOISE_PASS: &[&str] = &["model", "noise_assignment_steps"];
/// The MIR's expression arena, written empty when it is the HIR's.
const MIR_EXPRESSIONS: &[&str] = &["canonical_ir", "mir", "expressions"];

#[derive(serde::Serialize)]
struct SealedRecordRef<'a> {
    model: &'a CompiledModel,
    /// The noise pass as the ordinary pass with each step inserted before the
    /// ordinary step at its index. `None` when the model carries it whole: it
    /// is empty (the ordinary pass, by the model's convention) or does not
    /// contain the ordinary pass in order.
    noise_insertions: Option<Vec<(usize, &'a AssignmentStep)>>,
    canonical_ir: &'a CanonicalIrArtifact,
    mir_expressions_are_hir: bool,
}

#[derive(serde::Deserialize)]
struct SealedRecord {
    model: CompiledModel,
    noise_insertions: Option<Vec<(usize, AssignmentStep)>>,
    canonical_ir: CanonicalIrArtifact,
    mir_expressions_are_hir: bool,
}

/// Seal `model` and `canonical_ir` for another process to open with
/// [`open_sealed_veriloga_artifact`].
pub fn seal_veriloga_artifact(
    model: &CompiledModel,
    canonical_ir: &CanonicalIrArtifact,
) -> Result<SealedVerilogAArtifact, String> {
    let error = |error: veriloga_record::RecordCodecError| {
        format!("failed to seal the Verilog-A artifact: {error}")
    };
    let noise_insertions = noise_pass_insertions(model).map_err(error)?;
    let mir_expressions_are_hir = veriloga_record::encode(&canonical_ir.mir.expressions)
        .map_err(error)?
        == veriloga_record::encode(&canonical_ir.hir.expressions).map_err(error)?;
    let mut omitted = Vec::with_capacity(2);
    if noise_insertions.is_some() {
        omitted.push(NOISE_PASS);
    }
    if mir_expressions_are_hir {
        omitted.push(MIR_EXPRESSIONS);
    }
    let bytes = veriloga_record::encode_omitting(
        &SealedRecordRef {
            model,
            noise_insertions,
            canonical_ir,
            mir_expressions_are_hir,
        },
        &omitted,
    )
    .map_err(error)?;
    Ok(SealedVerilogAArtifact {
        bytes,
        fingerprint: super::veriloga_cache::runtime_artifact_fingerprint(
            model,
            Some(canonical_ir),
        )?,
    })
}

/// The model and canonical IR [`seal_veriloga_artifact`] sealed in `bytes`.
pub fn open_sealed_veriloga_artifact(
    bytes: &[u8],
) -> Result<(CompiledModel, CanonicalIrArtifact), String> {
    let SealedRecord {
        mut model,
        noise_insertions,
        mut canonical_ir,
        mir_expressions_are_hir,
    } = veriloga_record::decode(bytes)
        .map_err(|error| format!("Sealed Verilog-A artifact is invalid: {error}"))?;
    if mir_expressions_are_hir {
        if !canonical_ir.mir.expressions.is_empty() {
            return Err(
                "Sealed Verilog-A artifact carries a MIR expression arena it says is the HIR's"
                    .to_owned(),
            );
        }
        canonical_ir.mir.expressions = canonical_ir.hir.expressions.clone();
    }
    if let Some(insertions) = noise_insertions {
        if !model.noise_assignment_steps.is_empty() {
            return Err(
                "Sealed Verilog-A artifact carries a noise pass and its insertions both".to_owned(),
            );
        }
        model.noise_assignment_steps = with_insertions(&model.assignment_steps, insertions)?;
    }
    Ok((model, canonical_ir))
}

/// The steps that make `model`'s ordinary pass its noise pass, when inserting
/// steps is all it takes: each with the index of the ordinary step it goes
/// before. Steps compare by their encoding, so a step is kept only where the
/// noise pass has exactly it.
fn noise_pass_insertions(
    model: &CompiledModel,
) -> Result<Option<Vec<(usize, &AssignmentStep)>>, veriloga_record::RecordCodecError> {
    let ordinary = &model.assignment_steps;
    let noise = &model.noise_assignment_steps;
    if noise.is_empty() || noise.len() < ordinary.len() {
        return Ok(None);
    }
    let room = noise.len() - ordinary.len();
    let mut insertions = Vec::with_capacity(room);
    let mut next = 0;
    let mut wanted = None;
    for step in noise {
        if next < ordinary.len() {
            let ordinary_step = match wanted.take() {
                Some(encoded) => encoded,
                None => veriloga_record::encode(&ordinary[next])?,
            };
            if veriloga_record::encode(step)? == ordinary_step {
                next += 1;
                continue;
            }
            wanted = Some(ordinary_step);
        }
        if insertions.len() == room {
            return Ok(None);
        }
        insertions.push((next, step));
    }
    Ok((next == ordinary.len()).then_some(insertions))
}

/// `ordinary` with each step inserted before the ordinary step at its index.
fn with_insertions(
    ordinary: &[AssignmentStep],
    insertions: Vec<(usize, AssignmentStep)>,
) -> Result<Vec<AssignmentStep>, String> {
    let mut steps = Vec::with_capacity(ordinary.len() + insertions.len());
    let mut copied = 0;
    for (index, step) in insertions {
        if index < copied || index > ordinary.len() {
            return Err(
                "Sealed Verilog-A artifact inserts a noise step out of order or past the end"
                    .to_owned(),
            );
        }
        steps.extend_from_slice(&ordinary[copied..index]);
        copied = index;
        steps.push(step);
    }
    steps.extend_from_slice(&ordinary[copied..]);
    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `shaped` reads a noise process, so the noise pass is the ordinary pass
    /// with the shadows' steps inserted.
    const NOISE_SHADOWED: &str = r#"
module noise_ddx(p, n);
    inout p, n;
    electrical p, n;
    real gain, process, shaped;
    analog begin
        gain = ddx(V(p, n) * V(p, n), V(p, n));
        process = white_noise(1.0, "shared");
        shaped = gain * process;
        I(p, n) <+ shaped;
    end
endmodule
"#;

    fn compiled(source: &str, module: &str) -> rspice_veriloga::RuntimeCompileReport {
        let bundle = rspice_veriloga::VirtualSourceBundle::new(
            "fixture.va",
            [rspice_veriloga::VirtualSourceFile::new(
                "fixture.va",
                source,
            )],
        )
        .expect("the fixture bundle is well formed");
        rspice_veriloga::VerilogACompiler::default()
            .compile_virtual_runtime(
                &bundle,
                module,
                rspice_veriloga::VirtualCompileLimits::default(),
            )
            .expect("the fixture compiles")
            .runtime
    }

    fn encoded<T: serde::Serialize + ?Sized>(value: &T) -> Vec<u8> {
        veriloga_record::encode(value).expect("the value encodes")
    }

    #[test]
    fn a_sealed_artifact_opens_to_exactly_the_pair_it_sealed() {
        let report = compiled(NOISE_SHADOWED, "noise_ddx");
        let (model, canonical_ir) = (&report.model, &report.canonical_ir);
        assert!(
            model.noise_assignment_steps.len() > model.assignment_steps.len(),
            "the fixture's noise pass inserts the shadows' steps"
        );
        let insertions = noise_pass_insertions(model)
            .expect("the passes encode")
            .expect("the noise pass is the ordinary pass with steps inserted");
        assert_eq!(
            insertions.len(),
            model.noise_assignment_steps.len() - model.assignment_steps.len()
        );

        let sealed = seal_veriloga_artifact(model, canonical_ir).expect("the pair seals");
        assert!(
            sealed.bytes.len() < encoded(&(model, canonical_ir)).len(),
            "the twin passes and arenas are written once"
        );
        let (opened_model, opened_ir) =
            open_sealed_veriloga_artifact(&sealed.bytes).expect("the artifact opens");
        assert_eq!(encoded(&opened_model), encoded(model));
        assert_eq!(encoded(&opened_ir), encoded(canonical_ir));
        assert_eq!(
            sealed.fingerprint,
            super::super::veriloga_cache::runtime_artifact_fingerprint(
                &opened_model,
                Some(&opened_ir)
            )
            .expect("the opened pair fingerprints"),
            "an install of the opened pair knows it by the sealed fingerprint"
        );
        assert_eq!(
            seal_veriloga_artifact(&opened_model, &opened_ir).expect("the opened pair seals"),
            sealed,
            "the seal is a fixed point of its own encoding"
        );
    }

    #[test]
    fn a_damaged_artifact_is_refused() {
        let report = compiled(NOISE_SHADOWED, "noise_ddx");
        let sealed =
            seal_veriloga_artifact(&report.model, &report.canonical_ir).expect("the pair seals");
        assert!(open_sealed_veriloga_artifact(&sealed.bytes[..sealed.bytes.len() - 1]).is_err());

        let model = &report.model;
        let late = (
            model.assignment_steps.len() + 1,
            &model.noise_assignment_steps[0],
        );
        let past_the_end = veriloga_record::encode_omitting(
            &SealedRecordRef {
                model,
                noise_insertions: Some(vec![late]),
                canonical_ir: &report.canonical_ir,
                mir_expressions_are_hir: false,
            },
            &[NOISE_PASS],
        )
        .expect("the record encodes");
        let refused = open_sealed_veriloga_artifact(&past_the_end)
            .expect_err("an insertion past the ordinary pass is refused");
        assert!(refused.contains("noise step"), "{refused}");

        let both = veriloga_record::encode(&SealedRecordRef {
            model,
            noise_insertions: Some(Vec::new()),
            canonical_ir: &report.canonical_ir,
            mir_expressions_are_hir: false,
        })
        .expect("the record encodes");
        assert!(open_sealed_veriloga_artifact(&both).is_err());
    }
}
