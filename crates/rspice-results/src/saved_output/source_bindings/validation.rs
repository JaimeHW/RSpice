//! Validate exact saved-output source identities against retained data.

use super::*;
use crate::analysis_payload::AnalysisResultPayload;
use crate::dc_sweep::{DcSweepFamily, DcTraceView};
use crate::saved_output::validation::SavedOutputValidationRef;
use crate::saved_output::{
    SavedOutputMaterializationStatus, SavedOutputReceipt, device_current_probe,
    saved_output_references,
};

impl SavedOutputSourceBindings {
    pub(in crate::saved_output) fn validate<'a, I>(
        &self,
        receipt: &SavedOutputReceipt,
        analysis: &SavedOutputValidationRef<'a, I>,
        basis: Option<&I>,
    ) -> Result<(), String>
    where
        I: Iterator<Item = DcTraceView<'a>> + Clone,
    {
        let expected = saved_output_references(receipt.output_kind, &receipt.source_expression)?
            .ok_or_else(|| "this saved-output kind cannot carry source bindings".to_owned())?;
        if expected.iter().ne(self.references.keys()) {
            return Err("saved-output bindings do not match the expression references".to_owned());
        }
        let family = match &analysis.result_payload {
            Some(AnalysisResultPayload::DcSweep { evidence })
                if !matches!(evidence.family, DcSweepFamily::Single) =>
            {
                Some(evidence)
            }
            _ => None,
        };
        let deferred = receipt.status == SavedOutputMaterializationStatus::Deferred;
        let retained = |name: &str| {
            analysis.dc_op.as_ref().map_or_else(
                || {
                    basis
                        .cloned()
                        .unwrap_or_else(|| analysis.waveforms.clone())
                        .any(|wave| wave.name == name)
                },
                |op| {
                    op.node_voltages
                        .iter()
                        .chain(&op.branch_currents)
                        .any(|value| value.name == name)
                },
            )
        };
        match &self.axis {
            SavedOutputAxis::OperatingPoint if analysis.dc_op.is_none() => {
                return Err(
                    "saved-output OP axis requires solved operating-point tables".to_owned(),
                );
            }
            SavedOutputAxis::DcFamily if family.is_none() => {
                return Err("saved-output DC axis requires exact family evidence".to_owned());
            }
            SavedOutputAxis::Waveform { name }
                if name.is_empty()
                    || family.is_some()
                    || analysis.dc_op.is_some()
                    || (deferred || basis.is_some()) && !retained(name) =>
            {
                return Err("saved-output axis does not identify retained analysis data".to_owned());
            }
            _ => {}
        }
        for (reference, source) in &self.references {
            let current = reference.starts_with("i(") || device_current_probe(reference).is_some();
            match source {
                SavedOutputBoundSource::Ground if current => {
                    return Err("a current source cannot bind to the ground voltage".to_owned());
                }
                SavedOutputBoundSource::Waveform { name }
                    if name.is_empty()
                        || family.is_some()
                        || (deferred || basis.is_some()) && !retained(name) =>
                {
                    return Err(
                        "saved-output source does not identify retained analysis data".to_owned(),
                    );
                }
                SavedOutputBoundSource::DcQuantity { quantity } => {
                    let evidence = family.ok_or_else(|| {
                        "saved-output DC source requires exact family evidence".to_owned()
                    })?;
                    let physical = evidence.quantities.get(*quantity).ok_or_else(|| {
                        "saved-output DC quantity index is out of bounds".to_owned()
                    })?;
                    if (physical.unit() == "A") != current {
                        return Err(
                            "saved-output DC binding changes the voltage/current namespace"
                                .to_owned(),
                        );
                    }
                    if deferred
                        && !matches!(evidence.selection, crate::dc_sweep::DcCurveSelection::All)
                        && evidence
                            .curve_indices()
                            .filter(|curve| curve.quantity == *quantity)
                            .map(|curve| curve.member)
                            .collect::<std::collections::HashSet<_>>()
                            .len()
                            != evidence.member_count()
                    {
                        return Err(
                            "deferred saved-output source has missing DC members".to_owned()
                        );
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
