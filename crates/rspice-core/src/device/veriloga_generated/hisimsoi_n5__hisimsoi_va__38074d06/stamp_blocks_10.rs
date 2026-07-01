#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1831] && (s.v[85] != 0.0)) {
            if (s.v[613] == 1.0) {
                s.copy_ad(438, 556);
            } else {
                s.store_sub_from_scalar(438, 1.0, 556);
            }
        }

        if (s.b[1831] && (s.v[85] != 0.0)) {
            s.store_add_scaled_product_indices(584, 473, 1.0, 580, 438, 1.0);
            s.store_add_ad_lhs(585, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(438)), 473);
            s.store_add_scaled_inputs3_indices(586, 580, -1.0, 581, (-1.0), 471, 1.0);
        }

        if (s.b[1831] && (s.v[85] == 0.0)) {
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(586, 0.0);
            s.store_scalar(581, 0.0);
        }

        if ((!s.b[1831]) && (s.v[85] != 0.0)) {
            s.store_add_scaled_inputs3_indices(586, 584, -1.0, 585, (-1.0), 581, -1.0);
        }

        if ((!s.b[1831]) && (s.v[85] == 0.0)) {
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(586, 0.0);
            s.store_scalar(581, 0.0);
        }

        s.b[1836] = (s.v[613] == 1.0);
        s.store_scalar(1836, if s.b[1836] { 1.0 } else { 0.0 });

        if s.b[1836] {
            s.copy_ad(199, 9);
            s.copy_ad(263, 557);
            s.store_add(594, 23, 586);
            s.store_add(198, 24, 584);
            s.store_add_scaled_inputs3_indices(554, 23, (-1.0), 24, (-1.0), 25, (-1.0));
            s.store_add(196, 554, 581);
        }

        if (!s.b[1836]) {
            s.store_neg(199, 9);
            s.store_scalar(263, 0.0);
            s.store_add(594, 23, 586);
            s.store_add(198, 25, 585);
            s.store_add_scaled_inputs3_indices(554, 23, (-1.0), 24, (-1.0), 25, (-1.0));
            s.store_add(196, 554, 581);
        }

        s.b[1837] = (p.p43 == 1.0);
        s.store_scalar(1837, if s.b[1837] { 1.0 } else { 0.0 });

        if s.b[1837] {
            s.copy_ad(282, 35);
            s.copy_ad(284, 560);
            s.copy_ad(281, 36);
            s.copy_ad(283, 561);
        }

        s.b[1838] = ((p.p38 == 1.0) && (s.v[67] > 0.0));
        s.store_scalar(1838, if s.b[1838] { 1.0 } else { 0.0 });

        if s.b[1838] {
            s.copy_ad(563, 542);
        }

        if (!s.b[1838]) {
            s.store_scalar(563, 0.0);
        }

        s.copy_ad(9, 199);

        s.store_scalar(27, A::ddx_projection(&s.ad_value(594), Some(6), None));

        s.store_scale(27, 27, p.p50);

        s.store_scalar(28, A::ddx_projection(&s.ad_value(594), Some(7), None));

        s.store_scale(28, 28, p.p50);

        s.b[1840] = (p.p43 == 1.0);
        s.store_scalar(1840, if s.b[1840] { 1.0 } else { 0.0 });

        if s.b[1840] {
            s.store_scale(35, 282, p.p50);
            s.store_scale(36, 281, p.p50);
        }

        s.store_scale(610, 429, (4.0 * 1.3806226e-23));

        s.copy_ad(438, 439);

        s.store_mul(615, 610, 598);

        if ((s.v[615] > 0.0) && (s.v[558] > 0.0)) {
            s.store_sqrt_div(616, 558, 615);
        } else {
            s.store_scalar(616, 0.0);
        }

        if (s.v[613] > 0.0) {
            s.store_mul_sub_from_scalar_rhs(617, 616, 1.0, 438);
        } else {
            s.store_mul(617, 616, 438);
        }

        if (s.v[613] > 0.0) {
            s.store_mul(618, 616, 438);
        } else {
            s.store_mul_sub_from_scalar_rhs(618, 616, 1.0, 438);
        }

        s.b[1848] = ((p.p38 > 0.0) && (p.p242 > 0.0));
        s.store_scalar(1848, if s.b[1848] { 1.0 } else { 0.0 });

        s.b[1849] = (p.p43 == 1.0);
        s.store_scalar(1849, if s.b[1849] { 1.0 } else { 0.0 });

        s.b[1850] = ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0)));
        s.store_scalar(1850, if s.b[1850] { 1.0 } else { 0.0 });

    }

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
        var_ci_dn17: f64,
        var_ci_dn2: f64,
        var_ci_dn6: f64,
        var_ci_dn7: f64,
        var_cthe: f64,
        var_grg: f64,
        var_gth: f64,
        var_guard1222: f64,
        var_guard1224: f64,
        var_guard1225: f64,
        var_ibs: f64,
        var_ibs_dn0: f64,
        var_ibs_dn10: f64,
        var_ibs_dn11: f64,
        var_ibs_dn12: f64,
        var_ibs_dn17: f64,
        var_ibs_dn2: f64,
        var_ibs_dn6: f64,
        var_ibs_dn7: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn12: f64,
        var_ids_dn17: f64,
        var_ids_dn2: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_igb: f64,
        var_igb_dn0: f64,
        var_igb_dn10: f64,
        var_igb_dn11: f64,
        var_igb_dn12: f64,
        var_igb_dn17: f64,
        var_igb_dn2: f64,
        var_igb_dn6: f64,
        var_igb_dn7: f64,
        var_igd: f64,
        var_igd_dn0: f64,
        var_igd_dn10: f64,
        var_igd_dn11: f64,
        var_igd_dn12: f64,
        var_igd_dn17: f64,
        var_igd_dn2: f64,
        var_igd_dn6: f64,
        var_igd_dn7: f64,
        var_igidl: f64,
        var_igidl_dn0: f64,
        var_igidl_dn10: f64,
        var_igidl_dn11: f64,
        var_igidl_dn12: f64,
        var_igidl_dn17: f64,
        var_igidl_dn2: f64,
        var_igidl_dn6: f64,
        var_igidl_dn7: f64,
        var_igisl: f64,
        var_igisl_dn0: f64,
        var_igisl_dn10: f64,
        var_igisl_dn11: f64,
        var_igisl_dn12: f64,
        var_igisl_dn17: f64,
        var_igisl_dn2: f64,
        var_igisl_dn6: f64,
        var_igisl_dn7: f64,
        var_igs: f64,
        var_igs_dn0: f64,
        var_igs_dn10: f64,
        var_igs_dn11: f64,
        var_igs_dn12: f64,
        var_igs_dn17: f64,
        var_igs_dn2: f64,
        var_igs_dn6: f64,
        var_igs_dn7: f64,
        var_isub: f64,
        var_isub_dn0: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn17: f64,
        var_isub_dn2: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isubs: f64,
        var_isubs_dn0: f64,
        var_isubs_dn10: f64,
        var_isubs_dn11: f64,
        var_isubs_dn12: f64,
        var_isubs_dn17: f64,
        var_isubs_dn2: f64,
        var_isubs_dn6: f64,
        var_isubs_dn7: f64,
        var_itemp: f64,
        var_itemp_dn0: f64,
        var_itemp_dn10: f64,
        var_itemp_dn11: f64,
        var_itemp_dn12: f64,
        var_itemp_dn17: f64,
        var_itemp_dn2: f64,
        var_itemp_dn6: f64,
        var_itemp_dn7: f64,
        var_qb: f64,
        var_qb_dn0: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn12: f64,
        var_qb_dn13: f64,
        var_qb_dn15: f64,
        var_qb_dn16: f64,
        var_qb_dn17: f64,
        var_qb_dn18: f64,
        var_qb_dn2: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qbs: f64,
        var_qbs_dn0: f64,
        var_qbs_dn10: f64,
        var_qbs_dn11: f64,
        var_qbs_dn12: f64,
        var_qbs_dn17: f64,
        var_qbs_dn2: f64,
        var_qbs_dn6: f64,
        var_qbs_dn7: f64,
        var_qd: f64,
        var_qd_dn0: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn12: f64,
        var_qd_dn13: f64,
        var_qd_dn15: f64,
        var_qd_dn16: f64,
        var_qd_dn17: f64,
        var_qd_dn18: f64,
        var_qd_dn2: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qg: f64,
        var_qg_dn0: f64,
        var_qg_dn10: f64,
        var_qg_dn11: f64,
        var_qg_dn12: f64,
        var_qg_dn13: f64,
        var_qg_dn15: f64,
        var_qg_dn16: f64,
        var_qg_dn17: f64,
        var_qg_dn18: f64,
        var_qg_dn2: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_rdd: f64,
        var_rdd_dn0: f64,
        var_rdd_dn10: f64,
        var_rdd_dn11: f64,
        var_rdd_dn12: f64,
        var_rdd_dn17: f64,
        var_rdd_dn2: f64,
        var_rdd_dn6: f64,
        var_rdd_dn7: f64,
        var_rsd: f64,
        var_rsd_dn0: f64,
        var_rsd_dn10: f64,
        var_rsd_dn11: f64,
        var_rsd_dn12: f64,
        var_rsd_dn17: f64,
        var_rsd_dn2: f64,
        var_rsd_dn6: f64,
        var_rsd_dn7: f64,
        var_sigrat_d: f64,
        var_sigrat_d_dn0: f64,
        var_sigrat_d_dn10: f64,
        var_sigrat_d_dn11: f64,
        var_sigrat_d_dn12: f64,
        var_sigrat_d_dn13: f64,
        var_sigrat_d_dn15: f64,
        var_sigrat_d_dn16: f64,
        var_sigrat_d_dn17: f64,
        var_sigrat_d_dn18: f64,
        var_sigrat_d_dn2: f64,
        var_sigrat_d_dn6: f64,
        var_sigrat_d_dn7: f64,
        var_sigrat_s: f64,
        var_sigrat_s_dn0: f64,
        var_sigrat_s_dn10: f64,
        var_sigrat_s_dn11: f64,
        var_sigrat_s_dn12: f64,
        var_sigrat_s_dn13: f64,
        var_sigrat_s_dn15: f64,
        var_sigrat_s_dn16: f64,
        var_sigrat_s_dn17: f64,
        var_sigrat_s_dn18: f64,
        var_sigrat_s_dn2: f64,
        var_sigrat_s_dn6: f64,
        var_sigrat_s_dn7: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq2_e316: f64 = (p.p50 * var_ids);
        let eq2_e316_d_n0: f64 = (p.p50 * var_ids_dn0);
        let eq2_e316_d_n2: f64 = (p.p50 * var_ids_dn2);
        let eq2_e316_d_n6: f64 = (p.p50 * var_ids_dn6);
        let eq2_e316_d_n7: f64 = (p.p50 * var_ids_dn7);
        let eq2_e316_d_n10: f64 = (p.p50 * var_ids_dn10);
        let eq2_e316_d_n11: f64 = (p.p50 * var_ids_dn11);
        let eq2_e316_d_n12: f64 = (p.p50 * var_ids_dn12);
        let eq2_e316_d_n17: f64 = (p.p50 * var_ids_dn17);
        let eq2_value: f64 = eq2_e316;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq2_e316_d_n0), multiplicity * (eq2_e316_d_n2), multiplicity * (eq2_e316_d_n6), multiplicity * (eq2_e316_d_n7), multiplicity * (eq2_e316_d_n10), multiplicity * (eq2_e316_d_n11), multiplicity * (eq2_e316_d_n12), multiplicity * (eq2_e316_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq3_e322, eq3_e322_d_n0, eq3_e322_d_n2, eq3_e322_d_n6, eq3_e322_d_n7, eq3_e322_d_n10, eq3_e322_d_n11, eq3_e322_d_n12, eq3_e322_d_n17,) = {
    if (var_guard1222 != 0.0) {
        let eq3_e320: f64 = (p.p50 * var_igs);
        let eq3_e320_d_n0: f64 = (p.p50 * var_igs_dn0);
        let eq3_e320_d_n2: f64 = (p.p50 * var_igs_dn2);
        let eq3_e320_d_n6: f64 = (p.p50 * var_igs_dn6);
        let eq3_e320_d_n7: f64 = (p.p50 * var_igs_dn7);
        let eq3_e320_d_n10: f64 = (p.p50 * var_igs_dn10);
        let eq3_e320_d_n11: f64 = (p.p50 * var_igs_dn11);
        let eq3_e320_d_n12: f64 = (p.p50 * var_igs_dn12);
        let eq3_e320_d_n17: f64 = (p.p50 * var_igs_dn17);
        (eq3_e320, eq3_e320_d_n0, eq3_e320_d_n2, eq3_e320_d_n6, eq3_e320_d_n7, eq3_e320_d_n10, eq3_e320_d_n11, eq3_e320_d_n12, eq3_e320_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e322;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq3_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq3_e322_d_n0), multiplicity * (eq3_e322_d_n2), multiplicity * (eq3_e322_d_n6), multiplicity * (eq3_e322_d_n7), multiplicity * (eq3_e322_d_n10), multiplicity * (eq3_e322_d_n11), multiplicity * (eq3_e322_d_n12), multiplicity * (eq3_e322_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n2, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n17,) = {
    if (var_guard1222 != 0.0) {
        let eq4_e326: f64 = (p.p50 * var_igd);
        let eq4_e326_d_n0: f64 = (p.p50 * var_igd_dn0);
        let eq4_e326_d_n2: f64 = (p.p50 * var_igd_dn2);
        let eq4_e326_d_n6: f64 = (p.p50 * var_igd_dn6);
        let eq4_e326_d_n7: f64 = (p.p50 * var_igd_dn7);
        let eq4_e326_d_n10: f64 = (p.p50 * var_igd_dn10);
        let eq4_e326_d_n11: f64 = (p.p50 * var_igd_dn11);
        let eq4_e326_d_n12: f64 = (p.p50 * var_igd_dn12);
        let eq4_e326_d_n17: f64 = (p.p50 * var_igd_dn17);
        (eq4_e326, eq4_e326_d_n0, eq4_e326_d_n2, eq4_e326_d_n6, eq4_e326_d_n7, eq4_e326_d_n10, eq4_e326_d_n11, eq4_e326_d_n12, eq4_e326_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e328;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(6),
            multiplicity * (eq4_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq4_e328_d_n0), multiplicity * (eq4_e328_d_n2), multiplicity * (eq4_e328_d_n6), multiplicity * (eq4_e328_d_n7), multiplicity * (eq4_e328_d_n10), multiplicity * (eq4_e328_d_n11), multiplicity * (eq4_e328_d_n12), multiplicity * (eq4_e328_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n2, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n17,) = {
    if (var_guard1222 != 0.0) {
        let eq5_e332: f64 = (p.p50 * var_igb);
        let eq5_e332_d_n0: f64 = (p.p50 * var_igb_dn0);
        let eq5_e332_d_n2: f64 = (p.p50 * var_igb_dn2);
        let eq5_e332_d_n6: f64 = (p.p50 * var_igb_dn6);
        let eq5_e332_d_n7: f64 = (p.p50 * var_igb_dn7);
        let eq5_e332_d_n10: f64 = (p.p50 * var_igb_dn10);
        let eq5_e332_d_n11: f64 = (p.p50 * var_igb_dn11);
        let eq5_e332_d_n12: f64 = (p.p50 * var_igb_dn12);
        let eq5_e332_d_n17: f64 = (p.p50 * var_igb_dn17);
        (eq5_e332, eq5_e332_d_n0, eq5_e332_d_n2, eq5_e332_d_n6, eq5_e332_d_n7, eq5_e332_d_n10, eq5_e332_d_n11, eq5_e332_d_n12, eq5_e332_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e334;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq5_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq5_e334_d_n0), multiplicity * (eq5_e334_d_n2), multiplicity * (eq5_e334_d_n6), multiplicity * (eq5_e334_d_n7), multiplicity * (eq5_e334_d_n10), multiplicity * (eq5_e334_d_n11), multiplicity * (eq5_e334_d_n12), multiplicity * (eq5_e334_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n2, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n17,) = {
    if (p.p259 != 0.0) {
        let eq6_e338: f64 = ((nv7 - nv2) / var_rsd);
        let eq6_e338_d_n0: f64 = (-(((nv7 - nv2) * var_rsd_dn0) / (var_rsd * var_rsd)));
        let eq6_e338_d_n2: f64 = (((-var_rsd) - ((nv7 - nv2) * var_rsd_dn2)) / (var_rsd * var_rsd));
        let eq6_e338_d_n6: f64 = (-(((nv7 - nv2) * var_rsd_dn6) / (var_rsd * var_rsd)));
        let eq6_e338_d_n7: f64 = ((var_rsd - ((nv7 - nv2) * var_rsd_dn7)) / (var_rsd * var_rsd));
        let eq6_e338_d_n10: f64 = (-(((nv7 - nv2) * var_rsd_dn10) / (var_rsd * var_rsd)));
        let eq6_e338_d_n11: f64 = (-(((nv7 - nv2) * var_rsd_dn11) / (var_rsd * var_rsd)));
        let eq6_e338_d_n12: f64 = (-(((nv7 - nv2) * var_rsd_dn12) / (var_rsd * var_rsd)));
        let eq6_e338_d_n17: f64 = (-(((nv7 - nv2) * var_rsd_dn17) / (var_rsd * var_rsd)));
        (eq6_e338, eq6_e338_d_n0, eq6_e338_d_n2, eq6_e338_d_n6, eq6_e338_d_n7, eq6_e338_d_n10, eq6_e338_d_n11, eq6_e338_d_n12, eq6_e338_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e340;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(2),
            multiplicity * (eq6_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq6_e340_d_n0), multiplicity * (eq6_e340_d_n2), multiplicity * (eq6_e340_d_n6), multiplicity * (eq6_e340_d_n7), multiplicity * (eq6_e340_d_n10), multiplicity * (eq6_e340_d_n11), multiplicity * (eq6_e340_d_n12), multiplicity * (eq6_e340_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq8_e351, eq8_e351_d_n0, eq8_e351_d_n2, eq8_e351_d_n6, eq8_e351_d_n7, eq8_e351_d_n10, eq8_e351_d_n11, eq8_e351_d_n12, eq8_e351_d_n17,) = {
    if (p.p260 != 0.0) {
        let eq8_e349: f64 = ((nv0 - nv6) / var_rdd);
        let eq8_e349_d_n0: f64 = ((var_rdd - ((nv0 - nv6) * var_rdd_dn0)) / (var_rdd * var_rdd));
        let eq8_e349_d_n2: f64 = (-(((nv0 - nv6) * var_rdd_dn2) / (var_rdd * var_rdd)));
        let eq8_e349_d_n6: f64 = (((-var_rdd) - ((nv0 - nv6) * var_rdd_dn6)) / (var_rdd * var_rdd));
        let eq8_e349_d_n7: f64 = (-(((nv0 - nv6) * var_rdd_dn7) / (var_rdd * var_rdd)));
        let eq8_e349_d_n10: f64 = (-(((nv0 - nv6) * var_rdd_dn10) / (var_rdd * var_rdd)));
        let eq8_e349_d_n11: f64 = (-(((nv0 - nv6) * var_rdd_dn11) / (var_rdd * var_rdd)));
        let eq8_e349_d_n12: f64 = (-(((nv0 - nv6) * var_rdd_dn12) / (var_rdd * var_rdd)));
        let eq8_e349_d_n17: f64 = (-(((nv0 - nv6) * var_rdd_dn17) / (var_rdd * var_rdd)));
        (eq8_e349, eq8_e349_d_n0, eq8_e349_d_n2, eq8_e349_d_n6, eq8_e349_d_n7, eq8_e349_d_n10, eq8_e349_d_n11, eq8_e349_d_n12, eq8_e349_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e351;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(6),
            multiplicity * (eq8_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq8_e351_d_n0), multiplicity * (eq8_e351_d_n2), multiplicity * (eq8_e351_d_n6), multiplicity * (eq8_e351_d_n7), multiplicity * (eq8_e351_d_n10), multiplicity * (eq8_e351_d_n11), multiplicity * (eq8_e351_d_n12), multiplicity * (eq8_e351_d_n17)],
            [],
            [],
            1.0,
        );
        let eq10_e359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_qg);
        let eq10_e360: f64 = (p.p50 * eq10_e359);
        let eq10_e360_d_n0: f64 = (p.p50 * (var_qg_dn0 * ddt_scale));
        let eq10_e360_d_n2: f64 = (p.p50 * (var_qg_dn2 * ddt_scale));
        let eq10_e360_d_n6: f64 = (p.p50 * (var_qg_dn6 * ddt_scale));
        let eq10_e360_d_n7: f64 = (p.p50 * (var_qg_dn7 * ddt_scale));
        let eq10_e360_d_n10: f64 = (p.p50 * (var_qg_dn10 * ddt_scale));
        let eq10_e360_d_n11: f64 = (p.p50 * (var_qg_dn11 * ddt_scale));
        let eq10_e360_d_n12: f64 = (p.p50 * (var_qg_dn12 * ddt_scale));
        let eq10_e360_d_n13: f64 = (p.p50 * (var_qg_dn13 * ddt_scale));
        let eq10_e360_d_n15: f64 = (p.p50 * (var_qg_dn15 * ddt_scale));
        let eq10_e360_d_n16: f64 = (p.p50 * (var_qg_dn16 * ddt_scale));
        let eq10_e360_d_n17: f64 = (p.p50 * (var_qg_dn17 * ddt_scale));
        let eq10_e360_d_n18: f64 = (p.p50 * (var_qg_dn18 * ddt_scale));
        let eq10_value: f64 = eq10_e360;
        let eq10_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq10_node_derivatives: [f64; 12] = [eq10_e360_d_n0, eq10_e360_d_n2, eq10_e360_d_n6, eq10_e360_d_n7, eq10_e360_d_n10, eq10_e360_d_n11, eq10_e360_d_n12, eq10_e360_d_n13, eq10_e360_d_n15, eq10_e360_d_n16, eq10_e360_d_n17, eq10_e360_d_n18];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_qd);
        let eq11_e364: f64 = (p.p50 * eq11_e363);
        let eq11_e364_d_n0: f64 = (p.p50 * (var_qd_dn0 * ddt_scale));
        let eq11_e364_d_n2: f64 = (p.p50 * (var_qd_dn2 * ddt_scale));
        let eq11_e364_d_n6: f64 = (p.p50 * (var_qd_dn6 * ddt_scale));
        let eq11_e364_d_n7: f64 = (p.p50 * (var_qd_dn7 * ddt_scale));
        let eq11_e364_d_n10: f64 = (p.p50 * (var_qd_dn10 * ddt_scale));
        let eq11_e364_d_n11: f64 = (p.p50 * (var_qd_dn11 * ddt_scale));
        let eq11_e364_d_n12: f64 = (p.p50 * (var_qd_dn12 * ddt_scale));
        let eq11_e364_d_n13: f64 = (p.p50 * (var_qd_dn13 * ddt_scale));
        let eq11_e364_d_n15: f64 = (p.p50 * (var_qd_dn15 * ddt_scale));
        let eq11_e364_d_n16: f64 = (p.p50 * (var_qd_dn16 * ddt_scale));
        let eq11_e364_d_n17: f64 = (p.p50 * (var_qd_dn17 * ddt_scale));
        let eq11_e364_d_n18: f64 = (p.p50 * (var_qd_dn18 * ddt_scale));
        let eq11_value: f64 = eq11_e364;
        let eq11_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq11_node_derivatives: [f64; 12] = [eq11_e364_d_n0, eq11_e364_d_n2, eq11_e364_d_n6, eq11_e364_d_n7, eq11_e364_d_n10, eq11_e364_d_n11, eq11_e364_d_n12, eq11_e364_d_n13, eq11_e364_d_n15, eq11_e364_d_n16, eq11_e364_d_n17, eq11_e364_d_n18];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qb);
        let eq12_e368: f64 = (p.p50 * eq12_e367);
        let eq12_e368_d_n0: f64 = (p.p50 * (var_qb_dn0 * ddt_scale));
        let eq12_e368_d_n2: f64 = (p.p50 * (var_qb_dn2 * ddt_scale));
        let eq12_e368_d_n6: f64 = (p.p50 * (var_qb_dn6 * ddt_scale));
        let eq12_e368_d_n7: f64 = (p.p50 * (var_qb_dn7 * ddt_scale));
        let eq12_e368_d_n10: f64 = (p.p50 * (var_qb_dn10 * ddt_scale));
        let eq12_e368_d_n11: f64 = (p.p50 * (var_qb_dn11 * ddt_scale));
        let eq12_e368_d_n12: f64 = (p.p50 * (var_qb_dn12 * ddt_scale));
        let eq12_e368_d_n13: f64 = (p.p50 * (var_qb_dn13 * ddt_scale));
        let eq12_e368_d_n15: f64 = (p.p50 * (var_qb_dn15 * ddt_scale));
        let eq12_e368_d_n16: f64 = (p.p50 * (var_qb_dn16 * ddt_scale));
        let eq12_e368_d_n17: f64 = (p.p50 * (var_qb_dn17 * ddt_scale));
        let eq12_e368_d_n18: f64 = (p.p50 * (var_qb_dn18 * ddt_scale));
        let eq12_value: f64 = eq12_e368;
        let eq12_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq12_node_derivatives: [f64; 12] = [eq12_e368_d_n0, eq12_e368_d_n2, eq12_e368_d_n6, eq12_e368_d_n7, eq12_e368_d_n10, eq12_e368_d_n11, eq12_e368_d_n12, eq12_e368_d_n13, eq12_e368_d_n15, eq12_e368_d_n16, eq12_e368_d_n17, eq12_e368_d_n18];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq17_e394: f64 = (var_ci * (nv14 - 0.0));
        let eq17_e394_d_n0: f64 = (var_ci_dn0 * (nv14 - 0.0));
        let eq17_e394_d_n2: f64 = (var_ci_dn2 * (nv14 - 0.0));
        let eq17_e394_d_n6: f64 = (var_ci_dn6 * (nv14 - 0.0));
        let eq17_e394_d_n7: f64 = (var_ci_dn7 * (nv14 - 0.0));
        let eq17_e394_d_n10: f64 = (var_ci_dn10 * (nv14 - 0.0));
        let eq17_e394_d_n11: f64 = (var_ci_dn11 * (nv14 - 0.0));
        let eq17_e394_d_n12: f64 = (var_ci_dn12 * (nv14 - 0.0));
        let eq17_e394_d_n17: f64 = (var_ci_dn17 * (nv14 - 0.0));
        let eq17_value: f64 = eq17_e394;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            [0, 2, 6, 7, 10, 11, 12, 14, 17],
            [multiplicity * (eq17_e394_d_n0), multiplicity * (eq17_e394_d_n2), multiplicity * (eq17_e394_d_n6), multiplicity * (eq17_e394_d_n7), multiplicity * (eq17_e394_d_n10), multiplicity * (eq17_e394_d_n11), multiplicity * (eq17_e394_d_n12), multiplicity * (var_ci), multiplicity * (eq17_e394_d_n17)],
            [],
            [],
            1.0,
        );
        let eq18_e397: f64 = ((nv14 - 0.0) * var_sigrat_s);
        let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * var_sigrat_s_dn0);
        let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * var_sigrat_s_dn2);
        let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * var_sigrat_s_dn6);
        let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * var_sigrat_s_dn7);
        let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * var_sigrat_s_dn10);
        let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * var_sigrat_s_dn11);
        let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * var_sigrat_s_dn12);
        let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * var_sigrat_s_dn13);
        let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * var_sigrat_s_dn15);
        let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * var_sigrat_s_dn16);
        let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * var_sigrat_s_dn17);
        let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * var_sigrat_s_dn18);
        let eq18_e398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e397);
        let eq18_value: f64 = eq18_e398;
        let eq18_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq18_node_derivatives: [f64; 13] = [(eq18_e397_d_n0 * ddt_scale), (eq18_e397_d_n2 * ddt_scale), (eq18_e397_d_n6 * ddt_scale), (eq18_e397_d_n7 * ddt_scale), (eq18_e397_d_n10 * ddt_scale), (eq18_e397_d_n11 * ddt_scale), (eq18_e397_d_n12 * ddt_scale), (eq18_e397_d_n13 * ddt_scale), (var_sigrat_s * ddt_scale), (eq18_e397_d_n15 * ddt_scale), (eq18_e397_d_n16 * ddt_scale), (eq18_e397_d_n17 * ddt_scale), (eq18_e397_d_n18 * ddt_scale)];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e401: f64 = ((nv14 - 0.0) * var_sigrat_d);
        let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * var_sigrat_d_dn0);
        let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * var_sigrat_d_dn2);
        let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * var_sigrat_d_dn6);
        let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * var_sigrat_d_dn7);
        let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * var_sigrat_d_dn10);
        let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * var_sigrat_d_dn11);
        let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * var_sigrat_d_dn12);
        let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * var_sigrat_d_dn13);
        let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * var_sigrat_d_dn15);
        let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * var_sigrat_d_dn16);
        let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * var_sigrat_d_dn17);
        let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * var_sigrat_d_dn18);
        let eq19_e402: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e401);
        let eq19_value: f64 = eq19_e402;
        let eq19_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq19_node_derivatives: [f64; 13] = [(eq19_e401_d_n0 * ddt_scale), (eq19_e401_d_n2 * ddt_scale), (eq19_e401_d_n6 * ddt_scale), (eq19_e401_d_n7 * ddt_scale), (eq19_e401_d_n10 * ddt_scale), (eq19_e401_d_n11 * ddt_scale), (eq19_e401_d_n12 * ddt_scale), (eq19_e401_d_n13 * ddt_scale), (var_sigrat_d * ddt_scale), (eq19_e401_d_n15 * ddt_scale), (eq19_e401_d_n16 * ddt_scale), (eq19_e401_d_n17 * ddt_scale), (eq19_e401_d_n18 * ddt_scale)];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq25_e454, eq25_e454_d_n1, eq25_e454_d_n11,) = {
    if (p.p35 != 0.0) {
        let eq25_e452: f64 = (var_grg * (nv1 - nv11));
        (eq25_e452, var_grg, (-var_grg),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e454;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(11),
            multiplicity * (eq25_value),
            1,
            multiplicity * (eq25_e454_d_n1),
            11,
            multiplicity * (eq25_e454_d_n11),
        );
        let (eq27_e465, eq27_e465_d_n10,) = {
    if (var_guard1224 != 0.0) {
        let eq27_e463: f64 = ((nv10 - 0.0) * var_gth);
        (eq27_e463, var_gth,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e465;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq27_value),
            10,
            multiplicity * (eq27_e465_d_n10),
        );
        let (eq28_e470, eq28_e470_d_n0, eq28_e470_d_n2, eq28_e470_d_n6, eq28_e470_d_n7, eq28_e470_d_n10, eq28_e470_d_n11, eq28_e470_d_n12, eq28_e470_d_n17,) = {
    if (var_guard1224 != 0.0) {
        let eq28_e468: f64 = (-var_itemp);
        (eq28_e468, (-var_itemp_dn0), (-var_itemp_dn2), (-var_itemp_dn6), (-var_itemp_dn7), (-var_itemp_dn10), (-var_itemp_dn11), (-var_itemp_dn12), (-var_itemp_dn17),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e470;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(10),
            None,
            multiplicity * (eq28_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq28_e470_d_n0), multiplicity * (eq28_e470_d_n2), multiplicity * (eq28_e470_d_n6), multiplicity * (eq28_e470_d_n7), multiplicity * (eq28_e470_d_n10), multiplicity * (eq28_e470_d_n11), multiplicity * (eq28_e470_d_n12), multiplicity * (eq28_e470_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq30_e483, eq30_e483_d_n10,) = {
    if (var_guard1224 != 0.0) {
        let eq30_e480: f64 = (var_cthe * (nv10 - 0.0));
        let eq30_e481: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq30_e480);
        (eq30_e481, (var_cthe * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e483;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e483_d_n10),
        );
        let (eq32_e498, eq32_e498_d_n0, eq32_e498_d_n2, eq32_e498_d_n6, eq32_e498_d_n7, eq32_e498_d_n10, eq32_e498_d_n11, eq32_e498_d_n12, eq32_e498_d_n17,) = {
    if (var_guard1225 != 0.0) {
        let eq32_e495: f64 = (var_igidl + var_isub);
        let eq32_e495_d_n0: f64 = (var_igidl_dn0 + var_isub_dn0);
        let eq32_e495_d_n2: f64 = (var_igidl_dn2 + var_isub_dn2);
        let eq32_e495_d_n6: f64 = (var_igidl_dn6 + var_isub_dn6);
        let eq32_e495_d_n7: f64 = (var_igidl_dn7 + var_isub_dn7);
        let eq32_e495_d_n10: f64 = (var_igidl_dn10 + var_isub_dn10);
        let eq32_e495_d_n11: f64 = (var_igidl_dn11 + var_isub_dn11);
        let eq32_e495_d_n12: f64 = (var_igidl_dn12 + var_isub_dn12);
        let eq32_e495_d_n17: f64 = (var_igidl_dn17 + var_isub_dn17);
        let eq32_e496: f64 = (p.p50 * eq32_e495);
        let eq32_e496_d_n0: f64 = (p.p50 * eq32_e495_d_n0);
        let eq32_e496_d_n2: f64 = (p.p50 * eq32_e495_d_n2);
        let eq32_e496_d_n6: f64 = (p.p50 * eq32_e495_d_n6);
        let eq32_e496_d_n7: f64 = (p.p50 * eq32_e495_d_n7);
        let eq32_e496_d_n10: f64 = (p.p50 * eq32_e495_d_n10);
        let eq32_e496_d_n11: f64 = (p.p50 * eq32_e495_d_n11);
        let eq32_e496_d_n12: f64 = (p.p50 * eq32_e495_d_n12);
        let eq32_e496_d_n17: f64 = (p.p50 * eq32_e495_d_n17);
        (eq32_e496, eq32_e496_d_n0, eq32_e496_d_n2, eq32_e496_d_n6, eq32_e496_d_n7, eq32_e496_d_n10, eq32_e496_d_n11, eq32_e496_d_n12, eq32_e496_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e498;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq32_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq32_e498_d_n0), multiplicity * (eq32_e498_d_n2), multiplicity * (eq32_e498_d_n6), multiplicity * (eq32_e498_d_n7), multiplicity * (eq32_e498_d_n10), multiplicity * (eq32_e498_d_n11), multiplicity * (eq32_e498_d_n12), multiplicity * (eq32_e498_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq33_e506, eq33_e506_d_n0, eq33_e506_d_n2, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n17,) = {
    if (var_guard1225 != 0.0) {
        let eq33_e503: f64 = (var_igisl + var_isubs);
        let eq33_e503_d_n0: f64 = (var_igisl_dn0 + var_isubs_dn0);
        let eq33_e503_d_n2: f64 = (var_igisl_dn2 + var_isubs_dn2);
        let eq33_e503_d_n6: f64 = (var_igisl_dn6 + var_isubs_dn6);
        let eq33_e503_d_n7: f64 = (var_igisl_dn7 + var_isubs_dn7);
        let eq33_e503_d_n10: f64 = (var_igisl_dn10 + var_isubs_dn10);
        let eq33_e503_d_n11: f64 = (var_igisl_dn11 + var_isubs_dn11);
        let eq33_e503_d_n12: f64 = (var_igisl_dn12 + var_isubs_dn12);
        let eq33_e503_d_n17: f64 = (var_igisl_dn17 + var_isubs_dn17);
        let eq33_e504: f64 = (p.p50 * eq33_e503);
        let eq33_e504_d_n0: f64 = (p.p50 * eq33_e503_d_n0);
        let eq33_e504_d_n2: f64 = (p.p50 * eq33_e503_d_n2);
        let eq33_e504_d_n6: f64 = (p.p50 * eq33_e503_d_n6);
        let eq33_e504_d_n7: f64 = (p.p50 * eq33_e503_d_n7);
        let eq33_e504_d_n10: f64 = (p.p50 * eq33_e503_d_n10);
        let eq33_e504_d_n11: f64 = (p.p50 * eq33_e503_d_n11);
        let eq33_e504_d_n12: f64 = (p.p50 * eq33_e503_d_n12);
        let eq33_e504_d_n17: f64 = (p.p50 * eq33_e503_d_n17);
        (eq33_e504, eq33_e504_d_n0, eq33_e504_d_n2, eq33_e504_d_n6, eq33_e504_d_n7, eq33_e504_d_n10, eq33_e504_d_n11, eq33_e504_d_n12, eq33_e504_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e506;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq33_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq33_e506_d_n0), multiplicity * (eq33_e506_d_n2), multiplicity * (eq33_e506_d_n6), multiplicity * (eq33_e506_d_n7), multiplicity * (eq33_e506_d_n10), multiplicity * (eq33_e506_d_n11), multiplicity * (eq33_e506_d_n12), multiplicity * (eq33_e506_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n2, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n17,) = {
    if (var_guard1225 != 0.0) {
        let eq34_e511: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qbs);
        let eq34_e512: f64 = (var_ibs + eq34_e511);
        let eq34_e512_d_n0: f64 = (var_ibs_dn0 + (var_qbs_dn0 * ddt_scale));
        let eq34_e512_d_n2: f64 = (var_ibs_dn2 + (var_qbs_dn2 * ddt_scale));
        let eq34_e512_d_n6: f64 = (var_ibs_dn6 + (var_qbs_dn6 * ddt_scale));
        let eq34_e512_d_n7: f64 = (var_ibs_dn7 + (var_qbs_dn7 * ddt_scale));
        let eq34_e512_d_n10: f64 = (var_ibs_dn10 + (var_qbs_dn10 * ddt_scale));
        let eq34_e512_d_n11: f64 = (var_ibs_dn11 + (var_qbs_dn11 * ddt_scale));
        let eq34_e512_d_n12: f64 = (var_ibs_dn12 + (var_qbs_dn12 * ddt_scale));
        let eq34_e512_d_n17: f64 = (var_ibs_dn17 + (var_qbs_dn17 * ddt_scale));
        let eq34_e513: f64 = (p.p50 * eq34_e512);
        let eq34_e513_d_n0: f64 = (p.p50 * eq34_e512_d_n0);
        let eq34_e513_d_n2: f64 = (p.p50 * eq34_e512_d_n2);
        let eq34_e513_d_n6: f64 = (p.p50 * eq34_e512_d_n6);
        let eq34_e513_d_n7: f64 = (p.p50 * eq34_e512_d_n7);
        let eq34_e513_d_n10: f64 = (p.p50 * eq34_e512_d_n10);
        let eq34_e513_d_n11: f64 = (p.p50 * eq34_e512_d_n11);
        let eq34_e513_d_n12: f64 = (p.p50 * eq34_e512_d_n12);
        let eq34_e513_d_n17: f64 = (p.p50 * eq34_e512_d_n17);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n2, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e515;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq34_e515_d_n0), multiplicity * (eq34_e515_d_n2), multiplicity * (eq34_e515_d_n6), multiplicity * (eq34_e515_d_n7), multiplicity * (eq34_e515_d_n10), multiplicity * (eq34_e515_d_n11), multiplicity * (eq34_e515_d_n12), multiplicity * (eq34_e515_d_n17)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
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
        var_guard1225: f64,
        var_guard1226: f64,
        var_ibd: f64,
        var_ibd_dn0: f64,
        var_ibd_dn10: f64,
        var_ibd_dn11: f64,
        var_ibd_dn12: f64,
        var_ibd_dn17: f64,
        var_ibd_dn2: f64,
        var_ibd_dn6: f64,
        var_ibd_dn7: f64,
        var_igidl: f64,
        var_igidl_dn0: f64,
        var_igidl_dn10: f64,
        var_igidl_dn11: f64,
        var_igidl_dn12: f64,
        var_igidl_dn17: f64,
        var_igidl_dn2: f64,
        var_igidl_dn6: f64,
        var_igidl_dn7: f64,
        var_igisl: f64,
        var_igisl_dn0: f64,
        var_igisl_dn10: f64,
        var_igisl_dn11: f64,
        var_igisl_dn12: f64,
        var_igisl_dn17: f64,
        var_igisl_dn2: f64,
        var_igisl_dn6: f64,
        var_igisl_dn7: f64,
        var_iqb_nqs: f64,
        var_iqb_nqs_dn0: f64,
        var_iqb_nqs_dn10: f64,
        var_iqb_nqs_dn11: f64,
        var_iqb_nqs_dn12: f64,
        var_iqb_nqs_dn13: f64,
        var_iqb_nqs_dn15: f64,
        var_iqb_nqs_dn16: f64,
        var_iqb_nqs_dn17: f64,
        var_iqb_nqs_dn18: f64,
        var_iqb_nqs_dn2: f64,
        var_iqb_nqs_dn6: f64,
        var_iqb_nqs_dn7: f64,
        var_iqd_nqs: f64,
        var_iqd_nqs_dn0: f64,
        var_iqd_nqs_dn10: f64,
        var_iqd_nqs_dn11: f64,
        var_iqd_nqs_dn12: f64,
        var_iqd_nqs_dn13: f64,
        var_iqd_nqs_dn15: f64,
        var_iqd_nqs_dn16: f64,
        var_iqd_nqs_dn17: f64,
        var_iqd_nqs_dn18: f64,
        var_iqd_nqs_dn2: f64,
        var_iqd_nqs_dn6: f64,
        var_iqd_nqs_dn7: f64,
        var_iqh_nqs: f64,
        var_iqh_nqs_dn0: f64,
        var_iqh_nqs_dn10: f64,
        var_iqh_nqs_dn11: f64,
        var_iqh_nqs_dn12: f64,
        var_iqh_nqs_dn17: f64,
        var_iqh_nqs_dn2: f64,
        var_iqh_nqs_dn6: f64,
        var_iqh_nqs_dn7: f64,
        var_iqi_nqs: f64,
        var_iqi_nqs_dn0: f64,
        var_iqi_nqs_dn10: f64,
        var_iqi_nqs_dn11: f64,
        var_iqi_nqs_dn12: f64,
        var_iqi_nqs_dn17: f64,
        var_iqi_nqs_dn18: f64,
        var_iqi_nqs_dn2: f64,
        var_iqi_nqs_dn6: f64,
        var_iqi_nqs_dn7: f64,
        var_iqs_nqs: f64,
        var_iqs_nqs_dn0: f64,
        var_iqs_nqs_dn10: f64,
        var_iqs_nqs_dn11: f64,
        var_iqs_nqs_dn12: f64,
        var_iqs_nqs_dn13: f64,
        var_iqs_nqs_dn15: f64,
        var_iqs_nqs_dn16: f64,
        var_iqs_nqs_dn17: f64,
        var_iqs_nqs_dn18: f64,
        var_iqs_nqs_dn2: f64,
        var_iqs_nqs_dn6: f64,
        var_iqs_nqs_dn7: f64,
        var_isub: f64,
        var_isub_dn0: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn17: f64,
        var_isub_dn2: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isubs: f64,
        var_isubs_dn0: f64,
        var_isubs_dn10: f64,
        var_isubs_dn11: f64,
        var_isubs_dn12: f64,
        var_isubs_dn17: f64,
        var_isubs_dn2: f64,
        var_isubs_dn6: f64,
        var_isubs_dn7: f64,
        var_qbd: f64,
        var_qbd_dn0: f64,
        var_qbd_dn10: f64,
        var_qbd_dn11: f64,
        var_qbd_dn12: f64,
        var_qbd_dn17: f64,
        var_qbd_dn2: f64,
        var_qbd_dn6: f64,
        var_qbd_dn7: f64,
        var_rbulk: f64,
        var_rbulk_dn0: f64,
        var_rbulk_dn10: f64,
        var_rbulk_dn11: f64,
        var_rbulk_dn12: f64,
        var_rbulk_dn17: f64,
        var_rbulk_dn2: f64,
        var_rbulk_dn6: f64,
        var_rbulk_dn7: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n2, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n17,) = {
    if (var_guard1225 != 0.0) {
        let eq35_e520: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, var_qbd);
        let eq35_e521: f64 = (var_ibd + eq35_e520);
        let eq35_e521_d_n0: f64 = (var_ibd_dn0 + (var_qbd_dn0 * ddt_scale));
        let eq35_e521_d_n2: f64 = (var_ibd_dn2 + (var_qbd_dn2 * ddt_scale));
        let eq35_e521_d_n6: f64 = (var_ibd_dn6 + (var_qbd_dn6 * ddt_scale));
        let eq35_e521_d_n7: f64 = (var_ibd_dn7 + (var_qbd_dn7 * ddt_scale));
        let eq35_e521_d_n10: f64 = (var_ibd_dn10 + (var_qbd_dn10 * ddt_scale));
        let eq35_e521_d_n11: f64 = (var_ibd_dn11 + (var_qbd_dn11 * ddt_scale));
        let eq35_e521_d_n12: f64 = (var_ibd_dn12 + (var_qbd_dn12 * ddt_scale));
        let eq35_e521_d_n17: f64 = (var_ibd_dn17 + (var_qbd_dn17 * ddt_scale));
        let eq35_e522: f64 = (p.p50 * eq35_e521);
        let eq35_e522_d_n0: f64 = (p.p50 * eq35_e521_d_n0);
        let eq35_e522_d_n2: f64 = (p.p50 * eq35_e521_d_n2);
        let eq35_e522_d_n6: f64 = (p.p50 * eq35_e521_d_n6);
        let eq35_e522_d_n7: f64 = (p.p50 * eq35_e521_d_n7);
        let eq35_e522_d_n10: f64 = (p.p50 * eq35_e521_d_n10);
        let eq35_e522_d_n11: f64 = (p.p50 * eq35_e521_d_n11);
        let eq35_e522_d_n12: f64 = (p.p50 * eq35_e521_d_n12);
        let eq35_e522_d_n17: f64 = (p.p50 * eq35_e521_d_n17);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n2, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e524;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq35_e524_d_n0), multiplicity * (eq35_e524_d_n2), multiplicity * (eq35_e524_d_n6), multiplicity * (eq35_e524_d_n7), multiplicity * (eq35_e524_d_n10), multiplicity * (eq35_e524_d_n11), multiplicity * (eq35_e524_d_n12), multiplicity * (eq35_e524_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n2, eq36_e532_d_n4, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n17,) = {
    if ((var_guard1225 != 0.0) && (p.p261 != 0.0)) {
        let eq36_e530: f64 = ((nv4 - nv12) / var_rbulk);
        let eq36_e530_d_n0: f64 = (-(((nv4 - nv12) * var_rbulk_dn0) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n2: f64 = (-(((nv4 - nv12) * var_rbulk_dn2) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n4: f64 = (1.0 / var_rbulk);
        let eq36_e530_d_n6: f64 = (-(((nv4 - nv12) * var_rbulk_dn6) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n7: f64 = (-(((nv4 - nv12) * var_rbulk_dn7) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n10: f64 = (-(((nv4 - nv12) * var_rbulk_dn10) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n11: f64 = (-(((nv4 - nv12) * var_rbulk_dn11) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n12: f64 = (((-var_rbulk) - ((nv4 - nv12) * var_rbulk_dn12)) / (var_rbulk * var_rbulk));
        let eq36_e530_d_n17: f64 = (-(((nv4 - nv12) * var_rbulk_dn17) / (var_rbulk * var_rbulk)));
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n2, eq36_e530_d_n4, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e532;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(12),
            multiplicity * (eq36_value),
            [0, 2, 4, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq36_e532_d_n0), multiplicity * (eq36_e532_d_n2), multiplicity * (eq36_e532_d_n4), multiplicity * (eq36_e532_d_n6), multiplicity * (eq36_e532_d_n7), multiplicity * (eq36_e532_d_n10), multiplicity * (eq36_e532_d_n11), multiplicity * (eq36_e532_d_n12), multiplicity * (eq36_e532_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq42_e575, eq42_e575_d_n0, eq42_e575_d_n2, eq42_e575_d_n6, eq42_e575_d_n7, eq42_e575_d_n10, eq42_e575_d_n11, eq42_e575_d_n12, eq42_e575_d_n17, eq42_e575_d_n18,) = {
    if ((var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_dn17, var_iqi_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e575;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(18),
            None,
            multiplicity * (eq42_value),
            [0, 2, 6, 7, 10, 11, 12, 17, 18],
            [multiplicity * (eq42_e575_d_n0), multiplicity * (eq42_e575_d_n2), multiplicity * (eq42_e575_d_n6), multiplicity * (eq42_e575_d_n7), multiplicity * (eq42_e575_d_n10), multiplicity * (eq42_e575_d_n11), multiplicity * (eq42_e575_d_n12), multiplicity * (eq42_e575_d_n17), multiplicity * (eq42_e575_d_n18)],
            [],
            [],
            1.0,
        );
        let (eq43_e581, eq43_e581_d_n0, eq43_e581_d_n2, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18,) = {
    if ((var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e581;
        let eq43_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq43_node_derivatives: [f64; 12] = [eq43_e581_d_n0, eq43_e581_d_n2, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq46_e608, eq46_e608_d_n18,) = {
    if ((var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq46_e605);
        (eq46_e606, (eq46_e603 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e608;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq46_value),
            18,
            multiplicity * (eq46_e608_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13,) = {
    if ((var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq47_e616);
        (eq47_e617, (eq47_e614 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e619;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq47_value),
            13,
            multiplicity * (eq47_e619_d_n13),
        );
        let (eq50_e639, eq50_e639_d_n0, eq50_e639_d_n2, eq50_e639_d_n6, eq50_e639_d_n7, eq50_e639_d_n10, eq50_e639_d_n11, eq50_e639_d_n12, eq50_e639_d_n17,) = {
    if ((var_guard1225 != 0.0) && (var_guard1226 != 0.0)) {
        (var_iqh_nqs, var_iqh_nqs_dn0, var_iqh_nqs_dn2, var_iqh_nqs_dn6, var_iqh_nqs_dn7, var_iqh_nqs_dn10, var_iqh_nqs_dn11, var_iqh_nqs_dn12, var_iqh_nqs_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e639;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(17),
            None,
            multiplicity * (eq50_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq50_e639_d_n0), multiplicity * (eq50_e639_d_n2), multiplicity * (eq50_e639_d_n6), multiplicity * (eq50_e639_d_n7), multiplicity * (eq50_e639_d_n10), multiplicity * (eq50_e639_d_n11), multiplicity * (eq50_e639_d_n12), multiplicity * (eq50_e639_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq52_e658, eq52_e658_d_n17,) = {
    if ((var_guard1225 != 0.0) && (var_guard1226 != 0.0)) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e655);
        (eq52_e656, (eq52_e653 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e658;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq52_value),
            17,
            multiplicity * (eq52_e658_d_n17),
        );
        let (eq54_e674, eq54_e674_d_n0, eq54_e674_d_n2, eq54_e674_d_n6, eq54_e674_d_n7, eq54_e674_d_n10, eq54_e674_d_n11, eq54_e674_d_n12, eq54_e674_d_n17,) = {
    if (var_guard1225 == 0.0) {
        let eq54_e671: f64 = (var_igidl + var_isub);
        let eq54_e671_d_n0: f64 = (var_igidl_dn0 + var_isub_dn0);
        let eq54_e671_d_n2: f64 = (var_igidl_dn2 + var_isub_dn2);
        let eq54_e671_d_n6: f64 = (var_igidl_dn6 + var_isub_dn6);
        let eq54_e671_d_n7: f64 = (var_igidl_dn7 + var_isub_dn7);
        let eq54_e671_d_n10: f64 = (var_igidl_dn10 + var_isub_dn10);
        let eq54_e671_d_n11: f64 = (var_igidl_dn11 + var_isub_dn11);
        let eq54_e671_d_n12: f64 = (var_igidl_dn12 + var_isub_dn12);
        let eq54_e671_d_n17: f64 = (var_igidl_dn17 + var_isub_dn17);
        let eq54_e672: f64 = (p.p50 * eq54_e671);
        let eq54_e672_d_n0: f64 = (p.p50 * eq54_e671_d_n0);
        let eq54_e672_d_n2: f64 = (p.p50 * eq54_e671_d_n2);
        let eq54_e672_d_n6: f64 = (p.p50 * eq54_e671_d_n6);
        let eq54_e672_d_n7: f64 = (p.p50 * eq54_e671_d_n7);
        let eq54_e672_d_n10: f64 = (p.p50 * eq54_e671_d_n10);
        let eq54_e672_d_n11: f64 = (p.p50 * eq54_e671_d_n11);
        let eq54_e672_d_n12: f64 = (p.p50 * eq54_e671_d_n12);
        let eq54_e672_d_n17: f64 = (p.p50 * eq54_e671_d_n17);
        (eq54_e672, eq54_e672_d_n0, eq54_e672_d_n2, eq54_e672_d_n6, eq54_e672_d_n7, eq54_e672_d_n10, eq54_e672_d_n11, eq54_e672_d_n12, eq54_e672_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e674;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq54_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq54_e674_d_n0), multiplicity * (eq54_e674_d_n2), multiplicity * (eq54_e674_d_n6), multiplicity * (eq54_e674_d_n7), multiplicity * (eq54_e674_d_n10), multiplicity * (eq54_e674_d_n11), multiplicity * (eq54_e674_d_n12), multiplicity * (eq54_e674_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq55_e683, eq55_e683_d_n0, eq55_e683_d_n2, eq55_e683_d_n6, eq55_e683_d_n7, eq55_e683_d_n10, eq55_e683_d_n11, eq55_e683_d_n12, eq55_e683_d_n17,) = {
    if (var_guard1225 == 0.0) {
        let eq55_e680: f64 = (var_igisl + var_isubs);
        let eq55_e680_d_n0: f64 = (var_igisl_dn0 + var_isubs_dn0);
        let eq55_e680_d_n2: f64 = (var_igisl_dn2 + var_isubs_dn2);
        let eq55_e680_d_n6: f64 = (var_igisl_dn6 + var_isubs_dn6);
        let eq55_e680_d_n7: f64 = (var_igisl_dn7 + var_isubs_dn7);
        let eq55_e680_d_n10: f64 = (var_igisl_dn10 + var_isubs_dn10);
        let eq55_e680_d_n11: f64 = (var_igisl_dn11 + var_isubs_dn11);
        let eq55_e680_d_n12: f64 = (var_igisl_dn12 + var_isubs_dn12);
        let eq55_e680_d_n17: f64 = (var_igisl_dn17 + var_isubs_dn17);
        let eq55_e681: f64 = (p.p50 * eq55_e680);
        let eq55_e681_d_n0: f64 = (p.p50 * eq55_e680_d_n0);
        let eq55_e681_d_n2: f64 = (p.p50 * eq55_e680_d_n2);
        let eq55_e681_d_n6: f64 = (p.p50 * eq55_e680_d_n6);
        let eq55_e681_d_n7: f64 = (p.p50 * eq55_e680_d_n7);
        let eq55_e681_d_n10: f64 = (p.p50 * eq55_e680_d_n10);
        let eq55_e681_d_n11: f64 = (p.p50 * eq55_e680_d_n11);
        let eq55_e681_d_n12: f64 = (p.p50 * eq55_e680_d_n12);
        let eq55_e681_d_n17: f64 = (p.p50 * eq55_e680_d_n17);
        (eq55_e681, eq55_e681_d_n0, eq55_e681_d_n2, eq55_e681_d_n6, eq55_e681_d_n7, eq55_e681_d_n10, eq55_e681_d_n11, eq55_e681_d_n12, eq55_e681_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e683;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq55_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq55_e683_d_n0), multiplicity * (eq55_e683_d_n2), multiplicity * (eq55_e683_d_n6), multiplicity * (eq55_e683_d_n7), multiplicity * (eq55_e683_d_n10), multiplicity * (eq55_e683_d_n11), multiplicity * (eq55_e683_d_n12), multiplicity * (eq55_e683_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq57_e695, eq57_e695_d_n0, eq57_e695_d_n2, eq57_e695_d_n6, eq57_e695_d_n7, eq57_e695_d_n10, eq57_e695_d_n11, eq57_e695_d_n12, eq57_e695_d_n17,) = {
    if ((var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        (var_iqh_nqs, var_iqh_nqs_dn0, var_iqh_nqs_dn2, var_iqh_nqs_dn6, var_iqh_nqs_dn7, var_iqh_nqs_dn10, var_iqh_nqs_dn11, var_iqh_nqs_dn12, var_iqh_nqs_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e695;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(17),
            None,
            multiplicity * (eq57_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq57_e695_d_n0), multiplicity * (eq57_e695_d_n2), multiplicity * (eq57_e695_d_n6), multiplicity * (eq57_e695_d_n7), multiplicity * (eq57_e695_d_n10), multiplicity * (eq57_e695_d_n11), multiplicity * (eq57_e695_d_n12), multiplicity * (eq57_e695_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq59_e716, eq59_e716_d_n17,) = {
    if ((var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq59_e713);
        (eq59_e714, (eq59_e711 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e716;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq59_value),
            17,
            multiplicity * (eq59_e716_d_n17),
        );
        let (eq61_e731, eq61_e731_d_n0, eq61_e731_d_n2, eq61_e731_d_n6, eq61_e731_d_n7, eq61_e731_d_n10, eq61_e731_d_n11, eq61_e731_d_n12, eq61_e731_d_n13, eq61_e731_d_n15, eq61_e731_d_n16, eq61_e731_d_n17, eq61_e731_d_n18,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        (var_iqd_nqs, var_iqd_nqs_dn0, var_iqd_nqs_dn2, var_iqd_nqs_dn6, var_iqd_nqs_dn7, var_iqd_nqs_dn10, var_iqd_nqs_dn11, var_iqd_nqs_dn12, var_iqd_nqs_dn13, var_iqd_nqs_dn15, var_iqd_nqs_dn16, var_iqd_nqs_dn17, var_iqd_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e731;
        let eq61_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq61_node_derivatives: [f64; 12] = [eq61_e731_d_n0, eq61_e731_d_n2, eq61_e731_d_n6, eq61_e731_d_n7, eq61_e731_d_n10, eq61_e731_d_n11, eq61_e731_d_n12, eq61_e731_d_n13, eq61_e731_d_n15, eq61_e731_d_n16, eq61_e731_d_n17, eq61_e731_d_n18];
        let eq61_branch_derivative_indices: [usize; 0] = [];
        let eq61_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivative_indices,
            &eq61_node_derivatives,
            &eq61_branch_derivative_indices,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e738, eq62_e738_d_n0, eq62_e738_d_n2, eq62_e738_d_n6, eq62_e738_d_n7, eq62_e738_d_n10, eq62_e738_d_n11, eq62_e738_d_n12, eq62_e738_d_n13, eq62_e738_d_n15, eq62_e738_d_n16, eq62_e738_d_n17, eq62_e738_d_n18,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        (var_iqs_nqs, var_iqs_nqs_dn0, var_iqs_nqs_dn2, var_iqs_nqs_dn6, var_iqs_nqs_dn7, var_iqs_nqs_dn10, var_iqs_nqs_dn11, var_iqs_nqs_dn12, var_iqs_nqs_dn13, var_iqs_nqs_dn15, var_iqs_nqs_dn16, var_iqs_nqs_dn17, var_iqs_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e738;
        let eq62_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq62_node_derivatives: [f64; 12] = [eq62_e738_d_n0, eq62_e738_d_n2, eq62_e738_d_n6, eq62_e738_d_n7, eq62_e738_d_n10, eq62_e738_d_n11, eq62_e738_d_n12, eq62_e738_d_n13, eq62_e738_d_n15, eq62_e738_d_n16, eq62_e738_d_n17, eq62_e738_d_n18];
        let eq62_branch_derivative_indices: [usize; 0] = [];
        let eq62_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(16),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivative_indices,
            &eq62_node_derivatives,
            &eq62_branch_derivative_indices,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e745, eq63_e745_d_n0, eq63_e745_d_n2, eq63_e745_d_n6, eq63_e745_d_n7, eq63_e745_d_n10, eq63_e745_d_n11, eq63_e745_d_n12, eq63_e745_d_n13, eq63_e745_d_n15, eq63_e745_d_n16, eq63_e745_d_n17, eq63_e745_d_n18,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e745;
        let eq63_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq63_node_derivatives: [f64; 12] = [eq63_e745_d_n0, eq63_e745_d_n2, eq63_e745_d_n6, eq63_e745_d_n7, eq63_e745_d_n10, eq63_e745_d_n11, eq63_e745_d_n12, eq63_e745_d_n13, eq63_e745_d_n15, eq63_e745_d_n16, eq63_e745_d_n17, eq63_e745_d_n18];
        let eq63_branch_derivative_indices: [usize; 0] = [];
        let eq63_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivative_indices,
            &eq63_node_derivatives,
            &eq63_branch_derivative_indices,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq67_e784, eq67_e784_d_n15,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq67_e781);
        (eq67_e782, (eq67_e779 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e784;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq67_value),
            15,
            multiplicity * (eq67_e784_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq68_e793);
        (eq68_e794, (eq68_e791 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e796;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq68_value),
            16,
            multiplicity * (eq68_e796_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq69_e805);
        (eq69_e806, (eq69_e803 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e808;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq69_value),
            13,
            multiplicity * (eq69_e808_d_n13),
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let eq10_e359_q: f64 = s.v[594];
        let eq10_e360: f64 = (p.p50 * s.v[594]);
        let eq10_e360_q: f64 = (p.p50 * eq10_e359_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &s.dn[594],
            branches,
            &s.db[594],
            (multiplicity) * (p.p50),
        );
        let eq11_e363_q: f64 = s.v[198];
        let eq11_e364: f64 = (p.p50 * s.v[198]);
        let eq11_e364_q: f64 = (p.p50 * eq11_e363_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &s.dn[198],
            branches,
            &s.db[198],
            (multiplicity) * (p.p50),
        );
        let eq12_e367_q: f64 = s.v[196];
        let eq12_e368: f64 = (p.p50 * s.v[196]);
        let eq12_e368_q: f64 = (p.p50 * eq12_e367_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &s.dn[196],
            branches,
            &s.db[196],
            (multiplicity) * (p.p50),
        );
        let eq18_e397: f64 = ((nv14 - 0.0) * s.v[617]);
        let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);
        let eq18_e397_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);
        let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);
        let eq18_e397_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);
        let eq18_e397_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);
        let eq18_e397_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);
        let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);
        let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);
        let eq18_e397_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);
        let eq18_e397_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);
        let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);
        let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);
        let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);
        let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);
        let eq18_e397_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));
        let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);
        let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);
        let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);
        let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);
        let eq18_e397_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);
        let eq18_e397_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);
        let eq18_e397_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);
        let eq18_e397_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);
        let eq18_e397_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);
        let eq18_e397_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);
        let eq18_e397_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);
        let eq18_e397_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);
        let eq18_e397_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);
        let eq18_e397_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);
        let eq18_e397_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);
        let eq18_e397_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);
        let eq18_e397_d_b12: f64 = ((nv14 - 0.0) * s.db[617][12]);
        let eq18_e397_d_b13: f64 = ((nv14 - 0.0) * s.db[617][13]);
        let eq18_e397_d_b14: f64 = ((nv14 - 0.0) * s.db[617][14]);
        let eq18_e398_q: f64 = eq18_e397;
        let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e397_d_n0, eq18_e397_d_n1, eq18_e397_d_n2, eq18_e397_d_n3, eq18_e397_d_n4, eq18_e397_d_n5, eq18_e397_d_n6, eq18_e397_d_n7, eq18_e397_d_n8, eq18_e397_d_n9, eq18_e397_d_n10, eq18_e397_d_n11, eq18_e397_d_n12, eq18_e397_d_n13, eq18_e397_d_n14, eq18_e397_d_n15, eq18_e397_d_n16, eq18_e397_d_n17, eq18_e397_d_n18];
        let eq18_reactive_branch_derivatives: [f64; 15] = [eq18_e397_d_b0, eq18_e397_d_b1, eq18_e397_d_b2, eq18_e397_d_b3, eq18_e397_d_b4, eq18_e397_d_b5, eq18_e397_d_b6, eq18_e397_d_b7, eq18_e397_d_b8, eq18_e397_d_b9, eq18_e397_d_b10, eq18_e397_d_b11, eq18_e397_d_b12, eq18_e397_d_b13, eq18_e397_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e401: f64 = ((nv14 - 0.0) * s.v[618]);
        let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);
        let eq19_e401_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);
        let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);
        let eq19_e401_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);
        let eq19_e401_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);
        let eq19_e401_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);
        let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);
        let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);
        let eq19_e401_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);
        let eq19_e401_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);
        let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);
        let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);
        let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);
        let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);
        let eq19_e401_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));
        let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);
        let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);
        let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);
        let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);
        let eq19_e401_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);
        let eq19_e401_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);
        let eq19_e401_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);
        let eq19_e401_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);
        let eq19_e401_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);
        let eq19_e401_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);
        let eq19_e401_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);
        let eq19_e401_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);
        let eq19_e401_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);
        let eq19_e401_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);
        let eq19_e401_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);
        let eq19_e401_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);
        let eq19_e401_d_b12: f64 = ((nv14 - 0.0) * s.db[618][12]);
        let eq19_e401_d_b13: f64 = ((nv14 - 0.0) * s.db[618][13]);
        let eq19_e401_d_b14: f64 = ((nv14 - 0.0) * s.db[618][14]);
        let eq19_e402_q: f64 = eq19_e401;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e401_d_n0, eq19_e401_d_n1, eq19_e401_d_n2, eq19_e401_d_n3, eq19_e401_d_n4, eq19_e401_d_n5, eq19_e401_d_n6, eq19_e401_d_n7, eq19_e401_d_n8, eq19_e401_d_n9, eq19_e401_d_n10, eq19_e401_d_n11, eq19_e401_d_n12, eq19_e401_d_n13, eq19_e401_d_n14, eq19_e401_d_n15, eq19_e401_d_n16, eq19_e401_d_n17, eq19_e401_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 15] = [eq19_e401_d_b0, eq19_e401_d_b1, eq19_e401_d_b2, eq19_e401_d_b3, eq19_e401_d_b4, eq19_e401_d_b5, eq19_e401_d_b6, eq19_e401_d_b7, eq19_e401_d_b8, eq19_e401_d_b9, eq19_e401_d_b10, eq19_e401_d_b11, eq19_e401_d_b12, eq19_e401_d_b13, eq19_e401_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e483, eq30_e483_d_n0, eq30_e483_d_n1, eq30_e483_d_n2, eq30_e483_d_n3, eq30_e483_d_n4, eq30_e483_d_n5, eq30_e483_d_n6, eq30_e483_d_n7, eq30_e483_d_n8, eq30_e483_d_n9, eq30_e483_d_n10, eq30_e483_d_n11, eq30_e483_d_n12, eq30_e483_d_n13, eq30_e483_d_n14, eq30_e483_d_n15, eq30_e483_d_n16, eq30_e483_d_n17, eq30_e483_d_n18, eq30_e483_d_b0, eq30_e483_d_b1, eq30_e483_d_b2, eq30_e483_d_b3, eq30_e483_d_b4, eq30_e483_d_b5, eq30_e483_d_b6, eq30_e483_d_b7, eq30_e483_d_b8, eq30_e483_d_b9, eq30_e483_d_b10, eq30_e483_d_b11, eq30_e483_d_b12, eq30_e483_d_b13, eq30_e483_d_b14, eq30_e483_q,) = {
    if s.b[1848] {
        let eq30_e480: f64 = (s.v[563] * (nv10 - 0.0));
        let eq30_e480_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));
        let eq30_e480_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));
        let eq30_e480_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));
        let eq30_e480_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));
        let eq30_e480_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));
        let eq30_e480_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));
        let eq30_e480_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));
        let eq30_e480_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));
        let eq30_e480_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));
        let eq30_e480_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));
        let eq30_e480_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);
        let eq30_e480_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));
        let eq30_e480_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));
        let eq30_e480_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));
        let eq30_e480_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));
        let eq30_e480_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));
        let eq30_e480_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));
        let eq30_e480_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));
        let eq30_e480_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));
        let eq30_e480_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));
        let eq30_e480_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));
        let eq30_e480_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));
        let eq30_e480_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));
        let eq30_e480_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));
        let eq30_e480_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));
        let eq30_e480_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));
        let eq30_e480_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));
        let eq30_e480_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));
        let eq30_e480_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));
        let eq30_e480_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));
        let eq30_e480_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));
        let eq30_e480_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));
        let eq30_e480_d_b13: f64 = (s.db[563][13] * (nv10 - 0.0));
        let eq30_e480_d_b14: f64 = (s.db[563][14] * (nv10 - 0.0));
        let eq30_e481_q: f64 = eq30_e480;
        (eq30_e480, eq30_e480_d_n0, eq30_e480_d_n1, eq30_e480_d_n2, eq30_e480_d_n3, eq30_e480_d_n4, eq30_e480_d_n5, eq30_e480_d_n6, eq30_e480_d_n7, eq30_e480_d_n8, eq30_e480_d_n9, eq30_e480_d_n10, eq30_e480_d_n11, eq30_e480_d_n12, eq30_e480_d_n13, eq30_e480_d_n14, eq30_e480_d_n15, eq30_e480_d_n16, eq30_e480_d_n17, eq30_e480_d_n18, eq30_e480_d_b0, eq30_e480_d_b1, eq30_e480_d_b2, eq30_e480_d_b3, eq30_e480_d_b4, eq30_e480_d_b5, eq30_e480_d_b6, eq30_e480_d_b7, eq30_e480_d_b8, eq30_e480_d_b9, eq30_e480_d_b10, eq30_e480_d_b11, eq30_e480_d_b12, eq30_e480_d_b13, eq30_e480_d_b14, eq30_e481_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_reactive_node_derivatives: [f64; 19] = [eq30_e483_d_n0, eq30_e483_d_n1, eq30_e483_d_n2, eq30_e483_d_n3, eq30_e483_d_n4, eq30_e483_d_n5, eq30_e483_d_n6, eq30_e483_d_n7, eq30_e483_d_n8, eq30_e483_d_n9, eq30_e483_d_n10, eq30_e483_d_n11, eq30_e483_d_n12, eq30_e483_d_n13, eq30_e483_d_n14, eq30_e483_d_n15, eq30_e483_d_n16, eq30_e483_d_n17, eq30_e483_d_n18];
        let eq30_reactive_branch_derivatives: [f64; 15] = [eq30_e483_d_b0, eq30_e483_d_b1, eq30_e483_d_b2, eq30_e483_d_b3, eq30_e483_d_b4, eq30_e483_d_b5, eq30_e483_d_b6, eq30_e483_d_b7, eq30_e483_d_b8, eq30_e483_d_b9, eq30_e483_d_b10, eq30_e483_d_b11, eq30_e483_d_b12, eq30_e483_d_b13, eq30_e483_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            None,
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n1, eq34_e515_d_n2, eq34_e515_d_n3, eq34_e515_d_n4, eq34_e515_d_n5, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n8, eq34_e515_d_n9, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n13, eq34_e515_d_n14, eq34_e515_d_n15, eq34_e515_d_n16, eq34_e515_d_n17, eq34_e515_d_n18, eq34_e515_d_b0, eq34_e515_d_b1, eq34_e515_d_b2, eq34_e515_d_b3, eq34_e515_d_b4, eq34_e515_d_b5, eq34_e515_d_b6, eq34_e515_d_b7, eq34_e515_d_b8, eq34_e515_d_b9, eq34_e515_d_b10, eq34_e515_d_b11, eq34_e515_d_b12, eq34_e515_d_b13, eq34_e515_d_b14, eq34_e515_q, eq34_e515_q_d_n0, eq34_e515_q_d_n1, eq34_e515_q_d_n2, eq34_e515_q_d_n3, eq34_e515_q_d_n4, eq34_e515_q_d_n5, eq34_e515_q_d_n6, eq34_e515_q_d_n7, eq34_e515_q_d_n8, eq34_e515_q_d_n9, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, eq34_e515_q_d_n13, eq34_e515_q_d_n14, eq34_e515_q_d_n15, eq34_e515_q_d_n16, eq34_e515_q_d_n17, eq34_e515_q_d_n18, eq34_e515_q_d_b0, eq34_e515_q_d_b1, eq34_e515_q_d_b2, eq34_e515_q_d_b3, eq34_e515_q_d_b4, eq34_e515_q_d_b5, eq34_e515_q_d_b6, eq34_e515_q_d_b7, eq34_e515_q_d_b8, eq34_e515_q_d_b9, eq34_e515_q_d_b10, eq34_e515_q_d_b11, eq34_e515_q_d_b12, eq34_e515_q_d_b13, eq34_e515_q_d_b14,) = {
    if s.b[1849] {
        let eq34_e511_q: f64 = s.v[283];
        let eq34_e512: f64 = (s.v[281] + s.v[283]);
        let eq34_e512_d_n0: f64 = (s.dn[281][0] + s.dn[283][0]);
        let eq34_e512_d_n1: f64 = (s.dn[281][1] + s.dn[283][1]);
        let eq34_e512_d_n2: f64 = (s.dn[281][2] + s.dn[283][2]);
        let eq34_e512_d_n3: f64 = (s.dn[281][3] + s.dn[283][3]);
        let eq34_e512_d_n4: f64 = (s.dn[281][4] + s.dn[283][4]);
        let eq34_e512_d_n5: f64 = (s.dn[281][5] + s.dn[283][5]);
        let eq34_e512_d_n6: f64 = (s.dn[281][6] + s.dn[283][6]);
        let eq34_e512_d_n7: f64 = (s.dn[281][7] + s.dn[283][7]);
        let eq34_e512_d_n8: f64 = (s.dn[281][8] + s.dn[283][8]);
        let eq34_e512_d_n9: f64 = (s.dn[281][9] + s.dn[283][9]);
        let eq34_e512_d_n10: f64 = (s.dn[281][10] + s.dn[283][10]);
        let eq34_e512_d_n11: f64 = (s.dn[281][11] + s.dn[283][11]);
        let eq34_e512_d_n12: f64 = (s.dn[281][12] + s.dn[283][12]);
        let eq34_e512_d_n13: f64 = (s.dn[281][13] + s.dn[283][13]);
        let eq34_e512_d_n14: f64 = (s.dn[281][14] + s.dn[283][14]);
        let eq34_e512_d_n15: f64 = (s.dn[281][15] + s.dn[283][15]);
        let eq34_e512_d_n16: f64 = (s.dn[281][16] + s.dn[283][16]);
        let eq34_e512_d_n17: f64 = (s.dn[281][17] + s.dn[283][17]);
        let eq34_e512_d_n18: f64 = (s.dn[281][18] + s.dn[283][18]);
        let eq34_e512_d_b0: f64 = (s.db[281][0] + s.db[283][0]);
        let eq34_e512_d_b1: f64 = (s.db[281][1] + s.db[283][1]);
        let eq34_e512_d_b2: f64 = (s.db[281][2] + s.db[283][2]);
        let eq34_e512_d_b3: f64 = (s.db[281][3] + s.db[283][3]);
        let eq34_e512_d_b4: f64 = (s.db[281][4] + s.db[283][4]);
        let eq34_e512_d_b5: f64 = (s.db[281][5] + s.db[283][5]);
        let eq34_e512_d_b6: f64 = (s.db[281][6] + s.db[283][6]);
        let eq34_e512_d_b7: f64 = (s.db[281][7] + s.db[283][7]);
        let eq34_e512_d_b8: f64 = (s.db[281][8] + s.db[283][8]);
        let eq34_e512_d_b9: f64 = (s.db[281][9] + s.db[283][9]);
        let eq34_e512_d_b10: f64 = (s.db[281][10] + s.db[283][10]);
        let eq34_e512_d_b11: f64 = (s.db[281][11] + s.db[283][11]);
        let eq34_e512_d_b12: f64 = (s.db[281][12] + s.db[283][12]);
        let eq34_e512_d_b13: f64 = (s.db[281][13] + s.db[283][13]);
        let eq34_e512_d_b14: f64 = (s.db[281][14] + s.db[283][14]);
        let eq34_e512_q: f64 = eq34_e511_q;
        let eq34_e513: f64 = (p.p50 * eq34_e512);
        let eq34_e513_d_n0: f64 = (p.p50 * eq34_e512_d_n0);
        let eq34_e513_d_n1: f64 = (p.p50 * eq34_e512_d_n1);
        let eq34_e513_d_n2: f64 = (p.p50 * eq34_e512_d_n2);
        let eq34_e513_d_n3: f64 = (p.p50 * eq34_e512_d_n3);
        let eq34_e513_d_n4: f64 = (p.p50 * eq34_e512_d_n4);
        let eq34_e513_d_n5: f64 = (p.p50 * eq34_e512_d_n5);
        let eq34_e513_d_n6: f64 = (p.p50 * eq34_e512_d_n6);
        let eq34_e513_d_n7: f64 = (p.p50 * eq34_e512_d_n7);
        let eq34_e513_d_n8: f64 = (p.p50 * eq34_e512_d_n8);
        let eq34_e513_d_n9: f64 = (p.p50 * eq34_e512_d_n9);
        let eq34_e513_d_n10: f64 = (p.p50 * eq34_e512_d_n10);
        let eq34_e513_d_n11: f64 = (p.p50 * eq34_e512_d_n11);
        let eq34_e513_d_n12: f64 = (p.p50 * eq34_e512_d_n12);
        let eq34_e513_d_n13: f64 = (p.p50 * eq34_e512_d_n13);
        let eq34_e513_d_n14: f64 = (p.p50 * eq34_e512_d_n14);
        let eq34_e513_d_n15: f64 = (p.p50 * eq34_e512_d_n15);
        let eq34_e513_d_n16: f64 = (p.p50 * eq34_e512_d_n16);
        let eq34_e513_d_n17: f64 = (p.p50 * eq34_e512_d_n17);
        let eq34_e513_d_n18: f64 = (p.p50 * eq34_e512_d_n18);
        let eq34_e513_d_b0: f64 = (p.p50 * eq34_e512_d_b0);
        let eq34_e513_d_b1: f64 = (p.p50 * eq34_e512_d_b1);
        let eq34_e513_d_b2: f64 = (p.p50 * eq34_e512_d_b2);
        let eq34_e513_d_b3: f64 = (p.p50 * eq34_e512_d_b3);
        let eq34_e513_d_b4: f64 = (p.p50 * eq34_e512_d_b4);
        let eq34_e513_d_b5: f64 = (p.p50 * eq34_e512_d_b5);
        let eq34_e513_d_b6: f64 = (p.p50 * eq34_e512_d_b6);
        let eq34_e513_d_b7: f64 = (p.p50 * eq34_e512_d_b7);
        let eq34_e513_d_b8: f64 = (p.p50 * eq34_e512_d_b8);
        let eq34_e513_d_b9: f64 = (p.p50 * eq34_e512_d_b9);
        let eq34_e513_d_b10: f64 = (p.p50 * eq34_e512_d_b10);
        let eq34_e513_d_b11: f64 = (p.p50 * eq34_e512_d_b11);
        let eq34_e513_d_b12: f64 = (p.p50 * eq34_e512_d_b12);
        let eq34_e513_d_b13: f64 = (p.p50 * eq34_e512_d_b13);
        let eq34_e513_d_b14: f64 = (p.p50 * eq34_e512_d_b14);
        let eq34_e513_q: f64 = (p.p50 * eq34_e512_q);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n1, eq34_e513_d_n2, eq34_e513_d_n3, eq34_e513_d_n4, eq34_e513_d_n5, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n8, eq34_e513_d_n9, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n13, eq34_e513_d_n14, eq34_e513_d_n15, eq34_e513_d_n16, eq34_e513_d_n17, eq34_e513_d_n18, eq34_e513_d_b0, eq34_e513_d_b1, eq34_e513_d_b2, eq34_e513_d_b3, eq34_e513_d_b4, eq34_e513_d_b5, eq34_e513_d_b6, eq34_e513_d_b7, eq34_e513_d_b8, eq34_e513_d_b9, eq34_e513_d_b10, eq34_e513_d_b11, eq34_e513_d_b12, eq34_e513_d_b13, eq34_e513_d_b14, eq34_e513_q, (p.p50 * s.dn[283][0]), (p.p50 * s.dn[283][1]), (p.p50 * s.dn[283][2]), (p.p50 * s.dn[283][3]), (p.p50 * s.dn[283][4]), (p.p50 * s.dn[283][5]), (p.p50 * s.dn[283][6]), (p.p50 * s.dn[283][7]), (p.p50 * s.dn[283][8]), (p.p50 * s.dn[283][9]), (p.p50 * s.dn[283][10]), (p.p50 * s.dn[283][11]), (p.p50 * s.dn[283][12]), (p.p50 * s.dn[283][13]), (p.p50 * s.dn[283][14]), (p.p50 * s.dn[283][15]), (p.p50 * s.dn[283][16]), (p.p50 * s.dn[283][17]), (p.p50 * s.dn[283][18]), (p.p50 * s.db[283][0]), (p.p50 * s.db[283][1]), (p.p50 * s.db[283][2]), (p.p50 * s.db[283][3]), (p.p50 * s.db[283][4]), (p.p50 * s.db[283][5]), (p.p50 * s.db[283][6]), (p.p50 * s.db[283][7]), (p.p50 * s.db[283][8]), (p.p50 * s.db[283][9]), (p.p50 * s.db[283][10]), (p.p50 * s.db[283][11]), (p.p50 * s.db[283][12]), (p.p50 * s.db[283][13]), (p.p50 * s.db[283][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e515_q_d_n0, eq34_e515_q_d_n1, eq34_e515_q_d_n2, eq34_e515_q_d_n3, eq34_e515_q_d_n4, eq34_e515_q_d_n5, eq34_e515_q_d_n6, eq34_e515_q_d_n7, eq34_e515_q_d_n8, eq34_e515_q_d_n9, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, eq34_e515_q_d_n13, eq34_e515_q_d_n14, eq34_e515_q_d_n15, eq34_e515_q_d_n16, eq34_e515_q_d_n17, eq34_e515_q_d_n18];
        let eq34_reactive_branch_derivatives: [f64; 15] = [eq34_e515_q_d_b0, eq34_e515_q_d_b1, eq34_e515_q_d_b2, eq34_e515_q_d_b3, eq34_e515_q_d_b4, eq34_e515_q_d_b5, eq34_e515_q_d_b6, eq34_e515_q_d_b7, eq34_e515_q_d_b8, eq34_e515_q_d_b9, eq34_e515_q_d_b10, eq34_e515_q_d_b11, eq34_e515_q_d_b12, eq34_e515_q_d_b13, eq34_e515_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n1, eq35_e524_d_n2, eq35_e524_d_n3, eq35_e524_d_n4, eq35_e524_d_n5, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n8, eq35_e524_d_n9, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n13, eq35_e524_d_n14, eq35_e524_d_n15, eq35_e524_d_n16, eq35_e524_d_n17, eq35_e524_d_n18, eq35_e524_d_b0, eq35_e524_d_b1, eq35_e524_d_b2, eq35_e524_d_b3, eq35_e524_d_b4, eq35_e524_d_b5, eq35_e524_d_b6, eq35_e524_d_b7, eq35_e524_d_b8, eq35_e524_d_b9, eq35_e524_d_b10, eq35_e524_d_b11, eq35_e524_d_b12, eq35_e524_d_b13, eq35_e524_d_b14, eq35_e524_q, eq35_e524_q_d_n0, eq35_e524_q_d_n1, eq35_e524_q_d_n2, eq35_e524_q_d_n3, eq35_e524_q_d_n4, eq35_e524_q_d_n5, eq35_e524_q_d_n6, eq35_e524_q_d_n7, eq35_e524_q_d_n8, eq35_e524_q_d_n9, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, eq35_e524_q_d_n13, eq35_e524_q_d_n14, eq35_e524_q_d_n15, eq35_e524_q_d_n16, eq35_e524_q_d_n17, eq35_e524_q_d_n18, eq35_e524_q_d_b0, eq35_e524_q_d_b1, eq35_e524_q_d_b2, eq35_e524_q_d_b3, eq35_e524_q_d_b4, eq35_e524_q_d_b5, eq35_e524_q_d_b6, eq35_e524_q_d_b7, eq35_e524_q_d_b8, eq35_e524_q_d_b9, eq35_e524_q_d_b10, eq35_e524_q_d_b11, eq35_e524_q_d_b12, eq35_e524_q_d_b13, eq35_e524_q_d_b14,) = {
    if s.b[1849] {
        let eq35_e520_q: f64 = s.v[284];
        let eq35_e521: f64 = (s.v[282] + s.v[284]);
        let eq35_e521_d_n0: f64 = (s.dn[282][0] + s.dn[284][0]);
        let eq35_e521_d_n1: f64 = (s.dn[282][1] + s.dn[284][1]);
        let eq35_e521_d_n2: f64 = (s.dn[282][2] + s.dn[284][2]);
        let eq35_e521_d_n3: f64 = (s.dn[282][3] + s.dn[284][3]);
        let eq35_e521_d_n4: f64 = (s.dn[282][4] + s.dn[284][4]);
        let eq35_e521_d_n5: f64 = (s.dn[282][5] + s.dn[284][5]);
        let eq35_e521_d_n6: f64 = (s.dn[282][6] + s.dn[284][6]);
        let eq35_e521_d_n7: f64 = (s.dn[282][7] + s.dn[284][7]);
        let eq35_e521_d_n8: f64 = (s.dn[282][8] + s.dn[284][8]);
        let eq35_e521_d_n9: f64 = (s.dn[282][9] + s.dn[284][9]);
        let eq35_e521_d_n10: f64 = (s.dn[282][10] + s.dn[284][10]);
        let eq35_e521_d_n11: f64 = (s.dn[282][11] + s.dn[284][11]);
        let eq35_e521_d_n12: f64 = (s.dn[282][12] + s.dn[284][12]);
        let eq35_e521_d_n13: f64 = (s.dn[282][13] + s.dn[284][13]);
        let eq35_e521_d_n14: f64 = (s.dn[282][14] + s.dn[284][14]);
        let eq35_e521_d_n15: f64 = (s.dn[282][15] + s.dn[284][15]);
        let eq35_e521_d_n16: f64 = (s.dn[282][16] + s.dn[284][16]);
        let eq35_e521_d_n17: f64 = (s.dn[282][17] + s.dn[284][17]);
        let eq35_e521_d_n18: f64 = (s.dn[282][18] + s.dn[284][18]);
        let eq35_e521_d_b0: f64 = (s.db[282][0] + s.db[284][0]);
        let eq35_e521_d_b1: f64 = (s.db[282][1] + s.db[284][1]);
        let eq35_e521_d_b2: f64 = (s.db[282][2] + s.db[284][2]);
        let eq35_e521_d_b3: f64 = (s.db[282][3] + s.db[284][3]);
        let eq35_e521_d_b4: f64 = (s.db[282][4] + s.db[284][4]);
        let eq35_e521_d_b5: f64 = (s.db[282][5] + s.db[284][5]);
        let eq35_e521_d_b6: f64 = (s.db[282][6] + s.db[284][6]);
        let eq35_e521_d_b7: f64 = (s.db[282][7] + s.db[284][7]);
        let eq35_e521_d_b8: f64 = (s.db[282][8] + s.db[284][8]);
        let eq35_e521_d_b9: f64 = (s.db[282][9] + s.db[284][9]);
        let eq35_e521_d_b10: f64 = (s.db[282][10] + s.db[284][10]);
        let eq35_e521_d_b11: f64 = (s.db[282][11] + s.db[284][11]);
        let eq35_e521_d_b12: f64 = (s.db[282][12] + s.db[284][12]);
        let eq35_e521_d_b13: f64 = (s.db[282][13] + s.db[284][13]);
        let eq35_e521_d_b14: f64 = (s.db[282][14] + s.db[284][14]);
        let eq35_e521_q: f64 = eq35_e520_q;
        let eq35_e522: f64 = (p.p50 * eq35_e521);
        let eq35_e522_d_n0: f64 = (p.p50 * eq35_e521_d_n0);
        let eq35_e522_d_n1: f64 = (p.p50 * eq35_e521_d_n1);
        let eq35_e522_d_n2: f64 = (p.p50 * eq35_e521_d_n2);
        let eq35_e522_d_n3: f64 = (p.p50 * eq35_e521_d_n3);
        let eq35_e522_d_n4: f64 = (p.p50 * eq35_e521_d_n4);
        let eq35_e522_d_n5: f64 = (p.p50 * eq35_e521_d_n5);
        let eq35_e522_d_n6: f64 = (p.p50 * eq35_e521_d_n6);
        let eq35_e522_d_n7: f64 = (p.p50 * eq35_e521_d_n7);
        let eq35_e522_d_n8: f64 = (p.p50 * eq35_e521_d_n8);
        let eq35_e522_d_n9: f64 = (p.p50 * eq35_e521_d_n9);
        let eq35_e522_d_n10: f64 = (p.p50 * eq35_e521_d_n10);
        let eq35_e522_d_n11: f64 = (p.p50 * eq35_e521_d_n11);
        let eq35_e522_d_n12: f64 = (p.p50 * eq35_e521_d_n12);
        let eq35_e522_d_n13: f64 = (p.p50 * eq35_e521_d_n13);
        let eq35_e522_d_n14: f64 = (p.p50 * eq35_e521_d_n14);
        let eq35_e522_d_n15: f64 = (p.p50 * eq35_e521_d_n15);
        let eq35_e522_d_n16: f64 = (p.p50 * eq35_e521_d_n16);
        let eq35_e522_d_n17: f64 = (p.p50 * eq35_e521_d_n17);
        let eq35_e522_d_n18: f64 = (p.p50 * eq35_e521_d_n18);
        let eq35_e522_d_b0: f64 = (p.p50 * eq35_e521_d_b0);
        let eq35_e522_d_b1: f64 = (p.p50 * eq35_e521_d_b1);
        let eq35_e522_d_b2: f64 = (p.p50 * eq35_e521_d_b2);
        let eq35_e522_d_b3: f64 = (p.p50 * eq35_e521_d_b3);
        let eq35_e522_d_b4: f64 = (p.p50 * eq35_e521_d_b4);
        let eq35_e522_d_b5: f64 = (p.p50 * eq35_e521_d_b5);
        let eq35_e522_d_b6: f64 = (p.p50 * eq35_e521_d_b6);
        let eq35_e522_d_b7: f64 = (p.p50 * eq35_e521_d_b7);
        let eq35_e522_d_b8: f64 = (p.p50 * eq35_e521_d_b8);
        let eq35_e522_d_b9: f64 = (p.p50 * eq35_e521_d_b9);
        let eq35_e522_d_b10: f64 = (p.p50 * eq35_e521_d_b10);
        let eq35_e522_d_b11: f64 = (p.p50 * eq35_e521_d_b11);
        let eq35_e522_d_b12: f64 = (p.p50 * eq35_e521_d_b12);
        let eq35_e522_d_b13: f64 = (p.p50 * eq35_e521_d_b13);
        let eq35_e522_d_b14: f64 = (p.p50 * eq35_e521_d_b14);
        let eq35_e522_q: f64 = (p.p50 * eq35_e521_q);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n1, eq35_e522_d_n2, eq35_e522_d_n3, eq35_e522_d_n4, eq35_e522_d_n5, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n8, eq35_e522_d_n9, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n13, eq35_e522_d_n14, eq35_e522_d_n15, eq35_e522_d_n16, eq35_e522_d_n17, eq35_e522_d_n18, eq35_e522_d_b0, eq35_e522_d_b1, eq35_e522_d_b2, eq35_e522_d_b3, eq35_e522_d_b4, eq35_e522_d_b5, eq35_e522_d_b6, eq35_e522_d_b7, eq35_e522_d_b8, eq35_e522_d_b9, eq35_e522_d_b10, eq35_e522_d_b11, eq35_e522_d_b12, eq35_e522_d_b13, eq35_e522_d_b14, eq35_e522_q, (p.p50 * s.dn[284][0]), (p.p50 * s.dn[284][1]), (p.p50 * s.dn[284][2]), (p.p50 * s.dn[284][3]), (p.p50 * s.dn[284][4]), (p.p50 * s.dn[284][5]), (p.p50 * s.dn[284][6]), (p.p50 * s.dn[284][7]), (p.p50 * s.dn[284][8]), (p.p50 * s.dn[284][9]), (p.p50 * s.dn[284][10]), (p.p50 * s.dn[284][11]), (p.p50 * s.dn[284][12]), (p.p50 * s.dn[284][13]), (p.p50 * s.dn[284][14]), (p.p50 * s.dn[284][15]), (p.p50 * s.dn[284][16]), (p.p50 * s.dn[284][17]), (p.p50 * s.dn[284][18]), (p.p50 * s.db[284][0]), (p.p50 * s.db[284][1]), (p.p50 * s.db[284][2]), (p.p50 * s.db[284][3]), (p.p50 * s.db[284][4]), (p.p50 * s.db[284][5]), (p.p50 * s.db[284][6]), (p.p50 * s.db[284][7]), (p.p50 * s.db[284][8]), (p.p50 * s.db[284][9]), (p.p50 * s.db[284][10]), (p.p50 * s.db[284][11]), (p.p50 * s.db[284][12]), (p.p50 * s.db[284][13]), (p.p50 * s.db[284][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e524_q_d_n0, eq35_e524_q_d_n1, eq35_e524_q_d_n2, eq35_e524_q_d_n3, eq35_e524_q_d_n4, eq35_e524_q_d_n5, eq35_e524_q_d_n6, eq35_e524_q_d_n7, eq35_e524_q_d_n8, eq35_e524_q_d_n9, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, eq35_e524_q_d_n13, eq35_e524_q_d_n14, eq35_e524_q_d_n15, eq35_e524_q_d_n16, eq35_e524_q_d_n17, eq35_e524_q_d_n18];
        let eq35_reactive_branch_derivatives: [f64; 15] = [eq35_e524_q_d_b0, eq35_e524_q_d_b1, eq35_e524_q_d_b2, eq35_e524_q_d_b3, eq35_e524_q_d_b4, eq35_e524_q_d_b5, eq35_e524_q_d_b6, eq35_e524_q_d_b7, eq35_e524_q_d_b8, eq35_e524_q_d_b9, eq35_e524_q_d_b10, eq35_e524_q_d_b11, eq35_e524_q_d_b12, eq35_e524_q_d_b13, eq35_e524_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e608, eq46_e608_d_n18, eq46_e608_q,) = {
    if (s.b[1849] && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606_q: f64 = eq46_e605;
        (eq46_e605, eq46_e603, eq46_e606_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq46_e608_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13, eq47_e619_q,) = {
    if (s.b[1849] && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617_q: f64 = eq47_e616;
        (eq47_e616, eq47_e614, eq47_e617_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq47_e619_d_n13),
        );
        let (eq52_e658, eq52_e658_d_n17, eq52_e658_q,) = {
    if (s.b[1849] && s.b[1850]) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656_q: f64 = eq52_e655;
        (eq52_e655, eq52_e653, eq52_e656_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq52_e658_d_n17),
        );
        let (eq59_e716, eq59_e716_d_n17, eq59_e716_q,) = {
    if ((!s.b[1849]) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714_q: f64 = eq59_e713;
        (eq59_e713, eq59_e711, eq59_e714_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq59_e716_d_n17),
        );
        let (eq67_e784, eq67_e784_d_n15, eq67_e784_q,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782_q: f64 = eq67_e781;
        (eq67_e781, eq67_e779, eq67_e782_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq67_e784_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16, eq68_e796_q,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794_q: f64 = eq68_e793;
        (eq68_e793, eq68_e791, eq68_e794_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq68_e796_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13, eq69_e808_q,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806_q: f64 = eq69_e805;
        (eq69_e805, eq69_e803, eq69_e806_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq69_e808_d_n13),
        );
    }
}
