#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq31_e1188: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[743]);
        let eq31_e1188_d_n0: f64 = (s.dn[743][0] * ddt_scale);
        let eq31_e1188_d_n1: f64 = (s.dn[743][1] * ddt_scale);
        let eq31_e1188_d_n2: f64 = (s.dn[743][2] * ddt_scale);
        let eq31_e1188_d_n3: f64 = (s.dn[743][3] * ddt_scale);
        let eq31_e1188_d_n4: f64 = (s.dn[743][4] * ddt_scale);
        let eq31_e1188_d_n5: f64 = (s.dn[743][5] * ddt_scale);
        let eq31_e1188_d_n6: f64 = (s.dn[743][6] * ddt_scale);
        let eq31_e1188_d_n7: f64 = (s.dn[743][7] * ddt_scale);
        let eq31_e1188_d_n8: f64 = (s.dn[743][8] * ddt_scale);
        let eq31_e1188_d_n9: f64 = (s.dn[743][9] * ddt_scale);
        let eq31_e1188_d_n10: f64 = (s.dn[743][10] * ddt_scale);
        let eq31_e1188_d_n11: f64 = (s.dn[743][11] * ddt_scale);
        let eq31_e1188_d_n12: f64 = (s.dn[743][12] * ddt_scale);
        let eq31_e1188_d_n13: f64 = (s.dn[743][13] * ddt_scale);
        let eq31_e1188_d_n14: f64 = (s.dn[743][14] * ddt_scale);
        let eq31_e1188_d_n15: f64 = (s.dn[743][15] * ddt_scale);
        let eq31_e1188_d_n16: f64 = (s.dn[743][16] * ddt_scale);
        let eq31_e1188_d_n17: f64 = (s.dn[743][17] * ddt_scale);
        let eq31_e1188_d_n18: f64 = (s.dn[743][18] * ddt_scale);
        let eq31_e1188_d_b0: f64 = (s.db[743][0] * ddt_scale);
        let eq31_e1188_d_b1: f64 = (s.db[743][1] * ddt_scale);
        let eq31_e1188_d_b2: f64 = (s.db[743][2] * ddt_scale);
        let eq31_e1188_d_b3: f64 = (s.db[743][3] * ddt_scale);
        let eq31_e1188_d_b4: f64 = (s.db[743][4] * ddt_scale);
        let eq31_e1188_d_b5: f64 = (s.db[743][5] * ddt_scale);
        let eq31_e1188_d_b6: f64 = (s.db[743][6] * ddt_scale);
        let eq31_e1188_d_b7: f64 = (s.db[743][7] * ddt_scale);
        let eq31_e1188_d_b8: f64 = (s.db[743][8] * ddt_scale);
        let eq31_e1188_d_b9: f64 = (s.db[743][9] * ddt_scale);
        let eq31_e1188_d_b10: f64 = (s.db[743][10] * ddt_scale);
        let eq31_e1188_d_b11: f64 = (s.db[743][11] * ddt_scale);
        let eq31_e1188_d_b12: f64 = (s.db[743][12] * ddt_scale);
        let eq31_e1189: f64 = (p.p87 * eq31_e1188);
        let eq31_e1189_d_n0: f64 = (p.p87 * eq31_e1188_d_n0);
        let eq31_e1189_d_n1: f64 = (p.p87 * eq31_e1188_d_n1);
        let eq31_e1189_d_n2: f64 = (p.p87 * eq31_e1188_d_n2);
        let eq31_e1189_d_n3: f64 = (p.p87 * eq31_e1188_d_n3);
        let eq31_e1189_d_n4: f64 = (p.p87 * eq31_e1188_d_n4);
        let eq31_e1189_d_n5: f64 = (p.p87 * eq31_e1188_d_n5);
        let eq31_e1189_d_n6: f64 = (p.p87 * eq31_e1188_d_n6);
        let eq31_e1189_d_n7: f64 = (p.p87 * eq31_e1188_d_n7);
        let eq31_e1189_d_n8: f64 = (p.p87 * eq31_e1188_d_n8);
        let eq31_e1189_d_n9: f64 = (p.p87 * eq31_e1188_d_n9);
        let eq31_e1189_d_n10: f64 = (p.p87 * eq31_e1188_d_n10);
        let eq31_e1189_d_n11: f64 = (p.p87 * eq31_e1188_d_n11);
        let eq31_e1189_d_n12: f64 = (p.p87 * eq31_e1188_d_n12);
        let eq31_e1189_d_n13: f64 = (p.p87 * eq31_e1188_d_n13);
        let eq31_e1189_d_n14: f64 = (p.p87 * eq31_e1188_d_n14);
        let eq31_e1189_d_n15: f64 = (p.p87 * eq31_e1188_d_n15);
        let eq31_e1189_d_n16: f64 = (p.p87 * eq31_e1188_d_n16);
        let eq31_e1189_d_n17: f64 = (p.p87 * eq31_e1188_d_n17);
        let eq31_e1189_d_n18: f64 = (p.p87 * eq31_e1188_d_n18);
        let eq31_e1189_d_b0: f64 = (p.p87 * eq31_e1188_d_b0);
        let eq31_e1189_d_b1: f64 = (p.p87 * eq31_e1188_d_b1);
        let eq31_e1189_d_b2: f64 = (p.p87 * eq31_e1188_d_b2);
        let eq31_e1189_d_b3: f64 = (p.p87 * eq31_e1188_d_b3);
        let eq31_e1189_d_b4: f64 = (p.p87 * eq31_e1188_d_b4);
        let eq31_e1189_d_b5: f64 = (p.p87 * eq31_e1188_d_b5);
        let eq31_e1189_d_b6: f64 = (p.p87 * eq31_e1188_d_b6);
        let eq31_e1189_d_b7: f64 = (p.p87 * eq31_e1188_d_b7);
        let eq31_e1189_d_b8: f64 = (p.p87 * eq31_e1188_d_b8);
        let eq31_e1189_d_b9: f64 = (p.p87 * eq31_e1188_d_b9);
        let eq31_e1189_d_b10: f64 = (p.p87 * eq31_e1188_d_b10);
        let eq31_e1189_d_b11: f64 = (p.p87 * eq31_e1188_d_b11);
        let eq31_e1189_d_b12: f64 = (p.p87 * eq31_e1188_d_b12);
        let eq31_value: f64 = eq31_e1189;
        let eq31_node_derivatives: [f64; 19] = [eq31_e1189_d_n0, eq31_e1189_d_n1, eq31_e1189_d_n2, eq31_e1189_d_n3, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, eq31_e1189_d_n12, eq31_e1189_d_n13, eq31_e1189_d_n14, eq31_e1189_d_n15, eq31_e1189_d_n16, eq31_e1189_d_n17, eq31_e1189_d_n18];
        let eq31_branch_derivatives: [f64; 13] = [eq31_e1189_d_b0, eq31_e1189_d_b1, eq31_e1189_d_b2, eq31_e1189_d_b3, eq31_e1189_d_b4, eq31_e1189_d_b5, eq31_e1189_d_b6, eq31_e1189_d_b7, eq31_e1189_d_b8, eq31_e1189_d_b9, eq31_e1189_d_b10, eq31_e1189_d_b11, eq31_e1189_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, s.v[742]);
        let eq32_e1192_d_n0: f64 = (s.dn[742][0] * ddt_scale);
        let eq32_e1192_d_n1: f64 = (s.dn[742][1] * ddt_scale);
        let eq32_e1192_d_n2: f64 = (s.dn[742][2] * ddt_scale);
        let eq32_e1192_d_n3: f64 = (s.dn[742][3] * ddt_scale);
        let eq32_e1192_d_n4: f64 = (s.dn[742][4] * ddt_scale);
        let eq32_e1192_d_n5: f64 = (s.dn[742][5] * ddt_scale);
        let eq32_e1192_d_n6: f64 = (s.dn[742][6] * ddt_scale);
        let eq32_e1192_d_n7: f64 = (s.dn[742][7] * ddt_scale);
        let eq32_e1192_d_n8: f64 = (s.dn[742][8] * ddt_scale);
        let eq32_e1192_d_n9: f64 = (s.dn[742][9] * ddt_scale);
        let eq32_e1192_d_n10: f64 = (s.dn[742][10] * ddt_scale);
        let eq32_e1192_d_n11: f64 = (s.dn[742][11] * ddt_scale);
        let eq32_e1192_d_n12: f64 = (s.dn[742][12] * ddt_scale);
        let eq32_e1192_d_n13: f64 = (s.dn[742][13] * ddt_scale);
        let eq32_e1192_d_n14: f64 = (s.dn[742][14] * ddt_scale);
        let eq32_e1192_d_n15: f64 = (s.dn[742][15] * ddt_scale);
        let eq32_e1192_d_n16: f64 = (s.dn[742][16] * ddt_scale);
        let eq32_e1192_d_n17: f64 = (s.dn[742][17] * ddt_scale);
        let eq32_e1192_d_n18: f64 = (s.dn[742][18] * ddt_scale);
        let eq32_e1192_d_b0: f64 = (s.db[742][0] * ddt_scale);
        let eq32_e1192_d_b1: f64 = (s.db[742][1] * ddt_scale);
        let eq32_e1192_d_b2: f64 = (s.db[742][2] * ddt_scale);
        let eq32_e1192_d_b3: f64 = (s.db[742][3] * ddt_scale);
        let eq32_e1192_d_b4: f64 = (s.db[742][4] * ddt_scale);
        let eq32_e1192_d_b5: f64 = (s.db[742][5] * ddt_scale);
        let eq32_e1192_d_b6: f64 = (s.db[742][6] * ddt_scale);
        let eq32_e1192_d_b7: f64 = (s.db[742][7] * ddt_scale);
        let eq32_e1192_d_b8: f64 = (s.db[742][8] * ddt_scale);
        let eq32_e1192_d_b9: f64 = (s.db[742][9] * ddt_scale);
        let eq32_e1192_d_b10: f64 = (s.db[742][10] * ddt_scale);
        let eq32_e1192_d_b11: f64 = (s.db[742][11] * ddt_scale);
        let eq32_e1192_d_b12: f64 = (s.db[742][12] * ddt_scale);
        let eq32_e1193: f64 = (p.p87 * eq32_e1192);
        let eq32_e1193_d_n0: f64 = (p.p87 * eq32_e1192_d_n0);
        let eq32_e1193_d_n1: f64 = (p.p87 * eq32_e1192_d_n1);
        let eq32_e1193_d_n2: f64 = (p.p87 * eq32_e1192_d_n2);
        let eq32_e1193_d_n3: f64 = (p.p87 * eq32_e1192_d_n3);
        let eq32_e1193_d_n4: f64 = (p.p87 * eq32_e1192_d_n4);
        let eq32_e1193_d_n5: f64 = (p.p87 * eq32_e1192_d_n5);
        let eq32_e1193_d_n6: f64 = (p.p87 * eq32_e1192_d_n6);
        let eq32_e1193_d_n7: f64 = (p.p87 * eq32_e1192_d_n7);
        let eq32_e1193_d_n8: f64 = (p.p87 * eq32_e1192_d_n8);
        let eq32_e1193_d_n9: f64 = (p.p87 * eq32_e1192_d_n9);
        let eq32_e1193_d_n10: f64 = (p.p87 * eq32_e1192_d_n10);
        let eq32_e1193_d_n11: f64 = (p.p87 * eq32_e1192_d_n11);
        let eq32_e1193_d_n12: f64 = (p.p87 * eq32_e1192_d_n12);
        let eq32_e1193_d_n13: f64 = (p.p87 * eq32_e1192_d_n13);
        let eq32_e1193_d_n14: f64 = (p.p87 * eq32_e1192_d_n14);
        let eq32_e1193_d_n15: f64 = (p.p87 * eq32_e1192_d_n15);
        let eq32_e1193_d_n16: f64 = (p.p87 * eq32_e1192_d_n16);
        let eq32_e1193_d_n17: f64 = (p.p87 * eq32_e1192_d_n17);
        let eq32_e1193_d_n18: f64 = (p.p87 * eq32_e1192_d_n18);
        let eq32_e1193_d_b0: f64 = (p.p87 * eq32_e1192_d_b0);
        let eq32_e1193_d_b1: f64 = (p.p87 * eq32_e1192_d_b1);
        let eq32_e1193_d_b2: f64 = (p.p87 * eq32_e1192_d_b2);
        let eq32_e1193_d_b3: f64 = (p.p87 * eq32_e1192_d_b3);
        let eq32_e1193_d_b4: f64 = (p.p87 * eq32_e1192_d_b4);
        let eq32_e1193_d_b5: f64 = (p.p87 * eq32_e1192_d_b5);
        let eq32_e1193_d_b6: f64 = (p.p87 * eq32_e1192_d_b6);
        let eq32_e1193_d_b7: f64 = (p.p87 * eq32_e1192_d_b7);
        let eq32_e1193_d_b8: f64 = (p.p87 * eq32_e1192_d_b8);
        let eq32_e1193_d_b9: f64 = (p.p87 * eq32_e1192_d_b9);
        let eq32_e1193_d_b10: f64 = (p.p87 * eq32_e1192_d_b10);
        let eq32_e1193_d_b11: f64 = (p.p87 * eq32_e1192_d_b11);
        let eq32_e1193_d_b12: f64 = (p.p87 * eq32_e1192_d_b12);
        let eq32_value: f64 = eq32_e1193;
        let eq32_node_derivatives: [f64; 19] = [eq32_e1193_d_n0, eq32_e1193_d_n1, eq32_e1193_d_n2, eq32_e1193_d_n3, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, eq32_e1193_d_n12, eq32_e1193_d_n13, eq32_e1193_d_n14, eq32_e1193_d_n15, eq32_e1193_d_n16, eq32_e1193_d_n17, eq32_e1193_d_n18];
        let eq32_branch_derivatives: [f64; 13] = [eq32_e1193_d_b0, eq32_e1193_d_b1, eq32_e1193_d_b2, eq32_e1193_d_b3, eq32_e1193_d_b4, eq32_e1193_d_b5, eq32_e1193_d_b6, eq32_e1193_d_b7, eq32_e1193_d_b8, eq32_e1193_d_b9, eq32_e1193_d_b10, eq32_e1193_d_b11, eq32_e1193_d_b12];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e1196: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[744]);
        let eq33_e1196_d_n0: f64 = (s.dn[744][0] * ddt_scale);
        let eq33_e1196_d_n1: f64 = (s.dn[744][1] * ddt_scale);
        let eq33_e1196_d_n2: f64 = (s.dn[744][2] * ddt_scale);
        let eq33_e1196_d_n3: f64 = (s.dn[744][3] * ddt_scale);
        let eq33_e1196_d_n4: f64 = (s.dn[744][4] * ddt_scale);
        let eq33_e1196_d_n5: f64 = (s.dn[744][5] * ddt_scale);
        let eq33_e1196_d_n6: f64 = (s.dn[744][6] * ddt_scale);
        let eq33_e1196_d_n7: f64 = (s.dn[744][7] * ddt_scale);
        let eq33_e1196_d_n8: f64 = (s.dn[744][8] * ddt_scale);
        let eq33_e1196_d_n9: f64 = (s.dn[744][9] * ddt_scale);
        let eq33_e1196_d_n10: f64 = (s.dn[744][10] * ddt_scale);
        let eq33_e1196_d_n11: f64 = (s.dn[744][11] * ddt_scale);
        let eq33_e1196_d_n12: f64 = (s.dn[744][12] * ddt_scale);
        let eq33_e1196_d_n13: f64 = (s.dn[744][13] * ddt_scale);
        let eq33_e1196_d_n14: f64 = (s.dn[744][14] * ddt_scale);
        let eq33_e1196_d_n15: f64 = (s.dn[744][15] * ddt_scale);
        let eq33_e1196_d_n16: f64 = (s.dn[744][16] * ddt_scale);
        let eq33_e1196_d_n17: f64 = (s.dn[744][17] * ddt_scale);
        let eq33_e1196_d_n18: f64 = (s.dn[744][18] * ddt_scale);
        let eq33_e1196_d_b0: f64 = (s.db[744][0] * ddt_scale);
        let eq33_e1196_d_b1: f64 = (s.db[744][1] * ddt_scale);
        let eq33_e1196_d_b2: f64 = (s.db[744][2] * ddt_scale);
        let eq33_e1196_d_b3: f64 = (s.db[744][3] * ddt_scale);
        let eq33_e1196_d_b4: f64 = (s.db[744][4] * ddt_scale);
        let eq33_e1196_d_b5: f64 = (s.db[744][5] * ddt_scale);
        let eq33_e1196_d_b6: f64 = (s.db[744][6] * ddt_scale);
        let eq33_e1196_d_b7: f64 = (s.db[744][7] * ddt_scale);
        let eq33_e1196_d_b8: f64 = (s.db[744][8] * ddt_scale);
        let eq33_e1196_d_b9: f64 = (s.db[744][9] * ddt_scale);
        let eq33_e1196_d_b10: f64 = (s.db[744][10] * ddt_scale);
        let eq33_e1196_d_b11: f64 = (s.db[744][11] * ddt_scale);
        let eq33_e1196_d_b12: f64 = (s.db[744][12] * ddt_scale);
        let eq33_e1197: f64 = (p.p87 * eq33_e1196);
        let eq33_e1197_d_n0: f64 = (p.p87 * eq33_e1196_d_n0);
        let eq33_e1197_d_n1: f64 = (p.p87 * eq33_e1196_d_n1);
        let eq33_e1197_d_n2: f64 = (p.p87 * eq33_e1196_d_n2);
        let eq33_e1197_d_n3: f64 = (p.p87 * eq33_e1196_d_n3);
        let eq33_e1197_d_n4: f64 = (p.p87 * eq33_e1196_d_n4);
        let eq33_e1197_d_n5: f64 = (p.p87 * eq33_e1196_d_n5);
        let eq33_e1197_d_n6: f64 = (p.p87 * eq33_e1196_d_n6);
        let eq33_e1197_d_n7: f64 = (p.p87 * eq33_e1196_d_n7);
        let eq33_e1197_d_n8: f64 = (p.p87 * eq33_e1196_d_n8);
        let eq33_e1197_d_n9: f64 = (p.p87 * eq33_e1196_d_n9);
        let eq33_e1197_d_n10: f64 = (p.p87 * eq33_e1196_d_n10);
        let eq33_e1197_d_n11: f64 = (p.p87 * eq33_e1196_d_n11);
        let eq33_e1197_d_n12: f64 = (p.p87 * eq33_e1196_d_n12);
        let eq33_e1197_d_n13: f64 = (p.p87 * eq33_e1196_d_n13);
        let eq33_e1197_d_n14: f64 = (p.p87 * eq33_e1196_d_n14);
        let eq33_e1197_d_n15: f64 = (p.p87 * eq33_e1196_d_n15);
        let eq33_e1197_d_n16: f64 = (p.p87 * eq33_e1196_d_n16);
        let eq33_e1197_d_n17: f64 = (p.p87 * eq33_e1196_d_n17);
        let eq33_e1197_d_n18: f64 = (p.p87 * eq33_e1196_d_n18);
        let eq33_e1197_d_b0: f64 = (p.p87 * eq33_e1196_d_b0);
        let eq33_e1197_d_b1: f64 = (p.p87 * eq33_e1196_d_b1);
        let eq33_e1197_d_b2: f64 = (p.p87 * eq33_e1196_d_b2);
        let eq33_e1197_d_b3: f64 = (p.p87 * eq33_e1196_d_b3);
        let eq33_e1197_d_b4: f64 = (p.p87 * eq33_e1196_d_b4);
        let eq33_e1197_d_b5: f64 = (p.p87 * eq33_e1196_d_b5);
        let eq33_e1197_d_b6: f64 = (p.p87 * eq33_e1196_d_b6);
        let eq33_e1197_d_b7: f64 = (p.p87 * eq33_e1196_d_b7);
        let eq33_e1197_d_b8: f64 = (p.p87 * eq33_e1196_d_b8);
        let eq33_e1197_d_b9: f64 = (p.p87 * eq33_e1196_d_b9);
        let eq33_e1197_d_b10: f64 = (p.p87 * eq33_e1196_d_b10);
        let eq33_e1197_d_b11: f64 = (p.p87 * eq33_e1196_d_b11);
        let eq33_e1197_d_b12: f64 = (p.p87 * eq33_e1196_d_b12);
        let eq33_value: f64 = eq33_e1197;
        let eq33_node_derivatives: [f64; 19] = [eq33_e1197_d_n0, eq33_e1197_d_n1, eq33_e1197_d_n2, eq33_e1197_d_n3, eq33_e1197_d_n4, eq33_e1197_d_n5, eq33_e1197_d_n6, eq33_e1197_d_n7, eq33_e1197_d_n8, eq33_e1197_d_n9, eq33_e1197_d_n10, eq33_e1197_d_n11, eq33_e1197_d_n12, eq33_e1197_d_n13, eq33_e1197_d_n14, eq33_e1197_d_n15, eq33_e1197_d_n16, eq33_e1197_d_n17, eq33_e1197_d_n18];
        let eq33_branch_derivatives: [f64; 13] = [eq33_e1197_d_b0, eq33_e1197_d_b1, eq33_e1197_d_b2, eq33_e1197_d_b3, eq33_e1197_d_b4, eq33_e1197_d_b5, eq33_e1197_d_b6, eq33_e1197_d_b7, eq33_e1197_d_b8, eq33_e1197_d_b9, eq33_e1197_d_b10, eq33_e1197_d_b11, eq33_e1197_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e1199: f64 = (-p.p87);
        let eq34_e1201: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, s.v[299]);
        let eq34_e1201_d_n0: f64 = (s.dn[299][0] * ddt_scale);
        let eq34_e1201_d_n1: f64 = (s.dn[299][1] * ddt_scale);
        let eq34_e1201_d_n2: f64 = (s.dn[299][2] * ddt_scale);
        let eq34_e1201_d_n3: f64 = (s.dn[299][3] * ddt_scale);
        let eq34_e1201_d_n4: f64 = (s.dn[299][4] * ddt_scale);
        let eq34_e1201_d_n5: f64 = (s.dn[299][5] * ddt_scale);
        let eq34_e1201_d_n6: f64 = (s.dn[299][6] * ddt_scale);
        let eq34_e1201_d_n7: f64 = (s.dn[299][7] * ddt_scale);
        let eq34_e1201_d_n8: f64 = (s.dn[299][8] * ddt_scale);
        let eq34_e1201_d_n9: f64 = (s.dn[299][9] * ddt_scale);
        let eq34_e1201_d_n10: f64 = (s.dn[299][10] * ddt_scale);
        let eq34_e1201_d_n11: f64 = (s.dn[299][11] * ddt_scale);
        let eq34_e1201_d_n12: f64 = (s.dn[299][12] * ddt_scale);
        let eq34_e1201_d_n13: f64 = (s.dn[299][13] * ddt_scale);
        let eq34_e1201_d_n14: f64 = (s.dn[299][14] * ddt_scale);
        let eq34_e1201_d_n15: f64 = (s.dn[299][15] * ddt_scale);
        let eq34_e1201_d_n16: f64 = (s.dn[299][16] * ddt_scale);
        let eq34_e1201_d_n17: f64 = (s.dn[299][17] * ddt_scale);
        let eq34_e1201_d_n18: f64 = (s.dn[299][18] * ddt_scale);
        let eq34_e1201_d_b0: f64 = (s.db[299][0] * ddt_scale);
        let eq34_e1201_d_b1: f64 = (s.db[299][1] * ddt_scale);
        let eq34_e1201_d_b2: f64 = (s.db[299][2] * ddt_scale);
        let eq34_e1201_d_b3: f64 = (s.db[299][3] * ddt_scale);
        let eq34_e1201_d_b4: f64 = (s.db[299][4] * ddt_scale);
        let eq34_e1201_d_b5: f64 = (s.db[299][5] * ddt_scale);
        let eq34_e1201_d_b6: f64 = (s.db[299][6] * ddt_scale);
        let eq34_e1201_d_b7: f64 = (s.db[299][7] * ddt_scale);
        let eq34_e1201_d_b8: f64 = (s.db[299][8] * ddt_scale);
        let eq34_e1201_d_b9: f64 = (s.db[299][9] * ddt_scale);
        let eq34_e1201_d_b10: f64 = (s.db[299][10] * ddt_scale);
        let eq34_e1201_d_b11: f64 = (s.db[299][11] * ddt_scale);
        let eq34_e1201_d_b12: f64 = (s.db[299][12] * ddt_scale);
        let eq34_e1202: f64 = (eq34_e1199 * eq34_e1201);
        let eq34_e1202_d_n0: f64 = (eq34_e1199 * eq34_e1201_d_n0);
        let eq34_e1202_d_n1: f64 = (eq34_e1199 * eq34_e1201_d_n1);
        let eq34_e1202_d_n2: f64 = (eq34_e1199 * eq34_e1201_d_n2);
        let eq34_e1202_d_n3: f64 = (eq34_e1199 * eq34_e1201_d_n3);
        let eq34_e1202_d_n4: f64 = (eq34_e1199 * eq34_e1201_d_n4);
        let eq34_e1202_d_n5: f64 = (eq34_e1199 * eq34_e1201_d_n5);
        let eq34_e1202_d_n6: f64 = (eq34_e1199 * eq34_e1201_d_n6);
        let eq34_e1202_d_n7: f64 = (eq34_e1199 * eq34_e1201_d_n7);
        let eq34_e1202_d_n8: f64 = (eq34_e1199 * eq34_e1201_d_n8);
        let eq34_e1202_d_n9: f64 = (eq34_e1199 * eq34_e1201_d_n9);
        let eq34_e1202_d_n10: f64 = (eq34_e1199 * eq34_e1201_d_n10);
        let eq34_e1202_d_n11: f64 = (eq34_e1199 * eq34_e1201_d_n11);
        let eq34_e1202_d_n12: f64 = (eq34_e1199 * eq34_e1201_d_n12);
        let eq34_e1202_d_n13: f64 = (eq34_e1199 * eq34_e1201_d_n13);
        let eq34_e1202_d_n14: f64 = (eq34_e1199 * eq34_e1201_d_n14);
        let eq34_e1202_d_n15: f64 = (eq34_e1199 * eq34_e1201_d_n15);
        let eq34_e1202_d_n16: f64 = (eq34_e1199 * eq34_e1201_d_n16);
        let eq34_e1202_d_n17: f64 = (eq34_e1199 * eq34_e1201_d_n17);
        let eq34_e1202_d_n18: f64 = (eq34_e1199 * eq34_e1201_d_n18);
        let eq34_e1202_d_b0: f64 = (eq34_e1199 * eq34_e1201_d_b0);
        let eq34_e1202_d_b1: f64 = (eq34_e1199 * eq34_e1201_d_b1);
        let eq34_e1202_d_b2: f64 = (eq34_e1199 * eq34_e1201_d_b2);
        let eq34_e1202_d_b3: f64 = (eq34_e1199 * eq34_e1201_d_b3);
        let eq34_e1202_d_b4: f64 = (eq34_e1199 * eq34_e1201_d_b4);
        let eq34_e1202_d_b5: f64 = (eq34_e1199 * eq34_e1201_d_b5);
        let eq34_e1202_d_b6: f64 = (eq34_e1199 * eq34_e1201_d_b6);
        let eq34_e1202_d_b7: f64 = (eq34_e1199 * eq34_e1201_d_b7);
        let eq34_e1202_d_b8: f64 = (eq34_e1199 * eq34_e1201_d_b8);
        let eq34_e1202_d_b9: f64 = (eq34_e1199 * eq34_e1201_d_b9);
        let eq34_e1202_d_b10: f64 = (eq34_e1199 * eq34_e1201_d_b10);
        let eq34_e1202_d_b11: f64 = (eq34_e1199 * eq34_e1201_d_b11);
        let eq34_e1202_d_b12: f64 = (eq34_e1199 * eq34_e1201_d_b12);
        let eq34_value: f64 = eq34_e1202;
        let eq34_node_derivatives: [f64; 19] = [eq34_e1202_d_n0, eq34_e1202_d_n1, eq34_e1202_d_n2, eq34_e1202_d_n3, eq34_e1202_d_n4, eq34_e1202_d_n5, eq34_e1202_d_n6, eq34_e1202_d_n7, eq34_e1202_d_n8, eq34_e1202_d_n9, eq34_e1202_d_n10, eq34_e1202_d_n11, eq34_e1202_d_n12, eq34_e1202_d_n13, eq34_e1202_d_n14, eq34_e1202_d_n15, eq34_e1202_d_n16, eq34_e1202_d_n17, eq34_e1202_d_n18];
        let eq34_branch_derivatives: [f64; 13] = [eq34_e1202_d_b0, eq34_e1202_d_b1, eq34_e1202_d_b2, eq34_e1202_d_b3, eq34_e1202_d_b4, eq34_e1202_d_b5, eq34_e1202_d_b6, eq34_e1202_d_b7, eq34_e1202_d_b8, eq34_e1202_d_b9, eq34_e1202_d_b10, eq34_e1202_d_b11, eq34_e1202_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(0),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let eq35_e1204: f64 = (-p.p87);
        let eq35_e1206: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, s.v[301]);
        let eq35_e1206_d_n0: f64 = (s.dn[301][0] * ddt_scale);
        let eq35_e1206_d_n1: f64 = (s.dn[301][1] * ddt_scale);
        let eq35_e1206_d_n2: f64 = (s.dn[301][2] * ddt_scale);
        let eq35_e1206_d_n3: f64 = (s.dn[301][3] * ddt_scale);
        let eq35_e1206_d_n4: f64 = (s.dn[301][4] * ddt_scale);
        let eq35_e1206_d_n5: f64 = (s.dn[301][5] * ddt_scale);
        let eq35_e1206_d_n6: f64 = (s.dn[301][6] * ddt_scale);
        let eq35_e1206_d_n7: f64 = (s.dn[301][7] * ddt_scale);
        let eq35_e1206_d_n8: f64 = (s.dn[301][8] * ddt_scale);
        let eq35_e1206_d_n9: f64 = (s.dn[301][9] * ddt_scale);
        let eq35_e1206_d_n10: f64 = (s.dn[301][10] * ddt_scale);
        let eq35_e1206_d_n11: f64 = (s.dn[301][11] * ddt_scale);
        let eq35_e1206_d_n12: f64 = (s.dn[301][12] * ddt_scale);
        let eq35_e1206_d_n13: f64 = (s.dn[301][13] * ddt_scale);
        let eq35_e1206_d_n14: f64 = (s.dn[301][14] * ddt_scale);
        let eq35_e1206_d_n15: f64 = (s.dn[301][15] * ddt_scale);
        let eq35_e1206_d_n16: f64 = (s.dn[301][16] * ddt_scale);
        let eq35_e1206_d_n17: f64 = (s.dn[301][17] * ddt_scale);
        let eq35_e1206_d_n18: f64 = (s.dn[301][18] * ddt_scale);
        let eq35_e1206_d_b0: f64 = (s.db[301][0] * ddt_scale);
        let eq35_e1206_d_b1: f64 = (s.db[301][1] * ddt_scale);
        let eq35_e1206_d_b2: f64 = (s.db[301][2] * ddt_scale);
        let eq35_e1206_d_b3: f64 = (s.db[301][3] * ddt_scale);
        let eq35_e1206_d_b4: f64 = (s.db[301][4] * ddt_scale);
        let eq35_e1206_d_b5: f64 = (s.db[301][5] * ddt_scale);
        let eq35_e1206_d_b6: f64 = (s.db[301][6] * ddt_scale);
        let eq35_e1206_d_b7: f64 = (s.db[301][7] * ddt_scale);
        let eq35_e1206_d_b8: f64 = (s.db[301][8] * ddt_scale);
        let eq35_e1206_d_b9: f64 = (s.db[301][9] * ddt_scale);
        let eq35_e1206_d_b10: f64 = (s.db[301][10] * ddt_scale);
        let eq35_e1206_d_b11: f64 = (s.db[301][11] * ddt_scale);
        let eq35_e1206_d_b12: f64 = (s.db[301][12] * ddt_scale);
        let eq35_e1207: f64 = (eq35_e1204 * eq35_e1206);
        let eq35_e1207_d_n0: f64 = (eq35_e1204 * eq35_e1206_d_n0);
        let eq35_e1207_d_n1: f64 = (eq35_e1204 * eq35_e1206_d_n1);
        let eq35_e1207_d_n2: f64 = (eq35_e1204 * eq35_e1206_d_n2);
        let eq35_e1207_d_n3: f64 = (eq35_e1204 * eq35_e1206_d_n3);
        let eq35_e1207_d_n4: f64 = (eq35_e1204 * eq35_e1206_d_n4);
        let eq35_e1207_d_n5: f64 = (eq35_e1204 * eq35_e1206_d_n5);
        let eq35_e1207_d_n6: f64 = (eq35_e1204 * eq35_e1206_d_n6);
        let eq35_e1207_d_n7: f64 = (eq35_e1204 * eq35_e1206_d_n7);
        let eq35_e1207_d_n8: f64 = (eq35_e1204 * eq35_e1206_d_n8);
        let eq35_e1207_d_n9: f64 = (eq35_e1204 * eq35_e1206_d_n9);
        let eq35_e1207_d_n10: f64 = (eq35_e1204 * eq35_e1206_d_n10);
        let eq35_e1207_d_n11: f64 = (eq35_e1204 * eq35_e1206_d_n11);
        let eq35_e1207_d_n12: f64 = (eq35_e1204 * eq35_e1206_d_n12);
        let eq35_e1207_d_n13: f64 = (eq35_e1204 * eq35_e1206_d_n13);
        let eq35_e1207_d_n14: f64 = (eq35_e1204 * eq35_e1206_d_n14);
        let eq35_e1207_d_n15: f64 = (eq35_e1204 * eq35_e1206_d_n15);
        let eq35_e1207_d_n16: f64 = (eq35_e1204 * eq35_e1206_d_n16);
        let eq35_e1207_d_n17: f64 = (eq35_e1204 * eq35_e1206_d_n17);
        let eq35_e1207_d_n18: f64 = (eq35_e1204 * eq35_e1206_d_n18);
        let eq35_e1207_d_b0: f64 = (eq35_e1204 * eq35_e1206_d_b0);
        let eq35_e1207_d_b1: f64 = (eq35_e1204 * eq35_e1206_d_b1);
        let eq35_e1207_d_b2: f64 = (eq35_e1204 * eq35_e1206_d_b2);
        let eq35_e1207_d_b3: f64 = (eq35_e1204 * eq35_e1206_d_b3);
        let eq35_e1207_d_b4: f64 = (eq35_e1204 * eq35_e1206_d_b4);
        let eq35_e1207_d_b5: f64 = (eq35_e1204 * eq35_e1206_d_b5);
        let eq35_e1207_d_b6: f64 = (eq35_e1204 * eq35_e1206_d_b6);
        let eq35_e1207_d_b7: f64 = (eq35_e1204 * eq35_e1206_d_b7);
        let eq35_e1207_d_b8: f64 = (eq35_e1204 * eq35_e1206_d_b8);
        let eq35_e1207_d_b9: f64 = (eq35_e1204 * eq35_e1206_d_b9);
        let eq35_e1207_d_b10: f64 = (eq35_e1204 * eq35_e1206_d_b10);
        let eq35_e1207_d_b11: f64 = (eq35_e1204 * eq35_e1206_d_b11);
        let eq35_e1207_d_b12: f64 = (eq35_e1204 * eq35_e1206_d_b12);
        let eq35_value: f64 = eq35_e1207;
        let eq35_node_derivatives: [f64; 19] = [eq35_e1207_d_n0, eq35_e1207_d_n1, eq35_e1207_d_n2, eq35_e1207_d_n3, eq35_e1207_d_n4, eq35_e1207_d_n5, eq35_e1207_d_n6, eq35_e1207_d_n7, eq35_e1207_d_n8, eq35_e1207_d_n9, eq35_e1207_d_n10, eq35_e1207_d_n11, eq35_e1207_d_n12, eq35_e1207_d_n13, eq35_e1207_d_n14, eq35_e1207_d_n15, eq35_e1207_d_n16, eq35_e1207_d_n17, eq35_e1207_d_n18];
        let eq35_branch_derivatives: [f64; 13] = [eq35_e1207_d_b0, eq35_e1207_d_b1, eq35_e1207_d_b2, eq35_e1207_d_b3, eq35_e1207_d_b4, eq35_e1207_d_b5, eq35_e1207_d_b6, eq35_e1207_d_b7, eq35_e1207_d_b8, eq35_e1207_d_b9, eq35_e1207_d_b10, eq35_e1207_d_b11, eq35_e1207_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq40_e1233: f64 = (s.v[951] * (nv15 - 0.0));
        let eq40_e1233_d_n0: f64 = (s.dn[951][0] * (nv15 - 0.0));
        let eq40_e1233_d_n1: f64 = (s.dn[951][1] * (nv15 - 0.0));
        let eq40_e1233_d_n2: f64 = (s.dn[951][2] * (nv15 - 0.0));
        let eq40_e1233_d_n3: f64 = (s.dn[951][3] * (nv15 - 0.0));
        let eq40_e1233_d_n4: f64 = (s.dn[951][4] * (nv15 - 0.0));
        let eq40_e1233_d_n5: f64 = (s.dn[951][5] * (nv15 - 0.0));
        let eq40_e1233_d_n6: f64 = (s.dn[951][6] * (nv15 - 0.0));
        let eq40_e1233_d_n7: f64 = (s.dn[951][7] * (nv15 - 0.0));
        let eq40_e1233_d_n8: f64 = (s.dn[951][8] * (nv15 - 0.0));
        let eq40_e1233_d_n9: f64 = (s.dn[951][9] * (nv15 - 0.0));
        let eq40_e1233_d_n10: f64 = (s.dn[951][10] * (nv15 - 0.0));
        let eq40_e1233_d_n11: f64 = (s.dn[951][11] * (nv15 - 0.0));
        let eq40_e1233_d_n12: f64 = (s.dn[951][12] * (nv15 - 0.0));
        let eq40_e1233_d_n13: f64 = (s.dn[951][13] * (nv15 - 0.0));
        let eq40_e1233_d_n14: f64 = (s.dn[951][14] * (nv15 - 0.0));
        let eq40_e1233_d_n15: f64 = ((s.dn[951][15] * (nv15 - 0.0)) + s.v[951]);
        let eq40_e1233_d_n16: f64 = (s.dn[951][16] * (nv15 - 0.0));
        let eq40_e1233_d_n17: f64 = (s.dn[951][17] * (nv15 - 0.0));
        let eq40_e1233_d_n18: f64 = (s.dn[951][18] * (nv15 - 0.0));
        let eq40_e1233_d_b0: f64 = (s.db[951][0] * (nv15 - 0.0));
        let eq40_e1233_d_b1: f64 = (s.db[951][1] * (nv15 - 0.0));
        let eq40_e1233_d_b2: f64 = (s.db[951][2] * (nv15 - 0.0));
        let eq40_e1233_d_b3: f64 = (s.db[951][3] * (nv15 - 0.0));
        let eq40_e1233_d_b4: f64 = (s.db[951][4] * (nv15 - 0.0));
        let eq40_e1233_d_b5: f64 = (s.db[951][5] * (nv15 - 0.0));
        let eq40_e1233_d_b6: f64 = (s.db[951][6] * (nv15 - 0.0));
        let eq40_e1233_d_b7: f64 = (s.db[951][7] * (nv15 - 0.0));
        let eq40_e1233_d_b8: f64 = (s.db[951][8] * (nv15 - 0.0));
        let eq40_e1233_d_b9: f64 = (s.db[951][9] * (nv15 - 0.0));
        let eq40_e1233_d_b10: f64 = (s.db[951][10] * (nv15 - 0.0));
        let eq40_e1233_d_b11: f64 = (s.db[951][11] * (nv15 - 0.0));
        let eq40_e1233_d_b12: f64 = (s.db[951][12] * (nv15 - 0.0));
        let eq40_value: f64 = eq40_e1233;
        let eq40_node_derivatives: [f64; 19] = [eq40_e1233_d_n0, eq40_e1233_d_n1, eq40_e1233_d_n2, eq40_e1233_d_n3, eq40_e1233_d_n4, eq40_e1233_d_n5, eq40_e1233_d_n6, eq40_e1233_d_n7, eq40_e1233_d_n8, eq40_e1233_d_n9, eq40_e1233_d_n10, eq40_e1233_d_n11, eq40_e1233_d_n12, eq40_e1233_d_n13, eq40_e1233_d_n14, eq40_e1233_d_n15, eq40_e1233_d_n16, eq40_e1233_d_n17, eq40_e1233_d_n18];
        let eq40_branch_derivatives: [f64; 13] = [eq40_e1233_d_b0, eq40_e1233_d_b1, eq40_e1233_d_b2, eq40_e1233_d_b3, eq40_e1233_d_b4, eq40_e1233_d_b5, eq40_e1233_d_b6, eq40_e1233_d_b7, eq40_e1233_d_b8, eq40_e1233_d_b9, eq40_e1233_d_b10, eq40_e1233_d_b11, eq40_e1233_d_b12];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv15 - 0.0) * s.v[954]);
        let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * s.dn[954][0]);
        let eq41_e1236_d_n1: f64 = ((nv15 - 0.0) * s.dn[954][1]);
        let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * s.dn[954][2]);
        let eq41_e1236_d_n3: f64 = ((nv15 - 0.0) * s.dn[954][3]);
        let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * s.dn[954][4]);
        let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * s.dn[954][5]);
        let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * s.dn[954][6]);
        let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * s.dn[954][7]);
        let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * s.dn[954][8]);
        let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * s.dn[954][9]);
        let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * s.dn[954][10]);
        let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * s.dn[954][11]);
        let eq41_e1236_d_n12: f64 = ((nv15 - 0.0) * s.dn[954][12]);
        let eq41_e1236_d_n13: f64 = ((nv15 - 0.0) * s.dn[954][13]);
        let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * s.dn[954][14]);
        let eq41_e1236_d_n15: f64 = (s.v[954] + ((nv15 - 0.0) * s.dn[954][15]));
        let eq41_e1236_d_n16: f64 = ((nv15 - 0.0) * s.dn[954][16]);
        let eq41_e1236_d_n17: f64 = ((nv15 - 0.0) * s.dn[954][17]);
        let eq41_e1236_d_n18: f64 = ((nv15 - 0.0) * s.dn[954][18]);
        let eq41_e1236_d_b0: f64 = ((nv15 - 0.0) * s.db[954][0]);
        let eq41_e1236_d_b1: f64 = ((nv15 - 0.0) * s.db[954][1]);
        let eq41_e1236_d_b2: f64 = ((nv15 - 0.0) * s.db[954][2]);
        let eq41_e1236_d_b3: f64 = ((nv15 - 0.0) * s.db[954][3]);
        let eq41_e1236_d_b4: f64 = ((nv15 - 0.0) * s.db[954][4]);
        let eq41_e1236_d_b5: f64 = ((nv15 - 0.0) * s.db[954][5]);
        let eq41_e1236_d_b6: f64 = ((nv15 - 0.0) * s.db[954][6]);
        let eq41_e1236_d_b7: f64 = ((nv15 - 0.0) * s.db[954][7]);
        let eq41_e1236_d_b8: f64 = ((nv15 - 0.0) * s.db[954][8]);
        let eq41_e1236_d_b9: f64 = ((nv15 - 0.0) * s.db[954][9]);
        let eq41_e1236_d_b10: f64 = ((nv15 - 0.0) * s.db[954][10]);
        let eq41_e1236_d_b11: f64 = ((nv15 - 0.0) * s.db[954][11]);
        let eq41_e1236_d_b12: f64 = ((nv15 - 0.0) * s.db[954][12]);
        let eq41_e1237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq41_e1236);
        let eq41_e1237_d_n0: f64 = (eq41_e1236_d_n0 * ddt_scale);
        let eq41_e1237_d_n1: f64 = (eq41_e1236_d_n1 * ddt_scale);
        let eq41_e1237_d_n2: f64 = (eq41_e1236_d_n2 * ddt_scale);
        let eq41_e1237_d_n3: f64 = (eq41_e1236_d_n3 * ddt_scale);
        let eq41_e1237_d_n4: f64 = (eq41_e1236_d_n4 * ddt_scale);
        let eq41_e1237_d_n5: f64 = (eq41_e1236_d_n5 * ddt_scale);
        let eq41_e1237_d_n6: f64 = (eq41_e1236_d_n6 * ddt_scale);
        let eq41_e1237_d_n7: f64 = (eq41_e1236_d_n7 * ddt_scale);
        let eq41_e1237_d_n8: f64 = (eq41_e1236_d_n8 * ddt_scale);
        let eq41_e1237_d_n9: f64 = (eq41_e1236_d_n9 * ddt_scale);
        let eq41_e1237_d_n10: f64 = (eq41_e1236_d_n10 * ddt_scale);
        let eq41_e1237_d_n11: f64 = (eq41_e1236_d_n11 * ddt_scale);
        let eq41_e1237_d_n12: f64 = (eq41_e1236_d_n12 * ddt_scale);
        let eq41_e1237_d_n13: f64 = (eq41_e1236_d_n13 * ddt_scale);
        let eq41_e1237_d_n14: f64 = (eq41_e1236_d_n14 * ddt_scale);
        let eq41_e1237_d_n15: f64 = (eq41_e1236_d_n15 * ddt_scale);
        let eq41_e1237_d_n16: f64 = (eq41_e1236_d_n16 * ddt_scale);
        let eq41_e1237_d_n17: f64 = (eq41_e1236_d_n17 * ddt_scale);
        let eq41_e1237_d_n18: f64 = (eq41_e1236_d_n18 * ddt_scale);
        let eq41_e1237_d_b0: f64 = (eq41_e1236_d_b0 * ddt_scale);
        let eq41_e1237_d_b1: f64 = (eq41_e1236_d_b1 * ddt_scale);
        let eq41_e1237_d_b2: f64 = (eq41_e1236_d_b2 * ddt_scale);
        let eq41_e1237_d_b3: f64 = (eq41_e1236_d_b3 * ddt_scale);
        let eq41_e1237_d_b4: f64 = (eq41_e1236_d_b4 * ddt_scale);
        let eq41_e1237_d_b5: f64 = (eq41_e1236_d_b5 * ddt_scale);
        let eq41_e1237_d_b6: f64 = (eq41_e1236_d_b6 * ddt_scale);
        let eq41_e1237_d_b7: f64 = (eq41_e1236_d_b7 * ddt_scale);
        let eq41_e1237_d_b8: f64 = (eq41_e1236_d_b8 * ddt_scale);
        let eq41_e1237_d_b9: f64 = (eq41_e1236_d_b9 * ddt_scale);
        let eq41_e1237_d_b10: f64 = (eq41_e1236_d_b10 * ddt_scale);
        let eq41_e1237_d_b11: f64 = (eq41_e1236_d_b11 * ddt_scale);
        let eq41_e1237_d_b12: f64 = (eq41_e1236_d_b12 * ddt_scale);
        let eq41_value: f64 = eq41_e1237;
        let eq41_node_derivatives: [f64; 19] = [eq41_e1237_d_n0, eq41_e1237_d_n1, eq41_e1237_d_n2, eq41_e1237_d_n3, eq41_e1237_d_n4, eq41_e1237_d_n5, eq41_e1237_d_n6, eq41_e1237_d_n7, eq41_e1237_d_n8, eq41_e1237_d_n9, eq41_e1237_d_n10, eq41_e1237_d_n11, eq41_e1237_d_n12, eq41_e1237_d_n13, eq41_e1237_d_n14, eq41_e1237_d_n15, eq41_e1237_d_n16, eq41_e1237_d_n17, eq41_e1237_d_n18];
        let eq41_branch_derivatives: [f64; 13] = [eq41_e1237_d_b0, eq41_e1237_d_b1, eq41_e1237_d_b2, eq41_e1237_d_b3, eq41_e1237_d_b4, eq41_e1237_d_b5, eq41_e1237_d_b6, eq41_e1237_d_b7, eq41_e1237_d_b8, eq41_e1237_d_b9, eq41_e1237_d_b10, eq41_e1237_d_b11, eq41_e1237_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq42_e1240: f64 = ((nv15 - 0.0) * s.v[955]);
        let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * s.dn[955][0]);
        let eq42_e1240_d_n1: f64 = ((nv15 - 0.0) * s.dn[955][1]);
        let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * s.dn[955][2]);
        let eq42_e1240_d_n3: f64 = ((nv15 - 0.0) * s.dn[955][3]);
        let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * s.dn[955][4]);
        let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * s.dn[955][5]);
        let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * s.dn[955][6]);
        let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * s.dn[955][7]);
        let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * s.dn[955][8]);
        let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * s.dn[955][9]);
        let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * s.dn[955][10]);
        let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * s.dn[955][11]);
        let eq42_e1240_d_n12: f64 = ((nv15 - 0.0) * s.dn[955][12]);
        let eq42_e1240_d_n13: f64 = ((nv15 - 0.0) * s.dn[955][13]);
        let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * s.dn[955][14]);
        let eq42_e1240_d_n15: f64 = (s.v[955] + ((nv15 - 0.0) * s.dn[955][15]));
        let eq42_e1240_d_n16: f64 = ((nv15 - 0.0) * s.dn[955][16]);
        let eq42_e1240_d_n17: f64 = ((nv15 - 0.0) * s.dn[955][17]);
        let eq42_e1240_d_n18: f64 = ((nv15 - 0.0) * s.dn[955][18]);
        let eq42_e1240_d_b0: f64 = ((nv15 - 0.0) * s.db[955][0]);
        let eq42_e1240_d_b1: f64 = ((nv15 - 0.0) * s.db[955][1]);
        let eq42_e1240_d_b2: f64 = ((nv15 - 0.0) * s.db[955][2]);
        let eq42_e1240_d_b3: f64 = ((nv15 - 0.0) * s.db[955][3]);
        let eq42_e1240_d_b4: f64 = ((nv15 - 0.0) * s.db[955][4]);
        let eq42_e1240_d_b5: f64 = ((nv15 - 0.0) * s.db[955][5]);
        let eq42_e1240_d_b6: f64 = ((nv15 - 0.0) * s.db[955][6]);
        let eq42_e1240_d_b7: f64 = ((nv15 - 0.0) * s.db[955][7]);
        let eq42_e1240_d_b8: f64 = ((nv15 - 0.0) * s.db[955][8]);
        let eq42_e1240_d_b9: f64 = ((nv15 - 0.0) * s.db[955][9]);
        let eq42_e1240_d_b10: f64 = ((nv15 - 0.0) * s.db[955][10]);
        let eq42_e1240_d_b11: f64 = ((nv15 - 0.0) * s.db[955][11]);
        let eq42_e1240_d_b12: f64 = ((nv15 - 0.0) * s.db[955][12]);
        let eq42_e1241: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, eq42_e1240);
        let eq42_e1241_d_n0: f64 = (eq42_e1240_d_n0 * ddt_scale);
        let eq42_e1241_d_n1: f64 = (eq42_e1240_d_n1 * ddt_scale);
        let eq42_e1241_d_n2: f64 = (eq42_e1240_d_n2 * ddt_scale);
        let eq42_e1241_d_n3: f64 = (eq42_e1240_d_n3 * ddt_scale);
        let eq42_e1241_d_n4: f64 = (eq42_e1240_d_n4 * ddt_scale);
        let eq42_e1241_d_n5: f64 = (eq42_e1240_d_n5 * ddt_scale);
        let eq42_e1241_d_n6: f64 = (eq42_e1240_d_n6 * ddt_scale);
        let eq42_e1241_d_n7: f64 = (eq42_e1240_d_n7 * ddt_scale);
        let eq42_e1241_d_n8: f64 = (eq42_e1240_d_n8 * ddt_scale);
        let eq42_e1241_d_n9: f64 = (eq42_e1240_d_n9 * ddt_scale);
        let eq42_e1241_d_n10: f64 = (eq42_e1240_d_n10 * ddt_scale);
        let eq42_e1241_d_n11: f64 = (eq42_e1240_d_n11 * ddt_scale);
        let eq42_e1241_d_n12: f64 = (eq42_e1240_d_n12 * ddt_scale);
        let eq42_e1241_d_n13: f64 = (eq42_e1240_d_n13 * ddt_scale);
        let eq42_e1241_d_n14: f64 = (eq42_e1240_d_n14 * ddt_scale);
        let eq42_e1241_d_n15: f64 = (eq42_e1240_d_n15 * ddt_scale);
        let eq42_e1241_d_n16: f64 = (eq42_e1240_d_n16 * ddt_scale);
        let eq42_e1241_d_n17: f64 = (eq42_e1240_d_n17 * ddt_scale);
        let eq42_e1241_d_n18: f64 = (eq42_e1240_d_n18 * ddt_scale);
        let eq42_e1241_d_b0: f64 = (eq42_e1240_d_b0 * ddt_scale);
        let eq42_e1241_d_b1: f64 = (eq42_e1240_d_b1 * ddt_scale);
        let eq42_e1241_d_b2: f64 = (eq42_e1240_d_b2 * ddt_scale);
        let eq42_e1241_d_b3: f64 = (eq42_e1240_d_b3 * ddt_scale);
        let eq42_e1241_d_b4: f64 = (eq42_e1240_d_b4 * ddt_scale);
        let eq42_e1241_d_b5: f64 = (eq42_e1240_d_b5 * ddt_scale);
        let eq42_e1241_d_b6: f64 = (eq42_e1240_d_b6 * ddt_scale);
        let eq42_e1241_d_b7: f64 = (eq42_e1240_d_b7 * ddt_scale);
        let eq42_e1241_d_b8: f64 = (eq42_e1240_d_b8 * ddt_scale);
        let eq42_e1241_d_b9: f64 = (eq42_e1240_d_b9 * ddt_scale);
        let eq42_e1241_d_b10: f64 = (eq42_e1240_d_b10 * ddt_scale);
        let eq42_e1241_d_b11: f64 = (eq42_e1240_d_b11 * ddt_scale);
        let eq42_e1241_d_b12: f64 = (eq42_e1240_d_b12 * ddt_scale);
        let eq42_value: f64 = eq42_e1241;
        let eq42_node_derivatives: [f64; 19] = [eq42_e1241_d_n0, eq42_e1241_d_n1, eq42_e1241_d_n2, eq42_e1241_d_n3, eq42_e1241_d_n4, eq42_e1241_d_n5, eq42_e1241_d_n6, eq42_e1241_d_n7, eq42_e1241_d_n8, eq42_e1241_d_n9, eq42_e1241_d_n10, eq42_e1241_d_n11, eq42_e1241_d_n12, eq42_e1241_d_n13, eq42_e1241_d_n14, eq42_e1241_d_n15, eq42_e1241_d_n16, eq42_e1241_d_n17, eq42_e1241_d_n18];
        let eq42_branch_derivatives: [f64; 13] = [eq42_e1241_d_b0, eq42_e1241_d_b1, eq42_e1241_d_b2, eq42_e1241_d_b3, eq42_e1241_d_b4, eq42_e1241_d_b5, eq42_e1241_d_b6, eq42_e1241_d_b7, eq42_e1241_d_b8, eq42_e1241_d_b9, eq42_e1241_d_b10, eq42_e1241_d_b11, eq42_e1241_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq57_e1336, eq57_e1336_d_n0, eq57_e1336_d_n1, eq57_e1336_d_n2, eq57_e1336_d_n3, eq57_e1336_d_n4, eq57_e1336_d_n5, eq57_e1336_d_n6, eq57_e1336_d_n7, eq57_e1336_d_n8, eq57_e1336_d_n9, eq57_e1336_d_n10, eq57_e1336_d_n11, eq57_e1336_d_n12, eq57_e1336_d_n13, eq57_e1336_d_n14, eq57_e1336_d_n15, eq57_e1336_d_n16, eq57_e1336_d_n17, eq57_e1336_d_n18, eq57_e1336_d_b0, eq57_e1336_d_b1, eq57_e1336_d_b2, eq57_e1336_d_b3, eq57_e1336_d_b4, eq57_e1336_d_b5, eq57_e1336_d_b6, eq57_e1336_d_b7, eq57_e1336_d_b8, eq57_e1336_d_b9, eq57_e1336_d_b10, eq57_e1336_d_b11, eq57_e1336_d_b12,) = {
    if s.b[3413] {
        let eq57_e1334: f64 = (-s.v[802]);
        let eq57_e1334_d_n0: f64 = (-s.dn[802][0]);
        let eq57_e1334_d_n1: f64 = (-s.dn[802][1]);
        let eq57_e1334_d_n2: f64 = (-s.dn[802][2]);
        let eq57_e1334_d_n3: f64 = (-s.dn[802][3]);
        let eq57_e1334_d_n4: f64 = (-s.dn[802][4]);
        let eq57_e1334_d_n5: f64 = (-s.dn[802][5]);
        let eq57_e1334_d_n6: f64 = (-s.dn[802][6]);
        let eq57_e1334_d_n7: f64 = (-s.dn[802][7]);
        let eq57_e1334_d_n8: f64 = (-s.dn[802][8]);
        let eq57_e1334_d_n9: f64 = (-s.dn[802][9]);
        let eq57_e1334_d_n10: f64 = (-s.dn[802][10]);
        let eq57_e1334_d_n11: f64 = (-s.dn[802][11]);
        let eq57_e1334_d_n12: f64 = (-s.dn[802][12]);
        let eq57_e1334_d_n13: f64 = (-s.dn[802][13]);
        let eq57_e1334_d_n14: f64 = (-s.dn[802][14]);
        let eq57_e1334_d_n15: f64 = (-s.dn[802][15]);
        let eq57_e1334_d_n16: f64 = (-s.dn[802][16]);
        let eq57_e1334_d_n17: f64 = (-s.dn[802][17]);
        let eq57_e1334_d_n18: f64 = (-s.dn[802][18]);
        let eq57_e1334_d_b0: f64 = (-s.db[802][0]);
        let eq57_e1334_d_b1: f64 = (-s.db[802][1]);
        let eq57_e1334_d_b2: f64 = (-s.db[802][2]);
        let eq57_e1334_d_b3: f64 = (-s.db[802][3]);
        let eq57_e1334_d_b4: f64 = (-s.db[802][4]);
        let eq57_e1334_d_b5: f64 = (-s.db[802][5]);
        let eq57_e1334_d_b6: f64 = (-s.db[802][6]);
        let eq57_e1334_d_b7: f64 = (-s.db[802][7]);
        let eq57_e1334_d_b8: f64 = (-s.db[802][8]);
        let eq57_e1334_d_b9: f64 = (-s.db[802][9]);
        let eq57_e1334_d_b10: f64 = (-s.db[802][10]);
        let eq57_e1334_d_b11: f64 = (-s.db[802][11]);
        let eq57_e1334_d_b12: f64 = (-s.db[802][12]);
        (eq57_e1334, eq57_e1334_d_n0, eq57_e1334_d_n1, eq57_e1334_d_n2, eq57_e1334_d_n3, eq57_e1334_d_n4, eq57_e1334_d_n5, eq57_e1334_d_n6, eq57_e1334_d_n7, eq57_e1334_d_n8, eq57_e1334_d_n9, eq57_e1334_d_n10, eq57_e1334_d_n11, eq57_e1334_d_n12, eq57_e1334_d_n13, eq57_e1334_d_n14, eq57_e1334_d_n15, eq57_e1334_d_n16, eq57_e1334_d_n17, eq57_e1334_d_n18, eq57_e1334_d_b0, eq57_e1334_d_b1, eq57_e1334_d_b2, eq57_e1334_d_b3, eq57_e1334_d_b4, eq57_e1334_d_b5, eq57_e1334_d_b6, eq57_e1334_d_b7, eq57_e1334_d_b8, eq57_e1334_d_b9, eq57_e1334_d_b10, eq57_e1334_d_b11, eq57_e1334_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1336;
        let eq57_node_derivatives: [f64; 19] = [eq57_e1336_d_n0, eq57_e1336_d_n1, eq57_e1336_d_n2, eq57_e1336_d_n3, eq57_e1336_d_n4, eq57_e1336_d_n5, eq57_e1336_d_n6, eq57_e1336_d_n7, eq57_e1336_d_n8, eq57_e1336_d_n9, eq57_e1336_d_n10, eq57_e1336_d_n11, eq57_e1336_d_n12, eq57_e1336_d_n13, eq57_e1336_d_n14, eq57_e1336_d_n15, eq57_e1336_d_n16, eq57_e1336_d_n17, eq57_e1336_d_n18];
        let eq57_branch_derivatives: [f64; 13] = [eq57_e1336_d_b0, eq57_e1336_d_b1, eq57_e1336_d_b2, eq57_e1336_d_b3, eq57_e1336_d_b4, eq57_e1336_d_b5, eq57_e1336_d_b6, eq57_e1336_d_b7, eq57_e1336_d_b8, eq57_e1336_d_b9, eq57_e1336_d_b10, eq57_e1336_d_b11, eq57_e1336_d_b12];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq60_e1351, eq60_e1351_d_n0, eq60_e1351_d_n1, eq60_e1351_d_n2, eq60_e1351_d_n3, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n13, eq60_e1351_d_n14, eq60_e1351_d_n15, eq60_e1351_d_n16, eq60_e1351_d_n17, eq60_e1351_d_n18, eq60_e1351_d_b0, eq60_e1351_d_b1, eq60_e1351_d_b2, eq60_e1351_d_b3, eq60_e1351_d_b4, eq60_e1351_d_b5, eq60_e1351_d_b6, eq60_e1351_d_b7, eq60_e1351_d_b8, eq60_e1351_d_b9, eq60_e1351_d_b10, eq60_e1351_d_b11, eq60_e1351_d_b12,) = {
    if (p.p28 != 0.0) {
        (s.v[749], s.dn[749][0], s.dn[749][1], s.dn[749][2], s.dn[749][3], s.dn[749][4], s.dn[749][5], s.dn[749][6], s.dn[749][7], s.dn[749][8], s.dn[749][9], s.dn[749][10], s.dn[749][11], s.dn[749][12], s.dn[749][13], s.dn[749][14], s.dn[749][15], s.dn[749][16], s.dn[749][17], s.dn[749][18], s.db[749][0], s.db[749][1], s.db[749][2], s.db[749][3], s.db[749][4], s.db[749][5], s.db[749][6], s.db[749][7], s.db[749][8], s.db[749][9], s.db[749][10], s.db[749][11], s.db[749][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1351;
        let eq60_node_derivatives: [f64; 19] = [eq60_e1351_d_n0, eq60_e1351_d_n1, eq60_e1351_d_n2, eq60_e1351_d_n3, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n13, eq60_e1351_d_n14, eq60_e1351_d_n15, eq60_e1351_d_n16, eq60_e1351_d_n17, eq60_e1351_d_n18];
        let eq60_branch_derivatives: [f64; 13] = [eq60_e1351_d_b0, eq60_e1351_d_b1, eq60_e1351_d_b2, eq60_e1351_d_b3, eq60_e1351_d_b4, eq60_e1351_d_b5, eq60_e1351_d_b6, eq60_e1351_d_b7, eq60_e1351_d_b8, eq60_e1351_d_b9, eq60_e1351_d_b10, eq60_e1351_d_b11, eq60_e1351_d_b12];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq60_value),
            &eq60_node_derivatives,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1355, eq61_e1355_d_n0, eq61_e1355_d_n1, eq61_e1355_d_n2, eq61_e1355_d_n3, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n12, eq61_e1355_d_n13, eq61_e1355_d_n14, eq61_e1355_d_n15, eq61_e1355_d_n16, eq61_e1355_d_n17, eq61_e1355_d_n18, eq61_e1355_d_b0, eq61_e1355_d_b1, eq61_e1355_d_b2, eq61_e1355_d_b3, eq61_e1355_d_b4, eq61_e1355_d_b5, eq61_e1355_d_b6, eq61_e1355_d_b7, eq61_e1355_d_b8, eq61_e1355_d_b9, eq61_e1355_d_b10, eq61_e1355_d_b11, eq61_e1355_d_b12,) = {
    if (p.p28 != 0.0) {
        (s.v[750], s.dn[750][0], s.dn[750][1], s.dn[750][2], s.dn[750][3], s.dn[750][4], s.dn[750][5], s.dn[750][6], s.dn[750][7], s.dn[750][8], s.dn[750][9], s.dn[750][10], s.dn[750][11], s.dn[750][12], s.dn[750][13], s.dn[750][14], s.dn[750][15], s.dn[750][16], s.dn[750][17], s.dn[750][18], s.db[750][0], s.db[750][1], s.db[750][2], s.db[750][3], s.db[750][4], s.db[750][5], s.db[750][6], s.db[750][7], s.db[750][8], s.db[750][9], s.db[750][10], s.db[750][11], s.db[750][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1355;
        let eq61_node_derivatives: [f64; 19] = [eq61_e1355_d_n0, eq61_e1355_d_n1, eq61_e1355_d_n2, eq61_e1355_d_n3, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n12, eq61_e1355_d_n13, eq61_e1355_d_n14, eq61_e1355_d_n15, eq61_e1355_d_n16, eq61_e1355_d_n17, eq61_e1355_d_n18];
        let eq61_branch_derivatives: [f64; 13] = [eq61_e1355_d_b0, eq61_e1355_d_b1, eq61_e1355_d_b2, eq61_e1355_d_b3, eq61_e1355_d_b4, eq61_e1355_d_b5, eq61_e1355_d_b6, eq61_e1355_d_b7, eq61_e1355_d_b8, eq61_e1355_d_b9, eq61_e1355_d_b10, eq61_e1355_d_b11, eq61_e1355_d_b12];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1362, eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_n18, eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1362_d_b9, eq62_e1362_d_b10, eq62_e1362_d_b11, eq62_e1362_d_b12,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (s.v[800] * (nv12 - 0.0));
        let eq62_e1359_d_n0: f64 = (s.dn[800][0] * (nv12 - 0.0));
        let eq62_e1359_d_n1: f64 = (s.dn[800][1] * (nv12 - 0.0));
        let eq62_e1359_d_n2: f64 = (s.dn[800][2] * (nv12 - 0.0));
        let eq62_e1359_d_n3: f64 = (s.dn[800][3] * (nv12 - 0.0));
        let eq62_e1359_d_n4: f64 = (s.dn[800][4] * (nv12 - 0.0));
        let eq62_e1359_d_n5: f64 = (s.dn[800][5] * (nv12 - 0.0));
        let eq62_e1359_d_n6: f64 = (s.dn[800][6] * (nv12 - 0.0));
        let eq62_e1359_d_n7: f64 = (s.dn[800][7] * (nv12 - 0.0));
        let eq62_e1359_d_n8: f64 = (s.dn[800][8] * (nv12 - 0.0));
        let eq62_e1359_d_n9: f64 = (s.dn[800][9] * (nv12 - 0.0));
        let eq62_e1359_d_n10: f64 = (s.dn[800][10] * (nv12 - 0.0));
        let eq62_e1359_d_n11: f64 = (s.dn[800][11] * (nv12 - 0.0));
        let eq62_e1359_d_n12: f64 = ((s.dn[800][12] * (nv12 - 0.0)) + s.v[800]);
        let eq62_e1359_d_n13: f64 = (s.dn[800][13] * (nv12 - 0.0));
        let eq62_e1359_d_n14: f64 = (s.dn[800][14] * (nv12 - 0.0));
        let eq62_e1359_d_n15: f64 = (s.dn[800][15] * (nv12 - 0.0));
        let eq62_e1359_d_n16: f64 = (s.dn[800][16] * (nv12 - 0.0));
        let eq62_e1359_d_n17: f64 = (s.dn[800][17] * (nv12 - 0.0));
        let eq62_e1359_d_n18: f64 = (s.dn[800][18] * (nv12 - 0.0));
        let eq62_e1359_d_b0: f64 = (s.db[800][0] * (nv12 - 0.0));
        let eq62_e1359_d_b1: f64 = (s.db[800][1] * (nv12 - 0.0));
        let eq62_e1359_d_b2: f64 = (s.db[800][2] * (nv12 - 0.0));
        let eq62_e1359_d_b3: f64 = (s.db[800][3] * (nv12 - 0.0));
        let eq62_e1359_d_b4: f64 = (s.db[800][4] * (nv12 - 0.0));
        let eq62_e1359_d_b5: f64 = (s.db[800][5] * (nv12 - 0.0));
        let eq62_e1359_d_b6: f64 = (s.db[800][6] * (nv12 - 0.0));
        let eq62_e1359_d_b7: f64 = (s.db[800][7] * (nv12 - 0.0));
        let eq62_e1359_d_b8: f64 = (s.db[800][8] * (nv12 - 0.0));
        let eq62_e1359_d_b9: f64 = (s.db[800][9] * (nv12 - 0.0));
        let eq62_e1359_d_b10: f64 = (s.db[800][10] * (nv12 - 0.0));
        let eq62_e1359_d_b11: f64 = (s.db[800][11] * (nv12 - 0.0));
        let eq62_e1359_d_b12: f64 = (s.db[800][12] * (nv12 - 0.0));
        let eq62_e1360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, eq62_e1359);
        let eq62_e1360_d_n0: f64 = (eq62_e1359_d_n0 * ddt_scale);
        let eq62_e1360_d_n1: f64 = (eq62_e1359_d_n1 * ddt_scale);
        let eq62_e1360_d_n2: f64 = (eq62_e1359_d_n2 * ddt_scale);
        let eq62_e1360_d_n3: f64 = (eq62_e1359_d_n3 * ddt_scale);
        let eq62_e1360_d_n4: f64 = (eq62_e1359_d_n4 * ddt_scale);
        let eq62_e1360_d_n5: f64 = (eq62_e1359_d_n5 * ddt_scale);
        let eq62_e1360_d_n6: f64 = (eq62_e1359_d_n6 * ddt_scale);
        let eq62_e1360_d_n7: f64 = (eq62_e1359_d_n7 * ddt_scale);
        let eq62_e1360_d_n8: f64 = (eq62_e1359_d_n8 * ddt_scale);
        let eq62_e1360_d_n9: f64 = (eq62_e1359_d_n9 * ddt_scale);
        let eq62_e1360_d_n10: f64 = (eq62_e1359_d_n10 * ddt_scale);
        let eq62_e1360_d_n11: f64 = (eq62_e1359_d_n11 * ddt_scale);
        let eq62_e1360_d_n12: f64 = (eq62_e1359_d_n12 * ddt_scale);
        let eq62_e1360_d_n13: f64 = (eq62_e1359_d_n13 * ddt_scale);
        let eq62_e1360_d_n14: f64 = (eq62_e1359_d_n14 * ddt_scale);
        let eq62_e1360_d_n15: f64 = (eq62_e1359_d_n15 * ddt_scale);
        let eq62_e1360_d_n16: f64 = (eq62_e1359_d_n16 * ddt_scale);
        let eq62_e1360_d_n17: f64 = (eq62_e1359_d_n17 * ddt_scale);
        let eq62_e1360_d_n18: f64 = (eq62_e1359_d_n18 * ddt_scale);
        let eq62_e1360_d_b0: f64 = (eq62_e1359_d_b0 * ddt_scale);
        let eq62_e1360_d_b1: f64 = (eq62_e1359_d_b1 * ddt_scale);
        let eq62_e1360_d_b2: f64 = (eq62_e1359_d_b2 * ddt_scale);
        let eq62_e1360_d_b3: f64 = (eq62_e1359_d_b3 * ddt_scale);
        let eq62_e1360_d_b4: f64 = (eq62_e1359_d_b4 * ddt_scale);
        let eq62_e1360_d_b5: f64 = (eq62_e1359_d_b5 * ddt_scale);
        let eq62_e1360_d_b6: f64 = (eq62_e1359_d_b6 * ddt_scale);
        let eq62_e1360_d_b7: f64 = (eq62_e1359_d_b7 * ddt_scale);
        let eq62_e1360_d_b8: f64 = (eq62_e1359_d_b8 * ddt_scale);
        let eq62_e1360_d_b9: f64 = (eq62_e1359_d_b9 * ddt_scale);
        let eq62_e1360_d_b10: f64 = (eq62_e1359_d_b10 * ddt_scale);
        let eq62_e1360_d_b11: f64 = (eq62_e1359_d_b11 * ddt_scale);
        let eq62_e1360_d_b12: f64 = (eq62_e1359_d_b12 * ddt_scale);
        (eq62_e1360, eq62_e1360_d_n0, eq62_e1360_d_n1, eq62_e1360_d_n2, eq62_e1360_d_n3, eq62_e1360_d_n4, eq62_e1360_d_n5, eq62_e1360_d_n6, eq62_e1360_d_n7, eq62_e1360_d_n8, eq62_e1360_d_n9, eq62_e1360_d_n10, eq62_e1360_d_n11, eq62_e1360_d_n12, eq62_e1360_d_n13, eq62_e1360_d_n14, eq62_e1360_d_n15, eq62_e1360_d_n16, eq62_e1360_d_n17, eq62_e1360_d_n18, eq62_e1360_d_b0, eq62_e1360_d_b1, eq62_e1360_d_b2, eq62_e1360_d_b3, eq62_e1360_d_b4, eq62_e1360_d_b5, eq62_e1360_d_b6, eq62_e1360_d_b7, eq62_e1360_d_b8, eq62_e1360_d_b9, eq62_e1360_d_b10, eq62_e1360_d_b11, eq62_e1360_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1362;
        let eq62_node_derivatives: [f64; 19] = [eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_n18];
        let eq62_branch_derivatives: [f64; 13] = [eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1362_d_b9, eq62_e1362_d_b10, eq62_e1362_d_b11, eq62_e1362_d_b12];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e1369, eq63_e1369_d_n0, eq63_e1369_d_n1, eq63_e1369_d_n2, eq63_e1369_d_n3, eq63_e1369_d_n4, eq63_e1369_d_n5, eq63_e1369_d_n6, eq63_e1369_d_n7, eq63_e1369_d_n8, eq63_e1369_d_n9, eq63_e1369_d_n10, eq63_e1369_d_n11, eq63_e1369_d_n12, eq63_e1369_d_n13, eq63_e1369_d_n14, eq63_e1369_d_n15, eq63_e1369_d_n16, eq63_e1369_d_n17, eq63_e1369_d_n18, eq63_e1369_d_b0, eq63_e1369_d_b1, eq63_e1369_d_b2, eq63_e1369_d_b3, eq63_e1369_d_b4, eq63_e1369_d_b5, eq63_e1369_d_b6, eq63_e1369_d_b7, eq63_e1369_d_b8, eq63_e1369_d_b9, eq63_e1369_d_b10, eq63_e1369_d_b11, eq63_e1369_d_b12,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (s.v[801] * (nv13 - 0.0));
        let eq63_e1366_d_n0: f64 = (s.dn[801][0] * (nv13 - 0.0));
        let eq63_e1366_d_n1: f64 = (s.dn[801][1] * (nv13 - 0.0));
        let eq63_e1366_d_n2: f64 = (s.dn[801][2] * (nv13 - 0.0));
        let eq63_e1366_d_n3: f64 = (s.dn[801][3] * (nv13 - 0.0));
        let eq63_e1366_d_n4: f64 = (s.dn[801][4] * (nv13 - 0.0));
        let eq63_e1366_d_n5: f64 = (s.dn[801][5] * (nv13 - 0.0));
        let eq63_e1366_d_n6: f64 = (s.dn[801][6] * (nv13 - 0.0));
        let eq63_e1366_d_n7: f64 = (s.dn[801][7] * (nv13 - 0.0));
        let eq63_e1366_d_n8: f64 = (s.dn[801][8] * (nv13 - 0.0));
        let eq63_e1366_d_n9: f64 = (s.dn[801][9] * (nv13 - 0.0));
        let eq63_e1366_d_n10: f64 = (s.dn[801][10] * (nv13 - 0.0));
        let eq63_e1366_d_n11: f64 = (s.dn[801][11] * (nv13 - 0.0));
        let eq63_e1366_d_n12: f64 = (s.dn[801][12] * (nv13 - 0.0));
        let eq63_e1366_d_n13: f64 = ((s.dn[801][13] * (nv13 - 0.0)) + s.v[801]);
        let eq63_e1366_d_n14: f64 = (s.dn[801][14] * (nv13 - 0.0));
        let eq63_e1366_d_n15: f64 = (s.dn[801][15] * (nv13 - 0.0));
        let eq63_e1366_d_n16: f64 = (s.dn[801][16] * (nv13 - 0.0));
        let eq63_e1366_d_n17: f64 = (s.dn[801][17] * (nv13 - 0.0));
        let eq63_e1366_d_n18: f64 = (s.dn[801][18] * (nv13 - 0.0));
        let eq63_e1366_d_b0: f64 = (s.db[801][0] * (nv13 - 0.0));
        let eq63_e1366_d_b1: f64 = (s.db[801][1] * (nv13 - 0.0));
        let eq63_e1366_d_b2: f64 = (s.db[801][2] * (nv13 - 0.0));
        let eq63_e1366_d_b3: f64 = (s.db[801][3] * (nv13 - 0.0));
        let eq63_e1366_d_b4: f64 = (s.db[801][4] * (nv13 - 0.0));
        let eq63_e1366_d_b5: f64 = (s.db[801][5] * (nv13 - 0.0));
        let eq63_e1366_d_b6: f64 = (s.db[801][6] * (nv13 - 0.0));
        let eq63_e1366_d_b7: f64 = (s.db[801][7] * (nv13 - 0.0));
        let eq63_e1366_d_b8: f64 = (s.db[801][8] * (nv13 - 0.0));
        let eq63_e1366_d_b9: f64 = (s.db[801][9] * (nv13 - 0.0));
        let eq63_e1366_d_b10: f64 = (s.db[801][10] * (nv13 - 0.0));
        let eq63_e1366_d_b11: f64 = (s.db[801][11] * (nv13 - 0.0));
        let eq63_e1366_d_b12: f64 = (s.db[801][12] * (nv13 - 0.0));
        let eq63_e1367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, eq63_e1366);
        let eq63_e1367_d_n0: f64 = (eq63_e1366_d_n0 * ddt_scale);
        let eq63_e1367_d_n1: f64 = (eq63_e1366_d_n1 * ddt_scale);
        let eq63_e1367_d_n2: f64 = (eq63_e1366_d_n2 * ddt_scale);
        let eq63_e1367_d_n3: f64 = (eq63_e1366_d_n3 * ddt_scale);
        let eq63_e1367_d_n4: f64 = (eq63_e1366_d_n4 * ddt_scale);
        let eq63_e1367_d_n5: f64 = (eq63_e1366_d_n5 * ddt_scale);
        let eq63_e1367_d_n6: f64 = (eq63_e1366_d_n6 * ddt_scale);
        let eq63_e1367_d_n7: f64 = (eq63_e1366_d_n7 * ddt_scale);
        let eq63_e1367_d_n8: f64 = (eq63_e1366_d_n8 * ddt_scale);
        let eq63_e1367_d_n9: f64 = (eq63_e1366_d_n9 * ddt_scale);
        let eq63_e1367_d_n10: f64 = (eq63_e1366_d_n10 * ddt_scale);
        let eq63_e1367_d_n11: f64 = (eq63_e1366_d_n11 * ddt_scale);
        let eq63_e1367_d_n12: f64 = (eq63_e1366_d_n12 * ddt_scale);
        let eq63_e1367_d_n13: f64 = (eq63_e1366_d_n13 * ddt_scale);
        let eq63_e1367_d_n14: f64 = (eq63_e1366_d_n14 * ddt_scale);
        let eq63_e1367_d_n15: f64 = (eq63_e1366_d_n15 * ddt_scale);
        let eq63_e1367_d_n16: f64 = (eq63_e1366_d_n16 * ddt_scale);
        let eq63_e1367_d_n17: f64 = (eq63_e1366_d_n17 * ddt_scale);
        let eq63_e1367_d_n18: f64 = (eq63_e1366_d_n18 * ddt_scale);
        let eq63_e1367_d_b0: f64 = (eq63_e1366_d_b0 * ddt_scale);
        let eq63_e1367_d_b1: f64 = (eq63_e1366_d_b1 * ddt_scale);
        let eq63_e1367_d_b2: f64 = (eq63_e1366_d_b2 * ddt_scale);
        let eq63_e1367_d_b3: f64 = (eq63_e1366_d_b3 * ddt_scale);
        let eq63_e1367_d_b4: f64 = (eq63_e1366_d_b4 * ddt_scale);
        let eq63_e1367_d_b5: f64 = (eq63_e1366_d_b5 * ddt_scale);
        let eq63_e1367_d_b6: f64 = (eq63_e1366_d_b6 * ddt_scale);
        let eq63_e1367_d_b7: f64 = (eq63_e1366_d_b7 * ddt_scale);
        let eq63_e1367_d_b8: f64 = (eq63_e1366_d_b8 * ddt_scale);
        let eq63_e1367_d_b9: f64 = (eq63_e1366_d_b9 * ddt_scale);
        let eq63_e1367_d_b10: f64 = (eq63_e1366_d_b10 * ddt_scale);
        let eq63_e1367_d_b11: f64 = (eq63_e1366_d_b11 * ddt_scale);
        let eq63_e1367_d_b12: f64 = (eq63_e1366_d_b12 * ddt_scale);
        (eq63_e1367, eq63_e1367_d_n0, eq63_e1367_d_n1, eq63_e1367_d_n2, eq63_e1367_d_n3, eq63_e1367_d_n4, eq63_e1367_d_n5, eq63_e1367_d_n6, eq63_e1367_d_n7, eq63_e1367_d_n8, eq63_e1367_d_n9, eq63_e1367_d_n10, eq63_e1367_d_n11, eq63_e1367_d_n12, eq63_e1367_d_n13, eq63_e1367_d_n14, eq63_e1367_d_n15, eq63_e1367_d_n16, eq63_e1367_d_n17, eq63_e1367_d_n18, eq63_e1367_d_b0, eq63_e1367_d_b1, eq63_e1367_d_b2, eq63_e1367_d_b3, eq63_e1367_d_b4, eq63_e1367_d_b5, eq63_e1367_d_b6, eq63_e1367_d_b7, eq63_e1367_d_b8, eq63_e1367_d_b9, eq63_e1367_d_b10, eq63_e1367_d_b11, eq63_e1367_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1369;
        let eq63_node_derivatives: [f64; 19] = [eq63_e1369_d_n0, eq63_e1369_d_n1, eq63_e1369_d_n2, eq63_e1369_d_n3, eq63_e1369_d_n4, eq63_e1369_d_n5, eq63_e1369_d_n6, eq63_e1369_d_n7, eq63_e1369_d_n8, eq63_e1369_d_n9, eq63_e1369_d_n10, eq63_e1369_d_n11, eq63_e1369_d_n12, eq63_e1369_d_n13, eq63_e1369_d_n14, eq63_e1369_d_n15, eq63_e1369_d_n16, eq63_e1369_d_n17, eq63_e1369_d_n18];
        let eq63_branch_derivatives: [f64; 13] = [eq63_e1369_d_b0, eq63_e1369_d_b1, eq63_e1369_d_b2, eq63_e1369_d_b3, eq63_e1369_d_b4, eq63_e1369_d_b5, eq63_e1369_d_b6, eq63_e1369_d_b7, eq63_e1369_d_b8, eq63_e1369_d_b9, eq63_e1369_d_b10, eq63_e1369_d_b11, eq63_e1369_d_b12];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1383, eq66_e1383_d_n0, eq66_e1383_d_n1, eq66_e1383_d_n2, eq66_e1383_d_n3, eq66_e1383_d_n4, eq66_e1383_d_n5, eq66_e1383_d_n6, eq66_e1383_d_n7, eq66_e1383_d_n8, eq66_e1383_d_n9, eq66_e1383_d_n10, eq66_e1383_d_n11, eq66_e1383_d_n12, eq66_e1383_d_n13, eq66_e1383_d_n14, eq66_e1383_d_n15, eq66_e1383_d_n16, eq66_e1383_d_n17, eq66_e1383_d_n18, eq66_e1383_d_b0, eq66_e1383_d_b1, eq66_e1383_d_b2, eq66_e1383_d_b3, eq66_e1383_d_b4, eq66_e1383_d_b5, eq66_e1383_d_b6, eq66_e1383_d_b7, eq66_e1383_d_b8, eq66_e1383_d_b9, eq66_e1383_d_b10, eq66_e1383_d_b11, eq66_e1383_d_b12,) = {
    if (p.p29 != 0.0) {
        (s.v[815], s.dn[815][0], s.dn[815][1], s.dn[815][2], s.dn[815][3], s.dn[815][4], s.dn[815][5], s.dn[815][6], s.dn[815][7], s.dn[815][8], s.dn[815][9], s.dn[815][10], s.dn[815][11], s.dn[815][12], s.dn[815][13], s.dn[815][14], s.dn[815][15], s.dn[815][16], s.dn[815][17], s.dn[815][18], s.db[815][0], s.db[815][1], s.db[815][2], s.db[815][3], s.db[815][4], s.db[815][5], s.db[815][6], s.db[815][7], s.db[815][8], s.db[815][9], s.db[815][10], s.db[815][11], s.db[815][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1383;
        let eq66_node_derivatives: [f64; 19] = [eq66_e1383_d_n0, eq66_e1383_d_n1, eq66_e1383_d_n2, eq66_e1383_d_n3, eq66_e1383_d_n4, eq66_e1383_d_n5, eq66_e1383_d_n6, eq66_e1383_d_n7, eq66_e1383_d_n8, eq66_e1383_d_n9, eq66_e1383_d_n10, eq66_e1383_d_n11, eq66_e1383_d_n12, eq66_e1383_d_n13, eq66_e1383_d_n14, eq66_e1383_d_n15, eq66_e1383_d_n16, eq66_e1383_d_n17, eq66_e1383_d_n18];
        let eq66_branch_derivatives: [f64; 13] = [eq66_e1383_d_b0, eq66_e1383_d_b1, eq66_e1383_d_b2, eq66_e1383_d_b3, eq66_e1383_d_b4, eq66_e1383_d_b5, eq66_e1383_d_b6, eq66_e1383_d_b7, eq66_e1383_d_b8, eq66_e1383_d_b9, eq66_e1383_d_b10, eq66_e1383_d_b11, eq66_e1383_d_b12];
        stamper.stamp_current_dense_local(
            Some(14),
            None,
            multiplicity * (eq66_value),
            &eq66_node_derivatives,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let (eq67_e1388, eq67_e1388_d_n14,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, (nv14 - 0.0));
        (eq67_e1386, ddt_scale,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e1388;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq67_value),
            14,
            multiplicity * (eq67_e1388_d_n14),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq1_e1022, eq1_e1022_d_n0, eq1_e1022_d_n1, eq1_e1022_d_n2, eq1_e1022_d_n3, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n12, eq1_e1022_d_n13, eq1_e1022_d_n14, eq1_e1022_d_n15, eq1_e1022_d_n16, eq1_e1022_d_n17, eq1_e1022_d_n18, eq1_e1022_d_b0, eq1_e1022_d_b1, eq1_e1022_d_b2, eq1_e1022_d_b3, eq1_e1022_d_b4, eq1_e1022_d_b5, eq1_e1022_d_b6, eq1_e1022_d_b7, eq1_e1022_d_b8, eq1_e1022_d_b9, eq1_e1022_d_b10, eq1_e1022_d_b11, eq1_e1022_d_b12, eq1_e1022_q, eq1_e1022_q_d_n0, eq1_e1022_q_d_n1, eq1_e1022_q_d_n2, eq1_e1022_q_d_n3, eq1_e1022_q_d_n4, eq1_e1022_q_d_n5, eq1_e1022_q_d_n6, eq1_e1022_q_d_n7, eq1_e1022_q_d_n8, eq1_e1022_q_d_n9, eq1_e1022_q_d_n10, eq1_e1022_q_d_n11, eq1_e1022_q_d_n12, eq1_e1022_q_d_n13, eq1_e1022_q_d_n14, eq1_e1022_q_d_n15, eq1_e1022_q_d_n16, eq1_e1022_q_d_n17, eq1_e1022_q_d_n18, eq1_e1022_q_d_b0, eq1_e1022_q_d_b1, eq1_e1022_q_d_b2, eq1_e1022_q_d_b3, eq1_e1022_q_d_b4, eq1_e1022_q_d_b5, eq1_e1022_q_d_b6, eq1_e1022_q_d_b7, eq1_e1022_q_d_b8, eq1_e1022_q_d_b9, eq1_e1022_q_d_b10, eq1_e1022_q_d_b11, eq1_e1022_q_d_b12,) = {
    if s.b[3309] {
        let eq1_e1019_q: f64 = s.v[924];
        let eq1_e1020: f64 = (s.v[926] + s.v[924]);
        let eq1_e1020_d_n0: f64 = (s.dn[926][0] + s.dn[924][0]);
        let eq1_e1020_d_n1: f64 = (s.dn[926][1] + s.dn[924][1]);
        let eq1_e1020_d_n2: f64 = (s.dn[926][2] + s.dn[924][2]);
        let eq1_e1020_d_n3: f64 = (s.dn[926][3] + s.dn[924][3]);
        let eq1_e1020_d_n4: f64 = (s.dn[926][4] + s.dn[924][4]);
        let eq1_e1020_d_n5: f64 = (s.dn[926][5] + s.dn[924][5]);
        let eq1_e1020_d_n6: f64 = (s.dn[926][6] + s.dn[924][6]);
        let eq1_e1020_d_n7: f64 = (s.dn[926][7] + s.dn[924][7]);
        let eq1_e1020_d_n8: f64 = (s.dn[926][8] + s.dn[924][8]);
        let eq1_e1020_d_n9: f64 = (s.dn[926][9] + s.dn[924][9]);
        let eq1_e1020_d_n10: f64 = (s.dn[926][10] + s.dn[924][10]);
        let eq1_e1020_d_n11: f64 = (s.dn[926][11] + s.dn[924][11]);
        let eq1_e1020_d_n12: f64 = (s.dn[926][12] + s.dn[924][12]);
        let eq1_e1020_d_n13: f64 = (s.dn[926][13] + s.dn[924][13]);
        let eq1_e1020_d_n14: f64 = (s.dn[926][14] + s.dn[924][14]);
        let eq1_e1020_d_n15: f64 = (s.dn[926][15] + s.dn[924][15]);
        let eq1_e1020_d_n16: f64 = (s.dn[926][16] + s.dn[924][16]);
        let eq1_e1020_d_n17: f64 = (s.dn[926][17] + s.dn[924][17]);
        let eq1_e1020_d_n18: f64 = (s.dn[926][18] + s.dn[924][18]);
        let eq1_e1020_d_b0: f64 = (s.db[926][0] + s.db[924][0]);
        let eq1_e1020_d_b1: f64 = (s.db[926][1] + s.db[924][1]);
        let eq1_e1020_d_b2: f64 = (s.db[926][2] + s.db[924][2]);
        let eq1_e1020_d_b3: f64 = (s.db[926][3] + s.db[924][3]);
        let eq1_e1020_d_b4: f64 = (s.db[926][4] + s.db[924][4]);
        let eq1_e1020_d_b5: f64 = (s.db[926][5] + s.db[924][5]);
        let eq1_e1020_d_b6: f64 = (s.db[926][6] + s.db[924][6]);
        let eq1_e1020_d_b7: f64 = (s.db[926][7] + s.db[924][7]);
        let eq1_e1020_d_b8: f64 = (s.db[926][8] + s.db[924][8]);
        let eq1_e1020_d_b9: f64 = (s.db[926][9] + s.db[924][9]);
        let eq1_e1020_d_b10: f64 = (s.db[926][10] + s.db[924][10]);
        let eq1_e1020_d_b11: f64 = (s.db[926][11] + s.db[924][11]);
        let eq1_e1020_d_b12: f64 = (s.db[926][12] + s.db[924][12]);
        let eq1_e1020_q: f64 = eq1_e1019_q;
        (eq1_e1020, eq1_e1020_d_n0, eq1_e1020_d_n1, eq1_e1020_d_n2, eq1_e1020_d_n3, eq1_e1020_d_n4, eq1_e1020_d_n5, eq1_e1020_d_n6, eq1_e1020_d_n7, eq1_e1020_d_n8, eq1_e1020_d_n9, eq1_e1020_d_n10, eq1_e1020_d_n11, eq1_e1020_d_n12, eq1_e1020_d_n13, eq1_e1020_d_n14, eq1_e1020_d_n15, eq1_e1020_d_n16, eq1_e1020_d_n17, eq1_e1020_d_n18, eq1_e1020_d_b0, eq1_e1020_d_b1, eq1_e1020_d_b2, eq1_e1020_d_b3, eq1_e1020_d_b4, eq1_e1020_d_b5, eq1_e1020_d_b6, eq1_e1020_d_b7, eq1_e1020_d_b8, eq1_e1020_d_b9, eq1_e1020_d_b10, eq1_e1020_d_b11, eq1_e1020_d_b12, eq1_e1020_q, s.dn[924][0], s.dn[924][1], s.dn[924][2], s.dn[924][3], s.dn[924][4], s.dn[924][5], s.dn[924][6], s.dn[924][7], s.dn[924][8], s.dn[924][9], s.dn[924][10], s.dn[924][11], s.dn[924][12], s.dn[924][13], s.dn[924][14], s.dn[924][15], s.dn[924][16], s.dn[924][17], s.dn[924][18], s.db[924][0], s.db[924][1], s.db[924][2], s.db[924][3], s.db[924][4], s.db[924][5], s.db[924][6], s.db[924][7], s.db[924][8], s.db[924][9], s.db[924][10], s.db[924][11], s.db[924][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_reactive_node_derivatives: [f64; 19] = [eq1_e1022_q_d_n0, eq1_e1022_q_d_n1, eq1_e1022_q_d_n2, eq1_e1022_q_d_n3, eq1_e1022_q_d_n4, eq1_e1022_q_d_n5, eq1_e1022_q_d_n6, eq1_e1022_q_d_n7, eq1_e1022_q_d_n8, eq1_e1022_q_d_n9, eq1_e1022_q_d_n10, eq1_e1022_q_d_n11, eq1_e1022_q_d_n12, eq1_e1022_q_d_n13, eq1_e1022_q_d_n14, eq1_e1022_q_d_n15, eq1_e1022_q_d_n16, eq1_e1022_q_d_n17, eq1_e1022_q_d_n18];
        let eq1_reactive_branch_derivatives: [f64; 13] = [eq1_e1022_q_d_b0, eq1_e1022_q_d_b1, eq1_e1022_q_d_b2, eq1_e1022_q_d_b3, eq1_e1022_q_d_b4, eq1_e1022_q_d_b5, eq1_e1022_q_d_b6, eq1_e1022_q_d_b7, eq1_e1022_q_d_b8, eq1_e1022_q_d_b9, eq1_e1022_q_d_b10, eq1_e1022_q_d_b11, eq1_e1022_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[16]),
            None,
            nodes,
            &eq1_reactive_node_derivatives,
            branches,
            &eq1_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1029, eq2_e1029_d_n0, eq2_e1029_d_n1, eq2_e1029_d_n2, eq2_e1029_d_n3, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n12, eq2_e1029_d_n13, eq2_e1029_d_n14, eq2_e1029_d_n15, eq2_e1029_d_n16, eq2_e1029_d_n17, eq2_e1029_d_n18, eq2_e1029_d_b0, eq2_e1029_d_b1, eq2_e1029_d_b2, eq2_e1029_d_b3, eq2_e1029_d_b4, eq2_e1029_d_b5, eq2_e1029_d_b6, eq2_e1029_d_b7, eq2_e1029_d_b8, eq2_e1029_d_b9, eq2_e1029_d_b10, eq2_e1029_d_b11, eq2_e1029_d_b12, eq2_e1029_q, eq2_e1029_q_d_n0, eq2_e1029_q_d_n1, eq2_e1029_q_d_n2, eq2_e1029_q_d_n3, eq2_e1029_q_d_n4, eq2_e1029_q_d_n5, eq2_e1029_q_d_n6, eq2_e1029_q_d_n7, eq2_e1029_q_d_n8, eq2_e1029_q_d_n9, eq2_e1029_q_d_n10, eq2_e1029_q_d_n11, eq2_e1029_q_d_n12, eq2_e1029_q_d_n13, eq2_e1029_q_d_n14, eq2_e1029_q_d_n15, eq2_e1029_q_d_n16, eq2_e1029_q_d_n17, eq2_e1029_q_d_n18, eq2_e1029_q_d_b0, eq2_e1029_q_d_b1, eq2_e1029_q_d_b2, eq2_e1029_q_d_b3, eq2_e1029_q_d_b4, eq2_e1029_q_d_b5, eq2_e1029_q_d_b6, eq2_e1029_q_d_b7, eq2_e1029_q_d_b8, eq2_e1029_q_d_b9, eq2_e1029_q_d_b10, eq2_e1029_q_d_b11, eq2_e1029_q_d_b12,) = {
    if s.b[3309] {
        let eq2_e1026_q: f64 = s.v[925];
        let eq2_e1027: f64 = (s.v[927] + s.v[925]);
        let eq2_e1027_d_n0: f64 = (s.dn[927][0] + s.dn[925][0]);
        let eq2_e1027_d_n1: f64 = (s.dn[927][1] + s.dn[925][1]);
        let eq2_e1027_d_n2: f64 = (s.dn[927][2] + s.dn[925][2]);
        let eq2_e1027_d_n3: f64 = (s.dn[927][3] + s.dn[925][3]);
        let eq2_e1027_d_n4: f64 = (s.dn[927][4] + s.dn[925][4]);
        let eq2_e1027_d_n5: f64 = (s.dn[927][5] + s.dn[925][5]);
        let eq2_e1027_d_n6: f64 = (s.dn[927][6] + s.dn[925][6]);
        let eq2_e1027_d_n7: f64 = (s.dn[927][7] + s.dn[925][7]);
        let eq2_e1027_d_n8: f64 = (s.dn[927][8] + s.dn[925][8]);
        let eq2_e1027_d_n9: f64 = (s.dn[927][9] + s.dn[925][9]);
        let eq2_e1027_d_n10: f64 = (s.dn[927][10] + s.dn[925][10]);
        let eq2_e1027_d_n11: f64 = (s.dn[927][11] + s.dn[925][11]);
        let eq2_e1027_d_n12: f64 = (s.dn[927][12] + s.dn[925][12]);
        let eq2_e1027_d_n13: f64 = (s.dn[927][13] + s.dn[925][13]);
        let eq2_e1027_d_n14: f64 = (s.dn[927][14] + s.dn[925][14]);
        let eq2_e1027_d_n15: f64 = (s.dn[927][15] + s.dn[925][15]);
        let eq2_e1027_d_n16: f64 = (s.dn[927][16] + s.dn[925][16]);
        let eq2_e1027_d_n17: f64 = (s.dn[927][17] + s.dn[925][17]);
        let eq2_e1027_d_n18: f64 = (s.dn[927][18] + s.dn[925][18]);
        let eq2_e1027_d_b0: f64 = (s.db[927][0] + s.db[925][0]);
        let eq2_e1027_d_b1: f64 = (s.db[927][1] + s.db[925][1]);
        let eq2_e1027_d_b2: f64 = (s.db[927][2] + s.db[925][2]);
        let eq2_e1027_d_b3: f64 = (s.db[927][3] + s.db[925][3]);
        let eq2_e1027_d_b4: f64 = (s.db[927][4] + s.db[925][4]);
        let eq2_e1027_d_b5: f64 = (s.db[927][5] + s.db[925][5]);
        let eq2_e1027_d_b6: f64 = (s.db[927][6] + s.db[925][6]);
        let eq2_e1027_d_b7: f64 = (s.db[927][7] + s.db[925][7]);
        let eq2_e1027_d_b8: f64 = (s.db[927][8] + s.db[925][8]);
        let eq2_e1027_d_b9: f64 = (s.db[927][9] + s.db[925][9]);
        let eq2_e1027_d_b10: f64 = (s.db[927][10] + s.db[925][10]);
        let eq2_e1027_d_b11: f64 = (s.db[927][11] + s.db[925][11]);
        let eq2_e1027_d_b12: f64 = (s.db[927][12] + s.db[925][12]);
        let eq2_e1027_q: f64 = eq2_e1026_q;
        (eq2_e1027, eq2_e1027_d_n0, eq2_e1027_d_n1, eq2_e1027_d_n2, eq2_e1027_d_n3, eq2_e1027_d_n4, eq2_e1027_d_n5, eq2_e1027_d_n6, eq2_e1027_d_n7, eq2_e1027_d_n8, eq2_e1027_d_n9, eq2_e1027_d_n10, eq2_e1027_d_n11, eq2_e1027_d_n12, eq2_e1027_d_n13, eq2_e1027_d_n14, eq2_e1027_d_n15, eq2_e1027_d_n16, eq2_e1027_d_n17, eq2_e1027_d_n18, eq2_e1027_d_b0, eq2_e1027_d_b1, eq2_e1027_d_b2, eq2_e1027_d_b3, eq2_e1027_d_b4, eq2_e1027_d_b5, eq2_e1027_d_b6, eq2_e1027_d_b7, eq2_e1027_d_b8, eq2_e1027_d_b9, eq2_e1027_d_b10, eq2_e1027_d_b11, eq2_e1027_d_b12, eq2_e1027_q, s.dn[925][0], s.dn[925][1], s.dn[925][2], s.dn[925][3], s.dn[925][4], s.dn[925][5], s.dn[925][6], s.dn[925][7], s.dn[925][8], s.dn[925][9], s.dn[925][10], s.dn[925][11], s.dn[925][12], s.dn[925][13], s.dn[925][14], s.dn[925][15], s.dn[925][16], s.dn[925][17], s.dn[925][18], s.db[925][0], s.db[925][1], s.db[925][2], s.db[925][3], s.db[925][4], s.db[925][5], s.db[925][6], s.db[925][7], s.db[925][8], s.db[925][9], s.db[925][10], s.db[925][11], s.db[925][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_reactive_node_derivatives: [f64; 19] = [eq2_e1029_q_d_n0, eq2_e1029_q_d_n1, eq2_e1029_q_d_n2, eq2_e1029_q_d_n3, eq2_e1029_q_d_n4, eq2_e1029_q_d_n5, eq2_e1029_q_d_n6, eq2_e1029_q_d_n7, eq2_e1029_q_d_n8, eq2_e1029_q_d_n9, eq2_e1029_q_d_n10, eq2_e1029_q_d_n11, eq2_e1029_q_d_n12, eq2_e1029_q_d_n13, eq2_e1029_q_d_n14, eq2_e1029_q_d_n15, eq2_e1029_q_d_n16, eq2_e1029_q_d_n17, eq2_e1029_q_d_n18];
        let eq2_reactive_branch_derivatives: [f64; 13] = [eq2_e1029_q_d_b0, eq2_e1029_q_d_b1, eq2_e1029_q_d_b2, eq2_e1029_q_d_b3, eq2_e1029_q_d_b4, eq2_e1029_q_d_b5, eq2_e1029_q_d_b6, eq2_e1029_q_d_b7, eq2_e1029_q_d_b8, eq2_e1029_q_d_b9, eq2_e1029_q_d_b10, eq2_e1029_q_d_b11, eq2_e1029_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[17]),
            None,
            nodes,
            &eq2_reactive_node_derivatives,
            branches,
            &eq2_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1046, eq5_e1046_d_n0, eq5_e1046_d_n1, eq5_e1046_d_n2, eq5_e1046_d_n3, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n12, eq5_e1046_d_n13, eq5_e1046_d_n14, eq5_e1046_d_n15, eq5_e1046_d_n16, eq5_e1046_d_n17, eq5_e1046_d_n18, eq5_e1046_d_b0, eq5_e1046_d_b1, eq5_e1046_d_b2, eq5_e1046_d_b3, eq5_e1046_d_b4, eq5_e1046_d_b5, eq5_e1046_d_b6, eq5_e1046_d_b7, eq5_e1046_d_b8, eq5_e1046_d_b9, eq5_e1046_d_b10, eq5_e1046_d_b11, eq5_e1046_d_b12, eq5_e1046_q, eq5_e1046_q_d_n0, eq5_e1046_q_d_n1, eq5_e1046_q_d_n2, eq5_e1046_q_d_n3, eq5_e1046_q_d_n4, eq5_e1046_q_d_n5, eq5_e1046_q_d_n6, eq5_e1046_q_d_n7, eq5_e1046_q_d_n8, eq5_e1046_q_d_n9, eq5_e1046_q_d_n10, eq5_e1046_q_d_n11, eq5_e1046_q_d_n12, eq5_e1046_q_d_n13, eq5_e1046_q_d_n14, eq5_e1046_q_d_n15, eq5_e1046_q_d_n16, eq5_e1046_q_d_n17, eq5_e1046_q_d_n18, eq5_e1046_q_d_b0, eq5_e1046_q_d_b1, eq5_e1046_q_d_b2, eq5_e1046_q_d_b3, eq5_e1046_q_d_b4, eq5_e1046_q_d_b5, eq5_e1046_q_d_b6, eq5_e1046_q_d_b7, eq5_e1046_q_d_b8, eq5_e1046_q_d_b9, eq5_e1046_q_d_b10, eq5_e1046_q_d_b11, eq5_e1046_q_d_b12,) = {
    if s.b[3310] {
        let eq5_e1043_q: f64 = s.v[931];
        let eq5_e1044: f64 = (s.v[932] + s.v[931]);
        let eq5_e1044_d_n0: f64 = (s.dn[932][0] + s.dn[931][0]);
        let eq5_e1044_d_n1: f64 = (s.dn[932][1] + s.dn[931][1]);
        let eq5_e1044_d_n2: f64 = (s.dn[932][2] + s.dn[931][2]);
        let eq5_e1044_d_n3: f64 = (s.dn[932][3] + s.dn[931][3]);
        let eq5_e1044_d_n4: f64 = (s.dn[932][4] + s.dn[931][4]);
        let eq5_e1044_d_n5: f64 = (s.dn[932][5] + s.dn[931][5]);
        let eq5_e1044_d_n6: f64 = (s.dn[932][6] + s.dn[931][6]);
        let eq5_e1044_d_n7: f64 = (s.dn[932][7] + s.dn[931][7]);
        let eq5_e1044_d_n8: f64 = (s.dn[932][8] + s.dn[931][8]);
        let eq5_e1044_d_n9: f64 = (s.dn[932][9] + s.dn[931][9]);
        let eq5_e1044_d_n10: f64 = (s.dn[932][10] + s.dn[931][10]);
        let eq5_e1044_d_n11: f64 = (s.dn[932][11] + s.dn[931][11]);
        let eq5_e1044_d_n12: f64 = (s.dn[932][12] + s.dn[931][12]);
        let eq5_e1044_d_n13: f64 = (s.dn[932][13] + s.dn[931][13]);
        let eq5_e1044_d_n14: f64 = (s.dn[932][14] + s.dn[931][14]);
        let eq5_e1044_d_n15: f64 = (s.dn[932][15] + s.dn[931][15]);
        let eq5_e1044_d_n16: f64 = (s.dn[932][16] + s.dn[931][16]);
        let eq5_e1044_d_n17: f64 = (s.dn[932][17] + s.dn[931][17]);
        let eq5_e1044_d_n18: f64 = (s.dn[932][18] + s.dn[931][18]);
        let eq5_e1044_d_b0: f64 = (s.db[932][0] + s.db[931][0]);
        let eq5_e1044_d_b1: f64 = (s.db[932][1] + s.db[931][1]);
        let eq5_e1044_d_b2: f64 = (s.db[932][2] + s.db[931][2]);
        let eq5_e1044_d_b3: f64 = (s.db[932][3] + s.db[931][3]);
        let eq5_e1044_d_b4: f64 = (s.db[932][4] + s.db[931][4]);
        let eq5_e1044_d_b5: f64 = (s.db[932][5] + s.db[931][5]);
        let eq5_e1044_d_b6: f64 = (s.db[932][6] + s.db[931][6]);
        let eq5_e1044_d_b7: f64 = (s.db[932][7] + s.db[931][7]);
        let eq5_e1044_d_b8: f64 = (s.db[932][8] + s.db[931][8]);
        let eq5_e1044_d_b9: f64 = (s.db[932][9] + s.db[931][9]);
        let eq5_e1044_d_b10: f64 = (s.db[932][10] + s.db[931][10]);
        let eq5_e1044_d_b11: f64 = (s.db[932][11] + s.db[931][11]);
        let eq5_e1044_d_b12: f64 = (s.db[932][12] + s.db[931][12]);
        let eq5_e1044_q: f64 = eq5_e1043_q;
        (eq5_e1044, eq5_e1044_d_n0, eq5_e1044_d_n1, eq5_e1044_d_n2, eq5_e1044_d_n3, eq5_e1044_d_n4, eq5_e1044_d_n5, eq5_e1044_d_n6, eq5_e1044_d_n7, eq5_e1044_d_n8, eq5_e1044_d_n9, eq5_e1044_d_n10, eq5_e1044_d_n11, eq5_e1044_d_n12, eq5_e1044_d_n13, eq5_e1044_d_n14, eq5_e1044_d_n15, eq5_e1044_d_n16, eq5_e1044_d_n17, eq5_e1044_d_n18, eq5_e1044_d_b0, eq5_e1044_d_b1, eq5_e1044_d_b2, eq5_e1044_d_b3, eq5_e1044_d_b4, eq5_e1044_d_b5, eq5_e1044_d_b6, eq5_e1044_d_b7, eq5_e1044_d_b8, eq5_e1044_d_b9, eq5_e1044_d_b10, eq5_e1044_d_b11, eq5_e1044_d_b12, eq5_e1044_q, s.dn[931][0], s.dn[931][1], s.dn[931][2], s.dn[931][3], s.dn[931][4], s.dn[931][5], s.dn[931][6], s.dn[931][7], s.dn[931][8], s.dn[931][9], s.dn[931][10], s.dn[931][11], s.dn[931][12], s.dn[931][13], s.dn[931][14], s.dn[931][15], s.dn[931][16], s.dn[931][17], s.dn[931][18], s.db[931][0], s.db[931][1], s.db[931][2], s.db[931][3], s.db[931][4], s.db[931][5], s.db[931][6], s.db[931][7], s.db[931][8], s.db[931][9], s.db[931][10], s.db[931][11], s.db[931][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 19] = [eq5_e1046_q_d_n0, eq5_e1046_q_d_n1, eq5_e1046_q_d_n2, eq5_e1046_q_d_n3, eq5_e1046_q_d_n4, eq5_e1046_q_d_n5, eq5_e1046_q_d_n6, eq5_e1046_q_d_n7, eq5_e1046_q_d_n8, eq5_e1046_q_d_n9, eq5_e1046_q_d_n10, eq5_e1046_q_d_n11, eq5_e1046_q_d_n12, eq5_e1046_q_d_n13, eq5_e1046_q_d_n14, eq5_e1046_q_d_n15, eq5_e1046_q_d_n16, eq5_e1046_q_d_n17, eq5_e1046_q_d_n18];
        let eq5_reactive_branch_derivatives: [f64; 13] = [eq5_e1046_q_d_b0, eq5_e1046_q_d_b1, eq5_e1046_q_d_b2, eq5_e1046_q_d_b3, eq5_e1046_q_d_b4, eq5_e1046_q_d_b5, eq5_e1046_q_d_b6, eq5_e1046_q_d_b7, eq5_e1046_q_d_b8, eq5_e1046_q_d_b9, eq5_e1046_q_d_b10, eq5_e1046_q_d_b11, eq5_e1046_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[18]),
            None,
            nodes,
            &eq5_reactive_node_derivatives,
            branches,
            &eq5_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e1092_q: f64 = s.v[66];
        let eq15_e1093: f64 = (p.p87 * s.v[66]);
        let eq15_e1093_d_n0: f64 = (p.p87 * s.dn[66][0]);
        let eq15_e1093_d_n1: f64 = (p.p87 * s.dn[66][1]);
        let eq15_e1093_d_n2: f64 = (p.p87 * s.dn[66][2]);
        let eq15_e1093_d_n3: f64 = (p.p87 * s.dn[66][3]);
        let eq15_e1093_d_n4: f64 = (p.p87 * s.dn[66][4]);
        let eq15_e1093_d_n5: f64 = (p.p87 * s.dn[66][5]);
        let eq15_e1093_d_n6: f64 = (p.p87 * s.dn[66][6]);
        let eq15_e1093_d_n7: f64 = (p.p87 * s.dn[66][7]);
        let eq15_e1093_d_n8: f64 = (p.p87 * s.dn[66][8]);
        let eq15_e1093_d_n9: f64 = (p.p87 * s.dn[66][9]);
        let eq15_e1093_d_n10: f64 = (p.p87 * s.dn[66][10]);
        let eq15_e1093_d_n11: f64 = (p.p87 * s.dn[66][11]);
        let eq15_e1093_d_n12: f64 = (p.p87 * s.dn[66][12]);
        let eq15_e1093_d_n13: f64 = (p.p87 * s.dn[66][13]);
        let eq15_e1093_d_n14: f64 = (p.p87 * s.dn[66][14]);
        let eq15_e1093_d_n15: f64 = (p.p87 * s.dn[66][15]);
        let eq15_e1093_d_n16: f64 = (p.p87 * s.dn[66][16]);
        let eq15_e1093_d_n17: f64 = (p.p87 * s.dn[66][17]);
        let eq15_e1093_d_n18: f64 = (p.p87 * s.dn[66][18]);
        let eq15_e1093_d_b0: f64 = (p.p87 * s.db[66][0]);
        let eq15_e1093_d_b1: f64 = (p.p87 * s.db[66][1]);
        let eq15_e1093_d_b2: f64 = (p.p87 * s.db[66][2]);
        let eq15_e1093_d_b3: f64 = (p.p87 * s.db[66][3]);
        let eq15_e1093_d_b4: f64 = (p.p87 * s.db[66][4]);
        let eq15_e1093_d_b5: f64 = (p.p87 * s.db[66][5]);
        let eq15_e1093_d_b6: f64 = (p.p87 * s.db[66][6]);
        let eq15_e1093_d_b7: f64 = (p.p87 * s.db[66][7]);
        let eq15_e1093_d_b8: f64 = (p.p87 * s.db[66][8]);
        let eq15_e1093_d_b9: f64 = (p.p87 * s.db[66][9]);
        let eq15_e1093_d_b10: f64 = (p.p87 * s.db[66][10]);
        let eq15_e1093_d_b11: f64 = (p.p87 * s.db[66][11]);
        let eq15_e1093_d_b12: f64 = (p.p87 * s.db[66][12]);
        let eq15_e1093_q: f64 = (p.p87 * eq15_e1092_q);
        let eq15_e1093_q_d_n0: f64 = (p.p87 * s.dn[66][0]);
        let eq15_e1093_q_d_n1: f64 = (p.p87 * s.dn[66][1]);
        let eq15_e1093_q_d_n2: f64 = (p.p87 * s.dn[66][2]);
        let eq15_e1093_q_d_n3: f64 = (p.p87 * s.dn[66][3]);
        let eq15_e1093_q_d_n4: f64 = (p.p87 * s.dn[66][4]);
        let eq15_e1093_q_d_n5: f64 = (p.p87 * s.dn[66][5]);
        let eq15_e1093_q_d_n6: f64 = (p.p87 * s.dn[66][6]);
        let eq15_e1093_q_d_n7: f64 = (p.p87 * s.dn[66][7]);
        let eq15_e1093_q_d_n8: f64 = (p.p87 * s.dn[66][8]);
        let eq15_e1093_q_d_n9: f64 = (p.p87 * s.dn[66][9]);
        let eq15_e1093_q_d_n10: f64 = (p.p87 * s.dn[66][10]);
        let eq15_e1093_q_d_n11: f64 = (p.p87 * s.dn[66][11]);
        let eq15_e1093_q_d_n12: f64 = (p.p87 * s.dn[66][12]);
        let eq15_e1093_q_d_n13: f64 = (p.p87 * s.dn[66][13]);
        let eq15_e1093_q_d_n14: f64 = (p.p87 * s.dn[66][14]);
        let eq15_e1093_q_d_n15: f64 = (p.p87 * s.dn[66][15]);
        let eq15_e1093_q_d_n16: f64 = (p.p87 * s.dn[66][16]);
        let eq15_e1093_q_d_n17: f64 = (p.p87 * s.dn[66][17]);
        let eq15_e1093_q_d_n18: f64 = (p.p87 * s.dn[66][18]);
        let eq15_e1093_q_d_b0: f64 = (p.p87 * s.db[66][0]);
        let eq15_e1093_q_d_b1: f64 = (p.p87 * s.db[66][1]);
        let eq15_e1093_q_d_b2: f64 = (p.p87 * s.db[66][2]);
        let eq15_e1093_q_d_b3: f64 = (p.p87 * s.db[66][3]);
        let eq15_e1093_q_d_b4: f64 = (p.p87 * s.db[66][4]);
        let eq15_e1093_q_d_b5: f64 = (p.p87 * s.db[66][5]);
        let eq15_e1093_q_d_b6: f64 = (p.p87 * s.db[66][6]);
        let eq15_e1093_q_d_b7: f64 = (p.p87 * s.db[66][7]);
        let eq15_e1093_q_d_b8: f64 = (p.p87 * s.db[66][8]);
        let eq15_e1093_q_d_b9: f64 = (p.p87 * s.db[66][9]);
        let eq15_e1093_q_d_b10: f64 = (p.p87 * s.db[66][10]);
        let eq15_e1093_q_d_b11: f64 = (p.p87 * s.db[66][11]);
        let eq15_e1093_q_d_b12: f64 = (p.p87 * s.db[66][12]);
        let eq15_reactive_node_derivatives: [f64; 19] = [eq15_e1093_q_d_n0, eq15_e1093_q_d_n1, eq15_e1093_q_d_n2, eq15_e1093_q_d_n3, eq15_e1093_q_d_n4, eq15_e1093_q_d_n5, eq15_e1093_q_d_n6, eq15_e1093_q_d_n7, eq15_e1093_q_d_n8, eq15_e1093_q_d_n9, eq15_e1093_q_d_n10, eq15_e1093_q_d_n11, eq15_e1093_q_d_n12, eq15_e1093_q_d_n13, eq15_e1093_q_d_n14, eq15_e1093_q_d_n15, eq15_e1093_q_d_n16, eq15_e1093_q_d_n17, eq15_e1093_q_d_n18];
        let eq15_reactive_branch_derivatives: [f64; 13] = [eq15_e1093_q_d_b0, eq15_e1093_q_d_b1, eq15_e1093_q_d_b2, eq15_e1093_q_d_b3, eq15_e1093_q_d_b4, eq15_e1093_q_d_b5, eq15_e1093_q_d_b6, eq15_e1093_q_d_b7, eq15_e1093_q_d_b8, eq15_e1093_q_d_b9, eq15_e1093_q_d_b10, eq15_e1093_q_d_b11, eq15_e1093_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[2]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e1096_q: f64 = s.v[65];
        let eq16_e1097: f64 = (p.p87 * s.v[65]);
        let eq16_e1097_d_n0: f64 = (p.p87 * s.dn[65][0]);
        let eq16_e1097_d_n1: f64 = (p.p87 * s.dn[65][1]);
        let eq16_e1097_d_n2: f64 = (p.p87 * s.dn[65][2]);
        let eq16_e1097_d_n3: f64 = (p.p87 * s.dn[65][3]);
        let eq16_e1097_d_n4: f64 = (p.p87 * s.dn[65][4]);
        let eq16_e1097_d_n5: f64 = (p.p87 * s.dn[65][5]);
        let eq16_e1097_d_n6: f64 = (p.p87 * s.dn[65][6]);
        let eq16_e1097_d_n7: f64 = (p.p87 * s.dn[65][7]);
        let eq16_e1097_d_n8: f64 = (p.p87 * s.dn[65][8]);
        let eq16_e1097_d_n9: f64 = (p.p87 * s.dn[65][9]);
        let eq16_e1097_d_n10: f64 = (p.p87 * s.dn[65][10]);
        let eq16_e1097_d_n11: f64 = (p.p87 * s.dn[65][11]);
        let eq16_e1097_d_n12: f64 = (p.p87 * s.dn[65][12]);
        let eq16_e1097_d_n13: f64 = (p.p87 * s.dn[65][13]);
        let eq16_e1097_d_n14: f64 = (p.p87 * s.dn[65][14]);
        let eq16_e1097_d_n15: f64 = (p.p87 * s.dn[65][15]);
        let eq16_e1097_d_n16: f64 = (p.p87 * s.dn[65][16]);
        let eq16_e1097_d_n17: f64 = (p.p87 * s.dn[65][17]);
        let eq16_e1097_d_n18: f64 = (p.p87 * s.dn[65][18]);
        let eq16_e1097_d_b0: f64 = (p.p87 * s.db[65][0]);
        let eq16_e1097_d_b1: f64 = (p.p87 * s.db[65][1]);
        let eq16_e1097_d_b2: f64 = (p.p87 * s.db[65][2]);
        let eq16_e1097_d_b3: f64 = (p.p87 * s.db[65][3]);
        let eq16_e1097_d_b4: f64 = (p.p87 * s.db[65][4]);
        let eq16_e1097_d_b5: f64 = (p.p87 * s.db[65][5]);
        let eq16_e1097_d_b6: f64 = (p.p87 * s.db[65][6]);
        let eq16_e1097_d_b7: f64 = (p.p87 * s.db[65][7]);
        let eq16_e1097_d_b8: f64 = (p.p87 * s.db[65][8]);
        let eq16_e1097_d_b9: f64 = (p.p87 * s.db[65][9]);
        let eq16_e1097_d_b10: f64 = (p.p87 * s.db[65][10]);
        let eq16_e1097_d_b11: f64 = (p.p87 * s.db[65][11]);
        let eq16_e1097_d_b12: f64 = (p.p87 * s.db[65][12]);
        let eq16_e1097_q: f64 = (p.p87 * eq16_e1096_q);
        let eq16_e1097_q_d_n0: f64 = (p.p87 * s.dn[65][0]);
        let eq16_e1097_q_d_n1: f64 = (p.p87 * s.dn[65][1]);
        let eq16_e1097_q_d_n2: f64 = (p.p87 * s.dn[65][2]);
        let eq16_e1097_q_d_n3: f64 = (p.p87 * s.dn[65][3]);
        let eq16_e1097_q_d_n4: f64 = (p.p87 * s.dn[65][4]);
        let eq16_e1097_q_d_n5: f64 = (p.p87 * s.dn[65][5]);
        let eq16_e1097_q_d_n6: f64 = (p.p87 * s.dn[65][6]);
        let eq16_e1097_q_d_n7: f64 = (p.p87 * s.dn[65][7]);
        let eq16_e1097_q_d_n8: f64 = (p.p87 * s.dn[65][8]);
        let eq16_e1097_q_d_n9: f64 = (p.p87 * s.dn[65][9]);
        let eq16_e1097_q_d_n10: f64 = (p.p87 * s.dn[65][10]);
        let eq16_e1097_q_d_n11: f64 = (p.p87 * s.dn[65][11]);
        let eq16_e1097_q_d_n12: f64 = (p.p87 * s.dn[65][12]);
        let eq16_e1097_q_d_n13: f64 = (p.p87 * s.dn[65][13]);
        let eq16_e1097_q_d_n14: f64 = (p.p87 * s.dn[65][14]);
        let eq16_e1097_q_d_n15: f64 = (p.p87 * s.dn[65][15]);
        let eq16_e1097_q_d_n16: f64 = (p.p87 * s.dn[65][16]);
        let eq16_e1097_q_d_n17: f64 = (p.p87 * s.dn[65][17]);
        let eq16_e1097_q_d_n18: f64 = (p.p87 * s.dn[65][18]);
        let eq16_e1097_q_d_b0: f64 = (p.p87 * s.db[65][0]);
        let eq16_e1097_q_d_b1: f64 = (p.p87 * s.db[65][1]);
        let eq16_e1097_q_d_b2: f64 = (p.p87 * s.db[65][2]);
        let eq16_e1097_q_d_b3: f64 = (p.p87 * s.db[65][3]);
        let eq16_e1097_q_d_b4: f64 = (p.p87 * s.db[65][4]);
        let eq16_e1097_q_d_b5: f64 = (p.p87 * s.db[65][5]);
        let eq16_e1097_q_d_b6: f64 = (p.p87 * s.db[65][6]);
        let eq16_e1097_q_d_b7: f64 = (p.p87 * s.db[65][7]);
        let eq16_e1097_q_d_b8: f64 = (p.p87 * s.db[65][8]);
        let eq16_e1097_q_d_b9: f64 = (p.p87 * s.db[65][9]);
        let eq16_e1097_q_d_b10: f64 = (p.p87 * s.db[65][10]);
        let eq16_e1097_q_d_b11: f64 = (p.p87 * s.db[65][11]);
        let eq16_e1097_q_d_b12: f64 = (p.p87 * s.db[65][12]);
        let eq16_reactive_node_derivatives: [f64; 19] = [eq16_e1097_q_d_n0, eq16_e1097_q_d_n1, eq16_e1097_q_d_n2, eq16_e1097_q_d_n3, eq16_e1097_q_d_n4, eq16_e1097_q_d_n5, eq16_e1097_q_d_n6, eq16_e1097_q_d_n7, eq16_e1097_q_d_n8, eq16_e1097_q_d_n9, eq16_e1097_q_d_n10, eq16_e1097_q_d_n11, eq16_e1097_q_d_n12, eq16_e1097_q_d_n13, eq16_e1097_q_d_n14, eq16_e1097_q_d_n15, eq16_e1097_q_d_n16, eq16_e1097_q_d_n17, eq16_e1097_q_d_n18];
        let eq16_reactive_branch_derivatives: [f64; 13] = [eq16_e1097_q_d_b0, eq16_e1097_q_d_b1, eq16_e1097_q_d_b2, eq16_e1097_q_d_b3, eq16_e1097_q_d_b4, eq16_e1097_q_d_b5, eq16_e1097_q_d_b6, eq16_e1097_q_d_b7, eq16_e1097_q_d_b8, eq16_e1097_q_d_b9, eq16_e1097_q_d_b10, eq16_e1097_q_d_b11, eq16_e1097_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1116, eq19_e1116_d_n0, eq19_e1116_d_n1, eq19_e1116_d_n2, eq19_e1116_d_n3, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n12, eq19_e1116_d_n13, eq19_e1116_d_n14, eq19_e1116_d_n15, eq19_e1116_d_n16, eq19_e1116_d_n17, eq19_e1116_d_n18, eq19_e1116_d_b0, eq19_e1116_d_b1, eq19_e1116_d_b2, eq19_e1116_d_b3, eq19_e1116_d_b4, eq19_e1116_d_b5, eq19_e1116_d_b6, eq19_e1116_d_b7, eq19_e1116_d_b8, eq19_e1116_d_b9, eq19_e1116_d_b10, eq19_e1116_d_b11, eq19_e1116_d_b12, eq19_e1116_q, eq19_e1116_q_d_n0, eq19_e1116_q_d_n1, eq19_e1116_q_d_n2, eq19_e1116_q_d_n3, eq19_e1116_q_d_n4, eq19_e1116_q_d_n5, eq19_e1116_q_d_n6, eq19_e1116_q_d_n7, eq19_e1116_q_d_n8, eq19_e1116_q_d_n9, eq19_e1116_q_d_n10, eq19_e1116_q_d_n11, eq19_e1116_q_d_n12, eq19_e1116_q_d_n13, eq19_e1116_q_d_n14, eq19_e1116_q_d_n15, eq19_e1116_q_d_n16, eq19_e1116_q_d_n17, eq19_e1116_q_d_n18, eq19_e1116_q_d_b0, eq19_e1116_q_d_b1, eq19_e1116_q_d_b2, eq19_e1116_q_d_b3, eq19_e1116_q_d_b4, eq19_e1116_q_d_b5, eq19_e1116_q_d_b6, eq19_e1116_q_d_b7, eq19_e1116_q_d_b8, eq19_e1116_q_d_b9, eq19_e1116_q_d_b10, eq19_e1116_q_d_b11, eq19_e1116_q_d_b12,) = {
    if s.b[3409] {
        let eq19_e1113_q: f64 = s.v[68];
        let eq19_e1114: f64 = (p.p87 * s.v[68]);
        let eq19_e1114_d_n0: f64 = (p.p87 * s.dn[68][0]);
        let eq19_e1114_d_n1: f64 = (p.p87 * s.dn[68][1]);
        let eq19_e1114_d_n2: f64 = (p.p87 * s.dn[68][2]);
        let eq19_e1114_d_n3: f64 = (p.p87 * s.dn[68][3]);
        let eq19_e1114_d_n4: f64 = (p.p87 * s.dn[68][4]);
        let eq19_e1114_d_n5: f64 = (p.p87 * s.dn[68][5]);
        let eq19_e1114_d_n6: f64 = (p.p87 * s.dn[68][6]);
        let eq19_e1114_d_n7: f64 = (p.p87 * s.dn[68][7]);
        let eq19_e1114_d_n8: f64 = (p.p87 * s.dn[68][8]);
        let eq19_e1114_d_n9: f64 = (p.p87 * s.dn[68][9]);
        let eq19_e1114_d_n10: f64 = (p.p87 * s.dn[68][10]);
        let eq19_e1114_d_n11: f64 = (p.p87 * s.dn[68][11]);
        let eq19_e1114_d_n12: f64 = (p.p87 * s.dn[68][12]);
        let eq19_e1114_d_n13: f64 = (p.p87 * s.dn[68][13]);
        let eq19_e1114_d_n14: f64 = (p.p87 * s.dn[68][14]);
        let eq19_e1114_d_n15: f64 = (p.p87 * s.dn[68][15]);
        let eq19_e1114_d_n16: f64 = (p.p87 * s.dn[68][16]);
        let eq19_e1114_d_n17: f64 = (p.p87 * s.dn[68][17]);
        let eq19_e1114_d_n18: f64 = (p.p87 * s.dn[68][18]);
        let eq19_e1114_d_b0: f64 = (p.p87 * s.db[68][0]);
        let eq19_e1114_d_b1: f64 = (p.p87 * s.db[68][1]);
        let eq19_e1114_d_b2: f64 = (p.p87 * s.db[68][2]);
        let eq19_e1114_d_b3: f64 = (p.p87 * s.db[68][3]);
        let eq19_e1114_d_b4: f64 = (p.p87 * s.db[68][4]);
        let eq19_e1114_d_b5: f64 = (p.p87 * s.db[68][5]);
        let eq19_e1114_d_b6: f64 = (p.p87 * s.db[68][6]);
        let eq19_e1114_d_b7: f64 = (p.p87 * s.db[68][7]);
        let eq19_e1114_d_b8: f64 = (p.p87 * s.db[68][8]);
        let eq19_e1114_d_b9: f64 = (p.p87 * s.db[68][9]);
        let eq19_e1114_d_b10: f64 = (p.p87 * s.db[68][10]);
        let eq19_e1114_d_b11: f64 = (p.p87 * s.db[68][11]);
        let eq19_e1114_d_b12: f64 = (p.p87 * s.db[68][12]);
        let eq19_e1114_q: f64 = (p.p87 * eq19_e1113_q);
        let eq19_e1114_q_d_n0: f64 = (p.p87 * s.dn[68][0]);
        let eq19_e1114_q_d_n1: f64 = (p.p87 * s.dn[68][1]);
        let eq19_e1114_q_d_n2: f64 = (p.p87 * s.dn[68][2]);
        let eq19_e1114_q_d_n3: f64 = (p.p87 * s.dn[68][3]);
        let eq19_e1114_q_d_n4: f64 = (p.p87 * s.dn[68][4]);
        let eq19_e1114_q_d_n5: f64 = (p.p87 * s.dn[68][5]);
        let eq19_e1114_q_d_n6: f64 = (p.p87 * s.dn[68][6]);
        let eq19_e1114_q_d_n7: f64 = (p.p87 * s.dn[68][7]);
        let eq19_e1114_q_d_n8: f64 = (p.p87 * s.dn[68][8]);
        let eq19_e1114_q_d_n9: f64 = (p.p87 * s.dn[68][9]);
        let eq19_e1114_q_d_n10: f64 = (p.p87 * s.dn[68][10]);
        let eq19_e1114_q_d_n11: f64 = (p.p87 * s.dn[68][11]);
        let eq19_e1114_q_d_n12: f64 = (p.p87 * s.dn[68][12]);
        let eq19_e1114_q_d_n13: f64 = (p.p87 * s.dn[68][13]);
        let eq19_e1114_q_d_n14: f64 = (p.p87 * s.dn[68][14]);
        let eq19_e1114_q_d_n15: f64 = (p.p87 * s.dn[68][15]);
        let eq19_e1114_q_d_n16: f64 = (p.p87 * s.dn[68][16]);
        let eq19_e1114_q_d_n17: f64 = (p.p87 * s.dn[68][17]);
        let eq19_e1114_q_d_n18: f64 = (p.p87 * s.dn[68][18]);
        let eq19_e1114_q_d_b0: f64 = (p.p87 * s.db[68][0]);
        let eq19_e1114_q_d_b1: f64 = (p.p87 * s.db[68][1]);
        let eq19_e1114_q_d_b2: f64 = (p.p87 * s.db[68][2]);
        let eq19_e1114_q_d_b3: f64 = (p.p87 * s.db[68][3]);
        let eq19_e1114_q_d_b4: f64 = (p.p87 * s.db[68][4]);
        let eq19_e1114_q_d_b5: f64 = (p.p87 * s.db[68][5]);
        let eq19_e1114_q_d_b6: f64 = (p.p87 * s.db[68][6]);
        let eq19_e1114_q_d_b7: f64 = (p.p87 * s.db[68][7]);
        let eq19_e1114_q_d_b8: f64 = (p.p87 * s.db[68][8]);
        let eq19_e1114_q_d_b9: f64 = (p.p87 * s.db[68][9]);
        let eq19_e1114_q_d_b10: f64 = (p.p87 * s.db[68][10]);
        let eq19_e1114_q_d_b11: f64 = (p.p87 * s.db[68][11]);
        let eq19_e1114_q_d_b12: f64 = (p.p87 * s.db[68][12]);
        (eq19_e1114, eq19_e1114_d_n0, eq19_e1114_d_n1, eq19_e1114_d_n2, eq19_e1114_d_n3, eq19_e1114_d_n4, eq19_e1114_d_n5, eq19_e1114_d_n6, eq19_e1114_d_n7, eq19_e1114_d_n8, eq19_e1114_d_n9, eq19_e1114_d_n10, eq19_e1114_d_n11, eq19_e1114_d_n12, eq19_e1114_d_n13, eq19_e1114_d_n14, eq19_e1114_d_n15, eq19_e1114_d_n16, eq19_e1114_d_n17, eq19_e1114_d_n18, eq19_e1114_d_b0, eq19_e1114_d_b1, eq19_e1114_d_b2, eq19_e1114_d_b3, eq19_e1114_d_b4, eq19_e1114_d_b5, eq19_e1114_d_b6, eq19_e1114_d_b7, eq19_e1114_d_b8, eq19_e1114_d_b9, eq19_e1114_d_b10, eq19_e1114_d_b11, eq19_e1114_d_b12, eq19_e1114_q, eq19_e1114_q_d_n0, eq19_e1114_q_d_n1, eq19_e1114_q_d_n2, eq19_e1114_q_d_n3, eq19_e1114_q_d_n4, eq19_e1114_q_d_n5, eq19_e1114_q_d_n6, eq19_e1114_q_d_n7, eq19_e1114_q_d_n8, eq19_e1114_q_d_n9, eq19_e1114_q_d_n10, eq19_e1114_q_d_n11, eq19_e1114_q_d_n12, eq19_e1114_q_d_n13, eq19_e1114_q_d_n14, eq19_e1114_q_d_n15, eq19_e1114_q_d_n16, eq19_e1114_q_d_n17, eq19_e1114_q_d_n18, eq19_e1114_q_d_b0, eq19_e1114_q_d_b1, eq19_e1114_q_d_b2, eq19_e1114_q_d_b3, eq19_e1114_q_d_b4, eq19_e1114_q_d_b5, eq19_e1114_q_d_b6, eq19_e1114_q_d_b7, eq19_e1114_q_d_b8, eq19_e1114_q_d_b9, eq19_e1114_q_d_b10, eq19_e1114_q_d_b11, eq19_e1114_q_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e1116_q_d_n0, eq19_e1116_q_d_n1, eq19_e1116_q_d_n2, eq19_e1116_q_d_n3, eq19_e1116_q_d_n4, eq19_e1116_q_d_n5, eq19_e1116_q_d_n6, eq19_e1116_q_d_n7, eq19_e1116_q_d_n8, eq19_e1116_q_d_n9, eq19_e1116_q_d_n10, eq19_e1116_q_d_n11, eq19_e1116_q_d_n12, eq19_e1116_q_d_n13, eq19_e1116_q_d_n14, eq19_e1116_q_d_n15, eq19_e1116_q_d_n16, eq19_e1116_q_d_n17, eq19_e1116_q_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 13] = [eq19_e1116_q_d_b0, eq19_e1116_q_d_b1, eq19_e1116_q_d_b2, eq19_e1116_q_d_b3, eq19_e1116_q_d_b4, eq19_e1116_q_d_b5, eq19_e1116_q_d_b6, eq19_e1116_q_d_b7, eq19_e1116_q_d_b8, eq19_e1116_q_d_b9, eq19_e1116_q_d_b10, eq19_e1116_q_d_b11, eq19_e1116_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n1, eq20_e1123_d_n2, eq20_e1123_d_n3, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n12, eq20_e1123_d_n13, eq20_e1123_d_n14, eq20_e1123_d_n15, eq20_e1123_d_n16, eq20_e1123_d_n17, eq20_e1123_d_n18, eq20_e1123_d_b0, eq20_e1123_d_b1, eq20_e1123_d_b2, eq20_e1123_d_b3, eq20_e1123_d_b4, eq20_e1123_d_b5, eq20_e1123_d_b6, eq20_e1123_d_b7, eq20_e1123_d_b8, eq20_e1123_d_b9, eq20_e1123_d_b10, eq20_e1123_d_b11, eq20_e1123_d_b12, eq20_e1123_q, eq20_e1123_q_d_n0, eq20_e1123_q_d_n1, eq20_e1123_q_d_n2, eq20_e1123_q_d_n3, eq20_e1123_q_d_n4, eq20_e1123_q_d_n5, eq20_e1123_q_d_n6, eq20_e1123_q_d_n7, eq20_e1123_q_d_n8, eq20_e1123_q_d_n9, eq20_e1123_q_d_n10, eq20_e1123_q_d_n11, eq20_e1123_q_d_n12, eq20_e1123_q_d_n13, eq20_e1123_q_d_n14, eq20_e1123_q_d_n15, eq20_e1123_q_d_n16, eq20_e1123_q_d_n17, eq20_e1123_q_d_n18, eq20_e1123_q_d_b0, eq20_e1123_q_d_b1, eq20_e1123_q_d_b2, eq20_e1123_q_d_b3, eq20_e1123_q_d_b4, eq20_e1123_q_d_b5, eq20_e1123_q_d_b6, eq20_e1123_q_d_b7, eq20_e1123_q_d_b8, eq20_e1123_q_d_b9, eq20_e1123_q_d_b10, eq20_e1123_q_d_b11, eq20_e1123_q_d_b12,) = {
    if s.b[3409] {
        let eq20_e1120_q: f64 = s.v[67];
        let eq20_e1121: f64 = (p.p87 * s.v[67]);
        let eq20_e1121_d_n0: f64 = (p.p87 * s.dn[67][0]);
        let eq20_e1121_d_n1: f64 = (p.p87 * s.dn[67][1]);
        let eq20_e1121_d_n2: f64 = (p.p87 * s.dn[67][2]);
        let eq20_e1121_d_n3: f64 = (p.p87 * s.dn[67][3]);
        let eq20_e1121_d_n4: f64 = (p.p87 * s.dn[67][4]);
        let eq20_e1121_d_n5: f64 = (p.p87 * s.dn[67][5]);
        let eq20_e1121_d_n6: f64 = (p.p87 * s.dn[67][6]);
        let eq20_e1121_d_n7: f64 = (p.p87 * s.dn[67][7]);
        let eq20_e1121_d_n8: f64 = (p.p87 * s.dn[67][8]);
        let eq20_e1121_d_n9: f64 = (p.p87 * s.dn[67][9]);
        let eq20_e1121_d_n10: f64 = (p.p87 * s.dn[67][10]);
        let eq20_e1121_d_n11: f64 = (p.p87 * s.dn[67][11]);
        let eq20_e1121_d_n12: f64 = (p.p87 * s.dn[67][12]);
        let eq20_e1121_d_n13: f64 = (p.p87 * s.dn[67][13]);
        let eq20_e1121_d_n14: f64 = (p.p87 * s.dn[67][14]);
        let eq20_e1121_d_n15: f64 = (p.p87 * s.dn[67][15]);
        let eq20_e1121_d_n16: f64 = (p.p87 * s.dn[67][16]);
        let eq20_e1121_d_n17: f64 = (p.p87 * s.dn[67][17]);
        let eq20_e1121_d_n18: f64 = (p.p87 * s.dn[67][18]);
        let eq20_e1121_d_b0: f64 = (p.p87 * s.db[67][0]);
        let eq20_e1121_d_b1: f64 = (p.p87 * s.db[67][1]);
        let eq20_e1121_d_b2: f64 = (p.p87 * s.db[67][2]);
        let eq20_e1121_d_b3: f64 = (p.p87 * s.db[67][3]);
        let eq20_e1121_d_b4: f64 = (p.p87 * s.db[67][4]);
        let eq20_e1121_d_b5: f64 = (p.p87 * s.db[67][5]);
        let eq20_e1121_d_b6: f64 = (p.p87 * s.db[67][6]);
        let eq20_e1121_d_b7: f64 = (p.p87 * s.db[67][7]);
        let eq20_e1121_d_b8: f64 = (p.p87 * s.db[67][8]);
        let eq20_e1121_d_b9: f64 = (p.p87 * s.db[67][9]);
        let eq20_e1121_d_b10: f64 = (p.p87 * s.db[67][10]);
        let eq20_e1121_d_b11: f64 = (p.p87 * s.db[67][11]);
        let eq20_e1121_d_b12: f64 = (p.p87 * s.db[67][12]);
        let eq20_e1121_q: f64 = (p.p87 * eq20_e1120_q);
        let eq20_e1121_q_d_n0: f64 = (p.p87 * s.dn[67][0]);
        let eq20_e1121_q_d_n1: f64 = (p.p87 * s.dn[67][1]);
        let eq20_e1121_q_d_n2: f64 = (p.p87 * s.dn[67][2]);
        let eq20_e1121_q_d_n3: f64 = (p.p87 * s.dn[67][3]);
        let eq20_e1121_q_d_n4: f64 = (p.p87 * s.dn[67][4]);
        let eq20_e1121_q_d_n5: f64 = (p.p87 * s.dn[67][5]);
        let eq20_e1121_q_d_n6: f64 = (p.p87 * s.dn[67][6]);
        let eq20_e1121_q_d_n7: f64 = (p.p87 * s.dn[67][7]);
        let eq20_e1121_q_d_n8: f64 = (p.p87 * s.dn[67][8]);
        let eq20_e1121_q_d_n9: f64 = (p.p87 * s.dn[67][9]);
        let eq20_e1121_q_d_n10: f64 = (p.p87 * s.dn[67][10]);
        let eq20_e1121_q_d_n11: f64 = (p.p87 * s.dn[67][11]);
        let eq20_e1121_q_d_n12: f64 = (p.p87 * s.dn[67][12]);
        let eq20_e1121_q_d_n13: f64 = (p.p87 * s.dn[67][13]);
        let eq20_e1121_q_d_n14: f64 = (p.p87 * s.dn[67][14]);
        let eq20_e1121_q_d_n15: f64 = (p.p87 * s.dn[67][15]);
        let eq20_e1121_q_d_n16: f64 = (p.p87 * s.dn[67][16]);
        let eq20_e1121_q_d_n17: f64 = (p.p87 * s.dn[67][17]);
        let eq20_e1121_q_d_n18: f64 = (p.p87 * s.dn[67][18]);
        let eq20_e1121_q_d_b0: f64 = (p.p87 * s.db[67][0]);
        let eq20_e1121_q_d_b1: f64 = (p.p87 * s.db[67][1]);
        let eq20_e1121_q_d_b2: f64 = (p.p87 * s.db[67][2]);
        let eq20_e1121_q_d_b3: f64 = (p.p87 * s.db[67][3]);
        let eq20_e1121_q_d_b4: f64 = (p.p87 * s.db[67][4]);
        let eq20_e1121_q_d_b5: f64 = (p.p87 * s.db[67][5]);
        let eq20_e1121_q_d_b6: f64 = (p.p87 * s.db[67][6]);
        let eq20_e1121_q_d_b7: f64 = (p.p87 * s.db[67][7]);
        let eq20_e1121_q_d_b8: f64 = (p.p87 * s.db[67][8]);
        let eq20_e1121_q_d_b9: f64 = (p.p87 * s.db[67][9]);
        let eq20_e1121_q_d_b10: f64 = (p.p87 * s.db[67][10]);
        let eq20_e1121_q_d_b11: f64 = (p.p87 * s.db[67][11]);
        let eq20_e1121_q_d_b12: f64 = (p.p87 * s.db[67][12]);
        (eq20_e1121, eq20_e1121_d_n0, eq20_e1121_d_n1, eq20_e1121_d_n2, eq20_e1121_d_n3, eq20_e1121_d_n4, eq20_e1121_d_n5, eq20_e1121_d_n6, eq20_e1121_d_n7, eq20_e1121_d_n8, eq20_e1121_d_n9, eq20_e1121_d_n10, eq20_e1121_d_n11, eq20_e1121_d_n12, eq20_e1121_d_n13, eq20_e1121_d_n14, eq20_e1121_d_n15, eq20_e1121_d_n16, eq20_e1121_d_n17, eq20_e1121_d_n18, eq20_e1121_d_b0, eq20_e1121_d_b1, eq20_e1121_d_b2, eq20_e1121_d_b3, eq20_e1121_d_b4, eq20_e1121_d_b5, eq20_e1121_d_b6, eq20_e1121_d_b7, eq20_e1121_d_b8, eq20_e1121_d_b9, eq20_e1121_d_b10, eq20_e1121_d_b11, eq20_e1121_d_b12, eq20_e1121_q, eq20_e1121_q_d_n0, eq20_e1121_q_d_n1, eq20_e1121_q_d_n2, eq20_e1121_q_d_n3, eq20_e1121_q_d_n4, eq20_e1121_q_d_n5, eq20_e1121_q_d_n6, eq20_e1121_q_d_n7, eq20_e1121_q_d_n8, eq20_e1121_q_d_n9, eq20_e1121_q_d_n10, eq20_e1121_q_d_n11, eq20_e1121_q_d_n12, eq20_e1121_q_d_n13, eq20_e1121_q_d_n14, eq20_e1121_q_d_n15, eq20_e1121_q_d_n16, eq20_e1121_q_d_n17, eq20_e1121_q_d_n18, eq20_e1121_q_d_b0, eq20_e1121_q_d_b1, eq20_e1121_q_d_b2, eq20_e1121_q_d_b3, eq20_e1121_q_d_b4, eq20_e1121_q_d_b5, eq20_e1121_q_d_b6, eq20_e1121_q_d_b7, eq20_e1121_q_d_b8, eq20_e1121_q_d_b9, eq20_e1121_q_d_b10, eq20_e1121_q_d_b11, eq20_e1121_q_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e1123_q_d_n0, eq20_e1123_q_d_n1, eq20_e1123_q_d_n2, eq20_e1123_q_d_n3, eq20_e1123_q_d_n4, eq20_e1123_q_d_n5, eq20_e1123_q_d_n6, eq20_e1123_q_d_n7, eq20_e1123_q_d_n8, eq20_e1123_q_d_n9, eq20_e1123_q_d_n10, eq20_e1123_q_d_n11, eq20_e1123_q_d_n12, eq20_e1123_q_d_n13, eq20_e1123_q_d_n14, eq20_e1123_q_d_n15, eq20_e1123_q_d_n16, eq20_e1123_q_d_n17, eq20_e1123_q_d_n18];
        let eq20_reactive_branch_derivatives: [f64; 13] = [eq20_e1123_q_d_b0, eq20_e1123_q_d_b1, eq20_e1123_q_d_b2, eq20_e1123_q_d_b3, eq20_e1123_q_d_b4, eq20_e1123_q_d_b5, eq20_e1123_q_d_b6, eq20_e1123_q_d_b7, eq20_e1123_q_d_b8, eq20_e1123_q_d_b9, eq20_e1123_q_d_b10, eq20_e1123_q_d_b11, eq20_e1123_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq28_e1167: f64 = (s.v[18] + s.v[753]);
        let eq28_e1167_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);
        let eq28_e1167_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);
        let eq28_e1167_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);
        let eq28_e1167_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);
        let eq28_e1167_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);
        let eq28_e1167_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);
        let eq28_e1167_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);
        let eq28_e1167_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);
        let eq28_e1167_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);
        let eq28_e1167_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);
        let eq28_e1167_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);
        let eq28_e1167_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);
        let eq28_e1167_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);
        let eq28_e1167_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);
        let eq28_e1167_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);
        let eq28_e1167_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);
        let eq28_e1167_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);
        let eq28_e1167_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);
        let eq28_e1167_d_n18: f64 = (s.dn[18][18] + s.dn[753][18]);
        let eq28_e1167_d_b0: f64 = (s.db[18][0] + s.db[753][0]);
        let eq28_e1167_d_b1: f64 = (s.db[18][1] + s.db[753][1]);
        let eq28_e1167_d_b2: f64 = (s.db[18][2] + s.db[753][2]);
        let eq28_e1167_d_b3: f64 = (s.db[18][3] + s.db[753][3]);
        let eq28_e1167_d_b4: f64 = (s.db[18][4] + s.db[753][4]);
        let eq28_e1167_d_b5: f64 = (s.db[18][5] + s.db[753][5]);
        let eq28_e1167_d_b6: f64 = (s.db[18][6] + s.db[753][6]);
        let eq28_e1167_d_b7: f64 = (s.db[18][7] + s.db[753][7]);
        let eq28_e1167_d_b8: f64 = (s.db[18][8] + s.db[753][8]);
        let eq28_e1167_d_b9: f64 = (s.db[18][9] + s.db[753][9]);
        let eq28_e1167_d_b10: f64 = (s.db[18][10] + s.db[753][10]);
        let eq28_e1167_d_b11: f64 = (s.db[18][11] + s.db[753][11]);
        let eq28_e1167_d_b12: f64 = (s.db[18][12] + s.db[753][12]);
        let eq28_e1168_q: f64 = eq28_e1167;
        let eq28_e1169: f64 = (p.p87 * eq28_e1167);
        let eq28_e1169_d_n0: f64 = (p.p87 * eq28_e1167_d_n0);
        let eq28_e1169_d_n1: f64 = (p.p87 * eq28_e1167_d_n1);
        let eq28_e1169_d_n2: f64 = (p.p87 * eq28_e1167_d_n2);
        let eq28_e1169_d_n3: f64 = (p.p87 * eq28_e1167_d_n3);
        let eq28_e1169_d_n4: f64 = (p.p87 * eq28_e1167_d_n4);
        let eq28_e1169_d_n5: f64 = (p.p87 * eq28_e1167_d_n5);
        let eq28_e1169_d_n6: f64 = (p.p87 * eq28_e1167_d_n6);
        let eq28_e1169_d_n7: f64 = (p.p87 * eq28_e1167_d_n7);
        let eq28_e1169_d_n8: f64 = (p.p87 * eq28_e1167_d_n8);
        let eq28_e1169_d_n9: f64 = (p.p87 * eq28_e1167_d_n9);
        let eq28_e1169_d_n10: f64 = (p.p87 * eq28_e1167_d_n10);
        let eq28_e1169_d_n11: f64 = (p.p87 * eq28_e1167_d_n11);
        let eq28_e1169_d_n12: f64 = (p.p87 * eq28_e1167_d_n12);
        let eq28_e1169_d_n13: f64 = (p.p87 * eq28_e1167_d_n13);
        let eq28_e1169_d_n14: f64 = (p.p87 * eq28_e1167_d_n14);
        let eq28_e1169_d_n15: f64 = (p.p87 * eq28_e1167_d_n15);
        let eq28_e1169_d_n16: f64 = (p.p87 * eq28_e1167_d_n16);
        let eq28_e1169_d_n17: f64 = (p.p87 * eq28_e1167_d_n17);
        let eq28_e1169_d_n18: f64 = (p.p87 * eq28_e1167_d_n18);
        let eq28_e1169_d_b0: f64 = (p.p87 * eq28_e1167_d_b0);
        let eq28_e1169_d_b1: f64 = (p.p87 * eq28_e1167_d_b1);
        let eq28_e1169_d_b2: f64 = (p.p87 * eq28_e1167_d_b2);
        let eq28_e1169_d_b3: f64 = (p.p87 * eq28_e1167_d_b3);
        let eq28_e1169_d_b4: f64 = (p.p87 * eq28_e1167_d_b4);
        let eq28_e1169_d_b5: f64 = (p.p87 * eq28_e1167_d_b5);
        let eq28_e1169_d_b6: f64 = (p.p87 * eq28_e1167_d_b6);
        let eq28_e1169_d_b7: f64 = (p.p87 * eq28_e1167_d_b7);
        let eq28_e1169_d_b8: f64 = (p.p87 * eq28_e1167_d_b8);
        let eq28_e1169_d_b9: f64 = (p.p87 * eq28_e1167_d_b9);
        let eq28_e1169_d_b10: f64 = (p.p87 * eq28_e1167_d_b10);
        let eq28_e1169_d_b11: f64 = (p.p87 * eq28_e1167_d_b11);
        let eq28_e1169_d_b12: f64 = (p.p87 * eq28_e1167_d_b12);
        let eq28_e1169_q: f64 = (p.p87 * eq28_e1168_q);
        let eq28_e1169_q_d_n0: f64 = (p.p87 * eq28_e1167_d_n0);
        let eq28_e1169_q_d_n1: f64 = (p.p87 * eq28_e1167_d_n1);
        let eq28_e1169_q_d_n2: f64 = (p.p87 * eq28_e1167_d_n2);
        let eq28_e1169_q_d_n3: f64 = (p.p87 * eq28_e1167_d_n3);
        let eq28_e1169_q_d_n4: f64 = (p.p87 * eq28_e1167_d_n4);
        let eq28_e1169_q_d_n5: f64 = (p.p87 * eq28_e1167_d_n5);
        let eq28_e1169_q_d_n6: f64 = (p.p87 * eq28_e1167_d_n6);
        let eq28_e1169_q_d_n7: f64 = (p.p87 * eq28_e1167_d_n7);
        let eq28_e1169_q_d_n8: f64 = (p.p87 * eq28_e1167_d_n8);
        let eq28_e1169_q_d_n9: f64 = (p.p87 * eq28_e1167_d_n9);
        let eq28_e1169_q_d_n10: f64 = (p.p87 * eq28_e1167_d_n10);
        let eq28_e1169_q_d_n11: f64 = (p.p87 * eq28_e1167_d_n11);
        let eq28_e1169_q_d_n12: f64 = (p.p87 * eq28_e1167_d_n12);
        let eq28_e1169_q_d_n13: f64 = (p.p87 * eq28_e1167_d_n13);
        let eq28_e1169_q_d_n14: f64 = (p.p87 * eq28_e1167_d_n14);
        let eq28_e1169_q_d_n15: f64 = (p.p87 * eq28_e1167_d_n15);
        let eq28_e1169_q_d_n16: f64 = (p.p87 * eq28_e1167_d_n16);
        let eq28_e1169_q_d_n17: f64 = (p.p87 * eq28_e1167_d_n17);
        let eq28_e1169_q_d_n18: f64 = (p.p87 * eq28_e1167_d_n18);
        let eq28_e1169_q_d_b0: f64 = (p.p87 * eq28_e1167_d_b0);
        let eq28_e1169_q_d_b1: f64 = (p.p87 * eq28_e1167_d_b1);
        let eq28_e1169_q_d_b2: f64 = (p.p87 * eq28_e1167_d_b2);
        let eq28_e1169_q_d_b3: f64 = (p.p87 * eq28_e1167_d_b3);
        let eq28_e1169_q_d_b4: f64 = (p.p87 * eq28_e1167_d_b4);
        let eq28_e1169_q_d_b5: f64 = (p.p87 * eq28_e1167_d_b5);
        let eq28_e1169_q_d_b6: f64 = (p.p87 * eq28_e1167_d_b6);
        let eq28_e1169_q_d_b7: f64 = (p.p87 * eq28_e1167_d_b7);
        let eq28_e1169_q_d_b8: f64 = (p.p87 * eq28_e1167_d_b8);
        let eq28_e1169_q_d_b9: f64 = (p.p87 * eq28_e1167_d_b9);
        let eq28_e1169_q_d_b10: f64 = (p.p87 * eq28_e1167_d_b10);
        let eq28_e1169_q_d_b11: f64 = (p.p87 * eq28_e1167_d_b11);
        let eq28_e1169_q_d_b12: f64 = (p.p87 * eq28_e1167_d_b12);
        let eq28_reactive_node_derivatives: [f64; 19] = [eq28_e1169_q_d_n0, eq28_e1169_q_d_n1, eq28_e1169_q_d_n2, eq28_e1169_q_d_n3, eq28_e1169_q_d_n4, eq28_e1169_q_d_n5, eq28_e1169_q_d_n6, eq28_e1169_q_d_n7, eq28_e1169_q_d_n8, eq28_e1169_q_d_n9, eq28_e1169_q_d_n10, eq28_e1169_q_d_n11, eq28_e1169_q_d_n12, eq28_e1169_q_d_n13, eq28_e1169_q_d_n14, eq28_e1169_q_d_n15, eq28_e1169_q_d_n16, eq28_e1169_q_d_n17, eq28_e1169_q_d_n18];
        let eq28_reactive_branch_derivatives: [f64; 13] = [eq28_e1169_q_d_b0, eq28_e1169_q_d_b1, eq28_e1169_q_d_b2, eq28_e1169_q_d_b3, eq28_e1169_q_d_b4, eq28_e1169_q_d_b5, eq28_e1169_q_d_b6, eq28_e1169_q_d_b7, eq28_e1169_q_d_b8, eq28_e1169_q_d_b9, eq28_e1169_q_d_b10, eq28_e1169_q_d_b11, eq28_e1169_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e1173: f64 = (s.v[19] + s.v[751]);
        let eq29_e1173_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);
        let eq29_e1173_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);
        let eq29_e1173_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);
        let eq29_e1173_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);
        let eq29_e1173_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);
        let eq29_e1173_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);
        let eq29_e1173_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);
        let eq29_e1173_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);
        let eq29_e1173_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);
        let eq29_e1173_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);
        let eq29_e1173_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);
        let eq29_e1173_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);
        let eq29_e1173_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);
        let eq29_e1173_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);
        let eq29_e1173_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);
        let eq29_e1173_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);
        let eq29_e1173_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);
        let eq29_e1173_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);
        let eq29_e1173_d_n18: f64 = (s.dn[19][18] + s.dn[751][18]);
        let eq29_e1173_d_b0: f64 = (s.db[19][0] + s.db[751][0]);
        let eq29_e1173_d_b1: f64 = (s.db[19][1] + s.db[751][1]);
        let eq29_e1173_d_b2: f64 = (s.db[19][2] + s.db[751][2]);
        let eq29_e1173_d_b3: f64 = (s.db[19][3] + s.db[751][3]);
        let eq29_e1173_d_b4: f64 = (s.db[19][4] + s.db[751][4]);
        let eq29_e1173_d_b5: f64 = (s.db[19][5] + s.db[751][5]);
        let eq29_e1173_d_b6: f64 = (s.db[19][6] + s.db[751][6]);
        let eq29_e1173_d_b7: f64 = (s.db[19][7] + s.db[751][7]);
        let eq29_e1173_d_b8: f64 = (s.db[19][8] + s.db[751][8]);
        let eq29_e1173_d_b9: f64 = (s.db[19][9] + s.db[751][9]);
        let eq29_e1173_d_b10: f64 = (s.db[19][10] + s.db[751][10]);
        let eq29_e1173_d_b11: f64 = (s.db[19][11] + s.db[751][11]);
        let eq29_e1173_d_b12: f64 = (s.db[19][12] + s.db[751][12]);
        let eq29_e1174_q: f64 = eq29_e1173;
        let eq29_e1175: f64 = (p.p87 * eq29_e1173);
        let eq29_e1175_d_n0: f64 = (p.p87 * eq29_e1173_d_n0);
        let eq29_e1175_d_n1: f64 = (p.p87 * eq29_e1173_d_n1);
        let eq29_e1175_d_n2: f64 = (p.p87 * eq29_e1173_d_n2);
        let eq29_e1175_d_n3: f64 = (p.p87 * eq29_e1173_d_n3);
        let eq29_e1175_d_n4: f64 = (p.p87 * eq29_e1173_d_n4);
        let eq29_e1175_d_n5: f64 = (p.p87 * eq29_e1173_d_n5);
        let eq29_e1175_d_n6: f64 = (p.p87 * eq29_e1173_d_n6);
        let eq29_e1175_d_n7: f64 = (p.p87 * eq29_e1173_d_n7);
        let eq29_e1175_d_n8: f64 = (p.p87 * eq29_e1173_d_n8);
        let eq29_e1175_d_n9: f64 = (p.p87 * eq29_e1173_d_n9);
        let eq29_e1175_d_n10: f64 = (p.p87 * eq29_e1173_d_n10);
        let eq29_e1175_d_n11: f64 = (p.p87 * eq29_e1173_d_n11);
        let eq29_e1175_d_n12: f64 = (p.p87 * eq29_e1173_d_n12);
        let eq29_e1175_d_n13: f64 = (p.p87 * eq29_e1173_d_n13);
        let eq29_e1175_d_n14: f64 = (p.p87 * eq29_e1173_d_n14);
        let eq29_e1175_d_n15: f64 = (p.p87 * eq29_e1173_d_n15);
        let eq29_e1175_d_n16: f64 = (p.p87 * eq29_e1173_d_n16);
        let eq29_e1175_d_n17: f64 = (p.p87 * eq29_e1173_d_n17);
        let eq29_e1175_d_n18: f64 = (p.p87 * eq29_e1173_d_n18);
        let eq29_e1175_d_b0: f64 = (p.p87 * eq29_e1173_d_b0);
        let eq29_e1175_d_b1: f64 = (p.p87 * eq29_e1173_d_b1);
        let eq29_e1175_d_b2: f64 = (p.p87 * eq29_e1173_d_b2);
        let eq29_e1175_d_b3: f64 = (p.p87 * eq29_e1173_d_b3);
        let eq29_e1175_d_b4: f64 = (p.p87 * eq29_e1173_d_b4);
        let eq29_e1175_d_b5: f64 = (p.p87 * eq29_e1173_d_b5);
        let eq29_e1175_d_b6: f64 = (p.p87 * eq29_e1173_d_b6);
        let eq29_e1175_d_b7: f64 = (p.p87 * eq29_e1173_d_b7);
        let eq29_e1175_d_b8: f64 = (p.p87 * eq29_e1173_d_b8);
        let eq29_e1175_d_b9: f64 = (p.p87 * eq29_e1173_d_b9);
        let eq29_e1175_d_b10: f64 = (p.p87 * eq29_e1173_d_b10);
        let eq29_e1175_d_b11: f64 = (p.p87 * eq29_e1173_d_b11);
        let eq29_e1175_d_b12: f64 = (p.p87 * eq29_e1173_d_b12);
        let eq29_e1175_q: f64 = (p.p87 * eq29_e1174_q);
        let eq29_e1175_q_d_n0: f64 = (p.p87 * eq29_e1173_d_n0);
        let eq29_e1175_q_d_n1: f64 = (p.p87 * eq29_e1173_d_n1);
        let eq29_e1175_q_d_n2: f64 = (p.p87 * eq29_e1173_d_n2);
        let eq29_e1175_q_d_n3: f64 = (p.p87 * eq29_e1173_d_n3);
        let eq29_e1175_q_d_n4: f64 = (p.p87 * eq29_e1173_d_n4);
        let eq29_e1175_q_d_n5: f64 = (p.p87 * eq29_e1173_d_n5);
        let eq29_e1175_q_d_n6: f64 = (p.p87 * eq29_e1173_d_n6);
        let eq29_e1175_q_d_n7: f64 = (p.p87 * eq29_e1173_d_n7);
        let eq29_e1175_q_d_n8: f64 = (p.p87 * eq29_e1173_d_n8);
        let eq29_e1175_q_d_n9: f64 = (p.p87 * eq29_e1173_d_n9);
        let eq29_e1175_q_d_n10: f64 = (p.p87 * eq29_e1173_d_n10);
        let eq29_e1175_q_d_n11: f64 = (p.p87 * eq29_e1173_d_n11);
        let eq29_e1175_q_d_n12: f64 = (p.p87 * eq29_e1173_d_n12);
        let eq29_e1175_q_d_n13: f64 = (p.p87 * eq29_e1173_d_n13);
        let eq29_e1175_q_d_n14: f64 = (p.p87 * eq29_e1173_d_n14);
        let eq29_e1175_q_d_n15: f64 = (p.p87 * eq29_e1173_d_n15);
        let eq29_e1175_q_d_n16: f64 = (p.p87 * eq29_e1173_d_n16);
        let eq29_e1175_q_d_n17: f64 = (p.p87 * eq29_e1173_d_n17);
        let eq29_e1175_q_d_n18: f64 = (p.p87 * eq29_e1173_d_n18);
        let eq29_e1175_q_d_b0: f64 = (p.p87 * eq29_e1173_d_b0);
        let eq29_e1175_q_d_b1: f64 = (p.p87 * eq29_e1173_d_b1);
        let eq29_e1175_q_d_b2: f64 = (p.p87 * eq29_e1173_d_b2);
        let eq29_e1175_q_d_b3: f64 = (p.p87 * eq29_e1173_d_b3);
        let eq29_e1175_q_d_b4: f64 = (p.p87 * eq29_e1173_d_b4);
        let eq29_e1175_q_d_b5: f64 = (p.p87 * eq29_e1173_d_b5);
        let eq29_e1175_q_d_b6: f64 = (p.p87 * eq29_e1173_d_b6);
        let eq29_e1175_q_d_b7: f64 = (p.p87 * eq29_e1173_d_b7);
        let eq29_e1175_q_d_b8: f64 = (p.p87 * eq29_e1173_d_b8);
        let eq29_e1175_q_d_b9: f64 = (p.p87 * eq29_e1173_d_b9);
        let eq29_e1175_q_d_b10: f64 = (p.p87 * eq29_e1173_d_b10);
        let eq29_e1175_q_d_b11: f64 = (p.p87 * eq29_e1173_d_b11);
        let eq29_e1175_q_d_b12: f64 = (p.p87 * eq29_e1173_d_b12);
        let eq29_reactive_node_derivatives: [f64; 19] = [eq29_e1175_q_d_n0, eq29_e1175_q_d_n1, eq29_e1175_q_d_n2, eq29_e1175_q_d_n3, eq29_e1175_q_d_n4, eq29_e1175_q_d_n5, eq29_e1175_q_d_n6, eq29_e1175_q_d_n7, eq29_e1175_q_d_n8, eq29_e1175_q_d_n9, eq29_e1175_q_d_n10, eq29_e1175_q_d_n11, eq29_e1175_q_d_n12, eq29_e1175_q_d_n13, eq29_e1175_q_d_n14, eq29_e1175_q_d_n15, eq29_e1175_q_d_n16, eq29_e1175_q_d_n17, eq29_e1175_q_d_n18];
        let eq29_reactive_branch_derivatives: [f64; 13] = [eq29_e1175_q_d_b0, eq29_e1175_q_d_b1, eq29_e1175_q_d_b2, eq29_e1175_q_d_b3, eq29_e1175_q_d_b4, eq29_e1175_q_d_b5, eq29_e1175_q_d_b6, eq29_e1175_q_d_b7, eq29_e1175_q_d_b8, eq29_e1175_q_d_b9, eq29_e1175_q_d_b10, eq29_e1175_q_d_b11, eq29_e1175_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e1180: f64 = (s.v[753] + s.v[751]);
        let eq30_e1180_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);
        let eq30_e1180_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);
        let eq30_e1180_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);
        let eq30_e1180_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);
        let eq30_e1180_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);
        let eq30_e1180_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);
        let eq30_e1180_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);
        let eq30_e1180_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);
        let eq30_e1180_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);
        let eq30_e1180_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);
        let eq30_e1180_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);
        let eq30_e1180_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);
        let eq30_e1180_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);
        let eq30_e1180_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);
        let eq30_e1180_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);
        let eq30_e1180_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);
        let eq30_e1180_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);
        let eq30_e1180_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);
        let eq30_e1180_d_n18: f64 = (s.dn[753][18] + s.dn[751][18]);
        let eq30_e1180_d_b0: f64 = (s.db[753][0] + s.db[751][0]);
        let eq30_e1180_d_b1: f64 = (s.db[753][1] + s.db[751][1]);
        let eq30_e1180_d_b2: f64 = (s.db[753][2] + s.db[751][2]);
        let eq30_e1180_d_b3: f64 = (s.db[753][3] + s.db[751][3]);
        let eq30_e1180_d_b4: f64 = (s.db[753][4] + s.db[751][4]);
        let eq30_e1180_d_b5: f64 = (s.db[753][5] + s.db[751][5]);
        let eq30_e1180_d_b6: f64 = (s.db[753][6] + s.db[751][6]);
        let eq30_e1180_d_b7: f64 = (s.db[753][7] + s.db[751][7]);
        let eq30_e1180_d_b8: f64 = (s.db[753][8] + s.db[751][8]);
        let eq30_e1180_d_b9: f64 = (s.db[753][9] + s.db[751][9]);
        let eq30_e1180_d_b10: f64 = (s.db[753][10] + s.db[751][10]);
        let eq30_e1180_d_b11: f64 = (s.db[753][11] + s.db[751][11]);
        let eq30_e1180_d_b12: f64 = (s.db[753][12] + s.db[751][12]);
        let eq30_e1182: f64 = (eq30_e1180 + s.v[752]);
        let eq30_e1182_d_n0: f64 = (eq30_e1180_d_n0 + s.dn[752][0]);
        let eq30_e1182_d_n1: f64 = (eq30_e1180_d_n1 + s.dn[752][1]);
        let eq30_e1182_d_n2: f64 = (eq30_e1180_d_n2 + s.dn[752][2]);
        let eq30_e1182_d_n3: f64 = (eq30_e1180_d_n3 + s.dn[752][3]);
        let eq30_e1182_d_n4: f64 = (eq30_e1180_d_n4 + s.dn[752][4]);
        let eq30_e1182_d_n5: f64 = (eq30_e1180_d_n5 + s.dn[752][5]);
        let eq30_e1182_d_n6: f64 = (eq30_e1180_d_n6 + s.dn[752][6]);
        let eq30_e1182_d_n7: f64 = (eq30_e1180_d_n7 + s.dn[752][7]);
        let eq30_e1182_d_n8: f64 = (eq30_e1180_d_n8 + s.dn[752][8]);
        let eq30_e1182_d_n9: f64 = (eq30_e1180_d_n9 + s.dn[752][9]);
        let eq30_e1182_d_n10: f64 = (eq30_e1180_d_n10 + s.dn[752][10]);
        let eq30_e1182_d_n11: f64 = (eq30_e1180_d_n11 + s.dn[752][11]);
        let eq30_e1182_d_n12: f64 = (eq30_e1180_d_n12 + s.dn[752][12]);
        let eq30_e1182_d_n13: f64 = (eq30_e1180_d_n13 + s.dn[752][13]);
        let eq30_e1182_d_n14: f64 = (eq30_e1180_d_n14 + s.dn[752][14]);
        let eq30_e1182_d_n15: f64 = (eq30_e1180_d_n15 + s.dn[752][15]);
        let eq30_e1182_d_n16: f64 = (eq30_e1180_d_n16 + s.dn[752][16]);
        let eq30_e1182_d_n17: f64 = (eq30_e1180_d_n17 + s.dn[752][17]);
        let eq30_e1182_d_n18: f64 = (eq30_e1180_d_n18 + s.dn[752][18]);
        let eq30_e1182_d_b0: f64 = (eq30_e1180_d_b0 + s.db[752][0]);
        let eq30_e1182_d_b1: f64 = (eq30_e1180_d_b1 + s.db[752][1]);
        let eq30_e1182_d_b2: f64 = (eq30_e1180_d_b2 + s.db[752][2]);
        let eq30_e1182_d_b3: f64 = (eq30_e1180_d_b3 + s.db[752][3]);
        let eq30_e1182_d_b4: f64 = (eq30_e1180_d_b4 + s.db[752][4]);
        let eq30_e1182_d_b5: f64 = (eq30_e1180_d_b5 + s.db[752][5]);
        let eq30_e1182_d_b6: f64 = (eq30_e1180_d_b6 + s.db[752][6]);
        let eq30_e1182_d_b7: f64 = (eq30_e1180_d_b7 + s.db[752][7]);
        let eq30_e1182_d_b8: f64 = (eq30_e1180_d_b8 + s.db[752][8]);
        let eq30_e1182_d_b9: f64 = (eq30_e1180_d_b9 + s.db[752][9]);
        let eq30_e1182_d_b10: f64 = (eq30_e1180_d_b10 + s.db[752][10]);
        let eq30_e1182_d_b11: f64 = (eq30_e1180_d_b11 + s.db[752][11]);
        let eq30_e1182_d_b12: f64 = (eq30_e1180_d_b12 + s.db[752][12]);
        let eq30_e1183: f64 = (s.v[20] - eq30_e1182);
        let eq30_e1183_d_n0: f64 = (s.dn[20][0] - eq30_e1182_d_n0);
        let eq30_e1183_d_n1: f64 = (s.dn[20][1] - eq30_e1182_d_n1);
        let eq30_e1183_d_n2: f64 = (s.dn[20][2] - eq30_e1182_d_n2);
        let eq30_e1183_d_n3: f64 = (s.dn[20][3] - eq30_e1182_d_n3);
        let eq30_e1183_d_n4: f64 = (s.dn[20][4] - eq30_e1182_d_n4);
        let eq30_e1183_d_n5: f64 = (s.dn[20][5] - eq30_e1182_d_n5);
        let eq30_e1183_d_n6: f64 = (s.dn[20][6] - eq30_e1182_d_n6);
        let eq30_e1183_d_n7: f64 = (s.dn[20][7] - eq30_e1182_d_n7);
        let eq30_e1183_d_n8: f64 = (s.dn[20][8] - eq30_e1182_d_n8);
        let eq30_e1183_d_n9: f64 = (s.dn[20][9] - eq30_e1182_d_n9);
        let eq30_e1183_d_n10: f64 = (s.dn[20][10] - eq30_e1182_d_n10);
        let eq30_e1183_d_n11: f64 = (s.dn[20][11] - eq30_e1182_d_n11);
        let eq30_e1183_d_n12: f64 = (s.dn[20][12] - eq30_e1182_d_n12);
        let eq30_e1183_d_n13: f64 = (s.dn[20][13] - eq30_e1182_d_n13);
        let eq30_e1183_d_n14: f64 = (s.dn[20][14] - eq30_e1182_d_n14);
        let eq30_e1183_d_n15: f64 = (s.dn[20][15] - eq30_e1182_d_n15);
        let eq30_e1183_d_n16: f64 = (s.dn[20][16] - eq30_e1182_d_n16);
        let eq30_e1183_d_n17: f64 = (s.dn[20][17] - eq30_e1182_d_n17);
        let eq30_e1183_d_n18: f64 = (s.dn[20][18] - eq30_e1182_d_n18);
        let eq30_e1183_d_b0: f64 = (s.db[20][0] - eq30_e1182_d_b0);
        let eq30_e1183_d_b1: f64 = (s.db[20][1] - eq30_e1182_d_b1);
        let eq30_e1183_d_b2: f64 = (s.db[20][2] - eq30_e1182_d_b2);
        let eq30_e1183_d_b3: f64 = (s.db[20][3] - eq30_e1182_d_b3);
        let eq30_e1183_d_b4: f64 = (s.db[20][4] - eq30_e1182_d_b4);
        let eq30_e1183_d_b5: f64 = (s.db[20][5] - eq30_e1182_d_b5);
        let eq30_e1183_d_b6: f64 = (s.db[20][6] - eq30_e1182_d_b6);
        let eq30_e1183_d_b7: f64 = (s.db[20][7] - eq30_e1182_d_b7);
        let eq30_e1183_d_b8: f64 = (s.db[20][8] - eq30_e1182_d_b8);
        let eq30_e1183_d_b9: f64 = (s.db[20][9] - eq30_e1182_d_b9);
        let eq30_e1183_d_b10: f64 = (s.db[20][10] - eq30_e1182_d_b10);
        let eq30_e1183_d_b11: f64 = (s.db[20][11] - eq30_e1182_d_b11);
        let eq30_e1183_d_b12: f64 = (s.db[20][12] - eq30_e1182_d_b12);
        let eq30_e1184_q: f64 = eq30_e1183;
        let eq30_e1185: f64 = (p.p87 * eq30_e1183);
        let eq30_e1185_d_n0: f64 = (p.p87 * eq30_e1183_d_n0);
        let eq30_e1185_d_n1: f64 = (p.p87 * eq30_e1183_d_n1);
        let eq30_e1185_d_n2: f64 = (p.p87 * eq30_e1183_d_n2);
        let eq30_e1185_d_n3: f64 = (p.p87 * eq30_e1183_d_n3);
        let eq30_e1185_d_n4: f64 = (p.p87 * eq30_e1183_d_n4);
        let eq30_e1185_d_n5: f64 = (p.p87 * eq30_e1183_d_n5);
        let eq30_e1185_d_n6: f64 = (p.p87 * eq30_e1183_d_n6);
        let eq30_e1185_d_n7: f64 = (p.p87 * eq30_e1183_d_n7);
        let eq30_e1185_d_n8: f64 = (p.p87 * eq30_e1183_d_n8);
        let eq30_e1185_d_n9: f64 = (p.p87 * eq30_e1183_d_n9);
        let eq30_e1185_d_n10: f64 = (p.p87 * eq30_e1183_d_n10);
        let eq30_e1185_d_n11: f64 = (p.p87 * eq30_e1183_d_n11);
        let eq30_e1185_d_n12: f64 = (p.p87 * eq30_e1183_d_n12);
        let eq30_e1185_d_n13: f64 = (p.p87 * eq30_e1183_d_n13);
        let eq30_e1185_d_n14: f64 = (p.p87 * eq30_e1183_d_n14);
        let eq30_e1185_d_n15: f64 = (p.p87 * eq30_e1183_d_n15);
        let eq30_e1185_d_n16: f64 = (p.p87 * eq30_e1183_d_n16);
        let eq30_e1185_d_n17: f64 = (p.p87 * eq30_e1183_d_n17);
        let eq30_e1185_d_n18: f64 = (p.p87 * eq30_e1183_d_n18);
        let eq30_e1185_d_b0: f64 = (p.p87 * eq30_e1183_d_b0);
        let eq30_e1185_d_b1: f64 = (p.p87 * eq30_e1183_d_b1);
        let eq30_e1185_d_b2: f64 = (p.p87 * eq30_e1183_d_b2);
        let eq30_e1185_d_b3: f64 = (p.p87 * eq30_e1183_d_b3);
        let eq30_e1185_d_b4: f64 = (p.p87 * eq30_e1183_d_b4);
        let eq30_e1185_d_b5: f64 = (p.p87 * eq30_e1183_d_b5);
        let eq30_e1185_d_b6: f64 = (p.p87 * eq30_e1183_d_b6);
        let eq30_e1185_d_b7: f64 = (p.p87 * eq30_e1183_d_b7);
        let eq30_e1185_d_b8: f64 = (p.p87 * eq30_e1183_d_b8);
        let eq30_e1185_d_b9: f64 = (p.p87 * eq30_e1183_d_b9);
        let eq30_e1185_d_b10: f64 = (p.p87 * eq30_e1183_d_b10);
        let eq30_e1185_d_b11: f64 = (p.p87 * eq30_e1183_d_b11);
        let eq30_e1185_d_b12: f64 = (p.p87 * eq30_e1183_d_b12);
        let eq30_e1185_q: f64 = (p.p87 * eq30_e1184_q);
        let eq30_e1185_q_d_n0: f64 = (p.p87 * eq30_e1183_d_n0);
        let eq30_e1185_q_d_n1: f64 = (p.p87 * eq30_e1183_d_n1);
        let eq30_e1185_q_d_n2: f64 = (p.p87 * eq30_e1183_d_n2);
        let eq30_e1185_q_d_n3: f64 = (p.p87 * eq30_e1183_d_n3);
        let eq30_e1185_q_d_n4: f64 = (p.p87 * eq30_e1183_d_n4);
        let eq30_e1185_q_d_n5: f64 = (p.p87 * eq30_e1183_d_n5);
        let eq30_e1185_q_d_n6: f64 = (p.p87 * eq30_e1183_d_n6);
        let eq30_e1185_q_d_n7: f64 = (p.p87 * eq30_e1183_d_n7);
        let eq30_e1185_q_d_n8: f64 = (p.p87 * eq30_e1183_d_n8);
        let eq30_e1185_q_d_n9: f64 = (p.p87 * eq30_e1183_d_n9);
        let eq30_e1185_q_d_n10: f64 = (p.p87 * eq30_e1183_d_n10);
        let eq30_e1185_q_d_n11: f64 = (p.p87 * eq30_e1183_d_n11);
        let eq30_e1185_q_d_n12: f64 = (p.p87 * eq30_e1183_d_n12);
        let eq30_e1185_q_d_n13: f64 = (p.p87 * eq30_e1183_d_n13);
        let eq30_e1185_q_d_n14: f64 = (p.p87 * eq30_e1183_d_n14);
        let eq30_e1185_q_d_n15: f64 = (p.p87 * eq30_e1183_d_n15);
        let eq30_e1185_q_d_n16: f64 = (p.p87 * eq30_e1183_d_n16);
        let eq30_e1185_q_d_n17: f64 = (p.p87 * eq30_e1183_d_n17);
        let eq30_e1185_q_d_n18: f64 = (p.p87 * eq30_e1183_d_n18);
        let eq30_e1185_q_d_b0: f64 = (p.p87 * eq30_e1183_d_b0);
        let eq30_e1185_q_d_b1: f64 = (p.p87 * eq30_e1183_d_b1);
        let eq30_e1185_q_d_b2: f64 = (p.p87 * eq30_e1183_d_b2);
        let eq30_e1185_q_d_b3: f64 = (p.p87 * eq30_e1183_d_b3);
        let eq30_e1185_q_d_b4: f64 = (p.p87 * eq30_e1183_d_b4);
        let eq30_e1185_q_d_b5: f64 = (p.p87 * eq30_e1183_d_b5);
        let eq30_e1185_q_d_b6: f64 = (p.p87 * eq30_e1183_d_b6);
        let eq30_e1185_q_d_b7: f64 = (p.p87 * eq30_e1183_d_b7);
        let eq30_e1185_q_d_b8: f64 = (p.p87 * eq30_e1183_d_b8);
        let eq30_e1185_q_d_b9: f64 = (p.p87 * eq30_e1183_d_b9);
        let eq30_e1185_q_d_b10: f64 = (p.p87 * eq30_e1183_d_b10);
        let eq30_e1185_q_d_b11: f64 = (p.p87 * eq30_e1183_d_b11);
        let eq30_e1185_q_d_b12: f64 = (p.p87 * eq30_e1183_d_b12);
        let eq30_reactive_node_derivatives: [f64; 19] = [eq30_e1185_q_d_n0, eq30_e1185_q_d_n1, eq30_e1185_q_d_n2, eq30_e1185_q_d_n3, eq30_e1185_q_d_n4, eq30_e1185_q_d_n5, eq30_e1185_q_d_n6, eq30_e1185_q_d_n7, eq30_e1185_q_d_n8, eq30_e1185_q_d_n9, eq30_e1185_q_d_n10, eq30_e1185_q_d_n11, eq30_e1185_q_d_n12, eq30_e1185_q_d_n13, eq30_e1185_q_d_n14, eq30_e1185_q_d_n15, eq30_e1185_q_d_n16, eq30_e1185_q_d_n17, eq30_e1185_q_d_n18];
        let eq30_reactive_branch_derivatives: [f64; 13] = [eq30_e1185_q_d_b0, eq30_e1185_q_d_b1, eq30_e1185_q_d_b2, eq30_e1185_q_d_b3, eq30_e1185_q_d_b4, eq30_e1185_q_d_b5, eq30_e1185_q_d_b6, eq30_e1185_q_d_b7, eq30_e1185_q_d_b8, eq30_e1185_q_d_b9, eq30_e1185_q_d_b10, eq30_e1185_q_d_b11, eq30_e1185_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188_q: f64 = s.v[743];
        let eq31_e1189: f64 = (p.p87 * s.v[743]);
        let eq31_e1189_d_n0: f64 = (p.p87 * s.dn[743][0]);
        let eq31_e1189_d_n1: f64 = (p.p87 * s.dn[743][1]);
        let eq31_e1189_d_n2: f64 = (p.p87 * s.dn[743][2]);
        let eq31_e1189_d_n3: f64 = (p.p87 * s.dn[743][3]);
        let eq31_e1189_d_n4: f64 = (p.p87 * s.dn[743][4]);
        let eq31_e1189_d_n5: f64 = (p.p87 * s.dn[743][5]);
        let eq31_e1189_d_n6: f64 = (p.p87 * s.dn[743][6]);
        let eq31_e1189_d_n7: f64 = (p.p87 * s.dn[743][7]);
        let eq31_e1189_d_n8: f64 = (p.p87 * s.dn[743][8]);
        let eq31_e1189_d_n9: f64 = (p.p87 * s.dn[743][9]);
        let eq31_e1189_d_n10: f64 = (p.p87 * s.dn[743][10]);
        let eq31_e1189_d_n11: f64 = (p.p87 * s.dn[743][11]);
        let eq31_e1189_d_n12: f64 = (p.p87 * s.dn[743][12]);
        let eq31_e1189_d_n13: f64 = (p.p87 * s.dn[743][13]);
        let eq31_e1189_d_n14: f64 = (p.p87 * s.dn[743][14]);
        let eq31_e1189_d_n15: f64 = (p.p87 * s.dn[743][15]);
        let eq31_e1189_d_n16: f64 = (p.p87 * s.dn[743][16]);
        let eq31_e1189_d_n17: f64 = (p.p87 * s.dn[743][17]);
        let eq31_e1189_d_n18: f64 = (p.p87 * s.dn[743][18]);
        let eq31_e1189_d_b0: f64 = (p.p87 * s.db[743][0]);
        let eq31_e1189_d_b1: f64 = (p.p87 * s.db[743][1]);
        let eq31_e1189_d_b2: f64 = (p.p87 * s.db[743][2]);
        let eq31_e1189_d_b3: f64 = (p.p87 * s.db[743][3]);
        let eq31_e1189_d_b4: f64 = (p.p87 * s.db[743][4]);
        let eq31_e1189_d_b5: f64 = (p.p87 * s.db[743][5]);
        let eq31_e1189_d_b6: f64 = (p.p87 * s.db[743][6]);
        let eq31_e1189_d_b7: f64 = (p.p87 * s.db[743][7]);
        let eq31_e1189_d_b8: f64 = (p.p87 * s.db[743][8]);
        let eq31_e1189_d_b9: f64 = (p.p87 * s.db[743][9]);
        let eq31_e1189_d_b10: f64 = (p.p87 * s.db[743][10]);
        let eq31_e1189_d_b11: f64 = (p.p87 * s.db[743][11]);
        let eq31_e1189_d_b12: f64 = (p.p87 * s.db[743][12]);
        let eq31_e1189_q: f64 = (p.p87 * eq31_e1188_q);
        let eq31_e1189_q_d_n0: f64 = (p.p87 * s.dn[743][0]);
        let eq31_e1189_q_d_n1: f64 = (p.p87 * s.dn[743][1]);
        let eq31_e1189_q_d_n2: f64 = (p.p87 * s.dn[743][2]);
        let eq31_e1189_q_d_n3: f64 = (p.p87 * s.dn[743][3]);
        let eq31_e1189_q_d_n4: f64 = (p.p87 * s.dn[743][4]);
        let eq31_e1189_q_d_n5: f64 = (p.p87 * s.dn[743][5]);
        let eq31_e1189_q_d_n6: f64 = (p.p87 * s.dn[743][6]);
        let eq31_e1189_q_d_n7: f64 = (p.p87 * s.dn[743][7]);
        let eq31_e1189_q_d_n8: f64 = (p.p87 * s.dn[743][8]);
        let eq31_e1189_q_d_n9: f64 = (p.p87 * s.dn[743][9]);
        let eq31_e1189_q_d_n10: f64 = (p.p87 * s.dn[743][10]);
        let eq31_e1189_q_d_n11: f64 = (p.p87 * s.dn[743][11]);
        let eq31_e1189_q_d_n12: f64 = (p.p87 * s.dn[743][12]);
        let eq31_e1189_q_d_n13: f64 = (p.p87 * s.dn[743][13]);
        let eq31_e1189_q_d_n14: f64 = (p.p87 * s.dn[743][14]);
        let eq31_e1189_q_d_n15: f64 = (p.p87 * s.dn[743][15]);
        let eq31_e1189_q_d_n16: f64 = (p.p87 * s.dn[743][16]);
        let eq31_e1189_q_d_n17: f64 = (p.p87 * s.dn[743][17]);
        let eq31_e1189_q_d_n18: f64 = (p.p87 * s.dn[743][18]);
        let eq31_e1189_q_d_b0: f64 = (p.p87 * s.db[743][0]);
        let eq31_e1189_q_d_b1: f64 = (p.p87 * s.db[743][1]);
        let eq31_e1189_q_d_b2: f64 = (p.p87 * s.db[743][2]);
        let eq31_e1189_q_d_b3: f64 = (p.p87 * s.db[743][3]);
        let eq31_e1189_q_d_b4: f64 = (p.p87 * s.db[743][4]);
        let eq31_e1189_q_d_b5: f64 = (p.p87 * s.db[743][5]);
        let eq31_e1189_q_d_b6: f64 = (p.p87 * s.db[743][6]);
        let eq31_e1189_q_d_b7: f64 = (p.p87 * s.db[743][7]);
        let eq31_e1189_q_d_b8: f64 = (p.p87 * s.db[743][8]);
        let eq31_e1189_q_d_b9: f64 = (p.p87 * s.db[743][9]);
        let eq31_e1189_q_d_b10: f64 = (p.p87 * s.db[743][10]);
        let eq31_e1189_q_d_b11: f64 = (p.p87 * s.db[743][11]);
        let eq31_e1189_q_d_b12: f64 = (p.p87 * s.db[743][12]);
        let eq31_reactive_node_derivatives: [f64; 19] = [eq31_e1189_q_d_n0, eq31_e1189_q_d_n1, eq31_e1189_q_d_n2, eq31_e1189_q_d_n3, eq31_e1189_q_d_n4, eq31_e1189_q_d_n5, eq31_e1189_q_d_n6, eq31_e1189_q_d_n7, eq31_e1189_q_d_n8, eq31_e1189_q_d_n9, eq31_e1189_q_d_n10, eq31_e1189_q_d_n11, eq31_e1189_q_d_n12, eq31_e1189_q_d_n13, eq31_e1189_q_d_n14, eq31_e1189_q_d_n15, eq31_e1189_q_d_n16, eq31_e1189_q_d_n17, eq31_e1189_q_d_n18];
        let eq31_reactive_branch_derivatives: [f64; 13] = [eq31_e1189_q_d_b0, eq31_e1189_q_d_b1, eq31_e1189_q_d_b2, eq31_e1189_q_d_b3, eq31_e1189_q_d_b4, eq31_e1189_q_d_b5, eq31_e1189_q_d_b6, eq31_e1189_q_d_b7, eq31_e1189_q_d_b8, eq31_e1189_q_d_b9, eq31_e1189_q_d_b10, eq31_e1189_q_d_b11, eq31_e1189_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq32_e1192_q: f64 = s.v[742];
        let eq32_e1193: f64 = (p.p87 * s.v[742]);
        let eq32_e1193_d_n0: f64 = (p.p87 * s.dn[742][0]);
        let eq32_e1193_d_n1: f64 = (p.p87 * s.dn[742][1]);
        let eq32_e1193_d_n2: f64 = (p.p87 * s.dn[742][2]);
        let eq32_e1193_d_n3: f64 = (p.p87 * s.dn[742][3]);
        let eq32_e1193_d_n4: f64 = (p.p87 * s.dn[742][4]);
        let eq32_e1193_d_n5: f64 = (p.p87 * s.dn[742][5]);
        let eq32_e1193_d_n6: f64 = (p.p87 * s.dn[742][6]);
        let eq32_e1193_d_n7: f64 = (p.p87 * s.dn[742][7]);
        let eq32_e1193_d_n8: f64 = (p.p87 * s.dn[742][8]);
        let eq32_e1193_d_n9: f64 = (p.p87 * s.dn[742][9]);
        let eq32_e1193_d_n10: f64 = (p.p87 * s.dn[742][10]);
        let eq32_e1193_d_n11: f64 = (p.p87 * s.dn[742][11]);
        let eq32_e1193_d_n12: f64 = (p.p87 * s.dn[742][12]);
        let eq32_e1193_d_n13: f64 = (p.p87 * s.dn[742][13]);
        let eq32_e1193_d_n14: f64 = (p.p87 * s.dn[742][14]);
        let eq32_e1193_d_n15: f64 = (p.p87 * s.dn[742][15]);
        let eq32_e1193_d_n16: f64 = (p.p87 * s.dn[742][16]);
        let eq32_e1193_d_n17: f64 = (p.p87 * s.dn[742][17]);
        let eq32_e1193_d_n18: f64 = (p.p87 * s.dn[742][18]);
        let eq32_e1193_d_b0: f64 = (p.p87 * s.db[742][0]);
        let eq32_e1193_d_b1: f64 = (p.p87 * s.db[742][1]);
        let eq32_e1193_d_b2: f64 = (p.p87 * s.db[742][2]);
        let eq32_e1193_d_b3: f64 = (p.p87 * s.db[742][3]);
        let eq32_e1193_d_b4: f64 = (p.p87 * s.db[742][4]);
        let eq32_e1193_d_b5: f64 = (p.p87 * s.db[742][5]);
        let eq32_e1193_d_b6: f64 = (p.p87 * s.db[742][6]);
        let eq32_e1193_d_b7: f64 = (p.p87 * s.db[742][7]);
        let eq32_e1193_d_b8: f64 = (p.p87 * s.db[742][8]);
        let eq32_e1193_d_b9: f64 = (p.p87 * s.db[742][9]);
        let eq32_e1193_d_b10: f64 = (p.p87 * s.db[742][10]);
        let eq32_e1193_d_b11: f64 = (p.p87 * s.db[742][11]);
        let eq32_e1193_d_b12: f64 = (p.p87 * s.db[742][12]);
        let eq32_e1193_q: f64 = (p.p87 * eq32_e1192_q);
        let eq32_e1193_q_d_n0: f64 = (p.p87 * s.dn[742][0]);
        let eq32_e1193_q_d_n1: f64 = (p.p87 * s.dn[742][1]);
        let eq32_e1193_q_d_n2: f64 = (p.p87 * s.dn[742][2]);
        let eq32_e1193_q_d_n3: f64 = (p.p87 * s.dn[742][3]);
        let eq32_e1193_q_d_n4: f64 = (p.p87 * s.dn[742][4]);
        let eq32_e1193_q_d_n5: f64 = (p.p87 * s.dn[742][5]);
        let eq32_e1193_q_d_n6: f64 = (p.p87 * s.dn[742][6]);
        let eq32_e1193_q_d_n7: f64 = (p.p87 * s.dn[742][7]);
        let eq32_e1193_q_d_n8: f64 = (p.p87 * s.dn[742][8]);
        let eq32_e1193_q_d_n9: f64 = (p.p87 * s.dn[742][9]);
        let eq32_e1193_q_d_n10: f64 = (p.p87 * s.dn[742][10]);
        let eq32_e1193_q_d_n11: f64 = (p.p87 * s.dn[742][11]);
        let eq32_e1193_q_d_n12: f64 = (p.p87 * s.dn[742][12]);
        let eq32_e1193_q_d_n13: f64 = (p.p87 * s.dn[742][13]);
        let eq32_e1193_q_d_n14: f64 = (p.p87 * s.dn[742][14]);
        let eq32_e1193_q_d_n15: f64 = (p.p87 * s.dn[742][15]);
        let eq32_e1193_q_d_n16: f64 = (p.p87 * s.dn[742][16]);
        let eq32_e1193_q_d_n17: f64 = (p.p87 * s.dn[742][17]);
        let eq32_e1193_q_d_n18: f64 = (p.p87 * s.dn[742][18]);
        let eq32_e1193_q_d_b0: f64 = (p.p87 * s.db[742][0]);
        let eq32_e1193_q_d_b1: f64 = (p.p87 * s.db[742][1]);
        let eq32_e1193_q_d_b2: f64 = (p.p87 * s.db[742][2]);
        let eq32_e1193_q_d_b3: f64 = (p.p87 * s.db[742][3]);
        let eq32_e1193_q_d_b4: f64 = (p.p87 * s.db[742][4]);
        let eq32_e1193_q_d_b5: f64 = (p.p87 * s.db[742][5]);
        let eq32_e1193_q_d_b6: f64 = (p.p87 * s.db[742][6]);
        let eq32_e1193_q_d_b7: f64 = (p.p87 * s.db[742][7]);
        let eq32_e1193_q_d_b8: f64 = (p.p87 * s.db[742][8]);
        let eq32_e1193_q_d_b9: f64 = (p.p87 * s.db[742][9]);
        let eq32_e1193_q_d_b10: f64 = (p.p87 * s.db[742][10]);
        let eq32_e1193_q_d_b11: f64 = (p.p87 * s.db[742][11]);
        let eq32_e1193_q_d_b12: f64 = (p.p87 * s.db[742][12]);
        let eq32_reactive_node_derivatives: [f64; 19] = [eq32_e1193_q_d_n0, eq32_e1193_q_d_n1, eq32_e1193_q_d_n2, eq32_e1193_q_d_n3, eq32_e1193_q_d_n4, eq32_e1193_q_d_n5, eq32_e1193_q_d_n6, eq32_e1193_q_d_n7, eq32_e1193_q_d_n8, eq32_e1193_q_d_n9, eq32_e1193_q_d_n10, eq32_e1193_q_d_n11, eq32_e1193_q_d_n12, eq32_e1193_q_d_n13, eq32_e1193_q_d_n14, eq32_e1193_q_d_n15, eq32_e1193_q_d_n16, eq32_e1193_q_d_n17, eq32_e1193_q_d_n18];
        let eq32_reactive_branch_derivatives: [f64; 13] = [eq32_e1193_q_d_b0, eq32_e1193_q_d_b1, eq32_e1193_q_d_b2, eq32_e1193_q_d_b3, eq32_e1193_q_d_b4, eq32_e1193_q_d_b5, eq32_e1193_q_d_b6, eq32_e1193_q_d_b7, eq32_e1193_q_d_b8, eq32_e1193_q_d_b9, eq32_e1193_q_d_b10, eq32_e1193_q_d_b11, eq32_e1193_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e1196_q: f64 = s.v[744];
        let eq33_e1197: f64 = (p.p87 * s.v[744]);
        let eq33_e1197_d_n0: f64 = (p.p87 * s.dn[744][0]);
        let eq33_e1197_d_n1: f64 = (p.p87 * s.dn[744][1]);
        let eq33_e1197_d_n2: f64 = (p.p87 * s.dn[744][2]);
        let eq33_e1197_d_n3: f64 = (p.p87 * s.dn[744][3]);
        let eq33_e1197_d_n4: f64 = (p.p87 * s.dn[744][4]);
        let eq33_e1197_d_n5: f64 = (p.p87 * s.dn[744][5]);
        let eq33_e1197_d_n6: f64 = (p.p87 * s.dn[744][6]);
        let eq33_e1197_d_n7: f64 = (p.p87 * s.dn[744][7]);
        let eq33_e1197_d_n8: f64 = (p.p87 * s.dn[744][8]);
        let eq33_e1197_d_n9: f64 = (p.p87 * s.dn[744][9]);
        let eq33_e1197_d_n10: f64 = (p.p87 * s.dn[744][10]);
        let eq33_e1197_d_n11: f64 = (p.p87 * s.dn[744][11]);
        let eq33_e1197_d_n12: f64 = (p.p87 * s.dn[744][12]);
        let eq33_e1197_d_n13: f64 = (p.p87 * s.dn[744][13]);
        let eq33_e1197_d_n14: f64 = (p.p87 * s.dn[744][14]);
        let eq33_e1197_d_n15: f64 = (p.p87 * s.dn[744][15]);
        let eq33_e1197_d_n16: f64 = (p.p87 * s.dn[744][16]);
        let eq33_e1197_d_n17: f64 = (p.p87 * s.dn[744][17]);
        let eq33_e1197_d_n18: f64 = (p.p87 * s.dn[744][18]);
        let eq33_e1197_d_b0: f64 = (p.p87 * s.db[744][0]);
        let eq33_e1197_d_b1: f64 = (p.p87 * s.db[744][1]);
        let eq33_e1197_d_b2: f64 = (p.p87 * s.db[744][2]);
        let eq33_e1197_d_b3: f64 = (p.p87 * s.db[744][3]);
        let eq33_e1197_d_b4: f64 = (p.p87 * s.db[744][4]);
        let eq33_e1197_d_b5: f64 = (p.p87 * s.db[744][5]);
        let eq33_e1197_d_b6: f64 = (p.p87 * s.db[744][6]);
        let eq33_e1197_d_b7: f64 = (p.p87 * s.db[744][7]);
        let eq33_e1197_d_b8: f64 = (p.p87 * s.db[744][8]);
        let eq33_e1197_d_b9: f64 = (p.p87 * s.db[744][9]);
        let eq33_e1197_d_b10: f64 = (p.p87 * s.db[744][10]);
        let eq33_e1197_d_b11: f64 = (p.p87 * s.db[744][11]);
        let eq33_e1197_d_b12: f64 = (p.p87 * s.db[744][12]);
        let eq33_e1197_q: f64 = (p.p87 * eq33_e1196_q);
        let eq33_e1197_q_d_n0: f64 = (p.p87 * s.dn[744][0]);
        let eq33_e1197_q_d_n1: f64 = (p.p87 * s.dn[744][1]);
        let eq33_e1197_q_d_n2: f64 = (p.p87 * s.dn[744][2]);
        let eq33_e1197_q_d_n3: f64 = (p.p87 * s.dn[744][3]);
        let eq33_e1197_q_d_n4: f64 = (p.p87 * s.dn[744][4]);
        let eq33_e1197_q_d_n5: f64 = (p.p87 * s.dn[744][5]);
        let eq33_e1197_q_d_n6: f64 = (p.p87 * s.dn[744][6]);
        let eq33_e1197_q_d_n7: f64 = (p.p87 * s.dn[744][7]);
        let eq33_e1197_q_d_n8: f64 = (p.p87 * s.dn[744][8]);
        let eq33_e1197_q_d_n9: f64 = (p.p87 * s.dn[744][9]);
        let eq33_e1197_q_d_n10: f64 = (p.p87 * s.dn[744][10]);
        let eq33_e1197_q_d_n11: f64 = (p.p87 * s.dn[744][11]);
        let eq33_e1197_q_d_n12: f64 = (p.p87 * s.dn[744][12]);
        let eq33_e1197_q_d_n13: f64 = (p.p87 * s.dn[744][13]);
        let eq33_e1197_q_d_n14: f64 = (p.p87 * s.dn[744][14]);
        let eq33_e1197_q_d_n15: f64 = (p.p87 * s.dn[744][15]);
        let eq33_e1197_q_d_n16: f64 = (p.p87 * s.dn[744][16]);
        let eq33_e1197_q_d_n17: f64 = (p.p87 * s.dn[744][17]);
        let eq33_e1197_q_d_n18: f64 = (p.p87 * s.dn[744][18]);
        let eq33_e1197_q_d_b0: f64 = (p.p87 * s.db[744][0]);
        let eq33_e1197_q_d_b1: f64 = (p.p87 * s.db[744][1]);
        let eq33_e1197_q_d_b2: f64 = (p.p87 * s.db[744][2]);
        let eq33_e1197_q_d_b3: f64 = (p.p87 * s.db[744][3]);
        let eq33_e1197_q_d_b4: f64 = (p.p87 * s.db[744][4]);
        let eq33_e1197_q_d_b5: f64 = (p.p87 * s.db[744][5]);
        let eq33_e1197_q_d_b6: f64 = (p.p87 * s.db[744][6]);
        let eq33_e1197_q_d_b7: f64 = (p.p87 * s.db[744][7]);
        let eq33_e1197_q_d_b8: f64 = (p.p87 * s.db[744][8]);
        let eq33_e1197_q_d_b9: f64 = (p.p87 * s.db[744][9]);
        let eq33_e1197_q_d_b10: f64 = (p.p87 * s.db[744][10]);
        let eq33_e1197_q_d_b11: f64 = (p.p87 * s.db[744][11]);
        let eq33_e1197_q_d_b12: f64 = (p.p87 * s.db[744][12]);
        let eq33_reactive_node_derivatives: [f64; 19] = [eq33_e1197_q_d_n0, eq33_e1197_q_d_n1, eq33_e1197_q_d_n2, eq33_e1197_q_d_n3, eq33_e1197_q_d_n4, eq33_e1197_q_d_n5, eq33_e1197_q_d_n6, eq33_e1197_q_d_n7, eq33_e1197_q_d_n8, eq33_e1197_q_d_n9, eq33_e1197_q_d_n10, eq33_e1197_q_d_n11, eq33_e1197_q_d_n12, eq33_e1197_q_d_n13, eq33_e1197_q_d_n14, eq33_e1197_q_d_n15, eq33_e1197_q_d_n16, eq33_e1197_q_d_n17, eq33_e1197_q_d_n18];
        let eq33_reactive_branch_derivatives: [f64; 13] = [eq33_e1197_q_d_b0, eq33_e1197_q_d_b1, eq33_e1197_q_d_b2, eq33_e1197_q_d_b3, eq33_e1197_q_d_b4, eq33_e1197_q_d_b5, eq33_e1197_q_d_b6, eq33_e1197_q_d_b7, eq33_e1197_q_d_b8, eq33_e1197_q_d_b9, eq33_e1197_q_d_b10, eq33_e1197_q_d_b11, eq33_e1197_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e1199: f64 = (-p.p87);
        let eq34_e1201_q: f64 = s.v[299];
        let eq34_e1202: f64 = (eq34_e1199 * s.v[299]);
        let eq34_e1202_d_n0: f64 = (eq34_e1199 * s.dn[299][0]);
        let eq34_e1202_d_n1: f64 = (eq34_e1199 * s.dn[299][1]);
        let eq34_e1202_d_n2: f64 = (eq34_e1199 * s.dn[299][2]);
        let eq34_e1202_d_n3: f64 = (eq34_e1199 * s.dn[299][3]);
        let eq34_e1202_d_n4: f64 = (eq34_e1199 * s.dn[299][4]);
        let eq34_e1202_d_n5: f64 = (eq34_e1199 * s.dn[299][5]);
        let eq34_e1202_d_n6: f64 = (eq34_e1199 * s.dn[299][6]);
        let eq34_e1202_d_n7: f64 = (eq34_e1199 * s.dn[299][7]);
        let eq34_e1202_d_n8: f64 = (eq34_e1199 * s.dn[299][8]);
        let eq34_e1202_d_n9: f64 = (eq34_e1199 * s.dn[299][9]);
        let eq34_e1202_d_n10: f64 = (eq34_e1199 * s.dn[299][10]);
        let eq34_e1202_d_n11: f64 = (eq34_e1199 * s.dn[299][11]);
        let eq34_e1202_d_n12: f64 = (eq34_e1199 * s.dn[299][12]);
        let eq34_e1202_d_n13: f64 = (eq34_e1199 * s.dn[299][13]);
        let eq34_e1202_d_n14: f64 = (eq34_e1199 * s.dn[299][14]);
        let eq34_e1202_d_n15: f64 = (eq34_e1199 * s.dn[299][15]);
        let eq34_e1202_d_n16: f64 = (eq34_e1199 * s.dn[299][16]);
        let eq34_e1202_d_n17: f64 = (eq34_e1199 * s.dn[299][17]);
        let eq34_e1202_d_n18: f64 = (eq34_e1199 * s.dn[299][18]);
        let eq34_e1202_d_b0: f64 = (eq34_e1199 * s.db[299][0]);
        let eq34_e1202_d_b1: f64 = (eq34_e1199 * s.db[299][1]);
        let eq34_e1202_d_b2: f64 = (eq34_e1199 * s.db[299][2]);
        let eq34_e1202_d_b3: f64 = (eq34_e1199 * s.db[299][3]);
        let eq34_e1202_d_b4: f64 = (eq34_e1199 * s.db[299][4]);
        let eq34_e1202_d_b5: f64 = (eq34_e1199 * s.db[299][5]);
        let eq34_e1202_d_b6: f64 = (eq34_e1199 * s.db[299][6]);
        let eq34_e1202_d_b7: f64 = (eq34_e1199 * s.db[299][7]);
        let eq34_e1202_d_b8: f64 = (eq34_e1199 * s.db[299][8]);
        let eq34_e1202_d_b9: f64 = (eq34_e1199 * s.db[299][9]);
        let eq34_e1202_d_b10: f64 = (eq34_e1199 * s.db[299][10]);
        let eq34_e1202_d_b11: f64 = (eq34_e1199 * s.db[299][11]);
        let eq34_e1202_d_b12: f64 = (eq34_e1199 * s.db[299][12]);
        let eq34_e1202_q: f64 = (eq34_e1199 * eq34_e1201_q);
        let eq34_e1202_q_d_n0: f64 = (eq34_e1199 * s.dn[299][0]);
        let eq34_e1202_q_d_n1: f64 = (eq34_e1199 * s.dn[299][1]);
        let eq34_e1202_q_d_n2: f64 = (eq34_e1199 * s.dn[299][2]);
        let eq34_e1202_q_d_n3: f64 = (eq34_e1199 * s.dn[299][3]);
        let eq34_e1202_q_d_n4: f64 = (eq34_e1199 * s.dn[299][4]);
        let eq34_e1202_q_d_n5: f64 = (eq34_e1199 * s.dn[299][5]);
        let eq34_e1202_q_d_n6: f64 = (eq34_e1199 * s.dn[299][6]);
        let eq34_e1202_q_d_n7: f64 = (eq34_e1199 * s.dn[299][7]);
        let eq34_e1202_q_d_n8: f64 = (eq34_e1199 * s.dn[299][8]);
        let eq34_e1202_q_d_n9: f64 = (eq34_e1199 * s.dn[299][9]);
        let eq34_e1202_q_d_n10: f64 = (eq34_e1199 * s.dn[299][10]);
        let eq34_e1202_q_d_n11: f64 = (eq34_e1199 * s.dn[299][11]);
        let eq34_e1202_q_d_n12: f64 = (eq34_e1199 * s.dn[299][12]);
        let eq34_e1202_q_d_n13: f64 = (eq34_e1199 * s.dn[299][13]);
        let eq34_e1202_q_d_n14: f64 = (eq34_e1199 * s.dn[299][14]);
        let eq34_e1202_q_d_n15: f64 = (eq34_e1199 * s.dn[299][15]);
        let eq34_e1202_q_d_n16: f64 = (eq34_e1199 * s.dn[299][16]);
        let eq34_e1202_q_d_n17: f64 = (eq34_e1199 * s.dn[299][17]);
        let eq34_e1202_q_d_n18: f64 = (eq34_e1199 * s.dn[299][18]);
        let eq34_e1202_q_d_b0: f64 = (eq34_e1199 * s.db[299][0]);
        let eq34_e1202_q_d_b1: f64 = (eq34_e1199 * s.db[299][1]);
        let eq34_e1202_q_d_b2: f64 = (eq34_e1199 * s.db[299][2]);
        let eq34_e1202_q_d_b3: f64 = (eq34_e1199 * s.db[299][3]);
        let eq34_e1202_q_d_b4: f64 = (eq34_e1199 * s.db[299][4]);
        let eq34_e1202_q_d_b5: f64 = (eq34_e1199 * s.db[299][5]);
        let eq34_e1202_q_d_b6: f64 = (eq34_e1199 * s.db[299][6]);
        let eq34_e1202_q_d_b7: f64 = (eq34_e1199 * s.db[299][7]);
        let eq34_e1202_q_d_b8: f64 = (eq34_e1199 * s.db[299][8]);
        let eq34_e1202_q_d_b9: f64 = (eq34_e1199 * s.db[299][9]);
        let eq34_e1202_q_d_b10: f64 = (eq34_e1199 * s.db[299][10]);
        let eq34_e1202_q_d_b11: f64 = (eq34_e1199 * s.db[299][11]);
        let eq34_e1202_q_d_b12: f64 = (eq34_e1199 * s.db[299][12]);
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e1202_q_d_n0, eq34_e1202_q_d_n1, eq34_e1202_q_d_n2, eq34_e1202_q_d_n3, eq34_e1202_q_d_n4, eq34_e1202_q_d_n5, eq34_e1202_q_d_n6, eq34_e1202_q_d_n7, eq34_e1202_q_d_n8, eq34_e1202_q_d_n9, eq34_e1202_q_d_n10, eq34_e1202_q_d_n11, eq34_e1202_q_d_n12, eq34_e1202_q_d_n13, eq34_e1202_q_d_n14, eq34_e1202_q_d_n15, eq34_e1202_q_d_n16, eq34_e1202_q_d_n17, eq34_e1202_q_d_n18];
        let eq34_reactive_branch_derivatives: [f64; 13] = [eq34_e1202_q_d_b0, eq34_e1202_q_d_b1, eq34_e1202_q_d_b2, eq34_e1202_q_d_b3, eq34_e1202_q_d_b4, eq34_e1202_q_d_b5, eq34_e1202_q_d_b6, eq34_e1202_q_d_b7, eq34_e1202_q_d_b8, eq34_e1202_q_d_b9, eq34_e1202_q_d_b10, eq34_e1202_q_d_b11, eq34_e1202_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[0]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e1204: f64 = (-p.p87);
        let eq35_e1206_q: f64 = s.v[301];
        let eq35_e1207: f64 = (eq35_e1204 * s.v[301]);
        let eq35_e1207_d_n0: f64 = (eq35_e1204 * s.dn[301][0]);
        let eq35_e1207_d_n1: f64 = (eq35_e1204 * s.dn[301][1]);
        let eq35_e1207_d_n2: f64 = (eq35_e1204 * s.dn[301][2]);
        let eq35_e1207_d_n3: f64 = (eq35_e1204 * s.dn[301][3]);
        let eq35_e1207_d_n4: f64 = (eq35_e1204 * s.dn[301][4]);
        let eq35_e1207_d_n5: f64 = (eq35_e1204 * s.dn[301][5]);
        let eq35_e1207_d_n6: f64 = (eq35_e1204 * s.dn[301][6]);
        let eq35_e1207_d_n7: f64 = (eq35_e1204 * s.dn[301][7]);
        let eq35_e1207_d_n8: f64 = (eq35_e1204 * s.dn[301][8]);
        let eq35_e1207_d_n9: f64 = (eq35_e1204 * s.dn[301][9]);
        let eq35_e1207_d_n10: f64 = (eq35_e1204 * s.dn[301][10]);
        let eq35_e1207_d_n11: f64 = (eq35_e1204 * s.dn[301][11]);
        let eq35_e1207_d_n12: f64 = (eq35_e1204 * s.dn[301][12]);
        let eq35_e1207_d_n13: f64 = (eq35_e1204 * s.dn[301][13]);
        let eq35_e1207_d_n14: f64 = (eq35_e1204 * s.dn[301][14]);
        let eq35_e1207_d_n15: f64 = (eq35_e1204 * s.dn[301][15]);
        let eq35_e1207_d_n16: f64 = (eq35_e1204 * s.dn[301][16]);
        let eq35_e1207_d_n17: f64 = (eq35_e1204 * s.dn[301][17]);
        let eq35_e1207_d_n18: f64 = (eq35_e1204 * s.dn[301][18]);
        let eq35_e1207_d_b0: f64 = (eq35_e1204 * s.db[301][0]);
        let eq35_e1207_d_b1: f64 = (eq35_e1204 * s.db[301][1]);
        let eq35_e1207_d_b2: f64 = (eq35_e1204 * s.db[301][2]);
        let eq35_e1207_d_b3: f64 = (eq35_e1204 * s.db[301][3]);
        let eq35_e1207_d_b4: f64 = (eq35_e1204 * s.db[301][4]);
        let eq35_e1207_d_b5: f64 = (eq35_e1204 * s.db[301][5]);
        let eq35_e1207_d_b6: f64 = (eq35_e1204 * s.db[301][6]);
        let eq35_e1207_d_b7: f64 = (eq35_e1204 * s.db[301][7]);
        let eq35_e1207_d_b8: f64 = (eq35_e1204 * s.db[301][8]);
        let eq35_e1207_d_b9: f64 = (eq35_e1204 * s.db[301][9]);
        let eq35_e1207_d_b10: f64 = (eq35_e1204 * s.db[301][10]);
        let eq35_e1207_d_b11: f64 = (eq35_e1204 * s.db[301][11]);
        let eq35_e1207_d_b12: f64 = (eq35_e1204 * s.db[301][12]);
        let eq35_e1207_q: f64 = (eq35_e1204 * eq35_e1206_q);
        let eq35_e1207_q_d_n0: f64 = (eq35_e1204 * s.dn[301][0]);
        let eq35_e1207_q_d_n1: f64 = (eq35_e1204 * s.dn[301][1]);
        let eq35_e1207_q_d_n2: f64 = (eq35_e1204 * s.dn[301][2]);
        let eq35_e1207_q_d_n3: f64 = (eq35_e1204 * s.dn[301][3]);
        let eq35_e1207_q_d_n4: f64 = (eq35_e1204 * s.dn[301][4]);
        let eq35_e1207_q_d_n5: f64 = (eq35_e1204 * s.dn[301][5]);
        let eq35_e1207_q_d_n6: f64 = (eq35_e1204 * s.dn[301][6]);
        let eq35_e1207_q_d_n7: f64 = (eq35_e1204 * s.dn[301][7]);
        let eq35_e1207_q_d_n8: f64 = (eq35_e1204 * s.dn[301][8]);
        let eq35_e1207_q_d_n9: f64 = (eq35_e1204 * s.dn[301][9]);
        let eq35_e1207_q_d_n10: f64 = (eq35_e1204 * s.dn[301][10]);
        let eq35_e1207_q_d_n11: f64 = (eq35_e1204 * s.dn[301][11]);
        let eq35_e1207_q_d_n12: f64 = (eq35_e1204 * s.dn[301][12]);
        let eq35_e1207_q_d_n13: f64 = (eq35_e1204 * s.dn[301][13]);
        let eq35_e1207_q_d_n14: f64 = (eq35_e1204 * s.dn[301][14]);
        let eq35_e1207_q_d_n15: f64 = (eq35_e1204 * s.dn[301][15]);
        let eq35_e1207_q_d_n16: f64 = (eq35_e1204 * s.dn[301][16]);
        let eq35_e1207_q_d_n17: f64 = (eq35_e1204 * s.dn[301][17]);
        let eq35_e1207_q_d_n18: f64 = (eq35_e1204 * s.dn[301][18]);
        let eq35_e1207_q_d_b0: f64 = (eq35_e1204 * s.db[301][0]);
        let eq35_e1207_q_d_b1: f64 = (eq35_e1204 * s.db[301][1]);
        let eq35_e1207_q_d_b2: f64 = (eq35_e1204 * s.db[301][2]);
        let eq35_e1207_q_d_b3: f64 = (eq35_e1204 * s.db[301][3]);
        let eq35_e1207_q_d_b4: f64 = (eq35_e1204 * s.db[301][4]);
        let eq35_e1207_q_d_b5: f64 = (eq35_e1204 * s.db[301][5]);
        let eq35_e1207_q_d_b6: f64 = (eq35_e1204 * s.db[301][6]);
        let eq35_e1207_q_d_b7: f64 = (eq35_e1204 * s.db[301][7]);
        let eq35_e1207_q_d_b8: f64 = (eq35_e1204 * s.db[301][8]);
        let eq35_e1207_q_d_b9: f64 = (eq35_e1204 * s.db[301][9]);
        let eq35_e1207_q_d_b10: f64 = (eq35_e1204 * s.db[301][10]);
        let eq35_e1207_q_d_b11: f64 = (eq35_e1204 * s.db[301][11]);
        let eq35_e1207_q_d_b12: f64 = (eq35_e1204 * s.db[301][12]);
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e1207_q_d_n0, eq35_e1207_q_d_n1, eq35_e1207_q_d_n2, eq35_e1207_q_d_n3, eq35_e1207_q_d_n4, eq35_e1207_q_d_n5, eq35_e1207_q_d_n6, eq35_e1207_q_d_n7, eq35_e1207_q_d_n8, eq35_e1207_q_d_n9, eq35_e1207_q_d_n10, eq35_e1207_q_d_n11, eq35_e1207_q_d_n12, eq35_e1207_q_d_n13, eq35_e1207_q_d_n14, eq35_e1207_q_d_n15, eq35_e1207_q_d_n16, eq35_e1207_q_d_n17, eq35_e1207_q_d_n18];
        let eq35_reactive_branch_derivatives: [f64; 13] = [eq35_e1207_q_d_b0, eq35_e1207_q_d_b1, eq35_e1207_q_d_b2, eq35_e1207_q_d_b3, eq35_e1207_q_d_b4, eq35_e1207_q_d_b5, eq35_e1207_q_d_b6, eq35_e1207_q_d_b7, eq35_e1207_q_d_b8, eq35_e1207_q_d_b9, eq35_e1207_q_d_b10, eq35_e1207_q_d_b11, eq35_e1207_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv15 - 0.0) * s.v[954]);
        let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * s.dn[954][0]);
        let eq41_e1236_d_n1: f64 = ((nv15 - 0.0) * s.dn[954][1]);
        let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * s.dn[954][2]);
        let eq41_e1236_d_n3: f64 = ((nv15 - 0.0) * s.dn[954][3]);
        let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * s.dn[954][4]);
        let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * s.dn[954][5]);
        let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * s.dn[954][6]);
        let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * s.dn[954][7]);
        let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * s.dn[954][8]);
        let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * s.dn[954][9]);
        let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * s.dn[954][10]);
        let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * s.dn[954][11]);
        let eq41_e1236_d_n12: f64 = ((nv15 - 0.0) * s.dn[954][12]);
        let eq41_e1236_d_n13: f64 = ((nv15 - 0.0) * s.dn[954][13]);
        let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * s.dn[954][14]);
        let eq41_e1236_d_n15: f64 = (s.v[954] + ((nv15 - 0.0) * s.dn[954][15]));
        let eq41_e1236_d_n16: f64 = ((nv15 - 0.0) * s.dn[954][16]);
        let eq41_e1236_d_n17: f64 = ((nv15 - 0.0) * s.dn[954][17]);
        let eq41_e1236_d_n18: f64 = ((nv15 - 0.0) * s.dn[954][18]);
        let eq41_e1236_d_b0: f64 = ((nv15 - 0.0) * s.db[954][0]);
        let eq41_e1236_d_b1: f64 = ((nv15 - 0.0) * s.db[954][1]);
        let eq41_e1236_d_b2: f64 = ((nv15 - 0.0) * s.db[954][2]);
        let eq41_e1236_d_b3: f64 = ((nv15 - 0.0) * s.db[954][3]);
        let eq41_e1236_d_b4: f64 = ((nv15 - 0.0) * s.db[954][4]);
        let eq41_e1236_d_b5: f64 = ((nv15 - 0.0) * s.db[954][5]);
        let eq41_e1236_d_b6: f64 = ((nv15 - 0.0) * s.db[954][6]);
        let eq41_e1236_d_b7: f64 = ((nv15 - 0.0) * s.db[954][7]);
        let eq41_e1236_d_b8: f64 = ((nv15 - 0.0) * s.db[954][8]);
        let eq41_e1236_d_b9: f64 = ((nv15 - 0.0) * s.db[954][9]);
        let eq41_e1236_d_b10: f64 = ((nv15 - 0.0) * s.db[954][10]);
        let eq41_e1236_d_b11: f64 = ((nv15 - 0.0) * s.db[954][11]);
        let eq41_e1236_d_b12: f64 = ((nv15 - 0.0) * s.db[954][12]);
        let eq41_e1237_q: f64 = eq41_e1236;
        let eq41_reactive_node_derivatives: [f64; 19] = [eq41_e1236_d_n0, eq41_e1236_d_n1, eq41_e1236_d_n2, eq41_e1236_d_n3, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, eq41_e1236_d_n11, eq41_e1236_d_n12, eq41_e1236_d_n13, eq41_e1236_d_n14, eq41_e1236_d_n15, eq41_e1236_d_n16, eq41_e1236_d_n17, eq41_e1236_d_n18];
        let eq41_reactive_branch_derivatives: [f64; 13] = [eq41_e1236_d_b0, eq41_e1236_d_b1, eq41_e1236_d_b2, eq41_e1236_d_b3, eq41_e1236_d_b4, eq41_e1236_d_b5, eq41_e1236_d_b6, eq41_e1236_d_b7, eq41_e1236_d_b8, eq41_e1236_d_b9, eq41_e1236_d_b10, eq41_e1236_d_b11, eq41_e1236_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1240: f64 = ((nv15 - 0.0) * s.v[955]);
        let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * s.dn[955][0]);
        let eq42_e1240_d_n1: f64 = ((nv15 - 0.0) * s.dn[955][1]);
        let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * s.dn[955][2]);
        let eq42_e1240_d_n3: f64 = ((nv15 - 0.0) * s.dn[955][3]);
        let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * s.dn[955][4]);
        let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * s.dn[955][5]);
        let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * s.dn[955][6]);
        let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * s.dn[955][7]);
        let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * s.dn[955][8]);
        let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * s.dn[955][9]);
        let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * s.dn[955][10]);
        let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * s.dn[955][11]);
        let eq42_e1240_d_n12: f64 = ((nv15 - 0.0) * s.dn[955][12]);
        let eq42_e1240_d_n13: f64 = ((nv15 - 0.0) * s.dn[955][13]);
        let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * s.dn[955][14]);
        let eq42_e1240_d_n15: f64 = (s.v[955] + ((nv15 - 0.0) * s.dn[955][15]));
        let eq42_e1240_d_n16: f64 = ((nv15 - 0.0) * s.dn[955][16]);
        let eq42_e1240_d_n17: f64 = ((nv15 - 0.0) * s.dn[955][17]);
        let eq42_e1240_d_n18: f64 = ((nv15 - 0.0) * s.dn[955][18]);
        let eq42_e1240_d_b0: f64 = ((nv15 - 0.0) * s.db[955][0]);
        let eq42_e1240_d_b1: f64 = ((nv15 - 0.0) * s.db[955][1]);
        let eq42_e1240_d_b2: f64 = ((nv15 - 0.0) * s.db[955][2]);
        let eq42_e1240_d_b3: f64 = ((nv15 - 0.0) * s.db[955][3]);
        let eq42_e1240_d_b4: f64 = ((nv15 - 0.0) * s.db[955][4]);
        let eq42_e1240_d_b5: f64 = ((nv15 - 0.0) * s.db[955][5]);
        let eq42_e1240_d_b6: f64 = ((nv15 - 0.0) * s.db[955][6]);
        let eq42_e1240_d_b7: f64 = ((nv15 - 0.0) * s.db[955][7]);
        let eq42_e1240_d_b8: f64 = ((nv15 - 0.0) * s.db[955][8]);
        let eq42_e1240_d_b9: f64 = ((nv15 - 0.0) * s.db[955][9]);
        let eq42_e1240_d_b10: f64 = ((nv15 - 0.0) * s.db[955][10]);
        let eq42_e1240_d_b11: f64 = ((nv15 - 0.0) * s.db[955][11]);
        let eq42_e1240_d_b12: f64 = ((nv15 - 0.0) * s.db[955][12]);
        let eq42_e1241_q: f64 = eq42_e1240;
        let eq42_reactive_node_derivatives: [f64; 19] = [eq42_e1240_d_n0, eq42_e1240_d_n1, eq42_e1240_d_n2, eq42_e1240_d_n3, eq42_e1240_d_n4, eq42_e1240_d_n5, eq42_e1240_d_n6, eq42_e1240_d_n7, eq42_e1240_d_n8, eq42_e1240_d_n9, eq42_e1240_d_n10, eq42_e1240_d_n11, eq42_e1240_d_n12, eq42_e1240_d_n13, eq42_e1240_d_n14, eq42_e1240_d_n15, eq42_e1240_d_n16, eq42_e1240_d_n17, eq42_e1240_d_n18];
        let eq42_reactive_branch_derivatives: [f64; 13] = [eq42_e1240_d_b0, eq42_e1240_d_b1, eq42_e1240_d_b2, eq42_e1240_d_b3, eq42_e1240_d_b4, eq42_e1240_d_b5, eq42_e1240_d_b6, eq42_e1240_d_b7, eq42_e1240_d_b8, eq42_e1240_d_b9, eq42_e1240_d_b10, eq42_e1240_d_b11, eq42_e1240_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1362, eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_n18, eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1362_d_b9, eq62_e1362_d_b10, eq62_e1362_d_b11, eq62_e1362_d_b12, eq62_e1362_q, eq62_e1362_q_d_n0, eq62_e1362_q_d_n1, eq62_e1362_q_d_n2, eq62_e1362_q_d_n3, eq62_e1362_q_d_n4, eq62_e1362_q_d_n5, eq62_e1362_q_d_n6, eq62_e1362_q_d_n7, eq62_e1362_q_d_n8, eq62_e1362_q_d_n9, eq62_e1362_q_d_n10, eq62_e1362_q_d_n11, eq62_e1362_q_d_n12, eq62_e1362_q_d_n13, eq62_e1362_q_d_n14, eq62_e1362_q_d_n15, eq62_e1362_q_d_n16, eq62_e1362_q_d_n17, eq62_e1362_q_d_n18, eq62_e1362_q_d_b0, eq62_e1362_q_d_b1, eq62_e1362_q_d_b2, eq62_e1362_q_d_b3, eq62_e1362_q_d_b4, eq62_e1362_q_d_b5, eq62_e1362_q_d_b6, eq62_e1362_q_d_b7, eq62_e1362_q_d_b8, eq62_e1362_q_d_b9, eq62_e1362_q_d_b10, eq62_e1362_q_d_b11, eq62_e1362_q_d_b12,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (s.v[800] * (nv12 - 0.0));
        let eq62_e1359_d_n0: f64 = (s.dn[800][0] * (nv12 - 0.0));
        let eq62_e1359_d_n1: f64 = (s.dn[800][1] * (nv12 - 0.0));
        let eq62_e1359_d_n2: f64 = (s.dn[800][2] * (nv12 - 0.0));
        let eq62_e1359_d_n3: f64 = (s.dn[800][3] * (nv12 - 0.0));
        let eq62_e1359_d_n4: f64 = (s.dn[800][4] * (nv12 - 0.0));
        let eq62_e1359_d_n5: f64 = (s.dn[800][5] * (nv12 - 0.0));
        let eq62_e1359_d_n6: f64 = (s.dn[800][6] * (nv12 - 0.0));
        let eq62_e1359_d_n7: f64 = (s.dn[800][7] * (nv12 - 0.0));
        let eq62_e1359_d_n8: f64 = (s.dn[800][8] * (nv12 - 0.0));
        let eq62_e1359_d_n9: f64 = (s.dn[800][9] * (nv12 - 0.0));
        let eq62_e1359_d_n10: f64 = (s.dn[800][10] * (nv12 - 0.0));
        let eq62_e1359_d_n11: f64 = (s.dn[800][11] * (nv12 - 0.0));
        let eq62_e1359_d_n12: f64 = ((s.dn[800][12] * (nv12 - 0.0)) + s.v[800]);
        let eq62_e1359_d_n13: f64 = (s.dn[800][13] * (nv12 - 0.0));
        let eq62_e1359_d_n14: f64 = (s.dn[800][14] * (nv12 - 0.0));
        let eq62_e1359_d_n15: f64 = (s.dn[800][15] * (nv12 - 0.0));
        let eq62_e1359_d_n16: f64 = (s.dn[800][16] * (nv12 - 0.0));
        let eq62_e1359_d_n17: f64 = (s.dn[800][17] * (nv12 - 0.0));
        let eq62_e1359_d_n18: f64 = (s.dn[800][18] * (nv12 - 0.0));
        let eq62_e1359_d_b0: f64 = (s.db[800][0] * (nv12 - 0.0));
        let eq62_e1359_d_b1: f64 = (s.db[800][1] * (nv12 - 0.0));
        let eq62_e1359_d_b2: f64 = (s.db[800][2] * (nv12 - 0.0));
        let eq62_e1359_d_b3: f64 = (s.db[800][3] * (nv12 - 0.0));
        let eq62_e1359_d_b4: f64 = (s.db[800][4] * (nv12 - 0.0));
        let eq62_e1359_d_b5: f64 = (s.db[800][5] * (nv12 - 0.0));
        let eq62_e1359_d_b6: f64 = (s.db[800][6] * (nv12 - 0.0));
        let eq62_e1359_d_b7: f64 = (s.db[800][7] * (nv12 - 0.0));
        let eq62_e1359_d_b8: f64 = (s.db[800][8] * (nv12 - 0.0));
        let eq62_e1359_d_b9: f64 = (s.db[800][9] * (nv12 - 0.0));
        let eq62_e1359_d_b10: f64 = (s.db[800][10] * (nv12 - 0.0));
        let eq62_e1359_d_b11: f64 = (s.db[800][11] * (nv12 - 0.0));
        let eq62_e1359_d_b12: f64 = (s.db[800][12] * (nv12 - 0.0));
        let eq62_e1360_q: f64 = eq62_e1359;
        (eq62_e1359, eq62_e1359_d_n0, eq62_e1359_d_n1, eq62_e1359_d_n2, eq62_e1359_d_n3, eq62_e1359_d_n4, eq62_e1359_d_n5, eq62_e1359_d_n6, eq62_e1359_d_n7, eq62_e1359_d_n8, eq62_e1359_d_n9, eq62_e1359_d_n10, eq62_e1359_d_n11, eq62_e1359_d_n12, eq62_e1359_d_n13, eq62_e1359_d_n14, eq62_e1359_d_n15, eq62_e1359_d_n16, eq62_e1359_d_n17, eq62_e1359_d_n18, eq62_e1359_d_b0, eq62_e1359_d_b1, eq62_e1359_d_b2, eq62_e1359_d_b3, eq62_e1359_d_b4, eq62_e1359_d_b5, eq62_e1359_d_b6, eq62_e1359_d_b7, eq62_e1359_d_b8, eq62_e1359_d_b9, eq62_e1359_d_b10, eq62_e1359_d_b11, eq62_e1359_d_b12, eq62_e1360_q, eq62_e1359_d_n0, eq62_e1359_d_n1, eq62_e1359_d_n2, eq62_e1359_d_n3, eq62_e1359_d_n4, eq62_e1359_d_n5, eq62_e1359_d_n6, eq62_e1359_d_n7, eq62_e1359_d_n8, eq62_e1359_d_n9, eq62_e1359_d_n10, eq62_e1359_d_n11, eq62_e1359_d_n12, eq62_e1359_d_n13, eq62_e1359_d_n14, eq62_e1359_d_n15, eq62_e1359_d_n16, eq62_e1359_d_n17, eq62_e1359_d_n18, eq62_e1359_d_b0, eq62_e1359_d_b1, eq62_e1359_d_b2, eq62_e1359_d_b3, eq62_e1359_d_b4, eq62_e1359_d_b5, eq62_e1359_d_b6, eq62_e1359_d_b7, eq62_e1359_d_b8, eq62_e1359_d_b9, eq62_e1359_d_b10, eq62_e1359_d_b11, eq62_e1359_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_reactive_node_derivatives: [f64; 19] = [eq62_e1362_q_d_n0, eq62_e1362_q_d_n1, eq62_e1362_q_d_n2, eq62_e1362_q_d_n3, eq62_e1362_q_d_n4, eq62_e1362_q_d_n5, eq62_e1362_q_d_n6, eq62_e1362_q_d_n7, eq62_e1362_q_d_n8, eq62_e1362_q_d_n9, eq62_e1362_q_d_n10, eq62_e1362_q_d_n11, eq62_e1362_q_d_n12, eq62_e1362_q_d_n13, eq62_e1362_q_d_n14, eq62_e1362_q_d_n15, eq62_e1362_q_d_n16, eq62_e1362_q_d_n17, eq62_e1362_q_d_n18];
        let eq62_reactive_branch_derivatives: [f64; 13] = [eq62_e1362_q_d_b0, eq62_e1362_q_d_b1, eq62_e1362_q_d_b2, eq62_e1362_q_d_b3, eq62_e1362_q_d_b4, eq62_e1362_q_d_b5, eq62_e1362_q_d_b6, eq62_e1362_q_d_b7, eq62_e1362_q_d_b8, eq62_e1362_q_d_b9, eq62_e1362_q_d_b10, eq62_e1362_q_d_b11, eq62_e1362_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq62_reactive_node_derivatives,
            branches,
            &eq62_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq63_e1369, eq63_e1369_d_n0, eq63_e1369_d_n1, eq63_e1369_d_n2, eq63_e1369_d_n3, eq63_e1369_d_n4, eq63_e1369_d_n5, eq63_e1369_d_n6, eq63_e1369_d_n7, eq63_e1369_d_n8, eq63_e1369_d_n9, eq63_e1369_d_n10, eq63_e1369_d_n11, eq63_e1369_d_n12, eq63_e1369_d_n13, eq63_e1369_d_n14, eq63_e1369_d_n15, eq63_e1369_d_n16, eq63_e1369_d_n17, eq63_e1369_d_n18, eq63_e1369_d_b0, eq63_e1369_d_b1, eq63_e1369_d_b2, eq63_e1369_d_b3, eq63_e1369_d_b4, eq63_e1369_d_b5, eq63_e1369_d_b6, eq63_e1369_d_b7, eq63_e1369_d_b8, eq63_e1369_d_b9, eq63_e1369_d_b10, eq63_e1369_d_b11, eq63_e1369_d_b12, eq63_e1369_q, eq63_e1369_q_d_n0, eq63_e1369_q_d_n1, eq63_e1369_q_d_n2, eq63_e1369_q_d_n3, eq63_e1369_q_d_n4, eq63_e1369_q_d_n5, eq63_e1369_q_d_n6, eq63_e1369_q_d_n7, eq63_e1369_q_d_n8, eq63_e1369_q_d_n9, eq63_e1369_q_d_n10, eq63_e1369_q_d_n11, eq63_e1369_q_d_n12, eq63_e1369_q_d_n13, eq63_e1369_q_d_n14, eq63_e1369_q_d_n15, eq63_e1369_q_d_n16, eq63_e1369_q_d_n17, eq63_e1369_q_d_n18, eq63_e1369_q_d_b0, eq63_e1369_q_d_b1, eq63_e1369_q_d_b2, eq63_e1369_q_d_b3, eq63_e1369_q_d_b4, eq63_e1369_q_d_b5, eq63_e1369_q_d_b6, eq63_e1369_q_d_b7, eq63_e1369_q_d_b8, eq63_e1369_q_d_b9, eq63_e1369_q_d_b10, eq63_e1369_q_d_b11, eq63_e1369_q_d_b12,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (s.v[801] * (nv13 - 0.0));
        let eq63_e1366_d_n0: f64 = (s.dn[801][0] * (nv13 - 0.0));
        let eq63_e1366_d_n1: f64 = (s.dn[801][1] * (nv13 - 0.0));
        let eq63_e1366_d_n2: f64 = (s.dn[801][2] * (nv13 - 0.0));
        let eq63_e1366_d_n3: f64 = (s.dn[801][3] * (nv13 - 0.0));
        let eq63_e1366_d_n4: f64 = (s.dn[801][4] * (nv13 - 0.0));
        let eq63_e1366_d_n5: f64 = (s.dn[801][5] * (nv13 - 0.0));
        let eq63_e1366_d_n6: f64 = (s.dn[801][6] * (nv13 - 0.0));
        let eq63_e1366_d_n7: f64 = (s.dn[801][7] * (nv13 - 0.0));
        let eq63_e1366_d_n8: f64 = (s.dn[801][8] * (nv13 - 0.0));
        let eq63_e1366_d_n9: f64 = (s.dn[801][9] * (nv13 - 0.0));
        let eq63_e1366_d_n10: f64 = (s.dn[801][10] * (nv13 - 0.0));
        let eq63_e1366_d_n11: f64 = (s.dn[801][11] * (nv13 - 0.0));
        let eq63_e1366_d_n12: f64 = (s.dn[801][12] * (nv13 - 0.0));
        let eq63_e1366_d_n13: f64 = ((s.dn[801][13] * (nv13 - 0.0)) + s.v[801]);
        let eq63_e1366_d_n14: f64 = (s.dn[801][14] * (nv13 - 0.0));
        let eq63_e1366_d_n15: f64 = (s.dn[801][15] * (nv13 - 0.0));
        let eq63_e1366_d_n16: f64 = (s.dn[801][16] * (nv13 - 0.0));
        let eq63_e1366_d_n17: f64 = (s.dn[801][17] * (nv13 - 0.0));
        let eq63_e1366_d_n18: f64 = (s.dn[801][18] * (nv13 - 0.0));
        let eq63_e1366_d_b0: f64 = (s.db[801][0] * (nv13 - 0.0));
        let eq63_e1366_d_b1: f64 = (s.db[801][1] * (nv13 - 0.0));
        let eq63_e1366_d_b2: f64 = (s.db[801][2] * (nv13 - 0.0));
        let eq63_e1366_d_b3: f64 = (s.db[801][3] * (nv13 - 0.0));
        let eq63_e1366_d_b4: f64 = (s.db[801][4] * (nv13 - 0.0));
        let eq63_e1366_d_b5: f64 = (s.db[801][5] * (nv13 - 0.0));
        let eq63_e1366_d_b6: f64 = (s.db[801][6] * (nv13 - 0.0));
        let eq63_e1366_d_b7: f64 = (s.db[801][7] * (nv13 - 0.0));
        let eq63_e1366_d_b8: f64 = (s.db[801][8] * (nv13 - 0.0));
        let eq63_e1366_d_b9: f64 = (s.db[801][9] * (nv13 - 0.0));
        let eq63_e1366_d_b10: f64 = (s.db[801][10] * (nv13 - 0.0));
        let eq63_e1366_d_b11: f64 = (s.db[801][11] * (nv13 - 0.0));
        let eq63_e1366_d_b12: f64 = (s.db[801][12] * (nv13 - 0.0));
        let eq63_e1367_q: f64 = eq63_e1366;
        (eq63_e1366, eq63_e1366_d_n0, eq63_e1366_d_n1, eq63_e1366_d_n2, eq63_e1366_d_n3, eq63_e1366_d_n4, eq63_e1366_d_n5, eq63_e1366_d_n6, eq63_e1366_d_n7, eq63_e1366_d_n8, eq63_e1366_d_n9, eq63_e1366_d_n10, eq63_e1366_d_n11, eq63_e1366_d_n12, eq63_e1366_d_n13, eq63_e1366_d_n14, eq63_e1366_d_n15, eq63_e1366_d_n16, eq63_e1366_d_n17, eq63_e1366_d_n18, eq63_e1366_d_b0, eq63_e1366_d_b1, eq63_e1366_d_b2, eq63_e1366_d_b3, eq63_e1366_d_b4, eq63_e1366_d_b5, eq63_e1366_d_b6, eq63_e1366_d_b7, eq63_e1366_d_b8, eq63_e1366_d_b9, eq63_e1366_d_b10, eq63_e1366_d_b11, eq63_e1366_d_b12, eq63_e1367_q, eq63_e1366_d_n0, eq63_e1366_d_n1, eq63_e1366_d_n2, eq63_e1366_d_n3, eq63_e1366_d_n4, eq63_e1366_d_n5, eq63_e1366_d_n6, eq63_e1366_d_n7, eq63_e1366_d_n8, eq63_e1366_d_n9, eq63_e1366_d_n10, eq63_e1366_d_n11, eq63_e1366_d_n12, eq63_e1366_d_n13, eq63_e1366_d_n14, eq63_e1366_d_n15, eq63_e1366_d_n16, eq63_e1366_d_n17, eq63_e1366_d_n18, eq63_e1366_d_b0, eq63_e1366_d_b1, eq63_e1366_d_b2, eq63_e1366_d_b3, eq63_e1366_d_b4, eq63_e1366_d_b5, eq63_e1366_d_b6, eq63_e1366_d_b7, eq63_e1366_d_b8, eq63_e1366_d_b9, eq63_e1366_d_b10, eq63_e1366_d_b11, eq63_e1366_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_reactive_node_derivatives: [f64; 19] = [eq63_e1369_q_d_n0, eq63_e1369_q_d_n1, eq63_e1369_q_d_n2, eq63_e1369_q_d_n3, eq63_e1369_q_d_n4, eq63_e1369_q_d_n5, eq63_e1369_q_d_n6, eq63_e1369_q_d_n7, eq63_e1369_q_d_n8, eq63_e1369_q_d_n9, eq63_e1369_q_d_n10, eq63_e1369_q_d_n11, eq63_e1369_q_d_n12, eq63_e1369_q_d_n13, eq63_e1369_q_d_n14, eq63_e1369_q_d_n15, eq63_e1369_q_d_n16, eq63_e1369_q_d_n17, eq63_e1369_q_d_n18];
        let eq63_reactive_branch_derivatives: [f64; 13] = [eq63_e1369_q_d_b0, eq63_e1369_q_d_b1, eq63_e1369_q_d_b2, eq63_e1369_q_d_b3, eq63_e1369_q_d_b4, eq63_e1369_q_d_b5, eq63_e1369_q_d_b6, eq63_e1369_q_d_b7, eq63_e1369_q_d_b8, eq63_e1369_q_d_b9, eq63_e1369_q_d_b10, eq63_e1369_q_d_b11, eq63_e1369_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            None,
            nodes,
            &eq63_reactive_node_derivatives,
            branches,
            &eq63_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq67_e1388, eq67_e1388_d_n14, eq67_e1388_q, eq67_e1388_q_d_n14,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386_q: f64 = (nv14 - 0.0);
        ((nv14 - 0.0), 1.0, eq67_e1386_q, 1.0,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[14]),
            None,
            nodes[14],
            multiplicity * (eq67_e1388_q_d_n14),
        );
    }
}
