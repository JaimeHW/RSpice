#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_awl_slot: &mut f64,
        var_deltat_slot: &mut f64,
        var_deltavfb_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eps_cox_slot: &mut f64,
        var_eps_cox_l_slot: &mut f64,
        var_eps_cox_w_slot: &mut f64,
        var_epssil_slot: &mut f64,
        var_eta_qi_slot: &mut f64,
        var_gamma_s_slot: &mut f64,
        var_gamma_sqrt_phi_slot: &mut f64,
        var_gamma_sqrt_phi_dn0_slot: &mut f64,
        var_gamma_sqrt_phi_dn1_slot: &mut f64,
        var_gamma_sqrt_phi_dn2_slot: &mut f64,
        var_gamma_sqrt_phi_dn3_slot: &mut f64,
        var_guard1_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_inv_ucrit_slot: &mut f64,
        var_inv_vt_slot: &mut f64,
        var_kp_t_slot: &mut f64,
        var_kp_weff_slot: &mut f64,
        var_lc_slot: &mut f64,
        var_lc_lambda_slot: &mut f64,
        var_lc_ucrit_slot: &mut f64,
        var_leff_slot: &mut f64,
        var_log_vc_vt_slot: &mut f64,
        var_mode_slot: &mut f64,
        var_phi_t_slot: &mut f64,
        var_phi_t_dn0_slot: &mut f64,
        var_phi_t_dn1_slot: &mut f64,
        var_phi_t_dn2_slot: &mut f64,
        var_phi_t_dn3_slot: &mut f64,
        var_phi_vd_slot: &mut f64,
        var_phi_vd_dn0_slot: &mut f64,
        var_phi_vd_dn1_slot: &mut f64,
        var_phi_vd_dn2_slot: &mut f64,
        var_phi_vd_dn3_slot: &mut f64,
        var_phi_vs_slot: &mut f64,
        var_phi_vs_dn0_slot: &mut f64,
        var_phi_vs_dn1_slot: &mut f64,
        var_phi_vs_dn2_slot: &mut f64,
        var_phi_vs_dn3_slot: &mut f64,
        var_ratiot_slot: &mut f64,
        var_refeg_slot: &mut f64,
        var_sqrt_phi_slot: &mut f64,
        var_sqrt_phi_dn0_slot: &mut f64,
        var_sqrt_phi_dn1_slot: &mut f64,
        var_sqrt_phi_dn2_slot: &mut f64,
        var_sqrt_phi_dn3_slot: &mut f64,
        var_sqrt_phi_vd_vt_slot: &mut f64,
        var_sqrt_phi_vd_vt_dn0_slot: &mut f64,
        var_sqrt_phi_vd_vt_dn1_slot: &mut f64,
        var_sqrt_phi_vd_vt_dn2_slot: &mut f64,
        var_sqrt_phi_vd_vt_dn3_slot: &mut f64,
        var_sqrt_phi_vs_slot: &mut f64,
        var_sqrt_phi_vs_dn0_slot: &mut f64,
        var_sqrt_phi_vs_dn1_slot: &mut f64,
        var_sqrt_phi_vs_dn2_slot: &mut f64,
        var_sqrt_phi_vs_dn3_slot: &mut f64,
        var_sqrt_phi_vs_vt_slot: &mut f64,
        var_sqrt_phi_vs_vt_dn0_slot: &mut f64,
        var_sqrt_phi_vs_vt_dn1_slot: &mut f64,
        var_sqrt_phi_vs_vt_dn2_slot: &mut f64,
        var_sqrt_phi_vs_vt_dn3_slot: &mut f64,
        var_sqrt_vgstar_slot: &mut f64,
        var_sqrt_vgstar_dn0_slot: &mut f64,
        var_sqrt_vgstar_dn1_slot: &mut f64,
        var_sqrt_vgstar_dn2_slot: &mut f64,
        var_sqrt_vgstar_dn3_slot: &mut f64,
        var_sqrt_vp_vt_slot: &mut f64,
        var_sqrt_vp_vt_dn0_slot: &mut f64,
        var_sqrt_vp_vt_dn1_slot: &mut f64,
        var_sqrt_vp_vt_dn2_slot: &mut f64,
        var_sqrt_vp_vt_dn3_slot: &mut f64,
        var_sqv_slot: &mut f64,
        var_t_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_theta_vp_1_slot: &mut f64,
        var_theta_vp_1_dn0_slot: &mut f64,
        var_theta_vp_1_dn1_slot: &mut f64,
        var_theta_vp_1_dn2_slot: &mut f64,
        var_theta_vp_1_dn3_slot: &mut f64,
        var_tmp1_slot: &mut f64,
        var_tmp1_dn0_slot: &mut f64,
        var_tmp1_dn1_slot: &mut f64,
        var_tmp1_dn2_slot: &mut f64,
        var_tmp1_dn3_slot: &mut f64,
        var_tmp2_slot: &mut f64,
        var_tmp2_dn0_slot: &mut f64,
        var_tmp2_dn1_slot: &mut f64,
        var_tmp2_dn2_slot: &mut f64,
        var_tmp2_dn3_slot: &mut f64,
        var_tnom_slot: &mut f64,
        var_ucrit_t_slot: &mut f64,
        var_v0_slot: &mut f64,
        var_vc_slot: &mut f64,
        var_vd_slot: &mut f64,
        var_vd_dn0_slot: &mut f64,
        var_vd_dn2_slot: &mut f64,
        var_vd_dn3_slot: &mut f64,
        var_vg_slot: &mut f64,
        var_vg_dn1_slot: &mut f64,
        var_vg_dn3_slot: &mut f64,
        var_vgprime_slot: &mut f64,
        var_vgprime_dn0_slot: &mut f64,
        var_vgprime_dn1_slot: &mut f64,
        var_vgprime_dn2_slot: &mut f64,
        var_vgprime_dn3_slot: &mut f64,
        var_vgstar_slot: &mut f64,
        var_vgstar_dn0_slot: &mut f64,
        var_vgstar_dn1_slot: &mut f64,
        var_vgstar_dn2_slot: &mut f64,
        var_vgstar_dn3_slot: &mut f64,
        var_vl_slot: &mut f64,
        var_vpprime_slot: &mut f64,
        var_vpprime_dn0_slot: &mut f64,
        var_vpprime_dn1_slot: &mut f64,
        var_vpprime_dn2_slot: &mut f64,
        var_vpprime_dn3_slot: &mut f64,
        var_vs_slot: &mut f64,
        var_vs_dn0_slot: &mut f64,
        var_vs_dn2_slot: &mut f64,
        var_vs_dn3_slot: &mut f64,
        var_vt_slot: &mut f64,
        var_vt_01_slot: &mut f64,
        var_vt_2_slot: &mut f64,
        var_vt_4_slot: &mut f64,
        var_vt_vt_slot: &mut f64,
        var_vt_vt_16_slot: &mut f64,
        var_vt_vt_2_slot: &mut f64,
        var_vto_s_slot: &mut f64,
        var_vto_t_slot: &mut f64,
        var_weff_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_awl: f64 = *var_awl_slot;
        let mut var_deltat: f64 = *var_deltat_slot;
        let mut var_deltavfb: f64 = *var_deltavfb_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eps_cox: f64 = *var_eps_cox_slot;
        let mut var_eps_cox_l: f64 = *var_eps_cox_l_slot;
        let mut var_eps_cox_w: f64 = *var_eps_cox_w_slot;
        let mut var_epssil: f64 = *var_epssil_slot;
        let mut var_eta_qi: f64 = *var_eta_qi_slot;
        let mut var_gamma_s: f64 = *var_gamma_s_slot;
        let mut var_gamma_sqrt_phi: f64 = *var_gamma_sqrt_phi_slot;
        let mut var_gamma_sqrt_phi_dn0: f64 = *var_gamma_sqrt_phi_dn0_slot;
        let mut var_gamma_sqrt_phi_dn1: f64 = *var_gamma_sqrt_phi_dn1_slot;
        let mut var_gamma_sqrt_phi_dn2: f64 = *var_gamma_sqrt_phi_dn2_slot;
        let mut var_gamma_sqrt_phi_dn3: f64 = *var_gamma_sqrt_phi_dn3_slot;
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_inv_ucrit: f64 = *var_inv_ucrit_slot;
        let mut var_inv_vt: f64 = *var_inv_vt_slot;
        let mut var_kp_t: f64 = *var_kp_t_slot;
        let mut var_kp_weff: f64 = *var_kp_weff_slot;
        let mut var_lc: f64 = *var_lc_slot;
        let mut var_lc_lambda: f64 = *var_lc_lambda_slot;
        let mut var_lc_ucrit: f64 = *var_lc_ucrit_slot;
        let mut var_leff: f64 = *var_leff_slot;
        let mut var_log_vc_vt: f64 = *var_log_vc_vt_slot;
        let mut var_mode: f64 = *var_mode_slot;
        let mut var_phi_t: f64 = *var_phi_t_slot;
        let mut var_phi_t_dn0: f64 = *var_phi_t_dn0_slot;
        let mut var_phi_t_dn1: f64 = *var_phi_t_dn1_slot;
        let mut var_phi_t_dn2: f64 = *var_phi_t_dn2_slot;
        let mut var_phi_t_dn3: f64 = *var_phi_t_dn3_slot;
        let mut var_phi_vd: f64 = *var_phi_vd_slot;
        let mut var_phi_vd_dn0: f64 = *var_phi_vd_dn0_slot;
        let mut var_phi_vd_dn1: f64 = *var_phi_vd_dn1_slot;
        let mut var_phi_vd_dn2: f64 = *var_phi_vd_dn2_slot;
        let mut var_phi_vd_dn3: f64 = *var_phi_vd_dn3_slot;
        let mut var_phi_vs: f64 = *var_phi_vs_slot;
        let mut var_phi_vs_dn0: f64 = *var_phi_vs_dn0_slot;
        let mut var_phi_vs_dn1: f64 = *var_phi_vs_dn1_slot;
        let mut var_phi_vs_dn2: f64 = *var_phi_vs_dn2_slot;
        let mut var_phi_vs_dn3: f64 = *var_phi_vs_dn3_slot;
        let mut var_ratiot: f64 = *var_ratiot_slot;
        let mut var_refeg: f64 = *var_refeg_slot;
        let mut var_sqrt_phi: f64 = *var_sqrt_phi_slot;
        let mut var_sqrt_phi_dn0: f64 = *var_sqrt_phi_dn0_slot;
        let mut var_sqrt_phi_dn1: f64 = *var_sqrt_phi_dn1_slot;
        let mut var_sqrt_phi_dn2: f64 = *var_sqrt_phi_dn2_slot;
        let mut var_sqrt_phi_dn3: f64 = *var_sqrt_phi_dn3_slot;
        let mut var_sqrt_phi_vd_vt: f64 = *var_sqrt_phi_vd_vt_slot;
        let mut var_sqrt_phi_vd_vt_dn0: f64 = *var_sqrt_phi_vd_vt_dn0_slot;
        let mut var_sqrt_phi_vd_vt_dn1: f64 = *var_sqrt_phi_vd_vt_dn1_slot;
        let mut var_sqrt_phi_vd_vt_dn2: f64 = *var_sqrt_phi_vd_vt_dn2_slot;
        let mut var_sqrt_phi_vd_vt_dn3: f64 = *var_sqrt_phi_vd_vt_dn3_slot;
        let mut var_sqrt_phi_vs: f64 = *var_sqrt_phi_vs_slot;
        let mut var_sqrt_phi_vs_dn0: f64 = *var_sqrt_phi_vs_dn0_slot;
        let mut var_sqrt_phi_vs_dn1: f64 = *var_sqrt_phi_vs_dn1_slot;
        let mut var_sqrt_phi_vs_dn2: f64 = *var_sqrt_phi_vs_dn2_slot;
        let mut var_sqrt_phi_vs_dn3: f64 = *var_sqrt_phi_vs_dn3_slot;
        let mut var_sqrt_phi_vs_vt: f64 = *var_sqrt_phi_vs_vt_slot;
        let mut var_sqrt_phi_vs_vt_dn0: f64 = *var_sqrt_phi_vs_vt_dn0_slot;
        let mut var_sqrt_phi_vs_vt_dn1: f64 = *var_sqrt_phi_vs_vt_dn1_slot;
        let mut var_sqrt_phi_vs_vt_dn2: f64 = *var_sqrt_phi_vs_vt_dn2_slot;
        let mut var_sqrt_phi_vs_vt_dn3: f64 = *var_sqrt_phi_vs_vt_dn3_slot;
        let mut var_sqrt_vgstar: f64 = *var_sqrt_vgstar_slot;
        let mut var_sqrt_vgstar_dn0: f64 = *var_sqrt_vgstar_dn0_slot;
        let mut var_sqrt_vgstar_dn1: f64 = *var_sqrt_vgstar_dn1_slot;
        let mut var_sqrt_vgstar_dn2: f64 = *var_sqrt_vgstar_dn2_slot;
        let mut var_sqrt_vgstar_dn3: f64 = *var_sqrt_vgstar_dn3_slot;
        let mut var_sqrt_vp_vt: f64 = *var_sqrt_vp_vt_slot;
        let mut var_sqrt_vp_vt_dn0: f64 = *var_sqrt_vp_vt_dn0_slot;
        let mut var_sqrt_vp_vt_dn1: f64 = *var_sqrt_vp_vt_dn1_slot;
        let mut var_sqrt_vp_vt_dn2: f64 = *var_sqrt_vp_vt_dn2_slot;
        let mut var_sqrt_vp_vt_dn3: f64 = *var_sqrt_vp_vt_dn3_slot;
        let mut var_sqv: f64 = *var_sqv_slot;
        let mut var_t: f64 = *var_t_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_theta_vp_1: f64 = *var_theta_vp_1_slot;
        let mut var_theta_vp_1_dn0: f64 = *var_theta_vp_1_dn0_slot;
        let mut var_theta_vp_1_dn1: f64 = *var_theta_vp_1_dn1_slot;
        let mut var_theta_vp_1_dn2: f64 = *var_theta_vp_1_dn2_slot;
        let mut var_theta_vp_1_dn3: f64 = *var_theta_vp_1_dn3_slot;
        let mut var_tmp1: f64 = *var_tmp1_slot;
        let mut var_tmp1_dn0: f64 = *var_tmp1_dn0_slot;
        let mut var_tmp1_dn1: f64 = *var_tmp1_dn1_slot;
        let mut var_tmp1_dn2: f64 = *var_tmp1_dn2_slot;
        let mut var_tmp1_dn3: f64 = *var_tmp1_dn3_slot;
        let mut var_tmp2: f64 = *var_tmp2_slot;
        let mut var_tmp2_dn0: f64 = *var_tmp2_dn0_slot;
        let mut var_tmp2_dn1: f64 = *var_tmp2_dn1_slot;
        let mut var_tmp2_dn2: f64 = *var_tmp2_dn2_slot;
        let mut var_tmp2_dn3: f64 = *var_tmp2_dn3_slot;
        let mut var_tnom: f64 = *var_tnom_slot;
        let mut var_ucrit_t: f64 = *var_ucrit_t_slot;
        let mut var_v0: f64 = *var_v0_slot;
        let mut var_vc: f64 = *var_vc_slot;
        let mut var_vd: f64 = *var_vd_slot;
        let mut var_vd_dn0: f64 = *var_vd_dn0_slot;
        let mut var_vd_dn2: f64 = *var_vd_dn2_slot;
        let mut var_vd_dn3: f64 = *var_vd_dn3_slot;
        let mut var_vg: f64 = *var_vg_slot;
        let mut var_vg_dn1: f64 = *var_vg_dn1_slot;
        let mut var_vg_dn3: f64 = *var_vg_dn3_slot;
        let mut var_vgprime: f64 = *var_vgprime_slot;
        let mut var_vgprime_dn0: f64 = *var_vgprime_dn0_slot;
        let mut var_vgprime_dn1: f64 = *var_vgprime_dn1_slot;
        let mut var_vgprime_dn2: f64 = *var_vgprime_dn2_slot;
        let mut var_vgprime_dn3: f64 = *var_vgprime_dn3_slot;
        let mut var_vgstar: f64 = *var_vgstar_slot;
        let mut var_vgstar_dn0: f64 = *var_vgstar_dn0_slot;
        let mut var_vgstar_dn1: f64 = *var_vgstar_dn1_slot;
        let mut var_vgstar_dn2: f64 = *var_vgstar_dn2_slot;
        let mut var_vgstar_dn3: f64 = *var_vgstar_dn3_slot;
        let mut var_vl: f64 = *var_vl_slot;
        let mut var_vpprime: f64 = *var_vpprime_slot;
        let mut var_vpprime_dn0: f64 = *var_vpprime_dn0_slot;
        let mut var_vpprime_dn1: f64 = *var_vpprime_dn1_slot;
        let mut var_vpprime_dn2: f64 = *var_vpprime_dn2_slot;
        let mut var_vpprime_dn3: f64 = *var_vpprime_dn3_slot;
        let mut var_vs: f64 = *var_vs_slot;
        let mut var_vs_dn0: f64 = *var_vs_dn0_slot;
        let mut var_vs_dn2: f64 = *var_vs_dn2_slot;
        let mut var_vs_dn3: f64 = *var_vs_dn3_slot;
        let mut var_vt: f64 = *var_vt_slot;
        let mut var_vt_01: f64 = *var_vt_01_slot;
        let mut var_vt_2: f64 = *var_vt_2_slot;
        let mut var_vt_4: f64 = *var_vt_4_slot;
        let mut var_vt_vt: f64 = *var_vt_vt_slot;
        let mut var_vt_vt_16: f64 = *var_vt_vt_16_slot;
        let mut var_vt_vt_2: f64 = *var_vt_vt_2_slot;
        let mut var_vto_s: f64 = *var_vto_s_slot;
        let mut var_vto_t: f64 = *var_vto_t_slot;
        let mut var_weff: f64 = *var_weff_slot;

        let assign10_e194: f64 = (11.7 * 8.8541879239442e-12);
        var_epssil = assign10_e194;

        var_theta_vp_1 = 0.0;
        var_theta_vp_1_dn0 = 0.0;
        var_theta_vp_1_dn1 = 0.0;
        var_theta_vp_1_dn2 = 0.0;
        var_theta_vp_1_dn3 = 0.0;

        var_vpprime = 0.0;
        var_vpprime_dn0 = 0.0;
        var_vpprime_dn1 = 0.0;
        var_vpprime_dn2 = 0.0;
        var_vpprime_dn3 = 0.0;

        var_sqrt_vp_vt = 0.0;
        var_sqrt_vp_vt_dn0 = 0.0;
        var_sqrt_vp_vt_dn1 = 0.0;
        var_sqrt_vp_vt_dn2 = 0.0;
        var_sqrt_vp_vt_dn3 = 0.0;

        let assign60_e201: f64 = (var_epssil / p.p13);
        var_eps_cox = assign60_e201;

        let assign70_e204: f64 = (var_eps_cox * p.p14);
        let assign70_e205: f64 = (assign70_e204).sqrt();
        var_lc = assign70_e205;

        let assign80_e208: f64 = (var_lc * p.p25);
        var_lc_lambda = assign80_e208;

        let assign90_e211: f64 = (3.0 * var_eps_cox);
        let assign90_e213: f64 = (assign90_e211 * p.p28);
        var_eps_cox_w = assign90_e213;

        let assign100_e216: f64 = (var_eps_cox * p.p29);
        var_eps_cox_l = assign100_e216;

        let assign120_e223: f64 = (var_epssil * p.p22);
        let assign120_e224: f64 = (p.p13 / assign120_e223);
        var_t0 = assign120_e224;

        let assign130_e227: f64 = (p.p30 + p.p30);
        let assign130_e229: f64 = (assign130_e227 / p.p13);
        var_v0 = assign130_e229;

        let (assign140_e235,) = {
    if (p.p0 > 0.0) {
        (0.5,)
    } else {
        (0.3333333333333,)
    }
};
        var_eta_qi = assign140_e235;

        let assign150_e238: f64 = (-1e21);
        let assign150_e239: f64 = (-assign150_e238);
        let assign150_e240: f64 = if p.p3 == assign150_e239 { 1.0 } else { 0.0 };
        var_guard1 = assign150_e240;

        let (assign160_e246,) = {
    if (var_guard1 != 0.0) {
        let assign160_e242: f64 = ctx_temp;
        let assign160_e244: f64 = (assign160_e242 + p.p2);
        (assign160_e244,)
    } else {
        (var_t,)
    }
};
        var_t = assign160_e246;

        let (assign170_e253,) = {
    if (var_guard1 == 0.0) {
        let assign170_e251: f64 = (p.p3 + 273.15);
        (assign170_e251,)
    } else {
        (var_t,)
    }
};
        var_t = assign170_e253;

        let assign180_e256: f64 = (-1e21);
        let assign180_e257: f64 = (-assign180_e256);
        let assign180_e258: f64 = if p.p4 == assign180_e257 { 1.0 } else { 0.0 };
        var_guard2 = assign180_e258;

        let (assign190_e264,) = {
    if (var_guard2 != 0.0) {
        let assign190_e262: f64 = (25.0 + 273.15);
        (assign190_e262,)
    } else {
        (var_tnom,)
    }
};
        var_tnom = assign190_e264;

        let (assign200_e271,) = {
    if (var_guard2 == 0.0) {
        let assign200_e269: f64 = (p.p4 + 273.15);
        (assign200_e269,)
    } else {
        (var_tnom,)
    }
};
        var_tnom = assign200_e271;

        let assign210_e273: f64 = (var_t * THERMAL_VOLTAGE_PER_K);
        var_vt = assign210_e273;

        let assign220_e276: f64 = (0.1 * var_vt);
        var_vt_01 = assign220_e276;

        let assign230_e279: f64 = (1.0 / var_vt);
        var_inv_vt = assign230_e279;

        let assign240_e282: f64 = (var_vt + var_vt);
        var_vt_2 = assign240_e282;

        let assign250_e285: f64 = (var_vt_2 + var_vt_2);
        var_vt_4 = assign250_e285;

        let assign260_e288: f64 = (var_vt * var_vt);
        var_vt_vt = assign260_e288;

        let assign270_e291: f64 = (var_vt_vt + var_vt_vt);
        var_vt_vt_2 = assign270_e291;

        let assign280_e294: f64 = (16.0 * var_vt_vt);
        var_vt_vt_16 = assign280_e294;

        let assign290_e298: f64 = (0.000702 * var_t);
        let assign290_e300: f64 = (assign290_e298 * var_t);
        let assign290_e303: f64 = (var_t + 1108.0);
        let assign290_e304: f64 = (assign290_e300 / assign290_e303);
        let assign290_e305: f64 = (1.16 - assign290_e304);
        var_eg = assign290_e305;

        let assign300_e309: f64 = (0.000702 * var_tnom);
        let assign300_e311: f64 = (assign300_e309 * var_tnom);
        let assign300_e314: f64 = (var_tnom + 1108.0);
        let assign300_e315: f64 = (assign300_e311 / assign300_e314);
        let assign300_e316: f64 = (1.16 - assign300_e315);
        var_refeg = assign300_e316;

        let assign310_e319: f64 = (var_t - var_tnom);
        var_deltat = assign310_e319;

        let assign320_e322: f64 = (var_t / var_tnom);
        var_ratiot = assign320_e322;

        let assign330_e326: f64 = (p.p16 * var_deltat);
        let assign330_e327: f64 = (p.p15 - assign330_e326);
        var_vto_t = assign330_e327;

        let assign340_e331: f64 = (var_ratiot).powf(p.p20);
        let assign340_e332: f64 = (p.p19 * assign340_e331);
        var_kp_t = assign340_e332;

        let assign350_e336: f64 = (var_ratiot).powf(p.p24);
        let assign350_e337: f64 = (p.p23 * assign350_e336);
        var_ucrit_t = assign350_e337;

        let assign370_e347: f64 = (p.p18 * var_ratiot);
        let assign370_e350: f64 = (3.0 * var_vt);
        let assign370_e352: f64 = (var_ratiot).ln();
        let assign370_e353: f64 = (assign370_e350 * assign370_e352);
        let assign370_e354: f64 = (assign370_e347 - assign370_e353);
        let assign370_e357: f64 = (var_refeg * var_ratiot);
        let assign370_e358: f64 = (assign370_e354 - assign370_e357);
        let assign370_e360: f64 = (assign370_e358 + var_eg);
        var_phi_t = assign370_e360;
        var_phi_t_dn0 = 0.0;
        var_phi_t_dn1 = 0.0;
        var_phi_t_dn2 = 0.0;
        var_phi_t_dn3 = 0.0;

        var_tmp1 = 0.2;
        var_tmp1_dn0 = 0.0;
        var_tmp1_dn1 = 0.0;
        var_tmp1_dn2 = 0.0;
        var_tmp1_dn3 = 0.0;

        let assign390_e364: f64 = (var_phi_t - var_tmp1);
        var_tmp2 = assign390_e364;
        var_tmp2_dn0 = (var_phi_t_dn0 - var_tmp1_dn0);
        var_tmp2_dn1 = (var_phi_t_dn1 - var_tmp1_dn1);
        var_tmp2_dn2 = (var_phi_t_dn2 - var_tmp1_dn2);
        var_tmp2_dn3 = (var_phi_t_dn3 - var_tmp1_dn3);

        let assign400_e369: f64 = (var_tmp2 * var_tmp2);
        let assign400_e372: f64 = (var_vt * var_vt);
        let assign400_e373: f64 = (assign400_e369 + assign400_e372);
        let assign400_e374: f64 = (assign400_e373).sqrt();
        let assign400_e375: f64 = (var_tmp2 + assign400_e374);
        let assign400_e376: f64 = (0.5 * assign400_e375);
        let assign400_e378: f64 = (assign400_e376 + var_tmp1);
        var_phi_t = assign400_e378;
        var_phi_t_dn0 = ((0.5 * (var_tmp2_dn0 + (((var_tmp2_dn0 * var_tmp2) + (var_tmp2 * var_tmp2_dn0)) / (2.0 * assign400_e374)))) + var_tmp1_dn0);
        var_phi_t_dn1 = ((0.5 * (var_tmp2_dn1 + (((var_tmp2_dn1 * var_tmp2) + (var_tmp2 * var_tmp2_dn1)) / (2.0 * assign400_e374)))) + var_tmp1_dn1);
        var_phi_t_dn2 = ((0.5 * (var_tmp2_dn2 + (((var_tmp2_dn2 * var_tmp2) + (var_tmp2 * var_tmp2_dn2)) / (2.0 * assign400_e374)))) + var_tmp1_dn2);
        var_phi_t_dn3 = ((0.5 * (var_tmp2_dn3 + (((var_tmp2_dn3 * var_tmp2) + (var_tmp2 * var_tmp2_dn3)) / (2.0 * assign400_e374)))) + var_tmp1_dn3);

        let assign410_e380: f64 = (var_phi_t).sqrt();
        var_sqrt_phi = assign410_e380;
        var_sqrt_phi_dn0 = (var_phi_t_dn0 / (2.0 * assign410_e380));
        var_sqrt_phi_dn1 = (var_phi_t_dn1 / (2.0 * assign410_e380));
        var_sqrt_phi_dn2 = (var_phi_t_dn2 / (2.0 * assign410_e380));
        var_sqrt_phi_dn3 = (var_phi_t_dn3 / (2.0 * assign410_e380));

        let assign420_e383: f64 = (1.0 / var_ucrit_t);
        var_inv_ucrit = assign420_e383;

        let assign430_e386: f64 = (var_lc * var_ucrit_t);
        var_lc_ucrit = assign430_e386;

        let assign460_e395: f64 = (p.p5 + p.p26);
        var_leff = assign460_e395;

        let assign470_e398: f64 = (p.p6 + p.p27);
        var_weff = assign470_e398;

        let assign480_e401: f64 = (var_ucrit_t * var_leff);
        var_vc = assign480_e401;

        let assign490_e405: f64 = (0.5 * var_vc);
        let assign490_e407: f64 = (assign490_e405 * var_inv_vt);
        let assign490_e408: f64 = (assign490_e407).ln();
        let assign490_e410: f64 = (assign490_e408 - 0.6);
        let assign490_e411: f64 = (var_vt * assign490_e410);
        var_log_vc_vt = assign490_e411;

        let assign500_e415: f64 = (var_weff * var_leff);
        let assign500_e416: f64 = (assign500_e415).sqrt();
        let assign500_e417: f64 = (1.0 / assign500_e416);
        var_awl = assign500_e417;

        let assign510_e420: f64 = if p.p0 > 0.0 { 1.0 } else { 0.0 };
        var_guard3 = assign510_e420;

        let (assign520_e435,) = {
    if (var_guard3 != 0.0) {
        let (assign520_e433,) = {
            if (p.p38 != 1e-6) {
                let assign520_e428: f64 = (p.p38 - 1e-6);
                let assign520_e429: f64 = (var_awl * assign520_e428);
                let assign520_e431: f64 = (assign520_e429 + var_vto_t);
                (assign520_e431,)
            } else {
                (var_vto_t,)
            }
        };
        (assign520_e433,)
    } else {
        (var_vto_s,)
    }
};
        var_vto_s = assign520_e435;

        let (assign530_e452,) = {
    if (var_guard3 == 0.0) {
        let (assign530_e450,) = {
            if (p.p38 != 1e-6) {
                let assign530_e444: f64 = (1e-6 - p.p38);
                let assign530_e445: f64 = (var_awl * assign530_e444);
                let assign530_e447: f64 = (assign530_e445 - var_vto_t);
                (assign530_e447,)
            } else {
                let assign530_e449: f64 = (-var_vto_t);
                (assign530_e449,)
            }
        };
        (assign530_e450,)
    } else {
        (var_vto_s,)
    }
};
        var_vto_s = assign530_e452;

        let (assign540_e467,) = {
    if (p.p39 != 1e-6) {
        let assign540_e461: f64 = (p.p39 - 1e-6);
        let assign540_e463: f64 = (assign540_e461 * var_awl);
        let assign540_e464: f64 = (1.0 + assign540_e463);
        let assign540_e465: f64 = (var_kp_t * assign540_e464);
        (assign540_e465,)
    } else {
        (var_kp_t,)
    }
};
        let assign540_e468: f64 = (var_weff * assign540_e467);
        var_kp_weff = assign540_e468;

        let (assign550_e480,) = {
    if (p.p40 != 1e-6) {
        let assign550_e475: f64 = (p.p40 - 1e-6);
        let assign550_e477: f64 = (assign550_e475 * var_awl);
        let assign550_e478: f64 = (p.p17 + assign550_e477);
        (assign550_e478,)
    } else {
        (p.p17,)
    }
};
        var_gamma_s = assign550_e480;

        let assign560_e483: f64 = (var_gamma_s * var_sqrt_phi);
        var_gamma_sqrt_phi = assign560_e483;
        var_gamma_sqrt_phi_dn0 = (var_gamma_s * var_sqrt_phi_dn0);
        var_gamma_sqrt_phi_dn1 = (var_gamma_s * var_sqrt_phi_dn1);
        var_gamma_sqrt_phi_dn2 = (var_gamma_s * var_sqrt_phi_dn2);
        var_gamma_sqrt_phi_dn3 = (var_gamma_s * var_sqrt_phi_dn3);

        let assign570_e486: f64 = if var_v0 == 0.0 { 1.0 } else { 0.0 };
        var_guard4 = assign570_e486;

        let (assign580_e490,) = {
    if (var_guard4 != 0.0) {
        (0.0,)
    } else {
        (var_deltavfb,)
    }
};
        var_deltavfb = assign580_e490;

        let (assign590_e503,) = {
    if (var_guard4 == 0.0) {
        let assign590_e497: f64 = (p.p31 * p.p8);
        let assign590_e498: f64 = (var_leff / assign590_e497);
        let assign590_e500: f64 = (assign590_e498 - 0.1);
        let assign590_e501: f64 = (0.28 * assign590_e500);
        (assign590_e501,)
    } else {
        (var_vl,)
    }
};
        var_vl = assign590_e503;

        let (assign600_e521,) = {
    if (var_guard4 == 0.0) {
        let assign600_e512: f64 = (var_vl * var_vl);
        let assign600_e514: f64 = (assign600_e512 + 0.001936);
        let assign600_e515: f64 = (assign600_e514).sqrt();
        let assign600_e516: f64 = (var_vl + assign600_e515);
        let assign600_e517: f64 = (0.5 * assign600_e516);
        let assign600_e518: f64 = (1.0 + assign600_e517);
        let assign600_e519: f64 = (1.0 / assign600_e518);
        (assign600_e519,)
    } else {
        (var_sqv,)
    }
};
        var_sqv = assign600_e521;

        let (assign610_e530,) = {
    if (var_guard4 == 0.0) {
        let assign610_e526: f64 = (var_v0 * var_sqv);
        let assign610_e528: f64 = (assign610_e526 * var_sqv);
        (assign610_e528,)
    } else {
        (var_deltavfb,)
    }
};
        var_deltavfb = assign610_e530;

        let assign620_e533: f64 = (p.p0 * (nv1 - nv3));
        var_vg = assign620_e533;
        var_vg_dn1 = p.p0;
        var_vg_dn3 = (-p.p0);

        let assign630_e536: f64 = (p.p0 * (nv2 - nv3));
        var_vs = assign630_e536;
        var_vs_dn0 = 0.0;
        var_vs_dn2 = p.p0;
        var_vs_dn3 = (-p.p0);

        let assign640_e539: f64 = (p.p0 * (nv0 - nv3));
        var_vd = assign640_e539;
        var_vd_dn0 = p.p0;
        var_vd_dn2 = 0.0;
        var_vd_dn3 = (-p.p0);

        let assign650_e542: f64 = (var_vd - var_vs);
        let assign650_e544: f64 = if assign650_e542 < 0.0 { 1.0 } else { 0.0 };
        var_guard6 = assign650_e544;

        let (assign660_e549,) = {
    if (var_guard6 != 0.0) {
        let assign660_e547: f64 = (-1.0);
        (assign660_e547,)
    } else {
        (var_mode,)
    }
};
        var_mode = assign660_e549;

        let (assign670_e553, assign670_e553_d_n0, assign670_e553_d_n2, assign670_e553_d_n3,) = {
    if (var_guard6 != 0.0) {
        (var_vs, var_vs_dn0, var_vs_dn2, var_vs_dn3,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3,)
    }
};
        var_t1 = assign670_e553;
        var_t1_dn0 = assign670_e553_d_n0;
        var_t1_dn2 = assign670_e553_d_n2;
        var_t1_dn3 = assign670_e553_d_n3;

        let (assign680_e557, assign680_e557_d_n0, assign680_e557_d_n2, assign680_e557_d_n3,) = {
    if (var_guard6 != 0.0) {
        (var_vd, var_vd_dn0, var_vd_dn2, var_vd_dn3,)
    } else {
        (var_vs, var_vs_dn0, var_vs_dn2, var_vs_dn3,)
    }
};
        var_vs = assign680_e557;
        var_vs_dn0 = assign680_e557_d_n0;
        var_vs_dn2 = assign680_e557_d_n2;
        var_vs_dn3 = assign680_e557_d_n3;

        let (assign690_e561, assign690_e561_d_n0, assign690_e561_d_n2, assign690_e561_d_n3,) = {
    if (var_guard6 != 0.0) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3,)
    } else {
        (var_vd, var_vd_dn0, var_vd_dn2, var_vd_dn3,)
    }
};
        var_vd = assign690_e561;
        var_vd_dn0 = assign690_e561_d_n0;
        var_vd_dn2 = assign690_e561_d_n2;
        var_vd_dn3 = assign690_e561_d_n3;

        let (assign700_e566,) = {
    if (var_guard6 == 0.0) {
        (1.0,)
    } else {
        (var_mode,)
    }
};
        var_mode = assign700_e566;

        let assign710_e569: f64 = (var_vg - var_vto_s);
        let assign710_e571: f64 = (assign710_e569 - var_deltavfb);
        let assign710_e573: f64 = (assign710_e571 + var_phi_t);
        let assign710_e575: f64 = (assign710_e573 + var_gamma_sqrt_phi);
        var_vgstar = assign710_e575;
        var_vgstar_dn0 = (var_phi_t_dn0 + var_gamma_sqrt_phi_dn0);
        var_vgstar_dn1 = ((var_vg_dn1 + var_phi_t_dn1) + var_gamma_sqrt_phi_dn1);
        var_vgstar_dn2 = (var_phi_t_dn2 + var_gamma_sqrt_phi_dn2);
        var_vgstar_dn3 = ((var_vg_dn3 + var_phi_t_dn3) + var_gamma_sqrt_phi_dn3);

        let assign720_e578: f64 = (var_vgstar * var_vgstar);
        let assign720_e581: f64 = (2.0 * var_vt_vt_16);
        let assign720_e582: f64 = (assign720_e578 + assign720_e581);
        let assign720_e583: f64 = (assign720_e582).sqrt();
        var_sqrt_vgstar = assign720_e583;
        var_sqrt_vgstar_dn0 = (((var_vgstar_dn0 * var_vgstar) + (var_vgstar * var_vgstar_dn0)) / (2.0 * assign720_e583));
        var_sqrt_vgstar_dn1 = (((var_vgstar_dn1 * var_vgstar) + (var_vgstar * var_vgstar_dn1)) / (2.0 * assign720_e583));
        var_sqrt_vgstar_dn2 = (((var_vgstar_dn2 * var_vgstar) + (var_vgstar * var_vgstar_dn2)) / (2.0 * assign720_e583));
        var_sqrt_vgstar_dn3 = (((var_vgstar_dn3 * var_vgstar) + (var_vgstar * var_vgstar_dn3)) / (2.0 * assign720_e583));

        let assign730_e587: f64 = (var_vgstar + var_sqrt_vgstar);
        let assign730_e588: f64 = (0.5 * assign730_e587);
        var_vgprime = assign730_e588;
        var_vgprime_dn0 = (0.5 * (var_vgstar_dn0 + var_sqrt_vgstar_dn0));
        var_vgprime_dn1 = (0.5 * (var_vgstar_dn1 + var_sqrt_vgstar_dn1));
        var_vgprime_dn2 = (0.5 * (var_vgstar_dn2 + var_sqrt_vgstar_dn2));
        var_vgprime_dn3 = (0.5 * (var_vgstar_dn3 + var_sqrt_vgstar_dn3));

        let assign740_e591: f64 = (var_phi_t + var_vs);
        var_phi_vs = assign740_e591;
        var_phi_vs_dn0 = (var_phi_t_dn0 + var_vs_dn0);
        var_phi_vs_dn1 = var_phi_t_dn1;
        var_phi_vs_dn2 = (var_phi_t_dn2 + var_vs_dn2);
        var_phi_vs_dn3 = (var_phi_t_dn3 + var_vs_dn3);

        let assign750_e594: f64 = (var_phi_vs * var_phi_vs);
        let assign750_e596: f64 = (assign750_e594 + var_vt_vt_16);
        let assign750_e597: f64 = (assign750_e596).sqrt();
        var_sqrt_phi_vs_vt = assign750_e597;
        var_sqrt_phi_vs_vt_dn0 = (((var_phi_vs_dn0 * var_phi_vs) + (var_phi_vs * var_phi_vs_dn0)) / (2.0 * assign750_e597));
        var_sqrt_phi_vs_vt_dn1 = (((var_phi_vs_dn1 * var_phi_vs) + (var_phi_vs * var_phi_vs_dn1)) / (2.0 * assign750_e597));
        var_sqrt_phi_vs_vt_dn2 = (((var_phi_vs_dn2 * var_phi_vs) + (var_phi_vs * var_phi_vs_dn2)) / (2.0 * assign750_e597));
        var_sqrt_phi_vs_vt_dn3 = (((var_phi_vs_dn3 * var_phi_vs) + (var_phi_vs * var_phi_vs_dn3)) / (2.0 * assign750_e597));

        let assign760_e601: f64 = (var_phi_vs + var_sqrt_phi_vs_vt);
        let assign760_e602: f64 = (0.5 * assign760_e601);
        let assign760_e603: f64 = (assign760_e602).sqrt();
        var_sqrt_phi_vs = assign760_e603;
        var_sqrt_phi_vs_dn0 = ((0.5 * (var_phi_vs_dn0 + var_sqrt_phi_vs_vt_dn0)) / (2.0 * assign760_e603));
        var_sqrt_phi_vs_dn1 = ((0.5 * (var_phi_vs_dn1 + var_sqrt_phi_vs_vt_dn1)) / (2.0 * assign760_e603));
        var_sqrt_phi_vs_dn2 = ((0.5 * (var_phi_vs_dn2 + var_sqrt_phi_vs_vt_dn2)) / (2.0 * assign760_e603));
        var_sqrt_phi_vs_dn3 = ((0.5 * (var_phi_vs_dn3 + var_sqrt_phi_vs_vt_dn3)) / (2.0 * assign760_e603));

        let assign770_e606: f64 = (var_phi_t + var_vd);
        var_phi_vd = assign770_e606;
        var_phi_vd_dn0 = (var_phi_t_dn0 + var_vd_dn0);
        var_phi_vd_dn1 = var_phi_t_dn1;
        var_phi_vd_dn2 = (var_phi_t_dn2 + var_vd_dn2);
        var_phi_vd_dn3 = (var_phi_t_dn3 + var_vd_dn3);

        let assign780_e609: f64 = (var_phi_vd * var_phi_vd);
        let assign780_e611: f64 = (assign780_e609 + var_vt_vt_16);
        let assign780_e612: f64 = (assign780_e611).sqrt();
        var_sqrt_phi_vd_vt = assign780_e612;
        var_sqrt_phi_vd_vt_dn0 = (((var_phi_vd_dn0 * var_phi_vd) + (var_phi_vd * var_phi_vd_dn0)) / (2.0 * assign780_e612));
        var_sqrt_phi_vd_vt_dn1 = (((var_phi_vd_dn1 * var_phi_vd) + (var_phi_vd * var_phi_vd_dn1)) / (2.0 * assign780_e612));
        var_sqrt_phi_vd_vt_dn2 = (((var_phi_vd_dn2 * var_phi_vd) + (var_phi_vd * var_phi_vd_dn2)) / (2.0 * assign780_e612));
        var_sqrt_phi_vd_vt_dn3 = (((var_phi_vd_dn3 * var_phi_vd) + (var_phi_vd * var_phi_vd_dn3)) / (2.0 * assign780_e612));

        *var_awl_slot = var_awl;
        *var_deltat_slot = var_deltat;
        *var_deltavfb_slot = var_deltavfb;
        *var_eg_slot = var_eg;
        *var_eps_cox_slot = var_eps_cox;
        *var_eps_cox_l_slot = var_eps_cox_l;
        *var_eps_cox_w_slot = var_eps_cox_w;
        *var_epssil_slot = var_epssil;
        *var_eta_qi_slot = var_eta_qi;
        *var_gamma_s_slot = var_gamma_s;
        *var_gamma_sqrt_phi_slot = var_gamma_sqrt_phi;
        *var_gamma_sqrt_phi_dn0_slot = var_gamma_sqrt_phi_dn0;
        *var_gamma_sqrt_phi_dn1_slot = var_gamma_sqrt_phi_dn1;
        *var_gamma_sqrt_phi_dn2_slot = var_gamma_sqrt_phi_dn2;
        *var_gamma_sqrt_phi_dn3_slot = var_gamma_sqrt_phi_dn3;
        *var_guard1_slot = var_guard1;
        *var_guard2_slot = var_guard2;
        *var_guard3_slot = var_guard3;
        *var_guard4_slot = var_guard4;
        *var_guard6_slot = var_guard6;
        *var_inv_ucrit_slot = var_inv_ucrit;
        *var_inv_vt_slot = var_inv_vt;
        *var_kp_t_slot = var_kp_t;
        *var_kp_weff_slot = var_kp_weff;
        *var_lc_slot = var_lc;
        *var_lc_lambda_slot = var_lc_lambda;
        *var_lc_ucrit_slot = var_lc_ucrit;
        *var_leff_slot = var_leff;
        *var_log_vc_vt_slot = var_log_vc_vt;
        *var_mode_slot = var_mode;
        *var_phi_t_slot = var_phi_t;
        *var_phi_t_dn0_slot = var_phi_t_dn0;
        *var_phi_t_dn1_slot = var_phi_t_dn1;
        *var_phi_t_dn2_slot = var_phi_t_dn2;
        *var_phi_t_dn3_slot = var_phi_t_dn3;
        *var_phi_vd_slot = var_phi_vd;
        *var_phi_vd_dn0_slot = var_phi_vd_dn0;
        *var_phi_vd_dn1_slot = var_phi_vd_dn1;
        *var_phi_vd_dn2_slot = var_phi_vd_dn2;
        *var_phi_vd_dn3_slot = var_phi_vd_dn3;
        *var_phi_vs_slot = var_phi_vs;
        *var_phi_vs_dn0_slot = var_phi_vs_dn0;
        *var_phi_vs_dn1_slot = var_phi_vs_dn1;
        *var_phi_vs_dn2_slot = var_phi_vs_dn2;
        *var_phi_vs_dn3_slot = var_phi_vs_dn3;
        *var_ratiot_slot = var_ratiot;
        *var_refeg_slot = var_refeg;
        *var_sqrt_phi_slot = var_sqrt_phi;
        *var_sqrt_phi_dn0_slot = var_sqrt_phi_dn0;
        *var_sqrt_phi_dn1_slot = var_sqrt_phi_dn1;
        *var_sqrt_phi_dn2_slot = var_sqrt_phi_dn2;
        *var_sqrt_phi_dn3_slot = var_sqrt_phi_dn3;
        *var_sqrt_phi_vd_vt_slot = var_sqrt_phi_vd_vt;
        *var_sqrt_phi_vd_vt_dn0_slot = var_sqrt_phi_vd_vt_dn0;
        *var_sqrt_phi_vd_vt_dn1_slot = var_sqrt_phi_vd_vt_dn1;
        *var_sqrt_phi_vd_vt_dn2_slot = var_sqrt_phi_vd_vt_dn2;
        *var_sqrt_phi_vd_vt_dn3_slot = var_sqrt_phi_vd_vt_dn3;
        *var_sqrt_phi_vs_slot = var_sqrt_phi_vs;
        *var_sqrt_phi_vs_dn0_slot = var_sqrt_phi_vs_dn0;
        *var_sqrt_phi_vs_dn1_slot = var_sqrt_phi_vs_dn1;
        *var_sqrt_phi_vs_dn2_slot = var_sqrt_phi_vs_dn2;
        *var_sqrt_phi_vs_dn3_slot = var_sqrt_phi_vs_dn3;
        *var_sqrt_phi_vs_vt_slot = var_sqrt_phi_vs_vt;
        *var_sqrt_phi_vs_vt_dn0_slot = var_sqrt_phi_vs_vt_dn0;
        *var_sqrt_phi_vs_vt_dn1_slot = var_sqrt_phi_vs_vt_dn1;
        *var_sqrt_phi_vs_vt_dn2_slot = var_sqrt_phi_vs_vt_dn2;
        *var_sqrt_phi_vs_vt_dn3_slot = var_sqrt_phi_vs_vt_dn3;
        *var_sqrt_vgstar_slot = var_sqrt_vgstar;
        *var_sqrt_vgstar_dn0_slot = var_sqrt_vgstar_dn0;
        *var_sqrt_vgstar_dn1_slot = var_sqrt_vgstar_dn1;
        *var_sqrt_vgstar_dn2_slot = var_sqrt_vgstar_dn2;
        *var_sqrt_vgstar_dn3_slot = var_sqrt_vgstar_dn3;
        *var_sqrt_vp_vt_slot = var_sqrt_vp_vt;
        *var_sqrt_vp_vt_dn0_slot = var_sqrt_vp_vt_dn0;
        *var_sqrt_vp_vt_dn1_slot = var_sqrt_vp_vt_dn1;
        *var_sqrt_vp_vt_dn2_slot = var_sqrt_vp_vt_dn2;
        *var_sqrt_vp_vt_dn3_slot = var_sqrt_vp_vt_dn3;
        *var_sqv_slot = var_sqv;
        *var_t_slot = var_t;
        *var_t0_slot = var_t0;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_theta_vp_1_slot = var_theta_vp_1;
        *var_theta_vp_1_dn0_slot = var_theta_vp_1_dn0;
        *var_theta_vp_1_dn1_slot = var_theta_vp_1_dn1;
        *var_theta_vp_1_dn2_slot = var_theta_vp_1_dn2;
        *var_theta_vp_1_dn3_slot = var_theta_vp_1_dn3;
        *var_tmp1_slot = var_tmp1;
        *var_tmp1_dn0_slot = var_tmp1_dn0;
        *var_tmp1_dn1_slot = var_tmp1_dn1;
        *var_tmp1_dn2_slot = var_tmp1_dn2;
        *var_tmp1_dn3_slot = var_tmp1_dn3;
        *var_tmp2_slot = var_tmp2;
        *var_tmp2_dn0_slot = var_tmp2_dn0;
        *var_tmp2_dn1_slot = var_tmp2_dn1;
        *var_tmp2_dn2_slot = var_tmp2_dn2;
        *var_tmp2_dn3_slot = var_tmp2_dn3;
        *var_tnom_slot = var_tnom;
        *var_ucrit_t_slot = var_ucrit_t;
        *var_v0_slot = var_v0;
        *var_vc_slot = var_vc;
        *var_vd_slot = var_vd;
        *var_vd_dn0_slot = var_vd_dn0;
        *var_vd_dn2_slot = var_vd_dn2;
        *var_vd_dn3_slot = var_vd_dn3;
        *var_vg_slot = var_vg;
        *var_vg_dn1_slot = var_vg_dn1;
        *var_vg_dn3_slot = var_vg_dn3;
        *var_vgprime_slot = var_vgprime;
        *var_vgprime_dn0_slot = var_vgprime_dn0;
        *var_vgprime_dn1_slot = var_vgprime_dn1;
        *var_vgprime_dn2_slot = var_vgprime_dn2;
        *var_vgprime_dn3_slot = var_vgprime_dn3;
        *var_vgstar_slot = var_vgstar;
        *var_vgstar_dn0_slot = var_vgstar_dn0;
        *var_vgstar_dn1_slot = var_vgstar_dn1;
        *var_vgstar_dn2_slot = var_vgstar_dn2;
        *var_vgstar_dn3_slot = var_vgstar_dn3;
        *var_vl_slot = var_vl;
        *var_vpprime_slot = var_vpprime;
        *var_vpprime_dn0_slot = var_vpprime_dn0;
        *var_vpprime_dn1_slot = var_vpprime_dn1;
        *var_vpprime_dn2_slot = var_vpprime_dn2;
        *var_vpprime_dn3_slot = var_vpprime_dn3;
        *var_vs_slot = var_vs;
        *var_vs_dn0_slot = var_vs_dn0;
        *var_vs_dn2_slot = var_vs_dn2;
        *var_vs_dn3_slot = var_vs_dn3;
        *var_vt_slot = var_vt;
        *var_vt_01_slot = var_vt_01;
        *var_vt_2_slot = var_vt_2;
        *var_vt_4_slot = var_vt_4;
        *var_vt_vt_slot = var_vt_vt;
        *var_vt_vt_16_slot = var_vt_vt_16;
        *var_vt_vt_2_slot = var_vt_vt_2;
        *var_vto_s_slot = var_vto_s;
        *var_vto_t_slot = var_vto_t;
        *var_weff_slot = var_weff;
    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        var_eps_cox_l: f64,
        var_eps_cox_w: f64,
        var_gamma_s: f64,
        var_inv_vt: f64,
        var_leff: f64,
        var_log_vc_vt: f64,
        var_phi_t: f64,
        var_phi_t_dn0: f64,
        var_phi_t_dn1: f64,
        var_phi_t_dn2: f64,
        var_phi_t_dn3: f64,
        var_phi_vd: f64,
        var_phi_vd_dn0: f64,
        var_phi_vd_dn1: f64,
        var_phi_vd_dn2: f64,
        var_phi_vd_dn3: f64,
        var_sqrt_phi_vd_vt: f64,
        var_sqrt_phi_vd_vt_dn0: f64,
        var_sqrt_phi_vd_vt_dn1: f64,
        var_sqrt_phi_vd_vt_dn2: f64,
        var_sqrt_phi_vd_vt_dn3: f64,
        var_sqrt_phi_vs: f64,
        var_sqrt_phi_vs_dn0: f64,
        var_sqrt_phi_vs_dn1: f64,
        var_sqrt_phi_vs_dn2: f64,
        var_sqrt_phi_vs_dn3: f64,
        var_vc: f64,
        var_vd: f64,
        var_vd_dn0: f64,
        var_vd_dn2: f64,
        var_vd_dn3: f64,
        var_vgprime: f64,
        var_vgprime_dn0: f64,
        var_vgprime_dn1: f64,
        var_vgprime_dn2: f64,
        var_vgprime_dn3: f64,
        var_vs: f64,
        var_vs_dn0: f64,
        var_vs_dn2: f64,
        var_vs_dn3: f64,
        var_vt: f64,
        var_vt_01: f64,
        var_vt_vt_16: f64,
        var_weff: f64,
        var_big_sqrt_vp_slot: &mut f64,
        var_big_sqrt_vp0_slot: &mut f64,
        var_big_sqrt_vp0_dn0_slot: &mut f64,
        var_big_sqrt_vp0_dn1_slot: &mut f64,
        var_big_sqrt_vp0_dn2_slot: &mut f64,
        var_big_sqrt_vp0_dn3_slot: &mut f64,
        var_big_sqrt_vp_dn0_slot: &mut f64,
        var_big_sqrt_vp_dn1_slot: &mut f64,
        var_big_sqrt_vp_dn2_slot: &mut f64,
        var_big_sqrt_vp_dn3_slot: &mut f64,
        var_deltav_2_slot: &mut f64,
        var_deltav_2_dn0_slot: &mut f64,
        var_deltav_2_dn1_slot: &mut f64,
        var_deltav_2_dn2_slot: &mut f64,
        var_deltav_2_dn3_slot: &mut f64,
        var_dif_dv_slot: &mut f64,
        var_dif_dv_dn0_slot: &mut f64,
        var_dif_dv_dn1_slot: &mut f64,
        var_dif_dv_dn2_slot: &mut f64,
        var_dif_dv_dn3_slot: &mut f64,
        var_gammaprime_slot: &mut f64,
        var_gammaprime_dn0_slot: &mut f64,
        var_gammaprime_dn1_slot: &mut f64,
        var_gammaprime_dn2_slot: &mut f64,
        var_gammaprime_dn3_slot: &mut f64,
        var_gammastar_slot: &mut f64,
        var_gammastar_dn0_slot: &mut f64,
        var_gammastar_dn1_slot: &mut f64,
        var_gammastar_dn2_slot: &mut f64,
        var_gammastar_dn3_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_if__slot: &mut f64,
        var_if__dn0_slot: &mut f64,
        var_if__dn1_slot: &mut f64,
        var_if__dn2_slot: &mut f64,
        var_if__dn3_slot: &mut f64,
        var_leta_l_slot: &mut f64,
        var_sqrt_gammastar_slot: &mut f64,
        var_sqrt_gammastar_dn0_slot: &mut f64,
        var_sqrt_gammastar_dn1_slot: &mut f64,
        var_sqrt_gammastar_dn2_slot: &mut f64,
        var_sqrt_gammastar_dn3_slot: &mut f64,
        var_sqrt_if_slot: &mut f64,
        var_sqrt_if_dn0_slot: &mut f64,
        var_sqrt_if_dn1_slot: &mut f64,
        var_sqrt_if_dn2_slot: &mut f64,
        var_sqrt_if_dn3_slot: &mut f64,
        var_sqrt_phi_vd_slot: &mut f64,
        var_sqrt_phi_vd_dn0_slot: &mut f64,
        var_sqrt_phi_vd_dn1_slot: &mut f64,
        var_sqrt_phi_vd_dn2_slot: &mut f64,
        var_sqrt_phi_vd_dn3_slot: &mut f64,
        var_sqrt_phi_vp0_slot: &mut f64,
        var_sqrt_phi_vp0_dn0_slot: &mut f64,
        var_sqrt_phi_vp0_dn1_slot: &mut f64,
        var_sqrt_phi_vp0_dn2_slot: &mut f64,
        var_sqrt_phi_vp0_dn3_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_dn0_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_dn1_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_dn2_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_dn3_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_dn0_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_dn1_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_dn2_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_dn3_slot: &mut f64,
        var_sqrt_vdss_deltav_slot: &mut f64,
        var_sqrt_vdss_deltav_dn0_slot: &mut f64,
        var_sqrt_vdss_deltav_dn1_slot: &mut f64,
        var_sqrt_vdss_deltav_dn2_slot: &mut f64,
        var_sqrt_vdss_deltav_dn3_slot: &mut f64,
        var_sqrt_vdssprime_deltav_slot: &mut f64,
        var_sqrt_vdssprime_deltav_dn0_slot: &mut f64,
        var_sqrt_vdssprime_deltav_dn1_slot: &mut f64,
        var_sqrt_vdssprime_deltav_dn2_slot: &mut f64,
        var_sqrt_vdssprime_deltav_dn3_slot: &mut f64,
        var_tmp1_slot: &mut f64,
        var_tmp1_dn0_slot: &mut f64,
        var_tmp1_dn1_slot: &mut f64,
        var_tmp1_dn2_slot: &mut f64,
        var_tmp1_dn3_slot: &mut f64,
        var_vds_slot: &mut f64,
        var_vds_dn0_slot: &mut f64,
        var_vds_dn2_slot: &mut f64,
        var_vds_dn3_slot: &mut f64,
        var_vdsprime_slot: &mut f64,
        var_vdsprime_dn0_slot: &mut f64,
        var_vdsprime_dn1_slot: &mut f64,
        var_vdsprime_dn2_slot: &mut f64,
        var_vdsprime_dn3_slot: &mut f64,
        var_vdss_slot: &mut f64,
        var_vdss_dn0_slot: &mut f64,
        var_vdss_dn1_slot: &mut f64,
        var_vdss_dn2_slot: &mut f64,
        var_vdss_dn3_slot: &mut f64,
        var_vdss_sqrt_slot: &mut f64,
        var_vdss_sqrt_dn0_slot: &mut f64,
        var_vdss_sqrt_dn1_slot: &mut f64,
        var_vdss_sqrt_dn2_slot: &mut f64,
        var_vdss_sqrt_dn3_slot: &mut f64,
        var_vdssprime_slot: &mut f64,
        var_vdssprime_dn0_slot: &mut f64,
        var_vdssprime_dn1_slot: &mut f64,
        var_vdssprime_dn2_slot: &mut f64,
        var_vdssprime_dn3_slot: &mut f64,
        var_vdssprime_sqrt_slot: &mut f64,
        var_vdssprime_sqrt_dn0_slot: &mut f64,
        var_vdssprime_sqrt_dn1_slot: &mut f64,
        var_vdssprime_sqrt_dn2_slot: &mut f64,
        var_vdssprime_sqrt_dn3_slot: &mut f64,
        var_vip_slot: &mut f64,
        var_vip_dn0_slot: &mut f64,
        var_vip_dn1_slot: &mut f64,
        var_vip_dn2_slot: &mut f64,
        var_vip_dn3_slot: &mut f64,
        var_vp_slot: &mut f64,
        var_vp0_slot: &mut f64,
        var_vp0_dn0_slot: &mut f64,
        var_vp0_dn1_slot: &mut f64,
        var_vp0_dn2_slot: &mut f64,
        var_vp0_dn3_slot: &mut f64,
        var_vp_dn0_slot: &mut f64,
        var_vp_dn1_slot: &mut f64,
        var_vp_dn2_slot: &mut f64,
        var_vp_dn3_slot: &mut f64,
        var_vt_vc_slot: &mut f64,
        var_weta_w_slot: &mut f64,
        var_yk_slot: &mut f64,
        var_yk_dn0_slot: &mut f64,
        var_yk_dn1_slot: &mut f64,
        var_yk_dn2_slot: &mut f64,
        var_yk_dn3_slot: &mut f64,
        var_z0_slot: &mut f64,
        var_z0_dn0_slot: &mut f64,
        var_z0_dn1_slot: &mut f64,
        var_z0_dn2_slot: &mut f64,
        var_z0_dn3_slot: &mut f64,
        var_zk_slot: &mut f64,
        var_zk_dn0_slot: &mut f64,
        var_zk_dn1_slot: &mut f64,
        var_zk_dn2_slot: &mut f64,
        var_zk_dn3_slot: &mut f64,
    ) {
        let mut var_big_sqrt_vp: f64 = *var_big_sqrt_vp_slot;
        let mut var_big_sqrt_vp0: f64 = *var_big_sqrt_vp0_slot;
        let mut var_big_sqrt_vp0_dn0: f64 = *var_big_sqrt_vp0_dn0_slot;
        let mut var_big_sqrt_vp0_dn1: f64 = *var_big_sqrt_vp0_dn1_slot;
        let mut var_big_sqrt_vp0_dn2: f64 = *var_big_sqrt_vp0_dn2_slot;
        let mut var_big_sqrt_vp0_dn3: f64 = *var_big_sqrt_vp0_dn3_slot;
        let mut var_big_sqrt_vp_dn0: f64 = *var_big_sqrt_vp_dn0_slot;
        let mut var_big_sqrt_vp_dn1: f64 = *var_big_sqrt_vp_dn1_slot;
        let mut var_big_sqrt_vp_dn2: f64 = *var_big_sqrt_vp_dn2_slot;
        let mut var_big_sqrt_vp_dn3: f64 = *var_big_sqrt_vp_dn3_slot;
        let mut var_deltav_2: f64 = *var_deltav_2_slot;
        let mut var_deltav_2_dn0: f64 = *var_deltav_2_dn0_slot;
        let mut var_deltav_2_dn1: f64 = *var_deltav_2_dn1_slot;
        let mut var_deltav_2_dn2: f64 = *var_deltav_2_dn2_slot;
        let mut var_deltav_2_dn3: f64 = *var_deltav_2_dn3_slot;
        let mut var_dif_dv: f64 = *var_dif_dv_slot;
        let mut var_dif_dv_dn0: f64 = *var_dif_dv_dn0_slot;
        let mut var_dif_dv_dn1: f64 = *var_dif_dv_dn1_slot;
        let mut var_dif_dv_dn2: f64 = *var_dif_dv_dn2_slot;
        let mut var_dif_dv_dn3: f64 = *var_dif_dv_dn3_slot;
        let mut var_gammaprime: f64 = *var_gammaprime_slot;
        let mut var_gammaprime_dn0: f64 = *var_gammaprime_dn0_slot;
        let mut var_gammaprime_dn1: f64 = *var_gammaprime_dn1_slot;
        let mut var_gammaprime_dn2: f64 = *var_gammaprime_dn2_slot;
        let mut var_gammaprime_dn3: f64 = *var_gammaprime_dn3_slot;
        let mut var_gammastar: f64 = *var_gammastar_slot;
        let mut var_gammastar_dn0: f64 = *var_gammastar_dn0_slot;
        let mut var_gammastar_dn1: f64 = *var_gammastar_dn1_slot;
        let mut var_gammastar_dn2: f64 = *var_gammastar_dn2_slot;
        let mut var_gammastar_dn3: f64 = *var_gammastar_dn3_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_if_: f64 = *var_if__slot;
        let mut var_if__dn0: f64 = *var_if__dn0_slot;
        let mut var_if__dn1: f64 = *var_if__dn1_slot;
        let mut var_if__dn2: f64 = *var_if__dn2_slot;
        let mut var_if__dn3: f64 = *var_if__dn3_slot;
        let mut var_leta_l: f64 = *var_leta_l_slot;
        let mut var_sqrt_gammastar: f64 = *var_sqrt_gammastar_slot;
        let mut var_sqrt_gammastar_dn0: f64 = *var_sqrt_gammastar_dn0_slot;
        let mut var_sqrt_gammastar_dn1: f64 = *var_sqrt_gammastar_dn1_slot;
        let mut var_sqrt_gammastar_dn2: f64 = *var_sqrt_gammastar_dn2_slot;
        let mut var_sqrt_gammastar_dn3: f64 = *var_sqrt_gammastar_dn3_slot;
        let mut var_sqrt_if: f64 = *var_sqrt_if_slot;
        let mut var_sqrt_if_dn0: f64 = *var_sqrt_if_dn0_slot;
        let mut var_sqrt_if_dn1: f64 = *var_sqrt_if_dn1_slot;
        let mut var_sqrt_if_dn2: f64 = *var_sqrt_if_dn2_slot;
        let mut var_sqrt_if_dn3: f64 = *var_sqrt_if_dn3_slot;
        let mut var_sqrt_phi_vd: f64 = *var_sqrt_phi_vd_slot;
        let mut var_sqrt_phi_vd_dn0: f64 = *var_sqrt_phi_vd_dn0_slot;
        let mut var_sqrt_phi_vd_dn1: f64 = *var_sqrt_phi_vd_dn1_slot;
        let mut var_sqrt_phi_vd_dn2: f64 = *var_sqrt_phi_vd_dn2_slot;
        let mut var_sqrt_phi_vd_dn3: f64 = *var_sqrt_phi_vd_dn3_slot;
        let mut var_sqrt_phi_vp0: f64 = *var_sqrt_phi_vp0_slot;
        let mut var_sqrt_phi_vp0_dn0: f64 = *var_sqrt_phi_vp0_dn0_slot;
        let mut var_sqrt_phi_vp0_dn1: f64 = *var_sqrt_phi_vp0_dn1_slot;
        let mut var_sqrt_phi_vp0_dn2: f64 = *var_sqrt_phi_vp0_dn2_slot;
        let mut var_sqrt_phi_vp0_dn3: f64 = *var_sqrt_phi_vp0_dn3_slot;
        let mut var_sqrt_vds_vdss_deltav: f64 = *var_sqrt_vds_vdss_deltav_slot;
        let mut var_sqrt_vds_vdss_deltav_dn0: f64 = *var_sqrt_vds_vdss_deltav_dn0_slot;
        let mut var_sqrt_vds_vdss_deltav_dn1: f64 = *var_sqrt_vds_vdss_deltav_dn1_slot;
        let mut var_sqrt_vds_vdss_deltav_dn2: f64 = *var_sqrt_vds_vdss_deltav_dn2_slot;
        let mut var_sqrt_vds_vdss_deltav_dn3: f64 = *var_sqrt_vds_vdss_deltav_dn3_slot;
        let mut var_sqrt_vds_vdssprime_deltav: f64 = *var_sqrt_vds_vdssprime_deltav_slot;
        let mut var_sqrt_vds_vdssprime_deltav_dn0: f64 = *var_sqrt_vds_vdssprime_deltav_dn0_slot;
        let mut var_sqrt_vds_vdssprime_deltav_dn1: f64 = *var_sqrt_vds_vdssprime_deltav_dn1_slot;
        let mut var_sqrt_vds_vdssprime_deltav_dn2: f64 = *var_sqrt_vds_vdssprime_deltav_dn2_slot;
        let mut var_sqrt_vds_vdssprime_deltav_dn3: f64 = *var_sqrt_vds_vdssprime_deltav_dn3_slot;
        let mut var_sqrt_vdss_deltav: f64 = *var_sqrt_vdss_deltav_slot;
        let mut var_sqrt_vdss_deltav_dn0: f64 = *var_sqrt_vdss_deltav_dn0_slot;
        let mut var_sqrt_vdss_deltav_dn1: f64 = *var_sqrt_vdss_deltav_dn1_slot;
        let mut var_sqrt_vdss_deltav_dn2: f64 = *var_sqrt_vdss_deltav_dn2_slot;
        let mut var_sqrt_vdss_deltav_dn3: f64 = *var_sqrt_vdss_deltav_dn3_slot;
        let mut var_sqrt_vdssprime_deltav: f64 = *var_sqrt_vdssprime_deltav_slot;
        let mut var_sqrt_vdssprime_deltav_dn0: f64 = *var_sqrt_vdssprime_deltav_dn0_slot;
        let mut var_sqrt_vdssprime_deltav_dn1: f64 = *var_sqrt_vdssprime_deltav_dn1_slot;
        let mut var_sqrt_vdssprime_deltav_dn2: f64 = *var_sqrt_vdssprime_deltav_dn2_slot;
        let mut var_sqrt_vdssprime_deltav_dn3: f64 = *var_sqrt_vdssprime_deltav_dn3_slot;
        let mut var_tmp1: f64 = *var_tmp1_slot;
        let mut var_tmp1_dn0: f64 = *var_tmp1_dn0_slot;
        let mut var_tmp1_dn1: f64 = *var_tmp1_dn1_slot;
        let mut var_tmp1_dn2: f64 = *var_tmp1_dn2_slot;
        let mut var_tmp1_dn3: f64 = *var_tmp1_dn3_slot;
        let mut var_vds: f64 = *var_vds_slot;
        let mut var_vds_dn0: f64 = *var_vds_dn0_slot;
        let mut var_vds_dn2: f64 = *var_vds_dn2_slot;
        let mut var_vds_dn3: f64 = *var_vds_dn3_slot;
        let mut var_vdsprime: f64 = *var_vdsprime_slot;
        let mut var_vdsprime_dn0: f64 = *var_vdsprime_dn0_slot;
        let mut var_vdsprime_dn1: f64 = *var_vdsprime_dn1_slot;
        let mut var_vdsprime_dn2: f64 = *var_vdsprime_dn2_slot;
        let mut var_vdsprime_dn3: f64 = *var_vdsprime_dn3_slot;
        let mut var_vdss: f64 = *var_vdss_slot;
        let mut var_vdss_dn0: f64 = *var_vdss_dn0_slot;
        let mut var_vdss_dn1: f64 = *var_vdss_dn1_slot;
        let mut var_vdss_dn2: f64 = *var_vdss_dn2_slot;
        let mut var_vdss_dn3: f64 = *var_vdss_dn3_slot;
        let mut var_vdss_sqrt: f64 = *var_vdss_sqrt_slot;
        let mut var_vdss_sqrt_dn0: f64 = *var_vdss_sqrt_dn0_slot;
        let mut var_vdss_sqrt_dn1: f64 = *var_vdss_sqrt_dn1_slot;
        let mut var_vdss_sqrt_dn2: f64 = *var_vdss_sqrt_dn2_slot;
        let mut var_vdss_sqrt_dn3: f64 = *var_vdss_sqrt_dn3_slot;
        let mut var_vdssprime: f64 = *var_vdssprime_slot;
        let mut var_vdssprime_dn0: f64 = *var_vdssprime_dn0_slot;
        let mut var_vdssprime_dn1: f64 = *var_vdssprime_dn1_slot;
        let mut var_vdssprime_dn2: f64 = *var_vdssprime_dn2_slot;
        let mut var_vdssprime_dn3: f64 = *var_vdssprime_dn3_slot;
        let mut var_vdssprime_sqrt: f64 = *var_vdssprime_sqrt_slot;
        let mut var_vdssprime_sqrt_dn0: f64 = *var_vdssprime_sqrt_dn0_slot;
        let mut var_vdssprime_sqrt_dn1: f64 = *var_vdssprime_sqrt_dn1_slot;
        let mut var_vdssprime_sqrt_dn2: f64 = *var_vdssprime_sqrt_dn2_slot;
        let mut var_vdssprime_sqrt_dn3: f64 = *var_vdssprime_sqrt_dn3_slot;
        let mut var_vip: f64 = *var_vip_slot;
        let mut var_vip_dn0: f64 = *var_vip_dn0_slot;
        let mut var_vip_dn1: f64 = *var_vip_dn1_slot;
        let mut var_vip_dn2: f64 = *var_vip_dn2_slot;
        let mut var_vip_dn3: f64 = *var_vip_dn3_slot;
        let mut var_vp: f64 = *var_vp_slot;
        let mut var_vp0: f64 = *var_vp0_slot;
        let mut var_vp0_dn0: f64 = *var_vp0_dn0_slot;
        let mut var_vp0_dn1: f64 = *var_vp0_dn1_slot;
        let mut var_vp0_dn2: f64 = *var_vp0_dn2_slot;
        let mut var_vp0_dn3: f64 = *var_vp0_dn3_slot;
        let mut var_vp_dn0: f64 = *var_vp_dn0_slot;
        let mut var_vp_dn1: f64 = *var_vp_dn1_slot;
        let mut var_vp_dn2: f64 = *var_vp_dn2_slot;
        let mut var_vp_dn3: f64 = *var_vp_dn3_slot;
        let mut var_vt_vc: f64 = *var_vt_vc_slot;
        let mut var_weta_w: f64 = *var_weta_w_slot;
        let mut var_yk: f64 = *var_yk_slot;
        let mut var_yk_dn0: f64 = *var_yk_dn0_slot;
        let mut var_yk_dn1: f64 = *var_yk_dn1_slot;
        let mut var_yk_dn2: f64 = *var_yk_dn2_slot;
        let mut var_yk_dn3: f64 = *var_yk_dn3_slot;
        let mut var_z0: f64 = *var_z0_slot;
        let mut var_z0_dn0: f64 = *var_z0_dn0_slot;
        let mut var_z0_dn1: f64 = *var_z0_dn1_slot;
        let mut var_z0_dn2: f64 = *var_z0_dn2_slot;
        let mut var_z0_dn3: f64 = *var_z0_dn3_slot;
        let mut var_zk: f64 = *var_zk_slot;
        let mut var_zk_dn0: f64 = *var_zk_dn0_slot;
        let mut var_zk_dn1: f64 = *var_zk_dn1_slot;
        let mut var_zk_dn2: f64 = *var_zk_dn2_slot;
        let mut var_zk_dn3: f64 = *var_zk_dn3_slot;

        let assign790_e616: f64 = (var_phi_vd + var_sqrt_phi_vd_vt);
        let assign790_e617: f64 = (0.5 * assign790_e616);
        let assign790_e618: f64 = (assign790_e617).sqrt();
        var_sqrt_phi_vd = assign790_e618;
        var_sqrt_phi_vd_dn0 = ((0.5 * (var_phi_vd_dn0 + var_sqrt_phi_vd_vt_dn0)) / (2.0 * assign790_e618));
        var_sqrt_phi_vd_dn1 = ((0.5 * (var_phi_vd_dn1 + var_sqrt_phi_vd_vt_dn1)) / (2.0 * assign790_e618));
        var_sqrt_phi_vd_dn2 = ((0.5 * (var_phi_vd_dn2 + var_sqrt_phi_vd_vt_dn2)) / (2.0 * assign790_e618));
        var_sqrt_phi_vd_dn3 = ((0.5 * (var_phi_vd_dn3 + var_sqrt_phi_vd_vt_dn3)) / (2.0 * assign790_e618));

        let assign800_e621: f64 = (var_eps_cox_w * p.p7);
        let assign800_e623: f64 = (assign800_e621 / var_weff);
        var_weta_w = assign800_e623;

        let assign810_e626: f64 = (var_eps_cox_l * p.p8);
        let assign810_e628: f64 = (assign810_e626 / var_leff);
        var_leta_l = assign810_e628;

        let assign820_e632: f64 = (0.25 * var_gamma_s);
        let assign820_e634: f64 = (assign820_e632 * var_gamma_s);
        let assign820_e635: f64 = (var_vgprime + assign820_e634);
        let assign820_e636: f64 = (assign820_e635).sqrt();
        var_big_sqrt_vp0 = assign820_e636;
        var_big_sqrt_vp0_dn0 = (var_vgprime_dn0 / (2.0 * assign820_e636));
        var_big_sqrt_vp0_dn1 = (var_vgprime_dn1 / (2.0 * assign820_e636));
        var_big_sqrt_vp0_dn2 = (var_vgprime_dn2 / (2.0 * assign820_e636));
        var_big_sqrt_vp0_dn3 = (var_vgprime_dn3 / (2.0 * assign820_e636));

        let assign830_e639: f64 = (var_vgprime - var_phi_t);
        let assign830_e644: f64 = (0.5 * var_gamma_s);
        let assign830_e645: f64 = (var_big_sqrt_vp0 - assign830_e644);
        let assign830_e646: f64 = (var_gamma_s * assign830_e645);
        let assign830_e647: f64 = (assign830_e639 - assign830_e646);
        var_vp0 = assign830_e647;
        var_vp0_dn0 = ((var_vgprime_dn0 - var_phi_t_dn0) - (var_gamma_s * var_big_sqrt_vp0_dn0));
        var_vp0_dn1 = ((var_vgprime_dn1 - var_phi_t_dn1) - (var_gamma_s * var_big_sqrt_vp0_dn1));
        var_vp0_dn2 = ((var_vgprime_dn2 - var_phi_t_dn2) - (var_gamma_s * var_big_sqrt_vp0_dn2));
        var_vp0_dn3 = ((var_vgprime_dn3 - var_phi_t_dn3) - (var_gamma_s * var_big_sqrt_vp0_dn3));

        let assign840_e650: f64 = (var_vp0 + var_phi_t);
        let assign840_e652: f64 = (assign840_e650 + var_vt_01);
        let assign840_e653: f64 = (assign840_e652).sqrt();
        var_sqrt_phi_vp0 = assign840_e653;
        var_sqrt_phi_vp0_dn0 = ((var_vp0_dn0 + var_phi_t_dn0) / (2.0 * assign840_e653));
        var_sqrt_phi_vp0_dn1 = ((var_vp0_dn1 + var_phi_t_dn1) / (2.0 * assign840_e653));
        var_sqrt_phi_vp0_dn2 = ((var_vp0_dn2 + var_phi_t_dn2) / (2.0 * assign840_e653));
        var_sqrt_phi_vp0_dn3 = ((var_vp0_dn3 + var_phi_t_dn3) / (2.0 * assign840_e653));

        let assign850_e658: f64 = (var_sqrt_phi_vs + var_sqrt_phi_vd);
        let assign850_e659: f64 = (var_leta_l * assign850_e658);
        let assign850_e660: f64 = (var_gamma_s - assign850_e659);
        let assign850_e663: f64 = (var_weta_w * var_sqrt_phi_vp0);
        let assign850_e664: f64 = (assign850_e660 + assign850_e663);
        var_gammastar = assign850_e664;
        var_gammastar_dn0 = ((-(var_leta_l * (var_sqrt_phi_vs_dn0 + var_sqrt_phi_vd_dn0))) + (var_weta_w * var_sqrt_phi_vp0_dn0));
        var_gammastar_dn1 = ((-(var_leta_l * (var_sqrt_phi_vs_dn1 + var_sqrt_phi_vd_dn1))) + (var_weta_w * var_sqrt_phi_vp0_dn1));
        var_gammastar_dn2 = ((-(var_leta_l * (var_sqrt_phi_vs_dn2 + var_sqrt_phi_vd_dn2))) + (var_weta_w * var_sqrt_phi_vp0_dn2));
        var_gammastar_dn3 = ((-(var_leta_l * (var_sqrt_phi_vs_dn3 + var_sqrt_phi_vd_dn3))) + (var_weta_w * var_sqrt_phi_vp0_dn3));

        let assign860_e667: f64 = (var_gammastar * var_gammastar);
        let assign860_e669: f64 = (assign860_e667 + var_vt_01);
        let assign860_e670: f64 = (assign860_e669).sqrt();
        var_sqrt_gammastar = assign860_e670;
        var_sqrt_gammastar_dn0 = (((var_gammastar_dn0 * var_gammastar) + (var_gammastar * var_gammastar_dn0)) / (2.0 * assign860_e670));
        var_sqrt_gammastar_dn1 = (((var_gammastar_dn1 * var_gammastar) + (var_gammastar * var_gammastar_dn1)) / (2.0 * assign860_e670));
        var_sqrt_gammastar_dn2 = (((var_gammastar_dn2 * var_gammastar) + (var_gammastar * var_gammastar_dn2)) / (2.0 * assign860_e670));
        var_sqrt_gammastar_dn3 = (((var_gammastar_dn3 * var_gammastar) + (var_gammastar * var_gammastar_dn3)) / (2.0 * assign860_e670));

        let assign870_e674: f64 = (var_gammastar + var_sqrt_gammastar);
        let assign870_e675: f64 = (0.5 * assign870_e674);
        var_gammaprime = assign870_e675;
        var_gammaprime_dn0 = (0.5 * (var_gammastar_dn0 + var_sqrt_gammastar_dn0));
        var_gammaprime_dn1 = (0.5 * (var_gammastar_dn1 + var_sqrt_gammastar_dn1));
        var_gammaprime_dn2 = (0.5 * (var_gammastar_dn2 + var_sqrt_gammastar_dn2));
        var_gammaprime_dn3 = (0.5 * (var_gammastar_dn3 + var_sqrt_gammastar_dn3));

        let assign880_e679: f64 = (0.25 * var_gammaprime);
        let assign880_e681: f64 = (assign880_e679 * var_gammaprime);
        let assign880_e682: f64 = (var_vgprime + assign880_e681);
        let assign880_e683: f64 = (assign880_e682).sqrt();
        var_big_sqrt_vp = assign880_e683;
        var_big_sqrt_vp_dn0 = ((var_vgprime_dn0 + (((0.25 * var_gammaprime_dn0) * var_gammaprime) + (assign880_e679 * var_gammaprime_dn0))) / (2.0 * assign880_e683));
        var_big_sqrt_vp_dn1 = ((var_vgprime_dn1 + (((0.25 * var_gammaprime_dn1) * var_gammaprime) + (assign880_e679 * var_gammaprime_dn1))) / (2.0 * assign880_e683));
        var_big_sqrt_vp_dn2 = ((var_vgprime_dn2 + (((0.25 * var_gammaprime_dn2) * var_gammaprime) + (assign880_e679 * var_gammaprime_dn2))) / (2.0 * assign880_e683));
        var_big_sqrt_vp_dn3 = ((var_vgprime_dn3 + (((0.25 * var_gammaprime_dn3) * var_gammaprime) + (assign880_e679 * var_gammaprime_dn3))) / (2.0 * assign880_e683));

        let assign890_e686: f64 = (var_vgprime - var_phi_t);
        let assign890_e691: f64 = (0.5 * var_gammaprime);
        let assign890_e692: f64 = (var_big_sqrt_vp - assign890_e691);
        let assign890_e693: f64 = (var_gammaprime * assign890_e692);
        let assign890_e694: f64 = (assign890_e686 - assign890_e693);
        var_vp = assign890_e694;
        var_vp_dn0 = ((var_vgprime_dn0 - var_phi_t_dn0) - ((var_gammaprime_dn0 * assign890_e692) + (var_gammaprime * (var_big_sqrt_vp_dn0 - (0.5 * var_gammaprime_dn0)))));
        var_vp_dn1 = ((var_vgprime_dn1 - var_phi_t_dn1) - ((var_gammaprime_dn1 * assign890_e692) + (var_gammaprime * (var_big_sqrt_vp_dn1 - (0.5 * var_gammaprime_dn1)))));
        var_vp_dn2 = ((var_vgprime_dn2 - var_phi_t_dn2) - ((var_gammaprime_dn2 * assign890_e692) + (var_gammaprime * (var_big_sqrt_vp_dn2 - (0.5 * var_gammaprime_dn2)))));
        var_vp_dn3 = ((var_vgprime_dn3 - var_phi_t_dn3) - ((var_gammaprime_dn3 * assign890_e692) + (var_gammaprime * (var_big_sqrt_vp_dn3 - (0.5 * var_gammaprime_dn3)))));

        let assign900_e697: f64 = (var_vp - var_vs);
        let assign900_e699: f64 = (assign900_e697 * var_inv_vt);
        var_tmp1 = assign900_e699;
        var_tmp1_dn0 = ((var_vp_dn0 - var_vs_dn0) * var_inv_vt);
        var_tmp1_dn1 = (var_vp_dn1 * var_inv_vt);
        var_tmp1_dn2 = ((var_vp_dn2 - var_vs_dn2) * var_inv_vt);
        var_tmp1_dn3 = ((var_vp_dn3 - var_vs_dn3) * var_inv_vt);

        let assign910_e702: f64 = (-0.35);
        let assign910_e703: f64 = if var_tmp1 > assign910_e702 { 1.0 } else { 0.0 };
        var_guard7 = assign910_e703;

        let (assign920_e716, assign920_e716_d_n0, assign920_e716_d_n1, assign920_e716_d_n2, assign920_e716_d_n3,) = {
    if (var_guard7 != 0.0) {
        let assign920_e708: f64 = (1.3 + var_tmp1);
        let assign920_e711: f64 = (var_tmp1 + 1.6);
        let assign920_e712: f64 = (assign920_e711).ln();
        let assign920_e713: f64 = (assign920_e708 - assign920_e712);
        let assign920_e714: f64 = (2.0 / assign920_e713);
        (assign920_e714, (-((2.0 * (var_tmp1_dn0 - (var_tmp1_dn0 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (var_tmp1_dn1 - (var_tmp1_dn1 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (var_tmp1_dn2 - (var_tmp1_dn2 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (var_tmp1_dn3 - (var_tmp1_dn3 / assign920_e711))) / (assign920_e713 * assign920_e713))),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign920_e716;
        var_z0_dn0 = assign920_e716_d_n0;
        var_z0_dn1 = assign920_e716_d_n1;
        var_z0_dn2 = assign920_e716_d_n2;
        var_z0_dn3 = assign920_e716_d_n3;

        let (assign930_e729, assign930_e729_d_n0, assign930_e729_d_n1, assign930_e729_d_n2, assign930_e729_d_n3,) = {
    if (var_guard7 != 0.0) {
        let assign930_e720: f64 = (2.0 + var_z0);
        let assign930_e723: f64 = (1.0 + var_tmp1);
        let assign930_e725: f64 = (var_z0).ln();
        let assign930_e726: f64 = (assign930_e723 + assign930_e725);
        let assign930_e727: f64 = (assign930_e720 / assign930_e726);
        (assign930_e727, (((var_z0_dn0 * assign930_e726) - (assign930_e720 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign930_e726 * assign930_e726)), (((var_z0_dn1 * assign930_e726) - (assign930_e720 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign930_e726 * assign930_e726)), (((var_z0_dn2 * assign930_e726) - (assign930_e720 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign930_e726 * assign930_e726)), (((var_z0_dn3 * assign930_e726) - (assign930_e720 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign930_e726 * assign930_e726)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign930_e729;
        var_zk_dn0 = assign930_e729_d_n0;
        var_zk_dn1 = assign930_e729_d_n1;
        var_zk_dn2 = assign930_e729_d_n2;
        var_zk_dn3 = assign930_e729_d_n3;

        let (assign940_e742, assign940_e742_d_n0, assign940_e742_d_n1, assign940_e742_d_n2, assign940_e742_d_n3,) = {
    if (var_guard7 != 0.0) {
        let assign940_e733: f64 = (1.0 + var_tmp1);
        let assign940_e735: f64 = (var_zk).ln();
        let assign940_e736: f64 = (assign940_e733 + assign940_e735);
        let assign940_e739: f64 = (2.0 + var_zk);
        let assign940_e740: f64 = (assign940_e736 / assign940_e739);
        (assign940_e740, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign940_e739) - (assign940_e736 * var_zk_dn0)) / (assign940_e739 * assign940_e739)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign940_e739) - (assign940_e736 * var_zk_dn1)) / (assign940_e739 * assign940_e739)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign940_e739) - (assign940_e736 * var_zk_dn2)) / (assign940_e739 * assign940_e739)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign940_e739) - (assign940_e736 * var_zk_dn3)) / (assign940_e739 * assign940_e739)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign940_e742;
        var_yk_dn0 = assign940_e742_d_n0;
        var_yk_dn1 = assign940_e742_d_n1;
        var_yk_dn2 = assign940_e742_d_n2;
        var_yk_dn3 = assign940_e742_d_n3;

        let assign950_e745: f64 = (-15.0);
        let assign950_e746: f64 = if var_tmp1 > assign950_e745 { 1.0 } else { 0.0 };
        var_guard8 = assign950_e746;

        let (assign960_e757, assign960_e757_d_n0, assign960_e757_d_n1, assign960_e757_d_n2, assign960_e757_d_n3,) = {
    if ((var_guard7 == 0.0) && (var_guard8 != 0.0)) {
        let assign960_e753: f64 = (-var_tmp1);
        let assign960_e754: f64 = (assign960_e753).exp();
        let assign960_e755: f64 = (1.55 + assign960_e754);
        (assign960_e755, (assign960_e754 * (-var_tmp1_dn0)), (assign960_e754 * (-var_tmp1_dn1)), (assign960_e754 * (-var_tmp1_dn2)), (assign960_e754 * (-var_tmp1_dn3)),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign960_e757;
        var_z0_dn0 = assign960_e757_d_n0;
        var_z0_dn1 = assign960_e757_d_n1;
        var_z0_dn2 = assign960_e757_d_n2;
        var_z0_dn3 = assign960_e757_d_n3;

        let (assign970_e773, assign970_e773_d_n0, assign970_e773_d_n1, assign970_e773_d_n2, assign970_e773_d_n3,) = {
    if ((var_guard7 == 0.0) && (var_guard8 != 0.0)) {
        let assign970_e764: f64 = (2.0 + var_z0);
        let assign970_e767: f64 = (1.0 + var_tmp1);
        let assign970_e769: f64 = (var_z0).ln();
        let assign970_e770: f64 = (assign970_e767 + assign970_e769);
        let assign970_e771: f64 = (assign970_e764 / assign970_e770);
        (assign970_e771, (((var_z0_dn0 * assign970_e770) - (assign970_e764 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign970_e770 * assign970_e770)), (((var_z0_dn1 * assign970_e770) - (assign970_e764 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign970_e770 * assign970_e770)), (((var_z0_dn2 * assign970_e770) - (assign970_e764 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign970_e770 * assign970_e770)), (((var_z0_dn3 * assign970_e770) - (assign970_e764 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign970_e770 * assign970_e770)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign970_e773;
        var_zk_dn0 = assign970_e773_d_n0;
        var_zk_dn1 = assign970_e773_d_n1;
        var_zk_dn2 = assign970_e773_d_n2;
        var_zk_dn3 = assign970_e773_d_n3;

        let (assign980_e789, assign980_e789_d_n0, assign980_e789_d_n1, assign980_e789_d_n2, assign980_e789_d_n3,) = {
    if ((var_guard7 == 0.0) && (var_guard8 != 0.0)) {
        let assign980_e780: f64 = (1.0 + var_tmp1);
        let assign980_e782: f64 = (var_zk).ln();
        let assign980_e783: f64 = (assign980_e780 + assign980_e782);
        let assign980_e786: f64 = (2.0 + var_zk);
        let assign980_e787: f64 = (assign980_e783 / assign980_e786);
        (assign980_e787, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign980_e786) - (assign980_e783 * var_zk_dn0)) / (assign980_e786 * assign980_e786)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign980_e786) - (assign980_e783 * var_zk_dn1)) / (assign980_e786 * assign980_e786)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign980_e786) - (assign980_e783 * var_zk_dn2)) / (assign980_e786 * assign980_e786)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign980_e786) - (assign980_e783 * var_zk_dn3)) / (assign980_e786 * assign980_e786)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign980_e789;
        var_yk_dn0 = assign980_e789_d_n0;
        var_yk_dn1 = assign980_e789_d_n1;
        var_yk_dn2 = assign980_e789_d_n2;
        var_yk_dn3 = assign980_e789_d_n3;

        let assign990_e792: f64 = (-23.0);
        let assign990_e793: f64 = if var_tmp1 > assign990_e792 { 1.0 } else { 0.0 };
        var_guard9 = assign990_e793;

        let (assign1000_e809, assign1000_e809_d_n0, assign1000_e809_d_n1, assign1000_e809_d_n2, assign1000_e809_d_n3,) = {
    if (((var_guard7 == 0.0) && (var_guard8 == 0.0)) && (var_guard9 != 0.0)) {
        let assign1000_e804: f64 = (-var_tmp1);
        let assign1000_e805: f64 = (assign1000_e804).exp();
        let assign1000_e806: f64 = (2.0 + assign1000_e805);
        let assign1000_e807: f64 = (1.0 / assign1000_e806);
        (assign1000_e807, (-((assign1000_e805 * (-var_tmp1_dn0)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-var_tmp1_dn1)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-var_tmp1_dn2)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-var_tmp1_dn3)) / (assign1000_e806 * assign1000_e806))),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1000_e809;
        var_yk_dn0 = assign1000_e809_d_n0;
        var_yk_dn1 = assign1000_e809_d_n1;
        var_yk_dn2 = assign1000_e809_d_n2;
        var_yk_dn3 = assign1000_e809_d_n3;

        let (assign1010_e823, assign1010_e823_d_n0, assign1010_e823_d_n1, assign1010_e823_d_n2, assign1010_e823_d_n3,) = {
    if (((var_guard7 == 0.0) && (var_guard8 == 0.0)) && (var_guard9 == 0.0)) {
        let assign1010_e819: f64 = (var_tmp1).exp();
        let assign1010_e821: f64 = (assign1010_e819 + 1e-64);
        (assign1010_e821, (assign1010_e819 * var_tmp1_dn0), (assign1010_e819 * var_tmp1_dn1), (assign1010_e819 * var_tmp1_dn2), (assign1010_e819 * var_tmp1_dn3),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1010_e823;
        var_yk_dn0 = assign1010_e823_d_n0;
        var_yk_dn1 = assign1010_e823_d_n1;
        var_yk_dn2 = assign1010_e823_d_n2;
        var_yk_dn3 = assign1010_e823_d_n3;

        let assign1020_e827: f64 = (1.0 + var_yk);
        let assign1020_e828: f64 = (var_yk * assign1020_e827);
        var_if_ = assign1020_e828;
        var_if__dn0 = ((var_yk_dn0 * assign1020_e827) + (var_yk * var_yk_dn0));
        var_if__dn1 = ((var_yk_dn1 * assign1020_e827) + (var_yk * var_yk_dn1));
        var_if__dn2 = ((var_yk_dn2 * assign1020_e827) + (var_yk * var_yk_dn2));
        var_if__dn3 = ((var_yk_dn3 * assign1020_e827) + (var_yk * var_yk_dn3));

        let assign1030_e830: f64 = (var_if_).sqrt();
        var_sqrt_if = assign1030_e830;
        var_sqrt_if_dn0 = (var_if__dn0 / (2.0 * assign1030_e830));
        var_sqrt_if_dn1 = (var_if__dn1 / (2.0 * assign1030_e830));
        var_sqrt_if_dn2 = (var_if__dn2 / (2.0 * assign1030_e830));
        var_sqrt_if_dn3 = (var_if__dn3 / (2.0 * assign1030_e830));

        var_dif_dv = var_yk;
        var_dif_dv_dn0 = var_yk_dn0;
        var_dif_dv_dn1 = var_yk_dn1;
        var_dif_dv_dn2 = var_yk_dn2;
        var_dif_dv_dn3 = var_yk_dn3;

        let assign1050_e834: f64 = (var_vt / var_vc);
        var_vt_vc = assign1050_e834;

        let assign1060_e838: f64 = (var_sqrt_if * var_vt_vc);
        let assign1060_e839: f64 = (0.25 + assign1060_e838);
        let assign1060_e840: f64 = (assign1060_e839).sqrt();
        var_vdss_sqrt = assign1060_e840;
        var_vdss_sqrt_dn0 = ((var_sqrt_if_dn0 * var_vt_vc) / (2.0 * assign1060_e840));
        var_vdss_sqrt_dn1 = ((var_sqrt_if_dn1 * var_vt_vc) / (2.0 * assign1060_e840));
        var_vdss_sqrt_dn2 = ((var_sqrt_if_dn2 * var_vt_vc) / (2.0 * assign1060_e840));
        var_vdss_sqrt_dn3 = ((var_sqrt_if_dn3 * var_vt_vc) / (2.0 * assign1060_e840));

        let assign1070_e844: f64 = (var_vdss_sqrt - 0.5);
        let assign1070_e845: f64 = (var_vc * assign1070_e844);
        var_vdss = assign1070_e845;
        var_vdss_dn0 = (var_vc * var_vdss_sqrt_dn0);
        var_vdss_dn1 = (var_vc * var_vdss_sqrt_dn1);
        var_vdss_dn2 = (var_vc * var_vdss_sqrt_dn2);
        var_vdss_dn3 = (var_vc * var_vdss_sqrt_dn3);

        let assign1080_e849: f64 = (var_vd - var_vs);
        let assign1080_e850: f64 = (0.5 * assign1080_e849);
        var_vds = assign1080_e850;
        var_vds_dn0 = (0.5 * (var_vd_dn0 - var_vs_dn0));
        var_vds_dn2 = (0.5 * (var_vd_dn2 - var_vs_dn2));
        var_vds_dn3 = (0.5 * (var_vd_dn3 - var_vs_dn3));

        let assign1090_e856: f64 = (var_vdss * var_inv_vt);
        let assign1090_e857: f64 = (var_sqrt_if - assign1090_e856);
        let assign1090_e858: f64 = (p.p25 * assign1090_e857);
        let assign1090_e860: f64 = (assign1090_e858 + 0.015625);
        let assign1090_e861: f64 = (var_vt_vt_16 * assign1090_e860);
        var_deltav_2 = assign1090_e861;
        var_deltav_2_dn0 = (var_vt_vt_16 * (p.p25 * (var_sqrt_if_dn0 - (var_vdss_dn0 * var_inv_vt))));
        var_deltav_2_dn1 = (var_vt_vt_16 * (p.p25 * (var_sqrt_if_dn1 - (var_vdss_dn1 * var_inv_vt))));
        var_deltav_2_dn2 = (var_vt_vt_16 * (p.p25 * (var_sqrt_if_dn2 - (var_vdss_dn2 * var_inv_vt))));
        var_deltav_2_dn3 = (var_vt_vt_16 * (p.p25 * (var_sqrt_if_dn3 - (var_vdss_dn3 * var_inv_vt))));

        let assign1100_e864: f64 = (var_vdss * var_vdss);
        let assign1100_e866: f64 = (assign1100_e864 + var_deltav_2);
        let assign1100_e867: f64 = (assign1100_e866).sqrt();
        var_sqrt_vdss_deltav = assign1100_e867;
        var_sqrt_vdss_deltav_dn0 = ((((var_vdss_dn0 * var_vdss) + (var_vdss * var_vdss_dn0)) + var_deltav_2_dn0) / (2.0 * assign1100_e867));
        var_sqrt_vdss_deltav_dn1 = ((((var_vdss_dn1 * var_vdss) + (var_vdss * var_vdss_dn1)) + var_deltav_2_dn1) / (2.0 * assign1100_e867));
        var_sqrt_vdss_deltav_dn2 = ((((var_vdss_dn2 * var_vdss) + (var_vdss * var_vdss_dn2)) + var_deltav_2_dn2) / (2.0 * assign1100_e867));
        var_sqrt_vdss_deltav_dn3 = ((((var_vdss_dn3 * var_vdss) + (var_vdss * var_vdss_dn3)) + var_deltav_2_dn3) / (2.0 * assign1100_e867));

        let assign1110_e870: f64 = (var_vds - var_vdss);
        let assign1110_e873: f64 = (var_vds - var_vdss);
        let assign1110_e874: f64 = (assign1110_e870 * assign1110_e873);
        let assign1110_e876: f64 = (assign1110_e874 + var_deltav_2);
        let assign1110_e877: f64 = (assign1110_e876).sqrt();
        var_sqrt_vds_vdss_deltav = assign1110_e877;
        var_sqrt_vds_vdss_deltav_dn0 = (((((var_vds_dn0 - var_vdss_dn0) * assign1110_e873) + (assign1110_e870 * (var_vds_dn0 - var_vdss_dn0))) + var_deltav_2_dn0) / (2.0 * assign1110_e877));
        var_sqrt_vds_vdss_deltav_dn1 = (((((-var_vdss_dn1) * assign1110_e873) + (assign1110_e870 * (-var_vdss_dn1))) + var_deltav_2_dn1) / (2.0 * assign1110_e877));
        var_sqrt_vds_vdss_deltav_dn2 = (((((var_vds_dn2 - var_vdss_dn2) * assign1110_e873) + (assign1110_e870 * (var_vds_dn2 - var_vdss_dn2))) + var_deltav_2_dn2) / (2.0 * assign1110_e877));
        var_sqrt_vds_vdss_deltav_dn3 = (((((var_vds_dn3 - var_vdss_dn3) * assign1110_e873) + (assign1110_e870 * (var_vds_dn3 - var_vdss_dn3))) + var_deltav_2_dn3) / (2.0 * assign1110_e877));

        let assign1120_e880: f64 = (var_sqrt_vdss_deltav - var_sqrt_vds_vdss_deltav);
        var_vip = assign1120_e880;
        var_vip_dn0 = (var_sqrt_vdss_deltav_dn0 - var_sqrt_vds_vdss_deltav_dn0);
        var_vip_dn1 = (var_sqrt_vdss_deltav_dn1 - var_sqrt_vds_vdss_deltav_dn1);
        var_vip_dn2 = (var_sqrt_vdss_deltav_dn2 - var_sqrt_vds_vdss_deltav_dn2);
        var_vip_dn3 = (var_sqrt_vdss_deltav_dn3 - var_sqrt_vds_vdss_deltav_dn3);

        let assign1130_e885: f64 = (var_if_).ln();
        let assign1130_e886: f64 = (0.75 * assign1130_e885);
        let assign1130_e887: f64 = (var_sqrt_if - assign1130_e886);
        let assign1130_e889: f64 = (assign1130_e887 * var_vt_vc);
        let assign1130_e890: f64 = (0.25 + assign1130_e889);
        let assign1130_e891: f64 = (assign1130_e890).sqrt();
        var_vdssprime_sqrt = assign1130_e891;
        var_vdssprime_sqrt_dn0 = (((var_sqrt_if_dn0 - (0.75 * (var_if__dn0 / var_if_))) * var_vt_vc) / (2.0 * assign1130_e891));
        var_vdssprime_sqrt_dn1 = (((var_sqrt_if_dn1 - (0.75 * (var_if__dn1 / var_if_))) * var_vt_vc) / (2.0 * assign1130_e891));
        var_vdssprime_sqrt_dn2 = (((var_sqrt_if_dn2 - (0.75 * (var_if__dn2 / var_if_))) * var_vt_vc) / (2.0 * assign1130_e891));
        var_vdssprime_sqrt_dn3 = (((var_sqrt_if_dn3 - (0.75 * (var_if__dn3 / var_if_))) * var_vt_vc) / (2.0 * assign1130_e891));

        let assign1140_e895: f64 = (var_vdssprime_sqrt - 0.5);
        let assign1140_e896: f64 = (var_vc * assign1140_e895);
        let assign1140_e898: f64 = (assign1140_e896 + var_log_vc_vt);
        var_vdssprime = assign1140_e898;
        var_vdssprime_dn0 = (var_vc * var_vdssprime_sqrt_dn0);
        var_vdssprime_dn1 = (var_vc * var_vdssprime_sqrt_dn1);
        var_vdssprime_dn2 = (var_vc * var_vdssprime_sqrt_dn2);
        var_vdssprime_dn3 = (var_vc * var_vdssprime_sqrt_dn3);

        let assign1150_e901: f64 = (var_vds - var_vdssprime);
        var_vdsprime = assign1150_e901;
        var_vdsprime_dn0 = (var_vds_dn0 - var_vdssprime_dn0);
        var_vdsprime_dn1 = (-var_vdssprime_dn1);
        var_vdsprime_dn2 = (var_vds_dn2 - var_vdssprime_dn2);
        var_vdsprime_dn3 = (var_vds_dn3 - var_vdssprime_dn3);

        let assign1160_e904: f64 = (var_vdssprime * var_vdssprime);
        let assign1160_e906: f64 = (assign1160_e904 + var_deltav_2);
        let assign1160_e907: f64 = (assign1160_e906).sqrt();
        var_sqrt_vdssprime_deltav = assign1160_e907;
        var_sqrt_vdssprime_deltav_dn0 = ((((var_vdssprime_dn0 * var_vdssprime) + (var_vdssprime * var_vdssprime_dn0)) + var_deltav_2_dn0) / (2.0 * assign1160_e907));
        var_sqrt_vdssprime_deltav_dn1 = ((((var_vdssprime_dn1 * var_vdssprime) + (var_vdssprime * var_vdssprime_dn1)) + var_deltav_2_dn1) / (2.0 * assign1160_e907));
        var_sqrt_vdssprime_deltav_dn2 = ((((var_vdssprime_dn2 * var_vdssprime) + (var_vdssprime * var_vdssprime_dn2)) + var_deltav_2_dn2) / (2.0 * assign1160_e907));
        var_sqrt_vdssprime_deltav_dn3 = ((((var_vdssprime_dn3 * var_vdssprime) + (var_vdssprime * var_vdssprime_dn3)) + var_deltav_2_dn3) / (2.0 * assign1160_e907));

        let assign1170_e910: f64 = (var_vdsprime * var_vdsprime);
        let assign1170_e912: f64 = (assign1170_e910 + var_deltav_2);
        let assign1170_e913: f64 = (assign1170_e912).sqrt();
        var_sqrt_vds_vdssprime_deltav = assign1170_e913;
        var_sqrt_vds_vdssprime_deltav_dn0 = ((((var_vdsprime_dn0 * var_vdsprime) + (var_vdsprime * var_vdsprime_dn0)) + var_deltav_2_dn0) / (2.0 * assign1170_e913));
        var_sqrt_vds_vdssprime_deltav_dn1 = ((((var_vdsprime_dn1 * var_vdsprime) + (var_vdsprime * var_vdsprime_dn1)) + var_deltav_2_dn1) / (2.0 * assign1170_e913));
        var_sqrt_vds_vdssprime_deltav_dn2 = ((((var_vdsprime_dn2 * var_vdsprime) + (var_vdsprime * var_vdsprime_dn2)) + var_deltav_2_dn2) / (2.0 * assign1170_e913));
        var_sqrt_vds_vdssprime_deltav_dn3 = ((((var_vdsprime_dn3 * var_vdsprime) + (var_vdsprime * var_vdsprime_dn3)) + var_deltav_2_dn3) / (2.0 * assign1170_e913));

        let assign1180_e916: f64 = (var_vp - var_vds);
        let assign1180_e918: f64 = (assign1180_e916 - var_vs);
        let assign1180_e920: f64 = (assign1180_e918 - var_sqrt_vdssprime_deltav);
        let assign1180_e922: f64 = (assign1180_e920 + var_sqrt_vds_vdssprime_deltav);
        let assign1180_e924: f64 = (assign1180_e922 * var_inv_vt);
        var_tmp1 = assign1180_e924;
        var_tmp1_dn0 = (((((var_vp_dn0 - var_vds_dn0) - var_vs_dn0) - var_sqrt_vdssprime_deltav_dn0) + var_sqrt_vds_vdssprime_deltav_dn0) * var_inv_vt);
        var_tmp1_dn1 = (((var_vp_dn1 - var_sqrt_vdssprime_deltav_dn1) + var_sqrt_vds_vdssprime_deltav_dn1) * var_inv_vt);
        var_tmp1_dn2 = (((((var_vp_dn2 - var_vds_dn2) - var_vs_dn2) - var_sqrt_vdssprime_deltav_dn2) + var_sqrt_vds_vdssprime_deltav_dn2) * var_inv_vt);
        var_tmp1_dn3 = (((((var_vp_dn3 - var_vds_dn3) - var_vs_dn3) - var_sqrt_vdssprime_deltav_dn3) + var_sqrt_vds_vdssprime_deltav_dn3) * var_inv_vt);

        let assign1190_e927: f64 = (-0.35);
        let assign1190_e928: f64 = if var_tmp1 > assign1190_e927 { 1.0 } else { 0.0 };
        var_guard10 = assign1190_e928;

        let (assign1200_e941, assign1200_e941_d_n0, assign1200_e941_d_n1, assign1200_e941_d_n2, assign1200_e941_d_n3,) = {
    if (var_guard10 != 0.0) {
        let assign1200_e933: f64 = (1.3 + var_tmp1);
        let assign1200_e936: f64 = (var_tmp1 + 1.6);
        let assign1200_e937: f64 = (assign1200_e936).ln();
        let assign1200_e938: f64 = (assign1200_e933 - assign1200_e937);
        let assign1200_e939: f64 = (2.0 / assign1200_e938);
        (assign1200_e939, (-((2.0 * (var_tmp1_dn0 - (var_tmp1_dn0 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (var_tmp1_dn1 - (var_tmp1_dn1 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (var_tmp1_dn2 - (var_tmp1_dn2 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (var_tmp1_dn3 - (var_tmp1_dn3 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign1200_e941;
        var_z0_dn0 = assign1200_e941_d_n0;
        var_z0_dn1 = assign1200_e941_d_n1;
        var_z0_dn2 = assign1200_e941_d_n2;
        var_z0_dn3 = assign1200_e941_d_n3;

        let (assign1210_e954, assign1210_e954_d_n0, assign1210_e954_d_n1, assign1210_e954_d_n2, assign1210_e954_d_n3,) = {
    if (var_guard10 != 0.0) {
        let assign1210_e945: f64 = (2.0 + var_z0);
        let assign1210_e948: f64 = (1.0 + var_tmp1);
        let assign1210_e950: f64 = (var_z0).ln();
        let assign1210_e951: f64 = (assign1210_e948 + assign1210_e950);
        let assign1210_e952: f64 = (assign1210_e945 / assign1210_e951);
        (assign1210_e952, (((var_z0_dn0 * assign1210_e951) - (assign1210_e945 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign1210_e951 * assign1210_e951)), (((var_z0_dn1 * assign1210_e951) - (assign1210_e945 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign1210_e951 * assign1210_e951)), (((var_z0_dn2 * assign1210_e951) - (assign1210_e945 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign1210_e951 * assign1210_e951)), (((var_z0_dn3 * assign1210_e951) - (assign1210_e945 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign1210_e951 * assign1210_e951)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign1210_e954;
        var_zk_dn0 = assign1210_e954_d_n0;
        var_zk_dn1 = assign1210_e954_d_n1;
        var_zk_dn2 = assign1210_e954_d_n2;
        var_zk_dn3 = assign1210_e954_d_n3;

        let (assign1220_e967, assign1220_e967_d_n0, assign1220_e967_d_n1, assign1220_e967_d_n2, assign1220_e967_d_n3,) = {
    if (var_guard10 != 0.0) {
        let assign1220_e958: f64 = (1.0 + var_tmp1);
        let assign1220_e960: f64 = (var_zk).ln();
        let assign1220_e961: f64 = (assign1220_e958 + assign1220_e960);
        let assign1220_e964: f64 = (2.0 + var_zk);
        let assign1220_e965: f64 = (assign1220_e961 / assign1220_e964);
        (assign1220_e965, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign1220_e964) - (assign1220_e961 * var_zk_dn0)) / (assign1220_e964 * assign1220_e964)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign1220_e964) - (assign1220_e961 * var_zk_dn1)) / (assign1220_e964 * assign1220_e964)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign1220_e964) - (assign1220_e961 * var_zk_dn2)) / (assign1220_e964 * assign1220_e964)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign1220_e964) - (assign1220_e961 * var_zk_dn3)) / (assign1220_e964 * assign1220_e964)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1220_e967;
        var_yk_dn0 = assign1220_e967_d_n0;
        var_yk_dn1 = assign1220_e967_d_n1;
        var_yk_dn2 = assign1220_e967_d_n2;
        var_yk_dn3 = assign1220_e967_d_n3;

        let assign1230_e970: f64 = (-15.0);
        let assign1230_e971: f64 = if var_tmp1 > assign1230_e970 { 1.0 } else { 0.0 };
        var_guard11 = assign1230_e971;

        let (assign1240_e982, assign1240_e982_d_n0, assign1240_e982_d_n1, assign1240_e982_d_n2, assign1240_e982_d_n3,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let assign1240_e978: f64 = (-var_tmp1);
        let assign1240_e979: f64 = (assign1240_e978).exp();
        let assign1240_e980: f64 = (1.55 + assign1240_e979);
        (assign1240_e980, (assign1240_e979 * (-var_tmp1_dn0)), (assign1240_e979 * (-var_tmp1_dn1)), (assign1240_e979 * (-var_tmp1_dn2)), (assign1240_e979 * (-var_tmp1_dn3)),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign1240_e982;
        var_z0_dn0 = assign1240_e982_d_n0;
        var_z0_dn1 = assign1240_e982_d_n1;
        var_z0_dn2 = assign1240_e982_d_n2;
        var_z0_dn3 = assign1240_e982_d_n3;

        let (assign1250_e998, assign1250_e998_d_n0, assign1250_e998_d_n1, assign1250_e998_d_n2, assign1250_e998_d_n3,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let assign1250_e989: f64 = (2.0 + var_z0);
        let assign1250_e992: f64 = (1.0 + var_tmp1);
        let assign1250_e994: f64 = (var_z0).ln();
        let assign1250_e995: f64 = (assign1250_e992 + assign1250_e994);
        let assign1250_e996: f64 = (assign1250_e989 / assign1250_e995);
        (assign1250_e996, (((var_z0_dn0 * assign1250_e995) - (assign1250_e989 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign1250_e995 * assign1250_e995)), (((var_z0_dn1 * assign1250_e995) - (assign1250_e989 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign1250_e995 * assign1250_e995)), (((var_z0_dn2 * assign1250_e995) - (assign1250_e989 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign1250_e995 * assign1250_e995)), (((var_z0_dn3 * assign1250_e995) - (assign1250_e989 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign1250_e995 * assign1250_e995)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign1250_e998;
        var_zk_dn0 = assign1250_e998_d_n0;
        var_zk_dn1 = assign1250_e998_d_n1;
        var_zk_dn2 = assign1250_e998_d_n2;
        var_zk_dn3 = assign1250_e998_d_n3;

        let (assign1260_e1014, assign1260_e1014_d_n0, assign1260_e1014_d_n1, assign1260_e1014_d_n2, assign1260_e1014_d_n3,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let assign1260_e1005: f64 = (1.0 + var_tmp1);
        let assign1260_e1007: f64 = (var_zk).ln();
        let assign1260_e1008: f64 = (assign1260_e1005 + assign1260_e1007);
        let assign1260_e1011: f64 = (2.0 + var_zk);
        let assign1260_e1012: f64 = (assign1260_e1008 / assign1260_e1011);
        (assign1260_e1012, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign1260_e1011) - (assign1260_e1008 * var_zk_dn0)) / (assign1260_e1011 * assign1260_e1011)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign1260_e1011) - (assign1260_e1008 * var_zk_dn1)) / (assign1260_e1011 * assign1260_e1011)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign1260_e1011) - (assign1260_e1008 * var_zk_dn2)) / (assign1260_e1011 * assign1260_e1011)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign1260_e1011) - (assign1260_e1008 * var_zk_dn3)) / (assign1260_e1011 * assign1260_e1011)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1260_e1014;
        var_yk_dn0 = assign1260_e1014_d_n0;
        var_yk_dn1 = assign1260_e1014_d_n1;
        var_yk_dn2 = assign1260_e1014_d_n2;
        var_yk_dn3 = assign1260_e1014_d_n3;

        *var_big_sqrt_vp_slot = var_big_sqrt_vp;
        *var_big_sqrt_vp0_slot = var_big_sqrt_vp0;
        *var_big_sqrt_vp0_dn0_slot = var_big_sqrt_vp0_dn0;
        *var_big_sqrt_vp0_dn1_slot = var_big_sqrt_vp0_dn1;
        *var_big_sqrt_vp0_dn2_slot = var_big_sqrt_vp0_dn2;
        *var_big_sqrt_vp0_dn3_slot = var_big_sqrt_vp0_dn3;
        *var_big_sqrt_vp_dn0_slot = var_big_sqrt_vp_dn0;
        *var_big_sqrt_vp_dn1_slot = var_big_sqrt_vp_dn1;
        *var_big_sqrt_vp_dn2_slot = var_big_sqrt_vp_dn2;
        *var_big_sqrt_vp_dn3_slot = var_big_sqrt_vp_dn3;
        *var_deltav_2_slot = var_deltav_2;
        *var_deltav_2_dn0_slot = var_deltav_2_dn0;
        *var_deltav_2_dn1_slot = var_deltav_2_dn1;
        *var_deltav_2_dn2_slot = var_deltav_2_dn2;
        *var_deltav_2_dn3_slot = var_deltav_2_dn3;
        *var_dif_dv_slot = var_dif_dv;
        *var_dif_dv_dn0_slot = var_dif_dv_dn0;
        *var_dif_dv_dn1_slot = var_dif_dv_dn1;
        *var_dif_dv_dn2_slot = var_dif_dv_dn2;
        *var_dif_dv_dn3_slot = var_dif_dv_dn3;
        *var_gammaprime_slot = var_gammaprime;
        *var_gammaprime_dn0_slot = var_gammaprime_dn0;
        *var_gammaprime_dn1_slot = var_gammaprime_dn1;
        *var_gammaprime_dn2_slot = var_gammaprime_dn2;
        *var_gammaprime_dn3_slot = var_gammaprime_dn3;
        *var_gammastar_slot = var_gammastar;
        *var_gammastar_dn0_slot = var_gammastar_dn0;
        *var_gammastar_dn1_slot = var_gammastar_dn1;
        *var_gammastar_dn2_slot = var_gammastar_dn2;
        *var_gammastar_dn3_slot = var_gammastar_dn3;
        *var_guard10_slot = var_guard10;
        *var_guard11_slot = var_guard11;
        *var_guard7_slot = var_guard7;
        *var_guard8_slot = var_guard8;
        *var_guard9_slot = var_guard9;
        *var_if__slot = var_if_;
        *var_if__dn0_slot = var_if__dn0;
        *var_if__dn1_slot = var_if__dn1;
        *var_if__dn2_slot = var_if__dn2;
        *var_if__dn3_slot = var_if__dn3;
        *var_leta_l_slot = var_leta_l;
        *var_sqrt_gammastar_slot = var_sqrt_gammastar;
        *var_sqrt_gammastar_dn0_slot = var_sqrt_gammastar_dn0;
        *var_sqrt_gammastar_dn1_slot = var_sqrt_gammastar_dn1;
        *var_sqrt_gammastar_dn2_slot = var_sqrt_gammastar_dn2;
        *var_sqrt_gammastar_dn3_slot = var_sqrt_gammastar_dn3;
        *var_sqrt_if_slot = var_sqrt_if;
        *var_sqrt_if_dn0_slot = var_sqrt_if_dn0;
        *var_sqrt_if_dn1_slot = var_sqrt_if_dn1;
        *var_sqrt_if_dn2_slot = var_sqrt_if_dn2;
        *var_sqrt_if_dn3_slot = var_sqrt_if_dn3;
        *var_sqrt_phi_vd_slot = var_sqrt_phi_vd;
        *var_sqrt_phi_vd_dn0_slot = var_sqrt_phi_vd_dn0;
        *var_sqrt_phi_vd_dn1_slot = var_sqrt_phi_vd_dn1;
        *var_sqrt_phi_vd_dn2_slot = var_sqrt_phi_vd_dn2;
        *var_sqrt_phi_vd_dn3_slot = var_sqrt_phi_vd_dn3;
        *var_sqrt_phi_vp0_slot = var_sqrt_phi_vp0;
        *var_sqrt_phi_vp0_dn0_slot = var_sqrt_phi_vp0_dn0;
        *var_sqrt_phi_vp0_dn1_slot = var_sqrt_phi_vp0_dn1;
        *var_sqrt_phi_vp0_dn2_slot = var_sqrt_phi_vp0_dn2;
        *var_sqrt_phi_vp0_dn3_slot = var_sqrt_phi_vp0_dn3;
        *var_sqrt_vds_vdss_deltav_slot = var_sqrt_vds_vdss_deltav;
        *var_sqrt_vds_vdss_deltav_dn0_slot = var_sqrt_vds_vdss_deltav_dn0;
        *var_sqrt_vds_vdss_deltav_dn1_slot = var_sqrt_vds_vdss_deltav_dn1;
        *var_sqrt_vds_vdss_deltav_dn2_slot = var_sqrt_vds_vdss_deltav_dn2;
        *var_sqrt_vds_vdss_deltav_dn3_slot = var_sqrt_vds_vdss_deltav_dn3;
        *var_sqrt_vds_vdssprime_deltav_slot = var_sqrt_vds_vdssprime_deltav;
        *var_sqrt_vds_vdssprime_deltav_dn0_slot = var_sqrt_vds_vdssprime_deltav_dn0;
        *var_sqrt_vds_vdssprime_deltav_dn1_slot = var_sqrt_vds_vdssprime_deltav_dn1;
        *var_sqrt_vds_vdssprime_deltav_dn2_slot = var_sqrt_vds_vdssprime_deltav_dn2;
        *var_sqrt_vds_vdssprime_deltav_dn3_slot = var_sqrt_vds_vdssprime_deltav_dn3;
        *var_sqrt_vdss_deltav_slot = var_sqrt_vdss_deltav;
        *var_sqrt_vdss_deltav_dn0_slot = var_sqrt_vdss_deltav_dn0;
        *var_sqrt_vdss_deltav_dn1_slot = var_sqrt_vdss_deltav_dn1;
        *var_sqrt_vdss_deltav_dn2_slot = var_sqrt_vdss_deltav_dn2;
        *var_sqrt_vdss_deltav_dn3_slot = var_sqrt_vdss_deltav_dn3;
        *var_sqrt_vdssprime_deltav_slot = var_sqrt_vdssprime_deltav;
        *var_sqrt_vdssprime_deltav_dn0_slot = var_sqrt_vdssprime_deltav_dn0;
        *var_sqrt_vdssprime_deltav_dn1_slot = var_sqrt_vdssprime_deltav_dn1;
        *var_sqrt_vdssprime_deltav_dn2_slot = var_sqrt_vdssprime_deltav_dn2;
        *var_sqrt_vdssprime_deltav_dn3_slot = var_sqrt_vdssprime_deltav_dn3;
        *var_tmp1_slot = var_tmp1;
        *var_tmp1_dn0_slot = var_tmp1_dn0;
        *var_tmp1_dn1_slot = var_tmp1_dn1;
        *var_tmp1_dn2_slot = var_tmp1_dn2;
        *var_tmp1_dn3_slot = var_tmp1_dn3;
        *var_vds_slot = var_vds;
        *var_vds_dn0_slot = var_vds_dn0;
        *var_vds_dn2_slot = var_vds_dn2;
        *var_vds_dn3_slot = var_vds_dn3;
        *var_vdsprime_slot = var_vdsprime;
        *var_vdsprime_dn0_slot = var_vdsprime_dn0;
        *var_vdsprime_dn1_slot = var_vdsprime_dn1;
        *var_vdsprime_dn2_slot = var_vdsprime_dn2;
        *var_vdsprime_dn3_slot = var_vdsprime_dn3;
        *var_vdss_slot = var_vdss;
        *var_vdss_dn0_slot = var_vdss_dn0;
        *var_vdss_dn1_slot = var_vdss_dn1;
        *var_vdss_dn2_slot = var_vdss_dn2;
        *var_vdss_dn3_slot = var_vdss_dn3;
        *var_vdss_sqrt_slot = var_vdss_sqrt;
        *var_vdss_sqrt_dn0_slot = var_vdss_sqrt_dn0;
        *var_vdss_sqrt_dn1_slot = var_vdss_sqrt_dn1;
        *var_vdss_sqrt_dn2_slot = var_vdss_sqrt_dn2;
        *var_vdss_sqrt_dn3_slot = var_vdss_sqrt_dn3;
        *var_vdssprime_slot = var_vdssprime;
        *var_vdssprime_dn0_slot = var_vdssprime_dn0;
        *var_vdssprime_dn1_slot = var_vdssprime_dn1;
        *var_vdssprime_dn2_slot = var_vdssprime_dn2;
        *var_vdssprime_dn3_slot = var_vdssprime_dn3;
        *var_vdssprime_sqrt_slot = var_vdssprime_sqrt;
        *var_vdssprime_sqrt_dn0_slot = var_vdssprime_sqrt_dn0;
        *var_vdssprime_sqrt_dn1_slot = var_vdssprime_sqrt_dn1;
        *var_vdssprime_sqrt_dn2_slot = var_vdssprime_sqrt_dn2;
        *var_vdssprime_sqrt_dn3_slot = var_vdssprime_sqrt_dn3;
        *var_vip_slot = var_vip;
        *var_vip_dn0_slot = var_vip_dn0;
        *var_vip_dn1_slot = var_vip_dn1;
        *var_vip_dn2_slot = var_vip_dn2;
        *var_vip_dn3_slot = var_vip_dn3;
        *var_vp_slot = var_vp;
        *var_vp0_slot = var_vp0;
        *var_vp0_dn0_slot = var_vp0_dn0;
        *var_vp0_dn1_slot = var_vp0_dn1;
        *var_vp0_dn2_slot = var_vp0_dn2;
        *var_vp0_dn3_slot = var_vp0_dn3;
        *var_vp_dn0_slot = var_vp_dn0;
        *var_vp_dn1_slot = var_vp_dn1;
        *var_vp_dn2_slot = var_vp_dn2;
        *var_vp_dn3_slot = var_vp_dn3;
        *var_vt_vc_slot = var_vt_vc;
        *var_weta_w_slot = var_weta_w;
        *var_yk_slot = var_yk;
        *var_yk_dn0_slot = var_yk_dn0;
        *var_yk_dn1_slot = var_yk_dn1;
        *var_yk_dn2_slot = var_yk_dn2;
        *var_yk_dn3_slot = var_yk_dn3;
        *var_z0_slot = var_z0;
        *var_z0_dn0_slot = var_z0_dn0;
        *var_z0_dn1_slot = var_z0_dn1;
        *var_z0_dn2_slot = var_z0_dn2;
        *var_z0_dn3_slot = var_z0_dn3;
        *var_zk_slot = var_zk;
        *var_zk_dn0_slot = var_zk_dn0;
        *var_zk_dn1_slot = var_zk_dn1;
        *var_zk_dn2_slot = var_zk_dn2;
        *var_zk_dn3_slot = var_zk_dn3;
    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        var_eta_qi: f64,
        var_gamma_s: f64,
        var_gamma_sqrt_phi: f64,
        var_gamma_sqrt_phi_dn0: f64,
        var_gamma_sqrt_phi_dn1: f64,
        var_gamma_sqrt_phi_dn2: f64,
        var_gamma_sqrt_phi_dn3: f64,
        var_guard10: f64,
        var_guard11: f64,
        var_if_: f64,
        var_if__dn0: f64,
        var_if__dn1: f64,
        var_if__dn2: f64,
        var_if__dn3: f64,
        var_inv_ucrit: f64,
        var_inv_vt: f64,
        var_kp_weff: f64,
        var_lc_lambda: f64,
        var_lc_ucrit: f64,
        var_leff: f64,
        var_phi_t: f64,
        var_phi_t_dn0: f64,
        var_phi_t_dn1: f64,
        var_phi_t_dn2: f64,
        var_phi_t_dn3: f64,
        var_t0: f64,
        var_vd: f64,
        var_vd_dn0: f64,
        var_vd_dn2: f64,
        var_vd_dn3: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn2: f64,
        var_vds_dn3: f64,
        var_vip: f64,
        var_vip_dn0: f64,
        var_vip_dn1: f64,
        var_vip_dn2: f64,
        var_vip_dn3: f64,
        var_vp: f64,
        var_vp_dn0: f64,
        var_vp_dn1: f64,
        var_vp_dn2: f64,
        var_vp_dn3: f64,
        var_vt: f64,
        var_vt_4: f64,
        var_vt_vt_2: f64,
        var_beta_slot: &mut f64,
        var_beta_dn0_slot: &mut f64,
        var_beta_dn1_slot: &mut f64,
        var_beta_dn2_slot: &mut f64,
        var_beta_dn3_slot: &mut f64,
        var_deltal_slot: &mut f64,
        var_deltal_dn0_slot: &mut f64,
        var_deltal_dn1_slot: &mut f64,
        var_deltal_dn2_slot: &mut f64,
        var_deltal_dn3_slot: &mut f64,
        var_dir_dv_slot: &mut f64,
        var_dir_dv_dn0_slot: &mut f64,
        var_dir_dv_dn1_slot: &mut f64,
        var_dir_dv_dn2_slot: &mut f64,
        var_dir_dv_dn3_slot: &mut f64,
        var_dirprime_dv_slot: &mut f64,
        var_dirprime_dv_dn0_slot: &mut f64,
        var_dirprime_dv_dn1_slot: &mut f64,
        var_dirprime_dv_dn2_slot: &mut f64,
        var_dirprime_dv_dn3_slot: &mut f64,
        var_e0_q_1_slot: &mut f64,
        var_e0_q_1_dn0_slot: &mut f64,
        var_e0_q_1_dn1_slot: &mut f64,
        var_e0_q_1_dn2_slot: &mut f64,
        var_e0_q_1_dn3_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_ir_slot: &mut f64,
        var_ir_dn0_slot: &mut f64,
        var_ir_dn1_slot: &mut f64,
        var_ir_dn2_slot: &mut f64,
        var_ir_dn3_slot: &mut f64,
        var_irprime_slot: &mut f64,
        var_irprime_dn0_slot: &mut f64,
        var_irprime_dn1_slot: &mut f64,
        var_irprime_dn2_slot: &mut f64,
        var_irprime_dn3_slot: &mut f64,
        var_leq_slot: &mut f64,
        var_leq_dn0_slot: &mut f64,
        var_leq_dn1_slot: &mut f64,
        var_leq_dn2_slot: &mut f64,
        var_leq_dn3_slot: &mut f64,
        var_lmin_slot: &mut f64,
        var_lprime_slot: &mut f64,
        var_lprime_dn0_slot: &mut f64,
        var_lprime_dn1_slot: &mut f64,
        var_lprime_dn2_slot: &mut f64,
        var_lprime_dn3_slot: &mut f64,
        var_n_1_slot: &mut f64,
        var_n_1_dn0_slot: &mut f64,
        var_n_1_dn1_slot: &mut f64,
        var_n_1_dn2_slot: &mut f64,
        var_n_1_dn3_slot: &mut f64,
        var_n_1_n_slot: &mut f64,
        var_n_1_n_dn0_slot: &mut f64,
        var_n_1_n_dn1_slot: &mut f64,
        var_n_1_n_dn2_slot: &mut f64,
        var_n_1_n_dn3_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn0_slot: &mut f64,
        var_qb_dn1_slot: &mut f64,
        var_qb_dn2_slot: &mut f64,
        var_qb_dn3_slot: &mut f64,
        var_qi_slot: &mut f64,
        var_qi_dn0_slot: &mut f64,
        var_qi_dn1_slot: &mut f64,
        var_qi_dn2_slot: &mut f64,
        var_qi_dn3_slot: &mut f64,
        var_sif_slot: &mut f64,
        var_sif2_slot: &mut f64,
        var_sif2_dn0_slot: &mut f64,
        var_sif2_dn1_slot: &mut f64,
        var_sif2_dn2_slot: &mut f64,
        var_sif2_dn3_slot: &mut f64,
        var_sif_dn0_slot: &mut f64,
        var_sif_dn1_slot: &mut f64,
        var_sif_dn2_slot: &mut f64,
        var_sif_dn3_slot: &mut f64,
        var_sif_sir_2_slot: &mut f64,
        var_sif_sir_2_dn0_slot: &mut f64,
        var_sif_sir_2_dn1_slot: &mut f64,
        var_sif_sir_2_dn2_slot: &mut f64,
        var_sif_sir_2_dn3_slot: &mut f64,
        var_sir_slot: &mut f64,
        var_sir2_slot: &mut f64,
        var_sir2_dn0_slot: &mut f64,
        var_sir2_dn1_slot: &mut f64,
        var_sir2_dn2_slot: &mut f64,
        var_sir2_dn3_slot: &mut f64,
        var_sir_dn0_slot: &mut f64,
        var_sir_dn1_slot: &mut f64,
        var_sir_dn2_slot: &mut f64,
        var_sir_dn3_slot: &mut f64,
        var_sqrt_lprime_lmin_slot: &mut f64,
        var_sqrt_lprime_lmin_dn0_slot: &mut f64,
        var_sqrt_lprime_lmin_dn1_slot: &mut f64,
        var_sqrt_lprime_lmin_dn2_slot: &mut f64,
        var_sqrt_lprime_lmin_dn3_slot: &mut f64,
        var_sqrt_phi_vp_slot: &mut f64,
        var_sqrt_phi_vp_2_slot: &mut f64,
        var_sqrt_phi_vp_2_dn0_slot: &mut f64,
        var_sqrt_phi_vp_2_dn1_slot: &mut f64,
        var_sqrt_phi_vp_2_dn2_slot: &mut f64,
        var_sqrt_phi_vp_2_dn3_slot: &mut f64,
        var_sqrt_phi_vp_dn0_slot: &mut f64,
        var_sqrt_phi_vp_dn1_slot: &mut f64,
        var_sqrt_phi_vp_dn2_slot: &mut f64,
        var_sqrt_phi_vp_dn3_slot: &mut f64,
        var_sqrt_vp_vt_slot: &mut f64,
        var_sqrt_vp_vt_dn0_slot: &mut f64,
        var_sqrt_vp_vt_dn1_slot: &mut f64,
        var_sqrt_vp_vt_dn2_slot: &mut f64,
        var_sqrt_vp_vt_dn3_slot: &mut f64,
        var_t0_gamma_1_slot: &mut f64,
        var_t0_gamma_1_dn0_slot: &mut f64,
        var_t0_gamma_1_dn1_slot: &mut f64,
        var_t0_gamma_1_dn2_slot: &mut f64,
        var_t0_gamma_1_dn3_slot: &mut f64,
        var_theta_vp_1_slot: &mut f64,
        var_theta_vp_1_dn0_slot: &mut f64,
        var_theta_vp_1_dn1_slot: &mut f64,
        var_theta_vp_1_dn2_slot: &mut f64,
        var_theta_vp_1_dn3_slot: &mut f64,
        var_tmp1_slot: &mut f64,
        var_tmp1_dn0_slot: &mut f64,
        var_tmp1_dn1_slot: &mut f64,
        var_tmp1_dn2_slot: &mut f64,
        var_tmp1_dn3_slot: &mut f64,
        var_vp_phi_eps_slot: &mut f64,
        var_vp_phi_eps_dn0_slot: &mut f64,
        var_vp_phi_eps_dn1_slot: &mut f64,
        var_vp_phi_eps_dn2_slot: &mut f64,
        var_vp_phi_eps_dn3_slot: &mut f64,
        var_vpprime_slot: &mut f64,
        var_vpprime_dn0_slot: &mut f64,
        var_vpprime_dn1_slot: &mut f64,
        var_vpprime_dn2_slot: &mut f64,
        var_vpprime_dn3_slot: &mut f64,
        var_yk_slot: &mut f64,
        var_yk_dn0_slot: &mut f64,
        var_yk_dn1_slot: &mut f64,
        var_yk_dn2_slot: &mut f64,
        var_yk_dn3_slot: &mut f64,
        var_z0_slot: &mut f64,
        var_z0_dn0_slot: &mut f64,
        var_z0_dn1_slot: &mut f64,
        var_z0_dn2_slot: &mut f64,
        var_z0_dn3_slot: &mut f64,
        var_zk_slot: &mut f64,
        var_zk_dn0_slot: &mut f64,
        var_zk_dn1_slot: &mut f64,
        var_zk_dn2_slot: &mut f64,
        var_zk_dn3_slot: &mut f64,
    ) {
        let mut var_beta: f64 = *var_beta_slot;
        let mut var_beta_dn0: f64 = *var_beta_dn0_slot;
        let mut var_beta_dn1: f64 = *var_beta_dn1_slot;
        let mut var_beta_dn2: f64 = *var_beta_dn2_slot;
        let mut var_beta_dn3: f64 = *var_beta_dn3_slot;
        let mut var_deltal: f64 = *var_deltal_slot;
        let mut var_deltal_dn0: f64 = *var_deltal_dn0_slot;
        let mut var_deltal_dn1: f64 = *var_deltal_dn1_slot;
        let mut var_deltal_dn2: f64 = *var_deltal_dn2_slot;
        let mut var_deltal_dn3: f64 = *var_deltal_dn3_slot;
        let mut var_dir_dv: f64 = *var_dir_dv_slot;
        let mut var_dir_dv_dn0: f64 = *var_dir_dv_dn0_slot;
        let mut var_dir_dv_dn1: f64 = *var_dir_dv_dn1_slot;
        let mut var_dir_dv_dn2: f64 = *var_dir_dv_dn2_slot;
        let mut var_dir_dv_dn3: f64 = *var_dir_dv_dn3_slot;
        let mut var_dirprime_dv: f64 = *var_dirprime_dv_slot;
        let mut var_dirprime_dv_dn0: f64 = *var_dirprime_dv_dn0_slot;
        let mut var_dirprime_dv_dn1: f64 = *var_dirprime_dv_dn1_slot;
        let mut var_dirprime_dv_dn2: f64 = *var_dirprime_dv_dn2_slot;
        let mut var_dirprime_dv_dn3: f64 = *var_dirprime_dv_dn3_slot;
        let mut var_e0_q_1: f64 = *var_e0_q_1_slot;
        let mut var_e0_q_1_dn0: f64 = *var_e0_q_1_dn0_slot;
        let mut var_e0_q_1_dn1: f64 = *var_e0_q_1_dn1_slot;
        let mut var_e0_q_1_dn2: f64 = *var_e0_q_1_dn2_slot;
        let mut var_e0_q_1_dn3: f64 = *var_e0_q_1_dn3_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_ir: f64 = *var_ir_slot;
        let mut var_ir_dn0: f64 = *var_ir_dn0_slot;
        let mut var_ir_dn1: f64 = *var_ir_dn1_slot;
        let mut var_ir_dn2: f64 = *var_ir_dn2_slot;
        let mut var_ir_dn3: f64 = *var_ir_dn3_slot;
        let mut var_irprime: f64 = *var_irprime_slot;
        let mut var_irprime_dn0: f64 = *var_irprime_dn0_slot;
        let mut var_irprime_dn1: f64 = *var_irprime_dn1_slot;
        let mut var_irprime_dn2: f64 = *var_irprime_dn2_slot;
        let mut var_irprime_dn3: f64 = *var_irprime_dn3_slot;
        let mut var_leq: f64 = *var_leq_slot;
        let mut var_leq_dn0: f64 = *var_leq_dn0_slot;
        let mut var_leq_dn1: f64 = *var_leq_dn1_slot;
        let mut var_leq_dn2: f64 = *var_leq_dn2_slot;
        let mut var_leq_dn3: f64 = *var_leq_dn3_slot;
        let mut var_lmin: f64 = *var_lmin_slot;
        let mut var_lprime: f64 = *var_lprime_slot;
        let mut var_lprime_dn0: f64 = *var_lprime_dn0_slot;
        let mut var_lprime_dn1: f64 = *var_lprime_dn1_slot;
        let mut var_lprime_dn2: f64 = *var_lprime_dn2_slot;
        let mut var_lprime_dn3: f64 = *var_lprime_dn3_slot;
        let mut var_n_1: f64 = *var_n_1_slot;
        let mut var_n_1_dn0: f64 = *var_n_1_dn0_slot;
        let mut var_n_1_dn1: f64 = *var_n_1_dn1_slot;
        let mut var_n_1_dn2: f64 = *var_n_1_dn2_slot;
        let mut var_n_1_dn3: f64 = *var_n_1_dn3_slot;
        let mut var_n_1_n: f64 = *var_n_1_n_slot;
        let mut var_n_1_n_dn0: f64 = *var_n_1_n_dn0_slot;
        let mut var_n_1_n_dn1: f64 = *var_n_1_n_dn1_slot;
        let mut var_n_1_n_dn2: f64 = *var_n_1_n_dn2_slot;
        let mut var_n_1_n_dn3: f64 = *var_n_1_n_dn3_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn0: f64 = *var_qb_dn0_slot;
        let mut var_qb_dn1: f64 = *var_qb_dn1_slot;
        let mut var_qb_dn2: f64 = *var_qb_dn2_slot;
        let mut var_qb_dn3: f64 = *var_qb_dn3_slot;
        let mut var_qi: f64 = *var_qi_slot;
        let mut var_qi_dn0: f64 = *var_qi_dn0_slot;
        let mut var_qi_dn1: f64 = *var_qi_dn1_slot;
        let mut var_qi_dn2: f64 = *var_qi_dn2_slot;
        let mut var_qi_dn3: f64 = *var_qi_dn3_slot;
        let mut var_sif: f64 = *var_sif_slot;
        let mut var_sif2: f64 = *var_sif2_slot;
        let mut var_sif2_dn0: f64 = *var_sif2_dn0_slot;
        let mut var_sif2_dn1: f64 = *var_sif2_dn1_slot;
        let mut var_sif2_dn2: f64 = *var_sif2_dn2_slot;
        let mut var_sif2_dn3: f64 = *var_sif2_dn3_slot;
        let mut var_sif_dn0: f64 = *var_sif_dn0_slot;
        let mut var_sif_dn1: f64 = *var_sif_dn1_slot;
        let mut var_sif_dn2: f64 = *var_sif_dn2_slot;
        let mut var_sif_dn3: f64 = *var_sif_dn3_slot;
        let mut var_sif_sir_2: f64 = *var_sif_sir_2_slot;
        let mut var_sif_sir_2_dn0: f64 = *var_sif_sir_2_dn0_slot;
        let mut var_sif_sir_2_dn1: f64 = *var_sif_sir_2_dn1_slot;
        let mut var_sif_sir_2_dn2: f64 = *var_sif_sir_2_dn2_slot;
        let mut var_sif_sir_2_dn3: f64 = *var_sif_sir_2_dn3_slot;
        let mut var_sir: f64 = *var_sir_slot;
        let mut var_sir2: f64 = *var_sir2_slot;
        let mut var_sir2_dn0: f64 = *var_sir2_dn0_slot;
        let mut var_sir2_dn1: f64 = *var_sir2_dn1_slot;
        let mut var_sir2_dn2: f64 = *var_sir2_dn2_slot;
        let mut var_sir2_dn3: f64 = *var_sir2_dn3_slot;
        let mut var_sir_dn0: f64 = *var_sir_dn0_slot;
        let mut var_sir_dn1: f64 = *var_sir_dn1_slot;
        let mut var_sir_dn2: f64 = *var_sir_dn2_slot;
        let mut var_sir_dn3: f64 = *var_sir_dn3_slot;
        let mut var_sqrt_lprime_lmin: f64 = *var_sqrt_lprime_lmin_slot;
        let mut var_sqrt_lprime_lmin_dn0: f64 = *var_sqrt_lprime_lmin_dn0_slot;
        let mut var_sqrt_lprime_lmin_dn1: f64 = *var_sqrt_lprime_lmin_dn1_slot;
        let mut var_sqrt_lprime_lmin_dn2: f64 = *var_sqrt_lprime_lmin_dn2_slot;
        let mut var_sqrt_lprime_lmin_dn3: f64 = *var_sqrt_lprime_lmin_dn3_slot;
        let mut var_sqrt_phi_vp: f64 = *var_sqrt_phi_vp_slot;
        let mut var_sqrt_phi_vp_2: f64 = *var_sqrt_phi_vp_2_slot;
        let mut var_sqrt_phi_vp_2_dn0: f64 = *var_sqrt_phi_vp_2_dn0_slot;
        let mut var_sqrt_phi_vp_2_dn1: f64 = *var_sqrt_phi_vp_2_dn1_slot;
        let mut var_sqrt_phi_vp_2_dn2: f64 = *var_sqrt_phi_vp_2_dn2_slot;
        let mut var_sqrt_phi_vp_2_dn3: f64 = *var_sqrt_phi_vp_2_dn3_slot;
        let mut var_sqrt_phi_vp_dn0: f64 = *var_sqrt_phi_vp_dn0_slot;
        let mut var_sqrt_phi_vp_dn1: f64 = *var_sqrt_phi_vp_dn1_slot;
        let mut var_sqrt_phi_vp_dn2: f64 = *var_sqrt_phi_vp_dn2_slot;
        let mut var_sqrt_phi_vp_dn3: f64 = *var_sqrt_phi_vp_dn3_slot;
        let mut var_sqrt_vp_vt: f64 = *var_sqrt_vp_vt_slot;
        let mut var_sqrt_vp_vt_dn0: f64 = *var_sqrt_vp_vt_dn0_slot;
        let mut var_sqrt_vp_vt_dn1: f64 = *var_sqrt_vp_vt_dn1_slot;
        let mut var_sqrt_vp_vt_dn2: f64 = *var_sqrt_vp_vt_dn2_slot;
        let mut var_sqrt_vp_vt_dn3: f64 = *var_sqrt_vp_vt_dn3_slot;
        let mut var_t0_gamma_1: f64 = *var_t0_gamma_1_slot;
        let mut var_t0_gamma_1_dn0: f64 = *var_t0_gamma_1_dn0_slot;
        let mut var_t0_gamma_1_dn1: f64 = *var_t0_gamma_1_dn1_slot;
        let mut var_t0_gamma_1_dn2: f64 = *var_t0_gamma_1_dn2_slot;
        let mut var_t0_gamma_1_dn3: f64 = *var_t0_gamma_1_dn3_slot;
        let mut var_theta_vp_1: f64 = *var_theta_vp_1_slot;
        let mut var_theta_vp_1_dn0: f64 = *var_theta_vp_1_dn0_slot;
        let mut var_theta_vp_1_dn1: f64 = *var_theta_vp_1_dn1_slot;
        let mut var_theta_vp_1_dn2: f64 = *var_theta_vp_1_dn2_slot;
        let mut var_theta_vp_1_dn3: f64 = *var_theta_vp_1_dn3_slot;
        let mut var_tmp1: f64 = *var_tmp1_slot;
        let mut var_tmp1_dn0: f64 = *var_tmp1_dn0_slot;
        let mut var_tmp1_dn1: f64 = *var_tmp1_dn1_slot;
        let mut var_tmp1_dn2: f64 = *var_tmp1_dn2_slot;
        let mut var_tmp1_dn3: f64 = *var_tmp1_dn3_slot;
        let mut var_vp_phi_eps: f64 = *var_vp_phi_eps_slot;
        let mut var_vp_phi_eps_dn0: f64 = *var_vp_phi_eps_dn0_slot;
        let mut var_vp_phi_eps_dn1: f64 = *var_vp_phi_eps_dn1_slot;
        let mut var_vp_phi_eps_dn2: f64 = *var_vp_phi_eps_dn2_slot;
        let mut var_vp_phi_eps_dn3: f64 = *var_vp_phi_eps_dn3_slot;
        let mut var_vpprime: f64 = *var_vpprime_slot;
        let mut var_vpprime_dn0: f64 = *var_vpprime_dn0_slot;
        let mut var_vpprime_dn1: f64 = *var_vpprime_dn1_slot;
        let mut var_vpprime_dn2: f64 = *var_vpprime_dn2_slot;
        let mut var_vpprime_dn3: f64 = *var_vpprime_dn3_slot;
        let mut var_yk: f64 = *var_yk_slot;
        let mut var_yk_dn0: f64 = *var_yk_dn0_slot;
        let mut var_yk_dn1: f64 = *var_yk_dn1_slot;
        let mut var_yk_dn2: f64 = *var_yk_dn2_slot;
        let mut var_yk_dn3: f64 = *var_yk_dn3_slot;
        let mut var_z0: f64 = *var_z0_slot;
        let mut var_z0_dn0: f64 = *var_z0_dn0_slot;
        let mut var_z0_dn1: f64 = *var_z0_dn1_slot;
        let mut var_z0_dn2: f64 = *var_z0_dn2_slot;
        let mut var_z0_dn3: f64 = *var_z0_dn3_slot;
        let mut var_zk: f64 = *var_zk_slot;
        let mut var_zk_dn0: f64 = *var_zk_dn0_slot;
        let mut var_zk_dn1: f64 = *var_zk_dn1_slot;
        let mut var_zk_dn2: f64 = *var_zk_dn2_slot;
        let mut var_zk_dn3: f64 = *var_zk_dn3_slot;

        let assign1270_e1017: f64 = (-23.0);
        let assign1270_e1018: f64 = if var_tmp1 > assign1270_e1017 { 1.0 } else { 0.0 };
        var_guard12 = assign1270_e1018;

        let (assign1280_e1034, assign1280_e1034_d_n0, assign1280_e1034_d_n1, assign1280_e1034_d_n2, assign1280_e1034_d_n3,) = {
    if (((var_guard10 == 0.0) && (var_guard11 == 0.0)) && (var_guard12 != 0.0)) {
        let assign1280_e1029: f64 = (-var_tmp1);
        let assign1280_e1030: f64 = (assign1280_e1029).exp();
        let assign1280_e1031: f64 = (2.0 + assign1280_e1030);
        let assign1280_e1032: f64 = (1.0 / assign1280_e1031);
        (assign1280_e1032, (-((assign1280_e1030 * (-var_tmp1_dn0)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-var_tmp1_dn1)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-var_tmp1_dn2)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-var_tmp1_dn3)) / (assign1280_e1031 * assign1280_e1031))),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1280_e1034;
        var_yk_dn0 = assign1280_e1034_d_n0;
        var_yk_dn1 = assign1280_e1034_d_n1;
        var_yk_dn2 = assign1280_e1034_d_n2;
        var_yk_dn3 = assign1280_e1034_d_n3;

        let (assign1290_e1048, assign1290_e1048_d_n0, assign1290_e1048_d_n1, assign1290_e1048_d_n2, assign1290_e1048_d_n3,) = {
    if (((var_guard10 == 0.0) && (var_guard11 == 0.0)) && (var_guard12 == 0.0)) {
        let assign1290_e1044: f64 = (var_tmp1).exp();
        let assign1290_e1046: f64 = (assign1290_e1044 + 1e-64);
        (assign1290_e1046, (assign1290_e1044 * var_tmp1_dn0), (assign1290_e1044 * var_tmp1_dn1), (assign1290_e1044 * var_tmp1_dn2), (assign1290_e1044 * var_tmp1_dn3),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1290_e1048;
        var_yk_dn0 = assign1290_e1048_d_n0;
        var_yk_dn1 = assign1290_e1048_d_n1;
        var_yk_dn2 = assign1290_e1048_d_n2;
        var_yk_dn3 = assign1290_e1048_d_n3;

        let assign1300_e1052: f64 = (1.0 + var_yk);
        let assign1300_e1053: f64 = (var_yk * assign1300_e1052);
        var_irprime = assign1300_e1053;
        var_irprime_dn0 = ((var_yk_dn0 * assign1300_e1052) + (var_yk * var_yk_dn0));
        var_irprime_dn1 = ((var_yk_dn1 * assign1300_e1052) + (var_yk * var_yk_dn1));
        var_irprime_dn2 = ((var_yk_dn2 * assign1300_e1052) + (var_yk * var_yk_dn2));
        var_irprime_dn3 = ((var_yk_dn3 * assign1300_e1052) + (var_yk * var_yk_dn3));

        var_dirprime_dv = var_yk;
        var_dirprime_dv_dn0 = var_yk_dn0;
        var_dirprime_dv_dn1 = var_yk_dn1;
        var_dirprime_dv_dn2 = var_yk_dn2;
        var_dirprime_dv_dn3 = var_yk_dn3;

        let assign1330_e1061: f64 = (var_vds - var_vip);
        let assign1330_e1063: f64 = (assign1330_e1061 / var_lc_ucrit);
        let assign1330_e1064: f64 = (1.0 + assign1330_e1063);
        let assign1330_e1065: f64 = (assign1330_e1064).ln();
        let assign1330_e1066: f64 = (var_lc_lambda * assign1330_e1065);
        var_deltal = assign1330_e1066;
        var_deltal_dn0 = (var_lc_lambda * (((var_vds_dn0 - var_vip_dn0) / var_lc_ucrit) / assign1330_e1064));
        var_deltal_dn1 = (var_lc_lambda * (((-var_vip_dn1) / var_lc_ucrit) / assign1330_e1064));
        var_deltal_dn2 = (var_lc_lambda * (((var_vds_dn2 - var_vip_dn2) / var_lc_ucrit) / assign1330_e1064));
        var_deltal_dn3 = (var_lc_lambda * (((var_vds_dn3 - var_vip_dn3) / var_lc_ucrit) / assign1330_e1064));

        let assign1340_e1069: f64 = (var_leff - var_deltal);
        let assign1340_e1072: f64 = (var_vds + var_vip);
        let assign1340_e1074: f64 = (assign1340_e1072 * var_inv_ucrit);
        let assign1340_e1075: f64 = (assign1340_e1069 + assign1340_e1074);
        var_lprime = assign1340_e1075;
        var_lprime_dn0 = ((-var_deltal_dn0) + ((var_vds_dn0 + var_vip_dn0) * var_inv_ucrit));
        var_lprime_dn1 = ((-var_deltal_dn1) + (var_vip_dn1 * var_inv_ucrit));
        var_lprime_dn2 = ((-var_deltal_dn2) + ((var_vds_dn2 + var_vip_dn2) * var_inv_ucrit));
        var_lprime_dn3 = ((-var_deltal_dn3) + ((var_vds_dn3 + var_vip_dn3) * var_inv_ucrit));

        let assign1350_e1078: f64 = (0.1 * var_leff);
        var_lmin = assign1350_e1078;

        let assign1360_e1081: f64 = (var_lprime * var_lprime);
        let assign1360_e1084: f64 = (var_lmin * var_lmin);
        let assign1360_e1085: f64 = (assign1360_e1081 + assign1360_e1084);
        let assign1360_e1086: f64 = (assign1360_e1085).sqrt();
        var_sqrt_lprime_lmin = assign1360_e1086;
        var_sqrt_lprime_lmin_dn0 = (((var_lprime_dn0 * var_lprime) + (var_lprime * var_lprime_dn0)) / (2.0 * assign1360_e1086));
        var_sqrt_lprime_lmin_dn1 = (((var_lprime_dn1 * var_lprime) + (var_lprime * var_lprime_dn1)) / (2.0 * assign1360_e1086));
        var_sqrt_lprime_lmin_dn2 = (((var_lprime_dn2 * var_lprime) + (var_lprime * var_lprime_dn2)) / (2.0 * assign1360_e1086));
        var_sqrt_lprime_lmin_dn3 = (((var_lprime_dn3 * var_lprime) + (var_lprime * var_lprime_dn3)) / (2.0 * assign1360_e1086));

        let assign1370_e1090: f64 = (var_lprime + var_sqrt_lprime_lmin);
        let assign1370_e1091: f64 = (0.5 * assign1370_e1090);
        var_leq = assign1370_e1091;
        var_leq_dn0 = (0.5 * (var_lprime_dn0 + var_sqrt_lprime_lmin_dn0));
        var_leq_dn1 = (0.5 * (var_lprime_dn1 + var_sqrt_lprime_lmin_dn1));
        var_leq_dn2 = (0.5 * (var_lprime_dn2 + var_sqrt_lprime_lmin_dn2));
        var_leq_dn3 = (0.5 * (var_lprime_dn3 + var_sqrt_lprime_lmin_dn3));

        let assign1380_e1094: f64 = (var_vp - var_vd);
        let assign1380_e1096: f64 = (assign1380_e1094 * var_inv_vt);
        var_tmp1 = assign1380_e1096;
        var_tmp1_dn0 = ((var_vp_dn0 - var_vd_dn0) * var_inv_vt);
        var_tmp1_dn1 = (var_vp_dn1 * var_inv_vt);
        var_tmp1_dn2 = ((var_vp_dn2 - var_vd_dn2) * var_inv_vt);
        var_tmp1_dn3 = ((var_vp_dn3 - var_vd_dn3) * var_inv_vt);

        let assign1390_e1099: f64 = (-0.35);
        let assign1390_e1100: f64 = if var_tmp1 > assign1390_e1099 { 1.0 } else { 0.0 };
        var_guard13 = assign1390_e1100;

        let (assign1400_e1113, assign1400_e1113_d_n0, assign1400_e1113_d_n1, assign1400_e1113_d_n2, assign1400_e1113_d_n3,) = {
    if (var_guard13 != 0.0) {
        let assign1400_e1105: f64 = (1.3 + var_tmp1);
        let assign1400_e1108: f64 = (var_tmp1 + 1.6);
        let assign1400_e1109: f64 = (assign1400_e1108).ln();
        let assign1400_e1110: f64 = (assign1400_e1105 - assign1400_e1109);
        let assign1400_e1111: f64 = (2.0 / assign1400_e1110);
        (assign1400_e1111, (-((2.0 * (var_tmp1_dn0 - (var_tmp1_dn0 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (var_tmp1_dn1 - (var_tmp1_dn1 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (var_tmp1_dn2 - (var_tmp1_dn2 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (var_tmp1_dn3 - (var_tmp1_dn3 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign1400_e1113;
        var_z0_dn0 = assign1400_e1113_d_n0;
        var_z0_dn1 = assign1400_e1113_d_n1;
        var_z0_dn2 = assign1400_e1113_d_n2;
        var_z0_dn3 = assign1400_e1113_d_n3;

        let (assign1410_e1126, assign1410_e1126_d_n0, assign1410_e1126_d_n1, assign1410_e1126_d_n2, assign1410_e1126_d_n3,) = {
    if (var_guard13 != 0.0) {
        let assign1410_e1117: f64 = (2.0 + var_z0);
        let assign1410_e1120: f64 = (1.0 + var_tmp1);
        let assign1410_e1122: f64 = (var_z0).ln();
        let assign1410_e1123: f64 = (assign1410_e1120 + assign1410_e1122);
        let assign1410_e1124: f64 = (assign1410_e1117 / assign1410_e1123);
        (assign1410_e1124, (((var_z0_dn0 * assign1410_e1123) - (assign1410_e1117 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((var_z0_dn1 * assign1410_e1123) - (assign1410_e1117 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((var_z0_dn2 * assign1410_e1123) - (assign1410_e1117 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((var_z0_dn3 * assign1410_e1123) - (assign1410_e1117 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign1410_e1123 * assign1410_e1123)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign1410_e1126;
        var_zk_dn0 = assign1410_e1126_d_n0;
        var_zk_dn1 = assign1410_e1126_d_n1;
        var_zk_dn2 = assign1410_e1126_d_n2;
        var_zk_dn3 = assign1410_e1126_d_n3;

        let (assign1420_e1139, assign1420_e1139_d_n0, assign1420_e1139_d_n1, assign1420_e1139_d_n2, assign1420_e1139_d_n3,) = {
    if (var_guard13 != 0.0) {
        let assign1420_e1130: f64 = (1.0 + var_tmp1);
        let assign1420_e1132: f64 = (var_zk).ln();
        let assign1420_e1133: f64 = (assign1420_e1130 + assign1420_e1132);
        let assign1420_e1136: f64 = (2.0 + var_zk);
        let assign1420_e1137: f64 = (assign1420_e1133 / assign1420_e1136);
        (assign1420_e1137, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign1420_e1136) - (assign1420_e1133 * var_zk_dn0)) / (assign1420_e1136 * assign1420_e1136)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign1420_e1136) - (assign1420_e1133 * var_zk_dn1)) / (assign1420_e1136 * assign1420_e1136)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign1420_e1136) - (assign1420_e1133 * var_zk_dn2)) / (assign1420_e1136 * assign1420_e1136)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign1420_e1136) - (assign1420_e1133 * var_zk_dn3)) / (assign1420_e1136 * assign1420_e1136)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1420_e1139;
        var_yk_dn0 = assign1420_e1139_d_n0;
        var_yk_dn1 = assign1420_e1139_d_n1;
        var_yk_dn2 = assign1420_e1139_d_n2;
        var_yk_dn3 = assign1420_e1139_d_n3;

        let assign1430_e1142: f64 = (-15.0);
        let assign1430_e1143: f64 = if var_tmp1 > assign1430_e1142 { 1.0 } else { 0.0 };
        var_guard14 = assign1430_e1143;

        let (assign1440_e1154, assign1440_e1154_d_n0, assign1440_e1154_d_n1, assign1440_e1154_d_n2, assign1440_e1154_d_n3,) = {
    if ((var_guard13 == 0.0) && (var_guard14 != 0.0)) {
        let assign1440_e1150: f64 = (-var_tmp1);
        let assign1440_e1151: f64 = (assign1440_e1150).exp();
        let assign1440_e1152: f64 = (1.55 + assign1440_e1151);
        (assign1440_e1152, (assign1440_e1151 * (-var_tmp1_dn0)), (assign1440_e1151 * (-var_tmp1_dn1)), (assign1440_e1151 * (-var_tmp1_dn2)), (assign1440_e1151 * (-var_tmp1_dn3)),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign1440_e1154;
        var_z0_dn0 = assign1440_e1154_d_n0;
        var_z0_dn1 = assign1440_e1154_d_n1;
        var_z0_dn2 = assign1440_e1154_d_n2;
        var_z0_dn3 = assign1440_e1154_d_n3;

        let (assign1450_e1170, assign1450_e1170_d_n0, assign1450_e1170_d_n1, assign1450_e1170_d_n2, assign1450_e1170_d_n3,) = {
    if ((var_guard13 == 0.0) && (var_guard14 != 0.0)) {
        let assign1450_e1161: f64 = (2.0 + var_z0);
        let assign1450_e1164: f64 = (1.0 + var_tmp1);
        let assign1450_e1166: f64 = (var_z0).ln();
        let assign1450_e1167: f64 = (assign1450_e1164 + assign1450_e1166);
        let assign1450_e1168: f64 = (assign1450_e1161 / assign1450_e1167);
        (assign1450_e1168, (((var_z0_dn0 * assign1450_e1167) - (assign1450_e1161 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((var_z0_dn1 * assign1450_e1167) - (assign1450_e1161 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((var_z0_dn2 * assign1450_e1167) - (assign1450_e1161 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((var_z0_dn3 * assign1450_e1167) - (assign1450_e1161 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign1450_e1167 * assign1450_e1167)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign1450_e1170;
        var_zk_dn0 = assign1450_e1170_d_n0;
        var_zk_dn1 = assign1450_e1170_d_n1;
        var_zk_dn2 = assign1450_e1170_d_n2;
        var_zk_dn3 = assign1450_e1170_d_n3;

        let (assign1460_e1186, assign1460_e1186_d_n0, assign1460_e1186_d_n1, assign1460_e1186_d_n2, assign1460_e1186_d_n3,) = {
    if ((var_guard13 == 0.0) && (var_guard14 != 0.0)) {
        let assign1460_e1177: f64 = (1.0 + var_tmp1);
        let assign1460_e1179: f64 = (var_zk).ln();
        let assign1460_e1180: f64 = (assign1460_e1177 + assign1460_e1179);
        let assign1460_e1183: f64 = (2.0 + var_zk);
        let assign1460_e1184: f64 = (assign1460_e1180 / assign1460_e1183);
        (assign1460_e1184, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign1460_e1183) - (assign1460_e1180 * var_zk_dn0)) / (assign1460_e1183 * assign1460_e1183)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign1460_e1183) - (assign1460_e1180 * var_zk_dn1)) / (assign1460_e1183 * assign1460_e1183)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign1460_e1183) - (assign1460_e1180 * var_zk_dn2)) / (assign1460_e1183 * assign1460_e1183)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign1460_e1183) - (assign1460_e1180 * var_zk_dn3)) / (assign1460_e1183 * assign1460_e1183)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1460_e1186;
        var_yk_dn0 = assign1460_e1186_d_n0;
        var_yk_dn1 = assign1460_e1186_d_n1;
        var_yk_dn2 = assign1460_e1186_d_n2;
        var_yk_dn3 = assign1460_e1186_d_n3;

        let assign1470_e1189: f64 = (-23.0);
        let assign1470_e1190: f64 = if var_tmp1 > assign1470_e1189 { 1.0 } else { 0.0 };
        var_guard15 = assign1470_e1190;

        let (assign1480_e1206, assign1480_e1206_d_n0, assign1480_e1206_d_n1, assign1480_e1206_d_n2, assign1480_e1206_d_n3,) = {
    if (((var_guard13 == 0.0) && (var_guard14 == 0.0)) && (var_guard15 != 0.0)) {
        let assign1480_e1201: f64 = (-var_tmp1);
        let assign1480_e1202: f64 = (assign1480_e1201).exp();
        let assign1480_e1203: f64 = (2.0 + assign1480_e1202);
        let assign1480_e1204: f64 = (1.0 / assign1480_e1203);
        (assign1480_e1204, (-((assign1480_e1202 * (-var_tmp1_dn0)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-var_tmp1_dn1)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-var_tmp1_dn2)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-var_tmp1_dn3)) / (assign1480_e1203 * assign1480_e1203))),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1480_e1206;
        var_yk_dn0 = assign1480_e1206_d_n0;
        var_yk_dn1 = assign1480_e1206_d_n1;
        var_yk_dn2 = assign1480_e1206_d_n2;
        var_yk_dn3 = assign1480_e1206_d_n3;

        let (assign1490_e1220, assign1490_e1220_d_n0, assign1490_e1220_d_n1, assign1490_e1220_d_n2, assign1490_e1220_d_n3,) = {
    if (((var_guard13 == 0.0) && (var_guard14 == 0.0)) && (var_guard15 == 0.0)) {
        let assign1490_e1216: f64 = (var_tmp1).exp();
        let assign1490_e1218: f64 = (assign1490_e1216 + 1e-64);
        (assign1490_e1218, (assign1490_e1216 * var_tmp1_dn0), (assign1490_e1216 * var_tmp1_dn1), (assign1490_e1216 * var_tmp1_dn2), (assign1490_e1216 * var_tmp1_dn3),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1490_e1220;
        var_yk_dn0 = assign1490_e1220_d_n0;
        var_yk_dn1 = assign1490_e1220_d_n1;
        var_yk_dn2 = assign1490_e1220_d_n2;
        var_yk_dn3 = assign1490_e1220_d_n3;

        let assign1500_e1224: f64 = (1.0 + var_yk);
        let assign1500_e1225: f64 = (var_yk * assign1500_e1224);
        var_ir = assign1500_e1225;
        var_ir_dn0 = ((var_yk_dn0 * assign1500_e1224) + (var_yk * var_yk_dn0));
        var_ir_dn1 = ((var_yk_dn1 * assign1500_e1224) + (var_yk * var_yk_dn1));
        var_ir_dn2 = ((var_yk_dn2 * assign1500_e1224) + (var_yk * var_yk_dn2));
        var_ir_dn3 = ((var_yk_dn3 * assign1500_e1224) + (var_yk * var_yk_dn3));

        var_dir_dv = var_yk;
        var_dir_dv_dn0 = var_yk_dn0;
        var_dir_dv_dn1 = var_yk_dn1;
        var_dir_dv_dn2 = var_yk_dn2;
        var_dir_dv_dn3 = var_yk_dn3;

        let assign1530_e1231: f64 = (0.25 + var_if_);
        var_sif2 = assign1530_e1231;
        var_sif2_dn0 = var_if__dn0;
        var_sif2_dn1 = var_if__dn1;
        var_sif2_dn2 = var_if__dn2;
        var_sif2_dn3 = var_if__dn3;

        let assign1540_e1234: f64 = (0.25 + var_ir);
        var_sir2 = assign1540_e1234;
        var_sir2_dn0 = var_ir_dn0;
        var_sir2_dn1 = var_ir_dn1;
        var_sir2_dn2 = var_ir_dn2;
        var_sir2_dn3 = var_ir_dn3;

        let assign1550_e1236: f64 = (var_sif2).sqrt();
        var_sif = assign1550_e1236;
        var_sif_dn0 = (var_sif2_dn0 / (2.0 * assign1550_e1236));
        var_sif_dn1 = (var_sif2_dn1 / (2.0 * assign1550_e1236));
        var_sif_dn2 = (var_sif2_dn2 / (2.0 * assign1550_e1236));
        var_sif_dn3 = (var_sif2_dn3 / (2.0 * assign1550_e1236));

        let assign1560_e1238: f64 = (var_sir2).sqrt();
        var_sir = assign1560_e1238;
        var_sir_dn0 = (var_sir2_dn0 / (2.0 * assign1560_e1238));
        var_sir_dn1 = (var_sir2_dn1 / (2.0 * assign1560_e1238));
        var_sir_dn2 = (var_sir2_dn2 / (2.0 * assign1560_e1238));
        var_sir_dn3 = (var_sir2_dn3 / (2.0 * assign1560_e1238));

        let assign1570_e1241: f64 = (var_sif + var_sir);
        let assign1570_e1244: f64 = (var_sif + var_sir);
        let assign1570_e1245: f64 = (assign1570_e1241 * assign1570_e1244);
        var_sif_sir_2 = assign1570_e1245;
        var_sif_sir_2_dn0 = (((var_sif_dn0 + var_sir_dn0) * assign1570_e1244) + (assign1570_e1241 * (var_sif_dn0 + var_sir_dn0)));
        var_sif_sir_2_dn1 = (((var_sif_dn1 + var_sir_dn1) * assign1570_e1244) + (assign1570_e1241 * (var_sif_dn1 + var_sir_dn1)));
        var_sif_sir_2_dn2 = (((var_sif_dn2 + var_sir_dn2) * assign1570_e1244) + (assign1570_e1241 * (var_sif_dn2 + var_sir_dn2)));
        var_sif_sir_2_dn3 = (((var_sif_dn3 + var_sir_dn3) * assign1570_e1244) + (assign1570_e1241 * (var_sif_dn3 + var_sir_dn3)));

        let assign1580_e1248: f64 = (var_vp + var_phi_t);
        let assign1580_e1250: f64 = (assign1580_e1248 + 1e-6);
        var_vp_phi_eps = assign1580_e1250;
        var_vp_phi_eps_dn0 = (var_vp_dn0 + var_phi_t_dn0);
        var_vp_phi_eps_dn1 = (var_vp_dn1 + var_phi_t_dn1);
        var_vp_phi_eps_dn2 = (var_vp_dn2 + var_phi_t_dn2);
        var_vp_phi_eps_dn3 = (var_vp_dn3 + var_phi_t_dn3);

        let assign1590_e1253: f64 = (var_vp_phi_eps).sqrt();
        let assign1590_e1254: f64 = (2.0 * assign1590_e1253);
        var_sqrt_phi_vp_2 = assign1590_e1254;
        var_sqrt_phi_vp_2_dn0 = (2.0 * (var_vp_phi_eps_dn0 / (2.0 * assign1590_e1253)));
        var_sqrt_phi_vp_2_dn1 = (2.0 * (var_vp_phi_eps_dn1 / (2.0 * assign1590_e1253)));
        var_sqrt_phi_vp_2_dn2 = (2.0 * (var_vp_phi_eps_dn2 / (2.0 * assign1590_e1253)));
        var_sqrt_phi_vp_2_dn3 = (2.0 * (var_vp_phi_eps_dn3 / (2.0 * assign1590_e1253)));

        let assign1600_e1257: f64 = (var_gamma_s / var_sqrt_phi_vp_2);
        var_n_1 = assign1600_e1257;
        var_n_1_dn0 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn0) / (var_sqrt_phi_vp_2 * var_sqrt_phi_vp_2)));
        var_n_1_dn1 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn1) / (var_sqrt_phi_vp_2 * var_sqrt_phi_vp_2)));
        var_n_1_dn2 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn2) / (var_sqrt_phi_vp_2 * var_sqrt_phi_vp_2)));
        var_n_1_dn3 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn3) / (var_sqrt_phi_vp_2 * var_sqrt_phi_vp_2)));

        let assign1610_e1261: f64 = (var_sqrt_phi_vp_2 + var_gamma_s);
        let assign1610_e1262: f64 = (var_gamma_s / assign1610_e1261);
        var_n_1_n = assign1610_e1262;
        var_n_1_n_dn0 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn0) / (assign1610_e1261 * assign1610_e1261)));
        var_n_1_n_dn1 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn1) / (assign1610_e1261 * assign1610_e1261)));
        var_n_1_n_dn2 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn2) / (assign1610_e1261 * assign1610_e1261)));
        var_n_1_n_dn3 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn3) / (assign1610_e1261 * assign1610_e1261)));

        let assign1620_e1265: f64 = (1.0 + var_n_1);
        let assign1620_e1266: f64 = (-assign1620_e1265);
        let assign1620_e1268: f64 = (assign1620_e1266 * var_vt);
        let assign1620_e1271: f64 = (0.66666666 + 0.66666666);
        let assign1620_e1275: f64 = (var_sir * var_sif);
        let assign1620_e1276: f64 = (var_sir2 + assign1620_e1275);
        let assign1620_e1278: f64 = (assign1620_e1276 + var_sif2);
        let assign1620_e1279: f64 = (assign1620_e1271 * assign1620_e1278);
        let assign1620_e1282: f64 = (var_sif + var_sir);
        let assign1620_e1283: f64 = (assign1620_e1279 / assign1620_e1282);
        let assign1620_e1285: f64 = (assign1620_e1283 - 1.0);
        let assign1620_e1286: f64 = (assign1620_e1268 * assign1620_e1285);
        var_qi = assign1620_e1286;
        var_qi_dn0 = ((((-var_n_1_dn0) * var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((var_sir2_dn0 + ((var_sir_dn0 * var_sif) + (var_sir * var_sif_dn0))) + var_sif2_dn0)) * assign1620_e1282) - (assign1620_e1279 * (var_sif_dn0 + var_sir_dn0))) / (assign1620_e1282 * assign1620_e1282))));
        var_qi_dn1 = ((((-var_n_1_dn1) * var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((var_sir2_dn1 + ((var_sir_dn1 * var_sif) + (var_sir * var_sif_dn1))) + var_sif2_dn1)) * assign1620_e1282) - (assign1620_e1279 * (var_sif_dn1 + var_sir_dn1))) / (assign1620_e1282 * assign1620_e1282))));
        var_qi_dn2 = ((((-var_n_1_dn2) * var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((var_sir2_dn2 + ((var_sir_dn2 * var_sif) + (var_sir * var_sif_dn2))) + var_sif2_dn2)) * assign1620_e1282) - (assign1620_e1279 * (var_sif_dn2 + var_sir_dn2))) / (assign1620_e1282 * assign1620_e1282))));
        var_qi_dn3 = ((((-var_n_1_dn3) * var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((var_sir2_dn3 + ((var_sir_dn3 * var_sif) + (var_sir * var_sif_dn3))) + var_sif2_dn3)) * assign1620_e1282) - (assign1620_e1279 * (var_sif_dn3 + var_sir_dn3))) / (assign1620_e1282 * assign1620_e1282))));

        let assign1630_e1288: f64 = (-0.5);
        let assign1630_e1290: f64 = (assign1630_e1288 * var_gamma_s);
        let assign1630_e1292: f64 = (assign1630_e1290 * var_sqrt_phi_vp_2);
        let assign1630_e1295: f64 = (var_n_1_n * var_qi);
        let assign1630_e1296: f64 = (assign1630_e1292 - assign1630_e1295);
        var_qb = assign1630_e1296;
        var_qb_dn0 = ((assign1630_e1290 * var_sqrt_phi_vp_2_dn0) - ((var_n_1_n_dn0 * var_qi) + (var_n_1_n * var_qi_dn0)));
        var_qb_dn1 = ((assign1630_e1290 * var_sqrt_phi_vp_2_dn1) - ((var_n_1_n_dn1 * var_qi) + (var_n_1_n * var_qi_dn1)));
        var_qb_dn2 = ((assign1630_e1290 * var_sqrt_phi_vp_2_dn2) - ((var_n_1_n_dn2 * var_qi) + (var_n_1_n * var_qi_dn2)));
        var_qb_dn3 = ((assign1630_e1290 * var_sqrt_phi_vp_2_dn3) - ((var_n_1_n_dn3 * var_qi) + (var_n_1_n * var_qi_dn3)));

        let assign1640_e1299: f64 = if p.p22 == 0.0 { 1.0 } else { 0.0 };
        var_guard16 = assign1640_e1299;

        let (assign1650_e1308, assign1650_e1308_d_n0, assign1650_e1308_d_n1, assign1650_e1308_d_n2, assign1650_e1308_d_n3,) = {
    if (var_guard16 != 0.0) {
        let assign1650_e1303: f64 = (var_vp * var_vp);
        let assign1650_e1305: f64 = (assign1650_e1303 + var_vt_vt_2);
        let assign1650_e1306: f64 = (assign1650_e1305).sqrt();
        (assign1650_e1306, (((var_vp_dn0 * var_vp) + (var_vp * var_vp_dn0)) / (2.0 * assign1650_e1306)), (((var_vp_dn1 * var_vp) + (var_vp * var_vp_dn1)) / (2.0 * assign1650_e1306)), (((var_vp_dn2 * var_vp) + (var_vp * var_vp_dn2)) / (2.0 * assign1650_e1306)), (((var_vp_dn3 * var_vp) + (var_vp * var_vp_dn3)) / (2.0 * assign1650_e1306)),)
    } else {
        (var_sqrt_vp_vt, var_sqrt_vp_vt_dn0, var_sqrt_vp_vt_dn1, var_sqrt_vp_vt_dn2, var_sqrt_vp_vt_dn3,)
    }
};
        var_sqrt_vp_vt = assign1650_e1308;
        var_sqrt_vp_vt_dn0 = assign1650_e1308_d_n0;
        var_sqrt_vp_vt_dn1 = assign1650_e1308_d_n1;
        var_sqrt_vp_vt_dn2 = assign1650_e1308_d_n2;
        var_sqrt_vp_vt_dn3 = assign1650_e1308_d_n3;

        let (assign1660_e1316, assign1660_e1316_d_n0, assign1660_e1316_d_n1, assign1660_e1316_d_n2, assign1660_e1316_d_n3,) = {
    if (var_guard16 != 0.0) {
        let assign1660_e1313: f64 = (var_vp + var_sqrt_vp_vt);
        let assign1660_e1314: f64 = (0.5 * assign1660_e1313);
        (assign1660_e1314, (0.5 * (var_vp_dn0 + var_sqrt_vp_vt_dn0)), (0.5 * (var_vp_dn1 + var_sqrt_vp_vt_dn1)), (0.5 * (var_vp_dn2 + var_sqrt_vp_vt_dn2)), (0.5 * (var_vp_dn3 + var_sqrt_vp_vt_dn3)),)
    } else {
        (var_vpprime, var_vpprime_dn0, var_vpprime_dn1, var_vpprime_dn2, var_vpprime_dn3,)
    }
};
        var_vpprime = assign1660_e1316;
        var_vpprime_dn0 = assign1660_e1316_d_n0;
        var_vpprime_dn1 = assign1660_e1316_d_n1;
        var_vpprime_dn2 = assign1660_e1316_d_n2;
        var_vpprime_dn3 = assign1660_e1316_d_n3;

        let (assign1670_e1324, assign1670_e1324_d_n0, assign1670_e1324_d_n1, assign1670_e1324_d_n2, assign1670_e1324_d_n3,) = {
    if (var_guard16 != 0.0) {
        let assign1670_e1321: f64 = (p.p21 * var_vpprime);
        let assign1670_e1322: f64 = (1.0 + assign1670_e1321);
        (assign1670_e1322, (p.p21 * var_vpprime_dn0), (p.p21 * var_vpprime_dn1), (p.p21 * var_vpprime_dn2), (p.p21 * var_vpprime_dn3),)
    } else {
        (var_theta_vp_1, var_theta_vp_1_dn0, var_theta_vp_1_dn1, var_theta_vp_1_dn2, var_theta_vp_1_dn3,)
    }
};
        var_theta_vp_1 = assign1670_e1324;
        var_theta_vp_1_dn0 = assign1670_e1324_d_n0;
        var_theta_vp_1_dn1 = assign1670_e1324_d_n1;
        var_theta_vp_1_dn2 = assign1670_e1324_d_n2;
        var_theta_vp_1_dn3 = assign1670_e1324_d_n3;

        let (assign1680_e1332, assign1680_e1332_d_n0, assign1680_e1332_d_n1, assign1680_e1332_d_n2, assign1680_e1332_d_n3,) = {
    if (var_guard16 != 0.0) {
        let assign1680_e1329: f64 = (var_leq * var_theta_vp_1);
        let assign1680_e1330: f64 = (var_kp_weff / assign1680_e1329);
        (assign1680_e1330, (-((var_kp_weff * ((var_leq_dn0 * var_theta_vp_1) + (var_leq * var_theta_vp_1_dn0))) / (assign1680_e1329 * assign1680_e1329))), (-((var_kp_weff * ((var_leq_dn1 * var_theta_vp_1) + (var_leq * var_theta_vp_1_dn1))) / (assign1680_e1329 * assign1680_e1329))), (-((var_kp_weff * ((var_leq_dn2 * var_theta_vp_1) + (var_leq * var_theta_vp_1_dn2))) / (assign1680_e1329 * assign1680_e1329))), (-((var_kp_weff * ((var_leq_dn3 * var_theta_vp_1) + (var_leq * var_theta_vp_1_dn3))) / (assign1680_e1329 * assign1680_e1329))),)
    } else {
        (var_beta, var_beta_dn0, var_beta_dn1, var_beta_dn2, var_beta_dn3,)
    }
};
        var_beta = assign1680_e1332;
        var_beta_dn0 = assign1680_e1332_d_n0;
        var_beta_dn1 = assign1680_e1332_d_n1;
        var_beta_dn2 = assign1680_e1332_d_n2;
        var_beta_dn3 = assign1680_e1332_d_n3;

        let assign1690_e1336: f64 = (var_eta_qi * var_qi);
        let assign1690_e1337: f64 = (var_qb + assign1690_e1336);
        let assign1690_e1339: f64 = if assign1690_e1337 > 0.0 { 1.0 } else { 0.0 };
        var_guard17 = assign1690_e1339;

        let (assign1700_e1354, assign1700_e1354_d_n0, assign1700_e1354_d_n1, assign1700_e1354_d_n2, assign1700_e1354_d_n3,) = {
    if ((var_guard16 == 0.0) && (var_guard17 != 0.0)) {
        let assign1700_e1349: f64 = (var_eta_qi * var_qi);
        let assign1700_e1350: f64 = (var_qb + assign1700_e1349);
        let assign1700_e1351: f64 = (var_t0 * assign1700_e1350);
        let assign1700_e1352: f64 = (1.0 + assign1700_e1351);
        (assign1700_e1352, (var_t0 * (var_qb_dn0 + (var_eta_qi * var_qi_dn0))), (var_t0 * (var_qb_dn1 + (var_eta_qi * var_qi_dn1))), (var_t0 * (var_qb_dn2 + (var_eta_qi * var_qi_dn2))), (var_t0 * (var_qb_dn3 + (var_eta_qi * var_qi_dn3))),)
    } else {
        (var_e0_q_1, var_e0_q_1_dn0, var_e0_q_1_dn1, var_e0_q_1_dn2, var_e0_q_1_dn3,)
    }
};
        var_e0_q_1 = assign1700_e1354;
        var_e0_q_1_dn0 = assign1700_e1354_d_n0;
        var_e0_q_1_dn1 = assign1700_e1354_d_n1;
        var_e0_q_1_dn2 = assign1700_e1354_d_n2;
        var_e0_q_1_dn3 = assign1700_e1354_d_n3;

        let (assign1710_e1370, assign1710_e1370_d_n0, assign1710_e1370_d_n1, assign1710_e1370_d_n2, assign1710_e1370_d_n3,) = {
    if ((var_guard16 == 0.0) && (var_guard17 == 0.0)) {
        let assign1710_e1365: f64 = (var_eta_qi * var_qi);
        let assign1710_e1366: f64 = (var_qb + assign1710_e1365);
        let assign1710_e1367: f64 = (var_t0 * assign1710_e1366);
        let assign1710_e1368: f64 = (1.0 - assign1710_e1367);
        (assign1710_e1368, (-(var_t0 * (var_qb_dn0 + (var_eta_qi * var_qi_dn0)))), (-(var_t0 * (var_qb_dn1 + (var_eta_qi * var_qi_dn1)))), (-(var_t0 * (var_qb_dn2 + (var_eta_qi * var_qi_dn2)))), (-(var_t0 * (var_qb_dn3 + (var_eta_qi * var_qi_dn3)))),)
    } else {
        (var_e0_q_1, var_e0_q_1_dn0, var_e0_q_1_dn1, var_e0_q_1_dn2, var_e0_q_1_dn3,)
    }
};
        var_e0_q_1 = assign1710_e1370;
        var_e0_q_1_dn0 = assign1710_e1370_d_n0;
        var_e0_q_1_dn1 = assign1710_e1370_d_n1;
        var_e0_q_1_dn2 = assign1710_e1370_d_n2;
        var_e0_q_1_dn3 = assign1710_e1370_d_n3;

        let (assign1720_e1379, assign1720_e1379_d_n0, assign1720_e1379_d_n1, assign1720_e1379_d_n2, assign1720_e1379_d_n3,) = {
    if (var_guard16 == 0.0) {
        let assign1720_e1376: f64 = (var_t0 * var_gamma_sqrt_phi);
        let assign1720_e1377: f64 = (1.0 + assign1720_e1376);
        (assign1720_e1377, (var_t0 * var_gamma_sqrt_phi_dn0), (var_t0 * var_gamma_sqrt_phi_dn1), (var_t0 * var_gamma_sqrt_phi_dn2), (var_t0 * var_gamma_sqrt_phi_dn3),)
    } else {
        (var_t0_gamma_1, var_t0_gamma_1_dn0, var_t0_gamma_1_dn1, var_t0_gamma_1_dn2, var_t0_gamma_1_dn3,)
    }
};
        var_t0_gamma_1 = assign1720_e1379;
        var_t0_gamma_1_dn0 = assign1720_e1379_d_n0;
        var_t0_gamma_1_dn1 = assign1720_e1379_d_n1;
        var_t0_gamma_1_dn2 = assign1720_e1379_d_n2;
        var_t0_gamma_1_dn3 = assign1720_e1379_d_n3;

        let (assign1730_e1390, assign1730_e1390_d_n0, assign1730_e1390_d_n1, assign1730_e1390_d_n2, assign1730_e1390_d_n3,) = {
    if (var_guard16 == 0.0) {
        let assign1730_e1384: f64 = (var_kp_weff * var_t0_gamma_1);
        let assign1730_e1387: f64 = (var_leq * var_e0_q_1);
        let assign1730_e1388: f64 = (assign1730_e1384 / assign1730_e1387);
        (assign1730_e1388, ((((var_kp_weff * var_t0_gamma_1_dn0) * assign1730_e1387) - (assign1730_e1384 * ((var_leq_dn0 * var_e0_q_1) + (var_leq * var_e0_q_1_dn0)))) / (assign1730_e1387 * assign1730_e1387)), ((((var_kp_weff * var_t0_gamma_1_dn1) * assign1730_e1387) - (assign1730_e1384 * ((var_leq_dn1 * var_e0_q_1) + (var_leq * var_e0_q_1_dn1)))) / (assign1730_e1387 * assign1730_e1387)), ((((var_kp_weff * var_t0_gamma_1_dn2) * assign1730_e1387) - (assign1730_e1384 * ((var_leq_dn2 * var_e0_q_1) + (var_leq * var_e0_q_1_dn2)))) / (assign1730_e1387 * assign1730_e1387)), ((((var_kp_weff * var_t0_gamma_1_dn3) * assign1730_e1387) - (assign1730_e1384 * ((var_leq_dn3 * var_e0_q_1) + (var_leq * var_e0_q_1_dn3)))) / (assign1730_e1387 * assign1730_e1387)),)
    } else {
        (var_beta, var_beta_dn0, var_beta_dn1, var_beta_dn2, var_beta_dn3,)
    }
};
        var_beta = assign1730_e1390;
        var_beta_dn0 = assign1730_e1390_d_n0;
        var_beta_dn1 = assign1730_e1390_d_n1;
        var_beta_dn2 = assign1730_e1390_d_n2;
        var_beta_dn3 = assign1730_e1390_d_n3;

        let assign1740_e1393: f64 = (var_phi_t + var_vp);
        let assign1740_e1395: f64 = (assign1740_e1393 + var_vt_4);
        let assign1740_e1396: f64 = (assign1740_e1395).sqrt();
        var_sqrt_phi_vp = assign1740_e1396;
        var_sqrt_phi_vp_dn0 = ((var_phi_t_dn0 + var_vp_dn0) / (2.0 * assign1740_e1396));
        var_sqrt_phi_vp_dn1 = ((var_phi_t_dn1 + var_vp_dn1) / (2.0 * assign1740_e1396));
        var_sqrt_phi_vp_dn2 = ((var_phi_t_dn2 + var_vp_dn2) / (2.0 * assign1740_e1396));
        var_sqrt_phi_vp_dn3 = ((var_phi_t_dn3 + var_vp_dn3) / (2.0 * assign1740_e1396));

        *var_beta_slot = var_beta;
        *var_beta_dn0_slot = var_beta_dn0;
        *var_beta_dn1_slot = var_beta_dn1;
        *var_beta_dn2_slot = var_beta_dn2;
        *var_beta_dn3_slot = var_beta_dn3;
        *var_deltal_slot = var_deltal;
        *var_deltal_dn0_slot = var_deltal_dn0;
        *var_deltal_dn1_slot = var_deltal_dn1;
        *var_deltal_dn2_slot = var_deltal_dn2;
        *var_deltal_dn3_slot = var_deltal_dn3;
        *var_dir_dv_slot = var_dir_dv;
        *var_dir_dv_dn0_slot = var_dir_dv_dn0;
        *var_dir_dv_dn1_slot = var_dir_dv_dn1;
        *var_dir_dv_dn2_slot = var_dir_dv_dn2;
        *var_dir_dv_dn3_slot = var_dir_dv_dn3;
        *var_dirprime_dv_slot = var_dirprime_dv;
        *var_dirprime_dv_dn0_slot = var_dirprime_dv_dn0;
        *var_dirprime_dv_dn1_slot = var_dirprime_dv_dn1;
        *var_dirprime_dv_dn2_slot = var_dirprime_dv_dn2;
        *var_dirprime_dv_dn3_slot = var_dirprime_dv_dn3;
        *var_e0_q_1_slot = var_e0_q_1;
        *var_e0_q_1_dn0_slot = var_e0_q_1_dn0;
        *var_e0_q_1_dn1_slot = var_e0_q_1_dn1;
        *var_e0_q_1_dn2_slot = var_e0_q_1_dn2;
        *var_e0_q_1_dn3_slot = var_e0_q_1_dn3;
        *var_guard12_slot = var_guard12;
        *var_guard13_slot = var_guard13;
        *var_guard14_slot = var_guard14;
        *var_guard15_slot = var_guard15;
        *var_guard16_slot = var_guard16;
        *var_guard17_slot = var_guard17;
        *var_ir_slot = var_ir;
        *var_ir_dn0_slot = var_ir_dn0;
        *var_ir_dn1_slot = var_ir_dn1;
        *var_ir_dn2_slot = var_ir_dn2;
        *var_ir_dn3_slot = var_ir_dn3;
        *var_irprime_slot = var_irprime;
        *var_irprime_dn0_slot = var_irprime_dn0;
        *var_irprime_dn1_slot = var_irprime_dn1;
        *var_irprime_dn2_slot = var_irprime_dn2;
        *var_irprime_dn3_slot = var_irprime_dn3;
        *var_leq_slot = var_leq;
        *var_leq_dn0_slot = var_leq_dn0;
        *var_leq_dn1_slot = var_leq_dn1;
        *var_leq_dn2_slot = var_leq_dn2;
        *var_leq_dn3_slot = var_leq_dn3;
        *var_lmin_slot = var_lmin;
        *var_lprime_slot = var_lprime;
        *var_lprime_dn0_slot = var_lprime_dn0;
        *var_lprime_dn1_slot = var_lprime_dn1;
        *var_lprime_dn2_slot = var_lprime_dn2;
        *var_lprime_dn3_slot = var_lprime_dn3;
        *var_n_1_slot = var_n_1;
        *var_n_1_dn0_slot = var_n_1_dn0;
        *var_n_1_dn1_slot = var_n_1_dn1;
        *var_n_1_dn2_slot = var_n_1_dn2;
        *var_n_1_dn3_slot = var_n_1_dn3;
        *var_n_1_n_slot = var_n_1_n;
        *var_n_1_n_dn0_slot = var_n_1_n_dn0;
        *var_n_1_n_dn1_slot = var_n_1_n_dn1;
        *var_n_1_n_dn2_slot = var_n_1_n_dn2;
        *var_n_1_n_dn3_slot = var_n_1_n_dn3;
        *var_qb_slot = var_qb;
        *var_qb_dn0_slot = var_qb_dn0;
        *var_qb_dn1_slot = var_qb_dn1;
        *var_qb_dn2_slot = var_qb_dn2;
        *var_qb_dn3_slot = var_qb_dn3;
        *var_qi_slot = var_qi;
        *var_qi_dn0_slot = var_qi_dn0;
        *var_qi_dn1_slot = var_qi_dn1;
        *var_qi_dn2_slot = var_qi_dn2;
        *var_qi_dn3_slot = var_qi_dn3;
        *var_sif_slot = var_sif;
        *var_sif2_slot = var_sif2;
        *var_sif2_dn0_slot = var_sif2_dn0;
        *var_sif2_dn1_slot = var_sif2_dn1;
        *var_sif2_dn2_slot = var_sif2_dn2;
        *var_sif2_dn3_slot = var_sif2_dn3;
        *var_sif_dn0_slot = var_sif_dn0;
        *var_sif_dn1_slot = var_sif_dn1;
        *var_sif_dn2_slot = var_sif_dn2;
        *var_sif_dn3_slot = var_sif_dn3;
        *var_sif_sir_2_slot = var_sif_sir_2;
        *var_sif_sir_2_dn0_slot = var_sif_sir_2_dn0;
        *var_sif_sir_2_dn1_slot = var_sif_sir_2_dn1;
        *var_sif_sir_2_dn2_slot = var_sif_sir_2_dn2;
        *var_sif_sir_2_dn3_slot = var_sif_sir_2_dn3;
        *var_sir_slot = var_sir;
        *var_sir2_slot = var_sir2;
        *var_sir2_dn0_slot = var_sir2_dn0;
        *var_sir2_dn1_slot = var_sir2_dn1;
        *var_sir2_dn2_slot = var_sir2_dn2;
        *var_sir2_dn3_slot = var_sir2_dn3;
        *var_sir_dn0_slot = var_sir_dn0;
        *var_sir_dn1_slot = var_sir_dn1;
        *var_sir_dn2_slot = var_sir_dn2;
        *var_sir_dn3_slot = var_sir_dn3;
        *var_sqrt_lprime_lmin_slot = var_sqrt_lprime_lmin;
        *var_sqrt_lprime_lmin_dn0_slot = var_sqrt_lprime_lmin_dn0;
        *var_sqrt_lprime_lmin_dn1_slot = var_sqrt_lprime_lmin_dn1;
        *var_sqrt_lprime_lmin_dn2_slot = var_sqrt_lprime_lmin_dn2;
        *var_sqrt_lprime_lmin_dn3_slot = var_sqrt_lprime_lmin_dn3;
        *var_sqrt_phi_vp_slot = var_sqrt_phi_vp;
        *var_sqrt_phi_vp_2_slot = var_sqrt_phi_vp_2;
        *var_sqrt_phi_vp_2_dn0_slot = var_sqrt_phi_vp_2_dn0;
        *var_sqrt_phi_vp_2_dn1_slot = var_sqrt_phi_vp_2_dn1;
        *var_sqrt_phi_vp_2_dn2_slot = var_sqrt_phi_vp_2_dn2;
        *var_sqrt_phi_vp_2_dn3_slot = var_sqrt_phi_vp_2_dn3;
        *var_sqrt_phi_vp_dn0_slot = var_sqrt_phi_vp_dn0;
        *var_sqrt_phi_vp_dn1_slot = var_sqrt_phi_vp_dn1;
        *var_sqrt_phi_vp_dn2_slot = var_sqrt_phi_vp_dn2;
        *var_sqrt_phi_vp_dn3_slot = var_sqrt_phi_vp_dn3;
        *var_sqrt_vp_vt_slot = var_sqrt_vp_vt;
        *var_sqrt_vp_vt_dn0_slot = var_sqrt_vp_vt_dn0;
        *var_sqrt_vp_vt_dn1_slot = var_sqrt_vp_vt_dn1;
        *var_sqrt_vp_vt_dn2_slot = var_sqrt_vp_vt_dn2;
        *var_sqrt_vp_vt_dn3_slot = var_sqrt_vp_vt_dn3;
        *var_t0_gamma_1_slot = var_t0_gamma_1;
        *var_t0_gamma_1_dn0_slot = var_t0_gamma_1_dn0;
        *var_t0_gamma_1_dn1_slot = var_t0_gamma_1_dn1;
        *var_t0_gamma_1_dn2_slot = var_t0_gamma_1_dn2;
        *var_t0_gamma_1_dn3_slot = var_t0_gamma_1_dn3;
        *var_theta_vp_1_slot = var_theta_vp_1;
        *var_theta_vp_1_dn0_slot = var_theta_vp_1_dn0;
        *var_theta_vp_1_dn1_slot = var_theta_vp_1_dn1;
        *var_theta_vp_1_dn2_slot = var_theta_vp_1_dn2;
        *var_theta_vp_1_dn3_slot = var_theta_vp_1_dn3;
        *var_tmp1_slot = var_tmp1;
        *var_tmp1_dn0_slot = var_tmp1_dn0;
        *var_tmp1_dn1_slot = var_tmp1_dn1;
        *var_tmp1_dn2_slot = var_tmp1_dn2;
        *var_tmp1_dn3_slot = var_tmp1_dn3;
        *var_vp_phi_eps_slot = var_vp_phi_eps;
        *var_vp_phi_eps_dn0_slot = var_vp_phi_eps_dn0;
        *var_vp_phi_eps_dn1_slot = var_vp_phi_eps_dn1;
        *var_vp_phi_eps_dn2_slot = var_vp_phi_eps_dn2;
        *var_vp_phi_eps_dn3_slot = var_vp_phi_eps_dn3;
        *var_vpprime_slot = var_vpprime;
        *var_vpprime_dn0_slot = var_vpprime_dn0;
        *var_vpprime_dn1_slot = var_vpprime_dn1;
        *var_vpprime_dn2_slot = var_vpprime_dn2;
        *var_vpprime_dn3_slot = var_vpprime_dn3;
        *var_yk_slot = var_yk;
        *var_yk_dn0_slot = var_yk_dn0;
        *var_yk_dn1_slot = var_yk_dn1;
        *var_yk_dn2_slot = var_yk_dn2;
        *var_yk_dn3_slot = var_yk_dn3;
        *var_z0_slot = var_z0;
        *var_z0_dn0_slot = var_z0_dn0;
        *var_z0_dn1_slot = var_z0_dn1;
        *var_z0_dn2_slot = var_z0_dn2;
        *var_z0_dn3_slot = var_z0_dn3;
        *var_zk_slot = var_zk;
        *var_zk_dn0_slot = var_zk_dn0;
        *var_zk_dn1_slot = var_zk_dn1;
        *var_zk_dn2_slot = var_zk_dn2;
        *var_zk_dn3_slot = var_zk_dn3;
    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn0: f64,
        var_beta_dn1: f64,
        var_beta_dn2: f64,
        var_beta_dn3: f64,
        var_big_sqrt_vp: f64,
        var_big_sqrt_vp_dn0: f64,
        var_big_sqrt_vp_dn1: f64,
        var_big_sqrt_vp_dn2: f64,
        var_big_sqrt_vp_dn3: f64,
        var_dif_dv: f64,
        var_dif_dv_dn0: f64,
        var_dif_dv_dn1: f64,
        var_dif_dv_dn2: f64,
        var_dif_dv_dn3: f64,
        var_dir_dv: f64,
        var_dir_dv_dn0: f64,
        var_dir_dv_dn1: f64,
        var_dir_dv_dn2: f64,
        var_dir_dv_dn3: f64,
        var_dirprime_dv: f64,
        var_dirprime_dv_dn0: f64,
        var_dirprime_dv_dn1: f64,
        var_dirprime_dv_dn2: f64,
        var_dirprime_dv_dn3: f64,
        var_gamma_s: f64,
        var_gammaprime: f64,
        var_gammaprime_dn0: f64,
        var_gammaprime_dn1: f64,
        var_gammaprime_dn2: f64,
        var_gammaprime_dn3: f64,
        var_if_: f64,
        var_if__dn0: f64,
        var_if__dn1: f64,
        var_if__dn2: f64,
        var_if__dn3: f64,
        var_inv_ucrit: f64,
        var_inv_vt: f64,
        var_irprime: f64,
        var_irprime_dn0: f64,
        var_irprime_dn1: f64,
        var_irprime_dn2: f64,
        var_irprime_dn3: f64,
        var_lc_lambda: f64,
        var_lc_ucrit: f64,
        var_leta_l: f64,
        var_n_1: f64,
        var_n_1_dn0: f64,
        var_n_1_dn1: f64,
        var_n_1_dn2: f64,
        var_n_1_dn3: f64,
        var_n_1_n: f64,
        var_n_1_n_dn0: f64,
        var_n_1_n_dn1: f64,
        var_n_1_n_dn2: f64,
        var_n_1_n_dn3: f64,
        var_phi_t: f64,
        var_phi_t_dn0: f64,
        var_phi_t_dn1: f64,
        var_phi_t_dn2: f64,
        var_phi_t_dn3: f64,
        var_qi: f64,
        var_qi_dn0: f64,
        var_qi_dn1: f64,
        var_qi_dn2: f64,
        var_qi_dn3: f64,
        var_sif: f64,
        var_sif_dn0: f64,
        var_sif_dn1: f64,
        var_sif_dn2: f64,
        var_sif_dn3: f64,
        var_sif_sir_2: f64,
        var_sif_sir_2_dn0: f64,
        var_sif_sir_2_dn1: f64,
        var_sif_sir_2_dn2: f64,
        var_sif_sir_2_dn3: f64,
        var_sir: f64,
        var_sir_dn0: f64,
        var_sir_dn1: f64,
        var_sir_dn2: f64,
        var_sir_dn3: f64,
        var_sqrt_gammastar: f64,
        var_sqrt_gammastar_dn0: f64,
        var_sqrt_gammastar_dn1: f64,
        var_sqrt_gammastar_dn2: f64,
        var_sqrt_gammastar_dn3: f64,
        var_sqrt_if: f64,
        var_sqrt_if_dn0: f64,
        var_sqrt_if_dn1: f64,
        var_sqrt_if_dn2: f64,
        var_sqrt_if_dn3: f64,
        var_sqrt_lprime_lmin: f64,
        var_sqrt_lprime_lmin_dn0: f64,
        var_sqrt_lprime_lmin_dn1: f64,
        var_sqrt_lprime_lmin_dn2: f64,
        var_sqrt_lprime_lmin_dn3: f64,
        var_sqrt_phi_vd: f64,
        var_sqrt_phi_vd_dn0: f64,
        var_sqrt_phi_vd_dn1: f64,
        var_sqrt_phi_vd_dn2: f64,
        var_sqrt_phi_vd_dn3: f64,
        var_sqrt_phi_vd_vt: f64,
        var_sqrt_phi_vd_vt_dn0: f64,
        var_sqrt_phi_vd_vt_dn1: f64,
        var_sqrt_phi_vd_vt_dn2: f64,
        var_sqrt_phi_vd_vt_dn3: f64,
        var_sqrt_phi_vp: f64,
        var_sqrt_phi_vp_dn0: f64,
        var_sqrt_phi_vp_dn1: f64,
        var_sqrt_phi_vp_dn2: f64,
        var_sqrt_phi_vp_dn3: f64,
        var_sqrt_phi_vs: f64,
        var_sqrt_phi_vs_dn0: f64,
        var_sqrt_phi_vs_dn1: f64,
        var_sqrt_phi_vs_dn2: f64,
        var_sqrt_phi_vs_dn3: f64,
        var_sqrt_phi_vs_vt: f64,
        var_sqrt_phi_vs_vt_dn0: f64,
        var_sqrt_phi_vs_vt_dn1: f64,
        var_sqrt_phi_vs_vt_dn2: f64,
        var_sqrt_phi_vs_vt_dn3: f64,
        var_sqrt_vds_vdss_deltav: f64,
        var_sqrt_vds_vdss_deltav_dn0: f64,
        var_sqrt_vds_vdss_deltav_dn1: f64,
        var_sqrt_vds_vdss_deltav_dn2: f64,
        var_sqrt_vds_vdss_deltav_dn3: f64,
        var_sqrt_vds_vdssprime_deltav: f64,
        var_sqrt_vds_vdssprime_deltav_dn0: f64,
        var_sqrt_vds_vdssprime_deltav_dn1: f64,
        var_sqrt_vds_vdssprime_deltav_dn2: f64,
        var_sqrt_vds_vdssprime_deltav_dn3: f64,
        var_sqrt_vdss_deltav: f64,
        var_sqrt_vdss_deltav_dn0: f64,
        var_sqrt_vdss_deltav_dn1: f64,
        var_sqrt_vdss_deltav_dn2: f64,
        var_sqrt_vdss_deltav_dn3: f64,
        var_sqrt_vdssprime_deltav: f64,
        var_sqrt_vdssprime_deltav_dn0: f64,
        var_sqrt_vdssprime_deltav_dn1: f64,
        var_sqrt_vdssprime_deltav_dn2: f64,
        var_sqrt_vdssprime_deltav_dn3: f64,
        var_sqrt_vgstar: f64,
        var_sqrt_vgstar_dn0: f64,
        var_sqrt_vgstar_dn1: f64,
        var_sqrt_vgstar_dn2: f64,
        var_sqrt_vgstar_dn3: f64,
        var_sqrt_vp_vt: f64,
        var_sqrt_vp_vt_dn0: f64,
        var_sqrt_vp_vt_dn1: f64,
        var_sqrt_vp_vt_dn2: f64,
        var_sqrt_vp_vt_dn3: f64,
        var_theta_vp_1: f64,
        var_theta_vp_1_dn0: f64,
        var_theta_vp_1_dn1: f64,
        var_theta_vp_1_dn2: f64,
        var_theta_vp_1_dn3: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn2: f64,
        var_vds_dn3: f64,
        var_vdsprime: f64,
        var_vdsprime_dn0: f64,
        var_vdsprime_dn1: f64,
        var_vdsprime_dn2: f64,
        var_vdsprime_dn3: f64,
        var_vdss: f64,
        var_vdss_dn0: f64,
        var_vdss_dn1: f64,
        var_vdss_dn2: f64,
        var_vdss_dn3: f64,
        var_vdss_sqrt: f64,
        var_vdss_sqrt_dn0: f64,
        var_vdss_sqrt_dn1: f64,
        var_vdss_sqrt_dn2: f64,
        var_vdss_sqrt_dn3: f64,
        var_vdssprime: f64,
        var_vdssprime_dn0: f64,
        var_vdssprime_dn1: f64,
        var_vdssprime_dn2: f64,
        var_vdssprime_dn3: f64,
        var_vdssprime_sqrt: f64,
        var_vdssprime_sqrt_dn0: f64,
        var_vdssprime_sqrt_dn1: f64,
        var_vdssprime_sqrt_dn2: f64,
        var_vdssprime_sqrt_dn3: f64,
        var_vgprime: f64,
        var_vgprime_dn0: f64,
        var_vgprime_dn1: f64,
        var_vgprime_dn2: f64,
        var_vgprime_dn3: f64,
        var_vip: f64,
        var_vip_dn0: f64,
        var_vip_dn1: f64,
        var_vip_dn2: f64,
        var_vip_dn3: f64,
        var_vp: f64,
        var_vp_dn0: f64,
        var_vp_dn1: f64,
        var_vp_dn2: f64,
        var_vp_dn3: f64,
        var_vp_phi_eps: f64,
        var_vp_phi_eps_dn0: f64,
        var_vp_phi_eps_dn1: f64,
        var_vp_phi_eps_dn2: f64,
        var_vp_phi_eps_dn3: f64,
        var_vpprime: f64,
        var_vpprime_dn0: f64,
        var_vpprime_dn1: f64,
        var_vpprime_dn2: f64,
        var_vpprime_dn3: f64,
        var_vt: f64,
        var_vt_4: f64,
        var_vt_vt_2: f64,
        var_ddeltal_dvd_slot: &mut f64,
        var_ddeltal_dvd_dn0_slot: &mut f64,
        var_ddeltal_dvd_dn1_slot: &mut f64,
        var_ddeltal_dvd_dn2_slot: &mut f64,
        var_ddeltal_dvd_dn3_slot: &mut f64,
        var_ddeltal_dvs_slot: &mut f64,
        var_ddeltal_dvs_dn0_slot: &mut f64,
        var_ddeltal_dvs_dn1_slot: &mut f64,
        var_ddeltal_dvs_dn2_slot: &mut f64,
        var_ddeltal_dvs_dn3_slot: &mut f64,
        var_ddeltav_dvd_slot: &mut f64,
        var_ddeltav_dvd_dn0_slot: &mut f64,
        var_ddeltav_dvd_dn1_slot: &mut f64,
        var_ddeltav_dvd_dn2_slot: &mut f64,
        var_ddeltav_dvd_dn3_slot: &mut f64,
        var_ddeltav_dvs_slot: &mut f64,
        var_ddeltav_dvs_dn0_slot: &mut f64,
        var_ddeltav_dvs_dn1_slot: &mut f64,
        var_ddeltav_dvs_dn2_slot: &mut f64,
        var_ddeltav_dvs_dn3_slot: &mut f64,
        var_dgammaprime_dvd_slot: &mut f64,
        var_dgammaprime_dvd_dn0_slot: &mut f64,
        var_dgammaprime_dvd_dn1_slot: &mut f64,
        var_dgammaprime_dvd_dn2_slot: &mut f64,
        var_dgammaprime_dvd_dn3_slot: &mut f64,
        var_dgammaprime_dvs_slot: &mut f64,
        var_dgammaprime_dvs_dn0_slot: &mut f64,
        var_dgammaprime_dvs_dn1_slot: &mut f64,
        var_dgammaprime_dvs_dn2_slot: &mut f64,
        var_dgammaprime_dvs_dn3_slot: &mut f64,
        var_dif_dvd_slot: &mut f64,
        var_dif_dvd_dn0_slot: &mut f64,
        var_dif_dvd_dn1_slot: &mut f64,
        var_dif_dvd_dn2_slot: &mut f64,
        var_dif_dvd_dn3_slot: &mut f64,
        var_dif_dvs_slot: &mut f64,
        var_dif_dvs_dn0_slot: &mut f64,
        var_dif_dvs_dn1_slot: &mut f64,
        var_dif_dvs_dn2_slot: &mut f64,
        var_dif_dvs_dn3_slot: &mut f64,
        var_dir_dvd_slot: &mut f64,
        var_dir_dvd_dn0_slot: &mut f64,
        var_dir_dvd_dn1_slot: &mut f64,
        var_dir_dvd_dn2_slot: &mut f64,
        var_dir_dvd_dn3_slot: &mut f64,
        var_dir_dvs_slot: &mut f64,
        var_dir_dvs_dn0_slot: &mut f64,
        var_dir_dvs_dn1_slot: &mut f64,
        var_dir_dvs_dn2_slot: &mut f64,
        var_dir_dvs_dn3_slot: &mut f64,
        var_dirprime_dvd_slot: &mut f64,
        var_dirprime_dvd_dn0_slot: &mut f64,
        var_dirprime_dvd_dn1_slot: &mut f64,
        var_dirprime_dvd_dn2_slot: &mut f64,
        var_dirprime_dvd_dn3_slot: &mut f64,
        var_dirprime_dvs_slot: &mut f64,
        var_dirprime_dvs_dn0_slot: &mut f64,
        var_dirprime_dvs_dn1_slot: &mut f64,
        var_dirprime_dvs_dn2_slot: &mut f64,
        var_dirprime_dvs_dn3_slot: &mut f64,
        var_dleq_dvd_slot: &mut f64,
        var_dleq_dvd_dn0_slot: &mut f64,
        var_dleq_dvd_dn1_slot: &mut f64,
        var_dleq_dvd_dn2_slot: &mut f64,
        var_dleq_dvd_dn3_slot: &mut f64,
        var_dleq_dvs_slot: &mut f64,
        var_dleq_dvs_dn0_slot: &mut f64,
        var_dleq_dvs_dn1_slot: &mut f64,
        var_dleq_dvs_dn2_slot: &mut f64,
        var_dleq_dvs_dn3_slot: &mut f64,
        var_dqb_dvd_slot: &mut f64,
        var_dqb_dvd_dn0_slot: &mut f64,
        var_dqb_dvd_dn1_slot: &mut f64,
        var_dqb_dvd_dn2_slot: &mut f64,
        var_dqb_dvd_dn3_slot: &mut f64,
        var_dqb_dvs_slot: &mut f64,
        var_dqb_dvs_dn0_slot: &mut f64,
        var_dqb_dvs_dn1_slot: &mut f64,
        var_dqb_dvs_dn2_slot: &mut f64,
        var_dqb_dvs_dn3_slot: &mut f64,
        var_dqi_dvd_slot: &mut f64,
        var_dqi_dvd_dn0_slot: &mut f64,
        var_dqi_dvd_dn1_slot: &mut f64,
        var_dqi_dvd_dn2_slot: &mut f64,
        var_dqi_dvd_dn3_slot: &mut f64,
        var_dqi_dvs_slot: &mut f64,
        var_dqi_dvs_dn0_slot: &mut f64,
        var_dqi_dvs_dn1_slot: &mut f64,
        var_dqi_dvs_dn2_slot: &mut f64,
        var_dqi_dvs_dn3_slot: &mut f64,
        var_dvdss_dvd_slot: &mut f64,
        var_dvdss_dvd_dn0_slot: &mut f64,
        var_dvdss_dvd_dn1_slot: &mut f64,
        var_dvdss_dvd_dn2_slot: &mut f64,
        var_dvdss_dvd_dn3_slot: &mut f64,
        var_dvdss_dvs_slot: &mut f64,
        var_dvdss_dvs_dn0_slot: &mut f64,
        var_dvdss_dvs_dn1_slot: &mut f64,
        var_dvdss_dvs_dn2_slot: &mut f64,
        var_dvdss_dvs_dn3_slot: &mut f64,
        var_dvdssprime_dvd_slot: &mut f64,
        var_dvdssprime_dvd_dn0_slot: &mut f64,
        var_dvdssprime_dvd_dn1_slot: &mut f64,
        var_dvdssprime_dvd_dn2_slot: &mut f64,
        var_dvdssprime_dvd_dn3_slot: &mut f64,
        var_dvdssprime_dvs_slot: &mut f64,
        var_dvdssprime_dvs_dn0_slot: &mut f64,
        var_dvdssprime_dvs_dn1_slot: &mut f64,
        var_dvdssprime_dvs_dn2_slot: &mut f64,
        var_dvdssprime_dvs_dn3_slot: &mut f64,
        var_dvip_dvd_slot: &mut f64,
        var_dvip_dvd_dn0_slot: &mut f64,
        var_dvip_dvd_dn1_slot: &mut f64,
        var_dvip_dvd_dn2_slot: &mut f64,
        var_dvip_dvd_dn3_slot: &mut f64,
        var_dvip_dvs_slot: &mut f64,
        var_dvip_dvs_dn0_slot: &mut f64,
        var_dvip_dvs_dn1_slot: &mut f64,
        var_dvip_dvs_dn2_slot: &mut f64,
        var_dvip_dvs_dn3_slot: &mut f64,
        var_dvp_dvd_slot: &mut f64,
        var_dvp_dvd_dn0_slot: &mut f64,
        var_dvp_dvd_dn1_slot: &mut f64,
        var_dvp_dvd_dn2_slot: &mut f64,
        var_dvp_dvd_dn3_slot: &mut f64,
        var_dvp_dvs_slot: &mut f64,
        var_dvp_dvs_dn0_slot: &mut f64,
        var_dvp_dvs_dn1_slot: &mut f64,
        var_dvp_dvs_dn2_slot: &mut f64,
        var_dvp_dvs_dn3_slot: &mut f64,
        var_dvpprime_dvd_slot: &mut f64,
        var_dvpprime_dvd_dn0_slot: &mut f64,
        var_dvpprime_dvd_dn1_slot: &mut f64,
        var_dvpprime_dvd_dn2_slot: &mut f64,
        var_dvpprime_dvd_dn3_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_if_ir_slot: &mut f64,
        var_if_ir_dn0_slot: &mut f64,
        var_if_ir_dn1_slot: &mut f64,
        var_if_ir_dn2_slot: &mut f64,
        var_if_ir_dn3_slot: &mut f64,
        var_ispec_slot: &mut f64,
        var_ispec_dn0_slot: &mut f64,
        var_ispec_dn1_slot: &mut f64,
        var_ispec_dn2_slot: &mut f64,
        var_ispec_dn3_slot: &mut f64,
        var_n_slot: &mut f64,
        var_n_dn0_slot: &mut f64,
        var_n_dn1_slot: &mut f64,
        var_n_dn2_slot: &mut f64,
        var_n_dn3_slot: &mut f64,
        var_tmp1_slot: &mut f64,
        var_tmp1_dn0_slot: &mut f64,
        var_tmp1_dn1_slot: &mut f64,
        var_tmp1_dn2_slot: &mut f64,
        var_tmp1_dn3_slot: &mut f64,
        var_tmp2_slot: &mut f64,
        var_tmp2_dn0_slot: &mut f64,
        var_tmp2_dn1_slot: &mut f64,
        var_tmp2_dn2_slot: &mut f64,
        var_tmp2_dn3_slot: &mut f64,
        var_tmp3_slot: &mut f64,
        var_tmp3_dn0_slot: &mut f64,
        var_tmp3_dn1_slot: &mut f64,
        var_tmp3_dn2_slot: &mut f64,
        var_tmp3_dn3_slot: &mut f64,
    ) {
        let mut var_ddeltal_dvd: f64 = *var_ddeltal_dvd_slot;
        let mut var_ddeltal_dvd_dn0: f64 = *var_ddeltal_dvd_dn0_slot;
        let mut var_ddeltal_dvd_dn1: f64 = *var_ddeltal_dvd_dn1_slot;
        let mut var_ddeltal_dvd_dn2: f64 = *var_ddeltal_dvd_dn2_slot;
        let mut var_ddeltal_dvd_dn3: f64 = *var_ddeltal_dvd_dn3_slot;
        let mut var_ddeltal_dvs: f64 = *var_ddeltal_dvs_slot;
        let mut var_ddeltal_dvs_dn0: f64 = *var_ddeltal_dvs_dn0_slot;
        let mut var_ddeltal_dvs_dn1: f64 = *var_ddeltal_dvs_dn1_slot;
        let mut var_ddeltal_dvs_dn2: f64 = *var_ddeltal_dvs_dn2_slot;
        let mut var_ddeltal_dvs_dn3: f64 = *var_ddeltal_dvs_dn3_slot;
        let mut var_ddeltav_dvd: f64 = *var_ddeltav_dvd_slot;
        let mut var_ddeltav_dvd_dn0: f64 = *var_ddeltav_dvd_dn0_slot;
        let mut var_ddeltav_dvd_dn1: f64 = *var_ddeltav_dvd_dn1_slot;
        let mut var_ddeltav_dvd_dn2: f64 = *var_ddeltav_dvd_dn2_slot;
        let mut var_ddeltav_dvd_dn3: f64 = *var_ddeltav_dvd_dn3_slot;
        let mut var_ddeltav_dvs: f64 = *var_ddeltav_dvs_slot;
        let mut var_ddeltav_dvs_dn0: f64 = *var_ddeltav_dvs_dn0_slot;
        let mut var_ddeltav_dvs_dn1: f64 = *var_ddeltav_dvs_dn1_slot;
        let mut var_ddeltav_dvs_dn2: f64 = *var_ddeltav_dvs_dn2_slot;
        let mut var_ddeltav_dvs_dn3: f64 = *var_ddeltav_dvs_dn3_slot;
        let mut var_dgammaprime_dvd: f64 = *var_dgammaprime_dvd_slot;
        let mut var_dgammaprime_dvd_dn0: f64 = *var_dgammaprime_dvd_dn0_slot;
        let mut var_dgammaprime_dvd_dn1: f64 = *var_dgammaprime_dvd_dn1_slot;
        let mut var_dgammaprime_dvd_dn2: f64 = *var_dgammaprime_dvd_dn2_slot;
        let mut var_dgammaprime_dvd_dn3: f64 = *var_dgammaprime_dvd_dn3_slot;
        let mut var_dgammaprime_dvs: f64 = *var_dgammaprime_dvs_slot;
        let mut var_dgammaprime_dvs_dn0: f64 = *var_dgammaprime_dvs_dn0_slot;
        let mut var_dgammaprime_dvs_dn1: f64 = *var_dgammaprime_dvs_dn1_slot;
        let mut var_dgammaprime_dvs_dn2: f64 = *var_dgammaprime_dvs_dn2_slot;
        let mut var_dgammaprime_dvs_dn3: f64 = *var_dgammaprime_dvs_dn3_slot;
        let mut var_dif_dvd: f64 = *var_dif_dvd_slot;
        let mut var_dif_dvd_dn0: f64 = *var_dif_dvd_dn0_slot;
        let mut var_dif_dvd_dn1: f64 = *var_dif_dvd_dn1_slot;
        let mut var_dif_dvd_dn2: f64 = *var_dif_dvd_dn2_slot;
        let mut var_dif_dvd_dn3: f64 = *var_dif_dvd_dn3_slot;
        let mut var_dif_dvs: f64 = *var_dif_dvs_slot;
        let mut var_dif_dvs_dn0: f64 = *var_dif_dvs_dn0_slot;
        let mut var_dif_dvs_dn1: f64 = *var_dif_dvs_dn1_slot;
        let mut var_dif_dvs_dn2: f64 = *var_dif_dvs_dn2_slot;
        let mut var_dif_dvs_dn3: f64 = *var_dif_dvs_dn3_slot;
        let mut var_dir_dvd: f64 = *var_dir_dvd_slot;
        let mut var_dir_dvd_dn0: f64 = *var_dir_dvd_dn0_slot;
        let mut var_dir_dvd_dn1: f64 = *var_dir_dvd_dn1_slot;
        let mut var_dir_dvd_dn2: f64 = *var_dir_dvd_dn2_slot;
        let mut var_dir_dvd_dn3: f64 = *var_dir_dvd_dn3_slot;
        let mut var_dir_dvs: f64 = *var_dir_dvs_slot;
        let mut var_dir_dvs_dn0: f64 = *var_dir_dvs_dn0_slot;
        let mut var_dir_dvs_dn1: f64 = *var_dir_dvs_dn1_slot;
        let mut var_dir_dvs_dn2: f64 = *var_dir_dvs_dn2_slot;
        let mut var_dir_dvs_dn3: f64 = *var_dir_dvs_dn3_slot;
        let mut var_dirprime_dvd: f64 = *var_dirprime_dvd_slot;
        let mut var_dirprime_dvd_dn0: f64 = *var_dirprime_dvd_dn0_slot;
        let mut var_dirprime_dvd_dn1: f64 = *var_dirprime_dvd_dn1_slot;
        let mut var_dirprime_dvd_dn2: f64 = *var_dirprime_dvd_dn2_slot;
        let mut var_dirprime_dvd_dn3: f64 = *var_dirprime_dvd_dn3_slot;
        let mut var_dirprime_dvs: f64 = *var_dirprime_dvs_slot;
        let mut var_dirprime_dvs_dn0: f64 = *var_dirprime_dvs_dn0_slot;
        let mut var_dirprime_dvs_dn1: f64 = *var_dirprime_dvs_dn1_slot;
        let mut var_dirprime_dvs_dn2: f64 = *var_dirprime_dvs_dn2_slot;
        let mut var_dirprime_dvs_dn3: f64 = *var_dirprime_dvs_dn3_slot;
        let mut var_dleq_dvd: f64 = *var_dleq_dvd_slot;
        let mut var_dleq_dvd_dn0: f64 = *var_dleq_dvd_dn0_slot;
        let mut var_dleq_dvd_dn1: f64 = *var_dleq_dvd_dn1_slot;
        let mut var_dleq_dvd_dn2: f64 = *var_dleq_dvd_dn2_slot;
        let mut var_dleq_dvd_dn3: f64 = *var_dleq_dvd_dn3_slot;
        let mut var_dleq_dvs: f64 = *var_dleq_dvs_slot;
        let mut var_dleq_dvs_dn0: f64 = *var_dleq_dvs_dn0_slot;
        let mut var_dleq_dvs_dn1: f64 = *var_dleq_dvs_dn1_slot;
        let mut var_dleq_dvs_dn2: f64 = *var_dleq_dvs_dn2_slot;
        let mut var_dleq_dvs_dn3: f64 = *var_dleq_dvs_dn3_slot;
        let mut var_dqb_dvd: f64 = *var_dqb_dvd_slot;
        let mut var_dqb_dvd_dn0: f64 = *var_dqb_dvd_dn0_slot;
        let mut var_dqb_dvd_dn1: f64 = *var_dqb_dvd_dn1_slot;
        let mut var_dqb_dvd_dn2: f64 = *var_dqb_dvd_dn2_slot;
        let mut var_dqb_dvd_dn3: f64 = *var_dqb_dvd_dn3_slot;
        let mut var_dqb_dvs: f64 = *var_dqb_dvs_slot;
        let mut var_dqb_dvs_dn0: f64 = *var_dqb_dvs_dn0_slot;
        let mut var_dqb_dvs_dn1: f64 = *var_dqb_dvs_dn1_slot;
        let mut var_dqb_dvs_dn2: f64 = *var_dqb_dvs_dn2_slot;
        let mut var_dqb_dvs_dn3: f64 = *var_dqb_dvs_dn3_slot;
        let mut var_dqi_dvd: f64 = *var_dqi_dvd_slot;
        let mut var_dqi_dvd_dn0: f64 = *var_dqi_dvd_dn0_slot;
        let mut var_dqi_dvd_dn1: f64 = *var_dqi_dvd_dn1_slot;
        let mut var_dqi_dvd_dn2: f64 = *var_dqi_dvd_dn2_slot;
        let mut var_dqi_dvd_dn3: f64 = *var_dqi_dvd_dn3_slot;
        let mut var_dqi_dvs: f64 = *var_dqi_dvs_slot;
        let mut var_dqi_dvs_dn0: f64 = *var_dqi_dvs_dn0_slot;
        let mut var_dqi_dvs_dn1: f64 = *var_dqi_dvs_dn1_slot;
        let mut var_dqi_dvs_dn2: f64 = *var_dqi_dvs_dn2_slot;
        let mut var_dqi_dvs_dn3: f64 = *var_dqi_dvs_dn3_slot;
        let mut var_dvdss_dvd: f64 = *var_dvdss_dvd_slot;
        let mut var_dvdss_dvd_dn0: f64 = *var_dvdss_dvd_dn0_slot;
        let mut var_dvdss_dvd_dn1: f64 = *var_dvdss_dvd_dn1_slot;
        let mut var_dvdss_dvd_dn2: f64 = *var_dvdss_dvd_dn2_slot;
        let mut var_dvdss_dvd_dn3: f64 = *var_dvdss_dvd_dn3_slot;
        let mut var_dvdss_dvs: f64 = *var_dvdss_dvs_slot;
        let mut var_dvdss_dvs_dn0: f64 = *var_dvdss_dvs_dn0_slot;
        let mut var_dvdss_dvs_dn1: f64 = *var_dvdss_dvs_dn1_slot;
        let mut var_dvdss_dvs_dn2: f64 = *var_dvdss_dvs_dn2_slot;
        let mut var_dvdss_dvs_dn3: f64 = *var_dvdss_dvs_dn3_slot;
        let mut var_dvdssprime_dvd: f64 = *var_dvdssprime_dvd_slot;
        let mut var_dvdssprime_dvd_dn0: f64 = *var_dvdssprime_dvd_dn0_slot;
        let mut var_dvdssprime_dvd_dn1: f64 = *var_dvdssprime_dvd_dn1_slot;
        let mut var_dvdssprime_dvd_dn2: f64 = *var_dvdssprime_dvd_dn2_slot;
        let mut var_dvdssprime_dvd_dn3: f64 = *var_dvdssprime_dvd_dn3_slot;
        let mut var_dvdssprime_dvs: f64 = *var_dvdssprime_dvs_slot;
        let mut var_dvdssprime_dvs_dn0: f64 = *var_dvdssprime_dvs_dn0_slot;
        let mut var_dvdssprime_dvs_dn1: f64 = *var_dvdssprime_dvs_dn1_slot;
        let mut var_dvdssprime_dvs_dn2: f64 = *var_dvdssprime_dvs_dn2_slot;
        let mut var_dvdssprime_dvs_dn3: f64 = *var_dvdssprime_dvs_dn3_slot;
        let mut var_dvip_dvd: f64 = *var_dvip_dvd_slot;
        let mut var_dvip_dvd_dn0: f64 = *var_dvip_dvd_dn0_slot;
        let mut var_dvip_dvd_dn1: f64 = *var_dvip_dvd_dn1_slot;
        let mut var_dvip_dvd_dn2: f64 = *var_dvip_dvd_dn2_slot;
        let mut var_dvip_dvd_dn3: f64 = *var_dvip_dvd_dn3_slot;
        let mut var_dvip_dvs: f64 = *var_dvip_dvs_slot;
        let mut var_dvip_dvs_dn0: f64 = *var_dvip_dvs_dn0_slot;
        let mut var_dvip_dvs_dn1: f64 = *var_dvip_dvs_dn1_slot;
        let mut var_dvip_dvs_dn2: f64 = *var_dvip_dvs_dn2_slot;
        let mut var_dvip_dvs_dn3: f64 = *var_dvip_dvs_dn3_slot;
        let mut var_dvp_dvd: f64 = *var_dvp_dvd_slot;
        let mut var_dvp_dvd_dn0: f64 = *var_dvp_dvd_dn0_slot;
        let mut var_dvp_dvd_dn1: f64 = *var_dvp_dvd_dn1_slot;
        let mut var_dvp_dvd_dn2: f64 = *var_dvp_dvd_dn2_slot;
        let mut var_dvp_dvd_dn3: f64 = *var_dvp_dvd_dn3_slot;
        let mut var_dvp_dvs: f64 = *var_dvp_dvs_slot;
        let mut var_dvp_dvs_dn0: f64 = *var_dvp_dvs_dn0_slot;
        let mut var_dvp_dvs_dn1: f64 = *var_dvp_dvs_dn1_slot;
        let mut var_dvp_dvs_dn2: f64 = *var_dvp_dvs_dn2_slot;
        let mut var_dvp_dvs_dn3: f64 = *var_dvp_dvs_dn3_slot;
        let mut var_dvpprime_dvd: f64 = *var_dvpprime_dvd_slot;
        let mut var_dvpprime_dvd_dn0: f64 = *var_dvpprime_dvd_dn0_slot;
        let mut var_dvpprime_dvd_dn1: f64 = *var_dvpprime_dvd_dn1_slot;
        let mut var_dvpprime_dvd_dn2: f64 = *var_dvpprime_dvd_dn2_slot;
        let mut var_dvpprime_dvd_dn3: f64 = *var_dvpprime_dvd_dn3_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_if_ir: f64 = *var_if_ir_slot;
        let mut var_if_ir_dn0: f64 = *var_if_ir_dn0_slot;
        let mut var_if_ir_dn1: f64 = *var_if_ir_dn1_slot;
        let mut var_if_ir_dn2: f64 = *var_if_ir_dn2_slot;
        let mut var_if_ir_dn3: f64 = *var_if_ir_dn3_slot;
        let mut var_ispec: f64 = *var_ispec_slot;
        let mut var_ispec_dn0: f64 = *var_ispec_dn0_slot;
        let mut var_ispec_dn1: f64 = *var_ispec_dn1_slot;
        let mut var_ispec_dn2: f64 = *var_ispec_dn2_slot;
        let mut var_ispec_dn3: f64 = *var_ispec_dn3_slot;
        let mut var_n: f64 = *var_n_slot;
        let mut var_n_dn0: f64 = *var_n_dn0_slot;
        let mut var_n_dn1: f64 = *var_n_dn1_slot;
        let mut var_n_dn2: f64 = *var_n_dn2_slot;
        let mut var_n_dn3: f64 = *var_n_dn3_slot;
        let mut var_tmp1: f64 = *var_tmp1_slot;
        let mut var_tmp1_dn0: f64 = *var_tmp1_dn0_slot;
        let mut var_tmp1_dn1: f64 = *var_tmp1_dn1_slot;
        let mut var_tmp1_dn2: f64 = *var_tmp1_dn2_slot;
        let mut var_tmp1_dn3: f64 = *var_tmp1_dn3_slot;
        let mut var_tmp2: f64 = *var_tmp2_slot;
        let mut var_tmp2_dn0: f64 = *var_tmp2_dn0_slot;
        let mut var_tmp2_dn1: f64 = *var_tmp2_dn1_slot;
        let mut var_tmp2_dn2: f64 = *var_tmp2_dn2_slot;
        let mut var_tmp2_dn3: f64 = *var_tmp2_dn3_slot;
        let mut var_tmp3: f64 = *var_tmp3_slot;
        let mut var_tmp3_dn0: f64 = *var_tmp3_dn0_slot;
        let mut var_tmp3_dn1: f64 = *var_tmp3_dn1_slot;
        let mut var_tmp3_dn2: f64 = *var_tmp3_dn2_slot;
        let mut var_tmp3_dn3: f64 = *var_tmp3_dn3_slot;

        let assign1750_e1401: f64 = (2.0 * var_sqrt_phi_vp);
        let assign1750_e1402: f64 = (var_gamma_s / assign1750_e1401);
        let assign1750_e1403: f64 = (1.0 + assign1750_e1402);
        var_n = assign1750_e1403;
        var_n_dn0 = (-((var_gamma_s * (2.0 * var_sqrt_phi_vp_dn0)) / (assign1750_e1401 * assign1750_e1401)));
        var_n_dn1 = (-((var_gamma_s * (2.0 * var_sqrt_phi_vp_dn1)) / (assign1750_e1401 * assign1750_e1401)));
        var_n_dn2 = (-((var_gamma_s * (2.0 * var_sqrt_phi_vp_dn2)) / (assign1750_e1401 * assign1750_e1401)));
        var_n_dn3 = (-((var_gamma_s * (2.0 * var_sqrt_phi_vp_dn3)) / (assign1750_e1401 * assign1750_e1401)));

        let assign1760_e1406: f64 = (var_if_ - var_irprime);
        var_if_ir = assign1760_e1406;
        var_if_ir_dn0 = (var_if__dn0 - var_irprime_dn0);
        var_if_ir_dn1 = (var_if__dn1 - var_irprime_dn1);
        var_if_ir_dn2 = (var_if__dn2 - var_irprime_dn2);
        var_if_ir_dn3 = (var_if__dn3 - var_irprime_dn3);

        let assign1770_e1409: f64 = (var_vt_vt_2 * var_n);
        let assign1770_e1411: f64 = (assign1770_e1409 * var_beta);
        var_ispec = assign1770_e1411;
        var_ispec_dn0 = (((var_vt_vt_2 * var_n_dn0) * var_beta) + (assign1770_e1409 * var_beta_dn0));
        var_ispec_dn1 = (((var_vt_vt_2 * var_n_dn1) * var_beta) + (assign1770_e1409 * var_beta_dn1));
        var_ispec_dn2 = (((var_vt_vt_2 * var_n_dn2) * var_beta) + (assign1770_e1409 * var_beta_dn2));
        var_ispec_dn3 = (((var_vt_vt_2 * var_n_dn3) * var_beta) + (assign1770_e1409 * var_beta_dn3));

        let assign1820_e1436: f64 = (var_sqrt_gammastar + var_sqrt_gammastar);
        let assign1820_e1437: f64 = (var_gammaprime / assign1820_e1436);
        var_tmp1 = assign1820_e1437;
        var_tmp1_dn0 = (((var_gammaprime_dn0 * assign1820_e1436) - (var_gammaprime * (var_sqrt_gammastar_dn0 + var_sqrt_gammastar_dn0))) / (assign1820_e1436 * assign1820_e1436));
        var_tmp1_dn1 = (((var_gammaprime_dn1 * assign1820_e1436) - (var_gammaprime * (var_sqrt_gammastar_dn1 + var_sqrt_gammastar_dn1))) / (assign1820_e1436 * assign1820_e1436));
        var_tmp1_dn2 = (((var_gammaprime_dn2 * assign1820_e1436) - (var_gammaprime * (var_sqrt_gammastar_dn2 + var_sqrt_gammastar_dn2))) / (assign1820_e1436 * assign1820_e1436));
        var_tmp1_dn3 = (((var_gammaprime_dn3 * assign1820_e1436) - (var_gammaprime * (var_sqrt_gammastar_dn3 + var_sqrt_gammastar_dn3))) / (assign1820_e1436 * assign1820_e1436));

        let assign1830_e1440: f64 = (var_vgprime / var_sqrt_vgstar);
        var_tmp2 = assign1830_e1440;
        var_tmp2_dn0 = (((var_vgprime_dn0 * var_sqrt_vgstar) - (var_vgprime * var_sqrt_vgstar_dn0)) / (var_sqrt_vgstar * var_sqrt_vgstar));
        var_tmp2_dn1 = (((var_vgprime_dn1 * var_sqrt_vgstar) - (var_vgprime * var_sqrt_vgstar_dn1)) / (var_sqrt_vgstar * var_sqrt_vgstar));
        var_tmp2_dn2 = (((var_vgprime_dn2 * var_sqrt_vgstar) - (var_vgprime * var_sqrt_vgstar_dn2)) / (var_sqrt_vgstar * var_sqrt_vgstar));
        var_tmp2_dn3 = (((var_vgprime_dn3 * var_sqrt_vgstar) - (var_vgprime * var_sqrt_vgstar_dn3)) / (var_sqrt_vgstar * var_sqrt_vgstar));

        let assign1840_e1442: f64 = (-var_leta_l);
        let assign1840_e1444: f64 = (assign1840_e1442 * var_tmp1);
        let assign1840_e1446: f64 = (assign1840_e1444 * var_sqrt_phi_vd);
        let assign1840_e1448: f64 = (assign1840_e1446 / var_sqrt_phi_vd_vt);
        var_dgammaprime_dvd = assign1840_e1448;
        var_dgammaprime_dvd_dn0 = ((((((assign1840_e1442 * var_tmp1_dn0) * var_sqrt_phi_vd) + (assign1840_e1444 * var_sqrt_phi_vd_dn0)) * var_sqrt_phi_vd_vt) - (assign1840_e1446 * var_sqrt_phi_vd_vt_dn0)) / (var_sqrt_phi_vd_vt * var_sqrt_phi_vd_vt));
        var_dgammaprime_dvd_dn1 = ((((((assign1840_e1442 * var_tmp1_dn1) * var_sqrt_phi_vd) + (assign1840_e1444 * var_sqrt_phi_vd_dn1)) * var_sqrt_phi_vd_vt) - (assign1840_e1446 * var_sqrt_phi_vd_vt_dn1)) / (var_sqrt_phi_vd_vt * var_sqrt_phi_vd_vt));
        var_dgammaprime_dvd_dn2 = ((((((assign1840_e1442 * var_tmp1_dn2) * var_sqrt_phi_vd) + (assign1840_e1444 * var_sqrt_phi_vd_dn2)) * var_sqrt_phi_vd_vt) - (assign1840_e1446 * var_sqrt_phi_vd_vt_dn2)) / (var_sqrt_phi_vd_vt * var_sqrt_phi_vd_vt));
        var_dgammaprime_dvd_dn3 = ((((((assign1840_e1442 * var_tmp1_dn3) * var_sqrt_phi_vd) + (assign1840_e1444 * var_sqrt_phi_vd_dn3)) * var_sqrt_phi_vd_vt) - (assign1840_e1446 * var_sqrt_phi_vd_vt_dn3)) / (var_sqrt_phi_vd_vt * var_sqrt_phi_vd_vt));

        let assign1850_e1450: f64 = (-var_leta_l);
        let assign1850_e1452: f64 = (assign1850_e1450 * var_tmp1);
        let assign1850_e1454: f64 = (assign1850_e1452 * var_sqrt_phi_vs);
        let assign1850_e1456: f64 = (assign1850_e1454 / var_sqrt_phi_vs_vt);
        var_dgammaprime_dvs = assign1850_e1456;
        var_dgammaprime_dvs_dn0 = ((((((assign1850_e1450 * var_tmp1_dn0) * var_sqrt_phi_vs) + (assign1850_e1452 * var_sqrt_phi_vs_dn0)) * var_sqrt_phi_vs_vt) - (assign1850_e1454 * var_sqrt_phi_vs_vt_dn0)) / (var_sqrt_phi_vs_vt * var_sqrt_phi_vs_vt));
        var_dgammaprime_dvs_dn1 = ((((((assign1850_e1450 * var_tmp1_dn1) * var_sqrt_phi_vs) + (assign1850_e1452 * var_sqrt_phi_vs_dn1)) * var_sqrt_phi_vs_vt) - (assign1850_e1454 * var_sqrt_phi_vs_vt_dn1)) / (var_sqrt_phi_vs_vt * var_sqrt_phi_vs_vt));
        var_dgammaprime_dvs_dn2 = ((((((assign1850_e1450 * var_tmp1_dn2) * var_sqrt_phi_vs) + (assign1850_e1452 * var_sqrt_phi_vs_dn2)) * var_sqrt_phi_vs_vt) - (assign1850_e1454 * var_sqrt_phi_vs_vt_dn2)) / (var_sqrt_phi_vs_vt * var_sqrt_phi_vs_vt));
        var_dgammaprime_dvs_dn3 = ((((((assign1850_e1450 * var_tmp1_dn3) * var_sqrt_phi_vs) + (assign1850_e1452 * var_sqrt_phi_vs_dn3)) * var_sqrt_phi_vs_vt) - (assign1850_e1454 * var_sqrt_phi_vs_vt_dn3)) / (var_sqrt_phi_vs_vt * var_sqrt_phi_vs_vt));

        let assign1870_e1474: f64 = (var_vp + var_phi_t);
        let assign1870_e1476: f64 = (assign1870_e1474 / var_big_sqrt_vp);
        var_tmp3 = assign1870_e1476;
        var_tmp3_dn0 = ((((var_vp_dn0 + var_phi_t_dn0) * var_big_sqrt_vp) - (assign1870_e1474 * var_big_sqrt_vp_dn0)) / (var_big_sqrt_vp * var_big_sqrt_vp));
        var_tmp3_dn1 = ((((var_vp_dn1 + var_phi_t_dn1) * var_big_sqrt_vp) - (assign1870_e1474 * var_big_sqrt_vp_dn1)) / (var_big_sqrt_vp * var_big_sqrt_vp));
        var_tmp3_dn2 = ((((var_vp_dn2 + var_phi_t_dn2) * var_big_sqrt_vp) - (assign1870_e1474 * var_big_sqrt_vp_dn2)) / (var_big_sqrt_vp * var_big_sqrt_vp));
        var_tmp3_dn3 = ((((var_vp_dn3 + var_phi_t_dn3) * var_big_sqrt_vp) - (assign1870_e1474 * var_big_sqrt_vp_dn3)) / (var_big_sqrt_vp * var_big_sqrt_vp));

        let assign1880_e1478: f64 = (-var_tmp3);
        let assign1880_e1480: f64 = (assign1880_e1478 * var_dgammaprime_dvd);
        var_dvp_dvd = assign1880_e1480;
        var_dvp_dvd_dn0 = (((-var_tmp3_dn0) * var_dgammaprime_dvd) + (assign1880_e1478 * var_dgammaprime_dvd_dn0));
        var_dvp_dvd_dn1 = (((-var_tmp3_dn1) * var_dgammaprime_dvd) + (assign1880_e1478 * var_dgammaprime_dvd_dn1));
        var_dvp_dvd_dn2 = (((-var_tmp3_dn2) * var_dgammaprime_dvd) + (assign1880_e1478 * var_dgammaprime_dvd_dn2));
        var_dvp_dvd_dn3 = (((-var_tmp3_dn3) * var_dgammaprime_dvd) + (assign1880_e1478 * var_dgammaprime_dvd_dn3));

        let assign1890_e1482: f64 = (-var_tmp3);
        let assign1890_e1484: f64 = (assign1890_e1482 * var_dgammaprime_dvs);
        var_dvp_dvs = assign1890_e1484;
        var_dvp_dvs_dn0 = (((-var_tmp3_dn0) * var_dgammaprime_dvs) + (assign1890_e1482 * var_dgammaprime_dvs_dn0));
        var_dvp_dvs_dn1 = (((-var_tmp3_dn1) * var_dgammaprime_dvs) + (assign1890_e1482 * var_dgammaprime_dvs_dn1));
        var_dvp_dvs_dn2 = (((-var_tmp3_dn2) * var_dgammaprime_dvs) + (assign1890_e1482 * var_dgammaprime_dvs_dn2));
        var_dvp_dvs_dn3 = (((-var_tmp3_dn3) * var_dgammaprime_dvs) + (assign1890_e1482 * var_dgammaprime_dvs_dn3));

        let assign1910_e1501: f64 = (var_dif_dv * var_inv_vt);
        var_tmp1 = assign1910_e1501;
        var_tmp1_dn0 = (var_dif_dv_dn0 * var_inv_vt);
        var_tmp1_dn1 = (var_dif_dv_dn1 * var_inv_vt);
        var_tmp1_dn2 = (var_dif_dv_dn2 * var_inv_vt);
        var_tmp1_dn3 = (var_dif_dv_dn3 * var_inv_vt);

        let assign1920_e1504: f64 = (var_tmp1 * var_dvp_dvd);
        var_dif_dvd = assign1920_e1504;
        var_dif_dvd_dn0 = ((var_tmp1_dn0 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn0));
        var_dif_dvd_dn1 = ((var_tmp1_dn1 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn1));
        var_dif_dvd_dn2 = ((var_tmp1_dn2 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn2));
        var_dif_dvd_dn3 = ((var_tmp1_dn3 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn3));

        let assign1930_e1508: f64 = (var_dvp_dvs - 1.0);
        let assign1930_e1509: f64 = (var_tmp1 * assign1930_e1508);
        var_dif_dvs = assign1930_e1509;
        var_dif_dvs_dn0 = ((var_tmp1_dn0 * assign1930_e1508) + (var_tmp1 * var_dvp_dvs_dn0));
        var_dif_dvs_dn1 = ((var_tmp1_dn1 * assign1930_e1508) + (var_tmp1 * var_dvp_dvs_dn1));
        var_dif_dvs_dn2 = ((var_tmp1_dn2 * assign1930_e1508) + (var_tmp1 * var_dvp_dvs_dn2));
        var_dif_dvs_dn3 = ((var_tmp1_dn3 * assign1930_e1508) + (var_tmp1 * var_dvp_dvs_dn3));

        let assign1950_e1516: f64 = (4.0 * var_vdss_sqrt);
        let assign1950_e1518: f64 = (assign1950_e1516 * var_sqrt_if);
        let assign1950_e1519: f64 = (var_vt / assign1950_e1518);
        var_tmp1 = assign1950_e1519;
        var_tmp1_dn0 = (-((var_vt * (((4.0 * var_vdss_sqrt_dn0) * var_sqrt_if) + (assign1950_e1516 * var_sqrt_if_dn0))) / (assign1950_e1518 * assign1950_e1518)));
        var_tmp1_dn1 = (-((var_vt * (((4.0 * var_vdss_sqrt_dn1) * var_sqrt_if) + (assign1950_e1516 * var_sqrt_if_dn1))) / (assign1950_e1518 * assign1950_e1518)));
        var_tmp1_dn2 = (-((var_vt * (((4.0 * var_vdss_sqrt_dn2) * var_sqrt_if) + (assign1950_e1516 * var_sqrt_if_dn2))) / (assign1950_e1518 * assign1950_e1518)));
        var_tmp1_dn3 = (-((var_vt * (((4.0 * var_vdss_sqrt_dn3) * var_sqrt_if) + (assign1950_e1516 * var_sqrt_if_dn3))) / (assign1950_e1518 * assign1950_e1518)));

        let assign1960_e1522: f64 = (var_tmp1 * var_dif_dvd);
        var_dvdss_dvd = assign1960_e1522;
        var_dvdss_dvd_dn0 = ((var_tmp1_dn0 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn0));
        var_dvdss_dvd_dn1 = ((var_tmp1_dn1 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn1));
        var_dvdss_dvd_dn2 = ((var_tmp1_dn2 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn2));
        var_dvdss_dvd_dn3 = ((var_tmp1_dn3 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn3));

        let assign1970_e1525: f64 = (var_tmp1 * var_dif_dvs);
        var_dvdss_dvs = assign1970_e1525;
        var_dvdss_dvs_dn0 = ((var_tmp1_dn0 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn0));
        var_dvdss_dvs_dn1 = ((var_tmp1_dn1 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn1));
        var_dvdss_dvs_dn2 = ((var_tmp1_dn2 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn2));
        var_dvdss_dvs_dn3 = ((var_tmp1_dn3 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn3));

        let assign1990_e1531: f64 = (var_vt_4 + var_vt_4);
        let assign1990_e1533: f64 = (assign1990_e1531 * p.p25);
        var_tmp1 = assign1990_e1533;
        var_tmp1_dn0 = 0.0;
        var_tmp1_dn1 = 0.0;
        var_tmp1_dn2 = 0.0;
        var_tmp1_dn3 = 0.0;

        let assign2000_e1537: f64 = (var_sqrt_if + var_sqrt_if);
        let assign2000_e1538: f64 = (var_vt / assign2000_e1537);
        var_tmp2 = assign2000_e1538;
        var_tmp2_dn0 = (-((var_vt * (var_sqrt_if_dn0 + var_sqrt_if_dn0)) / (assign2000_e1537 * assign2000_e1537)));
        var_tmp2_dn1 = (-((var_vt * (var_sqrt_if_dn1 + var_sqrt_if_dn1)) / (assign2000_e1537 * assign2000_e1537)));
        var_tmp2_dn2 = (-((var_vt * (var_sqrt_if_dn2 + var_sqrt_if_dn2)) / (assign2000_e1537 * assign2000_e1537)));
        var_tmp2_dn3 = (-((var_vt * (var_sqrt_if_dn3 + var_sqrt_if_dn3)) / (assign2000_e1537 * assign2000_e1537)));

        let assign2010_e1542: f64 = (var_dif_dvd * var_tmp2);
        let assign2010_e1544: f64 = (assign2010_e1542 - var_dvdss_dvd);
        let assign2010_e1545: f64 = (var_tmp1 * assign2010_e1544);
        var_ddeltav_dvd = assign2010_e1545;
        var_ddeltav_dvd_dn0 = ((var_tmp1_dn0 * assign2010_e1544) + (var_tmp1 * (((var_dif_dvd_dn0 * var_tmp2) + (var_dif_dvd * var_tmp2_dn0)) - var_dvdss_dvd_dn0)));
        var_ddeltav_dvd_dn1 = ((var_tmp1_dn1 * assign2010_e1544) + (var_tmp1 * (((var_dif_dvd_dn1 * var_tmp2) + (var_dif_dvd * var_tmp2_dn1)) - var_dvdss_dvd_dn1)));
        var_ddeltav_dvd_dn2 = ((var_tmp1_dn2 * assign2010_e1544) + (var_tmp1 * (((var_dif_dvd_dn2 * var_tmp2) + (var_dif_dvd * var_tmp2_dn2)) - var_dvdss_dvd_dn2)));
        var_ddeltav_dvd_dn3 = ((var_tmp1_dn3 * assign2010_e1544) + (var_tmp1 * (((var_dif_dvd_dn3 * var_tmp2) + (var_dif_dvd * var_tmp2_dn3)) - var_dvdss_dvd_dn3)));

        let assign2020_e1549: f64 = (var_dif_dvs * var_tmp2);
        let assign2020_e1551: f64 = (assign2020_e1549 - var_dvdss_dvs);
        let assign2020_e1552: f64 = (var_tmp1 * assign2020_e1551);
        var_ddeltav_dvs = assign2020_e1552;
        var_ddeltav_dvs_dn0 = ((var_tmp1_dn0 * assign2020_e1551) + (var_tmp1 * (((var_dif_dvs_dn0 * var_tmp2) + (var_dif_dvs * var_tmp2_dn0)) - var_dvdss_dvs_dn0)));
        var_ddeltav_dvs_dn1 = ((var_tmp1_dn1 * assign2020_e1551) + (var_tmp1 * (((var_dif_dvs_dn1 * var_tmp2) + (var_dif_dvs * var_tmp2_dn1)) - var_dvdss_dvs_dn1)));
        var_ddeltav_dvs_dn2 = ((var_tmp1_dn2 * assign2020_e1551) + (var_tmp1 * (((var_dif_dvs_dn2 * var_tmp2) + (var_dif_dvs * var_tmp2_dn2)) - var_dvdss_dvs_dn2)));
        var_ddeltav_dvs_dn3 = ((var_tmp1_dn3 * assign2020_e1551) + (var_tmp1 * (((var_dif_dvs_dn3 * var_tmp2) + (var_dif_dvs * var_tmp2_dn3)) - var_dvdss_dvs_dn3)));

        let assign2040_e1562: f64 = (1.0 / var_sqrt_vdss_deltav);
        var_tmp1 = assign2040_e1562;
        var_tmp1_dn0 = (-(var_sqrt_vdss_deltav_dn0 / (var_sqrt_vdss_deltav * var_sqrt_vdss_deltav)));
        var_tmp1_dn1 = (-(var_sqrt_vdss_deltav_dn1 / (var_sqrt_vdss_deltav * var_sqrt_vdss_deltav)));
        var_tmp1_dn2 = (-(var_sqrt_vdss_deltav_dn2 / (var_sqrt_vdss_deltav * var_sqrt_vdss_deltav)));
        var_tmp1_dn3 = (-(var_sqrt_vdss_deltav_dn3 / (var_sqrt_vdss_deltav * var_sqrt_vdss_deltav)));

        let assign2050_e1565: f64 = (1.0 / var_sqrt_vds_vdss_deltav);
        var_tmp2 = assign2050_e1565;
        var_tmp2_dn0 = (-(var_sqrt_vds_vdss_deltav_dn0 / (var_sqrt_vds_vdss_deltav * var_sqrt_vds_vdss_deltav)));
        var_tmp2_dn1 = (-(var_sqrt_vds_vdss_deltav_dn1 / (var_sqrt_vds_vdss_deltav * var_sqrt_vds_vdss_deltav)));
        var_tmp2_dn2 = (-(var_sqrt_vds_vdss_deltav_dn2 / (var_sqrt_vds_vdss_deltav * var_sqrt_vds_vdss_deltav)));
        var_tmp2_dn3 = (-(var_sqrt_vds_vdss_deltav_dn3 / (var_sqrt_vds_vdss_deltav * var_sqrt_vds_vdss_deltav)));

        let assign2060_e1568: f64 = (var_vds - var_vdss);
        var_tmp3 = assign2060_e1568;
        var_tmp3_dn0 = (var_vds_dn0 - var_vdss_dn0);
        var_tmp3_dn1 = (-var_vdss_dn1);
        var_tmp3_dn2 = (var_vds_dn2 - var_vdss_dn2);
        var_tmp3_dn3 = (var_vds_dn3 - var_vdss_dn3);

        let assign2070_e1571: f64 = (var_vdss * var_dvdss_dvd);
        let assign2070_e1573: f64 = (assign2070_e1571 + var_ddeltav_dvd);
        let assign2070_e1575: f64 = (assign2070_e1573 * var_tmp1);
        let assign2070_e1579: f64 = (0.5 - var_dvdss_dvd);
        let assign2070_e1580: f64 = (var_tmp3 * assign2070_e1579);
        let assign2070_e1582: f64 = (assign2070_e1580 + var_ddeltav_dvd);
        let assign2070_e1584: f64 = (assign2070_e1582 * var_tmp2);
        let assign2070_e1585: f64 = (assign2070_e1575 - assign2070_e1584);
        var_dvip_dvd = assign2070_e1585;
        var_dvip_dvd_dn0 = ((((((var_vdss_dn0 * var_dvdss_dvd) + (var_vdss * var_dvdss_dvd_dn0)) + var_ddeltav_dvd_dn0) * var_tmp1) + (assign2070_e1573 * var_tmp1_dn0)) - (((((var_tmp3_dn0 * assign2070_e1579) + (var_tmp3 * (-var_dvdss_dvd_dn0))) + var_ddeltav_dvd_dn0) * var_tmp2) + (assign2070_e1582 * var_tmp2_dn0)));
        var_dvip_dvd_dn1 = ((((((var_vdss_dn1 * var_dvdss_dvd) + (var_vdss * var_dvdss_dvd_dn1)) + var_ddeltav_dvd_dn1) * var_tmp1) + (assign2070_e1573 * var_tmp1_dn1)) - (((((var_tmp3_dn1 * assign2070_e1579) + (var_tmp3 * (-var_dvdss_dvd_dn1))) + var_ddeltav_dvd_dn1) * var_tmp2) + (assign2070_e1582 * var_tmp2_dn1)));
        var_dvip_dvd_dn2 = ((((((var_vdss_dn2 * var_dvdss_dvd) + (var_vdss * var_dvdss_dvd_dn2)) + var_ddeltav_dvd_dn2) * var_tmp1) + (assign2070_e1573 * var_tmp1_dn2)) - (((((var_tmp3_dn2 * assign2070_e1579) + (var_tmp3 * (-var_dvdss_dvd_dn2))) + var_ddeltav_dvd_dn2) * var_tmp2) + (assign2070_e1582 * var_tmp2_dn2)));
        var_dvip_dvd_dn3 = ((((((var_vdss_dn3 * var_dvdss_dvd) + (var_vdss * var_dvdss_dvd_dn3)) + var_ddeltav_dvd_dn3) * var_tmp1) + (assign2070_e1573 * var_tmp1_dn3)) - (((((var_tmp3_dn3 * assign2070_e1579) + (var_tmp3 * (-var_dvdss_dvd_dn3))) + var_ddeltav_dvd_dn3) * var_tmp2) + (assign2070_e1582 * var_tmp2_dn3)));

        let assign2080_e1588: f64 = (var_vdss * var_dvdss_dvs);
        let assign2080_e1590: f64 = (assign2080_e1588 + var_ddeltav_dvs);
        let assign2080_e1592: f64 = (assign2080_e1590 * var_tmp1);
        let assign2080_e1595: f64 = (-0.5);
        let assign2080_e1597: f64 = (assign2080_e1595 - var_dvdss_dvs);
        let assign2080_e1598: f64 = (var_tmp3 * assign2080_e1597);
        let assign2080_e1600: f64 = (assign2080_e1598 + var_ddeltav_dvs);
        let assign2080_e1602: f64 = (assign2080_e1600 * var_tmp2);
        let assign2080_e1603: f64 = (assign2080_e1592 - assign2080_e1602);
        var_dvip_dvs = assign2080_e1603;
        var_dvip_dvs_dn0 = ((((((var_vdss_dn0 * var_dvdss_dvs) + (var_vdss * var_dvdss_dvs_dn0)) + var_ddeltav_dvs_dn0) * var_tmp1) + (assign2080_e1590 * var_tmp1_dn0)) - (((((var_tmp3_dn0 * assign2080_e1597) + (var_tmp3 * (-var_dvdss_dvs_dn0))) + var_ddeltav_dvs_dn0) * var_tmp2) + (assign2080_e1600 * var_tmp2_dn0)));
        var_dvip_dvs_dn1 = ((((((var_vdss_dn1 * var_dvdss_dvs) + (var_vdss * var_dvdss_dvs_dn1)) + var_ddeltav_dvs_dn1) * var_tmp1) + (assign2080_e1590 * var_tmp1_dn1)) - (((((var_tmp3_dn1 * assign2080_e1597) + (var_tmp3 * (-var_dvdss_dvs_dn1))) + var_ddeltav_dvs_dn1) * var_tmp2) + (assign2080_e1600 * var_tmp2_dn1)));
        var_dvip_dvs_dn2 = ((((((var_vdss_dn2 * var_dvdss_dvs) + (var_vdss * var_dvdss_dvs_dn2)) + var_ddeltav_dvs_dn2) * var_tmp1) + (assign2080_e1590 * var_tmp1_dn2)) - (((((var_tmp3_dn2 * assign2080_e1597) + (var_tmp3 * (-var_dvdss_dvs_dn2))) + var_ddeltav_dvs_dn2) * var_tmp2) + (assign2080_e1600 * var_tmp2_dn2)));
        var_dvip_dvs_dn3 = ((((((var_vdss_dn3 * var_dvdss_dvs) + (var_vdss * var_dvdss_dvs_dn3)) + var_ddeltav_dvs_dn3) * var_tmp1) + (assign2080_e1590 * var_tmp1_dn3)) - (((((var_tmp3_dn3 * assign2080_e1597) + (var_tmp3 * (-var_dvdss_dvs_dn3))) + var_ddeltav_dvs_dn3) * var_tmp2) + (assign2080_e1600 * var_tmp2_dn3)));

        let assign2100_e1623: f64 = (var_sqrt_if - 1.5);
        let assign2100_e1624: f64 = (var_vt * assign2100_e1623);
        let assign2100_e1627: f64 = (4.0 * var_vdssprime_sqrt);
        let assign2100_e1629: f64 = (assign2100_e1627 * var_if_);
        let assign2100_e1630: f64 = (assign2100_e1624 / assign2100_e1629);
        var_tmp1 = assign2100_e1630;
        var_tmp1_dn0 = ((((var_vt * var_sqrt_if_dn0) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * var_vdssprime_sqrt_dn0) * var_if_) + (assign2100_e1627 * var_if__dn0)))) / (assign2100_e1629 * assign2100_e1629));
        var_tmp1_dn1 = ((((var_vt * var_sqrt_if_dn1) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * var_vdssprime_sqrt_dn1) * var_if_) + (assign2100_e1627 * var_if__dn1)))) / (assign2100_e1629 * assign2100_e1629));
        var_tmp1_dn2 = ((((var_vt * var_sqrt_if_dn2) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * var_vdssprime_sqrt_dn2) * var_if_) + (assign2100_e1627 * var_if__dn2)))) / (assign2100_e1629 * assign2100_e1629));
        var_tmp1_dn3 = ((((var_vt * var_sqrt_if_dn3) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * var_vdssprime_sqrt_dn3) * var_if_) + (assign2100_e1627 * var_if__dn3)))) / (assign2100_e1629 * assign2100_e1629));

        let assign2110_e1633: f64 = (var_tmp1 * var_dif_dvd);
        var_dvdssprime_dvd = assign2110_e1633;
        var_dvdssprime_dvd_dn0 = ((var_tmp1_dn0 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn0));
        var_dvdssprime_dvd_dn1 = ((var_tmp1_dn1 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn1));
        var_dvdssprime_dvd_dn2 = ((var_tmp1_dn2 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn2));
        var_dvdssprime_dvd_dn3 = ((var_tmp1_dn3 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn3));

        let assign2120_e1636: f64 = (var_tmp1 * var_dif_dvs);
        var_dvdssprime_dvs = assign2120_e1636;
        var_dvdssprime_dvs_dn0 = ((var_tmp1_dn0 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn0));
        var_dvdssprime_dvs_dn1 = ((var_tmp1_dn1 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn1));
        var_dvdssprime_dvs_dn2 = ((var_tmp1_dn2 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn2));
        var_dvdssprime_dvs_dn3 = ((var_tmp1_dn3 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn3));

        let assign2140_e1642: f64 = (var_dirprime_dv * var_inv_vt);
        var_tmp1 = assign2140_e1642;
        var_tmp1_dn0 = (var_dirprime_dv_dn0 * var_inv_vt);
        var_tmp1_dn1 = (var_dirprime_dv_dn1 * var_inv_vt);
        var_tmp1_dn2 = (var_dirprime_dv_dn2 * var_inv_vt);
        var_tmp1_dn3 = (var_dirprime_dv_dn3 * var_inv_vt);

        let assign2150_e1645: f64 = (1.0 / var_sqrt_vdssprime_deltav);
        var_tmp2 = assign2150_e1645;
        var_tmp2_dn0 = (-(var_sqrt_vdssprime_deltav_dn0 / (var_sqrt_vdssprime_deltav * var_sqrt_vdssprime_deltav)));
        var_tmp2_dn1 = (-(var_sqrt_vdssprime_deltav_dn1 / (var_sqrt_vdssprime_deltav * var_sqrt_vdssprime_deltav)));
        var_tmp2_dn2 = (-(var_sqrt_vdssprime_deltav_dn2 / (var_sqrt_vdssprime_deltav * var_sqrt_vdssprime_deltav)));
        var_tmp2_dn3 = (-(var_sqrt_vdssprime_deltav_dn3 / (var_sqrt_vdssprime_deltav * var_sqrt_vdssprime_deltav)));

        let assign2160_e1648: f64 = (1.0 / var_sqrt_vds_vdssprime_deltav);
        var_tmp3 = assign2160_e1648;
        var_tmp3_dn0 = (-(var_sqrt_vds_vdssprime_deltav_dn0 / (var_sqrt_vds_vdssprime_deltav * var_sqrt_vds_vdssprime_deltav)));
        var_tmp3_dn1 = (-(var_sqrt_vds_vdssprime_deltav_dn1 / (var_sqrt_vds_vdssprime_deltav * var_sqrt_vds_vdssprime_deltav)));
        var_tmp3_dn2 = (-(var_sqrt_vds_vdssprime_deltav_dn2 / (var_sqrt_vds_vdssprime_deltav * var_sqrt_vds_vdssprime_deltav)));
        var_tmp3_dn3 = (-(var_sqrt_vds_vdssprime_deltav_dn3 / (var_sqrt_vds_vdssprime_deltav * var_sqrt_vds_vdssprime_deltav)));

        let assign2170_e1652: f64 = (var_dvp_dvd - 0.5);
        let assign2170_e1655: f64 = (var_vdssprime * var_dvdssprime_dvd);
        let assign2170_e1657: f64 = (assign2170_e1655 + var_ddeltav_dvd);
        let assign2170_e1659: f64 = (assign2170_e1657 * var_tmp2);
        let assign2170_e1660: f64 = (assign2170_e1652 - assign2170_e1659);
        let assign2170_e1664: f64 = (0.5 - var_dvdssprime_dvd);
        let assign2170_e1665: f64 = (var_vdsprime * assign2170_e1664);
        let assign2170_e1667: f64 = (assign2170_e1665 + var_ddeltav_dvd);
        let assign2170_e1669: f64 = (assign2170_e1667 * var_tmp3);
        let assign2170_e1670: f64 = (assign2170_e1660 + assign2170_e1669);
        let assign2170_e1671: f64 = (var_tmp1 * assign2170_e1670);
        var_dirprime_dvd = assign2170_e1671;
        var_dirprime_dvd_dn0 = ((var_tmp1_dn0 * assign2170_e1670) + (var_tmp1 * ((var_dvp_dvd_dn0 - (((((var_vdssprime_dn0 * var_dvdssprime_dvd) + (var_vdssprime * var_dvdssprime_dvd_dn0)) + var_ddeltav_dvd_dn0) * var_tmp2) + (assign2170_e1657 * var_tmp2_dn0))) + (((((var_vdsprime_dn0 * assign2170_e1664) + (var_vdsprime * (-var_dvdssprime_dvd_dn0))) + var_ddeltav_dvd_dn0) * var_tmp3) + (assign2170_e1667 * var_tmp3_dn0)))));
        var_dirprime_dvd_dn1 = ((var_tmp1_dn1 * assign2170_e1670) + (var_tmp1 * ((var_dvp_dvd_dn1 - (((((var_vdssprime_dn1 * var_dvdssprime_dvd) + (var_vdssprime * var_dvdssprime_dvd_dn1)) + var_ddeltav_dvd_dn1) * var_tmp2) + (assign2170_e1657 * var_tmp2_dn1))) + (((((var_vdsprime_dn1 * assign2170_e1664) + (var_vdsprime * (-var_dvdssprime_dvd_dn1))) + var_ddeltav_dvd_dn1) * var_tmp3) + (assign2170_e1667 * var_tmp3_dn1)))));
        var_dirprime_dvd_dn2 = ((var_tmp1_dn2 * assign2170_e1670) + (var_tmp1 * ((var_dvp_dvd_dn2 - (((((var_vdssprime_dn2 * var_dvdssprime_dvd) + (var_vdssprime * var_dvdssprime_dvd_dn2)) + var_ddeltav_dvd_dn2) * var_tmp2) + (assign2170_e1657 * var_tmp2_dn2))) + (((((var_vdsprime_dn2 * assign2170_e1664) + (var_vdsprime * (-var_dvdssprime_dvd_dn2))) + var_ddeltav_dvd_dn2) * var_tmp3) + (assign2170_e1667 * var_tmp3_dn2)))));
        var_dirprime_dvd_dn3 = ((var_tmp1_dn3 * assign2170_e1670) + (var_tmp1 * ((var_dvp_dvd_dn3 - (((((var_vdssprime_dn3 * var_dvdssprime_dvd) + (var_vdssprime * var_dvdssprime_dvd_dn3)) + var_ddeltav_dvd_dn3) * var_tmp2) + (assign2170_e1657 * var_tmp2_dn3))) + (((((var_vdsprime_dn3 * assign2170_e1664) + (var_vdsprime * (-var_dvdssprime_dvd_dn3))) + var_ddeltav_dvd_dn3) * var_tmp3) + (assign2170_e1667 * var_tmp3_dn3)))));

        let assign2180_e1675: f64 = (var_dvp_dvs - 0.5);
        let assign2180_e1678: f64 = (var_vdssprime * var_dvdssprime_dvs);
        let assign2180_e1680: f64 = (assign2180_e1678 + var_ddeltav_dvs);
        let assign2180_e1682: f64 = (assign2180_e1680 * var_tmp2);
        let assign2180_e1683: f64 = (assign2180_e1675 - assign2180_e1682);
        let assign2180_e1686: f64 = (-0.5);
        let assign2180_e1688: f64 = (assign2180_e1686 - var_dvdssprime_dvs);
        let assign2180_e1689: f64 = (var_vdsprime * assign2180_e1688);
        let assign2180_e1691: f64 = (assign2180_e1689 + var_ddeltav_dvs);
        let assign2180_e1693: f64 = (assign2180_e1691 * var_tmp3);
        let assign2180_e1694: f64 = (assign2180_e1683 + assign2180_e1693);
        let assign2180_e1695: f64 = (var_tmp1 * assign2180_e1694);
        var_dirprime_dvs = assign2180_e1695;
        var_dirprime_dvs_dn0 = ((var_tmp1_dn0 * assign2180_e1694) + (var_tmp1 * ((var_dvp_dvs_dn0 - (((((var_vdssprime_dn0 * var_dvdssprime_dvs) + (var_vdssprime * var_dvdssprime_dvs_dn0)) + var_ddeltav_dvs_dn0) * var_tmp2) + (assign2180_e1680 * var_tmp2_dn0))) + (((((var_vdsprime_dn0 * assign2180_e1688) + (var_vdsprime * (-var_dvdssprime_dvs_dn0))) + var_ddeltav_dvs_dn0) * var_tmp3) + (assign2180_e1691 * var_tmp3_dn0)))));
        var_dirprime_dvs_dn1 = ((var_tmp1_dn1 * assign2180_e1694) + (var_tmp1 * ((var_dvp_dvs_dn1 - (((((var_vdssprime_dn1 * var_dvdssprime_dvs) + (var_vdssprime * var_dvdssprime_dvs_dn1)) + var_ddeltav_dvs_dn1) * var_tmp2) + (assign2180_e1680 * var_tmp2_dn1))) + (((((var_vdsprime_dn1 * assign2180_e1688) + (var_vdsprime * (-var_dvdssprime_dvs_dn1))) + var_ddeltav_dvs_dn1) * var_tmp3) + (assign2180_e1691 * var_tmp3_dn1)))));
        var_dirprime_dvs_dn2 = ((var_tmp1_dn2 * assign2180_e1694) + (var_tmp1 * ((var_dvp_dvs_dn2 - (((((var_vdssprime_dn2 * var_dvdssprime_dvs) + (var_vdssprime * var_dvdssprime_dvs_dn2)) + var_ddeltav_dvs_dn2) * var_tmp2) + (assign2180_e1680 * var_tmp2_dn2))) + (((((var_vdsprime_dn2 * assign2180_e1688) + (var_vdsprime * (-var_dvdssprime_dvs_dn2))) + var_ddeltav_dvs_dn2) * var_tmp3) + (assign2180_e1691 * var_tmp3_dn2)))));
        var_dirprime_dvs_dn3 = ((var_tmp1_dn3 * assign2180_e1694) + (var_tmp1 * ((var_dvp_dvs_dn3 - (((((var_vdssprime_dn3 * var_dvdssprime_dvs) + (var_vdssprime * var_dvdssprime_dvs_dn3)) + var_ddeltav_dvs_dn3) * var_tmp2) + (assign2180_e1680 * var_tmp2_dn3))) + (((((var_vdsprime_dn3 * assign2180_e1688) + (var_vdsprime * (-var_dvdssprime_dvs_dn3))) + var_ddeltav_dvs_dn3) * var_tmp3) + (assign2180_e1691 * var_tmp3_dn3)))));

        let assign2200_e1719: f64 = (var_lc_ucrit + var_vds);
        let assign2200_e1721: f64 = (assign2200_e1719 - var_vip);
        let assign2200_e1722: f64 = (var_lc_lambda / assign2200_e1721);
        var_tmp1 = assign2200_e1722;
        var_tmp1_dn0 = (-((var_lc_lambda * (var_vds_dn0 - var_vip_dn0)) / (assign2200_e1721 * assign2200_e1721)));
        var_tmp1_dn1 = (-((var_lc_lambda * (-var_vip_dn1)) / (assign2200_e1721 * assign2200_e1721)));
        var_tmp1_dn2 = (-((var_lc_lambda * (var_vds_dn2 - var_vip_dn2)) / (assign2200_e1721 * assign2200_e1721)));
        var_tmp1_dn3 = (-((var_lc_lambda * (var_vds_dn3 - var_vip_dn3)) / (assign2200_e1721 * assign2200_e1721)));

        let assign2210_e1726: f64 = (0.5 - var_dvip_dvd);
        let assign2210_e1727: f64 = (var_tmp1 * assign2210_e1726);
        var_ddeltal_dvd = assign2210_e1727;
        var_ddeltal_dvd_dn0 = ((var_tmp1_dn0 * assign2210_e1726) + (var_tmp1 * (-var_dvip_dvd_dn0)));
        var_ddeltal_dvd_dn1 = ((var_tmp1_dn1 * assign2210_e1726) + (var_tmp1 * (-var_dvip_dvd_dn1)));
        var_ddeltal_dvd_dn2 = ((var_tmp1_dn2 * assign2210_e1726) + (var_tmp1 * (-var_dvip_dvd_dn2)));
        var_ddeltal_dvd_dn3 = ((var_tmp1_dn3 * assign2210_e1726) + (var_tmp1 * (-var_dvip_dvd_dn3)));

        let assign2220_e1730: f64 = (-0.5);
        let assign2220_e1732: f64 = (assign2220_e1730 - var_dvip_dvs);
        let assign2220_e1733: f64 = (var_tmp1 * assign2220_e1732);
        var_ddeltal_dvs = assign2220_e1733;
        var_ddeltal_dvs_dn0 = ((var_tmp1_dn0 * assign2220_e1732) + (var_tmp1 * (-var_dvip_dvs_dn0)));
        var_ddeltal_dvs_dn1 = ((var_tmp1_dn1 * assign2220_e1732) + (var_tmp1 * (-var_dvip_dvs_dn1)));
        var_ddeltal_dvs_dn2 = ((var_tmp1_dn2 * assign2220_e1732) + (var_tmp1 * (-var_dvip_dvs_dn2)));
        var_ddeltal_dvs_dn3 = ((var_tmp1_dn3 * assign2220_e1732) + (var_tmp1 * (-var_dvip_dvs_dn3)));

        let assign2240_e1740: f64 = (1.0 / var_sqrt_lprime_lmin);
        var_tmp1 = assign2240_e1740;
        var_tmp1_dn0 = (-(var_sqrt_lprime_lmin_dn0 / (var_sqrt_lprime_lmin * var_sqrt_lprime_lmin)));
        var_tmp1_dn1 = (-(var_sqrt_lprime_lmin_dn1 / (var_sqrt_lprime_lmin * var_sqrt_lprime_lmin)));
        var_tmp1_dn2 = (-(var_sqrt_lprime_lmin_dn2 / (var_sqrt_lprime_lmin * var_sqrt_lprime_lmin)));
        var_tmp1_dn3 = (-(var_sqrt_lprime_lmin_dn3 / (var_sqrt_lprime_lmin * var_sqrt_lprime_lmin)));

        let assign2250_e1743: f64 = (-var_ddeltal_dvd);
        let assign2250_e1746: f64 = (0.5 + var_dvip_dvd);
        let assign2250_e1748: f64 = (assign2250_e1746 * var_inv_ucrit);
        let assign2250_e1749: f64 = (assign2250_e1743 + assign2250_e1748);
        let assign2250_e1750: f64 = (var_tmp1 * assign2250_e1749);
        var_dleq_dvd = assign2250_e1750;
        var_dleq_dvd_dn0 = ((var_tmp1_dn0 * assign2250_e1749) + (var_tmp1 * ((-var_ddeltal_dvd_dn0) + (var_dvip_dvd_dn0 * var_inv_ucrit))));
        var_dleq_dvd_dn1 = ((var_tmp1_dn1 * assign2250_e1749) + (var_tmp1 * ((-var_ddeltal_dvd_dn1) + (var_dvip_dvd_dn1 * var_inv_ucrit))));
        var_dleq_dvd_dn2 = ((var_tmp1_dn2 * assign2250_e1749) + (var_tmp1 * ((-var_ddeltal_dvd_dn2) + (var_dvip_dvd_dn2 * var_inv_ucrit))));
        var_dleq_dvd_dn3 = ((var_tmp1_dn3 * assign2250_e1749) + (var_tmp1 * ((-var_ddeltal_dvd_dn3) + (var_dvip_dvd_dn3 * var_inv_ucrit))));

        let assign2260_e1753: f64 = (-var_ddeltal_dvs);
        let assign2260_e1755: f64 = (-0.5);
        let assign2260_e1757: f64 = (assign2260_e1755 + var_dvip_dvs);
        let assign2260_e1759: f64 = (assign2260_e1757 * var_inv_ucrit);
        let assign2260_e1760: f64 = (assign2260_e1753 + assign2260_e1759);
        let assign2260_e1761: f64 = (var_tmp1 * assign2260_e1760);
        var_dleq_dvs = assign2260_e1761;
        var_dleq_dvs_dn0 = ((var_tmp1_dn0 * assign2260_e1760) + (var_tmp1 * ((-var_ddeltal_dvs_dn0) + (var_dvip_dvs_dn0 * var_inv_ucrit))));
        var_dleq_dvs_dn1 = ((var_tmp1_dn1 * assign2260_e1760) + (var_tmp1 * ((-var_ddeltal_dvs_dn1) + (var_dvip_dvs_dn1 * var_inv_ucrit))));
        var_dleq_dvs_dn2 = ((var_tmp1_dn2 * assign2260_e1760) + (var_tmp1 * ((-var_ddeltal_dvs_dn2) + (var_dvip_dvs_dn2 * var_inv_ucrit))));
        var_dleq_dvs_dn3 = ((var_tmp1_dn3 * assign2260_e1760) + (var_tmp1 * ((-var_ddeltal_dvs_dn3) + (var_dvip_dvs_dn3 * var_inv_ucrit))));

        let assign2280_e1772: f64 = (var_dir_dv * var_inv_vt);
        var_tmp1 = assign2280_e1772;
        var_tmp1_dn0 = (var_dir_dv_dn0 * var_inv_vt);
        var_tmp1_dn1 = (var_dir_dv_dn1 * var_inv_vt);
        var_tmp1_dn2 = (var_dir_dv_dn2 * var_inv_vt);
        var_tmp1_dn3 = (var_dir_dv_dn3 * var_inv_vt);

        let assign2290_e1776: f64 = (var_dvp_dvd - 1.0);
        let assign2290_e1777: f64 = (var_tmp1 * assign2290_e1776);
        var_dir_dvd = assign2290_e1777;
        var_dir_dvd_dn0 = ((var_tmp1_dn0 * assign2290_e1776) + (var_tmp1 * var_dvp_dvd_dn0));
        var_dir_dvd_dn1 = ((var_tmp1_dn1 * assign2290_e1776) + (var_tmp1 * var_dvp_dvd_dn1));
        var_dir_dvd_dn2 = ((var_tmp1_dn2 * assign2290_e1776) + (var_tmp1 * var_dvp_dvd_dn2));
        var_dir_dvd_dn3 = ((var_tmp1_dn3 * assign2290_e1776) + (var_tmp1 * var_dvp_dvd_dn3));

        let assign2300_e1780: f64 = (var_tmp1 * var_dvp_dvs);
        var_dir_dvs = assign2300_e1780;
        var_dir_dvs_dn0 = ((var_tmp1_dn0 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn0));
        var_dir_dvs_dn1 = ((var_tmp1_dn1 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn1));
        var_dir_dvs_dn2 = ((var_tmp1_dn2 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn2));
        var_dir_dvs_dn3 = ((var_tmp1_dn3 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn3));

        let assign2320_e1786: f64 = (1.0 + var_n_1);
        let assign2320_e1787: f64 = (-assign2320_e1786);
        let assign2320_e1789: f64 = (assign2320_e1787 * var_vt);
        let assign2320_e1791: f64 = (assign2320_e1789 * 0.66666666);
        let assign2320_e1793: f64 = (assign2320_e1791 / var_sif_sir_2);
        var_tmp1 = assign2320_e1793;
        var_tmp1_dn0 = ((((((-var_n_1_dn0) * var_vt) * 0.66666666) * var_sif_sir_2) - (assign2320_e1791 * var_sif_sir_2_dn0)) / (var_sif_sir_2 * var_sif_sir_2));
        var_tmp1_dn1 = ((((((-var_n_1_dn1) * var_vt) * 0.66666666) * var_sif_sir_2) - (assign2320_e1791 * var_sif_sir_2_dn1)) / (var_sif_sir_2 * var_sif_sir_2));
        var_tmp1_dn2 = ((((((-var_n_1_dn2) * var_vt) * 0.66666666) * var_sif_sir_2) - (assign2320_e1791 * var_sif_sir_2_dn2)) / (var_sif_sir_2 * var_sif_sir_2));
        var_tmp1_dn3 = ((((((-var_n_1_dn3) * var_vt) * 0.66666666) * var_sif_sir_2) - (assign2320_e1791 * var_sif_sir_2_dn3)) / (var_sif_sir_2 * var_sif_sir_2));

        let assign2330_e1798: f64 = (2.0 * var_sir);
        let assign2330_e1799: f64 = (var_sif + assign2330_e1798);
        let assign2330_e1800: f64 = (var_tmp1 * assign2330_e1799);
        var_tmp2 = assign2330_e1800;
        var_tmp2_dn0 = ((var_tmp1_dn0 * assign2330_e1799) + (var_tmp1 * (var_sif_dn0 + (2.0 * var_sir_dn0))));
        var_tmp2_dn1 = ((var_tmp1_dn1 * assign2330_e1799) + (var_tmp1 * (var_sif_dn1 + (2.0 * var_sir_dn1))));
        var_tmp2_dn2 = ((var_tmp1_dn2 * assign2330_e1799) + (var_tmp1 * (var_sif_dn2 + (2.0 * var_sir_dn2))));
        var_tmp2_dn3 = ((var_tmp1_dn3 * assign2330_e1799) + (var_tmp1 * (var_sif_dn3 + (2.0 * var_sir_dn3))));

        let assign2340_e1805: f64 = (2.0 * var_sif);
        let assign2340_e1806: f64 = (var_sir + assign2340_e1805);
        let assign2340_e1807: f64 = (var_tmp1 * assign2340_e1806);
        var_tmp3 = assign2340_e1807;
        var_tmp3_dn0 = ((var_tmp1_dn0 * assign2340_e1806) + (var_tmp1 * (var_sir_dn0 + (2.0 * var_sif_dn0))));
        var_tmp3_dn1 = ((var_tmp1_dn1 * assign2340_e1806) + (var_tmp1 * (var_sir_dn1 + (2.0 * var_sif_dn1))));
        var_tmp3_dn2 = ((var_tmp1_dn2 * assign2340_e1806) + (var_tmp1 * (var_sir_dn2 + (2.0 * var_sif_dn2))));
        var_tmp3_dn3 = ((var_tmp1_dn3 * assign2340_e1806) + (var_tmp1 * (var_sir_dn3 + (2.0 * var_sif_dn3))));

        let assign2350_e1809: f64 = (-var_n_1);
        let assign2350_e1811: f64 = (assign2350_e1809 * var_qi);
        let assign2350_e1814: f64 = (2.0 + var_n_1);
        let assign2350_e1816: f64 = (assign2350_e1814 + var_n_1);
        let assign2350_e1818: f64 = (assign2350_e1816 * var_vp_phi_eps);
        let assign2350_e1819: f64 = (assign2350_e1811 / assign2350_e1818);
        var_tmp1 = assign2350_e1819;
        var_tmp1_dn0 = ((((((-var_n_1_dn0) * var_qi) + (assign2350_e1809 * var_qi_dn0)) * assign2350_e1818) - (assign2350_e1811 * (((var_n_1_dn0 + var_n_1_dn0) * var_vp_phi_eps) + (assign2350_e1816 * var_vp_phi_eps_dn0)))) / (assign2350_e1818 * assign2350_e1818));
        var_tmp1_dn1 = ((((((-var_n_1_dn1) * var_qi) + (assign2350_e1809 * var_qi_dn1)) * assign2350_e1818) - (assign2350_e1811 * (((var_n_1_dn1 + var_n_1_dn1) * var_vp_phi_eps) + (assign2350_e1816 * var_vp_phi_eps_dn1)))) / (assign2350_e1818 * assign2350_e1818));
        var_tmp1_dn2 = ((((((-var_n_1_dn2) * var_qi) + (assign2350_e1809 * var_qi_dn2)) * assign2350_e1818) - (assign2350_e1811 * (((var_n_1_dn2 + var_n_1_dn2) * var_vp_phi_eps) + (assign2350_e1816 * var_vp_phi_eps_dn2)))) / (assign2350_e1818 * assign2350_e1818));
        var_tmp1_dn3 = ((((((-var_n_1_dn3) * var_qi) + (assign2350_e1809 * var_qi_dn3)) * assign2350_e1818) - (assign2350_e1811 * (((var_n_1_dn3 + var_n_1_dn3) * var_vp_phi_eps) + (assign2350_e1816 * var_vp_phi_eps_dn3)))) / (assign2350_e1818 * assign2350_e1818));

        let assign2360_e1822: f64 = (var_tmp1 * var_dvp_dvd);
        let assign2360_e1825: f64 = (var_tmp2 * var_dif_dvd);
        let assign2360_e1826: f64 = (assign2360_e1822 + assign2360_e1825);
        let assign2360_e1829: f64 = (var_tmp3 * var_dir_dvd);
        let assign2360_e1830: f64 = (assign2360_e1826 + assign2360_e1829);
        var_dqi_dvd = assign2360_e1830;
        var_dqi_dvd_dn0 = ((((var_tmp1_dn0 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn0)) + ((var_tmp2_dn0 * var_dif_dvd) + (var_tmp2 * var_dif_dvd_dn0))) + ((var_tmp3_dn0 * var_dir_dvd) + (var_tmp3 * var_dir_dvd_dn0)));
        var_dqi_dvd_dn1 = ((((var_tmp1_dn1 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn1)) + ((var_tmp2_dn1 * var_dif_dvd) + (var_tmp2 * var_dif_dvd_dn1))) + ((var_tmp3_dn1 * var_dir_dvd) + (var_tmp3 * var_dir_dvd_dn1)));
        var_dqi_dvd_dn2 = ((((var_tmp1_dn2 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn2)) + ((var_tmp2_dn2 * var_dif_dvd) + (var_tmp2 * var_dif_dvd_dn2))) + ((var_tmp3_dn2 * var_dir_dvd) + (var_tmp3 * var_dir_dvd_dn2)));
        var_dqi_dvd_dn3 = ((((var_tmp1_dn3 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn3)) + ((var_tmp2_dn3 * var_dif_dvd) + (var_tmp2 * var_dif_dvd_dn3))) + ((var_tmp3_dn3 * var_dir_dvd) + (var_tmp3 * var_dir_dvd_dn3)));

        let assign2370_e1833: f64 = (var_tmp1 * var_dvp_dvs);
        let assign2370_e1836: f64 = (var_tmp2 * var_dif_dvs);
        let assign2370_e1837: f64 = (assign2370_e1833 + assign2370_e1836);
        let assign2370_e1840: f64 = (var_tmp3 * var_dir_dvs);
        let assign2370_e1841: f64 = (assign2370_e1837 + assign2370_e1840);
        var_dqi_dvs = assign2370_e1841;
        var_dqi_dvs_dn0 = ((((var_tmp1_dn0 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn0)) + ((var_tmp2_dn0 * var_dif_dvs) + (var_tmp2 * var_dif_dvs_dn0))) + ((var_tmp3_dn0 * var_dir_dvs) + (var_tmp3 * var_dir_dvs_dn0)));
        var_dqi_dvs_dn1 = ((((var_tmp1_dn1 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn1)) + ((var_tmp2_dn1 * var_dif_dvs) + (var_tmp2 * var_dif_dvs_dn1))) + ((var_tmp3_dn1 * var_dir_dvs) + (var_tmp3 * var_dir_dvs_dn1)));
        var_dqi_dvs_dn2 = ((((var_tmp1_dn2 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn2)) + ((var_tmp2_dn2 * var_dif_dvs) + (var_tmp2 * var_dif_dvs_dn2))) + ((var_tmp3_dn2 * var_dir_dvs) + (var_tmp3 * var_dir_dvs_dn2)));
        var_dqi_dvs_dn3 = ((((var_tmp1_dn3 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn3)) + ((var_tmp2_dn3 * var_dif_dvs) + (var_tmp2 * var_dif_dvs_dn3))) + ((var_tmp3_dn3 * var_dir_dvs) + (var_tmp3 * var_dir_dvs_dn3)));

        let assign2390_e1855: f64 = (1.0 + var_n_1);
        let assign2390_e1860: f64 = (1.0 + var_n_1);
        let assign2390_e1861: f64 = (2.0 * assign2390_e1860);
        let assign2390_e1863: f64 = (assign2390_e1861 * var_vp_phi_eps);
        let assign2390_e1864: f64 = (var_qi / assign2390_e1863);
        let assign2390_e1865: f64 = (assign2390_e1855 - assign2390_e1864);
        var_tmp1 = assign2390_e1865;
        var_tmp1_dn0 = (var_n_1_dn0 - (((var_qi_dn0 * assign2390_e1863) - (var_qi * (((2.0 * var_n_1_dn0) * var_vp_phi_eps) + (assign2390_e1861 * var_vp_phi_eps_dn0)))) / (assign2390_e1863 * assign2390_e1863)));
        var_tmp1_dn1 = (var_n_1_dn1 - (((var_qi_dn1 * assign2390_e1863) - (var_qi * (((2.0 * var_n_1_dn1) * var_vp_phi_eps) + (assign2390_e1861 * var_vp_phi_eps_dn1)))) / (assign2390_e1863 * assign2390_e1863)));
        var_tmp1_dn2 = (var_n_1_dn2 - (((var_qi_dn2 * assign2390_e1863) - (var_qi * (((2.0 * var_n_1_dn2) * var_vp_phi_eps) + (assign2390_e1861 * var_vp_phi_eps_dn2)))) / (assign2390_e1863 * assign2390_e1863)));
        var_tmp1_dn3 = (var_n_1_dn3 - (((var_qi_dn3 * assign2390_e1863) - (var_qi * (((2.0 * var_n_1_dn3) * var_vp_phi_eps) + (assign2390_e1861 * var_vp_phi_eps_dn3)))) / (assign2390_e1863 * assign2390_e1863)));

        let assign2400_e1867: f64 = (-var_n_1_n);
        let assign2400_e1870: f64 = (var_tmp1 * var_dvp_dvd);
        let assign2400_e1872: f64 = (assign2400_e1870 + var_dqi_dvd);
        let assign2400_e1873: f64 = (assign2400_e1867 * assign2400_e1872);
        var_dqb_dvd = assign2400_e1873;
        var_dqb_dvd_dn0 = (((-var_n_1_n_dn0) * assign2400_e1872) + (assign2400_e1867 * (((var_tmp1_dn0 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn0)) + var_dqi_dvd_dn0)));
        var_dqb_dvd_dn1 = (((-var_n_1_n_dn1) * assign2400_e1872) + (assign2400_e1867 * (((var_tmp1_dn1 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn1)) + var_dqi_dvd_dn1)));
        var_dqb_dvd_dn2 = (((-var_n_1_n_dn2) * assign2400_e1872) + (assign2400_e1867 * (((var_tmp1_dn2 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn2)) + var_dqi_dvd_dn2)));
        var_dqb_dvd_dn3 = (((-var_n_1_n_dn3) * assign2400_e1872) + (assign2400_e1867 * (((var_tmp1_dn3 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn3)) + var_dqi_dvd_dn3)));

        let assign2410_e1875: f64 = (-var_n_1_n);
        let assign2410_e1878: f64 = (var_tmp1 * var_dvp_dvs);
        let assign2410_e1880: f64 = (assign2410_e1878 + var_dqi_dvs);
        let assign2410_e1881: f64 = (assign2410_e1875 * assign2410_e1880);
        var_dqb_dvs = assign2410_e1881;
        var_dqb_dvs_dn0 = (((-var_n_1_n_dn0) * assign2410_e1880) + (assign2410_e1875 * (((var_tmp1_dn0 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn0)) + var_dqi_dvs_dn0)));
        var_dqb_dvs_dn1 = (((-var_n_1_n_dn1) * assign2410_e1880) + (assign2410_e1875 * (((var_tmp1_dn1 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn1)) + var_dqi_dvs_dn1)));
        var_dqb_dvs_dn2 = (((-var_n_1_n_dn2) * assign2410_e1880) + (assign2410_e1875 * (((var_tmp1_dn2 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn2)) + var_dqi_dvs_dn2)));
        var_dqb_dvs_dn3 = (((-var_n_1_n_dn3) * assign2410_e1880) + (assign2410_e1875 * (((var_tmp1_dn3 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn3)) + var_dqi_dvs_dn3)));

        let assign2430_e1892: f64 = if p.p22 == 0.0 { 1.0 } else { 0.0 };
        var_guard18 = assign2430_e1892;

        let (assign2440_e1902, assign2440_e1902_d_n0, assign2440_e1902_d_n1, assign2440_e1902_d_n2, assign2440_e1902_d_n3,) = {
    if (var_guard18 != 0.0) {
        let assign2440_e1896: f64 = (p.p21 * var_vpprime);
        let assign2440_e1899: f64 = (var_theta_vp_1 * var_sqrt_vp_vt);
        let assign2440_e1900: f64 = (assign2440_e1896 / assign2440_e1899);
        (assign2440_e1900, ((((p.p21 * var_vpprime_dn0) * assign2440_e1899) - (assign2440_e1896 * ((var_theta_vp_1_dn0 * var_sqrt_vp_vt) + (var_theta_vp_1 * var_sqrt_vp_vt_dn0)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * var_vpprime_dn1) * assign2440_e1899) - (assign2440_e1896 * ((var_theta_vp_1_dn1 * var_sqrt_vp_vt) + (var_theta_vp_1 * var_sqrt_vp_vt_dn1)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * var_vpprime_dn2) * assign2440_e1899) - (assign2440_e1896 * ((var_theta_vp_1_dn2 * var_sqrt_vp_vt) + (var_theta_vp_1 * var_sqrt_vp_vt_dn2)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * var_vpprime_dn3) * assign2440_e1899) - (assign2440_e1896 * ((var_theta_vp_1_dn3 * var_sqrt_vp_vt) + (var_theta_vp_1 * var_sqrt_vp_vt_dn3)))) / (assign2440_e1899 * assign2440_e1899)),)
    } else {
        (var_tmp1, var_tmp1_dn0, var_tmp1_dn1, var_tmp1_dn2, var_tmp1_dn3,)
    }
};
        var_tmp1 = assign2440_e1902;
        var_tmp1_dn0 = assign2440_e1902_d_n0;
        var_tmp1_dn1 = assign2440_e1902_d_n1;
        var_tmp1_dn2 = assign2440_e1902_d_n2;
        var_tmp1_dn3 = assign2440_e1902_d_n3;

        let (assign2450_e1908, assign2450_e1908_d_n0, assign2450_e1908_d_n1, assign2450_e1908_d_n2, assign2450_e1908_d_n3,) = {
    if (var_guard18 != 0.0) {
        let assign2450_e1906: f64 = (var_tmp1 * var_dvp_dvd);
        (assign2450_e1906, ((var_tmp1_dn0 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn0)), ((var_tmp1_dn1 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn1)), ((var_tmp1_dn2 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn2)), ((var_tmp1_dn3 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn3)),)
    } else {
        (var_dvpprime_dvd, var_dvpprime_dvd_dn0, var_dvpprime_dvd_dn1, var_dvpprime_dvd_dn2, var_dvpprime_dvd_dn3,)
    }
};
        var_dvpprime_dvd = assign2450_e1908;
        var_dvpprime_dvd_dn0 = assign2450_e1908_d_n0;
        var_dvpprime_dvd_dn1 = assign2450_e1908_d_n1;
        var_dvpprime_dvd_dn2 = assign2450_e1908_d_n2;
        var_dvpprime_dvd_dn3 = assign2450_e1908_d_n3;

        *var_ddeltal_dvd_slot = var_ddeltal_dvd;
        *var_ddeltal_dvd_dn0_slot = var_ddeltal_dvd_dn0;
        *var_ddeltal_dvd_dn1_slot = var_ddeltal_dvd_dn1;
        *var_ddeltal_dvd_dn2_slot = var_ddeltal_dvd_dn2;
        *var_ddeltal_dvd_dn3_slot = var_ddeltal_dvd_dn3;
        *var_ddeltal_dvs_slot = var_ddeltal_dvs;
        *var_ddeltal_dvs_dn0_slot = var_ddeltal_dvs_dn0;
        *var_ddeltal_dvs_dn1_slot = var_ddeltal_dvs_dn1;
        *var_ddeltal_dvs_dn2_slot = var_ddeltal_dvs_dn2;
        *var_ddeltal_dvs_dn3_slot = var_ddeltal_dvs_dn3;
        *var_ddeltav_dvd_slot = var_ddeltav_dvd;
        *var_ddeltav_dvd_dn0_slot = var_ddeltav_dvd_dn0;
        *var_ddeltav_dvd_dn1_slot = var_ddeltav_dvd_dn1;
        *var_ddeltav_dvd_dn2_slot = var_ddeltav_dvd_dn2;
        *var_ddeltav_dvd_dn3_slot = var_ddeltav_dvd_dn3;
        *var_ddeltav_dvs_slot = var_ddeltav_dvs;
        *var_ddeltav_dvs_dn0_slot = var_ddeltav_dvs_dn0;
        *var_ddeltav_dvs_dn1_slot = var_ddeltav_dvs_dn1;
        *var_ddeltav_dvs_dn2_slot = var_ddeltav_dvs_dn2;
        *var_ddeltav_dvs_dn3_slot = var_ddeltav_dvs_dn3;
        *var_dgammaprime_dvd_slot = var_dgammaprime_dvd;
        *var_dgammaprime_dvd_dn0_slot = var_dgammaprime_dvd_dn0;
        *var_dgammaprime_dvd_dn1_slot = var_dgammaprime_dvd_dn1;
        *var_dgammaprime_dvd_dn2_slot = var_dgammaprime_dvd_dn2;
        *var_dgammaprime_dvd_dn3_slot = var_dgammaprime_dvd_dn3;
        *var_dgammaprime_dvs_slot = var_dgammaprime_dvs;
        *var_dgammaprime_dvs_dn0_slot = var_dgammaprime_dvs_dn0;
        *var_dgammaprime_dvs_dn1_slot = var_dgammaprime_dvs_dn1;
        *var_dgammaprime_dvs_dn2_slot = var_dgammaprime_dvs_dn2;
        *var_dgammaprime_dvs_dn3_slot = var_dgammaprime_dvs_dn3;
        *var_dif_dvd_slot = var_dif_dvd;
        *var_dif_dvd_dn0_slot = var_dif_dvd_dn0;
        *var_dif_dvd_dn1_slot = var_dif_dvd_dn1;
        *var_dif_dvd_dn2_slot = var_dif_dvd_dn2;
        *var_dif_dvd_dn3_slot = var_dif_dvd_dn3;
        *var_dif_dvs_slot = var_dif_dvs;
        *var_dif_dvs_dn0_slot = var_dif_dvs_dn0;
        *var_dif_dvs_dn1_slot = var_dif_dvs_dn1;
        *var_dif_dvs_dn2_slot = var_dif_dvs_dn2;
        *var_dif_dvs_dn3_slot = var_dif_dvs_dn3;
        *var_dir_dvd_slot = var_dir_dvd;
        *var_dir_dvd_dn0_slot = var_dir_dvd_dn0;
        *var_dir_dvd_dn1_slot = var_dir_dvd_dn1;
        *var_dir_dvd_dn2_slot = var_dir_dvd_dn2;
        *var_dir_dvd_dn3_slot = var_dir_dvd_dn3;
        *var_dir_dvs_slot = var_dir_dvs;
        *var_dir_dvs_dn0_slot = var_dir_dvs_dn0;
        *var_dir_dvs_dn1_slot = var_dir_dvs_dn1;
        *var_dir_dvs_dn2_slot = var_dir_dvs_dn2;
        *var_dir_dvs_dn3_slot = var_dir_dvs_dn3;
        *var_dirprime_dvd_slot = var_dirprime_dvd;
        *var_dirprime_dvd_dn0_slot = var_dirprime_dvd_dn0;
        *var_dirprime_dvd_dn1_slot = var_dirprime_dvd_dn1;
        *var_dirprime_dvd_dn2_slot = var_dirprime_dvd_dn2;
        *var_dirprime_dvd_dn3_slot = var_dirprime_dvd_dn3;
        *var_dirprime_dvs_slot = var_dirprime_dvs;
        *var_dirprime_dvs_dn0_slot = var_dirprime_dvs_dn0;
        *var_dirprime_dvs_dn1_slot = var_dirprime_dvs_dn1;
        *var_dirprime_dvs_dn2_slot = var_dirprime_dvs_dn2;
        *var_dirprime_dvs_dn3_slot = var_dirprime_dvs_dn3;
        *var_dleq_dvd_slot = var_dleq_dvd;
        *var_dleq_dvd_dn0_slot = var_dleq_dvd_dn0;
        *var_dleq_dvd_dn1_slot = var_dleq_dvd_dn1;
        *var_dleq_dvd_dn2_slot = var_dleq_dvd_dn2;
        *var_dleq_dvd_dn3_slot = var_dleq_dvd_dn3;
        *var_dleq_dvs_slot = var_dleq_dvs;
        *var_dleq_dvs_dn0_slot = var_dleq_dvs_dn0;
        *var_dleq_dvs_dn1_slot = var_dleq_dvs_dn1;
        *var_dleq_dvs_dn2_slot = var_dleq_dvs_dn2;
        *var_dleq_dvs_dn3_slot = var_dleq_dvs_dn3;
        *var_dqb_dvd_slot = var_dqb_dvd;
        *var_dqb_dvd_dn0_slot = var_dqb_dvd_dn0;
        *var_dqb_dvd_dn1_slot = var_dqb_dvd_dn1;
        *var_dqb_dvd_dn2_slot = var_dqb_dvd_dn2;
        *var_dqb_dvd_dn3_slot = var_dqb_dvd_dn3;
        *var_dqb_dvs_slot = var_dqb_dvs;
        *var_dqb_dvs_dn0_slot = var_dqb_dvs_dn0;
        *var_dqb_dvs_dn1_slot = var_dqb_dvs_dn1;
        *var_dqb_dvs_dn2_slot = var_dqb_dvs_dn2;
        *var_dqb_dvs_dn3_slot = var_dqb_dvs_dn3;
        *var_dqi_dvd_slot = var_dqi_dvd;
        *var_dqi_dvd_dn0_slot = var_dqi_dvd_dn0;
        *var_dqi_dvd_dn1_slot = var_dqi_dvd_dn1;
        *var_dqi_dvd_dn2_slot = var_dqi_dvd_dn2;
        *var_dqi_dvd_dn3_slot = var_dqi_dvd_dn3;
        *var_dqi_dvs_slot = var_dqi_dvs;
        *var_dqi_dvs_dn0_slot = var_dqi_dvs_dn0;
        *var_dqi_dvs_dn1_slot = var_dqi_dvs_dn1;
        *var_dqi_dvs_dn2_slot = var_dqi_dvs_dn2;
        *var_dqi_dvs_dn3_slot = var_dqi_dvs_dn3;
        *var_dvdss_dvd_slot = var_dvdss_dvd;
        *var_dvdss_dvd_dn0_slot = var_dvdss_dvd_dn0;
        *var_dvdss_dvd_dn1_slot = var_dvdss_dvd_dn1;
        *var_dvdss_dvd_dn2_slot = var_dvdss_dvd_dn2;
        *var_dvdss_dvd_dn3_slot = var_dvdss_dvd_dn3;
        *var_dvdss_dvs_slot = var_dvdss_dvs;
        *var_dvdss_dvs_dn0_slot = var_dvdss_dvs_dn0;
        *var_dvdss_dvs_dn1_slot = var_dvdss_dvs_dn1;
        *var_dvdss_dvs_dn2_slot = var_dvdss_dvs_dn2;
        *var_dvdss_dvs_dn3_slot = var_dvdss_dvs_dn3;
        *var_dvdssprime_dvd_slot = var_dvdssprime_dvd;
        *var_dvdssprime_dvd_dn0_slot = var_dvdssprime_dvd_dn0;
        *var_dvdssprime_dvd_dn1_slot = var_dvdssprime_dvd_dn1;
        *var_dvdssprime_dvd_dn2_slot = var_dvdssprime_dvd_dn2;
        *var_dvdssprime_dvd_dn3_slot = var_dvdssprime_dvd_dn3;
        *var_dvdssprime_dvs_slot = var_dvdssprime_dvs;
        *var_dvdssprime_dvs_dn0_slot = var_dvdssprime_dvs_dn0;
        *var_dvdssprime_dvs_dn1_slot = var_dvdssprime_dvs_dn1;
        *var_dvdssprime_dvs_dn2_slot = var_dvdssprime_dvs_dn2;
        *var_dvdssprime_dvs_dn3_slot = var_dvdssprime_dvs_dn3;
        *var_dvip_dvd_slot = var_dvip_dvd;
        *var_dvip_dvd_dn0_slot = var_dvip_dvd_dn0;
        *var_dvip_dvd_dn1_slot = var_dvip_dvd_dn1;
        *var_dvip_dvd_dn2_slot = var_dvip_dvd_dn2;
        *var_dvip_dvd_dn3_slot = var_dvip_dvd_dn3;
        *var_dvip_dvs_slot = var_dvip_dvs;
        *var_dvip_dvs_dn0_slot = var_dvip_dvs_dn0;
        *var_dvip_dvs_dn1_slot = var_dvip_dvs_dn1;
        *var_dvip_dvs_dn2_slot = var_dvip_dvs_dn2;
        *var_dvip_dvs_dn3_slot = var_dvip_dvs_dn3;
        *var_dvp_dvd_slot = var_dvp_dvd;
        *var_dvp_dvd_dn0_slot = var_dvp_dvd_dn0;
        *var_dvp_dvd_dn1_slot = var_dvp_dvd_dn1;
        *var_dvp_dvd_dn2_slot = var_dvp_dvd_dn2;
        *var_dvp_dvd_dn3_slot = var_dvp_dvd_dn3;
        *var_dvp_dvs_slot = var_dvp_dvs;
        *var_dvp_dvs_dn0_slot = var_dvp_dvs_dn0;
        *var_dvp_dvs_dn1_slot = var_dvp_dvs_dn1;
        *var_dvp_dvs_dn2_slot = var_dvp_dvs_dn2;
        *var_dvp_dvs_dn3_slot = var_dvp_dvs_dn3;
        *var_dvpprime_dvd_slot = var_dvpprime_dvd;
        *var_dvpprime_dvd_dn0_slot = var_dvpprime_dvd_dn0;
        *var_dvpprime_dvd_dn1_slot = var_dvpprime_dvd_dn1;
        *var_dvpprime_dvd_dn2_slot = var_dvpprime_dvd_dn2;
        *var_dvpprime_dvd_dn3_slot = var_dvpprime_dvd_dn3;
        *var_guard18_slot = var_guard18;
        *var_if_ir_slot = var_if_ir;
        *var_if_ir_dn0_slot = var_if_ir_dn0;
        *var_if_ir_dn1_slot = var_if_ir_dn1;
        *var_if_ir_dn2_slot = var_if_ir_dn2;
        *var_if_ir_dn3_slot = var_if_ir_dn3;
        *var_ispec_slot = var_ispec;
        *var_ispec_dn0_slot = var_ispec_dn0;
        *var_ispec_dn1_slot = var_ispec_dn1;
        *var_ispec_dn2_slot = var_ispec_dn2;
        *var_ispec_dn3_slot = var_ispec_dn3;
        *var_n_slot = var_n;
        *var_n_dn0_slot = var_n_dn0;
        *var_n_dn1_slot = var_n_dn1;
        *var_n_dn2_slot = var_n_dn2;
        *var_n_dn3_slot = var_n_dn3;
        *var_tmp1_slot = var_tmp1;
        *var_tmp1_dn0_slot = var_tmp1_dn0;
        *var_tmp1_dn1_slot = var_tmp1_dn1;
        *var_tmp1_dn2_slot = var_tmp1_dn2;
        *var_tmp1_dn3_slot = var_tmp1_dn3;
        *var_tmp2_slot = var_tmp2;
        *var_tmp2_dn0_slot = var_tmp2_dn0;
        *var_tmp2_dn1_slot = var_tmp2_dn1;
        *var_tmp2_dn2_slot = var_tmp2_dn2;
        *var_tmp2_dn3_slot = var_tmp2_dn3;
        *var_tmp3_slot = var_tmp3;
        *var_tmp3_dn0_slot = var_tmp3_dn0;
        *var_tmp3_dn1_slot = var_tmp3_dn1;
        *var_tmp3_dn2_slot = var_tmp3_dn2;
        *var_tmp3_dn3_slot = var_tmp3_dn3;
    }

    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_deltat: f64,
        var_dif_dvd: f64,
        var_dif_dvd_dn0: f64,
        var_dif_dvd_dn1: f64,
        var_dif_dvd_dn2: f64,
        var_dif_dvd_dn3: f64,
        var_dif_dvs: f64,
        var_dif_dvs_dn0: f64,
        var_dif_dvs_dn1: f64,
        var_dif_dvs_dn2: f64,
        var_dif_dvs_dn3: f64,
        var_dirprime_dvd: f64,
        var_dirprime_dvd_dn0: f64,
        var_dirprime_dvd_dn1: f64,
        var_dirprime_dvd_dn2: f64,
        var_dirprime_dvd_dn3: f64,
        var_dirprime_dvs: f64,
        var_dirprime_dvs_dn0: f64,
        var_dirprime_dvs_dn1: f64,
        var_dirprime_dvs_dn2: f64,
        var_dirprime_dvs_dn3: f64,
        var_dleq_dvd: f64,
        var_dleq_dvd_dn0: f64,
        var_dleq_dvd_dn1: f64,
        var_dleq_dvd_dn2: f64,
        var_dleq_dvd_dn3: f64,
        var_dleq_dvs: f64,
        var_dleq_dvs_dn0: f64,
        var_dleq_dvs_dn1: f64,
        var_dleq_dvs_dn2: f64,
        var_dleq_dvs_dn3: f64,
        var_dqb_dvd: f64,
        var_dqb_dvd_dn0: f64,
        var_dqb_dvd_dn1: f64,
        var_dqb_dvd_dn2: f64,
        var_dqb_dvd_dn3: f64,
        var_dqb_dvs: f64,
        var_dqb_dvs_dn0: f64,
        var_dqb_dvs_dn1: f64,
        var_dqb_dvs_dn2: f64,
        var_dqb_dvs_dn3: f64,
        var_dqi_dvd: f64,
        var_dqi_dvd_dn0: f64,
        var_dqi_dvd_dn1: f64,
        var_dqi_dvd_dn2: f64,
        var_dqi_dvd_dn3: f64,
        var_dqi_dvs: f64,
        var_dqi_dvs_dn0: f64,
        var_dqi_dvs_dn1: f64,
        var_dqi_dvs_dn2: f64,
        var_dqi_dvs_dn3: f64,
        var_dvp_dvd: f64,
        var_dvp_dvd_dn0: f64,
        var_dvp_dvd_dn1: f64,
        var_dvp_dvd_dn2: f64,
        var_dvp_dvd_dn3: f64,
        var_dvp_dvs: f64,
        var_dvp_dvs_dn0: f64,
        var_dvp_dvs_dn1: f64,
        var_dvp_dvs_dn2: f64,
        var_dvp_dvs_dn3: f64,
        var_dvpprime_dvd: f64,
        var_dvpprime_dvd_dn0: f64,
        var_dvpprime_dvd_dn1: f64,
        var_dvpprime_dvd_dn2: f64,
        var_dvpprime_dvd_dn3: f64,
        var_e0_q_1: f64,
        var_e0_q_1_dn0: f64,
        var_e0_q_1_dn1: f64,
        var_e0_q_1_dn2: f64,
        var_e0_q_1_dn3: f64,
        var_eta_qi: f64,
        var_gamma_s: f64,
        var_gammaprime: f64,
        var_gammaprime_dn0: f64,
        var_gammaprime_dn1: f64,
        var_gammaprime_dn2: f64,
        var_gammaprime_dn3: f64,
        var_guard18: f64,
        var_if_ir: f64,
        var_if_ir_dn0: f64,
        var_if_ir_dn1: f64,
        var_if_ir_dn2: f64,
        var_if_ir_dn3: f64,
        var_ispec: f64,
        var_ispec_dn0: f64,
        var_ispec_dn1: f64,
        var_ispec_dn2: f64,
        var_ispec_dn3: f64,
        var_leff: f64,
        var_mode: f64,
        var_n: f64,
        var_n_dn0: f64,
        var_n_dn1: f64,
        var_n_dn2: f64,
        var_n_dn3: f64,
        var_phi_t: f64,
        var_phi_t_dn0: f64,
        var_phi_t_dn1: f64,
        var_phi_t_dn2: f64,
        var_phi_t_dn3: f64,
        var_sif: f64,
        var_sif2: f64,
        var_sif2_dn0: f64,
        var_sif2_dn1: f64,
        var_sif2_dn2: f64,
        var_sif2_dn3: f64,
        var_sif_dn0: f64,
        var_sif_dn1: f64,
        var_sif_dn2: f64,
        var_sif_dn3: f64,
        var_sif_sir_2: f64,
        var_sif_sir_2_dn0: f64,
        var_sif_sir_2_dn1: f64,
        var_sif_sir_2_dn2: f64,
        var_sif_sir_2_dn3: f64,
        var_sir: f64,
        var_sir2: f64,
        var_sir2_dn0: f64,
        var_sir2_dn1: f64,
        var_sir2_dn2: f64,
        var_sir2_dn3: f64,
        var_sir_dn0: f64,
        var_sir_dn1: f64,
        var_sir_dn2: f64,
        var_sir_dn3: f64,
        var_sqrt_phi_vp: f64,
        var_sqrt_phi_vp_2: f64,
        var_sqrt_phi_vp_2_dn0: f64,
        var_sqrt_phi_vp_2_dn1: f64,
        var_sqrt_phi_vp_2_dn2: f64,
        var_sqrt_phi_vp_2_dn3: f64,
        var_sqrt_phi_vp_dn0: f64,
        var_sqrt_phi_vp_dn1: f64,
        var_sqrt_phi_vp_dn2: f64,
        var_sqrt_phi_vp_dn3: f64,
        var_t0: f64,
        var_vgprime: f64,
        var_vgprime_dn0: f64,
        var_vgprime_dn1: f64,
        var_vgprime_dn2: f64,
        var_vgprime_dn3: f64,
        var_vgstar: f64,
        var_vgstar_dn0: f64,
        var_vgstar_dn1: f64,
        var_vgstar_dn2: f64,
        var_vgstar_dn3: f64,
        var_vp: f64,
        var_vp_dn0: f64,
        var_vp_dn1: f64,
        var_vp_dn2: f64,
        var_vp_dn3: f64,
        var_vt: f64,
        var_vt_4: f64,
        var_weff: f64,
        var_ad_i_slot: &mut f64,
        var_as_i_slot: &mut f64,
        var_cj_t_slot: &mut f64,
        var_cjsw_t_slot: &mut f64,
        var_cjswg_t_slot: &mut f64,
        var_csb_d_slot: &mut f64,
        var_csb_d_dn0_slot: &mut f64,
        var_csb_d_dn3_slot: &mut f64,
        var_cssw_d_slot: &mut f64,
        var_cssw_d_dn0_slot: &mut f64,
        var_cssw_d_dn3_slot: &mut f64,
        var_csswg_d_slot: &mut f64,
        var_csswg_d_dn0_slot: &mut f64,
        var_csswg_d_dn3_slot: &mut f64,
        var_dbeta_dvd_slot: &mut f64,
        var_dbeta_dvd_dn0_slot: &mut f64,
        var_dbeta_dvd_dn1_slot: &mut f64,
        var_dbeta_dvd_dn2_slot: &mut f64,
        var_dbeta_dvd_dn3_slot: &mut f64,
        var_dbeta_dvs_slot: &mut f64,
        var_dbeta_dvs_dn0_slot: &mut f64,
        var_dbeta_dvs_dn1_slot: &mut f64,
        var_dbeta_dvs_dn2_slot: &mut f64,
        var_dbeta_dvs_dn3_slot: &mut f64,
        var_ddt_qd_slot: &mut f64,
        var_ddt_qd_dn0_slot: &mut f64,
        var_ddt_qd_dn1_slot: &mut f64,
        var_ddt_qd_dn2_slot: &mut f64,
        var_ddt_qd_dn3_slot: &mut f64,
        var_ddt_qs_slot: &mut f64,
        var_ddt_qs_dn0_slot: &mut f64,
        var_ddt_qs_dn1_slot: &mut f64,
        var_ddt_qs_dn2_slot: &mut f64,
        var_ddt_qs_dn3_slot: &mut f64,
        var_dn_dvd_slot: &mut f64,
        var_dn_dvd_dn0_slot: &mut f64,
        var_dn_dvd_dn1_slot: &mut f64,
        var_dn_dvd_dn2_slot: &mut f64,
        var_dn_dvd_dn3_slot: &mut f64,
        var_dn_dvs_slot: &mut f64,
        var_dn_dvs_dn0_slot: &mut f64,
        var_dn_dvs_dn1_slot: &mut f64,
        var_dn_dvs_dn2_slot: &mut f64,
        var_dn_dvs_dn3_slot: &mut f64,
        var_dvpprime_dvs_slot: &mut f64,
        var_dvpprime_dvs_dn0_slot: &mut f64,
        var_dvpprime_dvs_dn1_slot: &mut f64,
        var_dvpprime_dvs_dn2_slot: &mut f64,
        var_dvpprime_dvs_dn3_slot: &mut f64,
        var_gds_slot: &mut f64,
        var_gds_dn0_slot: &mut f64,
        var_gds_dn1_slot: &mut f64,
        var_gds_dn2_slot: &mut f64,
        var_gds_dn3_slot: &mut f64,
        var_gms_slot: &mut f64,
        var_gms_dn0_slot: &mut f64,
        var_gms_dn1_slot: &mut f64,
        var_gms_dn2_slot: &mut f64,
        var_gms_dn3_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard24_slot: &mut f64,
        var_guard25_slot: &mut f64,
        var_guard26_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_n_vt_cox_slot: &mut f64,
        var_n_vt_cox_dn0_slot: &mut f64,
        var_n_vt_cox_dn1_slot: &mut f64,
        var_n_vt_cox_dn2_slot: &mut f64,
        var_n_vt_cox_dn3_slot: &mut f64,
        var_pb_t_slot: &mut f64,
        var_pbsw_t_slot: &mut f64,
        var_pbswg_t_slot: &mut f64,
        var_pd_i_slot: &mut f64,
        var_ps_i_slot: &mut f64,
        var_qb_1_slot: &mut f64,
        var_qb_1_dn0_slot: &mut f64,
        var_qb_1_dn1_slot: &mut f64,
        var_qb_1_dn2_slot: &mut f64,
        var_qb_1_dn3_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn1_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn3_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn0_slot: &mut f64,
        var_qg_dn1_slot: &mut f64,
        var_qg_dn2_slot: &mut f64,
        var_qg_dn3_slot: &mut f64,
        var_qi_1_slot: &mut f64,
        var_qi_1_dn0_slot: &mut f64,
        var_qi_1_dn1_slot: &mut f64,
        var_qi_1_dn2_slot: &mut f64,
        var_qi_1_dn3_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn0_slot: &mut f64,
        var_qs_dn1_slot: &mut f64,
        var_qs_dn2_slot: &mut f64,
        var_qs_dn3_slot: &mut f64,
        var_rdeff_slot: &mut f64,
        var_rseff_slot: &mut f64,
        var_sif3_slot: &mut f64,
        var_sif3_dn0_slot: &mut f64,
        var_sif3_dn1_slot: &mut f64,
        var_sif3_dn2_slot: &mut f64,
        var_sif3_dn3_slot: &mut f64,
        var_sir3_slot: &mut f64,
        var_sir3_dn0_slot: &mut f64,
        var_sir3_dn1_slot: &mut f64,
        var_sir3_dn2_slot: &mut f64,
        var_sir3_dn3_slot: &mut f64,
        var_sqrt_phi_vp2_2_slot: &mut f64,
        var_sqrt_phi_vp2_2_dn0_slot: &mut f64,
        var_sqrt_phi_vp2_2_dn1_slot: &mut f64,
        var_sqrt_phi_vp2_2_dn2_slot: &mut f64,
        var_sqrt_phi_vp2_2_dn3_slot: &mut f64,
        var_tmp1_slot: &mut f64,
        var_tmp1_dn0_slot: &mut f64,
        var_tmp1_dn1_slot: &mut f64,
        var_tmp1_dn2_slot: &mut f64,
        var_tmp1_dn3_slot: &mut f64,
        var_v_di_b_slot: &mut f64,
        var_v_di_b_dn0_slot: &mut f64,
        var_v_di_b_dn3_slot: &mut f64,
        var_v_si_b_slot: &mut f64,
        var_v_si_b_dn2_slot: &mut f64,
        var_v_si_b_dn3_slot: &mut f64,
        var_wlcox_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_ad_i: f64 = *var_ad_i_slot;
        let mut var_as_i: f64 = *var_as_i_slot;
        let mut var_cj_t: f64 = *var_cj_t_slot;
        let mut var_cjsw_t: f64 = *var_cjsw_t_slot;
        let mut var_cjswg_t: f64 = *var_cjswg_t_slot;
        let mut var_csb_d: f64 = *var_csb_d_slot;
        let mut var_csb_d_dn0: f64 = *var_csb_d_dn0_slot;
        let mut var_csb_d_dn3: f64 = *var_csb_d_dn3_slot;
        let mut var_cssw_d: f64 = *var_cssw_d_slot;
        let mut var_cssw_d_dn0: f64 = *var_cssw_d_dn0_slot;
        let mut var_cssw_d_dn3: f64 = *var_cssw_d_dn3_slot;
        let mut var_csswg_d: f64 = *var_csswg_d_slot;
        let mut var_csswg_d_dn0: f64 = *var_csswg_d_dn0_slot;
        let mut var_csswg_d_dn3: f64 = *var_csswg_d_dn3_slot;
        let mut var_dbeta_dvd: f64 = *var_dbeta_dvd_slot;
        let mut var_dbeta_dvd_dn0: f64 = *var_dbeta_dvd_dn0_slot;
        let mut var_dbeta_dvd_dn1: f64 = *var_dbeta_dvd_dn1_slot;
        let mut var_dbeta_dvd_dn2: f64 = *var_dbeta_dvd_dn2_slot;
        let mut var_dbeta_dvd_dn3: f64 = *var_dbeta_dvd_dn3_slot;
        let mut var_dbeta_dvs: f64 = *var_dbeta_dvs_slot;
        let mut var_dbeta_dvs_dn0: f64 = *var_dbeta_dvs_dn0_slot;
        let mut var_dbeta_dvs_dn1: f64 = *var_dbeta_dvs_dn1_slot;
        let mut var_dbeta_dvs_dn2: f64 = *var_dbeta_dvs_dn2_slot;
        let mut var_dbeta_dvs_dn3: f64 = *var_dbeta_dvs_dn3_slot;
        let mut var_ddt_qd: f64 = *var_ddt_qd_slot;
        let mut var_ddt_qd_dn0: f64 = *var_ddt_qd_dn0_slot;
        let mut var_ddt_qd_dn1: f64 = *var_ddt_qd_dn1_slot;
        let mut var_ddt_qd_dn2: f64 = *var_ddt_qd_dn2_slot;
        let mut var_ddt_qd_dn3: f64 = *var_ddt_qd_dn3_slot;
        let mut var_ddt_qs: f64 = *var_ddt_qs_slot;
        let mut var_ddt_qs_dn0: f64 = *var_ddt_qs_dn0_slot;
        let mut var_ddt_qs_dn1: f64 = *var_ddt_qs_dn1_slot;
        let mut var_ddt_qs_dn2: f64 = *var_ddt_qs_dn2_slot;
        let mut var_ddt_qs_dn3: f64 = *var_ddt_qs_dn3_slot;
        let mut var_dn_dvd: f64 = *var_dn_dvd_slot;
        let mut var_dn_dvd_dn0: f64 = *var_dn_dvd_dn0_slot;
        let mut var_dn_dvd_dn1: f64 = *var_dn_dvd_dn1_slot;
        let mut var_dn_dvd_dn2: f64 = *var_dn_dvd_dn2_slot;
        let mut var_dn_dvd_dn3: f64 = *var_dn_dvd_dn3_slot;
        let mut var_dn_dvs: f64 = *var_dn_dvs_slot;
        let mut var_dn_dvs_dn0: f64 = *var_dn_dvs_dn0_slot;
        let mut var_dn_dvs_dn1: f64 = *var_dn_dvs_dn1_slot;
        let mut var_dn_dvs_dn2: f64 = *var_dn_dvs_dn2_slot;
        let mut var_dn_dvs_dn3: f64 = *var_dn_dvs_dn3_slot;
        let mut var_dvpprime_dvs: f64 = *var_dvpprime_dvs_slot;
        let mut var_dvpprime_dvs_dn0: f64 = *var_dvpprime_dvs_dn0_slot;
        let mut var_dvpprime_dvs_dn1: f64 = *var_dvpprime_dvs_dn1_slot;
        let mut var_dvpprime_dvs_dn2: f64 = *var_dvpprime_dvs_dn2_slot;
        let mut var_dvpprime_dvs_dn3: f64 = *var_dvpprime_dvs_dn3_slot;
        let mut var_gds: f64 = *var_gds_slot;
        let mut var_gds_dn0: f64 = *var_gds_dn0_slot;
        let mut var_gds_dn1: f64 = *var_gds_dn1_slot;
        let mut var_gds_dn2: f64 = *var_gds_dn2_slot;
        let mut var_gds_dn3: f64 = *var_gds_dn3_slot;
        let mut var_gms: f64 = *var_gms_slot;
        let mut var_gms_dn0: f64 = *var_gms_dn0_slot;
        let mut var_gms_dn1: f64 = *var_gms_dn1_slot;
        let mut var_gms_dn2: f64 = *var_gms_dn2_slot;
        let mut var_gms_dn3: f64 = *var_gms_dn3_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard24: f64 = *var_guard24_slot;
        let mut var_guard25: f64 = *var_guard25_slot;
        let mut var_guard26: f64 = *var_guard26_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_n_vt_cox: f64 = *var_n_vt_cox_slot;
        let mut var_n_vt_cox_dn0: f64 = *var_n_vt_cox_dn0_slot;
        let mut var_n_vt_cox_dn1: f64 = *var_n_vt_cox_dn1_slot;
        let mut var_n_vt_cox_dn2: f64 = *var_n_vt_cox_dn2_slot;
        let mut var_n_vt_cox_dn3: f64 = *var_n_vt_cox_dn3_slot;
        let mut var_pb_t: f64 = *var_pb_t_slot;
        let mut var_pbsw_t: f64 = *var_pbsw_t_slot;
        let mut var_pbswg_t: f64 = *var_pbswg_t_slot;
        let mut var_pd_i: f64 = *var_pd_i_slot;
        let mut var_ps_i: f64 = *var_ps_i_slot;
        let mut var_qb_1: f64 = *var_qb_1_slot;
        let mut var_qb_1_dn0: f64 = *var_qb_1_dn0_slot;
        let mut var_qb_1_dn1: f64 = *var_qb_1_dn1_slot;
        let mut var_qb_1_dn2: f64 = *var_qb_1_dn2_slot;
        let mut var_qb_1_dn3: f64 = *var_qb_1_dn3_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn1: f64 = *var_qd_dn1_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn3: f64 = *var_qd_dn3_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn0: f64 = *var_qg_dn0_slot;
        let mut var_qg_dn1: f64 = *var_qg_dn1_slot;
        let mut var_qg_dn2: f64 = *var_qg_dn2_slot;
        let mut var_qg_dn3: f64 = *var_qg_dn3_slot;
        let mut var_qi_1: f64 = *var_qi_1_slot;
        let mut var_qi_1_dn0: f64 = *var_qi_1_dn0_slot;
        let mut var_qi_1_dn1: f64 = *var_qi_1_dn1_slot;
        let mut var_qi_1_dn2: f64 = *var_qi_1_dn2_slot;
        let mut var_qi_1_dn3: f64 = *var_qi_1_dn3_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn0: f64 = *var_qs_dn0_slot;
        let mut var_qs_dn1: f64 = *var_qs_dn1_slot;
        let mut var_qs_dn2: f64 = *var_qs_dn2_slot;
        let mut var_qs_dn3: f64 = *var_qs_dn3_slot;
        let mut var_rdeff: f64 = *var_rdeff_slot;
        let mut var_rseff: f64 = *var_rseff_slot;
        let mut var_sif3: f64 = *var_sif3_slot;
        let mut var_sif3_dn0: f64 = *var_sif3_dn0_slot;
        let mut var_sif3_dn1: f64 = *var_sif3_dn1_slot;
        let mut var_sif3_dn2: f64 = *var_sif3_dn2_slot;
        let mut var_sif3_dn3: f64 = *var_sif3_dn3_slot;
        let mut var_sir3: f64 = *var_sir3_slot;
        let mut var_sir3_dn0: f64 = *var_sir3_dn0_slot;
        let mut var_sir3_dn1: f64 = *var_sir3_dn1_slot;
        let mut var_sir3_dn2: f64 = *var_sir3_dn2_slot;
        let mut var_sir3_dn3: f64 = *var_sir3_dn3_slot;
        let mut var_sqrt_phi_vp2_2: f64 = *var_sqrt_phi_vp2_2_slot;
        let mut var_sqrt_phi_vp2_2_dn0: f64 = *var_sqrt_phi_vp2_2_dn0_slot;
        let mut var_sqrt_phi_vp2_2_dn1: f64 = *var_sqrt_phi_vp2_2_dn1_slot;
        let mut var_sqrt_phi_vp2_2_dn2: f64 = *var_sqrt_phi_vp2_2_dn2_slot;
        let mut var_sqrt_phi_vp2_2_dn3: f64 = *var_sqrt_phi_vp2_2_dn3_slot;
        let mut var_tmp1: f64 = *var_tmp1_slot;
        let mut var_tmp1_dn0: f64 = *var_tmp1_dn0_slot;
        let mut var_tmp1_dn1: f64 = *var_tmp1_dn1_slot;
        let mut var_tmp1_dn2: f64 = *var_tmp1_dn2_slot;
        let mut var_tmp1_dn3: f64 = *var_tmp1_dn3_slot;
        let mut var_v_di_b: f64 = *var_v_di_b_slot;
        let mut var_v_di_b_dn0: f64 = *var_v_di_b_dn0_slot;
        let mut var_v_di_b_dn3: f64 = *var_v_di_b_dn3_slot;
        let mut var_v_si_b: f64 = *var_v_si_b_slot;
        let mut var_v_si_b_dn2: f64 = *var_v_si_b_dn2_slot;
        let mut var_v_si_b_dn3: f64 = *var_v_si_b_dn3_slot;
        let mut var_wlcox: f64 = *var_wlcox_slot;

        let (assign2460_e1914, assign2460_e1914_d_n0, assign2460_e1914_d_n1, assign2460_e1914_d_n2, assign2460_e1914_d_n3,) = {
    if (var_guard18 != 0.0) {
        let assign2460_e1912: f64 = (var_tmp1 * var_dvp_dvs);
        (assign2460_e1912, ((var_tmp1_dn0 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn0)), ((var_tmp1_dn1 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn1)), ((var_tmp1_dn2 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn2)), ((var_tmp1_dn3 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn3)),)
    } else {
        (var_dvpprime_dvs, var_dvpprime_dvs_dn0, var_dvpprime_dvs_dn1, var_dvpprime_dvs_dn2, var_dvpprime_dvs_dn3,)
    }
};
        var_dvpprime_dvs = assign2460_e1914;
        var_dvpprime_dvs_dn0 = assign2460_e1914_d_n0;
        var_dvpprime_dvs_dn1 = assign2460_e1914_d_n1;
        var_dvpprime_dvs_dn2 = assign2460_e1914_d_n2;
        var_dvpprime_dvs_dn3 = assign2460_e1914_d_n3;

        let (assign2480_e1927, assign2480_e1927_d_n0, assign2480_e1927_d_n1, assign2480_e1927_d_n2, assign2480_e1927_d_n3,) = {
    if (var_guard18 != 0.0) {
        let assign2480_e1923: f64 = (-var_dleq_dvd);
        let assign2480_e1925: f64 = (assign2480_e1923 - var_dvpprime_dvd);
        (assign2480_e1925, ((-var_dleq_dvd_dn0) - var_dvpprime_dvd_dn0), ((-var_dleq_dvd_dn1) - var_dvpprime_dvd_dn1), ((-var_dleq_dvd_dn2) - var_dvpprime_dvd_dn2), ((-var_dleq_dvd_dn3) - var_dvpprime_dvd_dn3),)
    } else {
        (var_dbeta_dvd, var_dbeta_dvd_dn0, var_dbeta_dvd_dn1, var_dbeta_dvd_dn2, var_dbeta_dvd_dn3,)
    }
};
        var_dbeta_dvd = assign2480_e1927;
        var_dbeta_dvd_dn0 = assign2480_e1927_d_n0;
        var_dbeta_dvd_dn1 = assign2480_e1927_d_n1;
        var_dbeta_dvd_dn2 = assign2480_e1927_d_n2;
        var_dbeta_dvd_dn3 = assign2480_e1927_d_n3;

        let (assign2490_e1934, assign2490_e1934_d_n0, assign2490_e1934_d_n1, assign2490_e1934_d_n2, assign2490_e1934_d_n3,) = {
    if (var_guard18 != 0.0) {
        let assign2490_e1930: f64 = (-var_dleq_dvs);
        let assign2490_e1932: f64 = (assign2490_e1930 - var_dvpprime_dvs);
        (assign2490_e1932, ((-var_dleq_dvs_dn0) - var_dvpprime_dvs_dn0), ((-var_dleq_dvs_dn1) - var_dvpprime_dvs_dn1), ((-var_dleq_dvs_dn2) - var_dvpprime_dvs_dn2), ((-var_dleq_dvs_dn3) - var_dvpprime_dvs_dn3),)
    } else {
        (var_dbeta_dvs, var_dbeta_dvs_dn0, var_dbeta_dvs_dn1, var_dbeta_dvs_dn2, var_dbeta_dvs_dn3,)
    }
};
        var_dbeta_dvs = assign2490_e1934;
        var_dbeta_dvs_dn0 = assign2490_e1934_d_n0;
        var_dbeta_dvs_dn1 = assign2490_e1934_d_n1;
        var_dbeta_dvs_dn2 = assign2490_e1934_d_n2;
        var_dbeta_dvs_dn3 = assign2490_e1934_d_n3;

        let (assign2510_e1948, assign2510_e1948_d_n0, assign2510_e1948_d_n1, assign2510_e1948_d_n2, assign2510_e1948_d_n3,) = {
    if (var_guard18 == 0.0) {
        let assign2510_e1946: f64 = (var_t0 / var_e0_q_1);
        (assign2510_e1946, (-((var_t0 * var_e0_q_1_dn0) / (var_e0_q_1 * var_e0_q_1))), (-((var_t0 * var_e0_q_1_dn1) / (var_e0_q_1 * var_e0_q_1))), (-((var_t0 * var_e0_q_1_dn2) / (var_e0_q_1 * var_e0_q_1))), (-((var_t0 * var_e0_q_1_dn3) / (var_e0_q_1 * var_e0_q_1))),)
    } else {
        (var_tmp1, var_tmp1_dn0, var_tmp1_dn1, var_tmp1_dn2, var_tmp1_dn3,)
    }
};
        var_tmp1 = assign2510_e1948;
        var_tmp1_dn0 = assign2510_e1948_d_n0;
        var_tmp1_dn1 = assign2510_e1948_d_n1;
        var_tmp1_dn2 = assign2510_e1948_d_n2;
        var_tmp1_dn3 = assign2510_e1948_d_n3;

        let (assign2520_e1962, assign2520_e1962_d_n0, assign2520_e1962_d_n1, assign2520_e1962_d_n2, assign2520_e1962_d_n3,) = {
    if (var_guard18 == 0.0) {
        let assign2520_e1952: f64 = (-var_dleq_dvd);
        let assign2520_e1957: f64 = (var_eta_qi * var_dqi_dvd);
        let assign2520_e1958: f64 = (var_dqb_dvd + assign2520_e1957);
        let assign2520_e1959: f64 = (var_tmp1 * assign2520_e1958);
        let assign2520_e1960: f64 = (assign2520_e1952 + assign2520_e1959);
        (assign2520_e1960, ((-var_dleq_dvd_dn0) + ((var_tmp1_dn0 * assign2520_e1958) + (var_tmp1 * (var_dqb_dvd_dn0 + (var_eta_qi * var_dqi_dvd_dn0))))), ((-var_dleq_dvd_dn1) + ((var_tmp1_dn1 * assign2520_e1958) + (var_tmp1 * (var_dqb_dvd_dn1 + (var_eta_qi * var_dqi_dvd_dn1))))), ((-var_dleq_dvd_dn2) + ((var_tmp1_dn2 * assign2520_e1958) + (var_tmp1 * (var_dqb_dvd_dn2 + (var_eta_qi * var_dqi_dvd_dn2))))), ((-var_dleq_dvd_dn3) + ((var_tmp1_dn3 * assign2520_e1958) + (var_tmp1 * (var_dqb_dvd_dn3 + (var_eta_qi * var_dqi_dvd_dn3))))),)
    } else {
        (var_dbeta_dvd, var_dbeta_dvd_dn0, var_dbeta_dvd_dn1, var_dbeta_dvd_dn2, var_dbeta_dvd_dn3,)
    }
};
        var_dbeta_dvd = assign2520_e1962;
        var_dbeta_dvd_dn0 = assign2520_e1962_d_n0;
        var_dbeta_dvd_dn1 = assign2520_e1962_d_n1;
        var_dbeta_dvd_dn2 = assign2520_e1962_d_n2;
        var_dbeta_dvd_dn3 = assign2520_e1962_d_n3;

        let (assign2530_e1976, assign2530_e1976_d_n0, assign2530_e1976_d_n1, assign2530_e1976_d_n2, assign2530_e1976_d_n3,) = {
    if (var_guard18 == 0.0) {
        let assign2530_e1966: f64 = (-var_dleq_dvs);
        let assign2530_e1971: f64 = (var_eta_qi * var_dqi_dvs);
        let assign2530_e1972: f64 = (var_dqb_dvs + assign2530_e1971);
        let assign2530_e1973: f64 = (var_tmp1 * assign2530_e1972);
        let assign2530_e1974: f64 = (assign2530_e1966 + assign2530_e1973);
        (assign2530_e1974, ((-var_dleq_dvs_dn0) + ((var_tmp1_dn0 * assign2530_e1972) + (var_tmp1 * (var_dqb_dvs_dn0 + (var_eta_qi * var_dqi_dvs_dn0))))), ((-var_dleq_dvs_dn1) + ((var_tmp1_dn1 * assign2530_e1972) + (var_tmp1 * (var_dqb_dvs_dn1 + (var_eta_qi * var_dqi_dvs_dn1))))), ((-var_dleq_dvs_dn2) + ((var_tmp1_dn2 * assign2530_e1972) + (var_tmp1 * (var_dqb_dvs_dn2 + (var_eta_qi * var_dqi_dvs_dn2))))), ((-var_dleq_dvs_dn3) + ((var_tmp1_dn3 * assign2530_e1972) + (var_tmp1 * (var_dqb_dvs_dn3 + (var_eta_qi * var_dqi_dvs_dn3))))),)
    } else {
        (var_dbeta_dvs, var_dbeta_dvs_dn0, var_dbeta_dvs_dn1, var_dbeta_dvs_dn2, var_dbeta_dvs_dn3,)
    }
};
        var_dbeta_dvs = assign2530_e1976;
        var_dbeta_dvs_dn0 = assign2530_e1976_d_n0;
        var_dbeta_dvs_dn1 = assign2530_e1976_d_n1;
        var_dbeta_dvs_dn2 = assign2530_e1976_d_n2;
        var_dbeta_dvs_dn3 = assign2530_e1976_d_n3;

        let assign2550_e1992: f64 = (-var_gamma_s);
        let assign2550_e1995: f64 = (4.0 * var_n);
        let assign2550_e1997: f64 = (assign2550_e1995 * var_sqrt_phi_vp);
        let assign2550_e2000: f64 = (var_phi_t + var_vp);
        let assign2550_e2002: f64 = (assign2550_e2000 + var_vt_4);
        let assign2550_e2003: f64 = (assign2550_e1997 * assign2550_e2002);
        let assign2550_e2004: f64 = (assign2550_e1992 / assign2550_e2003);
        var_tmp1 = assign2550_e2004;
        var_tmp1_dn0 = (-((assign2550_e1992 * (((((4.0 * var_n_dn0) * var_sqrt_phi_vp) + (assign2550_e1995 * var_sqrt_phi_vp_dn0)) * assign2550_e2002) + (assign2550_e1997 * (var_phi_t_dn0 + var_vp_dn0)))) / (assign2550_e2003 * assign2550_e2003)));
        var_tmp1_dn1 = (-((assign2550_e1992 * (((((4.0 * var_n_dn1) * var_sqrt_phi_vp) + (assign2550_e1995 * var_sqrt_phi_vp_dn1)) * assign2550_e2002) + (assign2550_e1997 * (var_phi_t_dn1 + var_vp_dn1)))) / (assign2550_e2003 * assign2550_e2003)));
        var_tmp1_dn2 = (-((assign2550_e1992 * (((((4.0 * var_n_dn2) * var_sqrt_phi_vp) + (assign2550_e1995 * var_sqrt_phi_vp_dn2)) * assign2550_e2002) + (assign2550_e1997 * (var_phi_t_dn2 + var_vp_dn2)))) / (assign2550_e2003 * assign2550_e2003)));
        var_tmp1_dn3 = (-((assign2550_e1992 * (((((4.0 * var_n_dn3) * var_sqrt_phi_vp) + (assign2550_e1995 * var_sqrt_phi_vp_dn3)) * assign2550_e2002) + (assign2550_e1997 * (var_phi_t_dn3 + var_vp_dn3)))) / (assign2550_e2003 * assign2550_e2003)));

        let assign2560_e2007: f64 = (var_tmp1 * var_dvp_dvd);
        var_dn_dvd = assign2560_e2007;
        var_dn_dvd_dn0 = ((var_tmp1_dn0 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn0));
        var_dn_dvd_dn1 = ((var_tmp1_dn1 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn1));
        var_dn_dvd_dn2 = ((var_tmp1_dn2 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn2));
        var_dn_dvd_dn3 = ((var_tmp1_dn3 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn3));

        let assign2570_e2010: f64 = (var_tmp1 * var_dvp_dvs);
        var_dn_dvs = assign2570_e2010;
        var_dn_dvs_dn0 = ((var_tmp1_dn0 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn0));
        var_dn_dvs_dn1 = ((var_tmp1_dn1 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn1));
        var_dn_dvs_dn2 = ((var_tmp1_dn2 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn2));
        var_dn_dvs_dn3 = ((var_tmp1_dn3 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn3));

        let assign2590_e2017: f64 = (var_dn_dvd + var_dbeta_dvd);
        let assign2590_e2019: f64 = (assign2590_e2017 * var_if_ir);
        let assign2590_e2021: f64 = (assign2590_e2019 + var_dif_dvd);
        let assign2590_e2023: f64 = (assign2590_e2021 - var_dirprime_dvd);
        let assign2590_e2024: f64 = (var_ispec * assign2590_e2023);
        var_gds = assign2590_e2024;
        var_gds_dn0 = ((var_ispec_dn0 * assign2590_e2023) + (var_ispec * (((((var_dn_dvd_dn0 + var_dbeta_dvd_dn0) * var_if_ir) + (assign2590_e2017 * var_if_ir_dn0)) + var_dif_dvd_dn0) - var_dirprime_dvd_dn0)));
        var_gds_dn1 = ((var_ispec_dn1 * assign2590_e2023) + (var_ispec * (((((var_dn_dvd_dn1 + var_dbeta_dvd_dn1) * var_if_ir) + (assign2590_e2017 * var_if_ir_dn1)) + var_dif_dvd_dn1) - var_dirprime_dvd_dn1)));
        var_gds_dn2 = ((var_ispec_dn2 * assign2590_e2023) + (var_ispec * (((((var_dn_dvd_dn2 + var_dbeta_dvd_dn2) * var_if_ir) + (assign2590_e2017 * var_if_ir_dn2)) + var_dif_dvd_dn2) - var_dirprime_dvd_dn2)));
        var_gds_dn3 = ((var_ispec_dn3 * assign2590_e2023) + (var_ispec * (((((var_dn_dvd_dn3 + var_dbeta_dvd_dn3) * var_if_ir) + (assign2590_e2017 * var_if_ir_dn3)) + var_dif_dvd_dn3) - var_dirprime_dvd_dn3)));

        let assign2600_e2026: f64 = (-var_ispec);
        let assign2600_e2029: f64 = (var_dn_dvs + var_dbeta_dvs);
        let assign2600_e2031: f64 = (assign2600_e2029 * var_if_ir);
        let assign2600_e2033: f64 = (assign2600_e2031 + var_dif_dvs);
        let assign2600_e2035: f64 = (assign2600_e2033 - var_dirprime_dvs);
        let assign2600_e2036: f64 = (assign2600_e2026 * assign2600_e2035);
        var_gms = assign2600_e2036;
        var_gms_dn0 = (((-var_ispec_dn0) * assign2600_e2035) + (assign2600_e2026 * (((((var_dn_dvs_dn0 + var_dbeta_dvs_dn0) * var_if_ir) + (assign2600_e2029 * var_if_ir_dn0)) + var_dif_dvs_dn0) - var_dirprime_dvs_dn0)));
        var_gms_dn1 = (((-var_ispec_dn1) * assign2600_e2035) + (assign2600_e2026 * (((((var_dn_dvs_dn1 + var_dbeta_dvs_dn1) * var_if_ir) + (assign2600_e2029 * var_if_ir_dn1)) + var_dif_dvs_dn1) - var_dirprime_dvs_dn1)));
        var_gms_dn2 = (((-var_ispec_dn2) * assign2600_e2035) + (assign2600_e2026 * (((((var_dn_dvs_dn2 + var_dbeta_dvs_dn2) * var_if_ir) + (assign2600_e2029 * var_if_ir_dn2)) + var_dif_dvs_dn2) - var_dirprime_dvs_dn2)));
        var_gms_dn3 = (((-var_ispec_dn3) * assign2600_e2035) + (assign2600_e2026 * (((((var_dn_dvs_dn3 + var_dbeta_dvs_dn3) * var_if_ir) + (assign2600_e2029 * var_if_ir_dn3)) + var_dif_dvs_dn3) - var_dirprime_dvs_dn3)));

        let assign2630_e2055: f64 = (p.p36 * p.p37);
        let assign2630_e2058: f64 = (var_weff - p.p27);
        let assign2630_e2059: f64 = (assign2630_e2055 / assign2630_e2058);
        var_rseff = assign2630_e2059;

        let assign2640_e2062: f64 = (p.p36 * p.p37);
        let assign2640_e2065: f64 = (var_weff - p.p27);
        let assign2640_e2066: f64 = (assign2640_e2062 / assign2640_e2065);
        var_rdeff = assign2640_e2066;

        let assign2650_e2071: f64 = (var_gms * var_rseff);
        let assign2650_e2072: f64 = (1.0 + assign2650_e2071);
        let assign2650_e2075: f64 = (var_gds * var_rdeff);
        let assign2650_e2076: f64 = (assign2650_e2072 + assign2650_e2075);
        let assign2650_e2077: f64 = (1.0 / assign2650_e2076);
        var_tmp1 = assign2650_e2077;
        var_tmp1_dn0 = (-(((var_gms_dn0 * var_rseff) + (var_gds_dn0 * var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        var_tmp1_dn1 = (-(((var_gms_dn1 * var_rseff) + (var_gds_dn1 * var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        var_tmp1_dn2 = (-(((var_gms_dn2 * var_rseff) + (var_gds_dn2 * var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        var_tmp1_dn3 = (-(((var_gms_dn3 * var_rseff) + (var_gds_dn3 * var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));

        let assign2800_e2163: f64 = (var_weff * var_leff);
        let assign2800_e2165: f64 = (assign2800_e2163 * p.p13);
        var_wlcox = assign2800_e2165;

        let assign2810_e2168: f64 = (var_sif * var_sif2);
        var_sif3 = assign2810_e2168;
        var_sif3_dn0 = ((var_sif_dn0 * var_sif2) + (var_sif * var_sif2_dn0));
        var_sif3_dn1 = ((var_sif_dn1 * var_sif2) + (var_sif * var_sif2_dn1));
        var_sif3_dn2 = ((var_sif_dn2 * var_sif2) + (var_sif * var_sif2_dn2));
        var_sif3_dn3 = ((var_sif_dn3 * var_sif2) + (var_sif * var_sif2_dn3));

        let assign2820_e2171: f64 = (var_sir * var_sir2);
        var_sir3 = assign2820_e2171;
        var_sir3_dn0 = ((var_sir_dn0 * var_sir2) + (var_sir * var_sir2_dn0));
        var_sir3_dn1 = ((var_sir_dn1 * var_sir2) + (var_sir * var_sir2_dn1));
        var_sir3_dn2 = ((var_sir_dn2 * var_sir2) + (var_sir * var_sir2_dn2));
        var_sir3_dn3 = ((var_sir_dn3 * var_sir2) + (var_sir * var_sir2_dn3));

        let assign2830_e2175: f64 = (0.5 * var_vp);
        let assign2830_e2176: f64 = (var_phi_t + assign2830_e2175);
        let assign2830_e2177: f64 = (assign2830_e2176).sqrt();
        var_tmp1 = assign2830_e2177;
        var_tmp1_dn0 = ((var_phi_t_dn0 + (0.5 * var_vp_dn0)) / (2.0 * assign2830_e2177));
        var_tmp1_dn1 = ((var_phi_t_dn1 + (0.5 * var_vp_dn1)) / (2.0 * assign2830_e2177));
        var_tmp1_dn2 = ((var_phi_t_dn2 + (0.5 * var_vp_dn2)) / (2.0 * assign2830_e2177));
        var_tmp1_dn3 = ((var_phi_t_dn3 + (0.5 * var_vp_dn3)) / (2.0 * assign2830_e2177));

        let assign2840_e2180: f64 = (var_tmp1 + var_tmp1);
        var_sqrt_phi_vp2_2 = assign2840_e2180;
        var_sqrt_phi_vp2_2_dn0 = (var_tmp1_dn0 + var_tmp1_dn0);
        var_sqrt_phi_vp2_2_dn1 = (var_tmp1_dn1 + var_tmp1_dn1);
        var_sqrt_phi_vp2_2_dn2 = (var_tmp1_dn2 + var_tmp1_dn2);
        var_sqrt_phi_vp2_2_dn3 = (var_tmp1_dn3 + var_tmp1_dn3);

        let assign2850_e2184: f64 = (var_gammaprime / var_sqrt_phi_vp2_2);
        let assign2850_e2185: f64 = (1.0 + assign2850_e2184);
        let assign2850_e2187: f64 = (assign2850_e2185 * var_vt);
        let assign2850_e2189: f64 = (assign2850_e2187 * var_wlcox);
        var_n_vt_cox = assign2850_e2189;
        var_n_vt_cox_dn0 = (((((var_gammaprime_dn0 * var_sqrt_phi_vp2_2) - (var_gammaprime * var_sqrt_phi_vp2_2_dn0)) / (var_sqrt_phi_vp2_2 * var_sqrt_phi_vp2_2)) * var_vt) * var_wlcox);
        var_n_vt_cox_dn1 = (((((var_gammaprime_dn1 * var_sqrt_phi_vp2_2) - (var_gammaprime * var_sqrt_phi_vp2_2_dn1)) / (var_sqrt_phi_vp2_2 * var_sqrt_phi_vp2_2)) * var_vt) * var_wlcox);
        var_n_vt_cox_dn2 = (((((var_gammaprime_dn2 * var_sqrt_phi_vp2_2) - (var_gammaprime * var_sqrt_phi_vp2_2_dn2)) / (var_sqrt_phi_vp2_2 * var_sqrt_phi_vp2_2)) * var_vt) * var_wlcox);
        var_n_vt_cox_dn3 = (((((var_gammaprime_dn3 * var_sqrt_phi_vp2_2) - (var_gammaprime * var_sqrt_phi_vp2_2_dn3)) / (var_sqrt_phi_vp2_2 * var_sqrt_phi_vp2_2)) * var_vt) * var_wlcox);

        let assign2860_e2191: f64 = (-var_n_vt_cox);
        let assign2860_e2195: f64 = (3.0 * var_sir3);
        let assign2860_e2198: f64 = (6.0 * var_sir2);
        let assign2860_e2200: f64 = (assign2860_e2198 * var_sif);
        let assign2860_e2201: f64 = (assign2860_e2195 + assign2860_e2200);
        let assign2860_e2204: f64 = (4.0 * var_sir);
        let assign2860_e2206: f64 = (assign2860_e2204 * var_sif2);
        let assign2860_e2207: f64 = (assign2860_e2201 + assign2860_e2206);
        let assign2860_e2210: f64 = (2.0 * var_sif3);
        let assign2860_e2211: f64 = (assign2860_e2207 + assign2860_e2210);
        let assign2860_e2212: f64 = (0.266666666 * assign2860_e2211);
        let assign2860_e2214: f64 = (assign2860_e2212 / var_sif_sir_2);
        let assign2860_e2216: f64 = (assign2860_e2214 - 0.5);
        let assign2860_e2217: f64 = (assign2860_e2191 * assign2860_e2216);
        var_qd = assign2860_e2217;
        var_qd_dn0 = (((-var_n_vt_cox_dn0) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * var_sir3_dn0) + (((6.0 * var_sir2_dn0) * var_sif) + (assign2860_e2198 * var_sif_dn0))) + (((4.0 * var_sir_dn0) * var_sif2) + (assign2860_e2204 * var_sif2_dn0))) + (2.0 * var_sif3_dn0))) * var_sif_sir_2) - (assign2860_e2212 * var_sif_sir_2_dn0)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qd_dn1 = (((-var_n_vt_cox_dn1) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * var_sir3_dn1) + (((6.0 * var_sir2_dn1) * var_sif) + (assign2860_e2198 * var_sif_dn1))) + (((4.0 * var_sir_dn1) * var_sif2) + (assign2860_e2204 * var_sif2_dn1))) + (2.0 * var_sif3_dn1))) * var_sif_sir_2) - (assign2860_e2212 * var_sif_sir_2_dn1)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qd_dn2 = (((-var_n_vt_cox_dn2) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * var_sir3_dn2) + (((6.0 * var_sir2_dn2) * var_sif) + (assign2860_e2198 * var_sif_dn2))) + (((4.0 * var_sir_dn2) * var_sif2) + (assign2860_e2204 * var_sif2_dn2))) + (2.0 * var_sif3_dn2))) * var_sif_sir_2) - (assign2860_e2212 * var_sif_sir_2_dn2)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qd_dn3 = (((-var_n_vt_cox_dn3) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * var_sir3_dn3) + (((6.0 * var_sir2_dn3) * var_sif) + (assign2860_e2198 * var_sif_dn3))) + (((4.0 * var_sir_dn3) * var_sif2) + (assign2860_e2204 * var_sif2_dn3))) + (2.0 * var_sif3_dn3))) * var_sif_sir_2) - (assign2860_e2212 * var_sif_sir_2_dn3)) / (var_sif_sir_2 * var_sif_sir_2))));

        let assign2870_e2219: f64 = (-var_n_vt_cox);
        let assign2870_e2223: f64 = (3.0 * var_sif3);
        let assign2870_e2226: f64 = (6.0 * var_sif2);
        let assign2870_e2228: f64 = (assign2870_e2226 * var_sir);
        let assign2870_e2229: f64 = (assign2870_e2223 + assign2870_e2228);
        let assign2870_e2232: f64 = (4.0 * var_sif);
        let assign2870_e2234: f64 = (assign2870_e2232 * var_sir2);
        let assign2870_e2235: f64 = (assign2870_e2229 + assign2870_e2234);
        let assign2870_e2238: f64 = (2.0 * var_sir3);
        let assign2870_e2239: f64 = (assign2870_e2235 + assign2870_e2238);
        let assign2870_e2240: f64 = (0.266666666 * assign2870_e2239);
        let assign2870_e2242: f64 = (assign2870_e2240 / var_sif_sir_2);
        let assign2870_e2244: f64 = (assign2870_e2242 - 0.5);
        let assign2870_e2245: f64 = (assign2870_e2219 * assign2870_e2244);
        var_qs = assign2870_e2245;
        var_qs_dn0 = (((-var_n_vt_cox_dn0) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * var_sif3_dn0) + (((6.0 * var_sif2_dn0) * var_sir) + (assign2870_e2226 * var_sir_dn0))) + (((4.0 * var_sif_dn0) * var_sir2) + (assign2870_e2232 * var_sir2_dn0))) + (2.0 * var_sir3_dn0))) * var_sif_sir_2) - (assign2870_e2240 * var_sif_sir_2_dn0)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qs_dn1 = (((-var_n_vt_cox_dn1) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * var_sif3_dn1) + (((6.0 * var_sif2_dn1) * var_sir) + (assign2870_e2226 * var_sir_dn1))) + (((4.0 * var_sif_dn1) * var_sir2) + (assign2870_e2232 * var_sir2_dn1))) + (2.0 * var_sir3_dn1))) * var_sif_sir_2) - (assign2870_e2240 * var_sif_sir_2_dn1)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qs_dn2 = (((-var_n_vt_cox_dn2) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * var_sif3_dn2) + (((6.0 * var_sif2_dn2) * var_sir) + (assign2870_e2226 * var_sir_dn2))) + (((4.0 * var_sif_dn2) * var_sir2) + (assign2870_e2232 * var_sir2_dn2))) + (2.0 * var_sir3_dn2))) * var_sif_sir_2) - (assign2870_e2240 * var_sif_sir_2_dn2)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qs_dn3 = (((-var_n_vt_cox_dn3) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * var_sif3_dn3) + (((6.0 * var_sif2_dn3) * var_sir) + (assign2870_e2226 * var_sir_dn3))) + (((4.0 * var_sif_dn3) * var_sir2) + (assign2870_e2232 * var_sir2_dn3))) + (2.0 * var_sir3_dn3))) * var_sif_sir_2) - (assign2870_e2240 * var_sif_sir_2_dn3)) / (var_sif_sir_2 * var_sif_sir_2))));

        let assign2880_e2248: f64 = (var_qs + var_qd);
        var_qi_1 = assign2880_e2248;
        var_qi_1_dn0 = (var_qs_dn0 + var_qd_dn0);
        var_qi_1_dn1 = (var_qs_dn1 + var_qd_dn1);
        var_qi_1_dn2 = (var_qs_dn2 + var_qd_dn2);
        var_qi_1_dn3 = (var_qs_dn3 + var_qd_dn3);

        let assign2890_e2251: f64 = (-0.5);
        let assign2890_e2253: f64 = (assign2890_e2251 * var_gammaprime);
        let assign2890_e2255: f64 = (assign2890_e2253 * var_sqrt_phi_vp_2);
        let assign2890_e2257: f64 = (assign2890_e2255 + var_vgprime);
        let assign2890_e2259: f64 = (assign2890_e2257 - var_vgstar);
        let assign2890_e2260: f64 = (var_wlcox * assign2890_e2259);
        let assign2890_e2263: f64 = (var_qi_1 * var_gammaprime);
        let assign2890_e2266: f64 = (var_gammaprime + var_sqrt_phi_vp2_2);
        let assign2890_e2267: f64 = (assign2890_e2263 / assign2890_e2266);
        let assign2890_e2268: f64 = (assign2890_e2260 - assign2890_e2267);
        var_qb_1 = assign2890_e2268;
        var_qb_1_dn0 = ((var_wlcox * (((((assign2890_e2251 * var_gammaprime_dn0) * var_sqrt_phi_vp_2) + (assign2890_e2253 * var_sqrt_phi_vp_2_dn0)) + var_vgprime_dn0) - var_vgstar_dn0)) - (((((var_qi_1_dn0 * var_gammaprime) + (var_qi_1 * var_gammaprime_dn0)) * assign2890_e2266) - (assign2890_e2263 * (var_gammaprime_dn0 + var_sqrt_phi_vp2_2_dn0))) / (assign2890_e2266 * assign2890_e2266)));
        var_qb_1_dn1 = ((var_wlcox * (((((assign2890_e2251 * var_gammaprime_dn1) * var_sqrt_phi_vp_2) + (assign2890_e2253 * var_sqrt_phi_vp_2_dn1)) + var_vgprime_dn1) - var_vgstar_dn1)) - (((((var_qi_1_dn1 * var_gammaprime) + (var_qi_1 * var_gammaprime_dn1)) * assign2890_e2266) - (assign2890_e2263 * (var_gammaprime_dn1 + var_sqrt_phi_vp2_2_dn1))) / (assign2890_e2266 * assign2890_e2266)));
        var_qb_1_dn2 = ((var_wlcox * (((((assign2890_e2251 * var_gammaprime_dn2) * var_sqrt_phi_vp_2) + (assign2890_e2253 * var_sqrt_phi_vp_2_dn2)) + var_vgprime_dn2) - var_vgstar_dn2)) - (((((var_qi_1_dn2 * var_gammaprime) + (var_qi_1 * var_gammaprime_dn2)) * assign2890_e2266) - (assign2890_e2263 * (var_gammaprime_dn2 + var_sqrt_phi_vp2_2_dn2))) / (assign2890_e2266 * assign2890_e2266)));
        var_qb_1_dn3 = ((var_wlcox * (((((assign2890_e2251 * var_gammaprime_dn3) * var_sqrt_phi_vp_2) + (assign2890_e2253 * var_sqrt_phi_vp_2_dn3)) + var_vgprime_dn3) - var_vgstar_dn3)) - (((((var_qi_1_dn3 * var_gammaprime) + (var_qi_1 * var_gammaprime_dn3)) * assign2890_e2266) - (assign2890_e2263 * (var_gammaprime_dn3 + var_sqrt_phi_vp2_2_dn3))) / (assign2890_e2266 * assign2890_e2266)));

        let assign2900_e2270: f64 = (-var_qi_1);
        let assign2900_e2272: f64 = (assign2900_e2270 - var_qb_1);
        var_qg = assign2900_e2272;
        var_qg_dn0 = ((-var_qi_1_dn0) - var_qb_1_dn0);
        var_qg_dn1 = ((-var_qi_1_dn1) - var_qb_1_dn1);
        var_qg_dn2 = ((-var_qi_1_dn2) - var_qb_1_dn2);
        var_qg_dn3 = ((-var_qi_1_dn3) - var_qb_1_dn3);

        let assign2910_e2274: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_qd);
        var_ddt_qd = assign2910_e2274;
        var_ddt_qd_dn0 = (var_qd_dn0 * ddt_scale);
        var_ddt_qd_dn1 = (var_qd_dn1 * ddt_scale);
        var_ddt_qd_dn2 = (var_qd_dn2 * ddt_scale);
        var_ddt_qd_dn3 = (var_qd_dn3 * ddt_scale);

        let assign2920_e2276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_qs);
        var_ddt_qs = assign2920_e2276;
        var_ddt_qs_dn0 = (var_qs_dn0 * ddt_scale);
        var_ddt_qs_dn1 = (var_qs_dn1 * ddt_scale);
        var_ddt_qs_dn2 = (var_qs_dn2 * ddt_scale);
        var_ddt_qs_dn3 = (var_qs_dn3 * ddt_scale);

        let assign2930_e2279: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard21 = assign2930_e2279;

        let assign2960_e2312: f64 = if ((p.p9 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        var_guard24 = assign2960_e2312;

        let (assign2970_e2320,) = {
    if (var_guard24 != 0.0) {
        let assign2970_e2316: f64 = (2.0 * p.p37);
        let assign2970_e2318: f64 = (assign2970_e2316 * var_weff);
        (assign2970_e2318,)
    } else {
        (var_as_i,)
    }
};
        var_as_i = assign2970_e2320;

        let (assign2980_e2325,) = {
    if (var_guard24 == 0.0) {
        (p.p9,)
    } else {
        (var_as_i,)
    }
};
        var_as_i = assign2980_e2325;

        let assign2990_e2332: f64 = if ((p.p11 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        var_guard25 = assign2990_e2332;

        let (assign3000_e2342,) = {
    if (var_guard25 != 0.0) {
        let assign3000_e2336: f64 = (4.0 * p.p37);
        let assign3000_e2339: f64 = var_weff;
        let assign3000_e2340: f64 = (assign3000_e2336 + assign3000_e2339);
        (assign3000_e2340,)
    } else {
        (var_ps_i,)
    }
};
        var_ps_i = assign3000_e2342;

        let (assign3010_e2347,) = {
    if (var_guard25 == 0.0) {
        (p.p11,)
    } else {
        (var_ps_i,)
    }
};
        var_ps_i = assign3010_e2347;

        let assign3020_e2354: f64 = if ((p.p10 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        var_guard26 = assign3020_e2354;

        let (assign3030_e2362,) = {
    if (var_guard26 != 0.0) {
        let assign3030_e2358: f64 = (2.0 * p.p37);
        let assign3030_e2360: f64 = (assign3030_e2358 * var_weff);
        (assign3030_e2360,)
    } else {
        (var_ad_i,)
    }
};
        var_ad_i = assign3030_e2362;

        let (assign3040_e2367,) = {
    if (var_guard26 == 0.0) {
        (p.p10,)
    } else {
        (var_ad_i,)
    }
};
        var_ad_i = assign3040_e2367;

        let assign3050_e2374: f64 = if ((p.p12 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        var_guard27 = assign3050_e2374;

        let (assign3060_e2384,) = {
    if (var_guard27 != 0.0) {
        let assign3060_e2378: f64 = (4.0 * p.p37);
        let assign3060_e2381: f64 = var_weff;
        let assign3060_e2382: f64 = (assign3060_e2378 + assign3060_e2381);
        (assign3060_e2382,)
    } else {
        (var_pd_i,)
    }
};
        var_pd_i = assign3060_e2384;

        let (assign3070_e2389,) = {
    if (var_guard27 == 0.0) {
        (p.p12,)
    } else {
        (var_pd_i,)
    }
};
        var_pd_i = assign3070_e2389;

        let assign3120_e2418: f64 = (p.p69 * var_deltat);
        let assign3120_e2419: f64 = (p.p50 - assign3120_e2418);
        var_pb_t = assign3120_e2419;

        let assign3130_e2423: f64 = (p.p70 * var_deltat);
        let assign3130_e2424: f64 = (p.p51 - assign3130_e2423);
        var_pbsw_t = assign3130_e2424;

        let assign3140_e2428: f64 = (p.p71 * var_deltat);
        let assign3140_e2429: f64 = (p.p52 - assign3140_e2428);
        var_pbswg_t = assign3140_e2429;

        let assign3150_e2434: f64 = (p.p66 * var_deltat);
        let assign3150_e2435: f64 = (1.0 + assign3150_e2434);
        let assign3150_e2436: f64 = (p.p53 * assign3150_e2435);
        var_cj_t = assign3150_e2436;

        let assign3160_e2441: f64 = (p.p67 * var_deltat);
        let assign3160_e2442: f64 = (1.0 + assign3160_e2441);
        let assign3160_e2443: f64 = (p.p54 * assign3160_e2442);
        var_cjsw_t = assign3160_e2443;

        let assign3170_e2448: f64 = (p.p68 * var_deltat);
        let assign3170_e2449: f64 = (1.0 + assign3170_e2448);
        let assign3170_e2450: f64 = (p.p55 * assign3170_e2449);
        var_cjswg_t = assign3170_e2450;

        let assign3210_e2480: f64 = (p.p0 * (nv0 - nv3));
        var_v_di_b = assign3210_e2480;
        var_v_di_b_dn0 = p.p0;
        var_v_di_b_dn3 = (-p.p0);

        let assign3220_e2483: f64 = (p.p0 * (nv2 - nv3));
        var_v_si_b = assign3220_e2483;
        var_v_si_b_dn2 = p.p0;
        var_v_si_b_dn3 = (-p.p0);

        let assign3450_e2740: f64 = if var_v_di_b > 0.0 { 1.0 } else { 0.0 };
        var_guard32 = assign3450_e2740;

        let (assign3460_e2757, assign3460_e2757_d_n0, assign3460_e2757_d_n3,) = {
    if (var_guard32 != 0.0) {
        let assign3460_e2744: f64 = (var_cj_t * var_ad_i);
        let assign3460_e2746: f64 = (-p.p47);
        let assign3460_e2750: f64 = (var_v_di_b / var_pb_t);
        let assign3460_e2751: f64 = (1.0 + assign3460_e2750);
        let assign3460_e2752: f64 = (assign3460_e2751).ln();
        let assign3460_e2753: f64 = (assign3460_e2746 * assign3460_e2752);
        let assign3460_e2754: f64 = (assign3460_e2753).exp();
        let assign3460_e2755: f64 = (assign3460_e2744 * assign3460_e2754);
        (assign3460_e2755, (assign3460_e2744 * (assign3460_e2754 * (assign3460_e2746 * ((var_v_di_b_dn0 / var_pb_t) / assign3460_e2751)))), (assign3460_e2744 * (assign3460_e2754 * (assign3460_e2746 * ((var_v_di_b_dn3 / var_pb_t) / assign3460_e2751)))),)
    } else {
        (var_csb_d, var_csb_d_dn0, var_csb_d_dn3,)
    }
};
        var_csb_d = assign3460_e2757;
        var_csb_d_dn0 = assign3460_e2757_d_n0;
        var_csb_d_dn3 = assign3460_e2757_d_n3;

        let (assign3470_e2774, assign3470_e2774_d_n0, assign3470_e2774_d_n3,) = {
    if (var_guard32 != 0.0) {
        let assign3470_e2761: f64 = (var_cjsw_t * var_pd_i);
        let assign3470_e2763: f64 = (-p.p48);
        let assign3470_e2767: f64 = (var_v_di_b / var_pbsw_t);
        let assign3470_e2768: f64 = (1.0 + assign3470_e2767);
        let assign3470_e2769: f64 = (assign3470_e2768).ln();
        let assign3470_e2770: f64 = (assign3470_e2763 * assign3470_e2769);
        let assign3470_e2771: f64 = (assign3470_e2770).exp();
        let assign3470_e2772: f64 = (assign3470_e2761 * assign3470_e2771);
        (assign3470_e2772, (assign3470_e2761 * (assign3470_e2771 * (assign3470_e2763 * ((var_v_di_b_dn0 / var_pbsw_t) / assign3470_e2768)))), (assign3470_e2761 * (assign3470_e2771 * (assign3470_e2763 * ((var_v_di_b_dn3 / var_pbsw_t) / assign3470_e2768)))),)
    } else {
        (var_cssw_d, var_cssw_d_dn0, var_cssw_d_dn3,)
    }
};
        var_cssw_d = assign3470_e2774;
        var_cssw_d_dn0 = assign3470_e2774_d_n0;
        var_cssw_d_dn3 = assign3470_e2774_d_n3;

        let (assign3480_e2791, assign3480_e2791_d_n0, assign3480_e2791_d_n3,) = {
    if (var_guard32 != 0.0) {
        let assign3480_e2778: f64 = (var_cjswg_t * var_weff);
        let assign3480_e2780: f64 = (-p.p49);
        let assign3480_e2784: f64 = (var_v_di_b / var_pbswg_t);
        let assign3480_e2785: f64 = (1.0 + assign3480_e2784);
        let assign3480_e2786: f64 = (assign3480_e2785).ln();
        let assign3480_e2787: f64 = (assign3480_e2780 * assign3480_e2786);
        let assign3480_e2788: f64 = (assign3480_e2787).exp();
        let assign3480_e2789: f64 = (assign3480_e2778 * assign3480_e2788);
        (assign3480_e2789, (assign3480_e2778 * (assign3480_e2788 * (assign3480_e2780 * ((var_v_di_b_dn0 / var_pbswg_t) / assign3480_e2785)))), (assign3480_e2778 * (assign3480_e2788 * (assign3480_e2780 * ((var_v_di_b_dn3 / var_pbswg_t) / assign3480_e2785)))),)
    } else {
        (var_csswg_d, var_csswg_d_dn0, var_csswg_d_dn3,)
    }
};
        var_csswg_d = assign3480_e2791;
        var_csswg_d_dn0 = assign3480_e2791_d_n0;
        var_csswg_d_dn3 = assign3480_e2791_d_n3;

        let (assign3490_e2806, assign3490_e2806_d_n0, assign3490_e2806_d_n3,) = {
    if (var_guard32 == 0.0) {
        let assign3490_e2796: f64 = (var_cj_t * var_ad_i);
        let assign3490_e2800: f64 = (p.p47 * var_v_di_b);
        let assign3490_e2802: f64 = (assign3490_e2800 / var_pb_t);
        let assign3490_e2803: f64 = (1.0 - assign3490_e2802);
        let assign3490_e2804: f64 = (assign3490_e2796 * assign3490_e2803);
        (assign3490_e2804, (assign3490_e2796 * (-((p.p47 * var_v_di_b_dn0) / var_pb_t))), (assign3490_e2796 * (-((p.p47 * var_v_di_b_dn3) / var_pb_t))),)
    } else {
        (var_csb_d, var_csb_d_dn0, var_csb_d_dn3,)
    }
};
        var_csb_d = assign3490_e2806;
        var_csb_d_dn0 = assign3490_e2806_d_n0;
        var_csb_d_dn3 = assign3490_e2806_d_n3;

        *var_ad_i_slot = var_ad_i;
        *var_as_i_slot = var_as_i;
        *var_cj_t_slot = var_cj_t;
        *var_cjsw_t_slot = var_cjsw_t;
        *var_cjswg_t_slot = var_cjswg_t;
        *var_csb_d_slot = var_csb_d;
        *var_csb_d_dn0_slot = var_csb_d_dn0;
        *var_csb_d_dn3_slot = var_csb_d_dn3;
        *var_cssw_d_slot = var_cssw_d;
        *var_cssw_d_dn0_slot = var_cssw_d_dn0;
        *var_cssw_d_dn3_slot = var_cssw_d_dn3;
        *var_csswg_d_slot = var_csswg_d;
        *var_csswg_d_dn0_slot = var_csswg_d_dn0;
        *var_csswg_d_dn3_slot = var_csswg_d_dn3;
        *var_dbeta_dvd_slot = var_dbeta_dvd;
        *var_dbeta_dvd_dn0_slot = var_dbeta_dvd_dn0;
        *var_dbeta_dvd_dn1_slot = var_dbeta_dvd_dn1;
        *var_dbeta_dvd_dn2_slot = var_dbeta_dvd_dn2;
        *var_dbeta_dvd_dn3_slot = var_dbeta_dvd_dn3;
        *var_dbeta_dvs_slot = var_dbeta_dvs;
        *var_dbeta_dvs_dn0_slot = var_dbeta_dvs_dn0;
        *var_dbeta_dvs_dn1_slot = var_dbeta_dvs_dn1;
        *var_dbeta_dvs_dn2_slot = var_dbeta_dvs_dn2;
        *var_dbeta_dvs_dn3_slot = var_dbeta_dvs_dn3;
        *var_ddt_qd_slot = var_ddt_qd;
        *var_ddt_qd_dn0_slot = var_ddt_qd_dn0;
        *var_ddt_qd_dn1_slot = var_ddt_qd_dn1;
        *var_ddt_qd_dn2_slot = var_ddt_qd_dn2;
        *var_ddt_qd_dn3_slot = var_ddt_qd_dn3;
        *var_ddt_qs_slot = var_ddt_qs;
        *var_ddt_qs_dn0_slot = var_ddt_qs_dn0;
        *var_ddt_qs_dn1_slot = var_ddt_qs_dn1;
        *var_ddt_qs_dn2_slot = var_ddt_qs_dn2;
        *var_ddt_qs_dn3_slot = var_ddt_qs_dn3;
        *var_dn_dvd_slot = var_dn_dvd;
        *var_dn_dvd_dn0_slot = var_dn_dvd_dn0;
        *var_dn_dvd_dn1_slot = var_dn_dvd_dn1;
        *var_dn_dvd_dn2_slot = var_dn_dvd_dn2;
        *var_dn_dvd_dn3_slot = var_dn_dvd_dn3;
        *var_dn_dvs_slot = var_dn_dvs;
        *var_dn_dvs_dn0_slot = var_dn_dvs_dn0;
        *var_dn_dvs_dn1_slot = var_dn_dvs_dn1;
        *var_dn_dvs_dn2_slot = var_dn_dvs_dn2;
        *var_dn_dvs_dn3_slot = var_dn_dvs_dn3;
        *var_dvpprime_dvs_slot = var_dvpprime_dvs;
        *var_dvpprime_dvs_dn0_slot = var_dvpprime_dvs_dn0;
        *var_dvpprime_dvs_dn1_slot = var_dvpprime_dvs_dn1;
        *var_dvpprime_dvs_dn2_slot = var_dvpprime_dvs_dn2;
        *var_dvpprime_dvs_dn3_slot = var_dvpprime_dvs_dn3;
        *var_gds_slot = var_gds;
        *var_gds_dn0_slot = var_gds_dn0;
        *var_gds_dn1_slot = var_gds_dn1;
        *var_gds_dn2_slot = var_gds_dn2;
        *var_gds_dn3_slot = var_gds_dn3;
        *var_gms_slot = var_gms;
        *var_gms_dn0_slot = var_gms_dn0;
        *var_gms_dn1_slot = var_gms_dn1;
        *var_gms_dn2_slot = var_gms_dn2;
        *var_gms_dn3_slot = var_gms_dn3;
        *var_guard21_slot = var_guard21;
        *var_guard24_slot = var_guard24;
        *var_guard25_slot = var_guard25;
        *var_guard26_slot = var_guard26;
        *var_guard27_slot = var_guard27;
        *var_guard32_slot = var_guard32;
        *var_n_vt_cox_slot = var_n_vt_cox;
        *var_n_vt_cox_dn0_slot = var_n_vt_cox_dn0;
        *var_n_vt_cox_dn1_slot = var_n_vt_cox_dn1;
        *var_n_vt_cox_dn2_slot = var_n_vt_cox_dn2;
        *var_n_vt_cox_dn3_slot = var_n_vt_cox_dn3;
        *var_pb_t_slot = var_pb_t;
        *var_pbsw_t_slot = var_pbsw_t;
        *var_pbswg_t_slot = var_pbswg_t;
        *var_pd_i_slot = var_pd_i;
        *var_ps_i_slot = var_ps_i;
        *var_qb_1_slot = var_qb_1;
        *var_qb_1_dn0_slot = var_qb_1_dn0;
        *var_qb_1_dn1_slot = var_qb_1_dn1;
        *var_qb_1_dn2_slot = var_qb_1_dn2;
        *var_qb_1_dn3_slot = var_qb_1_dn3;
        *var_qd_slot = var_qd;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn1_slot = var_qd_dn1;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn3_slot = var_qd_dn3;
        *var_qg_slot = var_qg;
        *var_qg_dn0_slot = var_qg_dn0;
        *var_qg_dn1_slot = var_qg_dn1;
        *var_qg_dn2_slot = var_qg_dn2;
        *var_qg_dn3_slot = var_qg_dn3;
        *var_qi_1_slot = var_qi_1;
        *var_qi_1_dn0_slot = var_qi_1_dn0;
        *var_qi_1_dn1_slot = var_qi_1_dn1;
        *var_qi_1_dn2_slot = var_qi_1_dn2;
        *var_qi_1_dn3_slot = var_qi_1_dn3;
        *var_qs_slot = var_qs;
        *var_qs_dn0_slot = var_qs_dn0;
        *var_qs_dn1_slot = var_qs_dn1;
        *var_qs_dn2_slot = var_qs_dn2;
        *var_qs_dn3_slot = var_qs_dn3;
        *var_rdeff_slot = var_rdeff;
        *var_rseff_slot = var_rseff;
        *var_sif3_slot = var_sif3;
        *var_sif3_dn0_slot = var_sif3_dn0;
        *var_sif3_dn1_slot = var_sif3_dn1;
        *var_sif3_dn2_slot = var_sif3_dn2;
        *var_sif3_dn3_slot = var_sif3_dn3;
        *var_sir3_slot = var_sir3;
        *var_sir3_dn0_slot = var_sir3_dn0;
        *var_sir3_dn1_slot = var_sir3_dn1;
        *var_sir3_dn2_slot = var_sir3_dn2;
        *var_sir3_dn3_slot = var_sir3_dn3;
        *var_sqrt_phi_vp2_2_slot = var_sqrt_phi_vp2_2;
        *var_sqrt_phi_vp2_2_dn0_slot = var_sqrt_phi_vp2_2_dn0;
        *var_sqrt_phi_vp2_2_dn1_slot = var_sqrt_phi_vp2_2_dn1;
        *var_sqrt_phi_vp2_2_dn2_slot = var_sqrt_phi_vp2_2_dn2;
        *var_sqrt_phi_vp2_2_dn3_slot = var_sqrt_phi_vp2_2_dn3;
        *var_tmp1_slot = var_tmp1;
        *var_tmp1_dn0_slot = var_tmp1_dn0;
        *var_tmp1_dn1_slot = var_tmp1_dn1;
        *var_tmp1_dn2_slot = var_tmp1_dn2;
        *var_tmp1_dn3_slot = var_tmp1_dn3;
        *var_v_di_b_slot = var_v_di_b;
        *var_v_di_b_dn0_slot = var_v_di_b_dn0;
        *var_v_di_b_dn3_slot = var_v_di_b_dn3;
        *var_v_si_b_slot = var_v_si_b;
        *var_v_si_b_dn2_slot = var_v_si_b_dn2;
        *var_v_si_b_dn3_slot = var_v_si_b_dn3;
        *var_wlcox_slot = var_wlcox;
    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        var_as_i: f64,
        var_cj_t: f64,
        var_cjsw_t: f64,
        var_cjswg_t: f64,
        var_csb_d: f64,
        var_csb_d_dn0: f64,
        var_csb_d_dn3: f64,
        var_guard32: f64,
        var_pb_t: f64,
        var_pbsw_t: f64,
        var_pbswg_t: f64,
        var_pd_i: f64,
        var_ps_i: f64,
        var_v_di_b: f64,
        var_v_di_b_dn0: f64,
        var_v_di_b_dn3: f64,
        var_v_si_b: f64,
        var_v_si_b_dn2: f64,
        var_v_si_b_dn3: f64,
        var_weff: f64,
        var_csb_s_slot: &mut f64,
        var_csb_s_dn2_slot: &mut f64,
        var_csb_s_dn3_slot: &mut f64,
        var_cssw_d_slot: &mut f64,
        var_cssw_d_dn0_slot: &mut f64,
        var_cssw_d_dn3_slot: &mut f64,
        var_cssw_s_slot: &mut f64,
        var_cssw_s_dn2_slot: &mut f64,
        var_cssw_s_dn3_slot: &mut f64,
        var_csswg_d_slot: &mut f64,
        var_csswg_d_dn0_slot: &mut f64,
        var_csswg_d_dn3_slot: &mut f64,
        var_csswg_s_slot: &mut f64,
        var_csswg_s_dn2_slot: &mut f64,
        var_csswg_s_dn3_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_qjd_slot: &mut f64,
        var_qjd_dn0_slot: &mut f64,
        var_qjd_dn3_slot: &mut f64,
        var_qjs_slot: &mut f64,
        var_qjs_dn2_slot: &mut f64,
        var_qjs_dn3_slot: &mut f64,
    ) {
        let mut var_csb_s: f64 = *var_csb_s_slot;
        let mut var_csb_s_dn2: f64 = *var_csb_s_dn2_slot;
        let mut var_csb_s_dn3: f64 = *var_csb_s_dn3_slot;
        let mut var_cssw_d: f64 = *var_cssw_d_slot;
        let mut var_cssw_d_dn0: f64 = *var_cssw_d_dn0_slot;
        let mut var_cssw_d_dn3: f64 = *var_cssw_d_dn3_slot;
        let mut var_cssw_s: f64 = *var_cssw_s_slot;
        let mut var_cssw_s_dn2: f64 = *var_cssw_s_dn2_slot;
        let mut var_cssw_s_dn3: f64 = *var_cssw_s_dn3_slot;
        let mut var_csswg_d: f64 = *var_csswg_d_slot;
        let mut var_csswg_d_dn0: f64 = *var_csswg_d_dn0_slot;
        let mut var_csswg_d_dn3: f64 = *var_csswg_d_dn3_slot;
        let mut var_csswg_s: f64 = *var_csswg_s_slot;
        let mut var_csswg_s_dn2: f64 = *var_csswg_s_dn2_slot;
        let mut var_csswg_s_dn3: f64 = *var_csswg_s_dn3_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_qjd: f64 = *var_qjd_slot;
        let mut var_qjd_dn0: f64 = *var_qjd_dn0_slot;
        let mut var_qjd_dn3: f64 = *var_qjd_dn3_slot;
        let mut var_qjs: f64 = *var_qjs_slot;
        let mut var_qjs_dn2: f64 = *var_qjs_dn2_slot;
        let mut var_qjs_dn3: f64 = *var_qjs_dn3_slot;

        let (assign3500_e2821, assign3500_e2821_d_n0, assign3500_e2821_d_n3,) = {
    if (var_guard32 == 0.0) {
        let assign3500_e2811: f64 = (var_cjsw_t * var_pd_i);
        let assign3500_e2815: f64 = (p.p48 * var_v_di_b);
        let assign3500_e2817: f64 = (assign3500_e2815 / var_pbsw_t);
        let assign3500_e2818: f64 = (1.0 - assign3500_e2817);
        let assign3500_e2819: f64 = (assign3500_e2811 * assign3500_e2818);
        (assign3500_e2819, (assign3500_e2811 * (-((p.p48 * var_v_di_b_dn0) / var_pbsw_t))), (assign3500_e2811 * (-((p.p48 * var_v_di_b_dn3) / var_pbsw_t))),)
    } else {
        (var_cssw_d, var_cssw_d_dn0, var_cssw_d_dn3,)
    }
};
        var_cssw_d = assign3500_e2821;
        var_cssw_d_dn0 = assign3500_e2821_d_n0;
        var_cssw_d_dn3 = assign3500_e2821_d_n3;

        let (assign3510_e2836, assign3510_e2836_d_n0, assign3510_e2836_d_n3,) = {
    if (var_guard32 == 0.0) {
        let assign3510_e2826: f64 = (var_cjswg_t * var_weff);
        let assign3510_e2830: f64 = (p.p49 * var_v_di_b);
        let assign3510_e2832: f64 = (assign3510_e2830 / var_pbswg_t);
        let assign3510_e2833: f64 = (1.0 - assign3510_e2832);
        let assign3510_e2834: f64 = (assign3510_e2826 * assign3510_e2833);
        (assign3510_e2834, (assign3510_e2826 * (-((p.p49 * var_v_di_b_dn0) / var_pbswg_t))), (assign3510_e2826 * (-((p.p49 * var_v_di_b_dn3) / var_pbswg_t))),)
    } else {
        (var_csswg_d, var_csswg_d_dn0, var_csswg_d_dn3,)
    }
};
        var_csswg_d = assign3510_e2836;
        var_csswg_d_dn0 = assign3510_e2836_d_n0;
        var_csswg_d_dn3 = assign3510_e2836_d_n3;

        let assign3520_e2839: f64 = (var_csb_d + var_cssw_d);
        let assign3520_e2841: f64 = (assign3520_e2839 + var_csswg_d);
        let assign3520_e2843: f64 = (assign3520_e2841 * var_v_di_b);
        var_qjd = assign3520_e2843;
        var_qjd_dn0 = ((((var_csb_d_dn0 + var_cssw_d_dn0) + var_csswg_d_dn0) * var_v_di_b) + (assign3520_e2841 * var_v_di_b_dn0));
        var_qjd_dn3 = ((((var_csb_d_dn3 + var_cssw_d_dn3) + var_csswg_d_dn3) * var_v_di_b) + (assign3520_e2841 * var_v_di_b_dn3));

        let assign3530_e2846: f64 = if var_v_si_b > 0.0 { 1.0 } else { 0.0 };
        var_guard33 = assign3530_e2846;

        let (assign3540_e2863, assign3540_e2863_d_n2, assign3540_e2863_d_n3,) = {
    if (var_guard33 != 0.0) {
        let assign3540_e2850: f64 = (var_cj_t * var_as_i);
        let assign3540_e2852: f64 = (-p.p47);
        let assign3540_e2856: f64 = (var_v_si_b / var_pb_t);
        let assign3540_e2857: f64 = (1.0 + assign3540_e2856);
        let assign3540_e2858: f64 = (assign3540_e2857).ln();
        let assign3540_e2859: f64 = (assign3540_e2852 * assign3540_e2858);
        let assign3540_e2860: f64 = (assign3540_e2859).exp();
        let assign3540_e2861: f64 = (assign3540_e2850 * assign3540_e2860);
        (assign3540_e2861, (assign3540_e2850 * (assign3540_e2860 * (assign3540_e2852 * ((var_v_si_b_dn2 / var_pb_t) / assign3540_e2857)))), (assign3540_e2850 * (assign3540_e2860 * (assign3540_e2852 * ((var_v_si_b_dn3 / var_pb_t) / assign3540_e2857)))),)
    } else {
        (var_csb_s, var_csb_s_dn2, var_csb_s_dn3,)
    }
};
        var_csb_s = assign3540_e2863;
        var_csb_s_dn2 = assign3540_e2863_d_n2;
        var_csb_s_dn3 = assign3540_e2863_d_n3;

        let (assign3550_e2880, assign3550_e2880_d_n2, assign3550_e2880_d_n3,) = {
    if (var_guard33 != 0.0) {
        let assign3550_e2867: f64 = (var_cjsw_t * var_ps_i);
        let assign3550_e2869: f64 = (-p.p48);
        let assign3550_e2873: f64 = (var_v_si_b / var_pbsw_t);
        let assign3550_e2874: f64 = (1.0 + assign3550_e2873);
        let assign3550_e2875: f64 = (assign3550_e2874).ln();
        let assign3550_e2876: f64 = (assign3550_e2869 * assign3550_e2875);
        let assign3550_e2877: f64 = (assign3550_e2876).exp();
        let assign3550_e2878: f64 = (assign3550_e2867 * assign3550_e2877);
        (assign3550_e2878, (assign3550_e2867 * (assign3550_e2877 * (assign3550_e2869 * ((var_v_si_b_dn2 / var_pbsw_t) / assign3550_e2874)))), (assign3550_e2867 * (assign3550_e2877 * (assign3550_e2869 * ((var_v_si_b_dn3 / var_pbsw_t) / assign3550_e2874)))),)
    } else {
        (var_cssw_s, var_cssw_s_dn2, var_cssw_s_dn3,)
    }
};
        var_cssw_s = assign3550_e2880;
        var_cssw_s_dn2 = assign3550_e2880_d_n2;
        var_cssw_s_dn3 = assign3550_e2880_d_n3;

        let (assign3560_e2897, assign3560_e2897_d_n2, assign3560_e2897_d_n3,) = {
    if (var_guard33 != 0.0) {
        let assign3560_e2884: f64 = (var_cjswg_t * var_weff);
        let assign3560_e2886: f64 = (-p.p49);
        let assign3560_e2890: f64 = (var_v_si_b / var_pbswg_t);
        let assign3560_e2891: f64 = (1.0 + assign3560_e2890);
        let assign3560_e2892: f64 = (assign3560_e2891).ln();
        let assign3560_e2893: f64 = (assign3560_e2886 * assign3560_e2892);
        let assign3560_e2894: f64 = (assign3560_e2893).exp();
        let assign3560_e2895: f64 = (assign3560_e2884 * assign3560_e2894);
        (assign3560_e2895, (assign3560_e2884 * (assign3560_e2894 * (assign3560_e2886 * ((var_v_si_b_dn2 / var_pbswg_t) / assign3560_e2891)))), (assign3560_e2884 * (assign3560_e2894 * (assign3560_e2886 * ((var_v_si_b_dn3 / var_pbswg_t) / assign3560_e2891)))),)
    } else {
        (var_csswg_s, var_csswg_s_dn2, var_csswg_s_dn3,)
    }
};
        var_csswg_s = assign3560_e2897;
        var_csswg_s_dn2 = assign3560_e2897_d_n2;
        var_csswg_s_dn3 = assign3560_e2897_d_n3;

        let (assign3570_e2912, assign3570_e2912_d_n2, assign3570_e2912_d_n3,) = {
    if (var_guard33 == 0.0) {
        let assign3570_e2902: f64 = (var_cj_t * var_as_i);
        let assign3570_e2906: f64 = (p.p47 * var_v_si_b);
        let assign3570_e2908: f64 = (assign3570_e2906 / var_pb_t);
        let assign3570_e2909: f64 = (1.0 - assign3570_e2908);
        let assign3570_e2910: f64 = (assign3570_e2902 * assign3570_e2909);
        (assign3570_e2910, (assign3570_e2902 * (-((p.p47 * var_v_si_b_dn2) / var_pb_t))), (assign3570_e2902 * (-((p.p47 * var_v_si_b_dn3) / var_pb_t))),)
    } else {
        (var_csb_s, var_csb_s_dn2, var_csb_s_dn3,)
    }
};
        var_csb_s = assign3570_e2912;
        var_csb_s_dn2 = assign3570_e2912_d_n2;
        var_csb_s_dn3 = assign3570_e2912_d_n3;

        let (assign3580_e2927, assign3580_e2927_d_n2, assign3580_e2927_d_n3,) = {
    if (var_guard33 == 0.0) {
        let assign3580_e2917: f64 = (var_cjsw_t * var_ps_i);
        let assign3580_e2921: f64 = (p.p48 * var_v_si_b);
        let assign3580_e2923: f64 = (assign3580_e2921 / var_pbsw_t);
        let assign3580_e2924: f64 = (1.0 - assign3580_e2923);
        let assign3580_e2925: f64 = (assign3580_e2917 * assign3580_e2924);
        (assign3580_e2925, (assign3580_e2917 * (-((p.p48 * var_v_si_b_dn2) / var_pbsw_t))), (assign3580_e2917 * (-((p.p48 * var_v_si_b_dn3) / var_pbsw_t))),)
    } else {
        (var_cssw_s, var_cssw_s_dn2, var_cssw_s_dn3,)
    }
};
        var_cssw_s = assign3580_e2927;
        var_cssw_s_dn2 = assign3580_e2927_d_n2;
        var_cssw_s_dn3 = assign3580_e2927_d_n3;

        let (assign3590_e2942, assign3590_e2942_d_n2, assign3590_e2942_d_n3,) = {
    if (var_guard33 == 0.0) {
        let assign3590_e2932: f64 = (var_cjswg_t * var_weff);
        let assign3590_e2936: f64 = (p.p49 * var_v_si_b);
        let assign3590_e2938: f64 = (assign3590_e2936 / var_pbswg_t);
        let assign3590_e2939: f64 = (1.0 - assign3590_e2938);
        let assign3590_e2940: f64 = (assign3590_e2932 * assign3590_e2939);
        (assign3590_e2940, (assign3590_e2932 * (-((p.p49 * var_v_si_b_dn2) / var_pbswg_t))), (assign3590_e2932 * (-((p.p49 * var_v_si_b_dn3) / var_pbswg_t))),)
    } else {
        (var_csswg_s, var_csswg_s_dn2, var_csswg_s_dn3,)
    }
};
        var_csswg_s = assign3590_e2942;
        var_csswg_s_dn2 = assign3590_e2942_d_n2;
        var_csswg_s_dn3 = assign3590_e2942_d_n3;

        let assign3600_e2945: f64 = (var_csb_s + var_cssw_s);
        let assign3600_e2947: f64 = (assign3600_e2945 + var_csswg_s);
        let assign3600_e2949: f64 = (assign3600_e2947 * var_v_si_b);
        var_qjs = assign3600_e2949;
        var_qjs_dn2 = ((((var_csb_s_dn2 + var_cssw_s_dn2) + var_csswg_s_dn2) * var_v_si_b) + (assign3600_e2947 * var_v_si_b_dn2));
        var_qjs_dn3 = ((((var_csb_s_dn3 + var_cssw_s_dn3) + var_csswg_s_dn3) * var_v_si_b) + (assign3600_e2947 * var_v_si_b_dn3));

        *var_csb_s_slot = var_csb_s;
        *var_csb_s_dn2_slot = var_csb_s_dn2;
        *var_csb_s_dn3_slot = var_csb_s_dn3;
        *var_cssw_d_slot = var_cssw_d;
        *var_cssw_d_dn0_slot = var_cssw_d_dn0;
        *var_cssw_d_dn3_slot = var_cssw_d_dn3;
        *var_cssw_s_slot = var_cssw_s;
        *var_cssw_s_dn2_slot = var_cssw_s_dn2;
        *var_cssw_s_dn3_slot = var_cssw_s_dn3;
        *var_csswg_d_slot = var_csswg_d;
        *var_csswg_d_dn0_slot = var_csswg_d_dn0;
        *var_csswg_d_dn3_slot = var_csswg_d_dn3;
        *var_csswg_s_slot = var_csswg_s;
        *var_csswg_s_dn2_slot = var_csswg_s_dn2;
        *var_csswg_s_dn3_slot = var_csswg_s_dn3;
        *var_guard33_slot = var_guard33;
        *var_qjd_slot = var_qjd;
        *var_qjd_dn0_slot = var_qjd_dn0;
        *var_qjd_dn3_slot = var_qjd_dn3;
        *var_qjs_slot = var_qjs;
        *var_qjs_dn2_slot = var_qjs_dn2;
        *var_qjs_dn3_slot = var_qjs_dn3;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_awl_slot: &mut f64,
        var_awl_rv_slot: &mut f64,
        var_deltat_slot: &mut f64,
        var_deltat_rv_slot: &mut f64,
        var_deltavfb_slot: &mut f64,
        var_deltavfb_rv_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_rv_slot: &mut f64,
        var_eps_cox_slot: &mut f64,
        var_eps_cox_l_slot: &mut f64,
        var_eps_cox_l_rv_slot: &mut f64,
        var_eps_cox_rv_slot: &mut f64,
        var_eps_cox_w_slot: &mut f64,
        var_eps_cox_w_rv_slot: &mut f64,
        var_epssil_slot: &mut f64,
        var_epssil_rv_slot: &mut f64,
        var_eta_qi_slot: &mut f64,
        var_eta_qi_rv_slot: &mut f64,
        var_gamma_s_slot: &mut f64,
        var_gamma_s_rv_slot: &mut f64,
        var_gamma_sqrt_phi_slot: &mut f64,
        var_gamma_sqrt_phi_dn0_slot: &mut f64,
        var_gamma_sqrt_phi_dn1_slot: &mut f64,
        var_gamma_sqrt_phi_dn2_slot: &mut f64,
        var_gamma_sqrt_phi_dn3_slot: &mut f64,
        var_gamma_sqrt_phi_rv_slot: &mut f64,
        var_guard1_slot: &mut f64,
        var_guard1_rv_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard2_rv_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard3_rv_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard4_rv_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard6_rv_slot: &mut f64,
        var_inv_ucrit_slot: &mut f64,
        var_inv_ucrit_rv_slot: &mut f64,
        var_inv_vt_slot: &mut f64,
        var_inv_vt_rv_slot: &mut f64,
        var_kp_t_slot: &mut f64,
        var_kp_t_rv_slot: &mut f64,
        var_kp_weff_slot: &mut f64,
        var_kp_weff_rv_slot: &mut f64,
        var_lc_slot: &mut f64,
        var_lc_lambda_slot: &mut f64,
        var_lc_lambda_rv_slot: &mut f64,
        var_lc_rv_slot: &mut f64,
        var_lc_ucrit_slot: &mut f64,
        var_lc_ucrit_rv_slot: &mut f64,
        var_leff_slot: &mut f64,
        var_leff_rv_slot: &mut f64,
        var_log_vc_vt_slot: &mut f64,
        var_log_vc_vt_rv_slot: &mut f64,
        var_mode_slot: &mut f64,
        var_mode_rv_slot: &mut f64,
        var_phi_t_slot: &mut f64,
        var_phi_t_dn0_slot: &mut f64,
        var_phi_t_dn1_slot: &mut f64,
        var_phi_t_dn2_slot: &mut f64,
        var_phi_t_dn3_slot: &mut f64,
        var_phi_t_rv_slot: &mut f64,
        var_ratiot_slot: &mut f64,
        var_ratiot_rv_slot: &mut f64,
        var_refeg_slot: &mut f64,
        var_refeg_rv_slot: &mut f64,
        var_sqrt_phi_slot: &mut f64,
        var_sqrt_phi_dn0_slot: &mut f64,
        var_sqrt_phi_dn1_slot: &mut f64,
        var_sqrt_phi_dn2_slot: &mut f64,
        var_sqrt_phi_dn3_slot: &mut f64,
        var_sqrt_phi_rv_slot: &mut f64,
        var_sqrt_vp_vt_slot: &mut f64,
        var_sqrt_vp_vt_dn0_slot: &mut f64,
        var_sqrt_vp_vt_dn1_slot: &mut f64,
        var_sqrt_vp_vt_dn2_slot: &mut f64,
        var_sqrt_vp_vt_dn3_slot: &mut f64,
        var_sqrt_vp_vt_rv_slot: &mut f64,
        var_sqv_slot: &mut f64,
        var_sqv_rv_slot: &mut f64,
        var_t_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t_rv_slot: &mut f64,
        var_theta_vp_1_slot: &mut f64,
        var_theta_vp_1_dn0_slot: &mut f64,
        var_theta_vp_1_dn1_slot: &mut f64,
        var_theta_vp_1_dn2_slot: &mut f64,
        var_theta_vp_1_dn3_slot: &mut f64,
        var_theta_vp_1_rv_slot: &mut f64,
        var_tmp1_slot: &mut f64,
        var_tmp1_dn0_slot: &mut f64,
        var_tmp1_dn1_slot: &mut f64,
        var_tmp1_dn2_slot: &mut f64,
        var_tmp1_dn3_slot: &mut f64,
        var_tmp1_rv_slot: &mut f64,
        var_tmp2_slot: &mut f64,
        var_tmp2_dn0_slot: &mut f64,
        var_tmp2_dn1_slot: &mut f64,
        var_tmp2_dn2_slot: &mut f64,
        var_tmp2_dn3_slot: &mut f64,
        var_tmp2_rv_slot: &mut f64,
        var_tnom_slot: &mut f64,
        var_tnom_rv_slot: &mut f64,
        var_ucrit_t_slot: &mut f64,
        var_ucrit_t_rv_slot: &mut f64,
        var_v0_slot: &mut f64,
        var_v0_rv_slot: &mut f64,
        var_vc_slot: &mut f64,
        var_vc_rv_slot: &mut f64,
        var_vd_slot: &mut f64,
        var_vd_dn0_slot: &mut f64,
        var_vd_dn2_slot: &mut f64,
        var_vd_dn3_slot: &mut f64,
        var_vd_rv_slot: &mut f64,
        var_vg_slot: &mut f64,
        var_vg_dn1_slot: &mut f64,
        var_vg_dn3_slot: &mut f64,
        var_vg_rv_slot: &mut f64,
        var_vl_slot: &mut f64,
        var_vl_rv_slot: &mut f64,
        var_vpprime_slot: &mut f64,
        var_vpprime_dn0_slot: &mut f64,
        var_vpprime_dn1_slot: &mut f64,
        var_vpprime_dn2_slot: &mut f64,
        var_vpprime_dn3_slot: &mut f64,
        var_vpprime_rv_slot: &mut f64,
        var_vs_slot: &mut f64,
        var_vs_dn0_slot: &mut f64,
        var_vs_dn2_slot: &mut f64,
        var_vs_dn3_slot: &mut f64,
        var_vs_rv_slot: &mut f64,
        var_vt_slot: &mut f64,
        var_vt_01_slot: &mut f64,
        var_vt_01_rv_slot: &mut f64,
        var_vt_2_slot: &mut f64,
        var_vt_2_rv_slot: &mut f64,
        var_vt_4_slot: &mut f64,
        var_vt_4_rv_slot: &mut f64,
        var_vt_rv_slot: &mut f64,
        var_vt_vt_slot: &mut f64,
        var_vt_vt_16_slot: &mut f64,
        var_vt_vt_16_rv_slot: &mut f64,
        var_vt_vt_2_slot: &mut f64,
        var_vt_vt_2_rv_slot: &mut f64,
        var_vt_vt_rv_slot: &mut f64,
        var_vto_s_slot: &mut f64,
        var_vto_s_rv_slot: &mut f64,
        var_vto_t_slot: &mut f64,
        var_vto_t_rv_slot: &mut f64,
        var_weff_slot: &mut f64,
        var_weff_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_awl: f64 = *var_awl_slot;
        let mut var_awl_rv: f64 = *var_awl_rv_slot;
        let mut var_deltat: f64 = *var_deltat_slot;
        let mut var_deltat_rv: f64 = *var_deltat_rv_slot;
        let mut var_deltavfb: f64 = *var_deltavfb_slot;
        let mut var_deltavfb_rv: f64 = *var_deltavfb_rv_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_rv: f64 = *var_eg_rv_slot;
        let mut var_eps_cox: f64 = *var_eps_cox_slot;
        let mut var_eps_cox_l: f64 = *var_eps_cox_l_slot;
        let mut var_eps_cox_l_rv: f64 = *var_eps_cox_l_rv_slot;
        let mut var_eps_cox_rv: f64 = *var_eps_cox_rv_slot;
        let mut var_eps_cox_w: f64 = *var_eps_cox_w_slot;
        let mut var_eps_cox_w_rv: f64 = *var_eps_cox_w_rv_slot;
        let mut var_epssil: f64 = *var_epssil_slot;
        let mut var_epssil_rv: f64 = *var_epssil_rv_slot;
        let mut var_eta_qi: f64 = *var_eta_qi_slot;
        let mut var_eta_qi_rv: f64 = *var_eta_qi_rv_slot;
        let mut var_gamma_s: f64 = *var_gamma_s_slot;
        let mut var_gamma_s_rv: f64 = *var_gamma_s_rv_slot;
        let mut var_gamma_sqrt_phi: f64 = *var_gamma_sqrt_phi_slot;
        let mut var_gamma_sqrt_phi_dn0: f64 = *var_gamma_sqrt_phi_dn0_slot;
        let mut var_gamma_sqrt_phi_dn1: f64 = *var_gamma_sqrt_phi_dn1_slot;
        let mut var_gamma_sqrt_phi_dn2: f64 = *var_gamma_sqrt_phi_dn2_slot;
        let mut var_gamma_sqrt_phi_dn3: f64 = *var_gamma_sqrt_phi_dn3_slot;
        let mut var_gamma_sqrt_phi_rv: f64 = *var_gamma_sqrt_phi_rv_slot;
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard1_rv: f64 = *var_guard1_rv_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard2_rv: f64 = *var_guard2_rv_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard3_rv: f64 = *var_guard3_rv_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard4_rv: f64 = *var_guard4_rv_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard6_rv: f64 = *var_guard6_rv_slot;
        let mut var_inv_ucrit: f64 = *var_inv_ucrit_slot;
        let mut var_inv_ucrit_rv: f64 = *var_inv_ucrit_rv_slot;
        let mut var_inv_vt: f64 = *var_inv_vt_slot;
        let mut var_inv_vt_rv: f64 = *var_inv_vt_rv_slot;
        let mut var_kp_t: f64 = *var_kp_t_slot;
        let mut var_kp_t_rv: f64 = *var_kp_t_rv_slot;
        let mut var_kp_weff: f64 = *var_kp_weff_slot;
        let mut var_kp_weff_rv: f64 = *var_kp_weff_rv_slot;
        let mut var_lc: f64 = *var_lc_slot;
        let mut var_lc_lambda: f64 = *var_lc_lambda_slot;
        let mut var_lc_lambda_rv: f64 = *var_lc_lambda_rv_slot;
        let mut var_lc_rv: f64 = *var_lc_rv_slot;
        let mut var_lc_ucrit: f64 = *var_lc_ucrit_slot;
        let mut var_lc_ucrit_rv: f64 = *var_lc_ucrit_rv_slot;
        let mut var_leff: f64 = *var_leff_slot;
        let mut var_leff_rv: f64 = *var_leff_rv_slot;
        let mut var_log_vc_vt: f64 = *var_log_vc_vt_slot;
        let mut var_log_vc_vt_rv: f64 = *var_log_vc_vt_rv_slot;
        let mut var_mode: f64 = *var_mode_slot;
        let mut var_mode_rv: f64 = *var_mode_rv_slot;
        let mut var_phi_t: f64 = *var_phi_t_slot;
        let mut var_phi_t_dn0: f64 = *var_phi_t_dn0_slot;
        let mut var_phi_t_dn1: f64 = *var_phi_t_dn1_slot;
        let mut var_phi_t_dn2: f64 = *var_phi_t_dn2_slot;
        let mut var_phi_t_dn3: f64 = *var_phi_t_dn3_slot;
        let mut var_phi_t_rv: f64 = *var_phi_t_rv_slot;
        let mut var_ratiot: f64 = *var_ratiot_slot;
        let mut var_ratiot_rv: f64 = *var_ratiot_rv_slot;
        let mut var_refeg: f64 = *var_refeg_slot;
        let mut var_refeg_rv: f64 = *var_refeg_rv_slot;
        let mut var_sqrt_phi: f64 = *var_sqrt_phi_slot;
        let mut var_sqrt_phi_dn0: f64 = *var_sqrt_phi_dn0_slot;
        let mut var_sqrt_phi_dn1: f64 = *var_sqrt_phi_dn1_slot;
        let mut var_sqrt_phi_dn2: f64 = *var_sqrt_phi_dn2_slot;
        let mut var_sqrt_phi_dn3: f64 = *var_sqrt_phi_dn3_slot;
        let mut var_sqrt_phi_rv: f64 = *var_sqrt_phi_rv_slot;
        let mut var_sqrt_vp_vt: f64 = *var_sqrt_vp_vt_slot;
        let mut var_sqrt_vp_vt_dn0: f64 = *var_sqrt_vp_vt_dn0_slot;
        let mut var_sqrt_vp_vt_dn1: f64 = *var_sqrt_vp_vt_dn1_slot;
        let mut var_sqrt_vp_vt_dn2: f64 = *var_sqrt_vp_vt_dn2_slot;
        let mut var_sqrt_vp_vt_dn3: f64 = *var_sqrt_vp_vt_dn3_slot;
        let mut var_sqrt_vp_vt_rv: f64 = *var_sqrt_vp_vt_rv_slot;
        let mut var_sqv: f64 = *var_sqv_slot;
        let mut var_sqv_rv: f64 = *var_sqv_rv_slot;
        let mut var_t: f64 = *var_t_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t_rv: f64 = *var_t_rv_slot;
        let mut var_theta_vp_1: f64 = *var_theta_vp_1_slot;
        let mut var_theta_vp_1_dn0: f64 = *var_theta_vp_1_dn0_slot;
        let mut var_theta_vp_1_dn1: f64 = *var_theta_vp_1_dn1_slot;
        let mut var_theta_vp_1_dn2: f64 = *var_theta_vp_1_dn2_slot;
        let mut var_theta_vp_1_dn3: f64 = *var_theta_vp_1_dn3_slot;
        let mut var_theta_vp_1_rv: f64 = *var_theta_vp_1_rv_slot;
        let mut var_tmp1: f64 = *var_tmp1_slot;
        let mut var_tmp1_dn0: f64 = *var_tmp1_dn0_slot;
        let mut var_tmp1_dn1: f64 = *var_tmp1_dn1_slot;
        let mut var_tmp1_dn2: f64 = *var_tmp1_dn2_slot;
        let mut var_tmp1_dn3: f64 = *var_tmp1_dn3_slot;
        let mut var_tmp1_rv: f64 = *var_tmp1_rv_slot;
        let mut var_tmp2: f64 = *var_tmp2_slot;
        let mut var_tmp2_dn0: f64 = *var_tmp2_dn0_slot;
        let mut var_tmp2_dn1: f64 = *var_tmp2_dn1_slot;
        let mut var_tmp2_dn2: f64 = *var_tmp2_dn2_slot;
        let mut var_tmp2_dn3: f64 = *var_tmp2_dn3_slot;
        let mut var_tmp2_rv: f64 = *var_tmp2_rv_slot;
        let mut var_tnom: f64 = *var_tnom_slot;
        let mut var_tnom_rv: f64 = *var_tnom_rv_slot;
        let mut var_ucrit_t: f64 = *var_ucrit_t_slot;
        let mut var_ucrit_t_rv: f64 = *var_ucrit_t_rv_slot;
        let mut var_v0: f64 = *var_v0_slot;
        let mut var_v0_rv: f64 = *var_v0_rv_slot;
        let mut var_vc: f64 = *var_vc_slot;
        let mut var_vc_rv: f64 = *var_vc_rv_slot;
        let mut var_vd: f64 = *var_vd_slot;
        let mut var_vd_dn0: f64 = *var_vd_dn0_slot;
        let mut var_vd_dn2: f64 = *var_vd_dn2_slot;
        let mut var_vd_dn3: f64 = *var_vd_dn3_slot;
        let mut var_vd_rv: f64 = *var_vd_rv_slot;
        let mut var_vg: f64 = *var_vg_slot;
        let mut var_vg_dn1: f64 = *var_vg_dn1_slot;
        let mut var_vg_dn3: f64 = *var_vg_dn3_slot;
        let mut var_vg_rv: f64 = *var_vg_rv_slot;
        let mut var_vl: f64 = *var_vl_slot;
        let mut var_vl_rv: f64 = *var_vl_rv_slot;
        let mut var_vpprime: f64 = *var_vpprime_slot;
        let mut var_vpprime_dn0: f64 = *var_vpprime_dn0_slot;
        let mut var_vpprime_dn1: f64 = *var_vpprime_dn1_slot;
        let mut var_vpprime_dn2: f64 = *var_vpprime_dn2_slot;
        let mut var_vpprime_dn3: f64 = *var_vpprime_dn3_slot;
        let mut var_vpprime_rv: f64 = *var_vpprime_rv_slot;
        let mut var_vs: f64 = *var_vs_slot;
        let mut var_vs_dn0: f64 = *var_vs_dn0_slot;
        let mut var_vs_dn2: f64 = *var_vs_dn2_slot;
        let mut var_vs_dn3: f64 = *var_vs_dn3_slot;
        let mut var_vs_rv: f64 = *var_vs_rv_slot;
        let mut var_vt: f64 = *var_vt_slot;
        let mut var_vt_01: f64 = *var_vt_01_slot;
        let mut var_vt_01_rv: f64 = *var_vt_01_rv_slot;
        let mut var_vt_2: f64 = *var_vt_2_slot;
        let mut var_vt_2_rv: f64 = *var_vt_2_rv_slot;
        let mut var_vt_4: f64 = *var_vt_4_slot;
        let mut var_vt_4_rv: f64 = *var_vt_4_rv_slot;
        let mut var_vt_rv: f64 = *var_vt_rv_slot;
        let mut var_vt_vt: f64 = *var_vt_vt_slot;
        let mut var_vt_vt_16: f64 = *var_vt_vt_16_slot;
        let mut var_vt_vt_16_rv: f64 = *var_vt_vt_16_rv_slot;
        let mut var_vt_vt_2: f64 = *var_vt_vt_2_slot;
        let mut var_vt_vt_2_rv: f64 = *var_vt_vt_2_rv_slot;
        let mut var_vt_vt_rv: f64 = *var_vt_vt_rv_slot;
        let mut var_vto_s: f64 = *var_vto_s_slot;
        let mut var_vto_s_rv: f64 = *var_vto_s_rv_slot;
        let mut var_vto_t: f64 = *var_vto_t_slot;
        let mut var_vto_t_rv: f64 = *var_vto_t_rv_slot;
        let mut var_weff: f64 = *var_weff_slot;
        let mut var_weff_rv: f64 = *var_weff_rv_slot;

        let assign10_e194: f64 = (11.7 * 8.8541879239442e-12);
        var_epssil = assign10_e194;
        var_epssil_rv = 0.0;

        var_theta_vp_1 = 0.0;
        var_theta_vp_1_dn0 = 0.0;
        var_theta_vp_1_dn1 = 0.0;
        var_theta_vp_1_dn2 = 0.0;
        var_theta_vp_1_dn3 = 0.0;
        var_theta_vp_1_rv = 0.0;

        var_vpprime = 0.0;
        var_vpprime_dn0 = 0.0;
        var_vpprime_dn1 = 0.0;
        var_vpprime_dn2 = 0.0;
        var_vpprime_dn3 = 0.0;
        var_vpprime_rv = 0.0;

        var_sqrt_vp_vt = 0.0;
        var_sqrt_vp_vt_dn0 = 0.0;
        var_sqrt_vp_vt_dn1 = 0.0;
        var_sqrt_vp_vt_dn2 = 0.0;
        var_sqrt_vp_vt_dn3 = 0.0;
        var_sqrt_vp_vt_rv = 0.0;

        let assign60_e201: f64 = (var_epssil / p.p13);
        var_eps_cox = assign60_e201;
        var_eps_cox_rv = 0.0;

        let assign70_e204: f64 = (var_eps_cox * p.p14);
        let assign70_e205: f64 = (assign70_e204).sqrt();
        var_lc = assign70_e205;
        var_lc_rv = 0.0;

        let assign80_e208: f64 = (var_lc * p.p25);
        var_lc_lambda = assign80_e208;
        var_lc_lambda_rv = 0.0;

        let assign90_e211: f64 = (3.0 * var_eps_cox);
        let assign90_e213: f64 = (assign90_e211 * p.p28);
        var_eps_cox_w = assign90_e213;
        var_eps_cox_w_rv = 0.0;

        let assign100_e216: f64 = (var_eps_cox * p.p29);
        var_eps_cox_l = assign100_e216;
        var_eps_cox_l_rv = 0.0;

        let assign120_e223: f64 = (var_epssil * p.p22);
        let assign120_e224: f64 = (p.p13 / assign120_e223);
        var_t0 = assign120_e224;
        var_t0_rv = 0.0;

        let assign130_e227: f64 = (p.p30 + p.p30);
        let assign130_e229: f64 = (assign130_e227 / p.p13);
        var_v0 = assign130_e229;
        var_v0_rv = 0.0;

        let (assign140_e235,) = {
    if (p.p0 > 0.0) {
        (0.5,)
    } else {
        (0.3333333333333,)
    }
};
        var_eta_qi = assign140_e235;
        var_eta_qi_rv = 0.0;

        let assign150_e238: f64 = (-1e21);
        let assign150_e239: f64 = (-assign150_e238);
        let assign150_e240: f64 = if p.p3 == assign150_e239 { 1.0 } else { 0.0 };
        var_guard1 = assign150_e240;
        var_guard1_rv = 0.0;

        let (assign160_e246,) = {
    if (var_guard1 != 0.0) {
        let assign160_e242: f64 = ctx_temp;
        let assign160_e244: f64 = (assign160_e242 + p.p2);
        (assign160_e244,)
    } else {
        (var_t,)
    }
};
        var_t = assign160_e246;
        var_t_rv = 0.0;

        let (assign170_e253,) = {
    if (var_guard1 == 0.0) {
        let assign170_e251: f64 = (p.p3 + 273.15);
        (assign170_e251,)
    } else {
        (var_t,)
    }
};
        var_t = assign170_e253;
        var_t_rv = 0.0;

        let assign180_e256: f64 = (-1e21);
        let assign180_e257: f64 = (-assign180_e256);
        let assign180_e258: f64 = if p.p4 == assign180_e257 { 1.0 } else { 0.0 };
        var_guard2 = assign180_e258;
        var_guard2_rv = 0.0;

        let (assign190_e264,) = {
    if (var_guard2 != 0.0) {
        let assign190_e262: f64 = (25.0 + 273.15);
        (assign190_e262,)
    } else {
        (var_tnom,)
    }
};
        var_tnom = assign190_e264;
        var_tnom_rv = 0.0;

        let (assign200_e271,) = {
    if (var_guard2 == 0.0) {
        let assign200_e269: f64 = (p.p4 + 273.15);
        (assign200_e269,)
    } else {
        (var_tnom,)
    }
};
        var_tnom = assign200_e271;
        var_tnom_rv = 0.0;

        let assign210_e273: f64 = (var_t * THERMAL_VOLTAGE_PER_K);
        var_vt = assign210_e273;
        var_vt_rv = 0.0;

        let assign220_e276: f64 = (0.1 * var_vt);
        var_vt_01 = assign220_e276;
        var_vt_01_rv = 0.0;

        let assign230_e279: f64 = (1.0 / var_vt);
        var_inv_vt = assign230_e279;
        var_inv_vt_rv = 0.0;

        let assign240_e282: f64 = (var_vt + var_vt);
        var_vt_2 = assign240_e282;
        var_vt_2_rv = 0.0;

        let assign250_e285: f64 = (var_vt_2 + var_vt_2);
        var_vt_4 = assign250_e285;
        var_vt_4_rv = 0.0;

        let assign260_e288: f64 = (var_vt * var_vt);
        var_vt_vt = assign260_e288;
        var_vt_vt_rv = 0.0;

        let assign270_e291: f64 = (var_vt_vt + var_vt_vt);
        var_vt_vt_2 = assign270_e291;
        var_vt_vt_2_rv = 0.0;

        let assign280_e294: f64 = (16.0 * var_vt_vt);
        var_vt_vt_16 = assign280_e294;
        var_vt_vt_16_rv = 0.0;

        let assign290_e298: f64 = (0.000702 * var_t);
        let assign290_e300: f64 = (assign290_e298 * var_t);
        let assign290_e303: f64 = (var_t + 1108.0);
        let assign290_e304: f64 = (assign290_e300 / assign290_e303);
        let assign290_e305: f64 = (1.16 - assign290_e304);
        var_eg = assign290_e305;
        var_eg_rv = 0.0;

        let assign300_e309: f64 = (0.000702 * var_tnom);
        let assign300_e311: f64 = (assign300_e309 * var_tnom);
        let assign300_e314: f64 = (var_tnom + 1108.0);
        let assign300_e315: f64 = (assign300_e311 / assign300_e314);
        let assign300_e316: f64 = (1.16 - assign300_e315);
        var_refeg = assign300_e316;
        var_refeg_rv = 0.0;

        let assign310_e319: f64 = (var_t - var_tnom);
        var_deltat = assign310_e319;
        var_deltat_rv = 0.0;

        let assign320_e322: f64 = (var_t / var_tnom);
        var_ratiot = assign320_e322;
        var_ratiot_rv = 0.0;

        let assign330_e326: f64 = (p.p16 * var_deltat);
        let assign330_e327: f64 = (p.p15 - assign330_e326);
        var_vto_t = assign330_e327;
        var_vto_t_rv = 0.0;

        let assign340_e331: f64 = (var_ratiot).powf(p.p20);
        let assign340_e332: f64 = (p.p19 * assign340_e331);
        var_kp_t = assign340_e332;
        var_kp_t_rv = 0.0;

        let assign350_e336: f64 = (var_ratiot).powf(p.p24);
        let assign350_e337: f64 = (p.p23 * assign350_e336);
        var_ucrit_t = assign350_e337;
        var_ucrit_t_rv = 0.0;

        let assign370_e347: f64 = (p.p18 * var_ratiot);
        let assign370_e350: f64 = (3.0 * var_vt);
        let assign370_e352: f64 = (var_ratiot).ln();
        let assign370_e353: f64 = (assign370_e350 * assign370_e352);
        let assign370_e354: f64 = (assign370_e347 - assign370_e353);
        let assign370_e357: f64 = (var_refeg * var_ratiot);
        let assign370_e358: f64 = (assign370_e354 - assign370_e357);
        let assign370_e360: f64 = (assign370_e358 + var_eg);
        var_phi_t = assign370_e360;
        var_phi_t_dn0 = 0.0;
        var_phi_t_dn1 = 0.0;
        var_phi_t_dn2 = 0.0;
        var_phi_t_dn3 = 0.0;
        var_phi_t_rv = 0.0;

        var_tmp1 = 0.2;
        var_tmp1_dn0 = 0.0;
        var_tmp1_dn1 = 0.0;
        var_tmp1_dn2 = 0.0;
        var_tmp1_dn3 = 0.0;
        var_tmp1_rv = 0.0;

        let assign390_e364: f64 = (var_phi_t - var_tmp1);
        var_tmp2 = assign390_e364;
        var_tmp2_dn0 = (var_phi_t_dn0 - var_tmp1_dn0);
        var_tmp2_dn1 = (var_phi_t_dn1 - var_tmp1_dn1);
        var_tmp2_dn2 = (var_phi_t_dn2 - var_tmp1_dn2);
        var_tmp2_dn3 = (var_phi_t_dn3 - var_tmp1_dn3);
        var_tmp2_rv = 0.0;

        let assign400_e369: f64 = (var_tmp2 * var_tmp2);
        let assign400_e372: f64 = (var_vt * var_vt);
        let assign400_e373: f64 = (assign400_e369 + assign400_e372);
        let assign400_e374: f64 = (assign400_e373).sqrt();
        let assign400_e375: f64 = (var_tmp2 + assign400_e374);
        let assign400_e376: f64 = (0.5 * assign400_e375);
        let assign400_e378: f64 = (assign400_e376 + var_tmp1);
        var_phi_t = assign400_e378;
        var_phi_t_dn0 = ((0.5 * (var_tmp2_dn0 + (((var_tmp2_dn0 * var_tmp2) + (var_tmp2 * var_tmp2_dn0)) / (2.0 * assign400_e374)))) + var_tmp1_dn0);
        var_phi_t_dn1 = ((0.5 * (var_tmp2_dn1 + (((var_tmp2_dn1 * var_tmp2) + (var_tmp2 * var_tmp2_dn1)) / (2.0 * assign400_e374)))) + var_tmp1_dn1);
        var_phi_t_dn2 = ((0.5 * (var_tmp2_dn2 + (((var_tmp2_dn2 * var_tmp2) + (var_tmp2 * var_tmp2_dn2)) / (2.0 * assign400_e374)))) + var_tmp1_dn2);
        var_phi_t_dn3 = ((0.5 * (var_tmp2_dn3 + (((var_tmp2_dn3 * var_tmp2) + (var_tmp2 * var_tmp2_dn3)) / (2.0 * assign400_e374)))) + var_tmp1_dn3);
        var_phi_t_rv = 0.0;

        let assign410_e380: f64 = (var_phi_t).sqrt();
        var_sqrt_phi = assign410_e380;
        var_sqrt_phi_dn0 = (var_phi_t_dn0 / (2.0 * assign410_e380));
        var_sqrt_phi_dn1 = (var_phi_t_dn1 / (2.0 * assign410_e380));
        var_sqrt_phi_dn2 = (var_phi_t_dn2 / (2.0 * assign410_e380));
        var_sqrt_phi_dn3 = (var_phi_t_dn3 / (2.0 * assign410_e380));
        var_sqrt_phi_rv = 0.0;

        let assign420_e383: f64 = (1.0 / var_ucrit_t);
        var_inv_ucrit = assign420_e383;
        var_inv_ucrit_rv = 0.0;

        let assign430_e386: f64 = (var_lc * var_ucrit_t);
        var_lc_ucrit = assign430_e386;
        var_lc_ucrit_rv = 0.0;

        let assign460_e395: f64 = (p.p5 + p.p26);
        var_leff = assign460_e395;
        var_leff_rv = 0.0;

        let assign470_e398: f64 = (p.p6 + p.p27);
        var_weff = assign470_e398;
        var_weff_rv = 0.0;

        let assign480_e401: f64 = (var_ucrit_t * var_leff);
        var_vc = assign480_e401;
        var_vc_rv = 0.0;

        let assign490_e405: f64 = (0.5 * var_vc);
        let assign490_e407: f64 = (assign490_e405 * var_inv_vt);
        let assign490_e408: f64 = (assign490_e407).ln();
        let assign490_e410: f64 = (assign490_e408 - 0.6);
        let assign490_e411: f64 = (var_vt * assign490_e410);
        var_log_vc_vt = assign490_e411;
        var_log_vc_vt_rv = 0.0;

        let assign500_e415: f64 = (var_weff * var_leff);
        let assign500_e416: f64 = (assign500_e415).sqrt();
        let assign500_e417: f64 = (1.0 / assign500_e416);
        var_awl = assign500_e417;
        var_awl_rv = 0.0;

        let assign510_e420: f64 = if p.p0 > 0.0 { 1.0 } else { 0.0 };
        var_guard3 = assign510_e420;
        var_guard3_rv = 0.0;

        let (assign520_e435,) = {
    if (var_guard3 != 0.0) {
        let (assign520_e433,) = {
            if (p.p38 != 1e-6) {
                let assign520_e428: f64 = (p.p38 - 1e-6);
                let assign520_e429: f64 = (var_awl * assign520_e428);
                let assign520_e431: f64 = (assign520_e429 + var_vto_t);
                (assign520_e431,)
            } else {
                (var_vto_t,)
            }
        };
        (assign520_e433,)
    } else {
        (var_vto_s,)
    }
};
        var_vto_s = assign520_e435;
        var_vto_s_rv = 0.0;

        let (assign530_e452,) = {
    if (var_guard3 == 0.0) {
        let (assign530_e450,) = {
            if (p.p38 != 1e-6) {
                let assign530_e444: f64 = (1e-6 - p.p38);
                let assign530_e445: f64 = (var_awl * assign530_e444);
                let assign530_e447: f64 = (assign530_e445 - var_vto_t);
                (assign530_e447,)
            } else {
                let assign530_e449: f64 = (-var_vto_t);
                (assign530_e449,)
            }
        };
        (assign530_e450,)
    } else {
        (var_vto_s,)
    }
};
        var_vto_s = assign530_e452;
        var_vto_s_rv = 0.0;

        let (assign540_e467,) = {
    if (p.p39 != 1e-6) {
        let assign540_e461: f64 = (p.p39 - 1e-6);
        let assign540_e463: f64 = (assign540_e461 * var_awl);
        let assign540_e464: f64 = (1.0 + assign540_e463);
        let assign540_e465: f64 = (var_kp_t * assign540_e464);
        (assign540_e465,)
    } else {
        (var_kp_t,)
    }
};
        let assign540_e468: f64 = (var_weff * assign540_e467);
        var_kp_weff = assign540_e468;
        var_kp_weff_rv = 0.0;

        let (assign550_e480,) = {
    if (p.p40 != 1e-6) {
        let assign550_e475: f64 = (p.p40 - 1e-6);
        let assign550_e477: f64 = (assign550_e475 * var_awl);
        let assign550_e478: f64 = (p.p17 + assign550_e477);
        (assign550_e478,)
    } else {
        (p.p17,)
    }
};
        var_gamma_s = assign550_e480;
        var_gamma_s_rv = 0.0;

        let assign560_e483: f64 = (var_gamma_s * var_sqrt_phi);
        var_gamma_sqrt_phi = assign560_e483;
        var_gamma_sqrt_phi_dn0 = (var_gamma_s * var_sqrt_phi_dn0);
        var_gamma_sqrt_phi_dn1 = (var_gamma_s * var_sqrt_phi_dn1);
        var_gamma_sqrt_phi_dn2 = (var_gamma_s * var_sqrt_phi_dn2);
        var_gamma_sqrt_phi_dn3 = (var_gamma_s * var_sqrt_phi_dn3);
        var_gamma_sqrt_phi_rv = 0.0;

        let assign570_e486: f64 = if var_v0 == 0.0 { 1.0 } else { 0.0 };
        var_guard4 = assign570_e486;
        var_guard4_rv = 0.0;

        let (assign580_e490,) = {
    if (var_guard4 != 0.0) {
        (0.0,)
    } else {
        (var_deltavfb,)
    }
};
        var_deltavfb = assign580_e490;
        var_deltavfb_rv = 0.0;

        let (assign590_e503,) = {
    if (var_guard4 == 0.0) {
        let assign590_e497: f64 = (p.p31 * p.p8);
        let assign590_e498: f64 = (var_leff / assign590_e497);
        let assign590_e500: f64 = (assign590_e498 - 0.1);
        let assign590_e501: f64 = (0.28 * assign590_e500);
        (assign590_e501,)
    } else {
        (var_vl,)
    }
};
        var_vl = assign590_e503;
        var_vl_rv = 0.0;

        let (assign600_e521,) = {
    if (var_guard4 == 0.0) {
        let assign600_e512: f64 = (var_vl * var_vl);
        let assign600_e514: f64 = (assign600_e512 + 0.001936);
        let assign600_e515: f64 = (assign600_e514).sqrt();
        let assign600_e516: f64 = (var_vl + assign600_e515);
        let assign600_e517: f64 = (0.5 * assign600_e516);
        let assign600_e518: f64 = (1.0 + assign600_e517);
        let assign600_e519: f64 = (1.0 / assign600_e518);
        (assign600_e519,)
    } else {
        (var_sqv,)
    }
};
        var_sqv = assign600_e521;
        var_sqv_rv = 0.0;

        let (assign610_e530,) = {
    if (var_guard4 == 0.0) {
        let assign610_e526: f64 = (var_v0 * var_sqv);
        let assign610_e528: f64 = (assign610_e526 * var_sqv);
        (assign610_e528,)
    } else {
        (var_deltavfb,)
    }
};
        var_deltavfb = assign610_e530;
        var_deltavfb_rv = 0.0;

        let assign620_e533: f64 = (p.p0 * (nv1 - nv3));
        var_vg = assign620_e533;
        var_vg_dn1 = p.p0;
        var_vg_dn3 = (-p.p0);
        var_vg_rv = 0.0;

        let assign630_e536: f64 = (p.p0 * (nv2 - nv3));
        var_vs = assign630_e536;
        var_vs_dn0 = 0.0;
        var_vs_dn2 = p.p0;
        var_vs_dn3 = (-p.p0);
        var_vs_rv = 0.0;

        let assign640_e539: f64 = (p.p0 * (nv0 - nv3));
        var_vd = assign640_e539;
        var_vd_dn0 = p.p0;
        var_vd_dn2 = 0.0;
        var_vd_dn3 = (-p.p0);
        var_vd_rv = 0.0;

        let assign650_e542: f64 = (var_vd - var_vs);
        let assign650_e544: f64 = if assign650_e542 < 0.0 { 1.0 } else { 0.0 };
        var_guard6 = assign650_e544;
        var_guard6_rv = 0.0;

        let (assign660_e549,) = {
    if (var_guard6 != 0.0) {
        let assign660_e547: f64 = (-1.0);
        (assign660_e547,)
    } else {
        (var_mode,)
    }
};
        var_mode = assign660_e549;
        var_mode_rv = 0.0;

        let (assign670_e553, assign670_e553_d_n0, assign670_e553_d_n2, assign670_e553_d_n3,) = {
    if (var_guard6 != 0.0) {
        (var_vs, var_vs_dn0, var_vs_dn2, var_vs_dn3,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3,)
    }
};
        var_t1 = assign670_e553;
        var_t1_dn0 = assign670_e553_d_n0;
        var_t1_dn2 = assign670_e553_d_n2;
        var_t1_dn3 = assign670_e553_d_n3;
        var_t1_rv = 0.0;

        let (assign680_e557, assign680_e557_d_n0, assign680_e557_d_n2, assign680_e557_d_n3,) = {
    if (var_guard6 != 0.0) {
        (var_vd, var_vd_dn0, var_vd_dn2, var_vd_dn3,)
    } else {
        (var_vs, var_vs_dn0, var_vs_dn2, var_vs_dn3,)
    }
};
        var_vs = assign680_e557;
        var_vs_dn0 = assign680_e557_d_n0;
        var_vs_dn2 = assign680_e557_d_n2;
        var_vs_dn3 = assign680_e557_d_n3;
        var_vs_rv = 0.0;

        let (assign690_e561, assign690_e561_d_n0, assign690_e561_d_n2, assign690_e561_d_n3,) = {
    if (var_guard6 != 0.0) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3,)
    } else {
        (var_vd, var_vd_dn0, var_vd_dn2, var_vd_dn3,)
    }
};
        var_vd = assign690_e561;
        var_vd_dn0 = assign690_e561_d_n0;
        var_vd_dn2 = assign690_e561_d_n2;
        var_vd_dn3 = assign690_e561_d_n3;
        var_vd_rv = 0.0;

        let (assign700_e566,) = {
    if (var_guard6 == 0.0) {
        (1.0,)
    } else {
        (var_mode,)
    }
};
        var_mode = assign700_e566;
        var_mode_rv = 0.0;

        *var_awl_slot = var_awl;
        *var_awl_rv_slot = var_awl_rv;
        *var_deltat_slot = var_deltat;
        *var_deltat_rv_slot = var_deltat_rv;
        *var_deltavfb_slot = var_deltavfb;
        *var_deltavfb_rv_slot = var_deltavfb_rv;
        *var_eg_slot = var_eg;
        *var_eg_rv_slot = var_eg_rv;
        *var_eps_cox_slot = var_eps_cox;
        *var_eps_cox_l_slot = var_eps_cox_l;
        *var_eps_cox_l_rv_slot = var_eps_cox_l_rv;
        *var_eps_cox_rv_slot = var_eps_cox_rv;
        *var_eps_cox_w_slot = var_eps_cox_w;
        *var_eps_cox_w_rv_slot = var_eps_cox_w_rv;
        *var_epssil_slot = var_epssil;
        *var_epssil_rv_slot = var_epssil_rv;
        *var_eta_qi_slot = var_eta_qi;
        *var_eta_qi_rv_slot = var_eta_qi_rv;
        *var_gamma_s_slot = var_gamma_s;
        *var_gamma_s_rv_slot = var_gamma_s_rv;
        *var_gamma_sqrt_phi_slot = var_gamma_sqrt_phi;
        *var_gamma_sqrt_phi_dn0_slot = var_gamma_sqrt_phi_dn0;
        *var_gamma_sqrt_phi_dn1_slot = var_gamma_sqrt_phi_dn1;
        *var_gamma_sqrt_phi_dn2_slot = var_gamma_sqrt_phi_dn2;
        *var_gamma_sqrt_phi_dn3_slot = var_gamma_sqrt_phi_dn3;
        *var_gamma_sqrt_phi_rv_slot = var_gamma_sqrt_phi_rv;
        *var_guard1_slot = var_guard1;
        *var_guard1_rv_slot = var_guard1_rv;
        *var_guard2_slot = var_guard2;
        *var_guard2_rv_slot = var_guard2_rv;
        *var_guard3_slot = var_guard3;
        *var_guard3_rv_slot = var_guard3_rv;
        *var_guard4_slot = var_guard4;
        *var_guard4_rv_slot = var_guard4_rv;
        *var_guard6_slot = var_guard6;
        *var_guard6_rv_slot = var_guard6_rv;
        *var_inv_ucrit_slot = var_inv_ucrit;
        *var_inv_ucrit_rv_slot = var_inv_ucrit_rv;
        *var_inv_vt_slot = var_inv_vt;
        *var_inv_vt_rv_slot = var_inv_vt_rv;
        *var_kp_t_slot = var_kp_t;
        *var_kp_t_rv_slot = var_kp_t_rv;
        *var_kp_weff_slot = var_kp_weff;
        *var_kp_weff_rv_slot = var_kp_weff_rv;
        *var_lc_slot = var_lc;
        *var_lc_lambda_slot = var_lc_lambda;
        *var_lc_lambda_rv_slot = var_lc_lambda_rv;
        *var_lc_rv_slot = var_lc_rv;
        *var_lc_ucrit_slot = var_lc_ucrit;
        *var_lc_ucrit_rv_slot = var_lc_ucrit_rv;
        *var_leff_slot = var_leff;
        *var_leff_rv_slot = var_leff_rv;
        *var_log_vc_vt_slot = var_log_vc_vt;
        *var_log_vc_vt_rv_slot = var_log_vc_vt_rv;
        *var_mode_slot = var_mode;
        *var_mode_rv_slot = var_mode_rv;
        *var_phi_t_slot = var_phi_t;
        *var_phi_t_dn0_slot = var_phi_t_dn0;
        *var_phi_t_dn1_slot = var_phi_t_dn1;
        *var_phi_t_dn2_slot = var_phi_t_dn2;
        *var_phi_t_dn3_slot = var_phi_t_dn3;
        *var_phi_t_rv_slot = var_phi_t_rv;
        *var_ratiot_slot = var_ratiot;
        *var_ratiot_rv_slot = var_ratiot_rv;
        *var_refeg_slot = var_refeg;
        *var_refeg_rv_slot = var_refeg_rv;
        *var_sqrt_phi_slot = var_sqrt_phi;
        *var_sqrt_phi_dn0_slot = var_sqrt_phi_dn0;
        *var_sqrt_phi_dn1_slot = var_sqrt_phi_dn1;
        *var_sqrt_phi_dn2_slot = var_sqrt_phi_dn2;
        *var_sqrt_phi_dn3_slot = var_sqrt_phi_dn3;
        *var_sqrt_phi_rv_slot = var_sqrt_phi_rv;
        *var_sqrt_vp_vt_slot = var_sqrt_vp_vt;
        *var_sqrt_vp_vt_dn0_slot = var_sqrt_vp_vt_dn0;
        *var_sqrt_vp_vt_dn1_slot = var_sqrt_vp_vt_dn1;
        *var_sqrt_vp_vt_dn2_slot = var_sqrt_vp_vt_dn2;
        *var_sqrt_vp_vt_dn3_slot = var_sqrt_vp_vt_dn3;
        *var_sqrt_vp_vt_rv_slot = var_sqrt_vp_vt_rv;
        *var_sqv_slot = var_sqv;
        *var_sqv_rv_slot = var_sqv_rv;
        *var_t_slot = var_t;
        *var_t0_slot = var_t0;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_rv_slot = var_t1_rv;
        *var_t_rv_slot = var_t_rv;
        *var_theta_vp_1_slot = var_theta_vp_1;
        *var_theta_vp_1_dn0_slot = var_theta_vp_1_dn0;
        *var_theta_vp_1_dn1_slot = var_theta_vp_1_dn1;
        *var_theta_vp_1_dn2_slot = var_theta_vp_1_dn2;
        *var_theta_vp_1_dn3_slot = var_theta_vp_1_dn3;
        *var_theta_vp_1_rv_slot = var_theta_vp_1_rv;
        *var_tmp1_slot = var_tmp1;
        *var_tmp1_dn0_slot = var_tmp1_dn0;
        *var_tmp1_dn1_slot = var_tmp1_dn1;
        *var_tmp1_dn2_slot = var_tmp1_dn2;
        *var_tmp1_dn3_slot = var_tmp1_dn3;
        *var_tmp1_rv_slot = var_tmp1_rv;
        *var_tmp2_slot = var_tmp2;
        *var_tmp2_dn0_slot = var_tmp2_dn0;
        *var_tmp2_dn1_slot = var_tmp2_dn1;
        *var_tmp2_dn2_slot = var_tmp2_dn2;
        *var_tmp2_dn3_slot = var_tmp2_dn3;
        *var_tmp2_rv_slot = var_tmp2_rv;
        *var_tnom_slot = var_tnom;
        *var_tnom_rv_slot = var_tnom_rv;
        *var_ucrit_t_slot = var_ucrit_t;
        *var_ucrit_t_rv_slot = var_ucrit_t_rv;
        *var_v0_slot = var_v0;
        *var_v0_rv_slot = var_v0_rv;
        *var_vc_slot = var_vc;
        *var_vc_rv_slot = var_vc_rv;
        *var_vd_slot = var_vd;
        *var_vd_dn0_slot = var_vd_dn0;
        *var_vd_dn2_slot = var_vd_dn2;
        *var_vd_dn3_slot = var_vd_dn3;
        *var_vd_rv_slot = var_vd_rv;
        *var_vg_slot = var_vg;
        *var_vg_dn1_slot = var_vg_dn1;
        *var_vg_dn3_slot = var_vg_dn3;
        *var_vg_rv_slot = var_vg_rv;
        *var_vl_slot = var_vl;
        *var_vl_rv_slot = var_vl_rv;
        *var_vpprime_slot = var_vpprime;
        *var_vpprime_dn0_slot = var_vpprime_dn0;
        *var_vpprime_dn1_slot = var_vpprime_dn1;
        *var_vpprime_dn2_slot = var_vpprime_dn2;
        *var_vpprime_dn3_slot = var_vpprime_dn3;
        *var_vpprime_rv_slot = var_vpprime_rv;
        *var_vs_slot = var_vs;
        *var_vs_dn0_slot = var_vs_dn0;
        *var_vs_dn2_slot = var_vs_dn2;
        *var_vs_dn3_slot = var_vs_dn3;
        *var_vs_rv_slot = var_vs_rv;
        *var_vt_slot = var_vt;
        *var_vt_01_slot = var_vt_01;
        *var_vt_01_rv_slot = var_vt_01_rv;
        *var_vt_2_slot = var_vt_2;
        *var_vt_2_rv_slot = var_vt_2_rv;
        *var_vt_4_slot = var_vt_4;
        *var_vt_4_rv_slot = var_vt_4_rv;
        *var_vt_rv_slot = var_vt_rv;
        *var_vt_vt_slot = var_vt_vt;
        *var_vt_vt_16_slot = var_vt_vt_16;
        *var_vt_vt_16_rv_slot = var_vt_vt_16_rv;
        *var_vt_vt_2_slot = var_vt_vt_2;
        *var_vt_vt_2_rv_slot = var_vt_vt_2_rv;
        *var_vt_vt_rv_slot = var_vt_vt_rv;
        *var_vto_s_slot = var_vto_s;
        *var_vto_s_rv_slot = var_vto_s_rv;
        *var_vto_t_slot = var_vto_t;
        *var_vto_t_rv_slot = var_vto_t_rv;
        *var_weff_slot = var_weff;
        *var_weff_rv_slot = var_weff_rv;
    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        var_deltavfb: f64,
        var_eps_cox_l: f64,
        var_eps_cox_w: f64,
        var_gamma_s: f64,
        var_gamma_sqrt_phi: f64,
        var_gamma_sqrt_phi_dn0: f64,
        var_gamma_sqrt_phi_dn1: f64,
        var_gamma_sqrt_phi_dn2: f64,
        var_gamma_sqrt_phi_dn3: f64,
        var_inv_vt: f64,
        var_leff: f64,
        var_log_vc_vt: f64,
        var_phi_t: f64,
        var_phi_t_dn0: f64,
        var_phi_t_dn1: f64,
        var_phi_t_dn2: f64,
        var_phi_t_dn3: f64,
        var_vc: f64,
        var_vd: f64,
        var_vd_dn0: f64,
        var_vd_dn2: f64,
        var_vd_dn3: f64,
        var_vg: f64,
        var_vg_dn1: f64,
        var_vg_dn3: f64,
        var_vs: f64,
        var_vs_dn0: f64,
        var_vs_dn2: f64,
        var_vs_dn3: f64,
        var_vt: f64,
        var_vt_01: f64,
        var_vt_vt_16: f64,
        var_vto_s: f64,
        var_weff: f64,
        var_big_sqrt_vp_slot: &mut f64,
        var_big_sqrt_vp0_slot: &mut f64,
        var_big_sqrt_vp0_dn0_slot: &mut f64,
        var_big_sqrt_vp0_dn1_slot: &mut f64,
        var_big_sqrt_vp0_dn2_slot: &mut f64,
        var_big_sqrt_vp0_dn3_slot: &mut f64,
        var_big_sqrt_vp0_rv_slot: &mut f64,
        var_big_sqrt_vp_dn0_slot: &mut f64,
        var_big_sqrt_vp_dn1_slot: &mut f64,
        var_big_sqrt_vp_dn2_slot: &mut f64,
        var_big_sqrt_vp_dn3_slot: &mut f64,
        var_big_sqrt_vp_rv_slot: &mut f64,
        var_deltav_2_slot: &mut f64,
        var_deltav_2_dn0_slot: &mut f64,
        var_deltav_2_dn1_slot: &mut f64,
        var_deltav_2_dn2_slot: &mut f64,
        var_deltav_2_dn3_slot: &mut f64,
        var_deltav_2_rv_slot: &mut f64,
        var_dif_dv_slot: &mut f64,
        var_dif_dv_dn0_slot: &mut f64,
        var_dif_dv_dn1_slot: &mut f64,
        var_dif_dv_dn2_slot: &mut f64,
        var_dif_dv_dn3_slot: &mut f64,
        var_dif_dv_rv_slot: &mut f64,
        var_gammaprime_slot: &mut f64,
        var_gammaprime_dn0_slot: &mut f64,
        var_gammaprime_dn1_slot: &mut f64,
        var_gammaprime_dn2_slot: &mut f64,
        var_gammaprime_dn3_slot: &mut f64,
        var_gammaprime_rv_slot: &mut f64,
        var_gammastar_slot: &mut f64,
        var_gammastar_dn0_slot: &mut f64,
        var_gammastar_dn1_slot: &mut f64,
        var_gammastar_dn2_slot: &mut f64,
        var_gammastar_dn3_slot: &mut f64,
        var_gammastar_rv_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard7_rv_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard8_rv_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_guard9_rv_slot: &mut f64,
        var_if__slot: &mut f64,
        var_if__dn0_slot: &mut f64,
        var_if__dn1_slot: &mut f64,
        var_if__dn2_slot: &mut f64,
        var_if__dn3_slot: &mut f64,
        var_if__rv_slot: &mut f64,
        var_leta_l_slot: &mut f64,
        var_leta_l_rv_slot: &mut f64,
        var_phi_vd_slot: &mut f64,
        var_phi_vd_dn0_slot: &mut f64,
        var_phi_vd_dn1_slot: &mut f64,
        var_phi_vd_dn2_slot: &mut f64,
        var_phi_vd_dn3_slot: &mut f64,
        var_phi_vd_rv_slot: &mut f64,
        var_phi_vs_slot: &mut f64,
        var_phi_vs_dn0_slot: &mut f64,
        var_phi_vs_dn1_slot: &mut f64,
        var_phi_vs_dn2_slot: &mut f64,
        var_phi_vs_dn3_slot: &mut f64,
        var_phi_vs_rv_slot: &mut f64,
        var_sqrt_gammastar_slot: &mut f64,
        var_sqrt_gammastar_dn0_slot: &mut f64,
        var_sqrt_gammastar_dn1_slot: &mut f64,
        var_sqrt_gammastar_dn2_slot: &mut f64,
        var_sqrt_gammastar_dn3_slot: &mut f64,
        var_sqrt_gammastar_rv_slot: &mut f64,
        var_sqrt_if_slot: &mut f64,
        var_sqrt_if_dn0_slot: &mut f64,
        var_sqrt_if_dn1_slot: &mut f64,
        var_sqrt_if_dn2_slot: &mut f64,
        var_sqrt_if_dn3_slot: &mut f64,
        var_sqrt_if_rv_slot: &mut f64,
        var_sqrt_phi_vd_slot: &mut f64,
        var_sqrt_phi_vd_dn0_slot: &mut f64,
        var_sqrt_phi_vd_dn1_slot: &mut f64,
        var_sqrt_phi_vd_dn2_slot: &mut f64,
        var_sqrt_phi_vd_dn3_slot: &mut f64,
        var_sqrt_phi_vd_rv_slot: &mut f64,
        var_sqrt_phi_vd_vt_slot: &mut f64,
        var_sqrt_phi_vd_vt_dn0_slot: &mut f64,
        var_sqrt_phi_vd_vt_dn1_slot: &mut f64,
        var_sqrt_phi_vd_vt_dn2_slot: &mut f64,
        var_sqrt_phi_vd_vt_dn3_slot: &mut f64,
        var_sqrt_phi_vd_vt_rv_slot: &mut f64,
        var_sqrt_phi_vp0_slot: &mut f64,
        var_sqrt_phi_vp0_dn0_slot: &mut f64,
        var_sqrt_phi_vp0_dn1_slot: &mut f64,
        var_sqrt_phi_vp0_dn2_slot: &mut f64,
        var_sqrt_phi_vp0_dn3_slot: &mut f64,
        var_sqrt_phi_vp0_rv_slot: &mut f64,
        var_sqrt_phi_vs_slot: &mut f64,
        var_sqrt_phi_vs_dn0_slot: &mut f64,
        var_sqrt_phi_vs_dn1_slot: &mut f64,
        var_sqrt_phi_vs_dn2_slot: &mut f64,
        var_sqrt_phi_vs_dn3_slot: &mut f64,
        var_sqrt_phi_vs_rv_slot: &mut f64,
        var_sqrt_phi_vs_vt_slot: &mut f64,
        var_sqrt_phi_vs_vt_dn0_slot: &mut f64,
        var_sqrt_phi_vs_vt_dn1_slot: &mut f64,
        var_sqrt_phi_vs_vt_dn2_slot: &mut f64,
        var_sqrt_phi_vs_vt_dn3_slot: &mut f64,
        var_sqrt_phi_vs_vt_rv_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_dn0_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_dn1_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_dn2_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_dn3_slot: &mut f64,
        var_sqrt_vds_vdss_deltav_rv_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_dn0_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_dn1_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_dn2_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_dn3_slot: &mut f64,
        var_sqrt_vds_vdssprime_deltav_rv_slot: &mut f64,
        var_sqrt_vdss_deltav_slot: &mut f64,
        var_sqrt_vdss_deltav_dn0_slot: &mut f64,
        var_sqrt_vdss_deltav_dn1_slot: &mut f64,
        var_sqrt_vdss_deltav_dn2_slot: &mut f64,
        var_sqrt_vdss_deltav_dn3_slot: &mut f64,
        var_sqrt_vdss_deltav_rv_slot: &mut f64,
        var_sqrt_vdssprime_deltav_slot: &mut f64,
        var_sqrt_vdssprime_deltav_dn0_slot: &mut f64,
        var_sqrt_vdssprime_deltav_dn1_slot: &mut f64,
        var_sqrt_vdssprime_deltav_dn2_slot: &mut f64,
        var_sqrt_vdssprime_deltav_dn3_slot: &mut f64,
        var_sqrt_vdssprime_deltav_rv_slot: &mut f64,
        var_sqrt_vgstar_slot: &mut f64,
        var_sqrt_vgstar_dn0_slot: &mut f64,
        var_sqrt_vgstar_dn1_slot: &mut f64,
        var_sqrt_vgstar_dn2_slot: &mut f64,
        var_sqrt_vgstar_dn3_slot: &mut f64,
        var_sqrt_vgstar_rv_slot: &mut f64,
        var_tmp1_slot: &mut f64,
        var_tmp1_dn0_slot: &mut f64,
        var_tmp1_dn1_slot: &mut f64,
        var_tmp1_dn2_slot: &mut f64,
        var_tmp1_dn3_slot: &mut f64,
        var_tmp1_rv_slot: &mut f64,
        var_vds_slot: &mut f64,
        var_vds_dn0_slot: &mut f64,
        var_vds_dn2_slot: &mut f64,
        var_vds_dn3_slot: &mut f64,
        var_vds_rv_slot: &mut f64,
        var_vdsprime_slot: &mut f64,
        var_vdsprime_dn0_slot: &mut f64,
        var_vdsprime_dn1_slot: &mut f64,
        var_vdsprime_dn2_slot: &mut f64,
        var_vdsprime_dn3_slot: &mut f64,
        var_vdsprime_rv_slot: &mut f64,
        var_vdss_slot: &mut f64,
        var_vdss_dn0_slot: &mut f64,
        var_vdss_dn1_slot: &mut f64,
        var_vdss_dn2_slot: &mut f64,
        var_vdss_dn3_slot: &mut f64,
        var_vdss_rv_slot: &mut f64,
        var_vdss_sqrt_slot: &mut f64,
        var_vdss_sqrt_dn0_slot: &mut f64,
        var_vdss_sqrt_dn1_slot: &mut f64,
        var_vdss_sqrt_dn2_slot: &mut f64,
        var_vdss_sqrt_dn3_slot: &mut f64,
        var_vdss_sqrt_rv_slot: &mut f64,
        var_vdssprime_slot: &mut f64,
        var_vdssprime_dn0_slot: &mut f64,
        var_vdssprime_dn1_slot: &mut f64,
        var_vdssprime_dn2_slot: &mut f64,
        var_vdssprime_dn3_slot: &mut f64,
        var_vdssprime_rv_slot: &mut f64,
        var_vdssprime_sqrt_slot: &mut f64,
        var_vdssprime_sqrt_dn0_slot: &mut f64,
        var_vdssprime_sqrt_dn1_slot: &mut f64,
        var_vdssprime_sqrt_dn2_slot: &mut f64,
        var_vdssprime_sqrt_dn3_slot: &mut f64,
        var_vdssprime_sqrt_rv_slot: &mut f64,
        var_vgprime_slot: &mut f64,
        var_vgprime_dn0_slot: &mut f64,
        var_vgprime_dn1_slot: &mut f64,
        var_vgprime_dn2_slot: &mut f64,
        var_vgprime_dn3_slot: &mut f64,
        var_vgprime_rv_slot: &mut f64,
        var_vgstar_slot: &mut f64,
        var_vgstar_dn0_slot: &mut f64,
        var_vgstar_dn1_slot: &mut f64,
        var_vgstar_dn2_slot: &mut f64,
        var_vgstar_dn3_slot: &mut f64,
        var_vgstar_rv_slot: &mut f64,
        var_vip_slot: &mut f64,
        var_vip_dn0_slot: &mut f64,
        var_vip_dn1_slot: &mut f64,
        var_vip_dn2_slot: &mut f64,
        var_vip_dn3_slot: &mut f64,
        var_vip_rv_slot: &mut f64,
        var_vp_slot: &mut f64,
        var_vp0_slot: &mut f64,
        var_vp0_dn0_slot: &mut f64,
        var_vp0_dn1_slot: &mut f64,
        var_vp0_dn2_slot: &mut f64,
        var_vp0_dn3_slot: &mut f64,
        var_vp0_rv_slot: &mut f64,
        var_vp_dn0_slot: &mut f64,
        var_vp_dn1_slot: &mut f64,
        var_vp_dn2_slot: &mut f64,
        var_vp_dn3_slot: &mut f64,
        var_vp_rv_slot: &mut f64,
        var_vt_vc_slot: &mut f64,
        var_vt_vc_rv_slot: &mut f64,
        var_weta_w_slot: &mut f64,
        var_weta_w_rv_slot: &mut f64,
        var_yk_slot: &mut f64,
        var_yk_dn0_slot: &mut f64,
        var_yk_dn1_slot: &mut f64,
        var_yk_dn2_slot: &mut f64,
        var_yk_dn3_slot: &mut f64,
        var_yk_rv_slot: &mut f64,
        var_z0_slot: &mut f64,
        var_z0_dn0_slot: &mut f64,
        var_z0_dn1_slot: &mut f64,
        var_z0_dn2_slot: &mut f64,
        var_z0_dn3_slot: &mut f64,
        var_z0_rv_slot: &mut f64,
        var_zk_slot: &mut f64,
        var_zk_dn0_slot: &mut f64,
        var_zk_dn1_slot: &mut f64,
        var_zk_dn2_slot: &mut f64,
        var_zk_dn3_slot: &mut f64,
        var_zk_rv_slot: &mut f64,
    ) {
        let mut var_big_sqrt_vp: f64 = *var_big_sqrt_vp_slot;
        let mut var_big_sqrt_vp0: f64 = *var_big_sqrt_vp0_slot;
        let mut var_big_sqrt_vp0_dn0: f64 = *var_big_sqrt_vp0_dn0_slot;
        let mut var_big_sqrt_vp0_dn1: f64 = *var_big_sqrt_vp0_dn1_slot;
        let mut var_big_sqrt_vp0_dn2: f64 = *var_big_sqrt_vp0_dn2_slot;
        let mut var_big_sqrt_vp0_dn3: f64 = *var_big_sqrt_vp0_dn3_slot;
        let mut var_big_sqrt_vp0_rv: f64 = *var_big_sqrt_vp0_rv_slot;
        let mut var_big_sqrt_vp_dn0: f64 = *var_big_sqrt_vp_dn0_slot;
        let mut var_big_sqrt_vp_dn1: f64 = *var_big_sqrt_vp_dn1_slot;
        let mut var_big_sqrt_vp_dn2: f64 = *var_big_sqrt_vp_dn2_slot;
        let mut var_big_sqrt_vp_dn3: f64 = *var_big_sqrt_vp_dn3_slot;
        let mut var_big_sqrt_vp_rv: f64 = *var_big_sqrt_vp_rv_slot;
        let mut var_deltav_2: f64 = *var_deltav_2_slot;
        let mut var_deltav_2_dn0: f64 = *var_deltav_2_dn0_slot;
        let mut var_deltav_2_dn1: f64 = *var_deltav_2_dn1_slot;
        let mut var_deltav_2_dn2: f64 = *var_deltav_2_dn2_slot;
        let mut var_deltav_2_dn3: f64 = *var_deltav_2_dn3_slot;
        let mut var_deltav_2_rv: f64 = *var_deltav_2_rv_slot;
        let mut var_dif_dv: f64 = *var_dif_dv_slot;
        let mut var_dif_dv_dn0: f64 = *var_dif_dv_dn0_slot;
        let mut var_dif_dv_dn1: f64 = *var_dif_dv_dn1_slot;
        let mut var_dif_dv_dn2: f64 = *var_dif_dv_dn2_slot;
        let mut var_dif_dv_dn3: f64 = *var_dif_dv_dn3_slot;
        let mut var_dif_dv_rv: f64 = *var_dif_dv_rv_slot;
        let mut var_gammaprime: f64 = *var_gammaprime_slot;
        let mut var_gammaprime_dn0: f64 = *var_gammaprime_dn0_slot;
        let mut var_gammaprime_dn1: f64 = *var_gammaprime_dn1_slot;
        let mut var_gammaprime_dn2: f64 = *var_gammaprime_dn2_slot;
        let mut var_gammaprime_dn3: f64 = *var_gammaprime_dn3_slot;
        let mut var_gammaprime_rv: f64 = *var_gammaprime_rv_slot;
        let mut var_gammastar: f64 = *var_gammastar_slot;
        let mut var_gammastar_dn0: f64 = *var_gammastar_dn0_slot;
        let mut var_gammastar_dn1: f64 = *var_gammastar_dn1_slot;
        let mut var_gammastar_dn2: f64 = *var_gammastar_dn2_slot;
        let mut var_gammastar_dn3: f64 = *var_gammastar_dn3_slot;
        let mut var_gammastar_rv: f64 = *var_gammastar_rv_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard7_rv: f64 = *var_guard7_rv_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard8_rv: f64 = *var_guard8_rv_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_guard9_rv: f64 = *var_guard9_rv_slot;
        let mut var_if_: f64 = *var_if__slot;
        let mut var_if__dn0: f64 = *var_if__dn0_slot;
        let mut var_if__dn1: f64 = *var_if__dn1_slot;
        let mut var_if__dn2: f64 = *var_if__dn2_slot;
        let mut var_if__dn3: f64 = *var_if__dn3_slot;
        let mut var_if__rv: f64 = *var_if__rv_slot;
        let mut var_leta_l: f64 = *var_leta_l_slot;
        let mut var_leta_l_rv: f64 = *var_leta_l_rv_slot;
        let mut var_phi_vd: f64 = *var_phi_vd_slot;
        let mut var_phi_vd_dn0: f64 = *var_phi_vd_dn0_slot;
        let mut var_phi_vd_dn1: f64 = *var_phi_vd_dn1_slot;
        let mut var_phi_vd_dn2: f64 = *var_phi_vd_dn2_slot;
        let mut var_phi_vd_dn3: f64 = *var_phi_vd_dn3_slot;
        let mut var_phi_vd_rv: f64 = *var_phi_vd_rv_slot;
        let mut var_phi_vs: f64 = *var_phi_vs_slot;
        let mut var_phi_vs_dn0: f64 = *var_phi_vs_dn0_slot;
        let mut var_phi_vs_dn1: f64 = *var_phi_vs_dn1_slot;
        let mut var_phi_vs_dn2: f64 = *var_phi_vs_dn2_slot;
        let mut var_phi_vs_dn3: f64 = *var_phi_vs_dn3_slot;
        let mut var_phi_vs_rv: f64 = *var_phi_vs_rv_slot;
        let mut var_sqrt_gammastar: f64 = *var_sqrt_gammastar_slot;
        let mut var_sqrt_gammastar_dn0: f64 = *var_sqrt_gammastar_dn0_slot;
        let mut var_sqrt_gammastar_dn1: f64 = *var_sqrt_gammastar_dn1_slot;
        let mut var_sqrt_gammastar_dn2: f64 = *var_sqrt_gammastar_dn2_slot;
        let mut var_sqrt_gammastar_dn3: f64 = *var_sqrt_gammastar_dn3_slot;
        let mut var_sqrt_gammastar_rv: f64 = *var_sqrt_gammastar_rv_slot;
        let mut var_sqrt_if: f64 = *var_sqrt_if_slot;
        let mut var_sqrt_if_dn0: f64 = *var_sqrt_if_dn0_slot;
        let mut var_sqrt_if_dn1: f64 = *var_sqrt_if_dn1_slot;
        let mut var_sqrt_if_dn2: f64 = *var_sqrt_if_dn2_slot;
        let mut var_sqrt_if_dn3: f64 = *var_sqrt_if_dn3_slot;
        let mut var_sqrt_if_rv: f64 = *var_sqrt_if_rv_slot;
        let mut var_sqrt_phi_vd: f64 = *var_sqrt_phi_vd_slot;
        let mut var_sqrt_phi_vd_dn0: f64 = *var_sqrt_phi_vd_dn0_slot;
        let mut var_sqrt_phi_vd_dn1: f64 = *var_sqrt_phi_vd_dn1_slot;
        let mut var_sqrt_phi_vd_dn2: f64 = *var_sqrt_phi_vd_dn2_slot;
        let mut var_sqrt_phi_vd_dn3: f64 = *var_sqrt_phi_vd_dn3_slot;
        let mut var_sqrt_phi_vd_rv: f64 = *var_sqrt_phi_vd_rv_slot;
        let mut var_sqrt_phi_vd_vt: f64 = *var_sqrt_phi_vd_vt_slot;
        let mut var_sqrt_phi_vd_vt_dn0: f64 = *var_sqrt_phi_vd_vt_dn0_slot;
        let mut var_sqrt_phi_vd_vt_dn1: f64 = *var_sqrt_phi_vd_vt_dn1_slot;
        let mut var_sqrt_phi_vd_vt_dn2: f64 = *var_sqrt_phi_vd_vt_dn2_slot;
        let mut var_sqrt_phi_vd_vt_dn3: f64 = *var_sqrt_phi_vd_vt_dn3_slot;
        let mut var_sqrt_phi_vd_vt_rv: f64 = *var_sqrt_phi_vd_vt_rv_slot;
        let mut var_sqrt_phi_vp0: f64 = *var_sqrt_phi_vp0_slot;
        let mut var_sqrt_phi_vp0_dn0: f64 = *var_sqrt_phi_vp0_dn0_slot;
        let mut var_sqrt_phi_vp0_dn1: f64 = *var_sqrt_phi_vp0_dn1_slot;
        let mut var_sqrt_phi_vp0_dn2: f64 = *var_sqrt_phi_vp0_dn2_slot;
        let mut var_sqrt_phi_vp0_dn3: f64 = *var_sqrt_phi_vp0_dn3_slot;
        let mut var_sqrt_phi_vp0_rv: f64 = *var_sqrt_phi_vp0_rv_slot;
        let mut var_sqrt_phi_vs: f64 = *var_sqrt_phi_vs_slot;
        let mut var_sqrt_phi_vs_dn0: f64 = *var_sqrt_phi_vs_dn0_slot;
        let mut var_sqrt_phi_vs_dn1: f64 = *var_sqrt_phi_vs_dn1_slot;
        let mut var_sqrt_phi_vs_dn2: f64 = *var_sqrt_phi_vs_dn2_slot;
        let mut var_sqrt_phi_vs_dn3: f64 = *var_sqrt_phi_vs_dn3_slot;
        let mut var_sqrt_phi_vs_rv: f64 = *var_sqrt_phi_vs_rv_slot;
        let mut var_sqrt_phi_vs_vt: f64 = *var_sqrt_phi_vs_vt_slot;
        let mut var_sqrt_phi_vs_vt_dn0: f64 = *var_sqrt_phi_vs_vt_dn0_slot;
        let mut var_sqrt_phi_vs_vt_dn1: f64 = *var_sqrt_phi_vs_vt_dn1_slot;
        let mut var_sqrt_phi_vs_vt_dn2: f64 = *var_sqrt_phi_vs_vt_dn2_slot;
        let mut var_sqrt_phi_vs_vt_dn3: f64 = *var_sqrt_phi_vs_vt_dn3_slot;
        let mut var_sqrt_phi_vs_vt_rv: f64 = *var_sqrt_phi_vs_vt_rv_slot;
        let mut var_sqrt_vds_vdss_deltav: f64 = *var_sqrt_vds_vdss_deltav_slot;
        let mut var_sqrt_vds_vdss_deltav_dn0: f64 = *var_sqrt_vds_vdss_deltav_dn0_slot;
        let mut var_sqrt_vds_vdss_deltav_dn1: f64 = *var_sqrt_vds_vdss_deltav_dn1_slot;
        let mut var_sqrt_vds_vdss_deltav_dn2: f64 = *var_sqrt_vds_vdss_deltav_dn2_slot;
        let mut var_sqrt_vds_vdss_deltav_dn3: f64 = *var_sqrt_vds_vdss_deltav_dn3_slot;
        let mut var_sqrt_vds_vdss_deltav_rv: f64 = *var_sqrt_vds_vdss_deltav_rv_slot;
        let mut var_sqrt_vds_vdssprime_deltav: f64 = *var_sqrt_vds_vdssprime_deltav_slot;
        let mut var_sqrt_vds_vdssprime_deltav_dn0: f64 = *var_sqrt_vds_vdssprime_deltav_dn0_slot;
        let mut var_sqrt_vds_vdssprime_deltav_dn1: f64 = *var_sqrt_vds_vdssprime_deltav_dn1_slot;
        let mut var_sqrt_vds_vdssprime_deltav_dn2: f64 = *var_sqrt_vds_vdssprime_deltav_dn2_slot;
        let mut var_sqrt_vds_vdssprime_deltav_dn3: f64 = *var_sqrt_vds_vdssprime_deltav_dn3_slot;
        let mut var_sqrt_vds_vdssprime_deltav_rv: f64 = *var_sqrt_vds_vdssprime_deltav_rv_slot;
        let mut var_sqrt_vdss_deltav: f64 = *var_sqrt_vdss_deltav_slot;
        let mut var_sqrt_vdss_deltav_dn0: f64 = *var_sqrt_vdss_deltav_dn0_slot;
        let mut var_sqrt_vdss_deltav_dn1: f64 = *var_sqrt_vdss_deltav_dn1_slot;
        let mut var_sqrt_vdss_deltav_dn2: f64 = *var_sqrt_vdss_deltav_dn2_slot;
        let mut var_sqrt_vdss_deltav_dn3: f64 = *var_sqrt_vdss_deltav_dn3_slot;
        let mut var_sqrt_vdss_deltav_rv: f64 = *var_sqrt_vdss_deltav_rv_slot;
        let mut var_sqrt_vdssprime_deltav: f64 = *var_sqrt_vdssprime_deltav_slot;
        let mut var_sqrt_vdssprime_deltav_dn0: f64 = *var_sqrt_vdssprime_deltav_dn0_slot;
        let mut var_sqrt_vdssprime_deltav_dn1: f64 = *var_sqrt_vdssprime_deltav_dn1_slot;
        let mut var_sqrt_vdssprime_deltav_dn2: f64 = *var_sqrt_vdssprime_deltav_dn2_slot;
        let mut var_sqrt_vdssprime_deltav_dn3: f64 = *var_sqrt_vdssprime_deltav_dn3_slot;
        let mut var_sqrt_vdssprime_deltav_rv: f64 = *var_sqrt_vdssprime_deltav_rv_slot;
        let mut var_sqrt_vgstar: f64 = *var_sqrt_vgstar_slot;
        let mut var_sqrt_vgstar_dn0: f64 = *var_sqrt_vgstar_dn0_slot;
        let mut var_sqrt_vgstar_dn1: f64 = *var_sqrt_vgstar_dn1_slot;
        let mut var_sqrt_vgstar_dn2: f64 = *var_sqrt_vgstar_dn2_slot;
        let mut var_sqrt_vgstar_dn3: f64 = *var_sqrt_vgstar_dn3_slot;
        let mut var_sqrt_vgstar_rv: f64 = *var_sqrt_vgstar_rv_slot;
        let mut var_tmp1: f64 = *var_tmp1_slot;
        let mut var_tmp1_dn0: f64 = *var_tmp1_dn0_slot;
        let mut var_tmp1_dn1: f64 = *var_tmp1_dn1_slot;
        let mut var_tmp1_dn2: f64 = *var_tmp1_dn2_slot;
        let mut var_tmp1_dn3: f64 = *var_tmp1_dn3_slot;
        let mut var_tmp1_rv: f64 = *var_tmp1_rv_slot;
        let mut var_vds: f64 = *var_vds_slot;
        let mut var_vds_dn0: f64 = *var_vds_dn0_slot;
        let mut var_vds_dn2: f64 = *var_vds_dn2_slot;
        let mut var_vds_dn3: f64 = *var_vds_dn3_slot;
        let mut var_vds_rv: f64 = *var_vds_rv_slot;
        let mut var_vdsprime: f64 = *var_vdsprime_slot;
        let mut var_vdsprime_dn0: f64 = *var_vdsprime_dn0_slot;
        let mut var_vdsprime_dn1: f64 = *var_vdsprime_dn1_slot;
        let mut var_vdsprime_dn2: f64 = *var_vdsprime_dn2_slot;
        let mut var_vdsprime_dn3: f64 = *var_vdsprime_dn3_slot;
        let mut var_vdsprime_rv: f64 = *var_vdsprime_rv_slot;
        let mut var_vdss: f64 = *var_vdss_slot;
        let mut var_vdss_dn0: f64 = *var_vdss_dn0_slot;
        let mut var_vdss_dn1: f64 = *var_vdss_dn1_slot;
        let mut var_vdss_dn2: f64 = *var_vdss_dn2_slot;
        let mut var_vdss_dn3: f64 = *var_vdss_dn3_slot;
        let mut var_vdss_rv: f64 = *var_vdss_rv_slot;
        let mut var_vdss_sqrt: f64 = *var_vdss_sqrt_slot;
        let mut var_vdss_sqrt_dn0: f64 = *var_vdss_sqrt_dn0_slot;
        let mut var_vdss_sqrt_dn1: f64 = *var_vdss_sqrt_dn1_slot;
        let mut var_vdss_sqrt_dn2: f64 = *var_vdss_sqrt_dn2_slot;
        let mut var_vdss_sqrt_dn3: f64 = *var_vdss_sqrt_dn3_slot;
        let mut var_vdss_sqrt_rv: f64 = *var_vdss_sqrt_rv_slot;
        let mut var_vdssprime: f64 = *var_vdssprime_slot;
        let mut var_vdssprime_dn0: f64 = *var_vdssprime_dn0_slot;
        let mut var_vdssprime_dn1: f64 = *var_vdssprime_dn1_slot;
        let mut var_vdssprime_dn2: f64 = *var_vdssprime_dn2_slot;
        let mut var_vdssprime_dn3: f64 = *var_vdssprime_dn3_slot;
        let mut var_vdssprime_rv: f64 = *var_vdssprime_rv_slot;
        let mut var_vdssprime_sqrt: f64 = *var_vdssprime_sqrt_slot;
        let mut var_vdssprime_sqrt_dn0: f64 = *var_vdssprime_sqrt_dn0_slot;
        let mut var_vdssprime_sqrt_dn1: f64 = *var_vdssprime_sqrt_dn1_slot;
        let mut var_vdssprime_sqrt_dn2: f64 = *var_vdssprime_sqrt_dn2_slot;
        let mut var_vdssprime_sqrt_dn3: f64 = *var_vdssprime_sqrt_dn3_slot;
        let mut var_vdssprime_sqrt_rv: f64 = *var_vdssprime_sqrt_rv_slot;
        let mut var_vgprime: f64 = *var_vgprime_slot;
        let mut var_vgprime_dn0: f64 = *var_vgprime_dn0_slot;
        let mut var_vgprime_dn1: f64 = *var_vgprime_dn1_slot;
        let mut var_vgprime_dn2: f64 = *var_vgprime_dn2_slot;
        let mut var_vgprime_dn3: f64 = *var_vgprime_dn3_slot;
        let mut var_vgprime_rv: f64 = *var_vgprime_rv_slot;
        let mut var_vgstar: f64 = *var_vgstar_slot;
        let mut var_vgstar_dn0: f64 = *var_vgstar_dn0_slot;
        let mut var_vgstar_dn1: f64 = *var_vgstar_dn1_slot;
        let mut var_vgstar_dn2: f64 = *var_vgstar_dn2_slot;
        let mut var_vgstar_dn3: f64 = *var_vgstar_dn3_slot;
        let mut var_vgstar_rv: f64 = *var_vgstar_rv_slot;
        let mut var_vip: f64 = *var_vip_slot;
        let mut var_vip_dn0: f64 = *var_vip_dn0_slot;
        let mut var_vip_dn1: f64 = *var_vip_dn1_slot;
        let mut var_vip_dn2: f64 = *var_vip_dn2_slot;
        let mut var_vip_dn3: f64 = *var_vip_dn3_slot;
        let mut var_vip_rv: f64 = *var_vip_rv_slot;
        let mut var_vp: f64 = *var_vp_slot;
        let mut var_vp0: f64 = *var_vp0_slot;
        let mut var_vp0_dn0: f64 = *var_vp0_dn0_slot;
        let mut var_vp0_dn1: f64 = *var_vp0_dn1_slot;
        let mut var_vp0_dn2: f64 = *var_vp0_dn2_slot;
        let mut var_vp0_dn3: f64 = *var_vp0_dn3_slot;
        let mut var_vp0_rv: f64 = *var_vp0_rv_slot;
        let mut var_vp_dn0: f64 = *var_vp_dn0_slot;
        let mut var_vp_dn1: f64 = *var_vp_dn1_slot;
        let mut var_vp_dn2: f64 = *var_vp_dn2_slot;
        let mut var_vp_dn3: f64 = *var_vp_dn3_slot;
        let mut var_vp_rv: f64 = *var_vp_rv_slot;
        let mut var_vt_vc: f64 = *var_vt_vc_slot;
        let mut var_vt_vc_rv: f64 = *var_vt_vc_rv_slot;
        let mut var_weta_w: f64 = *var_weta_w_slot;
        let mut var_weta_w_rv: f64 = *var_weta_w_rv_slot;
        let mut var_yk: f64 = *var_yk_slot;
        let mut var_yk_dn0: f64 = *var_yk_dn0_slot;
        let mut var_yk_dn1: f64 = *var_yk_dn1_slot;
        let mut var_yk_dn2: f64 = *var_yk_dn2_slot;
        let mut var_yk_dn3: f64 = *var_yk_dn3_slot;
        let mut var_yk_rv: f64 = *var_yk_rv_slot;
        let mut var_z0: f64 = *var_z0_slot;
        let mut var_z0_dn0: f64 = *var_z0_dn0_slot;
        let mut var_z0_dn1: f64 = *var_z0_dn1_slot;
        let mut var_z0_dn2: f64 = *var_z0_dn2_slot;
        let mut var_z0_dn3: f64 = *var_z0_dn3_slot;
        let mut var_z0_rv: f64 = *var_z0_rv_slot;
        let mut var_zk: f64 = *var_zk_slot;
        let mut var_zk_dn0: f64 = *var_zk_dn0_slot;
        let mut var_zk_dn1: f64 = *var_zk_dn1_slot;
        let mut var_zk_dn2: f64 = *var_zk_dn2_slot;
        let mut var_zk_dn3: f64 = *var_zk_dn3_slot;
        let mut var_zk_rv: f64 = *var_zk_rv_slot;

        let assign710_e569: f64 = (var_vg - var_vto_s);
        let assign710_e571: f64 = (assign710_e569 - var_deltavfb);
        let assign710_e573: f64 = (assign710_e571 + var_phi_t);
        let assign710_e575: f64 = (assign710_e573 + var_gamma_sqrt_phi);
        var_vgstar = assign710_e575;
        var_vgstar_dn0 = (var_phi_t_dn0 + var_gamma_sqrt_phi_dn0);
        var_vgstar_dn1 = ((var_vg_dn1 + var_phi_t_dn1) + var_gamma_sqrt_phi_dn1);
        var_vgstar_dn2 = (var_phi_t_dn2 + var_gamma_sqrt_phi_dn2);
        var_vgstar_dn3 = ((var_vg_dn3 + var_phi_t_dn3) + var_gamma_sqrt_phi_dn3);
        var_vgstar_rv = 0.0;

        let assign720_e578: f64 = (var_vgstar * var_vgstar);
        let assign720_e581: f64 = (2.0 * var_vt_vt_16);
        let assign720_e582: f64 = (assign720_e578 + assign720_e581);
        let assign720_e583: f64 = (assign720_e582).sqrt();
        var_sqrt_vgstar = assign720_e583;
        var_sqrt_vgstar_dn0 = (((var_vgstar_dn0 * var_vgstar) + (var_vgstar * var_vgstar_dn0)) / (2.0 * assign720_e583));
        var_sqrt_vgstar_dn1 = (((var_vgstar_dn1 * var_vgstar) + (var_vgstar * var_vgstar_dn1)) / (2.0 * assign720_e583));
        var_sqrt_vgstar_dn2 = (((var_vgstar_dn2 * var_vgstar) + (var_vgstar * var_vgstar_dn2)) / (2.0 * assign720_e583));
        var_sqrt_vgstar_dn3 = (((var_vgstar_dn3 * var_vgstar) + (var_vgstar * var_vgstar_dn3)) / (2.0 * assign720_e583));
        var_sqrt_vgstar_rv = 0.0;

        let assign730_e587: f64 = (var_vgstar + var_sqrt_vgstar);
        let assign730_e588: f64 = (0.5 * assign730_e587);
        var_vgprime = assign730_e588;
        var_vgprime_dn0 = (0.5 * (var_vgstar_dn0 + var_sqrt_vgstar_dn0));
        var_vgprime_dn1 = (0.5 * (var_vgstar_dn1 + var_sqrt_vgstar_dn1));
        var_vgprime_dn2 = (0.5 * (var_vgstar_dn2 + var_sqrt_vgstar_dn2));
        var_vgprime_dn3 = (0.5 * (var_vgstar_dn3 + var_sqrt_vgstar_dn3));
        var_vgprime_rv = 0.0;

        let assign740_e591: f64 = (var_phi_t + var_vs);
        var_phi_vs = assign740_e591;
        var_phi_vs_dn0 = (var_phi_t_dn0 + var_vs_dn0);
        var_phi_vs_dn1 = var_phi_t_dn1;
        var_phi_vs_dn2 = (var_phi_t_dn2 + var_vs_dn2);
        var_phi_vs_dn3 = (var_phi_t_dn3 + var_vs_dn3);
        var_phi_vs_rv = 0.0;

        let assign750_e594: f64 = (var_phi_vs * var_phi_vs);
        let assign750_e596: f64 = (assign750_e594 + var_vt_vt_16);
        let assign750_e597: f64 = (assign750_e596).sqrt();
        var_sqrt_phi_vs_vt = assign750_e597;
        var_sqrt_phi_vs_vt_dn0 = (((var_phi_vs_dn0 * var_phi_vs) + (var_phi_vs * var_phi_vs_dn0)) / (2.0 * assign750_e597));
        var_sqrt_phi_vs_vt_dn1 = (((var_phi_vs_dn1 * var_phi_vs) + (var_phi_vs * var_phi_vs_dn1)) / (2.0 * assign750_e597));
        var_sqrt_phi_vs_vt_dn2 = (((var_phi_vs_dn2 * var_phi_vs) + (var_phi_vs * var_phi_vs_dn2)) / (2.0 * assign750_e597));
        var_sqrt_phi_vs_vt_dn3 = (((var_phi_vs_dn3 * var_phi_vs) + (var_phi_vs * var_phi_vs_dn3)) / (2.0 * assign750_e597));
        var_sqrt_phi_vs_vt_rv = 0.0;

        let assign760_e601: f64 = (var_phi_vs + var_sqrt_phi_vs_vt);
        let assign760_e602: f64 = (0.5 * assign760_e601);
        let assign760_e603: f64 = (assign760_e602).sqrt();
        var_sqrt_phi_vs = assign760_e603;
        var_sqrt_phi_vs_dn0 = ((0.5 * (var_phi_vs_dn0 + var_sqrt_phi_vs_vt_dn0)) / (2.0 * assign760_e603));
        var_sqrt_phi_vs_dn1 = ((0.5 * (var_phi_vs_dn1 + var_sqrt_phi_vs_vt_dn1)) / (2.0 * assign760_e603));
        var_sqrt_phi_vs_dn2 = ((0.5 * (var_phi_vs_dn2 + var_sqrt_phi_vs_vt_dn2)) / (2.0 * assign760_e603));
        var_sqrt_phi_vs_dn3 = ((0.5 * (var_phi_vs_dn3 + var_sqrt_phi_vs_vt_dn3)) / (2.0 * assign760_e603));
        var_sqrt_phi_vs_rv = 0.0;

        let assign770_e606: f64 = (var_phi_t + var_vd);
        var_phi_vd = assign770_e606;
        var_phi_vd_dn0 = (var_phi_t_dn0 + var_vd_dn0);
        var_phi_vd_dn1 = var_phi_t_dn1;
        var_phi_vd_dn2 = (var_phi_t_dn2 + var_vd_dn2);
        var_phi_vd_dn3 = (var_phi_t_dn3 + var_vd_dn3);
        var_phi_vd_rv = 0.0;

        let assign780_e609: f64 = (var_phi_vd * var_phi_vd);
        let assign780_e611: f64 = (assign780_e609 + var_vt_vt_16);
        let assign780_e612: f64 = (assign780_e611).sqrt();
        var_sqrt_phi_vd_vt = assign780_e612;
        var_sqrt_phi_vd_vt_dn0 = (((var_phi_vd_dn0 * var_phi_vd) + (var_phi_vd * var_phi_vd_dn0)) / (2.0 * assign780_e612));
        var_sqrt_phi_vd_vt_dn1 = (((var_phi_vd_dn1 * var_phi_vd) + (var_phi_vd * var_phi_vd_dn1)) / (2.0 * assign780_e612));
        var_sqrt_phi_vd_vt_dn2 = (((var_phi_vd_dn2 * var_phi_vd) + (var_phi_vd * var_phi_vd_dn2)) / (2.0 * assign780_e612));
        var_sqrt_phi_vd_vt_dn3 = (((var_phi_vd_dn3 * var_phi_vd) + (var_phi_vd * var_phi_vd_dn3)) / (2.0 * assign780_e612));
        var_sqrt_phi_vd_vt_rv = 0.0;

        let assign790_e616: f64 = (var_phi_vd + var_sqrt_phi_vd_vt);
        let assign790_e617: f64 = (0.5 * assign790_e616);
        let assign790_e618: f64 = (assign790_e617).sqrt();
        var_sqrt_phi_vd = assign790_e618;
        var_sqrt_phi_vd_dn0 = ((0.5 * (var_phi_vd_dn0 + var_sqrt_phi_vd_vt_dn0)) / (2.0 * assign790_e618));
        var_sqrt_phi_vd_dn1 = ((0.5 * (var_phi_vd_dn1 + var_sqrt_phi_vd_vt_dn1)) / (2.0 * assign790_e618));
        var_sqrt_phi_vd_dn2 = ((0.5 * (var_phi_vd_dn2 + var_sqrt_phi_vd_vt_dn2)) / (2.0 * assign790_e618));
        var_sqrt_phi_vd_dn3 = ((0.5 * (var_phi_vd_dn3 + var_sqrt_phi_vd_vt_dn3)) / (2.0 * assign790_e618));
        var_sqrt_phi_vd_rv = 0.0;

        let assign800_e621: f64 = (var_eps_cox_w * p.p7);
        let assign800_e623: f64 = (assign800_e621 / var_weff);
        var_weta_w = assign800_e623;
        var_weta_w_rv = 0.0;

        let assign810_e626: f64 = (var_eps_cox_l * p.p8);
        let assign810_e628: f64 = (assign810_e626 / var_leff);
        var_leta_l = assign810_e628;
        var_leta_l_rv = 0.0;

        let assign820_e632: f64 = (0.25 * var_gamma_s);
        let assign820_e634: f64 = (assign820_e632 * var_gamma_s);
        let assign820_e635: f64 = (var_vgprime + assign820_e634);
        let assign820_e636: f64 = (assign820_e635).sqrt();
        var_big_sqrt_vp0 = assign820_e636;
        var_big_sqrt_vp0_dn0 = (var_vgprime_dn0 / (2.0 * assign820_e636));
        var_big_sqrt_vp0_dn1 = (var_vgprime_dn1 / (2.0 * assign820_e636));
        var_big_sqrt_vp0_dn2 = (var_vgprime_dn2 / (2.0 * assign820_e636));
        var_big_sqrt_vp0_dn3 = (var_vgprime_dn3 / (2.0 * assign820_e636));
        var_big_sqrt_vp0_rv = 0.0;

        let assign830_e639: f64 = (var_vgprime - var_phi_t);
        let assign830_e644: f64 = (0.5 * var_gamma_s);
        let assign830_e645: f64 = (var_big_sqrt_vp0 - assign830_e644);
        let assign830_e646: f64 = (var_gamma_s * assign830_e645);
        let assign830_e647: f64 = (assign830_e639 - assign830_e646);
        var_vp0 = assign830_e647;
        var_vp0_dn0 = ((var_vgprime_dn0 - var_phi_t_dn0) - (var_gamma_s * var_big_sqrt_vp0_dn0));
        var_vp0_dn1 = ((var_vgprime_dn1 - var_phi_t_dn1) - (var_gamma_s * var_big_sqrt_vp0_dn1));
        var_vp0_dn2 = ((var_vgprime_dn2 - var_phi_t_dn2) - (var_gamma_s * var_big_sqrt_vp0_dn2));
        var_vp0_dn3 = ((var_vgprime_dn3 - var_phi_t_dn3) - (var_gamma_s * var_big_sqrt_vp0_dn3));
        var_vp0_rv = 0.0;

        let assign840_e650: f64 = (var_vp0 + var_phi_t);
        let assign840_e652: f64 = (assign840_e650 + var_vt_01);
        let assign840_e653: f64 = (assign840_e652).sqrt();
        var_sqrt_phi_vp0 = assign840_e653;
        var_sqrt_phi_vp0_dn0 = ((var_vp0_dn0 + var_phi_t_dn0) / (2.0 * assign840_e653));
        var_sqrt_phi_vp0_dn1 = ((var_vp0_dn1 + var_phi_t_dn1) / (2.0 * assign840_e653));
        var_sqrt_phi_vp0_dn2 = ((var_vp0_dn2 + var_phi_t_dn2) / (2.0 * assign840_e653));
        var_sqrt_phi_vp0_dn3 = ((var_vp0_dn3 + var_phi_t_dn3) / (2.0 * assign840_e653));
        var_sqrt_phi_vp0_rv = 0.0;

        let assign850_e658: f64 = (var_sqrt_phi_vs + var_sqrt_phi_vd);
        let assign850_e659: f64 = (var_leta_l * assign850_e658);
        let assign850_e660: f64 = (var_gamma_s - assign850_e659);
        let assign850_e663: f64 = (var_weta_w * var_sqrt_phi_vp0);
        let assign850_e664: f64 = (assign850_e660 + assign850_e663);
        var_gammastar = assign850_e664;
        var_gammastar_dn0 = ((-(var_leta_l * (var_sqrt_phi_vs_dn0 + var_sqrt_phi_vd_dn0))) + (var_weta_w * var_sqrt_phi_vp0_dn0));
        var_gammastar_dn1 = ((-(var_leta_l * (var_sqrt_phi_vs_dn1 + var_sqrt_phi_vd_dn1))) + (var_weta_w * var_sqrt_phi_vp0_dn1));
        var_gammastar_dn2 = ((-(var_leta_l * (var_sqrt_phi_vs_dn2 + var_sqrt_phi_vd_dn2))) + (var_weta_w * var_sqrt_phi_vp0_dn2));
        var_gammastar_dn3 = ((-(var_leta_l * (var_sqrt_phi_vs_dn3 + var_sqrt_phi_vd_dn3))) + (var_weta_w * var_sqrt_phi_vp0_dn3));
        var_gammastar_rv = 0.0;

        let assign860_e667: f64 = (var_gammastar * var_gammastar);
        let assign860_e669: f64 = (assign860_e667 + var_vt_01);
        let assign860_e670: f64 = (assign860_e669).sqrt();
        var_sqrt_gammastar = assign860_e670;
        var_sqrt_gammastar_dn0 = (((var_gammastar_dn0 * var_gammastar) + (var_gammastar * var_gammastar_dn0)) / (2.0 * assign860_e670));
        var_sqrt_gammastar_dn1 = (((var_gammastar_dn1 * var_gammastar) + (var_gammastar * var_gammastar_dn1)) / (2.0 * assign860_e670));
        var_sqrt_gammastar_dn2 = (((var_gammastar_dn2 * var_gammastar) + (var_gammastar * var_gammastar_dn2)) / (2.0 * assign860_e670));
        var_sqrt_gammastar_dn3 = (((var_gammastar_dn3 * var_gammastar) + (var_gammastar * var_gammastar_dn3)) / (2.0 * assign860_e670));
        var_sqrt_gammastar_rv = 0.0;

        let assign870_e674: f64 = (var_gammastar + var_sqrt_gammastar);
        let assign870_e675: f64 = (0.5 * assign870_e674);
        var_gammaprime = assign870_e675;
        var_gammaprime_dn0 = (0.5 * (var_gammastar_dn0 + var_sqrt_gammastar_dn0));
        var_gammaprime_dn1 = (0.5 * (var_gammastar_dn1 + var_sqrt_gammastar_dn1));
        var_gammaprime_dn2 = (0.5 * (var_gammastar_dn2 + var_sqrt_gammastar_dn2));
        var_gammaprime_dn3 = (0.5 * (var_gammastar_dn3 + var_sqrt_gammastar_dn3));
        var_gammaprime_rv = 0.0;

        let assign880_e679: f64 = (0.25 * var_gammaprime);
        let assign880_e681: f64 = (assign880_e679 * var_gammaprime);
        let assign880_e682: f64 = (var_vgprime + assign880_e681);
        let assign880_e683: f64 = (assign880_e682).sqrt();
        var_big_sqrt_vp = assign880_e683;
        var_big_sqrt_vp_dn0 = ((var_vgprime_dn0 + (((0.25 * var_gammaprime_dn0) * var_gammaprime) + (assign880_e679 * var_gammaprime_dn0))) / (2.0 * assign880_e683));
        var_big_sqrt_vp_dn1 = ((var_vgprime_dn1 + (((0.25 * var_gammaprime_dn1) * var_gammaprime) + (assign880_e679 * var_gammaprime_dn1))) / (2.0 * assign880_e683));
        var_big_sqrt_vp_dn2 = ((var_vgprime_dn2 + (((0.25 * var_gammaprime_dn2) * var_gammaprime) + (assign880_e679 * var_gammaprime_dn2))) / (2.0 * assign880_e683));
        var_big_sqrt_vp_dn3 = ((var_vgprime_dn3 + (((0.25 * var_gammaprime_dn3) * var_gammaprime) + (assign880_e679 * var_gammaprime_dn3))) / (2.0 * assign880_e683));
        var_big_sqrt_vp_rv = 0.0;

        let assign890_e686: f64 = (var_vgprime - var_phi_t);
        let assign890_e691: f64 = (0.5 * var_gammaprime);
        let assign890_e692: f64 = (var_big_sqrt_vp - assign890_e691);
        let assign890_e693: f64 = (var_gammaprime * assign890_e692);
        let assign890_e694: f64 = (assign890_e686 - assign890_e693);
        var_vp = assign890_e694;
        var_vp_dn0 = ((var_vgprime_dn0 - var_phi_t_dn0) - ((var_gammaprime_dn0 * assign890_e692) + (var_gammaprime * (var_big_sqrt_vp_dn0 - (0.5 * var_gammaprime_dn0)))));
        var_vp_dn1 = ((var_vgprime_dn1 - var_phi_t_dn1) - ((var_gammaprime_dn1 * assign890_e692) + (var_gammaprime * (var_big_sqrt_vp_dn1 - (0.5 * var_gammaprime_dn1)))));
        var_vp_dn2 = ((var_vgprime_dn2 - var_phi_t_dn2) - ((var_gammaprime_dn2 * assign890_e692) + (var_gammaprime * (var_big_sqrt_vp_dn2 - (0.5 * var_gammaprime_dn2)))));
        var_vp_dn3 = ((var_vgprime_dn3 - var_phi_t_dn3) - ((var_gammaprime_dn3 * assign890_e692) + (var_gammaprime * (var_big_sqrt_vp_dn3 - (0.5 * var_gammaprime_dn3)))));
        var_vp_rv = 0.0;

        let assign900_e697: f64 = (var_vp - var_vs);
        let assign900_e699: f64 = (assign900_e697 * var_inv_vt);
        var_tmp1 = assign900_e699;
        var_tmp1_dn0 = ((var_vp_dn0 - var_vs_dn0) * var_inv_vt);
        var_tmp1_dn1 = (var_vp_dn1 * var_inv_vt);
        var_tmp1_dn2 = ((var_vp_dn2 - var_vs_dn2) * var_inv_vt);
        var_tmp1_dn3 = ((var_vp_dn3 - var_vs_dn3) * var_inv_vt);
        var_tmp1_rv = 0.0;

        let assign910_e702: f64 = (-0.35);
        let assign910_e703: f64 = if var_tmp1 > assign910_e702 { 1.0 } else { 0.0 };
        var_guard7 = assign910_e703;
        var_guard7_rv = 0.0;

        let (assign920_e716, assign920_e716_d_n0, assign920_e716_d_n1, assign920_e716_d_n2, assign920_e716_d_n3,) = {
    if (var_guard7 != 0.0) {
        let assign920_e708: f64 = (1.3 + var_tmp1);
        let assign920_e711: f64 = (var_tmp1 + 1.6);
        let assign920_e712: f64 = (assign920_e711).ln();
        let assign920_e713: f64 = (assign920_e708 - assign920_e712);
        let assign920_e714: f64 = (2.0 / assign920_e713);
        (assign920_e714, (-((2.0 * (var_tmp1_dn0 - (var_tmp1_dn0 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (var_tmp1_dn1 - (var_tmp1_dn1 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (var_tmp1_dn2 - (var_tmp1_dn2 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (var_tmp1_dn3 - (var_tmp1_dn3 / assign920_e711))) / (assign920_e713 * assign920_e713))),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign920_e716;
        var_z0_dn0 = assign920_e716_d_n0;
        var_z0_dn1 = assign920_e716_d_n1;
        var_z0_dn2 = assign920_e716_d_n2;
        var_z0_dn3 = assign920_e716_d_n3;
        var_z0_rv = 0.0;

        let (assign930_e729, assign930_e729_d_n0, assign930_e729_d_n1, assign930_e729_d_n2, assign930_e729_d_n3,) = {
    if (var_guard7 != 0.0) {
        let assign930_e720: f64 = (2.0 + var_z0);
        let assign930_e723: f64 = (1.0 + var_tmp1);
        let assign930_e725: f64 = (var_z0).ln();
        let assign930_e726: f64 = (assign930_e723 + assign930_e725);
        let assign930_e727: f64 = (assign930_e720 / assign930_e726);
        (assign930_e727, (((var_z0_dn0 * assign930_e726) - (assign930_e720 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign930_e726 * assign930_e726)), (((var_z0_dn1 * assign930_e726) - (assign930_e720 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign930_e726 * assign930_e726)), (((var_z0_dn2 * assign930_e726) - (assign930_e720 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign930_e726 * assign930_e726)), (((var_z0_dn3 * assign930_e726) - (assign930_e720 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign930_e726 * assign930_e726)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign930_e729;
        var_zk_dn0 = assign930_e729_d_n0;
        var_zk_dn1 = assign930_e729_d_n1;
        var_zk_dn2 = assign930_e729_d_n2;
        var_zk_dn3 = assign930_e729_d_n3;
        var_zk_rv = 0.0;

        let (assign940_e742, assign940_e742_d_n0, assign940_e742_d_n1, assign940_e742_d_n2, assign940_e742_d_n3,) = {
    if (var_guard7 != 0.0) {
        let assign940_e733: f64 = (1.0 + var_tmp1);
        let assign940_e735: f64 = (var_zk).ln();
        let assign940_e736: f64 = (assign940_e733 + assign940_e735);
        let assign940_e739: f64 = (2.0 + var_zk);
        let assign940_e740: f64 = (assign940_e736 / assign940_e739);
        (assign940_e740, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign940_e739) - (assign940_e736 * var_zk_dn0)) / (assign940_e739 * assign940_e739)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign940_e739) - (assign940_e736 * var_zk_dn1)) / (assign940_e739 * assign940_e739)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign940_e739) - (assign940_e736 * var_zk_dn2)) / (assign940_e739 * assign940_e739)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign940_e739) - (assign940_e736 * var_zk_dn3)) / (assign940_e739 * assign940_e739)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign940_e742;
        var_yk_dn0 = assign940_e742_d_n0;
        var_yk_dn1 = assign940_e742_d_n1;
        var_yk_dn2 = assign940_e742_d_n2;
        var_yk_dn3 = assign940_e742_d_n3;
        var_yk_rv = 0.0;

        let assign950_e745: f64 = (-15.0);
        let assign950_e746: f64 = if var_tmp1 > assign950_e745 { 1.0 } else { 0.0 };
        var_guard8 = assign950_e746;
        var_guard8_rv = 0.0;

        let (assign960_e757, assign960_e757_d_n0, assign960_e757_d_n1, assign960_e757_d_n2, assign960_e757_d_n3,) = {
    if ((var_guard7 == 0.0) && (var_guard8 != 0.0)) {
        let assign960_e753: f64 = (-var_tmp1);
        let assign960_e754: f64 = (assign960_e753).exp();
        let assign960_e755: f64 = (1.55 + assign960_e754);
        (assign960_e755, (assign960_e754 * (-var_tmp1_dn0)), (assign960_e754 * (-var_tmp1_dn1)), (assign960_e754 * (-var_tmp1_dn2)), (assign960_e754 * (-var_tmp1_dn3)),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign960_e757;
        var_z0_dn0 = assign960_e757_d_n0;
        var_z0_dn1 = assign960_e757_d_n1;
        var_z0_dn2 = assign960_e757_d_n2;
        var_z0_dn3 = assign960_e757_d_n3;
        var_z0_rv = 0.0;

        let (assign970_e773, assign970_e773_d_n0, assign970_e773_d_n1, assign970_e773_d_n2, assign970_e773_d_n3,) = {
    if ((var_guard7 == 0.0) && (var_guard8 != 0.0)) {
        let assign970_e764: f64 = (2.0 + var_z0);
        let assign970_e767: f64 = (1.0 + var_tmp1);
        let assign970_e769: f64 = (var_z0).ln();
        let assign970_e770: f64 = (assign970_e767 + assign970_e769);
        let assign970_e771: f64 = (assign970_e764 / assign970_e770);
        (assign970_e771, (((var_z0_dn0 * assign970_e770) - (assign970_e764 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign970_e770 * assign970_e770)), (((var_z0_dn1 * assign970_e770) - (assign970_e764 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign970_e770 * assign970_e770)), (((var_z0_dn2 * assign970_e770) - (assign970_e764 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign970_e770 * assign970_e770)), (((var_z0_dn3 * assign970_e770) - (assign970_e764 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign970_e770 * assign970_e770)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign970_e773;
        var_zk_dn0 = assign970_e773_d_n0;
        var_zk_dn1 = assign970_e773_d_n1;
        var_zk_dn2 = assign970_e773_d_n2;
        var_zk_dn3 = assign970_e773_d_n3;
        var_zk_rv = 0.0;

        let (assign980_e789, assign980_e789_d_n0, assign980_e789_d_n1, assign980_e789_d_n2, assign980_e789_d_n3,) = {
    if ((var_guard7 == 0.0) && (var_guard8 != 0.0)) {
        let assign980_e780: f64 = (1.0 + var_tmp1);
        let assign980_e782: f64 = (var_zk).ln();
        let assign980_e783: f64 = (assign980_e780 + assign980_e782);
        let assign980_e786: f64 = (2.0 + var_zk);
        let assign980_e787: f64 = (assign980_e783 / assign980_e786);
        (assign980_e787, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign980_e786) - (assign980_e783 * var_zk_dn0)) / (assign980_e786 * assign980_e786)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign980_e786) - (assign980_e783 * var_zk_dn1)) / (assign980_e786 * assign980_e786)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign980_e786) - (assign980_e783 * var_zk_dn2)) / (assign980_e786 * assign980_e786)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign980_e786) - (assign980_e783 * var_zk_dn3)) / (assign980_e786 * assign980_e786)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign980_e789;
        var_yk_dn0 = assign980_e789_d_n0;
        var_yk_dn1 = assign980_e789_d_n1;
        var_yk_dn2 = assign980_e789_d_n2;
        var_yk_dn3 = assign980_e789_d_n3;
        var_yk_rv = 0.0;

        let assign990_e792: f64 = (-23.0);
        let assign990_e793: f64 = if var_tmp1 > assign990_e792 { 1.0 } else { 0.0 };
        var_guard9 = assign990_e793;
        var_guard9_rv = 0.0;

        let (assign1000_e809, assign1000_e809_d_n0, assign1000_e809_d_n1, assign1000_e809_d_n2, assign1000_e809_d_n3,) = {
    if (((var_guard7 == 0.0) && (var_guard8 == 0.0)) && (var_guard9 != 0.0)) {
        let assign1000_e804: f64 = (-var_tmp1);
        let assign1000_e805: f64 = (assign1000_e804).exp();
        let assign1000_e806: f64 = (2.0 + assign1000_e805);
        let assign1000_e807: f64 = (1.0 / assign1000_e806);
        (assign1000_e807, (-((assign1000_e805 * (-var_tmp1_dn0)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-var_tmp1_dn1)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-var_tmp1_dn2)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-var_tmp1_dn3)) / (assign1000_e806 * assign1000_e806))),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1000_e809;
        var_yk_dn0 = assign1000_e809_d_n0;
        var_yk_dn1 = assign1000_e809_d_n1;
        var_yk_dn2 = assign1000_e809_d_n2;
        var_yk_dn3 = assign1000_e809_d_n3;
        var_yk_rv = 0.0;

        let (assign1010_e823, assign1010_e823_d_n0, assign1010_e823_d_n1, assign1010_e823_d_n2, assign1010_e823_d_n3,) = {
    if (((var_guard7 == 0.0) && (var_guard8 == 0.0)) && (var_guard9 == 0.0)) {
        let assign1010_e819: f64 = (var_tmp1).exp();
        let assign1010_e821: f64 = (assign1010_e819 + 1e-64);
        (assign1010_e821, (assign1010_e819 * var_tmp1_dn0), (assign1010_e819 * var_tmp1_dn1), (assign1010_e819 * var_tmp1_dn2), (assign1010_e819 * var_tmp1_dn3),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1010_e823;
        var_yk_dn0 = assign1010_e823_d_n0;
        var_yk_dn1 = assign1010_e823_d_n1;
        var_yk_dn2 = assign1010_e823_d_n2;
        var_yk_dn3 = assign1010_e823_d_n3;
        var_yk_rv = 0.0;

        let assign1020_e827: f64 = (1.0 + var_yk);
        let assign1020_e828: f64 = (var_yk * assign1020_e827);
        var_if_ = assign1020_e828;
        var_if__dn0 = ((var_yk_dn0 * assign1020_e827) + (var_yk * var_yk_dn0));
        var_if__dn1 = ((var_yk_dn1 * assign1020_e827) + (var_yk * var_yk_dn1));
        var_if__dn2 = ((var_yk_dn2 * assign1020_e827) + (var_yk * var_yk_dn2));
        var_if__dn3 = ((var_yk_dn3 * assign1020_e827) + (var_yk * var_yk_dn3));
        var_if__rv = 0.0;

        let assign1030_e830: f64 = (var_if_).sqrt();
        var_sqrt_if = assign1030_e830;
        var_sqrt_if_dn0 = (var_if__dn0 / (2.0 * assign1030_e830));
        var_sqrt_if_dn1 = (var_if__dn1 / (2.0 * assign1030_e830));
        var_sqrt_if_dn2 = (var_if__dn2 / (2.0 * assign1030_e830));
        var_sqrt_if_dn3 = (var_if__dn3 / (2.0 * assign1030_e830));
        var_sqrt_if_rv = 0.0;

        var_dif_dv = var_yk;
        var_dif_dv_dn0 = var_yk_dn0;
        var_dif_dv_dn1 = var_yk_dn1;
        var_dif_dv_dn2 = var_yk_dn2;
        var_dif_dv_dn3 = var_yk_dn3;
        var_dif_dv_rv = 0.0;

        let assign1050_e834: f64 = (var_vt / var_vc);
        var_vt_vc = assign1050_e834;
        var_vt_vc_rv = 0.0;

        let assign1060_e838: f64 = (var_sqrt_if * var_vt_vc);
        let assign1060_e839: f64 = (0.25 + assign1060_e838);
        let assign1060_e840: f64 = (assign1060_e839).sqrt();
        var_vdss_sqrt = assign1060_e840;
        var_vdss_sqrt_dn0 = ((var_sqrt_if_dn0 * var_vt_vc) / (2.0 * assign1060_e840));
        var_vdss_sqrt_dn1 = ((var_sqrt_if_dn1 * var_vt_vc) / (2.0 * assign1060_e840));
        var_vdss_sqrt_dn2 = ((var_sqrt_if_dn2 * var_vt_vc) / (2.0 * assign1060_e840));
        var_vdss_sqrt_dn3 = ((var_sqrt_if_dn3 * var_vt_vc) / (2.0 * assign1060_e840));
        var_vdss_sqrt_rv = 0.0;

        let assign1070_e844: f64 = (var_vdss_sqrt - 0.5);
        let assign1070_e845: f64 = (var_vc * assign1070_e844);
        var_vdss = assign1070_e845;
        var_vdss_dn0 = (var_vc * var_vdss_sqrt_dn0);
        var_vdss_dn1 = (var_vc * var_vdss_sqrt_dn1);
        var_vdss_dn2 = (var_vc * var_vdss_sqrt_dn2);
        var_vdss_dn3 = (var_vc * var_vdss_sqrt_dn3);
        var_vdss_rv = 0.0;

        let assign1080_e849: f64 = (var_vd - var_vs);
        let assign1080_e850: f64 = (0.5 * assign1080_e849);
        var_vds = assign1080_e850;
        var_vds_dn0 = (0.5 * (var_vd_dn0 - var_vs_dn0));
        var_vds_dn2 = (0.5 * (var_vd_dn2 - var_vs_dn2));
        var_vds_dn3 = (0.5 * (var_vd_dn3 - var_vs_dn3));
        var_vds_rv = 0.0;

        let assign1090_e856: f64 = (var_vdss * var_inv_vt);
        let assign1090_e857: f64 = (var_sqrt_if - assign1090_e856);
        let assign1090_e858: f64 = (p.p25 * assign1090_e857);
        let assign1090_e860: f64 = (assign1090_e858 + 0.015625);
        let assign1090_e861: f64 = (var_vt_vt_16 * assign1090_e860);
        var_deltav_2 = assign1090_e861;
        var_deltav_2_dn0 = (var_vt_vt_16 * (p.p25 * (var_sqrt_if_dn0 - (var_vdss_dn0 * var_inv_vt))));
        var_deltav_2_dn1 = (var_vt_vt_16 * (p.p25 * (var_sqrt_if_dn1 - (var_vdss_dn1 * var_inv_vt))));
        var_deltav_2_dn2 = (var_vt_vt_16 * (p.p25 * (var_sqrt_if_dn2 - (var_vdss_dn2 * var_inv_vt))));
        var_deltav_2_dn3 = (var_vt_vt_16 * (p.p25 * (var_sqrt_if_dn3 - (var_vdss_dn3 * var_inv_vt))));
        var_deltav_2_rv = 0.0;

        let assign1100_e864: f64 = (var_vdss * var_vdss);
        let assign1100_e866: f64 = (assign1100_e864 + var_deltav_2);
        let assign1100_e867: f64 = (assign1100_e866).sqrt();
        var_sqrt_vdss_deltav = assign1100_e867;
        var_sqrt_vdss_deltav_dn0 = ((((var_vdss_dn0 * var_vdss) + (var_vdss * var_vdss_dn0)) + var_deltav_2_dn0) / (2.0 * assign1100_e867));
        var_sqrt_vdss_deltav_dn1 = ((((var_vdss_dn1 * var_vdss) + (var_vdss * var_vdss_dn1)) + var_deltav_2_dn1) / (2.0 * assign1100_e867));
        var_sqrt_vdss_deltav_dn2 = ((((var_vdss_dn2 * var_vdss) + (var_vdss * var_vdss_dn2)) + var_deltav_2_dn2) / (2.0 * assign1100_e867));
        var_sqrt_vdss_deltav_dn3 = ((((var_vdss_dn3 * var_vdss) + (var_vdss * var_vdss_dn3)) + var_deltav_2_dn3) / (2.0 * assign1100_e867));
        var_sqrt_vdss_deltav_rv = 0.0;

        let assign1110_e870: f64 = (var_vds - var_vdss);
        let assign1110_e873: f64 = (var_vds - var_vdss);
        let assign1110_e874: f64 = (assign1110_e870 * assign1110_e873);
        let assign1110_e876: f64 = (assign1110_e874 + var_deltav_2);
        let assign1110_e877: f64 = (assign1110_e876).sqrt();
        var_sqrt_vds_vdss_deltav = assign1110_e877;
        var_sqrt_vds_vdss_deltav_dn0 = (((((var_vds_dn0 - var_vdss_dn0) * assign1110_e873) + (assign1110_e870 * (var_vds_dn0 - var_vdss_dn0))) + var_deltav_2_dn0) / (2.0 * assign1110_e877));
        var_sqrt_vds_vdss_deltav_dn1 = (((((-var_vdss_dn1) * assign1110_e873) + (assign1110_e870 * (-var_vdss_dn1))) + var_deltav_2_dn1) / (2.0 * assign1110_e877));
        var_sqrt_vds_vdss_deltav_dn2 = (((((var_vds_dn2 - var_vdss_dn2) * assign1110_e873) + (assign1110_e870 * (var_vds_dn2 - var_vdss_dn2))) + var_deltav_2_dn2) / (2.0 * assign1110_e877));
        var_sqrt_vds_vdss_deltav_dn3 = (((((var_vds_dn3 - var_vdss_dn3) * assign1110_e873) + (assign1110_e870 * (var_vds_dn3 - var_vdss_dn3))) + var_deltav_2_dn3) / (2.0 * assign1110_e877));
        var_sqrt_vds_vdss_deltav_rv = 0.0;

        let assign1120_e880: f64 = (var_sqrt_vdss_deltav - var_sqrt_vds_vdss_deltav);
        var_vip = assign1120_e880;
        var_vip_dn0 = (var_sqrt_vdss_deltav_dn0 - var_sqrt_vds_vdss_deltav_dn0);
        var_vip_dn1 = (var_sqrt_vdss_deltav_dn1 - var_sqrt_vds_vdss_deltav_dn1);
        var_vip_dn2 = (var_sqrt_vdss_deltav_dn2 - var_sqrt_vds_vdss_deltav_dn2);
        var_vip_dn3 = (var_sqrt_vdss_deltav_dn3 - var_sqrt_vds_vdss_deltav_dn3);
        var_vip_rv = 0.0;

        let assign1130_e885: f64 = (var_if_).ln();
        let assign1130_e886: f64 = (0.75 * assign1130_e885);
        let assign1130_e887: f64 = (var_sqrt_if - assign1130_e886);
        let assign1130_e889: f64 = (assign1130_e887 * var_vt_vc);
        let assign1130_e890: f64 = (0.25 + assign1130_e889);
        let assign1130_e891: f64 = (assign1130_e890).sqrt();
        var_vdssprime_sqrt = assign1130_e891;
        var_vdssprime_sqrt_dn0 = (((var_sqrt_if_dn0 - (0.75 * (var_if__dn0 / var_if_))) * var_vt_vc) / (2.0 * assign1130_e891));
        var_vdssprime_sqrt_dn1 = (((var_sqrt_if_dn1 - (0.75 * (var_if__dn1 / var_if_))) * var_vt_vc) / (2.0 * assign1130_e891));
        var_vdssprime_sqrt_dn2 = (((var_sqrt_if_dn2 - (0.75 * (var_if__dn2 / var_if_))) * var_vt_vc) / (2.0 * assign1130_e891));
        var_vdssprime_sqrt_dn3 = (((var_sqrt_if_dn3 - (0.75 * (var_if__dn3 / var_if_))) * var_vt_vc) / (2.0 * assign1130_e891));
        var_vdssprime_sqrt_rv = 0.0;

        let assign1140_e895: f64 = (var_vdssprime_sqrt - 0.5);
        let assign1140_e896: f64 = (var_vc * assign1140_e895);
        let assign1140_e898: f64 = (assign1140_e896 + var_log_vc_vt);
        var_vdssprime = assign1140_e898;
        var_vdssprime_dn0 = (var_vc * var_vdssprime_sqrt_dn0);
        var_vdssprime_dn1 = (var_vc * var_vdssprime_sqrt_dn1);
        var_vdssprime_dn2 = (var_vc * var_vdssprime_sqrt_dn2);
        var_vdssprime_dn3 = (var_vc * var_vdssprime_sqrt_dn3);
        var_vdssprime_rv = 0.0;

        let assign1150_e901: f64 = (var_vds - var_vdssprime);
        var_vdsprime = assign1150_e901;
        var_vdsprime_dn0 = (var_vds_dn0 - var_vdssprime_dn0);
        var_vdsprime_dn1 = (-var_vdssprime_dn1);
        var_vdsprime_dn2 = (var_vds_dn2 - var_vdssprime_dn2);
        var_vdsprime_dn3 = (var_vds_dn3 - var_vdssprime_dn3);
        var_vdsprime_rv = 0.0;

        let assign1160_e904: f64 = (var_vdssprime * var_vdssprime);
        let assign1160_e906: f64 = (assign1160_e904 + var_deltav_2);
        let assign1160_e907: f64 = (assign1160_e906).sqrt();
        var_sqrt_vdssprime_deltav = assign1160_e907;
        var_sqrt_vdssprime_deltav_dn0 = ((((var_vdssprime_dn0 * var_vdssprime) + (var_vdssprime * var_vdssprime_dn0)) + var_deltav_2_dn0) / (2.0 * assign1160_e907));
        var_sqrt_vdssprime_deltav_dn1 = ((((var_vdssprime_dn1 * var_vdssprime) + (var_vdssprime * var_vdssprime_dn1)) + var_deltav_2_dn1) / (2.0 * assign1160_e907));
        var_sqrt_vdssprime_deltav_dn2 = ((((var_vdssprime_dn2 * var_vdssprime) + (var_vdssprime * var_vdssprime_dn2)) + var_deltav_2_dn2) / (2.0 * assign1160_e907));
        var_sqrt_vdssprime_deltav_dn3 = ((((var_vdssprime_dn3 * var_vdssprime) + (var_vdssprime * var_vdssprime_dn3)) + var_deltav_2_dn3) / (2.0 * assign1160_e907));
        var_sqrt_vdssprime_deltav_rv = 0.0;

        let assign1170_e910: f64 = (var_vdsprime * var_vdsprime);
        let assign1170_e912: f64 = (assign1170_e910 + var_deltav_2);
        let assign1170_e913: f64 = (assign1170_e912).sqrt();
        var_sqrt_vds_vdssprime_deltav = assign1170_e913;
        var_sqrt_vds_vdssprime_deltav_dn0 = ((((var_vdsprime_dn0 * var_vdsprime) + (var_vdsprime * var_vdsprime_dn0)) + var_deltav_2_dn0) / (2.0 * assign1170_e913));
        var_sqrt_vds_vdssprime_deltav_dn1 = ((((var_vdsprime_dn1 * var_vdsprime) + (var_vdsprime * var_vdsprime_dn1)) + var_deltav_2_dn1) / (2.0 * assign1170_e913));
        var_sqrt_vds_vdssprime_deltav_dn2 = ((((var_vdsprime_dn2 * var_vdsprime) + (var_vdsprime * var_vdsprime_dn2)) + var_deltav_2_dn2) / (2.0 * assign1170_e913));
        var_sqrt_vds_vdssprime_deltav_dn3 = ((((var_vdsprime_dn3 * var_vdsprime) + (var_vdsprime * var_vdsprime_dn3)) + var_deltav_2_dn3) / (2.0 * assign1170_e913));
        var_sqrt_vds_vdssprime_deltav_rv = 0.0;

        *var_big_sqrt_vp_slot = var_big_sqrt_vp;
        *var_big_sqrt_vp0_slot = var_big_sqrt_vp0;
        *var_big_sqrt_vp0_dn0_slot = var_big_sqrt_vp0_dn0;
        *var_big_sqrt_vp0_dn1_slot = var_big_sqrt_vp0_dn1;
        *var_big_sqrt_vp0_dn2_slot = var_big_sqrt_vp0_dn2;
        *var_big_sqrt_vp0_dn3_slot = var_big_sqrt_vp0_dn3;
        *var_big_sqrt_vp0_rv_slot = var_big_sqrt_vp0_rv;
        *var_big_sqrt_vp_dn0_slot = var_big_sqrt_vp_dn0;
        *var_big_sqrt_vp_dn1_slot = var_big_sqrt_vp_dn1;
        *var_big_sqrt_vp_dn2_slot = var_big_sqrt_vp_dn2;
        *var_big_sqrt_vp_dn3_slot = var_big_sqrt_vp_dn3;
        *var_big_sqrt_vp_rv_slot = var_big_sqrt_vp_rv;
        *var_deltav_2_slot = var_deltav_2;
        *var_deltav_2_dn0_slot = var_deltav_2_dn0;
        *var_deltav_2_dn1_slot = var_deltav_2_dn1;
        *var_deltav_2_dn2_slot = var_deltav_2_dn2;
        *var_deltav_2_dn3_slot = var_deltav_2_dn3;
        *var_deltav_2_rv_slot = var_deltav_2_rv;
        *var_dif_dv_slot = var_dif_dv;
        *var_dif_dv_dn0_slot = var_dif_dv_dn0;
        *var_dif_dv_dn1_slot = var_dif_dv_dn1;
        *var_dif_dv_dn2_slot = var_dif_dv_dn2;
        *var_dif_dv_dn3_slot = var_dif_dv_dn3;
        *var_dif_dv_rv_slot = var_dif_dv_rv;
        *var_gammaprime_slot = var_gammaprime;
        *var_gammaprime_dn0_slot = var_gammaprime_dn0;
        *var_gammaprime_dn1_slot = var_gammaprime_dn1;
        *var_gammaprime_dn2_slot = var_gammaprime_dn2;
        *var_gammaprime_dn3_slot = var_gammaprime_dn3;
        *var_gammaprime_rv_slot = var_gammaprime_rv;
        *var_gammastar_slot = var_gammastar;
        *var_gammastar_dn0_slot = var_gammastar_dn0;
        *var_gammastar_dn1_slot = var_gammastar_dn1;
        *var_gammastar_dn2_slot = var_gammastar_dn2;
        *var_gammastar_dn3_slot = var_gammastar_dn3;
        *var_gammastar_rv_slot = var_gammastar_rv;
        *var_guard7_slot = var_guard7;
        *var_guard7_rv_slot = var_guard7_rv;
        *var_guard8_slot = var_guard8;
        *var_guard8_rv_slot = var_guard8_rv;
        *var_guard9_slot = var_guard9;
        *var_guard9_rv_slot = var_guard9_rv;
        *var_if__slot = var_if_;
        *var_if__dn0_slot = var_if__dn0;
        *var_if__dn1_slot = var_if__dn1;
        *var_if__dn2_slot = var_if__dn2;
        *var_if__dn3_slot = var_if__dn3;
        *var_if__rv_slot = var_if__rv;
        *var_leta_l_slot = var_leta_l;
        *var_leta_l_rv_slot = var_leta_l_rv;
        *var_phi_vd_slot = var_phi_vd;
        *var_phi_vd_dn0_slot = var_phi_vd_dn0;
        *var_phi_vd_dn1_slot = var_phi_vd_dn1;
        *var_phi_vd_dn2_slot = var_phi_vd_dn2;
        *var_phi_vd_dn3_slot = var_phi_vd_dn3;
        *var_phi_vd_rv_slot = var_phi_vd_rv;
        *var_phi_vs_slot = var_phi_vs;
        *var_phi_vs_dn0_slot = var_phi_vs_dn0;
        *var_phi_vs_dn1_slot = var_phi_vs_dn1;
        *var_phi_vs_dn2_slot = var_phi_vs_dn2;
        *var_phi_vs_dn3_slot = var_phi_vs_dn3;
        *var_phi_vs_rv_slot = var_phi_vs_rv;
        *var_sqrt_gammastar_slot = var_sqrt_gammastar;
        *var_sqrt_gammastar_dn0_slot = var_sqrt_gammastar_dn0;
        *var_sqrt_gammastar_dn1_slot = var_sqrt_gammastar_dn1;
        *var_sqrt_gammastar_dn2_slot = var_sqrt_gammastar_dn2;
        *var_sqrt_gammastar_dn3_slot = var_sqrt_gammastar_dn3;
        *var_sqrt_gammastar_rv_slot = var_sqrt_gammastar_rv;
        *var_sqrt_if_slot = var_sqrt_if;
        *var_sqrt_if_dn0_slot = var_sqrt_if_dn0;
        *var_sqrt_if_dn1_slot = var_sqrt_if_dn1;
        *var_sqrt_if_dn2_slot = var_sqrt_if_dn2;
        *var_sqrt_if_dn3_slot = var_sqrt_if_dn3;
        *var_sqrt_if_rv_slot = var_sqrt_if_rv;
        *var_sqrt_phi_vd_slot = var_sqrt_phi_vd;
        *var_sqrt_phi_vd_dn0_slot = var_sqrt_phi_vd_dn0;
        *var_sqrt_phi_vd_dn1_slot = var_sqrt_phi_vd_dn1;
        *var_sqrt_phi_vd_dn2_slot = var_sqrt_phi_vd_dn2;
        *var_sqrt_phi_vd_dn3_slot = var_sqrt_phi_vd_dn3;
        *var_sqrt_phi_vd_rv_slot = var_sqrt_phi_vd_rv;
        *var_sqrt_phi_vd_vt_slot = var_sqrt_phi_vd_vt;
        *var_sqrt_phi_vd_vt_dn0_slot = var_sqrt_phi_vd_vt_dn0;
        *var_sqrt_phi_vd_vt_dn1_slot = var_sqrt_phi_vd_vt_dn1;
        *var_sqrt_phi_vd_vt_dn2_slot = var_sqrt_phi_vd_vt_dn2;
        *var_sqrt_phi_vd_vt_dn3_slot = var_sqrt_phi_vd_vt_dn3;
        *var_sqrt_phi_vd_vt_rv_slot = var_sqrt_phi_vd_vt_rv;
        *var_sqrt_phi_vp0_slot = var_sqrt_phi_vp0;
        *var_sqrt_phi_vp0_dn0_slot = var_sqrt_phi_vp0_dn0;
        *var_sqrt_phi_vp0_dn1_slot = var_sqrt_phi_vp0_dn1;
        *var_sqrt_phi_vp0_dn2_slot = var_sqrt_phi_vp0_dn2;
        *var_sqrt_phi_vp0_dn3_slot = var_sqrt_phi_vp0_dn3;
        *var_sqrt_phi_vp0_rv_slot = var_sqrt_phi_vp0_rv;
        *var_sqrt_phi_vs_slot = var_sqrt_phi_vs;
        *var_sqrt_phi_vs_dn0_slot = var_sqrt_phi_vs_dn0;
        *var_sqrt_phi_vs_dn1_slot = var_sqrt_phi_vs_dn1;
        *var_sqrt_phi_vs_dn2_slot = var_sqrt_phi_vs_dn2;
        *var_sqrt_phi_vs_dn3_slot = var_sqrt_phi_vs_dn3;
        *var_sqrt_phi_vs_rv_slot = var_sqrt_phi_vs_rv;
        *var_sqrt_phi_vs_vt_slot = var_sqrt_phi_vs_vt;
        *var_sqrt_phi_vs_vt_dn0_slot = var_sqrt_phi_vs_vt_dn0;
        *var_sqrt_phi_vs_vt_dn1_slot = var_sqrt_phi_vs_vt_dn1;
        *var_sqrt_phi_vs_vt_dn2_slot = var_sqrt_phi_vs_vt_dn2;
        *var_sqrt_phi_vs_vt_dn3_slot = var_sqrt_phi_vs_vt_dn3;
        *var_sqrt_phi_vs_vt_rv_slot = var_sqrt_phi_vs_vt_rv;
        *var_sqrt_vds_vdss_deltav_slot = var_sqrt_vds_vdss_deltav;
        *var_sqrt_vds_vdss_deltav_dn0_slot = var_sqrt_vds_vdss_deltav_dn0;
        *var_sqrt_vds_vdss_deltav_dn1_slot = var_sqrt_vds_vdss_deltav_dn1;
        *var_sqrt_vds_vdss_deltav_dn2_slot = var_sqrt_vds_vdss_deltav_dn2;
        *var_sqrt_vds_vdss_deltav_dn3_slot = var_sqrt_vds_vdss_deltav_dn3;
        *var_sqrt_vds_vdss_deltav_rv_slot = var_sqrt_vds_vdss_deltav_rv;
        *var_sqrt_vds_vdssprime_deltav_slot = var_sqrt_vds_vdssprime_deltav;
        *var_sqrt_vds_vdssprime_deltav_dn0_slot = var_sqrt_vds_vdssprime_deltav_dn0;
        *var_sqrt_vds_vdssprime_deltav_dn1_slot = var_sqrt_vds_vdssprime_deltav_dn1;
        *var_sqrt_vds_vdssprime_deltav_dn2_slot = var_sqrt_vds_vdssprime_deltav_dn2;
        *var_sqrt_vds_vdssprime_deltav_dn3_slot = var_sqrt_vds_vdssprime_deltav_dn3;
        *var_sqrt_vds_vdssprime_deltav_rv_slot = var_sqrt_vds_vdssprime_deltav_rv;
        *var_sqrt_vdss_deltav_slot = var_sqrt_vdss_deltav;
        *var_sqrt_vdss_deltav_dn0_slot = var_sqrt_vdss_deltav_dn0;
        *var_sqrt_vdss_deltav_dn1_slot = var_sqrt_vdss_deltav_dn1;
        *var_sqrt_vdss_deltav_dn2_slot = var_sqrt_vdss_deltav_dn2;
        *var_sqrt_vdss_deltav_dn3_slot = var_sqrt_vdss_deltav_dn3;
        *var_sqrt_vdss_deltav_rv_slot = var_sqrt_vdss_deltav_rv;
        *var_sqrt_vdssprime_deltav_slot = var_sqrt_vdssprime_deltav;
        *var_sqrt_vdssprime_deltav_dn0_slot = var_sqrt_vdssprime_deltav_dn0;
        *var_sqrt_vdssprime_deltav_dn1_slot = var_sqrt_vdssprime_deltav_dn1;
        *var_sqrt_vdssprime_deltav_dn2_slot = var_sqrt_vdssprime_deltav_dn2;
        *var_sqrt_vdssprime_deltav_dn3_slot = var_sqrt_vdssprime_deltav_dn3;
        *var_sqrt_vdssprime_deltav_rv_slot = var_sqrt_vdssprime_deltav_rv;
        *var_sqrt_vgstar_slot = var_sqrt_vgstar;
        *var_sqrt_vgstar_dn0_slot = var_sqrt_vgstar_dn0;
        *var_sqrt_vgstar_dn1_slot = var_sqrt_vgstar_dn1;
        *var_sqrt_vgstar_dn2_slot = var_sqrt_vgstar_dn2;
        *var_sqrt_vgstar_dn3_slot = var_sqrt_vgstar_dn3;
        *var_sqrt_vgstar_rv_slot = var_sqrt_vgstar_rv;
        *var_tmp1_slot = var_tmp1;
        *var_tmp1_dn0_slot = var_tmp1_dn0;
        *var_tmp1_dn1_slot = var_tmp1_dn1;
        *var_tmp1_dn2_slot = var_tmp1_dn2;
        *var_tmp1_dn3_slot = var_tmp1_dn3;
        *var_tmp1_rv_slot = var_tmp1_rv;
        *var_vds_slot = var_vds;
        *var_vds_dn0_slot = var_vds_dn0;
        *var_vds_dn2_slot = var_vds_dn2;
        *var_vds_dn3_slot = var_vds_dn3;
        *var_vds_rv_slot = var_vds_rv;
        *var_vdsprime_slot = var_vdsprime;
        *var_vdsprime_dn0_slot = var_vdsprime_dn0;
        *var_vdsprime_dn1_slot = var_vdsprime_dn1;
        *var_vdsprime_dn2_slot = var_vdsprime_dn2;
        *var_vdsprime_dn3_slot = var_vdsprime_dn3;
        *var_vdsprime_rv_slot = var_vdsprime_rv;
        *var_vdss_slot = var_vdss;
        *var_vdss_dn0_slot = var_vdss_dn0;
        *var_vdss_dn1_slot = var_vdss_dn1;
        *var_vdss_dn2_slot = var_vdss_dn2;
        *var_vdss_dn3_slot = var_vdss_dn3;
        *var_vdss_rv_slot = var_vdss_rv;
        *var_vdss_sqrt_slot = var_vdss_sqrt;
        *var_vdss_sqrt_dn0_slot = var_vdss_sqrt_dn0;
        *var_vdss_sqrt_dn1_slot = var_vdss_sqrt_dn1;
        *var_vdss_sqrt_dn2_slot = var_vdss_sqrt_dn2;
        *var_vdss_sqrt_dn3_slot = var_vdss_sqrt_dn3;
        *var_vdss_sqrt_rv_slot = var_vdss_sqrt_rv;
        *var_vdssprime_slot = var_vdssprime;
        *var_vdssprime_dn0_slot = var_vdssprime_dn0;
        *var_vdssprime_dn1_slot = var_vdssprime_dn1;
        *var_vdssprime_dn2_slot = var_vdssprime_dn2;
        *var_vdssprime_dn3_slot = var_vdssprime_dn3;
        *var_vdssprime_rv_slot = var_vdssprime_rv;
        *var_vdssprime_sqrt_slot = var_vdssprime_sqrt;
        *var_vdssprime_sqrt_dn0_slot = var_vdssprime_sqrt_dn0;
        *var_vdssprime_sqrt_dn1_slot = var_vdssprime_sqrt_dn1;
        *var_vdssprime_sqrt_dn2_slot = var_vdssprime_sqrt_dn2;
        *var_vdssprime_sqrt_dn3_slot = var_vdssprime_sqrt_dn3;
        *var_vdssprime_sqrt_rv_slot = var_vdssprime_sqrt_rv;
        *var_vgprime_slot = var_vgprime;
        *var_vgprime_dn0_slot = var_vgprime_dn0;
        *var_vgprime_dn1_slot = var_vgprime_dn1;
        *var_vgprime_dn2_slot = var_vgprime_dn2;
        *var_vgprime_dn3_slot = var_vgprime_dn3;
        *var_vgprime_rv_slot = var_vgprime_rv;
        *var_vgstar_slot = var_vgstar;
        *var_vgstar_dn0_slot = var_vgstar_dn0;
        *var_vgstar_dn1_slot = var_vgstar_dn1;
        *var_vgstar_dn2_slot = var_vgstar_dn2;
        *var_vgstar_dn3_slot = var_vgstar_dn3;
        *var_vgstar_rv_slot = var_vgstar_rv;
        *var_vip_slot = var_vip;
        *var_vip_dn0_slot = var_vip_dn0;
        *var_vip_dn1_slot = var_vip_dn1;
        *var_vip_dn2_slot = var_vip_dn2;
        *var_vip_dn3_slot = var_vip_dn3;
        *var_vip_rv_slot = var_vip_rv;
        *var_vp_slot = var_vp;
        *var_vp0_slot = var_vp0;
        *var_vp0_dn0_slot = var_vp0_dn0;
        *var_vp0_dn1_slot = var_vp0_dn1;
        *var_vp0_dn2_slot = var_vp0_dn2;
        *var_vp0_dn3_slot = var_vp0_dn3;
        *var_vp0_rv_slot = var_vp0_rv;
        *var_vp_dn0_slot = var_vp_dn0;
        *var_vp_dn1_slot = var_vp_dn1;
        *var_vp_dn2_slot = var_vp_dn2;
        *var_vp_dn3_slot = var_vp_dn3;
        *var_vp_rv_slot = var_vp_rv;
        *var_vt_vc_slot = var_vt_vc;
        *var_vt_vc_rv_slot = var_vt_vc_rv;
        *var_weta_w_slot = var_weta_w;
        *var_weta_w_rv_slot = var_weta_w_rv;
        *var_yk_slot = var_yk;
        *var_yk_dn0_slot = var_yk_dn0;
        *var_yk_dn1_slot = var_yk_dn1;
        *var_yk_dn2_slot = var_yk_dn2;
        *var_yk_dn3_slot = var_yk_dn3;
        *var_yk_rv_slot = var_yk_rv;
        *var_z0_slot = var_z0;
        *var_z0_dn0_slot = var_z0_dn0;
        *var_z0_dn1_slot = var_z0_dn1;
        *var_z0_dn2_slot = var_z0_dn2;
        *var_z0_dn3_slot = var_z0_dn3;
        *var_z0_rv_slot = var_z0_rv;
        *var_zk_slot = var_zk;
        *var_zk_dn0_slot = var_zk_dn0;
        *var_zk_dn1_slot = var_zk_dn1;
        *var_zk_dn2_slot = var_zk_dn2;
        *var_zk_dn3_slot = var_zk_dn3;
        *var_zk_rv_slot = var_zk_rv;
    }

    pub(super) fn stamp_reactive_block_2(
        var_gamma_s: f64,
        var_if_: f64,
        var_if__dn0: f64,
        var_if__dn1: f64,
        var_if__dn2: f64,
        var_if__dn3: f64,
        var_inv_ucrit: f64,
        var_inv_vt: f64,
        var_lc_lambda: f64,
        var_lc_ucrit: f64,
        var_leff: f64,
        var_phi_t: f64,
        var_phi_t_dn0: f64,
        var_phi_t_dn1: f64,
        var_phi_t_dn2: f64,
        var_phi_t_dn3: f64,
        var_sqrt_vds_vdssprime_deltav: f64,
        var_sqrt_vds_vdssprime_deltav_dn0: f64,
        var_sqrt_vds_vdssprime_deltav_dn1: f64,
        var_sqrt_vds_vdssprime_deltav_dn2: f64,
        var_sqrt_vds_vdssprime_deltav_dn3: f64,
        var_sqrt_vdssprime_deltav: f64,
        var_sqrt_vdssprime_deltav_dn0: f64,
        var_sqrt_vdssprime_deltav_dn1: f64,
        var_sqrt_vdssprime_deltav_dn2: f64,
        var_sqrt_vdssprime_deltav_dn3: f64,
        var_vd: f64,
        var_vd_dn0: f64,
        var_vd_dn2: f64,
        var_vd_dn3: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn2: f64,
        var_vds_dn3: f64,
        var_vip: f64,
        var_vip_dn0: f64,
        var_vip_dn1: f64,
        var_vip_dn2: f64,
        var_vip_dn3: f64,
        var_vp: f64,
        var_vp_dn0: f64,
        var_vp_dn1: f64,
        var_vp_dn2: f64,
        var_vp_dn3: f64,
        var_vs: f64,
        var_vs_dn0: f64,
        var_vs_dn2: f64,
        var_vs_dn3: f64,
        var_deltal_slot: &mut f64,
        var_deltal_dn0_slot: &mut f64,
        var_deltal_dn1_slot: &mut f64,
        var_deltal_dn2_slot: &mut f64,
        var_deltal_dn3_slot: &mut f64,
        var_deltal_rv_slot: &mut f64,
        var_dir_dv_slot: &mut f64,
        var_dir_dv_dn0_slot: &mut f64,
        var_dir_dv_dn1_slot: &mut f64,
        var_dir_dv_dn2_slot: &mut f64,
        var_dir_dv_dn3_slot: &mut f64,
        var_dir_dv_rv_slot: &mut f64,
        var_dirprime_dv_slot: &mut f64,
        var_dirprime_dv_dn0_slot: &mut f64,
        var_dirprime_dv_dn1_slot: &mut f64,
        var_dirprime_dv_dn2_slot: &mut f64,
        var_dirprime_dv_dn3_slot: &mut f64,
        var_dirprime_dv_rv_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard10_rv_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard11_rv_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard12_rv_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard13_rv_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard14_rv_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard15_rv_slot: &mut f64,
        var_ir_slot: &mut f64,
        var_ir_dn0_slot: &mut f64,
        var_ir_dn1_slot: &mut f64,
        var_ir_dn2_slot: &mut f64,
        var_ir_dn3_slot: &mut f64,
        var_ir_rv_slot: &mut f64,
        var_irprime_slot: &mut f64,
        var_irprime_dn0_slot: &mut f64,
        var_irprime_dn1_slot: &mut f64,
        var_irprime_dn2_slot: &mut f64,
        var_irprime_dn3_slot: &mut f64,
        var_irprime_rv_slot: &mut f64,
        var_leq_slot: &mut f64,
        var_leq_dn0_slot: &mut f64,
        var_leq_dn1_slot: &mut f64,
        var_leq_dn2_slot: &mut f64,
        var_leq_dn3_slot: &mut f64,
        var_leq_rv_slot: &mut f64,
        var_lmin_slot: &mut f64,
        var_lmin_rv_slot: &mut f64,
        var_lprime_slot: &mut f64,
        var_lprime_dn0_slot: &mut f64,
        var_lprime_dn1_slot: &mut f64,
        var_lprime_dn2_slot: &mut f64,
        var_lprime_dn3_slot: &mut f64,
        var_lprime_rv_slot: &mut f64,
        var_n_1_slot: &mut f64,
        var_n_1_dn0_slot: &mut f64,
        var_n_1_dn1_slot: &mut f64,
        var_n_1_dn2_slot: &mut f64,
        var_n_1_dn3_slot: &mut f64,
        var_n_1_n_slot: &mut f64,
        var_n_1_n_dn0_slot: &mut f64,
        var_n_1_n_dn1_slot: &mut f64,
        var_n_1_n_dn2_slot: &mut f64,
        var_n_1_n_dn3_slot: &mut f64,
        var_n_1_n_rv_slot: &mut f64,
        var_n_1_rv_slot: &mut f64,
        var_sif_slot: &mut f64,
        var_sif2_slot: &mut f64,
        var_sif2_dn0_slot: &mut f64,
        var_sif2_dn1_slot: &mut f64,
        var_sif2_dn2_slot: &mut f64,
        var_sif2_dn3_slot: &mut f64,
        var_sif2_rv_slot: &mut f64,
        var_sif_dn0_slot: &mut f64,
        var_sif_dn1_slot: &mut f64,
        var_sif_dn2_slot: &mut f64,
        var_sif_dn3_slot: &mut f64,
        var_sif_rv_slot: &mut f64,
        var_sif_sir_2_slot: &mut f64,
        var_sif_sir_2_dn0_slot: &mut f64,
        var_sif_sir_2_dn1_slot: &mut f64,
        var_sif_sir_2_dn2_slot: &mut f64,
        var_sif_sir_2_dn3_slot: &mut f64,
        var_sif_sir_2_rv_slot: &mut f64,
        var_sir_slot: &mut f64,
        var_sir2_slot: &mut f64,
        var_sir2_dn0_slot: &mut f64,
        var_sir2_dn1_slot: &mut f64,
        var_sir2_dn2_slot: &mut f64,
        var_sir2_dn3_slot: &mut f64,
        var_sir2_rv_slot: &mut f64,
        var_sir_dn0_slot: &mut f64,
        var_sir_dn1_slot: &mut f64,
        var_sir_dn2_slot: &mut f64,
        var_sir_dn3_slot: &mut f64,
        var_sir_rv_slot: &mut f64,
        var_sqrt_lprime_lmin_slot: &mut f64,
        var_sqrt_lprime_lmin_dn0_slot: &mut f64,
        var_sqrt_lprime_lmin_dn1_slot: &mut f64,
        var_sqrt_lprime_lmin_dn2_slot: &mut f64,
        var_sqrt_lprime_lmin_dn3_slot: &mut f64,
        var_sqrt_lprime_lmin_rv_slot: &mut f64,
        var_sqrt_phi_vp_2_slot: &mut f64,
        var_sqrt_phi_vp_2_dn0_slot: &mut f64,
        var_sqrt_phi_vp_2_dn1_slot: &mut f64,
        var_sqrt_phi_vp_2_dn2_slot: &mut f64,
        var_sqrt_phi_vp_2_dn3_slot: &mut f64,
        var_sqrt_phi_vp_2_rv_slot: &mut f64,
        var_tmp1_slot: &mut f64,
        var_tmp1_dn0_slot: &mut f64,
        var_tmp1_dn1_slot: &mut f64,
        var_tmp1_dn2_slot: &mut f64,
        var_tmp1_dn3_slot: &mut f64,
        var_tmp1_rv_slot: &mut f64,
        var_vp_phi_eps_slot: &mut f64,
        var_vp_phi_eps_dn0_slot: &mut f64,
        var_vp_phi_eps_dn1_slot: &mut f64,
        var_vp_phi_eps_dn2_slot: &mut f64,
        var_vp_phi_eps_dn3_slot: &mut f64,
        var_vp_phi_eps_rv_slot: &mut f64,
        var_yk_slot: &mut f64,
        var_yk_dn0_slot: &mut f64,
        var_yk_dn1_slot: &mut f64,
        var_yk_dn2_slot: &mut f64,
        var_yk_dn3_slot: &mut f64,
        var_yk_rv_slot: &mut f64,
        var_z0_slot: &mut f64,
        var_z0_dn0_slot: &mut f64,
        var_z0_dn1_slot: &mut f64,
        var_z0_dn2_slot: &mut f64,
        var_z0_dn3_slot: &mut f64,
        var_z0_rv_slot: &mut f64,
        var_zk_slot: &mut f64,
        var_zk_dn0_slot: &mut f64,
        var_zk_dn1_slot: &mut f64,
        var_zk_dn2_slot: &mut f64,
        var_zk_dn3_slot: &mut f64,
        var_zk_rv_slot: &mut f64,
    ) {
        let mut var_deltal: f64 = *var_deltal_slot;
        let mut var_deltal_dn0: f64 = *var_deltal_dn0_slot;
        let mut var_deltal_dn1: f64 = *var_deltal_dn1_slot;
        let mut var_deltal_dn2: f64 = *var_deltal_dn2_slot;
        let mut var_deltal_dn3: f64 = *var_deltal_dn3_slot;
        let mut var_deltal_rv: f64 = *var_deltal_rv_slot;
        let mut var_dir_dv: f64 = *var_dir_dv_slot;
        let mut var_dir_dv_dn0: f64 = *var_dir_dv_dn0_slot;
        let mut var_dir_dv_dn1: f64 = *var_dir_dv_dn1_slot;
        let mut var_dir_dv_dn2: f64 = *var_dir_dv_dn2_slot;
        let mut var_dir_dv_dn3: f64 = *var_dir_dv_dn3_slot;
        let mut var_dir_dv_rv: f64 = *var_dir_dv_rv_slot;
        let mut var_dirprime_dv: f64 = *var_dirprime_dv_slot;
        let mut var_dirprime_dv_dn0: f64 = *var_dirprime_dv_dn0_slot;
        let mut var_dirprime_dv_dn1: f64 = *var_dirprime_dv_dn1_slot;
        let mut var_dirprime_dv_dn2: f64 = *var_dirprime_dv_dn2_slot;
        let mut var_dirprime_dv_dn3: f64 = *var_dirprime_dv_dn3_slot;
        let mut var_dirprime_dv_rv: f64 = *var_dirprime_dv_rv_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard10_rv: f64 = *var_guard10_rv_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard11_rv: f64 = *var_guard11_rv_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard12_rv: f64 = *var_guard12_rv_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard13_rv: f64 = *var_guard13_rv_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard14_rv: f64 = *var_guard14_rv_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard15_rv: f64 = *var_guard15_rv_slot;
        let mut var_ir: f64 = *var_ir_slot;
        let mut var_ir_dn0: f64 = *var_ir_dn0_slot;
        let mut var_ir_dn1: f64 = *var_ir_dn1_slot;
        let mut var_ir_dn2: f64 = *var_ir_dn2_slot;
        let mut var_ir_dn3: f64 = *var_ir_dn3_slot;
        let mut var_ir_rv: f64 = *var_ir_rv_slot;
        let mut var_irprime: f64 = *var_irprime_slot;
        let mut var_irprime_dn0: f64 = *var_irprime_dn0_slot;
        let mut var_irprime_dn1: f64 = *var_irprime_dn1_slot;
        let mut var_irprime_dn2: f64 = *var_irprime_dn2_slot;
        let mut var_irprime_dn3: f64 = *var_irprime_dn3_slot;
        let mut var_irprime_rv: f64 = *var_irprime_rv_slot;
        let mut var_leq: f64 = *var_leq_slot;
        let mut var_leq_dn0: f64 = *var_leq_dn0_slot;
        let mut var_leq_dn1: f64 = *var_leq_dn1_slot;
        let mut var_leq_dn2: f64 = *var_leq_dn2_slot;
        let mut var_leq_dn3: f64 = *var_leq_dn3_slot;
        let mut var_leq_rv: f64 = *var_leq_rv_slot;
        let mut var_lmin: f64 = *var_lmin_slot;
        let mut var_lmin_rv: f64 = *var_lmin_rv_slot;
        let mut var_lprime: f64 = *var_lprime_slot;
        let mut var_lprime_dn0: f64 = *var_lprime_dn0_slot;
        let mut var_lprime_dn1: f64 = *var_lprime_dn1_slot;
        let mut var_lprime_dn2: f64 = *var_lprime_dn2_slot;
        let mut var_lprime_dn3: f64 = *var_lprime_dn3_slot;
        let mut var_lprime_rv: f64 = *var_lprime_rv_slot;
        let mut var_n_1: f64 = *var_n_1_slot;
        let mut var_n_1_dn0: f64 = *var_n_1_dn0_slot;
        let mut var_n_1_dn1: f64 = *var_n_1_dn1_slot;
        let mut var_n_1_dn2: f64 = *var_n_1_dn2_slot;
        let mut var_n_1_dn3: f64 = *var_n_1_dn3_slot;
        let mut var_n_1_n: f64 = *var_n_1_n_slot;
        let mut var_n_1_n_dn0: f64 = *var_n_1_n_dn0_slot;
        let mut var_n_1_n_dn1: f64 = *var_n_1_n_dn1_slot;
        let mut var_n_1_n_dn2: f64 = *var_n_1_n_dn2_slot;
        let mut var_n_1_n_dn3: f64 = *var_n_1_n_dn3_slot;
        let mut var_n_1_n_rv: f64 = *var_n_1_n_rv_slot;
        let mut var_n_1_rv: f64 = *var_n_1_rv_slot;
        let mut var_sif: f64 = *var_sif_slot;
        let mut var_sif2: f64 = *var_sif2_slot;
        let mut var_sif2_dn0: f64 = *var_sif2_dn0_slot;
        let mut var_sif2_dn1: f64 = *var_sif2_dn1_slot;
        let mut var_sif2_dn2: f64 = *var_sif2_dn2_slot;
        let mut var_sif2_dn3: f64 = *var_sif2_dn3_slot;
        let mut var_sif2_rv: f64 = *var_sif2_rv_slot;
        let mut var_sif_dn0: f64 = *var_sif_dn0_slot;
        let mut var_sif_dn1: f64 = *var_sif_dn1_slot;
        let mut var_sif_dn2: f64 = *var_sif_dn2_slot;
        let mut var_sif_dn3: f64 = *var_sif_dn3_slot;
        let mut var_sif_rv: f64 = *var_sif_rv_slot;
        let mut var_sif_sir_2: f64 = *var_sif_sir_2_slot;
        let mut var_sif_sir_2_dn0: f64 = *var_sif_sir_2_dn0_slot;
        let mut var_sif_sir_2_dn1: f64 = *var_sif_sir_2_dn1_slot;
        let mut var_sif_sir_2_dn2: f64 = *var_sif_sir_2_dn2_slot;
        let mut var_sif_sir_2_dn3: f64 = *var_sif_sir_2_dn3_slot;
        let mut var_sif_sir_2_rv: f64 = *var_sif_sir_2_rv_slot;
        let mut var_sir: f64 = *var_sir_slot;
        let mut var_sir2: f64 = *var_sir2_slot;
        let mut var_sir2_dn0: f64 = *var_sir2_dn0_slot;
        let mut var_sir2_dn1: f64 = *var_sir2_dn1_slot;
        let mut var_sir2_dn2: f64 = *var_sir2_dn2_slot;
        let mut var_sir2_dn3: f64 = *var_sir2_dn3_slot;
        let mut var_sir2_rv: f64 = *var_sir2_rv_slot;
        let mut var_sir_dn0: f64 = *var_sir_dn0_slot;
        let mut var_sir_dn1: f64 = *var_sir_dn1_slot;
        let mut var_sir_dn2: f64 = *var_sir_dn2_slot;
        let mut var_sir_dn3: f64 = *var_sir_dn3_slot;
        let mut var_sir_rv: f64 = *var_sir_rv_slot;
        let mut var_sqrt_lprime_lmin: f64 = *var_sqrt_lprime_lmin_slot;
        let mut var_sqrt_lprime_lmin_dn0: f64 = *var_sqrt_lprime_lmin_dn0_slot;
        let mut var_sqrt_lprime_lmin_dn1: f64 = *var_sqrt_lprime_lmin_dn1_slot;
        let mut var_sqrt_lprime_lmin_dn2: f64 = *var_sqrt_lprime_lmin_dn2_slot;
        let mut var_sqrt_lprime_lmin_dn3: f64 = *var_sqrt_lprime_lmin_dn3_slot;
        let mut var_sqrt_lprime_lmin_rv: f64 = *var_sqrt_lprime_lmin_rv_slot;
        let mut var_sqrt_phi_vp_2: f64 = *var_sqrt_phi_vp_2_slot;
        let mut var_sqrt_phi_vp_2_dn0: f64 = *var_sqrt_phi_vp_2_dn0_slot;
        let mut var_sqrt_phi_vp_2_dn1: f64 = *var_sqrt_phi_vp_2_dn1_slot;
        let mut var_sqrt_phi_vp_2_dn2: f64 = *var_sqrt_phi_vp_2_dn2_slot;
        let mut var_sqrt_phi_vp_2_dn3: f64 = *var_sqrt_phi_vp_2_dn3_slot;
        let mut var_sqrt_phi_vp_2_rv: f64 = *var_sqrt_phi_vp_2_rv_slot;
        let mut var_tmp1: f64 = *var_tmp1_slot;
        let mut var_tmp1_dn0: f64 = *var_tmp1_dn0_slot;
        let mut var_tmp1_dn1: f64 = *var_tmp1_dn1_slot;
        let mut var_tmp1_dn2: f64 = *var_tmp1_dn2_slot;
        let mut var_tmp1_dn3: f64 = *var_tmp1_dn3_slot;
        let mut var_tmp1_rv: f64 = *var_tmp1_rv_slot;
        let mut var_vp_phi_eps: f64 = *var_vp_phi_eps_slot;
        let mut var_vp_phi_eps_dn0: f64 = *var_vp_phi_eps_dn0_slot;
        let mut var_vp_phi_eps_dn1: f64 = *var_vp_phi_eps_dn1_slot;
        let mut var_vp_phi_eps_dn2: f64 = *var_vp_phi_eps_dn2_slot;
        let mut var_vp_phi_eps_dn3: f64 = *var_vp_phi_eps_dn3_slot;
        let mut var_vp_phi_eps_rv: f64 = *var_vp_phi_eps_rv_slot;
        let mut var_yk: f64 = *var_yk_slot;
        let mut var_yk_dn0: f64 = *var_yk_dn0_slot;
        let mut var_yk_dn1: f64 = *var_yk_dn1_slot;
        let mut var_yk_dn2: f64 = *var_yk_dn2_slot;
        let mut var_yk_dn3: f64 = *var_yk_dn3_slot;
        let mut var_yk_rv: f64 = *var_yk_rv_slot;
        let mut var_z0: f64 = *var_z0_slot;
        let mut var_z0_dn0: f64 = *var_z0_dn0_slot;
        let mut var_z0_dn1: f64 = *var_z0_dn1_slot;
        let mut var_z0_dn2: f64 = *var_z0_dn2_slot;
        let mut var_z0_dn3: f64 = *var_z0_dn3_slot;
        let mut var_z0_rv: f64 = *var_z0_rv_slot;
        let mut var_zk: f64 = *var_zk_slot;
        let mut var_zk_dn0: f64 = *var_zk_dn0_slot;
        let mut var_zk_dn1: f64 = *var_zk_dn1_slot;
        let mut var_zk_dn2: f64 = *var_zk_dn2_slot;
        let mut var_zk_dn3: f64 = *var_zk_dn3_slot;
        let mut var_zk_rv: f64 = *var_zk_rv_slot;

        let assign1180_e916: f64 = (var_vp - var_vds);
        let assign1180_e918: f64 = (assign1180_e916 - var_vs);
        let assign1180_e920: f64 = (assign1180_e918 - var_sqrt_vdssprime_deltav);
        let assign1180_e922: f64 = (assign1180_e920 + var_sqrt_vds_vdssprime_deltav);
        let assign1180_e924: f64 = (assign1180_e922 * var_inv_vt);
        var_tmp1 = assign1180_e924;
        var_tmp1_dn0 = (((((var_vp_dn0 - var_vds_dn0) - var_vs_dn0) - var_sqrt_vdssprime_deltav_dn0) + var_sqrt_vds_vdssprime_deltav_dn0) * var_inv_vt);
        var_tmp1_dn1 = (((var_vp_dn1 - var_sqrt_vdssprime_deltav_dn1) + var_sqrt_vds_vdssprime_deltav_dn1) * var_inv_vt);
        var_tmp1_dn2 = (((((var_vp_dn2 - var_vds_dn2) - var_vs_dn2) - var_sqrt_vdssprime_deltav_dn2) + var_sqrt_vds_vdssprime_deltav_dn2) * var_inv_vt);
        var_tmp1_dn3 = (((((var_vp_dn3 - var_vds_dn3) - var_vs_dn3) - var_sqrt_vdssprime_deltav_dn3) + var_sqrt_vds_vdssprime_deltav_dn3) * var_inv_vt);
        var_tmp1_rv = 0.0;

        let assign1190_e927: f64 = (-0.35);
        let assign1190_e928: f64 = if var_tmp1 > assign1190_e927 { 1.0 } else { 0.0 };
        var_guard10 = assign1190_e928;
        var_guard10_rv = 0.0;

        let (assign1200_e941, assign1200_e941_d_n0, assign1200_e941_d_n1, assign1200_e941_d_n2, assign1200_e941_d_n3,) = {
    if (var_guard10 != 0.0) {
        let assign1200_e933: f64 = (1.3 + var_tmp1);
        let assign1200_e936: f64 = (var_tmp1 + 1.6);
        let assign1200_e937: f64 = (assign1200_e936).ln();
        let assign1200_e938: f64 = (assign1200_e933 - assign1200_e937);
        let assign1200_e939: f64 = (2.0 / assign1200_e938);
        (assign1200_e939, (-((2.0 * (var_tmp1_dn0 - (var_tmp1_dn0 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (var_tmp1_dn1 - (var_tmp1_dn1 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (var_tmp1_dn2 - (var_tmp1_dn2 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (var_tmp1_dn3 - (var_tmp1_dn3 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign1200_e941;
        var_z0_dn0 = assign1200_e941_d_n0;
        var_z0_dn1 = assign1200_e941_d_n1;
        var_z0_dn2 = assign1200_e941_d_n2;
        var_z0_dn3 = assign1200_e941_d_n3;
        var_z0_rv = 0.0;

        let (assign1210_e954, assign1210_e954_d_n0, assign1210_e954_d_n1, assign1210_e954_d_n2, assign1210_e954_d_n3,) = {
    if (var_guard10 != 0.0) {
        let assign1210_e945: f64 = (2.0 + var_z0);
        let assign1210_e948: f64 = (1.0 + var_tmp1);
        let assign1210_e950: f64 = (var_z0).ln();
        let assign1210_e951: f64 = (assign1210_e948 + assign1210_e950);
        let assign1210_e952: f64 = (assign1210_e945 / assign1210_e951);
        (assign1210_e952, (((var_z0_dn0 * assign1210_e951) - (assign1210_e945 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign1210_e951 * assign1210_e951)), (((var_z0_dn1 * assign1210_e951) - (assign1210_e945 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign1210_e951 * assign1210_e951)), (((var_z0_dn2 * assign1210_e951) - (assign1210_e945 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign1210_e951 * assign1210_e951)), (((var_z0_dn3 * assign1210_e951) - (assign1210_e945 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign1210_e951 * assign1210_e951)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign1210_e954;
        var_zk_dn0 = assign1210_e954_d_n0;
        var_zk_dn1 = assign1210_e954_d_n1;
        var_zk_dn2 = assign1210_e954_d_n2;
        var_zk_dn3 = assign1210_e954_d_n3;
        var_zk_rv = 0.0;

        let (assign1220_e967, assign1220_e967_d_n0, assign1220_e967_d_n1, assign1220_e967_d_n2, assign1220_e967_d_n3,) = {
    if (var_guard10 != 0.0) {
        let assign1220_e958: f64 = (1.0 + var_tmp1);
        let assign1220_e960: f64 = (var_zk).ln();
        let assign1220_e961: f64 = (assign1220_e958 + assign1220_e960);
        let assign1220_e964: f64 = (2.0 + var_zk);
        let assign1220_e965: f64 = (assign1220_e961 / assign1220_e964);
        (assign1220_e965, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign1220_e964) - (assign1220_e961 * var_zk_dn0)) / (assign1220_e964 * assign1220_e964)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign1220_e964) - (assign1220_e961 * var_zk_dn1)) / (assign1220_e964 * assign1220_e964)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign1220_e964) - (assign1220_e961 * var_zk_dn2)) / (assign1220_e964 * assign1220_e964)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign1220_e964) - (assign1220_e961 * var_zk_dn3)) / (assign1220_e964 * assign1220_e964)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1220_e967;
        var_yk_dn0 = assign1220_e967_d_n0;
        var_yk_dn1 = assign1220_e967_d_n1;
        var_yk_dn2 = assign1220_e967_d_n2;
        var_yk_dn3 = assign1220_e967_d_n3;
        var_yk_rv = 0.0;

        let assign1230_e970: f64 = (-15.0);
        let assign1230_e971: f64 = if var_tmp1 > assign1230_e970 { 1.0 } else { 0.0 };
        var_guard11 = assign1230_e971;
        var_guard11_rv = 0.0;

        let (assign1240_e982, assign1240_e982_d_n0, assign1240_e982_d_n1, assign1240_e982_d_n2, assign1240_e982_d_n3,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let assign1240_e978: f64 = (-var_tmp1);
        let assign1240_e979: f64 = (assign1240_e978).exp();
        let assign1240_e980: f64 = (1.55 + assign1240_e979);
        (assign1240_e980, (assign1240_e979 * (-var_tmp1_dn0)), (assign1240_e979 * (-var_tmp1_dn1)), (assign1240_e979 * (-var_tmp1_dn2)), (assign1240_e979 * (-var_tmp1_dn3)),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign1240_e982;
        var_z0_dn0 = assign1240_e982_d_n0;
        var_z0_dn1 = assign1240_e982_d_n1;
        var_z0_dn2 = assign1240_e982_d_n2;
        var_z0_dn3 = assign1240_e982_d_n3;
        var_z0_rv = 0.0;

        let (assign1250_e998, assign1250_e998_d_n0, assign1250_e998_d_n1, assign1250_e998_d_n2, assign1250_e998_d_n3,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let assign1250_e989: f64 = (2.0 + var_z0);
        let assign1250_e992: f64 = (1.0 + var_tmp1);
        let assign1250_e994: f64 = (var_z0).ln();
        let assign1250_e995: f64 = (assign1250_e992 + assign1250_e994);
        let assign1250_e996: f64 = (assign1250_e989 / assign1250_e995);
        (assign1250_e996, (((var_z0_dn0 * assign1250_e995) - (assign1250_e989 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign1250_e995 * assign1250_e995)), (((var_z0_dn1 * assign1250_e995) - (assign1250_e989 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign1250_e995 * assign1250_e995)), (((var_z0_dn2 * assign1250_e995) - (assign1250_e989 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign1250_e995 * assign1250_e995)), (((var_z0_dn3 * assign1250_e995) - (assign1250_e989 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign1250_e995 * assign1250_e995)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign1250_e998;
        var_zk_dn0 = assign1250_e998_d_n0;
        var_zk_dn1 = assign1250_e998_d_n1;
        var_zk_dn2 = assign1250_e998_d_n2;
        var_zk_dn3 = assign1250_e998_d_n3;
        var_zk_rv = 0.0;

        let (assign1260_e1014, assign1260_e1014_d_n0, assign1260_e1014_d_n1, assign1260_e1014_d_n2, assign1260_e1014_d_n3,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let assign1260_e1005: f64 = (1.0 + var_tmp1);
        let assign1260_e1007: f64 = (var_zk).ln();
        let assign1260_e1008: f64 = (assign1260_e1005 + assign1260_e1007);
        let assign1260_e1011: f64 = (2.0 + var_zk);
        let assign1260_e1012: f64 = (assign1260_e1008 / assign1260_e1011);
        (assign1260_e1012, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign1260_e1011) - (assign1260_e1008 * var_zk_dn0)) / (assign1260_e1011 * assign1260_e1011)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign1260_e1011) - (assign1260_e1008 * var_zk_dn1)) / (assign1260_e1011 * assign1260_e1011)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign1260_e1011) - (assign1260_e1008 * var_zk_dn2)) / (assign1260_e1011 * assign1260_e1011)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign1260_e1011) - (assign1260_e1008 * var_zk_dn3)) / (assign1260_e1011 * assign1260_e1011)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1260_e1014;
        var_yk_dn0 = assign1260_e1014_d_n0;
        var_yk_dn1 = assign1260_e1014_d_n1;
        var_yk_dn2 = assign1260_e1014_d_n2;
        var_yk_dn3 = assign1260_e1014_d_n3;
        var_yk_rv = 0.0;

        let assign1270_e1017: f64 = (-23.0);
        let assign1270_e1018: f64 = if var_tmp1 > assign1270_e1017 { 1.0 } else { 0.0 };
        var_guard12 = assign1270_e1018;
        var_guard12_rv = 0.0;

        let (assign1280_e1034, assign1280_e1034_d_n0, assign1280_e1034_d_n1, assign1280_e1034_d_n2, assign1280_e1034_d_n3,) = {
    if (((var_guard10 == 0.0) && (var_guard11 == 0.0)) && (var_guard12 != 0.0)) {
        let assign1280_e1029: f64 = (-var_tmp1);
        let assign1280_e1030: f64 = (assign1280_e1029).exp();
        let assign1280_e1031: f64 = (2.0 + assign1280_e1030);
        let assign1280_e1032: f64 = (1.0 / assign1280_e1031);
        (assign1280_e1032, (-((assign1280_e1030 * (-var_tmp1_dn0)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-var_tmp1_dn1)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-var_tmp1_dn2)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-var_tmp1_dn3)) / (assign1280_e1031 * assign1280_e1031))),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1280_e1034;
        var_yk_dn0 = assign1280_e1034_d_n0;
        var_yk_dn1 = assign1280_e1034_d_n1;
        var_yk_dn2 = assign1280_e1034_d_n2;
        var_yk_dn3 = assign1280_e1034_d_n3;
        var_yk_rv = 0.0;

        let (assign1290_e1048, assign1290_e1048_d_n0, assign1290_e1048_d_n1, assign1290_e1048_d_n2, assign1290_e1048_d_n3,) = {
    if (((var_guard10 == 0.0) && (var_guard11 == 0.0)) && (var_guard12 == 0.0)) {
        let assign1290_e1044: f64 = (var_tmp1).exp();
        let assign1290_e1046: f64 = (assign1290_e1044 + 1e-64);
        (assign1290_e1046, (assign1290_e1044 * var_tmp1_dn0), (assign1290_e1044 * var_tmp1_dn1), (assign1290_e1044 * var_tmp1_dn2), (assign1290_e1044 * var_tmp1_dn3),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1290_e1048;
        var_yk_dn0 = assign1290_e1048_d_n0;
        var_yk_dn1 = assign1290_e1048_d_n1;
        var_yk_dn2 = assign1290_e1048_d_n2;
        var_yk_dn3 = assign1290_e1048_d_n3;
        var_yk_rv = 0.0;

        let assign1300_e1052: f64 = (1.0 + var_yk);
        let assign1300_e1053: f64 = (var_yk * assign1300_e1052);
        var_irprime = assign1300_e1053;
        var_irprime_dn0 = ((var_yk_dn0 * assign1300_e1052) + (var_yk * var_yk_dn0));
        var_irprime_dn1 = ((var_yk_dn1 * assign1300_e1052) + (var_yk * var_yk_dn1));
        var_irprime_dn2 = ((var_yk_dn2 * assign1300_e1052) + (var_yk * var_yk_dn2));
        var_irprime_dn3 = ((var_yk_dn3 * assign1300_e1052) + (var_yk * var_yk_dn3));
        var_irprime_rv = 0.0;

        var_dirprime_dv = var_yk;
        var_dirprime_dv_dn0 = var_yk_dn0;
        var_dirprime_dv_dn1 = var_yk_dn1;
        var_dirprime_dv_dn2 = var_yk_dn2;
        var_dirprime_dv_dn3 = var_yk_dn3;
        var_dirprime_dv_rv = 0.0;

        let assign1330_e1061: f64 = (var_vds - var_vip);
        let assign1330_e1063: f64 = (assign1330_e1061 / var_lc_ucrit);
        let assign1330_e1064: f64 = (1.0 + assign1330_e1063);
        let assign1330_e1065: f64 = (assign1330_e1064).ln();
        let assign1330_e1066: f64 = (var_lc_lambda * assign1330_e1065);
        var_deltal = assign1330_e1066;
        var_deltal_dn0 = (var_lc_lambda * (((var_vds_dn0 - var_vip_dn0) / var_lc_ucrit) / assign1330_e1064));
        var_deltal_dn1 = (var_lc_lambda * (((-var_vip_dn1) / var_lc_ucrit) / assign1330_e1064));
        var_deltal_dn2 = (var_lc_lambda * (((var_vds_dn2 - var_vip_dn2) / var_lc_ucrit) / assign1330_e1064));
        var_deltal_dn3 = (var_lc_lambda * (((var_vds_dn3 - var_vip_dn3) / var_lc_ucrit) / assign1330_e1064));
        var_deltal_rv = 0.0;

        let assign1340_e1069: f64 = (var_leff - var_deltal);
        let assign1340_e1072: f64 = (var_vds + var_vip);
        let assign1340_e1074: f64 = (assign1340_e1072 * var_inv_ucrit);
        let assign1340_e1075: f64 = (assign1340_e1069 + assign1340_e1074);
        var_lprime = assign1340_e1075;
        var_lprime_dn0 = ((-var_deltal_dn0) + ((var_vds_dn0 + var_vip_dn0) * var_inv_ucrit));
        var_lprime_dn1 = ((-var_deltal_dn1) + (var_vip_dn1 * var_inv_ucrit));
        var_lprime_dn2 = ((-var_deltal_dn2) + ((var_vds_dn2 + var_vip_dn2) * var_inv_ucrit));
        var_lprime_dn3 = ((-var_deltal_dn3) + ((var_vds_dn3 + var_vip_dn3) * var_inv_ucrit));
        var_lprime_rv = 0.0;

        let assign1350_e1078: f64 = (0.1 * var_leff);
        var_lmin = assign1350_e1078;
        var_lmin_rv = 0.0;

        let assign1360_e1081: f64 = (var_lprime * var_lprime);
        let assign1360_e1084: f64 = (var_lmin * var_lmin);
        let assign1360_e1085: f64 = (assign1360_e1081 + assign1360_e1084);
        let assign1360_e1086: f64 = (assign1360_e1085).sqrt();
        var_sqrt_lprime_lmin = assign1360_e1086;
        var_sqrt_lprime_lmin_dn0 = (((var_lprime_dn0 * var_lprime) + (var_lprime * var_lprime_dn0)) / (2.0 * assign1360_e1086));
        var_sqrt_lprime_lmin_dn1 = (((var_lprime_dn1 * var_lprime) + (var_lprime * var_lprime_dn1)) / (2.0 * assign1360_e1086));
        var_sqrt_lprime_lmin_dn2 = (((var_lprime_dn2 * var_lprime) + (var_lprime * var_lprime_dn2)) / (2.0 * assign1360_e1086));
        var_sqrt_lprime_lmin_dn3 = (((var_lprime_dn3 * var_lprime) + (var_lprime * var_lprime_dn3)) / (2.0 * assign1360_e1086));
        var_sqrt_lprime_lmin_rv = 0.0;

        let assign1370_e1090: f64 = (var_lprime + var_sqrt_lprime_lmin);
        let assign1370_e1091: f64 = (0.5 * assign1370_e1090);
        var_leq = assign1370_e1091;
        var_leq_dn0 = (0.5 * (var_lprime_dn0 + var_sqrt_lprime_lmin_dn0));
        var_leq_dn1 = (0.5 * (var_lprime_dn1 + var_sqrt_lprime_lmin_dn1));
        var_leq_dn2 = (0.5 * (var_lprime_dn2 + var_sqrt_lprime_lmin_dn2));
        var_leq_dn3 = (0.5 * (var_lprime_dn3 + var_sqrt_lprime_lmin_dn3));
        var_leq_rv = 0.0;

        let assign1380_e1094: f64 = (var_vp - var_vd);
        let assign1380_e1096: f64 = (assign1380_e1094 * var_inv_vt);
        var_tmp1 = assign1380_e1096;
        var_tmp1_dn0 = ((var_vp_dn0 - var_vd_dn0) * var_inv_vt);
        var_tmp1_dn1 = (var_vp_dn1 * var_inv_vt);
        var_tmp1_dn2 = ((var_vp_dn2 - var_vd_dn2) * var_inv_vt);
        var_tmp1_dn3 = ((var_vp_dn3 - var_vd_dn3) * var_inv_vt);
        var_tmp1_rv = 0.0;

        let assign1390_e1099: f64 = (-0.35);
        let assign1390_e1100: f64 = if var_tmp1 > assign1390_e1099 { 1.0 } else { 0.0 };
        var_guard13 = assign1390_e1100;
        var_guard13_rv = 0.0;

        let (assign1400_e1113, assign1400_e1113_d_n0, assign1400_e1113_d_n1, assign1400_e1113_d_n2, assign1400_e1113_d_n3,) = {
    if (var_guard13 != 0.0) {
        let assign1400_e1105: f64 = (1.3 + var_tmp1);
        let assign1400_e1108: f64 = (var_tmp1 + 1.6);
        let assign1400_e1109: f64 = (assign1400_e1108).ln();
        let assign1400_e1110: f64 = (assign1400_e1105 - assign1400_e1109);
        let assign1400_e1111: f64 = (2.0 / assign1400_e1110);
        (assign1400_e1111, (-((2.0 * (var_tmp1_dn0 - (var_tmp1_dn0 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (var_tmp1_dn1 - (var_tmp1_dn1 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (var_tmp1_dn2 - (var_tmp1_dn2 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (var_tmp1_dn3 - (var_tmp1_dn3 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign1400_e1113;
        var_z0_dn0 = assign1400_e1113_d_n0;
        var_z0_dn1 = assign1400_e1113_d_n1;
        var_z0_dn2 = assign1400_e1113_d_n2;
        var_z0_dn3 = assign1400_e1113_d_n3;
        var_z0_rv = 0.0;

        let (assign1410_e1126, assign1410_e1126_d_n0, assign1410_e1126_d_n1, assign1410_e1126_d_n2, assign1410_e1126_d_n3,) = {
    if (var_guard13 != 0.0) {
        let assign1410_e1117: f64 = (2.0 + var_z0);
        let assign1410_e1120: f64 = (1.0 + var_tmp1);
        let assign1410_e1122: f64 = (var_z0).ln();
        let assign1410_e1123: f64 = (assign1410_e1120 + assign1410_e1122);
        let assign1410_e1124: f64 = (assign1410_e1117 / assign1410_e1123);
        (assign1410_e1124, (((var_z0_dn0 * assign1410_e1123) - (assign1410_e1117 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((var_z0_dn1 * assign1410_e1123) - (assign1410_e1117 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((var_z0_dn2 * assign1410_e1123) - (assign1410_e1117 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((var_z0_dn3 * assign1410_e1123) - (assign1410_e1117 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign1410_e1123 * assign1410_e1123)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign1410_e1126;
        var_zk_dn0 = assign1410_e1126_d_n0;
        var_zk_dn1 = assign1410_e1126_d_n1;
        var_zk_dn2 = assign1410_e1126_d_n2;
        var_zk_dn3 = assign1410_e1126_d_n3;
        var_zk_rv = 0.0;

        let (assign1420_e1139, assign1420_e1139_d_n0, assign1420_e1139_d_n1, assign1420_e1139_d_n2, assign1420_e1139_d_n3,) = {
    if (var_guard13 != 0.0) {
        let assign1420_e1130: f64 = (1.0 + var_tmp1);
        let assign1420_e1132: f64 = (var_zk).ln();
        let assign1420_e1133: f64 = (assign1420_e1130 + assign1420_e1132);
        let assign1420_e1136: f64 = (2.0 + var_zk);
        let assign1420_e1137: f64 = (assign1420_e1133 / assign1420_e1136);
        (assign1420_e1137, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign1420_e1136) - (assign1420_e1133 * var_zk_dn0)) / (assign1420_e1136 * assign1420_e1136)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign1420_e1136) - (assign1420_e1133 * var_zk_dn1)) / (assign1420_e1136 * assign1420_e1136)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign1420_e1136) - (assign1420_e1133 * var_zk_dn2)) / (assign1420_e1136 * assign1420_e1136)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign1420_e1136) - (assign1420_e1133 * var_zk_dn3)) / (assign1420_e1136 * assign1420_e1136)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1420_e1139;
        var_yk_dn0 = assign1420_e1139_d_n0;
        var_yk_dn1 = assign1420_e1139_d_n1;
        var_yk_dn2 = assign1420_e1139_d_n2;
        var_yk_dn3 = assign1420_e1139_d_n3;
        var_yk_rv = 0.0;

        let assign1430_e1142: f64 = (-15.0);
        let assign1430_e1143: f64 = if var_tmp1 > assign1430_e1142 { 1.0 } else { 0.0 };
        var_guard14 = assign1430_e1143;
        var_guard14_rv = 0.0;

        let (assign1440_e1154, assign1440_e1154_d_n0, assign1440_e1154_d_n1, assign1440_e1154_d_n2, assign1440_e1154_d_n3,) = {
    if ((var_guard13 == 0.0) && (var_guard14 != 0.0)) {
        let assign1440_e1150: f64 = (-var_tmp1);
        let assign1440_e1151: f64 = (assign1440_e1150).exp();
        let assign1440_e1152: f64 = (1.55 + assign1440_e1151);
        (assign1440_e1152, (assign1440_e1151 * (-var_tmp1_dn0)), (assign1440_e1151 * (-var_tmp1_dn1)), (assign1440_e1151 * (-var_tmp1_dn2)), (assign1440_e1151 * (-var_tmp1_dn3)),)
    } else {
        (var_z0, var_z0_dn0, var_z0_dn1, var_z0_dn2, var_z0_dn3,)
    }
};
        var_z0 = assign1440_e1154;
        var_z0_dn0 = assign1440_e1154_d_n0;
        var_z0_dn1 = assign1440_e1154_d_n1;
        var_z0_dn2 = assign1440_e1154_d_n2;
        var_z0_dn3 = assign1440_e1154_d_n3;
        var_z0_rv = 0.0;

        let (assign1450_e1170, assign1450_e1170_d_n0, assign1450_e1170_d_n1, assign1450_e1170_d_n2, assign1450_e1170_d_n3,) = {
    if ((var_guard13 == 0.0) && (var_guard14 != 0.0)) {
        let assign1450_e1161: f64 = (2.0 + var_z0);
        let assign1450_e1164: f64 = (1.0 + var_tmp1);
        let assign1450_e1166: f64 = (var_z0).ln();
        let assign1450_e1167: f64 = (assign1450_e1164 + assign1450_e1166);
        let assign1450_e1168: f64 = (assign1450_e1161 / assign1450_e1167);
        (assign1450_e1168, (((var_z0_dn0 * assign1450_e1167) - (assign1450_e1161 * (var_tmp1_dn0 + (var_z0_dn0 / var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((var_z0_dn1 * assign1450_e1167) - (assign1450_e1161 * (var_tmp1_dn1 + (var_z0_dn1 / var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((var_z0_dn2 * assign1450_e1167) - (assign1450_e1161 * (var_tmp1_dn2 + (var_z0_dn2 / var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((var_z0_dn3 * assign1450_e1167) - (assign1450_e1161 * (var_tmp1_dn3 + (var_z0_dn3 / var_z0)))) / (assign1450_e1167 * assign1450_e1167)),)
    } else {
        (var_zk, var_zk_dn0, var_zk_dn1, var_zk_dn2, var_zk_dn3,)
    }
};
        var_zk = assign1450_e1170;
        var_zk_dn0 = assign1450_e1170_d_n0;
        var_zk_dn1 = assign1450_e1170_d_n1;
        var_zk_dn2 = assign1450_e1170_d_n2;
        var_zk_dn3 = assign1450_e1170_d_n3;
        var_zk_rv = 0.0;

        let (assign1460_e1186, assign1460_e1186_d_n0, assign1460_e1186_d_n1, assign1460_e1186_d_n2, assign1460_e1186_d_n3,) = {
    if ((var_guard13 == 0.0) && (var_guard14 != 0.0)) {
        let assign1460_e1177: f64 = (1.0 + var_tmp1);
        let assign1460_e1179: f64 = (var_zk).ln();
        let assign1460_e1180: f64 = (assign1460_e1177 + assign1460_e1179);
        let assign1460_e1183: f64 = (2.0 + var_zk);
        let assign1460_e1184: f64 = (assign1460_e1180 / assign1460_e1183);
        (assign1460_e1184, ((((var_tmp1_dn0 + (var_zk_dn0 / var_zk)) * assign1460_e1183) - (assign1460_e1180 * var_zk_dn0)) / (assign1460_e1183 * assign1460_e1183)), ((((var_tmp1_dn1 + (var_zk_dn1 / var_zk)) * assign1460_e1183) - (assign1460_e1180 * var_zk_dn1)) / (assign1460_e1183 * assign1460_e1183)), ((((var_tmp1_dn2 + (var_zk_dn2 / var_zk)) * assign1460_e1183) - (assign1460_e1180 * var_zk_dn2)) / (assign1460_e1183 * assign1460_e1183)), ((((var_tmp1_dn3 + (var_zk_dn3 / var_zk)) * assign1460_e1183) - (assign1460_e1180 * var_zk_dn3)) / (assign1460_e1183 * assign1460_e1183)),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1460_e1186;
        var_yk_dn0 = assign1460_e1186_d_n0;
        var_yk_dn1 = assign1460_e1186_d_n1;
        var_yk_dn2 = assign1460_e1186_d_n2;
        var_yk_dn3 = assign1460_e1186_d_n3;
        var_yk_rv = 0.0;

        let assign1470_e1189: f64 = (-23.0);
        let assign1470_e1190: f64 = if var_tmp1 > assign1470_e1189 { 1.0 } else { 0.0 };
        var_guard15 = assign1470_e1190;
        var_guard15_rv = 0.0;

        let (assign1480_e1206, assign1480_e1206_d_n0, assign1480_e1206_d_n1, assign1480_e1206_d_n2, assign1480_e1206_d_n3,) = {
    if (((var_guard13 == 0.0) && (var_guard14 == 0.0)) && (var_guard15 != 0.0)) {
        let assign1480_e1201: f64 = (-var_tmp1);
        let assign1480_e1202: f64 = (assign1480_e1201).exp();
        let assign1480_e1203: f64 = (2.0 + assign1480_e1202);
        let assign1480_e1204: f64 = (1.0 / assign1480_e1203);
        (assign1480_e1204, (-((assign1480_e1202 * (-var_tmp1_dn0)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-var_tmp1_dn1)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-var_tmp1_dn2)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-var_tmp1_dn3)) / (assign1480_e1203 * assign1480_e1203))),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1480_e1206;
        var_yk_dn0 = assign1480_e1206_d_n0;
        var_yk_dn1 = assign1480_e1206_d_n1;
        var_yk_dn2 = assign1480_e1206_d_n2;
        var_yk_dn3 = assign1480_e1206_d_n3;
        var_yk_rv = 0.0;

        let (assign1490_e1220, assign1490_e1220_d_n0, assign1490_e1220_d_n1, assign1490_e1220_d_n2, assign1490_e1220_d_n3,) = {
    if (((var_guard13 == 0.0) && (var_guard14 == 0.0)) && (var_guard15 == 0.0)) {
        let assign1490_e1216: f64 = (var_tmp1).exp();
        let assign1490_e1218: f64 = (assign1490_e1216 + 1e-64);
        (assign1490_e1218, (assign1490_e1216 * var_tmp1_dn0), (assign1490_e1216 * var_tmp1_dn1), (assign1490_e1216 * var_tmp1_dn2), (assign1490_e1216 * var_tmp1_dn3),)
    } else {
        (var_yk, var_yk_dn0, var_yk_dn1, var_yk_dn2, var_yk_dn3,)
    }
};
        var_yk = assign1490_e1220;
        var_yk_dn0 = assign1490_e1220_d_n0;
        var_yk_dn1 = assign1490_e1220_d_n1;
        var_yk_dn2 = assign1490_e1220_d_n2;
        var_yk_dn3 = assign1490_e1220_d_n3;
        var_yk_rv = 0.0;

        let assign1500_e1224: f64 = (1.0 + var_yk);
        let assign1500_e1225: f64 = (var_yk * assign1500_e1224);
        var_ir = assign1500_e1225;
        var_ir_dn0 = ((var_yk_dn0 * assign1500_e1224) + (var_yk * var_yk_dn0));
        var_ir_dn1 = ((var_yk_dn1 * assign1500_e1224) + (var_yk * var_yk_dn1));
        var_ir_dn2 = ((var_yk_dn2 * assign1500_e1224) + (var_yk * var_yk_dn2));
        var_ir_dn3 = ((var_yk_dn3 * assign1500_e1224) + (var_yk * var_yk_dn3));
        var_ir_rv = 0.0;

        var_dir_dv = var_yk;
        var_dir_dv_dn0 = var_yk_dn0;
        var_dir_dv_dn1 = var_yk_dn1;
        var_dir_dv_dn2 = var_yk_dn2;
        var_dir_dv_dn3 = var_yk_dn3;
        var_dir_dv_rv = 0.0;

        let assign1530_e1231: f64 = (0.25 + var_if_);
        var_sif2 = assign1530_e1231;
        var_sif2_dn0 = var_if__dn0;
        var_sif2_dn1 = var_if__dn1;
        var_sif2_dn2 = var_if__dn2;
        var_sif2_dn3 = var_if__dn3;
        var_sif2_rv = 0.0;

        let assign1540_e1234: f64 = (0.25 + var_ir);
        var_sir2 = assign1540_e1234;
        var_sir2_dn0 = var_ir_dn0;
        var_sir2_dn1 = var_ir_dn1;
        var_sir2_dn2 = var_ir_dn2;
        var_sir2_dn3 = var_ir_dn3;
        var_sir2_rv = 0.0;

        let assign1550_e1236: f64 = (var_sif2).sqrt();
        var_sif = assign1550_e1236;
        var_sif_dn0 = (var_sif2_dn0 / (2.0 * assign1550_e1236));
        var_sif_dn1 = (var_sif2_dn1 / (2.0 * assign1550_e1236));
        var_sif_dn2 = (var_sif2_dn2 / (2.0 * assign1550_e1236));
        var_sif_dn3 = (var_sif2_dn3 / (2.0 * assign1550_e1236));
        var_sif_rv = 0.0;

        let assign1560_e1238: f64 = (var_sir2).sqrt();
        var_sir = assign1560_e1238;
        var_sir_dn0 = (var_sir2_dn0 / (2.0 * assign1560_e1238));
        var_sir_dn1 = (var_sir2_dn1 / (2.0 * assign1560_e1238));
        var_sir_dn2 = (var_sir2_dn2 / (2.0 * assign1560_e1238));
        var_sir_dn3 = (var_sir2_dn3 / (2.0 * assign1560_e1238));
        var_sir_rv = 0.0;

        let assign1570_e1241: f64 = (var_sif + var_sir);
        let assign1570_e1244: f64 = (var_sif + var_sir);
        let assign1570_e1245: f64 = (assign1570_e1241 * assign1570_e1244);
        var_sif_sir_2 = assign1570_e1245;
        var_sif_sir_2_dn0 = (((var_sif_dn0 + var_sir_dn0) * assign1570_e1244) + (assign1570_e1241 * (var_sif_dn0 + var_sir_dn0)));
        var_sif_sir_2_dn1 = (((var_sif_dn1 + var_sir_dn1) * assign1570_e1244) + (assign1570_e1241 * (var_sif_dn1 + var_sir_dn1)));
        var_sif_sir_2_dn2 = (((var_sif_dn2 + var_sir_dn2) * assign1570_e1244) + (assign1570_e1241 * (var_sif_dn2 + var_sir_dn2)));
        var_sif_sir_2_dn3 = (((var_sif_dn3 + var_sir_dn3) * assign1570_e1244) + (assign1570_e1241 * (var_sif_dn3 + var_sir_dn3)));
        var_sif_sir_2_rv = 0.0;

        let assign1580_e1248: f64 = (var_vp + var_phi_t);
        let assign1580_e1250: f64 = (assign1580_e1248 + 1e-6);
        var_vp_phi_eps = assign1580_e1250;
        var_vp_phi_eps_dn0 = (var_vp_dn0 + var_phi_t_dn0);
        var_vp_phi_eps_dn1 = (var_vp_dn1 + var_phi_t_dn1);
        var_vp_phi_eps_dn2 = (var_vp_dn2 + var_phi_t_dn2);
        var_vp_phi_eps_dn3 = (var_vp_dn3 + var_phi_t_dn3);
        var_vp_phi_eps_rv = 0.0;

        let assign1590_e1253: f64 = (var_vp_phi_eps).sqrt();
        let assign1590_e1254: f64 = (2.0 * assign1590_e1253);
        var_sqrt_phi_vp_2 = assign1590_e1254;
        var_sqrt_phi_vp_2_dn0 = (2.0 * (var_vp_phi_eps_dn0 / (2.0 * assign1590_e1253)));
        var_sqrt_phi_vp_2_dn1 = (2.0 * (var_vp_phi_eps_dn1 / (2.0 * assign1590_e1253)));
        var_sqrt_phi_vp_2_dn2 = (2.0 * (var_vp_phi_eps_dn2 / (2.0 * assign1590_e1253)));
        var_sqrt_phi_vp_2_dn3 = (2.0 * (var_vp_phi_eps_dn3 / (2.0 * assign1590_e1253)));
        var_sqrt_phi_vp_2_rv = 0.0;

        let assign1600_e1257: f64 = (var_gamma_s / var_sqrt_phi_vp_2);
        var_n_1 = assign1600_e1257;
        var_n_1_dn0 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn0) / (var_sqrt_phi_vp_2 * var_sqrt_phi_vp_2)));
        var_n_1_dn1 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn1) / (var_sqrt_phi_vp_2 * var_sqrt_phi_vp_2)));
        var_n_1_dn2 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn2) / (var_sqrt_phi_vp_2 * var_sqrt_phi_vp_2)));
        var_n_1_dn3 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn3) / (var_sqrt_phi_vp_2 * var_sqrt_phi_vp_2)));
        var_n_1_rv = 0.0;

        let assign1610_e1261: f64 = (var_sqrt_phi_vp_2 + var_gamma_s);
        let assign1610_e1262: f64 = (var_gamma_s / assign1610_e1261);
        var_n_1_n = assign1610_e1262;
        var_n_1_n_dn0 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn0) / (assign1610_e1261 * assign1610_e1261)));
        var_n_1_n_dn1 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn1) / (assign1610_e1261 * assign1610_e1261)));
        var_n_1_n_dn2 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn2) / (assign1610_e1261 * assign1610_e1261)));
        var_n_1_n_dn3 = (-((var_gamma_s * var_sqrt_phi_vp_2_dn3) / (assign1610_e1261 * assign1610_e1261)));
        var_n_1_n_rv = 0.0;

        *var_deltal_slot = var_deltal;
        *var_deltal_dn0_slot = var_deltal_dn0;
        *var_deltal_dn1_slot = var_deltal_dn1;
        *var_deltal_dn2_slot = var_deltal_dn2;
        *var_deltal_dn3_slot = var_deltal_dn3;
        *var_deltal_rv_slot = var_deltal_rv;
        *var_dir_dv_slot = var_dir_dv;
        *var_dir_dv_dn0_slot = var_dir_dv_dn0;
        *var_dir_dv_dn1_slot = var_dir_dv_dn1;
        *var_dir_dv_dn2_slot = var_dir_dv_dn2;
        *var_dir_dv_dn3_slot = var_dir_dv_dn3;
        *var_dir_dv_rv_slot = var_dir_dv_rv;
        *var_dirprime_dv_slot = var_dirprime_dv;
        *var_dirprime_dv_dn0_slot = var_dirprime_dv_dn0;
        *var_dirprime_dv_dn1_slot = var_dirprime_dv_dn1;
        *var_dirprime_dv_dn2_slot = var_dirprime_dv_dn2;
        *var_dirprime_dv_dn3_slot = var_dirprime_dv_dn3;
        *var_dirprime_dv_rv_slot = var_dirprime_dv_rv;
        *var_guard10_slot = var_guard10;
        *var_guard10_rv_slot = var_guard10_rv;
        *var_guard11_slot = var_guard11;
        *var_guard11_rv_slot = var_guard11_rv;
        *var_guard12_slot = var_guard12;
        *var_guard12_rv_slot = var_guard12_rv;
        *var_guard13_slot = var_guard13;
        *var_guard13_rv_slot = var_guard13_rv;
        *var_guard14_slot = var_guard14;
        *var_guard14_rv_slot = var_guard14_rv;
        *var_guard15_slot = var_guard15;
        *var_guard15_rv_slot = var_guard15_rv;
        *var_ir_slot = var_ir;
        *var_ir_dn0_slot = var_ir_dn0;
        *var_ir_dn1_slot = var_ir_dn1;
        *var_ir_dn2_slot = var_ir_dn2;
        *var_ir_dn3_slot = var_ir_dn3;
        *var_ir_rv_slot = var_ir_rv;
        *var_irprime_slot = var_irprime;
        *var_irprime_dn0_slot = var_irprime_dn0;
        *var_irprime_dn1_slot = var_irprime_dn1;
        *var_irprime_dn2_slot = var_irprime_dn2;
        *var_irprime_dn3_slot = var_irprime_dn3;
        *var_irprime_rv_slot = var_irprime_rv;
        *var_leq_slot = var_leq;
        *var_leq_dn0_slot = var_leq_dn0;
        *var_leq_dn1_slot = var_leq_dn1;
        *var_leq_dn2_slot = var_leq_dn2;
        *var_leq_dn3_slot = var_leq_dn3;
        *var_leq_rv_slot = var_leq_rv;
        *var_lmin_slot = var_lmin;
        *var_lmin_rv_slot = var_lmin_rv;
        *var_lprime_slot = var_lprime;
        *var_lprime_dn0_slot = var_lprime_dn0;
        *var_lprime_dn1_slot = var_lprime_dn1;
        *var_lprime_dn2_slot = var_lprime_dn2;
        *var_lprime_dn3_slot = var_lprime_dn3;
        *var_lprime_rv_slot = var_lprime_rv;
        *var_n_1_slot = var_n_1;
        *var_n_1_dn0_slot = var_n_1_dn0;
        *var_n_1_dn1_slot = var_n_1_dn1;
        *var_n_1_dn2_slot = var_n_1_dn2;
        *var_n_1_dn3_slot = var_n_1_dn3;
        *var_n_1_n_slot = var_n_1_n;
        *var_n_1_n_dn0_slot = var_n_1_n_dn0;
        *var_n_1_n_dn1_slot = var_n_1_n_dn1;
        *var_n_1_n_dn2_slot = var_n_1_n_dn2;
        *var_n_1_n_dn3_slot = var_n_1_n_dn3;
        *var_n_1_n_rv_slot = var_n_1_n_rv;
        *var_n_1_rv_slot = var_n_1_rv;
        *var_sif_slot = var_sif;
        *var_sif2_slot = var_sif2;
        *var_sif2_dn0_slot = var_sif2_dn0;
        *var_sif2_dn1_slot = var_sif2_dn1;
        *var_sif2_dn2_slot = var_sif2_dn2;
        *var_sif2_dn3_slot = var_sif2_dn3;
        *var_sif2_rv_slot = var_sif2_rv;
        *var_sif_dn0_slot = var_sif_dn0;
        *var_sif_dn1_slot = var_sif_dn1;
        *var_sif_dn2_slot = var_sif_dn2;
        *var_sif_dn3_slot = var_sif_dn3;
        *var_sif_rv_slot = var_sif_rv;
        *var_sif_sir_2_slot = var_sif_sir_2;
        *var_sif_sir_2_dn0_slot = var_sif_sir_2_dn0;
        *var_sif_sir_2_dn1_slot = var_sif_sir_2_dn1;
        *var_sif_sir_2_dn2_slot = var_sif_sir_2_dn2;
        *var_sif_sir_2_dn3_slot = var_sif_sir_2_dn3;
        *var_sif_sir_2_rv_slot = var_sif_sir_2_rv;
        *var_sir_slot = var_sir;
        *var_sir2_slot = var_sir2;
        *var_sir2_dn0_slot = var_sir2_dn0;
        *var_sir2_dn1_slot = var_sir2_dn1;
        *var_sir2_dn2_slot = var_sir2_dn2;
        *var_sir2_dn3_slot = var_sir2_dn3;
        *var_sir2_rv_slot = var_sir2_rv;
        *var_sir_dn0_slot = var_sir_dn0;
        *var_sir_dn1_slot = var_sir_dn1;
        *var_sir_dn2_slot = var_sir_dn2;
        *var_sir_dn3_slot = var_sir_dn3;
        *var_sir_rv_slot = var_sir_rv;
        *var_sqrt_lprime_lmin_slot = var_sqrt_lprime_lmin;
        *var_sqrt_lprime_lmin_dn0_slot = var_sqrt_lprime_lmin_dn0;
        *var_sqrt_lprime_lmin_dn1_slot = var_sqrt_lprime_lmin_dn1;
        *var_sqrt_lprime_lmin_dn2_slot = var_sqrt_lprime_lmin_dn2;
        *var_sqrt_lprime_lmin_dn3_slot = var_sqrt_lprime_lmin_dn3;
        *var_sqrt_lprime_lmin_rv_slot = var_sqrt_lprime_lmin_rv;
        *var_sqrt_phi_vp_2_slot = var_sqrt_phi_vp_2;
        *var_sqrt_phi_vp_2_dn0_slot = var_sqrt_phi_vp_2_dn0;
        *var_sqrt_phi_vp_2_dn1_slot = var_sqrt_phi_vp_2_dn1;
        *var_sqrt_phi_vp_2_dn2_slot = var_sqrt_phi_vp_2_dn2;
        *var_sqrt_phi_vp_2_dn3_slot = var_sqrt_phi_vp_2_dn3;
        *var_sqrt_phi_vp_2_rv_slot = var_sqrt_phi_vp_2_rv;
        *var_tmp1_slot = var_tmp1;
        *var_tmp1_dn0_slot = var_tmp1_dn0;
        *var_tmp1_dn1_slot = var_tmp1_dn1;
        *var_tmp1_dn2_slot = var_tmp1_dn2;
        *var_tmp1_dn3_slot = var_tmp1_dn3;
        *var_tmp1_rv_slot = var_tmp1_rv;
        *var_vp_phi_eps_slot = var_vp_phi_eps;
        *var_vp_phi_eps_dn0_slot = var_vp_phi_eps_dn0;
        *var_vp_phi_eps_dn1_slot = var_vp_phi_eps_dn1;
        *var_vp_phi_eps_dn2_slot = var_vp_phi_eps_dn2;
        *var_vp_phi_eps_dn3_slot = var_vp_phi_eps_dn3;
        *var_vp_phi_eps_rv_slot = var_vp_phi_eps_rv;
        *var_yk_slot = var_yk;
        *var_yk_dn0_slot = var_yk_dn0;
        *var_yk_dn1_slot = var_yk_dn1;
        *var_yk_dn2_slot = var_yk_dn2;
        *var_yk_dn3_slot = var_yk_dn3;
        *var_yk_rv_slot = var_yk_rv;
        *var_z0_slot = var_z0;
        *var_z0_dn0_slot = var_z0_dn0;
        *var_z0_dn1_slot = var_z0_dn1;
        *var_z0_dn2_slot = var_z0_dn2;
        *var_z0_dn3_slot = var_z0_dn3;
        *var_z0_rv_slot = var_z0_rv;
        *var_zk_slot = var_zk;
        *var_zk_dn0_slot = var_zk_dn0;
        *var_zk_dn1_slot = var_zk_dn1;
        *var_zk_dn2_slot = var_zk_dn2;
        *var_zk_dn3_slot = var_zk_dn3;
        *var_zk_rv_slot = var_zk_rv;
    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        var_big_sqrt_vp: f64,
        var_big_sqrt_vp_dn0: f64,
        var_big_sqrt_vp_dn1: f64,
        var_big_sqrt_vp_dn2: f64,
        var_big_sqrt_vp_dn3: f64,
        var_dif_dv: f64,
        var_dif_dv_dn0: f64,
        var_dif_dv_dn1: f64,
        var_dif_dv_dn2: f64,
        var_dif_dv_dn3: f64,
        var_dirprime_dv: f64,
        var_dirprime_dv_dn0: f64,
        var_dirprime_dv_dn1: f64,
        var_dirprime_dv_dn2: f64,
        var_dirprime_dv_dn3: f64,
        var_eta_qi: f64,
        var_gamma_s: f64,
        var_gamma_sqrt_phi: f64,
        var_gamma_sqrt_phi_dn0: f64,
        var_gamma_sqrt_phi_dn1: f64,
        var_gamma_sqrt_phi_dn2: f64,
        var_gamma_sqrt_phi_dn3: f64,
        var_gammaprime: f64,
        var_gammaprime_dn0: f64,
        var_gammaprime_dn1: f64,
        var_gammaprime_dn2: f64,
        var_gammaprime_dn3: f64,
        var_if_: f64,
        var_if__dn0: f64,
        var_if__dn1: f64,
        var_if__dn2: f64,
        var_if__dn3: f64,
        var_inv_vt: f64,
        var_irprime: f64,
        var_irprime_dn0: f64,
        var_irprime_dn1: f64,
        var_irprime_dn2: f64,
        var_irprime_dn3: f64,
        var_kp_weff: f64,
        var_leq: f64,
        var_leq_dn0: f64,
        var_leq_dn1: f64,
        var_leq_dn2: f64,
        var_leq_dn3: f64,
        var_leta_l: f64,
        var_n_1: f64,
        var_n_1_dn0: f64,
        var_n_1_dn1: f64,
        var_n_1_dn2: f64,
        var_n_1_dn3: f64,
        var_n_1_n: f64,
        var_n_1_n_dn0: f64,
        var_n_1_n_dn1: f64,
        var_n_1_n_dn2: f64,
        var_n_1_n_dn3: f64,
        var_phi_t: f64,
        var_phi_t_dn0: f64,
        var_phi_t_dn1: f64,
        var_phi_t_dn2: f64,
        var_phi_t_dn3: f64,
        var_sif: f64,
        var_sif2: f64,
        var_sif2_dn0: f64,
        var_sif2_dn1: f64,
        var_sif2_dn2: f64,
        var_sif2_dn3: f64,
        var_sif_dn0: f64,
        var_sif_dn1: f64,
        var_sif_dn2: f64,
        var_sif_dn3: f64,
        var_sir: f64,
        var_sir2: f64,
        var_sir2_dn0: f64,
        var_sir2_dn1: f64,
        var_sir2_dn2: f64,
        var_sir2_dn3: f64,
        var_sir_dn0: f64,
        var_sir_dn1: f64,
        var_sir_dn2: f64,
        var_sir_dn3: f64,
        var_sqrt_gammastar: f64,
        var_sqrt_gammastar_dn0: f64,
        var_sqrt_gammastar_dn1: f64,
        var_sqrt_gammastar_dn2: f64,
        var_sqrt_gammastar_dn3: f64,
        var_sqrt_if: f64,
        var_sqrt_if_dn0: f64,
        var_sqrt_if_dn1: f64,
        var_sqrt_if_dn2: f64,
        var_sqrt_if_dn3: f64,
        var_sqrt_phi_vd: f64,
        var_sqrt_phi_vd_dn0: f64,
        var_sqrt_phi_vd_dn1: f64,
        var_sqrt_phi_vd_dn2: f64,
        var_sqrt_phi_vd_dn3: f64,
        var_sqrt_phi_vd_vt: f64,
        var_sqrt_phi_vd_vt_dn0: f64,
        var_sqrt_phi_vd_vt_dn1: f64,
        var_sqrt_phi_vd_vt_dn2: f64,
        var_sqrt_phi_vd_vt_dn3: f64,
        var_sqrt_phi_vp_2: f64,
        var_sqrt_phi_vp_2_dn0: f64,
        var_sqrt_phi_vp_2_dn1: f64,
        var_sqrt_phi_vp_2_dn2: f64,
        var_sqrt_phi_vp_2_dn3: f64,
        var_sqrt_phi_vs: f64,
        var_sqrt_phi_vs_dn0: f64,
        var_sqrt_phi_vs_dn1: f64,
        var_sqrt_phi_vs_dn2: f64,
        var_sqrt_phi_vs_dn3: f64,
        var_sqrt_phi_vs_vt: f64,
        var_sqrt_phi_vs_vt_dn0: f64,
        var_sqrt_phi_vs_vt_dn1: f64,
        var_sqrt_phi_vs_vt_dn2: f64,
        var_sqrt_phi_vs_vt_dn3: f64,
        var_sqrt_vds_vdss_deltav: f64,
        var_sqrt_vds_vdss_deltav_dn0: f64,
        var_sqrt_vds_vdss_deltav_dn1: f64,
        var_sqrt_vds_vdss_deltav_dn2: f64,
        var_sqrt_vds_vdss_deltav_dn3: f64,
        var_sqrt_vds_vdssprime_deltav: f64,
        var_sqrt_vds_vdssprime_deltav_dn0: f64,
        var_sqrt_vds_vdssprime_deltav_dn1: f64,
        var_sqrt_vds_vdssprime_deltav_dn2: f64,
        var_sqrt_vds_vdssprime_deltav_dn3: f64,
        var_sqrt_vdss_deltav: f64,
        var_sqrt_vdss_deltav_dn0: f64,
        var_sqrt_vdss_deltav_dn1: f64,
        var_sqrt_vdss_deltav_dn2: f64,
        var_sqrt_vdss_deltav_dn3: f64,
        var_sqrt_vdssprime_deltav: f64,
        var_sqrt_vdssprime_deltav_dn0: f64,
        var_sqrt_vdssprime_deltav_dn1: f64,
        var_sqrt_vdssprime_deltav_dn2: f64,
        var_sqrt_vdssprime_deltav_dn3: f64,
        var_sqrt_vgstar: f64,
        var_sqrt_vgstar_dn0: f64,
        var_sqrt_vgstar_dn1: f64,
        var_sqrt_vgstar_dn2: f64,
        var_sqrt_vgstar_dn3: f64,
        var_t0: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn2: f64,
        var_vds_dn3: f64,
        var_vdsprime: f64,
        var_vdsprime_dn0: f64,
        var_vdsprime_dn1: f64,
        var_vdsprime_dn2: f64,
        var_vdsprime_dn3: f64,
        var_vdss: f64,
        var_vdss_dn0: f64,
        var_vdss_dn1: f64,
        var_vdss_dn2: f64,
        var_vdss_dn3: f64,
        var_vdss_sqrt: f64,
        var_vdss_sqrt_dn0: f64,
        var_vdss_sqrt_dn1: f64,
        var_vdss_sqrt_dn2: f64,
        var_vdss_sqrt_dn3: f64,
        var_vdssprime: f64,
        var_vdssprime_dn0: f64,
        var_vdssprime_dn1: f64,
        var_vdssprime_dn2: f64,
        var_vdssprime_dn3: f64,
        var_vdssprime_sqrt: f64,
        var_vdssprime_sqrt_dn0: f64,
        var_vdssprime_sqrt_dn1: f64,
        var_vdssprime_sqrt_dn2: f64,
        var_vdssprime_sqrt_dn3: f64,
        var_vgprime: f64,
        var_vgprime_dn0: f64,
        var_vgprime_dn1: f64,
        var_vgprime_dn2: f64,
        var_vgprime_dn3: f64,
        var_vp: f64,
        var_vp_dn0: f64,
        var_vp_dn1: f64,
        var_vp_dn2: f64,
        var_vp_dn3: f64,
        var_vt: f64,
        var_vt_4: f64,
        var_vt_vt_2: f64,
        var_beta_slot: &mut f64,
        var_beta_dn0_slot: &mut f64,
        var_beta_dn1_slot: &mut f64,
        var_beta_dn2_slot: &mut f64,
        var_beta_dn3_slot: &mut f64,
        var_beta_rv_slot: &mut f64,
        var_ddeltav_dvd_slot: &mut f64,
        var_ddeltav_dvd_dn0_slot: &mut f64,
        var_ddeltav_dvd_dn1_slot: &mut f64,
        var_ddeltav_dvd_dn2_slot: &mut f64,
        var_ddeltav_dvd_dn3_slot: &mut f64,
        var_ddeltav_dvd_rv_slot: &mut f64,
        var_ddeltav_dvs_slot: &mut f64,
        var_ddeltav_dvs_dn0_slot: &mut f64,
        var_ddeltav_dvs_dn1_slot: &mut f64,
        var_ddeltav_dvs_dn2_slot: &mut f64,
        var_ddeltav_dvs_dn3_slot: &mut f64,
        var_ddeltav_dvs_rv_slot: &mut f64,
        var_dgammaprime_dvd_slot: &mut f64,
        var_dgammaprime_dvd_dn0_slot: &mut f64,
        var_dgammaprime_dvd_dn1_slot: &mut f64,
        var_dgammaprime_dvd_dn2_slot: &mut f64,
        var_dgammaprime_dvd_dn3_slot: &mut f64,
        var_dgammaprime_dvd_rv_slot: &mut f64,
        var_dgammaprime_dvs_slot: &mut f64,
        var_dgammaprime_dvs_dn0_slot: &mut f64,
        var_dgammaprime_dvs_dn1_slot: &mut f64,
        var_dgammaprime_dvs_dn2_slot: &mut f64,
        var_dgammaprime_dvs_dn3_slot: &mut f64,
        var_dgammaprime_dvs_rv_slot: &mut f64,
        var_dif_dvd_slot: &mut f64,
        var_dif_dvd_dn0_slot: &mut f64,
        var_dif_dvd_dn1_slot: &mut f64,
        var_dif_dvd_dn2_slot: &mut f64,
        var_dif_dvd_dn3_slot: &mut f64,
        var_dif_dvd_rv_slot: &mut f64,
        var_dif_dvs_slot: &mut f64,
        var_dif_dvs_dn0_slot: &mut f64,
        var_dif_dvs_dn1_slot: &mut f64,
        var_dif_dvs_dn2_slot: &mut f64,
        var_dif_dvs_dn3_slot: &mut f64,
        var_dif_dvs_rv_slot: &mut f64,
        var_dirprime_dvd_slot: &mut f64,
        var_dirprime_dvd_dn0_slot: &mut f64,
        var_dirprime_dvd_dn1_slot: &mut f64,
        var_dirprime_dvd_dn2_slot: &mut f64,
        var_dirprime_dvd_dn3_slot: &mut f64,
        var_dirprime_dvd_rv_slot: &mut f64,
        var_dirprime_dvs_slot: &mut f64,
        var_dirprime_dvs_dn0_slot: &mut f64,
        var_dirprime_dvs_dn1_slot: &mut f64,
        var_dirprime_dvs_dn2_slot: &mut f64,
        var_dirprime_dvs_dn3_slot: &mut f64,
        var_dirprime_dvs_rv_slot: &mut f64,
        var_dvdss_dvd_slot: &mut f64,
        var_dvdss_dvd_dn0_slot: &mut f64,
        var_dvdss_dvd_dn1_slot: &mut f64,
        var_dvdss_dvd_dn2_slot: &mut f64,
        var_dvdss_dvd_dn3_slot: &mut f64,
        var_dvdss_dvd_rv_slot: &mut f64,
        var_dvdss_dvs_slot: &mut f64,
        var_dvdss_dvs_dn0_slot: &mut f64,
        var_dvdss_dvs_dn1_slot: &mut f64,
        var_dvdss_dvs_dn2_slot: &mut f64,
        var_dvdss_dvs_dn3_slot: &mut f64,
        var_dvdss_dvs_rv_slot: &mut f64,
        var_dvdssprime_dvd_slot: &mut f64,
        var_dvdssprime_dvd_dn0_slot: &mut f64,
        var_dvdssprime_dvd_dn1_slot: &mut f64,
        var_dvdssprime_dvd_dn2_slot: &mut f64,
        var_dvdssprime_dvd_dn3_slot: &mut f64,
        var_dvdssprime_dvd_rv_slot: &mut f64,
        var_dvdssprime_dvs_slot: &mut f64,
        var_dvdssprime_dvs_dn0_slot: &mut f64,
        var_dvdssprime_dvs_dn1_slot: &mut f64,
        var_dvdssprime_dvs_dn2_slot: &mut f64,
        var_dvdssprime_dvs_dn3_slot: &mut f64,
        var_dvdssprime_dvs_rv_slot: &mut f64,
        var_dvip_dvd_slot: &mut f64,
        var_dvip_dvd_dn0_slot: &mut f64,
        var_dvip_dvd_dn1_slot: &mut f64,
        var_dvip_dvd_dn2_slot: &mut f64,
        var_dvip_dvd_dn3_slot: &mut f64,
        var_dvip_dvd_rv_slot: &mut f64,
        var_dvip_dvs_slot: &mut f64,
        var_dvip_dvs_dn0_slot: &mut f64,
        var_dvip_dvs_dn1_slot: &mut f64,
        var_dvip_dvs_dn2_slot: &mut f64,
        var_dvip_dvs_dn3_slot: &mut f64,
        var_dvip_dvs_rv_slot: &mut f64,
        var_dvp_dvd_slot: &mut f64,
        var_dvp_dvd_dn0_slot: &mut f64,
        var_dvp_dvd_dn1_slot: &mut f64,
        var_dvp_dvd_dn2_slot: &mut f64,
        var_dvp_dvd_dn3_slot: &mut f64,
        var_dvp_dvd_rv_slot: &mut f64,
        var_dvp_dvs_slot: &mut f64,
        var_dvp_dvs_dn0_slot: &mut f64,
        var_dvp_dvs_dn1_slot: &mut f64,
        var_dvp_dvs_dn2_slot: &mut f64,
        var_dvp_dvs_dn3_slot: &mut f64,
        var_dvp_dvs_rv_slot: &mut f64,
        var_e0_q_1_slot: &mut f64,
        var_e0_q_1_dn0_slot: &mut f64,
        var_e0_q_1_dn1_slot: &mut f64,
        var_e0_q_1_dn2_slot: &mut f64,
        var_e0_q_1_dn3_slot: &mut f64,
        var_e0_q_1_rv_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard16_rv_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard17_rv_slot: &mut f64,
        var_if_ir_slot: &mut f64,
        var_if_ir_dn0_slot: &mut f64,
        var_if_ir_dn1_slot: &mut f64,
        var_if_ir_dn2_slot: &mut f64,
        var_if_ir_dn3_slot: &mut f64,
        var_if_ir_rv_slot: &mut f64,
        var_ispec_slot: &mut f64,
        var_ispec_dn0_slot: &mut f64,
        var_ispec_dn1_slot: &mut f64,
        var_ispec_dn2_slot: &mut f64,
        var_ispec_dn3_slot: &mut f64,
        var_ispec_rv_slot: &mut f64,
        var_n_slot: &mut f64,
        var_n_dn0_slot: &mut f64,
        var_n_dn1_slot: &mut f64,
        var_n_dn2_slot: &mut f64,
        var_n_dn3_slot: &mut f64,
        var_n_rv_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn0_slot: &mut f64,
        var_qb_dn1_slot: &mut f64,
        var_qb_dn2_slot: &mut f64,
        var_qb_dn3_slot: &mut f64,
        var_qb_rv_slot: &mut f64,
        var_qi_slot: &mut f64,
        var_qi_dn0_slot: &mut f64,
        var_qi_dn1_slot: &mut f64,
        var_qi_dn2_slot: &mut f64,
        var_qi_dn3_slot: &mut f64,
        var_qi_rv_slot: &mut f64,
        var_sqrt_phi_vp_slot: &mut f64,
        var_sqrt_phi_vp_dn0_slot: &mut f64,
        var_sqrt_phi_vp_dn1_slot: &mut f64,
        var_sqrt_phi_vp_dn2_slot: &mut f64,
        var_sqrt_phi_vp_dn3_slot: &mut f64,
        var_sqrt_phi_vp_rv_slot: &mut f64,
        var_sqrt_vp_vt_slot: &mut f64,
        var_sqrt_vp_vt_dn0_slot: &mut f64,
        var_sqrt_vp_vt_dn1_slot: &mut f64,
        var_sqrt_vp_vt_dn2_slot: &mut f64,
        var_sqrt_vp_vt_dn3_slot: &mut f64,
        var_sqrt_vp_vt_rv_slot: &mut f64,
        var_t0_gamma_1_slot: &mut f64,
        var_t0_gamma_1_dn0_slot: &mut f64,
        var_t0_gamma_1_dn1_slot: &mut f64,
        var_t0_gamma_1_dn2_slot: &mut f64,
        var_t0_gamma_1_dn3_slot: &mut f64,
        var_t0_gamma_1_rv_slot: &mut f64,
        var_theta_vp_1_slot: &mut f64,
        var_theta_vp_1_dn0_slot: &mut f64,
        var_theta_vp_1_dn1_slot: &mut f64,
        var_theta_vp_1_dn2_slot: &mut f64,
        var_theta_vp_1_dn3_slot: &mut f64,
        var_theta_vp_1_rv_slot: &mut f64,
        var_tmp1_slot: &mut f64,
        var_tmp1_dn0_slot: &mut f64,
        var_tmp1_dn1_slot: &mut f64,
        var_tmp1_dn2_slot: &mut f64,
        var_tmp1_dn3_slot: &mut f64,
        var_tmp1_rv_slot: &mut f64,
        var_tmp2_slot: &mut f64,
        var_tmp2_dn0_slot: &mut f64,
        var_tmp2_dn1_slot: &mut f64,
        var_tmp2_dn2_slot: &mut f64,
        var_tmp2_dn3_slot: &mut f64,
        var_tmp2_rv_slot: &mut f64,
        var_tmp3_slot: &mut f64,
        var_tmp3_dn0_slot: &mut f64,
        var_tmp3_dn1_slot: &mut f64,
        var_tmp3_dn2_slot: &mut f64,
        var_tmp3_dn3_slot: &mut f64,
        var_tmp3_rv_slot: &mut f64,
        var_vpprime_slot: &mut f64,
        var_vpprime_dn0_slot: &mut f64,
        var_vpprime_dn1_slot: &mut f64,
        var_vpprime_dn2_slot: &mut f64,
        var_vpprime_dn3_slot: &mut f64,
        var_vpprime_rv_slot: &mut f64,
    ) {
        let mut var_beta: f64 = *var_beta_slot;
        let mut var_beta_dn0: f64 = *var_beta_dn0_slot;
        let mut var_beta_dn1: f64 = *var_beta_dn1_slot;
        let mut var_beta_dn2: f64 = *var_beta_dn2_slot;
        let mut var_beta_dn3: f64 = *var_beta_dn3_slot;
        let mut var_beta_rv: f64 = *var_beta_rv_slot;
        let mut var_ddeltav_dvd: f64 = *var_ddeltav_dvd_slot;
        let mut var_ddeltav_dvd_dn0: f64 = *var_ddeltav_dvd_dn0_slot;
        let mut var_ddeltav_dvd_dn1: f64 = *var_ddeltav_dvd_dn1_slot;
        let mut var_ddeltav_dvd_dn2: f64 = *var_ddeltav_dvd_dn2_slot;
        let mut var_ddeltav_dvd_dn3: f64 = *var_ddeltav_dvd_dn3_slot;
        let mut var_ddeltav_dvd_rv: f64 = *var_ddeltav_dvd_rv_slot;
        let mut var_ddeltav_dvs: f64 = *var_ddeltav_dvs_slot;
        let mut var_ddeltav_dvs_dn0: f64 = *var_ddeltav_dvs_dn0_slot;
        let mut var_ddeltav_dvs_dn1: f64 = *var_ddeltav_dvs_dn1_slot;
        let mut var_ddeltav_dvs_dn2: f64 = *var_ddeltav_dvs_dn2_slot;
        let mut var_ddeltav_dvs_dn3: f64 = *var_ddeltav_dvs_dn3_slot;
        let mut var_ddeltav_dvs_rv: f64 = *var_ddeltav_dvs_rv_slot;
        let mut var_dgammaprime_dvd: f64 = *var_dgammaprime_dvd_slot;
        let mut var_dgammaprime_dvd_dn0: f64 = *var_dgammaprime_dvd_dn0_slot;
        let mut var_dgammaprime_dvd_dn1: f64 = *var_dgammaprime_dvd_dn1_slot;
        let mut var_dgammaprime_dvd_dn2: f64 = *var_dgammaprime_dvd_dn2_slot;
        let mut var_dgammaprime_dvd_dn3: f64 = *var_dgammaprime_dvd_dn3_slot;
        let mut var_dgammaprime_dvd_rv: f64 = *var_dgammaprime_dvd_rv_slot;
        let mut var_dgammaprime_dvs: f64 = *var_dgammaprime_dvs_slot;
        let mut var_dgammaprime_dvs_dn0: f64 = *var_dgammaprime_dvs_dn0_slot;
        let mut var_dgammaprime_dvs_dn1: f64 = *var_dgammaprime_dvs_dn1_slot;
        let mut var_dgammaprime_dvs_dn2: f64 = *var_dgammaprime_dvs_dn2_slot;
        let mut var_dgammaprime_dvs_dn3: f64 = *var_dgammaprime_dvs_dn3_slot;
        let mut var_dgammaprime_dvs_rv: f64 = *var_dgammaprime_dvs_rv_slot;
        let mut var_dif_dvd: f64 = *var_dif_dvd_slot;
        let mut var_dif_dvd_dn0: f64 = *var_dif_dvd_dn0_slot;
        let mut var_dif_dvd_dn1: f64 = *var_dif_dvd_dn1_slot;
        let mut var_dif_dvd_dn2: f64 = *var_dif_dvd_dn2_slot;
        let mut var_dif_dvd_dn3: f64 = *var_dif_dvd_dn3_slot;
        let mut var_dif_dvd_rv: f64 = *var_dif_dvd_rv_slot;
        let mut var_dif_dvs: f64 = *var_dif_dvs_slot;
        let mut var_dif_dvs_dn0: f64 = *var_dif_dvs_dn0_slot;
        let mut var_dif_dvs_dn1: f64 = *var_dif_dvs_dn1_slot;
        let mut var_dif_dvs_dn2: f64 = *var_dif_dvs_dn2_slot;
        let mut var_dif_dvs_dn3: f64 = *var_dif_dvs_dn3_slot;
        let mut var_dif_dvs_rv: f64 = *var_dif_dvs_rv_slot;
        let mut var_dirprime_dvd: f64 = *var_dirprime_dvd_slot;
        let mut var_dirprime_dvd_dn0: f64 = *var_dirprime_dvd_dn0_slot;
        let mut var_dirprime_dvd_dn1: f64 = *var_dirprime_dvd_dn1_slot;
        let mut var_dirprime_dvd_dn2: f64 = *var_dirprime_dvd_dn2_slot;
        let mut var_dirprime_dvd_dn3: f64 = *var_dirprime_dvd_dn3_slot;
        let mut var_dirprime_dvd_rv: f64 = *var_dirprime_dvd_rv_slot;
        let mut var_dirprime_dvs: f64 = *var_dirprime_dvs_slot;
        let mut var_dirprime_dvs_dn0: f64 = *var_dirprime_dvs_dn0_slot;
        let mut var_dirprime_dvs_dn1: f64 = *var_dirprime_dvs_dn1_slot;
        let mut var_dirprime_dvs_dn2: f64 = *var_dirprime_dvs_dn2_slot;
        let mut var_dirprime_dvs_dn3: f64 = *var_dirprime_dvs_dn3_slot;
        let mut var_dirprime_dvs_rv: f64 = *var_dirprime_dvs_rv_slot;
        let mut var_dvdss_dvd: f64 = *var_dvdss_dvd_slot;
        let mut var_dvdss_dvd_dn0: f64 = *var_dvdss_dvd_dn0_slot;
        let mut var_dvdss_dvd_dn1: f64 = *var_dvdss_dvd_dn1_slot;
        let mut var_dvdss_dvd_dn2: f64 = *var_dvdss_dvd_dn2_slot;
        let mut var_dvdss_dvd_dn3: f64 = *var_dvdss_dvd_dn3_slot;
        let mut var_dvdss_dvd_rv: f64 = *var_dvdss_dvd_rv_slot;
        let mut var_dvdss_dvs: f64 = *var_dvdss_dvs_slot;
        let mut var_dvdss_dvs_dn0: f64 = *var_dvdss_dvs_dn0_slot;
        let mut var_dvdss_dvs_dn1: f64 = *var_dvdss_dvs_dn1_slot;
        let mut var_dvdss_dvs_dn2: f64 = *var_dvdss_dvs_dn2_slot;
        let mut var_dvdss_dvs_dn3: f64 = *var_dvdss_dvs_dn3_slot;
        let mut var_dvdss_dvs_rv: f64 = *var_dvdss_dvs_rv_slot;
        let mut var_dvdssprime_dvd: f64 = *var_dvdssprime_dvd_slot;
        let mut var_dvdssprime_dvd_dn0: f64 = *var_dvdssprime_dvd_dn0_slot;
        let mut var_dvdssprime_dvd_dn1: f64 = *var_dvdssprime_dvd_dn1_slot;
        let mut var_dvdssprime_dvd_dn2: f64 = *var_dvdssprime_dvd_dn2_slot;
        let mut var_dvdssprime_dvd_dn3: f64 = *var_dvdssprime_dvd_dn3_slot;
        let mut var_dvdssprime_dvd_rv: f64 = *var_dvdssprime_dvd_rv_slot;
        let mut var_dvdssprime_dvs: f64 = *var_dvdssprime_dvs_slot;
        let mut var_dvdssprime_dvs_dn0: f64 = *var_dvdssprime_dvs_dn0_slot;
        let mut var_dvdssprime_dvs_dn1: f64 = *var_dvdssprime_dvs_dn1_slot;
        let mut var_dvdssprime_dvs_dn2: f64 = *var_dvdssprime_dvs_dn2_slot;
        let mut var_dvdssprime_dvs_dn3: f64 = *var_dvdssprime_dvs_dn3_slot;
        let mut var_dvdssprime_dvs_rv: f64 = *var_dvdssprime_dvs_rv_slot;
        let mut var_dvip_dvd: f64 = *var_dvip_dvd_slot;
        let mut var_dvip_dvd_dn0: f64 = *var_dvip_dvd_dn0_slot;
        let mut var_dvip_dvd_dn1: f64 = *var_dvip_dvd_dn1_slot;
        let mut var_dvip_dvd_dn2: f64 = *var_dvip_dvd_dn2_slot;
        let mut var_dvip_dvd_dn3: f64 = *var_dvip_dvd_dn3_slot;
        let mut var_dvip_dvd_rv: f64 = *var_dvip_dvd_rv_slot;
        let mut var_dvip_dvs: f64 = *var_dvip_dvs_slot;
        let mut var_dvip_dvs_dn0: f64 = *var_dvip_dvs_dn0_slot;
        let mut var_dvip_dvs_dn1: f64 = *var_dvip_dvs_dn1_slot;
        let mut var_dvip_dvs_dn2: f64 = *var_dvip_dvs_dn2_slot;
        let mut var_dvip_dvs_dn3: f64 = *var_dvip_dvs_dn3_slot;
        let mut var_dvip_dvs_rv: f64 = *var_dvip_dvs_rv_slot;
        let mut var_dvp_dvd: f64 = *var_dvp_dvd_slot;
        let mut var_dvp_dvd_dn0: f64 = *var_dvp_dvd_dn0_slot;
        let mut var_dvp_dvd_dn1: f64 = *var_dvp_dvd_dn1_slot;
        let mut var_dvp_dvd_dn2: f64 = *var_dvp_dvd_dn2_slot;
        let mut var_dvp_dvd_dn3: f64 = *var_dvp_dvd_dn3_slot;
        let mut var_dvp_dvd_rv: f64 = *var_dvp_dvd_rv_slot;
        let mut var_dvp_dvs: f64 = *var_dvp_dvs_slot;
        let mut var_dvp_dvs_dn0: f64 = *var_dvp_dvs_dn0_slot;
        let mut var_dvp_dvs_dn1: f64 = *var_dvp_dvs_dn1_slot;
        let mut var_dvp_dvs_dn2: f64 = *var_dvp_dvs_dn2_slot;
        let mut var_dvp_dvs_dn3: f64 = *var_dvp_dvs_dn3_slot;
        let mut var_dvp_dvs_rv: f64 = *var_dvp_dvs_rv_slot;
        let mut var_e0_q_1: f64 = *var_e0_q_1_slot;
        let mut var_e0_q_1_dn0: f64 = *var_e0_q_1_dn0_slot;
        let mut var_e0_q_1_dn1: f64 = *var_e0_q_1_dn1_slot;
        let mut var_e0_q_1_dn2: f64 = *var_e0_q_1_dn2_slot;
        let mut var_e0_q_1_dn3: f64 = *var_e0_q_1_dn3_slot;
        let mut var_e0_q_1_rv: f64 = *var_e0_q_1_rv_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard16_rv: f64 = *var_guard16_rv_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard17_rv: f64 = *var_guard17_rv_slot;
        let mut var_if_ir: f64 = *var_if_ir_slot;
        let mut var_if_ir_dn0: f64 = *var_if_ir_dn0_slot;
        let mut var_if_ir_dn1: f64 = *var_if_ir_dn1_slot;
        let mut var_if_ir_dn2: f64 = *var_if_ir_dn2_slot;
        let mut var_if_ir_dn3: f64 = *var_if_ir_dn3_slot;
        let mut var_if_ir_rv: f64 = *var_if_ir_rv_slot;
        let mut var_ispec: f64 = *var_ispec_slot;
        let mut var_ispec_dn0: f64 = *var_ispec_dn0_slot;
        let mut var_ispec_dn1: f64 = *var_ispec_dn1_slot;
        let mut var_ispec_dn2: f64 = *var_ispec_dn2_slot;
        let mut var_ispec_dn3: f64 = *var_ispec_dn3_slot;
        let mut var_ispec_rv: f64 = *var_ispec_rv_slot;
        let mut var_n: f64 = *var_n_slot;
        let mut var_n_dn0: f64 = *var_n_dn0_slot;
        let mut var_n_dn1: f64 = *var_n_dn1_slot;
        let mut var_n_dn2: f64 = *var_n_dn2_slot;
        let mut var_n_dn3: f64 = *var_n_dn3_slot;
        let mut var_n_rv: f64 = *var_n_rv_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn0: f64 = *var_qb_dn0_slot;
        let mut var_qb_dn1: f64 = *var_qb_dn1_slot;
        let mut var_qb_dn2: f64 = *var_qb_dn2_slot;
        let mut var_qb_dn3: f64 = *var_qb_dn3_slot;
        let mut var_qb_rv: f64 = *var_qb_rv_slot;
        let mut var_qi: f64 = *var_qi_slot;
        let mut var_qi_dn0: f64 = *var_qi_dn0_slot;
        let mut var_qi_dn1: f64 = *var_qi_dn1_slot;
        let mut var_qi_dn2: f64 = *var_qi_dn2_slot;
        let mut var_qi_dn3: f64 = *var_qi_dn3_slot;
        let mut var_qi_rv: f64 = *var_qi_rv_slot;
        let mut var_sqrt_phi_vp: f64 = *var_sqrt_phi_vp_slot;
        let mut var_sqrt_phi_vp_dn0: f64 = *var_sqrt_phi_vp_dn0_slot;
        let mut var_sqrt_phi_vp_dn1: f64 = *var_sqrt_phi_vp_dn1_slot;
        let mut var_sqrt_phi_vp_dn2: f64 = *var_sqrt_phi_vp_dn2_slot;
        let mut var_sqrt_phi_vp_dn3: f64 = *var_sqrt_phi_vp_dn3_slot;
        let mut var_sqrt_phi_vp_rv: f64 = *var_sqrt_phi_vp_rv_slot;
        let mut var_sqrt_vp_vt: f64 = *var_sqrt_vp_vt_slot;
        let mut var_sqrt_vp_vt_dn0: f64 = *var_sqrt_vp_vt_dn0_slot;
        let mut var_sqrt_vp_vt_dn1: f64 = *var_sqrt_vp_vt_dn1_slot;
        let mut var_sqrt_vp_vt_dn2: f64 = *var_sqrt_vp_vt_dn2_slot;
        let mut var_sqrt_vp_vt_dn3: f64 = *var_sqrt_vp_vt_dn3_slot;
        let mut var_sqrt_vp_vt_rv: f64 = *var_sqrt_vp_vt_rv_slot;
        let mut var_t0_gamma_1: f64 = *var_t0_gamma_1_slot;
        let mut var_t0_gamma_1_dn0: f64 = *var_t0_gamma_1_dn0_slot;
        let mut var_t0_gamma_1_dn1: f64 = *var_t0_gamma_1_dn1_slot;
        let mut var_t0_gamma_1_dn2: f64 = *var_t0_gamma_1_dn2_slot;
        let mut var_t0_gamma_1_dn3: f64 = *var_t0_gamma_1_dn3_slot;
        let mut var_t0_gamma_1_rv: f64 = *var_t0_gamma_1_rv_slot;
        let mut var_theta_vp_1: f64 = *var_theta_vp_1_slot;
        let mut var_theta_vp_1_dn0: f64 = *var_theta_vp_1_dn0_slot;
        let mut var_theta_vp_1_dn1: f64 = *var_theta_vp_1_dn1_slot;
        let mut var_theta_vp_1_dn2: f64 = *var_theta_vp_1_dn2_slot;
        let mut var_theta_vp_1_dn3: f64 = *var_theta_vp_1_dn3_slot;
        let mut var_theta_vp_1_rv: f64 = *var_theta_vp_1_rv_slot;
        let mut var_tmp1: f64 = *var_tmp1_slot;
        let mut var_tmp1_dn0: f64 = *var_tmp1_dn0_slot;
        let mut var_tmp1_dn1: f64 = *var_tmp1_dn1_slot;
        let mut var_tmp1_dn2: f64 = *var_tmp1_dn2_slot;
        let mut var_tmp1_dn3: f64 = *var_tmp1_dn3_slot;
        let mut var_tmp1_rv: f64 = *var_tmp1_rv_slot;
        let mut var_tmp2: f64 = *var_tmp2_slot;
        let mut var_tmp2_dn0: f64 = *var_tmp2_dn0_slot;
        let mut var_tmp2_dn1: f64 = *var_tmp2_dn1_slot;
        let mut var_tmp2_dn2: f64 = *var_tmp2_dn2_slot;
        let mut var_tmp2_dn3: f64 = *var_tmp2_dn3_slot;
        let mut var_tmp2_rv: f64 = *var_tmp2_rv_slot;
        let mut var_tmp3: f64 = *var_tmp3_slot;
        let mut var_tmp3_dn0: f64 = *var_tmp3_dn0_slot;
        let mut var_tmp3_dn1: f64 = *var_tmp3_dn1_slot;
        let mut var_tmp3_dn2: f64 = *var_tmp3_dn2_slot;
        let mut var_tmp3_dn3: f64 = *var_tmp3_dn3_slot;
        let mut var_tmp3_rv: f64 = *var_tmp3_rv_slot;
        let mut var_vpprime: f64 = *var_vpprime_slot;
        let mut var_vpprime_dn0: f64 = *var_vpprime_dn0_slot;
        let mut var_vpprime_dn1: f64 = *var_vpprime_dn1_slot;
        let mut var_vpprime_dn2: f64 = *var_vpprime_dn2_slot;
        let mut var_vpprime_dn3: f64 = *var_vpprime_dn3_slot;
        let mut var_vpprime_rv: f64 = *var_vpprime_rv_slot;

        let assign1620_e1265: f64 = (1.0 + var_n_1);
        let assign1620_e1266: f64 = (-assign1620_e1265);
        let assign1620_e1268: f64 = (assign1620_e1266 * var_vt);
        let assign1620_e1271: f64 = (0.66666666 + 0.66666666);
        let assign1620_e1275: f64 = (var_sir * var_sif);
        let assign1620_e1276: f64 = (var_sir2 + assign1620_e1275);
        let assign1620_e1278: f64 = (assign1620_e1276 + var_sif2);
        let assign1620_e1279: f64 = (assign1620_e1271 * assign1620_e1278);
        let assign1620_e1282: f64 = (var_sif + var_sir);
        let assign1620_e1283: f64 = (assign1620_e1279 / assign1620_e1282);
        let assign1620_e1285: f64 = (assign1620_e1283 - 1.0);
        let assign1620_e1286: f64 = (assign1620_e1268 * assign1620_e1285);
        var_qi = assign1620_e1286;
        var_qi_dn0 = ((((-var_n_1_dn0) * var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((var_sir2_dn0 + ((var_sir_dn0 * var_sif) + (var_sir * var_sif_dn0))) + var_sif2_dn0)) * assign1620_e1282) - (assign1620_e1279 * (var_sif_dn0 + var_sir_dn0))) / (assign1620_e1282 * assign1620_e1282))));
        var_qi_dn1 = ((((-var_n_1_dn1) * var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((var_sir2_dn1 + ((var_sir_dn1 * var_sif) + (var_sir * var_sif_dn1))) + var_sif2_dn1)) * assign1620_e1282) - (assign1620_e1279 * (var_sif_dn1 + var_sir_dn1))) / (assign1620_e1282 * assign1620_e1282))));
        var_qi_dn2 = ((((-var_n_1_dn2) * var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((var_sir2_dn2 + ((var_sir_dn2 * var_sif) + (var_sir * var_sif_dn2))) + var_sif2_dn2)) * assign1620_e1282) - (assign1620_e1279 * (var_sif_dn2 + var_sir_dn2))) / (assign1620_e1282 * assign1620_e1282))));
        var_qi_dn3 = ((((-var_n_1_dn3) * var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((var_sir2_dn3 + ((var_sir_dn3 * var_sif) + (var_sir * var_sif_dn3))) + var_sif2_dn3)) * assign1620_e1282) - (assign1620_e1279 * (var_sif_dn3 + var_sir_dn3))) / (assign1620_e1282 * assign1620_e1282))));
        var_qi_rv = 0.0;

        let assign1630_e1288: f64 = (-0.5);
        let assign1630_e1290: f64 = (assign1630_e1288 * var_gamma_s);
        let assign1630_e1292: f64 = (assign1630_e1290 * var_sqrt_phi_vp_2);
        let assign1630_e1295: f64 = (var_n_1_n * var_qi);
        let assign1630_e1296: f64 = (assign1630_e1292 - assign1630_e1295);
        var_qb = assign1630_e1296;
        var_qb_dn0 = ((assign1630_e1290 * var_sqrt_phi_vp_2_dn0) - ((var_n_1_n_dn0 * var_qi) + (var_n_1_n * var_qi_dn0)));
        var_qb_dn1 = ((assign1630_e1290 * var_sqrt_phi_vp_2_dn1) - ((var_n_1_n_dn1 * var_qi) + (var_n_1_n * var_qi_dn1)));
        var_qb_dn2 = ((assign1630_e1290 * var_sqrt_phi_vp_2_dn2) - ((var_n_1_n_dn2 * var_qi) + (var_n_1_n * var_qi_dn2)));
        var_qb_dn3 = ((assign1630_e1290 * var_sqrt_phi_vp_2_dn3) - ((var_n_1_n_dn3 * var_qi) + (var_n_1_n * var_qi_dn3)));
        var_qb_rv = 0.0;

        let assign1640_e1299: f64 = if p.p22 == 0.0 { 1.0 } else { 0.0 };
        var_guard16 = assign1640_e1299;
        var_guard16_rv = 0.0;

        let (assign1650_e1308, assign1650_e1308_d_n0, assign1650_e1308_d_n1, assign1650_e1308_d_n2, assign1650_e1308_d_n3,) = {
    if (var_guard16 != 0.0) {
        let assign1650_e1303: f64 = (var_vp * var_vp);
        let assign1650_e1305: f64 = (assign1650_e1303 + var_vt_vt_2);
        let assign1650_e1306: f64 = (assign1650_e1305).sqrt();
        (assign1650_e1306, (((var_vp_dn0 * var_vp) + (var_vp * var_vp_dn0)) / (2.0 * assign1650_e1306)), (((var_vp_dn1 * var_vp) + (var_vp * var_vp_dn1)) / (2.0 * assign1650_e1306)), (((var_vp_dn2 * var_vp) + (var_vp * var_vp_dn2)) / (2.0 * assign1650_e1306)), (((var_vp_dn3 * var_vp) + (var_vp * var_vp_dn3)) / (2.0 * assign1650_e1306)),)
    } else {
        (var_sqrt_vp_vt, var_sqrt_vp_vt_dn0, var_sqrt_vp_vt_dn1, var_sqrt_vp_vt_dn2, var_sqrt_vp_vt_dn3,)
    }
};
        var_sqrt_vp_vt = assign1650_e1308;
        var_sqrt_vp_vt_dn0 = assign1650_e1308_d_n0;
        var_sqrt_vp_vt_dn1 = assign1650_e1308_d_n1;
        var_sqrt_vp_vt_dn2 = assign1650_e1308_d_n2;
        var_sqrt_vp_vt_dn3 = assign1650_e1308_d_n3;
        var_sqrt_vp_vt_rv = 0.0;

        let (assign1660_e1316, assign1660_e1316_d_n0, assign1660_e1316_d_n1, assign1660_e1316_d_n2, assign1660_e1316_d_n3,) = {
    if (var_guard16 != 0.0) {
        let assign1660_e1313: f64 = (var_vp + var_sqrt_vp_vt);
        let assign1660_e1314: f64 = (0.5 * assign1660_e1313);
        (assign1660_e1314, (0.5 * (var_vp_dn0 + var_sqrt_vp_vt_dn0)), (0.5 * (var_vp_dn1 + var_sqrt_vp_vt_dn1)), (0.5 * (var_vp_dn2 + var_sqrt_vp_vt_dn2)), (0.5 * (var_vp_dn3 + var_sqrt_vp_vt_dn3)),)
    } else {
        (var_vpprime, var_vpprime_dn0, var_vpprime_dn1, var_vpprime_dn2, var_vpprime_dn3,)
    }
};
        var_vpprime = assign1660_e1316;
        var_vpprime_dn0 = assign1660_e1316_d_n0;
        var_vpprime_dn1 = assign1660_e1316_d_n1;
        var_vpprime_dn2 = assign1660_e1316_d_n2;
        var_vpprime_dn3 = assign1660_e1316_d_n3;
        var_vpprime_rv = 0.0;

        let (assign1670_e1324, assign1670_e1324_d_n0, assign1670_e1324_d_n1, assign1670_e1324_d_n2, assign1670_e1324_d_n3,) = {
    if (var_guard16 != 0.0) {
        let assign1670_e1321: f64 = (p.p21 * var_vpprime);
        let assign1670_e1322: f64 = (1.0 + assign1670_e1321);
        (assign1670_e1322, (p.p21 * var_vpprime_dn0), (p.p21 * var_vpprime_dn1), (p.p21 * var_vpprime_dn2), (p.p21 * var_vpprime_dn3),)
    } else {
        (var_theta_vp_1, var_theta_vp_1_dn0, var_theta_vp_1_dn1, var_theta_vp_1_dn2, var_theta_vp_1_dn3,)
    }
};
        var_theta_vp_1 = assign1670_e1324;
        var_theta_vp_1_dn0 = assign1670_e1324_d_n0;
        var_theta_vp_1_dn1 = assign1670_e1324_d_n1;
        var_theta_vp_1_dn2 = assign1670_e1324_d_n2;
        var_theta_vp_1_dn3 = assign1670_e1324_d_n3;
        var_theta_vp_1_rv = 0.0;

        let (assign1680_e1332, assign1680_e1332_d_n0, assign1680_e1332_d_n1, assign1680_e1332_d_n2, assign1680_e1332_d_n3,) = {
    if (var_guard16 != 0.0) {
        let assign1680_e1329: f64 = (var_leq * var_theta_vp_1);
        let assign1680_e1330: f64 = (var_kp_weff / assign1680_e1329);
        (assign1680_e1330, (-((var_kp_weff * ((var_leq_dn0 * var_theta_vp_1) + (var_leq * var_theta_vp_1_dn0))) / (assign1680_e1329 * assign1680_e1329))), (-((var_kp_weff * ((var_leq_dn1 * var_theta_vp_1) + (var_leq * var_theta_vp_1_dn1))) / (assign1680_e1329 * assign1680_e1329))), (-((var_kp_weff * ((var_leq_dn2 * var_theta_vp_1) + (var_leq * var_theta_vp_1_dn2))) / (assign1680_e1329 * assign1680_e1329))), (-((var_kp_weff * ((var_leq_dn3 * var_theta_vp_1) + (var_leq * var_theta_vp_1_dn3))) / (assign1680_e1329 * assign1680_e1329))),)
    } else {
        (var_beta, var_beta_dn0, var_beta_dn1, var_beta_dn2, var_beta_dn3,)
    }
};
        var_beta = assign1680_e1332;
        var_beta_dn0 = assign1680_e1332_d_n0;
        var_beta_dn1 = assign1680_e1332_d_n1;
        var_beta_dn2 = assign1680_e1332_d_n2;
        var_beta_dn3 = assign1680_e1332_d_n3;
        var_beta_rv = 0.0;

        let assign1690_e1336: f64 = (var_eta_qi * var_qi);
        let assign1690_e1337: f64 = (var_qb + assign1690_e1336);
        let assign1690_e1339: f64 = if assign1690_e1337 > 0.0 { 1.0 } else { 0.0 };
        var_guard17 = assign1690_e1339;
        var_guard17_rv = 0.0;

        let (assign1700_e1354, assign1700_e1354_d_n0, assign1700_e1354_d_n1, assign1700_e1354_d_n2, assign1700_e1354_d_n3,) = {
    if ((var_guard16 == 0.0) && (var_guard17 != 0.0)) {
        let assign1700_e1349: f64 = (var_eta_qi * var_qi);
        let assign1700_e1350: f64 = (var_qb + assign1700_e1349);
        let assign1700_e1351: f64 = (var_t0 * assign1700_e1350);
        let assign1700_e1352: f64 = (1.0 + assign1700_e1351);
        (assign1700_e1352, (var_t0 * (var_qb_dn0 + (var_eta_qi * var_qi_dn0))), (var_t0 * (var_qb_dn1 + (var_eta_qi * var_qi_dn1))), (var_t0 * (var_qb_dn2 + (var_eta_qi * var_qi_dn2))), (var_t0 * (var_qb_dn3 + (var_eta_qi * var_qi_dn3))),)
    } else {
        (var_e0_q_1, var_e0_q_1_dn0, var_e0_q_1_dn1, var_e0_q_1_dn2, var_e0_q_1_dn3,)
    }
};
        var_e0_q_1 = assign1700_e1354;
        var_e0_q_1_dn0 = assign1700_e1354_d_n0;
        var_e0_q_1_dn1 = assign1700_e1354_d_n1;
        var_e0_q_1_dn2 = assign1700_e1354_d_n2;
        var_e0_q_1_dn3 = assign1700_e1354_d_n3;
        var_e0_q_1_rv = 0.0;

        let (assign1710_e1370, assign1710_e1370_d_n0, assign1710_e1370_d_n1, assign1710_e1370_d_n2, assign1710_e1370_d_n3,) = {
    if ((var_guard16 == 0.0) && (var_guard17 == 0.0)) {
        let assign1710_e1365: f64 = (var_eta_qi * var_qi);
        let assign1710_e1366: f64 = (var_qb + assign1710_e1365);
        let assign1710_e1367: f64 = (var_t0 * assign1710_e1366);
        let assign1710_e1368: f64 = (1.0 - assign1710_e1367);
        (assign1710_e1368, (-(var_t0 * (var_qb_dn0 + (var_eta_qi * var_qi_dn0)))), (-(var_t0 * (var_qb_dn1 + (var_eta_qi * var_qi_dn1)))), (-(var_t0 * (var_qb_dn2 + (var_eta_qi * var_qi_dn2)))), (-(var_t0 * (var_qb_dn3 + (var_eta_qi * var_qi_dn3)))),)
    } else {
        (var_e0_q_1, var_e0_q_1_dn0, var_e0_q_1_dn1, var_e0_q_1_dn2, var_e0_q_1_dn3,)
    }
};
        var_e0_q_1 = assign1710_e1370;
        var_e0_q_1_dn0 = assign1710_e1370_d_n0;
        var_e0_q_1_dn1 = assign1710_e1370_d_n1;
        var_e0_q_1_dn2 = assign1710_e1370_d_n2;
        var_e0_q_1_dn3 = assign1710_e1370_d_n3;
        var_e0_q_1_rv = 0.0;

        let (assign1720_e1379, assign1720_e1379_d_n0, assign1720_e1379_d_n1, assign1720_e1379_d_n2, assign1720_e1379_d_n3,) = {
    if (var_guard16 == 0.0) {
        let assign1720_e1376: f64 = (var_t0 * var_gamma_sqrt_phi);
        let assign1720_e1377: f64 = (1.0 + assign1720_e1376);
        (assign1720_e1377, (var_t0 * var_gamma_sqrt_phi_dn0), (var_t0 * var_gamma_sqrt_phi_dn1), (var_t0 * var_gamma_sqrt_phi_dn2), (var_t0 * var_gamma_sqrt_phi_dn3),)
    } else {
        (var_t0_gamma_1, var_t0_gamma_1_dn0, var_t0_gamma_1_dn1, var_t0_gamma_1_dn2, var_t0_gamma_1_dn3,)
    }
};
        var_t0_gamma_1 = assign1720_e1379;
        var_t0_gamma_1_dn0 = assign1720_e1379_d_n0;
        var_t0_gamma_1_dn1 = assign1720_e1379_d_n1;
        var_t0_gamma_1_dn2 = assign1720_e1379_d_n2;
        var_t0_gamma_1_dn3 = assign1720_e1379_d_n3;
        var_t0_gamma_1_rv = 0.0;

        let (assign1730_e1390, assign1730_e1390_d_n0, assign1730_e1390_d_n1, assign1730_e1390_d_n2, assign1730_e1390_d_n3,) = {
    if (var_guard16 == 0.0) {
        let assign1730_e1384: f64 = (var_kp_weff * var_t0_gamma_1);
        let assign1730_e1387: f64 = (var_leq * var_e0_q_1);
        let assign1730_e1388: f64 = (assign1730_e1384 / assign1730_e1387);
        (assign1730_e1388, ((((var_kp_weff * var_t0_gamma_1_dn0) * assign1730_e1387) - (assign1730_e1384 * ((var_leq_dn0 * var_e0_q_1) + (var_leq * var_e0_q_1_dn0)))) / (assign1730_e1387 * assign1730_e1387)), ((((var_kp_weff * var_t0_gamma_1_dn1) * assign1730_e1387) - (assign1730_e1384 * ((var_leq_dn1 * var_e0_q_1) + (var_leq * var_e0_q_1_dn1)))) / (assign1730_e1387 * assign1730_e1387)), ((((var_kp_weff * var_t0_gamma_1_dn2) * assign1730_e1387) - (assign1730_e1384 * ((var_leq_dn2 * var_e0_q_1) + (var_leq * var_e0_q_1_dn2)))) / (assign1730_e1387 * assign1730_e1387)), ((((var_kp_weff * var_t0_gamma_1_dn3) * assign1730_e1387) - (assign1730_e1384 * ((var_leq_dn3 * var_e0_q_1) + (var_leq * var_e0_q_1_dn3)))) / (assign1730_e1387 * assign1730_e1387)),)
    } else {
        (var_beta, var_beta_dn0, var_beta_dn1, var_beta_dn2, var_beta_dn3,)
    }
};
        var_beta = assign1730_e1390;
        var_beta_dn0 = assign1730_e1390_d_n0;
        var_beta_dn1 = assign1730_e1390_d_n1;
        var_beta_dn2 = assign1730_e1390_d_n2;
        var_beta_dn3 = assign1730_e1390_d_n3;
        var_beta_rv = 0.0;

        let assign1740_e1393: f64 = (var_phi_t + var_vp);
        let assign1740_e1395: f64 = (assign1740_e1393 + var_vt_4);
        let assign1740_e1396: f64 = (assign1740_e1395).sqrt();
        var_sqrt_phi_vp = assign1740_e1396;
        var_sqrt_phi_vp_dn0 = ((var_phi_t_dn0 + var_vp_dn0) / (2.0 * assign1740_e1396));
        var_sqrt_phi_vp_dn1 = ((var_phi_t_dn1 + var_vp_dn1) / (2.0 * assign1740_e1396));
        var_sqrt_phi_vp_dn2 = ((var_phi_t_dn2 + var_vp_dn2) / (2.0 * assign1740_e1396));
        var_sqrt_phi_vp_dn3 = ((var_phi_t_dn3 + var_vp_dn3) / (2.0 * assign1740_e1396));
        var_sqrt_phi_vp_rv = 0.0;

        let assign1750_e1401: f64 = (2.0 * var_sqrt_phi_vp);
        let assign1750_e1402: f64 = (var_gamma_s / assign1750_e1401);
        let assign1750_e1403: f64 = (1.0 + assign1750_e1402);
        var_n = assign1750_e1403;
        var_n_dn0 = (-((var_gamma_s * (2.0 * var_sqrt_phi_vp_dn0)) / (assign1750_e1401 * assign1750_e1401)));
        var_n_dn1 = (-((var_gamma_s * (2.0 * var_sqrt_phi_vp_dn1)) / (assign1750_e1401 * assign1750_e1401)));
        var_n_dn2 = (-((var_gamma_s * (2.0 * var_sqrt_phi_vp_dn2)) / (assign1750_e1401 * assign1750_e1401)));
        var_n_dn3 = (-((var_gamma_s * (2.0 * var_sqrt_phi_vp_dn3)) / (assign1750_e1401 * assign1750_e1401)));
        var_n_rv = 0.0;

        let assign1760_e1406: f64 = (var_if_ - var_irprime);
        var_if_ir = assign1760_e1406;
        var_if_ir_dn0 = (var_if__dn0 - var_irprime_dn0);
        var_if_ir_dn1 = (var_if__dn1 - var_irprime_dn1);
        var_if_ir_dn2 = (var_if__dn2 - var_irprime_dn2);
        var_if_ir_dn3 = (var_if__dn3 - var_irprime_dn3);
        var_if_ir_rv = 0.0;

        let assign1770_e1409: f64 = (var_vt_vt_2 * var_n);
        let assign1770_e1411: f64 = (assign1770_e1409 * var_beta);
        var_ispec = assign1770_e1411;
        var_ispec_dn0 = (((var_vt_vt_2 * var_n_dn0) * var_beta) + (assign1770_e1409 * var_beta_dn0));
        var_ispec_dn1 = (((var_vt_vt_2 * var_n_dn1) * var_beta) + (assign1770_e1409 * var_beta_dn1));
        var_ispec_dn2 = (((var_vt_vt_2 * var_n_dn2) * var_beta) + (assign1770_e1409 * var_beta_dn2));
        var_ispec_dn3 = (((var_vt_vt_2 * var_n_dn3) * var_beta) + (assign1770_e1409 * var_beta_dn3));
        var_ispec_rv = 0.0;

        let assign1820_e1436: f64 = (var_sqrt_gammastar + var_sqrt_gammastar);
        let assign1820_e1437: f64 = (var_gammaprime / assign1820_e1436);
        var_tmp1 = assign1820_e1437;
        var_tmp1_dn0 = (((var_gammaprime_dn0 * assign1820_e1436) - (var_gammaprime * (var_sqrt_gammastar_dn0 + var_sqrt_gammastar_dn0))) / (assign1820_e1436 * assign1820_e1436));
        var_tmp1_dn1 = (((var_gammaprime_dn1 * assign1820_e1436) - (var_gammaprime * (var_sqrt_gammastar_dn1 + var_sqrt_gammastar_dn1))) / (assign1820_e1436 * assign1820_e1436));
        var_tmp1_dn2 = (((var_gammaprime_dn2 * assign1820_e1436) - (var_gammaprime * (var_sqrt_gammastar_dn2 + var_sqrt_gammastar_dn2))) / (assign1820_e1436 * assign1820_e1436));
        var_tmp1_dn3 = (((var_gammaprime_dn3 * assign1820_e1436) - (var_gammaprime * (var_sqrt_gammastar_dn3 + var_sqrt_gammastar_dn3))) / (assign1820_e1436 * assign1820_e1436));
        var_tmp1_rv = 0.0;

        let assign1830_e1440: f64 = (var_vgprime / var_sqrt_vgstar);
        var_tmp2 = assign1830_e1440;
        var_tmp2_dn0 = (((var_vgprime_dn0 * var_sqrt_vgstar) - (var_vgprime * var_sqrt_vgstar_dn0)) / (var_sqrt_vgstar * var_sqrt_vgstar));
        var_tmp2_dn1 = (((var_vgprime_dn1 * var_sqrt_vgstar) - (var_vgprime * var_sqrt_vgstar_dn1)) / (var_sqrt_vgstar * var_sqrt_vgstar));
        var_tmp2_dn2 = (((var_vgprime_dn2 * var_sqrt_vgstar) - (var_vgprime * var_sqrt_vgstar_dn2)) / (var_sqrt_vgstar * var_sqrt_vgstar));
        var_tmp2_dn3 = (((var_vgprime_dn3 * var_sqrt_vgstar) - (var_vgprime * var_sqrt_vgstar_dn3)) / (var_sqrt_vgstar * var_sqrt_vgstar));
        var_tmp2_rv = 0.0;

        let assign1840_e1442: f64 = (-var_leta_l);
        let assign1840_e1444: f64 = (assign1840_e1442 * var_tmp1);
        let assign1840_e1446: f64 = (assign1840_e1444 * var_sqrt_phi_vd);
        let assign1840_e1448: f64 = (assign1840_e1446 / var_sqrt_phi_vd_vt);
        var_dgammaprime_dvd = assign1840_e1448;
        var_dgammaprime_dvd_dn0 = ((((((assign1840_e1442 * var_tmp1_dn0) * var_sqrt_phi_vd) + (assign1840_e1444 * var_sqrt_phi_vd_dn0)) * var_sqrt_phi_vd_vt) - (assign1840_e1446 * var_sqrt_phi_vd_vt_dn0)) / (var_sqrt_phi_vd_vt * var_sqrt_phi_vd_vt));
        var_dgammaprime_dvd_dn1 = ((((((assign1840_e1442 * var_tmp1_dn1) * var_sqrt_phi_vd) + (assign1840_e1444 * var_sqrt_phi_vd_dn1)) * var_sqrt_phi_vd_vt) - (assign1840_e1446 * var_sqrt_phi_vd_vt_dn1)) / (var_sqrt_phi_vd_vt * var_sqrt_phi_vd_vt));
        var_dgammaprime_dvd_dn2 = ((((((assign1840_e1442 * var_tmp1_dn2) * var_sqrt_phi_vd) + (assign1840_e1444 * var_sqrt_phi_vd_dn2)) * var_sqrt_phi_vd_vt) - (assign1840_e1446 * var_sqrt_phi_vd_vt_dn2)) / (var_sqrt_phi_vd_vt * var_sqrt_phi_vd_vt));
        var_dgammaprime_dvd_dn3 = ((((((assign1840_e1442 * var_tmp1_dn3) * var_sqrt_phi_vd) + (assign1840_e1444 * var_sqrt_phi_vd_dn3)) * var_sqrt_phi_vd_vt) - (assign1840_e1446 * var_sqrt_phi_vd_vt_dn3)) / (var_sqrt_phi_vd_vt * var_sqrt_phi_vd_vt));
        var_dgammaprime_dvd_rv = 0.0;

        let assign1850_e1450: f64 = (-var_leta_l);
        let assign1850_e1452: f64 = (assign1850_e1450 * var_tmp1);
        let assign1850_e1454: f64 = (assign1850_e1452 * var_sqrt_phi_vs);
        let assign1850_e1456: f64 = (assign1850_e1454 / var_sqrt_phi_vs_vt);
        var_dgammaprime_dvs = assign1850_e1456;
        var_dgammaprime_dvs_dn0 = ((((((assign1850_e1450 * var_tmp1_dn0) * var_sqrt_phi_vs) + (assign1850_e1452 * var_sqrt_phi_vs_dn0)) * var_sqrt_phi_vs_vt) - (assign1850_e1454 * var_sqrt_phi_vs_vt_dn0)) / (var_sqrt_phi_vs_vt * var_sqrt_phi_vs_vt));
        var_dgammaprime_dvs_dn1 = ((((((assign1850_e1450 * var_tmp1_dn1) * var_sqrt_phi_vs) + (assign1850_e1452 * var_sqrt_phi_vs_dn1)) * var_sqrt_phi_vs_vt) - (assign1850_e1454 * var_sqrt_phi_vs_vt_dn1)) / (var_sqrt_phi_vs_vt * var_sqrt_phi_vs_vt));
        var_dgammaprime_dvs_dn2 = ((((((assign1850_e1450 * var_tmp1_dn2) * var_sqrt_phi_vs) + (assign1850_e1452 * var_sqrt_phi_vs_dn2)) * var_sqrt_phi_vs_vt) - (assign1850_e1454 * var_sqrt_phi_vs_vt_dn2)) / (var_sqrt_phi_vs_vt * var_sqrt_phi_vs_vt));
        var_dgammaprime_dvs_dn3 = ((((((assign1850_e1450 * var_tmp1_dn3) * var_sqrt_phi_vs) + (assign1850_e1452 * var_sqrt_phi_vs_dn3)) * var_sqrt_phi_vs_vt) - (assign1850_e1454 * var_sqrt_phi_vs_vt_dn3)) / (var_sqrt_phi_vs_vt * var_sqrt_phi_vs_vt));
        var_dgammaprime_dvs_rv = 0.0;

        let assign1870_e1474: f64 = (var_vp + var_phi_t);
        let assign1870_e1476: f64 = (assign1870_e1474 / var_big_sqrt_vp);
        var_tmp3 = assign1870_e1476;
        var_tmp3_dn0 = ((((var_vp_dn0 + var_phi_t_dn0) * var_big_sqrt_vp) - (assign1870_e1474 * var_big_sqrt_vp_dn0)) / (var_big_sqrt_vp * var_big_sqrt_vp));
        var_tmp3_dn1 = ((((var_vp_dn1 + var_phi_t_dn1) * var_big_sqrt_vp) - (assign1870_e1474 * var_big_sqrt_vp_dn1)) / (var_big_sqrt_vp * var_big_sqrt_vp));
        var_tmp3_dn2 = ((((var_vp_dn2 + var_phi_t_dn2) * var_big_sqrt_vp) - (assign1870_e1474 * var_big_sqrt_vp_dn2)) / (var_big_sqrt_vp * var_big_sqrt_vp));
        var_tmp3_dn3 = ((((var_vp_dn3 + var_phi_t_dn3) * var_big_sqrt_vp) - (assign1870_e1474 * var_big_sqrt_vp_dn3)) / (var_big_sqrt_vp * var_big_sqrt_vp));
        var_tmp3_rv = 0.0;

        let assign1880_e1478: f64 = (-var_tmp3);
        let assign1880_e1480: f64 = (assign1880_e1478 * var_dgammaprime_dvd);
        var_dvp_dvd = assign1880_e1480;
        var_dvp_dvd_dn0 = (((-var_tmp3_dn0) * var_dgammaprime_dvd) + (assign1880_e1478 * var_dgammaprime_dvd_dn0));
        var_dvp_dvd_dn1 = (((-var_tmp3_dn1) * var_dgammaprime_dvd) + (assign1880_e1478 * var_dgammaprime_dvd_dn1));
        var_dvp_dvd_dn2 = (((-var_tmp3_dn2) * var_dgammaprime_dvd) + (assign1880_e1478 * var_dgammaprime_dvd_dn2));
        var_dvp_dvd_dn3 = (((-var_tmp3_dn3) * var_dgammaprime_dvd) + (assign1880_e1478 * var_dgammaprime_dvd_dn3));
        var_dvp_dvd_rv = 0.0;

        let assign1890_e1482: f64 = (-var_tmp3);
        let assign1890_e1484: f64 = (assign1890_e1482 * var_dgammaprime_dvs);
        var_dvp_dvs = assign1890_e1484;
        var_dvp_dvs_dn0 = (((-var_tmp3_dn0) * var_dgammaprime_dvs) + (assign1890_e1482 * var_dgammaprime_dvs_dn0));
        var_dvp_dvs_dn1 = (((-var_tmp3_dn1) * var_dgammaprime_dvs) + (assign1890_e1482 * var_dgammaprime_dvs_dn1));
        var_dvp_dvs_dn2 = (((-var_tmp3_dn2) * var_dgammaprime_dvs) + (assign1890_e1482 * var_dgammaprime_dvs_dn2));
        var_dvp_dvs_dn3 = (((-var_tmp3_dn3) * var_dgammaprime_dvs) + (assign1890_e1482 * var_dgammaprime_dvs_dn3));
        var_dvp_dvs_rv = 0.0;

        let assign1910_e1501: f64 = (var_dif_dv * var_inv_vt);
        var_tmp1 = assign1910_e1501;
        var_tmp1_dn0 = (var_dif_dv_dn0 * var_inv_vt);
        var_tmp1_dn1 = (var_dif_dv_dn1 * var_inv_vt);
        var_tmp1_dn2 = (var_dif_dv_dn2 * var_inv_vt);
        var_tmp1_dn3 = (var_dif_dv_dn3 * var_inv_vt);
        var_tmp1_rv = 0.0;

        let assign1920_e1504: f64 = (var_tmp1 * var_dvp_dvd);
        var_dif_dvd = assign1920_e1504;
        var_dif_dvd_dn0 = ((var_tmp1_dn0 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn0));
        var_dif_dvd_dn1 = ((var_tmp1_dn1 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn1));
        var_dif_dvd_dn2 = ((var_tmp1_dn2 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn2));
        var_dif_dvd_dn3 = ((var_tmp1_dn3 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn3));
        var_dif_dvd_rv = 0.0;

        let assign1930_e1508: f64 = (var_dvp_dvs - 1.0);
        let assign1930_e1509: f64 = (var_tmp1 * assign1930_e1508);
        var_dif_dvs = assign1930_e1509;
        var_dif_dvs_dn0 = ((var_tmp1_dn0 * assign1930_e1508) + (var_tmp1 * var_dvp_dvs_dn0));
        var_dif_dvs_dn1 = ((var_tmp1_dn1 * assign1930_e1508) + (var_tmp1 * var_dvp_dvs_dn1));
        var_dif_dvs_dn2 = ((var_tmp1_dn2 * assign1930_e1508) + (var_tmp1 * var_dvp_dvs_dn2));
        var_dif_dvs_dn3 = ((var_tmp1_dn3 * assign1930_e1508) + (var_tmp1 * var_dvp_dvs_dn3));
        var_dif_dvs_rv = 0.0;

        let assign1950_e1516: f64 = (4.0 * var_vdss_sqrt);
        let assign1950_e1518: f64 = (assign1950_e1516 * var_sqrt_if);
        let assign1950_e1519: f64 = (var_vt / assign1950_e1518);
        var_tmp1 = assign1950_e1519;
        var_tmp1_dn0 = (-((var_vt * (((4.0 * var_vdss_sqrt_dn0) * var_sqrt_if) + (assign1950_e1516 * var_sqrt_if_dn0))) / (assign1950_e1518 * assign1950_e1518)));
        var_tmp1_dn1 = (-((var_vt * (((4.0 * var_vdss_sqrt_dn1) * var_sqrt_if) + (assign1950_e1516 * var_sqrt_if_dn1))) / (assign1950_e1518 * assign1950_e1518)));
        var_tmp1_dn2 = (-((var_vt * (((4.0 * var_vdss_sqrt_dn2) * var_sqrt_if) + (assign1950_e1516 * var_sqrt_if_dn2))) / (assign1950_e1518 * assign1950_e1518)));
        var_tmp1_dn3 = (-((var_vt * (((4.0 * var_vdss_sqrt_dn3) * var_sqrt_if) + (assign1950_e1516 * var_sqrt_if_dn3))) / (assign1950_e1518 * assign1950_e1518)));
        var_tmp1_rv = 0.0;

        let assign1960_e1522: f64 = (var_tmp1 * var_dif_dvd);
        var_dvdss_dvd = assign1960_e1522;
        var_dvdss_dvd_dn0 = ((var_tmp1_dn0 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn0));
        var_dvdss_dvd_dn1 = ((var_tmp1_dn1 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn1));
        var_dvdss_dvd_dn2 = ((var_tmp1_dn2 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn2));
        var_dvdss_dvd_dn3 = ((var_tmp1_dn3 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn3));
        var_dvdss_dvd_rv = 0.0;

        let assign1970_e1525: f64 = (var_tmp1 * var_dif_dvs);
        var_dvdss_dvs = assign1970_e1525;
        var_dvdss_dvs_dn0 = ((var_tmp1_dn0 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn0));
        var_dvdss_dvs_dn1 = ((var_tmp1_dn1 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn1));
        var_dvdss_dvs_dn2 = ((var_tmp1_dn2 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn2));
        var_dvdss_dvs_dn3 = ((var_tmp1_dn3 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn3));
        var_dvdss_dvs_rv = 0.0;

        let assign1990_e1531: f64 = (var_vt_4 + var_vt_4);
        let assign1990_e1533: f64 = (assign1990_e1531 * p.p25);
        var_tmp1 = assign1990_e1533;
        var_tmp1_dn0 = 0.0;
        var_tmp1_dn1 = 0.0;
        var_tmp1_dn2 = 0.0;
        var_tmp1_dn3 = 0.0;
        var_tmp1_rv = 0.0;

        let assign2000_e1537: f64 = (var_sqrt_if + var_sqrt_if);
        let assign2000_e1538: f64 = (var_vt / assign2000_e1537);
        var_tmp2 = assign2000_e1538;
        var_tmp2_dn0 = (-((var_vt * (var_sqrt_if_dn0 + var_sqrt_if_dn0)) / (assign2000_e1537 * assign2000_e1537)));
        var_tmp2_dn1 = (-((var_vt * (var_sqrt_if_dn1 + var_sqrt_if_dn1)) / (assign2000_e1537 * assign2000_e1537)));
        var_tmp2_dn2 = (-((var_vt * (var_sqrt_if_dn2 + var_sqrt_if_dn2)) / (assign2000_e1537 * assign2000_e1537)));
        var_tmp2_dn3 = (-((var_vt * (var_sqrt_if_dn3 + var_sqrt_if_dn3)) / (assign2000_e1537 * assign2000_e1537)));
        var_tmp2_rv = 0.0;

        let assign2010_e1542: f64 = (var_dif_dvd * var_tmp2);
        let assign2010_e1544: f64 = (assign2010_e1542 - var_dvdss_dvd);
        let assign2010_e1545: f64 = (var_tmp1 * assign2010_e1544);
        var_ddeltav_dvd = assign2010_e1545;
        var_ddeltav_dvd_dn0 = ((var_tmp1_dn0 * assign2010_e1544) + (var_tmp1 * (((var_dif_dvd_dn0 * var_tmp2) + (var_dif_dvd * var_tmp2_dn0)) - var_dvdss_dvd_dn0)));
        var_ddeltav_dvd_dn1 = ((var_tmp1_dn1 * assign2010_e1544) + (var_tmp1 * (((var_dif_dvd_dn1 * var_tmp2) + (var_dif_dvd * var_tmp2_dn1)) - var_dvdss_dvd_dn1)));
        var_ddeltav_dvd_dn2 = ((var_tmp1_dn2 * assign2010_e1544) + (var_tmp1 * (((var_dif_dvd_dn2 * var_tmp2) + (var_dif_dvd * var_tmp2_dn2)) - var_dvdss_dvd_dn2)));
        var_ddeltav_dvd_dn3 = ((var_tmp1_dn3 * assign2010_e1544) + (var_tmp1 * (((var_dif_dvd_dn3 * var_tmp2) + (var_dif_dvd * var_tmp2_dn3)) - var_dvdss_dvd_dn3)));
        var_ddeltav_dvd_rv = 0.0;

        let assign2020_e1549: f64 = (var_dif_dvs * var_tmp2);
        let assign2020_e1551: f64 = (assign2020_e1549 - var_dvdss_dvs);
        let assign2020_e1552: f64 = (var_tmp1 * assign2020_e1551);
        var_ddeltav_dvs = assign2020_e1552;
        var_ddeltav_dvs_dn0 = ((var_tmp1_dn0 * assign2020_e1551) + (var_tmp1 * (((var_dif_dvs_dn0 * var_tmp2) + (var_dif_dvs * var_tmp2_dn0)) - var_dvdss_dvs_dn0)));
        var_ddeltav_dvs_dn1 = ((var_tmp1_dn1 * assign2020_e1551) + (var_tmp1 * (((var_dif_dvs_dn1 * var_tmp2) + (var_dif_dvs * var_tmp2_dn1)) - var_dvdss_dvs_dn1)));
        var_ddeltav_dvs_dn2 = ((var_tmp1_dn2 * assign2020_e1551) + (var_tmp1 * (((var_dif_dvs_dn2 * var_tmp2) + (var_dif_dvs * var_tmp2_dn2)) - var_dvdss_dvs_dn2)));
        var_ddeltav_dvs_dn3 = ((var_tmp1_dn3 * assign2020_e1551) + (var_tmp1 * (((var_dif_dvs_dn3 * var_tmp2) + (var_dif_dvs * var_tmp2_dn3)) - var_dvdss_dvs_dn3)));
        var_ddeltav_dvs_rv = 0.0;

        let assign2040_e1562: f64 = (1.0 / var_sqrt_vdss_deltav);
        var_tmp1 = assign2040_e1562;
        var_tmp1_dn0 = (-(var_sqrt_vdss_deltav_dn0 / (var_sqrt_vdss_deltav * var_sqrt_vdss_deltav)));
        var_tmp1_dn1 = (-(var_sqrt_vdss_deltav_dn1 / (var_sqrt_vdss_deltav * var_sqrt_vdss_deltav)));
        var_tmp1_dn2 = (-(var_sqrt_vdss_deltav_dn2 / (var_sqrt_vdss_deltav * var_sqrt_vdss_deltav)));
        var_tmp1_dn3 = (-(var_sqrt_vdss_deltav_dn3 / (var_sqrt_vdss_deltav * var_sqrt_vdss_deltav)));
        var_tmp1_rv = 0.0;

        let assign2050_e1565: f64 = (1.0 / var_sqrt_vds_vdss_deltav);
        var_tmp2 = assign2050_e1565;
        var_tmp2_dn0 = (-(var_sqrt_vds_vdss_deltav_dn0 / (var_sqrt_vds_vdss_deltav * var_sqrt_vds_vdss_deltav)));
        var_tmp2_dn1 = (-(var_sqrt_vds_vdss_deltav_dn1 / (var_sqrt_vds_vdss_deltav * var_sqrt_vds_vdss_deltav)));
        var_tmp2_dn2 = (-(var_sqrt_vds_vdss_deltav_dn2 / (var_sqrt_vds_vdss_deltav * var_sqrt_vds_vdss_deltav)));
        var_tmp2_dn3 = (-(var_sqrt_vds_vdss_deltav_dn3 / (var_sqrt_vds_vdss_deltav * var_sqrt_vds_vdss_deltav)));
        var_tmp2_rv = 0.0;

        let assign2060_e1568: f64 = (var_vds - var_vdss);
        var_tmp3 = assign2060_e1568;
        var_tmp3_dn0 = (var_vds_dn0 - var_vdss_dn0);
        var_tmp3_dn1 = (-var_vdss_dn1);
        var_tmp3_dn2 = (var_vds_dn2 - var_vdss_dn2);
        var_tmp3_dn3 = (var_vds_dn3 - var_vdss_dn3);
        var_tmp3_rv = 0.0;

        let assign2070_e1571: f64 = (var_vdss * var_dvdss_dvd);
        let assign2070_e1573: f64 = (assign2070_e1571 + var_ddeltav_dvd);
        let assign2070_e1575: f64 = (assign2070_e1573 * var_tmp1);
        let assign2070_e1579: f64 = (0.5 - var_dvdss_dvd);
        let assign2070_e1580: f64 = (var_tmp3 * assign2070_e1579);
        let assign2070_e1582: f64 = (assign2070_e1580 + var_ddeltav_dvd);
        let assign2070_e1584: f64 = (assign2070_e1582 * var_tmp2);
        let assign2070_e1585: f64 = (assign2070_e1575 - assign2070_e1584);
        var_dvip_dvd = assign2070_e1585;
        var_dvip_dvd_dn0 = ((((((var_vdss_dn0 * var_dvdss_dvd) + (var_vdss * var_dvdss_dvd_dn0)) + var_ddeltav_dvd_dn0) * var_tmp1) + (assign2070_e1573 * var_tmp1_dn0)) - (((((var_tmp3_dn0 * assign2070_e1579) + (var_tmp3 * (-var_dvdss_dvd_dn0))) + var_ddeltav_dvd_dn0) * var_tmp2) + (assign2070_e1582 * var_tmp2_dn0)));
        var_dvip_dvd_dn1 = ((((((var_vdss_dn1 * var_dvdss_dvd) + (var_vdss * var_dvdss_dvd_dn1)) + var_ddeltav_dvd_dn1) * var_tmp1) + (assign2070_e1573 * var_tmp1_dn1)) - (((((var_tmp3_dn1 * assign2070_e1579) + (var_tmp3 * (-var_dvdss_dvd_dn1))) + var_ddeltav_dvd_dn1) * var_tmp2) + (assign2070_e1582 * var_tmp2_dn1)));
        var_dvip_dvd_dn2 = ((((((var_vdss_dn2 * var_dvdss_dvd) + (var_vdss * var_dvdss_dvd_dn2)) + var_ddeltav_dvd_dn2) * var_tmp1) + (assign2070_e1573 * var_tmp1_dn2)) - (((((var_tmp3_dn2 * assign2070_e1579) + (var_tmp3 * (-var_dvdss_dvd_dn2))) + var_ddeltav_dvd_dn2) * var_tmp2) + (assign2070_e1582 * var_tmp2_dn2)));
        var_dvip_dvd_dn3 = ((((((var_vdss_dn3 * var_dvdss_dvd) + (var_vdss * var_dvdss_dvd_dn3)) + var_ddeltav_dvd_dn3) * var_tmp1) + (assign2070_e1573 * var_tmp1_dn3)) - (((((var_tmp3_dn3 * assign2070_e1579) + (var_tmp3 * (-var_dvdss_dvd_dn3))) + var_ddeltav_dvd_dn3) * var_tmp2) + (assign2070_e1582 * var_tmp2_dn3)));
        var_dvip_dvd_rv = 0.0;

        let assign2080_e1588: f64 = (var_vdss * var_dvdss_dvs);
        let assign2080_e1590: f64 = (assign2080_e1588 + var_ddeltav_dvs);
        let assign2080_e1592: f64 = (assign2080_e1590 * var_tmp1);
        let assign2080_e1595: f64 = (-0.5);
        let assign2080_e1597: f64 = (assign2080_e1595 - var_dvdss_dvs);
        let assign2080_e1598: f64 = (var_tmp3 * assign2080_e1597);
        let assign2080_e1600: f64 = (assign2080_e1598 + var_ddeltav_dvs);
        let assign2080_e1602: f64 = (assign2080_e1600 * var_tmp2);
        let assign2080_e1603: f64 = (assign2080_e1592 - assign2080_e1602);
        var_dvip_dvs = assign2080_e1603;
        var_dvip_dvs_dn0 = ((((((var_vdss_dn0 * var_dvdss_dvs) + (var_vdss * var_dvdss_dvs_dn0)) + var_ddeltav_dvs_dn0) * var_tmp1) + (assign2080_e1590 * var_tmp1_dn0)) - (((((var_tmp3_dn0 * assign2080_e1597) + (var_tmp3 * (-var_dvdss_dvs_dn0))) + var_ddeltav_dvs_dn0) * var_tmp2) + (assign2080_e1600 * var_tmp2_dn0)));
        var_dvip_dvs_dn1 = ((((((var_vdss_dn1 * var_dvdss_dvs) + (var_vdss * var_dvdss_dvs_dn1)) + var_ddeltav_dvs_dn1) * var_tmp1) + (assign2080_e1590 * var_tmp1_dn1)) - (((((var_tmp3_dn1 * assign2080_e1597) + (var_tmp3 * (-var_dvdss_dvs_dn1))) + var_ddeltav_dvs_dn1) * var_tmp2) + (assign2080_e1600 * var_tmp2_dn1)));
        var_dvip_dvs_dn2 = ((((((var_vdss_dn2 * var_dvdss_dvs) + (var_vdss * var_dvdss_dvs_dn2)) + var_ddeltav_dvs_dn2) * var_tmp1) + (assign2080_e1590 * var_tmp1_dn2)) - (((((var_tmp3_dn2 * assign2080_e1597) + (var_tmp3 * (-var_dvdss_dvs_dn2))) + var_ddeltav_dvs_dn2) * var_tmp2) + (assign2080_e1600 * var_tmp2_dn2)));
        var_dvip_dvs_dn3 = ((((((var_vdss_dn3 * var_dvdss_dvs) + (var_vdss * var_dvdss_dvs_dn3)) + var_ddeltav_dvs_dn3) * var_tmp1) + (assign2080_e1590 * var_tmp1_dn3)) - (((((var_tmp3_dn3 * assign2080_e1597) + (var_tmp3 * (-var_dvdss_dvs_dn3))) + var_ddeltav_dvs_dn3) * var_tmp2) + (assign2080_e1600 * var_tmp2_dn3)));
        var_dvip_dvs_rv = 0.0;

        let assign2100_e1623: f64 = (var_sqrt_if - 1.5);
        let assign2100_e1624: f64 = (var_vt * assign2100_e1623);
        let assign2100_e1627: f64 = (4.0 * var_vdssprime_sqrt);
        let assign2100_e1629: f64 = (assign2100_e1627 * var_if_);
        let assign2100_e1630: f64 = (assign2100_e1624 / assign2100_e1629);
        var_tmp1 = assign2100_e1630;
        var_tmp1_dn0 = ((((var_vt * var_sqrt_if_dn0) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * var_vdssprime_sqrt_dn0) * var_if_) + (assign2100_e1627 * var_if__dn0)))) / (assign2100_e1629 * assign2100_e1629));
        var_tmp1_dn1 = ((((var_vt * var_sqrt_if_dn1) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * var_vdssprime_sqrt_dn1) * var_if_) + (assign2100_e1627 * var_if__dn1)))) / (assign2100_e1629 * assign2100_e1629));
        var_tmp1_dn2 = ((((var_vt * var_sqrt_if_dn2) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * var_vdssprime_sqrt_dn2) * var_if_) + (assign2100_e1627 * var_if__dn2)))) / (assign2100_e1629 * assign2100_e1629));
        var_tmp1_dn3 = ((((var_vt * var_sqrt_if_dn3) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * var_vdssprime_sqrt_dn3) * var_if_) + (assign2100_e1627 * var_if__dn3)))) / (assign2100_e1629 * assign2100_e1629));
        var_tmp1_rv = 0.0;

        let assign2110_e1633: f64 = (var_tmp1 * var_dif_dvd);
        var_dvdssprime_dvd = assign2110_e1633;
        var_dvdssprime_dvd_dn0 = ((var_tmp1_dn0 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn0));
        var_dvdssprime_dvd_dn1 = ((var_tmp1_dn1 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn1));
        var_dvdssprime_dvd_dn2 = ((var_tmp1_dn2 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn2));
        var_dvdssprime_dvd_dn3 = ((var_tmp1_dn3 * var_dif_dvd) + (var_tmp1 * var_dif_dvd_dn3));
        var_dvdssprime_dvd_rv = 0.0;

        let assign2120_e1636: f64 = (var_tmp1 * var_dif_dvs);
        var_dvdssprime_dvs = assign2120_e1636;
        var_dvdssprime_dvs_dn0 = ((var_tmp1_dn0 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn0));
        var_dvdssprime_dvs_dn1 = ((var_tmp1_dn1 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn1));
        var_dvdssprime_dvs_dn2 = ((var_tmp1_dn2 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn2));
        var_dvdssprime_dvs_dn3 = ((var_tmp1_dn3 * var_dif_dvs) + (var_tmp1 * var_dif_dvs_dn3));
        var_dvdssprime_dvs_rv = 0.0;

        let assign2140_e1642: f64 = (var_dirprime_dv * var_inv_vt);
        var_tmp1 = assign2140_e1642;
        var_tmp1_dn0 = (var_dirprime_dv_dn0 * var_inv_vt);
        var_tmp1_dn1 = (var_dirprime_dv_dn1 * var_inv_vt);
        var_tmp1_dn2 = (var_dirprime_dv_dn2 * var_inv_vt);
        var_tmp1_dn3 = (var_dirprime_dv_dn3 * var_inv_vt);
        var_tmp1_rv = 0.0;

        let assign2150_e1645: f64 = (1.0 / var_sqrt_vdssprime_deltav);
        var_tmp2 = assign2150_e1645;
        var_tmp2_dn0 = (-(var_sqrt_vdssprime_deltav_dn0 / (var_sqrt_vdssprime_deltav * var_sqrt_vdssprime_deltav)));
        var_tmp2_dn1 = (-(var_sqrt_vdssprime_deltav_dn1 / (var_sqrt_vdssprime_deltav * var_sqrt_vdssprime_deltav)));
        var_tmp2_dn2 = (-(var_sqrt_vdssprime_deltav_dn2 / (var_sqrt_vdssprime_deltav * var_sqrt_vdssprime_deltav)));
        var_tmp2_dn3 = (-(var_sqrt_vdssprime_deltav_dn3 / (var_sqrt_vdssprime_deltav * var_sqrt_vdssprime_deltav)));
        var_tmp2_rv = 0.0;

        let assign2160_e1648: f64 = (1.0 / var_sqrt_vds_vdssprime_deltav);
        var_tmp3 = assign2160_e1648;
        var_tmp3_dn0 = (-(var_sqrt_vds_vdssprime_deltav_dn0 / (var_sqrt_vds_vdssprime_deltav * var_sqrt_vds_vdssprime_deltav)));
        var_tmp3_dn1 = (-(var_sqrt_vds_vdssprime_deltav_dn1 / (var_sqrt_vds_vdssprime_deltav * var_sqrt_vds_vdssprime_deltav)));
        var_tmp3_dn2 = (-(var_sqrt_vds_vdssprime_deltav_dn2 / (var_sqrt_vds_vdssprime_deltav * var_sqrt_vds_vdssprime_deltav)));
        var_tmp3_dn3 = (-(var_sqrt_vds_vdssprime_deltav_dn3 / (var_sqrt_vds_vdssprime_deltav * var_sqrt_vds_vdssprime_deltav)));
        var_tmp3_rv = 0.0;

        let assign2170_e1652: f64 = (var_dvp_dvd - 0.5);
        let assign2170_e1655: f64 = (var_vdssprime * var_dvdssprime_dvd);
        let assign2170_e1657: f64 = (assign2170_e1655 + var_ddeltav_dvd);
        let assign2170_e1659: f64 = (assign2170_e1657 * var_tmp2);
        let assign2170_e1660: f64 = (assign2170_e1652 - assign2170_e1659);
        let assign2170_e1664: f64 = (0.5 - var_dvdssprime_dvd);
        let assign2170_e1665: f64 = (var_vdsprime * assign2170_e1664);
        let assign2170_e1667: f64 = (assign2170_e1665 + var_ddeltav_dvd);
        let assign2170_e1669: f64 = (assign2170_e1667 * var_tmp3);
        let assign2170_e1670: f64 = (assign2170_e1660 + assign2170_e1669);
        let assign2170_e1671: f64 = (var_tmp1 * assign2170_e1670);
        var_dirprime_dvd = assign2170_e1671;
        var_dirprime_dvd_dn0 = ((var_tmp1_dn0 * assign2170_e1670) + (var_tmp1 * ((var_dvp_dvd_dn0 - (((((var_vdssprime_dn0 * var_dvdssprime_dvd) + (var_vdssprime * var_dvdssprime_dvd_dn0)) + var_ddeltav_dvd_dn0) * var_tmp2) + (assign2170_e1657 * var_tmp2_dn0))) + (((((var_vdsprime_dn0 * assign2170_e1664) + (var_vdsprime * (-var_dvdssprime_dvd_dn0))) + var_ddeltav_dvd_dn0) * var_tmp3) + (assign2170_e1667 * var_tmp3_dn0)))));
        var_dirprime_dvd_dn1 = ((var_tmp1_dn1 * assign2170_e1670) + (var_tmp1 * ((var_dvp_dvd_dn1 - (((((var_vdssprime_dn1 * var_dvdssprime_dvd) + (var_vdssprime * var_dvdssprime_dvd_dn1)) + var_ddeltav_dvd_dn1) * var_tmp2) + (assign2170_e1657 * var_tmp2_dn1))) + (((((var_vdsprime_dn1 * assign2170_e1664) + (var_vdsprime * (-var_dvdssprime_dvd_dn1))) + var_ddeltav_dvd_dn1) * var_tmp3) + (assign2170_e1667 * var_tmp3_dn1)))));
        var_dirprime_dvd_dn2 = ((var_tmp1_dn2 * assign2170_e1670) + (var_tmp1 * ((var_dvp_dvd_dn2 - (((((var_vdssprime_dn2 * var_dvdssprime_dvd) + (var_vdssprime * var_dvdssprime_dvd_dn2)) + var_ddeltav_dvd_dn2) * var_tmp2) + (assign2170_e1657 * var_tmp2_dn2))) + (((((var_vdsprime_dn2 * assign2170_e1664) + (var_vdsprime * (-var_dvdssprime_dvd_dn2))) + var_ddeltav_dvd_dn2) * var_tmp3) + (assign2170_e1667 * var_tmp3_dn2)))));
        var_dirprime_dvd_dn3 = ((var_tmp1_dn3 * assign2170_e1670) + (var_tmp1 * ((var_dvp_dvd_dn3 - (((((var_vdssprime_dn3 * var_dvdssprime_dvd) + (var_vdssprime * var_dvdssprime_dvd_dn3)) + var_ddeltav_dvd_dn3) * var_tmp2) + (assign2170_e1657 * var_tmp2_dn3))) + (((((var_vdsprime_dn3 * assign2170_e1664) + (var_vdsprime * (-var_dvdssprime_dvd_dn3))) + var_ddeltav_dvd_dn3) * var_tmp3) + (assign2170_e1667 * var_tmp3_dn3)))));
        var_dirprime_dvd_rv = 0.0;

        let assign2180_e1675: f64 = (var_dvp_dvs - 0.5);
        let assign2180_e1678: f64 = (var_vdssprime * var_dvdssprime_dvs);
        let assign2180_e1680: f64 = (assign2180_e1678 + var_ddeltav_dvs);
        let assign2180_e1682: f64 = (assign2180_e1680 * var_tmp2);
        let assign2180_e1683: f64 = (assign2180_e1675 - assign2180_e1682);
        let assign2180_e1686: f64 = (-0.5);
        let assign2180_e1688: f64 = (assign2180_e1686 - var_dvdssprime_dvs);
        let assign2180_e1689: f64 = (var_vdsprime * assign2180_e1688);
        let assign2180_e1691: f64 = (assign2180_e1689 + var_ddeltav_dvs);
        let assign2180_e1693: f64 = (assign2180_e1691 * var_tmp3);
        let assign2180_e1694: f64 = (assign2180_e1683 + assign2180_e1693);
        let assign2180_e1695: f64 = (var_tmp1 * assign2180_e1694);
        var_dirprime_dvs = assign2180_e1695;
        var_dirprime_dvs_dn0 = ((var_tmp1_dn0 * assign2180_e1694) + (var_tmp1 * ((var_dvp_dvs_dn0 - (((((var_vdssprime_dn0 * var_dvdssprime_dvs) + (var_vdssprime * var_dvdssprime_dvs_dn0)) + var_ddeltav_dvs_dn0) * var_tmp2) + (assign2180_e1680 * var_tmp2_dn0))) + (((((var_vdsprime_dn0 * assign2180_e1688) + (var_vdsprime * (-var_dvdssprime_dvs_dn0))) + var_ddeltav_dvs_dn0) * var_tmp3) + (assign2180_e1691 * var_tmp3_dn0)))));
        var_dirprime_dvs_dn1 = ((var_tmp1_dn1 * assign2180_e1694) + (var_tmp1 * ((var_dvp_dvs_dn1 - (((((var_vdssprime_dn1 * var_dvdssprime_dvs) + (var_vdssprime * var_dvdssprime_dvs_dn1)) + var_ddeltav_dvs_dn1) * var_tmp2) + (assign2180_e1680 * var_tmp2_dn1))) + (((((var_vdsprime_dn1 * assign2180_e1688) + (var_vdsprime * (-var_dvdssprime_dvs_dn1))) + var_ddeltav_dvs_dn1) * var_tmp3) + (assign2180_e1691 * var_tmp3_dn1)))));
        var_dirprime_dvs_dn2 = ((var_tmp1_dn2 * assign2180_e1694) + (var_tmp1 * ((var_dvp_dvs_dn2 - (((((var_vdssprime_dn2 * var_dvdssprime_dvs) + (var_vdssprime * var_dvdssprime_dvs_dn2)) + var_ddeltav_dvs_dn2) * var_tmp2) + (assign2180_e1680 * var_tmp2_dn2))) + (((((var_vdsprime_dn2 * assign2180_e1688) + (var_vdsprime * (-var_dvdssprime_dvs_dn2))) + var_ddeltav_dvs_dn2) * var_tmp3) + (assign2180_e1691 * var_tmp3_dn2)))));
        var_dirprime_dvs_dn3 = ((var_tmp1_dn3 * assign2180_e1694) + (var_tmp1 * ((var_dvp_dvs_dn3 - (((((var_vdssprime_dn3 * var_dvdssprime_dvs) + (var_vdssprime * var_dvdssprime_dvs_dn3)) + var_ddeltav_dvs_dn3) * var_tmp2) + (assign2180_e1680 * var_tmp2_dn3))) + (((((var_vdsprime_dn3 * assign2180_e1688) + (var_vdsprime * (-var_dvdssprime_dvs_dn3))) + var_ddeltav_dvs_dn3) * var_tmp3) + (assign2180_e1691 * var_tmp3_dn3)))));
        var_dirprime_dvs_rv = 0.0;

        *var_beta_slot = var_beta;
        *var_beta_dn0_slot = var_beta_dn0;
        *var_beta_dn1_slot = var_beta_dn1;
        *var_beta_dn2_slot = var_beta_dn2;
        *var_beta_dn3_slot = var_beta_dn3;
        *var_beta_rv_slot = var_beta_rv;
        *var_ddeltav_dvd_slot = var_ddeltav_dvd;
        *var_ddeltav_dvd_dn0_slot = var_ddeltav_dvd_dn0;
        *var_ddeltav_dvd_dn1_slot = var_ddeltav_dvd_dn1;
        *var_ddeltav_dvd_dn2_slot = var_ddeltav_dvd_dn2;
        *var_ddeltav_dvd_dn3_slot = var_ddeltav_dvd_dn3;
        *var_ddeltav_dvd_rv_slot = var_ddeltav_dvd_rv;
        *var_ddeltav_dvs_slot = var_ddeltav_dvs;
        *var_ddeltav_dvs_dn0_slot = var_ddeltav_dvs_dn0;
        *var_ddeltav_dvs_dn1_slot = var_ddeltav_dvs_dn1;
        *var_ddeltav_dvs_dn2_slot = var_ddeltav_dvs_dn2;
        *var_ddeltav_dvs_dn3_slot = var_ddeltav_dvs_dn3;
        *var_ddeltav_dvs_rv_slot = var_ddeltav_dvs_rv;
        *var_dgammaprime_dvd_slot = var_dgammaprime_dvd;
        *var_dgammaprime_dvd_dn0_slot = var_dgammaprime_dvd_dn0;
        *var_dgammaprime_dvd_dn1_slot = var_dgammaprime_dvd_dn1;
        *var_dgammaprime_dvd_dn2_slot = var_dgammaprime_dvd_dn2;
        *var_dgammaprime_dvd_dn3_slot = var_dgammaprime_dvd_dn3;
        *var_dgammaprime_dvd_rv_slot = var_dgammaprime_dvd_rv;
        *var_dgammaprime_dvs_slot = var_dgammaprime_dvs;
        *var_dgammaprime_dvs_dn0_slot = var_dgammaprime_dvs_dn0;
        *var_dgammaprime_dvs_dn1_slot = var_dgammaprime_dvs_dn1;
        *var_dgammaprime_dvs_dn2_slot = var_dgammaprime_dvs_dn2;
        *var_dgammaprime_dvs_dn3_slot = var_dgammaprime_dvs_dn3;
        *var_dgammaprime_dvs_rv_slot = var_dgammaprime_dvs_rv;
        *var_dif_dvd_slot = var_dif_dvd;
        *var_dif_dvd_dn0_slot = var_dif_dvd_dn0;
        *var_dif_dvd_dn1_slot = var_dif_dvd_dn1;
        *var_dif_dvd_dn2_slot = var_dif_dvd_dn2;
        *var_dif_dvd_dn3_slot = var_dif_dvd_dn3;
        *var_dif_dvd_rv_slot = var_dif_dvd_rv;
        *var_dif_dvs_slot = var_dif_dvs;
        *var_dif_dvs_dn0_slot = var_dif_dvs_dn0;
        *var_dif_dvs_dn1_slot = var_dif_dvs_dn1;
        *var_dif_dvs_dn2_slot = var_dif_dvs_dn2;
        *var_dif_dvs_dn3_slot = var_dif_dvs_dn3;
        *var_dif_dvs_rv_slot = var_dif_dvs_rv;
        *var_dirprime_dvd_slot = var_dirprime_dvd;
        *var_dirprime_dvd_dn0_slot = var_dirprime_dvd_dn0;
        *var_dirprime_dvd_dn1_slot = var_dirprime_dvd_dn1;
        *var_dirprime_dvd_dn2_slot = var_dirprime_dvd_dn2;
        *var_dirprime_dvd_dn3_slot = var_dirprime_dvd_dn3;
        *var_dirprime_dvd_rv_slot = var_dirprime_dvd_rv;
        *var_dirprime_dvs_slot = var_dirprime_dvs;
        *var_dirprime_dvs_dn0_slot = var_dirprime_dvs_dn0;
        *var_dirprime_dvs_dn1_slot = var_dirprime_dvs_dn1;
        *var_dirprime_dvs_dn2_slot = var_dirprime_dvs_dn2;
        *var_dirprime_dvs_dn3_slot = var_dirprime_dvs_dn3;
        *var_dirprime_dvs_rv_slot = var_dirprime_dvs_rv;
        *var_dvdss_dvd_slot = var_dvdss_dvd;
        *var_dvdss_dvd_dn0_slot = var_dvdss_dvd_dn0;
        *var_dvdss_dvd_dn1_slot = var_dvdss_dvd_dn1;
        *var_dvdss_dvd_dn2_slot = var_dvdss_dvd_dn2;
        *var_dvdss_dvd_dn3_slot = var_dvdss_dvd_dn3;
        *var_dvdss_dvd_rv_slot = var_dvdss_dvd_rv;
        *var_dvdss_dvs_slot = var_dvdss_dvs;
        *var_dvdss_dvs_dn0_slot = var_dvdss_dvs_dn0;
        *var_dvdss_dvs_dn1_slot = var_dvdss_dvs_dn1;
        *var_dvdss_dvs_dn2_slot = var_dvdss_dvs_dn2;
        *var_dvdss_dvs_dn3_slot = var_dvdss_dvs_dn3;
        *var_dvdss_dvs_rv_slot = var_dvdss_dvs_rv;
        *var_dvdssprime_dvd_slot = var_dvdssprime_dvd;
        *var_dvdssprime_dvd_dn0_slot = var_dvdssprime_dvd_dn0;
        *var_dvdssprime_dvd_dn1_slot = var_dvdssprime_dvd_dn1;
        *var_dvdssprime_dvd_dn2_slot = var_dvdssprime_dvd_dn2;
        *var_dvdssprime_dvd_dn3_slot = var_dvdssprime_dvd_dn3;
        *var_dvdssprime_dvd_rv_slot = var_dvdssprime_dvd_rv;
        *var_dvdssprime_dvs_slot = var_dvdssprime_dvs;
        *var_dvdssprime_dvs_dn0_slot = var_dvdssprime_dvs_dn0;
        *var_dvdssprime_dvs_dn1_slot = var_dvdssprime_dvs_dn1;
        *var_dvdssprime_dvs_dn2_slot = var_dvdssprime_dvs_dn2;
        *var_dvdssprime_dvs_dn3_slot = var_dvdssprime_dvs_dn3;
        *var_dvdssprime_dvs_rv_slot = var_dvdssprime_dvs_rv;
        *var_dvip_dvd_slot = var_dvip_dvd;
        *var_dvip_dvd_dn0_slot = var_dvip_dvd_dn0;
        *var_dvip_dvd_dn1_slot = var_dvip_dvd_dn1;
        *var_dvip_dvd_dn2_slot = var_dvip_dvd_dn2;
        *var_dvip_dvd_dn3_slot = var_dvip_dvd_dn3;
        *var_dvip_dvd_rv_slot = var_dvip_dvd_rv;
        *var_dvip_dvs_slot = var_dvip_dvs;
        *var_dvip_dvs_dn0_slot = var_dvip_dvs_dn0;
        *var_dvip_dvs_dn1_slot = var_dvip_dvs_dn1;
        *var_dvip_dvs_dn2_slot = var_dvip_dvs_dn2;
        *var_dvip_dvs_dn3_slot = var_dvip_dvs_dn3;
        *var_dvip_dvs_rv_slot = var_dvip_dvs_rv;
        *var_dvp_dvd_slot = var_dvp_dvd;
        *var_dvp_dvd_dn0_slot = var_dvp_dvd_dn0;
        *var_dvp_dvd_dn1_slot = var_dvp_dvd_dn1;
        *var_dvp_dvd_dn2_slot = var_dvp_dvd_dn2;
        *var_dvp_dvd_dn3_slot = var_dvp_dvd_dn3;
        *var_dvp_dvd_rv_slot = var_dvp_dvd_rv;
        *var_dvp_dvs_slot = var_dvp_dvs;
        *var_dvp_dvs_dn0_slot = var_dvp_dvs_dn0;
        *var_dvp_dvs_dn1_slot = var_dvp_dvs_dn1;
        *var_dvp_dvs_dn2_slot = var_dvp_dvs_dn2;
        *var_dvp_dvs_dn3_slot = var_dvp_dvs_dn3;
        *var_dvp_dvs_rv_slot = var_dvp_dvs_rv;
        *var_e0_q_1_slot = var_e0_q_1;
        *var_e0_q_1_dn0_slot = var_e0_q_1_dn0;
        *var_e0_q_1_dn1_slot = var_e0_q_1_dn1;
        *var_e0_q_1_dn2_slot = var_e0_q_1_dn2;
        *var_e0_q_1_dn3_slot = var_e0_q_1_dn3;
        *var_e0_q_1_rv_slot = var_e0_q_1_rv;
        *var_guard16_slot = var_guard16;
        *var_guard16_rv_slot = var_guard16_rv;
        *var_guard17_slot = var_guard17;
        *var_guard17_rv_slot = var_guard17_rv;
        *var_if_ir_slot = var_if_ir;
        *var_if_ir_dn0_slot = var_if_ir_dn0;
        *var_if_ir_dn1_slot = var_if_ir_dn1;
        *var_if_ir_dn2_slot = var_if_ir_dn2;
        *var_if_ir_dn3_slot = var_if_ir_dn3;
        *var_if_ir_rv_slot = var_if_ir_rv;
        *var_ispec_slot = var_ispec;
        *var_ispec_dn0_slot = var_ispec_dn0;
        *var_ispec_dn1_slot = var_ispec_dn1;
        *var_ispec_dn2_slot = var_ispec_dn2;
        *var_ispec_dn3_slot = var_ispec_dn3;
        *var_ispec_rv_slot = var_ispec_rv;
        *var_n_slot = var_n;
        *var_n_dn0_slot = var_n_dn0;
        *var_n_dn1_slot = var_n_dn1;
        *var_n_dn2_slot = var_n_dn2;
        *var_n_dn3_slot = var_n_dn3;
        *var_n_rv_slot = var_n_rv;
        *var_qb_slot = var_qb;
        *var_qb_dn0_slot = var_qb_dn0;
        *var_qb_dn1_slot = var_qb_dn1;
        *var_qb_dn2_slot = var_qb_dn2;
        *var_qb_dn3_slot = var_qb_dn3;
        *var_qb_rv_slot = var_qb_rv;
        *var_qi_slot = var_qi;
        *var_qi_dn0_slot = var_qi_dn0;
        *var_qi_dn1_slot = var_qi_dn1;
        *var_qi_dn2_slot = var_qi_dn2;
        *var_qi_dn3_slot = var_qi_dn3;
        *var_qi_rv_slot = var_qi_rv;
        *var_sqrt_phi_vp_slot = var_sqrt_phi_vp;
        *var_sqrt_phi_vp_dn0_slot = var_sqrt_phi_vp_dn0;
        *var_sqrt_phi_vp_dn1_slot = var_sqrt_phi_vp_dn1;
        *var_sqrt_phi_vp_dn2_slot = var_sqrt_phi_vp_dn2;
        *var_sqrt_phi_vp_dn3_slot = var_sqrt_phi_vp_dn3;
        *var_sqrt_phi_vp_rv_slot = var_sqrt_phi_vp_rv;
        *var_sqrt_vp_vt_slot = var_sqrt_vp_vt;
        *var_sqrt_vp_vt_dn0_slot = var_sqrt_vp_vt_dn0;
        *var_sqrt_vp_vt_dn1_slot = var_sqrt_vp_vt_dn1;
        *var_sqrt_vp_vt_dn2_slot = var_sqrt_vp_vt_dn2;
        *var_sqrt_vp_vt_dn3_slot = var_sqrt_vp_vt_dn3;
        *var_sqrt_vp_vt_rv_slot = var_sqrt_vp_vt_rv;
        *var_t0_gamma_1_slot = var_t0_gamma_1;
        *var_t0_gamma_1_dn0_slot = var_t0_gamma_1_dn0;
        *var_t0_gamma_1_dn1_slot = var_t0_gamma_1_dn1;
        *var_t0_gamma_1_dn2_slot = var_t0_gamma_1_dn2;
        *var_t0_gamma_1_dn3_slot = var_t0_gamma_1_dn3;
        *var_t0_gamma_1_rv_slot = var_t0_gamma_1_rv;
        *var_theta_vp_1_slot = var_theta_vp_1;
        *var_theta_vp_1_dn0_slot = var_theta_vp_1_dn0;
        *var_theta_vp_1_dn1_slot = var_theta_vp_1_dn1;
        *var_theta_vp_1_dn2_slot = var_theta_vp_1_dn2;
        *var_theta_vp_1_dn3_slot = var_theta_vp_1_dn3;
        *var_theta_vp_1_rv_slot = var_theta_vp_1_rv;
        *var_tmp1_slot = var_tmp1;
        *var_tmp1_dn0_slot = var_tmp1_dn0;
        *var_tmp1_dn1_slot = var_tmp1_dn1;
        *var_tmp1_dn2_slot = var_tmp1_dn2;
        *var_tmp1_dn3_slot = var_tmp1_dn3;
        *var_tmp1_rv_slot = var_tmp1_rv;
        *var_tmp2_slot = var_tmp2;
        *var_tmp2_dn0_slot = var_tmp2_dn0;
        *var_tmp2_dn1_slot = var_tmp2_dn1;
        *var_tmp2_dn2_slot = var_tmp2_dn2;
        *var_tmp2_dn3_slot = var_tmp2_dn3;
        *var_tmp2_rv_slot = var_tmp2_rv;
        *var_tmp3_slot = var_tmp3;
        *var_tmp3_dn0_slot = var_tmp3_dn0;
        *var_tmp3_dn1_slot = var_tmp3_dn1;
        *var_tmp3_dn2_slot = var_tmp3_dn2;
        *var_tmp3_dn3_slot = var_tmp3_dn3;
        *var_tmp3_rv_slot = var_tmp3_rv;
        *var_vpprime_slot = var_vpprime;
        *var_vpprime_dn0_slot = var_vpprime_dn0;
        *var_vpprime_dn1_slot = var_vpprime_dn1;
        *var_vpprime_dn2_slot = var_vpprime_dn2;
        *var_vpprime_dn3_slot = var_vpprime_dn3;
        *var_vpprime_rv_slot = var_vpprime_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        var_dif_dvd: f64,
        var_dif_dvd_dn0: f64,
        var_dif_dvd_dn1: f64,
        var_dif_dvd_dn2: f64,
        var_dif_dvd_dn3: f64,
        var_dif_dvs: f64,
        var_dif_dvs_dn0: f64,
        var_dif_dvs_dn1: f64,
        var_dif_dvs_dn2: f64,
        var_dif_dvs_dn3: f64,
        var_dir_dv: f64,
        var_dir_dv_dn0: f64,
        var_dir_dv_dn1: f64,
        var_dir_dv_dn2: f64,
        var_dir_dv_dn3: f64,
        var_dirprime_dvd: f64,
        var_dirprime_dvd_dn0: f64,
        var_dirprime_dvd_dn1: f64,
        var_dirprime_dvd_dn2: f64,
        var_dirprime_dvd_dn3: f64,
        var_dirprime_dvs: f64,
        var_dirprime_dvs_dn0: f64,
        var_dirprime_dvs_dn1: f64,
        var_dirprime_dvs_dn2: f64,
        var_dirprime_dvs_dn3: f64,
        var_dvip_dvd: f64,
        var_dvip_dvd_dn0: f64,
        var_dvip_dvd_dn1: f64,
        var_dvip_dvd_dn2: f64,
        var_dvip_dvd_dn3: f64,
        var_dvip_dvs: f64,
        var_dvip_dvs_dn0: f64,
        var_dvip_dvs_dn1: f64,
        var_dvip_dvs_dn2: f64,
        var_dvip_dvs_dn3: f64,
        var_dvp_dvd: f64,
        var_dvp_dvd_dn0: f64,
        var_dvp_dvd_dn1: f64,
        var_dvp_dvd_dn2: f64,
        var_dvp_dvd_dn3: f64,
        var_dvp_dvs: f64,
        var_dvp_dvs_dn0: f64,
        var_dvp_dvs_dn1: f64,
        var_dvp_dvs_dn2: f64,
        var_dvp_dvs_dn3: f64,
        var_e0_q_1: f64,
        var_e0_q_1_dn0: f64,
        var_e0_q_1_dn1: f64,
        var_e0_q_1_dn2: f64,
        var_e0_q_1_dn3: f64,
        var_eta_qi: f64,
        var_gamma_s: f64,
        var_gammaprime: f64,
        var_gammaprime_dn0: f64,
        var_gammaprime_dn1: f64,
        var_gammaprime_dn2: f64,
        var_gammaprime_dn3: f64,
        var_if_ir: f64,
        var_if_ir_dn0: f64,
        var_if_ir_dn1: f64,
        var_if_ir_dn2: f64,
        var_if_ir_dn3: f64,
        var_inv_ucrit: f64,
        var_inv_vt: f64,
        var_ispec: f64,
        var_ispec_dn0: f64,
        var_ispec_dn1: f64,
        var_ispec_dn2: f64,
        var_ispec_dn3: f64,
        var_lc_lambda: f64,
        var_lc_ucrit: f64,
        var_leff: f64,
        var_n: f64,
        var_n_1: f64,
        var_n_1_dn0: f64,
        var_n_1_dn1: f64,
        var_n_1_dn2: f64,
        var_n_1_dn3: f64,
        var_n_1_n: f64,
        var_n_1_n_dn0: f64,
        var_n_1_n_dn1: f64,
        var_n_1_n_dn2: f64,
        var_n_1_n_dn3: f64,
        var_n_dn0: f64,
        var_n_dn1: f64,
        var_n_dn2: f64,
        var_n_dn3: f64,
        var_phi_t: f64,
        var_phi_t_dn0: f64,
        var_phi_t_dn1: f64,
        var_phi_t_dn2: f64,
        var_phi_t_dn3: f64,
        var_qi: f64,
        var_qi_dn0: f64,
        var_qi_dn1: f64,
        var_qi_dn2: f64,
        var_qi_dn3: f64,
        var_sif: f64,
        var_sif2: f64,
        var_sif2_dn0: f64,
        var_sif2_dn1: f64,
        var_sif2_dn2: f64,
        var_sif2_dn3: f64,
        var_sif_dn0: f64,
        var_sif_dn1: f64,
        var_sif_dn2: f64,
        var_sif_dn3: f64,
        var_sif_sir_2: f64,
        var_sif_sir_2_dn0: f64,
        var_sif_sir_2_dn1: f64,
        var_sif_sir_2_dn2: f64,
        var_sif_sir_2_dn3: f64,
        var_sir: f64,
        var_sir2: f64,
        var_sir2_dn0: f64,
        var_sir2_dn1: f64,
        var_sir2_dn2: f64,
        var_sir2_dn3: f64,
        var_sir_dn0: f64,
        var_sir_dn1: f64,
        var_sir_dn2: f64,
        var_sir_dn3: f64,
        var_sqrt_lprime_lmin: f64,
        var_sqrt_lprime_lmin_dn0: f64,
        var_sqrt_lprime_lmin_dn1: f64,
        var_sqrt_lprime_lmin_dn2: f64,
        var_sqrt_lprime_lmin_dn3: f64,
        var_sqrt_phi_vp: f64,
        var_sqrt_phi_vp_dn0: f64,
        var_sqrt_phi_vp_dn1: f64,
        var_sqrt_phi_vp_dn2: f64,
        var_sqrt_phi_vp_dn3: f64,
        var_sqrt_vp_vt: f64,
        var_sqrt_vp_vt_dn0: f64,
        var_sqrt_vp_vt_dn1: f64,
        var_sqrt_vp_vt_dn2: f64,
        var_sqrt_vp_vt_dn3: f64,
        var_t0: f64,
        var_theta_vp_1: f64,
        var_theta_vp_1_dn0: f64,
        var_theta_vp_1_dn1: f64,
        var_theta_vp_1_dn2: f64,
        var_theta_vp_1_dn3: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn2: f64,
        var_vds_dn3: f64,
        var_vip: f64,
        var_vip_dn0: f64,
        var_vip_dn1: f64,
        var_vip_dn2: f64,
        var_vip_dn3: f64,
        var_vp: f64,
        var_vp_dn0: f64,
        var_vp_dn1: f64,
        var_vp_dn2: f64,
        var_vp_dn3: f64,
        var_vp_phi_eps: f64,
        var_vp_phi_eps_dn0: f64,
        var_vp_phi_eps_dn1: f64,
        var_vp_phi_eps_dn2: f64,
        var_vp_phi_eps_dn3: f64,
        var_vpprime: f64,
        var_vpprime_dn0: f64,
        var_vpprime_dn1: f64,
        var_vpprime_dn2: f64,
        var_vpprime_dn3: f64,
        var_vt: f64,
        var_vt_4: f64,
        var_weff: f64,
        var_dbeta_dvd_slot: &mut f64,
        var_dbeta_dvd_dn0_slot: &mut f64,
        var_dbeta_dvd_dn1_slot: &mut f64,
        var_dbeta_dvd_dn2_slot: &mut f64,
        var_dbeta_dvd_dn3_slot: &mut f64,
        var_dbeta_dvd_rv_slot: &mut f64,
        var_dbeta_dvs_slot: &mut f64,
        var_dbeta_dvs_dn0_slot: &mut f64,
        var_dbeta_dvs_dn1_slot: &mut f64,
        var_dbeta_dvs_dn2_slot: &mut f64,
        var_dbeta_dvs_dn3_slot: &mut f64,
        var_dbeta_dvs_rv_slot: &mut f64,
        var_ddeltal_dvd_slot: &mut f64,
        var_ddeltal_dvd_dn0_slot: &mut f64,
        var_ddeltal_dvd_dn1_slot: &mut f64,
        var_ddeltal_dvd_dn2_slot: &mut f64,
        var_ddeltal_dvd_dn3_slot: &mut f64,
        var_ddeltal_dvd_rv_slot: &mut f64,
        var_ddeltal_dvs_slot: &mut f64,
        var_ddeltal_dvs_dn0_slot: &mut f64,
        var_ddeltal_dvs_dn1_slot: &mut f64,
        var_ddeltal_dvs_dn2_slot: &mut f64,
        var_ddeltal_dvs_dn3_slot: &mut f64,
        var_ddeltal_dvs_rv_slot: &mut f64,
        var_dir_dvd_slot: &mut f64,
        var_dir_dvd_dn0_slot: &mut f64,
        var_dir_dvd_dn1_slot: &mut f64,
        var_dir_dvd_dn2_slot: &mut f64,
        var_dir_dvd_dn3_slot: &mut f64,
        var_dir_dvd_rv_slot: &mut f64,
        var_dir_dvs_slot: &mut f64,
        var_dir_dvs_dn0_slot: &mut f64,
        var_dir_dvs_dn1_slot: &mut f64,
        var_dir_dvs_dn2_slot: &mut f64,
        var_dir_dvs_dn3_slot: &mut f64,
        var_dir_dvs_rv_slot: &mut f64,
        var_dleq_dvd_slot: &mut f64,
        var_dleq_dvd_dn0_slot: &mut f64,
        var_dleq_dvd_dn1_slot: &mut f64,
        var_dleq_dvd_dn2_slot: &mut f64,
        var_dleq_dvd_dn3_slot: &mut f64,
        var_dleq_dvd_rv_slot: &mut f64,
        var_dleq_dvs_slot: &mut f64,
        var_dleq_dvs_dn0_slot: &mut f64,
        var_dleq_dvs_dn1_slot: &mut f64,
        var_dleq_dvs_dn2_slot: &mut f64,
        var_dleq_dvs_dn3_slot: &mut f64,
        var_dleq_dvs_rv_slot: &mut f64,
        var_dn_dvd_slot: &mut f64,
        var_dn_dvd_dn0_slot: &mut f64,
        var_dn_dvd_dn1_slot: &mut f64,
        var_dn_dvd_dn2_slot: &mut f64,
        var_dn_dvd_dn3_slot: &mut f64,
        var_dn_dvd_rv_slot: &mut f64,
        var_dn_dvs_slot: &mut f64,
        var_dn_dvs_dn0_slot: &mut f64,
        var_dn_dvs_dn1_slot: &mut f64,
        var_dn_dvs_dn2_slot: &mut f64,
        var_dn_dvs_dn3_slot: &mut f64,
        var_dn_dvs_rv_slot: &mut f64,
        var_dqb_dvd_slot: &mut f64,
        var_dqb_dvd_dn0_slot: &mut f64,
        var_dqb_dvd_dn1_slot: &mut f64,
        var_dqb_dvd_dn2_slot: &mut f64,
        var_dqb_dvd_dn3_slot: &mut f64,
        var_dqb_dvd_rv_slot: &mut f64,
        var_dqb_dvs_slot: &mut f64,
        var_dqb_dvs_dn0_slot: &mut f64,
        var_dqb_dvs_dn1_slot: &mut f64,
        var_dqb_dvs_dn2_slot: &mut f64,
        var_dqb_dvs_dn3_slot: &mut f64,
        var_dqb_dvs_rv_slot: &mut f64,
        var_dqi_dvd_slot: &mut f64,
        var_dqi_dvd_dn0_slot: &mut f64,
        var_dqi_dvd_dn1_slot: &mut f64,
        var_dqi_dvd_dn2_slot: &mut f64,
        var_dqi_dvd_dn3_slot: &mut f64,
        var_dqi_dvd_rv_slot: &mut f64,
        var_dqi_dvs_slot: &mut f64,
        var_dqi_dvs_dn0_slot: &mut f64,
        var_dqi_dvs_dn1_slot: &mut f64,
        var_dqi_dvs_dn2_slot: &mut f64,
        var_dqi_dvs_dn3_slot: &mut f64,
        var_dqi_dvs_rv_slot: &mut f64,
        var_dvpprime_dvd_slot: &mut f64,
        var_dvpprime_dvd_dn0_slot: &mut f64,
        var_dvpprime_dvd_dn1_slot: &mut f64,
        var_dvpprime_dvd_dn2_slot: &mut f64,
        var_dvpprime_dvd_dn3_slot: &mut f64,
        var_dvpprime_dvd_rv_slot: &mut f64,
        var_dvpprime_dvs_slot: &mut f64,
        var_dvpprime_dvs_dn0_slot: &mut f64,
        var_dvpprime_dvs_dn1_slot: &mut f64,
        var_dvpprime_dvs_dn2_slot: &mut f64,
        var_dvpprime_dvs_dn3_slot: &mut f64,
        var_dvpprime_dvs_rv_slot: &mut f64,
        var_gds_slot: &mut f64,
        var_gds_dn0_slot: &mut f64,
        var_gds_dn1_slot: &mut f64,
        var_gds_dn2_slot: &mut f64,
        var_gds_dn3_slot: &mut f64,
        var_gds_rv_slot: &mut f64,
        var_gms_slot: &mut f64,
        var_gms_dn0_slot: &mut f64,
        var_gms_dn1_slot: &mut f64,
        var_gms_dn2_slot: &mut f64,
        var_gms_dn3_slot: &mut f64,
        var_gms_rv_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_guard18_rv_slot: &mut f64,
        var_n_vt_cox_slot: &mut f64,
        var_n_vt_cox_dn0_slot: &mut f64,
        var_n_vt_cox_dn1_slot: &mut f64,
        var_n_vt_cox_dn2_slot: &mut f64,
        var_n_vt_cox_dn3_slot: &mut f64,
        var_n_vt_cox_rv_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn1_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn3_slot: &mut f64,
        var_qd_rv_slot: &mut f64,
        var_qi_1_slot: &mut f64,
        var_qi_1_dn0_slot: &mut f64,
        var_qi_1_dn1_slot: &mut f64,
        var_qi_1_dn2_slot: &mut f64,
        var_qi_1_dn3_slot: &mut f64,
        var_qi_1_rv_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn0_slot: &mut f64,
        var_qs_dn1_slot: &mut f64,
        var_qs_dn2_slot: &mut f64,
        var_qs_dn3_slot: &mut f64,
        var_qs_rv_slot: &mut f64,
        var_rdeff_slot: &mut f64,
        var_rdeff_rv_slot: &mut f64,
        var_rseff_slot: &mut f64,
        var_rseff_rv_slot: &mut f64,
        var_sif3_slot: &mut f64,
        var_sif3_dn0_slot: &mut f64,
        var_sif3_dn1_slot: &mut f64,
        var_sif3_dn2_slot: &mut f64,
        var_sif3_dn3_slot: &mut f64,
        var_sif3_rv_slot: &mut f64,
        var_sir3_slot: &mut f64,
        var_sir3_dn0_slot: &mut f64,
        var_sir3_dn1_slot: &mut f64,
        var_sir3_dn2_slot: &mut f64,
        var_sir3_dn3_slot: &mut f64,
        var_sir3_rv_slot: &mut f64,
        var_sqrt_phi_vp2_2_slot: &mut f64,
        var_sqrt_phi_vp2_2_dn0_slot: &mut f64,
        var_sqrt_phi_vp2_2_dn1_slot: &mut f64,
        var_sqrt_phi_vp2_2_dn2_slot: &mut f64,
        var_sqrt_phi_vp2_2_dn3_slot: &mut f64,
        var_sqrt_phi_vp2_2_rv_slot: &mut f64,
        var_tmp1_slot: &mut f64,
        var_tmp1_dn0_slot: &mut f64,
        var_tmp1_dn1_slot: &mut f64,
        var_tmp1_dn2_slot: &mut f64,
        var_tmp1_dn3_slot: &mut f64,
        var_tmp1_rv_slot: &mut f64,
        var_tmp2_slot: &mut f64,
        var_tmp2_dn0_slot: &mut f64,
        var_tmp2_dn1_slot: &mut f64,
        var_tmp2_dn2_slot: &mut f64,
        var_tmp2_dn3_slot: &mut f64,
        var_tmp2_rv_slot: &mut f64,
        var_tmp3_slot: &mut f64,
        var_tmp3_dn0_slot: &mut f64,
        var_tmp3_dn1_slot: &mut f64,
        var_tmp3_dn2_slot: &mut f64,
        var_tmp3_dn3_slot: &mut f64,
        var_tmp3_rv_slot: &mut f64,
        var_wlcox_slot: &mut f64,
        var_wlcox_rv_slot: &mut f64,
    ) {
        let mut var_dbeta_dvd: f64 = *var_dbeta_dvd_slot;
        let mut var_dbeta_dvd_dn0: f64 = *var_dbeta_dvd_dn0_slot;
        let mut var_dbeta_dvd_dn1: f64 = *var_dbeta_dvd_dn1_slot;
        let mut var_dbeta_dvd_dn2: f64 = *var_dbeta_dvd_dn2_slot;
        let mut var_dbeta_dvd_dn3: f64 = *var_dbeta_dvd_dn3_slot;
        let mut var_dbeta_dvd_rv: f64 = *var_dbeta_dvd_rv_slot;
        let mut var_dbeta_dvs: f64 = *var_dbeta_dvs_slot;
        let mut var_dbeta_dvs_dn0: f64 = *var_dbeta_dvs_dn0_slot;
        let mut var_dbeta_dvs_dn1: f64 = *var_dbeta_dvs_dn1_slot;
        let mut var_dbeta_dvs_dn2: f64 = *var_dbeta_dvs_dn2_slot;
        let mut var_dbeta_dvs_dn3: f64 = *var_dbeta_dvs_dn3_slot;
        let mut var_dbeta_dvs_rv: f64 = *var_dbeta_dvs_rv_slot;
        let mut var_ddeltal_dvd: f64 = *var_ddeltal_dvd_slot;
        let mut var_ddeltal_dvd_dn0: f64 = *var_ddeltal_dvd_dn0_slot;
        let mut var_ddeltal_dvd_dn1: f64 = *var_ddeltal_dvd_dn1_slot;
        let mut var_ddeltal_dvd_dn2: f64 = *var_ddeltal_dvd_dn2_slot;
        let mut var_ddeltal_dvd_dn3: f64 = *var_ddeltal_dvd_dn3_slot;
        let mut var_ddeltal_dvd_rv: f64 = *var_ddeltal_dvd_rv_slot;
        let mut var_ddeltal_dvs: f64 = *var_ddeltal_dvs_slot;
        let mut var_ddeltal_dvs_dn0: f64 = *var_ddeltal_dvs_dn0_slot;
        let mut var_ddeltal_dvs_dn1: f64 = *var_ddeltal_dvs_dn1_slot;
        let mut var_ddeltal_dvs_dn2: f64 = *var_ddeltal_dvs_dn2_slot;
        let mut var_ddeltal_dvs_dn3: f64 = *var_ddeltal_dvs_dn3_slot;
        let mut var_ddeltal_dvs_rv: f64 = *var_ddeltal_dvs_rv_slot;
        let mut var_dir_dvd: f64 = *var_dir_dvd_slot;
        let mut var_dir_dvd_dn0: f64 = *var_dir_dvd_dn0_slot;
        let mut var_dir_dvd_dn1: f64 = *var_dir_dvd_dn1_slot;
        let mut var_dir_dvd_dn2: f64 = *var_dir_dvd_dn2_slot;
        let mut var_dir_dvd_dn3: f64 = *var_dir_dvd_dn3_slot;
        let mut var_dir_dvd_rv: f64 = *var_dir_dvd_rv_slot;
        let mut var_dir_dvs: f64 = *var_dir_dvs_slot;
        let mut var_dir_dvs_dn0: f64 = *var_dir_dvs_dn0_slot;
        let mut var_dir_dvs_dn1: f64 = *var_dir_dvs_dn1_slot;
        let mut var_dir_dvs_dn2: f64 = *var_dir_dvs_dn2_slot;
        let mut var_dir_dvs_dn3: f64 = *var_dir_dvs_dn3_slot;
        let mut var_dir_dvs_rv: f64 = *var_dir_dvs_rv_slot;
        let mut var_dleq_dvd: f64 = *var_dleq_dvd_slot;
        let mut var_dleq_dvd_dn0: f64 = *var_dleq_dvd_dn0_slot;
        let mut var_dleq_dvd_dn1: f64 = *var_dleq_dvd_dn1_slot;
        let mut var_dleq_dvd_dn2: f64 = *var_dleq_dvd_dn2_slot;
        let mut var_dleq_dvd_dn3: f64 = *var_dleq_dvd_dn3_slot;
        let mut var_dleq_dvd_rv: f64 = *var_dleq_dvd_rv_slot;
        let mut var_dleq_dvs: f64 = *var_dleq_dvs_slot;
        let mut var_dleq_dvs_dn0: f64 = *var_dleq_dvs_dn0_slot;
        let mut var_dleq_dvs_dn1: f64 = *var_dleq_dvs_dn1_slot;
        let mut var_dleq_dvs_dn2: f64 = *var_dleq_dvs_dn2_slot;
        let mut var_dleq_dvs_dn3: f64 = *var_dleq_dvs_dn3_slot;
        let mut var_dleq_dvs_rv: f64 = *var_dleq_dvs_rv_slot;
        let mut var_dn_dvd: f64 = *var_dn_dvd_slot;
        let mut var_dn_dvd_dn0: f64 = *var_dn_dvd_dn0_slot;
        let mut var_dn_dvd_dn1: f64 = *var_dn_dvd_dn1_slot;
        let mut var_dn_dvd_dn2: f64 = *var_dn_dvd_dn2_slot;
        let mut var_dn_dvd_dn3: f64 = *var_dn_dvd_dn3_slot;
        let mut var_dn_dvd_rv: f64 = *var_dn_dvd_rv_slot;
        let mut var_dn_dvs: f64 = *var_dn_dvs_slot;
        let mut var_dn_dvs_dn0: f64 = *var_dn_dvs_dn0_slot;
        let mut var_dn_dvs_dn1: f64 = *var_dn_dvs_dn1_slot;
        let mut var_dn_dvs_dn2: f64 = *var_dn_dvs_dn2_slot;
        let mut var_dn_dvs_dn3: f64 = *var_dn_dvs_dn3_slot;
        let mut var_dn_dvs_rv: f64 = *var_dn_dvs_rv_slot;
        let mut var_dqb_dvd: f64 = *var_dqb_dvd_slot;
        let mut var_dqb_dvd_dn0: f64 = *var_dqb_dvd_dn0_slot;
        let mut var_dqb_dvd_dn1: f64 = *var_dqb_dvd_dn1_slot;
        let mut var_dqb_dvd_dn2: f64 = *var_dqb_dvd_dn2_slot;
        let mut var_dqb_dvd_dn3: f64 = *var_dqb_dvd_dn3_slot;
        let mut var_dqb_dvd_rv: f64 = *var_dqb_dvd_rv_slot;
        let mut var_dqb_dvs: f64 = *var_dqb_dvs_slot;
        let mut var_dqb_dvs_dn0: f64 = *var_dqb_dvs_dn0_slot;
        let mut var_dqb_dvs_dn1: f64 = *var_dqb_dvs_dn1_slot;
        let mut var_dqb_dvs_dn2: f64 = *var_dqb_dvs_dn2_slot;
        let mut var_dqb_dvs_dn3: f64 = *var_dqb_dvs_dn3_slot;
        let mut var_dqb_dvs_rv: f64 = *var_dqb_dvs_rv_slot;
        let mut var_dqi_dvd: f64 = *var_dqi_dvd_slot;
        let mut var_dqi_dvd_dn0: f64 = *var_dqi_dvd_dn0_slot;
        let mut var_dqi_dvd_dn1: f64 = *var_dqi_dvd_dn1_slot;
        let mut var_dqi_dvd_dn2: f64 = *var_dqi_dvd_dn2_slot;
        let mut var_dqi_dvd_dn3: f64 = *var_dqi_dvd_dn3_slot;
        let mut var_dqi_dvd_rv: f64 = *var_dqi_dvd_rv_slot;
        let mut var_dqi_dvs: f64 = *var_dqi_dvs_slot;
        let mut var_dqi_dvs_dn0: f64 = *var_dqi_dvs_dn0_slot;
        let mut var_dqi_dvs_dn1: f64 = *var_dqi_dvs_dn1_slot;
        let mut var_dqi_dvs_dn2: f64 = *var_dqi_dvs_dn2_slot;
        let mut var_dqi_dvs_dn3: f64 = *var_dqi_dvs_dn3_slot;
        let mut var_dqi_dvs_rv: f64 = *var_dqi_dvs_rv_slot;
        let mut var_dvpprime_dvd: f64 = *var_dvpprime_dvd_slot;
        let mut var_dvpprime_dvd_dn0: f64 = *var_dvpprime_dvd_dn0_slot;
        let mut var_dvpprime_dvd_dn1: f64 = *var_dvpprime_dvd_dn1_slot;
        let mut var_dvpprime_dvd_dn2: f64 = *var_dvpprime_dvd_dn2_slot;
        let mut var_dvpprime_dvd_dn3: f64 = *var_dvpprime_dvd_dn3_slot;
        let mut var_dvpprime_dvd_rv: f64 = *var_dvpprime_dvd_rv_slot;
        let mut var_dvpprime_dvs: f64 = *var_dvpprime_dvs_slot;
        let mut var_dvpprime_dvs_dn0: f64 = *var_dvpprime_dvs_dn0_slot;
        let mut var_dvpprime_dvs_dn1: f64 = *var_dvpprime_dvs_dn1_slot;
        let mut var_dvpprime_dvs_dn2: f64 = *var_dvpprime_dvs_dn2_slot;
        let mut var_dvpprime_dvs_dn3: f64 = *var_dvpprime_dvs_dn3_slot;
        let mut var_dvpprime_dvs_rv: f64 = *var_dvpprime_dvs_rv_slot;
        let mut var_gds: f64 = *var_gds_slot;
        let mut var_gds_dn0: f64 = *var_gds_dn0_slot;
        let mut var_gds_dn1: f64 = *var_gds_dn1_slot;
        let mut var_gds_dn2: f64 = *var_gds_dn2_slot;
        let mut var_gds_dn3: f64 = *var_gds_dn3_slot;
        let mut var_gds_rv: f64 = *var_gds_rv_slot;
        let mut var_gms: f64 = *var_gms_slot;
        let mut var_gms_dn0: f64 = *var_gms_dn0_slot;
        let mut var_gms_dn1: f64 = *var_gms_dn1_slot;
        let mut var_gms_dn2: f64 = *var_gms_dn2_slot;
        let mut var_gms_dn3: f64 = *var_gms_dn3_slot;
        let mut var_gms_rv: f64 = *var_gms_rv_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_guard18_rv: f64 = *var_guard18_rv_slot;
        let mut var_n_vt_cox: f64 = *var_n_vt_cox_slot;
        let mut var_n_vt_cox_dn0: f64 = *var_n_vt_cox_dn0_slot;
        let mut var_n_vt_cox_dn1: f64 = *var_n_vt_cox_dn1_slot;
        let mut var_n_vt_cox_dn2: f64 = *var_n_vt_cox_dn2_slot;
        let mut var_n_vt_cox_dn3: f64 = *var_n_vt_cox_dn3_slot;
        let mut var_n_vt_cox_rv: f64 = *var_n_vt_cox_rv_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn1: f64 = *var_qd_dn1_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn3: f64 = *var_qd_dn3_slot;
        let mut var_qd_rv: f64 = *var_qd_rv_slot;
        let mut var_qi_1: f64 = *var_qi_1_slot;
        let mut var_qi_1_dn0: f64 = *var_qi_1_dn0_slot;
        let mut var_qi_1_dn1: f64 = *var_qi_1_dn1_slot;
        let mut var_qi_1_dn2: f64 = *var_qi_1_dn2_slot;
        let mut var_qi_1_dn3: f64 = *var_qi_1_dn3_slot;
        let mut var_qi_1_rv: f64 = *var_qi_1_rv_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn0: f64 = *var_qs_dn0_slot;
        let mut var_qs_dn1: f64 = *var_qs_dn1_slot;
        let mut var_qs_dn2: f64 = *var_qs_dn2_slot;
        let mut var_qs_dn3: f64 = *var_qs_dn3_slot;
        let mut var_qs_rv: f64 = *var_qs_rv_slot;
        let mut var_rdeff: f64 = *var_rdeff_slot;
        let mut var_rdeff_rv: f64 = *var_rdeff_rv_slot;
        let mut var_rseff: f64 = *var_rseff_slot;
        let mut var_rseff_rv: f64 = *var_rseff_rv_slot;
        let mut var_sif3: f64 = *var_sif3_slot;
        let mut var_sif3_dn0: f64 = *var_sif3_dn0_slot;
        let mut var_sif3_dn1: f64 = *var_sif3_dn1_slot;
        let mut var_sif3_dn2: f64 = *var_sif3_dn2_slot;
        let mut var_sif3_dn3: f64 = *var_sif3_dn3_slot;
        let mut var_sif3_rv: f64 = *var_sif3_rv_slot;
        let mut var_sir3: f64 = *var_sir3_slot;
        let mut var_sir3_dn0: f64 = *var_sir3_dn0_slot;
        let mut var_sir3_dn1: f64 = *var_sir3_dn1_slot;
        let mut var_sir3_dn2: f64 = *var_sir3_dn2_slot;
        let mut var_sir3_dn3: f64 = *var_sir3_dn3_slot;
        let mut var_sir3_rv: f64 = *var_sir3_rv_slot;
        let mut var_sqrt_phi_vp2_2: f64 = *var_sqrt_phi_vp2_2_slot;
        let mut var_sqrt_phi_vp2_2_dn0: f64 = *var_sqrt_phi_vp2_2_dn0_slot;
        let mut var_sqrt_phi_vp2_2_dn1: f64 = *var_sqrt_phi_vp2_2_dn1_slot;
        let mut var_sqrt_phi_vp2_2_dn2: f64 = *var_sqrt_phi_vp2_2_dn2_slot;
        let mut var_sqrt_phi_vp2_2_dn3: f64 = *var_sqrt_phi_vp2_2_dn3_slot;
        let mut var_sqrt_phi_vp2_2_rv: f64 = *var_sqrt_phi_vp2_2_rv_slot;
        let mut var_tmp1: f64 = *var_tmp1_slot;
        let mut var_tmp1_dn0: f64 = *var_tmp1_dn0_slot;
        let mut var_tmp1_dn1: f64 = *var_tmp1_dn1_slot;
        let mut var_tmp1_dn2: f64 = *var_tmp1_dn2_slot;
        let mut var_tmp1_dn3: f64 = *var_tmp1_dn3_slot;
        let mut var_tmp1_rv: f64 = *var_tmp1_rv_slot;
        let mut var_tmp2: f64 = *var_tmp2_slot;
        let mut var_tmp2_dn0: f64 = *var_tmp2_dn0_slot;
        let mut var_tmp2_dn1: f64 = *var_tmp2_dn1_slot;
        let mut var_tmp2_dn2: f64 = *var_tmp2_dn2_slot;
        let mut var_tmp2_dn3: f64 = *var_tmp2_dn3_slot;
        let mut var_tmp2_rv: f64 = *var_tmp2_rv_slot;
        let mut var_tmp3: f64 = *var_tmp3_slot;
        let mut var_tmp3_dn0: f64 = *var_tmp3_dn0_slot;
        let mut var_tmp3_dn1: f64 = *var_tmp3_dn1_slot;
        let mut var_tmp3_dn2: f64 = *var_tmp3_dn2_slot;
        let mut var_tmp3_dn3: f64 = *var_tmp3_dn3_slot;
        let mut var_tmp3_rv: f64 = *var_tmp3_rv_slot;
        let mut var_wlcox: f64 = *var_wlcox_slot;
        let mut var_wlcox_rv: f64 = *var_wlcox_rv_slot;

        let assign2200_e1719: f64 = (var_lc_ucrit + var_vds);
        let assign2200_e1721: f64 = (assign2200_e1719 - var_vip);
        let assign2200_e1722: f64 = (var_lc_lambda / assign2200_e1721);
        var_tmp1 = assign2200_e1722;
        var_tmp1_dn0 = (-((var_lc_lambda * (var_vds_dn0 - var_vip_dn0)) / (assign2200_e1721 * assign2200_e1721)));
        var_tmp1_dn1 = (-((var_lc_lambda * (-var_vip_dn1)) / (assign2200_e1721 * assign2200_e1721)));
        var_tmp1_dn2 = (-((var_lc_lambda * (var_vds_dn2 - var_vip_dn2)) / (assign2200_e1721 * assign2200_e1721)));
        var_tmp1_dn3 = (-((var_lc_lambda * (var_vds_dn3 - var_vip_dn3)) / (assign2200_e1721 * assign2200_e1721)));
        var_tmp1_rv = 0.0;

        let assign2210_e1726: f64 = (0.5 - var_dvip_dvd);
        let assign2210_e1727: f64 = (var_tmp1 * assign2210_e1726);
        var_ddeltal_dvd = assign2210_e1727;
        var_ddeltal_dvd_dn0 = ((var_tmp1_dn0 * assign2210_e1726) + (var_tmp1 * (-var_dvip_dvd_dn0)));
        var_ddeltal_dvd_dn1 = ((var_tmp1_dn1 * assign2210_e1726) + (var_tmp1 * (-var_dvip_dvd_dn1)));
        var_ddeltal_dvd_dn2 = ((var_tmp1_dn2 * assign2210_e1726) + (var_tmp1 * (-var_dvip_dvd_dn2)));
        var_ddeltal_dvd_dn3 = ((var_tmp1_dn3 * assign2210_e1726) + (var_tmp1 * (-var_dvip_dvd_dn3)));
        var_ddeltal_dvd_rv = 0.0;

        let assign2220_e1730: f64 = (-0.5);
        let assign2220_e1732: f64 = (assign2220_e1730 - var_dvip_dvs);
        let assign2220_e1733: f64 = (var_tmp1 * assign2220_e1732);
        var_ddeltal_dvs = assign2220_e1733;
        var_ddeltal_dvs_dn0 = ((var_tmp1_dn0 * assign2220_e1732) + (var_tmp1 * (-var_dvip_dvs_dn0)));
        var_ddeltal_dvs_dn1 = ((var_tmp1_dn1 * assign2220_e1732) + (var_tmp1 * (-var_dvip_dvs_dn1)));
        var_ddeltal_dvs_dn2 = ((var_tmp1_dn2 * assign2220_e1732) + (var_tmp1 * (-var_dvip_dvs_dn2)));
        var_ddeltal_dvs_dn3 = ((var_tmp1_dn3 * assign2220_e1732) + (var_tmp1 * (-var_dvip_dvs_dn3)));
        var_ddeltal_dvs_rv = 0.0;

        let assign2240_e1740: f64 = (1.0 / var_sqrt_lprime_lmin);
        var_tmp1 = assign2240_e1740;
        var_tmp1_dn0 = (-(var_sqrt_lprime_lmin_dn0 / (var_sqrt_lprime_lmin * var_sqrt_lprime_lmin)));
        var_tmp1_dn1 = (-(var_sqrt_lprime_lmin_dn1 / (var_sqrt_lprime_lmin * var_sqrt_lprime_lmin)));
        var_tmp1_dn2 = (-(var_sqrt_lprime_lmin_dn2 / (var_sqrt_lprime_lmin * var_sqrt_lprime_lmin)));
        var_tmp1_dn3 = (-(var_sqrt_lprime_lmin_dn3 / (var_sqrt_lprime_lmin * var_sqrt_lprime_lmin)));
        var_tmp1_rv = 0.0;

        let assign2250_e1743: f64 = (-var_ddeltal_dvd);
        let assign2250_e1746: f64 = (0.5 + var_dvip_dvd);
        let assign2250_e1748: f64 = (assign2250_e1746 * var_inv_ucrit);
        let assign2250_e1749: f64 = (assign2250_e1743 + assign2250_e1748);
        let assign2250_e1750: f64 = (var_tmp1 * assign2250_e1749);
        var_dleq_dvd = assign2250_e1750;
        var_dleq_dvd_dn0 = ((var_tmp1_dn0 * assign2250_e1749) + (var_tmp1 * ((-var_ddeltal_dvd_dn0) + (var_dvip_dvd_dn0 * var_inv_ucrit))));
        var_dleq_dvd_dn1 = ((var_tmp1_dn1 * assign2250_e1749) + (var_tmp1 * ((-var_ddeltal_dvd_dn1) + (var_dvip_dvd_dn1 * var_inv_ucrit))));
        var_dleq_dvd_dn2 = ((var_tmp1_dn2 * assign2250_e1749) + (var_tmp1 * ((-var_ddeltal_dvd_dn2) + (var_dvip_dvd_dn2 * var_inv_ucrit))));
        var_dleq_dvd_dn3 = ((var_tmp1_dn3 * assign2250_e1749) + (var_tmp1 * ((-var_ddeltal_dvd_dn3) + (var_dvip_dvd_dn3 * var_inv_ucrit))));
        var_dleq_dvd_rv = 0.0;

        let assign2260_e1753: f64 = (-var_ddeltal_dvs);
        let assign2260_e1755: f64 = (-0.5);
        let assign2260_e1757: f64 = (assign2260_e1755 + var_dvip_dvs);
        let assign2260_e1759: f64 = (assign2260_e1757 * var_inv_ucrit);
        let assign2260_e1760: f64 = (assign2260_e1753 + assign2260_e1759);
        let assign2260_e1761: f64 = (var_tmp1 * assign2260_e1760);
        var_dleq_dvs = assign2260_e1761;
        var_dleq_dvs_dn0 = ((var_tmp1_dn0 * assign2260_e1760) + (var_tmp1 * ((-var_ddeltal_dvs_dn0) + (var_dvip_dvs_dn0 * var_inv_ucrit))));
        var_dleq_dvs_dn1 = ((var_tmp1_dn1 * assign2260_e1760) + (var_tmp1 * ((-var_ddeltal_dvs_dn1) + (var_dvip_dvs_dn1 * var_inv_ucrit))));
        var_dleq_dvs_dn2 = ((var_tmp1_dn2 * assign2260_e1760) + (var_tmp1 * ((-var_ddeltal_dvs_dn2) + (var_dvip_dvs_dn2 * var_inv_ucrit))));
        var_dleq_dvs_dn3 = ((var_tmp1_dn3 * assign2260_e1760) + (var_tmp1 * ((-var_ddeltal_dvs_dn3) + (var_dvip_dvs_dn3 * var_inv_ucrit))));
        var_dleq_dvs_rv = 0.0;

        let assign2280_e1772: f64 = (var_dir_dv * var_inv_vt);
        var_tmp1 = assign2280_e1772;
        var_tmp1_dn0 = (var_dir_dv_dn0 * var_inv_vt);
        var_tmp1_dn1 = (var_dir_dv_dn1 * var_inv_vt);
        var_tmp1_dn2 = (var_dir_dv_dn2 * var_inv_vt);
        var_tmp1_dn3 = (var_dir_dv_dn3 * var_inv_vt);
        var_tmp1_rv = 0.0;

        let assign2290_e1776: f64 = (var_dvp_dvd - 1.0);
        let assign2290_e1777: f64 = (var_tmp1 * assign2290_e1776);
        var_dir_dvd = assign2290_e1777;
        var_dir_dvd_dn0 = ((var_tmp1_dn0 * assign2290_e1776) + (var_tmp1 * var_dvp_dvd_dn0));
        var_dir_dvd_dn1 = ((var_tmp1_dn1 * assign2290_e1776) + (var_tmp1 * var_dvp_dvd_dn1));
        var_dir_dvd_dn2 = ((var_tmp1_dn2 * assign2290_e1776) + (var_tmp1 * var_dvp_dvd_dn2));
        var_dir_dvd_dn3 = ((var_tmp1_dn3 * assign2290_e1776) + (var_tmp1 * var_dvp_dvd_dn3));
        var_dir_dvd_rv = 0.0;

        let assign2300_e1780: f64 = (var_tmp1 * var_dvp_dvs);
        var_dir_dvs = assign2300_e1780;
        var_dir_dvs_dn0 = ((var_tmp1_dn0 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn0));
        var_dir_dvs_dn1 = ((var_tmp1_dn1 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn1));
        var_dir_dvs_dn2 = ((var_tmp1_dn2 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn2));
        var_dir_dvs_dn3 = ((var_tmp1_dn3 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn3));
        var_dir_dvs_rv = 0.0;

        let assign2320_e1786: f64 = (1.0 + var_n_1);
        let assign2320_e1787: f64 = (-assign2320_e1786);
        let assign2320_e1789: f64 = (assign2320_e1787 * var_vt);
        let assign2320_e1791: f64 = (assign2320_e1789 * 0.66666666);
        let assign2320_e1793: f64 = (assign2320_e1791 / var_sif_sir_2);
        var_tmp1 = assign2320_e1793;
        var_tmp1_dn0 = ((((((-var_n_1_dn0) * var_vt) * 0.66666666) * var_sif_sir_2) - (assign2320_e1791 * var_sif_sir_2_dn0)) / (var_sif_sir_2 * var_sif_sir_2));
        var_tmp1_dn1 = ((((((-var_n_1_dn1) * var_vt) * 0.66666666) * var_sif_sir_2) - (assign2320_e1791 * var_sif_sir_2_dn1)) / (var_sif_sir_2 * var_sif_sir_2));
        var_tmp1_dn2 = ((((((-var_n_1_dn2) * var_vt) * 0.66666666) * var_sif_sir_2) - (assign2320_e1791 * var_sif_sir_2_dn2)) / (var_sif_sir_2 * var_sif_sir_2));
        var_tmp1_dn3 = ((((((-var_n_1_dn3) * var_vt) * 0.66666666) * var_sif_sir_2) - (assign2320_e1791 * var_sif_sir_2_dn3)) / (var_sif_sir_2 * var_sif_sir_2));
        var_tmp1_rv = 0.0;

        let assign2330_e1798: f64 = (2.0 * var_sir);
        let assign2330_e1799: f64 = (var_sif + assign2330_e1798);
        let assign2330_e1800: f64 = (var_tmp1 * assign2330_e1799);
        var_tmp2 = assign2330_e1800;
        var_tmp2_dn0 = ((var_tmp1_dn0 * assign2330_e1799) + (var_tmp1 * (var_sif_dn0 + (2.0 * var_sir_dn0))));
        var_tmp2_dn1 = ((var_tmp1_dn1 * assign2330_e1799) + (var_tmp1 * (var_sif_dn1 + (2.0 * var_sir_dn1))));
        var_tmp2_dn2 = ((var_tmp1_dn2 * assign2330_e1799) + (var_tmp1 * (var_sif_dn2 + (2.0 * var_sir_dn2))));
        var_tmp2_dn3 = ((var_tmp1_dn3 * assign2330_e1799) + (var_tmp1 * (var_sif_dn3 + (2.0 * var_sir_dn3))));
        var_tmp2_rv = 0.0;

        let assign2340_e1805: f64 = (2.0 * var_sif);
        let assign2340_e1806: f64 = (var_sir + assign2340_e1805);
        let assign2340_e1807: f64 = (var_tmp1 * assign2340_e1806);
        var_tmp3 = assign2340_e1807;
        var_tmp3_dn0 = ((var_tmp1_dn0 * assign2340_e1806) + (var_tmp1 * (var_sir_dn0 + (2.0 * var_sif_dn0))));
        var_tmp3_dn1 = ((var_tmp1_dn1 * assign2340_e1806) + (var_tmp1 * (var_sir_dn1 + (2.0 * var_sif_dn1))));
        var_tmp3_dn2 = ((var_tmp1_dn2 * assign2340_e1806) + (var_tmp1 * (var_sir_dn2 + (2.0 * var_sif_dn2))));
        var_tmp3_dn3 = ((var_tmp1_dn3 * assign2340_e1806) + (var_tmp1 * (var_sir_dn3 + (2.0 * var_sif_dn3))));
        var_tmp3_rv = 0.0;

        let assign2350_e1809: f64 = (-var_n_1);
        let assign2350_e1811: f64 = (assign2350_e1809 * var_qi);
        let assign2350_e1814: f64 = (2.0 + var_n_1);
        let assign2350_e1816: f64 = (assign2350_e1814 + var_n_1);
        let assign2350_e1818: f64 = (assign2350_e1816 * var_vp_phi_eps);
        let assign2350_e1819: f64 = (assign2350_e1811 / assign2350_e1818);
        var_tmp1 = assign2350_e1819;
        var_tmp1_dn0 = ((((((-var_n_1_dn0) * var_qi) + (assign2350_e1809 * var_qi_dn0)) * assign2350_e1818) - (assign2350_e1811 * (((var_n_1_dn0 + var_n_1_dn0) * var_vp_phi_eps) + (assign2350_e1816 * var_vp_phi_eps_dn0)))) / (assign2350_e1818 * assign2350_e1818));
        var_tmp1_dn1 = ((((((-var_n_1_dn1) * var_qi) + (assign2350_e1809 * var_qi_dn1)) * assign2350_e1818) - (assign2350_e1811 * (((var_n_1_dn1 + var_n_1_dn1) * var_vp_phi_eps) + (assign2350_e1816 * var_vp_phi_eps_dn1)))) / (assign2350_e1818 * assign2350_e1818));
        var_tmp1_dn2 = ((((((-var_n_1_dn2) * var_qi) + (assign2350_e1809 * var_qi_dn2)) * assign2350_e1818) - (assign2350_e1811 * (((var_n_1_dn2 + var_n_1_dn2) * var_vp_phi_eps) + (assign2350_e1816 * var_vp_phi_eps_dn2)))) / (assign2350_e1818 * assign2350_e1818));
        var_tmp1_dn3 = ((((((-var_n_1_dn3) * var_qi) + (assign2350_e1809 * var_qi_dn3)) * assign2350_e1818) - (assign2350_e1811 * (((var_n_1_dn3 + var_n_1_dn3) * var_vp_phi_eps) + (assign2350_e1816 * var_vp_phi_eps_dn3)))) / (assign2350_e1818 * assign2350_e1818));
        var_tmp1_rv = 0.0;

        let assign2360_e1822: f64 = (var_tmp1 * var_dvp_dvd);
        let assign2360_e1825: f64 = (var_tmp2 * var_dif_dvd);
        let assign2360_e1826: f64 = (assign2360_e1822 + assign2360_e1825);
        let assign2360_e1829: f64 = (var_tmp3 * var_dir_dvd);
        let assign2360_e1830: f64 = (assign2360_e1826 + assign2360_e1829);
        var_dqi_dvd = assign2360_e1830;
        var_dqi_dvd_dn0 = ((((var_tmp1_dn0 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn0)) + ((var_tmp2_dn0 * var_dif_dvd) + (var_tmp2 * var_dif_dvd_dn0))) + ((var_tmp3_dn0 * var_dir_dvd) + (var_tmp3 * var_dir_dvd_dn0)));
        var_dqi_dvd_dn1 = ((((var_tmp1_dn1 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn1)) + ((var_tmp2_dn1 * var_dif_dvd) + (var_tmp2 * var_dif_dvd_dn1))) + ((var_tmp3_dn1 * var_dir_dvd) + (var_tmp3 * var_dir_dvd_dn1)));
        var_dqi_dvd_dn2 = ((((var_tmp1_dn2 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn2)) + ((var_tmp2_dn2 * var_dif_dvd) + (var_tmp2 * var_dif_dvd_dn2))) + ((var_tmp3_dn2 * var_dir_dvd) + (var_tmp3 * var_dir_dvd_dn2)));
        var_dqi_dvd_dn3 = ((((var_tmp1_dn3 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn3)) + ((var_tmp2_dn3 * var_dif_dvd) + (var_tmp2 * var_dif_dvd_dn3))) + ((var_tmp3_dn3 * var_dir_dvd) + (var_tmp3 * var_dir_dvd_dn3)));
        var_dqi_dvd_rv = 0.0;

        let assign2370_e1833: f64 = (var_tmp1 * var_dvp_dvs);
        let assign2370_e1836: f64 = (var_tmp2 * var_dif_dvs);
        let assign2370_e1837: f64 = (assign2370_e1833 + assign2370_e1836);
        let assign2370_e1840: f64 = (var_tmp3 * var_dir_dvs);
        let assign2370_e1841: f64 = (assign2370_e1837 + assign2370_e1840);
        var_dqi_dvs = assign2370_e1841;
        var_dqi_dvs_dn0 = ((((var_tmp1_dn0 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn0)) + ((var_tmp2_dn0 * var_dif_dvs) + (var_tmp2 * var_dif_dvs_dn0))) + ((var_tmp3_dn0 * var_dir_dvs) + (var_tmp3 * var_dir_dvs_dn0)));
        var_dqi_dvs_dn1 = ((((var_tmp1_dn1 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn1)) + ((var_tmp2_dn1 * var_dif_dvs) + (var_tmp2 * var_dif_dvs_dn1))) + ((var_tmp3_dn1 * var_dir_dvs) + (var_tmp3 * var_dir_dvs_dn1)));
        var_dqi_dvs_dn2 = ((((var_tmp1_dn2 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn2)) + ((var_tmp2_dn2 * var_dif_dvs) + (var_tmp2 * var_dif_dvs_dn2))) + ((var_tmp3_dn2 * var_dir_dvs) + (var_tmp3 * var_dir_dvs_dn2)));
        var_dqi_dvs_dn3 = ((((var_tmp1_dn3 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn3)) + ((var_tmp2_dn3 * var_dif_dvs) + (var_tmp2 * var_dif_dvs_dn3))) + ((var_tmp3_dn3 * var_dir_dvs) + (var_tmp3 * var_dir_dvs_dn3)));
        var_dqi_dvs_rv = 0.0;

        let assign2390_e1855: f64 = (1.0 + var_n_1);
        let assign2390_e1860: f64 = (1.0 + var_n_1);
        let assign2390_e1861: f64 = (2.0 * assign2390_e1860);
        let assign2390_e1863: f64 = (assign2390_e1861 * var_vp_phi_eps);
        let assign2390_e1864: f64 = (var_qi / assign2390_e1863);
        let assign2390_e1865: f64 = (assign2390_e1855 - assign2390_e1864);
        var_tmp1 = assign2390_e1865;
        var_tmp1_dn0 = (var_n_1_dn0 - (((var_qi_dn0 * assign2390_e1863) - (var_qi * (((2.0 * var_n_1_dn0) * var_vp_phi_eps) + (assign2390_e1861 * var_vp_phi_eps_dn0)))) / (assign2390_e1863 * assign2390_e1863)));
        var_tmp1_dn1 = (var_n_1_dn1 - (((var_qi_dn1 * assign2390_e1863) - (var_qi * (((2.0 * var_n_1_dn1) * var_vp_phi_eps) + (assign2390_e1861 * var_vp_phi_eps_dn1)))) / (assign2390_e1863 * assign2390_e1863)));
        var_tmp1_dn2 = (var_n_1_dn2 - (((var_qi_dn2 * assign2390_e1863) - (var_qi * (((2.0 * var_n_1_dn2) * var_vp_phi_eps) + (assign2390_e1861 * var_vp_phi_eps_dn2)))) / (assign2390_e1863 * assign2390_e1863)));
        var_tmp1_dn3 = (var_n_1_dn3 - (((var_qi_dn3 * assign2390_e1863) - (var_qi * (((2.0 * var_n_1_dn3) * var_vp_phi_eps) + (assign2390_e1861 * var_vp_phi_eps_dn3)))) / (assign2390_e1863 * assign2390_e1863)));
        var_tmp1_rv = 0.0;

        let assign2400_e1867: f64 = (-var_n_1_n);
        let assign2400_e1870: f64 = (var_tmp1 * var_dvp_dvd);
        let assign2400_e1872: f64 = (assign2400_e1870 + var_dqi_dvd);
        let assign2400_e1873: f64 = (assign2400_e1867 * assign2400_e1872);
        var_dqb_dvd = assign2400_e1873;
        var_dqb_dvd_dn0 = (((-var_n_1_n_dn0) * assign2400_e1872) + (assign2400_e1867 * (((var_tmp1_dn0 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn0)) + var_dqi_dvd_dn0)));
        var_dqb_dvd_dn1 = (((-var_n_1_n_dn1) * assign2400_e1872) + (assign2400_e1867 * (((var_tmp1_dn1 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn1)) + var_dqi_dvd_dn1)));
        var_dqb_dvd_dn2 = (((-var_n_1_n_dn2) * assign2400_e1872) + (assign2400_e1867 * (((var_tmp1_dn2 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn2)) + var_dqi_dvd_dn2)));
        var_dqb_dvd_dn3 = (((-var_n_1_n_dn3) * assign2400_e1872) + (assign2400_e1867 * (((var_tmp1_dn3 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn3)) + var_dqi_dvd_dn3)));
        var_dqb_dvd_rv = 0.0;

        let assign2410_e1875: f64 = (-var_n_1_n);
        let assign2410_e1878: f64 = (var_tmp1 * var_dvp_dvs);
        let assign2410_e1880: f64 = (assign2410_e1878 + var_dqi_dvs);
        let assign2410_e1881: f64 = (assign2410_e1875 * assign2410_e1880);
        var_dqb_dvs = assign2410_e1881;
        var_dqb_dvs_dn0 = (((-var_n_1_n_dn0) * assign2410_e1880) + (assign2410_e1875 * (((var_tmp1_dn0 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn0)) + var_dqi_dvs_dn0)));
        var_dqb_dvs_dn1 = (((-var_n_1_n_dn1) * assign2410_e1880) + (assign2410_e1875 * (((var_tmp1_dn1 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn1)) + var_dqi_dvs_dn1)));
        var_dqb_dvs_dn2 = (((-var_n_1_n_dn2) * assign2410_e1880) + (assign2410_e1875 * (((var_tmp1_dn2 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn2)) + var_dqi_dvs_dn2)));
        var_dqb_dvs_dn3 = (((-var_n_1_n_dn3) * assign2410_e1880) + (assign2410_e1875 * (((var_tmp1_dn3 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn3)) + var_dqi_dvs_dn3)));
        var_dqb_dvs_rv = 0.0;

        let assign2430_e1892: f64 = if p.p22 == 0.0 { 1.0 } else { 0.0 };
        var_guard18 = assign2430_e1892;
        var_guard18_rv = 0.0;

        let (assign2440_e1902, assign2440_e1902_d_n0, assign2440_e1902_d_n1, assign2440_e1902_d_n2, assign2440_e1902_d_n3,) = {
    if (var_guard18 != 0.0) {
        let assign2440_e1896: f64 = (p.p21 * var_vpprime);
        let assign2440_e1899: f64 = (var_theta_vp_1 * var_sqrt_vp_vt);
        let assign2440_e1900: f64 = (assign2440_e1896 / assign2440_e1899);
        (assign2440_e1900, ((((p.p21 * var_vpprime_dn0) * assign2440_e1899) - (assign2440_e1896 * ((var_theta_vp_1_dn0 * var_sqrt_vp_vt) + (var_theta_vp_1 * var_sqrt_vp_vt_dn0)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * var_vpprime_dn1) * assign2440_e1899) - (assign2440_e1896 * ((var_theta_vp_1_dn1 * var_sqrt_vp_vt) + (var_theta_vp_1 * var_sqrt_vp_vt_dn1)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * var_vpprime_dn2) * assign2440_e1899) - (assign2440_e1896 * ((var_theta_vp_1_dn2 * var_sqrt_vp_vt) + (var_theta_vp_1 * var_sqrt_vp_vt_dn2)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * var_vpprime_dn3) * assign2440_e1899) - (assign2440_e1896 * ((var_theta_vp_1_dn3 * var_sqrt_vp_vt) + (var_theta_vp_1 * var_sqrt_vp_vt_dn3)))) / (assign2440_e1899 * assign2440_e1899)),)
    } else {
        (var_tmp1, var_tmp1_dn0, var_tmp1_dn1, var_tmp1_dn2, var_tmp1_dn3,)
    }
};
        var_tmp1 = assign2440_e1902;
        var_tmp1_dn0 = assign2440_e1902_d_n0;
        var_tmp1_dn1 = assign2440_e1902_d_n1;
        var_tmp1_dn2 = assign2440_e1902_d_n2;
        var_tmp1_dn3 = assign2440_e1902_d_n3;
        var_tmp1_rv = 0.0;

        let (assign2450_e1908, assign2450_e1908_d_n0, assign2450_e1908_d_n1, assign2450_e1908_d_n2, assign2450_e1908_d_n3,) = {
    if (var_guard18 != 0.0) {
        let assign2450_e1906: f64 = (var_tmp1 * var_dvp_dvd);
        (assign2450_e1906, ((var_tmp1_dn0 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn0)), ((var_tmp1_dn1 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn1)), ((var_tmp1_dn2 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn2)), ((var_tmp1_dn3 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn3)),)
    } else {
        (var_dvpprime_dvd, var_dvpprime_dvd_dn0, var_dvpprime_dvd_dn1, var_dvpprime_dvd_dn2, var_dvpprime_dvd_dn3,)
    }
};
        var_dvpprime_dvd = assign2450_e1908;
        var_dvpprime_dvd_dn0 = assign2450_e1908_d_n0;
        var_dvpprime_dvd_dn1 = assign2450_e1908_d_n1;
        var_dvpprime_dvd_dn2 = assign2450_e1908_d_n2;
        var_dvpprime_dvd_dn3 = assign2450_e1908_d_n3;
        var_dvpprime_dvd_rv = 0.0;

        let (assign2460_e1914, assign2460_e1914_d_n0, assign2460_e1914_d_n1, assign2460_e1914_d_n2, assign2460_e1914_d_n3,) = {
    if (var_guard18 != 0.0) {
        let assign2460_e1912: f64 = (var_tmp1 * var_dvp_dvs);
        (assign2460_e1912, ((var_tmp1_dn0 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn0)), ((var_tmp1_dn1 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn1)), ((var_tmp1_dn2 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn2)), ((var_tmp1_dn3 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn3)),)
    } else {
        (var_dvpprime_dvs, var_dvpprime_dvs_dn0, var_dvpprime_dvs_dn1, var_dvpprime_dvs_dn2, var_dvpprime_dvs_dn3,)
    }
};
        var_dvpprime_dvs = assign2460_e1914;
        var_dvpprime_dvs_dn0 = assign2460_e1914_d_n0;
        var_dvpprime_dvs_dn1 = assign2460_e1914_d_n1;
        var_dvpprime_dvs_dn2 = assign2460_e1914_d_n2;
        var_dvpprime_dvs_dn3 = assign2460_e1914_d_n3;
        var_dvpprime_dvs_rv = 0.0;

        let (assign2480_e1927, assign2480_e1927_d_n0, assign2480_e1927_d_n1, assign2480_e1927_d_n2, assign2480_e1927_d_n3,) = {
    if (var_guard18 != 0.0) {
        let assign2480_e1923: f64 = (-var_dleq_dvd);
        let assign2480_e1925: f64 = (assign2480_e1923 - var_dvpprime_dvd);
        (assign2480_e1925, ((-var_dleq_dvd_dn0) - var_dvpprime_dvd_dn0), ((-var_dleq_dvd_dn1) - var_dvpprime_dvd_dn1), ((-var_dleq_dvd_dn2) - var_dvpprime_dvd_dn2), ((-var_dleq_dvd_dn3) - var_dvpprime_dvd_dn3),)
    } else {
        (var_dbeta_dvd, var_dbeta_dvd_dn0, var_dbeta_dvd_dn1, var_dbeta_dvd_dn2, var_dbeta_dvd_dn3,)
    }
};
        var_dbeta_dvd = assign2480_e1927;
        var_dbeta_dvd_dn0 = assign2480_e1927_d_n0;
        var_dbeta_dvd_dn1 = assign2480_e1927_d_n1;
        var_dbeta_dvd_dn2 = assign2480_e1927_d_n2;
        var_dbeta_dvd_dn3 = assign2480_e1927_d_n3;
        var_dbeta_dvd_rv = 0.0;

        let (assign2490_e1934, assign2490_e1934_d_n0, assign2490_e1934_d_n1, assign2490_e1934_d_n2, assign2490_e1934_d_n3,) = {
    if (var_guard18 != 0.0) {
        let assign2490_e1930: f64 = (-var_dleq_dvs);
        let assign2490_e1932: f64 = (assign2490_e1930 - var_dvpprime_dvs);
        (assign2490_e1932, ((-var_dleq_dvs_dn0) - var_dvpprime_dvs_dn0), ((-var_dleq_dvs_dn1) - var_dvpprime_dvs_dn1), ((-var_dleq_dvs_dn2) - var_dvpprime_dvs_dn2), ((-var_dleq_dvs_dn3) - var_dvpprime_dvs_dn3),)
    } else {
        (var_dbeta_dvs, var_dbeta_dvs_dn0, var_dbeta_dvs_dn1, var_dbeta_dvs_dn2, var_dbeta_dvs_dn3,)
    }
};
        var_dbeta_dvs = assign2490_e1934;
        var_dbeta_dvs_dn0 = assign2490_e1934_d_n0;
        var_dbeta_dvs_dn1 = assign2490_e1934_d_n1;
        var_dbeta_dvs_dn2 = assign2490_e1934_d_n2;
        var_dbeta_dvs_dn3 = assign2490_e1934_d_n3;
        var_dbeta_dvs_rv = 0.0;

        let (assign2510_e1948, assign2510_e1948_d_n0, assign2510_e1948_d_n1, assign2510_e1948_d_n2, assign2510_e1948_d_n3,) = {
    if (var_guard18 == 0.0) {
        let assign2510_e1946: f64 = (var_t0 / var_e0_q_1);
        (assign2510_e1946, (-((var_t0 * var_e0_q_1_dn0) / (var_e0_q_1 * var_e0_q_1))), (-((var_t0 * var_e0_q_1_dn1) / (var_e0_q_1 * var_e0_q_1))), (-((var_t0 * var_e0_q_1_dn2) / (var_e0_q_1 * var_e0_q_1))), (-((var_t0 * var_e0_q_1_dn3) / (var_e0_q_1 * var_e0_q_1))),)
    } else {
        (var_tmp1, var_tmp1_dn0, var_tmp1_dn1, var_tmp1_dn2, var_tmp1_dn3,)
    }
};
        var_tmp1 = assign2510_e1948;
        var_tmp1_dn0 = assign2510_e1948_d_n0;
        var_tmp1_dn1 = assign2510_e1948_d_n1;
        var_tmp1_dn2 = assign2510_e1948_d_n2;
        var_tmp1_dn3 = assign2510_e1948_d_n3;
        var_tmp1_rv = 0.0;

        let (assign2520_e1962, assign2520_e1962_d_n0, assign2520_e1962_d_n1, assign2520_e1962_d_n2, assign2520_e1962_d_n3,) = {
    if (var_guard18 == 0.0) {
        let assign2520_e1952: f64 = (-var_dleq_dvd);
        let assign2520_e1957: f64 = (var_eta_qi * var_dqi_dvd);
        let assign2520_e1958: f64 = (var_dqb_dvd + assign2520_e1957);
        let assign2520_e1959: f64 = (var_tmp1 * assign2520_e1958);
        let assign2520_e1960: f64 = (assign2520_e1952 + assign2520_e1959);
        (assign2520_e1960, ((-var_dleq_dvd_dn0) + ((var_tmp1_dn0 * assign2520_e1958) + (var_tmp1 * (var_dqb_dvd_dn0 + (var_eta_qi * var_dqi_dvd_dn0))))), ((-var_dleq_dvd_dn1) + ((var_tmp1_dn1 * assign2520_e1958) + (var_tmp1 * (var_dqb_dvd_dn1 + (var_eta_qi * var_dqi_dvd_dn1))))), ((-var_dleq_dvd_dn2) + ((var_tmp1_dn2 * assign2520_e1958) + (var_tmp1 * (var_dqb_dvd_dn2 + (var_eta_qi * var_dqi_dvd_dn2))))), ((-var_dleq_dvd_dn3) + ((var_tmp1_dn3 * assign2520_e1958) + (var_tmp1 * (var_dqb_dvd_dn3 + (var_eta_qi * var_dqi_dvd_dn3))))),)
    } else {
        (var_dbeta_dvd, var_dbeta_dvd_dn0, var_dbeta_dvd_dn1, var_dbeta_dvd_dn2, var_dbeta_dvd_dn3,)
    }
};
        var_dbeta_dvd = assign2520_e1962;
        var_dbeta_dvd_dn0 = assign2520_e1962_d_n0;
        var_dbeta_dvd_dn1 = assign2520_e1962_d_n1;
        var_dbeta_dvd_dn2 = assign2520_e1962_d_n2;
        var_dbeta_dvd_dn3 = assign2520_e1962_d_n3;
        var_dbeta_dvd_rv = 0.0;

        let (assign2530_e1976, assign2530_e1976_d_n0, assign2530_e1976_d_n1, assign2530_e1976_d_n2, assign2530_e1976_d_n3,) = {
    if (var_guard18 == 0.0) {
        let assign2530_e1966: f64 = (-var_dleq_dvs);
        let assign2530_e1971: f64 = (var_eta_qi * var_dqi_dvs);
        let assign2530_e1972: f64 = (var_dqb_dvs + assign2530_e1971);
        let assign2530_e1973: f64 = (var_tmp1 * assign2530_e1972);
        let assign2530_e1974: f64 = (assign2530_e1966 + assign2530_e1973);
        (assign2530_e1974, ((-var_dleq_dvs_dn0) + ((var_tmp1_dn0 * assign2530_e1972) + (var_tmp1 * (var_dqb_dvs_dn0 + (var_eta_qi * var_dqi_dvs_dn0))))), ((-var_dleq_dvs_dn1) + ((var_tmp1_dn1 * assign2530_e1972) + (var_tmp1 * (var_dqb_dvs_dn1 + (var_eta_qi * var_dqi_dvs_dn1))))), ((-var_dleq_dvs_dn2) + ((var_tmp1_dn2 * assign2530_e1972) + (var_tmp1 * (var_dqb_dvs_dn2 + (var_eta_qi * var_dqi_dvs_dn2))))), ((-var_dleq_dvs_dn3) + ((var_tmp1_dn3 * assign2530_e1972) + (var_tmp1 * (var_dqb_dvs_dn3 + (var_eta_qi * var_dqi_dvs_dn3))))),)
    } else {
        (var_dbeta_dvs, var_dbeta_dvs_dn0, var_dbeta_dvs_dn1, var_dbeta_dvs_dn2, var_dbeta_dvs_dn3,)
    }
};
        var_dbeta_dvs = assign2530_e1976;
        var_dbeta_dvs_dn0 = assign2530_e1976_d_n0;
        var_dbeta_dvs_dn1 = assign2530_e1976_d_n1;
        var_dbeta_dvs_dn2 = assign2530_e1976_d_n2;
        var_dbeta_dvs_dn3 = assign2530_e1976_d_n3;
        var_dbeta_dvs_rv = 0.0;

        let assign2550_e1992: f64 = (-var_gamma_s);
        let assign2550_e1995: f64 = (4.0 * var_n);
        let assign2550_e1997: f64 = (assign2550_e1995 * var_sqrt_phi_vp);
        let assign2550_e2000: f64 = (var_phi_t + var_vp);
        let assign2550_e2002: f64 = (assign2550_e2000 + var_vt_4);
        let assign2550_e2003: f64 = (assign2550_e1997 * assign2550_e2002);
        let assign2550_e2004: f64 = (assign2550_e1992 / assign2550_e2003);
        var_tmp1 = assign2550_e2004;
        var_tmp1_dn0 = (-((assign2550_e1992 * (((((4.0 * var_n_dn0) * var_sqrt_phi_vp) + (assign2550_e1995 * var_sqrt_phi_vp_dn0)) * assign2550_e2002) + (assign2550_e1997 * (var_phi_t_dn0 + var_vp_dn0)))) / (assign2550_e2003 * assign2550_e2003)));
        var_tmp1_dn1 = (-((assign2550_e1992 * (((((4.0 * var_n_dn1) * var_sqrt_phi_vp) + (assign2550_e1995 * var_sqrt_phi_vp_dn1)) * assign2550_e2002) + (assign2550_e1997 * (var_phi_t_dn1 + var_vp_dn1)))) / (assign2550_e2003 * assign2550_e2003)));
        var_tmp1_dn2 = (-((assign2550_e1992 * (((((4.0 * var_n_dn2) * var_sqrt_phi_vp) + (assign2550_e1995 * var_sqrt_phi_vp_dn2)) * assign2550_e2002) + (assign2550_e1997 * (var_phi_t_dn2 + var_vp_dn2)))) / (assign2550_e2003 * assign2550_e2003)));
        var_tmp1_dn3 = (-((assign2550_e1992 * (((((4.0 * var_n_dn3) * var_sqrt_phi_vp) + (assign2550_e1995 * var_sqrt_phi_vp_dn3)) * assign2550_e2002) + (assign2550_e1997 * (var_phi_t_dn3 + var_vp_dn3)))) / (assign2550_e2003 * assign2550_e2003)));
        var_tmp1_rv = 0.0;

        let assign2560_e2007: f64 = (var_tmp1 * var_dvp_dvd);
        var_dn_dvd = assign2560_e2007;
        var_dn_dvd_dn0 = ((var_tmp1_dn0 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn0));
        var_dn_dvd_dn1 = ((var_tmp1_dn1 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn1));
        var_dn_dvd_dn2 = ((var_tmp1_dn2 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn2));
        var_dn_dvd_dn3 = ((var_tmp1_dn3 * var_dvp_dvd) + (var_tmp1 * var_dvp_dvd_dn3));
        var_dn_dvd_rv = 0.0;

        let assign2570_e2010: f64 = (var_tmp1 * var_dvp_dvs);
        var_dn_dvs = assign2570_e2010;
        var_dn_dvs_dn0 = ((var_tmp1_dn0 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn0));
        var_dn_dvs_dn1 = ((var_tmp1_dn1 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn1));
        var_dn_dvs_dn2 = ((var_tmp1_dn2 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn2));
        var_dn_dvs_dn3 = ((var_tmp1_dn3 * var_dvp_dvs) + (var_tmp1 * var_dvp_dvs_dn3));
        var_dn_dvs_rv = 0.0;

        let assign2590_e2017: f64 = (var_dn_dvd + var_dbeta_dvd);
        let assign2590_e2019: f64 = (assign2590_e2017 * var_if_ir);
        let assign2590_e2021: f64 = (assign2590_e2019 + var_dif_dvd);
        let assign2590_e2023: f64 = (assign2590_e2021 - var_dirprime_dvd);
        let assign2590_e2024: f64 = (var_ispec * assign2590_e2023);
        var_gds = assign2590_e2024;
        var_gds_dn0 = ((var_ispec_dn0 * assign2590_e2023) + (var_ispec * (((((var_dn_dvd_dn0 + var_dbeta_dvd_dn0) * var_if_ir) + (assign2590_e2017 * var_if_ir_dn0)) + var_dif_dvd_dn0) - var_dirprime_dvd_dn0)));
        var_gds_dn1 = ((var_ispec_dn1 * assign2590_e2023) + (var_ispec * (((((var_dn_dvd_dn1 + var_dbeta_dvd_dn1) * var_if_ir) + (assign2590_e2017 * var_if_ir_dn1)) + var_dif_dvd_dn1) - var_dirprime_dvd_dn1)));
        var_gds_dn2 = ((var_ispec_dn2 * assign2590_e2023) + (var_ispec * (((((var_dn_dvd_dn2 + var_dbeta_dvd_dn2) * var_if_ir) + (assign2590_e2017 * var_if_ir_dn2)) + var_dif_dvd_dn2) - var_dirprime_dvd_dn2)));
        var_gds_dn3 = ((var_ispec_dn3 * assign2590_e2023) + (var_ispec * (((((var_dn_dvd_dn3 + var_dbeta_dvd_dn3) * var_if_ir) + (assign2590_e2017 * var_if_ir_dn3)) + var_dif_dvd_dn3) - var_dirprime_dvd_dn3)));
        var_gds_rv = 0.0;

        let assign2600_e2026: f64 = (-var_ispec);
        let assign2600_e2029: f64 = (var_dn_dvs + var_dbeta_dvs);
        let assign2600_e2031: f64 = (assign2600_e2029 * var_if_ir);
        let assign2600_e2033: f64 = (assign2600_e2031 + var_dif_dvs);
        let assign2600_e2035: f64 = (assign2600_e2033 - var_dirprime_dvs);
        let assign2600_e2036: f64 = (assign2600_e2026 * assign2600_e2035);
        var_gms = assign2600_e2036;
        var_gms_dn0 = (((-var_ispec_dn0) * assign2600_e2035) + (assign2600_e2026 * (((((var_dn_dvs_dn0 + var_dbeta_dvs_dn0) * var_if_ir) + (assign2600_e2029 * var_if_ir_dn0)) + var_dif_dvs_dn0) - var_dirprime_dvs_dn0)));
        var_gms_dn1 = (((-var_ispec_dn1) * assign2600_e2035) + (assign2600_e2026 * (((((var_dn_dvs_dn1 + var_dbeta_dvs_dn1) * var_if_ir) + (assign2600_e2029 * var_if_ir_dn1)) + var_dif_dvs_dn1) - var_dirprime_dvs_dn1)));
        var_gms_dn2 = (((-var_ispec_dn2) * assign2600_e2035) + (assign2600_e2026 * (((((var_dn_dvs_dn2 + var_dbeta_dvs_dn2) * var_if_ir) + (assign2600_e2029 * var_if_ir_dn2)) + var_dif_dvs_dn2) - var_dirprime_dvs_dn2)));
        var_gms_dn3 = (((-var_ispec_dn3) * assign2600_e2035) + (assign2600_e2026 * (((((var_dn_dvs_dn3 + var_dbeta_dvs_dn3) * var_if_ir) + (assign2600_e2029 * var_if_ir_dn3)) + var_dif_dvs_dn3) - var_dirprime_dvs_dn3)));
        var_gms_rv = 0.0;

        let assign2630_e2055: f64 = (p.p36 * p.p37);
        let assign2630_e2058: f64 = (var_weff - p.p27);
        let assign2630_e2059: f64 = (assign2630_e2055 / assign2630_e2058);
        var_rseff = assign2630_e2059;
        var_rseff_rv = 0.0;

        let assign2640_e2062: f64 = (p.p36 * p.p37);
        let assign2640_e2065: f64 = (var_weff - p.p27);
        let assign2640_e2066: f64 = (assign2640_e2062 / assign2640_e2065);
        var_rdeff = assign2640_e2066;
        var_rdeff_rv = 0.0;

        let assign2650_e2071: f64 = (var_gms * var_rseff);
        let assign2650_e2072: f64 = (1.0 + assign2650_e2071);
        let assign2650_e2075: f64 = (var_gds * var_rdeff);
        let assign2650_e2076: f64 = (assign2650_e2072 + assign2650_e2075);
        let assign2650_e2077: f64 = (1.0 / assign2650_e2076);
        var_tmp1 = assign2650_e2077;
        var_tmp1_dn0 = (-(((var_gms_dn0 * var_rseff) + (var_gds_dn0 * var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        var_tmp1_dn1 = (-(((var_gms_dn1 * var_rseff) + (var_gds_dn1 * var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        var_tmp1_dn2 = (-(((var_gms_dn2 * var_rseff) + (var_gds_dn2 * var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        var_tmp1_dn3 = (-(((var_gms_dn3 * var_rseff) + (var_gds_dn3 * var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        var_tmp1_rv = 0.0;

        let assign2800_e2163: f64 = (var_weff * var_leff);
        let assign2800_e2165: f64 = (assign2800_e2163 * p.p13);
        var_wlcox = assign2800_e2165;
        var_wlcox_rv = 0.0;

        let assign2810_e2168: f64 = (var_sif * var_sif2);
        var_sif3 = assign2810_e2168;
        var_sif3_dn0 = ((var_sif_dn0 * var_sif2) + (var_sif * var_sif2_dn0));
        var_sif3_dn1 = ((var_sif_dn1 * var_sif2) + (var_sif * var_sif2_dn1));
        var_sif3_dn2 = ((var_sif_dn2 * var_sif2) + (var_sif * var_sif2_dn2));
        var_sif3_dn3 = ((var_sif_dn3 * var_sif2) + (var_sif * var_sif2_dn3));
        var_sif3_rv = 0.0;

        let assign2820_e2171: f64 = (var_sir * var_sir2);
        var_sir3 = assign2820_e2171;
        var_sir3_dn0 = ((var_sir_dn0 * var_sir2) + (var_sir * var_sir2_dn0));
        var_sir3_dn1 = ((var_sir_dn1 * var_sir2) + (var_sir * var_sir2_dn1));
        var_sir3_dn2 = ((var_sir_dn2 * var_sir2) + (var_sir * var_sir2_dn2));
        var_sir3_dn3 = ((var_sir_dn3 * var_sir2) + (var_sir * var_sir2_dn3));
        var_sir3_rv = 0.0;

        let assign2830_e2175: f64 = (0.5 * var_vp);
        let assign2830_e2176: f64 = (var_phi_t + assign2830_e2175);
        let assign2830_e2177: f64 = (assign2830_e2176).sqrt();
        var_tmp1 = assign2830_e2177;
        var_tmp1_dn0 = ((var_phi_t_dn0 + (0.5 * var_vp_dn0)) / (2.0 * assign2830_e2177));
        var_tmp1_dn1 = ((var_phi_t_dn1 + (0.5 * var_vp_dn1)) / (2.0 * assign2830_e2177));
        var_tmp1_dn2 = ((var_phi_t_dn2 + (0.5 * var_vp_dn2)) / (2.0 * assign2830_e2177));
        var_tmp1_dn3 = ((var_phi_t_dn3 + (0.5 * var_vp_dn3)) / (2.0 * assign2830_e2177));
        var_tmp1_rv = 0.0;

        let assign2840_e2180: f64 = (var_tmp1 + var_tmp1);
        var_sqrt_phi_vp2_2 = assign2840_e2180;
        var_sqrt_phi_vp2_2_dn0 = (var_tmp1_dn0 + var_tmp1_dn0);
        var_sqrt_phi_vp2_2_dn1 = (var_tmp1_dn1 + var_tmp1_dn1);
        var_sqrt_phi_vp2_2_dn2 = (var_tmp1_dn2 + var_tmp1_dn2);
        var_sqrt_phi_vp2_2_dn3 = (var_tmp1_dn3 + var_tmp1_dn3);
        var_sqrt_phi_vp2_2_rv = 0.0;

        let assign2850_e2184: f64 = (var_gammaprime / var_sqrt_phi_vp2_2);
        let assign2850_e2185: f64 = (1.0 + assign2850_e2184);
        let assign2850_e2187: f64 = (assign2850_e2185 * var_vt);
        let assign2850_e2189: f64 = (assign2850_e2187 * var_wlcox);
        var_n_vt_cox = assign2850_e2189;
        var_n_vt_cox_dn0 = (((((var_gammaprime_dn0 * var_sqrt_phi_vp2_2) - (var_gammaprime * var_sqrt_phi_vp2_2_dn0)) / (var_sqrt_phi_vp2_2 * var_sqrt_phi_vp2_2)) * var_vt) * var_wlcox);
        var_n_vt_cox_dn1 = (((((var_gammaprime_dn1 * var_sqrt_phi_vp2_2) - (var_gammaprime * var_sqrt_phi_vp2_2_dn1)) / (var_sqrt_phi_vp2_2 * var_sqrt_phi_vp2_2)) * var_vt) * var_wlcox);
        var_n_vt_cox_dn2 = (((((var_gammaprime_dn2 * var_sqrt_phi_vp2_2) - (var_gammaprime * var_sqrt_phi_vp2_2_dn2)) / (var_sqrt_phi_vp2_2 * var_sqrt_phi_vp2_2)) * var_vt) * var_wlcox);
        var_n_vt_cox_dn3 = (((((var_gammaprime_dn3 * var_sqrt_phi_vp2_2) - (var_gammaprime * var_sqrt_phi_vp2_2_dn3)) / (var_sqrt_phi_vp2_2 * var_sqrt_phi_vp2_2)) * var_vt) * var_wlcox);
        var_n_vt_cox_rv = 0.0;

        let assign2860_e2191: f64 = (-var_n_vt_cox);
        let assign2860_e2195: f64 = (3.0 * var_sir3);
        let assign2860_e2198: f64 = (6.0 * var_sir2);
        let assign2860_e2200: f64 = (assign2860_e2198 * var_sif);
        let assign2860_e2201: f64 = (assign2860_e2195 + assign2860_e2200);
        let assign2860_e2204: f64 = (4.0 * var_sir);
        let assign2860_e2206: f64 = (assign2860_e2204 * var_sif2);
        let assign2860_e2207: f64 = (assign2860_e2201 + assign2860_e2206);
        let assign2860_e2210: f64 = (2.0 * var_sif3);
        let assign2860_e2211: f64 = (assign2860_e2207 + assign2860_e2210);
        let assign2860_e2212: f64 = (0.266666666 * assign2860_e2211);
        let assign2860_e2214: f64 = (assign2860_e2212 / var_sif_sir_2);
        let assign2860_e2216: f64 = (assign2860_e2214 - 0.5);
        let assign2860_e2217: f64 = (assign2860_e2191 * assign2860_e2216);
        var_qd = assign2860_e2217;
        var_qd_dn0 = (((-var_n_vt_cox_dn0) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * var_sir3_dn0) + (((6.0 * var_sir2_dn0) * var_sif) + (assign2860_e2198 * var_sif_dn0))) + (((4.0 * var_sir_dn0) * var_sif2) + (assign2860_e2204 * var_sif2_dn0))) + (2.0 * var_sif3_dn0))) * var_sif_sir_2) - (assign2860_e2212 * var_sif_sir_2_dn0)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qd_dn1 = (((-var_n_vt_cox_dn1) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * var_sir3_dn1) + (((6.0 * var_sir2_dn1) * var_sif) + (assign2860_e2198 * var_sif_dn1))) + (((4.0 * var_sir_dn1) * var_sif2) + (assign2860_e2204 * var_sif2_dn1))) + (2.0 * var_sif3_dn1))) * var_sif_sir_2) - (assign2860_e2212 * var_sif_sir_2_dn1)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qd_dn2 = (((-var_n_vt_cox_dn2) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * var_sir3_dn2) + (((6.0 * var_sir2_dn2) * var_sif) + (assign2860_e2198 * var_sif_dn2))) + (((4.0 * var_sir_dn2) * var_sif2) + (assign2860_e2204 * var_sif2_dn2))) + (2.0 * var_sif3_dn2))) * var_sif_sir_2) - (assign2860_e2212 * var_sif_sir_2_dn2)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qd_dn3 = (((-var_n_vt_cox_dn3) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * var_sir3_dn3) + (((6.0 * var_sir2_dn3) * var_sif) + (assign2860_e2198 * var_sif_dn3))) + (((4.0 * var_sir_dn3) * var_sif2) + (assign2860_e2204 * var_sif2_dn3))) + (2.0 * var_sif3_dn3))) * var_sif_sir_2) - (assign2860_e2212 * var_sif_sir_2_dn3)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qd_rv = 0.0;

        let assign2870_e2219: f64 = (-var_n_vt_cox);
        let assign2870_e2223: f64 = (3.0 * var_sif3);
        let assign2870_e2226: f64 = (6.0 * var_sif2);
        let assign2870_e2228: f64 = (assign2870_e2226 * var_sir);
        let assign2870_e2229: f64 = (assign2870_e2223 + assign2870_e2228);
        let assign2870_e2232: f64 = (4.0 * var_sif);
        let assign2870_e2234: f64 = (assign2870_e2232 * var_sir2);
        let assign2870_e2235: f64 = (assign2870_e2229 + assign2870_e2234);
        let assign2870_e2238: f64 = (2.0 * var_sir3);
        let assign2870_e2239: f64 = (assign2870_e2235 + assign2870_e2238);
        let assign2870_e2240: f64 = (0.266666666 * assign2870_e2239);
        let assign2870_e2242: f64 = (assign2870_e2240 / var_sif_sir_2);
        let assign2870_e2244: f64 = (assign2870_e2242 - 0.5);
        let assign2870_e2245: f64 = (assign2870_e2219 * assign2870_e2244);
        var_qs = assign2870_e2245;
        var_qs_dn0 = (((-var_n_vt_cox_dn0) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * var_sif3_dn0) + (((6.0 * var_sif2_dn0) * var_sir) + (assign2870_e2226 * var_sir_dn0))) + (((4.0 * var_sif_dn0) * var_sir2) + (assign2870_e2232 * var_sir2_dn0))) + (2.0 * var_sir3_dn0))) * var_sif_sir_2) - (assign2870_e2240 * var_sif_sir_2_dn0)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qs_dn1 = (((-var_n_vt_cox_dn1) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * var_sif3_dn1) + (((6.0 * var_sif2_dn1) * var_sir) + (assign2870_e2226 * var_sir_dn1))) + (((4.0 * var_sif_dn1) * var_sir2) + (assign2870_e2232 * var_sir2_dn1))) + (2.0 * var_sir3_dn1))) * var_sif_sir_2) - (assign2870_e2240 * var_sif_sir_2_dn1)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qs_dn2 = (((-var_n_vt_cox_dn2) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * var_sif3_dn2) + (((6.0 * var_sif2_dn2) * var_sir) + (assign2870_e2226 * var_sir_dn2))) + (((4.0 * var_sif_dn2) * var_sir2) + (assign2870_e2232 * var_sir2_dn2))) + (2.0 * var_sir3_dn2))) * var_sif_sir_2) - (assign2870_e2240 * var_sif_sir_2_dn2)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qs_dn3 = (((-var_n_vt_cox_dn3) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * var_sif3_dn3) + (((6.0 * var_sif2_dn3) * var_sir) + (assign2870_e2226 * var_sir_dn3))) + (((4.0 * var_sif_dn3) * var_sir2) + (assign2870_e2232 * var_sir2_dn3))) + (2.0 * var_sir3_dn3))) * var_sif_sir_2) - (assign2870_e2240 * var_sif_sir_2_dn3)) / (var_sif_sir_2 * var_sif_sir_2))));
        var_qs_rv = 0.0;

        let assign2880_e2248: f64 = (var_qs + var_qd);
        var_qi_1 = assign2880_e2248;
        var_qi_1_dn0 = (var_qs_dn0 + var_qd_dn0);
        var_qi_1_dn1 = (var_qs_dn1 + var_qd_dn1);
        var_qi_1_dn2 = (var_qs_dn2 + var_qd_dn2);
        var_qi_1_dn3 = (var_qs_dn3 + var_qd_dn3);
        var_qi_1_rv = 0.0;

        *var_dbeta_dvd_slot = var_dbeta_dvd;
        *var_dbeta_dvd_dn0_slot = var_dbeta_dvd_dn0;
        *var_dbeta_dvd_dn1_slot = var_dbeta_dvd_dn1;
        *var_dbeta_dvd_dn2_slot = var_dbeta_dvd_dn2;
        *var_dbeta_dvd_dn3_slot = var_dbeta_dvd_dn3;
        *var_dbeta_dvd_rv_slot = var_dbeta_dvd_rv;
        *var_dbeta_dvs_slot = var_dbeta_dvs;
        *var_dbeta_dvs_dn0_slot = var_dbeta_dvs_dn0;
        *var_dbeta_dvs_dn1_slot = var_dbeta_dvs_dn1;
        *var_dbeta_dvs_dn2_slot = var_dbeta_dvs_dn2;
        *var_dbeta_dvs_dn3_slot = var_dbeta_dvs_dn3;
        *var_dbeta_dvs_rv_slot = var_dbeta_dvs_rv;
        *var_ddeltal_dvd_slot = var_ddeltal_dvd;
        *var_ddeltal_dvd_dn0_slot = var_ddeltal_dvd_dn0;
        *var_ddeltal_dvd_dn1_slot = var_ddeltal_dvd_dn1;
        *var_ddeltal_dvd_dn2_slot = var_ddeltal_dvd_dn2;
        *var_ddeltal_dvd_dn3_slot = var_ddeltal_dvd_dn3;
        *var_ddeltal_dvd_rv_slot = var_ddeltal_dvd_rv;
        *var_ddeltal_dvs_slot = var_ddeltal_dvs;
        *var_ddeltal_dvs_dn0_slot = var_ddeltal_dvs_dn0;
        *var_ddeltal_dvs_dn1_slot = var_ddeltal_dvs_dn1;
        *var_ddeltal_dvs_dn2_slot = var_ddeltal_dvs_dn2;
        *var_ddeltal_dvs_dn3_slot = var_ddeltal_dvs_dn3;
        *var_ddeltal_dvs_rv_slot = var_ddeltal_dvs_rv;
        *var_dir_dvd_slot = var_dir_dvd;
        *var_dir_dvd_dn0_slot = var_dir_dvd_dn0;
        *var_dir_dvd_dn1_slot = var_dir_dvd_dn1;
        *var_dir_dvd_dn2_slot = var_dir_dvd_dn2;
        *var_dir_dvd_dn3_slot = var_dir_dvd_dn3;
        *var_dir_dvd_rv_slot = var_dir_dvd_rv;
        *var_dir_dvs_slot = var_dir_dvs;
        *var_dir_dvs_dn0_slot = var_dir_dvs_dn0;
        *var_dir_dvs_dn1_slot = var_dir_dvs_dn1;
        *var_dir_dvs_dn2_slot = var_dir_dvs_dn2;
        *var_dir_dvs_dn3_slot = var_dir_dvs_dn3;
        *var_dir_dvs_rv_slot = var_dir_dvs_rv;
        *var_dleq_dvd_slot = var_dleq_dvd;
        *var_dleq_dvd_dn0_slot = var_dleq_dvd_dn0;
        *var_dleq_dvd_dn1_slot = var_dleq_dvd_dn1;
        *var_dleq_dvd_dn2_slot = var_dleq_dvd_dn2;
        *var_dleq_dvd_dn3_slot = var_dleq_dvd_dn3;
        *var_dleq_dvd_rv_slot = var_dleq_dvd_rv;
        *var_dleq_dvs_slot = var_dleq_dvs;
        *var_dleq_dvs_dn0_slot = var_dleq_dvs_dn0;
        *var_dleq_dvs_dn1_slot = var_dleq_dvs_dn1;
        *var_dleq_dvs_dn2_slot = var_dleq_dvs_dn2;
        *var_dleq_dvs_dn3_slot = var_dleq_dvs_dn3;
        *var_dleq_dvs_rv_slot = var_dleq_dvs_rv;
        *var_dn_dvd_slot = var_dn_dvd;
        *var_dn_dvd_dn0_slot = var_dn_dvd_dn0;
        *var_dn_dvd_dn1_slot = var_dn_dvd_dn1;
        *var_dn_dvd_dn2_slot = var_dn_dvd_dn2;
        *var_dn_dvd_dn3_slot = var_dn_dvd_dn3;
        *var_dn_dvd_rv_slot = var_dn_dvd_rv;
        *var_dn_dvs_slot = var_dn_dvs;
        *var_dn_dvs_dn0_slot = var_dn_dvs_dn0;
        *var_dn_dvs_dn1_slot = var_dn_dvs_dn1;
        *var_dn_dvs_dn2_slot = var_dn_dvs_dn2;
        *var_dn_dvs_dn3_slot = var_dn_dvs_dn3;
        *var_dn_dvs_rv_slot = var_dn_dvs_rv;
        *var_dqb_dvd_slot = var_dqb_dvd;
        *var_dqb_dvd_dn0_slot = var_dqb_dvd_dn0;
        *var_dqb_dvd_dn1_slot = var_dqb_dvd_dn1;
        *var_dqb_dvd_dn2_slot = var_dqb_dvd_dn2;
        *var_dqb_dvd_dn3_slot = var_dqb_dvd_dn3;
        *var_dqb_dvd_rv_slot = var_dqb_dvd_rv;
        *var_dqb_dvs_slot = var_dqb_dvs;
        *var_dqb_dvs_dn0_slot = var_dqb_dvs_dn0;
        *var_dqb_dvs_dn1_slot = var_dqb_dvs_dn1;
        *var_dqb_dvs_dn2_slot = var_dqb_dvs_dn2;
        *var_dqb_dvs_dn3_slot = var_dqb_dvs_dn3;
        *var_dqb_dvs_rv_slot = var_dqb_dvs_rv;
        *var_dqi_dvd_slot = var_dqi_dvd;
        *var_dqi_dvd_dn0_slot = var_dqi_dvd_dn0;
        *var_dqi_dvd_dn1_slot = var_dqi_dvd_dn1;
        *var_dqi_dvd_dn2_slot = var_dqi_dvd_dn2;
        *var_dqi_dvd_dn3_slot = var_dqi_dvd_dn3;
        *var_dqi_dvd_rv_slot = var_dqi_dvd_rv;
        *var_dqi_dvs_slot = var_dqi_dvs;
        *var_dqi_dvs_dn0_slot = var_dqi_dvs_dn0;
        *var_dqi_dvs_dn1_slot = var_dqi_dvs_dn1;
        *var_dqi_dvs_dn2_slot = var_dqi_dvs_dn2;
        *var_dqi_dvs_dn3_slot = var_dqi_dvs_dn3;
        *var_dqi_dvs_rv_slot = var_dqi_dvs_rv;
        *var_dvpprime_dvd_slot = var_dvpprime_dvd;
        *var_dvpprime_dvd_dn0_slot = var_dvpprime_dvd_dn0;
        *var_dvpprime_dvd_dn1_slot = var_dvpprime_dvd_dn1;
        *var_dvpprime_dvd_dn2_slot = var_dvpprime_dvd_dn2;
        *var_dvpprime_dvd_dn3_slot = var_dvpprime_dvd_dn3;
        *var_dvpprime_dvd_rv_slot = var_dvpprime_dvd_rv;
        *var_dvpprime_dvs_slot = var_dvpprime_dvs;
        *var_dvpprime_dvs_dn0_slot = var_dvpprime_dvs_dn0;
        *var_dvpprime_dvs_dn1_slot = var_dvpprime_dvs_dn1;
        *var_dvpprime_dvs_dn2_slot = var_dvpprime_dvs_dn2;
        *var_dvpprime_dvs_dn3_slot = var_dvpprime_dvs_dn3;
        *var_dvpprime_dvs_rv_slot = var_dvpprime_dvs_rv;
        *var_gds_slot = var_gds;
        *var_gds_dn0_slot = var_gds_dn0;
        *var_gds_dn1_slot = var_gds_dn1;
        *var_gds_dn2_slot = var_gds_dn2;
        *var_gds_dn3_slot = var_gds_dn3;
        *var_gds_rv_slot = var_gds_rv;
        *var_gms_slot = var_gms;
        *var_gms_dn0_slot = var_gms_dn0;
        *var_gms_dn1_slot = var_gms_dn1;
        *var_gms_dn2_slot = var_gms_dn2;
        *var_gms_dn3_slot = var_gms_dn3;
        *var_gms_rv_slot = var_gms_rv;
        *var_guard18_slot = var_guard18;
        *var_guard18_rv_slot = var_guard18_rv;
        *var_n_vt_cox_slot = var_n_vt_cox;
        *var_n_vt_cox_dn0_slot = var_n_vt_cox_dn0;
        *var_n_vt_cox_dn1_slot = var_n_vt_cox_dn1;
        *var_n_vt_cox_dn2_slot = var_n_vt_cox_dn2;
        *var_n_vt_cox_dn3_slot = var_n_vt_cox_dn3;
        *var_n_vt_cox_rv_slot = var_n_vt_cox_rv;
        *var_qd_slot = var_qd;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn1_slot = var_qd_dn1;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn3_slot = var_qd_dn3;
        *var_qd_rv_slot = var_qd_rv;
        *var_qi_1_slot = var_qi_1;
        *var_qi_1_dn0_slot = var_qi_1_dn0;
        *var_qi_1_dn1_slot = var_qi_1_dn1;
        *var_qi_1_dn2_slot = var_qi_1_dn2;
        *var_qi_1_dn3_slot = var_qi_1_dn3;
        *var_qi_1_rv_slot = var_qi_1_rv;
        *var_qs_slot = var_qs;
        *var_qs_dn0_slot = var_qs_dn0;
        *var_qs_dn1_slot = var_qs_dn1;
        *var_qs_dn2_slot = var_qs_dn2;
        *var_qs_dn3_slot = var_qs_dn3;
        *var_qs_rv_slot = var_qs_rv;
        *var_rdeff_slot = var_rdeff;
        *var_rdeff_rv_slot = var_rdeff_rv;
        *var_rseff_slot = var_rseff;
        *var_rseff_rv_slot = var_rseff_rv;
        *var_sif3_slot = var_sif3;
        *var_sif3_dn0_slot = var_sif3_dn0;
        *var_sif3_dn1_slot = var_sif3_dn1;
        *var_sif3_dn2_slot = var_sif3_dn2;
        *var_sif3_dn3_slot = var_sif3_dn3;
        *var_sif3_rv_slot = var_sif3_rv;
        *var_sir3_slot = var_sir3;
        *var_sir3_dn0_slot = var_sir3_dn0;
        *var_sir3_dn1_slot = var_sir3_dn1;
        *var_sir3_dn2_slot = var_sir3_dn2;
        *var_sir3_dn3_slot = var_sir3_dn3;
        *var_sir3_rv_slot = var_sir3_rv;
        *var_sqrt_phi_vp2_2_slot = var_sqrt_phi_vp2_2;
        *var_sqrt_phi_vp2_2_dn0_slot = var_sqrt_phi_vp2_2_dn0;
        *var_sqrt_phi_vp2_2_dn1_slot = var_sqrt_phi_vp2_2_dn1;
        *var_sqrt_phi_vp2_2_dn2_slot = var_sqrt_phi_vp2_2_dn2;
        *var_sqrt_phi_vp2_2_dn3_slot = var_sqrt_phi_vp2_2_dn3;
        *var_sqrt_phi_vp2_2_rv_slot = var_sqrt_phi_vp2_2_rv;
        *var_tmp1_slot = var_tmp1;
        *var_tmp1_dn0_slot = var_tmp1_dn0;
        *var_tmp1_dn1_slot = var_tmp1_dn1;
        *var_tmp1_dn2_slot = var_tmp1_dn2;
        *var_tmp1_dn3_slot = var_tmp1_dn3;
        *var_tmp1_rv_slot = var_tmp1_rv;
        *var_tmp2_slot = var_tmp2;
        *var_tmp2_dn0_slot = var_tmp2_dn0;
        *var_tmp2_dn1_slot = var_tmp2_dn1;
        *var_tmp2_dn2_slot = var_tmp2_dn2;
        *var_tmp2_dn3_slot = var_tmp2_dn3;
        *var_tmp2_rv_slot = var_tmp2_rv;
        *var_tmp3_slot = var_tmp3;
        *var_tmp3_dn0_slot = var_tmp3_dn0;
        *var_tmp3_dn1_slot = var_tmp3_dn1;
        *var_tmp3_dn2_slot = var_tmp3_dn2;
        *var_tmp3_dn3_slot = var_tmp3_dn3;
        *var_tmp3_rv_slot = var_tmp3_rv;
        *var_wlcox_slot = var_wlcox;
        *var_wlcox_rv_slot = var_wlcox_rv;
    }

    pub(super) fn stamp_reactive_block_5(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_deltat: f64,
        var_gammaprime: f64,
        var_gammaprime_dn0: f64,
        var_gammaprime_dn1: f64,
        var_gammaprime_dn2: f64,
        var_gammaprime_dn3: f64,
        var_mode: f64,
        var_qd: f64,
        var_qd_dn0: f64,
        var_qd_dn1: f64,
        var_qd_dn2: f64,
        var_qd_dn3: f64,
        var_qi_1: f64,
        var_qi_1_dn0: f64,
        var_qi_1_dn1: f64,
        var_qi_1_dn2: f64,
        var_qi_1_dn3: f64,
        var_qs: f64,
        var_qs_dn0: f64,
        var_qs_dn1: f64,
        var_qs_dn2: f64,
        var_qs_dn3: f64,
        var_sqrt_phi_vp2_2: f64,
        var_sqrt_phi_vp2_2_dn0: f64,
        var_sqrt_phi_vp2_2_dn1: f64,
        var_sqrt_phi_vp2_2_dn2: f64,
        var_sqrt_phi_vp2_2_dn3: f64,
        var_sqrt_phi_vp_2: f64,
        var_sqrt_phi_vp_2_dn0: f64,
        var_sqrt_phi_vp_2_dn1: f64,
        var_sqrt_phi_vp_2_dn2: f64,
        var_sqrt_phi_vp_2_dn3: f64,
        var_vgprime: f64,
        var_vgprime_dn0: f64,
        var_vgprime_dn1: f64,
        var_vgprime_dn2: f64,
        var_vgprime_dn3: f64,
        var_vgstar: f64,
        var_vgstar_dn0: f64,
        var_vgstar_dn1: f64,
        var_vgstar_dn2: f64,
        var_vgstar_dn3: f64,
        var_weff: f64,
        var_wlcox: f64,
        var_ad_i_slot: &mut f64,
        var_ad_i_rv_slot: &mut f64,
        var_as_i_slot: &mut f64,
        var_as_i_rv_slot: &mut f64,
        var_cj_t_slot: &mut f64,
        var_cj_t_rv_slot: &mut f64,
        var_cjsw_t_slot: &mut f64,
        var_cjsw_t_rv_slot: &mut f64,
        var_cjswg_t_slot: &mut f64,
        var_cjswg_t_rv_slot: &mut f64,
        var_csb_d_slot: &mut f64,
        var_csb_d_dn0_slot: &mut f64,
        var_csb_d_dn3_slot: &mut f64,
        var_csb_d_rv_slot: &mut f64,
        var_csb_s_slot: &mut f64,
        var_csb_s_dn2_slot: &mut f64,
        var_csb_s_dn3_slot: &mut f64,
        var_csb_s_rv_slot: &mut f64,
        var_cssw_d_slot: &mut f64,
        var_cssw_d_dn0_slot: &mut f64,
        var_cssw_d_dn3_slot: &mut f64,
        var_cssw_d_rv_slot: &mut f64,
        var_cssw_s_slot: &mut f64,
        var_cssw_s_dn2_slot: &mut f64,
        var_cssw_s_dn3_slot: &mut f64,
        var_cssw_s_rv_slot: &mut f64,
        var_csswg_d_slot: &mut f64,
        var_csswg_d_dn0_slot: &mut f64,
        var_csswg_d_dn3_slot: &mut f64,
        var_csswg_d_rv_slot: &mut f64,
        var_csswg_s_slot: &mut f64,
        var_csswg_s_dn2_slot: &mut f64,
        var_csswg_s_dn3_slot: &mut f64,
        var_csswg_s_rv_slot: &mut f64,
        var_ddt_qd_slot: &mut f64,
        var_ddt_qd_dn0_slot: &mut f64,
        var_ddt_qd_dn1_slot: &mut f64,
        var_ddt_qd_dn2_slot: &mut f64,
        var_ddt_qd_dn3_slot: &mut f64,
        var_ddt_qd_rdn0_slot: &mut f64,
        var_ddt_qd_rdn1_slot: &mut f64,
        var_ddt_qd_rdn2_slot: &mut f64,
        var_ddt_qd_rdn3_slot: &mut f64,
        var_ddt_qd_rv_slot: &mut f64,
        var_ddt_qs_slot: &mut f64,
        var_ddt_qs_dn0_slot: &mut f64,
        var_ddt_qs_dn1_slot: &mut f64,
        var_ddt_qs_dn2_slot: &mut f64,
        var_ddt_qs_dn3_slot: &mut f64,
        var_ddt_qs_rdn0_slot: &mut f64,
        var_ddt_qs_rdn1_slot: &mut f64,
        var_ddt_qs_rdn2_slot: &mut f64,
        var_ddt_qs_rdn3_slot: &mut f64,
        var_ddt_qs_rv_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard21_rv_slot: &mut f64,
        var_guard24_slot: &mut f64,
        var_guard24_rv_slot: &mut f64,
        var_guard25_slot: &mut f64,
        var_guard25_rv_slot: &mut f64,
        var_guard26_slot: &mut f64,
        var_guard26_rv_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard27_rv_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard32_rv_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard33_rv_slot: &mut f64,
        var_pb_t_slot: &mut f64,
        var_pb_t_rv_slot: &mut f64,
        var_pbsw_t_slot: &mut f64,
        var_pbsw_t_rv_slot: &mut f64,
        var_pbswg_t_slot: &mut f64,
        var_pbswg_t_rv_slot: &mut f64,
        var_pd_i_slot: &mut f64,
        var_pd_i_rv_slot: &mut f64,
        var_ps_i_slot: &mut f64,
        var_ps_i_rv_slot: &mut f64,
        var_qb_1_slot: &mut f64,
        var_qb_1_dn0_slot: &mut f64,
        var_qb_1_dn1_slot: &mut f64,
        var_qb_1_dn2_slot: &mut f64,
        var_qb_1_dn3_slot: &mut f64,
        var_qb_1_rv_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn0_slot: &mut f64,
        var_qg_dn1_slot: &mut f64,
        var_qg_dn2_slot: &mut f64,
        var_qg_dn3_slot: &mut f64,
        var_qg_rv_slot: &mut f64,
        var_qjd_slot: &mut f64,
        var_qjd_dn0_slot: &mut f64,
        var_qjd_dn3_slot: &mut f64,
        var_qjd_rv_slot: &mut f64,
        var_qjs_slot: &mut f64,
        var_qjs_dn2_slot: &mut f64,
        var_qjs_dn3_slot: &mut f64,
        var_qjs_rv_slot: &mut f64,
        var_v_di_b_slot: &mut f64,
        var_v_di_b_dn0_slot: &mut f64,
        var_v_di_b_dn3_slot: &mut f64,
        var_v_di_b_rv_slot: &mut f64,
        var_v_si_b_slot: &mut f64,
        var_v_si_b_dn2_slot: &mut f64,
        var_v_si_b_dn3_slot: &mut f64,
        var_v_si_b_rv_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_ad_i: f64 = *var_ad_i_slot;
        let mut var_ad_i_rv: f64 = *var_ad_i_rv_slot;
        let mut var_as_i: f64 = *var_as_i_slot;
        let mut var_as_i_rv: f64 = *var_as_i_rv_slot;
        let mut var_cj_t: f64 = *var_cj_t_slot;
        let mut var_cj_t_rv: f64 = *var_cj_t_rv_slot;
        let mut var_cjsw_t: f64 = *var_cjsw_t_slot;
        let mut var_cjsw_t_rv: f64 = *var_cjsw_t_rv_slot;
        let mut var_cjswg_t: f64 = *var_cjswg_t_slot;
        let mut var_cjswg_t_rv: f64 = *var_cjswg_t_rv_slot;
        let mut var_csb_d: f64 = *var_csb_d_slot;
        let mut var_csb_d_dn0: f64 = *var_csb_d_dn0_slot;
        let mut var_csb_d_dn3: f64 = *var_csb_d_dn3_slot;
        let mut var_csb_d_rv: f64 = *var_csb_d_rv_slot;
        let mut var_csb_s: f64 = *var_csb_s_slot;
        let mut var_csb_s_dn2: f64 = *var_csb_s_dn2_slot;
        let mut var_csb_s_dn3: f64 = *var_csb_s_dn3_slot;
        let mut var_csb_s_rv: f64 = *var_csb_s_rv_slot;
        let mut var_cssw_d: f64 = *var_cssw_d_slot;
        let mut var_cssw_d_dn0: f64 = *var_cssw_d_dn0_slot;
        let mut var_cssw_d_dn3: f64 = *var_cssw_d_dn3_slot;
        let mut var_cssw_d_rv: f64 = *var_cssw_d_rv_slot;
        let mut var_cssw_s: f64 = *var_cssw_s_slot;
        let mut var_cssw_s_dn2: f64 = *var_cssw_s_dn2_slot;
        let mut var_cssw_s_dn3: f64 = *var_cssw_s_dn3_slot;
        let mut var_cssw_s_rv: f64 = *var_cssw_s_rv_slot;
        let mut var_csswg_d: f64 = *var_csswg_d_slot;
        let mut var_csswg_d_dn0: f64 = *var_csswg_d_dn0_slot;
        let mut var_csswg_d_dn3: f64 = *var_csswg_d_dn3_slot;
        let mut var_csswg_d_rv: f64 = *var_csswg_d_rv_slot;
        let mut var_csswg_s: f64 = *var_csswg_s_slot;
        let mut var_csswg_s_dn2: f64 = *var_csswg_s_dn2_slot;
        let mut var_csswg_s_dn3: f64 = *var_csswg_s_dn3_slot;
        let mut var_csswg_s_rv: f64 = *var_csswg_s_rv_slot;
        let mut var_ddt_qd: f64 = *var_ddt_qd_slot;
        let mut var_ddt_qd_dn0: f64 = *var_ddt_qd_dn0_slot;
        let mut var_ddt_qd_dn1: f64 = *var_ddt_qd_dn1_slot;
        let mut var_ddt_qd_dn2: f64 = *var_ddt_qd_dn2_slot;
        let mut var_ddt_qd_dn3: f64 = *var_ddt_qd_dn3_slot;
        let mut var_ddt_qd_rdn0: f64 = *var_ddt_qd_rdn0_slot;
        let mut var_ddt_qd_rdn1: f64 = *var_ddt_qd_rdn1_slot;
        let mut var_ddt_qd_rdn2: f64 = *var_ddt_qd_rdn2_slot;
        let mut var_ddt_qd_rdn3: f64 = *var_ddt_qd_rdn3_slot;
        let mut var_ddt_qd_rv: f64 = *var_ddt_qd_rv_slot;
        let mut var_ddt_qs: f64 = *var_ddt_qs_slot;
        let mut var_ddt_qs_dn0: f64 = *var_ddt_qs_dn0_slot;
        let mut var_ddt_qs_dn1: f64 = *var_ddt_qs_dn1_slot;
        let mut var_ddt_qs_dn2: f64 = *var_ddt_qs_dn2_slot;
        let mut var_ddt_qs_dn3: f64 = *var_ddt_qs_dn3_slot;
        let mut var_ddt_qs_rdn0: f64 = *var_ddt_qs_rdn0_slot;
        let mut var_ddt_qs_rdn1: f64 = *var_ddt_qs_rdn1_slot;
        let mut var_ddt_qs_rdn2: f64 = *var_ddt_qs_rdn2_slot;
        let mut var_ddt_qs_rdn3: f64 = *var_ddt_qs_rdn3_slot;
        let mut var_ddt_qs_rv: f64 = *var_ddt_qs_rv_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard21_rv: f64 = *var_guard21_rv_slot;
        let mut var_guard24: f64 = *var_guard24_slot;
        let mut var_guard24_rv: f64 = *var_guard24_rv_slot;
        let mut var_guard25: f64 = *var_guard25_slot;
        let mut var_guard25_rv: f64 = *var_guard25_rv_slot;
        let mut var_guard26: f64 = *var_guard26_slot;
        let mut var_guard26_rv: f64 = *var_guard26_rv_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard27_rv: f64 = *var_guard27_rv_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard32_rv: f64 = *var_guard32_rv_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard33_rv: f64 = *var_guard33_rv_slot;
        let mut var_pb_t: f64 = *var_pb_t_slot;
        let mut var_pb_t_rv: f64 = *var_pb_t_rv_slot;
        let mut var_pbsw_t: f64 = *var_pbsw_t_slot;
        let mut var_pbsw_t_rv: f64 = *var_pbsw_t_rv_slot;
        let mut var_pbswg_t: f64 = *var_pbswg_t_slot;
        let mut var_pbswg_t_rv: f64 = *var_pbswg_t_rv_slot;
        let mut var_pd_i: f64 = *var_pd_i_slot;
        let mut var_pd_i_rv: f64 = *var_pd_i_rv_slot;
        let mut var_ps_i: f64 = *var_ps_i_slot;
        let mut var_ps_i_rv: f64 = *var_ps_i_rv_slot;
        let mut var_qb_1: f64 = *var_qb_1_slot;
        let mut var_qb_1_dn0: f64 = *var_qb_1_dn0_slot;
        let mut var_qb_1_dn1: f64 = *var_qb_1_dn1_slot;
        let mut var_qb_1_dn2: f64 = *var_qb_1_dn2_slot;
        let mut var_qb_1_dn3: f64 = *var_qb_1_dn3_slot;
        let mut var_qb_1_rv: f64 = *var_qb_1_rv_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn0: f64 = *var_qg_dn0_slot;
        let mut var_qg_dn1: f64 = *var_qg_dn1_slot;
        let mut var_qg_dn2: f64 = *var_qg_dn2_slot;
        let mut var_qg_dn3: f64 = *var_qg_dn3_slot;
        let mut var_qg_rv: f64 = *var_qg_rv_slot;
        let mut var_qjd: f64 = *var_qjd_slot;
        let mut var_qjd_dn0: f64 = *var_qjd_dn0_slot;
        let mut var_qjd_dn3: f64 = *var_qjd_dn3_slot;
        let mut var_qjd_rv: f64 = *var_qjd_rv_slot;
        let mut var_qjs: f64 = *var_qjs_slot;
        let mut var_qjs_dn2: f64 = *var_qjs_dn2_slot;
        let mut var_qjs_dn3: f64 = *var_qjs_dn3_slot;
        let mut var_qjs_rv: f64 = *var_qjs_rv_slot;
        let mut var_v_di_b: f64 = *var_v_di_b_slot;
        let mut var_v_di_b_dn0: f64 = *var_v_di_b_dn0_slot;
        let mut var_v_di_b_dn3: f64 = *var_v_di_b_dn3_slot;
        let mut var_v_di_b_rv: f64 = *var_v_di_b_rv_slot;
        let mut var_v_si_b: f64 = *var_v_si_b_slot;
        let mut var_v_si_b_dn2: f64 = *var_v_si_b_dn2_slot;
        let mut var_v_si_b_dn3: f64 = *var_v_si_b_dn3_slot;
        let mut var_v_si_b_rv: f64 = *var_v_si_b_rv_slot;

        let assign2890_e2251: f64 = (-0.5);
        let assign2890_e2253: f64 = (assign2890_e2251 * var_gammaprime);
        let assign2890_e2255: f64 = (assign2890_e2253 * var_sqrt_phi_vp_2);
        let assign2890_e2257: f64 = (assign2890_e2255 + var_vgprime);
        let assign2890_e2259: f64 = (assign2890_e2257 - var_vgstar);
        let assign2890_e2260: f64 = (var_wlcox * assign2890_e2259);
        let assign2890_e2263: f64 = (var_qi_1 * var_gammaprime);
        let assign2890_e2266: f64 = (var_gammaprime + var_sqrt_phi_vp2_2);
        let assign2890_e2267: f64 = (assign2890_e2263 / assign2890_e2266);
        let assign2890_e2268: f64 = (assign2890_e2260 - assign2890_e2267);
        var_qb_1 = assign2890_e2268;
        var_qb_1_dn0 = ((var_wlcox * (((((assign2890_e2251 * var_gammaprime_dn0) * var_sqrt_phi_vp_2) + (assign2890_e2253 * var_sqrt_phi_vp_2_dn0)) + var_vgprime_dn0) - var_vgstar_dn0)) - (((((var_qi_1_dn0 * var_gammaprime) + (var_qi_1 * var_gammaprime_dn0)) * assign2890_e2266) - (assign2890_e2263 * (var_gammaprime_dn0 + var_sqrt_phi_vp2_2_dn0))) / (assign2890_e2266 * assign2890_e2266)));
        var_qb_1_dn1 = ((var_wlcox * (((((assign2890_e2251 * var_gammaprime_dn1) * var_sqrt_phi_vp_2) + (assign2890_e2253 * var_sqrt_phi_vp_2_dn1)) + var_vgprime_dn1) - var_vgstar_dn1)) - (((((var_qi_1_dn1 * var_gammaprime) + (var_qi_1 * var_gammaprime_dn1)) * assign2890_e2266) - (assign2890_e2263 * (var_gammaprime_dn1 + var_sqrt_phi_vp2_2_dn1))) / (assign2890_e2266 * assign2890_e2266)));
        var_qb_1_dn2 = ((var_wlcox * (((((assign2890_e2251 * var_gammaprime_dn2) * var_sqrt_phi_vp_2) + (assign2890_e2253 * var_sqrt_phi_vp_2_dn2)) + var_vgprime_dn2) - var_vgstar_dn2)) - (((((var_qi_1_dn2 * var_gammaprime) + (var_qi_1 * var_gammaprime_dn2)) * assign2890_e2266) - (assign2890_e2263 * (var_gammaprime_dn2 + var_sqrt_phi_vp2_2_dn2))) / (assign2890_e2266 * assign2890_e2266)));
        var_qb_1_dn3 = ((var_wlcox * (((((assign2890_e2251 * var_gammaprime_dn3) * var_sqrt_phi_vp_2) + (assign2890_e2253 * var_sqrt_phi_vp_2_dn3)) + var_vgprime_dn3) - var_vgstar_dn3)) - (((((var_qi_1_dn3 * var_gammaprime) + (var_qi_1 * var_gammaprime_dn3)) * assign2890_e2266) - (assign2890_e2263 * (var_gammaprime_dn3 + var_sqrt_phi_vp2_2_dn3))) / (assign2890_e2266 * assign2890_e2266)));
        var_qb_1_rv = 0.0;

        let assign2900_e2270: f64 = (-var_qi_1);
        let assign2900_e2272: f64 = (assign2900_e2270 - var_qb_1);
        var_qg = assign2900_e2272;
        var_qg_dn0 = ((-var_qi_1_dn0) - var_qb_1_dn0);
        var_qg_dn1 = ((-var_qi_1_dn1) - var_qb_1_dn1);
        var_qg_dn2 = ((-var_qi_1_dn2) - var_qb_1_dn2);
        var_qg_dn3 = ((-var_qi_1_dn3) - var_qb_1_dn3);
        var_qg_rv = 0.0;

        let assign2910_e2274_q: f64 = var_qd;
        var_ddt_qd = var_qd;
        var_ddt_qd_dn0 = var_qd_dn0;
        var_ddt_qd_dn1 = var_qd_dn1;
        var_ddt_qd_dn2 = var_qd_dn2;
        var_ddt_qd_dn3 = var_qd_dn3;
        var_ddt_qd_rv = assign2910_e2274_q;
        var_ddt_qd_rdn0 = var_qd_dn0;
        var_ddt_qd_rdn1 = var_qd_dn1;
        var_ddt_qd_rdn2 = var_qd_dn2;
        var_ddt_qd_rdn3 = var_qd_dn3;

        let assign2920_e2276_q: f64 = var_qs;
        var_ddt_qs = var_qs;
        var_ddt_qs_dn0 = var_qs_dn0;
        var_ddt_qs_dn1 = var_qs_dn1;
        var_ddt_qs_dn2 = var_qs_dn2;
        var_ddt_qs_dn3 = var_qs_dn3;
        var_ddt_qs_rv = assign2920_e2276_q;
        var_ddt_qs_rdn0 = var_qs_dn0;
        var_ddt_qs_rdn1 = var_qs_dn1;
        var_ddt_qs_rdn2 = var_qs_dn2;
        var_ddt_qs_rdn3 = var_qs_dn3;

        let assign2930_e2279: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard21 = assign2930_e2279;
        var_guard21_rv = 0.0;

        let assign2960_e2312: f64 = if ((p.p9 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        var_guard24 = assign2960_e2312;
        var_guard24_rv = 0.0;

        let (assign2970_e2320,) = {
    if (var_guard24 != 0.0) {
        let assign2970_e2316: f64 = (2.0 * p.p37);
        let assign2970_e2318: f64 = (assign2970_e2316 * var_weff);
        (assign2970_e2318,)
    } else {
        (var_as_i,)
    }
};
        var_as_i = assign2970_e2320;
        var_as_i_rv = 0.0;

        let (assign2980_e2325,) = {
    if (var_guard24 == 0.0) {
        (p.p9,)
    } else {
        (var_as_i,)
    }
};
        var_as_i = assign2980_e2325;
        var_as_i_rv = 0.0;

        let assign2990_e2332: f64 = if ((p.p11 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        var_guard25 = assign2990_e2332;
        var_guard25_rv = 0.0;

        let (assign3000_e2342,) = {
    if (var_guard25 != 0.0) {
        let assign3000_e2336: f64 = (4.0 * p.p37);
        let assign3000_e2339: f64 = var_weff;
        let assign3000_e2340: f64 = (assign3000_e2336 + assign3000_e2339);
        (assign3000_e2340,)
    } else {
        (var_ps_i,)
    }
};
        var_ps_i = assign3000_e2342;
        var_ps_i_rv = 0.0;

        let (assign3010_e2347,) = {
    if (var_guard25 == 0.0) {
        (p.p11,)
    } else {
        (var_ps_i,)
    }
};
        var_ps_i = assign3010_e2347;
        var_ps_i_rv = 0.0;

        let assign3020_e2354: f64 = if ((p.p10 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        var_guard26 = assign3020_e2354;
        var_guard26_rv = 0.0;

        let (assign3030_e2362,) = {
    if (var_guard26 != 0.0) {
        let assign3030_e2358: f64 = (2.0 * p.p37);
        let assign3030_e2360: f64 = (assign3030_e2358 * var_weff);
        (assign3030_e2360,)
    } else {
        (var_ad_i,)
    }
};
        var_ad_i = assign3030_e2362;
        var_ad_i_rv = 0.0;

        let (assign3040_e2367,) = {
    if (var_guard26 == 0.0) {
        (p.p10,)
    } else {
        (var_ad_i,)
    }
};
        var_ad_i = assign3040_e2367;
        var_ad_i_rv = 0.0;

        let assign3050_e2374: f64 = if ((p.p12 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        var_guard27 = assign3050_e2374;
        var_guard27_rv = 0.0;

        let (assign3060_e2384,) = {
    if (var_guard27 != 0.0) {
        let assign3060_e2378: f64 = (4.0 * p.p37);
        let assign3060_e2381: f64 = var_weff;
        let assign3060_e2382: f64 = (assign3060_e2378 + assign3060_e2381);
        (assign3060_e2382,)
    } else {
        (var_pd_i,)
    }
};
        var_pd_i = assign3060_e2384;
        var_pd_i_rv = 0.0;

        let (assign3070_e2389,) = {
    if (var_guard27 == 0.0) {
        (p.p12,)
    } else {
        (var_pd_i,)
    }
};
        var_pd_i = assign3070_e2389;
        var_pd_i_rv = 0.0;

        let assign3120_e2418: f64 = (p.p69 * var_deltat);
        let assign3120_e2419: f64 = (p.p50 - assign3120_e2418);
        var_pb_t = assign3120_e2419;
        var_pb_t_rv = 0.0;

        let assign3130_e2423: f64 = (p.p70 * var_deltat);
        let assign3130_e2424: f64 = (p.p51 - assign3130_e2423);
        var_pbsw_t = assign3130_e2424;
        var_pbsw_t_rv = 0.0;

        let assign3140_e2428: f64 = (p.p71 * var_deltat);
        let assign3140_e2429: f64 = (p.p52 - assign3140_e2428);
        var_pbswg_t = assign3140_e2429;
        var_pbswg_t_rv = 0.0;

        let assign3150_e2434: f64 = (p.p66 * var_deltat);
        let assign3150_e2435: f64 = (1.0 + assign3150_e2434);
        let assign3150_e2436: f64 = (p.p53 * assign3150_e2435);
        var_cj_t = assign3150_e2436;
        var_cj_t_rv = 0.0;

        let assign3160_e2441: f64 = (p.p67 * var_deltat);
        let assign3160_e2442: f64 = (1.0 + assign3160_e2441);
        let assign3160_e2443: f64 = (p.p54 * assign3160_e2442);
        var_cjsw_t = assign3160_e2443;
        var_cjsw_t_rv = 0.0;

        let assign3170_e2448: f64 = (p.p68 * var_deltat);
        let assign3170_e2449: f64 = (1.0 + assign3170_e2448);
        let assign3170_e2450: f64 = (p.p55 * assign3170_e2449);
        var_cjswg_t = assign3170_e2450;
        var_cjswg_t_rv = 0.0;

        let assign3210_e2480: f64 = (p.p0 * (nv0 - nv3));
        var_v_di_b = assign3210_e2480;
        var_v_di_b_dn0 = p.p0;
        var_v_di_b_dn3 = (-p.p0);
        var_v_di_b_rv = 0.0;

        let assign3220_e2483: f64 = (p.p0 * (nv2 - nv3));
        var_v_si_b = assign3220_e2483;
        var_v_si_b_dn2 = p.p0;
        var_v_si_b_dn3 = (-p.p0);
        var_v_si_b_rv = 0.0;

        let assign3450_e2740: f64 = if var_v_di_b > 0.0 { 1.0 } else { 0.0 };
        var_guard32 = assign3450_e2740;
        var_guard32_rv = 0.0;

        let (assign3460_e2757, assign3460_e2757_d_n0, assign3460_e2757_d_n3,) = {
    if (var_guard32 != 0.0) {
        let assign3460_e2744: f64 = (var_cj_t * var_ad_i);
        let assign3460_e2746: f64 = (-p.p47);
        let assign3460_e2750: f64 = (var_v_di_b / var_pb_t);
        let assign3460_e2751: f64 = (1.0 + assign3460_e2750);
        let assign3460_e2752: f64 = (assign3460_e2751).ln();
        let assign3460_e2753: f64 = (assign3460_e2746 * assign3460_e2752);
        let assign3460_e2754: f64 = (assign3460_e2753).exp();
        let assign3460_e2755: f64 = (assign3460_e2744 * assign3460_e2754);
        (assign3460_e2755, (assign3460_e2744 * (assign3460_e2754 * (assign3460_e2746 * ((var_v_di_b_dn0 / var_pb_t) / assign3460_e2751)))), (assign3460_e2744 * (assign3460_e2754 * (assign3460_e2746 * ((var_v_di_b_dn3 / var_pb_t) / assign3460_e2751)))),)
    } else {
        (var_csb_d, var_csb_d_dn0, var_csb_d_dn3,)
    }
};
        var_csb_d = assign3460_e2757;
        var_csb_d_dn0 = assign3460_e2757_d_n0;
        var_csb_d_dn3 = assign3460_e2757_d_n3;
        var_csb_d_rv = 0.0;

        let (assign3470_e2774, assign3470_e2774_d_n0, assign3470_e2774_d_n3,) = {
    if (var_guard32 != 0.0) {
        let assign3470_e2761: f64 = (var_cjsw_t * var_pd_i);
        let assign3470_e2763: f64 = (-p.p48);
        let assign3470_e2767: f64 = (var_v_di_b / var_pbsw_t);
        let assign3470_e2768: f64 = (1.0 + assign3470_e2767);
        let assign3470_e2769: f64 = (assign3470_e2768).ln();
        let assign3470_e2770: f64 = (assign3470_e2763 * assign3470_e2769);
        let assign3470_e2771: f64 = (assign3470_e2770).exp();
        let assign3470_e2772: f64 = (assign3470_e2761 * assign3470_e2771);
        (assign3470_e2772, (assign3470_e2761 * (assign3470_e2771 * (assign3470_e2763 * ((var_v_di_b_dn0 / var_pbsw_t) / assign3470_e2768)))), (assign3470_e2761 * (assign3470_e2771 * (assign3470_e2763 * ((var_v_di_b_dn3 / var_pbsw_t) / assign3470_e2768)))),)
    } else {
        (var_cssw_d, var_cssw_d_dn0, var_cssw_d_dn3,)
    }
};
        var_cssw_d = assign3470_e2774;
        var_cssw_d_dn0 = assign3470_e2774_d_n0;
        var_cssw_d_dn3 = assign3470_e2774_d_n3;
        var_cssw_d_rv = 0.0;

        let (assign3480_e2791, assign3480_e2791_d_n0, assign3480_e2791_d_n3,) = {
    if (var_guard32 != 0.0) {
        let assign3480_e2778: f64 = (var_cjswg_t * var_weff);
        let assign3480_e2780: f64 = (-p.p49);
        let assign3480_e2784: f64 = (var_v_di_b / var_pbswg_t);
        let assign3480_e2785: f64 = (1.0 + assign3480_e2784);
        let assign3480_e2786: f64 = (assign3480_e2785).ln();
        let assign3480_e2787: f64 = (assign3480_e2780 * assign3480_e2786);
        let assign3480_e2788: f64 = (assign3480_e2787).exp();
        let assign3480_e2789: f64 = (assign3480_e2778 * assign3480_e2788);
        (assign3480_e2789, (assign3480_e2778 * (assign3480_e2788 * (assign3480_e2780 * ((var_v_di_b_dn0 / var_pbswg_t) / assign3480_e2785)))), (assign3480_e2778 * (assign3480_e2788 * (assign3480_e2780 * ((var_v_di_b_dn3 / var_pbswg_t) / assign3480_e2785)))),)
    } else {
        (var_csswg_d, var_csswg_d_dn0, var_csswg_d_dn3,)
    }
};
        var_csswg_d = assign3480_e2791;
        var_csswg_d_dn0 = assign3480_e2791_d_n0;
        var_csswg_d_dn3 = assign3480_e2791_d_n3;
        var_csswg_d_rv = 0.0;

        let (assign3490_e2806, assign3490_e2806_d_n0, assign3490_e2806_d_n3,) = {
    if (var_guard32 == 0.0) {
        let assign3490_e2796: f64 = (var_cj_t * var_ad_i);
        let assign3490_e2800: f64 = (p.p47 * var_v_di_b);
        let assign3490_e2802: f64 = (assign3490_e2800 / var_pb_t);
        let assign3490_e2803: f64 = (1.0 - assign3490_e2802);
        let assign3490_e2804: f64 = (assign3490_e2796 * assign3490_e2803);
        (assign3490_e2804, (assign3490_e2796 * (-((p.p47 * var_v_di_b_dn0) / var_pb_t))), (assign3490_e2796 * (-((p.p47 * var_v_di_b_dn3) / var_pb_t))),)
    } else {
        (var_csb_d, var_csb_d_dn0, var_csb_d_dn3,)
    }
};
        var_csb_d = assign3490_e2806;
        var_csb_d_dn0 = assign3490_e2806_d_n0;
        var_csb_d_dn3 = assign3490_e2806_d_n3;
        var_csb_d_rv = 0.0;

        let (assign3500_e2821, assign3500_e2821_d_n0, assign3500_e2821_d_n3,) = {
    if (var_guard32 == 0.0) {
        let assign3500_e2811: f64 = (var_cjsw_t * var_pd_i);
        let assign3500_e2815: f64 = (p.p48 * var_v_di_b);
        let assign3500_e2817: f64 = (assign3500_e2815 / var_pbsw_t);
        let assign3500_e2818: f64 = (1.0 - assign3500_e2817);
        let assign3500_e2819: f64 = (assign3500_e2811 * assign3500_e2818);
        (assign3500_e2819, (assign3500_e2811 * (-((p.p48 * var_v_di_b_dn0) / var_pbsw_t))), (assign3500_e2811 * (-((p.p48 * var_v_di_b_dn3) / var_pbsw_t))),)
    } else {
        (var_cssw_d, var_cssw_d_dn0, var_cssw_d_dn3,)
    }
};
        var_cssw_d = assign3500_e2821;
        var_cssw_d_dn0 = assign3500_e2821_d_n0;
        var_cssw_d_dn3 = assign3500_e2821_d_n3;
        var_cssw_d_rv = 0.0;

        let (assign3510_e2836, assign3510_e2836_d_n0, assign3510_e2836_d_n3,) = {
    if (var_guard32 == 0.0) {
        let assign3510_e2826: f64 = (var_cjswg_t * var_weff);
        let assign3510_e2830: f64 = (p.p49 * var_v_di_b);
        let assign3510_e2832: f64 = (assign3510_e2830 / var_pbswg_t);
        let assign3510_e2833: f64 = (1.0 - assign3510_e2832);
        let assign3510_e2834: f64 = (assign3510_e2826 * assign3510_e2833);
        (assign3510_e2834, (assign3510_e2826 * (-((p.p49 * var_v_di_b_dn0) / var_pbswg_t))), (assign3510_e2826 * (-((p.p49 * var_v_di_b_dn3) / var_pbswg_t))),)
    } else {
        (var_csswg_d, var_csswg_d_dn0, var_csswg_d_dn3,)
    }
};
        var_csswg_d = assign3510_e2836;
        var_csswg_d_dn0 = assign3510_e2836_d_n0;
        var_csswg_d_dn3 = assign3510_e2836_d_n3;
        var_csswg_d_rv = 0.0;

        let assign3520_e2839: f64 = (var_csb_d + var_cssw_d);
        let assign3520_e2841: f64 = (assign3520_e2839 + var_csswg_d);
        let assign3520_e2843: f64 = (assign3520_e2841 * var_v_di_b);
        var_qjd = assign3520_e2843;
        var_qjd_dn0 = ((((var_csb_d_dn0 + var_cssw_d_dn0) + var_csswg_d_dn0) * var_v_di_b) + (assign3520_e2841 * var_v_di_b_dn0));
        var_qjd_dn3 = ((((var_csb_d_dn3 + var_cssw_d_dn3) + var_csswg_d_dn3) * var_v_di_b) + (assign3520_e2841 * var_v_di_b_dn3));
        var_qjd_rv = 0.0;

        let assign3530_e2846: f64 = if var_v_si_b > 0.0 { 1.0 } else { 0.0 };
        var_guard33 = assign3530_e2846;
        var_guard33_rv = 0.0;

        let (assign3540_e2863, assign3540_e2863_d_n2, assign3540_e2863_d_n3,) = {
    if (var_guard33 != 0.0) {
        let assign3540_e2850: f64 = (var_cj_t * var_as_i);
        let assign3540_e2852: f64 = (-p.p47);
        let assign3540_e2856: f64 = (var_v_si_b / var_pb_t);
        let assign3540_e2857: f64 = (1.0 + assign3540_e2856);
        let assign3540_e2858: f64 = (assign3540_e2857).ln();
        let assign3540_e2859: f64 = (assign3540_e2852 * assign3540_e2858);
        let assign3540_e2860: f64 = (assign3540_e2859).exp();
        let assign3540_e2861: f64 = (assign3540_e2850 * assign3540_e2860);
        (assign3540_e2861, (assign3540_e2850 * (assign3540_e2860 * (assign3540_e2852 * ((var_v_si_b_dn2 / var_pb_t) / assign3540_e2857)))), (assign3540_e2850 * (assign3540_e2860 * (assign3540_e2852 * ((var_v_si_b_dn3 / var_pb_t) / assign3540_e2857)))),)
    } else {
        (var_csb_s, var_csb_s_dn2, var_csb_s_dn3,)
    }
};
        var_csb_s = assign3540_e2863;
        var_csb_s_dn2 = assign3540_e2863_d_n2;
        var_csb_s_dn3 = assign3540_e2863_d_n3;
        var_csb_s_rv = 0.0;

        let (assign3550_e2880, assign3550_e2880_d_n2, assign3550_e2880_d_n3,) = {
    if (var_guard33 != 0.0) {
        let assign3550_e2867: f64 = (var_cjsw_t * var_ps_i);
        let assign3550_e2869: f64 = (-p.p48);
        let assign3550_e2873: f64 = (var_v_si_b / var_pbsw_t);
        let assign3550_e2874: f64 = (1.0 + assign3550_e2873);
        let assign3550_e2875: f64 = (assign3550_e2874).ln();
        let assign3550_e2876: f64 = (assign3550_e2869 * assign3550_e2875);
        let assign3550_e2877: f64 = (assign3550_e2876).exp();
        let assign3550_e2878: f64 = (assign3550_e2867 * assign3550_e2877);
        (assign3550_e2878, (assign3550_e2867 * (assign3550_e2877 * (assign3550_e2869 * ((var_v_si_b_dn2 / var_pbsw_t) / assign3550_e2874)))), (assign3550_e2867 * (assign3550_e2877 * (assign3550_e2869 * ((var_v_si_b_dn3 / var_pbsw_t) / assign3550_e2874)))),)
    } else {
        (var_cssw_s, var_cssw_s_dn2, var_cssw_s_dn3,)
    }
};
        var_cssw_s = assign3550_e2880;
        var_cssw_s_dn2 = assign3550_e2880_d_n2;
        var_cssw_s_dn3 = assign3550_e2880_d_n3;
        var_cssw_s_rv = 0.0;

        let (assign3560_e2897, assign3560_e2897_d_n2, assign3560_e2897_d_n3,) = {
    if (var_guard33 != 0.0) {
        let assign3560_e2884: f64 = (var_cjswg_t * var_weff);
        let assign3560_e2886: f64 = (-p.p49);
        let assign3560_e2890: f64 = (var_v_si_b / var_pbswg_t);
        let assign3560_e2891: f64 = (1.0 + assign3560_e2890);
        let assign3560_e2892: f64 = (assign3560_e2891).ln();
        let assign3560_e2893: f64 = (assign3560_e2886 * assign3560_e2892);
        let assign3560_e2894: f64 = (assign3560_e2893).exp();
        let assign3560_e2895: f64 = (assign3560_e2884 * assign3560_e2894);
        (assign3560_e2895, (assign3560_e2884 * (assign3560_e2894 * (assign3560_e2886 * ((var_v_si_b_dn2 / var_pbswg_t) / assign3560_e2891)))), (assign3560_e2884 * (assign3560_e2894 * (assign3560_e2886 * ((var_v_si_b_dn3 / var_pbswg_t) / assign3560_e2891)))),)
    } else {
        (var_csswg_s, var_csswg_s_dn2, var_csswg_s_dn3,)
    }
};
        var_csswg_s = assign3560_e2897;
        var_csswg_s_dn2 = assign3560_e2897_d_n2;
        var_csswg_s_dn3 = assign3560_e2897_d_n3;
        var_csswg_s_rv = 0.0;

        let (assign3570_e2912, assign3570_e2912_d_n2, assign3570_e2912_d_n3,) = {
    if (var_guard33 == 0.0) {
        let assign3570_e2902: f64 = (var_cj_t * var_as_i);
        let assign3570_e2906: f64 = (p.p47 * var_v_si_b);
        let assign3570_e2908: f64 = (assign3570_e2906 / var_pb_t);
        let assign3570_e2909: f64 = (1.0 - assign3570_e2908);
        let assign3570_e2910: f64 = (assign3570_e2902 * assign3570_e2909);
        (assign3570_e2910, (assign3570_e2902 * (-((p.p47 * var_v_si_b_dn2) / var_pb_t))), (assign3570_e2902 * (-((p.p47 * var_v_si_b_dn3) / var_pb_t))),)
    } else {
        (var_csb_s, var_csb_s_dn2, var_csb_s_dn3,)
    }
};
        var_csb_s = assign3570_e2912;
        var_csb_s_dn2 = assign3570_e2912_d_n2;
        var_csb_s_dn3 = assign3570_e2912_d_n3;
        var_csb_s_rv = 0.0;

        let (assign3580_e2927, assign3580_e2927_d_n2, assign3580_e2927_d_n3,) = {
    if (var_guard33 == 0.0) {
        let assign3580_e2917: f64 = (var_cjsw_t * var_ps_i);
        let assign3580_e2921: f64 = (p.p48 * var_v_si_b);
        let assign3580_e2923: f64 = (assign3580_e2921 / var_pbsw_t);
        let assign3580_e2924: f64 = (1.0 - assign3580_e2923);
        let assign3580_e2925: f64 = (assign3580_e2917 * assign3580_e2924);
        (assign3580_e2925, (assign3580_e2917 * (-((p.p48 * var_v_si_b_dn2) / var_pbsw_t))), (assign3580_e2917 * (-((p.p48 * var_v_si_b_dn3) / var_pbsw_t))),)
    } else {
        (var_cssw_s, var_cssw_s_dn2, var_cssw_s_dn3,)
    }
};
        var_cssw_s = assign3580_e2927;
        var_cssw_s_dn2 = assign3580_e2927_d_n2;
        var_cssw_s_dn3 = assign3580_e2927_d_n3;
        var_cssw_s_rv = 0.0;

        let (assign3590_e2942, assign3590_e2942_d_n2, assign3590_e2942_d_n3,) = {
    if (var_guard33 == 0.0) {
        let assign3590_e2932: f64 = (var_cjswg_t * var_weff);
        let assign3590_e2936: f64 = (p.p49 * var_v_si_b);
        let assign3590_e2938: f64 = (assign3590_e2936 / var_pbswg_t);
        let assign3590_e2939: f64 = (1.0 - assign3590_e2938);
        let assign3590_e2940: f64 = (assign3590_e2932 * assign3590_e2939);
        (assign3590_e2940, (assign3590_e2932 * (-((p.p49 * var_v_si_b_dn2) / var_pbswg_t))), (assign3590_e2932 * (-((p.p49 * var_v_si_b_dn3) / var_pbswg_t))),)
    } else {
        (var_csswg_s, var_csswg_s_dn2, var_csswg_s_dn3,)
    }
};
        var_csswg_s = assign3590_e2942;
        var_csswg_s_dn2 = assign3590_e2942_d_n2;
        var_csswg_s_dn3 = assign3590_e2942_d_n3;
        var_csswg_s_rv = 0.0;

        let assign3600_e2945: f64 = (var_csb_s + var_cssw_s);
        let assign3600_e2947: f64 = (assign3600_e2945 + var_csswg_s);
        let assign3600_e2949: f64 = (assign3600_e2947 * var_v_si_b);
        var_qjs = assign3600_e2949;
        var_qjs_dn2 = ((((var_csb_s_dn2 + var_cssw_s_dn2) + var_csswg_s_dn2) * var_v_si_b) + (assign3600_e2947 * var_v_si_b_dn2));
        var_qjs_dn3 = ((((var_csb_s_dn3 + var_cssw_s_dn3) + var_csswg_s_dn3) * var_v_si_b) + (assign3600_e2947 * var_v_si_b_dn3));
        var_qjs_rv = 0.0;

        *var_ad_i_slot = var_ad_i;
        *var_ad_i_rv_slot = var_ad_i_rv;
        *var_as_i_slot = var_as_i;
        *var_as_i_rv_slot = var_as_i_rv;
        *var_cj_t_slot = var_cj_t;
        *var_cj_t_rv_slot = var_cj_t_rv;
        *var_cjsw_t_slot = var_cjsw_t;
        *var_cjsw_t_rv_slot = var_cjsw_t_rv;
        *var_cjswg_t_slot = var_cjswg_t;
        *var_cjswg_t_rv_slot = var_cjswg_t_rv;
        *var_csb_d_slot = var_csb_d;
        *var_csb_d_dn0_slot = var_csb_d_dn0;
        *var_csb_d_dn3_slot = var_csb_d_dn3;
        *var_csb_d_rv_slot = var_csb_d_rv;
        *var_csb_s_slot = var_csb_s;
        *var_csb_s_dn2_slot = var_csb_s_dn2;
        *var_csb_s_dn3_slot = var_csb_s_dn3;
        *var_csb_s_rv_slot = var_csb_s_rv;
        *var_cssw_d_slot = var_cssw_d;
        *var_cssw_d_dn0_slot = var_cssw_d_dn0;
        *var_cssw_d_dn3_slot = var_cssw_d_dn3;
        *var_cssw_d_rv_slot = var_cssw_d_rv;
        *var_cssw_s_slot = var_cssw_s;
        *var_cssw_s_dn2_slot = var_cssw_s_dn2;
        *var_cssw_s_dn3_slot = var_cssw_s_dn3;
        *var_cssw_s_rv_slot = var_cssw_s_rv;
        *var_csswg_d_slot = var_csswg_d;
        *var_csswg_d_dn0_slot = var_csswg_d_dn0;
        *var_csswg_d_dn3_slot = var_csswg_d_dn3;
        *var_csswg_d_rv_slot = var_csswg_d_rv;
        *var_csswg_s_slot = var_csswg_s;
        *var_csswg_s_dn2_slot = var_csswg_s_dn2;
        *var_csswg_s_dn3_slot = var_csswg_s_dn3;
        *var_csswg_s_rv_slot = var_csswg_s_rv;
        *var_ddt_qd_slot = var_ddt_qd;
        *var_ddt_qd_dn0_slot = var_ddt_qd_dn0;
        *var_ddt_qd_dn1_slot = var_ddt_qd_dn1;
        *var_ddt_qd_dn2_slot = var_ddt_qd_dn2;
        *var_ddt_qd_dn3_slot = var_ddt_qd_dn3;
        *var_ddt_qd_rdn0_slot = var_ddt_qd_rdn0;
        *var_ddt_qd_rdn1_slot = var_ddt_qd_rdn1;
        *var_ddt_qd_rdn2_slot = var_ddt_qd_rdn2;
        *var_ddt_qd_rdn3_slot = var_ddt_qd_rdn3;
        *var_ddt_qd_rv_slot = var_ddt_qd_rv;
        *var_ddt_qs_slot = var_ddt_qs;
        *var_ddt_qs_dn0_slot = var_ddt_qs_dn0;
        *var_ddt_qs_dn1_slot = var_ddt_qs_dn1;
        *var_ddt_qs_dn2_slot = var_ddt_qs_dn2;
        *var_ddt_qs_dn3_slot = var_ddt_qs_dn3;
        *var_ddt_qs_rdn0_slot = var_ddt_qs_rdn0;
        *var_ddt_qs_rdn1_slot = var_ddt_qs_rdn1;
        *var_ddt_qs_rdn2_slot = var_ddt_qs_rdn2;
        *var_ddt_qs_rdn3_slot = var_ddt_qs_rdn3;
        *var_ddt_qs_rv_slot = var_ddt_qs_rv;
        *var_guard21_slot = var_guard21;
        *var_guard21_rv_slot = var_guard21_rv;
        *var_guard24_slot = var_guard24;
        *var_guard24_rv_slot = var_guard24_rv;
        *var_guard25_slot = var_guard25;
        *var_guard25_rv_slot = var_guard25_rv;
        *var_guard26_slot = var_guard26;
        *var_guard26_rv_slot = var_guard26_rv;
        *var_guard27_slot = var_guard27;
        *var_guard27_rv_slot = var_guard27_rv;
        *var_guard32_slot = var_guard32;
        *var_guard32_rv_slot = var_guard32_rv;
        *var_guard33_slot = var_guard33;
        *var_guard33_rv_slot = var_guard33_rv;
        *var_pb_t_slot = var_pb_t;
        *var_pb_t_rv_slot = var_pb_t_rv;
        *var_pbsw_t_slot = var_pbsw_t;
        *var_pbsw_t_rv_slot = var_pbsw_t_rv;
        *var_pbswg_t_slot = var_pbswg_t;
        *var_pbswg_t_rv_slot = var_pbswg_t_rv;
        *var_pd_i_slot = var_pd_i;
        *var_pd_i_rv_slot = var_pd_i_rv;
        *var_ps_i_slot = var_ps_i;
        *var_ps_i_rv_slot = var_ps_i_rv;
        *var_qb_1_slot = var_qb_1;
        *var_qb_1_dn0_slot = var_qb_1_dn0;
        *var_qb_1_dn1_slot = var_qb_1_dn1;
        *var_qb_1_dn2_slot = var_qb_1_dn2;
        *var_qb_1_dn3_slot = var_qb_1_dn3;
        *var_qb_1_rv_slot = var_qb_1_rv;
        *var_qg_slot = var_qg;
        *var_qg_dn0_slot = var_qg_dn0;
        *var_qg_dn1_slot = var_qg_dn1;
        *var_qg_dn2_slot = var_qg_dn2;
        *var_qg_dn3_slot = var_qg_dn3;
        *var_qg_rv_slot = var_qg_rv;
        *var_qjd_slot = var_qjd;
        *var_qjd_dn0_slot = var_qjd_dn0;
        *var_qjd_dn3_slot = var_qjd_dn3;
        *var_qjd_rv_slot = var_qjd_rv;
        *var_qjs_slot = var_qjs;
        *var_qjs_dn2_slot = var_qjs_dn2;
        *var_qjs_dn3_slot = var_qjs_dn3;
        *var_qjs_rv_slot = var_qjs_rv;
        *var_v_di_b_slot = var_v_di_b;
        *var_v_di_b_dn0_slot = var_v_di_b_dn0;
        *var_v_di_b_dn3_slot = var_v_di_b_dn3;
        *var_v_di_b_rv_slot = var_v_di_b_rv;
        *var_v_si_b_slot = var_v_si_b;
        *var_v_si_b_dn2_slot = var_v_si_b_dn2;
        *var_v_si_b_dn3_slot = var_v_si_b_dn3;
        *var_v_si_b_rv_slot = var_v_si_b_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_ddt_qd: f64,
        var_ddt_qd_dn0: f64,
        var_ddt_qd_dn1: f64,
        var_ddt_qd_dn2: f64,
        var_ddt_qd_dn3: f64,
        var_ddt_qs: f64,
        var_ddt_qs_dn0: f64,
        var_ddt_qs_dn1: f64,
        var_ddt_qs_dn2: f64,
        var_ddt_qs_dn3: f64,
        var_guard21: f64,
        var_qg: f64,
        var_qg_dn0: f64,
        var_qg_dn1: f64,
        var_qg_dn2: f64,
        var_qg_dn3: f64,
        var_qjd: f64,
        var_qjd_dn0: f64,
        var_qjd_dn3: f64,
        var_qjs: f64,
        var_qjs_dn2: f64,
        var_qjs_dn3: f64,
    ) {
        let (eq1_e92, eq1_e92_d_n0, eq1_e92_d_n1, eq1_e92_d_n2, eq1_e92_d_n3,) = {
    if (var_guard21 != 0.0) {
        let eq1_e90: f64 = (p.p0 * var_ddt_qd);
        let eq1_e90_d_n0: f64 = (p.p0 * var_ddt_qd_dn0);
        let eq1_e90_d_n1: f64 = (p.p0 * var_ddt_qd_dn1);
        let eq1_e90_d_n2: f64 = (p.p0 * var_ddt_qd_dn2);
        let eq1_e90_d_n3: f64 = (p.p0 * var_ddt_qd_dn3);
        (eq1_e90, eq1_e90_d_n0, eq1_e90_d_n1, eq1_e90_d_n2, eq1_e90_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e92;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (eq1_value),
            [0, 1, 2, 3],
            [multiplicity * (eq1_e92_d_n0), multiplicity * (eq1_e92_d_n1), multiplicity * (eq1_e92_d_n2), multiplicity * (eq1_e92_d_n3)],
            [],
            [],
            1.0,
        );
        let (eq2_e98, eq2_e98_d_n0, eq2_e98_d_n1, eq2_e98_d_n2, eq2_e98_d_n3,) = {
    if (var_guard21 != 0.0) {
        let eq2_e96: f64 = (p.p0 * var_ddt_qs);
        let eq2_e96_d_n0: f64 = (p.p0 * var_ddt_qs_dn0);
        let eq2_e96_d_n1: f64 = (p.p0 * var_ddt_qs_dn1);
        let eq2_e96_d_n2: f64 = (p.p0 * var_ddt_qs_dn2);
        let eq2_e96_d_n3: f64 = (p.p0 * var_ddt_qs_dn3);
        (eq2_e96, eq2_e96_d_n0, eq2_e96_d_n1, eq2_e96_d_n2, eq2_e96_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e98;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(2),
            Some(3),
            multiplicity * (eq2_value),
            [0, 1, 2, 3],
            [multiplicity * (eq2_e98_d_n0), multiplicity * (eq2_e98_d_n1), multiplicity * (eq2_e98_d_n2), multiplicity * (eq2_e98_d_n3)],
            [],
            [],
            1.0,
        );
        let (eq4_e111, eq4_e111_d_n0, eq4_e111_d_n1, eq4_e111_d_n2, eq4_e111_d_n3,) = {
    if (var_guard21 == 0.0) {
        let eq4_e109: f64 = (p.p0 * var_ddt_qd);
        let eq4_e109_d_n0: f64 = (p.p0 * var_ddt_qd_dn0);
        let eq4_e109_d_n1: f64 = (p.p0 * var_ddt_qd_dn1);
        let eq4_e109_d_n2: f64 = (p.p0 * var_ddt_qd_dn2);
        let eq4_e109_d_n3: f64 = (p.p0 * var_ddt_qd_dn3);
        (eq4_e109, eq4_e109_d_n0, eq4_e109_d_n1, eq4_e109_d_n2, eq4_e109_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e111;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(2),
            Some(3),
            multiplicity * (eq4_value),
            [0, 1, 2, 3],
            [multiplicity * (eq4_e111_d_n0), multiplicity * (eq4_e111_d_n1), multiplicity * (eq4_e111_d_n2), multiplicity * (eq4_e111_d_n3)],
            [],
            [],
            1.0,
        );
        let (eq5_e118, eq5_e118_d_n0, eq5_e118_d_n1, eq5_e118_d_n2, eq5_e118_d_n3,) = {
    if (var_guard21 == 0.0) {
        let eq5_e116: f64 = (p.p0 * var_ddt_qs);
        let eq5_e116_d_n0: f64 = (p.p0 * var_ddt_qs_dn0);
        let eq5_e116_d_n1: f64 = (p.p0 * var_ddt_qs_dn1);
        let eq5_e116_d_n2: f64 = (p.p0 * var_ddt_qs_dn2);
        let eq5_e116_d_n3: f64 = (p.p0 * var_ddt_qs_dn3);
        (eq5_e116, eq5_e116_d_n0, eq5_e116_d_n1, eq5_e116_d_n2, eq5_e116_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e118;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (eq5_value),
            [0, 1, 2, 3],
            [multiplicity * (eq5_e118_d_n0), multiplicity * (eq5_e118_d_n1), multiplicity * (eq5_e118_d_n2), multiplicity * (eq5_e118_d_n3)],
            [],
            [],
            1.0,
        );
        let eq7_e128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qg);
        let eq7_e129: f64 = (p.p0 * eq7_e128);
        let eq7_e129_d_n0: f64 = (p.p0 * (var_qg_dn0 * ddt_scale));
        let eq7_e129_d_n1: f64 = (p.p0 * (var_qg_dn1 * ddt_scale));
        let eq7_e129_d_n2: f64 = (p.p0 * (var_qg_dn2 * ddt_scale));
        let eq7_e129_d_n3: f64 = (p.p0 * (var_qg_dn3 * ddt_scale));
        let eq7_value: f64 = eq7_e129;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(3),
            multiplicity * (eq7_value),
            [0, 1, 2, 3],
            [multiplicity * (eq7_e129_d_n0), multiplicity * (eq7_e129_d_n1), multiplicity * (eq7_e129_d_n2), multiplicity * (eq7_e129_d_n3)],
            [],
            [],
            1.0,
        );
        let eq11_e178: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_qjd);
        let eq11_e180: f64 = (eq11_e178 * p.p0);
        let eq11_e180_d_n0: f64 = ((var_qjd_dn0 * ddt_scale) * p.p0);
        let eq11_e180_d_n3: f64 = ((var_qjd_dn3 * ddt_scale) * p.p0);
        let eq11_e182: f64 = (eq11_e180 * p.p7);
        let eq11_e182_d_n0: f64 = (eq11_e180_d_n0 * p.p7);
        let eq11_e182_d_n3: f64 = (eq11_e180_d_n3 * p.p7);
        let eq11_value: f64 = eq11_e182;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * (eq11_value),
            0,
            multiplicity * (eq11_e182_d_n0),
            3,
            multiplicity * (eq11_e182_d_n3),
        );
        let eq12_e184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, var_qjs);
        let eq12_e186: f64 = (eq12_e184 * p.p0);
        let eq12_e186_d_n2: f64 = ((var_qjs_dn2 * ddt_scale) * p.p0);
        let eq12_e186_d_n3: f64 = ((var_qjs_dn3 * ddt_scale) * p.p0);
        let eq12_e188: f64 = (eq12_e186 * p.p7);
        let eq12_e188_d_n2: f64 = (eq12_e186_d_n2 * p.p7);
        let eq12_e188_d_n3: f64 = (eq12_e186_d_n3 * p.p7);
        let eq12_value: f64 = eq12_e188;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * (eq12_value),
            2,
            multiplicity * (eq12_e188_d_n2),
            3,
            multiplicity * (eq12_e188_d_n3),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_ddt_qd: f64,
        var_ddt_qd_dn0: f64,
        var_ddt_qd_dn1: f64,
        var_ddt_qd_dn2: f64,
        var_ddt_qd_dn3: f64,
        var_ddt_qd_rdn0: f64,
        var_ddt_qd_rdn1: f64,
        var_ddt_qd_rdn2: f64,
        var_ddt_qd_rdn3: f64,
        var_ddt_qd_rv: f64,
        var_ddt_qs: f64,
        var_ddt_qs_dn0: f64,
        var_ddt_qs_dn1: f64,
        var_ddt_qs_dn2: f64,
        var_ddt_qs_dn3: f64,
        var_ddt_qs_rdn0: f64,
        var_ddt_qs_rdn1: f64,
        var_ddt_qs_rdn2: f64,
        var_ddt_qs_rdn3: f64,
        var_ddt_qs_rv: f64,
        var_guard21: f64,
        var_qg: f64,
        var_qg_dn0: f64,
        var_qg_dn1: f64,
        var_qg_dn2: f64,
        var_qg_dn3: f64,
        var_qjd: f64,
        var_qjd_dn0: f64,
        var_qjd_dn3: f64,
        var_qjs: f64,
        var_qjs_dn2: f64,
        var_qjs_dn3: f64,
    ) {
        let (eq1_e92, eq1_e92_d_n0, eq1_e92_d_n1, eq1_e92_d_n2, eq1_e92_d_n3, eq1_e92_q, eq1_e92_q_d_n0, eq1_e92_q_d_n1, eq1_e92_q_d_n2, eq1_e92_q_d_n3,) = {
    if (var_guard21 != 0.0) {
        let eq1_e89_q: f64 = var_ddt_qd_rv;
        let eq1_e90: f64 = (p.p0 * var_ddt_qd);
        let eq1_e90_d_n0: f64 = (p.p0 * var_ddt_qd_dn0);
        let eq1_e90_d_n1: f64 = (p.p0 * var_ddt_qd_dn1);
        let eq1_e90_d_n2: f64 = (p.p0 * var_ddt_qd_dn2);
        let eq1_e90_d_n3: f64 = (p.p0 * var_ddt_qd_dn3);
        let eq1_e90_q: f64 = (p.p0 * eq1_e89_q);
        let eq1_e90_q_d_n0: f64 = (p.p0 * var_ddt_qd_rdn0);
        let eq1_e90_q_d_n1: f64 = (p.p0 * var_ddt_qd_rdn1);
        let eq1_e90_q_d_n2: f64 = (p.p0 * var_ddt_qd_rdn2);
        let eq1_e90_q_d_n3: f64 = (p.p0 * var_ddt_qd_rdn3);
        (eq1_e90, eq1_e90_d_n0, eq1_e90_d_n1, eq1_e90_d_n2, eq1_e90_d_n3, eq1_e90_q, eq1_e90_q_d_n0, eq1_e90_q_d_n1, eq1_e90_q_d_n2, eq1_e90_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq1_e92_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq1_e92_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq1_e92_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq1_e92_q_d_n3)),
            ],
        );
        let (eq2_e98, eq2_e98_d_n0, eq2_e98_d_n1, eq2_e98_d_n2, eq2_e98_d_n3, eq2_e98_q, eq2_e98_q_d_n0, eq2_e98_q_d_n1, eq2_e98_q_d_n2, eq2_e98_q_d_n3,) = {
    if (var_guard21 != 0.0) {
        let eq2_e95_q: f64 = var_ddt_qs_rv;
        let eq2_e96: f64 = (p.p0 * var_ddt_qs);
        let eq2_e96_d_n0: f64 = (p.p0 * var_ddt_qs_dn0);
        let eq2_e96_d_n1: f64 = (p.p0 * var_ddt_qs_dn1);
        let eq2_e96_d_n2: f64 = (p.p0 * var_ddt_qs_dn2);
        let eq2_e96_d_n3: f64 = (p.p0 * var_ddt_qs_dn3);
        let eq2_e96_q: f64 = (p.p0 * eq2_e95_q);
        let eq2_e96_q_d_n0: f64 = (p.p0 * var_ddt_qs_rdn0);
        let eq2_e96_q_d_n1: f64 = (p.p0 * var_ddt_qs_rdn1);
        let eq2_e96_q_d_n2: f64 = (p.p0 * var_ddt_qs_rdn2);
        let eq2_e96_q_d_n3: f64 = (p.p0 * var_ddt_qs_rdn3);
        (eq2_e96, eq2_e96_d_n0, eq2_e96_d_n1, eq2_e96_d_n2, eq2_e96_d_n3, eq2_e96_q, eq2_e96_q_d_n0, eq2_e96_q_d_n1, eq2_e96_q_d_n2, eq2_e96_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq2_e98_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq2_e98_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq2_e98_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq2_e98_q_d_n3)),
            ],
        );
        let (eq4_e111, eq4_e111_d_n0, eq4_e111_d_n1, eq4_e111_d_n2, eq4_e111_d_n3, eq4_e111_q, eq4_e111_q_d_n0, eq4_e111_q_d_n1, eq4_e111_q_d_n2, eq4_e111_q_d_n3,) = {
    if (var_guard21 == 0.0) {
        let eq4_e108_q: f64 = var_ddt_qd_rv;
        let eq4_e109: f64 = (p.p0 * var_ddt_qd);
        let eq4_e109_d_n0: f64 = (p.p0 * var_ddt_qd_dn0);
        let eq4_e109_d_n1: f64 = (p.p0 * var_ddt_qd_dn1);
        let eq4_e109_d_n2: f64 = (p.p0 * var_ddt_qd_dn2);
        let eq4_e109_d_n3: f64 = (p.p0 * var_ddt_qd_dn3);
        let eq4_e109_q: f64 = (p.p0 * eq4_e108_q);
        let eq4_e109_q_d_n0: f64 = (p.p0 * var_ddt_qd_rdn0);
        let eq4_e109_q_d_n1: f64 = (p.p0 * var_ddt_qd_rdn1);
        let eq4_e109_q_d_n2: f64 = (p.p0 * var_ddt_qd_rdn2);
        let eq4_e109_q_d_n3: f64 = (p.p0 * var_ddt_qd_rdn3);
        (eq4_e109, eq4_e109_d_n0, eq4_e109_d_n1, eq4_e109_d_n2, eq4_e109_d_n3, eq4_e109_q, eq4_e109_q_d_n0, eq4_e109_q_d_n1, eq4_e109_q_d_n2, eq4_e109_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq4_e111_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq4_e111_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq4_e111_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq4_e111_q_d_n3)),
            ],
        );
        let (eq5_e118, eq5_e118_d_n0, eq5_e118_d_n1, eq5_e118_d_n2, eq5_e118_d_n3, eq5_e118_q, eq5_e118_q_d_n0, eq5_e118_q_d_n1, eq5_e118_q_d_n2, eq5_e118_q_d_n3,) = {
    if (var_guard21 == 0.0) {
        let eq5_e115_q: f64 = var_ddt_qs_rv;
        let eq5_e116: f64 = (p.p0 * var_ddt_qs);
        let eq5_e116_d_n0: f64 = (p.p0 * var_ddt_qs_dn0);
        let eq5_e116_d_n1: f64 = (p.p0 * var_ddt_qs_dn1);
        let eq5_e116_d_n2: f64 = (p.p0 * var_ddt_qs_dn2);
        let eq5_e116_d_n3: f64 = (p.p0 * var_ddt_qs_dn3);
        let eq5_e116_q: f64 = (p.p0 * eq5_e115_q);
        let eq5_e116_q_d_n0: f64 = (p.p0 * var_ddt_qs_rdn0);
        let eq5_e116_q_d_n1: f64 = (p.p0 * var_ddt_qs_rdn1);
        let eq5_e116_q_d_n2: f64 = (p.p0 * var_ddt_qs_rdn2);
        let eq5_e116_q_d_n3: f64 = (p.p0 * var_ddt_qs_rdn3);
        (eq5_e116, eq5_e116_d_n0, eq5_e116_d_n1, eq5_e116_d_n2, eq5_e116_d_n3, eq5_e116_q, eq5_e116_q_d_n0, eq5_e116_q_d_n1, eq5_e116_q_d_n2, eq5_e116_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq5_e118_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq5_e118_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq5_e118_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq5_e118_q_d_n3)),
            ],
        );
        let eq7_e128_q: f64 = var_qg;
        let eq7_e129: f64 = (p.p0 * var_qg);
        let eq7_e129_d_n0: f64 = (p.p0 * var_qg_dn0);
        let eq7_e129_d_n1: f64 = (p.p0 * var_qg_dn1);
        let eq7_e129_d_n2: f64 = (p.p0 * var_qg_dn2);
        let eq7_e129_d_n3: f64 = (p.p0 * var_qg_dn3);
        let eq7_e129_q: f64 = (p.p0 * eq7_e128_q);
        stamper.stamp_current_reactive(
            Some(nodes[1]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq7_e129_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq7_e129_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq7_e129_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq7_e129_d_n3)),
            ],
        );
        let eq11_e178_q: f64 = var_qjd;
        let eq11_e180: f64 = (var_qjd * p.p0);
        let eq11_e180_d_n0: f64 = (var_qjd_dn0 * p.p0);
        let eq11_e180_d_n3: f64 = (var_qjd_dn3 * p.p0);
        let eq11_e180_q: f64 = (eq11_e178_q * p.p0);
        let eq11_e182: f64 = (eq11_e180 * p.p7);
        let eq11_e182_d_n0: f64 = (eq11_e180_d_n0 * p.p7);
        let eq11_e182_d_n3: f64 = (eq11_e180_d_n3 * p.p7);
        let eq11_e182_q: f64 = (eq11_e180_q * p.p7);
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[3]),
            nodes[0],
            multiplicity * (eq11_e182_d_n0),
            nodes[3],
            multiplicity * (eq11_e182_d_n3),
        );
        let eq12_e184_q: f64 = var_qjs;
        let eq12_e186: f64 = (var_qjs * p.p0);
        let eq12_e186_d_n2: f64 = (var_qjs_dn2 * p.p0);
        let eq12_e186_d_n3: f64 = (var_qjs_dn3 * p.p0);
        let eq12_e186_q: f64 = (eq12_e184_q * p.p0);
        let eq12_e188: f64 = (eq12_e186 * p.p7);
        let eq12_e188_d_n2: f64 = (eq12_e186_d_n2 * p.p7);
        let eq12_e188_d_n3: f64 = (eq12_e186_d_n3 * p.p7);
        let eq12_e188_q: f64 = (eq12_e186_q * p.p7);
        stamper.stamp_current_reactive_node2(
            Some(nodes[2]),
            Some(nodes[3]),
            nodes[2],
            multiplicity * (eq12_e188_d_n2),
            nodes[3],
            multiplicity * (eq12_e188_d_n3),
        );
    }
}
