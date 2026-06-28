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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq30_e1184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[743]);
        let eq30_e1184_d_n0: f64 = (s.dn[743][0] * ddt_scale);
        let eq30_e1184_d_n1: f64 = (s.dn[743][1] * ddt_scale);
        let eq30_e1184_d_n2: f64 = (s.dn[743][2] * ddt_scale);
        let eq30_e1184_d_n3: f64 = (s.dn[743][3] * ddt_scale);
        let eq30_e1184_d_n4: f64 = (s.dn[743][4] * ddt_scale);
        let eq30_e1184_d_n5: f64 = (s.dn[743][5] * ddt_scale);
        let eq30_e1184_d_n6: f64 = (s.dn[743][6] * ddt_scale);
        let eq30_e1184_d_n7: f64 = (s.dn[743][7] * ddt_scale);
        let eq30_e1184_d_n8: f64 = (s.dn[743][8] * ddt_scale);
        let eq30_e1184_d_n9: f64 = (s.dn[743][9] * ddt_scale);
        let eq30_e1184_d_n10: f64 = (s.dn[743][10] * ddt_scale);
        let eq30_e1184_d_n11: f64 = (s.dn[743][11] * ddt_scale);
        let eq30_e1184_d_n12: f64 = (s.dn[743][12] * ddt_scale);
        let eq30_e1184_d_n13: f64 = (s.dn[743][13] * ddt_scale);
        let eq30_e1184_d_n14: f64 = (s.dn[743][14] * ddt_scale);
        let eq30_e1184_d_n15: f64 = (s.dn[743][15] * ddt_scale);
        let eq30_e1184_d_n16: f64 = (s.dn[743][16] * ddt_scale);
        let eq30_e1184_d_n17: f64 = (s.dn[743][17] * ddt_scale);
        let eq30_e1184_d_b0: f64 = (s.db[743][0] * ddt_scale);
        let eq30_e1184_d_b1: f64 = (s.db[743][1] * ddt_scale);
        let eq30_e1184_d_b2: f64 = (s.db[743][2] * ddt_scale);
        let eq30_e1184_d_b3: f64 = (s.db[743][3] * ddt_scale);
        let eq30_e1184_d_b4: f64 = (s.db[743][4] * ddt_scale);
        let eq30_e1184_d_b5: f64 = (s.db[743][5] * ddt_scale);
        let eq30_e1184_d_b6: f64 = (s.db[743][6] * ddt_scale);
        let eq30_e1184_d_b7: f64 = (s.db[743][7] * ddt_scale);
        let eq30_e1184_d_b8: f64 = (s.db[743][8] * ddt_scale);
        let eq30_e1184_d_b9: f64 = (s.db[743][9] * ddt_scale);
        let eq30_e1184_d_b10: f64 = (s.db[743][10] * ddt_scale);
        let eq30_e1184_d_b11: f64 = (s.db[743][11] * ddt_scale);
        let eq30_e1185: f64 = (p.p87 * eq30_e1184);
        let eq30_e1185_d_n0: f64 = (p.p87 * eq30_e1184_d_n0);
        let eq30_e1185_d_n1: f64 = (p.p87 * eq30_e1184_d_n1);
        let eq30_e1185_d_n2: f64 = (p.p87 * eq30_e1184_d_n2);
        let eq30_e1185_d_n3: f64 = (p.p87 * eq30_e1184_d_n3);
        let eq30_e1185_d_n4: f64 = (p.p87 * eq30_e1184_d_n4);
        let eq30_e1185_d_n5: f64 = (p.p87 * eq30_e1184_d_n5);
        let eq30_e1185_d_n6: f64 = (p.p87 * eq30_e1184_d_n6);
        let eq30_e1185_d_n7: f64 = (p.p87 * eq30_e1184_d_n7);
        let eq30_e1185_d_n8: f64 = (p.p87 * eq30_e1184_d_n8);
        let eq30_e1185_d_n9: f64 = (p.p87 * eq30_e1184_d_n9);
        let eq30_e1185_d_n10: f64 = (p.p87 * eq30_e1184_d_n10);
        let eq30_e1185_d_n11: f64 = (p.p87 * eq30_e1184_d_n11);
        let eq30_e1185_d_n12: f64 = (p.p87 * eq30_e1184_d_n12);
        let eq30_e1185_d_n13: f64 = (p.p87 * eq30_e1184_d_n13);
        let eq30_e1185_d_n14: f64 = (p.p87 * eq30_e1184_d_n14);
        let eq30_e1185_d_n15: f64 = (p.p87 * eq30_e1184_d_n15);
        let eq30_e1185_d_n16: f64 = (p.p87 * eq30_e1184_d_n16);
        let eq30_e1185_d_n17: f64 = (p.p87 * eq30_e1184_d_n17);
        let eq30_e1185_d_b0: f64 = (p.p87 * eq30_e1184_d_b0);
        let eq30_e1185_d_b1: f64 = (p.p87 * eq30_e1184_d_b1);
        let eq30_e1185_d_b2: f64 = (p.p87 * eq30_e1184_d_b2);
        let eq30_e1185_d_b3: f64 = (p.p87 * eq30_e1184_d_b3);
        let eq30_e1185_d_b4: f64 = (p.p87 * eq30_e1184_d_b4);
        let eq30_e1185_d_b5: f64 = (p.p87 * eq30_e1184_d_b5);
        let eq30_e1185_d_b6: f64 = (p.p87 * eq30_e1184_d_b6);
        let eq30_e1185_d_b7: f64 = (p.p87 * eq30_e1184_d_b7);
        let eq30_e1185_d_b8: f64 = (p.p87 * eq30_e1184_d_b8);
        let eq30_e1185_d_b9: f64 = (p.p87 * eq30_e1184_d_b9);
        let eq30_e1185_d_b10: f64 = (p.p87 * eq30_e1184_d_b10);
        let eq30_e1185_d_b11: f64 = (p.p87 * eq30_e1184_d_b11);
        let eq30_value: f64 = eq30_e1185;
        let eq30_node_derivatives: [f64; 18] = [eq30_e1185_d_n0, eq30_e1185_d_n1, eq30_e1185_d_n2, eq30_e1185_d_n3, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14, eq30_e1185_d_n15, eq30_e1185_d_n16, eq30_e1185_d_n17];
        let eq30_branch_derivatives: [f64; 12] = [eq30_e1185_d_b0, eq30_e1185_d_b1, eq30_e1185_d_b2, eq30_e1185_d_b3, eq30_e1185_d_b4, eq30_e1185_d_b5, eq30_e1185_d_b6, eq30_e1185_d_b7, eq30_e1185_d_b8, eq30_e1185_d_b9, eq30_e1185_d_b10, eq30_e1185_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, s.v[742]);
        let eq31_e1188_d_n0: f64 = (s.dn[742][0] * ddt_scale);
        let eq31_e1188_d_n1: f64 = (s.dn[742][1] * ddt_scale);
        let eq31_e1188_d_n2: f64 = (s.dn[742][2] * ddt_scale);
        let eq31_e1188_d_n3: f64 = (s.dn[742][3] * ddt_scale);
        let eq31_e1188_d_n4: f64 = (s.dn[742][4] * ddt_scale);
        let eq31_e1188_d_n5: f64 = (s.dn[742][5] * ddt_scale);
        let eq31_e1188_d_n6: f64 = (s.dn[742][6] * ddt_scale);
        let eq31_e1188_d_n7: f64 = (s.dn[742][7] * ddt_scale);
        let eq31_e1188_d_n8: f64 = (s.dn[742][8] * ddt_scale);
        let eq31_e1188_d_n9: f64 = (s.dn[742][9] * ddt_scale);
        let eq31_e1188_d_n10: f64 = (s.dn[742][10] * ddt_scale);
        let eq31_e1188_d_n11: f64 = (s.dn[742][11] * ddt_scale);
        let eq31_e1188_d_n12: f64 = (s.dn[742][12] * ddt_scale);
        let eq31_e1188_d_n13: f64 = (s.dn[742][13] * ddt_scale);
        let eq31_e1188_d_n14: f64 = (s.dn[742][14] * ddt_scale);
        let eq31_e1188_d_n15: f64 = (s.dn[742][15] * ddt_scale);
        let eq31_e1188_d_n16: f64 = (s.dn[742][16] * ddt_scale);
        let eq31_e1188_d_n17: f64 = (s.dn[742][17] * ddt_scale);
        let eq31_e1188_d_b0: f64 = (s.db[742][0] * ddt_scale);
        let eq31_e1188_d_b1: f64 = (s.db[742][1] * ddt_scale);
        let eq31_e1188_d_b2: f64 = (s.db[742][2] * ddt_scale);
        let eq31_e1188_d_b3: f64 = (s.db[742][3] * ddt_scale);
        let eq31_e1188_d_b4: f64 = (s.db[742][4] * ddt_scale);
        let eq31_e1188_d_b5: f64 = (s.db[742][5] * ddt_scale);
        let eq31_e1188_d_b6: f64 = (s.db[742][6] * ddt_scale);
        let eq31_e1188_d_b7: f64 = (s.db[742][7] * ddt_scale);
        let eq31_e1188_d_b8: f64 = (s.db[742][8] * ddt_scale);
        let eq31_e1188_d_b9: f64 = (s.db[742][9] * ddt_scale);
        let eq31_e1188_d_b10: f64 = (s.db[742][10] * ddt_scale);
        let eq31_e1188_d_b11: f64 = (s.db[742][11] * ddt_scale);
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
        let eq31_value: f64 = eq31_e1189;
        let eq31_node_derivatives: [f64; 18] = [eq31_e1189_d_n0, eq31_e1189_d_n1, eq31_e1189_d_n2, eq31_e1189_d_n3, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, eq31_e1189_d_n12, eq31_e1189_d_n13, eq31_e1189_d_n14, eq31_e1189_d_n15, eq31_e1189_d_n16, eq31_e1189_d_n17];
        let eq31_branch_derivatives: [f64; 12] = [eq31_e1189_d_b0, eq31_e1189_d_b1, eq31_e1189_d_b2, eq31_e1189_d_b3, eq31_e1189_d_b4, eq31_e1189_d_b5, eq31_e1189_d_b6, eq31_e1189_d_b7, eq31_e1189_d_b8, eq31_e1189_d_b9, eq31_e1189_d_b10, eq31_e1189_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[744]);
        let eq32_e1192_d_n0: f64 = (s.dn[744][0] * ddt_scale);
        let eq32_e1192_d_n1: f64 = (s.dn[744][1] * ddt_scale);
        let eq32_e1192_d_n2: f64 = (s.dn[744][2] * ddt_scale);
        let eq32_e1192_d_n3: f64 = (s.dn[744][3] * ddt_scale);
        let eq32_e1192_d_n4: f64 = (s.dn[744][4] * ddt_scale);
        let eq32_e1192_d_n5: f64 = (s.dn[744][5] * ddt_scale);
        let eq32_e1192_d_n6: f64 = (s.dn[744][6] * ddt_scale);
        let eq32_e1192_d_n7: f64 = (s.dn[744][7] * ddt_scale);
        let eq32_e1192_d_n8: f64 = (s.dn[744][8] * ddt_scale);
        let eq32_e1192_d_n9: f64 = (s.dn[744][9] * ddt_scale);
        let eq32_e1192_d_n10: f64 = (s.dn[744][10] * ddt_scale);
        let eq32_e1192_d_n11: f64 = (s.dn[744][11] * ddt_scale);
        let eq32_e1192_d_n12: f64 = (s.dn[744][12] * ddt_scale);
        let eq32_e1192_d_n13: f64 = (s.dn[744][13] * ddt_scale);
        let eq32_e1192_d_n14: f64 = (s.dn[744][14] * ddt_scale);
        let eq32_e1192_d_n15: f64 = (s.dn[744][15] * ddt_scale);
        let eq32_e1192_d_n16: f64 = (s.dn[744][16] * ddt_scale);
        let eq32_e1192_d_n17: f64 = (s.dn[744][17] * ddt_scale);
        let eq32_e1192_d_b0: f64 = (s.db[744][0] * ddt_scale);
        let eq32_e1192_d_b1: f64 = (s.db[744][1] * ddt_scale);
        let eq32_e1192_d_b2: f64 = (s.db[744][2] * ddt_scale);
        let eq32_e1192_d_b3: f64 = (s.db[744][3] * ddt_scale);
        let eq32_e1192_d_b4: f64 = (s.db[744][4] * ddt_scale);
        let eq32_e1192_d_b5: f64 = (s.db[744][5] * ddt_scale);
        let eq32_e1192_d_b6: f64 = (s.db[744][6] * ddt_scale);
        let eq32_e1192_d_b7: f64 = (s.db[744][7] * ddt_scale);
        let eq32_e1192_d_b8: f64 = (s.db[744][8] * ddt_scale);
        let eq32_e1192_d_b9: f64 = (s.db[744][9] * ddt_scale);
        let eq32_e1192_d_b10: f64 = (s.db[744][10] * ddt_scale);
        let eq32_e1192_d_b11: f64 = (s.db[744][11] * ddt_scale);
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
        let eq32_value: f64 = eq32_e1193;
        let eq32_node_derivatives: [f64; 18] = [eq32_e1193_d_n0, eq32_e1193_d_n1, eq32_e1193_d_n2, eq32_e1193_d_n3, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, eq32_e1193_d_n12, eq32_e1193_d_n13, eq32_e1193_d_n14, eq32_e1193_d_n15, eq32_e1193_d_n16, eq32_e1193_d_n17];
        let eq32_branch_derivatives: [f64; 12] = [eq32_e1193_d_b0, eq32_e1193_d_b1, eq32_e1193_d_b2, eq32_e1193_d_b3, eq32_e1193_d_b4, eq32_e1193_d_b5, eq32_e1193_d_b6, eq32_e1193_d_b7, eq32_e1193_d_b8, eq32_e1193_d_b9, eq32_e1193_d_b10, eq32_e1193_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e1195: f64 = (-p.p87);
        let eq33_e1197: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, s.v[299]);
        let eq33_e1197_d_n0: f64 = (s.dn[299][0] * ddt_scale);
        let eq33_e1197_d_n1: f64 = (s.dn[299][1] * ddt_scale);
        let eq33_e1197_d_n2: f64 = (s.dn[299][2] * ddt_scale);
        let eq33_e1197_d_n3: f64 = (s.dn[299][3] * ddt_scale);
        let eq33_e1197_d_n4: f64 = (s.dn[299][4] * ddt_scale);
        let eq33_e1197_d_n5: f64 = (s.dn[299][5] * ddt_scale);
        let eq33_e1197_d_n6: f64 = (s.dn[299][6] * ddt_scale);
        let eq33_e1197_d_n7: f64 = (s.dn[299][7] * ddt_scale);
        let eq33_e1197_d_n8: f64 = (s.dn[299][8] * ddt_scale);
        let eq33_e1197_d_n9: f64 = (s.dn[299][9] * ddt_scale);
        let eq33_e1197_d_n10: f64 = (s.dn[299][10] * ddt_scale);
        let eq33_e1197_d_n11: f64 = (s.dn[299][11] * ddt_scale);
        let eq33_e1197_d_n12: f64 = (s.dn[299][12] * ddt_scale);
        let eq33_e1197_d_n13: f64 = (s.dn[299][13] * ddt_scale);
        let eq33_e1197_d_n14: f64 = (s.dn[299][14] * ddt_scale);
        let eq33_e1197_d_n15: f64 = (s.dn[299][15] * ddt_scale);
        let eq33_e1197_d_n16: f64 = (s.dn[299][16] * ddt_scale);
        let eq33_e1197_d_n17: f64 = (s.dn[299][17] * ddt_scale);
        let eq33_e1197_d_b0: f64 = (s.db[299][0] * ddt_scale);
        let eq33_e1197_d_b1: f64 = (s.db[299][1] * ddt_scale);
        let eq33_e1197_d_b2: f64 = (s.db[299][2] * ddt_scale);
        let eq33_e1197_d_b3: f64 = (s.db[299][3] * ddt_scale);
        let eq33_e1197_d_b4: f64 = (s.db[299][4] * ddt_scale);
        let eq33_e1197_d_b5: f64 = (s.db[299][5] * ddt_scale);
        let eq33_e1197_d_b6: f64 = (s.db[299][6] * ddt_scale);
        let eq33_e1197_d_b7: f64 = (s.db[299][7] * ddt_scale);
        let eq33_e1197_d_b8: f64 = (s.db[299][8] * ddt_scale);
        let eq33_e1197_d_b9: f64 = (s.db[299][9] * ddt_scale);
        let eq33_e1197_d_b10: f64 = (s.db[299][10] * ddt_scale);
        let eq33_e1197_d_b11: f64 = (s.db[299][11] * ddt_scale);
        let eq33_e1198: f64 = (eq33_e1195 * eq33_e1197);
        let eq33_e1198_d_n0: f64 = (eq33_e1195 * eq33_e1197_d_n0);
        let eq33_e1198_d_n1: f64 = (eq33_e1195 * eq33_e1197_d_n1);
        let eq33_e1198_d_n2: f64 = (eq33_e1195 * eq33_e1197_d_n2);
        let eq33_e1198_d_n3: f64 = (eq33_e1195 * eq33_e1197_d_n3);
        let eq33_e1198_d_n4: f64 = (eq33_e1195 * eq33_e1197_d_n4);
        let eq33_e1198_d_n5: f64 = (eq33_e1195 * eq33_e1197_d_n5);
        let eq33_e1198_d_n6: f64 = (eq33_e1195 * eq33_e1197_d_n6);
        let eq33_e1198_d_n7: f64 = (eq33_e1195 * eq33_e1197_d_n7);
        let eq33_e1198_d_n8: f64 = (eq33_e1195 * eq33_e1197_d_n8);
        let eq33_e1198_d_n9: f64 = (eq33_e1195 * eq33_e1197_d_n9);
        let eq33_e1198_d_n10: f64 = (eq33_e1195 * eq33_e1197_d_n10);
        let eq33_e1198_d_n11: f64 = (eq33_e1195 * eq33_e1197_d_n11);
        let eq33_e1198_d_n12: f64 = (eq33_e1195 * eq33_e1197_d_n12);
        let eq33_e1198_d_n13: f64 = (eq33_e1195 * eq33_e1197_d_n13);
        let eq33_e1198_d_n14: f64 = (eq33_e1195 * eq33_e1197_d_n14);
        let eq33_e1198_d_n15: f64 = (eq33_e1195 * eq33_e1197_d_n15);
        let eq33_e1198_d_n16: f64 = (eq33_e1195 * eq33_e1197_d_n16);
        let eq33_e1198_d_n17: f64 = (eq33_e1195 * eq33_e1197_d_n17);
        let eq33_e1198_d_b0: f64 = (eq33_e1195 * eq33_e1197_d_b0);
        let eq33_e1198_d_b1: f64 = (eq33_e1195 * eq33_e1197_d_b1);
        let eq33_e1198_d_b2: f64 = (eq33_e1195 * eq33_e1197_d_b2);
        let eq33_e1198_d_b3: f64 = (eq33_e1195 * eq33_e1197_d_b3);
        let eq33_e1198_d_b4: f64 = (eq33_e1195 * eq33_e1197_d_b4);
        let eq33_e1198_d_b5: f64 = (eq33_e1195 * eq33_e1197_d_b5);
        let eq33_e1198_d_b6: f64 = (eq33_e1195 * eq33_e1197_d_b6);
        let eq33_e1198_d_b7: f64 = (eq33_e1195 * eq33_e1197_d_b7);
        let eq33_e1198_d_b8: f64 = (eq33_e1195 * eq33_e1197_d_b8);
        let eq33_e1198_d_b9: f64 = (eq33_e1195 * eq33_e1197_d_b9);
        let eq33_e1198_d_b10: f64 = (eq33_e1195 * eq33_e1197_d_b10);
        let eq33_e1198_d_b11: f64 = (eq33_e1195 * eq33_e1197_d_b11);
        let eq33_value: f64 = eq33_e1198;
        let eq33_node_derivatives: [f64; 18] = [eq33_e1198_d_n0, eq33_e1198_d_n1, eq33_e1198_d_n2, eq33_e1198_d_n3, eq33_e1198_d_n4, eq33_e1198_d_n5, eq33_e1198_d_n6, eq33_e1198_d_n7, eq33_e1198_d_n8, eq33_e1198_d_n9, eq33_e1198_d_n10, eq33_e1198_d_n11, eq33_e1198_d_n12, eq33_e1198_d_n13, eq33_e1198_d_n14, eq33_e1198_d_n15, eq33_e1198_d_n16, eq33_e1198_d_n17];
        let eq33_branch_derivatives: [f64; 12] = [eq33_e1198_d_b0, eq33_e1198_d_b1, eq33_e1198_d_b2, eq33_e1198_d_b3, eq33_e1198_d_b4, eq33_e1198_d_b5, eq33_e1198_d_b6, eq33_e1198_d_b7, eq33_e1198_d_b8, eq33_e1198_d_b9, eq33_e1198_d_b10, eq33_e1198_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(0),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e1200: f64 = (-p.p87);
        let eq34_e1202: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, s.v[301]);
        let eq34_e1202_d_n0: f64 = (s.dn[301][0] * ddt_scale);
        let eq34_e1202_d_n1: f64 = (s.dn[301][1] * ddt_scale);
        let eq34_e1202_d_n2: f64 = (s.dn[301][2] * ddt_scale);
        let eq34_e1202_d_n3: f64 = (s.dn[301][3] * ddt_scale);
        let eq34_e1202_d_n4: f64 = (s.dn[301][4] * ddt_scale);
        let eq34_e1202_d_n5: f64 = (s.dn[301][5] * ddt_scale);
        let eq34_e1202_d_n6: f64 = (s.dn[301][6] * ddt_scale);
        let eq34_e1202_d_n7: f64 = (s.dn[301][7] * ddt_scale);
        let eq34_e1202_d_n8: f64 = (s.dn[301][8] * ddt_scale);
        let eq34_e1202_d_n9: f64 = (s.dn[301][9] * ddt_scale);
        let eq34_e1202_d_n10: f64 = (s.dn[301][10] * ddt_scale);
        let eq34_e1202_d_n11: f64 = (s.dn[301][11] * ddt_scale);
        let eq34_e1202_d_n12: f64 = (s.dn[301][12] * ddt_scale);
        let eq34_e1202_d_n13: f64 = (s.dn[301][13] * ddt_scale);
        let eq34_e1202_d_n14: f64 = (s.dn[301][14] * ddt_scale);
        let eq34_e1202_d_n15: f64 = (s.dn[301][15] * ddt_scale);
        let eq34_e1202_d_n16: f64 = (s.dn[301][16] * ddt_scale);
        let eq34_e1202_d_n17: f64 = (s.dn[301][17] * ddt_scale);
        let eq34_e1202_d_b0: f64 = (s.db[301][0] * ddt_scale);
        let eq34_e1202_d_b1: f64 = (s.db[301][1] * ddt_scale);
        let eq34_e1202_d_b2: f64 = (s.db[301][2] * ddt_scale);
        let eq34_e1202_d_b3: f64 = (s.db[301][3] * ddt_scale);
        let eq34_e1202_d_b4: f64 = (s.db[301][4] * ddt_scale);
        let eq34_e1202_d_b5: f64 = (s.db[301][5] * ddt_scale);
        let eq34_e1202_d_b6: f64 = (s.db[301][6] * ddt_scale);
        let eq34_e1202_d_b7: f64 = (s.db[301][7] * ddt_scale);
        let eq34_e1202_d_b8: f64 = (s.db[301][8] * ddt_scale);
        let eq34_e1202_d_b9: f64 = (s.db[301][9] * ddt_scale);
        let eq34_e1202_d_b10: f64 = (s.db[301][10] * ddt_scale);
        let eq34_e1202_d_b11: f64 = (s.db[301][11] * ddt_scale);
        let eq34_e1203: f64 = (eq34_e1200 * eq34_e1202);
        let eq34_e1203_d_n0: f64 = (eq34_e1200 * eq34_e1202_d_n0);
        let eq34_e1203_d_n1: f64 = (eq34_e1200 * eq34_e1202_d_n1);
        let eq34_e1203_d_n2: f64 = (eq34_e1200 * eq34_e1202_d_n2);
        let eq34_e1203_d_n3: f64 = (eq34_e1200 * eq34_e1202_d_n3);
        let eq34_e1203_d_n4: f64 = (eq34_e1200 * eq34_e1202_d_n4);
        let eq34_e1203_d_n5: f64 = (eq34_e1200 * eq34_e1202_d_n5);
        let eq34_e1203_d_n6: f64 = (eq34_e1200 * eq34_e1202_d_n6);
        let eq34_e1203_d_n7: f64 = (eq34_e1200 * eq34_e1202_d_n7);
        let eq34_e1203_d_n8: f64 = (eq34_e1200 * eq34_e1202_d_n8);
        let eq34_e1203_d_n9: f64 = (eq34_e1200 * eq34_e1202_d_n9);
        let eq34_e1203_d_n10: f64 = (eq34_e1200 * eq34_e1202_d_n10);
        let eq34_e1203_d_n11: f64 = (eq34_e1200 * eq34_e1202_d_n11);
        let eq34_e1203_d_n12: f64 = (eq34_e1200 * eq34_e1202_d_n12);
        let eq34_e1203_d_n13: f64 = (eq34_e1200 * eq34_e1202_d_n13);
        let eq34_e1203_d_n14: f64 = (eq34_e1200 * eq34_e1202_d_n14);
        let eq34_e1203_d_n15: f64 = (eq34_e1200 * eq34_e1202_d_n15);
        let eq34_e1203_d_n16: f64 = (eq34_e1200 * eq34_e1202_d_n16);
        let eq34_e1203_d_n17: f64 = (eq34_e1200 * eq34_e1202_d_n17);
        let eq34_e1203_d_b0: f64 = (eq34_e1200 * eq34_e1202_d_b0);
        let eq34_e1203_d_b1: f64 = (eq34_e1200 * eq34_e1202_d_b1);
        let eq34_e1203_d_b2: f64 = (eq34_e1200 * eq34_e1202_d_b2);
        let eq34_e1203_d_b3: f64 = (eq34_e1200 * eq34_e1202_d_b3);
        let eq34_e1203_d_b4: f64 = (eq34_e1200 * eq34_e1202_d_b4);
        let eq34_e1203_d_b5: f64 = (eq34_e1200 * eq34_e1202_d_b5);
        let eq34_e1203_d_b6: f64 = (eq34_e1200 * eq34_e1202_d_b6);
        let eq34_e1203_d_b7: f64 = (eq34_e1200 * eq34_e1202_d_b7);
        let eq34_e1203_d_b8: f64 = (eq34_e1200 * eq34_e1202_d_b8);
        let eq34_e1203_d_b9: f64 = (eq34_e1200 * eq34_e1202_d_b9);
        let eq34_e1203_d_b10: f64 = (eq34_e1200 * eq34_e1202_d_b10);
        let eq34_e1203_d_b11: f64 = (eq34_e1200 * eq34_e1202_d_b11);
        let eq34_value: f64 = eq34_e1203;
        let eq34_node_derivatives: [f64; 18] = [eq34_e1203_d_n0, eq34_e1203_d_n1, eq34_e1203_d_n2, eq34_e1203_d_n3, eq34_e1203_d_n4, eq34_e1203_d_n5, eq34_e1203_d_n6, eq34_e1203_d_n7, eq34_e1203_d_n8, eq34_e1203_d_n9, eq34_e1203_d_n10, eq34_e1203_d_n11, eq34_e1203_d_n12, eq34_e1203_d_n13, eq34_e1203_d_n14, eq34_e1203_d_n15, eq34_e1203_d_n16, eq34_e1203_d_n17];
        let eq34_branch_derivatives: [f64; 12] = [eq34_e1203_d_b0, eq34_e1203_d_b1, eq34_e1203_d_b2, eq34_e1203_d_b3, eq34_e1203_d_b4, eq34_e1203_d_b5, eq34_e1203_d_b6, eq34_e1203_d_b7, eq34_e1203_d_b8, eq34_e1203_d_b9, eq34_e1203_d_b10, eq34_e1203_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let eq39_e1229: f64 = (s.v[951] * (nv14 - 0.0));
        let eq39_e1229_d_n0: f64 = (s.dn[951][0] * (nv14 - 0.0));
        let eq39_e1229_d_n1: f64 = (s.dn[951][1] * (nv14 - 0.0));
        let eq39_e1229_d_n2: f64 = (s.dn[951][2] * (nv14 - 0.0));
        let eq39_e1229_d_n3: f64 = (s.dn[951][3] * (nv14 - 0.0));
        let eq39_e1229_d_n4: f64 = (s.dn[951][4] * (nv14 - 0.0));
        let eq39_e1229_d_n5: f64 = (s.dn[951][5] * (nv14 - 0.0));
        let eq39_e1229_d_n6: f64 = (s.dn[951][6] * (nv14 - 0.0));
        let eq39_e1229_d_n7: f64 = (s.dn[951][7] * (nv14 - 0.0));
        let eq39_e1229_d_n8: f64 = (s.dn[951][8] * (nv14 - 0.0));
        let eq39_e1229_d_n9: f64 = (s.dn[951][9] * (nv14 - 0.0));
        let eq39_e1229_d_n10: f64 = (s.dn[951][10] * (nv14 - 0.0));
        let eq39_e1229_d_n11: f64 = (s.dn[951][11] * (nv14 - 0.0));
        let eq39_e1229_d_n12: f64 = (s.dn[951][12] * (nv14 - 0.0));
        let eq39_e1229_d_n13: f64 = (s.dn[951][13] * (nv14 - 0.0));
        let eq39_e1229_d_n14: f64 = ((s.dn[951][14] * (nv14 - 0.0)) + s.v[951]);
        let eq39_e1229_d_n15: f64 = (s.dn[951][15] * (nv14 - 0.0));
        let eq39_e1229_d_n16: f64 = (s.dn[951][16] * (nv14 - 0.0));
        let eq39_e1229_d_n17: f64 = (s.dn[951][17] * (nv14 - 0.0));
        let eq39_e1229_d_b0: f64 = (s.db[951][0] * (nv14 - 0.0));
        let eq39_e1229_d_b1: f64 = (s.db[951][1] * (nv14 - 0.0));
        let eq39_e1229_d_b2: f64 = (s.db[951][2] * (nv14 - 0.0));
        let eq39_e1229_d_b3: f64 = (s.db[951][3] * (nv14 - 0.0));
        let eq39_e1229_d_b4: f64 = (s.db[951][4] * (nv14 - 0.0));
        let eq39_e1229_d_b5: f64 = (s.db[951][5] * (nv14 - 0.0));
        let eq39_e1229_d_b6: f64 = (s.db[951][6] * (nv14 - 0.0));
        let eq39_e1229_d_b7: f64 = (s.db[951][7] * (nv14 - 0.0));
        let eq39_e1229_d_b8: f64 = (s.db[951][8] * (nv14 - 0.0));
        let eq39_e1229_d_b9: f64 = (s.db[951][9] * (nv14 - 0.0));
        let eq39_e1229_d_b10: f64 = (s.db[951][10] * (nv14 - 0.0));
        let eq39_e1229_d_b11: f64 = (s.db[951][11] * (nv14 - 0.0));
        let eq39_value: f64 = eq39_e1229;
        let eq39_node_derivatives: [f64; 18] = [eq39_e1229_d_n0, eq39_e1229_d_n1, eq39_e1229_d_n2, eq39_e1229_d_n3, eq39_e1229_d_n4, eq39_e1229_d_n5, eq39_e1229_d_n6, eq39_e1229_d_n7, eq39_e1229_d_n8, eq39_e1229_d_n9, eq39_e1229_d_n10, eq39_e1229_d_n11, eq39_e1229_d_n12, eq39_e1229_d_n13, eq39_e1229_d_n14, eq39_e1229_d_n15, eq39_e1229_d_n16, eq39_e1229_d_n17];
        let eq39_branch_derivatives: [f64; 12] = [eq39_e1229_d_b0, eq39_e1229_d_b1, eq39_e1229_d_b2, eq39_e1229_d_b3, eq39_e1229_d_b4, eq39_e1229_d_b5, eq39_e1229_d_b6, eq39_e1229_d_b7, eq39_e1229_d_b8, eq39_e1229_d_b9, eq39_e1229_d_b10, eq39_e1229_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let eq40_e1232: f64 = ((nv14 - 0.0) * s.v[954]);
        let eq40_e1232_d_n0: f64 = ((nv14 - 0.0) * s.dn[954][0]);
        let eq40_e1232_d_n1: f64 = ((nv14 - 0.0) * s.dn[954][1]);
        let eq40_e1232_d_n2: f64 = ((nv14 - 0.0) * s.dn[954][2]);
        let eq40_e1232_d_n3: f64 = ((nv14 - 0.0) * s.dn[954][3]);
        let eq40_e1232_d_n4: f64 = ((nv14 - 0.0) * s.dn[954][4]);
        let eq40_e1232_d_n5: f64 = ((nv14 - 0.0) * s.dn[954][5]);
        let eq40_e1232_d_n6: f64 = ((nv14 - 0.0) * s.dn[954][6]);
        let eq40_e1232_d_n7: f64 = ((nv14 - 0.0) * s.dn[954][7]);
        let eq40_e1232_d_n8: f64 = ((nv14 - 0.0) * s.dn[954][8]);
        let eq40_e1232_d_n9: f64 = ((nv14 - 0.0) * s.dn[954][9]);
        let eq40_e1232_d_n10: f64 = ((nv14 - 0.0) * s.dn[954][10]);
        let eq40_e1232_d_n11: f64 = ((nv14 - 0.0) * s.dn[954][11]);
        let eq40_e1232_d_n12: f64 = ((nv14 - 0.0) * s.dn[954][12]);
        let eq40_e1232_d_n13: f64 = ((nv14 - 0.0) * s.dn[954][13]);
        let eq40_e1232_d_n14: f64 = (s.v[954] + ((nv14 - 0.0) * s.dn[954][14]));
        let eq40_e1232_d_n15: f64 = ((nv14 - 0.0) * s.dn[954][15]);
        let eq40_e1232_d_n16: f64 = ((nv14 - 0.0) * s.dn[954][16]);
        let eq40_e1232_d_n17: f64 = ((nv14 - 0.0) * s.dn[954][17]);
        let eq40_e1232_d_b0: f64 = ((nv14 - 0.0) * s.db[954][0]);
        let eq40_e1232_d_b1: f64 = ((nv14 - 0.0) * s.db[954][1]);
        let eq40_e1232_d_b2: f64 = ((nv14 - 0.0) * s.db[954][2]);
        let eq40_e1232_d_b3: f64 = ((nv14 - 0.0) * s.db[954][3]);
        let eq40_e1232_d_b4: f64 = ((nv14 - 0.0) * s.db[954][4]);
        let eq40_e1232_d_b5: f64 = ((nv14 - 0.0) * s.db[954][5]);
        let eq40_e1232_d_b6: f64 = ((nv14 - 0.0) * s.db[954][6]);
        let eq40_e1232_d_b7: f64 = ((nv14 - 0.0) * s.db[954][7]);
        let eq40_e1232_d_b8: f64 = ((nv14 - 0.0) * s.db[954][8]);
        let eq40_e1232_d_b9: f64 = ((nv14 - 0.0) * s.db[954][9]);
        let eq40_e1232_d_b10: f64 = ((nv14 - 0.0) * s.db[954][10]);
        let eq40_e1232_d_b11: f64 = ((nv14 - 0.0) * s.db[954][11]);
        let eq40_e1233: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq40_e1232);
        let eq40_e1233_d_n0: f64 = (eq40_e1232_d_n0 * ddt_scale);
        let eq40_e1233_d_n1: f64 = (eq40_e1232_d_n1 * ddt_scale);
        let eq40_e1233_d_n2: f64 = (eq40_e1232_d_n2 * ddt_scale);
        let eq40_e1233_d_n3: f64 = (eq40_e1232_d_n3 * ddt_scale);
        let eq40_e1233_d_n4: f64 = (eq40_e1232_d_n4 * ddt_scale);
        let eq40_e1233_d_n5: f64 = (eq40_e1232_d_n5 * ddt_scale);
        let eq40_e1233_d_n6: f64 = (eq40_e1232_d_n6 * ddt_scale);
        let eq40_e1233_d_n7: f64 = (eq40_e1232_d_n7 * ddt_scale);
        let eq40_e1233_d_n8: f64 = (eq40_e1232_d_n8 * ddt_scale);
        let eq40_e1233_d_n9: f64 = (eq40_e1232_d_n9 * ddt_scale);
        let eq40_e1233_d_n10: f64 = (eq40_e1232_d_n10 * ddt_scale);
        let eq40_e1233_d_n11: f64 = (eq40_e1232_d_n11 * ddt_scale);
        let eq40_e1233_d_n12: f64 = (eq40_e1232_d_n12 * ddt_scale);
        let eq40_e1233_d_n13: f64 = (eq40_e1232_d_n13 * ddt_scale);
        let eq40_e1233_d_n14: f64 = (eq40_e1232_d_n14 * ddt_scale);
        let eq40_e1233_d_n15: f64 = (eq40_e1232_d_n15 * ddt_scale);
        let eq40_e1233_d_n16: f64 = (eq40_e1232_d_n16 * ddt_scale);
        let eq40_e1233_d_n17: f64 = (eq40_e1232_d_n17 * ddt_scale);
        let eq40_e1233_d_b0: f64 = (eq40_e1232_d_b0 * ddt_scale);
        let eq40_e1233_d_b1: f64 = (eq40_e1232_d_b1 * ddt_scale);
        let eq40_e1233_d_b2: f64 = (eq40_e1232_d_b2 * ddt_scale);
        let eq40_e1233_d_b3: f64 = (eq40_e1232_d_b3 * ddt_scale);
        let eq40_e1233_d_b4: f64 = (eq40_e1232_d_b4 * ddt_scale);
        let eq40_e1233_d_b5: f64 = (eq40_e1232_d_b5 * ddt_scale);
        let eq40_e1233_d_b6: f64 = (eq40_e1232_d_b6 * ddt_scale);
        let eq40_e1233_d_b7: f64 = (eq40_e1232_d_b7 * ddt_scale);
        let eq40_e1233_d_b8: f64 = (eq40_e1232_d_b8 * ddt_scale);
        let eq40_e1233_d_b9: f64 = (eq40_e1232_d_b9 * ddt_scale);
        let eq40_e1233_d_b10: f64 = (eq40_e1232_d_b10 * ddt_scale);
        let eq40_e1233_d_b11: f64 = (eq40_e1232_d_b11 * ddt_scale);
        let eq40_value: f64 = eq40_e1233;
        let eq40_node_derivatives: [f64; 18] = [eq40_e1233_d_n0, eq40_e1233_d_n1, eq40_e1233_d_n2, eq40_e1233_d_n3, eq40_e1233_d_n4, eq40_e1233_d_n5, eq40_e1233_d_n6, eq40_e1233_d_n7, eq40_e1233_d_n8, eq40_e1233_d_n9, eq40_e1233_d_n10, eq40_e1233_d_n11, eq40_e1233_d_n12, eq40_e1233_d_n13, eq40_e1233_d_n14, eq40_e1233_d_n15, eq40_e1233_d_n16, eq40_e1233_d_n17];
        let eq40_branch_derivatives: [f64; 12] = [eq40_e1233_d_b0, eq40_e1233_d_b1, eq40_e1233_d_b2, eq40_e1233_d_b3, eq40_e1233_d_b4, eq40_e1233_d_b5, eq40_e1233_d_b6, eq40_e1233_d_b7, eq40_e1233_d_b8, eq40_e1233_d_b9, eq40_e1233_d_b10, eq40_e1233_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq41_e1236: f64 = ((nv14 - 0.0) * s.v[955]);
        let eq41_e1236_d_n0: f64 = ((nv14 - 0.0) * s.dn[955][0]);
        let eq41_e1236_d_n1: f64 = ((nv14 - 0.0) * s.dn[955][1]);
        let eq41_e1236_d_n2: f64 = ((nv14 - 0.0) * s.dn[955][2]);
        let eq41_e1236_d_n3: f64 = ((nv14 - 0.0) * s.dn[955][3]);
        let eq41_e1236_d_n4: f64 = ((nv14 - 0.0) * s.dn[955][4]);
        let eq41_e1236_d_n5: f64 = ((nv14 - 0.0) * s.dn[955][5]);
        let eq41_e1236_d_n6: f64 = ((nv14 - 0.0) * s.dn[955][6]);
        let eq41_e1236_d_n7: f64 = ((nv14 - 0.0) * s.dn[955][7]);
        let eq41_e1236_d_n8: f64 = ((nv14 - 0.0) * s.dn[955][8]);
        let eq41_e1236_d_n9: f64 = ((nv14 - 0.0) * s.dn[955][9]);
        let eq41_e1236_d_n10: f64 = ((nv14 - 0.0) * s.dn[955][10]);
        let eq41_e1236_d_n11: f64 = ((nv14 - 0.0) * s.dn[955][11]);
        let eq41_e1236_d_n12: f64 = ((nv14 - 0.0) * s.dn[955][12]);
        let eq41_e1236_d_n13: f64 = ((nv14 - 0.0) * s.dn[955][13]);
        let eq41_e1236_d_n14: f64 = (s.v[955] + ((nv14 - 0.0) * s.dn[955][14]));
        let eq41_e1236_d_n15: f64 = ((nv14 - 0.0) * s.dn[955][15]);
        let eq41_e1236_d_n16: f64 = ((nv14 - 0.0) * s.dn[955][16]);
        let eq41_e1236_d_n17: f64 = ((nv14 - 0.0) * s.dn[955][17]);
        let eq41_e1236_d_b0: f64 = ((nv14 - 0.0) * s.db[955][0]);
        let eq41_e1236_d_b1: f64 = ((nv14 - 0.0) * s.db[955][1]);
        let eq41_e1236_d_b2: f64 = ((nv14 - 0.0) * s.db[955][2]);
        let eq41_e1236_d_b3: f64 = ((nv14 - 0.0) * s.db[955][3]);
        let eq41_e1236_d_b4: f64 = ((nv14 - 0.0) * s.db[955][4]);
        let eq41_e1236_d_b5: f64 = ((nv14 - 0.0) * s.db[955][5]);
        let eq41_e1236_d_b6: f64 = ((nv14 - 0.0) * s.db[955][6]);
        let eq41_e1236_d_b7: f64 = ((nv14 - 0.0) * s.db[955][7]);
        let eq41_e1236_d_b8: f64 = ((nv14 - 0.0) * s.db[955][8]);
        let eq41_e1236_d_b9: f64 = ((nv14 - 0.0) * s.db[955][9]);
        let eq41_e1236_d_b10: f64 = ((nv14 - 0.0) * s.db[955][10]);
        let eq41_e1236_d_b11: f64 = ((nv14 - 0.0) * s.db[955][11]);
        let eq41_e1237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, eq41_e1236);
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
        let eq41_value: f64 = eq41_e1237;
        let eq41_node_derivatives: [f64; 18] = [eq41_e1237_d_n0, eq41_e1237_d_n1, eq41_e1237_d_n2, eq41_e1237_d_n3, eq41_e1237_d_n4, eq41_e1237_d_n5, eq41_e1237_d_n6, eq41_e1237_d_n7, eq41_e1237_d_n8, eq41_e1237_d_n9, eq41_e1237_d_n10, eq41_e1237_d_n11, eq41_e1237_d_n12, eq41_e1237_d_n13, eq41_e1237_d_n14, eq41_e1237_d_n15, eq41_e1237_d_n16, eq41_e1237_d_n17];
        let eq41_branch_derivatives: [f64; 12] = [eq41_e1237_d_b0, eq41_e1237_d_b1, eq41_e1237_d_b2, eq41_e1237_d_b3, eq41_e1237_d_b4, eq41_e1237_d_b5, eq41_e1237_d_b6, eq41_e1237_d_b7, eq41_e1237_d_b8, eq41_e1237_d_b9, eq41_e1237_d_b10, eq41_e1237_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let (eq56_e1332, eq56_e1332_d_n0, eq56_e1332_d_n1, eq56_e1332_d_n2, eq56_e1332_d_n3, eq56_e1332_d_n4, eq56_e1332_d_n5, eq56_e1332_d_n6, eq56_e1332_d_n7, eq56_e1332_d_n8, eq56_e1332_d_n9, eq56_e1332_d_n10, eq56_e1332_d_n11, eq56_e1332_d_n12, eq56_e1332_d_n13, eq56_e1332_d_n14, eq56_e1332_d_n15, eq56_e1332_d_n16, eq56_e1332_d_n17, eq56_e1332_d_b0, eq56_e1332_d_b1, eq56_e1332_d_b2, eq56_e1332_d_b3, eq56_e1332_d_b4, eq56_e1332_d_b5, eq56_e1332_d_b6, eq56_e1332_d_b7, eq56_e1332_d_b8, eq56_e1332_d_b9, eq56_e1332_d_b10, eq56_e1332_d_b11,) = {
    if s.b[3409] {
        let eq56_e1330: f64 = (-s.v[802]);
        let eq56_e1330_d_n0: f64 = (-s.dn[802][0]);
        let eq56_e1330_d_n1: f64 = (-s.dn[802][1]);
        let eq56_e1330_d_n2: f64 = (-s.dn[802][2]);
        let eq56_e1330_d_n3: f64 = (-s.dn[802][3]);
        let eq56_e1330_d_n4: f64 = (-s.dn[802][4]);
        let eq56_e1330_d_n5: f64 = (-s.dn[802][5]);
        let eq56_e1330_d_n6: f64 = (-s.dn[802][6]);
        let eq56_e1330_d_n7: f64 = (-s.dn[802][7]);
        let eq56_e1330_d_n8: f64 = (-s.dn[802][8]);
        let eq56_e1330_d_n9: f64 = (-s.dn[802][9]);
        let eq56_e1330_d_n10: f64 = (-s.dn[802][10]);
        let eq56_e1330_d_n11: f64 = (-s.dn[802][11]);
        let eq56_e1330_d_n12: f64 = (-s.dn[802][12]);
        let eq56_e1330_d_n13: f64 = (-s.dn[802][13]);
        let eq56_e1330_d_n14: f64 = (-s.dn[802][14]);
        let eq56_e1330_d_n15: f64 = (-s.dn[802][15]);
        let eq56_e1330_d_n16: f64 = (-s.dn[802][16]);
        let eq56_e1330_d_n17: f64 = (-s.dn[802][17]);
        let eq56_e1330_d_b0: f64 = (-s.db[802][0]);
        let eq56_e1330_d_b1: f64 = (-s.db[802][1]);
        let eq56_e1330_d_b2: f64 = (-s.db[802][2]);
        let eq56_e1330_d_b3: f64 = (-s.db[802][3]);
        let eq56_e1330_d_b4: f64 = (-s.db[802][4]);
        let eq56_e1330_d_b5: f64 = (-s.db[802][5]);
        let eq56_e1330_d_b6: f64 = (-s.db[802][6]);
        let eq56_e1330_d_b7: f64 = (-s.db[802][7]);
        let eq56_e1330_d_b8: f64 = (-s.db[802][8]);
        let eq56_e1330_d_b9: f64 = (-s.db[802][9]);
        let eq56_e1330_d_b10: f64 = (-s.db[802][10]);
        let eq56_e1330_d_b11: f64 = (-s.db[802][11]);
        (eq56_e1330, eq56_e1330_d_n0, eq56_e1330_d_n1, eq56_e1330_d_n2, eq56_e1330_d_n3, eq56_e1330_d_n4, eq56_e1330_d_n5, eq56_e1330_d_n6, eq56_e1330_d_n7, eq56_e1330_d_n8, eq56_e1330_d_n9, eq56_e1330_d_n10, eq56_e1330_d_n11, eq56_e1330_d_n12, eq56_e1330_d_n13, eq56_e1330_d_n14, eq56_e1330_d_n15, eq56_e1330_d_n16, eq56_e1330_d_n17, eq56_e1330_d_b0, eq56_e1330_d_b1, eq56_e1330_d_b2, eq56_e1330_d_b3, eq56_e1330_d_b4, eq56_e1330_d_b5, eq56_e1330_d_b6, eq56_e1330_d_b7, eq56_e1330_d_b8, eq56_e1330_d_b9, eq56_e1330_d_b10, eq56_e1330_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e1332;
        let eq56_node_derivatives: [f64; 18] = [eq56_e1332_d_n0, eq56_e1332_d_n1, eq56_e1332_d_n2, eq56_e1332_d_n3, eq56_e1332_d_n4, eq56_e1332_d_n5, eq56_e1332_d_n6, eq56_e1332_d_n7, eq56_e1332_d_n8, eq56_e1332_d_n9, eq56_e1332_d_n10, eq56_e1332_d_n11, eq56_e1332_d_n12, eq56_e1332_d_n13, eq56_e1332_d_n14, eq56_e1332_d_n15, eq56_e1332_d_n16, eq56_e1332_d_n17];
        let eq56_branch_derivatives: [f64; 12] = [eq56_e1332_d_b0, eq56_e1332_d_b1, eq56_e1332_d_b2, eq56_e1332_d_b3, eq56_e1332_d_b4, eq56_e1332_d_b5, eq56_e1332_d_b6, eq56_e1332_d_b7, eq56_e1332_d_b8, eq56_e1332_d_b9, eq56_e1332_d_b10, eq56_e1332_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq59_e1347, eq59_e1347_d_n0, eq59_e1347_d_n1, eq59_e1347_d_n2, eq59_e1347_d_n3, eq59_e1347_d_n4, eq59_e1347_d_n5, eq59_e1347_d_n6, eq59_e1347_d_n7, eq59_e1347_d_n8, eq59_e1347_d_n9, eq59_e1347_d_n10, eq59_e1347_d_n11, eq59_e1347_d_n12, eq59_e1347_d_n13, eq59_e1347_d_n14, eq59_e1347_d_n15, eq59_e1347_d_n16, eq59_e1347_d_n17, eq59_e1347_d_b0, eq59_e1347_d_b1, eq59_e1347_d_b2, eq59_e1347_d_b3, eq59_e1347_d_b4, eq59_e1347_d_b5, eq59_e1347_d_b6, eq59_e1347_d_b7, eq59_e1347_d_b8, eq59_e1347_d_b9, eq59_e1347_d_b10, eq59_e1347_d_b11,) = {
    if (p.p28 != 0.0) {
        (s.v[749], s.dn[749][0], s.dn[749][1], s.dn[749][2], s.dn[749][3], s.dn[749][4], s.dn[749][5], s.dn[749][6], s.dn[749][7], s.dn[749][8], s.dn[749][9], s.dn[749][10], s.dn[749][11], s.dn[749][12], s.dn[749][13], s.dn[749][14], s.dn[749][15], s.dn[749][16], s.dn[749][17], s.db[749][0], s.db[749][1], s.db[749][2], s.db[749][3], s.db[749][4], s.db[749][5], s.db[749][6], s.db[749][7], s.db[749][8], s.db[749][9], s.db[749][10], s.db[749][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1347;
        let eq59_node_derivatives: [f64; 18] = [eq59_e1347_d_n0, eq59_e1347_d_n1, eq59_e1347_d_n2, eq59_e1347_d_n3, eq59_e1347_d_n4, eq59_e1347_d_n5, eq59_e1347_d_n6, eq59_e1347_d_n7, eq59_e1347_d_n8, eq59_e1347_d_n9, eq59_e1347_d_n10, eq59_e1347_d_n11, eq59_e1347_d_n12, eq59_e1347_d_n13, eq59_e1347_d_n14, eq59_e1347_d_n15, eq59_e1347_d_n16, eq59_e1347_d_n17];
        let eq59_branch_derivatives: [f64; 12] = [eq59_e1347_d_b0, eq59_e1347_d_b1, eq59_e1347_d_b2, eq59_e1347_d_b3, eq59_e1347_d_b4, eq59_e1347_d_b5, eq59_e1347_d_b6, eq59_e1347_d_b7, eq59_e1347_d_b8, eq59_e1347_d_b9, eq59_e1347_d_b10, eq59_e1347_d_b11];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e1351, eq60_e1351_d_n0, eq60_e1351_d_n1, eq60_e1351_d_n2, eq60_e1351_d_n3, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n13, eq60_e1351_d_n14, eq60_e1351_d_n15, eq60_e1351_d_n16, eq60_e1351_d_n17, eq60_e1351_d_b0, eq60_e1351_d_b1, eq60_e1351_d_b2, eq60_e1351_d_b3, eq60_e1351_d_b4, eq60_e1351_d_b5, eq60_e1351_d_b6, eq60_e1351_d_b7, eq60_e1351_d_b8, eq60_e1351_d_b9, eq60_e1351_d_b10, eq60_e1351_d_b11,) = {
    if (p.p28 != 0.0) {
        (s.v[750], s.dn[750][0], s.dn[750][1], s.dn[750][2], s.dn[750][3], s.dn[750][4], s.dn[750][5], s.dn[750][6], s.dn[750][7], s.dn[750][8], s.dn[750][9], s.dn[750][10], s.dn[750][11], s.dn[750][12], s.dn[750][13], s.dn[750][14], s.dn[750][15], s.dn[750][16], s.dn[750][17], s.db[750][0], s.db[750][1], s.db[750][2], s.db[750][3], s.db[750][4], s.db[750][5], s.db[750][6], s.db[750][7], s.db[750][8], s.db[750][9], s.db[750][10], s.db[750][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1351;
        let eq60_node_derivatives: [f64; 18] = [eq60_e1351_d_n0, eq60_e1351_d_n1, eq60_e1351_d_n2, eq60_e1351_d_n3, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n13, eq60_e1351_d_n14, eq60_e1351_d_n15, eq60_e1351_d_n16, eq60_e1351_d_n17];
        let eq60_branch_derivatives: [f64; 12] = [eq60_e1351_d_b0, eq60_e1351_d_b1, eq60_e1351_d_b2, eq60_e1351_d_b3, eq60_e1351_d_b4, eq60_e1351_d_b5, eq60_e1351_d_b6, eq60_e1351_d_b7, eq60_e1351_d_b8, eq60_e1351_d_b9, eq60_e1351_d_b10, eq60_e1351_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq60_value),
            &eq60_node_derivatives,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1358, eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17, eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8, eq61_e1358_d_b9, eq61_e1358_d_b10, eq61_e1358_d_b11,) = {
    if (p.p28 != 0.0) {
        let eq61_e1355: f64 = (s.v[800] * (nv11 - 0.0));
        let eq61_e1355_d_n0: f64 = (s.dn[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_n1: f64 = (s.dn[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_n2: f64 = (s.dn[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_n3: f64 = (s.dn[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_n4: f64 = (s.dn[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_n5: f64 = (s.dn[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_n6: f64 = (s.dn[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_n7: f64 = (s.dn[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_n8: f64 = (s.dn[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_n9: f64 = (s.dn[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_n10: f64 = (s.dn[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_n11: f64 = ((s.dn[800][11] * (nv11 - 0.0)) + s.v[800]);
        let eq61_e1355_d_n12: f64 = (s.dn[800][12] * (nv11 - 0.0));
        let eq61_e1355_d_n13: f64 = (s.dn[800][13] * (nv11 - 0.0));
        let eq61_e1355_d_n14: f64 = (s.dn[800][14] * (nv11 - 0.0));
        let eq61_e1355_d_n15: f64 = (s.dn[800][15] * (nv11 - 0.0));
        let eq61_e1355_d_n16: f64 = (s.dn[800][16] * (nv11 - 0.0));
        let eq61_e1355_d_n17: f64 = (s.dn[800][17] * (nv11 - 0.0));
        let eq61_e1355_d_b0: f64 = (s.db[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_b1: f64 = (s.db[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_b2: f64 = (s.db[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_b3: f64 = (s.db[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_b4: f64 = (s.db[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_b5: f64 = (s.db[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_b6: f64 = (s.db[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_b7: f64 = (s.db[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_b8: f64 = (s.db[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_b9: f64 = (s.db[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_b10: f64 = (s.db[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_b11: f64 = (s.db[800][11] * (nv11 - 0.0));
        let eq61_e1356: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, eq61_e1355);
        let eq61_e1356_d_n0: f64 = (eq61_e1355_d_n0 * ddt_scale);
        let eq61_e1356_d_n1: f64 = (eq61_e1355_d_n1 * ddt_scale);
        let eq61_e1356_d_n2: f64 = (eq61_e1355_d_n2 * ddt_scale);
        let eq61_e1356_d_n3: f64 = (eq61_e1355_d_n3 * ddt_scale);
        let eq61_e1356_d_n4: f64 = (eq61_e1355_d_n4 * ddt_scale);
        let eq61_e1356_d_n5: f64 = (eq61_e1355_d_n5 * ddt_scale);
        let eq61_e1356_d_n6: f64 = (eq61_e1355_d_n6 * ddt_scale);
        let eq61_e1356_d_n7: f64 = (eq61_e1355_d_n7 * ddt_scale);
        let eq61_e1356_d_n8: f64 = (eq61_e1355_d_n8 * ddt_scale);
        let eq61_e1356_d_n9: f64 = (eq61_e1355_d_n9 * ddt_scale);
        let eq61_e1356_d_n10: f64 = (eq61_e1355_d_n10 * ddt_scale);
        let eq61_e1356_d_n11: f64 = (eq61_e1355_d_n11 * ddt_scale);
        let eq61_e1356_d_n12: f64 = (eq61_e1355_d_n12 * ddt_scale);
        let eq61_e1356_d_n13: f64 = (eq61_e1355_d_n13 * ddt_scale);
        let eq61_e1356_d_n14: f64 = (eq61_e1355_d_n14 * ddt_scale);
        let eq61_e1356_d_n15: f64 = (eq61_e1355_d_n15 * ddt_scale);
        let eq61_e1356_d_n16: f64 = (eq61_e1355_d_n16 * ddt_scale);
        let eq61_e1356_d_n17: f64 = (eq61_e1355_d_n17 * ddt_scale);
        let eq61_e1356_d_b0: f64 = (eq61_e1355_d_b0 * ddt_scale);
        let eq61_e1356_d_b1: f64 = (eq61_e1355_d_b1 * ddt_scale);
        let eq61_e1356_d_b2: f64 = (eq61_e1355_d_b2 * ddt_scale);
        let eq61_e1356_d_b3: f64 = (eq61_e1355_d_b3 * ddt_scale);
        let eq61_e1356_d_b4: f64 = (eq61_e1355_d_b4 * ddt_scale);
        let eq61_e1356_d_b5: f64 = (eq61_e1355_d_b5 * ddt_scale);
        let eq61_e1356_d_b6: f64 = (eq61_e1355_d_b6 * ddt_scale);
        let eq61_e1356_d_b7: f64 = (eq61_e1355_d_b7 * ddt_scale);
        let eq61_e1356_d_b8: f64 = (eq61_e1355_d_b8 * ddt_scale);
        let eq61_e1356_d_b9: f64 = (eq61_e1355_d_b9 * ddt_scale);
        let eq61_e1356_d_b10: f64 = (eq61_e1355_d_b10 * ddt_scale);
        let eq61_e1356_d_b11: f64 = (eq61_e1355_d_b11 * ddt_scale);
        (eq61_e1356, eq61_e1356_d_n0, eq61_e1356_d_n1, eq61_e1356_d_n2, eq61_e1356_d_n3, eq61_e1356_d_n4, eq61_e1356_d_n5, eq61_e1356_d_n6, eq61_e1356_d_n7, eq61_e1356_d_n8, eq61_e1356_d_n9, eq61_e1356_d_n10, eq61_e1356_d_n11, eq61_e1356_d_n12, eq61_e1356_d_n13, eq61_e1356_d_n14, eq61_e1356_d_n15, eq61_e1356_d_n16, eq61_e1356_d_n17, eq61_e1356_d_b0, eq61_e1356_d_b1, eq61_e1356_d_b2, eq61_e1356_d_b3, eq61_e1356_d_b4, eq61_e1356_d_b5, eq61_e1356_d_b6, eq61_e1356_d_b7, eq61_e1356_d_b8, eq61_e1356_d_b9, eq61_e1356_d_b10, eq61_e1356_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1358;
        let eq61_node_derivatives: [f64; 18] = [eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17];
        let eq61_branch_derivatives: [f64; 12] = [eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8, eq61_e1358_d_b9, eq61_e1358_d_b10, eq61_e1358_d_b11];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1365, eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17, eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8, eq62_e1365_d_b9, eq62_e1365_d_b10, eq62_e1365_d_b11,) = {
    if (p.p28 != 0.0) {
        let eq62_e1362: f64 = (s.v[801] * (nv12 - 0.0));
        let eq62_e1362_d_n0: f64 = (s.dn[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_n1: f64 = (s.dn[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_n2: f64 = (s.dn[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_n3: f64 = (s.dn[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_n4: f64 = (s.dn[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_n5: f64 = (s.dn[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_n6: f64 = (s.dn[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_n7: f64 = (s.dn[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_n8: f64 = (s.dn[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_n9: f64 = (s.dn[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_n10: f64 = (s.dn[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_n11: f64 = (s.dn[801][11] * (nv12 - 0.0));
        let eq62_e1362_d_n12: f64 = ((s.dn[801][12] * (nv12 - 0.0)) + s.v[801]);
        let eq62_e1362_d_n13: f64 = (s.dn[801][13] * (nv12 - 0.0));
        let eq62_e1362_d_n14: f64 = (s.dn[801][14] * (nv12 - 0.0));
        let eq62_e1362_d_n15: f64 = (s.dn[801][15] * (nv12 - 0.0));
        let eq62_e1362_d_n16: f64 = (s.dn[801][16] * (nv12 - 0.0));
        let eq62_e1362_d_n17: f64 = (s.dn[801][17] * (nv12 - 0.0));
        let eq62_e1362_d_b0: f64 = (s.db[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_b1: f64 = (s.db[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_b2: f64 = (s.db[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_b3: f64 = (s.db[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_b4: f64 = (s.db[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_b5: f64 = (s.db[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_b6: f64 = (s.db[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_b7: f64 = (s.db[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_b8: f64 = (s.db[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_b9: f64 = (s.db[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_b10: f64 = (s.db[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_b11: f64 = (s.db[801][11] * (nv12 - 0.0));
        let eq62_e1363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, eq62_e1362);
        let eq62_e1363_d_n0: f64 = (eq62_e1362_d_n0 * ddt_scale);
        let eq62_e1363_d_n1: f64 = (eq62_e1362_d_n1 * ddt_scale);
        let eq62_e1363_d_n2: f64 = (eq62_e1362_d_n2 * ddt_scale);
        let eq62_e1363_d_n3: f64 = (eq62_e1362_d_n3 * ddt_scale);
        let eq62_e1363_d_n4: f64 = (eq62_e1362_d_n4 * ddt_scale);
        let eq62_e1363_d_n5: f64 = (eq62_e1362_d_n5 * ddt_scale);
        let eq62_e1363_d_n6: f64 = (eq62_e1362_d_n6 * ddt_scale);
        let eq62_e1363_d_n7: f64 = (eq62_e1362_d_n7 * ddt_scale);
        let eq62_e1363_d_n8: f64 = (eq62_e1362_d_n8 * ddt_scale);
        let eq62_e1363_d_n9: f64 = (eq62_e1362_d_n9 * ddt_scale);
        let eq62_e1363_d_n10: f64 = (eq62_e1362_d_n10 * ddt_scale);
        let eq62_e1363_d_n11: f64 = (eq62_e1362_d_n11 * ddt_scale);
        let eq62_e1363_d_n12: f64 = (eq62_e1362_d_n12 * ddt_scale);
        let eq62_e1363_d_n13: f64 = (eq62_e1362_d_n13 * ddt_scale);
        let eq62_e1363_d_n14: f64 = (eq62_e1362_d_n14 * ddt_scale);
        let eq62_e1363_d_n15: f64 = (eq62_e1362_d_n15 * ddt_scale);
        let eq62_e1363_d_n16: f64 = (eq62_e1362_d_n16 * ddt_scale);
        let eq62_e1363_d_n17: f64 = (eq62_e1362_d_n17 * ddt_scale);
        let eq62_e1363_d_b0: f64 = (eq62_e1362_d_b0 * ddt_scale);
        let eq62_e1363_d_b1: f64 = (eq62_e1362_d_b1 * ddt_scale);
        let eq62_e1363_d_b2: f64 = (eq62_e1362_d_b2 * ddt_scale);
        let eq62_e1363_d_b3: f64 = (eq62_e1362_d_b3 * ddt_scale);
        let eq62_e1363_d_b4: f64 = (eq62_e1362_d_b4 * ddt_scale);
        let eq62_e1363_d_b5: f64 = (eq62_e1362_d_b5 * ddt_scale);
        let eq62_e1363_d_b6: f64 = (eq62_e1362_d_b6 * ddt_scale);
        let eq62_e1363_d_b7: f64 = (eq62_e1362_d_b7 * ddt_scale);
        let eq62_e1363_d_b8: f64 = (eq62_e1362_d_b8 * ddt_scale);
        let eq62_e1363_d_b9: f64 = (eq62_e1362_d_b9 * ddt_scale);
        let eq62_e1363_d_b10: f64 = (eq62_e1362_d_b10 * ddt_scale);
        let eq62_e1363_d_b11: f64 = (eq62_e1362_d_b11 * ddt_scale);
        (eq62_e1363, eq62_e1363_d_n0, eq62_e1363_d_n1, eq62_e1363_d_n2, eq62_e1363_d_n3, eq62_e1363_d_n4, eq62_e1363_d_n5, eq62_e1363_d_n6, eq62_e1363_d_n7, eq62_e1363_d_n8, eq62_e1363_d_n9, eq62_e1363_d_n10, eq62_e1363_d_n11, eq62_e1363_d_n12, eq62_e1363_d_n13, eq62_e1363_d_n14, eq62_e1363_d_n15, eq62_e1363_d_n16, eq62_e1363_d_n17, eq62_e1363_d_b0, eq62_e1363_d_b1, eq62_e1363_d_b2, eq62_e1363_d_b3, eq62_e1363_d_b4, eq62_e1363_d_b5, eq62_e1363_d_b6, eq62_e1363_d_b7, eq62_e1363_d_b8, eq62_e1363_d_b9, eq62_e1363_d_b10, eq62_e1363_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1365;
        let eq62_node_derivatives: [f64; 18] = [eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17];
        let eq62_branch_derivatives: [f64; 12] = [eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8, eq62_e1365_d_b9, eq62_e1365_d_b10, eq62_e1365_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq65_e1379, eq65_e1379_d_n0, eq65_e1379_d_n1, eq65_e1379_d_n2, eq65_e1379_d_n3, eq65_e1379_d_n4, eq65_e1379_d_n5, eq65_e1379_d_n6, eq65_e1379_d_n7, eq65_e1379_d_n8, eq65_e1379_d_n9, eq65_e1379_d_n10, eq65_e1379_d_n11, eq65_e1379_d_n12, eq65_e1379_d_n13, eq65_e1379_d_n14, eq65_e1379_d_n15, eq65_e1379_d_n16, eq65_e1379_d_n17, eq65_e1379_d_b0, eq65_e1379_d_b1, eq65_e1379_d_b2, eq65_e1379_d_b3, eq65_e1379_d_b4, eq65_e1379_d_b5, eq65_e1379_d_b6, eq65_e1379_d_b7, eq65_e1379_d_b8, eq65_e1379_d_b9, eq65_e1379_d_b10, eq65_e1379_d_b11,) = {
    if (p.p29 != 0.0) {
        (s.v[815], s.dn[815][0], s.dn[815][1], s.dn[815][2], s.dn[815][3], s.dn[815][4], s.dn[815][5], s.dn[815][6], s.dn[815][7], s.dn[815][8], s.dn[815][9], s.dn[815][10], s.dn[815][11], s.dn[815][12], s.dn[815][13], s.dn[815][14], s.dn[815][15], s.dn[815][16], s.dn[815][17], s.db[815][0], s.db[815][1], s.db[815][2], s.db[815][3], s.db[815][4], s.db[815][5], s.db[815][6], s.db[815][7], s.db[815][8], s.db[815][9], s.db[815][10], s.db[815][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1379;
        let eq65_node_derivatives: [f64; 18] = [eq65_e1379_d_n0, eq65_e1379_d_n1, eq65_e1379_d_n2, eq65_e1379_d_n3, eq65_e1379_d_n4, eq65_e1379_d_n5, eq65_e1379_d_n6, eq65_e1379_d_n7, eq65_e1379_d_n8, eq65_e1379_d_n9, eq65_e1379_d_n10, eq65_e1379_d_n11, eq65_e1379_d_n12, eq65_e1379_d_n13, eq65_e1379_d_n14, eq65_e1379_d_n15, eq65_e1379_d_n16, eq65_e1379_d_n17];
        let eq65_branch_derivatives: [f64; 12] = [eq65_e1379_d_b0, eq65_e1379_d_b1, eq65_e1379_d_b2, eq65_e1379_d_b3, eq65_e1379_d_b4, eq65_e1379_d_b5, eq65_e1379_d_b6, eq65_e1379_d_b7, eq65_e1379_d_b8, eq65_e1379_d_b9, eq65_e1379_d_b10, eq65_e1379_d_b11];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1384, eq66_e1384_d_n13,) = {
    if (p.p29 != 0.0) {
        let eq66_e1382: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, (nv13 - 0.0));
        (eq66_e1382, ddt_scale,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1384;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq66_value),
            13,
            multiplicity * (eq66_e1384_d_n13),
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
        let (eq0_e1018, eq0_e1018_d_n0, eq0_e1018_d_n1, eq0_e1018_d_n2, eq0_e1018_d_n3, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n11, eq0_e1018_d_n12, eq0_e1018_d_n13, eq0_e1018_d_n14, eq0_e1018_d_n15, eq0_e1018_d_n16, eq0_e1018_d_n17, eq0_e1018_d_b0, eq0_e1018_d_b1, eq0_e1018_d_b2, eq0_e1018_d_b3, eq0_e1018_d_b4, eq0_e1018_d_b5, eq0_e1018_d_b6, eq0_e1018_d_b7, eq0_e1018_d_b8, eq0_e1018_d_b9, eq0_e1018_d_b10, eq0_e1018_d_b11, eq0_e1018_q, eq0_e1018_q_d_n0, eq0_e1018_q_d_n1, eq0_e1018_q_d_n2, eq0_e1018_q_d_n3, eq0_e1018_q_d_n4, eq0_e1018_q_d_n5, eq0_e1018_q_d_n6, eq0_e1018_q_d_n7, eq0_e1018_q_d_n8, eq0_e1018_q_d_n9, eq0_e1018_q_d_n10, eq0_e1018_q_d_n11, eq0_e1018_q_d_n12, eq0_e1018_q_d_n13, eq0_e1018_q_d_n14, eq0_e1018_q_d_n15, eq0_e1018_q_d_n16, eq0_e1018_q_d_n17, eq0_e1018_q_d_b0, eq0_e1018_q_d_b1, eq0_e1018_q_d_b2, eq0_e1018_q_d_b3, eq0_e1018_q_d_b4, eq0_e1018_q_d_b5, eq0_e1018_q_d_b6, eq0_e1018_q_d_b7, eq0_e1018_q_d_b8, eq0_e1018_q_d_b9, eq0_e1018_q_d_b10, eq0_e1018_q_d_b11,) = {
    if s.b[3305] {
        let eq0_e1015_q: f64 = s.v[924];
        let eq0_e1016: f64 = (s.v[926] + s.v[924]);
        let eq0_e1016_d_n0: f64 = (s.dn[926][0] + s.dn[924][0]);
        let eq0_e1016_d_n1: f64 = (s.dn[926][1] + s.dn[924][1]);
        let eq0_e1016_d_n2: f64 = (s.dn[926][2] + s.dn[924][2]);
        let eq0_e1016_d_n3: f64 = (s.dn[926][3] + s.dn[924][3]);
        let eq0_e1016_d_n4: f64 = (s.dn[926][4] + s.dn[924][4]);
        let eq0_e1016_d_n5: f64 = (s.dn[926][5] + s.dn[924][5]);
        let eq0_e1016_d_n6: f64 = (s.dn[926][6] + s.dn[924][6]);
        let eq0_e1016_d_n7: f64 = (s.dn[926][7] + s.dn[924][7]);
        let eq0_e1016_d_n8: f64 = (s.dn[926][8] + s.dn[924][8]);
        let eq0_e1016_d_n9: f64 = (s.dn[926][9] + s.dn[924][9]);
        let eq0_e1016_d_n10: f64 = (s.dn[926][10] + s.dn[924][10]);
        let eq0_e1016_d_n11: f64 = (s.dn[926][11] + s.dn[924][11]);
        let eq0_e1016_d_n12: f64 = (s.dn[926][12] + s.dn[924][12]);
        let eq0_e1016_d_n13: f64 = (s.dn[926][13] + s.dn[924][13]);
        let eq0_e1016_d_n14: f64 = (s.dn[926][14] + s.dn[924][14]);
        let eq0_e1016_d_n15: f64 = (s.dn[926][15] + s.dn[924][15]);
        let eq0_e1016_d_n16: f64 = (s.dn[926][16] + s.dn[924][16]);
        let eq0_e1016_d_n17: f64 = (s.dn[926][17] + s.dn[924][17]);
        let eq0_e1016_d_b0: f64 = (s.db[926][0] + s.db[924][0]);
        let eq0_e1016_d_b1: f64 = (s.db[926][1] + s.db[924][1]);
        let eq0_e1016_d_b2: f64 = (s.db[926][2] + s.db[924][2]);
        let eq0_e1016_d_b3: f64 = (s.db[926][3] + s.db[924][3]);
        let eq0_e1016_d_b4: f64 = (s.db[926][4] + s.db[924][4]);
        let eq0_e1016_d_b5: f64 = (s.db[926][5] + s.db[924][5]);
        let eq0_e1016_d_b6: f64 = (s.db[926][6] + s.db[924][6]);
        let eq0_e1016_d_b7: f64 = (s.db[926][7] + s.db[924][7]);
        let eq0_e1016_d_b8: f64 = (s.db[926][8] + s.db[924][8]);
        let eq0_e1016_d_b9: f64 = (s.db[926][9] + s.db[924][9]);
        let eq0_e1016_d_b10: f64 = (s.db[926][10] + s.db[924][10]);
        let eq0_e1016_d_b11: f64 = (s.db[926][11] + s.db[924][11]);
        let eq0_e1016_q: f64 = eq0_e1015_q;
        (eq0_e1016, eq0_e1016_d_n0, eq0_e1016_d_n1, eq0_e1016_d_n2, eq0_e1016_d_n3, eq0_e1016_d_n4, eq0_e1016_d_n5, eq0_e1016_d_n6, eq0_e1016_d_n7, eq0_e1016_d_n8, eq0_e1016_d_n9, eq0_e1016_d_n10, eq0_e1016_d_n11, eq0_e1016_d_n12, eq0_e1016_d_n13, eq0_e1016_d_n14, eq0_e1016_d_n15, eq0_e1016_d_n16, eq0_e1016_d_n17, eq0_e1016_d_b0, eq0_e1016_d_b1, eq0_e1016_d_b2, eq0_e1016_d_b3, eq0_e1016_d_b4, eq0_e1016_d_b5, eq0_e1016_d_b6, eq0_e1016_d_b7, eq0_e1016_d_b8, eq0_e1016_d_b9, eq0_e1016_d_b10, eq0_e1016_d_b11, eq0_e1016_q, s.dn[924][0], s.dn[924][1], s.dn[924][2], s.dn[924][3], s.dn[924][4], s.dn[924][5], s.dn[924][6], s.dn[924][7], s.dn[924][8], s.dn[924][9], s.dn[924][10], s.dn[924][11], s.dn[924][12], s.dn[924][13], s.dn[924][14], s.dn[924][15], s.dn[924][16], s.dn[924][17], s.db[924][0], s.db[924][1], s.db[924][2], s.db[924][3], s.db[924][4], s.db[924][5], s.db[924][6], s.db[924][7], s.db[924][8], s.db[924][9], s.db[924][10], s.db[924][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_reactive_node_derivatives: [f64; 18] = [eq0_e1018_q_d_n0, eq0_e1018_q_d_n1, eq0_e1018_q_d_n2, eq0_e1018_q_d_n3, eq0_e1018_q_d_n4, eq0_e1018_q_d_n5, eq0_e1018_q_d_n6, eq0_e1018_q_d_n7, eq0_e1018_q_d_n8, eq0_e1018_q_d_n9, eq0_e1018_q_d_n10, eq0_e1018_q_d_n11, eq0_e1018_q_d_n12, eq0_e1018_q_d_n13, eq0_e1018_q_d_n14, eq0_e1018_q_d_n15, eq0_e1018_q_d_n16, eq0_e1018_q_d_n17];
        let eq0_reactive_branch_derivatives: [f64; 12] = [eq0_e1018_q_d_b0, eq0_e1018_q_d_b1, eq0_e1018_q_d_b2, eq0_e1018_q_d_b3, eq0_e1018_q_d_b4, eq0_e1018_q_d_b5, eq0_e1018_q_d_b6, eq0_e1018_q_d_b7, eq0_e1018_q_d_b8, eq0_e1018_q_d_b9, eq0_e1018_q_d_b10, eq0_e1018_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq0_reactive_node_derivatives,
            branches,
            &eq0_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1025, eq1_e1025_d_n0, eq1_e1025_d_n1, eq1_e1025_d_n2, eq1_e1025_d_n3, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n11, eq1_e1025_d_n12, eq1_e1025_d_n13, eq1_e1025_d_n14, eq1_e1025_d_n15, eq1_e1025_d_n16, eq1_e1025_d_n17, eq1_e1025_d_b0, eq1_e1025_d_b1, eq1_e1025_d_b2, eq1_e1025_d_b3, eq1_e1025_d_b4, eq1_e1025_d_b5, eq1_e1025_d_b6, eq1_e1025_d_b7, eq1_e1025_d_b8, eq1_e1025_d_b9, eq1_e1025_d_b10, eq1_e1025_d_b11, eq1_e1025_q, eq1_e1025_q_d_n0, eq1_e1025_q_d_n1, eq1_e1025_q_d_n2, eq1_e1025_q_d_n3, eq1_e1025_q_d_n4, eq1_e1025_q_d_n5, eq1_e1025_q_d_n6, eq1_e1025_q_d_n7, eq1_e1025_q_d_n8, eq1_e1025_q_d_n9, eq1_e1025_q_d_n10, eq1_e1025_q_d_n11, eq1_e1025_q_d_n12, eq1_e1025_q_d_n13, eq1_e1025_q_d_n14, eq1_e1025_q_d_n15, eq1_e1025_q_d_n16, eq1_e1025_q_d_n17, eq1_e1025_q_d_b0, eq1_e1025_q_d_b1, eq1_e1025_q_d_b2, eq1_e1025_q_d_b3, eq1_e1025_q_d_b4, eq1_e1025_q_d_b5, eq1_e1025_q_d_b6, eq1_e1025_q_d_b7, eq1_e1025_q_d_b8, eq1_e1025_q_d_b9, eq1_e1025_q_d_b10, eq1_e1025_q_d_b11,) = {
    if s.b[3305] {
        let eq1_e1022_q: f64 = s.v[925];
        let eq1_e1023: f64 = (s.v[927] + s.v[925]);
        let eq1_e1023_d_n0: f64 = (s.dn[927][0] + s.dn[925][0]);
        let eq1_e1023_d_n1: f64 = (s.dn[927][1] + s.dn[925][1]);
        let eq1_e1023_d_n2: f64 = (s.dn[927][2] + s.dn[925][2]);
        let eq1_e1023_d_n3: f64 = (s.dn[927][3] + s.dn[925][3]);
        let eq1_e1023_d_n4: f64 = (s.dn[927][4] + s.dn[925][4]);
        let eq1_e1023_d_n5: f64 = (s.dn[927][5] + s.dn[925][5]);
        let eq1_e1023_d_n6: f64 = (s.dn[927][6] + s.dn[925][6]);
        let eq1_e1023_d_n7: f64 = (s.dn[927][7] + s.dn[925][7]);
        let eq1_e1023_d_n8: f64 = (s.dn[927][8] + s.dn[925][8]);
        let eq1_e1023_d_n9: f64 = (s.dn[927][9] + s.dn[925][9]);
        let eq1_e1023_d_n10: f64 = (s.dn[927][10] + s.dn[925][10]);
        let eq1_e1023_d_n11: f64 = (s.dn[927][11] + s.dn[925][11]);
        let eq1_e1023_d_n12: f64 = (s.dn[927][12] + s.dn[925][12]);
        let eq1_e1023_d_n13: f64 = (s.dn[927][13] + s.dn[925][13]);
        let eq1_e1023_d_n14: f64 = (s.dn[927][14] + s.dn[925][14]);
        let eq1_e1023_d_n15: f64 = (s.dn[927][15] + s.dn[925][15]);
        let eq1_e1023_d_n16: f64 = (s.dn[927][16] + s.dn[925][16]);
        let eq1_e1023_d_n17: f64 = (s.dn[927][17] + s.dn[925][17]);
        let eq1_e1023_d_b0: f64 = (s.db[927][0] + s.db[925][0]);
        let eq1_e1023_d_b1: f64 = (s.db[927][1] + s.db[925][1]);
        let eq1_e1023_d_b2: f64 = (s.db[927][2] + s.db[925][2]);
        let eq1_e1023_d_b3: f64 = (s.db[927][3] + s.db[925][3]);
        let eq1_e1023_d_b4: f64 = (s.db[927][4] + s.db[925][4]);
        let eq1_e1023_d_b5: f64 = (s.db[927][5] + s.db[925][5]);
        let eq1_e1023_d_b6: f64 = (s.db[927][6] + s.db[925][6]);
        let eq1_e1023_d_b7: f64 = (s.db[927][7] + s.db[925][7]);
        let eq1_e1023_d_b8: f64 = (s.db[927][8] + s.db[925][8]);
        let eq1_e1023_d_b9: f64 = (s.db[927][9] + s.db[925][9]);
        let eq1_e1023_d_b10: f64 = (s.db[927][10] + s.db[925][10]);
        let eq1_e1023_d_b11: f64 = (s.db[927][11] + s.db[925][11]);
        let eq1_e1023_q: f64 = eq1_e1022_q;
        (eq1_e1023, eq1_e1023_d_n0, eq1_e1023_d_n1, eq1_e1023_d_n2, eq1_e1023_d_n3, eq1_e1023_d_n4, eq1_e1023_d_n5, eq1_e1023_d_n6, eq1_e1023_d_n7, eq1_e1023_d_n8, eq1_e1023_d_n9, eq1_e1023_d_n10, eq1_e1023_d_n11, eq1_e1023_d_n12, eq1_e1023_d_n13, eq1_e1023_d_n14, eq1_e1023_d_n15, eq1_e1023_d_n16, eq1_e1023_d_n17, eq1_e1023_d_b0, eq1_e1023_d_b1, eq1_e1023_d_b2, eq1_e1023_d_b3, eq1_e1023_d_b4, eq1_e1023_d_b5, eq1_e1023_d_b6, eq1_e1023_d_b7, eq1_e1023_d_b8, eq1_e1023_d_b9, eq1_e1023_d_b10, eq1_e1023_d_b11, eq1_e1023_q, s.dn[925][0], s.dn[925][1], s.dn[925][2], s.dn[925][3], s.dn[925][4], s.dn[925][5], s.dn[925][6], s.dn[925][7], s.dn[925][8], s.dn[925][9], s.dn[925][10], s.dn[925][11], s.dn[925][12], s.dn[925][13], s.dn[925][14], s.dn[925][15], s.dn[925][16], s.dn[925][17], s.db[925][0], s.db[925][1], s.db[925][2], s.db[925][3], s.db[925][4], s.db[925][5], s.db[925][6], s.db[925][7], s.db[925][8], s.db[925][9], s.db[925][10], s.db[925][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_reactive_node_derivatives: [f64; 18] = [eq1_e1025_q_d_n0, eq1_e1025_q_d_n1, eq1_e1025_q_d_n2, eq1_e1025_q_d_n3, eq1_e1025_q_d_n4, eq1_e1025_q_d_n5, eq1_e1025_q_d_n6, eq1_e1025_q_d_n7, eq1_e1025_q_d_n8, eq1_e1025_q_d_n9, eq1_e1025_q_d_n10, eq1_e1025_q_d_n11, eq1_e1025_q_d_n12, eq1_e1025_q_d_n13, eq1_e1025_q_d_n14, eq1_e1025_q_d_n15, eq1_e1025_q_d_n16, eq1_e1025_q_d_n17];
        let eq1_reactive_branch_derivatives: [f64; 12] = [eq1_e1025_q_d_b0, eq1_e1025_q_d_b1, eq1_e1025_q_d_b2, eq1_e1025_q_d_b3, eq1_e1025_q_d_b4, eq1_e1025_q_d_b5, eq1_e1025_q_d_b6, eq1_e1025_q_d_b7, eq1_e1025_q_d_b8, eq1_e1025_q_d_b9, eq1_e1025_q_d_b10, eq1_e1025_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[16]),
            None,
            nodes,
            &eq1_reactive_node_derivatives,
            branches,
            &eq1_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq4_e1042, eq4_e1042_d_n0, eq4_e1042_d_n1, eq4_e1042_d_n2, eq4_e1042_d_n3, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n11, eq4_e1042_d_n12, eq4_e1042_d_n13, eq4_e1042_d_n14, eq4_e1042_d_n15, eq4_e1042_d_n16, eq4_e1042_d_n17, eq4_e1042_d_b0, eq4_e1042_d_b1, eq4_e1042_d_b2, eq4_e1042_d_b3, eq4_e1042_d_b4, eq4_e1042_d_b5, eq4_e1042_d_b6, eq4_e1042_d_b7, eq4_e1042_d_b8, eq4_e1042_d_b9, eq4_e1042_d_b10, eq4_e1042_d_b11, eq4_e1042_q, eq4_e1042_q_d_n0, eq4_e1042_q_d_n1, eq4_e1042_q_d_n2, eq4_e1042_q_d_n3, eq4_e1042_q_d_n4, eq4_e1042_q_d_n5, eq4_e1042_q_d_n6, eq4_e1042_q_d_n7, eq4_e1042_q_d_n8, eq4_e1042_q_d_n9, eq4_e1042_q_d_n10, eq4_e1042_q_d_n11, eq4_e1042_q_d_n12, eq4_e1042_q_d_n13, eq4_e1042_q_d_n14, eq4_e1042_q_d_n15, eq4_e1042_q_d_n16, eq4_e1042_q_d_n17, eq4_e1042_q_d_b0, eq4_e1042_q_d_b1, eq4_e1042_q_d_b2, eq4_e1042_q_d_b3, eq4_e1042_q_d_b4, eq4_e1042_q_d_b5, eq4_e1042_q_d_b6, eq4_e1042_q_d_b7, eq4_e1042_q_d_b8, eq4_e1042_q_d_b9, eq4_e1042_q_d_b10, eq4_e1042_q_d_b11,) = {
    if s.b[3306] {
        let eq4_e1039_q: f64 = s.v[931];
        let eq4_e1040: f64 = (s.v[932] + s.v[931]);
        let eq4_e1040_d_n0: f64 = (s.dn[932][0] + s.dn[931][0]);
        let eq4_e1040_d_n1: f64 = (s.dn[932][1] + s.dn[931][1]);
        let eq4_e1040_d_n2: f64 = (s.dn[932][2] + s.dn[931][2]);
        let eq4_e1040_d_n3: f64 = (s.dn[932][3] + s.dn[931][3]);
        let eq4_e1040_d_n4: f64 = (s.dn[932][4] + s.dn[931][4]);
        let eq4_e1040_d_n5: f64 = (s.dn[932][5] + s.dn[931][5]);
        let eq4_e1040_d_n6: f64 = (s.dn[932][6] + s.dn[931][6]);
        let eq4_e1040_d_n7: f64 = (s.dn[932][7] + s.dn[931][7]);
        let eq4_e1040_d_n8: f64 = (s.dn[932][8] + s.dn[931][8]);
        let eq4_e1040_d_n9: f64 = (s.dn[932][9] + s.dn[931][9]);
        let eq4_e1040_d_n10: f64 = (s.dn[932][10] + s.dn[931][10]);
        let eq4_e1040_d_n11: f64 = (s.dn[932][11] + s.dn[931][11]);
        let eq4_e1040_d_n12: f64 = (s.dn[932][12] + s.dn[931][12]);
        let eq4_e1040_d_n13: f64 = (s.dn[932][13] + s.dn[931][13]);
        let eq4_e1040_d_n14: f64 = (s.dn[932][14] + s.dn[931][14]);
        let eq4_e1040_d_n15: f64 = (s.dn[932][15] + s.dn[931][15]);
        let eq4_e1040_d_n16: f64 = (s.dn[932][16] + s.dn[931][16]);
        let eq4_e1040_d_n17: f64 = (s.dn[932][17] + s.dn[931][17]);
        let eq4_e1040_d_b0: f64 = (s.db[932][0] + s.db[931][0]);
        let eq4_e1040_d_b1: f64 = (s.db[932][1] + s.db[931][1]);
        let eq4_e1040_d_b2: f64 = (s.db[932][2] + s.db[931][2]);
        let eq4_e1040_d_b3: f64 = (s.db[932][3] + s.db[931][3]);
        let eq4_e1040_d_b4: f64 = (s.db[932][4] + s.db[931][4]);
        let eq4_e1040_d_b5: f64 = (s.db[932][5] + s.db[931][5]);
        let eq4_e1040_d_b6: f64 = (s.db[932][6] + s.db[931][6]);
        let eq4_e1040_d_b7: f64 = (s.db[932][7] + s.db[931][7]);
        let eq4_e1040_d_b8: f64 = (s.db[932][8] + s.db[931][8]);
        let eq4_e1040_d_b9: f64 = (s.db[932][9] + s.db[931][9]);
        let eq4_e1040_d_b10: f64 = (s.db[932][10] + s.db[931][10]);
        let eq4_e1040_d_b11: f64 = (s.db[932][11] + s.db[931][11]);
        let eq4_e1040_q: f64 = eq4_e1039_q;
        (eq4_e1040, eq4_e1040_d_n0, eq4_e1040_d_n1, eq4_e1040_d_n2, eq4_e1040_d_n3, eq4_e1040_d_n4, eq4_e1040_d_n5, eq4_e1040_d_n6, eq4_e1040_d_n7, eq4_e1040_d_n8, eq4_e1040_d_n9, eq4_e1040_d_n10, eq4_e1040_d_n11, eq4_e1040_d_n12, eq4_e1040_d_n13, eq4_e1040_d_n14, eq4_e1040_d_n15, eq4_e1040_d_n16, eq4_e1040_d_n17, eq4_e1040_d_b0, eq4_e1040_d_b1, eq4_e1040_d_b2, eq4_e1040_d_b3, eq4_e1040_d_b4, eq4_e1040_d_b5, eq4_e1040_d_b6, eq4_e1040_d_b7, eq4_e1040_d_b8, eq4_e1040_d_b9, eq4_e1040_d_b10, eq4_e1040_d_b11, eq4_e1040_q, s.dn[931][0], s.dn[931][1], s.dn[931][2], s.dn[931][3], s.dn[931][4], s.dn[931][5], s.dn[931][6], s.dn[931][7], s.dn[931][8], s.dn[931][9], s.dn[931][10], s.dn[931][11], s.dn[931][12], s.dn[931][13], s.dn[931][14], s.dn[931][15], s.dn[931][16], s.dn[931][17], s.db[931][0], s.db[931][1], s.db[931][2], s.db[931][3], s.db[931][4], s.db[931][5], s.db[931][6], s.db[931][7], s.db[931][8], s.db[931][9], s.db[931][10], s.db[931][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_reactive_node_derivatives: [f64; 18] = [eq4_e1042_q_d_n0, eq4_e1042_q_d_n1, eq4_e1042_q_d_n2, eq4_e1042_q_d_n3, eq4_e1042_q_d_n4, eq4_e1042_q_d_n5, eq4_e1042_q_d_n6, eq4_e1042_q_d_n7, eq4_e1042_q_d_n8, eq4_e1042_q_d_n9, eq4_e1042_q_d_n10, eq4_e1042_q_d_n11, eq4_e1042_q_d_n12, eq4_e1042_q_d_n13, eq4_e1042_q_d_n14, eq4_e1042_q_d_n15, eq4_e1042_q_d_n16, eq4_e1042_q_d_n17];
        let eq4_reactive_branch_derivatives: [f64; 12] = [eq4_e1042_q_d_b0, eq4_e1042_q_d_b1, eq4_e1042_q_d_b2, eq4_e1042_q_d_b3, eq4_e1042_q_d_b4, eq4_e1042_q_d_b5, eq4_e1042_q_d_b6, eq4_e1042_q_d_b7, eq4_e1042_q_d_b8, eq4_e1042_q_d_b9, eq4_e1042_q_d_b10, eq4_e1042_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[17]),
            None,
            nodes,
            &eq4_reactive_node_derivatives,
            branches,
            &eq4_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e1088_q: f64 = s.v[66];
        let eq14_e1089: f64 = (p.p87 * s.v[66]);
        let eq14_e1089_d_n0: f64 = (p.p87 * s.dn[66][0]);
        let eq14_e1089_d_n1: f64 = (p.p87 * s.dn[66][1]);
        let eq14_e1089_d_n2: f64 = (p.p87 * s.dn[66][2]);
        let eq14_e1089_d_n3: f64 = (p.p87 * s.dn[66][3]);
        let eq14_e1089_d_n4: f64 = (p.p87 * s.dn[66][4]);
        let eq14_e1089_d_n5: f64 = (p.p87 * s.dn[66][5]);
        let eq14_e1089_d_n6: f64 = (p.p87 * s.dn[66][6]);
        let eq14_e1089_d_n7: f64 = (p.p87 * s.dn[66][7]);
        let eq14_e1089_d_n8: f64 = (p.p87 * s.dn[66][8]);
        let eq14_e1089_d_n9: f64 = (p.p87 * s.dn[66][9]);
        let eq14_e1089_d_n10: f64 = (p.p87 * s.dn[66][10]);
        let eq14_e1089_d_n11: f64 = (p.p87 * s.dn[66][11]);
        let eq14_e1089_d_n12: f64 = (p.p87 * s.dn[66][12]);
        let eq14_e1089_d_n13: f64 = (p.p87 * s.dn[66][13]);
        let eq14_e1089_d_n14: f64 = (p.p87 * s.dn[66][14]);
        let eq14_e1089_d_n15: f64 = (p.p87 * s.dn[66][15]);
        let eq14_e1089_d_n16: f64 = (p.p87 * s.dn[66][16]);
        let eq14_e1089_d_n17: f64 = (p.p87 * s.dn[66][17]);
        let eq14_e1089_d_b0: f64 = (p.p87 * s.db[66][0]);
        let eq14_e1089_d_b1: f64 = (p.p87 * s.db[66][1]);
        let eq14_e1089_d_b2: f64 = (p.p87 * s.db[66][2]);
        let eq14_e1089_d_b3: f64 = (p.p87 * s.db[66][3]);
        let eq14_e1089_d_b4: f64 = (p.p87 * s.db[66][4]);
        let eq14_e1089_d_b5: f64 = (p.p87 * s.db[66][5]);
        let eq14_e1089_d_b6: f64 = (p.p87 * s.db[66][6]);
        let eq14_e1089_d_b7: f64 = (p.p87 * s.db[66][7]);
        let eq14_e1089_d_b8: f64 = (p.p87 * s.db[66][8]);
        let eq14_e1089_d_b9: f64 = (p.p87 * s.db[66][9]);
        let eq14_e1089_d_b10: f64 = (p.p87 * s.db[66][10]);
        let eq14_e1089_d_b11: f64 = (p.p87 * s.db[66][11]);
        let eq14_e1089_q: f64 = (p.p87 * eq14_e1088_q);
        let eq14_reactive_node_derivatives: [f64; 18] = [eq14_e1089_d_n0, eq14_e1089_d_n1, eq14_e1089_d_n2, eq14_e1089_d_n3, eq14_e1089_d_n4, eq14_e1089_d_n5, eq14_e1089_d_n6, eq14_e1089_d_n7, eq14_e1089_d_n8, eq14_e1089_d_n9, eq14_e1089_d_n10, eq14_e1089_d_n11, eq14_e1089_d_n12, eq14_e1089_d_n13, eq14_e1089_d_n14, eq14_e1089_d_n15, eq14_e1089_d_n16, eq14_e1089_d_n17];
        let eq14_reactive_branch_derivatives: [f64; 12] = [eq14_e1089_d_b0, eq14_e1089_d_b1, eq14_e1089_d_b2, eq14_e1089_d_b3, eq14_e1089_d_b4, eq14_e1089_d_b5, eq14_e1089_d_b6, eq14_e1089_d_b7, eq14_e1089_d_b8, eq14_e1089_d_b9, eq14_e1089_d_b10, eq14_e1089_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e1092_q: f64 = s.v[65];
        let eq15_e1093: f64 = (p.p87 * s.v[65]);
        let eq15_e1093_d_n0: f64 = (p.p87 * s.dn[65][0]);
        let eq15_e1093_d_n1: f64 = (p.p87 * s.dn[65][1]);
        let eq15_e1093_d_n2: f64 = (p.p87 * s.dn[65][2]);
        let eq15_e1093_d_n3: f64 = (p.p87 * s.dn[65][3]);
        let eq15_e1093_d_n4: f64 = (p.p87 * s.dn[65][4]);
        let eq15_e1093_d_n5: f64 = (p.p87 * s.dn[65][5]);
        let eq15_e1093_d_n6: f64 = (p.p87 * s.dn[65][6]);
        let eq15_e1093_d_n7: f64 = (p.p87 * s.dn[65][7]);
        let eq15_e1093_d_n8: f64 = (p.p87 * s.dn[65][8]);
        let eq15_e1093_d_n9: f64 = (p.p87 * s.dn[65][9]);
        let eq15_e1093_d_n10: f64 = (p.p87 * s.dn[65][10]);
        let eq15_e1093_d_n11: f64 = (p.p87 * s.dn[65][11]);
        let eq15_e1093_d_n12: f64 = (p.p87 * s.dn[65][12]);
        let eq15_e1093_d_n13: f64 = (p.p87 * s.dn[65][13]);
        let eq15_e1093_d_n14: f64 = (p.p87 * s.dn[65][14]);
        let eq15_e1093_d_n15: f64 = (p.p87 * s.dn[65][15]);
        let eq15_e1093_d_n16: f64 = (p.p87 * s.dn[65][16]);
        let eq15_e1093_d_n17: f64 = (p.p87 * s.dn[65][17]);
        let eq15_e1093_d_b0: f64 = (p.p87 * s.db[65][0]);
        let eq15_e1093_d_b1: f64 = (p.p87 * s.db[65][1]);
        let eq15_e1093_d_b2: f64 = (p.p87 * s.db[65][2]);
        let eq15_e1093_d_b3: f64 = (p.p87 * s.db[65][3]);
        let eq15_e1093_d_b4: f64 = (p.p87 * s.db[65][4]);
        let eq15_e1093_d_b5: f64 = (p.p87 * s.db[65][5]);
        let eq15_e1093_d_b6: f64 = (p.p87 * s.db[65][6]);
        let eq15_e1093_d_b7: f64 = (p.p87 * s.db[65][7]);
        let eq15_e1093_d_b8: f64 = (p.p87 * s.db[65][8]);
        let eq15_e1093_d_b9: f64 = (p.p87 * s.db[65][9]);
        let eq15_e1093_d_b10: f64 = (p.p87 * s.db[65][10]);
        let eq15_e1093_d_b11: f64 = (p.p87 * s.db[65][11]);
        let eq15_e1093_q: f64 = (p.p87 * eq15_e1092_q);
        let eq15_reactive_node_derivatives: [f64; 18] = [eq15_e1093_d_n0, eq15_e1093_d_n1, eq15_e1093_d_n2, eq15_e1093_d_n3, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, eq15_e1093_d_n12, eq15_e1093_d_n13, eq15_e1093_d_n14, eq15_e1093_d_n15, eq15_e1093_d_n16, eq15_e1093_d_n17];
        let eq15_reactive_branch_derivatives: [f64; 12] = [eq15_e1093_d_b0, eq15_e1093_d_b1, eq15_e1093_d_b2, eq15_e1093_d_b3, eq15_e1093_d_b4, eq15_e1093_d_b5, eq15_e1093_d_b6, eq15_e1093_d_b7, eq15_e1093_d_b8, eq15_e1093_d_b9, eq15_e1093_d_b10, eq15_e1093_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[0]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1112, eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17, eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8, eq18_e1112_d_b9, eq18_e1112_d_b10, eq18_e1112_d_b11, eq18_e1112_q,) = {
    if s.b[3405] {
        let eq18_e1109_q: f64 = s.v[68];
        let eq18_e1110: f64 = (p.p87 * s.v[68]);
        let eq18_e1110_d_n0: f64 = (p.p87 * s.dn[68][0]);
        let eq18_e1110_d_n1: f64 = (p.p87 * s.dn[68][1]);
        let eq18_e1110_d_n2: f64 = (p.p87 * s.dn[68][2]);
        let eq18_e1110_d_n3: f64 = (p.p87 * s.dn[68][3]);
        let eq18_e1110_d_n4: f64 = (p.p87 * s.dn[68][4]);
        let eq18_e1110_d_n5: f64 = (p.p87 * s.dn[68][5]);
        let eq18_e1110_d_n6: f64 = (p.p87 * s.dn[68][6]);
        let eq18_e1110_d_n7: f64 = (p.p87 * s.dn[68][7]);
        let eq18_e1110_d_n8: f64 = (p.p87 * s.dn[68][8]);
        let eq18_e1110_d_n9: f64 = (p.p87 * s.dn[68][9]);
        let eq18_e1110_d_n10: f64 = (p.p87 * s.dn[68][10]);
        let eq18_e1110_d_n11: f64 = (p.p87 * s.dn[68][11]);
        let eq18_e1110_d_n12: f64 = (p.p87 * s.dn[68][12]);
        let eq18_e1110_d_n13: f64 = (p.p87 * s.dn[68][13]);
        let eq18_e1110_d_n14: f64 = (p.p87 * s.dn[68][14]);
        let eq18_e1110_d_n15: f64 = (p.p87 * s.dn[68][15]);
        let eq18_e1110_d_n16: f64 = (p.p87 * s.dn[68][16]);
        let eq18_e1110_d_n17: f64 = (p.p87 * s.dn[68][17]);
        let eq18_e1110_d_b0: f64 = (p.p87 * s.db[68][0]);
        let eq18_e1110_d_b1: f64 = (p.p87 * s.db[68][1]);
        let eq18_e1110_d_b2: f64 = (p.p87 * s.db[68][2]);
        let eq18_e1110_d_b3: f64 = (p.p87 * s.db[68][3]);
        let eq18_e1110_d_b4: f64 = (p.p87 * s.db[68][4]);
        let eq18_e1110_d_b5: f64 = (p.p87 * s.db[68][5]);
        let eq18_e1110_d_b6: f64 = (p.p87 * s.db[68][6]);
        let eq18_e1110_d_b7: f64 = (p.p87 * s.db[68][7]);
        let eq18_e1110_d_b8: f64 = (p.p87 * s.db[68][8]);
        let eq18_e1110_d_b9: f64 = (p.p87 * s.db[68][9]);
        let eq18_e1110_d_b10: f64 = (p.p87 * s.db[68][10]);
        let eq18_e1110_d_b11: f64 = (p.p87 * s.db[68][11]);
        let eq18_e1110_q: f64 = (p.p87 * eq18_e1109_q);
        (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11, eq18_e1110_d_n12, eq18_e1110_d_n13, eq18_e1110_d_n14, eq18_e1110_d_n15, eq18_e1110_d_n16, eq18_e1110_d_n17, eq18_e1110_d_b0, eq18_e1110_d_b1, eq18_e1110_d_b2, eq18_e1110_d_b3, eq18_e1110_d_b4, eq18_e1110_d_b5, eq18_e1110_d_b6, eq18_e1110_d_b7, eq18_e1110_d_b8, eq18_e1110_d_b9, eq18_e1110_d_b10, eq18_e1110_d_b11, eq18_e1110_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_reactive_node_derivatives: [f64; 18] = [eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17];
        let eq18_reactive_branch_derivatives: [f64; 12] = [eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8, eq18_e1112_d_b9, eq18_e1112_d_b10, eq18_e1112_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1119, eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17, eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8, eq19_e1119_d_b9, eq19_e1119_d_b10, eq19_e1119_d_b11, eq19_e1119_q,) = {
    if s.b[3405] {
        let eq19_e1116_q: f64 = s.v[67];
        let eq19_e1117: f64 = (p.p87 * s.v[67]);
        let eq19_e1117_d_n0: f64 = (p.p87 * s.dn[67][0]);
        let eq19_e1117_d_n1: f64 = (p.p87 * s.dn[67][1]);
        let eq19_e1117_d_n2: f64 = (p.p87 * s.dn[67][2]);
        let eq19_e1117_d_n3: f64 = (p.p87 * s.dn[67][3]);
        let eq19_e1117_d_n4: f64 = (p.p87 * s.dn[67][4]);
        let eq19_e1117_d_n5: f64 = (p.p87 * s.dn[67][5]);
        let eq19_e1117_d_n6: f64 = (p.p87 * s.dn[67][6]);
        let eq19_e1117_d_n7: f64 = (p.p87 * s.dn[67][7]);
        let eq19_e1117_d_n8: f64 = (p.p87 * s.dn[67][8]);
        let eq19_e1117_d_n9: f64 = (p.p87 * s.dn[67][9]);
        let eq19_e1117_d_n10: f64 = (p.p87 * s.dn[67][10]);
        let eq19_e1117_d_n11: f64 = (p.p87 * s.dn[67][11]);
        let eq19_e1117_d_n12: f64 = (p.p87 * s.dn[67][12]);
        let eq19_e1117_d_n13: f64 = (p.p87 * s.dn[67][13]);
        let eq19_e1117_d_n14: f64 = (p.p87 * s.dn[67][14]);
        let eq19_e1117_d_n15: f64 = (p.p87 * s.dn[67][15]);
        let eq19_e1117_d_n16: f64 = (p.p87 * s.dn[67][16]);
        let eq19_e1117_d_n17: f64 = (p.p87 * s.dn[67][17]);
        let eq19_e1117_d_b0: f64 = (p.p87 * s.db[67][0]);
        let eq19_e1117_d_b1: f64 = (p.p87 * s.db[67][1]);
        let eq19_e1117_d_b2: f64 = (p.p87 * s.db[67][2]);
        let eq19_e1117_d_b3: f64 = (p.p87 * s.db[67][3]);
        let eq19_e1117_d_b4: f64 = (p.p87 * s.db[67][4]);
        let eq19_e1117_d_b5: f64 = (p.p87 * s.db[67][5]);
        let eq19_e1117_d_b6: f64 = (p.p87 * s.db[67][6]);
        let eq19_e1117_d_b7: f64 = (p.p87 * s.db[67][7]);
        let eq19_e1117_d_b8: f64 = (p.p87 * s.db[67][8]);
        let eq19_e1117_d_b9: f64 = (p.p87 * s.db[67][9]);
        let eq19_e1117_d_b10: f64 = (p.p87 * s.db[67][10]);
        let eq19_e1117_d_b11: f64 = (p.p87 * s.db[67][11]);
        let eq19_e1117_q: f64 = (p.p87 * eq19_e1116_q);
        (eq19_e1117, eq19_e1117_d_n0, eq19_e1117_d_n1, eq19_e1117_d_n2, eq19_e1117_d_n3, eq19_e1117_d_n4, eq19_e1117_d_n5, eq19_e1117_d_n6, eq19_e1117_d_n7, eq19_e1117_d_n8, eq19_e1117_d_n9, eq19_e1117_d_n10, eq19_e1117_d_n11, eq19_e1117_d_n12, eq19_e1117_d_n13, eq19_e1117_d_n14, eq19_e1117_d_n15, eq19_e1117_d_n16, eq19_e1117_d_n17, eq19_e1117_d_b0, eq19_e1117_d_b1, eq19_e1117_d_b2, eq19_e1117_d_b3, eq19_e1117_d_b4, eq19_e1117_d_b5, eq19_e1117_d_b6, eq19_e1117_d_b7, eq19_e1117_d_b8, eq19_e1117_d_b9, eq19_e1117_d_b10, eq19_e1117_d_b11, eq19_e1117_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 18] = [eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17];
        let eq19_reactive_branch_derivatives: [f64; 12] = [eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8, eq19_e1119_d_b9, eq19_e1119_d_b10, eq19_e1119_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq27_e1163: f64 = (s.v[18] + s.v[753]);
        let eq27_e1163_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);
        let eq27_e1163_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);
        let eq27_e1163_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);
        let eq27_e1163_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);
        let eq27_e1163_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);
        let eq27_e1163_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);
        let eq27_e1163_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);
        let eq27_e1163_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);
        let eq27_e1163_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);
        let eq27_e1163_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);
        let eq27_e1163_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);
        let eq27_e1163_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);
        let eq27_e1163_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);
        let eq27_e1163_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);
        let eq27_e1163_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);
        let eq27_e1163_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);
        let eq27_e1163_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);
        let eq27_e1163_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);
        let eq27_e1163_d_b0: f64 = (s.db[18][0] + s.db[753][0]);
        let eq27_e1163_d_b1: f64 = (s.db[18][1] + s.db[753][1]);
        let eq27_e1163_d_b2: f64 = (s.db[18][2] + s.db[753][2]);
        let eq27_e1163_d_b3: f64 = (s.db[18][3] + s.db[753][3]);
        let eq27_e1163_d_b4: f64 = (s.db[18][4] + s.db[753][4]);
        let eq27_e1163_d_b5: f64 = (s.db[18][5] + s.db[753][5]);
        let eq27_e1163_d_b6: f64 = (s.db[18][6] + s.db[753][6]);
        let eq27_e1163_d_b7: f64 = (s.db[18][7] + s.db[753][7]);
        let eq27_e1163_d_b8: f64 = (s.db[18][8] + s.db[753][8]);
        let eq27_e1163_d_b9: f64 = (s.db[18][9] + s.db[753][9]);
        let eq27_e1163_d_b10: f64 = (s.db[18][10] + s.db[753][10]);
        let eq27_e1163_d_b11: f64 = (s.db[18][11] + s.db[753][11]);
        let eq27_e1164_q: f64 = eq27_e1163;
        let eq27_e1165: f64 = (p.p87 * eq27_e1163);
        let eq27_e1165_d_n0: f64 = (p.p87 * eq27_e1163_d_n0);
        let eq27_e1165_d_n1: f64 = (p.p87 * eq27_e1163_d_n1);
        let eq27_e1165_d_n2: f64 = (p.p87 * eq27_e1163_d_n2);
        let eq27_e1165_d_n3: f64 = (p.p87 * eq27_e1163_d_n3);
        let eq27_e1165_d_n4: f64 = (p.p87 * eq27_e1163_d_n4);
        let eq27_e1165_d_n5: f64 = (p.p87 * eq27_e1163_d_n5);
        let eq27_e1165_d_n6: f64 = (p.p87 * eq27_e1163_d_n6);
        let eq27_e1165_d_n7: f64 = (p.p87 * eq27_e1163_d_n7);
        let eq27_e1165_d_n8: f64 = (p.p87 * eq27_e1163_d_n8);
        let eq27_e1165_d_n9: f64 = (p.p87 * eq27_e1163_d_n9);
        let eq27_e1165_d_n10: f64 = (p.p87 * eq27_e1163_d_n10);
        let eq27_e1165_d_n11: f64 = (p.p87 * eq27_e1163_d_n11);
        let eq27_e1165_d_n12: f64 = (p.p87 * eq27_e1163_d_n12);
        let eq27_e1165_d_n13: f64 = (p.p87 * eq27_e1163_d_n13);
        let eq27_e1165_d_n14: f64 = (p.p87 * eq27_e1163_d_n14);
        let eq27_e1165_d_n15: f64 = (p.p87 * eq27_e1163_d_n15);
        let eq27_e1165_d_n16: f64 = (p.p87 * eq27_e1163_d_n16);
        let eq27_e1165_d_n17: f64 = (p.p87 * eq27_e1163_d_n17);
        let eq27_e1165_d_b0: f64 = (p.p87 * eq27_e1163_d_b0);
        let eq27_e1165_d_b1: f64 = (p.p87 * eq27_e1163_d_b1);
        let eq27_e1165_d_b2: f64 = (p.p87 * eq27_e1163_d_b2);
        let eq27_e1165_d_b3: f64 = (p.p87 * eq27_e1163_d_b3);
        let eq27_e1165_d_b4: f64 = (p.p87 * eq27_e1163_d_b4);
        let eq27_e1165_d_b5: f64 = (p.p87 * eq27_e1163_d_b5);
        let eq27_e1165_d_b6: f64 = (p.p87 * eq27_e1163_d_b6);
        let eq27_e1165_d_b7: f64 = (p.p87 * eq27_e1163_d_b7);
        let eq27_e1165_d_b8: f64 = (p.p87 * eq27_e1163_d_b8);
        let eq27_e1165_d_b9: f64 = (p.p87 * eq27_e1163_d_b9);
        let eq27_e1165_d_b10: f64 = (p.p87 * eq27_e1163_d_b10);
        let eq27_e1165_d_b11: f64 = (p.p87 * eq27_e1163_d_b11);
        let eq27_e1165_q: f64 = (p.p87 * eq27_e1164_q);
        let eq27_reactive_node_derivatives: [f64; 18] = [eq27_e1165_d_n0, eq27_e1165_d_n1, eq27_e1165_d_n2, eq27_e1165_d_n3, eq27_e1165_d_n4, eq27_e1165_d_n5, eq27_e1165_d_n6, eq27_e1165_d_n7, eq27_e1165_d_n8, eq27_e1165_d_n9, eq27_e1165_d_n10, eq27_e1165_d_n11, eq27_e1165_d_n12, eq27_e1165_d_n13, eq27_e1165_d_n14, eq27_e1165_d_n15, eq27_e1165_d_n16, eq27_e1165_d_n17];
        let eq27_reactive_branch_derivatives: [f64; 12] = [eq27_e1165_d_b0, eq27_e1165_d_b1, eq27_e1165_d_b2, eq27_e1165_d_b3, eq27_e1165_d_b4, eq27_e1165_d_b5, eq27_e1165_d_b6, eq27_e1165_d_b7, eq27_e1165_d_b8, eq27_e1165_d_b9, eq27_e1165_d_b10, eq27_e1165_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e1169: f64 = (s.v[19] + s.v[751]);
        let eq28_e1169_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);
        let eq28_e1169_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);
        let eq28_e1169_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);
        let eq28_e1169_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);
        let eq28_e1169_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);
        let eq28_e1169_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);
        let eq28_e1169_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);
        let eq28_e1169_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);
        let eq28_e1169_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);
        let eq28_e1169_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);
        let eq28_e1169_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);
        let eq28_e1169_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);
        let eq28_e1169_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);
        let eq28_e1169_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);
        let eq28_e1169_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);
        let eq28_e1169_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);
        let eq28_e1169_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);
        let eq28_e1169_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);
        let eq28_e1169_d_b0: f64 = (s.db[19][0] + s.db[751][0]);
        let eq28_e1169_d_b1: f64 = (s.db[19][1] + s.db[751][1]);
        let eq28_e1169_d_b2: f64 = (s.db[19][2] + s.db[751][2]);
        let eq28_e1169_d_b3: f64 = (s.db[19][3] + s.db[751][3]);
        let eq28_e1169_d_b4: f64 = (s.db[19][4] + s.db[751][4]);
        let eq28_e1169_d_b5: f64 = (s.db[19][5] + s.db[751][5]);
        let eq28_e1169_d_b6: f64 = (s.db[19][6] + s.db[751][6]);
        let eq28_e1169_d_b7: f64 = (s.db[19][7] + s.db[751][7]);
        let eq28_e1169_d_b8: f64 = (s.db[19][8] + s.db[751][8]);
        let eq28_e1169_d_b9: f64 = (s.db[19][9] + s.db[751][9]);
        let eq28_e1169_d_b10: f64 = (s.db[19][10] + s.db[751][10]);
        let eq28_e1169_d_b11: f64 = (s.db[19][11] + s.db[751][11]);
        let eq28_e1170_q: f64 = eq28_e1169;
        let eq28_e1171: f64 = (p.p87 * eq28_e1169);
        let eq28_e1171_d_n0: f64 = (p.p87 * eq28_e1169_d_n0);
        let eq28_e1171_d_n1: f64 = (p.p87 * eq28_e1169_d_n1);
        let eq28_e1171_d_n2: f64 = (p.p87 * eq28_e1169_d_n2);
        let eq28_e1171_d_n3: f64 = (p.p87 * eq28_e1169_d_n3);
        let eq28_e1171_d_n4: f64 = (p.p87 * eq28_e1169_d_n4);
        let eq28_e1171_d_n5: f64 = (p.p87 * eq28_e1169_d_n5);
        let eq28_e1171_d_n6: f64 = (p.p87 * eq28_e1169_d_n6);
        let eq28_e1171_d_n7: f64 = (p.p87 * eq28_e1169_d_n7);
        let eq28_e1171_d_n8: f64 = (p.p87 * eq28_e1169_d_n8);
        let eq28_e1171_d_n9: f64 = (p.p87 * eq28_e1169_d_n9);
        let eq28_e1171_d_n10: f64 = (p.p87 * eq28_e1169_d_n10);
        let eq28_e1171_d_n11: f64 = (p.p87 * eq28_e1169_d_n11);
        let eq28_e1171_d_n12: f64 = (p.p87 * eq28_e1169_d_n12);
        let eq28_e1171_d_n13: f64 = (p.p87 * eq28_e1169_d_n13);
        let eq28_e1171_d_n14: f64 = (p.p87 * eq28_e1169_d_n14);
        let eq28_e1171_d_n15: f64 = (p.p87 * eq28_e1169_d_n15);
        let eq28_e1171_d_n16: f64 = (p.p87 * eq28_e1169_d_n16);
        let eq28_e1171_d_n17: f64 = (p.p87 * eq28_e1169_d_n17);
        let eq28_e1171_d_b0: f64 = (p.p87 * eq28_e1169_d_b0);
        let eq28_e1171_d_b1: f64 = (p.p87 * eq28_e1169_d_b1);
        let eq28_e1171_d_b2: f64 = (p.p87 * eq28_e1169_d_b2);
        let eq28_e1171_d_b3: f64 = (p.p87 * eq28_e1169_d_b3);
        let eq28_e1171_d_b4: f64 = (p.p87 * eq28_e1169_d_b4);
        let eq28_e1171_d_b5: f64 = (p.p87 * eq28_e1169_d_b5);
        let eq28_e1171_d_b6: f64 = (p.p87 * eq28_e1169_d_b6);
        let eq28_e1171_d_b7: f64 = (p.p87 * eq28_e1169_d_b7);
        let eq28_e1171_d_b8: f64 = (p.p87 * eq28_e1169_d_b8);
        let eq28_e1171_d_b9: f64 = (p.p87 * eq28_e1169_d_b9);
        let eq28_e1171_d_b10: f64 = (p.p87 * eq28_e1169_d_b10);
        let eq28_e1171_d_b11: f64 = (p.p87 * eq28_e1169_d_b11);
        let eq28_e1171_q: f64 = (p.p87 * eq28_e1170_q);
        let eq28_reactive_node_derivatives: [f64; 18] = [eq28_e1171_d_n0, eq28_e1171_d_n1, eq28_e1171_d_n2, eq28_e1171_d_n3, eq28_e1171_d_n4, eq28_e1171_d_n5, eq28_e1171_d_n6, eq28_e1171_d_n7, eq28_e1171_d_n8, eq28_e1171_d_n9, eq28_e1171_d_n10, eq28_e1171_d_n11, eq28_e1171_d_n12, eq28_e1171_d_n13, eq28_e1171_d_n14, eq28_e1171_d_n15, eq28_e1171_d_n16, eq28_e1171_d_n17];
        let eq28_reactive_branch_derivatives: [f64; 12] = [eq28_e1171_d_b0, eq28_e1171_d_b1, eq28_e1171_d_b2, eq28_e1171_d_b3, eq28_e1171_d_b4, eq28_e1171_d_b5, eq28_e1171_d_b6, eq28_e1171_d_b7, eq28_e1171_d_b8, eq28_e1171_d_b9, eq28_e1171_d_b10, eq28_e1171_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq29_e1176: f64 = (s.v[753] + s.v[751]);
        let eq29_e1176_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);
        let eq29_e1176_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);
        let eq29_e1176_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);
        let eq29_e1176_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);
        let eq29_e1176_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);
        let eq29_e1176_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);
        let eq29_e1176_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);
        let eq29_e1176_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);
        let eq29_e1176_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);
        let eq29_e1176_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);
        let eq29_e1176_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);
        let eq29_e1176_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);
        let eq29_e1176_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);
        let eq29_e1176_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);
        let eq29_e1176_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);
        let eq29_e1176_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);
        let eq29_e1176_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);
        let eq29_e1176_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);
        let eq29_e1176_d_b0: f64 = (s.db[753][0] + s.db[751][0]);
        let eq29_e1176_d_b1: f64 = (s.db[753][1] + s.db[751][1]);
        let eq29_e1176_d_b2: f64 = (s.db[753][2] + s.db[751][2]);
        let eq29_e1176_d_b3: f64 = (s.db[753][3] + s.db[751][3]);
        let eq29_e1176_d_b4: f64 = (s.db[753][4] + s.db[751][4]);
        let eq29_e1176_d_b5: f64 = (s.db[753][5] + s.db[751][5]);
        let eq29_e1176_d_b6: f64 = (s.db[753][6] + s.db[751][6]);
        let eq29_e1176_d_b7: f64 = (s.db[753][7] + s.db[751][7]);
        let eq29_e1176_d_b8: f64 = (s.db[753][8] + s.db[751][8]);
        let eq29_e1176_d_b9: f64 = (s.db[753][9] + s.db[751][9]);
        let eq29_e1176_d_b10: f64 = (s.db[753][10] + s.db[751][10]);
        let eq29_e1176_d_b11: f64 = (s.db[753][11] + s.db[751][11]);
        let eq29_e1178: f64 = (eq29_e1176 + s.v[752]);
        let eq29_e1178_d_n0: f64 = (eq29_e1176_d_n0 + s.dn[752][0]);
        let eq29_e1178_d_n1: f64 = (eq29_e1176_d_n1 + s.dn[752][1]);
        let eq29_e1178_d_n2: f64 = (eq29_e1176_d_n2 + s.dn[752][2]);
        let eq29_e1178_d_n3: f64 = (eq29_e1176_d_n3 + s.dn[752][3]);
        let eq29_e1178_d_n4: f64 = (eq29_e1176_d_n4 + s.dn[752][4]);
        let eq29_e1178_d_n5: f64 = (eq29_e1176_d_n5 + s.dn[752][5]);
        let eq29_e1178_d_n6: f64 = (eq29_e1176_d_n6 + s.dn[752][6]);
        let eq29_e1178_d_n7: f64 = (eq29_e1176_d_n7 + s.dn[752][7]);
        let eq29_e1178_d_n8: f64 = (eq29_e1176_d_n8 + s.dn[752][8]);
        let eq29_e1178_d_n9: f64 = (eq29_e1176_d_n9 + s.dn[752][9]);
        let eq29_e1178_d_n10: f64 = (eq29_e1176_d_n10 + s.dn[752][10]);
        let eq29_e1178_d_n11: f64 = (eq29_e1176_d_n11 + s.dn[752][11]);
        let eq29_e1178_d_n12: f64 = (eq29_e1176_d_n12 + s.dn[752][12]);
        let eq29_e1178_d_n13: f64 = (eq29_e1176_d_n13 + s.dn[752][13]);
        let eq29_e1178_d_n14: f64 = (eq29_e1176_d_n14 + s.dn[752][14]);
        let eq29_e1178_d_n15: f64 = (eq29_e1176_d_n15 + s.dn[752][15]);
        let eq29_e1178_d_n16: f64 = (eq29_e1176_d_n16 + s.dn[752][16]);
        let eq29_e1178_d_n17: f64 = (eq29_e1176_d_n17 + s.dn[752][17]);
        let eq29_e1178_d_b0: f64 = (eq29_e1176_d_b0 + s.db[752][0]);
        let eq29_e1178_d_b1: f64 = (eq29_e1176_d_b1 + s.db[752][1]);
        let eq29_e1178_d_b2: f64 = (eq29_e1176_d_b2 + s.db[752][2]);
        let eq29_e1178_d_b3: f64 = (eq29_e1176_d_b3 + s.db[752][3]);
        let eq29_e1178_d_b4: f64 = (eq29_e1176_d_b4 + s.db[752][4]);
        let eq29_e1178_d_b5: f64 = (eq29_e1176_d_b5 + s.db[752][5]);
        let eq29_e1178_d_b6: f64 = (eq29_e1176_d_b6 + s.db[752][6]);
        let eq29_e1178_d_b7: f64 = (eq29_e1176_d_b7 + s.db[752][7]);
        let eq29_e1178_d_b8: f64 = (eq29_e1176_d_b8 + s.db[752][8]);
        let eq29_e1178_d_b9: f64 = (eq29_e1176_d_b9 + s.db[752][9]);
        let eq29_e1178_d_b10: f64 = (eq29_e1176_d_b10 + s.db[752][10]);
        let eq29_e1178_d_b11: f64 = (eq29_e1176_d_b11 + s.db[752][11]);
        let eq29_e1179: f64 = (s.v[20] - eq29_e1178);
        let eq29_e1179_d_n0: f64 = (s.dn[20][0] - eq29_e1178_d_n0);
        let eq29_e1179_d_n1: f64 = (s.dn[20][1] - eq29_e1178_d_n1);
        let eq29_e1179_d_n2: f64 = (s.dn[20][2] - eq29_e1178_d_n2);
        let eq29_e1179_d_n3: f64 = (s.dn[20][3] - eq29_e1178_d_n3);
        let eq29_e1179_d_n4: f64 = (s.dn[20][4] - eq29_e1178_d_n4);
        let eq29_e1179_d_n5: f64 = (s.dn[20][5] - eq29_e1178_d_n5);
        let eq29_e1179_d_n6: f64 = (s.dn[20][6] - eq29_e1178_d_n6);
        let eq29_e1179_d_n7: f64 = (s.dn[20][7] - eq29_e1178_d_n7);
        let eq29_e1179_d_n8: f64 = (s.dn[20][8] - eq29_e1178_d_n8);
        let eq29_e1179_d_n9: f64 = (s.dn[20][9] - eq29_e1178_d_n9);
        let eq29_e1179_d_n10: f64 = (s.dn[20][10] - eq29_e1178_d_n10);
        let eq29_e1179_d_n11: f64 = (s.dn[20][11] - eq29_e1178_d_n11);
        let eq29_e1179_d_n12: f64 = (s.dn[20][12] - eq29_e1178_d_n12);
        let eq29_e1179_d_n13: f64 = (s.dn[20][13] - eq29_e1178_d_n13);
        let eq29_e1179_d_n14: f64 = (s.dn[20][14] - eq29_e1178_d_n14);
        let eq29_e1179_d_n15: f64 = (s.dn[20][15] - eq29_e1178_d_n15);
        let eq29_e1179_d_n16: f64 = (s.dn[20][16] - eq29_e1178_d_n16);
        let eq29_e1179_d_n17: f64 = (s.dn[20][17] - eq29_e1178_d_n17);
        let eq29_e1179_d_b0: f64 = (s.db[20][0] - eq29_e1178_d_b0);
        let eq29_e1179_d_b1: f64 = (s.db[20][1] - eq29_e1178_d_b1);
        let eq29_e1179_d_b2: f64 = (s.db[20][2] - eq29_e1178_d_b2);
        let eq29_e1179_d_b3: f64 = (s.db[20][3] - eq29_e1178_d_b3);
        let eq29_e1179_d_b4: f64 = (s.db[20][4] - eq29_e1178_d_b4);
        let eq29_e1179_d_b5: f64 = (s.db[20][5] - eq29_e1178_d_b5);
        let eq29_e1179_d_b6: f64 = (s.db[20][6] - eq29_e1178_d_b6);
        let eq29_e1179_d_b7: f64 = (s.db[20][7] - eq29_e1178_d_b7);
        let eq29_e1179_d_b8: f64 = (s.db[20][8] - eq29_e1178_d_b8);
        let eq29_e1179_d_b9: f64 = (s.db[20][9] - eq29_e1178_d_b9);
        let eq29_e1179_d_b10: f64 = (s.db[20][10] - eq29_e1178_d_b10);
        let eq29_e1179_d_b11: f64 = (s.db[20][11] - eq29_e1178_d_b11);
        let eq29_e1180_q: f64 = eq29_e1179;
        let eq29_e1181: f64 = (p.p87 * eq29_e1179);
        let eq29_e1181_d_n0: f64 = (p.p87 * eq29_e1179_d_n0);
        let eq29_e1181_d_n1: f64 = (p.p87 * eq29_e1179_d_n1);
        let eq29_e1181_d_n2: f64 = (p.p87 * eq29_e1179_d_n2);
        let eq29_e1181_d_n3: f64 = (p.p87 * eq29_e1179_d_n3);
        let eq29_e1181_d_n4: f64 = (p.p87 * eq29_e1179_d_n4);
        let eq29_e1181_d_n5: f64 = (p.p87 * eq29_e1179_d_n5);
        let eq29_e1181_d_n6: f64 = (p.p87 * eq29_e1179_d_n6);
        let eq29_e1181_d_n7: f64 = (p.p87 * eq29_e1179_d_n7);
        let eq29_e1181_d_n8: f64 = (p.p87 * eq29_e1179_d_n8);
        let eq29_e1181_d_n9: f64 = (p.p87 * eq29_e1179_d_n9);
        let eq29_e1181_d_n10: f64 = (p.p87 * eq29_e1179_d_n10);
        let eq29_e1181_d_n11: f64 = (p.p87 * eq29_e1179_d_n11);
        let eq29_e1181_d_n12: f64 = (p.p87 * eq29_e1179_d_n12);
        let eq29_e1181_d_n13: f64 = (p.p87 * eq29_e1179_d_n13);
        let eq29_e1181_d_n14: f64 = (p.p87 * eq29_e1179_d_n14);
        let eq29_e1181_d_n15: f64 = (p.p87 * eq29_e1179_d_n15);
        let eq29_e1181_d_n16: f64 = (p.p87 * eq29_e1179_d_n16);
        let eq29_e1181_d_n17: f64 = (p.p87 * eq29_e1179_d_n17);
        let eq29_e1181_d_b0: f64 = (p.p87 * eq29_e1179_d_b0);
        let eq29_e1181_d_b1: f64 = (p.p87 * eq29_e1179_d_b1);
        let eq29_e1181_d_b2: f64 = (p.p87 * eq29_e1179_d_b2);
        let eq29_e1181_d_b3: f64 = (p.p87 * eq29_e1179_d_b3);
        let eq29_e1181_d_b4: f64 = (p.p87 * eq29_e1179_d_b4);
        let eq29_e1181_d_b5: f64 = (p.p87 * eq29_e1179_d_b5);
        let eq29_e1181_d_b6: f64 = (p.p87 * eq29_e1179_d_b6);
        let eq29_e1181_d_b7: f64 = (p.p87 * eq29_e1179_d_b7);
        let eq29_e1181_d_b8: f64 = (p.p87 * eq29_e1179_d_b8);
        let eq29_e1181_d_b9: f64 = (p.p87 * eq29_e1179_d_b9);
        let eq29_e1181_d_b10: f64 = (p.p87 * eq29_e1179_d_b10);
        let eq29_e1181_d_b11: f64 = (p.p87 * eq29_e1179_d_b11);
        let eq29_e1181_q: f64 = (p.p87 * eq29_e1180_q);
        let eq29_reactive_node_derivatives: [f64; 18] = [eq29_e1181_d_n0, eq29_e1181_d_n1, eq29_e1181_d_n2, eq29_e1181_d_n3, eq29_e1181_d_n4, eq29_e1181_d_n5, eq29_e1181_d_n6, eq29_e1181_d_n7, eq29_e1181_d_n8, eq29_e1181_d_n9, eq29_e1181_d_n10, eq29_e1181_d_n11, eq29_e1181_d_n12, eq29_e1181_d_n13, eq29_e1181_d_n14, eq29_e1181_d_n15, eq29_e1181_d_n16, eq29_e1181_d_n17];
        let eq29_reactive_branch_derivatives: [f64; 12] = [eq29_e1181_d_b0, eq29_e1181_d_b1, eq29_e1181_d_b2, eq29_e1181_d_b3, eq29_e1181_d_b4, eq29_e1181_d_b5, eq29_e1181_d_b6, eq29_e1181_d_b7, eq29_e1181_d_b8, eq29_e1181_d_b9, eq29_e1181_d_b10, eq29_e1181_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e1184_q: f64 = s.v[743];
        let eq30_e1185: f64 = (p.p87 * s.v[743]);
        let eq30_e1185_d_n0: f64 = (p.p87 * s.dn[743][0]);
        let eq30_e1185_d_n1: f64 = (p.p87 * s.dn[743][1]);
        let eq30_e1185_d_n2: f64 = (p.p87 * s.dn[743][2]);
        let eq30_e1185_d_n3: f64 = (p.p87 * s.dn[743][3]);
        let eq30_e1185_d_n4: f64 = (p.p87 * s.dn[743][4]);
        let eq30_e1185_d_n5: f64 = (p.p87 * s.dn[743][5]);
        let eq30_e1185_d_n6: f64 = (p.p87 * s.dn[743][6]);
        let eq30_e1185_d_n7: f64 = (p.p87 * s.dn[743][7]);
        let eq30_e1185_d_n8: f64 = (p.p87 * s.dn[743][8]);
        let eq30_e1185_d_n9: f64 = (p.p87 * s.dn[743][9]);
        let eq30_e1185_d_n10: f64 = (p.p87 * s.dn[743][10]);
        let eq30_e1185_d_n11: f64 = (p.p87 * s.dn[743][11]);
        let eq30_e1185_d_n12: f64 = (p.p87 * s.dn[743][12]);
        let eq30_e1185_d_n13: f64 = (p.p87 * s.dn[743][13]);
        let eq30_e1185_d_n14: f64 = (p.p87 * s.dn[743][14]);
        let eq30_e1185_d_n15: f64 = (p.p87 * s.dn[743][15]);
        let eq30_e1185_d_n16: f64 = (p.p87 * s.dn[743][16]);
        let eq30_e1185_d_n17: f64 = (p.p87 * s.dn[743][17]);
        let eq30_e1185_d_b0: f64 = (p.p87 * s.db[743][0]);
        let eq30_e1185_d_b1: f64 = (p.p87 * s.db[743][1]);
        let eq30_e1185_d_b2: f64 = (p.p87 * s.db[743][2]);
        let eq30_e1185_d_b3: f64 = (p.p87 * s.db[743][3]);
        let eq30_e1185_d_b4: f64 = (p.p87 * s.db[743][4]);
        let eq30_e1185_d_b5: f64 = (p.p87 * s.db[743][5]);
        let eq30_e1185_d_b6: f64 = (p.p87 * s.db[743][6]);
        let eq30_e1185_d_b7: f64 = (p.p87 * s.db[743][7]);
        let eq30_e1185_d_b8: f64 = (p.p87 * s.db[743][8]);
        let eq30_e1185_d_b9: f64 = (p.p87 * s.db[743][9]);
        let eq30_e1185_d_b10: f64 = (p.p87 * s.db[743][10]);
        let eq30_e1185_d_b11: f64 = (p.p87 * s.db[743][11]);
        let eq30_e1185_q: f64 = (p.p87 * eq30_e1184_q);
        let eq30_reactive_node_derivatives: [f64; 18] = [eq30_e1185_d_n0, eq30_e1185_d_n1, eq30_e1185_d_n2, eq30_e1185_d_n3, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14, eq30_e1185_d_n15, eq30_e1185_d_n16, eq30_e1185_d_n17];
        let eq30_reactive_branch_derivatives: [f64; 12] = [eq30_e1185_d_b0, eq30_e1185_d_b1, eq30_e1185_d_b2, eq30_e1185_d_b3, eq30_e1185_d_b4, eq30_e1185_d_b5, eq30_e1185_d_b6, eq30_e1185_d_b7, eq30_e1185_d_b8, eq30_e1185_d_b9, eq30_e1185_d_b10, eq30_e1185_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188_q: f64 = s.v[742];
        let eq31_e1189: f64 = (p.p87 * s.v[742]);
        let eq31_e1189_d_n0: f64 = (p.p87 * s.dn[742][0]);
        let eq31_e1189_d_n1: f64 = (p.p87 * s.dn[742][1]);
        let eq31_e1189_d_n2: f64 = (p.p87 * s.dn[742][2]);
        let eq31_e1189_d_n3: f64 = (p.p87 * s.dn[742][3]);
        let eq31_e1189_d_n4: f64 = (p.p87 * s.dn[742][4]);
        let eq31_e1189_d_n5: f64 = (p.p87 * s.dn[742][5]);
        let eq31_e1189_d_n6: f64 = (p.p87 * s.dn[742][6]);
        let eq31_e1189_d_n7: f64 = (p.p87 * s.dn[742][7]);
        let eq31_e1189_d_n8: f64 = (p.p87 * s.dn[742][8]);
        let eq31_e1189_d_n9: f64 = (p.p87 * s.dn[742][9]);
        let eq31_e1189_d_n10: f64 = (p.p87 * s.dn[742][10]);
        let eq31_e1189_d_n11: f64 = (p.p87 * s.dn[742][11]);
        let eq31_e1189_d_n12: f64 = (p.p87 * s.dn[742][12]);
        let eq31_e1189_d_n13: f64 = (p.p87 * s.dn[742][13]);
        let eq31_e1189_d_n14: f64 = (p.p87 * s.dn[742][14]);
        let eq31_e1189_d_n15: f64 = (p.p87 * s.dn[742][15]);
        let eq31_e1189_d_n16: f64 = (p.p87 * s.dn[742][16]);
        let eq31_e1189_d_n17: f64 = (p.p87 * s.dn[742][17]);
        let eq31_e1189_d_b0: f64 = (p.p87 * s.db[742][0]);
        let eq31_e1189_d_b1: f64 = (p.p87 * s.db[742][1]);
        let eq31_e1189_d_b2: f64 = (p.p87 * s.db[742][2]);
        let eq31_e1189_d_b3: f64 = (p.p87 * s.db[742][3]);
        let eq31_e1189_d_b4: f64 = (p.p87 * s.db[742][4]);
        let eq31_e1189_d_b5: f64 = (p.p87 * s.db[742][5]);
        let eq31_e1189_d_b6: f64 = (p.p87 * s.db[742][6]);
        let eq31_e1189_d_b7: f64 = (p.p87 * s.db[742][7]);
        let eq31_e1189_d_b8: f64 = (p.p87 * s.db[742][8]);
        let eq31_e1189_d_b9: f64 = (p.p87 * s.db[742][9]);
        let eq31_e1189_d_b10: f64 = (p.p87 * s.db[742][10]);
        let eq31_e1189_d_b11: f64 = (p.p87 * s.db[742][11]);
        let eq31_e1189_q: f64 = (p.p87 * eq31_e1188_q);
        let eq31_reactive_node_derivatives: [f64; 18] = [eq31_e1189_d_n0, eq31_e1189_d_n1, eq31_e1189_d_n2, eq31_e1189_d_n3, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, eq31_e1189_d_n12, eq31_e1189_d_n13, eq31_e1189_d_n14, eq31_e1189_d_n15, eq31_e1189_d_n16, eq31_e1189_d_n17];
        let eq31_reactive_branch_derivatives: [f64; 12] = [eq31_e1189_d_b0, eq31_e1189_d_b1, eq31_e1189_d_b2, eq31_e1189_d_b3, eq31_e1189_d_b4, eq31_e1189_d_b5, eq31_e1189_d_b6, eq31_e1189_d_b7, eq31_e1189_d_b8, eq31_e1189_d_b9, eq31_e1189_d_b10, eq31_e1189_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192_q: f64 = s.v[744];
        let eq32_e1193: f64 = (p.p87 * s.v[744]);
        let eq32_e1193_d_n0: f64 = (p.p87 * s.dn[744][0]);
        let eq32_e1193_d_n1: f64 = (p.p87 * s.dn[744][1]);
        let eq32_e1193_d_n2: f64 = (p.p87 * s.dn[744][2]);
        let eq32_e1193_d_n3: f64 = (p.p87 * s.dn[744][3]);
        let eq32_e1193_d_n4: f64 = (p.p87 * s.dn[744][4]);
        let eq32_e1193_d_n5: f64 = (p.p87 * s.dn[744][5]);
        let eq32_e1193_d_n6: f64 = (p.p87 * s.dn[744][6]);
        let eq32_e1193_d_n7: f64 = (p.p87 * s.dn[744][7]);
        let eq32_e1193_d_n8: f64 = (p.p87 * s.dn[744][8]);
        let eq32_e1193_d_n9: f64 = (p.p87 * s.dn[744][9]);
        let eq32_e1193_d_n10: f64 = (p.p87 * s.dn[744][10]);
        let eq32_e1193_d_n11: f64 = (p.p87 * s.dn[744][11]);
        let eq32_e1193_d_n12: f64 = (p.p87 * s.dn[744][12]);
        let eq32_e1193_d_n13: f64 = (p.p87 * s.dn[744][13]);
        let eq32_e1193_d_n14: f64 = (p.p87 * s.dn[744][14]);
        let eq32_e1193_d_n15: f64 = (p.p87 * s.dn[744][15]);
        let eq32_e1193_d_n16: f64 = (p.p87 * s.dn[744][16]);
        let eq32_e1193_d_n17: f64 = (p.p87 * s.dn[744][17]);
        let eq32_e1193_d_b0: f64 = (p.p87 * s.db[744][0]);
        let eq32_e1193_d_b1: f64 = (p.p87 * s.db[744][1]);
        let eq32_e1193_d_b2: f64 = (p.p87 * s.db[744][2]);
        let eq32_e1193_d_b3: f64 = (p.p87 * s.db[744][3]);
        let eq32_e1193_d_b4: f64 = (p.p87 * s.db[744][4]);
        let eq32_e1193_d_b5: f64 = (p.p87 * s.db[744][5]);
        let eq32_e1193_d_b6: f64 = (p.p87 * s.db[744][6]);
        let eq32_e1193_d_b7: f64 = (p.p87 * s.db[744][7]);
        let eq32_e1193_d_b8: f64 = (p.p87 * s.db[744][8]);
        let eq32_e1193_d_b9: f64 = (p.p87 * s.db[744][9]);
        let eq32_e1193_d_b10: f64 = (p.p87 * s.db[744][10]);
        let eq32_e1193_d_b11: f64 = (p.p87 * s.db[744][11]);
        let eq32_e1193_q: f64 = (p.p87 * eq32_e1192_q);
        let eq32_reactive_node_derivatives: [f64; 18] = [eq32_e1193_d_n0, eq32_e1193_d_n1, eq32_e1193_d_n2, eq32_e1193_d_n3, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, eq32_e1193_d_n12, eq32_e1193_d_n13, eq32_e1193_d_n14, eq32_e1193_d_n15, eq32_e1193_d_n16, eq32_e1193_d_n17];
        let eq32_reactive_branch_derivatives: [f64; 12] = [eq32_e1193_d_b0, eq32_e1193_d_b1, eq32_e1193_d_b2, eq32_e1193_d_b3, eq32_e1193_d_b4, eq32_e1193_d_b5, eq32_e1193_d_b6, eq32_e1193_d_b7, eq32_e1193_d_b8, eq32_e1193_d_b9, eq32_e1193_d_b10, eq32_e1193_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e1195: f64 = (-p.p87);
        let eq33_e1197_q: f64 = s.v[299];
        let eq33_e1198: f64 = (eq33_e1195 * s.v[299]);
        let eq33_e1198_d_n0: f64 = (eq33_e1195 * s.dn[299][0]);
        let eq33_e1198_d_n1: f64 = (eq33_e1195 * s.dn[299][1]);
        let eq33_e1198_d_n2: f64 = (eq33_e1195 * s.dn[299][2]);
        let eq33_e1198_d_n3: f64 = (eq33_e1195 * s.dn[299][3]);
        let eq33_e1198_d_n4: f64 = (eq33_e1195 * s.dn[299][4]);
        let eq33_e1198_d_n5: f64 = (eq33_e1195 * s.dn[299][5]);
        let eq33_e1198_d_n6: f64 = (eq33_e1195 * s.dn[299][6]);
        let eq33_e1198_d_n7: f64 = (eq33_e1195 * s.dn[299][7]);
        let eq33_e1198_d_n8: f64 = (eq33_e1195 * s.dn[299][8]);
        let eq33_e1198_d_n9: f64 = (eq33_e1195 * s.dn[299][9]);
        let eq33_e1198_d_n10: f64 = (eq33_e1195 * s.dn[299][10]);
        let eq33_e1198_d_n11: f64 = (eq33_e1195 * s.dn[299][11]);
        let eq33_e1198_d_n12: f64 = (eq33_e1195 * s.dn[299][12]);
        let eq33_e1198_d_n13: f64 = (eq33_e1195 * s.dn[299][13]);
        let eq33_e1198_d_n14: f64 = (eq33_e1195 * s.dn[299][14]);
        let eq33_e1198_d_n15: f64 = (eq33_e1195 * s.dn[299][15]);
        let eq33_e1198_d_n16: f64 = (eq33_e1195 * s.dn[299][16]);
        let eq33_e1198_d_n17: f64 = (eq33_e1195 * s.dn[299][17]);
        let eq33_e1198_d_b0: f64 = (eq33_e1195 * s.db[299][0]);
        let eq33_e1198_d_b1: f64 = (eq33_e1195 * s.db[299][1]);
        let eq33_e1198_d_b2: f64 = (eq33_e1195 * s.db[299][2]);
        let eq33_e1198_d_b3: f64 = (eq33_e1195 * s.db[299][3]);
        let eq33_e1198_d_b4: f64 = (eq33_e1195 * s.db[299][4]);
        let eq33_e1198_d_b5: f64 = (eq33_e1195 * s.db[299][5]);
        let eq33_e1198_d_b6: f64 = (eq33_e1195 * s.db[299][6]);
        let eq33_e1198_d_b7: f64 = (eq33_e1195 * s.db[299][7]);
        let eq33_e1198_d_b8: f64 = (eq33_e1195 * s.db[299][8]);
        let eq33_e1198_d_b9: f64 = (eq33_e1195 * s.db[299][9]);
        let eq33_e1198_d_b10: f64 = (eq33_e1195 * s.db[299][10]);
        let eq33_e1198_d_b11: f64 = (eq33_e1195 * s.db[299][11]);
        let eq33_e1198_q: f64 = (eq33_e1195 * eq33_e1197_q);
        let eq33_reactive_node_derivatives: [f64; 18] = [eq33_e1198_d_n0, eq33_e1198_d_n1, eq33_e1198_d_n2, eq33_e1198_d_n3, eq33_e1198_d_n4, eq33_e1198_d_n5, eq33_e1198_d_n6, eq33_e1198_d_n7, eq33_e1198_d_n8, eq33_e1198_d_n9, eq33_e1198_d_n10, eq33_e1198_d_n11, eq33_e1198_d_n12, eq33_e1198_d_n13, eq33_e1198_d_n14, eq33_e1198_d_n15, eq33_e1198_d_n16, eq33_e1198_d_n17];
        let eq33_reactive_branch_derivatives: [f64; 12] = [eq33_e1198_d_b0, eq33_e1198_d_b1, eq33_e1198_d_b2, eq33_e1198_d_b3, eq33_e1198_d_b4, eq33_e1198_d_b5, eq33_e1198_d_b6, eq33_e1198_d_b7, eq33_e1198_d_b8, eq33_e1198_d_b9, eq33_e1198_d_b10, eq33_e1198_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[0]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e1200: f64 = (-p.p87);
        let eq34_e1202_q: f64 = s.v[301];
        let eq34_e1203: f64 = (eq34_e1200 * s.v[301]);
        let eq34_e1203_d_n0: f64 = (eq34_e1200 * s.dn[301][0]);
        let eq34_e1203_d_n1: f64 = (eq34_e1200 * s.dn[301][1]);
        let eq34_e1203_d_n2: f64 = (eq34_e1200 * s.dn[301][2]);
        let eq34_e1203_d_n3: f64 = (eq34_e1200 * s.dn[301][3]);
        let eq34_e1203_d_n4: f64 = (eq34_e1200 * s.dn[301][4]);
        let eq34_e1203_d_n5: f64 = (eq34_e1200 * s.dn[301][5]);
        let eq34_e1203_d_n6: f64 = (eq34_e1200 * s.dn[301][6]);
        let eq34_e1203_d_n7: f64 = (eq34_e1200 * s.dn[301][7]);
        let eq34_e1203_d_n8: f64 = (eq34_e1200 * s.dn[301][8]);
        let eq34_e1203_d_n9: f64 = (eq34_e1200 * s.dn[301][9]);
        let eq34_e1203_d_n10: f64 = (eq34_e1200 * s.dn[301][10]);
        let eq34_e1203_d_n11: f64 = (eq34_e1200 * s.dn[301][11]);
        let eq34_e1203_d_n12: f64 = (eq34_e1200 * s.dn[301][12]);
        let eq34_e1203_d_n13: f64 = (eq34_e1200 * s.dn[301][13]);
        let eq34_e1203_d_n14: f64 = (eq34_e1200 * s.dn[301][14]);
        let eq34_e1203_d_n15: f64 = (eq34_e1200 * s.dn[301][15]);
        let eq34_e1203_d_n16: f64 = (eq34_e1200 * s.dn[301][16]);
        let eq34_e1203_d_n17: f64 = (eq34_e1200 * s.dn[301][17]);
        let eq34_e1203_d_b0: f64 = (eq34_e1200 * s.db[301][0]);
        let eq34_e1203_d_b1: f64 = (eq34_e1200 * s.db[301][1]);
        let eq34_e1203_d_b2: f64 = (eq34_e1200 * s.db[301][2]);
        let eq34_e1203_d_b3: f64 = (eq34_e1200 * s.db[301][3]);
        let eq34_e1203_d_b4: f64 = (eq34_e1200 * s.db[301][4]);
        let eq34_e1203_d_b5: f64 = (eq34_e1200 * s.db[301][5]);
        let eq34_e1203_d_b6: f64 = (eq34_e1200 * s.db[301][6]);
        let eq34_e1203_d_b7: f64 = (eq34_e1200 * s.db[301][7]);
        let eq34_e1203_d_b8: f64 = (eq34_e1200 * s.db[301][8]);
        let eq34_e1203_d_b9: f64 = (eq34_e1200 * s.db[301][9]);
        let eq34_e1203_d_b10: f64 = (eq34_e1200 * s.db[301][10]);
        let eq34_e1203_d_b11: f64 = (eq34_e1200 * s.db[301][11]);
        let eq34_e1203_q: f64 = (eq34_e1200 * eq34_e1202_q);
        let eq34_reactive_node_derivatives: [f64; 18] = [eq34_e1203_d_n0, eq34_e1203_d_n1, eq34_e1203_d_n2, eq34_e1203_d_n3, eq34_e1203_d_n4, eq34_e1203_d_n5, eq34_e1203_d_n6, eq34_e1203_d_n7, eq34_e1203_d_n8, eq34_e1203_d_n9, eq34_e1203_d_n10, eq34_e1203_d_n11, eq34_e1203_d_n12, eq34_e1203_d_n13, eq34_e1203_d_n14, eq34_e1203_d_n15, eq34_e1203_d_n16, eq34_e1203_d_n17];
        let eq34_reactive_branch_derivatives: [f64; 12] = [eq34_e1203_d_b0, eq34_e1203_d_b1, eq34_e1203_d_b2, eq34_e1203_d_b3, eq34_e1203_d_b4, eq34_e1203_d_b5, eq34_e1203_d_b6, eq34_e1203_d_b7, eq34_e1203_d_b8, eq34_e1203_d_b9, eq34_e1203_d_b10, eq34_e1203_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let eq40_e1232: f64 = ((nv14 - 0.0) * s.v[954]);
        let eq40_e1232_d_n0: f64 = ((nv14 - 0.0) * s.dn[954][0]);
        let eq40_e1232_d_n1: f64 = ((nv14 - 0.0) * s.dn[954][1]);
        let eq40_e1232_d_n2: f64 = ((nv14 - 0.0) * s.dn[954][2]);
        let eq40_e1232_d_n3: f64 = ((nv14 - 0.0) * s.dn[954][3]);
        let eq40_e1232_d_n4: f64 = ((nv14 - 0.0) * s.dn[954][4]);
        let eq40_e1232_d_n5: f64 = ((nv14 - 0.0) * s.dn[954][5]);
        let eq40_e1232_d_n6: f64 = ((nv14 - 0.0) * s.dn[954][6]);
        let eq40_e1232_d_n7: f64 = ((nv14 - 0.0) * s.dn[954][7]);
        let eq40_e1232_d_n8: f64 = ((nv14 - 0.0) * s.dn[954][8]);
        let eq40_e1232_d_n9: f64 = ((nv14 - 0.0) * s.dn[954][9]);
        let eq40_e1232_d_n10: f64 = ((nv14 - 0.0) * s.dn[954][10]);
        let eq40_e1232_d_n11: f64 = ((nv14 - 0.0) * s.dn[954][11]);
        let eq40_e1232_d_n12: f64 = ((nv14 - 0.0) * s.dn[954][12]);
        let eq40_e1232_d_n13: f64 = ((nv14 - 0.0) * s.dn[954][13]);
        let eq40_e1232_d_n14: f64 = (s.v[954] + ((nv14 - 0.0) * s.dn[954][14]));
        let eq40_e1232_d_n15: f64 = ((nv14 - 0.0) * s.dn[954][15]);
        let eq40_e1232_d_n16: f64 = ((nv14 - 0.0) * s.dn[954][16]);
        let eq40_e1232_d_n17: f64 = ((nv14 - 0.0) * s.dn[954][17]);
        let eq40_e1232_d_b0: f64 = ((nv14 - 0.0) * s.db[954][0]);
        let eq40_e1232_d_b1: f64 = ((nv14 - 0.0) * s.db[954][1]);
        let eq40_e1232_d_b2: f64 = ((nv14 - 0.0) * s.db[954][2]);
        let eq40_e1232_d_b3: f64 = ((nv14 - 0.0) * s.db[954][3]);
        let eq40_e1232_d_b4: f64 = ((nv14 - 0.0) * s.db[954][4]);
        let eq40_e1232_d_b5: f64 = ((nv14 - 0.0) * s.db[954][5]);
        let eq40_e1232_d_b6: f64 = ((nv14 - 0.0) * s.db[954][6]);
        let eq40_e1232_d_b7: f64 = ((nv14 - 0.0) * s.db[954][7]);
        let eq40_e1232_d_b8: f64 = ((nv14 - 0.0) * s.db[954][8]);
        let eq40_e1232_d_b9: f64 = ((nv14 - 0.0) * s.db[954][9]);
        let eq40_e1232_d_b10: f64 = ((nv14 - 0.0) * s.db[954][10]);
        let eq40_e1232_d_b11: f64 = ((nv14 - 0.0) * s.db[954][11]);
        let eq40_e1233_q: f64 = eq40_e1232;
        let eq40_reactive_node_derivatives: [f64; 18] = [eq40_e1232_d_n0, eq40_e1232_d_n1, eq40_e1232_d_n2, eq40_e1232_d_n3, eq40_e1232_d_n4, eq40_e1232_d_n5, eq40_e1232_d_n6, eq40_e1232_d_n7, eq40_e1232_d_n8, eq40_e1232_d_n9, eq40_e1232_d_n10, eq40_e1232_d_n11, eq40_e1232_d_n12, eq40_e1232_d_n13, eq40_e1232_d_n14, eq40_e1232_d_n15, eq40_e1232_d_n16, eq40_e1232_d_n17];
        let eq40_reactive_branch_derivatives: [f64; 12] = [eq40_e1232_d_b0, eq40_e1232_d_b1, eq40_e1232_d_b2, eq40_e1232_d_b3, eq40_e1232_d_b4, eq40_e1232_d_b5, eq40_e1232_d_b6, eq40_e1232_d_b7, eq40_e1232_d_b8, eq40_e1232_d_b9, eq40_e1232_d_b10, eq40_e1232_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv14 - 0.0) * s.v[955]);
        let eq41_e1236_d_n0: f64 = ((nv14 - 0.0) * s.dn[955][0]);
        let eq41_e1236_d_n1: f64 = ((nv14 - 0.0) * s.dn[955][1]);
        let eq41_e1236_d_n2: f64 = ((nv14 - 0.0) * s.dn[955][2]);
        let eq41_e1236_d_n3: f64 = ((nv14 - 0.0) * s.dn[955][3]);
        let eq41_e1236_d_n4: f64 = ((nv14 - 0.0) * s.dn[955][4]);
        let eq41_e1236_d_n5: f64 = ((nv14 - 0.0) * s.dn[955][5]);
        let eq41_e1236_d_n6: f64 = ((nv14 - 0.0) * s.dn[955][6]);
        let eq41_e1236_d_n7: f64 = ((nv14 - 0.0) * s.dn[955][7]);
        let eq41_e1236_d_n8: f64 = ((nv14 - 0.0) * s.dn[955][8]);
        let eq41_e1236_d_n9: f64 = ((nv14 - 0.0) * s.dn[955][9]);
        let eq41_e1236_d_n10: f64 = ((nv14 - 0.0) * s.dn[955][10]);
        let eq41_e1236_d_n11: f64 = ((nv14 - 0.0) * s.dn[955][11]);
        let eq41_e1236_d_n12: f64 = ((nv14 - 0.0) * s.dn[955][12]);
        let eq41_e1236_d_n13: f64 = ((nv14 - 0.0) * s.dn[955][13]);
        let eq41_e1236_d_n14: f64 = (s.v[955] + ((nv14 - 0.0) * s.dn[955][14]));
        let eq41_e1236_d_n15: f64 = ((nv14 - 0.0) * s.dn[955][15]);
        let eq41_e1236_d_n16: f64 = ((nv14 - 0.0) * s.dn[955][16]);
        let eq41_e1236_d_n17: f64 = ((nv14 - 0.0) * s.dn[955][17]);
        let eq41_e1236_d_b0: f64 = ((nv14 - 0.0) * s.db[955][0]);
        let eq41_e1236_d_b1: f64 = ((nv14 - 0.0) * s.db[955][1]);
        let eq41_e1236_d_b2: f64 = ((nv14 - 0.0) * s.db[955][2]);
        let eq41_e1236_d_b3: f64 = ((nv14 - 0.0) * s.db[955][3]);
        let eq41_e1236_d_b4: f64 = ((nv14 - 0.0) * s.db[955][4]);
        let eq41_e1236_d_b5: f64 = ((nv14 - 0.0) * s.db[955][5]);
        let eq41_e1236_d_b6: f64 = ((nv14 - 0.0) * s.db[955][6]);
        let eq41_e1236_d_b7: f64 = ((nv14 - 0.0) * s.db[955][7]);
        let eq41_e1236_d_b8: f64 = ((nv14 - 0.0) * s.db[955][8]);
        let eq41_e1236_d_b9: f64 = ((nv14 - 0.0) * s.db[955][9]);
        let eq41_e1236_d_b10: f64 = ((nv14 - 0.0) * s.db[955][10]);
        let eq41_e1236_d_b11: f64 = ((nv14 - 0.0) * s.db[955][11]);
        let eq41_e1237_q: f64 = eq41_e1236;
        let eq41_reactive_node_derivatives: [f64; 18] = [eq41_e1236_d_n0, eq41_e1236_d_n1, eq41_e1236_d_n2, eq41_e1236_d_n3, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, eq41_e1236_d_n11, eq41_e1236_d_n12, eq41_e1236_d_n13, eq41_e1236_d_n14, eq41_e1236_d_n15, eq41_e1236_d_n16, eq41_e1236_d_n17];
        let eq41_reactive_branch_derivatives: [f64; 12] = [eq41_e1236_d_b0, eq41_e1236_d_b1, eq41_e1236_d_b2, eq41_e1236_d_b3, eq41_e1236_d_b4, eq41_e1236_d_b5, eq41_e1236_d_b6, eq41_e1236_d_b7, eq41_e1236_d_b8, eq41_e1236_d_b9, eq41_e1236_d_b10, eq41_e1236_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1358, eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17, eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8, eq61_e1358_d_b9, eq61_e1358_d_b10, eq61_e1358_d_b11, eq61_e1358_q,) = {
    if (p.p28 != 0.0) {
        let eq61_e1355: f64 = (s.v[800] * (nv11 - 0.0));
        let eq61_e1355_d_n0: f64 = (s.dn[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_n1: f64 = (s.dn[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_n2: f64 = (s.dn[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_n3: f64 = (s.dn[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_n4: f64 = (s.dn[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_n5: f64 = (s.dn[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_n6: f64 = (s.dn[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_n7: f64 = (s.dn[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_n8: f64 = (s.dn[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_n9: f64 = (s.dn[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_n10: f64 = (s.dn[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_n11: f64 = ((s.dn[800][11] * (nv11 - 0.0)) + s.v[800]);
        let eq61_e1355_d_n12: f64 = (s.dn[800][12] * (nv11 - 0.0));
        let eq61_e1355_d_n13: f64 = (s.dn[800][13] * (nv11 - 0.0));
        let eq61_e1355_d_n14: f64 = (s.dn[800][14] * (nv11 - 0.0));
        let eq61_e1355_d_n15: f64 = (s.dn[800][15] * (nv11 - 0.0));
        let eq61_e1355_d_n16: f64 = (s.dn[800][16] * (nv11 - 0.0));
        let eq61_e1355_d_n17: f64 = (s.dn[800][17] * (nv11 - 0.0));
        let eq61_e1355_d_b0: f64 = (s.db[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_b1: f64 = (s.db[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_b2: f64 = (s.db[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_b3: f64 = (s.db[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_b4: f64 = (s.db[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_b5: f64 = (s.db[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_b6: f64 = (s.db[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_b7: f64 = (s.db[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_b8: f64 = (s.db[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_b9: f64 = (s.db[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_b10: f64 = (s.db[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_b11: f64 = (s.db[800][11] * (nv11 - 0.0));
        let eq61_e1356_q: f64 = eq61_e1355;
        (eq61_e1355, eq61_e1355_d_n0, eq61_e1355_d_n1, eq61_e1355_d_n2, eq61_e1355_d_n3, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n12, eq61_e1355_d_n13, eq61_e1355_d_n14, eq61_e1355_d_n15, eq61_e1355_d_n16, eq61_e1355_d_n17, eq61_e1355_d_b0, eq61_e1355_d_b1, eq61_e1355_d_b2, eq61_e1355_d_b3, eq61_e1355_d_b4, eq61_e1355_d_b5, eq61_e1355_d_b6, eq61_e1355_d_b7, eq61_e1355_d_b8, eq61_e1355_d_b9, eq61_e1355_d_b10, eq61_e1355_d_b11, eq61_e1356_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 18] = [eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17];
        let eq61_reactive_branch_derivatives: [f64; 12] = [eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8, eq61_e1358_d_b9, eq61_e1358_d_b10, eq61_e1358_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            None,
            nodes,
            &eq61_reactive_node_derivatives,
            branches,
            &eq61_reactive_branch_derivatives,
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
        let (eq62_e1365, eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17, eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8, eq62_e1365_d_b9, eq62_e1365_d_b10, eq62_e1365_d_b11, eq62_e1365_q,) = {
    if (p.p28 != 0.0) {
        let eq62_e1362: f64 = (s.v[801] * (nv12 - 0.0));
        let eq62_e1362_d_n0: f64 = (s.dn[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_n1: f64 = (s.dn[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_n2: f64 = (s.dn[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_n3: f64 = (s.dn[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_n4: f64 = (s.dn[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_n5: f64 = (s.dn[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_n6: f64 = (s.dn[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_n7: f64 = (s.dn[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_n8: f64 = (s.dn[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_n9: f64 = (s.dn[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_n10: f64 = (s.dn[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_n11: f64 = (s.dn[801][11] * (nv12 - 0.0));
        let eq62_e1362_d_n12: f64 = ((s.dn[801][12] * (nv12 - 0.0)) + s.v[801]);
        let eq62_e1362_d_n13: f64 = (s.dn[801][13] * (nv12 - 0.0));
        let eq62_e1362_d_n14: f64 = (s.dn[801][14] * (nv12 - 0.0));
        let eq62_e1362_d_n15: f64 = (s.dn[801][15] * (nv12 - 0.0));
        let eq62_e1362_d_n16: f64 = (s.dn[801][16] * (nv12 - 0.0));
        let eq62_e1362_d_n17: f64 = (s.dn[801][17] * (nv12 - 0.0));
        let eq62_e1362_d_b0: f64 = (s.db[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_b1: f64 = (s.db[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_b2: f64 = (s.db[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_b3: f64 = (s.db[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_b4: f64 = (s.db[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_b5: f64 = (s.db[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_b6: f64 = (s.db[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_b7: f64 = (s.db[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_b8: f64 = (s.db[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_b9: f64 = (s.db[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_b10: f64 = (s.db[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_b11: f64 = (s.db[801][11] * (nv12 - 0.0));
        let eq62_e1363_q: f64 = eq62_e1362;
        (eq62_e1362, eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1362_d_b9, eq62_e1362_d_b10, eq62_e1362_d_b11, eq62_e1363_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_reactive_node_derivatives: [f64; 18] = [eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17];
        let eq62_reactive_branch_derivatives: [f64; 12] = [eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8, eq62_e1365_d_b9, eq62_e1365_d_b10, eq62_e1365_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq62_reactive_node_derivatives,
            branches,
            &eq62_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1384, eq66_e1384_d_n13, eq66_e1384_q,) = {
    if (p.p29 != 0.0) {
        let eq66_e1382_q: f64 = (nv13 - 0.0);
        ((nv13 - 0.0), 1.0, eq66_e1382_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq66_e1384_d_n13),
        );
    }
}
