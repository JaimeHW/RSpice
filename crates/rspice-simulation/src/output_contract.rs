//! Immutable saved-output contracts compiled against exact analysis identities.

use crate::execution_identity::analysis_kind_tag;
use rspice_app_types::canonical::content_digest;
use rspice_app_types::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SavedOutputId};
use rspice_results::saved_output::{
    SavedOutputKind, SavedOutputMaterializationStatus, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputReceipt, SavedOutputStreaming,
};
use rspice_simulation_contract::analysis_run_type::AnalysisRunType;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::saved_output::{SavedOutput, SavedOutputCompatibility};
use std::sync::Arc;

mod bindings;
mod preflight;
pub mod selection;
pub use bindings::SourceCandidate;
pub use preflight::{
    SavedOutputPreflightReport, SavedOutputSemanticStatus, SavedOutputStorageEstimate,
    preflight_saved_output, retained_engine_source_upper_bound_bytes,
};

const MAX_SELECTED_POINT_COUNT: usize = 10_000_000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransientSelectionGrid {
    pub start: f64,
    pub step: f64,
    pub stop: f64,
}

/// One output contract resolved for exactly one prepared analysis task.
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedSavedOutput {
    output_id: SavedOutputId,
    output_revision: ObjectRevision,
    analysis_id: AnalysisInstanceId,
    kind: SavedOutputKind,
    name: String,
    source_expression: String,
    complex_policy: rspice_results::saved_output::ComplexExpressionPolicy,
    policy: SavedOutputPolicy,
    precision: SavedOutputPrecision,
    streaming: SavedOutputStreaming,
    display_intent: rspice_results::saved_output::SavedOutputDisplayIntent,
    selection_grid: Option<TransientSelectionGrid>,
    candidates: Option<Arc<bindings::Candidates>>,
    digest: ContentDigest,
}

impl PreparedSavedOutput {
    pub fn prepare(
        output: &SavedOutput,
        analysis_id: AnalysisInstanceId,
        spec: &AnalysisSpec,
    ) -> Result<Option<Self>, String> {
        output.validate()?;
        // Automatic node probes belong to ordinary circuit waveforms. Other
        // families retain their native spectra, study statistics or reports.
        // Explicit output requests still use the full compatibility contract.
        if output.origin == rspice_simulation_contract::saved_output::SavedOutputOrigin::Automatic
            && (!matches!(
                spec.run_type(),
                AnalysisRunType::DcOp
                    | AnalysisRunType::DcSweep
                    | AnalysisRunType::Transient
                    | AnalysisRunType::Ac
                    | AnalysisRunType::TransientNoise
            ) || matches!(spec, AnalysisSpec::AcData { frequencies, table_options, .. }
                if table_options.from_netlist && frequencies.is_empty()))
        {
            return Ok(None);
        }
        let selected = match &output.compatible_analyses {
            SavedOutputCompatibility::OpTranAc => {
                matches!(
                    spec.run_type(),
                    AnalysisRunType::DcOp | AnalysisRunType::Transient | AnalysisRunType::Ac
                ) && output_kind_supports_run_type(output.kind, spec.run_type())
            }
            SavedOutputCompatibility::AllCompatibleAnalyses => {
                output_kind_supports_run_type(output.kind, spec.run_type())
            }
            SavedOutputCompatibility::SelectedAnalysis {
                analysis_id: selected,
            } => {
                if *selected != analysis_id {
                    false
                } else if !output_kind_supports_run_type(output.kind, spec.run_type()) {
                    return Err(format!(
                        "saved output '{}' selects analysis {analysis_id}, but {} outputs are incompatible with {}",
                        output.name,
                        output.kind.label(),
                        spec.run_type().display_name()
                    ));
                } else {
                    true
                }
            }
        };
        if !selected {
            return Ok(None);
        }
        if !output_kind_supports_run_type(output.kind, spec.run_type()) {
            return Err(format!(
                "saved output '{}' cannot be materialized by {}",
                output.name,
                spec.run_type().display_name()
            ));
        }
        validate_static_contract_semantics(output, spec)?;

        let selection_grid = match (output.save_policy, spec) {
            (
                SavedOutputPolicy::SelectedAndFinalPoints,
                AnalysisSpec::Transient {
                    stop_time,
                    step_time,
                    start_time,
                    ..
                }
                | AnalysisSpec::TransientNoise {
                    stop_time,
                    step_time,
                    start_time,
                    ..
                },
            ) => Some(TransientSelectionGrid {
                start: *start_time,
                step: *step_time,
                stop: *stop_time,
            }),
            _ => None,
        };
        if let Some(grid) = selection_grid {
            validate_selection_grid(grid)?;
        }
        let digest = output_contract_digest(output, analysis_id, spec, selection_grid);
        Ok(Some(Self {
            output_id: output.id,
            output_revision: output.revision,
            analysis_id,
            kind: output.kind,
            name: output.name.clone(),
            source_expression: output.source_expression.clone(),
            complex_policy: output.complex_policy,
            policy: output.save_policy,
            precision: output.stored_precision,
            streaming: output.streaming,
            display_intent: output.display_intent,
            selection_grid,
            candidates: None,
            digest,
        }))
    }

    /// Recompile this immutable contract for a deterministically derived
    /// analysis identity. PVT expansion happens after plan outputs have been
    /// prepared, so copying the old digest or identity would make the
    /// expanded task fail authentication (or, worse, retain data under the
    /// wrong analysis). The reconstructed authored contract deliberately uses
    /// `AllCompatibleAnalyses`: selection was already proven when the original
    /// prepared contract was created, and every derived task has the same OP
    /// run type.
    pub fn rebind_analysis(
        &self,
        analysis_id: AnalysisInstanceId,
        spec: &AnalysisSpec,
    ) -> Result<Self, String> {
        let output = SavedOutput {
            id: self.output_id,
            revision: self.output_revision,
            origin: rspice_simulation_contract::saved_output::SavedOutputOrigin::Plan,
            display_intent: self.display_intent,
            kind: self.kind,
            name: self.name.clone(),
            source_expression: self.source_expression.clone(),
            complex_policy: self.complex_policy,
            compatible_analyses: SavedOutputCompatibility::AllCompatibleAnalyses,
            save_policy: self.policy,
            stored_precision: self.precision,
            streaming: self.streaming,
        };
        Self::prepare(&output, analysis_id, spec)?.ok_or_else(|| {
            format!(
                "saved output '{}' is incompatible with derived analysis {analysis_id}",
                self.name
            )
        })
    }

    pub const fn output_id(&self) -> SavedOutputId {
        self.output_id
    }

    pub const fn output_revision(&self) -> ObjectRevision {
        self.output_revision
    }

    pub const fn analysis_id(&self) -> AnalysisInstanceId {
        self.analysis_id
    }

    pub const fn kind(&self) -> SavedOutputKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_expression(&self) -> &str {
        &self.source_expression
    }

    pub const fn policy(&self) -> SavedOutputPolicy {
        self.policy
    }

    pub const fn precision(&self) -> SavedOutputPrecision {
        self.precision
    }

    pub const fn streaming(&self) -> SavedOutputStreaming {
        self.streaming
    }

    pub const fn selection_grid(&self) -> Option<TransientSelectionGrid> {
        self.selection_grid
    }

    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    pub const fn complex_policy(&self) -> rspice_results::saved_output::ComplexExpressionPolicy {
        self.complex_policy
    }

    pub const fn display_intent(&self) -> rspice_results::saved_output::SavedOutputDisplayIntent {
        self.display_intent
    }

    /// Restore the output projection recorded by a deferred result receipt.
    pub fn from_deferred_receipt(receipt: &SavedOutputReceipt) -> Result<Self, String> {
        if receipt.status != SavedOutputMaterializationStatus::Deferred {
            return Err("saved-output receipt is not deferred".to_owned());
        }
        Ok(Self {
            output_id: receipt.output_id,
            output_revision: receipt.output_revision,
            analysis_id: receipt.analysis_id,
            kind: receipt.output_kind,
            name: receipt.name.clone(),
            source_expression: receipt.source_expression.clone(),
            complex_policy: receipt.complex_policy,
            policy: receipt.save_policy,
            precision: receipt.stored_precision,
            streaming: receipt.streaming,
            display_intent: receipt.display_intent,
            selection_grid: None,
            candidates: None,
            digest: receipt.contract_digest,
        })
    }

    pub fn receipt(
        &self,
        status: SavedOutputMaterializationStatus,
        source_bindings: Option<rspice_results::saved_output::SavedOutputSourceBindings>,
    ) -> SavedOutputReceipt {
        SavedOutputReceipt {
            output_id: self.output_id,
            output_revision: self.output_revision,
            analysis_id: self.analysis_id,
            contract_digest: self.digest,
            name: self.name.clone(),
            source_expression: self.source_expression.clone(),
            complex_policy: self.complex_policy,
            output_kind: self.kind,
            save_policy: self.policy,
            stored_precision: self.precision,
            streaming: self.streaming,
            display_intent: self.display_intent,
            source_bindings,
            status,
        }
    }
}

pub fn compile_saved_output_contracts<'a>(
    output: &SavedOutput,
    analyses: impl IntoIterator<Item = (AnalysisInstanceId, &'a AnalysisSpec)>,
) -> Result<Vec<PreparedSavedOutput>, String> {
    let mut contracts = Vec::new();
    for (analysis_id, spec) in analyses {
        if let Some(contract) = PreparedSavedOutput::prepare(output, analysis_id, spec)? {
            contracts.push(contract);
        }
    }
    if contracts.is_empty()
        && output.origin != rspice_simulation_contract::saved_output::SavedOutputOrigin::Automatic
    {
        return Err(format!(
            "saved output '{}' has no compatible enabled analysis",
            output.name
        ));
    }
    Ok(contracts)
}

fn output_kind_supports_run_type(kind: SavedOutputKind, run_type: AnalysisRunType) -> bool {
    match kind {
        SavedOutputKind::DerivedExpression
            if matches!(run_type, AnalysisRunType::Qpxf | AnalysisRunType::Qpnoise) =>
        {
            true
        }
        SavedOutputKind::RawVoltageOrCurrent | SavedOutputKind::DerivedExpression => matches!(
            run_type,
            AnalysisRunType::DcOp
                | AnalysisRunType::DcSweep
                | AnalysisRunType::Ac
                | AnalysisRunType::Transient
                | AnalysisRunType::Noise
                | AnalysisRunType::MonteCarlo
                | AnalysisRunType::Parametric
                | AnalysisRunType::Corner
                | AnalysisRunType::Optimization
                | AnalysisRunType::Soa
                | AnalysisRunType::SParameter
                | AnalysisRunType::Pac
                | AnalysisRunType::Pnoise
                | AnalysisRunType::Pxf
                | AnalysisRunType::Pss
                | AnalysisRunType::Qpss
                | AnalysisRunType::Qpac
                | AnalysisRunType::HarmonicBalance
                | AnalysisRunType::Envelope
                | AnalysisRunType::Fourier
                | AnalysisRunType::TransientNoise
        ),
        SavedOutputKind::DeviceOperatingPointQuantity => matches!(run_type, AnalysisRunType::DcOp),
        SavedOutputKind::NoiseContributor => matches!(
            run_type,
            AnalysisRunType::Noise
                | AnalysisRunType::Pnoise
                | AnalysisRunType::Qpnoise
                | AnalysisRunType::Hbnoise
                | AnalysisRunType::TransientNoise
        ),
        SavedOutputKind::RfPortQuantity => matches!(
            run_type,
            AnalysisRunType::SParameter | AnalysisRunType::Hbsp | AnalysisRunType::Psp
        ),
    }
}

fn validate_static_contract_semantics(
    output: &SavedOutput,
    spec: &AnalysisSpec,
) -> Result<(), String> {
    if output.kind != SavedOutputKind::RfPortQuantity {
        return Ok(());
    }
    let (output_port, input_port) = parse_rf_port(&output.source_expression)?;
    let port_count = match spec {
        // SP's configured ports are a fallback. Authored hierarchical ports
        // may replace them, so only the solved circuit can establish this bound.
        AnalysisSpec::SParameter { .. } => return Ok(()),
        AnalysisSpec::Hbsp { ports, .. } | AnalysisSpec::Psp { ports, .. } => ports.len(),
        _ => {
            return Err(format!(
                "saved output '{}' requires an RF-port analysis",
                output.name
            ));
        }
    };
    if output_port > port_count || input_port > port_count {
        return Err(format!(
            "saved output '{}' references S({output_port},{input_port}), but {} has {port_count} configured port{}",
            output.name,
            spec.run_type().display_name(),
            if port_count == 1 { "" } else { "s" }
        ));
    }
    Ok(())
}

pub fn parse_probe(expression: &str) -> Result<(String, Vec<String>), String> {
    let expression = expression.trim();
    let open = expression
        .find('(')
        .ok_or_else(|| "probe is missing '('".to_owned())?;
    let inner = expression[open + 1..]
        .strip_suffix(')')
        .ok_or_else(|| "probe is missing ')'".to_owned())?;
    Ok((
        expression[..open].trim().to_owned(),
        inner
            .split(',')
            .map(|value| value.trim().to_owned())
            .collect(),
    ))
}

pub fn parse_rf_port(expression: &str) -> Result<(usize, usize), String> {
    let (function, arguments) = parse_probe(expression)?;
    if !function.eq_ignore_ascii_case("S") || arguments.len() != 2 {
        return Err("RF quantity must use S(output, input)".to_owned());
    }
    let output = arguments[0]
        .parse::<usize>()
        .map_err(|_| "RF output port must be a positive integer".to_owned())?;
    let input = arguments[1]
        .parse::<usize>()
        .map_err(|_| "RF input port must be a positive integer".to_owned())?;
    if output == 0 || input == 0 {
        return Err("RF ports are one-based".to_owned());
    }
    Ok((output, input))
}

pub fn probe_identity(name: &str) -> (bool, &str) {
    let name = name.trim_matches('|');
    if rspice_results::saved_output::device_current_probe(name).is_some() {
        return (true, name);
    }
    if let Some(inner) = name.get(2..).and_then(|inner| inner.strip_suffix(')')) {
        if name
            .get(..2)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("I("))
        {
            return (true, inner);
        }
        if name
            .get(..2)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
        {
            return (false, inner);
        }
    }
    (false, name)
}

pub fn validate_selection_grid(grid: TransientSelectionGrid) -> Result<(), String> {
    if !grid.start.is_finite()
        || !grid.step.is_finite()
        || !grid.stop.is_finite()
        || grid.start < 0.0
        || grid.step <= 0.0
        || grid.stop < grid.start
    {
        return Err("selected-point transient grid is invalid".to_owned());
    }
    let count = ((grid.stop - grid.start) / grid.step).floor() + 2.0;
    if !count.is_finite() || count > MAX_SELECTED_POINT_COUNT as f64 {
        return Err(format!(
            "selected-point grid exceeds the {MAX_SELECTED_POINT_COUNT}-sample safety limit"
        ));
    }
    Ok(())
}

fn output_contract_digest(
    output: &SavedOutput,
    analysis_id: AnalysisInstanceId,
    spec: &AnalysisSpec,
    grid: Option<TransientSelectionGrid>,
) -> ContentDigest {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(output.id.as_uuid().as_bytes());
    bytes.extend_from_slice(&output.revision.get().to_be_bytes());
    bytes.extend_from_slice(analysis_id.as_uuid().as_bytes());
    bytes.push(analysis_kind_tag(spec));
    bytes.push(output_kind_tag(output.kind));
    append_string(&mut bytes, &output.name);
    append_string(&mut bytes, &output.source_expression);
    bytes.push(policy_tag(output.save_policy));
    bytes.push(precision_tag(output.stored_precision));
    bytes.push(streaming_tag(output.streaming));
    bytes.push(match output.display_intent {
        rspice_results::saved_output::SavedOutputDisplayIntent::Plot => 0,
        rspice_results::saved_output::SavedOutputDisplayIntent::DataBrowserOnly => 1,
    });
    if let Some(grid) = grid {
        bytes.push(1);
        bytes.extend_from_slice(&grid.start.to_bits().to_be_bytes());
        bytes.extend_from_slice(&grid.step.to_bits().to_be_bytes());
        bytes.extend_from_slice(&grid.stop.to_bits().to_be_bytes());
    } else {
        bytes.push(0);
    }
    // Historical contracts retain their exact digest. The new domain binds
    // rectangular evaluation without reinterpreting an old receipt.
    let domain = if output.complex_policy.is_legacy() {
        "rspice.prepared-saved-output/v1"
    } else {
        "rspice.prepared-saved-output/rectangular-v2"
    };
    content_digest(domain, &bytes)
}

fn append_string(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

pub const fn output_kind_tag(kind: SavedOutputKind) -> u8 {
    match kind {
        SavedOutputKind::RawVoltageOrCurrent => 0,
        SavedOutputKind::DerivedExpression => 1,
        SavedOutputKind::DeviceOperatingPointQuantity => 2,
        SavedOutputKind::NoiseContributor => 3,
        SavedOutputKind::RfPortQuantity => 4,
    }
}

pub const fn policy_tag(policy: SavedOutputPolicy) -> u8 {
    match policy {
        SavedOutputPolicy::EveryAcceptedPoint => 0,
        SavedOutputPolicy::SelectedAndFinalPoints => 1,
        SavedOutputPolicy::OnDemandFromRetainedState => 2,
        SavedOutputPolicy::FailureDiagnosticsOnly => 3,
    }
}

pub const fn precision_tag(precision: SavedOutputPrecision) -> u8 {
    match precision {
        SavedOutputPrecision::FullSourcePrecision => 0,
        SavedOutputPrecision::DisplayCacheWithFullSourcePrecision => 1,
    }
}

pub const fn streaming_tag(streaming: SavedOutputStreaming) -> u8 {
    match streaming {
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation => 0,
        SavedOutputStreaming::StoreOnly => 1,
    }
}
