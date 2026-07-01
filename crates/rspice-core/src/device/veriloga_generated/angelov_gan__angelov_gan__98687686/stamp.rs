#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;

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
    pub(crate) var_cdel_t: f64,
    pub(crate) var_cdel_t_dn3: f64,
    pub(crate) var_cdel_t_rv: f64,
    pub(crate) var_cgd: f64,
    pub(crate) var_cgd0_t: f64,
    pub(crate) var_cgd0_t_dn3: f64,
    pub(crate) var_cgd0_t_rv: f64,
    pub(crate) var_cgd_dn10: f64,
    pub(crate) var_cgd_dn11: f64,
    pub(crate) var_cgd_dn3: f64,
    pub(crate) var_cgd_dn5: f64,
    pub(crate) var_cgd_dn8: f64,
    pub(crate) var_cgd_rv: f64,
    pub(crate) var_cgs: f64,
    pub(crate) var_cgs0_t: f64,
    pub(crate) var_cgs0_t_dn3: f64,
    pub(crate) var_cgs0_t_rv: f64,
    pub(crate) var_cgs_dn10: f64,
    pub(crate) var_cgs_dn11: f64,
    pub(crate) var_cgs_dn3: f64,
    pub(crate) var_cgs_dn5: f64,
    pub(crate) var_cgs_dn8: f64,
    pub(crate) var_cgs_rv: f64,
    pub(crate) var_cgsdepl: f64,
    pub(crate) var_cgsdepl_dn11: f64,
    pub(crate) var_cgsdepl_dn8: f64,
    pub(crate) var_cgsdepl_rv: f64,
    pub(crate) var_ci: f64,
    pub(crate) var_ci_dn3: f64,
    pub(crate) var_ci_rv: f64,
    pub(crate) var_cosh0: f64,
    pub(crate) var_cosh0_dn3: f64,
    pub(crate) var_cosh0_dn5: f64,
    pub(crate) var_cosh0_dn8: f64,
    pub(crate) var_cosh0_rv: f64,
    pub(crate) var_cosh1: f64,
    pub(crate) var_cosh1_dn10: f64,
    pub(crate) var_cosh1_dn11: f64,
    pub(crate) var_cosh1_dn3: f64,
    pub(crate) var_cosh1_dn5: f64,
    pub(crate) var_cosh1_dn8: f64,
    pub(crate) var_cosh1_rv: f64,
    pub(crate) var_delta_t: f64,
    pub(crate) var_delta_t_dn3: f64,
    pub(crate) var_delta_t_rv: f64,
    pub(crate) var_guard1: f64,
    pub(crate) var_guard11: f64,
    pub(crate) var_guard11_rv: f64,
    pub(crate) var_guard12: f64,
    pub(crate) var_guard12_rv: f64,
    pub(crate) var_guard14: f64,
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
    pub(crate) var_guard1_rv: f64,
    pub(crate) var_guard2: f64,
    pub(crate) var_guard20: f64,
    pub(crate) var_guard20_rv: f64,
    pub(crate) var_guard21: f64,
    pub(crate) var_guard21_rv: f64,
    pub(crate) var_guard25: f64,
    pub(crate) var_guard26: f64,
    pub(crate) var_guard26_rv: f64,
    pub(crate) var_guard27: f64,
    pub(crate) var_guard27_rv: f64,
    pub(crate) var_guard28: f64,
    pub(crate) var_guard28_rv: f64,
    pub(crate) var_guard29: f64,
    pub(crate) var_guard29_rv: f64,
    pub(crate) var_guard2_rv: f64,
    pub(crate) var_guard3: f64,
    pub(crate) var_guard3_rv: f64,
    pub(crate) var_guard4: f64,
    pub(crate) var_guard44: f64,
    pub(crate) var_guard44_rv: f64,
    pub(crate) var_guard4_rv: f64,
    pub(crate) var_guard5: f64,
    pub(crate) var_guard5_rv: f64,
    pub(crate) var_guard6: f64,
    pub(crate) var_guard6_rv: f64,
    pub(crate) var_guard7: f64,
    pub(crate) var_guard7_rv: f64,
    pub(crate) var_guard8: f64,
    pub(crate) var_guard8_rv: f64,
    pub(crate) var_guard9: f64,
    pub(crate) var_guard9_rv: f64,
    pub(crate) var_k: f64,
    pub(crate) var_k_dn3: f64,
    pub(crate) var_k_rv: f64,
    pub(crate) var_lc1: f64,
    pub(crate) var_lc10: f64,
    pub(crate) var_lc10_dn3: f64,
    pub(crate) var_lc10_dn5: f64,
    pub(crate) var_lc10_dn8: f64,
    pub(crate) var_lc10_rv: f64,
    pub(crate) var_lc1_dn10: f64,
    pub(crate) var_lc1_dn11: f64,
    pub(crate) var_lc1_dn3: f64,
    pub(crate) var_lc1_dn5: f64,
    pub(crate) var_lc1_dn8: f64,
    pub(crate) var_lc1_rv: f64,
    pub(crate) var_lc4: f64,
    pub(crate) var_lc40: f64,
    pub(crate) var_lc40_dn3: f64,
    pub(crate) var_lc40_dn5: f64,
    pub(crate) var_lc40_dn8: f64,
    pub(crate) var_lc40_rv: f64,
    pub(crate) var_lc4_dn10: f64,
    pub(crate) var_lc4_dn11: f64,
    pub(crate) var_lc4_dn3: f64,
    pub(crate) var_lc4_dn5: f64,
    pub(crate) var_lc4_dn8: f64,
    pub(crate) var_lc4_rv: f64,
    pub(crate) var_mjc: f64,
    pub(crate) var_mjc_rv: f64,
    pub(crate) var_p10_t: f64,
    pub(crate) var_p10_t_dn3: f64,
    pub(crate) var_p10_t_rv: f64,
    pub(crate) var_p1_t: f64,
    pub(crate) var_p1_t_db1: f64,
    pub(crate) var_p1_t_dn10: f64,
    pub(crate) var_p1_t_dn12: f64,
    pub(crate) var_p1_t_dn3: f64,
    pub(crate) var_p1_t_dn4: f64,
    pub(crate) var_p1_t_dn5: f64,
    pub(crate) var_p1_t_dn8: f64,
    pub(crate) var_p1_t_rdb1: f64,
    pub(crate) var_p1_t_rv: f64,
    pub(crate) var_p1m: f64,
    pub(crate) var_p1m_db1: f64,
    pub(crate) var_p1m_dn10: f64,
    pub(crate) var_p1m_dn12: f64,
    pub(crate) var_p1m_dn3: f64,
    pub(crate) var_p1m_dn4: f64,
    pub(crate) var_p1m_dn5: f64,
    pub(crate) var_p1m_dn8: f64,
    pub(crate) var_p1m_rdb1: f64,
    pub(crate) var_p1m_rv: f64,
    pub(crate) var_p3_t: f64,
    pub(crate) var_p3_t_dn3: f64,
    pub(crate) var_p3_t_rv: f64,
    pub(crate) var_p40_t: f64,
    pub(crate) var_p40_t_dn3: f64,
    pub(crate) var_p40_t_rv: f64,
    pub(crate) var_pg_param: f64,
    pub(crate) var_pg_param_dn3: f64,
    pub(crate) var_pg_param_rv: f64,
    pub(crate) var_psi: f64,
    pub(crate) var_psi_1: f64,
    pub(crate) var_psi_1_dn11: f64,
    pub(crate) var_psi_1_dn3: f64,
    pub(crate) var_psi_1_dn5: f64,
    pub(crate) var_psi_1_dn8: f64,
    pub(crate) var_psi_1_rv: f64,
    pub(crate) var_psi_2: f64,
    pub(crate) var_psi_2_dn5: f64,
    pub(crate) var_psi_2_dn8: f64,
    pub(crate) var_psi_2_rv: f64,
    pub(crate) var_psi_3: f64,
    pub(crate) var_psi_3_dn5: f64,
    pub(crate) var_psi_3_dn8: f64,
    pub(crate) var_psi_3_rv: f64,
    pub(crate) var_psi_4: f64,
    pub(crate) var_psi_4_dn10: f64,
    pub(crate) var_psi_4_dn3: f64,
    pub(crate) var_psi_4_dn5: f64,
    pub(crate) var_psi_4_dn8: f64,
    pub(crate) var_psi_4_rv: f64,
    pub(crate) var_psi_db1: f64,
    pub(crate) var_psi_dn10: f64,
    pub(crate) var_psi_dn12: f64,
    pub(crate) var_psi_dn3: f64,
    pub(crate) var_psi_dn4: f64,
    pub(crate) var_psi_dn5: f64,
    pub(crate) var_psi_dn8: f64,
    pub(crate) var_psi_rdb1: f64,
    pub(crate) var_psi_rv: f64,
    pub(crate) var_qgd: f64,
    pub(crate) var_qgd0: f64,
    pub(crate) var_qgd0_dn3: f64,
    pub(crate) var_qgd0_dn5: f64,
    pub(crate) var_qgd0_dn8: f64,
    pub(crate) var_qgd0_rv: f64,
    pub(crate) var_qgd_dn10: f64,
    pub(crate) var_qgd_dn11: f64,
    pub(crate) var_qgd_dn3: f64,
    pub(crate) var_qgd_dn5: f64,
    pub(crate) var_qgd_dn8: f64,
    pub(crate) var_qgd_rv: f64,
    pub(crate) var_qgs: f64,
    pub(crate) var_qgs0: f64,
    pub(crate) var_qgs0_dn3: f64,
    pub(crate) var_qgs0_dn5: f64,
    pub(crate) var_qgs0_dn8: f64,
    pub(crate) var_qgs0_rv: f64,
    pub(crate) var_qgs_dn10: f64,
    pub(crate) var_qgs_dn11: f64,
    pub(crate) var_qgs_dn3: f64,
    pub(crate) var_qgs_dn5: f64,
    pub(crate) var_qgs_dn8: f64,
    pub(crate) var_qgs_rv: f64,
    pub(crate) var_qgsdepl: f64,
    pub(crate) var_qgsdepl0: f64,
    pub(crate) var_qgsdepl0_rv: f64,
    pub(crate) var_qgsdepl_dn11: f64,
    pub(crate) var_qgsdepl_dn8: f64,
    pub(crate) var_qgsdepl_rv: f64,
    pub(crate) var_rc1: f64,
    pub(crate) var_rc1_db1: f64,
    pub(crate) var_rc1_dn10: f64,
    pub(crate) var_rc1_dn12: f64,
    pub(crate) var_rc1_dn3: f64,
    pub(crate) var_rc1_dn4: f64,
    pub(crate) var_rc1_dn5: f64,
    pub(crate) var_rc1_dn8: f64,
    pub(crate) var_rc1_rdb1: f64,
    pub(crate) var_rc1_rv: f64,
    pub(crate) var_rc_t: f64,
    pub(crate) var_rc_t_dn3: f64,
    pub(crate) var_rc_t_rv: f64,
    pub(crate) var_rd1: f64,
    pub(crate) var_rd1_db1: f64,
    pub(crate) var_rd1_dn10: f64,
    pub(crate) var_rd1_dn12: f64,
    pub(crate) var_rd1_dn3: f64,
    pub(crate) var_rd1_dn4: f64,
    pub(crate) var_rd1_dn5: f64,
    pub(crate) var_rd1_dn8: f64,
    pub(crate) var_rd1_rdb1: f64,
    pub(crate) var_rd1_rv: f64,
    pub(crate) var_rd1_t: f64,
    pub(crate) var_rd1_t_db1: f64,
    pub(crate) var_rd1_t_dn10: f64,
    pub(crate) var_rd1_t_dn12: f64,
    pub(crate) var_rd1_t_dn3: f64,
    pub(crate) var_rd1_t_dn4: f64,
    pub(crate) var_rd1_t_dn5: f64,
    pub(crate) var_rd1_t_dn8: f64,
    pub(crate) var_rd1_t_rdb1: f64,
    pub(crate) var_rd1_t_rv: f64,
    pub(crate) var_rs1: f64,
    pub(crate) var_rs1_db1: f64,
    pub(crate) var_rs1_dn10: f64,
    pub(crate) var_rs1_dn12: f64,
    pub(crate) var_rs1_dn3: f64,
    pub(crate) var_rs1_dn4: f64,
    pub(crate) var_rs1_dn5: f64,
    pub(crate) var_rs1_dn8: f64,
    pub(crate) var_rs1_rdb1: f64,
    pub(crate) var_rs1_rv: f64,
    pub(crate) var_rs_t: f64,
    pub(crate) var_rs_t_db1: f64,
    pub(crate) var_rs_t_dn10: f64,
    pub(crate) var_rs_t_dn12: f64,
    pub(crate) var_rs_t_dn3: f64,
    pub(crate) var_rs_t_dn4: f64,
    pub(crate) var_rs_t_dn5: f64,
    pub(crate) var_rs_t_dn8: f64,
    pub(crate) var_rs_t_rdb1: f64,
    pub(crate) var_rs_t_rv: f64,
    pub(crate) var_t: f64,
    pub(crate) var_t0: f64,
    pub(crate) var_t0_db1: f64,
    pub(crate) var_t0_dn10: f64,
    pub(crate) var_t0_dn12: f64,
    pub(crate) var_t0_dn3: f64,
    pub(crate) var_t0_dn4: f64,
    pub(crate) var_t0_dn5: f64,
    pub(crate) var_t0_dn8: f64,
    pub(crate) var_t0_rdb1: f64,
    pub(crate) var_t0_rv: f64,
    pub(crate) var_t1: f64,
    pub(crate) var_t1_db1: f64,
    pub(crate) var_t1_dn10: f64,
    pub(crate) var_t1_dn12: f64,
    pub(crate) var_t1_dn3: f64,
    pub(crate) var_t1_dn4: f64,
    pub(crate) var_t1_dn5: f64,
    pub(crate) var_t1_dn8: f64,
    pub(crate) var_t1_rdb1: f64,
    pub(crate) var_t1_rv: f64,
    pub(crate) var_t2: f64,
    pub(crate) var_t2_db1: f64,
    pub(crate) var_t2_dn10: f64,
    pub(crate) var_t2_dn12: f64,
    pub(crate) var_t2_dn3: f64,
    pub(crate) var_t2_dn4: f64,
    pub(crate) var_t2_dn5: f64,
    pub(crate) var_t2_dn8: f64,
    pub(crate) var_t2_rdb1: f64,
    pub(crate) var_t2_rv: f64,
    pub(crate) var_t_dn3: f64,
    pub(crate) var_t_nom: f64,
    pub(crate) var_t_nom_rv: f64,
    pub(crate) var_t_rv: f64,
    pub(crate) var_tanh1: f64,
    pub(crate) var_tanh1_dn11: f64,
    pub(crate) var_tanh1_dn3: f64,
    pub(crate) var_tanh1_dn5: f64,
    pub(crate) var_tanh1_dn8: f64,
    pub(crate) var_tanh1_rv: f64,
    pub(crate) var_tanh2: f64,
    pub(crate) var_tanh2_dn5: f64,
    pub(crate) var_tanh2_dn8: f64,
    pub(crate) var_tanh2_rv: f64,
    pub(crate) var_tanh3: f64,
    pub(crate) var_tanh3_dn5: f64,
    pub(crate) var_tanh3_dn8: f64,
    pub(crate) var_tanh3_rv: f64,
    pub(crate) var_tanh4: f64,
    pub(crate) var_tanh4_dn10: f64,
    pub(crate) var_tanh4_dn3: f64,
    pub(crate) var_tanh4_dn5: f64,
    pub(crate) var_tanh4_dn8: f64,
    pub(crate) var_tanh4_rv: f64,
    pub(crate) var_tanh_psi: f64,
    pub(crate) var_tanh_psi1: f64,
    pub(crate) var_tanh_psi1_db1: f64,
    pub(crate) var_tanh_psi1_dn10: f64,
    pub(crate) var_tanh_psi1_dn12: f64,
    pub(crate) var_tanh_psi1_dn3: f64,
    pub(crate) var_tanh_psi1_dn4: f64,
    pub(crate) var_tanh_psi1_dn5: f64,
    pub(crate) var_tanh_psi1_dn8: f64,
    pub(crate) var_tanh_psi1_rdb1: f64,
    pub(crate) var_tanh_psi1_rv: f64,
    pub(crate) var_tanh_psi_db1: f64,
    pub(crate) var_tanh_psi_dn10: f64,
    pub(crate) var_tanh_psi_dn12: f64,
    pub(crate) var_tanh_psi_dn3: f64,
    pub(crate) var_tanh_psi_dn4: f64,
    pub(crate) var_tanh_psi_dn5: f64,
    pub(crate) var_tanh_psi_dn8: f64,
    pub(crate) var_tanh_psi_rdb1: f64,
    pub(crate) var_tanh_psi_rv: f64,
    pub(crate) var_vbg: f64,
    pub(crate) var_vbg_dn4: f64,
    pub(crate) var_vbg_dn8: f64,
    pub(crate) var_vbg_rv: f64,
    pub(crate) var_vdg: f64,
    pub(crate) var_vdg_dn10: f64,
    pub(crate) var_vdg_dn5: f64,
    pub(crate) var_vdg_rv: f64,
    pub(crate) var_vds: f64,
    pub(crate) var_vds_dn5: f64,
    pub(crate) var_vds_dn8: f64,
    pub(crate) var_vds_rv: f64,
    pub(crate) var_vgd: f64,
    pub(crate) var_vgd_dn10: f64,
    pub(crate) var_vgd_dn5: f64,
    pub(crate) var_vgd_rv: f64,
    pub(crate) var_vgdc: f64,
    pub(crate) var_vgdc_dn10: f64,
    pub(crate) var_vgdc_dn5: f64,
    pub(crate) var_vgdc_rv: f64,
    pub(crate) var_vgsc: f64,
    pub(crate) var_vgsc_dn11: f64,
    pub(crate) var_vgsc_dn8: f64,
    pub(crate) var_vgsc_rv: f64,
    pub(crate) var_vgsdel: f64,
    pub(crate) var_vgsdel_dn12: f64,
    pub(crate) var_vgsdel_dn8: f64,
    pub(crate) var_vgsdel_rv: f64,
    pub(crate) var_vjg_t: f64,
    pub(crate) var_vjg_t_dn3: f64,
    pub(crate) var_vjg_t_rv: f64,
    pub(crate) var_vpkm: f64,
    pub(crate) var_vpkm_dn10: f64,
    pub(crate) var_vpkm_dn3: f64,
    pub(crate) var_vpkm_dn4: f64,
    pub(crate) var_vpkm_dn5: f64,
    pub(crate) var_vpkm_dn8: f64,
    pub(crate) var_vpkm_rv: f64,
    pub(crate) var_vpkm_t: f64,
    pub(crate) var_vpkm_t_dn10: f64,
    pub(crate) var_vpkm_t_dn3: f64,
    pub(crate) var_vpkm_t_dn4: f64,
    pub(crate) var_vpkm_t_dn5: f64,
    pub(crate) var_vpkm_t_dn8: f64,
    pub(crate) var_vpkm_t_rv: f64,
    pub(crate) var_vpks_t: f64,
    pub(crate) var_vpks_t_dn3: f64,
    pub(crate) var_vpks_t_rv: f64,
    pub(crate) var_vrf: f64,
    pub(crate) var_vrf_dn4: f64,
    pub(crate) var_vrf_dn8: f64,
    pub(crate) var_vrf_rv: f64,
    pub(crate) var_vth: f64,
    pub(crate) var_vth_dn3: f64,
    pub(crate) var_vth_rv: f64,
    pub(crate) var_vtr_t: f64,
    pub(crate) var_vtr_t_dn3: f64,
    pub(crate) var_vtr_t_rv: f64,
    pub(crate) var_y: f64,
    pub(crate) var_y_dn11: f64,
    pub(crate) var_y_dn8: f64,
    pub(crate) var_y_rv: f64,
}

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
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
        let v0: f64 = nv12;
        let v1: f64 = nv8;
        let v2: f64 = (v0 - v1);
        let v3: f64 = nv10;
        let v4: f64 = nv5;
        let v5: f64 = (v3 - v4);
        let v6: f64 = (-v5);
        let v7: f64 = (v4 - v1);
        let v8: f64 = nv11;
        let v9: f64 = (v8 - v1);
        let v10: f64 = nv4;
        let v11: f64 = (v10 - v1);
        let v12: f64 = nv16;
        let v13: f64 = 0.0;
        let v32: f64 = nv3;
        let v33: f64 = ((v32) as f64).abs();
        let v34: f64 = (self.scalar_v23 + v33);
        let v35: f64 = (if (self.scalar_v31 != 0.0) { v34 } else { self.scalar_v23 });
        let v36: f64 = 8.617333262145179e-5;
        let v37: f64 = (v35 * v36);
        let v38: f64 = (v35 - self.scalar_v30);
        let v39: f64 = ((v38) as f64).abs();
        let v40: bool = (v39 > v13);
        let v43: bool = (v40 || self.scalar_v42);
        let v44: f64 = 1.0;
        let v46: f64 = ((v39) as f64).abs();
        let v47: f64 = (self.scalar_v45 * v46);
        let v48: f64 = (v44 + v47);
        let v49: f64 = (self.scalar_v41 * v48);
        let v52: f64 = (v46 * self.scalar_v51);
        let v53: f64 = (v44 + v52);
        let v54: f64 = (self.scalar_v50 * v53);
        let v55: f64 = (if v43 { v54 } else { v13 });
        let v58: f64 = (v46 * self.scalar_v57);
        let v59: f64 = (v44 + v58);
        let v60: f64 = (self.scalar_v56 * v59);
        let v61: f64 = (if v43 { v60 } else { v13 });
        let v64: f64 = (v46 * self.scalar_v63);
        let v65: f64 = (v44 + v64);
        let v66: f64 = (self.scalar_v62 * v65);
        let v67: f64 = (if v43 { v66 } else { v13 });
        let v71: f64 = (v46 * self.scalar_v70);
        let v72: f64 = (v44 + v71);
        let v73: f64 = (self.scalar_v69 * v72);
        let v74: f64 = (if v43 { v73 } else { v13 });
        let v77: f64 = (v39 * self.scalar_v76);
        let v78: f64 = (self.scalar_v75 + v77);
        let v79: f64 = (if v43 { v78 } else { v13 });
        let v82: f64 = (v39 * self.scalar_v81);
        let v83: f64 = (self.scalar_v80 + v82);
        let v84: f64 = (if v43 { v83 } else { v13 });
        let v87: f64 = (v39 * self.scalar_v86);
        let v88: f64 = (self.scalar_v85 + v87);
        let v89: f64 = (if v43 { v88 } else { v13 });
        let v98: bool = (v43 && self.scalar_v97);
        let v100: f64 = (v39 * v39);
        let v101: f64 = (self.scalar_v70 * v100);
        let v102: f64 = (v44 + v101);
        let v103: f64 = (self.scalar_v99 * v102);
        let v104: f64 = (if v98 { v103 } else { v13 });
        let v107: bool = (v43 && self.scalar_v106);
        let v108: f64 = (v72 * self.scalar_v99);
        let v109: f64 = (if v107 { v108 } else { v104 });
        let v110: bool = (!v43);
        let v111: f64 = (if v110 { self.scalar_v50 } else { v55 });
        let v112: f64 = (if v110 { self.scalar_v56 } else { v61 });
        let v113: f64 = (if v110 { self.scalar_v62 } else { v67 });
        let v114: f64 = (if v110 { self.scalar_v69 } else { v74 });
        let v115: f64 = (if v110 { self.scalar_v99 } else { v109 });
        let v116: f64 = (if v110 { self.scalar_v75 } else { v79 });
        let v117: f64 = (if v110 { self.scalar_v80 } else { v84 });
        let v118: f64 = (if v110 { self.scalar_v85 } else { v89 });
        let v123: f64 = 0.5;
        let v126: f64 = (self.scalar_v125 / v37);
        let v127: f64 = (if self.scalar_v122 { v126 } else { v13 });
        let v130: f64 = (if self.scalar_v128 { self.scalar_v129 } else { v127 });
        let v132: f64 = (v7 * self.scalar_v131);
        let v133: f64 = ((v132) as f64).cosh();
        let v135: f64 = (v11 * self.scalar_v134);
        let v138: f64 = 1e-12;
        let v139: f64 = (v133 * v133);
        let v140: f64 = (v138 + v139);
        let v141: f64 = (self.scalar_v137 / v140);
        let v142: f64 = (v44 + v141);
        let v143: f64 = (self.scalar_v136 * v142);
        let v145: f64 = (v46 * self.scalar_v144);
        let v146: f64 = (v44 + v145);
        let v147: f64 = (v143 * v146);
        let v150: f64 = (v46 * self.scalar_v149);
        let v151: f64 = (v44 + v150);
        let v152: f64 = (self.scalar_v148 * v151);
        let v154: f64 = (v116 - self.scalar_v153);
        let v156: f64 = (v7 * self.scalar_v155);
        let v157: f64 = ((v156) as f64).tanh();
        let v158: f64 = (self.scalar_v153 * v157);
        let v159: f64 = (v154 + v158);
        let v160: f64 = (v159 - v135);
        let v162: f64 = (v6 - v118);
        let v163: f64 = (self.scalar_v161 * v162);
        let v164: f64 = (v162 * v163);
        let v165: f64 = (v160 - v164);
        let v166: f64 = (v46 * self.scalar_v76);
        let v167: f64 = (v44 + v166);
        let v168: f64 = (v165 * v167);
        let v169: f64 = (v2 - v168);
        let v170: f64 = (v169 * v169);
        let v171: f64 = (v147 * v169);
        let v173: f64 = (v170 * self.scalar_v172);
        let v174: f64 = (v171 + v173);
        let v175: f64 = (v152 * v169);
        let v176: f64 = (v170 * v175);
        let v177: f64 = (v174 + v176);
        let v178: f64 = ((v177) as f64).tanh();
        let v179: f64 = (v44 + v178);
        let v180: f64 = { let limexp_arg = v177; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v181: f64 = (-v177);
        let v182: f64 = { let limexp_arg = v181; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v183: f64 = (v180 - v182);
        let v184: f64 = (v123 * v183);
        let v185: f64 = ((v184) as f64).tanh();
        let v186: f64 = (v44 + v185);
        let v188: f64 = (self.scalar_v155 * v179);
        let v189: f64 = (self.scalar_v187 + v188);
        let v190: f64 = (v7 * v189);
        let v191: f64 = ((v190) as f64).tanh();
        let v197: f64 = (v111 * v179);
        let v198: f64 = (v191 * v197);
        let v200: f64 = (v7 * self.scalar_v199);
        let v201: f64 = (v44 + v200);
        let v202: f64 = { let limexp_arg = v162; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v203: f64 = (v112 * v202);
        let v204: f64 = (v201 + v203);
        let v205: f64 = (v198 * v204);
        let v206: f64 = (if self.scalar_v192 { v205 } else { v13 });
        let v209: f64 = (v5 - v168);
        let v210: f64 = (if self.scalar_v208 { v209 } else { v133 });
        let v211: f64 = (v210 * v210);
        let v212: f64 = (if self.scalar_v208 { v211 } else { v169 });
        let v213: f64 = (v210 * v212);
        let v214: f64 = (if self.scalar_v208 { v213 } else { v170 });
        let v215: f64 = (v147 * v210);
        let v216: f64 = (self.scalar_v172 * v212);
        let v217: f64 = (v215 + v216);
        let v218: f64 = (v152 * v214);
        let v219: f64 = (v217 + v218);
        let v220: f64 = (if self.scalar_v208 { v219 } else { v13 });
        let v221: f64 = ((v220) as f64).tanh();
        let v222: f64 = (v44 + v221);
        let v223: f64 = (if self.scalar_v208 { v222 } else { v13 });
        let v224: f64 = (self.scalar_v155 * v223);
        let v225: f64 = (self.scalar_v187 + v224);
        let v226: f64 = (if self.scalar_v208 { v225 } else { v13 });
        let v228: f64 = (v179 * self.scalar_v227);
        let v229: f64 = (self.scalar_v199 + v228);
        let v230: f64 = (if self.scalar_v208 { v229 } else { v13 });
        let v231: f64 = (v44 + v191);
        let v232: f64 = (v197 * v231);
        let v233: f64 = (v7 * v230);
        let v234: f64 = (v44 + v233);
        let v236: f64 = (v7 - v118);
        let v237: f64 = (self.scalar_v235 * v236);
        let v238: f64 = { let limexp_arg = v237; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v239: f64 = (v112 * v238);
        let v240: f64 = (v234 + v239);
        let v241: f64 = (v232 * v240);
        let v242: f64 = (if self.scalar_v208 { v241 } else { v13 });
        let v243: f64 = (v223 * self.scalar_v227);
        let v244: f64 = (self.scalar_v199 + v243);
        let v245: f64 = (if self.scalar_v208 { v244 } else { v13 });
        let v246: f64 = (v7 * v226);
        let v247: f64 = ((v246) as f64).tanh();
        let v248: f64 = (if self.scalar_v208 { v247 } else { v13 });
        let v249: f64 = (v111 * v223);
        let v250: f64 = (v44 - v248);
        let v251: f64 = (v249 * v250);
        let v252: f64 = (v7 * v245);
        let v253: f64 = (v44 - v252);
        let v254: f64 = (v251 * v253);
        let v255: f64 = (if self.scalar_v208 { v254 } else { v13 });
        let v256: f64 = (v242 - v255);
        let v257: f64 = (v123 * v256);
        let v258: f64 = (if self.scalar_v208 { v257 } else { v206 });
        let v262: f64 = (if self.scalar_v261 { v169 } else { v210 });
        let v263: f64 = (v262 * v262);
        let v264: f64 = (if self.scalar_v261 { v263 } else { v212 });
        let v265: f64 = (self.scalar_v172 * v264);
        let v266: f64 = (v262 + v265);
        let v267: f64 = (v152 * v264);
        let v268: f64 = (v262 * v267);
        let v269: f64 = (v266 + v268);
        let v270: f64 = (v147 * v269);
        let v271: f64 = (if self.scalar_v261 { v270 } else { v177 });
        let v272: f64 = { let limexp_arg = v271; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v273: f64 = (-v271);
        let v274: f64 = { let limexp_arg = v273; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v275: f64 = (v272 - v274);
        let v276: f64 = (v123 * v275);
        let v277: f64 = ((v276) as f64).tanh();
        let v278: f64 = (v44 + v277);
        let v279: f64 = (if self.scalar_v261 { v278 } else { v186 });
        let v280: f64 = (self.scalar_v155 * v279);
        let v281: f64 = (self.scalar_v187 + v280);
        let v282: f64 = (if self.scalar_v261 { v281 } else { v13 });
        let v283: f64 = (v7 * v282);
        let v284: f64 = ((v283) as f64).tanh();
        let v285: f64 = (if self.scalar_v261 { v284 } else { v13 });
        let v286: f64 = (self.scalar_v227 * v279);
        let v287: f64 = (self.scalar_v199 + v286);
        let v288: f64 = (if self.scalar_v261 { v287 } else { v230 });
        let v289: f64 = (v111 * v279);
        let v290: f64 = (v285 * v289);
        let v291: f64 = (v7 * v288);
        let v292: f64 = (v44 + v291);
        let v293: f64 = (v162 * self.scalar_v235);
        let v294: f64 = { let limexp_arg = v293; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v295: f64 = (v112 * v294);
        let v296: f64 = (v292 + v295);
        let v297: f64 = (v290 * v296);
        let v298: f64 = (if self.scalar_v261 { v297 } else { v258 });
        let v302: f64 = (if self.scalar_v301 { v169 } else { v262 });
        let v303: f64 = (v302 * v302);
        let v304: f64 = (if self.scalar_v301 { v303 } else { v264 });
        let v305: f64 = (self.scalar_v172 * v304);
        let v306: f64 = (v302 + v305);
        let v307: f64 = (v152 * v304);
        let v308: f64 = (v302 * v307);
        let v309: f64 = (v306 + v308);
        let v310: f64 = (v147 * v309);
        let v311: f64 = (if self.scalar_v301 { v310 } else { v271 });
        let v312: f64 = (if self.scalar_v301 { v209 } else { v214 });
        let v313: f64 = (v312 * v312);
        let v314: f64 = (if self.scalar_v301 { v313 } else { v13 });
        let v315: f64 = (self.scalar_v172 * v314);
        let v316: f64 = (v312 + v315);
        let v317: f64 = (v152 * v312);
        let v318: f64 = (v314 * v317);
        let v319: f64 = (v316 + v318);
        let v320: f64 = (v147 * v319);
        let v321: f64 = (if self.scalar_v301 { v320 } else { v220 });
        let v322: f64 = { let limexp_arg = v311; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v323: f64 = (-v311);
        let v324: f64 = { let limexp_arg = v323; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v325: f64 = (v322 - v324);
        let v326: f64 = (v123 * v325);
        let v327: f64 = ((v326) as f64).tanh();
        let v328: f64 = (v44 + v327);
        let v329: f64 = (if self.scalar_v301 { v328 } else { v279 });
        let v330: f64 = { let limexp_arg = v321; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v331: f64 = (-v321);
        let v332: f64 = { let limexp_arg = v331; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v333: f64 = (v330 - v332);
        let v334: f64 = (v123 * v333);
        let v335: f64 = ((v334) as f64).tanh();
        let v336: f64 = (v44 + v335);
        let v337: f64 = (if self.scalar_v301 { v336 } else { v13 });
        let v338: f64 = (self.scalar_v155 * v329);
        let v339: f64 = (self.scalar_v187 + v338);
        let v340: f64 = (if self.scalar_v301 { v339 } else { v282 });
        let v341: f64 = (self.scalar_v155 * v337);
        let v342: f64 = (self.scalar_v187 + v341);
        let v343: f64 = (if self.scalar_v301 { v342 } else { v13 });
        let v344: f64 = (v7 * v340);
        let v345: f64 = ((v344) as f64).tanh();
        let v346: f64 = (if self.scalar_v301 { v345 } else { v285 });
        let v347: f64 = (v7 * v343);
        let v348: f64 = ((v347) as f64).tanh();
        let v349: f64 = (if self.scalar_v301 { v348 } else { v13 });
        let v350: f64 = (self.scalar_v227 * v337);
        let v351: f64 = (self.scalar_v199 + v350);
        let v352: f64 = (if self.scalar_v301 { v351 } else { v13 });
        let v353: f64 = (self.scalar_v227 * v329);
        let v354: f64 = (self.scalar_v199 + v353);
        let v355: f64 = (if self.scalar_v301 { v354 } else { v13 });
        let v356: f64 = (v111 * v329);
        let v357: f64 = (v44 + v346);
        let v358: f64 = (v356 * v357);
        let v359: f64 = (v7 * v355);
        let v360: f64 = (v44 + v359);
        let v361: f64 = (v239 + v360);
        let v362: f64 = (v358 * v361);
        let v363: f64 = (if self.scalar_v301 { v362 } else { v242 });
        let v364: f64 = (v111 * v337);
        let v365: f64 = (v44 - v349);
        let v366: f64 = (v364 * v365);
        let v367: f64 = (v7 * v352);
        let v368: f64 = (v44 - v367);
        let v369: f64 = (v366 * v368);
        let v370: f64 = (if self.scalar_v301 { v369 } else { v255 });
        let v371: f64 = (v363 - v370);
        let v372: f64 = (v123 * v371);
        let v373: f64 = (if self.scalar_v301 { v372 } else { v298 });
        let v377: f64 = (if self.scalar_v376 { v229 } else { v288 });
        let v378: f64 = (if self.scalar_v376 { v339 } else { v340 });
        let v379: f64 = (v7 * v378);
        let v380: f64 = ((v379) as f64).tanh();
        let v381: f64 = (if self.scalar_v376 { v380 } else { v346 });
        let v382: f64 = (v11 * v378);
        let v383: f64 = ((v382) as f64).tanh();
        let v384: f64 = (if self.scalar_v376 { v383 } else { v13 });
        let v386: f64 = (v384 * self.scalar_v385);
        let v387: f64 = (v381 + v386);
        let v388: f64 = (v197 * v387);
        let v389: f64 = (v11 * self.scalar_v385);
        let v390: f64 = (v7 + v389);
        let v391: f64 = (v377 * v390);
        let v392: f64 = (v44 + v391);
        let v393: f64 = (v239 + v392);
        let v394: f64 = (v388 * v393);
        let v395: f64 = (if self.scalar_v376 { v394 } else { v373 });
        let v401: f64 = -1.0;
        let v402: f64 = (-v117);
        let v403: f64 = ((v402) as f64).tanh();
        let v404: f64 = (v130 * v403);
        let v405: f64 = { let limexp_arg = v404; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v406: f64 = (if self.scalar_v400 { v405 } else { v302 });
        let v407: f64 = (v9 - v117);
        let v408: f64 = (if self.scalar_v400 { v407 } else { v13 });
        let v409: f64 = (-v9);
        let v411: f64 = (v409 - self.scalar_v410);
        let v412: f64 = (if self.scalar_v400 { v411 } else { v13 });
        let v413: f64 = (v5 - v117);
        let v414: f64 = (if self.scalar_v400 { v413 } else { v13 });
        let v416: f64 = (v6 - self.scalar_v415);
        let v417: f64 = (if self.scalar_v400 { v416 } else { v13 });
        let v419: f64 = (-v130);
        let v420: f64 = (v117 * v419);
        let v421: f64 = { let limexp_arg = v420; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v422: f64 = (if self.scalar_v418 { v421 } else { v406 });
        let v433: f64 = ((v407) as f64).tanh();
        let v434: f64 = (if self.scalar_v432 { v433 } else { v408 });
        let v435: f64 = ((v413) as f64).tanh();
        let v436: f64 = (if self.scalar_v432 { v435 } else { v414 });
        let v439: f64 = (if self.scalar_v438 { v407 } else { v434 });
        let v440: f64 = (if self.scalar_v438 { v413 } else { v436 });
        let v441: f64 = (if self.scalar_v418 { v411 } else { v412 });
        let v442: f64 = (if self.scalar_v418 { v416 } else { v417 });
        let v443: f64 = (self.scalar_v423 * v441);
        let v444: f64 = { let limexp_arg = v443; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v445: f64 = (v444 - self.scalar_v427);
        let v447: f64 = (v130 * v439);
        let v448: f64 = { let limexp_arg = v447; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v452: f64 = (v445 * self.scalar_v451);
        let v453: f64 = (v448 - v452);
        let v454: f64 = (v453 - v422);
        let v455: f64 = (self.scalar_v446 * v454);
        let v456: f64 = (self.scalar_v423 * v442);
        let v457: f64 = { let limexp_arg = v456; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v458: f64 = (v457 - self.scalar_v430);
        let v459: f64 = (v130 * v440);
        let v460: f64 = { let limexp_arg = v459; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v461: f64 = (self.scalar_v451 * v458);
        let v462: f64 = (v460 - v461);
        let v463: f64 = (v462 - v422);
        let v464: f64 = (self.scalar_v446 * v463);
        let v488: f64 = 5.5226012e-23;
        let v489: f64 = (v35 * v488);
        let v493: f64 = (v489 * self.scalar_v492);
        let v494: f64 = (v113 * v493);
        let v497: f64 = (v494 * self.scalar_v496);
        let v498: f64 = (if self.scalar_v487 { v497 } else { v13 });
        let v499: f64 = (v498 * v498);
        let v500: f64 = (v44 - v499);
        let v501: f64 = ((v500) as f64).sqrt();
        let v502: f64 = (if self.scalar_v487 { v501 } else { v13 });
        let v503: f64 = (-v498);
        let v504: f64 = 3.141592653589793;
        let v505: f64 = (v503 * v504);
        let v506: f64 = (if self.scalar_v487 { v505 } else { v13 });
        let v510: f64 = (-v395);
        let v512: f64 = nv15;
        let v513: f64 = (self.scalar_v511 * v512);
        let v515: f64 = nv7;
        let v516: f64 = (v515 - v4);
        let v517: f64 = (self.scalar_v514 * v516);
        let v519: f64 = (v7 * self.scalar_v518);
        let v520: f64 = nv6;
        let v521: f64 = (v520 - v10);
        let v522: f64 = (v114 * v521);
        let v523: f64 = (v138 * v521);
        let v526: f64 = (v8 - v0);
        let v527: f64 = (v526 / v115);
        let v528: f64 = (if self.scalar_v468 { v527 } else { v13 });
        let v532: f64 = nv14;
        let v533: f64 = (v8 - v532);
        let v534: f64 = (self.scalar_v531 * v533);
        let v535: f64 = (v532 - v1);
        let v536: f64 = (v535 / self.scalar_v469);
        let v537: f64 = (if self.scalar_v470 { v536 } else { v13 });
        let v540: f64 = nv13;
        let v541: f64 = (v540 - v3);
        let v542: f64 = (v541 / self.scalar_v471);
        let v543: f64 = (if self.scalar_v472 { v542 } else { v13 });
        let v547: f64 = (v540 - v8);
        let v548: f64 = (v547 / self.scalar_v473);
        let v549: f64 = (if self.scalar_v474 { v548 } else { v13 });
        let v564: f64 = 1e-15;
        let v565: f64 = nv2;
        let v566: f64 = (v0 - v565);
        let v567: f64 = (v138 * v566);
        let v571: f64 = nv17;
        let v572: f64 = (if self.scalar_v487 { v571 } else { v13 });
        let v573: f64 = nv18;
        let v574: f64 = (if self.scalar_v487 { v573 } else { v13 });
        let v575: f64 = (v506 * v571);
        let v576: f64 = (v502 * v573);
        let v577: f64 = (v575 + v576);
        let v578: f64 = (if self.scalar_v487 { v577 } else { v13 });
        let v583: f64 = (v7 * v395);
        let v584: f64 = ((v583) as f64).abs();
        let v585: f64 = (v9 * v455);
        let v586: f64 = ((v585) as f64).abs();
        let v587: f64 = (v584 + v586);
        let v588: f64 = (-v587);
        let v589: f64 = (if self.scalar_v509 { v588 } else { v13 });
        let v590: f64 = (v32 / v49);
        let v591: f64 = (if self.scalar_v509 { v590 } else { v13 });
        let v593: f64 = (v32 * v138);
        let v594: f64 = (if self.scalar_v592 { v593 } else { v13 });
        let v596: f64 = ((v132) as f64).sinh();
        let v597: f64 = (self.scalar_v131 * v596);
        let v598: f64 = (self.scalar_v595 * v596);
        let v600: f64 = (v133 * v597);
        let v601: f64 = (v600 + v600);
        let v602: f64 = (v133 * v598);
        let v603: f64 = (v602 + v602);
        let v604: f64 = (self.scalar_v137 * v601);
        let v605: f64 = (-v604);
        let v606: f64 = (v140 * v140);
        let v607: f64 = (v605 / v606);
        let v608: f64 = (self.scalar_v137 * v603);
        let v609: f64 = (-v608);
        let v610: f64 = (v609 / v606);
        let v611: f64 = (self.scalar_v136 * v607);
        let v612: f64 = (self.scalar_v136 * v610);
        let v613: f64 = (v146 * v611);
        let v614: f64 = (v146 * v612);
        let v616: f64 = (v157 * v157);
        let v617: f64 = (v44 - v616);
        let v618: f64 = (self.scalar_v155 * v617);
        let v619: f64 = (self.scalar_v615 * v617);
        let v620: f64 = (self.scalar_v153 * v618);
        let v621: f64 = (self.scalar_v153 * v619);
        let v622: f64 = (v621 - self.scalar_v599);
        let v624: f64 = (v163 + v163);
        let v625: f64 = (-v163);
        let v626: f64 = (v162 * self.scalar_v623);
        let v627: f64 = (v625 + v626);
        let v628: f64 = (v620 - v624);
        let v629: f64 = (-v627);
        let v630: f64 = (v167 * self.scalar_v599);
        let v631: f64 = (v167 * v628);
        let v632: f64 = (v167 * v622);
        let v633: f64 = (v167 * v629);
        let v634: f64 = (-v630);
        let v635: f64 = (-v631);
        let v636: f64 = (v401 - v632);
        let v637: f64 = (-v633);
        let v638: f64 = (v169 * v634);
        let v639: f64 = (v638 + v638);
        let v640: f64 = (v169 * v635);
        let v641: f64 = (v640 + v640);
        let v642: f64 = (v169 * v636);
        let v643: f64 = (v642 + v642);
        let v644: f64 = (v169 * v637);
        let v645: f64 = (v644 + v644);
        let v646: f64 = (v169 + v169);
        let v647: f64 = (v147 * v634);
        let v648: f64 = (v169 * v613);
        let v649: f64 = (v147 * v635);
        let v650: f64 = (v648 + v649);
        let v651: f64 = (v169 * v614);
        let v652: f64 = (v147 * v636);
        let v653: f64 = (v651 + v652);
        let v654: f64 = (v147 * v637);
        let v655: f64 = (self.scalar_v172 * v639);
        let v656: f64 = (self.scalar_v172 * v641);
        let v657: f64 = (self.scalar_v172 * v643);
        let v658: f64 = (self.scalar_v172 * v645);
        let v659: f64 = (self.scalar_v172 * v646);
        let v660: f64 = (v647 + v655);
        let v661: f64 = (v650 + v656);
        let v662: f64 = (v653 + v657);
        let v663: f64 = (v654 + v658);
        let v664: f64 = (v147 + v659);
        let v665: f64 = (v152 * v634);
        let v666: f64 = (v152 * v635);
        let v667: f64 = (v152 * v636);
        let v668: f64 = (v152 * v637);
        let v669: f64 = (v175 * v639);
        let v670: f64 = (v170 * v665);
        let v671: f64 = (v669 + v670);
        let v672: f64 = (v175 * v641);
        let v673: f64 = (v170 * v666);
        let v674: f64 = (v672 + v673);
        let v675: f64 = (v175 * v643);
        let v676: f64 = (v170 * v667);
        let v677: f64 = (v675 + v676);
        let v678: f64 = (v175 * v645);
        let v679: f64 = (v170 * v668);
        let v680: f64 = (v678 + v679);
        let v681: f64 = (v175 * v646);
        let v682: f64 = (v152 * v170);
        let v683: f64 = (v681 + v682);
        let v684: f64 = (v660 + v671);
        let v685: f64 = (v661 + v674);
        let v686: f64 = (v662 + v677);
        let v687: f64 = (v663 + v680);
        let v688: f64 = (v664 + v683);
        let v689: f64 = (v178 * v178);
        let v690: f64 = (v44 - v689);
        let v691: f64 = (v684 * v690);
        let v692: f64 = (v685 * v690);
        let v693: f64 = (v686 * v690);
        let v694: f64 = (v687 * v690);
        let v695: f64 = (v688 * v690);
        let v696: f64 = { let limexp_arg = v177; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v697: f64 = (v684 * v696);
        let v698: f64 = (v685 * v696);
        let v699: f64 = (v686 * v696);
        let v700: f64 = (v687 * v696);
        let v701: f64 = (v688 * v696);
        let v702: f64 = (-v684);
        let v703: f64 = (-v685);
        let v704: f64 = (-v686);
        let v705: f64 = (-v687);
        let v706: f64 = (-v688);
        let v707: f64 = { let limexp_arg = v181; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v708: f64 = (v702 * v707);
        let v709: f64 = (v703 * v707);
        let v710: f64 = (v704 * v707);
        let v711: f64 = (v705 * v707);
        let v712: f64 = (v706 * v707);
        let v713: f64 = (v697 - v708);
        let v714: f64 = (v698 - v709);
        let v715: f64 = (v699 - v710);
        let v716: f64 = (v700 - v711);
        let v717: f64 = (v701 - v712);
        let v718: f64 = (v123 * v713);
        let v719: f64 = (v123 * v714);
        let v720: f64 = (v123 * v715);
        let v721: f64 = (v123 * v716);
        let v722: f64 = (v123 * v717);
        let v723: f64 = (v185 * v185);
        let v724: f64 = (v44 - v723);
        let v725: f64 = (v718 * v724);
        let v726: f64 = (v719 * v724);
        let v727: f64 = (v720 * v724);
        let v728: f64 = (v721 * v724);
        let v729: f64 = (v722 * v724);
        let v730: f64 = (self.scalar_v155 * v691);
        let v731: f64 = (self.scalar_v155 * v692);
        let v732: f64 = (self.scalar_v155 * v693);
        let v733: f64 = (self.scalar_v155 * v694);
        let v734: f64 = (self.scalar_v155 * v695);
        let v735: f64 = (v7 * v730);
        let v736: f64 = (v7 * v731);
        let v737: f64 = (v189 + v736);
        let v738: f64 = (-v189);
        let v739: f64 = (v7 * v732);
        let v740: f64 = (v738 + v739);
        let v741: f64 = (v7 * v733);
        let v742: f64 = (v7 * v734);
        let v743: f64 = (v191 * v191);
        let v744: f64 = (v44 - v743);
        let v745: f64 = (v735 * v744);
        let v746: f64 = (v737 * v744);
        let v747: f64 = (v740 * v744);
        let v748: f64 = (v741 * v744);
        let v749: f64 = (v742 * v744);
        let v750: f64 = (v111 * v691);
        let v751: f64 = (v111 * v692);
        let v752: f64 = (v111 * v693);
        let v753: f64 = (v111 * v694);
        let v754: f64 = (v111 * v695);
        let v755: f64 = (v197 * v745);
        let v756: f64 = (v191 * v750);
        let v757: f64 = (v755 + v756);
        let v758: f64 = (v197 * v746);
        let v759: f64 = (v191 * v751);
        let v760: f64 = (v758 + v759);
        let v761: f64 = (v197 * v747);
        let v762: f64 = (v191 * v752);
        let v763: f64 = (v761 + v762);
        let v764: f64 = (v197 * v748);
        let v765: f64 = (v191 * v753);
        let v766: f64 = (v764 + v765);
        let v767: f64 = (v197 * v749);
        let v768: f64 = (v191 * v754);
        let v769: f64 = (v767 + v768);
        let v771: f64 = { let limexp_arg = v162; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v772: f64 = (-v771);
        let v773: f64 = (v112 * v771);
        let v774: f64 = (v112 * v772);
        let v775: f64 = (self.scalar_v199 + v773);
        let v776: f64 = (v204 * v757);
        let v777: f64 = (v204 * v760);
        let v778: f64 = (v198 * v775);
        let v779: f64 = (v777 + v778);
        let v780: f64 = (v204 * v763);
        let v781: f64 = (v198 * self.scalar_v770);
        let v782: f64 = (v780 + v781);
        let v783: f64 = (v204 * v766);
        let v784: f64 = (v198 * v774);
        let v785: f64 = (v783 + v784);
        let v786: f64 = (v204 * v769);
        let v787: f64 = (if self.scalar_v192 { v776 } else { v13 });
        let v788: f64 = (if self.scalar_v192 { v779 } else { v13 });
        let v789: f64 = (if self.scalar_v192 { v782 } else { v13 });
        let v790: f64 = (if self.scalar_v192 { v785 } else { v13 });
        let v791: f64 = (if self.scalar_v192 { v786 } else { v13 });
        let v792: f64 = (v401 - v631);
        let v793: f64 = (-v632);
        let v794: f64 = (v44 - v633);
        let v795: f64 = (if self.scalar_v208 { v634 } else { v13 });
        let v796: f64 = (if self.scalar_v208 { v792 } else { v597 });
        let v797: f64 = (if self.scalar_v208 { v793 } else { v598 });
        let v798: f64 = (if self.scalar_v208 { v794 } else { v13 });
        let v799: f64 = (v210 * v795);
        let v800: f64 = (v799 + v799);
        let v801: f64 = (v210 * v796);
        let v802: f64 = (v801 + v801);
        let v803: f64 = (v210 * v797);
        let v804: f64 = (v803 + v803);
        let v805: f64 = (v210 * v798);
        let v806: f64 = (v805 + v805);
        let v807: f64 = (if self.scalar_v208 { v800 } else { v634 });
        let v808: f64 = (if self.scalar_v208 { v802 } else { v635 });
        let v809: f64 = (if self.scalar_v208 { v804 } else { v636 });
        let v810: f64 = (if self.scalar_v208 { v806 } else { v637 });
        let v812: f64 = (v212 * v795);
        let v813: f64 = (v210 * v807);
        let v814: f64 = (v812 + v813);
        let v815: f64 = (v212 * v796);
        let v816: f64 = (v210 * v808);
        let v817: f64 = (v815 + v816);
        let v818: f64 = (v212 * v797);
        let v819: f64 = (v210 * v809);
        let v820: f64 = (v818 + v819);
        let v821: f64 = (v212 * v798);
        let v822: f64 = (v210 * v810);
        let v823: f64 = (v821 + v822);
        let v824: f64 = (v210 * self.scalar_v811);
        let v825: f64 = (if self.scalar_v208 { v814 } else { v639 });
        let v826: f64 = (if self.scalar_v208 { v817 } else { v641 });
        let v827: f64 = (if self.scalar_v208 { v820 } else { v643 });
        let v828: f64 = (if self.scalar_v208 { v823 } else { v645 });
        let v829: f64 = (if self.scalar_v208 { v824 } else { v646 });
        let v830: f64 = (v147 * v795);
        let v831: f64 = (v210 * v613);
        let v832: f64 = (v147 * v796);
        let v833: f64 = (v831 + v832);
        let v834: f64 = (v210 * v614);
        let v835: f64 = (v147 * v797);
        let v836: f64 = (v834 + v835);
        let v837: f64 = (v147 * v798);
        let v838: f64 = (self.scalar_v172 * v807);
        let v839: f64 = (self.scalar_v172 * v808);
        let v840: f64 = (self.scalar_v172 * v809);
        let v841: f64 = (self.scalar_v172 * v810);
        let v843: f64 = (v830 + v838);
        let v844: f64 = (v833 + v839);
        let v845: f64 = (v836 + v840);
        let v846: f64 = (v837 + v841);
        let v847: f64 = (v152 * v825);
        let v848: f64 = (v152 * v826);
        let v849: f64 = (v152 * v827);
        let v850: f64 = (v152 * v828);
        let v851: f64 = (v152 * v829);
        let v852: f64 = (v843 + v847);
        let v853: f64 = (v844 + v848);
        let v854: f64 = (v845 + v849);
        let v855: f64 = (v846 + v850);
        let v856: f64 = (self.scalar_v842 + v851);
        let v857: f64 = (if self.scalar_v208 { v852 } else { v13 });
        let v858: f64 = (if self.scalar_v208 { v853 } else { v13 });
        let v859: f64 = (if self.scalar_v208 { v854 } else { v13 });
        let v860: f64 = (if self.scalar_v208 { v855 } else { v13 });
        let v861: f64 = (if self.scalar_v208 { v856 } else { v13 });
        let v862: f64 = (v221 * v221);
        let v863: f64 = (v44 - v862);
        let v864: f64 = (v857 * v863);
        let v865: f64 = (v858 * v863);
        let v866: f64 = (v859 * v863);
        let v867: f64 = (v860 * v863);
        let v868: f64 = (v861 * v863);
        let v869: f64 = (if self.scalar_v208 { v864 } else { v13 });
        let v870: f64 = (if self.scalar_v208 { v865 } else { v13 });
        let v871: f64 = (if self.scalar_v208 { v866 } else { v13 });
        let v872: f64 = (if self.scalar_v208 { v867 } else { v13 });
        let v873: f64 = (if self.scalar_v208 { v868 } else { v13 });
        let v874: f64 = (self.scalar_v155 * v869);
        let v875: f64 = (self.scalar_v155 * v870);
        let v876: f64 = (self.scalar_v155 * v871);
        let v877: f64 = (self.scalar_v155 * v872);
        let v878: f64 = (self.scalar_v155 * v873);
        let v879: f64 = (if self.scalar_v208 { v874 } else { v13 });
        let v880: f64 = (if self.scalar_v208 { v875 } else { v13 });
        let v881: f64 = (if self.scalar_v208 { v876 } else { v13 });
        let v882: f64 = (if self.scalar_v208 { v877 } else { v13 });
        let v883: f64 = (if self.scalar_v208 { v878 } else { v13 });
        let v884: f64 = (self.scalar_v227 * v691);
        let v885: f64 = (self.scalar_v227 * v692);
        let v886: f64 = (self.scalar_v227 * v693);
        let v887: f64 = (self.scalar_v227 * v694);
        let v888: f64 = (self.scalar_v227 * v695);
        let v889: f64 = (if self.scalar_v208 { v884 } else { v13 });
        let v890: f64 = (if self.scalar_v208 { v885 } else { v13 });
        let v891: f64 = (if self.scalar_v208 { v886 } else { v13 });
        let v892: f64 = (if self.scalar_v208 { v887 } else { v13 });
        let v893: f64 = (if self.scalar_v208 { v888 } else { v13 });
        let v894: f64 = (v231 * v750);
        let v895: f64 = (v755 + v894);
        let v896: f64 = (v231 * v751);
        let v897: f64 = (v758 + v896);
        let v898: f64 = (v231 * v752);
        let v899: f64 = (v761 + v898);
        let v900: f64 = (v231 * v753);
        let v901: f64 = (v764 + v900);
        let v902: f64 = (v231 * v754);
        let v903: f64 = (v767 + v902);
        let v904: f64 = (v7 * v889);
        let v905: f64 = (v7 * v890);
        let v906: f64 = (v230 + v905);
        let v907: f64 = (-v230);
        let v908: f64 = (v7 * v891);
        let v909: f64 = (v907 + v908);
        let v910: f64 = (v7 * v892);
        let v911: f64 = (v7 * v893);
        let v913: f64 = { let limexp_arg = v237; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v914: f64 = (self.scalar_v235 * v913);
        let v915: f64 = (self.scalar_v912 * v913);
        let v916: f64 = (v112 * v914);
        let v917: f64 = (v112 * v915);
        let v918: f64 = (v906 + v916);
        let v919: f64 = (v909 + v917);
        let v920: f64 = (v240 * v895);
        let v921: f64 = (v232 * v904);
        let v922: f64 = (v920 + v921);
        let v923: f64 = (v240 * v897);
        let v924: f64 = (v232 * v918);
        let v925: f64 = (v923 + v924);
        let v926: f64 = (v240 * v899);
        let v927: f64 = (v232 * v919);
        let v928: f64 = (v926 + v927);
        let v929: f64 = (v240 * v901);
        let v930: f64 = (v232 * v910);
        let v931: f64 = (v929 + v930);
        let v932: f64 = (v240 * v903);
        let v933: f64 = (v232 * v911);
        let v934: f64 = (v932 + v933);
        let v935: f64 = (if self.scalar_v208 { v922 } else { v13 });
        let v936: f64 = (if self.scalar_v208 { v925 } else { v13 });
        let v937: f64 = (if self.scalar_v208 { v928 } else { v13 });
        let v938: f64 = (if self.scalar_v208 { v931 } else { v13 });
        let v939: f64 = (if self.scalar_v208 { v934 } else { v13 });
        let v940: f64 = (self.scalar_v227 * v869);
        let v941: f64 = (self.scalar_v227 * v870);
        let v942: f64 = (self.scalar_v227 * v871);
        let v943: f64 = (self.scalar_v227 * v872);
        let v944: f64 = (self.scalar_v227 * v873);
        let v945: f64 = (if self.scalar_v208 { v940 } else { v13 });
        let v946: f64 = (if self.scalar_v208 { v941 } else { v13 });
        let v947: f64 = (if self.scalar_v208 { v942 } else { v13 });
        let v948: f64 = (if self.scalar_v208 { v943 } else { v13 });
        let v949: f64 = (if self.scalar_v208 { v944 } else { v13 });
        let v950: f64 = (v7 * v879);
        let v951: f64 = (v7 * v880);
        let v952: f64 = (v226 + v951);
        let v953: f64 = (-v226);
        let v954: f64 = (v7 * v881);
        let v955: f64 = (v953 + v954);
        let v956: f64 = (v7 * v882);
        let v957: f64 = (v7 * v883);
        let v958: f64 = (v247 * v247);
        let v959: f64 = (v44 - v958);
        let v960: f64 = (v950 * v959);
        let v961: f64 = (v952 * v959);
        let v962: f64 = (v955 * v959);
        let v963: f64 = (v956 * v959);
        let v964: f64 = (v957 * v959);
        let v965: f64 = (if self.scalar_v208 { v960 } else { v13 });
        let v966: f64 = (if self.scalar_v208 { v961 } else { v13 });
        let v967: f64 = (if self.scalar_v208 { v962 } else { v13 });
        let v968: f64 = (if self.scalar_v208 { v963 } else { v13 });
        let v969: f64 = (if self.scalar_v208 { v964 } else { v13 });
        let v970: f64 = (v111 * v869);
        let v971: f64 = (v111 * v870);
        let v972: f64 = (v111 * v871);
        let v973: f64 = (v111 * v872);
        let v974: f64 = (v111 * v873);
        let v975: f64 = (-v965);
        let v976: f64 = (-v966);
        let v977: f64 = (-v967);
        let v978: f64 = (-v968);
        let v979: f64 = (-v969);
        let v980: f64 = (v250 * v970);
        let v981: f64 = (v249 * v975);
        let v982: f64 = (v980 + v981);
        let v983: f64 = (v250 * v971);
        let v984: f64 = (v249 * v976);
        let v985: f64 = (v983 + v984);
        let v986: f64 = (v250 * v972);
        let v987: f64 = (v249 * v977);
        let v988: f64 = (v986 + v987);
        let v989: f64 = (v250 * v973);
        let v990: f64 = (v249 * v978);
        let v991: f64 = (v989 + v990);
        let v992: f64 = (v250 * v974);
        let v993: f64 = (v249 * v979);
        let v994: f64 = (v992 + v993);
        let v995: f64 = (v7 * v945);
        let v996: f64 = (v7 * v946);
        let v997: f64 = (v245 + v996);
        let v998: f64 = (-v245);
        let v999: f64 = (v7 * v947);
        let v1000: f64 = (v998 + v999);
        let v1001: f64 = (v7 * v948);
        let v1002: f64 = (v7 * v949);
        let v1003: f64 = (-v995);
        let v1004: f64 = (-v997);
        let v1005: f64 = (-v1000);
        let v1006: f64 = (-v1001);
        let v1007: f64 = (-v1002);
        let v1008: f64 = (v253 * v982);
        let v1009: f64 = (v251 * v1003);
        let v1010: f64 = (v1008 + v1009);
        let v1011: f64 = (v253 * v985);
        let v1012: f64 = (v251 * v1004);
        let v1013: f64 = (v1011 + v1012);
        let v1014: f64 = (v253 * v988);
        let v1015: f64 = (v251 * v1005);
        let v1016: f64 = (v1014 + v1015);
        let v1017: f64 = (v253 * v991);
        let v1018: f64 = (v251 * v1006);
        let v1019: f64 = (v1017 + v1018);
        let v1020: f64 = (v253 * v994);
        let v1021: f64 = (v251 * v1007);
        let v1022: f64 = (v1020 + v1021);
        let v1023: f64 = (if self.scalar_v208 { v1010 } else { v13 });
        let v1024: f64 = (if self.scalar_v208 { v1013 } else { v13 });
        let v1025: f64 = (if self.scalar_v208 { v1016 } else { v13 });
        let v1026: f64 = (if self.scalar_v208 { v1019 } else { v13 });
        let v1027: f64 = (if self.scalar_v208 { v1022 } else { v13 });
        let v1028: f64 = (v935 - v1023);
        let v1029: f64 = (v936 - v1024);
        let v1030: f64 = (v937 - v1025);
        let v1031: f64 = (v938 - v1026);
        let v1032: f64 = (v939 - v1027);
        let v1033: f64 = (v123 * v1028);
        let v1034: f64 = (v123 * v1029);
        let v1035: f64 = (v123 * v1030);
        let v1036: f64 = (v123 * v1031);
        let v1037: f64 = (v123 * v1032);
        let v1038: f64 = (if self.scalar_v208 { v1033 } else { v787 });
        let v1039: f64 = (if self.scalar_v208 { v1034 } else { v788 });
        let v1040: f64 = (if self.scalar_v208 { v1035 } else { v789 });
        let v1041: f64 = (if self.scalar_v208 { v1036 } else { v790 });
        let v1042: f64 = (if self.scalar_v208 { v1037 } else { v791 });
        let v1043: f64 = (if self.scalar_v261 { v634 } else { v795 });
        let v1044: f64 = (if self.scalar_v261 { v635 } else { v796 });
        let v1045: f64 = (if self.scalar_v261 { v636 } else { v797 });
        let v1046: f64 = (if self.scalar_v261 { v637 } else { v798 });
        let v1048: f64 = (v262 * v1043);
        let v1049: f64 = (v1048 + v1048);
        let v1050: f64 = (v262 * v1044);
        let v1051: f64 = (v1050 + v1050);
        let v1052: f64 = (v262 * v1045);
        let v1053: f64 = (v1052 + v1052);
        let v1054: f64 = (v262 * v1046);
        let v1055: f64 = (v1054 + v1054);
        let v1056: f64 = (v262 * self.scalar_v1047);
        let v1057: f64 = (v1056 + v1056);
        let v1058: f64 = (if self.scalar_v261 { v1049 } else { v807 });
        let v1059: f64 = (if self.scalar_v261 { v1051 } else { v808 });
        let v1060: f64 = (if self.scalar_v261 { v1053 } else { v809 });
        let v1061: f64 = (if self.scalar_v261 { v1055 } else { v810 });
        let v1062: f64 = (if self.scalar_v261 { v1057 } else { self.scalar_v811 });
        let v1063: f64 = (self.scalar_v172 * v1058);
        let v1064: f64 = (self.scalar_v172 * v1059);
        let v1065: f64 = (self.scalar_v172 * v1060);
        let v1066: f64 = (self.scalar_v172 * v1061);
        let v1067: f64 = (self.scalar_v172 * v1062);
        let v1068: f64 = (v1043 + v1063);
        let v1069: f64 = (v1044 + v1064);
        let v1070: f64 = (v1045 + v1065);
        let v1071: f64 = (v1046 + v1066);
        let v1072: f64 = (self.scalar_v1047 + v1067);
        let v1073: f64 = (v152 * v1058);
        let v1074: f64 = (v152 * v1059);
        let v1075: f64 = (v152 * v1060);
        let v1076: f64 = (v152 * v1061);
        let v1077: f64 = (v152 * v1062);
        let v1078: f64 = (v267 * v1043);
        let v1079: f64 = (v262 * v1073);
        let v1080: f64 = (v1078 + v1079);
        let v1081: f64 = (v267 * v1044);
        let v1082: f64 = (v262 * v1074);
        let v1083: f64 = (v1081 + v1082);
        let v1084: f64 = (v267 * v1045);
        let v1085: f64 = (v262 * v1075);
        let v1086: f64 = (v1084 + v1085);
        let v1087: f64 = (v267 * v1046);
        let v1088: f64 = (v262 * v1076);
        let v1089: f64 = (v1087 + v1088);
        let v1090: f64 = (v267 * self.scalar_v1047);
        let v1091: f64 = (v262 * v1077);
        let v1092: f64 = (v1090 + v1091);
        let v1093: f64 = (v1068 + v1080);
        let v1094: f64 = (v1069 + v1083);
        let v1095: f64 = (v1070 + v1086);
        let v1096: f64 = (v1071 + v1089);
        let v1097: f64 = (v1072 + v1092);
        let v1098: f64 = (v147 * v1093);
        let v1099: f64 = (v269 * v613);
        let v1100: f64 = (v147 * v1094);
        let v1101: f64 = (v1099 + v1100);
        let v1102: f64 = (v269 * v614);
        let v1103: f64 = (v147 * v1095);
        let v1104: f64 = (v1102 + v1103);
        let v1105: f64 = (v147 * v1096);
        let v1106: f64 = (v147 * v1097);
        let v1107: f64 = (if self.scalar_v261 { v1098 } else { v684 });
        let v1108: f64 = (if self.scalar_v261 { v1101 } else { v685 });
        let v1109: f64 = (if self.scalar_v261 { v1104 } else { v686 });
        let v1110: f64 = (if self.scalar_v261 { v1105 } else { v687 });
        let v1111: f64 = (if self.scalar_v261 { v1106 } else { v688 });
        let v1112: f64 = { let limexp_arg = v271; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1113: f64 = (v1107 * v1112);
        let v1114: f64 = (v1108 * v1112);
        let v1115: f64 = (v1109 * v1112);
        let v1116: f64 = (v1110 * v1112);
        let v1117: f64 = (v1111 * v1112);
        let v1118: f64 = (-v1107);
        let v1119: f64 = (-v1108);
        let v1120: f64 = (-v1109);
        let v1121: f64 = (-v1110);
        let v1122: f64 = (-v1111);
        let v1123: f64 = { let limexp_arg = v273; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1124: f64 = (v1118 * v1123);
        let v1125: f64 = (v1119 * v1123);
        let v1126: f64 = (v1120 * v1123);
        let v1127: f64 = (v1121 * v1123);
        let v1128: f64 = (v1122 * v1123);
        let v1129: f64 = (v1113 - v1124);
        let v1130: f64 = (v1114 - v1125);
        let v1131: f64 = (v1115 - v1126);
        let v1132: f64 = (v1116 - v1127);
        let v1133: f64 = (v1117 - v1128);
        let v1134: f64 = (v123 * v1129);
        let v1135: f64 = (v123 * v1130);
        let v1136: f64 = (v123 * v1131);
        let v1137: f64 = (v123 * v1132);
        let v1138: f64 = (v123 * v1133);
        let v1139: f64 = (v277 * v277);
        let v1140: f64 = (v44 - v1139);
        let v1141: f64 = (v1134 * v1140);
        let v1142: f64 = (v1135 * v1140);
        let v1143: f64 = (v1136 * v1140);
        let v1144: f64 = (v1137 * v1140);
        let v1145: f64 = (v1138 * v1140);
        let v1146: f64 = (if self.scalar_v261 { v1141 } else { v725 });
        let v1147: f64 = (if self.scalar_v261 { v1142 } else { v726 });
        let v1148: f64 = (if self.scalar_v261 { v1143 } else { v727 });
        let v1149: f64 = (if self.scalar_v261 { v1144 } else { v728 });
        let v1150: f64 = (if self.scalar_v261 { v1145 } else { v729 });
        let v1151: f64 = (self.scalar_v155 * v1146);
        let v1152: f64 = (self.scalar_v155 * v1147);
        let v1153: f64 = (self.scalar_v155 * v1148);
        let v1154: f64 = (self.scalar_v155 * v1149);
        let v1155: f64 = (self.scalar_v155 * v1150);
        let v1156: f64 = (if self.scalar_v261 { v1151 } else { v13 });
        let v1157: f64 = (if self.scalar_v261 { v1152 } else { v13 });
        let v1158: f64 = (if self.scalar_v261 { v1153 } else { v13 });
        let v1159: f64 = (if self.scalar_v261 { v1154 } else { v13 });
        let v1160: f64 = (if self.scalar_v261 { v1155 } else { v13 });
        let v1161: f64 = (v7 * v1156);
        let v1162: f64 = (v7 * v1157);
        let v1163: f64 = (v282 + v1162);
        let v1164: f64 = (-v282);
        let v1165: f64 = (v7 * v1158);
        let v1166: f64 = (v1164 + v1165);
        let v1167: f64 = (v7 * v1159);
        let v1168: f64 = (v7 * v1160);
        let v1169: f64 = (v284 * v284);
        let v1170: f64 = (v44 - v1169);
        let v1171: f64 = (v1161 * v1170);
        let v1172: f64 = (v1163 * v1170);
        let v1173: f64 = (v1166 * v1170);
        let v1174: f64 = (v1167 * v1170);
        let v1175: f64 = (v1168 * v1170);
        let v1176: f64 = (if self.scalar_v261 { v1171 } else { v13 });
        let v1177: f64 = (if self.scalar_v261 { v1172 } else { v13 });
        let v1178: f64 = (if self.scalar_v261 { v1173 } else { v13 });
        let v1179: f64 = (if self.scalar_v261 { v1174 } else { v13 });
        let v1180: f64 = (if self.scalar_v261 { v1175 } else { v13 });
        let v1181: f64 = (self.scalar_v227 * v1146);
        let v1182: f64 = (self.scalar_v227 * v1147);
        let v1183: f64 = (self.scalar_v227 * v1148);
        let v1184: f64 = (self.scalar_v227 * v1149);
        let v1185: f64 = (self.scalar_v227 * v1150);
        let v1186: f64 = (if self.scalar_v261 { v1181 } else { v889 });
        let v1187: f64 = (if self.scalar_v261 { v1182 } else { v890 });
        let v1188: f64 = (if self.scalar_v261 { v1183 } else { v891 });
        let v1189: f64 = (if self.scalar_v261 { v1184 } else { v892 });
        let v1190: f64 = (if self.scalar_v261 { v1185 } else { v893 });
        let v1191: f64 = (v111 * v1146);
        let v1192: f64 = (v111 * v1147);
        let v1193: f64 = (v111 * v1148);
        let v1194: f64 = (v111 * v1149);
        let v1195: f64 = (v111 * v1150);
        let v1196: f64 = (v289 * v1176);
        let v1197: f64 = (v285 * v1191);
        let v1198: f64 = (v1196 + v1197);
        let v1199: f64 = (v289 * v1177);
        let v1200: f64 = (v285 * v1192);
        let v1201: f64 = (v1199 + v1200);
        let v1202: f64 = (v289 * v1178);
        let v1203: f64 = (v285 * v1193);
        let v1204: f64 = (v1202 + v1203);
        let v1205: f64 = (v289 * v1179);
        let v1206: f64 = (v285 * v1194);
        let v1207: f64 = (v1205 + v1206);
        let v1208: f64 = (v289 * v1180);
        let v1209: f64 = (v285 * v1195);
        let v1210: f64 = (v1208 + v1209);
        let v1211: f64 = (v7 * v1186);
        let v1212: f64 = (v7 * v1187);
        let v1213: f64 = (v288 + v1212);
        let v1214: f64 = (-v288);
        let v1215: f64 = (v7 * v1188);
        let v1216: f64 = (v1214 + v1215);
        let v1217: f64 = (v7 * v1189);
        let v1218: f64 = (v7 * v1190);
        let v1219: f64 = { let limexp_arg = v293; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1220: f64 = (self.scalar_v235 * v1219);
        let v1221: f64 = (self.scalar_v912 * v1219);
        let v1222: f64 = (v112 * v1220);
        let v1223: f64 = (v112 * v1221);
        let v1224: f64 = (v1213 + v1222);
        let v1225: f64 = (v1217 + v1223);
        let v1226: f64 = (v296 * v1198);
        let v1227: f64 = (v290 * v1211);
        let v1228: f64 = (v1226 + v1227);
        let v1229: f64 = (v296 * v1201);
        let v1230: f64 = (v290 * v1224);
        let v1231: f64 = (v1229 + v1230);
        let v1232: f64 = (v296 * v1204);
        let v1233: f64 = (v290 * v1216);
        let v1234: f64 = (v1232 + v1233);
        let v1235: f64 = (v296 * v1207);
        let v1236: f64 = (v290 * v1225);
        let v1237: f64 = (v1235 + v1236);
        let v1238: f64 = (v296 * v1210);
        let v1239: f64 = (v290 * v1218);
        let v1240: f64 = (v1238 + v1239);
        let v1241: f64 = (if self.scalar_v261 { v1228 } else { v1038 });
        let v1242: f64 = (if self.scalar_v261 { v1231 } else { v1039 });
        let v1243: f64 = (if self.scalar_v261 { v1234 } else { v1040 });
        let v1244: f64 = (if self.scalar_v261 { v1237 } else { v1041 });
        let v1245: f64 = (if self.scalar_v261 { v1240 } else { v1042 });
        let v1246: f64 = (if self.scalar_v301 { v634 } else { v1043 });
        let v1247: f64 = (if self.scalar_v301 { v635 } else { v1044 });
        let v1248: f64 = (if self.scalar_v301 { v636 } else { v1045 });
        let v1249: f64 = (if self.scalar_v301 { v637 } else { v1046 });
        let v1251: f64 = (v302 * v1246);
        let v1252: f64 = (v1251 + v1251);
        let v1253: f64 = (v302 * v1247);
        let v1254: f64 = (v1253 + v1253);
        let v1255: f64 = (v302 * v1248);
        let v1256: f64 = (v1255 + v1255);
        let v1257: f64 = (v302 * v1249);
        let v1258: f64 = (v1257 + v1257);
        let v1259: f64 = (v302 * self.scalar_v1250);
        let v1260: f64 = (v1259 + v1259);
        let v1261: f64 = (if self.scalar_v301 { v1252 } else { v1058 });
        let v1262: f64 = (if self.scalar_v301 { v1254 } else { v1059 });
        let v1263: f64 = (if self.scalar_v301 { v1256 } else { v1060 });
        let v1264: f64 = (if self.scalar_v301 { v1258 } else { v1061 });
        let v1265: f64 = (if self.scalar_v301 { v1260 } else { v1062 });
        let v1266: f64 = (self.scalar_v172 * v1261);
        let v1267: f64 = (self.scalar_v172 * v1262);
        let v1268: f64 = (self.scalar_v172 * v1263);
        let v1269: f64 = (self.scalar_v172 * v1264);
        let v1270: f64 = (self.scalar_v172 * v1265);
        let v1271: f64 = (v1246 + v1266);
        let v1272: f64 = (v1247 + v1267);
        let v1273: f64 = (v1248 + v1268);
        let v1274: f64 = (v1249 + v1269);
        let v1275: f64 = (self.scalar_v1250 + v1270);
        let v1276: f64 = (v152 * v1261);
        let v1277: f64 = (v152 * v1262);
        let v1278: f64 = (v152 * v1263);
        let v1279: f64 = (v152 * v1264);
        let v1280: f64 = (v152 * v1265);
        let v1281: f64 = (v307 * v1246);
        let v1282: f64 = (v302 * v1276);
        let v1283: f64 = (v1281 + v1282);
        let v1284: f64 = (v307 * v1247);
        let v1285: f64 = (v302 * v1277);
        let v1286: f64 = (v1284 + v1285);
        let v1287: f64 = (v307 * v1248);
        let v1288: f64 = (v302 * v1278);
        let v1289: f64 = (v1287 + v1288);
        let v1290: f64 = (v307 * v1249);
        let v1291: f64 = (v302 * v1279);
        let v1292: f64 = (v1290 + v1291);
        let v1293: f64 = (v307 * self.scalar_v1250);
        let v1294: f64 = (v302 * v1280);
        let v1295: f64 = (v1293 + v1294);
        let v1296: f64 = (v1271 + v1283);
        let v1297: f64 = (v1272 + v1286);
        let v1298: f64 = (v1273 + v1289);
        let v1299: f64 = (v1274 + v1292);
        let v1300: f64 = (v1275 + v1295);
        let v1301: f64 = (v147 * v1296);
        let v1302: f64 = (v309 * v613);
        let v1303: f64 = (v147 * v1297);
        let v1304: f64 = (v1302 + v1303);
        let v1305: f64 = (v309 * v614);
        let v1306: f64 = (v147 * v1298);
        let v1307: f64 = (v1305 + v1306);
        let v1308: f64 = (v147 * v1299);
        let v1309: f64 = (v147 * v1300);
        let v1310: f64 = (if self.scalar_v301 { v1301 } else { v1107 });
        let v1311: f64 = (if self.scalar_v301 { v1304 } else { v1108 });
        let v1312: f64 = (if self.scalar_v301 { v1307 } else { v1109 });
        let v1313: f64 = (if self.scalar_v301 { v1308 } else { v1110 });
        let v1314: f64 = (if self.scalar_v301 { v1309 } else { v1111 });
        let v1315: f64 = (if self.scalar_v301 { v634 } else { v825 });
        let v1316: f64 = (if self.scalar_v301 { v792 } else { v826 });
        let v1317: f64 = (if self.scalar_v301 { v793 } else { v827 });
        let v1318: f64 = (if self.scalar_v301 { v794 } else { v828 });
        let v1319: f64 = (if self.scalar_v301 { v13 } else { v829 });
        let v1320: f64 = (v312 * v1315);
        let v1321: f64 = (v1320 + v1320);
        let v1322: f64 = (v312 * v1316);
        let v1323: f64 = (v1322 + v1322);
        let v1324: f64 = (v312 * v1317);
        let v1325: f64 = (v1324 + v1324);
        let v1326: f64 = (v312 * v1318);
        let v1327: f64 = (v1326 + v1326);
        let v1328: f64 = (v312 * v1319);
        let v1329: f64 = (v1328 + v1328);
        let v1330: f64 = (if self.scalar_v301 { v1321 } else { v13 });
        let v1331: f64 = (if self.scalar_v301 { v1323 } else { v13 });
        let v1332: f64 = (if self.scalar_v301 { v1325 } else { v13 });
        let v1333: f64 = (if self.scalar_v301 { v1327 } else { v13 });
        let v1334: f64 = (if self.scalar_v301 { v1329 } else { v13 });
        let v1335: f64 = (self.scalar_v172 * v1330);
        let v1336: f64 = (self.scalar_v172 * v1331);
        let v1337: f64 = (self.scalar_v172 * v1332);
        let v1338: f64 = (self.scalar_v172 * v1333);
        let v1339: f64 = (self.scalar_v172 * v1334);
        let v1340: f64 = (v1315 + v1335);
        let v1341: f64 = (v1316 + v1336);
        let v1342: f64 = (v1317 + v1337);
        let v1343: f64 = (v1318 + v1338);
        let v1344: f64 = (v1319 + v1339);
        let v1345: f64 = (v152 * v1315);
        let v1346: f64 = (v152 * v1316);
        let v1347: f64 = (v152 * v1317);
        let v1348: f64 = (v152 * v1318);
        let v1349: f64 = (v152 * v1319);
        let v1350: f64 = (v317 * v1330);
        let v1351: f64 = (v314 * v1345);
        let v1352: f64 = (v1350 + v1351);
        let v1353: f64 = (v317 * v1331);
        let v1354: f64 = (v314 * v1346);
        let v1355: f64 = (v1353 + v1354);
        let v1356: f64 = (v317 * v1332);
        let v1357: f64 = (v314 * v1347);
        let v1358: f64 = (v1356 + v1357);
        let v1359: f64 = (v317 * v1333);
        let v1360: f64 = (v314 * v1348);
        let v1361: f64 = (v1359 + v1360);
        let v1362: f64 = (v317 * v1334);
        let v1363: f64 = (v314 * v1349);
        let v1364: f64 = (v1362 + v1363);
        let v1365: f64 = (v1340 + v1352);
        let v1366: f64 = (v1341 + v1355);
        let v1367: f64 = (v1342 + v1358);
        let v1368: f64 = (v1343 + v1361);
        let v1369: f64 = (v1344 + v1364);
        let v1370: f64 = (v147 * v1365);
        let v1371: f64 = (v319 * v613);
        let v1372: f64 = (v147 * v1366);
        let v1373: f64 = (v1371 + v1372);
        let v1374: f64 = (v319 * v614);
        let v1375: f64 = (v147 * v1367);
        let v1376: f64 = (v1374 + v1375);
        let v1377: f64 = (v147 * v1368);
        let v1378: f64 = (v147 * v1369);
        let v1379: f64 = (if self.scalar_v301 { v1370 } else { v857 });
        let v1380: f64 = (if self.scalar_v301 { v1373 } else { v858 });
        let v1381: f64 = (if self.scalar_v301 { v1376 } else { v859 });
        let v1382: f64 = (if self.scalar_v301 { v1377 } else { v860 });
        let v1383: f64 = (if self.scalar_v301 { v1378 } else { v861 });
        let v1384: f64 = { let limexp_arg = v311; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1385: f64 = (v1310 * v1384);
        let v1386: f64 = (v1311 * v1384);
        let v1387: f64 = (v1312 * v1384);
        let v1388: f64 = (v1313 * v1384);
        let v1389: f64 = (v1314 * v1384);
        let v1390: f64 = (-v1310);
        let v1391: f64 = (-v1311);
        let v1392: f64 = (-v1312);
        let v1393: f64 = (-v1313);
        let v1394: f64 = (-v1314);
        let v1395: f64 = { let limexp_arg = v323; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1396: f64 = (v1390 * v1395);
        let v1397: f64 = (v1391 * v1395);
        let v1398: f64 = (v1392 * v1395);
        let v1399: f64 = (v1393 * v1395);
        let v1400: f64 = (v1394 * v1395);
        let v1401: f64 = (v1385 - v1396);
        let v1402: f64 = (v1386 - v1397);
        let v1403: f64 = (v1387 - v1398);
        let v1404: f64 = (v1388 - v1399);
        let v1405: f64 = (v1389 - v1400);
        let v1406: f64 = (v123 * v1401);
        let v1407: f64 = (v123 * v1402);
        let v1408: f64 = (v123 * v1403);
        let v1409: f64 = (v123 * v1404);
        let v1410: f64 = (v123 * v1405);
        let v1411: f64 = (v327 * v327);
        let v1412: f64 = (v44 - v1411);
        let v1413: f64 = (v1406 * v1412);
        let v1414: f64 = (v1407 * v1412);
        let v1415: f64 = (v1408 * v1412);
        let v1416: f64 = (v1409 * v1412);
        let v1417: f64 = (v1410 * v1412);
        let v1418: f64 = (if self.scalar_v301 { v1413 } else { v1146 });
        let v1419: f64 = (if self.scalar_v301 { v1414 } else { v1147 });
        let v1420: f64 = (if self.scalar_v301 { v1415 } else { v1148 });
        let v1421: f64 = (if self.scalar_v301 { v1416 } else { v1149 });
        let v1422: f64 = (if self.scalar_v301 { v1417 } else { v1150 });
        let v1423: f64 = { let limexp_arg = v321; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1424: f64 = (v1379 * v1423);
        let v1425: f64 = (v1380 * v1423);
        let v1426: f64 = (v1381 * v1423);
        let v1427: f64 = (v1382 * v1423);
        let v1428: f64 = (v1383 * v1423);
        let v1429: f64 = (-v1379);
        let v1430: f64 = (-v1380);
        let v1431: f64 = (-v1381);
        let v1432: f64 = (-v1382);
        let v1433: f64 = (-v1383);
        let v1434: f64 = { let limexp_arg = v331; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1435: f64 = (v1429 * v1434);
        let v1436: f64 = (v1430 * v1434);
        let v1437: f64 = (v1431 * v1434);
        let v1438: f64 = (v1432 * v1434);
        let v1439: f64 = (v1433 * v1434);
        let v1440: f64 = (v1424 - v1435);
        let v1441: f64 = (v1425 - v1436);
        let v1442: f64 = (v1426 - v1437);
        let v1443: f64 = (v1427 - v1438);
        let v1444: f64 = (v1428 - v1439);
        let v1445: f64 = (v123 * v1440);
        let v1446: f64 = (v123 * v1441);
        let v1447: f64 = (v123 * v1442);
        let v1448: f64 = (v123 * v1443);
        let v1449: f64 = (v123 * v1444);
        let v1450: f64 = (v335 * v335);
        let v1451: f64 = (v44 - v1450);
        let v1452: f64 = (v1445 * v1451);
        let v1453: f64 = (v1446 * v1451);
        let v1454: f64 = (v1447 * v1451);
        let v1455: f64 = (v1448 * v1451);
        let v1456: f64 = (v1449 * v1451);
        let v1457: f64 = (if self.scalar_v301 { v1452 } else { v13 });
        let v1458: f64 = (if self.scalar_v301 { v1453 } else { v13 });
        let v1459: f64 = (if self.scalar_v301 { v1454 } else { v13 });
        let v1460: f64 = (if self.scalar_v301 { v1455 } else { v13 });
        let v1461: f64 = (if self.scalar_v301 { v1456 } else { v13 });
        let v1462: f64 = (self.scalar_v155 * v1418);
        let v1463: f64 = (self.scalar_v155 * v1419);
        let v1464: f64 = (self.scalar_v155 * v1420);
        let v1465: f64 = (self.scalar_v155 * v1421);
        let v1466: f64 = (self.scalar_v155 * v1422);
        let v1467: f64 = (if self.scalar_v301 { v1462 } else { v1156 });
        let v1468: f64 = (if self.scalar_v301 { v1463 } else { v1157 });
        let v1469: f64 = (if self.scalar_v301 { v1464 } else { v1158 });
        let v1470: f64 = (if self.scalar_v301 { v1465 } else { v1159 });
        let v1471: f64 = (if self.scalar_v301 { v1466 } else { v1160 });
        let v1472: f64 = (self.scalar_v155 * v1457);
        let v1473: f64 = (self.scalar_v155 * v1458);
        let v1474: f64 = (self.scalar_v155 * v1459);
        let v1475: f64 = (self.scalar_v155 * v1460);
        let v1476: f64 = (self.scalar_v155 * v1461);
        let v1477: f64 = (if self.scalar_v301 { v1472 } else { v13 });
        let v1478: f64 = (if self.scalar_v301 { v1473 } else { v13 });
        let v1479: f64 = (if self.scalar_v301 { v1474 } else { v13 });
        let v1480: f64 = (if self.scalar_v301 { v1475 } else { v13 });
        let v1481: f64 = (if self.scalar_v301 { v1476 } else { v13 });
        let v1482: f64 = (v7 * v1467);
        let v1483: f64 = (v7 * v1468);
        let v1484: f64 = (v340 + v1483);
        let v1485: f64 = (-v340);
        let v1486: f64 = (v7 * v1469);
        let v1487: f64 = (v1485 + v1486);
        let v1488: f64 = (v7 * v1470);
        let v1489: f64 = (v7 * v1471);
        let v1490: f64 = (v345 * v345);
        let v1491: f64 = (v44 - v1490);
        let v1492: f64 = (v1482 * v1491);
        let v1493: f64 = (v1484 * v1491);
        let v1494: f64 = (v1487 * v1491);
        let v1495: f64 = (v1488 * v1491);
        let v1496: f64 = (v1489 * v1491);
        let v1497: f64 = (if self.scalar_v301 { v1492 } else { v1176 });
        let v1498: f64 = (if self.scalar_v301 { v1493 } else { v1177 });
        let v1499: f64 = (if self.scalar_v301 { v1494 } else { v1178 });
        let v1500: f64 = (if self.scalar_v301 { v1495 } else { v1179 });
        let v1501: f64 = (if self.scalar_v301 { v1496 } else { v1180 });
        let v1502: f64 = (v7 * v1477);
        let v1503: f64 = (v7 * v1478);
        let v1504: f64 = (v343 + v1503);
        let v1505: f64 = (-v343);
        let v1506: f64 = (v7 * v1479);
        let v1507: f64 = (v1505 + v1506);
        let v1508: f64 = (v7 * v1480);
        let v1509: f64 = (v7 * v1481);
        let v1510: f64 = (v348 * v348);
        let v1511: f64 = (v44 - v1510);
        let v1512: f64 = (v1502 * v1511);
        let v1513: f64 = (v1504 * v1511);
        let v1514: f64 = (v1507 * v1511);
        let v1515: f64 = (v1508 * v1511);
        let v1516: f64 = (v1509 * v1511);
        let v1517: f64 = (if self.scalar_v301 { v1512 } else { v13 });
        let v1518: f64 = (if self.scalar_v301 { v1513 } else { v13 });
        let v1519: f64 = (if self.scalar_v301 { v1514 } else { v13 });
        let v1520: f64 = (if self.scalar_v301 { v1515 } else { v13 });
        let v1521: f64 = (if self.scalar_v301 { v1516 } else { v13 });
        let v1522: f64 = (self.scalar_v227 * v1457);
        let v1523: f64 = (self.scalar_v227 * v1458);
        let v1524: f64 = (self.scalar_v227 * v1459);
        let v1525: f64 = (self.scalar_v227 * v1460);
        let v1526: f64 = (self.scalar_v227 * v1461);
        let v1527: f64 = (if self.scalar_v301 { v1522 } else { v13 });
        let v1528: f64 = (if self.scalar_v301 { v1523 } else { v13 });
        let v1529: f64 = (if self.scalar_v301 { v1524 } else { v13 });
        let v1530: f64 = (if self.scalar_v301 { v1525 } else { v13 });
        let v1531: f64 = (if self.scalar_v301 { v1526 } else { v13 });
        let v1532: f64 = (self.scalar_v227 * v1418);
        let v1533: f64 = (self.scalar_v227 * v1419);
        let v1534: f64 = (self.scalar_v227 * v1420);
        let v1535: f64 = (self.scalar_v227 * v1421);
        let v1536: f64 = (self.scalar_v227 * v1422);
        let v1537: f64 = (if self.scalar_v301 { v1532 } else { v13 });
        let v1538: f64 = (if self.scalar_v301 { v1533 } else { v13 });
        let v1539: f64 = (if self.scalar_v301 { v1534 } else { v13 });
        let v1540: f64 = (if self.scalar_v301 { v1535 } else { v13 });
        let v1541: f64 = (if self.scalar_v301 { v1536 } else { v13 });
        let v1542: f64 = (v111 * v1418);
        let v1543: f64 = (v111 * v1419);
        let v1544: f64 = (v111 * v1420);
        let v1545: f64 = (v111 * v1421);
        let v1546: f64 = (v111 * v1422);
        let v1547: f64 = (v357 * v1542);
        let v1548: f64 = (v356 * v1497);
        let v1549: f64 = (v1547 + v1548);
        let v1550: f64 = (v357 * v1543);
        let v1551: f64 = (v356 * v1498);
        let v1552: f64 = (v1550 + v1551);
        let v1553: f64 = (v357 * v1544);
        let v1554: f64 = (v356 * v1499);
        let v1555: f64 = (v1553 + v1554);
        let v1556: f64 = (v357 * v1545);
        let v1557: f64 = (v356 * v1500);
        let v1558: f64 = (v1556 + v1557);
        let v1559: f64 = (v357 * v1546);
        let v1560: f64 = (v356 * v1501);
        let v1561: f64 = (v1559 + v1560);
        let v1562: f64 = (v7 * v1537);
        let v1563: f64 = (v7 * v1538);
        let v1564: f64 = (v355 + v1563);
        let v1565: f64 = (-v355);
        let v1566: f64 = (v7 * v1539);
        let v1567: f64 = (v1565 + v1566);
        let v1568: f64 = (v7 * v1540);
        let v1569: f64 = (v7 * v1541);
        let v1570: f64 = (v916 + v1564);
        let v1571: f64 = (v917 + v1567);
        let v1572: f64 = (v361 * v1549);
        let v1573: f64 = (v358 * v1562);
        let v1574: f64 = (v1572 + v1573);
        let v1575: f64 = (v361 * v1552);
        let v1576: f64 = (v358 * v1570);
        let v1577: f64 = (v1575 + v1576);
        let v1578: f64 = (v361 * v1555);
        let v1579: f64 = (v358 * v1571);
        let v1580: f64 = (v1578 + v1579);
        let v1581: f64 = (v361 * v1558);
        let v1582: f64 = (v358 * v1568);
        let v1583: f64 = (v1581 + v1582);
        let v1584: f64 = (v361 * v1561);
        let v1585: f64 = (v358 * v1569);
        let v1586: f64 = (v1584 + v1585);
        let v1587: f64 = (if self.scalar_v301 { v1574 } else { v935 });
        let v1588: f64 = (if self.scalar_v301 { v1577 } else { v936 });
        let v1589: f64 = (if self.scalar_v301 { v1580 } else { v937 });
        let v1590: f64 = (if self.scalar_v301 { v1583 } else { v938 });
        let v1591: f64 = (if self.scalar_v301 { v1586 } else { v939 });
        let v1592: f64 = (v111 * v1457);
        let v1593: f64 = (v111 * v1458);
        let v1594: f64 = (v111 * v1459);
        let v1595: f64 = (v111 * v1460);
        let v1596: f64 = (v111 * v1461);
        let v1597: f64 = (-v1517);
        let v1598: f64 = (-v1518);
        let v1599: f64 = (-v1519);
        let v1600: f64 = (-v1520);
        let v1601: f64 = (-v1521);
        let v1602: f64 = (v365 * v1592);
        let v1603: f64 = (v364 * v1597);
        let v1604: f64 = (v1602 + v1603);
        let v1605: f64 = (v365 * v1593);
        let v1606: f64 = (v364 * v1598);
        let v1607: f64 = (v1605 + v1606);
        let v1608: f64 = (v365 * v1594);
        let v1609: f64 = (v364 * v1599);
        let v1610: f64 = (v1608 + v1609);
        let v1611: f64 = (v365 * v1595);
        let v1612: f64 = (v364 * v1600);
        let v1613: f64 = (v1611 + v1612);
        let v1614: f64 = (v365 * v1596);
        let v1615: f64 = (v364 * v1601);
        let v1616: f64 = (v1614 + v1615);
        let v1617: f64 = (v7 * v1527);
        let v1618: f64 = (v7 * v1528);
        let v1619: f64 = (v352 + v1618);
        let v1620: f64 = (-v352);
        let v1621: f64 = (v7 * v1529);
        let v1622: f64 = (v1620 + v1621);
        let v1623: f64 = (v7 * v1530);
        let v1624: f64 = (v7 * v1531);
        let v1625: f64 = (-v1617);
        let v1626: f64 = (-v1619);
        let v1627: f64 = (-v1622);
        let v1628: f64 = (-v1623);
        let v1629: f64 = (-v1624);
        let v1630: f64 = (v368 * v1604);
        let v1631: f64 = (v366 * v1625);
        let v1632: f64 = (v1630 + v1631);
        let v1633: f64 = (v368 * v1607);
        let v1634: f64 = (v366 * v1626);
        let v1635: f64 = (v1633 + v1634);
        let v1636: f64 = (v368 * v1610);
        let v1637: f64 = (v366 * v1627);
        let v1638: f64 = (v1636 + v1637);
        let v1639: f64 = (v368 * v1613);
        let v1640: f64 = (v366 * v1628);
        let v1641: f64 = (v1639 + v1640);
        let v1642: f64 = (v368 * v1616);
        let v1643: f64 = (v366 * v1629);
        let v1644: f64 = (v1642 + v1643);
        let v1645: f64 = (if self.scalar_v301 { v1632 } else { v1023 });
        let v1646: f64 = (if self.scalar_v301 { v1635 } else { v1024 });
        let v1647: f64 = (if self.scalar_v301 { v1638 } else { v1025 });
        let v1648: f64 = (if self.scalar_v301 { v1641 } else { v1026 });
        let v1649: f64 = (if self.scalar_v301 { v1644 } else { v1027 });
        let v1650: f64 = (v1587 - v1645);
        let v1651: f64 = (v1588 - v1646);
        let v1652: f64 = (v1589 - v1647);
        let v1653: f64 = (v1590 - v1648);
        let v1654: f64 = (v1591 - v1649);
        let v1655: f64 = (v123 * v1650);
        let v1656: f64 = (v123 * v1651);
        let v1657: f64 = (v123 * v1652);
        let v1658: f64 = (v123 * v1653);
        let v1659: f64 = (v123 * v1654);
        let v1660: f64 = (if self.scalar_v301 { v1655 } else { v1241 });
        let v1661: f64 = (if self.scalar_v301 { v1656 } else { v1242 });
        let v1662: f64 = (if self.scalar_v301 { v1657 } else { v1243 });
        let v1663: f64 = (if self.scalar_v301 { v1658 } else { v1244 });
        let v1664: f64 = (if self.scalar_v301 { v1659 } else { v1245 });
        let v1665: f64 = (if self.scalar_v376 { v884 } else { v1186 });
        let v1666: f64 = (if self.scalar_v376 { v885 } else { v1187 });
        let v1667: f64 = (if self.scalar_v376 { v886 } else { v1188 });
        let v1668: f64 = (if self.scalar_v376 { v887 } else { v1189 });
        let v1669: f64 = (if self.scalar_v376 { v888 } else { v1190 });
        let v1670: f64 = (if self.scalar_v376 { v1462 } else { v1467 });
        let v1671: f64 = (if self.scalar_v376 { v1463 } else { v1468 });
        let v1672: f64 = (if self.scalar_v376 { v1464 } else { v1469 });
        let v1673: f64 = (if self.scalar_v376 { v1465 } else { v1470 });
        let v1674: f64 = (if self.scalar_v376 { v1466 } else { v1471 });
        let v1675: f64 = (v7 * v1670);
        let v1676: f64 = (v7 * v1671);
        let v1677: f64 = (v378 + v1676);
        let v1678: f64 = (-v378);
        let v1679: f64 = (v7 * v1672);
        let v1680: f64 = (v1678 + v1679);
        let v1681: f64 = (v7 * v1673);
        let v1682: f64 = (v7 * v1674);
        let v1683: f64 = (v380 * v380);
        let v1684: f64 = (v44 - v1683);
        let v1685: f64 = (v1675 * v1684);
        let v1686: f64 = (v1677 * v1684);
        let v1687: f64 = (v1680 * v1684);
        let v1688: f64 = (v1681 * v1684);
        let v1689: f64 = (v1682 * v1684);
        let v1690: f64 = (if self.scalar_v376 { v1685 } else { v1497 });
        let v1691: f64 = (if self.scalar_v376 { v1686 } else { v1498 });
        let v1692: f64 = (if self.scalar_v376 { v1687 } else { v1499 });
        let v1693: f64 = (if self.scalar_v376 { v1688 } else { v1500 });
        let v1694: f64 = (if self.scalar_v376 { v1689 } else { v1501 });
        let v1695: f64 = (v11 * v1670);
        let v1696: f64 = (v378 + v1695);
        let v1697: f64 = (v11 * v1671);
        let v1698: f64 = (v11 * v1672);
        let v1699: f64 = (v1678 + v1698);
        let v1700: f64 = (v11 * v1673);
        let v1701: f64 = (v11 * v1674);
        let v1702: f64 = (v383 * v383);
        let v1703: f64 = (v44 - v1702);
        let v1704: f64 = (v1696 * v1703);
        let v1705: f64 = (v1697 * v1703);
        let v1706: f64 = (v1699 * v1703);
        let v1707: f64 = (v1700 * v1703);
        let v1708: f64 = (v1701 * v1703);
        let v1709: f64 = (if self.scalar_v376 { v1704 } else { v13 });
        let v1710: f64 = (if self.scalar_v376 { v1705 } else { v13 });
        let v1711: f64 = (if self.scalar_v376 { v1706 } else { v13 });
        let v1712: f64 = (if self.scalar_v376 { v1707 } else { v13 });
        let v1713: f64 = (if self.scalar_v376 { v1708 } else { v13 });
        let v1714: f64 = (self.scalar_v385 * v1709);
        let v1715: f64 = (self.scalar_v385 * v1710);
        let v1716: f64 = (self.scalar_v385 * v1711);
        let v1717: f64 = (self.scalar_v385 * v1712);
        let v1718: f64 = (self.scalar_v385 * v1713);
        let v1719: f64 = (v1690 + v1714);
        let v1720: f64 = (v1691 + v1715);
        let v1721: f64 = (v1692 + v1716);
        let v1722: f64 = (v1693 + v1717);
        let v1723: f64 = (v1694 + v1718);
        let v1724: f64 = (v387 * v750);
        let v1725: f64 = (v197 * v1719);
        let v1726: f64 = (v1724 + v1725);
        let v1727: f64 = (v387 * v751);
        let v1728: f64 = (v197 * v1720);
        let v1729: f64 = (v1727 + v1728);
        let v1730: f64 = (v387 * v752);
        let v1731: f64 = (v197 * v1721);
        let v1732: f64 = (v1730 + v1731);
        let v1733: f64 = (v387 * v753);
        let v1734: f64 = (v197 * v1722);
        let v1735: f64 = (v1733 + v1734);
        let v1736: f64 = (v387 * v754);
        let v1737: f64 = (v197 * v1723);
        let v1738: f64 = (v1736 + v1737);
        let v1741: f64 = (v390 * v1665);
        let v1742: f64 = (v377 * self.scalar_v385);
        let v1743: f64 = (v1741 + v1742);
        let v1744: f64 = (v390 * v1666);
        let v1745: f64 = (v377 + v1744);
        let v1746: f64 = (v390 * v1667);
        let v1747: f64 = (v377 * self.scalar_v1740);
        let v1748: f64 = (v1746 + v1747);
        let v1749: f64 = (v390 * v1668);
        let v1750: f64 = (v390 * v1669);
        let v1751: f64 = (v916 + v1745);
        let v1752: f64 = (v917 + v1748);
        let v1753: f64 = (v393 * v1726);
        let v1754: f64 = (v388 * v1743);
        let v1755: f64 = (v1753 + v1754);
        let v1756: f64 = (v393 * v1729);
        let v1757: f64 = (v388 * v1751);
        let v1758: f64 = (v1756 + v1757);
        let v1759: f64 = (v393 * v1732);
        let v1760: f64 = (v388 * v1752);
        let v1761: f64 = (v1759 + v1760);
        let v1762: f64 = (v393 * v1735);
        let v1763: f64 = (v388 * v1749);
        let v1764: f64 = (v1762 + v1763);
        let v1765: f64 = (v393 * v1738);
        let v1766: f64 = (v388 * v1750);
        let v1767: f64 = (v1765 + v1766);
        let v1768: f64 = (if self.scalar_v376 { v1755 } else { v1660 });
        let v1769: f64 = (if self.scalar_v376 { v1758 } else { v1661 });
        let v1770: f64 = (if self.scalar_v376 { v1761 } else { v1662 });
        let v1771: f64 = (if self.scalar_v376 { v1764 } else { v1663 });
        let v1772: f64 = (if self.scalar_v376 { v1767 } else { v1664 });
        let v1773: f64 = (if self.scalar_v400 { v13 } else { v1246 });
        let v1774: f64 = (if self.scalar_v400 { v13 } else { v1247 });
        let v1775: f64 = (if self.scalar_v400 { v13 } else { v1248 });
        let v1776: f64 = (if self.scalar_v400 { v13 } else { v1249 });
        let v1780: f64 = (if self.scalar_v418 { v13 } else { v1773 });
        let v1781: f64 = (if self.scalar_v418 { v13 } else { v1774 });
        let v1782: f64 = (if self.scalar_v418 { v13 } else { v1775 });
        let v1783: f64 = (if self.scalar_v418 { v13 } else { v1776 });
        let v1785: f64 = (v433 * v433);
        let v1786: f64 = (v44 - v1785);
        let v1787: f64 = (-v1786);
        let v1788: f64 = (if self.scalar_v432 { v1787 } else { self.scalar_v1778 });
        let v1789: f64 = (if self.scalar_v432 { v1786 } else { self.scalar_v1779 });
        let v1790: f64 = (v435 * v435);
        let v1791: f64 = (v44 - v1790);
        let v1792: f64 = (-v1791);
        let v1793: f64 = (if self.scalar_v432 { v1792 } else { self.scalar_v1778 });
        let v1794: f64 = (if self.scalar_v432 { v1791 } else { self.scalar_v1779 });
        let v1795: f64 = (if self.scalar_v438 { v401 } else { v1788 });
        let v1796: f64 = (if self.scalar_v438 { v44 } else { v1789 });
        let v1797: f64 = (if self.scalar_v438 { v401 } else { v1793 });
        let v1798: f64 = (if self.scalar_v438 { v44 } else { v1794 });
        let v1803: f64 = { let limexp_arg = v443; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1804: f64 = (self.scalar_v1801 * v1803);
        let v1805: f64 = (self.scalar_v1802 * v1803);
        let v1806: f64 = (v130 * v1795);
        let v1807: f64 = (v130 * v1796);
        let v1808: f64 = { let limexp_arg = v447; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1809: f64 = (v1806 * v1808);
        let v1810: f64 = (v1807 * v1808);
        let v1811: f64 = (self.scalar_v451 * v1804);
        let v1812: f64 = (self.scalar_v451 * v1805);
        let v1813: f64 = (v1809 - v1811);
        let v1814: f64 = (v1810 - v1812);
        let v1815: f64 = (-v1780);
        let v1816: f64 = (-v1781);
        let v1817: f64 = (v1813 - v1782);
        let v1818: f64 = (-v1783);
        let v1820: f64 = (self.scalar_v446 * v1815);
        let v1821: f64 = (self.scalar_v446 * v1816);
        let v1822: f64 = (self.scalar_v446 * v1817);
        let v1823: f64 = (self.scalar_v446 * v1818);
        let v1824: f64 = (self.scalar_v446 * v1814);
        let v1826: f64 = { let limexp_arg = v456; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1827: f64 = (self.scalar_v1801 * v1826);
        let v1828: f64 = (self.scalar_v1802 * v1826);
        let v1829: f64 = (v130 * v1797);
        let v1830: f64 = (v130 * v1798);
        let v1831: f64 = { let limexp_arg = v459; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1832: f64 = (v1829 * v1831);
        let v1833: f64 = (v1830 * v1831);
        let v1834: f64 = (self.scalar_v451 * v1827);
        let v1835: f64 = (self.scalar_v451 * v1828);
        let v1836: f64 = (v1832 - v1834);
        let v1837: f64 = (v1833 - v1835);
        let v1838: f64 = (v1836 - v1781);
        let v1839: f64 = (-v1782);
        let v1840: f64 = (v1837 - v1783);
        let v1841: f64 = (self.scalar_v446 * v1838);
        let v1842: f64 = (self.scalar_v446 * v1839);
        let v1843: f64 = (self.scalar_v446 * v1840);
        let v1844: f64 = (-v1768);
        let v1845: f64 = (-v1769);
        let v1846: f64 = (-v1770);
        let v1847: f64 = (-v1771);
        let v1848: f64 = (-v1772);
        let v1851: f64 = (-v114);
        let v1852: f64 = -1e-12;
        let v1853: f64 = (v44 / v115);
        let v1854: f64 = (v401 / v115);
        let v1855: f64 = (if self.scalar_v468 { v1853 } else { v13 });
        let v1856: f64 = (if self.scalar_v468 { v1854 } else { v13 });
        let v1871: f64 = (if self.scalar_v487 { v506 } else { v13 });
        let v1872: f64 = (if self.scalar_v487 { v502 } else { v13 });
        let v1873: f64 = (v44 / v49);
        let v1874: f64 = (if self.scalar_v509 { v1873 } else { v13 });

        let d510_dn4: f64 = v1844;
        let d510_dn5: f64 = v1845;
        let d510_dn8: f64 = v1846;
        let d510_dn10: f64 = v1847;
        let d510_dn12: f64 = v1848;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(15),
            None,
            multiplicity * (v510),
            [4, 5, 8, 10, 12],
            [d510_dn4, d510_dn5, d510_dn8, d510_dn10, d510_dn12],
            [],
            [],
            multiplicity,
        );
        let d12_dn16: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v12),
            16,
            multiplicity * (d12_dn16),
        );
        let d12_dn16: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * (v12),
            16,
            multiplicity * (d12_dn16),
        );
        let d455_dn4: f64 = v1820;
        let d455_dn5: f64 = v1821;
        let d455_dn8: f64 = v1822;
        let d455_dn10: f64 = v1823;
        let d455_dn11: f64 = v1824;
        let d455_dn12: f64 = self.scalar_v1825;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(8),
            multiplicity * (v455),
            [4, 5, 8, 10, 11, 12],
            [d455_dn4, d455_dn5, d455_dn8, d455_dn10, d455_dn11, d455_dn12],
            [],
            [],
            multiplicity,
        );
        let d464_dn4: f64 = v1820;
        let d464_dn5: f64 = v1841;
        let d464_dn8: f64 = v1842;
        let d464_dn10: f64 = v1843;
        let d464_dn12: f64 = self.scalar_v1825;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * (v464),
            [4, 5, 8, 10, 12],
            [d464_dn4, d464_dn5, d464_dn8, d464_dn10, d464_dn12],
            [],
            [],
            multiplicity,
        );
        let d523_dn4: f64 = v1852;
        let d523_dn6: f64 = v138;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v523),
            4,
            multiplicity * (d523_dn4),
            6,
            multiplicity * (d523_dn6),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v525,
        );
        let d528_dn11: f64 = v1855;
        let d528_dn12: f64 = v1856;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(12),
            multiplicity * (v528),
            11,
            multiplicity * (d528_dn11),
            12,
            multiplicity * (d528_dn12),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(8),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v530,
        );
        let d537_dn8: f64 = self.scalar_v1860;
        let d537_dn14: f64 = self.scalar_v1861;
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * (v537),
            8,
            multiplicity * (d537_dn8),
            14,
            multiplicity * (d537_dn14),
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v539,
        );
        let d543_dn10: f64 = self.scalar_v1864;
        let d543_dn13: f64 = self.scalar_v1865;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(10),
            multiplicity * (v543),
            10,
            multiplicity * (d543_dn10),
            13,
            multiplicity * (d543_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(10),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v545,
        );
        stamper.stamp_current_const_local(
            Some(13),
            Some(10),
            multiplicity * (self.scalar_v546),
        );
        let d549_dn11: f64 = self.scalar_v1868;
        let d549_dn13: f64 = self.scalar_v1869;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(11),
            multiplicity * (v549),
            11,
            multiplicity * (d549_dn11),
            13,
            multiplicity * (d549_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(11),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v551,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            self.scalar_v553,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v555,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v557,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            self.scalar_v559,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            self.scalar_v561,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            self.scalar_v563,
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (v564),
        );
        stamper.stamp_current_const_local(
            Some(14),
            Some(2),
            multiplicity * (v138),
        );
        let d567_dn2: f64 = v1852;
        let d567_dn12: f64 = v138;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * (v567),
            2,
            multiplicity * (d567_dn2),
            12,
            multiplicity * (d567_dn12),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v569),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v569),
        );
        stamper.stamp_current_const_local(
            Some(17),
            None,
            multiplicity * (self.scalar_v570),
        );
        let d572_dn17: f64 = self.scalar_v1870;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v572),
            17,
            multiplicity * (d572_dn17),
        );
        stamper.stamp_current_const_local(
            Some(18),
            None,
            multiplicity * (self.scalar_v570),
        );
        let d574_dn18: f64 = self.scalar_v1870;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v574),
            18,
            multiplicity * (d574_dn18),
        );
        let d572_dn17: f64 = self.scalar_v1870;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (v572),
            17,
            multiplicity * (d572_dn17),
        );
        let d578_dn17: f64 = v1871;
        let d578_dn18: f64 = v1872;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v578),
            17,
            multiplicity * (d578_dn17),
            18,
            multiplicity * (d578_dn18),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v570),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (self.scalar_v570),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v570),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v580),
        );
        let d571_dn17: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v571),
            17,
            multiplicity * (d571_dn17),
        );
        let d573_dn18: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v573),
            18,
            multiplicity * (d573_dn18),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (self.scalar_v546),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (self.scalar_v546),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (self.scalar_v582),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (self.scalar_v582),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v589),
        );
        let d591_dn3: f64 = v1874;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v591),
            3,
            multiplicity * (d591_dn3),
        );
        let d594_dn3: f64 = self.scalar_v1875;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v594),
            3,
            multiplicity * (d594_dn3),
        );
        let d513_dn15: f64 = self.scalar_v511;
        let v513_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v513);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v513_ddt),
            15,
            multiplicity * (((d513_dn15) * ddt_scale)),
        );
        let d517_dn5: f64 = self.scalar_v1849;
        let d517_dn7: f64 = self.scalar_v514;
        let v517_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v517);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v517_ddt),
            5,
            multiplicity * (((d517_dn5) * ddt_scale)),
            7,
            multiplicity * (((d517_dn7) * ddt_scale)),
        );
        let d519_dn5: f64 = self.scalar_v518;
        let d519_dn8: f64 = self.scalar_v1850;
        let v519_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v519);
        stamper.stamp_current_node2_local(
            Some(5),
            Some(8),
            multiplicity * (v519_ddt),
            5,
            multiplicity * (((d519_dn5) * ddt_scale)),
            8,
            multiplicity * (((d519_dn8) * ddt_scale)),
        );
        let d522_dn4: f64 = v1851;
        let d522_dn6: f64 = v114;
        let v522_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v522);
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v522_ddt),
            4,
            multiplicity * (((d522_dn4) * ddt_scale)),
            6,
            multiplicity * (((d522_dn6) * ddt_scale)),
        );
        let d534_dn11: f64 = self.scalar_v531;
        let d534_dn14: f64 = self.scalar_v1857;
        let v534_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, v534);
        stamper.stamp_current_node2_local(
            Some(11),
            Some(14),
            multiplicity * (v534_ddt),
            11,
            multiplicity * (((d534_dn11) * ddt_scale)),
            14,
            multiplicity * (((d534_dn14) * ddt_scale)),
        );
        let mut locals = StampLocals::default();

        Self::stamp_transient_block_0(ctx, p, nodes, param_given, &mut locals);
        Self::stamp_transient_block_1(p, &mut locals);
        Self::stamp_transient_block_2(p, &mut locals);
        Self::stamp_transient_block_3(ctx, p, branches, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, &mut locals);

        stamper.stamp_potential_branch_local(
            Some(15),
            Some(16),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(2),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(0),
            18,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, &mut locals);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let v1: f64 = nv8;
        let v4: f64 = nv5;
        let v7: f64 = (v4 - v1);
        let v8: f64 = nv11;
        let v10: f64 = nv4;
        let v13: f64 = 0.0;
        let v32: f64 = nv3;
        let v33: f64 = ((v32) as f64).abs();
        let v34: f64 = (self.scalar_v23 + v33);
        let v35: f64 = (if (self.scalar_v31 != 0.0) { v34 } else { self.scalar_v23 });
        let v38: f64 = (v35 - self.scalar_v30);
        let v39: f64 = ((v38) as f64).abs();
        let v40: bool = (v39 > v13);
        let v43: bool = (v40 || self.scalar_v42);
        let v44: f64 = 1.0;
        let v46: f64 = ((v39) as f64).abs();
        let v71: f64 = (v46 * self.scalar_v70);
        let v72: f64 = (v44 + v71);
        let v73: f64 = (self.scalar_v69 * v72);
        let v74: f64 = (if v43 { v73 } else { v13 });
        let v110: bool = (!v43);
        let v114: f64 = (if v110 { self.scalar_v69 } else { v74 });
        let v512: f64 = nv15;
        let v513: f64 = (self.scalar_v511 * v512);
        let v515: f64 = nv7;
        let v516: f64 = (v515 - v4);
        let v517: f64 = (self.scalar_v514 * v516);
        let v519: f64 = (v7 * self.scalar_v518);
        let v520: f64 = nv6;
        let v521: f64 = (v520 - v10);
        let v522: f64 = (v114 * v521);
        let v532: f64 = nv14;
        let v533: f64 = (v8 - v532);
        let v534: f64 = (self.scalar_v531 * v533);
        let v1851: f64 = (-v114);

        let d513_dn15: f64 = self.scalar_v511;
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (d513_dn15),
        );
        let d517_dn5: f64 = self.scalar_v1849;
        let d517_dn7: f64 = self.scalar_v514;
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (d517_dn5),
            nodes[7],
            multiplicity * (d517_dn7),
        );
        let d519_dn5: f64 = self.scalar_v518;
        let d519_dn8: f64 = self.scalar_v1850;
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * (d519_dn5),
            nodes[8],
            multiplicity * (d519_dn8),
        );
        let d522_dn4: f64 = v1851;
        let d522_dn6: f64 = v114;
        stamper.stamp_current_reactive_node2(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * (d522_dn4),
            nodes[6],
            multiplicity * (d522_dn6),
        );
        let d534_dn11: f64 = self.scalar_v531;
        let d534_dn14: f64 = self.scalar_v1857;
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[14]),
            nodes[11],
            multiplicity * (d534_dn11),
            nodes[14],
            multiplicity * (d534_dn14),
        );
        let mut locals = StampLocals::default();

        Self::stamp_reactive_block_0(ctx, p, nodes, param_given, &mut locals);
        Self::stamp_reactive_block_1(p, &mut locals);
        Self::stamp_reactive_block_2(p, &mut locals);
        Self::stamp_reactive_block_3(ctx, p, branches, &mut locals);

        Self::stamp_reactive_equations_block_0(ctx, stamper, p, nodes, branches, multiplicity, &mut locals);
    }
}
