#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq5_e1056, eq5_e1056_d_n0, eq5_e1056_d_n1, eq5_e1056_d_n2, eq5_e1056_d_n3, eq5_e1056_d_n4, eq5_e1056_d_n5, eq5_e1056_d_n6, eq5_e1056_d_n7, eq5_e1056_d_n8, eq5_e1056_d_n9, eq5_e1056_d_n10, eq5_e1056_d_n11, eq5_e1056_d_n12, eq5_e1056_d_n13, eq5_e1056_d_n14, eq5_e1056_d_n15, eq5_e1056_d_n16, eq5_e1056_d_n17, eq5_e1056_d_n18, eq5_e1056_d_b0, eq5_e1056_d_b1, eq5_e1056_d_b2, eq5_e1056_d_b3, eq5_e1056_d_b4, eq5_e1056_d_b5, eq5_e1056_d_b6, eq5_e1056_d_b7, eq5_e1056_d_b8, eq5_e1056_d_b9, eq5_e1056_d_b10, eq5_e1056_d_b11, eq5_e1056_d_b12, eq5_e1056_q, eq5_e1056_q_d_n0, eq5_e1056_q_d_n1, eq5_e1056_q_d_n2, eq5_e1056_q_d_n3, eq5_e1056_q_d_n4, eq5_e1056_q_d_n5, eq5_e1056_q_d_n6, eq5_e1056_q_d_n7, eq5_e1056_q_d_n8, eq5_e1056_q_d_n9, eq5_e1056_q_d_n10, eq5_e1056_q_d_n11, eq5_e1056_q_d_n12, eq5_e1056_q_d_n13, eq5_e1056_q_d_n14, eq5_e1056_q_d_n15, eq5_e1056_q_d_n16, eq5_e1056_q_d_n17, eq5_e1056_q_d_n18, eq5_e1056_q_d_b0, eq5_e1056_q_d_b1, eq5_e1056_q_d_b2, eq5_e1056_q_d_b3, eq5_e1056_q_d_b4, eq5_e1056_q_d_b5, eq5_e1056_q_d_b6, eq5_e1056_q_d_b7, eq5_e1056_q_d_b8, eq5_e1056_q_d_b9, eq5_e1056_q_d_b10, eq5_e1056_q_d_b11, eq5_e1056_q_d_b12,) = {
    if s.b[3308] {
        let eq5_e1053_q: f64 = s.v[931];let eq5_e1054: f64 = (s.v[932] + s.v[931]);let eq5_e1054_d_n0: f64 = (s.dn[932][0] + s.dn[931][0]);let eq5_e1054_d_n1: f64 = (s.dn[932][1] + s.dn[931][1]);let eq5_e1054_d_n2: f64 = (s.dn[932][2] + s.dn[931][2]);let eq5_e1054_d_n3: f64 = (s.dn[932][3] + s.dn[931][3]);let eq5_e1054_d_n4: f64 = (s.dn[932][4] + s.dn[931][4]);let eq5_e1054_d_n5: f64 = (s.dn[932][5] + s.dn[931][5]);let eq5_e1054_d_n6: f64 = (s.dn[932][6] + s.dn[931][6]);let eq5_e1054_d_n7: f64 = (s.dn[932][7] + s.dn[931][7]);let eq5_e1054_d_n8: f64 = (s.dn[932][8] + s.dn[931][8]);let eq5_e1054_d_n9: f64 = (s.dn[932][9] + s.dn[931][9]);let eq5_e1054_d_n10: f64 = (s.dn[932][10] + s.dn[931][10]);let eq5_e1054_d_n11: f64 = (s.dn[932][11] + s.dn[931][11]);let eq5_e1054_d_n12: f64 = (s.dn[932][12] + s.dn[931][12]);let eq5_e1054_d_n13: f64 = (s.dn[932][13] + s.dn[931][13]);let eq5_e1054_d_n14: f64 = (s.dn[932][14] + s.dn[931][14]);let eq5_e1054_d_n15: f64 = (s.dn[932][15] + s.dn[931][15]);let eq5_e1054_d_n16: f64 = (s.dn[932][16] + s.dn[931][16]);let eq5_e1054_d_n17: f64 = (s.dn[932][17] + s.dn[931][17]);let eq5_e1054_d_n18: f64 = (s.dn[932][18] + s.dn[931][18]);let eq5_e1054_d_b0: f64 = (s.db[932][0] + s.db[931][0]);let eq5_e1054_d_b1: f64 = (s.db[932][1] + s.db[931][1]);let eq5_e1054_d_b2: f64 = (s.db[932][2] + s.db[931][2]);let eq5_e1054_d_b3: f64 = (s.db[932][3] + s.db[931][3]);let eq5_e1054_d_b4: f64 = (s.db[932][4] + s.db[931][4]);let eq5_e1054_d_b5: f64 = (s.db[932][5] + s.db[931][5]);let eq5_e1054_d_b6: f64 = (s.db[932][6] + s.db[931][6]);let eq5_e1054_d_b7: f64 = (s.db[932][7] + s.db[931][7]);let eq5_e1054_d_b8: f64 = (s.db[932][8] + s.db[931][8]);let eq5_e1054_d_b9: f64 = (s.db[932][9] + s.db[931][9]);let eq5_e1054_d_b10: f64 = (s.db[932][10] + s.db[931][10]);let eq5_e1054_d_b11: f64 = (s.db[932][11] + s.db[931][11]);let eq5_e1054_d_b12: f64 = (s.db[932][12] + s.db[931][12]);let eq5_e1054_q: f64 = eq5_e1053_q;
        (eq5_e1054, eq5_e1054_d_n0, eq5_e1054_d_n1, eq5_e1054_d_n2, eq5_e1054_d_n3, eq5_e1054_d_n4, eq5_e1054_d_n5, eq5_e1054_d_n6, eq5_e1054_d_n7, eq5_e1054_d_n8, eq5_e1054_d_n9, eq5_e1054_d_n10, eq5_e1054_d_n11, eq5_e1054_d_n12, eq5_e1054_d_n13, eq5_e1054_d_n14, eq5_e1054_d_n15, eq5_e1054_d_n16, eq5_e1054_d_n17, eq5_e1054_d_n18, eq5_e1054_d_b0, eq5_e1054_d_b1, eq5_e1054_d_b2, eq5_e1054_d_b3, eq5_e1054_d_b4, eq5_e1054_d_b5, eq5_e1054_d_b6, eq5_e1054_d_b7, eq5_e1054_d_b8, eq5_e1054_d_b9, eq5_e1054_d_b10, eq5_e1054_d_b11, eq5_e1054_d_b12, eq5_e1054_q, s.dn[931][0], s.dn[931][1], s.dn[931][2], s.dn[931][3], s.dn[931][4], s.dn[931][5], s.dn[931][6], s.dn[931][7], s.dn[931][8], s.dn[931][9], s.dn[931][10], s.dn[931][11], s.dn[931][12], s.dn[931][13], s.dn[931][14], s.dn[931][15], s.dn[931][16], s.dn[931][17], s.dn[931][18], s.db[931][0], s.db[931][1], s.db[931][2], s.db[931][3], s.db[931][4], s.db[931][5], s.db[931][6], s.db[931][7], s.db[931][8], s.db[931][9], s.db[931][10], s.db[931][11], s.db[931][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 19] = [eq5_e1056_q_d_n0, eq5_e1056_q_d_n1, eq5_e1056_q_d_n2, eq5_e1056_q_d_n3, eq5_e1056_q_d_n4, eq5_e1056_q_d_n5, eq5_e1056_q_d_n6, eq5_e1056_q_d_n7, eq5_e1056_q_d_n8, eq5_e1056_q_d_n9, eq5_e1056_q_d_n10, eq5_e1056_q_d_n11, eq5_e1056_q_d_n12, eq5_e1056_q_d_n13, eq5_e1056_q_d_n14, eq5_e1056_q_d_n15, eq5_e1056_q_d_n16, eq5_e1056_q_d_n17, eq5_e1056_q_d_n18];let eq5_reactive_branch_derivatives: [f64; 13] = [eq5_e1056_q_d_b0, eq5_e1056_q_d_b1, eq5_e1056_q_d_b2, eq5_e1056_q_d_b3, eq5_e1056_q_d_b4, eq5_e1056_q_d_b5, eq5_e1056_q_d_b6, eq5_e1056_q_d_b7, eq5_e1056_q_d_b8, eq5_e1056_q_d_b9, eq5_e1056_q_d_b10, eq5_e1056_q_d_b11, eq5_e1056_q_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(18),
            None,
            &eq5_reactive_node_derivatives,
            &eq5_reactive_branch_derivatives,
            multiplicity,
        );let eq15_e1102_q: f64 = s.v[66];let eq15_e1103: f64 = (p.p87 * s.v[66]);let eq15_e1103_q: f64 = (p.p87 * eq15_e1102_q);
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(2),
            &s.dn[66],
            &s.db[66],
            (multiplicity) * (p.p87),
        );let eq16_e1106_q: f64 = s.v[65];let eq16_e1107: f64 = (p.p87 * s.v[65]);let eq16_e1107_q: f64 = (p.p87 * eq16_e1106_q);
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            Some(0),
            &s.dn[65],
            &s.db[65],
            (multiplicity) * (p.p87),
        );
        let (eq19_e1126, eq19_e1126_d_n0, eq19_e1126_d_n1, eq19_e1126_d_n2, eq19_e1126_d_n3, eq19_e1126_d_n4, eq19_e1126_d_n5, eq19_e1126_d_n6, eq19_e1126_d_n7, eq19_e1126_d_n8, eq19_e1126_d_n9, eq19_e1126_d_n10, eq19_e1126_d_n11, eq19_e1126_d_n12, eq19_e1126_d_n13, eq19_e1126_d_n14, eq19_e1126_d_n15, eq19_e1126_d_n16, eq19_e1126_d_n17, eq19_e1126_d_n18, eq19_e1126_d_b0, eq19_e1126_d_b1, eq19_e1126_d_b2, eq19_e1126_d_b3, eq19_e1126_d_b4, eq19_e1126_d_b5, eq19_e1126_d_b6, eq19_e1126_d_b7, eq19_e1126_d_b8, eq19_e1126_d_b9, eq19_e1126_d_b10, eq19_e1126_d_b11, eq19_e1126_d_b12, eq19_e1126_q,) = {
    if s.b[3407] {
        let eq19_e1123_q: f64 = s.v[68];let eq19_e1124: f64 = (p.p87 * s.v[68]);let eq19_e1124_q: f64 = (p.p87 * eq19_e1123_q);
        (eq19_e1124, (p.p87 * s.dn[68][0]), (p.p87 * s.dn[68][1]), (p.p87 * s.dn[68][2]), (p.p87 * s.dn[68][3]), (p.p87 * s.dn[68][4]), (p.p87 * s.dn[68][5]), (p.p87 * s.dn[68][6]), (p.p87 * s.dn[68][7]), (p.p87 * s.dn[68][8]), (p.p87 * s.dn[68][9]), (p.p87 * s.dn[68][10]), (p.p87 * s.dn[68][11]), (p.p87 * s.dn[68][12]), (p.p87 * s.dn[68][13]), (p.p87 * s.dn[68][14]), (p.p87 * s.dn[68][15]), (p.p87 * s.dn[68][16]), (p.p87 * s.dn[68][17]), (p.p87 * s.dn[68][18]), (p.p87 * s.db[68][0]), (p.p87 * s.db[68][1]), (p.p87 * s.db[68][2]), (p.p87 * s.db[68][3]), (p.p87 * s.db[68][4]), (p.p87 * s.db[68][5]), (p.p87 * s.db[68][6]), (p.p87 * s.db[68][7]), (p.p87 * s.db[68][8]), (p.p87 * s.db[68][9]), (p.p87 * s.db[68][10]), (p.p87 * s.db[68][11]), (p.p87 * s.db[68][12]), eq19_e1124_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e1126_d_n0, eq19_e1126_d_n1, eq19_e1126_d_n2, eq19_e1126_d_n3, eq19_e1126_d_n4, eq19_e1126_d_n5, eq19_e1126_d_n6, eq19_e1126_d_n7, eq19_e1126_d_n8, eq19_e1126_d_n9, eq19_e1126_d_n10, eq19_e1126_d_n11, eq19_e1126_d_n12, eq19_e1126_d_n13, eq19_e1126_d_n14, eq19_e1126_d_n15, eq19_e1126_d_n16, eq19_e1126_d_n17, eq19_e1126_d_n18];let eq19_reactive_branch_derivatives: [f64; 13] = [eq19_e1126_d_b0, eq19_e1126_d_b1, eq19_e1126_d_b2, eq19_e1126_d_b3, eq19_e1126_d_b4, eq19_e1126_d_b5, eq19_e1126_d_b6, eq19_e1126_d_b7, eq19_e1126_d_b8, eq19_e1126_d_b9, eq19_e1126_d_b10, eq19_e1126_d_b11, eq19_e1126_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(8),
            &eq19_reactive_node_derivatives,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1133, eq20_e1133_d_n0, eq20_e1133_d_n1, eq20_e1133_d_n2, eq20_e1133_d_n3, eq20_e1133_d_n4, eq20_e1133_d_n5, eq20_e1133_d_n6, eq20_e1133_d_n7, eq20_e1133_d_n8, eq20_e1133_d_n9, eq20_e1133_d_n10, eq20_e1133_d_n11, eq20_e1133_d_n12, eq20_e1133_d_n13, eq20_e1133_d_n14, eq20_e1133_d_n15, eq20_e1133_d_n16, eq20_e1133_d_n17, eq20_e1133_d_n18, eq20_e1133_d_b0, eq20_e1133_d_b1, eq20_e1133_d_b2, eq20_e1133_d_b3, eq20_e1133_d_b4, eq20_e1133_d_b5, eq20_e1133_d_b6, eq20_e1133_d_b7, eq20_e1133_d_b8, eq20_e1133_d_b9, eq20_e1133_d_b10, eq20_e1133_d_b11, eq20_e1133_d_b12, eq20_e1133_q,) = {
    if s.b[3407] {
        let eq20_e1130_q: f64 = s.v[67];let eq20_e1131: f64 = (p.p87 * s.v[67]);let eq20_e1131_q: f64 = (p.p87 * eq20_e1130_q);
        (eq20_e1131, (p.p87 * s.dn[67][0]), (p.p87 * s.dn[67][1]), (p.p87 * s.dn[67][2]), (p.p87 * s.dn[67][3]), (p.p87 * s.dn[67][4]), (p.p87 * s.dn[67][5]), (p.p87 * s.dn[67][6]), (p.p87 * s.dn[67][7]), (p.p87 * s.dn[67][8]), (p.p87 * s.dn[67][9]), (p.p87 * s.dn[67][10]), (p.p87 * s.dn[67][11]), (p.p87 * s.dn[67][12]), (p.p87 * s.dn[67][13]), (p.p87 * s.dn[67][14]), (p.p87 * s.dn[67][15]), (p.p87 * s.dn[67][16]), (p.p87 * s.dn[67][17]), (p.p87 * s.dn[67][18]), (p.p87 * s.db[67][0]), (p.p87 * s.db[67][1]), (p.p87 * s.db[67][2]), (p.p87 * s.db[67][3]), (p.p87 * s.db[67][4]), (p.p87 * s.db[67][5]), (p.p87 * s.db[67][6]), (p.p87 * s.db[67][7]), (p.p87 * s.db[67][8]), (p.p87 * s.db[67][9]), (p.p87 * s.db[67][10]), (p.p87 * s.db[67][11]), (p.p87 * s.db[67][12]), eq20_e1131_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e1133_d_n0, eq20_e1133_d_n1, eq20_e1133_d_n2, eq20_e1133_d_n3, eq20_e1133_d_n4, eq20_e1133_d_n5, eq20_e1133_d_n6, eq20_e1133_d_n7, eq20_e1133_d_n8, eq20_e1133_d_n9, eq20_e1133_d_n10, eq20_e1133_d_n11, eq20_e1133_d_n12, eq20_e1133_d_n13, eq20_e1133_d_n14, eq20_e1133_d_n15, eq20_e1133_d_n16, eq20_e1133_d_n17, eq20_e1133_d_n18];let eq20_reactive_branch_derivatives: [f64; 13] = [eq20_e1133_d_b0, eq20_e1133_d_b1, eq20_e1133_d_b2, eq20_e1133_d_b3, eq20_e1133_d_b4, eq20_e1133_d_b5, eq20_e1133_d_b6, eq20_e1133_d_b7, eq20_e1133_d_b8, eq20_e1133_d_b9, eq20_e1133_d_b10, eq20_e1133_d_b11, eq20_e1133_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(6),
            &eq20_reactive_node_derivatives,
            &eq20_reactive_branch_derivatives,
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
        let eq28_e1177: f64 = (s.v[18] + s.v[753]);let eq28_e1177_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);let eq28_e1177_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);let eq28_e1177_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);let eq28_e1177_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);let eq28_e1177_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);let eq28_e1177_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);let eq28_e1177_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);let eq28_e1177_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);let eq28_e1177_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);let eq28_e1177_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);let eq28_e1177_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);let eq28_e1177_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);let eq28_e1177_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);let eq28_e1177_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);let eq28_e1177_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);let eq28_e1177_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);let eq28_e1177_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);let eq28_e1177_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);let eq28_e1177_d_n18: f64 = (s.dn[18][18] + s.dn[753][18]);let eq28_e1177_d_b0: f64 = (s.db[18][0] + s.db[753][0]);let eq28_e1177_d_b1: f64 = (s.db[18][1] + s.db[753][1]);let eq28_e1177_d_b2: f64 = (s.db[18][2] + s.db[753][2]);let eq28_e1177_d_b3: f64 = (s.db[18][3] + s.db[753][3]);let eq28_e1177_d_b4: f64 = (s.db[18][4] + s.db[753][4]);let eq28_e1177_d_b5: f64 = (s.db[18][5] + s.db[753][5]);let eq28_e1177_d_b6: f64 = (s.db[18][6] + s.db[753][6]);let eq28_e1177_d_b7: f64 = (s.db[18][7] + s.db[753][7]);let eq28_e1177_d_b8: f64 = (s.db[18][8] + s.db[753][8]);let eq28_e1177_d_b9: f64 = (s.db[18][9] + s.db[753][9]);let eq28_e1177_d_b10: f64 = (s.db[18][10] + s.db[753][10]);let eq28_e1177_d_b11: f64 = (s.db[18][11] + s.db[753][11]);let eq28_e1177_d_b12: f64 = (s.db[18][12] + s.db[753][12]);let eq28_e1178_q: f64 = eq28_e1177;let eq28_e1179: f64 = (p.p87 * eq28_e1177);let eq28_e1179_d_n0: f64 = (p.p87 * eq28_e1177_d_n0);let eq28_e1179_d_n1: f64 = (p.p87 * eq28_e1177_d_n1);let eq28_e1179_d_n2: f64 = (p.p87 * eq28_e1177_d_n2);let eq28_e1179_d_n3: f64 = (p.p87 * eq28_e1177_d_n3);let eq28_e1179_d_n4: f64 = (p.p87 * eq28_e1177_d_n4);let eq28_e1179_d_n5: f64 = (p.p87 * eq28_e1177_d_n5);let eq28_e1179_d_n6: f64 = (p.p87 * eq28_e1177_d_n6);let eq28_e1179_d_n7: f64 = (p.p87 * eq28_e1177_d_n7);let eq28_e1179_d_n8: f64 = (p.p87 * eq28_e1177_d_n8);let eq28_e1179_d_n9: f64 = (p.p87 * eq28_e1177_d_n9);let eq28_e1179_d_n10: f64 = (p.p87 * eq28_e1177_d_n10);let eq28_e1179_d_n11: f64 = (p.p87 * eq28_e1177_d_n11);let eq28_e1179_d_n12: f64 = (p.p87 * eq28_e1177_d_n12);let eq28_e1179_d_n13: f64 = (p.p87 * eq28_e1177_d_n13);let eq28_e1179_d_n14: f64 = (p.p87 * eq28_e1177_d_n14);let eq28_e1179_d_n15: f64 = (p.p87 * eq28_e1177_d_n15);let eq28_e1179_d_n16: f64 = (p.p87 * eq28_e1177_d_n16);let eq28_e1179_d_n17: f64 = (p.p87 * eq28_e1177_d_n17);let eq28_e1179_d_n18: f64 = (p.p87 * eq28_e1177_d_n18);let eq28_e1179_d_b0: f64 = (p.p87 * eq28_e1177_d_b0);let eq28_e1179_d_b1: f64 = (p.p87 * eq28_e1177_d_b1);let eq28_e1179_d_b2: f64 = (p.p87 * eq28_e1177_d_b2);let eq28_e1179_d_b3: f64 = (p.p87 * eq28_e1177_d_b3);let eq28_e1179_d_b4: f64 = (p.p87 * eq28_e1177_d_b4);let eq28_e1179_d_b5: f64 = (p.p87 * eq28_e1177_d_b5);let eq28_e1179_d_b6: f64 = (p.p87 * eq28_e1177_d_b6);let eq28_e1179_d_b7: f64 = (p.p87 * eq28_e1177_d_b7);let eq28_e1179_d_b8: f64 = (p.p87 * eq28_e1177_d_b8);let eq28_e1179_d_b9: f64 = (p.p87 * eq28_e1177_d_b9);let eq28_e1179_d_b10: f64 = (p.p87 * eq28_e1177_d_b10);let eq28_e1179_d_b11: f64 = (p.p87 * eq28_e1177_d_b11);let eq28_e1179_d_b12: f64 = (p.p87 * eq28_e1177_d_b12);let eq28_e1179_q: f64 = (p.p87 * eq28_e1178_q);
        let eq28_reactive_node_derivatives: [f64; 19] = [eq28_e1179_d_n0, eq28_e1179_d_n1, eq28_e1179_d_n2, eq28_e1179_d_n3, eq28_e1179_d_n4, eq28_e1179_d_n5, eq28_e1179_d_n6, eq28_e1179_d_n7, eq28_e1179_d_n8, eq28_e1179_d_n9, eq28_e1179_d_n10, eq28_e1179_d_n11, eq28_e1179_d_n12, eq28_e1179_d_n13, eq28_e1179_d_n14, eq28_e1179_d_n15, eq28_e1179_d_n16, eq28_e1179_d_n17, eq28_e1179_d_n18];let eq28_reactive_branch_derivatives: [f64; 13] = [eq28_e1179_d_b0, eq28_e1179_d_b1, eq28_e1179_d_b2, eq28_e1179_d_b3, eq28_e1179_d_b4, eq28_e1179_d_b5, eq28_e1179_d_b6, eq28_e1179_d_b7, eq28_e1179_d_b8, eq28_e1179_d_b9, eq28_e1179_d_b10, eq28_e1179_d_b11, eq28_e1179_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(8),
            &eq28_reactive_node_derivatives,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq29_e1183: f64 = (s.v[19] + s.v[751]);let eq29_e1183_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);let eq29_e1183_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);let eq29_e1183_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);let eq29_e1183_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);let eq29_e1183_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);let eq29_e1183_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);let eq29_e1183_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);let eq29_e1183_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);let eq29_e1183_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);let eq29_e1183_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);let eq29_e1183_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);let eq29_e1183_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);let eq29_e1183_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);let eq29_e1183_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);let eq29_e1183_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);let eq29_e1183_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);let eq29_e1183_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);let eq29_e1183_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);let eq29_e1183_d_n18: f64 = (s.dn[19][18] + s.dn[751][18]);let eq29_e1183_d_b0: f64 = (s.db[19][0] + s.db[751][0]);let eq29_e1183_d_b1: f64 = (s.db[19][1] + s.db[751][1]);let eq29_e1183_d_b2: f64 = (s.db[19][2] + s.db[751][2]);let eq29_e1183_d_b3: f64 = (s.db[19][3] + s.db[751][3]);let eq29_e1183_d_b4: f64 = (s.db[19][4] + s.db[751][4]);let eq29_e1183_d_b5: f64 = (s.db[19][5] + s.db[751][5]);let eq29_e1183_d_b6: f64 = (s.db[19][6] + s.db[751][6]);let eq29_e1183_d_b7: f64 = (s.db[19][7] + s.db[751][7]);let eq29_e1183_d_b8: f64 = (s.db[19][8] + s.db[751][8]);let eq29_e1183_d_b9: f64 = (s.db[19][9] + s.db[751][9]);let eq29_e1183_d_b10: f64 = (s.db[19][10] + s.db[751][10]);let eq29_e1183_d_b11: f64 = (s.db[19][11] + s.db[751][11]);let eq29_e1183_d_b12: f64 = (s.db[19][12] + s.db[751][12]);let eq29_e1184_q: f64 = eq29_e1183;let eq29_e1185: f64 = (p.p87 * eq29_e1183);let eq29_e1185_d_n0: f64 = (p.p87 * eq29_e1183_d_n0);let eq29_e1185_d_n1: f64 = (p.p87 * eq29_e1183_d_n1);let eq29_e1185_d_n2: f64 = (p.p87 * eq29_e1183_d_n2);let eq29_e1185_d_n3: f64 = (p.p87 * eq29_e1183_d_n3);let eq29_e1185_d_n4: f64 = (p.p87 * eq29_e1183_d_n4);let eq29_e1185_d_n5: f64 = (p.p87 * eq29_e1183_d_n5);let eq29_e1185_d_n6: f64 = (p.p87 * eq29_e1183_d_n6);let eq29_e1185_d_n7: f64 = (p.p87 * eq29_e1183_d_n7);let eq29_e1185_d_n8: f64 = (p.p87 * eq29_e1183_d_n8);let eq29_e1185_d_n9: f64 = (p.p87 * eq29_e1183_d_n9);let eq29_e1185_d_n10: f64 = (p.p87 * eq29_e1183_d_n10);let eq29_e1185_d_n11: f64 = (p.p87 * eq29_e1183_d_n11);let eq29_e1185_d_n12: f64 = (p.p87 * eq29_e1183_d_n12);let eq29_e1185_d_n13: f64 = (p.p87 * eq29_e1183_d_n13);let eq29_e1185_d_n14: f64 = (p.p87 * eq29_e1183_d_n14);let eq29_e1185_d_n15: f64 = (p.p87 * eq29_e1183_d_n15);let eq29_e1185_d_n16: f64 = (p.p87 * eq29_e1183_d_n16);let eq29_e1185_d_n17: f64 = (p.p87 * eq29_e1183_d_n17);let eq29_e1185_d_n18: f64 = (p.p87 * eq29_e1183_d_n18);let eq29_e1185_d_b0: f64 = (p.p87 * eq29_e1183_d_b0);let eq29_e1185_d_b1: f64 = (p.p87 * eq29_e1183_d_b1);let eq29_e1185_d_b2: f64 = (p.p87 * eq29_e1183_d_b2);let eq29_e1185_d_b3: f64 = (p.p87 * eq29_e1183_d_b3);let eq29_e1185_d_b4: f64 = (p.p87 * eq29_e1183_d_b4);let eq29_e1185_d_b5: f64 = (p.p87 * eq29_e1183_d_b5);let eq29_e1185_d_b6: f64 = (p.p87 * eq29_e1183_d_b6);let eq29_e1185_d_b7: f64 = (p.p87 * eq29_e1183_d_b7);let eq29_e1185_d_b8: f64 = (p.p87 * eq29_e1183_d_b8);let eq29_e1185_d_b9: f64 = (p.p87 * eq29_e1183_d_b9);let eq29_e1185_d_b10: f64 = (p.p87 * eq29_e1183_d_b10);let eq29_e1185_d_b11: f64 = (p.p87 * eq29_e1183_d_b11);let eq29_e1185_d_b12: f64 = (p.p87 * eq29_e1183_d_b12);let eq29_e1185_q: f64 = (p.p87 * eq29_e1184_q);
        let eq29_reactive_node_derivatives: [f64; 19] = [eq29_e1185_d_n0, eq29_e1185_d_n1, eq29_e1185_d_n2, eq29_e1185_d_n3, eq29_e1185_d_n4, eq29_e1185_d_n5, eq29_e1185_d_n6, eq29_e1185_d_n7, eq29_e1185_d_n8, eq29_e1185_d_n9, eq29_e1185_d_n10, eq29_e1185_d_n11, eq29_e1185_d_n12, eq29_e1185_d_n13, eq29_e1185_d_n14, eq29_e1185_d_n15, eq29_e1185_d_n16, eq29_e1185_d_n17, eq29_e1185_d_n18];let eq29_reactive_branch_derivatives: [f64; 13] = [eq29_e1185_d_b0, eq29_e1185_d_b1, eq29_e1185_d_b2, eq29_e1185_d_b3, eq29_e1185_d_b4, eq29_e1185_d_b5, eq29_e1185_d_b6, eq29_e1185_d_b7, eq29_e1185_d_b8, eq29_e1185_d_b9, eq29_e1185_d_b10, eq29_e1185_d_b11, eq29_e1185_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(8),
            &eq29_reactive_node_derivatives,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_4(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq30_e1190: f64 = (s.v[753] + s.v[751]);let eq30_e1190_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);let eq30_e1190_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);let eq30_e1190_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);let eq30_e1190_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);let eq30_e1190_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);let eq30_e1190_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);let eq30_e1190_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);let eq30_e1190_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);let eq30_e1190_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);let eq30_e1190_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);let eq30_e1190_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);let eq30_e1190_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);let eq30_e1190_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);let eq30_e1190_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);let eq30_e1190_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);let eq30_e1190_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);let eq30_e1190_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);let eq30_e1190_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);let eq30_e1190_d_n18: f64 = (s.dn[753][18] + s.dn[751][18]);let eq30_e1190_d_b0: f64 = (s.db[753][0] + s.db[751][0]);let eq30_e1190_d_b1: f64 = (s.db[753][1] + s.db[751][1]);let eq30_e1190_d_b2: f64 = (s.db[753][2] + s.db[751][2]);let eq30_e1190_d_b3: f64 = (s.db[753][3] + s.db[751][3]);let eq30_e1190_d_b4: f64 = (s.db[753][4] + s.db[751][4]);let eq30_e1190_d_b5: f64 = (s.db[753][5] + s.db[751][5]);let eq30_e1190_d_b6: f64 = (s.db[753][6] + s.db[751][6]);let eq30_e1190_d_b7: f64 = (s.db[753][7] + s.db[751][7]);let eq30_e1190_d_b8: f64 = (s.db[753][8] + s.db[751][8]);let eq30_e1190_d_b9: f64 = (s.db[753][9] + s.db[751][9]);let eq30_e1190_d_b10: f64 = (s.db[753][10] + s.db[751][10]);let eq30_e1190_d_b11: f64 = (s.db[753][11] + s.db[751][11]);let eq30_e1190_d_b12: f64 = (s.db[753][12] + s.db[751][12]);let eq30_e1192: f64 = (eq30_e1190 + s.v[752]);let eq30_e1192_d_n0: f64 = (eq30_e1190_d_n0 + s.dn[752][0]);let eq30_e1192_d_n1: f64 = (eq30_e1190_d_n1 + s.dn[752][1]);let eq30_e1192_d_n2: f64 = (eq30_e1190_d_n2 + s.dn[752][2]);let eq30_e1192_d_n3: f64 = (eq30_e1190_d_n3 + s.dn[752][3]);let eq30_e1192_d_n4: f64 = (eq30_e1190_d_n4 + s.dn[752][4]);let eq30_e1192_d_n5: f64 = (eq30_e1190_d_n5 + s.dn[752][5]);let eq30_e1192_d_n6: f64 = (eq30_e1190_d_n6 + s.dn[752][6]);let eq30_e1192_d_n7: f64 = (eq30_e1190_d_n7 + s.dn[752][7]);let eq30_e1192_d_n8: f64 = (eq30_e1190_d_n8 + s.dn[752][8]);let eq30_e1192_d_n9: f64 = (eq30_e1190_d_n9 + s.dn[752][9]);let eq30_e1192_d_n10: f64 = (eq30_e1190_d_n10 + s.dn[752][10]);let eq30_e1192_d_n11: f64 = (eq30_e1190_d_n11 + s.dn[752][11]);let eq30_e1192_d_n12: f64 = (eq30_e1190_d_n12 + s.dn[752][12]);let eq30_e1192_d_n13: f64 = (eq30_e1190_d_n13 + s.dn[752][13]);let eq30_e1192_d_n14: f64 = (eq30_e1190_d_n14 + s.dn[752][14]);let eq30_e1192_d_n15: f64 = (eq30_e1190_d_n15 + s.dn[752][15]);let eq30_e1192_d_n16: f64 = (eq30_e1190_d_n16 + s.dn[752][16]);let eq30_e1192_d_n17: f64 = (eq30_e1190_d_n17 + s.dn[752][17]);let eq30_e1192_d_n18: f64 = (eq30_e1190_d_n18 + s.dn[752][18]);let eq30_e1192_d_b0: f64 = (eq30_e1190_d_b0 + s.db[752][0]);let eq30_e1192_d_b1: f64 = (eq30_e1190_d_b1 + s.db[752][1]);let eq30_e1192_d_b2: f64 = (eq30_e1190_d_b2 + s.db[752][2]);let eq30_e1192_d_b3: f64 = (eq30_e1190_d_b3 + s.db[752][3]);let eq30_e1192_d_b4: f64 = (eq30_e1190_d_b4 + s.db[752][4]);let eq30_e1192_d_b5: f64 = (eq30_e1190_d_b5 + s.db[752][5]);let eq30_e1192_d_b6: f64 = (eq30_e1190_d_b6 + s.db[752][6]);let eq30_e1192_d_b7: f64 = (eq30_e1190_d_b7 + s.db[752][7]);let eq30_e1192_d_b8: f64 = (eq30_e1190_d_b8 + s.db[752][8]);let eq30_e1192_d_b9: f64 = (eq30_e1190_d_b9 + s.db[752][9]);let eq30_e1192_d_b10: f64 = (eq30_e1190_d_b10 + s.db[752][10]);let eq30_e1192_d_b11: f64 = (eq30_e1190_d_b11 + s.db[752][11]);let eq30_e1192_d_b12: f64 = (eq30_e1190_d_b12 + s.db[752][12]);let eq30_e1193: f64 = (s.v[20] - eq30_e1192);let eq30_e1193_d_n0: f64 = (s.dn[20][0] - eq30_e1192_d_n0);let eq30_e1193_d_n1: f64 = (s.dn[20][1] - eq30_e1192_d_n1);
        let eq30_e1193_d_n2: f64 = (s.dn[20][2] - eq30_e1192_d_n2);let eq30_e1193_d_n3: f64 = (s.dn[20][3] - eq30_e1192_d_n3);let eq30_e1193_d_n4: f64 = (s.dn[20][4] - eq30_e1192_d_n4);let eq30_e1193_d_n5: f64 = (s.dn[20][5] - eq30_e1192_d_n5);let eq30_e1193_d_n6: f64 = (s.dn[20][6] - eq30_e1192_d_n6);let eq30_e1193_d_n7: f64 = (s.dn[20][7] - eq30_e1192_d_n7);let eq30_e1193_d_n8: f64 = (s.dn[20][8] - eq30_e1192_d_n8);let eq30_e1193_d_n9: f64 = (s.dn[20][9] - eq30_e1192_d_n9);let eq30_e1193_d_n10: f64 = (s.dn[20][10] - eq30_e1192_d_n10);let eq30_e1193_d_n11: f64 = (s.dn[20][11] - eq30_e1192_d_n11);let eq30_e1193_d_n12: f64 = (s.dn[20][12] - eq30_e1192_d_n12);let eq30_e1193_d_n13: f64 = (s.dn[20][13] - eq30_e1192_d_n13);let eq30_e1193_d_n14: f64 = (s.dn[20][14] - eq30_e1192_d_n14);let eq30_e1193_d_n15: f64 = (s.dn[20][15] - eq30_e1192_d_n15);let eq30_e1193_d_n16: f64 = (s.dn[20][16] - eq30_e1192_d_n16);let eq30_e1193_d_n17: f64 = (s.dn[20][17] - eq30_e1192_d_n17);let eq30_e1193_d_n18: f64 = (s.dn[20][18] - eq30_e1192_d_n18);let eq30_e1193_d_b0: f64 = (s.db[20][0] - eq30_e1192_d_b0);let eq30_e1193_d_b1: f64 = (s.db[20][1] - eq30_e1192_d_b1);let eq30_e1193_d_b2: f64 = (s.db[20][2] - eq30_e1192_d_b2);let eq30_e1193_d_b3: f64 = (s.db[20][3] - eq30_e1192_d_b3);let eq30_e1193_d_b4: f64 = (s.db[20][4] - eq30_e1192_d_b4);let eq30_e1193_d_b5: f64 = (s.db[20][5] - eq30_e1192_d_b5);let eq30_e1193_d_b6: f64 = (s.db[20][6] - eq30_e1192_d_b6);let eq30_e1193_d_b7: f64 = (s.db[20][7] - eq30_e1192_d_b7);let eq30_e1193_d_b8: f64 = (s.db[20][8] - eq30_e1192_d_b8);let eq30_e1193_d_b9: f64 = (s.db[20][9] - eq30_e1192_d_b9);let eq30_e1193_d_b10: f64 = (s.db[20][10] - eq30_e1192_d_b10);let eq30_e1193_d_b11: f64 = (s.db[20][11] - eq30_e1192_d_b11);let eq30_e1193_d_b12: f64 = (s.db[20][12] - eq30_e1192_d_b12);let eq30_e1194_q: f64 = eq30_e1193;let eq30_e1195: f64 = (p.p87 * eq30_e1193);let eq30_e1195_d_n0: f64 = (p.p87 * eq30_e1193_d_n0);let eq30_e1195_d_n1: f64 = (p.p87 * eq30_e1193_d_n1);let eq30_e1195_d_n2: f64 = (p.p87 * eq30_e1193_d_n2);let eq30_e1195_d_n3: f64 = (p.p87 * eq30_e1193_d_n3);let eq30_e1195_d_n4: f64 = (p.p87 * eq30_e1193_d_n4);let eq30_e1195_d_n5: f64 = (p.p87 * eq30_e1193_d_n5);let eq30_e1195_d_n6: f64 = (p.p87 * eq30_e1193_d_n6);let eq30_e1195_d_n7: f64 = (p.p87 * eq30_e1193_d_n7);let eq30_e1195_d_n8: f64 = (p.p87 * eq30_e1193_d_n8);let eq30_e1195_d_n9: f64 = (p.p87 * eq30_e1193_d_n9);let eq30_e1195_d_n10: f64 = (p.p87 * eq30_e1193_d_n10);let eq30_e1195_d_n11: f64 = (p.p87 * eq30_e1193_d_n11);let eq30_e1195_d_n12: f64 = (p.p87 * eq30_e1193_d_n12);let eq30_e1195_d_n13: f64 = (p.p87 * eq30_e1193_d_n13);let eq30_e1195_d_n14: f64 = (p.p87 * eq30_e1193_d_n14);let eq30_e1195_d_n15: f64 = (p.p87 * eq30_e1193_d_n15);let eq30_e1195_d_n16: f64 = (p.p87 * eq30_e1193_d_n16);let eq30_e1195_d_n17: f64 = (p.p87 * eq30_e1193_d_n17);let eq30_e1195_d_n18: f64 = (p.p87 * eq30_e1193_d_n18);let eq30_e1195_d_b0: f64 = (p.p87 * eq30_e1193_d_b0);let eq30_e1195_d_b1: f64 = (p.p87 * eq30_e1193_d_b1);let eq30_e1195_d_b2: f64 = (p.p87 * eq30_e1193_d_b2);let eq30_e1195_d_b3: f64 = (p.p87 * eq30_e1193_d_b3);let eq30_e1195_d_b4: f64 = (p.p87 * eq30_e1193_d_b4);let eq30_e1195_d_b5: f64 = (p.p87 * eq30_e1193_d_b5);let eq30_e1195_d_b6: f64 = (p.p87 * eq30_e1193_d_b6);let eq30_e1195_d_b7: f64 = (p.p87 * eq30_e1193_d_b7);let eq30_e1195_d_b8: f64 = (p.p87 * eq30_e1193_d_b8);let eq30_e1195_d_b9: f64 = (p.p87 * eq30_e1193_d_b9);let eq30_e1195_d_b10: f64 = (p.p87 * eq30_e1193_d_b10);let eq30_e1195_d_b11: f64 = (p.p87 * eq30_e1193_d_b11);let eq30_e1195_d_b12: f64 = (p.p87 * eq30_e1193_d_b12);let eq30_e1195_q: f64 = (p.p87 * eq30_e1194_q);let eq30_reactive_node_derivatives: [f64; 19] = [eq30_e1195_d_n0, eq30_e1195_d_n1, eq30_e1195_d_n2, eq30_e1195_d_n3, eq30_e1195_d_n4, eq30_e1195_d_n5, eq30_e1195_d_n6, eq30_e1195_d_n7, eq30_e1195_d_n8, eq30_e1195_d_n9, eq30_e1195_d_n10, eq30_e1195_d_n11, eq30_e1195_d_n12, eq30_e1195_d_n13, eq30_e1195_d_n14, eq30_e1195_d_n15, eq30_e1195_d_n16, eq30_e1195_d_n17, eq30_e1195_d_n18];
        let eq30_reactive_branch_derivatives: [f64; 13] = [eq30_e1195_d_b0, eq30_e1195_d_b1, eq30_e1195_d_b2, eq30_e1195_d_b3, eq30_e1195_d_b4, eq30_e1195_d_b5, eq30_e1195_d_b6, eq30_e1195_d_b7, eq30_e1195_d_b8, eq30_e1195_d_b9, eq30_e1195_d_b10, eq30_e1195_d_b11, eq30_e1195_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(8),
            &eq30_reactive_node_derivatives,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);let eq31_e1198_q: f64 = s.v[743];let eq31_e1199: f64 = (p.p87 * s.v[743]);let eq31_e1199_q: f64 = (p.p87 * eq31_e1198_q);
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(2),
            &s.dn[743],
            &s.db[743],
            (multiplicity) * (p.p87),
        );let eq32_e1202_q: f64 = s.v[742];let eq32_e1203: f64 = (p.p87 * s.v[742]);let eq32_e1203_q: f64 = (p.p87 * eq32_e1202_q);
        stamper.stamp_current_reactive_dense_local(
            Some(0),
            Some(2),
            &s.dn[742],
            &s.db[742],
            (multiplicity) * (p.p87),
        );let eq33_e1206_q: f64 = s.v[744];let eq33_e1207: f64 = (p.p87 * s.v[744]);let eq33_e1207_q: f64 = (p.p87 * eq33_e1206_q);
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(2),
            &s.dn[744],
            &s.db[744],
            (multiplicity) * (p.p87),
        );let eq34_e1209: f64 = (-p.p87);let eq34_e1211_q: f64 = s.v[299];let eq34_e1212: f64 = (eq34_e1209 * s.v[299]);let eq34_e1212_q: f64 = (eq34_e1209 * eq34_e1211_q);
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(0),
            &s.dn[299],
            &s.db[299],
            (multiplicity) * (eq34_e1209),
        );let eq35_e1214: f64 = (-p.p87);let eq35_e1216_q: f64 = s.v[301];let eq35_e1217: f64 = (eq35_e1214 * s.v[301]);let eq35_e1217_q: f64 = (eq35_e1214 * eq35_e1216_q);
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
    if (p.p28 != 0.0) {
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
    if (p.p28 != 0.0) {
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
    if (p.p29 != 0.0) {
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
