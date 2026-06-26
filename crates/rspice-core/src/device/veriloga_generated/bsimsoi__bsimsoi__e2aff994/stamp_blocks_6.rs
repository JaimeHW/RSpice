#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq54_e2010, eq54_e2010_d_n0, eq54_e2010_d_n1, eq54_e2010_d_n2, eq54_e2010_d_n3, eq54_e2010_d_n4, eq54_e2010_d_n5, eq54_e2010_d_n6, eq54_e2010_d_n7, eq54_e2010_d_n8, eq54_e2010_d_n9, eq54_e2010_d_n10, eq54_e2010_d_n11, eq54_e2010_d_n12, eq54_e2010_d_n13, eq54_e2010_d_b0, eq54_e2010_d_b1, eq54_e2010_d_b2, eq54_e2010_d_b3, eq54_e2010_d_b4, eq54_e2010_d_b5, eq54_e2010_d_b6, eq54_e2010_d_b7, eq54_e2010_d_b8, eq54_e2010_d_b9, eq54_e2010_d_b10, eq54_e2010_d_b11,) = {
    if (!s.b[2011]) {
        (s.v[1096], s.dn[1096][0], s.dn[1096][1], s.dn[1096][2], s.dn[1096][3], s.dn[1096][4], s.dn[1096][5], s.dn[1096][6], s.dn[1096][7], s.dn[1096][8], s.dn[1096][9], s.dn[1096][10], s.dn[1096][11], s.dn[1096][12], s.dn[1096][13], s.db[1096][0], s.db[1096][1], s.db[1096][2], s.db[1096][3], s.db[1096][4], s.db[1096][5], s.db[1096][6], s.db[1096][7], s.db[1096][8], s.db[1096][9], s.db[1096][10], s.db[1096][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2010;
        let eq54_node_derivatives: [f64; 14] = [eq54_e2010_d_n0, eq54_e2010_d_n1, eq54_e2010_d_n2, eq54_e2010_d_n3, eq54_e2010_d_n4, eq54_e2010_d_n5, eq54_e2010_d_n6, eq54_e2010_d_n7, eq54_e2010_d_n8, eq54_e2010_d_n9, eq54_e2010_d_n10, eq54_e2010_d_n11, eq54_e2010_d_n12, eq54_e2010_d_n13];
        let eq54_branch_derivatives: [f64; 12] = [eq54_e2010_d_b0, eq54_e2010_d_b1, eq54_e2010_d_b2, eq54_e2010_d_b3, eq54_e2010_d_b4, eq54_e2010_d_b5, eq54_e2010_d_b6, eq54_e2010_d_b7, eq54_e2010_d_b8, eq54_e2010_d_b9, eq54_e2010_d_b10, eq54_e2010_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(10),
            multiplicity * (eq54_value),
            &eq54_node_derivatives,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e2017, eq55_e2017_d_n0, eq55_e2017_d_n1, eq55_e2017_d_n2, eq55_e2017_d_n3, eq55_e2017_d_n4, eq55_e2017_d_n5, eq55_e2017_d_n6, eq55_e2017_d_n7, eq55_e2017_d_n8, eq55_e2017_d_n9, eq55_e2017_d_n10, eq55_e2017_d_n11, eq55_e2017_d_n12, eq55_e2017_d_n13, eq55_e2017_d_b0, eq55_e2017_d_b1, eq55_e2017_d_b2, eq55_e2017_d_b3, eq55_e2017_d_b4, eq55_e2017_d_b5, eq55_e2017_d_b6, eq55_e2017_d_b7, eq55_e2017_d_b8, eq55_e2017_d_b9, eq55_e2017_d_b10, eq55_e2017_d_b11,) = {
    if (!s.b[2011]) {
        let eq55_e2015: f64 = (s.v[1095] + s.v[1097]);
        let eq55_e2015_d_n0: f64 = (s.dn[1095][0] + s.dn[1097][0]);
        let eq55_e2015_d_n1: f64 = (s.dn[1095][1] + s.dn[1097][1]);
        let eq55_e2015_d_n2: f64 = (s.dn[1095][2] + s.dn[1097][2]);
        let eq55_e2015_d_n3: f64 = (s.dn[1095][3] + s.dn[1097][3]);
        let eq55_e2015_d_n4: f64 = (s.dn[1095][4] + s.dn[1097][4]);
        let eq55_e2015_d_n5: f64 = (s.dn[1095][5] + s.dn[1097][5]);
        let eq55_e2015_d_n6: f64 = (s.dn[1095][6] + s.dn[1097][6]);
        let eq55_e2015_d_n7: f64 = (s.dn[1095][7] + s.dn[1097][7]);
        let eq55_e2015_d_n8: f64 = (s.dn[1095][8] + s.dn[1097][8]);
        let eq55_e2015_d_n9: f64 = (s.dn[1095][9] + s.dn[1097][9]);
        let eq55_e2015_d_n10: f64 = (s.dn[1095][10] + s.dn[1097][10]);
        let eq55_e2015_d_n11: f64 = (s.dn[1095][11] + s.dn[1097][11]);
        let eq55_e2015_d_n12: f64 = (s.dn[1095][12] + s.dn[1097][12]);
        let eq55_e2015_d_n13: f64 = (s.dn[1095][13] + s.dn[1097][13]);
        let eq55_e2015_d_b0: f64 = (s.db[1095][0] + s.db[1097][0]);
        let eq55_e2015_d_b1: f64 = (s.db[1095][1] + s.db[1097][1]);
        let eq55_e2015_d_b2: f64 = (s.db[1095][2] + s.db[1097][2]);
        let eq55_e2015_d_b3: f64 = (s.db[1095][3] + s.db[1097][3]);
        let eq55_e2015_d_b4: f64 = (s.db[1095][4] + s.db[1097][4]);
        let eq55_e2015_d_b5: f64 = (s.db[1095][5] + s.db[1097][5]);
        let eq55_e2015_d_b6: f64 = (s.db[1095][6] + s.db[1097][6]);
        let eq55_e2015_d_b7: f64 = (s.db[1095][7] + s.db[1097][7]);
        let eq55_e2015_d_b8: f64 = (s.db[1095][8] + s.db[1097][8]);
        let eq55_e2015_d_b9: f64 = (s.db[1095][9] + s.db[1097][9]);
        let eq55_e2015_d_b10: f64 = (s.db[1095][10] + s.db[1097][10]);
        let eq55_e2015_d_b11: f64 = (s.db[1095][11] + s.db[1097][11]);
        (eq55_e2015, eq55_e2015_d_n0, eq55_e2015_d_n1, eq55_e2015_d_n2, eq55_e2015_d_n3, eq55_e2015_d_n4, eq55_e2015_d_n5, eq55_e2015_d_n6, eq55_e2015_d_n7, eq55_e2015_d_n8, eq55_e2015_d_n9, eq55_e2015_d_n10, eq55_e2015_d_n11, eq55_e2015_d_n12, eq55_e2015_d_n13, eq55_e2015_d_b0, eq55_e2015_d_b1, eq55_e2015_d_b2, eq55_e2015_d_b3, eq55_e2015_d_b4, eq55_e2015_d_b5, eq55_e2015_d_b6, eq55_e2015_d_b7, eq55_e2015_d_b8, eq55_e2015_d_b9, eq55_e2015_d_b10, eq55_e2015_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2017;
        let eq55_node_derivatives: [f64; 14] = [eq55_e2017_d_n0, eq55_e2017_d_n1, eq55_e2017_d_n2, eq55_e2017_d_n3, eq55_e2017_d_n4, eq55_e2017_d_n5, eq55_e2017_d_n6, eq55_e2017_d_n7, eq55_e2017_d_n8, eq55_e2017_d_n9, eq55_e2017_d_n10, eq55_e2017_d_n11, eq55_e2017_d_n12, eq55_e2017_d_n13];
        let eq55_branch_derivatives: [f64; 12] = [eq55_e2017_d_b0, eq55_e2017_d_b1, eq55_e2017_d_b2, eq55_e2017_d_b3, eq55_e2017_d_b4, eq55_e2017_d_b5, eq55_e2017_d_b6, eq55_e2017_d_b7, eq55_e2017_d_b8, eq55_e2017_d_b9, eq55_e2017_d_b10, eq55_e2017_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e2021,) = {
    if s.b[2012] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e2021;
        stamper.stamp_potential_const_local(
            2,
            eq56_value,
        );
        let (eq57_e2028, eq57_e2028_d_n0, eq57_e2028_d_n1, eq57_e2028_d_n2, eq57_e2028_d_n3, eq57_e2028_d_n4, eq57_e2028_d_n5, eq57_e2028_d_n6, eq57_e2028_d_n7, eq57_e2028_d_n8, eq57_e2028_d_n9, eq57_e2028_d_n10, eq57_e2028_d_n11, eq57_e2028_d_n12, eq57_e2028_d_n13, eq57_e2028_d_b0, eq57_e2028_d_b1, eq57_e2028_d_b2, eq57_e2028_d_b3, eq57_e2028_d_b4, eq57_e2028_d_b5, eq57_e2028_d_b6, eq57_e2028_d_b7, eq57_e2028_d_b8, eq57_e2028_d_b9, eq57_e2028_d_b10, eq57_e2028_d_b11,) = {
    if (!s.b[2012]) {
        let eq57_e2026: f64 = ((nv1 - nv9) * s.v[2013]);
        let eq57_e2026_d_n0: f64 = ((nv1 - nv9) * s.dn[2013][0]);
        let eq57_e2026_d_n1: f64 = (s.v[2013] + ((nv1 - nv9) * s.dn[2013][1]));
        let eq57_e2026_d_n2: f64 = ((nv1 - nv9) * s.dn[2013][2]);
        let eq57_e2026_d_n3: f64 = ((nv1 - nv9) * s.dn[2013][3]);
        let eq57_e2026_d_n4: f64 = ((nv1 - nv9) * s.dn[2013][4]);
        let eq57_e2026_d_n5: f64 = ((nv1 - nv9) * s.dn[2013][5]);
        let eq57_e2026_d_n6: f64 = ((nv1 - nv9) * s.dn[2013][6]);
        let eq57_e2026_d_n7: f64 = ((nv1 - nv9) * s.dn[2013][7]);
        let eq57_e2026_d_n8: f64 = ((nv1 - nv9) * s.dn[2013][8]);
        let eq57_e2026_d_n9: f64 = ((-s.v[2013]) + ((nv1 - nv9) * s.dn[2013][9]));
        let eq57_e2026_d_n10: f64 = ((nv1 - nv9) * s.dn[2013][10]);
        let eq57_e2026_d_n11: f64 = ((nv1 - nv9) * s.dn[2013][11]);
        let eq57_e2026_d_n12: f64 = ((nv1 - nv9) * s.dn[2013][12]);
        let eq57_e2026_d_n13: f64 = ((nv1 - nv9) * s.dn[2013][13]);
        let eq57_e2026_d_b0: f64 = ((nv1 - nv9) * s.db[2013][0]);
        let eq57_e2026_d_b1: f64 = ((nv1 - nv9) * s.db[2013][1]);
        let eq57_e2026_d_b2: f64 = ((nv1 - nv9) * s.db[2013][2]);
        let eq57_e2026_d_b3: f64 = ((nv1 - nv9) * s.db[2013][3]);
        let eq57_e2026_d_b4: f64 = ((nv1 - nv9) * s.db[2013][4]);
        let eq57_e2026_d_b5: f64 = ((nv1 - nv9) * s.db[2013][5]);
        let eq57_e2026_d_b6: f64 = ((nv1 - nv9) * s.db[2013][6]);
        let eq57_e2026_d_b7: f64 = ((nv1 - nv9) * s.db[2013][7]);
        let eq57_e2026_d_b8: f64 = ((nv1 - nv9) * s.db[2013][8]);
        let eq57_e2026_d_b9: f64 = ((nv1 - nv9) * s.db[2013][9]);
        let eq57_e2026_d_b10: f64 = ((nv1 - nv9) * s.db[2013][10]);
        let eq57_e2026_d_b11: f64 = ((nv1 - nv9) * s.db[2013][11]);
        (eq57_e2026, eq57_e2026_d_n0, eq57_e2026_d_n1, eq57_e2026_d_n2, eq57_e2026_d_n3, eq57_e2026_d_n4, eq57_e2026_d_n5, eq57_e2026_d_n6, eq57_e2026_d_n7, eq57_e2026_d_n8, eq57_e2026_d_n9, eq57_e2026_d_n10, eq57_e2026_d_n11, eq57_e2026_d_n12, eq57_e2026_d_n13, eq57_e2026_d_b0, eq57_e2026_d_b1, eq57_e2026_d_b2, eq57_e2026_d_b3, eq57_e2026_d_b4, eq57_e2026_d_b5, eq57_e2026_d_b6, eq57_e2026_d_b7, eq57_e2026_d_b8, eq57_e2026_d_b9, eq57_e2026_d_b10, eq57_e2026_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2028;
        let eq57_node_derivatives: [f64; 14] = [eq57_e2028_d_n0, eq57_e2028_d_n1, eq57_e2028_d_n2, eq57_e2028_d_n3, eq57_e2028_d_n4, eq57_e2028_d_n5, eq57_e2028_d_n6, eq57_e2028_d_n7, eq57_e2028_d_n8, eq57_e2028_d_n9, eq57_e2028_d_n10, eq57_e2028_d_n11, eq57_e2028_d_n12, eq57_e2028_d_n13];
        let eq57_branch_derivatives: [f64; 12] = [eq57_e2028_d_b0, eq57_e2028_d_b1, eq57_e2028_d_b2, eq57_e2028_d_b3, eq57_e2028_d_b4, eq57_e2028_d_b5, eq57_e2028_d_b6, eq57_e2028_d_b7, eq57_e2028_d_b8, eq57_e2028_d_b9, eq57_e2028_d_b10, eq57_e2028_d_b11];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e2037,) = {
    if (!s.b[2012]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e2037;
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (eq58_value),
        );
        let (eq59_e2043, eq59_e2043_d_n0, eq59_e2043_d_n1, eq59_e2043_d_n2, eq59_e2043_d_n3, eq59_e2043_d_n4, eq59_e2043_d_n5, eq59_e2043_d_n6, eq59_e2043_d_n7, eq59_e2043_d_n8, eq59_e2043_d_n9, eq59_e2043_d_n10, eq59_e2043_d_n11, eq59_e2043_d_n12, eq59_e2043_d_n13, eq59_e2043_d_b0, eq59_e2043_d_b1, eq59_e2043_d_b2, eq59_e2043_d_b3, eq59_e2043_d_b4, eq59_e2043_d_b5, eq59_e2043_d_b6, eq59_e2043_d_b7, eq59_e2043_d_b8, eq59_e2043_d_b9, eq59_e2043_d_b10, eq59_e2043_d_b11,) = {
    if s.b[2016] {
        let eq59_e2041: f64 = ((nv0 - nv6) * s.v[618]);
        let eq59_e2041_d_n0: f64 = (s.v[618] + ((nv0 - nv6) * s.dn[618][0]));
        let eq59_e2041_d_n1: f64 = ((nv0 - nv6) * s.dn[618][1]);
        let eq59_e2041_d_n2: f64 = ((nv0 - nv6) * s.dn[618][2]);
        let eq59_e2041_d_n3: f64 = ((nv0 - nv6) * s.dn[618][3]);
        let eq59_e2041_d_n4: f64 = ((nv0 - nv6) * s.dn[618][4]);
        let eq59_e2041_d_n5: f64 = ((nv0 - nv6) * s.dn[618][5]);
        let eq59_e2041_d_n6: f64 = ((-s.v[618]) + ((nv0 - nv6) * s.dn[618][6]));
        let eq59_e2041_d_n7: f64 = ((nv0 - nv6) * s.dn[618][7]);
        let eq59_e2041_d_n8: f64 = ((nv0 - nv6) * s.dn[618][8]);
        let eq59_e2041_d_n9: f64 = ((nv0 - nv6) * s.dn[618][9]);
        let eq59_e2041_d_n10: f64 = ((nv0 - nv6) * s.dn[618][10]);
        let eq59_e2041_d_n11: f64 = ((nv0 - nv6) * s.dn[618][11]);
        let eq59_e2041_d_n12: f64 = ((nv0 - nv6) * s.dn[618][12]);
        let eq59_e2041_d_n13: f64 = ((nv0 - nv6) * s.dn[618][13]);
        let eq59_e2041_d_b0: f64 = ((nv0 - nv6) * s.db[618][0]);
        let eq59_e2041_d_b1: f64 = ((nv0 - nv6) * s.db[618][1]);
        let eq59_e2041_d_b2: f64 = ((nv0 - nv6) * s.db[618][2]);
        let eq59_e2041_d_b3: f64 = ((nv0 - nv6) * s.db[618][3]);
        let eq59_e2041_d_b4: f64 = ((nv0 - nv6) * s.db[618][4]);
        let eq59_e2041_d_b5: f64 = ((nv0 - nv6) * s.db[618][5]);
        let eq59_e2041_d_b6: f64 = ((nv0 - nv6) * s.db[618][6]);
        let eq59_e2041_d_b7: f64 = ((nv0 - nv6) * s.db[618][7]);
        let eq59_e2041_d_b8: f64 = ((nv0 - nv6) * s.db[618][8]);
        let eq59_e2041_d_b9: f64 = ((nv0 - nv6) * s.db[618][9]);
        let eq59_e2041_d_b10: f64 = ((nv0 - nv6) * s.db[618][10]);
        let eq59_e2041_d_b11: f64 = ((nv0 - nv6) * s.db[618][11]);
        (eq59_e2041, eq59_e2041_d_n0, eq59_e2041_d_n1, eq59_e2041_d_n2, eq59_e2041_d_n3, eq59_e2041_d_n4, eq59_e2041_d_n5, eq59_e2041_d_n6, eq59_e2041_d_n7, eq59_e2041_d_n8, eq59_e2041_d_n9, eq59_e2041_d_n10, eq59_e2041_d_n11, eq59_e2041_d_n12, eq59_e2041_d_n13, eq59_e2041_d_b0, eq59_e2041_d_b1, eq59_e2041_d_b2, eq59_e2041_d_b3, eq59_e2041_d_b4, eq59_e2041_d_b5, eq59_e2041_d_b6, eq59_e2041_d_b7, eq59_e2041_d_b8, eq59_e2041_d_b9, eq59_e2041_d_b10, eq59_e2041_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e2043;
        let eq59_node_derivatives: [f64; 14] = [eq59_e2043_d_n0, eq59_e2043_d_n1, eq59_e2043_d_n2, eq59_e2043_d_n3, eq59_e2043_d_n4, eq59_e2043_d_n5, eq59_e2043_d_n6, eq59_e2043_d_n7, eq59_e2043_d_n8, eq59_e2043_d_n9, eq59_e2043_d_n10, eq59_e2043_d_n11, eq59_e2043_d_n12, eq59_e2043_d_n13];
        let eq59_branch_derivatives: [f64; 12] = [eq59_e2043_d_b0, eq59_e2043_d_b1, eq59_e2043_d_b2, eq59_e2043_d_b3, eq59_e2043_d_b4, eq59_e2043_d_b5, eq59_e2043_d_b6, eq59_e2043_d_b7, eq59_e2043_d_b8, eq59_e2043_d_b9, eq59_e2043_d_b10, eq59_e2043_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e2048,) = {
    if (!s.b[2016]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2048;
        stamper.stamp_potential_const_local(
            3,
            eq60_value,
        );
        let (eq61_e2056,) = {
    if s.b[2017] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e2056;
        stamper.stamp_current_const_local(
            Some(0),
            Some(6),
            multiplicity * (eq61_value),
        );
        let (eq62_e2062, eq62_e2062_d_n0, eq62_e2062_d_n1, eq62_e2062_d_n2, eq62_e2062_d_n3, eq62_e2062_d_n4, eq62_e2062_d_n5, eq62_e2062_d_n6, eq62_e2062_d_n7, eq62_e2062_d_n8, eq62_e2062_d_n9, eq62_e2062_d_n10, eq62_e2062_d_n11, eq62_e2062_d_n12, eq62_e2062_d_n13, eq62_e2062_d_b0, eq62_e2062_d_b1, eq62_e2062_d_b2, eq62_e2062_d_b3, eq62_e2062_d_b4, eq62_e2062_d_b5, eq62_e2062_d_b6, eq62_e2062_d_b7, eq62_e2062_d_b8, eq62_e2062_d_b9, eq62_e2062_d_b10, eq62_e2062_d_b11,) = {
    if s.b[2018] {
        let eq62_e2060: f64 = ((nv2 - nv7) * s.v[617]);
        let eq62_e2060_d_n0: f64 = ((nv2 - nv7) * s.dn[617][0]);
        let eq62_e2060_d_n1: f64 = ((nv2 - nv7) * s.dn[617][1]);
        let eq62_e2060_d_n2: f64 = (s.v[617] + ((nv2 - nv7) * s.dn[617][2]));
        let eq62_e2060_d_n3: f64 = ((nv2 - nv7) * s.dn[617][3]);
        let eq62_e2060_d_n4: f64 = ((nv2 - nv7) * s.dn[617][4]);
        let eq62_e2060_d_n5: f64 = ((nv2 - nv7) * s.dn[617][5]);
        let eq62_e2060_d_n6: f64 = ((nv2 - nv7) * s.dn[617][6]);
        let eq62_e2060_d_n7: f64 = ((-s.v[617]) + ((nv2 - nv7) * s.dn[617][7]));
        let eq62_e2060_d_n8: f64 = ((nv2 - nv7) * s.dn[617][8]);
        let eq62_e2060_d_n9: f64 = ((nv2 - nv7) * s.dn[617][9]);
        let eq62_e2060_d_n10: f64 = ((nv2 - nv7) * s.dn[617][10]);
        let eq62_e2060_d_n11: f64 = ((nv2 - nv7) * s.dn[617][11]);
        let eq62_e2060_d_n12: f64 = ((nv2 - nv7) * s.dn[617][12]);
        let eq62_e2060_d_n13: f64 = ((nv2 - nv7) * s.dn[617][13]);
        let eq62_e2060_d_b0: f64 = ((nv2 - nv7) * s.db[617][0]);
        let eq62_e2060_d_b1: f64 = ((nv2 - nv7) * s.db[617][1]);
        let eq62_e2060_d_b2: f64 = ((nv2 - nv7) * s.db[617][2]);
        let eq62_e2060_d_b3: f64 = ((nv2 - nv7) * s.db[617][3]);
        let eq62_e2060_d_b4: f64 = ((nv2 - nv7) * s.db[617][4]);
        let eq62_e2060_d_b5: f64 = ((nv2 - nv7) * s.db[617][5]);
        let eq62_e2060_d_b6: f64 = ((nv2 - nv7) * s.db[617][6]);
        let eq62_e2060_d_b7: f64 = ((nv2 - nv7) * s.db[617][7]);
        let eq62_e2060_d_b8: f64 = ((nv2 - nv7) * s.db[617][8]);
        let eq62_e2060_d_b9: f64 = ((nv2 - nv7) * s.db[617][9]);
        let eq62_e2060_d_b10: f64 = ((nv2 - nv7) * s.db[617][10]);
        let eq62_e2060_d_b11: f64 = ((nv2 - nv7) * s.db[617][11]);
        (eq62_e2060, eq62_e2060_d_n0, eq62_e2060_d_n1, eq62_e2060_d_n2, eq62_e2060_d_n3, eq62_e2060_d_n4, eq62_e2060_d_n5, eq62_e2060_d_n6, eq62_e2060_d_n7, eq62_e2060_d_n8, eq62_e2060_d_n9, eq62_e2060_d_n10, eq62_e2060_d_n11, eq62_e2060_d_n12, eq62_e2060_d_n13, eq62_e2060_d_b0, eq62_e2060_d_b1, eq62_e2060_d_b2, eq62_e2060_d_b3, eq62_e2060_d_b4, eq62_e2060_d_b5, eq62_e2060_d_b6, eq62_e2060_d_b7, eq62_e2060_d_b8, eq62_e2060_d_b9, eq62_e2060_d_b10, eq62_e2060_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2062;
        let eq62_node_derivatives: [f64; 14] = [eq62_e2062_d_n0, eq62_e2062_d_n1, eq62_e2062_d_n2, eq62_e2062_d_n3, eq62_e2062_d_n4, eq62_e2062_d_n5, eq62_e2062_d_n6, eq62_e2062_d_n7, eq62_e2062_d_n8, eq62_e2062_d_n9, eq62_e2062_d_n10, eq62_e2062_d_n11, eq62_e2062_d_n12, eq62_e2062_d_n13];
        let eq62_branch_derivatives: [f64; 12] = [eq62_e2062_d_b0, eq62_e2062_d_b1, eq62_e2062_d_b2, eq62_e2062_d_b3, eq62_e2062_d_b4, eq62_e2062_d_b5, eq62_e2062_d_b6, eq62_e2062_d_b7, eq62_e2062_d_b8, eq62_e2062_d_b9, eq62_e2062_d_b10, eq62_e2062_d_b11];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e2067,) = {
    if (!s.b[2018]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e2067;
        stamper.stamp_potential_const_local(
            4,
            eq63_value,
        );
        let (eq64_e2075,) = {
    if s.b[2019] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e2075;
        stamper.stamp_current_const_local(
            Some(2),
            Some(7),
            multiplicity * (eq64_value),
        );
        let (eq65_e2081, eq65_e2081_d_n0, eq65_e2081_d_n1, eq65_e2081_d_n2, eq65_e2081_d_n3, eq65_e2081_d_n4, eq65_e2081_d_n5, eq65_e2081_d_n6, eq65_e2081_d_n7, eq65_e2081_d_n8, eq65_e2081_d_n9, eq65_e2081_d_n10, eq65_e2081_d_n11, eq65_e2081_d_n12, eq65_e2081_d_n13, eq65_e2081_d_b0, eq65_e2081_d_b1, eq65_e2081_d_b2, eq65_e2081_d_b3, eq65_e2081_d_b4, eq65_e2081_d_b5, eq65_e2081_d_b6, eq65_e2081_d_b7, eq65_e2081_d_b8, eq65_e2081_d_b9, eq65_e2081_d_b10, eq65_e2081_d_b11,) = {
    if s.b[2020] {
        let eq65_e2079: f64 = ((nv9 - nv8) * s.v[467]);
        let eq65_e2079_d_n0: f64 = ((nv9 - nv8) * s.dn[467][0]);
        let eq65_e2079_d_n1: f64 = ((nv9 - nv8) * s.dn[467][1]);
        let eq65_e2079_d_n2: f64 = ((nv9 - nv8) * s.dn[467][2]);
        let eq65_e2079_d_n3: f64 = ((nv9 - nv8) * s.dn[467][3]);
        let eq65_e2079_d_n4: f64 = ((nv9 - nv8) * s.dn[467][4]);
        let eq65_e2079_d_n5: f64 = ((nv9 - nv8) * s.dn[467][5]);
        let eq65_e2079_d_n6: f64 = ((nv9 - nv8) * s.dn[467][6]);
        let eq65_e2079_d_n7: f64 = ((nv9 - nv8) * s.dn[467][7]);
        let eq65_e2079_d_n8: f64 = ((-s.v[467]) + ((nv9 - nv8) * s.dn[467][8]));
        let eq65_e2079_d_n9: f64 = (s.v[467] + ((nv9 - nv8) * s.dn[467][9]));
        let eq65_e2079_d_n10: f64 = ((nv9 - nv8) * s.dn[467][10]);
        let eq65_e2079_d_n11: f64 = ((nv9 - nv8) * s.dn[467][11]);
        let eq65_e2079_d_n12: f64 = ((nv9 - nv8) * s.dn[467][12]);
        let eq65_e2079_d_n13: f64 = ((nv9 - nv8) * s.dn[467][13]);
        let eq65_e2079_d_b0: f64 = ((nv9 - nv8) * s.db[467][0]);
        let eq65_e2079_d_b1: f64 = ((nv9 - nv8) * s.db[467][1]);
        let eq65_e2079_d_b2: f64 = ((nv9 - nv8) * s.db[467][2]);
        let eq65_e2079_d_b3: f64 = ((nv9 - nv8) * s.db[467][3]);
        let eq65_e2079_d_b4: f64 = ((nv9 - nv8) * s.db[467][4]);
        let eq65_e2079_d_b5: f64 = ((nv9 - nv8) * s.db[467][5]);
        let eq65_e2079_d_b6: f64 = ((nv9 - nv8) * s.db[467][6]);
        let eq65_e2079_d_b7: f64 = ((nv9 - nv8) * s.db[467][7]);
        let eq65_e2079_d_b8: f64 = ((nv9 - nv8) * s.db[467][8]);
        let eq65_e2079_d_b9: f64 = ((nv9 - nv8) * s.db[467][9]);
        let eq65_e2079_d_b10: f64 = ((nv9 - nv8) * s.db[467][10]);
        let eq65_e2079_d_b11: f64 = ((nv9 - nv8) * s.db[467][11]);
        (eq65_e2079, eq65_e2079_d_n0, eq65_e2079_d_n1, eq65_e2079_d_n2, eq65_e2079_d_n3, eq65_e2079_d_n4, eq65_e2079_d_n5, eq65_e2079_d_n6, eq65_e2079_d_n7, eq65_e2079_d_n8, eq65_e2079_d_n9, eq65_e2079_d_n10, eq65_e2079_d_n11, eq65_e2079_d_n12, eq65_e2079_d_n13, eq65_e2079_d_b0, eq65_e2079_d_b1, eq65_e2079_d_b2, eq65_e2079_d_b3, eq65_e2079_d_b4, eq65_e2079_d_b5, eq65_e2079_d_b6, eq65_e2079_d_b7, eq65_e2079_d_b8, eq65_e2079_d_b9, eq65_e2079_d_b10, eq65_e2079_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e2081;
        let eq65_node_derivatives: [f64; 14] = [eq65_e2081_d_n0, eq65_e2081_d_n1, eq65_e2081_d_n2, eq65_e2081_d_n3, eq65_e2081_d_n4, eq65_e2081_d_n5, eq65_e2081_d_n6, eq65_e2081_d_n7, eq65_e2081_d_n8, eq65_e2081_d_n9, eq65_e2081_d_n10, eq65_e2081_d_n11, eq65_e2081_d_n12, eq65_e2081_d_n13];
        let eq65_branch_derivatives: [f64; 12] = [eq65_e2081_d_b0, eq65_e2081_d_b1, eq65_e2081_d_b2, eq65_e2081_d_b3, eq65_e2081_d_b4, eq65_e2081_d_b5, eq65_e2081_d_b6, eq65_e2081_d_b7, eq65_e2081_d_b8, eq65_e2081_d_b9, eq65_e2081_d_b10, eq65_e2081_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let (eq66_e2086,) = {
    if (!s.b[2020]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e2086;
        stamper.stamp_potential_const_local(
            5,
            eq66_value,
        );
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n1, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11, eq67_e2103_d_n12, eq67_e2103_d_n13, eq67_e2103_d_b0, eq67_e2103_d_b1, eq67_e2103_d_b2, eq67_e2103_d_b3, eq67_e2103_d_b4, eq67_e2103_d_b5, eq67_e2103_d_b6, eq67_e2103_d_b7, eq67_e2103_d_b8, eq67_e2103_d_b9, eq67_e2103_d_b10, eq67_e2103_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && s.b[2025]) {
        let eq67_e2094: f64 = (s.v[634] * s.v[1015]);
        let eq67_e2094_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq67_e2094_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq67_e2094_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq67_e2094_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq67_e2094_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq67_e2094_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq67_e2094_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq67_e2094_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq67_e2094_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq67_e2094_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq67_e2094_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq67_e2094_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq67_e2094_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq67_e2094_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq67_e2094_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));
        let eq67_e2094_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));
        let eq67_e2094_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));
        let eq67_e2094_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));
        let eq67_e2094_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));
        let eq67_e2094_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));
        let eq67_e2094_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));
        let eq67_e2094_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));
        let eq67_e2094_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));
        let eq67_e2094_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));
        let eq67_e2094_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));
        let eq67_e2094_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));
        let eq67_e2097: f64 = (s.v[634] * s.v[1016]);
        let eq67_e2097_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq67_e2097_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq67_e2097_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq67_e2097_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq67_e2097_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq67_e2097_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq67_e2097_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq67_e2097_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq67_e2097_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq67_e2097_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq67_e2097_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq67_e2097_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq67_e2097_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq67_e2097_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq67_e2097_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));
        let eq67_e2097_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));
        let eq67_e2097_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));
        let eq67_e2097_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));
        let eq67_e2097_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));
        let eq67_e2097_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq67_e2097_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));
        let eq67_e2097_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));
        let eq67_e2097_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));
        let eq67_e2097_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));
        let eq67_e2097_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));
        let eq67_e2097_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));
        let eq67_e2098: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, eq67_e2097);
        let eq67_e2098_d_n0: f64 = (eq67_e2097_d_n0 * ddt_scale);
        let eq67_e2098_d_n1: f64 = (eq67_e2097_d_n1 * ddt_scale);
        let eq67_e2098_d_n2: f64 = (eq67_e2097_d_n2 * ddt_scale);
        let eq67_e2098_d_n3: f64 = (eq67_e2097_d_n3 * ddt_scale);
        let eq67_e2098_d_n4: f64 = (eq67_e2097_d_n4 * ddt_scale);
        let eq67_e2098_d_n5: f64 = (eq67_e2097_d_n5 * ddt_scale);
        let eq67_e2098_d_n6: f64 = (eq67_e2097_d_n6 * ddt_scale);
        let eq67_e2098_d_n7: f64 = (eq67_e2097_d_n7 * ddt_scale);
        let eq67_e2098_d_n8: f64 = (eq67_e2097_d_n8 * ddt_scale);
        let eq67_e2098_d_n9: f64 = (eq67_e2097_d_n9 * ddt_scale);
        let eq67_e2098_d_n10: f64 = (eq67_e2097_d_n10 * ddt_scale);
        let eq67_e2098_d_n11: f64 = (eq67_e2097_d_n11 * ddt_scale);
        let eq67_e2098_d_n12: f64 = (eq67_e2097_d_n12 * ddt_scale);
        let eq67_e2098_d_n13: f64 = (eq67_e2097_d_n13 * ddt_scale);
        let eq67_e2098_d_b0: f64 = (eq67_e2097_d_b0 * ddt_scale);
        let eq67_e2098_d_b1: f64 = (eq67_e2097_d_b1 * ddt_scale);
        let eq67_e2098_d_b2: f64 = (eq67_e2097_d_b2 * ddt_scale);
        let eq67_e2098_d_b3: f64 = (eq67_e2097_d_b3 * ddt_scale);
        let eq67_e2098_d_b4: f64 = (eq67_e2097_d_b4 * ddt_scale);
        let eq67_e2098_d_b5: f64 = (eq67_e2097_d_b5 * ddt_scale);
        let eq67_e2098_d_b6: f64 = (eq67_e2097_d_b6 * ddt_scale);
        let eq67_e2098_d_b7: f64 = (eq67_e2097_d_b7 * ddt_scale);
        let eq67_e2098_d_b8: f64 = (eq67_e2097_d_b8 * ddt_scale);
        let eq67_e2098_d_b9: f64 = (eq67_e2097_d_b9 * ddt_scale);
        let eq67_e2098_d_b10: f64 = (eq67_e2097_d_b10 * ddt_scale);
        let eq67_e2098_d_b11: f64 = (eq67_e2097_d_b11 * ddt_scale);
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2098);
        let eq67_e2099_d_n0: f64 = (eq67_e2094_d_n0 + eq67_e2098_d_n0);
        let eq67_e2099_d_n1: f64 = (eq67_e2094_d_n1 + eq67_e2098_d_n1);
        let eq67_e2099_d_n2: f64 = (eq67_e2094_d_n2 + eq67_e2098_d_n2);
        let eq67_e2099_d_n3: f64 = (eq67_e2094_d_n3 + eq67_e2098_d_n3);
        let eq67_e2099_d_n4: f64 = (eq67_e2094_d_n4 + eq67_e2098_d_n4);
        let eq67_e2099_d_n5: f64 = (eq67_e2094_d_n5 + eq67_e2098_d_n5);
        let eq67_e2099_d_n6: f64 = (eq67_e2094_d_n6 + eq67_e2098_d_n6);
        let eq67_e2099_d_n7: f64 = (eq67_e2094_d_n7 + eq67_e2098_d_n7);
        let eq67_e2099_d_n8: f64 = (eq67_e2094_d_n8 + eq67_e2098_d_n8);
        let eq67_e2099_d_n9: f64 = (eq67_e2094_d_n9 + eq67_e2098_d_n9);
        let eq67_e2099_d_n10: f64 = (eq67_e2094_d_n10 + eq67_e2098_d_n10);
        let eq67_e2099_d_n11: f64 = (eq67_e2094_d_n11 + eq67_e2098_d_n11);
        let eq67_e2099_d_n12: f64 = (eq67_e2094_d_n12 + eq67_e2098_d_n12);
        let eq67_e2099_d_n13: f64 = (eq67_e2094_d_n13 + eq67_e2098_d_n13);
        let eq67_e2099_d_b0: f64 = (eq67_e2094_d_b0 + eq67_e2098_d_b0);
        let eq67_e2099_d_b1: f64 = (eq67_e2094_d_b1 + eq67_e2098_d_b1);
        let eq67_e2099_d_b2: f64 = (eq67_e2094_d_b2 + eq67_e2098_d_b2);
        let eq67_e2099_d_b3: f64 = (eq67_e2094_d_b3 + eq67_e2098_d_b3);
        let eq67_e2099_d_b4: f64 = (eq67_e2094_d_b4 + eq67_e2098_d_b4);
        let eq67_e2099_d_b5: f64 = (eq67_e2094_d_b5 + eq67_e2098_d_b5);
        let eq67_e2099_d_b6: f64 = (eq67_e2094_d_b6 + eq67_e2098_d_b6);
        let eq67_e2099_d_b7: f64 = (eq67_e2094_d_b7 + eq67_e2098_d_b7);
        let eq67_e2099_d_b8: f64 = (eq67_e2094_d_b8 + eq67_e2098_d_b8);
        let eq67_e2099_d_b9: f64 = (eq67_e2094_d_b9 + eq67_e2098_d_b9);
        let eq67_e2099_d_b10: f64 = (eq67_e2094_d_b10 + eq67_e2098_d_b10);
        let eq67_e2099_d_b11: f64 = (eq67_e2094_d_b11 + eq67_e2098_d_b11);
        let eq67_e2101: f64 = (eq67_e2099 - s.v[1017]);
        let eq67_e2101_d_n0: f64 = (eq67_e2099_d_n0 - s.dn[1017][0]);
        let eq67_e2101_d_n1: f64 = (eq67_e2099_d_n1 - s.dn[1017][1]);
        let eq67_e2101_d_n2: f64 = (eq67_e2099_d_n2 - s.dn[1017][2]);
        let eq67_e2101_d_n3: f64 = (eq67_e2099_d_n3 - s.dn[1017][3]);
        let eq67_e2101_d_n4: f64 = (eq67_e2099_d_n4 - s.dn[1017][4]);
        let eq67_e2101_d_n5: f64 = (eq67_e2099_d_n5 - s.dn[1017][5]);
        let eq67_e2101_d_n6: f64 = (eq67_e2099_d_n6 - s.dn[1017][6]);
        let eq67_e2101_d_n7: f64 = (eq67_e2099_d_n7 - s.dn[1017][7]);
        let eq67_e2101_d_n8: f64 = (eq67_e2099_d_n8 - s.dn[1017][8]);
        let eq67_e2101_d_n9: f64 = (eq67_e2099_d_n9 - s.dn[1017][9]);
        let eq67_e2101_d_n10: f64 = (eq67_e2099_d_n10 - s.dn[1017][10]);
        let eq67_e2101_d_n11: f64 = (eq67_e2099_d_n11 - s.dn[1017][11]);
        let eq67_e2101_d_n12: f64 = (eq67_e2099_d_n12 - s.dn[1017][12]);
        let eq67_e2101_d_n13: f64 = (eq67_e2099_d_n13 - s.dn[1017][13]);
        let eq67_e2101_d_b0: f64 = (eq67_e2099_d_b0 - s.db[1017][0]);
        let eq67_e2101_d_b1: f64 = (eq67_e2099_d_b1 - s.db[1017][1]);
        let eq67_e2101_d_b2: f64 = (eq67_e2099_d_b2 - s.db[1017][2]);
        let eq67_e2101_d_b3: f64 = (eq67_e2099_d_b3 - s.db[1017][3]);
        let eq67_e2101_d_b4: f64 = (eq67_e2099_d_b4 - s.db[1017][4]);
        let eq67_e2101_d_b5: f64 = (eq67_e2099_d_b5 - s.db[1017][5]);
        let eq67_e2101_d_b6: f64 = (eq67_e2099_d_b6 - s.db[1017][6]);
        let eq67_e2101_d_b7: f64 = (eq67_e2099_d_b7 - s.db[1017][7]);
        let eq67_e2101_d_b8: f64 = (eq67_e2099_d_b8 - s.db[1017][8]);
        let eq67_e2101_d_b9: f64 = (eq67_e2099_d_b9 - s.db[1017][9]);
        let eq67_e2101_d_b10: f64 = (eq67_e2099_d_b10 - s.db[1017][10]);
        let eq67_e2101_d_b11: f64 = (eq67_e2099_d_b11 - s.db[1017][11]);
        (eq67_e2101, eq67_e2101_d_n0, eq67_e2101_d_n1, eq67_e2101_d_n2, eq67_e2101_d_n3, eq67_e2101_d_n4, eq67_e2101_d_n5, eq67_e2101_d_n6, eq67_e2101_d_n7, eq67_e2101_d_n8, eq67_e2101_d_n9, eq67_e2101_d_n10, eq67_e2101_d_n11, eq67_e2101_d_n12, eq67_e2101_d_n13, eq67_e2101_d_b0, eq67_e2101_d_b1, eq67_e2101_d_b2, eq67_e2101_d_b3, eq67_e2101_d_b4, eq67_e2101_d_b5, eq67_e2101_d_b6, eq67_e2101_d_b7, eq67_e2101_d_b8, eq67_e2101_d_b9, eq67_e2101_d_b10, eq67_e2101_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2103;
        let eq67_node_derivatives: [f64; 14] = [eq67_e2103_d_n0, eq67_e2103_d_n1, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11, eq67_e2103_d_n12, eq67_e2103_d_n13];
        let eq67_branch_derivatives: [f64; 12] = [eq67_e2103_d_b0, eq67_e2103_d_b1, eq67_e2103_d_b2, eq67_e2103_d_b3, eq67_e2103_d_b4, eq67_e2103_d_b5, eq67_e2103_d_b6, eq67_e2103_d_b7, eq67_e2103_d_b8, eq67_e2103_d_b9, eq67_e2103_d_b10, eq67_e2103_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq67_value),
            &eq67_node_derivatives,
            &eq67_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n1, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11, eq68_e2121_d_n12, eq68_e2121_d_n13, eq68_e2121_d_b0, eq68_e2121_d_b1, eq68_e2121_d_b2, eq68_e2121_d_b3, eq68_e2121_d_b4, eq68_e2121_d_b5, eq68_e2121_d_b6, eq68_e2121_d_b7, eq68_e2121_d_b8, eq68_e2121_d_b9, eq68_e2121_d_b10, eq68_e2121_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && (!s.b[2025])) {
        let eq68_e2112: f64 = (s.v[634] * s.v[1015]);
        let eq68_e2112_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq68_e2112_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq68_e2112_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq68_e2112_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq68_e2112_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq68_e2112_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq68_e2112_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq68_e2112_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq68_e2112_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq68_e2112_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq68_e2112_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq68_e2112_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq68_e2112_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq68_e2112_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq68_e2112_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));
        let eq68_e2112_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));
        let eq68_e2112_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));
        let eq68_e2112_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));
        let eq68_e2112_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));
        let eq68_e2112_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));
        let eq68_e2112_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));
        let eq68_e2112_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));
        let eq68_e2112_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));
        let eq68_e2112_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));
        let eq68_e2112_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));
        let eq68_e2112_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));
        let eq68_e2115: f64 = (s.v[634] * s.v[1016]);
        let eq68_e2115_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq68_e2115_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq68_e2115_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq68_e2115_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq68_e2115_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq68_e2115_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq68_e2115_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq68_e2115_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq68_e2115_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq68_e2115_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq68_e2115_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq68_e2115_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq68_e2115_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq68_e2115_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq68_e2115_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));
        let eq68_e2115_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));
        let eq68_e2115_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));
        let eq68_e2115_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));
        let eq68_e2115_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));
        let eq68_e2115_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq68_e2115_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));
        let eq68_e2115_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));
        let eq68_e2115_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));
        let eq68_e2115_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));
        let eq68_e2115_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));
        let eq68_e2115_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));
        let eq68_e2116: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, eq68_e2115);
        let eq68_e2116_d_n0: f64 = (eq68_e2115_d_n0 * ddt_scale);
        let eq68_e2116_d_n1: f64 = (eq68_e2115_d_n1 * ddt_scale);
        let eq68_e2116_d_n2: f64 = (eq68_e2115_d_n2 * ddt_scale);
        let eq68_e2116_d_n3: f64 = (eq68_e2115_d_n3 * ddt_scale);
        let eq68_e2116_d_n4: f64 = (eq68_e2115_d_n4 * ddt_scale);
        let eq68_e2116_d_n5: f64 = (eq68_e2115_d_n5 * ddt_scale);
        let eq68_e2116_d_n6: f64 = (eq68_e2115_d_n6 * ddt_scale);
        let eq68_e2116_d_n7: f64 = (eq68_e2115_d_n7 * ddt_scale);
        let eq68_e2116_d_n8: f64 = (eq68_e2115_d_n8 * ddt_scale);
        let eq68_e2116_d_n9: f64 = (eq68_e2115_d_n9 * ddt_scale);
        let eq68_e2116_d_n10: f64 = (eq68_e2115_d_n10 * ddt_scale);
        let eq68_e2116_d_n11: f64 = (eq68_e2115_d_n11 * ddt_scale);
        let eq68_e2116_d_n12: f64 = (eq68_e2115_d_n12 * ddt_scale);
        let eq68_e2116_d_n13: f64 = (eq68_e2115_d_n13 * ddt_scale);
        let eq68_e2116_d_b0: f64 = (eq68_e2115_d_b0 * ddt_scale);
        let eq68_e2116_d_b1: f64 = (eq68_e2115_d_b1 * ddt_scale);
        let eq68_e2116_d_b2: f64 = (eq68_e2115_d_b2 * ddt_scale);
        let eq68_e2116_d_b3: f64 = (eq68_e2115_d_b3 * ddt_scale);
        let eq68_e2116_d_b4: f64 = (eq68_e2115_d_b4 * ddt_scale);
        let eq68_e2116_d_b5: f64 = (eq68_e2115_d_b5 * ddt_scale);
        let eq68_e2116_d_b6: f64 = (eq68_e2115_d_b6 * ddt_scale);
        let eq68_e2116_d_b7: f64 = (eq68_e2115_d_b7 * ddt_scale);
        let eq68_e2116_d_b8: f64 = (eq68_e2115_d_b8 * ddt_scale);
        let eq68_e2116_d_b9: f64 = (eq68_e2115_d_b9 * ddt_scale);
        let eq68_e2116_d_b10: f64 = (eq68_e2115_d_b10 * ddt_scale);
        let eq68_e2116_d_b11: f64 = (eq68_e2115_d_b11 * ddt_scale);
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2116);
        let eq68_e2117_d_n0: f64 = (eq68_e2112_d_n0 + eq68_e2116_d_n0);
        let eq68_e2117_d_n1: f64 = (eq68_e2112_d_n1 + eq68_e2116_d_n1);
        let eq68_e2117_d_n2: f64 = (eq68_e2112_d_n2 + eq68_e2116_d_n2);
        let eq68_e2117_d_n3: f64 = (eq68_e2112_d_n3 + eq68_e2116_d_n3);
        let eq68_e2117_d_n4: f64 = (eq68_e2112_d_n4 + eq68_e2116_d_n4);
        let eq68_e2117_d_n5: f64 = (eq68_e2112_d_n5 + eq68_e2116_d_n5);
        let eq68_e2117_d_n6: f64 = (eq68_e2112_d_n6 + eq68_e2116_d_n6);
        let eq68_e2117_d_n7: f64 = (eq68_e2112_d_n7 + eq68_e2116_d_n7);
        let eq68_e2117_d_n8: f64 = (eq68_e2112_d_n8 + eq68_e2116_d_n8);
        let eq68_e2117_d_n9: f64 = (eq68_e2112_d_n9 + eq68_e2116_d_n9);
        let eq68_e2117_d_n10: f64 = (eq68_e2112_d_n10 + eq68_e2116_d_n10);
        let eq68_e2117_d_n11: f64 = (eq68_e2112_d_n11 + eq68_e2116_d_n11);
        let eq68_e2117_d_n12: f64 = (eq68_e2112_d_n12 + eq68_e2116_d_n12);
        let eq68_e2117_d_n13: f64 = (eq68_e2112_d_n13 + eq68_e2116_d_n13);
        let eq68_e2117_d_b0: f64 = (eq68_e2112_d_b0 + eq68_e2116_d_b0);
        let eq68_e2117_d_b1: f64 = (eq68_e2112_d_b1 + eq68_e2116_d_b1);
        let eq68_e2117_d_b2: f64 = (eq68_e2112_d_b2 + eq68_e2116_d_b2);
        let eq68_e2117_d_b3: f64 = (eq68_e2112_d_b3 + eq68_e2116_d_b3);
        let eq68_e2117_d_b4: f64 = (eq68_e2112_d_b4 + eq68_e2116_d_b4);
        let eq68_e2117_d_b5: f64 = (eq68_e2112_d_b5 + eq68_e2116_d_b5);
        let eq68_e2117_d_b6: f64 = (eq68_e2112_d_b6 + eq68_e2116_d_b6);
        let eq68_e2117_d_b7: f64 = (eq68_e2112_d_b7 + eq68_e2116_d_b7);
        let eq68_e2117_d_b8: f64 = (eq68_e2112_d_b8 + eq68_e2116_d_b8);
        let eq68_e2117_d_b9: f64 = (eq68_e2112_d_b9 + eq68_e2116_d_b9);
        let eq68_e2117_d_b10: f64 = (eq68_e2112_d_b10 + eq68_e2116_d_b10);
        let eq68_e2117_d_b11: f64 = (eq68_e2112_d_b11 + eq68_e2116_d_b11);
        let eq68_e2119: f64 = (eq68_e2117 - s.v[1017]);
        let eq68_e2119_d_n0: f64 = (eq68_e2117_d_n0 - s.dn[1017][0]);
        let eq68_e2119_d_n1: f64 = (eq68_e2117_d_n1 - s.dn[1017][1]);
        let eq68_e2119_d_n2: f64 = (eq68_e2117_d_n2 - s.dn[1017][2]);
        let eq68_e2119_d_n3: f64 = (eq68_e2117_d_n3 - s.dn[1017][3]);
        let eq68_e2119_d_n4: f64 = (eq68_e2117_d_n4 - s.dn[1017][4]);
        let eq68_e2119_d_n5: f64 = (eq68_e2117_d_n5 - s.dn[1017][5]);
        let eq68_e2119_d_n6: f64 = (eq68_e2117_d_n6 - s.dn[1017][6]);
        let eq68_e2119_d_n7: f64 = (eq68_e2117_d_n7 - s.dn[1017][7]);
        let eq68_e2119_d_n8: f64 = (eq68_e2117_d_n8 - s.dn[1017][8]);
        let eq68_e2119_d_n9: f64 = (eq68_e2117_d_n9 - s.dn[1017][9]);
        let eq68_e2119_d_n10: f64 = (eq68_e2117_d_n10 - s.dn[1017][10]);
        let eq68_e2119_d_n11: f64 = (eq68_e2117_d_n11 - s.dn[1017][11]);
        let eq68_e2119_d_n12: f64 = (eq68_e2117_d_n12 - s.dn[1017][12]);
        let eq68_e2119_d_n13: f64 = (eq68_e2117_d_n13 - s.dn[1017][13]);
        let eq68_e2119_d_b0: f64 = (eq68_e2117_d_b0 - s.db[1017][0]);
        let eq68_e2119_d_b1: f64 = (eq68_e2117_d_b1 - s.db[1017][1]);
        let eq68_e2119_d_b2: f64 = (eq68_e2117_d_b2 - s.db[1017][2]);
        let eq68_e2119_d_b3: f64 = (eq68_e2117_d_b3 - s.db[1017][3]);
        let eq68_e2119_d_b4: f64 = (eq68_e2117_d_b4 - s.db[1017][4]);
        let eq68_e2119_d_b5: f64 = (eq68_e2117_d_b5 - s.db[1017][5]);
        let eq68_e2119_d_b6: f64 = (eq68_e2117_d_b6 - s.db[1017][6]);
        let eq68_e2119_d_b7: f64 = (eq68_e2117_d_b7 - s.db[1017][7]);
        let eq68_e2119_d_b8: f64 = (eq68_e2117_d_b8 - s.db[1017][8]);
        let eq68_e2119_d_b9: f64 = (eq68_e2117_d_b9 - s.db[1017][9]);
        let eq68_e2119_d_b10: f64 = (eq68_e2117_d_b10 - s.db[1017][10]);
        let eq68_e2119_d_b11: f64 = (eq68_e2117_d_b11 - s.db[1017][11]);
        (eq68_e2119, eq68_e2119_d_n0, eq68_e2119_d_n1, eq68_e2119_d_n2, eq68_e2119_d_n3, eq68_e2119_d_n4, eq68_e2119_d_n5, eq68_e2119_d_n6, eq68_e2119_d_n7, eq68_e2119_d_n8, eq68_e2119_d_n9, eq68_e2119_d_n10, eq68_e2119_d_n11, eq68_e2119_d_n12, eq68_e2119_d_n13, eq68_e2119_d_b0, eq68_e2119_d_b1, eq68_e2119_d_b2, eq68_e2119_d_b3, eq68_e2119_d_b4, eq68_e2119_d_b5, eq68_e2119_d_b6, eq68_e2119_d_b7, eq68_e2119_d_b8, eq68_e2119_d_b9, eq68_e2119_d_b10, eq68_e2119_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e2121;
        let eq68_node_derivatives: [f64; 14] = [eq68_e2121_d_n0, eq68_e2121_d_n1, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11, eq68_e2121_d_n12, eq68_e2121_d_n13];
        let eq68_branch_derivatives: [f64; 12] = [eq68_e2121_d_b0, eq68_e2121_d_b1, eq68_e2121_d_b2, eq68_e2121_d_b3, eq68_e2121_d_b4, eq68_e2121_d_b5, eq68_e2121_d_b6, eq68_e2121_d_b7, eq68_e2121_d_b8, eq68_e2121_d_b9, eq68_e2121_d_b10, eq68_e2121_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq68_value),
            &eq68_node_derivatives,
            &eq68_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n1, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11, eq69_e2137_d_n12, eq69_e2137_d_n13, eq69_e2137_d_b0, eq69_e2137_d_b1, eq69_e2137_d_b2, eq69_e2137_d_b3, eq69_e2137_d_b4, eq69_e2137_d_b5, eq69_e2137_d_b6, eq69_e2137_d_b7, eq69_e2137_d_b8, eq69_e2137_d_b9, eq69_e2137_d_b10, eq69_e2137_d_b11,) = {
    if (s.b[2021] && (!s.b[2024])) {
        let eq69_e2128: f64 = (s.v[634] * s.v[1015]);
        let eq69_e2128_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq69_e2128_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq69_e2128_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq69_e2128_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq69_e2128_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq69_e2128_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq69_e2128_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq69_e2128_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq69_e2128_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq69_e2128_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq69_e2128_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq69_e2128_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq69_e2128_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq69_e2128_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq69_e2128_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));
        let eq69_e2128_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));
        let eq69_e2128_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));
        let eq69_e2128_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));
        let eq69_e2128_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));
        let eq69_e2128_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));
        let eq69_e2128_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));
        let eq69_e2128_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));
        let eq69_e2128_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));
        let eq69_e2128_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));
        let eq69_e2128_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));
        let eq69_e2128_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));
        let eq69_e2131: f64 = (s.v[634] * s.v[1016]);
        let eq69_e2131_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq69_e2131_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq69_e2131_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq69_e2131_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq69_e2131_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq69_e2131_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq69_e2131_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq69_e2131_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq69_e2131_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq69_e2131_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq69_e2131_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq69_e2131_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq69_e2131_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq69_e2131_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq69_e2131_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));
        let eq69_e2131_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));
        let eq69_e2131_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));
        let eq69_e2131_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));
        let eq69_e2131_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));
        let eq69_e2131_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq69_e2131_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));
        let eq69_e2131_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));
        let eq69_e2131_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));
        let eq69_e2131_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));
        let eq69_e2131_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));
        let eq69_e2131_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));
        let eq69_e2132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, eq69_e2131);
        let eq69_e2132_d_n0: f64 = (eq69_e2131_d_n0 * ddt_scale);
        let eq69_e2132_d_n1: f64 = (eq69_e2131_d_n1 * ddt_scale);
        let eq69_e2132_d_n2: f64 = (eq69_e2131_d_n2 * ddt_scale);
        let eq69_e2132_d_n3: f64 = (eq69_e2131_d_n3 * ddt_scale);
        let eq69_e2132_d_n4: f64 = (eq69_e2131_d_n4 * ddt_scale);
        let eq69_e2132_d_n5: f64 = (eq69_e2131_d_n5 * ddt_scale);
        let eq69_e2132_d_n6: f64 = (eq69_e2131_d_n6 * ddt_scale);
        let eq69_e2132_d_n7: f64 = (eq69_e2131_d_n7 * ddt_scale);
        let eq69_e2132_d_n8: f64 = (eq69_e2131_d_n8 * ddt_scale);
        let eq69_e2132_d_n9: f64 = (eq69_e2131_d_n9 * ddt_scale);
        let eq69_e2132_d_n10: f64 = (eq69_e2131_d_n10 * ddt_scale);
        let eq69_e2132_d_n11: f64 = (eq69_e2131_d_n11 * ddt_scale);
        let eq69_e2132_d_n12: f64 = (eq69_e2131_d_n12 * ddt_scale);
        let eq69_e2132_d_n13: f64 = (eq69_e2131_d_n13 * ddt_scale);
        let eq69_e2132_d_b0: f64 = (eq69_e2131_d_b0 * ddt_scale);
        let eq69_e2132_d_b1: f64 = (eq69_e2131_d_b1 * ddt_scale);
        let eq69_e2132_d_b2: f64 = (eq69_e2131_d_b2 * ddt_scale);
        let eq69_e2132_d_b3: f64 = (eq69_e2131_d_b3 * ddt_scale);
        let eq69_e2132_d_b4: f64 = (eq69_e2131_d_b4 * ddt_scale);
        let eq69_e2132_d_b5: f64 = (eq69_e2131_d_b5 * ddt_scale);
        let eq69_e2132_d_b6: f64 = (eq69_e2131_d_b6 * ddt_scale);
        let eq69_e2132_d_b7: f64 = (eq69_e2131_d_b7 * ddt_scale);
        let eq69_e2132_d_b8: f64 = (eq69_e2131_d_b8 * ddt_scale);
        let eq69_e2132_d_b9: f64 = (eq69_e2131_d_b9 * ddt_scale);
        let eq69_e2132_d_b10: f64 = (eq69_e2131_d_b10 * ddt_scale);
        let eq69_e2132_d_b11: f64 = (eq69_e2131_d_b11 * ddt_scale);
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2132);
        let eq69_e2133_d_n0: f64 = (eq69_e2128_d_n0 + eq69_e2132_d_n0);
        let eq69_e2133_d_n1: f64 = (eq69_e2128_d_n1 + eq69_e2132_d_n1);
        let eq69_e2133_d_n2: f64 = (eq69_e2128_d_n2 + eq69_e2132_d_n2);
        let eq69_e2133_d_n3: f64 = (eq69_e2128_d_n3 + eq69_e2132_d_n3);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + eq69_e2132_d_n4);
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + eq69_e2132_d_n5);
        let eq69_e2133_d_n6: f64 = (eq69_e2128_d_n6 + eq69_e2132_d_n6);
        let eq69_e2133_d_n7: f64 = (eq69_e2128_d_n7 + eq69_e2132_d_n7);
        let eq69_e2133_d_n8: f64 = (eq69_e2128_d_n8 + eq69_e2132_d_n8);
        let eq69_e2133_d_n9: f64 = (eq69_e2128_d_n9 + eq69_e2132_d_n9);
        let eq69_e2133_d_n10: f64 = (eq69_e2128_d_n10 + eq69_e2132_d_n10);
        let eq69_e2133_d_n11: f64 = (eq69_e2128_d_n11 + eq69_e2132_d_n11);
        let eq69_e2133_d_n12: f64 = (eq69_e2128_d_n12 + eq69_e2132_d_n12);
        let eq69_e2133_d_n13: f64 = (eq69_e2128_d_n13 + eq69_e2132_d_n13);
        let eq69_e2133_d_b0: f64 = (eq69_e2128_d_b0 + eq69_e2132_d_b0);
        let eq69_e2133_d_b1: f64 = (eq69_e2128_d_b1 + eq69_e2132_d_b1);
        let eq69_e2133_d_b2: f64 = (eq69_e2128_d_b2 + eq69_e2132_d_b2);
        let eq69_e2133_d_b3: f64 = (eq69_e2128_d_b3 + eq69_e2132_d_b3);
        let eq69_e2133_d_b4: f64 = (eq69_e2128_d_b4 + eq69_e2132_d_b4);
        let eq69_e2133_d_b5: f64 = (eq69_e2128_d_b5 + eq69_e2132_d_b5);
        let eq69_e2133_d_b6: f64 = (eq69_e2128_d_b6 + eq69_e2132_d_b6);
        let eq69_e2133_d_b7: f64 = (eq69_e2128_d_b7 + eq69_e2132_d_b7);
        let eq69_e2133_d_b8: f64 = (eq69_e2128_d_b8 + eq69_e2132_d_b8);
        let eq69_e2133_d_b9: f64 = (eq69_e2128_d_b9 + eq69_e2132_d_b9);
        let eq69_e2133_d_b10: f64 = (eq69_e2128_d_b10 + eq69_e2132_d_b10);
        let eq69_e2133_d_b11: f64 = (eq69_e2128_d_b11 + eq69_e2132_d_b11);
        let eq69_e2135: f64 = (eq69_e2133 - s.v[1017]);
        let eq69_e2135_d_n0: f64 = (eq69_e2133_d_n0 - s.dn[1017][0]);
        let eq69_e2135_d_n1: f64 = (eq69_e2133_d_n1 - s.dn[1017][1]);
        let eq69_e2135_d_n2: f64 = (eq69_e2133_d_n2 - s.dn[1017][2]);
        let eq69_e2135_d_n3: f64 = (eq69_e2133_d_n3 - s.dn[1017][3]);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - s.dn[1017][4]);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - s.dn[1017][5]);
        let eq69_e2135_d_n6: f64 = (eq69_e2133_d_n6 - s.dn[1017][6]);
        let eq69_e2135_d_n7: f64 = (eq69_e2133_d_n7 - s.dn[1017][7]);
        let eq69_e2135_d_n8: f64 = (eq69_e2133_d_n8 - s.dn[1017][8]);
        let eq69_e2135_d_n9: f64 = (eq69_e2133_d_n9 - s.dn[1017][9]);
        let eq69_e2135_d_n10: f64 = (eq69_e2133_d_n10 - s.dn[1017][10]);
        let eq69_e2135_d_n11: f64 = (eq69_e2133_d_n11 - s.dn[1017][11]);
        let eq69_e2135_d_n12: f64 = (eq69_e2133_d_n12 - s.dn[1017][12]);
        let eq69_e2135_d_n13: f64 = (eq69_e2133_d_n13 - s.dn[1017][13]);
        let eq69_e2135_d_b0: f64 = (eq69_e2133_d_b0 - s.db[1017][0]);
        let eq69_e2135_d_b1: f64 = (eq69_e2133_d_b1 - s.db[1017][1]);
        let eq69_e2135_d_b2: f64 = (eq69_e2133_d_b2 - s.db[1017][2]);
        let eq69_e2135_d_b3: f64 = (eq69_e2133_d_b3 - s.db[1017][3]);
        let eq69_e2135_d_b4: f64 = (eq69_e2133_d_b4 - s.db[1017][4]);
        let eq69_e2135_d_b5: f64 = (eq69_e2133_d_b5 - s.db[1017][5]);
        let eq69_e2135_d_b6: f64 = (eq69_e2133_d_b6 - s.db[1017][6]);
        let eq69_e2135_d_b7: f64 = (eq69_e2133_d_b7 - s.db[1017][7]);
        let eq69_e2135_d_b8: f64 = (eq69_e2133_d_b8 - s.db[1017][8]);
        let eq69_e2135_d_b9: f64 = (eq69_e2133_d_b9 - s.db[1017][9]);
        let eq69_e2135_d_b10: f64 = (eq69_e2133_d_b10 - s.db[1017][10]);
        let eq69_e2135_d_b11: f64 = (eq69_e2133_d_b11 - s.db[1017][11]);
        (eq69_e2135, eq69_e2135_d_n0, eq69_e2135_d_n1, eq69_e2135_d_n2, eq69_e2135_d_n3, eq69_e2135_d_n4, eq69_e2135_d_n5, eq69_e2135_d_n6, eq69_e2135_d_n7, eq69_e2135_d_n8, eq69_e2135_d_n9, eq69_e2135_d_n10, eq69_e2135_d_n11, eq69_e2135_d_n12, eq69_e2135_d_n13, eq69_e2135_d_b0, eq69_e2135_d_b1, eq69_e2135_d_b2, eq69_e2135_d_b3, eq69_e2135_d_b4, eq69_e2135_d_b5, eq69_e2135_d_b6, eq69_e2135_d_b7, eq69_e2135_d_b8, eq69_e2135_d_b9, eq69_e2135_d_b10, eq69_e2135_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2137;
        let eq69_node_derivatives: [f64; 14] = [eq69_e2137_d_n0, eq69_e2137_d_n1, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11, eq69_e2137_d_n12, eq69_e2137_d_n13];
        let eq69_branch_derivatives: [f64; 12] = [eq69_e2137_d_b0, eq69_e2137_d_b1, eq69_e2137_d_b2, eq69_e2137_d_b3, eq69_e2137_d_b4, eq69_e2137_d_b5, eq69_e2137_d_b6, eq69_e2137_d_b7, eq69_e2137_d_b8, eq69_e2137_d_b9, eq69_e2137_d_b10, eq69_e2137_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq69_value),
            &eq69_node_derivatives,
            &eq69_branch_derivatives,
            multiplicity,
        );
        let (eq70_e2146,) = {
    if (((!s.b[2021]) && s.b[2026]) && s.b[2027]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e2146;
        stamper.stamp_potential_const_local(
            6,
            eq70_value,
        );
        let (eq71_e2156,) = {
    if (((!s.b[2021]) && s.b[2026]) && (!s.b[2027])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e2156;
        stamper.stamp_potential_const_local(
            7,
            eq71_value,
        );
        let (eq72_e2164,) = {
    if ((!s.b[2021]) && (!s.b[2026])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq72_value: f64 = eq72_e2164;
        stamper.stamp_potential_const_local(
            8,
            eq72_value,
        );
        let (eq73_e2168,) = {
    if s.b[2028] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e2168;
        stamper.stamp_potential_const_local(
            9,
            eq73_value,
        );
        let (eq74_e2172,) = {
    if s.b[2028] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e2172;
        stamper.stamp_potential_const_local(
            10,
            eq74_value,
        );
        let (eq75_e2179,) = {
    if ((!s.b[2028]) && s.b[2029]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e2179;
        stamper.stamp_potential_const_local(
            11,
            eq75_value,
        );
        let (eq76_e2187, eq76_e2187_d_n0, eq76_e2187_d_n1, eq76_e2187_d_n2, eq76_e2187_d_n3, eq76_e2187_d_n4, eq76_e2187_d_n5, eq76_e2187_d_n6, eq76_e2187_d_n7, eq76_e2187_d_n8, eq76_e2187_d_n9, eq76_e2187_d_n10, eq76_e2187_d_n11, eq76_e2187_d_n12, eq76_e2187_d_n13, eq76_e2187_d_b0, eq76_e2187_d_b1, eq76_e2187_d_b2, eq76_e2187_d_b3, eq76_e2187_d_b4, eq76_e2187_d_b5, eq76_e2187_d_b6, eq76_e2187_d_b7, eq76_e2187_d_b8, eq76_e2187_d_b9, eq76_e2187_d_b10, eq76_e2187_d_b11,) = {
    if (s.b[2037] && s.b[2038]) {
        let eq76_e2185: f64 = ((nv4 - nv10) * s.v[1021]);
        let eq76_e2185_d_n0: f64 = ((nv4 - nv10) * s.dn[1021][0]);
        let eq76_e2185_d_n1: f64 = ((nv4 - nv10) * s.dn[1021][1]);
        let eq76_e2185_d_n2: f64 = ((nv4 - nv10) * s.dn[1021][2]);
        let eq76_e2185_d_n3: f64 = ((nv4 - nv10) * s.dn[1021][3]);
        let eq76_e2185_d_n4: f64 = (s.v[1021] + ((nv4 - nv10) * s.dn[1021][4]));
        let eq76_e2185_d_n5: f64 = ((nv4 - nv10) * s.dn[1021][5]);
        let eq76_e2185_d_n6: f64 = ((nv4 - nv10) * s.dn[1021][6]);
        let eq76_e2185_d_n7: f64 = ((nv4 - nv10) * s.dn[1021][7]);
        let eq76_e2185_d_n8: f64 = ((nv4 - nv10) * s.dn[1021][8]);
        let eq76_e2185_d_n9: f64 = ((nv4 - nv10) * s.dn[1021][9]);
        let eq76_e2185_d_n10: f64 = ((-s.v[1021]) + ((nv4 - nv10) * s.dn[1021][10]));
        let eq76_e2185_d_n11: f64 = ((nv4 - nv10) * s.dn[1021][11]);
        let eq76_e2185_d_n12: f64 = ((nv4 - nv10) * s.dn[1021][12]);
        let eq76_e2185_d_n13: f64 = ((nv4 - nv10) * s.dn[1021][13]);
        let eq76_e2185_d_b0: f64 = ((nv4 - nv10) * s.db[1021][0]);
        let eq76_e2185_d_b1: f64 = ((nv4 - nv10) * s.db[1021][1]);
        let eq76_e2185_d_b2: f64 = ((nv4 - nv10) * s.db[1021][2]);
        let eq76_e2185_d_b3: f64 = ((nv4 - nv10) * s.db[1021][3]);
        let eq76_e2185_d_b4: f64 = ((nv4 - nv10) * s.db[1021][4]);
        let eq76_e2185_d_b5: f64 = ((nv4 - nv10) * s.db[1021][5]);
        let eq76_e2185_d_b6: f64 = ((nv4 - nv10) * s.db[1021][6]);
        let eq76_e2185_d_b7: f64 = ((nv4 - nv10) * s.db[1021][7]);
        let eq76_e2185_d_b8: f64 = ((nv4 - nv10) * s.db[1021][8]);
        let eq76_e2185_d_b9: f64 = ((nv4 - nv10) * s.db[1021][9]);
        let eq76_e2185_d_b10: f64 = ((nv4 - nv10) * s.db[1021][10]);
        let eq76_e2185_d_b11: f64 = ((nv4 - nv10) * s.db[1021][11]);
        (eq76_e2185, eq76_e2185_d_n0, eq76_e2185_d_n1, eq76_e2185_d_n2, eq76_e2185_d_n3, eq76_e2185_d_n4, eq76_e2185_d_n5, eq76_e2185_d_n6, eq76_e2185_d_n7, eq76_e2185_d_n8, eq76_e2185_d_n9, eq76_e2185_d_n10, eq76_e2185_d_n11, eq76_e2185_d_n12, eq76_e2185_d_n13, eq76_e2185_d_b0, eq76_e2185_d_b1, eq76_e2185_d_b2, eq76_e2185_d_b3, eq76_e2185_d_b4, eq76_e2185_d_b5, eq76_e2185_d_b6, eq76_e2185_d_b7, eq76_e2185_d_b8, eq76_e2185_d_b9, eq76_e2185_d_b10, eq76_e2185_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e2187;
        let eq76_node_derivatives: [f64; 14] = [eq76_e2187_d_n0, eq76_e2187_d_n1, eq76_e2187_d_n2, eq76_e2187_d_n3, eq76_e2187_d_n4, eq76_e2187_d_n5, eq76_e2187_d_n6, eq76_e2187_d_n7, eq76_e2187_d_n8, eq76_e2187_d_n9, eq76_e2187_d_n10, eq76_e2187_d_n11, eq76_e2187_d_n12, eq76_e2187_d_n13];
        let eq76_branch_derivatives: [f64; 12] = [eq76_e2187_d_b0, eq76_e2187_d_b1, eq76_e2187_d_b2, eq76_e2187_d_b3, eq76_e2187_d_b4, eq76_e2187_d_b5, eq76_e2187_d_b6, eq76_e2187_d_b7, eq76_e2187_d_b8, eq76_e2187_d_b9, eq76_e2187_d_b10, eq76_e2187_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(10),
            multiplicity * (eq76_value),
            &eq76_node_derivatives,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e2195, eq77_e2195_d_n0, eq77_e2195_d_n1, eq77_e2195_d_n2, eq77_e2195_d_n3, eq77_e2195_d_n4, eq77_e2195_d_n5, eq77_e2195_d_n6, eq77_e2195_d_n7, eq77_e2195_d_n8, eq77_e2195_d_n9, eq77_e2195_d_n10, eq77_e2195_d_n11, eq77_e2195_d_n12, eq77_e2195_d_n13, eq77_e2195_d_b0, eq77_e2195_d_b1, eq77_e2195_d_b2, eq77_e2195_d_b3, eq77_e2195_d_b4, eq77_e2195_d_b5, eq77_e2195_d_b6, eq77_e2195_d_b7, eq77_e2195_d_b8, eq77_e2195_d_b9, eq77_e2195_d_b10, eq77_e2195_d_b11,) = {
    if (s.b[2037] && s.b[2038]) {
        let eq77_e2193: f64 = ((nv4 - nv11) * s.v[1022]);
        let eq77_e2193_d_n0: f64 = ((nv4 - nv11) * s.dn[1022][0]);
        let eq77_e2193_d_n1: f64 = ((nv4 - nv11) * s.dn[1022][1]);
        let eq77_e2193_d_n2: f64 = ((nv4 - nv11) * s.dn[1022][2]);
        let eq77_e2193_d_n3: f64 = ((nv4 - nv11) * s.dn[1022][3]);
        let eq77_e2193_d_n4: f64 = (s.v[1022] + ((nv4 - nv11) * s.dn[1022][4]));
        let eq77_e2193_d_n5: f64 = ((nv4 - nv11) * s.dn[1022][5]);
        let eq77_e2193_d_n6: f64 = ((nv4 - nv11) * s.dn[1022][6]);
        let eq77_e2193_d_n7: f64 = ((nv4 - nv11) * s.dn[1022][7]);
        let eq77_e2193_d_n8: f64 = ((nv4 - nv11) * s.dn[1022][8]);
        let eq77_e2193_d_n9: f64 = ((nv4 - nv11) * s.dn[1022][9]);
        let eq77_e2193_d_n10: f64 = ((nv4 - nv11) * s.dn[1022][10]);
        let eq77_e2193_d_n11: f64 = ((-s.v[1022]) + ((nv4 - nv11) * s.dn[1022][11]));
        let eq77_e2193_d_n12: f64 = ((nv4 - nv11) * s.dn[1022][12]);
        let eq77_e2193_d_n13: f64 = ((nv4 - nv11) * s.dn[1022][13]);
        let eq77_e2193_d_b0: f64 = ((nv4 - nv11) * s.db[1022][0]);
        let eq77_e2193_d_b1: f64 = ((nv4 - nv11) * s.db[1022][1]);
        let eq77_e2193_d_b2: f64 = ((nv4 - nv11) * s.db[1022][2]);
        let eq77_e2193_d_b3: f64 = ((nv4 - nv11) * s.db[1022][3]);
        let eq77_e2193_d_b4: f64 = ((nv4 - nv11) * s.db[1022][4]);
        let eq77_e2193_d_b5: f64 = ((nv4 - nv11) * s.db[1022][5]);
        let eq77_e2193_d_b6: f64 = ((nv4 - nv11) * s.db[1022][6]);
        let eq77_e2193_d_b7: f64 = ((nv4 - nv11) * s.db[1022][7]);
        let eq77_e2193_d_b8: f64 = ((nv4 - nv11) * s.db[1022][8]);
        let eq77_e2193_d_b9: f64 = ((nv4 - nv11) * s.db[1022][9]);
        let eq77_e2193_d_b10: f64 = ((nv4 - nv11) * s.db[1022][10]);
        let eq77_e2193_d_b11: f64 = ((nv4 - nv11) * s.db[1022][11]);
        (eq77_e2193, eq77_e2193_d_n0, eq77_e2193_d_n1, eq77_e2193_d_n2, eq77_e2193_d_n3, eq77_e2193_d_n4, eq77_e2193_d_n5, eq77_e2193_d_n6, eq77_e2193_d_n7, eq77_e2193_d_n8, eq77_e2193_d_n9, eq77_e2193_d_n10, eq77_e2193_d_n11, eq77_e2193_d_n12, eq77_e2193_d_n13, eq77_e2193_d_b0, eq77_e2193_d_b1, eq77_e2193_d_b2, eq77_e2193_d_b3, eq77_e2193_d_b4, eq77_e2193_d_b5, eq77_e2193_d_b6, eq77_e2193_d_b7, eq77_e2193_d_b8, eq77_e2193_d_b9, eq77_e2193_d_b10, eq77_e2193_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e2195;
        let eq77_node_derivatives: [f64; 14] = [eq77_e2195_d_n0, eq77_e2195_d_n1, eq77_e2195_d_n2, eq77_e2195_d_n3, eq77_e2195_d_n4, eq77_e2195_d_n5, eq77_e2195_d_n6, eq77_e2195_d_n7, eq77_e2195_d_n8, eq77_e2195_d_n9, eq77_e2195_d_n10, eq77_e2195_d_n11, eq77_e2195_d_n12, eq77_e2195_d_n13];
        let eq77_branch_derivatives: [f64; 12] = [eq77_e2195_d_b0, eq77_e2195_d_b1, eq77_e2195_d_b2, eq77_e2195_d_b3, eq77_e2195_d_b4, eq77_e2195_d_b5, eq77_e2195_d_b6, eq77_e2195_d_b7, eq77_e2195_d_b8, eq77_e2195_d_b9, eq77_e2195_d_b10, eq77_e2195_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(11),
            multiplicity * (eq77_value),
            &eq77_node_derivatives,
            &eq77_branch_derivatives,
            multiplicity,
        );
        let eq78_e2198: f64 = (s.v[379] * s.v[496]);
        let eq78_e2198_d_n0: f64 = ((s.dn[379][0] * s.v[496]) + (s.v[379] * s.dn[496][0]));
        let eq78_e2198_d_n1: f64 = ((s.dn[379][1] * s.v[496]) + (s.v[379] * s.dn[496][1]));
        let eq78_e2198_d_n2: f64 = ((s.dn[379][2] * s.v[496]) + (s.v[379] * s.dn[496][2]));
        let eq78_e2198_d_n3: f64 = ((s.dn[379][3] * s.v[496]) + (s.v[379] * s.dn[496][3]));
        let eq78_e2198_d_n4: f64 = ((s.dn[379][4] * s.v[496]) + (s.v[379] * s.dn[496][4]));
        let eq78_e2198_d_n5: f64 = ((s.dn[379][5] * s.v[496]) + (s.v[379] * s.dn[496][5]));
        let eq78_e2198_d_n6: f64 = ((s.dn[379][6] * s.v[496]) + (s.v[379] * s.dn[496][6]));
        let eq78_e2198_d_n7: f64 = ((s.dn[379][7] * s.v[496]) + (s.v[379] * s.dn[496][7]));
        let eq78_e2198_d_n8: f64 = ((s.dn[379][8] * s.v[496]) + (s.v[379] * s.dn[496][8]));
        let eq78_e2198_d_n9: f64 = ((s.dn[379][9] * s.v[496]) + (s.v[379] * s.dn[496][9]));
        let eq78_e2198_d_n10: f64 = ((s.dn[379][10] * s.v[496]) + (s.v[379] * s.dn[496][10]));
        let eq78_e2198_d_n11: f64 = ((s.dn[379][11] * s.v[496]) + (s.v[379] * s.dn[496][11]));
        let eq78_e2198_d_n12: f64 = ((s.dn[379][12] * s.v[496]) + (s.v[379] * s.dn[496][12]));
        let eq78_e2198_d_n13: f64 = ((s.dn[379][13] * s.v[496]) + (s.v[379] * s.dn[496][13]));
        let eq78_e2198_d_b0: f64 = ((s.db[379][0] * s.v[496]) + (s.v[379] * s.db[496][0]));
        let eq78_e2198_d_b1: f64 = ((s.db[379][1] * s.v[496]) + (s.v[379] * s.db[496][1]));
        let eq78_e2198_d_b2: f64 = ((s.db[379][2] * s.v[496]) + (s.v[379] * s.db[496][2]));
        let eq78_e2198_d_b3: f64 = ((s.db[379][3] * s.v[496]) + (s.v[379] * s.db[496][3]));
        let eq78_e2198_d_b4: f64 = ((s.db[379][4] * s.v[496]) + (s.v[379] * s.db[496][4]));
        let eq78_e2198_d_b5: f64 = ((s.db[379][5] * s.v[496]) + (s.v[379] * s.db[496][5]));
        let eq78_e2198_d_b6: f64 = ((s.db[379][6] * s.v[496]) + (s.v[379] * s.db[496][6]));
        let eq78_e2198_d_b7: f64 = ((s.db[379][7] * s.v[496]) + (s.v[379] * s.db[496][7]));
        let eq78_e2198_d_b8: f64 = ((s.db[379][8] * s.v[496]) + (s.v[379] * s.db[496][8]));
        let eq78_e2198_d_b9: f64 = ((s.db[379][9] * s.v[496]) + (s.v[379] * s.db[496][9]));
        let eq78_e2198_d_b10: f64 = ((s.db[379][10] * s.v[496]) + (s.v[379] * s.db[496][10]));
        let eq78_e2198_d_b11: f64 = ((s.db[379][11] * s.v[496]) + (s.v[379] * s.db[496][11]));
        let eq78_e2201: f64 = ((nv10 - nv7) * s.v[1018]);
        let eq78_e2201_d_n7: f64 = (-s.v[1018]);
        let eq78_e2201_d_n10: f64 = s.v[1018];
        let eq78_e2202: f64 = (eq78_e2198 + eq78_e2201);
        let eq78_e2202_d_n7: f64 = (eq78_e2198_d_n7 + eq78_e2201_d_n7);
        let eq78_e2202_d_n10: f64 = (eq78_e2198_d_n10 + eq78_e2201_d_n10);
        let eq78_value: f64 = eq78_e2202;
        let eq78_node_derivatives: [f64; 14] = [eq78_e2198_d_n0, eq78_e2198_d_n1, eq78_e2198_d_n2, eq78_e2198_d_n3, eq78_e2198_d_n4, eq78_e2198_d_n5, eq78_e2198_d_n6, eq78_e2202_d_n7, eq78_e2198_d_n8, eq78_e2198_d_n9, eq78_e2202_d_n10, eq78_e2198_d_n11, eq78_e2198_d_n12, eq78_e2198_d_n13];
        let eq78_branch_derivatives: [f64; 12] = [eq78_e2198_d_b0, eq78_e2198_d_b1, eq78_e2198_d_b2, eq78_e2198_d_b3, eq78_e2198_d_b4, eq78_e2198_d_b5, eq78_e2198_d_b6, eq78_e2198_d_b7, eq78_e2198_d_b8, eq78_e2198_d_b9, eq78_e2198_d_b10, eq78_e2198_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq78_value),
            &eq78_node_derivatives,
            &eq78_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_8(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq79_e2205: f64 = (s.v[379] * s.v[497]);
        let eq79_e2205_d_n0: f64 = ((s.dn[379][0] * s.v[497]) + (s.v[379] * s.dn[497][0]));
        let eq79_e2205_d_n1: f64 = ((s.dn[379][1] * s.v[497]) + (s.v[379] * s.dn[497][1]));
        let eq79_e2205_d_n2: f64 = ((s.dn[379][2] * s.v[497]) + (s.v[379] * s.dn[497][2]));
        let eq79_e2205_d_n3: f64 = ((s.dn[379][3] * s.v[497]) + (s.v[379] * s.dn[497][3]));
        let eq79_e2205_d_n4: f64 = ((s.dn[379][4] * s.v[497]) + (s.v[379] * s.dn[497][4]));
        let eq79_e2205_d_n5: f64 = ((s.dn[379][5] * s.v[497]) + (s.v[379] * s.dn[497][5]));
        let eq79_e2205_d_n6: f64 = ((s.dn[379][6] * s.v[497]) + (s.v[379] * s.dn[497][6]));
        let eq79_e2205_d_n7: f64 = ((s.dn[379][7] * s.v[497]) + (s.v[379] * s.dn[497][7]));
        let eq79_e2205_d_n8: f64 = ((s.dn[379][8] * s.v[497]) + (s.v[379] * s.dn[497][8]));
        let eq79_e2205_d_n9: f64 = ((s.dn[379][9] * s.v[497]) + (s.v[379] * s.dn[497][9]));
        let eq79_e2205_d_n10: f64 = ((s.dn[379][10] * s.v[497]) + (s.v[379] * s.dn[497][10]));
        let eq79_e2205_d_n11: f64 = ((s.dn[379][11] * s.v[497]) + (s.v[379] * s.dn[497][11]));
        let eq79_e2205_d_n12: f64 = ((s.dn[379][12] * s.v[497]) + (s.v[379] * s.dn[497][12]));
        let eq79_e2205_d_n13: f64 = ((s.dn[379][13] * s.v[497]) + (s.v[379] * s.dn[497][13]));
        let eq79_e2205_d_b0: f64 = ((s.db[379][0] * s.v[497]) + (s.v[379] * s.db[497][0]));
        let eq79_e2205_d_b1: f64 = ((s.db[379][1] * s.v[497]) + (s.v[379] * s.db[497][1]));
        let eq79_e2205_d_b2: f64 = ((s.db[379][2] * s.v[497]) + (s.v[379] * s.db[497][2]));
        let eq79_e2205_d_b3: f64 = ((s.db[379][3] * s.v[497]) + (s.v[379] * s.db[497][3]));
        let eq79_e2205_d_b4: f64 = ((s.db[379][4] * s.v[497]) + (s.v[379] * s.db[497][4]));
        let eq79_e2205_d_b5: f64 = ((s.db[379][5] * s.v[497]) + (s.v[379] * s.db[497][5]));
        let eq79_e2205_d_b6: f64 = ((s.db[379][6] * s.v[497]) + (s.v[379] * s.db[497][6]));
        let eq79_e2205_d_b7: f64 = ((s.db[379][7] * s.v[497]) + (s.v[379] * s.db[497][7]));
        let eq79_e2205_d_b8: f64 = ((s.db[379][8] * s.v[497]) + (s.v[379] * s.db[497][8]));
        let eq79_e2205_d_b9: f64 = ((s.db[379][9] * s.v[497]) + (s.v[379] * s.db[497][9]));
        let eq79_e2205_d_b10: f64 = ((s.db[379][10] * s.v[497]) + (s.v[379] * s.db[497][10]));
        let eq79_e2205_d_b11: f64 = ((s.db[379][11] * s.v[497]) + (s.v[379] * s.db[497][11]));
        let eq79_e2208: f64 = ((nv10 - nv6) * s.v[1018]);
        let eq79_e2208_d_n6: f64 = (-s.v[1018]);
        let eq79_e2208_d_n10: f64 = s.v[1018];
        let eq79_e2209: f64 = (eq79_e2205 + eq79_e2208);
        let eq79_e2209_d_n6: f64 = (eq79_e2205_d_n6 + eq79_e2208_d_n6);
        let eq79_e2209_d_n10: f64 = (eq79_e2205_d_n10 + eq79_e2208_d_n10);
        let eq79_value: f64 = eq79_e2209;
        let eq79_node_derivatives: [f64; 14] = [eq79_e2205_d_n0, eq79_e2205_d_n1, eq79_e2205_d_n2, eq79_e2205_d_n3, eq79_e2205_d_n4, eq79_e2205_d_n5, eq79_e2209_d_n6, eq79_e2205_d_n7, eq79_e2205_d_n8, eq79_e2205_d_n9, eq79_e2209_d_n10, eq79_e2205_d_n11, eq79_e2205_d_n12, eq79_e2205_d_n13];
        let eq79_branch_derivatives: [f64; 12] = [eq79_e2205_d_b0, eq79_e2205_d_b1, eq79_e2205_d_b2, eq79_e2205_d_b3, eq79_e2205_d_b4, eq79_e2205_d_b5, eq79_e2205_d_b6, eq79_e2205_d_b7, eq79_e2205_d_b8, eq79_e2205_d_b9, eq79_e2205_d_b10, eq79_e2205_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq79_value),
            &eq79_node_derivatives,
            &eq79_branch_derivatives,
            multiplicity,
        );
        let eq80_e2212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 21, s.v[520]);
        let eq80_e2212_d_n0: f64 = (s.dn[520][0] * ddt_scale);
        let eq80_e2212_d_n1: f64 = (s.dn[520][1] * ddt_scale);
        let eq80_e2212_d_n2: f64 = (s.dn[520][2] * ddt_scale);
        let eq80_e2212_d_n3: f64 = (s.dn[520][3] * ddt_scale);
        let eq80_e2212_d_n4: f64 = (s.dn[520][4] * ddt_scale);
        let eq80_e2212_d_n5: f64 = (s.dn[520][5] * ddt_scale);
        let eq80_e2212_d_n6: f64 = (s.dn[520][6] * ddt_scale);
        let eq80_e2212_d_n7: f64 = (s.dn[520][7] * ddt_scale);
        let eq80_e2212_d_n8: f64 = (s.dn[520][8] * ddt_scale);
        let eq80_e2212_d_n9: f64 = (s.dn[520][9] * ddt_scale);
        let eq80_e2212_d_n10: f64 = (s.dn[520][10] * ddt_scale);
        let eq80_e2212_d_n11: f64 = (s.dn[520][11] * ddt_scale);
        let eq80_e2212_d_n12: f64 = (s.dn[520][12] * ddt_scale);
        let eq80_e2212_d_n13: f64 = (s.dn[520][13] * ddt_scale);
        let eq80_e2212_d_b0: f64 = (s.db[520][0] * ddt_scale);
        let eq80_e2212_d_b1: f64 = (s.db[520][1] * ddt_scale);
        let eq80_e2212_d_b2: f64 = (s.db[520][2] * ddt_scale);
        let eq80_e2212_d_b3: f64 = (s.db[520][3] * ddt_scale);
        let eq80_e2212_d_b4: f64 = (s.db[520][4] * ddt_scale);
        let eq80_e2212_d_b5: f64 = (s.db[520][5] * ddt_scale);
        let eq80_e2212_d_b6: f64 = (s.db[520][6] * ddt_scale);
        let eq80_e2212_d_b7: f64 = (s.db[520][7] * ddt_scale);
        let eq80_e2212_d_b8: f64 = (s.db[520][8] * ddt_scale);
        let eq80_e2212_d_b9: f64 = (s.db[520][9] * ddt_scale);
        let eq80_e2212_d_b10: f64 = (s.db[520][10] * ddt_scale);
        let eq80_e2212_d_b11: f64 = (s.db[520][11] * ddt_scale);
        let eq80_e2213: f64 = (s.v[379] * eq80_e2212);
        let eq80_e2213_d_n0: f64 = ((s.dn[379][0] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n0));
        let eq80_e2213_d_n1: f64 = ((s.dn[379][1] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n1));
        let eq80_e2213_d_n2: f64 = ((s.dn[379][2] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n2));
        let eq80_e2213_d_n3: f64 = ((s.dn[379][3] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n3));
        let eq80_e2213_d_n4: f64 = ((s.dn[379][4] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n4));
        let eq80_e2213_d_n5: f64 = ((s.dn[379][5] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n5));
        let eq80_e2213_d_n6: f64 = ((s.dn[379][6] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n6));
        let eq80_e2213_d_n7: f64 = ((s.dn[379][7] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n7));
        let eq80_e2213_d_n8: f64 = ((s.dn[379][8] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n8));
        let eq80_e2213_d_n9: f64 = ((s.dn[379][9] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n9));
        let eq80_e2213_d_n10: f64 = ((s.dn[379][10] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n10));
        let eq80_e2213_d_n11: f64 = ((s.dn[379][11] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n11));
        let eq80_e2213_d_n12: f64 = ((s.dn[379][12] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n12));
        let eq80_e2213_d_n13: f64 = ((s.dn[379][13] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n13));
        let eq80_e2213_d_b0: f64 = ((s.db[379][0] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b0));
        let eq80_e2213_d_b1: f64 = ((s.db[379][1] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b1));
        let eq80_e2213_d_b2: f64 = ((s.db[379][2] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b2));
        let eq80_e2213_d_b3: f64 = ((s.db[379][3] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b3));
        let eq80_e2213_d_b4: f64 = ((s.db[379][4] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b4));
        let eq80_e2213_d_b5: f64 = ((s.db[379][5] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b5));
        let eq80_e2213_d_b6: f64 = ((s.db[379][6] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b6));
        let eq80_e2213_d_b7: f64 = ((s.db[379][7] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b7));
        let eq80_e2213_d_b8: f64 = ((s.db[379][8] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b8));
        let eq80_e2213_d_b9: f64 = ((s.db[379][9] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b9));
        let eq80_e2213_d_b10: f64 = ((s.db[379][10] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b10));
        let eq80_e2213_d_b11: f64 = ((s.db[379][11] * eq80_e2212) + (s.v[379] * eq80_e2212_d_b11));
        let eq80_value: f64 = eq80_e2213;
        let eq80_node_derivatives: [f64; 14] = [eq80_e2213_d_n0, eq80_e2213_d_n1, eq80_e2213_d_n2, eq80_e2213_d_n3, eq80_e2213_d_n4, eq80_e2213_d_n5, eq80_e2213_d_n6, eq80_e2213_d_n7, eq80_e2213_d_n8, eq80_e2213_d_n9, eq80_e2213_d_n10, eq80_e2213_d_n11, eq80_e2213_d_n12, eq80_e2213_d_n13];
        let eq80_branch_derivatives: [f64; 12] = [eq80_e2213_d_b0, eq80_e2213_d_b1, eq80_e2213_d_b2, eq80_e2213_d_b3, eq80_e2213_d_b4, eq80_e2213_d_b5, eq80_e2213_d_b6, eq80_e2213_d_b7, eq80_e2213_d_b8, eq80_e2213_d_b9, eq80_e2213_d_b10, eq80_e2213_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq80_value),
            &eq80_node_derivatives,
            &eq80_branch_derivatives,
            multiplicity,
        );
        let eq81_e2216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 22, s.v[525]);
        let eq81_e2216_d_n0: f64 = (s.dn[525][0] * ddt_scale);
        let eq81_e2216_d_n1: f64 = (s.dn[525][1] * ddt_scale);
        let eq81_e2216_d_n2: f64 = (s.dn[525][2] * ddt_scale);
        let eq81_e2216_d_n3: f64 = (s.dn[525][3] * ddt_scale);
        let eq81_e2216_d_n4: f64 = (s.dn[525][4] * ddt_scale);
        let eq81_e2216_d_n5: f64 = (s.dn[525][5] * ddt_scale);
        let eq81_e2216_d_n6: f64 = (s.dn[525][6] * ddt_scale);
        let eq81_e2216_d_n7: f64 = (s.dn[525][7] * ddt_scale);
        let eq81_e2216_d_n8: f64 = (s.dn[525][8] * ddt_scale);
        let eq81_e2216_d_n9: f64 = (s.dn[525][9] * ddt_scale);
        let eq81_e2216_d_n10: f64 = (s.dn[525][10] * ddt_scale);
        let eq81_e2216_d_n11: f64 = (s.dn[525][11] * ddt_scale);
        let eq81_e2216_d_n12: f64 = (s.dn[525][12] * ddt_scale);
        let eq81_e2216_d_n13: f64 = (s.dn[525][13] * ddt_scale);
        let eq81_e2216_d_b0: f64 = (s.db[525][0] * ddt_scale);
        let eq81_e2216_d_b1: f64 = (s.db[525][1] * ddt_scale);
        let eq81_e2216_d_b2: f64 = (s.db[525][2] * ddt_scale);
        let eq81_e2216_d_b3: f64 = (s.db[525][3] * ddt_scale);
        let eq81_e2216_d_b4: f64 = (s.db[525][4] * ddt_scale);
        let eq81_e2216_d_b5: f64 = (s.db[525][5] * ddt_scale);
        let eq81_e2216_d_b6: f64 = (s.db[525][6] * ddt_scale);
        let eq81_e2216_d_b7: f64 = (s.db[525][7] * ddt_scale);
        let eq81_e2216_d_b8: f64 = (s.db[525][8] * ddt_scale);
        let eq81_e2216_d_b9: f64 = (s.db[525][9] * ddt_scale);
        let eq81_e2216_d_b10: f64 = (s.db[525][10] * ddt_scale);
        let eq81_e2216_d_b11: f64 = (s.db[525][11] * ddt_scale);
        let eq81_e2217: f64 = (s.v[379] * eq81_e2216);
        let eq81_e2217_d_n0: f64 = ((s.dn[379][0] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n0));
        let eq81_e2217_d_n1: f64 = ((s.dn[379][1] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n1));
        let eq81_e2217_d_n2: f64 = ((s.dn[379][2] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n2));
        let eq81_e2217_d_n3: f64 = ((s.dn[379][3] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n3));
        let eq81_e2217_d_n4: f64 = ((s.dn[379][4] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n4));
        let eq81_e2217_d_n5: f64 = ((s.dn[379][5] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n5));
        let eq81_e2217_d_n6: f64 = ((s.dn[379][6] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n6));
        let eq81_e2217_d_n7: f64 = ((s.dn[379][7] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n7));
        let eq81_e2217_d_n8: f64 = ((s.dn[379][8] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n8));
        let eq81_e2217_d_n9: f64 = ((s.dn[379][9] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n9));
        let eq81_e2217_d_n10: f64 = ((s.dn[379][10] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n10));
        let eq81_e2217_d_n11: f64 = ((s.dn[379][11] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n11));
        let eq81_e2217_d_n12: f64 = ((s.dn[379][12] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n12));
        let eq81_e2217_d_n13: f64 = ((s.dn[379][13] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n13));
        let eq81_e2217_d_b0: f64 = ((s.db[379][0] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b0));
        let eq81_e2217_d_b1: f64 = ((s.db[379][1] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b1));
        let eq81_e2217_d_b2: f64 = ((s.db[379][2] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b2));
        let eq81_e2217_d_b3: f64 = ((s.db[379][3] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b3));
        let eq81_e2217_d_b4: f64 = ((s.db[379][4] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b4));
        let eq81_e2217_d_b5: f64 = ((s.db[379][5] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b5));
        let eq81_e2217_d_b6: f64 = ((s.db[379][6] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b6));
        let eq81_e2217_d_b7: f64 = ((s.db[379][7] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b7));
        let eq81_e2217_d_b8: f64 = ((s.db[379][8] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b8));
        let eq81_e2217_d_b9: f64 = ((s.db[379][9] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b9));
        let eq81_e2217_d_b10: f64 = ((s.db[379][10] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b10));
        let eq81_e2217_d_b11: f64 = ((s.db[379][11] * eq81_e2216) + (s.v[379] * eq81_e2216_d_b11));
        let eq81_value: f64 = eq81_e2217;
        let eq81_node_derivatives: [f64; 14] = [eq81_e2217_d_n0, eq81_e2217_d_n1, eq81_e2217_d_n2, eq81_e2217_d_n3, eq81_e2217_d_n4, eq81_e2217_d_n5, eq81_e2217_d_n6, eq81_e2217_d_n7, eq81_e2217_d_n8, eq81_e2217_d_n9, eq81_e2217_d_n10, eq81_e2217_d_n11, eq81_e2217_d_n12, eq81_e2217_d_n13];
        let eq81_branch_derivatives: [f64; 12] = [eq81_e2217_d_b0, eq81_e2217_d_b1, eq81_e2217_d_b2, eq81_e2217_d_b3, eq81_e2217_d_b4, eq81_e2217_d_b5, eq81_e2217_d_b6, eq81_e2217_d_b7, eq81_e2217_d_b8, eq81_e2217_d_b9, eq81_e2217_d_b10, eq81_e2217_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq81_value),
            &eq81_node_derivatives,
            &eq81_branch_derivatives,
            multiplicity,
        );
        let (eq82_e2223, eq82_e2223_d_n7, eq82_e2223_d_n11,) = {
    if s.b[2039] {
        let eq82_e2221: f64 = ((nv11 - nv7) * s.v[1018]);
        let eq82_e2221_d_n7: f64 = (-s.v[1018]);
        let eq82_e2221_d_n11: f64 = s.v[1018];
        (eq82_e2221, eq82_e2221_d_n7, eq82_e2221_d_n11,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e2223;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(7),
            multiplicity * (eq82_value),
            7,
            multiplicity * (eq82_e2223_d_n7),
            11,
            multiplicity * (eq82_e2223_d_n11),
        );
        let (eq83_e2229, eq83_e2229_d_n6, eq83_e2229_d_n11,) = {
    if s.b[2039] {
        let eq83_e2227: f64 = ((nv11 - nv6) * s.v[1018]);
        let eq83_e2227_d_n6: f64 = (-s.v[1018]);
        let eq83_e2227_d_n11: f64 = s.v[1018];
        (eq83_e2227, eq83_e2227_d_n6, eq83_e2227_d_n11,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e2229;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(6),
            multiplicity * (eq83_value),
            6,
            multiplicity * (eq83_e2229_d_n6),
            11,
            multiplicity * (eq83_e2229_d_n11),
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq7_e1546, eq7_e1546_d_n0, eq7_e1546_d_n1, eq7_e1546_d_n2, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, eq7_e1546_d_n13, eq7_e1546_d_b0, eq7_e1546_d_b1, eq7_e1546_d_b2, eq7_e1546_d_b3, eq7_e1546_d_b4, eq7_e1546_d_b5, eq7_e1546_d_b6, eq7_e1546_d_b7, eq7_e1546_d_b8, eq7_e1546_d_b9, eq7_e1546_d_b10, eq7_e1546_d_b11, eq7_e1546_q, eq7_e1546_q_d_n0, eq7_e1546_q_d_n1, eq7_e1546_q_d_n2, eq7_e1546_q_d_n3, eq7_e1546_q_d_n4, eq7_e1546_q_d_n5, eq7_e1546_q_d_n6, eq7_e1546_q_d_n7, eq7_e1546_q_d_n8, eq7_e1546_q_d_n9, eq7_e1546_q_d_n10, eq7_e1546_q_d_n11, eq7_e1546_q_d_n12, eq7_e1546_q_d_n13, eq7_e1546_q_d_b0, eq7_e1546_q_d_b1, eq7_e1546_q_d_b2, eq7_e1546_q_d_b3, eq7_e1546_q_d_b4, eq7_e1546_q_d_b5, eq7_e1546_q_d_b6, eq7_e1546_q_d_b7, eq7_e1546_q_d_b8, eq7_e1546_q_d_b9, eq7_e1546_q_d_b10, eq7_e1546_q_d_b11,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq7_e1535: f64 = (s.v[622] * s.v[199]);
        let eq7_e1535_d_n0: f64 = (s.dn[622][0] * s.v[199]);
        let eq7_e1535_d_n1: f64 = (s.dn[622][1] * s.v[199]);
        let eq7_e1535_d_n2: f64 = (s.dn[622][2] * s.v[199]);
        let eq7_e1535_d_n3: f64 = (s.dn[622][3] * s.v[199]);
        let eq7_e1535_d_n4: f64 = (s.dn[622][4] * s.v[199]);
        let eq7_e1535_d_n5: f64 = (s.dn[622][5] * s.v[199]);
        let eq7_e1535_d_n6: f64 = (s.dn[622][6] * s.v[199]);
        let eq7_e1535_d_n7: f64 = (s.dn[622][7] * s.v[199]);
        let eq7_e1535_d_n8: f64 = (s.dn[622][8] * s.v[199]);
        let eq7_e1535_d_n9: f64 = (s.dn[622][9] * s.v[199]);
        let eq7_e1535_d_n10: f64 = (s.dn[622][10] * s.v[199]);
        let eq7_e1535_d_n11: f64 = (s.dn[622][11] * s.v[199]);
        let eq7_e1535_d_n12: f64 = (s.dn[622][12] * s.v[199]);
        let eq7_e1535_d_n13: f64 = (s.dn[622][13] * s.v[199]);
        let eq7_e1535_d_b0: f64 = (s.db[622][0] * s.v[199]);
        let eq7_e1535_d_b1: f64 = (s.db[622][1] * s.v[199]);
        let eq7_e1535_d_b2: f64 = (s.db[622][2] * s.v[199]);
        let eq7_e1535_d_b3: f64 = (s.db[622][3] * s.v[199]);
        let eq7_e1535_d_b4: f64 = (s.db[622][4] * s.v[199]);
        let eq7_e1535_d_b5: f64 = (s.db[622][5] * s.v[199]);
        let eq7_e1535_d_b6: f64 = (s.db[622][6] * s.v[199]);
        let eq7_e1535_d_b7: f64 = (s.db[622][7] * s.v[199]);
        let eq7_e1535_d_b8: f64 = (s.db[622][8] * s.v[199]);
        let eq7_e1535_d_b9: f64 = (s.db[622][9] * s.v[199]);
        let eq7_e1535_d_b10: f64 = (s.db[622][10] * s.v[199]);
        let eq7_e1535_d_b11: f64 = (s.db[622][11] * s.v[199]);
        let eq7_e1537: f64 = (eq7_e1535 * s.v[183]);
        let eq7_e1537_d_n0: f64 = (eq7_e1535_d_n0 * s.v[183]);
        let eq7_e1537_d_n1: f64 = (eq7_e1535_d_n1 * s.v[183]);
        let eq7_e1537_d_n2: f64 = (eq7_e1535_d_n2 * s.v[183]);
        let eq7_e1537_d_n3: f64 = (eq7_e1535_d_n3 * s.v[183]);
        let eq7_e1537_d_n4: f64 = (eq7_e1535_d_n4 * s.v[183]);
        let eq7_e1537_d_n5: f64 = (eq7_e1535_d_n5 * s.v[183]);
        let eq7_e1537_d_n6: f64 = (eq7_e1535_d_n6 * s.v[183]);
        let eq7_e1537_d_n7: f64 = (eq7_e1535_d_n7 * s.v[183]);
        let eq7_e1537_d_n8: f64 = (eq7_e1535_d_n8 * s.v[183]);
        let eq7_e1537_d_n9: f64 = (eq7_e1535_d_n9 * s.v[183]);
        let eq7_e1537_d_n10: f64 = (eq7_e1535_d_n10 * s.v[183]);
        let eq7_e1537_d_n11: f64 = (eq7_e1535_d_n11 * s.v[183]);
        let eq7_e1537_d_n12: f64 = (eq7_e1535_d_n12 * s.v[183]);
        let eq7_e1537_d_n13: f64 = (eq7_e1535_d_n13 * s.v[183]);
        let eq7_e1537_d_b0: f64 = (eq7_e1535_d_b0 * s.v[183]);
        let eq7_e1537_d_b1: f64 = (eq7_e1535_d_b1 * s.v[183]);
        let eq7_e1537_d_b2: f64 = (eq7_e1535_d_b2 * s.v[183]);
        let eq7_e1537_d_b3: f64 = (eq7_e1535_d_b3 * s.v[183]);
        let eq7_e1537_d_b4: f64 = (eq7_e1535_d_b4 * s.v[183]);
        let eq7_e1537_d_b5: f64 = (eq7_e1535_d_b5 * s.v[183]);
        let eq7_e1537_d_b6: f64 = (eq7_e1535_d_b6 * s.v[183]);
        let eq7_e1537_d_b7: f64 = (eq7_e1535_d_b7 * s.v[183]);
        let eq7_e1537_d_b8: f64 = (eq7_e1535_d_b8 * s.v[183]);
        let eq7_e1537_d_b9: f64 = (eq7_e1535_d_b9 * s.v[183]);
        let eq7_e1537_d_b10: f64 = (eq7_e1535_d_b10 * s.v[183]);
        let eq7_e1537_d_b11: f64 = (eq7_e1535_d_b11 * s.v[183]);
        let eq7_e1539: f64 = (eq7_e1537 * p.p2);
        let eq7_e1539_d_n0: f64 = (eq7_e1537_d_n0 * p.p2);
        let eq7_e1539_d_n1: f64 = (eq7_e1537_d_n1 * p.p2);
        let eq7_e1539_d_n2: f64 = (eq7_e1537_d_n2 * p.p2);
        let eq7_e1539_d_n3: f64 = (eq7_e1537_d_n3 * p.p2);
        let eq7_e1539_d_n4: f64 = (eq7_e1537_d_n4 * p.p2);
        let eq7_e1539_d_n5: f64 = (eq7_e1537_d_n5 * p.p2);
        let eq7_e1539_d_n6: f64 = (eq7_e1537_d_n6 * p.p2);
        let eq7_e1539_d_n7: f64 = (eq7_e1537_d_n7 * p.p2);
        let eq7_e1539_d_n8: f64 = (eq7_e1537_d_n8 * p.p2);
        let eq7_e1539_d_n9: f64 = (eq7_e1537_d_n9 * p.p2);
        let eq7_e1539_d_n10: f64 = (eq7_e1537_d_n10 * p.p2);
        let eq7_e1539_d_n11: f64 = (eq7_e1537_d_n11 * p.p2);
        let eq7_e1539_d_n12: f64 = (eq7_e1537_d_n12 * p.p2);
        let eq7_e1539_d_n13: f64 = (eq7_e1537_d_n13 * p.p2);
        let eq7_e1539_d_b0: f64 = (eq7_e1537_d_b0 * p.p2);
        let eq7_e1539_d_b1: f64 = (eq7_e1537_d_b1 * p.p2);
        let eq7_e1539_d_b2: f64 = (eq7_e1537_d_b2 * p.p2);
        let eq7_e1539_d_b3: f64 = (eq7_e1537_d_b3 * p.p2);
        let eq7_e1539_d_b4: f64 = (eq7_e1537_d_b4 * p.p2);
        let eq7_e1539_d_b5: f64 = (eq7_e1537_d_b5 * p.p2);
        let eq7_e1539_d_b6: f64 = (eq7_e1537_d_b6 * p.p2);
        let eq7_e1539_d_b7: f64 = (eq7_e1537_d_b7 * p.p2);
        let eq7_e1539_d_b8: f64 = (eq7_e1537_d_b8 * p.p2);
        let eq7_e1539_d_b9: f64 = (eq7_e1537_d_b9 * p.p2);
        let eq7_e1539_d_b10: f64 = (eq7_e1537_d_b10 * p.p2);
        let eq7_e1539_d_b11: f64 = (eq7_e1537_d_b11 * p.p2);
        let eq7_e1541: f64 = (eq7_e1539 * s.v[184]);
        let eq7_e1541_d_n0: f64 = (eq7_e1539_d_n0 * s.v[184]);
        let eq7_e1541_d_n1: f64 = (eq7_e1539_d_n1 * s.v[184]);
        let eq7_e1541_d_n2: f64 = (eq7_e1539_d_n2 * s.v[184]);
        let eq7_e1541_d_n3: f64 = (eq7_e1539_d_n3 * s.v[184]);
        let eq7_e1541_d_n4: f64 = (eq7_e1539_d_n4 * s.v[184]);
        let eq7_e1541_d_n5: f64 = (eq7_e1539_d_n5 * s.v[184]);
        let eq7_e1541_d_n6: f64 = (eq7_e1539_d_n6 * s.v[184]);
        let eq7_e1541_d_n7: f64 = (eq7_e1539_d_n7 * s.v[184]);
        let eq7_e1541_d_n8: f64 = (eq7_e1539_d_n8 * s.v[184]);
        let eq7_e1541_d_n9: f64 = (eq7_e1539_d_n9 * s.v[184]);
        let eq7_e1541_d_n10: f64 = (eq7_e1539_d_n10 * s.v[184]);
        let eq7_e1541_d_n11: f64 = (eq7_e1539_d_n11 * s.v[184]);
        let eq7_e1541_d_n12: f64 = (eq7_e1539_d_n12 * s.v[184]);
        let eq7_e1541_d_n13: f64 = (eq7_e1539_d_n13 * s.v[184]);
        let eq7_e1541_d_b0: f64 = (eq7_e1539_d_b0 * s.v[184]);
        let eq7_e1541_d_b1: f64 = (eq7_e1539_d_b1 * s.v[184]);
        let eq7_e1541_d_b2: f64 = (eq7_e1539_d_b2 * s.v[184]);
        let eq7_e1541_d_b3: f64 = (eq7_e1539_d_b3 * s.v[184]);
        let eq7_e1541_d_b4: f64 = (eq7_e1539_d_b4 * s.v[184]);
        let eq7_e1541_d_b5: f64 = (eq7_e1539_d_b5 * s.v[184]);
        let eq7_e1541_d_b6: f64 = (eq7_e1539_d_b6 * s.v[184]);
        let eq7_e1541_d_b7: f64 = (eq7_e1539_d_b7 * s.v[184]);
        let eq7_e1541_d_b8: f64 = (eq7_e1539_d_b8 * s.v[184]);
        let eq7_e1541_d_b9: f64 = (eq7_e1539_d_b9 * s.v[184]);
        let eq7_e1541_d_b10: f64 = (eq7_e1539_d_b10 * s.v[184]);
        let eq7_e1541_d_b11: f64 = (eq7_e1539_d_b11 * s.v[184]);
        let eq7_e1543: f64 = (eq7_e1541 * (nv12 - 0.0));
        let eq7_e1543_d_n0: f64 = (eq7_e1541_d_n0 * (nv12 - 0.0));
        let eq7_e1543_d_n1: f64 = (eq7_e1541_d_n1 * (nv12 - 0.0));
        let eq7_e1543_d_n2: f64 = (eq7_e1541_d_n2 * (nv12 - 0.0));
        let eq7_e1543_d_n3: f64 = (eq7_e1541_d_n3 * (nv12 - 0.0));
        let eq7_e1543_d_n4: f64 = (eq7_e1541_d_n4 * (nv12 - 0.0));
        let eq7_e1543_d_n5: f64 = (eq7_e1541_d_n5 * (nv12 - 0.0));
        let eq7_e1543_d_n6: f64 = (eq7_e1541_d_n6 * (nv12 - 0.0));
        let eq7_e1543_d_n7: f64 = (eq7_e1541_d_n7 * (nv12 - 0.0));
        let eq7_e1543_d_n8: f64 = (eq7_e1541_d_n8 * (nv12 - 0.0));
        let eq7_e1543_d_n9: f64 = (eq7_e1541_d_n9 * (nv12 - 0.0));
        let eq7_e1543_d_n10: f64 = (eq7_e1541_d_n10 * (nv12 - 0.0));
        let eq7_e1543_d_n11: f64 = (eq7_e1541_d_n11 * (nv12 - 0.0));
        let eq7_e1543_d_n12: f64 = ((eq7_e1541_d_n12 * (nv12 - 0.0)) + eq7_e1541);
        let eq7_e1543_d_n13: f64 = (eq7_e1541_d_n13 * (nv12 - 0.0));
        let eq7_e1543_d_b0: f64 = (eq7_e1541_d_b0 * (nv12 - 0.0));
        let eq7_e1543_d_b1: f64 = (eq7_e1541_d_b1 * (nv12 - 0.0));
        let eq7_e1543_d_b2: f64 = (eq7_e1541_d_b2 * (nv12 - 0.0));
        let eq7_e1543_d_b3: f64 = (eq7_e1541_d_b3 * (nv12 - 0.0));
        let eq7_e1543_d_b4: f64 = (eq7_e1541_d_b4 * (nv12 - 0.0));
        let eq7_e1543_d_b5: f64 = (eq7_e1541_d_b5 * (nv12 - 0.0));
        let eq7_e1543_d_b6: f64 = (eq7_e1541_d_b6 * (nv12 - 0.0));
        let eq7_e1543_d_b7: f64 = (eq7_e1541_d_b7 * (nv12 - 0.0));
        let eq7_e1543_d_b8: f64 = (eq7_e1541_d_b8 * (nv12 - 0.0));
        let eq7_e1543_d_b9: f64 = (eq7_e1541_d_b9 * (nv12 - 0.0));
        let eq7_e1543_d_b10: f64 = (eq7_e1541_d_b10 * (nv12 - 0.0));
        let eq7_e1543_d_b11: f64 = (eq7_e1541_d_b11 * (nv12 - 0.0));
        let eq7_e1544_q: f64 = eq7_e1543;
        (eq7_e1543, eq7_e1543_d_n0, eq7_e1543_d_n1, eq7_e1543_d_n2, eq7_e1543_d_n3, eq7_e1543_d_n4, eq7_e1543_d_n5, eq7_e1543_d_n6, eq7_e1543_d_n7, eq7_e1543_d_n8, eq7_e1543_d_n9, eq7_e1543_d_n10, eq7_e1543_d_n11, eq7_e1543_d_n12, eq7_e1543_d_n13, eq7_e1543_d_b0, eq7_e1543_d_b1, eq7_e1543_d_b2, eq7_e1543_d_b3, eq7_e1543_d_b4, eq7_e1543_d_b5, eq7_e1543_d_b6, eq7_e1543_d_b7, eq7_e1543_d_b8, eq7_e1543_d_b9, eq7_e1543_d_b10, eq7_e1543_d_b11, eq7_e1544_q, eq7_e1543_d_n0, eq7_e1543_d_n1, eq7_e1543_d_n2, eq7_e1543_d_n3, eq7_e1543_d_n4, eq7_e1543_d_n5, eq7_e1543_d_n6, eq7_e1543_d_n7, eq7_e1543_d_n8, eq7_e1543_d_n9, eq7_e1543_d_n10, eq7_e1543_d_n11, eq7_e1543_d_n12, eq7_e1543_d_n13, eq7_e1543_d_b0, eq7_e1543_d_b1, eq7_e1543_d_b2, eq7_e1543_d_b3, eq7_e1543_d_b4, eq7_e1543_d_b5, eq7_e1543_d_b6, eq7_e1543_d_b7, eq7_e1543_d_b8, eq7_e1543_d_b9, eq7_e1543_d_b10, eq7_e1543_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 14] = [eq7_e1546_q_d_n0, eq7_e1546_q_d_n1, eq7_e1546_q_d_n2, eq7_e1546_q_d_n3, eq7_e1546_q_d_n4, eq7_e1546_q_d_n5, eq7_e1546_q_d_n6, eq7_e1546_q_d_n7, eq7_e1546_q_d_n8, eq7_e1546_q_d_n9, eq7_e1546_q_d_n10, eq7_e1546_q_d_n11, eq7_e1546_q_d_n12, eq7_e1546_q_d_n13];
        let eq7_reactive_branch_derivatives: [f64; 12] = [eq7_e1546_q_d_b0, eq7_e1546_q_d_b1, eq7_e1546_q_d_b2, eq7_e1546_q_d_b3, eq7_e1546_q_d_b4, eq7_e1546_q_d_b5, eq7_e1546_q_d_b6, eq7_e1546_q_d_b7, eq7_e1546_q_d_b8, eq7_e1546_q_d_b9, eq7_e1546_q_d_b10, eq7_e1546_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e1600, eq10_e1600_d_n0, eq10_e1600_d_n1, eq10_e1600_d_n2, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, eq10_e1600_d_n13, eq10_e1600_d_b0, eq10_e1600_d_b1, eq10_e1600_d_b2, eq10_e1600_d_b3, eq10_e1600_d_b4, eq10_e1600_d_b5, eq10_e1600_d_b6, eq10_e1600_d_b7, eq10_e1600_d_b8, eq10_e1600_d_b9, eq10_e1600_d_b10, eq10_e1600_d_b11, eq10_e1600_q, eq10_e1600_q_d_n0, eq10_e1600_q_d_n1, eq10_e1600_q_d_n2, eq10_e1600_q_d_n3, eq10_e1600_q_d_n4, eq10_e1600_q_d_n5, eq10_e1600_q_d_n6, eq10_e1600_q_d_n7, eq10_e1600_q_d_n8, eq10_e1600_q_d_n9, eq10_e1600_q_d_n10, eq10_e1600_q_d_n11, eq10_e1600_q_d_n12, eq10_e1600_q_d_n13, eq10_e1600_q_d_b0, eq10_e1600_q_d_b1, eq10_e1600_q_d_b2, eq10_e1600_q_d_b3, eq10_e1600_q_d_b4, eq10_e1600_q_d_b5, eq10_e1600_q_d_b6, eq10_e1600_q_d_b7, eq10_e1600_q_d_b8, eq10_e1600_q_d_b9, eq10_e1600_q_d_b10, eq10_e1600_q_d_b11,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq10_e1584: f64 = (1.0 + s.v[211]);
        let eq10_e1586: f64 = (eq10_e1584 * s.v[622]);
        let eq10_e1586_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq10_e1584 * s.dn[622][0]));
        let eq10_e1586_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq10_e1584 * s.dn[622][1]));
        let eq10_e1586_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq10_e1584 * s.dn[622][2]));
        let eq10_e1586_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq10_e1584 * s.dn[622][3]));
        let eq10_e1586_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq10_e1584 * s.dn[622][4]));
        let eq10_e1586_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq10_e1584 * s.dn[622][5]));
        let eq10_e1586_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq10_e1584 * s.dn[622][6]));
        let eq10_e1586_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq10_e1584 * s.dn[622][7]));
        let eq10_e1586_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq10_e1584 * s.dn[622][8]));
        let eq10_e1586_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq10_e1584 * s.dn[622][9]));
        let eq10_e1586_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq10_e1584 * s.dn[622][10]));
        let eq10_e1586_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq10_e1584 * s.dn[622][11]));
        let eq10_e1586_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq10_e1584 * s.dn[622][12]));
        let eq10_e1586_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq10_e1584 * s.dn[622][13]));
        let eq10_e1586_d_b0: f64 = ((s.db[211][0] * s.v[622]) + (eq10_e1584 * s.db[622][0]));
        let eq10_e1586_d_b1: f64 = ((s.db[211][1] * s.v[622]) + (eq10_e1584 * s.db[622][1]));
        let eq10_e1586_d_b2: f64 = ((s.db[211][2] * s.v[622]) + (eq10_e1584 * s.db[622][2]));
        let eq10_e1586_d_b3: f64 = ((s.db[211][3] * s.v[622]) + (eq10_e1584 * s.db[622][3]));
        let eq10_e1586_d_b4: f64 = ((s.db[211][4] * s.v[622]) + (eq10_e1584 * s.db[622][4]));
        let eq10_e1586_d_b5: f64 = ((s.db[211][5] * s.v[622]) + (eq10_e1584 * s.db[622][5]));
        let eq10_e1586_d_b6: f64 = ((s.db[211][6] * s.v[622]) + (eq10_e1584 * s.db[622][6]));
        let eq10_e1586_d_b7: f64 = ((s.db[211][7] * s.v[622]) + (eq10_e1584 * s.db[622][7]));
        let eq10_e1586_d_b8: f64 = ((s.db[211][8] * s.v[622]) + (eq10_e1584 * s.db[622][8]));
        let eq10_e1586_d_b9: f64 = ((s.db[211][9] * s.v[622]) + (eq10_e1584 * s.db[622][9]));
        let eq10_e1586_d_b10: f64 = ((s.db[211][10] * s.v[622]) + (eq10_e1584 * s.db[622][10]));
        let eq10_e1586_d_b11: f64 = ((s.db[211][11] * s.v[622]) + (eq10_e1584 * s.db[622][11]));
        let eq10_e1588: f64 = (eq10_e1586 * s.v[199]);
        let eq10_e1588_d_n0: f64 = (eq10_e1586_d_n0 * s.v[199]);
        let eq10_e1588_d_n1: f64 = (eq10_e1586_d_n1 * s.v[199]);
        let eq10_e1588_d_n2: f64 = (eq10_e1586_d_n2 * s.v[199]);
        let eq10_e1588_d_n3: f64 = (eq10_e1586_d_n3 * s.v[199]);
        let eq10_e1588_d_n4: f64 = (eq10_e1586_d_n4 * s.v[199]);
        let eq10_e1588_d_n5: f64 = (eq10_e1586_d_n5 * s.v[199]);
        let eq10_e1588_d_n6: f64 = (eq10_e1586_d_n6 * s.v[199]);
        let eq10_e1588_d_n7: f64 = (eq10_e1586_d_n7 * s.v[199]);
        let eq10_e1588_d_n8: f64 = (eq10_e1586_d_n8 * s.v[199]);
        let eq10_e1588_d_n9: f64 = (eq10_e1586_d_n9 * s.v[199]);
        let eq10_e1588_d_n10: f64 = (eq10_e1586_d_n10 * s.v[199]);
        let eq10_e1588_d_n11: f64 = (eq10_e1586_d_n11 * s.v[199]);
        let eq10_e1588_d_n12: f64 = (eq10_e1586_d_n12 * s.v[199]);
        let eq10_e1588_d_n13: f64 = (eq10_e1586_d_n13 * s.v[199]);
        let eq10_e1588_d_b0: f64 = (eq10_e1586_d_b0 * s.v[199]);
        let eq10_e1588_d_b1: f64 = (eq10_e1586_d_b1 * s.v[199]);
        let eq10_e1588_d_b2: f64 = (eq10_e1586_d_b2 * s.v[199]);
        let eq10_e1588_d_b3: f64 = (eq10_e1586_d_b3 * s.v[199]);
        let eq10_e1588_d_b4: f64 = (eq10_e1586_d_b4 * s.v[199]);
        let eq10_e1588_d_b5: f64 = (eq10_e1586_d_b5 * s.v[199]);
        let eq10_e1588_d_b6: f64 = (eq10_e1586_d_b6 * s.v[199]);
        let eq10_e1588_d_b7: f64 = (eq10_e1586_d_b7 * s.v[199]);
        let eq10_e1588_d_b8: f64 = (eq10_e1586_d_b8 * s.v[199]);
        let eq10_e1588_d_b9: f64 = (eq10_e1586_d_b9 * s.v[199]);
        let eq10_e1588_d_b10: f64 = (eq10_e1586_d_b10 * s.v[199]);
        let eq10_e1588_d_b11: f64 = (eq10_e1586_d_b11 * s.v[199]);
        let eq10_e1590: f64 = (eq10_e1588 * s.v[183]);
        let eq10_e1590_d_n0: f64 = (eq10_e1588_d_n0 * s.v[183]);
        let eq10_e1590_d_n1: f64 = (eq10_e1588_d_n1 * s.v[183]);
        let eq10_e1590_d_n2: f64 = (eq10_e1588_d_n2 * s.v[183]);
        let eq10_e1590_d_n3: f64 = (eq10_e1588_d_n3 * s.v[183]);
        let eq10_e1590_d_n4: f64 = (eq10_e1588_d_n4 * s.v[183]);
        let eq10_e1590_d_n5: f64 = (eq10_e1588_d_n5 * s.v[183]);
        let eq10_e1590_d_n6: f64 = (eq10_e1588_d_n6 * s.v[183]);
        let eq10_e1590_d_n7: f64 = (eq10_e1588_d_n7 * s.v[183]);
        let eq10_e1590_d_n8: f64 = (eq10_e1588_d_n8 * s.v[183]);
        let eq10_e1590_d_n9: f64 = (eq10_e1588_d_n9 * s.v[183]);
        let eq10_e1590_d_n10: f64 = (eq10_e1588_d_n10 * s.v[183]);
        let eq10_e1590_d_n11: f64 = (eq10_e1588_d_n11 * s.v[183]);
        let eq10_e1590_d_n12: f64 = (eq10_e1588_d_n12 * s.v[183]);
        let eq10_e1590_d_n13: f64 = (eq10_e1588_d_n13 * s.v[183]);
        let eq10_e1590_d_b0: f64 = (eq10_e1588_d_b0 * s.v[183]);
        let eq10_e1590_d_b1: f64 = (eq10_e1588_d_b1 * s.v[183]);
        let eq10_e1590_d_b2: f64 = (eq10_e1588_d_b2 * s.v[183]);
        let eq10_e1590_d_b3: f64 = (eq10_e1588_d_b3 * s.v[183]);
        let eq10_e1590_d_b4: f64 = (eq10_e1588_d_b4 * s.v[183]);
        let eq10_e1590_d_b5: f64 = (eq10_e1588_d_b5 * s.v[183]);
        let eq10_e1590_d_b6: f64 = (eq10_e1588_d_b6 * s.v[183]);
        let eq10_e1590_d_b7: f64 = (eq10_e1588_d_b7 * s.v[183]);
        let eq10_e1590_d_b8: f64 = (eq10_e1588_d_b8 * s.v[183]);
        let eq10_e1590_d_b9: f64 = (eq10_e1588_d_b9 * s.v[183]);
        let eq10_e1590_d_b10: f64 = (eq10_e1588_d_b10 * s.v[183]);
        let eq10_e1590_d_b11: f64 = (eq10_e1588_d_b11 * s.v[183]);
        let eq10_e1592: f64 = (eq10_e1590 * p.p2);
        let eq10_e1592_d_n0: f64 = (eq10_e1590_d_n0 * p.p2);
        let eq10_e1592_d_n1: f64 = (eq10_e1590_d_n1 * p.p2);
        let eq10_e1592_d_n2: f64 = (eq10_e1590_d_n2 * p.p2);
        let eq10_e1592_d_n3: f64 = (eq10_e1590_d_n3 * p.p2);
        let eq10_e1592_d_n4: f64 = (eq10_e1590_d_n4 * p.p2);
        let eq10_e1592_d_n5: f64 = (eq10_e1590_d_n5 * p.p2);
        let eq10_e1592_d_n6: f64 = (eq10_e1590_d_n6 * p.p2);
        let eq10_e1592_d_n7: f64 = (eq10_e1590_d_n7 * p.p2);
        let eq10_e1592_d_n8: f64 = (eq10_e1590_d_n8 * p.p2);
        let eq10_e1592_d_n9: f64 = (eq10_e1590_d_n9 * p.p2);
        let eq10_e1592_d_n10: f64 = (eq10_e1590_d_n10 * p.p2);
        let eq10_e1592_d_n11: f64 = (eq10_e1590_d_n11 * p.p2);
        let eq10_e1592_d_n12: f64 = (eq10_e1590_d_n12 * p.p2);
        let eq10_e1592_d_n13: f64 = (eq10_e1590_d_n13 * p.p2);
        let eq10_e1592_d_b0: f64 = (eq10_e1590_d_b0 * p.p2);
        let eq10_e1592_d_b1: f64 = (eq10_e1590_d_b1 * p.p2);
        let eq10_e1592_d_b2: f64 = (eq10_e1590_d_b2 * p.p2);
        let eq10_e1592_d_b3: f64 = (eq10_e1590_d_b3 * p.p2);
        let eq10_e1592_d_b4: f64 = (eq10_e1590_d_b4 * p.p2);
        let eq10_e1592_d_b5: f64 = (eq10_e1590_d_b5 * p.p2);
        let eq10_e1592_d_b6: f64 = (eq10_e1590_d_b6 * p.p2);
        let eq10_e1592_d_b7: f64 = (eq10_e1590_d_b7 * p.p2);
        let eq10_e1592_d_b8: f64 = (eq10_e1590_d_b8 * p.p2);
        let eq10_e1592_d_b9: f64 = (eq10_e1590_d_b9 * p.p2);
        let eq10_e1592_d_b10: f64 = (eq10_e1590_d_b10 * p.p2);
        let eq10_e1592_d_b11: f64 = (eq10_e1590_d_b11 * p.p2);
        let eq10_e1594: f64 = (eq10_e1592 * s.v[184]);
        let eq10_e1594_d_n0: f64 = (eq10_e1592_d_n0 * s.v[184]);
        let eq10_e1594_d_n1: f64 = (eq10_e1592_d_n1 * s.v[184]);
        let eq10_e1594_d_n2: f64 = (eq10_e1592_d_n2 * s.v[184]);
        let eq10_e1594_d_n3: f64 = (eq10_e1592_d_n3 * s.v[184]);
        let eq10_e1594_d_n4: f64 = (eq10_e1592_d_n4 * s.v[184]);
        let eq10_e1594_d_n5: f64 = (eq10_e1592_d_n5 * s.v[184]);
        let eq10_e1594_d_n6: f64 = (eq10_e1592_d_n6 * s.v[184]);
        let eq10_e1594_d_n7: f64 = (eq10_e1592_d_n7 * s.v[184]);
        let eq10_e1594_d_n8: f64 = (eq10_e1592_d_n8 * s.v[184]);
        let eq10_e1594_d_n9: f64 = (eq10_e1592_d_n9 * s.v[184]);
        let eq10_e1594_d_n10: f64 = (eq10_e1592_d_n10 * s.v[184]);
        let eq10_e1594_d_n11: f64 = (eq10_e1592_d_n11 * s.v[184]);
        let eq10_e1594_d_n12: f64 = (eq10_e1592_d_n12 * s.v[184]);
        let eq10_e1594_d_n13: f64 = (eq10_e1592_d_n13 * s.v[184]);
        let eq10_e1594_d_b0: f64 = (eq10_e1592_d_b0 * s.v[184]);
        let eq10_e1594_d_b1: f64 = (eq10_e1592_d_b1 * s.v[184]);
        let eq10_e1594_d_b2: f64 = (eq10_e1592_d_b2 * s.v[184]);
        let eq10_e1594_d_b3: f64 = (eq10_e1592_d_b3 * s.v[184]);
        let eq10_e1594_d_b4: f64 = (eq10_e1592_d_b4 * s.v[184]);
        let eq10_e1594_d_b5: f64 = (eq10_e1592_d_b5 * s.v[184]);
        let eq10_e1594_d_b6: f64 = (eq10_e1592_d_b6 * s.v[184]);
        let eq10_e1594_d_b7: f64 = (eq10_e1592_d_b7 * s.v[184]);
        let eq10_e1594_d_b8: f64 = (eq10_e1592_d_b8 * s.v[184]);
        let eq10_e1594_d_b9: f64 = (eq10_e1592_d_b9 * s.v[184]);
        let eq10_e1594_d_b10: f64 = (eq10_e1592_d_b10 * s.v[184]);
        let eq10_e1594_d_b11: f64 = (eq10_e1592_d_b11 * s.v[184]);
        let eq10_e1596: f64 = (eq10_e1594 * (nv12 - 0.0));
        let eq10_e1596_d_n0: f64 = (eq10_e1594_d_n0 * (nv12 - 0.0));
        let eq10_e1596_d_n1: f64 = (eq10_e1594_d_n1 * (nv12 - 0.0));
        let eq10_e1596_d_n2: f64 = (eq10_e1594_d_n2 * (nv12 - 0.0));
        let eq10_e1596_d_n3: f64 = (eq10_e1594_d_n3 * (nv12 - 0.0));
        let eq10_e1596_d_n4: f64 = (eq10_e1594_d_n4 * (nv12 - 0.0));
        let eq10_e1596_d_n5: f64 = (eq10_e1594_d_n5 * (nv12 - 0.0));
        let eq10_e1596_d_n6: f64 = (eq10_e1594_d_n6 * (nv12 - 0.0));
        let eq10_e1596_d_n7: f64 = (eq10_e1594_d_n7 * (nv12 - 0.0));
        let eq10_e1596_d_n8: f64 = (eq10_e1594_d_n8 * (nv12 - 0.0));
        let eq10_e1596_d_n9: f64 = (eq10_e1594_d_n9 * (nv12 - 0.0));
        let eq10_e1596_d_n10: f64 = (eq10_e1594_d_n10 * (nv12 - 0.0));
        let eq10_e1596_d_n11: f64 = (eq10_e1594_d_n11 * (nv12 - 0.0));
        let eq10_e1596_d_n12: f64 = ((eq10_e1594_d_n12 * (nv12 - 0.0)) + eq10_e1594);
        let eq10_e1596_d_n13: f64 = (eq10_e1594_d_n13 * (nv12 - 0.0));
        let eq10_e1596_d_b0: f64 = (eq10_e1594_d_b0 * (nv12 - 0.0));
        let eq10_e1596_d_b1: f64 = (eq10_e1594_d_b1 * (nv12 - 0.0));
        let eq10_e1596_d_b2: f64 = (eq10_e1594_d_b2 * (nv12 - 0.0));
        let eq10_e1596_d_b3: f64 = (eq10_e1594_d_b3 * (nv12 - 0.0));
        let eq10_e1596_d_b4: f64 = (eq10_e1594_d_b4 * (nv12 - 0.0));
        let eq10_e1596_d_b5: f64 = (eq10_e1594_d_b5 * (nv12 - 0.0));
        let eq10_e1596_d_b6: f64 = (eq10_e1594_d_b6 * (nv12 - 0.0));
        let eq10_e1596_d_b7: f64 = (eq10_e1594_d_b7 * (nv12 - 0.0));
        let eq10_e1596_d_b8: f64 = (eq10_e1594_d_b8 * (nv12 - 0.0));
        let eq10_e1596_d_b9: f64 = (eq10_e1594_d_b9 * (nv12 - 0.0));
        let eq10_e1596_d_b10: f64 = (eq10_e1594_d_b10 * (nv12 - 0.0));
        let eq10_e1596_d_b11: f64 = (eq10_e1594_d_b11 * (nv12 - 0.0));
        let eq10_e1597: f64 = (0.5 * eq10_e1596);
        let eq10_e1597_d_n0: f64 = (0.5 * eq10_e1596_d_n0);
        let eq10_e1597_d_n1: f64 = (0.5 * eq10_e1596_d_n1);
        let eq10_e1597_d_n2: f64 = (0.5 * eq10_e1596_d_n2);
        let eq10_e1597_d_n3: f64 = (0.5 * eq10_e1596_d_n3);
        let eq10_e1597_d_n4: f64 = (0.5 * eq10_e1596_d_n4);
        let eq10_e1597_d_n5: f64 = (0.5 * eq10_e1596_d_n5);
        let eq10_e1597_d_n6: f64 = (0.5 * eq10_e1596_d_n6);
        let eq10_e1597_d_n7: f64 = (0.5 * eq10_e1596_d_n7);
        let eq10_e1597_d_n8: f64 = (0.5 * eq10_e1596_d_n8);
        let eq10_e1597_d_n9: f64 = (0.5 * eq10_e1596_d_n9);
        let eq10_e1597_d_n10: f64 = (0.5 * eq10_e1596_d_n10);
        let eq10_e1597_d_n11: f64 = (0.5 * eq10_e1596_d_n11);
        let eq10_e1597_d_n12: f64 = (0.5 * eq10_e1596_d_n12);
        let eq10_e1597_d_n13: f64 = (0.5 * eq10_e1596_d_n13);
        let eq10_e1597_d_b0: f64 = (0.5 * eq10_e1596_d_b0);
        let eq10_e1597_d_b1: f64 = (0.5 * eq10_e1596_d_b1);
        let eq10_e1597_d_b2: f64 = (0.5 * eq10_e1596_d_b2);
        let eq10_e1597_d_b3: f64 = (0.5 * eq10_e1596_d_b3);
        let eq10_e1597_d_b4: f64 = (0.5 * eq10_e1596_d_b4);
        let eq10_e1597_d_b5: f64 = (0.5 * eq10_e1596_d_b5);
        let eq10_e1597_d_b6: f64 = (0.5 * eq10_e1596_d_b6);
        let eq10_e1597_d_b7: f64 = (0.5 * eq10_e1596_d_b7);
        let eq10_e1597_d_b8: f64 = (0.5 * eq10_e1596_d_b8);
        let eq10_e1597_d_b9: f64 = (0.5 * eq10_e1596_d_b9);
        let eq10_e1597_d_b10: f64 = (0.5 * eq10_e1596_d_b10);
        let eq10_e1597_d_b11: f64 = (0.5 * eq10_e1596_d_b11);
        let eq10_e1598_q: f64 = eq10_e1597;
        (eq10_e1597, eq10_e1597_d_n0, eq10_e1597_d_n1, eq10_e1597_d_n2, eq10_e1597_d_n3, eq10_e1597_d_n4, eq10_e1597_d_n5, eq10_e1597_d_n6, eq10_e1597_d_n7, eq10_e1597_d_n8, eq10_e1597_d_n9, eq10_e1597_d_n10, eq10_e1597_d_n11, eq10_e1597_d_n12, eq10_e1597_d_n13, eq10_e1597_d_b0, eq10_e1597_d_b1, eq10_e1597_d_b2, eq10_e1597_d_b3, eq10_e1597_d_b4, eq10_e1597_d_b5, eq10_e1597_d_b6, eq10_e1597_d_b7, eq10_e1597_d_b8, eq10_e1597_d_b9, eq10_e1597_d_b10, eq10_e1597_d_b11, eq10_e1598_q, eq10_e1597_d_n0, eq10_e1597_d_n1, eq10_e1597_d_n2, eq10_e1597_d_n3, eq10_e1597_d_n4, eq10_e1597_d_n5, eq10_e1597_d_n6, eq10_e1597_d_n7, eq10_e1597_d_n8, eq10_e1597_d_n9, eq10_e1597_d_n10, eq10_e1597_d_n11, eq10_e1597_d_n12, eq10_e1597_d_n13, eq10_e1597_d_b0, eq10_e1597_d_b1, eq10_e1597_d_b2, eq10_e1597_d_b3, eq10_e1597_d_b4, eq10_e1597_d_b5, eq10_e1597_d_b6, eq10_e1597_d_b7, eq10_e1597_d_b8, eq10_e1597_d_b9, eq10_e1597_d_b10, eq10_e1597_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 14] = [eq10_e1600_q_d_n0, eq10_e1600_q_d_n1, eq10_e1600_q_d_n2, eq10_e1600_q_d_n3, eq10_e1600_q_d_n4, eq10_e1600_q_d_n5, eq10_e1600_q_d_n6, eq10_e1600_q_d_n7, eq10_e1600_q_d_n8, eq10_e1600_q_d_n9, eq10_e1600_q_d_n10, eq10_e1600_q_d_n11, eq10_e1600_q_d_n12, eq10_e1600_q_d_n13];
        let eq10_reactive_branch_derivatives: [f64; 12] = [eq10_e1600_q_d_b0, eq10_e1600_q_d_b1, eq10_e1600_q_d_b2, eq10_e1600_q_d_b3, eq10_e1600_q_d_b4, eq10_e1600_q_d_b5, eq10_e1600_q_d_b6, eq10_e1600_q_d_b7, eq10_e1600_q_d_b8, eq10_e1600_q_d_b9, eq10_e1600_q_d_b10, eq10_e1600_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq11_e1626, eq11_e1626_d_n0, eq11_e1626_d_n1, eq11_e1626_d_n2, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, eq11_e1626_d_n13, eq11_e1626_d_b0, eq11_e1626_d_b1, eq11_e1626_d_b2, eq11_e1626_d_b3, eq11_e1626_d_b4, eq11_e1626_d_b5, eq11_e1626_d_b6, eq11_e1626_d_b7, eq11_e1626_d_b8, eq11_e1626_d_b9, eq11_e1626_d_b10, eq11_e1626_d_b11, eq11_e1626_q, eq11_e1626_q_d_n0, eq11_e1626_q_d_n1, eq11_e1626_q_d_n2, eq11_e1626_q_d_n3, eq11_e1626_q_d_n4, eq11_e1626_q_d_n5, eq11_e1626_q_d_n6, eq11_e1626_q_d_n7, eq11_e1626_q_d_n8, eq11_e1626_q_d_n9, eq11_e1626_q_d_n10, eq11_e1626_q_d_n11, eq11_e1626_q_d_n12, eq11_e1626_q_d_n13, eq11_e1626_q_d_b0, eq11_e1626_q_d_b1, eq11_e1626_q_d_b2, eq11_e1626_q_d_b3, eq11_e1626_q_d_b4, eq11_e1626_q_d_b5, eq11_e1626_q_d_b6, eq11_e1626_q_d_b7, eq11_e1626_q_d_b8, eq11_e1626_q_d_b9, eq11_e1626_q_d_b10, eq11_e1626_q_d_b11,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq11_e1610: f64 = (1.0 - s.v[211]);
        let eq11_e1610_d_n0: f64 = (-s.dn[211][0]);
        let eq11_e1610_d_n1: f64 = (-s.dn[211][1]);
        let eq11_e1610_d_n2: f64 = (-s.dn[211][2]);
        let eq11_e1610_d_n3: f64 = (-s.dn[211][3]);
        let eq11_e1610_d_n4: f64 = (-s.dn[211][4]);
        let eq11_e1610_d_n5: f64 = (-s.dn[211][5]);
        let eq11_e1610_d_n6: f64 = (-s.dn[211][6]);
        let eq11_e1610_d_n7: f64 = (-s.dn[211][7]);
        let eq11_e1610_d_n8: f64 = (-s.dn[211][8]);
        let eq11_e1610_d_n9: f64 = (-s.dn[211][9]);
        let eq11_e1610_d_n10: f64 = (-s.dn[211][10]);
        let eq11_e1610_d_n11: f64 = (-s.dn[211][11]);
        let eq11_e1610_d_n12: f64 = (-s.dn[211][12]);
        let eq11_e1610_d_n13: f64 = (-s.dn[211][13]);
        let eq11_e1610_d_b0: f64 = (-s.db[211][0]);
        let eq11_e1610_d_b1: f64 = (-s.db[211][1]);
        let eq11_e1610_d_b2: f64 = (-s.db[211][2]);
        let eq11_e1610_d_b3: f64 = (-s.db[211][3]);
        let eq11_e1610_d_b4: f64 = (-s.db[211][4]);
        let eq11_e1610_d_b5: f64 = (-s.db[211][5]);
        let eq11_e1610_d_b6: f64 = (-s.db[211][6]);
        let eq11_e1610_d_b7: f64 = (-s.db[211][7]);
        let eq11_e1610_d_b8: f64 = (-s.db[211][8]);
        let eq11_e1610_d_b9: f64 = (-s.db[211][9]);
        let eq11_e1610_d_b10: f64 = (-s.db[211][10]);
        let eq11_e1610_d_b11: f64 = (-s.db[211][11]);
        let eq11_e1612: f64 = (eq11_e1610 * s.v[622]);
        let eq11_e1612_d_n0: f64 = ((eq11_e1610_d_n0 * s.v[622]) + (eq11_e1610 * s.dn[622][0]));
        let eq11_e1612_d_n1: f64 = ((eq11_e1610_d_n1 * s.v[622]) + (eq11_e1610 * s.dn[622][1]));
        let eq11_e1612_d_n2: f64 = ((eq11_e1610_d_n2 * s.v[622]) + (eq11_e1610 * s.dn[622][2]));
        let eq11_e1612_d_n3: f64 = ((eq11_e1610_d_n3 * s.v[622]) + (eq11_e1610 * s.dn[622][3]));
        let eq11_e1612_d_n4: f64 = ((eq11_e1610_d_n4 * s.v[622]) + (eq11_e1610 * s.dn[622][4]));
        let eq11_e1612_d_n5: f64 = ((eq11_e1610_d_n5 * s.v[622]) + (eq11_e1610 * s.dn[622][5]));
        let eq11_e1612_d_n6: f64 = ((eq11_e1610_d_n6 * s.v[622]) + (eq11_e1610 * s.dn[622][6]));
        let eq11_e1612_d_n7: f64 = ((eq11_e1610_d_n7 * s.v[622]) + (eq11_e1610 * s.dn[622][7]));
        let eq11_e1612_d_n8: f64 = ((eq11_e1610_d_n8 * s.v[622]) + (eq11_e1610 * s.dn[622][8]));
        let eq11_e1612_d_n9: f64 = ((eq11_e1610_d_n9 * s.v[622]) + (eq11_e1610 * s.dn[622][9]));
        let eq11_e1612_d_n10: f64 = ((eq11_e1610_d_n10 * s.v[622]) + (eq11_e1610 * s.dn[622][10]));
        let eq11_e1612_d_n11: f64 = ((eq11_e1610_d_n11 * s.v[622]) + (eq11_e1610 * s.dn[622][11]));
        let eq11_e1612_d_n12: f64 = ((eq11_e1610_d_n12 * s.v[622]) + (eq11_e1610 * s.dn[622][12]));
        let eq11_e1612_d_n13: f64 = ((eq11_e1610_d_n13 * s.v[622]) + (eq11_e1610 * s.dn[622][13]));
        let eq11_e1612_d_b0: f64 = ((eq11_e1610_d_b0 * s.v[622]) + (eq11_e1610 * s.db[622][0]));
        let eq11_e1612_d_b1: f64 = ((eq11_e1610_d_b1 * s.v[622]) + (eq11_e1610 * s.db[622][1]));
        let eq11_e1612_d_b2: f64 = ((eq11_e1610_d_b2 * s.v[622]) + (eq11_e1610 * s.db[622][2]));
        let eq11_e1612_d_b3: f64 = ((eq11_e1610_d_b3 * s.v[622]) + (eq11_e1610 * s.db[622][3]));
        let eq11_e1612_d_b4: f64 = ((eq11_e1610_d_b4 * s.v[622]) + (eq11_e1610 * s.db[622][4]));
        let eq11_e1612_d_b5: f64 = ((eq11_e1610_d_b5 * s.v[622]) + (eq11_e1610 * s.db[622][5]));
        let eq11_e1612_d_b6: f64 = ((eq11_e1610_d_b6 * s.v[622]) + (eq11_e1610 * s.db[622][6]));
        let eq11_e1612_d_b7: f64 = ((eq11_e1610_d_b7 * s.v[622]) + (eq11_e1610 * s.db[622][7]));
        let eq11_e1612_d_b8: f64 = ((eq11_e1610_d_b8 * s.v[622]) + (eq11_e1610 * s.db[622][8]));
        let eq11_e1612_d_b9: f64 = ((eq11_e1610_d_b9 * s.v[622]) + (eq11_e1610 * s.db[622][9]));
        let eq11_e1612_d_b10: f64 = ((eq11_e1610_d_b10 * s.v[622]) + (eq11_e1610 * s.db[622][10]));
        let eq11_e1612_d_b11: f64 = ((eq11_e1610_d_b11 * s.v[622]) + (eq11_e1610 * s.db[622][11]));
        let eq11_e1614: f64 = (eq11_e1612 * s.v[199]);
        let eq11_e1614_d_n0: f64 = (eq11_e1612_d_n0 * s.v[199]);
        let eq11_e1614_d_n1: f64 = (eq11_e1612_d_n1 * s.v[199]);
        let eq11_e1614_d_n2: f64 = (eq11_e1612_d_n2 * s.v[199]);
        let eq11_e1614_d_n3: f64 = (eq11_e1612_d_n3 * s.v[199]);
        let eq11_e1614_d_n4: f64 = (eq11_e1612_d_n4 * s.v[199]);
        let eq11_e1614_d_n5: f64 = (eq11_e1612_d_n5 * s.v[199]);
        let eq11_e1614_d_n6: f64 = (eq11_e1612_d_n6 * s.v[199]);
        let eq11_e1614_d_n7: f64 = (eq11_e1612_d_n7 * s.v[199]);
        let eq11_e1614_d_n8: f64 = (eq11_e1612_d_n8 * s.v[199]);
        let eq11_e1614_d_n9: f64 = (eq11_e1612_d_n9 * s.v[199]);
        let eq11_e1614_d_n10: f64 = (eq11_e1612_d_n10 * s.v[199]);
        let eq11_e1614_d_n11: f64 = (eq11_e1612_d_n11 * s.v[199]);
        let eq11_e1614_d_n12: f64 = (eq11_e1612_d_n12 * s.v[199]);
        let eq11_e1614_d_n13: f64 = (eq11_e1612_d_n13 * s.v[199]);
        let eq11_e1614_d_b0: f64 = (eq11_e1612_d_b0 * s.v[199]);
        let eq11_e1614_d_b1: f64 = (eq11_e1612_d_b1 * s.v[199]);
        let eq11_e1614_d_b2: f64 = (eq11_e1612_d_b2 * s.v[199]);
        let eq11_e1614_d_b3: f64 = (eq11_e1612_d_b3 * s.v[199]);
        let eq11_e1614_d_b4: f64 = (eq11_e1612_d_b4 * s.v[199]);
        let eq11_e1614_d_b5: f64 = (eq11_e1612_d_b5 * s.v[199]);
        let eq11_e1614_d_b6: f64 = (eq11_e1612_d_b6 * s.v[199]);
        let eq11_e1614_d_b7: f64 = (eq11_e1612_d_b7 * s.v[199]);
        let eq11_e1614_d_b8: f64 = (eq11_e1612_d_b8 * s.v[199]);
        let eq11_e1614_d_b9: f64 = (eq11_e1612_d_b9 * s.v[199]);
        let eq11_e1614_d_b10: f64 = (eq11_e1612_d_b10 * s.v[199]);
        let eq11_e1614_d_b11: f64 = (eq11_e1612_d_b11 * s.v[199]);
        let eq11_e1616: f64 = (eq11_e1614 * s.v[183]);
        let eq11_e1616_d_n0: f64 = (eq11_e1614_d_n0 * s.v[183]);
        let eq11_e1616_d_n1: f64 = (eq11_e1614_d_n1 * s.v[183]);
        let eq11_e1616_d_n2: f64 = (eq11_e1614_d_n2 * s.v[183]);
        let eq11_e1616_d_n3: f64 = (eq11_e1614_d_n3 * s.v[183]);
        let eq11_e1616_d_n4: f64 = (eq11_e1614_d_n4 * s.v[183]);
        let eq11_e1616_d_n5: f64 = (eq11_e1614_d_n5 * s.v[183]);
        let eq11_e1616_d_n6: f64 = (eq11_e1614_d_n6 * s.v[183]);
        let eq11_e1616_d_n7: f64 = (eq11_e1614_d_n7 * s.v[183]);
        let eq11_e1616_d_n8: f64 = (eq11_e1614_d_n8 * s.v[183]);
        let eq11_e1616_d_n9: f64 = (eq11_e1614_d_n9 * s.v[183]);
        let eq11_e1616_d_n10: f64 = (eq11_e1614_d_n10 * s.v[183]);
        let eq11_e1616_d_n11: f64 = (eq11_e1614_d_n11 * s.v[183]);
        let eq11_e1616_d_n12: f64 = (eq11_e1614_d_n12 * s.v[183]);
        let eq11_e1616_d_n13: f64 = (eq11_e1614_d_n13 * s.v[183]);
        let eq11_e1616_d_b0: f64 = (eq11_e1614_d_b0 * s.v[183]);
        let eq11_e1616_d_b1: f64 = (eq11_e1614_d_b1 * s.v[183]);
        let eq11_e1616_d_b2: f64 = (eq11_e1614_d_b2 * s.v[183]);
        let eq11_e1616_d_b3: f64 = (eq11_e1614_d_b3 * s.v[183]);
        let eq11_e1616_d_b4: f64 = (eq11_e1614_d_b4 * s.v[183]);
        let eq11_e1616_d_b5: f64 = (eq11_e1614_d_b5 * s.v[183]);
        let eq11_e1616_d_b6: f64 = (eq11_e1614_d_b6 * s.v[183]);
        let eq11_e1616_d_b7: f64 = (eq11_e1614_d_b7 * s.v[183]);
        let eq11_e1616_d_b8: f64 = (eq11_e1614_d_b8 * s.v[183]);
        let eq11_e1616_d_b9: f64 = (eq11_e1614_d_b9 * s.v[183]);
        let eq11_e1616_d_b10: f64 = (eq11_e1614_d_b10 * s.v[183]);
        let eq11_e1616_d_b11: f64 = (eq11_e1614_d_b11 * s.v[183]);
        let eq11_e1618: f64 = (eq11_e1616 * p.p2);
        let eq11_e1618_d_n0: f64 = (eq11_e1616_d_n0 * p.p2);
        let eq11_e1618_d_n1: f64 = (eq11_e1616_d_n1 * p.p2);
        let eq11_e1618_d_n2: f64 = (eq11_e1616_d_n2 * p.p2);
        let eq11_e1618_d_n3: f64 = (eq11_e1616_d_n3 * p.p2);
        let eq11_e1618_d_n4: f64 = (eq11_e1616_d_n4 * p.p2);
        let eq11_e1618_d_n5: f64 = (eq11_e1616_d_n5 * p.p2);
        let eq11_e1618_d_n6: f64 = (eq11_e1616_d_n6 * p.p2);
        let eq11_e1618_d_n7: f64 = (eq11_e1616_d_n7 * p.p2);
        let eq11_e1618_d_n8: f64 = (eq11_e1616_d_n8 * p.p2);
        let eq11_e1618_d_n9: f64 = (eq11_e1616_d_n9 * p.p2);
        let eq11_e1618_d_n10: f64 = (eq11_e1616_d_n10 * p.p2);
        let eq11_e1618_d_n11: f64 = (eq11_e1616_d_n11 * p.p2);
        let eq11_e1618_d_n12: f64 = (eq11_e1616_d_n12 * p.p2);
        let eq11_e1618_d_n13: f64 = (eq11_e1616_d_n13 * p.p2);
        let eq11_e1618_d_b0: f64 = (eq11_e1616_d_b0 * p.p2);
        let eq11_e1618_d_b1: f64 = (eq11_e1616_d_b1 * p.p2);
        let eq11_e1618_d_b2: f64 = (eq11_e1616_d_b2 * p.p2);
        let eq11_e1618_d_b3: f64 = (eq11_e1616_d_b3 * p.p2);
        let eq11_e1618_d_b4: f64 = (eq11_e1616_d_b4 * p.p2);
        let eq11_e1618_d_b5: f64 = (eq11_e1616_d_b5 * p.p2);
        let eq11_e1618_d_b6: f64 = (eq11_e1616_d_b6 * p.p2);
        let eq11_e1618_d_b7: f64 = (eq11_e1616_d_b7 * p.p2);
        let eq11_e1618_d_b8: f64 = (eq11_e1616_d_b8 * p.p2);
        let eq11_e1618_d_b9: f64 = (eq11_e1616_d_b9 * p.p2);
        let eq11_e1618_d_b10: f64 = (eq11_e1616_d_b10 * p.p2);
        let eq11_e1618_d_b11: f64 = (eq11_e1616_d_b11 * p.p2);
        let eq11_e1620: f64 = (eq11_e1618 * s.v[184]);
        let eq11_e1620_d_n0: f64 = (eq11_e1618_d_n0 * s.v[184]);
        let eq11_e1620_d_n1: f64 = (eq11_e1618_d_n1 * s.v[184]);
        let eq11_e1620_d_n2: f64 = (eq11_e1618_d_n2 * s.v[184]);
        let eq11_e1620_d_n3: f64 = (eq11_e1618_d_n3 * s.v[184]);
        let eq11_e1620_d_n4: f64 = (eq11_e1618_d_n4 * s.v[184]);
        let eq11_e1620_d_n5: f64 = (eq11_e1618_d_n5 * s.v[184]);
        let eq11_e1620_d_n6: f64 = (eq11_e1618_d_n6 * s.v[184]);
        let eq11_e1620_d_n7: f64 = (eq11_e1618_d_n7 * s.v[184]);
        let eq11_e1620_d_n8: f64 = (eq11_e1618_d_n8 * s.v[184]);
        let eq11_e1620_d_n9: f64 = (eq11_e1618_d_n9 * s.v[184]);
        let eq11_e1620_d_n10: f64 = (eq11_e1618_d_n10 * s.v[184]);
        let eq11_e1620_d_n11: f64 = (eq11_e1618_d_n11 * s.v[184]);
        let eq11_e1620_d_n12: f64 = (eq11_e1618_d_n12 * s.v[184]);
        let eq11_e1620_d_n13: f64 = (eq11_e1618_d_n13 * s.v[184]);
        let eq11_e1620_d_b0: f64 = (eq11_e1618_d_b0 * s.v[184]);
        let eq11_e1620_d_b1: f64 = (eq11_e1618_d_b1 * s.v[184]);
        let eq11_e1620_d_b2: f64 = (eq11_e1618_d_b2 * s.v[184]);
        let eq11_e1620_d_b3: f64 = (eq11_e1618_d_b3 * s.v[184]);
        let eq11_e1620_d_b4: f64 = (eq11_e1618_d_b4 * s.v[184]);
        let eq11_e1620_d_b5: f64 = (eq11_e1618_d_b5 * s.v[184]);
        let eq11_e1620_d_b6: f64 = (eq11_e1618_d_b6 * s.v[184]);
        let eq11_e1620_d_b7: f64 = (eq11_e1618_d_b7 * s.v[184]);
        let eq11_e1620_d_b8: f64 = (eq11_e1618_d_b8 * s.v[184]);
        let eq11_e1620_d_b9: f64 = (eq11_e1618_d_b9 * s.v[184]);
        let eq11_e1620_d_b10: f64 = (eq11_e1618_d_b10 * s.v[184]);
        let eq11_e1620_d_b11: f64 = (eq11_e1618_d_b11 * s.v[184]);
        let eq11_e1622: f64 = (eq11_e1620 * (nv12 - 0.0));
        let eq11_e1622_d_n0: f64 = (eq11_e1620_d_n0 * (nv12 - 0.0));
        let eq11_e1622_d_n1: f64 = (eq11_e1620_d_n1 * (nv12 - 0.0));
        let eq11_e1622_d_n2: f64 = (eq11_e1620_d_n2 * (nv12 - 0.0));
        let eq11_e1622_d_n3: f64 = (eq11_e1620_d_n3 * (nv12 - 0.0));
        let eq11_e1622_d_n4: f64 = (eq11_e1620_d_n4 * (nv12 - 0.0));
        let eq11_e1622_d_n5: f64 = (eq11_e1620_d_n5 * (nv12 - 0.0));
        let eq11_e1622_d_n6: f64 = (eq11_e1620_d_n6 * (nv12 - 0.0));
        let eq11_e1622_d_n7: f64 = (eq11_e1620_d_n7 * (nv12 - 0.0));
        let eq11_e1622_d_n8: f64 = (eq11_e1620_d_n8 * (nv12 - 0.0));
        let eq11_e1622_d_n9: f64 = (eq11_e1620_d_n9 * (nv12 - 0.0));
        let eq11_e1622_d_n10: f64 = (eq11_e1620_d_n10 * (nv12 - 0.0));
        let eq11_e1622_d_n11: f64 = (eq11_e1620_d_n11 * (nv12 - 0.0));
        let eq11_e1622_d_n12: f64 = ((eq11_e1620_d_n12 * (nv12 - 0.0)) + eq11_e1620);
        let eq11_e1622_d_n13: f64 = (eq11_e1620_d_n13 * (nv12 - 0.0));
        let eq11_e1622_d_b0: f64 = (eq11_e1620_d_b0 * (nv12 - 0.0));
        let eq11_e1622_d_b1: f64 = (eq11_e1620_d_b1 * (nv12 - 0.0));
        let eq11_e1622_d_b2: f64 = (eq11_e1620_d_b2 * (nv12 - 0.0));
        let eq11_e1622_d_b3: f64 = (eq11_e1620_d_b3 * (nv12 - 0.0));
        let eq11_e1622_d_b4: f64 = (eq11_e1620_d_b4 * (nv12 - 0.0));
        let eq11_e1622_d_b5: f64 = (eq11_e1620_d_b5 * (nv12 - 0.0));
        let eq11_e1622_d_b6: f64 = (eq11_e1620_d_b6 * (nv12 - 0.0));
        let eq11_e1622_d_b7: f64 = (eq11_e1620_d_b7 * (nv12 - 0.0));
        let eq11_e1622_d_b8: f64 = (eq11_e1620_d_b8 * (nv12 - 0.0));
        let eq11_e1622_d_b9: f64 = (eq11_e1620_d_b9 * (nv12 - 0.0));
        let eq11_e1622_d_b10: f64 = (eq11_e1620_d_b10 * (nv12 - 0.0));
        let eq11_e1622_d_b11: f64 = (eq11_e1620_d_b11 * (nv12 - 0.0));
        let eq11_e1623: f64 = (0.5 * eq11_e1622);
        let eq11_e1623_d_n0: f64 = (0.5 * eq11_e1622_d_n0);
        let eq11_e1623_d_n1: f64 = (0.5 * eq11_e1622_d_n1);
        let eq11_e1623_d_n2: f64 = (0.5 * eq11_e1622_d_n2);
        let eq11_e1623_d_n3: f64 = (0.5 * eq11_e1622_d_n3);
        let eq11_e1623_d_n4: f64 = (0.5 * eq11_e1622_d_n4);
        let eq11_e1623_d_n5: f64 = (0.5 * eq11_e1622_d_n5);
        let eq11_e1623_d_n6: f64 = (0.5 * eq11_e1622_d_n6);
        let eq11_e1623_d_n7: f64 = (0.5 * eq11_e1622_d_n7);
        let eq11_e1623_d_n8: f64 = (0.5 * eq11_e1622_d_n8);
        let eq11_e1623_d_n9: f64 = (0.5 * eq11_e1622_d_n9);
        let eq11_e1623_d_n10: f64 = (0.5 * eq11_e1622_d_n10);
        let eq11_e1623_d_n11: f64 = (0.5 * eq11_e1622_d_n11);
        let eq11_e1623_d_n12: f64 = (0.5 * eq11_e1622_d_n12);
        let eq11_e1623_d_n13: f64 = (0.5 * eq11_e1622_d_n13);
        let eq11_e1623_d_b0: f64 = (0.5 * eq11_e1622_d_b0);
        let eq11_e1623_d_b1: f64 = (0.5 * eq11_e1622_d_b1);
        let eq11_e1623_d_b2: f64 = (0.5 * eq11_e1622_d_b2);
        let eq11_e1623_d_b3: f64 = (0.5 * eq11_e1622_d_b3);
        let eq11_e1623_d_b4: f64 = (0.5 * eq11_e1622_d_b4);
        let eq11_e1623_d_b5: f64 = (0.5 * eq11_e1622_d_b5);
        let eq11_e1623_d_b6: f64 = (0.5 * eq11_e1622_d_b6);
        let eq11_e1623_d_b7: f64 = (0.5 * eq11_e1622_d_b7);
        let eq11_e1623_d_b8: f64 = (0.5 * eq11_e1622_d_b8);
        let eq11_e1623_d_b9: f64 = (0.5 * eq11_e1622_d_b9);
        let eq11_e1623_d_b10: f64 = (0.5 * eq11_e1622_d_b10);
        let eq11_e1623_d_b11: f64 = (0.5 * eq11_e1622_d_b11);
        let eq11_e1624_q: f64 = eq11_e1623;
        (eq11_e1623, eq11_e1623_d_n0, eq11_e1623_d_n1, eq11_e1623_d_n2, eq11_e1623_d_n3, eq11_e1623_d_n4, eq11_e1623_d_n5, eq11_e1623_d_n6, eq11_e1623_d_n7, eq11_e1623_d_n8, eq11_e1623_d_n9, eq11_e1623_d_n10, eq11_e1623_d_n11, eq11_e1623_d_n12, eq11_e1623_d_n13, eq11_e1623_d_b0, eq11_e1623_d_b1, eq11_e1623_d_b2, eq11_e1623_d_b3, eq11_e1623_d_b4, eq11_e1623_d_b5, eq11_e1623_d_b6, eq11_e1623_d_b7, eq11_e1623_d_b8, eq11_e1623_d_b9, eq11_e1623_d_b10, eq11_e1623_d_b11, eq11_e1624_q, eq11_e1623_d_n0, eq11_e1623_d_n1, eq11_e1623_d_n2, eq11_e1623_d_n3, eq11_e1623_d_n4, eq11_e1623_d_n5, eq11_e1623_d_n6, eq11_e1623_d_n7, eq11_e1623_d_n8, eq11_e1623_d_n9, eq11_e1623_d_n10, eq11_e1623_d_n11, eq11_e1623_d_n12, eq11_e1623_d_n13, eq11_e1623_d_b0, eq11_e1623_d_b1, eq11_e1623_d_b2, eq11_e1623_d_b3, eq11_e1623_d_b4, eq11_e1623_d_b5, eq11_e1623_d_b6, eq11_e1623_d_b7, eq11_e1623_d_b8, eq11_e1623_d_b9, eq11_e1623_d_b10, eq11_e1623_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 14] = [eq11_e1626_q_d_n0, eq11_e1626_q_d_n1, eq11_e1626_q_d_n2, eq11_e1626_q_d_n3, eq11_e1626_q_d_n4, eq11_e1626_q_d_n5, eq11_e1626_q_d_n6, eq11_e1626_q_d_n7, eq11_e1626_q_d_n8, eq11_e1626_q_d_n9, eq11_e1626_q_d_n10, eq11_e1626_q_d_n11, eq11_e1626_q_d_n12, eq11_e1626_q_d_n13];
        let eq11_reactive_branch_derivatives: [f64; 12] = [eq11_e1626_q_d_b0, eq11_e1626_q_d_b1, eq11_e1626_q_d_b2, eq11_e1626_q_d_b3, eq11_e1626_q_d_b4, eq11_e1626_q_d_b5, eq11_e1626_q_d_b6, eq11_e1626_q_d_b7, eq11_e1626_q_d_b8, eq11_e1626_q_d_b9, eq11_e1626_q_d_b10, eq11_e1626_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1784, eq24_e1784_d_n0, eq24_e1784_d_n1, eq24_e1784_d_n2, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, eq24_e1784_d_n13, eq24_e1784_d_b0, eq24_e1784_d_b1, eq24_e1784_d_b2, eq24_e1784_d_b3, eq24_e1784_d_b4, eq24_e1784_d_b5, eq24_e1784_d_b6, eq24_e1784_d_b7, eq24_e1784_d_b8, eq24_e1784_d_b9, eq24_e1784_d_b10, eq24_e1784_d_b11, eq24_e1784_q, eq24_e1784_q_d_n0, eq24_e1784_q_d_n1, eq24_e1784_q_d_n2, eq24_e1784_q_d_n3, eq24_e1784_q_d_n4, eq24_e1784_q_d_n5, eq24_e1784_q_d_n6, eq24_e1784_q_d_n7, eq24_e1784_q_d_n8, eq24_e1784_q_d_n9, eq24_e1784_q_d_n10, eq24_e1784_q_d_n11, eq24_e1784_q_d_n12, eq24_e1784_q_d_n13, eq24_e1784_q_d_b0, eq24_e1784_q_d_b1, eq24_e1784_q_d_b2, eq24_e1784_q_d_b3, eq24_e1784_q_d_b4, eq24_e1784_q_d_b5, eq24_e1784_q_d_b6, eq24_e1784_q_d_b7, eq24_e1784_q_d_b8, eq24_e1784_q_d_b9, eq24_e1784_q_d_b10, eq24_e1784_q_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq24_e1773: f64 = (s.v[622] * s.v[199]);
        let eq24_e1773_d_n0: f64 = (s.dn[622][0] * s.v[199]);
        let eq24_e1773_d_n1: f64 = (s.dn[622][1] * s.v[199]);
        let eq24_e1773_d_n2: f64 = (s.dn[622][2] * s.v[199]);
        let eq24_e1773_d_n3: f64 = (s.dn[622][3] * s.v[199]);
        let eq24_e1773_d_n4: f64 = (s.dn[622][4] * s.v[199]);
        let eq24_e1773_d_n5: f64 = (s.dn[622][5] * s.v[199]);
        let eq24_e1773_d_n6: f64 = (s.dn[622][6] * s.v[199]);
        let eq24_e1773_d_n7: f64 = (s.dn[622][7] * s.v[199]);
        let eq24_e1773_d_n8: f64 = (s.dn[622][8] * s.v[199]);
        let eq24_e1773_d_n9: f64 = (s.dn[622][9] * s.v[199]);
        let eq24_e1773_d_n10: f64 = (s.dn[622][10] * s.v[199]);
        let eq24_e1773_d_n11: f64 = (s.dn[622][11] * s.v[199]);
        let eq24_e1773_d_n12: f64 = (s.dn[622][12] * s.v[199]);
        let eq24_e1773_d_n13: f64 = (s.dn[622][13] * s.v[199]);
        let eq24_e1773_d_b0: f64 = (s.db[622][0] * s.v[199]);
        let eq24_e1773_d_b1: f64 = (s.db[622][1] * s.v[199]);
        let eq24_e1773_d_b2: f64 = (s.db[622][2] * s.v[199]);
        let eq24_e1773_d_b3: f64 = (s.db[622][3] * s.v[199]);
        let eq24_e1773_d_b4: f64 = (s.db[622][4] * s.v[199]);
        let eq24_e1773_d_b5: f64 = (s.db[622][5] * s.v[199]);
        let eq24_e1773_d_b6: f64 = (s.db[622][6] * s.v[199]);
        let eq24_e1773_d_b7: f64 = (s.db[622][7] * s.v[199]);
        let eq24_e1773_d_b8: f64 = (s.db[622][8] * s.v[199]);
        let eq24_e1773_d_b9: f64 = (s.db[622][9] * s.v[199]);
        let eq24_e1773_d_b10: f64 = (s.db[622][10] * s.v[199]);
        let eq24_e1773_d_b11: f64 = (s.db[622][11] * s.v[199]);
        let eq24_e1775: f64 = (eq24_e1773 * s.v[183]);
        let eq24_e1775_d_n0: f64 = (eq24_e1773_d_n0 * s.v[183]);
        let eq24_e1775_d_n1: f64 = (eq24_e1773_d_n1 * s.v[183]);
        let eq24_e1775_d_n2: f64 = (eq24_e1773_d_n2 * s.v[183]);
        let eq24_e1775_d_n3: f64 = (eq24_e1773_d_n3 * s.v[183]);
        let eq24_e1775_d_n4: f64 = (eq24_e1773_d_n4 * s.v[183]);
        let eq24_e1775_d_n5: f64 = (eq24_e1773_d_n5 * s.v[183]);
        let eq24_e1775_d_n6: f64 = (eq24_e1773_d_n6 * s.v[183]);
        let eq24_e1775_d_n7: f64 = (eq24_e1773_d_n7 * s.v[183]);
        let eq24_e1775_d_n8: f64 = (eq24_e1773_d_n8 * s.v[183]);
        let eq24_e1775_d_n9: f64 = (eq24_e1773_d_n9 * s.v[183]);
        let eq24_e1775_d_n10: f64 = (eq24_e1773_d_n10 * s.v[183]);
        let eq24_e1775_d_n11: f64 = (eq24_e1773_d_n11 * s.v[183]);
        let eq24_e1775_d_n12: f64 = (eq24_e1773_d_n12 * s.v[183]);
        let eq24_e1775_d_n13: f64 = (eq24_e1773_d_n13 * s.v[183]);
        let eq24_e1775_d_b0: f64 = (eq24_e1773_d_b0 * s.v[183]);
        let eq24_e1775_d_b1: f64 = (eq24_e1773_d_b1 * s.v[183]);
        let eq24_e1775_d_b2: f64 = (eq24_e1773_d_b2 * s.v[183]);
        let eq24_e1775_d_b3: f64 = (eq24_e1773_d_b3 * s.v[183]);
        let eq24_e1775_d_b4: f64 = (eq24_e1773_d_b4 * s.v[183]);
        let eq24_e1775_d_b5: f64 = (eq24_e1773_d_b5 * s.v[183]);
        let eq24_e1775_d_b6: f64 = (eq24_e1773_d_b6 * s.v[183]);
        let eq24_e1775_d_b7: f64 = (eq24_e1773_d_b7 * s.v[183]);
        let eq24_e1775_d_b8: f64 = (eq24_e1773_d_b8 * s.v[183]);
        let eq24_e1775_d_b9: f64 = (eq24_e1773_d_b9 * s.v[183]);
        let eq24_e1775_d_b10: f64 = (eq24_e1773_d_b10 * s.v[183]);
        let eq24_e1775_d_b11: f64 = (eq24_e1773_d_b11 * s.v[183]);
        let eq24_e1777: f64 = (eq24_e1775 * p.p2);
        let eq24_e1777_d_n0: f64 = (eq24_e1775_d_n0 * p.p2);
        let eq24_e1777_d_n1: f64 = (eq24_e1775_d_n1 * p.p2);
        let eq24_e1777_d_n2: f64 = (eq24_e1775_d_n2 * p.p2);
        let eq24_e1777_d_n3: f64 = (eq24_e1775_d_n3 * p.p2);
        let eq24_e1777_d_n4: f64 = (eq24_e1775_d_n4 * p.p2);
        let eq24_e1777_d_n5: f64 = (eq24_e1775_d_n5 * p.p2);
        let eq24_e1777_d_n6: f64 = (eq24_e1775_d_n6 * p.p2);
        let eq24_e1777_d_n7: f64 = (eq24_e1775_d_n7 * p.p2);
        let eq24_e1777_d_n8: f64 = (eq24_e1775_d_n8 * p.p2);
        let eq24_e1777_d_n9: f64 = (eq24_e1775_d_n9 * p.p2);
        let eq24_e1777_d_n10: f64 = (eq24_e1775_d_n10 * p.p2);
        let eq24_e1777_d_n11: f64 = (eq24_e1775_d_n11 * p.p2);
        let eq24_e1777_d_n12: f64 = (eq24_e1775_d_n12 * p.p2);
        let eq24_e1777_d_n13: f64 = (eq24_e1775_d_n13 * p.p2);
        let eq24_e1777_d_b0: f64 = (eq24_e1775_d_b0 * p.p2);
        let eq24_e1777_d_b1: f64 = (eq24_e1775_d_b1 * p.p2);
        let eq24_e1777_d_b2: f64 = (eq24_e1775_d_b2 * p.p2);
        let eq24_e1777_d_b3: f64 = (eq24_e1775_d_b3 * p.p2);
        let eq24_e1777_d_b4: f64 = (eq24_e1775_d_b4 * p.p2);
        let eq24_e1777_d_b5: f64 = (eq24_e1775_d_b5 * p.p2);
        let eq24_e1777_d_b6: f64 = (eq24_e1775_d_b6 * p.p2);
        let eq24_e1777_d_b7: f64 = (eq24_e1775_d_b7 * p.p2);
        let eq24_e1777_d_b8: f64 = (eq24_e1775_d_b8 * p.p2);
        let eq24_e1777_d_b9: f64 = (eq24_e1775_d_b9 * p.p2);
        let eq24_e1777_d_b10: f64 = (eq24_e1775_d_b10 * p.p2);
        let eq24_e1777_d_b11: f64 = (eq24_e1775_d_b11 * p.p2);
        let eq24_e1779: f64 = (eq24_e1777 * s.v[184]);
        let eq24_e1779_d_n0: f64 = (eq24_e1777_d_n0 * s.v[184]);
        let eq24_e1779_d_n1: f64 = (eq24_e1777_d_n1 * s.v[184]);
        let eq24_e1779_d_n2: f64 = (eq24_e1777_d_n2 * s.v[184]);
        let eq24_e1779_d_n3: f64 = (eq24_e1777_d_n3 * s.v[184]);
        let eq24_e1779_d_n4: f64 = (eq24_e1777_d_n4 * s.v[184]);
        let eq24_e1779_d_n5: f64 = (eq24_e1777_d_n5 * s.v[184]);
        let eq24_e1779_d_n6: f64 = (eq24_e1777_d_n6 * s.v[184]);
        let eq24_e1779_d_n7: f64 = (eq24_e1777_d_n7 * s.v[184]);
        let eq24_e1779_d_n8: f64 = (eq24_e1777_d_n8 * s.v[184]);
        let eq24_e1779_d_n9: f64 = (eq24_e1777_d_n9 * s.v[184]);
        let eq24_e1779_d_n10: f64 = (eq24_e1777_d_n10 * s.v[184]);
        let eq24_e1779_d_n11: f64 = (eq24_e1777_d_n11 * s.v[184]);
        let eq24_e1779_d_n12: f64 = (eq24_e1777_d_n12 * s.v[184]);
        let eq24_e1779_d_n13: f64 = (eq24_e1777_d_n13 * s.v[184]);
        let eq24_e1779_d_b0: f64 = (eq24_e1777_d_b0 * s.v[184]);
        let eq24_e1779_d_b1: f64 = (eq24_e1777_d_b1 * s.v[184]);
        let eq24_e1779_d_b2: f64 = (eq24_e1777_d_b2 * s.v[184]);
        let eq24_e1779_d_b3: f64 = (eq24_e1777_d_b3 * s.v[184]);
        let eq24_e1779_d_b4: f64 = (eq24_e1777_d_b4 * s.v[184]);
        let eq24_e1779_d_b5: f64 = (eq24_e1777_d_b5 * s.v[184]);
        let eq24_e1779_d_b6: f64 = (eq24_e1777_d_b6 * s.v[184]);
        let eq24_e1779_d_b7: f64 = (eq24_e1777_d_b7 * s.v[184]);
        let eq24_e1779_d_b8: f64 = (eq24_e1777_d_b8 * s.v[184]);
        let eq24_e1779_d_b9: f64 = (eq24_e1777_d_b9 * s.v[184]);
        let eq24_e1779_d_b10: f64 = (eq24_e1777_d_b10 * s.v[184]);
        let eq24_e1779_d_b11: f64 = (eq24_e1777_d_b11 * s.v[184]);
        let eq24_e1781: f64 = (eq24_e1779 * (nv12 - 0.0));
        let eq24_e1781_d_n0: f64 = (eq24_e1779_d_n0 * (nv12 - 0.0));
        let eq24_e1781_d_n1: f64 = (eq24_e1779_d_n1 * (nv12 - 0.0));
        let eq24_e1781_d_n2: f64 = (eq24_e1779_d_n2 * (nv12 - 0.0));
        let eq24_e1781_d_n3: f64 = (eq24_e1779_d_n3 * (nv12 - 0.0));
        let eq24_e1781_d_n4: f64 = (eq24_e1779_d_n4 * (nv12 - 0.0));
        let eq24_e1781_d_n5: f64 = (eq24_e1779_d_n5 * (nv12 - 0.0));
        let eq24_e1781_d_n6: f64 = (eq24_e1779_d_n6 * (nv12 - 0.0));
        let eq24_e1781_d_n7: f64 = (eq24_e1779_d_n7 * (nv12 - 0.0));
        let eq24_e1781_d_n8: f64 = (eq24_e1779_d_n8 * (nv12 - 0.0));
        let eq24_e1781_d_n9: f64 = (eq24_e1779_d_n9 * (nv12 - 0.0));
        let eq24_e1781_d_n10: f64 = (eq24_e1779_d_n10 * (nv12 - 0.0));
        let eq24_e1781_d_n11: f64 = (eq24_e1779_d_n11 * (nv12 - 0.0));
        let eq24_e1781_d_n12: f64 = ((eq24_e1779_d_n12 * (nv12 - 0.0)) + eq24_e1779);
        let eq24_e1781_d_n13: f64 = (eq24_e1779_d_n13 * (nv12 - 0.0));
        let eq24_e1781_d_b0: f64 = (eq24_e1779_d_b0 * (nv12 - 0.0));
        let eq24_e1781_d_b1: f64 = (eq24_e1779_d_b1 * (nv12 - 0.0));
        let eq24_e1781_d_b2: f64 = (eq24_e1779_d_b2 * (nv12 - 0.0));
        let eq24_e1781_d_b3: f64 = (eq24_e1779_d_b3 * (nv12 - 0.0));
        let eq24_e1781_d_b4: f64 = (eq24_e1779_d_b4 * (nv12 - 0.0));
        let eq24_e1781_d_b5: f64 = (eq24_e1779_d_b5 * (nv12 - 0.0));
        let eq24_e1781_d_b6: f64 = (eq24_e1779_d_b6 * (nv12 - 0.0));
        let eq24_e1781_d_b7: f64 = (eq24_e1779_d_b7 * (nv12 - 0.0));
        let eq24_e1781_d_b8: f64 = (eq24_e1779_d_b8 * (nv12 - 0.0));
        let eq24_e1781_d_b9: f64 = (eq24_e1779_d_b9 * (nv12 - 0.0));
        let eq24_e1781_d_b10: f64 = (eq24_e1779_d_b10 * (nv12 - 0.0));
        let eq24_e1781_d_b11: f64 = (eq24_e1779_d_b11 * (nv12 - 0.0));
        let eq24_e1782_q: f64 = eq24_e1781;
        (eq24_e1781, eq24_e1781_d_n0, eq24_e1781_d_n1, eq24_e1781_d_n2, eq24_e1781_d_n3, eq24_e1781_d_n4, eq24_e1781_d_n5, eq24_e1781_d_n6, eq24_e1781_d_n7, eq24_e1781_d_n8, eq24_e1781_d_n9, eq24_e1781_d_n10, eq24_e1781_d_n11, eq24_e1781_d_n12, eq24_e1781_d_n13, eq24_e1781_d_b0, eq24_e1781_d_b1, eq24_e1781_d_b2, eq24_e1781_d_b3, eq24_e1781_d_b4, eq24_e1781_d_b5, eq24_e1781_d_b6, eq24_e1781_d_b7, eq24_e1781_d_b8, eq24_e1781_d_b9, eq24_e1781_d_b10, eq24_e1781_d_b11, eq24_e1782_q, eq24_e1781_d_n0, eq24_e1781_d_n1, eq24_e1781_d_n2, eq24_e1781_d_n3, eq24_e1781_d_n4, eq24_e1781_d_n5, eq24_e1781_d_n6, eq24_e1781_d_n7, eq24_e1781_d_n8, eq24_e1781_d_n9, eq24_e1781_d_n10, eq24_e1781_d_n11, eq24_e1781_d_n12, eq24_e1781_d_n13, eq24_e1781_d_b0, eq24_e1781_d_b1, eq24_e1781_d_b2, eq24_e1781_d_b3, eq24_e1781_d_b4, eq24_e1781_d_b5, eq24_e1781_d_b6, eq24_e1781_d_b7, eq24_e1781_d_b8, eq24_e1781_d_b9, eq24_e1781_d_b10, eq24_e1781_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_reactive_node_derivatives: [f64; 14] = [eq24_e1784_q_d_n0, eq24_e1784_q_d_n1, eq24_e1784_q_d_n2, eq24_e1784_q_d_n3, eq24_e1784_q_d_n4, eq24_e1784_q_d_n5, eq24_e1784_q_d_n6, eq24_e1784_q_d_n7, eq24_e1784_q_d_n8, eq24_e1784_q_d_n9, eq24_e1784_q_d_n10, eq24_e1784_q_d_n11, eq24_e1784_q_d_n12, eq24_e1784_q_d_n13];
        let eq24_reactive_branch_derivatives: [f64; 12] = [eq24_e1784_q_d_b0, eq24_e1784_q_d_b1, eq24_e1784_q_d_b2, eq24_e1784_q_d_b3, eq24_e1784_q_d_b4, eq24_e1784_q_d_b5, eq24_e1784_q_d_b6, eq24_e1784_q_d_b7, eq24_e1784_q_d_b8, eq24_e1784_q_d_b9, eq24_e1784_q_d_b10, eq24_e1784_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq27_e1841, eq27_e1841_d_n0, eq27_e1841_d_n1, eq27_e1841_d_n2, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_d_n13, eq27_e1841_d_b0, eq27_e1841_d_b1, eq27_e1841_d_b2, eq27_e1841_d_b3, eq27_e1841_d_b4, eq27_e1841_d_b5, eq27_e1841_d_b6, eq27_e1841_d_b7, eq27_e1841_d_b8, eq27_e1841_d_b9, eq27_e1841_d_b10, eq27_e1841_d_b11, eq27_e1841_q, eq27_e1841_q_d_n0, eq27_e1841_q_d_n1, eq27_e1841_q_d_n2, eq27_e1841_q_d_n3, eq27_e1841_q_d_n4, eq27_e1841_q_d_n5, eq27_e1841_q_d_n6, eq27_e1841_q_d_n7, eq27_e1841_q_d_n8, eq27_e1841_q_d_n9, eq27_e1841_q_d_n10, eq27_e1841_q_d_n11, eq27_e1841_q_d_n12, eq27_e1841_q_d_n13, eq27_e1841_q_d_b0, eq27_e1841_q_d_b1, eq27_e1841_q_d_b2, eq27_e1841_q_d_b3, eq27_e1841_q_d_b4, eq27_e1841_q_d_b5, eq27_e1841_q_d_b6, eq27_e1841_q_d_b7, eq27_e1841_q_d_b8, eq27_e1841_q_d_b9, eq27_e1841_q_d_b10, eq27_e1841_q_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq27_e1825: f64 = (1.0 + s.v[211]);
        let eq27_e1827: f64 = (eq27_e1825 * s.v[622]);
        let eq27_e1827_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq27_e1825 * s.dn[622][0]));
        let eq27_e1827_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq27_e1825 * s.dn[622][1]));
        let eq27_e1827_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq27_e1825 * s.dn[622][2]));
        let eq27_e1827_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq27_e1825 * s.dn[622][3]));
        let eq27_e1827_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq27_e1825 * s.dn[622][4]));
        let eq27_e1827_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq27_e1825 * s.dn[622][5]));
        let eq27_e1827_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq27_e1825 * s.dn[622][6]));
        let eq27_e1827_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq27_e1825 * s.dn[622][7]));
        let eq27_e1827_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq27_e1825 * s.dn[622][8]));
        let eq27_e1827_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq27_e1825 * s.dn[622][9]));
        let eq27_e1827_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq27_e1825 * s.dn[622][10]));
        let eq27_e1827_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq27_e1825 * s.dn[622][11]));
        let eq27_e1827_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq27_e1825 * s.dn[622][12]));
        let eq27_e1827_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq27_e1825 * s.dn[622][13]));
        let eq27_e1827_d_b0: f64 = ((s.db[211][0] * s.v[622]) + (eq27_e1825 * s.db[622][0]));
        let eq27_e1827_d_b1: f64 = ((s.db[211][1] * s.v[622]) + (eq27_e1825 * s.db[622][1]));
        let eq27_e1827_d_b2: f64 = ((s.db[211][2] * s.v[622]) + (eq27_e1825 * s.db[622][2]));
        let eq27_e1827_d_b3: f64 = ((s.db[211][3] * s.v[622]) + (eq27_e1825 * s.db[622][3]));
        let eq27_e1827_d_b4: f64 = ((s.db[211][4] * s.v[622]) + (eq27_e1825 * s.db[622][4]));
        let eq27_e1827_d_b5: f64 = ((s.db[211][5] * s.v[622]) + (eq27_e1825 * s.db[622][5]));
        let eq27_e1827_d_b6: f64 = ((s.db[211][6] * s.v[622]) + (eq27_e1825 * s.db[622][6]));
        let eq27_e1827_d_b7: f64 = ((s.db[211][7] * s.v[622]) + (eq27_e1825 * s.db[622][7]));
        let eq27_e1827_d_b8: f64 = ((s.db[211][8] * s.v[622]) + (eq27_e1825 * s.db[622][8]));
        let eq27_e1827_d_b9: f64 = ((s.db[211][9] * s.v[622]) + (eq27_e1825 * s.db[622][9]));
        let eq27_e1827_d_b10: f64 = ((s.db[211][10] * s.v[622]) + (eq27_e1825 * s.db[622][10]));
        let eq27_e1827_d_b11: f64 = ((s.db[211][11] * s.v[622]) + (eq27_e1825 * s.db[622][11]));
        let eq27_e1829: f64 = (eq27_e1827 * s.v[199]);
        let eq27_e1829_d_n0: f64 = (eq27_e1827_d_n0 * s.v[199]);
        let eq27_e1829_d_n1: f64 = (eq27_e1827_d_n1 * s.v[199]);
        let eq27_e1829_d_n2: f64 = (eq27_e1827_d_n2 * s.v[199]);
        let eq27_e1829_d_n3: f64 = (eq27_e1827_d_n3 * s.v[199]);
        let eq27_e1829_d_n4: f64 = (eq27_e1827_d_n4 * s.v[199]);
        let eq27_e1829_d_n5: f64 = (eq27_e1827_d_n5 * s.v[199]);
        let eq27_e1829_d_n6: f64 = (eq27_e1827_d_n6 * s.v[199]);
        let eq27_e1829_d_n7: f64 = (eq27_e1827_d_n7 * s.v[199]);
        let eq27_e1829_d_n8: f64 = (eq27_e1827_d_n8 * s.v[199]);
        let eq27_e1829_d_n9: f64 = (eq27_e1827_d_n9 * s.v[199]);
        let eq27_e1829_d_n10: f64 = (eq27_e1827_d_n10 * s.v[199]);
        let eq27_e1829_d_n11: f64 = (eq27_e1827_d_n11 * s.v[199]);
        let eq27_e1829_d_n12: f64 = (eq27_e1827_d_n12 * s.v[199]);
        let eq27_e1829_d_n13: f64 = (eq27_e1827_d_n13 * s.v[199]);
        let eq27_e1829_d_b0: f64 = (eq27_e1827_d_b0 * s.v[199]);
        let eq27_e1829_d_b1: f64 = (eq27_e1827_d_b1 * s.v[199]);
        let eq27_e1829_d_b2: f64 = (eq27_e1827_d_b2 * s.v[199]);
        let eq27_e1829_d_b3: f64 = (eq27_e1827_d_b3 * s.v[199]);
        let eq27_e1829_d_b4: f64 = (eq27_e1827_d_b4 * s.v[199]);
        let eq27_e1829_d_b5: f64 = (eq27_e1827_d_b5 * s.v[199]);
        let eq27_e1829_d_b6: f64 = (eq27_e1827_d_b6 * s.v[199]);
        let eq27_e1829_d_b7: f64 = (eq27_e1827_d_b7 * s.v[199]);
        let eq27_e1829_d_b8: f64 = (eq27_e1827_d_b8 * s.v[199]);
        let eq27_e1829_d_b9: f64 = (eq27_e1827_d_b9 * s.v[199]);
        let eq27_e1829_d_b10: f64 = (eq27_e1827_d_b10 * s.v[199]);
        let eq27_e1829_d_b11: f64 = (eq27_e1827_d_b11 * s.v[199]);
        let eq27_e1831: f64 = (eq27_e1829 * s.v[183]);
        let eq27_e1831_d_n0: f64 = (eq27_e1829_d_n0 * s.v[183]);
        let eq27_e1831_d_n1: f64 = (eq27_e1829_d_n1 * s.v[183]);
        let eq27_e1831_d_n2: f64 = (eq27_e1829_d_n2 * s.v[183]);
        let eq27_e1831_d_n3: f64 = (eq27_e1829_d_n3 * s.v[183]);
        let eq27_e1831_d_n4: f64 = (eq27_e1829_d_n4 * s.v[183]);
        let eq27_e1831_d_n5: f64 = (eq27_e1829_d_n5 * s.v[183]);
        let eq27_e1831_d_n6: f64 = (eq27_e1829_d_n6 * s.v[183]);
        let eq27_e1831_d_n7: f64 = (eq27_e1829_d_n7 * s.v[183]);
        let eq27_e1831_d_n8: f64 = (eq27_e1829_d_n8 * s.v[183]);
        let eq27_e1831_d_n9: f64 = (eq27_e1829_d_n9 * s.v[183]);
        let eq27_e1831_d_n10: f64 = (eq27_e1829_d_n10 * s.v[183]);
        let eq27_e1831_d_n11: f64 = (eq27_e1829_d_n11 * s.v[183]);
        let eq27_e1831_d_n12: f64 = (eq27_e1829_d_n12 * s.v[183]);
        let eq27_e1831_d_n13: f64 = (eq27_e1829_d_n13 * s.v[183]);
        let eq27_e1831_d_b0: f64 = (eq27_e1829_d_b0 * s.v[183]);
        let eq27_e1831_d_b1: f64 = (eq27_e1829_d_b1 * s.v[183]);
        let eq27_e1831_d_b2: f64 = (eq27_e1829_d_b2 * s.v[183]);
        let eq27_e1831_d_b3: f64 = (eq27_e1829_d_b3 * s.v[183]);
        let eq27_e1831_d_b4: f64 = (eq27_e1829_d_b4 * s.v[183]);
        let eq27_e1831_d_b5: f64 = (eq27_e1829_d_b5 * s.v[183]);
        let eq27_e1831_d_b6: f64 = (eq27_e1829_d_b6 * s.v[183]);
        let eq27_e1831_d_b7: f64 = (eq27_e1829_d_b7 * s.v[183]);
        let eq27_e1831_d_b8: f64 = (eq27_e1829_d_b8 * s.v[183]);
        let eq27_e1831_d_b9: f64 = (eq27_e1829_d_b9 * s.v[183]);
        let eq27_e1831_d_b10: f64 = (eq27_e1829_d_b10 * s.v[183]);
        let eq27_e1831_d_b11: f64 = (eq27_e1829_d_b11 * s.v[183]);
        let eq27_e1833: f64 = (eq27_e1831 * p.p2);
        let eq27_e1833_d_n0: f64 = (eq27_e1831_d_n0 * p.p2);
        let eq27_e1833_d_n1: f64 = (eq27_e1831_d_n1 * p.p2);
        let eq27_e1833_d_n2: f64 = (eq27_e1831_d_n2 * p.p2);
        let eq27_e1833_d_n3: f64 = (eq27_e1831_d_n3 * p.p2);
        let eq27_e1833_d_n4: f64 = (eq27_e1831_d_n4 * p.p2);
        let eq27_e1833_d_n5: f64 = (eq27_e1831_d_n5 * p.p2);
        let eq27_e1833_d_n6: f64 = (eq27_e1831_d_n6 * p.p2);
        let eq27_e1833_d_n7: f64 = (eq27_e1831_d_n7 * p.p2);
        let eq27_e1833_d_n8: f64 = (eq27_e1831_d_n8 * p.p2);
        let eq27_e1833_d_n9: f64 = (eq27_e1831_d_n9 * p.p2);
        let eq27_e1833_d_n10: f64 = (eq27_e1831_d_n10 * p.p2);
        let eq27_e1833_d_n11: f64 = (eq27_e1831_d_n11 * p.p2);
        let eq27_e1833_d_n12: f64 = (eq27_e1831_d_n12 * p.p2);
        let eq27_e1833_d_n13: f64 = (eq27_e1831_d_n13 * p.p2);
        let eq27_e1833_d_b0: f64 = (eq27_e1831_d_b0 * p.p2);
        let eq27_e1833_d_b1: f64 = (eq27_e1831_d_b1 * p.p2);
        let eq27_e1833_d_b2: f64 = (eq27_e1831_d_b2 * p.p2);
        let eq27_e1833_d_b3: f64 = (eq27_e1831_d_b3 * p.p2);
        let eq27_e1833_d_b4: f64 = (eq27_e1831_d_b4 * p.p2);
        let eq27_e1833_d_b5: f64 = (eq27_e1831_d_b5 * p.p2);
        let eq27_e1833_d_b6: f64 = (eq27_e1831_d_b6 * p.p2);
        let eq27_e1833_d_b7: f64 = (eq27_e1831_d_b7 * p.p2);
        let eq27_e1833_d_b8: f64 = (eq27_e1831_d_b8 * p.p2);
        let eq27_e1833_d_b9: f64 = (eq27_e1831_d_b9 * p.p2);
        let eq27_e1833_d_b10: f64 = (eq27_e1831_d_b10 * p.p2);
        let eq27_e1833_d_b11: f64 = (eq27_e1831_d_b11 * p.p2);
        let eq27_e1835: f64 = (eq27_e1833 * s.v[184]);
        let eq27_e1835_d_n0: f64 = (eq27_e1833_d_n0 * s.v[184]);
        let eq27_e1835_d_n1: f64 = (eq27_e1833_d_n1 * s.v[184]);
        let eq27_e1835_d_n2: f64 = (eq27_e1833_d_n2 * s.v[184]);
        let eq27_e1835_d_n3: f64 = (eq27_e1833_d_n3 * s.v[184]);
        let eq27_e1835_d_n4: f64 = (eq27_e1833_d_n4 * s.v[184]);
        let eq27_e1835_d_n5: f64 = (eq27_e1833_d_n5 * s.v[184]);
        let eq27_e1835_d_n6: f64 = (eq27_e1833_d_n6 * s.v[184]);
        let eq27_e1835_d_n7: f64 = (eq27_e1833_d_n7 * s.v[184]);
        let eq27_e1835_d_n8: f64 = (eq27_e1833_d_n8 * s.v[184]);
        let eq27_e1835_d_n9: f64 = (eq27_e1833_d_n9 * s.v[184]);
        let eq27_e1835_d_n10: f64 = (eq27_e1833_d_n10 * s.v[184]);
        let eq27_e1835_d_n11: f64 = (eq27_e1833_d_n11 * s.v[184]);
        let eq27_e1835_d_n12: f64 = (eq27_e1833_d_n12 * s.v[184]);
        let eq27_e1835_d_n13: f64 = (eq27_e1833_d_n13 * s.v[184]);
        let eq27_e1835_d_b0: f64 = (eq27_e1833_d_b0 * s.v[184]);
        let eq27_e1835_d_b1: f64 = (eq27_e1833_d_b1 * s.v[184]);
        let eq27_e1835_d_b2: f64 = (eq27_e1833_d_b2 * s.v[184]);
        let eq27_e1835_d_b3: f64 = (eq27_e1833_d_b3 * s.v[184]);
        let eq27_e1835_d_b4: f64 = (eq27_e1833_d_b4 * s.v[184]);
        let eq27_e1835_d_b5: f64 = (eq27_e1833_d_b5 * s.v[184]);
        let eq27_e1835_d_b6: f64 = (eq27_e1833_d_b6 * s.v[184]);
        let eq27_e1835_d_b7: f64 = (eq27_e1833_d_b7 * s.v[184]);
        let eq27_e1835_d_b8: f64 = (eq27_e1833_d_b8 * s.v[184]);
        let eq27_e1835_d_b9: f64 = (eq27_e1833_d_b9 * s.v[184]);
        let eq27_e1835_d_b10: f64 = (eq27_e1833_d_b10 * s.v[184]);
        let eq27_e1835_d_b11: f64 = (eq27_e1833_d_b11 * s.v[184]);
        let eq27_e1837: f64 = (eq27_e1835 * (nv12 - 0.0));
        let eq27_e1837_d_n0: f64 = (eq27_e1835_d_n0 * (nv12 - 0.0));
        let eq27_e1837_d_n1: f64 = (eq27_e1835_d_n1 * (nv12 - 0.0));
        let eq27_e1837_d_n2: f64 = (eq27_e1835_d_n2 * (nv12 - 0.0));
        let eq27_e1837_d_n3: f64 = (eq27_e1835_d_n3 * (nv12 - 0.0));
        let eq27_e1837_d_n4: f64 = (eq27_e1835_d_n4 * (nv12 - 0.0));
        let eq27_e1837_d_n5: f64 = (eq27_e1835_d_n5 * (nv12 - 0.0));
        let eq27_e1837_d_n6: f64 = (eq27_e1835_d_n6 * (nv12 - 0.0));
        let eq27_e1837_d_n7: f64 = (eq27_e1835_d_n7 * (nv12 - 0.0));
        let eq27_e1837_d_n8: f64 = (eq27_e1835_d_n8 * (nv12 - 0.0));
        let eq27_e1837_d_n9: f64 = (eq27_e1835_d_n9 * (nv12 - 0.0));
        let eq27_e1837_d_n10: f64 = (eq27_e1835_d_n10 * (nv12 - 0.0));
        let eq27_e1837_d_n11: f64 = (eq27_e1835_d_n11 * (nv12 - 0.0));
        let eq27_e1837_d_n12: f64 = ((eq27_e1835_d_n12 * (nv12 - 0.0)) + eq27_e1835);
        let eq27_e1837_d_n13: f64 = (eq27_e1835_d_n13 * (nv12 - 0.0));
        let eq27_e1837_d_b0: f64 = (eq27_e1835_d_b0 * (nv12 - 0.0));
        let eq27_e1837_d_b1: f64 = (eq27_e1835_d_b1 * (nv12 - 0.0));
        let eq27_e1837_d_b2: f64 = (eq27_e1835_d_b2 * (nv12 - 0.0));
        let eq27_e1837_d_b3: f64 = (eq27_e1835_d_b3 * (nv12 - 0.0));
        let eq27_e1837_d_b4: f64 = (eq27_e1835_d_b4 * (nv12 - 0.0));
        let eq27_e1837_d_b5: f64 = (eq27_e1835_d_b5 * (nv12 - 0.0));
        let eq27_e1837_d_b6: f64 = (eq27_e1835_d_b6 * (nv12 - 0.0));
        let eq27_e1837_d_b7: f64 = (eq27_e1835_d_b7 * (nv12 - 0.0));
        let eq27_e1837_d_b8: f64 = (eq27_e1835_d_b8 * (nv12 - 0.0));
        let eq27_e1837_d_b9: f64 = (eq27_e1835_d_b9 * (nv12 - 0.0));
        let eq27_e1837_d_b10: f64 = (eq27_e1835_d_b10 * (nv12 - 0.0));
        let eq27_e1837_d_b11: f64 = (eq27_e1835_d_b11 * (nv12 - 0.0));
        let eq27_e1838: f64 = (0.5 * eq27_e1837);
        let eq27_e1838_d_n0: f64 = (0.5 * eq27_e1837_d_n0);
        let eq27_e1838_d_n1: f64 = (0.5 * eq27_e1837_d_n1);
        let eq27_e1838_d_n2: f64 = (0.5 * eq27_e1837_d_n2);
        let eq27_e1838_d_n3: f64 = (0.5 * eq27_e1837_d_n3);
        let eq27_e1838_d_n4: f64 = (0.5 * eq27_e1837_d_n4);
        let eq27_e1838_d_n5: f64 = (0.5 * eq27_e1837_d_n5);
        let eq27_e1838_d_n6: f64 = (0.5 * eq27_e1837_d_n6);
        let eq27_e1838_d_n7: f64 = (0.5 * eq27_e1837_d_n7);
        let eq27_e1838_d_n8: f64 = (0.5 * eq27_e1837_d_n8);
        let eq27_e1838_d_n9: f64 = (0.5 * eq27_e1837_d_n9);
        let eq27_e1838_d_n10: f64 = (0.5 * eq27_e1837_d_n10);
        let eq27_e1838_d_n11: f64 = (0.5 * eq27_e1837_d_n11);
        let eq27_e1838_d_n12: f64 = (0.5 * eq27_e1837_d_n12);
        let eq27_e1838_d_n13: f64 = (0.5 * eq27_e1837_d_n13);
        let eq27_e1838_d_b0: f64 = (0.5 * eq27_e1837_d_b0);
        let eq27_e1838_d_b1: f64 = (0.5 * eq27_e1837_d_b1);
        let eq27_e1838_d_b2: f64 = (0.5 * eq27_e1837_d_b2);
        let eq27_e1838_d_b3: f64 = (0.5 * eq27_e1837_d_b3);
        let eq27_e1838_d_b4: f64 = (0.5 * eq27_e1837_d_b4);
        let eq27_e1838_d_b5: f64 = (0.5 * eq27_e1837_d_b5);
        let eq27_e1838_d_b6: f64 = (0.5 * eq27_e1837_d_b6);
        let eq27_e1838_d_b7: f64 = (0.5 * eq27_e1837_d_b7);
        let eq27_e1838_d_b8: f64 = (0.5 * eq27_e1837_d_b8);
        let eq27_e1838_d_b9: f64 = (0.5 * eq27_e1837_d_b9);
        let eq27_e1838_d_b10: f64 = (0.5 * eq27_e1837_d_b10);
        let eq27_e1838_d_b11: f64 = (0.5 * eq27_e1837_d_b11);
        let eq27_e1839_q: f64 = eq27_e1838;
        (eq27_e1838, eq27_e1838_d_n0, eq27_e1838_d_n1, eq27_e1838_d_n2, eq27_e1838_d_n3, eq27_e1838_d_n4, eq27_e1838_d_n5, eq27_e1838_d_n6, eq27_e1838_d_n7, eq27_e1838_d_n8, eq27_e1838_d_n9, eq27_e1838_d_n10, eq27_e1838_d_n11, eq27_e1838_d_n12, eq27_e1838_d_n13, eq27_e1838_d_b0, eq27_e1838_d_b1, eq27_e1838_d_b2, eq27_e1838_d_b3, eq27_e1838_d_b4, eq27_e1838_d_b5, eq27_e1838_d_b6, eq27_e1838_d_b7, eq27_e1838_d_b8, eq27_e1838_d_b9, eq27_e1838_d_b10, eq27_e1838_d_b11, eq27_e1839_q, eq27_e1838_d_n0, eq27_e1838_d_n1, eq27_e1838_d_n2, eq27_e1838_d_n3, eq27_e1838_d_n4, eq27_e1838_d_n5, eq27_e1838_d_n6, eq27_e1838_d_n7, eq27_e1838_d_n8, eq27_e1838_d_n9, eq27_e1838_d_n10, eq27_e1838_d_n11, eq27_e1838_d_n12, eq27_e1838_d_n13, eq27_e1838_d_b0, eq27_e1838_d_b1, eq27_e1838_d_b2, eq27_e1838_d_b3, eq27_e1838_d_b4, eq27_e1838_d_b5, eq27_e1838_d_b6, eq27_e1838_d_b7, eq27_e1838_d_b8, eq27_e1838_d_b9, eq27_e1838_d_b10, eq27_e1838_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_reactive_node_derivatives: [f64; 14] = [eq27_e1841_q_d_n0, eq27_e1841_q_d_n1, eq27_e1841_q_d_n2, eq27_e1841_q_d_n3, eq27_e1841_q_d_n4, eq27_e1841_q_d_n5, eq27_e1841_q_d_n6, eq27_e1841_q_d_n7, eq27_e1841_q_d_n8, eq27_e1841_q_d_n9, eq27_e1841_q_d_n10, eq27_e1841_q_d_n11, eq27_e1841_q_d_n12, eq27_e1841_q_d_n13];
        let eq27_reactive_branch_derivatives: [f64; 12] = [eq27_e1841_q_d_b0, eq27_e1841_q_d_b1, eq27_e1841_q_d_b2, eq27_e1841_q_d_b3, eq27_e1841_q_d_b4, eq27_e1841_q_d_b5, eq27_e1841_q_d_b6, eq27_e1841_q_d_b7, eq27_e1841_q_d_b8, eq27_e1841_q_d_b9, eq27_e1841_q_d_b10, eq27_e1841_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1868, eq28_e1868_d_n0, eq28_e1868_d_n1, eq28_e1868_d_n2, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_d_n13, eq28_e1868_d_b0, eq28_e1868_d_b1, eq28_e1868_d_b2, eq28_e1868_d_b3, eq28_e1868_d_b4, eq28_e1868_d_b5, eq28_e1868_d_b6, eq28_e1868_d_b7, eq28_e1868_d_b8, eq28_e1868_d_b9, eq28_e1868_d_b10, eq28_e1868_d_b11, eq28_e1868_q, eq28_e1868_q_d_n0, eq28_e1868_q_d_n1, eq28_e1868_q_d_n2, eq28_e1868_q_d_n3, eq28_e1868_q_d_n4, eq28_e1868_q_d_n5, eq28_e1868_q_d_n6, eq28_e1868_q_d_n7, eq28_e1868_q_d_n8, eq28_e1868_q_d_n9, eq28_e1868_q_d_n10, eq28_e1868_q_d_n11, eq28_e1868_q_d_n12, eq28_e1868_q_d_n13, eq28_e1868_q_d_b0, eq28_e1868_q_d_b1, eq28_e1868_q_d_b2, eq28_e1868_q_d_b3, eq28_e1868_q_d_b4, eq28_e1868_q_d_b5, eq28_e1868_q_d_b6, eq28_e1868_q_d_b7, eq28_e1868_q_d_b8, eq28_e1868_q_d_b9, eq28_e1868_q_d_b10, eq28_e1868_q_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq28_e1852: f64 = (1.0 - s.v[211]);
        let eq28_e1852_d_n0: f64 = (-s.dn[211][0]);
        let eq28_e1852_d_n1: f64 = (-s.dn[211][1]);
        let eq28_e1852_d_n2: f64 = (-s.dn[211][2]);
        let eq28_e1852_d_n3: f64 = (-s.dn[211][3]);
        let eq28_e1852_d_n4: f64 = (-s.dn[211][4]);
        let eq28_e1852_d_n5: f64 = (-s.dn[211][5]);
        let eq28_e1852_d_n6: f64 = (-s.dn[211][6]);
        let eq28_e1852_d_n7: f64 = (-s.dn[211][7]);
        let eq28_e1852_d_n8: f64 = (-s.dn[211][8]);
        let eq28_e1852_d_n9: f64 = (-s.dn[211][9]);
        let eq28_e1852_d_n10: f64 = (-s.dn[211][10]);
        let eq28_e1852_d_n11: f64 = (-s.dn[211][11]);
        let eq28_e1852_d_n12: f64 = (-s.dn[211][12]);
        let eq28_e1852_d_n13: f64 = (-s.dn[211][13]);
        let eq28_e1852_d_b0: f64 = (-s.db[211][0]);
        let eq28_e1852_d_b1: f64 = (-s.db[211][1]);
        let eq28_e1852_d_b2: f64 = (-s.db[211][2]);
        let eq28_e1852_d_b3: f64 = (-s.db[211][3]);
        let eq28_e1852_d_b4: f64 = (-s.db[211][4]);
        let eq28_e1852_d_b5: f64 = (-s.db[211][5]);
        let eq28_e1852_d_b6: f64 = (-s.db[211][6]);
        let eq28_e1852_d_b7: f64 = (-s.db[211][7]);
        let eq28_e1852_d_b8: f64 = (-s.db[211][8]);
        let eq28_e1852_d_b9: f64 = (-s.db[211][9]);
        let eq28_e1852_d_b10: f64 = (-s.db[211][10]);
        let eq28_e1852_d_b11: f64 = (-s.db[211][11]);
        let eq28_e1854: f64 = (eq28_e1852 * s.v[622]);
        let eq28_e1854_d_n0: f64 = ((eq28_e1852_d_n0 * s.v[622]) + (eq28_e1852 * s.dn[622][0]));
        let eq28_e1854_d_n1: f64 = ((eq28_e1852_d_n1 * s.v[622]) + (eq28_e1852 * s.dn[622][1]));
        let eq28_e1854_d_n2: f64 = ((eq28_e1852_d_n2 * s.v[622]) + (eq28_e1852 * s.dn[622][2]));
        let eq28_e1854_d_n3: f64 = ((eq28_e1852_d_n3 * s.v[622]) + (eq28_e1852 * s.dn[622][3]));
        let eq28_e1854_d_n4: f64 = ((eq28_e1852_d_n4 * s.v[622]) + (eq28_e1852 * s.dn[622][4]));
        let eq28_e1854_d_n5: f64 = ((eq28_e1852_d_n5 * s.v[622]) + (eq28_e1852 * s.dn[622][5]));
        let eq28_e1854_d_n6: f64 = ((eq28_e1852_d_n6 * s.v[622]) + (eq28_e1852 * s.dn[622][6]));
        let eq28_e1854_d_n7: f64 = ((eq28_e1852_d_n7 * s.v[622]) + (eq28_e1852 * s.dn[622][7]));
        let eq28_e1854_d_n8: f64 = ((eq28_e1852_d_n8 * s.v[622]) + (eq28_e1852 * s.dn[622][8]));
        let eq28_e1854_d_n9: f64 = ((eq28_e1852_d_n9 * s.v[622]) + (eq28_e1852 * s.dn[622][9]));
        let eq28_e1854_d_n10: f64 = ((eq28_e1852_d_n10 * s.v[622]) + (eq28_e1852 * s.dn[622][10]));
        let eq28_e1854_d_n11: f64 = ((eq28_e1852_d_n11 * s.v[622]) + (eq28_e1852 * s.dn[622][11]));
        let eq28_e1854_d_n12: f64 = ((eq28_e1852_d_n12 * s.v[622]) + (eq28_e1852 * s.dn[622][12]));
        let eq28_e1854_d_n13: f64 = ((eq28_e1852_d_n13 * s.v[622]) + (eq28_e1852 * s.dn[622][13]));
        let eq28_e1854_d_b0: f64 = ((eq28_e1852_d_b0 * s.v[622]) + (eq28_e1852 * s.db[622][0]));
        let eq28_e1854_d_b1: f64 = ((eq28_e1852_d_b1 * s.v[622]) + (eq28_e1852 * s.db[622][1]));
        let eq28_e1854_d_b2: f64 = ((eq28_e1852_d_b2 * s.v[622]) + (eq28_e1852 * s.db[622][2]));
        let eq28_e1854_d_b3: f64 = ((eq28_e1852_d_b3 * s.v[622]) + (eq28_e1852 * s.db[622][3]));
        let eq28_e1854_d_b4: f64 = ((eq28_e1852_d_b4 * s.v[622]) + (eq28_e1852 * s.db[622][4]));
        let eq28_e1854_d_b5: f64 = ((eq28_e1852_d_b5 * s.v[622]) + (eq28_e1852 * s.db[622][5]));
        let eq28_e1854_d_b6: f64 = ((eq28_e1852_d_b6 * s.v[622]) + (eq28_e1852 * s.db[622][6]));
        let eq28_e1854_d_b7: f64 = ((eq28_e1852_d_b7 * s.v[622]) + (eq28_e1852 * s.db[622][7]));
        let eq28_e1854_d_b8: f64 = ((eq28_e1852_d_b8 * s.v[622]) + (eq28_e1852 * s.db[622][8]));
        let eq28_e1854_d_b9: f64 = ((eq28_e1852_d_b9 * s.v[622]) + (eq28_e1852 * s.db[622][9]));
        let eq28_e1854_d_b10: f64 = ((eq28_e1852_d_b10 * s.v[622]) + (eq28_e1852 * s.db[622][10]));
        let eq28_e1854_d_b11: f64 = ((eq28_e1852_d_b11 * s.v[622]) + (eq28_e1852 * s.db[622][11]));
        let eq28_e1856: f64 = (eq28_e1854 * s.v[199]);
        let eq28_e1856_d_n0: f64 = (eq28_e1854_d_n0 * s.v[199]);
        let eq28_e1856_d_n1: f64 = (eq28_e1854_d_n1 * s.v[199]);
        let eq28_e1856_d_n2: f64 = (eq28_e1854_d_n2 * s.v[199]);
        let eq28_e1856_d_n3: f64 = (eq28_e1854_d_n3 * s.v[199]);
        let eq28_e1856_d_n4: f64 = (eq28_e1854_d_n4 * s.v[199]);
        let eq28_e1856_d_n5: f64 = (eq28_e1854_d_n5 * s.v[199]);
        let eq28_e1856_d_n6: f64 = (eq28_e1854_d_n6 * s.v[199]);
        let eq28_e1856_d_n7: f64 = (eq28_e1854_d_n7 * s.v[199]);
        let eq28_e1856_d_n8: f64 = (eq28_e1854_d_n8 * s.v[199]);
        let eq28_e1856_d_n9: f64 = (eq28_e1854_d_n9 * s.v[199]);
        let eq28_e1856_d_n10: f64 = (eq28_e1854_d_n10 * s.v[199]);
        let eq28_e1856_d_n11: f64 = (eq28_e1854_d_n11 * s.v[199]);
        let eq28_e1856_d_n12: f64 = (eq28_e1854_d_n12 * s.v[199]);
        let eq28_e1856_d_n13: f64 = (eq28_e1854_d_n13 * s.v[199]);
        let eq28_e1856_d_b0: f64 = (eq28_e1854_d_b0 * s.v[199]);
        let eq28_e1856_d_b1: f64 = (eq28_e1854_d_b1 * s.v[199]);
        let eq28_e1856_d_b2: f64 = (eq28_e1854_d_b2 * s.v[199]);
        let eq28_e1856_d_b3: f64 = (eq28_e1854_d_b3 * s.v[199]);
        let eq28_e1856_d_b4: f64 = (eq28_e1854_d_b4 * s.v[199]);
        let eq28_e1856_d_b5: f64 = (eq28_e1854_d_b5 * s.v[199]);
        let eq28_e1856_d_b6: f64 = (eq28_e1854_d_b6 * s.v[199]);
        let eq28_e1856_d_b7: f64 = (eq28_e1854_d_b7 * s.v[199]);
        let eq28_e1856_d_b8: f64 = (eq28_e1854_d_b8 * s.v[199]);
        let eq28_e1856_d_b9: f64 = (eq28_e1854_d_b9 * s.v[199]);
        let eq28_e1856_d_b10: f64 = (eq28_e1854_d_b10 * s.v[199]);
        let eq28_e1856_d_b11: f64 = (eq28_e1854_d_b11 * s.v[199]);
        let eq28_e1858: f64 = (eq28_e1856 * s.v[183]);
        let eq28_e1858_d_n0: f64 = (eq28_e1856_d_n0 * s.v[183]);
        let eq28_e1858_d_n1: f64 = (eq28_e1856_d_n1 * s.v[183]);
        let eq28_e1858_d_n2: f64 = (eq28_e1856_d_n2 * s.v[183]);
        let eq28_e1858_d_n3: f64 = (eq28_e1856_d_n3 * s.v[183]);
        let eq28_e1858_d_n4: f64 = (eq28_e1856_d_n4 * s.v[183]);
        let eq28_e1858_d_n5: f64 = (eq28_e1856_d_n5 * s.v[183]);
        let eq28_e1858_d_n6: f64 = (eq28_e1856_d_n6 * s.v[183]);
        let eq28_e1858_d_n7: f64 = (eq28_e1856_d_n7 * s.v[183]);
        let eq28_e1858_d_n8: f64 = (eq28_e1856_d_n8 * s.v[183]);
        let eq28_e1858_d_n9: f64 = (eq28_e1856_d_n9 * s.v[183]);
        let eq28_e1858_d_n10: f64 = (eq28_e1856_d_n10 * s.v[183]);
        let eq28_e1858_d_n11: f64 = (eq28_e1856_d_n11 * s.v[183]);
        let eq28_e1858_d_n12: f64 = (eq28_e1856_d_n12 * s.v[183]);
        let eq28_e1858_d_n13: f64 = (eq28_e1856_d_n13 * s.v[183]);
        let eq28_e1858_d_b0: f64 = (eq28_e1856_d_b0 * s.v[183]);
        let eq28_e1858_d_b1: f64 = (eq28_e1856_d_b1 * s.v[183]);
        let eq28_e1858_d_b2: f64 = (eq28_e1856_d_b2 * s.v[183]);
        let eq28_e1858_d_b3: f64 = (eq28_e1856_d_b3 * s.v[183]);
        let eq28_e1858_d_b4: f64 = (eq28_e1856_d_b4 * s.v[183]);
        let eq28_e1858_d_b5: f64 = (eq28_e1856_d_b5 * s.v[183]);
        let eq28_e1858_d_b6: f64 = (eq28_e1856_d_b6 * s.v[183]);
        let eq28_e1858_d_b7: f64 = (eq28_e1856_d_b7 * s.v[183]);
        let eq28_e1858_d_b8: f64 = (eq28_e1856_d_b8 * s.v[183]);
        let eq28_e1858_d_b9: f64 = (eq28_e1856_d_b9 * s.v[183]);
        let eq28_e1858_d_b10: f64 = (eq28_e1856_d_b10 * s.v[183]);
        let eq28_e1858_d_b11: f64 = (eq28_e1856_d_b11 * s.v[183]);
        let eq28_e1860: f64 = (eq28_e1858 * p.p2);
        let eq28_e1860_d_n0: f64 = (eq28_e1858_d_n0 * p.p2);
        let eq28_e1860_d_n1: f64 = (eq28_e1858_d_n1 * p.p2);
        let eq28_e1860_d_n2: f64 = (eq28_e1858_d_n2 * p.p2);
        let eq28_e1860_d_n3: f64 = (eq28_e1858_d_n3 * p.p2);
        let eq28_e1860_d_n4: f64 = (eq28_e1858_d_n4 * p.p2);
        let eq28_e1860_d_n5: f64 = (eq28_e1858_d_n5 * p.p2);
        let eq28_e1860_d_n6: f64 = (eq28_e1858_d_n6 * p.p2);
        let eq28_e1860_d_n7: f64 = (eq28_e1858_d_n7 * p.p2);
        let eq28_e1860_d_n8: f64 = (eq28_e1858_d_n8 * p.p2);
        let eq28_e1860_d_n9: f64 = (eq28_e1858_d_n9 * p.p2);
        let eq28_e1860_d_n10: f64 = (eq28_e1858_d_n10 * p.p2);
        let eq28_e1860_d_n11: f64 = (eq28_e1858_d_n11 * p.p2);
        let eq28_e1860_d_n12: f64 = (eq28_e1858_d_n12 * p.p2);
        let eq28_e1860_d_n13: f64 = (eq28_e1858_d_n13 * p.p2);
        let eq28_e1860_d_b0: f64 = (eq28_e1858_d_b0 * p.p2);
        let eq28_e1860_d_b1: f64 = (eq28_e1858_d_b1 * p.p2);
        let eq28_e1860_d_b2: f64 = (eq28_e1858_d_b2 * p.p2);
        let eq28_e1860_d_b3: f64 = (eq28_e1858_d_b3 * p.p2);
        let eq28_e1860_d_b4: f64 = (eq28_e1858_d_b4 * p.p2);
        let eq28_e1860_d_b5: f64 = (eq28_e1858_d_b5 * p.p2);
        let eq28_e1860_d_b6: f64 = (eq28_e1858_d_b6 * p.p2);
        let eq28_e1860_d_b7: f64 = (eq28_e1858_d_b7 * p.p2);
        let eq28_e1860_d_b8: f64 = (eq28_e1858_d_b8 * p.p2);
        let eq28_e1860_d_b9: f64 = (eq28_e1858_d_b9 * p.p2);
        let eq28_e1860_d_b10: f64 = (eq28_e1858_d_b10 * p.p2);
        let eq28_e1860_d_b11: f64 = (eq28_e1858_d_b11 * p.p2);
        let eq28_e1862: f64 = (eq28_e1860 * s.v[184]);
        let eq28_e1862_d_n0: f64 = (eq28_e1860_d_n0 * s.v[184]);
        let eq28_e1862_d_n1: f64 = (eq28_e1860_d_n1 * s.v[184]);
        let eq28_e1862_d_n2: f64 = (eq28_e1860_d_n2 * s.v[184]);
        let eq28_e1862_d_n3: f64 = (eq28_e1860_d_n3 * s.v[184]);
        let eq28_e1862_d_n4: f64 = (eq28_e1860_d_n4 * s.v[184]);
        let eq28_e1862_d_n5: f64 = (eq28_e1860_d_n5 * s.v[184]);
        let eq28_e1862_d_n6: f64 = (eq28_e1860_d_n6 * s.v[184]);
        let eq28_e1862_d_n7: f64 = (eq28_e1860_d_n7 * s.v[184]);
        let eq28_e1862_d_n8: f64 = (eq28_e1860_d_n8 * s.v[184]);
        let eq28_e1862_d_n9: f64 = (eq28_e1860_d_n9 * s.v[184]);
        let eq28_e1862_d_n10: f64 = (eq28_e1860_d_n10 * s.v[184]);
        let eq28_e1862_d_n11: f64 = (eq28_e1860_d_n11 * s.v[184]);
        let eq28_e1862_d_n12: f64 = (eq28_e1860_d_n12 * s.v[184]);
        let eq28_e1862_d_n13: f64 = (eq28_e1860_d_n13 * s.v[184]);
        let eq28_e1862_d_b0: f64 = (eq28_e1860_d_b0 * s.v[184]);
        let eq28_e1862_d_b1: f64 = (eq28_e1860_d_b1 * s.v[184]);
        let eq28_e1862_d_b2: f64 = (eq28_e1860_d_b2 * s.v[184]);
        let eq28_e1862_d_b3: f64 = (eq28_e1860_d_b3 * s.v[184]);
        let eq28_e1862_d_b4: f64 = (eq28_e1860_d_b4 * s.v[184]);
        let eq28_e1862_d_b5: f64 = (eq28_e1860_d_b5 * s.v[184]);
        let eq28_e1862_d_b6: f64 = (eq28_e1860_d_b6 * s.v[184]);
        let eq28_e1862_d_b7: f64 = (eq28_e1860_d_b7 * s.v[184]);
        let eq28_e1862_d_b8: f64 = (eq28_e1860_d_b8 * s.v[184]);
        let eq28_e1862_d_b9: f64 = (eq28_e1860_d_b9 * s.v[184]);
        let eq28_e1862_d_b10: f64 = (eq28_e1860_d_b10 * s.v[184]);
        let eq28_e1862_d_b11: f64 = (eq28_e1860_d_b11 * s.v[184]);
        let eq28_e1864: f64 = (eq28_e1862 * (nv12 - 0.0));
        let eq28_e1864_d_n0: f64 = (eq28_e1862_d_n0 * (nv12 - 0.0));
        let eq28_e1864_d_n1: f64 = (eq28_e1862_d_n1 * (nv12 - 0.0));
        let eq28_e1864_d_n2: f64 = (eq28_e1862_d_n2 * (nv12 - 0.0));
        let eq28_e1864_d_n3: f64 = (eq28_e1862_d_n3 * (nv12 - 0.0));
        let eq28_e1864_d_n4: f64 = (eq28_e1862_d_n4 * (nv12 - 0.0));
        let eq28_e1864_d_n5: f64 = (eq28_e1862_d_n5 * (nv12 - 0.0));
        let eq28_e1864_d_n6: f64 = (eq28_e1862_d_n6 * (nv12 - 0.0));
        let eq28_e1864_d_n7: f64 = (eq28_e1862_d_n7 * (nv12 - 0.0));
        let eq28_e1864_d_n8: f64 = (eq28_e1862_d_n8 * (nv12 - 0.0));
        let eq28_e1864_d_n9: f64 = (eq28_e1862_d_n9 * (nv12 - 0.0));
        let eq28_e1864_d_n10: f64 = (eq28_e1862_d_n10 * (nv12 - 0.0));
        let eq28_e1864_d_n11: f64 = (eq28_e1862_d_n11 * (nv12 - 0.0));
        let eq28_e1864_d_n12: f64 = ((eq28_e1862_d_n12 * (nv12 - 0.0)) + eq28_e1862);
        let eq28_e1864_d_n13: f64 = (eq28_e1862_d_n13 * (nv12 - 0.0));
        let eq28_e1864_d_b0: f64 = (eq28_e1862_d_b0 * (nv12 - 0.0));
        let eq28_e1864_d_b1: f64 = (eq28_e1862_d_b1 * (nv12 - 0.0));
        let eq28_e1864_d_b2: f64 = (eq28_e1862_d_b2 * (nv12 - 0.0));
        let eq28_e1864_d_b3: f64 = (eq28_e1862_d_b3 * (nv12 - 0.0));
        let eq28_e1864_d_b4: f64 = (eq28_e1862_d_b4 * (nv12 - 0.0));
        let eq28_e1864_d_b5: f64 = (eq28_e1862_d_b5 * (nv12 - 0.0));
        let eq28_e1864_d_b6: f64 = (eq28_e1862_d_b6 * (nv12 - 0.0));
        let eq28_e1864_d_b7: f64 = (eq28_e1862_d_b7 * (nv12 - 0.0));
        let eq28_e1864_d_b8: f64 = (eq28_e1862_d_b8 * (nv12 - 0.0));
        let eq28_e1864_d_b9: f64 = (eq28_e1862_d_b9 * (nv12 - 0.0));
        let eq28_e1864_d_b10: f64 = (eq28_e1862_d_b10 * (nv12 - 0.0));
        let eq28_e1864_d_b11: f64 = (eq28_e1862_d_b11 * (nv12 - 0.0));
        let eq28_e1865: f64 = (0.5 * eq28_e1864);
        let eq28_e1865_d_n0: f64 = (0.5 * eq28_e1864_d_n0);
        let eq28_e1865_d_n1: f64 = (0.5 * eq28_e1864_d_n1);
        let eq28_e1865_d_n2: f64 = (0.5 * eq28_e1864_d_n2);
        let eq28_e1865_d_n3: f64 = (0.5 * eq28_e1864_d_n3);
        let eq28_e1865_d_n4: f64 = (0.5 * eq28_e1864_d_n4);
        let eq28_e1865_d_n5: f64 = (0.5 * eq28_e1864_d_n5);
        let eq28_e1865_d_n6: f64 = (0.5 * eq28_e1864_d_n6);
        let eq28_e1865_d_n7: f64 = (0.5 * eq28_e1864_d_n7);
        let eq28_e1865_d_n8: f64 = (0.5 * eq28_e1864_d_n8);
        let eq28_e1865_d_n9: f64 = (0.5 * eq28_e1864_d_n9);
        let eq28_e1865_d_n10: f64 = (0.5 * eq28_e1864_d_n10);
        let eq28_e1865_d_n11: f64 = (0.5 * eq28_e1864_d_n11);
        let eq28_e1865_d_n12: f64 = (0.5 * eq28_e1864_d_n12);
        let eq28_e1865_d_n13: f64 = (0.5 * eq28_e1864_d_n13);
        let eq28_e1865_d_b0: f64 = (0.5 * eq28_e1864_d_b0);
        let eq28_e1865_d_b1: f64 = (0.5 * eq28_e1864_d_b1);
        let eq28_e1865_d_b2: f64 = (0.5 * eq28_e1864_d_b2);
        let eq28_e1865_d_b3: f64 = (0.5 * eq28_e1864_d_b3);
        let eq28_e1865_d_b4: f64 = (0.5 * eq28_e1864_d_b4);
        let eq28_e1865_d_b5: f64 = (0.5 * eq28_e1864_d_b5);
        let eq28_e1865_d_b6: f64 = (0.5 * eq28_e1864_d_b6);
        let eq28_e1865_d_b7: f64 = (0.5 * eq28_e1864_d_b7);
        let eq28_e1865_d_b8: f64 = (0.5 * eq28_e1864_d_b8);
        let eq28_e1865_d_b9: f64 = (0.5 * eq28_e1864_d_b9);
        let eq28_e1865_d_b10: f64 = (0.5 * eq28_e1864_d_b10);
        let eq28_e1865_d_b11: f64 = (0.5 * eq28_e1864_d_b11);
        let eq28_e1866_q: f64 = eq28_e1865;
        (eq28_e1865, eq28_e1865_d_n0, eq28_e1865_d_n1, eq28_e1865_d_n2, eq28_e1865_d_n3, eq28_e1865_d_n4, eq28_e1865_d_n5, eq28_e1865_d_n6, eq28_e1865_d_n7, eq28_e1865_d_n8, eq28_e1865_d_n9, eq28_e1865_d_n10, eq28_e1865_d_n11, eq28_e1865_d_n12, eq28_e1865_d_n13, eq28_e1865_d_b0, eq28_e1865_d_b1, eq28_e1865_d_b2, eq28_e1865_d_b3, eq28_e1865_d_b4, eq28_e1865_d_b5, eq28_e1865_d_b6, eq28_e1865_d_b7, eq28_e1865_d_b8, eq28_e1865_d_b9, eq28_e1865_d_b10, eq28_e1865_d_b11, eq28_e1866_q, eq28_e1865_d_n0, eq28_e1865_d_n1, eq28_e1865_d_n2, eq28_e1865_d_n3, eq28_e1865_d_n4, eq28_e1865_d_n5, eq28_e1865_d_n6, eq28_e1865_d_n7, eq28_e1865_d_n8, eq28_e1865_d_n9, eq28_e1865_d_n10, eq28_e1865_d_n11, eq28_e1865_d_n12, eq28_e1865_d_n13, eq28_e1865_d_b0, eq28_e1865_d_b1, eq28_e1865_d_b2, eq28_e1865_d_b3, eq28_e1865_d_b4, eq28_e1865_d_b5, eq28_e1865_d_b6, eq28_e1865_d_b7, eq28_e1865_d_b8, eq28_e1865_d_b9, eq28_e1865_d_b10, eq28_e1865_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 14] = [eq28_e1868_q_d_n0, eq28_e1868_q_d_n1, eq28_e1868_q_d_n2, eq28_e1868_q_d_n3, eq28_e1868_q_d_n4, eq28_e1868_q_d_n5, eq28_e1868_q_d_n6, eq28_e1868_q_d_n7, eq28_e1868_q_d_n8, eq28_e1868_q_d_n9, eq28_e1868_q_d_n10, eq28_e1868_q_d_n11, eq28_e1868_q_d_n12, eq28_e1868_q_d_n13];
        let eq28_reactive_branch_derivatives: [f64; 12] = [eq28_e1868_q_d_b0, eq28_e1868_q_d_b1, eq28_e1868_q_d_b2, eq28_e1868_q_d_b3, eq28_e1868_q_d_b4, eq28_e1868_q_d_b5, eq28_e1868_q_d_b6, eq28_e1868_q_d_b7, eq28_e1868_q_d_b8, eq28_e1868_q_d_b9, eq28_e1868_q_d_b10, eq28_e1868_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e1938_q: f64 = s.v[1057];
        let eq35_reactive_node_derivatives: [f64; 14] = [s.dn[1057][0], s.dn[1057][1], s.dn[1057][2], s.dn[1057][3], s.dn[1057][4], s.dn[1057][5], s.dn[1057][6], s.dn[1057][7], s.dn[1057][8], s.dn[1057][9], s.dn[1057][10], s.dn[1057][11], s.dn[1057][12], s.dn[1057][13]];
        let eq35_reactive_branch_derivatives: [f64; 12] = [s.db[1057][0], s.db[1057][1], s.db[1057][2], s.db[1057][3], s.db[1057][4], s.db[1057][5], s.db[1057][6], s.db[1057][7], s.db[1057][8], s.db[1057][9], s.db[1057][10], s.db[1057][11]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[10]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let eq36_e1940_q: f64 = s.v[1058];
        let eq36_reactive_node_derivatives: [f64; 14] = [s.dn[1058][0], s.dn[1058][1], s.dn[1058][2], s.dn[1058][3], s.dn[1058][4], s.dn[1058][5], s.dn[1058][6], s.dn[1058][7], s.dn[1058][8], s.dn[1058][9], s.dn[1058][10], s.dn[1058][11], s.dn[1058][12], s.dn[1058][13]];
        let eq36_reactive_branch_derivatives: [f64; 12] = [s.db[1058][0], s.db[1058][1], s.db[1058][2], s.db[1058][3], s.db[1058][4], s.db[1058][5], s.db[1058][6], s.db[1058][7], s.db[1058][8], s.db[1058][9], s.db[1058][10], s.db[1058][11]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[11]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e1942_q: f64 = s.v[1051];
        let eq37_reactive_node_derivatives: [f64; 14] = [s.dn[1051][0], s.dn[1051][1], s.dn[1051][2], s.dn[1051][3], s.dn[1051][4], s.dn[1051][5], s.dn[1051][6], s.dn[1051][7], s.dn[1051][8], s.dn[1051][9], s.dn[1051][10], s.dn[1051][11], s.dn[1051][12], s.dn[1051][13]];
        let eq37_reactive_branch_derivatives: [f64; 12] = [s.db[1051][0], s.db[1051][1], s.db[1051][2], s.db[1051][3], s.db[1051][4], s.db[1051][5], s.db[1051][6], s.db[1051][7], s.db[1051][8], s.db[1051][9], s.db[1051][10], s.db[1051][11]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e1944_q: f64 = s.v[1052];
        let eq38_reactive_node_derivatives: [f64; 14] = [s.dn[1052][0], s.dn[1052][1], s.dn[1052][2], s.dn[1052][3], s.dn[1052][4], s.dn[1052][5], s.dn[1052][6], s.dn[1052][7], s.dn[1052][8], s.dn[1052][9], s.dn[1052][10], s.dn[1052][11], s.dn[1052][12], s.dn[1052][13]];
        let eq38_reactive_branch_derivatives: [f64; 12] = [s.db[1052][0], s.db[1052][1], s.db[1052][2], s.db[1052][3], s.db[1052][4], s.db[1052][5], s.db[1052][6], s.db[1052][7], s.db[1052][8], s.db[1052][9], s.db[1052][10], s.db[1052][11]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let eq39_e1946_q: f64 = s.v[1054];
        let eq39_reactive_node_derivatives: [f64; 14] = [s.dn[1054][0], s.dn[1054][1], s.dn[1054][2], s.dn[1054][3], s.dn[1054][4], s.dn[1054][5], s.dn[1054][6], s.dn[1054][7], s.dn[1054][8], s.dn[1054][9], s.dn[1054][10], s.dn[1054][11], s.dn[1054][12], s.dn[1054][13]];
        let eq39_reactive_branch_derivatives: [f64; 12] = [s.db[1054][0], s.db[1054][1], s.db[1054][2], s.db[1054][3], s.db[1054][4], s.db[1054][5], s.db[1054][6], s.db[1054][7], s.db[1054][8], s.db[1054][9], s.db[1054][10], s.db[1054][11]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[10]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq40_e1948_q: f64 = s.v[1055];
        let eq40_reactive_node_derivatives: [f64; 14] = [s.dn[1055][0], s.dn[1055][1], s.dn[1055][2], s.dn[1055][3], s.dn[1055][4], s.dn[1055][5], s.dn[1055][6], s.dn[1055][7], s.dn[1055][8], s.dn[1055][9], s.dn[1055][10], s.dn[1055][11], s.dn[1055][12], s.dn[1055][13]];
        let eq40_reactive_branch_derivatives: [f64; 12] = [s.db[1055][0], s.db[1055][1], s.db[1055][2], s.db[1055][3], s.db[1055][4], s.db[1055][5], s.db[1055][6], s.db[1055][7], s.db[1055][8], s.db[1055][9], s.db[1055][10], s.db[1055][11]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1950: f64 = (-s.v[379]);
        let eq41_e1950_d_n0: f64 = (-s.dn[379][0]);
        let eq41_e1950_d_n1: f64 = (-s.dn[379][1]);
        let eq41_e1950_d_n2: f64 = (-s.dn[379][2]);
        let eq41_e1950_d_n3: f64 = (-s.dn[379][3]);
        let eq41_e1950_d_n4: f64 = (-s.dn[379][4]);
        let eq41_e1950_d_n5: f64 = (-s.dn[379][5]);
        let eq41_e1950_d_n6: f64 = (-s.dn[379][6]);
        let eq41_e1950_d_n7: f64 = (-s.dn[379][7]);
        let eq41_e1950_d_n8: f64 = (-s.dn[379][8]);
        let eq41_e1950_d_n9: f64 = (-s.dn[379][9]);
        let eq41_e1950_d_n10: f64 = (-s.dn[379][10]);
        let eq41_e1950_d_n11: f64 = (-s.dn[379][11]);
        let eq41_e1950_d_n12: f64 = (-s.dn[379][12]);
        let eq41_e1950_d_n13: f64 = (-s.dn[379][13]);
        let eq41_e1950_d_b0: f64 = (-s.db[379][0]);
        let eq41_e1950_d_b1: f64 = (-s.db[379][1]);
        let eq41_e1950_d_b2: f64 = (-s.db[379][2]);
        let eq41_e1950_d_b3: f64 = (-s.db[379][3]);
        let eq41_e1950_d_b4: f64 = (-s.db[379][4]);
        let eq41_e1950_d_b5: f64 = (-s.db[379][5]);
        let eq41_e1950_d_b6: f64 = (-s.db[379][6]);
        let eq41_e1950_d_b7: f64 = (-s.db[379][7]);
        let eq41_e1950_d_b8: f64 = (-s.db[379][8]);
        let eq41_e1950_d_b9: f64 = (-s.db[379][9]);
        let eq41_e1950_d_b10: f64 = (-s.db[379][10]);
        let eq41_e1950_d_b11: f64 = (-s.db[379][11]);
        let eq41_e1952: f64 = (eq41_e1950 * s.v[423]);
        let eq41_e1952_d_n0: f64 = ((eq41_e1950_d_n0 * s.v[423]) + (eq41_e1950 * s.dn[423][0]));
        let eq41_e1952_d_n1: f64 = ((eq41_e1950_d_n1 * s.v[423]) + (eq41_e1950 * s.dn[423][1]));
        let eq41_e1952_d_n2: f64 = ((eq41_e1950_d_n2 * s.v[423]) + (eq41_e1950 * s.dn[423][2]));
        let eq41_e1952_d_n3: f64 = ((eq41_e1950_d_n3 * s.v[423]) + (eq41_e1950 * s.dn[423][3]));
        let eq41_e1952_d_n4: f64 = ((eq41_e1950_d_n4 * s.v[423]) + (eq41_e1950 * s.dn[423][4]));
        let eq41_e1952_d_n5: f64 = ((eq41_e1950_d_n5 * s.v[423]) + (eq41_e1950 * s.dn[423][5]));
        let eq41_e1952_d_n6: f64 = ((eq41_e1950_d_n6 * s.v[423]) + (eq41_e1950 * s.dn[423][6]));
        let eq41_e1952_d_n7: f64 = ((eq41_e1950_d_n7 * s.v[423]) + (eq41_e1950 * s.dn[423][7]));
        let eq41_e1952_d_n8: f64 = ((eq41_e1950_d_n8 * s.v[423]) + (eq41_e1950 * s.dn[423][8]));
        let eq41_e1952_d_n9: f64 = ((eq41_e1950_d_n9 * s.v[423]) + (eq41_e1950 * s.dn[423][9]));
        let eq41_e1952_d_n10: f64 = ((eq41_e1950_d_n10 * s.v[423]) + (eq41_e1950 * s.dn[423][10]));
        let eq41_e1952_d_n11: f64 = ((eq41_e1950_d_n11 * s.v[423]) + (eq41_e1950 * s.dn[423][11]));
        let eq41_e1952_d_n12: f64 = ((eq41_e1950_d_n12 * s.v[423]) + (eq41_e1950 * s.dn[423][12]));
        let eq41_e1952_d_n13: f64 = ((eq41_e1950_d_n13 * s.v[423]) + (eq41_e1950 * s.dn[423][13]));
        let eq41_e1952_d_b0: f64 = ((eq41_e1950_d_b0 * s.v[423]) + (eq41_e1950 * s.db[423][0]));
        let eq41_e1952_d_b1: f64 = ((eq41_e1950_d_b1 * s.v[423]) + (eq41_e1950 * s.db[423][1]));
        let eq41_e1952_d_b2: f64 = ((eq41_e1950_d_b2 * s.v[423]) + (eq41_e1950 * s.db[423][2]));
        let eq41_e1952_d_b3: f64 = ((eq41_e1950_d_b3 * s.v[423]) + (eq41_e1950 * s.db[423][3]));
        let eq41_e1952_d_b4: f64 = ((eq41_e1950_d_b4 * s.v[423]) + (eq41_e1950 * s.db[423][4]));
        let eq41_e1952_d_b5: f64 = ((eq41_e1950_d_b5 * s.v[423]) + (eq41_e1950 * s.db[423][5]));
        let eq41_e1952_d_b6: f64 = ((eq41_e1950_d_b6 * s.v[423]) + (eq41_e1950 * s.db[423][6]));
        let eq41_e1952_d_b7: f64 = ((eq41_e1950_d_b7 * s.v[423]) + (eq41_e1950 * s.db[423][7]));
        let eq41_e1952_d_b8: f64 = ((eq41_e1950_d_b8 * s.v[423]) + (eq41_e1950 * s.db[423][8]));
        let eq41_e1952_d_b9: f64 = ((eq41_e1950_d_b9 * s.v[423]) + (eq41_e1950 * s.db[423][9]));
        let eq41_e1952_d_b10: f64 = ((eq41_e1950_d_b10 * s.v[423]) + (eq41_e1950 * s.db[423][10]));
        let eq41_e1952_d_b11: f64 = ((eq41_e1950_d_b11 * s.v[423]) + (eq41_e1950 * s.db[423][11]));
        let eq41_e1953_q: f64 = eq41_e1952;
        let eq41_reactive_node_derivatives: [f64; 14] = [eq41_e1952_d_n0, eq41_e1952_d_n1, eq41_e1952_d_n2, eq41_e1952_d_n3, eq41_e1952_d_n4, eq41_e1952_d_n5, eq41_e1952_d_n6, eq41_e1952_d_n7, eq41_e1952_d_n8, eq41_e1952_d_n9, eq41_e1952_d_n10, eq41_e1952_d_n11, eq41_e1952_d_n12, eq41_e1952_d_n13];
        let eq41_reactive_branch_derivatives: [f64; 12] = [eq41_e1952_d_b0, eq41_e1952_d_b1, eq41_e1952_d_b2, eq41_e1952_d_b3, eq41_e1952_d_b4, eq41_e1952_d_b5, eq41_e1952_d_b6, eq41_e1952_d_b7, eq41_e1952_d_b8, eq41_e1952_d_b9, eq41_e1952_d_b10, eq41_e1952_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1955: f64 = (-s.v[379]);
        let eq42_e1955_d_n0: f64 = (-s.dn[379][0]);
        let eq42_e1955_d_n1: f64 = (-s.dn[379][1]);
        let eq42_e1955_d_n2: f64 = (-s.dn[379][2]);
        let eq42_e1955_d_n3: f64 = (-s.dn[379][3]);
        let eq42_e1955_d_n4: f64 = (-s.dn[379][4]);
        let eq42_e1955_d_n5: f64 = (-s.dn[379][5]);
        let eq42_e1955_d_n6: f64 = (-s.dn[379][6]);
        let eq42_e1955_d_n7: f64 = (-s.dn[379][7]);
        let eq42_e1955_d_n8: f64 = (-s.dn[379][8]);
        let eq42_e1955_d_n9: f64 = (-s.dn[379][9]);
        let eq42_e1955_d_n10: f64 = (-s.dn[379][10]);
        let eq42_e1955_d_n11: f64 = (-s.dn[379][11]);
        let eq42_e1955_d_n12: f64 = (-s.dn[379][12]);
        let eq42_e1955_d_n13: f64 = (-s.dn[379][13]);
        let eq42_e1955_d_b0: f64 = (-s.db[379][0]);
        let eq42_e1955_d_b1: f64 = (-s.db[379][1]);
        let eq42_e1955_d_b2: f64 = (-s.db[379][2]);
        let eq42_e1955_d_b3: f64 = (-s.db[379][3]);
        let eq42_e1955_d_b4: f64 = (-s.db[379][4]);
        let eq42_e1955_d_b5: f64 = (-s.db[379][5]);
        let eq42_e1955_d_b6: f64 = (-s.db[379][6]);
        let eq42_e1955_d_b7: f64 = (-s.db[379][7]);
        let eq42_e1955_d_b8: f64 = (-s.db[379][8]);
        let eq42_e1955_d_b9: f64 = (-s.db[379][9]);
        let eq42_e1955_d_b10: f64 = (-s.db[379][10]);
        let eq42_e1955_d_b11: f64 = (-s.db[379][11]);
        let eq42_e1957: f64 = (eq42_e1955 * s.v[424]);
        let eq42_e1957_d_n0: f64 = ((eq42_e1955_d_n0 * s.v[424]) + (eq42_e1955 * s.dn[424][0]));
        let eq42_e1957_d_n1: f64 = ((eq42_e1955_d_n1 * s.v[424]) + (eq42_e1955 * s.dn[424][1]));
        let eq42_e1957_d_n2: f64 = ((eq42_e1955_d_n2 * s.v[424]) + (eq42_e1955 * s.dn[424][2]));
        let eq42_e1957_d_n3: f64 = ((eq42_e1955_d_n3 * s.v[424]) + (eq42_e1955 * s.dn[424][3]));
        let eq42_e1957_d_n4: f64 = ((eq42_e1955_d_n4 * s.v[424]) + (eq42_e1955 * s.dn[424][4]));
        let eq42_e1957_d_n5: f64 = ((eq42_e1955_d_n5 * s.v[424]) + (eq42_e1955 * s.dn[424][5]));
        let eq42_e1957_d_n6: f64 = ((eq42_e1955_d_n6 * s.v[424]) + (eq42_e1955 * s.dn[424][6]));
        let eq42_e1957_d_n7: f64 = ((eq42_e1955_d_n7 * s.v[424]) + (eq42_e1955 * s.dn[424][7]));
        let eq42_e1957_d_n8: f64 = ((eq42_e1955_d_n8 * s.v[424]) + (eq42_e1955 * s.dn[424][8]));
        let eq42_e1957_d_n9: f64 = ((eq42_e1955_d_n9 * s.v[424]) + (eq42_e1955 * s.dn[424][9]));
        let eq42_e1957_d_n10: f64 = ((eq42_e1955_d_n10 * s.v[424]) + (eq42_e1955 * s.dn[424][10]));
        let eq42_e1957_d_n11: f64 = ((eq42_e1955_d_n11 * s.v[424]) + (eq42_e1955 * s.dn[424][11]));
        let eq42_e1957_d_n12: f64 = ((eq42_e1955_d_n12 * s.v[424]) + (eq42_e1955 * s.dn[424][12]));
        let eq42_e1957_d_n13: f64 = ((eq42_e1955_d_n13 * s.v[424]) + (eq42_e1955 * s.dn[424][13]));
        let eq42_e1957_d_b0: f64 = ((eq42_e1955_d_b0 * s.v[424]) + (eq42_e1955 * s.db[424][0]));
        let eq42_e1957_d_b1: f64 = ((eq42_e1955_d_b1 * s.v[424]) + (eq42_e1955 * s.db[424][1]));
        let eq42_e1957_d_b2: f64 = ((eq42_e1955_d_b2 * s.v[424]) + (eq42_e1955 * s.db[424][2]));
        let eq42_e1957_d_b3: f64 = ((eq42_e1955_d_b3 * s.v[424]) + (eq42_e1955 * s.db[424][3]));
        let eq42_e1957_d_b4: f64 = ((eq42_e1955_d_b4 * s.v[424]) + (eq42_e1955 * s.db[424][4]));
        let eq42_e1957_d_b5: f64 = ((eq42_e1955_d_b5 * s.v[424]) + (eq42_e1955 * s.db[424][5]));
        let eq42_e1957_d_b6: f64 = ((eq42_e1955_d_b6 * s.v[424]) + (eq42_e1955 * s.db[424][6]));
        let eq42_e1957_d_b7: f64 = ((eq42_e1955_d_b7 * s.v[424]) + (eq42_e1955 * s.db[424][7]));
        let eq42_e1957_d_b8: f64 = ((eq42_e1955_d_b8 * s.v[424]) + (eq42_e1955 * s.db[424][8]));
        let eq42_e1957_d_b9: f64 = ((eq42_e1955_d_b9 * s.v[424]) + (eq42_e1955 * s.db[424][9]));
        let eq42_e1957_d_b10: f64 = ((eq42_e1955_d_b10 * s.v[424]) + (eq42_e1955 * s.db[424][10]));
        let eq42_e1957_d_b11: f64 = ((eq42_e1955_d_b11 * s.v[424]) + (eq42_e1955 * s.db[424][11]));
        let eq42_e1958_q: f64 = eq42_e1957;
        let eq42_reactive_node_derivatives: [f64; 14] = [eq42_e1957_d_n0, eq42_e1957_d_n1, eq42_e1957_d_n2, eq42_e1957_d_n3, eq42_e1957_d_n4, eq42_e1957_d_n5, eq42_e1957_d_n6, eq42_e1957_d_n7, eq42_e1957_d_n8, eq42_e1957_d_n9, eq42_e1957_d_n10, eq42_e1957_d_n11, eq42_e1957_d_n12, eq42_e1957_d_n13];
        let eq42_reactive_branch_derivatives: [f64; 12] = [eq42_e1957_d_b0, eq42_e1957_d_b1, eq42_e1957_d_b2, eq42_e1957_d_b3, eq42_e1957_d_b4, eq42_e1957_d_b5, eq42_e1957_d_b6, eq42_e1957_d_b7, eq42_e1957_d_b8, eq42_e1957_d_b9, eq42_e1957_d_b10, eq42_e1957_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e1960: f64 = (-s.v[379]);
        let eq43_e1960_d_n0: f64 = (-s.dn[379][0]);
        let eq43_e1960_d_n1: f64 = (-s.dn[379][1]);
        let eq43_e1960_d_n2: f64 = (-s.dn[379][2]);
        let eq43_e1960_d_n3: f64 = (-s.dn[379][3]);
        let eq43_e1960_d_n4: f64 = (-s.dn[379][4]);
        let eq43_e1960_d_n5: f64 = (-s.dn[379][5]);
        let eq43_e1960_d_n6: f64 = (-s.dn[379][6]);
        let eq43_e1960_d_n7: f64 = (-s.dn[379][7]);
        let eq43_e1960_d_n8: f64 = (-s.dn[379][8]);
        let eq43_e1960_d_n9: f64 = (-s.dn[379][9]);
        let eq43_e1960_d_n10: f64 = (-s.dn[379][10]);
        let eq43_e1960_d_n11: f64 = (-s.dn[379][11]);
        let eq43_e1960_d_n12: f64 = (-s.dn[379][12]);
        let eq43_e1960_d_n13: f64 = (-s.dn[379][13]);
        let eq43_e1960_d_b0: f64 = (-s.db[379][0]);
        let eq43_e1960_d_b1: f64 = (-s.db[379][1]);
        let eq43_e1960_d_b2: f64 = (-s.db[379][2]);
        let eq43_e1960_d_b3: f64 = (-s.db[379][3]);
        let eq43_e1960_d_b4: f64 = (-s.db[379][4]);
        let eq43_e1960_d_b5: f64 = (-s.db[379][5]);
        let eq43_e1960_d_b6: f64 = (-s.db[379][6]);
        let eq43_e1960_d_b7: f64 = (-s.db[379][7]);
        let eq43_e1960_d_b8: f64 = (-s.db[379][8]);
        let eq43_e1960_d_b9: f64 = (-s.db[379][9]);
        let eq43_e1960_d_b10: f64 = (-s.db[379][10]);
        let eq43_e1960_d_b11: f64 = (-s.db[379][11]);
        let eq43_e1962: f64 = (eq43_e1960 * s.v[421]);
        let eq43_e1962_d_n0: f64 = ((eq43_e1960_d_n0 * s.v[421]) + (eq43_e1960 * s.dn[421][0]));
        let eq43_e1962_d_n1: f64 = ((eq43_e1960_d_n1 * s.v[421]) + (eq43_e1960 * s.dn[421][1]));
        let eq43_e1962_d_n2: f64 = ((eq43_e1960_d_n2 * s.v[421]) + (eq43_e1960 * s.dn[421][2]));
        let eq43_e1962_d_n3: f64 = ((eq43_e1960_d_n3 * s.v[421]) + (eq43_e1960 * s.dn[421][3]));
        let eq43_e1962_d_n4: f64 = ((eq43_e1960_d_n4 * s.v[421]) + (eq43_e1960 * s.dn[421][4]));
        let eq43_e1962_d_n5: f64 = ((eq43_e1960_d_n5 * s.v[421]) + (eq43_e1960 * s.dn[421][5]));
        let eq43_e1962_d_n6: f64 = ((eq43_e1960_d_n6 * s.v[421]) + (eq43_e1960 * s.dn[421][6]));
        let eq43_e1962_d_n7: f64 = ((eq43_e1960_d_n7 * s.v[421]) + (eq43_e1960 * s.dn[421][7]));
        let eq43_e1962_d_n8: f64 = ((eq43_e1960_d_n8 * s.v[421]) + (eq43_e1960 * s.dn[421][8]));
        let eq43_e1962_d_n9: f64 = ((eq43_e1960_d_n9 * s.v[421]) + (eq43_e1960 * s.dn[421][9]));
        let eq43_e1962_d_n10: f64 = ((eq43_e1960_d_n10 * s.v[421]) + (eq43_e1960 * s.dn[421][10]));
        let eq43_e1962_d_n11: f64 = ((eq43_e1960_d_n11 * s.v[421]) + (eq43_e1960 * s.dn[421][11]));
        let eq43_e1962_d_n12: f64 = ((eq43_e1960_d_n12 * s.v[421]) + (eq43_e1960 * s.dn[421][12]));
        let eq43_e1962_d_n13: f64 = ((eq43_e1960_d_n13 * s.v[421]) + (eq43_e1960 * s.dn[421][13]));
        let eq43_e1962_d_b0: f64 = ((eq43_e1960_d_b0 * s.v[421]) + (eq43_e1960 * s.db[421][0]));
        let eq43_e1962_d_b1: f64 = ((eq43_e1960_d_b1 * s.v[421]) + (eq43_e1960 * s.db[421][1]));
        let eq43_e1962_d_b2: f64 = ((eq43_e1960_d_b2 * s.v[421]) + (eq43_e1960 * s.db[421][2]));
        let eq43_e1962_d_b3: f64 = ((eq43_e1960_d_b3 * s.v[421]) + (eq43_e1960 * s.db[421][3]));
        let eq43_e1962_d_b4: f64 = ((eq43_e1960_d_b4 * s.v[421]) + (eq43_e1960 * s.db[421][4]));
        let eq43_e1962_d_b5: f64 = ((eq43_e1960_d_b5 * s.v[421]) + (eq43_e1960 * s.db[421][5]));
        let eq43_e1962_d_b6: f64 = ((eq43_e1960_d_b6 * s.v[421]) + (eq43_e1960 * s.db[421][6]));
        let eq43_e1962_d_b7: f64 = ((eq43_e1960_d_b7 * s.v[421]) + (eq43_e1960 * s.db[421][7]));
        let eq43_e1962_d_b8: f64 = ((eq43_e1960_d_b8 * s.v[421]) + (eq43_e1960 * s.db[421][8]));
        let eq43_e1962_d_b9: f64 = ((eq43_e1960_d_b9 * s.v[421]) + (eq43_e1960 * s.db[421][9]));
        let eq43_e1962_d_b10: f64 = ((eq43_e1960_d_b10 * s.v[421]) + (eq43_e1960 * s.db[421][10]));
        let eq43_e1962_d_b11: f64 = ((eq43_e1960_d_b11 * s.v[421]) + (eq43_e1960 * s.db[421][11]));
        let eq43_e1963_q: f64 = eq43_e1962;
        let eq43_reactive_node_derivatives: [f64; 14] = [eq43_e1962_d_n0, eq43_e1962_d_n1, eq43_e1962_d_n2, eq43_e1962_d_n3, eq43_e1962_d_n4, eq43_e1962_d_n5, eq43_e1962_d_n6, eq43_e1962_d_n7, eq43_e1962_d_n8, eq43_e1962_d_n9, eq43_e1962_d_n10, eq43_e1962_d_n11, eq43_e1962_d_n12, eq43_e1962_d_n13];
        let eq43_reactive_branch_derivatives: [f64; 12] = [eq43_e1962_d_b0, eq43_e1962_d_b1, eq43_e1962_d_b2, eq43_e1962_d_b3, eq43_e1962_d_b4, eq43_e1962_d_b5, eq43_e1962_d_b6, eq43_e1962_d_b7, eq43_e1962_d_b8, eq43_e1962_d_b9, eq43_e1962_d_b10, eq43_e1962_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[10]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let eq45_e1969_q: f64 = s.v[1039];
        let eq45_e1970: f64 = (s.v[379] * s.v[1039]);
        let eq45_e1970_d_n0: f64 = ((s.dn[379][0] * s.v[1039]) + (s.v[379] * s.dn[1039][0]));
        let eq45_e1970_d_n1: f64 = ((s.dn[379][1] * s.v[1039]) + (s.v[379] * s.dn[1039][1]));
        let eq45_e1970_d_n2: f64 = ((s.dn[379][2] * s.v[1039]) + (s.v[379] * s.dn[1039][2]));
        let eq45_e1970_d_n3: f64 = ((s.dn[379][3] * s.v[1039]) + (s.v[379] * s.dn[1039][3]));
        let eq45_e1970_d_n4: f64 = ((s.dn[379][4] * s.v[1039]) + (s.v[379] * s.dn[1039][4]));
        let eq45_e1970_d_n5: f64 = ((s.dn[379][5] * s.v[1039]) + (s.v[379] * s.dn[1039][5]));
        let eq45_e1970_d_n6: f64 = ((s.dn[379][6] * s.v[1039]) + (s.v[379] * s.dn[1039][6]));
        let eq45_e1970_d_n7: f64 = ((s.dn[379][7] * s.v[1039]) + (s.v[379] * s.dn[1039][7]));
        let eq45_e1970_d_n8: f64 = ((s.dn[379][8] * s.v[1039]) + (s.v[379] * s.dn[1039][8]));
        let eq45_e1970_d_n9: f64 = ((s.dn[379][9] * s.v[1039]) + (s.v[379] * s.dn[1039][9]));
        let eq45_e1970_d_n10: f64 = ((s.dn[379][10] * s.v[1039]) + (s.v[379] * s.dn[1039][10]));
        let eq45_e1970_d_n11: f64 = ((s.dn[379][11] * s.v[1039]) + (s.v[379] * s.dn[1039][11]));
        let eq45_e1970_d_n12: f64 = ((s.dn[379][12] * s.v[1039]) + (s.v[379] * s.dn[1039][12]));
        let eq45_e1970_d_n13: f64 = ((s.dn[379][13] * s.v[1039]) + (s.v[379] * s.dn[1039][13]));
        let eq45_e1970_d_b0: f64 = ((s.db[379][0] * s.v[1039]) + (s.v[379] * s.db[1039][0]));
        let eq45_e1970_d_b1: f64 = ((s.db[379][1] * s.v[1039]) + (s.v[379] * s.db[1039][1]));
        let eq45_e1970_d_b2: f64 = ((s.db[379][2] * s.v[1039]) + (s.v[379] * s.db[1039][2]));
        let eq45_e1970_d_b3: f64 = ((s.db[379][3] * s.v[1039]) + (s.v[379] * s.db[1039][3]));
        let eq45_e1970_d_b4: f64 = ((s.db[379][4] * s.v[1039]) + (s.v[379] * s.db[1039][4]));
        let eq45_e1970_d_b5: f64 = ((s.db[379][5] * s.v[1039]) + (s.v[379] * s.db[1039][5]));
        let eq45_e1970_d_b6: f64 = ((s.db[379][6] * s.v[1039]) + (s.v[379] * s.db[1039][6]));
        let eq45_e1970_d_b7: f64 = ((s.db[379][7] * s.v[1039]) + (s.v[379] * s.db[1039][7]));
        let eq45_e1970_d_b8: f64 = ((s.db[379][8] * s.v[1039]) + (s.v[379] * s.db[1039][8]));
        let eq45_e1970_d_b9: f64 = ((s.db[379][9] * s.v[1039]) + (s.v[379] * s.db[1039][9]));
        let eq45_e1970_d_b10: f64 = ((s.db[379][10] * s.v[1039]) + (s.v[379] * s.db[1039][10]));
        let eq45_e1970_d_b11: f64 = ((s.db[379][11] * s.v[1039]) + (s.v[379] * s.db[1039][11]));
        let eq45_e1970_q: f64 = (s.v[379] * eq45_e1969_q);
        let eq45_e1970_q_d_n0: f64 = ((s.dn[379][0] * eq45_e1969_q) + (s.v[379] * s.dn[1039][0]));
        let eq45_e1970_q_d_n1: f64 = ((s.dn[379][1] * eq45_e1969_q) + (s.v[379] * s.dn[1039][1]));
        let eq45_e1970_q_d_n2: f64 = ((s.dn[379][2] * eq45_e1969_q) + (s.v[379] * s.dn[1039][2]));
        let eq45_e1970_q_d_n3: f64 = ((s.dn[379][3] * eq45_e1969_q) + (s.v[379] * s.dn[1039][3]));
        let eq45_e1970_q_d_n4: f64 = ((s.dn[379][4] * eq45_e1969_q) + (s.v[379] * s.dn[1039][4]));
        let eq45_e1970_q_d_n5: f64 = ((s.dn[379][5] * eq45_e1969_q) + (s.v[379] * s.dn[1039][5]));
        let eq45_e1970_q_d_n6: f64 = ((s.dn[379][6] * eq45_e1969_q) + (s.v[379] * s.dn[1039][6]));
        let eq45_e1970_q_d_n7: f64 = ((s.dn[379][7] * eq45_e1969_q) + (s.v[379] * s.dn[1039][7]));
        let eq45_e1970_q_d_n8: f64 = ((s.dn[379][8] * eq45_e1969_q) + (s.v[379] * s.dn[1039][8]));
        let eq45_e1970_q_d_n9: f64 = ((s.dn[379][9] * eq45_e1969_q) + (s.v[379] * s.dn[1039][9]));
        let eq45_e1970_q_d_n10: f64 = ((s.dn[379][10] * eq45_e1969_q) + (s.v[379] * s.dn[1039][10]));
        let eq45_e1970_q_d_n11: f64 = ((s.dn[379][11] * eq45_e1969_q) + (s.v[379] * s.dn[1039][11]));
        let eq45_e1970_q_d_n12: f64 = ((s.dn[379][12] * eq45_e1969_q) + (s.v[379] * s.dn[1039][12]));
        let eq45_e1970_q_d_n13: f64 = ((s.dn[379][13] * eq45_e1969_q) + (s.v[379] * s.dn[1039][13]));
        let eq45_e1970_q_d_b0: f64 = ((s.db[379][0] * eq45_e1969_q) + (s.v[379] * s.db[1039][0]));
        let eq45_e1970_q_d_b1: f64 = ((s.db[379][1] * eq45_e1969_q) + (s.v[379] * s.db[1039][1]));
        let eq45_e1970_q_d_b2: f64 = ((s.db[379][2] * eq45_e1969_q) + (s.v[379] * s.db[1039][2]));
        let eq45_e1970_q_d_b3: f64 = ((s.db[379][3] * eq45_e1969_q) + (s.v[379] * s.db[1039][3]));
        let eq45_e1970_q_d_b4: f64 = ((s.db[379][4] * eq45_e1969_q) + (s.v[379] * s.db[1039][4]));
        let eq45_e1970_q_d_b5: f64 = ((s.db[379][5] * eq45_e1969_q) + (s.v[379] * s.db[1039][5]));
        let eq45_e1970_q_d_b6: f64 = ((s.db[379][6] * eq45_e1969_q) + (s.v[379] * s.db[1039][6]));
        let eq45_e1970_q_d_b7: f64 = ((s.db[379][7] * eq45_e1969_q) + (s.v[379] * s.db[1039][7]));
        let eq45_e1970_q_d_b8: f64 = ((s.db[379][8] * eq45_e1969_q) + (s.v[379] * s.db[1039][8]));
        let eq45_e1970_q_d_b9: f64 = ((s.db[379][9] * eq45_e1969_q) + (s.v[379] * s.db[1039][9]));
        let eq45_e1970_q_d_b10: f64 = ((s.db[379][10] * eq45_e1969_q) + (s.v[379] * s.db[1039][10]));
        let eq45_e1970_q_d_b11: f64 = ((s.db[379][11] * eq45_e1969_q) + (s.v[379] * s.db[1039][11]));
        let eq45_reactive_node_derivatives: [f64; 14] = [eq45_e1970_q_d_n0, eq45_e1970_q_d_n1, eq45_e1970_q_d_n2, eq45_e1970_q_d_n3, eq45_e1970_q_d_n4, eq45_e1970_q_d_n5, eq45_e1970_q_d_n6, eq45_e1970_q_d_n7, eq45_e1970_q_d_n8, eq45_e1970_q_d_n9, eq45_e1970_q_d_n10, eq45_e1970_q_d_n11, eq45_e1970_q_d_n12, eq45_e1970_q_d_n13];
        let eq45_reactive_branch_derivatives: [f64; 12] = [eq45_e1970_q_d_b0, eq45_e1970_q_d_b1, eq45_e1970_q_d_b2, eq45_e1970_q_d_b3, eq45_e1970_q_d_b4, eq45_e1970_q_d_b5, eq45_e1970_q_d_b6, eq45_e1970_q_d_b7, eq45_e1970_q_d_b8, eq45_e1970_q_d_b9, eq45_e1970_q_d_b10, eq45_e1970_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1972_q: f64 = s.v[1047];
        let eq46_reactive_node_derivatives: [f64; 14] = [s.dn[1047][0], s.dn[1047][1], s.dn[1047][2], s.dn[1047][3], s.dn[1047][4], s.dn[1047][5], s.dn[1047][6], s.dn[1047][7], s.dn[1047][8], s.dn[1047][9], s.dn[1047][10], s.dn[1047][11], s.dn[1047][12], s.dn[1047][13]];
        let eq46_reactive_branch_derivatives: [f64; 12] = [s.db[1047][0], s.db[1047][1], s.db[1047][2], s.db[1047][3], s.db[1047][4], s.db[1047][5], s.db[1047][6], s.db[1047][7], s.db[1047][8], s.db[1047][9], s.db[1047][10], s.db[1047][11]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let eq47_e1974_q: f64 = s.v[1046];
        let eq47_reactive_node_derivatives: [f64; 14] = [s.dn[1046][0], s.dn[1046][1], s.dn[1046][2], s.dn[1046][3], s.dn[1046][4], s.dn[1046][5], s.dn[1046][6], s.dn[1046][7], s.dn[1046][8], s.dn[1046][9], s.dn[1046][10], s.dn[1046][11], s.dn[1046][12], s.dn[1046][13]];
        let eq47_reactive_branch_derivatives: [f64; 12] = [s.db[1046][0], s.db[1046][1], s.db[1046][2], s.db[1046][3], s.db[1046][4], s.db[1046][5], s.db[1046][6], s.db[1046][7], s.db[1046][8], s.db[1046][9], s.db[1046][10], s.db[1046][11]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n1, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11, eq67_e2103_d_n12, eq67_e2103_d_n13, eq67_e2103_d_b0, eq67_e2103_d_b1, eq67_e2103_d_b2, eq67_e2103_d_b3, eq67_e2103_d_b4, eq67_e2103_d_b5, eq67_e2103_d_b6, eq67_e2103_d_b7, eq67_e2103_d_b8, eq67_e2103_d_b9, eq67_e2103_d_b10, eq67_e2103_d_b11, eq67_e2103_q, eq67_e2103_q_d_n0, eq67_e2103_q_d_n1, eq67_e2103_q_d_n2, eq67_e2103_q_d_n3, eq67_e2103_q_d_n4, eq67_e2103_q_d_n5, eq67_e2103_q_d_n6, eq67_e2103_q_d_n7, eq67_e2103_q_d_n8, eq67_e2103_q_d_n9, eq67_e2103_q_d_n10, eq67_e2103_q_d_n11, eq67_e2103_q_d_n12, eq67_e2103_q_d_n13, eq67_e2103_q_d_b0, eq67_e2103_q_d_b1, eq67_e2103_q_d_b2, eq67_e2103_q_d_b3, eq67_e2103_q_d_b4, eq67_e2103_q_d_b5, eq67_e2103_q_d_b6, eq67_e2103_q_d_b7, eq67_e2103_q_d_b8, eq67_e2103_q_d_b9, eq67_e2103_q_d_b10, eq67_e2103_q_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && s.b[2025]) {
        let eq67_e2094: f64 = (s.v[634] * s.v[1015]);
        let eq67_e2094_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq67_e2094_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq67_e2094_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq67_e2094_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq67_e2094_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq67_e2094_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq67_e2094_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq67_e2094_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq67_e2094_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq67_e2094_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq67_e2094_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq67_e2094_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq67_e2094_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq67_e2094_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq67_e2094_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));
        let eq67_e2094_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));
        let eq67_e2094_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));
        let eq67_e2094_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));
        let eq67_e2094_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));
        let eq67_e2094_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));
        let eq67_e2094_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));
        let eq67_e2094_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));
        let eq67_e2094_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));
        let eq67_e2094_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));
        let eq67_e2094_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));
        let eq67_e2094_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));
        let eq67_e2097: f64 = (s.v[634] * s.v[1016]);
        let eq67_e2097_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq67_e2097_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq67_e2097_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq67_e2097_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq67_e2097_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq67_e2097_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq67_e2097_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq67_e2097_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq67_e2097_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq67_e2097_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq67_e2097_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq67_e2097_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq67_e2097_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq67_e2097_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq67_e2097_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));
        let eq67_e2097_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));
        let eq67_e2097_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));
        let eq67_e2097_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));
        let eq67_e2097_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));
        let eq67_e2097_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq67_e2097_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));
        let eq67_e2097_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));
        let eq67_e2097_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));
        let eq67_e2097_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));
        let eq67_e2097_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));
        let eq67_e2097_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));
        let eq67_e2098_q: f64 = eq67_e2097;
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2097);
        let eq67_e2099_d_n0: f64 = (eq67_e2094_d_n0 + eq67_e2097_d_n0);
        let eq67_e2099_d_n1: f64 = (eq67_e2094_d_n1 + eq67_e2097_d_n1);
        let eq67_e2099_d_n2: f64 = (eq67_e2094_d_n2 + eq67_e2097_d_n2);
        let eq67_e2099_d_n3: f64 = (eq67_e2094_d_n3 + eq67_e2097_d_n3);
        let eq67_e2099_d_n4: f64 = (eq67_e2094_d_n4 + eq67_e2097_d_n4);
        let eq67_e2099_d_n5: f64 = (eq67_e2094_d_n5 + eq67_e2097_d_n5);
        let eq67_e2099_d_n6: f64 = (eq67_e2094_d_n6 + eq67_e2097_d_n6);
        let eq67_e2099_d_n7: f64 = (eq67_e2094_d_n7 + eq67_e2097_d_n7);
        let eq67_e2099_d_n8: f64 = (eq67_e2094_d_n8 + eq67_e2097_d_n8);
        let eq67_e2099_d_n9: f64 = (eq67_e2094_d_n9 + eq67_e2097_d_n9);
        let eq67_e2099_d_n10: f64 = (eq67_e2094_d_n10 + eq67_e2097_d_n10);
        let eq67_e2099_d_n11: f64 = (eq67_e2094_d_n11 + eq67_e2097_d_n11);
        let eq67_e2099_d_n12: f64 = (eq67_e2094_d_n12 + eq67_e2097_d_n12);
        let eq67_e2099_d_n13: f64 = (eq67_e2094_d_n13 + eq67_e2097_d_n13);
        let eq67_e2099_d_b0: f64 = (eq67_e2094_d_b0 + eq67_e2097_d_b0);
        let eq67_e2099_d_b1: f64 = (eq67_e2094_d_b1 + eq67_e2097_d_b1);
        let eq67_e2099_d_b2: f64 = (eq67_e2094_d_b2 + eq67_e2097_d_b2);
        let eq67_e2099_d_b3: f64 = (eq67_e2094_d_b3 + eq67_e2097_d_b3);
        let eq67_e2099_d_b4: f64 = (eq67_e2094_d_b4 + eq67_e2097_d_b4);
        let eq67_e2099_d_b5: f64 = (eq67_e2094_d_b5 + eq67_e2097_d_b5);
        let eq67_e2099_d_b6: f64 = (eq67_e2094_d_b6 + eq67_e2097_d_b6);
        let eq67_e2099_d_b7: f64 = (eq67_e2094_d_b7 + eq67_e2097_d_b7);
        let eq67_e2099_d_b8: f64 = (eq67_e2094_d_b8 + eq67_e2097_d_b8);
        let eq67_e2099_d_b9: f64 = (eq67_e2094_d_b9 + eq67_e2097_d_b9);
        let eq67_e2099_d_b10: f64 = (eq67_e2094_d_b10 + eq67_e2097_d_b10);
        let eq67_e2099_d_b11: f64 = (eq67_e2094_d_b11 + eq67_e2097_d_b11);
        let eq67_e2099_q: f64 = eq67_e2098_q;
        let eq67_e2101: f64 = (eq67_e2099 - s.v[1017]);
        let eq67_e2101_d_n0: f64 = (eq67_e2099_d_n0 - s.dn[1017][0]);
        let eq67_e2101_d_n1: f64 = (eq67_e2099_d_n1 - s.dn[1017][1]);
        let eq67_e2101_d_n2: f64 = (eq67_e2099_d_n2 - s.dn[1017][2]);
        let eq67_e2101_d_n3: f64 = (eq67_e2099_d_n3 - s.dn[1017][3]);
        let eq67_e2101_d_n4: f64 = (eq67_e2099_d_n4 - s.dn[1017][4]);
        let eq67_e2101_d_n5: f64 = (eq67_e2099_d_n5 - s.dn[1017][5]);
        let eq67_e2101_d_n6: f64 = (eq67_e2099_d_n6 - s.dn[1017][6]);
        let eq67_e2101_d_n7: f64 = (eq67_e2099_d_n7 - s.dn[1017][7]);
        let eq67_e2101_d_n8: f64 = (eq67_e2099_d_n8 - s.dn[1017][8]);
        let eq67_e2101_d_n9: f64 = (eq67_e2099_d_n9 - s.dn[1017][9]);
        let eq67_e2101_d_n10: f64 = (eq67_e2099_d_n10 - s.dn[1017][10]);
        let eq67_e2101_d_n11: f64 = (eq67_e2099_d_n11 - s.dn[1017][11]);
        let eq67_e2101_d_n12: f64 = (eq67_e2099_d_n12 - s.dn[1017][12]);
        let eq67_e2101_d_n13: f64 = (eq67_e2099_d_n13 - s.dn[1017][13]);
        let eq67_e2101_d_b0: f64 = (eq67_e2099_d_b0 - s.db[1017][0]);
        let eq67_e2101_d_b1: f64 = (eq67_e2099_d_b1 - s.db[1017][1]);
        let eq67_e2101_d_b2: f64 = (eq67_e2099_d_b2 - s.db[1017][2]);
        let eq67_e2101_d_b3: f64 = (eq67_e2099_d_b3 - s.db[1017][3]);
        let eq67_e2101_d_b4: f64 = (eq67_e2099_d_b4 - s.db[1017][4]);
        let eq67_e2101_d_b5: f64 = (eq67_e2099_d_b5 - s.db[1017][5]);
        let eq67_e2101_d_b6: f64 = (eq67_e2099_d_b6 - s.db[1017][6]);
        let eq67_e2101_d_b7: f64 = (eq67_e2099_d_b7 - s.db[1017][7]);
        let eq67_e2101_d_b8: f64 = (eq67_e2099_d_b8 - s.db[1017][8]);
        let eq67_e2101_d_b9: f64 = (eq67_e2099_d_b9 - s.db[1017][9]);
        let eq67_e2101_d_b10: f64 = (eq67_e2099_d_b10 - s.db[1017][10]);
        let eq67_e2101_d_b11: f64 = (eq67_e2099_d_b11 - s.db[1017][11]);
        let eq67_e2101_q: f64 = eq67_e2099_q;
        (eq67_e2101, eq67_e2101_d_n0, eq67_e2101_d_n1, eq67_e2101_d_n2, eq67_e2101_d_n3, eq67_e2101_d_n4, eq67_e2101_d_n5, eq67_e2101_d_n6, eq67_e2101_d_n7, eq67_e2101_d_n8, eq67_e2101_d_n9, eq67_e2101_d_n10, eq67_e2101_d_n11, eq67_e2101_d_n12, eq67_e2101_d_n13, eq67_e2101_d_b0, eq67_e2101_d_b1, eq67_e2101_d_b2, eq67_e2101_d_b3, eq67_e2101_d_b4, eq67_e2101_d_b5, eq67_e2101_d_b6, eq67_e2101_d_b7, eq67_e2101_d_b8, eq67_e2101_d_b9, eq67_e2101_d_b10, eq67_e2101_d_b11, eq67_e2101_q, eq67_e2097_d_n0, eq67_e2097_d_n1, eq67_e2097_d_n2, eq67_e2097_d_n3, eq67_e2097_d_n4, eq67_e2097_d_n5, eq67_e2097_d_n6, eq67_e2097_d_n7, eq67_e2097_d_n8, eq67_e2097_d_n9, eq67_e2097_d_n10, eq67_e2097_d_n11, eq67_e2097_d_n12, eq67_e2097_d_n13, eq67_e2097_d_b0, eq67_e2097_d_b1, eq67_e2097_d_b2, eq67_e2097_d_b3, eq67_e2097_d_b4, eq67_e2097_d_b5, eq67_e2097_d_b6, eq67_e2097_d_b7, eq67_e2097_d_b8, eq67_e2097_d_b9, eq67_e2097_d_b10, eq67_e2097_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_reactive_node_derivatives: [f64; 14] = [eq67_e2103_q_d_n0, eq67_e2103_q_d_n1, eq67_e2103_q_d_n2, eq67_e2103_q_d_n3, eq67_e2103_q_d_n4, eq67_e2103_q_d_n5, eq67_e2103_q_d_n6, eq67_e2103_q_d_n7, eq67_e2103_q_d_n8, eq67_e2103_q_d_n9, eq67_e2103_q_d_n10, eq67_e2103_q_d_n11, eq67_e2103_q_d_n12, eq67_e2103_q_d_n13];
        let eq67_reactive_branch_derivatives: [f64; 12] = [eq67_e2103_q_d_b0, eq67_e2103_q_d_b1, eq67_e2103_q_d_b2, eq67_e2103_q_d_b3, eq67_e2103_q_d_b4, eq67_e2103_q_d_b5, eq67_e2103_q_d_b6, eq67_e2103_q_d_b7, eq67_e2103_q_d_b8, eq67_e2103_q_d_b9, eq67_e2103_q_d_b10, eq67_e2103_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq67_reactive_node_derivatives,
            branches,
            &eq67_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_4(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n1, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11, eq68_e2121_d_n12, eq68_e2121_d_n13, eq68_e2121_d_b0, eq68_e2121_d_b1, eq68_e2121_d_b2, eq68_e2121_d_b3, eq68_e2121_d_b4, eq68_e2121_d_b5, eq68_e2121_d_b6, eq68_e2121_d_b7, eq68_e2121_d_b8, eq68_e2121_d_b9, eq68_e2121_d_b10, eq68_e2121_d_b11, eq68_e2121_q, eq68_e2121_q_d_n0, eq68_e2121_q_d_n1, eq68_e2121_q_d_n2, eq68_e2121_q_d_n3, eq68_e2121_q_d_n4, eq68_e2121_q_d_n5, eq68_e2121_q_d_n6, eq68_e2121_q_d_n7, eq68_e2121_q_d_n8, eq68_e2121_q_d_n9, eq68_e2121_q_d_n10, eq68_e2121_q_d_n11, eq68_e2121_q_d_n12, eq68_e2121_q_d_n13, eq68_e2121_q_d_b0, eq68_e2121_q_d_b1, eq68_e2121_q_d_b2, eq68_e2121_q_d_b3, eq68_e2121_q_d_b4, eq68_e2121_q_d_b5, eq68_e2121_q_d_b6, eq68_e2121_q_d_b7, eq68_e2121_q_d_b8, eq68_e2121_q_d_b9, eq68_e2121_q_d_b10, eq68_e2121_q_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && (!s.b[2025])) {
        let eq68_e2112: f64 = (s.v[634] * s.v[1015]);
        let eq68_e2112_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq68_e2112_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq68_e2112_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq68_e2112_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq68_e2112_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq68_e2112_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq68_e2112_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq68_e2112_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq68_e2112_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq68_e2112_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq68_e2112_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq68_e2112_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq68_e2112_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq68_e2112_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq68_e2112_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));
        let eq68_e2112_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));
        let eq68_e2112_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));
        let eq68_e2112_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));
        let eq68_e2112_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));
        let eq68_e2112_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));
        let eq68_e2112_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));
        let eq68_e2112_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));
        let eq68_e2112_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));
        let eq68_e2112_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));
        let eq68_e2112_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));
        let eq68_e2112_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));
        let eq68_e2115: f64 = (s.v[634] * s.v[1016]);
        let eq68_e2115_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq68_e2115_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq68_e2115_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq68_e2115_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq68_e2115_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq68_e2115_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq68_e2115_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq68_e2115_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq68_e2115_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq68_e2115_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq68_e2115_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq68_e2115_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq68_e2115_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq68_e2115_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq68_e2115_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));
        let eq68_e2115_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));
        let eq68_e2115_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));
        let eq68_e2115_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));
        let eq68_e2115_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));
        let eq68_e2115_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq68_e2115_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));
        let eq68_e2115_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));
        let eq68_e2115_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));
        let eq68_e2115_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));
        let eq68_e2115_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));
        let eq68_e2115_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));
        let eq68_e2116_q: f64 = eq68_e2115;
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2115);
        let eq68_e2117_d_n0: f64 = (eq68_e2112_d_n0 + eq68_e2115_d_n0);
        let eq68_e2117_d_n1: f64 = (eq68_e2112_d_n1 + eq68_e2115_d_n1);
        let eq68_e2117_d_n2: f64 = (eq68_e2112_d_n2 + eq68_e2115_d_n2);
        let eq68_e2117_d_n3: f64 = (eq68_e2112_d_n3 + eq68_e2115_d_n3);
        let eq68_e2117_d_n4: f64 = (eq68_e2112_d_n4 + eq68_e2115_d_n4);
        let eq68_e2117_d_n5: f64 = (eq68_e2112_d_n5 + eq68_e2115_d_n5);
        let eq68_e2117_d_n6: f64 = (eq68_e2112_d_n6 + eq68_e2115_d_n6);
        let eq68_e2117_d_n7: f64 = (eq68_e2112_d_n7 + eq68_e2115_d_n7);
        let eq68_e2117_d_n8: f64 = (eq68_e2112_d_n8 + eq68_e2115_d_n8);
        let eq68_e2117_d_n9: f64 = (eq68_e2112_d_n9 + eq68_e2115_d_n9);
        let eq68_e2117_d_n10: f64 = (eq68_e2112_d_n10 + eq68_e2115_d_n10);
        let eq68_e2117_d_n11: f64 = (eq68_e2112_d_n11 + eq68_e2115_d_n11);
        let eq68_e2117_d_n12: f64 = (eq68_e2112_d_n12 + eq68_e2115_d_n12);
        let eq68_e2117_d_n13: f64 = (eq68_e2112_d_n13 + eq68_e2115_d_n13);
        let eq68_e2117_d_b0: f64 = (eq68_e2112_d_b0 + eq68_e2115_d_b0);
        let eq68_e2117_d_b1: f64 = (eq68_e2112_d_b1 + eq68_e2115_d_b1);
        let eq68_e2117_d_b2: f64 = (eq68_e2112_d_b2 + eq68_e2115_d_b2);
        let eq68_e2117_d_b3: f64 = (eq68_e2112_d_b3 + eq68_e2115_d_b3);
        let eq68_e2117_d_b4: f64 = (eq68_e2112_d_b4 + eq68_e2115_d_b4);
        let eq68_e2117_d_b5: f64 = (eq68_e2112_d_b5 + eq68_e2115_d_b5);
        let eq68_e2117_d_b6: f64 = (eq68_e2112_d_b6 + eq68_e2115_d_b6);
        let eq68_e2117_d_b7: f64 = (eq68_e2112_d_b7 + eq68_e2115_d_b7);
        let eq68_e2117_d_b8: f64 = (eq68_e2112_d_b8 + eq68_e2115_d_b8);
        let eq68_e2117_d_b9: f64 = (eq68_e2112_d_b9 + eq68_e2115_d_b9);
        let eq68_e2117_d_b10: f64 = (eq68_e2112_d_b10 + eq68_e2115_d_b10);
        let eq68_e2117_d_b11: f64 = (eq68_e2112_d_b11 + eq68_e2115_d_b11);
        let eq68_e2117_q: f64 = eq68_e2116_q;
        let eq68_e2119: f64 = (eq68_e2117 - s.v[1017]);
        let eq68_e2119_d_n0: f64 = (eq68_e2117_d_n0 - s.dn[1017][0]);
        let eq68_e2119_d_n1: f64 = (eq68_e2117_d_n1 - s.dn[1017][1]);
        let eq68_e2119_d_n2: f64 = (eq68_e2117_d_n2 - s.dn[1017][2]);
        let eq68_e2119_d_n3: f64 = (eq68_e2117_d_n3 - s.dn[1017][3]);
        let eq68_e2119_d_n4: f64 = (eq68_e2117_d_n4 - s.dn[1017][4]);
        let eq68_e2119_d_n5: f64 = (eq68_e2117_d_n5 - s.dn[1017][5]);
        let eq68_e2119_d_n6: f64 = (eq68_e2117_d_n6 - s.dn[1017][6]);
        let eq68_e2119_d_n7: f64 = (eq68_e2117_d_n7 - s.dn[1017][7]);
        let eq68_e2119_d_n8: f64 = (eq68_e2117_d_n8 - s.dn[1017][8]);
        let eq68_e2119_d_n9: f64 = (eq68_e2117_d_n9 - s.dn[1017][9]);
        let eq68_e2119_d_n10: f64 = (eq68_e2117_d_n10 - s.dn[1017][10]);
        let eq68_e2119_d_n11: f64 = (eq68_e2117_d_n11 - s.dn[1017][11]);
        let eq68_e2119_d_n12: f64 = (eq68_e2117_d_n12 - s.dn[1017][12]);
        let eq68_e2119_d_n13: f64 = (eq68_e2117_d_n13 - s.dn[1017][13]);
        let eq68_e2119_d_b0: f64 = (eq68_e2117_d_b0 - s.db[1017][0]);
        let eq68_e2119_d_b1: f64 = (eq68_e2117_d_b1 - s.db[1017][1]);
        let eq68_e2119_d_b2: f64 = (eq68_e2117_d_b2 - s.db[1017][2]);
        let eq68_e2119_d_b3: f64 = (eq68_e2117_d_b3 - s.db[1017][3]);
        let eq68_e2119_d_b4: f64 = (eq68_e2117_d_b4 - s.db[1017][4]);
        let eq68_e2119_d_b5: f64 = (eq68_e2117_d_b5 - s.db[1017][5]);
        let eq68_e2119_d_b6: f64 = (eq68_e2117_d_b6 - s.db[1017][6]);
        let eq68_e2119_d_b7: f64 = (eq68_e2117_d_b7 - s.db[1017][7]);
        let eq68_e2119_d_b8: f64 = (eq68_e2117_d_b8 - s.db[1017][8]);
        let eq68_e2119_d_b9: f64 = (eq68_e2117_d_b9 - s.db[1017][9]);
        let eq68_e2119_d_b10: f64 = (eq68_e2117_d_b10 - s.db[1017][10]);
        let eq68_e2119_d_b11: f64 = (eq68_e2117_d_b11 - s.db[1017][11]);
        let eq68_e2119_q: f64 = eq68_e2117_q;
        (eq68_e2119, eq68_e2119_d_n0, eq68_e2119_d_n1, eq68_e2119_d_n2, eq68_e2119_d_n3, eq68_e2119_d_n4, eq68_e2119_d_n5, eq68_e2119_d_n6, eq68_e2119_d_n7, eq68_e2119_d_n8, eq68_e2119_d_n9, eq68_e2119_d_n10, eq68_e2119_d_n11, eq68_e2119_d_n12, eq68_e2119_d_n13, eq68_e2119_d_b0, eq68_e2119_d_b1, eq68_e2119_d_b2, eq68_e2119_d_b3, eq68_e2119_d_b4, eq68_e2119_d_b5, eq68_e2119_d_b6, eq68_e2119_d_b7, eq68_e2119_d_b8, eq68_e2119_d_b9, eq68_e2119_d_b10, eq68_e2119_d_b11, eq68_e2119_q, eq68_e2115_d_n0, eq68_e2115_d_n1, eq68_e2115_d_n2, eq68_e2115_d_n3, eq68_e2115_d_n4, eq68_e2115_d_n5, eq68_e2115_d_n6, eq68_e2115_d_n7, eq68_e2115_d_n8, eq68_e2115_d_n9, eq68_e2115_d_n10, eq68_e2115_d_n11, eq68_e2115_d_n12, eq68_e2115_d_n13, eq68_e2115_d_b0, eq68_e2115_d_b1, eq68_e2115_d_b2, eq68_e2115_d_b3, eq68_e2115_d_b4, eq68_e2115_d_b5, eq68_e2115_d_b6, eq68_e2115_d_b7, eq68_e2115_d_b8, eq68_e2115_d_b9, eq68_e2115_d_b10, eq68_e2115_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_reactive_node_derivatives: [f64; 14] = [eq68_e2121_q_d_n0, eq68_e2121_q_d_n1, eq68_e2121_q_d_n2, eq68_e2121_q_d_n3, eq68_e2121_q_d_n4, eq68_e2121_q_d_n5, eq68_e2121_q_d_n6, eq68_e2121_q_d_n7, eq68_e2121_q_d_n8, eq68_e2121_q_d_n9, eq68_e2121_q_d_n10, eq68_e2121_q_d_n11, eq68_e2121_q_d_n12, eq68_e2121_q_d_n13];
        let eq68_reactive_branch_derivatives: [f64; 12] = [eq68_e2121_q_d_b0, eq68_e2121_q_d_b1, eq68_e2121_q_d_b2, eq68_e2121_q_d_b3, eq68_e2121_q_d_b4, eq68_e2121_q_d_b5, eq68_e2121_q_d_b6, eq68_e2121_q_d_b7, eq68_e2121_q_d_b8, eq68_e2121_q_d_b9, eq68_e2121_q_d_b10, eq68_e2121_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq68_reactive_node_derivatives,
            branches,
            &eq68_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n1, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11, eq69_e2137_d_n12, eq69_e2137_d_n13, eq69_e2137_d_b0, eq69_e2137_d_b1, eq69_e2137_d_b2, eq69_e2137_d_b3, eq69_e2137_d_b4, eq69_e2137_d_b5, eq69_e2137_d_b6, eq69_e2137_d_b7, eq69_e2137_d_b8, eq69_e2137_d_b9, eq69_e2137_d_b10, eq69_e2137_d_b11, eq69_e2137_q, eq69_e2137_q_d_n0, eq69_e2137_q_d_n1, eq69_e2137_q_d_n2, eq69_e2137_q_d_n3, eq69_e2137_q_d_n4, eq69_e2137_q_d_n5, eq69_e2137_q_d_n6, eq69_e2137_q_d_n7, eq69_e2137_q_d_n8, eq69_e2137_q_d_n9, eq69_e2137_q_d_n10, eq69_e2137_q_d_n11, eq69_e2137_q_d_n12, eq69_e2137_q_d_n13, eq69_e2137_q_d_b0, eq69_e2137_q_d_b1, eq69_e2137_q_d_b2, eq69_e2137_q_d_b3, eq69_e2137_q_d_b4, eq69_e2137_q_d_b5, eq69_e2137_q_d_b6, eq69_e2137_q_d_b7, eq69_e2137_q_d_b8, eq69_e2137_q_d_b9, eq69_e2137_q_d_b10, eq69_e2137_q_d_b11,) = {
    if (s.b[2021] && (!s.b[2024])) {
        let eq69_e2128: f64 = (s.v[634] * s.v[1015]);
        let eq69_e2128_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq69_e2128_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq69_e2128_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq69_e2128_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq69_e2128_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq69_e2128_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq69_e2128_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq69_e2128_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq69_e2128_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq69_e2128_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq69_e2128_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq69_e2128_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq69_e2128_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq69_e2128_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq69_e2128_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));
        let eq69_e2128_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));
        let eq69_e2128_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));
        let eq69_e2128_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));
        let eq69_e2128_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));
        let eq69_e2128_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));
        let eq69_e2128_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));
        let eq69_e2128_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));
        let eq69_e2128_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));
        let eq69_e2128_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));
        let eq69_e2128_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));
        let eq69_e2128_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));
        let eq69_e2131: f64 = (s.v[634] * s.v[1016]);
        let eq69_e2131_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq69_e2131_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq69_e2131_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq69_e2131_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq69_e2131_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq69_e2131_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq69_e2131_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq69_e2131_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq69_e2131_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq69_e2131_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq69_e2131_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq69_e2131_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq69_e2131_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq69_e2131_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq69_e2131_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));
        let eq69_e2131_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));
        let eq69_e2131_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));
        let eq69_e2131_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));
        let eq69_e2131_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));
        let eq69_e2131_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq69_e2131_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));
        let eq69_e2131_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));
        let eq69_e2131_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));
        let eq69_e2131_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));
        let eq69_e2131_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));
        let eq69_e2131_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));
        let eq69_e2132_q: f64 = eq69_e2131;
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2131);
        let eq69_e2133_d_n0: f64 = (eq69_e2128_d_n0 + eq69_e2131_d_n0);
        let eq69_e2133_d_n1: f64 = (eq69_e2128_d_n1 + eq69_e2131_d_n1);
        let eq69_e2133_d_n2: f64 = (eq69_e2128_d_n2 + eq69_e2131_d_n2);
        let eq69_e2133_d_n3: f64 = (eq69_e2128_d_n3 + eq69_e2131_d_n3);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + eq69_e2131_d_n4);
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + eq69_e2131_d_n5);
        let eq69_e2133_d_n6: f64 = (eq69_e2128_d_n6 + eq69_e2131_d_n6);
        let eq69_e2133_d_n7: f64 = (eq69_e2128_d_n7 + eq69_e2131_d_n7);
        let eq69_e2133_d_n8: f64 = (eq69_e2128_d_n8 + eq69_e2131_d_n8);
        let eq69_e2133_d_n9: f64 = (eq69_e2128_d_n9 + eq69_e2131_d_n9);
        let eq69_e2133_d_n10: f64 = (eq69_e2128_d_n10 + eq69_e2131_d_n10);
        let eq69_e2133_d_n11: f64 = (eq69_e2128_d_n11 + eq69_e2131_d_n11);
        let eq69_e2133_d_n12: f64 = (eq69_e2128_d_n12 + eq69_e2131_d_n12);
        let eq69_e2133_d_n13: f64 = (eq69_e2128_d_n13 + eq69_e2131_d_n13);
        let eq69_e2133_d_b0: f64 = (eq69_e2128_d_b0 + eq69_e2131_d_b0);
        let eq69_e2133_d_b1: f64 = (eq69_e2128_d_b1 + eq69_e2131_d_b1);
        let eq69_e2133_d_b2: f64 = (eq69_e2128_d_b2 + eq69_e2131_d_b2);
        let eq69_e2133_d_b3: f64 = (eq69_e2128_d_b3 + eq69_e2131_d_b3);
        let eq69_e2133_d_b4: f64 = (eq69_e2128_d_b4 + eq69_e2131_d_b4);
        let eq69_e2133_d_b5: f64 = (eq69_e2128_d_b5 + eq69_e2131_d_b5);
        let eq69_e2133_d_b6: f64 = (eq69_e2128_d_b6 + eq69_e2131_d_b6);
        let eq69_e2133_d_b7: f64 = (eq69_e2128_d_b7 + eq69_e2131_d_b7);
        let eq69_e2133_d_b8: f64 = (eq69_e2128_d_b8 + eq69_e2131_d_b8);
        let eq69_e2133_d_b9: f64 = (eq69_e2128_d_b9 + eq69_e2131_d_b9);
        let eq69_e2133_d_b10: f64 = (eq69_e2128_d_b10 + eq69_e2131_d_b10);
        let eq69_e2133_d_b11: f64 = (eq69_e2128_d_b11 + eq69_e2131_d_b11);
        let eq69_e2133_q: f64 = eq69_e2132_q;
        let eq69_e2135: f64 = (eq69_e2133 - s.v[1017]);
        let eq69_e2135_d_n0: f64 = (eq69_e2133_d_n0 - s.dn[1017][0]);
        let eq69_e2135_d_n1: f64 = (eq69_e2133_d_n1 - s.dn[1017][1]);
        let eq69_e2135_d_n2: f64 = (eq69_e2133_d_n2 - s.dn[1017][2]);
        let eq69_e2135_d_n3: f64 = (eq69_e2133_d_n3 - s.dn[1017][3]);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - s.dn[1017][4]);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - s.dn[1017][5]);
        let eq69_e2135_d_n6: f64 = (eq69_e2133_d_n6 - s.dn[1017][6]);
        let eq69_e2135_d_n7: f64 = (eq69_e2133_d_n7 - s.dn[1017][7]);
        let eq69_e2135_d_n8: f64 = (eq69_e2133_d_n8 - s.dn[1017][8]);
        let eq69_e2135_d_n9: f64 = (eq69_e2133_d_n9 - s.dn[1017][9]);
        let eq69_e2135_d_n10: f64 = (eq69_e2133_d_n10 - s.dn[1017][10]);
        let eq69_e2135_d_n11: f64 = (eq69_e2133_d_n11 - s.dn[1017][11]);
        let eq69_e2135_d_n12: f64 = (eq69_e2133_d_n12 - s.dn[1017][12]);
        let eq69_e2135_d_n13: f64 = (eq69_e2133_d_n13 - s.dn[1017][13]);
        let eq69_e2135_d_b0: f64 = (eq69_e2133_d_b0 - s.db[1017][0]);
        let eq69_e2135_d_b1: f64 = (eq69_e2133_d_b1 - s.db[1017][1]);
        let eq69_e2135_d_b2: f64 = (eq69_e2133_d_b2 - s.db[1017][2]);
        let eq69_e2135_d_b3: f64 = (eq69_e2133_d_b3 - s.db[1017][3]);
        let eq69_e2135_d_b4: f64 = (eq69_e2133_d_b4 - s.db[1017][4]);
        let eq69_e2135_d_b5: f64 = (eq69_e2133_d_b5 - s.db[1017][5]);
        let eq69_e2135_d_b6: f64 = (eq69_e2133_d_b6 - s.db[1017][6]);
        let eq69_e2135_d_b7: f64 = (eq69_e2133_d_b7 - s.db[1017][7]);
        let eq69_e2135_d_b8: f64 = (eq69_e2133_d_b8 - s.db[1017][8]);
        let eq69_e2135_d_b9: f64 = (eq69_e2133_d_b9 - s.db[1017][9]);
        let eq69_e2135_d_b10: f64 = (eq69_e2133_d_b10 - s.db[1017][10]);
        let eq69_e2135_d_b11: f64 = (eq69_e2133_d_b11 - s.db[1017][11]);
        let eq69_e2135_q: f64 = eq69_e2133_q;
        (eq69_e2135, eq69_e2135_d_n0, eq69_e2135_d_n1, eq69_e2135_d_n2, eq69_e2135_d_n3, eq69_e2135_d_n4, eq69_e2135_d_n5, eq69_e2135_d_n6, eq69_e2135_d_n7, eq69_e2135_d_n8, eq69_e2135_d_n9, eq69_e2135_d_n10, eq69_e2135_d_n11, eq69_e2135_d_n12, eq69_e2135_d_n13, eq69_e2135_d_b0, eq69_e2135_d_b1, eq69_e2135_d_b2, eq69_e2135_d_b3, eq69_e2135_d_b4, eq69_e2135_d_b5, eq69_e2135_d_b6, eq69_e2135_d_b7, eq69_e2135_d_b8, eq69_e2135_d_b9, eq69_e2135_d_b10, eq69_e2135_d_b11, eq69_e2135_q, eq69_e2131_d_n0, eq69_e2131_d_n1, eq69_e2131_d_n2, eq69_e2131_d_n3, eq69_e2131_d_n4, eq69_e2131_d_n5, eq69_e2131_d_n6, eq69_e2131_d_n7, eq69_e2131_d_n8, eq69_e2131_d_n9, eq69_e2131_d_n10, eq69_e2131_d_n11, eq69_e2131_d_n12, eq69_e2131_d_n13, eq69_e2131_d_b0, eq69_e2131_d_b1, eq69_e2131_d_b2, eq69_e2131_d_b3, eq69_e2131_d_b4, eq69_e2131_d_b5, eq69_e2131_d_b6, eq69_e2131_d_b7, eq69_e2131_d_b8, eq69_e2131_d_b9, eq69_e2131_d_b10, eq69_e2131_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_reactive_node_derivatives: [f64; 14] = [eq69_e2137_q_d_n0, eq69_e2137_q_d_n1, eq69_e2137_q_d_n2, eq69_e2137_q_d_n3, eq69_e2137_q_d_n4, eq69_e2137_q_d_n5, eq69_e2137_q_d_n6, eq69_e2137_q_d_n7, eq69_e2137_q_d_n8, eq69_e2137_q_d_n9, eq69_e2137_q_d_n10, eq69_e2137_q_d_n11, eq69_e2137_q_d_n12, eq69_e2137_q_d_n13];
        let eq69_reactive_branch_derivatives: [f64; 12] = [eq69_e2137_q_d_b0, eq69_e2137_q_d_b1, eq69_e2137_q_d_b2, eq69_e2137_q_d_b3, eq69_e2137_q_d_b4, eq69_e2137_q_d_b5, eq69_e2137_q_d_b6, eq69_e2137_q_d_b7, eq69_e2137_q_d_b8, eq69_e2137_q_d_b9, eq69_e2137_q_d_b10, eq69_e2137_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq69_reactive_node_derivatives,
            branches,
            &eq69_reactive_branch_derivatives,
            multiplicity,
        );
        let eq80_e2212_q: f64 = s.v[520];
        let eq80_e2213: f64 = (s.v[379] * s.v[520]);
        let eq80_e2213_d_n0: f64 = ((s.dn[379][0] * s.v[520]) + (s.v[379] * s.dn[520][0]));
        let eq80_e2213_d_n1: f64 = ((s.dn[379][1] * s.v[520]) + (s.v[379] * s.dn[520][1]));
        let eq80_e2213_d_n2: f64 = ((s.dn[379][2] * s.v[520]) + (s.v[379] * s.dn[520][2]));
        let eq80_e2213_d_n3: f64 = ((s.dn[379][3] * s.v[520]) + (s.v[379] * s.dn[520][3]));
        let eq80_e2213_d_n4: f64 = ((s.dn[379][4] * s.v[520]) + (s.v[379] * s.dn[520][4]));
        let eq80_e2213_d_n5: f64 = ((s.dn[379][5] * s.v[520]) + (s.v[379] * s.dn[520][5]));
        let eq80_e2213_d_n6: f64 = ((s.dn[379][6] * s.v[520]) + (s.v[379] * s.dn[520][6]));
        let eq80_e2213_d_n7: f64 = ((s.dn[379][7] * s.v[520]) + (s.v[379] * s.dn[520][7]));
        let eq80_e2213_d_n8: f64 = ((s.dn[379][8] * s.v[520]) + (s.v[379] * s.dn[520][8]));
        let eq80_e2213_d_n9: f64 = ((s.dn[379][9] * s.v[520]) + (s.v[379] * s.dn[520][9]));
        let eq80_e2213_d_n10: f64 = ((s.dn[379][10] * s.v[520]) + (s.v[379] * s.dn[520][10]));
        let eq80_e2213_d_n11: f64 = ((s.dn[379][11] * s.v[520]) + (s.v[379] * s.dn[520][11]));
        let eq80_e2213_d_n12: f64 = ((s.dn[379][12] * s.v[520]) + (s.v[379] * s.dn[520][12]));
        let eq80_e2213_d_n13: f64 = ((s.dn[379][13] * s.v[520]) + (s.v[379] * s.dn[520][13]));
        let eq80_e2213_d_b0: f64 = ((s.db[379][0] * s.v[520]) + (s.v[379] * s.db[520][0]));
        let eq80_e2213_d_b1: f64 = ((s.db[379][1] * s.v[520]) + (s.v[379] * s.db[520][1]));
        let eq80_e2213_d_b2: f64 = ((s.db[379][2] * s.v[520]) + (s.v[379] * s.db[520][2]));
        let eq80_e2213_d_b3: f64 = ((s.db[379][3] * s.v[520]) + (s.v[379] * s.db[520][3]));
        let eq80_e2213_d_b4: f64 = ((s.db[379][4] * s.v[520]) + (s.v[379] * s.db[520][4]));
        let eq80_e2213_d_b5: f64 = ((s.db[379][5] * s.v[520]) + (s.v[379] * s.db[520][5]));
        let eq80_e2213_d_b6: f64 = ((s.db[379][6] * s.v[520]) + (s.v[379] * s.db[520][6]));
        let eq80_e2213_d_b7: f64 = ((s.db[379][7] * s.v[520]) + (s.v[379] * s.db[520][7]));
        let eq80_e2213_d_b8: f64 = ((s.db[379][8] * s.v[520]) + (s.v[379] * s.db[520][8]));
        let eq80_e2213_d_b9: f64 = ((s.db[379][9] * s.v[520]) + (s.v[379] * s.db[520][9]));
        let eq80_e2213_d_b10: f64 = ((s.db[379][10] * s.v[520]) + (s.v[379] * s.db[520][10]));
        let eq80_e2213_d_b11: f64 = ((s.db[379][11] * s.v[520]) + (s.v[379] * s.db[520][11]));
        let eq80_e2213_q: f64 = (s.v[379] * eq80_e2212_q);
        let eq80_e2213_q_d_n0: f64 = ((s.dn[379][0] * eq80_e2212_q) + (s.v[379] * s.dn[520][0]));
        let eq80_e2213_q_d_n1: f64 = ((s.dn[379][1] * eq80_e2212_q) + (s.v[379] * s.dn[520][1]));
        let eq80_e2213_q_d_n2: f64 = ((s.dn[379][2] * eq80_e2212_q) + (s.v[379] * s.dn[520][2]));
        let eq80_e2213_q_d_n3: f64 = ((s.dn[379][3] * eq80_e2212_q) + (s.v[379] * s.dn[520][3]));
        let eq80_e2213_q_d_n4: f64 = ((s.dn[379][4] * eq80_e2212_q) + (s.v[379] * s.dn[520][4]));
        let eq80_e2213_q_d_n5: f64 = ((s.dn[379][5] * eq80_e2212_q) + (s.v[379] * s.dn[520][5]));
        let eq80_e2213_q_d_n6: f64 = ((s.dn[379][6] * eq80_e2212_q) + (s.v[379] * s.dn[520][6]));
        let eq80_e2213_q_d_n7: f64 = ((s.dn[379][7] * eq80_e2212_q) + (s.v[379] * s.dn[520][7]));
        let eq80_e2213_q_d_n8: f64 = ((s.dn[379][8] * eq80_e2212_q) + (s.v[379] * s.dn[520][8]));
        let eq80_e2213_q_d_n9: f64 = ((s.dn[379][9] * eq80_e2212_q) + (s.v[379] * s.dn[520][9]));
        let eq80_e2213_q_d_n10: f64 = ((s.dn[379][10] * eq80_e2212_q) + (s.v[379] * s.dn[520][10]));
        let eq80_e2213_q_d_n11: f64 = ((s.dn[379][11] * eq80_e2212_q) + (s.v[379] * s.dn[520][11]));
        let eq80_e2213_q_d_n12: f64 = ((s.dn[379][12] * eq80_e2212_q) + (s.v[379] * s.dn[520][12]));
        let eq80_e2213_q_d_n13: f64 = ((s.dn[379][13] * eq80_e2212_q) + (s.v[379] * s.dn[520][13]));
        let eq80_e2213_q_d_b0: f64 = ((s.db[379][0] * eq80_e2212_q) + (s.v[379] * s.db[520][0]));
        let eq80_e2213_q_d_b1: f64 = ((s.db[379][1] * eq80_e2212_q) + (s.v[379] * s.db[520][1]));
        let eq80_e2213_q_d_b2: f64 = ((s.db[379][2] * eq80_e2212_q) + (s.v[379] * s.db[520][2]));
        let eq80_e2213_q_d_b3: f64 = ((s.db[379][3] * eq80_e2212_q) + (s.v[379] * s.db[520][3]));
        let eq80_e2213_q_d_b4: f64 = ((s.db[379][4] * eq80_e2212_q) + (s.v[379] * s.db[520][4]));
        let eq80_e2213_q_d_b5: f64 = ((s.db[379][5] * eq80_e2212_q) + (s.v[379] * s.db[520][5]));
        let eq80_e2213_q_d_b6: f64 = ((s.db[379][6] * eq80_e2212_q) + (s.v[379] * s.db[520][6]));
        let eq80_e2213_q_d_b7: f64 = ((s.db[379][7] * eq80_e2212_q) + (s.v[379] * s.db[520][7]));
        let eq80_e2213_q_d_b8: f64 = ((s.db[379][8] * eq80_e2212_q) + (s.v[379] * s.db[520][8]));
        let eq80_e2213_q_d_b9: f64 = ((s.db[379][9] * eq80_e2212_q) + (s.v[379] * s.db[520][9]));
        let eq80_e2213_q_d_b10: f64 = ((s.db[379][10] * eq80_e2212_q) + (s.v[379] * s.db[520][10]));
        let eq80_e2213_q_d_b11: f64 = ((s.db[379][11] * eq80_e2212_q) + (s.v[379] * s.db[520][11]));
        let eq80_reactive_node_derivatives: [f64; 14] = [eq80_e2213_q_d_n0, eq80_e2213_q_d_n1, eq80_e2213_q_d_n2, eq80_e2213_q_d_n3, eq80_e2213_q_d_n4, eq80_e2213_q_d_n5, eq80_e2213_q_d_n6, eq80_e2213_q_d_n7, eq80_e2213_q_d_n8, eq80_e2213_q_d_n9, eq80_e2213_q_d_n10, eq80_e2213_q_d_n11, eq80_e2213_q_d_n12, eq80_e2213_q_d_n13];
        let eq80_reactive_branch_derivatives: [f64; 12] = [eq80_e2213_q_d_b0, eq80_e2213_q_d_b1, eq80_e2213_q_d_b2, eq80_e2213_q_d_b3, eq80_e2213_q_d_b4, eq80_e2213_q_d_b5, eq80_e2213_q_d_b6, eq80_e2213_q_d_b7, eq80_e2213_q_d_b8, eq80_e2213_q_d_b9, eq80_e2213_q_d_b10, eq80_e2213_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq80_reactive_node_derivatives,
            branches,
            &eq80_reactive_branch_derivatives,
            multiplicity,
        );
        let eq81_e2216_q: f64 = s.v[525];
        let eq81_e2217: f64 = (s.v[379] * s.v[525]);
        let eq81_e2217_d_n0: f64 = ((s.dn[379][0] * s.v[525]) + (s.v[379] * s.dn[525][0]));
        let eq81_e2217_d_n1: f64 = ((s.dn[379][1] * s.v[525]) + (s.v[379] * s.dn[525][1]));
        let eq81_e2217_d_n2: f64 = ((s.dn[379][2] * s.v[525]) + (s.v[379] * s.dn[525][2]));
        let eq81_e2217_d_n3: f64 = ((s.dn[379][3] * s.v[525]) + (s.v[379] * s.dn[525][3]));
        let eq81_e2217_d_n4: f64 = ((s.dn[379][4] * s.v[525]) + (s.v[379] * s.dn[525][4]));
        let eq81_e2217_d_n5: f64 = ((s.dn[379][5] * s.v[525]) + (s.v[379] * s.dn[525][5]));
        let eq81_e2217_d_n6: f64 = ((s.dn[379][6] * s.v[525]) + (s.v[379] * s.dn[525][6]));
        let eq81_e2217_d_n7: f64 = ((s.dn[379][7] * s.v[525]) + (s.v[379] * s.dn[525][7]));
        let eq81_e2217_d_n8: f64 = ((s.dn[379][8] * s.v[525]) + (s.v[379] * s.dn[525][8]));
        let eq81_e2217_d_n9: f64 = ((s.dn[379][9] * s.v[525]) + (s.v[379] * s.dn[525][9]));
        let eq81_e2217_d_n10: f64 = ((s.dn[379][10] * s.v[525]) + (s.v[379] * s.dn[525][10]));
        let eq81_e2217_d_n11: f64 = ((s.dn[379][11] * s.v[525]) + (s.v[379] * s.dn[525][11]));
        let eq81_e2217_d_n12: f64 = ((s.dn[379][12] * s.v[525]) + (s.v[379] * s.dn[525][12]));
        let eq81_e2217_d_n13: f64 = ((s.dn[379][13] * s.v[525]) + (s.v[379] * s.dn[525][13]));
        let eq81_e2217_d_b0: f64 = ((s.db[379][0] * s.v[525]) + (s.v[379] * s.db[525][0]));
        let eq81_e2217_d_b1: f64 = ((s.db[379][1] * s.v[525]) + (s.v[379] * s.db[525][1]));
        let eq81_e2217_d_b2: f64 = ((s.db[379][2] * s.v[525]) + (s.v[379] * s.db[525][2]));
        let eq81_e2217_d_b3: f64 = ((s.db[379][3] * s.v[525]) + (s.v[379] * s.db[525][3]));
        let eq81_e2217_d_b4: f64 = ((s.db[379][4] * s.v[525]) + (s.v[379] * s.db[525][4]));
        let eq81_e2217_d_b5: f64 = ((s.db[379][5] * s.v[525]) + (s.v[379] * s.db[525][5]));
        let eq81_e2217_d_b6: f64 = ((s.db[379][6] * s.v[525]) + (s.v[379] * s.db[525][6]));
        let eq81_e2217_d_b7: f64 = ((s.db[379][7] * s.v[525]) + (s.v[379] * s.db[525][7]));
        let eq81_e2217_d_b8: f64 = ((s.db[379][8] * s.v[525]) + (s.v[379] * s.db[525][8]));
        let eq81_e2217_d_b9: f64 = ((s.db[379][9] * s.v[525]) + (s.v[379] * s.db[525][9]));
        let eq81_e2217_d_b10: f64 = ((s.db[379][10] * s.v[525]) + (s.v[379] * s.db[525][10]));
        let eq81_e2217_d_b11: f64 = ((s.db[379][11] * s.v[525]) + (s.v[379] * s.db[525][11]));
        let eq81_e2217_q: f64 = (s.v[379] * eq81_e2216_q);
        let eq81_e2217_q_d_n0: f64 = ((s.dn[379][0] * eq81_e2216_q) + (s.v[379] * s.dn[525][0]));
        let eq81_e2217_q_d_n1: f64 = ((s.dn[379][1] * eq81_e2216_q) + (s.v[379] * s.dn[525][1]));
        let eq81_e2217_q_d_n2: f64 = ((s.dn[379][2] * eq81_e2216_q) + (s.v[379] * s.dn[525][2]));
        let eq81_e2217_q_d_n3: f64 = ((s.dn[379][3] * eq81_e2216_q) + (s.v[379] * s.dn[525][3]));
        let eq81_e2217_q_d_n4: f64 = ((s.dn[379][4] * eq81_e2216_q) + (s.v[379] * s.dn[525][4]));
        let eq81_e2217_q_d_n5: f64 = ((s.dn[379][5] * eq81_e2216_q) + (s.v[379] * s.dn[525][5]));
        let eq81_e2217_q_d_n6: f64 = ((s.dn[379][6] * eq81_e2216_q) + (s.v[379] * s.dn[525][6]));
        let eq81_e2217_q_d_n7: f64 = ((s.dn[379][7] * eq81_e2216_q) + (s.v[379] * s.dn[525][7]));
        let eq81_e2217_q_d_n8: f64 = ((s.dn[379][8] * eq81_e2216_q) + (s.v[379] * s.dn[525][8]));
        let eq81_e2217_q_d_n9: f64 = ((s.dn[379][9] * eq81_e2216_q) + (s.v[379] * s.dn[525][9]));
        let eq81_e2217_q_d_n10: f64 = ((s.dn[379][10] * eq81_e2216_q) + (s.v[379] * s.dn[525][10]));
        let eq81_e2217_q_d_n11: f64 = ((s.dn[379][11] * eq81_e2216_q) + (s.v[379] * s.dn[525][11]));
        let eq81_e2217_q_d_n12: f64 = ((s.dn[379][12] * eq81_e2216_q) + (s.v[379] * s.dn[525][12]));
        let eq81_e2217_q_d_n13: f64 = ((s.dn[379][13] * eq81_e2216_q) + (s.v[379] * s.dn[525][13]));
        let eq81_e2217_q_d_b0: f64 = ((s.db[379][0] * eq81_e2216_q) + (s.v[379] * s.db[525][0]));
        let eq81_e2217_q_d_b1: f64 = ((s.db[379][1] * eq81_e2216_q) + (s.v[379] * s.db[525][1]));
        let eq81_e2217_q_d_b2: f64 = ((s.db[379][2] * eq81_e2216_q) + (s.v[379] * s.db[525][2]));
        let eq81_e2217_q_d_b3: f64 = ((s.db[379][3] * eq81_e2216_q) + (s.v[379] * s.db[525][3]));
        let eq81_e2217_q_d_b4: f64 = ((s.db[379][4] * eq81_e2216_q) + (s.v[379] * s.db[525][4]));
        let eq81_e2217_q_d_b5: f64 = ((s.db[379][5] * eq81_e2216_q) + (s.v[379] * s.db[525][5]));
        let eq81_e2217_q_d_b6: f64 = ((s.db[379][6] * eq81_e2216_q) + (s.v[379] * s.db[525][6]));
        let eq81_e2217_q_d_b7: f64 = ((s.db[379][7] * eq81_e2216_q) + (s.v[379] * s.db[525][7]));
        let eq81_e2217_q_d_b8: f64 = ((s.db[379][8] * eq81_e2216_q) + (s.v[379] * s.db[525][8]));
        let eq81_e2217_q_d_b9: f64 = ((s.db[379][9] * eq81_e2216_q) + (s.v[379] * s.db[525][9]));
        let eq81_e2217_q_d_b10: f64 = ((s.db[379][10] * eq81_e2216_q) + (s.v[379] * s.db[525][10]));
        let eq81_e2217_q_d_b11: f64 = ((s.db[379][11] * eq81_e2216_q) + (s.v[379] * s.db[525][11]));
        let eq81_reactive_node_derivatives: [f64; 14] = [eq81_e2217_q_d_n0, eq81_e2217_q_d_n1, eq81_e2217_q_d_n2, eq81_e2217_q_d_n3, eq81_e2217_q_d_n4, eq81_e2217_q_d_n5, eq81_e2217_q_d_n6, eq81_e2217_q_d_n7, eq81_e2217_q_d_n8, eq81_e2217_q_d_n9, eq81_e2217_q_d_n10, eq81_e2217_q_d_n11, eq81_e2217_q_d_n12, eq81_e2217_q_d_n13];
        let eq81_reactive_branch_derivatives: [f64; 12] = [eq81_e2217_q_d_b0, eq81_e2217_q_d_b1, eq81_e2217_q_d_b2, eq81_e2217_q_d_b3, eq81_e2217_q_d_b4, eq81_e2217_q_d_b5, eq81_e2217_q_d_b6, eq81_e2217_q_d_b7, eq81_e2217_q_d_b8, eq81_e2217_q_d_b9, eq81_e2217_q_d_b10, eq81_e2217_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq81_reactive_node_derivatives,
            branches,
            &eq81_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
