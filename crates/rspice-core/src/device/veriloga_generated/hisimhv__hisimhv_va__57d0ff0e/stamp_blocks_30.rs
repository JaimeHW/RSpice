#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);let eq31_e1198_q: f64 = s.v[743];let eq31_e1199: f64 = (p[87] * s.v[743]);let eq31_e1199_q: f64 = (p[87] * eq31_e1198_q);
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(2),
            &s.dn[743],
            &s.db[743],
            (multiplicity) * (p[87]),
        );let eq32_e1202_q: f64 = s.v[742];let eq32_e1203: f64 = (p[87] * s.v[742]);let eq32_e1203_q: f64 = (p[87] * eq32_e1202_q);
        stamper.stamp_current_reactive_dense_local(
            Some(0),
            Some(2),
            &s.dn[742],
            &s.db[742],
            (multiplicity) * (p[87]),
        );let eq33_e1206_q: f64 = s.v[744];let eq33_e1207: f64 = (p[87] * s.v[744]);let eq33_e1207_q: f64 = (p[87] * eq33_e1206_q);
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(2),
            &s.dn[744],
            &s.db[744],
            (multiplicity) * (p[87]),
        );let eq34_e1209: f64 = (-p[87]);let eq34_e1211_q: f64 = s.v[299];let eq34_e1212: f64 = (eq34_e1209 * s.v[299]);let eq34_e1212_q: f64 = (eq34_e1209 * eq34_e1211_q);
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(0),
            &s.dn[299],
            &s.db[299],
            (multiplicity) * (eq34_e1209),
        );let eq35_e1214: f64 = (-p[87]);let eq35_e1216_q: f64 = s.v[301];let eq35_e1217: f64 = (eq35_e1214 * s.v[301]);let eq35_e1217_q: f64 = (eq35_e1214 * eq35_e1216_q);
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(2),
            &s.dn[301],
            &s.db[301],
            (multiplicity) * (eq35_e1214),
        );let eq41_e1246: f64 = ((nv15 - 0.0) * s.v[954]);let eq41_e1246_d_n0: f64 = ((nv15 - 0.0) * s.dn[954][0]);let eq41_e1246_d_n1: f64 = ((nv15 - 0.0) * s.dn[954][1]);let eq41_e1246_d_n2: f64 = ((nv15 - 0.0) * s.dn[954][2]);let eq41_e1246_d_n3: f64 = ((nv15 - 0.0) * s.dn[954][3]);let eq41_e1246_d_n4: f64 = ((nv15 - 0.0) * s.dn[954][4]);let eq41_e1246_d_n5: f64 = ((nv15 - 0.0) * s.dn[954][5]);let eq41_e1246_d_n6: f64 = ((nv15 - 0.0) * s.dn[954][6]);let eq41_e1246_d_n7: f64 = ((nv15 - 0.0) * s.dn[954][7]);let eq41_e1246_d_n8: f64 = ((nv15 - 0.0) * s.dn[954][8]);let eq41_e1246_d_n9: f64 = ((nv15 - 0.0) * s.dn[954][9]);let eq41_e1246_d_n10: f64 = ((nv15 - 0.0) * s.dn[954][10]);let eq41_e1246_d_n11: f64 = ((nv15 - 0.0) * s.dn[954][11]);let eq41_e1246_d_n12: f64 = ((nv15 - 0.0) * s.dn[954][12]);let eq41_e1246_d_n13: f64 = ((nv15 - 0.0) * s.dn[954][13]);let eq41_e1246_d_n14: f64 = ((nv15 - 0.0) * s.dn[954][14]);let eq41_e1246_d_n15: f64 = (s.v[954] + ((nv15 - 0.0) * s.dn[954][15]));let eq41_e1246_d_n16: f64 = ((nv15 - 0.0) * s.dn[954][16]);let eq41_e1246_d_n17: f64 = ((nv15 - 0.0) * s.dn[954][17]);let eq41_e1246_d_n18: f64 = ((nv15 - 0.0) * s.dn[954][18]);let eq41_e1246_d_b0: f64 = ((nv15 - 0.0) * s.db[954][0]);let eq41_e1246_d_b1: f64 = ((nv15 - 0.0) * s.db[954][1]);let eq41_e1246_d_b2: f64 = ((nv15 - 0.0) * s.db[954][2]);let eq41_e1246_d_b3: f64 = ((nv15 - 0.0) * s.db[954][3]);let eq41_e1246_d_b4: f64 = ((nv15 - 0.0) * s.db[954][4]);let eq41_e1246_d_b5: f64 = ((nv15 - 0.0) * s.db[954][5]);let eq41_e1246_d_b6: f64 = ((nv15 - 0.0) * s.db[954][6]);let eq41_e1246_d_b7: f64 = ((nv15 - 0.0) * s.db[954][7]);let eq41_e1246_d_b8: f64 = ((nv15 - 0.0) * s.db[954][8]);let eq41_e1246_d_b9: f64 = ((nv15 - 0.0) * s.db[954][9]);let eq41_e1246_d_b10: f64 = ((nv15 - 0.0) * s.db[954][10]);let eq41_e1246_d_b11: f64 = ((nv15 - 0.0) * s.db[954][11]);let eq41_e1246_d_b12: f64 = ((nv15 - 0.0) * s.db[954][12]);let eq41_e1247_q: f64 = eq41_e1246;let eq41_reactive_node_derivatives: [f64; 19] = [eq41_e1246_d_n0, eq41_e1246_d_n1, eq41_e1246_d_n2, eq41_e1246_d_n3, eq41_e1246_d_n4, eq41_e1246_d_n5, eq41_e1246_d_n6, eq41_e1246_d_n7, eq41_e1246_d_n8, eq41_e1246_d_n9, eq41_e1246_d_n10, eq41_e1246_d_n11, eq41_e1246_d_n12, eq41_e1246_d_n13, eq41_e1246_d_n14, eq41_e1246_d_n15, eq41_e1246_d_n16, eq41_e1246_d_n17, eq41_e1246_d_n18];let eq41_reactive_branch_derivatives: [f64; 13] = [eq41_e1246_d_b0, eq41_e1246_d_b1, eq41_e1246_d_b2, eq41_e1246_d_b3, eq41_e1246_d_b4, eq41_e1246_d_b5, eq41_e1246_d_b6, eq41_e1246_d_b7, eq41_e1246_d_b8, eq41_e1246_d_b9, eq41_e1246_d_b10, eq41_e1246_d_b11, eq41_e1246_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(8),
            &eq41_reactive_node_derivatives,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let nv15 = ctx.node_voltage(nodes[15]);let eq42_e1250: f64 = ((nv15 - 0.0) * s.v[955]);let eq42_e1250_d_n0: f64 = ((nv15 - 0.0) * s.dn[955][0]);let eq42_e1250_d_n1: f64 = ((nv15 - 0.0) * s.dn[955][1]);let eq42_e1250_d_n2: f64 = ((nv15 - 0.0) * s.dn[955][2]);let eq42_e1250_d_n3: f64 = ((nv15 - 0.0) * s.dn[955][3]);let eq42_e1250_d_n4: f64 = ((nv15 - 0.0) * s.dn[955][4]);let eq42_e1250_d_n5: f64 = ((nv15 - 0.0) * s.dn[955][5]);let eq42_e1250_d_n6: f64 = ((nv15 - 0.0) * s.dn[955][6]);let eq42_e1250_d_n7: f64 = ((nv15 - 0.0) * s.dn[955][7]);let eq42_e1250_d_n8: f64 = ((nv15 - 0.0) * s.dn[955][8]);let eq42_e1250_d_n9: f64 = ((nv15 - 0.0) * s.dn[955][9]);let eq42_e1250_d_n10: f64 = ((nv15 - 0.0) * s.dn[955][10]);let eq42_e1250_d_n11: f64 = ((nv15 - 0.0) * s.dn[955][11]);let eq42_e1250_d_n12: f64 = ((nv15 - 0.0) * s.dn[955][12]);let eq42_e1250_d_n13: f64 = ((nv15 - 0.0) * s.dn[955][13]);let eq42_e1250_d_n14: f64 = ((nv15 - 0.0) * s.dn[955][14]);let eq42_e1250_d_n15: f64 = (s.v[955] + ((nv15 - 0.0) * s.dn[955][15]));let eq42_e1250_d_n16: f64 = ((nv15 - 0.0) * s.dn[955][16]);let eq42_e1250_d_n17: f64 = ((nv15 - 0.0) * s.dn[955][17]);let eq42_e1250_d_n18: f64 = ((nv15 - 0.0) * s.dn[955][18]);let eq42_e1250_d_b0: f64 = ((nv15 - 0.0) * s.db[955][0]);let eq42_e1250_d_b1: f64 = ((nv15 - 0.0) * s.db[955][1]);let eq42_e1250_d_b2: f64 = ((nv15 - 0.0) * s.db[955][2]);let eq42_e1250_d_b3: f64 = ((nv15 - 0.0) * s.db[955][3]);let eq42_e1250_d_b4: f64 = ((nv15 - 0.0) * s.db[955][4]);let eq42_e1250_d_b5: f64 = ((nv15 - 0.0) * s.db[955][5]);let eq42_e1250_d_b6: f64 = ((nv15 - 0.0) * s.db[955][6]);let eq42_e1250_d_b7: f64 = ((nv15 - 0.0) * s.db[955][7]);let eq42_e1250_d_b8: f64 = ((nv15 - 0.0) * s.db[955][8]);let eq42_e1250_d_b9: f64 = ((nv15 - 0.0) * s.db[955][9]);let eq42_e1250_d_b10: f64 = ((nv15 - 0.0) * s.db[955][10]);let eq42_e1250_d_b11: f64 = ((nv15 - 0.0) * s.db[955][11]);let eq42_e1250_d_b12: f64 = ((nv15 - 0.0) * s.db[955][12]);let eq42_e1251_q: f64 = eq42_e1250;let eq42_reactive_node_derivatives: [f64; 19] = [eq42_e1250_d_n0, eq42_e1250_d_n1, eq42_e1250_d_n2, eq42_e1250_d_n3, eq42_e1250_d_n4, eq42_e1250_d_n5, eq42_e1250_d_n6, eq42_e1250_d_n7, eq42_e1250_d_n8, eq42_e1250_d_n9, eq42_e1250_d_n10, eq42_e1250_d_n11, eq42_e1250_d_n12, eq42_e1250_d_n13, eq42_e1250_d_n14, eq42_e1250_d_n15, eq42_e1250_d_n16, eq42_e1250_d_n17, eq42_e1250_d_n18];let eq42_reactive_branch_derivatives: [f64; 13] = [eq42_e1250_d_b0, eq42_e1250_d_b1, eq42_e1250_d_b2, eq42_e1250_d_b3, eq42_e1250_d_b4, eq42_e1250_d_b5, eq42_e1250_d_b6, eq42_e1250_d_b7, eq42_e1250_d_b8, eq42_e1250_d_b9, eq42_e1250_d_b10, eq42_e1250_d_b11, eq42_e1250_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(6),
            &eq42_reactive_node_derivatives,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );let eq59_e1356: f64 = (s.v[767] * (nv5 - 0.0));let eq59_e1356_d_n0: f64 = (s.dn[767][0] * (nv5 - 0.0));let eq59_e1356_d_n1: f64 = (s.dn[767][1] * (nv5 - 0.0));let eq59_e1356_d_n2: f64 = (s.dn[767][2] * (nv5 - 0.0));let eq59_e1356_d_n3: f64 = (s.dn[767][3] * (nv5 - 0.0));let eq59_e1356_d_n4: f64 = (s.dn[767][4] * (nv5 - 0.0));let eq59_e1356_d_n5: f64 = ((s.dn[767][5] * (nv5 - 0.0)) + s.v[767]);let eq59_e1356_d_n6: f64 = (s.dn[767][6] * (nv5 - 0.0));let eq59_e1356_d_n7: f64 = (s.dn[767][7] * (nv5 - 0.0));let eq59_e1356_d_n8: f64 = (s.dn[767][8] * (nv5 - 0.0));let eq59_e1356_d_n9: f64 = (s.dn[767][9] * (nv5 - 0.0));let eq59_e1356_d_n10: f64 = (s.dn[767][10] * (nv5 - 0.0));let eq59_e1356_d_n11: f64 = (s.dn[767][11] * (nv5 - 0.0));let eq59_e1356_d_n12: f64 = (s.dn[767][12] * (nv5 - 0.0));let eq59_e1356_d_n13: f64 = (s.dn[767][13] * (nv5 - 0.0));let eq59_e1356_d_n14: f64 = (s.dn[767][14] * (nv5 - 0.0));let eq59_e1356_d_n15: f64 = (s.dn[767][15] * (nv5 - 0.0));let eq59_e1356_d_n16: f64 = (s.dn[767][16] * (nv5 - 0.0));let eq59_e1356_d_n17: f64 = (s.dn[767][17] * (nv5 - 0.0));let eq59_e1356_d_n18: f64 = (s.dn[767][18] * (nv5 - 0.0));let eq59_e1356_d_b0: f64 = (s.db[767][0] * (nv5 - 0.0));let eq59_e1356_d_b1: f64 = (s.db[767][1] * (nv5 - 0.0));let eq59_e1356_d_b2: f64 = (s.db[767][2] * (nv5 - 0.0));let eq59_e1356_d_b3: f64 = (s.db[767][3] * (nv5 - 0.0));let eq59_e1356_d_b4: f64 = (s.db[767][4] * (nv5 - 0.0));let eq59_e1356_d_b5: f64 = (s.db[767][5] * (nv5 - 0.0));let eq59_e1356_d_b6: f64 = (s.db[767][6] * (nv5 - 0.0));let eq59_e1356_d_b7: f64 = (s.db[767][7] * (nv5 - 0.0));let eq59_e1356_d_b8: f64 = (s.db[767][8] * (nv5 - 0.0));let eq59_e1356_d_b9: f64 = (s.db[767][9] * (nv5 - 0.0));let eq59_e1356_d_b10: f64 = (s.db[767][10] * (nv5 - 0.0));let eq59_e1356_d_b11: f64 = (s.db[767][11] * (nv5 - 0.0));let eq59_e1356_d_b12: f64 = (s.db[767][12] * (nv5 - 0.0));let eq59_e1357_q: f64 = eq59_e1356;let eq59_reactive_node_derivatives: [f64; 19] = [eq59_e1356_d_n0, eq59_e1356_d_n1, eq59_e1356_d_n2, eq59_e1356_d_n3, eq59_e1356_d_n4, eq59_e1356_d_n5, eq59_e1356_d_n6, eq59_e1356_d_n7, eq59_e1356_d_n8, eq59_e1356_d_n9, eq59_e1356_d_n10, eq59_e1356_d_n11, eq59_e1356_d_n12, eq59_e1356_d_n13, eq59_e1356_d_n14, eq59_e1356_d_n15, eq59_e1356_d_n16, eq59_e1356_d_n17, eq59_e1356_d_n18];let eq59_reactive_branch_derivatives: [f64; 13] = [eq59_e1356_d_b0, eq59_e1356_d_b1, eq59_e1356_d_b2, eq59_e1356_d_b3, eq59_e1356_d_b4, eq59_e1356_d_b5, eq59_e1356_d_b6, eq59_e1356_d_b7, eq59_e1356_d_b8, eq59_e1356_d_b9, eq59_e1356_d_b10, eq59_e1356_d_b11, eq59_e1356_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            None,
            &eq59_reactive_node_derivatives,
            &eq59_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let nv14 = ctx.node_voltage(nodes[14]);
        let (eq62_e1372, eq62_e1372_d_n0, eq62_e1372_d_n1, eq62_e1372_d_n2, eq62_e1372_d_n3, eq62_e1372_d_n4, eq62_e1372_d_n5, eq62_e1372_d_n6, eq62_e1372_d_n7, eq62_e1372_d_n8, eq62_e1372_d_n9, eq62_e1372_d_n10, eq62_e1372_d_n11, eq62_e1372_d_n12, eq62_e1372_d_n13, eq62_e1372_d_n14, eq62_e1372_d_n15, eq62_e1372_d_n16, eq62_e1372_d_n17, eq62_e1372_d_n18, eq62_e1372_d_b0, eq62_e1372_d_b1, eq62_e1372_d_b2, eq62_e1372_d_b3, eq62_e1372_d_b4, eq62_e1372_d_b5, eq62_e1372_d_b6, eq62_e1372_d_b7, eq62_e1372_d_b8, eq62_e1372_d_b9, eq62_e1372_d_b10, eq62_e1372_d_b11, eq62_e1372_d_b12, eq62_e1372_q,) = {
    if (p[28] != 0.0) {
        let eq62_e1369: f64 = (s.v[800] * (nv12 - 0.0));let eq62_e1369_d_n0: f64 = (s.dn[800][0] * (nv12 - 0.0));let eq62_e1369_d_n1: f64 = (s.dn[800][1] * (nv12 - 0.0));let eq62_e1369_d_n2: f64 = (s.dn[800][2] * (nv12 - 0.0));let eq62_e1369_d_n3: f64 = (s.dn[800][3] * (nv12 - 0.0));let eq62_e1369_d_n4: f64 = (s.dn[800][4] * (nv12 - 0.0));let eq62_e1369_d_n5: f64 = (s.dn[800][5] * (nv12 - 0.0));let eq62_e1369_d_n6: f64 = (s.dn[800][6] * (nv12 - 0.0));let eq62_e1369_d_n7: f64 = (s.dn[800][7] * (nv12 - 0.0));let eq62_e1369_d_n8: f64 = (s.dn[800][8] * (nv12 - 0.0));let eq62_e1369_d_n9: f64 = (s.dn[800][9] * (nv12 - 0.0));let eq62_e1369_d_n10: f64 = (s.dn[800][10] * (nv12 - 0.0));let eq62_e1369_d_n11: f64 = (s.dn[800][11] * (nv12 - 0.0));let eq62_e1369_d_n12: f64 = ((s.dn[800][12] * (nv12 - 0.0)) + s.v[800]);let eq62_e1369_d_n13: f64 = (s.dn[800][13] * (nv12 - 0.0));let eq62_e1369_d_n14: f64 = (s.dn[800][14] * (nv12 - 0.0));let eq62_e1369_d_n15: f64 = (s.dn[800][15] * (nv12 - 0.0));let eq62_e1369_d_n16: f64 = (s.dn[800][16] * (nv12 - 0.0));let eq62_e1369_d_n17: f64 = (s.dn[800][17] * (nv12 - 0.0));let eq62_e1369_d_n18: f64 = (s.dn[800][18] * (nv12 - 0.0));let eq62_e1369_d_b0: f64 = (s.db[800][0] * (nv12 - 0.0));let eq62_e1369_d_b1: f64 = (s.db[800][1] * (nv12 - 0.0));let eq62_e1369_d_b2: f64 = (s.db[800][2] * (nv12 - 0.0));let eq62_e1369_d_b3: f64 = (s.db[800][3] * (nv12 - 0.0));let eq62_e1369_d_b4: f64 = (s.db[800][4] * (nv12 - 0.0));let eq62_e1369_d_b5: f64 = (s.db[800][5] * (nv12 - 0.0));let eq62_e1369_d_b6: f64 = (s.db[800][6] * (nv12 - 0.0));let eq62_e1369_d_b7: f64 = (s.db[800][7] * (nv12 - 0.0));let eq62_e1369_d_b8: f64 = (s.db[800][8] * (nv12 - 0.0));let eq62_e1369_d_b9: f64 = (s.db[800][9] * (nv12 - 0.0));let eq62_e1369_d_b10: f64 = (s.db[800][10] * (nv12 - 0.0));let eq62_e1369_d_b11: f64 = (s.db[800][11] * (nv12 - 0.0));let eq62_e1369_d_b12: f64 = (s.db[800][12] * (nv12 - 0.0));let eq62_e1370_q: f64 = eq62_e1369;
        (eq62_e1369, eq62_e1369_d_n0, eq62_e1369_d_n1, eq62_e1369_d_n2, eq62_e1369_d_n3, eq62_e1369_d_n4, eq62_e1369_d_n5, eq62_e1369_d_n6, eq62_e1369_d_n7, eq62_e1369_d_n8, eq62_e1369_d_n9, eq62_e1369_d_n10, eq62_e1369_d_n11, eq62_e1369_d_n12, eq62_e1369_d_n13, eq62_e1369_d_n14, eq62_e1369_d_n15, eq62_e1369_d_n16, eq62_e1369_d_n17, eq62_e1369_d_n18, eq62_e1369_d_b0, eq62_e1369_d_b1, eq62_e1369_d_b2, eq62_e1369_d_b3, eq62_e1369_d_b4, eq62_e1369_d_b5, eq62_e1369_d_b6, eq62_e1369_d_b7, eq62_e1369_d_b8, eq62_e1369_d_b9, eq62_e1369_d_b10, eq62_e1369_d_b11, eq62_e1369_d_b12, eq62_e1370_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_reactive_node_derivatives: [f64; 19] = [eq62_e1372_d_n0, eq62_e1372_d_n1, eq62_e1372_d_n2, eq62_e1372_d_n3, eq62_e1372_d_n4, eq62_e1372_d_n5, eq62_e1372_d_n6, eq62_e1372_d_n7, eq62_e1372_d_n8, eq62_e1372_d_n9, eq62_e1372_d_n10, eq62_e1372_d_n11, eq62_e1372_d_n12, eq62_e1372_d_n13, eq62_e1372_d_n14, eq62_e1372_d_n15, eq62_e1372_d_n16, eq62_e1372_d_n17, eq62_e1372_d_n18];let eq62_reactive_branch_derivatives: [f64; 13] = [eq62_e1372_d_b0, eq62_e1372_d_b1, eq62_e1372_d_b2, eq62_e1372_d_b3, eq62_e1372_d_b4, eq62_e1372_d_b5, eq62_e1372_d_b6, eq62_e1372_d_b7, eq62_e1372_d_b8, eq62_e1372_d_b9, eq62_e1372_d_b10, eq62_e1372_d_b11, eq62_e1372_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(12),
            None,
            &eq62_reactive_node_derivatives,
            &eq62_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq63_e1379, eq63_e1379_d_n0, eq63_e1379_d_n1, eq63_e1379_d_n2, eq63_e1379_d_n3, eq63_e1379_d_n4, eq63_e1379_d_n5, eq63_e1379_d_n6, eq63_e1379_d_n7, eq63_e1379_d_n8, eq63_e1379_d_n9, eq63_e1379_d_n10, eq63_e1379_d_n11, eq63_e1379_d_n12, eq63_e1379_d_n13, eq63_e1379_d_n14, eq63_e1379_d_n15, eq63_e1379_d_n16, eq63_e1379_d_n17, eq63_e1379_d_n18, eq63_e1379_d_b0, eq63_e1379_d_b1, eq63_e1379_d_b2, eq63_e1379_d_b3, eq63_e1379_d_b4, eq63_e1379_d_b5, eq63_e1379_d_b6, eq63_e1379_d_b7, eq63_e1379_d_b8, eq63_e1379_d_b9, eq63_e1379_d_b10, eq63_e1379_d_b11, eq63_e1379_d_b12, eq63_e1379_q,) = {
    if (p[28] != 0.0) {
        let eq63_e1376: f64 = (s.v[801] * (nv13 - 0.0));let eq63_e1376_d_n0: f64 = (s.dn[801][0] * (nv13 - 0.0));let eq63_e1376_d_n1: f64 = (s.dn[801][1] * (nv13 - 0.0));let eq63_e1376_d_n2: f64 = (s.dn[801][2] * (nv13 - 0.0));let eq63_e1376_d_n3: f64 = (s.dn[801][3] * (nv13 - 0.0));let eq63_e1376_d_n4: f64 = (s.dn[801][4] * (nv13 - 0.0));let eq63_e1376_d_n5: f64 = (s.dn[801][5] * (nv13 - 0.0));let eq63_e1376_d_n6: f64 = (s.dn[801][6] * (nv13 - 0.0));let eq63_e1376_d_n7: f64 = (s.dn[801][7] * (nv13 - 0.0));let eq63_e1376_d_n8: f64 = (s.dn[801][8] * (nv13 - 0.0));let eq63_e1376_d_n9: f64 = (s.dn[801][9] * (nv13 - 0.0));let eq63_e1376_d_n10: f64 = (s.dn[801][10] * (nv13 - 0.0));let eq63_e1376_d_n11: f64 = (s.dn[801][11] * (nv13 - 0.0));let eq63_e1376_d_n12: f64 = (s.dn[801][12] * (nv13 - 0.0));let eq63_e1376_d_n13: f64 = ((s.dn[801][13] * (nv13 - 0.0)) + s.v[801]);let eq63_e1376_d_n14: f64 = (s.dn[801][14] * (nv13 - 0.0));let eq63_e1376_d_n15: f64 = (s.dn[801][15] * (nv13 - 0.0));let eq63_e1376_d_n16: f64 = (s.dn[801][16] * (nv13 - 0.0));let eq63_e1376_d_n17: f64 = (s.dn[801][17] * (nv13 - 0.0));let eq63_e1376_d_n18: f64 = (s.dn[801][18] * (nv13 - 0.0));let eq63_e1376_d_b0: f64 = (s.db[801][0] * (nv13 - 0.0));let eq63_e1376_d_b1: f64 = (s.db[801][1] * (nv13 - 0.0));let eq63_e1376_d_b2: f64 = (s.db[801][2] * (nv13 - 0.0));let eq63_e1376_d_b3: f64 = (s.db[801][3] * (nv13 - 0.0));let eq63_e1376_d_b4: f64 = (s.db[801][4] * (nv13 - 0.0));let eq63_e1376_d_b5: f64 = (s.db[801][5] * (nv13 - 0.0));let eq63_e1376_d_b6: f64 = (s.db[801][6] * (nv13 - 0.0));let eq63_e1376_d_b7: f64 = (s.db[801][7] * (nv13 - 0.0));let eq63_e1376_d_b8: f64 = (s.db[801][8] * (nv13 - 0.0));let eq63_e1376_d_b9: f64 = (s.db[801][9] * (nv13 - 0.0));let eq63_e1376_d_b10: f64 = (s.db[801][10] * (nv13 - 0.0));let eq63_e1376_d_b11: f64 = (s.db[801][11] * (nv13 - 0.0));let eq63_e1376_d_b12: f64 = (s.db[801][12] * (nv13 - 0.0));let eq63_e1377_q: f64 = eq63_e1376;
        (eq63_e1376, eq63_e1376_d_n0, eq63_e1376_d_n1, eq63_e1376_d_n2, eq63_e1376_d_n3, eq63_e1376_d_n4, eq63_e1376_d_n5, eq63_e1376_d_n6, eq63_e1376_d_n7, eq63_e1376_d_n8, eq63_e1376_d_n9, eq63_e1376_d_n10, eq63_e1376_d_n11, eq63_e1376_d_n12, eq63_e1376_d_n13, eq63_e1376_d_n14, eq63_e1376_d_n15, eq63_e1376_d_n16, eq63_e1376_d_n17, eq63_e1376_d_n18, eq63_e1376_d_b0, eq63_e1376_d_b1, eq63_e1376_d_b2, eq63_e1376_d_b3, eq63_e1376_d_b4, eq63_e1376_d_b5, eq63_e1376_d_b6, eq63_e1376_d_b7, eq63_e1376_d_b8, eq63_e1376_d_b9, eq63_e1376_d_b10, eq63_e1376_d_b11, eq63_e1376_d_b12, eq63_e1377_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_reactive_node_derivatives: [f64; 19] = [eq63_e1379_d_n0, eq63_e1379_d_n1, eq63_e1379_d_n2, eq63_e1379_d_n3, eq63_e1379_d_n4, eq63_e1379_d_n5, eq63_e1379_d_n6, eq63_e1379_d_n7, eq63_e1379_d_n8, eq63_e1379_d_n9, eq63_e1379_d_n10, eq63_e1379_d_n11, eq63_e1379_d_n12, eq63_e1379_d_n13, eq63_e1379_d_n14, eq63_e1379_d_n15, eq63_e1379_d_n16, eq63_e1379_d_n17, eq63_e1379_d_n18];let eq63_reactive_branch_derivatives: [f64; 13] = [eq63_e1379_d_b0, eq63_e1379_d_b1, eq63_e1379_d_b2, eq63_e1379_d_b3, eq63_e1379_d_b4, eq63_e1379_d_b5, eq63_e1379_d_b6, eq63_e1379_d_b7, eq63_e1379_d_b8, eq63_e1379_d_b9, eq63_e1379_d_b10, eq63_e1379_d_b11, eq63_e1379_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(13),
            None,
            &eq63_reactive_node_derivatives,
            &eq63_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq67_e1398, eq67_e1398_d_n14, eq67_e1398_q,) = {
    if (p[29] != 0.0) {
        let eq67_e1396_q: f64 = (nv14 - 0.0);
        ((nv14 - 0.0), 1.0, eq67_e1396_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(14),
            None,
            14,
            multiplicity * (eq67_e1398_d_n14),
        );
    }
}
