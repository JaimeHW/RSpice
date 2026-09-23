//! Portable accepted classic-MOS limiter and cached operating-point state.
use super::*;

pub(crate) const MOSFET_CHECKPOINT_RUNTIME_TAG: &str = "native-classic-mos-accepted-v1";
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AcceptedMosfetNonlinearCheckpoint {
    pub(crate) instance_name: String,
    pub(crate) runtime_tag: String,
    pub(crate) level: i32,
    pub(crate) junction_model: u8,
    pub(crate) region: u8,
    pub(crate) seed_evaluations: u8,
    // Accepted history, valid cache, authored OFF, pending OFF, legacy BSIM.
    pub(crate) flags: [bool; 5],
    pub(crate) values: [Value; 32],
}
impl AcceptedMosfetNonlinearCheckpoint {
    pub(crate) fn validate_numeric_state(&self) -> Result<(), String> {
        if self.runtime_tag != MOSFET_CHECKPOINT_RUNTIME_TAG
            || self.instance_name.is_empty()
            || self.instance_name.chars().any(char::is_whitespace)
            || self.level <= 0
            || self.junction_model > 1
            || self.region > 2
            || self.seed_evaluations > 2
            || self.values.iter().any(|v| !v.is_finite())
            || self.values[0] < 0.0
            || self.values[1] <= 0.0
        {
            return Err(format!(
                "MOSFET '{}' accepted nonlinear state is invalid",
                self.instance_name
            ));
        }
        Ok(())
    }
}
impl Mosfet {
    fn checkpoint_junction_model(&self) -> u8 {
        match self.body_junction_model {
            MosBodyJunctionModel::NgspiceReverseClamp => 0,
            MosBodyJunctionModel::XyceClassicLinearizedReverse => 1,
        }
    }
    pub(crate) fn accepted_nonlinear_checkpoint(
        &self,
    ) -> Result<AcceptedMosfetNonlinearCheckpoint, String> {
        let state = AcceptedMosfetNonlinearCheckpoint {
            instance_name: self.name.clone(),
            runtime_tag: MOSFET_CHECKPOINT_RUNTIME_TAG.to_string(),
            level: self.level,
            junction_model: self.checkpoint_junction_model(),
            region: match self.region {
                MosRegion::Cutoff => 0,
                MosRegion::Linear => 1,
                MosRegion::Saturation => 2,
            },
            seed_evaluations: self.initial_off_seed_evaluations,
            flags: [
                self.has_branch_history,
                self.linearization_cache_valid,
                self.initial_off,
                self.initial_off_seed_pending,
                self.uses_legacy_bsim(),
            ],
            values: [
                self.junction_gmin,
                self.vt,
                self.vgs,
                self.vds,
                self.vbs,
                self.eval_vgs,
                self.eval_vds,
                self.eval_vbs,
                self.id,
                self.gm,
                self.gds,
                self.gmb,
                self.gss,
                self.id_eq,
                self.ibs,
                self.gbs,
                self.ibd,
                self.gbd,
                self.vgs_prev,
                self.vds_prev,
                self.vbs_prev,
                self.eval_vgs_prev,
                self.eval_vds_prev,
                self.eval_vbs_prev,
                self.id_prev,
                self.gm_prev,
                self.gout_prev,
                self.gmb_prev,
                self.ibs_prev,
                self.gbs_prev,
                self.ibd_prev,
                self.gbd_prev,
            ],
        };
        self.validate_accepted_nonlinear_checkpoint(&state)?;
        Ok(state)
    }
    pub(crate) fn validate_accepted_nonlinear_checkpoint(
        &self,
        state: &AcceptedMosfetNonlinearCheckpoint,
    ) -> Result<(), String> {
        if state.instance_name != self.name
            || state.level != self.level
            || state.junction_model != self.checkpoint_junction_model()
            || state.flags[2] != self.initial_off
            || state.flags[4] != self.uses_legacy_bsim()
        {
            return Err(format!(
                "MOSFET '{}' accepted state identity/model/startup mismatch",
                self.name
            ));
        }
        state.validate_numeric_state()
    }
    pub(crate) fn restore_accepted_nonlinear_checkpoint(
        &mut self,
        state: &AcceptedMosfetNonlinearCheckpoint,
    ) -> Result<(), String> {
        self.validate_accepted_nonlinear_checkpoint(state)?;
        [
            self.junction_gmin,
            self.vt,
            self.vgs,
            self.vds,
            self.vbs,
            self.eval_vgs,
            self.eval_vds,
            self.eval_vbs,
            self.id,
            self.gm,
            self.gds,
            self.gmb,
            self.gss,
            self.id_eq,
            self.ibs,
            self.gbs,
            self.ibd,
            self.gbd,
            self.vgs_prev,
            self.vds_prev,
            self.vbs_prev,
            self.eval_vgs_prev,
            self.eval_vds_prev,
            self.eval_vbs_prev,
            self.id_prev,
            self.gm_prev,
            self.gout_prev,
            self.gmb_prev,
            self.ibs_prev,
            self.gbs_prev,
            self.ibd_prev,
            self.gbd_prev,
        ] = state.values;
        self.region = match state.region {
            0 => MosRegion::Cutoff,
            1 => MosRegion::Linear,
            _ => MosRegion::Saturation,
        };
        self.has_branch_history = state.flags[0];
        self.linearization_cache_valid = state.flags[1];
        self.initial_off_seed_pending = state.flags[3];
        self.initial_off_seed_evaluations = state.seed_evaluations;
        Ok(())
    }
}
