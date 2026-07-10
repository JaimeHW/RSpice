#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
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
        let (eq0_e500, eq0_e500_d_n0, eq0_e500_d_n1, eq0_e500_d_n2, eq0_e500_d_n3, eq0_e500_d_n4, eq0_e500_d_n5, eq0_e500_d_n6, eq0_e500_d_n7, eq0_e500_d_n8, eq0_e500_d_n9, eq0_e500_d_b0, eq0_e500_d_b1, eq0_e500_d_b2, eq0_e500_d_b3,) = {
    if s.b[1763] {
        let eq0_e498: f64 = (p.p14 * s.v[361]);
        (eq0_e498, (p.p14 * s.dn[361][0]), (p.p14 * s.dn[361][1]), (p.p14 * s.dn[361][2]), (p.p14 * s.dn[361][3]), (p.p14 * s.dn[361][4]), (p.p14 * s.dn[361][5]), (p.p14 * s.dn[361][6]), (p.p14 * s.dn[361][7]), (p.p14 * s.dn[361][8]), (p.p14 * s.dn[361][9]), (p.p14 * s.db[361][0]), (p.p14 * s.db[361][1]), (p.p14 * s.db[361][2]), (p.p14 * s.db[361][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e500;let eq0_node_derivatives: [f64; 10] = [eq0_e500_d_n0, eq0_e500_d_n1, eq0_e500_d_n2, eq0_e500_d_n3, eq0_e500_d_n4, eq0_e500_d_n5, eq0_e500_d_n6, eq0_e500_d_n7, eq0_e500_d_n8, eq0_e500_d_n9];let eq0_branch_derivatives: [f64; 4] = [eq0_e500_d_b0, eq0_e500_d_b1, eq0_e500_d_b2, eq0_e500_d_b3];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e507, eq1_e507_d_n0, eq1_e507_d_n1, eq1_e507_d_n2, eq1_e507_d_n3, eq1_e507_d_n4, eq1_e507_d_n5, eq1_e507_d_n6, eq1_e507_d_n7, eq1_e507_d_n8, eq1_e507_d_n9, eq1_e507_d_b0, eq1_e507_d_b1, eq1_e507_d_b2, eq1_e507_d_b3,) = {
    if (!s.b[1763]) {
        let eq1_e505: f64 = (p.p14 * s.v[361]);
        (eq1_e505, (p.p14 * s.dn[361][0]), (p.p14 * s.dn[361][1]), (p.p14 * s.dn[361][2]), (p.p14 * s.dn[361][3]), (p.p14 * s.dn[361][4]), (p.p14 * s.dn[361][5]), (p.p14 * s.dn[361][6]), (p.p14 * s.dn[361][7]), (p.p14 * s.dn[361][8]), (p.p14 * s.dn[361][9]), (p.p14 * s.db[361][0]), (p.p14 * s.db[361][1]), (p.p14 * s.db[361][2]), (p.p14 * s.db[361][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e507;let eq1_node_derivatives: [f64; 10] = [eq1_e507_d_n0, eq1_e507_d_n1, eq1_e507_d_n2, eq1_e507_d_n3, eq1_e507_d_n4, eq1_e507_d_n5, eq1_e507_d_n6, eq1_e507_d_n7, eq1_e507_d_n8, eq1_e507_d_n9];let eq1_branch_derivatives: [f64; 4] = [eq1_e507_d_b0, eq1_e507_d_b1, eq1_e507_d_b2, eq1_e507_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );let eq2_e511: f64 = (s.v[364] - s.v[365]);let eq2_e511_d_n0: f64 = (s.dn[364][0] - s.dn[365][0]);let eq2_e511_d_n1: f64 = (s.dn[364][1] - s.dn[365][1]);let eq2_e511_d_n2: f64 = (s.dn[364][2] - s.dn[365][2]);let eq2_e511_d_n3: f64 = (s.dn[364][3] - s.dn[365][3]);let eq2_e511_d_n4: f64 = (s.dn[364][4] - s.dn[365][4]);let eq2_e511_d_n5: f64 = (s.dn[364][5] - s.dn[365][5]);let eq2_e511_d_n6: f64 = (s.dn[364][6] - s.dn[365][6]);let eq2_e511_d_n7: f64 = (s.dn[364][7] - s.dn[365][7]);let eq2_e511_d_n8: f64 = (s.dn[364][8] - s.dn[365][8]);let eq2_e511_d_n9: f64 = (s.dn[364][9] - s.dn[365][9]);let eq2_e511_d_b0: f64 = (s.db[364][0] - s.db[365][0]);let eq2_e511_d_b1: f64 = (s.db[364][1] - s.db[365][1]);let eq2_e511_d_b2: f64 = (s.db[364][2] - s.db[365][2]);let eq2_e511_d_b3: f64 = (s.db[364][3] - s.db[365][3]);let eq2_e512: f64 = (p.p14 * eq2_e511);let eq2_e512_d_n0: f64 = (p.p14 * eq2_e511_d_n0);let eq2_e512_d_n1: f64 = (p.p14 * eq2_e511_d_n1);let eq2_e512_d_n2: f64 = (p.p14 * eq2_e511_d_n2);let eq2_e512_d_n3: f64 = (p.p14 * eq2_e511_d_n3);let eq2_e512_d_n4: f64 = (p.p14 * eq2_e511_d_n4);let eq2_e512_d_n5: f64 = (p.p14 * eq2_e511_d_n5);let eq2_e512_d_n6: f64 = (p.p14 * eq2_e511_d_n6);let eq2_e512_d_n7: f64 = (p.p14 * eq2_e511_d_n7);let eq2_e512_d_n8: f64 = (p.p14 * eq2_e511_d_n8);let eq2_e512_d_n9: f64 = (p.p14 * eq2_e511_d_n9);let eq2_e512_d_b0: f64 = (p.p14 * eq2_e511_d_b0);let eq2_e512_d_b1: f64 = (p.p14 * eq2_e511_d_b1);let eq2_e512_d_b2: f64 = (p.p14 * eq2_e511_d_b2);let eq2_e512_d_b3: f64 = (p.p14 * eq2_e511_d_b3);let eq2_value: f64 = eq2_e512;let eq2_node_derivatives: [f64; 10] = [eq2_e512_d_n0, eq2_e512_d_n1, eq2_e512_d_n2, eq2_e512_d_n3, eq2_e512_d_n4, eq2_e512_d_n5, eq2_e512_d_n6, eq2_e512_d_n7, eq2_e512_d_n8, eq2_e512_d_n9];let eq2_branch_derivatives: [f64; 4] = [eq2_e512_d_b0, eq2_e512_d_b1, eq2_e512_d_b2, eq2_e512_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );let eq3_e515: f64 = (p.p14 * s.v[362]);let eq3_value: f64 = eq3_e515;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            &s.dn[362],
            &s.db[362],
            (multiplicity) * (p.p14),
        );let eq4_e518: f64 = (p.p14 * s.v[363]);let eq4_value: f64 = eq4_e518;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq4_value),
            &s.dn[363],
            &s.db[363],
            (multiplicity) * (p.p14),
        );let eq8_e524: f64 = (p.p31 * s.v[471]);let eq8_e526: f64 = (eq8_e524 * (nv7 - nv6));let eq8_value: f64 = eq8_e526;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(6),
            multiplicity * (eq8_value),
            6,
            multiplicity * ((-eq8_e524)),
            7,
            multiplicity * (eq8_e524),
        );let eq9_value: f64 = s.v[1761];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq9_value),
            &s.dn[1761],
            &s.db[1761],
            multiplicity,
        );let eq10_value: f64 = s.v[1762];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq10_value),
            &s.dn[1762],
            &s.db[1762],
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
        let (eq11_e538, eq11_e538_d_n0, eq11_e538_d_n1, eq11_e538_d_n2, eq11_e538_d_n3, eq11_e538_d_n4, eq11_e538_d_n5, eq11_e538_d_n6, eq11_e538_d_n7, eq11_e538_d_n8, eq11_e538_d_n9, eq11_e538_d_b0, eq11_e538_d_b1, eq11_e538_d_b2, eq11_e538_d_b3,) = {
    if s.b[1764] {
        let eq11_e532: f64 = (p.p31 * s.v[13]);let eq11_e534: f64 = (eq11_e532 * s.v[312]);let eq11_e534_d_n0: f64 = (((p.p31 * s.dn[13][0]) * s.v[312]) + (eq11_e532 * s.dn[312][0]));let eq11_e534_d_n1: f64 = (((p.p31 * s.dn[13][1]) * s.v[312]) + (eq11_e532 * s.dn[312][1]));let eq11_e534_d_n2: f64 = (((p.p31 * s.dn[13][2]) * s.v[312]) + (eq11_e532 * s.dn[312][2]));let eq11_e534_d_n3: f64 = (((p.p31 * s.dn[13][3]) * s.v[312]) + (eq11_e532 * s.dn[312][3]));let eq11_e534_d_n4: f64 = (((p.p31 * s.dn[13][4]) * s.v[312]) + (eq11_e532 * s.dn[312][4]));let eq11_e534_d_n5: f64 = (((p.p31 * s.dn[13][5]) * s.v[312]) + (eq11_e532 * s.dn[312][5]));let eq11_e534_d_n6: f64 = (((p.p31 * s.dn[13][6]) * s.v[312]) + (eq11_e532 * s.dn[312][6]));let eq11_e534_d_n7: f64 = (((p.p31 * s.dn[13][7]) * s.v[312]) + (eq11_e532 * s.dn[312][7]));let eq11_e534_d_n8: f64 = (((p.p31 * s.dn[13][8]) * s.v[312]) + (eq11_e532 * s.dn[312][8]));let eq11_e534_d_n9: f64 = (((p.p31 * s.dn[13][9]) * s.v[312]) + (eq11_e532 * s.dn[312][9]));let eq11_e534_d_b0: f64 = (((p.p31 * s.db[13][0]) * s.v[312]) + (eq11_e532 * s.db[312][0]));let eq11_e534_d_b1: f64 = (((p.p31 * s.db[13][1]) * s.v[312]) + (eq11_e532 * s.db[312][1]));let eq11_e534_d_b2: f64 = (((p.p31 * s.db[13][2]) * s.v[312]) + (eq11_e532 * s.db[312][2]));let eq11_e534_d_b3: f64 = (((p.p31 * s.db[13][3]) * s.v[312]) + (eq11_e532 * s.db[312][3]));let eq11_e536: f64 = (eq11_e534 * (nv1 - nv9));let eq11_e536_d_n0: f64 = (eq11_e534_d_n0 * (nv1 - nv9));let eq11_e536_d_n1: f64 = ((eq11_e534_d_n1 * (nv1 - nv9)) + eq11_e534);let eq11_e536_d_n2: f64 = (eq11_e534_d_n2 * (nv1 - nv9));let eq11_e536_d_n3: f64 = (eq11_e534_d_n3 * (nv1 - nv9));let eq11_e536_d_n4: f64 = (eq11_e534_d_n4 * (nv1 - nv9));let eq11_e536_d_n5: f64 = (eq11_e534_d_n5 * (nv1 - nv9));let eq11_e536_d_n6: f64 = (eq11_e534_d_n6 * (nv1 - nv9));let eq11_e536_d_n7: f64 = (eq11_e534_d_n7 * (nv1 - nv9));let eq11_e536_d_n8: f64 = (eq11_e534_d_n8 * (nv1 - nv9));let eq11_e536_d_n9: f64 = ((eq11_e534_d_n9 * (nv1 - nv9)) + (-eq11_e534));let eq11_e536_d_b0: f64 = (eq11_e534_d_b0 * (nv1 - nv9));let eq11_e536_d_b1: f64 = (eq11_e534_d_b1 * (nv1 - nv9));let eq11_e536_d_b2: f64 = (eq11_e534_d_b2 * (nv1 - nv9));let eq11_e536_d_b3: f64 = (eq11_e534_d_b3 * (nv1 - nv9));
        (eq11_e536, eq11_e536_d_n0, eq11_e536_d_n1, eq11_e536_d_n2, eq11_e536_d_n3, eq11_e536_d_n4, eq11_e536_d_n5, eq11_e536_d_n6, eq11_e536_d_n7, eq11_e536_d_n8, eq11_e536_d_n9, eq11_e536_d_b0, eq11_e536_d_b1, eq11_e536_d_b2, eq11_e536_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e538;let eq11_node_derivatives: [f64; 10] = [eq11_e538_d_n0, eq11_e538_d_n1, eq11_e538_d_n2, eq11_e538_d_n3, eq11_e538_d_n4, eq11_e538_d_n5, eq11_e538_d_n6, eq11_e538_d_n7, eq11_e538_d_n8, eq11_e538_d_n9];let eq11_branch_derivatives: [f64; 4] = [eq11_e538_d_b0, eq11_e538_d_b1, eq11_e538_d_b2, eq11_e538_d_b3];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq13_e553,) = {
    if (!s.b[1764]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e553;
        stamper.stamp_potential_const_local(
            0,
            eq13_value,
        );
        let (eq14_e563, eq14_e563_d_n0, eq14_e563_d_n1, eq14_e563_d_n2, eq14_e563_d_n3, eq14_e563_d_n4, eq14_e563_d_n5, eq14_e563_d_n6, eq14_e563_d_n7, eq14_e563_d_n8, eq14_e563_d_n9, eq14_e563_d_b0, eq14_e563_d_b1, eq14_e563_d_b2, eq14_e563_d_b3,) = {
    if s.b[1765] {
        let eq14_e557: f64 = (p.p31 * s.v[13]);let eq14_e559: f64 = (eq14_e557 * s.v[316]);let eq14_e559_d_n0: f64 = (((p.p31 * s.dn[13][0]) * s.v[316]) + (eq14_e557 * s.dn[316][0]));let eq14_e559_d_n1: f64 = (((p.p31 * s.dn[13][1]) * s.v[316]) + (eq14_e557 * s.dn[316][1]));let eq14_e559_d_n2: f64 = (((p.p31 * s.dn[13][2]) * s.v[316]) + (eq14_e557 * s.dn[316][2]));let eq14_e559_d_n3: f64 = (((p.p31 * s.dn[13][3]) * s.v[316]) + (eq14_e557 * s.dn[316][3]));let eq14_e559_d_n4: f64 = (((p.p31 * s.dn[13][4]) * s.v[316]) + (eq14_e557 * s.dn[316][4]));let eq14_e559_d_n5: f64 = (((p.p31 * s.dn[13][5]) * s.v[316]) + (eq14_e557 * s.dn[316][5]));let eq14_e559_d_n6: f64 = (((p.p31 * s.dn[13][6]) * s.v[316]) + (eq14_e557 * s.dn[316][6]));let eq14_e559_d_n7: f64 = (((p.p31 * s.dn[13][7]) * s.v[316]) + (eq14_e557 * s.dn[316][7]));let eq14_e559_d_n8: f64 = (((p.p31 * s.dn[13][8]) * s.v[316]) + (eq14_e557 * s.dn[316][8]));let eq14_e559_d_n9: f64 = (((p.p31 * s.dn[13][9]) * s.v[316]) + (eq14_e557 * s.dn[316][9]));let eq14_e559_d_b0: f64 = (((p.p31 * s.db[13][0]) * s.v[316]) + (eq14_e557 * s.db[316][0]));let eq14_e559_d_b1: f64 = (((p.p31 * s.db[13][1]) * s.v[316]) + (eq14_e557 * s.db[316][1]));let eq14_e559_d_b2: f64 = (((p.p31 * s.db[13][2]) * s.v[316]) + (eq14_e557 * s.db[316][2]));let eq14_e559_d_b3: f64 = (((p.p31 * s.db[13][3]) * s.v[316]) + (eq14_e557 * s.db[316][3]));let eq14_e561: f64 = (eq14_e559 * (nv2 - nv6));let eq14_e561_d_n0: f64 = (eq14_e559_d_n0 * (nv2 - nv6));let eq14_e561_d_n1: f64 = (eq14_e559_d_n1 * (nv2 - nv6));let eq14_e561_d_n2: f64 = ((eq14_e559_d_n2 * (nv2 - nv6)) + eq14_e559);let eq14_e561_d_n3: f64 = (eq14_e559_d_n3 * (nv2 - nv6));let eq14_e561_d_n4: f64 = (eq14_e559_d_n4 * (nv2 - nv6));let eq14_e561_d_n5: f64 = (eq14_e559_d_n5 * (nv2 - nv6));let eq14_e561_d_n6: f64 = ((eq14_e559_d_n6 * (nv2 - nv6)) + (-eq14_e559));let eq14_e561_d_n7: f64 = (eq14_e559_d_n7 * (nv2 - nv6));let eq14_e561_d_n8: f64 = (eq14_e559_d_n8 * (nv2 - nv6));let eq14_e561_d_n9: f64 = (eq14_e559_d_n9 * (nv2 - nv6));let eq14_e561_d_b0: f64 = (eq14_e559_d_b0 * (nv2 - nv6));let eq14_e561_d_b1: f64 = (eq14_e559_d_b1 * (nv2 - nv6));let eq14_e561_d_b2: f64 = (eq14_e559_d_b2 * (nv2 - nv6));let eq14_e561_d_b3: f64 = (eq14_e559_d_b3 * (nv2 - nv6));
        (eq14_e561, eq14_e561_d_n0, eq14_e561_d_n1, eq14_e561_d_n2, eq14_e561_d_n3, eq14_e561_d_n4, eq14_e561_d_n5, eq14_e561_d_n6, eq14_e561_d_n7, eq14_e561_d_n8, eq14_e561_d_n9, eq14_e561_d_b0, eq14_e561_d_b1, eq14_e561_d_b2, eq14_e561_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e563;let eq14_node_derivatives: [f64; 10] = [eq14_e563_d_n0, eq14_e563_d_n1, eq14_e563_d_n2, eq14_e563_d_n3, eq14_e563_d_n4, eq14_e563_d_n5, eq14_e563_d_n6, eq14_e563_d_n7, eq14_e563_d_n8, eq14_e563_d_n9];let eq14_branch_derivatives: [f64; 4] = [eq14_e563_d_b0, eq14_e563_d_b1, eq14_e563_d_b2, eq14_e563_d_b3];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq16_e578,) = {
    if (!s.b[1765]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e578;
        stamper.stamp_potential_const_local(
            1,
            eq16_value,
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
        let nv0 = ctx.node_voltage(nodes[0]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq17_e588, eq17_e588_d_n0, eq17_e588_d_n1, eq17_e588_d_n2, eq17_e588_d_n3, eq17_e588_d_n4, eq17_e588_d_n5, eq17_e588_d_n6, eq17_e588_d_n7, eq17_e588_d_n8, eq17_e588_d_n9, eq17_e588_d_b0, eq17_e588_d_b1, eq17_e588_d_b2, eq17_e588_d_b3,) = {
    if s.b[1766] {
        let eq17_e582: f64 = (p.p31 * s.v[13]);let eq17_e584: f64 = (eq17_e582 * s.v[320]);let eq17_e584_d_n0: f64 = (((p.p31 * s.dn[13][0]) * s.v[320]) + (eq17_e582 * s.dn[320][0]));let eq17_e584_d_n1: f64 = (((p.p31 * s.dn[13][1]) * s.v[320]) + (eq17_e582 * s.dn[320][1]));let eq17_e584_d_n2: f64 = (((p.p31 * s.dn[13][2]) * s.v[320]) + (eq17_e582 * s.dn[320][2]));let eq17_e584_d_n3: f64 = (((p.p31 * s.dn[13][3]) * s.v[320]) + (eq17_e582 * s.dn[320][3]));let eq17_e584_d_n4: f64 = (((p.p31 * s.dn[13][4]) * s.v[320]) + (eq17_e582 * s.dn[320][4]));let eq17_e584_d_n5: f64 = (((p.p31 * s.dn[13][5]) * s.v[320]) + (eq17_e582 * s.dn[320][5]));let eq17_e584_d_n6: f64 = (((p.p31 * s.dn[13][6]) * s.v[320]) + (eq17_e582 * s.dn[320][6]));let eq17_e584_d_n7: f64 = (((p.p31 * s.dn[13][7]) * s.v[320]) + (eq17_e582 * s.dn[320][7]));let eq17_e584_d_n8: f64 = (((p.p31 * s.dn[13][8]) * s.v[320]) + (eq17_e582 * s.dn[320][8]));let eq17_e584_d_n9: f64 = (((p.p31 * s.dn[13][9]) * s.v[320]) + (eq17_e582 * s.dn[320][9]));let eq17_e584_d_b0: f64 = (((p.p31 * s.db[13][0]) * s.v[320]) + (eq17_e582 * s.db[320][0]));let eq17_e584_d_b1: f64 = (((p.p31 * s.db[13][1]) * s.v[320]) + (eq17_e582 * s.db[320][1]));let eq17_e584_d_b2: f64 = (((p.p31 * s.db[13][2]) * s.v[320]) + (eq17_e582 * s.db[320][2]));let eq17_e584_d_b3: f64 = (((p.p31 * s.db[13][3]) * s.v[320]) + (eq17_e582 * s.db[320][3]));let eq17_e586: f64 = (eq17_e584 * (nv0 - nv7));let eq17_e586_d_n0: f64 = ((eq17_e584_d_n0 * (nv0 - nv7)) + eq17_e584);let eq17_e586_d_n1: f64 = (eq17_e584_d_n1 * (nv0 - nv7));let eq17_e586_d_n2: f64 = (eq17_e584_d_n2 * (nv0 - nv7));let eq17_e586_d_n3: f64 = (eq17_e584_d_n3 * (nv0 - nv7));let eq17_e586_d_n4: f64 = (eq17_e584_d_n4 * (nv0 - nv7));let eq17_e586_d_n5: f64 = (eq17_e584_d_n5 * (nv0 - nv7));let eq17_e586_d_n6: f64 = (eq17_e584_d_n6 * (nv0 - nv7));let eq17_e586_d_n7: f64 = ((eq17_e584_d_n7 * (nv0 - nv7)) + (-eq17_e584));let eq17_e586_d_n8: f64 = (eq17_e584_d_n8 * (nv0 - nv7));let eq17_e586_d_n9: f64 = (eq17_e584_d_n9 * (nv0 - nv7));let eq17_e586_d_b0: f64 = (eq17_e584_d_b0 * (nv0 - nv7));let eq17_e586_d_b1: f64 = (eq17_e584_d_b1 * (nv0 - nv7));let eq17_e586_d_b2: f64 = (eq17_e584_d_b2 * (nv0 - nv7));let eq17_e586_d_b3: f64 = (eq17_e584_d_b3 * (nv0 - nv7));
        (eq17_e586, eq17_e586_d_n0, eq17_e586_d_n1, eq17_e586_d_n2, eq17_e586_d_n3, eq17_e586_d_n4, eq17_e586_d_n5, eq17_e586_d_n6, eq17_e586_d_n7, eq17_e586_d_n8, eq17_e586_d_n9, eq17_e586_d_b0, eq17_e586_d_b1, eq17_e586_d_b2, eq17_e586_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e588;let eq17_node_derivatives: [f64; 10] = [eq17_e588_d_n0, eq17_e588_d_n1, eq17_e588_d_n2, eq17_e588_d_n3, eq17_e588_d_n4, eq17_e588_d_n5, eq17_e588_d_n6, eq17_e588_d_n7, eq17_e588_d_n8, eq17_e588_d_n9];let eq17_branch_derivatives: [f64; 4] = [eq17_e588_d_b0, eq17_e588_d_b1, eq17_e588_d_b2, eq17_e588_d_b3];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq19_e603,) = {
    if (!s.b[1766]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e603;
        stamper.stamp_potential_const_local(
            2,
            eq19_value,
        );
        let (eq20_e613, eq20_e613_d_n0, eq20_e613_d_n1, eq20_e613_d_n2, eq20_e613_d_n3, eq20_e613_d_n4, eq20_e613_d_n5, eq20_e613_d_n6, eq20_e613_d_n7, eq20_e613_d_n8, eq20_e613_d_n9, eq20_e613_d_b0, eq20_e613_d_b1, eq20_e613_d_b2, eq20_e613_d_b3,) = {
    if s.b[1767] {
        let eq20_e607: f64 = (p.p31 * s.v[13]);let eq20_e609: f64 = (eq20_e607 * s.v[323]);let eq20_e609_d_n0: f64 = (((p.p31 * s.dn[13][0]) * s.v[323]) + (eq20_e607 * s.dn[323][0]));let eq20_e609_d_n1: f64 = (((p.p31 * s.dn[13][1]) * s.v[323]) + (eq20_e607 * s.dn[323][1]));let eq20_e609_d_n2: f64 = (((p.p31 * s.dn[13][2]) * s.v[323]) + (eq20_e607 * s.dn[323][2]));let eq20_e609_d_n3: f64 = (((p.p31 * s.dn[13][3]) * s.v[323]) + (eq20_e607 * s.dn[323][3]));let eq20_e609_d_n4: f64 = (((p.p31 * s.dn[13][4]) * s.v[323]) + (eq20_e607 * s.dn[323][4]));let eq20_e609_d_n5: f64 = (((p.p31 * s.dn[13][5]) * s.v[323]) + (eq20_e607 * s.dn[323][5]));let eq20_e609_d_n6: f64 = (((p.p31 * s.dn[13][6]) * s.v[323]) + (eq20_e607 * s.dn[323][6]));let eq20_e609_d_n7: f64 = (((p.p31 * s.dn[13][7]) * s.v[323]) + (eq20_e607 * s.dn[323][7]));let eq20_e609_d_n8: f64 = (((p.p31 * s.dn[13][8]) * s.v[323]) + (eq20_e607 * s.dn[323][8]));let eq20_e609_d_n9: f64 = (((p.p31 * s.dn[13][9]) * s.v[323]) + (eq20_e607 * s.dn[323][9]));let eq20_e609_d_b0: f64 = (((p.p31 * s.db[13][0]) * s.v[323]) + (eq20_e607 * s.db[323][0]));let eq20_e609_d_b1: f64 = (((p.p31 * s.db[13][1]) * s.v[323]) + (eq20_e607 * s.db[323][1]));let eq20_e609_d_b2: f64 = (((p.p31 * s.db[13][2]) * s.v[323]) + (eq20_e607 * s.db[323][2]));let eq20_e609_d_b3: f64 = (((p.p31 * s.db[13][3]) * s.v[323]) + (eq20_e607 * s.db[323][3]));let eq20_e611: f64 = (eq20_e609 * (nv3 - nv8));let eq20_e611_d_n0: f64 = (eq20_e609_d_n0 * (nv3 - nv8));let eq20_e611_d_n1: f64 = (eq20_e609_d_n1 * (nv3 - nv8));let eq20_e611_d_n2: f64 = (eq20_e609_d_n2 * (nv3 - nv8));let eq20_e611_d_n3: f64 = ((eq20_e609_d_n3 * (nv3 - nv8)) + eq20_e609);let eq20_e611_d_n4: f64 = (eq20_e609_d_n4 * (nv3 - nv8));let eq20_e611_d_n5: f64 = (eq20_e609_d_n5 * (nv3 - nv8));let eq20_e611_d_n6: f64 = (eq20_e609_d_n6 * (nv3 - nv8));let eq20_e611_d_n7: f64 = (eq20_e609_d_n7 * (nv3 - nv8));let eq20_e611_d_n8: f64 = ((eq20_e609_d_n8 * (nv3 - nv8)) + (-eq20_e609));let eq20_e611_d_n9: f64 = (eq20_e609_d_n9 * (nv3 - nv8));let eq20_e611_d_b0: f64 = (eq20_e609_d_b0 * (nv3 - nv8));let eq20_e611_d_b1: f64 = (eq20_e609_d_b1 * (nv3 - nv8));let eq20_e611_d_b2: f64 = (eq20_e609_d_b2 * (nv3 - nv8));let eq20_e611_d_b3: f64 = (eq20_e609_d_b3 * (nv3 - nv8));
        (eq20_e611, eq20_e611_d_n0, eq20_e611_d_n1, eq20_e611_d_n2, eq20_e611_d_n3, eq20_e611_d_n4, eq20_e611_d_n5, eq20_e611_d_n6, eq20_e611_d_n7, eq20_e611_d_n8, eq20_e611_d_n9, eq20_e611_d_b0, eq20_e611_d_b1, eq20_e611_d_b2, eq20_e611_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e613;let eq20_node_derivatives: [f64; 10] = [eq20_e613_d_n0, eq20_e613_d_n1, eq20_e613_d_n2, eq20_e613_d_n3, eq20_e613_d_n4, eq20_e613_d_n5, eq20_e613_d_n6, eq20_e613_d_n7, eq20_e613_d_n8, eq20_e613_d_n9];let eq20_branch_derivatives: [f64; 4] = [eq20_e613_d_b0, eq20_e613_d_b1, eq20_e613_d_b2, eq20_e613_d_b3];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq22_e628,) = {
    if (!s.b[1767]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e628;
        stamper.stamp_potential_const_local(
            3,
            eq22_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
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
        let eq23_e631: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[358]);let eq23_e633: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, s.v[373]);let eq23_e634: f64 = (eq23_e631 + eq23_e633);let eq23_e634_d_n0: f64 = ((s.dn[358][0] * ddt_scale) + (s.dn[373][0] * ddt_scale));let eq23_e634_d_n1: f64 = ((s.dn[358][1] * ddt_scale) + (s.dn[373][1] * ddt_scale));let eq23_e634_d_n2: f64 = ((s.dn[358][2] * ddt_scale) + (s.dn[373][2] * ddt_scale));let eq23_e634_d_n3: f64 = ((s.dn[358][3] * ddt_scale) + (s.dn[373][3] * ddt_scale));let eq23_e634_d_n4: f64 = ((s.dn[358][4] * ddt_scale) + (s.dn[373][4] * ddt_scale));let eq23_e634_d_n5: f64 = ((s.dn[358][5] * ddt_scale) + (s.dn[373][5] * ddt_scale));let eq23_e634_d_n6: f64 = ((s.dn[358][6] * ddt_scale) + (s.dn[373][6] * ddt_scale));let eq23_e634_d_n7: f64 = ((s.dn[358][7] * ddt_scale) + (s.dn[373][7] * ddt_scale));let eq23_e634_d_n8: f64 = ((s.dn[358][8] * ddt_scale) + (s.dn[373][8] * ddt_scale));let eq23_e634_d_n9: f64 = ((s.dn[358][9] * ddt_scale) + (s.dn[373][9] * ddt_scale));let eq23_e634_d_b0: f64 = ((s.db[358][0] * ddt_scale) + (s.db[373][0] * ddt_scale));let eq23_e634_d_b1: f64 = ((s.db[358][1] * ddt_scale) + (s.db[373][1] * ddt_scale));let eq23_e634_d_b2: f64 = ((s.db[358][2] * ddt_scale) + (s.db[373][2] * ddt_scale));let eq23_e634_d_b3: f64 = ((s.db[358][3] * ddt_scale) + (s.db[373][3] * ddt_scale));let eq23_e636: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[377]);let eq23_e637: f64 = (eq23_e634 + eq23_e636);let eq23_e637_d_n0: f64 = (eq23_e634_d_n0 + (s.dn[377][0] * ddt_scale));let eq23_e637_d_n1: f64 = (eq23_e634_d_n1 + (s.dn[377][1] * ddt_scale));let eq23_e637_d_n2: f64 = (eq23_e634_d_n2 + (s.dn[377][2] * ddt_scale));let eq23_e637_d_n3: f64 = (eq23_e634_d_n3 + (s.dn[377][3] * ddt_scale));let eq23_e637_d_n4: f64 = (eq23_e634_d_n4 + (s.dn[377][4] * ddt_scale));let eq23_e637_d_n5: f64 = (eq23_e634_d_n5 + (s.dn[377][5] * ddt_scale));let eq23_e637_d_n6: f64 = (eq23_e634_d_n6 + (s.dn[377][6] * ddt_scale));let eq23_e637_d_n7: f64 = (eq23_e634_d_n7 + (s.dn[377][7] * ddt_scale));let eq23_e637_d_n8: f64 = (eq23_e634_d_n8 + (s.dn[377][8] * ddt_scale));let eq23_e637_d_n9: f64 = (eq23_e634_d_n9 + (s.dn[377][9] * ddt_scale));let eq23_e637_d_b0: f64 = (eq23_e634_d_b0 + (s.db[377][0] * ddt_scale));let eq23_e637_d_b1: f64 = (eq23_e634_d_b1 + (s.db[377][1] * ddt_scale));let eq23_e637_d_b2: f64 = (eq23_e634_d_b2 + (s.db[377][2] * ddt_scale));let eq23_e637_d_b3: f64 = (eq23_e634_d_b3 + (s.db[377][3] * ddt_scale));let eq23_e638: f64 = (p.p14 * eq23_e637);let eq23_e638_d_n0: f64 = (p.p14 * eq23_e637_d_n0);let eq23_e638_d_n1: f64 = (p.p14 * eq23_e637_d_n1);let eq23_e638_d_n2: f64 = (p.p14 * eq23_e637_d_n2);let eq23_e638_d_n3: f64 = (p.p14 * eq23_e637_d_n3);let eq23_e638_d_n4: f64 = (p.p14 * eq23_e637_d_n4);let eq23_e638_d_n5: f64 = (p.p14 * eq23_e637_d_n5);let eq23_e638_d_n6: f64 = (p.p14 * eq23_e637_d_n6);let eq23_e638_d_n7: f64 = (p.p14 * eq23_e637_d_n7);let eq23_e638_d_n8: f64 = (p.p14 * eq23_e637_d_n8);let eq23_e638_d_n9: f64 = (p.p14 * eq23_e637_d_n9);let eq23_e638_d_b0: f64 = (p.p14 * eq23_e637_d_b0);let eq23_e638_d_b1: f64 = (p.p14 * eq23_e637_d_b1);let eq23_e638_d_b2: f64 = (p.p14 * eq23_e637_d_b2);let eq23_e638_d_b3: f64 = (p.p14 * eq23_e637_d_b3);let eq23_value: f64 = eq23_e638;let eq23_node_derivatives: [f64; 10] = [eq23_e638_d_n0, eq23_e638_d_n1, eq23_e638_d_n2, eq23_e638_d_n3, eq23_e638_d_n4, eq23_e638_d_n5, eq23_e638_d_n6, eq23_e638_d_n7, eq23_e638_d_n8, eq23_e638_d_n9];
        let eq23_branch_derivatives: [f64; 4] = [eq23_e638_d_b0, eq23_e638_d_b1, eq23_e638_d_b2, eq23_e638_d_b3];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
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
        let eq24_e641: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, s.v[367]);let eq24_e643: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, s.v[369]);let eq24_e644: f64 = (eq24_e641 + eq24_e643);let eq24_e644_d_n0: f64 = ((s.dn[367][0] * ddt_scale) + (s.dn[369][0] * ddt_scale));let eq24_e644_d_n1: f64 = ((s.dn[367][1] * ddt_scale) + (s.dn[369][1] * ddt_scale));let eq24_e644_d_n2: f64 = ((s.dn[367][2] * ddt_scale) + (s.dn[369][2] * ddt_scale));let eq24_e644_d_n3: f64 = ((s.dn[367][3] * ddt_scale) + (s.dn[369][3] * ddt_scale));let eq24_e644_d_n4: f64 = ((s.dn[367][4] * ddt_scale) + (s.dn[369][4] * ddt_scale));let eq24_e644_d_n5: f64 = ((s.dn[367][5] * ddt_scale) + (s.dn[369][5] * ddt_scale));let eq24_e644_d_n6: f64 = ((s.dn[367][6] * ddt_scale) + (s.dn[369][6] * ddt_scale));let eq24_e644_d_n7: f64 = ((s.dn[367][7] * ddt_scale) + (s.dn[369][7] * ddt_scale));let eq24_e644_d_n8: f64 = ((s.dn[367][8] * ddt_scale) + (s.dn[369][8] * ddt_scale));let eq24_e644_d_n9: f64 = ((s.dn[367][9] * ddt_scale) + (s.dn[369][9] * ddt_scale));let eq24_e644_d_b0: f64 = ((s.db[367][0] * ddt_scale) + (s.db[369][0] * ddt_scale));let eq24_e644_d_b1: f64 = ((s.db[367][1] * ddt_scale) + (s.db[369][1] * ddt_scale));let eq24_e644_d_b2: f64 = ((s.db[367][2] * ddt_scale) + (s.db[369][2] * ddt_scale));let eq24_e644_d_b3: f64 = ((s.db[367][3] * ddt_scale) + (s.db[369][3] * ddt_scale));let eq24_e646: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, s.v[376]);let eq24_e647: f64 = (eq24_e644 + eq24_e646);let eq24_e647_d_n0: f64 = (eq24_e644_d_n0 + (s.dn[376][0] * ddt_scale));let eq24_e647_d_n1: f64 = (eq24_e644_d_n1 + (s.dn[376][1] * ddt_scale));let eq24_e647_d_n2: f64 = (eq24_e644_d_n2 + (s.dn[376][2] * ddt_scale));let eq24_e647_d_n3: f64 = (eq24_e644_d_n3 + (s.dn[376][3] * ddt_scale));let eq24_e647_d_n4: f64 = (eq24_e644_d_n4 + (s.dn[376][4] * ddt_scale));let eq24_e647_d_n5: f64 = (eq24_e644_d_n5 + (s.dn[376][5] * ddt_scale));let eq24_e647_d_n6: f64 = (eq24_e644_d_n6 + (s.dn[376][6] * ddt_scale));let eq24_e647_d_n7: f64 = (eq24_e644_d_n7 + (s.dn[376][7] * ddt_scale));let eq24_e647_d_n8: f64 = (eq24_e644_d_n8 + (s.dn[376][8] * ddt_scale));let eq24_e647_d_n9: f64 = (eq24_e644_d_n9 + (s.dn[376][9] * ddt_scale));let eq24_e647_d_b0: f64 = (eq24_e644_d_b0 + (s.db[376][0] * ddt_scale));let eq24_e647_d_b1: f64 = (eq24_e644_d_b1 + (s.db[376][1] * ddt_scale));let eq24_e647_d_b2: f64 = (eq24_e644_d_b2 + (s.db[376][2] * ddt_scale));let eq24_e647_d_b3: f64 = (eq24_e644_d_b3 + (s.db[376][3] * ddt_scale));let eq24_e648: f64 = (p.p14 * eq24_e647);let eq24_e648_d_n0: f64 = (p.p14 * eq24_e647_d_n0);let eq24_e648_d_n1: f64 = (p.p14 * eq24_e647_d_n1);let eq24_e648_d_n2: f64 = (p.p14 * eq24_e647_d_n2);let eq24_e648_d_n3: f64 = (p.p14 * eq24_e647_d_n3);let eq24_e648_d_n4: f64 = (p.p14 * eq24_e647_d_n4);let eq24_e648_d_n5: f64 = (p.p14 * eq24_e647_d_n5);let eq24_e648_d_n6: f64 = (p.p14 * eq24_e647_d_n6);let eq24_e648_d_n7: f64 = (p.p14 * eq24_e647_d_n7);let eq24_e648_d_n8: f64 = (p.p14 * eq24_e647_d_n8);let eq24_e648_d_n9: f64 = (p.p14 * eq24_e647_d_n9);let eq24_e648_d_b0: f64 = (p.p14 * eq24_e647_d_b0);let eq24_e648_d_b1: f64 = (p.p14 * eq24_e647_d_b1);let eq24_e648_d_b2: f64 = (p.p14 * eq24_e647_d_b2);let eq24_e648_d_b3: f64 = (p.p14 * eq24_e647_d_b3);let eq24_value: f64 = eq24_e648;let eq24_node_derivatives: [f64; 10] = [eq24_e648_d_n0, eq24_e648_d_n1, eq24_e648_d_n2, eq24_e648_d_n3, eq24_e648_d_n4, eq24_e648_d_n5, eq24_e648_d_n6, eq24_e648_d_n7, eq24_e648_d_n8, eq24_e648_d_n9];
        let eq24_branch_derivatives: [f64; 4] = [eq24_e648_d_b0, eq24_e648_d_b1, eq24_e648_d_b2, eq24_e648_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );let eq25_e651: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[372]);let eq25_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, s.v[378]);let eq25_e654: f64 = (eq25_e651 + eq25_e653);let eq25_e654_d_n0: f64 = ((s.dn[372][0] * ddt_scale) + (s.dn[378][0] * ddt_scale));let eq25_e654_d_n1: f64 = ((s.dn[372][1] * ddt_scale) + (s.dn[378][1] * ddt_scale));let eq25_e654_d_n2: f64 = ((s.dn[372][2] * ddt_scale) + (s.dn[378][2] * ddt_scale));let eq25_e654_d_n3: f64 = ((s.dn[372][3] * ddt_scale) + (s.dn[378][3] * ddt_scale));let eq25_e654_d_n4: f64 = ((s.dn[372][4] * ddt_scale) + (s.dn[378][4] * ddt_scale));let eq25_e654_d_n5: f64 = ((s.dn[372][5] * ddt_scale) + (s.dn[378][5] * ddt_scale));let eq25_e654_d_n6: f64 = ((s.dn[372][6] * ddt_scale) + (s.dn[378][6] * ddt_scale));let eq25_e654_d_n7: f64 = ((s.dn[372][7] * ddt_scale) + (s.dn[378][7] * ddt_scale));let eq25_e654_d_n8: f64 = ((s.dn[372][8] * ddt_scale) + (s.dn[378][8] * ddt_scale));let eq25_e654_d_n9: f64 = ((s.dn[372][9] * ddt_scale) + (s.dn[378][9] * ddt_scale));let eq25_e654_d_b0: f64 = ((s.db[372][0] * ddt_scale) + (s.db[378][0] * ddt_scale));let eq25_e654_d_b1: f64 = ((s.db[372][1] * ddt_scale) + (s.db[378][1] * ddt_scale));let eq25_e654_d_b2: f64 = ((s.db[372][2] * ddt_scale) + (s.db[378][2] * ddt_scale));let eq25_e654_d_b3: f64 = ((s.db[372][3] * ddt_scale) + (s.db[378][3] * ddt_scale));let eq25_e655: f64 = (p.p14 * eq25_e654);let eq25_e655_d_n0: f64 = (p.p14 * eq25_e654_d_n0);let eq25_e655_d_n1: f64 = (p.p14 * eq25_e654_d_n1);let eq25_e655_d_n2: f64 = (p.p14 * eq25_e654_d_n2);let eq25_e655_d_n3: f64 = (p.p14 * eq25_e654_d_n3);let eq25_e655_d_n4: f64 = (p.p14 * eq25_e654_d_n4);let eq25_e655_d_n5: f64 = (p.p14 * eq25_e654_d_n5);let eq25_e655_d_n6: f64 = (p.p14 * eq25_e654_d_n6);let eq25_e655_d_n7: f64 = (p.p14 * eq25_e654_d_n7);let eq25_e655_d_n8: f64 = (p.p14 * eq25_e654_d_n8);let eq25_e655_d_n9: f64 = (p.p14 * eq25_e654_d_n9);let eq25_e655_d_b0: f64 = (p.p14 * eq25_e654_d_b0);let eq25_e655_d_b1: f64 = (p.p14 * eq25_e654_d_b1);let eq25_e655_d_b2: f64 = (p.p14 * eq25_e654_d_b2);let eq25_e655_d_b3: f64 = (p.p14 * eq25_e654_d_b3);let eq25_value: f64 = eq25_e655;let eq25_node_derivatives: [f64; 10] = [eq25_e655_d_n0, eq25_e655_d_n1, eq25_e655_d_n2, eq25_e655_d_n3, eq25_e655_d_n4, eq25_e655_d_n5, eq25_e655_d_n6, eq25_e655_d_n7, eq25_e655_d_n8, eq25_e655_d_n9];let eq25_branch_derivatives: [f64; 4] = [eq25_e655_d_b0, eq25_e655_d_b1, eq25_e655_d_b2, eq25_e655_d_b3];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
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
        let eq26_e658: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, s.v[370]);let eq26_e659: f64 = (p.p14 * eq26_e658);let eq26_e659_d_n0: f64 = (p.p14 * (s.dn[370][0] * ddt_scale));let eq26_e659_d_n1: f64 = (p.p14 * (s.dn[370][1] * ddt_scale));let eq26_e659_d_n2: f64 = (p.p14 * (s.dn[370][2] * ddt_scale));let eq26_e659_d_n3: f64 = (p.p14 * (s.dn[370][3] * ddt_scale));let eq26_e659_d_n4: f64 = (p.p14 * (s.dn[370][4] * ddt_scale));let eq26_e659_d_n5: f64 = (p.p14 * (s.dn[370][5] * ddt_scale));let eq26_e659_d_n6: f64 = (p.p14 * (s.dn[370][6] * ddt_scale));let eq26_e659_d_n7: f64 = (p.p14 * (s.dn[370][7] * ddt_scale));let eq26_e659_d_n8: f64 = (p.p14 * (s.dn[370][8] * ddt_scale));let eq26_e659_d_n9: f64 = (p.p14 * (s.dn[370][9] * ddt_scale));let eq26_e659_d_b0: f64 = (p.p14 * (s.db[370][0] * ddt_scale));let eq26_e659_d_b1: f64 = (p.p14 * (s.db[370][1] * ddt_scale));let eq26_e659_d_b2: f64 = (p.p14 * (s.db[370][2] * ddt_scale));let eq26_e659_d_b3: f64 = (p.p14 * (s.db[370][3] * ddt_scale));let eq26_value: f64 = eq26_e659;let eq26_node_derivatives: [f64; 10] = [eq26_e659_d_n0, eq26_e659_d_n1, eq26_e659_d_n2, eq26_e659_d_n3, eq26_e659_d_n4, eq26_e659_d_n5, eq26_e659_d_n6, eq26_e659_d_n7, eq26_e659_d_n8, eq26_e659_d_n9];let eq26_branch_derivatives: [f64; 4] = [eq26_e659_d_b0, eq26_e659_d_b1, eq26_e659_d_b2, eq26_e659_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );let eq27_e662: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, s.v[357]);let eq27_e664: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[366]);let eq27_e665: f64 = (eq27_e662 + eq27_e664);let eq27_e665_d_n0: f64 = ((s.dn[357][0] * ddt_scale) + (s.dn[366][0] * ddt_scale));let eq27_e665_d_n1: f64 = ((s.dn[357][1] * ddt_scale) + (s.dn[366][1] * ddt_scale));let eq27_e665_d_n2: f64 = ((s.dn[357][2] * ddt_scale) + (s.dn[366][2] * ddt_scale));let eq27_e665_d_n3: f64 = ((s.dn[357][3] * ddt_scale) + (s.dn[366][3] * ddt_scale));let eq27_e665_d_n4: f64 = ((s.dn[357][4] * ddt_scale) + (s.dn[366][4] * ddt_scale));let eq27_e665_d_n5: f64 = ((s.dn[357][5] * ddt_scale) + (s.dn[366][5] * ddt_scale));let eq27_e665_d_n6: f64 = ((s.dn[357][6] * ddt_scale) + (s.dn[366][6] * ddt_scale));let eq27_e665_d_n7: f64 = ((s.dn[357][7] * ddt_scale) + (s.dn[366][7] * ddt_scale));let eq27_e665_d_n8: f64 = ((s.dn[357][8] * ddt_scale) + (s.dn[366][8] * ddt_scale));let eq27_e665_d_n9: f64 = ((s.dn[357][9] * ddt_scale) + (s.dn[366][9] * ddt_scale));let eq27_e665_d_b0: f64 = ((s.db[357][0] * ddt_scale) + (s.db[366][0] * ddt_scale));let eq27_e665_d_b1: f64 = ((s.db[357][1] * ddt_scale) + (s.db[366][1] * ddt_scale));let eq27_e665_d_b2: f64 = ((s.db[357][2] * ddt_scale) + (s.db[366][2] * ddt_scale));let eq27_e665_d_b3: f64 = ((s.db[357][3] * ddt_scale) + (s.db[366][3] * ddt_scale));let eq27_e667: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, s.v[368]);let eq27_e668: f64 = (eq27_e665 + eq27_e667);let eq27_e668_d_n0: f64 = (eq27_e665_d_n0 + (s.dn[368][0] * ddt_scale));let eq27_e668_d_n1: f64 = (eq27_e665_d_n1 + (s.dn[368][1] * ddt_scale));let eq27_e668_d_n2: f64 = (eq27_e665_d_n2 + (s.dn[368][2] * ddt_scale));let eq27_e668_d_n3: f64 = (eq27_e665_d_n3 + (s.dn[368][3] * ddt_scale));let eq27_e668_d_n4: f64 = (eq27_e665_d_n4 + (s.dn[368][4] * ddt_scale));let eq27_e668_d_n5: f64 = (eq27_e665_d_n5 + (s.dn[368][5] * ddt_scale));let eq27_e668_d_n6: f64 = (eq27_e665_d_n6 + (s.dn[368][6] * ddt_scale));let eq27_e668_d_n7: f64 = (eq27_e665_d_n7 + (s.dn[368][7] * ddt_scale));let eq27_e668_d_n8: f64 = (eq27_e665_d_n8 + (s.dn[368][8] * ddt_scale));let eq27_e668_d_n9: f64 = (eq27_e665_d_n9 + (s.dn[368][9] * ddt_scale));let eq27_e668_d_b0: f64 = (eq27_e665_d_b0 + (s.db[368][0] * ddt_scale));let eq27_e668_d_b1: f64 = (eq27_e665_d_b1 + (s.db[368][1] * ddt_scale));let eq27_e668_d_b2: f64 = (eq27_e665_d_b2 + (s.db[368][2] * ddt_scale));let eq27_e668_d_b3: f64 = (eq27_e665_d_b3 + (s.db[368][3] * ddt_scale));let eq27_e670: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, s.v[375]);let eq27_e671: f64 = (eq27_e668 + eq27_e670);let eq27_e671_d_n0: f64 = (eq27_e668_d_n0 + (s.dn[375][0] * ddt_scale));let eq27_e671_d_n1: f64 = (eq27_e668_d_n1 + (s.dn[375][1] * ddt_scale));let eq27_e671_d_n2: f64 = (eq27_e668_d_n2 + (s.dn[375][2] * ddt_scale));let eq27_e671_d_n3: f64 = (eq27_e668_d_n3 + (s.dn[375][3] * ddt_scale));let eq27_e671_d_n4: f64 = (eq27_e668_d_n4 + (s.dn[375][4] * ddt_scale));let eq27_e671_d_n5: f64 = (eq27_e668_d_n5 + (s.dn[375][5] * ddt_scale));let eq27_e671_d_n6: f64 = (eq27_e668_d_n6 + (s.dn[375][6] * ddt_scale));let eq27_e671_d_n7: f64 = (eq27_e668_d_n7 + (s.dn[375][7] * ddt_scale));let eq27_e671_d_n8: f64 = (eq27_e668_d_n8 + (s.dn[375][8] * ddt_scale));
        let eq27_e671_d_n9: f64 = (eq27_e668_d_n9 + (s.dn[375][9] * ddt_scale));let eq27_e671_d_b0: f64 = (eq27_e668_d_b0 + (s.db[375][0] * ddt_scale));let eq27_e671_d_b1: f64 = (eq27_e668_d_b1 + (s.db[375][1] * ddt_scale));let eq27_e671_d_b2: f64 = (eq27_e668_d_b2 + (s.db[375][2] * ddt_scale));let eq27_e671_d_b3: f64 = (eq27_e668_d_b3 + (s.db[375][3] * ddt_scale));let eq27_e672: f64 = (p.p14 * eq27_e671);let eq27_e672_d_n0: f64 = (p.p14 * eq27_e671_d_n0);let eq27_e672_d_n1: f64 = (p.p14 * eq27_e671_d_n1);let eq27_e672_d_n2: f64 = (p.p14 * eq27_e671_d_n2);let eq27_e672_d_n3: f64 = (p.p14 * eq27_e671_d_n3);let eq27_e672_d_n4: f64 = (p.p14 * eq27_e671_d_n4);let eq27_e672_d_n5: f64 = (p.p14 * eq27_e671_d_n5);let eq27_e672_d_n6: f64 = (p.p14 * eq27_e671_d_n6);let eq27_e672_d_n7: f64 = (p.p14 * eq27_e671_d_n7);let eq27_e672_d_n8: f64 = (p.p14 * eq27_e671_d_n8);let eq27_e672_d_n9: f64 = (p.p14 * eq27_e671_d_n9);let eq27_e672_d_b0: f64 = (p.p14 * eq27_e671_d_b0);let eq27_e672_d_b1: f64 = (p.p14 * eq27_e671_d_b1);let eq27_e672_d_b2: f64 = (p.p14 * eq27_e671_d_b2);let eq27_e672_d_b3: f64 = (p.p14 * eq27_e671_d_b3);let eq27_value: f64 = eq27_e672;let eq27_node_derivatives: [f64; 10] = [eq27_e672_d_n0, eq27_e672_d_n1, eq27_e672_d_n2, eq27_e672_d_n3, eq27_e672_d_n4, eq27_e672_d_n5, eq27_e672_d_n6, eq27_e672_d_n7, eq27_e672_d_n8, eq27_e672_d_n9];let eq27_branch_derivatives: [f64; 4] = [eq27_e672_d_b0, eq27_e672_d_b1, eq27_e672_d_b2, eq27_e672_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
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
        let eq28_e675: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, s.v[359]);let eq28_e677: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, s.v[371]);let eq28_e678: f64 = (eq28_e675 + eq28_e677);let eq28_e678_d_n0: f64 = ((s.dn[359][0] * ddt_scale) + (s.dn[371][0] * ddt_scale));let eq28_e678_d_n1: f64 = ((s.dn[359][1] * ddt_scale) + (s.dn[371][1] * ddt_scale));let eq28_e678_d_n2: f64 = ((s.dn[359][2] * ddt_scale) + (s.dn[371][2] * ddt_scale));let eq28_e678_d_n3: f64 = ((s.dn[359][3] * ddt_scale) + (s.dn[371][3] * ddt_scale));let eq28_e678_d_n4: f64 = ((s.dn[359][4] * ddt_scale) + (s.dn[371][4] * ddt_scale));let eq28_e678_d_n5: f64 = ((s.dn[359][5] * ddt_scale) + (s.dn[371][5] * ddt_scale));let eq28_e678_d_n6: f64 = ((s.dn[359][6] * ddt_scale) + (s.dn[371][6] * ddt_scale));let eq28_e678_d_n7: f64 = ((s.dn[359][7] * ddt_scale) + (s.dn[371][7] * ddt_scale));let eq28_e678_d_n8: f64 = ((s.dn[359][8] * ddt_scale) + (s.dn[371][8] * ddt_scale));let eq28_e678_d_n9: f64 = ((s.dn[359][9] * ddt_scale) + (s.dn[371][9] * ddt_scale));let eq28_e678_d_b0: f64 = ((s.db[359][0] * ddt_scale) + (s.db[371][0] * ddt_scale));let eq28_e678_d_b1: f64 = ((s.db[359][1] * ddt_scale) + (s.db[371][1] * ddt_scale));let eq28_e678_d_b2: f64 = ((s.db[359][2] * ddt_scale) + (s.db[371][2] * ddt_scale));let eq28_e678_d_b3: f64 = ((s.db[359][3] * ddt_scale) + (s.db[371][3] * ddt_scale));let eq28_e679: f64 = (p.p14 * eq28_e678);let eq28_e679_d_n0: f64 = (p.p14 * eq28_e678_d_n0);let eq28_e679_d_n1: f64 = (p.p14 * eq28_e678_d_n1);let eq28_e679_d_n2: f64 = (p.p14 * eq28_e678_d_n2);let eq28_e679_d_n3: f64 = (p.p14 * eq28_e678_d_n3);let eq28_e679_d_n4: f64 = (p.p14 * eq28_e678_d_n4);let eq28_e679_d_n5: f64 = (p.p14 * eq28_e678_d_n5);let eq28_e679_d_n6: f64 = (p.p14 * eq28_e678_d_n6);let eq28_e679_d_n7: f64 = (p.p14 * eq28_e678_d_n7);let eq28_e679_d_n8: f64 = (p.p14 * eq28_e678_d_n8);let eq28_e679_d_n9: f64 = (p.p14 * eq28_e678_d_n9);let eq28_e679_d_b0: f64 = (p.p14 * eq28_e678_d_b0);let eq28_e679_d_b1: f64 = (p.p14 * eq28_e678_d_b1);let eq28_e679_d_b2: f64 = (p.p14 * eq28_e678_d_b2);let eq28_e679_d_b3: f64 = (p.p14 * eq28_e678_d_b3);let eq28_value: f64 = eq28_e679;let eq28_node_derivatives: [f64; 10] = [eq28_e679_d_n0, eq28_e679_d_n1, eq28_e679_d_n2, eq28_e679_d_n3, eq28_e679_d_n4, eq28_e679_d_n5, eq28_e679_d_n6, eq28_e679_d_n7, eq28_e679_d_n8, eq28_e679_d_n9];let eq28_branch_derivatives: [f64; 4] = [eq28_e679_d_b0, eq28_e679_d_b1, eq28_e679_d_b2, eq28_e679_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );let eq29_e681: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, s.v[374]);let eq29_value: f64 = eq29_e681;
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq29_value),
            &s.dn[374],
            &s.db[374],
            (multiplicity) * (ddt_scale),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
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
        let nv5 = ctx.node_voltage(nodes[5]);let eq31_e687: f64 = (s.v[1793] * (nv5 - 0.0));let eq31_e687_d_n0: f64 = (s.dn[1793][0] * (nv5 - 0.0));let eq31_e687_d_n1: f64 = (s.dn[1793][1] * (nv5 - 0.0));let eq31_e687_d_n2: f64 = (s.dn[1793][2] * (nv5 - 0.0));let eq31_e687_d_n3: f64 = (s.dn[1793][3] * (nv5 - 0.0));let eq31_e687_d_n4: f64 = (s.dn[1793][4] * (nv5 - 0.0));let eq31_e687_d_n5: f64 = ((s.dn[1793][5] * (nv5 - 0.0)) + s.v[1793]);let eq31_e687_d_n6: f64 = (s.dn[1793][6] * (nv5 - 0.0));let eq31_e687_d_n7: f64 = (s.dn[1793][7] * (nv5 - 0.0));let eq31_e687_d_n8: f64 = (s.dn[1793][8] * (nv5 - 0.0));let eq31_e687_d_n9: f64 = (s.dn[1793][9] * (nv5 - 0.0));let eq31_e687_d_b0: f64 = (s.db[1793][0] * (nv5 - 0.0));let eq31_e687_d_b1: f64 = (s.db[1793][1] * (nv5 - 0.0));let eq31_e687_d_b2: f64 = (s.db[1793][2] * (nv5 - 0.0));let eq31_e687_d_b3: f64 = (s.db[1793][3] * (nv5 - 0.0));let eq31_value: f64 = eq31_e687;let eq31_node_derivatives: [f64; 10] = [eq31_e687_d_n0, eq31_e687_d_n1, eq31_e687_d_n2, eq31_e687_d_n3, eq31_e687_d_n4, eq31_e687_d_n5, eq31_e687_d_n6, eq31_e687_d_n7, eq31_e687_d_n8, eq31_e687_d_n9];let eq31_branch_derivatives: [f64; 4] = [eq31_e687_d_b0, eq31_e687_d_b1, eq31_e687_d_b2, eq31_e687_d_b3];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );let eq32_e690: f64 = (s.v[1790] * (nv5 - 0.0));let eq32_e690_d_n0: f64 = (s.dn[1790][0] * (nv5 - 0.0));let eq32_e690_d_n1: f64 = (s.dn[1790][1] * (nv5 - 0.0));let eq32_e690_d_n2: f64 = (s.dn[1790][2] * (nv5 - 0.0));let eq32_e690_d_n3: f64 = (s.dn[1790][3] * (nv5 - 0.0));let eq32_e690_d_n4: f64 = (s.dn[1790][4] * (nv5 - 0.0));let eq32_e690_d_n5: f64 = ((s.dn[1790][5] * (nv5 - 0.0)) + s.v[1790]);let eq32_e690_d_n6: f64 = (s.dn[1790][6] * (nv5 - 0.0));let eq32_e690_d_n7: f64 = (s.dn[1790][7] * (nv5 - 0.0));let eq32_e690_d_n8: f64 = (s.dn[1790][8] * (nv5 - 0.0));let eq32_e690_d_n9: f64 = (s.dn[1790][9] * (nv5 - 0.0));let eq32_e690_d_b0: f64 = (s.db[1790][0] * (nv5 - 0.0));let eq32_e690_d_b1: f64 = (s.db[1790][1] * (nv5 - 0.0));let eq32_e690_d_b2: f64 = (s.db[1790][2] * (nv5 - 0.0));let eq32_e690_d_b3: f64 = (s.db[1790][3] * (nv5 - 0.0));let eq32_e691: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq32_e690);let eq32_value: f64 = eq32_e691;let eq32_node_derivatives: [f64; 10] = [(eq32_e690_d_n0 * ddt_scale), (eq32_e690_d_n1 * ddt_scale), (eq32_e690_d_n2 * ddt_scale), (eq32_e690_d_n3 * ddt_scale), (eq32_e690_d_n4 * ddt_scale), (eq32_e690_d_n5 * ddt_scale), (eq32_e690_d_n6 * ddt_scale), (eq32_e690_d_n7 * ddt_scale), (eq32_e690_d_n8 * ddt_scale), (eq32_e690_d_n9 * ddt_scale)];let eq32_branch_derivatives: [f64; 4] = [(eq32_e690_d_b0 * ddt_scale), (eq32_e690_d_b1 * ddt_scale), (eq32_e690_d_b2 * ddt_scale), (eq32_e690_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );let eq33_e693: f64 = (-s.v[1791]);let eq33_e695: f64 = (eq33_e693 * (nv5 - 0.0));let eq33_e695_d_n0: f64 = ((-s.dn[1791][0]) * (nv5 - 0.0));let eq33_e695_d_n1: f64 = ((-s.dn[1791][1]) * (nv5 - 0.0));let eq33_e695_d_n2: f64 = ((-s.dn[1791][2]) * (nv5 - 0.0));let eq33_e695_d_n3: f64 = ((-s.dn[1791][3]) * (nv5 - 0.0));let eq33_e695_d_n4: f64 = ((-s.dn[1791][4]) * (nv5 - 0.0));let eq33_e695_d_n5: f64 = (((-s.dn[1791][5]) * (nv5 - 0.0)) + eq33_e693);let eq33_e695_d_n6: f64 = ((-s.dn[1791][6]) * (nv5 - 0.0));let eq33_e695_d_n7: f64 = ((-s.dn[1791][7]) * (nv5 - 0.0));let eq33_e695_d_n8: f64 = ((-s.dn[1791][8]) * (nv5 - 0.0));let eq33_e695_d_n9: f64 = ((-s.dn[1791][9]) * (nv5 - 0.0));let eq33_e695_d_b0: f64 = ((-s.db[1791][0]) * (nv5 - 0.0));let eq33_e695_d_b1: f64 = ((-s.db[1791][1]) * (nv5 - 0.0));let eq33_e695_d_b2: f64 = ((-s.db[1791][2]) * (nv5 - 0.0));let eq33_e695_d_b3: f64 = ((-s.db[1791][3]) * (nv5 - 0.0));let eq33_e696: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq33_e695);let eq33_value: f64 = eq33_e696;let eq33_node_derivatives: [f64; 10] = [(eq33_e695_d_n0 * ddt_scale), (eq33_e695_d_n1 * ddt_scale), (eq33_e695_d_n2 * ddt_scale), (eq33_e695_d_n3 * ddt_scale), (eq33_e695_d_n4 * ddt_scale), (eq33_e695_d_n5 * ddt_scale), (eq33_e695_d_n6 * ddt_scale), (eq33_e695_d_n7 * ddt_scale), (eq33_e695_d_n8 * ddt_scale), (eq33_e695_d_n9 * ddt_scale)];let eq33_branch_derivatives: [f64; 4] = [(eq33_e695_d_b0 * ddt_scale), (eq33_e695_d_b1 * ddt_scale), (eq33_e695_d_b2 * ddt_scale), (eq33_e695_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );let eq34_e698: f64 = (-s.v[1792]);let eq34_e700: f64 = (eq34_e698 * (nv5 - 0.0));let eq34_e700_d_n0: f64 = ((-s.dn[1792][0]) * (nv5 - 0.0));let eq34_e700_d_n1: f64 = ((-s.dn[1792][1]) * (nv5 - 0.0));let eq34_e700_d_n2: f64 = ((-s.dn[1792][2]) * (nv5 - 0.0));let eq34_e700_d_n3: f64 = ((-s.dn[1792][3]) * (nv5 - 0.0));let eq34_e700_d_n4: f64 = ((-s.dn[1792][4]) * (nv5 - 0.0));let eq34_e700_d_n5: f64 = (((-s.dn[1792][5]) * (nv5 - 0.0)) + eq34_e698);let eq34_e700_d_n6: f64 = ((-s.dn[1792][6]) * (nv5 - 0.0));let eq34_e700_d_n7: f64 = ((-s.dn[1792][7]) * (nv5 - 0.0));let eq34_e700_d_n8: f64 = ((-s.dn[1792][8]) * (nv5 - 0.0));let eq34_e700_d_n9: f64 = ((-s.dn[1792][9]) * (nv5 - 0.0));let eq34_e700_d_b0: f64 = ((-s.db[1792][0]) * (nv5 - 0.0));let eq34_e700_d_b1: f64 = ((-s.db[1792][1]) * (nv5 - 0.0));let eq34_e700_d_b2: f64 = ((-s.db[1792][2]) * (nv5 - 0.0));let eq34_e700_d_b3: f64 = ((-s.db[1792][3]) * (nv5 - 0.0));let eq34_e701: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq34_e700);let eq34_value: f64 = eq34_e701;let eq34_node_derivatives: [f64; 10] = [(eq34_e700_d_n0 * ddt_scale), (eq34_e700_d_n1 * ddt_scale), (eq34_e700_d_n2 * ddt_scale), (eq34_e700_d_n3 * ddt_scale), (eq34_e700_d_n4 * ddt_scale), (eq34_e700_d_n5 * ddt_scale), (eq34_e700_d_n6 * ddt_scale), (eq34_e700_d_n7 * ddt_scale), (eq34_e700_d_n8 * ddt_scale), (eq34_e700_d_n9 * ddt_scale)];let eq34_branch_derivatives: [f64; 4] = [(eq34_e700_d_b0 * ddt_scale), (eq34_e700_d_b1 * ddt_scale), (eq34_e700_d_b2 * ddt_scale), (eq34_e700_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq23_e631_q: f64 = s.v[358];let eq23_e633_q: f64 = s.v[373];let eq23_e634: f64 = (s.v[358] + s.v[373]);let eq23_e634_d_n0: f64 = (s.dn[358][0] + s.dn[373][0]);let eq23_e634_d_n1: f64 = (s.dn[358][1] + s.dn[373][1]);let eq23_e634_d_n2: f64 = (s.dn[358][2] + s.dn[373][2]);let eq23_e634_d_n3: f64 = (s.dn[358][3] + s.dn[373][3]);let eq23_e634_d_n4: f64 = (s.dn[358][4] + s.dn[373][4]);let eq23_e634_d_n5: f64 = (s.dn[358][5] + s.dn[373][5]);let eq23_e634_d_n6: f64 = (s.dn[358][6] + s.dn[373][6]);let eq23_e634_d_n7: f64 = (s.dn[358][7] + s.dn[373][7]);let eq23_e634_d_n8: f64 = (s.dn[358][8] + s.dn[373][8]);let eq23_e634_d_n9: f64 = (s.dn[358][9] + s.dn[373][9]);let eq23_e634_d_b0: f64 = (s.db[358][0] + s.db[373][0]);let eq23_e634_d_b1: f64 = (s.db[358][1] + s.db[373][1]);let eq23_e634_d_b2: f64 = (s.db[358][2] + s.db[373][2]);let eq23_e634_d_b3: f64 = (s.db[358][3] + s.db[373][3]);let eq23_e634_q: f64 = (eq23_e631_q + eq23_e633_q);let eq23_e636_q: f64 = s.v[377];let eq23_e637: f64 = (eq23_e634 + s.v[377]);let eq23_e637_d_n0: f64 = (eq23_e634_d_n0 + s.dn[377][0]);let eq23_e637_d_n1: f64 = (eq23_e634_d_n1 + s.dn[377][1]);let eq23_e637_d_n2: f64 = (eq23_e634_d_n2 + s.dn[377][2]);let eq23_e637_d_n3: f64 = (eq23_e634_d_n3 + s.dn[377][3]);let eq23_e637_d_n4: f64 = (eq23_e634_d_n4 + s.dn[377][4]);let eq23_e637_d_n5: f64 = (eq23_e634_d_n5 + s.dn[377][5]);let eq23_e637_d_n6: f64 = (eq23_e634_d_n6 + s.dn[377][6]);let eq23_e637_d_n7: f64 = (eq23_e634_d_n7 + s.dn[377][7]);let eq23_e637_d_n8: f64 = (eq23_e634_d_n8 + s.dn[377][8]);let eq23_e637_d_n9: f64 = (eq23_e634_d_n9 + s.dn[377][9]);let eq23_e637_d_b0: f64 = (eq23_e634_d_b0 + s.db[377][0]);let eq23_e637_d_b1: f64 = (eq23_e634_d_b1 + s.db[377][1]);let eq23_e637_d_b2: f64 = (eq23_e634_d_b2 + s.db[377][2]);let eq23_e637_d_b3: f64 = (eq23_e634_d_b3 + s.db[377][3]);let eq23_e637_q: f64 = (eq23_e634_q + eq23_e636_q);let eq23_e638: f64 = (p.p14 * eq23_e637);let eq23_e638_d_n0: f64 = (p.p14 * eq23_e637_d_n0);let eq23_e638_d_n1: f64 = (p.p14 * eq23_e637_d_n1);let eq23_e638_d_n2: f64 = (p.p14 * eq23_e637_d_n2);let eq23_e638_d_n3: f64 = (p.p14 * eq23_e637_d_n3);let eq23_e638_d_n4: f64 = (p.p14 * eq23_e637_d_n4);let eq23_e638_d_n5: f64 = (p.p14 * eq23_e637_d_n5);let eq23_e638_d_n6: f64 = (p.p14 * eq23_e637_d_n6);let eq23_e638_d_n7: f64 = (p.p14 * eq23_e637_d_n7);let eq23_e638_d_n8: f64 = (p.p14 * eq23_e637_d_n8);let eq23_e638_d_n9: f64 = (p.p14 * eq23_e637_d_n9);let eq23_e638_d_b0: f64 = (p.p14 * eq23_e637_d_b0);let eq23_e638_d_b1: f64 = (p.p14 * eq23_e637_d_b1);let eq23_e638_d_b2: f64 = (p.p14 * eq23_e637_d_b2);let eq23_e638_d_b3: f64 = (p.p14 * eq23_e637_d_b3);let eq23_e638_q: f64 = (p.p14 * eq23_e637_q);let eq23_reactive_node_derivatives: [f64; 10] = [eq23_e638_d_n0, eq23_e638_d_n1, eq23_e638_d_n2, eq23_e638_d_n3, eq23_e638_d_n4, eq23_e638_d_n5, eq23_e638_d_n6, eq23_e638_d_n7, eq23_e638_d_n8, eq23_e638_d_n9];let eq23_reactive_branch_derivatives: [f64; 4] = [eq23_e638_d_b0, eq23_e638_d_b1, eq23_e638_d_b2, eq23_e638_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(6),
            &eq23_reactive_node_derivatives,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq24_e641_q: f64 = s.v[367];let eq24_e643_q: f64 = s.v[369];let eq24_e644: f64 = (s.v[367] + s.v[369]);let eq24_e644_d_n0: f64 = (s.dn[367][0] + s.dn[369][0]);let eq24_e644_d_n1: f64 = (s.dn[367][1] + s.dn[369][1]);let eq24_e644_d_n2: f64 = (s.dn[367][2] + s.dn[369][2]);let eq24_e644_d_n3: f64 = (s.dn[367][3] + s.dn[369][3]);let eq24_e644_d_n4: f64 = (s.dn[367][4] + s.dn[369][4]);let eq24_e644_d_n5: f64 = (s.dn[367][5] + s.dn[369][5]);let eq24_e644_d_n6: f64 = (s.dn[367][6] + s.dn[369][6]);let eq24_e644_d_n7: f64 = (s.dn[367][7] + s.dn[369][7]);let eq24_e644_d_n8: f64 = (s.dn[367][8] + s.dn[369][8]);let eq24_e644_d_n9: f64 = (s.dn[367][9] + s.dn[369][9]);let eq24_e644_d_b0: f64 = (s.db[367][0] + s.db[369][0]);let eq24_e644_d_b1: f64 = (s.db[367][1] + s.db[369][1]);let eq24_e644_d_b2: f64 = (s.db[367][2] + s.db[369][2]);let eq24_e644_d_b3: f64 = (s.db[367][3] + s.db[369][3]);let eq24_e644_q: f64 = (eq24_e641_q + eq24_e643_q);let eq24_e646_q: f64 = s.v[376];let eq24_e647: f64 = (eq24_e644 + s.v[376]);let eq24_e647_d_n0: f64 = (eq24_e644_d_n0 + s.dn[376][0]);let eq24_e647_d_n1: f64 = (eq24_e644_d_n1 + s.dn[376][1]);let eq24_e647_d_n2: f64 = (eq24_e644_d_n2 + s.dn[376][2]);let eq24_e647_d_n3: f64 = (eq24_e644_d_n3 + s.dn[376][3]);let eq24_e647_d_n4: f64 = (eq24_e644_d_n4 + s.dn[376][4]);let eq24_e647_d_n5: f64 = (eq24_e644_d_n5 + s.dn[376][5]);let eq24_e647_d_n6: f64 = (eq24_e644_d_n6 + s.dn[376][6]);let eq24_e647_d_n7: f64 = (eq24_e644_d_n7 + s.dn[376][7]);let eq24_e647_d_n8: f64 = (eq24_e644_d_n8 + s.dn[376][8]);let eq24_e647_d_n9: f64 = (eq24_e644_d_n9 + s.dn[376][9]);let eq24_e647_d_b0: f64 = (eq24_e644_d_b0 + s.db[376][0]);let eq24_e647_d_b1: f64 = (eq24_e644_d_b1 + s.db[376][1]);let eq24_e647_d_b2: f64 = (eq24_e644_d_b2 + s.db[376][2]);let eq24_e647_d_b3: f64 = (eq24_e644_d_b3 + s.db[376][3]);let eq24_e647_q: f64 = (eq24_e644_q + eq24_e646_q);let eq24_e648: f64 = (p.p14 * eq24_e647);let eq24_e648_d_n0: f64 = (p.p14 * eq24_e647_d_n0);let eq24_e648_d_n1: f64 = (p.p14 * eq24_e647_d_n1);let eq24_e648_d_n2: f64 = (p.p14 * eq24_e647_d_n2);let eq24_e648_d_n3: f64 = (p.p14 * eq24_e647_d_n3);let eq24_e648_d_n4: f64 = (p.p14 * eq24_e647_d_n4);let eq24_e648_d_n5: f64 = (p.p14 * eq24_e647_d_n5);let eq24_e648_d_n6: f64 = (p.p14 * eq24_e647_d_n6);let eq24_e648_d_n7: f64 = (p.p14 * eq24_e647_d_n7);let eq24_e648_d_n8: f64 = (p.p14 * eq24_e647_d_n8);let eq24_e648_d_n9: f64 = (p.p14 * eq24_e647_d_n9);let eq24_e648_d_b0: f64 = (p.p14 * eq24_e647_d_b0);let eq24_e648_d_b1: f64 = (p.p14 * eq24_e647_d_b1);let eq24_e648_d_b2: f64 = (p.p14 * eq24_e647_d_b2);let eq24_e648_d_b3: f64 = (p.p14 * eq24_e647_d_b3);let eq24_e648_q: f64 = (p.p14 * eq24_e647_q);let eq24_reactive_node_derivatives: [f64; 10] = [eq24_e648_d_n0, eq24_e648_d_n1, eq24_e648_d_n2, eq24_e648_d_n3, eq24_e648_d_n4, eq24_e648_d_n5, eq24_e648_d_n6, eq24_e648_d_n7, eq24_e648_d_n8, eq24_e648_d_n9];let eq24_reactive_branch_derivatives: [f64; 4] = [eq24_e648_d_b0, eq24_e648_d_b1, eq24_e648_d_b2, eq24_e648_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(7),
            &eq24_reactive_node_derivatives,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );let eq25_e651_q: f64 = s.v[372];let eq25_e653_q: f64 = s.v[378];let eq25_e654: f64 = (s.v[372] + s.v[378]);let eq25_e654_d_n0: f64 = (s.dn[372][0] + s.dn[378][0]);let eq25_e654_d_n1: f64 = (s.dn[372][1] + s.dn[378][1]);let eq25_e654_d_n2: f64 = (s.dn[372][2] + s.dn[378][2]);let eq25_e654_d_n3: f64 = (s.dn[372][3] + s.dn[378][3]);let eq25_e654_d_n4: f64 = (s.dn[372][4] + s.dn[378][4]);let eq25_e654_d_n5: f64 = (s.dn[372][5] + s.dn[378][5]);let eq25_e654_d_n6: f64 = (s.dn[372][6] + s.dn[378][6]);let eq25_e654_d_n7: f64 = (s.dn[372][7] + s.dn[378][7]);let eq25_e654_d_n8: f64 = (s.dn[372][8] + s.dn[378][8]);let eq25_e654_d_n9: f64 = (s.dn[372][9] + s.dn[378][9]);let eq25_e654_d_b0: f64 = (s.db[372][0] + s.db[378][0]);let eq25_e654_d_b1: f64 = (s.db[372][1] + s.db[378][1]);let eq25_e654_d_b2: f64 = (s.db[372][2] + s.db[378][2]);let eq25_e654_d_b3: f64 = (s.db[372][3] + s.db[378][3]);let eq25_e654_q: f64 = (eq25_e651_q + eq25_e653_q);let eq25_e655: f64 = (p.p14 * eq25_e654);let eq25_e655_d_n0: f64 = (p.p14 * eq25_e654_d_n0);let eq25_e655_d_n1: f64 = (p.p14 * eq25_e654_d_n1);let eq25_e655_d_n2: f64 = (p.p14 * eq25_e654_d_n2);let eq25_e655_d_n3: f64 = (p.p14 * eq25_e654_d_n3);let eq25_e655_d_n4: f64 = (p.p14 * eq25_e654_d_n4);let eq25_e655_d_n5: f64 = (p.p14 * eq25_e654_d_n5);let eq25_e655_d_n6: f64 = (p.p14 * eq25_e654_d_n6);let eq25_e655_d_n7: f64 = (p.p14 * eq25_e654_d_n7);let eq25_e655_d_n8: f64 = (p.p14 * eq25_e654_d_n8);let eq25_e655_d_n9: f64 = (p.p14 * eq25_e654_d_n9);let eq25_e655_d_b0: f64 = (p.p14 * eq25_e654_d_b0);let eq25_e655_d_b1: f64 = (p.p14 * eq25_e654_d_b1);let eq25_e655_d_b2: f64 = (p.p14 * eq25_e654_d_b2);let eq25_e655_d_b3: f64 = (p.p14 * eq25_e654_d_b3);let eq25_e655_q: f64 = (p.p14 * eq25_e654_q);let eq25_reactive_node_derivatives: [f64; 10] = [eq25_e655_d_n0, eq25_e655_d_n1, eq25_e655_d_n2, eq25_e655_d_n3, eq25_e655_d_n4, eq25_e655_d_n5, eq25_e655_d_n6, eq25_e655_d_n7, eq25_e655_d_n8, eq25_e655_d_n9];let eq25_reactive_branch_derivatives: [f64; 4] = [eq25_e655_d_b0, eq25_e655_d_b1, eq25_e655_d_b2, eq25_e655_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(7),
            &eq25_reactive_node_derivatives,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq26_e658_q: f64 = s.v[370];let eq26_e659: f64 = (p.p14 * s.v[370]);let eq26_e659_q: f64 = (p.p14 * eq26_e658_q);
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(8),
            &s.dn[370],
            &s.db[370],
            (multiplicity) * (p.p14),
        );let eq27_e662_q: f64 = s.v[357];let eq27_e664_q: f64 = s.v[366];let eq27_e665: f64 = (s.v[357] + s.v[366]);let eq27_e665_d_n0: f64 = (s.dn[357][0] + s.dn[366][0]);let eq27_e665_d_n1: f64 = (s.dn[357][1] + s.dn[366][1]);let eq27_e665_d_n2: f64 = (s.dn[357][2] + s.dn[366][2]);let eq27_e665_d_n3: f64 = (s.dn[357][3] + s.dn[366][3]);let eq27_e665_d_n4: f64 = (s.dn[357][4] + s.dn[366][4]);let eq27_e665_d_n5: f64 = (s.dn[357][5] + s.dn[366][5]);let eq27_e665_d_n6: f64 = (s.dn[357][6] + s.dn[366][6]);let eq27_e665_d_n7: f64 = (s.dn[357][7] + s.dn[366][7]);let eq27_e665_d_n8: f64 = (s.dn[357][8] + s.dn[366][8]);let eq27_e665_d_n9: f64 = (s.dn[357][9] + s.dn[366][9]);let eq27_e665_d_b0: f64 = (s.db[357][0] + s.db[366][0]);let eq27_e665_d_b1: f64 = (s.db[357][1] + s.db[366][1]);let eq27_e665_d_b2: f64 = (s.db[357][2] + s.db[366][2]);let eq27_e665_d_b3: f64 = (s.db[357][3] + s.db[366][3]);let eq27_e665_q: f64 = (eq27_e662_q + eq27_e664_q);let eq27_e667_q: f64 = s.v[368];let eq27_e668: f64 = (eq27_e665 + s.v[368]);let eq27_e668_d_n0: f64 = (eq27_e665_d_n0 + s.dn[368][0]);let eq27_e668_d_n1: f64 = (eq27_e665_d_n1 + s.dn[368][1]);let eq27_e668_d_n2: f64 = (eq27_e665_d_n2 + s.dn[368][2]);let eq27_e668_d_n3: f64 = (eq27_e665_d_n3 + s.dn[368][3]);let eq27_e668_d_n4: f64 = (eq27_e665_d_n4 + s.dn[368][4]);let eq27_e668_d_n5: f64 = (eq27_e665_d_n5 + s.dn[368][5]);let eq27_e668_d_n6: f64 = (eq27_e665_d_n6 + s.dn[368][6]);let eq27_e668_d_n7: f64 = (eq27_e665_d_n7 + s.dn[368][7]);let eq27_e668_d_n8: f64 = (eq27_e665_d_n8 + s.dn[368][8]);let eq27_e668_d_n9: f64 = (eq27_e665_d_n9 + s.dn[368][9]);let eq27_e668_d_b0: f64 = (eq27_e665_d_b0 + s.db[368][0]);let eq27_e668_d_b1: f64 = (eq27_e665_d_b1 + s.db[368][1]);let eq27_e668_d_b2: f64 = (eq27_e665_d_b2 + s.db[368][2]);let eq27_e668_d_b3: f64 = (eq27_e665_d_b3 + s.db[368][3]);let eq27_e668_q: f64 = (eq27_e665_q + eq27_e667_q);let eq27_e670_q: f64 = s.v[375];let eq27_e671: f64 = (eq27_e668 + s.v[375]);let eq27_e671_d_n0: f64 = (eq27_e668_d_n0 + s.dn[375][0]);let eq27_e671_d_n1: f64 = (eq27_e668_d_n1 + s.dn[375][1]);let eq27_e671_d_n2: f64 = (eq27_e668_d_n2 + s.dn[375][2]);let eq27_e671_d_n3: f64 = (eq27_e668_d_n3 + s.dn[375][3]);let eq27_e671_d_n4: f64 = (eq27_e668_d_n4 + s.dn[375][4]);let eq27_e671_d_n5: f64 = (eq27_e668_d_n5 + s.dn[375][5]);let eq27_e671_d_n6: f64 = (eq27_e668_d_n6 + s.dn[375][6]);let eq27_e671_d_n7: f64 = (eq27_e668_d_n7 + s.dn[375][7]);let eq27_e671_d_n8: f64 = (eq27_e668_d_n8 + s.dn[375][8]);let eq27_e671_d_n9: f64 = (eq27_e668_d_n9 + s.dn[375][9]);let eq27_e671_d_b0: f64 = (eq27_e668_d_b0 + s.db[375][0]);let eq27_e671_d_b1: f64 = (eq27_e668_d_b1 + s.db[375][1]);let eq27_e671_d_b2: f64 = (eq27_e668_d_b2 + s.db[375][2]);let eq27_e671_d_b3: f64 = (eq27_e668_d_b3 + s.db[375][3]);let eq27_e671_q: f64 = (eq27_e668_q + eq27_e670_q);let eq27_e672: f64 = (p.p14 * eq27_e671);let eq27_e672_d_n0: f64 = (p.p14 * eq27_e671_d_n0);let eq27_e672_d_n1: f64 = (p.p14 * eq27_e671_d_n1);let eq27_e672_d_n2: f64 = (p.p14 * eq27_e671_d_n2);let eq27_e672_d_n3: f64 = (p.p14 * eq27_e671_d_n3);let eq27_e672_d_n4: f64 = (p.p14 * eq27_e671_d_n4);let eq27_e672_d_n5: f64 = (p.p14 * eq27_e671_d_n5);let eq27_e672_d_n6: f64 = (p.p14 * eq27_e671_d_n6);let eq27_e672_d_n7: f64 = (p.p14 * eq27_e671_d_n7);let eq27_e672_d_n8: f64 = (p.p14 * eq27_e671_d_n8);let eq27_e672_d_n9: f64 = (p.p14 * eq27_e671_d_n9);let eq27_e672_d_b0: f64 = (p.p14 * eq27_e671_d_b0);let eq27_e672_d_b1: f64 = (p.p14 * eq27_e671_d_b1);let eq27_e672_d_b2: f64 = (p.p14 * eq27_e671_d_b2);let eq27_e672_d_b3: f64 = (p.p14 * eq27_e671_d_b3);let eq27_e672_q: f64 = (p.p14 * eq27_e671_q);let eq27_reactive_node_derivatives: [f64; 10] = [eq27_e672_d_n0, eq27_e672_d_n1, eq27_e672_d_n2, eq27_e672_d_n3, eq27_e672_d_n4, eq27_e672_d_n5, eq27_e672_d_n6, eq27_e672_d_n7, eq27_e672_d_n8, eq27_e672_d_n9];let eq27_reactive_branch_derivatives: [f64; 4] = [eq27_e672_d_b0, eq27_e672_d_b1, eq27_e672_d_b2, eq27_e672_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(6),
            &eq27_reactive_node_derivatives,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let eq28_e675_q: f64 = s.v[359];let eq28_e677_q: f64 = s.v[371];let eq28_e678: f64 = (s.v[359] + s.v[371]);let eq28_e678_d_n0: f64 = (s.dn[359][0] + s.dn[371][0]);let eq28_e678_d_n1: f64 = (s.dn[359][1] + s.dn[371][1]);let eq28_e678_d_n2: f64 = (s.dn[359][2] + s.dn[371][2]);let eq28_e678_d_n3: f64 = (s.dn[359][3] + s.dn[371][3]);let eq28_e678_d_n4: f64 = (s.dn[359][4] + s.dn[371][4]);let eq28_e678_d_n5: f64 = (s.dn[359][5] + s.dn[371][5]);let eq28_e678_d_n6: f64 = (s.dn[359][6] + s.dn[371][6]);let eq28_e678_d_n7: f64 = (s.dn[359][7] + s.dn[371][7]);let eq28_e678_d_n8: f64 = (s.dn[359][8] + s.dn[371][8]);let eq28_e678_d_n9: f64 = (s.dn[359][9] + s.dn[371][9]);let eq28_e678_d_b0: f64 = (s.db[359][0] + s.db[371][0]);let eq28_e678_d_b1: f64 = (s.db[359][1] + s.db[371][1]);let eq28_e678_d_b2: f64 = (s.db[359][2] + s.db[371][2]);let eq28_e678_d_b3: f64 = (s.db[359][3] + s.db[371][3]);let eq28_e678_q: f64 = (eq28_e675_q + eq28_e677_q);let eq28_e679: f64 = (p.p14 * eq28_e678);let eq28_e679_d_n0: f64 = (p.p14 * eq28_e678_d_n0);let eq28_e679_d_n1: f64 = (p.p14 * eq28_e678_d_n1);let eq28_e679_d_n2: f64 = (p.p14 * eq28_e678_d_n2);let eq28_e679_d_n3: f64 = (p.p14 * eq28_e678_d_n3);let eq28_e679_d_n4: f64 = (p.p14 * eq28_e678_d_n4);let eq28_e679_d_n5: f64 = (p.p14 * eq28_e678_d_n5);let eq28_e679_d_n6: f64 = (p.p14 * eq28_e678_d_n6);let eq28_e679_d_n7: f64 = (p.p14 * eq28_e678_d_n7);let eq28_e679_d_n8: f64 = (p.p14 * eq28_e678_d_n8);let eq28_e679_d_n9: f64 = (p.p14 * eq28_e678_d_n9);let eq28_e679_d_b0: f64 = (p.p14 * eq28_e678_d_b0);let eq28_e679_d_b1: f64 = (p.p14 * eq28_e678_d_b1);let eq28_e679_d_b2: f64 = (p.p14 * eq28_e678_d_b2);let eq28_e679_d_b3: f64 = (p.p14 * eq28_e678_d_b3);let eq28_e679_q: f64 = (p.p14 * eq28_e678_q);let eq28_reactive_node_derivatives: [f64; 10] = [eq28_e679_d_n0, eq28_e679_d_n1, eq28_e679_d_n2, eq28_e679_d_n3, eq28_e679_d_n4, eq28_e679_d_n5, eq28_e679_d_n6, eq28_e679_d_n7, eq28_e679_d_n8, eq28_e679_d_n9];let eq28_reactive_branch_derivatives: [f64; 4] = [eq28_e679_d_b0, eq28_e679_d_b1, eq28_e679_d_b2, eq28_e679_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(6),
            &eq28_reactive_node_derivatives,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );let eq29_e681_q: f64 = s.v[374];
        stamper.stamp_current_reactive_dense_local(
            Some(4),
            None,
            &s.dn[374],
            &s.db[374],
            multiplicity,
        );let eq32_e690: f64 = (s.v[1790] * (nv5 - 0.0));let eq32_e690_d_n0: f64 = (s.dn[1790][0] * (nv5 - 0.0));let eq32_e690_d_n1: f64 = (s.dn[1790][1] * (nv5 - 0.0));let eq32_e690_d_n2: f64 = (s.dn[1790][2] * (nv5 - 0.0));let eq32_e690_d_n3: f64 = (s.dn[1790][3] * (nv5 - 0.0));let eq32_e690_d_n4: f64 = (s.dn[1790][4] * (nv5 - 0.0));let eq32_e690_d_n5: f64 = ((s.dn[1790][5] * (nv5 - 0.0)) + s.v[1790]);let eq32_e690_d_n6: f64 = (s.dn[1790][6] * (nv5 - 0.0));let eq32_e690_d_n7: f64 = (s.dn[1790][7] * (nv5 - 0.0));let eq32_e690_d_n8: f64 = (s.dn[1790][8] * (nv5 - 0.0));let eq32_e690_d_n9: f64 = (s.dn[1790][9] * (nv5 - 0.0));let eq32_e690_d_b0: f64 = (s.db[1790][0] * (nv5 - 0.0));let eq32_e690_d_b1: f64 = (s.db[1790][1] * (nv5 - 0.0));let eq32_e690_d_b2: f64 = (s.db[1790][2] * (nv5 - 0.0));let eq32_e690_d_b3: f64 = (s.db[1790][3] * (nv5 - 0.0));let eq32_e691_q: f64 = eq32_e690;let eq32_reactive_node_derivatives: [f64; 10] = [eq32_e690_d_n0, eq32_e690_d_n1, eq32_e690_d_n2, eq32_e690_d_n3, eq32_e690_d_n4, eq32_e690_d_n5, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9];let eq32_reactive_branch_derivatives: [f64; 4] = [eq32_e690_d_b0, eq32_e690_d_b1, eq32_e690_d_b2, eq32_e690_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            None,
            &eq32_reactive_node_derivatives,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );let eq33_e693: f64 = (-s.v[1791]);let eq33_e695: f64 = (eq33_e693 * (nv5 - 0.0));let eq33_e695_d_n0: f64 = ((-s.dn[1791][0]) * (nv5 - 0.0));let eq33_e695_d_n1: f64 = ((-s.dn[1791][1]) * (nv5 - 0.0));let eq33_e695_d_n2: f64 = ((-s.dn[1791][2]) * (nv5 - 0.0));let eq33_e695_d_n3: f64 = ((-s.dn[1791][3]) * (nv5 - 0.0));let eq33_e695_d_n4: f64 = ((-s.dn[1791][4]) * (nv5 - 0.0));let eq33_e695_d_n5: f64 = (((-s.dn[1791][5]) * (nv5 - 0.0)) + eq33_e693);let eq33_e695_d_n6: f64 = ((-s.dn[1791][6]) * (nv5 - 0.0));let eq33_e695_d_n7: f64 = ((-s.dn[1791][7]) * (nv5 - 0.0));let eq33_e695_d_n8: f64 = ((-s.dn[1791][8]) * (nv5 - 0.0));let eq33_e695_d_n9: f64 = ((-s.dn[1791][9]) * (nv5 - 0.0));let eq33_e695_d_b0: f64 = ((-s.db[1791][0]) * (nv5 - 0.0));let eq33_e695_d_b1: f64 = ((-s.db[1791][1]) * (nv5 - 0.0));let eq33_e695_d_b2: f64 = ((-s.db[1791][2]) * (nv5 - 0.0));let eq33_e695_d_b3: f64 = ((-s.db[1791][3]) * (nv5 - 0.0));let eq33_e696_q: f64 = eq33_e695;let eq33_reactive_node_derivatives: [f64; 10] = [eq33_e695_d_n0, eq33_e695_d_n1, eq33_e695_d_n2, eq33_e695_d_n3, eq33_e695_d_n4, eq33_e695_d_n5, eq33_e695_d_n6, eq33_e695_d_n7, eq33_e695_d_n8, eq33_e695_d_n9];let eq33_reactive_branch_derivatives: [f64; 4] = [eq33_e695_d_b0, eq33_e695_d_b1, eq33_e695_d_b2, eq33_e695_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(6),
            &eq33_reactive_node_derivatives,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let eq34_e698: f64 = (-s.v[1792]);let eq34_e700: f64 = (eq34_e698 * (nv5 - 0.0));let eq34_e700_d_n0: f64 = ((-s.dn[1792][0]) * (nv5 - 0.0));let eq34_e700_d_n1: f64 = ((-s.dn[1792][1]) * (nv5 - 0.0));let eq34_e700_d_n2: f64 = ((-s.dn[1792][2]) * (nv5 - 0.0));let eq34_e700_d_n3: f64 = ((-s.dn[1792][3]) * (nv5 - 0.0));let eq34_e700_d_n4: f64 = ((-s.dn[1792][4]) * (nv5 - 0.0));let eq34_e700_d_n5: f64 = (((-s.dn[1792][5]) * (nv5 - 0.0)) + eq34_e698);let eq34_e700_d_n6: f64 = ((-s.dn[1792][6]) * (nv5 - 0.0));let eq34_e700_d_n7: f64 = ((-s.dn[1792][7]) * (nv5 - 0.0));let eq34_e700_d_n8: f64 = ((-s.dn[1792][8]) * (nv5 - 0.0));let eq34_e700_d_n9: f64 = ((-s.dn[1792][9]) * (nv5 - 0.0));let eq34_e700_d_b0: f64 = ((-s.db[1792][0]) * (nv5 - 0.0));let eq34_e700_d_b1: f64 = ((-s.db[1792][1]) * (nv5 - 0.0));let eq34_e700_d_b2: f64 = ((-s.db[1792][2]) * (nv5 - 0.0));let eq34_e700_d_b3: f64 = ((-s.db[1792][3]) * (nv5 - 0.0));let eq34_e701_q: f64 = eq34_e700;let eq34_reactive_node_derivatives: [f64; 10] = [eq34_e700_d_n0, eq34_e700_d_n1, eq34_e700_d_n2, eq34_e700_d_n3, eq34_e700_d_n4, eq34_e700_d_n5, eq34_e700_d_n6, eq34_e700_d_n7, eq34_e700_d_n8, eq34_e700_d_n9];let eq34_reactive_branch_derivatives: [f64; 4] = [eq34_e700_d_b0, eq34_e700_d_b1, eq34_e700_d_b2, eq34_e700_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(7),
            &eq34_reactive_node_derivatives,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
