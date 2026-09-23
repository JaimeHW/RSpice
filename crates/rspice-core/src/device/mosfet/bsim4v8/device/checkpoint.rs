//! Complete accepted BSIM4 limiter/evaluation state, independent of topology.
//! Engine-owned charge-integration histories accompany this record at capture.
use super::*;

pub(crate) const BSIM4_CHECKPOINT_RUNTIME_TAG: &str = "native-bsim4v8-accepted-v1";
pub(crate) const BSIM4_CHECKPOINT_VALUE_COUNT: usize = 122;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AcceptedBsim4NonlinearCheckpoint {
    pub(crate) instance_name: String,
    pub(crate) runtime_tag: String,
    pub(crate) values: [Value; BSIM4_CHECKPOINT_VALUE_COUNT],
    pub(crate) flags: [bool; 7],
    pub(crate) mode: i32,
    pub(crate) seed_evaluations: u8,
}

impl AcceptedBsim4NonlinearCheckpoint {
    pub(crate) fn validate_numeric_state(&self) -> Result<(), String> {
        if self.runtime_tag != BSIM4_CHECKPOINT_RUNTIME_TAG
            || self.instance_name.is_empty()
            || self.instance_name.chars().any(char::is_whitespace)
            || self.values.iter().any(|value| !value.is_finite())
            || self.values[0] < 0.0
            || !(-1..=1).contains(&self.mode)
            || self.seed_evaluations > 2
            || (!self.flags[5] && self.values[119..].iter().any(|v| v.to_bits() != 0))
            || (!self.flags[6] && self.values[77..119].iter().any(|v| v.to_bits() != 0))
        {
            return Err(format!(
                "BSIM4 '{}' accepted nonlinear state is invalid",
                self.instance_name
            ));
        }
        Ok(())
    }
}

impl Bsim4v8Device {
    pub(crate) fn accepted_nonlinear_checkpoint(
        &self,
    ) -> Result<AcceptedBsim4NonlinearCheckpoint, String> {
        let charge = self.op.charge.clone().unwrap_or_default();
        let raw = self.initial_off_seed_raw.unwrap_or(Bsim4v8Bias {
            vds: 0.0,
            vgs: 0.0,
            vbs: 0.0,
        });
        let state = AcceptedBsim4NonlinearCheckpoint {
            instance_name: self.name.clone(),
            runtime_tag: BSIM4_CHECKPOINT_RUNTIME_TAG.to_string(),
            values: [
                self.gmin,
                self.bias.vds,
                self.bias.vgs,
                self.bias.vbs,
                self.converged_ref.vds,
                self.converged_ref.vgs,
                self.converged_ref.vbs,
                self.von_prev,
                self.junction_bias.vbs,
                self.junction_bias.vbd,
                self.converged_junction_ref.vbs,
                self.converged_junction_ref.vbd,
                self.op.cd,
                self.op.csub,
                self.op.igidl,
                self.op.igisl,
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
                self.op.ggidld,
                self.op.ggidlg,
                self.op.ggidlb,
                self.op.ggidls,
                self.op.ggisls,
                self.op.ggislg,
                self.op.ggislb,
                self.op.ggisld,
                self.op.igcs,
                self.op.gigcsg,
                self.op.gigcsd,
                self.op.gigcss,
                self.op.gigcsb,
                self.op.igcd,
                self.op.gigcdg,
                self.op.gigcdd,
                self.op.gigcds,
                self.op.gigcdb,
                self.op.igs,
                self.op.gigsg,
                self.op.gigss,
                self.op.igd,
                self.op.gigdg,
                self.op.gigdd,
                self.op.igb,
                self.op.gigbg,
                self.op.gigbd,
                self.op.gigbs,
                self.op.gigbb,
                self.op.von,
                self.op.vdsat,
                self.op.output_vdsat,
                self.op.vdseff,
                self.op.vgsteff,
                self.op.ueff,
                self.op.abulk,
                self.op.rds,
                self.op.grdsw,
                self.op.thetavth,
                self.op.esat_l,
                self.op.ab_ov_vgst2vtm,
                self.op.idovvds,
                self.op.coxeff,
                self.op.nstar,
                self.op.qinv,
                self.op.noi_gd0,
                self.op.gcrg,
                self.op.gcrgg,
                self.op.gcrgd,
                self.op.gcrgs,
                self.op.gcrgb,
                charge.qgate,
                charge.qbulk,
                charge.qdrn,
                charge.qsrc,
                charge.qchqs,
                charge.cox_wl,
                charge.taunet,
                charge.gcrg,
                charge.gcrgg,
                charge.gcrgd,
                charge.gcrgs,
                charge.gcrgb,
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
                charge.csgb,
                charge.csdb,
                charge.cssb,
                charge.cgbb,
                charge.cdbb,
                charge.cbbb,
                charge.csbb,
                charge.cgdo,
                charge.qgdo,
                charge.cgso,
                charge.qgso,
                charge.cgbo,
                charge.qg_node,
                charge.qgmid_node,
                charge.qd_node,
                charge.qs_node,
                charge.qb_node,
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
        state: &AcceptedBsim4NonlinearCheckpoint,
    ) -> Result<(), String> {
        if state.instance_name != self.name || state.flags[3] != self.initial_off {
            return Err(format!(
                "BSIM4 '{}' accepted state identity/startup mismatch",
                self.name
            ));
        }
        state.validate_numeric_state()
    }

    pub(crate) fn restore_accepted_nonlinear_checkpoint(
        &mut self,
        state: &AcceptedBsim4NonlinearCheckpoint,
    ) -> Result<(), String> {
        self.validate_accepted_nonlinear_checkpoint(state)?;
        let mut charge = Bsim4v8Charge::default();
        let mut raw = Bsim4v8Bias {
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
            self.junction_bias.vbs,
            self.junction_bias.vbd,
            self.converged_junction_ref.vbs,
            self.converged_junction_ref.vbd,
            self.op.cd,
            self.op.csub,
            self.op.igidl,
            self.op.igisl,
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
            self.op.ggidld,
            self.op.ggidlg,
            self.op.ggidlb,
            self.op.ggidls,
            self.op.ggisls,
            self.op.ggislg,
            self.op.ggislb,
            self.op.ggisld,
            self.op.igcs,
            self.op.gigcsg,
            self.op.gigcsd,
            self.op.gigcss,
            self.op.gigcsb,
            self.op.igcd,
            self.op.gigcdg,
            self.op.gigcdd,
            self.op.gigcds,
            self.op.gigcdb,
            self.op.igs,
            self.op.gigsg,
            self.op.gigss,
            self.op.igd,
            self.op.gigdg,
            self.op.gigdd,
            self.op.igb,
            self.op.gigbg,
            self.op.gigbd,
            self.op.gigbs,
            self.op.gigbb,
            self.op.von,
            self.op.vdsat,
            self.op.output_vdsat,
            self.op.vdseff,
            self.op.vgsteff,
            self.op.ueff,
            self.op.abulk,
            self.op.rds,
            self.op.grdsw,
            self.op.thetavth,
            self.op.esat_l,
            self.op.ab_ov_vgst2vtm,
            self.op.idovvds,
            self.op.coxeff,
            self.op.nstar,
            self.op.qinv,
            self.op.noi_gd0,
            self.op.gcrg,
            self.op.gcrgg,
            self.op.gcrgd,
            self.op.gcrgs,
            self.op.gcrgb,
            charge.qgate,
            charge.qbulk,
            charge.qdrn,
            charge.qsrc,
            charge.qchqs,
            charge.cox_wl,
            charge.taunet,
            charge.gcrg,
            charge.gcrgg,
            charge.gcrgd,
            charge.gcrgs,
            charge.gcrgb,
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
            charge.csgb,
            charge.csdb,
            charge.cssb,
            charge.cgbb,
            charge.cdbb,
            charge.cbbb,
            charge.csbb,
            charge.cgdo,
            charge.qgdo,
            charge.cgso,
            charge.qgso,
            charge.cgbo,
            charge.qg_node,
            charge.qgmid_node,
            charge.qd_node,
            charge.qs_node,
            charge.qb_node,
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
