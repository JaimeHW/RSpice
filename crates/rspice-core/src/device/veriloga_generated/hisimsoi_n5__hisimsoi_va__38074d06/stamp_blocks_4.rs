#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
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
            s.store_add_scaled_inputs3(586, s.ad_value(580), -1.0, s.ad_value(581), (-1.0), s.ad_value(471), 1.0);
        }

        if (s.b[1831] && (s.v[85] == 0.0)) {
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(586, 0.0);
            s.store_scalar(581, 0.0);
        }

        if ((!s.b[1831]) && (s.v[85] != 0.0)) {
            s.store_add_scaled_inputs3(586, s.ad_value(584), -1.0, s.ad_value(585), (-1.0), s.ad_value(581), -1.0);
        }

        if ((!s.b[1831]) && (s.v[85] == 0.0)) {
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(586, 0.0);
            s.store_scalar(581, 0.0);
        }

        s.b[1836] = (s.v[613] == 1.0);
        s.v[1836] = if s.b[1836] { 1.0 } else { 0.0 };

        if s.b[1836] {
            s.copy_ad(199, 9);
            s.copy_ad(263, 557);
            s.store_add(594, 23, 586);
            s.store_add(198, 24, 584);
            s.store_neg_ad(554, A::add_scaled_inputs3(s.ad_value(23), 1.0, s.ad_value(24), 1.0, s.ad_value(25), 1.0));
            s.store_add(196, 554, 581);
        }

        if (!s.b[1836]) {
            s.store_neg(199, 9);
            s.store_scalar(263, 0.0);
            s.store_add(594, 23, 586);
            s.store_add(198, 25, 585);
            s.store_neg_ad(554, A::add_scaled_inputs3(s.ad_value(23), 1.0, s.ad_value(24), 1.0, s.ad_value(25), 1.0));
            s.store_add(196, 554, 581);
        }

        s.b[1837] = (p.p43 == 1.0);
        s.v[1837] = if s.b[1837] { 1.0 } else { 0.0 };

        if s.b[1837] {
            s.copy_ad(282, 35);
            s.copy_ad(284, 560);
            s.copy_ad(281, 36);
            s.copy_ad(283, 561);
        }

        s.b[1838] = ((p.p38 == 1.0) && (s.v[67] > 0.0));
        s.v[1838] = if s.b[1838] { 1.0 } else { 0.0 };

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
        s.v[1840] = if s.b[1840] { 1.0 } else { 0.0 };

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
        s.v[1848] = if s.b[1848] { 1.0 } else { 0.0 };

        s.b[1849] = (p.p43 == 1.0);
        s.v[1849] = if s.b[1849] { 1.0 } else { 0.0 };

        s.b[1850] = ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0)));
        s.v[1850] = if s.b[1850] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq0_value: f64 = 0.0;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e313,) = {
    if s.b[627] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e313;
        stamper.stamp_potential_const_local(
            1,
            eq1_value,
        );
        let eq2_e316: f64 = (p.p50 * s.v[199]);
        let eq2_e316_d_n0: f64 = (p.p50 * s.dn[199][0]);
        let eq2_e316_d_n1: f64 = (p.p50 * s.dn[199][1]);
        let eq2_e316_d_n2: f64 = (p.p50 * s.dn[199][2]);
        let eq2_e316_d_n3: f64 = (p.p50 * s.dn[199][3]);
        let eq2_e316_d_n4: f64 = (p.p50 * s.dn[199][4]);
        let eq2_e316_d_n5: f64 = (p.p50 * s.dn[199][5]);
        let eq2_e316_d_n6: f64 = (p.p50 * s.dn[199][6]);
        let eq2_e316_d_n7: f64 = (p.p50 * s.dn[199][7]);
        let eq2_e316_d_n8: f64 = (p.p50 * s.dn[199][8]);
        let eq2_e316_d_n9: f64 = (p.p50 * s.dn[199][9]);
        let eq2_e316_d_n10: f64 = (p.p50 * s.dn[199][10]);
        let eq2_e316_d_n11: f64 = (p.p50 * s.dn[199][11]);
        let eq2_e316_d_n12: f64 = (p.p50 * s.dn[199][12]);
        let eq2_e316_d_n13: f64 = (p.p50 * s.dn[199][13]);
        let eq2_e316_d_n14: f64 = (p.p50 * s.dn[199][14]);
        let eq2_e316_d_n15: f64 = (p.p50 * s.dn[199][15]);
        let eq2_e316_d_n16: f64 = (p.p50 * s.dn[199][16]);
        let eq2_e316_d_n17: f64 = (p.p50 * s.dn[199][17]);
        let eq2_e316_d_n18: f64 = (p.p50 * s.dn[199][18]);
        let eq2_e316_d_b0: f64 = (p.p50 * s.db[199][0]);
        let eq2_e316_d_b1: f64 = (p.p50 * s.db[199][1]);
        let eq2_e316_d_b2: f64 = (p.p50 * s.db[199][2]);
        let eq2_e316_d_b3: f64 = (p.p50 * s.db[199][3]);
        let eq2_e316_d_b4: f64 = (p.p50 * s.db[199][4]);
        let eq2_e316_d_b5: f64 = (p.p50 * s.db[199][5]);
        let eq2_e316_d_b6: f64 = (p.p50 * s.db[199][6]);
        let eq2_e316_d_b7: f64 = (p.p50 * s.db[199][7]);
        let eq2_e316_d_b8: f64 = (p.p50 * s.db[199][8]);
        let eq2_e316_d_b9: f64 = (p.p50 * s.db[199][9]);
        let eq2_e316_d_b10: f64 = (p.p50 * s.db[199][10]);
        let eq2_e316_d_b11: f64 = (p.p50 * s.db[199][11]);
        let eq2_e316_d_b12: f64 = (p.p50 * s.db[199][12]);
        let eq2_e316_d_b13: f64 = (p.p50 * s.db[199][13]);
        let eq2_e316_d_b14: f64 = (p.p50 * s.db[199][14]);
        let eq2_value: f64 = eq2_e316;
        let eq2_node_derivatives: [f64; 19] = [eq2_e316_d_n0, eq2_e316_d_n1, eq2_e316_d_n2, eq2_e316_d_n3, eq2_e316_d_n4, eq2_e316_d_n5, eq2_e316_d_n6, eq2_e316_d_n7, eq2_e316_d_n8, eq2_e316_d_n9, eq2_e316_d_n10, eq2_e316_d_n11, eq2_e316_d_n12, eq2_e316_d_n13, eq2_e316_d_n14, eq2_e316_d_n15, eq2_e316_d_n16, eq2_e316_d_n17, eq2_e316_d_n18];
        let eq2_branch_derivatives: [f64; 15] = [eq2_e316_d_b0, eq2_e316_d_b1, eq2_e316_d_b2, eq2_e316_d_b3, eq2_e316_d_b4, eq2_e316_d_b5, eq2_e316_d_b6, eq2_e316_d_b7, eq2_e316_d_b8, eq2_e316_d_b9, eq2_e316_d_b10, eq2_e316_d_b11, eq2_e316_d_b12, eq2_e316_d_b13, eq2_e316_d_b14];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e322, eq3_e322_d_n0, eq3_e322_d_n1, eq3_e322_d_n2, eq3_e322_d_n3, eq3_e322_d_n4, eq3_e322_d_n5, eq3_e322_d_n6, eq3_e322_d_n7, eq3_e322_d_n8, eq3_e322_d_n9, eq3_e322_d_n10, eq3_e322_d_n11, eq3_e322_d_n12, eq3_e322_d_n13, eq3_e322_d_n14, eq3_e322_d_n15, eq3_e322_d_n16, eq3_e322_d_n17, eq3_e322_d_n18, eq3_e322_d_b0, eq3_e322_d_b1, eq3_e322_d_b2, eq3_e322_d_b3, eq3_e322_d_b4, eq3_e322_d_b5, eq3_e322_d_b6, eq3_e322_d_b7, eq3_e322_d_b8, eq3_e322_d_b9, eq3_e322_d_b10, eq3_e322_d_b11, eq3_e322_d_b12, eq3_e322_d_b13, eq3_e322_d_b14,) = {
    if s.b[1846] {
        let eq3_e320: f64 = (p.p50 * s.v[306]);
        let eq3_e320_d_n0: f64 = (p.p50 * s.dn[306][0]);
        let eq3_e320_d_n1: f64 = (p.p50 * s.dn[306][1]);
        let eq3_e320_d_n2: f64 = (p.p50 * s.dn[306][2]);
        let eq3_e320_d_n3: f64 = (p.p50 * s.dn[306][3]);
        let eq3_e320_d_n4: f64 = (p.p50 * s.dn[306][4]);
        let eq3_e320_d_n5: f64 = (p.p50 * s.dn[306][5]);
        let eq3_e320_d_n6: f64 = (p.p50 * s.dn[306][6]);
        let eq3_e320_d_n7: f64 = (p.p50 * s.dn[306][7]);
        let eq3_e320_d_n8: f64 = (p.p50 * s.dn[306][8]);
        let eq3_e320_d_n9: f64 = (p.p50 * s.dn[306][9]);
        let eq3_e320_d_n10: f64 = (p.p50 * s.dn[306][10]);
        let eq3_e320_d_n11: f64 = (p.p50 * s.dn[306][11]);
        let eq3_e320_d_n12: f64 = (p.p50 * s.dn[306][12]);
        let eq3_e320_d_n13: f64 = (p.p50 * s.dn[306][13]);
        let eq3_e320_d_n14: f64 = (p.p50 * s.dn[306][14]);
        let eq3_e320_d_n15: f64 = (p.p50 * s.dn[306][15]);
        let eq3_e320_d_n16: f64 = (p.p50 * s.dn[306][16]);
        let eq3_e320_d_n17: f64 = (p.p50 * s.dn[306][17]);
        let eq3_e320_d_n18: f64 = (p.p50 * s.dn[306][18]);
        let eq3_e320_d_b0: f64 = (p.p50 * s.db[306][0]);
        let eq3_e320_d_b1: f64 = (p.p50 * s.db[306][1]);
        let eq3_e320_d_b2: f64 = (p.p50 * s.db[306][2]);
        let eq3_e320_d_b3: f64 = (p.p50 * s.db[306][3]);
        let eq3_e320_d_b4: f64 = (p.p50 * s.db[306][4]);
        let eq3_e320_d_b5: f64 = (p.p50 * s.db[306][5]);
        let eq3_e320_d_b6: f64 = (p.p50 * s.db[306][6]);
        let eq3_e320_d_b7: f64 = (p.p50 * s.db[306][7]);
        let eq3_e320_d_b8: f64 = (p.p50 * s.db[306][8]);
        let eq3_e320_d_b9: f64 = (p.p50 * s.db[306][9]);
        let eq3_e320_d_b10: f64 = (p.p50 * s.db[306][10]);
        let eq3_e320_d_b11: f64 = (p.p50 * s.db[306][11]);
        let eq3_e320_d_b12: f64 = (p.p50 * s.db[306][12]);
        let eq3_e320_d_b13: f64 = (p.p50 * s.db[306][13]);
        let eq3_e320_d_b14: f64 = (p.p50 * s.db[306][14]);
        (eq3_e320, eq3_e320_d_n0, eq3_e320_d_n1, eq3_e320_d_n2, eq3_e320_d_n3, eq3_e320_d_n4, eq3_e320_d_n5, eq3_e320_d_n6, eq3_e320_d_n7, eq3_e320_d_n8, eq3_e320_d_n9, eq3_e320_d_n10, eq3_e320_d_n11, eq3_e320_d_n12, eq3_e320_d_n13, eq3_e320_d_n14, eq3_e320_d_n15, eq3_e320_d_n16, eq3_e320_d_n17, eq3_e320_d_n18, eq3_e320_d_b0, eq3_e320_d_b1, eq3_e320_d_b2, eq3_e320_d_b3, eq3_e320_d_b4, eq3_e320_d_b5, eq3_e320_d_b6, eq3_e320_d_b7, eq3_e320_d_b8, eq3_e320_d_b9, eq3_e320_d_b10, eq3_e320_d_b11, eq3_e320_d_b12, eq3_e320_d_b13, eq3_e320_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e322;
        let eq3_node_derivatives: [f64; 19] = [eq3_e322_d_n0, eq3_e322_d_n1, eq3_e322_d_n2, eq3_e322_d_n3, eq3_e322_d_n4, eq3_e322_d_n5, eq3_e322_d_n6, eq3_e322_d_n7, eq3_e322_d_n8, eq3_e322_d_n9, eq3_e322_d_n10, eq3_e322_d_n11, eq3_e322_d_n12, eq3_e322_d_n13, eq3_e322_d_n14, eq3_e322_d_n15, eq3_e322_d_n16, eq3_e322_d_n17, eq3_e322_d_n18];
        let eq3_branch_derivatives: [f64; 15] = [eq3_e322_d_b0, eq3_e322_d_b1, eq3_e322_d_b2, eq3_e322_d_b3, eq3_e322_d_b4, eq3_e322_d_b5, eq3_e322_d_b6, eq3_e322_d_b7, eq3_e322_d_b8, eq3_e322_d_b9, eq3_e322_d_b10, eq3_e322_d_b11, eq3_e322_d_b12, eq3_e322_d_b13, eq3_e322_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n1, eq4_e328_d_n2, eq4_e328_d_n3, eq4_e328_d_n4, eq4_e328_d_n5, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n8, eq4_e328_d_n9, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n13, eq4_e328_d_n14, eq4_e328_d_n15, eq4_e328_d_n16, eq4_e328_d_n17, eq4_e328_d_n18, eq4_e328_d_b0, eq4_e328_d_b1, eq4_e328_d_b2, eq4_e328_d_b3, eq4_e328_d_b4, eq4_e328_d_b5, eq4_e328_d_b6, eq4_e328_d_b7, eq4_e328_d_b8, eq4_e328_d_b9, eq4_e328_d_b10, eq4_e328_d_b11, eq4_e328_d_b12, eq4_e328_d_b13, eq4_e328_d_b14,) = {
    if s.b[1846] {
        let eq4_e326: f64 = (p.p50 * s.v[307]);
        let eq4_e326_d_n0: f64 = (p.p50 * s.dn[307][0]);
        let eq4_e326_d_n1: f64 = (p.p50 * s.dn[307][1]);
        let eq4_e326_d_n2: f64 = (p.p50 * s.dn[307][2]);
        let eq4_e326_d_n3: f64 = (p.p50 * s.dn[307][3]);
        let eq4_e326_d_n4: f64 = (p.p50 * s.dn[307][4]);
        let eq4_e326_d_n5: f64 = (p.p50 * s.dn[307][5]);
        let eq4_e326_d_n6: f64 = (p.p50 * s.dn[307][6]);
        let eq4_e326_d_n7: f64 = (p.p50 * s.dn[307][7]);
        let eq4_e326_d_n8: f64 = (p.p50 * s.dn[307][8]);
        let eq4_e326_d_n9: f64 = (p.p50 * s.dn[307][9]);
        let eq4_e326_d_n10: f64 = (p.p50 * s.dn[307][10]);
        let eq4_e326_d_n11: f64 = (p.p50 * s.dn[307][11]);
        let eq4_e326_d_n12: f64 = (p.p50 * s.dn[307][12]);
        let eq4_e326_d_n13: f64 = (p.p50 * s.dn[307][13]);
        let eq4_e326_d_n14: f64 = (p.p50 * s.dn[307][14]);
        let eq4_e326_d_n15: f64 = (p.p50 * s.dn[307][15]);
        let eq4_e326_d_n16: f64 = (p.p50 * s.dn[307][16]);
        let eq4_e326_d_n17: f64 = (p.p50 * s.dn[307][17]);
        let eq4_e326_d_n18: f64 = (p.p50 * s.dn[307][18]);
        let eq4_e326_d_b0: f64 = (p.p50 * s.db[307][0]);
        let eq4_e326_d_b1: f64 = (p.p50 * s.db[307][1]);
        let eq4_e326_d_b2: f64 = (p.p50 * s.db[307][2]);
        let eq4_e326_d_b3: f64 = (p.p50 * s.db[307][3]);
        let eq4_e326_d_b4: f64 = (p.p50 * s.db[307][4]);
        let eq4_e326_d_b5: f64 = (p.p50 * s.db[307][5]);
        let eq4_e326_d_b6: f64 = (p.p50 * s.db[307][6]);
        let eq4_e326_d_b7: f64 = (p.p50 * s.db[307][7]);
        let eq4_e326_d_b8: f64 = (p.p50 * s.db[307][8]);
        let eq4_e326_d_b9: f64 = (p.p50 * s.db[307][9]);
        let eq4_e326_d_b10: f64 = (p.p50 * s.db[307][10]);
        let eq4_e326_d_b11: f64 = (p.p50 * s.db[307][11]);
        let eq4_e326_d_b12: f64 = (p.p50 * s.db[307][12]);
        let eq4_e326_d_b13: f64 = (p.p50 * s.db[307][13]);
        let eq4_e326_d_b14: f64 = (p.p50 * s.db[307][14]);
        (eq4_e326, eq4_e326_d_n0, eq4_e326_d_n1, eq4_e326_d_n2, eq4_e326_d_n3, eq4_e326_d_n4, eq4_e326_d_n5, eq4_e326_d_n6, eq4_e326_d_n7, eq4_e326_d_n8, eq4_e326_d_n9, eq4_e326_d_n10, eq4_e326_d_n11, eq4_e326_d_n12, eq4_e326_d_n13, eq4_e326_d_n14, eq4_e326_d_n15, eq4_e326_d_n16, eq4_e326_d_n17, eq4_e326_d_n18, eq4_e326_d_b0, eq4_e326_d_b1, eq4_e326_d_b2, eq4_e326_d_b3, eq4_e326_d_b4, eq4_e326_d_b5, eq4_e326_d_b6, eq4_e326_d_b7, eq4_e326_d_b8, eq4_e326_d_b9, eq4_e326_d_b10, eq4_e326_d_b11, eq4_e326_d_b12, eq4_e326_d_b13, eq4_e326_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e328;
        let eq4_node_derivatives: [f64; 19] = [eq4_e328_d_n0, eq4_e328_d_n1, eq4_e328_d_n2, eq4_e328_d_n3, eq4_e328_d_n4, eq4_e328_d_n5, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n8, eq4_e328_d_n9, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n13, eq4_e328_d_n14, eq4_e328_d_n15, eq4_e328_d_n16, eq4_e328_d_n17, eq4_e328_d_n18];
        let eq4_branch_derivatives: [f64; 15] = [eq4_e328_d_b0, eq4_e328_d_b1, eq4_e328_d_b2, eq4_e328_d_b3, eq4_e328_d_b4, eq4_e328_d_b5, eq4_e328_d_b6, eq4_e328_d_b7, eq4_e328_d_b8, eq4_e328_d_b9, eq4_e328_d_b10, eq4_e328_d_b11, eq4_e328_d_b12, eq4_e328_d_b13, eq4_e328_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n1, eq5_e334_d_n2, eq5_e334_d_n3, eq5_e334_d_n4, eq5_e334_d_n5, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n8, eq5_e334_d_n9, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n13, eq5_e334_d_n14, eq5_e334_d_n15, eq5_e334_d_n16, eq5_e334_d_n17, eq5_e334_d_n18, eq5_e334_d_b0, eq5_e334_d_b1, eq5_e334_d_b2, eq5_e334_d_b3, eq5_e334_d_b4, eq5_e334_d_b5, eq5_e334_d_b6, eq5_e334_d_b7, eq5_e334_d_b8, eq5_e334_d_b9, eq5_e334_d_b10, eq5_e334_d_b11, eq5_e334_d_b12, eq5_e334_d_b13, eq5_e334_d_b14,) = {
    if s.b[1846] {
        let eq5_e332: f64 = (p.p50 * s.v[308]);
        let eq5_e332_d_n0: f64 = (p.p50 * s.dn[308][0]);
        let eq5_e332_d_n1: f64 = (p.p50 * s.dn[308][1]);
        let eq5_e332_d_n2: f64 = (p.p50 * s.dn[308][2]);
        let eq5_e332_d_n3: f64 = (p.p50 * s.dn[308][3]);
        let eq5_e332_d_n4: f64 = (p.p50 * s.dn[308][4]);
        let eq5_e332_d_n5: f64 = (p.p50 * s.dn[308][5]);
        let eq5_e332_d_n6: f64 = (p.p50 * s.dn[308][6]);
        let eq5_e332_d_n7: f64 = (p.p50 * s.dn[308][7]);
        let eq5_e332_d_n8: f64 = (p.p50 * s.dn[308][8]);
        let eq5_e332_d_n9: f64 = (p.p50 * s.dn[308][9]);
        let eq5_e332_d_n10: f64 = (p.p50 * s.dn[308][10]);
        let eq5_e332_d_n11: f64 = (p.p50 * s.dn[308][11]);
        let eq5_e332_d_n12: f64 = (p.p50 * s.dn[308][12]);
        let eq5_e332_d_n13: f64 = (p.p50 * s.dn[308][13]);
        let eq5_e332_d_n14: f64 = (p.p50 * s.dn[308][14]);
        let eq5_e332_d_n15: f64 = (p.p50 * s.dn[308][15]);
        let eq5_e332_d_n16: f64 = (p.p50 * s.dn[308][16]);
        let eq5_e332_d_n17: f64 = (p.p50 * s.dn[308][17]);
        let eq5_e332_d_n18: f64 = (p.p50 * s.dn[308][18]);
        let eq5_e332_d_b0: f64 = (p.p50 * s.db[308][0]);
        let eq5_e332_d_b1: f64 = (p.p50 * s.db[308][1]);
        let eq5_e332_d_b2: f64 = (p.p50 * s.db[308][2]);
        let eq5_e332_d_b3: f64 = (p.p50 * s.db[308][3]);
        let eq5_e332_d_b4: f64 = (p.p50 * s.db[308][4]);
        let eq5_e332_d_b5: f64 = (p.p50 * s.db[308][5]);
        let eq5_e332_d_b6: f64 = (p.p50 * s.db[308][6]);
        let eq5_e332_d_b7: f64 = (p.p50 * s.db[308][7]);
        let eq5_e332_d_b8: f64 = (p.p50 * s.db[308][8]);
        let eq5_e332_d_b9: f64 = (p.p50 * s.db[308][9]);
        let eq5_e332_d_b10: f64 = (p.p50 * s.db[308][10]);
        let eq5_e332_d_b11: f64 = (p.p50 * s.db[308][11]);
        let eq5_e332_d_b12: f64 = (p.p50 * s.db[308][12]);
        let eq5_e332_d_b13: f64 = (p.p50 * s.db[308][13]);
        let eq5_e332_d_b14: f64 = (p.p50 * s.db[308][14]);
        (eq5_e332, eq5_e332_d_n0, eq5_e332_d_n1, eq5_e332_d_n2, eq5_e332_d_n3, eq5_e332_d_n4, eq5_e332_d_n5, eq5_e332_d_n6, eq5_e332_d_n7, eq5_e332_d_n8, eq5_e332_d_n9, eq5_e332_d_n10, eq5_e332_d_n11, eq5_e332_d_n12, eq5_e332_d_n13, eq5_e332_d_n14, eq5_e332_d_n15, eq5_e332_d_n16, eq5_e332_d_n17, eq5_e332_d_n18, eq5_e332_d_b0, eq5_e332_d_b1, eq5_e332_d_b2, eq5_e332_d_b3, eq5_e332_d_b4, eq5_e332_d_b5, eq5_e332_d_b6, eq5_e332_d_b7, eq5_e332_d_b8, eq5_e332_d_b9, eq5_e332_d_b10, eq5_e332_d_b11, eq5_e332_d_b12, eq5_e332_d_b13, eq5_e332_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e334;
        let eq5_node_derivatives: [f64; 19] = [eq5_e334_d_n0, eq5_e334_d_n1, eq5_e334_d_n2, eq5_e334_d_n3, eq5_e334_d_n4, eq5_e334_d_n5, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n8, eq5_e334_d_n9, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n13, eq5_e334_d_n14, eq5_e334_d_n15, eq5_e334_d_n16, eq5_e334_d_n17, eq5_e334_d_n18];
        let eq5_branch_derivatives: [f64; 15] = [eq5_e334_d_b0, eq5_e334_d_b1, eq5_e334_d_b2, eq5_e334_d_b3, eq5_e334_d_b4, eq5_e334_d_b5, eq5_e334_d_b6, eq5_e334_d_b7, eq5_e334_d_b8, eq5_e334_d_b9, eq5_e334_d_b10, eq5_e334_d_b11, eq5_e334_d_b12, eq5_e334_d_b13, eq5_e334_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n1, eq6_e340_d_n2, eq6_e340_d_n3, eq6_e340_d_n4, eq6_e340_d_n5, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n8, eq6_e340_d_n9, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n13, eq6_e340_d_n14, eq6_e340_d_n15, eq6_e340_d_n16, eq6_e340_d_n17, eq6_e340_d_n18, eq6_e340_d_b0, eq6_e340_d_b1, eq6_e340_d_b2, eq6_e340_d_b3, eq6_e340_d_b4, eq6_e340_d_b5, eq6_e340_d_b6, eq6_e340_d_b7, eq6_e340_d_b8, eq6_e340_d_b9, eq6_e340_d_b10, eq6_e340_d_b11, eq6_e340_d_b12, eq6_e340_d_b13, eq6_e340_d_b14,) = {
    if (p.p259 != 0.0) {
        let eq6_e338: f64 = ((nv7 - nv2) / s.v[1]);
        let eq6_e338_d_n0: f64 = (-(((nv7 - nv2) * s.dn[1][0]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n1: f64 = (-(((nv7 - nv2) * s.dn[1][1]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n2: f64 = (((-s.v[1]) - ((nv7 - nv2) * s.dn[1][2])) / (s.v[1] * s.v[1]));
        let eq6_e338_d_n3: f64 = (-(((nv7 - nv2) * s.dn[1][3]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n4: f64 = (-(((nv7 - nv2) * s.dn[1][4]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n5: f64 = (-(((nv7 - nv2) * s.dn[1][5]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n6: f64 = (-(((nv7 - nv2) * s.dn[1][6]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n7: f64 = ((s.v[1] - ((nv7 - nv2) * s.dn[1][7])) / (s.v[1] * s.v[1]));
        let eq6_e338_d_n8: f64 = (-(((nv7 - nv2) * s.dn[1][8]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n9: f64 = (-(((nv7 - nv2) * s.dn[1][9]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n10: f64 = (-(((nv7 - nv2) * s.dn[1][10]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n11: f64 = (-(((nv7 - nv2) * s.dn[1][11]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n12: f64 = (-(((nv7 - nv2) * s.dn[1][12]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n13: f64 = (-(((nv7 - nv2) * s.dn[1][13]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n14: f64 = (-(((nv7 - nv2) * s.dn[1][14]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n15: f64 = (-(((nv7 - nv2) * s.dn[1][15]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n16: f64 = (-(((nv7 - nv2) * s.dn[1][16]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n17: f64 = (-(((nv7 - nv2) * s.dn[1][17]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n18: f64 = (-(((nv7 - nv2) * s.dn[1][18]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b0: f64 = (-(((nv7 - nv2) * s.db[1][0]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b1: f64 = (-(((nv7 - nv2) * s.db[1][1]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b2: f64 = (-(((nv7 - nv2) * s.db[1][2]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b3: f64 = (-(((nv7 - nv2) * s.db[1][3]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b4: f64 = (-(((nv7 - nv2) * s.db[1][4]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b5: f64 = (-(((nv7 - nv2) * s.db[1][5]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b6: f64 = (-(((nv7 - nv2) * s.db[1][6]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b7: f64 = (-(((nv7 - nv2) * s.db[1][7]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b8: f64 = (-(((nv7 - nv2) * s.db[1][8]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b9: f64 = (-(((nv7 - nv2) * s.db[1][9]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b10: f64 = (-(((nv7 - nv2) * s.db[1][10]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b11: f64 = (-(((nv7 - nv2) * s.db[1][11]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b12: f64 = (-(((nv7 - nv2) * s.db[1][12]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b13: f64 = (-(((nv7 - nv2) * s.db[1][13]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b14: f64 = (-(((nv7 - nv2) * s.db[1][14]) / (s.v[1] * s.v[1])));
        (eq6_e338, eq6_e338_d_n0, eq6_e338_d_n1, eq6_e338_d_n2, eq6_e338_d_n3, eq6_e338_d_n4, eq6_e338_d_n5, eq6_e338_d_n6, eq6_e338_d_n7, eq6_e338_d_n8, eq6_e338_d_n9, eq6_e338_d_n10, eq6_e338_d_n11, eq6_e338_d_n12, eq6_e338_d_n13, eq6_e338_d_n14, eq6_e338_d_n15, eq6_e338_d_n16, eq6_e338_d_n17, eq6_e338_d_n18, eq6_e338_d_b0, eq6_e338_d_b1, eq6_e338_d_b2, eq6_e338_d_b3, eq6_e338_d_b4, eq6_e338_d_b5, eq6_e338_d_b6, eq6_e338_d_b7, eq6_e338_d_b8, eq6_e338_d_b9, eq6_e338_d_b10, eq6_e338_d_b11, eq6_e338_d_b12, eq6_e338_d_b13, eq6_e338_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e340;
        let eq6_node_derivatives: [f64; 19] = [eq6_e340_d_n0, eq6_e340_d_n1, eq6_e340_d_n2, eq6_e340_d_n3, eq6_e340_d_n4, eq6_e340_d_n5, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n8, eq6_e340_d_n9, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n13, eq6_e340_d_n14, eq6_e340_d_n15, eq6_e340_d_n16, eq6_e340_d_n17, eq6_e340_d_n18];
        let eq6_branch_derivatives: [f64; 15] = [eq6_e340_d_b0, eq6_e340_d_b1, eq6_e340_d_b2, eq6_e340_d_b3, eq6_e340_d_b4, eq6_e340_d_b5, eq6_e340_d_b6, eq6_e340_d_b7, eq6_e340_d_b8, eq6_e340_d_b9, eq6_e340_d_b10, eq6_e340_d_b11, eq6_e340_d_b12, eq6_e340_d_b13, eq6_e340_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e345,) = {
    if (p.p259 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e345;
        stamper.stamp_potential_const_local(
            2,
            eq7_value,
        );
        let (eq8_e351, eq8_e351_d_n0, eq8_e351_d_n1, eq8_e351_d_n2, eq8_e351_d_n3, eq8_e351_d_n4, eq8_e351_d_n5, eq8_e351_d_n6, eq8_e351_d_n7, eq8_e351_d_n8, eq8_e351_d_n9, eq8_e351_d_n10, eq8_e351_d_n11, eq8_e351_d_n12, eq8_e351_d_n13, eq8_e351_d_n14, eq8_e351_d_n15, eq8_e351_d_n16, eq8_e351_d_n17, eq8_e351_d_n18, eq8_e351_d_b0, eq8_e351_d_b1, eq8_e351_d_b2, eq8_e351_d_b3, eq8_e351_d_b4, eq8_e351_d_b5, eq8_e351_d_b6, eq8_e351_d_b7, eq8_e351_d_b8, eq8_e351_d_b9, eq8_e351_d_b10, eq8_e351_d_b11, eq8_e351_d_b12, eq8_e351_d_b13, eq8_e351_d_b14,) = {
    if (p.p260 != 0.0) {
        let eq8_e349: f64 = ((nv0 - nv6) / s.v[0]);
        let eq8_e349_d_n0: f64 = ((s.v[0] - ((nv0 - nv6) * s.dn[0][0])) / (s.v[0] * s.v[0]));
        let eq8_e349_d_n1: f64 = (-(((nv0 - nv6) * s.dn[0][1]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n2: f64 = (-(((nv0 - nv6) * s.dn[0][2]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n3: f64 = (-(((nv0 - nv6) * s.dn[0][3]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n4: f64 = (-(((nv0 - nv6) * s.dn[0][4]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n5: f64 = (-(((nv0 - nv6) * s.dn[0][5]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n6: f64 = (((-s.v[0]) - ((nv0 - nv6) * s.dn[0][6])) / (s.v[0] * s.v[0]));
        let eq8_e349_d_n7: f64 = (-(((nv0 - nv6) * s.dn[0][7]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n8: f64 = (-(((nv0 - nv6) * s.dn[0][8]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n9: f64 = (-(((nv0 - nv6) * s.dn[0][9]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n10: f64 = (-(((nv0 - nv6) * s.dn[0][10]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n11: f64 = (-(((nv0 - nv6) * s.dn[0][11]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n12: f64 = (-(((nv0 - nv6) * s.dn[0][12]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n13: f64 = (-(((nv0 - nv6) * s.dn[0][13]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n14: f64 = (-(((nv0 - nv6) * s.dn[0][14]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n15: f64 = (-(((nv0 - nv6) * s.dn[0][15]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n16: f64 = (-(((nv0 - nv6) * s.dn[0][16]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n17: f64 = (-(((nv0 - nv6) * s.dn[0][17]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n18: f64 = (-(((nv0 - nv6) * s.dn[0][18]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b0: f64 = (-(((nv0 - nv6) * s.db[0][0]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b1: f64 = (-(((nv0 - nv6) * s.db[0][1]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b2: f64 = (-(((nv0 - nv6) * s.db[0][2]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b3: f64 = (-(((nv0 - nv6) * s.db[0][3]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b4: f64 = (-(((nv0 - nv6) * s.db[0][4]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b5: f64 = (-(((nv0 - nv6) * s.db[0][5]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b6: f64 = (-(((nv0 - nv6) * s.db[0][6]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b7: f64 = (-(((nv0 - nv6) * s.db[0][7]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b8: f64 = (-(((nv0 - nv6) * s.db[0][8]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b9: f64 = (-(((nv0 - nv6) * s.db[0][9]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b10: f64 = (-(((nv0 - nv6) * s.db[0][10]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b11: f64 = (-(((nv0 - nv6) * s.db[0][11]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b12: f64 = (-(((nv0 - nv6) * s.db[0][12]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b13: f64 = (-(((nv0 - nv6) * s.db[0][13]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b14: f64 = (-(((nv0 - nv6) * s.db[0][14]) / (s.v[0] * s.v[0])));
        (eq8_e349, eq8_e349_d_n0, eq8_e349_d_n1, eq8_e349_d_n2, eq8_e349_d_n3, eq8_e349_d_n4, eq8_e349_d_n5, eq8_e349_d_n6, eq8_e349_d_n7, eq8_e349_d_n8, eq8_e349_d_n9, eq8_e349_d_n10, eq8_e349_d_n11, eq8_e349_d_n12, eq8_e349_d_n13, eq8_e349_d_n14, eq8_e349_d_n15, eq8_e349_d_n16, eq8_e349_d_n17, eq8_e349_d_n18, eq8_e349_d_b0, eq8_e349_d_b1, eq8_e349_d_b2, eq8_e349_d_b3, eq8_e349_d_b4, eq8_e349_d_b5, eq8_e349_d_b6, eq8_e349_d_b7, eq8_e349_d_b8, eq8_e349_d_b9, eq8_e349_d_b10, eq8_e349_d_b11, eq8_e349_d_b12, eq8_e349_d_b13, eq8_e349_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e351;
        let eq8_node_derivatives: [f64; 19] = [eq8_e351_d_n0, eq8_e351_d_n1, eq8_e351_d_n2, eq8_e351_d_n3, eq8_e351_d_n4, eq8_e351_d_n5, eq8_e351_d_n6, eq8_e351_d_n7, eq8_e351_d_n8, eq8_e351_d_n9, eq8_e351_d_n10, eq8_e351_d_n11, eq8_e351_d_n12, eq8_e351_d_n13, eq8_e351_d_n14, eq8_e351_d_n15, eq8_e351_d_n16, eq8_e351_d_n17, eq8_e351_d_n18];
        let eq8_branch_derivatives: [f64; 15] = [eq8_e351_d_b0, eq8_e351_d_b1, eq8_e351_d_b2, eq8_e351_d_b3, eq8_e351_d_b4, eq8_e351_d_b5, eq8_e351_d_b6, eq8_e351_d_b7, eq8_e351_d_b8, eq8_e351_d_b9, eq8_e351_d_b10, eq8_e351_d_b11, eq8_e351_d_b12, eq8_e351_d_b13, eq8_e351_d_b14];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e356,) = {
    if (p.p260 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e356;
        stamper.stamp_potential_const_local(
            3,
            eq9_value,
        );
        let eq10_e359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[594]);
        let eq10_e359_d_n0: f64 = (s.dn[594][0] * ddt_scale);
        let eq10_e359_d_n1: f64 = (s.dn[594][1] * ddt_scale);
        let eq10_e359_d_n2: f64 = (s.dn[594][2] * ddt_scale);
        let eq10_e359_d_n3: f64 = (s.dn[594][3] * ddt_scale);
        let eq10_e359_d_n4: f64 = (s.dn[594][4] * ddt_scale);
        let eq10_e359_d_n5: f64 = (s.dn[594][5] * ddt_scale);
        let eq10_e359_d_n6: f64 = (s.dn[594][6] * ddt_scale);
        let eq10_e359_d_n7: f64 = (s.dn[594][7] * ddt_scale);
        let eq10_e359_d_n8: f64 = (s.dn[594][8] * ddt_scale);
        let eq10_e359_d_n9: f64 = (s.dn[594][9] * ddt_scale);
        let eq10_e359_d_n10: f64 = (s.dn[594][10] * ddt_scale);
        let eq10_e359_d_n11: f64 = (s.dn[594][11] * ddt_scale);
        let eq10_e359_d_n12: f64 = (s.dn[594][12] * ddt_scale);
        let eq10_e359_d_n13: f64 = (s.dn[594][13] * ddt_scale);
        let eq10_e359_d_n14: f64 = (s.dn[594][14] * ddt_scale);
        let eq10_e359_d_n15: f64 = (s.dn[594][15] * ddt_scale);
        let eq10_e359_d_n16: f64 = (s.dn[594][16] * ddt_scale);
        let eq10_e359_d_n17: f64 = (s.dn[594][17] * ddt_scale);
        let eq10_e359_d_n18: f64 = (s.dn[594][18] * ddt_scale);
        let eq10_e359_d_b0: f64 = (s.db[594][0] * ddt_scale);
        let eq10_e359_d_b1: f64 = (s.db[594][1] * ddt_scale);
        let eq10_e359_d_b2: f64 = (s.db[594][2] * ddt_scale);
        let eq10_e359_d_b3: f64 = (s.db[594][3] * ddt_scale);
        let eq10_e359_d_b4: f64 = (s.db[594][4] * ddt_scale);
        let eq10_e359_d_b5: f64 = (s.db[594][5] * ddt_scale);
        let eq10_e359_d_b6: f64 = (s.db[594][6] * ddt_scale);
        let eq10_e359_d_b7: f64 = (s.db[594][7] * ddt_scale);
        let eq10_e359_d_b8: f64 = (s.db[594][8] * ddt_scale);
        let eq10_e359_d_b9: f64 = (s.db[594][9] * ddt_scale);
        let eq10_e359_d_b10: f64 = (s.db[594][10] * ddt_scale);
        let eq10_e359_d_b11: f64 = (s.db[594][11] * ddt_scale);
        let eq10_e359_d_b12: f64 = (s.db[594][12] * ddt_scale);
        let eq10_e359_d_b13: f64 = (s.db[594][13] * ddt_scale);
        let eq10_e359_d_b14: f64 = (s.db[594][14] * ddt_scale);
        let eq10_e360: f64 = (p.p50 * eq10_e359);
        let eq10_e360_d_n0: f64 = (p.p50 * eq10_e359_d_n0);
        let eq10_e360_d_n1: f64 = (p.p50 * eq10_e359_d_n1);
        let eq10_e360_d_n2: f64 = (p.p50 * eq10_e359_d_n2);
        let eq10_e360_d_n3: f64 = (p.p50 * eq10_e359_d_n3);
        let eq10_e360_d_n4: f64 = (p.p50 * eq10_e359_d_n4);
        let eq10_e360_d_n5: f64 = (p.p50 * eq10_e359_d_n5);
        let eq10_e360_d_n6: f64 = (p.p50 * eq10_e359_d_n6);
        let eq10_e360_d_n7: f64 = (p.p50 * eq10_e359_d_n7);
        let eq10_e360_d_n8: f64 = (p.p50 * eq10_e359_d_n8);
        let eq10_e360_d_n9: f64 = (p.p50 * eq10_e359_d_n9);
        let eq10_e360_d_n10: f64 = (p.p50 * eq10_e359_d_n10);
        let eq10_e360_d_n11: f64 = (p.p50 * eq10_e359_d_n11);
        let eq10_e360_d_n12: f64 = (p.p50 * eq10_e359_d_n12);
        let eq10_e360_d_n13: f64 = (p.p50 * eq10_e359_d_n13);
        let eq10_e360_d_n14: f64 = (p.p50 * eq10_e359_d_n14);
        let eq10_e360_d_n15: f64 = (p.p50 * eq10_e359_d_n15);
        let eq10_e360_d_n16: f64 = (p.p50 * eq10_e359_d_n16);
        let eq10_e360_d_n17: f64 = (p.p50 * eq10_e359_d_n17);
        let eq10_e360_d_n18: f64 = (p.p50 * eq10_e359_d_n18);
        let eq10_e360_d_b0: f64 = (p.p50 * eq10_e359_d_b0);
        let eq10_e360_d_b1: f64 = (p.p50 * eq10_e359_d_b1);
        let eq10_e360_d_b2: f64 = (p.p50 * eq10_e359_d_b2);
        let eq10_e360_d_b3: f64 = (p.p50 * eq10_e359_d_b3);
        let eq10_e360_d_b4: f64 = (p.p50 * eq10_e359_d_b4);
        let eq10_e360_d_b5: f64 = (p.p50 * eq10_e359_d_b5);
        let eq10_e360_d_b6: f64 = (p.p50 * eq10_e359_d_b6);
        let eq10_e360_d_b7: f64 = (p.p50 * eq10_e359_d_b7);
        let eq10_e360_d_b8: f64 = (p.p50 * eq10_e359_d_b8);
        let eq10_e360_d_b9: f64 = (p.p50 * eq10_e359_d_b9);
        let eq10_e360_d_b10: f64 = (p.p50 * eq10_e359_d_b10);
        let eq10_e360_d_b11: f64 = (p.p50 * eq10_e359_d_b11);
        let eq10_e360_d_b12: f64 = (p.p50 * eq10_e359_d_b12);
        let eq10_e360_d_b13: f64 = (p.p50 * eq10_e359_d_b13);
        let eq10_e360_d_b14: f64 = (p.p50 * eq10_e359_d_b14);
        let eq10_value: f64 = eq10_e360;
        let eq10_node_derivatives: [f64; 19] = [eq10_e360_d_n0, eq10_e360_d_n1, eq10_e360_d_n2, eq10_e360_d_n3, eq10_e360_d_n4, eq10_e360_d_n5, eq10_e360_d_n6, eq10_e360_d_n7, eq10_e360_d_n8, eq10_e360_d_n9, eq10_e360_d_n10, eq10_e360_d_n11, eq10_e360_d_n12, eq10_e360_d_n13, eq10_e360_d_n14, eq10_e360_d_n15, eq10_e360_d_n16, eq10_e360_d_n17, eq10_e360_d_n18];
        let eq10_branch_derivatives: [f64; 15] = [eq10_e360_d_b0, eq10_e360_d_b1, eq10_e360_d_b2, eq10_e360_d_b3, eq10_e360_d_b4, eq10_e360_d_b5, eq10_e360_d_b6, eq10_e360_d_b7, eq10_e360_d_b8, eq10_e360_d_b9, eq10_e360_d_b10, eq10_e360_d_b11, eq10_e360_d_b12, eq10_e360_d_b13, eq10_e360_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq11_e363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[198]);
        let eq11_e363_d_n0: f64 = (s.dn[198][0] * ddt_scale);
        let eq11_e363_d_n1: f64 = (s.dn[198][1] * ddt_scale);
        let eq11_e363_d_n2: f64 = (s.dn[198][2] * ddt_scale);
        let eq11_e363_d_n3: f64 = (s.dn[198][3] * ddt_scale);
        let eq11_e363_d_n4: f64 = (s.dn[198][4] * ddt_scale);
        let eq11_e363_d_n5: f64 = (s.dn[198][5] * ddt_scale);
        let eq11_e363_d_n6: f64 = (s.dn[198][6] * ddt_scale);
        let eq11_e363_d_n7: f64 = (s.dn[198][7] * ddt_scale);
        let eq11_e363_d_n8: f64 = (s.dn[198][8] * ddt_scale);
        let eq11_e363_d_n9: f64 = (s.dn[198][9] * ddt_scale);
        let eq11_e363_d_n10: f64 = (s.dn[198][10] * ddt_scale);
        let eq11_e363_d_n11: f64 = (s.dn[198][11] * ddt_scale);
        let eq11_e363_d_n12: f64 = (s.dn[198][12] * ddt_scale);
        let eq11_e363_d_n13: f64 = (s.dn[198][13] * ddt_scale);
        let eq11_e363_d_n14: f64 = (s.dn[198][14] * ddt_scale);
        let eq11_e363_d_n15: f64 = (s.dn[198][15] * ddt_scale);
        let eq11_e363_d_n16: f64 = (s.dn[198][16] * ddt_scale);
        let eq11_e363_d_n17: f64 = (s.dn[198][17] * ddt_scale);
        let eq11_e363_d_n18: f64 = (s.dn[198][18] * ddt_scale);
        let eq11_e363_d_b0: f64 = (s.db[198][0] * ddt_scale);
        let eq11_e363_d_b1: f64 = (s.db[198][1] * ddt_scale);
        let eq11_e363_d_b2: f64 = (s.db[198][2] * ddt_scale);
        let eq11_e363_d_b3: f64 = (s.db[198][3] * ddt_scale);
        let eq11_e363_d_b4: f64 = (s.db[198][4] * ddt_scale);
        let eq11_e363_d_b5: f64 = (s.db[198][5] * ddt_scale);
        let eq11_e363_d_b6: f64 = (s.db[198][6] * ddt_scale);
        let eq11_e363_d_b7: f64 = (s.db[198][7] * ddt_scale);
        let eq11_e363_d_b8: f64 = (s.db[198][8] * ddt_scale);
        let eq11_e363_d_b9: f64 = (s.db[198][9] * ddt_scale);
        let eq11_e363_d_b10: f64 = (s.db[198][10] * ddt_scale);
        let eq11_e363_d_b11: f64 = (s.db[198][11] * ddt_scale);
        let eq11_e363_d_b12: f64 = (s.db[198][12] * ddt_scale);
        let eq11_e363_d_b13: f64 = (s.db[198][13] * ddt_scale);
        let eq11_e363_d_b14: f64 = (s.db[198][14] * ddt_scale);
        let eq11_e364: f64 = (p.p50 * eq11_e363);
        let eq11_e364_d_n0: f64 = (p.p50 * eq11_e363_d_n0);
        let eq11_e364_d_n1: f64 = (p.p50 * eq11_e363_d_n1);
        let eq11_e364_d_n2: f64 = (p.p50 * eq11_e363_d_n2);
        let eq11_e364_d_n3: f64 = (p.p50 * eq11_e363_d_n3);
        let eq11_e364_d_n4: f64 = (p.p50 * eq11_e363_d_n4);
        let eq11_e364_d_n5: f64 = (p.p50 * eq11_e363_d_n5);
        let eq11_e364_d_n6: f64 = (p.p50 * eq11_e363_d_n6);
        let eq11_e364_d_n7: f64 = (p.p50 * eq11_e363_d_n7);
        let eq11_e364_d_n8: f64 = (p.p50 * eq11_e363_d_n8);
        let eq11_e364_d_n9: f64 = (p.p50 * eq11_e363_d_n9);
        let eq11_e364_d_n10: f64 = (p.p50 * eq11_e363_d_n10);
        let eq11_e364_d_n11: f64 = (p.p50 * eq11_e363_d_n11);
        let eq11_e364_d_n12: f64 = (p.p50 * eq11_e363_d_n12);
        let eq11_e364_d_n13: f64 = (p.p50 * eq11_e363_d_n13);
        let eq11_e364_d_n14: f64 = (p.p50 * eq11_e363_d_n14);
        let eq11_e364_d_n15: f64 = (p.p50 * eq11_e363_d_n15);
        let eq11_e364_d_n16: f64 = (p.p50 * eq11_e363_d_n16);
        let eq11_e364_d_n17: f64 = (p.p50 * eq11_e363_d_n17);
        let eq11_e364_d_n18: f64 = (p.p50 * eq11_e363_d_n18);
        let eq11_e364_d_b0: f64 = (p.p50 * eq11_e363_d_b0);
        let eq11_e364_d_b1: f64 = (p.p50 * eq11_e363_d_b1);
        let eq11_e364_d_b2: f64 = (p.p50 * eq11_e363_d_b2);
        let eq11_e364_d_b3: f64 = (p.p50 * eq11_e363_d_b3);
        let eq11_e364_d_b4: f64 = (p.p50 * eq11_e363_d_b4);
        let eq11_e364_d_b5: f64 = (p.p50 * eq11_e363_d_b5);
        let eq11_e364_d_b6: f64 = (p.p50 * eq11_e363_d_b6);
        let eq11_e364_d_b7: f64 = (p.p50 * eq11_e363_d_b7);
        let eq11_e364_d_b8: f64 = (p.p50 * eq11_e363_d_b8);
        let eq11_e364_d_b9: f64 = (p.p50 * eq11_e363_d_b9);
        let eq11_e364_d_b10: f64 = (p.p50 * eq11_e363_d_b10);
        let eq11_e364_d_b11: f64 = (p.p50 * eq11_e363_d_b11);
        let eq11_e364_d_b12: f64 = (p.p50 * eq11_e363_d_b12);
        let eq11_e364_d_b13: f64 = (p.p50 * eq11_e363_d_b13);
        let eq11_e364_d_b14: f64 = (p.p50 * eq11_e363_d_b14);
        let eq11_value: f64 = eq11_e364;
        let eq11_node_derivatives: [f64; 19] = [eq11_e364_d_n0, eq11_e364_d_n1, eq11_e364_d_n2, eq11_e364_d_n3, eq11_e364_d_n4, eq11_e364_d_n5, eq11_e364_d_n6, eq11_e364_d_n7, eq11_e364_d_n8, eq11_e364_d_n9, eq11_e364_d_n10, eq11_e364_d_n11, eq11_e364_d_n12, eq11_e364_d_n13, eq11_e364_d_n14, eq11_e364_d_n15, eq11_e364_d_n16, eq11_e364_d_n17, eq11_e364_d_n18];
        let eq11_branch_derivatives: [f64; 15] = [eq11_e364_d_b0, eq11_e364_d_b1, eq11_e364_d_b2, eq11_e364_d_b3, eq11_e364_d_b4, eq11_e364_d_b5, eq11_e364_d_b6, eq11_e364_d_b7, eq11_e364_d_b8, eq11_e364_d_b9, eq11_e364_d_b10, eq11_e364_d_b11, eq11_e364_d_b12, eq11_e364_d_b13, eq11_e364_d_b14];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[196]);
        let eq12_e367_d_n0: f64 = (s.dn[196][0] * ddt_scale);
        let eq12_e367_d_n1: f64 = (s.dn[196][1] * ddt_scale);
        let eq12_e367_d_n2: f64 = (s.dn[196][2] * ddt_scale);
        let eq12_e367_d_n3: f64 = (s.dn[196][3] * ddt_scale);
        let eq12_e367_d_n4: f64 = (s.dn[196][4] * ddt_scale);
        let eq12_e367_d_n5: f64 = (s.dn[196][5] * ddt_scale);
        let eq12_e367_d_n6: f64 = (s.dn[196][6] * ddt_scale);
        let eq12_e367_d_n7: f64 = (s.dn[196][7] * ddt_scale);
        let eq12_e367_d_n8: f64 = (s.dn[196][8] * ddt_scale);
        let eq12_e367_d_n9: f64 = (s.dn[196][9] * ddt_scale);
        let eq12_e367_d_n10: f64 = (s.dn[196][10] * ddt_scale);
        let eq12_e367_d_n11: f64 = (s.dn[196][11] * ddt_scale);
        let eq12_e367_d_n12: f64 = (s.dn[196][12] * ddt_scale);
        let eq12_e367_d_n13: f64 = (s.dn[196][13] * ddt_scale);
        let eq12_e367_d_n14: f64 = (s.dn[196][14] * ddt_scale);
        let eq12_e367_d_n15: f64 = (s.dn[196][15] * ddt_scale);
        let eq12_e367_d_n16: f64 = (s.dn[196][16] * ddt_scale);
        let eq12_e367_d_n17: f64 = (s.dn[196][17] * ddt_scale);
        let eq12_e367_d_n18: f64 = (s.dn[196][18] * ddt_scale);
        let eq12_e367_d_b0: f64 = (s.db[196][0] * ddt_scale);
        let eq12_e367_d_b1: f64 = (s.db[196][1] * ddt_scale);
        let eq12_e367_d_b2: f64 = (s.db[196][2] * ddt_scale);
        let eq12_e367_d_b3: f64 = (s.db[196][3] * ddt_scale);
        let eq12_e367_d_b4: f64 = (s.db[196][4] * ddt_scale);
        let eq12_e367_d_b5: f64 = (s.db[196][5] * ddt_scale);
        let eq12_e367_d_b6: f64 = (s.db[196][6] * ddt_scale);
        let eq12_e367_d_b7: f64 = (s.db[196][7] * ddt_scale);
        let eq12_e367_d_b8: f64 = (s.db[196][8] * ddt_scale);
        let eq12_e367_d_b9: f64 = (s.db[196][9] * ddt_scale);
        let eq12_e367_d_b10: f64 = (s.db[196][10] * ddt_scale);
        let eq12_e367_d_b11: f64 = (s.db[196][11] * ddt_scale);
        let eq12_e367_d_b12: f64 = (s.db[196][12] * ddt_scale);
        let eq12_e367_d_b13: f64 = (s.db[196][13] * ddt_scale);
        let eq12_e367_d_b14: f64 = (s.db[196][14] * ddt_scale);
        let eq12_e368: f64 = (p.p50 * eq12_e367);
        let eq12_e368_d_n0: f64 = (p.p50 * eq12_e367_d_n0);
        let eq12_e368_d_n1: f64 = (p.p50 * eq12_e367_d_n1);
        let eq12_e368_d_n2: f64 = (p.p50 * eq12_e367_d_n2);
        let eq12_e368_d_n3: f64 = (p.p50 * eq12_e367_d_n3);
        let eq12_e368_d_n4: f64 = (p.p50 * eq12_e367_d_n4);
        let eq12_e368_d_n5: f64 = (p.p50 * eq12_e367_d_n5);
        let eq12_e368_d_n6: f64 = (p.p50 * eq12_e367_d_n6);
        let eq12_e368_d_n7: f64 = (p.p50 * eq12_e367_d_n7);
        let eq12_e368_d_n8: f64 = (p.p50 * eq12_e367_d_n8);
        let eq12_e368_d_n9: f64 = (p.p50 * eq12_e367_d_n9);
        let eq12_e368_d_n10: f64 = (p.p50 * eq12_e367_d_n10);
        let eq12_e368_d_n11: f64 = (p.p50 * eq12_e367_d_n11);
        let eq12_e368_d_n12: f64 = (p.p50 * eq12_e367_d_n12);
        let eq12_e368_d_n13: f64 = (p.p50 * eq12_e367_d_n13);
        let eq12_e368_d_n14: f64 = (p.p50 * eq12_e367_d_n14);
        let eq12_e368_d_n15: f64 = (p.p50 * eq12_e367_d_n15);
        let eq12_e368_d_n16: f64 = (p.p50 * eq12_e367_d_n16);
        let eq12_e368_d_n17: f64 = (p.p50 * eq12_e367_d_n17);
        let eq12_e368_d_n18: f64 = (p.p50 * eq12_e367_d_n18);
        let eq12_e368_d_b0: f64 = (p.p50 * eq12_e367_d_b0);
        let eq12_e368_d_b1: f64 = (p.p50 * eq12_e367_d_b1);
        let eq12_e368_d_b2: f64 = (p.p50 * eq12_e367_d_b2);
        let eq12_e368_d_b3: f64 = (p.p50 * eq12_e367_d_b3);
        let eq12_e368_d_b4: f64 = (p.p50 * eq12_e367_d_b4);
        let eq12_e368_d_b5: f64 = (p.p50 * eq12_e367_d_b5);
        let eq12_e368_d_b6: f64 = (p.p50 * eq12_e367_d_b6);
        let eq12_e368_d_b7: f64 = (p.p50 * eq12_e367_d_b7);
        let eq12_e368_d_b8: f64 = (p.p50 * eq12_e367_d_b8);
        let eq12_e368_d_b9: f64 = (p.p50 * eq12_e367_d_b9);
        let eq12_e368_d_b10: f64 = (p.p50 * eq12_e367_d_b10);
        let eq12_e368_d_b11: f64 = (p.p50 * eq12_e367_d_b11);
        let eq12_e368_d_b12: f64 = (p.p50 * eq12_e367_d_b12);
        let eq12_e368_d_b13: f64 = (p.p50 * eq12_e367_d_b13);
        let eq12_e368_d_b14: f64 = (p.p50 * eq12_e367_d_b14);
        let eq12_value: f64 = eq12_e368;
        let eq12_node_derivatives: [f64; 19] = [eq12_e368_d_n0, eq12_e368_d_n1, eq12_e368_d_n2, eq12_e368_d_n3, eq12_e368_d_n4, eq12_e368_d_n5, eq12_e368_d_n6, eq12_e368_d_n7, eq12_e368_d_n8, eq12_e368_d_n9, eq12_e368_d_n10, eq12_e368_d_n11, eq12_e368_d_n12, eq12_e368_d_n13, eq12_e368_d_n14, eq12_e368_d_n15, eq12_e368_d_n16, eq12_e368_d_n17, eq12_e368_d_n18];
        let eq12_branch_derivatives: [f64; 15] = [eq12_e368_d_b0, eq12_e368_d_b1, eq12_e368_d_b2, eq12_e368_d_b3, eq12_e368_d_b4, eq12_e368_d_b5, eq12_e368_d_b6, eq12_e368_d_b7, eq12_e368_d_b8, eq12_e368_d_b9, eq12_e368_d_b10, eq12_e368_d_b11, eq12_e368_d_b12, eq12_e368_d_b13, eq12_e368_d_b14];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (eq13_value),
        );
        let eq14_e379: f64 = (nv14 - 0.0);
        let eq14_value: f64 = eq14_e379;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq14_value),
            14,
            multiplicity * (1.0),
        );
        let eq15_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (eq15_value),
        );
        let eq16_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (eq16_value),
        );
        let eq17_e394: f64 = (s.v[614] * (nv14 - 0.0));
        let eq17_e394_d_n0: f64 = (s.dn[614][0] * (nv14 - 0.0));
        let eq17_e394_d_n1: f64 = (s.dn[614][1] * (nv14 - 0.0));
        let eq17_e394_d_n2: f64 = (s.dn[614][2] * (nv14 - 0.0));
        let eq17_e394_d_n3: f64 = (s.dn[614][3] * (nv14 - 0.0));
        let eq17_e394_d_n4: f64 = (s.dn[614][4] * (nv14 - 0.0));
        let eq17_e394_d_n5: f64 = (s.dn[614][5] * (nv14 - 0.0));
        let eq17_e394_d_n6: f64 = (s.dn[614][6] * (nv14 - 0.0));
        let eq17_e394_d_n7: f64 = (s.dn[614][7] * (nv14 - 0.0));
        let eq17_e394_d_n8: f64 = (s.dn[614][8] * (nv14 - 0.0));
        let eq17_e394_d_n9: f64 = (s.dn[614][9] * (nv14 - 0.0));
        let eq17_e394_d_n10: f64 = (s.dn[614][10] * (nv14 - 0.0));
        let eq17_e394_d_n11: f64 = (s.dn[614][11] * (nv14 - 0.0));
        let eq17_e394_d_n12: f64 = (s.dn[614][12] * (nv14 - 0.0));
        let eq17_e394_d_n13: f64 = (s.dn[614][13] * (nv14 - 0.0));
        let eq17_e394_d_n14: f64 = ((s.dn[614][14] * (nv14 - 0.0)) + s.v[614]);
        let eq17_e394_d_n15: f64 = (s.dn[614][15] * (nv14 - 0.0));
        let eq17_e394_d_n16: f64 = (s.dn[614][16] * (nv14 - 0.0));
        let eq17_e394_d_n17: f64 = (s.dn[614][17] * (nv14 - 0.0));
        let eq17_e394_d_n18: f64 = (s.dn[614][18] * (nv14 - 0.0));
        let eq17_e394_d_b0: f64 = (s.db[614][0] * (nv14 - 0.0));
        let eq17_e394_d_b1: f64 = (s.db[614][1] * (nv14 - 0.0));
        let eq17_e394_d_b2: f64 = (s.db[614][2] * (nv14 - 0.0));
        let eq17_e394_d_b3: f64 = (s.db[614][3] * (nv14 - 0.0));
        let eq17_e394_d_b4: f64 = (s.db[614][4] * (nv14 - 0.0));
        let eq17_e394_d_b5: f64 = (s.db[614][5] * (nv14 - 0.0));
        let eq17_e394_d_b6: f64 = (s.db[614][6] * (nv14 - 0.0));
        let eq17_e394_d_b7: f64 = (s.db[614][7] * (nv14 - 0.0));
        let eq17_e394_d_b8: f64 = (s.db[614][8] * (nv14 - 0.0));
        let eq17_e394_d_b9: f64 = (s.db[614][9] * (nv14 - 0.0));
        let eq17_e394_d_b10: f64 = (s.db[614][10] * (nv14 - 0.0));
        let eq17_e394_d_b11: f64 = (s.db[614][11] * (nv14 - 0.0));
        let eq17_e394_d_b12: f64 = (s.db[614][12] * (nv14 - 0.0));
        let eq17_e394_d_b13: f64 = (s.db[614][13] * (nv14 - 0.0));
        let eq17_e394_d_b14: f64 = (s.db[614][14] * (nv14 - 0.0));
        let eq17_value: f64 = eq17_e394;
        let eq17_node_derivatives: [f64; 19] = [eq17_e394_d_n0, eq17_e394_d_n1, eq17_e394_d_n2, eq17_e394_d_n3, eq17_e394_d_n4, eq17_e394_d_n5, eq17_e394_d_n6, eq17_e394_d_n7, eq17_e394_d_n8, eq17_e394_d_n9, eq17_e394_d_n10, eq17_e394_d_n11, eq17_e394_d_n12, eq17_e394_d_n13, eq17_e394_d_n14, eq17_e394_d_n15, eq17_e394_d_n16, eq17_e394_d_n17, eq17_e394_d_n18];
        let eq17_branch_derivatives: [f64; 15] = [eq17_e394_d_b0, eq17_e394_d_b1, eq17_e394_d_b2, eq17_e394_d_b3, eq17_e394_d_b4, eq17_e394_d_b5, eq17_e394_d_b6, eq17_e394_d_b7, eq17_e394_d_b8, eq17_e394_d_b9, eq17_e394_d_b10, eq17_e394_d_b11, eq17_e394_d_b12, eq17_e394_d_b13, eq17_e394_d_b14];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
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
        let eq18_e398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq18_e397);
        let eq18_e398_d_n0: f64 = (eq18_e397_d_n0 * ddt_scale);
        let eq18_e398_d_n1: f64 = (eq18_e397_d_n1 * ddt_scale);
        let eq18_e398_d_n2: f64 = (eq18_e397_d_n2 * ddt_scale);
        let eq18_e398_d_n3: f64 = (eq18_e397_d_n3 * ddt_scale);
        let eq18_e398_d_n4: f64 = (eq18_e397_d_n4 * ddt_scale);
        let eq18_e398_d_n5: f64 = (eq18_e397_d_n5 * ddt_scale);
        let eq18_e398_d_n6: f64 = (eq18_e397_d_n6 * ddt_scale);
        let eq18_e398_d_n7: f64 = (eq18_e397_d_n7 * ddt_scale);
        let eq18_e398_d_n8: f64 = (eq18_e397_d_n8 * ddt_scale);
        let eq18_e398_d_n9: f64 = (eq18_e397_d_n9 * ddt_scale);
        let eq18_e398_d_n10: f64 = (eq18_e397_d_n10 * ddt_scale);
        let eq18_e398_d_n11: f64 = (eq18_e397_d_n11 * ddt_scale);
        let eq18_e398_d_n12: f64 = (eq18_e397_d_n12 * ddt_scale);
        let eq18_e398_d_n13: f64 = (eq18_e397_d_n13 * ddt_scale);
        let eq18_e398_d_n14: f64 = (eq18_e397_d_n14 * ddt_scale);
        let eq18_e398_d_n15: f64 = (eq18_e397_d_n15 * ddt_scale);
        let eq18_e398_d_n16: f64 = (eq18_e397_d_n16 * ddt_scale);
        let eq18_e398_d_n17: f64 = (eq18_e397_d_n17 * ddt_scale);
        let eq18_e398_d_n18: f64 = (eq18_e397_d_n18 * ddt_scale);
        let eq18_e398_d_b0: f64 = (eq18_e397_d_b0 * ddt_scale);
        let eq18_e398_d_b1: f64 = (eq18_e397_d_b1 * ddt_scale);
        let eq18_e398_d_b2: f64 = (eq18_e397_d_b2 * ddt_scale);
        let eq18_e398_d_b3: f64 = (eq18_e397_d_b3 * ddt_scale);
        let eq18_e398_d_b4: f64 = (eq18_e397_d_b4 * ddt_scale);
        let eq18_e398_d_b5: f64 = (eq18_e397_d_b5 * ddt_scale);
        let eq18_e398_d_b6: f64 = (eq18_e397_d_b6 * ddt_scale);
        let eq18_e398_d_b7: f64 = (eq18_e397_d_b7 * ddt_scale);
        let eq18_e398_d_b8: f64 = (eq18_e397_d_b8 * ddt_scale);
        let eq18_e398_d_b9: f64 = (eq18_e397_d_b9 * ddt_scale);
        let eq18_e398_d_b10: f64 = (eq18_e397_d_b10 * ddt_scale);
        let eq18_e398_d_b11: f64 = (eq18_e397_d_b11 * ddt_scale);
        let eq18_e398_d_b12: f64 = (eq18_e397_d_b12 * ddt_scale);
        let eq18_e398_d_b13: f64 = (eq18_e397_d_b13 * ddt_scale);
        let eq18_e398_d_b14: f64 = (eq18_e397_d_b14 * ddt_scale);
        let eq18_value: f64 = eq18_e398;
        let eq18_node_derivatives: [f64; 19] = [eq18_e398_d_n0, eq18_e398_d_n1, eq18_e398_d_n2, eq18_e398_d_n3, eq18_e398_d_n4, eq18_e398_d_n5, eq18_e398_d_n6, eq18_e398_d_n7, eq18_e398_d_n8, eq18_e398_d_n9, eq18_e398_d_n10, eq18_e398_d_n11, eq18_e398_d_n12, eq18_e398_d_n13, eq18_e398_d_n14, eq18_e398_d_n15, eq18_e398_d_n16, eq18_e398_d_n17, eq18_e398_d_n18];
        let eq18_branch_derivatives: [f64; 15] = [eq18_e398_d_b0, eq18_e398_d_b1, eq18_e398_d_b2, eq18_e398_d_b3, eq18_e398_d_b4, eq18_e398_d_b5, eq18_e398_d_b6, eq18_e398_d_b7, eq18_e398_d_b8, eq18_e398_d_b9, eq18_e398_d_b10, eq18_e398_d_b11, eq18_e398_d_b12, eq18_e398_d_b13, eq18_e398_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
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
        let eq19_e402: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq19_e401);
        let eq19_e402_d_n0: f64 = (eq19_e401_d_n0 * ddt_scale);
        let eq19_e402_d_n1: f64 = (eq19_e401_d_n1 * ddt_scale);
        let eq19_e402_d_n2: f64 = (eq19_e401_d_n2 * ddt_scale);
        let eq19_e402_d_n3: f64 = (eq19_e401_d_n3 * ddt_scale);
        let eq19_e402_d_n4: f64 = (eq19_e401_d_n4 * ddt_scale);
        let eq19_e402_d_n5: f64 = (eq19_e401_d_n5 * ddt_scale);
        let eq19_e402_d_n6: f64 = (eq19_e401_d_n6 * ddt_scale);
        let eq19_e402_d_n7: f64 = (eq19_e401_d_n7 * ddt_scale);
        let eq19_e402_d_n8: f64 = (eq19_e401_d_n8 * ddt_scale);
        let eq19_e402_d_n9: f64 = (eq19_e401_d_n9 * ddt_scale);
        let eq19_e402_d_n10: f64 = (eq19_e401_d_n10 * ddt_scale);
        let eq19_e402_d_n11: f64 = (eq19_e401_d_n11 * ddt_scale);
        let eq19_e402_d_n12: f64 = (eq19_e401_d_n12 * ddt_scale);
        let eq19_e402_d_n13: f64 = (eq19_e401_d_n13 * ddt_scale);
        let eq19_e402_d_n14: f64 = (eq19_e401_d_n14 * ddt_scale);
        let eq19_e402_d_n15: f64 = (eq19_e401_d_n15 * ddt_scale);
        let eq19_e402_d_n16: f64 = (eq19_e401_d_n16 * ddt_scale);
        let eq19_e402_d_n17: f64 = (eq19_e401_d_n17 * ddt_scale);
        let eq19_e402_d_n18: f64 = (eq19_e401_d_n18 * ddt_scale);
        let eq19_e402_d_b0: f64 = (eq19_e401_d_b0 * ddt_scale);
        let eq19_e402_d_b1: f64 = (eq19_e401_d_b1 * ddt_scale);
        let eq19_e402_d_b2: f64 = (eq19_e401_d_b2 * ddt_scale);
        let eq19_e402_d_b3: f64 = (eq19_e401_d_b3 * ddt_scale);
        let eq19_e402_d_b4: f64 = (eq19_e401_d_b4 * ddt_scale);
        let eq19_e402_d_b5: f64 = (eq19_e401_d_b5 * ddt_scale);
        let eq19_e402_d_b6: f64 = (eq19_e401_d_b6 * ddt_scale);
        let eq19_e402_d_b7: f64 = (eq19_e401_d_b7 * ddt_scale);
        let eq19_e402_d_b8: f64 = (eq19_e401_d_b8 * ddt_scale);
        let eq19_e402_d_b9: f64 = (eq19_e401_d_b9 * ddt_scale);
        let eq19_e402_d_b10: f64 = (eq19_e401_d_b10 * ddt_scale);
        let eq19_e402_d_b11: f64 = (eq19_e401_d_b11 * ddt_scale);
        let eq19_e402_d_b12: f64 = (eq19_e401_d_b12 * ddt_scale);
        let eq19_e402_d_b13: f64 = (eq19_e401_d_b13 * ddt_scale);
        let eq19_e402_d_b14: f64 = (eq19_e401_d_b14 * ddt_scale);
        let eq19_value: f64 = eq19_e402;
        let eq19_node_derivatives: [f64; 19] = [eq19_e402_d_n0, eq19_e402_d_n1, eq19_e402_d_n2, eq19_e402_d_n3, eq19_e402_d_n4, eq19_e402_d_n5, eq19_e402_d_n6, eq19_e402_d_n7, eq19_e402_d_n8, eq19_e402_d_n9, eq19_e402_d_n10, eq19_e402_d_n11, eq19_e402_d_n12, eq19_e402_d_n13, eq19_e402_d_n14, eq19_e402_d_n15, eq19_e402_d_n16, eq19_e402_d_n17, eq19_e402_d_n18];
        let eq19_branch_derivatives: [f64; 15] = [eq19_e402_d_b0, eq19_e402_d_b1, eq19_e402_d_b2, eq19_e402_d_b3, eq19_e402_d_b4, eq19_e402_d_b5, eq19_e402_d_b6, eq19_e402_d_b7, eq19_e402_d_b8, eq19_e402_d_b9, eq19_e402_d_b10, eq19_e402_d_b11, eq19_e402_d_b12, eq19_e402_d_b13, eq19_e402_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e410,) = {
    if (p.p259 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e410;
        stamper.stamp_current_const_local(
            Some(7),
            Some(2),
            multiplicity * (eq20_value),
        );
        let (eq21_e418,) = {
    if (p.p260 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e418;
        stamper.stamp_current_const_local(
            Some(0),
            Some(6),
            multiplicity * (eq21_value),
        );
        let (eq22_e428,) = {
    if s.b[1847] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e428;
        stamper.stamp_current_const_local(
            Some(11),
            Some(6),
            multiplicity * (eq22_value),
        );
        let (eq23_e438,) = {
    if s.b[1847] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e438;
        stamper.stamp_current_const_local(
            Some(11),
            Some(7),
            multiplicity * (eq23_value),
        );
        let (eq24_e448,) = {
    if s.b[1847] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e448;
        stamper.stamp_current_const_local(
            Some(11),
            Some(12),
            multiplicity * (eq24_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq25_e454, eq25_e454_d_n0, eq25_e454_d_n1, eq25_e454_d_n2, eq25_e454_d_n3, eq25_e454_d_n4, eq25_e454_d_n5, eq25_e454_d_n6, eq25_e454_d_n7, eq25_e454_d_n8, eq25_e454_d_n9, eq25_e454_d_n10, eq25_e454_d_n11, eq25_e454_d_n12, eq25_e454_d_n13, eq25_e454_d_n14, eq25_e454_d_n15, eq25_e454_d_n16, eq25_e454_d_n17, eq25_e454_d_n18, eq25_e454_d_b0, eq25_e454_d_b1, eq25_e454_d_b2, eq25_e454_d_b3, eq25_e454_d_b4, eq25_e454_d_b5, eq25_e454_d_b6, eq25_e454_d_b7, eq25_e454_d_b8, eq25_e454_d_b9, eq25_e454_d_b10, eq25_e454_d_b11, eq25_e454_d_b12, eq25_e454_d_b13, eq25_e454_d_b14,) = {
    if (p.p35 != 0.0) {
        let eq25_e452: f64 = (s.v[551] * (nv1 - nv11));
        let eq25_e452_d_n0: f64 = (s.dn[551][0] * (nv1 - nv11));
        let eq25_e452_d_n1: f64 = ((s.dn[551][1] * (nv1 - nv11)) + s.v[551]);
        let eq25_e452_d_n2: f64 = (s.dn[551][2] * (nv1 - nv11));
        let eq25_e452_d_n3: f64 = (s.dn[551][3] * (nv1 - nv11));
        let eq25_e452_d_n4: f64 = (s.dn[551][4] * (nv1 - nv11));
        let eq25_e452_d_n5: f64 = (s.dn[551][5] * (nv1 - nv11));
        let eq25_e452_d_n6: f64 = (s.dn[551][6] * (nv1 - nv11));
        let eq25_e452_d_n7: f64 = (s.dn[551][7] * (nv1 - nv11));
        let eq25_e452_d_n8: f64 = (s.dn[551][8] * (nv1 - nv11));
        let eq25_e452_d_n9: f64 = (s.dn[551][9] * (nv1 - nv11));
        let eq25_e452_d_n10: f64 = (s.dn[551][10] * (nv1 - nv11));
        let eq25_e452_d_n11: f64 = ((s.dn[551][11] * (nv1 - nv11)) + (-s.v[551]));
        let eq25_e452_d_n12: f64 = (s.dn[551][12] * (nv1 - nv11));
        let eq25_e452_d_n13: f64 = (s.dn[551][13] * (nv1 - nv11));
        let eq25_e452_d_n14: f64 = (s.dn[551][14] * (nv1 - nv11));
        let eq25_e452_d_n15: f64 = (s.dn[551][15] * (nv1 - nv11));
        let eq25_e452_d_n16: f64 = (s.dn[551][16] * (nv1 - nv11));
        let eq25_e452_d_n17: f64 = (s.dn[551][17] * (nv1 - nv11));
        let eq25_e452_d_n18: f64 = (s.dn[551][18] * (nv1 - nv11));
        let eq25_e452_d_b0: f64 = (s.db[551][0] * (nv1 - nv11));
        let eq25_e452_d_b1: f64 = (s.db[551][1] * (nv1 - nv11));
        let eq25_e452_d_b2: f64 = (s.db[551][2] * (nv1 - nv11));
        let eq25_e452_d_b3: f64 = (s.db[551][3] * (nv1 - nv11));
        let eq25_e452_d_b4: f64 = (s.db[551][4] * (nv1 - nv11));
        let eq25_e452_d_b5: f64 = (s.db[551][5] * (nv1 - nv11));
        let eq25_e452_d_b6: f64 = (s.db[551][6] * (nv1 - nv11));
        let eq25_e452_d_b7: f64 = (s.db[551][7] * (nv1 - nv11));
        let eq25_e452_d_b8: f64 = (s.db[551][8] * (nv1 - nv11));
        let eq25_e452_d_b9: f64 = (s.db[551][9] * (nv1 - nv11));
        let eq25_e452_d_b10: f64 = (s.db[551][10] * (nv1 - nv11));
        let eq25_e452_d_b11: f64 = (s.db[551][11] * (nv1 - nv11));
        let eq25_e452_d_b12: f64 = (s.db[551][12] * (nv1 - nv11));
        let eq25_e452_d_b13: f64 = (s.db[551][13] * (nv1 - nv11));
        let eq25_e452_d_b14: f64 = (s.db[551][14] * (nv1 - nv11));
        (eq25_e452, eq25_e452_d_n0, eq25_e452_d_n1, eq25_e452_d_n2, eq25_e452_d_n3, eq25_e452_d_n4, eq25_e452_d_n5, eq25_e452_d_n6, eq25_e452_d_n7, eq25_e452_d_n8, eq25_e452_d_n9, eq25_e452_d_n10, eq25_e452_d_n11, eq25_e452_d_n12, eq25_e452_d_n13, eq25_e452_d_n14, eq25_e452_d_n15, eq25_e452_d_n16, eq25_e452_d_n17, eq25_e452_d_n18, eq25_e452_d_b0, eq25_e452_d_b1, eq25_e452_d_b2, eq25_e452_d_b3, eq25_e452_d_b4, eq25_e452_d_b5, eq25_e452_d_b6, eq25_e452_d_b7, eq25_e452_d_b8, eq25_e452_d_b9, eq25_e452_d_b10, eq25_e452_d_b11, eq25_e452_d_b12, eq25_e452_d_b13, eq25_e452_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e454;
        let eq25_node_derivatives: [f64; 19] = [eq25_e454_d_n0, eq25_e454_d_n1, eq25_e454_d_n2, eq25_e454_d_n3, eq25_e454_d_n4, eq25_e454_d_n5, eq25_e454_d_n6, eq25_e454_d_n7, eq25_e454_d_n8, eq25_e454_d_n9, eq25_e454_d_n10, eq25_e454_d_n11, eq25_e454_d_n12, eq25_e454_d_n13, eq25_e454_d_n14, eq25_e454_d_n15, eq25_e454_d_n16, eq25_e454_d_n17, eq25_e454_d_n18];
        let eq25_branch_derivatives: [f64; 15] = [eq25_e454_d_b0, eq25_e454_d_b1, eq25_e454_d_b2, eq25_e454_d_b3, eq25_e454_d_b4, eq25_e454_d_b5, eq25_e454_d_b6, eq25_e454_d_b7, eq25_e454_d_b8, eq25_e454_d_b9, eq25_e454_d_b10, eq25_e454_d_b11, eq25_e454_d_b12, eq25_e454_d_b13, eq25_e454_d_b14];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(11),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e459,) = {
    if (p.p35 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e459;
        stamper.stamp_potential_const_local(
            4,
            eq26_value,
        );
        let (eq27_e465, eq27_e465_d_n0, eq27_e465_d_n1, eq27_e465_d_n2, eq27_e465_d_n3, eq27_e465_d_n4, eq27_e465_d_n5, eq27_e465_d_n6, eq27_e465_d_n7, eq27_e465_d_n8, eq27_e465_d_n9, eq27_e465_d_n10, eq27_e465_d_n11, eq27_e465_d_n12, eq27_e465_d_n13, eq27_e465_d_n14, eq27_e465_d_n15, eq27_e465_d_n16, eq27_e465_d_n17, eq27_e465_d_n18, eq27_e465_d_b0, eq27_e465_d_b1, eq27_e465_d_b2, eq27_e465_d_b3, eq27_e465_d_b4, eq27_e465_d_b5, eq27_e465_d_b6, eq27_e465_d_b7, eq27_e465_d_b8, eq27_e465_d_b9, eq27_e465_d_b10, eq27_e465_d_b11, eq27_e465_d_b12, eq27_e465_d_b13, eq27_e465_d_b14,) = {
    if s.b[1848] {
        let eq27_e463: f64 = ((nv10 - 0.0) * s.v[589]);
        let eq27_e463_d_n0: f64 = ((nv10 - 0.0) * s.dn[589][0]);
        let eq27_e463_d_n1: f64 = ((nv10 - 0.0) * s.dn[589][1]);
        let eq27_e463_d_n2: f64 = ((nv10 - 0.0) * s.dn[589][2]);
        let eq27_e463_d_n3: f64 = ((nv10 - 0.0) * s.dn[589][3]);
        let eq27_e463_d_n4: f64 = ((nv10 - 0.0) * s.dn[589][4]);
        let eq27_e463_d_n5: f64 = ((nv10 - 0.0) * s.dn[589][5]);
        let eq27_e463_d_n6: f64 = ((nv10 - 0.0) * s.dn[589][6]);
        let eq27_e463_d_n7: f64 = ((nv10 - 0.0) * s.dn[589][7]);
        let eq27_e463_d_n8: f64 = ((nv10 - 0.0) * s.dn[589][8]);
        let eq27_e463_d_n9: f64 = ((nv10 - 0.0) * s.dn[589][9]);
        let eq27_e463_d_n10: f64 = (s.v[589] + ((nv10 - 0.0) * s.dn[589][10]));
        let eq27_e463_d_n11: f64 = ((nv10 - 0.0) * s.dn[589][11]);
        let eq27_e463_d_n12: f64 = ((nv10 - 0.0) * s.dn[589][12]);
        let eq27_e463_d_n13: f64 = ((nv10 - 0.0) * s.dn[589][13]);
        let eq27_e463_d_n14: f64 = ((nv10 - 0.0) * s.dn[589][14]);
        let eq27_e463_d_n15: f64 = ((nv10 - 0.0) * s.dn[589][15]);
        let eq27_e463_d_n16: f64 = ((nv10 - 0.0) * s.dn[589][16]);
        let eq27_e463_d_n17: f64 = ((nv10 - 0.0) * s.dn[589][17]);
        let eq27_e463_d_n18: f64 = ((nv10 - 0.0) * s.dn[589][18]);
        let eq27_e463_d_b0: f64 = ((nv10 - 0.0) * s.db[589][0]);
        let eq27_e463_d_b1: f64 = ((nv10 - 0.0) * s.db[589][1]);
        let eq27_e463_d_b2: f64 = ((nv10 - 0.0) * s.db[589][2]);
        let eq27_e463_d_b3: f64 = ((nv10 - 0.0) * s.db[589][3]);
        let eq27_e463_d_b4: f64 = ((nv10 - 0.0) * s.db[589][4]);
        let eq27_e463_d_b5: f64 = ((nv10 - 0.0) * s.db[589][5]);
        let eq27_e463_d_b6: f64 = ((nv10 - 0.0) * s.db[589][6]);
        let eq27_e463_d_b7: f64 = ((nv10 - 0.0) * s.db[589][7]);
        let eq27_e463_d_b8: f64 = ((nv10 - 0.0) * s.db[589][8]);
        let eq27_e463_d_b9: f64 = ((nv10 - 0.0) * s.db[589][9]);
        let eq27_e463_d_b10: f64 = ((nv10 - 0.0) * s.db[589][10]);
        let eq27_e463_d_b11: f64 = ((nv10 - 0.0) * s.db[589][11]);
        let eq27_e463_d_b12: f64 = ((nv10 - 0.0) * s.db[589][12]);
        let eq27_e463_d_b13: f64 = ((nv10 - 0.0) * s.db[589][13]);
        let eq27_e463_d_b14: f64 = ((nv10 - 0.0) * s.db[589][14]);
        (eq27_e463, eq27_e463_d_n0, eq27_e463_d_n1, eq27_e463_d_n2, eq27_e463_d_n3, eq27_e463_d_n4, eq27_e463_d_n5, eq27_e463_d_n6, eq27_e463_d_n7, eq27_e463_d_n8, eq27_e463_d_n9, eq27_e463_d_n10, eq27_e463_d_n11, eq27_e463_d_n12, eq27_e463_d_n13, eq27_e463_d_n14, eq27_e463_d_n15, eq27_e463_d_n16, eq27_e463_d_n17, eq27_e463_d_n18, eq27_e463_d_b0, eq27_e463_d_b1, eq27_e463_d_b2, eq27_e463_d_b3, eq27_e463_d_b4, eq27_e463_d_b5, eq27_e463_d_b6, eq27_e463_d_b7, eq27_e463_d_b8, eq27_e463_d_b9, eq27_e463_d_b10, eq27_e463_d_b11, eq27_e463_d_b12, eq27_e463_d_b13, eq27_e463_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e465;
        let eq27_node_derivatives: [f64; 19] = [eq27_e465_d_n0, eq27_e465_d_n1, eq27_e465_d_n2, eq27_e465_d_n3, eq27_e465_d_n4, eq27_e465_d_n5, eq27_e465_d_n6, eq27_e465_d_n7, eq27_e465_d_n8, eq27_e465_d_n9, eq27_e465_d_n10, eq27_e465_d_n11, eq27_e465_d_n12, eq27_e465_d_n13, eq27_e465_d_n14, eq27_e465_d_n15, eq27_e465_d_n16, eq27_e465_d_n17, eq27_e465_d_n18];
        let eq27_branch_derivatives: [f64; 15] = [eq27_e465_d_b0, eq27_e465_d_b1, eq27_e465_d_b2, eq27_e465_d_b3, eq27_e465_d_b4, eq27_e465_d_b5, eq27_e465_d_b6, eq27_e465_d_b7, eq27_e465_d_b8, eq27_e465_d_b9, eq27_e465_d_b10, eq27_e465_d_b11, eq27_e465_d_b12, eq27_e465_d_b13, eq27_e465_d_b14];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e470, eq28_e470_d_n0, eq28_e470_d_n1, eq28_e470_d_n2, eq28_e470_d_n3, eq28_e470_d_n4, eq28_e470_d_n5, eq28_e470_d_n6, eq28_e470_d_n7, eq28_e470_d_n8, eq28_e470_d_n9, eq28_e470_d_n10, eq28_e470_d_n11, eq28_e470_d_n12, eq28_e470_d_n13, eq28_e470_d_n14, eq28_e470_d_n15, eq28_e470_d_n16, eq28_e470_d_n17, eq28_e470_d_n18, eq28_e470_d_b0, eq28_e470_d_b1, eq28_e470_d_b2, eq28_e470_d_b3, eq28_e470_d_b4, eq28_e470_d_b5, eq28_e470_d_b6, eq28_e470_d_b7, eq28_e470_d_b8, eq28_e470_d_b9, eq28_e470_d_b10, eq28_e470_d_b11, eq28_e470_d_b12, eq28_e470_d_b13, eq28_e470_d_b14,) = {
    if s.b[1848] {
        let eq28_e468: f64 = (-s.v[595]);
        let eq28_e468_d_n0: f64 = (-s.dn[595][0]);
        let eq28_e468_d_n1: f64 = (-s.dn[595][1]);
        let eq28_e468_d_n2: f64 = (-s.dn[595][2]);
        let eq28_e468_d_n3: f64 = (-s.dn[595][3]);
        let eq28_e468_d_n4: f64 = (-s.dn[595][4]);
        let eq28_e468_d_n5: f64 = (-s.dn[595][5]);
        let eq28_e468_d_n6: f64 = (-s.dn[595][6]);
        let eq28_e468_d_n7: f64 = (-s.dn[595][7]);
        let eq28_e468_d_n8: f64 = (-s.dn[595][8]);
        let eq28_e468_d_n9: f64 = (-s.dn[595][9]);
        let eq28_e468_d_n10: f64 = (-s.dn[595][10]);
        let eq28_e468_d_n11: f64 = (-s.dn[595][11]);
        let eq28_e468_d_n12: f64 = (-s.dn[595][12]);
        let eq28_e468_d_n13: f64 = (-s.dn[595][13]);
        let eq28_e468_d_n14: f64 = (-s.dn[595][14]);
        let eq28_e468_d_n15: f64 = (-s.dn[595][15]);
        let eq28_e468_d_n16: f64 = (-s.dn[595][16]);
        let eq28_e468_d_n17: f64 = (-s.dn[595][17]);
        let eq28_e468_d_n18: f64 = (-s.dn[595][18]);
        let eq28_e468_d_b0: f64 = (-s.db[595][0]);
        let eq28_e468_d_b1: f64 = (-s.db[595][1]);
        let eq28_e468_d_b2: f64 = (-s.db[595][2]);
        let eq28_e468_d_b3: f64 = (-s.db[595][3]);
        let eq28_e468_d_b4: f64 = (-s.db[595][4]);
        let eq28_e468_d_b5: f64 = (-s.db[595][5]);
        let eq28_e468_d_b6: f64 = (-s.db[595][6]);
        let eq28_e468_d_b7: f64 = (-s.db[595][7]);
        let eq28_e468_d_b8: f64 = (-s.db[595][8]);
        let eq28_e468_d_b9: f64 = (-s.db[595][9]);
        let eq28_e468_d_b10: f64 = (-s.db[595][10]);
        let eq28_e468_d_b11: f64 = (-s.db[595][11]);
        let eq28_e468_d_b12: f64 = (-s.db[595][12]);
        let eq28_e468_d_b13: f64 = (-s.db[595][13]);
        let eq28_e468_d_b14: f64 = (-s.db[595][14]);
        (eq28_e468, eq28_e468_d_n0, eq28_e468_d_n1, eq28_e468_d_n2, eq28_e468_d_n3, eq28_e468_d_n4, eq28_e468_d_n5, eq28_e468_d_n6, eq28_e468_d_n7, eq28_e468_d_n8, eq28_e468_d_n9, eq28_e468_d_n10, eq28_e468_d_n11, eq28_e468_d_n12, eq28_e468_d_n13, eq28_e468_d_n14, eq28_e468_d_n15, eq28_e468_d_n16, eq28_e468_d_n17, eq28_e468_d_n18, eq28_e468_d_b0, eq28_e468_d_b1, eq28_e468_d_b2, eq28_e468_d_b3, eq28_e468_d_b4, eq28_e468_d_b5, eq28_e468_d_b6, eq28_e468_d_b7, eq28_e468_d_b8, eq28_e468_d_b9, eq28_e468_d_b10, eq28_e468_d_b11, eq28_e468_d_b12, eq28_e468_d_b13, eq28_e468_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e470;
        let eq28_node_derivatives: [f64; 19] = [eq28_e470_d_n0, eq28_e470_d_n1, eq28_e470_d_n2, eq28_e470_d_n3, eq28_e470_d_n4, eq28_e470_d_n5, eq28_e470_d_n6, eq28_e470_d_n7, eq28_e470_d_n8, eq28_e470_d_n9, eq28_e470_d_n10, eq28_e470_d_n11, eq28_e470_d_n12, eq28_e470_d_n13, eq28_e470_d_n14, eq28_e470_d_n15, eq28_e470_d_n16, eq28_e470_d_n17, eq28_e470_d_n18];
        let eq28_branch_derivatives: [f64; 15] = [eq28_e470_d_b0, eq28_e470_d_b1, eq28_e470_d_b2, eq28_e470_d_b3, eq28_e470_d_b4, eq28_e470_d_b5, eq28_e470_d_b6, eq28_e470_d_b7, eq28_e470_d_b8, eq28_e470_d_b9, eq28_e470_d_b10, eq28_e470_d_b11, eq28_e470_d_b12, eq28_e470_d_b13, eq28_e470_d_b14];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e476, eq29_e476_d_n10,) = {
    if s.b[1848] {
        let eq29_e474: f64 = ((nv10 - 0.0) * 1e-12);
        let eq29_e474_d_n10: f64 = 1e-12;
        (eq29_e474, eq29_e474_d_n10,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e476;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq29_value),
            10,
            multiplicity * (eq29_e476_d_n10),
        );
        let (eq30_e483, eq30_e483_d_n0, eq30_e483_d_n1, eq30_e483_d_n2, eq30_e483_d_n3, eq30_e483_d_n4, eq30_e483_d_n5, eq30_e483_d_n6, eq30_e483_d_n7, eq30_e483_d_n8, eq30_e483_d_n9, eq30_e483_d_n10, eq30_e483_d_n11, eq30_e483_d_n12, eq30_e483_d_n13, eq30_e483_d_n14, eq30_e483_d_n15, eq30_e483_d_n16, eq30_e483_d_n17, eq30_e483_d_n18, eq30_e483_d_b0, eq30_e483_d_b1, eq30_e483_d_b2, eq30_e483_d_b3, eq30_e483_d_b4, eq30_e483_d_b5, eq30_e483_d_b6, eq30_e483_d_b7, eq30_e483_d_b8, eq30_e483_d_b9, eq30_e483_d_b10, eq30_e483_d_b11, eq30_e483_d_b12, eq30_e483_d_b13, eq30_e483_d_b14,) = {
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
        let eq30_e481: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq30_e480);
        let eq30_e481_d_n0: f64 = (eq30_e480_d_n0 * ddt_scale);
        let eq30_e481_d_n1: f64 = (eq30_e480_d_n1 * ddt_scale);
        let eq30_e481_d_n2: f64 = (eq30_e480_d_n2 * ddt_scale);
        let eq30_e481_d_n3: f64 = (eq30_e480_d_n3 * ddt_scale);
        let eq30_e481_d_n4: f64 = (eq30_e480_d_n4 * ddt_scale);
        let eq30_e481_d_n5: f64 = (eq30_e480_d_n5 * ddt_scale);
        let eq30_e481_d_n6: f64 = (eq30_e480_d_n6 * ddt_scale);
        let eq30_e481_d_n7: f64 = (eq30_e480_d_n7 * ddt_scale);
        let eq30_e481_d_n8: f64 = (eq30_e480_d_n8 * ddt_scale);
        let eq30_e481_d_n9: f64 = (eq30_e480_d_n9 * ddt_scale);
        let eq30_e481_d_n10: f64 = (eq30_e480_d_n10 * ddt_scale);
        let eq30_e481_d_n11: f64 = (eq30_e480_d_n11 * ddt_scale);
        let eq30_e481_d_n12: f64 = (eq30_e480_d_n12 * ddt_scale);
        let eq30_e481_d_n13: f64 = (eq30_e480_d_n13 * ddt_scale);
        let eq30_e481_d_n14: f64 = (eq30_e480_d_n14 * ddt_scale);
        let eq30_e481_d_n15: f64 = (eq30_e480_d_n15 * ddt_scale);
        let eq30_e481_d_n16: f64 = (eq30_e480_d_n16 * ddt_scale);
        let eq30_e481_d_n17: f64 = (eq30_e480_d_n17 * ddt_scale);
        let eq30_e481_d_n18: f64 = (eq30_e480_d_n18 * ddt_scale);
        let eq30_e481_d_b0: f64 = (eq30_e480_d_b0 * ddt_scale);
        let eq30_e481_d_b1: f64 = (eq30_e480_d_b1 * ddt_scale);
        let eq30_e481_d_b2: f64 = (eq30_e480_d_b2 * ddt_scale);
        let eq30_e481_d_b3: f64 = (eq30_e480_d_b3 * ddt_scale);
        let eq30_e481_d_b4: f64 = (eq30_e480_d_b4 * ddt_scale);
        let eq30_e481_d_b5: f64 = (eq30_e480_d_b5 * ddt_scale);
        let eq30_e481_d_b6: f64 = (eq30_e480_d_b6 * ddt_scale);
        let eq30_e481_d_b7: f64 = (eq30_e480_d_b7 * ddt_scale);
        let eq30_e481_d_b8: f64 = (eq30_e480_d_b8 * ddt_scale);
        let eq30_e481_d_b9: f64 = (eq30_e480_d_b9 * ddt_scale);
        let eq30_e481_d_b10: f64 = (eq30_e480_d_b10 * ddt_scale);
        let eq30_e481_d_b11: f64 = (eq30_e480_d_b11 * ddt_scale);
        let eq30_e481_d_b12: f64 = (eq30_e480_d_b12 * ddt_scale);
        let eq30_e481_d_b13: f64 = (eq30_e480_d_b13 * ddt_scale);
        let eq30_e481_d_b14: f64 = (eq30_e480_d_b14 * ddt_scale);
        (eq30_e481, eq30_e481_d_n0, eq30_e481_d_n1, eq30_e481_d_n2, eq30_e481_d_n3, eq30_e481_d_n4, eq30_e481_d_n5, eq30_e481_d_n6, eq30_e481_d_n7, eq30_e481_d_n8, eq30_e481_d_n9, eq30_e481_d_n10, eq30_e481_d_n11, eq30_e481_d_n12, eq30_e481_d_n13, eq30_e481_d_n14, eq30_e481_d_n15, eq30_e481_d_n16, eq30_e481_d_n17, eq30_e481_d_n18, eq30_e481_d_b0, eq30_e481_d_b1, eq30_e481_d_b2, eq30_e481_d_b3, eq30_e481_d_b4, eq30_e481_d_b5, eq30_e481_d_b6, eq30_e481_d_b7, eq30_e481_d_b8, eq30_e481_d_b9, eq30_e481_d_b10, eq30_e481_d_b11, eq30_e481_d_b12, eq30_e481_d_b13, eq30_e481_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e483;
        let eq30_node_derivatives: [f64; 19] = [eq30_e483_d_n0, eq30_e483_d_n1, eq30_e483_d_n2, eq30_e483_d_n3, eq30_e483_d_n4, eq30_e483_d_n5, eq30_e483_d_n6, eq30_e483_d_n7, eq30_e483_d_n8, eq30_e483_d_n9, eq30_e483_d_n10, eq30_e483_d_n11, eq30_e483_d_n12, eq30_e483_d_n13, eq30_e483_d_n14, eq30_e483_d_n15, eq30_e483_d_n16, eq30_e483_d_n17, eq30_e483_d_n18];
        let eq30_branch_derivatives: [f64; 15] = [eq30_e483_d_b0, eq30_e483_d_b1, eq30_e483_d_b2, eq30_e483_d_b3, eq30_e483_d_b4, eq30_e483_d_b5, eq30_e483_d_b6, eq30_e483_d_b7, eq30_e483_d_b8, eq30_e483_d_b9, eq30_e483_d_b10, eq30_e483_d_b11, eq30_e483_d_b12, eq30_e483_d_b13, eq30_e483_d_b14];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e490, eq31_e490_d_n10,) = {
    if (!s.b[1848]) {
        let eq31_e488: f64 = ((nv10 - 0.0) * 10000.0);
        let eq31_e488_d_n10: f64 = 10000.0;
        (eq31_e488, eq31_e488_d_n10,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e490;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq31_value),
            10,
            multiplicity * (eq31_e490_d_n10),
        );
        let (eq32_e498, eq32_e498_d_n0, eq32_e498_d_n1, eq32_e498_d_n2, eq32_e498_d_n3, eq32_e498_d_n4, eq32_e498_d_n5, eq32_e498_d_n6, eq32_e498_d_n7, eq32_e498_d_n8, eq32_e498_d_n9, eq32_e498_d_n10, eq32_e498_d_n11, eq32_e498_d_n12, eq32_e498_d_n13, eq32_e498_d_n14, eq32_e498_d_n15, eq32_e498_d_n16, eq32_e498_d_n17, eq32_e498_d_n18, eq32_e498_d_b0, eq32_e498_d_b1, eq32_e498_d_b2, eq32_e498_d_b3, eq32_e498_d_b4, eq32_e498_d_b5, eq32_e498_d_b6, eq32_e498_d_b7, eq32_e498_d_b8, eq32_e498_d_b9, eq32_e498_d_b10, eq32_e498_d_b11, eq32_e498_d_b12, eq32_e498_d_b13, eq32_e498_d_b14,) = {
    if s.b[1849] {
        let eq32_e495: f64 = (s.v[311] + s.v[263]);
        let eq32_e495_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);
        let eq32_e495_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);
        let eq32_e495_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);
        let eq32_e495_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);
        let eq32_e495_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);
        let eq32_e495_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);
        let eq32_e495_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);
        let eq32_e495_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);
        let eq32_e495_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);
        let eq32_e495_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);
        let eq32_e495_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);
        let eq32_e495_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);
        let eq32_e495_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);
        let eq32_e495_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);
        let eq32_e495_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);
        let eq32_e495_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);
        let eq32_e495_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);
        let eq32_e495_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);
        let eq32_e495_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);
        let eq32_e495_d_b0: f64 = (s.db[311][0] + s.db[263][0]);
        let eq32_e495_d_b1: f64 = (s.db[311][1] + s.db[263][1]);
        let eq32_e495_d_b2: f64 = (s.db[311][2] + s.db[263][2]);
        let eq32_e495_d_b3: f64 = (s.db[311][3] + s.db[263][3]);
        let eq32_e495_d_b4: f64 = (s.db[311][4] + s.db[263][4]);
        let eq32_e495_d_b5: f64 = (s.db[311][5] + s.db[263][5]);
        let eq32_e495_d_b6: f64 = (s.db[311][6] + s.db[263][6]);
        let eq32_e495_d_b7: f64 = (s.db[311][7] + s.db[263][7]);
        let eq32_e495_d_b8: f64 = (s.db[311][8] + s.db[263][8]);
        let eq32_e495_d_b9: f64 = (s.db[311][9] + s.db[263][9]);
        let eq32_e495_d_b10: f64 = (s.db[311][10] + s.db[263][10]);
        let eq32_e495_d_b11: f64 = (s.db[311][11] + s.db[263][11]);
        let eq32_e495_d_b12: f64 = (s.db[311][12] + s.db[263][12]);
        let eq32_e495_d_b13: f64 = (s.db[311][13] + s.db[263][13]);
        let eq32_e495_d_b14: f64 = (s.db[311][14] + s.db[263][14]);
        let eq32_e496: f64 = (p.p50 * eq32_e495);
        let eq32_e496_d_n0: f64 = (p.p50 * eq32_e495_d_n0);
        let eq32_e496_d_n1: f64 = (p.p50 * eq32_e495_d_n1);
        let eq32_e496_d_n2: f64 = (p.p50 * eq32_e495_d_n2);
        let eq32_e496_d_n3: f64 = (p.p50 * eq32_e495_d_n3);
        let eq32_e496_d_n4: f64 = (p.p50 * eq32_e495_d_n4);
        let eq32_e496_d_n5: f64 = (p.p50 * eq32_e495_d_n5);
        let eq32_e496_d_n6: f64 = (p.p50 * eq32_e495_d_n6);
        let eq32_e496_d_n7: f64 = (p.p50 * eq32_e495_d_n7);
        let eq32_e496_d_n8: f64 = (p.p50 * eq32_e495_d_n8);
        let eq32_e496_d_n9: f64 = (p.p50 * eq32_e495_d_n9);
        let eq32_e496_d_n10: f64 = (p.p50 * eq32_e495_d_n10);
        let eq32_e496_d_n11: f64 = (p.p50 * eq32_e495_d_n11);
        let eq32_e496_d_n12: f64 = (p.p50 * eq32_e495_d_n12);
        let eq32_e496_d_n13: f64 = (p.p50 * eq32_e495_d_n13);
        let eq32_e496_d_n14: f64 = (p.p50 * eq32_e495_d_n14);
        let eq32_e496_d_n15: f64 = (p.p50 * eq32_e495_d_n15);
        let eq32_e496_d_n16: f64 = (p.p50 * eq32_e495_d_n16);
        let eq32_e496_d_n17: f64 = (p.p50 * eq32_e495_d_n17);
        let eq32_e496_d_n18: f64 = (p.p50 * eq32_e495_d_n18);
        let eq32_e496_d_b0: f64 = (p.p50 * eq32_e495_d_b0);
        let eq32_e496_d_b1: f64 = (p.p50 * eq32_e495_d_b1);
        let eq32_e496_d_b2: f64 = (p.p50 * eq32_e495_d_b2);
        let eq32_e496_d_b3: f64 = (p.p50 * eq32_e495_d_b3);
        let eq32_e496_d_b4: f64 = (p.p50 * eq32_e495_d_b4);
        let eq32_e496_d_b5: f64 = (p.p50 * eq32_e495_d_b5);
        let eq32_e496_d_b6: f64 = (p.p50 * eq32_e495_d_b6);
        let eq32_e496_d_b7: f64 = (p.p50 * eq32_e495_d_b7);
        let eq32_e496_d_b8: f64 = (p.p50 * eq32_e495_d_b8);
        let eq32_e496_d_b9: f64 = (p.p50 * eq32_e495_d_b9);
        let eq32_e496_d_b10: f64 = (p.p50 * eq32_e495_d_b10);
        let eq32_e496_d_b11: f64 = (p.p50 * eq32_e495_d_b11);
        let eq32_e496_d_b12: f64 = (p.p50 * eq32_e495_d_b12);
        let eq32_e496_d_b13: f64 = (p.p50 * eq32_e495_d_b13);
        let eq32_e496_d_b14: f64 = (p.p50 * eq32_e495_d_b14);
        (eq32_e496, eq32_e496_d_n0, eq32_e496_d_n1, eq32_e496_d_n2, eq32_e496_d_n3, eq32_e496_d_n4, eq32_e496_d_n5, eq32_e496_d_n6, eq32_e496_d_n7, eq32_e496_d_n8, eq32_e496_d_n9, eq32_e496_d_n10, eq32_e496_d_n11, eq32_e496_d_n12, eq32_e496_d_n13, eq32_e496_d_n14, eq32_e496_d_n15, eq32_e496_d_n16, eq32_e496_d_n17, eq32_e496_d_n18, eq32_e496_d_b0, eq32_e496_d_b1, eq32_e496_d_b2, eq32_e496_d_b3, eq32_e496_d_b4, eq32_e496_d_b5, eq32_e496_d_b6, eq32_e496_d_b7, eq32_e496_d_b8, eq32_e496_d_b9, eq32_e496_d_b10, eq32_e496_d_b11, eq32_e496_d_b12, eq32_e496_d_b13, eq32_e496_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e498;
        let eq32_node_derivatives: [f64; 19] = [eq32_e498_d_n0, eq32_e498_d_n1, eq32_e498_d_n2, eq32_e498_d_n3, eq32_e498_d_n4, eq32_e498_d_n5, eq32_e498_d_n6, eq32_e498_d_n7, eq32_e498_d_n8, eq32_e498_d_n9, eq32_e498_d_n10, eq32_e498_d_n11, eq32_e498_d_n12, eq32_e498_d_n13, eq32_e498_d_n14, eq32_e498_d_n15, eq32_e498_d_n16, eq32_e498_d_n17, eq32_e498_d_n18];
        let eq32_branch_derivatives: [f64; 15] = [eq32_e498_d_b0, eq32_e498_d_b1, eq32_e498_d_b2, eq32_e498_d_b3, eq32_e498_d_b4, eq32_e498_d_b5, eq32_e498_d_b6, eq32_e498_d_b7, eq32_e498_d_b8, eq32_e498_d_b9, eq32_e498_d_b10, eq32_e498_d_b11, eq32_e498_d_b12, eq32_e498_d_b13, eq32_e498_d_b14];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(12),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let (eq33_e506, eq33_e506_d_n0, eq33_e506_d_n1, eq33_e506_d_n2, eq33_e506_d_n3, eq33_e506_d_n4, eq33_e506_d_n5, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n8, eq33_e506_d_n9, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n13, eq33_e506_d_n14, eq33_e506_d_n15, eq33_e506_d_n16, eq33_e506_d_n17, eq33_e506_d_n18, eq33_e506_d_b0, eq33_e506_d_b1, eq33_e506_d_b2, eq33_e506_d_b3, eq33_e506_d_b4, eq33_e506_d_b5, eq33_e506_d_b6, eq33_e506_d_b7, eq33_e506_d_b8, eq33_e506_d_b9, eq33_e506_d_b10, eq33_e506_d_b11, eq33_e506_d_b12, eq33_e506_d_b13, eq33_e506_d_b14,) = {
    if s.b[1849] {
        let eq33_e503: f64 = (s.v[312] + s.v[573]);
        let eq33_e503_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);
        let eq33_e503_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);
        let eq33_e503_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);
        let eq33_e503_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);
        let eq33_e503_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);
        let eq33_e503_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);
        let eq33_e503_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);
        let eq33_e503_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);
        let eq33_e503_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);
        let eq33_e503_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);
        let eq33_e503_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);
        let eq33_e503_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);
        let eq33_e503_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);
        let eq33_e503_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);
        let eq33_e503_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);
        let eq33_e503_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);
        let eq33_e503_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);
        let eq33_e503_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);
        let eq33_e503_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);
        let eq33_e503_d_b0: f64 = (s.db[312][0] + s.db[573][0]);
        let eq33_e503_d_b1: f64 = (s.db[312][1] + s.db[573][1]);
        let eq33_e503_d_b2: f64 = (s.db[312][2] + s.db[573][2]);
        let eq33_e503_d_b3: f64 = (s.db[312][3] + s.db[573][3]);
        let eq33_e503_d_b4: f64 = (s.db[312][4] + s.db[573][4]);
        let eq33_e503_d_b5: f64 = (s.db[312][5] + s.db[573][5]);
        let eq33_e503_d_b6: f64 = (s.db[312][6] + s.db[573][6]);
        let eq33_e503_d_b7: f64 = (s.db[312][7] + s.db[573][7]);
        let eq33_e503_d_b8: f64 = (s.db[312][8] + s.db[573][8]);
        let eq33_e503_d_b9: f64 = (s.db[312][9] + s.db[573][9]);
        let eq33_e503_d_b10: f64 = (s.db[312][10] + s.db[573][10]);
        let eq33_e503_d_b11: f64 = (s.db[312][11] + s.db[573][11]);
        let eq33_e503_d_b12: f64 = (s.db[312][12] + s.db[573][12]);
        let eq33_e503_d_b13: f64 = (s.db[312][13] + s.db[573][13]);
        let eq33_e503_d_b14: f64 = (s.db[312][14] + s.db[573][14]);
        let eq33_e504: f64 = (p.p50 * eq33_e503);
        let eq33_e504_d_n0: f64 = (p.p50 * eq33_e503_d_n0);
        let eq33_e504_d_n1: f64 = (p.p50 * eq33_e503_d_n1);
        let eq33_e504_d_n2: f64 = (p.p50 * eq33_e503_d_n2);
        let eq33_e504_d_n3: f64 = (p.p50 * eq33_e503_d_n3);
        let eq33_e504_d_n4: f64 = (p.p50 * eq33_e503_d_n4);
        let eq33_e504_d_n5: f64 = (p.p50 * eq33_e503_d_n5);
        let eq33_e504_d_n6: f64 = (p.p50 * eq33_e503_d_n6);
        let eq33_e504_d_n7: f64 = (p.p50 * eq33_e503_d_n7);
        let eq33_e504_d_n8: f64 = (p.p50 * eq33_e503_d_n8);
        let eq33_e504_d_n9: f64 = (p.p50 * eq33_e503_d_n9);
        let eq33_e504_d_n10: f64 = (p.p50 * eq33_e503_d_n10);
        let eq33_e504_d_n11: f64 = (p.p50 * eq33_e503_d_n11);
        let eq33_e504_d_n12: f64 = (p.p50 * eq33_e503_d_n12);
        let eq33_e504_d_n13: f64 = (p.p50 * eq33_e503_d_n13);
        let eq33_e504_d_n14: f64 = (p.p50 * eq33_e503_d_n14);
        let eq33_e504_d_n15: f64 = (p.p50 * eq33_e503_d_n15);
        let eq33_e504_d_n16: f64 = (p.p50 * eq33_e503_d_n16);
        let eq33_e504_d_n17: f64 = (p.p50 * eq33_e503_d_n17);
        let eq33_e504_d_n18: f64 = (p.p50 * eq33_e503_d_n18);
        let eq33_e504_d_b0: f64 = (p.p50 * eq33_e503_d_b0);
        let eq33_e504_d_b1: f64 = (p.p50 * eq33_e503_d_b1);
        let eq33_e504_d_b2: f64 = (p.p50 * eq33_e503_d_b2);
        let eq33_e504_d_b3: f64 = (p.p50 * eq33_e503_d_b3);
        let eq33_e504_d_b4: f64 = (p.p50 * eq33_e503_d_b4);
        let eq33_e504_d_b5: f64 = (p.p50 * eq33_e503_d_b5);
        let eq33_e504_d_b6: f64 = (p.p50 * eq33_e503_d_b6);
        let eq33_e504_d_b7: f64 = (p.p50 * eq33_e503_d_b7);
        let eq33_e504_d_b8: f64 = (p.p50 * eq33_e503_d_b8);
        let eq33_e504_d_b9: f64 = (p.p50 * eq33_e503_d_b9);
        let eq33_e504_d_b10: f64 = (p.p50 * eq33_e503_d_b10);
        let eq33_e504_d_b11: f64 = (p.p50 * eq33_e503_d_b11);
        let eq33_e504_d_b12: f64 = (p.p50 * eq33_e503_d_b12);
        let eq33_e504_d_b13: f64 = (p.p50 * eq33_e503_d_b13);
        let eq33_e504_d_b14: f64 = (p.p50 * eq33_e503_d_b14);
        (eq33_e504, eq33_e504_d_n0, eq33_e504_d_n1, eq33_e504_d_n2, eq33_e504_d_n3, eq33_e504_d_n4, eq33_e504_d_n5, eq33_e504_d_n6, eq33_e504_d_n7, eq33_e504_d_n8, eq33_e504_d_n9, eq33_e504_d_n10, eq33_e504_d_n11, eq33_e504_d_n12, eq33_e504_d_n13, eq33_e504_d_n14, eq33_e504_d_n15, eq33_e504_d_n16, eq33_e504_d_n17, eq33_e504_d_n18, eq33_e504_d_b0, eq33_e504_d_b1, eq33_e504_d_b2, eq33_e504_d_b3, eq33_e504_d_b4, eq33_e504_d_b5, eq33_e504_d_b6, eq33_e504_d_b7, eq33_e504_d_b8, eq33_e504_d_b9, eq33_e504_d_b10, eq33_e504_d_b11, eq33_e504_d_b12, eq33_e504_d_b13, eq33_e504_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e506;
        let eq33_node_derivatives: [f64; 19] = [eq33_e506_d_n0, eq33_e506_d_n1, eq33_e506_d_n2, eq33_e506_d_n3, eq33_e506_d_n4, eq33_e506_d_n5, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n8, eq33_e506_d_n9, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n13, eq33_e506_d_n14, eq33_e506_d_n15, eq33_e506_d_n16, eq33_e506_d_n17, eq33_e506_d_n18];
        let eq33_branch_derivatives: [f64; 15] = [eq33_e506_d_b0, eq33_e506_d_b1, eq33_e506_d_b2, eq33_e506_d_b3, eq33_e506_d_b4, eq33_e506_d_b5, eq33_e506_d_b6, eq33_e506_d_b7, eq33_e506_d_b8, eq33_e506_d_b9, eq33_e506_d_b10, eq33_e506_d_b11, eq33_e506_d_b12, eq33_e506_d_b13, eq33_e506_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(12),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n1, eq34_e515_d_n2, eq34_e515_d_n3, eq34_e515_d_n4, eq34_e515_d_n5, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n8, eq34_e515_d_n9, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n13, eq34_e515_d_n14, eq34_e515_d_n15, eq34_e515_d_n16, eq34_e515_d_n17, eq34_e515_d_n18, eq34_e515_d_b0, eq34_e515_d_b1, eq34_e515_d_b2, eq34_e515_d_b3, eq34_e515_d_b4, eq34_e515_d_b5, eq34_e515_d_b6, eq34_e515_d_b7, eq34_e515_d_b8, eq34_e515_d_b9, eq34_e515_d_b10, eq34_e515_d_b11, eq34_e515_d_b12, eq34_e515_d_b13, eq34_e515_d_b14,) = {
    if s.b[1849] {
        let eq34_e511: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[283]);
        let eq34_e511_d_n0: f64 = (s.dn[283][0] * ddt_scale);
        let eq34_e511_d_n1: f64 = (s.dn[283][1] * ddt_scale);
        let eq34_e511_d_n2: f64 = (s.dn[283][2] * ddt_scale);
        let eq34_e511_d_n3: f64 = (s.dn[283][3] * ddt_scale);
        let eq34_e511_d_n4: f64 = (s.dn[283][4] * ddt_scale);
        let eq34_e511_d_n5: f64 = (s.dn[283][5] * ddt_scale);
        let eq34_e511_d_n6: f64 = (s.dn[283][6] * ddt_scale);
        let eq34_e511_d_n7: f64 = (s.dn[283][7] * ddt_scale);
        let eq34_e511_d_n8: f64 = (s.dn[283][8] * ddt_scale);
        let eq34_e511_d_n9: f64 = (s.dn[283][9] * ddt_scale);
        let eq34_e511_d_n10: f64 = (s.dn[283][10] * ddt_scale);
        let eq34_e511_d_n11: f64 = (s.dn[283][11] * ddt_scale);
        let eq34_e511_d_n12: f64 = (s.dn[283][12] * ddt_scale);
        let eq34_e511_d_n13: f64 = (s.dn[283][13] * ddt_scale);
        let eq34_e511_d_n14: f64 = (s.dn[283][14] * ddt_scale);
        let eq34_e511_d_n15: f64 = (s.dn[283][15] * ddt_scale);
        let eq34_e511_d_n16: f64 = (s.dn[283][16] * ddt_scale);
        let eq34_e511_d_n17: f64 = (s.dn[283][17] * ddt_scale);
        let eq34_e511_d_n18: f64 = (s.dn[283][18] * ddt_scale);
        let eq34_e511_d_b0: f64 = (s.db[283][0] * ddt_scale);
        let eq34_e511_d_b1: f64 = (s.db[283][1] * ddt_scale);
        let eq34_e511_d_b2: f64 = (s.db[283][2] * ddt_scale);
        let eq34_e511_d_b3: f64 = (s.db[283][3] * ddt_scale);
        let eq34_e511_d_b4: f64 = (s.db[283][4] * ddt_scale);
        let eq34_e511_d_b5: f64 = (s.db[283][5] * ddt_scale);
        let eq34_e511_d_b6: f64 = (s.db[283][6] * ddt_scale);
        let eq34_e511_d_b7: f64 = (s.db[283][7] * ddt_scale);
        let eq34_e511_d_b8: f64 = (s.db[283][8] * ddt_scale);
        let eq34_e511_d_b9: f64 = (s.db[283][9] * ddt_scale);
        let eq34_e511_d_b10: f64 = (s.db[283][10] * ddt_scale);
        let eq34_e511_d_b11: f64 = (s.db[283][11] * ddt_scale);
        let eq34_e511_d_b12: f64 = (s.db[283][12] * ddt_scale);
        let eq34_e511_d_b13: f64 = (s.db[283][13] * ddt_scale);
        let eq34_e511_d_b14: f64 = (s.db[283][14] * ddt_scale);
        let eq34_e512: f64 = (s.v[281] + eq34_e511);
        let eq34_e512_d_n0: f64 = (s.dn[281][0] + eq34_e511_d_n0);
        let eq34_e512_d_n1: f64 = (s.dn[281][1] + eq34_e511_d_n1);
        let eq34_e512_d_n2: f64 = (s.dn[281][2] + eq34_e511_d_n2);
        let eq34_e512_d_n3: f64 = (s.dn[281][3] + eq34_e511_d_n3);
        let eq34_e512_d_n4: f64 = (s.dn[281][4] + eq34_e511_d_n4);
        let eq34_e512_d_n5: f64 = (s.dn[281][5] + eq34_e511_d_n5);
        let eq34_e512_d_n6: f64 = (s.dn[281][6] + eq34_e511_d_n6);
        let eq34_e512_d_n7: f64 = (s.dn[281][7] + eq34_e511_d_n7);
        let eq34_e512_d_n8: f64 = (s.dn[281][8] + eq34_e511_d_n8);
        let eq34_e512_d_n9: f64 = (s.dn[281][9] + eq34_e511_d_n9);
        let eq34_e512_d_n10: f64 = (s.dn[281][10] + eq34_e511_d_n10);
        let eq34_e512_d_n11: f64 = (s.dn[281][11] + eq34_e511_d_n11);
        let eq34_e512_d_n12: f64 = (s.dn[281][12] + eq34_e511_d_n12);
        let eq34_e512_d_n13: f64 = (s.dn[281][13] + eq34_e511_d_n13);
        let eq34_e512_d_n14: f64 = (s.dn[281][14] + eq34_e511_d_n14);
        let eq34_e512_d_n15: f64 = (s.dn[281][15] + eq34_e511_d_n15);
        let eq34_e512_d_n16: f64 = (s.dn[281][16] + eq34_e511_d_n16);
        let eq34_e512_d_n17: f64 = (s.dn[281][17] + eq34_e511_d_n17);
        let eq34_e512_d_n18: f64 = (s.dn[281][18] + eq34_e511_d_n18);
        let eq34_e512_d_b0: f64 = (s.db[281][0] + eq34_e511_d_b0);
        let eq34_e512_d_b1: f64 = (s.db[281][1] + eq34_e511_d_b1);
        let eq34_e512_d_b2: f64 = (s.db[281][2] + eq34_e511_d_b2);
        let eq34_e512_d_b3: f64 = (s.db[281][3] + eq34_e511_d_b3);
        let eq34_e512_d_b4: f64 = (s.db[281][4] + eq34_e511_d_b4);
        let eq34_e512_d_b5: f64 = (s.db[281][5] + eq34_e511_d_b5);
        let eq34_e512_d_b6: f64 = (s.db[281][6] + eq34_e511_d_b6);
        let eq34_e512_d_b7: f64 = (s.db[281][7] + eq34_e511_d_b7);
        let eq34_e512_d_b8: f64 = (s.db[281][8] + eq34_e511_d_b8);
        let eq34_e512_d_b9: f64 = (s.db[281][9] + eq34_e511_d_b9);
        let eq34_e512_d_b10: f64 = (s.db[281][10] + eq34_e511_d_b10);
        let eq34_e512_d_b11: f64 = (s.db[281][11] + eq34_e511_d_b11);
        let eq34_e512_d_b12: f64 = (s.db[281][12] + eq34_e511_d_b12);
        let eq34_e512_d_b13: f64 = (s.db[281][13] + eq34_e511_d_b13);
        let eq34_e512_d_b14: f64 = (s.db[281][14] + eq34_e511_d_b14);
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
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n1, eq34_e513_d_n2, eq34_e513_d_n3, eq34_e513_d_n4, eq34_e513_d_n5, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n8, eq34_e513_d_n9, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n13, eq34_e513_d_n14, eq34_e513_d_n15, eq34_e513_d_n16, eq34_e513_d_n17, eq34_e513_d_n18, eq34_e513_d_b0, eq34_e513_d_b1, eq34_e513_d_b2, eq34_e513_d_b3, eq34_e513_d_b4, eq34_e513_d_b5, eq34_e513_d_b6, eq34_e513_d_b7, eq34_e513_d_b8, eq34_e513_d_b9, eq34_e513_d_b10, eq34_e513_d_b11, eq34_e513_d_b12, eq34_e513_d_b13, eq34_e513_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e515;
        let eq34_node_derivatives: [f64; 19] = [eq34_e515_d_n0, eq34_e515_d_n1, eq34_e515_d_n2, eq34_e515_d_n3, eq34_e515_d_n4, eq34_e515_d_n5, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n8, eq34_e515_d_n9, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n13, eq34_e515_d_n14, eq34_e515_d_n15, eq34_e515_d_n16, eq34_e515_d_n17, eq34_e515_d_n18];
        let eq34_branch_derivatives: [f64; 15] = [eq34_e515_d_b0, eq34_e515_d_b1, eq34_e515_d_b2, eq34_e515_d_b3, eq34_e515_d_b4, eq34_e515_d_b5, eq34_e515_d_b6, eq34_e515_d_b7, eq34_e515_d_b8, eq34_e515_d_b9, eq34_e515_d_b10, eq34_e515_d_b11, eq34_e515_d_b12, eq34_e515_d_b13, eq34_e515_d_b14];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n1, eq35_e524_d_n2, eq35_e524_d_n3, eq35_e524_d_n4, eq35_e524_d_n5, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n8, eq35_e524_d_n9, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n13, eq35_e524_d_n14, eq35_e524_d_n15, eq35_e524_d_n16, eq35_e524_d_n17, eq35_e524_d_n18, eq35_e524_d_b0, eq35_e524_d_b1, eq35_e524_d_b2, eq35_e524_d_b3, eq35_e524_d_b4, eq35_e524_d_b5, eq35_e524_d_b6, eq35_e524_d_b7, eq35_e524_d_b8, eq35_e524_d_b9, eq35_e524_d_b10, eq35_e524_d_b11, eq35_e524_d_b12, eq35_e524_d_b13, eq35_e524_d_b14,) = {
    if s.b[1849] {
        let eq35_e520: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, s.v[284]);
        let eq35_e520_d_n0: f64 = (s.dn[284][0] * ddt_scale);
        let eq35_e520_d_n1: f64 = (s.dn[284][1] * ddt_scale);
        let eq35_e520_d_n2: f64 = (s.dn[284][2] * ddt_scale);
        let eq35_e520_d_n3: f64 = (s.dn[284][3] * ddt_scale);
        let eq35_e520_d_n4: f64 = (s.dn[284][4] * ddt_scale);
        let eq35_e520_d_n5: f64 = (s.dn[284][5] * ddt_scale);
        let eq35_e520_d_n6: f64 = (s.dn[284][6] * ddt_scale);
        let eq35_e520_d_n7: f64 = (s.dn[284][7] * ddt_scale);
        let eq35_e520_d_n8: f64 = (s.dn[284][8] * ddt_scale);
        let eq35_e520_d_n9: f64 = (s.dn[284][9] * ddt_scale);
        let eq35_e520_d_n10: f64 = (s.dn[284][10] * ddt_scale);
        let eq35_e520_d_n11: f64 = (s.dn[284][11] * ddt_scale);
        let eq35_e520_d_n12: f64 = (s.dn[284][12] * ddt_scale);
        let eq35_e520_d_n13: f64 = (s.dn[284][13] * ddt_scale);
        let eq35_e520_d_n14: f64 = (s.dn[284][14] * ddt_scale);
        let eq35_e520_d_n15: f64 = (s.dn[284][15] * ddt_scale);
        let eq35_e520_d_n16: f64 = (s.dn[284][16] * ddt_scale);
        let eq35_e520_d_n17: f64 = (s.dn[284][17] * ddt_scale);
        let eq35_e520_d_n18: f64 = (s.dn[284][18] * ddt_scale);
        let eq35_e520_d_b0: f64 = (s.db[284][0] * ddt_scale);
        let eq35_e520_d_b1: f64 = (s.db[284][1] * ddt_scale);
        let eq35_e520_d_b2: f64 = (s.db[284][2] * ddt_scale);
        let eq35_e520_d_b3: f64 = (s.db[284][3] * ddt_scale);
        let eq35_e520_d_b4: f64 = (s.db[284][4] * ddt_scale);
        let eq35_e520_d_b5: f64 = (s.db[284][5] * ddt_scale);
        let eq35_e520_d_b6: f64 = (s.db[284][6] * ddt_scale);
        let eq35_e520_d_b7: f64 = (s.db[284][7] * ddt_scale);
        let eq35_e520_d_b8: f64 = (s.db[284][8] * ddt_scale);
        let eq35_e520_d_b9: f64 = (s.db[284][9] * ddt_scale);
        let eq35_e520_d_b10: f64 = (s.db[284][10] * ddt_scale);
        let eq35_e520_d_b11: f64 = (s.db[284][11] * ddt_scale);
        let eq35_e520_d_b12: f64 = (s.db[284][12] * ddt_scale);
        let eq35_e520_d_b13: f64 = (s.db[284][13] * ddt_scale);
        let eq35_e520_d_b14: f64 = (s.db[284][14] * ddt_scale);
        let eq35_e521: f64 = (s.v[282] + eq35_e520);
        let eq35_e521_d_n0: f64 = (s.dn[282][0] + eq35_e520_d_n0);
        let eq35_e521_d_n1: f64 = (s.dn[282][1] + eq35_e520_d_n1);
        let eq35_e521_d_n2: f64 = (s.dn[282][2] + eq35_e520_d_n2);
        let eq35_e521_d_n3: f64 = (s.dn[282][3] + eq35_e520_d_n3);
        let eq35_e521_d_n4: f64 = (s.dn[282][4] + eq35_e520_d_n4);
        let eq35_e521_d_n5: f64 = (s.dn[282][5] + eq35_e520_d_n5);
        let eq35_e521_d_n6: f64 = (s.dn[282][6] + eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (s.dn[282][7] + eq35_e520_d_n7);
        let eq35_e521_d_n8: f64 = (s.dn[282][8] + eq35_e520_d_n8);
        let eq35_e521_d_n9: f64 = (s.dn[282][9] + eq35_e520_d_n9);
        let eq35_e521_d_n10: f64 = (s.dn[282][10] + eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (s.dn[282][11] + eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (s.dn[282][12] + eq35_e520_d_n12);
        let eq35_e521_d_n13: f64 = (s.dn[282][13] + eq35_e520_d_n13);
        let eq35_e521_d_n14: f64 = (s.dn[282][14] + eq35_e520_d_n14);
        let eq35_e521_d_n15: f64 = (s.dn[282][15] + eq35_e520_d_n15);
        let eq35_e521_d_n16: f64 = (s.dn[282][16] + eq35_e520_d_n16);
        let eq35_e521_d_n17: f64 = (s.dn[282][17] + eq35_e520_d_n17);
        let eq35_e521_d_n18: f64 = (s.dn[282][18] + eq35_e520_d_n18);
        let eq35_e521_d_b0: f64 = (s.db[282][0] + eq35_e520_d_b0);
        let eq35_e521_d_b1: f64 = (s.db[282][1] + eq35_e520_d_b1);
        let eq35_e521_d_b2: f64 = (s.db[282][2] + eq35_e520_d_b2);
        let eq35_e521_d_b3: f64 = (s.db[282][3] + eq35_e520_d_b3);
        let eq35_e521_d_b4: f64 = (s.db[282][4] + eq35_e520_d_b4);
        let eq35_e521_d_b5: f64 = (s.db[282][5] + eq35_e520_d_b5);
        let eq35_e521_d_b6: f64 = (s.db[282][6] + eq35_e520_d_b6);
        let eq35_e521_d_b7: f64 = (s.db[282][7] + eq35_e520_d_b7);
        let eq35_e521_d_b8: f64 = (s.db[282][8] + eq35_e520_d_b8);
        let eq35_e521_d_b9: f64 = (s.db[282][9] + eq35_e520_d_b9);
        let eq35_e521_d_b10: f64 = (s.db[282][10] + eq35_e520_d_b10);
        let eq35_e521_d_b11: f64 = (s.db[282][11] + eq35_e520_d_b11);
        let eq35_e521_d_b12: f64 = (s.db[282][12] + eq35_e520_d_b12);
        let eq35_e521_d_b13: f64 = (s.db[282][13] + eq35_e520_d_b13);
        let eq35_e521_d_b14: f64 = (s.db[282][14] + eq35_e520_d_b14);
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
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n1, eq35_e522_d_n2, eq35_e522_d_n3, eq35_e522_d_n4, eq35_e522_d_n5, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n8, eq35_e522_d_n9, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n13, eq35_e522_d_n14, eq35_e522_d_n15, eq35_e522_d_n16, eq35_e522_d_n17, eq35_e522_d_n18, eq35_e522_d_b0, eq35_e522_d_b1, eq35_e522_d_b2, eq35_e522_d_b3, eq35_e522_d_b4, eq35_e522_d_b5, eq35_e522_d_b6, eq35_e522_d_b7, eq35_e522_d_b8, eq35_e522_d_b9, eq35_e522_d_b10, eq35_e522_d_b11, eq35_e522_d_b12, eq35_e522_d_b13, eq35_e522_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e524;
        let eq35_node_derivatives: [f64; 19] = [eq35_e524_d_n0, eq35_e524_d_n1, eq35_e524_d_n2, eq35_e524_d_n3, eq35_e524_d_n4, eq35_e524_d_n5, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n8, eq35_e524_d_n9, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n13, eq35_e524_d_n14, eq35_e524_d_n15, eq35_e524_d_n16, eq35_e524_d_n17, eq35_e524_d_n18];
        let eq35_branch_derivatives: [f64; 15] = [eq35_e524_d_b0, eq35_e524_d_b1, eq35_e524_d_b2, eq35_e524_d_b3, eq35_e524_d_b4, eq35_e524_d_b5, eq35_e524_d_b6, eq35_e524_d_b7, eq35_e524_d_b8, eq35_e524_d_b9, eq35_e524_d_b10, eq35_e524_d_b11, eq35_e524_d_b12, eq35_e524_d_b13, eq35_e524_d_b14];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n1, eq36_e532_d_n2, eq36_e532_d_n3, eq36_e532_d_n4, eq36_e532_d_n5, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n8, eq36_e532_d_n9, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n13, eq36_e532_d_n14, eq36_e532_d_n15, eq36_e532_d_n16, eq36_e532_d_n17, eq36_e532_d_n18, eq36_e532_d_b0, eq36_e532_d_b1, eq36_e532_d_b2, eq36_e532_d_b3, eq36_e532_d_b4, eq36_e532_d_b5, eq36_e532_d_b6, eq36_e532_d_b7, eq36_e532_d_b8, eq36_e532_d_b9, eq36_e532_d_b10, eq36_e532_d_b11, eq36_e532_d_b12, eq36_e532_d_b13, eq36_e532_d_b14,) = {
    if (s.b[1849] && (p.p261 != 0.0)) {
        let eq36_e530: f64 = ((nv4 - nv12) / s.v[2]);
        let eq36_e530_d_n0: f64 = (-(((nv4 - nv12) * s.dn[2][0]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n1: f64 = (-(((nv4 - nv12) * s.dn[2][1]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n2: f64 = (-(((nv4 - nv12) * s.dn[2][2]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n3: f64 = (-(((nv4 - nv12) * s.dn[2][3]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n4: f64 = ((s.v[2] - ((nv4 - nv12) * s.dn[2][4])) / (s.v[2] * s.v[2]));
        let eq36_e530_d_n5: f64 = (-(((nv4 - nv12) * s.dn[2][5]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n6: f64 = (-(((nv4 - nv12) * s.dn[2][6]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n7: f64 = (-(((nv4 - nv12) * s.dn[2][7]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n8: f64 = (-(((nv4 - nv12) * s.dn[2][8]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n9: f64 = (-(((nv4 - nv12) * s.dn[2][9]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n10: f64 = (-(((nv4 - nv12) * s.dn[2][10]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n11: f64 = (-(((nv4 - nv12) * s.dn[2][11]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n12: f64 = (((-s.v[2]) - ((nv4 - nv12) * s.dn[2][12])) / (s.v[2] * s.v[2]));
        let eq36_e530_d_n13: f64 = (-(((nv4 - nv12) * s.dn[2][13]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n14: f64 = (-(((nv4 - nv12) * s.dn[2][14]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n15: f64 = (-(((nv4 - nv12) * s.dn[2][15]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n16: f64 = (-(((nv4 - nv12) * s.dn[2][16]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n17: f64 = (-(((nv4 - nv12) * s.dn[2][17]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_n18: f64 = (-(((nv4 - nv12) * s.dn[2][18]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b0: f64 = (-(((nv4 - nv12) * s.db[2][0]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b1: f64 = (-(((nv4 - nv12) * s.db[2][1]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b2: f64 = (-(((nv4 - nv12) * s.db[2][2]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b3: f64 = (-(((nv4 - nv12) * s.db[2][3]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b4: f64 = (-(((nv4 - nv12) * s.db[2][4]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b5: f64 = (-(((nv4 - nv12) * s.db[2][5]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b6: f64 = (-(((nv4 - nv12) * s.db[2][6]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b7: f64 = (-(((nv4 - nv12) * s.db[2][7]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b8: f64 = (-(((nv4 - nv12) * s.db[2][8]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b9: f64 = (-(((nv4 - nv12) * s.db[2][9]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b10: f64 = (-(((nv4 - nv12) * s.db[2][10]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b11: f64 = (-(((nv4 - nv12) * s.db[2][11]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b12: f64 = (-(((nv4 - nv12) * s.db[2][12]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b13: f64 = (-(((nv4 - nv12) * s.db[2][13]) / (s.v[2] * s.v[2])));
        let eq36_e530_d_b14: f64 = (-(((nv4 - nv12) * s.db[2][14]) / (s.v[2] * s.v[2])));
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n1, eq36_e530_d_n2, eq36_e530_d_n3, eq36_e530_d_n4, eq36_e530_d_n5, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n8, eq36_e530_d_n9, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n13, eq36_e530_d_n14, eq36_e530_d_n15, eq36_e530_d_n16, eq36_e530_d_n17, eq36_e530_d_n18, eq36_e530_d_b0, eq36_e530_d_b1, eq36_e530_d_b2, eq36_e530_d_b3, eq36_e530_d_b4, eq36_e530_d_b5, eq36_e530_d_b6, eq36_e530_d_b7, eq36_e530_d_b8, eq36_e530_d_b9, eq36_e530_d_b10, eq36_e530_d_b11, eq36_e530_d_b12, eq36_e530_d_b13, eq36_e530_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e532;
        let eq36_node_derivatives: [f64; 19] = [eq36_e532_d_n0, eq36_e532_d_n1, eq36_e532_d_n2, eq36_e532_d_n3, eq36_e532_d_n4, eq36_e532_d_n5, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n8, eq36_e532_d_n9, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n13, eq36_e532_d_n14, eq36_e532_d_n15, eq36_e532_d_n16, eq36_e532_d_n17, eq36_e532_d_n18];
        let eq36_branch_derivatives: [f64; 15] = [eq36_e532_d_b0, eq36_e532_d_b1, eq36_e532_d_b2, eq36_e532_d_b3, eq36_e532_d_b4, eq36_e532_d_b5, eq36_e532_d_b6, eq36_e532_d_b7, eq36_e532_d_b8, eq36_e532_d_b9, eq36_e532_d_b10, eq36_e532_d_b11, eq36_e532_d_b12, eq36_e532_d_b13, eq36_e532_d_b14];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(12),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq37_e539,) = {
    if (s.b[1849] && (p.p261 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e539;
        stamper.stamp_potential_const_local(
            5,
            eq37_value,
        );
        let (eq38_e547, eq38_e547_d_n0, eq38_e547_d_n1, eq38_e547_d_n2, eq38_e547_d_n3, eq38_e547_d_n4, eq38_e547_d_n5, eq38_e547_d_n6, eq38_e547_d_n7, eq38_e547_d_n8, eq38_e547_d_n9, eq38_e547_d_n10, eq38_e547_d_n11, eq38_e547_d_n12, eq38_e547_d_n13, eq38_e547_d_n14, eq38_e547_d_n15, eq38_e547_d_n16, eq38_e547_d_n17, eq38_e547_d_n18, eq38_e547_d_b0, eq38_e547_d_b1, eq38_e547_d_b2, eq38_e547_d_b3, eq38_e547_d_b4, eq38_e547_d_b5, eq38_e547_d_b6, eq38_e547_d_b7, eq38_e547_d_b8, eq38_e547_d_b9, eq38_e547_d_b10, eq38_e547_d_b11, eq38_e547_d_b12, eq38_e547_d_b13, eq38_e547_d_b14,) = {
    if (s.b[1849] && (p.p262 != 0.0)) {
        let eq38_e545: f64 = (s.v[553] * (nv9 - nv12));
        let eq38_e545_d_n0: f64 = (s.dn[553][0] * (nv9 - nv12));
        let eq38_e545_d_n1: f64 = (s.dn[553][1] * (nv9 - nv12));
        let eq38_e545_d_n2: f64 = (s.dn[553][2] * (nv9 - nv12));
        let eq38_e545_d_n3: f64 = (s.dn[553][3] * (nv9 - nv12));
        let eq38_e545_d_n4: f64 = (s.dn[553][4] * (nv9 - nv12));
        let eq38_e545_d_n5: f64 = (s.dn[553][5] * (nv9 - nv12));
        let eq38_e545_d_n6: f64 = (s.dn[553][6] * (nv9 - nv12));
        let eq38_e545_d_n7: f64 = (s.dn[553][7] * (nv9 - nv12));
        let eq38_e545_d_n8: f64 = (s.dn[553][8] * (nv9 - nv12));
        let eq38_e545_d_n9: f64 = ((s.dn[553][9] * (nv9 - nv12)) + s.v[553]);
        let eq38_e545_d_n10: f64 = (s.dn[553][10] * (nv9 - nv12));
        let eq38_e545_d_n11: f64 = (s.dn[553][11] * (nv9 - nv12));
        let eq38_e545_d_n12: f64 = ((s.dn[553][12] * (nv9 - nv12)) + (-s.v[553]));
        let eq38_e545_d_n13: f64 = (s.dn[553][13] * (nv9 - nv12));
        let eq38_e545_d_n14: f64 = (s.dn[553][14] * (nv9 - nv12));
        let eq38_e545_d_n15: f64 = (s.dn[553][15] * (nv9 - nv12));
        let eq38_e545_d_n16: f64 = (s.dn[553][16] * (nv9 - nv12));
        let eq38_e545_d_n17: f64 = (s.dn[553][17] * (nv9 - nv12));
        let eq38_e545_d_n18: f64 = (s.dn[553][18] * (nv9 - nv12));
        let eq38_e545_d_b0: f64 = (s.db[553][0] * (nv9 - nv12));
        let eq38_e545_d_b1: f64 = (s.db[553][1] * (nv9 - nv12));
        let eq38_e545_d_b2: f64 = (s.db[553][2] * (nv9 - nv12));
        let eq38_e545_d_b3: f64 = (s.db[553][3] * (nv9 - nv12));
        let eq38_e545_d_b4: f64 = (s.db[553][4] * (nv9 - nv12));
        let eq38_e545_d_b5: f64 = (s.db[553][5] * (nv9 - nv12));
        let eq38_e545_d_b6: f64 = (s.db[553][6] * (nv9 - nv12));
        let eq38_e545_d_b7: f64 = (s.db[553][7] * (nv9 - nv12));
        let eq38_e545_d_b8: f64 = (s.db[553][8] * (nv9 - nv12));
        let eq38_e545_d_b9: f64 = (s.db[553][9] * (nv9 - nv12));
        let eq38_e545_d_b10: f64 = (s.db[553][10] * (nv9 - nv12));
        let eq38_e545_d_b11: f64 = (s.db[553][11] * (nv9 - nv12));
        let eq38_e545_d_b12: f64 = (s.db[553][12] * (nv9 - nv12));
        let eq38_e545_d_b13: f64 = (s.db[553][13] * (nv9 - nv12));
        let eq38_e545_d_b14: f64 = (s.db[553][14] * (nv9 - nv12));
        (eq38_e545, eq38_e545_d_n0, eq38_e545_d_n1, eq38_e545_d_n2, eq38_e545_d_n3, eq38_e545_d_n4, eq38_e545_d_n5, eq38_e545_d_n6, eq38_e545_d_n7, eq38_e545_d_n8, eq38_e545_d_n9, eq38_e545_d_n10, eq38_e545_d_n11, eq38_e545_d_n12, eq38_e545_d_n13, eq38_e545_d_n14, eq38_e545_d_n15, eq38_e545_d_n16, eq38_e545_d_n17, eq38_e545_d_n18, eq38_e545_d_b0, eq38_e545_d_b1, eq38_e545_d_b2, eq38_e545_d_b3, eq38_e545_d_b4, eq38_e545_d_b5, eq38_e545_d_b6, eq38_e545_d_b7, eq38_e545_d_b8, eq38_e545_d_b9, eq38_e545_d_b10, eq38_e545_d_b11, eq38_e545_d_b12, eq38_e545_d_b13, eq38_e545_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e547;
        let eq38_node_derivatives: [f64; 19] = [eq38_e547_d_n0, eq38_e547_d_n1, eq38_e547_d_n2, eq38_e547_d_n3, eq38_e547_d_n4, eq38_e547_d_n5, eq38_e547_d_n6, eq38_e547_d_n7, eq38_e547_d_n8, eq38_e547_d_n9, eq38_e547_d_n10, eq38_e547_d_n11, eq38_e547_d_n12, eq38_e547_d_n13, eq38_e547_d_n14, eq38_e547_d_n15, eq38_e547_d_n16, eq38_e547_d_n17, eq38_e547_d_n18];
        let eq38_branch_derivatives: [f64; 15] = [eq38_e547_d_b0, eq38_e547_d_b1, eq38_e547_d_b2, eq38_e547_d_b3, eq38_e547_d_b4, eq38_e547_d_b5, eq38_e547_d_b6, eq38_e547_d_b7, eq38_e547_d_b8, eq38_e547_d_b9, eq38_e547_d_b10, eq38_e547_d_b11, eq38_e547_d_b12, eq38_e547_d_b13, eq38_e547_d_b14];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(12),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e555, eq39_e555_d_n0, eq39_e555_d_n1, eq39_e555_d_n2, eq39_e555_d_n3, eq39_e555_d_n4, eq39_e555_d_n5, eq39_e555_d_n6, eq39_e555_d_n7, eq39_e555_d_n8, eq39_e555_d_n9, eq39_e555_d_n10, eq39_e555_d_n11, eq39_e555_d_n12, eq39_e555_d_n13, eq39_e555_d_n14, eq39_e555_d_n15, eq39_e555_d_n16, eq39_e555_d_n17, eq39_e555_d_n18, eq39_e555_d_b0, eq39_e555_d_b1, eq39_e555_d_b2, eq39_e555_d_b3, eq39_e555_d_b4, eq39_e555_d_b5, eq39_e555_d_b6, eq39_e555_d_b7, eq39_e555_d_b8, eq39_e555_d_b9, eq39_e555_d_b10, eq39_e555_d_b11, eq39_e555_d_b12, eq39_e555_d_b13, eq39_e555_d_b14,) = {
    if (s.b[1849] && (p.p262 != 0.0)) {
        let eq39_e553: f64 = (s.v[552] * (nv8 - nv12));
        let eq39_e553_d_n0: f64 = (s.dn[552][0] * (nv8 - nv12));
        let eq39_e553_d_n1: f64 = (s.dn[552][1] * (nv8 - nv12));
        let eq39_e553_d_n2: f64 = (s.dn[552][2] * (nv8 - nv12));
        let eq39_e553_d_n3: f64 = (s.dn[552][3] * (nv8 - nv12));
        let eq39_e553_d_n4: f64 = (s.dn[552][4] * (nv8 - nv12));
        let eq39_e553_d_n5: f64 = (s.dn[552][5] * (nv8 - nv12));
        let eq39_e553_d_n6: f64 = (s.dn[552][6] * (nv8 - nv12));
        let eq39_e553_d_n7: f64 = (s.dn[552][7] * (nv8 - nv12));
        let eq39_e553_d_n8: f64 = ((s.dn[552][8] * (nv8 - nv12)) + s.v[552]);
        let eq39_e553_d_n9: f64 = (s.dn[552][9] * (nv8 - nv12));
        let eq39_e553_d_n10: f64 = (s.dn[552][10] * (nv8 - nv12));
        let eq39_e553_d_n11: f64 = (s.dn[552][11] * (nv8 - nv12));
        let eq39_e553_d_n12: f64 = ((s.dn[552][12] * (nv8 - nv12)) + (-s.v[552]));
        let eq39_e553_d_n13: f64 = (s.dn[552][13] * (nv8 - nv12));
        let eq39_e553_d_n14: f64 = (s.dn[552][14] * (nv8 - nv12));
        let eq39_e553_d_n15: f64 = (s.dn[552][15] * (nv8 - nv12));
        let eq39_e553_d_n16: f64 = (s.dn[552][16] * (nv8 - nv12));
        let eq39_e553_d_n17: f64 = (s.dn[552][17] * (nv8 - nv12));
        let eq39_e553_d_n18: f64 = (s.dn[552][18] * (nv8 - nv12));
        let eq39_e553_d_b0: f64 = (s.db[552][0] * (nv8 - nv12));
        let eq39_e553_d_b1: f64 = (s.db[552][1] * (nv8 - nv12));
        let eq39_e553_d_b2: f64 = (s.db[552][2] * (nv8 - nv12));
        let eq39_e553_d_b3: f64 = (s.db[552][3] * (nv8 - nv12));
        let eq39_e553_d_b4: f64 = (s.db[552][4] * (nv8 - nv12));
        let eq39_e553_d_b5: f64 = (s.db[552][5] * (nv8 - nv12));
        let eq39_e553_d_b6: f64 = (s.db[552][6] * (nv8 - nv12));
        let eq39_e553_d_b7: f64 = (s.db[552][7] * (nv8 - nv12));
        let eq39_e553_d_b8: f64 = (s.db[552][8] * (nv8 - nv12));
        let eq39_e553_d_b9: f64 = (s.db[552][9] * (nv8 - nv12));
        let eq39_e553_d_b10: f64 = (s.db[552][10] * (nv8 - nv12));
        let eq39_e553_d_b11: f64 = (s.db[552][11] * (nv8 - nv12));
        let eq39_e553_d_b12: f64 = (s.db[552][12] * (nv8 - nv12));
        let eq39_e553_d_b13: f64 = (s.db[552][13] * (nv8 - nv12));
        let eq39_e553_d_b14: f64 = (s.db[552][14] * (nv8 - nv12));
        (eq39_e553, eq39_e553_d_n0, eq39_e553_d_n1, eq39_e553_d_n2, eq39_e553_d_n3, eq39_e553_d_n4, eq39_e553_d_n5, eq39_e553_d_n6, eq39_e553_d_n7, eq39_e553_d_n8, eq39_e553_d_n9, eq39_e553_d_n10, eq39_e553_d_n11, eq39_e553_d_n12, eq39_e553_d_n13, eq39_e553_d_n14, eq39_e553_d_n15, eq39_e553_d_n16, eq39_e553_d_n17, eq39_e553_d_n18, eq39_e553_d_b0, eq39_e553_d_b1, eq39_e553_d_b2, eq39_e553_d_b3, eq39_e553_d_b4, eq39_e553_d_b5, eq39_e553_d_b6, eq39_e553_d_b7, eq39_e553_d_b8, eq39_e553_d_b9, eq39_e553_d_b10, eq39_e553_d_b11, eq39_e553_d_b12, eq39_e553_d_b13, eq39_e553_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e555;
        let eq39_node_derivatives: [f64; 19] = [eq39_e555_d_n0, eq39_e555_d_n1, eq39_e555_d_n2, eq39_e555_d_n3, eq39_e555_d_n4, eq39_e555_d_n5, eq39_e555_d_n6, eq39_e555_d_n7, eq39_e555_d_n8, eq39_e555_d_n9, eq39_e555_d_n10, eq39_e555_d_n11, eq39_e555_d_n12, eq39_e555_d_n13, eq39_e555_d_n14, eq39_e555_d_n15, eq39_e555_d_n16, eq39_e555_d_n17, eq39_e555_d_n18];
        let eq39_branch_derivatives: [f64; 15] = [eq39_e555_d_b0, eq39_e555_d_b1, eq39_e555_d_b2, eq39_e555_d_b3, eq39_e555_d_b4, eq39_e555_d_b5, eq39_e555_d_b6, eq39_e555_d_b7, eq39_e555_d_b8, eq39_e555_d_b9, eq39_e555_d_b10, eq39_e555_d_b11, eq39_e555_d_b12, eq39_e555_d_b13, eq39_e555_d_b14];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(12),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e562,) = {
    if (s.b[1849] && (p.p262 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e562;
        stamper.stamp_potential_const_local(
            6,
            eq40_value,
        );
        let (eq41_e569,) = {
    if (s.b[1849] && (p.p262 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e569;
        stamper.stamp_potential_const_local(
            7,
            eq41_value,
        );
        let (eq42_e575, eq42_e575_d_n0, eq42_e575_d_n1, eq42_e575_d_n2, eq42_e575_d_n3, eq42_e575_d_n4, eq42_e575_d_n5, eq42_e575_d_n6, eq42_e575_d_n7, eq42_e575_d_n8, eq42_e575_d_n9, eq42_e575_d_n10, eq42_e575_d_n11, eq42_e575_d_n12, eq42_e575_d_n13, eq42_e575_d_n14, eq42_e575_d_n15, eq42_e575_d_n16, eq42_e575_d_n17, eq42_e575_d_n18, eq42_e575_d_b0, eq42_e575_d_b1, eq42_e575_d_b2, eq42_e575_d_b3, eq42_e575_d_b4, eq42_e575_d_b5, eq42_e575_d_b6, eq42_e575_d_b7, eq42_e575_d_b8, eq42_e575_d_b9, eq42_e575_d_b10, eq42_e575_d_b11, eq42_e575_d_b12, eq42_e575_d_b13, eq42_e575_d_b14,) = {
    if (s.b[1849] && (p.p34 != 0.0)) {
        (s.v[582], s.dn[582][0], s.dn[582][1], s.dn[582][2], s.dn[582][3], s.dn[582][4], s.dn[582][5], s.dn[582][6], s.dn[582][7], s.dn[582][8], s.dn[582][9], s.dn[582][10], s.dn[582][11], s.dn[582][12], s.dn[582][13], s.dn[582][14], s.dn[582][15], s.dn[582][16], s.dn[582][17], s.dn[582][18], s.db[582][0], s.db[582][1], s.db[582][2], s.db[582][3], s.db[582][4], s.db[582][5], s.db[582][6], s.db[582][7], s.db[582][8], s.db[582][9], s.db[582][10], s.db[582][11], s.db[582][12], s.db[582][13], s.db[582][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e575;
        let eq42_node_derivatives: [f64; 19] = [eq42_e575_d_n0, eq42_e575_d_n1, eq42_e575_d_n2, eq42_e575_d_n3, eq42_e575_d_n4, eq42_e575_d_n5, eq42_e575_d_n6, eq42_e575_d_n7, eq42_e575_d_n8, eq42_e575_d_n9, eq42_e575_d_n10, eq42_e575_d_n11, eq42_e575_d_n12, eq42_e575_d_n13, eq42_e575_d_n14, eq42_e575_d_n15, eq42_e575_d_n16, eq42_e575_d_n17, eq42_e575_d_n18];
        let eq42_branch_derivatives: [f64; 15] = [eq42_e575_d_b0, eq42_e575_d_b1, eq42_e575_d_b2, eq42_e575_d_b3, eq42_e575_d_b4, eq42_e575_d_b5, eq42_e575_d_b6, eq42_e575_d_b7, eq42_e575_d_b8, eq42_e575_d_b9, eq42_e575_d_b10, eq42_e575_d_b11, eq42_e575_d_b12, eq42_e575_d_b13, eq42_e575_d_b14];
        stamper.stamp_current_dense_local(
            Some(18),
            None,
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq43_e581, eq43_e581_d_n0, eq43_e581_d_n1, eq43_e581_d_n2, eq43_e581_d_n3, eq43_e581_d_n4, eq43_e581_d_n5, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n8, eq43_e581_d_n9, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n14, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18, eq43_e581_d_b0, eq43_e581_d_b1, eq43_e581_d_b2, eq43_e581_d_b3, eq43_e581_d_b4, eq43_e581_d_b5, eq43_e581_d_b6, eq43_e581_d_b7, eq43_e581_d_b8, eq43_e581_d_b9, eq43_e581_d_b10, eq43_e581_d_b11, eq43_e581_d_b12, eq43_e581_d_b13, eq43_e581_d_b14,) = {
    if (s.b[1849] && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12], s.db[583][13], s.db[583][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e581;
        let eq43_node_derivatives: [f64; 19] = [eq43_e581_d_n0, eq43_e581_d_n1, eq43_e581_d_n2, eq43_e581_d_n3, eq43_e581_d_n4, eq43_e581_d_n5, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n8, eq43_e581_d_n9, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n14, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18];
        let eq43_branch_derivatives: [f64; 15] = [eq43_e581_d_b0, eq43_e581_d_b1, eq43_e581_d_b2, eq43_e581_d_b3, eq43_e581_d_b4, eq43_e581_d_b5, eq43_e581_d_b6, eq43_e581_d_b7, eq43_e581_d_b8, eq43_e581_d_b9, eq43_e581_d_b10, eq43_e581_d_b11, eq43_e581_d_b12, eq43_e581_d_b13, eq43_e581_d_b14];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e589, eq44_e589_d_n18,) = {
    if (s.b[1849] && (p.p34 != 0.0)) {
        let eq44_e587: f64 = ((nv18 - 0.0) * 1e-12);
        let eq44_e587_d_n18: f64 = 1e-12;
        (eq44_e587, eq44_e587_d_n18,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e589;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq44_value),
            18,
            multiplicity * (eq44_e589_d_n18),
        );
        let (eq45_e597, eq45_e597_d_n13,) = {
    if (s.b[1849] && (p.p34 != 0.0)) {
        let eq45_e595: f64 = ((nv13 - 0.0) * 1e-12);
        let eq45_e595_d_n13: f64 = 1e-12;
        (eq45_e595, eq45_e595_d_n13,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e597;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq45_value),
            13,
            multiplicity * (eq45_e597_d_n13),
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq46_e608, eq46_e608_d_n18,) = {
    if (s.b[1849] && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq46_e605);
        let eq46_e606_d_n18: f64 = (eq46_e603 * ddt_scale);
        (eq46_e606, eq46_e606_d_n18,)
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
    if (s.b[1849] && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq47_e616);
        let eq47_e617_d_n13: f64 = (eq47_e614 * ddt_scale);
        (eq47_e617, eq47_e617_d_n13,)
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
        let (eq48_e626,) = {
    if (s.b[1849] && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e626;
        stamper.stamp_potential_const_local(
            8,
            eq48_value,
        );
        let (eq49_e633,) = {
    if (s.b[1849] && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e633;
        stamper.stamp_potential_const_local(
            9,
            eq49_value,
        );
        let (eq50_e639, eq50_e639_d_n0, eq50_e639_d_n1, eq50_e639_d_n2, eq50_e639_d_n3, eq50_e639_d_n4, eq50_e639_d_n5, eq50_e639_d_n6, eq50_e639_d_n7, eq50_e639_d_n8, eq50_e639_d_n9, eq50_e639_d_n10, eq50_e639_d_n11, eq50_e639_d_n12, eq50_e639_d_n13, eq50_e639_d_n14, eq50_e639_d_n15, eq50_e639_d_n16, eq50_e639_d_n17, eq50_e639_d_n18, eq50_e639_d_b0, eq50_e639_d_b1, eq50_e639_d_b2, eq50_e639_d_b3, eq50_e639_d_b4, eq50_e639_d_b5, eq50_e639_d_b6, eq50_e639_d_b7, eq50_e639_d_b8, eq50_e639_d_b9, eq50_e639_d_b10, eq50_e639_d_b11, eq50_e639_d_b12, eq50_e639_d_b13, eq50_e639_d_b14,) = {
    if (s.b[1849] && s.b[1850]) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12], s.db[592][13], s.db[592][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e639;
        let eq50_node_derivatives: [f64; 19] = [eq50_e639_d_n0, eq50_e639_d_n1, eq50_e639_d_n2, eq50_e639_d_n3, eq50_e639_d_n4, eq50_e639_d_n5, eq50_e639_d_n6, eq50_e639_d_n7, eq50_e639_d_n8, eq50_e639_d_n9, eq50_e639_d_n10, eq50_e639_d_n11, eq50_e639_d_n12, eq50_e639_d_n13, eq50_e639_d_n14, eq50_e639_d_n15, eq50_e639_d_n16, eq50_e639_d_n17, eq50_e639_d_n18];
        let eq50_branch_derivatives: [f64; 15] = [eq50_e639_d_b0, eq50_e639_d_b1, eq50_e639_d_b2, eq50_e639_d_b3, eq50_e639_d_b4, eq50_e639_d_b5, eq50_e639_d_b6, eq50_e639_d_b7, eq50_e639_d_b8, eq50_e639_d_b9, eq50_e639_d_b10, eq50_e639_d_b11, eq50_e639_d_b12, eq50_e639_d_b13, eq50_e639_d_b14];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e647, eq51_e647_d_n17,) = {
    if (s.b[1849] && s.b[1850]) {
        let eq51_e645: f64 = ((nv17 - 0.0) * 1e-12);
        let eq51_e645_d_n17: f64 = 1e-12;
        (eq51_e645, eq51_e645_d_n17,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e647;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq51_value),
            17,
            multiplicity * (eq51_e647_d_n17),
        );
        let (eq52_e658, eq52_e658_d_n17,) = {
    if (s.b[1849] && s.b[1850]) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq52_e655);
        let eq52_e656_d_n17: f64 = (eq52_e653 * ddt_scale);
        (eq52_e656, eq52_e656_d_n17,)
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
        let (eq53_e665,) = {
    if (s.b[1849] && (!s.b[1850])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e665;
        stamper.stamp_potential_const_local(
            10,
            eq53_value,
        );
        let (eq54_e674, eq54_e674_d_n0, eq54_e674_d_n1, eq54_e674_d_n2, eq54_e674_d_n3, eq54_e674_d_n4, eq54_e674_d_n5, eq54_e674_d_n6, eq54_e674_d_n7, eq54_e674_d_n8, eq54_e674_d_n9, eq54_e674_d_n10, eq54_e674_d_n11, eq54_e674_d_n12, eq54_e674_d_n13, eq54_e674_d_n14, eq54_e674_d_n15, eq54_e674_d_n16, eq54_e674_d_n17, eq54_e674_d_n18, eq54_e674_d_b0, eq54_e674_d_b1, eq54_e674_d_b2, eq54_e674_d_b3, eq54_e674_d_b4, eq54_e674_d_b5, eq54_e674_d_b6, eq54_e674_d_b7, eq54_e674_d_b8, eq54_e674_d_b9, eq54_e674_d_b10, eq54_e674_d_b11, eq54_e674_d_b12, eq54_e674_d_b13, eq54_e674_d_b14,) = {
    if (!s.b[1849]) {
        let eq54_e671: f64 = (s.v[311] + s.v[263]);
        let eq54_e671_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);
        let eq54_e671_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);
        let eq54_e671_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);
        let eq54_e671_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);
        let eq54_e671_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);
        let eq54_e671_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);
        let eq54_e671_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);
        let eq54_e671_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);
        let eq54_e671_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);
        let eq54_e671_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);
        let eq54_e671_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);
        let eq54_e671_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);
        let eq54_e671_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);
        let eq54_e671_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);
        let eq54_e671_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);
        let eq54_e671_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);
        let eq54_e671_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);
        let eq54_e671_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);
        let eq54_e671_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);
        let eq54_e671_d_b0: f64 = (s.db[311][0] + s.db[263][0]);
        let eq54_e671_d_b1: f64 = (s.db[311][1] + s.db[263][1]);
        let eq54_e671_d_b2: f64 = (s.db[311][2] + s.db[263][2]);
        let eq54_e671_d_b3: f64 = (s.db[311][3] + s.db[263][3]);
        let eq54_e671_d_b4: f64 = (s.db[311][4] + s.db[263][4]);
        let eq54_e671_d_b5: f64 = (s.db[311][5] + s.db[263][5]);
        let eq54_e671_d_b6: f64 = (s.db[311][6] + s.db[263][6]);
        let eq54_e671_d_b7: f64 = (s.db[311][7] + s.db[263][7]);
        let eq54_e671_d_b8: f64 = (s.db[311][8] + s.db[263][8]);
        let eq54_e671_d_b9: f64 = (s.db[311][9] + s.db[263][9]);
        let eq54_e671_d_b10: f64 = (s.db[311][10] + s.db[263][10]);
        let eq54_e671_d_b11: f64 = (s.db[311][11] + s.db[263][11]);
        let eq54_e671_d_b12: f64 = (s.db[311][12] + s.db[263][12]);
        let eq54_e671_d_b13: f64 = (s.db[311][13] + s.db[263][13]);
        let eq54_e671_d_b14: f64 = (s.db[311][14] + s.db[263][14]);
        let eq54_e672: f64 = (p.p50 * eq54_e671);
        let eq54_e672_d_n0: f64 = (p.p50 * eq54_e671_d_n0);
        let eq54_e672_d_n1: f64 = (p.p50 * eq54_e671_d_n1);
        let eq54_e672_d_n2: f64 = (p.p50 * eq54_e671_d_n2);
        let eq54_e672_d_n3: f64 = (p.p50 * eq54_e671_d_n3);
        let eq54_e672_d_n4: f64 = (p.p50 * eq54_e671_d_n4);
        let eq54_e672_d_n5: f64 = (p.p50 * eq54_e671_d_n5);
        let eq54_e672_d_n6: f64 = (p.p50 * eq54_e671_d_n6);
        let eq54_e672_d_n7: f64 = (p.p50 * eq54_e671_d_n7);
        let eq54_e672_d_n8: f64 = (p.p50 * eq54_e671_d_n8);
        let eq54_e672_d_n9: f64 = (p.p50 * eq54_e671_d_n9);
        let eq54_e672_d_n10: f64 = (p.p50 * eq54_e671_d_n10);
        let eq54_e672_d_n11: f64 = (p.p50 * eq54_e671_d_n11);
        let eq54_e672_d_n12: f64 = (p.p50 * eq54_e671_d_n12);
        let eq54_e672_d_n13: f64 = (p.p50 * eq54_e671_d_n13);
        let eq54_e672_d_n14: f64 = (p.p50 * eq54_e671_d_n14);
        let eq54_e672_d_n15: f64 = (p.p50 * eq54_e671_d_n15);
        let eq54_e672_d_n16: f64 = (p.p50 * eq54_e671_d_n16);
        let eq54_e672_d_n17: f64 = (p.p50 * eq54_e671_d_n17);
        let eq54_e672_d_n18: f64 = (p.p50 * eq54_e671_d_n18);
        let eq54_e672_d_b0: f64 = (p.p50 * eq54_e671_d_b0);
        let eq54_e672_d_b1: f64 = (p.p50 * eq54_e671_d_b1);
        let eq54_e672_d_b2: f64 = (p.p50 * eq54_e671_d_b2);
        let eq54_e672_d_b3: f64 = (p.p50 * eq54_e671_d_b3);
        let eq54_e672_d_b4: f64 = (p.p50 * eq54_e671_d_b4);
        let eq54_e672_d_b5: f64 = (p.p50 * eq54_e671_d_b5);
        let eq54_e672_d_b6: f64 = (p.p50 * eq54_e671_d_b6);
        let eq54_e672_d_b7: f64 = (p.p50 * eq54_e671_d_b7);
        let eq54_e672_d_b8: f64 = (p.p50 * eq54_e671_d_b8);
        let eq54_e672_d_b9: f64 = (p.p50 * eq54_e671_d_b9);
        let eq54_e672_d_b10: f64 = (p.p50 * eq54_e671_d_b10);
        let eq54_e672_d_b11: f64 = (p.p50 * eq54_e671_d_b11);
        let eq54_e672_d_b12: f64 = (p.p50 * eq54_e671_d_b12);
        let eq54_e672_d_b13: f64 = (p.p50 * eq54_e671_d_b13);
        let eq54_e672_d_b14: f64 = (p.p50 * eq54_e671_d_b14);
        (eq54_e672, eq54_e672_d_n0, eq54_e672_d_n1, eq54_e672_d_n2, eq54_e672_d_n3, eq54_e672_d_n4, eq54_e672_d_n5, eq54_e672_d_n6, eq54_e672_d_n7, eq54_e672_d_n8, eq54_e672_d_n9, eq54_e672_d_n10, eq54_e672_d_n11, eq54_e672_d_n12, eq54_e672_d_n13, eq54_e672_d_n14, eq54_e672_d_n15, eq54_e672_d_n16, eq54_e672_d_n17, eq54_e672_d_n18, eq54_e672_d_b0, eq54_e672_d_b1, eq54_e672_d_b2, eq54_e672_d_b3, eq54_e672_d_b4, eq54_e672_d_b5, eq54_e672_d_b6, eq54_e672_d_b7, eq54_e672_d_b8, eq54_e672_d_b9, eq54_e672_d_b10, eq54_e672_d_b11, eq54_e672_d_b12, eq54_e672_d_b13, eq54_e672_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e674;
        let eq54_node_derivatives: [f64; 19] = [eq54_e674_d_n0, eq54_e674_d_n1, eq54_e674_d_n2, eq54_e674_d_n3, eq54_e674_d_n4, eq54_e674_d_n5, eq54_e674_d_n6, eq54_e674_d_n7, eq54_e674_d_n8, eq54_e674_d_n9, eq54_e674_d_n10, eq54_e674_d_n11, eq54_e674_d_n12, eq54_e674_d_n13, eq54_e674_d_n14, eq54_e674_d_n15, eq54_e674_d_n16, eq54_e674_d_n17, eq54_e674_d_n18];
        let eq54_branch_derivatives: [f64; 15] = [eq54_e674_d_b0, eq54_e674_d_b1, eq54_e674_d_b2, eq54_e674_d_b3, eq54_e674_d_b4, eq54_e674_d_b5, eq54_e674_d_b6, eq54_e674_d_b7, eq54_e674_d_b8, eq54_e674_d_b9, eq54_e674_d_b10, eq54_e674_d_b11, eq54_e674_d_b12, eq54_e674_d_b13, eq54_e674_d_b14];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq54_value),
            &eq54_node_derivatives,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e683, eq55_e683_d_n0, eq55_e683_d_n1, eq55_e683_d_n2, eq55_e683_d_n3, eq55_e683_d_n4, eq55_e683_d_n5, eq55_e683_d_n6, eq55_e683_d_n7, eq55_e683_d_n8, eq55_e683_d_n9, eq55_e683_d_n10, eq55_e683_d_n11, eq55_e683_d_n12, eq55_e683_d_n13, eq55_e683_d_n14, eq55_e683_d_n15, eq55_e683_d_n16, eq55_e683_d_n17, eq55_e683_d_n18, eq55_e683_d_b0, eq55_e683_d_b1, eq55_e683_d_b2, eq55_e683_d_b3, eq55_e683_d_b4, eq55_e683_d_b5, eq55_e683_d_b6, eq55_e683_d_b7, eq55_e683_d_b8, eq55_e683_d_b9, eq55_e683_d_b10, eq55_e683_d_b11, eq55_e683_d_b12, eq55_e683_d_b13, eq55_e683_d_b14,) = {
    if (!s.b[1849]) {
        let eq55_e680: f64 = (s.v[312] + s.v[573]);
        let eq55_e680_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);
        let eq55_e680_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);
        let eq55_e680_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);
        let eq55_e680_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);
        let eq55_e680_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);
        let eq55_e680_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);
        let eq55_e680_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);
        let eq55_e680_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);
        let eq55_e680_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);
        let eq55_e680_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);
        let eq55_e680_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);
        let eq55_e680_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);
        let eq55_e680_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);
        let eq55_e680_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);
        let eq55_e680_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);
        let eq55_e680_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);
        let eq55_e680_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);
        let eq55_e680_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);
        let eq55_e680_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);
        let eq55_e680_d_b0: f64 = (s.db[312][0] + s.db[573][0]);
        let eq55_e680_d_b1: f64 = (s.db[312][1] + s.db[573][1]);
        let eq55_e680_d_b2: f64 = (s.db[312][2] + s.db[573][2]);
        let eq55_e680_d_b3: f64 = (s.db[312][3] + s.db[573][3]);
        let eq55_e680_d_b4: f64 = (s.db[312][4] + s.db[573][4]);
        let eq55_e680_d_b5: f64 = (s.db[312][5] + s.db[573][5]);
        let eq55_e680_d_b6: f64 = (s.db[312][6] + s.db[573][6]);
        let eq55_e680_d_b7: f64 = (s.db[312][7] + s.db[573][7]);
        let eq55_e680_d_b8: f64 = (s.db[312][8] + s.db[573][8]);
        let eq55_e680_d_b9: f64 = (s.db[312][9] + s.db[573][9]);
        let eq55_e680_d_b10: f64 = (s.db[312][10] + s.db[573][10]);
        let eq55_e680_d_b11: f64 = (s.db[312][11] + s.db[573][11]);
        let eq55_e680_d_b12: f64 = (s.db[312][12] + s.db[573][12]);
        let eq55_e680_d_b13: f64 = (s.db[312][13] + s.db[573][13]);
        let eq55_e680_d_b14: f64 = (s.db[312][14] + s.db[573][14]);
        let eq55_e681: f64 = (p.p50 * eq55_e680);
        let eq55_e681_d_n0: f64 = (p.p50 * eq55_e680_d_n0);
        let eq55_e681_d_n1: f64 = (p.p50 * eq55_e680_d_n1);
        let eq55_e681_d_n2: f64 = (p.p50 * eq55_e680_d_n2);
        let eq55_e681_d_n3: f64 = (p.p50 * eq55_e680_d_n3);
        let eq55_e681_d_n4: f64 = (p.p50 * eq55_e680_d_n4);
        let eq55_e681_d_n5: f64 = (p.p50 * eq55_e680_d_n5);
        let eq55_e681_d_n6: f64 = (p.p50 * eq55_e680_d_n6);
        let eq55_e681_d_n7: f64 = (p.p50 * eq55_e680_d_n7);
        let eq55_e681_d_n8: f64 = (p.p50 * eq55_e680_d_n8);
        let eq55_e681_d_n9: f64 = (p.p50 * eq55_e680_d_n9);
        let eq55_e681_d_n10: f64 = (p.p50 * eq55_e680_d_n10);
        let eq55_e681_d_n11: f64 = (p.p50 * eq55_e680_d_n11);
        let eq55_e681_d_n12: f64 = (p.p50 * eq55_e680_d_n12);
        let eq55_e681_d_n13: f64 = (p.p50 * eq55_e680_d_n13);
        let eq55_e681_d_n14: f64 = (p.p50 * eq55_e680_d_n14);
        let eq55_e681_d_n15: f64 = (p.p50 * eq55_e680_d_n15);
        let eq55_e681_d_n16: f64 = (p.p50 * eq55_e680_d_n16);
        let eq55_e681_d_n17: f64 = (p.p50 * eq55_e680_d_n17);
        let eq55_e681_d_n18: f64 = (p.p50 * eq55_e680_d_n18);
        let eq55_e681_d_b0: f64 = (p.p50 * eq55_e680_d_b0);
        let eq55_e681_d_b1: f64 = (p.p50 * eq55_e680_d_b1);
        let eq55_e681_d_b2: f64 = (p.p50 * eq55_e680_d_b2);
        let eq55_e681_d_b3: f64 = (p.p50 * eq55_e680_d_b3);
        let eq55_e681_d_b4: f64 = (p.p50 * eq55_e680_d_b4);
        let eq55_e681_d_b5: f64 = (p.p50 * eq55_e680_d_b5);
        let eq55_e681_d_b6: f64 = (p.p50 * eq55_e680_d_b6);
        let eq55_e681_d_b7: f64 = (p.p50 * eq55_e680_d_b7);
        let eq55_e681_d_b8: f64 = (p.p50 * eq55_e680_d_b8);
        let eq55_e681_d_b9: f64 = (p.p50 * eq55_e680_d_b9);
        let eq55_e681_d_b10: f64 = (p.p50 * eq55_e680_d_b10);
        let eq55_e681_d_b11: f64 = (p.p50 * eq55_e680_d_b11);
        let eq55_e681_d_b12: f64 = (p.p50 * eq55_e680_d_b12);
        let eq55_e681_d_b13: f64 = (p.p50 * eq55_e680_d_b13);
        let eq55_e681_d_b14: f64 = (p.p50 * eq55_e680_d_b14);
        (eq55_e681, eq55_e681_d_n0, eq55_e681_d_n1, eq55_e681_d_n2, eq55_e681_d_n3, eq55_e681_d_n4, eq55_e681_d_n5, eq55_e681_d_n6, eq55_e681_d_n7, eq55_e681_d_n8, eq55_e681_d_n9, eq55_e681_d_n10, eq55_e681_d_n11, eq55_e681_d_n12, eq55_e681_d_n13, eq55_e681_d_n14, eq55_e681_d_n15, eq55_e681_d_n16, eq55_e681_d_n17, eq55_e681_d_n18, eq55_e681_d_b0, eq55_e681_d_b1, eq55_e681_d_b2, eq55_e681_d_b3, eq55_e681_d_b4, eq55_e681_d_b5, eq55_e681_d_b6, eq55_e681_d_b7, eq55_e681_d_b8, eq55_e681_d_b9, eq55_e681_d_b10, eq55_e681_d_b11, eq55_e681_d_b12, eq55_e681_d_b13, eq55_e681_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e683;
        let eq55_node_derivatives: [f64; 19] = [eq55_e683_d_n0, eq55_e683_d_n1, eq55_e683_d_n2, eq55_e683_d_n3, eq55_e683_d_n4, eq55_e683_d_n5, eq55_e683_d_n6, eq55_e683_d_n7, eq55_e683_d_n8, eq55_e683_d_n9, eq55_e683_d_n10, eq55_e683_d_n11, eq55_e683_d_n12, eq55_e683_d_n13, eq55_e683_d_n14, eq55_e683_d_n15, eq55_e683_d_n16, eq55_e683_d_n17, eq55_e683_d_n18];
        let eq55_branch_derivatives: [f64; 15] = [eq55_e683_d_b0, eq55_e683_d_b1, eq55_e683_d_b2, eq55_e683_d_b3, eq55_e683_d_b4, eq55_e683_d_b5, eq55_e683_d_b6, eq55_e683_d_b7, eq55_e683_d_b8, eq55_e683_d_b9, eq55_e683_d_b10, eq55_e683_d_b11, eq55_e683_d_b12, eq55_e683_d_b13, eq55_e683_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e688,) = {
    if (!s.b[1849]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e688;
        stamper.stamp_potential_const_local(
            11,
            eq56_value,
        );
        let (eq57_e695, eq57_e695_d_n0, eq57_e695_d_n1, eq57_e695_d_n2, eq57_e695_d_n3, eq57_e695_d_n4, eq57_e695_d_n5, eq57_e695_d_n6, eq57_e695_d_n7, eq57_e695_d_n8, eq57_e695_d_n9, eq57_e695_d_n10, eq57_e695_d_n11, eq57_e695_d_n12, eq57_e695_d_n13, eq57_e695_d_n14, eq57_e695_d_n15, eq57_e695_d_n16, eq57_e695_d_n17, eq57_e695_d_n18, eq57_e695_d_b0, eq57_e695_d_b1, eq57_e695_d_b2, eq57_e695_d_b3, eq57_e695_d_b4, eq57_e695_d_b5, eq57_e695_d_b6, eq57_e695_d_b7, eq57_e695_d_b8, eq57_e695_d_b9, eq57_e695_d_b10, eq57_e695_d_b11, eq57_e695_d_b12, eq57_e695_d_b13, eq57_e695_d_b14,) = {
    if ((!s.b[1849]) && (p.p37 != 0.0)) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12], s.db[592][13], s.db[592][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e695;
        let eq57_node_derivatives: [f64; 19] = [eq57_e695_d_n0, eq57_e695_d_n1, eq57_e695_d_n2, eq57_e695_d_n3, eq57_e695_d_n4, eq57_e695_d_n5, eq57_e695_d_n6, eq57_e695_d_n7, eq57_e695_d_n8, eq57_e695_d_n9, eq57_e695_d_n10, eq57_e695_d_n11, eq57_e695_d_n12, eq57_e695_d_n13, eq57_e695_d_n14, eq57_e695_d_n15, eq57_e695_d_n16, eq57_e695_d_n17, eq57_e695_d_n18];
        let eq57_branch_derivatives: [f64; 15] = [eq57_e695_d_b0, eq57_e695_d_b1, eq57_e695_d_b2, eq57_e695_d_b3, eq57_e695_d_b4, eq57_e695_d_b5, eq57_e695_d_b6, eq57_e695_d_b7, eq57_e695_d_b8, eq57_e695_d_b9, eq57_e695_d_b10, eq57_e695_d_b11, eq57_e695_d_b12, eq57_e695_d_b13, eq57_e695_d_b14];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e704, eq58_e704_d_n17,) = {
    if ((!s.b[1849]) && (p.p37 != 0.0)) {
        let eq58_e702: f64 = ((nv17 - 0.0) * 1e-12);
        let eq58_e702_d_n17: f64 = 1e-12;
        (eq58_e702, eq58_e702_d_n17,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e704;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq58_value),
            17,
            multiplicity * (eq58_e704_d_n17),
        );
        let (eq59_e716, eq59_e716_d_n17,) = {
    if ((!s.b[1849]) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq59_e713);
        let eq59_e714_d_n17: f64 = (eq59_e711 * ddt_scale);
        (eq59_e714, eq59_e714_d_n17,)
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
        let (eq60_e724,) = {
    if ((!s.b[1849]) && (p.p37 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e724;
        stamper.stamp_potential_const_local(
            12,
            eq60_value,
        );
        let (eq61_e731, eq61_e731_d_n0, eq61_e731_d_n1, eq61_e731_d_n2, eq61_e731_d_n3, eq61_e731_d_n4, eq61_e731_d_n5, eq61_e731_d_n6, eq61_e731_d_n7, eq61_e731_d_n8, eq61_e731_d_n9, eq61_e731_d_n10, eq61_e731_d_n11, eq61_e731_d_n12, eq61_e731_d_n13, eq61_e731_d_n14, eq61_e731_d_n15, eq61_e731_d_n16, eq61_e731_d_n17, eq61_e731_d_n18, eq61_e731_d_b0, eq61_e731_d_b1, eq61_e731_d_b2, eq61_e731_d_b3, eq61_e731_d_b4, eq61_e731_d_b5, eq61_e731_d_b6, eq61_e731_d_b7, eq61_e731_d_b8, eq61_e731_d_b9, eq61_e731_d_b10, eq61_e731_d_b11, eq61_e731_d_b12, eq61_e731_d_b13, eq61_e731_d_b14,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        (s.v[574], s.dn[574][0], s.dn[574][1], s.dn[574][2], s.dn[574][3], s.dn[574][4], s.dn[574][5], s.dn[574][6], s.dn[574][7], s.dn[574][8], s.dn[574][9], s.dn[574][10], s.dn[574][11], s.dn[574][12], s.dn[574][13], s.dn[574][14], s.dn[574][15], s.dn[574][16], s.dn[574][17], s.dn[574][18], s.db[574][0], s.db[574][1], s.db[574][2], s.db[574][3], s.db[574][4], s.db[574][5], s.db[574][6], s.db[574][7], s.db[574][8], s.db[574][9], s.db[574][10], s.db[574][11], s.db[574][12], s.db[574][13], s.db[574][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e731;
        let eq61_node_derivatives: [f64; 19] = [eq61_e731_d_n0, eq61_e731_d_n1, eq61_e731_d_n2, eq61_e731_d_n3, eq61_e731_d_n4, eq61_e731_d_n5, eq61_e731_d_n6, eq61_e731_d_n7, eq61_e731_d_n8, eq61_e731_d_n9, eq61_e731_d_n10, eq61_e731_d_n11, eq61_e731_d_n12, eq61_e731_d_n13, eq61_e731_d_n14, eq61_e731_d_n15, eq61_e731_d_n16, eq61_e731_d_n17, eq61_e731_d_n18];
        let eq61_branch_derivatives: [f64; 15] = [eq61_e731_d_b0, eq61_e731_d_b1, eq61_e731_d_b2, eq61_e731_d_b3, eq61_e731_d_b4, eq61_e731_d_b5, eq61_e731_d_b6, eq61_e731_d_b7, eq61_e731_d_b8, eq61_e731_d_b9, eq61_e731_d_b10, eq61_e731_d_b11, eq61_e731_d_b12, eq61_e731_d_b13, eq61_e731_d_b14];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e738, eq62_e738_d_n0, eq62_e738_d_n1, eq62_e738_d_n2, eq62_e738_d_n3, eq62_e738_d_n4, eq62_e738_d_n5, eq62_e738_d_n6, eq62_e738_d_n7, eq62_e738_d_n8, eq62_e738_d_n9, eq62_e738_d_n10, eq62_e738_d_n11, eq62_e738_d_n12, eq62_e738_d_n13, eq62_e738_d_n14, eq62_e738_d_n15, eq62_e738_d_n16, eq62_e738_d_n17, eq62_e738_d_n18, eq62_e738_d_b0, eq62_e738_d_b1, eq62_e738_d_b2, eq62_e738_d_b3, eq62_e738_d_b4, eq62_e738_d_b5, eq62_e738_d_b6, eq62_e738_d_b7, eq62_e738_d_b8, eq62_e738_d_b9, eq62_e738_d_b10, eq62_e738_d_b11, eq62_e738_d_b12, eq62_e738_d_b13, eq62_e738_d_b14,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        (s.v[575], s.dn[575][0], s.dn[575][1], s.dn[575][2], s.dn[575][3], s.dn[575][4], s.dn[575][5], s.dn[575][6], s.dn[575][7], s.dn[575][8], s.dn[575][9], s.dn[575][10], s.dn[575][11], s.dn[575][12], s.dn[575][13], s.dn[575][14], s.dn[575][15], s.dn[575][16], s.dn[575][17], s.dn[575][18], s.db[575][0], s.db[575][1], s.db[575][2], s.db[575][3], s.db[575][4], s.db[575][5], s.db[575][6], s.db[575][7], s.db[575][8], s.db[575][9], s.db[575][10], s.db[575][11], s.db[575][12], s.db[575][13], s.db[575][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e738;
        let eq62_node_derivatives: [f64; 19] = [eq62_e738_d_n0, eq62_e738_d_n1, eq62_e738_d_n2, eq62_e738_d_n3, eq62_e738_d_n4, eq62_e738_d_n5, eq62_e738_d_n6, eq62_e738_d_n7, eq62_e738_d_n8, eq62_e738_d_n9, eq62_e738_d_n10, eq62_e738_d_n11, eq62_e738_d_n12, eq62_e738_d_n13, eq62_e738_d_n14, eq62_e738_d_n15, eq62_e738_d_n16, eq62_e738_d_n17, eq62_e738_d_n18];
        let eq62_branch_derivatives: [f64; 15] = [eq62_e738_d_b0, eq62_e738_d_b1, eq62_e738_d_b2, eq62_e738_d_b3, eq62_e738_d_b4, eq62_e738_d_b5, eq62_e738_d_b6, eq62_e738_d_b7, eq62_e738_d_b8, eq62_e738_d_b9, eq62_e738_d_b10, eq62_e738_d_b11, eq62_e738_d_b12, eq62_e738_d_b13, eq62_e738_d_b14];
        stamper.stamp_current_dense_local(
            Some(16),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e745, eq63_e745_d_n0, eq63_e745_d_n1, eq63_e745_d_n2, eq63_e745_d_n3, eq63_e745_d_n4, eq63_e745_d_n5, eq63_e745_d_n6, eq63_e745_d_n7, eq63_e745_d_n8, eq63_e745_d_n9, eq63_e745_d_n10, eq63_e745_d_n11, eq63_e745_d_n12, eq63_e745_d_n13, eq63_e745_d_n14, eq63_e745_d_n15, eq63_e745_d_n16, eq63_e745_d_n17, eq63_e745_d_n18, eq63_e745_d_b0, eq63_e745_d_b1, eq63_e745_d_b2, eq63_e745_d_b3, eq63_e745_d_b4, eq63_e745_d_b5, eq63_e745_d_b6, eq63_e745_d_b7, eq63_e745_d_b8, eq63_e745_d_b9, eq63_e745_d_b10, eq63_e745_d_b11, eq63_e745_d_b12, eq63_e745_d_b13, eq63_e745_d_b14,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12], s.db[583][13], s.db[583][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e745;
        let eq63_node_derivatives: [f64; 19] = [eq63_e745_d_n0, eq63_e745_d_n1, eq63_e745_d_n2, eq63_e745_d_n3, eq63_e745_d_n4, eq63_e745_d_n5, eq63_e745_d_n6, eq63_e745_d_n7, eq63_e745_d_n8, eq63_e745_d_n9, eq63_e745_d_n10, eq63_e745_d_n11, eq63_e745_d_n12, eq63_e745_d_n13, eq63_e745_d_n14, eq63_e745_d_n15, eq63_e745_d_n16, eq63_e745_d_n17, eq63_e745_d_n18];
        let eq63_branch_derivatives: [f64; 15] = [eq63_e745_d_b0, eq63_e745_d_b1, eq63_e745_d_b2, eq63_e745_d_b3, eq63_e745_d_b4, eq63_e745_d_b5, eq63_e745_d_b6, eq63_e745_d_b7, eq63_e745_d_b8, eq63_e745_d_b9, eq63_e745_d_b10, eq63_e745_d_b11, eq63_e745_d_b12, eq63_e745_d_b13, eq63_e745_d_b14];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e754, eq64_e754_d_n15,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq64_e752: f64 = ((nv15 - 0.0) * 1e-12);
        let eq64_e752_d_n15: f64 = 1e-12;
        (eq64_e752, eq64_e752_d_n15,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e754;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq64_value),
            15,
            multiplicity * (eq64_e754_d_n15),
        );
        let (eq65_e763, eq65_e763_d_n16,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq65_e761: f64 = ((nv16 - 0.0) * 1e-12);
        let eq65_e761_d_n16: f64 = 1e-12;
        (eq65_e761, eq65_e761_d_n16,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e763;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq65_value),
            16,
            multiplicity * (eq65_e763_d_n16),
        );
        let (eq66_e772, eq66_e772_d_n13,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq66_e770: f64 = ((nv13 - 0.0) * 1e-12);
        let eq66_e770_d_n13: f64 = 1e-12;
        (eq66_e770, eq66_e770_d_n13,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e772;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq66_value),
            13,
            multiplicity * (eq66_e772_d_n13),
        );
        let (eq67_e784, eq67_e784_d_n15,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq67_e781);
        let eq67_e782_d_n15: f64 = (eq67_e779 * ddt_scale);
        (eq67_e782, eq67_e782_d_n15,)
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
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq68_e796, eq68_e796_d_n16,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq68_e793);
        let eq68_e794_d_n16: f64 = (eq68_e791 * ddt_scale);
        (eq68_e794, eq68_e794_d_n16,)
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
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq69_e805);
        let eq69_e806_d_n13: f64 = (eq69_e803 * ddt_scale);
        (eq69_e806, eq69_e806_d_n13,)
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
        let (eq70_e816,) = {
    if ((!s.b[1849]) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e816;
        stamper.stamp_potential_const_local(
            13,
            eq70_value,
        );
        let (eq71_e824,) = {
    if ((!s.b[1849]) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e824;
        stamper.stamp_potential_const_local(
            14,
            eq71_value,
        );
        let (eq72_e832,) = {
    if ((!s.b[1849]) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq72_value: f64 = eq72_e832;
        stamper.stamp_potential_const_local(
            15,
            eq72_value,
        );
        let (eq73_e836,) = {
    if s.b[1851] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e836;
        stamper.stamp_potential_const_local(
            16,
            eq73_value,
        );
        let (eq74_e841,) = {
    if (!s.b[1851]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e841;
        stamper.stamp_potential_const_local(
            17,
            eq74_value,
        );
        let (eq75_e846,) = {
    if (!s.b[1851]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e846;
        stamper.stamp_potential_const_local(
            18,
            eq75_value,
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq10_e359_q: f64 = s.v[594];
        let eq10_e360: f64 = (p.p50 * s.v[594]);
        let eq10_e360_d_n0: f64 = (p.p50 * s.dn[594][0]);
        let eq10_e360_d_n1: f64 = (p.p50 * s.dn[594][1]);
        let eq10_e360_d_n2: f64 = (p.p50 * s.dn[594][2]);
        let eq10_e360_d_n3: f64 = (p.p50 * s.dn[594][3]);
        let eq10_e360_d_n4: f64 = (p.p50 * s.dn[594][4]);
        let eq10_e360_d_n5: f64 = (p.p50 * s.dn[594][5]);
        let eq10_e360_d_n6: f64 = (p.p50 * s.dn[594][6]);
        let eq10_e360_d_n7: f64 = (p.p50 * s.dn[594][7]);
        let eq10_e360_d_n8: f64 = (p.p50 * s.dn[594][8]);
        let eq10_e360_d_n9: f64 = (p.p50 * s.dn[594][9]);
        let eq10_e360_d_n10: f64 = (p.p50 * s.dn[594][10]);
        let eq10_e360_d_n11: f64 = (p.p50 * s.dn[594][11]);
        let eq10_e360_d_n12: f64 = (p.p50 * s.dn[594][12]);
        let eq10_e360_d_n13: f64 = (p.p50 * s.dn[594][13]);
        let eq10_e360_d_n14: f64 = (p.p50 * s.dn[594][14]);
        let eq10_e360_d_n15: f64 = (p.p50 * s.dn[594][15]);
        let eq10_e360_d_n16: f64 = (p.p50 * s.dn[594][16]);
        let eq10_e360_d_n17: f64 = (p.p50 * s.dn[594][17]);
        let eq10_e360_d_n18: f64 = (p.p50 * s.dn[594][18]);
        let eq10_e360_d_b0: f64 = (p.p50 * s.db[594][0]);
        let eq10_e360_d_b1: f64 = (p.p50 * s.db[594][1]);
        let eq10_e360_d_b2: f64 = (p.p50 * s.db[594][2]);
        let eq10_e360_d_b3: f64 = (p.p50 * s.db[594][3]);
        let eq10_e360_d_b4: f64 = (p.p50 * s.db[594][4]);
        let eq10_e360_d_b5: f64 = (p.p50 * s.db[594][5]);
        let eq10_e360_d_b6: f64 = (p.p50 * s.db[594][6]);
        let eq10_e360_d_b7: f64 = (p.p50 * s.db[594][7]);
        let eq10_e360_d_b8: f64 = (p.p50 * s.db[594][8]);
        let eq10_e360_d_b9: f64 = (p.p50 * s.db[594][9]);
        let eq10_e360_d_b10: f64 = (p.p50 * s.db[594][10]);
        let eq10_e360_d_b11: f64 = (p.p50 * s.db[594][11]);
        let eq10_e360_d_b12: f64 = (p.p50 * s.db[594][12]);
        let eq10_e360_d_b13: f64 = (p.p50 * s.db[594][13]);
        let eq10_e360_d_b14: f64 = (p.p50 * s.db[594][14]);
        let eq10_e360_q: f64 = (p.p50 * eq10_e359_q);
        let eq10_e360_q_d_n0: f64 = (p.p50 * s.dn[594][0]);
        let eq10_e360_q_d_n1: f64 = (p.p50 * s.dn[594][1]);
        let eq10_e360_q_d_n2: f64 = (p.p50 * s.dn[594][2]);
        let eq10_e360_q_d_n3: f64 = (p.p50 * s.dn[594][3]);
        let eq10_e360_q_d_n4: f64 = (p.p50 * s.dn[594][4]);
        let eq10_e360_q_d_n5: f64 = (p.p50 * s.dn[594][5]);
        let eq10_e360_q_d_n6: f64 = (p.p50 * s.dn[594][6]);
        let eq10_e360_q_d_n7: f64 = (p.p50 * s.dn[594][7]);
        let eq10_e360_q_d_n8: f64 = (p.p50 * s.dn[594][8]);
        let eq10_e360_q_d_n9: f64 = (p.p50 * s.dn[594][9]);
        let eq10_e360_q_d_n10: f64 = (p.p50 * s.dn[594][10]);
        let eq10_e360_q_d_n11: f64 = (p.p50 * s.dn[594][11]);
        let eq10_e360_q_d_n12: f64 = (p.p50 * s.dn[594][12]);
        let eq10_e360_q_d_n13: f64 = (p.p50 * s.dn[594][13]);
        let eq10_e360_q_d_n14: f64 = (p.p50 * s.dn[594][14]);
        let eq10_e360_q_d_n15: f64 = (p.p50 * s.dn[594][15]);
        let eq10_e360_q_d_n16: f64 = (p.p50 * s.dn[594][16]);
        let eq10_e360_q_d_n17: f64 = (p.p50 * s.dn[594][17]);
        let eq10_e360_q_d_n18: f64 = (p.p50 * s.dn[594][18]);
        let eq10_e360_q_d_b0: f64 = (p.p50 * s.db[594][0]);
        let eq10_e360_q_d_b1: f64 = (p.p50 * s.db[594][1]);
        let eq10_e360_q_d_b2: f64 = (p.p50 * s.db[594][2]);
        let eq10_e360_q_d_b3: f64 = (p.p50 * s.db[594][3]);
        let eq10_e360_q_d_b4: f64 = (p.p50 * s.db[594][4]);
        let eq10_e360_q_d_b5: f64 = (p.p50 * s.db[594][5]);
        let eq10_e360_q_d_b6: f64 = (p.p50 * s.db[594][6]);
        let eq10_e360_q_d_b7: f64 = (p.p50 * s.db[594][7]);
        let eq10_e360_q_d_b8: f64 = (p.p50 * s.db[594][8]);
        let eq10_e360_q_d_b9: f64 = (p.p50 * s.db[594][9]);
        let eq10_e360_q_d_b10: f64 = (p.p50 * s.db[594][10]);
        let eq10_e360_q_d_b11: f64 = (p.p50 * s.db[594][11]);
        let eq10_e360_q_d_b12: f64 = (p.p50 * s.db[594][12]);
        let eq10_e360_q_d_b13: f64 = (p.p50 * s.db[594][13]);
        let eq10_e360_q_d_b14: f64 = (p.p50 * s.db[594][14]);
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e360_q_d_n0, eq10_e360_q_d_n1, eq10_e360_q_d_n2, eq10_e360_q_d_n3, eq10_e360_q_d_n4, eq10_e360_q_d_n5, eq10_e360_q_d_n6, eq10_e360_q_d_n7, eq10_e360_q_d_n8, eq10_e360_q_d_n9, eq10_e360_q_d_n10, eq10_e360_q_d_n11, eq10_e360_q_d_n12, eq10_e360_q_d_n13, eq10_e360_q_d_n14, eq10_e360_q_d_n15, eq10_e360_q_d_n16, eq10_e360_q_d_n17, eq10_e360_q_d_n18];
        let eq10_reactive_branch_derivatives: [f64; 15] = [eq10_e360_q_d_b0, eq10_e360_q_d_b1, eq10_e360_q_d_b2, eq10_e360_q_d_b3, eq10_e360_q_d_b4, eq10_e360_q_d_b5, eq10_e360_q_d_b6, eq10_e360_q_d_b7, eq10_e360_q_d_b8, eq10_e360_q_d_b9, eq10_e360_q_d_b10, eq10_e360_q_d_b11, eq10_e360_q_d_b12, eq10_e360_q_d_b13, eq10_e360_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e363_q: f64 = s.v[198];
        let eq11_e364: f64 = (p.p50 * s.v[198]);
        let eq11_e364_d_n0: f64 = (p.p50 * s.dn[198][0]);
        let eq11_e364_d_n1: f64 = (p.p50 * s.dn[198][1]);
        let eq11_e364_d_n2: f64 = (p.p50 * s.dn[198][2]);
        let eq11_e364_d_n3: f64 = (p.p50 * s.dn[198][3]);
        let eq11_e364_d_n4: f64 = (p.p50 * s.dn[198][4]);
        let eq11_e364_d_n5: f64 = (p.p50 * s.dn[198][5]);
        let eq11_e364_d_n6: f64 = (p.p50 * s.dn[198][6]);
        let eq11_e364_d_n7: f64 = (p.p50 * s.dn[198][7]);
        let eq11_e364_d_n8: f64 = (p.p50 * s.dn[198][8]);
        let eq11_e364_d_n9: f64 = (p.p50 * s.dn[198][9]);
        let eq11_e364_d_n10: f64 = (p.p50 * s.dn[198][10]);
        let eq11_e364_d_n11: f64 = (p.p50 * s.dn[198][11]);
        let eq11_e364_d_n12: f64 = (p.p50 * s.dn[198][12]);
        let eq11_e364_d_n13: f64 = (p.p50 * s.dn[198][13]);
        let eq11_e364_d_n14: f64 = (p.p50 * s.dn[198][14]);
        let eq11_e364_d_n15: f64 = (p.p50 * s.dn[198][15]);
        let eq11_e364_d_n16: f64 = (p.p50 * s.dn[198][16]);
        let eq11_e364_d_n17: f64 = (p.p50 * s.dn[198][17]);
        let eq11_e364_d_n18: f64 = (p.p50 * s.dn[198][18]);
        let eq11_e364_d_b0: f64 = (p.p50 * s.db[198][0]);
        let eq11_e364_d_b1: f64 = (p.p50 * s.db[198][1]);
        let eq11_e364_d_b2: f64 = (p.p50 * s.db[198][2]);
        let eq11_e364_d_b3: f64 = (p.p50 * s.db[198][3]);
        let eq11_e364_d_b4: f64 = (p.p50 * s.db[198][4]);
        let eq11_e364_d_b5: f64 = (p.p50 * s.db[198][5]);
        let eq11_e364_d_b6: f64 = (p.p50 * s.db[198][6]);
        let eq11_e364_d_b7: f64 = (p.p50 * s.db[198][7]);
        let eq11_e364_d_b8: f64 = (p.p50 * s.db[198][8]);
        let eq11_e364_d_b9: f64 = (p.p50 * s.db[198][9]);
        let eq11_e364_d_b10: f64 = (p.p50 * s.db[198][10]);
        let eq11_e364_d_b11: f64 = (p.p50 * s.db[198][11]);
        let eq11_e364_d_b12: f64 = (p.p50 * s.db[198][12]);
        let eq11_e364_d_b13: f64 = (p.p50 * s.db[198][13]);
        let eq11_e364_d_b14: f64 = (p.p50 * s.db[198][14]);
        let eq11_e364_q: f64 = (p.p50 * eq11_e363_q);
        let eq11_e364_q_d_n0: f64 = (p.p50 * s.dn[198][0]);
        let eq11_e364_q_d_n1: f64 = (p.p50 * s.dn[198][1]);
        let eq11_e364_q_d_n2: f64 = (p.p50 * s.dn[198][2]);
        let eq11_e364_q_d_n3: f64 = (p.p50 * s.dn[198][3]);
        let eq11_e364_q_d_n4: f64 = (p.p50 * s.dn[198][4]);
        let eq11_e364_q_d_n5: f64 = (p.p50 * s.dn[198][5]);
        let eq11_e364_q_d_n6: f64 = (p.p50 * s.dn[198][6]);
        let eq11_e364_q_d_n7: f64 = (p.p50 * s.dn[198][7]);
        let eq11_e364_q_d_n8: f64 = (p.p50 * s.dn[198][8]);
        let eq11_e364_q_d_n9: f64 = (p.p50 * s.dn[198][9]);
        let eq11_e364_q_d_n10: f64 = (p.p50 * s.dn[198][10]);
        let eq11_e364_q_d_n11: f64 = (p.p50 * s.dn[198][11]);
        let eq11_e364_q_d_n12: f64 = (p.p50 * s.dn[198][12]);
        let eq11_e364_q_d_n13: f64 = (p.p50 * s.dn[198][13]);
        let eq11_e364_q_d_n14: f64 = (p.p50 * s.dn[198][14]);
        let eq11_e364_q_d_n15: f64 = (p.p50 * s.dn[198][15]);
        let eq11_e364_q_d_n16: f64 = (p.p50 * s.dn[198][16]);
        let eq11_e364_q_d_n17: f64 = (p.p50 * s.dn[198][17]);
        let eq11_e364_q_d_n18: f64 = (p.p50 * s.dn[198][18]);
        let eq11_e364_q_d_b0: f64 = (p.p50 * s.db[198][0]);
        let eq11_e364_q_d_b1: f64 = (p.p50 * s.db[198][1]);
        let eq11_e364_q_d_b2: f64 = (p.p50 * s.db[198][2]);
        let eq11_e364_q_d_b3: f64 = (p.p50 * s.db[198][3]);
        let eq11_e364_q_d_b4: f64 = (p.p50 * s.db[198][4]);
        let eq11_e364_q_d_b5: f64 = (p.p50 * s.db[198][5]);
        let eq11_e364_q_d_b6: f64 = (p.p50 * s.db[198][6]);
        let eq11_e364_q_d_b7: f64 = (p.p50 * s.db[198][7]);
        let eq11_e364_q_d_b8: f64 = (p.p50 * s.db[198][8]);
        let eq11_e364_q_d_b9: f64 = (p.p50 * s.db[198][9]);
        let eq11_e364_q_d_b10: f64 = (p.p50 * s.db[198][10]);
        let eq11_e364_q_d_b11: f64 = (p.p50 * s.db[198][11]);
        let eq11_e364_q_d_b12: f64 = (p.p50 * s.db[198][12]);
        let eq11_e364_q_d_b13: f64 = (p.p50 * s.db[198][13]);
        let eq11_e364_q_d_b14: f64 = (p.p50 * s.db[198][14]);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e364_q_d_n0, eq11_e364_q_d_n1, eq11_e364_q_d_n2, eq11_e364_q_d_n3, eq11_e364_q_d_n4, eq11_e364_q_d_n5, eq11_e364_q_d_n6, eq11_e364_q_d_n7, eq11_e364_q_d_n8, eq11_e364_q_d_n9, eq11_e364_q_d_n10, eq11_e364_q_d_n11, eq11_e364_q_d_n12, eq11_e364_q_d_n13, eq11_e364_q_d_n14, eq11_e364_q_d_n15, eq11_e364_q_d_n16, eq11_e364_q_d_n17, eq11_e364_q_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 15] = [eq11_e364_q_d_b0, eq11_e364_q_d_b1, eq11_e364_q_d_b2, eq11_e364_q_d_b3, eq11_e364_q_d_b4, eq11_e364_q_d_b5, eq11_e364_q_d_b6, eq11_e364_q_d_b7, eq11_e364_q_d_b8, eq11_e364_q_d_b9, eq11_e364_q_d_b10, eq11_e364_q_d_b11, eq11_e364_q_d_b12, eq11_e364_q_d_b13, eq11_e364_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e367_q: f64 = s.v[196];
        let eq12_e368: f64 = (p.p50 * s.v[196]);
        let eq12_e368_d_n0: f64 = (p.p50 * s.dn[196][0]);
        let eq12_e368_d_n1: f64 = (p.p50 * s.dn[196][1]);
        let eq12_e368_d_n2: f64 = (p.p50 * s.dn[196][2]);
        let eq12_e368_d_n3: f64 = (p.p50 * s.dn[196][3]);
        let eq12_e368_d_n4: f64 = (p.p50 * s.dn[196][4]);
        let eq12_e368_d_n5: f64 = (p.p50 * s.dn[196][5]);
        let eq12_e368_d_n6: f64 = (p.p50 * s.dn[196][6]);
        let eq12_e368_d_n7: f64 = (p.p50 * s.dn[196][7]);
        let eq12_e368_d_n8: f64 = (p.p50 * s.dn[196][8]);
        let eq12_e368_d_n9: f64 = (p.p50 * s.dn[196][9]);
        let eq12_e368_d_n10: f64 = (p.p50 * s.dn[196][10]);
        let eq12_e368_d_n11: f64 = (p.p50 * s.dn[196][11]);
        let eq12_e368_d_n12: f64 = (p.p50 * s.dn[196][12]);
        let eq12_e368_d_n13: f64 = (p.p50 * s.dn[196][13]);
        let eq12_e368_d_n14: f64 = (p.p50 * s.dn[196][14]);
        let eq12_e368_d_n15: f64 = (p.p50 * s.dn[196][15]);
        let eq12_e368_d_n16: f64 = (p.p50 * s.dn[196][16]);
        let eq12_e368_d_n17: f64 = (p.p50 * s.dn[196][17]);
        let eq12_e368_d_n18: f64 = (p.p50 * s.dn[196][18]);
        let eq12_e368_d_b0: f64 = (p.p50 * s.db[196][0]);
        let eq12_e368_d_b1: f64 = (p.p50 * s.db[196][1]);
        let eq12_e368_d_b2: f64 = (p.p50 * s.db[196][2]);
        let eq12_e368_d_b3: f64 = (p.p50 * s.db[196][3]);
        let eq12_e368_d_b4: f64 = (p.p50 * s.db[196][4]);
        let eq12_e368_d_b5: f64 = (p.p50 * s.db[196][5]);
        let eq12_e368_d_b6: f64 = (p.p50 * s.db[196][6]);
        let eq12_e368_d_b7: f64 = (p.p50 * s.db[196][7]);
        let eq12_e368_d_b8: f64 = (p.p50 * s.db[196][8]);
        let eq12_e368_d_b9: f64 = (p.p50 * s.db[196][9]);
        let eq12_e368_d_b10: f64 = (p.p50 * s.db[196][10]);
        let eq12_e368_d_b11: f64 = (p.p50 * s.db[196][11]);
        let eq12_e368_d_b12: f64 = (p.p50 * s.db[196][12]);
        let eq12_e368_d_b13: f64 = (p.p50 * s.db[196][13]);
        let eq12_e368_d_b14: f64 = (p.p50 * s.db[196][14]);
        let eq12_e368_q: f64 = (p.p50 * eq12_e367_q);
        let eq12_e368_q_d_n0: f64 = (p.p50 * s.dn[196][0]);
        let eq12_e368_q_d_n1: f64 = (p.p50 * s.dn[196][1]);
        let eq12_e368_q_d_n2: f64 = (p.p50 * s.dn[196][2]);
        let eq12_e368_q_d_n3: f64 = (p.p50 * s.dn[196][3]);
        let eq12_e368_q_d_n4: f64 = (p.p50 * s.dn[196][4]);
        let eq12_e368_q_d_n5: f64 = (p.p50 * s.dn[196][5]);
        let eq12_e368_q_d_n6: f64 = (p.p50 * s.dn[196][6]);
        let eq12_e368_q_d_n7: f64 = (p.p50 * s.dn[196][7]);
        let eq12_e368_q_d_n8: f64 = (p.p50 * s.dn[196][8]);
        let eq12_e368_q_d_n9: f64 = (p.p50 * s.dn[196][9]);
        let eq12_e368_q_d_n10: f64 = (p.p50 * s.dn[196][10]);
        let eq12_e368_q_d_n11: f64 = (p.p50 * s.dn[196][11]);
        let eq12_e368_q_d_n12: f64 = (p.p50 * s.dn[196][12]);
        let eq12_e368_q_d_n13: f64 = (p.p50 * s.dn[196][13]);
        let eq12_e368_q_d_n14: f64 = (p.p50 * s.dn[196][14]);
        let eq12_e368_q_d_n15: f64 = (p.p50 * s.dn[196][15]);
        let eq12_e368_q_d_n16: f64 = (p.p50 * s.dn[196][16]);
        let eq12_e368_q_d_n17: f64 = (p.p50 * s.dn[196][17]);
        let eq12_e368_q_d_n18: f64 = (p.p50 * s.dn[196][18]);
        let eq12_e368_q_d_b0: f64 = (p.p50 * s.db[196][0]);
        let eq12_e368_q_d_b1: f64 = (p.p50 * s.db[196][1]);
        let eq12_e368_q_d_b2: f64 = (p.p50 * s.db[196][2]);
        let eq12_e368_q_d_b3: f64 = (p.p50 * s.db[196][3]);
        let eq12_e368_q_d_b4: f64 = (p.p50 * s.db[196][4]);
        let eq12_e368_q_d_b5: f64 = (p.p50 * s.db[196][5]);
        let eq12_e368_q_d_b6: f64 = (p.p50 * s.db[196][6]);
        let eq12_e368_q_d_b7: f64 = (p.p50 * s.db[196][7]);
        let eq12_e368_q_d_b8: f64 = (p.p50 * s.db[196][8]);
        let eq12_e368_q_d_b9: f64 = (p.p50 * s.db[196][9]);
        let eq12_e368_q_d_b10: f64 = (p.p50 * s.db[196][10]);
        let eq12_e368_q_d_b11: f64 = (p.p50 * s.db[196][11]);
        let eq12_e368_q_d_b12: f64 = (p.p50 * s.db[196][12]);
        let eq12_e368_q_d_b13: f64 = (p.p50 * s.db[196][13]);
        let eq12_e368_q_d_b14: f64 = (p.p50 * s.db[196][14]);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e368_q_d_n0, eq12_e368_q_d_n1, eq12_e368_q_d_n2, eq12_e368_q_d_n3, eq12_e368_q_d_n4, eq12_e368_q_d_n5, eq12_e368_q_d_n6, eq12_e368_q_d_n7, eq12_e368_q_d_n8, eq12_e368_q_d_n9, eq12_e368_q_d_n10, eq12_e368_q_d_n11, eq12_e368_q_d_n12, eq12_e368_q_d_n13, eq12_e368_q_d_n14, eq12_e368_q_d_n15, eq12_e368_q_d_n16, eq12_e368_q_d_n17, eq12_e368_q_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 15] = [eq12_e368_q_d_b0, eq12_e368_q_d_b1, eq12_e368_q_d_b2, eq12_e368_q_d_b3, eq12_e368_q_d_b4, eq12_e368_q_d_b5, eq12_e368_q_d_b6, eq12_e368_q_d_b7, eq12_e368_q_d_b8, eq12_e368_q_d_b9, eq12_e368_q_d_b10, eq12_e368_q_d_b11, eq12_e368_q_d_b12, eq12_e368_q_d_b13, eq12_e368_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
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
        let (eq30_e483, eq30_e483_d_n0, eq30_e483_d_n1, eq30_e483_d_n2, eq30_e483_d_n3, eq30_e483_d_n4, eq30_e483_d_n5, eq30_e483_d_n6, eq30_e483_d_n7, eq30_e483_d_n8, eq30_e483_d_n9, eq30_e483_d_n10, eq30_e483_d_n11, eq30_e483_d_n12, eq30_e483_d_n13, eq30_e483_d_n14, eq30_e483_d_n15, eq30_e483_d_n16, eq30_e483_d_n17, eq30_e483_d_n18, eq30_e483_d_b0, eq30_e483_d_b1, eq30_e483_d_b2, eq30_e483_d_b3, eq30_e483_d_b4, eq30_e483_d_b5, eq30_e483_d_b6, eq30_e483_d_b7, eq30_e483_d_b8, eq30_e483_d_b9, eq30_e483_d_b10, eq30_e483_d_b11, eq30_e483_d_b12, eq30_e483_d_b13, eq30_e483_d_b14, eq30_e483_q, eq30_e483_q_d_n0, eq30_e483_q_d_n1, eq30_e483_q_d_n2, eq30_e483_q_d_n3, eq30_e483_q_d_n4, eq30_e483_q_d_n5, eq30_e483_q_d_n6, eq30_e483_q_d_n7, eq30_e483_q_d_n8, eq30_e483_q_d_n9, eq30_e483_q_d_n10, eq30_e483_q_d_n11, eq30_e483_q_d_n12, eq30_e483_q_d_n13, eq30_e483_q_d_n14, eq30_e483_q_d_n15, eq30_e483_q_d_n16, eq30_e483_q_d_n17, eq30_e483_q_d_n18, eq30_e483_q_d_b0, eq30_e483_q_d_b1, eq30_e483_q_d_b2, eq30_e483_q_d_b3, eq30_e483_q_d_b4, eq30_e483_q_d_b5, eq30_e483_q_d_b6, eq30_e483_q_d_b7, eq30_e483_q_d_b8, eq30_e483_q_d_b9, eq30_e483_q_d_b10, eq30_e483_q_d_b11, eq30_e483_q_d_b12, eq30_e483_q_d_b13, eq30_e483_q_d_b14,) = {
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
        (eq30_e480, eq30_e480_d_n0, eq30_e480_d_n1, eq30_e480_d_n2, eq30_e480_d_n3, eq30_e480_d_n4, eq30_e480_d_n5, eq30_e480_d_n6, eq30_e480_d_n7, eq30_e480_d_n8, eq30_e480_d_n9, eq30_e480_d_n10, eq30_e480_d_n11, eq30_e480_d_n12, eq30_e480_d_n13, eq30_e480_d_n14, eq30_e480_d_n15, eq30_e480_d_n16, eq30_e480_d_n17, eq30_e480_d_n18, eq30_e480_d_b0, eq30_e480_d_b1, eq30_e480_d_b2, eq30_e480_d_b3, eq30_e480_d_b4, eq30_e480_d_b5, eq30_e480_d_b6, eq30_e480_d_b7, eq30_e480_d_b8, eq30_e480_d_b9, eq30_e480_d_b10, eq30_e480_d_b11, eq30_e480_d_b12, eq30_e480_d_b13, eq30_e480_d_b14, eq30_e481_q, eq30_e480_d_n0, eq30_e480_d_n1, eq30_e480_d_n2, eq30_e480_d_n3, eq30_e480_d_n4, eq30_e480_d_n5, eq30_e480_d_n6, eq30_e480_d_n7, eq30_e480_d_n8, eq30_e480_d_n9, eq30_e480_d_n10, eq30_e480_d_n11, eq30_e480_d_n12, eq30_e480_d_n13, eq30_e480_d_n14, eq30_e480_d_n15, eq30_e480_d_n16, eq30_e480_d_n17, eq30_e480_d_n18, eq30_e480_d_b0, eq30_e480_d_b1, eq30_e480_d_b2, eq30_e480_d_b3, eq30_e480_d_b4, eq30_e480_d_b5, eq30_e480_d_b6, eq30_e480_d_b7, eq30_e480_d_b8, eq30_e480_d_b9, eq30_e480_d_b10, eq30_e480_d_b11, eq30_e480_d_b12, eq30_e480_d_b13, eq30_e480_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_reactive_node_derivatives: [f64; 19] = [eq30_e483_q_d_n0, eq30_e483_q_d_n1, eq30_e483_q_d_n2, eq30_e483_q_d_n3, eq30_e483_q_d_n4, eq30_e483_q_d_n5, eq30_e483_q_d_n6, eq30_e483_q_d_n7, eq30_e483_q_d_n8, eq30_e483_q_d_n9, eq30_e483_q_d_n10, eq30_e483_q_d_n11, eq30_e483_q_d_n12, eq30_e483_q_d_n13, eq30_e483_q_d_n14, eq30_e483_q_d_n15, eq30_e483_q_d_n16, eq30_e483_q_d_n17, eq30_e483_q_d_n18];
        let eq30_reactive_branch_derivatives: [f64; 15] = [eq30_e483_q_d_b0, eq30_e483_q_d_b1, eq30_e483_q_d_b2, eq30_e483_q_d_b3, eq30_e483_q_d_b4, eq30_e483_q_d_b5, eq30_e483_q_d_b6, eq30_e483_q_d_b7, eq30_e483_q_d_b8, eq30_e483_q_d_b9, eq30_e483_q_d_b10, eq30_e483_q_d_b11, eq30_e483_q_d_b12, eq30_e483_q_d_b13, eq30_e483_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            None,
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
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
        let eq34_e513_q_d_n0: f64 = (p.p50 * s.dn[283][0]);
        let eq34_e513_q_d_n1: f64 = (p.p50 * s.dn[283][1]);
        let eq34_e513_q_d_n2: f64 = (p.p50 * s.dn[283][2]);
        let eq34_e513_q_d_n3: f64 = (p.p50 * s.dn[283][3]);
        let eq34_e513_q_d_n4: f64 = (p.p50 * s.dn[283][4]);
        let eq34_e513_q_d_n5: f64 = (p.p50 * s.dn[283][5]);
        let eq34_e513_q_d_n6: f64 = (p.p50 * s.dn[283][6]);
        let eq34_e513_q_d_n7: f64 = (p.p50 * s.dn[283][7]);
        let eq34_e513_q_d_n8: f64 = (p.p50 * s.dn[283][8]);
        let eq34_e513_q_d_n9: f64 = (p.p50 * s.dn[283][9]);
        let eq34_e513_q_d_n10: f64 = (p.p50 * s.dn[283][10]);
        let eq34_e513_q_d_n11: f64 = (p.p50 * s.dn[283][11]);
        let eq34_e513_q_d_n12: f64 = (p.p50 * s.dn[283][12]);
        let eq34_e513_q_d_n13: f64 = (p.p50 * s.dn[283][13]);
        let eq34_e513_q_d_n14: f64 = (p.p50 * s.dn[283][14]);
        let eq34_e513_q_d_n15: f64 = (p.p50 * s.dn[283][15]);
        let eq34_e513_q_d_n16: f64 = (p.p50 * s.dn[283][16]);
        let eq34_e513_q_d_n17: f64 = (p.p50 * s.dn[283][17]);
        let eq34_e513_q_d_n18: f64 = (p.p50 * s.dn[283][18]);
        let eq34_e513_q_d_b0: f64 = (p.p50 * s.db[283][0]);
        let eq34_e513_q_d_b1: f64 = (p.p50 * s.db[283][1]);
        let eq34_e513_q_d_b2: f64 = (p.p50 * s.db[283][2]);
        let eq34_e513_q_d_b3: f64 = (p.p50 * s.db[283][3]);
        let eq34_e513_q_d_b4: f64 = (p.p50 * s.db[283][4]);
        let eq34_e513_q_d_b5: f64 = (p.p50 * s.db[283][5]);
        let eq34_e513_q_d_b6: f64 = (p.p50 * s.db[283][6]);
        let eq34_e513_q_d_b7: f64 = (p.p50 * s.db[283][7]);
        let eq34_e513_q_d_b8: f64 = (p.p50 * s.db[283][8]);
        let eq34_e513_q_d_b9: f64 = (p.p50 * s.db[283][9]);
        let eq34_e513_q_d_b10: f64 = (p.p50 * s.db[283][10]);
        let eq34_e513_q_d_b11: f64 = (p.p50 * s.db[283][11]);
        let eq34_e513_q_d_b12: f64 = (p.p50 * s.db[283][12]);
        let eq34_e513_q_d_b13: f64 = (p.p50 * s.db[283][13]);
        let eq34_e513_q_d_b14: f64 = (p.p50 * s.db[283][14]);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n1, eq34_e513_d_n2, eq34_e513_d_n3, eq34_e513_d_n4, eq34_e513_d_n5, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n8, eq34_e513_d_n9, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n13, eq34_e513_d_n14, eq34_e513_d_n15, eq34_e513_d_n16, eq34_e513_d_n17, eq34_e513_d_n18, eq34_e513_d_b0, eq34_e513_d_b1, eq34_e513_d_b2, eq34_e513_d_b3, eq34_e513_d_b4, eq34_e513_d_b5, eq34_e513_d_b6, eq34_e513_d_b7, eq34_e513_d_b8, eq34_e513_d_b9, eq34_e513_d_b10, eq34_e513_d_b11, eq34_e513_d_b12, eq34_e513_d_b13, eq34_e513_d_b14, eq34_e513_q, eq34_e513_q_d_n0, eq34_e513_q_d_n1, eq34_e513_q_d_n2, eq34_e513_q_d_n3, eq34_e513_q_d_n4, eq34_e513_q_d_n5, eq34_e513_q_d_n6, eq34_e513_q_d_n7, eq34_e513_q_d_n8, eq34_e513_q_d_n9, eq34_e513_q_d_n10, eq34_e513_q_d_n11, eq34_e513_q_d_n12, eq34_e513_q_d_n13, eq34_e513_q_d_n14, eq34_e513_q_d_n15, eq34_e513_q_d_n16, eq34_e513_q_d_n17, eq34_e513_q_d_n18, eq34_e513_q_d_b0, eq34_e513_q_d_b1, eq34_e513_q_d_b2, eq34_e513_q_d_b3, eq34_e513_q_d_b4, eq34_e513_q_d_b5, eq34_e513_q_d_b6, eq34_e513_q_d_b7, eq34_e513_q_d_b8, eq34_e513_q_d_b9, eq34_e513_q_d_b10, eq34_e513_q_d_b11, eq34_e513_q_d_b12, eq34_e513_q_d_b13, eq34_e513_q_d_b14,)
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
        let eq35_e522_q_d_n0: f64 = (p.p50 * s.dn[284][0]);
        let eq35_e522_q_d_n1: f64 = (p.p50 * s.dn[284][1]);
        let eq35_e522_q_d_n2: f64 = (p.p50 * s.dn[284][2]);
        let eq35_e522_q_d_n3: f64 = (p.p50 * s.dn[284][3]);
        let eq35_e522_q_d_n4: f64 = (p.p50 * s.dn[284][4]);
        let eq35_e522_q_d_n5: f64 = (p.p50 * s.dn[284][5]);
        let eq35_e522_q_d_n6: f64 = (p.p50 * s.dn[284][6]);
        let eq35_e522_q_d_n7: f64 = (p.p50 * s.dn[284][7]);
        let eq35_e522_q_d_n8: f64 = (p.p50 * s.dn[284][8]);
        let eq35_e522_q_d_n9: f64 = (p.p50 * s.dn[284][9]);
        let eq35_e522_q_d_n10: f64 = (p.p50 * s.dn[284][10]);
        let eq35_e522_q_d_n11: f64 = (p.p50 * s.dn[284][11]);
        let eq35_e522_q_d_n12: f64 = (p.p50 * s.dn[284][12]);
        let eq35_e522_q_d_n13: f64 = (p.p50 * s.dn[284][13]);
        let eq35_e522_q_d_n14: f64 = (p.p50 * s.dn[284][14]);
        let eq35_e522_q_d_n15: f64 = (p.p50 * s.dn[284][15]);
        let eq35_e522_q_d_n16: f64 = (p.p50 * s.dn[284][16]);
        let eq35_e522_q_d_n17: f64 = (p.p50 * s.dn[284][17]);
        let eq35_e522_q_d_n18: f64 = (p.p50 * s.dn[284][18]);
        let eq35_e522_q_d_b0: f64 = (p.p50 * s.db[284][0]);
        let eq35_e522_q_d_b1: f64 = (p.p50 * s.db[284][1]);
        let eq35_e522_q_d_b2: f64 = (p.p50 * s.db[284][2]);
        let eq35_e522_q_d_b3: f64 = (p.p50 * s.db[284][3]);
        let eq35_e522_q_d_b4: f64 = (p.p50 * s.db[284][4]);
        let eq35_e522_q_d_b5: f64 = (p.p50 * s.db[284][5]);
        let eq35_e522_q_d_b6: f64 = (p.p50 * s.db[284][6]);
        let eq35_e522_q_d_b7: f64 = (p.p50 * s.db[284][7]);
        let eq35_e522_q_d_b8: f64 = (p.p50 * s.db[284][8]);
        let eq35_e522_q_d_b9: f64 = (p.p50 * s.db[284][9]);
        let eq35_e522_q_d_b10: f64 = (p.p50 * s.db[284][10]);
        let eq35_e522_q_d_b11: f64 = (p.p50 * s.db[284][11]);
        let eq35_e522_q_d_b12: f64 = (p.p50 * s.db[284][12]);
        let eq35_e522_q_d_b13: f64 = (p.p50 * s.db[284][13]);
        let eq35_e522_q_d_b14: f64 = (p.p50 * s.db[284][14]);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n1, eq35_e522_d_n2, eq35_e522_d_n3, eq35_e522_d_n4, eq35_e522_d_n5, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n8, eq35_e522_d_n9, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n13, eq35_e522_d_n14, eq35_e522_d_n15, eq35_e522_d_n16, eq35_e522_d_n17, eq35_e522_d_n18, eq35_e522_d_b0, eq35_e522_d_b1, eq35_e522_d_b2, eq35_e522_d_b3, eq35_e522_d_b4, eq35_e522_d_b5, eq35_e522_d_b6, eq35_e522_d_b7, eq35_e522_d_b8, eq35_e522_d_b9, eq35_e522_d_b10, eq35_e522_d_b11, eq35_e522_d_b12, eq35_e522_d_b13, eq35_e522_d_b14, eq35_e522_q, eq35_e522_q_d_n0, eq35_e522_q_d_n1, eq35_e522_q_d_n2, eq35_e522_q_d_n3, eq35_e522_q_d_n4, eq35_e522_q_d_n5, eq35_e522_q_d_n6, eq35_e522_q_d_n7, eq35_e522_q_d_n8, eq35_e522_q_d_n9, eq35_e522_q_d_n10, eq35_e522_q_d_n11, eq35_e522_q_d_n12, eq35_e522_q_d_n13, eq35_e522_q_d_n14, eq35_e522_q_d_n15, eq35_e522_q_d_n16, eq35_e522_q_d_n17, eq35_e522_q_d_n18, eq35_e522_q_d_b0, eq35_e522_q_d_b1, eq35_e522_q_d_b2, eq35_e522_q_d_b3, eq35_e522_q_d_b4, eq35_e522_q_d_b5, eq35_e522_q_d_b6, eq35_e522_q_d_b7, eq35_e522_q_d_b8, eq35_e522_q_d_b9, eq35_e522_q_d_b10, eq35_e522_q_d_b11, eq35_e522_q_d_b12, eq35_e522_q_d_b13, eq35_e522_q_d_b14,)
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
        let (eq46_e608, eq46_e608_d_n18, eq46_e608_q, eq46_e608_q_d_n18,) = {
    if (s.b[1849] && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606_q: f64 = eq46_e605;
        (eq46_e605, eq46_e603, eq46_e606_q, eq46_e603,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq46_e608_q_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13, eq47_e619_q, eq47_e619_q_d_n13,) = {
    if (s.b[1849] && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617_q: f64 = eq47_e616;
        (eq47_e616, eq47_e614, eq47_e617_q, eq47_e614,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq47_e619_q_d_n13),
        );
        let (eq52_e658, eq52_e658_d_n17, eq52_e658_q, eq52_e658_q_d_n17,) = {
    if (s.b[1849] && s.b[1850]) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656_q: f64 = eq52_e655;
        (eq52_e655, eq52_e653, eq52_e656_q, eq52_e653,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq52_e658_q_d_n17),
        );
        let (eq59_e716, eq59_e716_d_n17, eq59_e716_q, eq59_e716_q_d_n17,) = {
    if ((!s.b[1849]) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714_q: f64 = eq59_e713;
        (eq59_e713, eq59_e711, eq59_e714_q, eq59_e711,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq59_e716_q_d_n17),
        );
        let (eq67_e784, eq67_e784_d_n15, eq67_e784_q, eq67_e784_q_d_n15,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782_q: f64 = eq67_e781;
        (eq67_e781, eq67_e779, eq67_e782_q, eq67_e779,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq67_e784_q_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16, eq68_e796_q, eq68_e796_q_d_n16,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794_q: f64 = eq68_e793;
        (eq68_e793, eq68_e791, eq68_e794_q, eq68_e791,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq68_e796_q_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13, eq69_e808_q, eq69_e808_q_d_n13,) = {
    if ((!s.b[1849]) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806_q: f64 = eq69_e805;
        (eq69_e805, eq69_e803, eq69_e806_q, eq69_e803,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq69_e808_q_d_n13),
        );
    }
}
