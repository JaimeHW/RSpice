//! Indexed quantity lookup within each exact DC member's solved basis.
//! Build the sparse index once per analysis. Expression evaluation copies only
//! the samples it actually reads; raw outputs share their source sample owners.

use std::collections::HashMap;

use super::*;
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::dc_sweep::{DcSweepEvidence, DcSweepFamily, DcTraceView};

pub(super) struct Sources<'a, W> {
    evidence: Arc<DcSweepEvidence>,
    quantities: HashMap<(bool, String), usize>,
    curves: HashMap<(usize, usize), &'a W>,
    axes: HashMap<usize, &'a W>,
}

impl<'a, W: OutputWaveform> Sources<'a, W> {
    pub(super) fn new(
        analysis: &AnalysisResult<W>,
        source: &'a [W],
    ) -> Option<Result<Self, String>> {
        let Some(AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload else {
            return None;
        };
        if matches!(evidence.family, DcSweepFamily::Single) {
            return None;
        }
        Some(Self::build(Arc::clone(evidence), source))
    }

    fn build(evidence: Arc<DcSweepEvidence>, source: &'a [W]) -> Result<Self, String> {
        evidence.validate_retained_traces(source.iter().map(|waveform| DcTraceView {
            name: &waveform.as_ref().name,
            unit: waveform.as_ref().unit.as_deref(),
            x: &waveform.as_ref().x,
            sample_count: waveform.as_ref().y.len(),
            complex: waveform.as_ref().complex.is_some(),
        }))?;
        let by_name = source
            .iter()
            .map(|w| (w.as_ref().name.as_str(), w))
            .collect::<HashMap<_, _>>();
        let quantities = evidence
            .quantities
            .iter()
            .enumerate()
            .map(|(index, quantity)| {
                (
                    (quantity.unit() == "A", quantity.name().to_ascii_lowercase()),
                    index,
                )
            })
            .collect();
        let mut curves = HashMap::new();
        let mut axes = HashMap::new();
        for curve in evidence.curve_indices() {
            let name = evidence.trace_name(&evidence.quantities[curve.quantity], curve.member);
            let waveform = by_name[name.as_str()]; // Coverage was validated above.
            curves.insert((curve.quantity, curve.member), waveform);
            axes.entry(curve.member).or_insert(waveform);
        }
        Ok(Self {
            evidence,
            quantities,
            curves,
            axes,
        })
    }

    pub(super) fn quantity(&self, signal: &str) -> Option<usize> {
        let (current, name) = probe_identity(signal.trim());
        self.quantities
            .get(&(current, name.to_ascii_lowercase()))
            .copied()
    }

    pub(super) fn curve(&self, quantity: usize, member: usize) -> Option<&'a W> {
        self.curves.get(&(quantity, member)).copied()
    }

    pub(super) fn resolve(
        &self,
        contract: &PreparedSavedOutput,
        bindings: &rspice_results::saved_output::SavedOutputSourceBindings,
    ) -> Result<Vec<W>, String> {
        let mut outputs = Vec::new();
        for member in 0..self.evidence.member_count() {
            let context = super::bindings::Context {
                bindings,
                waveforms: &[],
                family: Some((self, member)),
                axis: self.axes.get(&member).copied(),
                complex_policy: contract.complex_policy(),
            };
            let result = match contract.kind() {
                SavedOutputKind::RawVoltageOrCurrent => probe::resolve_bound_raw_probe(
                    contract.source_expression(),
                    contract.name(),
                    self.axes.get(&member).copied(),
                    false,
                    |name| context.resolve(name),
                ),
                SavedOutputKind::DerivedExpression => resolve_derived_with(
                    contract.source_expression(),
                    contract.name(),
                    &context,
                    self.axes.get(&member).copied(),
                ),
                _ => Err("this output kind has no DC member source".to_owned()),
            };
            let mut output =
                result.map_err(|reason| format!("DC member {}: {reason}", member + 1))?;
            output.as_mut().name = self.evidence.member_trace_name(contract.name(), member);
            outputs.push(output);
        }
        Ok(outputs)
    }
}
