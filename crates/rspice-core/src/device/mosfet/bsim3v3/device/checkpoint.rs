//! Complete accepted BSIM3 limiter/evaluation state, independent of topology.
//! Engine-owned charge-integration histories accompany this record at capture.
use super::*;

pub(crate) const BSIM3_CHECKPOINT_RUNTIME_TAG: &str = "native-bsim3v3-accepted-v1";
pub(crate) const BSIM3_CHECKPOINT_VALUE_COUNT: usize = 61;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AcceptedBsim3NonlinearCheckpoint {
    pub(crate) instance_name: String,
    pub(crate) runtime_tag: String,
    pub(crate) values: [Value; BSIM3_CHECKPOINT_VALUE_COUNT],
    pub(crate) flags: [bool; 7],
    pub(crate) mode: i32,
    pub(crate) seed_evaluations: u8,
}

impl AcceptedBsim3NonlinearCheckpoint {
    pub(crate) fn validate_numeric_state(&self) -> Result<(), String> {
        if self.runtime_tag != BSIM3_CHECKPOINT_RUNTIME_TAG
            || self.instance_name.is_empty()
            || self.instance_name.chars().any(char::is_whitespace)
            || self.values.iter().any(|value| !value.is_finite())
            || self.values[0] < 0.0
            || !(-1..=1).contains(&self.mode)
            || self.seed_evaluations > 2
            || (!self.flags[5] && self.values[58..].iter().any(|v| v.to_bits() != 0))
            || (!self.flags[6] && self.values[30..58].iter().any(|v| v.to_bits() != 0))
        {
            return Err(format!(
                "BSIM3 '{}' accepted nonlinear state is invalid",
                self.instance_name
            ));
        }
        Ok(())
    }
}

impl Bsim3v3Device {
    pub(crate) fn accepted_nonlinear_checkpoint(
        &self,
    ) -> Result<AcceptedBsim3NonlinearCheckpoint, String> {
        let charge = self.op.charge.clone().unwrap_or_default();
        let raw = self.initial_off_seed_raw.unwrap_or(Bsim3v3Bias {
            vds: 0.0,
            vgs: 0.0,
            vbs: 0.0,
        });
        let state = AcceptedBsim3NonlinearCheckpoint {
            instance_name: self.name.clone(),
            runtime_tag: BSIM3_CHECKPOINT_RUNTIME_TAG.to_string(),
            values: [
                self.gmin,
                self.bias.vds,
                self.bias.vgs,
                self.bias.vbs,
                self.converged_ref.vds,
                self.converged_ref.vgs,
                self.converged_ref.vbs,
                self.von_prev,
                self.op.cd,
                self.op.csub,
                self.op.cbs,
                self.op.cbd,
                self.op.gm,
                self.op.gds,
                self.op.gmbs,
                self.op.gbd,
                self.op.gbs,
                self.op.gbbs,
                self.op.gbgs,
                self.op.gbds,
                self.op.von,
                self.op.vdsat,
                self.op.ueff,
                self.op.vgsteff,
                self.op.vdseff,
                self.op.abulk,
                self.op.rds,
                self.op.thetavth,
                self.op.ab_ov_vgst2vtm,
                self.op.qinv,
                charge.qgate,
                charge.qbulk,
                charge.qdrn,
                charge.qdrn_channel,
                charge.qbs,
                charge.qbd,
                charge.capbs,
                charge.capbd,
                charge.cggb,
                charge.cgdb,
                charge.cgsb,
                charge.cdgb,
                charge.cddb,
                charge.cdsb,
                charge.cbgb,
                charge.cbdb,
                charge.cbsb,
                charge.cgdo,
                charge.cgso,
                charge.cgbo,
                charge.qcheq,
                charge.cox_wl,
                charge.gtau,
                charge.cqgb,
                charge.cqdb,
                charge.cqsb,
                charge.cqbb,
                charge.taunet,
                raw.vds,
                raw.vgs,
                raw.vbs,
            ],
            flags: [
                self.has_history,
                self.limit_anchor_valid.get(),
                self.last_limited.get(),
                self.initial_off,
                self.initial_off_seed_pending,
                self.initial_off_seed_raw.is_some(),
                self.op.charge.is_some(),
            ],
            mode: self.op.mode,
            seed_evaluations: self.initial_off_seed_evaluations,
        };
        self.validate_accepted_nonlinear_checkpoint(&state)?;
        Ok(state)
    }

    pub(crate) fn validate_accepted_nonlinear_checkpoint(
        &self,
        state: &AcceptedBsim3NonlinearCheckpoint,
    ) -> Result<(), String> {
        if state.instance_name != self.name || state.flags[3] != self.initial_off {
            return Err(format!(
                "BSIM3 '{}' accepted state identity/startup mismatch",
                self.name
            ));
        }
        state.validate_numeric_state()
    }

    pub(crate) fn restore_accepted_nonlinear_checkpoint(
        &mut self,
        state: &AcceptedBsim3NonlinearCheckpoint,
    ) -> Result<(), String> {
        self.validate_accepted_nonlinear_checkpoint(state)?;
        let mut charge = Bsim3v3Charge::default();
        let mut raw = Bsim3v3Bias {
            vds: 0.0,
            vgs: 0.0,
            vbs: 0.0,
        };
        [
            self.gmin,
            self.bias.vds,
            self.bias.vgs,
            self.bias.vbs,
            self.converged_ref.vds,
            self.converged_ref.vgs,
            self.converged_ref.vbs,
            self.von_prev,
            self.op.cd,
            self.op.csub,
            self.op.cbs,
            self.op.cbd,
            self.op.gm,
            self.op.gds,
            self.op.gmbs,
            self.op.gbd,
            self.op.gbs,
            self.op.gbbs,
            self.op.gbgs,
            self.op.gbds,
            self.op.von,
            self.op.vdsat,
            self.op.ueff,
            self.op.vgsteff,
            self.op.vdseff,
            self.op.abulk,
            self.op.rds,
            self.op.thetavth,
            self.op.ab_ov_vgst2vtm,
            self.op.qinv,
            charge.qgate,
            charge.qbulk,
            charge.qdrn,
            charge.qdrn_channel,
            charge.qbs,
            charge.qbd,
            charge.capbs,
            charge.capbd,
            charge.cggb,
            charge.cgdb,
            charge.cgsb,
            charge.cdgb,
            charge.cddb,
            charge.cdsb,
            charge.cbgb,
            charge.cbdb,
            charge.cbsb,
            charge.cgdo,
            charge.cgso,
            charge.cgbo,
            charge.qcheq,
            charge.cox_wl,
            charge.gtau,
            charge.cqgb,
            charge.cqdb,
            charge.cqsb,
            charge.cqbb,
            charge.taunet,
            raw.vds,
            raw.vgs,
            raw.vbs,
        ] = state.values;
        self.has_history = state.flags[0];
        self.limit_anchor_valid.set(state.flags[1]);
        self.last_limited.set(state.flags[2]);
        self.initial_off_seed_pending = state.flags[4];
        self.initial_off_seed_raw = state.flags[5].then_some(raw);
        self.op.charge = state.flags[6].then_some(charge);
        self.op.mode = state.mode;
        self.initial_off_seed_evaluations = state.seed_evaluations;
        Ok(())
    }
}
