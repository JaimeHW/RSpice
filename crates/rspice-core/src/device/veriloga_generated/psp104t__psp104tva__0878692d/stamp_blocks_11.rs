#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let __rspice_deriv_cse_0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let __rspice_deriv_cse_13: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let __rspice_deriv_cse_14: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let __rspice_deriv_cse_15: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let __rspice_deriv_cse_16: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let __rspice_deriv_cse_17: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let __rspice_deriv_cse_18: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let __rspice_deriv_cse_19: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq39_e1291: f64 = (s.v[15] * s.v[306]);
        let eq39_e1291_d_n0: f64 = ((s.dn[15][0] * s.v[306]) + (s.v[15] * s.dn[306][0]));
        let eq39_e1291_d_n1: f64 = ((s.dn[15][1] * s.v[306]) + (s.v[15] * s.dn[306][1]));
        let eq39_e1291_d_n2: f64 = ((s.dn[15][2] * s.v[306]) + (s.v[15] * s.dn[306][2]));
        let eq39_e1291_d_n3: f64 = ((s.dn[15][3] * s.v[306]) + (s.v[15] * s.dn[306][3]));
        let eq39_e1291_d_n4: f64 = ((s.dn[15][4] * s.v[306]) + (s.v[15] * s.dn[306][4]));
        let eq39_e1291_d_n5: f64 = ((s.dn[15][5] * s.v[306]) + (s.v[15] * s.dn[306][5]));
        let eq39_e1291_d_n6: f64 = ((s.dn[15][6] * s.v[306]) + (s.v[15] * s.dn[306][6]));
        let eq39_e1291_d_n7: f64 = ((s.dn[15][7] * s.v[306]) + (s.v[15] * s.dn[306][7]));
        let eq39_e1291_d_n8: f64 = ((s.dn[15][8] * s.v[306]) + (s.v[15] * s.dn[306][8]));
        let eq39_e1291_d_n9: f64 = ((s.dn[15][9] * s.v[306]) + (s.v[15] * s.dn[306][9]));
        let eq39_e1291_d_n10: f64 = ((s.dn[15][10] * s.v[306]) + (s.v[15] * s.dn[306][10]));
        let eq39_e1291_d_n11: f64 = ((s.dn[15][11] * s.v[306]) + (s.v[15] * s.dn[306][11]));
        let eq39_e1291_d_n12: f64 = ((s.dn[15][12] * s.v[306]) + (s.v[15] * s.dn[306][12]));
        let eq39_e1291_d_b0: f64 = ((s.db[15][0] * s.v[306]) + (s.v[15] * s.db[306][0]));
        let eq39_e1291_d_b1: f64 = ((s.db[15][1] * s.v[306]) + (s.v[15] * s.db[306][1]));
        let eq39_e1291_d_b2: f64 = ((s.db[15][2] * s.v[306]) + (s.v[15] * s.db[306][2]));
        let eq39_e1291_d_b3: f64 = ((s.db[15][3] * s.v[306]) + (s.v[15] * s.db[306][3]));
        let eq39_e1291_d_b4: f64 = ((s.db[15][4] * s.v[306]) + (s.v[15] * s.db[306][4]));
        let eq39_e1291_d_b5: f64 = ((s.db[15][5] * s.v[306]) + (s.v[15] * s.db[306][5]));
        let eq39_e1291_d_b6: f64 = ((s.db[15][6] * s.v[306]) + (s.v[15] * s.db[306][6]));
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1293_d_n0: f64 = (eq39_e1291_d_n0 * (nv4 - 0.0));
        let eq39_e1293_d_n1: f64 = (eq39_e1291_d_n1 * (nv4 - 0.0));
        let eq39_e1293_d_n2: f64 = (eq39_e1291_d_n2 * (nv4 - 0.0));
        let eq39_e1293_d_n3: f64 = (eq39_e1291_d_n3 * (nv4 - 0.0));
        let eq39_e1293_d_n4: f64 = ((eq39_e1291_d_n4 * (nv4 - 0.0)) + eq39_e1291);
        let eq39_e1293_d_n5: f64 = (eq39_e1291_d_n5 * (nv4 - 0.0));
        let eq39_e1293_d_n6: f64 = (eq39_e1291_d_n6 * (nv4 - 0.0));
        let eq39_e1293_d_n7: f64 = (eq39_e1291_d_n7 * (nv4 - 0.0));
        let eq39_e1293_d_n8: f64 = (eq39_e1291_d_n8 * (nv4 - 0.0));
        let eq39_e1293_d_n9: f64 = (eq39_e1291_d_n9 * (nv4 - 0.0));
        let eq39_e1293_d_n10: f64 = (eq39_e1291_d_n10 * (nv4 - 0.0));
        let eq39_e1293_d_n11: f64 = (eq39_e1291_d_n11 * (nv4 - 0.0));
        let eq39_e1293_d_n12: f64 = (eq39_e1291_d_n12 * (nv4 - 0.0));
        let eq39_e1293_d_b0: f64 = (eq39_e1291_d_b0 * (nv4 - 0.0));
        let eq39_e1293_d_b1: f64 = (eq39_e1291_d_b1 * (nv4 - 0.0));
        let eq39_e1293_d_b2: f64 = (eq39_e1291_d_b2 * (nv4 - 0.0));
        let eq39_e1293_d_b3: f64 = (eq39_e1291_d_b3 * (nv4 - 0.0));
        let eq39_e1293_d_b4: f64 = (eq39_e1291_d_b4 * (nv4 - 0.0));
        let eq39_e1293_d_b5: f64 = (eq39_e1291_d_b5 * (nv4 - 0.0));
        let eq39_e1293_d_b6: f64 = (eq39_e1291_d_b6 * (nv4 - 0.0));
        let eq39_e1294_q: f64 = eq39_e1293;
        let eq39_reactive_node_derivatives: [f64; 13] = [eq39_e1293_d_n0, eq39_e1293_d_n1, eq39_e1293_d_n2, eq39_e1293_d_n3, eq39_e1293_d_n4, eq39_e1293_d_n5, eq39_e1293_d_n6, eq39_e1293_d_n7, eq39_e1293_d_n8, eq39_e1293_d_n9, eq39_e1293_d_n10, eq39_e1293_d_n11, eq39_e1293_d_n12];
        let eq39_reactive_branch_derivatives: [f64; 7] = [eq39_e1293_d_b0, eq39_e1293_d_b1, eq39_e1293_d_b2, eq39_e1293_d_b3, eq39_e1293_d_b4, eq39_e1293_d_b5, eq39_e1293_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1302: f64 = (s.v[0] * s.v[15]);
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1304_d_n0: f64 = (__rspice_deriv_cse_0 * p.p33);
        let eq41_e1304_d_n1: f64 = (__rspice_deriv_cse_1 * p.p33);
        let eq41_e1304_d_n2: f64 = (__rspice_deriv_cse_2 * p.p33);
        let eq41_e1304_d_n3: f64 = (__rspice_deriv_cse_3 * p.p33);
        let eq41_e1304_d_n4: f64 = (__rspice_deriv_cse_4 * p.p33);
        let eq41_e1304_d_n5: f64 = (__rspice_deriv_cse_5 * p.p33);
        let eq41_e1304_d_n6: f64 = (__rspice_deriv_cse_6 * p.p33);
        let eq41_e1304_d_n7: f64 = (__rspice_deriv_cse_7 * p.p33);
        let eq41_e1304_d_n8: f64 = (__rspice_deriv_cse_8 * p.p33);
        let eq41_e1304_d_n9: f64 = (__rspice_deriv_cse_9 * p.p33);
        let eq41_e1304_d_n10: f64 = (__rspice_deriv_cse_10 * p.p33);
        let eq41_e1304_d_n11: f64 = (__rspice_deriv_cse_11 * p.p33);
        let eq41_e1304_d_n12: f64 = (__rspice_deriv_cse_12 * p.p33);
        let eq41_e1304_d_b0: f64 = (__rspice_deriv_cse_13 * p.p33);
        let eq41_e1304_d_b1: f64 = (__rspice_deriv_cse_14 * p.p33);
        let eq41_e1304_d_b2: f64 = (__rspice_deriv_cse_15 * p.p33);
        let eq41_e1304_d_b3: f64 = (__rspice_deriv_cse_16 * p.p33);
        let eq41_e1304_d_b4: f64 = (__rspice_deriv_cse_17 * p.p33);
        let eq41_e1304_d_b5: f64 = (__rspice_deriv_cse_18 * p.p33);
        let eq41_e1304_d_b6: f64 = (__rspice_deriv_cse_19 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * s.v[840]);
        let eq41_e1306_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[840]) + (eq41_e1304 * s.dn[840][0]));
        let eq41_e1306_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[840]) + (eq41_e1304 * s.dn[840][1]));
        let eq41_e1306_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[840]) + (eq41_e1304 * s.dn[840][2]));
        let eq41_e1306_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[840]) + (eq41_e1304 * s.dn[840][3]));
        let eq41_e1306_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[840]) + (eq41_e1304 * s.dn[840][4]));
        let eq41_e1306_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[840]) + (eq41_e1304 * s.dn[840][5]));
        let eq41_e1306_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[840]) + (eq41_e1304 * s.dn[840][6]));
        let eq41_e1306_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[840]) + (eq41_e1304 * s.dn[840][7]));
        let eq41_e1306_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[840]) + (eq41_e1304 * s.dn[840][8]));
        let eq41_e1306_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[840]) + (eq41_e1304 * s.dn[840][9]));
        let eq41_e1306_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[840]) + (eq41_e1304 * s.dn[840][10]));
        let eq41_e1306_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[840]) + (eq41_e1304 * s.dn[840][11]));
        let eq41_e1306_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[840]) + (eq41_e1304 * s.dn[840][12]));
        let eq41_e1306_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[840]) + (eq41_e1304 * s.db[840][0]));
        let eq41_e1306_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[840]) + (eq41_e1304 * s.db[840][1]));
        let eq41_e1306_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[840]) + (eq41_e1304 * s.db[840][2]));
        let eq41_e1306_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[840]) + (eq41_e1304 * s.db[840][3]));
        let eq41_e1306_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[840]) + (eq41_e1304 * s.db[840][4]));
        let eq41_e1306_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[840]) + (eq41_e1304 * s.db[840][5]));
        let eq41_e1306_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[840]) + (eq41_e1304 * s.db[840][6]));
        let eq41_e1307_q: f64 = eq41_e1306;
        let eq41_reactive_node_derivatives: [f64; 13] = [eq41_e1306_d_n0, eq41_e1306_d_n1, eq41_e1306_d_n2, eq41_e1306_d_n3, eq41_e1306_d_n4, eq41_e1306_d_n5, eq41_e1306_d_n6, eq41_e1306_d_n7, eq41_e1306_d_n8, eq41_e1306_d_n9, eq41_e1306_d_n10, eq41_e1306_d_n11, eq41_e1306_d_n12];
        let eq41_reactive_branch_derivatives: [f64; 7] = [eq41_e1306_d_b0, eq41_e1306_d_b1, eq41_e1306_d_b2, eq41_e1306_d_b3, eq41_e1306_d_b4, eq41_e1306_d_b5, eq41_e1306_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1310: f64 = (s.v[0] * s.v[15]);
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * s.v[841]);
        let eq42_e1314_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[841]) + (eq42_e1312 * s.dn[841][0]));
        let eq42_e1314_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[841]) + (eq42_e1312 * s.dn[841][1]));
        let eq42_e1314_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[841]) + (eq42_e1312 * s.dn[841][2]));
        let eq42_e1314_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[841]) + (eq42_e1312 * s.dn[841][3]));
        let eq42_e1314_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[841]) + (eq42_e1312 * s.dn[841][4]));
        let eq42_e1314_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[841]) + (eq42_e1312 * s.dn[841][5]));
        let eq42_e1314_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[841]) + (eq42_e1312 * s.dn[841][6]));
        let eq42_e1314_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[841]) + (eq42_e1312 * s.dn[841][7]));
        let eq42_e1314_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[841]) + (eq42_e1312 * s.dn[841][8]));
        let eq42_e1314_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[841]) + (eq42_e1312 * s.dn[841][9]));
        let eq42_e1314_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[841]) + (eq42_e1312 * s.dn[841][10]));
        let eq42_e1314_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[841]) + (eq42_e1312 * s.dn[841][11]));
        let eq42_e1314_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[841]) + (eq42_e1312 * s.dn[841][12]));
        let eq42_e1314_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[841]) + (eq42_e1312 * s.db[841][0]));
        let eq42_e1314_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[841]) + (eq42_e1312 * s.db[841][1]));
        let eq42_e1314_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[841]) + (eq42_e1312 * s.db[841][2]));
        let eq42_e1314_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[841]) + (eq42_e1312 * s.db[841][3]));
        let eq42_e1314_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[841]) + (eq42_e1312 * s.db[841][4]));
        let eq42_e1314_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[841]) + (eq42_e1312 * s.db[841][5]));
        let eq42_e1314_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[841]) + (eq42_e1312 * s.db[841][6]));
        let eq42_e1315_q: f64 = eq42_e1314;
        let eq42_reactive_node_derivatives: [f64; 13] = [eq42_e1314_d_n0, eq42_e1314_d_n1, eq42_e1314_d_n2, eq42_e1314_d_n3, eq42_e1314_d_n4, eq42_e1314_d_n5, eq42_e1314_d_n6, eq42_e1314_d_n7, eq42_e1314_d_n8, eq42_e1314_d_n9, eq42_e1314_d_n10, eq42_e1314_d_n11, eq42_e1314_d_n12];
        let eq42_reactive_branch_derivatives: [f64; 7] = [eq42_e1314_d_b0, eq42_e1314_d_b1, eq42_e1314_d_b2, eq42_e1314_d_b3, eq42_e1314_d_b4, eq42_e1314_d_b5, eq42_e1314_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e1318: f64 = (s.v[0] * s.v[15]);
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * s.v[842]);
        let eq43_e1322_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[842]) + (eq43_e1320 * s.dn[842][0]));
        let eq43_e1322_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[842]) + (eq43_e1320 * s.dn[842][1]));
        let eq43_e1322_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[842]) + (eq43_e1320 * s.dn[842][2]));
        let eq43_e1322_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[842]) + (eq43_e1320 * s.dn[842][3]));
        let eq43_e1322_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[842]) + (eq43_e1320 * s.dn[842][4]));
        let eq43_e1322_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[842]) + (eq43_e1320 * s.dn[842][5]));
        let eq43_e1322_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[842]) + (eq43_e1320 * s.dn[842][6]));
        let eq43_e1322_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[842]) + (eq43_e1320 * s.dn[842][7]));
        let eq43_e1322_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[842]) + (eq43_e1320 * s.dn[842][8]));
        let eq43_e1322_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[842]) + (eq43_e1320 * s.dn[842][9]));
        let eq43_e1322_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[842]) + (eq43_e1320 * s.dn[842][10]));
        let eq43_e1322_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[842]) + (eq43_e1320 * s.dn[842][11]));
        let eq43_e1322_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[842]) + (eq43_e1320 * s.dn[842][12]));
        let eq43_e1322_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[842]) + (eq43_e1320 * s.db[842][0]));
        let eq43_e1322_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[842]) + (eq43_e1320 * s.db[842][1]));
        let eq43_e1322_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[842]) + (eq43_e1320 * s.db[842][2]));
        let eq43_e1322_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[842]) + (eq43_e1320 * s.db[842][3]));
        let eq43_e1322_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[842]) + (eq43_e1320 * s.db[842][4]));
        let eq43_e1322_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[842]) + (eq43_e1320 * s.db[842][5]));
        let eq43_e1322_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[842]) + (eq43_e1320 * s.db[842][6]));
        let eq43_e1323_q: f64 = eq43_e1322;
        let eq43_reactive_node_derivatives: [f64; 13] = [eq43_e1322_d_n0, eq43_e1322_d_n1, eq43_e1322_d_n2, eq43_e1322_d_n3, eq43_e1322_d_n4, eq43_e1322_d_n5, eq43_e1322_d_n6, eq43_e1322_d_n7, eq43_e1322_d_n8, eq43_e1322_d_n9, eq43_e1322_d_n10, eq43_e1322_d_n11, eq43_e1322_d_n12];
        let eq43_reactive_branch_derivatives: [f64; 7] = [eq43_e1322_d_b0, eq43_e1322_d_b1, eq43_e1322_d_b2, eq43_e1322_d_b3, eq43_e1322_d_b4, eq43_e1322_d_b5, eq43_e1322_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let eq44_e1326: f64 = (s.v[0] * s.v[15]);
        let eq44_e1328: f64 = (eq44_e1326 * p.p33);
        let eq44_e1330: f64 = (eq44_e1328 * s.v[843]);
        let eq44_e1330_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[843]) + (eq44_e1328 * s.dn[843][0]));
        let eq44_e1330_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[843]) + (eq44_e1328 * s.dn[843][1]));
        let eq44_e1330_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[843]) + (eq44_e1328 * s.dn[843][2]));
        let eq44_e1330_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[843]) + (eq44_e1328 * s.dn[843][3]));
        let eq44_e1330_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[843]) + (eq44_e1328 * s.dn[843][4]));
        let eq44_e1330_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[843]) + (eq44_e1328 * s.dn[843][5]));
        let eq44_e1330_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[843]) + (eq44_e1328 * s.dn[843][6]));
        let eq44_e1330_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[843]) + (eq44_e1328 * s.dn[843][7]));
        let eq44_e1330_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[843]) + (eq44_e1328 * s.dn[843][8]));
        let eq44_e1330_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[843]) + (eq44_e1328 * s.dn[843][9]));
        let eq44_e1330_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[843]) + (eq44_e1328 * s.dn[843][10]));
        let eq44_e1330_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[843]) + (eq44_e1328 * s.dn[843][11]));
        let eq44_e1330_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[843]) + (eq44_e1328 * s.dn[843][12]));
        let eq44_e1330_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[843]) + (eq44_e1328 * s.db[843][0]));
        let eq44_e1330_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[843]) + (eq44_e1328 * s.db[843][1]));
        let eq44_e1330_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[843]) + (eq44_e1328 * s.db[843][2]));
        let eq44_e1330_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[843]) + (eq44_e1328 * s.db[843][3]));
        let eq44_e1330_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[843]) + (eq44_e1328 * s.db[843][4]));
        let eq44_e1330_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[843]) + (eq44_e1328 * s.db[843][5]));
        let eq44_e1330_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[843]) + (eq44_e1328 * s.db[843][6]));
        let eq44_e1331_q: f64 = eq44_e1330;
        let eq44_reactive_node_derivatives: [f64; 13] = [eq44_e1330_d_n0, eq44_e1330_d_n1, eq44_e1330_d_n2, eq44_e1330_d_n3, eq44_e1330_d_n4, eq44_e1330_d_n5, eq44_e1330_d_n6, eq44_e1330_d_n7, eq44_e1330_d_n8, eq44_e1330_d_n9, eq44_e1330_d_n10, eq44_e1330_d_n11, eq44_e1330_d_n12];
        let eq44_reactive_branch_derivatives: [f64; 7] = [eq44_e1330_d_b0, eq44_e1330_d_b1, eq44_e1330_d_b2, eq44_e1330_d_b3, eq44_e1330_d_b4, eq44_e1330_d_b5, eq44_e1330_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq44_reactive_node_derivatives,
            branches,
            &eq44_reactive_branch_derivatives,
            multiplicity,
        );
        let eq45_e1334: f64 = (s.v[0] * s.v[15]);
        let eq45_e1336: f64 = (eq45_e1334 * p.p33);
        let eq45_e1338: f64 = (eq45_e1336 * s.v[844]);
        let eq45_e1338_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[844]) + (eq45_e1336 * s.dn[844][0]));
        let eq45_e1338_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[844]) + (eq45_e1336 * s.dn[844][1]));
        let eq45_e1338_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[844]) + (eq45_e1336 * s.dn[844][2]));
        let eq45_e1338_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[844]) + (eq45_e1336 * s.dn[844][3]));
        let eq45_e1338_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[844]) + (eq45_e1336 * s.dn[844][4]));
        let eq45_e1338_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[844]) + (eq45_e1336 * s.dn[844][5]));
        let eq45_e1338_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[844]) + (eq45_e1336 * s.dn[844][6]));
        let eq45_e1338_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[844]) + (eq45_e1336 * s.dn[844][7]));
        let eq45_e1338_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[844]) + (eq45_e1336 * s.dn[844][8]));
        let eq45_e1338_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[844]) + (eq45_e1336 * s.dn[844][9]));
        let eq45_e1338_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[844]) + (eq45_e1336 * s.dn[844][10]));
        let eq45_e1338_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[844]) + (eq45_e1336 * s.dn[844][11]));
        let eq45_e1338_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[844]) + (eq45_e1336 * s.dn[844][12]));
        let eq45_e1338_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[844]) + (eq45_e1336 * s.db[844][0]));
        let eq45_e1338_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[844]) + (eq45_e1336 * s.db[844][1]));
        let eq45_e1338_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[844]) + (eq45_e1336 * s.db[844][2]));
        let eq45_e1338_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[844]) + (eq45_e1336 * s.db[844][3]));
        let eq45_e1338_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[844]) + (eq45_e1336 * s.db[844][4]));
        let eq45_e1338_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[844]) + (eq45_e1336 * s.db[844][5]));
        let eq45_e1338_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[844]) + (eq45_e1336 * s.db[844][6]));
        let eq45_e1339_q: f64 = eq45_e1338;
        let eq45_reactive_node_derivatives: [f64; 13] = [eq45_e1338_d_n0, eq45_e1338_d_n1, eq45_e1338_d_n2, eq45_e1338_d_n3, eq45_e1338_d_n4, eq45_e1338_d_n5, eq45_e1338_d_n6, eq45_e1338_d_n7, eq45_e1338_d_n8, eq45_e1338_d_n9, eq45_e1338_d_n10, eq45_e1338_d_n11, eq45_e1338_d_n12];
        let eq45_reactive_branch_derivatives: [f64; 7] = [eq45_e1338_d_b0, eq45_e1338_d_b1, eq45_e1338_d_b2, eq45_e1338_d_b3, eq45_e1338_d_b4, eq45_e1338_d_b5, eq45_e1338_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1342: f64 = (s.v[0] * s.v[15]);
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * s.v[845]);
        let eq46_e1346_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[845]) + (eq46_e1344 * s.dn[845][0]));
        let eq46_e1346_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[845]) + (eq46_e1344 * s.dn[845][1]));
        let eq46_e1346_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[845]) + (eq46_e1344 * s.dn[845][2]));
        let eq46_e1346_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[845]) + (eq46_e1344 * s.dn[845][3]));
        let eq46_e1346_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[845]) + (eq46_e1344 * s.dn[845][4]));
        let eq46_e1346_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[845]) + (eq46_e1344 * s.dn[845][5]));
        let eq46_e1346_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[845]) + (eq46_e1344 * s.dn[845][6]));
        let eq46_e1346_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[845]) + (eq46_e1344 * s.dn[845][7]));
        let eq46_e1346_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[845]) + (eq46_e1344 * s.dn[845][8]));
        let eq46_e1346_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[845]) + (eq46_e1344 * s.dn[845][9]));
        let eq46_e1346_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[845]) + (eq46_e1344 * s.dn[845][10]));
        let eq46_e1346_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[845]) + (eq46_e1344 * s.dn[845][11]));
        let eq46_e1346_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[845]) + (eq46_e1344 * s.dn[845][12]));
        let eq46_e1346_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[845]) + (eq46_e1344 * s.db[845][0]));
        let eq46_e1346_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[845]) + (eq46_e1344 * s.db[845][1]));
        let eq46_e1346_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[845]) + (eq46_e1344 * s.db[845][2]));
        let eq46_e1346_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[845]) + (eq46_e1344 * s.db[845][3]));
        let eq46_e1346_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[845]) + (eq46_e1344 * s.db[845][4]));
        let eq46_e1346_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[845]) + (eq46_e1344 * s.db[845][5]));
        let eq46_e1346_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[845]) + (eq46_e1344 * s.db[845][6]));
        let eq46_e1347_q: f64 = eq46_e1346;
        let eq46_reactive_node_derivatives: [f64; 13] = [eq46_e1346_d_n0, eq46_e1346_d_n1, eq46_e1346_d_n2, eq46_e1346_d_n3, eq46_e1346_d_n4, eq46_e1346_d_n5, eq46_e1346_d_n6, eq46_e1346_d_n7, eq46_e1346_d_n8, eq46_e1346_d_n9, eq46_e1346_d_n10, eq46_e1346_d_n11, eq46_e1346_d_n12];
        let eq46_reactive_branch_derivatives: [f64; 7] = [eq46_e1346_d_b0, eq46_e1346_d_b1, eq46_e1346_d_b2, eq46_e1346_d_b3, eq46_e1346_d_b4, eq46_e1346_d_b5, eq46_e1346_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[9]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let __rspice_deriv_cse_0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let __rspice_deriv_cse_13: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let __rspice_deriv_cse_14: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let __rspice_deriv_cse_15: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let __rspice_deriv_cse_16: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let __rspice_deriv_cse_17: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let __rspice_deriv_cse_18: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let __rspice_deriv_cse_19: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq47_e1350: f64 = (s.v[0] * s.v[15]);
        let eq47_e1352: f64 = (eq47_e1350 * p.p33);
        let eq47_e1352_d_n0: f64 = (__rspice_deriv_cse_0 * p.p33);
        let eq47_e1352_d_n1: f64 = (__rspice_deriv_cse_1 * p.p33);
        let eq47_e1352_d_n2: f64 = (__rspice_deriv_cse_2 * p.p33);
        let eq47_e1352_d_n3: f64 = (__rspice_deriv_cse_3 * p.p33);
        let eq47_e1352_d_n4: f64 = (__rspice_deriv_cse_4 * p.p33);
        let eq47_e1352_d_n5: f64 = (__rspice_deriv_cse_5 * p.p33);
        let eq47_e1352_d_n6: f64 = (__rspice_deriv_cse_6 * p.p33);
        let eq47_e1352_d_n7: f64 = (__rspice_deriv_cse_7 * p.p33);
        let eq47_e1352_d_n8: f64 = (__rspice_deriv_cse_8 * p.p33);
        let eq47_e1352_d_n9: f64 = (__rspice_deriv_cse_9 * p.p33);
        let eq47_e1352_d_n10: f64 = (__rspice_deriv_cse_10 * p.p33);
        let eq47_e1352_d_n11: f64 = (__rspice_deriv_cse_11 * p.p33);
        let eq47_e1352_d_n12: f64 = (__rspice_deriv_cse_12 * p.p33);
        let eq47_e1352_d_b0: f64 = (__rspice_deriv_cse_13 * p.p33);
        let eq47_e1352_d_b1: f64 = (__rspice_deriv_cse_14 * p.p33);
        let eq47_e1352_d_b2: f64 = (__rspice_deriv_cse_15 * p.p33);
        let eq47_e1352_d_b3: f64 = (__rspice_deriv_cse_16 * p.p33);
        let eq47_e1352_d_b4: f64 = (__rspice_deriv_cse_17 * p.p33);
        let eq47_e1352_d_b5: f64 = (__rspice_deriv_cse_18 * p.p33);
        let eq47_e1352_d_b6: f64 = (__rspice_deriv_cse_19 * p.p33);
        let eq47_e1354: f64 = (eq47_e1352 * s.v[846]);
        let eq47_e1354_d_n0: f64 = ((eq47_e1352_d_n0 * s.v[846]) + (eq47_e1352 * s.dn[846][0]));
        let eq47_e1354_d_n1: f64 = ((eq47_e1352_d_n1 * s.v[846]) + (eq47_e1352 * s.dn[846][1]));
        let eq47_e1354_d_n2: f64 = ((eq47_e1352_d_n2 * s.v[846]) + (eq47_e1352 * s.dn[846][2]));
        let eq47_e1354_d_n3: f64 = ((eq47_e1352_d_n3 * s.v[846]) + (eq47_e1352 * s.dn[846][3]));
        let eq47_e1354_d_n4: f64 = ((eq47_e1352_d_n4 * s.v[846]) + (eq47_e1352 * s.dn[846][4]));
        let eq47_e1354_d_n5: f64 = ((eq47_e1352_d_n5 * s.v[846]) + (eq47_e1352 * s.dn[846][5]));
        let eq47_e1354_d_n6: f64 = ((eq47_e1352_d_n6 * s.v[846]) + (eq47_e1352 * s.dn[846][6]));
        let eq47_e1354_d_n7: f64 = ((eq47_e1352_d_n7 * s.v[846]) + (eq47_e1352 * s.dn[846][7]));
        let eq47_e1354_d_n8: f64 = ((eq47_e1352_d_n8 * s.v[846]) + (eq47_e1352 * s.dn[846][8]));
        let eq47_e1354_d_n9: f64 = ((eq47_e1352_d_n9 * s.v[846]) + (eq47_e1352 * s.dn[846][9]));
        let eq47_e1354_d_n10: f64 = ((eq47_e1352_d_n10 * s.v[846]) + (eq47_e1352 * s.dn[846][10]));
        let eq47_e1354_d_n11: f64 = ((eq47_e1352_d_n11 * s.v[846]) + (eq47_e1352 * s.dn[846][11]));
        let eq47_e1354_d_n12: f64 = ((eq47_e1352_d_n12 * s.v[846]) + (eq47_e1352 * s.dn[846][12]));
        let eq47_e1354_d_b0: f64 = ((eq47_e1352_d_b0 * s.v[846]) + (eq47_e1352 * s.db[846][0]));
        let eq47_e1354_d_b1: f64 = ((eq47_e1352_d_b1 * s.v[846]) + (eq47_e1352 * s.db[846][1]));
        let eq47_e1354_d_b2: f64 = ((eq47_e1352_d_b2 * s.v[846]) + (eq47_e1352 * s.db[846][2]));
        let eq47_e1354_d_b3: f64 = ((eq47_e1352_d_b3 * s.v[846]) + (eq47_e1352 * s.db[846][3]));
        let eq47_e1354_d_b4: f64 = ((eq47_e1352_d_b4 * s.v[846]) + (eq47_e1352 * s.db[846][4]));
        let eq47_e1354_d_b5: f64 = ((eq47_e1352_d_b5 * s.v[846]) + (eq47_e1352 * s.db[846][5]));
        let eq47_e1354_d_b6: f64 = ((eq47_e1352_d_b6 * s.v[846]) + (eq47_e1352 * s.db[846][6]));
        let eq47_e1355_q: f64 = eq47_e1354;
        let eq47_reactive_node_derivatives: [f64; 13] = [eq47_e1354_d_n0, eq47_e1354_d_n1, eq47_e1354_d_n2, eq47_e1354_d_n3, eq47_e1354_d_n4, eq47_e1354_d_n5, eq47_e1354_d_n6, eq47_e1354_d_n7, eq47_e1354_d_n8, eq47_e1354_d_n9, eq47_e1354_d_n10, eq47_e1354_d_n11, eq47_e1354_d_n12];
        let eq47_reactive_branch_derivatives: [f64; 7] = [eq47_e1354_d_b0, eq47_e1354_d_b1, eq47_e1354_d_b2, eq47_e1354_d_b3, eq47_e1354_d_b4, eq47_e1354_d_b5, eq47_e1354_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let eq48_e1358: f64 = (s.v[0] * s.v[15]);
        let eq48_e1360: f64 = (eq48_e1358 * p.p33);
        let eq48_e1362: f64 = (eq48_e1360 * s.v[847]);
        let eq48_e1362_d_n0: f64 = ((eq47_e1352_d_n0 * s.v[847]) + (eq48_e1360 * s.dn[847][0]));
        let eq48_e1362_d_n1: f64 = ((eq47_e1352_d_n1 * s.v[847]) + (eq48_e1360 * s.dn[847][1]));
        let eq48_e1362_d_n2: f64 = ((eq47_e1352_d_n2 * s.v[847]) + (eq48_e1360 * s.dn[847][2]));
        let eq48_e1362_d_n3: f64 = ((eq47_e1352_d_n3 * s.v[847]) + (eq48_e1360 * s.dn[847][3]));
        let eq48_e1362_d_n4: f64 = ((eq47_e1352_d_n4 * s.v[847]) + (eq48_e1360 * s.dn[847][4]));
        let eq48_e1362_d_n5: f64 = ((eq47_e1352_d_n5 * s.v[847]) + (eq48_e1360 * s.dn[847][5]));
        let eq48_e1362_d_n6: f64 = ((eq47_e1352_d_n6 * s.v[847]) + (eq48_e1360 * s.dn[847][6]));
        let eq48_e1362_d_n7: f64 = ((eq47_e1352_d_n7 * s.v[847]) + (eq48_e1360 * s.dn[847][7]));
        let eq48_e1362_d_n8: f64 = ((eq47_e1352_d_n8 * s.v[847]) + (eq48_e1360 * s.dn[847][8]));
        let eq48_e1362_d_n9: f64 = ((eq47_e1352_d_n9 * s.v[847]) + (eq48_e1360 * s.dn[847][9]));
        let eq48_e1362_d_n10: f64 = ((eq47_e1352_d_n10 * s.v[847]) + (eq48_e1360 * s.dn[847][10]));
        let eq48_e1362_d_n11: f64 = ((eq47_e1352_d_n11 * s.v[847]) + (eq48_e1360 * s.dn[847][11]));
        let eq48_e1362_d_n12: f64 = ((eq47_e1352_d_n12 * s.v[847]) + (eq48_e1360 * s.dn[847][12]));
        let eq48_e1362_d_b0: f64 = ((eq47_e1352_d_b0 * s.v[847]) + (eq48_e1360 * s.db[847][0]));
        let eq48_e1362_d_b1: f64 = ((eq47_e1352_d_b1 * s.v[847]) + (eq48_e1360 * s.db[847][1]));
        let eq48_e1362_d_b2: f64 = ((eq47_e1352_d_b2 * s.v[847]) + (eq48_e1360 * s.db[847][2]));
        let eq48_e1362_d_b3: f64 = ((eq47_e1352_d_b3 * s.v[847]) + (eq48_e1360 * s.db[847][3]));
        let eq48_e1362_d_b4: f64 = ((eq47_e1352_d_b4 * s.v[847]) + (eq48_e1360 * s.db[847][4]));
        let eq48_e1362_d_b5: f64 = ((eq47_e1352_d_b5 * s.v[847]) + (eq48_e1360 * s.db[847][5]));
        let eq48_e1362_d_b6: f64 = ((eq47_e1352_d_b6 * s.v[847]) + (eq48_e1360 * s.db[847][6]));
        let eq48_e1363_q: f64 = eq48_e1362;
        let eq48_reactive_node_derivatives: [f64; 13] = [eq48_e1362_d_n0, eq48_e1362_d_n1, eq48_e1362_d_n2, eq48_e1362_d_n3, eq48_e1362_d_n4, eq48_e1362_d_n5, eq48_e1362_d_n6, eq48_e1362_d_n7, eq48_e1362_d_n8, eq48_e1362_d_n9, eq48_e1362_d_n10, eq48_e1362_d_n11, eq48_e1362_d_n12];
        let eq48_reactive_branch_derivatives: [f64; 7] = [eq48_e1362_d_b0, eq48_e1362_d_b1, eq48_e1362_d_b2, eq48_e1362_d_b3, eq48_e1362_d_b4, eq48_e1362_d_b5, eq48_e1362_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let eq51_e1374: f64 = (s.v[849] * (nv5 - 0.0));
        let eq51_e1374_d_n0: f64 = (s.dn[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_n1: f64 = (s.dn[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_n2: f64 = (s.dn[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_n3: f64 = (s.dn[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (s.dn[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_n5: f64 = ((s.dn[849][5] * (nv5 - 0.0)) + s.v[849]);
        let eq51_e1374_d_n6: f64 = (s.dn[849][6] * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (s.dn[849][7] * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (s.dn[849][8] * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (s.dn[849][9] * (nv5 - 0.0));
        let eq51_e1374_d_n10: f64 = (s.dn[849][10] * (nv5 - 0.0));
        let eq51_e1374_d_n11: f64 = (s.dn[849][11] * (nv5 - 0.0));
        let eq51_e1374_d_n12: f64 = (s.dn[849][12] * (nv5 - 0.0));
        let eq51_e1374_d_b0: f64 = (s.db[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_b1: f64 = (s.db[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_b2: f64 = (s.db[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_b3: f64 = (s.db[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_b4: f64 = (s.db[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_b5: f64 = (s.db[849][5] * (nv5 - 0.0));
        let eq51_e1374_d_b6: f64 = (s.db[849][6] * (nv5 - 0.0));
        let eq51_e1375_q: f64 = eq51_e1374;
        let eq51_reactive_node_derivatives: [f64; 13] = [eq51_e1374_d_n0, eq51_e1374_d_n1, eq51_e1374_d_n2, eq51_e1374_d_n3, eq51_e1374_d_n4, eq51_e1374_d_n5, eq51_e1374_d_n6, eq51_e1374_d_n7, eq51_e1374_d_n8, eq51_e1374_d_n9, eq51_e1374_d_n10, eq51_e1374_d_n11, eq51_e1374_d_n12];
        let eq51_reactive_branch_derivatives: [f64; 7] = [eq51_e1374_d_b0, eq51_e1374_d_b1, eq51_e1374_d_b2, eq51_e1374_d_b3, eq51_e1374_d_b4, eq51_e1374_d_b5, eq51_e1374_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let eq52_e1378: f64 = (s.v[15] * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / (2.0 * eq52_e1379);
        let eq52_e1379_d_n0: f64 = ((s.dn[15][0] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n1: f64 = ((s.dn[15][1] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n2: f64 = ((s.dn[15][2] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n3: f64 = ((s.dn[15][3] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n4: f64 = ((s.dn[15][4] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n5: f64 = ((s.dn[15][5] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n6: f64 = ((s.dn[15][6] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n7: f64 = ((s.dn[15][7] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n8: f64 = ((s.dn[15][8] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n9: f64 = ((s.dn[15][9] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n10: f64 = ((s.dn[15][10] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n11: f64 = ((s.dn[15][11] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n12: f64 = ((s.dn[15][12] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b0: f64 = ((s.db[15][0] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b1: f64 = ((s.db[15][1] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b2: f64 = ((s.db[15][2] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b3: f64 = ((s.db[15][3] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b4: f64 = ((s.db[15][4] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b5: f64 = ((s.db[15][5] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b6: f64 = ((s.db[15][6] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1381_d_n0: f64 = (eq52_e1379_d_n0 * 0.5);
        let eq52_e1381_d_n1: f64 = (eq52_e1379_d_n1 * 0.5);
        let eq52_e1381_d_n2: f64 = (eq52_e1379_d_n2 * 0.5);
        let eq52_e1381_d_n3: f64 = (eq52_e1379_d_n3 * 0.5);
        let eq52_e1381_d_n4: f64 = (eq52_e1379_d_n4 * 0.5);
        let eq52_e1381_d_n5: f64 = (eq52_e1379_d_n5 * 0.5);
        let eq52_e1381_d_n6: f64 = (eq52_e1379_d_n6 * 0.5);
        let eq52_e1381_d_n7: f64 = (eq52_e1379_d_n7 * 0.5);
        let eq52_e1381_d_n8: f64 = (eq52_e1379_d_n8 * 0.5);
        let eq52_e1381_d_n9: f64 = (eq52_e1379_d_n9 * 0.5);
        let eq52_e1381_d_n10: f64 = (eq52_e1379_d_n10 * 0.5);
        let eq52_e1381_d_n11: f64 = (eq52_e1379_d_n11 * 0.5);
        let eq52_e1381_d_n12: f64 = (eq52_e1379_d_n12 * 0.5);
        let eq52_e1381_d_b0: f64 = (eq52_e1379_d_b0 * 0.5);
        let eq52_e1381_d_b1: f64 = (eq52_e1379_d_b1 * 0.5);
        let eq52_e1381_d_b2: f64 = (eq52_e1379_d_b2 * 0.5);
        let eq52_e1381_d_b3: f64 = (eq52_e1379_d_b3 * 0.5);
        let eq52_e1381_d_b4: f64 = (eq52_e1379_d_b4 * 0.5);
        let eq52_e1381_d_b5: f64 = (eq52_e1379_d_b5 * 0.5);
        let eq52_e1381_d_b6: f64 = (eq52_e1379_d_b6 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * s.v[849]);
        let eq52_e1383_d_n0: f64 = ((eq52_e1381_d_n0 * s.v[849]) + (eq52_e1381 * s.dn[849][0]));
        let eq52_e1383_d_n1: f64 = ((eq52_e1381_d_n1 * s.v[849]) + (eq52_e1381 * s.dn[849][1]));
        let eq52_e1383_d_n2: f64 = ((eq52_e1381_d_n2 * s.v[849]) + (eq52_e1381 * s.dn[849][2]));
        let eq52_e1383_d_n3: f64 = ((eq52_e1381_d_n3 * s.v[849]) + (eq52_e1381 * s.dn[849][3]));
        let eq52_e1383_d_n4: f64 = ((eq52_e1381_d_n4 * s.v[849]) + (eq52_e1381 * s.dn[849][4]));
        let eq52_e1383_d_n5: f64 = ((eq52_e1381_d_n5 * s.v[849]) + (eq52_e1381 * s.dn[849][5]));
        let eq52_e1383_d_n6: f64 = ((eq52_e1381_d_n6 * s.v[849]) + (eq52_e1381 * s.dn[849][6]));
        let eq52_e1383_d_n7: f64 = ((eq52_e1381_d_n7 * s.v[849]) + (eq52_e1381 * s.dn[849][7]));
        let eq52_e1383_d_n8: f64 = ((eq52_e1381_d_n8 * s.v[849]) + (eq52_e1381 * s.dn[849][8]));
        let eq52_e1383_d_n9: f64 = ((eq52_e1381_d_n9 * s.v[849]) + (eq52_e1381 * s.dn[849][9]));
        let eq52_e1383_d_n10: f64 = ((eq52_e1381_d_n10 * s.v[849]) + (eq52_e1381 * s.dn[849][10]));
        let eq52_e1383_d_n11: f64 = ((eq52_e1381_d_n11 * s.v[849]) + (eq52_e1381 * s.dn[849][11]));
        let eq52_e1383_d_n12: f64 = ((eq52_e1381_d_n12 * s.v[849]) + (eq52_e1381 * s.dn[849][12]));
        let eq52_e1383_d_b0: f64 = ((eq52_e1381_d_b0 * s.v[849]) + (eq52_e1381 * s.db[849][0]));
        let eq52_e1383_d_b1: f64 = ((eq52_e1381_d_b1 * s.v[849]) + (eq52_e1381 * s.db[849][1]));
        let eq52_e1383_d_b2: f64 = ((eq52_e1381_d_b2 * s.v[849]) + (eq52_e1381 * s.db[849][2]));
        let eq52_e1383_d_b3: f64 = ((eq52_e1381_d_b3 * s.v[849]) + (eq52_e1381 * s.db[849][3]));
        let eq52_e1383_d_b4: f64 = ((eq52_e1381_d_b4 * s.v[849]) + (eq52_e1381 * s.db[849][4]));
        let eq52_e1383_d_b5: f64 = ((eq52_e1381_d_b5 * s.v[849]) + (eq52_e1381 * s.db[849][5]));
        let eq52_e1383_d_b6: f64 = ((eq52_e1381_d_b6 * s.v[849]) + (eq52_e1381 * s.db[849][6]));
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n0: f64 = (eq52_e1383_d_n0 * (nv5 - 0.0));
        let eq52_e1385_d_n1: f64 = (eq52_e1383_d_n1 * (nv5 - 0.0));
        let eq52_e1385_d_n2: f64 = (eq52_e1383_d_n2 * (nv5 - 0.0));
        let eq52_e1385_d_n3: f64 = (eq52_e1383_d_n3 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n5: f64 = ((eq52_e1383_d_n5 * (nv5 - 0.0)) + eq52_e1383);
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1385_d_n10: f64 = (eq52_e1383_d_n10 * (nv5 - 0.0));
        let eq52_e1385_d_n11: f64 = (eq52_e1383_d_n11 * (nv5 - 0.0));
        let eq52_e1385_d_n12: f64 = (eq52_e1383_d_n12 * (nv5 - 0.0));
        let eq52_e1385_d_b0: f64 = (eq52_e1383_d_b0 * (nv5 - 0.0));
        let eq52_e1385_d_b1: f64 = (eq52_e1383_d_b1 * (nv5 - 0.0));
        let eq52_e1385_d_b2: f64 = (eq52_e1383_d_b2 * (nv5 - 0.0));
        let eq52_e1385_d_b3: f64 = (eq52_e1383_d_b3 * (nv5 - 0.0));
        let eq52_e1385_d_b4: f64 = (eq52_e1383_d_b4 * (nv5 - 0.0));
        let eq52_e1385_d_b5: f64 = (eq52_e1383_d_b5 * (nv5 - 0.0));
        let eq52_e1385_d_b6: f64 = (eq52_e1383_d_b6 * (nv5 - 0.0));
        let eq52_e1386_q: f64 = eq52_e1385;
        let eq52_e1387: f64 = (-eq52_e1385);
        let eq52_e1387_q: f64 = (-eq52_e1386_q);
        let eq52_reactive_node_derivatives: [f64; 13] = [(-eq52_e1385_d_n0), (-eq52_e1385_d_n1), (-eq52_e1385_d_n2), (-eq52_e1385_d_n3), (-eq52_e1385_d_n4), (-eq52_e1385_d_n5), (-eq52_e1385_d_n6), (-eq52_e1385_d_n7), (-eq52_e1385_d_n8), (-eq52_e1385_d_n9), (-eq52_e1385_d_n10), (-eq52_e1385_d_n11), (-eq52_e1385_d_n12)];
        let eq52_reactive_branch_derivatives: [f64; 7] = [(-eq52_e1385_d_b0), (-eq52_e1385_d_b1), (-eq52_e1385_d_b2), (-eq52_e1385_d_b3), (-eq52_e1385_d_b4), (-eq52_e1385_d_b5), (-eq52_e1385_d_b6)];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let eq53_e1390: f64 = (s.v[15] * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let __rspice_inv_cse_1: f64 = 1.0 / (2.0 * eq53_e1391);
        let eq53_e1391_d_n0: f64 = ((s.dn[15][0] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n1: f64 = ((s.dn[15][1] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n2: f64 = ((s.dn[15][2] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n3: f64 = ((s.dn[15][3] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n4: f64 = ((s.dn[15][4] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n5: f64 = ((s.dn[15][5] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n6: f64 = ((s.dn[15][6] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n7: f64 = ((s.dn[15][7] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n8: f64 = ((s.dn[15][8] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n9: f64 = ((s.dn[15][9] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n10: f64 = ((s.dn[15][10] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n11: f64 = ((s.dn[15][11] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n12: f64 = ((s.dn[15][12] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b0: f64 = ((s.db[15][0] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b1: f64 = ((s.db[15][1] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b2: f64 = ((s.db[15][2] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b3: f64 = ((s.db[15][3] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b4: f64 = ((s.db[15][4] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b5: f64 = ((s.db[15][5] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b6: f64 = ((s.db[15][6] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1393_d_n0: f64 = (eq53_e1391_d_n0 * 0.5);
        let eq53_e1393_d_n1: f64 = (eq53_e1391_d_n1 * 0.5);
        let eq53_e1393_d_n2: f64 = (eq53_e1391_d_n2 * 0.5);
        let eq53_e1393_d_n3: f64 = (eq53_e1391_d_n3 * 0.5);
        let eq53_e1393_d_n4: f64 = (eq53_e1391_d_n4 * 0.5);
        let eq53_e1393_d_n5: f64 = (eq53_e1391_d_n5 * 0.5);
        let eq53_e1393_d_n6: f64 = (eq53_e1391_d_n6 * 0.5);
        let eq53_e1393_d_n7: f64 = (eq53_e1391_d_n7 * 0.5);
        let eq53_e1393_d_n8: f64 = (eq53_e1391_d_n8 * 0.5);
        let eq53_e1393_d_n9: f64 = (eq53_e1391_d_n9 * 0.5);
        let eq53_e1393_d_n10: f64 = (eq53_e1391_d_n10 * 0.5);
        let eq53_e1393_d_n11: f64 = (eq53_e1391_d_n11 * 0.5);
        let eq53_e1393_d_n12: f64 = (eq53_e1391_d_n12 * 0.5);
        let eq53_e1393_d_b0: f64 = (eq53_e1391_d_b0 * 0.5);
        let eq53_e1393_d_b1: f64 = (eq53_e1391_d_b1 * 0.5);
        let eq53_e1393_d_b2: f64 = (eq53_e1391_d_b2 * 0.5);
        let eq53_e1393_d_b3: f64 = (eq53_e1391_d_b3 * 0.5);
        let eq53_e1393_d_b4: f64 = (eq53_e1391_d_b4 * 0.5);
        let eq53_e1393_d_b5: f64 = (eq53_e1391_d_b5 * 0.5);
        let eq53_e1393_d_b6: f64 = (eq53_e1391_d_b6 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * s.v[849]);
        let eq53_e1395_d_n0: f64 = ((eq53_e1393_d_n0 * s.v[849]) + (eq53_e1393 * s.dn[849][0]));
        let eq53_e1395_d_n1: f64 = ((eq53_e1393_d_n1 * s.v[849]) + (eq53_e1393 * s.dn[849][1]));
        let eq53_e1395_d_n2: f64 = ((eq53_e1393_d_n2 * s.v[849]) + (eq53_e1393 * s.dn[849][2]));
        let eq53_e1395_d_n3: f64 = ((eq53_e1393_d_n3 * s.v[849]) + (eq53_e1393 * s.dn[849][3]));
        let eq53_e1395_d_n4: f64 = ((eq53_e1393_d_n4 * s.v[849]) + (eq53_e1393 * s.dn[849][4]));
        let eq53_e1395_d_n5: f64 = ((eq53_e1393_d_n5 * s.v[849]) + (eq53_e1393 * s.dn[849][5]));
        let eq53_e1395_d_n6: f64 = ((eq53_e1393_d_n6 * s.v[849]) + (eq53_e1393 * s.dn[849][6]));
        let eq53_e1395_d_n7: f64 = ((eq53_e1393_d_n7 * s.v[849]) + (eq53_e1393 * s.dn[849][7]));
        let eq53_e1395_d_n8: f64 = ((eq53_e1393_d_n8 * s.v[849]) + (eq53_e1393 * s.dn[849][8]));
        let eq53_e1395_d_n9: f64 = ((eq53_e1393_d_n9 * s.v[849]) + (eq53_e1393 * s.dn[849][9]));
        let eq53_e1395_d_n10: f64 = ((eq53_e1393_d_n10 * s.v[849]) + (eq53_e1393 * s.dn[849][10]));
        let eq53_e1395_d_n11: f64 = ((eq53_e1393_d_n11 * s.v[849]) + (eq53_e1393 * s.dn[849][11]));
        let eq53_e1395_d_n12: f64 = ((eq53_e1393_d_n12 * s.v[849]) + (eq53_e1393 * s.dn[849][12]));
        let eq53_e1395_d_b0: f64 = ((eq53_e1393_d_b0 * s.v[849]) + (eq53_e1393 * s.db[849][0]));
        let eq53_e1395_d_b1: f64 = ((eq53_e1393_d_b1 * s.v[849]) + (eq53_e1393 * s.db[849][1]));
        let eq53_e1395_d_b2: f64 = ((eq53_e1393_d_b2 * s.v[849]) + (eq53_e1393 * s.db[849][2]));
        let eq53_e1395_d_b3: f64 = ((eq53_e1393_d_b3 * s.v[849]) + (eq53_e1393 * s.db[849][3]));
        let eq53_e1395_d_b4: f64 = ((eq53_e1393_d_b4 * s.v[849]) + (eq53_e1393 * s.db[849][4]));
        let eq53_e1395_d_b5: f64 = ((eq53_e1393_d_b5 * s.v[849]) + (eq53_e1393 * s.db[849][5]));
        let eq53_e1395_d_b6: f64 = ((eq53_e1393_d_b6 * s.v[849]) + (eq53_e1393 * s.db[849][6]));
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n0: f64 = (eq53_e1395_d_n0 * (nv5 - 0.0));
        let eq53_e1397_d_n1: f64 = (eq53_e1395_d_n1 * (nv5 - 0.0));
        let eq53_e1397_d_n2: f64 = (eq53_e1395_d_n2 * (nv5 - 0.0));
        let eq53_e1397_d_n3: f64 = (eq53_e1395_d_n3 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n5: f64 = ((eq53_e1395_d_n5 * (nv5 - 0.0)) + eq53_e1395);
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1397_d_n10: f64 = (eq53_e1395_d_n10 * (nv5 - 0.0));
        let eq53_e1397_d_n11: f64 = (eq53_e1395_d_n11 * (nv5 - 0.0));
        let eq53_e1397_d_n12: f64 = (eq53_e1395_d_n12 * (nv5 - 0.0));
        let eq53_e1397_d_b0: f64 = (eq53_e1395_d_b0 * (nv5 - 0.0));
        let eq53_e1397_d_b1: f64 = (eq53_e1395_d_b1 * (nv5 - 0.0));
        let eq53_e1397_d_b2: f64 = (eq53_e1395_d_b2 * (nv5 - 0.0));
        let eq53_e1397_d_b3: f64 = (eq53_e1395_d_b3 * (nv5 - 0.0));
        let eq53_e1397_d_b4: f64 = (eq53_e1395_d_b4 * (nv5 - 0.0));
        let eq53_e1397_d_b5: f64 = (eq53_e1395_d_b5 * (nv5 - 0.0));
        let eq53_e1397_d_b6: f64 = (eq53_e1395_d_b6 * (nv5 - 0.0));
        let eq53_e1398_q: f64 = eq53_e1397;
        let eq53_e1399: f64 = (-eq53_e1397);
        let eq53_e1399_q: f64 = (-eq53_e1398_q);
        let eq53_reactive_node_derivatives: [f64; 13] = [(-eq53_e1397_d_n0), (-eq53_e1397_d_n1), (-eq53_e1397_d_n2), (-eq53_e1397_d_n3), (-eq53_e1397_d_n4), (-eq53_e1397_d_n5), (-eq53_e1397_d_n6), (-eq53_e1397_d_n7), (-eq53_e1397_d_n8), (-eq53_e1397_d_n9), (-eq53_e1397_d_n10), (-eq53_e1397_d_n11), (-eq53_e1397_d_n12)];
        let eq53_reactive_branch_derivatives: [f64; 7] = [(-eq53_e1397_d_b0), (-eq53_e1397_d_b1), (-eq53_e1397_d_b2), (-eq53_e1397_d_b3), (-eq53_e1397_d_b4), (-eq53_e1397_d_b5), (-eq53_e1397_d_b6)];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
