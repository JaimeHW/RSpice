#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq27_e1173: f64 = (s.v[18] + s.v[753]);let eq27_e1173_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);let eq27_e1173_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);let eq27_e1173_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);let eq27_e1173_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);let eq27_e1173_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);let eq27_e1173_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);let eq27_e1173_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);let eq27_e1173_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);let eq27_e1173_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);let eq27_e1173_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);let eq27_e1173_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);let eq27_e1173_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);let eq27_e1173_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);let eq27_e1173_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);let eq27_e1173_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);let eq27_e1173_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);let eq27_e1173_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);let eq27_e1173_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);let eq27_e1173_d_b0: f64 = (s.db[18][0] + s.db[753][0]);let eq27_e1173_d_b1: f64 = (s.db[18][1] + s.db[753][1]);let eq27_e1173_d_b2: f64 = (s.db[18][2] + s.db[753][2]);let eq27_e1173_d_b3: f64 = (s.db[18][3] + s.db[753][3]);let eq27_e1173_d_b4: f64 = (s.db[18][4] + s.db[753][4]);let eq27_e1173_d_b5: f64 = (s.db[18][5] + s.db[753][5]);let eq27_e1173_d_b6: f64 = (s.db[18][6] + s.db[753][6]);let eq27_e1173_d_b7: f64 = (s.db[18][7] + s.db[753][7]);let eq27_e1173_d_b8: f64 = (s.db[18][8] + s.db[753][8]);let eq27_e1173_d_b9: f64 = (s.db[18][9] + s.db[753][9]);let eq27_e1173_d_b10: f64 = (s.db[18][10] + s.db[753][10]);let eq27_e1173_d_b11: f64 = (s.db[18][11] + s.db[753][11]);let eq27_e1174_q: f64 = eq27_e1173;let eq27_e1175: f64 = (p.p87 * eq27_e1173);let eq27_e1175_d_n0: f64 = (p.p87 * eq27_e1173_d_n0);let eq27_e1175_d_n1: f64 = (p.p87 * eq27_e1173_d_n1);let eq27_e1175_d_n2: f64 = (p.p87 * eq27_e1173_d_n2);let eq27_e1175_d_n3: f64 = (p.p87 * eq27_e1173_d_n3);let eq27_e1175_d_n4: f64 = (p.p87 * eq27_e1173_d_n4);let eq27_e1175_d_n5: f64 = (p.p87 * eq27_e1173_d_n5);let eq27_e1175_d_n6: f64 = (p.p87 * eq27_e1173_d_n6);let eq27_e1175_d_n7: f64 = (p.p87 * eq27_e1173_d_n7);let eq27_e1175_d_n8: f64 = (p.p87 * eq27_e1173_d_n8);let eq27_e1175_d_n9: f64 = (p.p87 * eq27_e1173_d_n9);let eq27_e1175_d_n10: f64 = (p.p87 * eq27_e1173_d_n10);let eq27_e1175_d_n11: f64 = (p.p87 * eq27_e1173_d_n11);let eq27_e1175_d_n12: f64 = (p.p87 * eq27_e1173_d_n12);let eq27_e1175_d_n13: f64 = (p.p87 * eq27_e1173_d_n13);let eq27_e1175_d_n14: f64 = (p.p87 * eq27_e1173_d_n14);let eq27_e1175_d_n15: f64 = (p.p87 * eq27_e1173_d_n15);let eq27_e1175_d_n16: f64 = (p.p87 * eq27_e1173_d_n16);let eq27_e1175_d_n17: f64 = (p.p87 * eq27_e1173_d_n17);let eq27_e1175_d_b0: f64 = (p.p87 * eq27_e1173_d_b0);let eq27_e1175_d_b1: f64 = (p.p87 * eq27_e1173_d_b1);let eq27_e1175_d_b2: f64 = (p.p87 * eq27_e1173_d_b2);let eq27_e1175_d_b3: f64 = (p.p87 * eq27_e1173_d_b3);let eq27_e1175_d_b4: f64 = (p.p87 * eq27_e1173_d_b4);let eq27_e1175_d_b5: f64 = (p.p87 * eq27_e1173_d_b5);let eq27_e1175_d_b6: f64 = (p.p87 * eq27_e1173_d_b6);let eq27_e1175_d_b7: f64 = (p.p87 * eq27_e1173_d_b7);let eq27_e1175_d_b8: f64 = (p.p87 * eq27_e1173_d_b8);let eq27_e1175_d_b9: f64 = (p.p87 * eq27_e1173_d_b9);let eq27_e1175_d_b10: f64 = (p.p87 * eq27_e1173_d_b10);let eq27_e1175_d_b11: f64 = (p.p87 * eq27_e1173_d_b11);let eq27_e1175_q: f64 = (p.p87 * eq27_e1174_q);let eq27_reactive_node_derivatives: [f64; 18] = [eq27_e1175_d_n0, eq27_e1175_d_n1, eq27_e1175_d_n2, eq27_e1175_d_n3, eq27_e1175_d_n4, eq27_e1175_d_n5, eq27_e1175_d_n6, eq27_e1175_d_n7, eq27_e1175_d_n8, eq27_e1175_d_n9, eq27_e1175_d_n10, eq27_e1175_d_n11, eq27_e1175_d_n12, eq27_e1175_d_n13, eq27_e1175_d_n14, eq27_e1175_d_n15, eq27_e1175_d_n16, eq27_e1175_d_n17];
        let eq27_reactive_branch_derivatives: [f64; 12] = [eq27_e1175_d_b0, eq27_e1175_d_b1, eq27_e1175_d_b2, eq27_e1175_d_b3, eq27_e1175_d_b4, eq27_e1175_d_b5, eq27_e1175_d_b6, eq27_e1175_d_b7, eq27_e1175_d_b8, eq27_e1175_d_b9, eq27_e1175_d_b10, eq27_e1175_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(7),
            &eq27_reactive_node_derivatives,
            &eq27_reactive_branch_derivatives,
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
        let eq28_e1179: f64 = (s.v[19] + s.v[751]);let eq28_e1179_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);let eq28_e1179_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);let eq28_e1179_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);let eq28_e1179_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);let eq28_e1179_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);let eq28_e1179_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);let eq28_e1179_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);let eq28_e1179_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);let eq28_e1179_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);let eq28_e1179_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);let eq28_e1179_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);let eq28_e1179_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);let eq28_e1179_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);let eq28_e1179_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);let eq28_e1179_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);let eq28_e1179_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);let eq28_e1179_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);let eq28_e1179_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);let eq28_e1179_d_b0: f64 = (s.db[19][0] + s.db[751][0]);let eq28_e1179_d_b1: f64 = (s.db[19][1] + s.db[751][1]);let eq28_e1179_d_b2: f64 = (s.db[19][2] + s.db[751][2]);let eq28_e1179_d_b3: f64 = (s.db[19][3] + s.db[751][3]);let eq28_e1179_d_b4: f64 = (s.db[19][4] + s.db[751][4]);let eq28_e1179_d_b5: f64 = (s.db[19][5] + s.db[751][5]);let eq28_e1179_d_b6: f64 = (s.db[19][6] + s.db[751][6]);let eq28_e1179_d_b7: f64 = (s.db[19][7] + s.db[751][7]);let eq28_e1179_d_b8: f64 = (s.db[19][8] + s.db[751][8]);let eq28_e1179_d_b9: f64 = (s.db[19][9] + s.db[751][9]);let eq28_e1179_d_b10: f64 = (s.db[19][10] + s.db[751][10]);let eq28_e1179_d_b11: f64 = (s.db[19][11] + s.db[751][11]);let eq28_e1180_q: f64 = eq28_e1179;let eq28_e1181: f64 = (p.p87 * eq28_e1179);let eq28_e1181_d_n0: f64 = (p.p87 * eq28_e1179_d_n0);let eq28_e1181_d_n1: f64 = (p.p87 * eq28_e1179_d_n1);let eq28_e1181_d_n2: f64 = (p.p87 * eq28_e1179_d_n2);let eq28_e1181_d_n3: f64 = (p.p87 * eq28_e1179_d_n3);let eq28_e1181_d_n4: f64 = (p.p87 * eq28_e1179_d_n4);let eq28_e1181_d_n5: f64 = (p.p87 * eq28_e1179_d_n5);let eq28_e1181_d_n6: f64 = (p.p87 * eq28_e1179_d_n6);let eq28_e1181_d_n7: f64 = (p.p87 * eq28_e1179_d_n7);let eq28_e1181_d_n8: f64 = (p.p87 * eq28_e1179_d_n8);let eq28_e1181_d_n9: f64 = (p.p87 * eq28_e1179_d_n9);let eq28_e1181_d_n10: f64 = (p.p87 * eq28_e1179_d_n10);let eq28_e1181_d_n11: f64 = (p.p87 * eq28_e1179_d_n11);let eq28_e1181_d_n12: f64 = (p.p87 * eq28_e1179_d_n12);let eq28_e1181_d_n13: f64 = (p.p87 * eq28_e1179_d_n13);let eq28_e1181_d_n14: f64 = (p.p87 * eq28_e1179_d_n14);let eq28_e1181_d_n15: f64 = (p.p87 * eq28_e1179_d_n15);let eq28_e1181_d_n16: f64 = (p.p87 * eq28_e1179_d_n16);let eq28_e1181_d_n17: f64 = (p.p87 * eq28_e1179_d_n17);let eq28_e1181_d_b0: f64 = (p.p87 * eq28_e1179_d_b0);let eq28_e1181_d_b1: f64 = (p.p87 * eq28_e1179_d_b1);let eq28_e1181_d_b2: f64 = (p.p87 * eq28_e1179_d_b2);let eq28_e1181_d_b3: f64 = (p.p87 * eq28_e1179_d_b3);let eq28_e1181_d_b4: f64 = (p.p87 * eq28_e1179_d_b4);let eq28_e1181_d_b5: f64 = (p.p87 * eq28_e1179_d_b5);let eq28_e1181_d_b6: f64 = (p.p87 * eq28_e1179_d_b6);let eq28_e1181_d_b7: f64 = (p.p87 * eq28_e1179_d_b7);let eq28_e1181_d_b8: f64 = (p.p87 * eq28_e1179_d_b8);let eq28_e1181_d_b9: f64 = (p.p87 * eq28_e1179_d_b9);let eq28_e1181_d_b10: f64 = (p.p87 * eq28_e1179_d_b10);let eq28_e1181_d_b11: f64 = (p.p87 * eq28_e1179_d_b11);let eq28_e1181_q: f64 = (p.p87 * eq28_e1180_q);let eq28_reactive_node_derivatives: [f64; 18] = [eq28_e1181_d_n0, eq28_e1181_d_n1, eq28_e1181_d_n2, eq28_e1181_d_n3, eq28_e1181_d_n4, eq28_e1181_d_n5, eq28_e1181_d_n6, eq28_e1181_d_n7, eq28_e1181_d_n8, eq28_e1181_d_n9, eq28_e1181_d_n10, eq28_e1181_d_n11, eq28_e1181_d_n12, eq28_e1181_d_n13, eq28_e1181_d_n14, eq28_e1181_d_n15, eq28_e1181_d_n16, eq28_e1181_d_n17];
        let eq28_reactive_branch_derivatives: [f64; 12] = [eq28_e1181_d_b0, eq28_e1181_d_b1, eq28_e1181_d_b2, eq28_e1181_d_b3, eq28_e1181_d_b4, eq28_e1181_d_b5, eq28_e1181_d_b6, eq28_e1181_d_b7, eq28_e1181_d_b8, eq28_e1181_d_b9, eq28_e1181_d_b10, eq28_e1181_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(7),
            &eq28_reactive_node_derivatives,
            &eq28_reactive_branch_derivatives,
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
        let eq29_e1186: f64 = (s.v[753] + s.v[751]);let eq29_e1186_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);let eq29_e1186_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);let eq29_e1186_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);let eq29_e1186_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);let eq29_e1186_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);let eq29_e1186_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);let eq29_e1186_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);let eq29_e1186_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);let eq29_e1186_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);let eq29_e1186_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);let eq29_e1186_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);let eq29_e1186_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);let eq29_e1186_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);let eq29_e1186_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);let eq29_e1186_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);let eq29_e1186_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);let eq29_e1186_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);let eq29_e1186_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);let eq29_e1186_d_b0: f64 = (s.db[753][0] + s.db[751][0]);let eq29_e1186_d_b1: f64 = (s.db[753][1] + s.db[751][1]);let eq29_e1186_d_b2: f64 = (s.db[753][2] + s.db[751][2]);let eq29_e1186_d_b3: f64 = (s.db[753][3] + s.db[751][3]);let eq29_e1186_d_b4: f64 = (s.db[753][4] + s.db[751][4]);let eq29_e1186_d_b5: f64 = (s.db[753][5] + s.db[751][5]);let eq29_e1186_d_b6: f64 = (s.db[753][6] + s.db[751][6]);let eq29_e1186_d_b7: f64 = (s.db[753][7] + s.db[751][7]);let eq29_e1186_d_b8: f64 = (s.db[753][8] + s.db[751][8]);let eq29_e1186_d_b9: f64 = (s.db[753][9] + s.db[751][9]);let eq29_e1186_d_b10: f64 = (s.db[753][10] + s.db[751][10]);let eq29_e1186_d_b11: f64 = (s.db[753][11] + s.db[751][11]);let eq29_e1188: f64 = (eq29_e1186 + s.v[752]);let eq29_e1188_d_n0: f64 = (eq29_e1186_d_n0 + s.dn[752][0]);let eq29_e1188_d_n1: f64 = (eq29_e1186_d_n1 + s.dn[752][1]);let eq29_e1188_d_n2: f64 = (eq29_e1186_d_n2 + s.dn[752][2]);let eq29_e1188_d_n3: f64 = (eq29_e1186_d_n3 + s.dn[752][3]);let eq29_e1188_d_n4: f64 = (eq29_e1186_d_n4 + s.dn[752][4]);let eq29_e1188_d_n5: f64 = (eq29_e1186_d_n5 + s.dn[752][5]);let eq29_e1188_d_n6: f64 = (eq29_e1186_d_n6 + s.dn[752][6]);let eq29_e1188_d_n7: f64 = (eq29_e1186_d_n7 + s.dn[752][7]);let eq29_e1188_d_n8: f64 = (eq29_e1186_d_n8 + s.dn[752][8]);let eq29_e1188_d_n9: f64 = (eq29_e1186_d_n9 + s.dn[752][9]);let eq29_e1188_d_n10: f64 = (eq29_e1186_d_n10 + s.dn[752][10]);let eq29_e1188_d_n11: f64 = (eq29_e1186_d_n11 + s.dn[752][11]);let eq29_e1188_d_n12: f64 = (eq29_e1186_d_n12 + s.dn[752][12]);let eq29_e1188_d_n13: f64 = (eq29_e1186_d_n13 + s.dn[752][13]);let eq29_e1188_d_n14: f64 = (eq29_e1186_d_n14 + s.dn[752][14]);let eq29_e1188_d_n15: f64 = (eq29_e1186_d_n15 + s.dn[752][15]);let eq29_e1188_d_n16: f64 = (eq29_e1186_d_n16 + s.dn[752][16]);let eq29_e1188_d_n17: f64 = (eq29_e1186_d_n17 + s.dn[752][17]);let eq29_e1188_d_b0: f64 = (eq29_e1186_d_b0 + s.db[752][0]);let eq29_e1188_d_b1: f64 = (eq29_e1186_d_b1 + s.db[752][1]);let eq29_e1188_d_b2: f64 = (eq29_e1186_d_b2 + s.db[752][2]);let eq29_e1188_d_b3: f64 = (eq29_e1186_d_b3 + s.db[752][3]);let eq29_e1188_d_b4: f64 = (eq29_e1186_d_b4 + s.db[752][4]);let eq29_e1188_d_b5: f64 = (eq29_e1186_d_b5 + s.db[752][5]);let eq29_e1188_d_b6: f64 = (eq29_e1186_d_b6 + s.db[752][6]);let eq29_e1188_d_b7: f64 = (eq29_e1186_d_b7 + s.db[752][7]);let eq29_e1188_d_b8: f64 = (eq29_e1186_d_b8 + s.db[752][8]);let eq29_e1188_d_b9: f64 = (eq29_e1186_d_b9 + s.db[752][9]);let eq29_e1188_d_b10: f64 = (eq29_e1186_d_b10 + s.db[752][10]);let eq29_e1188_d_b11: f64 = (eq29_e1186_d_b11 + s.db[752][11]);let eq29_e1189: f64 = (s.v[20] - eq29_e1188);let eq29_e1189_d_n0: f64 = (s.dn[20][0] - eq29_e1188_d_n0);let eq29_e1189_d_n1: f64 = (s.dn[20][1] - eq29_e1188_d_n1);let eq29_e1189_d_n2: f64 = (s.dn[20][2] - eq29_e1188_d_n2);let eq29_e1189_d_n3: f64 = (s.dn[20][3] - eq29_e1188_d_n3);let eq29_e1189_d_n4: f64 = (s.dn[20][4] - eq29_e1188_d_n4);let eq29_e1189_d_n5: f64 = (s.dn[20][5] - eq29_e1188_d_n5);
        let eq29_e1189_d_n6: f64 = (s.dn[20][6] - eq29_e1188_d_n6);let eq29_e1189_d_n7: f64 = (s.dn[20][7] - eq29_e1188_d_n7);let eq29_e1189_d_n8: f64 = (s.dn[20][8] - eq29_e1188_d_n8);let eq29_e1189_d_n9: f64 = (s.dn[20][9] - eq29_e1188_d_n9);let eq29_e1189_d_n10: f64 = (s.dn[20][10] - eq29_e1188_d_n10);let eq29_e1189_d_n11: f64 = (s.dn[20][11] - eq29_e1188_d_n11);let eq29_e1189_d_n12: f64 = (s.dn[20][12] - eq29_e1188_d_n12);let eq29_e1189_d_n13: f64 = (s.dn[20][13] - eq29_e1188_d_n13);let eq29_e1189_d_n14: f64 = (s.dn[20][14] - eq29_e1188_d_n14);let eq29_e1189_d_n15: f64 = (s.dn[20][15] - eq29_e1188_d_n15);let eq29_e1189_d_n16: f64 = (s.dn[20][16] - eq29_e1188_d_n16);let eq29_e1189_d_n17: f64 = (s.dn[20][17] - eq29_e1188_d_n17);let eq29_e1189_d_b0: f64 = (s.db[20][0] - eq29_e1188_d_b0);let eq29_e1189_d_b1: f64 = (s.db[20][1] - eq29_e1188_d_b1);let eq29_e1189_d_b2: f64 = (s.db[20][2] - eq29_e1188_d_b2);let eq29_e1189_d_b3: f64 = (s.db[20][3] - eq29_e1188_d_b3);let eq29_e1189_d_b4: f64 = (s.db[20][4] - eq29_e1188_d_b4);let eq29_e1189_d_b5: f64 = (s.db[20][5] - eq29_e1188_d_b5);let eq29_e1189_d_b6: f64 = (s.db[20][6] - eq29_e1188_d_b6);let eq29_e1189_d_b7: f64 = (s.db[20][7] - eq29_e1188_d_b7);let eq29_e1189_d_b8: f64 = (s.db[20][8] - eq29_e1188_d_b8);let eq29_e1189_d_b9: f64 = (s.db[20][9] - eq29_e1188_d_b9);let eq29_e1189_d_b10: f64 = (s.db[20][10] - eq29_e1188_d_b10);let eq29_e1189_d_b11: f64 = (s.db[20][11] - eq29_e1188_d_b11);let eq29_e1190_q: f64 = eq29_e1189;let eq29_e1191: f64 = (p.p87 * eq29_e1189);let eq29_e1191_d_n0: f64 = (p.p87 * eq29_e1189_d_n0);let eq29_e1191_d_n1: f64 = (p.p87 * eq29_e1189_d_n1);let eq29_e1191_d_n2: f64 = (p.p87 * eq29_e1189_d_n2);let eq29_e1191_d_n3: f64 = (p.p87 * eq29_e1189_d_n3);let eq29_e1191_d_n4: f64 = (p.p87 * eq29_e1189_d_n4);let eq29_e1191_d_n5: f64 = (p.p87 * eq29_e1189_d_n5);let eq29_e1191_d_n6: f64 = (p.p87 * eq29_e1189_d_n6);let eq29_e1191_d_n7: f64 = (p.p87 * eq29_e1189_d_n7);let eq29_e1191_d_n8: f64 = (p.p87 * eq29_e1189_d_n8);let eq29_e1191_d_n9: f64 = (p.p87 * eq29_e1189_d_n9);let eq29_e1191_d_n10: f64 = (p.p87 * eq29_e1189_d_n10);let eq29_e1191_d_n11: f64 = (p.p87 * eq29_e1189_d_n11);let eq29_e1191_d_n12: f64 = (p.p87 * eq29_e1189_d_n12);let eq29_e1191_d_n13: f64 = (p.p87 * eq29_e1189_d_n13);let eq29_e1191_d_n14: f64 = (p.p87 * eq29_e1189_d_n14);let eq29_e1191_d_n15: f64 = (p.p87 * eq29_e1189_d_n15);let eq29_e1191_d_n16: f64 = (p.p87 * eq29_e1189_d_n16);let eq29_e1191_d_n17: f64 = (p.p87 * eq29_e1189_d_n17);let eq29_e1191_d_b0: f64 = (p.p87 * eq29_e1189_d_b0);let eq29_e1191_d_b1: f64 = (p.p87 * eq29_e1189_d_b1);let eq29_e1191_d_b2: f64 = (p.p87 * eq29_e1189_d_b2);let eq29_e1191_d_b3: f64 = (p.p87 * eq29_e1189_d_b3);let eq29_e1191_d_b4: f64 = (p.p87 * eq29_e1189_d_b4);let eq29_e1191_d_b5: f64 = (p.p87 * eq29_e1189_d_b5);let eq29_e1191_d_b6: f64 = (p.p87 * eq29_e1189_d_b6);let eq29_e1191_d_b7: f64 = (p.p87 * eq29_e1189_d_b7);let eq29_e1191_d_b8: f64 = (p.p87 * eq29_e1189_d_b8);let eq29_e1191_d_b9: f64 = (p.p87 * eq29_e1189_d_b9);let eq29_e1191_d_b10: f64 = (p.p87 * eq29_e1189_d_b10);let eq29_e1191_d_b11: f64 = (p.p87 * eq29_e1189_d_b11);let eq29_e1191_q: f64 = (p.p87 * eq29_e1190_q);let eq29_reactive_node_derivatives: [f64; 18] = [eq29_e1191_d_n0, eq29_e1191_d_n1, eq29_e1191_d_n2, eq29_e1191_d_n3, eq29_e1191_d_n4, eq29_e1191_d_n5, eq29_e1191_d_n6, eq29_e1191_d_n7, eq29_e1191_d_n8, eq29_e1191_d_n9, eq29_e1191_d_n10, eq29_e1191_d_n11, eq29_e1191_d_n12, eq29_e1191_d_n13, eq29_e1191_d_n14, eq29_e1191_d_n15, eq29_e1191_d_n16, eq29_e1191_d_n17];let eq29_reactive_branch_derivatives: [f64; 12] = [eq29_e1191_d_b0, eq29_e1191_d_b1, eq29_e1191_d_b2, eq29_e1191_d_b3, eq29_e1191_d_b4, eq29_e1191_d_b5, eq29_e1191_d_b6, eq29_e1191_d_b7, eq29_e1191_d_b8, eq29_e1191_d_b9, eq29_e1191_d_b10, eq29_e1191_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(7),
            &eq29_reactive_node_derivatives,
            &eq29_reactive_branch_derivatives,
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
        let nv14 = ctx.node_voltage(nodes[14]);let eq30_e1194_q: f64 = s.v[743];let eq30_e1195: f64 = (p.p87 * s.v[743]);let eq30_e1195_q: f64 = (p.p87 * eq30_e1194_q);
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(2),
            &s.dn[743],
            &s.db[743],
            (multiplicity) * (p.p87),
        );let eq31_e1198_q: f64 = s.v[742];let eq31_e1199: f64 = (p.p87 * s.v[742]);let eq31_e1199_q: f64 = (p.p87 * eq31_e1198_q);
        stamper.stamp_current_reactive_dense_local(
            Some(0),
            Some(2),
            &s.dn[742],
            &s.db[742],
            (multiplicity) * (p.p87),
        );let eq32_e1202_q: f64 = s.v[744];let eq32_e1203: f64 = (p.p87 * s.v[744]);let eq32_e1203_q: f64 = (p.p87 * eq32_e1202_q);
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(2),
            &s.dn[744],
            &s.db[744],
            (multiplicity) * (p.p87),
        );let eq33_e1205: f64 = (-p.p87);let eq33_e1207_q: f64 = s.v[299];let eq33_e1208: f64 = (eq33_e1205 * s.v[299]);let eq33_e1208_q: f64 = (eq33_e1205 * eq33_e1207_q);
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(0),
            &s.dn[299],
            &s.db[299],
            (multiplicity) * (eq33_e1205),
        );let eq34_e1210: f64 = (-p.p87);let eq34_e1212_q: f64 = s.v[301];let eq34_e1213: f64 = (eq34_e1210 * s.v[301]);let eq34_e1213_q: f64 = (eq34_e1210 * eq34_e1212_q);
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(2),
            &s.dn[301],
            &s.db[301],
            (multiplicity) * (eq34_e1210),
        );let eq40_e1242: f64 = ((nv14 - 0.0) * s.v[954]);let eq40_e1242_d_n0: f64 = ((nv14 - 0.0) * s.dn[954][0]);let eq40_e1242_d_n1: f64 = ((nv14 - 0.0) * s.dn[954][1]);let eq40_e1242_d_n2: f64 = ((nv14 - 0.0) * s.dn[954][2]);let eq40_e1242_d_n3: f64 = ((nv14 - 0.0) * s.dn[954][3]);let eq40_e1242_d_n4: f64 = ((nv14 - 0.0) * s.dn[954][4]);let eq40_e1242_d_n5: f64 = ((nv14 - 0.0) * s.dn[954][5]);let eq40_e1242_d_n6: f64 = ((nv14 - 0.0) * s.dn[954][6]);let eq40_e1242_d_n7: f64 = ((nv14 - 0.0) * s.dn[954][7]);let eq40_e1242_d_n8: f64 = ((nv14 - 0.0) * s.dn[954][8]);let eq40_e1242_d_n9: f64 = ((nv14 - 0.0) * s.dn[954][9]);let eq40_e1242_d_n10: f64 = ((nv14 - 0.0) * s.dn[954][10]);let eq40_e1242_d_n11: f64 = ((nv14 - 0.0) * s.dn[954][11]);let eq40_e1242_d_n12: f64 = ((nv14 - 0.0) * s.dn[954][12]);let eq40_e1242_d_n13: f64 = ((nv14 - 0.0) * s.dn[954][13]);let eq40_e1242_d_n14: f64 = (s.v[954] + ((nv14 - 0.0) * s.dn[954][14]));let eq40_e1242_d_n15: f64 = ((nv14 - 0.0) * s.dn[954][15]);let eq40_e1242_d_n16: f64 = ((nv14 - 0.0) * s.dn[954][16]);let eq40_e1242_d_n17: f64 = ((nv14 - 0.0) * s.dn[954][17]);let eq40_e1242_d_b0: f64 = ((nv14 - 0.0) * s.db[954][0]);let eq40_e1242_d_b1: f64 = ((nv14 - 0.0) * s.db[954][1]);let eq40_e1242_d_b2: f64 = ((nv14 - 0.0) * s.db[954][2]);let eq40_e1242_d_b3: f64 = ((nv14 - 0.0) * s.db[954][3]);let eq40_e1242_d_b4: f64 = ((nv14 - 0.0) * s.db[954][4]);let eq40_e1242_d_b5: f64 = ((nv14 - 0.0) * s.db[954][5]);let eq40_e1242_d_b6: f64 = ((nv14 - 0.0) * s.db[954][6]);let eq40_e1242_d_b7: f64 = ((nv14 - 0.0) * s.db[954][7]);let eq40_e1242_d_b8: f64 = ((nv14 - 0.0) * s.db[954][8]);let eq40_e1242_d_b9: f64 = ((nv14 - 0.0) * s.db[954][9]);let eq40_e1242_d_b10: f64 = ((nv14 - 0.0) * s.db[954][10]);let eq40_e1242_d_b11: f64 = ((nv14 - 0.0) * s.db[954][11]);let eq40_e1243_q: f64 = eq40_e1242;let eq40_reactive_node_derivatives: [f64; 18] = [eq40_e1242_d_n0, eq40_e1242_d_n1, eq40_e1242_d_n2, eq40_e1242_d_n3, eq40_e1242_d_n4, eq40_e1242_d_n5, eq40_e1242_d_n6, eq40_e1242_d_n7, eq40_e1242_d_n8, eq40_e1242_d_n9, eq40_e1242_d_n10, eq40_e1242_d_n11, eq40_e1242_d_n12, eq40_e1242_d_n13, eq40_e1242_d_n14, eq40_e1242_d_n15, eq40_e1242_d_n16, eq40_e1242_d_n17];let eq40_reactive_branch_derivatives: [f64; 12] = [eq40_e1242_d_b0, eq40_e1242_d_b1, eq40_e1242_d_b2, eq40_e1242_d_b3, eq40_e1242_d_b4, eq40_e1242_d_b5, eq40_e1242_d_b6, eq40_e1242_d_b7, eq40_e1242_d_b8, eq40_e1242_d_b9, eq40_e1242_d_b10, eq40_e1242_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(7),
            &eq40_reactive_node_derivatives,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );let eq41_e1246: f64 = ((nv14 - 0.0) * s.v[955]);let eq41_e1246_d_n0: f64 = ((nv14 - 0.0) * s.dn[955][0]);let eq41_e1246_d_n1: f64 = ((nv14 - 0.0) * s.dn[955][1]);let eq41_e1246_d_n2: f64 = ((nv14 - 0.0) * s.dn[955][2]);let eq41_e1246_d_n3: f64 = ((nv14 - 0.0) * s.dn[955][3]);let eq41_e1246_d_n4: f64 = ((nv14 - 0.0) * s.dn[955][4]);let eq41_e1246_d_n5: f64 = ((nv14 - 0.0) * s.dn[955][5]);let eq41_e1246_d_n6: f64 = ((nv14 - 0.0) * s.dn[955][6]);let eq41_e1246_d_n7: f64 = ((nv14 - 0.0) * s.dn[955][7]);let eq41_e1246_d_n8: f64 = ((nv14 - 0.0) * s.dn[955][8]);let eq41_e1246_d_n9: f64 = ((nv14 - 0.0) * s.dn[955][9]);let eq41_e1246_d_n10: f64 = ((nv14 - 0.0) * s.dn[955][10]);let eq41_e1246_d_n11: f64 = ((nv14 - 0.0) * s.dn[955][11]);let eq41_e1246_d_n12: f64 = ((nv14 - 0.0) * s.dn[955][12]);let eq41_e1246_d_n13: f64 = ((nv14 - 0.0) * s.dn[955][13]);let eq41_e1246_d_n14: f64 = (s.v[955] + ((nv14 - 0.0) * s.dn[955][14]));let eq41_e1246_d_n15: f64 = ((nv14 - 0.0) * s.dn[955][15]);let eq41_e1246_d_n16: f64 = ((nv14 - 0.0) * s.dn[955][16]);let eq41_e1246_d_n17: f64 = ((nv14 - 0.0) * s.dn[955][17]);let eq41_e1246_d_b0: f64 = ((nv14 - 0.0) * s.db[955][0]);let eq41_e1246_d_b1: f64 = ((nv14 - 0.0) * s.db[955][1]);let eq41_e1246_d_b2: f64 = ((nv14 - 0.0) * s.db[955][2]);let eq41_e1246_d_b3: f64 = ((nv14 - 0.0) * s.db[955][3]);let eq41_e1246_d_b4: f64 = ((nv14 - 0.0) * s.db[955][4]);let eq41_e1246_d_b5: f64 = ((nv14 - 0.0) * s.db[955][5]);let eq41_e1246_d_b6: f64 = ((nv14 - 0.0) * s.db[955][6]);let eq41_e1246_d_b7: f64 = ((nv14 - 0.0) * s.db[955][7]);let eq41_e1246_d_b8: f64 = ((nv14 - 0.0) * s.db[955][8]);let eq41_e1246_d_b9: f64 = ((nv14 - 0.0) * s.db[955][9]);let eq41_e1246_d_b10: f64 = ((nv14 - 0.0) * s.db[955][10]);let eq41_e1246_d_b11: f64 = ((nv14 - 0.0) * s.db[955][11]);let eq41_e1247_q: f64 = eq41_e1246;let eq41_reactive_node_derivatives: [f64; 18] = [eq41_e1246_d_n0, eq41_e1246_d_n1, eq41_e1246_d_n2, eq41_e1246_d_n3, eq41_e1246_d_n4, eq41_e1246_d_n5, eq41_e1246_d_n6, eq41_e1246_d_n7, eq41_e1246_d_n8, eq41_e1246_d_n9, eq41_e1246_d_n10, eq41_e1246_d_n11, eq41_e1246_d_n12, eq41_e1246_d_n13, eq41_e1246_d_n14, eq41_e1246_d_n15, eq41_e1246_d_n16, eq41_e1246_d_n17];let eq41_reactive_branch_derivatives: [f64; 12] = [eq41_e1246_d_b0, eq41_e1246_d_b1, eq41_e1246_d_b2, eq41_e1246_d_b3, eq41_e1246_d_b4, eq41_e1246_d_b5, eq41_e1246_d_b6, eq41_e1246_d_b7, eq41_e1246_d_b8, eq41_e1246_d_b9, eq41_e1246_d_b10, eq41_e1246_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(5),
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
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let nv11 = ctx.node_voltage(nodes[11]);let eq58_e1352: f64 = (s.v[767] * (nv4 - 0.0));let eq58_e1352_d_n0: f64 = (s.dn[767][0] * (nv4 - 0.0));let eq58_e1352_d_n1: f64 = (s.dn[767][1] * (nv4 - 0.0));let eq58_e1352_d_n2: f64 = (s.dn[767][2] * (nv4 - 0.0));let eq58_e1352_d_n3: f64 = (s.dn[767][3] * (nv4 - 0.0));let eq58_e1352_d_n4: f64 = ((s.dn[767][4] * (nv4 - 0.0)) + s.v[767]);let eq58_e1352_d_n5: f64 = (s.dn[767][5] * (nv4 - 0.0));let eq58_e1352_d_n6: f64 = (s.dn[767][6] * (nv4 - 0.0));let eq58_e1352_d_n7: f64 = (s.dn[767][7] * (nv4 - 0.0));let eq58_e1352_d_n8: f64 = (s.dn[767][8] * (nv4 - 0.0));let eq58_e1352_d_n9: f64 = (s.dn[767][9] * (nv4 - 0.0));let eq58_e1352_d_n10: f64 = (s.dn[767][10] * (nv4 - 0.0));let eq58_e1352_d_n11: f64 = (s.dn[767][11] * (nv4 - 0.0));let eq58_e1352_d_n12: f64 = (s.dn[767][12] * (nv4 - 0.0));let eq58_e1352_d_n13: f64 = (s.dn[767][13] * (nv4 - 0.0));let eq58_e1352_d_n14: f64 = (s.dn[767][14] * (nv4 - 0.0));let eq58_e1352_d_n15: f64 = (s.dn[767][15] * (nv4 - 0.0));let eq58_e1352_d_n16: f64 = (s.dn[767][16] * (nv4 - 0.0));let eq58_e1352_d_n17: f64 = (s.dn[767][17] * (nv4 - 0.0));let eq58_e1352_d_b0: f64 = (s.db[767][0] * (nv4 - 0.0));let eq58_e1352_d_b1: f64 = (s.db[767][1] * (nv4 - 0.0));let eq58_e1352_d_b2: f64 = (s.db[767][2] * (nv4 - 0.0));let eq58_e1352_d_b3: f64 = (s.db[767][3] * (nv4 - 0.0));let eq58_e1352_d_b4: f64 = (s.db[767][4] * (nv4 - 0.0));let eq58_e1352_d_b5: f64 = (s.db[767][5] * (nv4 - 0.0));let eq58_e1352_d_b6: f64 = (s.db[767][6] * (nv4 - 0.0));let eq58_e1352_d_b7: f64 = (s.db[767][7] * (nv4 - 0.0));let eq58_e1352_d_b8: f64 = (s.db[767][8] * (nv4 - 0.0));let eq58_e1352_d_b9: f64 = (s.db[767][9] * (nv4 - 0.0));let eq58_e1352_d_b10: f64 = (s.db[767][10] * (nv4 - 0.0));let eq58_e1352_d_b11: f64 = (s.db[767][11] * (nv4 - 0.0));let eq58_e1353_q: f64 = eq58_e1352;let eq58_reactive_node_derivatives: [f64; 18] = [eq58_e1352_d_n0, eq58_e1352_d_n1, eq58_e1352_d_n2, eq58_e1352_d_n3, eq58_e1352_d_n4, eq58_e1352_d_n5, eq58_e1352_d_n6, eq58_e1352_d_n7, eq58_e1352_d_n8, eq58_e1352_d_n9, eq58_e1352_d_n10, eq58_e1352_d_n11, eq58_e1352_d_n12, eq58_e1352_d_n13, eq58_e1352_d_n14, eq58_e1352_d_n15, eq58_e1352_d_n16, eq58_e1352_d_n17];let eq58_reactive_branch_derivatives: [f64; 12] = [eq58_e1352_d_b0, eq58_e1352_d_b1, eq58_e1352_d_b2, eq58_e1352_d_b3, eq58_e1352_d_b4, eq58_e1352_d_b5, eq58_e1352_d_b6, eq58_e1352_d_b7, eq58_e1352_d_b8, eq58_e1352_d_b9, eq58_e1352_d_b10, eq58_e1352_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(4),
            None,
            &eq58_reactive_node_derivatives,
            &eq58_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1368, eq61_e1368_d_n0, eq61_e1368_d_n1, eq61_e1368_d_n2, eq61_e1368_d_n3, eq61_e1368_d_n4, eq61_e1368_d_n5, eq61_e1368_d_n6, eq61_e1368_d_n7, eq61_e1368_d_n8, eq61_e1368_d_n9, eq61_e1368_d_n10, eq61_e1368_d_n11, eq61_e1368_d_n12, eq61_e1368_d_n13, eq61_e1368_d_n14, eq61_e1368_d_n15, eq61_e1368_d_n16, eq61_e1368_d_n17, eq61_e1368_d_b0, eq61_e1368_d_b1, eq61_e1368_d_b2, eq61_e1368_d_b3, eq61_e1368_d_b4, eq61_e1368_d_b5, eq61_e1368_d_b6, eq61_e1368_d_b7, eq61_e1368_d_b8, eq61_e1368_d_b9, eq61_e1368_d_b10, eq61_e1368_d_b11, eq61_e1368_q,) = {
    if (p.p28 != 0.0) {
        let eq61_e1365: f64 = (s.v[800] * (nv11 - 0.0));let eq61_e1365_d_n0: f64 = (s.dn[800][0] * (nv11 - 0.0));let eq61_e1365_d_n1: f64 = (s.dn[800][1] * (nv11 - 0.0));let eq61_e1365_d_n2: f64 = (s.dn[800][2] * (nv11 - 0.0));let eq61_e1365_d_n3: f64 = (s.dn[800][3] * (nv11 - 0.0));let eq61_e1365_d_n4: f64 = (s.dn[800][4] * (nv11 - 0.0));let eq61_e1365_d_n5: f64 = (s.dn[800][5] * (nv11 - 0.0));let eq61_e1365_d_n6: f64 = (s.dn[800][6] * (nv11 - 0.0));let eq61_e1365_d_n7: f64 = (s.dn[800][7] * (nv11 - 0.0));let eq61_e1365_d_n8: f64 = (s.dn[800][8] * (nv11 - 0.0));let eq61_e1365_d_n9: f64 = (s.dn[800][9] * (nv11 - 0.0));let eq61_e1365_d_n10: f64 = (s.dn[800][10] * (nv11 - 0.0));let eq61_e1365_d_n11: f64 = ((s.dn[800][11] * (nv11 - 0.0)) + s.v[800]);let eq61_e1365_d_n12: f64 = (s.dn[800][12] * (nv11 - 0.0));let eq61_e1365_d_n13: f64 = (s.dn[800][13] * (nv11 - 0.0));let eq61_e1365_d_n14: f64 = (s.dn[800][14] * (nv11 - 0.0));let eq61_e1365_d_n15: f64 = (s.dn[800][15] * (nv11 - 0.0));let eq61_e1365_d_n16: f64 = (s.dn[800][16] * (nv11 - 0.0));let eq61_e1365_d_n17: f64 = (s.dn[800][17] * (nv11 - 0.0));let eq61_e1365_d_b0: f64 = (s.db[800][0] * (nv11 - 0.0));let eq61_e1365_d_b1: f64 = (s.db[800][1] * (nv11 - 0.0));let eq61_e1365_d_b2: f64 = (s.db[800][2] * (nv11 - 0.0));let eq61_e1365_d_b3: f64 = (s.db[800][3] * (nv11 - 0.0));let eq61_e1365_d_b4: f64 = (s.db[800][4] * (nv11 - 0.0));let eq61_e1365_d_b5: f64 = (s.db[800][5] * (nv11 - 0.0));let eq61_e1365_d_b6: f64 = (s.db[800][6] * (nv11 - 0.0));let eq61_e1365_d_b7: f64 = (s.db[800][7] * (nv11 - 0.0));let eq61_e1365_d_b8: f64 = (s.db[800][8] * (nv11 - 0.0));let eq61_e1365_d_b9: f64 = (s.db[800][9] * (nv11 - 0.0));let eq61_e1365_d_b10: f64 = (s.db[800][10] * (nv11 - 0.0));let eq61_e1365_d_b11: f64 = (s.db[800][11] * (nv11 - 0.0));let eq61_e1366_q: f64 = eq61_e1365;
        (eq61_e1365, eq61_e1365_d_n0, eq61_e1365_d_n1, eq61_e1365_d_n2, eq61_e1365_d_n3, eq61_e1365_d_n4, eq61_e1365_d_n5, eq61_e1365_d_n6, eq61_e1365_d_n7, eq61_e1365_d_n8, eq61_e1365_d_n9, eq61_e1365_d_n10, eq61_e1365_d_n11, eq61_e1365_d_n12, eq61_e1365_d_n13, eq61_e1365_d_n14, eq61_e1365_d_n15, eq61_e1365_d_n16, eq61_e1365_d_n17, eq61_e1365_d_b0, eq61_e1365_d_b1, eq61_e1365_d_b2, eq61_e1365_d_b3, eq61_e1365_d_b4, eq61_e1365_d_b5, eq61_e1365_d_b6, eq61_e1365_d_b7, eq61_e1365_d_b8, eq61_e1365_d_b9, eq61_e1365_d_b10, eq61_e1365_d_b11, eq61_e1366_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 18] = [eq61_e1368_d_n0, eq61_e1368_d_n1, eq61_e1368_d_n2, eq61_e1368_d_n3, eq61_e1368_d_n4, eq61_e1368_d_n5, eq61_e1368_d_n6, eq61_e1368_d_n7, eq61_e1368_d_n8, eq61_e1368_d_n9, eq61_e1368_d_n10, eq61_e1368_d_n11, eq61_e1368_d_n12, eq61_e1368_d_n13, eq61_e1368_d_n14, eq61_e1368_d_n15, eq61_e1368_d_n16, eq61_e1368_d_n17];let eq61_reactive_branch_derivatives: [f64; 12] = [eq61_e1368_d_b0, eq61_e1368_d_b1, eq61_e1368_d_b2, eq61_e1368_d_b3, eq61_e1368_d_b4, eq61_e1368_d_b5, eq61_e1368_d_b6, eq61_e1368_d_b7, eq61_e1368_d_b8, eq61_e1368_d_b9, eq61_e1368_d_b10, eq61_e1368_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            None,
            &eq61_reactive_node_derivatives,
            &eq61_reactive_branch_derivatives,
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
        let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);
        let (eq62_e1375, eq62_e1375_d_n0, eq62_e1375_d_n1, eq62_e1375_d_n2, eq62_e1375_d_n3, eq62_e1375_d_n4, eq62_e1375_d_n5, eq62_e1375_d_n6, eq62_e1375_d_n7, eq62_e1375_d_n8, eq62_e1375_d_n9, eq62_e1375_d_n10, eq62_e1375_d_n11, eq62_e1375_d_n12, eq62_e1375_d_n13, eq62_e1375_d_n14, eq62_e1375_d_n15, eq62_e1375_d_n16, eq62_e1375_d_n17, eq62_e1375_d_b0, eq62_e1375_d_b1, eq62_e1375_d_b2, eq62_e1375_d_b3, eq62_e1375_d_b4, eq62_e1375_d_b5, eq62_e1375_d_b6, eq62_e1375_d_b7, eq62_e1375_d_b8, eq62_e1375_d_b9, eq62_e1375_d_b10, eq62_e1375_d_b11, eq62_e1375_q,) = {
    if (p.p28 != 0.0) {
        let eq62_e1372: f64 = (s.v[801] * (nv12 - 0.0));let eq62_e1372_d_n0: f64 = (s.dn[801][0] * (nv12 - 0.0));let eq62_e1372_d_n1: f64 = (s.dn[801][1] * (nv12 - 0.0));let eq62_e1372_d_n2: f64 = (s.dn[801][2] * (nv12 - 0.0));let eq62_e1372_d_n3: f64 = (s.dn[801][3] * (nv12 - 0.0));let eq62_e1372_d_n4: f64 = (s.dn[801][4] * (nv12 - 0.0));let eq62_e1372_d_n5: f64 = (s.dn[801][5] * (nv12 - 0.0));let eq62_e1372_d_n6: f64 = (s.dn[801][6] * (nv12 - 0.0));let eq62_e1372_d_n7: f64 = (s.dn[801][7] * (nv12 - 0.0));let eq62_e1372_d_n8: f64 = (s.dn[801][8] * (nv12 - 0.0));let eq62_e1372_d_n9: f64 = (s.dn[801][9] * (nv12 - 0.0));let eq62_e1372_d_n10: f64 = (s.dn[801][10] * (nv12 - 0.0));let eq62_e1372_d_n11: f64 = (s.dn[801][11] * (nv12 - 0.0));let eq62_e1372_d_n12: f64 = ((s.dn[801][12] * (nv12 - 0.0)) + s.v[801]);let eq62_e1372_d_n13: f64 = (s.dn[801][13] * (nv12 - 0.0));let eq62_e1372_d_n14: f64 = (s.dn[801][14] * (nv12 - 0.0));let eq62_e1372_d_n15: f64 = (s.dn[801][15] * (nv12 - 0.0));let eq62_e1372_d_n16: f64 = (s.dn[801][16] * (nv12 - 0.0));let eq62_e1372_d_n17: f64 = (s.dn[801][17] * (nv12 - 0.0));let eq62_e1372_d_b0: f64 = (s.db[801][0] * (nv12 - 0.0));let eq62_e1372_d_b1: f64 = (s.db[801][1] * (nv12 - 0.0));let eq62_e1372_d_b2: f64 = (s.db[801][2] * (nv12 - 0.0));let eq62_e1372_d_b3: f64 = (s.db[801][3] * (nv12 - 0.0));let eq62_e1372_d_b4: f64 = (s.db[801][4] * (nv12 - 0.0));let eq62_e1372_d_b5: f64 = (s.db[801][5] * (nv12 - 0.0));let eq62_e1372_d_b6: f64 = (s.db[801][6] * (nv12 - 0.0));let eq62_e1372_d_b7: f64 = (s.db[801][7] * (nv12 - 0.0));let eq62_e1372_d_b8: f64 = (s.db[801][8] * (nv12 - 0.0));let eq62_e1372_d_b9: f64 = (s.db[801][9] * (nv12 - 0.0));let eq62_e1372_d_b10: f64 = (s.db[801][10] * (nv12 - 0.0));let eq62_e1372_d_b11: f64 = (s.db[801][11] * (nv12 - 0.0));let eq62_e1373_q: f64 = eq62_e1372;
        (eq62_e1372, eq62_e1372_d_n0, eq62_e1372_d_n1, eq62_e1372_d_n2, eq62_e1372_d_n3, eq62_e1372_d_n4, eq62_e1372_d_n5, eq62_e1372_d_n6, eq62_e1372_d_n7, eq62_e1372_d_n8, eq62_e1372_d_n9, eq62_e1372_d_n10, eq62_e1372_d_n11, eq62_e1372_d_n12, eq62_e1372_d_n13, eq62_e1372_d_n14, eq62_e1372_d_n15, eq62_e1372_d_n16, eq62_e1372_d_n17, eq62_e1372_d_b0, eq62_e1372_d_b1, eq62_e1372_d_b2, eq62_e1372_d_b3, eq62_e1372_d_b4, eq62_e1372_d_b5, eq62_e1372_d_b6, eq62_e1372_d_b7, eq62_e1372_d_b8, eq62_e1372_d_b9, eq62_e1372_d_b10, eq62_e1372_d_b11, eq62_e1373_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_reactive_node_derivatives: [f64; 18] = [eq62_e1375_d_n0, eq62_e1375_d_n1, eq62_e1375_d_n2, eq62_e1375_d_n3, eq62_e1375_d_n4, eq62_e1375_d_n5, eq62_e1375_d_n6, eq62_e1375_d_n7, eq62_e1375_d_n8, eq62_e1375_d_n9, eq62_e1375_d_n10, eq62_e1375_d_n11, eq62_e1375_d_n12, eq62_e1375_d_n13, eq62_e1375_d_n14, eq62_e1375_d_n15, eq62_e1375_d_n16, eq62_e1375_d_n17];let eq62_reactive_branch_derivatives: [f64; 12] = [eq62_e1375_d_b0, eq62_e1375_d_b1, eq62_e1375_d_b2, eq62_e1375_d_b3, eq62_e1375_d_b4, eq62_e1375_d_b5, eq62_e1375_d_b6, eq62_e1375_d_b7, eq62_e1375_d_b8, eq62_e1375_d_b9, eq62_e1375_d_b10, eq62_e1375_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(12),
            None,
            &eq62_reactive_node_derivatives,
            &eq62_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1394, eq66_e1394_d_n13, eq66_e1394_q,) = {
    if (p.p29 != 0.0) {
        let eq66_e1392_q: f64 = (nv13 - 0.0);
        ((nv13 - 0.0), 1.0, eq66_e1392_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(13),
            None,
            13,
            multiplicity * (eq66_e1394_d_n13),
        );
    }
}
