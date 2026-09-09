//! Indexed quantity lookup within each exact DC member's solved basis.
//! Build the sparse index once per analysis. Expression evaluation copies only
//! the samples it actually reads; raw outputs share their source sample owners.

use std::collections::HashMap;

use super::*;
use crate::state::{AnalysisResultPayload, DcSweepEvidence, DcSweepFamily, DcTraceView};

pub(super) struct Sources<'a> {
    evidence: Arc<DcSweepEvidence>,
    quantities: HashMap<(bool, String), usize>,
    curves: HashMap<(usize, usize), &'a WaveformData>,
    axes: HashMap<usize, &'a WaveformData>,
}

impl<'a> Sources<'a> {
    pub(super) fn new(
        analysis: &AnalysisResult,
        source: &'a [WaveformData],
    ) -> Option<Result<Self, String>> {
        let Some(AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload else {
            return None;
        };
        if matches!(evidence.family, DcSweepFamily::Single) {
            return None;
        }
        Some(Self::build(Arc::clone(evidence), source))
    }

    fn build(evidence: Arc<DcSweepEvidence>, source: &'a [WaveformData]) -> Result<Self, String> {
        evidence.validate_retained_traces(source.iter().map(|waveform| DcTraceView {
            name: &waveform.name,
            unit: waveform.unit.as_deref(),
            x: &waveform.x,
            sample_count: waveform.y.len(),
            complex: waveform.complex.is_some(),
        }))?;
        let by_name = source
            .iter()
            .map(|w| (w.name.as_str(), w))
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

    fn find(&self, member: usize, signal: &str) -> Option<&'a WaveformData> {
        let (current, name) = probe_identity(signal.trim());
        let quantity = self.quantities.get(&(current, name.to_ascii_lowercase()))?;
        self.curves.get(&(*quantity, member)).copied()
    }

    pub(super) fn resolve(
        &self,
        contract: &PreparedSavedOutput,
    ) -> Result<Vec<WaveformData>, String> {
        let mut outputs = Vec::new();
        for member in 0..self.evidence.member_count() {
            let result = match contract.kind {
                SavedOutputKind::RawVoltageOrCurrent => resolve_raw_probe_with(
                    &contract.source_expression,
                    &contract.name,
                    self.axes.get(&member).copied(),
                    false,
                    |name| self.find(member, name),
                ),
                SavedOutputKind::DerivedExpression => resolve_derived_with(
                    &contract.source_expression,
                    &contract.name,
                    &MemberContext {
                        sources: self,
                        member,
                    },
                    self.axes.get(&member).copied(),
                ),
                _ => Err("this output kind has no DC member source".to_owned()),
            };
            let mut output =
                result.map_err(|reason| format!("DC member {}: {reason}", member + 1))?;
            output.name = self.evidence.member_trace_name(&contract.name, member);
            outputs.push(output);
        }
        Ok(outputs)
    }
}

struct MemberContext<'a, 'b> {
    sources: &'b Sources<'a>,
    member: usize,
}

impl calculator::EvaluationContext for MemberContext<'_, '_> {
    fn get_waveform(
        &self,
        signal: &str,
        _dataset: Option<&str>,
    ) -> Result<CalcValue, calculator::EvaluationError> {
        if matches!(
            signal.to_ascii_uppercase().as_str(),
            "TIME" | "T" | "FREQ" | "FREQUENCY"
        ) {
            return self
                .sources
                .axes
                .get(&self.member)
                .map(|waveform| {
                    CalcValue::create_waveform(waveform.x.to_vec(), waveform.x.to_vec())
                })
                .ok_or_else(|| calculator::EvaluationError::IdentifierNotFound(signal.to_owned()));
        }
        if let Some(value) =
            calculator::canonical_ground_value(signal, self.sources.axes.get(&self.member).copied())
        {
            return value;
        }
        self.sources
            .find(self.member, signal)
            .map(|waveform| CalcValue::create_waveform(waveform.x.to_vec(), waveform.y.to_vec()))
            .ok_or_else(|| calculator::EvaluationError::IdentifierNotFound(signal.to_owned()))
    }
}
