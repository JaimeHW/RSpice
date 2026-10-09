//! Build-specific command discovery without executing a circuit.
//!
//! Execution routes and result mappings are different contracts. In particular,
//! a named core analysis need not have a CLI handler, and a mapped JSON result
//! does not imply that a flat file can retain all its evidence.

use crate::cli::{CapabilitiesArgs, CliError, InputFormat, OutputFormat};
use clap::ValueEnum;
use rspice_core::engine::ControlCircuit;
use rspice_core::execution::{
    ANALYSIS_CAPABILITY_MATRIX, ANALYSIS_RESULT_DOCUMENT_SCHEMA, ANALYSIS_RESULT_DOCUMENT_VERSION,
    AnalysisKind, MappingStatus, analysis_result_kind,
};
use serde_json::{Value, json};

const SCOPE: &str = "Frontend routes and representations in this build; individual circuits still require admission. This report is not numerical or platform qualification.";

pub fn execute(args: CapabilitiesArgs, quiet: bool) -> Result<(), CliError> {
    if args.json {
        return crate::console::line(format_args!("{}", report()));
    }
    if quiet {
        return Ok(());
    }
    crate::console::line(format_args!("RSpice CLI capabilities"))?;
    crate::console::line(format_args!("{SCOPE}"))?;
    crate::console::line(format_args!(
        "{:<14} {:<13} {:<13} Result family (typed JSON)",
        "Analysis", "Execution", "Control"
    ))?;
    for kind in AnalysisKind::ALL {
        let execution = execution_support(kind);
        let control = control_support(kind);
        crate::console::line(format_args!(
            "{:<14} {:<13} {:<13} {}",
            kind.tag(),
            execution.status,
            control.status,
            analysis_result_kind(kind).map_or("unavailable", |family| family.tag()),
        ))?;
        if let Some(reason) = execution.reason {
            crate::console::line(format_args!("  {reason}"))?;
        }
        if let Some(reason) = control.reason {
            crate::console::line(format_args!("  Control: {reason}"))?;
        }
    }
    crate::console::line(format_args!("Additional workflows:"))?;
    crate::console::line(format_args!("  study_files: partial — {STUDY_SUPPORT}"))?;
    for (name, reason) in WORKFLOW_GAPS {
        crate::console::line(format_args!("  {name}: unsupported — {reason}"))?;
    }
    crate::console::line(format_args!(
        "Use --json for result mappings, format names, schema versions, and build identity."
    ))
}

#[derive(serde::Serialize)]
struct Support {
    status: &'static str,
    reason: Option<&'static str>,
}

impl Support {
    const fn supported() -> Self {
        Self {
            status: "supported",
            reason: None,
        }
    }

    const fn unsupported(reason: &'static str) -> Self {
        Self {
            status: "unsupported",
            reason: Some(reason),
        }
    }

    const fn partial(reason: &'static str) -> Self {
        Self {
            status: "partial",
            reason: Some(reason),
        }
    }
}

// Deliberately independent of analysis_result_kind: the presence of a schema
// must never turn into a claim that this executable has an execution route.
fn execution_support(kind: AnalysisKind) -> Support {
    match kind {
        AnalysisKind::ImplicitOp
        | AnalysisKind::Op
        | AnalysisKind::Dc
        | AnalysisKind::Ac
        | AnalysisKind::Tran
        | AnalysisKind::Noise
        | AnalysisKind::Sp
        | AnalysisKind::Stb
        | AnalysisKind::Distortion
        | AnalysisKind::PoleZero
        | AnalysisKind::Sensitivity
        | AnalysisKind::TransferFunction
        | AnalysisKind::Pss
        | AnalysisKind::Pac
        | AnalysisKind::Pxf
        | AnalysisKind::PNoise
        | AnalysisKind::Pstb
        | AnalysisKind::HarmonicBalance
        | AnalysisKind::Envelope
        | AnalysisKind::MonteCarlo
        | AnalysisKind::Fourier
        | AnalysisKind::Fft
        | AnalysisKind::Qpss
        | AnalysisKind::Qpac
        | AnalysisKind::Qpxf
        | AnalysisKind::Qpnoise
        | AnalysisKind::DcMatch => Support::supported(),
        AnalysisKind::Soa
        | AnalysisKind::Optimize
        | AnalysisKind::Psp
        | AnalysisKind::Hbsp
        | AnalysisKind::HbNoise => Support::partial(
            "Available through typed study run requests and retained study result JSON; declarative run and flat-format adapters are not exposed.",
        ),
        _ => Support::unsupported(
            "This CLI build has no declared execution route for this identity.",
        ),
    }
}

fn control_support(kind: AnalysisKind) -> Support {
    if ControlCircuit::analysis_command_name(kind).is_some() {
        Support::supported()
    } else if matches!(kind, AnalysisKind::Fourier | AnalysisKind::Fft) {
        Support::partial(
            "Authored post-processing runs with control tran/run; there is no direct command for this analysis.",
        )
    } else {
        Support::unsupported(
            "No direct control-host analysis handler; declarative execution is reported separately.",
        )
    }
}

fn mapping(status: MappingStatus) -> Value {
    json!({
        "status": match status {
            MappingStatus::Mapped => "mapped",
            MappingStatus::Partial(_) => "partial",
            MappingStatus::Unsupported(_) => "unsupported",
        },
        "reason": status.note(),
    })
}

const STUDY_SUPPORT: &str = "study check/plan/run prepare and execute version 1 typed task graphs with retained result JSON. Custom execution limits are enforced by basic analyses, STB, S-parameter, TF, distortion, and DC mismatch; remaining advanced routes reject overrides. Structured sweeps, external compiled model binding, and saved-study-result conversion are not exposed yet.";

const WORKFLOW_GAPS: [(&str, &str); 5] = [
    (
        "saved_result_processing",
        "Saved results can be converted and compared, but not remeasured, transformed, or plotted by a standalone command.",
    ),
    (
        "persistent_campaigns",
        "Parallel deck variants and corners exist; a durable multi-netlist job manifest and selective retry do not.",
    ),
    (
        "monte_carlo_continuation",
        "Indexed START batches exist; checkpoint continuation and population aggregation are not exposed by the CLI.",
    ),
    (
        "partial_spectrum_conversion",
        "FFT conversion preserves a complete transform; selecting a spectral subset is rejected.",
    ),
    (
        "veriloga_strict_lrm",
        "compile-va --strict returns an unsupported-capability error.",
    ),
];

fn format_names<T: ValueEnum>() -> Vec<String> {
    T::value_variants()
        .iter()
        .filter_map(|value| {
            value
                .to_possible_value()
                .map(|value| value.get_name().to_owned())
        })
        .collect()
}

fn report() -> Value {
    let analyses: Vec<_> = AnalysisKind::ALL
        .into_iter()
        .map(|kind| {
            json!({
                "id": kind.tag(),
                "execution": execution_support(kind),
                "control": control_support(kind),
                "control_command": ControlCircuit::analysis_command_name(kind),
                "result_family": analysis_result_kind(kind).map(|family| family.tag()),
            })
        })
        .collect();
    let result_families: Vec<_> = ANALYSIS_CAPABILITY_MATRIX
        .iter()
        .map(|row| {
            json!({
                "id": row.result.tag(),
                "scalar": mapping(row.cli.scalar),
                "step": mapping(row.cli.stepped),
                "temperature": mapping(row.cli.temperature),
            })
        })
        .collect();
    let mut workflows: Vec<_> = WORKFLOW_GAPS
        .iter()
        .map(|(id, reason)| {
            json!({
                "id": id,
                "support": Support::unsupported(reason),
            })
        })
        .collect();
    workflows.insert(
        0,
        json!({ "id": "study_files", "support": Support::partial(STUDY_SUPPORT) }),
    );
    json!({
        "schema": "rspice.capabilities",
        "schema_version": 1,
        "tool": super::health::tool_identity(),
        "scope": SCOPE,
        "analyses": analyses,
        "result_document": {
            "schema": ANALYSIS_RESULT_DOCUMENT_SCHEMA,
            "schema_version": ANALYSIS_RESULT_DOCUMENT_VERSION,
            "format": "json",
            "families": result_families,
        },
        "formats": {
            "input": format_names::<InputFormat>(),
            "output": format_names::<OutputFormat>(),
            "scope": "Command-level format names; result-specific representability is checked before export. Flat projections may omit typed evidence.",
        },
        "workflows": workflows,
    })
}
