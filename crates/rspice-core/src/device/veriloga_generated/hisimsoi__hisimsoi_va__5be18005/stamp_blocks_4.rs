#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1833] && (s.v[85] != 0.0)) {
            if (s.v[613] == 1.0) {
                s.copy_ad(438, 556);
            } else {
                s.store_sub_from_scalar(438, 1.0, 556);
            }
        }

        if (s.b[1833] && (s.v[85] != 0.0)) {
            s.store_add_scaled_product_indices(584, 473, 1.0, 580, 438, 1.0);
            s.store_add_ad_lhs(585, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(438)), 473);
            s.store_add_scaled_inputs3(586, s.ad_value(580), -1.0, s.ad_value(581), (-1.0), s.ad_value(471), 1.0);
        }

        if (s.b[1833] && (s.v[85] == 0.0)) {
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(586, 0.0);
            s.store_scalar(581, 0.0);
        }

        if ((!s.b[1833]) && (s.v[85] != 0.0)) {
            s.store_add_scaled_inputs3(586, s.ad_value(584), -1.0, s.ad_value(585), (-1.0), s.ad_value(581), -1.0);
        }

        if ((!s.b[1833]) && (s.v[85] == 0.0)) {
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(586, 0.0);
            s.store_scalar(581, 0.0);
        }

        s.b[1838] = (s.v[613] == 1.0);
        s.v[1838] = if s.b[1838] { 1.0 } else { 0.0 };

        if s.b[1838] {
            s.copy_ad(199, 9);
            s.copy_ad(263, 557);
            s.store_add(594, 23, 586);
            s.store_add(198, 24, 584);
            s.store_neg_ad(554, A::add_scaled_inputs3(s.ad_value(23), 1.0, s.ad_value(24), 1.0, s.ad_value(25), 1.0));
            s.store_add(196, 554, 581);
        }

        if (!s.b[1838]) {
            s.store_neg(199, 9);
            s.store_scalar(263, 0.0);
            s.store_add(594, 23, 586);
            s.store_add(198, 25, 585);
            s.store_neg_ad(554, A::add_scaled_inputs3(s.ad_value(23), 1.0, s.ad_value(24), 1.0, s.ad_value(25), 1.0));
            s.store_add(196, 554, 581);
        }

        s.b[1839] = (p.p43 == 1.0);
        s.v[1839] = if s.b[1839] { 1.0 } else { 0.0 };

        if s.b[1839] {
            s.copy_ad(282, 35);
            s.copy_ad(284, 560);
            s.copy_ad(281, 36);
            s.copy_ad(283, 561);
        }

        s.b[1840] = ((p.p38 == 1.0) && (s.v[67] > 0.0));
        s.v[1840] = if s.b[1840] { 1.0 } else { 0.0 };

        if s.b[1840] {
            s.copy_ad(563, 542);
        }

        if (!s.b[1840]) {
            s.store_scalar(563, 0.0);
        }

        s.copy_ad(9, 199);

        s.store_scalar(27, A::ddx_projection(&s.ad_value(594), Some(6), None));

        s.store_scale(27, 27, p.p50);

        s.store_scalar(28, A::ddx_projection(&s.ad_value(594), Some(7), None));

        s.store_scale(28, 28, p.p50);

        s.b[1842] = (p.p43 == 1.0);
        s.v[1842] = if s.b[1842] { 1.0 } else { 0.0 };

        if s.b[1842] {
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

        s.b[1850] = ((p.p38 > 0.0) && (p.p242 > 0.0));
        s.v[1850] = if s.b[1850] { 1.0 } else { 0.0 };

        s.b[1851] = (p.p43 == 1.0);
        s.v[1851] = if s.b[1851] { 1.0 } else { 0.0 };

        s.b[1852] = ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0)));
        s.v[1852] = if s.b[1852] { 1.0 } else { 0.0 };

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
        let (eq0_e312,) = {
    if s.b[625] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e312;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e317,) = {
    if (!s.b[625]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e317;
        stamper.stamp_potential_const_local(
            1,
            eq1_value,
        );
        let (eq2_e321,) = {
    if s.b[629] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e321;
        stamper.stamp_potential_const_local(
            2,
            eq2_value,
        );
        let eq3_e324: f64 = (p.p50 * s.v[199]);
        let eq3_e324_d_n0: f64 = (p.p50 * s.dn[199][0]);
        let eq3_e324_d_n1: f64 = (p.p50 * s.dn[199][1]);
        let eq3_e324_d_n2: f64 = (p.p50 * s.dn[199][2]);
        let eq3_e324_d_n3: f64 = (p.p50 * s.dn[199][3]);
        let eq3_e324_d_n4: f64 = (p.p50 * s.dn[199][4]);
        let eq3_e324_d_n5: f64 = (p.p50 * s.dn[199][5]);
        let eq3_e324_d_n6: f64 = (p.p50 * s.dn[199][6]);
        let eq3_e324_d_n7: f64 = (p.p50 * s.dn[199][7]);
        let eq3_e324_d_n8: f64 = (p.p50 * s.dn[199][8]);
        let eq3_e324_d_n9: f64 = (p.p50 * s.dn[199][9]);
        let eq3_e324_d_n10: f64 = (p.p50 * s.dn[199][10]);
        let eq3_e324_d_n11: f64 = (p.p50 * s.dn[199][11]);
        let eq3_e324_d_n12: f64 = (p.p50 * s.dn[199][12]);
        let eq3_e324_d_n13: f64 = (p.p50 * s.dn[199][13]);
        let eq3_e324_d_n14: f64 = (p.p50 * s.dn[199][14]);
        let eq3_e324_d_n15: f64 = (p.p50 * s.dn[199][15]);
        let eq3_e324_d_n16: f64 = (p.p50 * s.dn[199][16]);
        let eq3_e324_d_n17: f64 = (p.p50 * s.dn[199][17]);
        let eq3_e324_d_n18: f64 = (p.p50 * s.dn[199][18]);
        let eq3_e324_d_b0: f64 = (p.p50 * s.db[199][0]);
        let eq3_e324_d_b1: f64 = (p.p50 * s.db[199][1]);
        let eq3_e324_d_b2: f64 = (p.p50 * s.db[199][2]);
        let eq3_e324_d_b3: f64 = (p.p50 * s.db[199][3]);
        let eq3_e324_d_b4: f64 = (p.p50 * s.db[199][4]);
        let eq3_e324_d_b5: f64 = (p.p50 * s.db[199][5]);
        let eq3_e324_d_b6: f64 = (p.p50 * s.db[199][6]);
        let eq3_e324_d_b7: f64 = (p.p50 * s.db[199][7]);
        let eq3_e324_d_b8: f64 = (p.p50 * s.db[199][8]);
        let eq3_e324_d_b9: f64 = (p.p50 * s.db[199][9]);
        let eq3_e324_d_b10: f64 = (p.p50 * s.db[199][10]);
        let eq3_e324_d_b11: f64 = (p.p50 * s.db[199][11]);
        let eq3_e324_d_b12: f64 = (p.p50 * s.db[199][12]);
        let eq3_e324_d_b13: f64 = (p.p50 * s.db[199][13]);
        let eq3_e324_d_b14: f64 = (p.p50 * s.db[199][14]);
        let eq3_e324_d_b15: f64 = (p.p50 * s.db[199][15]);
        let eq3_value: f64 = eq3_e324;
        let eq3_node_derivatives: [f64; 19] = [eq3_e324_d_n0, eq3_e324_d_n1, eq3_e324_d_n2, eq3_e324_d_n3, eq3_e324_d_n4, eq3_e324_d_n5, eq3_e324_d_n6, eq3_e324_d_n7, eq3_e324_d_n8, eq3_e324_d_n9, eq3_e324_d_n10, eq3_e324_d_n11, eq3_e324_d_n12, eq3_e324_d_n13, eq3_e324_d_n14, eq3_e324_d_n15, eq3_e324_d_n16, eq3_e324_d_n17, eq3_e324_d_n18];
        let eq3_branch_derivatives: [f64; 16] = [eq3_e324_d_b0, eq3_e324_d_b1, eq3_e324_d_b2, eq3_e324_d_b3, eq3_e324_d_b4, eq3_e324_d_b5, eq3_e324_d_b6, eq3_e324_d_b7, eq3_e324_d_b8, eq3_e324_d_b9, eq3_e324_d_b10, eq3_e324_d_b11, eq3_e324_d_b12, eq3_e324_d_b13, eq3_e324_d_b14, eq3_e324_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e330, eq4_e330_d_n0, eq4_e330_d_n1, eq4_e330_d_n2, eq4_e330_d_n3, eq4_e330_d_n4, eq4_e330_d_n5, eq4_e330_d_n6, eq4_e330_d_n7, eq4_e330_d_n8, eq4_e330_d_n9, eq4_e330_d_n10, eq4_e330_d_n11, eq4_e330_d_n12, eq4_e330_d_n13, eq4_e330_d_n14, eq4_e330_d_n15, eq4_e330_d_n16, eq4_e330_d_n17, eq4_e330_d_n18, eq4_e330_d_b0, eq4_e330_d_b1, eq4_e330_d_b2, eq4_e330_d_b3, eq4_e330_d_b4, eq4_e330_d_b5, eq4_e330_d_b6, eq4_e330_d_b7, eq4_e330_d_b8, eq4_e330_d_b9, eq4_e330_d_b10, eq4_e330_d_b11, eq4_e330_d_b12, eq4_e330_d_b13, eq4_e330_d_b14, eq4_e330_d_b15,) = {
    if s.b[1848] {
        let eq4_e328: f64 = (p.p50 * s.v[306]);
        let eq4_e328_d_n0: f64 = (p.p50 * s.dn[306][0]);
        let eq4_e328_d_n1: f64 = (p.p50 * s.dn[306][1]);
        let eq4_e328_d_n2: f64 = (p.p50 * s.dn[306][2]);
        let eq4_e328_d_n3: f64 = (p.p50 * s.dn[306][3]);
        let eq4_e328_d_n4: f64 = (p.p50 * s.dn[306][4]);
        let eq4_e328_d_n5: f64 = (p.p50 * s.dn[306][5]);
        let eq4_e328_d_n6: f64 = (p.p50 * s.dn[306][6]);
        let eq4_e328_d_n7: f64 = (p.p50 * s.dn[306][7]);
        let eq4_e328_d_n8: f64 = (p.p50 * s.dn[306][8]);
        let eq4_e328_d_n9: f64 = (p.p50 * s.dn[306][9]);
        let eq4_e328_d_n10: f64 = (p.p50 * s.dn[306][10]);
        let eq4_e328_d_n11: f64 = (p.p50 * s.dn[306][11]);
        let eq4_e328_d_n12: f64 = (p.p50 * s.dn[306][12]);
        let eq4_e328_d_n13: f64 = (p.p50 * s.dn[306][13]);
        let eq4_e328_d_n14: f64 = (p.p50 * s.dn[306][14]);
        let eq4_e328_d_n15: f64 = (p.p50 * s.dn[306][15]);
        let eq4_e328_d_n16: f64 = (p.p50 * s.dn[306][16]);
        let eq4_e328_d_n17: f64 = (p.p50 * s.dn[306][17]);
        let eq4_e328_d_n18: f64 = (p.p50 * s.dn[306][18]);
        let eq4_e328_d_b0: f64 = (p.p50 * s.db[306][0]);
        let eq4_e328_d_b1: f64 = (p.p50 * s.db[306][1]);
        let eq4_e328_d_b2: f64 = (p.p50 * s.db[306][2]);
        let eq4_e328_d_b3: f64 = (p.p50 * s.db[306][3]);
        let eq4_e328_d_b4: f64 = (p.p50 * s.db[306][4]);
        let eq4_e328_d_b5: f64 = (p.p50 * s.db[306][5]);
        let eq4_e328_d_b6: f64 = (p.p50 * s.db[306][6]);
        let eq4_e328_d_b7: f64 = (p.p50 * s.db[306][7]);
        let eq4_e328_d_b8: f64 = (p.p50 * s.db[306][8]);
        let eq4_e328_d_b9: f64 = (p.p50 * s.db[306][9]);
        let eq4_e328_d_b10: f64 = (p.p50 * s.db[306][10]);
        let eq4_e328_d_b11: f64 = (p.p50 * s.db[306][11]);
        let eq4_e328_d_b12: f64 = (p.p50 * s.db[306][12]);
        let eq4_e328_d_b13: f64 = (p.p50 * s.db[306][13]);
        let eq4_e328_d_b14: f64 = (p.p50 * s.db[306][14]);
        let eq4_e328_d_b15: f64 = (p.p50 * s.db[306][15]);
        (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n1, eq4_e328_d_n2, eq4_e328_d_n3, eq4_e328_d_n4, eq4_e328_d_n5, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n8, eq4_e328_d_n9, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n13, eq4_e328_d_n14, eq4_e328_d_n15, eq4_e328_d_n16, eq4_e328_d_n17, eq4_e328_d_n18, eq4_e328_d_b0, eq4_e328_d_b1, eq4_e328_d_b2, eq4_e328_d_b3, eq4_e328_d_b4, eq4_e328_d_b5, eq4_e328_d_b6, eq4_e328_d_b7, eq4_e328_d_b8, eq4_e328_d_b9, eq4_e328_d_b10, eq4_e328_d_b11, eq4_e328_d_b12, eq4_e328_d_b13, eq4_e328_d_b14, eq4_e328_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e330;
        let eq4_node_derivatives: [f64; 19] = [eq4_e330_d_n0, eq4_e330_d_n1, eq4_e330_d_n2, eq4_e330_d_n3, eq4_e330_d_n4, eq4_e330_d_n5, eq4_e330_d_n6, eq4_e330_d_n7, eq4_e330_d_n8, eq4_e330_d_n9, eq4_e330_d_n10, eq4_e330_d_n11, eq4_e330_d_n12, eq4_e330_d_n13, eq4_e330_d_n14, eq4_e330_d_n15, eq4_e330_d_n16, eq4_e330_d_n17, eq4_e330_d_n18];
        let eq4_branch_derivatives: [f64; 16] = [eq4_e330_d_b0, eq4_e330_d_b1, eq4_e330_d_b2, eq4_e330_d_b3, eq4_e330_d_b4, eq4_e330_d_b5, eq4_e330_d_b6, eq4_e330_d_b7, eq4_e330_d_b8, eq4_e330_d_b9, eq4_e330_d_b10, eq4_e330_d_b11, eq4_e330_d_b12, eq4_e330_d_b13, eq4_e330_d_b14, eq4_e330_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e336, eq5_e336_d_n0, eq5_e336_d_n1, eq5_e336_d_n2, eq5_e336_d_n3, eq5_e336_d_n4, eq5_e336_d_n5, eq5_e336_d_n6, eq5_e336_d_n7, eq5_e336_d_n8, eq5_e336_d_n9, eq5_e336_d_n10, eq5_e336_d_n11, eq5_e336_d_n12, eq5_e336_d_n13, eq5_e336_d_n14, eq5_e336_d_n15, eq5_e336_d_n16, eq5_e336_d_n17, eq5_e336_d_n18, eq5_e336_d_b0, eq5_e336_d_b1, eq5_e336_d_b2, eq5_e336_d_b3, eq5_e336_d_b4, eq5_e336_d_b5, eq5_e336_d_b6, eq5_e336_d_b7, eq5_e336_d_b8, eq5_e336_d_b9, eq5_e336_d_b10, eq5_e336_d_b11, eq5_e336_d_b12, eq5_e336_d_b13, eq5_e336_d_b14, eq5_e336_d_b15,) = {
    if s.b[1848] {
        let eq5_e334: f64 = (p.p50 * s.v[307]);
        let eq5_e334_d_n0: f64 = (p.p50 * s.dn[307][0]);
        let eq5_e334_d_n1: f64 = (p.p50 * s.dn[307][1]);
        let eq5_e334_d_n2: f64 = (p.p50 * s.dn[307][2]);
        let eq5_e334_d_n3: f64 = (p.p50 * s.dn[307][3]);
        let eq5_e334_d_n4: f64 = (p.p50 * s.dn[307][4]);
        let eq5_e334_d_n5: f64 = (p.p50 * s.dn[307][5]);
        let eq5_e334_d_n6: f64 = (p.p50 * s.dn[307][6]);
        let eq5_e334_d_n7: f64 = (p.p50 * s.dn[307][7]);
        let eq5_e334_d_n8: f64 = (p.p50 * s.dn[307][8]);
        let eq5_e334_d_n9: f64 = (p.p50 * s.dn[307][9]);
        let eq5_e334_d_n10: f64 = (p.p50 * s.dn[307][10]);
        let eq5_e334_d_n11: f64 = (p.p50 * s.dn[307][11]);
        let eq5_e334_d_n12: f64 = (p.p50 * s.dn[307][12]);
        let eq5_e334_d_n13: f64 = (p.p50 * s.dn[307][13]);
        let eq5_e334_d_n14: f64 = (p.p50 * s.dn[307][14]);
        let eq5_e334_d_n15: f64 = (p.p50 * s.dn[307][15]);
        let eq5_e334_d_n16: f64 = (p.p50 * s.dn[307][16]);
        let eq5_e334_d_n17: f64 = (p.p50 * s.dn[307][17]);
        let eq5_e334_d_n18: f64 = (p.p50 * s.dn[307][18]);
        let eq5_e334_d_b0: f64 = (p.p50 * s.db[307][0]);
        let eq5_e334_d_b1: f64 = (p.p50 * s.db[307][1]);
        let eq5_e334_d_b2: f64 = (p.p50 * s.db[307][2]);
        let eq5_e334_d_b3: f64 = (p.p50 * s.db[307][3]);
        let eq5_e334_d_b4: f64 = (p.p50 * s.db[307][4]);
        let eq5_e334_d_b5: f64 = (p.p50 * s.db[307][5]);
        let eq5_e334_d_b6: f64 = (p.p50 * s.db[307][6]);
        let eq5_e334_d_b7: f64 = (p.p50 * s.db[307][7]);
        let eq5_e334_d_b8: f64 = (p.p50 * s.db[307][8]);
        let eq5_e334_d_b9: f64 = (p.p50 * s.db[307][9]);
        let eq5_e334_d_b10: f64 = (p.p50 * s.db[307][10]);
        let eq5_e334_d_b11: f64 = (p.p50 * s.db[307][11]);
        let eq5_e334_d_b12: f64 = (p.p50 * s.db[307][12]);
        let eq5_e334_d_b13: f64 = (p.p50 * s.db[307][13]);
        let eq5_e334_d_b14: f64 = (p.p50 * s.db[307][14]);
        let eq5_e334_d_b15: f64 = (p.p50 * s.db[307][15]);
        (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n1, eq5_e334_d_n2, eq5_e334_d_n3, eq5_e334_d_n4, eq5_e334_d_n5, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n8, eq5_e334_d_n9, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n13, eq5_e334_d_n14, eq5_e334_d_n15, eq5_e334_d_n16, eq5_e334_d_n17, eq5_e334_d_n18, eq5_e334_d_b0, eq5_e334_d_b1, eq5_e334_d_b2, eq5_e334_d_b3, eq5_e334_d_b4, eq5_e334_d_b5, eq5_e334_d_b6, eq5_e334_d_b7, eq5_e334_d_b8, eq5_e334_d_b9, eq5_e334_d_b10, eq5_e334_d_b11, eq5_e334_d_b12, eq5_e334_d_b13, eq5_e334_d_b14, eq5_e334_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e336;
        let eq5_node_derivatives: [f64; 19] = [eq5_e336_d_n0, eq5_e336_d_n1, eq5_e336_d_n2, eq5_e336_d_n3, eq5_e336_d_n4, eq5_e336_d_n5, eq5_e336_d_n6, eq5_e336_d_n7, eq5_e336_d_n8, eq5_e336_d_n9, eq5_e336_d_n10, eq5_e336_d_n11, eq5_e336_d_n12, eq5_e336_d_n13, eq5_e336_d_n14, eq5_e336_d_n15, eq5_e336_d_n16, eq5_e336_d_n17, eq5_e336_d_n18];
        let eq5_branch_derivatives: [f64; 16] = [eq5_e336_d_b0, eq5_e336_d_b1, eq5_e336_d_b2, eq5_e336_d_b3, eq5_e336_d_b4, eq5_e336_d_b5, eq5_e336_d_b6, eq5_e336_d_b7, eq5_e336_d_b8, eq5_e336_d_b9, eq5_e336_d_b10, eq5_e336_d_b11, eq5_e336_d_b12, eq5_e336_d_b13, eq5_e336_d_b14, eq5_e336_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e342, eq6_e342_d_n0, eq6_e342_d_n1, eq6_e342_d_n2, eq6_e342_d_n3, eq6_e342_d_n4, eq6_e342_d_n5, eq6_e342_d_n6, eq6_e342_d_n7, eq6_e342_d_n8, eq6_e342_d_n9, eq6_e342_d_n10, eq6_e342_d_n11, eq6_e342_d_n12, eq6_e342_d_n13, eq6_e342_d_n14, eq6_e342_d_n15, eq6_e342_d_n16, eq6_e342_d_n17, eq6_e342_d_n18, eq6_e342_d_b0, eq6_e342_d_b1, eq6_e342_d_b2, eq6_e342_d_b3, eq6_e342_d_b4, eq6_e342_d_b5, eq6_e342_d_b6, eq6_e342_d_b7, eq6_e342_d_b8, eq6_e342_d_b9, eq6_e342_d_b10, eq6_e342_d_b11, eq6_e342_d_b12, eq6_e342_d_b13, eq6_e342_d_b14, eq6_e342_d_b15,) = {
    if s.b[1848] {
        let eq6_e340: f64 = (p.p50 * s.v[308]);
        let eq6_e340_d_n0: f64 = (p.p50 * s.dn[308][0]);
        let eq6_e340_d_n1: f64 = (p.p50 * s.dn[308][1]);
        let eq6_e340_d_n2: f64 = (p.p50 * s.dn[308][2]);
        let eq6_e340_d_n3: f64 = (p.p50 * s.dn[308][3]);
        let eq6_e340_d_n4: f64 = (p.p50 * s.dn[308][4]);
        let eq6_e340_d_n5: f64 = (p.p50 * s.dn[308][5]);
        let eq6_e340_d_n6: f64 = (p.p50 * s.dn[308][6]);
        let eq6_e340_d_n7: f64 = (p.p50 * s.dn[308][7]);
        let eq6_e340_d_n8: f64 = (p.p50 * s.dn[308][8]);
        let eq6_e340_d_n9: f64 = (p.p50 * s.dn[308][9]);
        let eq6_e340_d_n10: f64 = (p.p50 * s.dn[308][10]);
        let eq6_e340_d_n11: f64 = (p.p50 * s.dn[308][11]);
        let eq6_e340_d_n12: f64 = (p.p50 * s.dn[308][12]);
        let eq6_e340_d_n13: f64 = (p.p50 * s.dn[308][13]);
        let eq6_e340_d_n14: f64 = (p.p50 * s.dn[308][14]);
        let eq6_e340_d_n15: f64 = (p.p50 * s.dn[308][15]);
        let eq6_e340_d_n16: f64 = (p.p50 * s.dn[308][16]);
        let eq6_e340_d_n17: f64 = (p.p50 * s.dn[308][17]);
        let eq6_e340_d_n18: f64 = (p.p50 * s.dn[308][18]);
        let eq6_e340_d_b0: f64 = (p.p50 * s.db[308][0]);
        let eq6_e340_d_b1: f64 = (p.p50 * s.db[308][1]);
        let eq6_e340_d_b2: f64 = (p.p50 * s.db[308][2]);
        let eq6_e340_d_b3: f64 = (p.p50 * s.db[308][3]);
        let eq6_e340_d_b4: f64 = (p.p50 * s.db[308][4]);
        let eq6_e340_d_b5: f64 = (p.p50 * s.db[308][5]);
        let eq6_e340_d_b6: f64 = (p.p50 * s.db[308][6]);
        let eq6_e340_d_b7: f64 = (p.p50 * s.db[308][7]);
        let eq6_e340_d_b8: f64 = (p.p50 * s.db[308][8]);
        let eq6_e340_d_b9: f64 = (p.p50 * s.db[308][9]);
        let eq6_e340_d_b10: f64 = (p.p50 * s.db[308][10]);
        let eq6_e340_d_b11: f64 = (p.p50 * s.db[308][11]);
        let eq6_e340_d_b12: f64 = (p.p50 * s.db[308][12]);
        let eq6_e340_d_b13: f64 = (p.p50 * s.db[308][13]);
        let eq6_e340_d_b14: f64 = (p.p50 * s.db[308][14]);
        let eq6_e340_d_b15: f64 = (p.p50 * s.db[308][15]);
        (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n1, eq6_e340_d_n2, eq6_e340_d_n3, eq6_e340_d_n4, eq6_e340_d_n5, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n8, eq6_e340_d_n9, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n13, eq6_e340_d_n14, eq6_e340_d_n15, eq6_e340_d_n16, eq6_e340_d_n17, eq6_e340_d_n18, eq6_e340_d_b0, eq6_e340_d_b1, eq6_e340_d_b2, eq6_e340_d_b3, eq6_e340_d_b4, eq6_e340_d_b5, eq6_e340_d_b6, eq6_e340_d_b7, eq6_e340_d_b8, eq6_e340_d_b9, eq6_e340_d_b10, eq6_e340_d_b11, eq6_e340_d_b12, eq6_e340_d_b13, eq6_e340_d_b14, eq6_e340_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e342;
        let eq6_node_derivatives: [f64; 19] = [eq6_e342_d_n0, eq6_e342_d_n1, eq6_e342_d_n2, eq6_e342_d_n3, eq6_e342_d_n4, eq6_e342_d_n5, eq6_e342_d_n6, eq6_e342_d_n7, eq6_e342_d_n8, eq6_e342_d_n9, eq6_e342_d_n10, eq6_e342_d_n11, eq6_e342_d_n12, eq6_e342_d_n13, eq6_e342_d_n14, eq6_e342_d_n15, eq6_e342_d_n16, eq6_e342_d_n17, eq6_e342_d_n18];
        let eq6_branch_derivatives: [f64; 16] = [eq6_e342_d_b0, eq6_e342_d_b1, eq6_e342_d_b2, eq6_e342_d_b3, eq6_e342_d_b4, eq6_e342_d_b5, eq6_e342_d_b6, eq6_e342_d_b7, eq6_e342_d_b8, eq6_e342_d_b9, eq6_e342_d_b10, eq6_e342_d_b11, eq6_e342_d_b12, eq6_e342_d_b13, eq6_e342_d_b14, eq6_e342_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e348, eq7_e348_d_n0, eq7_e348_d_n1, eq7_e348_d_n2, eq7_e348_d_n3, eq7_e348_d_n4, eq7_e348_d_n5, eq7_e348_d_n6, eq7_e348_d_n7, eq7_e348_d_n8, eq7_e348_d_n9, eq7_e348_d_n10, eq7_e348_d_n11, eq7_e348_d_n12, eq7_e348_d_n13, eq7_e348_d_n14, eq7_e348_d_n15, eq7_e348_d_n16, eq7_e348_d_n17, eq7_e348_d_n18, eq7_e348_d_b0, eq7_e348_d_b1, eq7_e348_d_b2, eq7_e348_d_b3, eq7_e348_d_b4, eq7_e348_d_b5, eq7_e348_d_b6, eq7_e348_d_b7, eq7_e348_d_b8, eq7_e348_d_b9, eq7_e348_d_b10, eq7_e348_d_b11, eq7_e348_d_b12, eq7_e348_d_b13, eq7_e348_d_b14, eq7_e348_d_b15,) = {
    if (p.p259 != 0.0) {
        let eq7_e346: f64 = ((nv7 - nv2) / s.v[1]);
        let eq7_e346_d_n0: f64 = (-(((nv7 - nv2) * s.dn[1][0]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n1: f64 = (-(((nv7 - nv2) * s.dn[1][1]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n2: f64 = (((-s.v[1]) - ((nv7 - nv2) * s.dn[1][2])) / (s.v[1] * s.v[1]));
        let eq7_e346_d_n3: f64 = (-(((nv7 - nv2) * s.dn[1][3]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n4: f64 = (-(((nv7 - nv2) * s.dn[1][4]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n5: f64 = (-(((nv7 - nv2) * s.dn[1][5]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n6: f64 = (-(((nv7 - nv2) * s.dn[1][6]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n7: f64 = ((s.v[1] - ((nv7 - nv2) * s.dn[1][7])) / (s.v[1] * s.v[1]));
        let eq7_e346_d_n8: f64 = (-(((nv7 - nv2) * s.dn[1][8]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n9: f64 = (-(((nv7 - nv2) * s.dn[1][9]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n10: f64 = (-(((nv7 - nv2) * s.dn[1][10]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n11: f64 = (-(((nv7 - nv2) * s.dn[1][11]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n12: f64 = (-(((nv7 - nv2) * s.dn[1][12]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n13: f64 = (-(((nv7 - nv2) * s.dn[1][13]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n14: f64 = (-(((nv7 - nv2) * s.dn[1][14]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n15: f64 = (-(((nv7 - nv2) * s.dn[1][15]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n16: f64 = (-(((nv7 - nv2) * s.dn[1][16]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n17: f64 = (-(((nv7 - nv2) * s.dn[1][17]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n18: f64 = (-(((nv7 - nv2) * s.dn[1][18]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b0: f64 = (-(((nv7 - nv2) * s.db[1][0]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b1: f64 = (-(((nv7 - nv2) * s.db[1][1]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b2: f64 = (-(((nv7 - nv2) * s.db[1][2]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b3: f64 = (-(((nv7 - nv2) * s.db[1][3]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b4: f64 = (-(((nv7 - nv2) * s.db[1][4]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b5: f64 = (-(((nv7 - nv2) * s.db[1][5]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b6: f64 = (-(((nv7 - nv2) * s.db[1][6]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b7: f64 = (-(((nv7 - nv2) * s.db[1][7]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b8: f64 = (-(((nv7 - nv2) * s.db[1][8]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b9: f64 = (-(((nv7 - nv2) * s.db[1][9]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b10: f64 = (-(((nv7 - nv2) * s.db[1][10]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b11: f64 = (-(((nv7 - nv2) * s.db[1][11]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b12: f64 = (-(((nv7 - nv2) * s.db[1][12]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b13: f64 = (-(((nv7 - nv2) * s.db[1][13]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b14: f64 = (-(((nv7 - nv2) * s.db[1][14]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b15: f64 = (-(((nv7 - nv2) * s.db[1][15]) / (s.v[1] * s.v[1])));
        (eq7_e346, eq7_e346_d_n0, eq7_e346_d_n1, eq7_e346_d_n2, eq7_e346_d_n3, eq7_e346_d_n4, eq7_e346_d_n5, eq7_e346_d_n6, eq7_e346_d_n7, eq7_e346_d_n8, eq7_e346_d_n9, eq7_e346_d_n10, eq7_e346_d_n11, eq7_e346_d_n12, eq7_e346_d_n13, eq7_e346_d_n14, eq7_e346_d_n15, eq7_e346_d_n16, eq7_e346_d_n17, eq7_e346_d_n18, eq7_e346_d_b0, eq7_e346_d_b1, eq7_e346_d_b2, eq7_e346_d_b3, eq7_e346_d_b4, eq7_e346_d_b5, eq7_e346_d_b6, eq7_e346_d_b7, eq7_e346_d_b8, eq7_e346_d_b9, eq7_e346_d_b10, eq7_e346_d_b11, eq7_e346_d_b12, eq7_e346_d_b13, eq7_e346_d_b14, eq7_e346_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e348;
        let eq7_node_derivatives: [f64; 19] = [eq7_e348_d_n0, eq7_e348_d_n1, eq7_e348_d_n2, eq7_e348_d_n3, eq7_e348_d_n4, eq7_e348_d_n5, eq7_e348_d_n6, eq7_e348_d_n7, eq7_e348_d_n8, eq7_e348_d_n9, eq7_e348_d_n10, eq7_e348_d_n11, eq7_e348_d_n12, eq7_e348_d_n13, eq7_e348_d_n14, eq7_e348_d_n15, eq7_e348_d_n16, eq7_e348_d_n17, eq7_e348_d_n18];
        let eq7_branch_derivatives: [f64; 16] = [eq7_e348_d_b0, eq7_e348_d_b1, eq7_e348_d_b2, eq7_e348_d_b3, eq7_e348_d_b4, eq7_e348_d_b5, eq7_e348_d_b6, eq7_e348_d_b7, eq7_e348_d_b8, eq7_e348_d_b9, eq7_e348_d_b10, eq7_e348_d_b11, eq7_e348_d_b12, eq7_e348_d_b13, eq7_e348_d_b14, eq7_e348_d_b15];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e353,) = {
    if (p.p259 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e353;
        stamper.stamp_potential_const_local(
            3,
            eq8_value,
        );
        let (eq9_e359, eq9_e359_d_n0, eq9_e359_d_n1, eq9_e359_d_n2, eq9_e359_d_n3, eq9_e359_d_n4, eq9_e359_d_n5, eq9_e359_d_n6, eq9_e359_d_n7, eq9_e359_d_n8, eq9_e359_d_n9, eq9_e359_d_n10, eq9_e359_d_n11, eq9_e359_d_n12, eq9_e359_d_n13, eq9_e359_d_n14, eq9_e359_d_n15, eq9_e359_d_n16, eq9_e359_d_n17, eq9_e359_d_n18, eq9_e359_d_b0, eq9_e359_d_b1, eq9_e359_d_b2, eq9_e359_d_b3, eq9_e359_d_b4, eq9_e359_d_b5, eq9_e359_d_b6, eq9_e359_d_b7, eq9_e359_d_b8, eq9_e359_d_b9, eq9_e359_d_b10, eq9_e359_d_b11, eq9_e359_d_b12, eq9_e359_d_b13, eq9_e359_d_b14, eq9_e359_d_b15,) = {
    if (p.p260 != 0.0) {
        let eq9_e357: f64 = ((nv0 - nv6) / s.v[0]);
        let eq9_e357_d_n0: f64 = ((s.v[0] - ((nv0 - nv6) * s.dn[0][0])) / (s.v[0] * s.v[0]));
        let eq9_e357_d_n1: f64 = (-(((nv0 - nv6) * s.dn[0][1]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n2: f64 = (-(((nv0 - nv6) * s.dn[0][2]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n3: f64 = (-(((nv0 - nv6) * s.dn[0][3]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n4: f64 = (-(((nv0 - nv6) * s.dn[0][4]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n5: f64 = (-(((nv0 - nv6) * s.dn[0][5]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n6: f64 = (((-s.v[0]) - ((nv0 - nv6) * s.dn[0][6])) / (s.v[0] * s.v[0]));
        let eq9_e357_d_n7: f64 = (-(((nv0 - nv6) * s.dn[0][7]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n8: f64 = (-(((nv0 - nv6) * s.dn[0][8]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n9: f64 = (-(((nv0 - nv6) * s.dn[0][9]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n10: f64 = (-(((nv0 - nv6) * s.dn[0][10]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n11: f64 = (-(((nv0 - nv6) * s.dn[0][11]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n12: f64 = (-(((nv0 - nv6) * s.dn[0][12]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n13: f64 = (-(((nv0 - nv6) * s.dn[0][13]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n14: f64 = (-(((nv0 - nv6) * s.dn[0][14]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n15: f64 = (-(((nv0 - nv6) * s.dn[0][15]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n16: f64 = (-(((nv0 - nv6) * s.dn[0][16]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n17: f64 = (-(((nv0 - nv6) * s.dn[0][17]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n18: f64 = (-(((nv0 - nv6) * s.dn[0][18]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b0: f64 = (-(((nv0 - nv6) * s.db[0][0]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b1: f64 = (-(((nv0 - nv6) * s.db[0][1]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b2: f64 = (-(((nv0 - nv6) * s.db[0][2]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b3: f64 = (-(((nv0 - nv6) * s.db[0][3]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b4: f64 = (-(((nv0 - nv6) * s.db[0][4]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b5: f64 = (-(((nv0 - nv6) * s.db[0][5]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b6: f64 = (-(((nv0 - nv6) * s.db[0][6]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b7: f64 = (-(((nv0 - nv6) * s.db[0][7]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b8: f64 = (-(((nv0 - nv6) * s.db[0][8]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b9: f64 = (-(((nv0 - nv6) * s.db[0][9]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b10: f64 = (-(((nv0 - nv6) * s.db[0][10]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b11: f64 = (-(((nv0 - nv6) * s.db[0][11]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b12: f64 = (-(((nv0 - nv6) * s.db[0][12]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b13: f64 = (-(((nv0 - nv6) * s.db[0][13]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b14: f64 = (-(((nv0 - nv6) * s.db[0][14]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b15: f64 = (-(((nv0 - nv6) * s.db[0][15]) / (s.v[0] * s.v[0])));
        (eq9_e357, eq9_e357_d_n0, eq9_e357_d_n1, eq9_e357_d_n2, eq9_e357_d_n3, eq9_e357_d_n4, eq9_e357_d_n5, eq9_e357_d_n6, eq9_e357_d_n7, eq9_e357_d_n8, eq9_e357_d_n9, eq9_e357_d_n10, eq9_e357_d_n11, eq9_e357_d_n12, eq9_e357_d_n13, eq9_e357_d_n14, eq9_e357_d_n15, eq9_e357_d_n16, eq9_e357_d_n17, eq9_e357_d_n18, eq9_e357_d_b0, eq9_e357_d_b1, eq9_e357_d_b2, eq9_e357_d_b3, eq9_e357_d_b4, eq9_e357_d_b5, eq9_e357_d_b6, eq9_e357_d_b7, eq9_e357_d_b8, eq9_e357_d_b9, eq9_e357_d_b10, eq9_e357_d_b11, eq9_e357_d_b12, eq9_e357_d_b13, eq9_e357_d_b14, eq9_e357_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e359;
        let eq9_node_derivatives: [f64; 19] = [eq9_e359_d_n0, eq9_e359_d_n1, eq9_e359_d_n2, eq9_e359_d_n3, eq9_e359_d_n4, eq9_e359_d_n5, eq9_e359_d_n6, eq9_e359_d_n7, eq9_e359_d_n8, eq9_e359_d_n9, eq9_e359_d_n10, eq9_e359_d_n11, eq9_e359_d_n12, eq9_e359_d_n13, eq9_e359_d_n14, eq9_e359_d_n15, eq9_e359_d_n16, eq9_e359_d_n17, eq9_e359_d_n18];
        let eq9_branch_derivatives: [f64; 16] = [eq9_e359_d_b0, eq9_e359_d_b1, eq9_e359_d_b2, eq9_e359_d_b3, eq9_e359_d_b4, eq9_e359_d_b5, eq9_e359_d_b6, eq9_e359_d_b7, eq9_e359_d_b8, eq9_e359_d_b9, eq9_e359_d_b10, eq9_e359_d_b11, eq9_e359_d_b12, eq9_e359_d_b13, eq9_e359_d_b14, eq9_e359_d_b15];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e364,) = {
    if (p.p260 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e364;
        stamper.stamp_potential_const_local(
            4,
            eq10_value,
        );
        let eq11_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[594]);
        let eq11_e367_d_n0: f64 = (s.dn[594][0] * ddt_scale);
        let eq11_e367_d_n1: f64 = (s.dn[594][1] * ddt_scale);
        let eq11_e367_d_n2: f64 = (s.dn[594][2] * ddt_scale);
        let eq11_e367_d_n3: f64 = (s.dn[594][3] * ddt_scale);
        let eq11_e367_d_n4: f64 = (s.dn[594][4] * ddt_scale);
        let eq11_e367_d_n5: f64 = (s.dn[594][5] * ddt_scale);
        let eq11_e367_d_n6: f64 = (s.dn[594][6] * ddt_scale);
        let eq11_e367_d_n7: f64 = (s.dn[594][7] * ddt_scale);
        let eq11_e367_d_n8: f64 = (s.dn[594][8] * ddt_scale);
        let eq11_e367_d_n9: f64 = (s.dn[594][9] * ddt_scale);
        let eq11_e367_d_n10: f64 = (s.dn[594][10] * ddt_scale);
        let eq11_e367_d_n11: f64 = (s.dn[594][11] * ddt_scale);
        let eq11_e367_d_n12: f64 = (s.dn[594][12] * ddt_scale);
        let eq11_e367_d_n13: f64 = (s.dn[594][13] * ddt_scale);
        let eq11_e367_d_n14: f64 = (s.dn[594][14] * ddt_scale);
        let eq11_e367_d_n15: f64 = (s.dn[594][15] * ddt_scale);
        let eq11_e367_d_n16: f64 = (s.dn[594][16] * ddt_scale);
        let eq11_e367_d_n17: f64 = (s.dn[594][17] * ddt_scale);
        let eq11_e367_d_n18: f64 = (s.dn[594][18] * ddt_scale);
        let eq11_e367_d_b0: f64 = (s.db[594][0] * ddt_scale);
        let eq11_e367_d_b1: f64 = (s.db[594][1] * ddt_scale);
        let eq11_e367_d_b2: f64 = (s.db[594][2] * ddt_scale);
        let eq11_e367_d_b3: f64 = (s.db[594][3] * ddt_scale);
        let eq11_e367_d_b4: f64 = (s.db[594][4] * ddt_scale);
        let eq11_e367_d_b5: f64 = (s.db[594][5] * ddt_scale);
        let eq11_e367_d_b6: f64 = (s.db[594][6] * ddt_scale);
        let eq11_e367_d_b7: f64 = (s.db[594][7] * ddt_scale);
        let eq11_e367_d_b8: f64 = (s.db[594][8] * ddt_scale);
        let eq11_e367_d_b9: f64 = (s.db[594][9] * ddt_scale);
        let eq11_e367_d_b10: f64 = (s.db[594][10] * ddt_scale);
        let eq11_e367_d_b11: f64 = (s.db[594][11] * ddt_scale);
        let eq11_e367_d_b12: f64 = (s.db[594][12] * ddt_scale);
        let eq11_e367_d_b13: f64 = (s.db[594][13] * ddt_scale);
        let eq11_e367_d_b14: f64 = (s.db[594][14] * ddt_scale);
        let eq11_e367_d_b15: f64 = (s.db[594][15] * ddt_scale);
        let eq11_e368: f64 = (p.p50 * eq11_e367);
        let eq11_e368_d_n0: f64 = (p.p50 * eq11_e367_d_n0);
        let eq11_e368_d_n1: f64 = (p.p50 * eq11_e367_d_n1);
        let eq11_e368_d_n2: f64 = (p.p50 * eq11_e367_d_n2);
        let eq11_e368_d_n3: f64 = (p.p50 * eq11_e367_d_n3);
        let eq11_e368_d_n4: f64 = (p.p50 * eq11_e367_d_n4);
        let eq11_e368_d_n5: f64 = (p.p50 * eq11_e367_d_n5);
        let eq11_e368_d_n6: f64 = (p.p50 * eq11_e367_d_n6);
        let eq11_e368_d_n7: f64 = (p.p50 * eq11_e367_d_n7);
        let eq11_e368_d_n8: f64 = (p.p50 * eq11_e367_d_n8);
        let eq11_e368_d_n9: f64 = (p.p50 * eq11_e367_d_n9);
        let eq11_e368_d_n10: f64 = (p.p50 * eq11_e367_d_n10);
        let eq11_e368_d_n11: f64 = (p.p50 * eq11_e367_d_n11);
        let eq11_e368_d_n12: f64 = (p.p50 * eq11_e367_d_n12);
        let eq11_e368_d_n13: f64 = (p.p50 * eq11_e367_d_n13);
        let eq11_e368_d_n14: f64 = (p.p50 * eq11_e367_d_n14);
        let eq11_e368_d_n15: f64 = (p.p50 * eq11_e367_d_n15);
        let eq11_e368_d_n16: f64 = (p.p50 * eq11_e367_d_n16);
        let eq11_e368_d_n17: f64 = (p.p50 * eq11_e367_d_n17);
        let eq11_e368_d_n18: f64 = (p.p50 * eq11_e367_d_n18);
        let eq11_e368_d_b0: f64 = (p.p50 * eq11_e367_d_b0);
        let eq11_e368_d_b1: f64 = (p.p50 * eq11_e367_d_b1);
        let eq11_e368_d_b2: f64 = (p.p50 * eq11_e367_d_b2);
        let eq11_e368_d_b3: f64 = (p.p50 * eq11_e367_d_b3);
        let eq11_e368_d_b4: f64 = (p.p50 * eq11_e367_d_b4);
        let eq11_e368_d_b5: f64 = (p.p50 * eq11_e367_d_b5);
        let eq11_e368_d_b6: f64 = (p.p50 * eq11_e367_d_b6);
        let eq11_e368_d_b7: f64 = (p.p50 * eq11_e367_d_b7);
        let eq11_e368_d_b8: f64 = (p.p50 * eq11_e367_d_b8);
        let eq11_e368_d_b9: f64 = (p.p50 * eq11_e367_d_b9);
        let eq11_e368_d_b10: f64 = (p.p50 * eq11_e367_d_b10);
        let eq11_e368_d_b11: f64 = (p.p50 * eq11_e367_d_b11);
        let eq11_e368_d_b12: f64 = (p.p50 * eq11_e367_d_b12);
        let eq11_e368_d_b13: f64 = (p.p50 * eq11_e367_d_b13);
        let eq11_e368_d_b14: f64 = (p.p50 * eq11_e367_d_b14);
        let eq11_e368_d_b15: f64 = (p.p50 * eq11_e367_d_b15);
        let eq11_value: f64 = eq11_e368;
        let eq11_node_derivatives: [f64; 19] = [eq11_e368_d_n0, eq11_e368_d_n1, eq11_e368_d_n2, eq11_e368_d_n3, eq11_e368_d_n4, eq11_e368_d_n5, eq11_e368_d_n6, eq11_e368_d_n7, eq11_e368_d_n8, eq11_e368_d_n9, eq11_e368_d_n10, eq11_e368_d_n11, eq11_e368_d_n12, eq11_e368_d_n13, eq11_e368_d_n14, eq11_e368_d_n15, eq11_e368_d_n16, eq11_e368_d_n17, eq11_e368_d_n18];
        let eq11_branch_derivatives: [f64; 16] = [eq11_e368_d_b0, eq11_e368_d_b1, eq11_e368_d_b2, eq11_e368_d_b3, eq11_e368_d_b4, eq11_e368_d_b5, eq11_e368_d_b6, eq11_e368_d_b7, eq11_e368_d_b8, eq11_e368_d_b9, eq11_e368_d_b10, eq11_e368_d_b11, eq11_e368_d_b12, eq11_e368_d_b13, eq11_e368_d_b14, eq11_e368_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
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
        let eq12_e371: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[198]);
        let eq12_e371_d_n0: f64 = (s.dn[198][0] * ddt_scale);
        let eq12_e371_d_n1: f64 = (s.dn[198][1] * ddt_scale);
        let eq12_e371_d_n2: f64 = (s.dn[198][2] * ddt_scale);
        let eq12_e371_d_n3: f64 = (s.dn[198][3] * ddt_scale);
        let eq12_e371_d_n4: f64 = (s.dn[198][4] * ddt_scale);
        let eq12_e371_d_n5: f64 = (s.dn[198][5] * ddt_scale);
        let eq12_e371_d_n6: f64 = (s.dn[198][6] * ddt_scale);
        let eq12_e371_d_n7: f64 = (s.dn[198][7] * ddt_scale);
        let eq12_e371_d_n8: f64 = (s.dn[198][8] * ddt_scale);
        let eq12_e371_d_n9: f64 = (s.dn[198][9] * ddt_scale);
        let eq12_e371_d_n10: f64 = (s.dn[198][10] * ddt_scale);
        let eq12_e371_d_n11: f64 = (s.dn[198][11] * ddt_scale);
        let eq12_e371_d_n12: f64 = (s.dn[198][12] * ddt_scale);
        let eq12_e371_d_n13: f64 = (s.dn[198][13] * ddt_scale);
        let eq12_e371_d_n14: f64 = (s.dn[198][14] * ddt_scale);
        let eq12_e371_d_n15: f64 = (s.dn[198][15] * ddt_scale);
        let eq12_e371_d_n16: f64 = (s.dn[198][16] * ddt_scale);
        let eq12_e371_d_n17: f64 = (s.dn[198][17] * ddt_scale);
        let eq12_e371_d_n18: f64 = (s.dn[198][18] * ddt_scale);
        let eq12_e371_d_b0: f64 = (s.db[198][0] * ddt_scale);
        let eq12_e371_d_b1: f64 = (s.db[198][1] * ddt_scale);
        let eq12_e371_d_b2: f64 = (s.db[198][2] * ddt_scale);
        let eq12_e371_d_b3: f64 = (s.db[198][3] * ddt_scale);
        let eq12_e371_d_b4: f64 = (s.db[198][4] * ddt_scale);
        let eq12_e371_d_b5: f64 = (s.db[198][5] * ddt_scale);
        let eq12_e371_d_b6: f64 = (s.db[198][6] * ddt_scale);
        let eq12_e371_d_b7: f64 = (s.db[198][7] * ddt_scale);
        let eq12_e371_d_b8: f64 = (s.db[198][8] * ddt_scale);
        let eq12_e371_d_b9: f64 = (s.db[198][9] * ddt_scale);
        let eq12_e371_d_b10: f64 = (s.db[198][10] * ddt_scale);
        let eq12_e371_d_b11: f64 = (s.db[198][11] * ddt_scale);
        let eq12_e371_d_b12: f64 = (s.db[198][12] * ddt_scale);
        let eq12_e371_d_b13: f64 = (s.db[198][13] * ddt_scale);
        let eq12_e371_d_b14: f64 = (s.db[198][14] * ddt_scale);
        let eq12_e371_d_b15: f64 = (s.db[198][15] * ddt_scale);
        let eq12_e372: f64 = (p.p50 * eq12_e371);
        let eq12_e372_d_n0: f64 = (p.p50 * eq12_e371_d_n0);
        let eq12_e372_d_n1: f64 = (p.p50 * eq12_e371_d_n1);
        let eq12_e372_d_n2: f64 = (p.p50 * eq12_e371_d_n2);
        let eq12_e372_d_n3: f64 = (p.p50 * eq12_e371_d_n3);
        let eq12_e372_d_n4: f64 = (p.p50 * eq12_e371_d_n4);
        let eq12_e372_d_n5: f64 = (p.p50 * eq12_e371_d_n5);
        let eq12_e372_d_n6: f64 = (p.p50 * eq12_e371_d_n6);
        let eq12_e372_d_n7: f64 = (p.p50 * eq12_e371_d_n7);
        let eq12_e372_d_n8: f64 = (p.p50 * eq12_e371_d_n8);
        let eq12_e372_d_n9: f64 = (p.p50 * eq12_e371_d_n9);
        let eq12_e372_d_n10: f64 = (p.p50 * eq12_e371_d_n10);
        let eq12_e372_d_n11: f64 = (p.p50 * eq12_e371_d_n11);
        let eq12_e372_d_n12: f64 = (p.p50 * eq12_e371_d_n12);
        let eq12_e372_d_n13: f64 = (p.p50 * eq12_e371_d_n13);
        let eq12_e372_d_n14: f64 = (p.p50 * eq12_e371_d_n14);
        let eq12_e372_d_n15: f64 = (p.p50 * eq12_e371_d_n15);
        let eq12_e372_d_n16: f64 = (p.p50 * eq12_e371_d_n16);
        let eq12_e372_d_n17: f64 = (p.p50 * eq12_e371_d_n17);
        let eq12_e372_d_n18: f64 = (p.p50 * eq12_e371_d_n18);
        let eq12_e372_d_b0: f64 = (p.p50 * eq12_e371_d_b0);
        let eq12_e372_d_b1: f64 = (p.p50 * eq12_e371_d_b1);
        let eq12_e372_d_b2: f64 = (p.p50 * eq12_e371_d_b2);
        let eq12_e372_d_b3: f64 = (p.p50 * eq12_e371_d_b3);
        let eq12_e372_d_b4: f64 = (p.p50 * eq12_e371_d_b4);
        let eq12_e372_d_b5: f64 = (p.p50 * eq12_e371_d_b5);
        let eq12_e372_d_b6: f64 = (p.p50 * eq12_e371_d_b6);
        let eq12_e372_d_b7: f64 = (p.p50 * eq12_e371_d_b7);
        let eq12_e372_d_b8: f64 = (p.p50 * eq12_e371_d_b8);
        let eq12_e372_d_b9: f64 = (p.p50 * eq12_e371_d_b9);
        let eq12_e372_d_b10: f64 = (p.p50 * eq12_e371_d_b10);
        let eq12_e372_d_b11: f64 = (p.p50 * eq12_e371_d_b11);
        let eq12_e372_d_b12: f64 = (p.p50 * eq12_e371_d_b12);
        let eq12_e372_d_b13: f64 = (p.p50 * eq12_e371_d_b13);
        let eq12_e372_d_b14: f64 = (p.p50 * eq12_e371_d_b14);
        let eq12_e372_d_b15: f64 = (p.p50 * eq12_e371_d_b15);
        let eq12_value: f64 = eq12_e372;
        let eq12_node_derivatives: [f64; 19] = [eq12_e372_d_n0, eq12_e372_d_n1, eq12_e372_d_n2, eq12_e372_d_n3, eq12_e372_d_n4, eq12_e372_d_n5, eq12_e372_d_n6, eq12_e372_d_n7, eq12_e372_d_n8, eq12_e372_d_n9, eq12_e372_d_n10, eq12_e372_d_n11, eq12_e372_d_n12, eq12_e372_d_n13, eq12_e372_d_n14, eq12_e372_d_n15, eq12_e372_d_n16, eq12_e372_d_n17, eq12_e372_d_n18];
        let eq12_branch_derivatives: [f64; 16] = [eq12_e372_d_b0, eq12_e372_d_b1, eq12_e372_d_b2, eq12_e372_d_b3, eq12_e372_d_b4, eq12_e372_d_b5, eq12_e372_d_b6, eq12_e372_d_b7, eq12_e372_d_b8, eq12_e372_d_b9, eq12_e372_d_b10, eq12_e372_d_b11, eq12_e372_d_b12, eq12_e372_d_b13, eq12_e372_d_b14, eq12_e372_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[196]);
        let eq13_e375_d_n0: f64 = (s.dn[196][0] * ddt_scale);
        let eq13_e375_d_n1: f64 = (s.dn[196][1] * ddt_scale);
        let eq13_e375_d_n2: f64 = (s.dn[196][2] * ddt_scale);
        let eq13_e375_d_n3: f64 = (s.dn[196][3] * ddt_scale);
        let eq13_e375_d_n4: f64 = (s.dn[196][4] * ddt_scale);
        let eq13_e375_d_n5: f64 = (s.dn[196][5] * ddt_scale);
        let eq13_e375_d_n6: f64 = (s.dn[196][6] * ddt_scale);
        let eq13_e375_d_n7: f64 = (s.dn[196][7] * ddt_scale);
        let eq13_e375_d_n8: f64 = (s.dn[196][8] * ddt_scale);
        let eq13_e375_d_n9: f64 = (s.dn[196][9] * ddt_scale);
        let eq13_e375_d_n10: f64 = (s.dn[196][10] * ddt_scale);
        let eq13_e375_d_n11: f64 = (s.dn[196][11] * ddt_scale);
        let eq13_e375_d_n12: f64 = (s.dn[196][12] * ddt_scale);
        let eq13_e375_d_n13: f64 = (s.dn[196][13] * ddt_scale);
        let eq13_e375_d_n14: f64 = (s.dn[196][14] * ddt_scale);
        let eq13_e375_d_n15: f64 = (s.dn[196][15] * ddt_scale);
        let eq13_e375_d_n16: f64 = (s.dn[196][16] * ddt_scale);
        let eq13_e375_d_n17: f64 = (s.dn[196][17] * ddt_scale);
        let eq13_e375_d_n18: f64 = (s.dn[196][18] * ddt_scale);
        let eq13_e375_d_b0: f64 = (s.db[196][0] * ddt_scale);
        let eq13_e375_d_b1: f64 = (s.db[196][1] * ddt_scale);
        let eq13_e375_d_b2: f64 = (s.db[196][2] * ddt_scale);
        let eq13_e375_d_b3: f64 = (s.db[196][3] * ddt_scale);
        let eq13_e375_d_b4: f64 = (s.db[196][4] * ddt_scale);
        let eq13_e375_d_b5: f64 = (s.db[196][5] * ddt_scale);
        let eq13_e375_d_b6: f64 = (s.db[196][6] * ddt_scale);
        let eq13_e375_d_b7: f64 = (s.db[196][7] * ddt_scale);
        let eq13_e375_d_b8: f64 = (s.db[196][8] * ddt_scale);
        let eq13_e375_d_b9: f64 = (s.db[196][9] * ddt_scale);
        let eq13_e375_d_b10: f64 = (s.db[196][10] * ddt_scale);
        let eq13_e375_d_b11: f64 = (s.db[196][11] * ddt_scale);
        let eq13_e375_d_b12: f64 = (s.db[196][12] * ddt_scale);
        let eq13_e375_d_b13: f64 = (s.db[196][13] * ddt_scale);
        let eq13_e375_d_b14: f64 = (s.db[196][14] * ddt_scale);
        let eq13_e375_d_b15: f64 = (s.db[196][15] * ddt_scale);
        let eq13_e376: f64 = (p.p50 * eq13_e375);
        let eq13_e376_d_n0: f64 = (p.p50 * eq13_e375_d_n0);
        let eq13_e376_d_n1: f64 = (p.p50 * eq13_e375_d_n1);
        let eq13_e376_d_n2: f64 = (p.p50 * eq13_e375_d_n2);
        let eq13_e376_d_n3: f64 = (p.p50 * eq13_e375_d_n3);
        let eq13_e376_d_n4: f64 = (p.p50 * eq13_e375_d_n4);
        let eq13_e376_d_n5: f64 = (p.p50 * eq13_e375_d_n5);
        let eq13_e376_d_n6: f64 = (p.p50 * eq13_e375_d_n6);
        let eq13_e376_d_n7: f64 = (p.p50 * eq13_e375_d_n7);
        let eq13_e376_d_n8: f64 = (p.p50 * eq13_e375_d_n8);
        let eq13_e376_d_n9: f64 = (p.p50 * eq13_e375_d_n9);
        let eq13_e376_d_n10: f64 = (p.p50 * eq13_e375_d_n10);
        let eq13_e376_d_n11: f64 = (p.p50 * eq13_e375_d_n11);
        let eq13_e376_d_n12: f64 = (p.p50 * eq13_e375_d_n12);
        let eq13_e376_d_n13: f64 = (p.p50 * eq13_e375_d_n13);
        let eq13_e376_d_n14: f64 = (p.p50 * eq13_e375_d_n14);
        let eq13_e376_d_n15: f64 = (p.p50 * eq13_e375_d_n15);
        let eq13_e376_d_n16: f64 = (p.p50 * eq13_e375_d_n16);
        let eq13_e376_d_n17: f64 = (p.p50 * eq13_e375_d_n17);
        let eq13_e376_d_n18: f64 = (p.p50 * eq13_e375_d_n18);
        let eq13_e376_d_b0: f64 = (p.p50 * eq13_e375_d_b0);
        let eq13_e376_d_b1: f64 = (p.p50 * eq13_e375_d_b1);
        let eq13_e376_d_b2: f64 = (p.p50 * eq13_e375_d_b2);
        let eq13_e376_d_b3: f64 = (p.p50 * eq13_e375_d_b3);
        let eq13_e376_d_b4: f64 = (p.p50 * eq13_e375_d_b4);
        let eq13_e376_d_b5: f64 = (p.p50 * eq13_e375_d_b5);
        let eq13_e376_d_b6: f64 = (p.p50 * eq13_e375_d_b6);
        let eq13_e376_d_b7: f64 = (p.p50 * eq13_e375_d_b7);
        let eq13_e376_d_b8: f64 = (p.p50 * eq13_e375_d_b8);
        let eq13_e376_d_b9: f64 = (p.p50 * eq13_e375_d_b9);
        let eq13_e376_d_b10: f64 = (p.p50 * eq13_e375_d_b10);
        let eq13_e376_d_b11: f64 = (p.p50 * eq13_e375_d_b11);
        let eq13_e376_d_b12: f64 = (p.p50 * eq13_e375_d_b12);
        let eq13_e376_d_b13: f64 = (p.p50 * eq13_e375_d_b13);
        let eq13_e376_d_b14: f64 = (p.p50 * eq13_e375_d_b14);
        let eq13_e376_d_b15: f64 = (p.p50 * eq13_e375_d_b15);
        let eq13_value: f64 = eq13_e376;
        let eq13_node_derivatives: [f64; 19] = [eq13_e376_d_n0, eq13_e376_d_n1, eq13_e376_d_n2, eq13_e376_d_n3, eq13_e376_d_n4, eq13_e376_d_n5, eq13_e376_d_n6, eq13_e376_d_n7, eq13_e376_d_n8, eq13_e376_d_n9, eq13_e376_d_n10, eq13_e376_d_n11, eq13_e376_d_n12, eq13_e376_d_n13, eq13_e376_d_n14, eq13_e376_d_n15, eq13_e376_d_n16, eq13_e376_d_n17, eq13_e376_d_n18];
        let eq13_branch_derivatives: [f64; 16] = [eq13_e376_d_b0, eq13_e376_d_b1, eq13_e376_d_b2, eq13_e376_d_b3, eq13_e376_d_b4, eq13_e376_d_b5, eq13_e376_d_b6, eq13_e376_d_b7, eq13_e376_d_b8, eq13_e376_d_b9, eq13_e376_d_b10, eq13_e376_d_b11, eq13_e376_d_b12, eq13_e376_d_b13, eq13_e376_d_b14, eq13_e376_d_b15];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (eq14_value),
        );
        let eq16_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (eq16_value),
        );
        let eq17_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
        );
        let eq18_e402: f64 = (s.v[614] * (nv14 - 0.0));
        let eq18_e402_d_n0: f64 = (s.dn[614][0] * (nv14 - 0.0));
        let eq18_e402_d_n1: f64 = (s.dn[614][1] * (nv14 - 0.0));
        let eq18_e402_d_n2: f64 = (s.dn[614][2] * (nv14 - 0.0));
        let eq18_e402_d_n3: f64 = (s.dn[614][3] * (nv14 - 0.0));
        let eq18_e402_d_n4: f64 = (s.dn[614][4] * (nv14 - 0.0));
        let eq18_e402_d_n5: f64 = (s.dn[614][5] * (nv14 - 0.0));
        let eq18_e402_d_n6: f64 = (s.dn[614][6] * (nv14 - 0.0));
        let eq18_e402_d_n7: f64 = (s.dn[614][7] * (nv14 - 0.0));
        let eq18_e402_d_n8: f64 = (s.dn[614][8] * (nv14 - 0.0));
        let eq18_e402_d_n9: f64 = (s.dn[614][9] * (nv14 - 0.0));
        let eq18_e402_d_n10: f64 = (s.dn[614][10] * (nv14 - 0.0));
        let eq18_e402_d_n11: f64 = (s.dn[614][11] * (nv14 - 0.0));
        let eq18_e402_d_n12: f64 = (s.dn[614][12] * (nv14 - 0.0));
        let eq18_e402_d_n13: f64 = (s.dn[614][13] * (nv14 - 0.0));
        let eq18_e402_d_n14: f64 = ((s.dn[614][14] * (nv14 - 0.0)) + s.v[614]);
        let eq18_e402_d_n15: f64 = (s.dn[614][15] * (nv14 - 0.0));
        let eq18_e402_d_n16: f64 = (s.dn[614][16] * (nv14 - 0.0));
        let eq18_e402_d_n17: f64 = (s.dn[614][17] * (nv14 - 0.0));
        let eq18_e402_d_n18: f64 = (s.dn[614][18] * (nv14 - 0.0));
        let eq18_e402_d_b0: f64 = (s.db[614][0] * (nv14 - 0.0));
        let eq18_e402_d_b1: f64 = (s.db[614][1] * (nv14 - 0.0));
        let eq18_e402_d_b2: f64 = (s.db[614][2] * (nv14 - 0.0));
        let eq18_e402_d_b3: f64 = (s.db[614][3] * (nv14 - 0.0));
        let eq18_e402_d_b4: f64 = (s.db[614][4] * (nv14 - 0.0));
        let eq18_e402_d_b5: f64 = (s.db[614][5] * (nv14 - 0.0));
        let eq18_e402_d_b6: f64 = (s.db[614][6] * (nv14 - 0.0));
        let eq18_e402_d_b7: f64 = (s.db[614][7] * (nv14 - 0.0));
        let eq18_e402_d_b8: f64 = (s.db[614][8] * (nv14 - 0.0));
        let eq18_e402_d_b9: f64 = (s.db[614][9] * (nv14 - 0.0));
        let eq18_e402_d_b10: f64 = (s.db[614][10] * (nv14 - 0.0));
        let eq18_e402_d_b11: f64 = (s.db[614][11] * (nv14 - 0.0));
        let eq18_e402_d_b12: f64 = (s.db[614][12] * (nv14 - 0.0));
        let eq18_e402_d_b13: f64 = (s.db[614][13] * (nv14 - 0.0));
        let eq18_e402_d_b14: f64 = (s.db[614][14] * (nv14 - 0.0));
        let eq18_e402_d_b15: f64 = (s.db[614][15] * (nv14 - 0.0));
        let eq18_value: f64 = eq18_e402;
        let eq18_node_derivatives: [f64; 19] = [eq18_e402_d_n0, eq18_e402_d_n1, eq18_e402_d_n2, eq18_e402_d_n3, eq18_e402_d_n4, eq18_e402_d_n5, eq18_e402_d_n6, eq18_e402_d_n7, eq18_e402_d_n8, eq18_e402_d_n9, eq18_e402_d_n10, eq18_e402_d_n11, eq18_e402_d_n12, eq18_e402_d_n13, eq18_e402_d_n14, eq18_e402_d_n15, eq18_e402_d_n16, eq18_e402_d_n17, eq18_e402_d_n18];
        let eq18_branch_derivatives: [f64; 16] = [eq18_e402_d_b0, eq18_e402_d_b1, eq18_e402_d_b2, eq18_e402_d_b3, eq18_e402_d_b4, eq18_e402_d_b5, eq18_e402_d_b6, eq18_e402_d_b7, eq18_e402_d_b8, eq18_e402_d_b9, eq18_e402_d_b10, eq18_e402_d_b11, eq18_e402_d_b12, eq18_e402_d_b13, eq18_e402_d_b14, eq18_e402_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e405: f64 = ((nv14 - 0.0) * s.v[617]);
        let eq19_e405_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);
        let eq19_e405_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);
        let eq19_e405_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);
        let eq19_e405_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);
        let eq19_e405_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);
        let eq19_e405_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);
        let eq19_e405_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);
        let eq19_e405_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);
        let eq19_e405_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);
        let eq19_e405_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);
        let eq19_e405_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);
        let eq19_e405_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);
        let eq19_e405_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);
        let eq19_e405_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);
        let eq19_e405_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));
        let eq19_e405_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);
        let eq19_e405_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);
        let eq19_e405_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);
        let eq19_e405_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);
        let eq19_e405_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);
        let eq19_e405_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);
        let eq19_e405_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);
        let eq19_e405_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);
        let eq19_e405_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);
        let eq19_e405_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);
        let eq19_e405_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);
        let eq19_e405_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);
        let eq19_e405_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);
        let eq19_e405_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);
        let eq19_e405_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);
        let eq19_e405_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);
        let eq19_e405_d_b12: f64 = ((nv14 - 0.0) * s.db[617][12]);
        let eq19_e405_d_b13: f64 = ((nv14 - 0.0) * s.db[617][13]);
        let eq19_e405_d_b14: f64 = ((nv14 - 0.0) * s.db[617][14]);
        let eq19_e405_d_b15: f64 = ((nv14 - 0.0) * s.db[617][15]);
        let eq19_e406: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq19_e405);
        let eq19_e406_d_n0: f64 = (eq19_e405_d_n0 * ddt_scale);
        let eq19_e406_d_n1: f64 = (eq19_e405_d_n1 * ddt_scale);
        let eq19_e406_d_n2: f64 = (eq19_e405_d_n2 * ddt_scale);
        let eq19_e406_d_n3: f64 = (eq19_e405_d_n3 * ddt_scale);
        let eq19_e406_d_n4: f64 = (eq19_e405_d_n4 * ddt_scale);
        let eq19_e406_d_n5: f64 = (eq19_e405_d_n5 * ddt_scale);
        let eq19_e406_d_n6: f64 = (eq19_e405_d_n6 * ddt_scale);
        let eq19_e406_d_n7: f64 = (eq19_e405_d_n7 * ddt_scale);
        let eq19_e406_d_n8: f64 = (eq19_e405_d_n8 * ddt_scale);
        let eq19_e406_d_n9: f64 = (eq19_e405_d_n9 * ddt_scale);
        let eq19_e406_d_n10: f64 = (eq19_e405_d_n10 * ddt_scale);
        let eq19_e406_d_n11: f64 = (eq19_e405_d_n11 * ddt_scale);
        let eq19_e406_d_n12: f64 = (eq19_e405_d_n12 * ddt_scale);
        let eq19_e406_d_n13: f64 = (eq19_e405_d_n13 * ddt_scale);
        let eq19_e406_d_n14: f64 = (eq19_e405_d_n14 * ddt_scale);
        let eq19_e406_d_n15: f64 = (eq19_e405_d_n15 * ddt_scale);
        let eq19_e406_d_n16: f64 = (eq19_e405_d_n16 * ddt_scale);
        let eq19_e406_d_n17: f64 = (eq19_e405_d_n17 * ddt_scale);
        let eq19_e406_d_n18: f64 = (eq19_e405_d_n18 * ddt_scale);
        let eq19_e406_d_b0: f64 = (eq19_e405_d_b0 * ddt_scale);
        let eq19_e406_d_b1: f64 = (eq19_e405_d_b1 * ddt_scale);
        let eq19_e406_d_b2: f64 = (eq19_e405_d_b2 * ddt_scale);
        let eq19_e406_d_b3: f64 = (eq19_e405_d_b3 * ddt_scale);
        let eq19_e406_d_b4: f64 = (eq19_e405_d_b4 * ddt_scale);
        let eq19_e406_d_b5: f64 = (eq19_e405_d_b5 * ddt_scale);
        let eq19_e406_d_b6: f64 = (eq19_e405_d_b6 * ddt_scale);
        let eq19_e406_d_b7: f64 = (eq19_e405_d_b7 * ddt_scale);
        let eq19_e406_d_b8: f64 = (eq19_e405_d_b8 * ddt_scale);
        let eq19_e406_d_b9: f64 = (eq19_e405_d_b9 * ddt_scale);
        let eq19_e406_d_b10: f64 = (eq19_e405_d_b10 * ddt_scale);
        let eq19_e406_d_b11: f64 = (eq19_e405_d_b11 * ddt_scale);
        let eq19_e406_d_b12: f64 = (eq19_e405_d_b12 * ddt_scale);
        let eq19_e406_d_b13: f64 = (eq19_e405_d_b13 * ddt_scale);
        let eq19_e406_d_b14: f64 = (eq19_e405_d_b14 * ddt_scale);
        let eq19_e406_d_b15: f64 = (eq19_e405_d_b15 * ddt_scale);
        let eq19_value: f64 = eq19_e406;
        let eq19_node_derivatives: [f64; 19] = [eq19_e406_d_n0, eq19_e406_d_n1, eq19_e406_d_n2, eq19_e406_d_n3, eq19_e406_d_n4, eq19_e406_d_n5, eq19_e406_d_n6, eq19_e406_d_n7, eq19_e406_d_n8, eq19_e406_d_n9, eq19_e406_d_n10, eq19_e406_d_n11, eq19_e406_d_n12, eq19_e406_d_n13, eq19_e406_d_n14, eq19_e406_d_n15, eq19_e406_d_n16, eq19_e406_d_n17, eq19_e406_d_n18];
        let eq19_branch_derivatives: [f64; 16] = [eq19_e406_d_b0, eq19_e406_d_b1, eq19_e406_d_b2, eq19_e406_d_b3, eq19_e406_d_b4, eq19_e406_d_b5, eq19_e406_d_b6, eq19_e406_d_b7, eq19_e406_d_b8, eq19_e406_d_b9, eq19_e406_d_b10, eq19_e406_d_b11, eq19_e406_d_b12, eq19_e406_d_b13, eq19_e406_d_b14, eq19_e406_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e409: f64 = ((nv14 - 0.0) * s.v[618]);
        let eq20_e409_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);
        let eq20_e409_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);
        let eq20_e409_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);
        let eq20_e409_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);
        let eq20_e409_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);
        let eq20_e409_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);
        let eq20_e409_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);
        let eq20_e409_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);
        let eq20_e409_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);
        let eq20_e409_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);
        let eq20_e409_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);
        let eq20_e409_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);
        let eq20_e409_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);
        let eq20_e409_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);
        let eq20_e409_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));
        let eq20_e409_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);
        let eq20_e409_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);
        let eq20_e409_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);
        let eq20_e409_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);
        let eq20_e409_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);
        let eq20_e409_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);
        let eq20_e409_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);
        let eq20_e409_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);
        let eq20_e409_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);
        let eq20_e409_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);
        let eq20_e409_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);
        let eq20_e409_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);
        let eq20_e409_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);
        let eq20_e409_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);
        let eq20_e409_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);
        let eq20_e409_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);
        let eq20_e409_d_b12: f64 = ((nv14 - 0.0) * s.db[618][12]);
        let eq20_e409_d_b13: f64 = ((nv14 - 0.0) * s.db[618][13]);
        let eq20_e409_d_b14: f64 = ((nv14 - 0.0) * s.db[618][14]);
        let eq20_e409_d_b15: f64 = ((nv14 - 0.0) * s.db[618][15]);
        let eq20_e410: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq20_e409);
        let eq20_e410_d_n0: f64 = (eq20_e409_d_n0 * ddt_scale);
        let eq20_e410_d_n1: f64 = (eq20_e409_d_n1 * ddt_scale);
        let eq20_e410_d_n2: f64 = (eq20_e409_d_n2 * ddt_scale);
        let eq20_e410_d_n3: f64 = (eq20_e409_d_n3 * ddt_scale);
        let eq20_e410_d_n4: f64 = (eq20_e409_d_n4 * ddt_scale);
        let eq20_e410_d_n5: f64 = (eq20_e409_d_n5 * ddt_scale);
        let eq20_e410_d_n6: f64 = (eq20_e409_d_n6 * ddt_scale);
        let eq20_e410_d_n7: f64 = (eq20_e409_d_n7 * ddt_scale);
        let eq20_e410_d_n8: f64 = (eq20_e409_d_n8 * ddt_scale);
        let eq20_e410_d_n9: f64 = (eq20_e409_d_n9 * ddt_scale);
        let eq20_e410_d_n10: f64 = (eq20_e409_d_n10 * ddt_scale);
        let eq20_e410_d_n11: f64 = (eq20_e409_d_n11 * ddt_scale);
        let eq20_e410_d_n12: f64 = (eq20_e409_d_n12 * ddt_scale);
        let eq20_e410_d_n13: f64 = (eq20_e409_d_n13 * ddt_scale);
        let eq20_e410_d_n14: f64 = (eq20_e409_d_n14 * ddt_scale);
        let eq20_e410_d_n15: f64 = (eq20_e409_d_n15 * ddt_scale);
        let eq20_e410_d_n16: f64 = (eq20_e409_d_n16 * ddt_scale);
        let eq20_e410_d_n17: f64 = (eq20_e409_d_n17 * ddt_scale);
        let eq20_e410_d_n18: f64 = (eq20_e409_d_n18 * ddt_scale);
        let eq20_e410_d_b0: f64 = (eq20_e409_d_b0 * ddt_scale);
        let eq20_e410_d_b1: f64 = (eq20_e409_d_b1 * ddt_scale);
        let eq20_e410_d_b2: f64 = (eq20_e409_d_b2 * ddt_scale);
        let eq20_e410_d_b3: f64 = (eq20_e409_d_b3 * ddt_scale);
        let eq20_e410_d_b4: f64 = (eq20_e409_d_b4 * ddt_scale);
        let eq20_e410_d_b5: f64 = (eq20_e409_d_b5 * ddt_scale);
        let eq20_e410_d_b6: f64 = (eq20_e409_d_b6 * ddt_scale);
        let eq20_e410_d_b7: f64 = (eq20_e409_d_b7 * ddt_scale);
        let eq20_e410_d_b8: f64 = (eq20_e409_d_b8 * ddt_scale);
        let eq20_e410_d_b9: f64 = (eq20_e409_d_b9 * ddt_scale);
        let eq20_e410_d_b10: f64 = (eq20_e409_d_b10 * ddt_scale);
        let eq20_e410_d_b11: f64 = (eq20_e409_d_b11 * ddt_scale);
        let eq20_e410_d_b12: f64 = (eq20_e409_d_b12 * ddt_scale);
        let eq20_e410_d_b13: f64 = (eq20_e409_d_b13 * ddt_scale);
        let eq20_e410_d_b14: f64 = (eq20_e409_d_b14 * ddt_scale);
        let eq20_e410_d_b15: f64 = (eq20_e409_d_b15 * ddt_scale);
        let eq20_value: f64 = eq20_e410;
        let eq20_node_derivatives: [f64; 19] = [eq20_e410_d_n0, eq20_e410_d_n1, eq20_e410_d_n2, eq20_e410_d_n3, eq20_e410_d_n4, eq20_e410_d_n5, eq20_e410_d_n6, eq20_e410_d_n7, eq20_e410_d_n8, eq20_e410_d_n9, eq20_e410_d_n10, eq20_e410_d_n11, eq20_e410_d_n12, eq20_e410_d_n13, eq20_e410_d_n14, eq20_e410_d_n15, eq20_e410_d_n16, eq20_e410_d_n17, eq20_e410_d_n18];
        let eq20_branch_derivatives: [f64; 16] = [eq20_e410_d_b0, eq20_e410_d_b1, eq20_e410_d_b2, eq20_e410_d_b3, eq20_e410_d_b4, eq20_e410_d_b5, eq20_e410_d_b6, eq20_e410_d_b7, eq20_e410_d_b8, eq20_e410_d_b9, eq20_e410_d_b10, eq20_e410_d_b11, eq20_e410_d_b12, eq20_e410_d_b13, eq20_e410_d_b14, eq20_e410_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq21_e418,) = {
    if (p.p259 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e418;
        stamper.stamp_current_const_local(
            Some(7),
            Some(2),
            multiplicity * (eq21_value),
        );
        let (eq22_e426,) = {
    if (p.p260 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e426;
        stamper.stamp_current_const_local(
            Some(0),
            Some(6),
            multiplicity * (eq22_value),
        );
        let (eq23_e436,) = {
    if s.b[1849] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e436;
        stamper.stamp_current_const_local(
            Some(11),
            Some(6),
            multiplicity * (eq23_value),
        );
        let (eq24_e446,) = {
    if s.b[1849] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e446;
        stamper.stamp_current_const_local(
            Some(11),
            Some(7),
            multiplicity * (eq24_value),
        );
        let (eq25_e456,) = {
    if s.b[1849] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e456;
        stamper.stamp_current_const_local(
            Some(11),
            Some(12),
            multiplicity * (eq25_value),
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
        let (eq26_e462, eq26_e462_d_n0, eq26_e462_d_n1, eq26_e462_d_n2, eq26_e462_d_n3, eq26_e462_d_n4, eq26_e462_d_n5, eq26_e462_d_n6, eq26_e462_d_n7, eq26_e462_d_n8, eq26_e462_d_n9, eq26_e462_d_n10, eq26_e462_d_n11, eq26_e462_d_n12, eq26_e462_d_n13, eq26_e462_d_n14, eq26_e462_d_n15, eq26_e462_d_n16, eq26_e462_d_n17, eq26_e462_d_n18, eq26_e462_d_b0, eq26_e462_d_b1, eq26_e462_d_b2, eq26_e462_d_b3, eq26_e462_d_b4, eq26_e462_d_b5, eq26_e462_d_b6, eq26_e462_d_b7, eq26_e462_d_b8, eq26_e462_d_b9, eq26_e462_d_b10, eq26_e462_d_b11, eq26_e462_d_b12, eq26_e462_d_b13, eq26_e462_d_b14, eq26_e462_d_b15,) = {
    if (p.p35 != 0.0) {
        let eq26_e460: f64 = (s.v[551] * (nv1 - nv11));
        let eq26_e460_d_n0: f64 = (s.dn[551][0] * (nv1 - nv11));
        let eq26_e460_d_n1: f64 = ((s.dn[551][1] * (nv1 - nv11)) + s.v[551]);
        let eq26_e460_d_n2: f64 = (s.dn[551][2] * (nv1 - nv11));
        let eq26_e460_d_n3: f64 = (s.dn[551][3] * (nv1 - nv11));
        let eq26_e460_d_n4: f64 = (s.dn[551][4] * (nv1 - nv11));
        let eq26_e460_d_n5: f64 = (s.dn[551][5] * (nv1 - nv11));
        let eq26_e460_d_n6: f64 = (s.dn[551][6] * (nv1 - nv11));
        let eq26_e460_d_n7: f64 = (s.dn[551][7] * (nv1 - nv11));
        let eq26_e460_d_n8: f64 = (s.dn[551][8] * (nv1 - nv11));
        let eq26_e460_d_n9: f64 = (s.dn[551][9] * (nv1 - nv11));
        let eq26_e460_d_n10: f64 = (s.dn[551][10] * (nv1 - nv11));
        let eq26_e460_d_n11: f64 = ((s.dn[551][11] * (nv1 - nv11)) + (-s.v[551]));
        let eq26_e460_d_n12: f64 = (s.dn[551][12] * (nv1 - nv11));
        let eq26_e460_d_n13: f64 = (s.dn[551][13] * (nv1 - nv11));
        let eq26_e460_d_n14: f64 = (s.dn[551][14] * (nv1 - nv11));
        let eq26_e460_d_n15: f64 = (s.dn[551][15] * (nv1 - nv11));
        let eq26_e460_d_n16: f64 = (s.dn[551][16] * (nv1 - nv11));
        let eq26_e460_d_n17: f64 = (s.dn[551][17] * (nv1 - nv11));
        let eq26_e460_d_n18: f64 = (s.dn[551][18] * (nv1 - nv11));
        let eq26_e460_d_b0: f64 = (s.db[551][0] * (nv1 - nv11));
        let eq26_e460_d_b1: f64 = (s.db[551][1] * (nv1 - nv11));
        let eq26_e460_d_b2: f64 = (s.db[551][2] * (nv1 - nv11));
        let eq26_e460_d_b3: f64 = (s.db[551][3] * (nv1 - nv11));
        let eq26_e460_d_b4: f64 = (s.db[551][4] * (nv1 - nv11));
        let eq26_e460_d_b5: f64 = (s.db[551][5] * (nv1 - nv11));
        let eq26_e460_d_b6: f64 = (s.db[551][6] * (nv1 - nv11));
        let eq26_e460_d_b7: f64 = (s.db[551][7] * (nv1 - nv11));
        let eq26_e460_d_b8: f64 = (s.db[551][8] * (nv1 - nv11));
        let eq26_e460_d_b9: f64 = (s.db[551][9] * (nv1 - nv11));
        let eq26_e460_d_b10: f64 = (s.db[551][10] * (nv1 - nv11));
        let eq26_e460_d_b11: f64 = (s.db[551][11] * (nv1 - nv11));
        let eq26_e460_d_b12: f64 = (s.db[551][12] * (nv1 - nv11));
        let eq26_e460_d_b13: f64 = (s.db[551][13] * (nv1 - nv11));
        let eq26_e460_d_b14: f64 = (s.db[551][14] * (nv1 - nv11));
        let eq26_e460_d_b15: f64 = (s.db[551][15] * (nv1 - nv11));
        (eq26_e460, eq26_e460_d_n0, eq26_e460_d_n1, eq26_e460_d_n2, eq26_e460_d_n3, eq26_e460_d_n4, eq26_e460_d_n5, eq26_e460_d_n6, eq26_e460_d_n7, eq26_e460_d_n8, eq26_e460_d_n9, eq26_e460_d_n10, eq26_e460_d_n11, eq26_e460_d_n12, eq26_e460_d_n13, eq26_e460_d_n14, eq26_e460_d_n15, eq26_e460_d_n16, eq26_e460_d_n17, eq26_e460_d_n18, eq26_e460_d_b0, eq26_e460_d_b1, eq26_e460_d_b2, eq26_e460_d_b3, eq26_e460_d_b4, eq26_e460_d_b5, eq26_e460_d_b6, eq26_e460_d_b7, eq26_e460_d_b8, eq26_e460_d_b9, eq26_e460_d_b10, eq26_e460_d_b11, eq26_e460_d_b12, eq26_e460_d_b13, eq26_e460_d_b14, eq26_e460_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e462;
        let eq26_node_derivatives: [f64; 19] = [eq26_e462_d_n0, eq26_e462_d_n1, eq26_e462_d_n2, eq26_e462_d_n3, eq26_e462_d_n4, eq26_e462_d_n5, eq26_e462_d_n6, eq26_e462_d_n7, eq26_e462_d_n8, eq26_e462_d_n9, eq26_e462_d_n10, eq26_e462_d_n11, eq26_e462_d_n12, eq26_e462_d_n13, eq26_e462_d_n14, eq26_e462_d_n15, eq26_e462_d_n16, eq26_e462_d_n17, eq26_e462_d_n18];
        let eq26_branch_derivatives: [f64; 16] = [eq26_e462_d_b0, eq26_e462_d_b1, eq26_e462_d_b2, eq26_e462_d_b3, eq26_e462_d_b4, eq26_e462_d_b5, eq26_e462_d_b6, eq26_e462_d_b7, eq26_e462_d_b8, eq26_e462_d_b9, eq26_e462_d_b10, eq26_e462_d_b11, eq26_e462_d_b12, eq26_e462_d_b13, eq26_e462_d_b14, eq26_e462_d_b15];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(11),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e467,) = {
    if (p.p35 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e467;
        stamper.stamp_potential_const_local(
            5,
            eq27_value,
        );
        let (eq28_e473, eq28_e473_d_n0, eq28_e473_d_n1, eq28_e473_d_n2, eq28_e473_d_n3, eq28_e473_d_n4, eq28_e473_d_n5, eq28_e473_d_n6, eq28_e473_d_n7, eq28_e473_d_n8, eq28_e473_d_n9, eq28_e473_d_n10, eq28_e473_d_n11, eq28_e473_d_n12, eq28_e473_d_n13, eq28_e473_d_n14, eq28_e473_d_n15, eq28_e473_d_n16, eq28_e473_d_n17, eq28_e473_d_n18, eq28_e473_d_b0, eq28_e473_d_b1, eq28_e473_d_b2, eq28_e473_d_b3, eq28_e473_d_b4, eq28_e473_d_b5, eq28_e473_d_b6, eq28_e473_d_b7, eq28_e473_d_b8, eq28_e473_d_b9, eq28_e473_d_b10, eq28_e473_d_b11, eq28_e473_d_b12, eq28_e473_d_b13, eq28_e473_d_b14, eq28_e473_d_b15,) = {
    if s.b[1850] {
        let eq28_e471: f64 = ((nv10 - 0.0) * s.v[589]);
        let eq28_e471_d_n0: f64 = ((nv10 - 0.0) * s.dn[589][0]);
        let eq28_e471_d_n1: f64 = ((nv10 - 0.0) * s.dn[589][1]);
        let eq28_e471_d_n2: f64 = ((nv10 - 0.0) * s.dn[589][2]);
        let eq28_e471_d_n3: f64 = ((nv10 - 0.0) * s.dn[589][3]);
        let eq28_e471_d_n4: f64 = ((nv10 - 0.0) * s.dn[589][4]);
        let eq28_e471_d_n5: f64 = ((nv10 - 0.0) * s.dn[589][5]);
        let eq28_e471_d_n6: f64 = ((nv10 - 0.0) * s.dn[589][6]);
        let eq28_e471_d_n7: f64 = ((nv10 - 0.0) * s.dn[589][7]);
        let eq28_e471_d_n8: f64 = ((nv10 - 0.0) * s.dn[589][8]);
        let eq28_e471_d_n9: f64 = ((nv10 - 0.0) * s.dn[589][9]);
        let eq28_e471_d_n10: f64 = (s.v[589] + ((nv10 - 0.0) * s.dn[589][10]));
        let eq28_e471_d_n11: f64 = ((nv10 - 0.0) * s.dn[589][11]);
        let eq28_e471_d_n12: f64 = ((nv10 - 0.0) * s.dn[589][12]);
        let eq28_e471_d_n13: f64 = ((nv10 - 0.0) * s.dn[589][13]);
        let eq28_e471_d_n14: f64 = ((nv10 - 0.0) * s.dn[589][14]);
        let eq28_e471_d_n15: f64 = ((nv10 - 0.0) * s.dn[589][15]);
        let eq28_e471_d_n16: f64 = ((nv10 - 0.0) * s.dn[589][16]);
        let eq28_e471_d_n17: f64 = ((nv10 - 0.0) * s.dn[589][17]);
        let eq28_e471_d_n18: f64 = ((nv10 - 0.0) * s.dn[589][18]);
        let eq28_e471_d_b0: f64 = ((nv10 - 0.0) * s.db[589][0]);
        let eq28_e471_d_b1: f64 = ((nv10 - 0.0) * s.db[589][1]);
        let eq28_e471_d_b2: f64 = ((nv10 - 0.0) * s.db[589][2]);
        let eq28_e471_d_b3: f64 = ((nv10 - 0.0) * s.db[589][3]);
        let eq28_e471_d_b4: f64 = ((nv10 - 0.0) * s.db[589][4]);
        let eq28_e471_d_b5: f64 = ((nv10 - 0.0) * s.db[589][5]);
        let eq28_e471_d_b6: f64 = ((nv10 - 0.0) * s.db[589][6]);
        let eq28_e471_d_b7: f64 = ((nv10 - 0.0) * s.db[589][7]);
        let eq28_e471_d_b8: f64 = ((nv10 - 0.0) * s.db[589][8]);
        let eq28_e471_d_b9: f64 = ((nv10 - 0.0) * s.db[589][9]);
        let eq28_e471_d_b10: f64 = ((nv10 - 0.0) * s.db[589][10]);
        let eq28_e471_d_b11: f64 = ((nv10 - 0.0) * s.db[589][11]);
        let eq28_e471_d_b12: f64 = ((nv10 - 0.0) * s.db[589][12]);
        let eq28_e471_d_b13: f64 = ((nv10 - 0.0) * s.db[589][13]);
        let eq28_e471_d_b14: f64 = ((nv10 - 0.0) * s.db[589][14]);
        let eq28_e471_d_b15: f64 = ((nv10 - 0.0) * s.db[589][15]);
        (eq28_e471, eq28_e471_d_n0, eq28_e471_d_n1, eq28_e471_d_n2, eq28_e471_d_n3, eq28_e471_d_n4, eq28_e471_d_n5, eq28_e471_d_n6, eq28_e471_d_n7, eq28_e471_d_n8, eq28_e471_d_n9, eq28_e471_d_n10, eq28_e471_d_n11, eq28_e471_d_n12, eq28_e471_d_n13, eq28_e471_d_n14, eq28_e471_d_n15, eq28_e471_d_n16, eq28_e471_d_n17, eq28_e471_d_n18, eq28_e471_d_b0, eq28_e471_d_b1, eq28_e471_d_b2, eq28_e471_d_b3, eq28_e471_d_b4, eq28_e471_d_b5, eq28_e471_d_b6, eq28_e471_d_b7, eq28_e471_d_b8, eq28_e471_d_b9, eq28_e471_d_b10, eq28_e471_d_b11, eq28_e471_d_b12, eq28_e471_d_b13, eq28_e471_d_b14, eq28_e471_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e473;
        let eq28_node_derivatives: [f64; 19] = [eq28_e473_d_n0, eq28_e473_d_n1, eq28_e473_d_n2, eq28_e473_d_n3, eq28_e473_d_n4, eq28_e473_d_n5, eq28_e473_d_n6, eq28_e473_d_n7, eq28_e473_d_n8, eq28_e473_d_n9, eq28_e473_d_n10, eq28_e473_d_n11, eq28_e473_d_n12, eq28_e473_d_n13, eq28_e473_d_n14, eq28_e473_d_n15, eq28_e473_d_n16, eq28_e473_d_n17, eq28_e473_d_n18];
        let eq28_branch_derivatives: [f64; 16] = [eq28_e473_d_b0, eq28_e473_d_b1, eq28_e473_d_b2, eq28_e473_d_b3, eq28_e473_d_b4, eq28_e473_d_b5, eq28_e473_d_b6, eq28_e473_d_b7, eq28_e473_d_b8, eq28_e473_d_b9, eq28_e473_d_b10, eq28_e473_d_b11, eq28_e473_d_b12, eq28_e473_d_b13, eq28_e473_d_b14, eq28_e473_d_b15];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e478, eq29_e478_d_n0, eq29_e478_d_n1, eq29_e478_d_n2, eq29_e478_d_n3, eq29_e478_d_n4, eq29_e478_d_n5, eq29_e478_d_n6, eq29_e478_d_n7, eq29_e478_d_n8, eq29_e478_d_n9, eq29_e478_d_n10, eq29_e478_d_n11, eq29_e478_d_n12, eq29_e478_d_n13, eq29_e478_d_n14, eq29_e478_d_n15, eq29_e478_d_n16, eq29_e478_d_n17, eq29_e478_d_n18, eq29_e478_d_b0, eq29_e478_d_b1, eq29_e478_d_b2, eq29_e478_d_b3, eq29_e478_d_b4, eq29_e478_d_b5, eq29_e478_d_b6, eq29_e478_d_b7, eq29_e478_d_b8, eq29_e478_d_b9, eq29_e478_d_b10, eq29_e478_d_b11, eq29_e478_d_b12, eq29_e478_d_b13, eq29_e478_d_b14, eq29_e478_d_b15,) = {
    if s.b[1850] {
        let eq29_e476: f64 = (-s.v[595]);
        let eq29_e476_d_n0: f64 = (-s.dn[595][0]);
        let eq29_e476_d_n1: f64 = (-s.dn[595][1]);
        let eq29_e476_d_n2: f64 = (-s.dn[595][2]);
        let eq29_e476_d_n3: f64 = (-s.dn[595][3]);
        let eq29_e476_d_n4: f64 = (-s.dn[595][4]);
        let eq29_e476_d_n5: f64 = (-s.dn[595][5]);
        let eq29_e476_d_n6: f64 = (-s.dn[595][6]);
        let eq29_e476_d_n7: f64 = (-s.dn[595][7]);
        let eq29_e476_d_n8: f64 = (-s.dn[595][8]);
        let eq29_e476_d_n9: f64 = (-s.dn[595][9]);
        let eq29_e476_d_n10: f64 = (-s.dn[595][10]);
        let eq29_e476_d_n11: f64 = (-s.dn[595][11]);
        let eq29_e476_d_n12: f64 = (-s.dn[595][12]);
        let eq29_e476_d_n13: f64 = (-s.dn[595][13]);
        let eq29_e476_d_n14: f64 = (-s.dn[595][14]);
        let eq29_e476_d_n15: f64 = (-s.dn[595][15]);
        let eq29_e476_d_n16: f64 = (-s.dn[595][16]);
        let eq29_e476_d_n17: f64 = (-s.dn[595][17]);
        let eq29_e476_d_n18: f64 = (-s.dn[595][18]);
        let eq29_e476_d_b0: f64 = (-s.db[595][0]);
        let eq29_e476_d_b1: f64 = (-s.db[595][1]);
        let eq29_e476_d_b2: f64 = (-s.db[595][2]);
        let eq29_e476_d_b3: f64 = (-s.db[595][3]);
        let eq29_e476_d_b4: f64 = (-s.db[595][4]);
        let eq29_e476_d_b5: f64 = (-s.db[595][5]);
        let eq29_e476_d_b6: f64 = (-s.db[595][6]);
        let eq29_e476_d_b7: f64 = (-s.db[595][7]);
        let eq29_e476_d_b8: f64 = (-s.db[595][8]);
        let eq29_e476_d_b9: f64 = (-s.db[595][9]);
        let eq29_e476_d_b10: f64 = (-s.db[595][10]);
        let eq29_e476_d_b11: f64 = (-s.db[595][11]);
        let eq29_e476_d_b12: f64 = (-s.db[595][12]);
        let eq29_e476_d_b13: f64 = (-s.db[595][13]);
        let eq29_e476_d_b14: f64 = (-s.db[595][14]);
        let eq29_e476_d_b15: f64 = (-s.db[595][15]);
        (eq29_e476, eq29_e476_d_n0, eq29_e476_d_n1, eq29_e476_d_n2, eq29_e476_d_n3, eq29_e476_d_n4, eq29_e476_d_n5, eq29_e476_d_n6, eq29_e476_d_n7, eq29_e476_d_n8, eq29_e476_d_n9, eq29_e476_d_n10, eq29_e476_d_n11, eq29_e476_d_n12, eq29_e476_d_n13, eq29_e476_d_n14, eq29_e476_d_n15, eq29_e476_d_n16, eq29_e476_d_n17, eq29_e476_d_n18, eq29_e476_d_b0, eq29_e476_d_b1, eq29_e476_d_b2, eq29_e476_d_b3, eq29_e476_d_b4, eq29_e476_d_b5, eq29_e476_d_b6, eq29_e476_d_b7, eq29_e476_d_b8, eq29_e476_d_b9, eq29_e476_d_b10, eq29_e476_d_b11, eq29_e476_d_b12, eq29_e476_d_b13, eq29_e476_d_b14, eq29_e476_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e478;
        let eq29_node_derivatives: [f64; 19] = [eq29_e478_d_n0, eq29_e478_d_n1, eq29_e478_d_n2, eq29_e478_d_n3, eq29_e478_d_n4, eq29_e478_d_n5, eq29_e478_d_n6, eq29_e478_d_n7, eq29_e478_d_n8, eq29_e478_d_n9, eq29_e478_d_n10, eq29_e478_d_n11, eq29_e478_d_n12, eq29_e478_d_n13, eq29_e478_d_n14, eq29_e478_d_n15, eq29_e478_d_n16, eq29_e478_d_n17, eq29_e478_d_n18];
        let eq29_branch_derivatives: [f64; 16] = [eq29_e478_d_b0, eq29_e478_d_b1, eq29_e478_d_b2, eq29_e478_d_b3, eq29_e478_d_b4, eq29_e478_d_b5, eq29_e478_d_b6, eq29_e478_d_b7, eq29_e478_d_b8, eq29_e478_d_b9, eq29_e478_d_b10, eq29_e478_d_b11, eq29_e478_d_b12, eq29_e478_d_b13, eq29_e478_d_b14, eq29_e478_d_b15];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq31_e491, eq31_e491_d_n0, eq31_e491_d_n1, eq31_e491_d_n2, eq31_e491_d_n3, eq31_e491_d_n4, eq31_e491_d_n5, eq31_e491_d_n6, eq31_e491_d_n7, eq31_e491_d_n8, eq31_e491_d_n9, eq31_e491_d_n10, eq31_e491_d_n11, eq31_e491_d_n12, eq31_e491_d_n13, eq31_e491_d_n14, eq31_e491_d_n15, eq31_e491_d_n16, eq31_e491_d_n17, eq31_e491_d_n18, eq31_e491_d_b0, eq31_e491_d_b1, eq31_e491_d_b2, eq31_e491_d_b3, eq31_e491_d_b4, eq31_e491_d_b5, eq31_e491_d_b6, eq31_e491_d_b7, eq31_e491_d_b8, eq31_e491_d_b9, eq31_e491_d_b10, eq31_e491_d_b11, eq31_e491_d_b12, eq31_e491_d_b13, eq31_e491_d_b14, eq31_e491_d_b15,) = {
    if s.b[1850] {
        let eq31_e488: f64 = (s.v[563] * (nv10 - 0.0));
        let eq31_e488_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));
        let eq31_e488_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));
        let eq31_e488_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));
        let eq31_e488_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));
        let eq31_e488_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));
        let eq31_e488_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));
        let eq31_e488_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));
        let eq31_e488_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));
        let eq31_e488_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));
        let eq31_e488_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));
        let eq31_e488_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);
        let eq31_e488_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));
        let eq31_e488_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));
        let eq31_e488_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));
        let eq31_e488_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));
        let eq31_e488_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));
        let eq31_e488_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));
        let eq31_e488_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));
        let eq31_e488_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));
        let eq31_e488_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));
        let eq31_e488_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));
        let eq31_e488_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));
        let eq31_e488_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));
        let eq31_e488_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));
        let eq31_e488_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));
        let eq31_e488_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));
        let eq31_e488_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));
        let eq31_e488_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));
        let eq31_e488_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));
        let eq31_e488_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));
        let eq31_e488_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));
        let eq31_e488_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));
        let eq31_e488_d_b13: f64 = (s.db[563][13] * (nv10 - 0.0));
        let eq31_e488_d_b14: f64 = (s.db[563][14] * (nv10 - 0.0));
        let eq31_e488_d_b15: f64 = (s.db[563][15] * (nv10 - 0.0));
        let eq31_e489: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq31_e488);
        let eq31_e489_d_n0: f64 = (eq31_e488_d_n0 * ddt_scale);
        let eq31_e489_d_n1: f64 = (eq31_e488_d_n1 * ddt_scale);
        let eq31_e489_d_n2: f64 = (eq31_e488_d_n2 * ddt_scale);
        let eq31_e489_d_n3: f64 = (eq31_e488_d_n3 * ddt_scale);
        let eq31_e489_d_n4: f64 = (eq31_e488_d_n4 * ddt_scale);
        let eq31_e489_d_n5: f64 = (eq31_e488_d_n5 * ddt_scale);
        let eq31_e489_d_n6: f64 = (eq31_e488_d_n6 * ddt_scale);
        let eq31_e489_d_n7: f64 = (eq31_e488_d_n7 * ddt_scale);
        let eq31_e489_d_n8: f64 = (eq31_e488_d_n8 * ddt_scale);
        let eq31_e489_d_n9: f64 = (eq31_e488_d_n9 * ddt_scale);
        let eq31_e489_d_n10: f64 = (eq31_e488_d_n10 * ddt_scale);
        let eq31_e489_d_n11: f64 = (eq31_e488_d_n11 * ddt_scale);
        let eq31_e489_d_n12: f64 = (eq31_e488_d_n12 * ddt_scale);
        let eq31_e489_d_n13: f64 = (eq31_e488_d_n13 * ddt_scale);
        let eq31_e489_d_n14: f64 = (eq31_e488_d_n14 * ddt_scale);
        let eq31_e489_d_n15: f64 = (eq31_e488_d_n15 * ddt_scale);
        let eq31_e489_d_n16: f64 = (eq31_e488_d_n16 * ddt_scale);
        let eq31_e489_d_n17: f64 = (eq31_e488_d_n17 * ddt_scale);
        let eq31_e489_d_n18: f64 = (eq31_e488_d_n18 * ddt_scale);
        let eq31_e489_d_b0: f64 = (eq31_e488_d_b0 * ddt_scale);
        let eq31_e489_d_b1: f64 = (eq31_e488_d_b1 * ddt_scale);
        let eq31_e489_d_b2: f64 = (eq31_e488_d_b2 * ddt_scale);
        let eq31_e489_d_b3: f64 = (eq31_e488_d_b3 * ddt_scale);
        let eq31_e489_d_b4: f64 = (eq31_e488_d_b4 * ddt_scale);
        let eq31_e489_d_b5: f64 = (eq31_e488_d_b5 * ddt_scale);
        let eq31_e489_d_b6: f64 = (eq31_e488_d_b6 * ddt_scale);
        let eq31_e489_d_b7: f64 = (eq31_e488_d_b7 * ddt_scale);
        let eq31_e489_d_b8: f64 = (eq31_e488_d_b8 * ddt_scale);
        let eq31_e489_d_b9: f64 = (eq31_e488_d_b9 * ddt_scale);
        let eq31_e489_d_b10: f64 = (eq31_e488_d_b10 * ddt_scale);
        let eq31_e489_d_b11: f64 = (eq31_e488_d_b11 * ddt_scale);
        let eq31_e489_d_b12: f64 = (eq31_e488_d_b12 * ddt_scale);
        let eq31_e489_d_b13: f64 = (eq31_e488_d_b13 * ddt_scale);
        let eq31_e489_d_b14: f64 = (eq31_e488_d_b14 * ddt_scale);
        let eq31_e489_d_b15: f64 = (eq31_e488_d_b15 * ddt_scale);
        (eq31_e489, eq31_e489_d_n0, eq31_e489_d_n1, eq31_e489_d_n2, eq31_e489_d_n3, eq31_e489_d_n4, eq31_e489_d_n5, eq31_e489_d_n6, eq31_e489_d_n7, eq31_e489_d_n8, eq31_e489_d_n9, eq31_e489_d_n10, eq31_e489_d_n11, eq31_e489_d_n12, eq31_e489_d_n13, eq31_e489_d_n14, eq31_e489_d_n15, eq31_e489_d_n16, eq31_e489_d_n17, eq31_e489_d_n18, eq31_e489_d_b0, eq31_e489_d_b1, eq31_e489_d_b2, eq31_e489_d_b3, eq31_e489_d_b4, eq31_e489_d_b5, eq31_e489_d_b6, eq31_e489_d_b7, eq31_e489_d_b8, eq31_e489_d_b9, eq31_e489_d_b10, eq31_e489_d_b11, eq31_e489_d_b12, eq31_e489_d_b13, eq31_e489_d_b14, eq31_e489_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e491;
        let eq31_node_derivatives: [f64; 19] = [eq31_e491_d_n0, eq31_e491_d_n1, eq31_e491_d_n2, eq31_e491_d_n3, eq31_e491_d_n4, eq31_e491_d_n5, eq31_e491_d_n6, eq31_e491_d_n7, eq31_e491_d_n8, eq31_e491_d_n9, eq31_e491_d_n10, eq31_e491_d_n11, eq31_e491_d_n12, eq31_e491_d_n13, eq31_e491_d_n14, eq31_e491_d_n15, eq31_e491_d_n16, eq31_e491_d_n17, eq31_e491_d_n18];
        let eq31_branch_derivatives: [f64; 16] = [eq31_e491_d_b0, eq31_e491_d_b1, eq31_e491_d_b2, eq31_e491_d_b3, eq31_e491_d_b4, eq31_e491_d_b5, eq31_e491_d_b6, eq31_e491_d_b7, eq31_e491_d_b8, eq31_e491_d_b9, eq31_e491_d_b10, eq31_e491_d_b11, eq31_e491_d_b12, eq31_e491_d_b13, eq31_e491_d_b14, eq31_e491_d_b15];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq33_e506, eq33_e506_d_n0, eq33_e506_d_n1, eq33_e506_d_n2, eq33_e506_d_n3, eq33_e506_d_n4, eq33_e506_d_n5, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n8, eq33_e506_d_n9, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n13, eq33_e506_d_n14, eq33_e506_d_n15, eq33_e506_d_n16, eq33_e506_d_n17, eq33_e506_d_n18, eq33_e506_d_b0, eq33_e506_d_b1, eq33_e506_d_b2, eq33_e506_d_b3, eq33_e506_d_b4, eq33_e506_d_b5, eq33_e506_d_b6, eq33_e506_d_b7, eq33_e506_d_b8, eq33_e506_d_b9, eq33_e506_d_b10, eq33_e506_d_b11, eq33_e506_d_b12, eq33_e506_d_b13, eq33_e506_d_b14, eq33_e506_d_b15,) = {
    if s.b[1851] {
        let eq33_e503: f64 = (s.v[311] + s.v[263]);
        let eq33_e503_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);
        let eq33_e503_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);
        let eq33_e503_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);
        let eq33_e503_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);
        let eq33_e503_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);
        let eq33_e503_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);
        let eq33_e503_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);
        let eq33_e503_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);
        let eq33_e503_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);
        let eq33_e503_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);
        let eq33_e503_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);
        let eq33_e503_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);
        let eq33_e503_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);
        let eq33_e503_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);
        let eq33_e503_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);
        let eq33_e503_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);
        let eq33_e503_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);
        let eq33_e503_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);
        let eq33_e503_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);
        let eq33_e503_d_b0: f64 = (s.db[311][0] + s.db[263][0]);
        let eq33_e503_d_b1: f64 = (s.db[311][1] + s.db[263][1]);
        let eq33_e503_d_b2: f64 = (s.db[311][2] + s.db[263][2]);
        let eq33_e503_d_b3: f64 = (s.db[311][3] + s.db[263][3]);
        let eq33_e503_d_b4: f64 = (s.db[311][4] + s.db[263][4]);
        let eq33_e503_d_b5: f64 = (s.db[311][5] + s.db[263][5]);
        let eq33_e503_d_b6: f64 = (s.db[311][6] + s.db[263][6]);
        let eq33_e503_d_b7: f64 = (s.db[311][7] + s.db[263][7]);
        let eq33_e503_d_b8: f64 = (s.db[311][8] + s.db[263][8]);
        let eq33_e503_d_b9: f64 = (s.db[311][9] + s.db[263][9]);
        let eq33_e503_d_b10: f64 = (s.db[311][10] + s.db[263][10]);
        let eq33_e503_d_b11: f64 = (s.db[311][11] + s.db[263][11]);
        let eq33_e503_d_b12: f64 = (s.db[311][12] + s.db[263][12]);
        let eq33_e503_d_b13: f64 = (s.db[311][13] + s.db[263][13]);
        let eq33_e503_d_b14: f64 = (s.db[311][14] + s.db[263][14]);
        let eq33_e503_d_b15: f64 = (s.db[311][15] + s.db[263][15]);
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
        let eq33_e504_d_b15: f64 = (p.p50 * eq33_e503_d_b15);
        (eq33_e504, eq33_e504_d_n0, eq33_e504_d_n1, eq33_e504_d_n2, eq33_e504_d_n3, eq33_e504_d_n4, eq33_e504_d_n5, eq33_e504_d_n6, eq33_e504_d_n7, eq33_e504_d_n8, eq33_e504_d_n9, eq33_e504_d_n10, eq33_e504_d_n11, eq33_e504_d_n12, eq33_e504_d_n13, eq33_e504_d_n14, eq33_e504_d_n15, eq33_e504_d_n16, eq33_e504_d_n17, eq33_e504_d_n18, eq33_e504_d_b0, eq33_e504_d_b1, eq33_e504_d_b2, eq33_e504_d_b3, eq33_e504_d_b4, eq33_e504_d_b5, eq33_e504_d_b6, eq33_e504_d_b7, eq33_e504_d_b8, eq33_e504_d_b9, eq33_e504_d_b10, eq33_e504_d_b11, eq33_e504_d_b12, eq33_e504_d_b13, eq33_e504_d_b14, eq33_e504_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e506;
        let eq33_node_derivatives: [f64; 19] = [eq33_e506_d_n0, eq33_e506_d_n1, eq33_e506_d_n2, eq33_e506_d_n3, eq33_e506_d_n4, eq33_e506_d_n5, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n8, eq33_e506_d_n9, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n13, eq33_e506_d_n14, eq33_e506_d_n15, eq33_e506_d_n16, eq33_e506_d_n17, eq33_e506_d_n18];
        let eq33_branch_derivatives: [f64; 16] = [eq33_e506_d_b0, eq33_e506_d_b1, eq33_e506_d_b2, eq33_e506_d_b3, eq33_e506_d_b4, eq33_e506_d_b5, eq33_e506_d_b6, eq33_e506_d_b7, eq33_e506_d_b8, eq33_e506_d_b9, eq33_e506_d_b10, eq33_e506_d_b11, eq33_e506_d_b12, eq33_e506_d_b13, eq33_e506_d_b14, eq33_e506_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(12),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e514, eq34_e514_d_n0, eq34_e514_d_n1, eq34_e514_d_n2, eq34_e514_d_n3, eq34_e514_d_n4, eq34_e514_d_n5, eq34_e514_d_n6, eq34_e514_d_n7, eq34_e514_d_n8, eq34_e514_d_n9, eq34_e514_d_n10, eq34_e514_d_n11, eq34_e514_d_n12, eq34_e514_d_n13, eq34_e514_d_n14, eq34_e514_d_n15, eq34_e514_d_n16, eq34_e514_d_n17, eq34_e514_d_n18, eq34_e514_d_b0, eq34_e514_d_b1, eq34_e514_d_b2, eq34_e514_d_b3, eq34_e514_d_b4, eq34_e514_d_b5, eq34_e514_d_b6, eq34_e514_d_b7, eq34_e514_d_b8, eq34_e514_d_b9, eq34_e514_d_b10, eq34_e514_d_b11, eq34_e514_d_b12, eq34_e514_d_b13, eq34_e514_d_b14, eq34_e514_d_b15,) = {
    if s.b[1851] {
        let eq34_e511: f64 = (s.v[312] + s.v[573]);
        let eq34_e511_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);
        let eq34_e511_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);
        let eq34_e511_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);
        let eq34_e511_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);
        let eq34_e511_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);
        let eq34_e511_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);
        let eq34_e511_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);
        let eq34_e511_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);
        let eq34_e511_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);
        let eq34_e511_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);
        let eq34_e511_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);
        let eq34_e511_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);
        let eq34_e511_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);
        let eq34_e511_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);
        let eq34_e511_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);
        let eq34_e511_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);
        let eq34_e511_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);
        let eq34_e511_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);
        let eq34_e511_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);
        let eq34_e511_d_b0: f64 = (s.db[312][0] + s.db[573][0]);
        let eq34_e511_d_b1: f64 = (s.db[312][1] + s.db[573][1]);
        let eq34_e511_d_b2: f64 = (s.db[312][2] + s.db[573][2]);
        let eq34_e511_d_b3: f64 = (s.db[312][3] + s.db[573][3]);
        let eq34_e511_d_b4: f64 = (s.db[312][4] + s.db[573][4]);
        let eq34_e511_d_b5: f64 = (s.db[312][5] + s.db[573][5]);
        let eq34_e511_d_b6: f64 = (s.db[312][6] + s.db[573][6]);
        let eq34_e511_d_b7: f64 = (s.db[312][7] + s.db[573][7]);
        let eq34_e511_d_b8: f64 = (s.db[312][8] + s.db[573][8]);
        let eq34_e511_d_b9: f64 = (s.db[312][9] + s.db[573][9]);
        let eq34_e511_d_b10: f64 = (s.db[312][10] + s.db[573][10]);
        let eq34_e511_d_b11: f64 = (s.db[312][11] + s.db[573][11]);
        let eq34_e511_d_b12: f64 = (s.db[312][12] + s.db[573][12]);
        let eq34_e511_d_b13: f64 = (s.db[312][13] + s.db[573][13]);
        let eq34_e511_d_b14: f64 = (s.db[312][14] + s.db[573][14]);
        let eq34_e511_d_b15: f64 = (s.db[312][15] + s.db[573][15]);
        let eq34_e512: f64 = (p.p50 * eq34_e511);
        let eq34_e512_d_n0: f64 = (p.p50 * eq34_e511_d_n0);
        let eq34_e512_d_n1: f64 = (p.p50 * eq34_e511_d_n1);
        let eq34_e512_d_n2: f64 = (p.p50 * eq34_e511_d_n2);
        let eq34_e512_d_n3: f64 = (p.p50 * eq34_e511_d_n3);
        let eq34_e512_d_n4: f64 = (p.p50 * eq34_e511_d_n4);
        let eq34_e512_d_n5: f64 = (p.p50 * eq34_e511_d_n5);
        let eq34_e512_d_n6: f64 = (p.p50 * eq34_e511_d_n6);
        let eq34_e512_d_n7: f64 = (p.p50 * eq34_e511_d_n7);
        let eq34_e512_d_n8: f64 = (p.p50 * eq34_e511_d_n8);
        let eq34_e512_d_n9: f64 = (p.p50 * eq34_e511_d_n9);
        let eq34_e512_d_n10: f64 = (p.p50 * eq34_e511_d_n10);
        let eq34_e512_d_n11: f64 = (p.p50 * eq34_e511_d_n11);
        let eq34_e512_d_n12: f64 = (p.p50 * eq34_e511_d_n12);
        let eq34_e512_d_n13: f64 = (p.p50 * eq34_e511_d_n13);
        let eq34_e512_d_n14: f64 = (p.p50 * eq34_e511_d_n14);
        let eq34_e512_d_n15: f64 = (p.p50 * eq34_e511_d_n15);
        let eq34_e512_d_n16: f64 = (p.p50 * eq34_e511_d_n16);
        let eq34_e512_d_n17: f64 = (p.p50 * eq34_e511_d_n17);
        let eq34_e512_d_n18: f64 = (p.p50 * eq34_e511_d_n18);
        let eq34_e512_d_b0: f64 = (p.p50 * eq34_e511_d_b0);
        let eq34_e512_d_b1: f64 = (p.p50 * eq34_e511_d_b1);
        let eq34_e512_d_b2: f64 = (p.p50 * eq34_e511_d_b2);
        let eq34_e512_d_b3: f64 = (p.p50 * eq34_e511_d_b3);
        let eq34_e512_d_b4: f64 = (p.p50 * eq34_e511_d_b4);
        let eq34_e512_d_b5: f64 = (p.p50 * eq34_e511_d_b5);
        let eq34_e512_d_b6: f64 = (p.p50 * eq34_e511_d_b6);
        let eq34_e512_d_b7: f64 = (p.p50 * eq34_e511_d_b7);
        let eq34_e512_d_b8: f64 = (p.p50 * eq34_e511_d_b8);
        let eq34_e512_d_b9: f64 = (p.p50 * eq34_e511_d_b9);
        let eq34_e512_d_b10: f64 = (p.p50 * eq34_e511_d_b10);
        let eq34_e512_d_b11: f64 = (p.p50 * eq34_e511_d_b11);
        let eq34_e512_d_b12: f64 = (p.p50 * eq34_e511_d_b12);
        let eq34_e512_d_b13: f64 = (p.p50 * eq34_e511_d_b13);
        let eq34_e512_d_b14: f64 = (p.p50 * eq34_e511_d_b14);
        let eq34_e512_d_b15: f64 = (p.p50 * eq34_e511_d_b15);
        (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n1, eq34_e512_d_n2, eq34_e512_d_n3, eq34_e512_d_n4, eq34_e512_d_n5, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n8, eq34_e512_d_n9, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n13, eq34_e512_d_n14, eq34_e512_d_n15, eq34_e512_d_n16, eq34_e512_d_n17, eq34_e512_d_n18, eq34_e512_d_b0, eq34_e512_d_b1, eq34_e512_d_b2, eq34_e512_d_b3, eq34_e512_d_b4, eq34_e512_d_b5, eq34_e512_d_b6, eq34_e512_d_b7, eq34_e512_d_b8, eq34_e512_d_b9, eq34_e512_d_b10, eq34_e512_d_b11, eq34_e512_d_b12, eq34_e512_d_b13, eq34_e512_d_b14, eq34_e512_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e514;
        let eq34_node_derivatives: [f64; 19] = [eq34_e514_d_n0, eq34_e514_d_n1, eq34_e514_d_n2, eq34_e514_d_n3, eq34_e514_d_n4, eq34_e514_d_n5, eq34_e514_d_n6, eq34_e514_d_n7, eq34_e514_d_n8, eq34_e514_d_n9, eq34_e514_d_n10, eq34_e514_d_n11, eq34_e514_d_n12, eq34_e514_d_n13, eq34_e514_d_n14, eq34_e514_d_n15, eq34_e514_d_n16, eq34_e514_d_n17, eq34_e514_d_n18];
        let eq34_branch_derivatives: [f64; 16] = [eq34_e514_d_b0, eq34_e514_d_b1, eq34_e514_d_b2, eq34_e514_d_b3, eq34_e514_d_b4, eq34_e514_d_b5, eq34_e514_d_b6, eq34_e514_d_b7, eq34_e514_d_b8, eq34_e514_d_b9, eq34_e514_d_b10, eq34_e514_d_b11, eq34_e514_d_b12, eq34_e514_d_b13, eq34_e514_d_b14, eq34_e514_d_b15];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(12),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n1, eq35_e523_d_n2, eq35_e523_d_n3, eq35_e523_d_n4, eq35_e523_d_n5, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n8, eq35_e523_d_n9, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n13, eq35_e523_d_n14, eq35_e523_d_n15, eq35_e523_d_n16, eq35_e523_d_n17, eq35_e523_d_n18, eq35_e523_d_b0, eq35_e523_d_b1, eq35_e523_d_b2, eq35_e523_d_b3, eq35_e523_d_b4, eq35_e523_d_b5, eq35_e523_d_b6, eq35_e523_d_b7, eq35_e523_d_b8, eq35_e523_d_b9, eq35_e523_d_b10, eq35_e523_d_b11, eq35_e523_d_b12, eq35_e523_d_b13, eq35_e523_d_b14, eq35_e523_d_b15,) = {
    if s.b[1851] {
        let eq35_e519: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[283]);
        let eq35_e519_d_n0: f64 = (s.dn[283][0] * ddt_scale);
        let eq35_e519_d_n1: f64 = (s.dn[283][1] * ddt_scale);
        let eq35_e519_d_n2: f64 = (s.dn[283][2] * ddt_scale);
        let eq35_e519_d_n3: f64 = (s.dn[283][3] * ddt_scale);
        let eq35_e519_d_n4: f64 = (s.dn[283][4] * ddt_scale);
        let eq35_e519_d_n5: f64 = (s.dn[283][5] * ddt_scale);
        let eq35_e519_d_n6: f64 = (s.dn[283][6] * ddt_scale);
        let eq35_e519_d_n7: f64 = (s.dn[283][7] * ddt_scale);
        let eq35_e519_d_n8: f64 = (s.dn[283][8] * ddt_scale);
        let eq35_e519_d_n9: f64 = (s.dn[283][9] * ddt_scale);
        let eq35_e519_d_n10: f64 = (s.dn[283][10] * ddt_scale);
        let eq35_e519_d_n11: f64 = (s.dn[283][11] * ddt_scale);
        let eq35_e519_d_n12: f64 = (s.dn[283][12] * ddt_scale);
        let eq35_e519_d_n13: f64 = (s.dn[283][13] * ddt_scale);
        let eq35_e519_d_n14: f64 = (s.dn[283][14] * ddt_scale);
        let eq35_e519_d_n15: f64 = (s.dn[283][15] * ddt_scale);
        let eq35_e519_d_n16: f64 = (s.dn[283][16] * ddt_scale);
        let eq35_e519_d_n17: f64 = (s.dn[283][17] * ddt_scale);
        let eq35_e519_d_n18: f64 = (s.dn[283][18] * ddt_scale);
        let eq35_e519_d_b0: f64 = (s.db[283][0] * ddt_scale);
        let eq35_e519_d_b1: f64 = (s.db[283][1] * ddt_scale);
        let eq35_e519_d_b2: f64 = (s.db[283][2] * ddt_scale);
        let eq35_e519_d_b3: f64 = (s.db[283][3] * ddt_scale);
        let eq35_e519_d_b4: f64 = (s.db[283][4] * ddt_scale);
        let eq35_e519_d_b5: f64 = (s.db[283][5] * ddt_scale);
        let eq35_e519_d_b6: f64 = (s.db[283][6] * ddt_scale);
        let eq35_e519_d_b7: f64 = (s.db[283][7] * ddt_scale);
        let eq35_e519_d_b8: f64 = (s.db[283][8] * ddt_scale);
        let eq35_e519_d_b9: f64 = (s.db[283][9] * ddt_scale);
        let eq35_e519_d_b10: f64 = (s.db[283][10] * ddt_scale);
        let eq35_e519_d_b11: f64 = (s.db[283][11] * ddt_scale);
        let eq35_e519_d_b12: f64 = (s.db[283][12] * ddt_scale);
        let eq35_e519_d_b13: f64 = (s.db[283][13] * ddt_scale);
        let eq35_e519_d_b14: f64 = (s.db[283][14] * ddt_scale);
        let eq35_e519_d_b15: f64 = (s.db[283][15] * ddt_scale);
        let eq35_e520: f64 = (s.v[281] + eq35_e519);
        let eq35_e520_d_n0: f64 = (s.dn[281][0] + eq35_e519_d_n0);
        let eq35_e520_d_n1: f64 = (s.dn[281][1] + eq35_e519_d_n1);
        let eq35_e520_d_n2: f64 = (s.dn[281][2] + eq35_e519_d_n2);
        let eq35_e520_d_n3: f64 = (s.dn[281][3] + eq35_e519_d_n3);
        let eq35_e520_d_n4: f64 = (s.dn[281][4] + eq35_e519_d_n4);
        let eq35_e520_d_n5: f64 = (s.dn[281][5] + eq35_e519_d_n5);
        let eq35_e520_d_n6: f64 = (s.dn[281][6] + eq35_e519_d_n6);
        let eq35_e520_d_n7: f64 = (s.dn[281][7] + eq35_e519_d_n7);
        let eq35_e520_d_n8: f64 = (s.dn[281][8] + eq35_e519_d_n8);
        let eq35_e520_d_n9: f64 = (s.dn[281][9] + eq35_e519_d_n9);
        let eq35_e520_d_n10: f64 = (s.dn[281][10] + eq35_e519_d_n10);
        let eq35_e520_d_n11: f64 = (s.dn[281][11] + eq35_e519_d_n11);
        let eq35_e520_d_n12: f64 = (s.dn[281][12] + eq35_e519_d_n12);
        let eq35_e520_d_n13: f64 = (s.dn[281][13] + eq35_e519_d_n13);
        let eq35_e520_d_n14: f64 = (s.dn[281][14] + eq35_e519_d_n14);
        let eq35_e520_d_n15: f64 = (s.dn[281][15] + eq35_e519_d_n15);
        let eq35_e520_d_n16: f64 = (s.dn[281][16] + eq35_e519_d_n16);
        let eq35_e520_d_n17: f64 = (s.dn[281][17] + eq35_e519_d_n17);
        let eq35_e520_d_n18: f64 = (s.dn[281][18] + eq35_e519_d_n18);
        let eq35_e520_d_b0: f64 = (s.db[281][0] + eq35_e519_d_b0);
        let eq35_e520_d_b1: f64 = (s.db[281][1] + eq35_e519_d_b1);
        let eq35_e520_d_b2: f64 = (s.db[281][2] + eq35_e519_d_b2);
        let eq35_e520_d_b3: f64 = (s.db[281][3] + eq35_e519_d_b3);
        let eq35_e520_d_b4: f64 = (s.db[281][4] + eq35_e519_d_b4);
        let eq35_e520_d_b5: f64 = (s.db[281][5] + eq35_e519_d_b5);
        let eq35_e520_d_b6: f64 = (s.db[281][6] + eq35_e519_d_b6);
        let eq35_e520_d_b7: f64 = (s.db[281][7] + eq35_e519_d_b7);
        let eq35_e520_d_b8: f64 = (s.db[281][8] + eq35_e519_d_b8);
        let eq35_e520_d_b9: f64 = (s.db[281][9] + eq35_e519_d_b9);
        let eq35_e520_d_b10: f64 = (s.db[281][10] + eq35_e519_d_b10);
        let eq35_e520_d_b11: f64 = (s.db[281][11] + eq35_e519_d_b11);
        let eq35_e520_d_b12: f64 = (s.db[281][12] + eq35_e519_d_b12);
        let eq35_e520_d_b13: f64 = (s.db[281][13] + eq35_e519_d_b13);
        let eq35_e520_d_b14: f64 = (s.db[281][14] + eq35_e519_d_b14);
        let eq35_e520_d_b15: f64 = (s.db[281][15] + eq35_e519_d_b15);
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n1: f64 = (p.p50 * eq35_e520_d_n1);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n3: f64 = (p.p50 * eq35_e520_d_n3);
        let eq35_e521_d_n4: f64 = (p.p50 * eq35_e520_d_n4);
        let eq35_e521_d_n5: f64 = (p.p50 * eq35_e520_d_n5);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n8: f64 = (p.p50 * eq35_e520_d_n8);
        let eq35_e521_d_n9: f64 = (p.p50 * eq35_e520_d_n9);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n13: f64 = (p.p50 * eq35_e520_d_n13);
        let eq35_e521_d_n14: f64 = (p.p50 * eq35_e520_d_n14);
        let eq35_e521_d_n15: f64 = (p.p50 * eq35_e520_d_n15);
        let eq35_e521_d_n16: f64 = (p.p50 * eq35_e520_d_n16);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        let eq35_e521_d_n18: f64 = (p.p50 * eq35_e520_d_n18);
        let eq35_e521_d_b0: f64 = (p.p50 * eq35_e520_d_b0);
        let eq35_e521_d_b1: f64 = (p.p50 * eq35_e520_d_b1);
        let eq35_e521_d_b2: f64 = (p.p50 * eq35_e520_d_b2);
        let eq35_e521_d_b3: f64 = (p.p50 * eq35_e520_d_b3);
        let eq35_e521_d_b4: f64 = (p.p50 * eq35_e520_d_b4);
        let eq35_e521_d_b5: f64 = (p.p50 * eq35_e520_d_b5);
        let eq35_e521_d_b6: f64 = (p.p50 * eq35_e520_d_b6);
        let eq35_e521_d_b7: f64 = (p.p50 * eq35_e520_d_b7);
        let eq35_e521_d_b8: f64 = (p.p50 * eq35_e520_d_b8);
        let eq35_e521_d_b9: f64 = (p.p50 * eq35_e520_d_b9);
        let eq35_e521_d_b10: f64 = (p.p50 * eq35_e520_d_b10);
        let eq35_e521_d_b11: f64 = (p.p50 * eq35_e520_d_b11);
        let eq35_e521_d_b12: f64 = (p.p50 * eq35_e520_d_b12);
        let eq35_e521_d_b13: f64 = (p.p50 * eq35_e520_d_b13);
        let eq35_e521_d_b14: f64 = (p.p50 * eq35_e520_d_b14);
        let eq35_e521_d_b15: f64 = (p.p50 * eq35_e520_d_b15);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n1, eq35_e521_d_n2, eq35_e521_d_n3, eq35_e521_d_n4, eq35_e521_d_n5, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n8, eq35_e521_d_n9, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n13, eq35_e521_d_n14, eq35_e521_d_n15, eq35_e521_d_n16, eq35_e521_d_n17, eq35_e521_d_n18, eq35_e521_d_b0, eq35_e521_d_b1, eq35_e521_d_b2, eq35_e521_d_b3, eq35_e521_d_b4, eq35_e521_d_b5, eq35_e521_d_b6, eq35_e521_d_b7, eq35_e521_d_b8, eq35_e521_d_b9, eq35_e521_d_b10, eq35_e521_d_b11, eq35_e521_d_b12, eq35_e521_d_b13, eq35_e521_d_b14, eq35_e521_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e523;
        let eq35_node_derivatives: [f64; 19] = [eq35_e523_d_n0, eq35_e523_d_n1, eq35_e523_d_n2, eq35_e523_d_n3, eq35_e523_d_n4, eq35_e523_d_n5, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n8, eq35_e523_d_n9, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n13, eq35_e523_d_n14, eq35_e523_d_n15, eq35_e523_d_n16, eq35_e523_d_n17, eq35_e523_d_n18];
        let eq35_branch_derivatives: [f64; 16] = [eq35_e523_d_b0, eq35_e523_d_b1, eq35_e523_d_b2, eq35_e523_d_b3, eq35_e523_d_b4, eq35_e523_d_b5, eq35_e523_d_b6, eq35_e523_d_b7, eq35_e523_d_b8, eq35_e523_d_b9, eq35_e523_d_b10, eq35_e523_d_b11, eq35_e523_d_b12, eq35_e523_d_b13, eq35_e523_d_b14, eq35_e523_d_b15];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n1, eq36_e532_d_n2, eq36_e532_d_n3, eq36_e532_d_n4, eq36_e532_d_n5, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n8, eq36_e532_d_n9, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n13, eq36_e532_d_n14, eq36_e532_d_n15, eq36_e532_d_n16, eq36_e532_d_n17, eq36_e532_d_n18, eq36_e532_d_b0, eq36_e532_d_b1, eq36_e532_d_b2, eq36_e532_d_b3, eq36_e532_d_b4, eq36_e532_d_b5, eq36_e532_d_b6, eq36_e532_d_b7, eq36_e532_d_b8, eq36_e532_d_b9, eq36_e532_d_b10, eq36_e532_d_b11, eq36_e532_d_b12, eq36_e532_d_b13, eq36_e532_d_b14, eq36_e532_d_b15,) = {
    if s.b[1851] {
        let eq36_e528: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, s.v[284]);
        let eq36_e528_d_n0: f64 = (s.dn[284][0] * ddt_scale);
        let eq36_e528_d_n1: f64 = (s.dn[284][1] * ddt_scale);
        let eq36_e528_d_n2: f64 = (s.dn[284][2] * ddt_scale);
        let eq36_e528_d_n3: f64 = (s.dn[284][3] * ddt_scale);
        let eq36_e528_d_n4: f64 = (s.dn[284][4] * ddt_scale);
        let eq36_e528_d_n5: f64 = (s.dn[284][5] * ddt_scale);
        let eq36_e528_d_n6: f64 = (s.dn[284][6] * ddt_scale);
        let eq36_e528_d_n7: f64 = (s.dn[284][7] * ddt_scale);
        let eq36_e528_d_n8: f64 = (s.dn[284][8] * ddt_scale);
        let eq36_e528_d_n9: f64 = (s.dn[284][9] * ddt_scale);
        let eq36_e528_d_n10: f64 = (s.dn[284][10] * ddt_scale);
        let eq36_e528_d_n11: f64 = (s.dn[284][11] * ddt_scale);
        let eq36_e528_d_n12: f64 = (s.dn[284][12] * ddt_scale);
        let eq36_e528_d_n13: f64 = (s.dn[284][13] * ddt_scale);
        let eq36_e528_d_n14: f64 = (s.dn[284][14] * ddt_scale);
        let eq36_e528_d_n15: f64 = (s.dn[284][15] * ddt_scale);
        let eq36_e528_d_n16: f64 = (s.dn[284][16] * ddt_scale);
        let eq36_e528_d_n17: f64 = (s.dn[284][17] * ddt_scale);
        let eq36_e528_d_n18: f64 = (s.dn[284][18] * ddt_scale);
        let eq36_e528_d_b0: f64 = (s.db[284][0] * ddt_scale);
        let eq36_e528_d_b1: f64 = (s.db[284][1] * ddt_scale);
        let eq36_e528_d_b2: f64 = (s.db[284][2] * ddt_scale);
        let eq36_e528_d_b3: f64 = (s.db[284][3] * ddt_scale);
        let eq36_e528_d_b4: f64 = (s.db[284][4] * ddt_scale);
        let eq36_e528_d_b5: f64 = (s.db[284][5] * ddt_scale);
        let eq36_e528_d_b6: f64 = (s.db[284][6] * ddt_scale);
        let eq36_e528_d_b7: f64 = (s.db[284][7] * ddt_scale);
        let eq36_e528_d_b8: f64 = (s.db[284][8] * ddt_scale);
        let eq36_e528_d_b9: f64 = (s.db[284][9] * ddt_scale);
        let eq36_e528_d_b10: f64 = (s.db[284][10] * ddt_scale);
        let eq36_e528_d_b11: f64 = (s.db[284][11] * ddt_scale);
        let eq36_e528_d_b12: f64 = (s.db[284][12] * ddt_scale);
        let eq36_e528_d_b13: f64 = (s.db[284][13] * ddt_scale);
        let eq36_e528_d_b14: f64 = (s.db[284][14] * ddt_scale);
        let eq36_e528_d_b15: f64 = (s.db[284][15] * ddt_scale);
        let eq36_e529: f64 = (s.v[282] + eq36_e528);
        let eq36_e529_d_n0: f64 = (s.dn[282][0] + eq36_e528_d_n0);
        let eq36_e529_d_n1: f64 = (s.dn[282][1] + eq36_e528_d_n1);
        let eq36_e529_d_n2: f64 = (s.dn[282][2] + eq36_e528_d_n2);
        let eq36_e529_d_n3: f64 = (s.dn[282][3] + eq36_e528_d_n3);
        let eq36_e529_d_n4: f64 = (s.dn[282][4] + eq36_e528_d_n4);
        let eq36_e529_d_n5: f64 = (s.dn[282][5] + eq36_e528_d_n5);
        let eq36_e529_d_n6: f64 = (s.dn[282][6] + eq36_e528_d_n6);
        let eq36_e529_d_n7: f64 = (s.dn[282][7] + eq36_e528_d_n7);
        let eq36_e529_d_n8: f64 = (s.dn[282][8] + eq36_e528_d_n8);
        let eq36_e529_d_n9: f64 = (s.dn[282][9] + eq36_e528_d_n9);
        let eq36_e529_d_n10: f64 = (s.dn[282][10] + eq36_e528_d_n10);
        let eq36_e529_d_n11: f64 = (s.dn[282][11] + eq36_e528_d_n11);
        let eq36_e529_d_n12: f64 = (s.dn[282][12] + eq36_e528_d_n12);
        let eq36_e529_d_n13: f64 = (s.dn[282][13] + eq36_e528_d_n13);
        let eq36_e529_d_n14: f64 = (s.dn[282][14] + eq36_e528_d_n14);
        let eq36_e529_d_n15: f64 = (s.dn[282][15] + eq36_e528_d_n15);
        let eq36_e529_d_n16: f64 = (s.dn[282][16] + eq36_e528_d_n16);
        let eq36_e529_d_n17: f64 = (s.dn[282][17] + eq36_e528_d_n17);
        let eq36_e529_d_n18: f64 = (s.dn[282][18] + eq36_e528_d_n18);
        let eq36_e529_d_b0: f64 = (s.db[282][0] + eq36_e528_d_b0);
        let eq36_e529_d_b1: f64 = (s.db[282][1] + eq36_e528_d_b1);
        let eq36_e529_d_b2: f64 = (s.db[282][2] + eq36_e528_d_b2);
        let eq36_e529_d_b3: f64 = (s.db[282][3] + eq36_e528_d_b3);
        let eq36_e529_d_b4: f64 = (s.db[282][4] + eq36_e528_d_b4);
        let eq36_e529_d_b5: f64 = (s.db[282][5] + eq36_e528_d_b5);
        let eq36_e529_d_b6: f64 = (s.db[282][6] + eq36_e528_d_b6);
        let eq36_e529_d_b7: f64 = (s.db[282][7] + eq36_e528_d_b7);
        let eq36_e529_d_b8: f64 = (s.db[282][8] + eq36_e528_d_b8);
        let eq36_e529_d_b9: f64 = (s.db[282][9] + eq36_e528_d_b9);
        let eq36_e529_d_b10: f64 = (s.db[282][10] + eq36_e528_d_b10);
        let eq36_e529_d_b11: f64 = (s.db[282][11] + eq36_e528_d_b11);
        let eq36_e529_d_b12: f64 = (s.db[282][12] + eq36_e528_d_b12);
        let eq36_e529_d_b13: f64 = (s.db[282][13] + eq36_e528_d_b13);
        let eq36_e529_d_b14: f64 = (s.db[282][14] + eq36_e528_d_b14);
        let eq36_e529_d_b15: f64 = (s.db[282][15] + eq36_e528_d_b15);
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n1: f64 = (p.p50 * eq36_e529_d_n1);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n3: f64 = (p.p50 * eq36_e529_d_n3);
        let eq36_e530_d_n4: f64 = (p.p50 * eq36_e529_d_n4);
        let eq36_e530_d_n5: f64 = (p.p50 * eq36_e529_d_n5);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n8: f64 = (p.p50 * eq36_e529_d_n8);
        let eq36_e530_d_n9: f64 = (p.p50 * eq36_e529_d_n9);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n13: f64 = (p.p50 * eq36_e529_d_n13);
        let eq36_e530_d_n14: f64 = (p.p50 * eq36_e529_d_n14);
        let eq36_e530_d_n15: f64 = (p.p50 * eq36_e529_d_n15);
        let eq36_e530_d_n16: f64 = (p.p50 * eq36_e529_d_n16);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        let eq36_e530_d_n18: f64 = (p.p50 * eq36_e529_d_n18);
        let eq36_e530_d_b0: f64 = (p.p50 * eq36_e529_d_b0);
        let eq36_e530_d_b1: f64 = (p.p50 * eq36_e529_d_b1);
        let eq36_e530_d_b2: f64 = (p.p50 * eq36_e529_d_b2);
        let eq36_e530_d_b3: f64 = (p.p50 * eq36_e529_d_b3);
        let eq36_e530_d_b4: f64 = (p.p50 * eq36_e529_d_b4);
        let eq36_e530_d_b5: f64 = (p.p50 * eq36_e529_d_b5);
        let eq36_e530_d_b6: f64 = (p.p50 * eq36_e529_d_b6);
        let eq36_e530_d_b7: f64 = (p.p50 * eq36_e529_d_b7);
        let eq36_e530_d_b8: f64 = (p.p50 * eq36_e529_d_b8);
        let eq36_e530_d_b9: f64 = (p.p50 * eq36_e529_d_b9);
        let eq36_e530_d_b10: f64 = (p.p50 * eq36_e529_d_b10);
        let eq36_e530_d_b11: f64 = (p.p50 * eq36_e529_d_b11);
        let eq36_e530_d_b12: f64 = (p.p50 * eq36_e529_d_b12);
        let eq36_e530_d_b13: f64 = (p.p50 * eq36_e529_d_b13);
        let eq36_e530_d_b14: f64 = (p.p50 * eq36_e529_d_b14);
        let eq36_e530_d_b15: f64 = (p.p50 * eq36_e529_d_b15);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n1, eq36_e530_d_n2, eq36_e530_d_n3, eq36_e530_d_n4, eq36_e530_d_n5, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n8, eq36_e530_d_n9, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n13, eq36_e530_d_n14, eq36_e530_d_n15, eq36_e530_d_n16, eq36_e530_d_n17, eq36_e530_d_n18, eq36_e530_d_b0, eq36_e530_d_b1, eq36_e530_d_b2, eq36_e530_d_b3, eq36_e530_d_b4, eq36_e530_d_b5, eq36_e530_d_b6, eq36_e530_d_b7, eq36_e530_d_b8, eq36_e530_d_b9, eq36_e530_d_b10, eq36_e530_d_b11, eq36_e530_d_b12, eq36_e530_d_b13, eq36_e530_d_b14, eq36_e530_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e532;
        let eq36_node_derivatives: [f64; 19] = [eq36_e532_d_n0, eq36_e532_d_n1, eq36_e532_d_n2, eq36_e532_d_n3, eq36_e532_d_n4, eq36_e532_d_n5, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n8, eq36_e532_d_n9, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n13, eq36_e532_d_n14, eq36_e532_d_n15, eq36_e532_d_n16, eq36_e532_d_n17, eq36_e532_d_n18];
        let eq36_branch_derivatives: [f64; 16] = [eq36_e532_d_b0, eq36_e532_d_b1, eq36_e532_d_b2, eq36_e532_d_b3, eq36_e532_d_b4, eq36_e532_d_b5, eq36_e532_d_b6, eq36_e532_d_b7, eq36_e532_d_b8, eq36_e532_d_b9, eq36_e532_d_b10, eq36_e532_d_b11, eq36_e532_d_b12, eq36_e532_d_b13, eq36_e532_d_b14, eq36_e532_d_b15];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq37_e540, eq37_e540_d_n0, eq37_e540_d_n1, eq37_e540_d_n2, eq37_e540_d_n3, eq37_e540_d_n4, eq37_e540_d_n5, eq37_e540_d_n6, eq37_e540_d_n7, eq37_e540_d_n8, eq37_e540_d_n9, eq37_e540_d_n10, eq37_e540_d_n11, eq37_e540_d_n12, eq37_e540_d_n13, eq37_e540_d_n14, eq37_e540_d_n15, eq37_e540_d_n16, eq37_e540_d_n17, eq37_e540_d_n18, eq37_e540_d_b0, eq37_e540_d_b1, eq37_e540_d_b2, eq37_e540_d_b3, eq37_e540_d_b4, eq37_e540_d_b5, eq37_e540_d_b6, eq37_e540_d_b7, eq37_e540_d_b8, eq37_e540_d_b9, eq37_e540_d_b10, eq37_e540_d_b11, eq37_e540_d_b12, eq37_e540_d_b13, eq37_e540_d_b14, eq37_e540_d_b15,) = {
    if (s.b[1851] && (p.p261 != 0.0)) {
        let eq37_e538: f64 = ((nv4 - nv12) / s.v[2]);
        let eq37_e538_d_n0: f64 = (-(((nv4 - nv12) * s.dn[2][0]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n1: f64 = (-(((nv4 - nv12) * s.dn[2][1]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n2: f64 = (-(((nv4 - nv12) * s.dn[2][2]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n3: f64 = (-(((nv4 - nv12) * s.dn[2][3]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n4: f64 = ((s.v[2] - ((nv4 - nv12) * s.dn[2][4])) / (s.v[2] * s.v[2]));
        let eq37_e538_d_n5: f64 = (-(((nv4 - nv12) * s.dn[2][5]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n6: f64 = (-(((nv4 - nv12) * s.dn[2][6]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n7: f64 = (-(((nv4 - nv12) * s.dn[2][7]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n8: f64 = (-(((nv4 - nv12) * s.dn[2][8]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n9: f64 = (-(((nv4 - nv12) * s.dn[2][9]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n10: f64 = (-(((nv4 - nv12) * s.dn[2][10]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n11: f64 = (-(((nv4 - nv12) * s.dn[2][11]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n12: f64 = (((-s.v[2]) - ((nv4 - nv12) * s.dn[2][12])) / (s.v[2] * s.v[2]));
        let eq37_e538_d_n13: f64 = (-(((nv4 - nv12) * s.dn[2][13]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n14: f64 = (-(((nv4 - nv12) * s.dn[2][14]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n15: f64 = (-(((nv4 - nv12) * s.dn[2][15]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n16: f64 = (-(((nv4 - nv12) * s.dn[2][16]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n17: f64 = (-(((nv4 - nv12) * s.dn[2][17]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n18: f64 = (-(((nv4 - nv12) * s.dn[2][18]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b0: f64 = (-(((nv4 - nv12) * s.db[2][0]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b1: f64 = (-(((nv4 - nv12) * s.db[2][1]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b2: f64 = (-(((nv4 - nv12) * s.db[2][2]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b3: f64 = (-(((nv4 - nv12) * s.db[2][3]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b4: f64 = (-(((nv4 - nv12) * s.db[2][4]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b5: f64 = (-(((nv4 - nv12) * s.db[2][5]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b6: f64 = (-(((nv4 - nv12) * s.db[2][6]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b7: f64 = (-(((nv4 - nv12) * s.db[2][7]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b8: f64 = (-(((nv4 - nv12) * s.db[2][8]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b9: f64 = (-(((nv4 - nv12) * s.db[2][9]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b10: f64 = (-(((nv4 - nv12) * s.db[2][10]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b11: f64 = (-(((nv4 - nv12) * s.db[2][11]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b12: f64 = (-(((nv4 - nv12) * s.db[2][12]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b13: f64 = (-(((nv4 - nv12) * s.db[2][13]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b14: f64 = (-(((nv4 - nv12) * s.db[2][14]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b15: f64 = (-(((nv4 - nv12) * s.db[2][15]) / (s.v[2] * s.v[2])));
        (eq37_e538, eq37_e538_d_n0, eq37_e538_d_n1, eq37_e538_d_n2, eq37_e538_d_n3, eq37_e538_d_n4, eq37_e538_d_n5, eq37_e538_d_n6, eq37_e538_d_n7, eq37_e538_d_n8, eq37_e538_d_n9, eq37_e538_d_n10, eq37_e538_d_n11, eq37_e538_d_n12, eq37_e538_d_n13, eq37_e538_d_n14, eq37_e538_d_n15, eq37_e538_d_n16, eq37_e538_d_n17, eq37_e538_d_n18, eq37_e538_d_b0, eq37_e538_d_b1, eq37_e538_d_b2, eq37_e538_d_b3, eq37_e538_d_b4, eq37_e538_d_b5, eq37_e538_d_b6, eq37_e538_d_b7, eq37_e538_d_b8, eq37_e538_d_b9, eq37_e538_d_b10, eq37_e538_d_b11, eq37_e538_d_b12, eq37_e538_d_b13, eq37_e538_d_b14, eq37_e538_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e540;
        let eq37_node_derivatives: [f64; 19] = [eq37_e540_d_n0, eq37_e540_d_n1, eq37_e540_d_n2, eq37_e540_d_n3, eq37_e540_d_n4, eq37_e540_d_n5, eq37_e540_d_n6, eq37_e540_d_n7, eq37_e540_d_n8, eq37_e540_d_n9, eq37_e540_d_n10, eq37_e540_d_n11, eq37_e540_d_n12, eq37_e540_d_n13, eq37_e540_d_n14, eq37_e540_d_n15, eq37_e540_d_n16, eq37_e540_d_n17, eq37_e540_d_n18];
        let eq37_branch_derivatives: [f64; 16] = [eq37_e540_d_b0, eq37_e540_d_b1, eq37_e540_d_b2, eq37_e540_d_b3, eq37_e540_d_b4, eq37_e540_d_b5, eq37_e540_d_b6, eq37_e540_d_b7, eq37_e540_d_b8, eq37_e540_d_b9, eq37_e540_d_b10, eq37_e540_d_b11, eq37_e540_d_b12, eq37_e540_d_b13, eq37_e540_d_b14, eq37_e540_d_b15];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(12),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let (eq38_e547,) = {
    if (s.b[1851] && (p.p261 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq38_value: f64 = eq38_e547;
        stamper.stamp_potential_const_local(
            6,
            eq38_value,
        );
        let (eq41_e570,) = {
    if (s.b[1851] && (p.p262 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e570;
        stamper.stamp_potential_const_local(
            7,
            eq41_value,
        );
        let (eq42_e577,) = {
    if (s.b[1851] && (p.p262 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e577;
        stamper.stamp_potential_const_local(
            8,
            eq42_value,
        );
        let (eq43_e583, eq43_e583_d_n0, eq43_e583_d_n1, eq43_e583_d_n2, eq43_e583_d_n3, eq43_e583_d_n4, eq43_e583_d_n5, eq43_e583_d_n6, eq43_e583_d_n7, eq43_e583_d_n8, eq43_e583_d_n9, eq43_e583_d_n10, eq43_e583_d_n11, eq43_e583_d_n12, eq43_e583_d_n13, eq43_e583_d_n14, eq43_e583_d_n15, eq43_e583_d_n16, eq43_e583_d_n17, eq43_e583_d_n18, eq43_e583_d_b0, eq43_e583_d_b1, eq43_e583_d_b2, eq43_e583_d_b3, eq43_e583_d_b4, eq43_e583_d_b5, eq43_e583_d_b6, eq43_e583_d_b7, eq43_e583_d_b8, eq43_e583_d_b9, eq43_e583_d_b10, eq43_e583_d_b11, eq43_e583_d_b12, eq43_e583_d_b13, eq43_e583_d_b14, eq43_e583_d_b15,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        (s.v[582], s.dn[582][0], s.dn[582][1], s.dn[582][2], s.dn[582][3], s.dn[582][4], s.dn[582][5], s.dn[582][6], s.dn[582][7], s.dn[582][8], s.dn[582][9], s.dn[582][10], s.dn[582][11], s.dn[582][12], s.dn[582][13], s.dn[582][14], s.dn[582][15], s.dn[582][16], s.dn[582][17], s.dn[582][18], s.db[582][0], s.db[582][1], s.db[582][2], s.db[582][3], s.db[582][4], s.db[582][5], s.db[582][6], s.db[582][7], s.db[582][8], s.db[582][9], s.db[582][10], s.db[582][11], s.db[582][12], s.db[582][13], s.db[582][14], s.db[582][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e583;
        let eq43_node_derivatives: [f64; 19] = [eq43_e583_d_n0, eq43_e583_d_n1, eq43_e583_d_n2, eq43_e583_d_n3, eq43_e583_d_n4, eq43_e583_d_n5, eq43_e583_d_n6, eq43_e583_d_n7, eq43_e583_d_n8, eq43_e583_d_n9, eq43_e583_d_n10, eq43_e583_d_n11, eq43_e583_d_n12, eq43_e583_d_n13, eq43_e583_d_n14, eq43_e583_d_n15, eq43_e583_d_n16, eq43_e583_d_n17, eq43_e583_d_n18];
        let eq43_branch_derivatives: [f64; 16] = [eq43_e583_d_b0, eq43_e583_d_b1, eq43_e583_d_b2, eq43_e583_d_b3, eq43_e583_d_b4, eq43_e583_d_b5, eq43_e583_d_b6, eq43_e583_d_b7, eq43_e583_d_b8, eq43_e583_d_b9, eq43_e583_d_b10, eq43_e583_d_b11, eq43_e583_d_b12, eq43_e583_d_b13, eq43_e583_d_b14, eq43_e583_d_b15];
        stamper.stamp_current_dense_local(
            Some(18),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e589, eq44_e589_d_n0, eq44_e589_d_n1, eq44_e589_d_n2, eq44_e589_d_n3, eq44_e589_d_n4, eq44_e589_d_n5, eq44_e589_d_n6, eq44_e589_d_n7, eq44_e589_d_n8, eq44_e589_d_n9, eq44_e589_d_n10, eq44_e589_d_n11, eq44_e589_d_n12, eq44_e589_d_n13, eq44_e589_d_n14, eq44_e589_d_n15, eq44_e589_d_n16, eq44_e589_d_n17, eq44_e589_d_n18, eq44_e589_d_b0, eq44_e589_d_b1, eq44_e589_d_b2, eq44_e589_d_b3, eq44_e589_d_b4, eq44_e589_d_b5, eq44_e589_d_b6, eq44_e589_d_b7, eq44_e589_d_b8, eq44_e589_d_b9, eq44_e589_d_b10, eq44_e589_d_b11, eq44_e589_d_b12, eq44_e589_d_b13, eq44_e589_d_b14, eq44_e589_d_b15,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12], s.db[583][13], s.db[583][14], s.db[583][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e589;
        let eq44_node_derivatives: [f64; 19] = [eq44_e589_d_n0, eq44_e589_d_n1, eq44_e589_d_n2, eq44_e589_d_n3, eq44_e589_d_n4, eq44_e589_d_n5, eq44_e589_d_n6, eq44_e589_d_n7, eq44_e589_d_n8, eq44_e589_d_n9, eq44_e589_d_n10, eq44_e589_d_n11, eq44_e589_d_n12, eq44_e589_d_n13, eq44_e589_d_n14, eq44_e589_d_n15, eq44_e589_d_n16, eq44_e589_d_n17, eq44_e589_d_n18];
        let eq44_branch_derivatives: [f64; 16] = [eq44_e589_d_b0, eq44_e589_d_b1, eq44_e589_d_b2, eq44_e589_d_b3, eq44_e589_d_b4, eq44_e589_d_b5, eq44_e589_d_b6, eq44_e589_d_b7, eq44_e589_d_b8, eq44_e589_d_b9, eq44_e589_d_b10, eq44_e589_d_b11, eq44_e589_d_b12, eq44_e589_d_b13, eq44_e589_d_b14, eq44_e589_d_b15];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq47_e616, eq47_e616_d_n18,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq47_e613);
        let eq47_e614_d_n18: f64 = (eq47_e611 * ddt_scale);
        (eq47_e614, eq47_e614_d_n18,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e616;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq47_value),
            18,
            multiplicity * (eq47_e616_d_n18),
        );
        let (eq48_e627, eq48_e627_d_n13,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq48_e624);
        let eq48_e625_d_n13: f64 = (eq48_e622 * ddt_scale);
        (eq48_e625, eq48_e625_d_n13,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e627;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq48_value),
            13,
            multiplicity * (eq48_e627_d_n13),
        );
        let (eq49_e634,) = {
    if (s.b[1851] && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e634;
        stamper.stamp_potential_const_local(
            9,
            eq49_value,
        );
        let (eq50_e641,) = {
    if (s.b[1851] && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e641;
        stamper.stamp_potential_const_local(
            10,
            eq50_value,
        );
        let (eq51_e647, eq51_e647_d_n0, eq51_e647_d_n1, eq51_e647_d_n2, eq51_e647_d_n3, eq51_e647_d_n4, eq51_e647_d_n5, eq51_e647_d_n6, eq51_e647_d_n7, eq51_e647_d_n8, eq51_e647_d_n9, eq51_e647_d_n10, eq51_e647_d_n11, eq51_e647_d_n12, eq51_e647_d_n13, eq51_e647_d_n14, eq51_e647_d_n15, eq51_e647_d_n16, eq51_e647_d_n17, eq51_e647_d_n18, eq51_e647_d_b0, eq51_e647_d_b1, eq51_e647_d_b2, eq51_e647_d_b3, eq51_e647_d_b4, eq51_e647_d_b5, eq51_e647_d_b6, eq51_e647_d_b7, eq51_e647_d_b8, eq51_e647_d_b9, eq51_e647_d_b10, eq51_e647_d_b11, eq51_e647_d_b12, eq51_e647_d_b13, eq51_e647_d_b14, eq51_e647_d_b15,) = {
    if (s.b[1851] && s.b[1852]) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12], s.db[592][13], s.db[592][14], s.db[592][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e647;
        let eq51_node_derivatives: [f64; 19] = [eq51_e647_d_n0, eq51_e647_d_n1, eq51_e647_d_n2, eq51_e647_d_n3, eq51_e647_d_n4, eq51_e647_d_n5, eq51_e647_d_n6, eq51_e647_d_n7, eq51_e647_d_n8, eq51_e647_d_n9, eq51_e647_d_n10, eq51_e647_d_n11, eq51_e647_d_n12, eq51_e647_d_n13, eq51_e647_d_n14, eq51_e647_d_n15, eq51_e647_d_n16, eq51_e647_d_n17, eq51_e647_d_n18];
        let eq51_branch_derivatives: [f64; 16] = [eq51_e647_d_b0, eq51_e647_d_b1, eq51_e647_d_b2, eq51_e647_d_b3, eq51_e647_d_b4, eq51_e647_d_b5, eq51_e647_d_b6, eq51_e647_d_b7, eq51_e647_d_b8, eq51_e647_d_b9, eq51_e647_d_b10, eq51_e647_d_b11, eq51_e647_d_b12, eq51_e647_d_b13, eq51_e647_d_b14, eq51_e647_d_b15];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq53_e666, eq53_e666_d_n17,) = {
    if (s.b[1851] && s.b[1852]) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq53_e663);
        let eq53_e664_d_n17: f64 = (eq53_e661 * ddt_scale);
        (eq53_e664, eq53_e664_d_n17,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e666;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq53_value),
            17,
            multiplicity * (eq53_e666_d_n17),
        );
        let (eq54_e673,) = {
    if (s.b[1851] && (!s.b[1852])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e673;
        stamper.stamp_potential_const_local(
            11,
            eq54_value,
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
        let (eq55_e682, eq55_e682_d_n0, eq55_e682_d_n1, eq55_e682_d_n2, eq55_e682_d_n3, eq55_e682_d_n4, eq55_e682_d_n5, eq55_e682_d_n6, eq55_e682_d_n7, eq55_e682_d_n8, eq55_e682_d_n9, eq55_e682_d_n10, eq55_e682_d_n11, eq55_e682_d_n12, eq55_e682_d_n13, eq55_e682_d_n14, eq55_e682_d_n15, eq55_e682_d_n16, eq55_e682_d_n17, eq55_e682_d_n18, eq55_e682_d_b0, eq55_e682_d_b1, eq55_e682_d_b2, eq55_e682_d_b3, eq55_e682_d_b4, eq55_e682_d_b5, eq55_e682_d_b6, eq55_e682_d_b7, eq55_e682_d_b8, eq55_e682_d_b9, eq55_e682_d_b10, eq55_e682_d_b11, eq55_e682_d_b12, eq55_e682_d_b13, eq55_e682_d_b14, eq55_e682_d_b15,) = {
    if (!s.b[1851]) {
        let eq55_e679: f64 = (s.v[311] + s.v[263]);
        let eq55_e679_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);
        let eq55_e679_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);
        let eq55_e679_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);
        let eq55_e679_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);
        let eq55_e679_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);
        let eq55_e679_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);
        let eq55_e679_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);
        let eq55_e679_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);
        let eq55_e679_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);
        let eq55_e679_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);
        let eq55_e679_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);
        let eq55_e679_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);
        let eq55_e679_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);
        let eq55_e679_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);
        let eq55_e679_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);
        let eq55_e679_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);
        let eq55_e679_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);
        let eq55_e679_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);
        let eq55_e679_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);
        let eq55_e679_d_b0: f64 = (s.db[311][0] + s.db[263][0]);
        let eq55_e679_d_b1: f64 = (s.db[311][1] + s.db[263][1]);
        let eq55_e679_d_b2: f64 = (s.db[311][2] + s.db[263][2]);
        let eq55_e679_d_b3: f64 = (s.db[311][3] + s.db[263][3]);
        let eq55_e679_d_b4: f64 = (s.db[311][4] + s.db[263][4]);
        let eq55_e679_d_b5: f64 = (s.db[311][5] + s.db[263][5]);
        let eq55_e679_d_b6: f64 = (s.db[311][6] + s.db[263][6]);
        let eq55_e679_d_b7: f64 = (s.db[311][7] + s.db[263][7]);
        let eq55_e679_d_b8: f64 = (s.db[311][8] + s.db[263][8]);
        let eq55_e679_d_b9: f64 = (s.db[311][9] + s.db[263][9]);
        let eq55_e679_d_b10: f64 = (s.db[311][10] + s.db[263][10]);
        let eq55_e679_d_b11: f64 = (s.db[311][11] + s.db[263][11]);
        let eq55_e679_d_b12: f64 = (s.db[311][12] + s.db[263][12]);
        let eq55_e679_d_b13: f64 = (s.db[311][13] + s.db[263][13]);
        let eq55_e679_d_b14: f64 = (s.db[311][14] + s.db[263][14]);
        let eq55_e679_d_b15: f64 = (s.db[311][15] + s.db[263][15]);
        let eq55_e680: f64 = (p.p50 * eq55_e679);
        let eq55_e680_d_n0: f64 = (p.p50 * eq55_e679_d_n0);
        let eq55_e680_d_n1: f64 = (p.p50 * eq55_e679_d_n1);
        let eq55_e680_d_n2: f64 = (p.p50 * eq55_e679_d_n2);
        let eq55_e680_d_n3: f64 = (p.p50 * eq55_e679_d_n3);
        let eq55_e680_d_n4: f64 = (p.p50 * eq55_e679_d_n4);
        let eq55_e680_d_n5: f64 = (p.p50 * eq55_e679_d_n5);
        let eq55_e680_d_n6: f64 = (p.p50 * eq55_e679_d_n6);
        let eq55_e680_d_n7: f64 = (p.p50 * eq55_e679_d_n7);
        let eq55_e680_d_n8: f64 = (p.p50 * eq55_e679_d_n8);
        let eq55_e680_d_n9: f64 = (p.p50 * eq55_e679_d_n9);
        let eq55_e680_d_n10: f64 = (p.p50 * eq55_e679_d_n10);
        let eq55_e680_d_n11: f64 = (p.p50 * eq55_e679_d_n11);
        let eq55_e680_d_n12: f64 = (p.p50 * eq55_e679_d_n12);
        let eq55_e680_d_n13: f64 = (p.p50 * eq55_e679_d_n13);
        let eq55_e680_d_n14: f64 = (p.p50 * eq55_e679_d_n14);
        let eq55_e680_d_n15: f64 = (p.p50 * eq55_e679_d_n15);
        let eq55_e680_d_n16: f64 = (p.p50 * eq55_e679_d_n16);
        let eq55_e680_d_n17: f64 = (p.p50 * eq55_e679_d_n17);
        let eq55_e680_d_n18: f64 = (p.p50 * eq55_e679_d_n18);
        let eq55_e680_d_b0: f64 = (p.p50 * eq55_e679_d_b0);
        let eq55_e680_d_b1: f64 = (p.p50 * eq55_e679_d_b1);
        let eq55_e680_d_b2: f64 = (p.p50 * eq55_e679_d_b2);
        let eq55_e680_d_b3: f64 = (p.p50 * eq55_e679_d_b3);
        let eq55_e680_d_b4: f64 = (p.p50 * eq55_e679_d_b4);
        let eq55_e680_d_b5: f64 = (p.p50 * eq55_e679_d_b5);
        let eq55_e680_d_b6: f64 = (p.p50 * eq55_e679_d_b6);
        let eq55_e680_d_b7: f64 = (p.p50 * eq55_e679_d_b7);
        let eq55_e680_d_b8: f64 = (p.p50 * eq55_e679_d_b8);
        let eq55_e680_d_b9: f64 = (p.p50 * eq55_e679_d_b9);
        let eq55_e680_d_b10: f64 = (p.p50 * eq55_e679_d_b10);
        let eq55_e680_d_b11: f64 = (p.p50 * eq55_e679_d_b11);
        let eq55_e680_d_b12: f64 = (p.p50 * eq55_e679_d_b12);
        let eq55_e680_d_b13: f64 = (p.p50 * eq55_e679_d_b13);
        let eq55_e680_d_b14: f64 = (p.p50 * eq55_e679_d_b14);
        let eq55_e680_d_b15: f64 = (p.p50 * eq55_e679_d_b15);
        (eq55_e680, eq55_e680_d_n0, eq55_e680_d_n1, eq55_e680_d_n2, eq55_e680_d_n3, eq55_e680_d_n4, eq55_e680_d_n5, eq55_e680_d_n6, eq55_e680_d_n7, eq55_e680_d_n8, eq55_e680_d_n9, eq55_e680_d_n10, eq55_e680_d_n11, eq55_e680_d_n12, eq55_e680_d_n13, eq55_e680_d_n14, eq55_e680_d_n15, eq55_e680_d_n16, eq55_e680_d_n17, eq55_e680_d_n18, eq55_e680_d_b0, eq55_e680_d_b1, eq55_e680_d_b2, eq55_e680_d_b3, eq55_e680_d_b4, eq55_e680_d_b5, eq55_e680_d_b6, eq55_e680_d_b7, eq55_e680_d_b8, eq55_e680_d_b9, eq55_e680_d_b10, eq55_e680_d_b11, eq55_e680_d_b12, eq55_e680_d_b13, eq55_e680_d_b14, eq55_e680_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e682;
        let eq55_node_derivatives: [f64; 19] = [eq55_e682_d_n0, eq55_e682_d_n1, eq55_e682_d_n2, eq55_e682_d_n3, eq55_e682_d_n4, eq55_e682_d_n5, eq55_e682_d_n6, eq55_e682_d_n7, eq55_e682_d_n8, eq55_e682_d_n9, eq55_e682_d_n10, eq55_e682_d_n11, eq55_e682_d_n12, eq55_e682_d_n13, eq55_e682_d_n14, eq55_e682_d_n15, eq55_e682_d_n16, eq55_e682_d_n17, eq55_e682_d_n18];
        let eq55_branch_derivatives: [f64; 16] = [eq55_e682_d_b0, eq55_e682_d_b1, eq55_e682_d_b2, eq55_e682_d_b3, eq55_e682_d_b4, eq55_e682_d_b5, eq55_e682_d_b6, eq55_e682_d_b7, eq55_e682_d_b8, eq55_e682_d_b9, eq55_e682_d_b10, eq55_e682_d_b11, eq55_e682_d_b12, eq55_e682_d_b13, eq55_e682_d_b14, eq55_e682_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e691, eq56_e691_d_n0, eq56_e691_d_n1, eq56_e691_d_n2, eq56_e691_d_n3, eq56_e691_d_n4, eq56_e691_d_n5, eq56_e691_d_n6, eq56_e691_d_n7, eq56_e691_d_n8, eq56_e691_d_n9, eq56_e691_d_n10, eq56_e691_d_n11, eq56_e691_d_n12, eq56_e691_d_n13, eq56_e691_d_n14, eq56_e691_d_n15, eq56_e691_d_n16, eq56_e691_d_n17, eq56_e691_d_n18, eq56_e691_d_b0, eq56_e691_d_b1, eq56_e691_d_b2, eq56_e691_d_b3, eq56_e691_d_b4, eq56_e691_d_b5, eq56_e691_d_b6, eq56_e691_d_b7, eq56_e691_d_b8, eq56_e691_d_b9, eq56_e691_d_b10, eq56_e691_d_b11, eq56_e691_d_b12, eq56_e691_d_b13, eq56_e691_d_b14, eq56_e691_d_b15,) = {
    if (!s.b[1851]) {
        let eq56_e688: f64 = (s.v[312] + s.v[573]);
        let eq56_e688_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);
        let eq56_e688_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);
        let eq56_e688_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);
        let eq56_e688_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);
        let eq56_e688_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);
        let eq56_e688_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);
        let eq56_e688_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);
        let eq56_e688_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);
        let eq56_e688_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);
        let eq56_e688_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);
        let eq56_e688_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);
        let eq56_e688_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);
        let eq56_e688_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);
        let eq56_e688_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);
        let eq56_e688_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);
        let eq56_e688_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);
        let eq56_e688_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);
        let eq56_e688_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);
        let eq56_e688_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);
        let eq56_e688_d_b0: f64 = (s.db[312][0] + s.db[573][0]);
        let eq56_e688_d_b1: f64 = (s.db[312][1] + s.db[573][1]);
        let eq56_e688_d_b2: f64 = (s.db[312][2] + s.db[573][2]);
        let eq56_e688_d_b3: f64 = (s.db[312][3] + s.db[573][3]);
        let eq56_e688_d_b4: f64 = (s.db[312][4] + s.db[573][4]);
        let eq56_e688_d_b5: f64 = (s.db[312][5] + s.db[573][5]);
        let eq56_e688_d_b6: f64 = (s.db[312][6] + s.db[573][6]);
        let eq56_e688_d_b7: f64 = (s.db[312][7] + s.db[573][7]);
        let eq56_e688_d_b8: f64 = (s.db[312][8] + s.db[573][8]);
        let eq56_e688_d_b9: f64 = (s.db[312][9] + s.db[573][9]);
        let eq56_e688_d_b10: f64 = (s.db[312][10] + s.db[573][10]);
        let eq56_e688_d_b11: f64 = (s.db[312][11] + s.db[573][11]);
        let eq56_e688_d_b12: f64 = (s.db[312][12] + s.db[573][12]);
        let eq56_e688_d_b13: f64 = (s.db[312][13] + s.db[573][13]);
        let eq56_e688_d_b14: f64 = (s.db[312][14] + s.db[573][14]);
        let eq56_e688_d_b15: f64 = (s.db[312][15] + s.db[573][15]);
        let eq56_e689: f64 = (p.p50 * eq56_e688);
        let eq56_e689_d_n0: f64 = (p.p50 * eq56_e688_d_n0);
        let eq56_e689_d_n1: f64 = (p.p50 * eq56_e688_d_n1);
        let eq56_e689_d_n2: f64 = (p.p50 * eq56_e688_d_n2);
        let eq56_e689_d_n3: f64 = (p.p50 * eq56_e688_d_n3);
        let eq56_e689_d_n4: f64 = (p.p50 * eq56_e688_d_n4);
        let eq56_e689_d_n5: f64 = (p.p50 * eq56_e688_d_n5);
        let eq56_e689_d_n6: f64 = (p.p50 * eq56_e688_d_n6);
        let eq56_e689_d_n7: f64 = (p.p50 * eq56_e688_d_n7);
        let eq56_e689_d_n8: f64 = (p.p50 * eq56_e688_d_n8);
        let eq56_e689_d_n9: f64 = (p.p50 * eq56_e688_d_n9);
        let eq56_e689_d_n10: f64 = (p.p50 * eq56_e688_d_n10);
        let eq56_e689_d_n11: f64 = (p.p50 * eq56_e688_d_n11);
        let eq56_e689_d_n12: f64 = (p.p50 * eq56_e688_d_n12);
        let eq56_e689_d_n13: f64 = (p.p50 * eq56_e688_d_n13);
        let eq56_e689_d_n14: f64 = (p.p50 * eq56_e688_d_n14);
        let eq56_e689_d_n15: f64 = (p.p50 * eq56_e688_d_n15);
        let eq56_e689_d_n16: f64 = (p.p50 * eq56_e688_d_n16);
        let eq56_e689_d_n17: f64 = (p.p50 * eq56_e688_d_n17);
        let eq56_e689_d_n18: f64 = (p.p50 * eq56_e688_d_n18);
        let eq56_e689_d_b0: f64 = (p.p50 * eq56_e688_d_b0);
        let eq56_e689_d_b1: f64 = (p.p50 * eq56_e688_d_b1);
        let eq56_e689_d_b2: f64 = (p.p50 * eq56_e688_d_b2);
        let eq56_e689_d_b3: f64 = (p.p50 * eq56_e688_d_b3);
        let eq56_e689_d_b4: f64 = (p.p50 * eq56_e688_d_b4);
        let eq56_e689_d_b5: f64 = (p.p50 * eq56_e688_d_b5);
        let eq56_e689_d_b6: f64 = (p.p50 * eq56_e688_d_b6);
        let eq56_e689_d_b7: f64 = (p.p50 * eq56_e688_d_b7);
        let eq56_e689_d_b8: f64 = (p.p50 * eq56_e688_d_b8);
        let eq56_e689_d_b9: f64 = (p.p50 * eq56_e688_d_b9);
        let eq56_e689_d_b10: f64 = (p.p50 * eq56_e688_d_b10);
        let eq56_e689_d_b11: f64 = (p.p50 * eq56_e688_d_b11);
        let eq56_e689_d_b12: f64 = (p.p50 * eq56_e688_d_b12);
        let eq56_e689_d_b13: f64 = (p.p50 * eq56_e688_d_b13);
        let eq56_e689_d_b14: f64 = (p.p50 * eq56_e688_d_b14);
        let eq56_e689_d_b15: f64 = (p.p50 * eq56_e688_d_b15);
        (eq56_e689, eq56_e689_d_n0, eq56_e689_d_n1, eq56_e689_d_n2, eq56_e689_d_n3, eq56_e689_d_n4, eq56_e689_d_n5, eq56_e689_d_n6, eq56_e689_d_n7, eq56_e689_d_n8, eq56_e689_d_n9, eq56_e689_d_n10, eq56_e689_d_n11, eq56_e689_d_n12, eq56_e689_d_n13, eq56_e689_d_n14, eq56_e689_d_n15, eq56_e689_d_n16, eq56_e689_d_n17, eq56_e689_d_n18, eq56_e689_d_b0, eq56_e689_d_b1, eq56_e689_d_b2, eq56_e689_d_b3, eq56_e689_d_b4, eq56_e689_d_b5, eq56_e689_d_b6, eq56_e689_d_b7, eq56_e689_d_b8, eq56_e689_d_b9, eq56_e689_d_b10, eq56_e689_d_b11, eq56_e689_d_b12, eq56_e689_d_b13, eq56_e689_d_b14, eq56_e689_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e691;
        let eq56_node_derivatives: [f64; 19] = [eq56_e691_d_n0, eq56_e691_d_n1, eq56_e691_d_n2, eq56_e691_d_n3, eq56_e691_d_n4, eq56_e691_d_n5, eq56_e691_d_n6, eq56_e691_d_n7, eq56_e691_d_n8, eq56_e691_d_n9, eq56_e691_d_n10, eq56_e691_d_n11, eq56_e691_d_n12, eq56_e691_d_n13, eq56_e691_d_n14, eq56_e691_d_n15, eq56_e691_d_n16, eq56_e691_d_n17, eq56_e691_d_n18];
        let eq56_branch_derivatives: [f64; 16] = [eq56_e691_d_b0, eq56_e691_d_b1, eq56_e691_d_b2, eq56_e691_d_b3, eq56_e691_d_b4, eq56_e691_d_b5, eq56_e691_d_b6, eq56_e691_d_b7, eq56_e691_d_b8, eq56_e691_d_b9, eq56_e691_d_b10, eq56_e691_d_b11, eq56_e691_d_b12, eq56_e691_d_b13, eq56_e691_d_b14, eq56_e691_d_b15];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e696,) = {
    if (!s.b[1851]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq57_value: f64 = eq57_e696;
        stamper.stamp_potential_const_local(
            12,
            eq57_value,
        );
        let (eq58_e703, eq58_e703_d_n0, eq58_e703_d_n1, eq58_e703_d_n2, eq58_e703_d_n3, eq58_e703_d_n4, eq58_e703_d_n5, eq58_e703_d_n6, eq58_e703_d_n7, eq58_e703_d_n8, eq58_e703_d_n9, eq58_e703_d_n10, eq58_e703_d_n11, eq58_e703_d_n12, eq58_e703_d_n13, eq58_e703_d_n14, eq58_e703_d_n15, eq58_e703_d_n16, eq58_e703_d_n17, eq58_e703_d_n18, eq58_e703_d_b0, eq58_e703_d_b1, eq58_e703_d_b2, eq58_e703_d_b3, eq58_e703_d_b4, eq58_e703_d_b5, eq58_e703_d_b6, eq58_e703_d_b7, eq58_e703_d_b8, eq58_e703_d_b9, eq58_e703_d_b10, eq58_e703_d_b11, eq58_e703_d_b12, eq58_e703_d_b13, eq58_e703_d_b14, eq58_e703_d_b15,) = {
    if ((!s.b[1851]) && (p.p37 != 0.0)) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12], s.db[592][13], s.db[592][14], s.db[592][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e703;
        let eq58_node_derivatives: [f64; 19] = [eq58_e703_d_n0, eq58_e703_d_n1, eq58_e703_d_n2, eq58_e703_d_n3, eq58_e703_d_n4, eq58_e703_d_n5, eq58_e703_d_n6, eq58_e703_d_n7, eq58_e703_d_n8, eq58_e703_d_n9, eq58_e703_d_n10, eq58_e703_d_n11, eq58_e703_d_n12, eq58_e703_d_n13, eq58_e703_d_n14, eq58_e703_d_n15, eq58_e703_d_n16, eq58_e703_d_n17, eq58_e703_d_n18];
        let eq58_branch_derivatives: [f64; 16] = [eq58_e703_d_b0, eq58_e703_d_b1, eq58_e703_d_b2, eq58_e703_d_b3, eq58_e703_d_b4, eq58_e703_d_b5, eq58_e703_d_b6, eq58_e703_d_b7, eq58_e703_d_b8, eq58_e703_d_b9, eq58_e703_d_b10, eq58_e703_d_b11, eq58_e703_d_b12, eq58_e703_d_b13, eq58_e703_d_b14, eq58_e703_d_b15];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq60_e724, eq60_e724_d_n17,) = {
    if ((!s.b[1851]) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq60_e721);
        let eq60_e722_d_n17: f64 = (eq60_e719 * ddt_scale);
        (eq60_e722, eq60_e722_d_n17,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e724;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq60_value),
            17,
            multiplicity * (eq60_e724_d_n17),
        );
        let (eq61_e732,) = {
    if ((!s.b[1851]) && (p.p37 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e732;
        stamper.stamp_potential_const_local(
            13,
            eq61_value,
        );
        let (eq62_e739, eq62_e739_d_n0, eq62_e739_d_n1, eq62_e739_d_n2, eq62_e739_d_n3, eq62_e739_d_n4, eq62_e739_d_n5, eq62_e739_d_n6, eq62_e739_d_n7, eq62_e739_d_n8, eq62_e739_d_n9, eq62_e739_d_n10, eq62_e739_d_n11, eq62_e739_d_n12, eq62_e739_d_n13, eq62_e739_d_n14, eq62_e739_d_n15, eq62_e739_d_n16, eq62_e739_d_n17, eq62_e739_d_n18, eq62_e739_d_b0, eq62_e739_d_b1, eq62_e739_d_b2, eq62_e739_d_b3, eq62_e739_d_b4, eq62_e739_d_b5, eq62_e739_d_b6, eq62_e739_d_b7, eq62_e739_d_b8, eq62_e739_d_b9, eq62_e739_d_b10, eq62_e739_d_b11, eq62_e739_d_b12, eq62_e739_d_b13, eq62_e739_d_b14, eq62_e739_d_b15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        (s.v[574], s.dn[574][0], s.dn[574][1], s.dn[574][2], s.dn[574][3], s.dn[574][4], s.dn[574][5], s.dn[574][6], s.dn[574][7], s.dn[574][8], s.dn[574][9], s.dn[574][10], s.dn[574][11], s.dn[574][12], s.dn[574][13], s.dn[574][14], s.dn[574][15], s.dn[574][16], s.dn[574][17], s.dn[574][18], s.db[574][0], s.db[574][1], s.db[574][2], s.db[574][3], s.db[574][4], s.db[574][5], s.db[574][6], s.db[574][7], s.db[574][8], s.db[574][9], s.db[574][10], s.db[574][11], s.db[574][12], s.db[574][13], s.db[574][14], s.db[574][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e739;
        let eq62_node_derivatives: [f64; 19] = [eq62_e739_d_n0, eq62_e739_d_n1, eq62_e739_d_n2, eq62_e739_d_n3, eq62_e739_d_n4, eq62_e739_d_n5, eq62_e739_d_n6, eq62_e739_d_n7, eq62_e739_d_n8, eq62_e739_d_n9, eq62_e739_d_n10, eq62_e739_d_n11, eq62_e739_d_n12, eq62_e739_d_n13, eq62_e739_d_n14, eq62_e739_d_n15, eq62_e739_d_n16, eq62_e739_d_n17, eq62_e739_d_n18];
        let eq62_branch_derivatives: [f64; 16] = [eq62_e739_d_b0, eq62_e739_d_b1, eq62_e739_d_b2, eq62_e739_d_b3, eq62_e739_d_b4, eq62_e739_d_b5, eq62_e739_d_b6, eq62_e739_d_b7, eq62_e739_d_b8, eq62_e739_d_b9, eq62_e739_d_b10, eq62_e739_d_b11, eq62_e739_d_b12, eq62_e739_d_b13, eq62_e739_d_b14, eq62_e739_d_b15];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e746, eq63_e746_d_n0, eq63_e746_d_n1, eq63_e746_d_n2, eq63_e746_d_n3, eq63_e746_d_n4, eq63_e746_d_n5, eq63_e746_d_n6, eq63_e746_d_n7, eq63_e746_d_n8, eq63_e746_d_n9, eq63_e746_d_n10, eq63_e746_d_n11, eq63_e746_d_n12, eq63_e746_d_n13, eq63_e746_d_n14, eq63_e746_d_n15, eq63_e746_d_n16, eq63_e746_d_n17, eq63_e746_d_n18, eq63_e746_d_b0, eq63_e746_d_b1, eq63_e746_d_b2, eq63_e746_d_b3, eq63_e746_d_b4, eq63_e746_d_b5, eq63_e746_d_b6, eq63_e746_d_b7, eq63_e746_d_b8, eq63_e746_d_b9, eq63_e746_d_b10, eq63_e746_d_b11, eq63_e746_d_b12, eq63_e746_d_b13, eq63_e746_d_b14, eq63_e746_d_b15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        (s.v[575], s.dn[575][0], s.dn[575][1], s.dn[575][2], s.dn[575][3], s.dn[575][4], s.dn[575][5], s.dn[575][6], s.dn[575][7], s.dn[575][8], s.dn[575][9], s.dn[575][10], s.dn[575][11], s.dn[575][12], s.dn[575][13], s.dn[575][14], s.dn[575][15], s.dn[575][16], s.dn[575][17], s.dn[575][18], s.db[575][0], s.db[575][1], s.db[575][2], s.db[575][3], s.db[575][4], s.db[575][5], s.db[575][6], s.db[575][7], s.db[575][8], s.db[575][9], s.db[575][10], s.db[575][11], s.db[575][12], s.db[575][13], s.db[575][14], s.db[575][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e746;
        let eq63_node_derivatives: [f64; 19] = [eq63_e746_d_n0, eq63_e746_d_n1, eq63_e746_d_n2, eq63_e746_d_n3, eq63_e746_d_n4, eq63_e746_d_n5, eq63_e746_d_n6, eq63_e746_d_n7, eq63_e746_d_n8, eq63_e746_d_n9, eq63_e746_d_n10, eq63_e746_d_n11, eq63_e746_d_n12, eq63_e746_d_n13, eq63_e746_d_n14, eq63_e746_d_n15, eq63_e746_d_n16, eq63_e746_d_n17, eq63_e746_d_n18];
        let eq63_branch_derivatives: [f64; 16] = [eq63_e746_d_b0, eq63_e746_d_b1, eq63_e746_d_b2, eq63_e746_d_b3, eq63_e746_d_b4, eq63_e746_d_b5, eq63_e746_d_b6, eq63_e746_d_b7, eq63_e746_d_b8, eq63_e746_d_b9, eq63_e746_d_b10, eq63_e746_d_b11, eq63_e746_d_b12, eq63_e746_d_b13, eq63_e746_d_b14, eq63_e746_d_b15];
        stamper.stamp_current_dense_local(
            Some(16),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e753, eq64_e753_d_n0, eq64_e753_d_n1, eq64_e753_d_n2, eq64_e753_d_n3, eq64_e753_d_n4, eq64_e753_d_n5, eq64_e753_d_n6, eq64_e753_d_n7, eq64_e753_d_n8, eq64_e753_d_n9, eq64_e753_d_n10, eq64_e753_d_n11, eq64_e753_d_n12, eq64_e753_d_n13, eq64_e753_d_n14, eq64_e753_d_n15, eq64_e753_d_n16, eq64_e753_d_n17, eq64_e753_d_n18, eq64_e753_d_b0, eq64_e753_d_b1, eq64_e753_d_b2, eq64_e753_d_b3, eq64_e753_d_b4, eq64_e753_d_b5, eq64_e753_d_b6, eq64_e753_d_b7, eq64_e753_d_b8, eq64_e753_d_b9, eq64_e753_d_b10, eq64_e753_d_b11, eq64_e753_d_b12, eq64_e753_d_b13, eq64_e753_d_b14, eq64_e753_d_b15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12], s.db[583][13], s.db[583][14], s.db[583][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e753;
        let eq64_node_derivatives: [f64; 19] = [eq64_e753_d_n0, eq64_e753_d_n1, eq64_e753_d_n2, eq64_e753_d_n3, eq64_e753_d_n4, eq64_e753_d_n5, eq64_e753_d_n6, eq64_e753_d_n7, eq64_e753_d_n8, eq64_e753_d_n9, eq64_e753_d_n10, eq64_e753_d_n11, eq64_e753_d_n12, eq64_e753_d_n13, eq64_e753_d_n14, eq64_e753_d_n15, eq64_e753_d_n16, eq64_e753_d_n17, eq64_e753_d_n18];
        let eq64_branch_derivatives: [f64; 16] = [eq64_e753_d_b0, eq64_e753_d_b1, eq64_e753_d_b2, eq64_e753_d_b3, eq64_e753_d_b4, eq64_e753_d_b5, eq64_e753_d_b6, eq64_e753_d_b7, eq64_e753_d_b8, eq64_e753_d_b9, eq64_e753_d_b10, eq64_e753_d_b11, eq64_e753_d_b12, eq64_e753_d_b13, eq64_e753_d_b14, eq64_e753_d_b15];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq64_value),
            &eq64_node_derivatives,
            &eq64_branch_derivatives,
            multiplicity,
        );
        let (eq68_e792, eq68_e792_d_n15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq68_e789);
        let eq68_e790_d_n15: f64 = (eq68_e787 * ddt_scale);
        (eq68_e790, eq68_e790_d_n15,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e792;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq68_value),
            15,
            multiplicity * (eq68_e792_d_n15),
        );
        let (eq69_e804, eq69_e804_d_n16,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq69_e801);
        let eq69_e802_d_n16: f64 = (eq69_e799 * ddt_scale);
        (eq69_e802, eq69_e802_d_n16,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e804;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq69_value),
            16,
            multiplicity * (eq69_e804_d_n16),
        );
        let (eq70_e816, eq70_e816_d_n13,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq70_e813);
        let eq70_e814_d_n13: f64 = (eq70_e811 * ddt_scale);
        (eq70_e814, eq70_e814_d_n13,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e816;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq70_value),
            13,
            multiplicity * (eq70_e816_d_n13),
        );
        let (eq71_e824,) = {
    if ((!s.b[1851]) && (p.p34 == 0.0)) {
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
    if ((!s.b[1851]) && (p.p34 == 0.0)) {
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
        let (eq73_e840,) = {
    if ((!s.b[1851]) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e840;
        stamper.stamp_potential_const_local(
            16,
            eq73_value,
        );
        let (eq74_e844,) = {
    if s.b[1853] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e844;
        stamper.stamp_potential_const_local(
            17,
            eq74_value,
        );
        let (eq75_e849,) = {
    if (!s.b[1853]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e849;
        stamper.stamp_potential_const_local(
            18,
            eq75_value,
        );
        let (eq76_e854,) = {
    if (!s.b[1853]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq76_value: f64 = eq76_e854;
        stamper.stamp_potential_const_local(
            19,
            eq76_value,
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
        let eq11_e367_q: f64 = s.v[594];
        let eq11_e368: f64 = (p.p50 * s.v[594]);
        let eq11_e368_d_n0: f64 = (p.p50 * s.dn[594][0]);
        let eq11_e368_d_n1: f64 = (p.p50 * s.dn[594][1]);
        let eq11_e368_d_n2: f64 = (p.p50 * s.dn[594][2]);
        let eq11_e368_d_n3: f64 = (p.p50 * s.dn[594][3]);
        let eq11_e368_d_n4: f64 = (p.p50 * s.dn[594][4]);
        let eq11_e368_d_n5: f64 = (p.p50 * s.dn[594][5]);
        let eq11_e368_d_n6: f64 = (p.p50 * s.dn[594][6]);
        let eq11_e368_d_n7: f64 = (p.p50 * s.dn[594][7]);
        let eq11_e368_d_n8: f64 = (p.p50 * s.dn[594][8]);
        let eq11_e368_d_n9: f64 = (p.p50 * s.dn[594][9]);
        let eq11_e368_d_n10: f64 = (p.p50 * s.dn[594][10]);
        let eq11_e368_d_n11: f64 = (p.p50 * s.dn[594][11]);
        let eq11_e368_d_n12: f64 = (p.p50 * s.dn[594][12]);
        let eq11_e368_d_n13: f64 = (p.p50 * s.dn[594][13]);
        let eq11_e368_d_n14: f64 = (p.p50 * s.dn[594][14]);
        let eq11_e368_d_n15: f64 = (p.p50 * s.dn[594][15]);
        let eq11_e368_d_n16: f64 = (p.p50 * s.dn[594][16]);
        let eq11_e368_d_n17: f64 = (p.p50 * s.dn[594][17]);
        let eq11_e368_d_n18: f64 = (p.p50 * s.dn[594][18]);
        let eq11_e368_d_b0: f64 = (p.p50 * s.db[594][0]);
        let eq11_e368_d_b1: f64 = (p.p50 * s.db[594][1]);
        let eq11_e368_d_b2: f64 = (p.p50 * s.db[594][2]);
        let eq11_e368_d_b3: f64 = (p.p50 * s.db[594][3]);
        let eq11_e368_d_b4: f64 = (p.p50 * s.db[594][4]);
        let eq11_e368_d_b5: f64 = (p.p50 * s.db[594][5]);
        let eq11_e368_d_b6: f64 = (p.p50 * s.db[594][6]);
        let eq11_e368_d_b7: f64 = (p.p50 * s.db[594][7]);
        let eq11_e368_d_b8: f64 = (p.p50 * s.db[594][8]);
        let eq11_e368_d_b9: f64 = (p.p50 * s.db[594][9]);
        let eq11_e368_d_b10: f64 = (p.p50 * s.db[594][10]);
        let eq11_e368_d_b11: f64 = (p.p50 * s.db[594][11]);
        let eq11_e368_d_b12: f64 = (p.p50 * s.db[594][12]);
        let eq11_e368_d_b13: f64 = (p.p50 * s.db[594][13]);
        let eq11_e368_d_b14: f64 = (p.p50 * s.db[594][14]);
        let eq11_e368_d_b15: f64 = (p.p50 * s.db[594][15]);
        let eq11_e368_q: f64 = (p.p50 * eq11_e367_q);
        let eq11_e368_q_d_n0: f64 = (p.p50 * s.dn[594][0]);
        let eq11_e368_q_d_n1: f64 = (p.p50 * s.dn[594][1]);
        let eq11_e368_q_d_n2: f64 = (p.p50 * s.dn[594][2]);
        let eq11_e368_q_d_n3: f64 = (p.p50 * s.dn[594][3]);
        let eq11_e368_q_d_n4: f64 = (p.p50 * s.dn[594][4]);
        let eq11_e368_q_d_n5: f64 = (p.p50 * s.dn[594][5]);
        let eq11_e368_q_d_n6: f64 = (p.p50 * s.dn[594][6]);
        let eq11_e368_q_d_n7: f64 = (p.p50 * s.dn[594][7]);
        let eq11_e368_q_d_n8: f64 = (p.p50 * s.dn[594][8]);
        let eq11_e368_q_d_n9: f64 = (p.p50 * s.dn[594][9]);
        let eq11_e368_q_d_n10: f64 = (p.p50 * s.dn[594][10]);
        let eq11_e368_q_d_n11: f64 = (p.p50 * s.dn[594][11]);
        let eq11_e368_q_d_n12: f64 = (p.p50 * s.dn[594][12]);
        let eq11_e368_q_d_n13: f64 = (p.p50 * s.dn[594][13]);
        let eq11_e368_q_d_n14: f64 = (p.p50 * s.dn[594][14]);
        let eq11_e368_q_d_n15: f64 = (p.p50 * s.dn[594][15]);
        let eq11_e368_q_d_n16: f64 = (p.p50 * s.dn[594][16]);
        let eq11_e368_q_d_n17: f64 = (p.p50 * s.dn[594][17]);
        let eq11_e368_q_d_n18: f64 = (p.p50 * s.dn[594][18]);
        let eq11_e368_q_d_b0: f64 = (p.p50 * s.db[594][0]);
        let eq11_e368_q_d_b1: f64 = (p.p50 * s.db[594][1]);
        let eq11_e368_q_d_b2: f64 = (p.p50 * s.db[594][2]);
        let eq11_e368_q_d_b3: f64 = (p.p50 * s.db[594][3]);
        let eq11_e368_q_d_b4: f64 = (p.p50 * s.db[594][4]);
        let eq11_e368_q_d_b5: f64 = (p.p50 * s.db[594][5]);
        let eq11_e368_q_d_b6: f64 = (p.p50 * s.db[594][6]);
        let eq11_e368_q_d_b7: f64 = (p.p50 * s.db[594][7]);
        let eq11_e368_q_d_b8: f64 = (p.p50 * s.db[594][8]);
        let eq11_e368_q_d_b9: f64 = (p.p50 * s.db[594][9]);
        let eq11_e368_q_d_b10: f64 = (p.p50 * s.db[594][10]);
        let eq11_e368_q_d_b11: f64 = (p.p50 * s.db[594][11]);
        let eq11_e368_q_d_b12: f64 = (p.p50 * s.db[594][12]);
        let eq11_e368_q_d_b13: f64 = (p.p50 * s.db[594][13]);
        let eq11_e368_q_d_b14: f64 = (p.p50 * s.db[594][14]);
        let eq11_e368_q_d_b15: f64 = (p.p50 * s.db[594][15]);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e368_q_d_n0, eq11_e368_q_d_n1, eq11_e368_q_d_n2, eq11_e368_q_d_n3, eq11_e368_q_d_n4, eq11_e368_q_d_n5, eq11_e368_q_d_n6, eq11_e368_q_d_n7, eq11_e368_q_d_n8, eq11_e368_q_d_n9, eq11_e368_q_d_n10, eq11_e368_q_d_n11, eq11_e368_q_d_n12, eq11_e368_q_d_n13, eq11_e368_q_d_n14, eq11_e368_q_d_n15, eq11_e368_q_d_n16, eq11_e368_q_d_n17, eq11_e368_q_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 16] = [eq11_e368_q_d_b0, eq11_e368_q_d_b1, eq11_e368_q_d_b2, eq11_e368_q_d_b3, eq11_e368_q_d_b4, eq11_e368_q_d_b5, eq11_e368_q_d_b6, eq11_e368_q_d_b7, eq11_e368_q_d_b8, eq11_e368_q_d_b9, eq11_e368_q_d_b10, eq11_e368_q_d_b11, eq11_e368_q_d_b12, eq11_e368_q_d_b13, eq11_e368_q_d_b14, eq11_e368_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e371_q: f64 = s.v[198];
        let eq12_e372: f64 = (p.p50 * s.v[198]);
        let eq12_e372_d_n0: f64 = (p.p50 * s.dn[198][0]);
        let eq12_e372_d_n1: f64 = (p.p50 * s.dn[198][1]);
        let eq12_e372_d_n2: f64 = (p.p50 * s.dn[198][2]);
        let eq12_e372_d_n3: f64 = (p.p50 * s.dn[198][3]);
        let eq12_e372_d_n4: f64 = (p.p50 * s.dn[198][4]);
        let eq12_e372_d_n5: f64 = (p.p50 * s.dn[198][5]);
        let eq12_e372_d_n6: f64 = (p.p50 * s.dn[198][6]);
        let eq12_e372_d_n7: f64 = (p.p50 * s.dn[198][7]);
        let eq12_e372_d_n8: f64 = (p.p50 * s.dn[198][8]);
        let eq12_e372_d_n9: f64 = (p.p50 * s.dn[198][9]);
        let eq12_e372_d_n10: f64 = (p.p50 * s.dn[198][10]);
        let eq12_e372_d_n11: f64 = (p.p50 * s.dn[198][11]);
        let eq12_e372_d_n12: f64 = (p.p50 * s.dn[198][12]);
        let eq12_e372_d_n13: f64 = (p.p50 * s.dn[198][13]);
        let eq12_e372_d_n14: f64 = (p.p50 * s.dn[198][14]);
        let eq12_e372_d_n15: f64 = (p.p50 * s.dn[198][15]);
        let eq12_e372_d_n16: f64 = (p.p50 * s.dn[198][16]);
        let eq12_e372_d_n17: f64 = (p.p50 * s.dn[198][17]);
        let eq12_e372_d_n18: f64 = (p.p50 * s.dn[198][18]);
        let eq12_e372_d_b0: f64 = (p.p50 * s.db[198][0]);
        let eq12_e372_d_b1: f64 = (p.p50 * s.db[198][1]);
        let eq12_e372_d_b2: f64 = (p.p50 * s.db[198][2]);
        let eq12_e372_d_b3: f64 = (p.p50 * s.db[198][3]);
        let eq12_e372_d_b4: f64 = (p.p50 * s.db[198][4]);
        let eq12_e372_d_b5: f64 = (p.p50 * s.db[198][5]);
        let eq12_e372_d_b6: f64 = (p.p50 * s.db[198][6]);
        let eq12_e372_d_b7: f64 = (p.p50 * s.db[198][7]);
        let eq12_e372_d_b8: f64 = (p.p50 * s.db[198][8]);
        let eq12_e372_d_b9: f64 = (p.p50 * s.db[198][9]);
        let eq12_e372_d_b10: f64 = (p.p50 * s.db[198][10]);
        let eq12_e372_d_b11: f64 = (p.p50 * s.db[198][11]);
        let eq12_e372_d_b12: f64 = (p.p50 * s.db[198][12]);
        let eq12_e372_d_b13: f64 = (p.p50 * s.db[198][13]);
        let eq12_e372_d_b14: f64 = (p.p50 * s.db[198][14]);
        let eq12_e372_d_b15: f64 = (p.p50 * s.db[198][15]);
        let eq12_e372_q: f64 = (p.p50 * eq12_e371_q);
        let eq12_e372_q_d_n0: f64 = (p.p50 * s.dn[198][0]);
        let eq12_e372_q_d_n1: f64 = (p.p50 * s.dn[198][1]);
        let eq12_e372_q_d_n2: f64 = (p.p50 * s.dn[198][2]);
        let eq12_e372_q_d_n3: f64 = (p.p50 * s.dn[198][3]);
        let eq12_e372_q_d_n4: f64 = (p.p50 * s.dn[198][4]);
        let eq12_e372_q_d_n5: f64 = (p.p50 * s.dn[198][5]);
        let eq12_e372_q_d_n6: f64 = (p.p50 * s.dn[198][6]);
        let eq12_e372_q_d_n7: f64 = (p.p50 * s.dn[198][7]);
        let eq12_e372_q_d_n8: f64 = (p.p50 * s.dn[198][8]);
        let eq12_e372_q_d_n9: f64 = (p.p50 * s.dn[198][9]);
        let eq12_e372_q_d_n10: f64 = (p.p50 * s.dn[198][10]);
        let eq12_e372_q_d_n11: f64 = (p.p50 * s.dn[198][11]);
        let eq12_e372_q_d_n12: f64 = (p.p50 * s.dn[198][12]);
        let eq12_e372_q_d_n13: f64 = (p.p50 * s.dn[198][13]);
        let eq12_e372_q_d_n14: f64 = (p.p50 * s.dn[198][14]);
        let eq12_e372_q_d_n15: f64 = (p.p50 * s.dn[198][15]);
        let eq12_e372_q_d_n16: f64 = (p.p50 * s.dn[198][16]);
        let eq12_e372_q_d_n17: f64 = (p.p50 * s.dn[198][17]);
        let eq12_e372_q_d_n18: f64 = (p.p50 * s.dn[198][18]);
        let eq12_e372_q_d_b0: f64 = (p.p50 * s.db[198][0]);
        let eq12_e372_q_d_b1: f64 = (p.p50 * s.db[198][1]);
        let eq12_e372_q_d_b2: f64 = (p.p50 * s.db[198][2]);
        let eq12_e372_q_d_b3: f64 = (p.p50 * s.db[198][3]);
        let eq12_e372_q_d_b4: f64 = (p.p50 * s.db[198][4]);
        let eq12_e372_q_d_b5: f64 = (p.p50 * s.db[198][5]);
        let eq12_e372_q_d_b6: f64 = (p.p50 * s.db[198][6]);
        let eq12_e372_q_d_b7: f64 = (p.p50 * s.db[198][7]);
        let eq12_e372_q_d_b8: f64 = (p.p50 * s.db[198][8]);
        let eq12_e372_q_d_b9: f64 = (p.p50 * s.db[198][9]);
        let eq12_e372_q_d_b10: f64 = (p.p50 * s.db[198][10]);
        let eq12_e372_q_d_b11: f64 = (p.p50 * s.db[198][11]);
        let eq12_e372_q_d_b12: f64 = (p.p50 * s.db[198][12]);
        let eq12_e372_q_d_b13: f64 = (p.p50 * s.db[198][13]);
        let eq12_e372_q_d_b14: f64 = (p.p50 * s.db[198][14]);
        let eq12_e372_q_d_b15: f64 = (p.p50 * s.db[198][15]);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e372_q_d_n0, eq12_e372_q_d_n1, eq12_e372_q_d_n2, eq12_e372_q_d_n3, eq12_e372_q_d_n4, eq12_e372_q_d_n5, eq12_e372_q_d_n6, eq12_e372_q_d_n7, eq12_e372_q_d_n8, eq12_e372_q_d_n9, eq12_e372_q_d_n10, eq12_e372_q_d_n11, eq12_e372_q_d_n12, eq12_e372_q_d_n13, eq12_e372_q_d_n14, eq12_e372_q_d_n15, eq12_e372_q_d_n16, eq12_e372_q_d_n17, eq12_e372_q_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 16] = [eq12_e372_q_d_b0, eq12_e372_q_d_b1, eq12_e372_q_d_b2, eq12_e372_q_d_b3, eq12_e372_q_d_b4, eq12_e372_q_d_b5, eq12_e372_q_d_b6, eq12_e372_q_d_b7, eq12_e372_q_d_b8, eq12_e372_q_d_b9, eq12_e372_q_d_b10, eq12_e372_q_d_b11, eq12_e372_q_d_b12, eq12_e372_q_d_b13, eq12_e372_q_d_b14, eq12_e372_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e375_q: f64 = s.v[196];
        let eq13_e376: f64 = (p.p50 * s.v[196]);
        let eq13_e376_d_n0: f64 = (p.p50 * s.dn[196][0]);
        let eq13_e376_d_n1: f64 = (p.p50 * s.dn[196][1]);
        let eq13_e376_d_n2: f64 = (p.p50 * s.dn[196][2]);
        let eq13_e376_d_n3: f64 = (p.p50 * s.dn[196][3]);
        let eq13_e376_d_n4: f64 = (p.p50 * s.dn[196][4]);
        let eq13_e376_d_n5: f64 = (p.p50 * s.dn[196][5]);
        let eq13_e376_d_n6: f64 = (p.p50 * s.dn[196][6]);
        let eq13_e376_d_n7: f64 = (p.p50 * s.dn[196][7]);
        let eq13_e376_d_n8: f64 = (p.p50 * s.dn[196][8]);
        let eq13_e376_d_n9: f64 = (p.p50 * s.dn[196][9]);
        let eq13_e376_d_n10: f64 = (p.p50 * s.dn[196][10]);
        let eq13_e376_d_n11: f64 = (p.p50 * s.dn[196][11]);
        let eq13_e376_d_n12: f64 = (p.p50 * s.dn[196][12]);
        let eq13_e376_d_n13: f64 = (p.p50 * s.dn[196][13]);
        let eq13_e376_d_n14: f64 = (p.p50 * s.dn[196][14]);
        let eq13_e376_d_n15: f64 = (p.p50 * s.dn[196][15]);
        let eq13_e376_d_n16: f64 = (p.p50 * s.dn[196][16]);
        let eq13_e376_d_n17: f64 = (p.p50 * s.dn[196][17]);
        let eq13_e376_d_n18: f64 = (p.p50 * s.dn[196][18]);
        let eq13_e376_d_b0: f64 = (p.p50 * s.db[196][0]);
        let eq13_e376_d_b1: f64 = (p.p50 * s.db[196][1]);
        let eq13_e376_d_b2: f64 = (p.p50 * s.db[196][2]);
        let eq13_e376_d_b3: f64 = (p.p50 * s.db[196][3]);
        let eq13_e376_d_b4: f64 = (p.p50 * s.db[196][4]);
        let eq13_e376_d_b5: f64 = (p.p50 * s.db[196][5]);
        let eq13_e376_d_b6: f64 = (p.p50 * s.db[196][6]);
        let eq13_e376_d_b7: f64 = (p.p50 * s.db[196][7]);
        let eq13_e376_d_b8: f64 = (p.p50 * s.db[196][8]);
        let eq13_e376_d_b9: f64 = (p.p50 * s.db[196][9]);
        let eq13_e376_d_b10: f64 = (p.p50 * s.db[196][10]);
        let eq13_e376_d_b11: f64 = (p.p50 * s.db[196][11]);
        let eq13_e376_d_b12: f64 = (p.p50 * s.db[196][12]);
        let eq13_e376_d_b13: f64 = (p.p50 * s.db[196][13]);
        let eq13_e376_d_b14: f64 = (p.p50 * s.db[196][14]);
        let eq13_e376_d_b15: f64 = (p.p50 * s.db[196][15]);
        let eq13_e376_q: f64 = (p.p50 * eq13_e375_q);
        let eq13_e376_q_d_n0: f64 = (p.p50 * s.dn[196][0]);
        let eq13_e376_q_d_n1: f64 = (p.p50 * s.dn[196][1]);
        let eq13_e376_q_d_n2: f64 = (p.p50 * s.dn[196][2]);
        let eq13_e376_q_d_n3: f64 = (p.p50 * s.dn[196][3]);
        let eq13_e376_q_d_n4: f64 = (p.p50 * s.dn[196][4]);
        let eq13_e376_q_d_n5: f64 = (p.p50 * s.dn[196][5]);
        let eq13_e376_q_d_n6: f64 = (p.p50 * s.dn[196][6]);
        let eq13_e376_q_d_n7: f64 = (p.p50 * s.dn[196][7]);
        let eq13_e376_q_d_n8: f64 = (p.p50 * s.dn[196][8]);
        let eq13_e376_q_d_n9: f64 = (p.p50 * s.dn[196][9]);
        let eq13_e376_q_d_n10: f64 = (p.p50 * s.dn[196][10]);
        let eq13_e376_q_d_n11: f64 = (p.p50 * s.dn[196][11]);
        let eq13_e376_q_d_n12: f64 = (p.p50 * s.dn[196][12]);
        let eq13_e376_q_d_n13: f64 = (p.p50 * s.dn[196][13]);
        let eq13_e376_q_d_n14: f64 = (p.p50 * s.dn[196][14]);
        let eq13_e376_q_d_n15: f64 = (p.p50 * s.dn[196][15]);
        let eq13_e376_q_d_n16: f64 = (p.p50 * s.dn[196][16]);
        let eq13_e376_q_d_n17: f64 = (p.p50 * s.dn[196][17]);
        let eq13_e376_q_d_n18: f64 = (p.p50 * s.dn[196][18]);
        let eq13_e376_q_d_b0: f64 = (p.p50 * s.db[196][0]);
        let eq13_e376_q_d_b1: f64 = (p.p50 * s.db[196][1]);
        let eq13_e376_q_d_b2: f64 = (p.p50 * s.db[196][2]);
        let eq13_e376_q_d_b3: f64 = (p.p50 * s.db[196][3]);
        let eq13_e376_q_d_b4: f64 = (p.p50 * s.db[196][4]);
        let eq13_e376_q_d_b5: f64 = (p.p50 * s.db[196][5]);
        let eq13_e376_q_d_b6: f64 = (p.p50 * s.db[196][6]);
        let eq13_e376_q_d_b7: f64 = (p.p50 * s.db[196][7]);
        let eq13_e376_q_d_b8: f64 = (p.p50 * s.db[196][8]);
        let eq13_e376_q_d_b9: f64 = (p.p50 * s.db[196][9]);
        let eq13_e376_q_d_b10: f64 = (p.p50 * s.db[196][10]);
        let eq13_e376_q_d_b11: f64 = (p.p50 * s.db[196][11]);
        let eq13_e376_q_d_b12: f64 = (p.p50 * s.db[196][12]);
        let eq13_e376_q_d_b13: f64 = (p.p50 * s.db[196][13]);
        let eq13_e376_q_d_b14: f64 = (p.p50 * s.db[196][14]);
        let eq13_e376_q_d_b15: f64 = (p.p50 * s.db[196][15]);
        let eq13_reactive_node_derivatives: [f64; 19] = [eq13_e376_q_d_n0, eq13_e376_q_d_n1, eq13_e376_q_d_n2, eq13_e376_q_d_n3, eq13_e376_q_d_n4, eq13_e376_q_d_n5, eq13_e376_q_d_n6, eq13_e376_q_d_n7, eq13_e376_q_d_n8, eq13_e376_q_d_n9, eq13_e376_q_d_n10, eq13_e376_q_d_n11, eq13_e376_q_d_n12, eq13_e376_q_d_n13, eq13_e376_q_d_n14, eq13_e376_q_d_n15, eq13_e376_q_d_n16, eq13_e376_q_d_n17, eq13_e376_q_d_n18];
        let eq13_reactive_branch_derivatives: [f64; 16] = [eq13_e376_q_d_b0, eq13_e376_q_d_b1, eq13_e376_q_d_b2, eq13_e376_q_d_b3, eq13_e376_q_d_b4, eq13_e376_q_d_b5, eq13_e376_q_d_b6, eq13_e376_q_d_b7, eq13_e376_q_d_b8, eq13_e376_q_d_b9, eq13_e376_q_d_b10, eq13_e376_q_d_b11, eq13_e376_q_d_b12, eq13_e376_q_d_b13, eq13_e376_q_d_b14, eq13_e376_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e405: f64 = ((nv14 - 0.0) * s.v[617]);
        let eq19_e405_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);
        let eq19_e405_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);
        let eq19_e405_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);
        let eq19_e405_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);
        let eq19_e405_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);
        let eq19_e405_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);
        let eq19_e405_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);
        let eq19_e405_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);
        let eq19_e405_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);
        let eq19_e405_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);
        let eq19_e405_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);
        let eq19_e405_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);
        let eq19_e405_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);
        let eq19_e405_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);
        let eq19_e405_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));
        let eq19_e405_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);
        let eq19_e405_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);
        let eq19_e405_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);
        let eq19_e405_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);
        let eq19_e405_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);
        let eq19_e405_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);
        let eq19_e405_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);
        let eq19_e405_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);
        let eq19_e405_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);
        let eq19_e405_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);
        let eq19_e405_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);
        let eq19_e405_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);
        let eq19_e405_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);
        let eq19_e405_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);
        let eq19_e405_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);
        let eq19_e405_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);
        let eq19_e405_d_b12: f64 = ((nv14 - 0.0) * s.db[617][12]);
        let eq19_e405_d_b13: f64 = ((nv14 - 0.0) * s.db[617][13]);
        let eq19_e405_d_b14: f64 = ((nv14 - 0.0) * s.db[617][14]);
        let eq19_e405_d_b15: f64 = ((nv14 - 0.0) * s.db[617][15]);
        let eq19_e406_q: f64 = eq19_e405;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e405_d_n0, eq19_e405_d_n1, eq19_e405_d_n2, eq19_e405_d_n3, eq19_e405_d_n4, eq19_e405_d_n5, eq19_e405_d_n6, eq19_e405_d_n7, eq19_e405_d_n8, eq19_e405_d_n9, eq19_e405_d_n10, eq19_e405_d_n11, eq19_e405_d_n12, eq19_e405_d_n13, eq19_e405_d_n14, eq19_e405_d_n15, eq19_e405_d_n16, eq19_e405_d_n17, eq19_e405_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 16] = [eq19_e405_d_b0, eq19_e405_d_b1, eq19_e405_d_b2, eq19_e405_d_b3, eq19_e405_d_b4, eq19_e405_d_b5, eq19_e405_d_b6, eq19_e405_d_b7, eq19_e405_d_b8, eq19_e405_d_b9, eq19_e405_d_b10, eq19_e405_d_b11, eq19_e405_d_b12, eq19_e405_d_b13, eq19_e405_d_b14, eq19_e405_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e409: f64 = ((nv14 - 0.0) * s.v[618]);
        let eq20_e409_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);
        let eq20_e409_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);
        let eq20_e409_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);
        let eq20_e409_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);
        let eq20_e409_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);
        let eq20_e409_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);
        let eq20_e409_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);
        let eq20_e409_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);
        let eq20_e409_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);
        let eq20_e409_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);
        let eq20_e409_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);
        let eq20_e409_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);
        let eq20_e409_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);
        let eq20_e409_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);
        let eq20_e409_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));
        let eq20_e409_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);
        let eq20_e409_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);
        let eq20_e409_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);
        let eq20_e409_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);
        let eq20_e409_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);
        let eq20_e409_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);
        let eq20_e409_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);
        let eq20_e409_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);
        let eq20_e409_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);
        let eq20_e409_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);
        let eq20_e409_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);
        let eq20_e409_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);
        let eq20_e409_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);
        let eq20_e409_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);
        let eq20_e409_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);
        let eq20_e409_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);
        let eq20_e409_d_b12: f64 = ((nv14 - 0.0) * s.db[618][12]);
        let eq20_e409_d_b13: f64 = ((nv14 - 0.0) * s.db[618][13]);
        let eq20_e409_d_b14: f64 = ((nv14 - 0.0) * s.db[618][14]);
        let eq20_e409_d_b15: f64 = ((nv14 - 0.0) * s.db[618][15]);
        let eq20_e410_q: f64 = eq20_e409;
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e409_d_n0, eq20_e409_d_n1, eq20_e409_d_n2, eq20_e409_d_n3, eq20_e409_d_n4, eq20_e409_d_n5, eq20_e409_d_n6, eq20_e409_d_n7, eq20_e409_d_n8, eq20_e409_d_n9, eq20_e409_d_n10, eq20_e409_d_n11, eq20_e409_d_n12, eq20_e409_d_n13, eq20_e409_d_n14, eq20_e409_d_n15, eq20_e409_d_n16, eq20_e409_d_n17, eq20_e409_d_n18];
        let eq20_reactive_branch_derivatives: [f64; 16] = [eq20_e409_d_b0, eq20_e409_d_b1, eq20_e409_d_b2, eq20_e409_d_b3, eq20_e409_d_b4, eq20_e409_d_b5, eq20_e409_d_b6, eq20_e409_d_b7, eq20_e409_d_b8, eq20_e409_d_b9, eq20_e409_d_b10, eq20_e409_d_b11, eq20_e409_d_b12, eq20_e409_d_b13, eq20_e409_d_b14, eq20_e409_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq31_e491, eq31_e491_d_n0, eq31_e491_d_n1, eq31_e491_d_n2, eq31_e491_d_n3, eq31_e491_d_n4, eq31_e491_d_n5, eq31_e491_d_n6, eq31_e491_d_n7, eq31_e491_d_n8, eq31_e491_d_n9, eq31_e491_d_n10, eq31_e491_d_n11, eq31_e491_d_n12, eq31_e491_d_n13, eq31_e491_d_n14, eq31_e491_d_n15, eq31_e491_d_n16, eq31_e491_d_n17, eq31_e491_d_n18, eq31_e491_d_b0, eq31_e491_d_b1, eq31_e491_d_b2, eq31_e491_d_b3, eq31_e491_d_b4, eq31_e491_d_b5, eq31_e491_d_b6, eq31_e491_d_b7, eq31_e491_d_b8, eq31_e491_d_b9, eq31_e491_d_b10, eq31_e491_d_b11, eq31_e491_d_b12, eq31_e491_d_b13, eq31_e491_d_b14, eq31_e491_d_b15, eq31_e491_q, eq31_e491_q_d_n0, eq31_e491_q_d_n1, eq31_e491_q_d_n2, eq31_e491_q_d_n3, eq31_e491_q_d_n4, eq31_e491_q_d_n5, eq31_e491_q_d_n6, eq31_e491_q_d_n7, eq31_e491_q_d_n8, eq31_e491_q_d_n9, eq31_e491_q_d_n10, eq31_e491_q_d_n11, eq31_e491_q_d_n12, eq31_e491_q_d_n13, eq31_e491_q_d_n14, eq31_e491_q_d_n15, eq31_e491_q_d_n16, eq31_e491_q_d_n17, eq31_e491_q_d_n18, eq31_e491_q_d_b0, eq31_e491_q_d_b1, eq31_e491_q_d_b2, eq31_e491_q_d_b3, eq31_e491_q_d_b4, eq31_e491_q_d_b5, eq31_e491_q_d_b6, eq31_e491_q_d_b7, eq31_e491_q_d_b8, eq31_e491_q_d_b9, eq31_e491_q_d_b10, eq31_e491_q_d_b11, eq31_e491_q_d_b12, eq31_e491_q_d_b13, eq31_e491_q_d_b14, eq31_e491_q_d_b15,) = {
    if s.b[1850] {
        let eq31_e488: f64 = (s.v[563] * (nv10 - 0.0));
        let eq31_e488_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));
        let eq31_e488_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));
        let eq31_e488_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));
        let eq31_e488_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));
        let eq31_e488_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));
        let eq31_e488_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));
        let eq31_e488_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));
        let eq31_e488_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));
        let eq31_e488_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));
        let eq31_e488_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));
        let eq31_e488_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);
        let eq31_e488_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));
        let eq31_e488_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));
        let eq31_e488_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));
        let eq31_e488_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));
        let eq31_e488_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));
        let eq31_e488_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));
        let eq31_e488_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));
        let eq31_e488_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));
        let eq31_e488_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));
        let eq31_e488_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));
        let eq31_e488_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));
        let eq31_e488_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));
        let eq31_e488_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));
        let eq31_e488_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));
        let eq31_e488_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));
        let eq31_e488_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));
        let eq31_e488_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));
        let eq31_e488_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));
        let eq31_e488_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));
        let eq31_e488_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));
        let eq31_e488_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));
        let eq31_e488_d_b13: f64 = (s.db[563][13] * (nv10 - 0.0));
        let eq31_e488_d_b14: f64 = (s.db[563][14] * (nv10 - 0.0));
        let eq31_e488_d_b15: f64 = (s.db[563][15] * (nv10 - 0.0));
        let eq31_e489_q: f64 = eq31_e488;
        (eq31_e488, eq31_e488_d_n0, eq31_e488_d_n1, eq31_e488_d_n2, eq31_e488_d_n3, eq31_e488_d_n4, eq31_e488_d_n5, eq31_e488_d_n6, eq31_e488_d_n7, eq31_e488_d_n8, eq31_e488_d_n9, eq31_e488_d_n10, eq31_e488_d_n11, eq31_e488_d_n12, eq31_e488_d_n13, eq31_e488_d_n14, eq31_e488_d_n15, eq31_e488_d_n16, eq31_e488_d_n17, eq31_e488_d_n18, eq31_e488_d_b0, eq31_e488_d_b1, eq31_e488_d_b2, eq31_e488_d_b3, eq31_e488_d_b4, eq31_e488_d_b5, eq31_e488_d_b6, eq31_e488_d_b7, eq31_e488_d_b8, eq31_e488_d_b9, eq31_e488_d_b10, eq31_e488_d_b11, eq31_e488_d_b12, eq31_e488_d_b13, eq31_e488_d_b14, eq31_e488_d_b15, eq31_e489_q, eq31_e488_d_n0, eq31_e488_d_n1, eq31_e488_d_n2, eq31_e488_d_n3, eq31_e488_d_n4, eq31_e488_d_n5, eq31_e488_d_n6, eq31_e488_d_n7, eq31_e488_d_n8, eq31_e488_d_n9, eq31_e488_d_n10, eq31_e488_d_n11, eq31_e488_d_n12, eq31_e488_d_n13, eq31_e488_d_n14, eq31_e488_d_n15, eq31_e488_d_n16, eq31_e488_d_n17, eq31_e488_d_n18, eq31_e488_d_b0, eq31_e488_d_b1, eq31_e488_d_b2, eq31_e488_d_b3, eq31_e488_d_b4, eq31_e488_d_b5, eq31_e488_d_b6, eq31_e488_d_b7, eq31_e488_d_b8, eq31_e488_d_b9, eq31_e488_d_b10, eq31_e488_d_b11, eq31_e488_d_b12, eq31_e488_d_b13, eq31_e488_d_b14, eq31_e488_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_reactive_node_derivatives: [f64; 19] = [eq31_e491_q_d_n0, eq31_e491_q_d_n1, eq31_e491_q_d_n2, eq31_e491_q_d_n3, eq31_e491_q_d_n4, eq31_e491_q_d_n5, eq31_e491_q_d_n6, eq31_e491_q_d_n7, eq31_e491_q_d_n8, eq31_e491_q_d_n9, eq31_e491_q_d_n10, eq31_e491_q_d_n11, eq31_e491_q_d_n12, eq31_e491_q_d_n13, eq31_e491_q_d_n14, eq31_e491_q_d_n15, eq31_e491_q_d_n16, eq31_e491_q_d_n17, eq31_e491_q_d_n18];
        let eq31_reactive_branch_derivatives: [f64; 16] = [eq31_e491_q_d_b0, eq31_e491_q_d_b1, eq31_e491_q_d_b2, eq31_e491_q_d_b3, eq31_e491_q_d_b4, eq31_e491_q_d_b5, eq31_e491_q_d_b6, eq31_e491_q_d_b7, eq31_e491_q_d_b8, eq31_e491_q_d_b9, eq31_e491_q_d_b10, eq31_e491_q_d_b11, eq31_e491_q_d_b12, eq31_e491_q_d_b13, eq31_e491_q_d_b14, eq31_e491_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            None,
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
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
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n1, eq35_e523_d_n2, eq35_e523_d_n3, eq35_e523_d_n4, eq35_e523_d_n5, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n8, eq35_e523_d_n9, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n13, eq35_e523_d_n14, eq35_e523_d_n15, eq35_e523_d_n16, eq35_e523_d_n17, eq35_e523_d_n18, eq35_e523_d_b0, eq35_e523_d_b1, eq35_e523_d_b2, eq35_e523_d_b3, eq35_e523_d_b4, eq35_e523_d_b5, eq35_e523_d_b6, eq35_e523_d_b7, eq35_e523_d_b8, eq35_e523_d_b9, eq35_e523_d_b10, eq35_e523_d_b11, eq35_e523_d_b12, eq35_e523_d_b13, eq35_e523_d_b14, eq35_e523_d_b15, eq35_e523_q, eq35_e523_q_d_n0, eq35_e523_q_d_n1, eq35_e523_q_d_n2, eq35_e523_q_d_n3, eq35_e523_q_d_n4, eq35_e523_q_d_n5, eq35_e523_q_d_n6, eq35_e523_q_d_n7, eq35_e523_q_d_n8, eq35_e523_q_d_n9, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, eq35_e523_q_d_n13, eq35_e523_q_d_n14, eq35_e523_q_d_n15, eq35_e523_q_d_n16, eq35_e523_q_d_n17, eq35_e523_q_d_n18, eq35_e523_q_d_b0, eq35_e523_q_d_b1, eq35_e523_q_d_b2, eq35_e523_q_d_b3, eq35_e523_q_d_b4, eq35_e523_q_d_b5, eq35_e523_q_d_b6, eq35_e523_q_d_b7, eq35_e523_q_d_b8, eq35_e523_q_d_b9, eq35_e523_q_d_b10, eq35_e523_q_d_b11, eq35_e523_q_d_b12, eq35_e523_q_d_b13, eq35_e523_q_d_b14, eq35_e523_q_d_b15,) = {
    if s.b[1851] {
        let eq35_e519_q: f64 = s.v[283];
        let eq35_e520: f64 = (s.v[281] + s.v[283]);
        let eq35_e520_d_n0: f64 = (s.dn[281][0] + s.dn[283][0]);
        let eq35_e520_d_n1: f64 = (s.dn[281][1] + s.dn[283][1]);
        let eq35_e520_d_n2: f64 = (s.dn[281][2] + s.dn[283][2]);
        let eq35_e520_d_n3: f64 = (s.dn[281][3] + s.dn[283][3]);
        let eq35_e520_d_n4: f64 = (s.dn[281][4] + s.dn[283][4]);
        let eq35_e520_d_n5: f64 = (s.dn[281][5] + s.dn[283][5]);
        let eq35_e520_d_n6: f64 = (s.dn[281][6] + s.dn[283][6]);
        let eq35_e520_d_n7: f64 = (s.dn[281][7] + s.dn[283][7]);
        let eq35_e520_d_n8: f64 = (s.dn[281][8] + s.dn[283][8]);
        let eq35_e520_d_n9: f64 = (s.dn[281][9] + s.dn[283][9]);
        let eq35_e520_d_n10: f64 = (s.dn[281][10] + s.dn[283][10]);
        let eq35_e520_d_n11: f64 = (s.dn[281][11] + s.dn[283][11]);
        let eq35_e520_d_n12: f64 = (s.dn[281][12] + s.dn[283][12]);
        let eq35_e520_d_n13: f64 = (s.dn[281][13] + s.dn[283][13]);
        let eq35_e520_d_n14: f64 = (s.dn[281][14] + s.dn[283][14]);
        let eq35_e520_d_n15: f64 = (s.dn[281][15] + s.dn[283][15]);
        let eq35_e520_d_n16: f64 = (s.dn[281][16] + s.dn[283][16]);
        let eq35_e520_d_n17: f64 = (s.dn[281][17] + s.dn[283][17]);
        let eq35_e520_d_n18: f64 = (s.dn[281][18] + s.dn[283][18]);
        let eq35_e520_d_b0: f64 = (s.db[281][0] + s.db[283][0]);
        let eq35_e520_d_b1: f64 = (s.db[281][1] + s.db[283][1]);
        let eq35_e520_d_b2: f64 = (s.db[281][2] + s.db[283][2]);
        let eq35_e520_d_b3: f64 = (s.db[281][3] + s.db[283][3]);
        let eq35_e520_d_b4: f64 = (s.db[281][4] + s.db[283][4]);
        let eq35_e520_d_b5: f64 = (s.db[281][5] + s.db[283][5]);
        let eq35_e520_d_b6: f64 = (s.db[281][6] + s.db[283][6]);
        let eq35_e520_d_b7: f64 = (s.db[281][7] + s.db[283][7]);
        let eq35_e520_d_b8: f64 = (s.db[281][8] + s.db[283][8]);
        let eq35_e520_d_b9: f64 = (s.db[281][9] + s.db[283][9]);
        let eq35_e520_d_b10: f64 = (s.db[281][10] + s.db[283][10]);
        let eq35_e520_d_b11: f64 = (s.db[281][11] + s.db[283][11]);
        let eq35_e520_d_b12: f64 = (s.db[281][12] + s.db[283][12]);
        let eq35_e520_d_b13: f64 = (s.db[281][13] + s.db[283][13]);
        let eq35_e520_d_b14: f64 = (s.db[281][14] + s.db[283][14]);
        let eq35_e520_d_b15: f64 = (s.db[281][15] + s.db[283][15]);
        let eq35_e520_q: f64 = eq35_e519_q;
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n1: f64 = (p.p50 * eq35_e520_d_n1);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n3: f64 = (p.p50 * eq35_e520_d_n3);
        let eq35_e521_d_n4: f64 = (p.p50 * eq35_e520_d_n4);
        let eq35_e521_d_n5: f64 = (p.p50 * eq35_e520_d_n5);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n8: f64 = (p.p50 * eq35_e520_d_n8);
        let eq35_e521_d_n9: f64 = (p.p50 * eq35_e520_d_n9);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n13: f64 = (p.p50 * eq35_e520_d_n13);
        let eq35_e521_d_n14: f64 = (p.p50 * eq35_e520_d_n14);
        let eq35_e521_d_n15: f64 = (p.p50 * eq35_e520_d_n15);
        let eq35_e521_d_n16: f64 = (p.p50 * eq35_e520_d_n16);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        let eq35_e521_d_n18: f64 = (p.p50 * eq35_e520_d_n18);
        let eq35_e521_d_b0: f64 = (p.p50 * eq35_e520_d_b0);
        let eq35_e521_d_b1: f64 = (p.p50 * eq35_e520_d_b1);
        let eq35_e521_d_b2: f64 = (p.p50 * eq35_e520_d_b2);
        let eq35_e521_d_b3: f64 = (p.p50 * eq35_e520_d_b3);
        let eq35_e521_d_b4: f64 = (p.p50 * eq35_e520_d_b4);
        let eq35_e521_d_b5: f64 = (p.p50 * eq35_e520_d_b5);
        let eq35_e521_d_b6: f64 = (p.p50 * eq35_e520_d_b6);
        let eq35_e521_d_b7: f64 = (p.p50 * eq35_e520_d_b7);
        let eq35_e521_d_b8: f64 = (p.p50 * eq35_e520_d_b8);
        let eq35_e521_d_b9: f64 = (p.p50 * eq35_e520_d_b9);
        let eq35_e521_d_b10: f64 = (p.p50 * eq35_e520_d_b10);
        let eq35_e521_d_b11: f64 = (p.p50 * eq35_e520_d_b11);
        let eq35_e521_d_b12: f64 = (p.p50 * eq35_e520_d_b12);
        let eq35_e521_d_b13: f64 = (p.p50 * eq35_e520_d_b13);
        let eq35_e521_d_b14: f64 = (p.p50 * eq35_e520_d_b14);
        let eq35_e521_d_b15: f64 = (p.p50 * eq35_e520_d_b15);
        let eq35_e521_q: f64 = (p.p50 * eq35_e520_q);
        let eq35_e521_q_d_n0: f64 = (p.p50 * s.dn[283][0]);
        let eq35_e521_q_d_n1: f64 = (p.p50 * s.dn[283][1]);
        let eq35_e521_q_d_n2: f64 = (p.p50 * s.dn[283][2]);
        let eq35_e521_q_d_n3: f64 = (p.p50 * s.dn[283][3]);
        let eq35_e521_q_d_n4: f64 = (p.p50 * s.dn[283][4]);
        let eq35_e521_q_d_n5: f64 = (p.p50 * s.dn[283][5]);
        let eq35_e521_q_d_n6: f64 = (p.p50 * s.dn[283][6]);
        let eq35_e521_q_d_n7: f64 = (p.p50 * s.dn[283][7]);
        let eq35_e521_q_d_n8: f64 = (p.p50 * s.dn[283][8]);
        let eq35_e521_q_d_n9: f64 = (p.p50 * s.dn[283][9]);
        let eq35_e521_q_d_n10: f64 = (p.p50 * s.dn[283][10]);
        let eq35_e521_q_d_n11: f64 = (p.p50 * s.dn[283][11]);
        let eq35_e521_q_d_n12: f64 = (p.p50 * s.dn[283][12]);
        let eq35_e521_q_d_n13: f64 = (p.p50 * s.dn[283][13]);
        let eq35_e521_q_d_n14: f64 = (p.p50 * s.dn[283][14]);
        let eq35_e521_q_d_n15: f64 = (p.p50 * s.dn[283][15]);
        let eq35_e521_q_d_n16: f64 = (p.p50 * s.dn[283][16]);
        let eq35_e521_q_d_n17: f64 = (p.p50 * s.dn[283][17]);
        let eq35_e521_q_d_n18: f64 = (p.p50 * s.dn[283][18]);
        let eq35_e521_q_d_b0: f64 = (p.p50 * s.db[283][0]);
        let eq35_e521_q_d_b1: f64 = (p.p50 * s.db[283][1]);
        let eq35_e521_q_d_b2: f64 = (p.p50 * s.db[283][2]);
        let eq35_e521_q_d_b3: f64 = (p.p50 * s.db[283][3]);
        let eq35_e521_q_d_b4: f64 = (p.p50 * s.db[283][4]);
        let eq35_e521_q_d_b5: f64 = (p.p50 * s.db[283][5]);
        let eq35_e521_q_d_b6: f64 = (p.p50 * s.db[283][6]);
        let eq35_e521_q_d_b7: f64 = (p.p50 * s.db[283][7]);
        let eq35_e521_q_d_b8: f64 = (p.p50 * s.db[283][8]);
        let eq35_e521_q_d_b9: f64 = (p.p50 * s.db[283][9]);
        let eq35_e521_q_d_b10: f64 = (p.p50 * s.db[283][10]);
        let eq35_e521_q_d_b11: f64 = (p.p50 * s.db[283][11]);
        let eq35_e521_q_d_b12: f64 = (p.p50 * s.db[283][12]);
        let eq35_e521_q_d_b13: f64 = (p.p50 * s.db[283][13]);
        let eq35_e521_q_d_b14: f64 = (p.p50 * s.db[283][14]);
        let eq35_e521_q_d_b15: f64 = (p.p50 * s.db[283][15]);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n1, eq35_e521_d_n2, eq35_e521_d_n3, eq35_e521_d_n4, eq35_e521_d_n5, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n8, eq35_e521_d_n9, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n13, eq35_e521_d_n14, eq35_e521_d_n15, eq35_e521_d_n16, eq35_e521_d_n17, eq35_e521_d_n18, eq35_e521_d_b0, eq35_e521_d_b1, eq35_e521_d_b2, eq35_e521_d_b3, eq35_e521_d_b4, eq35_e521_d_b5, eq35_e521_d_b6, eq35_e521_d_b7, eq35_e521_d_b8, eq35_e521_d_b9, eq35_e521_d_b10, eq35_e521_d_b11, eq35_e521_d_b12, eq35_e521_d_b13, eq35_e521_d_b14, eq35_e521_d_b15, eq35_e521_q, eq35_e521_q_d_n0, eq35_e521_q_d_n1, eq35_e521_q_d_n2, eq35_e521_q_d_n3, eq35_e521_q_d_n4, eq35_e521_q_d_n5, eq35_e521_q_d_n6, eq35_e521_q_d_n7, eq35_e521_q_d_n8, eq35_e521_q_d_n9, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, eq35_e521_q_d_n13, eq35_e521_q_d_n14, eq35_e521_q_d_n15, eq35_e521_q_d_n16, eq35_e521_q_d_n17, eq35_e521_q_d_n18, eq35_e521_q_d_b0, eq35_e521_q_d_b1, eq35_e521_q_d_b2, eq35_e521_q_d_b3, eq35_e521_q_d_b4, eq35_e521_q_d_b5, eq35_e521_q_d_b6, eq35_e521_q_d_b7, eq35_e521_q_d_b8, eq35_e521_q_d_b9, eq35_e521_q_d_b10, eq35_e521_q_d_b11, eq35_e521_q_d_b12, eq35_e521_q_d_b13, eq35_e521_q_d_b14, eq35_e521_q_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e523_q_d_n0, eq35_e523_q_d_n1, eq35_e523_q_d_n2, eq35_e523_q_d_n3, eq35_e523_q_d_n4, eq35_e523_q_d_n5, eq35_e523_q_d_n6, eq35_e523_q_d_n7, eq35_e523_q_d_n8, eq35_e523_q_d_n9, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, eq35_e523_q_d_n13, eq35_e523_q_d_n14, eq35_e523_q_d_n15, eq35_e523_q_d_n16, eq35_e523_q_d_n17, eq35_e523_q_d_n18];
        let eq35_reactive_branch_derivatives: [f64; 16] = [eq35_e523_q_d_b0, eq35_e523_q_d_b1, eq35_e523_q_d_b2, eq35_e523_q_d_b3, eq35_e523_q_d_b4, eq35_e523_q_d_b5, eq35_e523_q_d_b6, eq35_e523_q_d_b7, eq35_e523_q_d_b8, eq35_e523_q_d_b9, eq35_e523_q_d_b10, eq35_e523_q_d_b11, eq35_e523_q_d_b12, eq35_e523_q_d_b13, eq35_e523_q_d_b14, eq35_e523_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n1, eq36_e532_d_n2, eq36_e532_d_n3, eq36_e532_d_n4, eq36_e532_d_n5, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n8, eq36_e532_d_n9, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n13, eq36_e532_d_n14, eq36_e532_d_n15, eq36_e532_d_n16, eq36_e532_d_n17, eq36_e532_d_n18, eq36_e532_d_b0, eq36_e532_d_b1, eq36_e532_d_b2, eq36_e532_d_b3, eq36_e532_d_b4, eq36_e532_d_b5, eq36_e532_d_b6, eq36_e532_d_b7, eq36_e532_d_b8, eq36_e532_d_b9, eq36_e532_d_b10, eq36_e532_d_b11, eq36_e532_d_b12, eq36_e532_d_b13, eq36_e532_d_b14, eq36_e532_d_b15, eq36_e532_q, eq36_e532_q_d_n0, eq36_e532_q_d_n1, eq36_e532_q_d_n2, eq36_e532_q_d_n3, eq36_e532_q_d_n4, eq36_e532_q_d_n5, eq36_e532_q_d_n6, eq36_e532_q_d_n7, eq36_e532_q_d_n8, eq36_e532_q_d_n9, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, eq36_e532_q_d_n13, eq36_e532_q_d_n14, eq36_e532_q_d_n15, eq36_e532_q_d_n16, eq36_e532_q_d_n17, eq36_e532_q_d_n18, eq36_e532_q_d_b0, eq36_e532_q_d_b1, eq36_e532_q_d_b2, eq36_e532_q_d_b3, eq36_e532_q_d_b4, eq36_e532_q_d_b5, eq36_e532_q_d_b6, eq36_e532_q_d_b7, eq36_e532_q_d_b8, eq36_e532_q_d_b9, eq36_e532_q_d_b10, eq36_e532_q_d_b11, eq36_e532_q_d_b12, eq36_e532_q_d_b13, eq36_e532_q_d_b14, eq36_e532_q_d_b15,) = {
    if s.b[1851] {
        let eq36_e528_q: f64 = s.v[284];
        let eq36_e529: f64 = (s.v[282] + s.v[284]);
        let eq36_e529_d_n0: f64 = (s.dn[282][0] + s.dn[284][0]);
        let eq36_e529_d_n1: f64 = (s.dn[282][1] + s.dn[284][1]);
        let eq36_e529_d_n2: f64 = (s.dn[282][2] + s.dn[284][2]);
        let eq36_e529_d_n3: f64 = (s.dn[282][3] + s.dn[284][3]);
        let eq36_e529_d_n4: f64 = (s.dn[282][4] + s.dn[284][4]);
        let eq36_e529_d_n5: f64 = (s.dn[282][5] + s.dn[284][5]);
        let eq36_e529_d_n6: f64 = (s.dn[282][6] + s.dn[284][6]);
        let eq36_e529_d_n7: f64 = (s.dn[282][7] + s.dn[284][7]);
        let eq36_e529_d_n8: f64 = (s.dn[282][8] + s.dn[284][8]);
        let eq36_e529_d_n9: f64 = (s.dn[282][9] + s.dn[284][9]);
        let eq36_e529_d_n10: f64 = (s.dn[282][10] + s.dn[284][10]);
        let eq36_e529_d_n11: f64 = (s.dn[282][11] + s.dn[284][11]);
        let eq36_e529_d_n12: f64 = (s.dn[282][12] + s.dn[284][12]);
        let eq36_e529_d_n13: f64 = (s.dn[282][13] + s.dn[284][13]);
        let eq36_e529_d_n14: f64 = (s.dn[282][14] + s.dn[284][14]);
        let eq36_e529_d_n15: f64 = (s.dn[282][15] + s.dn[284][15]);
        let eq36_e529_d_n16: f64 = (s.dn[282][16] + s.dn[284][16]);
        let eq36_e529_d_n17: f64 = (s.dn[282][17] + s.dn[284][17]);
        let eq36_e529_d_n18: f64 = (s.dn[282][18] + s.dn[284][18]);
        let eq36_e529_d_b0: f64 = (s.db[282][0] + s.db[284][0]);
        let eq36_e529_d_b1: f64 = (s.db[282][1] + s.db[284][1]);
        let eq36_e529_d_b2: f64 = (s.db[282][2] + s.db[284][2]);
        let eq36_e529_d_b3: f64 = (s.db[282][3] + s.db[284][3]);
        let eq36_e529_d_b4: f64 = (s.db[282][4] + s.db[284][4]);
        let eq36_e529_d_b5: f64 = (s.db[282][5] + s.db[284][5]);
        let eq36_e529_d_b6: f64 = (s.db[282][6] + s.db[284][6]);
        let eq36_e529_d_b7: f64 = (s.db[282][7] + s.db[284][7]);
        let eq36_e529_d_b8: f64 = (s.db[282][8] + s.db[284][8]);
        let eq36_e529_d_b9: f64 = (s.db[282][9] + s.db[284][9]);
        let eq36_e529_d_b10: f64 = (s.db[282][10] + s.db[284][10]);
        let eq36_e529_d_b11: f64 = (s.db[282][11] + s.db[284][11]);
        let eq36_e529_d_b12: f64 = (s.db[282][12] + s.db[284][12]);
        let eq36_e529_d_b13: f64 = (s.db[282][13] + s.db[284][13]);
        let eq36_e529_d_b14: f64 = (s.db[282][14] + s.db[284][14]);
        let eq36_e529_d_b15: f64 = (s.db[282][15] + s.db[284][15]);
        let eq36_e529_q: f64 = eq36_e528_q;
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n1: f64 = (p.p50 * eq36_e529_d_n1);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n3: f64 = (p.p50 * eq36_e529_d_n3);
        let eq36_e530_d_n4: f64 = (p.p50 * eq36_e529_d_n4);
        let eq36_e530_d_n5: f64 = (p.p50 * eq36_e529_d_n5);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n8: f64 = (p.p50 * eq36_e529_d_n8);
        let eq36_e530_d_n9: f64 = (p.p50 * eq36_e529_d_n9);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n13: f64 = (p.p50 * eq36_e529_d_n13);
        let eq36_e530_d_n14: f64 = (p.p50 * eq36_e529_d_n14);
        let eq36_e530_d_n15: f64 = (p.p50 * eq36_e529_d_n15);
        let eq36_e530_d_n16: f64 = (p.p50 * eq36_e529_d_n16);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        let eq36_e530_d_n18: f64 = (p.p50 * eq36_e529_d_n18);
        let eq36_e530_d_b0: f64 = (p.p50 * eq36_e529_d_b0);
        let eq36_e530_d_b1: f64 = (p.p50 * eq36_e529_d_b1);
        let eq36_e530_d_b2: f64 = (p.p50 * eq36_e529_d_b2);
        let eq36_e530_d_b3: f64 = (p.p50 * eq36_e529_d_b3);
        let eq36_e530_d_b4: f64 = (p.p50 * eq36_e529_d_b4);
        let eq36_e530_d_b5: f64 = (p.p50 * eq36_e529_d_b5);
        let eq36_e530_d_b6: f64 = (p.p50 * eq36_e529_d_b6);
        let eq36_e530_d_b7: f64 = (p.p50 * eq36_e529_d_b7);
        let eq36_e530_d_b8: f64 = (p.p50 * eq36_e529_d_b8);
        let eq36_e530_d_b9: f64 = (p.p50 * eq36_e529_d_b9);
        let eq36_e530_d_b10: f64 = (p.p50 * eq36_e529_d_b10);
        let eq36_e530_d_b11: f64 = (p.p50 * eq36_e529_d_b11);
        let eq36_e530_d_b12: f64 = (p.p50 * eq36_e529_d_b12);
        let eq36_e530_d_b13: f64 = (p.p50 * eq36_e529_d_b13);
        let eq36_e530_d_b14: f64 = (p.p50 * eq36_e529_d_b14);
        let eq36_e530_d_b15: f64 = (p.p50 * eq36_e529_d_b15);
        let eq36_e530_q: f64 = (p.p50 * eq36_e529_q);
        let eq36_e530_q_d_n0: f64 = (p.p50 * s.dn[284][0]);
        let eq36_e530_q_d_n1: f64 = (p.p50 * s.dn[284][1]);
        let eq36_e530_q_d_n2: f64 = (p.p50 * s.dn[284][2]);
        let eq36_e530_q_d_n3: f64 = (p.p50 * s.dn[284][3]);
        let eq36_e530_q_d_n4: f64 = (p.p50 * s.dn[284][4]);
        let eq36_e530_q_d_n5: f64 = (p.p50 * s.dn[284][5]);
        let eq36_e530_q_d_n6: f64 = (p.p50 * s.dn[284][6]);
        let eq36_e530_q_d_n7: f64 = (p.p50 * s.dn[284][7]);
        let eq36_e530_q_d_n8: f64 = (p.p50 * s.dn[284][8]);
        let eq36_e530_q_d_n9: f64 = (p.p50 * s.dn[284][9]);
        let eq36_e530_q_d_n10: f64 = (p.p50 * s.dn[284][10]);
        let eq36_e530_q_d_n11: f64 = (p.p50 * s.dn[284][11]);
        let eq36_e530_q_d_n12: f64 = (p.p50 * s.dn[284][12]);
        let eq36_e530_q_d_n13: f64 = (p.p50 * s.dn[284][13]);
        let eq36_e530_q_d_n14: f64 = (p.p50 * s.dn[284][14]);
        let eq36_e530_q_d_n15: f64 = (p.p50 * s.dn[284][15]);
        let eq36_e530_q_d_n16: f64 = (p.p50 * s.dn[284][16]);
        let eq36_e530_q_d_n17: f64 = (p.p50 * s.dn[284][17]);
        let eq36_e530_q_d_n18: f64 = (p.p50 * s.dn[284][18]);
        let eq36_e530_q_d_b0: f64 = (p.p50 * s.db[284][0]);
        let eq36_e530_q_d_b1: f64 = (p.p50 * s.db[284][1]);
        let eq36_e530_q_d_b2: f64 = (p.p50 * s.db[284][2]);
        let eq36_e530_q_d_b3: f64 = (p.p50 * s.db[284][3]);
        let eq36_e530_q_d_b4: f64 = (p.p50 * s.db[284][4]);
        let eq36_e530_q_d_b5: f64 = (p.p50 * s.db[284][5]);
        let eq36_e530_q_d_b6: f64 = (p.p50 * s.db[284][6]);
        let eq36_e530_q_d_b7: f64 = (p.p50 * s.db[284][7]);
        let eq36_e530_q_d_b8: f64 = (p.p50 * s.db[284][8]);
        let eq36_e530_q_d_b9: f64 = (p.p50 * s.db[284][9]);
        let eq36_e530_q_d_b10: f64 = (p.p50 * s.db[284][10]);
        let eq36_e530_q_d_b11: f64 = (p.p50 * s.db[284][11]);
        let eq36_e530_q_d_b12: f64 = (p.p50 * s.db[284][12]);
        let eq36_e530_q_d_b13: f64 = (p.p50 * s.db[284][13]);
        let eq36_e530_q_d_b14: f64 = (p.p50 * s.db[284][14]);
        let eq36_e530_q_d_b15: f64 = (p.p50 * s.db[284][15]);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n1, eq36_e530_d_n2, eq36_e530_d_n3, eq36_e530_d_n4, eq36_e530_d_n5, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n8, eq36_e530_d_n9, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n13, eq36_e530_d_n14, eq36_e530_d_n15, eq36_e530_d_n16, eq36_e530_d_n17, eq36_e530_d_n18, eq36_e530_d_b0, eq36_e530_d_b1, eq36_e530_d_b2, eq36_e530_d_b3, eq36_e530_d_b4, eq36_e530_d_b5, eq36_e530_d_b6, eq36_e530_d_b7, eq36_e530_d_b8, eq36_e530_d_b9, eq36_e530_d_b10, eq36_e530_d_b11, eq36_e530_d_b12, eq36_e530_d_b13, eq36_e530_d_b14, eq36_e530_d_b15, eq36_e530_q, eq36_e530_q_d_n0, eq36_e530_q_d_n1, eq36_e530_q_d_n2, eq36_e530_q_d_n3, eq36_e530_q_d_n4, eq36_e530_q_d_n5, eq36_e530_q_d_n6, eq36_e530_q_d_n7, eq36_e530_q_d_n8, eq36_e530_q_d_n9, eq36_e530_q_d_n10, eq36_e530_q_d_n11, eq36_e530_q_d_n12, eq36_e530_q_d_n13, eq36_e530_q_d_n14, eq36_e530_q_d_n15, eq36_e530_q_d_n16, eq36_e530_q_d_n17, eq36_e530_q_d_n18, eq36_e530_q_d_b0, eq36_e530_q_d_b1, eq36_e530_q_d_b2, eq36_e530_q_d_b3, eq36_e530_q_d_b4, eq36_e530_q_d_b5, eq36_e530_q_d_b6, eq36_e530_q_d_b7, eq36_e530_q_d_b8, eq36_e530_q_d_b9, eq36_e530_q_d_b10, eq36_e530_q_d_b11, eq36_e530_q_d_b12, eq36_e530_q_d_b13, eq36_e530_q_d_b14, eq36_e530_q_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 19] = [eq36_e532_q_d_n0, eq36_e532_q_d_n1, eq36_e532_q_d_n2, eq36_e532_q_d_n3, eq36_e532_q_d_n4, eq36_e532_q_d_n5, eq36_e532_q_d_n6, eq36_e532_q_d_n7, eq36_e532_q_d_n8, eq36_e532_q_d_n9, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, eq36_e532_q_d_n13, eq36_e532_q_d_n14, eq36_e532_q_d_n15, eq36_e532_q_d_n16, eq36_e532_q_d_n17, eq36_e532_q_d_n18];
        let eq36_reactive_branch_derivatives: [f64; 16] = [eq36_e532_q_d_b0, eq36_e532_q_d_b1, eq36_e532_q_d_b2, eq36_e532_q_d_b3, eq36_e532_q_d_b4, eq36_e532_q_d_b5, eq36_e532_q_d_b6, eq36_e532_q_d_b7, eq36_e532_q_d_b8, eq36_e532_q_d_b9, eq36_e532_q_d_b10, eq36_e532_q_d_b11, eq36_e532_q_d_b12, eq36_e532_q_d_b13, eq36_e532_q_d_b14, eq36_e532_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e616, eq47_e616_d_n18, eq47_e616_q, eq47_e616_q_d_n18,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614_q: f64 = eq47_e613;
        (eq47_e613, eq47_e611, eq47_e614_q, eq47_e611,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq47_e616_q_d_n18),
        );
        let (eq48_e627, eq48_e627_d_n13, eq48_e627_q, eq48_e627_q_d_n13,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625_q: f64 = eq48_e624;
        (eq48_e624, eq48_e622, eq48_e625_q, eq48_e622,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq48_e627_q_d_n13),
        );
        let (eq53_e666, eq53_e666_d_n17, eq53_e666_q, eq53_e666_q_d_n17,) = {
    if (s.b[1851] && s.b[1852]) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664_q: f64 = eq53_e663;
        (eq53_e663, eq53_e661, eq53_e664_q, eq53_e661,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq53_e666_q_d_n17),
        );
        let (eq60_e724, eq60_e724_d_n17, eq60_e724_q, eq60_e724_q_d_n17,) = {
    if ((!s.b[1851]) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722_q: f64 = eq60_e721;
        (eq60_e721, eq60_e719, eq60_e722_q, eq60_e719,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq60_e724_q_d_n17),
        );
        let (eq68_e792, eq68_e792_d_n15, eq68_e792_q, eq68_e792_q_d_n15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790_q: f64 = eq68_e789;
        (eq68_e789, eq68_e787, eq68_e790_q, eq68_e787,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq68_e792_q_d_n15),
        );
        let (eq69_e804, eq69_e804_d_n16, eq69_e804_q, eq69_e804_q_d_n16,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802_q: f64 = eq69_e801;
        (eq69_e801, eq69_e799, eq69_e802_q, eq69_e799,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq69_e804_q_d_n16),
        );
        let (eq70_e816, eq70_e816_d_n13, eq70_e816_q, eq70_e816_q_d_n13,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814_q: f64 = eq70_e813;
        (eq70_e813, eq70_e811, eq70_e814_q, eq70_e811,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq70_e816_q_d_n13),
        );
    }
}
