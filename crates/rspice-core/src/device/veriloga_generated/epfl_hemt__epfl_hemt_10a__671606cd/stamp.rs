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
    pub(crate) var_alpha_clm: f64,
    pub(crate) var_beta: f64,
    pub(crate) var_beta_dn4: f64,
    pub(crate) var_cb: f64,
    pub(crate) var_cdsc: f64,
    pub(crate) var_cdsc_dn1: f64,
    pub(crate) var_cdsc_dn4: f64,
    pub(crate) var_cdsc_dn5: f64,
    pub(crate) var_cdsc_dn6: f64,
    pub(crate) var_dat: f64,
    pub(crate) var_dat_dn4: f64,
    pub(crate) var_deltal: f64,
    pub(crate) var_deltal_dn1: f64,
    pub(crate) var_deltal_dn4: f64,
    pub(crate) var_deltal_dn5: f64,
    pub(crate) var_deltal_dn6: f64,
    pub(crate) var_dos_2d: f64,
    pub(crate) var_dqsd2: f64,
    pub(crate) var_dqsd2_dn1: f64,
    pub(crate) var_dqsd2_dn4: f64,
    pub(crate) var_dqsd2_dn5: f64,
    pub(crate) var_dqsd2_dn6: f64,
    pub(crate) var_dr: f64,
    pub(crate) var_dr_dn1: f64,
    pub(crate) var_dr_dn4: f64,
    pub(crate) var_dr_dn5: f64,
    pub(crate) var_dr_dn6: f64,
    pub(crate) var_dtot: f64,
    pub(crate) var_dtot_dn1: f64,
    pub(crate) var_dtot_dn4: f64,
    pub(crate) var_dtot_dn5: f64,
    pub(crate) var_dtot_dn6: f64,
    pub(crate) var_dv_clm: f64,
    pub(crate) var_dv_clm_dn1: f64,
    pub(crate) var_dv_clm_dn4: f64,
    pub(crate) var_dv_clm_dn5: f64,
    pub(crate) var_dv_clm_dn6: f64,
    pub(crate) var_dvsat: f64,
    pub(crate) var_dvsat_dn1: f64,
    pub(crate) var_dvsat_dn4: f64,
    pub(crate) var_dvsat_dn5: f64,
    pub(crate) var_dvsat_dn6: f64,
    pub(crate) var_dvth_dibl: f64,
    pub(crate) var_dvth_dibl_dn1: f64,
    pub(crate) var_dvth_dibl_dn4: f64,
    pub(crate) var_dvth_dibl_dn5: f64,
    pub(crate) var_dvth_dibl_dn6: f64,
    pub(crate) var_e_clm: f64,
    pub(crate) var_e_clm2: f64,
    pub(crate) var_e_clm2_dn1: f64,
    pub(crate) var_e_clm2_dn4: f64,
    pub(crate) var_e_clm2_dn5: f64,
    pub(crate) var_e_clm2_dn6: f64,
    pub(crate) var_e_clm_dn1: f64,
    pub(crate) var_e_clm_dn4: f64,
    pub(crate) var_e_clm_dn5: f64,
    pub(crate) var_e_clm_dn6: f64,
    pub(crate) var_e_clmp2: f64,
    pub(crate) var_e_clmp2_dn1: f64,
    pub(crate) var_e_clmp2_dn4: f64,
    pub(crate) var_e_clmp2_dn5: f64,
    pub(crate) var_e_clmp2_dn6: f64,
    pub(crate) var_e_clmx2: f64,
    pub(crate) var_e_clmx2_dn1: f64,
    pub(crate) var_e_clmx2_dn4: f64,
    pub(crate) var_e_clmx2_dn5: f64,
    pub(crate) var_e_clmx2_dn6: f64,
    pub(crate) var_e_clmx2xqs: f64,
    pub(crate) var_e_clmx2xqs_dn1: f64,
    pub(crate) var_e_clmx2xqs_dn4: f64,
    pub(crate) var_e_clmx2xqs_dn5: f64,
    pub(crate) var_e_clmx2xqs_dn6: f64,
    pub(crate) var_eefff: f64,
    pub(crate) var_eeffm: f64,
    pub(crate) var_eeffm_dn1: f64,
    pub(crate) var_eeffm_dn4: f64,
    pub(crate) var_eeffm_dn5: f64,
    pub(crate) var_eeffm_dn6: f64,
    pub(crate) var_eeffs: f64,
    pub(crate) var_eeffs_dn1: f64,
    pub(crate) var_eeffs_dn4: f64,
    pub(crate) var_eeffs_dn5: f64,
    pub(crate) var_eeffs_dn6: f64,
    pub(crate) var_eggan: f64,
    pub(crate) var_eggan_dn4: f64,
    pub(crate) var_ep_algan: f64,
    pub(crate) var_ep_gan: f64,
    pub(crate) var_eta_mu: f64,
    pub(crate) var_g_clm: f64,
    pub(crate) var_gam: f64,
    pub(crate) var_gam_dn4: f64,
    pub(crate) var_guard2: f64,
    pub(crate) var_guard3: f64,
    pub(crate) var_guard4: f64,
    pub(crate) var_guard5: f64,
    pub(crate) var_guard6: f64,
    pub(crate) var_guard6_rv: f64,
    pub(crate) var_iaccsat: f64,
    pub(crate) var_iaccsat_dn4: f64,
    pub(crate) var_ids: f64,
    pub(crate) var_ids_dn1: f64,
    pub(crate) var_ids_dn4: f64,
    pub(crate) var_ids_dn5: f64,
    pub(crate) var_ids_dn6: f64,
    pub(crate) var_inv_nut: f64,
    pub(crate) var_inv_nut_dn1: f64,
    pub(crate) var_inv_nut_dn4: f64,
    pub(crate) var_inv_nut_dn5: f64,
    pub(crate) var_inv_nut_dn6: f64,
    pub(crate) var_inv_ut: f64,
    pub(crate) var_inv_ut_dn4: f64,
    pub(crate) var_isp: f64,
    pub(crate) var_isp_dn1: f64,
    pub(crate) var_isp_dn4: f64,
    pub(crate) var_isp_dn5: f64,
    pub(crate) var_isp_dn6: f64,
    pub(crate) var_lambdac: f64,
    pub(crate) var_lambdac_dn1: f64,
    pub(crate) var_lambdac_dn4: f64,
    pub(crate) var_lambdac_dn5: f64,
    pub(crate) var_lambdac_dn6: f64,
    pub(crate) var_leff: f64,
    pub(crate) var_mdm2: f64,
    pub(crate) var_me: f64,
    pub(crate) var_muacc: f64,
    pub(crate) var_muacc_dn4: f64,
    pub(crate) var_mut_: f64,
    pub(crate) var_mut__dn4: f64,
    pub(crate) var_n: f64,
    pub(crate) var_n0: f64,
    pub(crate) var_n0_dn4: f64,
    pub(crate) var_n_dn1: f64,
    pub(crate) var_n_dn4: f64,
    pub(crate) var_n_dn5: f64,
    pub(crate) var_n_dn6: f64,
    pub(crate) var_n_q: f64,
    pub(crate) var_n_q_dn1: f64,
    pub(crate) var_n_q_dn4: f64,
    pub(crate) var_n_q_dn5: f64,
    pub(crate) var_n_q_dn6: f64,
    pub(crate) var_nc_gan: f64,
    pub(crate) var_ni_gan: f64,
    pub(crate) var_ni_gan_dn4: f64,
    pub(crate) var_nq: f64,
    pub(crate) var_nq_dn1: f64,
    pub(crate) var_nq_dn4: f64,
    pub(crate) var_nq_dn5: f64,
    pub(crate) var_nq_dn6: f64,
    pub(crate) var_nsacc: f64,
    pub(crate) var_nut: f64,
    pub(crate) var_nut_dn1: f64,
    pub(crate) var_nut_dn4: f64,
    pub(crate) var_nut_dn5: f64,
    pub(crate) var_nut_dn6: f64,
    pub(crate) var_nv: f64,
    pub(crate) var_pdiss: f64,
    pub(crate) var_pdiss_dn1: f64,
    pub(crate) var_pdiss_dn4: f64,
    pub(crate) var_pdiss_dn5: f64,
    pub(crate) var_pdiss_dn6: f64,
    pub(crate) var_phib: f64,
    pub(crate) var_phib_dn4: f64,
    pub(crate) var_pi: f64,
    pub(crate) var_pinch_n: f64,
    pub(crate) var_pinch_n_dn1: f64,
    pub(crate) var_pinch_n_dn4: f64,
    pub(crate) var_pinch_n_dn5: f64,
    pub(crate) var_pinch_n_dn6: f64,
    pub(crate) var_pinch_norm_cl: f64,
    pub(crate) var_pinch_norm_cl_dn1: f64,
    pub(crate) var_pinch_norm_cl_dn4: f64,
    pub(crate) var_pinch_norm_cl_dn5: f64,
    pub(crate) var_pinch_norm_cl_dn6: f64,
    pub(crate) var_psiavg: f64,
    pub(crate) var_psiavg_dn1: f64,
    pub(crate) var_psiavg_dn4: f64,
    pub(crate) var_psiavg_dn5: f64,
    pub(crate) var_psiavg_dn6: f64,
    pub(crate) var_q: f64,
    pub(crate) var_qacc: f64,
    pub(crate) var_qb: f64,
    pub(crate) var_qb_dn1: f64,
    pub(crate) var_qb_dn4: f64,
    pub(crate) var_qb_dn5: f64,
    pub(crate) var_qb_dn6: f64,
    pub(crate) var_qba: f64,
    pub(crate) var_qba_dn1: f64,
    pub(crate) var_qba_dn4: f64,
    pub(crate) var_qba_dn5: f64,
    pub(crate) var_qba_dn6: f64,
    pub(crate) var_qbs: f64,
    pub(crate) var_qbs_dn1: f64,
    pub(crate) var_qbs_dn4: f64,
    pub(crate) var_qbs_dn5: f64,
    pub(crate) var_qbs_dn6: f64,
    pub(crate) var_qd: f64,
    pub(crate) var_qd_dn1: f64,
    pub(crate) var_qd_dn4: f64,
    pub(crate) var_qd_dn5: f64,
    pub(crate) var_qd_dn6: f64,
    pub(crate) var_qdeff: f64,
    pub(crate) var_qdeff_dn1: f64,
    pub(crate) var_qdeff_dn4: f64,
    pub(crate) var_qdeff_dn5: f64,
    pub(crate) var_qdeff_dn6: f64,
    pub(crate) var_qdsat: f64,
    pub(crate) var_qdsat_dn1: f64,
    pub(crate) var_qdsat_dn4: f64,
    pub(crate) var_qdsat_dn5: f64,
    pub(crate) var_qdsat_dn6: f64,
    pub(crate) var_qia: f64,
    pub(crate) var_qia_dn1: f64,
    pub(crate) var_qia_dn4: f64,
    pub(crate) var_qia_dn5: f64,
    pub(crate) var_qia_dn6: f64,
    pub(crate) var_qis: f64,
    pub(crate) var_qis_dn1: f64,
    pub(crate) var_qis_dn4: f64,
    pub(crate) var_qis_dn5: f64,
    pub(crate) var_qis_dn6: f64,
    pub(crate) var_qs: f64,
    pub(crate) var_qs_1: f64,
    pub(crate) var_qs_1_dn1: f64,
    pub(crate) var_qs_1_dn4: f64,
    pub(crate) var_qs_1_dn5: f64,
    pub(crate) var_qs_1_dn6: f64,
    pub(crate) var_qs_dn1: f64,
    pub(crate) var_qs_dn4: f64,
    pub(crate) var_qs_dn5: f64,
    pub(crate) var_qs_dn6: f64,
    pub(crate) var_qs_qsat: f64,
    pub(crate) var_qs_qsat2: f64,
    pub(crate) var_qs_qsat2_dn1: f64,
    pub(crate) var_qs_qsat2_dn4: f64,
    pub(crate) var_qs_qsat2_dn5: f64,
    pub(crate) var_qs_qsat2_dn6: f64,
    pub(crate) var_qs_qsat_dn1: f64,
    pub(crate) var_qs_qsat_dn4: f64,
    pub(crate) var_qs_qsat_dn5: f64,
    pub(crate) var_qs_qsat_dn6: f64,
    pub(crate) var_qsp: f64,
    pub(crate) var_qsp_dn1: f64,
    pub(crate) var_qsp_dn4: f64,
    pub(crate) var_qsp_dn5: f64,
    pub(crate) var_qsp_dn6: f64,
    pub(crate) var_rcod: f64,
    pub(crate) var_rcos: f64,
    pub(crate) var_rd: f64,
    pub(crate) var_rd0: f64,
    pub(crate) var_rd0_dn4: f64,
    pub(crate) var_rd1: f64,
    pub(crate) var_rd1_dn1: f64,
    pub(crate) var_rd1_dn4: f64,
    pub(crate) var_rd1_dn5: f64,
    pub(crate) var_rd1_dn6: f64,
    pub(crate) var_rd_dn1: f64,
    pub(crate) var_rd_dn4: f64,
    pub(crate) var_rd_dn5: f64,
    pub(crate) var_rd_dn6: f64,
    pub(crate) var_rs: f64,
    pub(crate) var_rs0: f64,
    pub(crate) var_rs0_dn4: f64,
    pub(crate) var_rs1: f64,
    pub(crate) var_rs1_dn1: f64,
    pub(crate) var_rs1_dn4: f64,
    pub(crate) var_rs1_dn5: f64,
    pub(crate) var_rs1_dn6: f64,
    pub(crate) var_rs_dn1: f64,
    pub(crate) var_rs_dn4: f64,
    pub(crate) var_rs_dn5: f64,
    pub(crate) var_rs_dn6: f64,
    pub(crate) var_sqrtpsip: f64,
    pub(crate) var_sqrtpsip_dn1: f64,
    pub(crate) var_sqrtpsip_dn4: f64,
    pub(crate) var_sqrtpsip_dn5: f64,
    pub(crate) var_sqrtpsip_dn6: f64,
    pub(crate) var_t0: f64,
    pub(crate) var_t0_dn1: f64,
    pub(crate) var_t0_dn4: f64,
    pub(crate) var_t0_dn5: f64,
    pub(crate) var_t0_dn6: f64,
    pub(crate) var_t1: f64,
    pub(crate) var_t1_dn1: f64,
    pub(crate) var_t1_dn4: f64,
    pub(crate) var_t1_dn5: f64,
    pub(crate) var_t1_dn6: f64,
    pub(crate) var_t2: f64,
    pub(crate) var_t2_dn1: f64,
    pub(crate) var_t2_dn4: f64,
    pub(crate) var_t2_dn5: f64,
    pub(crate) var_t2_dn6: f64,
    pub(crate) var_t3: f64,
    pub(crate) var_t3_dn1: f64,
    pub(crate) var_t3_dn4: f64,
    pub(crate) var_t3_dn5: f64,
    pub(crate) var_t3_dn6: f64,
    pub(crate) var_t4: f64,
    pub(crate) var_t4_dn1: f64,
    pub(crate) var_t4_dn4: f64,
    pub(crate) var_t4_dn5: f64,
    pub(crate) var_t4_dn6: f64,
    pub(crate) var_t7: f64,
    pub(crate) var_t7_dn1: f64,
    pub(crate) var_t7_dn4: f64,
    pub(crate) var_t7_dn5: f64,
    pub(crate) var_t7_dn6: f64,
    pub(crate) var_t8: f64,
    pub(crate) var_t8_dn1: f64,
    pub(crate) var_t8_dn4: f64,
    pub(crate) var_t8_dn5: f64,
    pub(crate) var_t8_dn6: f64,
    pub(crate) var_t9: f64,
    pub(crate) var_t9_dn1: f64,
    pub(crate) var_t9_dn4: f64,
    pub(crate) var_t9_dn5: f64,
    pub(crate) var_t9_dn6: f64,
    pub(crate) var_tambk: f64,
    pub(crate) var_tdut: f64,
    pub(crate) var_tdut_dn4: f64,
    pub(crate) var_tmp_vdp1: f64,
    pub(crate) var_tmp_vdp1_dn1: f64,
    pub(crate) var_tmp_vdp1_dn4: f64,
    pub(crate) var_tmp_vdp1_dn5: f64,
    pub(crate) var_tmp_vdp1_dn6: f64,
    pub(crate) var_tmp_vdp2: f64,
    pub(crate) var_tmp_vdp2_dn1: f64,
    pub(crate) var_tmp_vdp2_dn4: f64,
    pub(crate) var_tmp_vdp2_dn5: f64,
    pub(crate) var_tmp_vdp2_dn6: f64,
    pub(crate) var_tmp_vdp3: f64,
    pub(crate) var_tmp_vdp3_dn1: f64,
    pub(crate) var_tmp_vdp3_dn4: f64,
    pub(crate) var_tmp_vdp3_dn5: f64,
    pub(crate) var_tmp_vdp3_dn6: f64,
    pub(crate) var_tmp_vdsat1: f64,
    pub(crate) var_tmp_vdsat11: f64,
    pub(crate) var_tmp_vdsat11_dn1: f64,
    pub(crate) var_tmp_vdsat11_dn4: f64,
    pub(crate) var_tmp_vdsat11_dn5: f64,
    pub(crate) var_tmp_vdsat11_dn6: f64,
    pub(crate) var_tmp_vdsat1_dn1: f64,
    pub(crate) var_tmp_vdsat1_dn4: f64,
    pub(crate) var_tmp_vdsat1_dn5: f64,
    pub(crate) var_tmp_vdsat1_dn6: f64,
    pub(crate) var_tmp_vdsat2: f64,
    pub(crate) var_tmp_vdsat2_dn1: f64,
    pub(crate) var_tmp_vdsat2_dn4: f64,
    pub(crate) var_tmp_vdsat2_dn5: f64,
    pub(crate) var_tmp_vdsat2_dn6: f64,
    pub(crate) var_tratio: f64,
    pub(crate) var_tratio_dn4: f64,
    pub(crate) var_tsh: f64,
    pub(crate) var_tsh_dn4: f64,
    pub(crate) var_u_clm: f64,
    pub(crate) var_u_clm_dn1: f64,
    pub(crate) var_u_clm_dn4: f64,
    pub(crate) var_u_clm_dn5: f64,
    pub(crate) var_u_clm_dn6: f64,
    pub(crate) var_ucrit_t: f64,
    pub(crate) var_ucrit_t_dn4: f64,
    pub(crate) var_ueff: f64,
    pub(crate) var_ueff_dn1: f64,
    pub(crate) var_ueff_dn4: f64,
    pub(crate) var_ueff_dn5: f64,
    pub(crate) var_ueff_dn6: f64,
    pub(crate) var_ut: f64,
    pub(crate) var_ut_dn4: f64,
    pub(crate) var_v_s: f64,
    pub(crate) var_v_s_dn4: f64,
    pub(crate) var_va: f64,
    pub(crate) var_va_dn1: f64,
    pub(crate) var_va_dn4: f64,
    pub(crate) var_va_dn5: f64,
    pub(crate) var_va_dn6: f64,
    pub(crate) var_vbsx: f64,
    pub(crate) var_vbsx_dn1: f64,
    pub(crate) var_vbsx_dn4: f64,
    pub(crate) var_vbsx_dn5: f64,
    pub(crate) var_vbsx_dn6: f64,
    pub(crate) var_vd: f64,
    pub(crate) var_vd_1: f64,
    pub(crate) var_vd_1_dn1: f64,
    pub(crate) var_vd_1_dn4: f64,
    pub(crate) var_vd_1_dn5: f64,
    pub(crate) var_vd_1_dn6: f64,
    pub(crate) var_vd_dn5: f64,
    pub(crate) var_vd_dn6: f64,
    pub(crate) var_vdar: f64,
    pub(crate) var_vdar_dn1: f64,
    pub(crate) var_vdar_dn4: f64,
    pub(crate) var_vdar_dn5: f64,
    pub(crate) var_vdar_dn6: f64,
    pub(crate) var_vdp: f64,
    pub(crate) var_vdp_dn1: f64,
    pub(crate) var_vdp_dn4: f64,
    pub(crate) var_vdp_dn5: f64,
    pub(crate) var_vdp_dn6: f64,
    pub(crate) var_vds: f64,
    pub(crate) var_vds_dn5: f64,
    pub(crate) var_vds_dn6: f64,
    pub(crate) var_vdsat: f64,
    pub(crate) var_vdsat_dn1: f64,
    pub(crate) var_vdsat_dn4: f64,
    pub(crate) var_vdsat_dn5: f64,
    pub(crate) var_vdsat_dn6: f64,
    pub(crate) var_vdssat: f64,
    pub(crate) var_vdssat_dn1: f64,
    pub(crate) var_vdssat_dn4: f64,
    pub(crate) var_vdssat_dn5: f64,
    pub(crate) var_vdssat_dn6: f64,
    pub(crate) var_vdsx: f64,
    pub(crate) var_vdsx_dn1: f64,
    pub(crate) var_vdsx_dn4: f64,
    pub(crate) var_vdsx_dn5: f64,
    pub(crate) var_vdsx_dn6: f64,
    pub(crate) var_vg: f64,
    pub(crate) var_vg_1: f64,
    pub(crate) var_vg_1_dn1: f64,
    pub(crate) var_vg_1_dn4: f64,
    pub(crate) var_vg_1_dn5: f64,
    pub(crate) var_vg_1_dn6: f64,
    pub(crate) var_vg_dn1: f64,
    pub(crate) var_vg_dn6: f64,
    pub(crate) var_vg_va: f64,
    pub(crate) var_vg_va_dn1: f64,
    pub(crate) var_vg_va_dn4: f64,
    pub(crate) var_vg_va_dn5: f64,
    pub(crate) var_vg_va_dn6: f64,
    pub(crate) var_vmr: f64,
    pub(crate) var_vmr_dn1: f64,
    pub(crate) var_vmr_dn4: f64,
    pub(crate) var_vmr_dn5: f64,
    pub(crate) var_vmr_dn6: f64,
    pub(crate) var_vp: f64,
    pub(crate) var_vp_dn1: f64,
    pub(crate) var_vp_dn4: f64,
    pub(crate) var_vp_dn5: f64,
    pub(crate) var_vp_dn6: f64,
    pub(crate) var_vs: f64,
    pub(crate) var_vs_1: f64,
    pub(crate) var_vs_1_dn1: f64,
    pub(crate) var_vs_1_dn4: f64,
    pub(crate) var_vs_1_dn5: f64,
    pub(crate) var_vs_1_dn6: f64,
    pub(crate) var_vs_dn6: f64,
    pub(crate) var_vsar: f64,
    pub(crate) var_vsar_dn1: f64,
    pub(crate) var_vsar_dn4: f64,
    pub(crate) var_vsar_dn5: f64,
    pub(crate) var_vsar_dn6: f64,
    pub(crate) var_vsatacc: f64,
    pub(crate) var_vsatacc_dn4: f64,
    pub(crate) var_vth_shift: f64,
    pub(crate) var_vth_shift_dn1: f64,
    pub(crate) var_vth_shift_dn4: f64,
    pub(crate) var_vth_shift_dn5: f64,
    pub(crate) var_vth_shift_dn6: f64,
    pub(crate) var_weff: f64,
}

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv4 = ctx.node_voltage(nodes[4]);
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

        stamper.stamp_potential_branch_local(
            Some(3),
            Some(9),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            v0,
        );
        let mut locals = StampLocals::default();

        Self::stamp_transient_block_0(ctx, p, nodes, &mut locals);
        Self::stamp_transient_block_1(&mut locals);
        Self::stamp_transient_block_2(p, &mut locals);
        Self::stamp_transient_block_3(&mut locals);
        Self::stamp_transient_block_4(p, &mut locals);
        Self::stamp_transient_block_5(ctx, p, nodes, &mut locals);

        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(5),
            1,
            multiplicity,
        );

        let eq0_value: f64 = locals.var_vsar;
        stamper.stamp_potential_sparse_local::<4, 0>(
            0,
            eq0_value,
            [1, 4, 5, 6],
            [locals.var_vsar_dn1, locals.var_vsar_dn4, locals.var_vsar_dn5, locals.var_vsar_dn6],
            [],
            [],
        );
        let eq1_value: f64 = locals.var_ids;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq1_value),
            [1, 4, 5, 6],
            [multiplicity * (locals.var_ids_dn1), multiplicity * (locals.var_ids_dn4), multiplicity * (locals.var_ids_dn5), multiplicity * (locals.var_ids_dn6)],
            [],
            [],
            1.0,
        );
        let eq2_value: f64 = locals.var_vdar;
        stamper.stamp_potential_sparse_local::<4, 0>(
            1,
            eq2_value,
            [1, 4, 5, 6],
            [locals.var_vdar_dn1, locals.var_vdar_dn4, locals.var_vdar_dn5, locals.var_vdar_dn6],
            [],
            [],
        );
        let (eq4_e64, eq4_e64_d_n4,) = {
    if (locals.var_guard6 != 0.0) {
        let eq4_e61: f64 = (p.p36 * (nv4 - 0.0));
        let eq4_e62: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq4_e61);
        (eq4_e62, (p.p36 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e64;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq4_value),
            4,
            multiplicity * (eq4_e64_d_n4),
        );
        let (eq5_e68, eq5_e68_d_n1, eq5_e68_d_n4, eq5_e68_d_n5, eq5_e68_d_n6,) = {
    if (locals.var_guard6 != 0.0) {
        (locals.var_pdiss, locals.var_pdiss_dn1, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e68;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            None,
            multiplicity * (eq5_value),
            [1, 4, 5, 6],
            [multiplicity * (eq5_e68_d_n1), multiplicity * (eq5_e68_d_n4), multiplicity * (eq5_e68_d_n5), multiplicity * (eq5_e68_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq6_e74, eq6_e74_d_n4,) = {
    if (locals.var_guard6 != 0.0) {
        let eq6_e72: f64 = ((nv4 - 0.0) / p.p35);
        let eq6_e72_d_n4: f64 = (1.0 / p.p35);
        (eq6_e72, eq6_e72_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e74;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq6_value),
            4,
            multiplicity * (eq6_e74_d_n4),
        );
        let (eq7_e81, eq7_e81_d_n4,) = {
    if (locals.var_guard6 == 0.0) {
        let eq7_e79: f64 = ((nv4 - 0.0) * 1000000000.0);
        (eq7_e79, 1000000000.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e81;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq7_value),
            4,
            multiplicity * (eq7_e81_d_n4),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let multiplicity = (*self).multiplicity;
        let mut locals = StampLocals::default();

        let assign1950_e93260: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign1950_e93260;
        locals.var_guard6_rv = 0.0;

        let (eq4_e64, eq4_e64_d_n4, eq4_e64_q,) = {
    if (locals.var_guard6 != 0.0) {
        let eq4_e61: f64 = (p.p36 * (nv4 - 0.0));
        let eq4_e62_q: f64 = eq4_e61;
        (eq4_e61, p.p36, eq4_e62_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq4_e64_d_n4),
        );
    }
}
