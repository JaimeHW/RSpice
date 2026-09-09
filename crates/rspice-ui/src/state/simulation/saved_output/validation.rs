//! The receipt contract shared by live retention and project reload.
//! A receipt identifies an authored output once, and its materialized members
//! must name actual samples in the same immutable analysis.

use std::collections::{HashMap, HashSet};

use super::*;
use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType, DcSweepFamily};

impl AnalysisResult {
    pub(in crate::state::simulation) fn validate_saved_output_receipts(
        &self,
    ) -> Result<(), String> {
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
            .iter()
            .map(|waveform| (waveform.name.as_str(), waveform))
            .collect::<HashMap<_, _>>();
        let mut identities = HashSet::new();
        let mut digests = HashSet::new();
        let mut raw_dc_axis: Option<&[f64]> = None;
        for receipt in &self.saved_output_receipts {
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
                        if receipt.output_kind == crate::state::SavedOutputKind::RawVoltageOrCurrent
                        {
                            let waveform = waveforms[member.waveform_name.as_str()];
                            let unit =
                                crate::state::workspace::raw_probe_unit(&receipt.source_expression)
                                    .ok_or_else(|| {
                                        "saved-output DC raw probe has no voltage/current identity"
                                            .to_owned()
                                    })?;
                            if waveform.unit.as_deref() != Some(unit) || waveform.complex.is_some()
                            {
                                return Err(
                                    "saved-output DC raw member has an inconsistent quantity"
                                        .to_owned(),
                                );
                            }
                            if let Some(axis) = raw_dc_axis {
                                if !std::ptr::eq(axis, waveform.x.as_slice())
                                    && axis != waveform.x.as_slice()
                                {
                                    return Err(
                                        "saved-output DC raw members have different primary axes"
                                            .to_owned(),
                                    );
                                }
                            } else {
                                super::super::dc_sweep::validate_axis(&waveform.x)?;
                                raw_dc_axis = Some(&waveform.x);
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
            && !std::ptr::eq(axis, source.x.as_slice())
            && axis != source.x.as_slice()
        {
            return Err(
                "saved-output DC raw members differ from the retained solved primary axis"
                    .to_owned(),
            );
        }
        Ok(())
    }
}
