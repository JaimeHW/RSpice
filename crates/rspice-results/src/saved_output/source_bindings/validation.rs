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
    pub(in crate::saved_output) fn validate<'a, I, J>(
        &self,
        receipt: &SavedOutputReceipt,
        analysis: &SavedOutputValidationRef<'a, I>,
        basis: Option<&J>,
    ) -> Result<(), String>
    where
        I: Iterator<Item = DcTraceView<'a>> + Clone,
        J: Iterator<Item = DcTraceView<'a>> + Clone,
    {
        let expected = saved_output_references(receipt.output_kind, &receipt.source_expression)?
            .ok_or_else(|| "this kind of saved output cannot name source signals".to_owned())?;
        if expected.iter().ne(self.references.keys()) {
            return Err(
                "saved-output sources do not match the signals its expression names".to_owned(),
            );
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
                    basis.map_or_else(
                        || analysis.waveforms.clone().any(|wave| wave.name == name),
                        |basis| basis.clone().any(|wave| wave.name == name),
                    )
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
                return Err("saved-output DC axis needs the DC sweep family".to_owned());
            }
            SavedOutputAxis::Waveform { name }
                if name.is_empty()
                    || family.is_some()
                    || analysis.dc_op.is_some()
                    || (deferred || basis.is_some()) && !retained(name) =>
            {
                return Err("saved-output axis does not name data the analysis kept".to_owned());
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
                        "saved-output source does not name data the analysis kept".to_owned()
                    );
                }
                SavedOutputBoundSource::DcQuantity { quantity } => {
                    let evidence = family.ok_or_else(|| {
                        "saved-output DC source needs the DC sweep family".to_owned()
                    })?;
                    let physical = evidence.quantities.get(*quantity).ok_or_else(|| {
                        "saved-output DC quantity index is out of bounds".to_owned()
                    })?;
                    if (physical.unit() == "A") != current {
                        return Err(
                            "saved-output DC source switches between a voltage and a current"
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
                            "saved output left to compute later is missing DC members".to_owned()
                        );
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
