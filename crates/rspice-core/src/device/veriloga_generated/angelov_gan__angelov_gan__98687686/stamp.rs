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
        let v77: f64 = (v46 * self.scalar_v76);
        let v78: f64 = (v44 + v77);
        let v79: f64 = (self.scalar_v75 * v78);
        let v80: f64 = (if v43 { v79 } else { v13 });
        let v83: f64 = (v39 * self.scalar_v82);
        let v84: f64 = (self.scalar_v81 + v83);
        let v85: f64 = (if v43 { v84 } else { v13 });
        let v97: f64 = (v39 * self.scalar_v96);
        let v98: f64 = (self.scalar_v95 + v97);
        let v99: f64 = (if v43 { v98 } else { v13 });
        let v102: f64 = (v39 * self.scalar_v101);
        let v103: f64 = (self.scalar_v100 + v102);
        let v104: f64 = (if v43 { v103 } else { v13 });
        let v113: bool = (v43 && self.scalar_v112);
        let v115: f64 = (v39 * v39);
        let v116: f64 = (self.scalar_v76 * v115);
        let v117: f64 = (v44 + v116);
        let v118: f64 = (self.scalar_v114 * v117);
        let v119: f64 = (if v113 { v118 } else { v13 });
        let v124: bool = (v43 && self.scalar_v123);
        let v125: f64 = (v78 * self.scalar_v114);
        let v126: f64 = (if v124 { v125 } else { v119 });
        let v129: bool = (!v43);
        let v130: f64 = (if v129 { self.scalar_v50 } else { v55 });
        let v131: f64 = (if v129 { self.scalar_v56 } else { v61 });
        let v132: f64 = (if v129 { self.scalar_v62 } else { v67 });
        let v134: f64 = (if v129 { self.scalar_v75 } else { v80 });
        let v135: f64 = (if v129 { self.scalar_v114 } else { v126 });
        let v137: f64 = (if v129 { self.scalar_v81 } else { v85 });
        let v140: f64 = (if v129 { self.scalar_v95 } else { v99 });
        let v141: f64 = (if v129 { self.scalar_v100 } else { v104 });
        let v146: f64 = 0.5;
        let v149: f64 = (self.scalar_v148 / v37);
        let v150: f64 = (if self.scalar_v145 { v149 } else { v13 });
        let v153: f64 = (if self.scalar_v151 { self.scalar_v152 } else { v150 });
        let v155: f64 = (v7 * self.scalar_v154);
        let v156: f64 = ((v155) as f64).cosh();
        let v158: f64 = (v11 * self.scalar_v157);
        let v161: f64 = 1e-12;
        let v162: f64 = (v156 * v156);
        let v163: f64 = (v161 + v162);
        let v164: f64 = (self.scalar_v160 / v163);
        let v165: f64 = (v44 + v164);
        let v166: f64 = (self.scalar_v159 * v165);
        let v168: f64 = (v46 * self.scalar_v167);
        let v169: f64 = (v44 + v168);
        let v170: f64 = (v166 * v169);
        let v173: f64 = (v46 * self.scalar_v172);
        let v174: f64 = (v44 + v173);
        let v175: f64 = (self.scalar_v171 * v174);
        let v177: f64 = (v137 - self.scalar_v176);
        let v179: f64 = (v7 * self.scalar_v178);
        let v180: f64 = ((v179) as f64).tanh();
        let v181: f64 = (self.scalar_v176 * v180);
        let v182: f64 = (v177 + v181);
        let v183: f64 = (v182 - v158);
        let v185: f64 = (v6 - v141);
        let v186: f64 = (self.scalar_v184 * v185);
        let v187: f64 = (v185 * v186);
        let v188: f64 = (v183 - v187);
        let v189: f64 = (v46 * self.scalar_v82);
        let v190: f64 = (v44 + v189);
        let v191: f64 = (v188 * v190);
        let v192: f64 = (v2 - v191);
        let v193: f64 = (v192 * v192);
        let v194: f64 = (v170 * v192);
        let v196: f64 = (v193 * self.scalar_v195);
        let v197: f64 = (v194 + v196);
        let v198: f64 = (v175 * v192);
        let v199: f64 = (v193 * v198);
        let v200: f64 = (v197 + v199);
        let v201: f64 = ((v200) as f64).tanh();
        let v202: f64 = (v44 + v201);
        let v203: f64 = { let limexp_arg = v200; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v204: f64 = (-v200);
        let v205: f64 = { let limexp_arg = v204; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v206: f64 = (v203 - v205);
        let v207: f64 = (v146 * v206);
        let v208: f64 = ((v207) as f64).tanh();
        let v209: f64 = (v44 + v208);
        let v211: f64 = (self.scalar_v178 * v202);
        let v212: f64 = (self.scalar_v210 + v211);
        let v213: f64 = (v7 * v212);
        let v214: f64 = ((v213) as f64).tanh();
        let v220: f64 = (v130 * v202);
        let v221: f64 = (v214 * v220);
        let v223: f64 = (v7 * self.scalar_v222);
        let v224: f64 = (v44 + v223);
        let v225: f64 = { let limexp_arg = v185; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v226: f64 = (v131 * v225);
        let v227: f64 = (v224 + v226);
        let v228: f64 = (v221 * v227);
        let v229: f64 = (if self.scalar_v215 { v228 } else { v13 });
        let v232: f64 = (v5 - v191);
        let v233: f64 = (if self.scalar_v231 { v232 } else { v156 });
        let v234: f64 = (v233 * v233);
        let v235: f64 = (if self.scalar_v231 { v234 } else { v192 });
        let v236: f64 = (v233 * v235);
        let v237: f64 = (if self.scalar_v231 { v236 } else { v193 });
        let v238: f64 = (v170 * v233);
        let v239: f64 = (self.scalar_v195 * v235);
        let v240: f64 = (v238 + v239);
        let v241: f64 = (v175 * v237);
        let v242: f64 = (v240 + v241);
        let v243: f64 = (if self.scalar_v231 { v242 } else { v13 });
        let v244: f64 = ((v243) as f64).tanh();
        let v245: f64 = (v44 + v244);
        let v246: f64 = (if self.scalar_v231 { v245 } else { v13 });
        let v247: f64 = (self.scalar_v178 * v246);
        let v248: f64 = (self.scalar_v210 + v247);
        let v249: f64 = (if self.scalar_v231 { v248 } else { v13 });
        let v251: f64 = (v202 * self.scalar_v250);
        let v252: f64 = (self.scalar_v222 + v251);
        let v253: f64 = (if self.scalar_v231 { v252 } else { v13 });
        let v254: f64 = (v44 + v214);
        let v255: f64 = (v220 * v254);
        let v256: f64 = (v7 * v253);
        let v257: f64 = (v44 + v256);
        let v259: f64 = (v7 - v141);
        let v260: f64 = (self.scalar_v258 * v259);
        let v261: f64 = { let limexp_arg = v260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v262: f64 = (v131 * v261);
        let v263: f64 = (v257 + v262);
        let v264: f64 = (v255 * v263);
        let v265: f64 = (if self.scalar_v231 { v264 } else { v13 });
        let v266: f64 = (v246 * self.scalar_v250);
        let v267: f64 = (self.scalar_v222 + v266);
        let v268: f64 = (if self.scalar_v231 { v267 } else { v13 });
        let v269: f64 = (v7 * v249);
        let v270: f64 = ((v269) as f64).tanh();
        let v271: f64 = (if self.scalar_v231 { v270 } else { v13 });
        let v272: f64 = (v130 * v246);
        let v273: f64 = (v44 - v271);
        let v274: f64 = (v272 * v273);
        let v275: f64 = (v7 * v268);
        let v276: f64 = (v44 - v275);
        let v277: f64 = (v274 * v276);
        let v278: f64 = (if self.scalar_v231 { v277 } else { v13 });
        let v279: f64 = (v265 - v278);
        let v280: f64 = (v146 * v279);
        let v281: f64 = (if self.scalar_v231 { v280 } else { v229 });
        let v285: f64 = (if self.scalar_v284 { v192 } else { v233 });
        let v286: f64 = (v285 * v285);
        let v287: f64 = (if self.scalar_v284 { v286 } else { v235 });
        let v288: f64 = (self.scalar_v195 * v287);
        let v289: f64 = (v285 + v288);
        let v290: f64 = (v175 * v287);
        let v291: f64 = (v285 * v290);
        let v292: f64 = (v289 + v291);
        let v293: f64 = (v170 * v292);
        let v294: f64 = (if self.scalar_v284 { v293 } else { v200 });
        let v295: f64 = { let limexp_arg = v294; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v296: f64 = (-v294);
        let v297: f64 = { let limexp_arg = v296; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v298: f64 = (v295 - v297);
        let v299: f64 = (v146 * v298);
        let v300: f64 = ((v299) as f64).tanh();
        let v301: f64 = (v44 + v300);
        let v302: f64 = (if self.scalar_v284 { v301 } else { v209 });
        let v303: f64 = (self.scalar_v178 * v302);
        let v304: f64 = (self.scalar_v210 + v303);
        let v305: f64 = (if self.scalar_v284 { v304 } else { v13 });
        let v306: f64 = (v7 * v305);
        let v307: f64 = ((v306) as f64).tanh();
        let v308: f64 = (if self.scalar_v284 { v307 } else { v13 });
        let v309: f64 = (self.scalar_v250 * v302);
        let v310: f64 = (self.scalar_v222 + v309);
        let v311: f64 = (if self.scalar_v284 { v310 } else { v253 });
        let v312: f64 = (v130 * v302);
        let v313: f64 = (v308 * v312);
        let v314: f64 = (v7 * v311);
        let v315: f64 = (v44 + v314);
        let v316: f64 = (v185 * self.scalar_v258);
        let v317: f64 = { let limexp_arg = v316; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v318: f64 = (v131 * v317);
        let v319: f64 = (v315 + v318);
        let v320: f64 = (v313 * v319);
        let v321: f64 = (if self.scalar_v284 { v320 } else { v281 });
        let v325: f64 = (if self.scalar_v324 { v192 } else { v285 });
        let v326: f64 = (v325 * v325);
        let v327: f64 = (if self.scalar_v324 { v326 } else { v287 });
        let v328: f64 = (self.scalar_v195 * v327);
        let v329: f64 = (v325 + v328);
        let v330: f64 = (v175 * v327);
        let v331: f64 = (v325 * v330);
        let v332: f64 = (v329 + v331);
        let v333: f64 = (v170 * v332);
        let v334: f64 = (if self.scalar_v324 { v333 } else { v294 });
        let v335: f64 = (if self.scalar_v324 { v232 } else { v237 });
        let v336: f64 = (v335 * v335);
        let v337: f64 = (if self.scalar_v324 { v336 } else { v13 });
        let v338: f64 = (self.scalar_v195 * v337);
        let v339: f64 = (v335 + v338);
        let v340: f64 = (v175 * v335);
        let v341: f64 = (v337 * v340);
        let v342: f64 = (v339 + v341);
        let v343: f64 = (v170 * v342);
        let v344: f64 = (if self.scalar_v324 { v343 } else { v243 });
        let v345: f64 = { let limexp_arg = v334; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v346: f64 = (-v334);
        let v347: f64 = { let limexp_arg = v346; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v348: f64 = (v345 - v347);
        let v349: f64 = (v146 * v348);
        let v350: f64 = ((v349) as f64).tanh();
        let v351: f64 = (v44 + v350);
        let v352: f64 = (if self.scalar_v324 { v351 } else { v302 });
        let v353: f64 = { let limexp_arg = v344; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v354: f64 = (-v344);
        let v355: f64 = { let limexp_arg = v354; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v356: f64 = (v353 - v355);
        let v357: f64 = (v146 * v356);
        let v358: f64 = ((v357) as f64).tanh();
        let v359: f64 = (v44 + v358);
        let v360: f64 = (if self.scalar_v324 { v359 } else { v13 });
        let v361: f64 = (self.scalar_v178 * v352);
        let v362: f64 = (self.scalar_v210 + v361);
        let v363: f64 = (if self.scalar_v324 { v362 } else { v305 });
        let v364: f64 = (self.scalar_v178 * v360);
        let v365: f64 = (self.scalar_v210 + v364);
        let v366: f64 = (if self.scalar_v324 { v365 } else { v13 });
        let v367: f64 = (v7 * v363);
        let v368: f64 = ((v367) as f64).tanh();
        let v369: f64 = (if self.scalar_v324 { v368 } else { v308 });
        let v370: f64 = (v7 * v366);
        let v371: f64 = ((v370) as f64).tanh();
        let v372: f64 = (if self.scalar_v324 { v371 } else { v13 });
        let v373: f64 = (self.scalar_v250 * v360);
        let v374: f64 = (self.scalar_v222 + v373);
        let v375: f64 = (if self.scalar_v324 { v374 } else { v13 });
        let v376: f64 = (self.scalar_v250 * v352);
        let v377: f64 = (self.scalar_v222 + v376);
        let v378: f64 = (if self.scalar_v324 { v377 } else { v13 });
        let v379: f64 = (v130 * v352);
        let v380: f64 = (v44 + v369);
        let v381: f64 = (v379 * v380);
        let v382: f64 = (v7 * v378);
        let v383: f64 = (v44 + v382);
        let v384: f64 = (v262 + v383);
        let v385: f64 = (v381 * v384);
        let v386: f64 = (if self.scalar_v324 { v385 } else { v265 });
        let v387: f64 = (v130 * v360);
        let v388: f64 = (v44 - v372);
        let v389: f64 = (v387 * v388);
        let v390: f64 = (v7 * v375);
        let v391: f64 = (v44 - v390);
        let v392: f64 = (v389 * v391);
        let v393: f64 = (if self.scalar_v324 { v392 } else { v278 });
        let v394: f64 = (v386 - v393);
        let v395: f64 = (v146 * v394);
        let v396: f64 = (if self.scalar_v324 { v395 } else { v321 });
        let v400: f64 = (if self.scalar_v399 { v252 } else { v311 });
        let v401: f64 = (if self.scalar_v399 { v362 } else { v363 });
        let v402: f64 = (v7 * v401);
        let v403: f64 = ((v402) as f64).tanh();
        let v404: f64 = (if self.scalar_v399 { v403 } else { v369 });
        let v405: f64 = (v11 * v401);
        let v406: f64 = ((v405) as f64).tanh();
        let v407: f64 = (if self.scalar_v399 { v406 } else { v13 });
        let v409: f64 = (v407 * self.scalar_v408);
        let v410: f64 = (v404 + v409);
        let v411: f64 = (v220 * v410);
        let v412: f64 = (v11 * self.scalar_v408);
        let v413: f64 = (v7 + v412);
        let v414: f64 = (v400 * v413);
        let v415: f64 = (v44 + v414);
        let v416: f64 = (v262 + v415);
        let v417: f64 = (v411 * v416);
        let v418: f64 = (if self.scalar_v399 { v417 } else { v396 });
        let v424: f64 = -1.0;
        let v425: f64 = (-v140);
        let v426: f64 = ((v425) as f64).tanh();
        let v427: f64 = (v153 * v426);
        let v428: f64 = { let limexp_arg = v427; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v429: f64 = (if self.scalar_v423 { v428 } else { v325 });
        let v430: f64 = (v9 - v140);
        let v431: f64 = (if self.scalar_v423 { v430 } else { v13 });
        let v432: f64 = (-v9);
        let v434: f64 = (v432 - self.scalar_v433);
        let v435: f64 = (if self.scalar_v423 { v434 } else { v13 });
        let v436: f64 = (v5 - v140);
        let v437: f64 = (if self.scalar_v423 { v436 } else { v13 });
        let v439: f64 = (v6 - self.scalar_v438);
        let v440: f64 = (if self.scalar_v423 { v439 } else { v13 });
        let v442: f64 = (-v153);
        let v443: f64 = (v140 * v442);
        let v444: f64 = { let limexp_arg = v443; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v445: f64 = (if self.scalar_v441 { v444 } else { v429 });
        let v456: f64 = ((v430) as f64).tanh();
        let v457: f64 = (if self.scalar_v455 { v456 } else { v431 });
        let v458: f64 = ((v436) as f64).tanh();
        let v459: f64 = (if self.scalar_v455 { v458 } else { v437 });
        let v462: f64 = (if self.scalar_v461 { v430 } else { v457 });
        let v463: f64 = (if self.scalar_v461 { v436 } else { v459 });
        let v464: f64 = (if self.scalar_v441 { v434 } else { v435 });
        let v465: f64 = (if self.scalar_v441 { v439 } else { v440 });
        let v466: f64 = (self.scalar_v446 * v464);
        let v467: f64 = { let limexp_arg = v466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v468: f64 = (v467 - self.scalar_v450);
        let v470: f64 = (v153 * v462);
        let v471: f64 = { let limexp_arg = v470; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v475: f64 = (v468 * self.scalar_v474);
        let v476: f64 = (v471 - v475);
        let v477: f64 = (v476 - v445);
        let v478: f64 = (self.scalar_v469 * v477);
        let v479: f64 = (self.scalar_v446 * v465);
        let v480: f64 = { let limexp_arg = v479; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v481: f64 = (v480 - self.scalar_v453);
        let v482: f64 = (v153 * v463);
        let v483: f64 = { let limexp_arg = v482; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v484: f64 = (self.scalar_v474 * v481);
        let v485: f64 = (v483 - v484);
        let v486: f64 = (v485 - v445);
        let v487: f64 = (self.scalar_v469 * v486);
        let v715: f64 = 5.5226012e-23;
        let v716: f64 = (v35 * v715);
        let v720: f64 = (v716 * self.scalar_v719);
        let v721: f64 = (v132 * v720);
        let v724: f64 = (v721 * self.scalar_v723);
        let v725: f64 = (if self.scalar_v714 { v724 } else { v13 });
        let v726: f64 = (v725 * v725);
        let v727: f64 = (v44 - v726);
        let v728: f64 = ((v727) as f64).sqrt();
        let v729: f64 = (if self.scalar_v714 { v728 } else { v13 });
        let v730: f64 = (-v725);
        let v731: f64 = 3.141592653589793;
        let v732: f64 = (v730 * v731);
        let v733: f64 = (if self.scalar_v714 { v732 } else { v13 });
        let v739: f64 = (-v418);
        let v741: f64 = nv15;
        let v742: f64 = (self.scalar_v740 * v741);
        let v755: f64 = nv7;
        let v756: f64 = (v755 - v4);
        let v757: f64 = (self.scalar_v754 * v756);
        let v759: f64 = (v7 * self.scalar_v758);
        let v760: f64 = nv6;
        let v761: f64 = (v760 - v10);
        let v762: f64 = (v134 * v761);
        let v763: f64 = (v161 * v761);
        let v766: f64 = (v8 - v0);
        let v767: f64 = (v766 / v135);
        let v768: f64 = (if self.scalar_v695 { v767 } else { v13 });
        let v775: f64 = nv14;
        let v776: f64 = (v8 - v775);
        let v777: f64 = (self.scalar_v774 * v776);
        let v778: f64 = (v775 - v1);
        let v779: f64 = (v778 / self.scalar_v696);
        let v780: f64 = (if self.scalar_v697 { v779 } else { v13 });
        let v783: f64 = nv13;
        let v784: f64 = (v783 - v3);
        let v785: f64 = (v784 / self.scalar_v698);
        let v786: f64 = (if self.scalar_v699 { v785 } else { v13 });
        let v790: f64 = (v783 - v8);
        let v791: f64 = (v790 / self.scalar_v700);
        let v792: f64 = (if self.scalar_v701 { v791 } else { v13 });
        let v807: f64 = 1e-15;
        let v808: f64 = nv2;
        let v809: f64 = (v0 - v808);
        let v810: f64 = (v161 * v809);
        let v814: f64 = nv17;
        let v815: f64 = (if self.scalar_v714 { v814 } else { v13 });
        let v816: f64 = nv18;
        let v817: f64 = (if self.scalar_v714 { v816 } else { v13 });
        let v818: f64 = (v733 * v814);
        let v819: f64 = (v729 * v816);
        let v820: f64 = (v818 + v819);
        let v821: f64 = (if self.scalar_v714 { v820 } else { v13 });
        let v830: f64 = (v7 * v418);
        let v831: f64 = ((v830) as f64).abs();
        let v832: f64 = (v9 * v478);
        let v833: f64 = ((v832) as f64).abs();
        let v834: f64 = (v831 + v833);
        let v835: f64 = (-v834);
        let v836: f64 = (if self.scalar_v738 { v835 } else { v13 });
        let v837: f64 = (v32 / v49);
        let v838: f64 = (if self.scalar_v738 { v837 } else { v13 });
        let v844: f64 = (v32 * v161);
        let v845: f64 = (if self.scalar_v843 { v844 } else { v13 });
        let v847: f64 = ((v155) as f64).sinh();
        let v848: f64 = (self.scalar_v154 * v847);
        let v849: f64 = (self.scalar_v846 * v847);
        let v851: f64 = (v156 * v848);
        let v852: f64 = (v851 + v851);
        let v853: f64 = (v156 * v849);
        let v854: f64 = (v853 + v853);
        let v855: f64 = (self.scalar_v160 * v852);
        let v856: f64 = (-v855);
        let v857: f64 = (v163 * v163);
        let v858: f64 = (v856 / v857);
        let v859: f64 = (self.scalar_v160 * v854);
        let v860: f64 = (-v859);
        let v861: f64 = (v860 / v857);
        let v862: f64 = (self.scalar_v159 * v858);
        let v863: f64 = (self.scalar_v159 * v861);
        let v864: f64 = (v169 * v862);
        let v865: f64 = (v169 * v863);
        let v867: f64 = (v180 * v180);
        let v868: f64 = (v44 - v867);
        let v869: f64 = (self.scalar_v178 * v868);
        let v870: f64 = (self.scalar_v866 * v868);
        let v871: f64 = (self.scalar_v176 * v869);
        let v872: f64 = (self.scalar_v176 * v870);
        let v873: f64 = (v872 - self.scalar_v850);
        let v875: f64 = (v186 + v186);
        let v876: f64 = (-v186);
        let v877: f64 = (v185 * self.scalar_v874);
        let v878: f64 = (v876 + v877);
        let v879: f64 = (v871 - v875);
        let v880: f64 = (-v878);
        let v881: f64 = (v190 * self.scalar_v850);
        let v882: f64 = (v190 * v879);
        let v883: f64 = (v190 * v873);
        let v884: f64 = (v190 * v880);
        let v885: f64 = (-v881);
        let v886: f64 = (-v882);
        let v887: f64 = (v424 - v883);
        let v888: f64 = (-v884);
        let v889: f64 = (v192 * v885);
        let v890: f64 = (v889 + v889);
        let v891: f64 = (v192 * v886);
        let v892: f64 = (v891 + v891);
        let v893: f64 = (v192 * v887);
        let v894: f64 = (v893 + v893);
        let v895: f64 = (v192 * v888);
        let v896: f64 = (v895 + v895);
        let v897: f64 = (v192 + v192);
        let v898: f64 = (v170 * v885);
        let v899: f64 = (v192 * v864);
        let v900: f64 = (v170 * v886);
        let v901: f64 = (v899 + v900);
        let v902: f64 = (v192 * v865);
        let v903: f64 = (v170 * v887);
        let v904: f64 = (v902 + v903);
        let v905: f64 = (v170 * v888);
        let v906: f64 = (self.scalar_v195 * v890);
        let v907: f64 = (self.scalar_v195 * v892);
        let v908: f64 = (self.scalar_v195 * v894);
        let v909: f64 = (self.scalar_v195 * v896);
        let v910: f64 = (self.scalar_v195 * v897);
        let v911: f64 = (v898 + v906);
        let v912: f64 = (v901 + v907);
        let v913: f64 = (v904 + v908);
        let v914: f64 = (v905 + v909);
        let v915: f64 = (v170 + v910);
        let v916: f64 = (v175 * v885);
        let v917: f64 = (v175 * v886);
        let v918: f64 = (v175 * v887);
        let v919: f64 = (v175 * v888);
        let v920: f64 = (v198 * v890);
        let v921: f64 = (v193 * v916);
        let v922: f64 = (v920 + v921);
        let v923: f64 = (v198 * v892);
        let v924: f64 = (v193 * v917);
        let v925: f64 = (v923 + v924);
        let v926: f64 = (v198 * v894);
        let v927: f64 = (v193 * v918);
        let v928: f64 = (v926 + v927);
        let v929: f64 = (v198 * v896);
        let v930: f64 = (v193 * v919);
        let v931: f64 = (v929 + v930);
        let v932: f64 = (v198 * v897);
        let v933: f64 = (v175 * v193);
        let v934: f64 = (v932 + v933);
        let v935: f64 = (v911 + v922);
        let v936: f64 = (v912 + v925);
        let v937: f64 = (v913 + v928);
        let v938: f64 = (v914 + v931);
        let v939: f64 = (v915 + v934);
        let v940: f64 = (v201 * v201);
        let v941: f64 = (v44 - v940);
        let v942: f64 = (v935 * v941);
        let v943: f64 = (v936 * v941);
        let v944: f64 = (v937 * v941);
        let v945: f64 = (v938 * v941);
        let v946: f64 = (v939 * v941);
        let v947: f64 = { let limexp_arg = v200; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v948: f64 = (v935 * v947);
        let v949: f64 = (v936 * v947);
        let v950: f64 = (v937 * v947);
        let v951: f64 = (v938 * v947);
        let v952: f64 = (v939 * v947);
        let v953: f64 = (-v935);
        let v954: f64 = (-v936);
        let v955: f64 = (-v937);
        let v956: f64 = (-v938);
        let v957: f64 = (-v939);
        let v958: f64 = { let limexp_arg = v204; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v959: f64 = (v953 * v958);
        let v960: f64 = (v954 * v958);
        let v961: f64 = (v955 * v958);
        let v962: f64 = (v956 * v958);
        let v963: f64 = (v957 * v958);
        let v964: f64 = (v948 - v959);
        let v965: f64 = (v949 - v960);
        let v966: f64 = (v950 - v961);
        let v967: f64 = (v951 - v962);
        let v968: f64 = (v952 - v963);
        let v969: f64 = (v146 * v964);
        let v970: f64 = (v146 * v965);
        let v971: f64 = (v146 * v966);
        let v972: f64 = (v146 * v967);
        let v973: f64 = (v146 * v968);
        let v974: f64 = (v208 * v208);
        let v975: f64 = (v44 - v974);
        let v976: f64 = (v969 * v975);
        let v977: f64 = (v970 * v975);
        let v978: f64 = (v971 * v975);
        let v979: f64 = (v972 * v975);
        let v980: f64 = (v973 * v975);
        let v981: f64 = (self.scalar_v178 * v942);
        let v982: f64 = (self.scalar_v178 * v943);
        let v983: f64 = (self.scalar_v178 * v944);
        let v984: f64 = (self.scalar_v178 * v945);
        let v985: f64 = (self.scalar_v178 * v946);
        let v986: f64 = (v7 * v981);
        let v987: f64 = (v7 * v982);
        let v988: f64 = (v212 + v987);
        let v989: f64 = (-v212);
        let v990: f64 = (v7 * v983);
        let v991: f64 = (v989 + v990);
        let v992: f64 = (v7 * v984);
        let v993: f64 = (v7 * v985);
        let v994: f64 = (v214 * v214);
        let v995: f64 = (v44 - v994);
        let v996: f64 = (v986 * v995);
        let v997: f64 = (v988 * v995);
        let v998: f64 = (v991 * v995);
        let v999: f64 = (v992 * v995);
        let v1000: f64 = (v993 * v995);
        let v1001: f64 = (v130 * v942);
        let v1002: f64 = (v130 * v943);
        let v1003: f64 = (v130 * v944);
        let v1004: f64 = (v130 * v945);
        let v1005: f64 = (v130 * v946);
        let v1006: f64 = (v220 * v996);
        let v1007: f64 = (v214 * v1001);
        let v1008: f64 = (v1006 + v1007);
        let v1009: f64 = (v220 * v997);
        let v1010: f64 = (v214 * v1002);
        let v1011: f64 = (v1009 + v1010);
        let v1012: f64 = (v220 * v998);
        let v1013: f64 = (v214 * v1003);
        let v1014: f64 = (v1012 + v1013);
        let v1015: f64 = (v220 * v999);
        let v1016: f64 = (v214 * v1004);
        let v1017: f64 = (v1015 + v1016);
        let v1018: f64 = (v220 * v1000);
        let v1019: f64 = (v214 * v1005);
        let v1020: f64 = (v1018 + v1019);
        let v1022: f64 = { let limexp_arg = v185; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1023: f64 = (-v1022);
        let v1024: f64 = (v131 * v1022);
        let v1025: f64 = (v131 * v1023);
        let v1026: f64 = (self.scalar_v222 + v1024);
        let v1027: f64 = (v227 * v1008);
        let v1028: f64 = (v227 * v1011);
        let v1029: f64 = (v221 * v1026);
        let v1030: f64 = (v1028 + v1029);
        let v1031: f64 = (v227 * v1014);
        let v1032: f64 = (v221 * self.scalar_v1021);
        let v1033: f64 = (v1031 + v1032);
        let v1034: f64 = (v227 * v1017);
        let v1035: f64 = (v221 * v1025);
        let v1036: f64 = (v1034 + v1035);
        let v1037: f64 = (v227 * v1020);
        let v1038: f64 = (if self.scalar_v215 { v1027 } else { v13 });
        let v1039: f64 = (if self.scalar_v215 { v1030 } else { v13 });
        let v1040: f64 = (if self.scalar_v215 { v1033 } else { v13 });
        let v1041: f64 = (if self.scalar_v215 { v1036 } else { v13 });
        let v1042: f64 = (if self.scalar_v215 { v1037 } else { v13 });
        let v1043: f64 = (v424 - v882);
        let v1044: f64 = (-v883);
        let v1045: f64 = (v44 - v884);
        let v1046: f64 = (if self.scalar_v231 { v885 } else { v13 });
        let v1047: f64 = (if self.scalar_v231 { v1043 } else { v848 });
        let v1048: f64 = (if self.scalar_v231 { v1044 } else { v849 });
        let v1049: f64 = (if self.scalar_v231 { v1045 } else { v13 });
        let v1050: f64 = (v233 * v1046);
        let v1051: f64 = (v1050 + v1050);
        let v1052: f64 = (v233 * v1047);
        let v1053: f64 = (v1052 + v1052);
        let v1054: f64 = (v233 * v1048);
        let v1055: f64 = (v1054 + v1054);
        let v1056: f64 = (v233 * v1049);
        let v1057: f64 = (v1056 + v1056);
        let v1058: f64 = (if self.scalar_v231 { v1051 } else { v885 });
        let v1059: f64 = (if self.scalar_v231 { v1053 } else { v886 });
        let v1060: f64 = (if self.scalar_v231 { v1055 } else { v887 });
        let v1061: f64 = (if self.scalar_v231 { v1057 } else { v888 });
        let v1063: f64 = (v235 * v1046);
        let v1064: f64 = (v233 * v1058);
        let v1065: f64 = (v1063 + v1064);
        let v1066: f64 = (v235 * v1047);
        let v1067: f64 = (v233 * v1059);
        let v1068: f64 = (v1066 + v1067);
        let v1069: f64 = (v235 * v1048);
        let v1070: f64 = (v233 * v1060);
        let v1071: f64 = (v1069 + v1070);
        let v1072: f64 = (v235 * v1049);
        let v1073: f64 = (v233 * v1061);
        let v1074: f64 = (v1072 + v1073);
        let v1075: f64 = (v233 * self.scalar_v1062);
        let v1076: f64 = (if self.scalar_v231 { v1065 } else { v890 });
        let v1077: f64 = (if self.scalar_v231 { v1068 } else { v892 });
        let v1078: f64 = (if self.scalar_v231 { v1071 } else { v894 });
        let v1079: f64 = (if self.scalar_v231 { v1074 } else { v896 });
        let v1080: f64 = (if self.scalar_v231 { v1075 } else { v897 });
        let v1081: f64 = (v170 * v1046);
        let v1082: f64 = (v233 * v864);
        let v1083: f64 = (v170 * v1047);
        let v1084: f64 = (v1082 + v1083);
        let v1085: f64 = (v233 * v865);
        let v1086: f64 = (v170 * v1048);
        let v1087: f64 = (v1085 + v1086);
        let v1088: f64 = (v170 * v1049);
        let v1089: f64 = (self.scalar_v195 * v1058);
        let v1090: f64 = (self.scalar_v195 * v1059);
        let v1091: f64 = (self.scalar_v195 * v1060);
        let v1092: f64 = (self.scalar_v195 * v1061);
        let v1094: f64 = (v1081 + v1089);
        let v1095: f64 = (v1084 + v1090);
        let v1096: f64 = (v1087 + v1091);
        let v1097: f64 = (v1088 + v1092);
        let v1098: f64 = (v175 * v1076);
        let v1099: f64 = (v175 * v1077);
        let v1100: f64 = (v175 * v1078);
        let v1101: f64 = (v175 * v1079);
        let v1102: f64 = (v175 * v1080);
        let v1103: f64 = (v1094 + v1098);
        let v1104: f64 = (v1095 + v1099);
        let v1105: f64 = (v1096 + v1100);
        let v1106: f64 = (v1097 + v1101);
        let v1107: f64 = (self.scalar_v1093 + v1102);
        let v1108: f64 = (if self.scalar_v231 { v1103 } else { v13 });
        let v1109: f64 = (if self.scalar_v231 { v1104 } else { v13 });
        let v1110: f64 = (if self.scalar_v231 { v1105 } else { v13 });
        let v1111: f64 = (if self.scalar_v231 { v1106 } else { v13 });
        let v1112: f64 = (if self.scalar_v231 { v1107 } else { v13 });
        let v1113: f64 = (v244 * v244);
        let v1114: f64 = (v44 - v1113);
        let v1115: f64 = (v1108 * v1114);
        let v1116: f64 = (v1109 * v1114);
        let v1117: f64 = (v1110 * v1114);
        let v1118: f64 = (v1111 * v1114);
        let v1119: f64 = (v1112 * v1114);
        let v1120: f64 = (if self.scalar_v231 { v1115 } else { v13 });
        let v1121: f64 = (if self.scalar_v231 { v1116 } else { v13 });
        let v1122: f64 = (if self.scalar_v231 { v1117 } else { v13 });
        let v1123: f64 = (if self.scalar_v231 { v1118 } else { v13 });
        let v1124: f64 = (if self.scalar_v231 { v1119 } else { v13 });
        let v1125: f64 = (self.scalar_v178 * v1120);
        let v1126: f64 = (self.scalar_v178 * v1121);
        let v1127: f64 = (self.scalar_v178 * v1122);
        let v1128: f64 = (self.scalar_v178 * v1123);
        let v1129: f64 = (self.scalar_v178 * v1124);
        let v1130: f64 = (if self.scalar_v231 { v1125 } else { v13 });
        let v1131: f64 = (if self.scalar_v231 { v1126 } else { v13 });
        let v1132: f64 = (if self.scalar_v231 { v1127 } else { v13 });
        let v1133: f64 = (if self.scalar_v231 { v1128 } else { v13 });
        let v1134: f64 = (if self.scalar_v231 { v1129 } else { v13 });
        let v1135: f64 = (self.scalar_v250 * v942);
        let v1136: f64 = (self.scalar_v250 * v943);
        let v1137: f64 = (self.scalar_v250 * v944);
        let v1138: f64 = (self.scalar_v250 * v945);
        let v1139: f64 = (self.scalar_v250 * v946);
        let v1140: f64 = (if self.scalar_v231 { v1135 } else { v13 });
        let v1141: f64 = (if self.scalar_v231 { v1136 } else { v13 });
        let v1142: f64 = (if self.scalar_v231 { v1137 } else { v13 });
        let v1143: f64 = (if self.scalar_v231 { v1138 } else { v13 });
        let v1144: f64 = (if self.scalar_v231 { v1139 } else { v13 });
        let v1145: f64 = (v254 * v1001);
        let v1146: f64 = (v1006 + v1145);
        let v1147: f64 = (v254 * v1002);
        let v1148: f64 = (v1009 + v1147);
        let v1149: f64 = (v254 * v1003);
        let v1150: f64 = (v1012 + v1149);
        let v1151: f64 = (v254 * v1004);
        let v1152: f64 = (v1015 + v1151);
        let v1153: f64 = (v254 * v1005);
        let v1154: f64 = (v1018 + v1153);
        let v1155: f64 = (v7 * v1140);
        let v1156: f64 = (v7 * v1141);
        let v1157: f64 = (v253 + v1156);
        let v1158: f64 = (-v253);
        let v1159: f64 = (v7 * v1142);
        let v1160: f64 = (v1158 + v1159);
        let v1161: f64 = (v7 * v1143);
        let v1162: f64 = (v7 * v1144);
        let v1164: f64 = { let limexp_arg = v260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1165: f64 = (self.scalar_v258 * v1164);
        let v1166: f64 = (self.scalar_v1163 * v1164);
        let v1167: f64 = (v131 * v1165);
        let v1168: f64 = (v131 * v1166);
        let v1169: f64 = (v1157 + v1167);
        let v1170: f64 = (v1160 + v1168);
        let v1171: f64 = (v263 * v1146);
        let v1172: f64 = (v255 * v1155);
        let v1173: f64 = (v1171 + v1172);
        let v1174: f64 = (v263 * v1148);
        let v1175: f64 = (v255 * v1169);
        let v1176: f64 = (v1174 + v1175);
        let v1177: f64 = (v263 * v1150);
        let v1178: f64 = (v255 * v1170);
        let v1179: f64 = (v1177 + v1178);
        let v1180: f64 = (v263 * v1152);
        let v1181: f64 = (v255 * v1161);
        let v1182: f64 = (v1180 + v1181);
        let v1183: f64 = (v263 * v1154);
        let v1184: f64 = (v255 * v1162);
        let v1185: f64 = (v1183 + v1184);
        let v1186: f64 = (if self.scalar_v231 { v1173 } else { v13 });
        let v1187: f64 = (if self.scalar_v231 { v1176 } else { v13 });
        let v1188: f64 = (if self.scalar_v231 { v1179 } else { v13 });
        let v1189: f64 = (if self.scalar_v231 { v1182 } else { v13 });
        let v1190: f64 = (if self.scalar_v231 { v1185 } else { v13 });
        let v1191: f64 = (self.scalar_v250 * v1120);
        let v1192: f64 = (self.scalar_v250 * v1121);
        let v1193: f64 = (self.scalar_v250 * v1122);
        let v1194: f64 = (self.scalar_v250 * v1123);
        let v1195: f64 = (self.scalar_v250 * v1124);
        let v1196: f64 = (if self.scalar_v231 { v1191 } else { v13 });
        let v1197: f64 = (if self.scalar_v231 { v1192 } else { v13 });
        let v1198: f64 = (if self.scalar_v231 { v1193 } else { v13 });
        let v1199: f64 = (if self.scalar_v231 { v1194 } else { v13 });
        let v1200: f64 = (if self.scalar_v231 { v1195 } else { v13 });
        let v1201: f64 = (v7 * v1130);
        let v1202: f64 = (v7 * v1131);
        let v1203: f64 = (v249 + v1202);
        let v1204: f64 = (-v249);
        let v1205: f64 = (v7 * v1132);
        let v1206: f64 = (v1204 + v1205);
        let v1207: f64 = (v7 * v1133);
        let v1208: f64 = (v7 * v1134);
        let v1209: f64 = (v270 * v270);
        let v1210: f64 = (v44 - v1209);
        let v1211: f64 = (v1201 * v1210);
        let v1212: f64 = (v1203 * v1210);
        let v1213: f64 = (v1206 * v1210);
        let v1214: f64 = (v1207 * v1210);
        let v1215: f64 = (v1208 * v1210);
        let v1216: f64 = (if self.scalar_v231 { v1211 } else { v13 });
        let v1217: f64 = (if self.scalar_v231 { v1212 } else { v13 });
        let v1218: f64 = (if self.scalar_v231 { v1213 } else { v13 });
        let v1219: f64 = (if self.scalar_v231 { v1214 } else { v13 });
        let v1220: f64 = (if self.scalar_v231 { v1215 } else { v13 });
        let v1221: f64 = (v130 * v1120);
        let v1222: f64 = (v130 * v1121);
        let v1223: f64 = (v130 * v1122);
        let v1224: f64 = (v130 * v1123);
        let v1225: f64 = (v130 * v1124);
        let v1226: f64 = (-v1216);
        let v1227: f64 = (-v1217);
        let v1228: f64 = (-v1218);
        let v1229: f64 = (-v1219);
        let v1230: f64 = (-v1220);
        let v1231: f64 = (v273 * v1221);
        let v1232: f64 = (v272 * v1226);
        let v1233: f64 = (v1231 + v1232);
        let v1234: f64 = (v273 * v1222);
        let v1235: f64 = (v272 * v1227);
        let v1236: f64 = (v1234 + v1235);
        let v1237: f64 = (v273 * v1223);
        let v1238: f64 = (v272 * v1228);
        let v1239: f64 = (v1237 + v1238);
        let v1240: f64 = (v273 * v1224);
        let v1241: f64 = (v272 * v1229);
        let v1242: f64 = (v1240 + v1241);
        let v1243: f64 = (v273 * v1225);
        let v1244: f64 = (v272 * v1230);
        let v1245: f64 = (v1243 + v1244);
        let v1246: f64 = (v7 * v1196);
        let v1247: f64 = (v7 * v1197);
        let v1248: f64 = (v268 + v1247);
        let v1249: f64 = (-v268);
        let v1250: f64 = (v7 * v1198);
        let v1251: f64 = (v1249 + v1250);
        let v1252: f64 = (v7 * v1199);
        let v1253: f64 = (v7 * v1200);
        let v1254: f64 = (-v1246);
        let v1255: f64 = (-v1248);
        let v1256: f64 = (-v1251);
        let v1257: f64 = (-v1252);
        let v1258: f64 = (-v1253);
        let v1259: f64 = (v276 * v1233);
        let v1260: f64 = (v274 * v1254);
        let v1261: f64 = (v1259 + v1260);
        let v1262: f64 = (v276 * v1236);
        let v1263: f64 = (v274 * v1255);
        let v1264: f64 = (v1262 + v1263);
        let v1265: f64 = (v276 * v1239);
        let v1266: f64 = (v274 * v1256);
        let v1267: f64 = (v1265 + v1266);
        let v1268: f64 = (v276 * v1242);
        let v1269: f64 = (v274 * v1257);
        let v1270: f64 = (v1268 + v1269);
        let v1271: f64 = (v276 * v1245);
        let v1272: f64 = (v274 * v1258);
        let v1273: f64 = (v1271 + v1272);
        let v1274: f64 = (if self.scalar_v231 { v1261 } else { v13 });
        let v1275: f64 = (if self.scalar_v231 { v1264 } else { v13 });
        let v1276: f64 = (if self.scalar_v231 { v1267 } else { v13 });
        let v1277: f64 = (if self.scalar_v231 { v1270 } else { v13 });
        let v1278: f64 = (if self.scalar_v231 { v1273 } else { v13 });
        let v1279: f64 = (v1186 - v1274);
        let v1280: f64 = (v1187 - v1275);
        let v1281: f64 = (v1188 - v1276);
        let v1282: f64 = (v1189 - v1277);
        let v1283: f64 = (v1190 - v1278);
        let v1284: f64 = (v146 * v1279);
        let v1285: f64 = (v146 * v1280);
        let v1286: f64 = (v146 * v1281);
        let v1287: f64 = (v146 * v1282);
        let v1288: f64 = (v146 * v1283);
        let v1289: f64 = (if self.scalar_v231 { v1284 } else { v1038 });
        let v1290: f64 = (if self.scalar_v231 { v1285 } else { v1039 });
        let v1291: f64 = (if self.scalar_v231 { v1286 } else { v1040 });
        let v1292: f64 = (if self.scalar_v231 { v1287 } else { v1041 });
        let v1293: f64 = (if self.scalar_v231 { v1288 } else { v1042 });
        let v1294: f64 = (if self.scalar_v284 { v885 } else { v1046 });
        let v1295: f64 = (if self.scalar_v284 { v886 } else { v1047 });
        let v1296: f64 = (if self.scalar_v284 { v887 } else { v1048 });
        let v1297: f64 = (if self.scalar_v284 { v888 } else { v1049 });
        let v1299: f64 = (v285 * v1294);
        let v1300: f64 = (v1299 + v1299);
        let v1301: f64 = (v285 * v1295);
        let v1302: f64 = (v1301 + v1301);
        let v1303: f64 = (v285 * v1296);
        let v1304: f64 = (v1303 + v1303);
        let v1305: f64 = (v285 * v1297);
        let v1306: f64 = (v1305 + v1305);
        let v1307: f64 = (v285 * self.scalar_v1298);
        let v1308: f64 = (v1307 + v1307);
        let v1309: f64 = (if self.scalar_v284 { v1300 } else { v1058 });
        let v1310: f64 = (if self.scalar_v284 { v1302 } else { v1059 });
        let v1311: f64 = (if self.scalar_v284 { v1304 } else { v1060 });
        let v1312: f64 = (if self.scalar_v284 { v1306 } else { v1061 });
        let v1313: f64 = (if self.scalar_v284 { v1308 } else { self.scalar_v1062 });
        let v1314: f64 = (self.scalar_v195 * v1309);
        let v1315: f64 = (self.scalar_v195 * v1310);
        let v1316: f64 = (self.scalar_v195 * v1311);
        let v1317: f64 = (self.scalar_v195 * v1312);
        let v1318: f64 = (self.scalar_v195 * v1313);
        let v1319: f64 = (v1294 + v1314);
        let v1320: f64 = (v1295 + v1315);
        let v1321: f64 = (v1296 + v1316);
        let v1322: f64 = (v1297 + v1317);
        let v1323: f64 = (self.scalar_v1298 + v1318);
        let v1324: f64 = (v175 * v1309);
        let v1325: f64 = (v175 * v1310);
        let v1326: f64 = (v175 * v1311);
        let v1327: f64 = (v175 * v1312);
        let v1328: f64 = (v175 * v1313);
        let v1329: f64 = (v290 * v1294);
        let v1330: f64 = (v285 * v1324);
        let v1331: f64 = (v1329 + v1330);
        let v1332: f64 = (v290 * v1295);
        let v1333: f64 = (v285 * v1325);
        let v1334: f64 = (v1332 + v1333);
        let v1335: f64 = (v290 * v1296);
        let v1336: f64 = (v285 * v1326);
        let v1337: f64 = (v1335 + v1336);
        let v1338: f64 = (v290 * v1297);
        let v1339: f64 = (v285 * v1327);
        let v1340: f64 = (v1338 + v1339);
        let v1341: f64 = (v290 * self.scalar_v1298);
        let v1342: f64 = (v285 * v1328);
        let v1343: f64 = (v1341 + v1342);
        let v1344: f64 = (v1319 + v1331);
        let v1345: f64 = (v1320 + v1334);
        let v1346: f64 = (v1321 + v1337);
        let v1347: f64 = (v1322 + v1340);
        let v1348: f64 = (v1323 + v1343);
        let v1349: f64 = (v170 * v1344);
        let v1350: f64 = (v292 * v864);
        let v1351: f64 = (v170 * v1345);
        let v1352: f64 = (v1350 + v1351);
        let v1353: f64 = (v292 * v865);
        let v1354: f64 = (v170 * v1346);
        let v1355: f64 = (v1353 + v1354);
        let v1356: f64 = (v170 * v1347);
        let v1357: f64 = (v170 * v1348);
        let v1358: f64 = (if self.scalar_v284 { v1349 } else { v935 });
        let v1359: f64 = (if self.scalar_v284 { v1352 } else { v936 });
        let v1360: f64 = (if self.scalar_v284 { v1355 } else { v937 });
        let v1361: f64 = (if self.scalar_v284 { v1356 } else { v938 });
        let v1362: f64 = (if self.scalar_v284 { v1357 } else { v939 });
        let v1363: f64 = { let limexp_arg = v294; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1364: f64 = (v1358 * v1363);
        let v1365: f64 = (v1359 * v1363);
        let v1366: f64 = (v1360 * v1363);
        let v1367: f64 = (v1361 * v1363);
        let v1368: f64 = (v1362 * v1363);
        let v1369: f64 = (-v1358);
        let v1370: f64 = (-v1359);
        let v1371: f64 = (-v1360);
        let v1372: f64 = (-v1361);
        let v1373: f64 = (-v1362);
        let v1374: f64 = { let limexp_arg = v296; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1375: f64 = (v1369 * v1374);
        let v1376: f64 = (v1370 * v1374);
        let v1377: f64 = (v1371 * v1374);
        let v1378: f64 = (v1372 * v1374);
        let v1379: f64 = (v1373 * v1374);
        let v1380: f64 = (v1364 - v1375);
        let v1381: f64 = (v1365 - v1376);
        let v1382: f64 = (v1366 - v1377);
        let v1383: f64 = (v1367 - v1378);
        let v1384: f64 = (v1368 - v1379);
        let v1385: f64 = (v146 * v1380);
        let v1386: f64 = (v146 * v1381);
        let v1387: f64 = (v146 * v1382);
        let v1388: f64 = (v146 * v1383);
        let v1389: f64 = (v146 * v1384);
        let v1390: f64 = (v300 * v300);
        let v1391: f64 = (v44 - v1390);
        let v1392: f64 = (v1385 * v1391);
        let v1393: f64 = (v1386 * v1391);
        let v1394: f64 = (v1387 * v1391);
        let v1395: f64 = (v1388 * v1391);
        let v1396: f64 = (v1389 * v1391);
        let v1397: f64 = (if self.scalar_v284 { v1392 } else { v976 });
        let v1398: f64 = (if self.scalar_v284 { v1393 } else { v977 });
        let v1399: f64 = (if self.scalar_v284 { v1394 } else { v978 });
        let v1400: f64 = (if self.scalar_v284 { v1395 } else { v979 });
        let v1401: f64 = (if self.scalar_v284 { v1396 } else { v980 });
        let v1402: f64 = (self.scalar_v178 * v1397);
        let v1403: f64 = (self.scalar_v178 * v1398);
        let v1404: f64 = (self.scalar_v178 * v1399);
        let v1405: f64 = (self.scalar_v178 * v1400);
        let v1406: f64 = (self.scalar_v178 * v1401);
        let v1407: f64 = (if self.scalar_v284 { v1402 } else { v13 });
        let v1408: f64 = (if self.scalar_v284 { v1403 } else { v13 });
        let v1409: f64 = (if self.scalar_v284 { v1404 } else { v13 });
        let v1410: f64 = (if self.scalar_v284 { v1405 } else { v13 });
        let v1411: f64 = (if self.scalar_v284 { v1406 } else { v13 });
        let v1412: f64 = (v7 * v1407);
        let v1413: f64 = (v7 * v1408);
        let v1414: f64 = (v305 + v1413);
        let v1415: f64 = (-v305);
        let v1416: f64 = (v7 * v1409);
        let v1417: f64 = (v1415 + v1416);
        let v1418: f64 = (v7 * v1410);
        let v1419: f64 = (v7 * v1411);
        let v1420: f64 = (v307 * v307);
        let v1421: f64 = (v44 - v1420);
        let v1422: f64 = (v1412 * v1421);
        let v1423: f64 = (v1414 * v1421);
        let v1424: f64 = (v1417 * v1421);
        let v1425: f64 = (v1418 * v1421);
        let v1426: f64 = (v1419 * v1421);
        let v1427: f64 = (if self.scalar_v284 { v1422 } else { v13 });
        let v1428: f64 = (if self.scalar_v284 { v1423 } else { v13 });
        let v1429: f64 = (if self.scalar_v284 { v1424 } else { v13 });
        let v1430: f64 = (if self.scalar_v284 { v1425 } else { v13 });
        let v1431: f64 = (if self.scalar_v284 { v1426 } else { v13 });
        let v1432: f64 = (self.scalar_v250 * v1397);
        let v1433: f64 = (self.scalar_v250 * v1398);
        let v1434: f64 = (self.scalar_v250 * v1399);
        let v1435: f64 = (self.scalar_v250 * v1400);
        let v1436: f64 = (self.scalar_v250 * v1401);
        let v1437: f64 = (if self.scalar_v284 { v1432 } else { v1140 });
        let v1438: f64 = (if self.scalar_v284 { v1433 } else { v1141 });
        let v1439: f64 = (if self.scalar_v284 { v1434 } else { v1142 });
        let v1440: f64 = (if self.scalar_v284 { v1435 } else { v1143 });
        let v1441: f64 = (if self.scalar_v284 { v1436 } else { v1144 });
        let v1442: f64 = (v130 * v1397);
        let v1443: f64 = (v130 * v1398);
        let v1444: f64 = (v130 * v1399);
        let v1445: f64 = (v130 * v1400);
        let v1446: f64 = (v130 * v1401);
        let v1447: f64 = (v312 * v1427);
        let v1448: f64 = (v308 * v1442);
        let v1449: f64 = (v1447 + v1448);
        let v1450: f64 = (v312 * v1428);
        let v1451: f64 = (v308 * v1443);
        let v1452: f64 = (v1450 + v1451);
        let v1453: f64 = (v312 * v1429);
        let v1454: f64 = (v308 * v1444);
        let v1455: f64 = (v1453 + v1454);
        let v1456: f64 = (v312 * v1430);
        let v1457: f64 = (v308 * v1445);
        let v1458: f64 = (v1456 + v1457);
        let v1459: f64 = (v312 * v1431);
        let v1460: f64 = (v308 * v1446);
        let v1461: f64 = (v1459 + v1460);
        let v1462: f64 = (v7 * v1437);
        let v1463: f64 = (v7 * v1438);
        let v1464: f64 = (v311 + v1463);
        let v1465: f64 = (-v311);
        let v1466: f64 = (v7 * v1439);
        let v1467: f64 = (v1465 + v1466);
        let v1468: f64 = (v7 * v1440);
        let v1469: f64 = (v7 * v1441);
        let v1470: f64 = { let limexp_arg = v316; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1471: f64 = (self.scalar_v258 * v1470);
        let v1472: f64 = (self.scalar_v1163 * v1470);
        let v1473: f64 = (v131 * v1471);
        let v1474: f64 = (v131 * v1472);
        let v1475: f64 = (v1464 + v1473);
        let v1476: f64 = (v1468 + v1474);
        let v1477: f64 = (v319 * v1449);
        let v1478: f64 = (v313 * v1462);
        let v1479: f64 = (v1477 + v1478);
        let v1480: f64 = (v319 * v1452);
        let v1481: f64 = (v313 * v1475);
        let v1482: f64 = (v1480 + v1481);
        let v1483: f64 = (v319 * v1455);
        let v1484: f64 = (v313 * v1467);
        let v1485: f64 = (v1483 + v1484);
        let v1486: f64 = (v319 * v1458);
        let v1487: f64 = (v313 * v1476);
        let v1488: f64 = (v1486 + v1487);
        let v1489: f64 = (v319 * v1461);
        let v1490: f64 = (v313 * v1469);
        let v1491: f64 = (v1489 + v1490);
        let v1492: f64 = (if self.scalar_v284 { v1479 } else { v1289 });
        let v1493: f64 = (if self.scalar_v284 { v1482 } else { v1290 });
        let v1494: f64 = (if self.scalar_v284 { v1485 } else { v1291 });
        let v1495: f64 = (if self.scalar_v284 { v1488 } else { v1292 });
        let v1496: f64 = (if self.scalar_v284 { v1491 } else { v1293 });
        let v1497: f64 = (if self.scalar_v324 { v885 } else { v1294 });
        let v1498: f64 = (if self.scalar_v324 { v886 } else { v1295 });
        let v1499: f64 = (if self.scalar_v324 { v887 } else { v1296 });
        let v1500: f64 = (if self.scalar_v324 { v888 } else { v1297 });
        let v1502: f64 = (v325 * v1497);
        let v1503: f64 = (v1502 + v1502);
        let v1504: f64 = (v325 * v1498);
        let v1505: f64 = (v1504 + v1504);
        let v1506: f64 = (v325 * v1499);
        let v1507: f64 = (v1506 + v1506);
        let v1508: f64 = (v325 * v1500);
        let v1509: f64 = (v1508 + v1508);
        let v1510: f64 = (v325 * self.scalar_v1501);
        let v1511: f64 = (v1510 + v1510);
        let v1512: f64 = (if self.scalar_v324 { v1503 } else { v1309 });
        let v1513: f64 = (if self.scalar_v324 { v1505 } else { v1310 });
        let v1514: f64 = (if self.scalar_v324 { v1507 } else { v1311 });
        let v1515: f64 = (if self.scalar_v324 { v1509 } else { v1312 });
        let v1516: f64 = (if self.scalar_v324 { v1511 } else { v1313 });
        let v1517: f64 = (self.scalar_v195 * v1512);
        let v1518: f64 = (self.scalar_v195 * v1513);
        let v1519: f64 = (self.scalar_v195 * v1514);
        let v1520: f64 = (self.scalar_v195 * v1515);
        let v1521: f64 = (self.scalar_v195 * v1516);
        let v1522: f64 = (v1497 + v1517);
        let v1523: f64 = (v1498 + v1518);
        let v1524: f64 = (v1499 + v1519);
        let v1525: f64 = (v1500 + v1520);
        let v1526: f64 = (self.scalar_v1501 + v1521);
        let v1527: f64 = (v175 * v1512);
        let v1528: f64 = (v175 * v1513);
        let v1529: f64 = (v175 * v1514);
        let v1530: f64 = (v175 * v1515);
        let v1531: f64 = (v175 * v1516);
        let v1532: f64 = (v330 * v1497);
        let v1533: f64 = (v325 * v1527);
        let v1534: f64 = (v1532 + v1533);
        let v1535: f64 = (v330 * v1498);
        let v1536: f64 = (v325 * v1528);
        let v1537: f64 = (v1535 + v1536);
        let v1538: f64 = (v330 * v1499);
        let v1539: f64 = (v325 * v1529);
        let v1540: f64 = (v1538 + v1539);
        let v1541: f64 = (v330 * v1500);
        let v1542: f64 = (v325 * v1530);
        let v1543: f64 = (v1541 + v1542);
        let v1544: f64 = (v330 * self.scalar_v1501);
        let v1545: f64 = (v325 * v1531);
        let v1546: f64 = (v1544 + v1545);
        let v1547: f64 = (v1522 + v1534);
        let v1548: f64 = (v1523 + v1537);
        let v1549: f64 = (v1524 + v1540);
        let v1550: f64 = (v1525 + v1543);
        let v1551: f64 = (v1526 + v1546);
        let v1552: f64 = (v170 * v1547);
        let v1553: f64 = (v332 * v864);
        let v1554: f64 = (v170 * v1548);
        let v1555: f64 = (v1553 + v1554);
        let v1556: f64 = (v332 * v865);
        let v1557: f64 = (v170 * v1549);
        let v1558: f64 = (v1556 + v1557);
        let v1559: f64 = (v170 * v1550);
        let v1560: f64 = (v170 * v1551);
        let v1561: f64 = (if self.scalar_v324 { v1552 } else { v1358 });
        let v1562: f64 = (if self.scalar_v324 { v1555 } else { v1359 });
        let v1563: f64 = (if self.scalar_v324 { v1558 } else { v1360 });
        let v1564: f64 = (if self.scalar_v324 { v1559 } else { v1361 });
        let v1565: f64 = (if self.scalar_v324 { v1560 } else { v1362 });
        let v1566: f64 = (if self.scalar_v324 { v885 } else { v1076 });
        let v1567: f64 = (if self.scalar_v324 { v1043 } else { v1077 });
        let v1568: f64 = (if self.scalar_v324 { v1044 } else { v1078 });
        let v1569: f64 = (if self.scalar_v324 { v1045 } else { v1079 });
        let v1570: f64 = (if self.scalar_v324 { v13 } else { v1080 });
        let v1571: f64 = (v335 * v1566);
        let v1572: f64 = (v1571 + v1571);
        let v1573: f64 = (v335 * v1567);
        let v1574: f64 = (v1573 + v1573);
        let v1575: f64 = (v335 * v1568);
        let v1576: f64 = (v1575 + v1575);
        let v1577: f64 = (v335 * v1569);
        let v1578: f64 = (v1577 + v1577);
        let v1579: f64 = (v335 * v1570);
        let v1580: f64 = (v1579 + v1579);
        let v1581: f64 = (if self.scalar_v324 { v1572 } else { v13 });
        let v1582: f64 = (if self.scalar_v324 { v1574 } else { v13 });
        let v1583: f64 = (if self.scalar_v324 { v1576 } else { v13 });
        let v1584: f64 = (if self.scalar_v324 { v1578 } else { v13 });
        let v1585: f64 = (if self.scalar_v324 { v1580 } else { v13 });
        let v1586: f64 = (self.scalar_v195 * v1581);
        let v1587: f64 = (self.scalar_v195 * v1582);
        let v1588: f64 = (self.scalar_v195 * v1583);
        let v1589: f64 = (self.scalar_v195 * v1584);
        let v1590: f64 = (self.scalar_v195 * v1585);
        let v1591: f64 = (v1566 + v1586);
        let v1592: f64 = (v1567 + v1587);
        let v1593: f64 = (v1568 + v1588);
        let v1594: f64 = (v1569 + v1589);
        let v1595: f64 = (v1570 + v1590);
        let v1596: f64 = (v175 * v1566);
        let v1597: f64 = (v175 * v1567);
        let v1598: f64 = (v175 * v1568);
        let v1599: f64 = (v175 * v1569);
        let v1600: f64 = (v175 * v1570);
        let v1601: f64 = (v340 * v1581);
        let v1602: f64 = (v337 * v1596);
        let v1603: f64 = (v1601 + v1602);
        let v1604: f64 = (v340 * v1582);
        let v1605: f64 = (v337 * v1597);
        let v1606: f64 = (v1604 + v1605);
        let v1607: f64 = (v340 * v1583);
        let v1608: f64 = (v337 * v1598);
        let v1609: f64 = (v1607 + v1608);
        let v1610: f64 = (v340 * v1584);
        let v1611: f64 = (v337 * v1599);
        let v1612: f64 = (v1610 + v1611);
        let v1613: f64 = (v340 * v1585);
        let v1614: f64 = (v337 * v1600);
        let v1615: f64 = (v1613 + v1614);
        let v1616: f64 = (v1591 + v1603);
        let v1617: f64 = (v1592 + v1606);
        let v1618: f64 = (v1593 + v1609);
        let v1619: f64 = (v1594 + v1612);
        let v1620: f64 = (v1595 + v1615);
        let v1621: f64 = (v170 * v1616);
        let v1622: f64 = (v342 * v864);
        let v1623: f64 = (v170 * v1617);
        let v1624: f64 = (v1622 + v1623);
        let v1625: f64 = (v342 * v865);
        let v1626: f64 = (v170 * v1618);
        let v1627: f64 = (v1625 + v1626);
        let v1628: f64 = (v170 * v1619);
        let v1629: f64 = (v170 * v1620);
        let v1630: f64 = (if self.scalar_v324 { v1621 } else { v1108 });
        let v1631: f64 = (if self.scalar_v324 { v1624 } else { v1109 });
        let v1632: f64 = (if self.scalar_v324 { v1627 } else { v1110 });
        let v1633: f64 = (if self.scalar_v324 { v1628 } else { v1111 });
        let v1634: f64 = (if self.scalar_v324 { v1629 } else { v1112 });
        let v1635: f64 = { let limexp_arg = v334; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1636: f64 = (v1561 * v1635);
        let v1637: f64 = (v1562 * v1635);
        let v1638: f64 = (v1563 * v1635);
        let v1639: f64 = (v1564 * v1635);
        let v1640: f64 = (v1565 * v1635);
        let v1641: f64 = (-v1561);
        let v1642: f64 = (-v1562);
        let v1643: f64 = (-v1563);
        let v1644: f64 = (-v1564);
        let v1645: f64 = (-v1565);
        let v1646: f64 = { let limexp_arg = v346; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1647: f64 = (v1641 * v1646);
        let v1648: f64 = (v1642 * v1646);
        let v1649: f64 = (v1643 * v1646);
        let v1650: f64 = (v1644 * v1646);
        let v1651: f64 = (v1645 * v1646);
        let v1652: f64 = (v1636 - v1647);
        let v1653: f64 = (v1637 - v1648);
        let v1654: f64 = (v1638 - v1649);
        let v1655: f64 = (v1639 - v1650);
        let v1656: f64 = (v1640 - v1651);
        let v1657: f64 = (v146 * v1652);
        let v1658: f64 = (v146 * v1653);
        let v1659: f64 = (v146 * v1654);
        let v1660: f64 = (v146 * v1655);
        let v1661: f64 = (v146 * v1656);
        let v1662: f64 = (v350 * v350);
        let v1663: f64 = (v44 - v1662);
        let v1664: f64 = (v1657 * v1663);
        let v1665: f64 = (v1658 * v1663);
        let v1666: f64 = (v1659 * v1663);
        let v1667: f64 = (v1660 * v1663);
        let v1668: f64 = (v1661 * v1663);
        let v1669: f64 = (if self.scalar_v324 { v1664 } else { v1397 });
        let v1670: f64 = (if self.scalar_v324 { v1665 } else { v1398 });
        let v1671: f64 = (if self.scalar_v324 { v1666 } else { v1399 });
        let v1672: f64 = (if self.scalar_v324 { v1667 } else { v1400 });
        let v1673: f64 = (if self.scalar_v324 { v1668 } else { v1401 });
        let v1674: f64 = { let limexp_arg = v344; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1675: f64 = (v1630 * v1674);
        let v1676: f64 = (v1631 * v1674);
        let v1677: f64 = (v1632 * v1674);
        let v1678: f64 = (v1633 * v1674);
        let v1679: f64 = (v1634 * v1674);
        let v1680: f64 = (-v1630);
        let v1681: f64 = (-v1631);
        let v1682: f64 = (-v1632);
        let v1683: f64 = (-v1633);
        let v1684: f64 = (-v1634);
        let v1685: f64 = { let limexp_arg = v354; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1686: f64 = (v1680 * v1685);
        let v1687: f64 = (v1681 * v1685);
        let v1688: f64 = (v1682 * v1685);
        let v1689: f64 = (v1683 * v1685);
        let v1690: f64 = (v1684 * v1685);
        let v1691: f64 = (v1675 - v1686);
        let v1692: f64 = (v1676 - v1687);
        let v1693: f64 = (v1677 - v1688);
        let v1694: f64 = (v1678 - v1689);
        let v1695: f64 = (v1679 - v1690);
        let v1696: f64 = (v146 * v1691);
        let v1697: f64 = (v146 * v1692);
        let v1698: f64 = (v146 * v1693);
        let v1699: f64 = (v146 * v1694);
        let v1700: f64 = (v146 * v1695);
        let v1701: f64 = (v358 * v358);
        let v1702: f64 = (v44 - v1701);
        let v1703: f64 = (v1696 * v1702);
        let v1704: f64 = (v1697 * v1702);
        let v1705: f64 = (v1698 * v1702);
        let v1706: f64 = (v1699 * v1702);
        let v1707: f64 = (v1700 * v1702);
        let v1708: f64 = (if self.scalar_v324 { v1703 } else { v13 });
        let v1709: f64 = (if self.scalar_v324 { v1704 } else { v13 });
        let v1710: f64 = (if self.scalar_v324 { v1705 } else { v13 });
        let v1711: f64 = (if self.scalar_v324 { v1706 } else { v13 });
        let v1712: f64 = (if self.scalar_v324 { v1707 } else { v13 });
        let v1713: f64 = (self.scalar_v178 * v1669);
        let v1714: f64 = (self.scalar_v178 * v1670);
        let v1715: f64 = (self.scalar_v178 * v1671);
        let v1716: f64 = (self.scalar_v178 * v1672);
        let v1717: f64 = (self.scalar_v178 * v1673);
        let v1718: f64 = (if self.scalar_v324 { v1713 } else { v1407 });
        let v1719: f64 = (if self.scalar_v324 { v1714 } else { v1408 });
        let v1720: f64 = (if self.scalar_v324 { v1715 } else { v1409 });
        let v1721: f64 = (if self.scalar_v324 { v1716 } else { v1410 });
        let v1722: f64 = (if self.scalar_v324 { v1717 } else { v1411 });
        let v1723: f64 = (self.scalar_v178 * v1708);
        let v1724: f64 = (self.scalar_v178 * v1709);
        let v1725: f64 = (self.scalar_v178 * v1710);
        let v1726: f64 = (self.scalar_v178 * v1711);
        let v1727: f64 = (self.scalar_v178 * v1712);
        let v1728: f64 = (if self.scalar_v324 { v1723 } else { v13 });
        let v1729: f64 = (if self.scalar_v324 { v1724 } else { v13 });
        let v1730: f64 = (if self.scalar_v324 { v1725 } else { v13 });
        let v1731: f64 = (if self.scalar_v324 { v1726 } else { v13 });
        let v1732: f64 = (if self.scalar_v324 { v1727 } else { v13 });
        let v1733: f64 = (v7 * v1718);
        let v1734: f64 = (v7 * v1719);
        let v1735: f64 = (v363 + v1734);
        let v1736: f64 = (-v363);
        let v1737: f64 = (v7 * v1720);
        let v1738: f64 = (v1736 + v1737);
        let v1739: f64 = (v7 * v1721);
        let v1740: f64 = (v7 * v1722);
        let v1741: f64 = (v368 * v368);
        let v1742: f64 = (v44 - v1741);
        let v1743: f64 = (v1733 * v1742);
        let v1744: f64 = (v1735 * v1742);
        let v1745: f64 = (v1738 * v1742);
        let v1746: f64 = (v1739 * v1742);
        let v1747: f64 = (v1740 * v1742);
        let v1748: f64 = (if self.scalar_v324 { v1743 } else { v1427 });
        let v1749: f64 = (if self.scalar_v324 { v1744 } else { v1428 });
        let v1750: f64 = (if self.scalar_v324 { v1745 } else { v1429 });
        let v1751: f64 = (if self.scalar_v324 { v1746 } else { v1430 });
        let v1752: f64 = (if self.scalar_v324 { v1747 } else { v1431 });
        let v1753: f64 = (v7 * v1728);
        let v1754: f64 = (v7 * v1729);
        let v1755: f64 = (v366 + v1754);
        let v1756: f64 = (-v366);
        let v1757: f64 = (v7 * v1730);
        let v1758: f64 = (v1756 + v1757);
        let v1759: f64 = (v7 * v1731);
        let v1760: f64 = (v7 * v1732);
        let v1761: f64 = (v371 * v371);
        let v1762: f64 = (v44 - v1761);
        let v1763: f64 = (v1753 * v1762);
        let v1764: f64 = (v1755 * v1762);
        let v1765: f64 = (v1758 * v1762);
        let v1766: f64 = (v1759 * v1762);
        let v1767: f64 = (v1760 * v1762);
        let v1768: f64 = (if self.scalar_v324 { v1763 } else { v13 });
        let v1769: f64 = (if self.scalar_v324 { v1764 } else { v13 });
        let v1770: f64 = (if self.scalar_v324 { v1765 } else { v13 });
        let v1771: f64 = (if self.scalar_v324 { v1766 } else { v13 });
        let v1772: f64 = (if self.scalar_v324 { v1767 } else { v13 });
        let v1773: f64 = (self.scalar_v250 * v1708);
        let v1774: f64 = (self.scalar_v250 * v1709);
        let v1775: f64 = (self.scalar_v250 * v1710);
        let v1776: f64 = (self.scalar_v250 * v1711);
        let v1777: f64 = (self.scalar_v250 * v1712);
        let v1778: f64 = (if self.scalar_v324 { v1773 } else { v13 });
        let v1779: f64 = (if self.scalar_v324 { v1774 } else { v13 });
        let v1780: f64 = (if self.scalar_v324 { v1775 } else { v13 });
        let v1781: f64 = (if self.scalar_v324 { v1776 } else { v13 });
        let v1782: f64 = (if self.scalar_v324 { v1777 } else { v13 });
        let v1783: f64 = (self.scalar_v250 * v1669);
        let v1784: f64 = (self.scalar_v250 * v1670);
        let v1785: f64 = (self.scalar_v250 * v1671);
        let v1786: f64 = (self.scalar_v250 * v1672);
        let v1787: f64 = (self.scalar_v250 * v1673);
        let v1788: f64 = (if self.scalar_v324 { v1783 } else { v13 });
        let v1789: f64 = (if self.scalar_v324 { v1784 } else { v13 });
        let v1790: f64 = (if self.scalar_v324 { v1785 } else { v13 });
        let v1791: f64 = (if self.scalar_v324 { v1786 } else { v13 });
        let v1792: f64 = (if self.scalar_v324 { v1787 } else { v13 });
        let v1793: f64 = (v130 * v1669);
        let v1794: f64 = (v130 * v1670);
        let v1795: f64 = (v130 * v1671);
        let v1796: f64 = (v130 * v1672);
        let v1797: f64 = (v130 * v1673);
        let v1798: f64 = (v380 * v1793);
        let v1799: f64 = (v379 * v1748);
        let v1800: f64 = (v1798 + v1799);
        let v1801: f64 = (v380 * v1794);
        let v1802: f64 = (v379 * v1749);
        let v1803: f64 = (v1801 + v1802);
        let v1804: f64 = (v380 * v1795);
        let v1805: f64 = (v379 * v1750);
        let v1806: f64 = (v1804 + v1805);
        let v1807: f64 = (v380 * v1796);
        let v1808: f64 = (v379 * v1751);
        let v1809: f64 = (v1807 + v1808);
        let v1810: f64 = (v380 * v1797);
        let v1811: f64 = (v379 * v1752);
        let v1812: f64 = (v1810 + v1811);
        let v1813: f64 = (v7 * v1788);
        let v1814: f64 = (v7 * v1789);
        let v1815: f64 = (v378 + v1814);
        let v1816: f64 = (-v378);
        let v1817: f64 = (v7 * v1790);
        let v1818: f64 = (v1816 + v1817);
        let v1819: f64 = (v7 * v1791);
        let v1820: f64 = (v7 * v1792);
        let v1821: f64 = (v1167 + v1815);
        let v1822: f64 = (v1168 + v1818);
        let v1823: f64 = (v384 * v1800);
        let v1824: f64 = (v381 * v1813);
        let v1825: f64 = (v1823 + v1824);
        let v1826: f64 = (v384 * v1803);
        let v1827: f64 = (v381 * v1821);
        let v1828: f64 = (v1826 + v1827);
        let v1829: f64 = (v384 * v1806);
        let v1830: f64 = (v381 * v1822);
        let v1831: f64 = (v1829 + v1830);
        let v1832: f64 = (v384 * v1809);
        let v1833: f64 = (v381 * v1819);
        let v1834: f64 = (v1832 + v1833);
        let v1835: f64 = (v384 * v1812);
        let v1836: f64 = (v381 * v1820);
        let v1837: f64 = (v1835 + v1836);
        let v1838: f64 = (if self.scalar_v324 { v1825 } else { v1186 });
        let v1839: f64 = (if self.scalar_v324 { v1828 } else { v1187 });
        let v1840: f64 = (if self.scalar_v324 { v1831 } else { v1188 });
        let v1841: f64 = (if self.scalar_v324 { v1834 } else { v1189 });
        let v1842: f64 = (if self.scalar_v324 { v1837 } else { v1190 });
        let v1843: f64 = (v130 * v1708);
        let v1844: f64 = (v130 * v1709);
        let v1845: f64 = (v130 * v1710);
        let v1846: f64 = (v130 * v1711);
        let v1847: f64 = (v130 * v1712);
        let v1848: f64 = (-v1768);
        let v1849: f64 = (-v1769);
        let v1850: f64 = (-v1770);
        let v1851: f64 = (-v1771);
        let v1852: f64 = (-v1772);
        let v1853: f64 = (v388 * v1843);
        let v1854: f64 = (v387 * v1848);
        let v1855: f64 = (v1853 + v1854);
        let v1856: f64 = (v388 * v1844);
        let v1857: f64 = (v387 * v1849);
        let v1858: f64 = (v1856 + v1857);
        let v1859: f64 = (v388 * v1845);
        let v1860: f64 = (v387 * v1850);
        let v1861: f64 = (v1859 + v1860);
        let v1862: f64 = (v388 * v1846);
        let v1863: f64 = (v387 * v1851);
        let v1864: f64 = (v1862 + v1863);
        let v1865: f64 = (v388 * v1847);
        let v1866: f64 = (v387 * v1852);
        let v1867: f64 = (v1865 + v1866);
        let v1868: f64 = (v7 * v1778);
        let v1869: f64 = (v7 * v1779);
        let v1870: f64 = (v375 + v1869);
        let v1871: f64 = (-v375);
        let v1872: f64 = (v7 * v1780);
        let v1873: f64 = (v1871 + v1872);
        let v1874: f64 = (v7 * v1781);
        let v1875: f64 = (v7 * v1782);
        let v1876: f64 = (-v1868);
        let v1877: f64 = (-v1870);
        let v1878: f64 = (-v1873);
        let v1879: f64 = (-v1874);
        let v1880: f64 = (-v1875);
        let v1881: f64 = (v391 * v1855);
        let v1882: f64 = (v389 * v1876);
        let v1883: f64 = (v1881 + v1882);
        let v1884: f64 = (v391 * v1858);
        let v1885: f64 = (v389 * v1877);
        let v1886: f64 = (v1884 + v1885);
        let v1887: f64 = (v391 * v1861);
        let v1888: f64 = (v389 * v1878);
        let v1889: f64 = (v1887 + v1888);
        let v1890: f64 = (v391 * v1864);
        let v1891: f64 = (v389 * v1879);
        let v1892: f64 = (v1890 + v1891);
        let v1893: f64 = (v391 * v1867);
        let v1894: f64 = (v389 * v1880);
        let v1895: f64 = (v1893 + v1894);
        let v1896: f64 = (if self.scalar_v324 { v1883 } else { v1274 });
        let v1897: f64 = (if self.scalar_v324 { v1886 } else { v1275 });
        let v1898: f64 = (if self.scalar_v324 { v1889 } else { v1276 });
        let v1899: f64 = (if self.scalar_v324 { v1892 } else { v1277 });
        let v1900: f64 = (if self.scalar_v324 { v1895 } else { v1278 });
        let v1901: f64 = (v1838 - v1896);
        let v1902: f64 = (v1839 - v1897);
        let v1903: f64 = (v1840 - v1898);
        let v1904: f64 = (v1841 - v1899);
        let v1905: f64 = (v1842 - v1900);
        let v1906: f64 = (v146 * v1901);
        let v1907: f64 = (v146 * v1902);
        let v1908: f64 = (v146 * v1903);
        let v1909: f64 = (v146 * v1904);
        let v1910: f64 = (v146 * v1905);
        let v1911: f64 = (if self.scalar_v324 { v1906 } else { v1492 });
        let v1912: f64 = (if self.scalar_v324 { v1907 } else { v1493 });
        let v1913: f64 = (if self.scalar_v324 { v1908 } else { v1494 });
        let v1914: f64 = (if self.scalar_v324 { v1909 } else { v1495 });
        let v1915: f64 = (if self.scalar_v324 { v1910 } else { v1496 });
        let v1916: f64 = (if self.scalar_v399 { v1135 } else { v1437 });
        let v1917: f64 = (if self.scalar_v399 { v1136 } else { v1438 });
        let v1918: f64 = (if self.scalar_v399 { v1137 } else { v1439 });
        let v1919: f64 = (if self.scalar_v399 { v1138 } else { v1440 });
        let v1920: f64 = (if self.scalar_v399 { v1139 } else { v1441 });
        let v1921: f64 = (if self.scalar_v399 { v1713 } else { v1718 });
        let v1922: f64 = (if self.scalar_v399 { v1714 } else { v1719 });
        let v1923: f64 = (if self.scalar_v399 { v1715 } else { v1720 });
        let v1924: f64 = (if self.scalar_v399 { v1716 } else { v1721 });
        let v1925: f64 = (if self.scalar_v399 { v1717 } else { v1722 });
        let v1926: f64 = (v7 * v1921);
        let v1927: f64 = (v7 * v1922);
        let v1928: f64 = (v401 + v1927);
        let v1929: f64 = (-v401);
        let v1930: f64 = (v7 * v1923);
        let v1931: f64 = (v1929 + v1930);
        let v1932: f64 = (v7 * v1924);
        let v1933: f64 = (v7 * v1925);
        let v1934: f64 = (v403 * v403);
        let v1935: f64 = (v44 - v1934);
        let v1936: f64 = (v1926 * v1935);
        let v1937: f64 = (v1928 * v1935);
        let v1938: f64 = (v1931 * v1935);
        let v1939: f64 = (v1932 * v1935);
        let v1940: f64 = (v1933 * v1935);
        let v1941: f64 = (if self.scalar_v399 { v1936 } else { v1748 });
        let v1942: f64 = (if self.scalar_v399 { v1937 } else { v1749 });
        let v1943: f64 = (if self.scalar_v399 { v1938 } else { v1750 });
        let v1944: f64 = (if self.scalar_v399 { v1939 } else { v1751 });
        let v1945: f64 = (if self.scalar_v399 { v1940 } else { v1752 });
        let v1946: f64 = (v11 * v1921);
        let v1947: f64 = (v401 + v1946);
        let v1948: f64 = (v11 * v1922);
        let v1949: f64 = (v11 * v1923);
        let v1950: f64 = (v1929 + v1949);
        let v1951: f64 = (v11 * v1924);
        let v1952: f64 = (v11 * v1925);
        let v1953: f64 = (v406 * v406);
        let v1954: f64 = (v44 - v1953);
        let v1955: f64 = (v1947 * v1954);
        let v1956: f64 = (v1948 * v1954);
        let v1957: f64 = (v1950 * v1954);
        let v1958: f64 = (v1951 * v1954);
        let v1959: f64 = (v1952 * v1954);
        let v1960: f64 = (if self.scalar_v399 { v1955 } else { v13 });
        let v1961: f64 = (if self.scalar_v399 { v1956 } else { v13 });
        let v1962: f64 = (if self.scalar_v399 { v1957 } else { v13 });
        let v1963: f64 = (if self.scalar_v399 { v1958 } else { v13 });
        let v1964: f64 = (if self.scalar_v399 { v1959 } else { v13 });
        let v1965: f64 = (self.scalar_v408 * v1960);
        let v1966: f64 = (self.scalar_v408 * v1961);
        let v1967: f64 = (self.scalar_v408 * v1962);
        let v1968: f64 = (self.scalar_v408 * v1963);
        let v1969: f64 = (self.scalar_v408 * v1964);
        let v1970: f64 = (v1941 + v1965);
        let v1971: f64 = (v1942 + v1966);
        let v1972: f64 = (v1943 + v1967);
        let v1973: f64 = (v1944 + v1968);
        let v1974: f64 = (v1945 + v1969);
        let v1975: f64 = (v410 * v1001);
        let v1976: f64 = (v220 * v1970);
        let v1977: f64 = (v1975 + v1976);
        let v1978: f64 = (v410 * v1002);
        let v1979: f64 = (v220 * v1971);
        let v1980: f64 = (v1978 + v1979);
        let v1981: f64 = (v410 * v1003);
        let v1982: f64 = (v220 * v1972);
        let v1983: f64 = (v1981 + v1982);
        let v1984: f64 = (v410 * v1004);
        let v1985: f64 = (v220 * v1973);
        let v1986: f64 = (v1984 + v1985);
        let v1987: f64 = (v410 * v1005);
        let v1988: f64 = (v220 * v1974);
        let v1989: f64 = (v1987 + v1988);
        let v1992: f64 = (v413 * v1916);
        let v1993: f64 = (v400 * self.scalar_v408);
        let v1994: f64 = (v1992 + v1993);
        let v1995: f64 = (v413 * v1917);
        let v1996: f64 = (v400 + v1995);
        let v1997: f64 = (v413 * v1918);
        let v1998: f64 = (v400 * self.scalar_v1991);
        let v1999: f64 = (v1997 + v1998);
        let v2000: f64 = (v413 * v1919);
        let v2001: f64 = (v413 * v1920);
        let v2002: f64 = (v1167 + v1996);
        let v2003: f64 = (v1168 + v1999);
        let v2004: f64 = (v416 * v1977);
        let v2005: f64 = (v411 * v1994);
        let v2006: f64 = (v2004 + v2005);
        let v2007: f64 = (v416 * v1980);
        let v2008: f64 = (v411 * v2002);
        let v2009: f64 = (v2007 + v2008);
        let v2010: f64 = (v416 * v1983);
        let v2011: f64 = (v411 * v2003);
        let v2012: f64 = (v2010 + v2011);
        let v2013: f64 = (v416 * v1986);
        let v2014: f64 = (v411 * v2000);
        let v2015: f64 = (v2013 + v2014);
        let v2016: f64 = (v416 * v1989);
        let v2017: f64 = (v411 * v2001);
        let v2018: f64 = (v2016 + v2017);
        let v2019: f64 = (if self.scalar_v399 { v2006 } else { v1911 });
        let v2020: f64 = (if self.scalar_v399 { v2009 } else { v1912 });
        let v2021: f64 = (if self.scalar_v399 { v2012 } else { v1913 });
        let v2022: f64 = (if self.scalar_v399 { v2015 } else { v1914 });
        let v2023: f64 = (if self.scalar_v399 { v2018 } else { v1915 });
        let v2024: f64 = (if self.scalar_v423 { v13 } else { v1497 });
        let v2025: f64 = (if self.scalar_v423 { v13 } else { v1498 });
        let v2026: f64 = (if self.scalar_v423 { v13 } else { v1499 });
        let v2027: f64 = (if self.scalar_v423 { v13 } else { v1500 });
        let v2031: f64 = (if self.scalar_v441 { v13 } else { v2024 });
        let v2032: f64 = (if self.scalar_v441 { v13 } else { v2025 });
        let v2033: f64 = (if self.scalar_v441 { v13 } else { v2026 });
        let v2034: f64 = (if self.scalar_v441 { v13 } else { v2027 });
        let v2036: f64 = (v456 * v456);
        let v2037: f64 = (v44 - v2036);
        let v2038: f64 = (-v2037);
        let v2039: f64 = (if self.scalar_v455 { v2038 } else { self.scalar_v2029 });
        let v2040: f64 = (if self.scalar_v455 { v2037 } else { self.scalar_v2030 });
        let v2041: f64 = (v458 * v458);
        let v2042: f64 = (v44 - v2041);
        let v2043: f64 = (-v2042);
        let v2044: f64 = (if self.scalar_v455 { v2043 } else { self.scalar_v2029 });
        let v2045: f64 = (if self.scalar_v455 { v2042 } else { self.scalar_v2030 });
        let v2046: f64 = (if self.scalar_v461 { v424 } else { v2039 });
        let v2047: f64 = (if self.scalar_v461 { v44 } else { v2040 });
        let v2048: f64 = (if self.scalar_v461 { v424 } else { v2044 });
        let v2049: f64 = (if self.scalar_v461 { v44 } else { v2045 });
        let v2054: f64 = { let limexp_arg = v466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2055: f64 = (self.scalar_v2052 * v2054);
        let v2056: f64 = (self.scalar_v2053 * v2054);
        let v2057: f64 = (v153 * v2046);
        let v2058: f64 = (v153 * v2047);
        let v2059: f64 = { let limexp_arg = v470; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2060: f64 = (v2057 * v2059);
        let v2061: f64 = (v2058 * v2059);
        let v2062: f64 = (self.scalar_v474 * v2055);
        let v2063: f64 = (self.scalar_v474 * v2056);
        let v2064: f64 = (v2060 - v2062);
        let v2065: f64 = (v2061 - v2063);
        let v2066: f64 = (-v2031);
        let v2067: f64 = (-v2032);
        let v2068: f64 = (v2064 - v2033);
        let v2069: f64 = (-v2034);
        let v2071: f64 = (self.scalar_v469 * v2066);
        let v2072: f64 = (self.scalar_v469 * v2067);
        let v2073: f64 = (self.scalar_v469 * v2068);
        let v2074: f64 = (self.scalar_v469 * v2069);
        let v2075: f64 = (self.scalar_v469 * v2065);
        let v2077: f64 = { let limexp_arg = v479; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2078: f64 = (self.scalar_v2052 * v2077);
        let v2079: f64 = (self.scalar_v2053 * v2077);
        let v2080: f64 = (v153 * v2048);
        let v2081: f64 = (v153 * v2049);
        let v2082: f64 = { let limexp_arg = v482; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2083: f64 = (v2080 * v2082);
        let v2084: f64 = (v2081 * v2082);
        let v2085: f64 = (self.scalar_v474 * v2078);
        let v2086: f64 = (self.scalar_v474 * v2079);
        let v2087: f64 = (v2083 - v2085);
        let v2088: f64 = (v2084 - v2086);
        let v2089: f64 = (v2087 - v2032);
        let v2090: f64 = (-v2033);
        let v2091: f64 = (v2088 - v2034);
        let v2092: f64 = (self.scalar_v469 * v2089);
        let v2093: f64 = (self.scalar_v469 * v2090);
        let v2094: f64 = (self.scalar_v469 * v2091);
        let v2474: f64 = (-v2019);
        let v2475: f64 = (-v2020);
        let v2476: f64 = (-v2021);
        let v2477: f64 = (-v2022);
        let v2478: f64 = (-v2023);
        let v2522: f64 = (-v134);
        let v2523: f64 = -1e-12;
        let v2524: f64 = (v44 / v135);
        let v2525: f64 = (v424 / v135);
        let v2526: f64 = (if self.scalar_v695 { v2524 } else { v13 });
        let v2527: f64 = (if self.scalar_v695 { v2525 } else { v13 });
        let v2547: f64 = (if self.scalar_v714 { v733 } else { v13 });
        let v2548: f64 = (if self.scalar_v714 { v729 } else { v13 });
        let v2551: f64 = (v44 / v49);
        let v2552: f64 = (if self.scalar_v738 { v2551 } else { v13 });

        let d739_dn4: f64 = v2474;
        let d739_dn5: f64 = v2475;
        let d739_dn8: f64 = v2476;
        let d739_dn10: f64 = v2477;
        let d739_dn12: f64 = v2478;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(15),
            None,
            multiplicity * (v739),
            [4, 5, 8, 10, 12],
            [d739_dn4, d739_dn5, d739_dn8, d739_dn10, d739_dn12],
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
        let d478_dn4: f64 = v2071;
        let d478_dn5: f64 = v2072;
        let d478_dn8: f64 = v2073;
        let d478_dn10: f64 = v2074;
        let d478_dn11: f64 = v2075;
        let d478_dn12: f64 = self.scalar_v2076;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(8),
            multiplicity * (v478),
            [4, 5, 8, 10, 11, 12],
            [d478_dn4, d478_dn5, d478_dn8, d478_dn10, d478_dn11, d478_dn12],
            [],
            [],
            multiplicity,
        );
        let d487_dn4: f64 = v2071;
        let d487_dn5: f64 = v2092;
        let d487_dn8: f64 = v2093;
        let d487_dn10: f64 = v2094;
        let d487_dn12: f64 = self.scalar_v2076;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * (v487),
            [4, 5, 8, 10, 12],
            [d487_dn4, d487_dn5, d487_dn8, d487_dn10, d487_dn12],
            [],
            [],
            multiplicity,
        );
        let d763_dn4: f64 = v2523;
        let d763_dn6: f64 = v161;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v763),
            4,
            multiplicity * (d763_dn4),
            6,
            multiplicity * (d763_dn6),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v765,
        );
        let d768_dn11: f64 = v2526;
        let d768_dn12: f64 = v2527;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(12),
            multiplicity * (v768),
            11,
            multiplicity * (d768_dn11),
            12,
            multiplicity * (d768_dn12),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(8),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v773,
        );
        let d780_dn8: f64 = self.scalar_v2536;
        let d780_dn14: f64 = self.scalar_v2537;
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * (v780),
            8,
            multiplicity * (d780_dn8),
            14,
            multiplicity * (d780_dn14),
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v782,
        );
        let d786_dn10: f64 = self.scalar_v2540;
        let d786_dn13: f64 = self.scalar_v2541;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(10),
            multiplicity * (v786),
            10,
            multiplicity * (d786_dn10),
            13,
            multiplicity * (d786_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(10),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v788,
        );
        stamper.stamp_current_const_local(
            Some(13),
            Some(10),
            multiplicity * (self.scalar_v789),
        );
        let d792_dn11: f64 = self.scalar_v2544;
        let d792_dn13: f64 = self.scalar_v2545;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(11),
            multiplicity * (v792),
            11,
            multiplicity * (d792_dn11),
            13,
            multiplicity * (d792_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(11),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v794,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            self.scalar_v796,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v798,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v800,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            self.scalar_v802,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            self.scalar_v804,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            self.scalar_v806,
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (v807),
        );
        stamper.stamp_current_const_local(
            Some(14),
            Some(2),
            multiplicity * (v161),
        );
        let d810_dn2: f64 = v2523;
        let d810_dn12: f64 = v161;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * (v810),
            2,
            multiplicity * (d810_dn2),
            12,
            multiplicity * (d810_dn12),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v812),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v812),
        );
        stamper.stamp_current_const_local(
            Some(17),
            None,
            multiplicity * (self.scalar_v813),
        );
        let d815_dn17: f64 = self.scalar_v2546;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v815),
            17,
            multiplicity * (d815_dn17),
        );
        stamper.stamp_current_const_local(
            Some(18),
            None,
            multiplicity * (self.scalar_v813),
        );
        let d817_dn18: f64 = self.scalar_v2546;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v817),
            18,
            multiplicity * (d817_dn18),
        );
        let d815_dn17: f64 = self.scalar_v2546;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (v815),
            17,
            multiplicity * (d815_dn17),
        );
        let d821_dn17: f64 = v2547;
        let d821_dn18: f64 = v2548;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v821),
            17,
            multiplicity * (d821_dn17),
            18,
            multiplicity * (d821_dn18),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v813),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (self.scalar_v813),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v813),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v827),
        );
        let d814_dn17: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v814),
            17,
            multiplicity * (d814_dn17),
        );
        let d816_dn18: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v816),
            18,
            multiplicity * (d816_dn18),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (self.scalar_v789),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (self.scalar_v789),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (self.scalar_v829),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (self.scalar_v829),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v836),
        );
        let d838_dn3: f64 = v2552;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v838),
            3,
            multiplicity * (d838_dn3),
        );
        let d845_dn3: f64 = self.scalar_v2555;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v845),
            3,
            multiplicity * (d845_dn3),
        );
        let d742_dn15: f64 = self.scalar_v740;
        let v742_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v742);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v742_ddt),
            15,
            multiplicity * (((d742_dn15) * ddt_scale)),
        );
        let d757_dn5: f64 = self.scalar_v2520;
        let d757_dn7: f64 = self.scalar_v754;
        let v757_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v757);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v757_ddt),
            5,
            multiplicity * (((d757_dn5) * ddt_scale)),
            7,
            multiplicity * (((d757_dn7) * ddt_scale)),
        );
        let d759_dn5: f64 = self.scalar_v758;
        let d759_dn8: f64 = self.scalar_v2521;
        let v759_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v759);
        stamper.stamp_current_node2_local(
            Some(5),
            Some(8),
            multiplicity * (v759_ddt),
            5,
            multiplicity * (((d759_dn5) * ddt_scale)),
            8,
            multiplicity * (((d759_dn8) * ddt_scale)),
        );
        let d762_dn4: f64 = v2522;
        let d762_dn6: f64 = v134;
        let v762_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v762);
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v762_ddt),
            4,
            multiplicity * (((d762_dn4) * ddt_scale)),
            6,
            multiplicity * (((d762_dn6) * ddt_scale)),
        );
        let d777_dn11: f64 = self.scalar_v774;
        let d777_dn14: f64 = self.scalar_v2533;
        let v777_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, v777);
        stamper.stamp_current_node2_local(
            Some(11),
            Some(14),
            multiplicity * (v777_ddt),
            11,
            multiplicity * (((d777_dn11) * ddt_scale)),
            14,
            multiplicity * (((d777_dn14) * ddt_scale)),
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
        let v77: f64 = (v46 * self.scalar_v76);
        let v78: f64 = (v44 + v77);
        let v79: f64 = (self.scalar_v75 * v78);
        let v80: f64 = (if v43 { v79 } else { v13 });
        let v129: bool = (!v43);
        let v134: f64 = (if v129 { self.scalar_v75 } else { v80 });
        let v741: f64 = nv15;
        let v742: f64 = (self.scalar_v740 * v741);
        let v755: f64 = nv7;
        let v756: f64 = (v755 - v4);
        let v757: f64 = (self.scalar_v754 * v756);
        let v759: f64 = (v7 * self.scalar_v758);
        let v760: f64 = nv6;
        let v761: f64 = (v760 - v10);
        let v762: f64 = (v134 * v761);
        let v775: f64 = nv14;
        let v776: f64 = (v8 - v775);
        let v777: f64 = (self.scalar_v774 * v776);
        let v2522: f64 = (-v134);

        let d742_dn15: f64 = self.scalar_v740;
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (d742_dn15),
        );
        let d757_dn5: f64 = self.scalar_v2520;
        let d757_dn7: f64 = self.scalar_v754;
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (d757_dn5),
            nodes[7],
            multiplicity * (d757_dn7),
        );
        let d759_dn5: f64 = self.scalar_v758;
        let d759_dn8: f64 = self.scalar_v2521;
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * (d759_dn5),
            nodes[8],
            multiplicity * (d759_dn8),
        );
        let d762_dn4: f64 = v2522;
        let d762_dn6: f64 = v134;
        stamper.stamp_current_reactive_node2(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * (d762_dn4),
            nodes[6],
            multiplicity * (d762_dn6),
        );
        let d777_dn11: f64 = self.scalar_v774;
        let d777_dn14: f64 = self.scalar_v2533;
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[14]),
            nodes[11],
            multiplicity * (d777_dn11),
            nodes[14],
            multiplicity * (d777_dn14),
        );
        let mut locals = StampLocals::default();

        Self::stamp_reactive_block_0(ctx, p, nodes, param_given, &mut locals);
        Self::stamp_reactive_block_1(p, &mut locals);
        Self::stamp_reactive_block_2(p, &mut locals);
        Self::stamp_reactive_block_3(ctx, p, branches, &mut locals);

        Self::stamp_reactive_equations_block_0(ctx, stamper, p, nodes, branches, multiplicity, &mut locals);
    }
}
