#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;
#[path = "stamp_blocks_1.rs"]
mod stamp_blocks_1;
#[path = "stamp_blocks_2.rs"]
mod stamp_blocks_2;
#[path = "stamp_blocks_3.rs"]
mod stamp_blocks_3;
#[path = "stamp_blocks_4.rs"]
mod stamp_blocks_4;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    ddt_previous_value_scale: f64,
    ddt_older_value_scale: f64,
    ddt_previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        let result = value * ddt_scale
            - previous_value * ddt_previous_value_scale
            - older_value * ddt_older_value_scale
            - derivative_previous[slot] * ddt_previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        current[slot] = value;
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

#[inline]
fn ddt_jacobian(ddt_active: bool, ddt_scale: f64, derivative: f64) -> f64 {
    if ddt_active {
        derivative * ddt_scale
    } else {
        0.0
    }
}

#[inline]
fn eval_idt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    ddt_active: bool,
    idt_scale: f64,
    slot: usize,
    value: f64,
    ic: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated idt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { ic };
    let current_value = if ddt_active {
        previous_value + value * idt_scale
    } else {
        ic
    };
    current[slot] = current_value;
    if !ddt_active {
        previous[slot] = current_value;
        initialized[slot] = true;
    }
    current_value
}

#[inline]
fn idt_jacobian(timestep: f64, derivative: f64) -> f64 {
    if timestep.abs() > Instance::DDT_EPSILON {
        derivative * timestep
    } else {
        0.0
    }
}

#[derive(Default)]
pub(crate) struct StampLocals {
    pub(crate) var_a0: f64,
    pub(crate) var_a0_dn3: f64,
    pub(crate) var_a0_dn4: f64,
    pub(crate) var_a0_dn5: f64,
    pub(crate) var_a0_dn6: f64,
    pub(crate) var_a0_dn7: f64,
    pub(crate) var_a0_dn8: f64,
    pub(crate) var_a0_rv: f64,
    pub(crate) var_aaux: f64,
    pub(crate) var_aaux_dn3: f64,
    pub(crate) var_aaux_dn4: f64,
    pub(crate) var_aaux_dn5: f64,
    pub(crate) var_aaux_dn6: f64,
    pub(crate) var_aaux_dn7: f64,
    pub(crate) var_aaux_dn8: f64,
    pub(crate) var_aaux_rv: f64,
    pub(crate) var_aechvb: f64,
    pub(crate) var_agidl_i: f64,
    pub(crate) var_agidl_i_rv: f64,
    pub(crate) var_agisl_i: f64,
    pub(crate) var_agisl_i_rv: f64,
    pub(crate) var_aigbacc_i: f64,
    pub(crate) var_aigbacc_i_rv: f64,
    pub(crate) var_aigbinv_i: f64,
    pub(crate) var_aigbinv_i_rv: f64,
    pub(crate) var_aigc_i: f64,
    pub(crate) var_aigc_i_rv: f64,
    pub(crate) var_aigd_i: f64,
    pub(crate) var_aigd_i_rv: f64,
    pub(crate) var_aigs_i: f64,
    pub(crate) var_aigs_i_rv: f64,
    pub(crate) var_alpha0_i: f64,
    pub(crate) var_alpha0_i_rv: f64,
    pub(crate) var_alpha1_i: f64,
    pub(crate) var_alpha1_i_rv: f64,
    pub(crate) var_ascl_i: f64,
    pub(crate) var_ascl_i_rv: f64,
    pub(crate) var_at_i: f64,
    pub(crate) var_at_i_rv: f64,
    pub(crate) var_atb_i: f64,
    pub(crate) var_atb_i_rv: f64,
    pub(crate) var_auxb1: f64,
    pub(crate) var_auxb1_dn3: f64,
    pub(crate) var_auxb1_dn4: f64,
    pub(crate) var_auxb1_dn5: f64,
    pub(crate) var_auxb1_dn6: f64,
    pub(crate) var_auxb1_dn7: f64,
    pub(crate) var_auxb1_dn8: f64,
    pub(crate) var_auxb1_rv: f64,
    pub(crate) var_bechvb: f64,
    pub(crate) var_bechvb_rv: f64,
    pub(crate) var_beta: f64,
    pub(crate) var_beta0_i: f64,
    pub(crate) var_beta0_i_rv: f64,
    pub(crate) var_beta0_t: f64,
    pub(crate) var_beta0_t_dn4: f64,
    pub(crate) var_beta0_t_rv: f64,
    pub(crate) var_beta_dn3: f64,
    pub(crate) var_beta_dn4: f64,
    pub(crate) var_beta_dn5: f64,
    pub(crate) var_beta_dn6: f64,
    pub(crate) var_beta_dn7: f64,
    pub(crate) var_beta_dn8: f64,
    pub(crate) var_beta_rv: f64,
    pub(crate) var_bgidl_i: f64,
    pub(crate) var_bgidl_i_rv: f64,
    pub(crate) var_bgidl_t: f64,
    pub(crate) var_bgidl_t_dn4: f64,
    pub(crate) var_bgidl_t_rv: f64,
    pub(crate) var_bgisl_i: f64,
    pub(crate) var_bgisl_i_rv: f64,
    pub(crate) var_bgisl_t: f64,
    pub(crate) var_bgisl_t_dn4: f64,
    pub(crate) var_bgisl_t_rv: f64,
    pub(crate) var_bigbacc_i: f64,
    pub(crate) var_bigbacc_i_rv: f64,
    pub(crate) var_bigbinv_i: f64,
    pub(crate) var_bigbinv_i_rv: f64,
    pub(crate) var_bigc_i: f64,
    pub(crate) var_bigc_i_rv: f64,
    pub(crate) var_bigd_i: f64,
    pub(crate) var_bigd_i_rv: f64,
    pub(crate) var_bigs_i: f64,
    pub(crate) var_bigs_i_rv: f64,
    pub(crate) var_bpfactor: f64,
    pub(crate) var_bpfactor_rv: f64,
    pub(crate) var_bpfactornw_i: f64,
    pub(crate) var_bpfactornw_i_rv: f64,
    pub(crate) var_bpfactorpw_i: f64,
    pub(crate) var_bpfactorpw_i_rv: f64,
    pub(crate) var_bscl_i: f64,
    pub(crate) var_bscl_i_rv: f64,
    pub(crate) var_cbgcbg_i: f64,
    pub(crate) var_cbgcbg_i_rv: f64,
    pub(crate) var_cdbox: f64,
    pub(crate) var_cdbox_dn3: f64,
    pub(crate) var_cdbox_dn4: f64,
    pub(crate) var_cdbox_dn5: f64,
    pub(crate) var_cdbox_dn6: f64,
    pub(crate) var_cdbox_dn7: f64,
    pub(crate) var_cdbox_dn8: f64,
    pub(crate) var_cdbox_rv: f64,
    pub(crate) var_cdsc_i: f64,
    pub(crate) var_cdsc_i_rv: f64,
    pub(crate) var_cdscd_i: f64,
    pub(crate) var_cdscd_i_rv: f64,
    pub(crate) var_cfd_i: f64,
    pub(crate) var_cfd_i_rv: f64,
    pub(crate) var_cfs_i: f64,
    pub(crate) var_cfs_i_rv: f64,
    pub(crate) var_cigbacc_i: f64,
    pub(crate) var_cigbacc_i_rv: f64,
    pub(crate) var_cigbinv_i: f64,
    pub(crate) var_cigbinv_i_rv: f64,
    pub(crate) var_cigc_i: f64,
    pub(crate) var_cigc_i_rv: f64,
    pub(crate) var_cigd_i: f64,
    pub(crate) var_cigd_i_rv: f64,
    pub(crate) var_cigs_i: f64,
    pub(crate) var_cigs_i_rv: f64,
    pub(crate) var_cit_i: f64,
    pub(crate) var_cit_i_rv: f64,
    pub(crate) var_coth1: f64,
    pub(crate) var_coth1_dn3: f64,
    pub(crate) var_coth1_dn4: f64,
    pub(crate) var_coth1_dn5: f64,
    pub(crate) var_coth1_dn6: f64,
    pub(crate) var_coth1_dn7: f64,
    pub(crate) var_coth1_dn8: f64,
    pub(crate) var_coth1_rv: f64,
    pub(crate) var_cox1: f64,
    pub(crate) var_cox1_rv: f64,
    pub(crate) var_cox1p: f64,
    pub(crate) var_cox1p_rv: f64,
    pub(crate) var_cox2: f64,
    pub(crate) var_cox2_rv: f64,
    pub(crate) var_coxeff: f64,
    pub(crate) var_coxeff_dn3: f64,
    pub(crate) var_coxeff_dn4: f64,
    pub(crate) var_coxeff_dn5: f64,
    pub(crate) var_coxeff_dn6: f64,
    pub(crate) var_coxeff_dn7: f64,
    pub(crate) var_coxeff_dn8: f64,
    pub(crate) var_coxeff_rv: f64,
    pub(crate) var_csbox: f64,
    pub(crate) var_csbox_dn3: f64,
    pub(crate) var_csbox_dn4: f64,
    pub(crate) var_csbox_dn5: f64,
    pub(crate) var_csbox_dn6: f64,
    pub(crate) var_csbox_dn7: f64,
    pub(crate) var_csbox_dn8: f64,
    pub(crate) var_csbox_rv: f64,
    pub(crate) var_csc1: f64,
    pub(crate) var_csc1_dn3: f64,
    pub(crate) var_csc1_dn4: f64,
    pub(crate) var_csc1_dn5: f64,
    pub(crate) var_csc1_dn6: f64,
    pub(crate) var_csc1_dn7: f64,
    pub(crate) var_csc1_dn8: f64,
    pub(crate) var_csc1_rv: f64,
    pub(crate) var_csi: f64,
    pub(crate) var_csi_rv: f64,
    pub(crate) var_cth: f64,
    pub(crate) var_cth_rv: f64,
    pub(crate) var_dbgnw_i: f64,
    pub(crate) var_dbgnw_i_rv: f64,
    pub(crate) var_dbgpw_i: f64,
    pub(crate) var_dbgpw_i_rv: f64,
    pub(crate) var_delclm: f64,
    pub(crate) var_delclm_dn3: f64,
    pub(crate) var_delclm_dn4: f64,
    pub(crate) var_delclm_dn5: f64,
    pub(crate) var_delclm_dn6: f64,
    pub(crate) var_delclm_dn7: f64,
    pub(crate) var_delclm_dn8: f64,
    pub(crate) var_delclm_rv: f64,
    pub(crate) var_delta: f64,
    pub(crate) var_delta_dn3: f64,
    pub(crate) var_delta_dn4: f64,
    pub(crate) var_delta_dn5: f64,
    pub(crate) var_delta_dn6: f64,
    pub(crate) var_delta_dn7: f64,
    pub(crate) var_delta_dn8: f64,
    pub(crate) var_delta_rv: f64,
    pub(crate) var_deltaphi1: f64,
    pub(crate) var_deltaphi1_dn4: f64,
    pub(crate) var_deltaphi1_rv: f64,
    pub(crate) var_deltaphi2: f64,
    pub(crate) var_deltaphi2_dn3: f64,
    pub(crate) var_deltaphi2_dn4: f64,
    pub(crate) var_deltaphi2_dn5: f64,
    pub(crate) var_deltaphi2_dn6: f64,
    pub(crate) var_deltaphi2_dn7: f64,
    pub(crate) var_deltaphi2_dn8: f64,
    pub(crate) var_deltaphi2_rv: f64,
    pub(crate) var_deltemp: f64,
    pub(crate) var_deltemp_dn4: f64,
    pub(crate) var_deltemp_rv: f64,
    pub(crate) var_devsign: f64,
    pub(crate) var_devsign_rv: f64,
    pub(crate) var_devtemp: f64,
    pub(crate) var_devtemp_dn4: f64,
    pub(crate) var_devtemp_rv: f64,
    pub(crate) var_df: f64,
    pub(crate) var_df_dn3: f64,
    pub(crate) var_df_dn4: f64,
    pub(crate) var_df_dn5: f64,
    pub(crate) var_df_dn6: f64,
    pub(crate) var_df_dn7: f64,
    pub(crate) var_df_dn8: f64,
    pub(crate) var_df_rv: f64,
    pub(crate) var_dg1: f64,
    pub(crate) var_dg1_dn3: f64,
    pub(crate) var_dg1_dn4: f64,
    pub(crate) var_dg1_dn5: f64,
    pub(crate) var_dg1_dn6: f64,
    pub(crate) var_dg1_dn7: f64,
    pub(crate) var_dg1_dn8: f64,
    pub(crate) var_dg1_rv: f64,
    pub(crate) var_dg2: f64,
    pub(crate) var_dg2_dn3: f64,
    pub(crate) var_dg2_dn4: f64,
    pub(crate) var_dg2_dn5: f64,
    pub(crate) var_dg2_dn6: f64,
    pub(crate) var_dg2_dn7: f64,
    pub(crate) var_dg2_dn8: f64,
    pub(crate) var_dg2_rv: f64,
    pub(crate) var_diblfactor: f64,
    pub(crate) var_diblfactor_dn3: f64,
    pub(crate) var_diblfactor_dn4: f64,
    pub(crate) var_diblfactor_dn5: f64,
    pub(crate) var_diblfactor_dn6: f64,
    pub(crate) var_diblfactor_dn7: f64,
    pub(crate) var_diblfactor_dn8: f64,
    pub(crate) var_diblfactor_rv: f64,
    pub(crate) var_diffvds: f64,
    pub(crate) var_diffvds_dn3: f64,
    pub(crate) var_diffvds_dn4: f64,
    pub(crate) var_diffvds_dn5: f64,
    pub(crate) var_diffvds_dn6: f64,
    pub(crate) var_diffvds_dn7: f64,
    pub(crate) var_diffvds_dn8: f64,
    pub(crate) var_diffvds_rv: f64,
    pub(crate) var_digc_i: f64,
    pub(crate) var_digc_i_rv: f64,
    pub(crate) var_digd_i: f64,
    pub(crate) var_digd_i_rv: f64,
    pub(crate) var_digs_i: f64,
    pub(crate) var_digs_i_rv: f64,
    pub(crate) var_dlcv: f64,
    pub(crate) var_dlcv_rv: f64,
    pub(crate) var_dliv: f64,
    pub(crate) var_dliv_rv: f64,
    pub(crate) var_dlogsinhqsqdqsqrt: f64,
    pub(crate) var_dlogsinhqsqdqsqrt_dn3: f64,
    pub(crate) var_dlogsinhqsqdqsqrt_dn4: f64,
    pub(crate) var_dlogsinhqsqdqsqrt_dn5: f64,
    pub(crate) var_dlogsinhqsqdqsqrt_dn6: f64,
    pub(crate) var_dlogsinhqsqdqsqrt_dn7: f64,
    pub(crate) var_dlogsinhqsqdqsqrt_dn8: f64,
    pub(crate) var_dlogsinhqsqdqsqrt_rv: f64,
    pub(crate) var_dmob: f64,
    pub(crate) var_dmob_cv: f64,
    pub(crate) var_dmob_cv_dn3: f64,
    pub(crate) var_dmob_cv_dn4: f64,
    pub(crate) var_dmob_cv_dn5: f64,
    pub(crate) var_dmob_cv_dn6: f64,
    pub(crate) var_dmob_cv_dn7: f64,
    pub(crate) var_dmob_cv_dn8: f64,
    pub(crate) var_dmob_cv_rv: f64,
    pub(crate) var_dmob_dn3: f64,
    pub(crate) var_dmob_dn4: f64,
    pub(crate) var_dmob_dn5: f64,
    pub(crate) var_dmob_dn6: f64,
    pub(crate) var_dmob_dn7: f64,
    pub(crate) var_dmob_dn8: f64,
    pub(crate) var_dmob_rv: f64,
    pub(crate) var_dmobs: f64,
    pub(crate) var_dmobs_dn3: f64,
    pub(crate) var_dmobs_dn4: f64,
    pub(crate) var_dmobs_dn5: f64,
    pub(crate) var_dmobs_dn6: f64,
    pub(crate) var_dmobs_dn7: f64,
    pub(crate) var_dmobs_dn8: f64,
    pub(crate) var_dmobs_rv: f64,
    pub(crate) var_dq2: f64,
    pub(crate) var_dq2_dn3: f64,
    pub(crate) var_dq2_dn4: f64,
    pub(crate) var_dq2_dn5: f64,
    pub(crate) var_dq2_dn6: f64,
    pub(crate) var_dq2_dn7: f64,
    pub(crate) var_dq2_dn8: f64,
    pub(crate) var_dq2_rv: f64,
    pub(crate) var_dqcoth: f64,
    pub(crate) var_dqcoth_dn3: f64,
    pub(crate) var_dqcoth_dn4: f64,
    pub(crate) var_dqcoth_dn5: f64,
    pub(crate) var_dqcoth_dn6: f64,
    pub(crate) var_dqcoth_dn7: f64,
    pub(crate) var_dqcoth_dn8: f64,
    pub(crate) var_dqcoth_rv: f64,
    pub(crate) var_dqcothqdqsqrt: f64,
    pub(crate) var_dqcothqdqsqrt_dn3: f64,
    pub(crate) var_dqcothqdqsqrt_dn4: f64,
    pub(crate) var_dqcothqdqsqrt_dn5: f64,
    pub(crate) var_dqcothqdqsqrt_dn6: f64,
    pub(crate) var_dqcothqdqsqrt_dn7: f64,
    pub(crate) var_dqcothqdqsqrt_dn8: f64,
    pub(crate) var_dqcothqdqsqrt_rv: f64,
    pub(crate) var_dqi: f64,
    pub(crate) var_dqi_dn3: f64,
    pub(crate) var_dqi_dn4: f64,
    pub(crate) var_dqi_dn5: f64,
    pub(crate) var_dqi_dn6: f64,
    pub(crate) var_dqi_dn7: f64,
    pub(crate) var_dqi_dn8: f64,
    pub(crate) var_dqi_rv: f64,
    pub(crate) var_dqsqrt: f64,
    pub(crate) var_dqsqrt_dn3: f64,
    pub(crate) var_dqsqrt_dn4: f64,
    pub(crate) var_dqsqrt_dn5: f64,
    pub(crate) var_dqsqrt_dn6: f64,
    pub(crate) var_dqsqrt_dn7: f64,
    pub(crate) var_dqsqrt_dn8: f64,
    pub(crate) var_dqsqrt_rv: f64,
    pub(crate) var_dr: f64,
    pub(crate) var_dr_dn3: f64,
    pub(crate) var_dr_dn4: f64,
    pub(crate) var_dr_dn5: f64,
    pub(crate) var_dr_dn6: f64,
    pub(crate) var_dr_dn7: f64,
    pub(crate) var_dr_dn8: f64,
    pub(crate) var_dr_rv: f64,
    pub(crate) var_drout_i: f64,
    pub(crate) var_drout_i_rv: f64,
    pub(crate) var_dsc0_i: f64,
    pub(crate) var_dsc0_i_rv: f64,
    pub(crate) var_dsc1_i: f64,
    pub(crate) var_dsc1_i_rv: f64,
    pub(crate) var_dsub_i: f64,
    pub(crate) var_dsub_i_rv: f64,
    pub(crate) var_dvsat: f64,
    pub(crate) var_dvsat_dn3: f64,
    pub(crate) var_dvsat_dn4: f64,
    pub(crate) var_dvsat_dn5: f64,
    pub(crate) var_dvsat_dn6: f64,
    pub(crate) var_dvsat_dn7: f64,
    pub(crate) var_dvsat_dn8: f64,
    pub(crate) var_dvsat_rv: f64,
    pub(crate) var_dvt0_i: f64,
    pub(crate) var_dvt0_i_rv: f64,
    pub(crate) var_dvt1_i: f64,
    pub(crate) var_dvt1_i_rv: f64,
    pub(crate) var_dvth_all: f64,
    pub(crate) var_dvth_all_dn3: f64,
    pub(crate) var_dvth_all_dn4: f64,
    pub(crate) var_dvth_all_dn5: f64,
    pub(crate) var_dvth_all_dn6: f64,
    pub(crate) var_dvth_all_dn7: f64,
    pub(crate) var_dvth_all_dn8: f64,
    pub(crate) var_dvth_all_rv: f64,
    pub(crate) var_dvth_dibl: f64,
    pub(crate) var_dvth_dibl_dn3: f64,
    pub(crate) var_dvth_dibl_dn4: f64,
    pub(crate) var_dvth_dibl_dn5: f64,
    pub(crate) var_dvth_dibl_dn6: f64,
    pub(crate) var_dvth_dibl_dn7: f64,
    pub(crate) var_dvth_dibl_dn8: f64,
    pub(crate) var_dvth_dibl_rv: f64,
    pub(crate) var_dvth_dsc: f64,
    pub(crate) var_dvth_dsc_dn5: f64,
    pub(crate) var_dvth_dsc_dn6: f64,
    pub(crate) var_dvth_dsc_rv: f64,
    pub(crate) var_dvth_nbody: f64,
    pub(crate) var_dvth_nbody_rv: f64,
    pub(crate) var_dvth_rsce: f64,
    pub(crate) var_dvth_rsce_dn3: f64,
    pub(crate) var_dvth_rsce_dn4: f64,
    pub(crate) var_dvth_rsce_dn5: f64,
    pub(crate) var_dvth_rsce_dn6: f64,
    pub(crate) var_dvth_rsce_dn7: f64,
    pub(crate) var_dvth_rsce_dn8: f64,
    pub(crate) var_dvth_rsce_rv: f64,
    pub(crate) var_dvth_temp: f64,
    pub(crate) var_dvth_temp0: f64,
    pub(crate) var_dvth_temp0_dn4: f64,
    pub(crate) var_dvth_temp0_rv: f64,
    pub(crate) var_dvth_temp_dn3: f64,
    pub(crate) var_dvth_temp_dn4: f64,
    pub(crate) var_dvth_temp_dn5: f64,
    pub(crate) var_dvth_temp_dn6: f64,
    pub(crate) var_dvth_temp_dn7: f64,
    pub(crate) var_dvth_temp_dn8: f64,
    pub(crate) var_dvth_temp_rv: f64,
    pub(crate) var_dvth_vbg: f64,
    pub(crate) var_dvth_vbg_dn3: f64,
    pub(crate) var_dvth_vbg_dn4: f64,
    pub(crate) var_dvth_vbg_dn5: f64,
    pub(crate) var_dvth_vbg_dn6: f64,
    pub(crate) var_dvth_vbg_dn7: f64,
    pub(crate) var_dvth_vbg_dn8: f64,
    pub(crate) var_dvth_vbg_rv: f64,
    pub(crate) var_dvth_vtroll: f64,
    pub(crate) var_dvth_vtroll_dn3: f64,
    pub(crate) var_dvth_vtroll_dn4: f64,
    pub(crate) var_dvth_vtroll_dn5: f64,
    pub(crate) var_dvth_vtroll_dn6: f64,
    pub(crate) var_dvth_vtroll_dn7: f64,
    pub(crate) var_dvth_vtroll_dn8: f64,
    pub(crate) var_dvth_vtroll_rv: f64,
    pub(crate) var_dvtp0_i: f64,
    pub(crate) var_dvtp0_i_rv: f64,
    pub(crate) var_dvtp1_i: f64,
    pub(crate) var_dvtp1_i_rv: f64,
    pub(crate) var_dwcv: f64,
    pub(crate) var_dwcv_rv: f64,
    pub(crate) var_dwiv: f64,
    pub(crate) var_dwiv_rv: f64,
    pub(crate) var_eefffactor: f64,
    pub(crate) var_eefffactor2: f64,
    pub(crate) var_eefffactor2_rv: f64,
    pub(crate) var_eefffactor_rv: f64,
    pub(crate) var_eeffm: f64,
    pub(crate) var_eeffm2: f64,
    pub(crate) var_eeffm2_dn3: f64,
    pub(crate) var_eeffm2_dn4: f64,
    pub(crate) var_eeffm2_dn5: f64,
    pub(crate) var_eeffm2_dn6: f64,
    pub(crate) var_eeffm2_dn7: f64,
    pub(crate) var_eeffm2_dn8: f64,
    pub(crate) var_eeffm2_rv: f64,
    pub(crate) var_eeffm_cv: f64,
    pub(crate) var_eeffm_cv_dn3: f64,
    pub(crate) var_eeffm_cv_dn4: f64,
    pub(crate) var_eeffm_cv_dn5: f64,
    pub(crate) var_eeffm_cv_dn6: f64,
    pub(crate) var_eeffm_cv_dn7: f64,
    pub(crate) var_eeffm_cv_dn8: f64,
    pub(crate) var_eeffm_cv_rv: f64,
    pub(crate) var_eeffm_dn3: f64,
    pub(crate) var_eeffm_dn4: f64,
    pub(crate) var_eeffm_dn5: f64,
    pub(crate) var_eeffm_dn6: f64,
    pub(crate) var_eeffm_dn7: f64,
    pub(crate) var_eeffm_dn8: f64,
    pub(crate) var_eeffm_rv: f64,
    pub(crate) var_eeffs: f64,
    pub(crate) var_eeffs2: f64,
    pub(crate) var_eeffs2_dn3: f64,
    pub(crate) var_eeffs2_dn4: f64,
    pub(crate) var_eeffs2_dn5: f64,
    pub(crate) var_eeffs2_dn6: f64,
    pub(crate) var_eeffs2_dn7: f64,
    pub(crate) var_eeffs2_dn8: f64,
    pub(crate) var_eeffs2_rv: f64,
    pub(crate) var_eeffs_dn3: f64,
    pub(crate) var_eeffs_dn4: f64,
    pub(crate) var_eeffs_dn5: f64,
    pub(crate) var_eeffs_dn6: f64,
    pub(crate) var_eeffs_dn7: f64,
    pub(crate) var_eeffs_dn8: f64,
    pub(crate) var_eeffs_rv: f64,
    pub(crate) var_eg: f64,
    pub(crate) var_eg_dn4: f64,
    pub(crate) var_eg_rv: f64,
    pub(crate) var_egidl_i: f64,
    pub(crate) var_egidl_i_rv: f64,
    pub(crate) var_egisl_i: f64,
    pub(crate) var_egisl_i_rv: f64,
    pub(crate) var_eigbinv_i: f64,
    pub(crate) var_eigbinv_i_rv: f64,
    pub(crate) var_epsratio: f64,
    pub(crate) var_epsratio_rv: f64,
    pub(crate) var_epssi: f64,
    pub(crate) var_epssi_rv: f64,
    pub(crate) var_esat: f64,
    pub(crate) var_esat1: f64,
    pub(crate) var_esat1_dn3: f64,
    pub(crate) var_esat1_dn4: f64,
    pub(crate) var_esat1_dn5: f64,
    pub(crate) var_esat1_dn6: f64,
    pub(crate) var_esat1_dn7: f64,
    pub(crate) var_esat1_dn8: f64,
    pub(crate) var_esat1_rv: f64,
    pub(crate) var_esat1l: f64,
    pub(crate) var_esat1l_dn3: f64,
    pub(crate) var_esat1l_dn4: f64,
    pub(crate) var_esat1l_dn5: f64,
    pub(crate) var_esat1l_dn6: f64,
    pub(crate) var_esat1l_dn7: f64,
    pub(crate) var_esat1l_dn8: f64,
    pub(crate) var_esat1l_rv: f64,
    pub(crate) var_esat_dn3: f64,
    pub(crate) var_esat_dn4: f64,
    pub(crate) var_esat_dn5: f64,
    pub(crate) var_esat_dn6: f64,
    pub(crate) var_esat_dn7: f64,
    pub(crate) var_esat_dn8: f64,
    pub(crate) var_esat_rv: f64,
    pub(crate) var_esatcv: f64,
    pub(crate) var_esatcv_dn3: f64,
    pub(crate) var_esatcv_dn4: f64,
    pub(crate) var_esatcv_dn5: f64,
    pub(crate) var_esatcv_dn6: f64,
    pub(crate) var_esatcv_dn7: f64,
    pub(crate) var_esatcv_dn8: f64,
    pub(crate) var_esatcv_rv: f64,
    pub(crate) var_esatcvl: f64,
    pub(crate) var_esatcvl_dn3: f64,
    pub(crate) var_esatcvl_dn4: f64,
    pub(crate) var_esatcvl_dn5: f64,
    pub(crate) var_esatcvl_dn6: f64,
    pub(crate) var_esatcvl_dn7: f64,
    pub(crate) var_esatcvl_dn8: f64,
    pub(crate) var_esatcvl_rv: f64,
    pub(crate) var_esatl: f64,
    pub(crate) var_esatl_dn3: f64,
    pub(crate) var_esatl_dn4: f64,
    pub(crate) var_esatl_dn5: f64,
    pub(crate) var_esatl_dn6: f64,
    pub(crate) var_esatl_dn7: f64,
    pub(crate) var_esatl_dn8: f64,
    pub(crate) var_esatl_rv: f64,
    pub(crate) var_esatnoi: f64,
    pub(crate) var_esatnoi_dn3: f64,
    pub(crate) var_esatnoi_dn4: f64,
    pub(crate) var_esatnoi_dn5: f64,
    pub(crate) var_esatnoi_dn6: f64,
    pub(crate) var_esatnoi_dn7: f64,
    pub(crate) var_esatnoi_dn8: f64,
    pub(crate) var_esatnoi_rv: f64,
    pub(crate) var_eta0_i: f64,
    pub(crate) var_eta0_i_rv: f64,
    pub(crate) var_eta0_t: f64,
    pub(crate) var_eta0_t_dn4: f64,
    pub(crate) var_eta0_t_rv: f64,
    pub(crate) var_eta1_i: f64,
    pub(crate) var_eta1_i_rv: f64,
    pub(crate) var_eta_mu: f64,
    pub(crate) var_eta_mu2: f64,
    pub(crate) var_eta_mu2_rv: f64,
    pub(crate) var_eta_mu_cv: f64,
    pub(crate) var_eta_mu_cv_rv: f64,
    pub(crate) var_eta_mu_rv: f64,
    pub(crate) var_etab_i: f64,
    pub(crate) var_etab_i_rv: f64,
    pub(crate) var_etamob2_i: f64,
    pub(crate) var_etamob2_i_rv: f64,
    pub(crate) var_etamob_i: f64,
    pub(crate) var_etamob_i_rv: f64,
    pub(crate) var_etaqm_i: f64,
    pub(crate) var_etaqm_i_rv: f64,
    pub(crate) var_eu2_i: f64,
    pub(crate) var_eu2_i_rv: f64,
    pub(crate) var_eu_i: f64,
    pub(crate) var_eu_i_rv: f64,
    pub(crate) var_eub2_i: f64,
    pub(crate) var_eub2_i_rv: f64,
    pub(crate) var_eub_i: f64,
    pub(crate) var_eub_i_rv: f64,
    pub(crate) var_f: f64,
    pub(crate) var_f_dn3: f64,
    pub(crate) var_f_dn4: f64,
    pub(crate) var_f_dn5: f64,
    pub(crate) var_f_dn6: f64,
    pub(crate) var_f_dn7: f64,
    pub(crate) var_f_dn8: f64,
    pub(crate) var_f_rv: f64,
    pub(crate) var_g: f64,
    pub(crate) var_g_dn3: f64,
    pub(crate) var_g_dn4: f64,
    pub(crate) var_g_dn5: f64,
    pub(crate) var_g_dn6: f64,
    pub(crate) var_g_dn7: f64,
    pub(crate) var_g_dn8: f64,
    pub(crate) var_g_rv: f64,
    pub(crate) var_gamma0: f64,
    pub(crate) var_gamma0_rv: f64,
    pub(crate) var_gcrg: f64,
    pub(crate) var_gcrg_dn3: f64,
    pub(crate) var_gcrg_dn4: f64,
    pub(crate) var_gcrg_dn5: f64,
    pub(crate) var_gcrg_dn6: f64,
    pub(crate) var_gcrg_dn7: f64,
    pub(crate) var_gcrg_dn8: f64,
    pub(crate) var_gdpr: f64,
    pub(crate) var_gdpr_dn3: f64,
    pub(crate) var_gdpr_dn4: f64,
    pub(crate) var_gdpr_dn5: f64,
    pub(crate) var_gdpr_dn6: f64,
    pub(crate) var_gdpr_dn7: f64,
    pub(crate) var_gdpr_dn8: f64,
    pub(crate) var_gspr: f64,
    pub(crate) var_gspr_dn3: f64,
    pub(crate) var_gspr_dn4: f64,
    pub(crate) var_gspr_dn5: f64,
    pub(crate) var_gspr_dn6: f64,
    pub(crate) var_gspr_dn7: f64,
    pub(crate) var_gspr_dn8: f64,
    pub(crate) var_guard104: f64,
    pub(crate) var_guard104_rv: f64,
    pub(crate) var_guard105: f64,
    pub(crate) var_guard105_rv: f64,
    pub(crate) var_guard106: f64,
    pub(crate) var_guard106_rv: f64,
    pub(crate) var_guard107: f64,
    pub(crate) var_guard107_rv: f64,
    pub(crate) var_guard108: f64,
    pub(crate) var_guard108_rv: f64,
    pub(crate) var_guard109: f64,
    pub(crate) var_guard109_rv: f64,
    pub(crate) var_guard116: f64,
    pub(crate) var_guard116_rv: f64,
    pub(crate) var_guard117: f64,
    pub(crate) var_guard117_rv: f64,
    pub(crate) var_guard118: f64,
    pub(crate) var_guard118_rv: f64,
    pub(crate) var_guard119: f64,
    pub(crate) var_guard119_rv: f64,
    pub(crate) var_guard120: f64,
    pub(crate) var_guard120_rv: f64,
    pub(crate) var_guard121: f64,
    pub(crate) var_guard121_rv: f64,
    pub(crate) var_guard122: f64,
    pub(crate) var_guard122_rv: f64,
    pub(crate) var_guard123: f64,
    pub(crate) var_guard123_rv: f64,
    pub(crate) var_guard124: f64,
    pub(crate) var_guard125: f64,
    pub(crate) var_guard126: f64,
    pub(crate) var_guard126_rv: f64,
    pub(crate) var_guard127: f64,
    pub(crate) var_guard127_rv: f64,
    pub(crate) var_guard128: f64,
    pub(crate) var_guard129: f64,
    pub(crate) var_guard129_rv: f64,
    pub(crate) var_guard130: f64,
    pub(crate) var_guard131: f64,
    pub(crate) var_guard131_rv: f64,
    pub(crate) var_guard132: f64,
    pub(crate) var_guard132_rv: f64,
    pub(crate) var_guard133: f64,
    pub(crate) var_guard133_rv: f64,
    pub(crate) var_guard134: f64,
    pub(crate) var_guard134_rv: f64,
    pub(crate) var_guard136: f64,
    pub(crate) var_guard136_rv: f64,
    pub(crate) var_guard137: f64,
    pub(crate) var_guard137_rv: f64,
    pub(crate) var_guard138: f64,
    pub(crate) var_guard139: f64,
    pub(crate) var_guard14: f64,
    pub(crate) var_guard140: f64,
    pub(crate) var_guard147: f64,
    pub(crate) var_guard147_rv: f64,
    pub(crate) var_guard148: f64,
    pub(crate) var_guard14_rv: f64,
    pub(crate) var_guard15: f64,
    pub(crate) var_guard15_rv: f64,
    pub(crate) var_guard16: f64,
    pub(crate) var_guard16_rv: f64,
    pub(crate) var_guard17: f64,
    pub(crate) var_guard17_rv: f64,
    pub(crate) var_guard18: f64,
    pub(crate) var_guard18_rv: f64,
    pub(crate) var_guard19: f64,
    pub(crate) var_guard19_rv: f64,
    pub(crate) var_guard20: f64,
    pub(crate) var_guard20_rv: f64,
    pub(crate) var_guard21: f64,
    pub(crate) var_guard21_rv: f64,
    pub(crate) var_guard22: f64,
    pub(crate) var_guard22_rv: f64,
    pub(crate) var_guard23: f64,
    pub(crate) var_guard23_rv: f64,
    pub(crate) var_guard24: f64,
    pub(crate) var_guard24_rv: f64,
    pub(crate) var_guard25: f64,
    pub(crate) var_guard25_rv: f64,
    pub(crate) var_guard26: f64,
    pub(crate) var_guard26_rv: f64,
    pub(crate) var_guard27: f64,
    pub(crate) var_guard27_rv: f64,
    pub(crate) var_guard28: f64,
    pub(crate) var_guard28_rv: f64,
    pub(crate) var_guard3: f64,
    pub(crate) var_guard31: f64,
    pub(crate) var_guard31_rv: f64,
    pub(crate) var_guard32: f64,
    pub(crate) var_guard32_rv: f64,
    pub(crate) var_guard33: f64,
    pub(crate) var_guard34: f64,
    pub(crate) var_guard35: f64,
    pub(crate) var_guard36: f64,
    pub(crate) var_guard37: f64,
    pub(crate) var_guard37_rv: f64,
    pub(crate) var_guard3_rv: f64,
    pub(crate) var_guard4: f64,
    pub(crate) var_guard41: f64,
    pub(crate) var_guard41_rv: f64,
    pub(crate) var_guard42: f64,
    pub(crate) var_guard42_rv: f64,
    pub(crate) var_guard43: f64,
    pub(crate) var_guard43_rv: f64,
    pub(crate) var_guard44: f64,
    pub(crate) var_guard44_rv: f64,
    pub(crate) var_guard45: f64,
    pub(crate) var_guard45_rv: f64,
    pub(crate) var_guard46: f64,
    pub(crate) var_guard46_rv: f64,
    pub(crate) var_guard47: f64,
    pub(crate) var_guard47_rv: f64,
    pub(crate) var_guard48: f64,
    pub(crate) var_guard49: f64,
    pub(crate) var_guard4_rv: f64,
    pub(crate) var_guard5: f64,
    pub(crate) var_guard50: f64,
    pub(crate) var_guard51: f64,
    pub(crate) var_guard52: f64,
    pub(crate) var_guard52_rv: f64,
    pub(crate) var_guard53: f64,
    pub(crate) var_guard53_rv: f64,
    pub(crate) var_guard54: f64,
    pub(crate) var_guard54_rv: f64,
    pub(crate) var_guard55: f64,
    pub(crate) var_guard56: f64,
    pub(crate) var_guard56_rv: f64,
    pub(crate) var_guard59: f64,
    pub(crate) var_guard59_rv: f64,
    pub(crate) var_guard5_rv: f64,
    pub(crate) var_guard61: f64,
    pub(crate) var_guard61_rv: f64,
    pub(crate) var_guard62: f64,
    pub(crate) var_guard62_rv: f64,
    pub(crate) var_guard63: f64,
    pub(crate) var_guard63_rv: f64,
    pub(crate) var_guard64: f64,
    pub(crate) var_guard64_rv: f64,
    pub(crate) var_guard65: f64,
    pub(crate) var_guard65_rv: f64,
    pub(crate) var_guard66: f64,
    pub(crate) var_guard66_rv: f64,
    pub(crate) var_guard67: f64,
    pub(crate) var_guard67_rv: f64,
    pub(crate) var_guard68: f64,
    pub(crate) var_guard68_rv: f64,
    pub(crate) var_guard69: f64,
    pub(crate) var_guard69_rv: f64,
    pub(crate) var_guard70: f64,
    pub(crate) var_guard70_rv: f64,
    pub(crate) var_guard71: f64,
    pub(crate) var_guard71_rv: f64,
    pub(crate) var_guard72: f64,
    pub(crate) var_guard72_rv: f64,
    pub(crate) var_guard73: f64,
    pub(crate) var_guard73_rv: f64,
    pub(crate) var_guard74: f64,
    pub(crate) var_guard74_rv: f64,
    pub(crate) var_guard75: f64,
    pub(crate) var_guard75_rv: f64,
    pub(crate) var_guard76: f64,
    pub(crate) var_guard76_rv: f64,
    pub(crate) var_guard77: f64,
    pub(crate) var_guard77_rv: f64,
    pub(crate) var_guard78: f64,
    pub(crate) var_guard78_rv: f64,
    pub(crate) var_guard79: f64,
    pub(crate) var_guard79_rv: f64,
    pub(crate) var_guard80: f64,
    pub(crate) var_guard80_rv: f64,
    pub(crate) var_guard81: f64,
    pub(crate) var_guard81_rv: f64,
    pub(crate) var_guard82: f64,
    pub(crate) var_guard82_rv: f64,
    pub(crate) var_guard87: f64,
    pub(crate) var_guard87_rv: f64,
    pub(crate) var_guard88: f64,
    pub(crate) var_guard88_rv: f64,
    pub(crate) var_guard89: f64,
    pub(crate) var_guard89_rv: f64,
    pub(crate) var_guard90: f64,
    pub(crate) var_guard90_rv: f64,
    pub(crate) var_guard91: f64,
    pub(crate) var_guard91_rv: f64,
    pub(crate) var_guard92: f64,
    pub(crate) var_guard92_rv: f64,
    pub(crate) var_guard93: f64,
    pub(crate) var_guard93_rv: f64,
    pub(crate) var_guard94: f64,
    pub(crate) var_guard94_rv: f64,
    pub(crate) var_guard95: f64,
    pub(crate) var_guard95_rv: f64,
    pub(crate) var_guard96: f64,
    pub(crate) var_guard96_rv: f64,
    pub(crate) var_guard97: f64,
    pub(crate) var_guard97_rv: f64,
    pub(crate) var_guard98: f64,
    pub(crate) var_guard98_rv: f64,
    pub(crate) var_guard99: f64,
    pub(crate) var_guard99_rv: f64,
    pub(crate) var_ids: f64,
    pub(crate) var_ids0: f64,
    pub(crate) var_ids0_dn3: f64,
    pub(crate) var_ids0_dn4: f64,
    pub(crate) var_ids0_dn5: f64,
    pub(crate) var_ids0_dn6: f64,
    pub(crate) var_ids0_dn7: f64,
    pub(crate) var_ids0_dn8: f64,
    pub(crate) var_ids0_ov_dqi: f64,
    pub(crate) var_ids0_ov_dqi_dn3: f64,
    pub(crate) var_ids0_ov_dqi_dn4: f64,
    pub(crate) var_ids0_ov_dqi_dn5: f64,
    pub(crate) var_ids0_ov_dqi_dn6: f64,
    pub(crate) var_ids0_ov_dqi_dn7: f64,
    pub(crate) var_ids0_ov_dqi_dn8: f64,
    pub(crate) var_ids0_ov_dqi_rv: f64,
    pub(crate) var_ids0_rv: f64,
    pub(crate) var_ids_dn3: f64,
    pub(crate) var_ids_dn4: f64,
    pub(crate) var_ids_dn5: f64,
    pub(crate) var_ids_dn6: f64,
    pub(crate) var_ids_dn7: f64,
    pub(crate) var_ids_dn8: f64,
    pub(crate) var_ids_rv: f64,
    pub(crate) var_idsovvds: f64,
    pub(crate) var_idsovvds_dn3: f64,
    pub(crate) var_idsovvds_dn4: f64,
    pub(crate) var_idsovvds_dn5: f64,
    pub(crate) var_idsovvds_dn6: f64,
    pub(crate) var_idsovvds_dn7: f64,
    pub(crate) var_idsovvds_dn8: f64,
    pub(crate) var_igbacc: f64,
    pub(crate) var_igbacc_dn3: f64,
    pub(crate) var_igbacc_dn4: f64,
    pub(crate) var_igbacc_dn5: f64,
    pub(crate) var_igbacc_dn6: f64,
    pub(crate) var_igbacc_dn7: f64,
    pub(crate) var_igbacc_dn8: f64,
    pub(crate) var_igbd: f64,
    pub(crate) var_igbd_dn3: f64,
    pub(crate) var_igbd_dn4: f64,
    pub(crate) var_igbd_dn5: f64,
    pub(crate) var_igbd_dn6: f64,
    pub(crate) var_igbd_dn7: f64,
    pub(crate) var_igbd_dn8: f64,
    pub(crate) var_igbinv: f64,
    pub(crate) var_igbinv_dn3: f64,
    pub(crate) var_igbinv_dn4: f64,
    pub(crate) var_igbinv_dn5: f64,
    pub(crate) var_igbinv_dn6: f64,
    pub(crate) var_igbinv_dn7: f64,
    pub(crate) var_igbinv_dn8: f64,
    pub(crate) var_igbs: f64,
    pub(crate) var_igbs_dn3: f64,
    pub(crate) var_igbs_dn4: f64,
    pub(crate) var_igbs_dn5: f64,
    pub(crate) var_igbs_dn6: f64,
    pub(crate) var_igbs_dn7: f64,
    pub(crate) var_igbs_dn8: f64,
    pub(crate) var_igc0: f64,
    pub(crate) var_igc0_dn3: f64,
    pub(crate) var_igc0_dn4: f64,
    pub(crate) var_igc0_dn5: f64,
    pub(crate) var_igc0_dn6: f64,
    pub(crate) var_igc0_dn7: f64,
    pub(crate) var_igc0_dn8: f64,
    pub(crate) var_igcd: f64,
    pub(crate) var_igcd_dn3: f64,
    pub(crate) var_igcd_dn4: f64,
    pub(crate) var_igcd_dn5: f64,
    pub(crate) var_igcd_dn6: f64,
    pub(crate) var_igcd_dn7: f64,
    pub(crate) var_igcd_dn8: f64,
    pub(crate) var_igcs: f64,
    pub(crate) var_igcs_dn3: f64,
    pub(crate) var_igcs_dn4: f64,
    pub(crate) var_igcs_dn5: f64,
    pub(crate) var_igcs_dn6: f64,
    pub(crate) var_igcs_dn7: f64,
    pub(crate) var_igcs_dn8: f64,
    pub(crate) var_igd: f64,
    pub(crate) var_igd_dn3: f64,
    pub(crate) var_igd_dn4: f64,
    pub(crate) var_igd_dn5: f64,
    pub(crate) var_igd_dn6: f64,
    pub(crate) var_igd_dn7: f64,
    pub(crate) var_igd_dn8: f64,
    pub(crate) var_igidl: f64,
    pub(crate) var_igidl_dn3: f64,
    pub(crate) var_igidl_dn4: f64,
    pub(crate) var_igidl_dn5: f64,
    pub(crate) var_igidl_dn6: f64,
    pub(crate) var_igidl_dn7: f64,
    pub(crate) var_igidl_dn8: f64,
    pub(crate) var_igisl: f64,
    pub(crate) var_igisl_dn3: f64,
    pub(crate) var_igisl_dn4: f64,
    pub(crate) var_igisl_dn5: f64,
    pub(crate) var_igisl_dn6: f64,
    pub(crate) var_igisl_dn7: f64,
    pub(crate) var_igisl_dn8: f64,
    pub(crate) var_igs: f64,
    pub(crate) var_igs_dn3: f64,
    pub(crate) var_igs_dn4: f64,
    pub(crate) var_igs_dn5: f64,
    pub(crate) var_igs_dn6: f64,
    pub(crate) var_igs_dn7: f64,
    pub(crate) var_igs_dn8: f64,
    pub(crate) var_igsd_mult: f64,
    pub(crate) var_igsd_mult0: f64,
    pub(crate) var_igsd_mult0_dn3: f64,
    pub(crate) var_igsd_mult0_dn4: f64,
    pub(crate) var_igsd_mult0_dn5: f64,
    pub(crate) var_igsd_mult0_dn6: f64,
    pub(crate) var_igsd_mult0_dn7: f64,
    pub(crate) var_igsd_mult0_dn8: f64,
    pub(crate) var_igsd_mult_dn3: f64,
    pub(crate) var_igsd_mult_dn4: f64,
    pub(crate) var_igsd_mult_dn5: f64,
    pub(crate) var_igsd_mult_dn6: f64,
    pub(crate) var_igsd_mult_dn7: f64,
    pub(crate) var_igsd_mult_dn8: f64,
    pub(crate) var_igt_i: f64,
    pub(crate) var_igtemp: f64,
    pub(crate) var_igtemp_dn4: f64,
    pub(crate) var_iii: f64,
    pub(crate) var_iii_dn3: f64,
    pub(crate) var_iii_dn4: f64,
    pub(crate) var_iii_dn5: f64,
    pub(crate) var_iii_dn6: f64,
    pub(crate) var_iii_dn7: f64,
    pub(crate) var_iii_dn8: f64,
    pub(crate) var_iit_i: f64,
    pub(crate) var_iit_i_rv: f64,
    pub(crate) var_imgtoxp: f64,
    pub(crate) var_imgtoxp_rv: f64,
    pub(crate) var_inv_l: f64,
    pub(crate) var_inv_l_rv: f64,
    pub(crate) var_inv_mexp: f64,
    pub(crate) var_inv_mexp_rv: f64,
    pub(crate) var_inv_w: f64,
    pub(crate) var_inv_w_rv: f64,
    pub(crate) var_inv_wl: f64,
    pub(crate) var_inv_wl_rv: f64,
    pub(crate) var_k01_i: f64,
    pub(crate) var_k01_i_rv: f64,
    pub(crate) var_k0_i: f64,
    pub(crate) var_k0_i_rv: f64,
    pub(crate) var_k0_t: f64,
    pub(crate) var_k0_t_dn4: f64,
    pub(crate) var_k0_t_rv: f64,
    pub(crate) var_k0si1_i: f64,
    pub(crate) var_k0si1_i_rv: f64,
    pub(crate) var_k0si_i: f64,
    pub(crate) var_k0si_i_rv: f64,
    pub(crate) var_k0si_t: f64,
    pub(crate) var_k0si_t_dn4: f64,
    pub(crate) var_k0si_t_rv: f64,
    pub(crate) var_k0sisat1_i: f64,
    pub(crate) var_k0sisat1_i_rv: f64,
    pub(crate) var_k0sisat_i: f64,
    pub(crate) var_k0sisat_i_rv: f64,
    pub(crate) var_k0sisat_t: f64,
    pub(crate) var_k0sisat_t_dn4: f64,
    pub(crate) var_k0sisat_t_rv: f64,
    pub(crate) var_k1: f64,
    pub(crate) var_k1_2: f64,
    pub(crate) var_k1_2_rv: f64,
    pub(crate) var_k1_rv: f64,
    pub(crate) var_k1rsce_i: f64,
    pub(crate) var_k1rsce_i_rv: f64,
    pub(crate) var_k2: f64,
    pub(crate) var_k2_rv: f64,
    pub(crate) var_kbg0nw_i: f64,
    pub(crate) var_kbg0nw_i_rv: f64,
    pub(crate) var_kbg0pw_i: f64,
    pub(crate) var_kbg0pw_i_rv: f64,
    pub(crate) var_kbg1nw_i: f64,
    pub(crate) var_kbg1nw_i_rv: f64,
    pub(crate) var_kbg1pw_i: f64,
    pub(crate) var_kbg1pw_i_rv: f64,
    pub(crate) var_kbg2nw_i: f64,
    pub(crate) var_kbg2nw_i_rv: f64,
    pub(crate) var_kbg2pw_i: f64,
    pub(crate) var_kbg2pw_i_rv: f64,
    pub(crate) var_keq_k2: f64,
    pub(crate) var_keq_k2_rv: f64,
    pub(crate) var_ksativ_i: f64,
    pub(crate) var_ksativ_i_rv: f64,
    pub(crate) var_ksativb_i: f64,
    pub(crate) var_ksativb_i_rv: f64,
    pub(crate) var_ksubiv_i: f64,
    pub(crate) var_ksubiv_i_rv: f64,
    pub(crate) var_kvbg: f64,
    pub(crate) var_kvbg_dn3: f64,
    pub(crate) var_kvbg_dn4: f64,
    pub(crate) var_kvbg_dn5: f64,
    pub(crate) var_kvbg_dn6: f64,
    pub(crate) var_kvbg_dn7: f64,
    pub(crate) var_kvbg_dn8: f64,
    pub(crate) var_kvbg_rv: f64,
    pub(crate) var_l_lln: f64,
    pub(crate) var_l_lln_rv: f64,
    pub(crate) var_l_wln: f64,
    pub(crate) var_l_wln_rv: f64,
    pub(crate) var_leff: f64,
    pub(crate) var_leff_rv: f64,
    pub(crate) var_leffcv: f64,
    pub(crate) var_leffcv_rv: f64,
    pub(crate) var_leffnoi: f64,
    pub(crate) var_leffnoi_rv: f64,
    pub(crate) var_leffnoisq: f64,
    pub(crate) var_leffnoisq_rv: f64,
    pub(crate) var_lintnoi_i: f64,
    pub(crate) var_lintnoi_i_rv: f64,
    pub(crate) var_litl: f64,
    pub(crate) var_litl_rv: f64,
    pub(crate) var_lna0: f64,
    pub(crate) var_lna0_dn3: f64,
    pub(crate) var_lna0_dn4: f64,
    pub(crate) var_lna0_dn5: f64,
    pub(crate) var_lna0_dn6: f64,
    pub(crate) var_lna0_dn7: f64,
    pub(crate) var_lna0_dn8: f64,
    pub(crate) var_lna0_rv: f64,
    pub(crate) var_lnew: f64,
    pub(crate) var_lnew_rv: f64,
    pub(crate) var_lovd_i: f64,
    pub(crate) var_lovd_i_rv: f64,
    pub(crate) var_lovs_i: f64,
    pub(crate) var_lovs_i_rv: f64,
    pub(crate) var_lpe0_i: f64,
    pub(crate) var_lpe0_i_rv: f64,
    pub(crate) var_lw_lln_lwn: f64,
    pub(crate) var_lw_lln_lwn_rv: f64,
    pub(crate) var_lw_wln_wwn: f64,
    pub(crate) var_lw_wln_wwn_rv: f64,
    pub(crate) var_mclm: f64,
    pub(crate) var_mclm_dn3: f64,
    pub(crate) var_mclm_dn4: f64,
    pub(crate) var_mclm_dn5: f64,
    pub(crate) var_mclm_dn6: f64,
    pub(crate) var_mclm_dn7: f64,
    pub(crate) var_mclm_dn8: f64,
    pub(crate) var_mclm_rv: f64,
    pub(crate) var_mclmcv: f64,
    pub(crate) var_mclmcv_dn3: f64,
    pub(crate) var_mclmcv_dn4: f64,
    pub(crate) var_mclmcv_dn5: f64,
    pub(crate) var_mclmcv_dn6: f64,
    pub(crate) var_mclmcv_dn7: f64,
    pub(crate) var_mclmcv_dn8: f64,
    pub(crate) var_mclmcv_rv: f64,
    pub(crate) var_mexp_i: f64,
    pub(crate) var_mexp_i_rv: f64,
    pub(crate) var_mexp_t: f64,
    pub(crate) var_mexp_t_dn4: f64,
    pub(crate) var_mexp_t_rv: f64,
    pub(crate) var_mnud: f64,
    pub(crate) var_mnud_dn3: f64,
    pub(crate) var_mnud_dn4: f64,
    pub(crate) var_mnud_dn5: f64,
    pub(crate) var_mnud_dn6: f64,
    pub(crate) var_mnud_dn7: f64,
    pub(crate) var_mnud_dn8: f64,
    pub(crate) var_mnud_rv: f64,
    pub(crate) var_moc: f64,
    pub(crate) var_moc_dn3: f64,
    pub(crate) var_moc_dn4: f64,
    pub(crate) var_moc_dn5: f64,
    pub(crate) var_moc_dn6: f64,
    pub(crate) var_moc_dn7: f64,
    pub(crate) var_moc_dn8: f64,
    pub(crate) var_moc_rv: f64,
    pub(crate) var_mpower_i: f64,
    pub(crate) var_mpower_i_rv: f64,
    pub(crate) var_n0: f64,
    pub(crate) var_n0_dn3: f64,
    pub(crate) var_n0_dn4: f64,
    pub(crate) var_n0_dn5: f64,
    pub(crate) var_n0_dn6: f64,
    pub(crate) var_n0_dn7: f64,
    pub(crate) var_n0_dn8: f64,
    pub(crate) var_n0_rv: f64,
    pub(crate) var_nbody_i: f64,
    pub(crate) var_nbody_i_rv: f64,
    pub(crate) var_ni: f64,
    pub(crate) var_ni_dn3: f64,
    pub(crate) var_ni_dn4: f64,
    pub(crate) var_ni_dn5: f64,
    pub(crate) var_ni_dn6: f64,
    pub(crate) var_ni_dn7: f64,
    pub(crate) var_ni_dn8: f64,
    pub(crate) var_ni_rv: f64,
    pub(crate) var_nigbacc_i: f64,
    pub(crate) var_nigbacc_i_rv: f64,
    pub(crate) var_nigbinv_i: f64,
    pub(crate) var_nigbinv_i_rv: f64,
    pub(crate) var_nl: f64,
    pub(crate) var_nl_dn3: f64,
    pub(crate) var_nl_dn4: f64,
    pub(crate) var_nl_dn5: f64,
    pub(crate) var_nl_dn6: f64,
    pub(crate) var_nl_dn7: f64,
    pub(crate) var_nl_dn8: f64,
    pub(crate) var_nl_rv: f64,
    pub(crate) var_noia2_i: f64,
    pub(crate) var_noia2_i_rv: f64,
    pub(crate) var_noiaeff: f64,
    pub(crate) var_noiaeff_dn3: f64,
    pub(crate) var_noiaeff_dn4: f64,
    pub(crate) var_noiaeff_dn5: f64,
    pub(crate) var_noiaeff_dn6: f64,
    pub(crate) var_noiaeff_dn7: f64,
    pub(crate) var_noiaeff_dn8: f64,
    pub(crate) var_noiaeff_rv: f64,
    pub(crate) var_nsd_i: f64,
    pub(crate) var_nsd_i_rv: f64,
    pub(crate) var_nstar: f64,
    pub(crate) var_nstar_dn3: f64,
    pub(crate) var_nstar_dn4: f64,
    pub(crate) var_nstar_dn5: f64,
    pub(crate) var_nstar_dn6: f64,
    pub(crate) var_nstar_dn7: f64,
    pub(crate) var_nstar_dn8: f64,
    pub(crate) var_nstar_rv: f64,
    pub(crate) var_ntox_i: f64,
    pub(crate) var_nvtm: f64,
    pub(crate) var_nvtm_dn3: f64,
    pub(crate) var_nvtm_dn4: f64,
    pub(crate) var_nvtm_dn5: f64,
    pub(crate) var_nvtm_dn6: f64,
    pub(crate) var_nvtm_dn7: f64,
    pub(crate) var_nvtm_dn8: f64,
    pub(crate) var_nvtm_rv: f64,
    pub(crate) var_pclm_i: f64,
    pub(crate) var_pclm_i_rv: f64,
    pub(crate) var_pclmcv_i: f64,
    pub(crate) var_pclmcv_i_rv: f64,
    pub(crate) var_pdibl1_i: f64,
    pub(crate) var_pdibl1_i_rv: f64,
    pub(crate) var_pdibl2_i: f64,
    pub(crate) var_pdibl2_i_rv: f64,
    pub(crate) var_pgidl_i: f64,
    pub(crate) var_pgidl_i_rv: f64,
    pub(crate) var_pgisl_i: f64,
    pub(crate) var_pgisl_i_rv: f64,
    pub(crate) var_phi1: f64,
    pub(crate) var_phi1_0: f64,
    pub(crate) var_phi1_0_dn3: f64,
    pub(crate) var_phi1_0_dn4: f64,
    pub(crate) var_phi1_0_dn5: f64,
    pub(crate) var_phi1_0_dn6: f64,
    pub(crate) var_phi1_0_dn7: f64,
    pub(crate) var_phi1_0_dn8: f64,
    pub(crate) var_phi1_0_rv: f64,
    pub(crate) var_phi1_dn3: f64,
    pub(crate) var_phi1_dn4: f64,
    pub(crate) var_phi1_dn5: f64,
    pub(crate) var_phi1_dn6: f64,
    pub(crate) var_phi1_dn7: f64,
    pub(crate) var_phi1_dn8: f64,
    pub(crate) var_phi1_rv: f64,
    pub(crate) var_phi2: f64,
    pub(crate) var_phi2_dn3: f64,
    pub(crate) var_phi2_dn4: f64,
    pub(crate) var_phi2_dn5: f64,
    pub(crate) var_phi2_dn6: f64,
    pub(crate) var_phi2_dn7: f64,
    pub(crate) var_phi2_dn8: f64,
    pub(crate) var_phi2_rv: f64,
    pub(crate) var_phi2sub: f64,
    pub(crate) var_phi2sub_dn3: f64,
    pub(crate) var_phi2sub_dn4: f64,
    pub(crate) var_phi2sub_dn5: f64,
    pub(crate) var_phi2sub_dn6: f64,
    pub(crate) var_phi2sub_dn7: f64,
    pub(crate) var_phi2sub_dn8: f64,
    pub(crate) var_phi2sub_rv: f64,
    pub(crate) var_phib: f64,
    pub(crate) var_phib_dn3: f64,
    pub(crate) var_phib_dn4: f64,
    pub(crate) var_phib_dn5: f64,
    pub(crate) var_phib_dn6: f64,
    pub(crate) var_phib_dn7: f64,
    pub(crate) var_phib_dn8: f64,
    pub(crate) var_phib_rv: f64,
    pub(crate) var_phifs: f64,
    pub(crate) var_phifs_dn3: f64,
    pub(crate) var_phifs_dn4: f64,
    pub(crate) var_phifs_dn5: f64,
    pub(crate) var_phifs_dn6: f64,
    pub(crate) var_phifs_dn7: f64,
    pub(crate) var_phifs_dn8: f64,
    pub(crate) var_phifs_rv: f64,
    pub(crate) var_phig1_i: f64,
    pub(crate) var_phig1_i_rv: f64,
    pub(crate) var_phig2_i: f64,
    pub(crate) var_phig2_i_dn3: f64,
    pub(crate) var_phig2_i_dn4: f64,
    pub(crate) var_phig2_i_dn5: f64,
    pub(crate) var_phig2_i_dn6: f64,
    pub(crate) var_phig2_i_dn7: f64,
    pub(crate) var_phig2_i_dn8: f64,
    pub(crate) var_phig2_i_rv: f64,
    pub(crate) var_phin_i: f64,
    pub(crate) var_phin_i_rv: f64,
    pub(crate) var_phiref: f64,
    pub(crate) var_phiref_dn4: f64,
    pub(crate) var_phiref_rv: f64,
    pub(crate) var_phisd: f64,
    pub(crate) var_phisd_dn3: f64,
    pub(crate) var_phisd_dn4: f64,
    pub(crate) var_phisd_dn5: f64,
    pub(crate) var_phisd_dn6: f64,
    pub(crate) var_phisd_dn7: f64,
    pub(crate) var_phisd_dn8: f64,
    pub(crate) var_phisd_rv: f64,
    pub(crate) var_phissat: f64,
    pub(crate) var_phissat_dn3: f64,
    pub(crate) var_phissat_dn4: f64,
    pub(crate) var_phissat_dn5: f64,
    pub(crate) var_phissat_dn6: f64,
    pub(crate) var_phissat_dn7: f64,
    pub(crate) var_phissat_dn8: f64,
    pub(crate) var_phissat_rv: f64,
    pub(crate) var_phissatback: f64,
    pub(crate) var_phissatback2: f64,
    pub(crate) var_phissatback2_dn3: f64,
    pub(crate) var_phissatback2_dn4: f64,
    pub(crate) var_phissatback2_dn5: f64,
    pub(crate) var_phissatback2_dn6: f64,
    pub(crate) var_phissatback2_dn7: f64,
    pub(crate) var_phissatback2_dn8: f64,
    pub(crate) var_phissatback2_rv: f64,
    pub(crate) var_phissatback_dn3: f64,
    pub(crate) var_phissatback_dn4: f64,
    pub(crate) var_phissatback_dn5: f64,
    pub(crate) var_phissatback_dn6: f64,
    pub(crate) var_phissatback_dn7: f64,
    pub(crate) var_phissatback_dn8: f64,
    pub(crate) var_phissatback_rv: f64,
    pub(crate) var_phist: f64,
    pub(crate) var_phist_dn3: f64,
    pub(crate) var_phist_dn4: f64,
    pub(crate) var_phist_dn5: f64,
    pub(crate) var_phist_dn6: f64,
    pub(crate) var_phist_dn7: f64,
    pub(crate) var_phist_dn8: f64,
    pub(crate) var_phist_rv: f64,
    pub(crate) var_phisub: f64,
    pub(crate) var_phisub_dn3: f64,
    pub(crate) var_phisub_dn4: f64,
    pub(crate) var_phisub_dn5: f64,
    pub(crate) var_phisub_dn6: f64,
    pub(crate) var_phisub_dn7: f64,
    pub(crate) var_phisub_dn8: f64,
    pub(crate) var_phisub_rv: f64,
    pub(crate) var_pigcd_i: f64,
    pub(crate) var_pigcd_i_rv: f64,
    pub(crate) var_poxedge_i: f64,
    pub(crate) var_poxedge_i_rv: f64,
    pub(crate) var_pqm_i: f64,
    pub(crate) var_pqm_i_rv: f64,
    pub(crate) var_prt_i: f64,
    pub(crate) var_prt_i_rv: f64,
    pub(crate) var_prwb_i: f64,
    pub(crate) var_prwb_i_rv: f64,
    pub(crate) var_prwg_i: f64,
    pub(crate) var_prwg_i_rv: f64,
    pub(crate) var_ptwg_i: f64,
    pub(crate) var_ptwg_i_rv: f64,
    pub(crate) var_ptwg_t: f64,
    pub(crate) var_ptwg_t_dn4: f64,
    pub(crate) var_ptwg_t_rv: f64,
    pub(crate) var_ptwgb2_i: f64,
    pub(crate) var_ptwgb2_i_rv: f64,
    pub(crate) var_ptwgb_i: f64,
    pub(crate) var_ptwgb_i_rv: f64,
    pub(crate) var_ptwgt_i: f64,
    pub(crate) var_ptwgt_i_rv: f64,
    pub(crate) var_pvag_i: f64,
    pub(crate) var_pvag_i_rv: f64,
    pub(crate) var_pvagfactor: f64,
    pub(crate) var_pvagfactor_dn3: f64,
    pub(crate) var_pvagfactor_dn4: f64,
    pub(crate) var_pvagfactor_dn5: f64,
    pub(crate) var_pvagfactor_dn6: f64,
    pub(crate) var_pvagfactor_dn7: f64,
    pub(crate) var_pvagfactor_dn8: f64,
    pub(crate) var_pvagfactor_rv: f64,
    pub(crate) var_q: f64,
    pub(crate) var_q1: f64,
    pub(crate) var_q1_dn3: f64,
    pub(crate) var_q1_dn4: f64,
    pub(crate) var_q1_dn5: f64,
    pub(crate) var_q1_dn6: f64,
    pub(crate) var_q1_dn7: f64,
    pub(crate) var_q1_dn8: f64,
    pub(crate) var_q1_rv: f64,
    pub(crate) var_q2: f64,
    pub(crate) var_q2_dn3: f64,
    pub(crate) var_q2_dn4: f64,
    pub(crate) var_q2_dn5: f64,
    pub(crate) var_q2_dn6: f64,
    pub(crate) var_q2_dn7: f64,
    pub(crate) var_q2_dn8: f64,
    pub(crate) var_q2_rv: f64,
    pub(crate) var_q_dn3: f64,
    pub(crate) var_q_dn4: f64,
    pub(crate) var_q_dn5: f64,
    pub(crate) var_q_dn6: f64,
    pub(crate) var_q_dn7: f64,
    pub(crate) var_q_dn8: f64,
    pub(crate) var_q_rv: f64,
    pub(crate) var_qb0: f64,
    pub(crate) var_qb0_rv: f64,
    pub(crate) var_qba: f64,
    pub(crate) var_qba_rv: f64,
    pub(crate) var_qbackd: f64,
    pub(crate) var_qbackd_dn3: f64,
    pub(crate) var_qbackd_dn4: f64,
    pub(crate) var_qbackd_dn5: f64,
    pub(crate) var_qbackd_dn6: f64,
    pub(crate) var_qbackd_dn7: f64,
    pub(crate) var_qbackd_dn8: f64,
    pub(crate) var_qbackd_rv: f64,
    pub(crate) var_qbacks: f64,
    pub(crate) var_qbacks_dn3: f64,
    pub(crate) var_qbacks_dn4: f64,
    pub(crate) var_qbacks_dn5: f64,
    pub(crate) var_qbacks_dn6: f64,
    pub(crate) var_qbacks_dn7: f64,
    pub(crate) var_qbacks_dn8: f64,
    pub(crate) var_qbacks_rv: f64,
    pub(crate) var_qbg: f64,
    pub(crate) var_qbg_dn3: f64,
    pub(crate) var_qbg_dn4: f64,
    pub(crate) var_qbg_dn5: f64,
    pub(crate) var_qbg_dn6: f64,
    pub(crate) var_qbg_dn7: f64,
    pub(crate) var_qbg_dn8: f64,
    pub(crate) var_qbg_rv: f64,
    pub(crate) var_qbgi: f64,
    pub(crate) var_qbgi_dn3: f64,
    pub(crate) var_qbgi_dn4: f64,
    pub(crate) var_qbgi_dn5: f64,
    pub(crate) var_qbgi_dn6: f64,
    pub(crate) var_qbgi_dn7: f64,
    pub(crate) var_qbgi_dn8: f64,
    pub(crate) var_qbgi_rv: f64,
    pub(crate) var_qbs: f64,
    pub(crate) var_qbs_rv: f64,
    pub(crate) var_qcoth: f64,
    pub(crate) var_qcoth1: f64,
    pub(crate) var_qcoth1_dn3: f64,
    pub(crate) var_qcoth1_dn4: f64,
    pub(crate) var_qcoth1_dn5: f64,
    pub(crate) var_qcoth1_dn6: f64,
    pub(crate) var_qcoth1_dn7: f64,
    pub(crate) var_qcoth1_dn8: f64,
    pub(crate) var_qcoth1_rv: f64,
    pub(crate) var_qcoth_dn3: f64,
    pub(crate) var_qcoth_dn4: f64,
    pub(crate) var_qcoth_dn5: f64,
    pub(crate) var_qcoth_dn6: f64,
    pub(crate) var_qcoth_dn7: f64,
    pub(crate) var_qcoth_dn8: f64,
    pub(crate) var_qcoth_rv: f64,
    pub(crate) var_qd: f64,
    pub(crate) var_qd_dn3: f64,
    pub(crate) var_qd_dn4: f64,
    pub(crate) var_qd_dn5: f64,
    pub(crate) var_qd_dn6: f64,
    pub(crate) var_qd_dn7: f64,
    pub(crate) var_qd_dn8: f64,
    pub(crate) var_qd_rv: f64,
    pub(crate) var_qdbg: f64,
    pub(crate) var_qdbg_dn3: f64,
    pub(crate) var_qdbg_dn4: f64,
    pub(crate) var_qdbg_dn5: f64,
    pub(crate) var_qdbg_dn6: f64,
    pub(crate) var_qdbg_dn7: f64,
    pub(crate) var_qdbg_dn8: f64,
    pub(crate) var_qdbg_rv: f64,
    pub(crate) var_qdi: f64,
    pub(crate) var_qdi_dn3: f64,
    pub(crate) var_qdi_dn4: f64,
    pub(crate) var_qdi_dn5: f64,
    pub(crate) var_qdi_dn6: f64,
    pub(crate) var_qdi_dn7: f64,
    pub(crate) var_qdi_dn8: f64,
    pub(crate) var_qdi_rv: f64,
    pub(crate) var_qfg: f64,
    pub(crate) var_qfg_dn3: f64,
    pub(crate) var_qfg_dn4: f64,
    pub(crate) var_qfg_dn5: f64,
    pub(crate) var_qfg_dn6: f64,
    pub(crate) var_qfg_dn7: f64,
    pub(crate) var_qfg_dn8: f64,
    pub(crate) var_qfg_rv: f64,
    pub(crate) var_qfgd_of: f64,
    pub(crate) var_qfgd_of_dn5: f64,
    pub(crate) var_qfgd_of_dn7: f64,
    pub(crate) var_qfgd_of_rv: f64,
    pub(crate) var_qfgd_ov: f64,
    pub(crate) var_qfgd_ov_dn3: f64,
    pub(crate) var_qfgd_ov_dn4: f64,
    pub(crate) var_qfgd_ov_dn5: f64,
    pub(crate) var_qfgd_ov_dn6: f64,
    pub(crate) var_qfgd_ov_dn7: f64,
    pub(crate) var_qfgd_ov_dn8: f64,
    pub(crate) var_qfgd_ov_rv: f64,
    pub(crate) var_qfgd_parasitic: f64,
    pub(crate) var_qfgd_parasitic_dn3: f64,
    pub(crate) var_qfgd_parasitic_dn4: f64,
    pub(crate) var_qfgd_parasitic_dn5: f64,
    pub(crate) var_qfgd_parasitic_dn6: f64,
    pub(crate) var_qfgd_parasitic_dn7: f64,
    pub(crate) var_qfgd_parasitic_dn8: f64,
    pub(crate) var_qfgd_parasitic_rv: f64,
    pub(crate) var_qfgi: f64,
    pub(crate) var_qfgi_dn3: f64,
    pub(crate) var_qfgi_dn4: f64,
    pub(crate) var_qfgi_dn5: f64,
    pub(crate) var_qfgi_dn6: f64,
    pub(crate) var_qfgi_dn7: f64,
    pub(crate) var_qfgi_dn8: f64,
    pub(crate) var_qfgi_rv: f64,
    pub(crate) var_qfgs_of: f64,
    pub(crate) var_qfgs_of_dn6: f64,
    pub(crate) var_qfgs_of_dn7: f64,
    pub(crate) var_qfgs_of_rv: f64,
    pub(crate) var_qfgs_ov: f64,
    pub(crate) var_qfgs_ov_dn3: f64,
    pub(crate) var_qfgs_ov_dn4: f64,
    pub(crate) var_qfgs_ov_dn5: f64,
    pub(crate) var_qfgs_ov_dn6: f64,
    pub(crate) var_qfgs_ov_dn7: f64,
    pub(crate) var_qfgs_ov_dn8: f64,
    pub(crate) var_qfgs_ov_rv: f64,
    pub(crate) var_qfgs_parasitic: f64,
    pub(crate) var_qfgs_parasitic_dn3: f64,
    pub(crate) var_qfgs_parasitic_dn4: f64,
    pub(crate) var_qfgs_parasitic_dn5: f64,
    pub(crate) var_qfgs_parasitic_dn6: f64,
    pub(crate) var_qfgs_parasitic_dn7: f64,
    pub(crate) var_qfgs_parasitic_dn8: f64,
    pub(crate) var_qfgs_parasitic_rv: f64,
    pub(crate) var_qfrontd: f64,
    pub(crate) var_qfrontd_dn3: f64,
    pub(crate) var_qfrontd_dn4: f64,
    pub(crate) var_qfrontd_dn5: f64,
    pub(crate) var_qfrontd_dn6: f64,
    pub(crate) var_qfrontd_dn7: f64,
    pub(crate) var_qfrontd_dn8: f64,
    pub(crate) var_qfrontd_rv: f64,
    pub(crate) var_qfronts: f64,
    pub(crate) var_qfronts_dn3: f64,
    pub(crate) var_qfronts_dn4: f64,
    pub(crate) var_qfronts_dn5: f64,
    pub(crate) var_qfronts_dn6: f64,
    pub(crate) var_qfronts_dn7: f64,
    pub(crate) var_qfronts_dn8: f64,
    pub(crate) var_qfronts_rv: f64,
    pub(crate) var_qia: f64,
    pub(crate) var_qia2: f64,
    pub(crate) var_qia2_dn3: f64,
    pub(crate) var_qia2_dn4: f64,
    pub(crate) var_qia2_dn5: f64,
    pub(crate) var_qia2_dn6: f64,
    pub(crate) var_qia2_dn7: f64,
    pub(crate) var_qia2_dn8: f64,
    pub(crate) var_qia2_rv: f64,
    pub(crate) var_qia_dn3: f64,
    pub(crate) var_qia_dn4: f64,
    pub(crate) var_qia_dn5: f64,
    pub(crate) var_qia_dn6: f64,
    pub(crate) var_qia_dn7: f64,
    pub(crate) var_qia_dn8: f64,
    pub(crate) var_qia_rv: f64,
    pub(crate) var_qib2: f64,
    pub(crate) var_qib2_dn3: f64,
    pub(crate) var_qib2_dn4: f64,
    pub(crate) var_qib2_dn5: f64,
    pub(crate) var_qib2_dn6: f64,
    pub(crate) var_qib2_dn7: f64,
    pub(crate) var_qib2_dn8: f64,
    pub(crate) var_qib2_rv: f64,
    pub(crate) var_qicored: f64,
    pub(crate) var_qicored_dn3: f64,
    pub(crate) var_qicored_dn4: f64,
    pub(crate) var_qicored_dn5: f64,
    pub(crate) var_qicored_dn6: f64,
    pub(crate) var_qicored_dn7: f64,
    pub(crate) var_qicored_dn8: f64,
    pub(crate) var_qicored_rv: f64,
    pub(crate) var_qicores: f64,
    pub(crate) var_qicores_dn3: f64,
    pub(crate) var_qicores_dn4: f64,
    pub(crate) var_qicores_dn5: f64,
    pub(crate) var_qicores_dn6: f64,
    pub(crate) var_qicores_dn7: f64,
    pub(crate) var_qicores_dn8: f64,
    pub(crate) var_qicores_rv: f64,
    pub(crate) var_qid: f64,
    pub(crate) var_qid_dn3: f64,
    pub(crate) var_qid_dn4: f64,
    pub(crate) var_qid_dn5: f64,
    pub(crate) var_qid_dn6: f64,
    pub(crate) var_qid_dn7: f64,
    pub(crate) var_qid_dn8: f64,
    pub(crate) var_qid_rv: f64,
    pub(crate) var_qinv: f64,
    pub(crate) var_qinv_dn3: f64,
    pub(crate) var_qinv_dn4: f64,
    pub(crate) var_qinv_dn5: f64,
    pub(crate) var_qinv_dn6: f64,
    pub(crate) var_qinv_dn7: f64,
    pub(crate) var_qinv_dn8: f64,
    pub(crate) var_qinv_rv: f64,
    pub(crate) var_qis: f64,
    pub(crate) var_qis_dn3: f64,
    pub(crate) var_qis_dn4: f64,
    pub(crate) var_qis_dn5: f64,
    pub(crate) var_qis_dn6: f64,
    pub(crate) var_qis_dn7: f64,
    pub(crate) var_qis_dn8: f64,
    pub(crate) var_qis_rv: f64,
    pub(crate) var_qm0_i: f64,
    pub(crate) var_qm0_i_rv: f64,
    pub(crate) var_qmtcencv_i: f64,
    pub(crate) var_qmtcencv_i_rv: f64,
    pub(crate) var_qs: f64,
    pub(crate) var_qs_dn3: f64,
    pub(crate) var_qs_dn4: f64,
    pub(crate) var_qs_dn5: f64,
    pub(crate) var_qs_dn6: f64,
    pub(crate) var_qs_dn7: f64,
    pub(crate) var_qs_dn8: f64,
    pub(crate) var_qs_rv: f64,
    pub(crate) var_qsbg: f64,
    pub(crate) var_qsbg_dn3: f64,
    pub(crate) var_qsbg_dn4: f64,
    pub(crate) var_qsbg_dn5: f64,
    pub(crate) var_qsbg_dn6: f64,
    pub(crate) var_qsbg_dn7: f64,
    pub(crate) var_qsbg_dn8: f64,
    pub(crate) var_qsbg_rv: f64,
    pub(crate) var_qsi: f64,
    pub(crate) var_qsi_dn3: f64,
    pub(crate) var_qsi_dn4: f64,
    pub(crate) var_qsi_dn5: f64,
    pub(crate) var_qsi_dn6: f64,
    pub(crate) var_qsi_dn7: f64,
    pub(crate) var_qsi_dn8: f64,
    pub(crate) var_qsi_rv: f64,
    pub(crate) var_qsq1: f64,
    pub(crate) var_qsq1_dn3: f64,
    pub(crate) var_qsq1_dn4: f64,
    pub(crate) var_qsq1_dn5: f64,
    pub(crate) var_qsq1_dn6: f64,
    pub(crate) var_qsq1_dn7: f64,
    pub(crate) var_qsq1_dn8: f64,
    pub(crate) var_qsq1_rv: f64,
    pub(crate) var_qsqrt: f64,
    pub(crate) var_qsqrt1: f64,
    pub(crate) var_qsqrt1_dn3: f64,
    pub(crate) var_qsqrt1_dn4: f64,
    pub(crate) var_qsqrt1_dn5: f64,
    pub(crate) var_qsqrt1_dn6: f64,
    pub(crate) var_qsqrt1_dn7: f64,
    pub(crate) var_qsqrt1_dn8: f64,
    pub(crate) var_qsqrt1_rv: f64,
    pub(crate) var_qsqrt_dn3: f64,
    pub(crate) var_qsqrt_dn4: f64,
    pub(crate) var_qsqrt_dn5: f64,
    pub(crate) var_qsqrt_dn6: f64,
    pub(crate) var_qsqrt_dn7: f64,
    pub(crate) var_qsqrt_dn8: f64,
    pub(crate) var_qsqrt_rv: f64,
    pub(crate) var_qsref_i: f64,
    pub(crate) var_qsref_i_rv: f64,
    pub(crate) var_qt: f64,
    pub(crate) var_qt_dn3: f64,
    pub(crate) var_qt_dn4: f64,
    pub(crate) var_qt_dn5: f64,
    pub(crate) var_qt_dn6: f64,
    pub(crate) var_qt_dn7: f64,
    pub(crate) var_qt_dn8: f64,
    pub(crate) var_qt_rv: f64,
    pub(crate) var_qth: f64,
    pub(crate) var_qth_rv: f64,
    pub(crate) var_qtotd: f64,
    pub(crate) var_qtotd_dn3: f64,
    pub(crate) var_qtotd_dn4: f64,
    pub(crate) var_qtotd_dn5: f64,
    pub(crate) var_qtotd_dn6: f64,
    pub(crate) var_qtotd_dn7: f64,
    pub(crate) var_qtotd_dn8: f64,
    pub(crate) var_qtotd_rv: f64,
    pub(crate) var_qtots: f64,
    pub(crate) var_qtots_dn3: f64,
    pub(crate) var_qtots_dn4: f64,
    pub(crate) var_qtots_dn5: f64,
    pub(crate) var_qtots_dn6: f64,
    pub(crate) var_qtots_dn7: f64,
    pub(crate) var_qtots_dn8: f64,
    pub(crate) var_qtots_rv: f64,
    pub(crate) var_rdrain: f64,
    pub(crate) var_rdrain_dn3: f64,
    pub(crate) var_rdrain_dn4: f64,
    pub(crate) var_rdrain_dn5: f64,
    pub(crate) var_rdrain_dn6: f64,
    pub(crate) var_rdrain_dn7: f64,
    pub(crate) var_rdrain_dn8: f64,
    pub(crate) var_rdraingeo: f64,
    pub(crate) var_rdraingeo_rv: f64,
    pub(crate) var_rdsi: f64,
    pub(crate) var_rdsi_dn3: f64,
    pub(crate) var_rdsi_dn4: f64,
    pub(crate) var_rdsi_dn5: f64,
    pub(crate) var_rdsi_dn6: f64,
    pub(crate) var_rdsi_dn7: f64,
    pub(crate) var_rdsi_dn8: f64,
    pub(crate) var_rdsi_rv: f64,
    pub(crate) var_rdss: f64,
    pub(crate) var_rdss_dn3: f64,
    pub(crate) var_rdss_dn4: f64,
    pub(crate) var_rdss_dn5: f64,
    pub(crate) var_rdss_dn6: f64,
    pub(crate) var_rdss_dn7: f64,
    pub(crate) var_rdss_dn8: f64,
    pub(crate) var_rdss_rv: f64,
    pub(crate) var_rdstemp: f64,
    pub(crate) var_rdstemp_dn4: f64,
    pub(crate) var_rdstemp_rv: f64,
    pub(crate) var_rdsw_i: f64,
    pub(crate) var_rdsw_i_rv: f64,
    pub(crate) var_rdswmin_i: f64,
    pub(crate) var_rdswmin_i_rv: f64,
    pub(crate) var_rdw_i: f64,
    pub(crate) var_rdwmin_i: f64,
    pub(crate) var_rsource: f64,
    pub(crate) var_rsource_dn3: f64,
    pub(crate) var_rsource_dn4: f64,
    pub(crate) var_rsource_dn5: f64,
    pub(crate) var_rsource_dn6: f64,
    pub(crate) var_rsource_dn7: f64,
    pub(crate) var_rsource_dn8: f64,
    pub(crate) var_rsourcegeo: f64,
    pub(crate) var_rsourcegeo_rv: f64,
    pub(crate) var_rsw_i: f64,
    pub(crate) var_rswmin_i: f64,
    pub(crate) var_scl: f64,
    pub(crate) var_scl_dn3: f64,
    pub(crate) var_scl_dn4: f64,
    pub(crate) var_scl_dn5: f64,
    pub(crate) var_scl_dn6: f64,
    pub(crate) var_scl_dn7: f64,
    pub(crate) var_scl_dn8: f64,
    pub(crate) var_scl_rv: f64,
    pub(crate) var_sclf: f64,
    pub(crate) var_sclf_rv: f64,
    pub(crate) var_sclm: f64,
    pub(crate) var_sclm_rv: f64,
    pub(crate) var_sigvds: f64,
    pub(crate) var_sigvds_rv: f64,
    pub(crate) var_ssi: f64,
    pub(crate) var_ssi_dn3: f64,
    pub(crate) var_ssi_dn4: f64,
    pub(crate) var_ssi_dn5: f64,
    pub(crate) var_ssi_dn6: f64,
    pub(crate) var_ssi_dn7: f64,
    pub(crate) var_ssi_dn8: f64,
    pub(crate) var_ssi_rv: f64,
    pub(crate) var_swi: f64,
    pub(crate) var_swi_dn3: f64,
    pub(crate) var_swi_dn4: f64,
    pub(crate) var_swi_dn5: f64,
    pub(crate) var_swi_dn6: f64,
    pub(crate) var_swi_dn7: f64,
    pub(crate) var_swi_dn8: f64,
    pub(crate) var_swi_rv: f64,
    pub(crate) var_symmetry_factor: f64,
    pub(crate) var_symmetry_factor_dn5: f64,
    pub(crate) var_symmetry_factor_dn6: f64,
    pub(crate) var_symmetry_factor_rv: f64,
    pub(crate) var_t0: f64,
    pub(crate) var_t0_dn3: f64,
    pub(crate) var_t0_dn4: f64,
    pub(crate) var_t0_dn5: f64,
    pub(crate) var_t0_dn6: f64,
    pub(crate) var_t0_dn7: f64,
    pub(crate) var_t0_dn8: f64,
    pub(crate) var_t0_rv: f64,
    pub(crate) var_t1: f64,
    pub(crate) var_t10: f64,
    pub(crate) var_t10_dn3: f64,
    pub(crate) var_t10_dn4: f64,
    pub(crate) var_t10_dn5: f64,
    pub(crate) var_t10_dn6: f64,
    pub(crate) var_t10_dn7: f64,
    pub(crate) var_t10_dn8: f64,
    pub(crate) var_t10_rv: f64,
    pub(crate) var_t11: f64,
    pub(crate) var_t11_dn3: f64,
    pub(crate) var_t11_dn4: f64,
    pub(crate) var_t11_dn5: f64,
    pub(crate) var_t11_dn6: f64,
    pub(crate) var_t11_dn7: f64,
    pub(crate) var_t11_dn8: f64,
    pub(crate) var_t11_rv: f64,
    pub(crate) var_t1__blk110: f64,
    pub(crate) var_t1__blk110_dn3: f64,
    pub(crate) var_t1__blk110_dn4: f64,
    pub(crate) var_t1__blk110_dn5: f64,
    pub(crate) var_t1__blk110_dn6: f64,
    pub(crate) var_t1__blk110_dn7: f64,
    pub(crate) var_t1__blk110_dn8: f64,
    pub(crate) var_t1__blk110_rv: f64,
    pub(crate) var_t1_dn3: f64,
    pub(crate) var_t1_dn4: f64,
    pub(crate) var_t1_dn5: f64,
    pub(crate) var_t1_dn6: f64,
    pub(crate) var_t1_dn7: f64,
    pub(crate) var_t1_dn8: f64,
    pub(crate) var_t1_exp: f64,
    pub(crate) var_t1_exp_dn3: f64,
    pub(crate) var_t1_exp_dn4: f64,
    pub(crate) var_t1_exp_dn5: f64,
    pub(crate) var_t1_exp_dn6: f64,
    pub(crate) var_t1_exp_dn7: f64,
    pub(crate) var_t1_exp_dn8: f64,
    pub(crate) var_t1_exp_rv: f64,
    pub(crate) var_t1_rv: f64,
    pub(crate) var_t2: f64,
    pub(crate) var_t2__blk100: f64,
    pub(crate) var_t2__blk100_dn3: f64,
    pub(crate) var_t2__blk100_dn4: f64,
    pub(crate) var_t2__blk100_dn5: f64,
    pub(crate) var_t2__blk100_dn6: f64,
    pub(crate) var_t2__blk100_dn7: f64,
    pub(crate) var_t2__blk100_dn8: f64,
    pub(crate) var_t2__blk100_rv: f64,
    pub(crate) var_t2__blk102: f64,
    pub(crate) var_t2__blk102_dn3: f64,
    pub(crate) var_t2__blk102_dn4: f64,
    pub(crate) var_t2__blk102_dn5: f64,
    pub(crate) var_t2__blk102_dn6: f64,
    pub(crate) var_t2__blk102_dn7: f64,
    pub(crate) var_t2__blk102_dn8: f64,
    pub(crate) var_t2__blk102_rv: f64,
    pub(crate) var_t2__blk114: f64,
    pub(crate) var_t2__blk114_dn3: f64,
    pub(crate) var_t2__blk114_dn4: f64,
    pub(crate) var_t2__blk114_dn5: f64,
    pub(crate) var_t2__blk114_dn6: f64,
    pub(crate) var_t2__blk114_dn7: f64,
    pub(crate) var_t2__blk114_dn8: f64,
    pub(crate) var_t2__blk114_rv: f64,
    pub(crate) var_t2__blk83: f64,
    pub(crate) var_t2__blk83_dn3: f64,
    pub(crate) var_t2__blk83_dn4: f64,
    pub(crate) var_t2__blk83_dn5: f64,
    pub(crate) var_t2__blk83_dn6: f64,
    pub(crate) var_t2__blk83_dn7: f64,
    pub(crate) var_t2__blk83_dn8: f64,
    pub(crate) var_t2__blk83_rv: f64,
    pub(crate) var_t2__blk85: f64,
    pub(crate) var_t2__blk85_dn3: f64,
    pub(crate) var_t2__blk85_dn4: f64,
    pub(crate) var_t2__blk85_dn5: f64,
    pub(crate) var_t2__blk85_dn6: f64,
    pub(crate) var_t2__blk85_dn7: f64,
    pub(crate) var_t2__blk85_dn8: f64,
    pub(crate) var_t2__blk85_rv: f64,
    pub(crate) var_t2_dn3: f64,
    pub(crate) var_t2_dn4: f64,
    pub(crate) var_t2_dn5: f64,
    pub(crate) var_t2_dn6: f64,
    pub(crate) var_t2_dn7: f64,
    pub(crate) var_t2_dn8: f64,
    pub(crate) var_t2_rv: f64,
    pub(crate) var_t3: f64,
    pub(crate) var_t3__blk101: f64,
    pub(crate) var_t3__blk101_dn3: f64,
    pub(crate) var_t3__blk101_dn4: f64,
    pub(crate) var_t3__blk101_dn5: f64,
    pub(crate) var_t3__blk101_dn6: f64,
    pub(crate) var_t3__blk101_dn7: f64,
    pub(crate) var_t3__blk101_dn8: f64,
    pub(crate) var_t3__blk101_rv: f64,
    pub(crate) var_t3__blk103: f64,
    pub(crate) var_t3__blk103_dn3: f64,
    pub(crate) var_t3__blk103_dn4: f64,
    pub(crate) var_t3__blk103_dn5: f64,
    pub(crate) var_t3__blk103_dn6: f64,
    pub(crate) var_t3__blk103_dn7: f64,
    pub(crate) var_t3__blk103_dn8: f64,
    pub(crate) var_t3__blk103_rv: f64,
    pub(crate) var_t3__blk115: f64,
    pub(crate) var_t3__blk115_dn3: f64,
    pub(crate) var_t3__blk115_dn4: f64,
    pub(crate) var_t3__blk115_dn5: f64,
    pub(crate) var_t3__blk115_dn6: f64,
    pub(crate) var_t3__blk115_dn7: f64,
    pub(crate) var_t3__blk115_dn8: f64,
    pub(crate) var_t3__blk115_rv: f64,
    pub(crate) var_t3__blk84: f64,
    pub(crate) var_t3__blk84_dn3: f64,
    pub(crate) var_t3__blk84_dn4: f64,
    pub(crate) var_t3__blk84_dn5: f64,
    pub(crate) var_t3__blk84_dn6: f64,
    pub(crate) var_t3__blk84_dn7: f64,
    pub(crate) var_t3__blk84_dn8: f64,
    pub(crate) var_t3__blk84_rv: f64,
    pub(crate) var_t3__blk86: f64,
    pub(crate) var_t3__blk86_dn3: f64,
    pub(crate) var_t3__blk86_dn4: f64,
    pub(crate) var_t3__blk86_dn5: f64,
    pub(crate) var_t3__blk86_dn6: f64,
    pub(crate) var_t3__blk86_dn7: f64,
    pub(crate) var_t3__blk86_dn8: f64,
    pub(crate) var_t3__blk86_rv: f64,
    pub(crate) var_t3_dn3: f64,
    pub(crate) var_t3_dn4: f64,
    pub(crate) var_t3_dn5: f64,
    pub(crate) var_t3_dn6: f64,
    pub(crate) var_t3_dn7: f64,
    pub(crate) var_t3_dn8: f64,
    pub(crate) var_t3_rv: f64,
    pub(crate) var_t4: f64,
    pub(crate) var_t4__blk111: f64,
    pub(crate) var_t4__blk111_dn3: f64,
    pub(crate) var_t4__blk111_dn4: f64,
    pub(crate) var_t4__blk111_dn5: f64,
    pub(crate) var_t4__blk111_dn6: f64,
    pub(crate) var_t4__blk111_dn7: f64,
    pub(crate) var_t4__blk111_dn8: f64,
    pub(crate) var_t4__blk111_rv: f64,
    pub(crate) var_t4_dn3: f64,
    pub(crate) var_t4_dn4: f64,
    pub(crate) var_t4_dn5: f64,
    pub(crate) var_t4_dn6: f64,
    pub(crate) var_t4_dn7: f64,
    pub(crate) var_t4_dn8: f64,
    pub(crate) var_t4_rv: f64,
    pub(crate) var_t5: f64,
    pub(crate) var_t5_dn3: f64,
    pub(crate) var_t5_dn4: f64,
    pub(crate) var_t5_dn5: f64,
    pub(crate) var_t5_dn6: f64,
    pub(crate) var_t5_dn7: f64,
    pub(crate) var_t5_dn8: f64,
    pub(crate) var_t5_rv: f64,
    pub(crate) var_t6: f64,
    pub(crate) var_t6_dn3: f64,
    pub(crate) var_t6_dn4: f64,
    pub(crate) var_t6_dn5: f64,
    pub(crate) var_t6_dn6: f64,
    pub(crate) var_t6_dn7: f64,
    pub(crate) var_t6_dn8: f64,
    pub(crate) var_t6_rv: f64,
    pub(crate) var_t7: f64,
    pub(crate) var_t7_dn3: f64,
    pub(crate) var_t7_dn4: f64,
    pub(crate) var_t7_dn5: f64,
    pub(crate) var_t7_dn6: f64,
    pub(crate) var_t7_dn7: f64,
    pub(crate) var_t7_dn8: f64,
    pub(crate) var_t7_rv: f64,
    pub(crate) var_t8: f64,
    pub(crate) var_t8_dn3: f64,
    pub(crate) var_t8_dn4: f64,
    pub(crate) var_t8_dn5: f64,
    pub(crate) var_t8_dn6: f64,
    pub(crate) var_t8_dn7: f64,
    pub(crate) var_t8_dn8: f64,
    pub(crate) var_t8_rv: f64,
    pub(crate) var_t9: f64,
    pub(crate) var_t9_dn3: f64,
    pub(crate) var_t9_dn4: f64,
    pub(crate) var_t9_dn5: f64,
    pub(crate) var_t9_dn6: f64,
    pub(crate) var_t9_dn7: f64,
    pub(crate) var_t9_dn8: f64,
    pub(crate) var_t9_rv: f64,
    pub(crate) var_ta: f64,
    pub(crate) var_ta_dn3: f64,
    pub(crate) var_ta_dn4: f64,
    pub(crate) var_ta_dn5: f64,
    pub(crate) var_ta_dn6: f64,
    pub(crate) var_ta_dn7: f64,
    pub(crate) var_ta_dn8: f64,
    pub(crate) var_ta_rv: f64,
    pub(crate) var_tb: f64,
    pub(crate) var_tb_dn3: f64,
    pub(crate) var_tb_dn4: f64,
    pub(crate) var_tb_dn5: f64,
    pub(crate) var_tb_dn6: f64,
    pub(crate) var_tb_dn7: f64,
    pub(crate) var_tb_dn8: f64,
    pub(crate) var_tb_rv: f64,
    pub(crate) var_tc: f64,
    pub(crate) var_tc_dn3: f64,
    pub(crate) var_tc_dn4: f64,
    pub(crate) var_tc_dn5: f64,
    pub(crate) var_tc_dn6: f64,
    pub(crate) var_tc_dn7: f64,
    pub(crate) var_tc_dn8: f64,
    pub(crate) var_tc_rv: f64,
    pub(crate) var_tcen: f64,
    pub(crate) var_tcen0: f64,
    pub(crate) var_tcen0_rv: f64,
    pub(crate) var_tcen_dn3: f64,
    pub(crate) var_tcen_dn4: f64,
    pub(crate) var_tcen_dn5: f64,
    pub(crate) var_tcen_dn6: f64,
    pub(crate) var_tcen_dn7: f64,
    pub(crate) var_tcen_dn8: f64,
    pub(crate) var_tcen_rv: f64,
    pub(crate) var_teff: f64,
    pub(crate) var_teff_rv: f64,
    pub(crate) var_temp: f64,
    pub(crate) var_temp_dn3: f64,
    pub(crate) var_temp_dn4: f64,
    pub(crate) var_temp_dn5: f64,
    pub(crate) var_temp_dn6: f64,
    pub(crate) var_temp_dn7: f64,
    pub(crate) var_temp_dn8: f64,
    pub(crate) var_temp_rv: f64,
    pub(crate) var_tgidl_i: f64,
    pub(crate) var_tgidl_i_rv: f64,
    pub(crate) var_tgisl_i: f64,
    pub(crate) var_tgisl_i_rv: f64,
    pub(crate) var_theta_dibl: f64,
    pub(crate) var_theta_dibl_dn3: f64,
    pub(crate) var_theta_dibl_dn4: f64,
    pub(crate) var_theta_dibl_dn5: f64,
    pub(crate) var_theta_dibl_dn6: f64,
    pub(crate) var_theta_dibl_dn7: f64,
    pub(crate) var_theta_dibl_dn8: f64,
    pub(crate) var_theta_dibl_rv: f64,
    pub(crate) var_theta_dits: f64,
    pub(crate) var_theta_dits_dn3: f64,
    pub(crate) var_theta_dits_dn4: f64,
    pub(crate) var_theta_dits_dn5: f64,
    pub(crate) var_theta_dits_dn6: f64,
    pub(crate) var_theta_dits_dn7: f64,
    pub(crate) var_theta_dits_dn8: f64,
    pub(crate) var_theta_dits_rv: f64,
    pub(crate) var_theta_rsce: f64,
    pub(crate) var_theta_rsce_rv: f64,
    pub(crate) var_theta_sce: f64,
    pub(crate) var_theta_sce_dn3: f64,
    pub(crate) var_theta_sce_dn4: f64,
    pub(crate) var_theta_sce_dn5: f64,
    pub(crate) var_theta_sce_dn6: f64,
    pub(crate) var_theta_sce_dn7: f64,
    pub(crate) var_theta_sce_dn8: f64,
    pub(crate) var_theta_sce_rv: f64,
    pub(crate) var_tmaxk: f64,
    pub(crate) var_tmaxk_rv: f64,
    pub(crate) var_tmp: f64,
    pub(crate) var_tmp_dn3: f64,
    pub(crate) var_tmp_dn4: f64,
    pub(crate) var_tmp_dn5: f64,
    pub(crate) var_tmp_dn6: f64,
    pub(crate) var_tmp_dn7: f64,
    pub(crate) var_tmp_dn8: f64,
    pub(crate) var_tmp_rv: f64,
    pub(crate) var_tnom: f64,
    pub(crate) var_tnom_rv: f64,
    pub(crate) var_toxratio: f64,
    pub(crate) var_toxratio_dn3: f64,
    pub(crate) var_toxratio_dn4: f64,
    pub(crate) var_toxratio_dn5: f64,
    pub(crate) var_toxratio_dn6: f64,
    pub(crate) var_toxratio_dn7: f64,
    pub(crate) var_toxratio_dn8: f64,
    pub(crate) var_toxratioedge: f64,
    pub(crate) var_toxratioedge_dn3: f64,
    pub(crate) var_toxratioedge_dn4: f64,
    pub(crate) var_toxratioedge_dn5: f64,
    pub(crate) var_toxratioedge_dn6: f64,
    pub(crate) var_toxratioedge_dn7: f64,
    pub(crate) var_toxratioedge_dn8: f64,
    pub(crate) var_tratio: f64,
    pub(crate) var_tratio_dn4: f64,
    pub(crate) var_tratio_rv: f64,
    pub(crate) var_u02_i: f64,
    pub(crate) var_u02_i_rv: f64,
    pub(crate) var_u0_i: f64,
    pub(crate) var_u0_i_rv: f64,
    pub(crate) var_u0_t: f64,
    pub(crate) var_u0_t_dn4: f64,
    pub(crate) var_u0_t_rv: f64,
    pub(crate) var_ua1_i: f64,
    pub(crate) var_ua1_i_rv: f64,
    pub(crate) var_ua2_i: f64,
    pub(crate) var_ua2_i_rv: f64,
    pub(crate) var_ua_i: f64,
    pub(crate) var_ua_i_rv: f64,
    pub(crate) var_ua_t: f64,
    pub(crate) var_ua_t_dn4: f64,
    pub(crate) var_ua_t_rv: f64,
    pub(crate) var_uc2_i: f64,
    pub(crate) var_uc2_i_rv: f64,
    pub(crate) var_uc_i: f64,
    pub(crate) var_uc_i_rv: f64,
    pub(crate) var_uc_t: f64,
    pub(crate) var_uc_t_dn4: f64,
    pub(crate) var_uc_t_rv: f64,
    pub(crate) var_ucs2_i: f64,
    pub(crate) var_ucs2_i_rv: f64,
    pub(crate) var_ucs_i: f64,
    pub(crate) var_ucs_i_rv: f64,
    pub(crate) var_ucs_t: f64,
    pub(crate) var_ucs_t_dn4: f64,
    pub(crate) var_ucs_t_rv: f64,
    pub(crate) var_ucste_i: f64,
    pub(crate) var_ucste_i_rv: f64,
    pub(crate) var_ud1_i: f64,
    pub(crate) var_ud1_i_rv: f64,
    pub(crate) var_ud2_i: f64,
    pub(crate) var_ud2_i_rv: f64,
    pub(crate) var_ud_i: f64,
    pub(crate) var_ud_i_rv: f64,
    pub(crate) var_ud_t: f64,
    pub(crate) var_ud_t_dn4: f64,
    pub(crate) var_ud_t_rv: f64,
    pub(crate) var_udb2_i: f64,
    pub(crate) var_udb2_i_rv: f64,
    pub(crate) var_udb_i: f64,
    pub(crate) var_udb_i_rv: f64,
    pub(crate) var_ueff1: f64,
    pub(crate) var_ueff1_dn3: f64,
    pub(crate) var_ueff1_dn4: f64,
    pub(crate) var_ueff1_dn5: f64,
    pub(crate) var_ueff1_dn6: f64,
    pub(crate) var_ueff1_dn7: f64,
    pub(crate) var_ueff1_dn8: f64,
    pub(crate) var_ueff1_rv: f64,
    pub(crate) var_ueff2: f64,
    pub(crate) var_ueff2_dn3: f64,
    pub(crate) var_ueff2_dn4: f64,
    pub(crate) var_ueff2_dn5: f64,
    pub(crate) var_ueff2_dn6: f64,
    pub(crate) var_ueff2_dn7: f64,
    pub(crate) var_ueff2_dn8: f64,
    pub(crate) var_ueff2_rv: f64,
    pub(crate) var_up2_i: f64,
    pub(crate) var_up2_i_rv: f64,
    pub(crate) var_up_i: f64,
    pub(crate) var_up_i_rv: f64,
    pub(crate) var_ute_i: f64,
    pub(crate) var_ute_i_rv: f64,
    pub(crate) var_utl_i: f64,
    pub(crate) var_utl_i_rv: f64,
    pub(crate) var_utotal: f64,
    pub(crate) var_utotal_dn3: f64,
    pub(crate) var_utotal_dn4: f64,
    pub(crate) var_utotal_dn5: f64,
    pub(crate) var_utotal_dn6: f64,
    pub(crate) var_utotal_dn7: f64,
    pub(crate) var_utotal_dn8: f64,
    pub(crate) var_utotal_rv: f64,
    pub(crate) var_vadibl: f64,
    pub(crate) var_vadibl_dn3: f64,
    pub(crate) var_vadibl_dn4: f64,
    pub(crate) var_vadibl_dn5: f64,
    pub(crate) var_vadibl_dn6: f64,
    pub(crate) var_vadibl_dn7: f64,
    pub(crate) var_vadibl_dn8: f64,
    pub(crate) var_vadibl_rv: f64,
    pub(crate) var_vaux_igbacc: f64,
    pub(crate) var_vaux_igbacc_dn3: f64,
    pub(crate) var_vaux_igbacc_dn4: f64,
    pub(crate) var_vaux_igbacc_dn5: f64,
    pub(crate) var_vaux_igbacc_dn6: f64,
    pub(crate) var_vaux_igbacc_dn7: f64,
    pub(crate) var_vaux_igbacc_dn8: f64,
    pub(crate) var_vaux_igbinv: f64,
    pub(crate) var_vaux_igbinv_dn3: f64,
    pub(crate) var_vaux_igbinv_dn4: f64,
    pub(crate) var_vaux_igbinv_dn5: f64,
    pub(crate) var_vaux_igbinv_dn6: f64,
    pub(crate) var_vaux_igbinv_dn7: f64,
    pub(crate) var_vaux_igbinv_dn8: f64,
    pub(crate) var_vbegidl_i: f64,
    pub(crate) var_vbegidl_i_rv: f64,
    pub(crate) var_vbegisl_i: f64,
    pub(crate) var_vbegisl_i_rv: f64,
    pub(crate) var_vbgd: f64,
    pub(crate) var_vbgd_dn3: f64,
    pub(crate) var_vbgd_dn5: f64,
    pub(crate) var_vbgd_dn6: f64,
    pub(crate) var_vbgd_noswap: f64,
    pub(crate) var_vbgd_noswap_dn3: f64,
    pub(crate) var_vbgd_noswap_dn5: f64,
    pub(crate) var_vbgd_noswap_rv: f64,
    pub(crate) var_vbgd_rv: f64,
    pub(crate) var_vbgidl_i: f64,
    pub(crate) var_vbgidl_i_rv: f64,
    pub(crate) var_vbgisl_i: f64,
    pub(crate) var_vbgisl_i_rv: f64,
    pub(crate) var_vbgs: f64,
    pub(crate) var_vbgs_dn3: f64,
    pub(crate) var_vbgs_dn5: f64,
    pub(crate) var_vbgs_dn6: f64,
    pub(crate) var_vbgs_noswap: f64,
    pub(crate) var_vbgs_noswap_dn3: f64,
    pub(crate) var_vbgs_noswap_dn6: f64,
    pub(crate) var_vbgs_noswap_rv: f64,
    pub(crate) var_vbgs_rv: f64,
    pub(crate) var_vbgx: f64,
    pub(crate) var_vbgx_dn3: f64,
    pub(crate) var_vbgx_dn5: f64,
    pub(crate) var_vbgx_dn6: f64,
    pub(crate) var_vbgx_rv: f64,
    pub(crate) var_vbgxpos: f64,
    pub(crate) var_vbgxpos_dn3: f64,
    pub(crate) var_vbgxpos_dn5: f64,
    pub(crate) var_vbgxpos_dn6: f64,
    pub(crate) var_vbgxpos_rv: f64,
    pub(crate) var_vbi: f64,
    pub(crate) var_vbi_dn3: f64,
    pub(crate) var_vbi_dn4: f64,
    pub(crate) var_vbi_dn5: f64,
    pub(crate) var_vbi_dn6: f64,
    pub(crate) var_vbi_dn7: f64,
    pub(crate) var_vbi_dn8: f64,
    pub(crate) var_vbi_rv: f64,
    pub(crate) var_vds: f64,
    pub(crate) var_vds_dn5: f64,
    pub(crate) var_vds_dn6: f64,
    pub(crate) var_vds_noswap: f64,
    pub(crate) var_vds_noswap_dn5: f64,
    pub(crate) var_vds_noswap_dn6: f64,
    pub(crate) var_vds_noswap_rv: f64,
    pub(crate) var_vds_rv: f64,
    pub(crate) var_vdsat: f64,
    pub(crate) var_vdsat_dn3: f64,
    pub(crate) var_vdsat_dn4: f64,
    pub(crate) var_vdsat_dn5: f64,
    pub(crate) var_vdsat_dn6: f64,
    pub(crate) var_vdsat_dn7: f64,
    pub(crate) var_vdsat_dn8: f64,
    pub(crate) var_vdsat_rv: f64,
    pub(crate) var_vdseff: f64,
    pub(crate) var_vdseff_dn3: f64,
    pub(crate) var_vdseff_dn4: f64,
    pub(crate) var_vdseff_dn5: f64,
    pub(crate) var_vdseff_dn6: f64,
    pub(crate) var_vdseff_dn7: f64,
    pub(crate) var_vdseff_dn8: f64,
    pub(crate) var_vdseff_rv: f64,
    pub(crate) var_vdseffx: f64,
    pub(crate) var_vdseffx_dn3: f64,
    pub(crate) var_vdseffx_dn4: f64,
    pub(crate) var_vdseffx_dn5: f64,
    pub(crate) var_vdseffx_dn6: f64,
    pub(crate) var_vdseffx_dn7: f64,
    pub(crate) var_vdseffx_dn8: f64,
    pub(crate) var_vdseffx_rv: f64,
    pub(crate) var_vdsx: f64,
    pub(crate) var_vdsx_dn5: f64,
    pub(crate) var_vdsx_dn6: f64,
    pub(crate) var_vdsx_rv: f64,
    pub(crate) var_vfbsd: f64,
    pub(crate) var_vfbsd_bg: f64,
    pub(crate) var_vfbsd_bg_dn3: f64,
    pub(crate) var_vfbsd_bg_dn4: f64,
    pub(crate) var_vfbsd_bg_dn5: f64,
    pub(crate) var_vfbsd_bg_dn6: f64,
    pub(crate) var_vfbsd_bg_dn7: f64,
    pub(crate) var_vfbsd_bg_dn8: f64,
    pub(crate) var_vfbsd_bg_rv: f64,
    pub(crate) var_vfbsd_dn3: f64,
    pub(crate) var_vfbsd_dn4: f64,
    pub(crate) var_vfbsd_dn5: f64,
    pub(crate) var_vfbsd_dn6: f64,
    pub(crate) var_vfbsd_dn7: f64,
    pub(crate) var_vfbsd_dn8: f64,
    pub(crate) var_vfbsd_rv: f64,
    pub(crate) var_vfbzb: f64,
    pub(crate) var_vfbzb_dn3: f64,
    pub(crate) var_vfbzb_dn4: f64,
    pub(crate) var_vfbzb_dn5: f64,
    pub(crate) var_vfbzb_dn6: f64,
    pub(crate) var_vfbzb_dn7: f64,
    pub(crate) var_vfbzb_dn8: f64,
    pub(crate) var_vfbzb_rv: f64,
    pub(crate) var_vfgd_eff: f64,
    pub(crate) var_vfgd_eff_dn3: f64,
    pub(crate) var_vfgd_eff_dn4: f64,
    pub(crate) var_vfgd_eff_dn5: f64,
    pub(crate) var_vfgd_eff_dn6: f64,
    pub(crate) var_vfgd_eff_dn7: f64,
    pub(crate) var_vfgd_eff_dn8: f64,
    pub(crate) var_vfgd_eff_rv: f64,
    pub(crate) var_vfgd_ov: f64,
    pub(crate) var_vfgd_ov_dn3: f64,
    pub(crate) var_vfgd_ov_dn4: f64,
    pub(crate) var_vfgd_ov_dn5: f64,
    pub(crate) var_vfgd_ov_dn6: f64,
    pub(crate) var_vfgd_ov_dn7: f64,
    pub(crate) var_vfgd_ov_dn8: f64,
    pub(crate) var_vfgd_ov_rv: f64,
    pub(crate) var_vfgs: f64,
    pub(crate) var_vfgs_dn5: f64,
    pub(crate) var_vfgs_dn6: f64,
    pub(crate) var_vfgs_dn8: f64,
    pub(crate) var_vfgs_eff: f64,
    pub(crate) var_vfgs_eff_dn3: f64,
    pub(crate) var_vfgs_eff_dn4: f64,
    pub(crate) var_vfgs_eff_dn5: f64,
    pub(crate) var_vfgs_eff_dn6: f64,
    pub(crate) var_vfgs_eff_dn7: f64,
    pub(crate) var_vfgs_eff_dn8: f64,
    pub(crate) var_vfgs_eff_rv: f64,
    pub(crate) var_vfgs_ov: f64,
    pub(crate) var_vfgs_ov_dn3: f64,
    pub(crate) var_vfgs_ov_dn4: f64,
    pub(crate) var_vfgs_ov_dn5: f64,
    pub(crate) var_vfgs_ov_dn6: f64,
    pub(crate) var_vfgs_ov_dn7: f64,
    pub(crate) var_vfgs_ov_dn8: f64,
    pub(crate) var_vfgs_ov_rv: f64,
    pub(crate) var_vfgs_rv: f64,
    pub(crate) var_vgbg: f64,
    pub(crate) var_vgbg_dn3: f64,
    pub(crate) var_vgbg_dn8: f64,
    pub(crate) var_vgbg_rv: f64,
    pub(crate) var_vgd_eff: f64,
    pub(crate) var_vgd_eff_dn3: f64,
    pub(crate) var_vgd_eff_dn4: f64,
    pub(crate) var_vgd_eff_dn5: f64,
    pub(crate) var_vgd_eff_dn6: f64,
    pub(crate) var_vgd_eff_dn7: f64,
    pub(crate) var_vgd_eff_dn8: f64,
    pub(crate) var_vgd_eff_rv: f64,
    pub(crate) var_vgd_noswap: f64,
    pub(crate) var_vgd_noswap_dn5: f64,
    pub(crate) var_vgd_noswap_dn8: f64,
    pub(crate) var_vgd_noswap_rv: f64,
    pub(crate) var_vgd_ov_noswap: f64,
    pub(crate) var_vgd_ov_noswap_dn5: f64,
    pub(crate) var_vgd_ov_noswap_dn7: f64,
    pub(crate) var_vgd_ov_noswap_rv: f64,
    pub(crate) var_vgfb1: f64,
    pub(crate) var_vgfb1_dn4: f64,
    pub(crate) var_vgfb1_dn5: f64,
    pub(crate) var_vgfb1_dn6: f64,
    pub(crate) var_vgfb1_dn8: f64,
    pub(crate) var_vgfb1_rv: f64,
    pub(crate) var_vgfb1eff: f64,
    pub(crate) var_vgfb1eff_dn3: f64,
    pub(crate) var_vgfb1eff_dn4: f64,
    pub(crate) var_vgfb1eff_dn5: f64,
    pub(crate) var_vgfb1eff_dn6: f64,
    pub(crate) var_vgfb1eff_dn7: f64,
    pub(crate) var_vgfb1eff_dn8: f64,
    pub(crate) var_vgfb1eff_rv: f64,
    pub(crate) var_vgfb2: f64,
    pub(crate) var_vgfb2_dn3: f64,
    pub(crate) var_vgfb2_dn4: f64,
    pub(crate) var_vgfb2_dn5: f64,
    pub(crate) var_vgfb2_dn6: f64,
    pub(crate) var_vgfb2_dn7: f64,
    pub(crate) var_vgfb2_dn8: f64,
    pub(crate) var_vgfb2_rv: f64,
    pub(crate) var_vgfb2eff: f64,
    pub(crate) var_vgfb2eff_dn5: f64,
    pub(crate) var_vgfb2eff_dn6: f64,
    pub(crate) var_vgfb2eff_rv: f64,
    pub(crate) var_vgs_eff: f64,
    pub(crate) var_vgs_eff_dn3: f64,
    pub(crate) var_vgs_eff_dn4: f64,
    pub(crate) var_vgs_eff_dn5: f64,
    pub(crate) var_vgs_eff_dn6: f64,
    pub(crate) var_vgs_eff_dn7: f64,
    pub(crate) var_vgs_eff_dn8: f64,
    pub(crate) var_vgs_eff_rv: f64,
    pub(crate) var_vgs_noswap: f64,
    pub(crate) var_vgs_noswap_dn6: f64,
    pub(crate) var_vgs_noswap_dn8: f64,
    pub(crate) var_vgs_noswap_rv: f64,
    pub(crate) var_vgs_ov_noswap: f64,
    pub(crate) var_vgs_ov_noswap_dn6: f64,
    pub(crate) var_vgs_ov_noswap_dn7: f64,
    pub(crate) var_vgs_ov_noswap_rv: f64,
    pub(crate) var_vgst2vtm: f64,
    pub(crate) var_vgst2vtm_dn3: f64,
    pub(crate) var_vgst2vtm_dn4: f64,
    pub(crate) var_vgst2vtm_dn5: f64,
    pub(crate) var_vgst2vtm_dn6: f64,
    pub(crate) var_vgst2vtm_dn7: f64,
    pub(crate) var_vgst2vtm_dn8: f64,
    pub(crate) var_vgst2vtm_rv: f64,
    pub(crate) var_vknee1: f64,
    pub(crate) var_vknee1_rv: f64,
    pub(crate) var_vknee1nw_i: f64,
    pub(crate) var_vknee1nw_i_rv: f64,
    pub(crate) var_vknee1pw_i: f64,
    pub(crate) var_vknee1pw_i_rv: f64,
    pub(crate) var_vknee2: f64,
    pub(crate) var_vknee2_rv: f64,
    pub(crate) var_vknee2nw_i: f64,
    pub(crate) var_vknee2nw_i_rv: f64,
    pub(crate) var_vknee2pw_i: f64,
    pub(crate) var_vknee2pw_i_rv: f64,
    pub(crate) var_voxacc: f64,
    pub(crate) var_voxacc_dn3: f64,
    pub(crate) var_voxacc_dn4: f64,
    pub(crate) var_voxacc_dn5: f64,
    pub(crate) var_voxacc_dn6: f64,
    pub(crate) var_voxacc_dn7: f64,
    pub(crate) var_voxacc_dn8: f64,
    pub(crate) var_voxacc_rv: f64,
    pub(crate) var_vsat1_i: f64,
    pub(crate) var_vsat1_i_rv: f64,
    pub(crate) var_vsat1_t: f64,
    pub(crate) var_vsat1_t_dn4: f64,
    pub(crate) var_vsat1_t_rv: f64,
    pub(crate) var_vsat_i: f64,
    pub(crate) var_vsat_i_rv: f64,
    pub(crate) var_vsat_t: f64,
    pub(crate) var_vsat_t_dn4: f64,
    pub(crate) var_vsat_t_rv: f64,
    pub(crate) var_vsatb_i: f64,
    pub(crate) var_vsatb_i_rv: f64,
    pub(crate) var_vsatb_t: f64,
    pub(crate) var_vsatb_t_dn4: f64,
    pub(crate) var_vsatb_t_rv: f64,
    pub(crate) var_vsatcv_i: f64,
    pub(crate) var_vsatcv_i_rv: f64,
    pub(crate) var_vsatcv_t: f64,
    pub(crate) var_vsatcv_t_dn4: f64,
    pub(crate) var_vsatcv_t_rv: f64,
    pub(crate) var_vsubdep: f64,
    pub(crate) var_vsubdep0: f64,
    pub(crate) var_vsubdep0_rv: f64,
    pub(crate) var_vsubdep_dn3: f64,
    pub(crate) var_vsubdep_dn4: f64,
    pub(crate) var_vsubdep_dn5: f64,
    pub(crate) var_vsubdep_dn6: f64,
    pub(crate) var_vsubdep_dn7: f64,
    pub(crate) var_vsubdep_dn8: f64,
    pub(crate) var_vsubdep_rv: f64,
    pub(crate) var_vtm: f64,
    pub(crate) var_vtm_dn4: f64,
    pub(crate) var_vtm_rv: f64,
    pub(crate) var_w1: f64,
    pub(crate) var_w1_dn3: f64,
    pub(crate) var_w1_dn4: f64,
    pub(crate) var_w1_dn5: f64,
    pub(crate) var_w1_dn6: f64,
    pub(crate) var_w1_dn7: f64,
    pub(crate) var_w1_dn8: f64,
    pub(crate) var_w1_rv: f64,
    pub(crate) var_w2: f64,
    pub(crate) var_w2_dn3: f64,
    pub(crate) var_w2_dn4: f64,
    pub(crate) var_w2_dn5: f64,
    pub(crate) var_w2_dn6: f64,
    pub(crate) var_w2_dn7: f64,
    pub(crate) var_w2_dn8: f64,
    pub(crate) var_w2_rv: f64,
    pub(crate) var_w_lwn: f64,
    pub(crate) var_w_lwn_rv: f64,
    pub(crate) var_w_wwn: f64,
    pub(crate) var_w_wwn_rv: f64,
    pub(crate) var_weff: f64,
    pub(crate) var_weff_rv: f64,
    pub(crate) var_weffcv: f64,
    pub(crate) var_weffcv_rv: f64,
    pub(crate) var_weffwrfactor: f64,
    pub(crate) var_weffwrfactor_rv: f64,
    pub(crate) var_welsign: f64,
    pub(crate) var_welsign_rv: f64,
    pub(crate) var_wf: f64,
    pub(crate) var_wf_dn3: f64,
    pub(crate) var_wf_dn4: f64,
    pub(crate) var_wf_dn5: f64,
    pub(crate) var_wf_dn6: f64,
    pub(crate) var_wf_dn7: f64,
    pub(crate) var_wf_dn8: f64,
    pub(crate) var_wnew: f64,
    pub(crate) var_wnew_rv: f64,
    pub(crate) var_wr: f64,
    pub(crate) var_wr_dn3: f64,
    pub(crate) var_wr_dn4: f64,
    pub(crate) var_wr_dn5: f64,
    pub(crate) var_wr_dn6: f64,
    pub(crate) var_wr_dn7: f64,
    pub(crate) var_wr_dn8: f64,
    pub(crate) var_wr_i: f64,
    pub(crate) var_wr_i_rv: f64,
    pub(crate) var_wvcox: f64,
    pub(crate) var_wvcox_dn4: f64,
    pub(crate) var_wvcox_rv: f64,
    pub(crate) var_xg1: f64,
    pub(crate) var_xg1_dn3: f64,
    pub(crate) var_xg1_dn4: f64,
    pub(crate) var_xg1_dn5: f64,
    pub(crate) var_xg1_dn6: f64,
    pub(crate) var_xg1_dn7: f64,
    pub(crate) var_xg1_dn8: f64,
    pub(crate) var_xg1_rv: f64,
    pub(crate) var_xg2: f64,
    pub(crate) var_xg2_dn3: f64,
    pub(crate) var_xg2_dn4: f64,
    pub(crate) var_xg2_dn5: f64,
    pub(crate) var_xg2_dn6: f64,
    pub(crate) var_xg2_dn7: f64,
    pub(crate) var_xg2_dn8: f64,
    pub(crate) var_xg2_rv: f64,
    pub(crate) var_xrcrg1_i: f64,
    pub(crate) var_xrcrg1_i_rv: f64,
    pub(crate) var_xrcrg2_i: f64,
    pub(crate) var_xsat: f64,
    pub(crate) var_xsat_dn3: f64,
    pub(crate) var_xsat_dn4: f64,
    pub(crate) var_xsat_dn5: f64,
    pub(crate) var_xsat_dn6: f64,
    pub(crate) var_xsat_dn7: f64,
    pub(crate) var_xsat_dn8: f64,
    pub(crate) var_xsat_rv: f64,
}

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_older = self.ddt_state_older.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_derivative_current = self.ddt_derivative_current.as_mut();
        let ddt_derivative_previous = self.ddt_derivative_previous.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_scale = self.ddt_coefficients.derivative_scale;
        let ddt_previous_value_scale = self.ddt_coefficients.previous_value_scale;
        let ddt_older_value_scale = self.ddt_coefficients.older_value_scale;
        let ddt_previous_derivative_scale = self.ddt_coefficients.previous_derivative_scale;
        let v0: f64 = 0.0;
        let v1: f64 = 1.0;
        let v58: f64 = 2.0;
        let v171: f64 = 1e-38;
        let v172: f64 = 0.5;
        let v185: f64 = 300.15;
        let v206: f64 = ctx_temp;
        let v207: f64 = nv4;
        let v208: f64 = (v206 + v207);
        let v210: f64 = (v208 + self.scalar_v209);
        let v211: f64 = (if self.scalar_v176 { v210 } else { v0 });
        let v213: f64 = (if self.scalar_v182 { self.scalar_v212 } else { v211 });
        let v216: f64 = (v213 + self.scalar_v215);
        let v217: f64 = (v213 - self.scalar_v215);
        let v218: f64 = (v217 * v217);
        let v219: f64 = 2.5e-5;
        let v220: f64 = (v218 + v219);
        let v221: f64 = ((v220) as f64).sqrt();
        let v222: f64 = (v216 - v221);
        let v223: f64 = (v172 * v222);
        let v224: f64 = 8.61708e-5;
        let v225: f64 = (v223 * v224);
        let v228: f64 = (v223 * self.scalar_v227);
        let v229: f64 = (v223 * v228);
        let v231: f64 = (v223 + self.scalar_v230);
        let v232: f64 = (v229 / v231);
        let v233: f64 = (self.scalar_v226 - v232);
        let v234: f64 = (v223 / v185);
        let v235: f64 = ((v234) as f64).sqrt();
        let v236: f64 = (v234 * v235);
        let v238: f64 = (v236 * self.scalar_v237);
        let v241: f64 = (v58 * v225);
        let v242: f64 = (v233 / v241);
        let v243: f64 = (self.scalar_v240 - v242);
        let v244: f64 = { let limited_exp_arg = v243; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let v245: f64 = (v238 * v244);
        let v246: f64 = (v245 * v245);
        let v247: f64 = (v172 * v233);
        let v249: f64 = (self.scalar_v248 / v245);
        let v250: bool = (v249 > v171);
        let v251: f64 = (if v250 { v249 } else { v171 });
        let v252: f64 = ((v251) as f64).ln();
        let v253: f64 = (v225 * v252);
        let v254: f64 = (v247 - v253);
        let v255: f64 = (v254 * v254);
        let v256: f64 = 4.0;
        let v257: f64 = 4e-8;
        let v258: f64 = (v255 + v257);
        let v259: f64 = ((v258) as f64).sqrt();
        let v260: f64 = (v254 + v259);
        let v261: f64 = (v172 * v260);
        let v262: f64 = (v247 - v261);
        let v271: f64 = (v262 + self.scalar_v270);
        let v272: f64 = (if self.scalar_v268 { v271 } else { self.scalar_v98 });
        let v275: f64 = (self.scalar_v269 + v272);
        let v276: f64 = (v275 - v262);
        let v277: f64 = (if self.scalar_v274 { v276 } else { v272 });
        let v279: f64 = (v233 / v58);
        let v280: f64 = (self.scalar_v278 + v279);
        let v281: f64 = (self.scalar_v108 / v245);
        let v282: bool = (v281 > v171);
        let v283: f64 = (if v282 { v281 } else { v171 });
        let v284: f64 = ((v283) as f64).ln();
        let v285: f64 = (v225 * v284);
        let v286: bool = (v279 < v285);
        let v287: f64 = (if v286 { v279 } else { v285 });
        let v288: f64 = (self.scalar_v9 * v287);
        let v289: f64 = (v280 - v288);
        let v290: f64 = (self.scalar_v88 - v289);
        let v291: f64 = (self.scalar_v9 * v290);
        let v292: f64 = (v277 - v289);
        let v293: f64 = (self.scalar_v9 * v292);
        let v294: f64 = nv6;
        let v295: f64 = nv5;
        let v296: f64 = nv3;
        let v297: f64 = (v296 - v294);
        let v298: f64 = (self.scalar_v9 * v297);
        let v299: f64 = (v296 - v295);
        let v300: f64 = (self.scalar_v9 * v299);
        let v301: f64 = nv7;
        let v302: f64 = (v301 - v295);
        let v303: f64 = (self.scalar_v9 * v302);
        let v304: f64 = (v301 - v294);
        let v305: f64 = (self.scalar_v9 * v304);
        let v306: f64 = 0.02;
        let v311: f64 = (v304 * self.scalar_v310);
        let v314: f64 = (v302 * self.scalar_v313);
        let v315: f64 = (v305 - v291);
        let v316: f64 = (v306 + v315);
        let v318: f64 = (v298 - v293);
        let v320: f64 = (v318 - self.scalar_v319);
        let v321: f64 = (self.scalar_v317 * v320);
        let v323: f64 = (v321 * self.scalar_v322);
        let v324: f64 = (v316 + v323);
        let v325: f64 = (v324 * v324);
        let v326: f64 = 0.08;
        let v327: f64 = (v325 + v326);
        let v328: f64 = ((v327) as f64).sqrt();
        let v329: f64 = (v324 - v328);
        let v330: f64 = (v172 * v329);
        let v331: f64 = (v315 - v330);
        let v337: f64 = (v256 * v330);
        let v338: f64 = (v337 / self.scalar_v335);
        let v339: f64 = (v1 - v338);
        let v340: f64 = ((v339) as f64).sqrt();
        let v341: f64 = (v340 - v1);
        let v342: f64 = (self.scalar_v336 * v341);
        let v343: f64 = (v331 - v342);
        let v344: f64 = (self.scalar_v334 * v343);
        let v345: f64 = (v311 + v344);
        let v346: f64 = (v303 - v291);
        let v347: f64 = (v306 + v346);
        let v348: f64 = (v300 - v293);
        let v350: f64 = (v348 - self.scalar_v349);
        let v351: f64 = (self.scalar_v317 * v350);
        let v353: f64 = (v351 * self.scalar_v352);
        let v354: f64 = (v347 + v353);
        let v355: f64 = (v354 * v354);
        let v356: f64 = (v326 + v355);
        let v357: f64 = ((v356) as f64).sqrt();
        let v358: f64 = (v354 - v357);
        let v359: f64 = (v172 * v358);
        let v360: f64 = (v346 - v359);
        let v365: f64 = (v256 * v359);
        let v366: f64 = (v365 / self.scalar_v363);
        let v367: f64 = (v1 - v366);
        let v368: f64 = ((v367) as f64).sqrt();
        let v369: f64 = (v368 - v1);
        let v370: f64 = (self.scalar_v364 * v369);
        let v371: f64 = (v360 - v370);
        let v372: f64 = (self.scalar_v362 * v371);
        let v373: f64 = (v314 + v372);
        let v375: f64 = (v304 * self.scalar_v374);
        let v377: f64 = (v302 * self.scalar_v376);
        let v378: f64 = (v345 + v375);
        let v379: f64 = (v373 + v377);
        let v380: f64 = (self.scalar_v14 * v378);
        let v381: f64 = (self.scalar_v14 * v379);
        let v388: f64 = nv1;
        let v392: f64 = (v388 - v301);
        let v393: f64 = (self.scalar_v387 * v392);
        let v394: f64 = (if self.scalar_v386 { v393 } else { v0 });
        let v395: f64 = (self.scalar_v183 * v207);
        let v396: f64 = (if self.scalar_v176 { v395 } else { v0 });
        let v400: f64 = (v217 * self.scalar_v399);
        let v401: f64 = (v400 + v400);
        let v402: f64 = (v58 * v221);
        let v403: f64 = (v401 / v402);
        let v404: f64 = (self.scalar_v399 - v403);
        let v405: f64 = (v172 * v404);
        let v406: f64 = (v224 * v405);
        let v407: f64 = (self.scalar_v227 * v405);
        let v408: f64 = (v228 * v405);
        let v409: f64 = (v223 * v407);
        let v410: f64 = (v408 + v409);
        let v411: f64 = (v231 * v410);
        let v412: f64 = (v229 * v405);
        let v413: f64 = (v411 - v412);
        let v414: f64 = (v231 * v231);
        let v415: f64 = (v413 / v414);
        let v416: f64 = (-v415);
        let v417: f64 = (v405 / v185);
        let v418: f64 = (v58 * v235);
        let v419: f64 = (v417 / v418);
        let v420: f64 = (v235 * v417);
        let v421: f64 = (v234 * v419);
        let v422: f64 = (v420 + v421);
        let v423: f64 = (self.scalar_v237 * v422);
        let v424: f64 = (v58 * v406);
        let v425: f64 = (v241 * v416);
        let v426: f64 = (v233 * v424);
        let v427: f64 = (v425 - v426);
        let v428: f64 = (v241 * v241);
        let v429: f64 = (v427 / v428);
        let v430: f64 = (-v429);
        let v431: f64 = { let limited_exp_arg = v243; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } };
        let v432: f64 = (v430 * v431);
        let v433: f64 = (v244 * v423);
        let v434: f64 = (v238 * v432);
        let v435: f64 = (v433 + v434);
        let v436: f64 = (v172 * v416);
        let v437: f64 = (self.scalar_v248 * v435);
        let v438: f64 = (-v437);
        let v439: f64 = (v438 / v246);
        let v440: f64 = (if v250 { v439 } else { v0 });
        let v441: f64 = (v440 / v251);
        let v442: f64 = (v252 * v406);
        let v443: f64 = (v225 * v441);
        let v444: f64 = (v442 + v443);
        let v445: f64 = (v436 - v444);
        let v446: f64 = (v254 * v445);
        let v447: f64 = (v446 + v446);
        let v448: f64 = (v58 * v259);
        let v449: f64 = (v447 / v448);
        let v450: f64 = (v445 + v449);
        let v451: f64 = (v172 * v450);
        let v452: f64 = (v436 - v451);
        let v453: f64 = (if self.scalar_v268 { v452 } else { v0 });
        let v454: f64 = (v453 - v452);
        let v455: f64 = (if self.scalar_v274 { v454 } else { v453 });
        let v456: f64 = (v416 / v58);
        let v457: f64 = (self.scalar_v108 * v435);
        let v458: f64 = (-v457);
        let v459: f64 = (v458 / v246);
        let v460: f64 = (if v282 { v459 } else { v0 });
        let v461: f64 = (v460 / v283);
        let v462: f64 = (v284 * v406);
        let v463: f64 = (v225 * v461);
        let v464: f64 = (v462 + v463);
        let v465: f64 = (if v286 { v456 } else { v464 });
        let v466: f64 = (self.scalar_v9 * v465);
        let v467: f64 = (v456 - v466);
        let v468: f64 = (-v467);
        let v469: f64 = (self.scalar_v9 * v468);
        let v470: f64 = (v455 - v467);
        let v471: f64 = (self.scalar_v9 * v470);
        let v472: f64 = (-v469);
        let v475: f64 = (-v471);
        let v477: f64 = (self.scalar_v317 * v475);
        let v480: f64 = (self.scalar_v322 * v477);
        let v482: f64 = (v472 + v480);
        let v484: f64 = (v324 * self.scalar_v479);
        let v485: f64 = (v484 + v484);
        let v486: f64 = (v324 * v482);
        let v487: f64 = (v486 + v486);
        let v488: f64 = (v324 * self.scalar_v483);
        let v489: f64 = (v488 + v488);
        let v490: f64 = (self.scalar_v9 * v324);
        let v491: f64 = (v490 + v490);
        let v492: f64 = (v58 * v328);
        let v493: f64 = (v485 / v492);
        let v494: f64 = (v487 / v492);
        let v495: f64 = (v489 / v492);
        let v496: f64 = (v491 / v492);
        let v497: f64 = (self.scalar_v479 - v493);
        let v498: f64 = (v482 - v494);
        let v499: f64 = (self.scalar_v483 - v495);
        let v500: f64 = (self.scalar_v9 - v496);
        let v501: f64 = (v172 * v497);
        let v502: f64 = (v172 * v498);
        let v503: f64 = (v172 * v499);
        let v504: f64 = (v172 * v500);
        let v505: f64 = (-v501);
        let v506: f64 = (v472 - v502);
        let v507: f64 = (self.scalar_v307 - v503);
        let v508: f64 = (self.scalar_v9 - v504);
        let v509: f64 = (v256 * v501);
        let v510: f64 = (v256 * v502);
        let v511: f64 = (v256 * v503);
        let v512: f64 = (v256 * v504);
        let v513: f64 = (v509 / self.scalar_v335);
        let v514: f64 = (v510 / self.scalar_v335);
        let v515: f64 = (v511 / self.scalar_v335);
        let v516: f64 = (v512 / self.scalar_v335);
        let v517: f64 = (-v513);
        let v518: f64 = (-v514);
        let v519: f64 = (-v515);
        let v520: f64 = (-v516);
        let v521: f64 = (v58 * v340);
        let v522: f64 = (v517 / v521);
        let v523: f64 = (v518 / v521);
        let v524: f64 = (v519 / v521);
        let v525: f64 = (v520 / v521);
        let v526: f64 = (self.scalar_v336 * v522);
        let v527: f64 = (self.scalar_v336 * v523);
        let v528: f64 = (self.scalar_v336 * v524);
        let v529: f64 = (self.scalar_v336 * v525);
        let v530: f64 = (v505 - v526);
        let v531: f64 = (v506 - v527);
        let v532: f64 = (v507 - v528);
        let v533: f64 = (v508 - v529);
        let v534: f64 = (self.scalar_v334 * v530);
        let v535: f64 = (self.scalar_v334 * v531);
        let v536: f64 = (self.scalar_v334 * v532);
        let v537: f64 = (self.scalar_v334 * v533);
        let v538: f64 = (self.scalar_v473 + v536);
        let v539: f64 = (self.scalar_v310 + v537);
        let v541: f64 = (self.scalar_v352 * v477);
        let v543: f64 = (v472 + v541);
        let v545: f64 = (v354 * self.scalar_v540);
        let v546: f64 = (v545 + v545);
        let v547: f64 = (v354 * v543);
        let v548: f64 = (v547 + v547);
        let v549: f64 = (v354 * self.scalar_v544);
        let v550: f64 = (v549 + v549);
        let v551: f64 = (self.scalar_v9 * v354);
        let v552: f64 = (v551 + v551);
        let v553: f64 = (v58 * v357);
        let v554: f64 = (v546 / v553);
        let v555: f64 = (v548 / v553);
        let v556: f64 = (v550 / v553);
        let v557: f64 = (v552 / v553);
        let v558: f64 = (self.scalar_v540 - v554);
        let v559: f64 = (v543 - v555);
        let v560: f64 = (self.scalar_v544 - v556);
        let v561: f64 = (self.scalar_v9 - v557);
        let v562: f64 = (v172 * v558);
        let v563: f64 = (v172 * v559);
        let v564: f64 = (v172 * v560);
        let v565: f64 = (v172 * v561);
        let v566: f64 = (-v562);
        let v567: f64 = (v472 - v563);
        let v568: f64 = (self.scalar_v307 - v564);
        let v569: f64 = (self.scalar_v9 - v565);
        let v570: f64 = (v256 * v562);
        let v571: f64 = (v256 * v563);
        let v572: f64 = (v256 * v564);
        let v573: f64 = (v256 * v565);
        let v574: f64 = (v570 / self.scalar_v363);
        let v575: f64 = (v571 / self.scalar_v363);
        let v576: f64 = (v572 / self.scalar_v363);
        let v577: f64 = (v573 / self.scalar_v363);
        let v578: f64 = (-v574);
        let v579: f64 = (-v575);
        let v580: f64 = (-v576);
        let v581: f64 = (-v577);
        let v582: f64 = (v58 * v368);
        let v583: f64 = (v578 / v582);
        let v584: f64 = (v579 / v582);
        let v585: f64 = (v580 / v582);
        let v586: f64 = (v581 / v582);
        let v587: f64 = (self.scalar_v364 * v583);
        let v588: f64 = (self.scalar_v364 * v584);
        let v589: f64 = (self.scalar_v364 * v585);
        let v590: f64 = (self.scalar_v364 * v586);
        let v591: f64 = (v566 - v587);
        let v592: f64 = (v567 - v588);
        let v593: f64 = (v568 - v589);
        let v594: f64 = (v569 - v590);
        let v595: f64 = (self.scalar_v362 * v591);
        let v596: f64 = (self.scalar_v362 * v592);
        let v597: f64 = (self.scalar_v362 * v593);
        let v598: f64 = (self.scalar_v362 * v594);
        let v599: f64 = (self.scalar_v474 + v597);
        let v600: f64 = (self.scalar_v313 + v598);
        let v603: f64 = (v538 + self.scalar_v601);
        let v604: f64 = (self.scalar_v374 + v539);
        let v605: f64 = (v599 + self.scalar_v602);
        let v606: f64 = (self.scalar_v376 + v600);
        let v607: f64 = (self.scalar_v14 * v534);
        let v608: f64 = (self.scalar_v14 * v535);
        let v609: f64 = (self.scalar_v14 * v603);
        let v610: f64 = (self.scalar_v14 * v604);
        let v611: f64 = (self.scalar_v14 * v595);
        let v612: f64 = (self.scalar_v14 * v596);
        let v613: f64 = (self.scalar_v14 * v605);
        let v614: f64 = (self.scalar_v14 * v606);

        stamper.stamp_potential_branch_local(
            Some(0),
            Some(5),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v389,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(6),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v389,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v390,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v391,
        );
        let d394_dn1: f64 = self.scalar_v616;
        let d394_dn7: f64 = self.scalar_v617;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(7),
            multiplicity * (v394),
            1,
            multiplicity * (d394_dn1),
            7,
            multiplicity * (d394_dn7),
        );
        let d396_dn4: f64 = self.scalar_v618;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v396),
            4,
            multiplicity * (d396_dn4),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v397,
        );
        let d380_dn3: f64 = v607;
        let d380_dn4: f64 = v608;
        let d380_dn6: f64 = v609;
        let d380_dn7: f64 = v610;
        let v380_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v380);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(6),
            multiplicity * (v380_ddt),
            [3, 4, 6, 7],
            [((d380_dn3) * ddt_scale), ((d380_dn4) * ddt_scale), ((d380_dn6) * ddt_scale), ((d380_dn7) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d381_dn3: f64 = v611;
        let d381_dn4: f64 = v612;
        let d381_dn5: f64 = v613;
        let d381_dn7: f64 = v614;
        let v381_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v381);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(5),
            multiplicity * (v381_ddt),
            [3, 4, 5, 7],
            [((d381_dn3) * ddt_scale), ((d381_dn4) * ddt_scale), ((d381_dn5) * ddt_scale), ((d381_dn7) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let mut locals = StampLocals::default();

        Self::stamp_transient_block_0(p, &mut locals);
        Self::stamp_transient_block_1(p, &mut locals);
        Self::stamp_transient_block_2(p, param_given, &mut locals);
        Self::stamp_transient_block_3(p, &mut locals);
        Self::stamp_transient_block_4(ctx, p, nodes, param_given, &mut locals);
        Self::stamp_transient_block_5(ctx, p, nodes, &mut locals);
        Self::stamp_transient_block_6(ctx, p, nodes, &mut locals);
        Self::stamp_transient_block_7(p, &mut locals);
        Self::stamp_transient_block_8(p, &mut locals);
        Self::stamp_transient_block_9(&mut locals);
        Self::stamp_transient_block_10(&mut locals);
        Self::stamp_transient_block_11(&mut locals);
        Self::stamp_transient_block_12(&mut locals);
        Self::stamp_transient_block_13(&mut locals);
        Self::stamp_transient_block_14(p, &mut locals);
        Self::stamp_transient_block_15(p, &mut locals);
        Self::stamp_transient_block_16(&mut locals);
        Self::stamp_transient_block_17(&mut locals);
        Self::stamp_transient_block_18(&mut locals);
        Self::stamp_transient_block_19(&mut locals);
        Self::stamp_transient_block_20(&mut locals);
        Self::stamp_transient_block_21(p, &mut locals);
        Self::stamp_transient_block_22(p, &mut locals);
        Self::stamp_transient_block_23(p, &mut locals);
        Self::stamp_transient_block_24(p, &mut locals);
        Self::stamp_transient_block_25(ctx, p, nodes, &mut locals);
        Self::stamp_transient_block_26(p, &mut locals);
        Self::stamp_transient_block_27(p, &mut locals);
        Self::stamp_transient_block_28(p, &mut locals);
        Self::stamp_transient_block_29(p, &mut locals);
        Self::stamp_transient_block_30(p, &mut locals);

        Self::stamp_transient_equations_block_0(ctx, stamper, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, &mut locals);
        Self::stamp_transient_equations_block_1(ctx, stamper, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, &mut locals);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let v0: f64 = 0.0;
        let v1: f64 = 1.0;
        let v58: f64 = 2.0;
        let v171: f64 = 1e-38;
        let v172: f64 = 0.5;
        let v185: f64 = 300.15;
        let v206: f64 = ctx_temp;
        let v207: f64 = nv4;
        let v208: f64 = (v206 + v207);
        let v210: f64 = (v208 + self.scalar_v209);
        let v211: f64 = (if self.scalar_v176 { v210 } else { v0 });
        let v213: f64 = (if self.scalar_v182 { self.scalar_v212 } else { v211 });
        let v216: f64 = (v213 + self.scalar_v215);
        let v217: f64 = (v213 - self.scalar_v215);
        let v218: f64 = (v217 * v217);
        let v219: f64 = 2.5e-5;
        let v220: f64 = (v218 + v219);
        let v221: f64 = ((v220) as f64).sqrt();
        let v222: f64 = (v216 - v221);
        let v223: f64 = (v172 * v222);
        let v224: f64 = 8.61708e-5;
        let v225: f64 = (v223 * v224);
        let v228: f64 = (v223 * self.scalar_v227);
        let v229: f64 = (v223 * v228);
        let v231: f64 = (v223 + self.scalar_v230);
        let v232: f64 = (v229 / v231);
        let v233: f64 = (self.scalar_v226 - v232);
        let v234: f64 = (v223 / v185);
        let v235: f64 = ((v234) as f64).sqrt();
        let v236: f64 = (v234 * v235);
        let v238: f64 = (v236 * self.scalar_v237);
        let v241: f64 = (v58 * v225);
        let v242: f64 = (v233 / v241);
        let v243: f64 = (self.scalar_v240 - v242);
        let v244: f64 = { let limited_exp_arg = v243; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let v245: f64 = (v238 * v244);
        let v246: f64 = (v245 * v245);
        let v247: f64 = (v172 * v233);
        let v249: f64 = (self.scalar_v248 / v245);
        let v250: bool = (v249 > v171);
        let v251: f64 = (if v250 { v249 } else { v171 });
        let v252: f64 = ((v251) as f64).ln();
        let v253: f64 = (v225 * v252);
        let v254: f64 = (v247 - v253);
        let v255: f64 = (v254 * v254);
        let v256: f64 = 4.0;
        let v257: f64 = 4e-8;
        let v258: f64 = (v255 + v257);
        let v259: f64 = ((v258) as f64).sqrt();
        let v260: f64 = (v254 + v259);
        let v261: f64 = (v172 * v260);
        let v262: f64 = (v247 - v261);
        let v271: f64 = (v262 + self.scalar_v270);
        let v272: f64 = (if self.scalar_v268 { v271 } else { self.scalar_v98 });
        let v275: f64 = (self.scalar_v269 + v272);
        let v276: f64 = (v275 - v262);
        let v277: f64 = (if self.scalar_v274 { v276 } else { v272 });
        let v279: f64 = (v233 / v58);
        let v280: f64 = (self.scalar_v278 + v279);
        let v281: f64 = (self.scalar_v108 / v245);
        let v282: bool = (v281 > v171);
        let v283: f64 = (if v282 { v281 } else { v171 });
        let v284: f64 = ((v283) as f64).ln();
        let v285: f64 = (v225 * v284);
        let v286: bool = (v279 < v285);
        let v287: f64 = (if v286 { v279 } else { v285 });
        let v288: f64 = (self.scalar_v9 * v287);
        let v289: f64 = (v280 - v288);
        let v290: f64 = (self.scalar_v88 - v289);
        let v291: f64 = (self.scalar_v9 * v290);
        let v292: f64 = (v277 - v289);
        let v293: f64 = (self.scalar_v9 * v292);
        let v294: f64 = nv6;
        let v295: f64 = nv5;
        let v296: f64 = nv3;
        let v297: f64 = (v296 - v294);
        let v298: f64 = (self.scalar_v9 * v297);
        let v299: f64 = (v296 - v295);
        let v300: f64 = (self.scalar_v9 * v299);
        let v301: f64 = nv7;
        let v302: f64 = (v301 - v295);
        let v303: f64 = (self.scalar_v9 * v302);
        let v304: f64 = (v301 - v294);
        let v305: f64 = (self.scalar_v9 * v304);
        let v306: f64 = 0.02;
        let v311: f64 = (v304 * self.scalar_v310);
        let v314: f64 = (v302 * self.scalar_v313);
        let v315: f64 = (v305 - v291);
        let v316: f64 = (v306 + v315);
        let v318: f64 = (v298 - v293);
        let v320: f64 = (v318 - self.scalar_v319);
        let v321: f64 = (self.scalar_v317 * v320);
        let v323: f64 = (v321 * self.scalar_v322);
        let v324: f64 = (v316 + v323);
        let v325: f64 = (v324 * v324);
        let v326: f64 = 0.08;
        let v327: f64 = (v325 + v326);
        let v328: f64 = ((v327) as f64).sqrt();
        let v329: f64 = (v324 - v328);
        let v330: f64 = (v172 * v329);
        let v331: f64 = (v315 - v330);
        let v337: f64 = (v256 * v330);
        let v338: f64 = (v337 / self.scalar_v335);
        let v339: f64 = (v1 - v338);
        let v340: f64 = ((v339) as f64).sqrt();
        let v341: f64 = (v340 - v1);
        let v342: f64 = (self.scalar_v336 * v341);
        let v343: f64 = (v331 - v342);
        let v344: f64 = (self.scalar_v334 * v343);
        let v345: f64 = (v311 + v344);
        let v346: f64 = (v303 - v291);
        let v347: f64 = (v306 + v346);
        let v348: f64 = (v300 - v293);
        let v350: f64 = (v348 - self.scalar_v349);
        let v351: f64 = (self.scalar_v317 * v350);
        let v353: f64 = (v351 * self.scalar_v352);
        let v354: f64 = (v347 + v353);
        let v355: f64 = (v354 * v354);
        let v356: f64 = (v326 + v355);
        let v357: f64 = ((v356) as f64).sqrt();
        let v358: f64 = (v354 - v357);
        let v359: f64 = (v172 * v358);
        let v360: f64 = (v346 - v359);
        let v365: f64 = (v256 * v359);
        let v366: f64 = (v365 / self.scalar_v363);
        let v367: f64 = (v1 - v366);
        let v368: f64 = ((v367) as f64).sqrt();
        let v369: f64 = (v368 - v1);
        let v370: f64 = (self.scalar_v364 * v369);
        let v371: f64 = (v360 - v370);
        let v372: f64 = (self.scalar_v362 * v371);
        let v373: f64 = (v314 + v372);
        let v375: f64 = (v304 * self.scalar_v374);
        let v377: f64 = (v302 * self.scalar_v376);
        let v378: f64 = (v345 + v375);
        let v379: f64 = (v373 + v377);
        let v380: f64 = (self.scalar_v14 * v378);
        let v381: f64 = (self.scalar_v14 * v379);
        let v400: f64 = (v217 * self.scalar_v399);
        let v401: f64 = (v400 + v400);
        let v402: f64 = (v58 * v221);
        let v403: f64 = (v401 / v402);
        let v404: f64 = (self.scalar_v399 - v403);
        let v405: f64 = (v172 * v404);
        let v406: f64 = (v224 * v405);
        let v407: f64 = (self.scalar_v227 * v405);
        let v408: f64 = (v228 * v405);
        let v409: f64 = (v223 * v407);
        let v410: f64 = (v408 + v409);
        let v411: f64 = (v231 * v410);
        let v412: f64 = (v229 * v405);
        let v413: f64 = (v411 - v412);
        let v414: f64 = (v231 * v231);
        let v415: f64 = (v413 / v414);
        let v416: f64 = (-v415);
        let v417: f64 = (v405 / v185);
        let v418: f64 = (v58 * v235);
        let v419: f64 = (v417 / v418);
        let v420: f64 = (v235 * v417);
        let v421: f64 = (v234 * v419);
        let v422: f64 = (v420 + v421);
        let v423: f64 = (self.scalar_v237 * v422);
        let v424: f64 = (v58 * v406);
        let v425: f64 = (v241 * v416);
        let v426: f64 = (v233 * v424);
        let v427: f64 = (v425 - v426);
        let v428: f64 = (v241 * v241);
        let v429: f64 = (v427 / v428);
        let v430: f64 = (-v429);
        let v431: f64 = { let limited_exp_arg = v243; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } };
        let v432: f64 = (v430 * v431);
        let v433: f64 = (v244 * v423);
        let v434: f64 = (v238 * v432);
        let v435: f64 = (v433 + v434);
        let v436: f64 = (v172 * v416);
        let v437: f64 = (self.scalar_v248 * v435);
        let v438: f64 = (-v437);
        let v439: f64 = (v438 / v246);
        let v440: f64 = (if v250 { v439 } else { v0 });
        let v441: f64 = (v440 / v251);
        let v442: f64 = (v252 * v406);
        let v443: f64 = (v225 * v441);
        let v444: f64 = (v442 + v443);
        let v445: f64 = (v436 - v444);
        let v446: f64 = (v254 * v445);
        let v447: f64 = (v446 + v446);
        let v448: f64 = (v58 * v259);
        let v449: f64 = (v447 / v448);
        let v450: f64 = (v445 + v449);
        let v451: f64 = (v172 * v450);
        let v452: f64 = (v436 - v451);
        let v453: f64 = (if self.scalar_v268 { v452 } else { v0 });
        let v454: f64 = (v453 - v452);
        let v455: f64 = (if self.scalar_v274 { v454 } else { v453 });
        let v456: f64 = (v416 / v58);
        let v457: f64 = (self.scalar_v108 * v435);
        let v458: f64 = (-v457);
        let v459: f64 = (v458 / v246);
        let v460: f64 = (if v282 { v459 } else { v0 });
        let v461: f64 = (v460 / v283);
        let v462: f64 = (v284 * v406);
        let v463: f64 = (v225 * v461);
        let v464: f64 = (v462 + v463);
        let v465: f64 = (if v286 { v456 } else { v464 });
        let v466: f64 = (self.scalar_v9 * v465);
        let v467: f64 = (v456 - v466);
        let v468: f64 = (-v467);
        let v469: f64 = (self.scalar_v9 * v468);
        let v470: f64 = (v455 - v467);
        let v471: f64 = (self.scalar_v9 * v470);
        let v472: f64 = (-v469);
        let v475: f64 = (-v471);
        let v477: f64 = (self.scalar_v317 * v475);
        let v480: f64 = (self.scalar_v322 * v477);
        let v482: f64 = (v472 + v480);
        let v484: f64 = (v324 * self.scalar_v479);
        let v485: f64 = (v484 + v484);
        let v486: f64 = (v324 * v482);
        let v487: f64 = (v486 + v486);
        let v488: f64 = (v324 * self.scalar_v483);
        let v489: f64 = (v488 + v488);
        let v490: f64 = (self.scalar_v9 * v324);
        let v491: f64 = (v490 + v490);
        let v492: f64 = (v58 * v328);
        let v493: f64 = (v485 / v492);
        let v494: f64 = (v487 / v492);
        let v495: f64 = (v489 / v492);
        let v496: f64 = (v491 / v492);
        let v497: f64 = (self.scalar_v479 - v493);
        let v498: f64 = (v482 - v494);
        let v499: f64 = (self.scalar_v483 - v495);
        let v500: f64 = (self.scalar_v9 - v496);
        let v501: f64 = (v172 * v497);
        let v502: f64 = (v172 * v498);
        let v503: f64 = (v172 * v499);
        let v504: f64 = (v172 * v500);
        let v505: f64 = (-v501);
        let v506: f64 = (v472 - v502);
        let v507: f64 = (self.scalar_v307 - v503);
        let v508: f64 = (self.scalar_v9 - v504);
        let v509: f64 = (v256 * v501);
        let v510: f64 = (v256 * v502);
        let v511: f64 = (v256 * v503);
        let v512: f64 = (v256 * v504);
        let v513: f64 = (v509 / self.scalar_v335);
        let v514: f64 = (v510 / self.scalar_v335);
        let v515: f64 = (v511 / self.scalar_v335);
        let v516: f64 = (v512 / self.scalar_v335);
        let v517: f64 = (-v513);
        let v518: f64 = (-v514);
        let v519: f64 = (-v515);
        let v520: f64 = (-v516);
        let v521: f64 = (v58 * v340);
        let v522: f64 = (v517 / v521);
        let v523: f64 = (v518 / v521);
        let v524: f64 = (v519 / v521);
        let v525: f64 = (v520 / v521);
        let v526: f64 = (self.scalar_v336 * v522);
        let v527: f64 = (self.scalar_v336 * v523);
        let v528: f64 = (self.scalar_v336 * v524);
        let v529: f64 = (self.scalar_v336 * v525);
        let v530: f64 = (v505 - v526);
        let v531: f64 = (v506 - v527);
        let v532: f64 = (v507 - v528);
        let v533: f64 = (v508 - v529);
        let v534: f64 = (self.scalar_v334 * v530);
        let v535: f64 = (self.scalar_v334 * v531);
        let v536: f64 = (self.scalar_v334 * v532);
        let v537: f64 = (self.scalar_v334 * v533);
        let v538: f64 = (self.scalar_v473 + v536);
        let v539: f64 = (self.scalar_v310 + v537);
        let v541: f64 = (self.scalar_v352 * v477);
        let v543: f64 = (v472 + v541);
        let v545: f64 = (v354 * self.scalar_v540);
        let v546: f64 = (v545 + v545);
        let v547: f64 = (v354 * v543);
        let v548: f64 = (v547 + v547);
        let v549: f64 = (v354 * self.scalar_v544);
        let v550: f64 = (v549 + v549);
        let v551: f64 = (self.scalar_v9 * v354);
        let v552: f64 = (v551 + v551);
        let v553: f64 = (v58 * v357);
        let v554: f64 = (v546 / v553);
        let v555: f64 = (v548 / v553);
        let v556: f64 = (v550 / v553);
        let v557: f64 = (v552 / v553);
        let v558: f64 = (self.scalar_v540 - v554);
        let v559: f64 = (v543 - v555);
        let v560: f64 = (self.scalar_v544 - v556);
        let v561: f64 = (self.scalar_v9 - v557);
        let v562: f64 = (v172 * v558);
        let v563: f64 = (v172 * v559);
        let v564: f64 = (v172 * v560);
        let v565: f64 = (v172 * v561);
        let v566: f64 = (-v562);
        let v567: f64 = (v472 - v563);
        let v568: f64 = (self.scalar_v307 - v564);
        let v569: f64 = (self.scalar_v9 - v565);
        let v570: f64 = (v256 * v562);
        let v571: f64 = (v256 * v563);
        let v572: f64 = (v256 * v564);
        let v573: f64 = (v256 * v565);
        let v574: f64 = (v570 / self.scalar_v363);
        let v575: f64 = (v571 / self.scalar_v363);
        let v576: f64 = (v572 / self.scalar_v363);
        let v577: f64 = (v573 / self.scalar_v363);
        let v578: f64 = (-v574);
        let v579: f64 = (-v575);
        let v580: f64 = (-v576);
        let v581: f64 = (-v577);
        let v582: f64 = (v58 * v368);
        let v583: f64 = (v578 / v582);
        let v584: f64 = (v579 / v582);
        let v585: f64 = (v580 / v582);
        let v586: f64 = (v581 / v582);
        let v587: f64 = (self.scalar_v364 * v583);
        let v588: f64 = (self.scalar_v364 * v584);
        let v589: f64 = (self.scalar_v364 * v585);
        let v590: f64 = (self.scalar_v364 * v586);
        let v591: f64 = (v566 - v587);
        let v592: f64 = (v567 - v588);
        let v593: f64 = (v568 - v589);
        let v594: f64 = (v569 - v590);
        let v595: f64 = (self.scalar_v362 * v591);
        let v596: f64 = (self.scalar_v362 * v592);
        let v597: f64 = (self.scalar_v362 * v593);
        let v598: f64 = (self.scalar_v362 * v594);
        let v599: f64 = (self.scalar_v474 + v597);
        let v600: f64 = (self.scalar_v313 + v598);
        let v603: f64 = (v538 + self.scalar_v601);
        let v604: f64 = (self.scalar_v374 + v539);
        let v605: f64 = (v599 + self.scalar_v602);
        let v606: f64 = (self.scalar_v376 + v600);
        let v607: f64 = (self.scalar_v14 * v534);
        let v608: f64 = (self.scalar_v14 * v535);
        let v609: f64 = (self.scalar_v14 * v603);
        let v610: f64 = (self.scalar_v14 * v604);
        let v611: f64 = (self.scalar_v14 * v595);
        let v612: f64 = (self.scalar_v14 * v596);
        let v613: f64 = (self.scalar_v14 * v605);
        let v614: f64 = (self.scalar_v14 * v606);

        let d380_dn3: f64 = v607;
        let d380_dn4: f64 = v608;
        let d380_dn6: f64 = v609;
        let d380_dn7: f64 = v610;
        let v380_reactive_nodes: [usize; 4] = [nodes[3], nodes[4], nodes[6], nodes[7]];
        let v380_reactive_node_derivatives: [f64; 4] = [d380_dn3, d380_dn4, d380_dn6, d380_dn7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            &v380_reactive_nodes,
            &v380_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d381_dn3: f64 = v611;
        let d381_dn4: f64 = v612;
        let d381_dn5: f64 = v613;
        let d381_dn7: f64 = v614;
        let v381_reactive_nodes: [usize; 4] = [nodes[3], nodes[4], nodes[5], nodes[7]];
        let v381_reactive_node_derivatives: [f64; 4] = [d381_dn3, d381_dn4, d381_dn5, d381_dn7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &v381_reactive_nodes,
            &v381_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let mut locals = StampLocals::default();

        Self::stamp_reactive_block_0(p, &mut locals);
        Self::stamp_reactive_block_1(p, &mut locals);
        Self::stamp_reactive_block_2(p, &mut locals);
        Self::stamp_reactive_block_3(p, param_given, &mut locals);
        Self::stamp_reactive_block_4(ctx, p, nodes, &mut locals);
        Self::stamp_reactive_block_5(p, param_given, &mut locals);
        Self::stamp_reactive_block_6(ctx, p, nodes, &mut locals);
        Self::stamp_reactive_block_7(p, &mut locals);
        Self::stamp_reactive_block_8(p, &mut locals);
        Self::stamp_reactive_block_9(&mut locals);
        Self::stamp_reactive_block_10(&mut locals);
        Self::stamp_reactive_block_11(&mut locals);
        Self::stamp_reactive_block_12(&mut locals);
        Self::stamp_reactive_block_13(&mut locals);
        Self::stamp_reactive_block_14(p, &mut locals);
        Self::stamp_reactive_block_15(p, &mut locals);
        Self::stamp_reactive_block_16(p, &mut locals);
        Self::stamp_reactive_block_17(&mut locals);
        Self::stamp_reactive_block_18(&mut locals);
        Self::stamp_reactive_block_19(&mut locals);
        Self::stamp_reactive_block_20(&mut locals);
        Self::stamp_reactive_block_21(&mut locals);
        Self::stamp_reactive_block_22(p, &mut locals);
        Self::stamp_reactive_block_23(p, &mut locals);
        Self::stamp_reactive_block_24(p, &mut locals);
        Self::stamp_reactive_block_25(p, &mut locals);
        Self::stamp_reactive_block_26(ctx, p, nodes, &mut locals);
        Self::stamp_reactive_block_27(p, &mut locals);
        Self::stamp_reactive_block_28(p, &mut locals);
        Self::stamp_reactive_block_29(p, &mut locals);
        Self::stamp_reactive_block_30(p, &mut locals);

        Self::stamp_reactive_equations_block_0(ctx, stamper, nodes, branches, multiplicity, &mut locals);
    }
}
