#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
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
        var_ci: f64,
        var_ci_dn0: f64,
        var_ci_dn10: f64,
        var_ci_dn11: f64,
        var_ci_dn12: f64,
        var_ci_dn2: f64,
        var_ci_dn4: f64,
        var_ci_dn5: f64,
        var_ci_dn6: f64,
        var_ci_dn8: f64,
        var_cthe: f64,
        var_cthe_dn0: f64,
        var_cthe_dn10: f64,
        var_cthe_dn11: f64,
        var_cthe_dn12: f64,
        var_cthe_dn2: f64,
        var_cthe_dn4: f64,
        var_cthe_dn5: f64,
        var_cthe_dn6: f64,
        var_cthe_dn8: f64,
        var_gth: f64,
        var_gth_dn0: f64,
        var_gth_dn10: f64,
        var_gth_dn11: f64,
        var_gth_dn12: f64,
        var_gth_dn2: f64,
        var_gth_dn4: f64,
        var_gth_dn5: f64,
        var_gth_dn6: f64,
        var_gth_dn8: f64,
        var_guard443: f64,
        var_guard444: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn12: f64,
        var_ids_dn2: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn8: f64,
        var_igb: f64,
        var_igb_dn0: f64,
        var_igb_dn10: f64,
        var_igb_dn11: f64,
        var_igb_dn12: f64,
        var_igb_dn2: f64,
        var_igb_dn4: f64,
        var_igb_dn5: f64,
        var_igb_dn6: f64,
        var_igb_dn8: f64,
        var_igd: f64,
        var_igd_dn0: f64,
        var_igd_dn10: f64,
        var_igd_dn11: f64,
        var_igd_dn12: f64,
        var_igd_dn2: f64,
        var_igd_dn4: f64,
        var_igd_dn5: f64,
        var_igd_dn6: f64,
        var_igd_dn8: f64,
        var_igidl: f64,
        var_igidl_dn0: f64,
        var_igidl_dn10: f64,
        var_igidl_dn11: f64,
        var_igidl_dn12: f64,
        var_igidl_dn2: f64,
        var_igidl_dn4: f64,
        var_igidl_dn5: f64,
        var_igidl_dn6: f64,
        var_igidl_dn8: f64,
        var_igisl: f64,
        var_igisl_dn0: f64,
        var_igisl_dn10: f64,
        var_igisl_dn11: f64,
        var_igisl_dn12: f64,
        var_igisl_dn2: f64,
        var_igisl_dn4: f64,
        var_igisl_dn5: f64,
        var_igisl_dn6: f64,
        var_igisl_dn8: f64,
        var_igs: f64,
        var_igs_dn0: f64,
        var_igs_dn10: f64,
        var_igs_dn11: f64,
        var_igs_dn12: f64,
        var_igs_dn2: f64,
        var_igs_dn4: f64,
        var_igs_dn5: f64,
        var_igs_dn6: f64,
        var_igs_dn8: f64,
        var_iqb_nqs: f64,
        var_iqb_nqs_dn0: f64,
        var_iqb_nqs_dn10: f64,
        var_iqb_nqs_dn11: f64,
        var_iqb_nqs_dn12: f64,
        var_iqb_nqs_dn2: f64,
        var_iqb_nqs_dn4: f64,
        var_iqb_nqs_dn5: f64,
        var_iqb_nqs_dn6: f64,
        var_iqb_nqs_dn8: f64,
        var_iqb_nqs_dn9: f64,
        var_iqh_nqs: f64,
        var_iqh_nqs_dn0: f64,
        var_iqh_nqs_dn10: f64,
        var_iqh_nqs_dn11: f64,
        var_iqh_nqs_dn12: f64,
        var_iqh_nqs_dn2: f64,
        var_iqh_nqs_dn4: f64,
        var_iqh_nqs_dn5: f64,
        var_iqh_nqs_dn6: f64,
        var_iqh_nqs_dn8: f64,
        var_iqi_nqs: f64,
        var_iqi_nqs_dn0: f64,
        var_iqi_nqs_dn10: f64,
        var_iqi_nqs_dn11: f64,
        var_iqi_nqs_dn12: f64,
        var_iqi_nqs_dn2: f64,
        var_iqi_nqs_dn4: f64,
        var_iqi_nqs_dn5: f64,
        var_iqi_nqs_dn6: f64,
        var_iqi_nqs_dn8: f64,
        var_isub: f64,
        var_isub_dn0: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn2: f64,
        var_isub_dn4: f64,
        var_isub_dn5: f64,
        var_isub_dn6: f64,
        var_isub_dn8: f64,
        var_isubs: f64,
        var_isubs_dn0: f64,
        var_isubs_dn10: f64,
        var_isubs_dn11: f64,
        var_isubs_dn12: f64,
        var_isubs_dn2: f64,
        var_isubs_dn4: f64,
        var_isubs_dn5: f64,
        var_isubs_dn6: f64,
        var_isubs_dn8: f64,
        var_qb: f64,
        var_qb_dn0: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn12: f64,
        var_qb_dn2: f64,
        var_qb_dn4: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn8: f64,
        var_qb_nqs: f64,
        var_qb_nqs_dn9: f64,
        var_qd: f64,
        var_qd_dn0: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn12: f64,
        var_qd_dn2: f64,
        var_qd_dn4: f64,
        var_qd_dn5: f64,
        var_qd_dn6: f64,
        var_qd_dn8: f64,
        var_qd_nqs: f64,
        var_qd_nqs_dn0: f64,
        var_qd_nqs_dn10: f64,
        var_qd_nqs_dn11: f64,
        var_qd_nqs_dn12: f64,
        var_qd_nqs_dn2: f64,
        var_qd_nqs_dn4: f64,
        var_qd_nqs_dn5: f64,
        var_qd_nqs_dn6: f64,
        var_qd_nqs_dn8: f64,
        var_qg: f64,
        var_qg_dn0: f64,
        var_qg_dn10: f64,
        var_qg_dn11: f64,
        var_qg_dn12: f64,
        var_qg_dn2: f64,
        var_qg_dn4: f64,
        var_qg_dn5: f64,
        var_qg_dn6: f64,
        var_qg_dn8: f64,
        var_qg_nqs: f64,
        var_qg_nqs_dn8: f64,
        var_qg_nqs_dn9: f64,
        var_rdd: f64,
        var_rdd_dn0: f64,
        var_rdd_dn10: f64,
        var_rdd_dn11: f64,
        var_rdd_dn12: f64,
        var_rdd_dn2: f64,
        var_rdd_dn4: f64,
        var_rdd_dn5: f64,
        var_rdd_dn6: f64,
        var_rdd_dn8: f64,
        var_rpower: f64,
        var_rpower_dn0: f64,
        var_rpower_dn10: f64,
        var_rpower_dn11: f64,
        var_rpower_dn12: f64,
        var_rpower_dn2: f64,
        var_rpower_dn4: f64,
        var_rpower_dn5: f64,
        var_rpower_dn6: f64,
        var_rpower_dn8: f64,
        var_rsd: f64,
        var_rsd_dn0: f64,
        var_rsd_dn10: f64,
        var_rsd_dn11: f64,
        var_rsd_dn12: f64,
        var_rsd_dn2: f64,
        var_rsd_dn4: f64,
        var_rsd_dn5: f64,
        var_rsd_dn6: f64,
        var_rsd_dn8: f64,
        var_sigrat_d: f64,
        var_sigrat_d_dn0: f64,
        var_sigrat_d_dn10: f64,
        var_sigrat_d_dn11: f64,
        var_sigrat_d_dn12: f64,
        var_sigrat_d_dn2: f64,
        var_sigrat_d_dn4: f64,
        var_sigrat_d_dn5: f64,
        var_sigrat_d_dn6: f64,
        var_sigrat_d_dn8: f64,
        var_sigrat_s: f64,
        var_sigrat_s_dn0: f64,
        var_sigrat_s_dn10: f64,
        var_sigrat_s_dn11: f64,
        var_sigrat_s_dn12: f64,
        var_sigrat_s_dn2: f64,
        var_sigrat_s_dn4: f64,
        var_sigrat_s_dn5: f64,
        var_sigrat_s_dn6: f64,
        var_sigrat_s_dn8: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq0_e342: f64 = (p.p33 * var_ids);
        let eq0_e342_d_n0: f64 = (p.p33 * var_ids_dn0);
        let eq0_e342_d_n2: f64 = (p.p33 * var_ids_dn2);
        let eq0_e342_d_n4: f64 = (p.p33 * var_ids_dn4);
        let eq0_e342_d_n5: f64 = (p.p33 * var_ids_dn5);
        let eq0_e342_d_n6: f64 = (p.p33 * var_ids_dn6);
        let eq0_e342_d_n8: f64 = (p.p33 * var_ids_dn8);
        let eq0_e342_d_n10: f64 = (p.p33 * var_ids_dn10);
        let eq0_e342_d_n11: f64 = (p.p33 * var_ids_dn11);
        let eq0_e342_d_n12: f64 = (p.p33 * var_ids_dn12);
        let eq0_value: f64 = eq0_e342;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq0_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq0_e342_d_n0), multiplicity * (eq0_e342_d_n2), multiplicity * (eq0_e342_d_n4), multiplicity * (eq0_e342_d_n5), multiplicity * (eq0_e342_d_n6), multiplicity * (eq0_e342_d_n8), multiplicity * (eq0_e342_d_n10), multiplicity * (eq0_e342_d_n11), multiplicity * (eq0_e342_d_n12)],
            [],
            [],
            1.0,
        );
        let eq1_e346: f64 = (var_igidl + var_isub);
        let eq1_e346_d_n0: f64 = (var_igidl_dn0 + var_isub_dn0);
        let eq1_e346_d_n2: f64 = (var_igidl_dn2 + var_isub_dn2);
        let eq1_e346_d_n4: f64 = (var_igidl_dn4 + var_isub_dn4);
        let eq1_e346_d_n5: f64 = (var_igidl_dn5 + var_isub_dn5);
        let eq1_e346_d_n6: f64 = (var_igidl_dn6 + var_isub_dn6);
        let eq1_e346_d_n8: f64 = (var_igidl_dn8 + var_isub_dn8);
        let eq1_e346_d_n10: f64 = (var_igidl_dn10 + var_isub_dn10);
        let eq1_e346_d_n11: f64 = (var_igidl_dn11 + var_isub_dn11);
        let eq1_e346_d_n12: f64 = (var_igidl_dn12 + var_isub_dn12);
        let eq1_e347: f64 = (p.p33 * eq1_e346);
        let eq1_e347_d_n0: f64 = (p.p33 * eq1_e346_d_n0);
        let eq1_e347_d_n2: f64 = (p.p33 * eq1_e346_d_n2);
        let eq1_e347_d_n4: f64 = (p.p33 * eq1_e346_d_n4);
        let eq1_e347_d_n5: f64 = (p.p33 * eq1_e346_d_n5);
        let eq1_e347_d_n6: f64 = (p.p33 * eq1_e346_d_n6);
        let eq1_e347_d_n8: f64 = (p.p33 * eq1_e346_d_n8);
        let eq1_e347_d_n10: f64 = (p.p33 * eq1_e346_d_n10);
        let eq1_e347_d_n11: f64 = (p.p33 * eq1_e346_d_n11);
        let eq1_e347_d_n12: f64 = (p.p33 * eq1_e346_d_n12);
        let eq1_value: f64 = eq1_e347;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq1_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq1_e347_d_n0), multiplicity * (eq1_e347_d_n2), multiplicity * (eq1_e347_d_n4), multiplicity * (eq1_e347_d_n5), multiplicity * (eq1_e347_d_n6), multiplicity * (eq1_e347_d_n8), multiplicity * (eq1_e347_d_n10), multiplicity * (eq1_e347_d_n11), multiplicity * (eq1_e347_d_n12)],
            [],
            [],
            1.0,
        );
        let eq2_e351: f64 = (var_igisl + var_isubs);
        let eq2_e351_d_n0: f64 = (var_igisl_dn0 + var_isubs_dn0);
        let eq2_e351_d_n2: f64 = (var_igisl_dn2 + var_isubs_dn2);
        let eq2_e351_d_n4: f64 = (var_igisl_dn4 + var_isubs_dn4);
        let eq2_e351_d_n5: f64 = (var_igisl_dn5 + var_isubs_dn5);
        let eq2_e351_d_n6: f64 = (var_igisl_dn6 + var_isubs_dn6);
        let eq2_e351_d_n8: f64 = (var_igisl_dn8 + var_isubs_dn8);
        let eq2_e351_d_n10: f64 = (var_igisl_dn10 + var_isubs_dn10);
        let eq2_e351_d_n11: f64 = (var_igisl_dn11 + var_isubs_dn11);
        let eq2_e351_d_n12: f64 = (var_igisl_dn12 + var_isubs_dn12);
        let eq2_e352: f64 = (p.p33 * eq2_e351);
        let eq2_e352_d_n0: f64 = (p.p33 * eq2_e351_d_n0);
        let eq2_e352_d_n2: f64 = (p.p33 * eq2_e351_d_n2);
        let eq2_e352_d_n4: f64 = (p.p33 * eq2_e351_d_n4);
        let eq2_e352_d_n5: f64 = (p.p33 * eq2_e351_d_n5);
        let eq2_e352_d_n6: f64 = (p.p33 * eq2_e351_d_n6);
        let eq2_e352_d_n8: f64 = (p.p33 * eq2_e351_d_n8);
        let eq2_e352_d_n10: f64 = (p.p33 * eq2_e351_d_n10);
        let eq2_e352_d_n11: f64 = (p.p33 * eq2_e351_d_n11);
        let eq2_e352_d_n12: f64 = (p.p33 * eq2_e351_d_n12);
        let eq2_value: f64 = eq2_e352;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(12),
            Some(11),
            multiplicity * (eq2_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq2_e352_d_n0), multiplicity * (eq2_e352_d_n2), multiplicity * (eq2_e352_d_n4), multiplicity * (eq2_e352_d_n5), multiplicity * (eq2_e352_d_n6), multiplicity * (eq2_e352_d_n8), multiplicity * (eq2_e352_d_n10), multiplicity * (eq2_e352_d_n11), multiplicity * (eq2_e352_d_n12)],
            [],
            [],
            1.0,
        );
        let eq3_e355: f64 = (p.p33 * var_igs);
        let eq3_e355_d_n0: f64 = (p.p33 * var_igs_dn0);
        let eq3_e355_d_n2: f64 = (p.p33 * var_igs_dn2);
        let eq3_e355_d_n4: f64 = (p.p33 * var_igs_dn4);
        let eq3_e355_d_n5: f64 = (p.p33 * var_igs_dn5);
        let eq3_e355_d_n6: f64 = (p.p33 * var_igs_dn6);
        let eq3_e355_d_n8: f64 = (p.p33 * var_igs_dn8);
        let eq3_e355_d_n10: f64 = (p.p33 * var_igs_dn10);
        let eq3_e355_d_n11: f64 = (p.p33 * var_igs_dn11);
        let eq3_e355_d_n12: f64 = (p.p33 * var_igs_dn12);
        let eq3_value: f64 = eq3_e355;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq3_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq3_e355_d_n0), multiplicity * (eq3_e355_d_n2), multiplicity * (eq3_e355_d_n4), multiplicity * (eq3_e355_d_n5), multiplicity * (eq3_e355_d_n6), multiplicity * (eq3_e355_d_n8), multiplicity * (eq3_e355_d_n10), multiplicity * (eq3_e355_d_n11), multiplicity * (eq3_e355_d_n12)],
            [],
            [],
            1.0,
        );
        let eq4_e358: f64 = (p.p33 * var_igd);
        let eq4_e358_d_n0: f64 = (p.p33 * var_igd_dn0);
        let eq4_e358_d_n2: f64 = (p.p33 * var_igd_dn2);
        let eq4_e358_d_n4: f64 = (p.p33 * var_igd_dn4);
        let eq4_e358_d_n5: f64 = (p.p33 * var_igd_dn5);
        let eq4_e358_d_n6: f64 = (p.p33 * var_igd_dn6);
        let eq4_e358_d_n8: f64 = (p.p33 * var_igd_dn8);
        let eq4_e358_d_n10: f64 = (p.p33 * var_igd_dn10);
        let eq4_e358_d_n11: f64 = (p.p33 * var_igd_dn11);
        let eq4_e358_d_n12: f64 = (p.p33 * var_igd_dn12);
        let eq4_value: f64 = eq4_e358;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(11),
            multiplicity * (eq4_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq4_e358_d_n0), multiplicity * (eq4_e358_d_n2), multiplicity * (eq4_e358_d_n4), multiplicity * (eq4_e358_d_n5), multiplicity * (eq4_e358_d_n6), multiplicity * (eq4_e358_d_n8), multiplicity * (eq4_e358_d_n10), multiplicity * (eq4_e358_d_n11), multiplicity * (eq4_e358_d_n12)],
            [],
            [],
            1.0,
        );
        let eq5_e361: f64 = (p.p33 * var_igb);
        let eq5_e361_d_n0: f64 = (p.p33 * var_igb_dn0);
        let eq5_e361_d_n2: f64 = (p.p33 * var_igb_dn2);
        let eq5_e361_d_n4: f64 = (p.p33 * var_igb_dn4);
        let eq5_e361_d_n5: f64 = (p.p33 * var_igb_dn5);
        let eq5_e361_d_n6: f64 = (p.p33 * var_igb_dn6);
        let eq5_e361_d_n8: f64 = (p.p33 * var_igb_dn8);
        let eq5_e361_d_n10: f64 = (p.p33 * var_igb_dn10);
        let eq5_e361_d_n11: f64 = (p.p33 * var_igb_dn11);
        let eq5_e361_d_n12: f64 = (p.p33 * var_igb_dn12);
        let eq5_value: f64 = eq5_e361;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq5_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq5_e361_d_n0), multiplicity * (eq5_e361_d_n2), multiplicity * (eq5_e361_d_n4), multiplicity * (eq5_e361_d_n5), multiplicity * (eq5_e361_d_n6), multiplicity * (eq5_e361_d_n8), multiplicity * (eq5_e361_d_n10), multiplicity * (eq5_e361_d_n11), multiplicity * (eq5_e361_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq6_e367, eq6_e367_d_n0, eq6_e367_d_n2, eq6_e367_d_n4, eq6_e367_d_n5, eq6_e367_d_n6, eq6_e367_d_n8, eq6_e367_d_n10, eq6_e367_d_n11, eq6_e367_d_n12,) = {
    if (p.p312 != 0.0) {
        let eq6_e365: f64 = ((nv12 - nv2) / var_rsd);
        let eq6_e365_d_n0: f64 = (-(((nv12 - nv2) * var_rsd_dn0) / (var_rsd * var_rsd)));
        let eq6_e365_d_n2: f64 = (((-var_rsd) - ((nv12 - nv2) * var_rsd_dn2)) / (var_rsd * var_rsd));
        let eq6_e365_d_n4: f64 = (-(((nv12 - nv2) * var_rsd_dn4) / (var_rsd * var_rsd)));
        let eq6_e365_d_n5: f64 = (-(((nv12 - nv2) * var_rsd_dn5) / (var_rsd * var_rsd)));
        let eq6_e365_d_n6: f64 = (-(((nv12 - nv2) * var_rsd_dn6) / (var_rsd * var_rsd)));
        let eq6_e365_d_n8: f64 = (-(((nv12 - nv2) * var_rsd_dn8) / (var_rsd * var_rsd)));
        let eq6_e365_d_n10: f64 = (-(((nv12 - nv2) * var_rsd_dn10) / (var_rsd * var_rsd)));
        let eq6_e365_d_n11: f64 = (-(((nv12 - nv2) * var_rsd_dn11) / (var_rsd * var_rsd)));
        let eq6_e365_d_n12: f64 = ((var_rsd - ((nv12 - nv2) * var_rsd_dn12)) / (var_rsd * var_rsd));
        (eq6_e365, eq6_e365_d_n0, eq6_e365_d_n2, eq6_e365_d_n4, eq6_e365_d_n5, eq6_e365_d_n6, eq6_e365_d_n8, eq6_e365_d_n10, eq6_e365_d_n11, eq6_e365_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e367;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(12),
            Some(2),
            multiplicity * (eq6_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq6_e367_d_n0), multiplicity * (eq6_e367_d_n2), multiplicity * (eq6_e367_d_n4), multiplicity * (eq6_e367_d_n5), multiplicity * (eq6_e367_d_n6), multiplicity * (eq6_e367_d_n8), multiplicity * (eq6_e367_d_n10), multiplicity * (eq6_e367_d_n11), multiplicity * (eq6_e367_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq8_e378, eq8_e378_d_n0, eq8_e378_d_n2, eq8_e378_d_n4, eq8_e378_d_n5, eq8_e378_d_n6, eq8_e378_d_n8, eq8_e378_d_n10, eq8_e378_d_n11, eq8_e378_d_n12,) = {
    if (p.p313 != 0.0) {
        let eq8_e376: f64 = ((nv0 - nv11) / var_rdd);
        let eq8_e376_d_n0: f64 = ((var_rdd - ((nv0 - nv11) * var_rdd_dn0)) / (var_rdd * var_rdd));
        let eq8_e376_d_n2: f64 = (-(((nv0 - nv11) * var_rdd_dn2) / (var_rdd * var_rdd)));
        let eq8_e376_d_n4: f64 = (-(((nv0 - nv11) * var_rdd_dn4) / (var_rdd * var_rdd)));
        let eq8_e376_d_n5: f64 = (-(((nv0 - nv11) * var_rdd_dn5) / (var_rdd * var_rdd)));
        let eq8_e376_d_n6: f64 = (-(((nv0 - nv11) * var_rdd_dn6) / (var_rdd * var_rdd)));
        let eq8_e376_d_n8: f64 = (-(((nv0 - nv11) * var_rdd_dn8) / (var_rdd * var_rdd)));
        let eq8_e376_d_n10: f64 = (-(((nv0 - nv11) * var_rdd_dn10) / (var_rdd * var_rdd)));
        let eq8_e376_d_n11: f64 = (((-var_rdd) - ((nv0 - nv11) * var_rdd_dn11)) / (var_rdd * var_rdd));
        let eq8_e376_d_n12: f64 = (-(((nv0 - nv11) * var_rdd_dn12) / (var_rdd * var_rdd)));
        (eq8_e376, eq8_e376_d_n0, eq8_e376_d_n2, eq8_e376_d_n4, eq8_e376_d_n5, eq8_e376_d_n6, eq8_e376_d_n8, eq8_e376_d_n10, eq8_e376_d_n11, eq8_e376_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e378;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(11),
            multiplicity * (eq8_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq8_e378_d_n0), multiplicity * (eq8_e378_d_n2), multiplicity * (eq8_e378_d_n4), multiplicity * (eq8_e378_d_n5), multiplicity * (eq8_e378_d_n6), multiplicity * (eq8_e378_d_n8), multiplicity * (eq8_e378_d_n10), multiplicity * (eq8_e378_d_n11), multiplicity * (eq8_e378_d_n12)],
            [],
            [],
            1.0,
        );
        let eq10_e387: f64 = (var_qg + var_qg_nqs);
        let eq10_e387_d_n8: f64 = (var_qg_dn8 + var_qg_nqs_dn8);
        let eq10_e388: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq10_e387);
        let eq10_e389: f64 = (p.p33 * eq10_e388);
        let eq10_e389_d_n0: f64 = (p.p33 * (var_qg_dn0 * ddt_scale));
        let eq10_e389_d_n2: f64 = (p.p33 * (var_qg_dn2 * ddt_scale));
        let eq10_e389_d_n4: f64 = (p.p33 * (var_qg_dn4 * ddt_scale));
        let eq10_e389_d_n5: f64 = (p.p33 * (var_qg_dn5 * ddt_scale));
        let eq10_e389_d_n6: f64 = (p.p33 * (var_qg_dn6 * ddt_scale));
        let eq10_e389_d_n8: f64 = (p.p33 * (eq10_e387_d_n8 * ddt_scale));
        let eq10_e389_d_n9: f64 = (p.p33 * (var_qg_nqs_dn9 * ddt_scale));
        let eq10_e389_d_n10: f64 = (p.p33 * (var_qg_dn10 * ddt_scale));
        let eq10_e389_d_n11: f64 = (p.p33 * (var_qg_dn11 * ddt_scale));
        let eq10_e389_d_n12: f64 = (p.p33 * (var_qg_dn12 * ddt_scale));
        let eq10_value: f64 = eq10_e389;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq10_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq10_e389_d_n0), multiplicity * (eq10_e389_d_n2), multiplicity * (eq10_e389_d_n4), multiplicity * (eq10_e389_d_n5), multiplicity * (eq10_e389_d_n6), multiplicity * (eq10_e389_d_n8), multiplicity * (eq10_e389_d_n9), multiplicity * (eq10_e389_d_n10), multiplicity * (eq10_e389_d_n11), multiplicity * (eq10_e389_d_n12)],
            [],
            [],
            1.0,
        );
        let eq11_e393: f64 = (var_qd + var_qd_nqs);
        let eq11_e393_d_n0: f64 = (var_qd_dn0 + var_qd_nqs_dn0);
        let eq11_e393_d_n2: f64 = (var_qd_dn2 + var_qd_nqs_dn2);
        let eq11_e393_d_n4: f64 = (var_qd_dn4 + var_qd_nqs_dn4);
        let eq11_e393_d_n5: f64 = (var_qd_dn5 + var_qd_nqs_dn5);
        let eq11_e393_d_n6: f64 = (var_qd_dn6 + var_qd_nqs_dn6);
        let eq11_e393_d_n8: f64 = (var_qd_dn8 + var_qd_nqs_dn8);
        let eq11_e393_d_n10: f64 = (var_qd_dn10 + var_qd_nqs_dn10);
        let eq11_e393_d_n11: f64 = (var_qd_dn11 + var_qd_nqs_dn11);
        let eq11_e393_d_n12: f64 = (var_qd_dn12 + var_qd_nqs_dn12);
        let eq11_e394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e393);
        let eq11_e395: f64 = (p.p33 * eq11_e394);
        let eq11_e395_d_n0: f64 = (p.p33 * (eq11_e393_d_n0 * ddt_scale));
        let eq11_e395_d_n2: f64 = (p.p33 * (eq11_e393_d_n2 * ddt_scale));
        let eq11_e395_d_n4: f64 = (p.p33 * (eq11_e393_d_n4 * ddt_scale));
        let eq11_e395_d_n5: f64 = (p.p33 * (eq11_e393_d_n5 * ddt_scale));
        let eq11_e395_d_n6: f64 = (p.p33 * (eq11_e393_d_n6 * ddt_scale));
        let eq11_e395_d_n8: f64 = (p.p33 * (eq11_e393_d_n8 * ddt_scale));
        let eq11_e395_d_n10: f64 = (p.p33 * (eq11_e393_d_n10 * ddt_scale));
        let eq11_e395_d_n11: f64 = (p.p33 * (eq11_e393_d_n11 * ddt_scale));
        let eq11_e395_d_n12: f64 = (p.p33 * (eq11_e393_d_n12 * ddt_scale));
        let eq11_value: f64 = eq11_e395;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq11_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq11_e395_d_n0), multiplicity * (eq11_e395_d_n2), multiplicity * (eq11_e395_d_n4), multiplicity * (eq11_e395_d_n5), multiplicity * (eq11_e395_d_n6), multiplicity * (eq11_e395_d_n8), multiplicity * (eq11_e395_d_n10), multiplicity * (eq11_e395_d_n11), multiplicity * (eq11_e395_d_n12)],
            [],
            [],
            1.0,
        );
        let eq12_e399: f64 = (var_qb + var_qb_nqs);
        let eq12_e400: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e399);
        let eq12_e401: f64 = (p.p33 * eq12_e400);
        let eq12_e401_d_n0: f64 = (p.p33 * (var_qb_dn0 * ddt_scale));
        let eq12_e401_d_n2: f64 = (p.p33 * (var_qb_dn2 * ddt_scale));
        let eq12_e401_d_n4: f64 = (p.p33 * (var_qb_dn4 * ddt_scale));
        let eq12_e401_d_n5: f64 = (p.p33 * (var_qb_dn5 * ddt_scale));
        let eq12_e401_d_n6: f64 = (p.p33 * (var_qb_dn6 * ddt_scale));
        let eq12_e401_d_n8: f64 = (p.p33 * (var_qb_dn8 * ddt_scale));
        let eq12_e401_d_n9: f64 = (p.p33 * (var_qb_nqs_dn9 * ddt_scale));
        let eq12_e401_d_n10: f64 = (p.p33 * (var_qb_dn10 * ddt_scale));
        let eq12_e401_d_n11: f64 = (p.p33 * (var_qb_dn11 * ddt_scale));
        let eq12_e401_d_n12: f64 = (p.p33 * (var_qb_dn12 * ddt_scale));
        let eq12_value: f64 = eq12_e401;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq12_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq12_e401_d_n0), multiplicity * (eq12_e401_d_n2), multiplicity * (eq12_e401_d_n4), multiplicity * (eq12_e401_d_n5), multiplicity * (eq12_e401_d_n6), multiplicity * (eq12_e401_d_n8), multiplicity * (eq12_e401_d_n9), multiplicity * (eq12_e401_d_n10), multiplicity * (eq12_e401_d_n11), multiplicity * (eq12_e401_d_n12)],
            [],
            [],
            1.0,
        );
        let eq17_e427: f64 = (var_ci * (nv7 - 0.0));
        let eq17_e427_d_n0: f64 = (var_ci_dn0 * (nv7 - 0.0));
        let eq17_e427_d_n2: f64 = (var_ci_dn2 * (nv7 - 0.0));
        let eq17_e427_d_n4: f64 = (var_ci_dn4 * (nv7 - 0.0));
        let eq17_e427_d_n5: f64 = (var_ci_dn5 * (nv7 - 0.0));
        let eq17_e427_d_n6: f64 = (var_ci_dn6 * (nv7 - 0.0));
        let eq17_e427_d_n8: f64 = (var_ci_dn8 * (nv7 - 0.0));
        let eq17_e427_d_n10: f64 = (var_ci_dn10 * (nv7 - 0.0));
        let eq17_e427_d_n11: f64 = (var_ci_dn11 * (nv7 - 0.0));
        let eq17_e427_d_n12: f64 = (var_ci_dn12 * (nv7 - 0.0));
        let eq17_value: f64 = eq17_e427;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq17_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * (eq17_e427_d_n0), multiplicity * (eq17_e427_d_n2), multiplicity * (eq17_e427_d_n4), multiplicity * (eq17_e427_d_n5), multiplicity * (eq17_e427_d_n6), multiplicity * (var_ci), multiplicity * (eq17_e427_d_n8), multiplicity * (eq17_e427_d_n10), multiplicity * (eq17_e427_d_n11), multiplicity * (eq17_e427_d_n12)],
            [],
            [],
            1.0,
        );
        let eq18_e430: f64 = ((nv7 - 0.0) * var_sigrat_s);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * var_sigrat_s_dn0);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * var_sigrat_s_dn2);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * var_sigrat_s_dn4);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * var_sigrat_s_dn5);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * var_sigrat_s_dn6);
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * var_sigrat_s_dn8);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * var_sigrat_s_dn10);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * var_sigrat_s_dn11);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * var_sigrat_s_dn12);
        let eq18_e431: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e430);
        let eq18_value: f64 = eq18_e431;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq18_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * ((eq18_e430_d_n0 * ddt_scale)), multiplicity * ((eq18_e430_d_n2 * ddt_scale)), multiplicity * ((eq18_e430_d_n4 * ddt_scale)), multiplicity * ((eq18_e430_d_n5 * ddt_scale)), multiplicity * ((eq18_e430_d_n6 * ddt_scale)), multiplicity * ((var_sigrat_s * ddt_scale)), multiplicity * ((eq18_e430_d_n8 * ddt_scale)), multiplicity * ((eq18_e430_d_n10 * ddt_scale)), multiplicity * ((eq18_e430_d_n11 * ddt_scale)), multiplicity * ((eq18_e430_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq19_e434: f64 = ((nv7 - 0.0) * var_sigrat_d);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * var_sigrat_d_dn0);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * var_sigrat_d_dn2);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * var_sigrat_d_dn4);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * var_sigrat_d_dn5);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * var_sigrat_d_dn6);
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * var_sigrat_d_dn8);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * var_sigrat_d_dn10);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * var_sigrat_d_dn11);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * var_sigrat_d_dn12);
        let eq19_e435: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e434);
        let eq19_value: f64 = eq19_e435;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(11),
            multiplicity * (eq19_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * ((eq19_e434_d_n0 * ddt_scale)), multiplicity * ((eq19_e434_d_n2 * ddt_scale)), multiplicity * ((eq19_e434_d_n4 * ddt_scale)), multiplicity * ((eq19_e434_d_n5 * ddt_scale)), multiplicity * ((eq19_e434_d_n6 * ddt_scale)), multiplicity * ((var_sigrat_d * ddt_scale)), multiplicity * ((eq19_e434_d_n8 * ddt_scale)), multiplicity * ((eq19_e434_d_n10 * ddt_scale)), multiplicity * ((eq19_e434_d_n11 * ddt_scale)), multiplicity * ((eq19_e434_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n2, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n8, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12,) = {
    if (var_guard443 != 0.0) {
        let eq28_e487: f64 = (-var_rpower);
        let eq28_e490: f64 = (var_cthe * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (var_cthe_dn0 * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (var_cthe_dn2 * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((var_cthe_dn4 * (nv4 - 0.0)) + var_cthe);
        let eq28_e490_d_n5: f64 = (var_cthe_dn5 * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (var_cthe_dn6 * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (var_cthe_dn8 * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (var_cthe_dn10 * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (var_cthe_dn11 * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (var_cthe_dn12 * (nv4 - 0.0));
        let eq28_e491: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq28_e490);
        let eq28_e492: f64 = (eq28_e487 + eq28_e491);
        let eq28_e492_d_n0: f64 = ((-var_rpower_dn0) + (eq28_e490_d_n0 * ddt_scale));
        let eq28_e492_d_n2: f64 = ((-var_rpower_dn2) + (eq28_e490_d_n2 * ddt_scale));
        let eq28_e492_d_n4: f64 = ((-var_rpower_dn4) + (eq28_e490_d_n4 * ddt_scale));
        let eq28_e492_d_n5: f64 = ((-var_rpower_dn5) + (eq28_e490_d_n5 * ddt_scale));
        let eq28_e492_d_n6: f64 = ((-var_rpower_dn6) + (eq28_e490_d_n6 * ddt_scale));
        let eq28_e492_d_n8: f64 = ((-var_rpower_dn8) + (eq28_e490_d_n8 * ddt_scale));
        let eq28_e492_d_n10: f64 = ((-var_rpower_dn10) + (eq28_e490_d_n10 * ddt_scale));
        let eq28_e492_d_n11: f64 = ((-var_rpower_dn11) + (eq28_e490_d_n11 * ddt_scale));
        let eq28_e492_d_n12: f64 = ((-var_rpower_dn12) + (eq28_e490_d_n12 * ddt_scale));
        let eq28_e495: f64 = ((nv4 - 0.0) * var_gth);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * var_gth_dn0);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * var_gth_dn2);
        let eq28_e495_d_n4: f64 = (var_gth + ((nv4 - 0.0) * var_gth_dn4));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * var_gth_dn5);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * var_gth_dn6);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * var_gth_dn8);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * var_gth_dn10);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * var_gth_dn11);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * var_gth_dn12);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n2, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n8, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e498;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            None,
            multiplicity * (eq28_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq28_e498_d_n0), multiplicity * (eq28_e498_d_n2), multiplicity * (eq28_e498_d_n4), multiplicity * (eq28_e498_d_n5), multiplicity * (eq28_e498_d_n6), multiplicity * (eq28_e498_d_n8), multiplicity * (eq28_e498_d_n10), multiplicity * (eq28_e498_d_n11), multiplicity * (eq28_e498_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n2, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n8, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12,) = {
    if (var_guard444 != 0.0) {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e509: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq30_e508);
        let eq30_e510: f64 = (var_iqh_nqs + eq30_e509);
        let eq30_e510_d_n10: f64 = (var_iqh_nqs_dn10 + (1e-9 * ddt_scale));
        (eq30_e510, var_iqh_nqs_dn0, var_iqh_nqs_dn2, var_iqh_nqs_dn4, var_iqh_nqs_dn5, var_iqh_nqs_dn6, var_iqh_nqs_dn8, eq30_e510_d_n10, var_iqh_nqs_dn11, var_iqh_nqs_dn12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e512;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            None,
            multiplicity * (eq30_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq30_e512_d_n0), multiplicity * (eq30_e512_d_n2), multiplicity * (eq30_e512_d_n4), multiplicity * (eq30_e512_d_n5), multiplicity * (eq30_e512_d_n6), multiplicity * (eq30_e512_d_n8), multiplicity * (eq30_e512_d_n10), multiplicity * (eq30_e512_d_n11), multiplicity * (eq30_e512_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n2, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n8, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e523: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq32_e522);
        let eq32_e524: f64 = (var_iqi_nqs + eq32_e523);
        let eq32_e524_d_n8: f64 = (var_iqi_nqs_dn8 + (1e-9 * ddt_scale));
        (eq32_e524, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn4, var_iqi_nqs_dn5, var_iqi_nqs_dn6, eq32_e524_d_n8, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e526;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            None,
            multiplicity * (eq32_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq32_e526_d_n0), multiplicity * (eq32_e526_d_n2), multiplicity * (eq32_e526_d_n4), multiplicity * (eq32_e526_d_n5), multiplicity * (eq32_e526_d_n6), multiplicity * (eq32_e526_d_n8), multiplicity * (eq32_e526_d_n10), multiplicity * (eq32_e526_d_n11), multiplicity * (eq32_e526_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n2, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e532: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq33_e531);
        let eq33_e533: f64 = (var_iqb_nqs + eq33_e532);
        let eq33_e533_d_n9: f64 = (var_iqb_nqs_dn9 + (1e-9 * ddt_scale));
        (eq33_e533, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn4, var_iqb_nqs_dn5, var_iqb_nqs_dn6, var_iqb_nqs_dn8, eq33_e533_d_n9, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e535;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            None,
            multiplicity * (eq33_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq33_e535_d_n0), multiplicity * (eq33_e535_d_n2), multiplicity * (eq33_e535_d_n4), multiplicity * (eq33_e535_d_n5), multiplicity * (eq33_e535_d_n6), multiplicity * (eq33_e535_d_n8), multiplicity * (eq33_e535_d_n9), multiplicity * (eq33_e535_d_n10), multiplicity * (eq33_e535_d_n11), multiplicity * (eq33_e535_d_n12)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq10_e387: f64 = (s.v[561] + s.v[554]);
        let eq10_e387_d_n0: f64 = (s.dn[561][0] + s.dn[554][0]);
        let eq10_e387_d_n1: f64 = (s.dn[561][1] + s.dn[554][1]);
        let eq10_e387_d_n2: f64 = (s.dn[561][2] + s.dn[554][2]);
        let eq10_e387_d_n3: f64 = (s.dn[561][3] + s.dn[554][3]);
        let eq10_e387_d_n4: f64 = (s.dn[561][4] + s.dn[554][4]);
        let eq10_e387_d_n5: f64 = (s.dn[561][5] + s.dn[554][5]);
        let eq10_e387_d_n6: f64 = (s.dn[561][6] + s.dn[554][6]);
        let eq10_e387_d_n7: f64 = (s.dn[561][7] + s.dn[554][7]);
        let eq10_e387_d_n8: f64 = (s.dn[561][8] + s.dn[554][8]);
        let eq10_e387_d_n9: f64 = (s.dn[561][9] + s.dn[554][9]);
        let eq10_e387_d_n10: f64 = (s.dn[561][10] + s.dn[554][10]);
        let eq10_e387_d_n11: f64 = (s.dn[561][11] + s.dn[554][11]);
        let eq10_e387_d_n12: f64 = (s.dn[561][12] + s.dn[554][12]);
        let eq10_e387_d_b0: f64 = (s.db[561][0] + s.db[554][0]);
        let eq10_e387_d_b1: f64 = (s.db[561][1] + s.db[554][1]);
        let eq10_e387_d_b2: f64 = (s.db[561][2] + s.db[554][2]);
        let eq10_e387_d_b3: f64 = (s.db[561][3] + s.db[554][3]);
        let eq10_e387_d_b4: f64 = (s.db[561][4] + s.db[554][4]);
        let eq10_e387_d_b5: f64 = (s.db[561][5] + s.db[554][5]);
        let eq10_e387_d_b6: f64 = (s.db[561][6] + s.db[554][6]);
        let eq10_e387_d_b7: f64 = (s.db[561][7] + s.db[554][7]);
        let eq10_e388_q: f64 = eq10_e387;
        let eq10_e389: f64 = (p.p33 * eq10_e387);
        let eq10_e389_d_n0: f64 = (p.p33 * eq10_e387_d_n0);
        let eq10_e389_d_n1: f64 = (p.p33 * eq10_e387_d_n1);
        let eq10_e389_d_n2: f64 = (p.p33 * eq10_e387_d_n2);
        let eq10_e389_d_n3: f64 = (p.p33 * eq10_e387_d_n3);
        let eq10_e389_d_n4: f64 = (p.p33 * eq10_e387_d_n4);
        let eq10_e389_d_n5: f64 = (p.p33 * eq10_e387_d_n5);
        let eq10_e389_d_n6: f64 = (p.p33 * eq10_e387_d_n6);
        let eq10_e389_d_n7: f64 = (p.p33 * eq10_e387_d_n7);
        let eq10_e389_d_n8: f64 = (p.p33 * eq10_e387_d_n8);
        let eq10_e389_d_n9: f64 = (p.p33 * eq10_e387_d_n9);
        let eq10_e389_d_n10: f64 = (p.p33 * eq10_e387_d_n10);
        let eq10_e389_d_n11: f64 = (p.p33 * eq10_e387_d_n11);
        let eq10_e389_d_n12: f64 = (p.p33 * eq10_e387_d_n12);
        let eq10_e389_d_b0: f64 = (p.p33 * eq10_e387_d_b0);
        let eq10_e389_d_b1: f64 = (p.p33 * eq10_e387_d_b1);
        let eq10_e389_d_b2: f64 = (p.p33 * eq10_e387_d_b2);
        let eq10_e389_d_b3: f64 = (p.p33 * eq10_e387_d_b3);
        let eq10_e389_d_b4: f64 = (p.p33 * eq10_e387_d_b4);
        let eq10_e389_d_b5: f64 = (p.p33 * eq10_e387_d_b5);
        let eq10_e389_d_b6: f64 = (p.p33 * eq10_e387_d_b6);
        let eq10_e389_d_b7: f64 = (p.p33 * eq10_e387_d_b7);
        let eq10_e389_q: f64 = (p.p33 * eq10_e388_q);
        let eq10_reactive_node_derivatives: [f64; 13] = [eq10_e389_d_n0, eq10_e389_d_n1, eq10_e389_d_n2, eq10_e389_d_n3, eq10_e389_d_n4, eq10_e389_d_n5, eq10_e389_d_n6, eq10_e389_d_n7, eq10_e389_d_n8, eq10_e389_d_n9, eq10_e389_d_n10, eq10_e389_d_n11, eq10_e389_d_n12];
        let eq10_reactive_branch_derivatives: [f64; 8] = [eq10_e389_d_b0, eq10_e389_d_b1, eq10_e389_d_b2, eq10_e389_d_b3, eq10_e389_d_b4, eq10_e389_d_b5, eq10_e389_d_b6, eq10_e389_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e393: f64 = (s.v[93] + s.v[552]);
        let eq11_e393_d_n0: f64 = (s.dn[93][0] + s.dn[552][0]);
        let eq11_e393_d_n1: f64 = (s.dn[93][1] + s.dn[552][1]);
        let eq11_e393_d_n2: f64 = (s.dn[93][2] + s.dn[552][2]);
        let eq11_e393_d_n3: f64 = (s.dn[93][3] + s.dn[552][3]);
        let eq11_e393_d_n4: f64 = (s.dn[93][4] + s.dn[552][4]);
        let eq11_e393_d_n5: f64 = (s.dn[93][5] + s.dn[552][5]);
        let eq11_e393_d_n6: f64 = (s.dn[93][6] + s.dn[552][6]);
        let eq11_e393_d_n7: f64 = (s.dn[93][7] + s.dn[552][7]);
        let eq11_e393_d_n8: f64 = (s.dn[93][8] + s.dn[552][8]);
        let eq11_e393_d_n9: f64 = (s.dn[93][9] + s.dn[552][9]);
        let eq11_e393_d_n10: f64 = (s.dn[93][10] + s.dn[552][10]);
        let eq11_e393_d_n11: f64 = (s.dn[93][11] + s.dn[552][11]);
        let eq11_e393_d_n12: f64 = (s.dn[93][12] + s.dn[552][12]);
        let eq11_e393_d_b0: f64 = (s.db[93][0] + s.db[552][0]);
        let eq11_e393_d_b1: f64 = (s.db[93][1] + s.db[552][1]);
        let eq11_e393_d_b2: f64 = (s.db[93][2] + s.db[552][2]);
        let eq11_e393_d_b3: f64 = (s.db[93][3] + s.db[552][3]);
        let eq11_e393_d_b4: f64 = (s.db[93][4] + s.db[552][4]);
        let eq11_e393_d_b5: f64 = (s.db[93][5] + s.db[552][5]);
        let eq11_e393_d_b6: f64 = (s.db[93][6] + s.db[552][6]);
        let eq11_e393_d_b7: f64 = (s.db[93][7] + s.db[552][7]);
        let eq11_e394_q: f64 = eq11_e393;
        let eq11_e395: f64 = (p.p33 * eq11_e393);
        let eq11_e395_d_n0: f64 = (p.p33 * eq11_e393_d_n0);
        let eq11_e395_d_n1: f64 = (p.p33 * eq11_e393_d_n1);
        let eq11_e395_d_n2: f64 = (p.p33 * eq11_e393_d_n2);
        let eq11_e395_d_n3: f64 = (p.p33 * eq11_e393_d_n3);
        let eq11_e395_d_n4: f64 = (p.p33 * eq11_e393_d_n4);
        let eq11_e395_d_n5: f64 = (p.p33 * eq11_e393_d_n5);
        let eq11_e395_d_n6: f64 = (p.p33 * eq11_e393_d_n6);
        let eq11_e395_d_n7: f64 = (p.p33 * eq11_e393_d_n7);
        let eq11_e395_d_n8: f64 = (p.p33 * eq11_e393_d_n8);
        let eq11_e395_d_n9: f64 = (p.p33 * eq11_e393_d_n9);
        let eq11_e395_d_n10: f64 = (p.p33 * eq11_e393_d_n10);
        let eq11_e395_d_n11: f64 = (p.p33 * eq11_e393_d_n11);
        let eq11_e395_d_n12: f64 = (p.p33 * eq11_e393_d_n12);
        let eq11_e395_d_b0: f64 = (p.p33 * eq11_e393_d_b0);
        let eq11_e395_d_b1: f64 = (p.p33 * eq11_e393_d_b1);
        let eq11_e395_d_b2: f64 = (p.p33 * eq11_e393_d_b2);
        let eq11_e395_d_b3: f64 = (p.p33 * eq11_e393_d_b3);
        let eq11_e395_d_b4: f64 = (p.p33 * eq11_e393_d_b4);
        let eq11_e395_d_b5: f64 = (p.p33 * eq11_e393_d_b5);
        let eq11_e395_d_b6: f64 = (p.p33 * eq11_e393_d_b6);
        let eq11_e395_d_b7: f64 = (p.p33 * eq11_e393_d_b7);
        let eq11_e395_q: f64 = (p.p33 * eq11_e394_q);
        let eq11_reactive_node_derivatives: [f64; 13] = [eq11_e395_d_n0, eq11_e395_d_n1, eq11_e395_d_n2, eq11_e395_d_n3, eq11_e395_d_n4, eq11_e395_d_n5, eq11_e395_d_n6, eq11_e395_d_n7, eq11_e395_d_n8, eq11_e395_d_n9, eq11_e395_d_n10, eq11_e395_d_n11, eq11_e395_d_n12];
        let eq11_reactive_branch_derivatives: [f64; 8] = [eq11_e395_d_b0, eq11_e395_d_b1, eq11_e395_d_b2, eq11_e395_d_b3, eq11_e395_d_b4, eq11_e395_d_b5, eq11_e395_d_b6, eq11_e395_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e399: f64 = (s.v[90] + s.v[548]);
        let eq12_e399_d_n0: f64 = (s.dn[90][0] + s.dn[548][0]);
        let eq12_e399_d_n1: f64 = (s.dn[90][1] + s.dn[548][1]);
        let eq12_e399_d_n2: f64 = (s.dn[90][2] + s.dn[548][2]);
        let eq12_e399_d_n3: f64 = (s.dn[90][3] + s.dn[548][3]);
        let eq12_e399_d_n4: f64 = (s.dn[90][4] + s.dn[548][4]);
        let eq12_e399_d_n5: f64 = (s.dn[90][5] + s.dn[548][5]);
        let eq12_e399_d_n6: f64 = (s.dn[90][6] + s.dn[548][6]);
        let eq12_e399_d_n7: f64 = (s.dn[90][7] + s.dn[548][7]);
        let eq12_e399_d_n8: f64 = (s.dn[90][8] + s.dn[548][8]);
        let eq12_e399_d_n9: f64 = (s.dn[90][9] + s.dn[548][9]);
        let eq12_e399_d_n10: f64 = (s.dn[90][10] + s.dn[548][10]);
        let eq12_e399_d_n11: f64 = (s.dn[90][11] + s.dn[548][11]);
        let eq12_e399_d_n12: f64 = (s.dn[90][12] + s.dn[548][12]);
        let eq12_e399_d_b0: f64 = (s.db[90][0] + s.db[548][0]);
        let eq12_e399_d_b1: f64 = (s.db[90][1] + s.db[548][1]);
        let eq12_e399_d_b2: f64 = (s.db[90][2] + s.db[548][2]);
        let eq12_e399_d_b3: f64 = (s.db[90][3] + s.db[548][3]);
        let eq12_e399_d_b4: f64 = (s.db[90][4] + s.db[548][4]);
        let eq12_e399_d_b5: f64 = (s.db[90][5] + s.db[548][5]);
        let eq12_e399_d_b6: f64 = (s.db[90][6] + s.db[548][6]);
        let eq12_e399_d_b7: f64 = (s.db[90][7] + s.db[548][7]);
        let eq12_e400_q: f64 = eq12_e399;
        let eq12_e401: f64 = (p.p33 * eq12_e399);
        let eq12_e401_d_n0: f64 = (p.p33 * eq12_e399_d_n0);
        let eq12_e401_d_n1: f64 = (p.p33 * eq12_e399_d_n1);
        let eq12_e401_d_n2: f64 = (p.p33 * eq12_e399_d_n2);
        let eq12_e401_d_n3: f64 = (p.p33 * eq12_e399_d_n3);
        let eq12_e401_d_n4: f64 = (p.p33 * eq12_e399_d_n4);
        let eq12_e401_d_n5: f64 = (p.p33 * eq12_e399_d_n5);
        let eq12_e401_d_n6: f64 = (p.p33 * eq12_e399_d_n6);
        let eq12_e401_d_n7: f64 = (p.p33 * eq12_e399_d_n7);
        let eq12_e401_d_n8: f64 = (p.p33 * eq12_e399_d_n8);
        let eq12_e401_d_n9: f64 = (p.p33 * eq12_e399_d_n9);
        let eq12_e401_d_n10: f64 = (p.p33 * eq12_e399_d_n10);
        let eq12_e401_d_n11: f64 = (p.p33 * eq12_e399_d_n11);
        let eq12_e401_d_n12: f64 = (p.p33 * eq12_e399_d_n12);
        let eq12_e401_d_b0: f64 = (p.p33 * eq12_e399_d_b0);
        let eq12_e401_d_b1: f64 = (p.p33 * eq12_e399_d_b1);
        let eq12_e401_d_b2: f64 = (p.p33 * eq12_e399_d_b2);
        let eq12_e401_d_b3: f64 = (p.p33 * eq12_e399_d_b3);
        let eq12_e401_d_b4: f64 = (p.p33 * eq12_e399_d_b4);
        let eq12_e401_d_b5: f64 = (p.p33 * eq12_e399_d_b5);
        let eq12_e401_d_b6: f64 = (p.p33 * eq12_e399_d_b6);
        let eq12_e401_d_b7: f64 = (p.p33 * eq12_e399_d_b7);
        let eq12_e401_q: f64 = (p.p33 * eq12_e400_q);
        let eq12_reactive_node_derivatives: [f64; 13] = [eq12_e401_d_n0, eq12_e401_d_n1, eq12_e401_d_n2, eq12_e401_d_n3, eq12_e401_d_n4, eq12_e401_d_n5, eq12_e401_d_n6, eq12_e401_d_n7, eq12_e401_d_n8, eq12_e401_d_n9, eq12_e401_d_n10, eq12_e401_d_n11, eq12_e401_d_n12];
        let eq12_reactive_branch_derivatives: [f64; 8] = [eq12_e401_d_b0, eq12_e401_d_b1, eq12_e401_d_b2, eq12_e401_d_b3, eq12_e401_d_b4, eq12_e401_d_b5, eq12_e401_d_b6, eq12_e401_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[12]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e430: f64 = ((nv7 - 0.0) * s.v[611]);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * s.dn[611][0]);
        let eq18_e430_d_n1: f64 = ((nv7 - 0.0) * s.dn[611][1]);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * s.dn[611][2]);
        let eq18_e430_d_n3: f64 = ((nv7 - 0.0) * s.dn[611][3]);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * s.dn[611][4]);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * s.dn[611][5]);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * s.dn[611][6]);
        let eq18_e430_d_n7: f64 = (s.v[611] + ((nv7 - 0.0) * s.dn[611][7]));
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * s.dn[611][8]);
        let eq18_e430_d_n9: f64 = ((nv7 - 0.0) * s.dn[611][9]);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * s.dn[611][10]);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * s.dn[611][11]);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * s.dn[611][12]);
        let eq18_e430_d_b0: f64 = ((nv7 - 0.0) * s.db[611][0]);
        let eq18_e430_d_b1: f64 = ((nv7 - 0.0) * s.db[611][1]);
        let eq18_e430_d_b2: f64 = ((nv7 - 0.0) * s.db[611][2]);
        let eq18_e430_d_b3: f64 = ((nv7 - 0.0) * s.db[611][3]);
        let eq18_e430_d_b4: f64 = ((nv7 - 0.0) * s.db[611][4]);
        let eq18_e430_d_b5: f64 = ((nv7 - 0.0) * s.db[611][5]);
        let eq18_e430_d_b6: f64 = ((nv7 - 0.0) * s.db[611][6]);
        let eq18_e430_d_b7: f64 = ((nv7 - 0.0) * s.db[611][7]);
        let eq18_e431_q: f64 = eq18_e430;
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e430_d_n0, eq18_e430_d_n1, eq18_e430_d_n2, eq18_e430_d_n3, eq18_e430_d_n4, eq18_e430_d_n5, eq18_e430_d_n6, eq18_e430_d_n7, eq18_e430_d_n8, eq18_e430_d_n9, eq18_e430_d_n10, eq18_e430_d_n11, eq18_e430_d_n12];
        let eq18_reactive_branch_derivatives: [f64; 8] = [eq18_e430_d_b0, eq18_e430_d_b1, eq18_e430_d_b2, eq18_e430_d_b3, eq18_e430_d_b4, eq18_e430_d_b5, eq18_e430_d_b6, eq18_e430_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e434: f64 = ((nv7 - 0.0) * s.v[612]);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * s.dn[612][0]);
        let eq19_e434_d_n1: f64 = ((nv7 - 0.0) * s.dn[612][1]);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * s.dn[612][2]);
        let eq19_e434_d_n3: f64 = ((nv7 - 0.0) * s.dn[612][3]);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * s.dn[612][4]);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * s.dn[612][5]);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * s.dn[612][6]);
        let eq19_e434_d_n7: f64 = (s.v[612] + ((nv7 - 0.0) * s.dn[612][7]));
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * s.dn[612][8]);
        let eq19_e434_d_n9: f64 = ((nv7 - 0.0) * s.dn[612][9]);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * s.dn[612][10]);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * s.dn[612][11]);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * s.dn[612][12]);
        let eq19_e434_d_b0: f64 = ((nv7 - 0.0) * s.db[612][0]);
        let eq19_e434_d_b1: f64 = ((nv7 - 0.0) * s.db[612][1]);
        let eq19_e434_d_b2: f64 = ((nv7 - 0.0) * s.db[612][2]);
        let eq19_e434_d_b3: f64 = ((nv7 - 0.0) * s.db[612][3]);
        let eq19_e434_d_b4: f64 = ((nv7 - 0.0) * s.db[612][4]);
        let eq19_e434_d_b5: f64 = ((nv7 - 0.0) * s.db[612][5]);
        let eq19_e434_d_b6: f64 = ((nv7 - 0.0) * s.db[612][6]);
        let eq19_e434_d_b7: f64 = ((nv7 - 0.0) * s.db[612][7]);
        let eq19_e435_q: f64 = eq19_e434;
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e434_d_n0, eq19_e434_d_n1, eq19_e434_d_n2, eq19_e434_d_n3, eq19_e434_d_n4, eq19_e434_d_n5, eq19_e434_d_n6, eq19_e434_d_n7, eq19_e434_d_n8, eq19_e434_d_n9, eq19_e434_d_n10, eq19_e434_d_n11, eq19_e434_d_n12];
        let eq19_reactive_branch_derivatives: [f64; 8] = [eq19_e434_d_b0, eq19_e434_d_b1, eq19_e434_d_b2, eq19_e434_d_b3, eq19_e434_d_b4, eq19_e434_d_b5, eq19_e434_d_b6, eq19_e434_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n1, eq28_e498_d_n2, eq28_e498_d_n3, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n7, eq28_e498_d_n8, eq28_e498_d_n9, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12, eq28_e498_d_b0, eq28_e498_d_b1, eq28_e498_d_b2, eq28_e498_d_b3, eq28_e498_d_b4, eq28_e498_d_b5, eq28_e498_d_b6, eq28_e498_d_b7, eq28_e498_q, eq28_e498_q_d_n0, eq28_e498_q_d_n1, eq28_e498_q_d_n2, eq28_e498_q_d_n3, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, eq28_e498_q_d_n7, eq28_e498_q_d_n8, eq28_e498_q_d_n9, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12, eq28_e498_q_d_b0, eq28_e498_q_d_b1, eq28_e498_q_d_b2, eq28_e498_q_d_b3, eq28_e498_q_d_b4, eq28_e498_q_d_b5, eq28_e498_q_d_b6, eq28_e498_q_d_b7,) = {
    if s.b[1094] {
        let eq28_e487: f64 = (-s.v[547]);
        let eq28_e490: f64 = (s.v[516] * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (s.dn[516][0] * (nv4 - 0.0));
        let eq28_e490_d_n1: f64 = (s.dn[516][1] * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (s.dn[516][2] * (nv4 - 0.0));
        let eq28_e490_d_n3: f64 = (s.dn[516][3] * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((s.dn[516][4] * (nv4 - 0.0)) + s.v[516]);
        let eq28_e490_d_n5: f64 = (s.dn[516][5] * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (s.dn[516][6] * (nv4 - 0.0));
        let eq28_e490_d_n7: f64 = (s.dn[516][7] * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (s.dn[516][8] * (nv4 - 0.0));
        let eq28_e490_d_n9: f64 = (s.dn[516][9] * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (s.dn[516][10] * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (s.dn[516][11] * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (s.dn[516][12] * (nv4 - 0.0));
        let eq28_e490_d_b0: f64 = (s.db[516][0] * (nv4 - 0.0));
        let eq28_e490_d_b1: f64 = (s.db[516][1] * (nv4 - 0.0));
        let eq28_e490_d_b2: f64 = (s.db[516][2] * (nv4 - 0.0));
        let eq28_e490_d_b3: f64 = (s.db[516][3] * (nv4 - 0.0));
        let eq28_e490_d_b4: f64 = (s.db[516][4] * (nv4 - 0.0));
        let eq28_e490_d_b5: f64 = (s.db[516][5] * (nv4 - 0.0));
        let eq28_e490_d_b6: f64 = (s.db[516][6] * (nv4 - 0.0));
        let eq28_e490_d_b7: f64 = (s.db[516][7] * (nv4 - 0.0));
        let eq28_e491_q: f64 = eq28_e490;
        let eq28_e492: f64 = (eq28_e487 + eq28_e490);
        let eq28_e492_d_n0: f64 = ((-s.dn[547][0]) + eq28_e490_d_n0);
        let eq28_e492_d_n1: f64 = ((-s.dn[547][1]) + eq28_e490_d_n1);
        let eq28_e492_d_n2: f64 = ((-s.dn[547][2]) + eq28_e490_d_n2);
        let eq28_e492_d_n3: f64 = ((-s.dn[547][3]) + eq28_e490_d_n3);
        let eq28_e492_d_n4: f64 = ((-s.dn[547][4]) + eq28_e490_d_n4);
        let eq28_e492_d_n5: f64 = ((-s.dn[547][5]) + eq28_e490_d_n5);
        let eq28_e492_d_n6: f64 = ((-s.dn[547][6]) + eq28_e490_d_n6);
        let eq28_e492_d_n7: f64 = ((-s.dn[547][7]) + eq28_e490_d_n7);
        let eq28_e492_d_n8: f64 = ((-s.dn[547][8]) + eq28_e490_d_n8);
        let eq28_e492_d_n9: f64 = ((-s.dn[547][9]) + eq28_e490_d_n9);
        let eq28_e492_d_n10: f64 = ((-s.dn[547][10]) + eq28_e490_d_n10);
        let eq28_e492_d_n11: f64 = ((-s.dn[547][11]) + eq28_e490_d_n11);
        let eq28_e492_d_n12: f64 = ((-s.dn[547][12]) + eq28_e490_d_n12);
        let eq28_e492_d_b0: f64 = ((-s.db[547][0]) + eq28_e490_d_b0);
        let eq28_e492_d_b1: f64 = ((-s.db[547][1]) + eq28_e490_d_b1);
        let eq28_e492_d_b2: f64 = ((-s.db[547][2]) + eq28_e490_d_b2);
        let eq28_e492_d_b3: f64 = ((-s.db[547][3]) + eq28_e490_d_b3);
        let eq28_e492_d_b4: f64 = ((-s.db[547][4]) + eq28_e490_d_b4);
        let eq28_e492_d_b5: f64 = ((-s.db[547][5]) + eq28_e490_d_b5);
        let eq28_e492_d_b6: f64 = ((-s.db[547][6]) + eq28_e490_d_b6);
        let eq28_e492_d_b7: f64 = ((-s.db[547][7]) + eq28_e490_d_b7);
        let eq28_e492_q: f64 = eq28_e491_q;
        let eq28_e495: f64 = ((nv4 - 0.0) * s.v[557]);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * s.dn[557][0]);
        let eq28_e495_d_n1: f64 = ((nv4 - 0.0) * s.dn[557][1]);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * s.dn[557][2]);
        let eq28_e495_d_n3: f64 = ((nv4 - 0.0) * s.dn[557][3]);
        let eq28_e495_d_n4: f64 = (s.v[557] + ((nv4 - 0.0) * s.dn[557][4]));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * s.dn[557][5]);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * s.dn[557][6]);
        let eq28_e495_d_n7: f64 = ((nv4 - 0.0) * s.dn[557][7]);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * s.dn[557][8]);
        let eq28_e495_d_n9: f64 = ((nv4 - 0.0) * s.dn[557][9]);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * s.dn[557][10]);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * s.dn[557][11]);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * s.dn[557][12]);
        let eq28_e495_d_b0: f64 = ((nv4 - 0.0) * s.db[557][0]);
        let eq28_e495_d_b1: f64 = ((nv4 - 0.0) * s.db[557][1]);
        let eq28_e495_d_b2: f64 = ((nv4 - 0.0) * s.db[557][2]);
        let eq28_e495_d_b3: f64 = ((nv4 - 0.0) * s.db[557][3]);
        let eq28_e495_d_b4: f64 = ((nv4 - 0.0) * s.db[557][4]);
        let eq28_e495_d_b5: f64 = ((nv4 - 0.0) * s.db[557][5]);
        let eq28_e495_d_b6: f64 = ((nv4 - 0.0) * s.db[557][6]);
        let eq28_e495_d_b7: f64 = ((nv4 - 0.0) * s.db[557][7]);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n1: f64 = (eq28_e492_d_n1 + eq28_e495_d_n1);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n3: f64 = (eq28_e492_d_n3 + eq28_e495_d_n3);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n7: f64 = (eq28_e492_d_n7 + eq28_e495_d_n7);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n9: f64 = (eq28_e492_d_n9 + eq28_e495_d_n9);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        let eq28_e496_d_b0: f64 = (eq28_e492_d_b0 + eq28_e495_d_b0);
        let eq28_e496_d_b1: f64 = (eq28_e492_d_b1 + eq28_e495_d_b1);
        let eq28_e496_d_b2: f64 = (eq28_e492_d_b2 + eq28_e495_d_b2);
        let eq28_e496_d_b3: f64 = (eq28_e492_d_b3 + eq28_e495_d_b3);
        let eq28_e496_d_b4: f64 = (eq28_e492_d_b4 + eq28_e495_d_b4);
        let eq28_e496_d_b5: f64 = (eq28_e492_d_b5 + eq28_e495_d_b5);
        let eq28_e496_d_b6: f64 = (eq28_e492_d_b6 + eq28_e495_d_b6);
        let eq28_e496_d_b7: f64 = (eq28_e492_d_b7 + eq28_e495_d_b7);
        let eq28_e496_q: f64 = eq28_e492_q;
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n1, eq28_e496_d_n2, eq28_e496_d_n3, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n7, eq28_e496_d_n8, eq28_e496_d_n9, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12, eq28_e496_d_b0, eq28_e496_d_b1, eq28_e496_d_b2, eq28_e496_d_b3, eq28_e496_d_b4, eq28_e496_d_b5, eq28_e496_d_b6, eq28_e496_d_b7, eq28_e496_q, eq28_e490_d_n0, eq28_e490_d_n1, eq28_e490_d_n2, eq28_e490_d_n3, eq28_e490_d_n4, eq28_e490_d_n5, eq28_e490_d_n6, eq28_e490_d_n7, eq28_e490_d_n8, eq28_e490_d_n9, eq28_e490_d_n10, eq28_e490_d_n11, eq28_e490_d_n12, eq28_e490_d_b0, eq28_e490_d_b1, eq28_e490_d_b2, eq28_e490_d_b3, eq28_e490_d_b4, eq28_e490_d_b5, eq28_e490_d_b6, eq28_e490_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e498_q_d_n0, eq28_e498_q_d_n1, eq28_e498_q_d_n2, eq28_e498_q_d_n3, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, eq28_e498_q_d_n7, eq28_e498_q_d_n8, eq28_e498_q_d_n9, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12];
        let eq28_reactive_branch_derivatives: [f64; 8] = [eq28_e498_q_d_b0, eq28_e498_q_d_b1, eq28_e498_q_d_b2, eq28_e498_q_d_b3, eq28_e498_q_d_b4, eq28_e498_q_d_b5, eq28_e498_q_d_b6, eq28_e498_q_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n1, eq30_e512_d_n2, eq30_e512_d_n3, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n7, eq30_e512_d_n8, eq30_e512_d_n9, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12, eq30_e512_d_b0, eq30_e512_d_b1, eq30_e512_d_b2, eq30_e512_d_b3, eq30_e512_d_b4, eq30_e512_d_b5, eq30_e512_d_b6, eq30_e512_d_b7, eq30_e512_q, eq30_e512_q_d_n10,) = {
    if s.b[1095] {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e509_q: f64 = eq30_e508;
        let eq30_e510: f64 = (s.v[558] + eq30_e508);
        let eq30_e510_d_n10: f64 = (s.dn[558][10] + 1e-9);
        let eq30_e510_q: f64 = eq30_e509_q;
        (eq30_e510, s.dn[558][0], s.dn[558][1], s.dn[558][2], s.dn[558][3], s.dn[558][4], s.dn[558][5], s.dn[558][6], s.dn[558][7], s.dn[558][8], s.dn[558][9], eq30_e510_d_n10, s.dn[558][11], s.dn[558][12], s.db[558][0], s.db[558][1], s.db[558][2], s.db[558][3], s.db[558][4], s.db[558][5], s.db[558][6], s.db[558][7], eq30_e510_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e512_q_d_n10),
        );
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n1, eq32_e526_d_n2, eq32_e526_d_n3, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n7, eq32_e526_d_n8, eq32_e526_d_n9, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12, eq32_e526_d_b0, eq32_e526_d_b1, eq32_e526_d_b2, eq32_e526_d_b3, eq32_e526_d_b4, eq32_e526_d_b5, eq32_e526_d_b6, eq32_e526_d_b7, eq32_e526_q, eq32_e526_q_d_n8,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e523_q: f64 = eq32_e522;
        let eq32_e524: f64 = (s.v[549] + eq32_e522);
        let eq32_e524_d_n8: f64 = (s.dn[549][8] + 1e-9);
        let eq32_e524_q: f64 = eq32_e523_q;
        (eq32_e524, s.dn[549][0], s.dn[549][1], s.dn[549][2], s.dn[549][3], s.dn[549][4], s.dn[549][5], s.dn[549][6], s.dn[549][7], eq32_e524_d_n8, s.dn[549][9], s.dn[549][10], s.dn[549][11], s.dn[549][12], s.db[549][0], s.db[549][1], s.db[549][2], s.db[549][3], s.db[549][4], s.db[549][5], s.db[549][6], s.db[549][7], eq32_e524_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[8]),
            None,
            nodes[8],
            multiplicity * (eq32_e526_q_d_n8),
        );
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n1, eq33_e535_d_n2, eq33_e535_d_n3, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n7, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12, eq33_e535_d_b0, eq33_e535_d_b1, eq33_e535_d_b2, eq33_e535_d_b3, eq33_e535_d_b4, eq33_e535_d_b5, eq33_e535_d_b6, eq33_e535_d_b7, eq33_e535_q, eq33_e535_q_d_n9,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e532_q: f64 = eq33_e531;
        let eq33_e533: f64 = (s.v[550] + eq33_e531);
        let eq33_e533_d_n9: f64 = (s.dn[550][9] + 1e-9);
        let eq33_e533_q: f64 = eq33_e532_q;
        (eq33_e533, s.dn[550][0], s.dn[550][1], s.dn[550][2], s.dn[550][3], s.dn[550][4], s.dn[550][5], s.dn[550][6], s.dn[550][7], s.dn[550][8], eq33_e533_d_n9, s.dn[550][10], s.dn[550][11], s.dn[550][12], s.db[550][0], s.db[550][1], s.db[550][2], s.db[550][3], s.db[550][4], s.db[550][5], s.db[550][6], s.db[550][7], eq33_e533_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (eq33_e535_q_d_n9),
        );
    }
}
