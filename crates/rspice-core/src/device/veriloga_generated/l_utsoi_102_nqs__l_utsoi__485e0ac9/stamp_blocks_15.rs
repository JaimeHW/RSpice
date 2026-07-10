#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_117(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1894] {s.store_offset_sqrt_ad(1857, A::offset(A::div_scaled_inputs2(s.ad_value(1834), 2.0, s.ad_value(1856), (-2.0), s.ad_value(1835), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1838), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1846)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);
        let (eq0_e510, eq0_e510_d_n0, eq0_e510_d_n1, eq0_e510_d_n2, eq0_e510_d_n3, eq0_e510_d_n4, eq0_e510_d_n5, eq0_e510_d_n6, eq0_e510_d_n7, eq0_e510_d_n8, eq0_e510_d_n9, eq0_e510_d_n10, eq0_e510_d_n11, eq0_e510_d_n12, eq0_e510_d_n13, eq0_e510_d_b0, eq0_e510_d_b1, eq0_e510_d_b2, eq0_e510_d_b3,) = {
    if s.b[1767] {
        let eq0_e508: f64 = (p.p14 * s.v[365]);
        (eq0_e508, (p.p14 * s.dn[365][0]), (p.p14 * s.dn[365][1]), (p.p14 * s.dn[365][2]), (p.p14 * s.dn[365][3]), (p.p14 * s.dn[365][4]), (p.p14 * s.dn[365][5]), (p.p14 * s.dn[365][6]), (p.p14 * s.dn[365][7]), (p.p14 * s.dn[365][8]), (p.p14 * s.dn[365][9]), (p.p14 * s.dn[365][10]), (p.p14 * s.dn[365][11]), (p.p14 * s.dn[365][12]), (p.p14 * s.dn[365][13]), (p.p14 * s.db[365][0]), (p.p14 * s.db[365][1]), (p.p14 * s.db[365][2]), (p.p14 * s.db[365][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e510;let eq0_node_derivatives: [f64; 14] = [eq0_e510_d_n0, eq0_e510_d_n1, eq0_e510_d_n2, eq0_e510_d_n3, eq0_e510_d_n4, eq0_e510_d_n5, eq0_e510_d_n6, eq0_e510_d_n7, eq0_e510_d_n8, eq0_e510_d_n9, eq0_e510_d_n10, eq0_e510_d_n11, eq0_e510_d_n12, eq0_e510_d_n13];let eq0_branch_derivatives: [f64; 4] = [eq0_e510_d_b0, eq0_e510_d_b1, eq0_e510_d_b2, eq0_e510_d_b3];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e517, eq1_e517_d_n0, eq1_e517_d_n1, eq1_e517_d_n2, eq1_e517_d_n3, eq1_e517_d_n4, eq1_e517_d_n5, eq1_e517_d_n6, eq1_e517_d_n7, eq1_e517_d_n8, eq1_e517_d_n9, eq1_e517_d_n10, eq1_e517_d_n11, eq1_e517_d_n12, eq1_e517_d_n13, eq1_e517_d_b0, eq1_e517_d_b1, eq1_e517_d_b2, eq1_e517_d_b3,) = {
    if (!s.b[1767]) {
        let eq1_e515: f64 = (p.p14 * s.v[365]);
        (eq1_e515, (p.p14 * s.dn[365][0]), (p.p14 * s.dn[365][1]), (p.p14 * s.dn[365][2]), (p.p14 * s.dn[365][3]), (p.p14 * s.dn[365][4]), (p.p14 * s.dn[365][5]), (p.p14 * s.dn[365][6]), (p.p14 * s.dn[365][7]), (p.p14 * s.dn[365][8]), (p.p14 * s.dn[365][9]), (p.p14 * s.dn[365][10]), (p.p14 * s.dn[365][11]), (p.p14 * s.dn[365][12]), (p.p14 * s.dn[365][13]), (p.p14 * s.db[365][0]), (p.p14 * s.db[365][1]), (p.p14 * s.db[365][2]), (p.p14 * s.db[365][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e517;let eq1_node_derivatives: [f64; 14] = [eq1_e517_d_n0, eq1_e517_d_n1, eq1_e517_d_n2, eq1_e517_d_n3, eq1_e517_d_n4, eq1_e517_d_n5, eq1_e517_d_n6, eq1_e517_d_n7, eq1_e517_d_n8, eq1_e517_d_n9, eq1_e517_d_n10, eq1_e517_d_n11, eq1_e517_d_n12, eq1_e517_d_n13];let eq1_branch_derivatives: [f64; 4] = [eq1_e517_d_b0, eq1_e517_d_b1, eq1_e517_d_b2, eq1_e517_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );let eq2_e521: f64 = (s.v[368] - s.v[369]);let eq2_e521_d_n0: f64 = (s.dn[368][0] - s.dn[369][0]);let eq2_e521_d_n1: f64 = (s.dn[368][1] - s.dn[369][1]);let eq2_e521_d_n2: f64 = (s.dn[368][2] - s.dn[369][2]);let eq2_e521_d_n3: f64 = (s.dn[368][3] - s.dn[369][3]);let eq2_e521_d_n4: f64 = (s.dn[368][4] - s.dn[369][4]);let eq2_e521_d_n5: f64 = (s.dn[368][5] - s.dn[369][5]);let eq2_e521_d_n6: f64 = (s.dn[368][6] - s.dn[369][6]);let eq2_e521_d_n7: f64 = (s.dn[368][7] - s.dn[369][7]);let eq2_e521_d_n8: f64 = (s.dn[368][8] - s.dn[369][8]);let eq2_e521_d_n9: f64 = (s.dn[368][9] - s.dn[369][9]);let eq2_e521_d_n10: f64 = (s.dn[368][10] - s.dn[369][10]);let eq2_e521_d_n11: f64 = (s.dn[368][11] - s.dn[369][11]);let eq2_e521_d_n12: f64 = (s.dn[368][12] - s.dn[369][12]);let eq2_e521_d_n13: f64 = (s.dn[368][13] - s.dn[369][13]);let eq2_e521_d_b0: f64 = (s.db[368][0] - s.db[369][0]);let eq2_e521_d_b1: f64 = (s.db[368][1] - s.db[369][1]);let eq2_e521_d_b2: f64 = (s.db[368][2] - s.db[369][2]);let eq2_e521_d_b3: f64 = (s.db[368][3] - s.db[369][3]);let eq2_e522: f64 = (p.p14 * eq2_e521);let eq2_e522_d_n0: f64 = (p.p14 * eq2_e521_d_n0);let eq2_e522_d_n1: f64 = (p.p14 * eq2_e521_d_n1);let eq2_e522_d_n2: f64 = (p.p14 * eq2_e521_d_n2);let eq2_e522_d_n3: f64 = (p.p14 * eq2_e521_d_n3);let eq2_e522_d_n4: f64 = (p.p14 * eq2_e521_d_n4);let eq2_e522_d_n5: f64 = (p.p14 * eq2_e521_d_n5);let eq2_e522_d_n6: f64 = (p.p14 * eq2_e521_d_n6);let eq2_e522_d_n7: f64 = (p.p14 * eq2_e521_d_n7);let eq2_e522_d_n8: f64 = (p.p14 * eq2_e521_d_n8);let eq2_e522_d_n9: f64 = (p.p14 * eq2_e521_d_n9);let eq2_e522_d_n10: f64 = (p.p14 * eq2_e521_d_n10);let eq2_e522_d_n11: f64 = (p.p14 * eq2_e521_d_n11);let eq2_e522_d_n12: f64 = (p.p14 * eq2_e521_d_n12);let eq2_e522_d_n13: f64 = (p.p14 * eq2_e521_d_n13);let eq2_e522_d_b0: f64 = (p.p14 * eq2_e521_d_b0);let eq2_e522_d_b1: f64 = (p.p14 * eq2_e521_d_b1);let eq2_e522_d_b2: f64 = (p.p14 * eq2_e521_d_b2);let eq2_e522_d_b3: f64 = (p.p14 * eq2_e521_d_b3);let eq2_value: f64 = eq2_e522;let eq2_node_derivatives: [f64; 14] = [eq2_e522_d_n0, eq2_e522_d_n1, eq2_e522_d_n2, eq2_e522_d_n3, eq2_e522_d_n4, eq2_e522_d_n5, eq2_e522_d_n6, eq2_e522_d_n7, eq2_e522_d_n8, eq2_e522_d_n9, eq2_e522_d_n10, eq2_e522_d_n11, eq2_e522_d_n12, eq2_e522_d_n13];let eq2_branch_derivatives: [f64; 4] = [eq2_e522_d_b0, eq2_e522_d_b1, eq2_e522_d_b2, eq2_e522_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );let eq3_e525: f64 = (p.p14 * s.v[366]);let eq3_value: f64 = eq3_e525;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            &s.dn[366],
            &s.db[366],
            (multiplicity) * (p.p14),
        );let eq4_e528: f64 = (p.p14 * s.v[367]);let eq4_value: f64 = eq4_e528;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq4_value),
            &s.dn[367],
            &s.db[367],
            (multiplicity) * (p.p14),
        );let eq8_e534: f64 = (p.p31 * s.v[475]);let eq8_e536: f64 = (eq8_e534 * (nv7 - nv6));let eq8_value: f64 = eq8_e536;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(6),
            multiplicity * (eq8_value),
            6,
            multiplicity * ((-eq8_e534)),
            7,
            multiplicity * (eq8_e534),
        );let eq9_value: f64 = s.v[1765];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq9_value),
            &s.dn[1765],
            &s.db[1765],
            multiplicity,
        );let eq10_value: f64 = s.v[1766];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq10_value),
            &s.dn[1766],
            &s.db[1766],
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);let nv6 = ctx.node_voltage(nodes[6]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq11_e548, eq11_e548_d_n0, eq11_e548_d_n1, eq11_e548_d_n2, eq11_e548_d_n3, eq11_e548_d_n4, eq11_e548_d_n5, eq11_e548_d_n6, eq11_e548_d_n7, eq11_e548_d_n8, eq11_e548_d_n9, eq11_e548_d_n10, eq11_e548_d_n11, eq11_e548_d_n12, eq11_e548_d_n13, eq11_e548_d_b0, eq11_e548_d_b1, eq11_e548_d_b2, eq11_e548_d_b3,) = {
    if s.b[1768] {
        let eq11_e542: f64 = (p.p31 * s.v[13]);let eq11_e544: f64 = (eq11_e542 * s.v[316]);let eq11_e544_d_n0: f64 = (((p.p31 * s.dn[13][0]) * s.v[316]) + (eq11_e542 * s.dn[316][0]));let eq11_e544_d_n1: f64 = (((p.p31 * s.dn[13][1]) * s.v[316]) + (eq11_e542 * s.dn[316][1]));let eq11_e544_d_n2: f64 = (((p.p31 * s.dn[13][2]) * s.v[316]) + (eq11_e542 * s.dn[316][2]));let eq11_e544_d_n3: f64 = (((p.p31 * s.dn[13][3]) * s.v[316]) + (eq11_e542 * s.dn[316][3]));let eq11_e544_d_n4: f64 = (((p.p31 * s.dn[13][4]) * s.v[316]) + (eq11_e542 * s.dn[316][4]));let eq11_e544_d_n5: f64 = (((p.p31 * s.dn[13][5]) * s.v[316]) + (eq11_e542 * s.dn[316][5]));let eq11_e544_d_n6: f64 = (((p.p31 * s.dn[13][6]) * s.v[316]) + (eq11_e542 * s.dn[316][6]));let eq11_e544_d_n7: f64 = (((p.p31 * s.dn[13][7]) * s.v[316]) + (eq11_e542 * s.dn[316][7]));let eq11_e544_d_n8: f64 = (((p.p31 * s.dn[13][8]) * s.v[316]) + (eq11_e542 * s.dn[316][8]));let eq11_e544_d_n9: f64 = (((p.p31 * s.dn[13][9]) * s.v[316]) + (eq11_e542 * s.dn[316][9]));let eq11_e544_d_n10: f64 = (((p.p31 * s.dn[13][10]) * s.v[316]) + (eq11_e542 * s.dn[316][10]));let eq11_e544_d_n11: f64 = (((p.p31 * s.dn[13][11]) * s.v[316]) + (eq11_e542 * s.dn[316][11]));let eq11_e544_d_n12: f64 = (((p.p31 * s.dn[13][12]) * s.v[316]) + (eq11_e542 * s.dn[316][12]));let eq11_e544_d_n13: f64 = (((p.p31 * s.dn[13][13]) * s.v[316]) + (eq11_e542 * s.dn[316][13]));let eq11_e544_d_b0: f64 = (((p.p31 * s.db[13][0]) * s.v[316]) + (eq11_e542 * s.db[316][0]));let eq11_e544_d_b1: f64 = (((p.p31 * s.db[13][1]) * s.v[316]) + (eq11_e542 * s.db[316][1]));let eq11_e544_d_b2: f64 = (((p.p31 * s.db[13][2]) * s.v[316]) + (eq11_e542 * s.db[316][2]));let eq11_e544_d_b3: f64 = (((p.p31 * s.db[13][3]) * s.v[316]) + (eq11_e542 * s.db[316][3]));let eq11_e546: f64 = (eq11_e544 * (nv1 - nv9));let eq11_e546_d_n0: f64 = (eq11_e544_d_n0 * (nv1 - nv9));let eq11_e546_d_n1: f64 = ((eq11_e544_d_n1 * (nv1 - nv9)) + eq11_e544);let eq11_e546_d_n2: f64 = (eq11_e544_d_n2 * (nv1 - nv9));let eq11_e546_d_n3: f64 = (eq11_e544_d_n3 * (nv1 - nv9));let eq11_e546_d_n4: f64 = (eq11_e544_d_n4 * (nv1 - nv9));let eq11_e546_d_n5: f64 = (eq11_e544_d_n5 * (nv1 - nv9));let eq11_e546_d_n6: f64 = (eq11_e544_d_n6 * (nv1 - nv9));let eq11_e546_d_n7: f64 = (eq11_e544_d_n7 * (nv1 - nv9));let eq11_e546_d_n8: f64 = (eq11_e544_d_n8 * (nv1 - nv9));let eq11_e546_d_n9: f64 = ((eq11_e544_d_n9 * (nv1 - nv9)) + (-eq11_e544));let eq11_e546_d_n10: f64 = (eq11_e544_d_n10 * (nv1 - nv9));let eq11_e546_d_n11: f64 = (eq11_e544_d_n11 * (nv1 - nv9));let eq11_e546_d_n12: f64 = (eq11_e544_d_n12 * (nv1 - nv9));let eq11_e546_d_n13: f64 = (eq11_e544_d_n13 * (nv1 - nv9));let eq11_e546_d_b0: f64 = (eq11_e544_d_b0 * (nv1 - nv9));let eq11_e546_d_b1: f64 = (eq11_e544_d_b1 * (nv1 - nv9));let eq11_e546_d_b2: f64 = (eq11_e544_d_b2 * (nv1 - nv9));let eq11_e546_d_b3: f64 = (eq11_e544_d_b3 * (nv1 - nv9));
        (eq11_e546, eq11_e546_d_n0, eq11_e546_d_n1, eq11_e546_d_n2, eq11_e546_d_n3, eq11_e546_d_n4, eq11_e546_d_n5, eq11_e546_d_n6, eq11_e546_d_n7, eq11_e546_d_n8, eq11_e546_d_n9, eq11_e546_d_n10, eq11_e546_d_n11, eq11_e546_d_n12, eq11_e546_d_n13, eq11_e546_d_b0, eq11_e546_d_b1, eq11_e546_d_b2, eq11_e546_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e548;let eq11_node_derivatives: [f64; 14] = [eq11_e548_d_n0, eq11_e548_d_n1, eq11_e548_d_n2, eq11_e548_d_n3, eq11_e548_d_n4, eq11_e548_d_n5, eq11_e548_d_n6, eq11_e548_d_n7, eq11_e548_d_n8, eq11_e548_d_n9, eq11_e548_d_n10, eq11_e548_d_n11, eq11_e548_d_n12, eq11_e548_d_n13];let eq11_branch_derivatives: [f64; 4] = [eq11_e548_d_b0, eq11_e548_d_b1, eq11_e548_d_b2, eq11_e548_d_b3];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq13_e563,) = {
    if (!s.b[1768]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e563;
        stamper.stamp_potential_const_local(
            0,
            eq13_value,
        );
        let (eq14_e573, eq14_e573_d_n0, eq14_e573_d_n1, eq14_e573_d_n2, eq14_e573_d_n3, eq14_e573_d_n4, eq14_e573_d_n5, eq14_e573_d_n6, eq14_e573_d_n7, eq14_e573_d_n8, eq14_e573_d_n9, eq14_e573_d_n10, eq14_e573_d_n11, eq14_e573_d_n12, eq14_e573_d_n13, eq14_e573_d_b0, eq14_e573_d_b1, eq14_e573_d_b2, eq14_e573_d_b3,) = {
    if s.b[1769] {
        let eq14_e567: f64 = (p.p31 * s.v[13]);let eq14_e569: f64 = (eq14_e567 * s.v[320]);let eq14_e569_d_n0: f64 = (((p.p31 * s.dn[13][0]) * s.v[320]) + (eq14_e567 * s.dn[320][0]));let eq14_e569_d_n1: f64 = (((p.p31 * s.dn[13][1]) * s.v[320]) + (eq14_e567 * s.dn[320][1]));let eq14_e569_d_n2: f64 = (((p.p31 * s.dn[13][2]) * s.v[320]) + (eq14_e567 * s.dn[320][2]));let eq14_e569_d_n3: f64 = (((p.p31 * s.dn[13][3]) * s.v[320]) + (eq14_e567 * s.dn[320][3]));let eq14_e569_d_n4: f64 = (((p.p31 * s.dn[13][4]) * s.v[320]) + (eq14_e567 * s.dn[320][4]));let eq14_e569_d_n5: f64 = (((p.p31 * s.dn[13][5]) * s.v[320]) + (eq14_e567 * s.dn[320][5]));let eq14_e569_d_n6: f64 = (((p.p31 * s.dn[13][6]) * s.v[320]) + (eq14_e567 * s.dn[320][6]));let eq14_e569_d_n7: f64 = (((p.p31 * s.dn[13][7]) * s.v[320]) + (eq14_e567 * s.dn[320][7]));let eq14_e569_d_n8: f64 = (((p.p31 * s.dn[13][8]) * s.v[320]) + (eq14_e567 * s.dn[320][8]));let eq14_e569_d_n9: f64 = (((p.p31 * s.dn[13][9]) * s.v[320]) + (eq14_e567 * s.dn[320][9]));let eq14_e569_d_n10: f64 = (((p.p31 * s.dn[13][10]) * s.v[320]) + (eq14_e567 * s.dn[320][10]));let eq14_e569_d_n11: f64 = (((p.p31 * s.dn[13][11]) * s.v[320]) + (eq14_e567 * s.dn[320][11]));let eq14_e569_d_n12: f64 = (((p.p31 * s.dn[13][12]) * s.v[320]) + (eq14_e567 * s.dn[320][12]));let eq14_e569_d_n13: f64 = (((p.p31 * s.dn[13][13]) * s.v[320]) + (eq14_e567 * s.dn[320][13]));let eq14_e569_d_b0: f64 = (((p.p31 * s.db[13][0]) * s.v[320]) + (eq14_e567 * s.db[320][0]));let eq14_e569_d_b1: f64 = (((p.p31 * s.db[13][1]) * s.v[320]) + (eq14_e567 * s.db[320][1]));let eq14_e569_d_b2: f64 = (((p.p31 * s.db[13][2]) * s.v[320]) + (eq14_e567 * s.db[320][2]));let eq14_e569_d_b3: f64 = (((p.p31 * s.db[13][3]) * s.v[320]) + (eq14_e567 * s.db[320][3]));let eq14_e571: f64 = (eq14_e569 * (nv2 - nv6));let eq14_e571_d_n0: f64 = (eq14_e569_d_n0 * (nv2 - nv6));let eq14_e571_d_n1: f64 = (eq14_e569_d_n1 * (nv2 - nv6));let eq14_e571_d_n2: f64 = ((eq14_e569_d_n2 * (nv2 - nv6)) + eq14_e569);let eq14_e571_d_n3: f64 = (eq14_e569_d_n3 * (nv2 - nv6));let eq14_e571_d_n4: f64 = (eq14_e569_d_n4 * (nv2 - nv6));let eq14_e571_d_n5: f64 = (eq14_e569_d_n5 * (nv2 - nv6));let eq14_e571_d_n6: f64 = ((eq14_e569_d_n6 * (nv2 - nv6)) + (-eq14_e569));let eq14_e571_d_n7: f64 = (eq14_e569_d_n7 * (nv2 - nv6));let eq14_e571_d_n8: f64 = (eq14_e569_d_n8 * (nv2 - nv6));let eq14_e571_d_n9: f64 = (eq14_e569_d_n9 * (nv2 - nv6));let eq14_e571_d_n10: f64 = (eq14_e569_d_n10 * (nv2 - nv6));let eq14_e571_d_n11: f64 = (eq14_e569_d_n11 * (nv2 - nv6));let eq14_e571_d_n12: f64 = (eq14_e569_d_n12 * (nv2 - nv6));let eq14_e571_d_n13: f64 = (eq14_e569_d_n13 * (nv2 - nv6));let eq14_e571_d_b0: f64 = (eq14_e569_d_b0 * (nv2 - nv6));let eq14_e571_d_b1: f64 = (eq14_e569_d_b1 * (nv2 - nv6));let eq14_e571_d_b2: f64 = (eq14_e569_d_b2 * (nv2 - nv6));let eq14_e571_d_b3: f64 = (eq14_e569_d_b3 * (nv2 - nv6));
        (eq14_e571, eq14_e571_d_n0, eq14_e571_d_n1, eq14_e571_d_n2, eq14_e571_d_n3, eq14_e571_d_n4, eq14_e571_d_n5, eq14_e571_d_n6, eq14_e571_d_n7, eq14_e571_d_n8, eq14_e571_d_n9, eq14_e571_d_n10, eq14_e571_d_n11, eq14_e571_d_n12, eq14_e571_d_n13, eq14_e571_d_b0, eq14_e571_d_b1, eq14_e571_d_b2, eq14_e571_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e573;let eq14_node_derivatives: [f64; 14] = [eq14_e573_d_n0, eq14_e573_d_n1, eq14_e573_d_n2, eq14_e573_d_n3, eq14_e573_d_n4, eq14_e573_d_n5, eq14_e573_d_n6, eq14_e573_d_n7, eq14_e573_d_n8, eq14_e573_d_n9, eq14_e573_d_n10, eq14_e573_d_n11, eq14_e573_d_n12, eq14_e573_d_n13];let eq14_branch_derivatives: [f64; 4] = [eq14_e573_d_b0, eq14_e573_d_b1, eq14_e573_d_b2, eq14_e573_d_b3];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv7 = ctx.node_voltage(nodes[7]);
        let (eq16_e588,) = {
    if (!s.b[1769]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e588;
        stamper.stamp_potential_const_local(
            1,
            eq16_value,
        );
        let (eq17_e598, eq17_e598_d_n0, eq17_e598_d_n1, eq17_e598_d_n2, eq17_e598_d_n3, eq17_e598_d_n4, eq17_e598_d_n5, eq17_e598_d_n6, eq17_e598_d_n7, eq17_e598_d_n8, eq17_e598_d_n9, eq17_e598_d_n10, eq17_e598_d_n11, eq17_e598_d_n12, eq17_e598_d_n13, eq17_e598_d_b0, eq17_e598_d_b1, eq17_e598_d_b2, eq17_e598_d_b3,) = {
    if s.b[1770] {
        let eq17_e592: f64 = (p.p31 * s.v[13]);let eq17_e594: f64 = (eq17_e592 * s.v[324]);let eq17_e594_d_n0: f64 = (((p.p31 * s.dn[13][0]) * s.v[324]) + (eq17_e592 * s.dn[324][0]));let eq17_e594_d_n1: f64 = (((p.p31 * s.dn[13][1]) * s.v[324]) + (eq17_e592 * s.dn[324][1]));let eq17_e594_d_n2: f64 = (((p.p31 * s.dn[13][2]) * s.v[324]) + (eq17_e592 * s.dn[324][2]));let eq17_e594_d_n3: f64 = (((p.p31 * s.dn[13][3]) * s.v[324]) + (eq17_e592 * s.dn[324][3]));let eq17_e594_d_n4: f64 = (((p.p31 * s.dn[13][4]) * s.v[324]) + (eq17_e592 * s.dn[324][4]));let eq17_e594_d_n5: f64 = (((p.p31 * s.dn[13][5]) * s.v[324]) + (eq17_e592 * s.dn[324][5]));let eq17_e594_d_n6: f64 = (((p.p31 * s.dn[13][6]) * s.v[324]) + (eq17_e592 * s.dn[324][6]));let eq17_e594_d_n7: f64 = (((p.p31 * s.dn[13][7]) * s.v[324]) + (eq17_e592 * s.dn[324][7]));let eq17_e594_d_n8: f64 = (((p.p31 * s.dn[13][8]) * s.v[324]) + (eq17_e592 * s.dn[324][8]));let eq17_e594_d_n9: f64 = (((p.p31 * s.dn[13][9]) * s.v[324]) + (eq17_e592 * s.dn[324][9]));let eq17_e594_d_n10: f64 = (((p.p31 * s.dn[13][10]) * s.v[324]) + (eq17_e592 * s.dn[324][10]));let eq17_e594_d_n11: f64 = (((p.p31 * s.dn[13][11]) * s.v[324]) + (eq17_e592 * s.dn[324][11]));let eq17_e594_d_n12: f64 = (((p.p31 * s.dn[13][12]) * s.v[324]) + (eq17_e592 * s.dn[324][12]));let eq17_e594_d_n13: f64 = (((p.p31 * s.dn[13][13]) * s.v[324]) + (eq17_e592 * s.dn[324][13]));let eq17_e594_d_b0: f64 = (((p.p31 * s.db[13][0]) * s.v[324]) + (eq17_e592 * s.db[324][0]));let eq17_e594_d_b1: f64 = (((p.p31 * s.db[13][1]) * s.v[324]) + (eq17_e592 * s.db[324][1]));let eq17_e594_d_b2: f64 = (((p.p31 * s.db[13][2]) * s.v[324]) + (eq17_e592 * s.db[324][2]));let eq17_e594_d_b3: f64 = (((p.p31 * s.db[13][3]) * s.v[324]) + (eq17_e592 * s.db[324][3]));let eq17_e596: f64 = (eq17_e594 * (nv0 - nv7));let eq17_e596_d_n0: f64 = ((eq17_e594_d_n0 * (nv0 - nv7)) + eq17_e594);let eq17_e596_d_n1: f64 = (eq17_e594_d_n1 * (nv0 - nv7));let eq17_e596_d_n2: f64 = (eq17_e594_d_n2 * (nv0 - nv7));let eq17_e596_d_n3: f64 = (eq17_e594_d_n3 * (nv0 - nv7));let eq17_e596_d_n4: f64 = (eq17_e594_d_n4 * (nv0 - nv7));let eq17_e596_d_n5: f64 = (eq17_e594_d_n5 * (nv0 - nv7));let eq17_e596_d_n6: f64 = (eq17_e594_d_n6 * (nv0 - nv7));let eq17_e596_d_n7: f64 = ((eq17_e594_d_n7 * (nv0 - nv7)) + (-eq17_e594));let eq17_e596_d_n8: f64 = (eq17_e594_d_n8 * (nv0 - nv7));let eq17_e596_d_n9: f64 = (eq17_e594_d_n9 * (nv0 - nv7));let eq17_e596_d_n10: f64 = (eq17_e594_d_n10 * (nv0 - nv7));let eq17_e596_d_n11: f64 = (eq17_e594_d_n11 * (nv0 - nv7));let eq17_e596_d_n12: f64 = (eq17_e594_d_n12 * (nv0 - nv7));let eq17_e596_d_n13: f64 = (eq17_e594_d_n13 * (nv0 - nv7));let eq17_e596_d_b0: f64 = (eq17_e594_d_b0 * (nv0 - nv7));let eq17_e596_d_b1: f64 = (eq17_e594_d_b1 * (nv0 - nv7));let eq17_e596_d_b2: f64 = (eq17_e594_d_b2 * (nv0 - nv7));let eq17_e596_d_b3: f64 = (eq17_e594_d_b3 * (nv0 - nv7));
        (eq17_e596, eq17_e596_d_n0, eq17_e596_d_n1, eq17_e596_d_n2, eq17_e596_d_n3, eq17_e596_d_n4, eq17_e596_d_n5, eq17_e596_d_n6, eq17_e596_d_n7, eq17_e596_d_n8, eq17_e596_d_n9, eq17_e596_d_n10, eq17_e596_d_n11, eq17_e596_d_n12, eq17_e596_d_n13, eq17_e596_d_b0, eq17_e596_d_b1, eq17_e596_d_b2, eq17_e596_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e598;let eq17_node_derivatives: [f64; 14] = [eq17_e598_d_n0, eq17_e598_d_n1, eq17_e598_d_n2, eq17_e598_d_n3, eq17_e598_d_n4, eq17_e598_d_n5, eq17_e598_d_n6, eq17_e598_d_n7, eq17_e598_d_n8, eq17_e598_d_n9, eq17_e598_d_n10, eq17_e598_d_n11, eq17_e598_d_n12, eq17_e598_d_n13];let eq17_branch_derivatives: [f64; 4] = [eq17_e598_d_b0, eq17_e598_d_b1, eq17_e598_d_b2, eq17_e598_d_b3];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq19_e613,) = {
    if (!s.b[1770]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e613;
        stamper.stamp_potential_const_local(
            2,
            eq19_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq20_e623, eq20_e623_d_n0, eq20_e623_d_n1, eq20_e623_d_n2, eq20_e623_d_n3, eq20_e623_d_n4, eq20_e623_d_n5, eq20_e623_d_n6, eq20_e623_d_n7, eq20_e623_d_n8, eq20_e623_d_n9, eq20_e623_d_n10, eq20_e623_d_n11, eq20_e623_d_n12, eq20_e623_d_n13, eq20_e623_d_b0, eq20_e623_d_b1, eq20_e623_d_b2, eq20_e623_d_b3,) = {
    if s.b[1771] {
        let eq20_e617: f64 = (p.p31 * s.v[13]);let eq20_e619: f64 = (eq20_e617 * s.v[327]);let eq20_e619_d_n0: f64 = (((p.p31 * s.dn[13][0]) * s.v[327]) + (eq20_e617 * s.dn[327][0]));let eq20_e619_d_n1: f64 = (((p.p31 * s.dn[13][1]) * s.v[327]) + (eq20_e617 * s.dn[327][1]));let eq20_e619_d_n2: f64 = (((p.p31 * s.dn[13][2]) * s.v[327]) + (eq20_e617 * s.dn[327][2]));let eq20_e619_d_n3: f64 = (((p.p31 * s.dn[13][3]) * s.v[327]) + (eq20_e617 * s.dn[327][3]));let eq20_e619_d_n4: f64 = (((p.p31 * s.dn[13][4]) * s.v[327]) + (eq20_e617 * s.dn[327][4]));let eq20_e619_d_n5: f64 = (((p.p31 * s.dn[13][5]) * s.v[327]) + (eq20_e617 * s.dn[327][5]));let eq20_e619_d_n6: f64 = (((p.p31 * s.dn[13][6]) * s.v[327]) + (eq20_e617 * s.dn[327][6]));let eq20_e619_d_n7: f64 = (((p.p31 * s.dn[13][7]) * s.v[327]) + (eq20_e617 * s.dn[327][7]));let eq20_e619_d_n8: f64 = (((p.p31 * s.dn[13][8]) * s.v[327]) + (eq20_e617 * s.dn[327][8]));let eq20_e619_d_n9: f64 = (((p.p31 * s.dn[13][9]) * s.v[327]) + (eq20_e617 * s.dn[327][9]));let eq20_e619_d_n10: f64 = (((p.p31 * s.dn[13][10]) * s.v[327]) + (eq20_e617 * s.dn[327][10]));let eq20_e619_d_n11: f64 = (((p.p31 * s.dn[13][11]) * s.v[327]) + (eq20_e617 * s.dn[327][11]));let eq20_e619_d_n12: f64 = (((p.p31 * s.dn[13][12]) * s.v[327]) + (eq20_e617 * s.dn[327][12]));let eq20_e619_d_n13: f64 = (((p.p31 * s.dn[13][13]) * s.v[327]) + (eq20_e617 * s.dn[327][13]));let eq20_e619_d_b0: f64 = (((p.p31 * s.db[13][0]) * s.v[327]) + (eq20_e617 * s.db[327][0]));let eq20_e619_d_b1: f64 = (((p.p31 * s.db[13][1]) * s.v[327]) + (eq20_e617 * s.db[327][1]));let eq20_e619_d_b2: f64 = (((p.p31 * s.db[13][2]) * s.v[327]) + (eq20_e617 * s.db[327][2]));let eq20_e619_d_b3: f64 = (((p.p31 * s.db[13][3]) * s.v[327]) + (eq20_e617 * s.db[327][3]));let eq20_e621: f64 = (eq20_e619 * (nv3 - nv8));let eq20_e621_d_n0: f64 = (eq20_e619_d_n0 * (nv3 - nv8));let eq20_e621_d_n1: f64 = (eq20_e619_d_n1 * (nv3 - nv8));let eq20_e621_d_n2: f64 = (eq20_e619_d_n2 * (nv3 - nv8));let eq20_e621_d_n3: f64 = ((eq20_e619_d_n3 * (nv3 - nv8)) + eq20_e619);let eq20_e621_d_n4: f64 = (eq20_e619_d_n4 * (nv3 - nv8));let eq20_e621_d_n5: f64 = (eq20_e619_d_n5 * (nv3 - nv8));let eq20_e621_d_n6: f64 = (eq20_e619_d_n6 * (nv3 - nv8));let eq20_e621_d_n7: f64 = (eq20_e619_d_n7 * (nv3 - nv8));let eq20_e621_d_n8: f64 = ((eq20_e619_d_n8 * (nv3 - nv8)) + (-eq20_e619));let eq20_e621_d_n9: f64 = (eq20_e619_d_n9 * (nv3 - nv8));let eq20_e621_d_n10: f64 = (eq20_e619_d_n10 * (nv3 - nv8));let eq20_e621_d_n11: f64 = (eq20_e619_d_n11 * (nv3 - nv8));let eq20_e621_d_n12: f64 = (eq20_e619_d_n12 * (nv3 - nv8));let eq20_e621_d_n13: f64 = (eq20_e619_d_n13 * (nv3 - nv8));let eq20_e621_d_b0: f64 = (eq20_e619_d_b0 * (nv3 - nv8));let eq20_e621_d_b1: f64 = (eq20_e619_d_b1 * (nv3 - nv8));let eq20_e621_d_b2: f64 = (eq20_e619_d_b2 * (nv3 - nv8));let eq20_e621_d_b3: f64 = (eq20_e619_d_b3 * (nv3 - nv8));
        (eq20_e621, eq20_e621_d_n0, eq20_e621_d_n1, eq20_e621_d_n2, eq20_e621_d_n3, eq20_e621_d_n4, eq20_e621_d_n5, eq20_e621_d_n6, eq20_e621_d_n7, eq20_e621_d_n8, eq20_e621_d_n9, eq20_e621_d_n10, eq20_e621_d_n11, eq20_e621_d_n12, eq20_e621_d_n13, eq20_e621_d_b0, eq20_e621_d_b1, eq20_e621_d_b2, eq20_e621_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e623;let eq20_node_derivatives: [f64; 14] = [eq20_e623_d_n0, eq20_e623_d_n1, eq20_e623_d_n2, eq20_e623_d_n3, eq20_e623_d_n4, eq20_e623_d_n5, eq20_e623_d_n6, eq20_e623_d_n7, eq20_e623_d_n8, eq20_e623_d_n9, eq20_e623_d_n10, eq20_e623_d_n11, eq20_e623_d_n12, eq20_e623_d_n13];let eq20_branch_derivatives: [f64; 4] = [eq20_e623_d_b0, eq20_e623_d_b1, eq20_e623_d_b2, eq20_e623_d_b3];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq22_e638,) = {
    if (!s.b[1771]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e638;
        stamper.stamp_potential_const_local(
            3,
            eq22_value,
        );let eq23_e642: f64 = (s.v[1774] + s.v[1775]);let eq23_e642_d_n0: f64 = (s.dn[1774][0] + s.dn[1775][0]);let eq23_e642_d_n1: f64 = (s.dn[1774][1] + s.dn[1775][1]);let eq23_e642_d_n2: f64 = (s.dn[1774][2] + s.dn[1775][2]);let eq23_e642_d_n3: f64 = (s.dn[1774][3] + s.dn[1775][3]);let eq23_e642_d_n4: f64 = (s.dn[1774][4] + s.dn[1775][4]);let eq23_e642_d_n5: f64 = (s.dn[1774][5] + s.dn[1775][5]);let eq23_e642_d_n6: f64 = (s.dn[1774][6] + s.dn[1775][6]);let eq23_e642_d_n7: f64 = (s.dn[1774][7] + s.dn[1775][7]);let eq23_e642_d_n8: f64 = (s.dn[1774][8] + s.dn[1775][8]);let eq23_e642_d_n9: f64 = (s.dn[1774][9] + s.dn[1775][9]);let eq23_e642_d_n10: f64 = (s.dn[1774][10] + s.dn[1775][10]);let eq23_e642_d_n11: f64 = (s.dn[1774][11] + s.dn[1775][11]);let eq23_e642_d_n12: f64 = (s.dn[1774][12] + s.dn[1775][12]);let eq23_e642_d_n13: f64 = (s.dn[1774][13] + s.dn[1775][13]);let eq23_e642_d_b0: f64 = (s.db[1774][0] + s.db[1775][0]);let eq23_e642_d_b1: f64 = (s.db[1774][1] + s.db[1775][1]);let eq23_e642_d_b2: f64 = (s.db[1774][2] + s.db[1775][2]);let eq23_e642_d_b3: f64 = (s.db[1774][3] + s.db[1775][3]);let eq23_e643: f64 = (s.v[181] * eq23_e642);let eq23_e643_d_n0: f64 = ((s.dn[181][0] * eq23_e642) + (s.v[181] * eq23_e642_d_n0));let eq23_e643_d_n1: f64 = ((s.dn[181][1] * eq23_e642) + (s.v[181] * eq23_e642_d_n1));let eq23_e643_d_n2: f64 = ((s.dn[181][2] * eq23_e642) + (s.v[181] * eq23_e642_d_n2));let eq23_e643_d_n3: f64 = ((s.dn[181][3] * eq23_e642) + (s.v[181] * eq23_e642_d_n3));let eq23_e643_d_n4: f64 = ((s.dn[181][4] * eq23_e642) + (s.v[181] * eq23_e642_d_n4));let eq23_e643_d_n5: f64 = ((s.dn[181][5] * eq23_e642) + (s.v[181] * eq23_e642_d_n5));let eq23_e643_d_n6: f64 = ((s.dn[181][6] * eq23_e642) + (s.v[181] * eq23_e642_d_n6));let eq23_e643_d_n7: f64 = ((s.dn[181][7] * eq23_e642) + (s.v[181] * eq23_e642_d_n7));let eq23_e643_d_n8: f64 = ((s.dn[181][8] * eq23_e642) + (s.v[181] * eq23_e642_d_n8));let eq23_e643_d_n9: f64 = ((s.dn[181][9] * eq23_e642) + (s.v[181] * eq23_e642_d_n9));let eq23_e643_d_n10: f64 = ((s.dn[181][10] * eq23_e642) + (s.v[181] * eq23_e642_d_n10));let eq23_e643_d_n11: f64 = ((s.dn[181][11] * eq23_e642) + (s.v[181] * eq23_e642_d_n11));let eq23_e643_d_n12: f64 = ((s.dn[181][12] * eq23_e642) + (s.v[181] * eq23_e642_d_n12));let eq23_e643_d_n13: f64 = ((s.dn[181][13] * eq23_e642) + (s.v[181] * eq23_e642_d_n13));let eq23_e643_d_b0: f64 = ((s.db[181][0] * eq23_e642) + (s.v[181] * eq23_e642_d_b0));let eq23_e643_d_b1: f64 = ((s.db[181][1] * eq23_e642) + (s.v[181] * eq23_e642_d_b1));let eq23_e643_d_b2: f64 = ((s.db[181][2] * eq23_e642) + (s.v[181] * eq23_e642_d_b2));let eq23_e643_d_b3: f64 = ((s.db[181][3] * eq23_e642) + (s.v[181] * eq23_e642_d_b3));let eq23_e644: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq23_e643);let eq23_value: f64 = eq23_e644;let eq23_node_derivatives: [f64; 14] = [(eq23_e643_d_n0 * ddt_scale), (eq23_e643_d_n1 * ddt_scale), (eq23_e643_d_n2 * ddt_scale), (eq23_e643_d_n3 * ddt_scale), (eq23_e643_d_n4 * ddt_scale), (eq23_e643_d_n5 * ddt_scale), (eq23_e643_d_n6 * ddt_scale), (eq23_e643_d_n7 * ddt_scale), (eq23_e643_d_n8 * ddt_scale), (eq23_e643_d_n9 * ddt_scale), (eq23_e643_d_n10 * ddt_scale), (eq23_e643_d_n11 * ddt_scale), (eq23_e643_d_n12 * ddt_scale), (eq23_e643_d_n13 * ddt_scale)];let eq23_branch_derivatives: [f64; 4] = [(eq23_e643_d_b0 * ddt_scale), (eq23_e643_d_b1 * ddt_scale), (eq23_e643_d_b2 * ddt_scale), (eq23_e643_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(13),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let eq24_e647: f64 = (s.v[1773] * (nv10 - nv13));let eq24_e647_d_n0: f64 = (s.dn[1773][0] * (nv10 - nv13));let eq24_e647_d_n1: f64 = (s.dn[1773][1] * (nv10 - nv13));let eq24_e647_d_n2: f64 = (s.dn[1773][2] * (nv10 - nv13));let eq24_e647_d_n3: f64 = (s.dn[1773][3] * (nv10 - nv13));let eq24_e647_d_n4: f64 = (s.dn[1773][4] * (nv10 - nv13));let eq24_e647_d_n5: f64 = (s.dn[1773][5] * (nv10 - nv13));let eq24_e647_d_n6: f64 = (s.dn[1773][6] * (nv10 - nv13));let eq24_e647_d_n7: f64 = (s.dn[1773][7] * (nv10 - nv13));let eq24_e647_d_n8: f64 = (s.dn[1773][8] * (nv10 - nv13));let eq24_e647_d_n9: f64 = (s.dn[1773][9] * (nv10 - nv13));let eq24_e647_d_n10: f64 = ((s.dn[1773][10] * (nv10 - nv13)) + s.v[1773]);let eq24_e647_d_n11: f64 = (s.dn[1773][11] * (nv10 - nv13));let eq24_e647_d_n12: f64 = (s.dn[1773][12] * (nv10 - nv13));let eq24_e647_d_n13: f64 = ((s.dn[1773][13] * (nv10 - nv13)) + (-s.v[1773]));let eq24_e647_d_b0: f64 = (s.db[1773][0] * (nv10 - nv13));let eq24_e647_d_b1: f64 = (s.db[1773][1] * (nv10 - nv13));let eq24_e647_d_b2: f64 = (s.db[1773][2] * (nv10 - nv13));let eq24_e647_d_b3: f64 = (s.db[1773][3] * (nv10 - nv13));let eq24_value: f64 = eq24_e647;let eq24_node_derivatives: [f64; 14] = [eq24_e647_d_n0, eq24_e647_d_n1, eq24_e647_d_n2, eq24_e647_d_n3, eq24_e647_d_n4, eq24_e647_d_n5, eq24_e647_d_n6, eq24_e647_d_n7, eq24_e647_d_n8, eq24_e647_d_n9, eq24_e647_d_n10, eq24_e647_d_n11, eq24_e647_d_n12, eq24_e647_d_n13];let eq24_branch_derivatives: [f64; 4] = [eq24_e647_d_b0, eq24_e647_d_b1, eq24_e647_d_b2, eq24_e647_d_b3];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(13),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );let eq25_e650: f64 = (1e-9 * (nv10 - nv13));let eq25_e651: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq25_e650);let eq25_value: f64 = eq25_e651;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(13),
            multiplicity * (eq25_value),
            10,
            multiplicity * ((1e-9 * ddt_scale)),
            13,
            multiplicity * (((-1e-9) * ddt_scale)),
        );let eq26_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[1776]);let eq26_value: f64 = eq26_e653;
        stamper.stamp_current_dense_local(
            Some(12),
            Some(13),
            multiplicity * (eq26_value),
            &s.dn[1776],
            &s.db[1776],
            (multiplicity) * (ddt_scale),
        );let eq27_e656: f64 = (s.v[1773] * (nv12 - nv13));let eq27_e656_d_n0: f64 = (s.dn[1773][0] * (nv12 - nv13));let eq27_e656_d_n1: f64 = (s.dn[1773][1] * (nv12 - nv13));let eq27_e656_d_n2: f64 = (s.dn[1773][2] * (nv12 - nv13));let eq27_e656_d_n3: f64 = (s.dn[1773][3] * (nv12 - nv13));let eq27_e656_d_n4: f64 = (s.dn[1773][4] * (nv12 - nv13));let eq27_e656_d_n5: f64 = (s.dn[1773][5] * (nv12 - nv13));let eq27_e656_d_n6: f64 = (s.dn[1773][6] * (nv12 - nv13));let eq27_e656_d_n7: f64 = (s.dn[1773][7] * (nv12 - nv13));let eq27_e656_d_n8: f64 = (s.dn[1773][8] * (nv12 - nv13));let eq27_e656_d_n9: f64 = (s.dn[1773][9] * (nv12 - nv13));let eq27_e656_d_n10: f64 = (s.dn[1773][10] * (nv12 - nv13));let eq27_e656_d_n11: f64 = (s.dn[1773][11] * (nv12 - nv13));let eq27_e656_d_n12: f64 = ((s.dn[1773][12] * (nv12 - nv13)) + s.v[1773]);let eq27_e656_d_n13: f64 = ((s.dn[1773][13] * (nv12 - nv13)) + (-s.v[1773]));let eq27_e656_d_b0: f64 = (s.db[1773][0] * (nv12 - nv13));let eq27_e656_d_b1: f64 = (s.db[1773][1] * (nv12 - nv13));let eq27_e656_d_b2: f64 = (s.db[1773][2] * (nv12 - nv13));let eq27_e656_d_b3: f64 = (s.db[1773][3] * (nv12 - nv13));let eq27_value: f64 = eq27_e656;let eq27_node_derivatives: [f64; 14] = [eq27_e656_d_n0, eq27_e656_d_n1, eq27_e656_d_n2, eq27_e656_d_n3, eq27_e656_d_n4, eq27_e656_d_n5, eq27_e656_d_n6, eq27_e656_d_n7, eq27_e656_d_n8, eq27_e656_d_n9, eq27_e656_d_n10, eq27_e656_d_n11, eq27_e656_d_n12, eq27_e656_d_n13];let eq27_branch_derivatives: [f64; 4] = [eq27_e656_d_b0, eq27_e656_d_b1, eq27_e656_d_b2, eq27_e656_d_b3];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(13),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );let eq28_e659: f64 = (1e-9 * (nv12 - nv13));let eq28_e660: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq28_e659);let eq28_value: f64 = eq28_e660;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(13),
            multiplicity * (eq28_value),
            12,
            multiplicity * ((1e-9 * ddt_scale)),
            13,
            multiplicity * (((-1e-9) * ddt_scale)),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let eq29_e662: f64 = (s.v[182]).sqrt();let __rspice_inv_cse_0: f64 = 1.0 / (2.0 * eq29_e662);let eq29_e662_d_n0: f64 = (s.dn[182][0] * __rspice_inv_cse_0);let eq29_e662_d_n1: f64 = (s.dn[182][1] * __rspice_inv_cse_0);let eq29_e662_d_n2: f64 = (s.dn[182][2] * __rspice_inv_cse_0);let eq29_e662_d_n3: f64 = (s.dn[182][3] * __rspice_inv_cse_0);let eq29_e662_d_n4: f64 = (s.dn[182][4] * __rspice_inv_cse_0);let eq29_e662_d_n5: f64 = (s.dn[182][5] * __rspice_inv_cse_0);let eq29_e662_d_n6: f64 = (s.dn[182][6] * __rspice_inv_cse_0);let eq29_e662_d_n7: f64 = (s.dn[182][7] * __rspice_inv_cse_0);let eq29_e662_d_n8: f64 = (s.dn[182][8] * __rspice_inv_cse_0);let eq29_e662_d_n9: f64 = (s.dn[182][9] * __rspice_inv_cse_0);let eq29_e662_d_n10: f64 = (s.dn[182][10] * __rspice_inv_cse_0);let eq29_e662_d_n11: f64 = (s.dn[182][11] * __rspice_inv_cse_0);let eq29_e662_d_n12: f64 = (s.dn[182][12] * __rspice_inv_cse_0);let eq29_e662_d_n13: f64 = (s.dn[182][13] * __rspice_inv_cse_0);let eq29_e662_d_b0: f64 = (s.db[182][0] * __rspice_inv_cse_0);let eq29_e662_d_b1: f64 = (s.db[182][1] * __rspice_inv_cse_0);let eq29_e662_d_b2: f64 = (s.db[182][2] * __rspice_inv_cse_0);let eq29_e662_d_b3: f64 = (s.db[182][3] * __rspice_inv_cse_0);let eq29_e665: f64 = (1.0 - s.v[181]);let eq29_e668: f64 = (s.v[1774] + s.v[1775]);let eq29_e668_d_n0: f64 = (s.dn[1774][0] + s.dn[1775][0]);let eq29_e668_d_n1: f64 = (s.dn[1774][1] + s.dn[1775][1]);let eq29_e668_d_n2: f64 = (s.dn[1774][2] + s.dn[1775][2]);let eq29_e668_d_n3: f64 = (s.dn[1774][3] + s.dn[1775][3]);let eq29_e668_d_n4: f64 = (s.dn[1774][4] + s.dn[1775][4]);let eq29_e668_d_n5: f64 = (s.dn[1774][5] + s.dn[1775][5]);let eq29_e668_d_n6: f64 = (s.dn[1774][6] + s.dn[1775][6]);let eq29_e668_d_n7: f64 = (s.dn[1774][7] + s.dn[1775][7]);let eq29_e668_d_n8: f64 = (s.dn[1774][8] + s.dn[1775][8]);let eq29_e668_d_n9: f64 = (s.dn[1774][9] + s.dn[1775][9]);let eq29_e668_d_n10: f64 = (s.dn[1774][10] + s.dn[1775][10]);let eq29_e668_d_n11: f64 = (s.dn[1774][11] + s.dn[1775][11]);let eq29_e668_d_n12: f64 = (s.dn[1774][12] + s.dn[1775][12]);let eq29_e668_d_n13: f64 = (s.dn[1774][13] + s.dn[1775][13]);let eq29_e668_d_b0: f64 = (s.db[1774][0] + s.db[1775][0]);let eq29_e668_d_b1: f64 = (s.db[1774][1] + s.db[1775][1]);let eq29_e668_d_b2: f64 = (s.db[1774][2] + s.db[1775][2]);let eq29_e668_d_b3: f64 = (s.db[1774][3] + s.db[1775][3]);let eq29_e669: f64 = (eq29_e665 * eq29_e668);let eq29_e669_d_n0: f64 = (((-s.dn[181][0]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n0));let eq29_e669_d_n1: f64 = (((-s.dn[181][1]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n1));let eq29_e669_d_n2: f64 = (((-s.dn[181][2]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n2));let eq29_e669_d_n3: f64 = (((-s.dn[181][3]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n3));let eq29_e669_d_n4: f64 = (((-s.dn[181][4]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n4));let eq29_e669_d_n5: f64 = (((-s.dn[181][5]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n5));let eq29_e669_d_n6: f64 = (((-s.dn[181][6]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n6));let eq29_e669_d_n7: f64 = (((-s.dn[181][7]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n7));let eq29_e669_d_n8: f64 = (((-s.dn[181][8]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n8));let eq29_e669_d_n9: f64 = (((-s.dn[181][9]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n9));let eq29_e669_d_n10: f64 = (((-s.dn[181][10]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n10));let eq29_e669_d_n11: f64 = (((-s.dn[181][11]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n11));let eq29_e669_d_n12: f64 = (((-s.dn[181][12]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n12));let eq29_e669_d_n13: f64 = (((-s.dn[181][13]) * eq29_e668) + (eq29_e665 * eq29_e668_d_n13));let eq29_e669_d_b0: f64 = (((-s.db[181][0]) * eq29_e668) + (eq29_e665 * eq29_e668_d_b0));let eq29_e669_d_b1: f64 = (((-s.db[181][1]) * eq29_e668) + (eq29_e665 * eq29_e668_d_b1));let eq29_e669_d_b2: f64 = (((-s.db[181][2]) * eq29_e668) + (eq29_e665 * eq29_e668_d_b2));let eq29_e669_d_b3: f64 = (((-s.db[181][3]) * eq29_e668) + (eq29_e665 * eq29_e668_d_b3));
        let eq29_e670: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq29_e669);let eq29_e671: f64 = (eq29_e662 * eq29_e670);let eq29_e671_d_n0: f64 = ((eq29_e662_d_n0 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n0 * ddt_scale)));let eq29_e671_d_n1: f64 = ((eq29_e662_d_n1 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n1 * ddt_scale)));let eq29_e671_d_n2: f64 = ((eq29_e662_d_n2 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n2 * ddt_scale)));let eq29_e671_d_n3: f64 = ((eq29_e662_d_n3 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n3 * ddt_scale)));let eq29_e671_d_n4: f64 = ((eq29_e662_d_n4 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n4 * ddt_scale)));let eq29_e671_d_n5: f64 = ((eq29_e662_d_n5 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n5 * ddt_scale)));let eq29_e671_d_n6: f64 = ((eq29_e662_d_n6 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n6 * ddt_scale)));let eq29_e671_d_n7: f64 = ((eq29_e662_d_n7 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n7 * ddt_scale)));let eq29_e671_d_n8: f64 = ((eq29_e662_d_n8 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n8 * ddt_scale)));let eq29_e671_d_n9: f64 = ((eq29_e662_d_n9 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n9 * ddt_scale)));let eq29_e671_d_n10: f64 = ((eq29_e662_d_n10 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n10 * ddt_scale)));let eq29_e671_d_n11: f64 = ((eq29_e662_d_n11 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n11 * ddt_scale)));let eq29_e671_d_n12: f64 = ((eq29_e662_d_n12 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n12 * ddt_scale)));let eq29_e671_d_n13: f64 = ((eq29_e662_d_n13 * eq29_e670) + (eq29_e662 * (eq29_e669_d_n13 * ddt_scale)));let eq29_e671_d_b0: f64 = ((eq29_e662_d_b0 * eq29_e670) + (eq29_e662 * (eq29_e669_d_b0 * ddt_scale)));let eq29_e671_d_b1: f64 = ((eq29_e662_d_b1 * eq29_e670) + (eq29_e662 * (eq29_e669_d_b1 * ddt_scale)));let eq29_e671_d_b2: f64 = ((eq29_e662_d_b2 * eq29_e670) + (eq29_e662 * (eq29_e669_d_b2 * ddt_scale)));let eq29_e671_d_b3: f64 = ((eq29_e662_d_b3 * eq29_e670) + (eq29_e662 * (eq29_e669_d_b3 * ddt_scale)));let eq29_value: f64 = eq29_e671;let eq29_node_derivatives: [f64; 14] = [eq29_e671_d_n0, eq29_e671_d_n1, eq29_e671_d_n2, eq29_e671_d_n3, eq29_e671_d_n4, eq29_e671_d_n5, eq29_e671_d_n6, eq29_e671_d_n7, eq29_e671_d_n8, eq29_e671_d_n9, eq29_e671_d_n10, eq29_e671_d_n11, eq29_e671_d_n12, eq29_e671_d_n13];let eq29_branch_derivatives: [f64; 4] = [eq29_e671_d_b0, eq29_e671_d_b1, eq29_e671_d_b2, eq29_e671_d_b3];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(13),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);let nv13 = ctx.node_voltage(nodes[13]);let eq30_e674: f64 = (s.v[1773] * (nv11 - nv13));let eq30_e674_d_n0: f64 = (s.dn[1773][0] * (nv11 - nv13));let eq30_e674_d_n1: f64 = (s.dn[1773][1] * (nv11 - nv13));let eq30_e674_d_n2: f64 = (s.dn[1773][2] * (nv11 - nv13));let eq30_e674_d_n3: f64 = (s.dn[1773][3] * (nv11 - nv13));let eq30_e674_d_n4: f64 = (s.dn[1773][4] * (nv11 - nv13));let eq30_e674_d_n5: f64 = (s.dn[1773][5] * (nv11 - nv13));let eq30_e674_d_n6: f64 = (s.dn[1773][6] * (nv11 - nv13));let eq30_e674_d_n7: f64 = (s.dn[1773][7] * (nv11 - nv13));let eq30_e674_d_n8: f64 = (s.dn[1773][8] * (nv11 - nv13));let eq30_e674_d_n9: f64 = (s.dn[1773][9] * (nv11 - nv13));let eq30_e674_d_n10: f64 = (s.dn[1773][10] * (nv11 - nv13));let eq30_e674_d_n11: f64 = ((s.dn[1773][11] * (nv11 - nv13)) + s.v[1773]);let eq30_e674_d_n12: f64 = (s.dn[1773][12] * (nv11 - nv13));let eq30_e674_d_n13: f64 = ((s.dn[1773][13] * (nv11 - nv13)) + (-s.v[1773]));let eq30_e674_d_b0: f64 = (s.db[1773][0] * (nv11 - nv13));let eq30_e674_d_b1: f64 = (s.db[1773][1] * (nv11 - nv13));let eq30_e674_d_b2: f64 = (s.db[1773][2] * (nv11 - nv13));let eq30_e674_d_b3: f64 = (s.db[1773][3] * (nv11 - nv13));let eq30_value: f64 = eq30_e674;let eq30_node_derivatives: [f64; 14] = [eq30_e674_d_n0, eq30_e674_d_n1, eq30_e674_d_n2, eq30_e674_d_n3, eq30_e674_d_n4, eq30_e674_d_n5, eq30_e674_d_n6, eq30_e674_d_n7, eq30_e674_d_n8, eq30_e674_d_n9, eq30_e674_d_n10, eq30_e674_d_n11, eq30_e674_d_n12, eq30_e674_d_n13];let eq30_branch_derivatives: [f64; 4] = [eq30_e674_d_b0, eq30_e674_d_b1, eq30_e674_d_b2, eq30_e674_d_b3];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(13),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );let eq31_e678: f64 = (1e-9 * (nv11 - nv13));let eq31_e679: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq31_e678);let eq31_e680: f64 = (s.v[182] * eq31_e679);let eq31_e680_d_n11: f64 = ((s.dn[182][11] * eq31_e679) + (s.v[182] * (1e-9 * ddt_scale)));let eq31_e680_d_n13: f64 = ((s.dn[182][13] * eq31_e679) + (s.v[182] * ((-1e-9) * ddt_scale)));let eq31_value: f64 = eq31_e680;let eq31_node_derivatives: [f64; 14] = [(s.dn[182][0] * eq31_e679), (s.dn[182][1] * eq31_e679), (s.dn[182][2] * eq31_e679), (s.dn[182][3] * eq31_e679), (s.dn[182][4] * eq31_e679), (s.dn[182][5] * eq31_e679), (s.dn[182][6] * eq31_e679), (s.dn[182][7] * eq31_e679), (s.dn[182][8] * eq31_e679), (s.dn[182][9] * eq31_e679), (s.dn[182][10] * eq31_e679), eq31_e680_d_n11, (s.dn[182][12] * eq31_e679), eq31_e680_d_n13];let eq31_branch_derivatives: [f64; 4] = [(s.db[182][0] * eq31_e679), (s.db[182][1] * eq31_e679), (s.db[182][2] * eq31_e679), (s.db[182][3] * eq31_e679)];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(13),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let eq32_e683: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[362]);let eq32_e685: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, s.v[377]);let eq32_e686: f64 = (eq32_e683 + eq32_e685);let eq32_e686_d_n0: f64 = ((s.dn[362][0] * ddt_scale) + (s.dn[377][0] * ddt_scale));let eq32_e686_d_n1: f64 = ((s.dn[362][1] * ddt_scale) + (s.dn[377][1] * ddt_scale));let eq32_e686_d_n2: f64 = ((s.dn[362][2] * ddt_scale) + (s.dn[377][2] * ddt_scale));let eq32_e686_d_n3: f64 = ((s.dn[362][3] * ddt_scale) + (s.dn[377][3] * ddt_scale));let eq32_e686_d_n4: f64 = ((s.dn[362][4] * ddt_scale) + (s.dn[377][4] * ddt_scale));let eq32_e686_d_n5: f64 = ((s.dn[362][5] * ddt_scale) + (s.dn[377][5] * ddt_scale));let eq32_e686_d_n6: f64 = ((s.dn[362][6] * ddt_scale) + (s.dn[377][6] * ddt_scale));let eq32_e686_d_n7: f64 = ((s.dn[362][7] * ddt_scale) + (s.dn[377][7] * ddt_scale));let eq32_e686_d_n8: f64 = ((s.dn[362][8] * ddt_scale) + (s.dn[377][8] * ddt_scale));let eq32_e686_d_n9: f64 = ((s.dn[362][9] * ddt_scale) + (s.dn[377][9] * ddt_scale));let eq32_e686_d_n10: f64 = ((s.dn[362][10] * ddt_scale) + (s.dn[377][10] * ddt_scale));let eq32_e686_d_n11: f64 = ((s.dn[362][11] * ddt_scale) + (s.dn[377][11] * ddt_scale));let eq32_e686_d_n12: f64 = ((s.dn[362][12] * ddt_scale) + (s.dn[377][12] * ddt_scale));let eq32_e686_d_n13: f64 = ((s.dn[362][13] * ddt_scale) + (s.dn[377][13] * ddt_scale));let eq32_e686_d_b0: f64 = ((s.db[362][0] * ddt_scale) + (s.db[377][0] * ddt_scale));let eq32_e686_d_b1: f64 = ((s.db[362][1] * ddt_scale) + (s.db[377][1] * ddt_scale));let eq32_e686_d_b2: f64 = ((s.db[362][2] * ddt_scale) + (s.db[377][2] * ddt_scale));let eq32_e686_d_b3: f64 = ((s.db[362][3] * ddt_scale) + (s.db[377][3] * ddt_scale));let eq32_e688: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, s.v[381]);let eq32_e689: f64 = (eq32_e686 + eq32_e688);let eq32_e689_d_n0: f64 = (eq32_e686_d_n0 + (s.dn[381][0] * ddt_scale));let eq32_e689_d_n1: f64 = (eq32_e686_d_n1 + (s.dn[381][1] * ddt_scale));let eq32_e689_d_n2: f64 = (eq32_e686_d_n2 + (s.dn[381][2] * ddt_scale));let eq32_e689_d_n3: f64 = (eq32_e686_d_n3 + (s.dn[381][3] * ddt_scale));let eq32_e689_d_n4: f64 = (eq32_e686_d_n4 + (s.dn[381][4] * ddt_scale));let eq32_e689_d_n5: f64 = (eq32_e686_d_n5 + (s.dn[381][5] * ddt_scale));let eq32_e689_d_n6: f64 = (eq32_e686_d_n6 + (s.dn[381][6] * ddt_scale));let eq32_e689_d_n7: f64 = (eq32_e686_d_n7 + (s.dn[381][7] * ddt_scale));let eq32_e689_d_n8: f64 = (eq32_e686_d_n8 + (s.dn[381][8] * ddt_scale));let eq32_e689_d_n9: f64 = (eq32_e686_d_n9 + (s.dn[381][9] * ddt_scale));let eq32_e689_d_n10: f64 = (eq32_e686_d_n10 + (s.dn[381][10] * ddt_scale));let eq32_e689_d_n11: f64 = (eq32_e686_d_n11 + (s.dn[381][11] * ddt_scale));let eq32_e689_d_n12: f64 = (eq32_e686_d_n12 + (s.dn[381][12] * ddt_scale));let eq32_e689_d_n13: f64 = (eq32_e686_d_n13 + (s.dn[381][13] * ddt_scale));let eq32_e689_d_b0: f64 = (eq32_e686_d_b0 + (s.db[381][0] * ddt_scale));let eq32_e689_d_b1: f64 = (eq32_e686_d_b1 + (s.db[381][1] * ddt_scale));let eq32_e689_d_b2: f64 = (eq32_e686_d_b2 + (s.db[381][2] * ddt_scale));let eq32_e689_d_b3: f64 = (eq32_e686_d_b3 + (s.db[381][3] * ddt_scale));let eq32_e690: f64 = (p.p14 * eq32_e689);let eq32_e690_d_n0: f64 = (p.p14 * eq32_e689_d_n0);let eq32_e690_d_n1: f64 = (p.p14 * eq32_e689_d_n1);let eq32_e690_d_n2: f64 = (p.p14 * eq32_e689_d_n2);let eq32_e690_d_n3: f64 = (p.p14 * eq32_e689_d_n3);let eq32_e690_d_n4: f64 = (p.p14 * eq32_e689_d_n4);
        let eq32_e690_d_n5: f64 = (p.p14 * eq32_e689_d_n5);let eq32_e690_d_n6: f64 = (p.p14 * eq32_e689_d_n6);let eq32_e690_d_n7: f64 = (p.p14 * eq32_e689_d_n7);let eq32_e690_d_n8: f64 = (p.p14 * eq32_e689_d_n8);let eq32_e690_d_n9: f64 = (p.p14 * eq32_e689_d_n9);let eq32_e690_d_n10: f64 = (p.p14 * eq32_e689_d_n10);let eq32_e690_d_n11: f64 = (p.p14 * eq32_e689_d_n11);let eq32_e690_d_n12: f64 = (p.p14 * eq32_e689_d_n12);let eq32_e690_d_n13: f64 = (p.p14 * eq32_e689_d_n13);let eq32_e690_d_b0: f64 = (p.p14 * eq32_e689_d_b0);let eq32_e690_d_b1: f64 = (p.p14 * eq32_e689_d_b1);let eq32_e690_d_b2: f64 = (p.p14 * eq32_e689_d_b2);let eq32_e690_d_b3: f64 = (p.p14 * eq32_e689_d_b3);let eq32_value: f64 = eq32_e690;let eq32_node_derivatives: [f64; 14] = [eq32_e690_d_n0, eq32_e690_d_n1, eq32_e690_d_n2, eq32_e690_d_n3, eq32_e690_d_n4, eq32_e690_d_n5, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9, eq32_e690_d_n10, eq32_e690_d_n11, eq32_e690_d_n12, eq32_e690_d_n13];let eq32_branch_derivatives: [f64; 4] = [eq32_e690_d_b0, eq32_e690_d_b1, eq32_e690_d_b2, eq32_e690_d_b3];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_8(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let eq33_e693: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, s.v[371]);let eq33_e695: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[373]);let eq33_e696: f64 = (eq33_e693 + eq33_e695);let eq33_e696_d_n0: f64 = ((s.dn[371][0] * ddt_scale) + (s.dn[373][0] * ddt_scale));let eq33_e696_d_n1: f64 = ((s.dn[371][1] * ddt_scale) + (s.dn[373][1] * ddt_scale));let eq33_e696_d_n2: f64 = ((s.dn[371][2] * ddt_scale) + (s.dn[373][2] * ddt_scale));let eq33_e696_d_n3: f64 = ((s.dn[371][3] * ddt_scale) + (s.dn[373][3] * ddt_scale));let eq33_e696_d_n4: f64 = ((s.dn[371][4] * ddt_scale) + (s.dn[373][4] * ddt_scale));let eq33_e696_d_n5: f64 = ((s.dn[371][5] * ddt_scale) + (s.dn[373][5] * ddt_scale));let eq33_e696_d_n6: f64 = ((s.dn[371][6] * ddt_scale) + (s.dn[373][6] * ddt_scale));let eq33_e696_d_n7: f64 = ((s.dn[371][7] * ddt_scale) + (s.dn[373][7] * ddt_scale));let eq33_e696_d_n8: f64 = ((s.dn[371][8] * ddt_scale) + (s.dn[373][8] * ddt_scale));let eq33_e696_d_n9: f64 = ((s.dn[371][9] * ddt_scale) + (s.dn[373][9] * ddt_scale));let eq33_e696_d_n10: f64 = ((s.dn[371][10] * ddt_scale) + (s.dn[373][10] * ddt_scale));let eq33_e696_d_n11: f64 = ((s.dn[371][11] * ddt_scale) + (s.dn[373][11] * ddt_scale));let eq33_e696_d_n12: f64 = ((s.dn[371][12] * ddt_scale) + (s.dn[373][12] * ddt_scale));let eq33_e696_d_n13: f64 = ((s.dn[371][13] * ddt_scale) + (s.dn[373][13] * ddt_scale));let eq33_e696_d_b0: f64 = ((s.db[371][0] * ddt_scale) + (s.db[373][0] * ddt_scale));let eq33_e696_d_b1: f64 = ((s.db[371][1] * ddt_scale) + (s.db[373][1] * ddt_scale));let eq33_e696_d_b2: f64 = ((s.db[371][2] * ddt_scale) + (s.db[373][2] * ddt_scale));let eq33_e696_d_b3: f64 = ((s.db[371][3] * ddt_scale) + (s.db[373][3] * ddt_scale));let eq33_e698: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, s.v[380]);let eq33_e699: f64 = (eq33_e696 + eq33_e698);let eq33_e699_d_n0: f64 = (eq33_e696_d_n0 + (s.dn[380][0] * ddt_scale));let eq33_e699_d_n1: f64 = (eq33_e696_d_n1 + (s.dn[380][1] * ddt_scale));let eq33_e699_d_n2: f64 = (eq33_e696_d_n2 + (s.dn[380][2] * ddt_scale));let eq33_e699_d_n3: f64 = (eq33_e696_d_n3 + (s.dn[380][3] * ddt_scale));let eq33_e699_d_n4: f64 = (eq33_e696_d_n4 + (s.dn[380][4] * ddt_scale));let eq33_e699_d_n5: f64 = (eq33_e696_d_n5 + (s.dn[380][5] * ddt_scale));let eq33_e699_d_n6: f64 = (eq33_e696_d_n6 + (s.dn[380][6] * ddt_scale));let eq33_e699_d_n7: f64 = (eq33_e696_d_n7 + (s.dn[380][7] * ddt_scale));let eq33_e699_d_n8: f64 = (eq33_e696_d_n8 + (s.dn[380][8] * ddt_scale));let eq33_e699_d_n9: f64 = (eq33_e696_d_n9 + (s.dn[380][9] * ddt_scale));let eq33_e699_d_n10: f64 = (eq33_e696_d_n10 + (s.dn[380][10] * ddt_scale));let eq33_e699_d_n11: f64 = (eq33_e696_d_n11 + (s.dn[380][11] * ddt_scale));let eq33_e699_d_n12: f64 = (eq33_e696_d_n12 + (s.dn[380][12] * ddt_scale));let eq33_e699_d_n13: f64 = (eq33_e696_d_n13 + (s.dn[380][13] * ddt_scale));let eq33_e699_d_b0: f64 = (eq33_e696_d_b0 + (s.db[380][0] * ddt_scale));let eq33_e699_d_b1: f64 = (eq33_e696_d_b1 + (s.db[380][1] * ddt_scale));let eq33_e699_d_b2: f64 = (eq33_e696_d_b2 + (s.db[380][2] * ddt_scale));let eq33_e699_d_b3: f64 = (eq33_e696_d_b3 + (s.db[380][3] * ddt_scale));let eq33_e700: f64 = (p.p14 * eq33_e699);let eq33_e700_d_n0: f64 = (p.p14 * eq33_e699_d_n0);let eq33_e700_d_n1: f64 = (p.p14 * eq33_e699_d_n1);let eq33_e700_d_n2: f64 = (p.p14 * eq33_e699_d_n2);let eq33_e700_d_n3: f64 = (p.p14 * eq33_e699_d_n3);let eq33_e700_d_n4: f64 = (p.p14 * eq33_e699_d_n4);
        let eq33_e700_d_n5: f64 = (p.p14 * eq33_e699_d_n5);let eq33_e700_d_n6: f64 = (p.p14 * eq33_e699_d_n6);let eq33_e700_d_n7: f64 = (p.p14 * eq33_e699_d_n7);let eq33_e700_d_n8: f64 = (p.p14 * eq33_e699_d_n8);let eq33_e700_d_n9: f64 = (p.p14 * eq33_e699_d_n9);let eq33_e700_d_n10: f64 = (p.p14 * eq33_e699_d_n10);let eq33_e700_d_n11: f64 = (p.p14 * eq33_e699_d_n11);let eq33_e700_d_n12: f64 = (p.p14 * eq33_e699_d_n12);let eq33_e700_d_n13: f64 = (p.p14 * eq33_e699_d_n13);let eq33_e700_d_b0: f64 = (p.p14 * eq33_e699_d_b0);let eq33_e700_d_b1: f64 = (p.p14 * eq33_e699_d_b1);let eq33_e700_d_b2: f64 = (p.p14 * eq33_e699_d_b2);let eq33_e700_d_b3: f64 = (p.p14 * eq33_e699_d_b3);let eq33_value: f64 = eq33_e700;let eq33_node_derivatives: [f64; 14] = [eq33_e700_d_n0, eq33_e700_d_n1, eq33_e700_d_n2, eq33_e700_d_n3, eq33_e700_d_n4, eq33_e700_d_n5, eq33_e700_d_n6, eq33_e700_d_n7, eq33_e700_d_n8, eq33_e700_d_n9, eq33_e700_d_n10, eq33_e700_d_n11, eq33_e700_d_n12, eq33_e700_d_n13];let eq33_branch_derivatives: [f64; 4] = [eq33_e700_d_b0, eq33_e700_d_b1, eq33_e700_d_b2, eq33_e700_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_9(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let eq34_e703: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, s.v[376]);let eq34_e705: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, s.v[382]);let eq34_e706: f64 = (eq34_e703 + eq34_e705);let eq34_e706_d_n0: f64 = ((s.dn[376][0] * ddt_scale) + (s.dn[382][0] * ddt_scale));let eq34_e706_d_n1: f64 = ((s.dn[376][1] * ddt_scale) + (s.dn[382][1] * ddt_scale));let eq34_e706_d_n2: f64 = ((s.dn[376][2] * ddt_scale) + (s.dn[382][2] * ddt_scale));let eq34_e706_d_n3: f64 = ((s.dn[376][3] * ddt_scale) + (s.dn[382][3] * ddt_scale));let eq34_e706_d_n4: f64 = ((s.dn[376][4] * ddt_scale) + (s.dn[382][4] * ddt_scale));let eq34_e706_d_n5: f64 = ((s.dn[376][5] * ddt_scale) + (s.dn[382][5] * ddt_scale));let eq34_e706_d_n6: f64 = ((s.dn[376][6] * ddt_scale) + (s.dn[382][6] * ddt_scale));let eq34_e706_d_n7: f64 = ((s.dn[376][7] * ddt_scale) + (s.dn[382][7] * ddt_scale));let eq34_e706_d_n8: f64 = ((s.dn[376][8] * ddt_scale) + (s.dn[382][8] * ddt_scale));let eq34_e706_d_n9: f64 = ((s.dn[376][9] * ddt_scale) + (s.dn[382][9] * ddt_scale));let eq34_e706_d_n10: f64 = ((s.dn[376][10] * ddt_scale) + (s.dn[382][10] * ddt_scale));let eq34_e706_d_n11: f64 = ((s.dn[376][11] * ddt_scale) + (s.dn[382][11] * ddt_scale));let eq34_e706_d_n12: f64 = ((s.dn[376][12] * ddt_scale) + (s.dn[382][12] * ddt_scale));let eq34_e706_d_n13: f64 = ((s.dn[376][13] * ddt_scale) + (s.dn[382][13] * ddt_scale));let eq34_e706_d_b0: f64 = ((s.db[376][0] * ddt_scale) + (s.db[382][0] * ddt_scale));let eq34_e706_d_b1: f64 = ((s.db[376][1] * ddt_scale) + (s.db[382][1] * ddt_scale));let eq34_e706_d_b2: f64 = ((s.db[376][2] * ddt_scale) + (s.db[382][2] * ddt_scale));let eq34_e706_d_b3: f64 = ((s.db[376][3] * ddt_scale) + (s.db[382][3] * ddt_scale));let eq34_e707: f64 = (p.p14 * eq34_e706);let eq34_e707_d_n0: f64 = (p.p14 * eq34_e706_d_n0);let eq34_e707_d_n1: f64 = (p.p14 * eq34_e706_d_n1);let eq34_e707_d_n2: f64 = (p.p14 * eq34_e706_d_n2);let eq34_e707_d_n3: f64 = (p.p14 * eq34_e706_d_n3);let eq34_e707_d_n4: f64 = (p.p14 * eq34_e706_d_n4);let eq34_e707_d_n5: f64 = (p.p14 * eq34_e706_d_n5);let eq34_e707_d_n6: f64 = (p.p14 * eq34_e706_d_n6);let eq34_e707_d_n7: f64 = (p.p14 * eq34_e706_d_n7);let eq34_e707_d_n8: f64 = (p.p14 * eq34_e706_d_n8);let eq34_e707_d_n9: f64 = (p.p14 * eq34_e706_d_n9);let eq34_e707_d_n10: f64 = (p.p14 * eq34_e706_d_n10);let eq34_e707_d_n11: f64 = (p.p14 * eq34_e706_d_n11);let eq34_e707_d_n12: f64 = (p.p14 * eq34_e706_d_n12);let eq34_e707_d_n13: f64 = (p.p14 * eq34_e706_d_n13);let eq34_e707_d_b0: f64 = (p.p14 * eq34_e706_d_b0);let eq34_e707_d_b1: f64 = (p.p14 * eq34_e706_d_b1);let eq34_e707_d_b2: f64 = (p.p14 * eq34_e706_d_b2);let eq34_e707_d_b3: f64 = (p.p14 * eq34_e706_d_b3);let eq34_value: f64 = eq34_e707;let eq34_node_derivatives: [f64; 14] = [eq34_e707_d_n0, eq34_e707_d_n1, eq34_e707_d_n2, eq34_e707_d_n3, eq34_e707_d_n4, eq34_e707_d_n5, eq34_e707_d_n6, eq34_e707_d_n7, eq34_e707_d_n8, eq34_e707_d_n9, eq34_e707_d_n10, eq34_e707_d_n11, eq34_e707_d_n12, eq34_e707_d_n13];let eq34_branch_derivatives: [f64; 4] = [eq34_e707_d_b0, eq34_e707_d_b1, eq34_e707_d_b2, eq34_e707_d_b3];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );let eq35_e710: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, s.v[374]);let eq35_e711: f64 = (p.p14 * eq35_e710);let eq35_e711_d_n0: f64 = (p.p14 * (s.dn[374][0] * ddt_scale));let eq35_e711_d_n1: f64 = (p.p14 * (s.dn[374][1] * ddt_scale));let eq35_e711_d_n2: f64 = (p.p14 * (s.dn[374][2] * ddt_scale));let eq35_e711_d_n3: f64 = (p.p14 * (s.dn[374][3] * ddt_scale));let eq35_e711_d_n4: f64 = (p.p14 * (s.dn[374][4] * ddt_scale));let eq35_e711_d_n5: f64 = (p.p14 * (s.dn[374][5] * ddt_scale));let eq35_e711_d_n6: f64 = (p.p14 * (s.dn[374][6] * ddt_scale));let eq35_e711_d_n7: f64 = (p.p14 * (s.dn[374][7] * ddt_scale));let eq35_e711_d_n8: f64 = (p.p14 * (s.dn[374][8] * ddt_scale));let eq35_e711_d_n9: f64 = (p.p14 * (s.dn[374][9] * ddt_scale));let eq35_e711_d_n10: f64 = (p.p14 * (s.dn[374][10] * ddt_scale));let eq35_e711_d_n11: f64 = (p.p14 * (s.dn[374][11] * ddt_scale));let eq35_e711_d_n12: f64 = (p.p14 * (s.dn[374][12] * ddt_scale));let eq35_e711_d_n13: f64 = (p.p14 * (s.dn[374][13] * ddt_scale));let eq35_e711_d_b0: f64 = (p.p14 * (s.db[374][0] * ddt_scale));let eq35_e711_d_b1: f64 = (p.p14 * (s.db[374][1] * ddt_scale));let eq35_e711_d_b2: f64 = (p.p14 * (s.db[374][2] * ddt_scale));let eq35_e711_d_b3: f64 = (p.p14 * (s.db[374][3] * ddt_scale));let eq35_value: f64 = eq35_e711;let eq35_node_derivatives: [f64; 14] = [eq35_e711_d_n0, eq35_e711_d_n1, eq35_e711_d_n2, eq35_e711_d_n3, eq35_e711_d_n4, eq35_e711_d_n5, eq35_e711_d_n6, eq35_e711_d_n7, eq35_e711_d_n8, eq35_e711_d_n9, eq35_e711_d_n10, eq35_e711_d_n11, eq35_e711_d_n12, eq35_e711_d_n13];let eq35_branch_derivatives: [f64; 4] = [eq35_e711_d_b0, eq35_e711_d_b1, eq35_e711_d_b2, eq35_e711_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_10(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);let nv13 = ctx.node_voltage(nodes[13]);let eq36_e714: f64 = (-s.v[1773]);let eq36_e716: f64 = (eq36_e714 * p.p32);let eq36_e716_d_n0: f64 = ((-s.dn[1773][0]) * p.p32);let eq36_e716_d_n1: f64 = ((-s.dn[1773][1]) * p.p32);let eq36_e716_d_n2: f64 = ((-s.dn[1773][2]) * p.p32);let eq36_e716_d_n3: f64 = ((-s.dn[1773][3]) * p.p32);let eq36_e716_d_n4: f64 = ((-s.dn[1773][4]) * p.p32);let eq36_e716_d_n5: f64 = ((-s.dn[1773][5]) * p.p32);let eq36_e716_d_n6: f64 = ((-s.dn[1773][6]) * p.p32);let eq36_e716_d_n7: f64 = ((-s.dn[1773][7]) * p.p32);let eq36_e716_d_n8: f64 = ((-s.dn[1773][8]) * p.p32);let eq36_e716_d_n9: f64 = ((-s.dn[1773][9]) * p.p32);let eq36_e716_d_n10: f64 = ((-s.dn[1773][10]) * p.p32);let eq36_e716_d_n11: f64 = ((-s.dn[1773][11]) * p.p32);let eq36_e716_d_n12: f64 = ((-s.dn[1773][12]) * p.p32);let eq36_e716_d_n13: f64 = ((-s.dn[1773][13]) * p.p32);let eq36_e716_d_b0: f64 = ((-s.db[1773][0]) * p.p32);let eq36_e716_d_b1: f64 = ((-s.db[1773][1]) * p.p32);let eq36_e716_d_b2: f64 = ((-s.db[1773][2]) * p.p32);let eq36_e716_d_b3: f64 = ((-s.db[1773][3]) * p.p32);let eq36_e718: f64 = (eq36_e716 * s.v[13]);let eq36_e718_d_n0: f64 = ((eq36_e716_d_n0 * s.v[13]) + (eq36_e716 * s.dn[13][0]));let eq36_e718_d_n1: f64 = ((eq36_e716_d_n1 * s.v[13]) + (eq36_e716 * s.dn[13][1]));let eq36_e718_d_n2: f64 = ((eq36_e716_d_n2 * s.v[13]) + (eq36_e716 * s.dn[13][2]));let eq36_e718_d_n3: f64 = ((eq36_e716_d_n3 * s.v[13]) + (eq36_e716 * s.dn[13][3]));let eq36_e718_d_n4: f64 = ((eq36_e716_d_n4 * s.v[13]) + (eq36_e716 * s.dn[13][4]));let eq36_e718_d_n5: f64 = ((eq36_e716_d_n5 * s.v[13]) + (eq36_e716 * s.dn[13][5]));let eq36_e718_d_n6: f64 = ((eq36_e716_d_n6 * s.v[13]) + (eq36_e716 * s.dn[13][6]));let eq36_e718_d_n7: f64 = ((eq36_e716_d_n7 * s.v[13]) + (eq36_e716 * s.dn[13][7]));let eq36_e718_d_n8: f64 = ((eq36_e716_d_n8 * s.v[13]) + (eq36_e716 * s.dn[13][8]));let eq36_e718_d_n9: f64 = ((eq36_e716_d_n9 * s.v[13]) + (eq36_e716 * s.dn[13][9]));let eq36_e718_d_n10: f64 = ((eq36_e716_d_n10 * s.v[13]) + (eq36_e716 * s.dn[13][10]));let eq36_e718_d_n11: f64 = ((eq36_e716_d_n11 * s.v[13]) + (eq36_e716 * s.dn[13][11]));let eq36_e718_d_n12: f64 = ((eq36_e716_d_n12 * s.v[13]) + (eq36_e716 * s.dn[13][12]));let eq36_e718_d_n13: f64 = ((eq36_e716_d_n13 * s.v[13]) + (eq36_e716 * s.dn[13][13]));let eq36_e718_d_b0: f64 = ((eq36_e716_d_b0 * s.v[13]) + (eq36_e716 * s.db[13][0]));let eq36_e718_d_b1: f64 = ((eq36_e716_d_b1 * s.v[13]) + (eq36_e716 * s.db[13][1]));let eq36_e718_d_b2: f64 = ((eq36_e716_d_b2 * s.v[13]) + (eq36_e716 * s.db[13][2]));let eq36_e718_d_b3: f64 = ((eq36_e716_d_b3 * s.v[13]) + (eq36_e716 * s.db[13][3]));let eq36_e722: f64 = (s.v[182]).sqrt();let __rspice_inv_cse_0: f64 = 1.0 / (2.0 * eq36_e722);let eq36_e722_d_n0: f64 = (s.dn[182][0] * __rspice_inv_cse_0);let eq36_e722_d_n1: f64 = (s.dn[182][1] * __rspice_inv_cse_0);let eq36_e722_d_n2: f64 = (s.dn[182][2] * __rspice_inv_cse_0);let eq36_e722_d_n3: f64 = (s.dn[182][3] * __rspice_inv_cse_0);let eq36_e722_d_n4: f64 = (s.dn[182][4] * __rspice_inv_cse_0);let eq36_e722_d_n5: f64 = (s.dn[182][5] * __rspice_inv_cse_0);let eq36_e722_d_n6: f64 = (s.dn[182][6] * __rspice_inv_cse_0);let eq36_e722_d_n7: f64 = (s.dn[182][7] * __rspice_inv_cse_0);let eq36_e722_d_n8: f64 = (s.dn[182][8] * __rspice_inv_cse_0);let eq36_e722_d_n9: f64 = (s.dn[182][9] * __rspice_inv_cse_0);let eq36_e722_d_n10: f64 = (s.dn[182][10] * __rspice_inv_cse_0);let eq36_e722_d_n11: f64 = (s.dn[182][11] * __rspice_inv_cse_0);let eq36_e722_d_n12: f64 = (s.dn[182][12] * __rspice_inv_cse_0);let eq36_e722_d_n13: f64 = (s.dn[182][13] * __rspice_inv_cse_0);let eq36_e722_d_b0: f64 = (s.db[182][0] * __rspice_inv_cse_0);let eq36_e722_d_b1: f64 = (s.db[182][1] * __rspice_inv_cse_0);let eq36_e722_d_b2: f64 = (s.db[182][2] * __rspice_inv_cse_0);let eq36_e722_d_b3: f64 = (s.db[182][3] * __rspice_inv_cse_0);let eq36_e723: f64 = ((nv11 - nv13) / eq36_e722);let eq36_e723_d_n0: f64 = (-(((nv11 - nv13) * eq36_e722_d_n0) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n1: f64 = (-(((nv11 - nv13) * eq36_e722_d_n1) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n2: f64 = (-(((nv11 - nv13) * eq36_e722_d_n2) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n3: f64 = (-(((nv11 - nv13) * eq36_e722_d_n3) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n4: f64 = (-(((nv11 - nv13) * eq36_e722_d_n4) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n5: f64 = (-(((nv11 - nv13) * eq36_e722_d_n5) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n6: f64 = (-(((nv11 - nv13) * eq36_e722_d_n6) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n7: f64 = (-(((nv11 - nv13) * eq36_e722_d_n7) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n8: f64 = (-(((nv11 - nv13) * eq36_e722_d_n8) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n9: f64 = (-(((nv11 - nv13) * eq36_e722_d_n9) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n10: f64 = (-(((nv11 - nv13) * eq36_e722_d_n10) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n11: f64 = ((eq36_e722 - ((nv11 - nv13) * eq36_e722_d_n11)) / (eq36_e722 * eq36_e722));let eq36_e723_d_n12: f64 = (-(((nv11 - nv13) * eq36_e722_d_n12) / (eq36_e722 * eq36_e722)));let eq36_e723_d_n13: f64 = (((-eq36_e722) - ((nv11 - nv13) * eq36_e722_d_n13)) / (eq36_e722 * eq36_e722));let eq36_e723_d_b0: f64 = (-(((nv11 - nv13) * eq36_e722_d_b0) / (eq36_e722 * eq36_e722)));let eq36_e723_d_b1: f64 = (-(((nv11 - nv13) * eq36_e722_d_b1) / (eq36_e722 * eq36_e722)));let eq36_e723_d_b2: f64 = (-(((nv11 - nv13) * eq36_e722_d_b2) / (eq36_e722 * eq36_e722)));let eq36_e723_d_b3: f64 = (-(((nv11 - nv13) * eq36_e722_d_b3) / (eq36_e722 * eq36_e722)));let eq36_e724: f64 = ((nv10 - nv13) + eq36_e723);let eq36_e724_d_n10: f64 = (1.0 + eq36_e723_d_n10);let eq36_e724_d_n13: f64 = (-1.0 + eq36_e723_d_n13);let eq36_e725: f64 = (eq36_e718 * eq36_e724);let eq36_e725_d_n0: f64 = ((eq36_e718_d_n0 * eq36_e724) + (eq36_e718 * eq36_e723_d_n0));let eq36_e725_d_n1: f64 = ((eq36_e718_d_n1 * eq36_e724) + (eq36_e718 * eq36_e723_d_n1));let eq36_e725_d_n2: f64 = ((eq36_e718_d_n2 * eq36_e724) + (eq36_e718 * eq36_e723_d_n2));let eq36_e725_d_n3: f64 = ((eq36_e718_d_n3 * eq36_e724) + (eq36_e718 * eq36_e723_d_n3));let eq36_e725_d_n4: f64 = ((eq36_e718_d_n4 * eq36_e724) + (eq36_e718 * eq36_e723_d_n4));let eq36_e725_d_n5: f64 = ((eq36_e718_d_n5 * eq36_e724) + (eq36_e718 * eq36_e723_d_n5));let eq36_e725_d_n6: f64 = ((eq36_e718_d_n6 * eq36_e724) + (eq36_e718 * eq36_e723_d_n6));let eq36_e725_d_n7: f64 = ((eq36_e718_d_n7 * eq36_e724) + (eq36_e718 * eq36_e723_d_n7));let eq36_e725_d_n8: f64 = ((eq36_e718_d_n8 * eq36_e724) + (eq36_e718 * eq36_e723_d_n8));let eq36_e725_d_n9: f64 = ((eq36_e718_d_n9 * eq36_e724) + (eq36_e718 * eq36_e723_d_n9));let eq36_e725_d_n10: f64 = ((eq36_e718_d_n10 * eq36_e724) + (eq36_e718 * eq36_e724_d_n10));let eq36_e725_d_n11: f64 = ((eq36_e718_d_n11 * eq36_e724) + (eq36_e718 * eq36_e723_d_n11));let eq36_e725_d_n12: f64 = ((eq36_e718_d_n12 * eq36_e724) + (eq36_e718 * eq36_e723_d_n12));let eq36_e725_d_n13: f64 = ((eq36_e718_d_n13 * eq36_e724) + (eq36_e718 * eq36_e724_d_n13));let eq36_e725_d_b0: f64 = ((eq36_e718_d_b0 * eq36_e724) + (eq36_e718 * eq36_e723_d_b0));let eq36_e725_d_b1: f64 = ((eq36_e718_d_b1 * eq36_e724) + (eq36_e718 * eq36_e723_d_b1));let eq36_e725_d_b2: f64 = ((eq36_e718_d_b2 * eq36_e724) + (eq36_e718 * eq36_e723_d_b2));let eq36_e725_d_b3: f64 = ((eq36_e718_d_b3 * eq36_e724) + (eq36_e718 * eq36_e723_d_b3));let eq36_e727: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, s.v[362]);let eq36_e728: f64 = (eq36_e725 - eq36_e727);let eq36_e728_d_n0: f64 = (eq36_e725_d_n0 - (s.dn[362][0] * ddt_scale));let eq36_e728_d_n1: f64 = (eq36_e725_d_n1 - (s.dn[362][1] * ddt_scale));let eq36_e728_d_n2: f64 = (eq36_e725_d_n2 - (s.dn[362][2] * ddt_scale));let eq36_e728_d_n3: f64 = (eq36_e725_d_n3 - (s.dn[362][3] * ddt_scale));let eq36_e728_d_n4: f64 = (eq36_e725_d_n4 - (s.dn[362][4] * ddt_scale));
        let eq36_e728_d_n5: f64 = (eq36_e725_d_n5 - (s.dn[362][5] * ddt_scale));let eq36_e728_d_n6: f64 = (eq36_e725_d_n6 - (s.dn[362][6] * ddt_scale));let eq36_e728_d_n7: f64 = (eq36_e725_d_n7 - (s.dn[362][7] * ddt_scale));let eq36_e728_d_n8: f64 = (eq36_e725_d_n8 - (s.dn[362][8] * ddt_scale));let eq36_e728_d_n9: f64 = (eq36_e725_d_n9 - (s.dn[362][9] * ddt_scale));let eq36_e728_d_n10: f64 = (eq36_e725_d_n10 - (s.dn[362][10] * ddt_scale));let eq36_e728_d_n11: f64 = (eq36_e725_d_n11 - (s.dn[362][11] * ddt_scale));let eq36_e728_d_n12: f64 = (eq36_e725_d_n12 - (s.dn[362][12] * ddt_scale));let eq36_e728_d_n13: f64 = (eq36_e725_d_n13 - (s.dn[362][13] * ddt_scale));let eq36_e728_d_b0: f64 = (eq36_e725_d_b0 - (s.db[362][0] * ddt_scale));let eq36_e728_d_b1: f64 = (eq36_e725_d_b1 - (s.db[362][1] * ddt_scale));let eq36_e728_d_b2: f64 = (eq36_e725_d_b2 - (s.db[362][2] * ddt_scale));let eq36_e728_d_b3: f64 = (eq36_e725_d_b3 - (s.db[362][3] * ddt_scale));let eq36_e730: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, s.v[370]);let eq36_e731: f64 = (eq36_e728 + eq36_e730);let eq36_e731_d_n0: f64 = (eq36_e728_d_n0 + (s.dn[370][0] * ddt_scale));let eq36_e731_d_n1: f64 = (eq36_e728_d_n1 + (s.dn[370][1] * ddt_scale));let eq36_e731_d_n2: f64 = (eq36_e728_d_n2 + (s.dn[370][2] * ddt_scale));let eq36_e731_d_n3: f64 = (eq36_e728_d_n3 + (s.dn[370][3] * ddt_scale));let eq36_e731_d_n4: f64 = (eq36_e728_d_n4 + (s.dn[370][4] * ddt_scale));let eq36_e731_d_n5: f64 = (eq36_e728_d_n5 + (s.dn[370][5] * ddt_scale));let eq36_e731_d_n6: f64 = (eq36_e728_d_n6 + (s.dn[370][6] * ddt_scale));let eq36_e731_d_n7: f64 = (eq36_e728_d_n7 + (s.dn[370][7] * ddt_scale));let eq36_e731_d_n8: f64 = (eq36_e728_d_n8 + (s.dn[370][8] * ddt_scale));let eq36_e731_d_n9: f64 = (eq36_e728_d_n9 + (s.dn[370][9] * ddt_scale));let eq36_e731_d_n10: f64 = (eq36_e728_d_n10 + (s.dn[370][10] * ddt_scale));let eq36_e731_d_n11: f64 = (eq36_e728_d_n11 + (s.dn[370][11] * ddt_scale));let eq36_e731_d_n12: f64 = (eq36_e728_d_n12 + (s.dn[370][12] * ddt_scale));let eq36_e731_d_n13: f64 = (eq36_e728_d_n13 + (s.dn[370][13] * ddt_scale));let eq36_e731_d_b0: f64 = (eq36_e728_d_b0 + (s.db[370][0] * ddt_scale));let eq36_e731_d_b1: f64 = (eq36_e728_d_b1 + (s.db[370][1] * ddt_scale));let eq36_e731_d_b2: f64 = (eq36_e728_d_b2 + (s.db[370][2] * ddt_scale));let eq36_e731_d_b3: f64 = (eq36_e728_d_b3 + (s.db[370][3] * ddt_scale));let eq36_e733: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, s.v[372]);let eq36_e734: f64 = (eq36_e731 + eq36_e733);let eq36_e734_d_n0: f64 = (eq36_e731_d_n0 + (s.dn[372][0] * ddt_scale));let eq36_e734_d_n1: f64 = (eq36_e731_d_n1 + (s.dn[372][1] * ddt_scale));let eq36_e734_d_n2: f64 = (eq36_e731_d_n2 + (s.dn[372][2] * ddt_scale));let eq36_e734_d_n3: f64 = (eq36_e731_d_n3 + (s.dn[372][3] * ddt_scale));let eq36_e734_d_n4: f64 = (eq36_e731_d_n4 + (s.dn[372][4] * ddt_scale));let eq36_e734_d_n5: f64 = (eq36_e731_d_n5 + (s.dn[372][5] * ddt_scale));let eq36_e734_d_n6: f64 = (eq36_e731_d_n6 + (s.dn[372][6] * ddt_scale));let eq36_e734_d_n7: f64 = (eq36_e731_d_n7 + (s.dn[372][7] * ddt_scale));let eq36_e734_d_n8: f64 = (eq36_e731_d_n8 + (s.dn[372][8] * ddt_scale));let eq36_e734_d_n9: f64 = (eq36_e731_d_n9 + (s.dn[372][9] * ddt_scale));let eq36_e734_d_n10: f64 = (eq36_e731_d_n10 + (s.dn[372][10] * ddt_scale));let eq36_e734_d_n11: f64 = (eq36_e731_d_n11 + (s.dn[372][11] * ddt_scale));let eq36_e734_d_n12: f64 = (eq36_e731_d_n12 + (s.dn[372][12] * ddt_scale));let eq36_e734_d_n13: f64 = (eq36_e731_d_n13 + (s.dn[372][13] * ddt_scale));let eq36_e734_d_b0: f64 = (eq36_e731_d_b0 + (s.db[372][0] * ddt_scale));let eq36_e734_d_b1: f64 = (eq36_e731_d_b1 + (s.db[372][1] * ddt_scale));
        let eq36_e734_d_b2: f64 = (eq36_e731_d_b2 + (s.db[372][2] * ddt_scale));let eq36_e734_d_b3: f64 = (eq36_e731_d_b3 + (s.db[372][3] * ddt_scale));let eq36_e736: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, s.v[379]);let eq36_e737: f64 = (eq36_e734 + eq36_e736);let eq36_e737_d_n0: f64 = (eq36_e734_d_n0 + (s.dn[379][0] * ddt_scale));let eq36_e737_d_n1: f64 = (eq36_e734_d_n1 + (s.dn[379][1] * ddt_scale));let eq36_e737_d_n2: f64 = (eq36_e734_d_n2 + (s.dn[379][2] * ddt_scale));let eq36_e737_d_n3: f64 = (eq36_e734_d_n3 + (s.dn[379][3] * ddt_scale));let eq36_e737_d_n4: f64 = (eq36_e734_d_n4 + (s.dn[379][4] * ddt_scale));let eq36_e737_d_n5: f64 = (eq36_e734_d_n5 + (s.dn[379][5] * ddt_scale));let eq36_e737_d_n6: f64 = (eq36_e734_d_n6 + (s.dn[379][6] * ddt_scale));let eq36_e737_d_n7: f64 = (eq36_e734_d_n7 + (s.dn[379][7] * ddt_scale));let eq36_e737_d_n8: f64 = (eq36_e734_d_n8 + (s.dn[379][8] * ddt_scale));let eq36_e737_d_n9: f64 = (eq36_e734_d_n9 + (s.dn[379][9] * ddt_scale));let eq36_e737_d_n10: f64 = (eq36_e734_d_n10 + (s.dn[379][10] * ddt_scale));let eq36_e737_d_n11: f64 = (eq36_e734_d_n11 + (s.dn[379][11] * ddt_scale));let eq36_e737_d_n12: f64 = (eq36_e734_d_n12 + (s.dn[379][12] * ddt_scale));let eq36_e737_d_n13: f64 = (eq36_e734_d_n13 + (s.dn[379][13] * ddt_scale));let eq36_e737_d_b0: f64 = (eq36_e734_d_b0 + (s.db[379][0] * ddt_scale));let eq36_e737_d_b1: f64 = (eq36_e734_d_b1 + (s.db[379][1] * ddt_scale));let eq36_e737_d_b2: f64 = (eq36_e734_d_b2 + (s.db[379][2] * ddt_scale));let eq36_e737_d_b3: f64 = (eq36_e734_d_b3 + (s.db[379][3] * ddt_scale));let eq36_e738: f64 = (p.p14 * eq36_e737);let eq36_e738_d_n0: f64 = (p.p14 * eq36_e737_d_n0);let eq36_e738_d_n1: f64 = (p.p14 * eq36_e737_d_n1);let eq36_e738_d_n2: f64 = (p.p14 * eq36_e737_d_n2);let eq36_e738_d_n3: f64 = (p.p14 * eq36_e737_d_n3);let eq36_e738_d_n4: f64 = (p.p14 * eq36_e737_d_n4);let eq36_e738_d_n5: f64 = (p.p14 * eq36_e737_d_n5);let eq36_e738_d_n6: f64 = (p.p14 * eq36_e737_d_n6);let eq36_e738_d_n7: f64 = (p.p14 * eq36_e737_d_n7);let eq36_e738_d_n8: f64 = (p.p14 * eq36_e737_d_n8);let eq36_e738_d_n9: f64 = (p.p14 * eq36_e737_d_n9);let eq36_e738_d_n10: f64 = (p.p14 * eq36_e737_d_n10);let eq36_e738_d_n11: f64 = (p.p14 * eq36_e737_d_n11);let eq36_e738_d_n12: f64 = (p.p14 * eq36_e737_d_n12);let eq36_e738_d_n13: f64 = (p.p14 * eq36_e737_d_n13);let eq36_e738_d_b0: f64 = (p.p14 * eq36_e737_d_b0);let eq36_e738_d_b1: f64 = (p.p14 * eq36_e737_d_b1);let eq36_e738_d_b2: f64 = (p.p14 * eq36_e737_d_b2);let eq36_e738_d_b3: f64 = (p.p14 * eq36_e737_d_b3);let eq36_value: f64 = eq36_e738;let eq36_node_derivatives: [f64; 14] = [eq36_e738_d_n0, eq36_e738_d_n1, eq36_e738_d_n2, eq36_e738_d_n3, eq36_e738_d_n4, eq36_e738_d_n5, eq36_e738_d_n6, eq36_e738_d_n7, eq36_e738_d_n8, eq36_e738_d_n9, eq36_e738_d_n10, eq36_e738_d_n11, eq36_e738_d_n12, eq36_e738_d_n13];let eq36_branch_derivatives: [f64; 4] = [eq36_e738_d_b0, eq36_e738_d_b1, eq36_e738_d_b2, eq36_e738_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_11(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let eq37_e741: f64 = (-s.v[1773]);let eq37_e743: f64 = (eq37_e741 * p.p31);let eq37_e743_d_n0: f64 = ((-s.dn[1773][0]) * p.p31);let eq37_e743_d_n1: f64 = ((-s.dn[1773][1]) * p.p31);let eq37_e743_d_n2: f64 = ((-s.dn[1773][2]) * p.p31);let eq37_e743_d_n3: f64 = ((-s.dn[1773][3]) * p.p31);let eq37_e743_d_n4: f64 = ((-s.dn[1773][4]) * p.p31);let eq37_e743_d_n5: f64 = ((-s.dn[1773][5]) * p.p31);let eq37_e743_d_n6: f64 = ((-s.dn[1773][6]) * p.p31);let eq37_e743_d_n7: f64 = ((-s.dn[1773][7]) * p.p31);let eq37_e743_d_n8: f64 = ((-s.dn[1773][8]) * p.p31);let eq37_e743_d_n9: f64 = ((-s.dn[1773][9]) * p.p31);let eq37_e743_d_n10: f64 = ((-s.dn[1773][10]) * p.p31);let eq37_e743_d_n11: f64 = ((-s.dn[1773][11]) * p.p31);let eq37_e743_d_n12: f64 = ((-s.dn[1773][12]) * p.p31);let eq37_e743_d_n13: f64 = ((-s.dn[1773][13]) * p.p31);let eq37_e743_d_b0: f64 = ((-s.db[1773][0]) * p.p31);let eq37_e743_d_b1: f64 = ((-s.db[1773][1]) * p.p31);let eq37_e743_d_b2: f64 = ((-s.db[1773][2]) * p.p31);let eq37_e743_d_b3: f64 = ((-s.db[1773][3]) * p.p31);let eq37_e745: f64 = (eq37_e743 * s.v[13]);let eq37_e745_d_n0: f64 = ((eq37_e743_d_n0 * s.v[13]) + (eq37_e743 * s.dn[13][0]));let eq37_e745_d_n1: f64 = ((eq37_e743_d_n1 * s.v[13]) + (eq37_e743 * s.dn[13][1]));let eq37_e745_d_n2: f64 = ((eq37_e743_d_n2 * s.v[13]) + (eq37_e743 * s.dn[13][2]));let eq37_e745_d_n3: f64 = ((eq37_e743_d_n3 * s.v[13]) + (eq37_e743 * s.dn[13][3]));let eq37_e745_d_n4: f64 = ((eq37_e743_d_n4 * s.v[13]) + (eq37_e743 * s.dn[13][4]));let eq37_e745_d_n5: f64 = ((eq37_e743_d_n5 * s.v[13]) + (eq37_e743 * s.dn[13][5]));let eq37_e745_d_n6: f64 = ((eq37_e743_d_n6 * s.v[13]) + (eq37_e743 * s.dn[13][6]));let eq37_e745_d_n7: f64 = ((eq37_e743_d_n7 * s.v[13]) + (eq37_e743 * s.dn[13][7]));let eq37_e745_d_n8: f64 = ((eq37_e743_d_n8 * s.v[13]) + (eq37_e743 * s.dn[13][8]));let eq37_e745_d_n9: f64 = ((eq37_e743_d_n9 * s.v[13]) + (eq37_e743 * s.dn[13][9]));let eq37_e745_d_n10: f64 = ((eq37_e743_d_n10 * s.v[13]) + (eq37_e743 * s.dn[13][10]));let eq37_e745_d_n11: f64 = ((eq37_e743_d_n11 * s.v[13]) + (eq37_e743 * s.dn[13][11]));let eq37_e745_d_n12: f64 = ((eq37_e743_d_n12 * s.v[13]) + (eq37_e743 * s.dn[13][12]));let eq37_e745_d_n13: f64 = ((eq37_e743_d_n13 * s.v[13]) + (eq37_e743 * s.dn[13][13]));let eq37_e745_d_b0: f64 = ((eq37_e743_d_b0 * s.v[13]) + (eq37_e743 * s.db[13][0]));let eq37_e745_d_b1: f64 = ((eq37_e743_d_b1 * s.v[13]) + (eq37_e743 * s.db[13][1]));let eq37_e745_d_b2: f64 = ((eq37_e743_d_b2 * s.v[13]) + (eq37_e743 * s.db[13][2]));let eq37_e745_d_b3: f64 = ((eq37_e743_d_b3 * s.v[13]) + (eq37_e743 * s.db[13][3]));let eq37_e747: f64 = (eq37_e745 * (nv12 - nv13));let eq37_e747_d_n0: f64 = (eq37_e745_d_n0 * (nv12 - nv13));let eq37_e747_d_n1: f64 = (eq37_e745_d_n1 * (nv12 - nv13));let eq37_e747_d_n2: f64 = (eq37_e745_d_n2 * (nv12 - nv13));let eq37_e747_d_n3: f64 = (eq37_e745_d_n3 * (nv12 - nv13));let eq37_e747_d_n4: f64 = (eq37_e745_d_n4 * (nv12 - nv13));let eq37_e747_d_n5: f64 = (eq37_e745_d_n5 * (nv12 - nv13));let eq37_e747_d_n6: f64 = (eq37_e745_d_n6 * (nv12 - nv13));let eq37_e747_d_n7: f64 = (eq37_e745_d_n7 * (nv12 - nv13));let eq37_e747_d_n8: f64 = (eq37_e745_d_n8 * (nv12 - nv13));let eq37_e747_d_n9: f64 = (eq37_e745_d_n9 * (nv12 - nv13));let eq37_e747_d_n10: f64 = (eq37_e745_d_n10 * (nv12 - nv13));let eq37_e747_d_n11: f64 = (eq37_e745_d_n11 * (nv12 - nv13));let eq37_e747_d_n12: f64 = ((eq37_e745_d_n12 * (nv12 - nv13)) + eq37_e745);let eq37_e747_d_n13: f64 = ((eq37_e745_d_n13 * (nv12 - nv13)) + (-eq37_e745));let eq37_e747_d_b0: f64 = (eq37_e745_d_b0 * (nv12 - nv13));let eq37_e747_d_b1: f64 = (eq37_e745_d_b1 * (nv12 - nv13));let eq37_e747_d_b2: f64 = (eq37_e745_d_b2 * (nv12 - nv13));let eq37_e747_d_b3: f64 = (eq37_e745_d_b3 * (nv12 - nv13));
        let eq37_e749: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, s.v[375]);let eq37_e750: f64 = (eq37_e747 + eq37_e749);let eq37_e750_d_n0: f64 = (eq37_e747_d_n0 + (s.dn[375][0] * ddt_scale));let eq37_e750_d_n1: f64 = (eq37_e747_d_n1 + (s.dn[375][1] * ddt_scale));let eq37_e750_d_n2: f64 = (eq37_e747_d_n2 + (s.dn[375][2] * ddt_scale));let eq37_e750_d_n3: f64 = (eq37_e747_d_n3 + (s.dn[375][3] * ddt_scale));let eq37_e750_d_n4: f64 = (eq37_e747_d_n4 + (s.dn[375][4] * ddt_scale));let eq37_e750_d_n5: f64 = (eq37_e747_d_n5 + (s.dn[375][5] * ddt_scale));let eq37_e750_d_n6: f64 = (eq37_e747_d_n6 + (s.dn[375][6] * ddt_scale));let eq37_e750_d_n7: f64 = (eq37_e747_d_n7 + (s.dn[375][7] * ddt_scale));let eq37_e750_d_n8: f64 = (eq37_e747_d_n8 + (s.dn[375][8] * ddt_scale));let eq37_e750_d_n9: f64 = (eq37_e747_d_n9 + (s.dn[375][9] * ddt_scale));let eq37_e750_d_n10: f64 = (eq37_e747_d_n10 + (s.dn[375][10] * ddt_scale));let eq37_e750_d_n11: f64 = (eq37_e747_d_n11 + (s.dn[375][11] * ddt_scale));let eq37_e750_d_n12: f64 = (eq37_e747_d_n12 + (s.dn[375][12] * ddt_scale));let eq37_e750_d_n13: f64 = (eq37_e747_d_n13 + (s.dn[375][13] * ddt_scale));let eq37_e750_d_b0: f64 = (eq37_e747_d_b0 + (s.db[375][0] * ddt_scale));let eq37_e750_d_b1: f64 = (eq37_e747_d_b1 + (s.db[375][1] * ddt_scale));let eq37_e750_d_b2: f64 = (eq37_e747_d_b2 + (s.db[375][2] * ddt_scale));let eq37_e750_d_b3: f64 = (eq37_e747_d_b3 + (s.db[375][3] * ddt_scale));let eq37_e751: f64 = (p.p14 * eq37_e750);let eq37_e751_d_n0: f64 = (p.p14 * eq37_e750_d_n0);let eq37_e751_d_n1: f64 = (p.p14 * eq37_e750_d_n1);let eq37_e751_d_n2: f64 = (p.p14 * eq37_e750_d_n2);let eq37_e751_d_n3: f64 = (p.p14 * eq37_e750_d_n3);let eq37_e751_d_n4: f64 = (p.p14 * eq37_e750_d_n4);let eq37_e751_d_n5: f64 = (p.p14 * eq37_e750_d_n5);let eq37_e751_d_n6: f64 = (p.p14 * eq37_e750_d_n6);let eq37_e751_d_n7: f64 = (p.p14 * eq37_e750_d_n7);let eq37_e751_d_n8: f64 = (p.p14 * eq37_e750_d_n8);let eq37_e751_d_n9: f64 = (p.p14 * eq37_e750_d_n9);let eq37_e751_d_n10: f64 = (p.p14 * eq37_e750_d_n10);let eq37_e751_d_n11: f64 = (p.p14 * eq37_e750_d_n11);let eq37_e751_d_n12: f64 = (p.p14 * eq37_e750_d_n12);let eq37_e751_d_n13: f64 = (p.p14 * eq37_e750_d_n13);let eq37_e751_d_b0: f64 = (p.p14 * eq37_e750_d_b0);let eq37_e751_d_b1: f64 = (p.p14 * eq37_e750_d_b1);let eq37_e751_d_b2: f64 = (p.p14 * eq37_e750_d_b2);let eq37_e751_d_b3: f64 = (p.p14 * eq37_e750_d_b3);let eq37_value: f64 = eq37_e751;let eq37_node_derivatives: [f64; 14] = [eq37_e751_d_n0, eq37_e751_d_n1, eq37_e751_d_n2, eq37_e751_d_n3, eq37_e751_d_n4, eq37_e751_d_n5, eq37_e751_d_n6, eq37_e751_d_n7, eq37_e751_d_n8, eq37_e751_d_n9, eq37_e751_d_n10, eq37_e751_d_n11, eq37_e751_d_n12, eq37_e751_d_n13];let eq37_branch_derivatives: [f64; 4] = [eq37_e751_d_b0, eq37_e751_d_b1, eq37_e751_d_b2, eq37_e751_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_12(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let eq38_e753: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, s.v[378]);let eq38_value: f64 = eq38_e753;
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq38_value),
            &s.dn[378],
            &s.db[378],
            (multiplicity) * (ddt_scale),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_13(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let eq40_e759: f64 = (s.v[1803] * (nv5 - 0.0));let eq40_e759_d_n0: f64 = (s.dn[1803][0] * (nv5 - 0.0));let eq40_e759_d_n1: f64 = (s.dn[1803][1] * (nv5 - 0.0));let eq40_e759_d_n2: f64 = (s.dn[1803][2] * (nv5 - 0.0));let eq40_e759_d_n3: f64 = (s.dn[1803][3] * (nv5 - 0.0));let eq40_e759_d_n4: f64 = (s.dn[1803][4] * (nv5 - 0.0));let eq40_e759_d_n5: f64 = ((s.dn[1803][5] * (nv5 - 0.0)) + s.v[1803]);let eq40_e759_d_n6: f64 = (s.dn[1803][6] * (nv5 - 0.0));let eq40_e759_d_n7: f64 = (s.dn[1803][7] * (nv5 - 0.0));let eq40_e759_d_n8: f64 = (s.dn[1803][8] * (nv5 - 0.0));let eq40_e759_d_n9: f64 = (s.dn[1803][9] * (nv5 - 0.0));let eq40_e759_d_n10: f64 = (s.dn[1803][10] * (nv5 - 0.0));let eq40_e759_d_n11: f64 = (s.dn[1803][11] * (nv5 - 0.0));let eq40_e759_d_n12: f64 = (s.dn[1803][12] * (nv5 - 0.0));let eq40_e759_d_n13: f64 = (s.dn[1803][13] * (nv5 - 0.0));let eq40_e759_d_b0: f64 = (s.db[1803][0] * (nv5 - 0.0));let eq40_e759_d_b1: f64 = (s.db[1803][1] * (nv5 - 0.0));let eq40_e759_d_b2: f64 = (s.db[1803][2] * (nv5 - 0.0));let eq40_e759_d_b3: f64 = (s.db[1803][3] * (nv5 - 0.0));let eq40_value: f64 = eq40_e759;let eq40_node_derivatives: [f64; 14] = [eq40_e759_d_n0, eq40_e759_d_n1, eq40_e759_d_n2, eq40_e759_d_n3, eq40_e759_d_n4, eq40_e759_d_n5, eq40_e759_d_n6, eq40_e759_d_n7, eq40_e759_d_n8, eq40_e759_d_n9, eq40_e759_d_n10, eq40_e759_d_n11, eq40_e759_d_n12, eq40_e759_d_n13];let eq40_branch_derivatives: [f64; 4] = [eq40_e759_d_b0, eq40_e759_d_b1, eq40_e759_d_b2, eq40_e759_d_b3];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );let eq41_e762: f64 = (s.v[1800] * (nv5 - 0.0));let eq41_e762_d_n0: f64 = (s.dn[1800][0] * (nv5 - 0.0));let eq41_e762_d_n1: f64 = (s.dn[1800][1] * (nv5 - 0.0));let eq41_e762_d_n2: f64 = (s.dn[1800][2] * (nv5 - 0.0));let eq41_e762_d_n3: f64 = (s.dn[1800][3] * (nv5 - 0.0));let eq41_e762_d_n4: f64 = (s.dn[1800][4] * (nv5 - 0.0));let eq41_e762_d_n5: f64 = ((s.dn[1800][5] * (nv5 - 0.0)) + s.v[1800]);let eq41_e762_d_n6: f64 = (s.dn[1800][6] * (nv5 - 0.0));let eq41_e762_d_n7: f64 = (s.dn[1800][7] * (nv5 - 0.0));let eq41_e762_d_n8: f64 = (s.dn[1800][8] * (nv5 - 0.0));let eq41_e762_d_n9: f64 = (s.dn[1800][9] * (nv5 - 0.0));let eq41_e762_d_n10: f64 = (s.dn[1800][10] * (nv5 - 0.0));let eq41_e762_d_n11: f64 = (s.dn[1800][11] * (nv5 - 0.0));let eq41_e762_d_n12: f64 = (s.dn[1800][12] * (nv5 - 0.0));let eq41_e762_d_n13: f64 = (s.dn[1800][13] * (nv5 - 0.0));let eq41_e762_d_b0: f64 = (s.db[1800][0] * (nv5 - 0.0));let eq41_e762_d_b1: f64 = (s.db[1800][1] * (nv5 - 0.0));let eq41_e762_d_b2: f64 = (s.db[1800][2] * (nv5 - 0.0));let eq41_e762_d_b3: f64 = (s.db[1800][3] * (nv5 - 0.0));let eq41_e763: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, eq41_e762);let eq41_value: f64 = eq41_e763;let eq41_node_derivatives: [f64; 14] = [(eq41_e762_d_n0 * ddt_scale), (eq41_e762_d_n1 * ddt_scale), (eq41_e762_d_n2 * ddt_scale), (eq41_e762_d_n3 * ddt_scale), (eq41_e762_d_n4 * ddt_scale), (eq41_e762_d_n5 * ddt_scale), (eq41_e762_d_n6 * ddt_scale), (eq41_e762_d_n7 * ddt_scale), (eq41_e762_d_n8 * ddt_scale), (eq41_e762_d_n9 * ddt_scale), (eq41_e762_d_n10 * ddt_scale), (eq41_e762_d_n11 * ddt_scale), (eq41_e762_d_n12 * ddt_scale), (eq41_e762_d_n13 * ddt_scale)];let eq41_branch_derivatives: [f64; 4] = [(eq41_e762_d_b0 * ddt_scale), (eq41_e762_d_b1 * ddt_scale), (eq41_e762_d_b2 * ddt_scale), (eq41_e762_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );let eq42_e765: f64 = (-s.v[1801]);let eq42_e767: f64 = (eq42_e765 * (nv5 - 0.0));let eq42_e767_d_n0: f64 = ((-s.dn[1801][0]) * (nv5 - 0.0));let eq42_e767_d_n1: f64 = ((-s.dn[1801][1]) * (nv5 - 0.0));let eq42_e767_d_n2: f64 = ((-s.dn[1801][2]) * (nv5 - 0.0));let eq42_e767_d_n3: f64 = ((-s.dn[1801][3]) * (nv5 - 0.0));let eq42_e767_d_n4: f64 = ((-s.dn[1801][4]) * (nv5 - 0.0));let eq42_e767_d_n5: f64 = (((-s.dn[1801][5]) * (nv5 - 0.0)) + eq42_e765);let eq42_e767_d_n6: f64 = ((-s.dn[1801][6]) * (nv5 - 0.0));let eq42_e767_d_n7: f64 = ((-s.dn[1801][7]) * (nv5 - 0.0));let eq42_e767_d_n8: f64 = ((-s.dn[1801][8]) * (nv5 - 0.0));let eq42_e767_d_n9: f64 = ((-s.dn[1801][9]) * (nv5 - 0.0));let eq42_e767_d_n10: f64 = ((-s.dn[1801][10]) * (nv5 - 0.0));let eq42_e767_d_n11: f64 = ((-s.dn[1801][11]) * (nv5 - 0.0));let eq42_e767_d_n12: f64 = ((-s.dn[1801][12]) * (nv5 - 0.0));let eq42_e767_d_n13: f64 = ((-s.dn[1801][13]) * (nv5 - 0.0));let eq42_e767_d_b0: f64 = ((-s.db[1801][0]) * (nv5 - 0.0));let eq42_e767_d_b1: f64 = ((-s.db[1801][1]) * (nv5 - 0.0));let eq42_e767_d_b2: f64 = ((-s.db[1801][2]) * (nv5 - 0.0));let eq42_e767_d_b3: f64 = ((-s.db[1801][3]) * (nv5 - 0.0));let eq42_e768: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, eq42_e767);let eq42_value: f64 = eq42_e768;let eq42_node_derivatives: [f64; 14] = [(eq42_e767_d_n0 * ddt_scale), (eq42_e767_d_n1 * ddt_scale), (eq42_e767_d_n2 * ddt_scale), (eq42_e767_d_n3 * ddt_scale), (eq42_e767_d_n4 * ddt_scale), (eq42_e767_d_n5 * ddt_scale), (eq42_e767_d_n6 * ddt_scale), (eq42_e767_d_n7 * ddt_scale), (eq42_e767_d_n8 * ddt_scale), (eq42_e767_d_n9 * ddt_scale), (eq42_e767_d_n10 * ddt_scale), (eq42_e767_d_n11 * ddt_scale), (eq42_e767_d_n12 * ddt_scale), (eq42_e767_d_n13 * ddt_scale)];let eq42_branch_derivatives: [f64; 4] = [(eq42_e767_d_b0 * ddt_scale), (eq42_e767_d_b1 * ddt_scale), (eq42_e767_d_b2 * ddt_scale), (eq42_e767_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_14(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let eq43_e770: f64 = (-s.v[1802]);let eq43_e772: f64 = (eq43_e770 * (nv5 - 0.0));let eq43_e772_d_n0: f64 = ((-s.dn[1802][0]) * (nv5 - 0.0));let eq43_e772_d_n1: f64 = ((-s.dn[1802][1]) * (nv5 - 0.0));let eq43_e772_d_n2: f64 = ((-s.dn[1802][2]) * (nv5 - 0.0));let eq43_e772_d_n3: f64 = ((-s.dn[1802][3]) * (nv5 - 0.0));let eq43_e772_d_n4: f64 = ((-s.dn[1802][4]) * (nv5 - 0.0));let eq43_e772_d_n5: f64 = (((-s.dn[1802][5]) * (nv5 - 0.0)) + eq43_e770);let eq43_e772_d_n6: f64 = ((-s.dn[1802][6]) * (nv5 - 0.0));let eq43_e772_d_n7: f64 = ((-s.dn[1802][7]) * (nv5 - 0.0));let eq43_e772_d_n8: f64 = ((-s.dn[1802][8]) * (nv5 - 0.0));let eq43_e772_d_n9: f64 = ((-s.dn[1802][9]) * (nv5 - 0.0));let eq43_e772_d_n10: f64 = ((-s.dn[1802][10]) * (nv5 - 0.0));let eq43_e772_d_n11: f64 = ((-s.dn[1802][11]) * (nv5 - 0.0));let eq43_e772_d_n12: f64 = ((-s.dn[1802][12]) * (nv5 - 0.0));let eq43_e772_d_n13: f64 = ((-s.dn[1802][13]) * (nv5 - 0.0));let eq43_e772_d_b0: f64 = ((-s.db[1802][0]) * (nv5 - 0.0));let eq43_e772_d_b1: f64 = ((-s.db[1802][1]) * (nv5 - 0.0));let eq43_e772_d_b2: f64 = ((-s.db[1802][2]) * (nv5 - 0.0));let eq43_e772_d_b3: f64 = ((-s.db[1802][3]) * (nv5 - 0.0));let eq43_e773: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, eq43_e772);let eq43_value: f64 = eq43_e773;let eq43_node_derivatives: [f64; 14] = [(eq43_e772_d_n0 * ddt_scale), (eq43_e772_d_n1 * ddt_scale), (eq43_e772_d_n2 * ddt_scale), (eq43_e772_d_n3 * ddt_scale), (eq43_e772_d_n4 * ddt_scale), (eq43_e772_d_n5 * ddt_scale), (eq43_e772_d_n6 * ddt_scale), (eq43_e772_d_n7 * ddt_scale), (eq43_e772_d_n8 * ddt_scale), (eq43_e772_d_n9 * ddt_scale), (eq43_e772_d_n10 * ddt_scale), (eq43_e772_d_n11 * ddt_scale), (eq43_e772_d_n12 * ddt_scale), (eq43_e772_d_n13 * ddt_scale)];let eq43_branch_derivatives: [f64; 4] = [(eq43_e772_d_b0 * ddt_scale), (eq43_e772_d_b1 * ddt_scale), (eq43_e772_d_b2 * ddt_scale), (eq43_e772_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
    }
}
