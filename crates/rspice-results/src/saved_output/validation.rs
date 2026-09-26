//! Receipt and materialization validation against borrowed retained data.

use super::*;
use crate::analysis_payload::AnalysisResultPayload;
use crate::analysis_type::AnalysisType;
use crate::dc_sweep::{DcSweepFamily, DcTraceView};
use crate::operating_point::DcOpResult;
use crate::provenance::AnalysisResultProvenance;
use std::collections::{HashMap, HashSet};

/// Borrowed evidence used to authenticate saved-output receipts without copying samples.
pub struct SavedOutputValidationRef<'a, I> {
    pub analysis_type: AnalysisType,
    pub waveforms: I,
    pub dc_op: Option<&'a DcOpResult>,
    pub result_payload: Option<&'a AnalysisResultPayload>,
    pub saved_output_receipts: &'a [SavedOutputReceipt],
    pub success: bool,
    pub provenance: Option<&'a AnalysisResultProvenance>,
}

impl<'a, I> SavedOutputValidationRef<'a, I>
where
    I: Iterator<Item = DcTraceView<'a>> + Clone,
{
    pub fn validate(&self, quasi_periodic_basis: Option<I>) -> Result<(), String> {
        if self.saved_output_receipts.is_empty() {
            return Ok(());
        }
        if self.saved_output_receipts.iter().any(|receipt| {
            matches!(
                receipt.status,
                SavedOutputMaterializationStatus::MaterializedDcFamily { .. }
            )
        }) {
            let Some(AnalysisResultPayload::DcSweep { evidence }) = &self.result_payload else {
                return Err("saved-output DC members require exact sweep evidence".to_owned());
            };
            evidence.validate()?;
        }
        let waveforms = self
            .waveforms
            .clone()
            .map(|waveform| (waveform.name, waveform))
            .collect::<HashMap<_, _>>();
        let mut identities = HashSet::new();
        let mut digests = HashSet::new();
        let mut raw_dc_axis: Option<&[f64]> = None;
        for receipt in self.saved_output_receipts {
            if receipt.output_kind != SavedOutputKind::DerivedExpression
                && !receipt.complex_policy.is_legacy()
            {
                return Err(
                    "complex expression policy applies only to derived-output receipts".to_owned(),
                );
            }
            if let Some(bindings) = &receipt.source_bindings {
                bindings.validate(receipt, self, quasi_periodic_basis.as_ref())?;
            }
            if !identities.insert(receipt.output_id) {
                return Err(format!(
                    "duplicate saved-output identity {}",
                    receipt.output_id
                ));
            }
            if !digests.insert(receipt.contract_digest) {
                return Err(format!(
                    "duplicate saved-output contract digest {}",
                    receipt.contract_digest
                ));
            }
            if receipt.name.trim().is_empty() || receipt.source_expression.trim().is_empty() {
                return Err(
                    "saved-output receipt has an empty name or source expression".to_owned(),
                );
            }
            if self
                .provenance
                .as_ref()
                .is_some_and(|source| receipt.analysis_id != source.source_instance_id())
            {
                return Err(
                    "saved-output analysis identity does not match result provenance".to_owned(),
                );
            }
            for (name, count) in receipt.status.materialized_waveforms() {
                if self.success && receipt.save_policy == SavedOutputPolicy::FailureDiagnosticsOnly
                {
                    return Err("saved-output receipt materializes failure-only data on a successful analysis".to_owned());
                }
                let waveform = waveforms.get(name).ok_or_else(|| {
                    format!("saved-output receipt names absent materialized waveform '{name}'")
                })?;
                if usize::try_from(count).ok() != Some(waveform.x.len()) {
                    return Err(format!(
                        "saved-output sample count does not match waveform '{name}'"
                    ));
                }
            }
            match &receipt.status {
                SavedOutputMaterializationStatus::MaterializedDcFamily { members } => {
                    let Some(AnalysisResultPayload::DcSweep { evidence }) = &self.result_payload
                    else {
                        return Err(
                            "saved-output DC members require exact sweep evidence".to_owned()
                        );
                    };
                    if self.analysis_type != AnalysisType::DcSweep
                        || matches!(evidence.family, DcSweepFamily::Single)
                        || members.is_empty()
                        || members.len() != evidence.member_count()
                    {
                        return Err(
                            "saved-output members do not cover their declared DC family".to_owned()
                        );
                    }
                    for (index, member) in members.iter().enumerate() {
                        if member.member != index
                            || member.waveform_name
                                != evidence.member_trace_name(&receipt.name, index)
                            || member.sample_count == 0
                        {
                            return Err("saved-output DC member identity, name, or sample count is inconsistent".to_owned());
                        }
                        if receipt.output_kind == SavedOutputKind::RawVoltageOrCurrent {
                            let waveform = &waveforms[member.waveform_name.as_str()];
                            let unit =
                                raw_probe_unit(&receipt.source_expression).ok_or_else(|| {
                                    "saved-output DC raw probe has no voltage/current identity"
                                        .to_owned()
                                })?;
                            if waveform.unit != Some(unit) || waveform.complex {
                                return Err(
                                    "saved-output DC raw member has an inconsistent quantity"
                                        .to_owned(),
                                );
                            }
                            if let Some(axis) = raw_dc_axis {
                                if !std::ptr::eq(axis, waveform.x) && axis != waveform.x {
                                    return Err(
                                        "saved-output DC raw members have different primary axes"
                                            .to_owned(),
                                    );
                                }
                            } else {
                                evidence.validate_axis(waveform.x)?;
                                raw_dc_axis = Some(waveform.x);
                            }
                        }
                    }
                }
                SavedOutputMaterializationStatus::SuppressedOnSuccess if !self.success => {
                    return Err(
                        "saved-output receipt suppresses failure diagnostics on a failed analysis"
                            .to_owned(),
                    );
                }
                SavedOutputMaterializationStatus::SuppressedOnSuccess
                    if receipt.save_policy != SavedOutputPolicy::FailureDiagnosticsOnly =>
                {
                    return Err("saved-output receipt uses success suppression for a non-diagnostic save policy".to_owned());
                }
                SavedOutputMaterializationStatus::Deferred
                    if receipt.save_policy != SavedOutputPolicy::OnDemandFromRetainedState =>
                {
                    return Err("saved-output receipt defers a save policy that requires immediate materialization".to_owned());
                }
                SavedOutputMaterializationStatus::Unavailable { reason }
                    if reason.trim().is_empty() =>
                {
                    return Err(
                        "saved-output receipt has an empty unavailability reason".to_owned()
                    );
                }
                SavedOutputMaterializationStatus::Materialized { .. }
                | SavedOutputMaterializationStatus::Deferred
                | SavedOutputMaterializationStatus::SuppressedOnSuccess
                | SavedOutputMaterializationStatus::Unavailable { .. } => {}
            }
        }
        if let (Some(axis), Some(AnalysisResultPayload::DcSweep { evidence })) =
            (raw_dc_axis, &self.result_payload)
            && let Some(curve) = evidence.curve_indices().next()
            && let Some(source) = waveforms.get(
                evidence
                    .trace_name(&evidence.quantities[curve.quantity], curve.member)
                    .as_str(),
            )
            && !std::ptr::eq(axis, source.x)
            && axis != source.x
        {
            return Err(
                "saved-output DC raw members differ from the retained solved primary axis"
                    .to_owned(),
            );
        }
        Ok(())
    }
}
