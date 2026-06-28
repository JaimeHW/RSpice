#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq14_e1376, eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13, eq14_e1376_d_b0, eq14_e1376_d_b1, eq14_e1376_d_b2, eq14_e1376_d_b3, eq14_e1376_d_b4, eq14_e1376_d_b5, eq14_e1376_d_b6, eq14_e1376_d_b7, eq14_e1376_d_b8, eq14_e1376_d_b9, eq14_e1376_d_b10, eq14_e1376_d_b11, eq14_e1376_d_b12, eq14_e1376_d_b13, eq14_e1376_d_b14, eq14_e1376_d_b15, eq14_e1376_d_b16, eq14_e1376_d_b17, eq14_e1376_q,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq14_e1367: f64 = (p.p33 * 0.5);
        let eq14_e1369: f64 = (eq14_e1367 * s.v[1501]);
        let eq14_e1369_d_n0: f64 = (eq14_e1367 * s.dn[1501][0]);
        let eq14_e1369_d_n1: f64 = (eq14_e1367 * s.dn[1501][1]);
        let eq14_e1369_d_n2: f64 = (eq14_e1367 * s.dn[1501][2]);
        let eq14_e1369_d_n3: f64 = (eq14_e1367 * s.dn[1501][3]);
        let eq14_e1369_d_n4: f64 = (eq14_e1367 * s.dn[1501][4]);
        let eq14_e1369_d_n5: f64 = (eq14_e1367 * s.dn[1501][5]);
        let eq14_e1369_d_n6: f64 = (eq14_e1367 * s.dn[1501][6]);
        let eq14_e1369_d_n7: f64 = (eq14_e1367 * s.dn[1501][7]);
        let eq14_e1369_d_n8: f64 = (eq14_e1367 * s.dn[1501][8]);
        let eq14_e1369_d_n9: f64 = (eq14_e1367 * s.dn[1501][9]);
        let eq14_e1369_d_n10: f64 = (eq14_e1367 * s.dn[1501][10]);
        let eq14_e1369_d_n11: f64 = (eq14_e1367 * s.dn[1501][11]);
        let eq14_e1369_d_n12: f64 = (eq14_e1367 * s.dn[1501][12]);
        let eq14_e1369_d_n13: f64 = (eq14_e1367 * s.dn[1501][13]);
        let eq14_e1369_d_b0: f64 = (eq14_e1367 * s.db[1501][0]);
        let eq14_e1369_d_b1: f64 = (eq14_e1367 * s.db[1501][1]);
        let eq14_e1369_d_b2: f64 = (eq14_e1367 * s.db[1501][2]);
        let eq14_e1369_d_b3: f64 = (eq14_e1367 * s.db[1501][3]);
        let eq14_e1369_d_b4: f64 = (eq14_e1367 * s.db[1501][4]);
        let eq14_e1369_d_b5: f64 = (eq14_e1367 * s.db[1501][5]);
        let eq14_e1369_d_b6: f64 = (eq14_e1367 * s.db[1501][6]);
        let eq14_e1369_d_b7: f64 = (eq14_e1367 * s.db[1501][7]);
        let eq14_e1369_d_b8: f64 = (eq14_e1367 * s.db[1501][8]);
        let eq14_e1369_d_b9: f64 = (eq14_e1367 * s.db[1501][9]);
        let eq14_e1369_d_b10: f64 = (eq14_e1367 * s.db[1501][10]);
        let eq14_e1369_d_b11: f64 = (eq14_e1367 * s.db[1501][11]);
        let eq14_e1369_d_b12: f64 = (eq14_e1367 * s.db[1501][12]);
        let eq14_e1369_d_b13: f64 = (eq14_e1367 * s.db[1501][13]);
        let eq14_e1369_d_b14: f64 = (eq14_e1367 * s.db[1501][14]);
        let eq14_e1369_d_b15: f64 = (eq14_e1367 * s.db[1501][15]);
        let eq14_e1369_d_b16: f64 = (eq14_e1367 * s.db[1501][16]);
        let eq14_e1369_d_b17: f64 = (eq14_e1367 * s.db[1501][17]);
        let eq14_e1371: f64 = (eq14_e1369 * p.p226);
        let eq14_e1371_d_n0: f64 = (eq14_e1369_d_n0 * p.p226);
        let eq14_e1371_d_n1: f64 = (eq14_e1369_d_n1 * p.p226);
        let eq14_e1371_d_n2: f64 = (eq14_e1369_d_n2 * p.p226);
        let eq14_e1371_d_n3: f64 = (eq14_e1369_d_n3 * p.p226);
        let eq14_e1371_d_n4: f64 = (eq14_e1369_d_n4 * p.p226);
        let eq14_e1371_d_n5: f64 = (eq14_e1369_d_n5 * p.p226);
        let eq14_e1371_d_n6: f64 = (eq14_e1369_d_n6 * p.p226);
        let eq14_e1371_d_n7: f64 = (eq14_e1369_d_n7 * p.p226);
        let eq14_e1371_d_n8: f64 = (eq14_e1369_d_n8 * p.p226);
        let eq14_e1371_d_n9: f64 = (eq14_e1369_d_n9 * p.p226);
        let eq14_e1371_d_n10: f64 = (eq14_e1369_d_n10 * p.p226);
        let eq14_e1371_d_n11: f64 = (eq14_e1369_d_n11 * p.p226);
        let eq14_e1371_d_n12: f64 = (eq14_e1369_d_n12 * p.p226);
        let eq14_e1371_d_n13: f64 = (eq14_e1369_d_n13 * p.p226);
        let eq14_e1371_d_b0: f64 = (eq14_e1369_d_b0 * p.p226);
        let eq14_e1371_d_b1: f64 = (eq14_e1369_d_b1 * p.p226);
        let eq14_e1371_d_b2: f64 = (eq14_e1369_d_b2 * p.p226);
        let eq14_e1371_d_b3: f64 = (eq14_e1369_d_b3 * p.p226);
        let eq14_e1371_d_b4: f64 = (eq14_e1369_d_b4 * p.p226);
        let eq14_e1371_d_b5: f64 = (eq14_e1369_d_b5 * p.p226);
        let eq14_e1371_d_b6: f64 = (eq14_e1369_d_b6 * p.p226);
        let eq14_e1371_d_b7: f64 = (eq14_e1369_d_b7 * p.p226);
        let eq14_e1371_d_b8: f64 = (eq14_e1369_d_b8 * p.p226);
        let eq14_e1371_d_b9: f64 = (eq14_e1369_d_b9 * p.p226);
        let eq14_e1371_d_b10: f64 = (eq14_e1369_d_b10 * p.p226);
        let eq14_e1371_d_b11: f64 = (eq14_e1369_d_b11 * p.p226);
        let eq14_e1371_d_b12: f64 = (eq14_e1369_d_b12 * p.p226);
        let eq14_e1371_d_b13: f64 = (eq14_e1369_d_b13 * p.p226);
        let eq14_e1371_d_b14: f64 = (eq14_e1369_d_b14 * p.p226);
        let eq14_e1371_d_b15: f64 = (eq14_e1369_d_b15 * p.p226);
        let eq14_e1371_d_b16: f64 = (eq14_e1369_d_b16 * p.p226);
        let eq14_e1371_d_b17: f64 = (eq14_e1369_d_b17 * p.p226);
        let eq14_e1373: f64 = (eq14_e1371 * (nv13 - 0.0));
        let eq14_e1373_d_n0: f64 = (eq14_e1371_d_n0 * (nv13 - 0.0));
        let eq14_e1373_d_n1: f64 = (eq14_e1371_d_n1 * (nv13 - 0.0));
        let eq14_e1373_d_n2: f64 = (eq14_e1371_d_n2 * (nv13 - 0.0));
        let eq14_e1373_d_n3: f64 = (eq14_e1371_d_n3 * (nv13 - 0.0));
        let eq14_e1373_d_n4: f64 = (eq14_e1371_d_n4 * (nv13 - 0.0));
        let eq14_e1373_d_n5: f64 = (eq14_e1371_d_n5 * (nv13 - 0.0));
        let eq14_e1373_d_n6: f64 = (eq14_e1371_d_n6 * (nv13 - 0.0));
        let eq14_e1373_d_n7: f64 = (eq14_e1371_d_n7 * (nv13 - 0.0));
        let eq14_e1373_d_n8: f64 = (eq14_e1371_d_n8 * (nv13 - 0.0));
        let eq14_e1373_d_n9: f64 = (eq14_e1371_d_n9 * (nv13 - 0.0));
        let eq14_e1373_d_n10: f64 = (eq14_e1371_d_n10 * (nv13 - 0.0));
        let eq14_e1373_d_n11: f64 = (eq14_e1371_d_n11 * (nv13 - 0.0));
        let eq14_e1373_d_n12: f64 = (eq14_e1371_d_n12 * (nv13 - 0.0));
        let eq14_e1373_d_n13: f64 = ((eq14_e1371_d_n13 * (nv13 - 0.0)) + eq14_e1371);
        let eq14_e1373_d_b0: f64 = (eq14_e1371_d_b0 * (nv13 - 0.0));
        let eq14_e1373_d_b1: f64 = (eq14_e1371_d_b1 * (nv13 - 0.0));
        let eq14_e1373_d_b2: f64 = (eq14_e1371_d_b2 * (nv13 - 0.0));
        let eq14_e1373_d_b3: f64 = (eq14_e1371_d_b3 * (nv13 - 0.0));
        let eq14_e1373_d_b4: f64 = (eq14_e1371_d_b4 * (nv13 - 0.0));
        let eq14_e1373_d_b5: f64 = (eq14_e1371_d_b5 * (nv13 - 0.0));
        let eq14_e1373_d_b6: f64 = (eq14_e1371_d_b6 * (nv13 - 0.0));
        let eq14_e1373_d_b7: f64 = (eq14_e1371_d_b7 * (nv13 - 0.0));
        let eq14_e1373_d_b8: f64 = (eq14_e1371_d_b8 * (nv13 - 0.0));
        let eq14_e1373_d_b9: f64 = (eq14_e1371_d_b9 * (nv13 - 0.0));
        let eq14_e1373_d_b10: f64 = (eq14_e1371_d_b10 * (nv13 - 0.0));
        let eq14_e1373_d_b11: f64 = (eq14_e1371_d_b11 * (nv13 - 0.0));
        let eq14_e1373_d_b12: f64 = (eq14_e1371_d_b12 * (nv13 - 0.0));
        let eq14_e1373_d_b13: f64 = (eq14_e1371_d_b13 * (nv13 - 0.0));
        let eq14_e1373_d_b14: f64 = (eq14_e1371_d_b14 * (nv13 - 0.0));
        let eq14_e1373_d_b15: f64 = (eq14_e1371_d_b15 * (nv13 - 0.0));
        let eq14_e1373_d_b16: f64 = (eq14_e1371_d_b16 * (nv13 - 0.0));
        let eq14_e1373_d_b17: f64 = (eq14_e1371_d_b17 * (nv13 - 0.0));
        let eq14_e1374_q: f64 = eq14_e1373;
        (eq14_e1373, eq14_e1373_d_n0, eq14_e1373_d_n1, eq14_e1373_d_n2, eq14_e1373_d_n3, eq14_e1373_d_n4, eq14_e1373_d_n5, eq14_e1373_d_n6, eq14_e1373_d_n7, eq14_e1373_d_n8, eq14_e1373_d_n9, eq14_e1373_d_n10, eq14_e1373_d_n11, eq14_e1373_d_n12, eq14_e1373_d_n13, eq14_e1373_d_b0, eq14_e1373_d_b1, eq14_e1373_d_b2, eq14_e1373_d_b3, eq14_e1373_d_b4, eq14_e1373_d_b5, eq14_e1373_d_b6, eq14_e1373_d_b7, eq14_e1373_d_b8, eq14_e1373_d_b9, eq14_e1373_d_b10, eq14_e1373_d_b11, eq14_e1373_d_b12, eq14_e1373_d_b13, eq14_e1373_d_b14, eq14_e1373_d_b15, eq14_e1373_d_b16, eq14_e1373_d_b17, eq14_e1374_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_reactive_node_derivatives: [f64; 14] = [eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13];
        let eq14_reactive_branch_derivatives: [f64; 18] = [eq14_e1376_d_b0, eq14_e1376_d_b1, eq14_e1376_d_b2, eq14_e1376_d_b3, eq14_e1376_d_b4, eq14_e1376_d_b5, eq14_e1376_d_b6, eq14_e1376_d_b7, eq14_e1376_d_b8, eq14_e1376_d_b9, eq14_e1376_d_b10, eq14_e1376_d_b11, eq14_e1376_d_b12, eq14_e1376_d_b13, eq14_e1376_d_b14, eq14_e1376_d_b15, eq14_e1376_d_b16, eq14_e1376_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1396, eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13, eq15_e1396_d_b0, eq15_e1396_d_b1, eq15_e1396_d_b2, eq15_e1396_d_b3, eq15_e1396_d_b4, eq15_e1396_d_b5, eq15_e1396_d_b6, eq15_e1396_d_b7, eq15_e1396_d_b8, eq15_e1396_d_b9, eq15_e1396_d_b10, eq15_e1396_d_b11, eq15_e1396_d_b12, eq15_e1396_d_b13, eq15_e1396_d_b14, eq15_e1396_d_b15, eq15_e1396_d_b16, eq15_e1396_d_b17, eq15_e1396_q,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq15_e1387: f64 = (p.p33 * 0.5);
        let eq15_e1389: f64 = (eq15_e1387 * s.v[1501]);
        let eq15_e1389_d_n0: f64 = (eq15_e1387 * s.dn[1501][0]);
        let eq15_e1389_d_n1: f64 = (eq15_e1387 * s.dn[1501][1]);
        let eq15_e1389_d_n2: f64 = (eq15_e1387 * s.dn[1501][2]);
        let eq15_e1389_d_n3: f64 = (eq15_e1387 * s.dn[1501][3]);
        let eq15_e1389_d_n4: f64 = (eq15_e1387 * s.dn[1501][4]);
        let eq15_e1389_d_n5: f64 = (eq15_e1387 * s.dn[1501][5]);
        let eq15_e1389_d_n6: f64 = (eq15_e1387 * s.dn[1501][6]);
        let eq15_e1389_d_n7: f64 = (eq15_e1387 * s.dn[1501][7]);
        let eq15_e1389_d_n8: f64 = (eq15_e1387 * s.dn[1501][8]);
        let eq15_e1389_d_n9: f64 = (eq15_e1387 * s.dn[1501][9]);
        let eq15_e1389_d_n10: f64 = (eq15_e1387 * s.dn[1501][10]);
        let eq15_e1389_d_n11: f64 = (eq15_e1387 * s.dn[1501][11]);
        let eq15_e1389_d_n12: f64 = (eq15_e1387 * s.dn[1501][12]);
        let eq15_e1389_d_n13: f64 = (eq15_e1387 * s.dn[1501][13]);
        let eq15_e1389_d_b0: f64 = (eq15_e1387 * s.db[1501][0]);
        let eq15_e1389_d_b1: f64 = (eq15_e1387 * s.db[1501][1]);
        let eq15_e1389_d_b2: f64 = (eq15_e1387 * s.db[1501][2]);
        let eq15_e1389_d_b3: f64 = (eq15_e1387 * s.db[1501][3]);
        let eq15_e1389_d_b4: f64 = (eq15_e1387 * s.db[1501][4]);
        let eq15_e1389_d_b5: f64 = (eq15_e1387 * s.db[1501][5]);
        let eq15_e1389_d_b6: f64 = (eq15_e1387 * s.db[1501][6]);
        let eq15_e1389_d_b7: f64 = (eq15_e1387 * s.db[1501][7]);
        let eq15_e1389_d_b8: f64 = (eq15_e1387 * s.db[1501][8]);
        let eq15_e1389_d_b9: f64 = (eq15_e1387 * s.db[1501][9]);
        let eq15_e1389_d_b10: f64 = (eq15_e1387 * s.db[1501][10]);
        let eq15_e1389_d_b11: f64 = (eq15_e1387 * s.db[1501][11]);
        let eq15_e1389_d_b12: f64 = (eq15_e1387 * s.db[1501][12]);
        let eq15_e1389_d_b13: f64 = (eq15_e1387 * s.db[1501][13]);
        let eq15_e1389_d_b14: f64 = (eq15_e1387 * s.db[1501][14]);
        let eq15_e1389_d_b15: f64 = (eq15_e1387 * s.db[1501][15]);
        let eq15_e1389_d_b16: f64 = (eq15_e1387 * s.db[1501][16]);
        let eq15_e1389_d_b17: f64 = (eq15_e1387 * s.db[1501][17]);
        let eq15_e1391: f64 = (eq15_e1389 * p.p226);
        let eq15_e1391_d_n0: f64 = (eq15_e1389_d_n0 * p.p226);
        let eq15_e1391_d_n1: f64 = (eq15_e1389_d_n1 * p.p226);
        let eq15_e1391_d_n2: f64 = (eq15_e1389_d_n2 * p.p226);
        let eq15_e1391_d_n3: f64 = (eq15_e1389_d_n3 * p.p226);
        let eq15_e1391_d_n4: f64 = (eq15_e1389_d_n4 * p.p226);
        let eq15_e1391_d_n5: f64 = (eq15_e1389_d_n5 * p.p226);
        let eq15_e1391_d_n6: f64 = (eq15_e1389_d_n6 * p.p226);
        let eq15_e1391_d_n7: f64 = (eq15_e1389_d_n7 * p.p226);
        let eq15_e1391_d_n8: f64 = (eq15_e1389_d_n8 * p.p226);
        let eq15_e1391_d_n9: f64 = (eq15_e1389_d_n9 * p.p226);
        let eq15_e1391_d_n10: f64 = (eq15_e1389_d_n10 * p.p226);
        let eq15_e1391_d_n11: f64 = (eq15_e1389_d_n11 * p.p226);
        let eq15_e1391_d_n12: f64 = (eq15_e1389_d_n12 * p.p226);
        let eq15_e1391_d_n13: f64 = (eq15_e1389_d_n13 * p.p226);
        let eq15_e1391_d_b0: f64 = (eq15_e1389_d_b0 * p.p226);
        let eq15_e1391_d_b1: f64 = (eq15_e1389_d_b1 * p.p226);
        let eq15_e1391_d_b2: f64 = (eq15_e1389_d_b2 * p.p226);
        let eq15_e1391_d_b3: f64 = (eq15_e1389_d_b3 * p.p226);
        let eq15_e1391_d_b4: f64 = (eq15_e1389_d_b4 * p.p226);
        let eq15_e1391_d_b5: f64 = (eq15_e1389_d_b5 * p.p226);
        let eq15_e1391_d_b6: f64 = (eq15_e1389_d_b6 * p.p226);
        let eq15_e1391_d_b7: f64 = (eq15_e1389_d_b7 * p.p226);
        let eq15_e1391_d_b8: f64 = (eq15_e1389_d_b8 * p.p226);
        let eq15_e1391_d_b9: f64 = (eq15_e1389_d_b9 * p.p226);
        let eq15_e1391_d_b10: f64 = (eq15_e1389_d_b10 * p.p226);
        let eq15_e1391_d_b11: f64 = (eq15_e1389_d_b11 * p.p226);
        let eq15_e1391_d_b12: f64 = (eq15_e1389_d_b12 * p.p226);
        let eq15_e1391_d_b13: f64 = (eq15_e1389_d_b13 * p.p226);
        let eq15_e1391_d_b14: f64 = (eq15_e1389_d_b14 * p.p226);
        let eq15_e1391_d_b15: f64 = (eq15_e1389_d_b15 * p.p226);
        let eq15_e1391_d_b16: f64 = (eq15_e1389_d_b16 * p.p226);
        let eq15_e1391_d_b17: f64 = (eq15_e1389_d_b17 * p.p226);
        let eq15_e1393: f64 = (eq15_e1391 * (nv13 - 0.0));
        let eq15_e1393_d_n0: f64 = (eq15_e1391_d_n0 * (nv13 - 0.0));
        let eq15_e1393_d_n1: f64 = (eq15_e1391_d_n1 * (nv13 - 0.0));
        let eq15_e1393_d_n2: f64 = (eq15_e1391_d_n2 * (nv13 - 0.0));
        let eq15_e1393_d_n3: f64 = (eq15_e1391_d_n3 * (nv13 - 0.0));
        let eq15_e1393_d_n4: f64 = (eq15_e1391_d_n4 * (nv13 - 0.0));
        let eq15_e1393_d_n5: f64 = (eq15_e1391_d_n5 * (nv13 - 0.0));
        let eq15_e1393_d_n6: f64 = (eq15_e1391_d_n6 * (nv13 - 0.0));
        let eq15_e1393_d_n7: f64 = (eq15_e1391_d_n7 * (nv13 - 0.0));
        let eq15_e1393_d_n8: f64 = (eq15_e1391_d_n8 * (nv13 - 0.0));
        let eq15_e1393_d_n9: f64 = (eq15_e1391_d_n9 * (nv13 - 0.0));
        let eq15_e1393_d_n10: f64 = (eq15_e1391_d_n10 * (nv13 - 0.0));
        let eq15_e1393_d_n11: f64 = (eq15_e1391_d_n11 * (nv13 - 0.0));
        let eq15_e1393_d_n12: f64 = (eq15_e1391_d_n12 * (nv13 - 0.0));
        let eq15_e1393_d_n13: f64 = ((eq15_e1391_d_n13 * (nv13 - 0.0)) + eq15_e1391);
        let eq15_e1393_d_b0: f64 = (eq15_e1391_d_b0 * (nv13 - 0.0));
        let eq15_e1393_d_b1: f64 = (eq15_e1391_d_b1 * (nv13 - 0.0));
        let eq15_e1393_d_b2: f64 = (eq15_e1391_d_b2 * (nv13 - 0.0));
        let eq15_e1393_d_b3: f64 = (eq15_e1391_d_b3 * (nv13 - 0.0));
        let eq15_e1393_d_b4: f64 = (eq15_e1391_d_b4 * (nv13 - 0.0));
        let eq15_e1393_d_b5: f64 = (eq15_e1391_d_b5 * (nv13 - 0.0));
        let eq15_e1393_d_b6: f64 = (eq15_e1391_d_b6 * (nv13 - 0.0));
        let eq15_e1393_d_b7: f64 = (eq15_e1391_d_b7 * (nv13 - 0.0));
        let eq15_e1393_d_b8: f64 = (eq15_e1391_d_b8 * (nv13 - 0.0));
        let eq15_e1393_d_b9: f64 = (eq15_e1391_d_b9 * (nv13 - 0.0));
        let eq15_e1393_d_b10: f64 = (eq15_e1391_d_b10 * (nv13 - 0.0));
        let eq15_e1393_d_b11: f64 = (eq15_e1391_d_b11 * (nv13 - 0.0));
        let eq15_e1393_d_b12: f64 = (eq15_e1391_d_b12 * (nv13 - 0.0));
        let eq15_e1393_d_b13: f64 = (eq15_e1391_d_b13 * (nv13 - 0.0));
        let eq15_e1393_d_b14: f64 = (eq15_e1391_d_b14 * (nv13 - 0.0));
        let eq15_e1393_d_b15: f64 = (eq15_e1391_d_b15 * (nv13 - 0.0));
        let eq15_e1393_d_b16: f64 = (eq15_e1391_d_b16 * (nv13 - 0.0));
        let eq15_e1393_d_b17: f64 = (eq15_e1391_d_b17 * (nv13 - 0.0));
        let eq15_e1394_q: f64 = eq15_e1393;
        (eq15_e1393, eq15_e1393_d_n0, eq15_e1393_d_n1, eq15_e1393_d_n2, eq15_e1393_d_n3, eq15_e1393_d_n4, eq15_e1393_d_n5, eq15_e1393_d_n6, eq15_e1393_d_n7, eq15_e1393_d_n8, eq15_e1393_d_n9, eq15_e1393_d_n10, eq15_e1393_d_n11, eq15_e1393_d_n12, eq15_e1393_d_n13, eq15_e1393_d_b0, eq15_e1393_d_b1, eq15_e1393_d_b2, eq15_e1393_d_b3, eq15_e1393_d_b4, eq15_e1393_d_b5, eq15_e1393_d_b6, eq15_e1393_d_b7, eq15_e1393_d_b8, eq15_e1393_d_b9, eq15_e1393_d_b10, eq15_e1393_d_b11, eq15_e1393_d_b12, eq15_e1393_d_b13, eq15_e1393_d_b14, eq15_e1393_d_b15, eq15_e1393_d_b16, eq15_e1393_d_b17, eq15_e1394_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_reactive_node_derivatives: [f64; 14] = [eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13];
        let eq15_reactive_branch_derivatives: [f64; 18] = [eq15_e1396_d_b0, eq15_e1396_d_b1, eq15_e1396_d_b2, eq15_e1396_d_b3, eq15_e1396_d_b4, eq15_e1396_d_b5, eq15_e1396_d_b6, eq15_e1396_d_b7, eq15_e1396_d_b8, eq15_e1396_d_b9, eq15_e1396_d_b10, eq15_e1396_d_b11, eq15_e1396_d_b12, eq15_e1396_d_b13, eq15_e1396_d_b14, eq15_e1396_d_b15, eq15_e1396_d_b16, eq15_e1396_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq44_e1647: f64 = (p.p33 * s.v[92]);
        let eq44_e1647_d_n0: f64 = (p.p33 * s.dn[92][0]);
        let eq44_e1647_d_n1: f64 = (p.p33 * s.dn[92][1]);
        let eq44_e1647_d_n2: f64 = (p.p33 * s.dn[92][2]);
        let eq44_e1647_d_n3: f64 = (p.p33 * s.dn[92][3]);
        let eq44_e1647_d_n4: f64 = (p.p33 * s.dn[92][4]);
        let eq44_e1647_d_n5: f64 = (p.p33 * s.dn[92][5]);
        let eq44_e1647_d_n6: f64 = (p.p33 * s.dn[92][6]);
        let eq44_e1647_d_n7: f64 = (p.p33 * s.dn[92][7]);
        let eq44_e1647_d_n8: f64 = (p.p33 * s.dn[92][8]);
        let eq44_e1647_d_n9: f64 = (p.p33 * s.dn[92][9]);
        let eq44_e1647_d_n10: f64 = (p.p33 * s.dn[92][10]);
        let eq44_e1647_d_n11: f64 = (p.p33 * s.dn[92][11]);
        let eq44_e1647_d_n12: f64 = (p.p33 * s.dn[92][12]);
        let eq44_e1647_d_n13: f64 = (p.p33 * s.dn[92][13]);
        let eq44_e1647_d_b0: f64 = (p.p33 * s.db[92][0]);
        let eq44_e1647_d_b1: f64 = (p.p33 * s.db[92][1]);
        let eq44_e1647_d_b2: f64 = (p.p33 * s.db[92][2]);
        let eq44_e1647_d_b3: f64 = (p.p33 * s.db[92][3]);
        let eq44_e1647_d_b4: f64 = (p.p33 * s.db[92][4]);
        let eq44_e1647_d_b5: f64 = (p.p33 * s.db[92][5]);
        let eq44_e1647_d_b6: f64 = (p.p33 * s.db[92][6]);
        let eq44_e1647_d_b7: f64 = (p.p33 * s.db[92][7]);
        let eq44_e1647_d_b8: f64 = (p.p33 * s.db[92][8]);
        let eq44_e1647_d_b9: f64 = (p.p33 * s.db[92][9]);
        let eq44_e1647_d_b10: f64 = (p.p33 * s.db[92][10]);
        let eq44_e1647_d_b11: f64 = (p.p33 * s.db[92][11]);
        let eq44_e1647_d_b12: f64 = (p.p33 * s.db[92][12]);
        let eq44_e1647_d_b13: f64 = (p.p33 * s.db[92][13]);
        let eq44_e1647_d_b14: f64 = (p.p33 * s.db[92][14]);
        let eq44_e1647_d_b15: f64 = (p.p33 * s.db[92][15]);
        let eq44_e1647_d_b16: f64 = (p.p33 * s.db[92][16]);
        let eq44_e1647_d_b17: f64 = (p.p33 * s.db[92][17]);
        let eq44_e1648_q: f64 = eq44_e1647;
        let eq44_reactive_node_derivatives: [f64; 14] = [eq44_e1647_d_n0, eq44_e1647_d_n1, eq44_e1647_d_n2, eq44_e1647_d_n3, eq44_e1647_d_n4, eq44_e1647_d_n5, eq44_e1647_d_n6, eq44_e1647_d_n7, eq44_e1647_d_n8, eq44_e1647_d_n9, eq44_e1647_d_n10, eq44_e1647_d_n11, eq44_e1647_d_n12, eq44_e1647_d_n13];
        let eq44_reactive_branch_derivatives: [f64; 18] = [eq44_e1647_d_b0, eq44_e1647_d_b1, eq44_e1647_d_b2, eq44_e1647_d_b3, eq44_e1647_d_b4, eq44_e1647_d_b5, eq44_e1647_d_b6, eq44_e1647_d_b7, eq44_e1647_d_b8, eq44_e1647_d_b9, eq44_e1647_d_b10, eq44_e1647_d_b11, eq44_e1647_d_b12, eq44_e1647_d_b13, eq44_e1647_d_b14, eq44_e1647_d_b15, eq44_e1647_d_b16, eq44_e1647_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq44_reactive_node_derivatives,
            branches,
            &eq44_reactive_branch_derivatives,
            multiplicity,
        );
        let eq45_e1651: f64 = (p.p33 * s.v[93]);
        let eq45_e1651_d_n0: f64 = (p.p33 * s.dn[93][0]);
        let eq45_e1651_d_n1: f64 = (p.p33 * s.dn[93][1]);
        let eq45_e1651_d_n2: f64 = (p.p33 * s.dn[93][2]);
        let eq45_e1651_d_n3: f64 = (p.p33 * s.dn[93][3]);
        let eq45_e1651_d_n4: f64 = (p.p33 * s.dn[93][4]);
        let eq45_e1651_d_n5: f64 = (p.p33 * s.dn[93][5]);
        let eq45_e1651_d_n6: f64 = (p.p33 * s.dn[93][6]);
        let eq45_e1651_d_n7: f64 = (p.p33 * s.dn[93][7]);
        let eq45_e1651_d_n8: f64 = (p.p33 * s.dn[93][8]);
        let eq45_e1651_d_n9: f64 = (p.p33 * s.dn[93][9]);
        let eq45_e1651_d_n10: f64 = (p.p33 * s.dn[93][10]);
        let eq45_e1651_d_n11: f64 = (p.p33 * s.dn[93][11]);
        let eq45_e1651_d_n12: f64 = (p.p33 * s.dn[93][12]);
        let eq45_e1651_d_n13: f64 = (p.p33 * s.dn[93][13]);
        let eq45_e1651_d_b0: f64 = (p.p33 * s.db[93][0]);
        let eq45_e1651_d_b1: f64 = (p.p33 * s.db[93][1]);
        let eq45_e1651_d_b2: f64 = (p.p33 * s.db[93][2]);
        let eq45_e1651_d_b3: f64 = (p.p33 * s.db[93][3]);
        let eq45_e1651_d_b4: f64 = (p.p33 * s.db[93][4]);
        let eq45_e1651_d_b5: f64 = (p.p33 * s.db[93][5]);
        let eq45_e1651_d_b6: f64 = (p.p33 * s.db[93][6]);
        let eq45_e1651_d_b7: f64 = (p.p33 * s.db[93][7]);
        let eq45_e1651_d_b8: f64 = (p.p33 * s.db[93][8]);
        let eq45_e1651_d_b9: f64 = (p.p33 * s.db[93][9]);
        let eq45_e1651_d_b10: f64 = (p.p33 * s.db[93][10]);
        let eq45_e1651_d_b11: f64 = (p.p33 * s.db[93][11]);
        let eq45_e1651_d_b12: f64 = (p.p33 * s.db[93][12]);
        let eq45_e1651_d_b13: f64 = (p.p33 * s.db[93][13]);
        let eq45_e1651_d_b14: f64 = (p.p33 * s.db[93][14]);
        let eq45_e1651_d_b15: f64 = (p.p33 * s.db[93][15]);
        let eq45_e1651_d_b16: f64 = (p.p33 * s.db[93][16]);
        let eq45_e1651_d_b17: f64 = (p.p33 * s.db[93][17]);
        let eq45_e1652_q: f64 = eq45_e1651;
        let eq45_reactive_node_derivatives: [f64; 14] = [eq45_e1651_d_n0, eq45_e1651_d_n1, eq45_e1651_d_n2, eq45_e1651_d_n3, eq45_e1651_d_n4, eq45_e1651_d_n5, eq45_e1651_d_n6, eq45_e1651_d_n7, eq45_e1651_d_n8, eq45_e1651_d_n9, eq45_e1651_d_n10, eq45_e1651_d_n11, eq45_e1651_d_n12, eq45_e1651_d_n13];
        let eq45_reactive_branch_derivatives: [f64; 18] = [eq45_e1651_d_b0, eq45_e1651_d_b1, eq45_e1651_d_b2, eq45_e1651_d_b3, eq45_e1651_d_b4, eq45_e1651_d_b5, eq45_e1651_d_b6, eq45_e1651_d_b7, eq45_e1651_d_b8, eq45_e1651_d_b9, eq45_e1651_d_b10, eq45_e1651_d_b11, eq45_e1651_d_b12, eq45_e1651_d_b13, eq45_e1651_d_b14, eq45_e1651_d_b15, eq45_e1651_d_b16, eq45_e1651_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1656: f64 = (p.p33 * s.v[916]);
        let eq46_e1656_d_n0: f64 = (p.p33 * s.dn[916][0]);
        let eq46_e1656_d_n1: f64 = (p.p33 * s.dn[916][1]);
        let eq46_e1656_d_n2: f64 = (p.p33 * s.dn[916][2]);
        let eq46_e1656_d_n3: f64 = (p.p33 * s.dn[916][3]);
        let eq46_e1656_d_n4: f64 = (p.p33 * s.dn[916][4]);
        let eq46_e1656_d_n5: f64 = (p.p33 * s.dn[916][5]);
        let eq46_e1656_d_n6: f64 = (p.p33 * s.dn[916][6]);
        let eq46_e1656_d_n7: f64 = (p.p33 * s.dn[916][7]);
        let eq46_e1656_d_n8: f64 = (p.p33 * s.dn[916][8]);
        let eq46_e1656_d_n9: f64 = (p.p33 * s.dn[916][9]);
        let eq46_e1656_d_n10: f64 = (p.p33 * s.dn[916][10]);
        let eq46_e1656_d_n11: f64 = (p.p33 * s.dn[916][11]);
        let eq46_e1656_d_n12: f64 = (p.p33 * s.dn[916][12]);
        let eq46_e1656_d_n13: f64 = (p.p33 * s.dn[916][13]);
        let eq46_e1656_d_b0: f64 = (p.p33 * s.db[916][0]);
        let eq46_e1656_d_b1: f64 = (p.p33 * s.db[916][1]);
        let eq46_e1656_d_b2: f64 = (p.p33 * s.db[916][2]);
        let eq46_e1656_d_b3: f64 = (p.p33 * s.db[916][3]);
        let eq46_e1656_d_b4: f64 = (p.p33 * s.db[916][4]);
        let eq46_e1656_d_b5: f64 = (p.p33 * s.db[916][5]);
        let eq46_e1656_d_b6: f64 = (p.p33 * s.db[916][6]);
        let eq46_e1656_d_b7: f64 = (p.p33 * s.db[916][7]);
        let eq46_e1656_d_b8: f64 = (p.p33 * s.db[916][8]);
        let eq46_e1656_d_b9: f64 = (p.p33 * s.db[916][9]);
        let eq46_e1656_d_b10: f64 = (p.p33 * s.db[916][10]);
        let eq46_e1656_d_b11: f64 = (p.p33 * s.db[916][11]);
        let eq46_e1656_d_b12: f64 = (p.p33 * s.db[916][12]);
        let eq46_e1656_d_b13: f64 = (p.p33 * s.db[916][13]);
        let eq46_e1656_d_b14: f64 = (p.p33 * s.db[916][14]);
        let eq46_e1656_d_b15: f64 = (p.p33 * s.db[916][15]);
        let eq46_e1656_d_b16: f64 = (p.p33 * s.db[916][16]);
        let eq46_e1656_d_b17: f64 = (p.p33 * s.db[916][17]);
        let eq46_e1657_q: f64 = eq46_e1656;
        let eq46_e1658: f64 = (p.p37 * eq46_e1656);
        let eq46_e1658_d_n0: f64 = (p.p37 * eq46_e1656_d_n0);
        let eq46_e1658_d_n1: f64 = (p.p37 * eq46_e1656_d_n1);
        let eq46_e1658_d_n2: f64 = (p.p37 * eq46_e1656_d_n2);
        let eq46_e1658_d_n3: f64 = (p.p37 * eq46_e1656_d_n3);
        let eq46_e1658_d_n4: f64 = (p.p37 * eq46_e1656_d_n4);
        let eq46_e1658_d_n5: f64 = (p.p37 * eq46_e1656_d_n5);
        let eq46_e1658_d_n6: f64 = (p.p37 * eq46_e1656_d_n6);
        let eq46_e1658_d_n7: f64 = (p.p37 * eq46_e1656_d_n7);
        let eq46_e1658_d_n8: f64 = (p.p37 * eq46_e1656_d_n8);
        let eq46_e1658_d_n9: f64 = (p.p37 * eq46_e1656_d_n9);
        let eq46_e1658_d_n10: f64 = (p.p37 * eq46_e1656_d_n10);
        let eq46_e1658_d_n11: f64 = (p.p37 * eq46_e1656_d_n11);
        let eq46_e1658_d_n12: f64 = (p.p37 * eq46_e1656_d_n12);
        let eq46_e1658_d_n13: f64 = (p.p37 * eq46_e1656_d_n13);
        let eq46_e1658_d_b0: f64 = (p.p37 * eq46_e1656_d_b0);
        let eq46_e1658_d_b1: f64 = (p.p37 * eq46_e1656_d_b1);
        let eq46_e1658_d_b2: f64 = (p.p37 * eq46_e1656_d_b2);
        let eq46_e1658_d_b3: f64 = (p.p37 * eq46_e1656_d_b3);
        let eq46_e1658_d_b4: f64 = (p.p37 * eq46_e1656_d_b4);
        let eq46_e1658_d_b5: f64 = (p.p37 * eq46_e1656_d_b5);
        let eq46_e1658_d_b6: f64 = (p.p37 * eq46_e1656_d_b6);
        let eq46_e1658_d_b7: f64 = (p.p37 * eq46_e1656_d_b7);
        let eq46_e1658_d_b8: f64 = (p.p37 * eq46_e1656_d_b8);
        let eq46_e1658_d_b9: f64 = (p.p37 * eq46_e1656_d_b9);
        let eq46_e1658_d_b10: f64 = (p.p37 * eq46_e1656_d_b10);
        let eq46_e1658_d_b11: f64 = (p.p37 * eq46_e1656_d_b11);
        let eq46_e1658_d_b12: f64 = (p.p37 * eq46_e1656_d_b12);
        let eq46_e1658_d_b13: f64 = (p.p37 * eq46_e1656_d_b13);
        let eq46_e1658_d_b14: f64 = (p.p37 * eq46_e1656_d_b14);
        let eq46_e1658_d_b15: f64 = (p.p37 * eq46_e1656_d_b15);
        let eq46_e1658_d_b16: f64 = (p.p37 * eq46_e1656_d_b16);
        let eq46_e1658_d_b17: f64 = (p.p37 * eq46_e1656_d_b17);
        let eq46_e1658_q: f64 = (p.p37 * eq46_e1657_q);
        let eq46_reactive_node_derivatives: [f64; 14] = [eq46_e1658_d_n0, eq46_e1658_d_n1, eq46_e1658_d_n2, eq46_e1658_d_n3, eq46_e1658_d_n4, eq46_e1658_d_n5, eq46_e1658_d_n6, eq46_e1658_d_n7, eq46_e1658_d_n8, eq46_e1658_d_n9, eq46_e1658_d_n10, eq46_e1658_d_n11, eq46_e1658_d_n12, eq46_e1658_d_n13];
        let eq46_reactive_branch_derivatives: [f64; 18] = [eq46_e1658_d_b0, eq46_e1658_d_b1, eq46_e1658_d_b2, eq46_e1658_d_b3, eq46_e1658_d_b4, eq46_e1658_d_b5, eq46_e1658_d_b6, eq46_e1658_d_b7, eq46_e1658_d_b8, eq46_e1658_d_b9, eq46_e1658_d_b10, eq46_e1658_d_b11, eq46_e1658_d_b12, eq46_e1658_d_b13, eq46_e1658_d_b14, eq46_e1658_d_b15, eq46_e1658_d_b16, eq46_e1658_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let eq47_e1662: f64 = (p.p33 * s.v[920]);
        let eq47_e1662_d_n0: f64 = (p.p33 * s.dn[920][0]);
        let eq47_e1662_d_n1: f64 = (p.p33 * s.dn[920][1]);
        let eq47_e1662_d_n2: f64 = (p.p33 * s.dn[920][2]);
        let eq47_e1662_d_n3: f64 = (p.p33 * s.dn[920][3]);
        let eq47_e1662_d_n4: f64 = (p.p33 * s.dn[920][4]);
        let eq47_e1662_d_n5: f64 = (p.p33 * s.dn[920][5]);
        let eq47_e1662_d_n6: f64 = (p.p33 * s.dn[920][6]);
        let eq47_e1662_d_n7: f64 = (p.p33 * s.dn[920][7]);
        let eq47_e1662_d_n8: f64 = (p.p33 * s.dn[920][8]);
        let eq47_e1662_d_n9: f64 = (p.p33 * s.dn[920][9]);
        let eq47_e1662_d_n10: f64 = (p.p33 * s.dn[920][10]);
        let eq47_e1662_d_n11: f64 = (p.p33 * s.dn[920][11]);
        let eq47_e1662_d_n12: f64 = (p.p33 * s.dn[920][12]);
        let eq47_e1662_d_n13: f64 = (p.p33 * s.dn[920][13]);
        let eq47_e1662_d_b0: f64 = (p.p33 * s.db[920][0]);
        let eq47_e1662_d_b1: f64 = (p.p33 * s.db[920][1]);
        let eq47_e1662_d_b2: f64 = (p.p33 * s.db[920][2]);
        let eq47_e1662_d_b3: f64 = (p.p33 * s.db[920][3]);
        let eq47_e1662_d_b4: f64 = (p.p33 * s.db[920][4]);
        let eq47_e1662_d_b5: f64 = (p.p33 * s.db[920][5]);
        let eq47_e1662_d_b6: f64 = (p.p33 * s.db[920][6]);
        let eq47_e1662_d_b7: f64 = (p.p33 * s.db[920][7]);
        let eq47_e1662_d_b8: f64 = (p.p33 * s.db[920][8]);
        let eq47_e1662_d_b9: f64 = (p.p33 * s.db[920][9]);
        let eq47_e1662_d_b10: f64 = (p.p33 * s.db[920][10]);
        let eq47_e1662_d_b11: f64 = (p.p33 * s.db[920][11]);
        let eq47_e1662_d_b12: f64 = (p.p33 * s.db[920][12]);
        let eq47_e1662_d_b13: f64 = (p.p33 * s.db[920][13]);
        let eq47_e1662_d_b14: f64 = (p.p33 * s.db[920][14]);
        let eq47_e1662_d_b15: f64 = (p.p33 * s.db[920][15]);
        let eq47_e1662_d_b16: f64 = (p.p33 * s.db[920][16]);
        let eq47_e1662_d_b17: f64 = (p.p33 * s.db[920][17]);
        let eq47_e1663_q: f64 = eq47_e1662;
        let eq47_e1664: f64 = (p.p37 * eq47_e1662);
        let eq47_e1664_d_n0: f64 = (p.p37 * eq47_e1662_d_n0);
        let eq47_e1664_d_n1: f64 = (p.p37 * eq47_e1662_d_n1);
        let eq47_e1664_d_n2: f64 = (p.p37 * eq47_e1662_d_n2);
        let eq47_e1664_d_n3: f64 = (p.p37 * eq47_e1662_d_n3);
        let eq47_e1664_d_n4: f64 = (p.p37 * eq47_e1662_d_n4);
        let eq47_e1664_d_n5: f64 = (p.p37 * eq47_e1662_d_n5);
        let eq47_e1664_d_n6: f64 = (p.p37 * eq47_e1662_d_n6);
        let eq47_e1664_d_n7: f64 = (p.p37 * eq47_e1662_d_n7);
        let eq47_e1664_d_n8: f64 = (p.p37 * eq47_e1662_d_n8);
        let eq47_e1664_d_n9: f64 = (p.p37 * eq47_e1662_d_n9);
        let eq47_e1664_d_n10: f64 = (p.p37 * eq47_e1662_d_n10);
        let eq47_e1664_d_n11: f64 = (p.p37 * eq47_e1662_d_n11);
        let eq47_e1664_d_n12: f64 = (p.p37 * eq47_e1662_d_n12);
        let eq47_e1664_d_n13: f64 = (p.p37 * eq47_e1662_d_n13);
        let eq47_e1664_d_b0: f64 = (p.p37 * eq47_e1662_d_b0);
        let eq47_e1664_d_b1: f64 = (p.p37 * eq47_e1662_d_b1);
        let eq47_e1664_d_b2: f64 = (p.p37 * eq47_e1662_d_b2);
        let eq47_e1664_d_b3: f64 = (p.p37 * eq47_e1662_d_b3);
        let eq47_e1664_d_b4: f64 = (p.p37 * eq47_e1662_d_b4);
        let eq47_e1664_d_b5: f64 = (p.p37 * eq47_e1662_d_b5);
        let eq47_e1664_d_b6: f64 = (p.p37 * eq47_e1662_d_b6);
        let eq47_e1664_d_b7: f64 = (p.p37 * eq47_e1662_d_b7);
        let eq47_e1664_d_b8: f64 = (p.p37 * eq47_e1662_d_b8);
        let eq47_e1664_d_b9: f64 = (p.p37 * eq47_e1662_d_b9);
        let eq47_e1664_d_b10: f64 = (p.p37 * eq47_e1662_d_b10);
        let eq47_e1664_d_b11: f64 = (p.p37 * eq47_e1662_d_b11);
        let eq47_e1664_d_b12: f64 = (p.p37 * eq47_e1662_d_b12);
        let eq47_e1664_d_b13: f64 = (p.p37 * eq47_e1662_d_b13);
        let eq47_e1664_d_b14: f64 = (p.p37 * eq47_e1662_d_b14);
        let eq47_e1664_d_b15: f64 = (p.p37 * eq47_e1662_d_b15);
        let eq47_e1664_d_b16: f64 = (p.p37 * eq47_e1662_d_b16);
        let eq47_e1664_d_b17: f64 = (p.p37 * eq47_e1662_d_b17);
        let eq47_e1664_q: f64 = (p.p37 * eq47_e1663_q);
        let eq47_reactive_node_derivatives: [f64; 14] = [eq47_e1664_d_n0, eq47_e1664_d_n1, eq47_e1664_d_n2, eq47_e1664_d_n3, eq47_e1664_d_n4, eq47_e1664_d_n5, eq47_e1664_d_n6, eq47_e1664_d_n7, eq47_e1664_d_n8, eq47_e1664_d_n9, eq47_e1664_d_n10, eq47_e1664_d_n11, eq47_e1664_d_n12, eq47_e1664_d_n13];
        let eq47_reactive_branch_derivatives: [f64; 18] = [eq47_e1664_d_b0, eq47_e1664_d_b1, eq47_e1664_d_b2, eq47_e1664_d_b3, eq47_e1664_d_b4, eq47_e1664_d_b5, eq47_e1664_d_b6, eq47_e1664_d_b7, eq47_e1664_d_b8, eq47_e1664_d_b9, eq47_e1664_d_b10, eq47_e1664_d_b11, eq47_e1664_d_b12, eq47_e1664_d_b13, eq47_e1664_d_b14, eq47_e1664_d_b15, eq47_e1664_d_b16, eq47_e1664_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq48_e1668: f64 = (p.p33 * s.v[909]);
        let eq48_e1668_d_n0: f64 = (p.p33 * s.dn[909][0]);
        let eq48_e1668_d_n1: f64 = (p.p33 * s.dn[909][1]);
        let eq48_e1668_d_n2: f64 = (p.p33 * s.dn[909][2]);
        let eq48_e1668_d_n3: f64 = (p.p33 * s.dn[909][3]);
        let eq48_e1668_d_n4: f64 = (p.p33 * s.dn[909][4]);
        let eq48_e1668_d_n5: f64 = (p.p33 * s.dn[909][5]);
        let eq48_e1668_d_n6: f64 = (p.p33 * s.dn[909][6]);
        let eq48_e1668_d_n7: f64 = (p.p33 * s.dn[909][7]);
        let eq48_e1668_d_n8: f64 = (p.p33 * s.dn[909][8]);
        let eq48_e1668_d_n9: f64 = (p.p33 * s.dn[909][9]);
        let eq48_e1668_d_n10: f64 = (p.p33 * s.dn[909][10]);
        let eq48_e1668_d_n11: f64 = (p.p33 * s.dn[909][11]);
        let eq48_e1668_d_n12: f64 = (p.p33 * s.dn[909][12]);
        let eq48_e1668_d_n13: f64 = (p.p33 * s.dn[909][13]);
        let eq48_e1668_d_b0: f64 = (p.p33 * s.db[909][0]);
        let eq48_e1668_d_b1: f64 = (p.p33 * s.db[909][1]);
        let eq48_e1668_d_b2: f64 = (p.p33 * s.db[909][2]);
        let eq48_e1668_d_b3: f64 = (p.p33 * s.db[909][3]);
        let eq48_e1668_d_b4: f64 = (p.p33 * s.db[909][4]);
        let eq48_e1668_d_b5: f64 = (p.p33 * s.db[909][5]);
        let eq48_e1668_d_b6: f64 = (p.p33 * s.db[909][6]);
        let eq48_e1668_d_b7: f64 = (p.p33 * s.db[909][7]);
        let eq48_e1668_d_b8: f64 = (p.p33 * s.db[909][8]);
        let eq48_e1668_d_b9: f64 = (p.p33 * s.db[909][9]);
        let eq48_e1668_d_b10: f64 = (p.p33 * s.db[909][10]);
        let eq48_e1668_d_b11: f64 = (p.p33 * s.db[909][11]);
        let eq48_e1668_d_b12: f64 = (p.p33 * s.db[909][12]);
        let eq48_e1668_d_b13: f64 = (p.p33 * s.db[909][13]);
        let eq48_e1668_d_b14: f64 = (p.p33 * s.db[909][14]);
        let eq48_e1668_d_b15: f64 = (p.p33 * s.db[909][15]);
        let eq48_e1668_d_b16: f64 = (p.p33 * s.db[909][16]);
        let eq48_e1668_d_b17: f64 = (p.p33 * s.db[909][17]);
        let eq48_e1669_q: f64 = eq48_e1668;
        let eq48_e1670: f64 = (p.p37 * eq48_e1668);
        let eq48_e1670_d_n0: f64 = (p.p37 * eq48_e1668_d_n0);
        let eq48_e1670_d_n1: f64 = (p.p37 * eq48_e1668_d_n1);
        let eq48_e1670_d_n2: f64 = (p.p37 * eq48_e1668_d_n2);
        let eq48_e1670_d_n3: f64 = (p.p37 * eq48_e1668_d_n3);
        let eq48_e1670_d_n4: f64 = (p.p37 * eq48_e1668_d_n4);
        let eq48_e1670_d_n5: f64 = (p.p37 * eq48_e1668_d_n5);
        let eq48_e1670_d_n6: f64 = (p.p37 * eq48_e1668_d_n6);
        let eq48_e1670_d_n7: f64 = (p.p37 * eq48_e1668_d_n7);
        let eq48_e1670_d_n8: f64 = (p.p37 * eq48_e1668_d_n8);
        let eq48_e1670_d_n9: f64 = (p.p37 * eq48_e1668_d_n9);
        let eq48_e1670_d_n10: f64 = (p.p37 * eq48_e1668_d_n10);
        let eq48_e1670_d_n11: f64 = (p.p37 * eq48_e1668_d_n11);
        let eq48_e1670_d_n12: f64 = (p.p37 * eq48_e1668_d_n12);
        let eq48_e1670_d_n13: f64 = (p.p37 * eq48_e1668_d_n13);
        let eq48_e1670_d_b0: f64 = (p.p37 * eq48_e1668_d_b0);
        let eq48_e1670_d_b1: f64 = (p.p37 * eq48_e1668_d_b1);
        let eq48_e1670_d_b2: f64 = (p.p37 * eq48_e1668_d_b2);
        let eq48_e1670_d_b3: f64 = (p.p37 * eq48_e1668_d_b3);
        let eq48_e1670_d_b4: f64 = (p.p37 * eq48_e1668_d_b4);
        let eq48_e1670_d_b5: f64 = (p.p37 * eq48_e1668_d_b5);
        let eq48_e1670_d_b6: f64 = (p.p37 * eq48_e1668_d_b6);
        let eq48_e1670_d_b7: f64 = (p.p37 * eq48_e1668_d_b7);
        let eq48_e1670_d_b8: f64 = (p.p37 * eq48_e1668_d_b8);
        let eq48_e1670_d_b9: f64 = (p.p37 * eq48_e1668_d_b9);
        let eq48_e1670_d_b10: f64 = (p.p37 * eq48_e1668_d_b10);
        let eq48_e1670_d_b11: f64 = (p.p37 * eq48_e1668_d_b11);
        let eq48_e1670_d_b12: f64 = (p.p37 * eq48_e1668_d_b12);
        let eq48_e1670_d_b13: f64 = (p.p37 * eq48_e1668_d_b13);
        let eq48_e1670_d_b14: f64 = (p.p37 * eq48_e1668_d_b14);
        let eq48_e1670_d_b15: f64 = (p.p37 * eq48_e1668_d_b15);
        let eq48_e1670_d_b16: f64 = (p.p37 * eq48_e1668_d_b16);
        let eq48_e1670_d_b17: f64 = (p.p37 * eq48_e1668_d_b17);
        let eq48_e1670_q: f64 = (p.p37 * eq48_e1669_q);
        let eq48_reactive_node_derivatives: [f64; 14] = [eq48_e1670_d_n0, eq48_e1670_d_n1, eq48_e1670_d_n2, eq48_e1670_d_n3, eq48_e1670_d_n4, eq48_e1670_d_n5, eq48_e1670_d_n6, eq48_e1670_d_n7, eq48_e1670_d_n8, eq48_e1670_d_n9, eq48_e1670_d_n10, eq48_e1670_d_n11, eq48_e1670_d_n12, eq48_e1670_d_n13];
        let eq48_reactive_branch_derivatives: [f64; 18] = [eq48_e1670_d_b0, eq48_e1670_d_b1, eq48_e1670_d_b2, eq48_e1670_d_b3, eq48_e1670_d_b4, eq48_e1670_d_b5, eq48_e1670_d_b6, eq48_e1670_d_b7, eq48_e1670_d_b8, eq48_e1670_d_b9, eq48_e1670_d_b10, eq48_e1670_d_b11, eq48_e1670_d_b12, eq48_e1670_d_b13, eq48_e1670_d_b14, eq48_e1670_d_b15, eq48_e1670_d_b16, eq48_e1670_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let eq49_e1674: f64 = (p.p33 * s.v[910]);
        let eq49_e1674_d_n0: f64 = (p.p33 * s.dn[910][0]);
        let eq49_e1674_d_n1: f64 = (p.p33 * s.dn[910][1]);
        let eq49_e1674_d_n2: f64 = (p.p33 * s.dn[910][2]);
        let eq49_e1674_d_n3: f64 = (p.p33 * s.dn[910][3]);
        let eq49_e1674_d_n4: f64 = (p.p33 * s.dn[910][4]);
        let eq49_e1674_d_n5: f64 = (p.p33 * s.dn[910][5]);
        let eq49_e1674_d_n6: f64 = (p.p33 * s.dn[910][6]);
        let eq49_e1674_d_n7: f64 = (p.p33 * s.dn[910][7]);
        let eq49_e1674_d_n8: f64 = (p.p33 * s.dn[910][8]);
        let eq49_e1674_d_n9: f64 = (p.p33 * s.dn[910][9]);
        let eq49_e1674_d_n10: f64 = (p.p33 * s.dn[910][10]);
        let eq49_e1674_d_n11: f64 = (p.p33 * s.dn[910][11]);
        let eq49_e1674_d_n12: f64 = (p.p33 * s.dn[910][12]);
        let eq49_e1674_d_n13: f64 = (p.p33 * s.dn[910][13]);
        let eq49_e1674_d_b0: f64 = (p.p33 * s.db[910][0]);
        let eq49_e1674_d_b1: f64 = (p.p33 * s.db[910][1]);
        let eq49_e1674_d_b2: f64 = (p.p33 * s.db[910][2]);
        let eq49_e1674_d_b3: f64 = (p.p33 * s.db[910][3]);
        let eq49_e1674_d_b4: f64 = (p.p33 * s.db[910][4]);
        let eq49_e1674_d_b5: f64 = (p.p33 * s.db[910][5]);
        let eq49_e1674_d_b6: f64 = (p.p33 * s.db[910][6]);
        let eq49_e1674_d_b7: f64 = (p.p33 * s.db[910][7]);
        let eq49_e1674_d_b8: f64 = (p.p33 * s.db[910][8]);
        let eq49_e1674_d_b9: f64 = (p.p33 * s.db[910][9]);
        let eq49_e1674_d_b10: f64 = (p.p33 * s.db[910][10]);
        let eq49_e1674_d_b11: f64 = (p.p33 * s.db[910][11]);
        let eq49_e1674_d_b12: f64 = (p.p33 * s.db[910][12]);
        let eq49_e1674_d_b13: f64 = (p.p33 * s.db[910][13]);
        let eq49_e1674_d_b14: f64 = (p.p33 * s.db[910][14]);
        let eq49_e1674_d_b15: f64 = (p.p33 * s.db[910][15]);
        let eq49_e1674_d_b16: f64 = (p.p33 * s.db[910][16]);
        let eq49_e1674_d_b17: f64 = (p.p33 * s.db[910][17]);
        let eq49_e1675_q: f64 = eq49_e1674;
        let eq49_e1676: f64 = (p.p37 * eq49_e1674);
        let eq49_e1676_d_n0: f64 = (p.p37 * eq49_e1674_d_n0);
        let eq49_e1676_d_n1: f64 = (p.p37 * eq49_e1674_d_n1);
        let eq49_e1676_d_n2: f64 = (p.p37 * eq49_e1674_d_n2);
        let eq49_e1676_d_n3: f64 = (p.p37 * eq49_e1674_d_n3);
        let eq49_e1676_d_n4: f64 = (p.p37 * eq49_e1674_d_n4);
        let eq49_e1676_d_n5: f64 = (p.p37 * eq49_e1674_d_n5);
        let eq49_e1676_d_n6: f64 = (p.p37 * eq49_e1674_d_n6);
        let eq49_e1676_d_n7: f64 = (p.p37 * eq49_e1674_d_n7);
        let eq49_e1676_d_n8: f64 = (p.p37 * eq49_e1674_d_n8);
        let eq49_e1676_d_n9: f64 = (p.p37 * eq49_e1674_d_n9);
        let eq49_e1676_d_n10: f64 = (p.p37 * eq49_e1674_d_n10);
        let eq49_e1676_d_n11: f64 = (p.p37 * eq49_e1674_d_n11);
        let eq49_e1676_d_n12: f64 = (p.p37 * eq49_e1674_d_n12);
        let eq49_e1676_d_n13: f64 = (p.p37 * eq49_e1674_d_n13);
        let eq49_e1676_d_b0: f64 = (p.p37 * eq49_e1674_d_b0);
        let eq49_e1676_d_b1: f64 = (p.p37 * eq49_e1674_d_b1);
        let eq49_e1676_d_b2: f64 = (p.p37 * eq49_e1674_d_b2);
        let eq49_e1676_d_b3: f64 = (p.p37 * eq49_e1674_d_b3);
        let eq49_e1676_d_b4: f64 = (p.p37 * eq49_e1674_d_b4);
        let eq49_e1676_d_b5: f64 = (p.p37 * eq49_e1674_d_b5);
        let eq49_e1676_d_b6: f64 = (p.p37 * eq49_e1674_d_b6);
        let eq49_e1676_d_b7: f64 = (p.p37 * eq49_e1674_d_b7);
        let eq49_e1676_d_b8: f64 = (p.p37 * eq49_e1674_d_b8);
        let eq49_e1676_d_b9: f64 = (p.p37 * eq49_e1674_d_b9);
        let eq49_e1676_d_b10: f64 = (p.p37 * eq49_e1674_d_b10);
        let eq49_e1676_d_b11: f64 = (p.p37 * eq49_e1674_d_b11);
        let eq49_e1676_d_b12: f64 = (p.p37 * eq49_e1674_d_b12);
        let eq49_e1676_d_b13: f64 = (p.p37 * eq49_e1674_d_b13);
        let eq49_e1676_d_b14: f64 = (p.p37 * eq49_e1674_d_b14);
        let eq49_e1676_d_b15: f64 = (p.p37 * eq49_e1674_d_b15);
        let eq49_e1676_d_b16: f64 = (p.p37 * eq49_e1674_d_b16);
        let eq49_e1676_d_b17: f64 = (p.p37 * eq49_e1674_d_b17);
        let eq49_e1676_q: f64 = (p.p37 * eq49_e1675_q);
        let eq49_reactive_node_derivatives: [f64; 14] = [eq49_e1676_d_n0, eq49_e1676_d_n1, eq49_e1676_d_n2, eq49_e1676_d_n3, eq49_e1676_d_n4, eq49_e1676_d_n5, eq49_e1676_d_n6, eq49_e1676_d_n7, eq49_e1676_d_n8, eq49_e1676_d_n9, eq49_e1676_d_n10, eq49_e1676_d_n11, eq49_e1676_d_n12, eq49_e1676_d_n13];
        let eq49_reactive_branch_derivatives: [f64; 18] = [eq49_e1676_d_b0, eq49_e1676_d_b1, eq49_e1676_d_b2, eq49_e1676_d_b3, eq49_e1676_d_b4, eq49_e1676_d_b5, eq49_e1676_d_b6, eq49_e1676_d_b7, eq49_e1676_d_b8, eq49_e1676_d_b9, eq49_e1676_d_b10, eq49_e1676_d_b11, eq49_e1676_d_b12, eq49_e1676_d_b13, eq49_e1676_d_b14, eq49_e1676_d_b15, eq49_e1676_d_b16, eq49_e1676_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq49_reactive_node_derivatives,
            branches,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e1685, eq50_e1685_d_n0, eq50_e1685_d_n1, eq50_e1685_d_n2, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, eq50_e1685_d_n13, eq50_e1685_d_b0, eq50_e1685_d_b1, eq50_e1685_d_b2, eq50_e1685_d_b3, eq50_e1685_d_b4, eq50_e1685_d_b5, eq50_e1685_d_b6, eq50_e1685_d_b7, eq50_e1685_d_b8, eq50_e1685_d_b9, eq50_e1685_d_b10, eq50_e1685_d_b11, eq50_e1685_d_b12, eq50_e1685_d_b13, eq50_e1685_d_b14, eq50_e1685_d_b15, eq50_e1685_d_b16, eq50_e1685_d_b17, eq50_e1685_q,) = {
    if s.b[1553] {
        let eq50_e1681: f64 = (p.p33 * s.v[895]);
        let eq50_e1681_d_n0: f64 = (p.p33 * s.dn[895][0]);
        let eq50_e1681_d_n1: f64 = (p.p33 * s.dn[895][1]);
        let eq50_e1681_d_n2: f64 = (p.p33 * s.dn[895][2]);
        let eq50_e1681_d_n3: f64 = (p.p33 * s.dn[895][3]);
        let eq50_e1681_d_n4: f64 = (p.p33 * s.dn[895][4]);
        let eq50_e1681_d_n5: f64 = (p.p33 * s.dn[895][5]);
        let eq50_e1681_d_n6: f64 = (p.p33 * s.dn[895][6]);
        let eq50_e1681_d_n7: f64 = (p.p33 * s.dn[895][7]);
        let eq50_e1681_d_n8: f64 = (p.p33 * s.dn[895][8]);
        let eq50_e1681_d_n9: f64 = (p.p33 * s.dn[895][9]);
        let eq50_e1681_d_n10: f64 = (p.p33 * s.dn[895][10]);
        let eq50_e1681_d_n11: f64 = (p.p33 * s.dn[895][11]);
        let eq50_e1681_d_n12: f64 = (p.p33 * s.dn[895][12]);
        let eq50_e1681_d_n13: f64 = (p.p33 * s.dn[895][13]);
        let eq50_e1681_d_b0: f64 = (p.p33 * s.db[895][0]);
        let eq50_e1681_d_b1: f64 = (p.p33 * s.db[895][1]);
        let eq50_e1681_d_b2: f64 = (p.p33 * s.db[895][2]);
        let eq50_e1681_d_b3: f64 = (p.p33 * s.db[895][3]);
        let eq50_e1681_d_b4: f64 = (p.p33 * s.db[895][4]);
        let eq50_e1681_d_b5: f64 = (p.p33 * s.db[895][5]);
        let eq50_e1681_d_b6: f64 = (p.p33 * s.db[895][6]);
        let eq50_e1681_d_b7: f64 = (p.p33 * s.db[895][7]);
        let eq50_e1681_d_b8: f64 = (p.p33 * s.db[895][8]);
        let eq50_e1681_d_b9: f64 = (p.p33 * s.db[895][9]);
        let eq50_e1681_d_b10: f64 = (p.p33 * s.db[895][10]);
        let eq50_e1681_d_b11: f64 = (p.p33 * s.db[895][11]);
        let eq50_e1681_d_b12: f64 = (p.p33 * s.db[895][12]);
        let eq50_e1681_d_b13: f64 = (p.p33 * s.db[895][13]);
        let eq50_e1681_d_b14: f64 = (p.p33 * s.db[895][14]);
        let eq50_e1681_d_b15: f64 = (p.p33 * s.db[895][15]);
        let eq50_e1681_d_b16: f64 = (p.p33 * s.db[895][16]);
        let eq50_e1681_d_b17: f64 = (p.p33 * s.db[895][17]);
        let eq50_e1682_q: f64 = eq50_e1681;
        let eq50_e1683: f64 = (p.p37 * eq50_e1681);
        let eq50_e1683_d_n0: f64 = (p.p37 * eq50_e1681_d_n0);
        let eq50_e1683_d_n1: f64 = (p.p37 * eq50_e1681_d_n1);
        let eq50_e1683_d_n2: f64 = (p.p37 * eq50_e1681_d_n2);
        let eq50_e1683_d_n3: f64 = (p.p37 * eq50_e1681_d_n3);
        let eq50_e1683_d_n4: f64 = (p.p37 * eq50_e1681_d_n4);
        let eq50_e1683_d_n5: f64 = (p.p37 * eq50_e1681_d_n5);
        let eq50_e1683_d_n6: f64 = (p.p37 * eq50_e1681_d_n6);
        let eq50_e1683_d_n7: f64 = (p.p37 * eq50_e1681_d_n7);
        let eq50_e1683_d_n8: f64 = (p.p37 * eq50_e1681_d_n8);
        let eq50_e1683_d_n9: f64 = (p.p37 * eq50_e1681_d_n9);
        let eq50_e1683_d_n10: f64 = (p.p37 * eq50_e1681_d_n10);
        let eq50_e1683_d_n11: f64 = (p.p37 * eq50_e1681_d_n11);
        let eq50_e1683_d_n12: f64 = (p.p37 * eq50_e1681_d_n12);
        let eq50_e1683_d_n13: f64 = (p.p37 * eq50_e1681_d_n13);
        let eq50_e1683_d_b0: f64 = (p.p37 * eq50_e1681_d_b0);
        let eq50_e1683_d_b1: f64 = (p.p37 * eq50_e1681_d_b1);
        let eq50_e1683_d_b2: f64 = (p.p37 * eq50_e1681_d_b2);
        let eq50_e1683_d_b3: f64 = (p.p37 * eq50_e1681_d_b3);
        let eq50_e1683_d_b4: f64 = (p.p37 * eq50_e1681_d_b4);
        let eq50_e1683_d_b5: f64 = (p.p37 * eq50_e1681_d_b5);
        let eq50_e1683_d_b6: f64 = (p.p37 * eq50_e1681_d_b6);
        let eq50_e1683_d_b7: f64 = (p.p37 * eq50_e1681_d_b7);
        let eq50_e1683_d_b8: f64 = (p.p37 * eq50_e1681_d_b8);
        let eq50_e1683_d_b9: f64 = (p.p37 * eq50_e1681_d_b9);
        let eq50_e1683_d_b10: f64 = (p.p37 * eq50_e1681_d_b10);
        let eq50_e1683_d_b11: f64 = (p.p37 * eq50_e1681_d_b11);
        let eq50_e1683_d_b12: f64 = (p.p37 * eq50_e1681_d_b12);
        let eq50_e1683_d_b13: f64 = (p.p37 * eq50_e1681_d_b13);
        let eq50_e1683_d_b14: f64 = (p.p37 * eq50_e1681_d_b14);
        let eq50_e1683_d_b15: f64 = (p.p37 * eq50_e1681_d_b15);
        let eq50_e1683_d_b16: f64 = (p.p37 * eq50_e1681_d_b16);
        let eq50_e1683_d_b17: f64 = (p.p37 * eq50_e1681_d_b17);
        let eq50_e1683_q: f64 = (p.p37 * eq50_e1682_q);
        (eq50_e1683, eq50_e1683_d_n0, eq50_e1683_d_n1, eq50_e1683_d_n2, eq50_e1683_d_n3, eq50_e1683_d_n4, eq50_e1683_d_n5, eq50_e1683_d_n6, eq50_e1683_d_n7, eq50_e1683_d_n8, eq50_e1683_d_n9, eq50_e1683_d_n10, eq50_e1683_d_n11, eq50_e1683_d_n12, eq50_e1683_d_n13, eq50_e1683_d_b0, eq50_e1683_d_b1, eq50_e1683_d_b2, eq50_e1683_d_b3, eq50_e1683_d_b4, eq50_e1683_d_b5, eq50_e1683_d_b6, eq50_e1683_d_b7, eq50_e1683_d_b8, eq50_e1683_d_b9, eq50_e1683_d_b10, eq50_e1683_d_b11, eq50_e1683_d_b12, eq50_e1683_d_b13, eq50_e1683_d_b14, eq50_e1683_d_b15, eq50_e1683_d_b16, eq50_e1683_d_b17, eq50_e1683_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 14] = [eq50_e1685_d_n0, eq50_e1685_d_n1, eq50_e1685_d_n2, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, eq50_e1685_d_n13];
        let eq50_reactive_branch_derivatives: [f64; 18] = [eq50_e1685_d_b0, eq50_e1685_d_b1, eq50_e1685_d_b2, eq50_e1685_d_b3, eq50_e1685_d_b4, eq50_e1685_d_b5, eq50_e1685_d_b6, eq50_e1685_d_b7, eq50_e1685_d_b8, eq50_e1685_d_b9, eq50_e1685_d_b10, eq50_e1685_d_b11, eq50_e1685_d_b12, eq50_e1685_d_b13, eq50_e1685_d_b14, eq50_e1685_d_b15, eq50_e1685_d_b16, eq50_e1685_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e1694, eq51_e1694_d_n0, eq51_e1694_d_n1, eq51_e1694_d_n2, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, eq51_e1694_d_n13, eq51_e1694_d_b0, eq51_e1694_d_b1, eq51_e1694_d_b2, eq51_e1694_d_b3, eq51_e1694_d_b4, eq51_e1694_d_b5, eq51_e1694_d_b6, eq51_e1694_d_b7, eq51_e1694_d_b8, eq51_e1694_d_b9, eq51_e1694_d_b10, eq51_e1694_d_b11, eq51_e1694_d_b12, eq51_e1694_d_b13, eq51_e1694_d_b14, eq51_e1694_d_b15, eq51_e1694_d_b16, eq51_e1694_d_b17, eq51_e1694_q,) = {
    if s.b[1553] {
        let eq51_e1690: f64 = (p.p33 * s.v[896]);
        let eq51_e1690_d_n0: f64 = (p.p33 * s.dn[896][0]);
        let eq51_e1690_d_n1: f64 = (p.p33 * s.dn[896][1]);
        let eq51_e1690_d_n2: f64 = (p.p33 * s.dn[896][2]);
        let eq51_e1690_d_n3: f64 = (p.p33 * s.dn[896][3]);
        let eq51_e1690_d_n4: f64 = (p.p33 * s.dn[896][4]);
        let eq51_e1690_d_n5: f64 = (p.p33 * s.dn[896][5]);
        let eq51_e1690_d_n6: f64 = (p.p33 * s.dn[896][6]);
        let eq51_e1690_d_n7: f64 = (p.p33 * s.dn[896][7]);
        let eq51_e1690_d_n8: f64 = (p.p33 * s.dn[896][8]);
        let eq51_e1690_d_n9: f64 = (p.p33 * s.dn[896][9]);
        let eq51_e1690_d_n10: f64 = (p.p33 * s.dn[896][10]);
        let eq51_e1690_d_n11: f64 = (p.p33 * s.dn[896][11]);
        let eq51_e1690_d_n12: f64 = (p.p33 * s.dn[896][12]);
        let eq51_e1690_d_n13: f64 = (p.p33 * s.dn[896][13]);
        let eq51_e1690_d_b0: f64 = (p.p33 * s.db[896][0]);
        let eq51_e1690_d_b1: f64 = (p.p33 * s.db[896][1]);
        let eq51_e1690_d_b2: f64 = (p.p33 * s.db[896][2]);
        let eq51_e1690_d_b3: f64 = (p.p33 * s.db[896][3]);
        let eq51_e1690_d_b4: f64 = (p.p33 * s.db[896][4]);
        let eq51_e1690_d_b5: f64 = (p.p33 * s.db[896][5]);
        let eq51_e1690_d_b6: f64 = (p.p33 * s.db[896][6]);
        let eq51_e1690_d_b7: f64 = (p.p33 * s.db[896][7]);
        let eq51_e1690_d_b8: f64 = (p.p33 * s.db[896][8]);
        let eq51_e1690_d_b9: f64 = (p.p33 * s.db[896][9]);
        let eq51_e1690_d_b10: f64 = (p.p33 * s.db[896][10]);
        let eq51_e1690_d_b11: f64 = (p.p33 * s.db[896][11]);
        let eq51_e1690_d_b12: f64 = (p.p33 * s.db[896][12]);
        let eq51_e1690_d_b13: f64 = (p.p33 * s.db[896][13]);
        let eq51_e1690_d_b14: f64 = (p.p33 * s.db[896][14]);
        let eq51_e1690_d_b15: f64 = (p.p33 * s.db[896][15]);
        let eq51_e1690_d_b16: f64 = (p.p33 * s.db[896][16]);
        let eq51_e1690_d_b17: f64 = (p.p33 * s.db[896][17]);
        let eq51_e1691_q: f64 = eq51_e1690;
        let eq51_e1692: f64 = (p.p37 * eq51_e1690);
        let eq51_e1692_d_n0: f64 = (p.p37 * eq51_e1690_d_n0);
        let eq51_e1692_d_n1: f64 = (p.p37 * eq51_e1690_d_n1);
        let eq51_e1692_d_n2: f64 = (p.p37 * eq51_e1690_d_n2);
        let eq51_e1692_d_n3: f64 = (p.p37 * eq51_e1690_d_n3);
        let eq51_e1692_d_n4: f64 = (p.p37 * eq51_e1690_d_n4);
        let eq51_e1692_d_n5: f64 = (p.p37 * eq51_e1690_d_n5);
        let eq51_e1692_d_n6: f64 = (p.p37 * eq51_e1690_d_n6);
        let eq51_e1692_d_n7: f64 = (p.p37 * eq51_e1690_d_n7);
        let eq51_e1692_d_n8: f64 = (p.p37 * eq51_e1690_d_n8);
        let eq51_e1692_d_n9: f64 = (p.p37 * eq51_e1690_d_n9);
        let eq51_e1692_d_n10: f64 = (p.p37 * eq51_e1690_d_n10);
        let eq51_e1692_d_n11: f64 = (p.p37 * eq51_e1690_d_n11);
        let eq51_e1692_d_n12: f64 = (p.p37 * eq51_e1690_d_n12);
        let eq51_e1692_d_n13: f64 = (p.p37 * eq51_e1690_d_n13);
        let eq51_e1692_d_b0: f64 = (p.p37 * eq51_e1690_d_b0);
        let eq51_e1692_d_b1: f64 = (p.p37 * eq51_e1690_d_b1);
        let eq51_e1692_d_b2: f64 = (p.p37 * eq51_e1690_d_b2);
        let eq51_e1692_d_b3: f64 = (p.p37 * eq51_e1690_d_b3);
        let eq51_e1692_d_b4: f64 = (p.p37 * eq51_e1690_d_b4);
        let eq51_e1692_d_b5: f64 = (p.p37 * eq51_e1690_d_b5);
        let eq51_e1692_d_b6: f64 = (p.p37 * eq51_e1690_d_b6);
        let eq51_e1692_d_b7: f64 = (p.p37 * eq51_e1690_d_b7);
        let eq51_e1692_d_b8: f64 = (p.p37 * eq51_e1690_d_b8);
        let eq51_e1692_d_b9: f64 = (p.p37 * eq51_e1690_d_b9);
        let eq51_e1692_d_b10: f64 = (p.p37 * eq51_e1690_d_b10);
        let eq51_e1692_d_b11: f64 = (p.p37 * eq51_e1690_d_b11);
        let eq51_e1692_d_b12: f64 = (p.p37 * eq51_e1690_d_b12);
        let eq51_e1692_d_b13: f64 = (p.p37 * eq51_e1690_d_b13);
        let eq51_e1692_d_b14: f64 = (p.p37 * eq51_e1690_d_b14);
        let eq51_e1692_d_b15: f64 = (p.p37 * eq51_e1690_d_b15);
        let eq51_e1692_d_b16: f64 = (p.p37 * eq51_e1690_d_b16);
        let eq51_e1692_d_b17: f64 = (p.p37 * eq51_e1690_d_b17);
        let eq51_e1692_q: f64 = (p.p37 * eq51_e1691_q);
        (eq51_e1692, eq51_e1692_d_n0, eq51_e1692_d_n1, eq51_e1692_d_n2, eq51_e1692_d_n3, eq51_e1692_d_n4, eq51_e1692_d_n5, eq51_e1692_d_n6, eq51_e1692_d_n7, eq51_e1692_d_n8, eq51_e1692_d_n9, eq51_e1692_d_n10, eq51_e1692_d_n11, eq51_e1692_d_n12, eq51_e1692_d_n13, eq51_e1692_d_b0, eq51_e1692_d_b1, eq51_e1692_d_b2, eq51_e1692_d_b3, eq51_e1692_d_b4, eq51_e1692_d_b5, eq51_e1692_d_b6, eq51_e1692_d_b7, eq51_e1692_d_b8, eq51_e1692_d_b9, eq51_e1692_d_b10, eq51_e1692_d_b11, eq51_e1692_d_b12, eq51_e1692_d_b13, eq51_e1692_d_b14, eq51_e1692_d_b15, eq51_e1692_d_b16, eq51_e1692_d_b17, eq51_e1692_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 14] = [eq51_e1694_d_n0, eq51_e1694_d_n1, eq51_e1694_d_n2, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, eq51_e1694_d_n13];
        let eq51_reactive_branch_derivatives: [f64; 18] = [eq51_e1694_d_b0, eq51_e1694_d_b1, eq51_e1694_d_b2, eq51_e1694_d_b3, eq51_e1694_d_b4, eq51_e1694_d_b5, eq51_e1694_d_b6, eq51_e1694_d_b7, eq51_e1694_d_b8, eq51_e1694_d_b9, eq51_e1694_d_b10, eq51_e1694_d_b11, eq51_e1694_d_b12, eq51_e1694_d_b13, eq51_e1694_d_b14, eq51_e1694_d_b15, eq51_e1694_d_b16, eq51_e1694_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[8]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq52_e1703, eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13, eq52_e1703_d_b0, eq52_e1703_d_b1, eq52_e1703_d_b2, eq52_e1703_d_b3, eq52_e1703_d_b4, eq52_e1703_d_b5, eq52_e1703_d_b6, eq52_e1703_d_b7, eq52_e1703_d_b8, eq52_e1703_d_b9, eq52_e1703_d_b10, eq52_e1703_d_b11, eq52_e1703_d_b12, eq52_e1703_d_b13, eq52_e1703_d_b14, eq52_e1703_d_b15, eq52_e1703_d_b16, eq52_e1703_d_b17, eq52_e1703_q,) = {
    if s.b[1553] {
        let eq52_e1698: f64 = (p.p33 * (nv10 - nv3));
        let eq52_e1698_d_n3: f64 = (-p.p33);
        let eq52_e1698_d_n10: f64 = p.p33;
        let eq52_e1700: f64 = (eq52_e1698 * s.v[336]);
        let eq52_e1700_d_n0: f64 = (eq52_e1698 * s.dn[336][0]);
        let eq52_e1700_d_n1: f64 = (eq52_e1698 * s.dn[336][1]);
        let eq52_e1700_d_n2: f64 = (eq52_e1698 * s.dn[336][2]);
        let eq52_e1700_d_n3: f64 = ((eq52_e1698_d_n3 * s.v[336]) + (eq52_e1698 * s.dn[336][3]));
        let eq52_e1700_d_n4: f64 = (eq52_e1698 * s.dn[336][4]);
        let eq52_e1700_d_n5: f64 = (eq52_e1698 * s.dn[336][5]);
        let eq52_e1700_d_n6: f64 = (eq52_e1698 * s.dn[336][6]);
        let eq52_e1700_d_n7: f64 = (eq52_e1698 * s.dn[336][7]);
        let eq52_e1700_d_n8: f64 = (eq52_e1698 * s.dn[336][8]);
        let eq52_e1700_d_n9: f64 = (eq52_e1698 * s.dn[336][9]);
        let eq52_e1700_d_n10: f64 = ((eq52_e1698_d_n10 * s.v[336]) + (eq52_e1698 * s.dn[336][10]));
        let eq52_e1700_d_n11: f64 = (eq52_e1698 * s.dn[336][11]);
        let eq52_e1700_d_n12: f64 = (eq52_e1698 * s.dn[336][12]);
        let eq52_e1700_d_n13: f64 = (eq52_e1698 * s.dn[336][13]);
        let eq52_e1700_d_b0: f64 = (eq52_e1698 * s.db[336][0]);
        let eq52_e1700_d_b1: f64 = (eq52_e1698 * s.db[336][1]);
        let eq52_e1700_d_b2: f64 = (eq52_e1698 * s.db[336][2]);
        let eq52_e1700_d_b3: f64 = (eq52_e1698 * s.db[336][3]);
        let eq52_e1700_d_b4: f64 = (eq52_e1698 * s.db[336][4]);
        let eq52_e1700_d_b5: f64 = (eq52_e1698 * s.db[336][5]);
        let eq52_e1700_d_b6: f64 = (eq52_e1698 * s.db[336][6]);
        let eq52_e1700_d_b7: f64 = (eq52_e1698 * s.db[336][7]);
        let eq52_e1700_d_b8: f64 = (eq52_e1698 * s.db[336][8]);
        let eq52_e1700_d_b9: f64 = (eq52_e1698 * s.db[336][9]);
        let eq52_e1700_d_b10: f64 = (eq52_e1698 * s.db[336][10]);
        let eq52_e1700_d_b11: f64 = (eq52_e1698 * s.db[336][11]);
        let eq52_e1700_d_b12: f64 = (eq52_e1698 * s.db[336][12]);
        let eq52_e1700_d_b13: f64 = (eq52_e1698 * s.db[336][13]);
        let eq52_e1700_d_b14: f64 = (eq52_e1698 * s.db[336][14]);
        let eq52_e1700_d_b15: f64 = (eq52_e1698 * s.db[336][15]);
        let eq52_e1700_d_b16: f64 = (eq52_e1698 * s.db[336][16]);
        let eq52_e1700_d_b17: f64 = (eq52_e1698 * s.db[336][17]);
        let eq52_e1701_q: f64 = eq52_e1700;
        (eq52_e1700, eq52_e1700_d_n0, eq52_e1700_d_n1, eq52_e1700_d_n2, eq52_e1700_d_n3, eq52_e1700_d_n4, eq52_e1700_d_n5, eq52_e1700_d_n6, eq52_e1700_d_n7, eq52_e1700_d_n8, eq52_e1700_d_n9, eq52_e1700_d_n10, eq52_e1700_d_n11, eq52_e1700_d_n12, eq52_e1700_d_n13, eq52_e1700_d_b0, eq52_e1700_d_b1, eq52_e1700_d_b2, eq52_e1700_d_b3, eq52_e1700_d_b4, eq52_e1700_d_b5, eq52_e1700_d_b6, eq52_e1700_d_b7, eq52_e1700_d_b8, eq52_e1700_d_b9, eq52_e1700_d_b10, eq52_e1700_d_b11, eq52_e1700_d_b12, eq52_e1700_d_b13, eq52_e1700_d_b14, eq52_e1700_d_b15, eq52_e1700_d_b16, eq52_e1700_d_b17, eq52_e1701_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 14] = [eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13];
        let eq52_reactive_branch_derivatives: [f64; 18] = [eq52_e1703_d_b0, eq52_e1703_d_b1, eq52_e1703_d_b2, eq52_e1703_d_b3, eq52_e1703_d_b4, eq52_e1703_d_b5, eq52_e1703_d_b6, eq52_e1703_d_b7, eq52_e1703_d_b8, eq52_e1703_d_b9, eq52_e1703_d_b10, eq52_e1703_d_b11, eq52_e1703_d_b12, eq52_e1703_d_b13, eq52_e1703_d_b14, eq52_e1703_d_b15, eq52_e1703_d_b16, eq52_e1703_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[3]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq53_e1713, eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13, eq53_e1713_d_b0, eq53_e1713_d_b1, eq53_e1713_d_b2, eq53_e1713_d_b3, eq53_e1713_d_b4, eq53_e1713_d_b5, eq53_e1713_d_b6, eq53_e1713_d_b7, eq53_e1713_d_b8, eq53_e1713_d_b9, eq53_e1713_d_b10, eq53_e1713_d_b11, eq53_e1713_d_b12, eq53_e1713_d_b13, eq53_e1713_d_b14, eq53_e1713_d_b15, eq53_e1713_d_b16, eq53_e1713_d_b17, eq53_e1713_q,) = {
    if (!s.b[1553]) {
        let eq53_e1709: f64 = (p.p33 * s.v[895]);
        let eq53_e1709_d_n0: f64 = (p.p33 * s.dn[895][0]);
        let eq53_e1709_d_n1: f64 = (p.p33 * s.dn[895][1]);
        let eq53_e1709_d_n2: f64 = (p.p33 * s.dn[895][2]);
        let eq53_e1709_d_n3: f64 = (p.p33 * s.dn[895][3]);
        let eq53_e1709_d_n4: f64 = (p.p33 * s.dn[895][4]);
        let eq53_e1709_d_n5: f64 = (p.p33 * s.dn[895][5]);
        let eq53_e1709_d_n6: f64 = (p.p33 * s.dn[895][6]);
        let eq53_e1709_d_n7: f64 = (p.p33 * s.dn[895][7]);
        let eq53_e1709_d_n8: f64 = (p.p33 * s.dn[895][8]);
        let eq53_e1709_d_n9: f64 = (p.p33 * s.dn[895][9]);
        let eq53_e1709_d_n10: f64 = (p.p33 * s.dn[895][10]);
        let eq53_e1709_d_n11: f64 = (p.p33 * s.dn[895][11]);
        let eq53_e1709_d_n12: f64 = (p.p33 * s.dn[895][12]);
        let eq53_e1709_d_n13: f64 = (p.p33 * s.dn[895][13]);
        let eq53_e1709_d_b0: f64 = (p.p33 * s.db[895][0]);
        let eq53_e1709_d_b1: f64 = (p.p33 * s.db[895][1]);
        let eq53_e1709_d_b2: f64 = (p.p33 * s.db[895][2]);
        let eq53_e1709_d_b3: f64 = (p.p33 * s.db[895][3]);
        let eq53_e1709_d_b4: f64 = (p.p33 * s.db[895][4]);
        let eq53_e1709_d_b5: f64 = (p.p33 * s.db[895][5]);
        let eq53_e1709_d_b6: f64 = (p.p33 * s.db[895][6]);
        let eq53_e1709_d_b7: f64 = (p.p33 * s.db[895][7]);
        let eq53_e1709_d_b8: f64 = (p.p33 * s.db[895][8]);
        let eq53_e1709_d_b9: f64 = (p.p33 * s.db[895][9]);
        let eq53_e1709_d_b10: f64 = (p.p33 * s.db[895][10]);
        let eq53_e1709_d_b11: f64 = (p.p33 * s.db[895][11]);
        let eq53_e1709_d_b12: f64 = (p.p33 * s.db[895][12]);
        let eq53_e1709_d_b13: f64 = (p.p33 * s.db[895][13]);
        let eq53_e1709_d_b14: f64 = (p.p33 * s.db[895][14]);
        let eq53_e1709_d_b15: f64 = (p.p33 * s.db[895][15]);
        let eq53_e1709_d_b16: f64 = (p.p33 * s.db[895][16]);
        let eq53_e1709_d_b17: f64 = (p.p33 * s.db[895][17]);
        let eq53_e1710_q: f64 = eq53_e1709;
        let eq53_e1711: f64 = (p.p37 * eq53_e1709);
        let eq53_e1711_d_n0: f64 = (p.p37 * eq53_e1709_d_n0);
        let eq53_e1711_d_n1: f64 = (p.p37 * eq53_e1709_d_n1);
        let eq53_e1711_d_n2: f64 = (p.p37 * eq53_e1709_d_n2);
        let eq53_e1711_d_n3: f64 = (p.p37 * eq53_e1709_d_n3);
        let eq53_e1711_d_n4: f64 = (p.p37 * eq53_e1709_d_n4);
        let eq53_e1711_d_n5: f64 = (p.p37 * eq53_e1709_d_n5);
        let eq53_e1711_d_n6: f64 = (p.p37 * eq53_e1709_d_n6);
        let eq53_e1711_d_n7: f64 = (p.p37 * eq53_e1709_d_n7);
        let eq53_e1711_d_n8: f64 = (p.p37 * eq53_e1709_d_n8);
        let eq53_e1711_d_n9: f64 = (p.p37 * eq53_e1709_d_n9);
        let eq53_e1711_d_n10: f64 = (p.p37 * eq53_e1709_d_n10);
        let eq53_e1711_d_n11: f64 = (p.p37 * eq53_e1709_d_n11);
        let eq53_e1711_d_n12: f64 = (p.p37 * eq53_e1709_d_n12);
        let eq53_e1711_d_n13: f64 = (p.p37 * eq53_e1709_d_n13);
        let eq53_e1711_d_b0: f64 = (p.p37 * eq53_e1709_d_b0);
        let eq53_e1711_d_b1: f64 = (p.p37 * eq53_e1709_d_b1);
        let eq53_e1711_d_b2: f64 = (p.p37 * eq53_e1709_d_b2);
        let eq53_e1711_d_b3: f64 = (p.p37 * eq53_e1709_d_b3);
        let eq53_e1711_d_b4: f64 = (p.p37 * eq53_e1709_d_b4);
        let eq53_e1711_d_b5: f64 = (p.p37 * eq53_e1709_d_b5);
        let eq53_e1711_d_b6: f64 = (p.p37 * eq53_e1709_d_b6);
        let eq53_e1711_d_b7: f64 = (p.p37 * eq53_e1709_d_b7);
        let eq53_e1711_d_b8: f64 = (p.p37 * eq53_e1709_d_b8);
        let eq53_e1711_d_b9: f64 = (p.p37 * eq53_e1709_d_b9);
        let eq53_e1711_d_b10: f64 = (p.p37 * eq53_e1709_d_b10);
        let eq53_e1711_d_b11: f64 = (p.p37 * eq53_e1709_d_b11);
        let eq53_e1711_d_b12: f64 = (p.p37 * eq53_e1709_d_b12);
        let eq53_e1711_d_b13: f64 = (p.p37 * eq53_e1709_d_b13);
        let eq53_e1711_d_b14: f64 = (p.p37 * eq53_e1709_d_b14);
        let eq53_e1711_d_b15: f64 = (p.p37 * eq53_e1709_d_b15);
        let eq53_e1711_d_b16: f64 = (p.p37 * eq53_e1709_d_b16);
        let eq53_e1711_d_b17: f64 = (p.p37 * eq53_e1709_d_b17);
        let eq53_e1711_q: f64 = (p.p37 * eq53_e1710_q);
        (eq53_e1711, eq53_e1711_d_n0, eq53_e1711_d_n1, eq53_e1711_d_n2, eq53_e1711_d_n3, eq53_e1711_d_n4, eq53_e1711_d_n5, eq53_e1711_d_n6, eq53_e1711_d_n7, eq53_e1711_d_n8, eq53_e1711_d_n9, eq53_e1711_d_n10, eq53_e1711_d_n11, eq53_e1711_d_n12, eq53_e1711_d_n13, eq53_e1711_d_b0, eq53_e1711_d_b1, eq53_e1711_d_b2, eq53_e1711_d_b3, eq53_e1711_d_b4, eq53_e1711_d_b5, eq53_e1711_d_b6, eq53_e1711_d_b7, eq53_e1711_d_b8, eq53_e1711_d_b9, eq53_e1711_d_b10, eq53_e1711_d_b11, eq53_e1711_d_b12, eq53_e1711_d_b13, eq53_e1711_d_b14, eq53_e1711_d_b15, eq53_e1711_d_b16, eq53_e1711_d_b17, eq53_e1711_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 14] = [eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13];
        let eq53_reactive_branch_derivatives: [f64; 18] = [eq53_e1713_d_b0, eq53_e1713_d_b1, eq53_e1713_d_b2, eq53_e1713_d_b3, eq53_e1713_d_b4, eq53_e1713_d_b5, eq53_e1713_d_b6, eq53_e1713_d_b7, eq53_e1713_d_b8, eq53_e1713_d_b9, eq53_e1713_d_b10, eq53_e1713_d_b11, eq53_e1713_d_b12, eq53_e1713_d_b13, eq53_e1713_d_b14, eq53_e1713_d_b15, eq53_e1713_d_b16, eq53_e1713_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq54_e1723, eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13, eq54_e1723_d_b0, eq54_e1723_d_b1, eq54_e1723_d_b2, eq54_e1723_d_b3, eq54_e1723_d_b4, eq54_e1723_d_b5, eq54_e1723_d_b6, eq54_e1723_d_b7, eq54_e1723_d_b8, eq54_e1723_d_b9, eq54_e1723_d_b10, eq54_e1723_d_b11, eq54_e1723_d_b12, eq54_e1723_d_b13, eq54_e1723_d_b14, eq54_e1723_d_b15, eq54_e1723_d_b16, eq54_e1723_d_b17, eq54_e1723_q,) = {
    if (!s.b[1553]) {
        let eq54_e1719: f64 = (p.p33 * s.v[896]);
        let eq54_e1719_d_n0: f64 = (p.p33 * s.dn[896][0]);
        let eq54_e1719_d_n1: f64 = (p.p33 * s.dn[896][1]);
        let eq54_e1719_d_n2: f64 = (p.p33 * s.dn[896][2]);
        let eq54_e1719_d_n3: f64 = (p.p33 * s.dn[896][3]);
        let eq54_e1719_d_n4: f64 = (p.p33 * s.dn[896][4]);
        let eq54_e1719_d_n5: f64 = (p.p33 * s.dn[896][5]);
        let eq54_e1719_d_n6: f64 = (p.p33 * s.dn[896][6]);
        let eq54_e1719_d_n7: f64 = (p.p33 * s.dn[896][7]);
        let eq54_e1719_d_n8: f64 = (p.p33 * s.dn[896][8]);
        let eq54_e1719_d_n9: f64 = (p.p33 * s.dn[896][9]);
        let eq54_e1719_d_n10: f64 = (p.p33 * s.dn[896][10]);
        let eq54_e1719_d_n11: f64 = (p.p33 * s.dn[896][11]);
        let eq54_e1719_d_n12: f64 = (p.p33 * s.dn[896][12]);
        let eq54_e1719_d_n13: f64 = (p.p33 * s.dn[896][13]);
        let eq54_e1719_d_b0: f64 = (p.p33 * s.db[896][0]);
        let eq54_e1719_d_b1: f64 = (p.p33 * s.db[896][1]);
        let eq54_e1719_d_b2: f64 = (p.p33 * s.db[896][2]);
        let eq54_e1719_d_b3: f64 = (p.p33 * s.db[896][3]);
        let eq54_e1719_d_b4: f64 = (p.p33 * s.db[896][4]);
        let eq54_e1719_d_b5: f64 = (p.p33 * s.db[896][5]);
        let eq54_e1719_d_b6: f64 = (p.p33 * s.db[896][6]);
        let eq54_e1719_d_b7: f64 = (p.p33 * s.db[896][7]);
        let eq54_e1719_d_b8: f64 = (p.p33 * s.db[896][8]);
        let eq54_e1719_d_b9: f64 = (p.p33 * s.db[896][9]);
        let eq54_e1719_d_b10: f64 = (p.p33 * s.db[896][10]);
        let eq54_e1719_d_b11: f64 = (p.p33 * s.db[896][11]);
        let eq54_e1719_d_b12: f64 = (p.p33 * s.db[896][12]);
        let eq54_e1719_d_b13: f64 = (p.p33 * s.db[896][13]);
        let eq54_e1719_d_b14: f64 = (p.p33 * s.db[896][14]);
        let eq54_e1719_d_b15: f64 = (p.p33 * s.db[896][15]);
        let eq54_e1719_d_b16: f64 = (p.p33 * s.db[896][16]);
        let eq54_e1719_d_b17: f64 = (p.p33 * s.db[896][17]);
        let eq54_e1720_q: f64 = eq54_e1719;
        let eq54_e1721: f64 = (p.p37 * eq54_e1719);
        let eq54_e1721_d_n0: f64 = (p.p37 * eq54_e1719_d_n0);
        let eq54_e1721_d_n1: f64 = (p.p37 * eq54_e1719_d_n1);
        let eq54_e1721_d_n2: f64 = (p.p37 * eq54_e1719_d_n2);
        let eq54_e1721_d_n3: f64 = (p.p37 * eq54_e1719_d_n3);
        let eq54_e1721_d_n4: f64 = (p.p37 * eq54_e1719_d_n4);
        let eq54_e1721_d_n5: f64 = (p.p37 * eq54_e1719_d_n5);
        let eq54_e1721_d_n6: f64 = (p.p37 * eq54_e1719_d_n6);
        let eq54_e1721_d_n7: f64 = (p.p37 * eq54_e1719_d_n7);
        let eq54_e1721_d_n8: f64 = (p.p37 * eq54_e1719_d_n8);
        let eq54_e1721_d_n9: f64 = (p.p37 * eq54_e1719_d_n9);
        let eq54_e1721_d_n10: f64 = (p.p37 * eq54_e1719_d_n10);
        let eq54_e1721_d_n11: f64 = (p.p37 * eq54_e1719_d_n11);
        let eq54_e1721_d_n12: f64 = (p.p37 * eq54_e1719_d_n12);
        let eq54_e1721_d_n13: f64 = (p.p37 * eq54_e1719_d_n13);
        let eq54_e1721_d_b0: f64 = (p.p37 * eq54_e1719_d_b0);
        let eq54_e1721_d_b1: f64 = (p.p37 * eq54_e1719_d_b1);
        let eq54_e1721_d_b2: f64 = (p.p37 * eq54_e1719_d_b2);
        let eq54_e1721_d_b3: f64 = (p.p37 * eq54_e1719_d_b3);
        let eq54_e1721_d_b4: f64 = (p.p37 * eq54_e1719_d_b4);
        let eq54_e1721_d_b5: f64 = (p.p37 * eq54_e1719_d_b5);
        let eq54_e1721_d_b6: f64 = (p.p37 * eq54_e1719_d_b6);
        let eq54_e1721_d_b7: f64 = (p.p37 * eq54_e1719_d_b7);
        let eq54_e1721_d_b8: f64 = (p.p37 * eq54_e1719_d_b8);
        let eq54_e1721_d_b9: f64 = (p.p37 * eq54_e1719_d_b9);
        let eq54_e1721_d_b10: f64 = (p.p37 * eq54_e1719_d_b10);
        let eq54_e1721_d_b11: f64 = (p.p37 * eq54_e1719_d_b11);
        let eq54_e1721_d_b12: f64 = (p.p37 * eq54_e1719_d_b12);
        let eq54_e1721_d_b13: f64 = (p.p37 * eq54_e1719_d_b13);
        let eq54_e1721_d_b14: f64 = (p.p37 * eq54_e1719_d_b14);
        let eq54_e1721_d_b15: f64 = (p.p37 * eq54_e1719_d_b15);
        let eq54_e1721_d_b16: f64 = (p.p37 * eq54_e1719_d_b16);
        let eq54_e1721_d_b17: f64 = (p.p37 * eq54_e1719_d_b17);
        let eq54_e1721_q: f64 = (p.p37 * eq54_e1720_q);
        (eq54_e1721, eq54_e1721_d_n0, eq54_e1721_d_n1, eq54_e1721_d_n2, eq54_e1721_d_n3, eq54_e1721_d_n4, eq54_e1721_d_n5, eq54_e1721_d_n6, eq54_e1721_d_n7, eq54_e1721_d_n8, eq54_e1721_d_n9, eq54_e1721_d_n10, eq54_e1721_d_n11, eq54_e1721_d_n12, eq54_e1721_d_n13, eq54_e1721_d_b0, eq54_e1721_d_b1, eq54_e1721_d_b2, eq54_e1721_d_b3, eq54_e1721_d_b4, eq54_e1721_d_b5, eq54_e1721_d_b6, eq54_e1721_d_b7, eq54_e1721_d_b8, eq54_e1721_d_b9, eq54_e1721_d_b10, eq54_e1721_d_b11, eq54_e1721_d_b12, eq54_e1721_d_b13, eq54_e1721_d_b14, eq54_e1721_d_b15, eq54_e1721_d_b16, eq54_e1721_d_b17, eq54_e1721_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_reactive_node_derivatives: [f64; 14] = [eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13];
        let eq54_reactive_branch_derivatives: [f64; 18] = [eq54_e1723_d_b0, eq54_e1723_d_b1, eq54_e1723_d_b2, eq54_e1723_d_b3, eq54_e1723_d_b4, eq54_e1723_d_b5, eq54_e1723_d_b6, eq54_e1723_d_b7, eq54_e1723_d_b8, eq54_e1723_d_b9, eq54_e1723_d_b10, eq54_e1723_d_b11, eq54_e1723_d_b12, eq54_e1723_d_b13, eq54_e1723_d_b14, eq54_e1723_d_b15, eq54_e1723_d_b16, eq54_e1723_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq54_reactive_node_derivatives,
            branches,
            &eq54_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq55_e1733, eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13, eq55_e1733_d_b0, eq55_e1733_d_b1, eq55_e1733_d_b2, eq55_e1733_d_b3, eq55_e1733_d_b4, eq55_e1733_d_b5, eq55_e1733_d_b6, eq55_e1733_d_b7, eq55_e1733_d_b8, eq55_e1733_d_b9, eq55_e1733_d_b10, eq55_e1733_d_b11, eq55_e1733_d_b12, eq55_e1733_d_b13, eq55_e1733_d_b14, eq55_e1733_d_b15, eq55_e1733_d_b16, eq55_e1733_d_b17, eq55_e1733_q,) = {
    if (!s.b[1553]) {
        let eq55_e1728: f64 = (p.p33 * (nv9 - nv3));
        let eq55_e1728_d_n3: f64 = (-p.p33);
        let eq55_e1728_d_n9: f64 = p.p33;
        let eq55_e1730: f64 = (eq55_e1728 * s.v[336]);
        let eq55_e1730_d_n0: f64 = (eq55_e1728 * s.dn[336][0]);
        let eq55_e1730_d_n1: f64 = (eq55_e1728 * s.dn[336][1]);
        let eq55_e1730_d_n2: f64 = (eq55_e1728 * s.dn[336][2]);
        let eq55_e1730_d_n3: f64 = ((eq55_e1728_d_n3 * s.v[336]) + (eq55_e1728 * s.dn[336][3]));
        let eq55_e1730_d_n4: f64 = (eq55_e1728 * s.dn[336][4]);
        let eq55_e1730_d_n5: f64 = (eq55_e1728 * s.dn[336][5]);
        let eq55_e1730_d_n6: f64 = (eq55_e1728 * s.dn[336][6]);
        let eq55_e1730_d_n7: f64 = (eq55_e1728 * s.dn[336][7]);
        let eq55_e1730_d_n8: f64 = (eq55_e1728 * s.dn[336][8]);
        let eq55_e1730_d_n9: f64 = ((eq55_e1728_d_n9 * s.v[336]) + (eq55_e1728 * s.dn[336][9]));
        let eq55_e1730_d_n10: f64 = (eq55_e1728 * s.dn[336][10]);
        let eq55_e1730_d_n11: f64 = (eq55_e1728 * s.dn[336][11]);
        let eq55_e1730_d_n12: f64 = (eq55_e1728 * s.dn[336][12]);
        let eq55_e1730_d_n13: f64 = (eq55_e1728 * s.dn[336][13]);
        let eq55_e1730_d_b0: f64 = (eq55_e1728 * s.db[336][0]);
        let eq55_e1730_d_b1: f64 = (eq55_e1728 * s.db[336][1]);
        let eq55_e1730_d_b2: f64 = (eq55_e1728 * s.db[336][2]);
        let eq55_e1730_d_b3: f64 = (eq55_e1728 * s.db[336][3]);
        let eq55_e1730_d_b4: f64 = (eq55_e1728 * s.db[336][4]);
        let eq55_e1730_d_b5: f64 = (eq55_e1728 * s.db[336][5]);
        let eq55_e1730_d_b6: f64 = (eq55_e1728 * s.db[336][6]);
        let eq55_e1730_d_b7: f64 = (eq55_e1728 * s.db[336][7]);
        let eq55_e1730_d_b8: f64 = (eq55_e1728 * s.db[336][8]);
        let eq55_e1730_d_b9: f64 = (eq55_e1728 * s.db[336][9]);
        let eq55_e1730_d_b10: f64 = (eq55_e1728 * s.db[336][10]);
        let eq55_e1730_d_b11: f64 = (eq55_e1728 * s.db[336][11]);
        let eq55_e1730_d_b12: f64 = (eq55_e1728 * s.db[336][12]);
        let eq55_e1730_d_b13: f64 = (eq55_e1728 * s.db[336][13]);
        let eq55_e1730_d_b14: f64 = (eq55_e1728 * s.db[336][14]);
        let eq55_e1730_d_b15: f64 = (eq55_e1728 * s.db[336][15]);
        let eq55_e1730_d_b16: f64 = (eq55_e1728 * s.db[336][16]);
        let eq55_e1730_d_b17: f64 = (eq55_e1728 * s.db[336][17]);
        let eq55_e1731_q: f64 = eq55_e1730;
        (eq55_e1730, eq55_e1730_d_n0, eq55_e1730_d_n1, eq55_e1730_d_n2, eq55_e1730_d_n3, eq55_e1730_d_n4, eq55_e1730_d_n5, eq55_e1730_d_n6, eq55_e1730_d_n7, eq55_e1730_d_n8, eq55_e1730_d_n9, eq55_e1730_d_n10, eq55_e1730_d_n11, eq55_e1730_d_n12, eq55_e1730_d_n13, eq55_e1730_d_b0, eq55_e1730_d_b1, eq55_e1730_d_b2, eq55_e1730_d_b3, eq55_e1730_d_b4, eq55_e1730_d_b5, eq55_e1730_d_b6, eq55_e1730_d_b7, eq55_e1730_d_b8, eq55_e1730_d_b9, eq55_e1730_d_b10, eq55_e1730_d_b11, eq55_e1730_d_b12, eq55_e1730_d_b13, eq55_e1730_d_b14, eq55_e1730_d_b15, eq55_e1730_d_b16, eq55_e1730_d_b17, eq55_e1731_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 14] = [eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13];
        let eq55_reactive_branch_derivatives: [f64; 18] = [eq55_e1733_d_b0, eq55_e1733_d_b1, eq55_e1733_d_b2, eq55_e1733_d_b3, eq55_e1733_d_b4, eq55_e1733_d_b5, eq55_e1733_d_b6, eq55_e1733_d_b7, eq55_e1733_d_b8, eq55_e1733_d_b9, eq55_e1733_d_b10, eq55_e1733_d_b11, eq55_e1733_d_b12, eq55_e1733_d_b13, eq55_e1733_d_b14, eq55_e1733_d_b15, eq55_e1733_d_b16, eq55_e1733_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[3]),
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let eq56_e1736: f64 = (p.p33 * s.v[87]);
        let eq56_e1736_d_n0: f64 = (p.p33 * s.dn[87][0]);
        let eq56_e1736_d_n1: f64 = (p.p33 * s.dn[87][1]);
        let eq56_e1736_d_n2: f64 = (p.p33 * s.dn[87][2]);
        let eq56_e1736_d_n3: f64 = (p.p33 * s.dn[87][3]);
        let eq56_e1736_d_n4: f64 = (p.p33 * s.dn[87][4]);
        let eq56_e1736_d_n5: f64 = (p.p33 * s.dn[87][5]);
        let eq56_e1736_d_n6: f64 = (p.p33 * s.dn[87][6]);
        let eq56_e1736_d_n7: f64 = (p.p33 * s.dn[87][7]);
        let eq56_e1736_d_n8: f64 = (p.p33 * s.dn[87][8]);
        let eq56_e1736_d_n9: f64 = (p.p33 * s.dn[87][9]);
        let eq56_e1736_d_n10: f64 = (p.p33 * s.dn[87][10]);
        let eq56_e1736_d_n11: f64 = (p.p33 * s.dn[87][11]);
        let eq56_e1736_d_n12: f64 = (p.p33 * s.dn[87][12]);
        let eq56_e1736_d_n13: f64 = (p.p33 * s.dn[87][13]);
        let eq56_e1736_d_b0: f64 = (p.p33 * s.db[87][0]);
        let eq56_e1736_d_b1: f64 = (p.p33 * s.db[87][1]);
        let eq56_e1736_d_b2: f64 = (p.p33 * s.db[87][2]);
        let eq56_e1736_d_b3: f64 = (p.p33 * s.db[87][3]);
        let eq56_e1736_d_b4: f64 = (p.p33 * s.db[87][4]);
        let eq56_e1736_d_b5: f64 = (p.p33 * s.db[87][5]);
        let eq56_e1736_d_b6: f64 = (p.p33 * s.db[87][6]);
        let eq56_e1736_d_b7: f64 = (p.p33 * s.db[87][7]);
        let eq56_e1736_d_b8: f64 = (p.p33 * s.db[87][8]);
        let eq56_e1736_d_b9: f64 = (p.p33 * s.db[87][9]);
        let eq56_e1736_d_b10: f64 = (p.p33 * s.db[87][10]);
        let eq56_e1736_d_b11: f64 = (p.p33 * s.db[87][11]);
        let eq56_e1736_d_b12: f64 = (p.p33 * s.db[87][12]);
        let eq56_e1736_d_b13: f64 = (p.p33 * s.db[87][13]);
        let eq56_e1736_d_b14: f64 = (p.p33 * s.db[87][14]);
        let eq56_e1736_d_b15: f64 = (p.p33 * s.db[87][15]);
        let eq56_e1736_d_b16: f64 = (p.p33 * s.db[87][16]);
        let eq56_e1736_d_b17: f64 = (p.p33 * s.db[87][17]);
        let eq56_e1737_q: f64 = eq56_e1736;
        let eq56_reactive_node_derivatives: [f64; 14] = [eq56_e1736_d_n0, eq56_e1736_d_n1, eq56_e1736_d_n2, eq56_e1736_d_n3, eq56_e1736_d_n4, eq56_e1736_d_n5, eq56_e1736_d_n6, eq56_e1736_d_n7, eq56_e1736_d_n8, eq56_e1736_d_n9, eq56_e1736_d_n10, eq56_e1736_d_n11, eq56_e1736_d_n12, eq56_e1736_d_n13];
        let eq56_reactive_branch_derivatives: [f64; 18] = [eq56_e1736_d_b0, eq56_e1736_d_b1, eq56_e1736_d_b2, eq56_e1736_d_b3, eq56_e1736_d_b4, eq56_e1736_d_b5, eq56_e1736_d_b6, eq56_e1736_d_b7, eq56_e1736_d_b8, eq56_e1736_d_b9, eq56_e1736_d_b10, eq56_e1736_d_b11, eq56_e1736_d_b12, eq56_e1736_d_b13, eq56_e1736_d_b14, eq56_e1736_d_b15, eq56_e1736_d_b16, eq56_e1736_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let eq57_e1740: f64 = (p.p33 * s.v[86]);
        let eq57_e1740_d_n0: f64 = (p.p33 * s.dn[86][0]);
        let eq57_e1740_d_n1: f64 = (p.p33 * s.dn[86][1]);
        let eq57_e1740_d_n2: f64 = (p.p33 * s.dn[86][2]);
        let eq57_e1740_d_n3: f64 = (p.p33 * s.dn[86][3]);
        let eq57_e1740_d_n4: f64 = (p.p33 * s.dn[86][4]);
        let eq57_e1740_d_n5: f64 = (p.p33 * s.dn[86][5]);
        let eq57_e1740_d_n6: f64 = (p.p33 * s.dn[86][6]);
        let eq57_e1740_d_n7: f64 = (p.p33 * s.dn[86][7]);
        let eq57_e1740_d_n8: f64 = (p.p33 * s.dn[86][8]);
        let eq57_e1740_d_n9: f64 = (p.p33 * s.dn[86][9]);
        let eq57_e1740_d_n10: f64 = (p.p33 * s.dn[86][10]);
        let eq57_e1740_d_n11: f64 = (p.p33 * s.dn[86][11]);
        let eq57_e1740_d_n12: f64 = (p.p33 * s.dn[86][12]);
        let eq57_e1740_d_n13: f64 = (p.p33 * s.dn[86][13]);
        let eq57_e1740_d_b0: f64 = (p.p33 * s.db[86][0]);
        let eq57_e1740_d_b1: f64 = (p.p33 * s.db[86][1]);
        let eq57_e1740_d_b2: f64 = (p.p33 * s.db[86][2]);
        let eq57_e1740_d_b3: f64 = (p.p33 * s.db[86][3]);
        let eq57_e1740_d_b4: f64 = (p.p33 * s.db[86][4]);
        let eq57_e1740_d_b5: f64 = (p.p33 * s.db[86][5]);
        let eq57_e1740_d_b6: f64 = (p.p33 * s.db[86][6]);
        let eq57_e1740_d_b7: f64 = (p.p33 * s.db[86][7]);
        let eq57_e1740_d_b8: f64 = (p.p33 * s.db[86][8]);
        let eq57_e1740_d_b9: f64 = (p.p33 * s.db[86][9]);
        let eq57_e1740_d_b10: f64 = (p.p33 * s.db[86][10]);
        let eq57_e1740_d_b11: f64 = (p.p33 * s.db[86][11]);
        let eq57_e1740_d_b12: f64 = (p.p33 * s.db[86][12]);
        let eq57_e1740_d_b13: f64 = (p.p33 * s.db[86][13]);
        let eq57_e1740_d_b14: f64 = (p.p33 * s.db[86][14]);
        let eq57_e1740_d_b15: f64 = (p.p33 * s.db[86][15]);
        let eq57_e1740_d_b16: f64 = (p.p33 * s.db[86][16]);
        let eq57_e1740_d_b17: f64 = (p.p33 * s.db[86][17]);
        let eq57_e1741_q: f64 = eq57_e1740;
        let eq57_reactive_node_derivatives: [f64; 14] = [eq57_e1740_d_n0, eq57_e1740_d_n1, eq57_e1740_d_n2, eq57_e1740_d_n3, eq57_e1740_d_n4, eq57_e1740_d_n5, eq57_e1740_d_n6, eq57_e1740_d_n7, eq57_e1740_d_n8, eq57_e1740_d_n9, eq57_e1740_d_n10, eq57_e1740_d_n11, eq57_e1740_d_n12, eq57_e1740_d_n13];
        let eq57_reactive_branch_derivatives: [f64; 18] = [eq57_e1740_d_b0, eq57_e1740_d_b1, eq57_e1740_d_b2, eq57_e1740_d_b3, eq57_e1740_d_b4, eq57_e1740_d_b5, eq57_e1740_d_b6, eq57_e1740_d_b7, eq57_e1740_d_b8, eq57_e1740_d_b9, eq57_e1740_d_b10, eq57_e1740_d_b11, eq57_e1740_d_b12, eq57_e1740_d_b13, eq57_e1740_d_b14, eq57_e1740_d_b15, eq57_e1740_d_b16, eq57_e1740_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[3]),
            nodes,
            &eq57_reactive_node_derivatives,
            branches,
            &eq57_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq71_e1869, eq71_e1869_d_n0, eq71_e1869_d_n1, eq71_e1869_d_n2, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12, eq71_e1869_d_n13, eq71_e1869_d_b0, eq71_e1869_d_b1, eq71_e1869_d_b2, eq71_e1869_d_b3, eq71_e1869_d_b4, eq71_e1869_d_b5, eq71_e1869_d_b6, eq71_e1869_d_b7, eq71_e1869_d_b8, eq71_e1869_d_b9, eq71_e1869_d_b10, eq71_e1869_d_b11, eq71_e1869_d_b12, eq71_e1869_d_b13, eq71_e1869_d_b14, eq71_e1869_d_b15, eq71_e1869_d_b16, eq71_e1869_d_b17, eq71_e1869_q, eq71_e1869_q_d_n0, eq71_e1869_q_d_n1, eq71_e1869_q_d_n2, eq71_e1869_q_d_n3, eq71_e1869_q_d_n4, eq71_e1869_q_d_n5, eq71_e1869_q_d_n6, eq71_e1869_q_d_n7, eq71_e1869_q_d_n8, eq71_e1869_q_d_n9, eq71_e1869_q_d_n10, eq71_e1869_q_d_n11, eq71_e1869_q_d_n12, eq71_e1869_q_d_n13, eq71_e1869_q_d_b0, eq71_e1869_q_d_b1, eq71_e1869_q_d_b2, eq71_e1869_q_d_b3, eq71_e1869_q_d_b4, eq71_e1869_q_d_b5, eq71_e1869_q_d_b6, eq71_e1869_q_d_b7, eq71_e1869_q_d_b8, eq71_e1869_q_d_b9, eq71_e1869_q_d_b10, eq71_e1869_q_d_b11, eq71_e1869_q_d_b12, eq71_e1869_q_d_b13, eq71_e1869_q_d_b14, eq71_e1869_q_d_b15, eq71_e1869_q_d_b16, eq71_e1869_q_d_b17,) = {
    if ((s.b[1559] && s.b[1560]) && s.b[1561]) {
        let eq71_e1856: f64 = (-s.v[885]);
        let eq71_e1856_d_n0: f64 = (-s.dn[885][0]);
        let eq71_e1856_d_n1: f64 = (-s.dn[885][1]);
        let eq71_e1856_d_n2: f64 = (-s.dn[885][2]);
        let eq71_e1856_d_n3: f64 = (-s.dn[885][3]);
        let eq71_e1856_d_n4: f64 = (-s.dn[885][4]);
        let eq71_e1856_d_n5: f64 = (-s.dn[885][5]);
        let eq71_e1856_d_n6: f64 = (-s.dn[885][6]);
        let eq71_e1856_d_n7: f64 = (-s.dn[885][7]);
        let eq71_e1856_d_n8: f64 = (-s.dn[885][8]);
        let eq71_e1856_d_n9: f64 = (-s.dn[885][9]);
        let eq71_e1856_d_n10: f64 = (-s.dn[885][10]);
        let eq71_e1856_d_n11: f64 = (-s.dn[885][11]);
        let eq71_e1856_d_n12: f64 = (-s.dn[885][12]);
        let eq71_e1856_d_n13: f64 = (-s.dn[885][13]);
        let eq71_e1856_d_b0: f64 = (-s.db[885][0]);
        let eq71_e1856_d_b1: f64 = (-s.db[885][1]);
        let eq71_e1856_d_b2: f64 = (-s.db[885][2]);
        let eq71_e1856_d_b3: f64 = (-s.db[885][3]);
        let eq71_e1856_d_b4: f64 = (-s.db[885][4]);
        let eq71_e1856_d_b5: f64 = (-s.db[885][5]);
        let eq71_e1856_d_b6: f64 = (-s.db[885][6]);
        let eq71_e1856_d_b7: f64 = (-s.db[885][7]);
        let eq71_e1856_d_b8: f64 = (-s.db[885][8]);
        let eq71_e1856_d_b9: f64 = (-s.db[885][9]);
        let eq71_e1856_d_b10: f64 = (-s.db[885][10]);
        let eq71_e1856_d_b11: f64 = (-s.db[885][11]);
        let eq71_e1856_d_b12: f64 = (-s.db[885][12]);
        let eq71_e1856_d_b13: f64 = (-s.db[885][13]);
        let eq71_e1856_d_b14: f64 = (-s.db[885][14]);
        let eq71_e1856_d_b15: f64 = (-s.db[885][15]);
        let eq71_e1856_d_b16: f64 = (-s.db[885][16]);
        let eq71_e1856_d_b17: f64 = (-s.db[885][17]);
        let eq71_e1858: f64 = (eq71_e1856 * s.v[822]);
        let eq71_e1858_d_n0: f64 = ((eq71_e1856_d_n0 * s.v[822]) + (eq71_e1856 * s.dn[822][0]));
        let eq71_e1858_d_n1: f64 = ((eq71_e1856_d_n1 * s.v[822]) + (eq71_e1856 * s.dn[822][1]));
        let eq71_e1858_d_n2: f64 = ((eq71_e1856_d_n2 * s.v[822]) + (eq71_e1856 * s.dn[822][2]));
        let eq71_e1858_d_n3: f64 = ((eq71_e1856_d_n3 * s.v[822]) + (eq71_e1856 * s.dn[822][3]));
        let eq71_e1858_d_n4: f64 = ((eq71_e1856_d_n4 * s.v[822]) + (eq71_e1856 * s.dn[822][4]));
        let eq71_e1858_d_n5: f64 = ((eq71_e1856_d_n5 * s.v[822]) + (eq71_e1856 * s.dn[822][5]));
        let eq71_e1858_d_n6: f64 = ((eq71_e1856_d_n6 * s.v[822]) + (eq71_e1856 * s.dn[822][6]));
        let eq71_e1858_d_n7: f64 = ((eq71_e1856_d_n7 * s.v[822]) + (eq71_e1856 * s.dn[822][7]));
        let eq71_e1858_d_n8: f64 = ((eq71_e1856_d_n8 * s.v[822]) + (eq71_e1856 * s.dn[822][8]));
        let eq71_e1858_d_n9: f64 = ((eq71_e1856_d_n9 * s.v[822]) + (eq71_e1856 * s.dn[822][9]));
        let eq71_e1858_d_n10: f64 = ((eq71_e1856_d_n10 * s.v[822]) + (eq71_e1856 * s.dn[822][10]));
        let eq71_e1858_d_n11: f64 = ((eq71_e1856_d_n11 * s.v[822]) + (eq71_e1856 * s.dn[822][11]));
        let eq71_e1858_d_n12: f64 = ((eq71_e1856_d_n12 * s.v[822]) + (eq71_e1856 * s.dn[822][12]));
        let eq71_e1858_d_n13: f64 = ((eq71_e1856_d_n13 * s.v[822]) + (eq71_e1856 * s.dn[822][13]));
        let eq71_e1858_d_b0: f64 = ((eq71_e1856_d_b0 * s.v[822]) + (eq71_e1856 * s.db[822][0]));
        let eq71_e1858_d_b1: f64 = ((eq71_e1856_d_b1 * s.v[822]) + (eq71_e1856 * s.db[822][1]));
        let eq71_e1858_d_b2: f64 = ((eq71_e1856_d_b2 * s.v[822]) + (eq71_e1856 * s.db[822][2]));
        let eq71_e1858_d_b3: f64 = ((eq71_e1856_d_b3 * s.v[822]) + (eq71_e1856 * s.db[822][3]));
        let eq71_e1858_d_b4: f64 = ((eq71_e1856_d_b4 * s.v[822]) + (eq71_e1856 * s.db[822][4]));
        let eq71_e1858_d_b5: f64 = ((eq71_e1856_d_b5 * s.v[822]) + (eq71_e1856 * s.db[822][5]));
        let eq71_e1858_d_b6: f64 = ((eq71_e1856_d_b6 * s.v[822]) + (eq71_e1856 * s.db[822][6]));
        let eq71_e1858_d_b7: f64 = ((eq71_e1856_d_b7 * s.v[822]) + (eq71_e1856 * s.db[822][7]));
        let eq71_e1858_d_b8: f64 = ((eq71_e1856_d_b8 * s.v[822]) + (eq71_e1856 * s.db[822][8]));
        let eq71_e1858_d_b9: f64 = ((eq71_e1856_d_b9 * s.v[822]) + (eq71_e1856 * s.db[822][9]));
        let eq71_e1858_d_b10: f64 = ((eq71_e1856_d_b10 * s.v[822]) + (eq71_e1856 * s.db[822][10]));
        let eq71_e1858_d_b11: f64 = ((eq71_e1856_d_b11 * s.v[822]) + (eq71_e1856 * s.db[822][11]));
        let eq71_e1858_d_b12: f64 = ((eq71_e1856_d_b12 * s.v[822]) + (eq71_e1856 * s.db[822][12]));
        let eq71_e1858_d_b13: f64 = ((eq71_e1856_d_b13 * s.v[822]) + (eq71_e1856 * s.db[822][13]));
        let eq71_e1858_d_b14: f64 = ((eq71_e1856_d_b14 * s.v[822]) + (eq71_e1856 * s.db[822][14]));
        let eq71_e1858_d_b15: f64 = ((eq71_e1856_d_b15 * s.v[822]) + (eq71_e1856 * s.db[822][15]));
        let eq71_e1858_d_b16: f64 = ((eq71_e1856_d_b16 * s.v[822]) + (eq71_e1856 * s.db[822][16]));
        let eq71_e1858_d_b17: f64 = ((eq71_e1856_d_b17 * s.v[822]) + (eq71_e1856 * s.db[822][17]));
        let eq71_e1861: f64 = (s.v[410] * s.v[158]);
        let eq71_e1861_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq71_e1861_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq71_e1861_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq71_e1861_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq71_e1861_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq71_e1861_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq71_e1861_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq71_e1861_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq71_e1861_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq71_e1861_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq71_e1861_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq71_e1861_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq71_e1861_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq71_e1861_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq71_e1861_d_b0: f64 = (s.db[410][0] * s.v[158]);
        let eq71_e1861_d_b1: f64 = (s.db[410][1] * s.v[158]);
        let eq71_e1861_d_b2: f64 = (s.db[410][2] * s.v[158]);
        let eq71_e1861_d_b3: f64 = (s.db[410][3] * s.v[158]);
        let eq71_e1861_d_b4: f64 = (s.db[410][4] * s.v[158]);
        let eq71_e1861_d_b5: f64 = (s.db[410][5] * s.v[158]);
        let eq71_e1861_d_b6: f64 = (s.db[410][6] * s.v[158]);
        let eq71_e1861_d_b7: f64 = (s.db[410][7] * s.v[158]);
        let eq71_e1861_d_b8: f64 = (s.db[410][8] * s.v[158]);
        let eq71_e1861_d_b9: f64 = (s.db[410][9] * s.v[158]);
        let eq71_e1861_d_b10: f64 = (s.db[410][10] * s.v[158]);
        let eq71_e1861_d_b11: f64 = (s.db[410][11] * s.v[158]);
        let eq71_e1861_d_b12: f64 = (s.db[410][12] * s.v[158]);
        let eq71_e1861_d_b13: f64 = (s.db[410][13] * s.v[158]);
        let eq71_e1861_d_b14: f64 = (s.db[410][14] * s.v[158]);
        let eq71_e1861_d_b15: f64 = (s.db[410][15] * s.v[158]);
        let eq71_e1861_d_b16: f64 = (s.db[410][16] * s.v[158]);
        let eq71_e1861_d_b17: f64 = (s.db[410][17] * s.v[158]);
        let eq71_e1862_q: f64 = eq71_e1861;
        let eq71_e1863: f64 = (eq71_e1858 + eq71_e1861);
        let eq71_e1863_d_n0: f64 = (eq71_e1858_d_n0 + eq71_e1861_d_n0);
        let eq71_e1863_d_n1: f64 = (eq71_e1858_d_n1 + eq71_e1861_d_n1);
        let eq71_e1863_d_n2: f64 = (eq71_e1858_d_n2 + eq71_e1861_d_n2);
        let eq71_e1863_d_n3: f64 = (eq71_e1858_d_n3 + eq71_e1861_d_n3);
        let eq71_e1863_d_n4: f64 = (eq71_e1858_d_n4 + eq71_e1861_d_n4);
        let eq71_e1863_d_n5: f64 = (eq71_e1858_d_n5 + eq71_e1861_d_n5);
        let eq71_e1863_d_n6: f64 = (eq71_e1858_d_n6 + eq71_e1861_d_n6);
        let eq71_e1863_d_n7: f64 = (eq71_e1858_d_n7 + eq71_e1861_d_n7);
        let eq71_e1863_d_n8: f64 = (eq71_e1858_d_n8 + eq71_e1861_d_n8);
        let eq71_e1863_d_n9: f64 = (eq71_e1858_d_n9 + eq71_e1861_d_n9);
        let eq71_e1863_d_n10: f64 = (eq71_e1858_d_n10 + eq71_e1861_d_n10);
        let eq71_e1863_d_n11: f64 = (eq71_e1858_d_n11 + eq71_e1861_d_n11);
        let eq71_e1863_d_n12: f64 = (eq71_e1858_d_n12 + eq71_e1861_d_n12);
        let eq71_e1863_d_n13: f64 = (eq71_e1858_d_n13 + eq71_e1861_d_n13);
        let eq71_e1863_d_b0: f64 = (eq71_e1858_d_b0 + eq71_e1861_d_b0);
        let eq71_e1863_d_b1: f64 = (eq71_e1858_d_b1 + eq71_e1861_d_b1);
        let eq71_e1863_d_b2: f64 = (eq71_e1858_d_b2 + eq71_e1861_d_b2);
        let eq71_e1863_d_b3: f64 = (eq71_e1858_d_b3 + eq71_e1861_d_b3);
        let eq71_e1863_d_b4: f64 = (eq71_e1858_d_b4 + eq71_e1861_d_b4);
        let eq71_e1863_d_b5: f64 = (eq71_e1858_d_b5 + eq71_e1861_d_b5);
        let eq71_e1863_d_b6: f64 = (eq71_e1858_d_b6 + eq71_e1861_d_b6);
        let eq71_e1863_d_b7: f64 = (eq71_e1858_d_b7 + eq71_e1861_d_b7);
        let eq71_e1863_d_b8: f64 = (eq71_e1858_d_b8 + eq71_e1861_d_b8);
        let eq71_e1863_d_b9: f64 = (eq71_e1858_d_b9 + eq71_e1861_d_b9);
        let eq71_e1863_d_b10: f64 = (eq71_e1858_d_b10 + eq71_e1861_d_b10);
        let eq71_e1863_d_b11: f64 = (eq71_e1858_d_b11 + eq71_e1861_d_b11);
        let eq71_e1863_d_b12: f64 = (eq71_e1858_d_b12 + eq71_e1861_d_b12);
        let eq71_e1863_d_b13: f64 = (eq71_e1858_d_b13 + eq71_e1861_d_b13);
        let eq71_e1863_d_b14: f64 = (eq71_e1858_d_b14 + eq71_e1861_d_b14);
        let eq71_e1863_d_b15: f64 = (eq71_e1858_d_b15 + eq71_e1861_d_b15);
        let eq71_e1863_d_b16: f64 = (eq71_e1858_d_b16 + eq71_e1861_d_b16);
        let eq71_e1863_d_b17: f64 = (eq71_e1858_d_b17 + eq71_e1861_d_b17);
        let eq71_e1863_q: f64 = eq71_e1862_q;
        let eq71_e1866: f64 = (s.v[410] / s.v[157]);
        let eq71_e1866_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq71_e1866_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq71_e1866_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq71_e1866_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq71_e1866_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq71_e1866_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq71_e1866_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq71_e1866_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq71_e1866_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq71_e1866_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq71_e1866_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq71_e1866_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq71_e1866_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq71_e1866_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq71_e1866_d_b0: f64 = (s.db[410][0] / s.v[157]);
        let eq71_e1866_d_b1: f64 = (s.db[410][1] / s.v[157]);
        let eq71_e1866_d_b2: f64 = (s.db[410][2] / s.v[157]);
        let eq71_e1866_d_b3: f64 = (s.db[410][3] / s.v[157]);
        let eq71_e1866_d_b4: f64 = (s.db[410][4] / s.v[157]);
        let eq71_e1866_d_b5: f64 = (s.db[410][5] / s.v[157]);
        let eq71_e1866_d_b6: f64 = (s.db[410][6] / s.v[157]);
        let eq71_e1866_d_b7: f64 = (s.db[410][7] / s.v[157]);
        let eq71_e1866_d_b8: f64 = (s.db[410][8] / s.v[157]);
        let eq71_e1866_d_b9: f64 = (s.db[410][9] / s.v[157]);
        let eq71_e1866_d_b10: f64 = (s.db[410][10] / s.v[157]);
        let eq71_e1866_d_b11: f64 = (s.db[410][11] / s.v[157]);
        let eq71_e1866_d_b12: f64 = (s.db[410][12] / s.v[157]);
        let eq71_e1866_d_b13: f64 = (s.db[410][13] / s.v[157]);
        let eq71_e1866_d_b14: f64 = (s.db[410][14] / s.v[157]);
        let eq71_e1866_d_b15: f64 = (s.db[410][15] / s.v[157]);
        let eq71_e1866_d_b16: f64 = (s.db[410][16] / s.v[157]);
        let eq71_e1866_d_b17: f64 = (s.db[410][17] / s.v[157]);
        let eq71_e1867: f64 = (eq71_e1863 + eq71_e1866);
        let eq71_e1867_d_n0: f64 = (eq71_e1863_d_n0 + eq71_e1866_d_n0);
        let eq71_e1867_d_n1: f64 = (eq71_e1863_d_n1 + eq71_e1866_d_n1);
        let eq71_e1867_d_n2: f64 = (eq71_e1863_d_n2 + eq71_e1866_d_n2);
        let eq71_e1867_d_n3: f64 = (eq71_e1863_d_n3 + eq71_e1866_d_n3);
        let eq71_e1867_d_n4: f64 = (eq71_e1863_d_n4 + eq71_e1866_d_n4);
        let eq71_e1867_d_n5: f64 = (eq71_e1863_d_n5 + eq71_e1866_d_n5);
        let eq71_e1867_d_n6: f64 = (eq71_e1863_d_n6 + eq71_e1866_d_n6);
        let eq71_e1867_d_n7: f64 = (eq71_e1863_d_n7 + eq71_e1866_d_n7);
        let eq71_e1867_d_n8: f64 = (eq71_e1863_d_n8 + eq71_e1866_d_n8);
        let eq71_e1867_d_n9: f64 = (eq71_e1863_d_n9 + eq71_e1866_d_n9);
        let eq71_e1867_d_n10: f64 = (eq71_e1863_d_n10 + eq71_e1866_d_n10);
        let eq71_e1867_d_n11: f64 = (eq71_e1863_d_n11 + eq71_e1866_d_n11);
        let eq71_e1867_d_n12: f64 = (eq71_e1863_d_n12 + eq71_e1866_d_n12);
        let eq71_e1867_d_n13: f64 = (eq71_e1863_d_n13 + eq71_e1866_d_n13);
        let eq71_e1867_d_b0: f64 = (eq71_e1863_d_b0 + eq71_e1866_d_b0);
        let eq71_e1867_d_b1: f64 = (eq71_e1863_d_b1 + eq71_e1866_d_b1);
        let eq71_e1867_d_b2: f64 = (eq71_e1863_d_b2 + eq71_e1866_d_b2);
        let eq71_e1867_d_b3: f64 = (eq71_e1863_d_b3 + eq71_e1866_d_b3);
        let eq71_e1867_d_b4: f64 = (eq71_e1863_d_b4 + eq71_e1866_d_b4);
        let eq71_e1867_d_b5: f64 = (eq71_e1863_d_b5 + eq71_e1866_d_b5);
        let eq71_e1867_d_b6: f64 = (eq71_e1863_d_b6 + eq71_e1866_d_b6);
        let eq71_e1867_d_b7: f64 = (eq71_e1863_d_b7 + eq71_e1866_d_b7);
        let eq71_e1867_d_b8: f64 = (eq71_e1863_d_b8 + eq71_e1866_d_b8);
        let eq71_e1867_d_b9: f64 = (eq71_e1863_d_b9 + eq71_e1866_d_b9);
        let eq71_e1867_d_b10: f64 = (eq71_e1863_d_b10 + eq71_e1866_d_b10);
        let eq71_e1867_d_b11: f64 = (eq71_e1863_d_b11 + eq71_e1866_d_b11);
        let eq71_e1867_d_b12: f64 = (eq71_e1863_d_b12 + eq71_e1866_d_b12);
        let eq71_e1867_d_b13: f64 = (eq71_e1863_d_b13 + eq71_e1866_d_b13);
        let eq71_e1867_d_b14: f64 = (eq71_e1863_d_b14 + eq71_e1866_d_b14);
        let eq71_e1867_d_b15: f64 = (eq71_e1863_d_b15 + eq71_e1866_d_b15);
        let eq71_e1867_d_b16: f64 = (eq71_e1863_d_b16 + eq71_e1866_d_b16);
        let eq71_e1867_d_b17: f64 = (eq71_e1863_d_b17 + eq71_e1866_d_b17);
        let eq71_e1867_q: f64 = eq71_e1863_q;
        (eq71_e1867, eq71_e1867_d_n0, eq71_e1867_d_n1, eq71_e1867_d_n2, eq71_e1867_d_n3, eq71_e1867_d_n4, eq71_e1867_d_n5, eq71_e1867_d_n6, eq71_e1867_d_n7, eq71_e1867_d_n8, eq71_e1867_d_n9, eq71_e1867_d_n10, eq71_e1867_d_n11, eq71_e1867_d_n12, eq71_e1867_d_n13, eq71_e1867_d_b0, eq71_e1867_d_b1, eq71_e1867_d_b2, eq71_e1867_d_b3, eq71_e1867_d_b4, eq71_e1867_d_b5, eq71_e1867_d_b6, eq71_e1867_d_b7, eq71_e1867_d_b8, eq71_e1867_d_b9, eq71_e1867_d_b10, eq71_e1867_d_b11, eq71_e1867_d_b12, eq71_e1867_d_b13, eq71_e1867_d_b14, eq71_e1867_d_b15, eq71_e1867_d_b16, eq71_e1867_d_b17, eq71_e1867_q, eq71_e1861_d_n0, eq71_e1861_d_n1, eq71_e1861_d_n2, eq71_e1861_d_n3, eq71_e1861_d_n4, eq71_e1861_d_n5, eq71_e1861_d_n6, eq71_e1861_d_n7, eq71_e1861_d_n8, eq71_e1861_d_n9, eq71_e1861_d_n10, eq71_e1861_d_n11, eq71_e1861_d_n12, eq71_e1861_d_n13, eq71_e1861_d_b0, eq71_e1861_d_b1, eq71_e1861_d_b2, eq71_e1861_d_b3, eq71_e1861_d_b4, eq71_e1861_d_b5, eq71_e1861_d_b6, eq71_e1861_d_b7, eq71_e1861_d_b8, eq71_e1861_d_b9, eq71_e1861_d_b10, eq71_e1861_d_b11, eq71_e1861_d_b12, eq71_e1861_d_b13, eq71_e1861_d_b14, eq71_e1861_d_b15, eq71_e1861_d_b16, eq71_e1861_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 14] = [eq71_e1869_q_d_n0, eq71_e1869_q_d_n1, eq71_e1869_q_d_n2, eq71_e1869_q_d_n3, eq71_e1869_q_d_n4, eq71_e1869_q_d_n5, eq71_e1869_q_d_n6, eq71_e1869_q_d_n7, eq71_e1869_q_d_n8, eq71_e1869_q_d_n9, eq71_e1869_q_d_n10, eq71_e1869_q_d_n11, eq71_e1869_q_d_n12, eq71_e1869_q_d_n13];
        let eq71_reactive_branch_derivatives: [f64; 18] = [eq71_e1869_q_d_b0, eq71_e1869_q_d_b1, eq71_e1869_q_d_b2, eq71_e1869_q_d_b3, eq71_e1869_q_d_b4, eq71_e1869_q_d_b5, eq71_e1869_q_d_b6, eq71_e1869_q_d_b7, eq71_e1869_q_d_b8, eq71_e1869_q_d_b9, eq71_e1869_q_d_b10, eq71_e1869_q_d_b11, eq71_e1869_q_d_b12, eq71_e1869_q_d_b13, eq71_e1869_q_d_b14, eq71_e1869_q_d_b15, eq71_e1869_q_d_b16, eq71_e1869_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq71_reactive_node_derivatives,
            branches,
            &eq71_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq72_e1892, eq72_e1892_d_n0, eq72_e1892_d_n1, eq72_e1892_d_n2, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12, eq72_e1892_d_n13, eq72_e1892_d_b0, eq72_e1892_d_b1, eq72_e1892_d_b2, eq72_e1892_d_b3, eq72_e1892_d_b4, eq72_e1892_d_b5, eq72_e1892_d_b6, eq72_e1892_d_b7, eq72_e1892_d_b8, eq72_e1892_d_b9, eq72_e1892_d_b10, eq72_e1892_d_b11, eq72_e1892_d_b12, eq72_e1892_d_b13, eq72_e1892_d_b14, eq72_e1892_d_b15, eq72_e1892_d_b16, eq72_e1892_d_b17, eq72_e1892_q, eq72_e1892_q_d_n0, eq72_e1892_q_d_n1, eq72_e1892_q_d_n2, eq72_e1892_q_d_n3, eq72_e1892_q_d_n4, eq72_e1892_q_d_n5, eq72_e1892_q_d_n6, eq72_e1892_q_d_n7, eq72_e1892_q_d_n8, eq72_e1892_q_d_n9, eq72_e1892_q_d_n10, eq72_e1892_q_d_n11, eq72_e1892_q_d_n12, eq72_e1892_q_d_n13, eq72_e1892_q_d_b0, eq72_e1892_q_d_b1, eq72_e1892_q_d_b2, eq72_e1892_q_d_b3, eq72_e1892_q_d_b4, eq72_e1892_q_d_b5, eq72_e1892_q_d_b6, eq72_e1892_q_d_b7, eq72_e1892_q_d_b8, eq72_e1892_q_d_b9, eq72_e1892_q_d_b10, eq72_e1892_q_d_b11, eq72_e1892_q_d_b12, eq72_e1892_q_d_b13, eq72_e1892_q_d_b14, eq72_e1892_q_d_b15, eq72_e1892_q_d_b16, eq72_e1892_q_d_b17,) = {
    if (((s.b[1559] && s.b[1560]) && (!s.b[1561])) && s.b[1562]) {
        let eq72_e1879: f64 = (-s.v[885]);
        let eq72_e1879_d_n0: f64 = (-s.dn[885][0]);
        let eq72_e1879_d_n1: f64 = (-s.dn[885][1]);
        let eq72_e1879_d_n2: f64 = (-s.dn[885][2]);
        let eq72_e1879_d_n3: f64 = (-s.dn[885][3]);
        let eq72_e1879_d_n4: f64 = (-s.dn[885][4]);
        let eq72_e1879_d_n5: f64 = (-s.dn[885][5]);
        let eq72_e1879_d_n6: f64 = (-s.dn[885][6]);
        let eq72_e1879_d_n7: f64 = (-s.dn[885][7]);
        let eq72_e1879_d_n8: f64 = (-s.dn[885][8]);
        let eq72_e1879_d_n9: f64 = (-s.dn[885][9]);
        let eq72_e1879_d_n10: f64 = (-s.dn[885][10]);
        let eq72_e1879_d_n11: f64 = (-s.dn[885][11]);
        let eq72_e1879_d_n12: f64 = (-s.dn[885][12]);
        let eq72_e1879_d_n13: f64 = (-s.dn[885][13]);
        let eq72_e1879_d_b0: f64 = (-s.db[885][0]);
        let eq72_e1879_d_b1: f64 = (-s.db[885][1]);
        let eq72_e1879_d_b2: f64 = (-s.db[885][2]);
        let eq72_e1879_d_b3: f64 = (-s.db[885][3]);
        let eq72_e1879_d_b4: f64 = (-s.db[885][4]);
        let eq72_e1879_d_b5: f64 = (-s.db[885][5]);
        let eq72_e1879_d_b6: f64 = (-s.db[885][6]);
        let eq72_e1879_d_b7: f64 = (-s.db[885][7]);
        let eq72_e1879_d_b8: f64 = (-s.db[885][8]);
        let eq72_e1879_d_b9: f64 = (-s.db[885][9]);
        let eq72_e1879_d_b10: f64 = (-s.db[885][10]);
        let eq72_e1879_d_b11: f64 = (-s.db[885][11]);
        let eq72_e1879_d_b12: f64 = (-s.db[885][12]);
        let eq72_e1879_d_b13: f64 = (-s.db[885][13]);
        let eq72_e1879_d_b14: f64 = (-s.db[885][14]);
        let eq72_e1879_d_b15: f64 = (-s.db[885][15]);
        let eq72_e1879_d_b16: f64 = (-s.db[885][16]);
        let eq72_e1879_d_b17: f64 = (-s.db[885][17]);
        let eq72_e1881: f64 = (eq72_e1879 * s.v[822]);
        let eq72_e1881_d_n0: f64 = ((eq72_e1879_d_n0 * s.v[822]) + (eq72_e1879 * s.dn[822][0]));
        let eq72_e1881_d_n1: f64 = ((eq72_e1879_d_n1 * s.v[822]) + (eq72_e1879 * s.dn[822][1]));
        let eq72_e1881_d_n2: f64 = ((eq72_e1879_d_n2 * s.v[822]) + (eq72_e1879 * s.dn[822][2]));
        let eq72_e1881_d_n3: f64 = ((eq72_e1879_d_n3 * s.v[822]) + (eq72_e1879 * s.dn[822][3]));
        let eq72_e1881_d_n4: f64 = ((eq72_e1879_d_n4 * s.v[822]) + (eq72_e1879 * s.dn[822][4]));
        let eq72_e1881_d_n5: f64 = ((eq72_e1879_d_n5 * s.v[822]) + (eq72_e1879 * s.dn[822][5]));
        let eq72_e1881_d_n6: f64 = ((eq72_e1879_d_n6 * s.v[822]) + (eq72_e1879 * s.dn[822][6]));
        let eq72_e1881_d_n7: f64 = ((eq72_e1879_d_n7 * s.v[822]) + (eq72_e1879 * s.dn[822][7]));
        let eq72_e1881_d_n8: f64 = ((eq72_e1879_d_n8 * s.v[822]) + (eq72_e1879 * s.dn[822][8]));
        let eq72_e1881_d_n9: f64 = ((eq72_e1879_d_n9 * s.v[822]) + (eq72_e1879 * s.dn[822][9]));
        let eq72_e1881_d_n10: f64 = ((eq72_e1879_d_n10 * s.v[822]) + (eq72_e1879 * s.dn[822][10]));
        let eq72_e1881_d_n11: f64 = ((eq72_e1879_d_n11 * s.v[822]) + (eq72_e1879 * s.dn[822][11]));
        let eq72_e1881_d_n12: f64 = ((eq72_e1879_d_n12 * s.v[822]) + (eq72_e1879 * s.dn[822][12]));
        let eq72_e1881_d_n13: f64 = ((eq72_e1879_d_n13 * s.v[822]) + (eq72_e1879 * s.dn[822][13]));
        let eq72_e1881_d_b0: f64 = ((eq72_e1879_d_b0 * s.v[822]) + (eq72_e1879 * s.db[822][0]));
        let eq72_e1881_d_b1: f64 = ((eq72_e1879_d_b1 * s.v[822]) + (eq72_e1879 * s.db[822][1]));
        let eq72_e1881_d_b2: f64 = ((eq72_e1879_d_b2 * s.v[822]) + (eq72_e1879 * s.db[822][2]));
        let eq72_e1881_d_b3: f64 = ((eq72_e1879_d_b3 * s.v[822]) + (eq72_e1879 * s.db[822][3]));
        let eq72_e1881_d_b4: f64 = ((eq72_e1879_d_b4 * s.v[822]) + (eq72_e1879 * s.db[822][4]));
        let eq72_e1881_d_b5: f64 = ((eq72_e1879_d_b5 * s.v[822]) + (eq72_e1879 * s.db[822][5]));
        let eq72_e1881_d_b6: f64 = ((eq72_e1879_d_b6 * s.v[822]) + (eq72_e1879 * s.db[822][6]));
        let eq72_e1881_d_b7: f64 = ((eq72_e1879_d_b7 * s.v[822]) + (eq72_e1879 * s.db[822][7]));
        let eq72_e1881_d_b8: f64 = ((eq72_e1879_d_b8 * s.v[822]) + (eq72_e1879 * s.db[822][8]));
        let eq72_e1881_d_b9: f64 = ((eq72_e1879_d_b9 * s.v[822]) + (eq72_e1879 * s.db[822][9]));
        let eq72_e1881_d_b10: f64 = ((eq72_e1879_d_b10 * s.v[822]) + (eq72_e1879 * s.db[822][10]));
        let eq72_e1881_d_b11: f64 = ((eq72_e1879_d_b11 * s.v[822]) + (eq72_e1879 * s.db[822][11]));
        let eq72_e1881_d_b12: f64 = ((eq72_e1879_d_b12 * s.v[822]) + (eq72_e1879 * s.db[822][12]));
        let eq72_e1881_d_b13: f64 = ((eq72_e1879_d_b13 * s.v[822]) + (eq72_e1879 * s.db[822][13]));
        let eq72_e1881_d_b14: f64 = ((eq72_e1879_d_b14 * s.v[822]) + (eq72_e1879 * s.db[822][14]));
        let eq72_e1881_d_b15: f64 = ((eq72_e1879_d_b15 * s.v[822]) + (eq72_e1879 * s.db[822][15]));
        let eq72_e1881_d_b16: f64 = ((eq72_e1879_d_b16 * s.v[822]) + (eq72_e1879 * s.db[822][16]));
        let eq72_e1881_d_b17: f64 = ((eq72_e1879_d_b17 * s.v[822]) + (eq72_e1879 * s.db[822][17]));
        let eq72_e1884: f64 = (s.v[410] * s.v[158]);
        let eq72_e1884_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq72_e1884_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq72_e1884_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq72_e1884_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq72_e1884_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq72_e1884_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq72_e1884_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq72_e1884_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq72_e1884_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq72_e1884_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq72_e1884_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq72_e1884_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq72_e1884_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq72_e1884_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq72_e1884_d_b0: f64 = (s.db[410][0] * s.v[158]);
        let eq72_e1884_d_b1: f64 = (s.db[410][1] * s.v[158]);
        let eq72_e1884_d_b2: f64 = (s.db[410][2] * s.v[158]);
        let eq72_e1884_d_b3: f64 = (s.db[410][3] * s.v[158]);
        let eq72_e1884_d_b4: f64 = (s.db[410][4] * s.v[158]);
        let eq72_e1884_d_b5: f64 = (s.db[410][5] * s.v[158]);
        let eq72_e1884_d_b6: f64 = (s.db[410][6] * s.v[158]);
        let eq72_e1884_d_b7: f64 = (s.db[410][7] * s.v[158]);
        let eq72_e1884_d_b8: f64 = (s.db[410][8] * s.v[158]);
        let eq72_e1884_d_b9: f64 = (s.db[410][9] * s.v[158]);
        let eq72_e1884_d_b10: f64 = (s.db[410][10] * s.v[158]);
        let eq72_e1884_d_b11: f64 = (s.db[410][11] * s.v[158]);
        let eq72_e1884_d_b12: f64 = (s.db[410][12] * s.v[158]);
        let eq72_e1884_d_b13: f64 = (s.db[410][13] * s.v[158]);
        let eq72_e1884_d_b14: f64 = (s.db[410][14] * s.v[158]);
        let eq72_e1884_d_b15: f64 = (s.db[410][15] * s.v[158]);
        let eq72_e1884_d_b16: f64 = (s.db[410][16] * s.v[158]);
        let eq72_e1884_d_b17: f64 = (s.db[410][17] * s.v[158]);
        let eq72_e1885_q: f64 = eq72_e1884;
        let eq72_e1886: f64 = (eq72_e1881 + eq72_e1884);
        let eq72_e1886_d_n0: f64 = (eq72_e1881_d_n0 + eq72_e1884_d_n0);
        let eq72_e1886_d_n1: f64 = (eq72_e1881_d_n1 + eq72_e1884_d_n1);
        let eq72_e1886_d_n2: f64 = (eq72_e1881_d_n2 + eq72_e1884_d_n2);
        let eq72_e1886_d_n3: f64 = (eq72_e1881_d_n3 + eq72_e1884_d_n3);
        let eq72_e1886_d_n4: f64 = (eq72_e1881_d_n4 + eq72_e1884_d_n4);
        let eq72_e1886_d_n5: f64 = (eq72_e1881_d_n5 + eq72_e1884_d_n5);
        let eq72_e1886_d_n6: f64 = (eq72_e1881_d_n6 + eq72_e1884_d_n6);
        let eq72_e1886_d_n7: f64 = (eq72_e1881_d_n7 + eq72_e1884_d_n7);
        let eq72_e1886_d_n8: f64 = (eq72_e1881_d_n8 + eq72_e1884_d_n8);
        let eq72_e1886_d_n9: f64 = (eq72_e1881_d_n9 + eq72_e1884_d_n9);
        let eq72_e1886_d_n10: f64 = (eq72_e1881_d_n10 + eq72_e1884_d_n10);
        let eq72_e1886_d_n11: f64 = (eq72_e1881_d_n11 + eq72_e1884_d_n11);
        let eq72_e1886_d_n12: f64 = (eq72_e1881_d_n12 + eq72_e1884_d_n12);
        let eq72_e1886_d_n13: f64 = (eq72_e1881_d_n13 + eq72_e1884_d_n13);
        let eq72_e1886_d_b0: f64 = (eq72_e1881_d_b0 + eq72_e1884_d_b0);
        let eq72_e1886_d_b1: f64 = (eq72_e1881_d_b1 + eq72_e1884_d_b1);
        let eq72_e1886_d_b2: f64 = (eq72_e1881_d_b2 + eq72_e1884_d_b2);
        let eq72_e1886_d_b3: f64 = (eq72_e1881_d_b3 + eq72_e1884_d_b3);
        let eq72_e1886_d_b4: f64 = (eq72_e1881_d_b4 + eq72_e1884_d_b4);
        let eq72_e1886_d_b5: f64 = (eq72_e1881_d_b5 + eq72_e1884_d_b5);
        let eq72_e1886_d_b6: f64 = (eq72_e1881_d_b6 + eq72_e1884_d_b6);
        let eq72_e1886_d_b7: f64 = (eq72_e1881_d_b7 + eq72_e1884_d_b7);
        let eq72_e1886_d_b8: f64 = (eq72_e1881_d_b8 + eq72_e1884_d_b8);
        let eq72_e1886_d_b9: f64 = (eq72_e1881_d_b9 + eq72_e1884_d_b9);
        let eq72_e1886_d_b10: f64 = (eq72_e1881_d_b10 + eq72_e1884_d_b10);
        let eq72_e1886_d_b11: f64 = (eq72_e1881_d_b11 + eq72_e1884_d_b11);
        let eq72_e1886_d_b12: f64 = (eq72_e1881_d_b12 + eq72_e1884_d_b12);
        let eq72_e1886_d_b13: f64 = (eq72_e1881_d_b13 + eq72_e1884_d_b13);
        let eq72_e1886_d_b14: f64 = (eq72_e1881_d_b14 + eq72_e1884_d_b14);
        let eq72_e1886_d_b15: f64 = (eq72_e1881_d_b15 + eq72_e1884_d_b15);
        let eq72_e1886_d_b16: f64 = (eq72_e1881_d_b16 + eq72_e1884_d_b16);
        let eq72_e1886_d_b17: f64 = (eq72_e1881_d_b17 + eq72_e1884_d_b17);
        let eq72_e1886_q: f64 = eq72_e1885_q;
        let eq72_e1889: f64 = (s.v[410] / s.v[157]);
        let eq72_e1889_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq72_e1889_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq72_e1889_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq72_e1889_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq72_e1889_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq72_e1889_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq72_e1889_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq72_e1889_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq72_e1889_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq72_e1889_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq72_e1889_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq72_e1889_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq72_e1889_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq72_e1889_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq72_e1889_d_b0: f64 = (s.db[410][0] / s.v[157]);
        let eq72_e1889_d_b1: f64 = (s.db[410][1] / s.v[157]);
        let eq72_e1889_d_b2: f64 = (s.db[410][2] / s.v[157]);
        let eq72_e1889_d_b3: f64 = (s.db[410][3] / s.v[157]);
        let eq72_e1889_d_b4: f64 = (s.db[410][4] / s.v[157]);
        let eq72_e1889_d_b5: f64 = (s.db[410][5] / s.v[157]);
        let eq72_e1889_d_b6: f64 = (s.db[410][6] / s.v[157]);
        let eq72_e1889_d_b7: f64 = (s.db[410][7] / s.v[157]);
        let eq72_e1889_d_b8: f64 = (s.db[410][8] / s.v[157]);
        let eq72_e1889_d_b9: f64 = (s.db[410][9] / s.v[157]);
        let eq72_e1889_d_b10: f64 = (s.db[410][10] / s.v[157]);
        let eq72_e1889_d_b11: f64 = (s.db[410][11] / s.v[157]);
        let eq72_e1889_d_b12: f64 = (s.db[410][12] / s.v[157]);
        let eq72_e1889_d_b13: f64 = (s.db[410][13] / s.v[157]);
        let eq72_e1889_d_b14: f64 = (s.db[410][14] / s.v[157]);
        let eq72_e1889_d_b15: f64 = (s.db[410][15] / s.v[157]);
        let eq72_e1889_d_b16: f64 = (s.db[410][16] / s.v[157]);
        let eq72_e1889_d_b17: f64 = (s.db[410][17] / s.v[157]);
        let eq72_e1890: f64 = (eq72_e1886 + eq72_e1889);
        let eq72_e1890_d_n0: f64 = (eq72_e1886_d_n0 + eq72_e1889_d_n0);
        let eq72_e1890_d_n1: f64 = (eq72_e1886_d_n1 + eq72_e1889_d_n1);
        let eq72_e1890_d_n2: f64 = (eq72_e1886_d_n2 + eq72_e1889_d_n2);
        let eq72_e1890_d_n3: f64 = (eq72_e1886_d_n3 + eq72_e1889_d_n3);
        let eq72_e1890_d_n4: f64 = (eq72_e1886_d_n4 + eq72_e1889_d_n4);
        let eq72_e1890_d_n5: f64 = (eq72_e1886_d_n5 + eq72_e1889_d_n5);
        let eq72_e1890_d_n6: f64 = (eq72_e1886_d_n6 + eq72_e1889_d_n6);
        let eq72_e1890_d_n7: f64 = (eq72_e1886_d_n7 + eq72_e1889_d_n7);
        let eq72_e1890_d_n8: f64 = (eq72_e1886_d_n8 + eq72_e1889_d_n8);
        let eq72_e1890_d_n9: f64 = (eq72_e1886_d_n9 + eq72_e1889_d_n9);
        let eq72_e1890_d_n10: f64 = (eq72_e1886_d_n10 + eq72_e1889_d_n10);
        let eq72_e1890_d_n11: f64 = (eq72_e1886_d_n11 + eq72_e1889_d_n11);
        let eq72_e1890_d_n12: f64 = (eq72_e1886_d_n12 + eq72_e1889_d_n12);
        let eq72_e1890_d_n13: f64 = (eq72_e1886_d_n13 + eq72_e1889_d_n13);
        let eq72_e1890_d_b0: f64 = (eq72_e1886_d_b0 + eq72_e1889_d_b0);
        let eq72_e1890_d_b1: f64 = (eq72_e1886_d_b1 + eq72_e1889_d_b1);
        let eq72_e1890_d_b2: f64 = (eq72_e1886_d_b2 + eq72_e1889_d_b2);
        let eq72_e1890_d_b3: f64 = (eq72_e1886_d_b3 + eq72_e1889_d_b3);
        let eq72_e1890_d_b4: f64 = (eq72_e1886_d_b4 + eq72_e1889_d_b4);
        let eq72_e1890_d_b5: f64 = (eq72_e1886_d_b5 + eq72_e1889_d_b5);
        let eq72_e1890_d_b6: f64 = (eq72_e1886_d_b6 + eq72_e1889_d_b6);
        let eq72_e1890_d_b7: f64 = (eq72_e1886_d_b7 + eq72_e1889_d_b7);
        let eq72_e1890_d_b8: f64 = (eq72_e1886_d_b8 + eq72_e1889_d_b8);
        let eq72_e1890_d_b9: f64 = (eq72_e1886_d_b9 + eq72_e1889_d_b9);
        let eq72_e1890_d_b10: f64 = (eq72_e1886_d_b10 + eq72_e1889_d_b10);
        let eq72_e1890_d_b11: f64 = (eq72_e1886_d_b11 + eq72_e1889_d_b11);
        let eq72_e1890_d_b12: f64 = (eq72_e1886_d_b12 + eq72_e1889_d_b12);
        let eq72_e1890_d_b13: f64 = (eq72_e1886_d_b13 + eq72_e1889_d_b13);
        let eq72_e1890_d_b14: f64 = (eq72_e1886_d_b14 + eq72_e1889_d_b14);
        let eq72_e1890_d_b15: f64 = (eq72_e1886_d_b15 + eq72_e1889_d_b15);
        let eq72_e1890_d_b16: f64 = (eq72_e1886_d_b16 + eq72_e1889_d_b16);
        let eq72_e1890_d_b17: f64 = (eq72_e1886_d_b17 + eq72_e1889_d_b17);
        let eq72_e1890_q: f64 = eq72_e1886_q;
        (eq72_e1890, eq72_e1890_d_n0, eq72_e1890_d_n1, eq72_e1890_d_n2, eq72_e1890_d_n3, eq72_e1890_d_n4, eq72_e1890_d_n5, eq72_e1890_d_n6, eq72_e1890_d_n7, eq72_e1890_d_n8, eq72_e1890_d_n9, eq72_e1890_d_n10, eq72_e1890_d_n11, eq72_e1890_d_n12, eq72_e1890_d_n13, eq72_e1890_d_b0, eq72_e1890_d_b1, eq72_e1890_d_b2, eq72_e1890_d_b3, eq72_e1890_d_b4, eq72_e1890_d_b5, eq72_e1890_d_b6, eq72_e1890_d_b7, eq72_e1890_d_b8, eq72_e1890_d_b9, eq72_e1890_d_b10, eq72_e1890_d_b11, eq72_e1890_d_b12, eq72_e1890_d_b13, eq72_e1890_d_b14, eq72_e1890_d_b15, eq72_e1890_d_b16, eq72_e1890_d_b17, eq72_e1890_q, eq72_e1884_d_n0, eq72_e1884_d_n1, eq72_e1884_d_n2, eq72_e1884_d_n3, eq72_e1884_d_n4, eq72_e1884_d_n5, eq72_e1884_d_n6, eq72_e1884_d_n7, eq72_e1884_d_n8, eq72_e1884_d_n9, eq72_e1884_d_n10, eq72_e1884_d_n11, eq72_e1884_d_n12, eq72_e1884_d_n13, eq72_e1884_d_b0, eq72_e1884_d_b1, eq72_e1884_d_b2, eq72_e1884_d_b3, eq72_e1884_d_b4, eq72_e1884_d_b5, eq72_e1884_d_b6, eq72_e1884_d_b7, eq72_e1884_d_b8, eq72_e1884_d_b9, eq72_e1884_d_b10, eq72_e1884_d_b11, eq72_e1884_d_b12, eq72_e1884_d_b13, eq72_e1884_d_b14, eq72_e1884_d_b15, eq72_e1884_d_b16, eq72_e1884_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_reactive_node_derivatives: [f64; 14] = [eq72_e1892_q_d_n0, eq72_e1892_q_d_n1, eq72_e1892_q_d_n2, eq72_e1892_q_d_n3, eq72_e1892_q_d_n4, eq72_e1892_q_d_n5, eq72_e1892_q_d_n6, eq72_e1892_q_d_n7, eq72_e1892_q_d_n8, eq72_e1892_q_d_n9, eq72_e1892_q_d_n10, eq72_e1892_q_d_n11, eq72_e1892_q_d_n12, eq72_e1892_q_d_n13];
        let eq72_reactive_branch_derivatives: [f64; 18] = [eq72_e1892_q_d_b0, eq72_e1892_q_d_b1, eq72_e1892_q_d_b2, eq72_e1892_q_d_b3, eq72_e1892_q_d_b4, eq72_e1892_q_d_b5, eq72_e1892_q_d_b6, eq72_e1892_q_d_b7, eq72_e1892_q_d_b8, eq72_e1892_q_d_b9, eq72_e1892_q_d_b10, eq72_e1892_q_d_b11, eq72_e1892_q_d_b12, eq72_e1892_q_d_b13, eq72_e1892_q_d_b14, eq72_e1892_q_d_b15, eq72_e1892_q_d_b16, eq72_e1892_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq72_reactive_node_derivatives,
            branches,
            &eq72_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1920, eq73_e1920_d_n0, eq73_e1920_d_n1, eq73_e1920_d_n2, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12, eq73_e1920_d_n13, eq73_e1920_d_b0, eq73_e1920_d_b1, eq73_e1920_d_b2, eq73_e1920_d_b3, eq73_e1920_d_b4, eq73_e1920_d_b5, eq73_e1920_d_b6, eq73_e1920_d_b7, eq73_e1920_d_b8, eq73_e1920_d_b9, eq73_e1920_d_b10, eq73_e1920_d_b11, eq73_e1920_d_b12, eq73_e1920_d_b13, eq73_e1920_d_b14, eq73_e1920_d_b15, eq73_e1920_d_b16, eq73_e1920_d_b17, eq73_e1920_q, eq73_e1920_q_d_n0, eq73_e1920_q_d_n1, eq73_e1920_q_d_n2, eq73_e1920_q_d_n3, eq73_e1920_q_d_n4, eq73_e1920_q_d_n5, eq73_e1920_q_d_n6, eq73_e1920_q_d_n7, eq73_e1920_q_d_n8, eq73_e1920_q_d_n9, eq73_e1920_q_d_n10, eq73_e1920_q_d_n11, eq73_e1920_q_d_n12, eq73_e1920_q_d_n13, eq73_e1920_q_d_b0, eq73_e1920_q_d_b1, eq73_e1920_q_d_b2, eq73_e1920_q_d_b3, eq73_e1920_q_d_b4, eq73_e1920_q_d_b5, eq73_e1920_q_d_b6, eq73_e1920_q_d_b7, eq73_e1920_q_d_b8, eq73_e1920_q_d_b9, eq73_e1920_q_d_b10, eq73_e1920_q_d_b11, eq73_e1920_q_d_b12, eq73_e1920_q_d_b13, eq73_e1920_q_d_b14, eq73_e1920_q_d_b15, eq73_e1920_q_d_b16, eq73_e1920_q_d_b17,) = {
    if ((((s.b[1559] && s.b[1560]) && (!s.b[1561])) && (!s.b[1562])) && s.b[1563]) {
        let eq73_e1906: f64 = (s.v[885] / p.p30);
        let eq73_e1906_d_n0: f64 = (s.dn[885][0] / p.p30);
        let eq73_e1906_d_n1: f64 = (s.dn[885][1] / p.p30);
        let eq73_e1906_d_n2: f64 = (s.dn[885][2] / p.p30);
        let eq73_e1906_d_n3: f64 = (s.dn[885][3] / p.p30);
        let eq73_e1906_d_n4: f64 = (s.dn[885][4] / p.p30);
        let eq73_e1906_d_n5: f64 = (s.dn[885][5] / p.p30);
        let eq73_e1906_d_n6: f64 = (s.dn[885][6] / p.p30);
        let eq73_e1906_d_n7: f64 = (s.dn[885][7] / p.p30);
        let eq73_e1906_d_n8: f64 = (s.dn[885][8] / p.p30);
        let eq73_e1906_d_n9: f64 = (s.dn[885][9] / p.p30);
        let eq73_e1906_d_n10: f64 = (s.dn[885][10] / p.p30);
        let eq73_e1906_d_n11: f64 = (s.dn[885][11] / p.p30);
        let eq73_e1906_d_n12: f64 = (s.dn[885][12] / p.p30);
        let eq73_e1906_d_n13: f64 = (s.dn[885][13] / p.p30);
        let eq73_e1906_d_b0: f64 = (s.db[885][0] / p.p30);
        let eq73_e1906_d_b1: f64 = (s.db[885][1] / p.p30);
        let eq73_e1906_d_b2: f64 = (s.db[885][2] / p.p30);
        let eq73_e1906_d_b3: f64 = (s.db[885][3] / p.p30);
        let eq73_e1906_d_b4: f64 = (s.db[885][4] / p.p30);
        let eq73_e1906_d_b5: f64 = (s.db[885][5] / p.p30);
        let eq73_e1906_d_b6: f64 = (s.db[885][6] / p.p30);
        let eq73_e1906_d_b7: f64 = (s.db[885][7] / p.p30);
        let eq73_e1906_d_b8: f64 = (s.db[885][8] / p.p30);
        let eq73_e1906_d_b9: f64 = (s.db[885][9] / p.p30);
        let eq73_e1906_d_b10: f64 = (s.db[885][10] / p.p30);
        let eq73_e1906_d_b11: f64 = (s.db[885][11] / p.p30);
        let eq73_e1906_d_b12: f64 = (s.db[885][12] / p.p30);
        let eq73_e1906_d_b13: f64 = (s.db[885][13] / p.p30);
        let eq73_e1906_d_b14: f64 = (s.db[885][14] / p.p30);
        let eq73_e1906_d_b15: f64 = (s.db[885][15] / p.p30);
        let eq73_e1906_d_b16: f64 = (s.db[885][16] / p.p30);
        let eq73_e1906_d_b17: f64 = (s.db[885][17] / p.p30);
        let eq73_e1907: f64 = (-eq73_e1906);
        let eq73_e1907_d_n0: f64 = (-eq73_e1906_d_n0);
        let eq73_e1907_d_n1: f64 = (-eq73_e1906_d_n1);
        let eq73_e1907_d_n2: f64 = (-eq73_e1906_d_n2);
        let eq73_e1907_d_n3: f64 = (-eq73_e1906_d_n3);
        let eq73_e1907_d_n4: f64 = (-eq73_e1906_d_n4);
        let eq73_e1907_d_n5: f64 = (-eq73_e1906_d_n5);
        let eq73_e1907_d_n6: f64 = (-eq73_e1906_d_n6);
        let eq73_e1907_d_n7: f64 = (-eq73_e1906_d_n7);
        let eq73_e1907_d_n8: f64 = (-eq73_e1906_d_n8);
        let eq73_e1907_d_n9: f64 = (-eq73_e1906_d_n9);
        let eq73_e1907_d_n10: f64 = (-eq73_e1906_d_n10);
        let eq73_e1907_d_n11: f64 = (-eq73_e1906_d_n11);
        let eq73_e1907_d_n12: f64 = (-eq73_e1906_d_n12);
        let eq73_e1907_d_n13: f64 = (-eq73_e1906_d_n13);
        let eq73_e1907_d_b0: f64 = (-eq73_e1906_d_b0);
        let eq73_e1907_d_b1: f64 = (-eq73_e1906_d_b1);
        let eq73_e1907_d_b2: f64 = (-eq73_e1906_d_b2);
        let eq73_e1907_d_b3: f64 = (-eq73_e1906_d_b3);
        let eq73_e1907_d_b4: f64 = (-eq73_e1906_d_b4);
        let eq73_e1907_d_b5: f64 = (-eq73_e1906_d_b5);
        let eq73_e1907_d_b6: f64 = (-eq73_e1906_d_b6);
        let eq73_e1907_d_b7: f64 = (-eq73_e1906_d_b7);
        let eq73_e1907_d_b8: f64 = (-eq73_e1906_d_b8);
        let eq73_e1907_d_b9: f64 = (-eq73_e1906_d_b9);
        let eq73_e1907_d_b10: f64 = (-eq73_e1906_d_b10);
        let eq73_e1907_d_b11: f64 = (-eq73_e1906_d_b11);
        let eq73_e1907_d_b12: f64 = (-eq73_e1906_d_b12);
        let eq73_e1907_d_b13: f64 = (-eq73_e1906_d_b13);
        let eq73_e1907_d_b14: f64 = (-eq73_e1906_d_b14);
        let eq73_e1907_d_b15: f64 = (-eq73_e1906_d_b15);
        let eq73_e1907_d_b16: f64 = (-eq73_e1906_d_b16);
        let eq73_e1907_d_b17: f64 = (-eq73_e1906_d_b17);
        let eq73_e1909: f64 = (eq73_e1907 * s.v[822]);
        let eq73_e1909_d_n0: f64 = ((eq73_e1907_d_n0 * s.v[822]) + (eq73_e1907 * s.dn[822][0]));
        let eq73_e1909_d_n1: f64 = ((eq73_e1907_d_n1 * s.v[822]) + (eq73_e1907 * s.dn[822][1]));
        let eq73_e1909_d_n2: f64 = ((eq73_e1907_d_n2 * s.v[822]) + (eq73_e1907 * s.dn[822][2]));
        let eq73_e1909_d_n3: f64 = ((eq73_e1907_d_n3 * s.v[822]) + (eq73_e1907 * s.dn[822][3]));
        let eq73_e1909_d_n4: f64 = ((eq73_e1907_d_n4 * s.v[822]) + (eq73_e1907 * s.dn[822][4]));
        let eq73_e1909_d_n5: f64 = ((eq73_e1907_d_n5 * s.v[822]) + (eq73_e1907 * s.dn[822][5]));
        let eq73_e1909_d_n6: f64 = ((eq73_e1907_d_n6 * s.v[822]) + (eq73_e1907 * s.dn[822][6]));
        let eq73_e1909_d_n7: f64 = ((eq73_e1907_d_n7 * s.v[822]) + (eq73_e1907 * s.dn[822][7]));
        let eq73_e1909_d_n8: f64 = ((eq73_e1907_d_n8 * s.v[822]) + (eq73_e1907 * s.dn[822][8]));
        let eq73_e1909_d_n9: f64 = ((eq73_e1907_d_n9 * s.v[822]) + (eq73_e1907 * s.dn[822][9]));
        let eq73_e1909_d_n10: f64 = ((eq73_e1907_d_n10 * s.v[822]) + (eq73_e1907 * s.dn[822][10]));
        let eq73_e1909_d_n11: f64 = ((eq73_e1907_d_n11 * s.v[822]) + (eq73_e1907 * s.dn[822][11]));
        let eq73_e1909_d_n12: f64 = ((eq73_e1907_d_n12 * s.v[822]) + (eq73_e1907 * s.dn[822][12]));
        let eq73_e1909_d_n13: f64 = ((eq73_e1907_d_n13 * s.v[822]) + (eq73_e1907 * s.dn[822][13]));
        let eq73_e1909_d_b0: f64 = ((eq73_e1907_d_b0 * s.v[822]) + (eq73_e1907 * s.db[822][0]));
        let eq73_e1909_d_b1: f64 = ((eq73_e1907_d_b1 * s.v[822]) + (eq73_e1907 * s.db[822][1]));
        let eq73_e1909_d_b2: f64 = ((eq73_e1907_d_b2 * s.v[822]) + (eq73_e1907 * s.db[822][2]));
        let eq73_e1909_d_b3: f64 = ((eq73_e1907_d_b3 * s.v[822]) + (eq73_e1907 * s.db[822][3]));
        let eq73_e1909_d_b4: f64 = ((eq73_e1907_d_b4 * s.v[822]) + (eq73_e1907 * s.db[822][4]));
        let eq73_e1909_d_b5: f64 = ((eq73_e1907_d_b5 * s.v[822]) + (eq73_e1907 * s.db[822][5]));
        let eq73_e1909_d_b6: f64 = ((eq73_e1907_d_b6 * s.v[822]) + (eq73_e1907 * s.db[822][6]));
        let eq73_e1909_d_b7: f64 = ((eq73_e1907_d_b7 * s.v[822]) + (eq73_e1907 * s.db[822][7]));
        let eq73_e1909_d_b8: f64 = ((eq73_e1907_d_b8 * s.v[822]) + (eq73_e1907 * s.db[822][8]));
        let eq73_e1909_d_b9: f64 = ((eq73_e1907_d_b9 * s.v[822]) + (eq73_e1907 * s.db[822][9]));
        let eq73_e1909_d_b10: f64 = ((eq73_e1907_d_b10 * s.v[822]) + (eq73_e1907 * s.db[822][10]));
        let eq73_e1909_d_b11: f64 = ((eq73_e1907_d_b11 * s.v[822]) + (eq73_e1907 * s.db[822][11]));
        let eq73_e1909_d_b12: f64 = ((eq73_e1907_d_b12 * s.v[822]) + (eq73_e1907 * s.db[822][12]));
        let eq73_e1909_d_b13: f64 = ((eq73_e1907_d_b13 * s.v[822]) + (eq73_e1907 * s.db[822][13]));
        let eq73_e1909_d_b14: f64 = ((eq73_e1907_d_b14 * s.v[822]) + (eq73_e1907 * s.db[822][14]));
        let eq73_e1909_d_b15: f64 = ((eq73_e1907_d_b15 * s.v[822]) + (eq73_e1907 * s.db[822][15]));
        let eq73_e1909_d_b16: f64 = ((eq73_e1907_d_b16 * s.v[822]) + (eq73_e1907 * s.db[822][16]));
        let eq73_e1909_d_b17: f64 = ((eq73_e1907_d_b17 * s.v[822]) + (eq73_e1907 * s.db[822][17]));
        let eq73_e1912: f64 = (s.v[410] * s.v[158]);
        let eq73_e1912_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq73_e1912_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq73_e1912_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq73_e1912_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq73_e1912_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq73_e1912_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq73_e1912_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq73_e1912_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq73_e1912_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq73_e1912_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq73_e1912_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq73_e1912_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq73_e1912_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq73_e1912_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq73_e1912_d_b0: f64 = (s.db[410][0] * s.v[158]);
        let eq73_e1912_d_b1: f64 = (s.db[410][1] * s.v[158]);
        let eq73_e1912_d_b2: f64 = (s.db[410][2] * s.v[158]);
        let eq73_e1912_d_b3: f64 = (s.db[410][3] * s.v[158]);
        let eq73_e1912_d_b4: f64 = (s.db[410][4] * s.v[158]);
        let eq73_e1912_d_b5: f64 = (s.db[410][5] * s.v[158]);
        let eq73_e1912_d_b6: f64 = (s.db[410][6] * s.v[158]);
        let eq73_e1912_d_b7: f64 = (s.db[410][7] * s.v[158]);
        let eq73_e1912_d_b8: f64 = (s.db[410][8] * s.v[158]);
        let eq73_e1912_d_b9: f64 = (s.db[410][9] * s.v[158]);
        let eq73_e1912_d_b10: f64 = (s.db[410][10] * s.v[158]);
        let eq73_e1912_d_b11: f64 = (s.db[410][11] * s.v[158]);
        let eq73_e1912_d_b12: f64 = (s.db[410][12] * s.v[158]);
        let eq73_e1912_d_b13: f64 = (s.db[410][13] * s.v[158]);
        let eq73_e1912_d_b14: f64 = (s.db[410][14] * s.v[158]);
        let eq73_e1912_d_b15: f64 = (s.db[410][15] * s.v[158]);
        let eq73_e1912_d_b16: f64 = (s.db[410][16] * s.v[158]);
        let eq73_e1912_d_b17: f64 = (s.db[410][17] * s.v[158]);
        let eq73_e1913_q: f64 = eq73_e1912;
        let eq73_e1914: f64 = (eq73_e1909 + eq73_e1912);
        let eq73_e1914_d_n0: f64 = (eq73_e1909_d_n0 + eq73_e1912_d_n0);
        let eq73_e1914_d_n1: f64 = (eq73_e1909_d_n1 + eq73_e1912_d_n1);
        let eq73_e1914_d_n2: f64 = (eq73_e1909_d_n2 + eq73_e1912_d_n2);
        let eq73_e1914_d_n3: f64 = (eq73_e1909_d_n3 + eq73_e1912_d_n3);
        let eq73_e1914_d_n4: f64 = (eq73_e1909_d_n4 + eq73_e1912_d_n4);
        let eq73_e1914_d_n5: f64 = (eq73_e1909_d_n5 + eq73_e1912_d_n5);
        let eq73_e1914_d_n6: f64 = (eq73_e1909_d_n6 + eq73_e1912_d_n6);
        let eq73_e1914_d_n7: f64 = (eq73_e1909_d_n7 + eq73_e1912_d_n7);
        let eq73_e1914_d_n8: f64 = (eq73_e1909_d_n8 + eq73_e1912_d_n8);
        let eq73_e1914_d_n9: f64 = (eq73_e1909_d_n9 + eq73_e1912_d_n9);
        let eq73_e1914_d_n10: f64 = (eq73_e1909_d_n10 + eq73_e1912_d_n10);
        let eq73_e1914_d_n11: f64 = (eq73_e1909_d_n11 + eq73_e1912_d_n11);
        let eq73_e1914_d_n12: f64 = (eq73_e1909_d_n12 + eq73_e1912_d_n12);
        let eq73_e1914_d_n13: f64 = (eq73_e1909_d_n13 + eq73_e1912_d_n13);
        let eq73_e1914_d_b0: f64 = (eq73_e1909_d_b0 + eq73_e1912_d_b0);
        let eq73_e1914_d_b1: f64 = (eq73_e1909_d_b1 + eq73_e1912_d_b1);
        let eq73_e1914_d_b2: f64 = (eq73_e1909_d_b2 + eq73_e1912_d_b2);
        let eq73_e1914_d_b3: f64 = (eq73_e1909_d_b3 + eq73_e1912_d_b3);
        let eq73_e1914_d_b4: f64 = (eq73_e1909_d_b4 + eq73_e1912_d_b4);
        let eq73_e1914_d_b5: f64 = (eq73_e1909_d_b5 + eq73_e1912_d_b5);
        let eq73_e1914_d_b6: f64 = (eq73_e1909_d_b6 + eq73_e1912_d_b6);
        let eq73_e1914_d_b7: f64 = (eq73_e1909_d_b7 + eq73_e1912_d_b7);
        let eq73_e1914_d_b8: f64 = (eq73_e1909_d_b8 + eq73_e1912_d_b8);
        let eq73_e1914_d_b9: f64 = (eq73_e1909_d_b9 + eq73_e1912_d_b9);
        let eq73_e1914_d_b10: f64 = (eq73_e1909_d_b10 + eq73_e1912_d_b10);
        let eq73_e1914_d_b11: f64 = (eq73_e1909_d_b11 + eq73_e1912_d_b11);
        let eq73_e1914_d_b12: f64 = (eq73_e1909_d_b12 + eq73_e1912_d_b12);
        let eq73_e1914_d_b13: f64 = (eq73_e1909_d_b13 + eq73_e1912_d_b13);
        let eq73_e1914_d_b14: f64 = (eq73_e1909_d_b14 + eq73_e1912_d_b14);
        let eq73_e1914_d_b15: f64 = (eq73_e1909_d_b15 + eq73_e1912_d_b15);
        let eq73_e1914_d_b16: f64 = (eq73_e1909_d_b16 + eq73_e1912_d_b16);
        let eq73_e1914_d_b17: f64 = (eq73_e1909_d_b17 + eq73_e1912_d_b17);
        let eq73_e1914_q: f64 = eq73_e1913_q;
        let eq73_e1917: f64 = (s.v[410] / s.v[157]);
        let eq73_e1917_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq73_e1917_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq73_e1917_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq73_e1917_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq73_e1917_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq73_e1917_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq73_e1917_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq73_e1917_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq73_e1917_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq73_e1917_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq73_e1917_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq73_e1917_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq73_e1917_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq73_e1917_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq73_e1917_d_b0: f64 = (s.db[410][0] / s.v[157]);
        let eq73_e1917_d_b1: f64 = (s.db[410][1] / s.v[157]);
        let eq73_e1917_d_b2: f64 = (s.db[410][2] / s.v[157]);
        let eq73_e1917_d_b3: f64 = (s.db[410][3] / s.v[157]);
        let eq73_e1917_d_b4: f64 = (s.db[410][4] / s.v[157]);
        let eq73_e1917_d_b5: f64 = (s.db[410][5] / s.v[157]);
        let eq73_e1917_d_b6: f64 = (s.db[410][6] / s.v[157]);
        let eq73_e1917_d_b7: f64 = (s.db[410][7] / s.v[157]);
        let eq73_e1917_d_b8: f64 = (s.db[410][8] / s.v[157]);
        let eq73_e1917_d_b9: f64 = (s.db[410][9] / s.v[157]);
        let eq73_e1917_d_b10: f64 = (s.db[410][10] / s.v[157]);
        let eq73_e1917_d_b11: f64 = (s.db[410][11] / s.v[157]);
        let eq73_e1917_d_b12: f64 = (s.db[410][12] / s.v[157]);
        let eq73_e1917_d_b13: f64 = (s.db[410][13] / s.v[157]);
        let eq73_e1917_d_b14: f64 = (s.db[410][14] / s.v[157]);
        let eq73_e1917_d_b15: f64 = (s.db[410][15] / s.v[157]);
        let eq73_e1917_d_b16: f64 = (s.db[410][16] / s.v[157]);
        let eq73_e1917_d_b17: f64 = (s.db[410][17] / s.v[157]);
        let eq73_e1918: f64 = (eq73_e1914 + eq73_e1917);
        let eq73_e1918_d_n0: f64 = (eq73_e1914_d_n0 + eq73_e1917_d_n0);
        let eq73_e1918_d_n1: f64 = (eq73_e1914_d_n1 + eq73_e1917_d_n1);
        let eq73_e1918_d_n2: f64 = (eq73_e1914_d_n2 + eq73_e1917_d_n2);
        let eq73_e1918_d_n3: f64 = (eq73_e1914_d_n3 + eq73_e1917_d_n3);
        let eq73_e1918_d_n4: f64 = (eq73_e1914_d_n4 + eq73_e1917_d_n4);
        let eq73_e1918_d_n5: f64 = (eq73_e1914_d_n5 + eq73_e1917_d_n5);
        let eq73_e1918_d_n6: f64 = (eq73_e1914_d_n6 + eq73_e1917_d_n6);
        let eq73_e1918_d_n7: f64 = (eq73_e1914_d_n7 + eq73_e1917_d_n7);
        let eq73_e1918_d_n8: f64 = (eq73_e1914_d_n8 + eq73_e1917_d_n8);
        let eq73_e1918_d_n9: f64 = (eq73_e1914_d_n9 + eq73_e1917_d_n9);
        let eq73_e1918_d_n10: f64 = (eq73_e1914_d_n10 + eq73_e1917_d_n10);
        let eq73_e1918_d_n11: f64 = (eq73_e1914_d_n11 + eq73_e1917_d_n11);
        let eq73_e1918_d_n12: f64 = (eq73_e1914_d_n12 + eq73_e1917_d_n12);
        let eq73_e1918_d_n13: f64 = (eq73_e1914_d_n13 + eq73_e1917_d_n13);
        let eq73_e1918_d_b0: f64 = (eq73_e1914_d_b0 + eq73_e1917_d_b0);
        let eq73_e1918_d_b1: f64 = (eq73_e1914_d_b1 + eq73_e1917_d_b1);
        let eq73_e1918_d_b2: f64 = (eq73_e1914_d_b2 + eq73_e1917_d_b2);
        let eq73_e1918_d_b3: f64 = (eq73_e1914_d_b3 + eq73_e1917_d_b3);
        let eq73_e1918_d_b4: f64 = (eq73_e1914_d_b4 + eq73_e1917_d_b4);
        let eq73_e1918_d_b5: f64 = (eq73_e1914_d_b5 + eq73_e1917_d_b5);
        let eq73_e1918_d_b6: f64 = (eq73_e1914_d_b6 + eq73_e1917_d_b6);
        let eq73_e1918_d_b7: f64 = (eq73_e1914_d_b7 + eq73_e1917_d_b7);
        let eq73_e1918_d_b8: f64 = (eq73_e1914_d_b8 + eq73_e1917_d_b8);
        let eq73_e1918_d_b9: f64 = (eq73_e1914_d_b9 + eq73_e1917_d_b9);
        let eq73_e1918_d_b10: f64 = (eq73_e1914_d_b10 + eq73_e1917_d_b10);
        let eq73_e1918_d_b11: f64 = (eq73_e1914_d_b11 + eq73_e1917_d_b11);
        let eq73_e1918_d_b12: f64 = (eq73_e1914_d_b12 + eq73_e1917_d_b12);
        let eq73_e1918_d_b13: f64 = (eq73_e1914_d_b13 + eq73_e1917_d_b13);
        let eq73_e1918_d_b14: f64 = (eq73_e1914_d_b14 + eq73_e1917_d_b14);
        let eq73_e1918_d_b15: f64 = (eq73_e1914_d_b15 + eq73_e1917_d_b15);
        let eq73_e1918_d_b16: f64 = (eq73_e1914_d_b16 + eq73_e1917_d_b16);
        let eq73_e1918_d_b17: f64 = (eq73_e1914_d_b17 + eq73_e1917_d_b17);
        let eq73_e1918_q: f64 = eq73_e1914_q;
        (eq73_e1918, eq73_e1918_d_n0, eq73_e1918_d_n1, eq73_e1918_d_n2, eq73_e1918_d_n3, eq73_e1918_d_n4, eq73_e1918_d_n5, eq73_e1918_d_n6, eq73_e1918_d_n7, eq73_e1918_d_n8, eq73_e1918_d_n9, eq73_e1918_d_n10, eq73_e1918_d_n11, eq73_e1918_d_n12, eq73_e1918_d_n13, eq73_e1918_d_b0, eq73_e1918_d_b1, eq73_e1918_d_b2, eq73_e1918_d_b3, eq73_e1918_d_b4, eq73_e1918_d_b5, eq73_e1918_d_b6, eq73_e1918_d_b7, eq73_e1918_d_b8, eq73_e1918_d_b9, eq73_e1918_d_b10, eq73_e1918_d_b11, eq73_e1918_d_b12, eq73_e1918_d_b13, eq73_e1918_d_b14, eq73_e1918_d_b15, eq73_e1918_d_b16, eq73_e1918_d_b17, eq73_e1918_q, eq73_e1912_d_n0, eq73_e1912_d_n1, eq73_e1912_d_n2, eq73_e1912_d_n3, eq73_e1912_d_n4, eq73_e1912_d_n5, eq73_e1912_d_n6, eq73_e1912_d_n7, eq73_e1912_d_n8, eq73_e1912_d_n9, eq73_e1912_d_n10, eq73_e1912_d_n11, eq73_e1912_d_n12, eq73_e1912_d_n13, eq73_e1912_d_b0, eq73_e1912_d_b1, eq73_e1912_d_b2, eq73_e1912_d_b3, eq73_e1912_d_b4, eq73_e1912_d_b5, eq73_e1912_d_b6, eq73_e1912_d_b7, eq73_e1912_d_b8, eq73_e1912_d_b9, eq73_e1912_d_b10, eq73_e1912_d_b11, eq73_e1912_d_b12, eq73_e1912_d_b13, eq73_e1912_d_b14, eq73_e1912_d_b15, eq73_e1912_d_b16, eq73_e1912_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 14] = [eq73_e1920_q_d_n0, eq73_e1920_q_d_n1, eq73_e1920_q_d_n2, eq73_e1920_q_d_n3, eq73_e1920_q_d_n4, eq73_e1920_q_d_n5, eq73_e1920_q_d_n6, eq73_e1920_q_d_n7, eq73_e1920_q_d_n8, eq73_e1920_q_d_n9, eq73_e1920_q_d_n10, eq73_e1920_q_d_n11, eq73_e1920_q_d_n12, eq73_e1920_q_d_n13];
        let eq73_reactive_branch_derivatives: [f64; 18] = [eq73_e1920_q_d_b0, eq73_e1920_q_d_b1, eq73_e1920_q_d_b2, eq73_e1920_q_d_b3, eq73_e1920_q_d_b4, eq73_e1920_q_d_b5, eq73_e1920_q_d_b6, eq73_e1920_q_d_b7, eq73_e1920_q_d_b8, eq73_e1920_q_d_b9, eq73_e1920_q_d_b10, eq73_e1920_q_d_b11, eq73_e1920_q_d_b12, eq73_e1920_q_d_b13, eq73_e1920_q_d_b14, eq73_e1920_q_d_b15, eq73_e1920_q_d_b16, eq73_e1920_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_4(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq74_e1947, eq74_e1947_d_n0, eq74_e1947_d_n1, eq74_e1947_d_n2, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12, eq74_e1947_d_n13, eq74_e1947_d_b0, eq74_e1947_d_b1, eq74_e1947_d_b2, eq74_e1947_d_b3, eq74_e1947_d_b4, eq74_e1947_d_b5, eq74_e1947_d_b6, eq74_e1947_d_b7, eq74_e1947_d_b8, eq74_e1947_d_b9, eq74_e1947_d_b10, eq74_e1947_d_b11, eq74_e1947_d_b12, eq74_e1947_d_b13, eq74_e1947_d_b14, eq74_e1947_d_b15, eq74_e1947_d_b16, eq74_e1947_d_b17, eq74_e1947_q, eq74_e1947_q_d_n0, eq74_e1947_q_d_n1, eq74_e1947_q_d_n2, eq74_e1947_q_d_n3, eq74_e1947_q_d_n4, eq74_e1947_q_d_n5, eq74_e1947_q_d_n6, eq74_e1947_q_d_n7, eq74_e1947_q_d_n8, eq74_e1947_q_d_n9, eq74_e1947_q_d_n10, eq74_e1947_q_d_n11, eq74_e1947_q_d_n12, eq74_e1947_q_d_n13, eq74_e1947_q_d_b0, eq74_e1947_q_d_b1, eq74_e1947_q_d_b2, eq74_e1947_q_d_b3, eq74_e1947_q_d_b4, eq74_e1947_q_d_b5, eq74_e1947_q_d_b6, eq74_e1947_q_d_b7, eq74_e1947_q_d_b8, eq74_e1947_q_d_b9, eq74_e1947_q_d_b10, eq74_e1947_q_d_b11, eq74_e1947_q_d_b12, eq74_e1947_q_d_b13, eq74_e1947_q_d_b14, eq74_e1947_q_d_b15, eq74_e1947_q_d_b16, eq74_e1947_q_d_b17,) = {
    if ((((s.b[1559] && s.b[1560]) && (!s.b[1561])) && (!s.b[1562])) && (!s.b[1563])) {
        let eq74_e1934: f64 = (-s.v[885]);
        let eq74_e1934_d_n0: f64 = (-s.dn[885][0]);
        let eq74_e1934_d_n1: f64 = (-s.dn[885][1]);
        let eq74_e1934_d_n2: f64 = (-s.dn[885][2]);
        let eq74_e1934_d_n3: f64 = (-s.dn[885][3]);
        let eq74_e1934_d_n4: f64 = (-s.dn[885][4]);
        let eq74_e1934_d_n5: f64 = (-s.dn[885][5]);
        let eq74_e1934_d_n6: f64 = (-s.dn[885][6]);
        let eq74_e1934_d_n7: f64 = (-s.dn[885][7]);
        let eq74_e1934_d_n8: f64 = (-s.dn[885][8]);
        let eq74_e1934_d_n9: f64 = (-s.dn[885][9]);
        let eq74_e1934_d_n10: f64 = (-s.dn[885][10]);
        let eq74_e1934_d_n11: f64 = (-s.dn[885][11]);
        let eq74_e1934_d_n12: f64 = (-s.dn[885][12]);
        let eq74_e1934_d_n13: f64 = (-s.dn[885][13]);
        let eq74_e1934_d_b0: f64 = (-s.db[885][0]);
        let eq74_e1934_d_b1: f64 = (-s.db[885][1]);
        let eq74_e1934_d_b2: f64 = (-s.db[885][2]);
        let eq74_e1934_d_b3: f64 = (-s.db[885][3]);
        let eq74_e1934_d_b4: f64 = (-s.db[885][4]);
        let eq74_e1934_d_b5: f64 = (-s.db[885][5]);
        let eq74_e1934_d_b6: f64 = (-s.db[885][6]);
        let eq74_e1934_d_b7: f64 = (-s.db[885][7]);
        let eq74_e1934_d_b8: f64 = (-s.db[885][8]);
        let eq74_e1934_d_b9: f64 = (-s.db[885][9]);
        let eq74_e1934_d_b10: f64 = (-s.db[885][10]);
        let eq74_e1934_d_b11: f64 = (-s.db[885][11]);
        let eq74_e1934_d_b12: f64 = (-s.db[885][12]);
        let eq74_e1934_d_b13: f64 = (-s.db[885][13]);
        let eq74_e1934_d_b14: f64 = (-s.db[885][14]);
        let eq74_e1934_d_b15: f64 = (-s.db[885][15]);
        let eq74_e1934_d_b16: f64 = (-s.db[885][16]);
        let eq74_e1934_d_b17: f64 = (-s.db[885][17]);
        let eq74_e1936: f64 = (eq74_e1934 * s.v[822]);
        let eq74_e1936_d_n0: f64 = ((eq74_e1934_d_n0 * s.v[822]) + (eq74_e1934 * s.dn[822][0]));
        let eq74_e1936_d_n1: f64 = ((eq74_e1934_d_n1 * s.v[822]) + (eq74_e1934 * s.dn[822][1]));
        let eq74_e1936_d_n2: f64 = ((eq74_e1934_d_n2 * s.v[822]) + (eq74_e1934 * s.dn[822][2]));
        let eq74_e1936_d_n3: f64 = ((eq74_e1934_d_n3 * s.v[822]) + (eq74_e1934 * s.dn[822][3]));
        let eq74_e1936_d_n4: f64 = ((eq74_e1934_d_n4 * s.v[822]) + (eq74_e1934 * s.dn[822][4]));
        let eq74_e1936_d_n5: f64 = ((eq74_e1934_d_n5 * s.v[822]) + (eq74_e1934 * s.dn[822][5]));
        let eq74_e1936_d_n6: f64 = ((eq74_e1934_d_n6 * s.v[822]) + (eq74_e1934 * s.dn[822][6]));
        let eq74_e1936_d_n7: f64 = ((eq74_e1934_d_n7 * s.v[822]) + (eq74_e1934 * s.dn[822][7]));
        let eq74_e1936_d_n8: f64 = ((eq74_e1934_d_n8 * s.v[822]) + (eq74_e1934 * s.dn[822][8]));
        let eq74_e1936_d_n9: f64 = ((eq74_e1934_d_n9 * s.v[822]) + (eq74_e1934 * s.dn[822][9]));
        let eq74_e1936_d_n10: f64 = ((eq74_e1934_d_n10 * s.v[822]) + (eq74_e1934 * s.dn[822][10]));
        let eq74_e1936_d_n11: f64 = ((eq74_e1934_d_n11 * s.v[822]) + (eq74_e1934 * s.dn[822][11]));
        let eq74_e1936_d_n12: f64 = ((eq74_e1934_d_n12 * s.v[822]) + (eq74_e1934 * s.dn[822][12]));
        let eq74_e1936_d_n13: f64 = ((eq74_e1934_d_n13 * s.v[822]) + (eq74_e1934 * s.dn[822][13]));
        let eq74_e1936_d_b0: f64 = ((eq74_e1934_d_b0 * s.v[822]) + (eq74_e1934 * s.db[822][0]));
        let eq74_e1936_d_b1: f64 = ((eq74_e1934_d_b1 * s.v[822]) + (eq74_e1934 * s.db[822][1]));
        let eq74_e1936_d_b2: f64 = ((eq74_e1934_d_b2 * s.v[822]) + (eq74_e1934 * s.db[822][2]));
        let eq74_e1936_d_b3: f64 = ((eq74_e1934_d_b3 * s.v[822]) + (eq74_e1934 * s.db[822][3]));
        let eq74_e1936_d_b4: f64 = ((eq74_e1934_d_b4 * s.v[822]) + (eq74_e1934 * s.db[822][4]));
        let eq74_e1936_d_b5: f64 = ((eq74_e1934_d_b5 * s.v[822]) + (eq74_e1934 * s.db[822][5]));
        let eq74_e1936_d_b6: f64 = ((eq74_e1934_d_b6 * s.v[822]) + (eq74_e1934 * s.db[822][6]));
        let eq74_e1936_d_b7: f64 = ((eq74_e1934_d_b7 * s.v[822]) + (eq74_e1934 * s.db[822][7]));
        let eq74_e1936_d_b8: f64 = ((eq74_e1934_d_b8 * s.v[822]) + (eq74_e1934 * s.db[822][8]));
        let eq74_e1936_d_b9: f64 = ((eq74_e1934_d_b9 * s.v[822]) + (eq74_e1934 * s.db[822][9]));
        let eq74_e1936_d_b10: f64 = ((eq74_e1934_d_b10 * s.v[822]) + (eq74_e1934 * s.db[822][10]));
        let eq74_e1936_d_b11: f64 = ((eq74_e1934_d_b11 * s.v[822]) + (eq74_e1934 * s.db[822][11]));
        let eq74_e1936_d_b12: f64 = ((eq74_e1934_d_b12 * s.v[822]) + (eq74_e1934 * s.db[822][12]));
        let eq74_e1936_d_b13: f64 = ((eq74_e1934_d_b13 * s.v[822]) + (eq74_e1934 * s.db[822][13]));
        let eq74_e1936_d_b14: f64 = ((eq74_e1934_d_b14 * s.v[822]) + (eq74_e1934 * s.db[822][14]));
        let eq74_e1936_d_b15: f64 = ((eq74_e1934_d_b15 * s.v[822]) + (eq74_e1934 * s.db[822][15]));
        let eq74_e1936_d_b16: f64 = ((eq74_e1934_d_b16 * s.v[822]) + (eq74_e1934 * s.db[822][16]));
        let eq74_e1936_d_b17: f64 = ((eq74_e1934_d_b17 * s.v[822]) + (eq74_e1934 * s.db[822][17]));
        let eq74_e1939: f64 = (s.v[410] * s.v[158]);
        let eq74_e1939_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq74_e1939_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq74_e1939_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq74_e1939_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq74_e1939_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq74_e1939_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq74_e1939_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq74_e1939_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq74_e1939_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq74_e1939_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq74_e1939_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq74_e1939_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq74_e1939_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq74_e1939_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq74_e1939_d_b0: f64 = (s.db[410][0] * s.v[158]);
        let eq74_e1939_d_b1: f64 = (s.db[410][1] * s.v[158]);
        let eq74_e1939_d_b2: f64 = (s.db[410][2] * s.v[158]);
        let eq74_e1939_d_b3: f64 = (s.db[410][3] * s.v[158]);
        let eq74_e1939_d_b4: f64 = (s.db[410][4] * s.v[158]);
        let eq74_e1939_d_b5: f64 = (s.db[410][5] * s.v[158]);
        let eq74_e1939_d_b6: f64 = (s.db[410][6] * s.v[158]);
        let eq74_e1939_d_b7: f64 = (s.db[410][7] * s.v[158]);
        let eq74_e1939_d_b8: f64 = (s.db[410][8] * s.v[158]);
        let eq74_e1939_d_b9: f64 = (s.db[410][9] * s.v[158]);
        let eq74_e1939_d_b10: f64 = (s.db[410][10] * s.v[158]);
        let eq74_e1939_d_b11: f64 = (s.db[410][11] * s.v[158]);
        let eq74_e1939_d_b12: f64 = (s.db[410][12] * s.v[158]);
        let eq74_e1939_d_b13: f64 = (s.db[410][13] * s.v[158]);
        let eq74_e1939_d_b14: f64 = (s.db[410][14] * s.v[158]);
        let eq74_e1939_d_b15: f64 = (s.db[410][15] * s.v[158]);
        let eq74_e1939_d_b16: f64 = (s.db[410][16] * s.v[158]);
        let eq74_e1939_d_b17: f64 = (s.db[410][17] * s.v[158]);
        let eq74_e1940_q: f64 = eq74_e1939;
        let eq74_e1941: f64 = (eq74_e1936 + eq74_e1939);
        let eq74_e1941_d_n0: f64 = (eq74_e1936_d_n0 + eq74_e1939_d_n0);
        let eq74_e1941_d_n1: f64 = (eq74_e1936_d_n1 + eq74_e1939_d_n1);
        let eq74_e1941_d_n2: f64 = (eq74_e1936_d_n2 + eq74_e1939_d_n2);
        let eq74_e1941_d_n3: f64 = (eq74_e1936_d_n3 + eq74_e1939_d_n3);
        let eq74_e1941_d_n4: f64 = (eq74_e1936_d_n4 + eq74_e1939_d_n4);
        let eq74_e1941_d_n5: f64 = (eq74_e1936_d_n5 + eq74_e1939_d_n5);
        let eq74_e1941_d_n6: f64 = (eq74_e1936_d_n6 + eq74_e1939_d_n6);
        let eq74_e1941_d_n7: f64 = (eq74_e1936_d_n7 + eq74_e1939_d_n7);
        let eq74_e1941_d_n8: f64 = (eq74_e1936_d_n8 + eq74_e1939_d_n8);
        let eq74_e1941_d_n9: f64 = (eq74_e1936_d_n9 + eq74_e1939_d_n9);
        let eq74_e1941_d_n10: f64 = (eq74_e1936_d_n10 + eq74_e1939_d_n10);
        let eq74_e1941_d_n11: f64 = (eq74_e1936_d_n11 + eq74_e1939_d_n11);
        let eq74_e1941_d_n12: f64 = (eq74_e1936_d_n12 + eq74_e1939_d_n12);
        let eq74_e1941_d_n13: f64 = (eq74_e1936_d_n13 + eq74_e1939_d_n13);
        let eq74_e1941_d_b0: f64 = (eq74_e1936_d_b0 + eq74_e1939_d_b0);
        let eq74_e1941_d_b1: f64 = (eq74_e1936_d_b1 + eq74_e1939_d_b1);
        let eq74_e1941_d_b2: f64 = (eq74_e1936_d_b2 + eq74_e1939_d_b2);
        let eq74_e1941_d_b3: f64 = (eq74_e1936_d_b3 + eq74_e1939_d_b3);
        let eq74_e1941_d_b4: f64 = (eq74_e1936_d_b4 + eq74_e1939_d_b4);
        let eq74_e1941_d_b5: f64 = (eq74_e1936_d_b5 + eq74_e1939_d_b5);
        let eq74_e1941_d_b6: f64 = (eq74_e1936_d_b6 + eq74_e1939_d_b6);
        let eq74_e1941_d_b7: f64 = (eq74_e1936_d_b7 + eq74_e1939_d_b7);
        let eq74_e1941_d_b8: f64 = (eq74_e1936_d_b8 + eq74_e1939_d_b8);
        let eq74_e1941_d_b9: f64 = (eq74_e1936_d_b9 + eq74_e1939_d_b9);
        let eq74_e1941_d_b10: f64 = (eq74_e1936_d_b10 + eq74_e1939_d_b10);
        let eq74_e1941_d_b11: f64 = (eq74_e1936_d_b11 + eq74_e1939_d_b11);
        let eq74_e1941_d_b12: f64 = (eq74_e1936_d_b12 + eq74_e1939_d_b12);
        let eq74_e1941_d_b13: f64 = (eq74_e1936_d_b13 + eq74_e1939_d_b13);
        let eq74_e1941_d_b14: f64 = (eq74_e1936_d_b14 + eq74_e1939_d_b14);
        let eq74_e1941_d_b15: f64 = (eq74_e1936_d_b15 + eq74_e1939_d_b15);
        let eq74_e1941_d_b16: f64 = (eq74_e1936_d_b16 + eq74_e1939_d_b16);
        let eq74_e1941_d_b17: f64 = (eq74_e1936_d_b17 + eq74_e1939_d_b17);
        let eq74_e1941_q: f64 = eq74_e1940_q;
        let eq74_e1944: f64 = (s.v[410] / s.v[157]);
        let eq74_e1944_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq74_e1944_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq74_e1944_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq74_e1944_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq74_e1944_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq74_e1944_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq74_e1944_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq74_e1944_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq74_e1944_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq74_e1944_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq74_e1944_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq74_e1944_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq74_e1944_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq74_e1944_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq74_e1944_d_b0: f64 = (s.db[410][0] / s.v[157]);
        let eq74_e1944_d_b1: f64 = (s.db[410][1] / s.v[157]);
        let eq74_e1944_d_b2: f64 = (s.db[410][2] / s.v[157]);
        let eq74_e1944_d_b3: f64 = (s.db[410][3] / s.v[157]);
        let eq74_e1944_d_b4: f64 = (s.db[410][4] / s.v[157]);
        let eq74_e1944_d_b5: f64 = (s.db[410][5] / s.v[157]);
        let eq74_e1944_d_b6: f64 = (s.db[410][6] / s.v[157]);
        let eq74_e1944_d_b7: f64 = (s.db[410][7] / s.v[157]);
        let eq74_e1944_d_b8: f64 = (s.db[410][8] / s.v[157]);
        let eq74_e1944_d_b9: f64 = (s.db[410][9] / s.v[157]);
        let eq74_e1944_d_b10: f64 = (s.db[410][10] / s.v[157]);
        let eq74_e1944_d_b11: f64 = (s.db[410][11] / s.v[157]);
        let eq74_e1944_d_b12: f64 = (s.db[410][12] / s.v[157]);
        let eq74_e1944_d_b13: f64 = (s.db[410][13] / s.v[157]);
        let eq74_e1944_d_b14: f64 = (s.db[410][14] / s.v[157]);
        let eq74_e1944_d_b15: f64 = (s.db[410][15] / s.v[157]);
        let eq74_e1944_d_b16: f64 = (s.db[410][16] / s.v[157]);
        let eq74_e1944_d_b17: f64 = (s.db[410][17] / s.v[157]);
        let eq74_e1945: f64 = (eq74_e1941 + eq74_e1944);
        let eq74_e1945_d_n0: f64 = (eq74_e1941_d_n0 + eq74_e1944_d_n0);
        let eq74_e1945_d_n1: f64 = (eq74_e1941_d_n1 + eq74_e1944_d_n1);
        let eq74_e1945_d_n2: f64 = (eq74_e1941_d_n2 + eq74_e1944_d_n2);
        let eq74_e1945_d_n3: f64 = (eq74_e1941_d_n3 + eq74_e1944_d_n3);
        let eq74_e1945_d_n4: f64 = (eq74_e1941_d_n4 + eq74_e1944_d_n4);
        let eq74_e1945_d_n5: f64 = (eq74_e1941_d_n5 + eq74_e1944_d_n5);
        let eq74_e1945_d_n6: f64 = (eq74_e1941_d_n6 + eq74_e1944_d_n6);
        let eq74_e1945_d_n7: f64 = (eq74_e1941_d_n7 + eq74_e1944_d_n7);
        let eq74_e1945_d_n8: f64 = (eq74_e1941_d_n8 + eq74_e1944_d_n8);
        let eq74_e1945_d_n9: f64 = (eq74_e1941_d_n9 + eq74_e1944_d_n9);
        let eq74_e1945_d_n10: f64 = (eq74_e1941_d_n10 + eq74_e1944_d_n10);
        let eq74_e1945_d_n11: f64 = (eq74_e1941_d_n11 + eq74_e1944_d_n11);
        let eq74_e1945_d_n12: f64 = (eq74_e1941_d_n12 + eq74_e1944_d_n12);
        let eq74_e1945_d_n13: f64 = (eq74_e1941_d_n13 + eq74_e1944_d_n13);
        let eq74_e1945_d_b0: f64 = (eq74_e1941_d_b0 + eq74_e1944_d_b0);
        let eq74_e1945_d_b1: f64 = (eq74_e1941_d_b1 + eq74_e1944_d_b1);
        let eq74_e1945_d_b2: f64 = (eq74_e1941_d_b2 + eq74_e1944_d_b2);
        let eq74_e1945_d_b3: f64 = (eq74_e1941_d_b3 + eq74_e1944_d_b3);
        let eq74_e1945_d_b4: f64 = (eq74_e1941_d_b4 + eq74_e1944_d_b4);
        let eq74_e1945_d_b5: f64 = (eq74_e1941_d_b5 + eq74_e1944_d_b5);
        let eq74_e1945_d_b6: f64 = (eq74_e1941_d_b6 + eq74_e1944_d_b6);
        let eq74_e1945_d_b7: f64 = (eq74_e1941_d_b7 + eq74_e1944_d_b7);
        let eq74_e1945_d_b8: f64 = (eq74_e1941_d_b8 + eq74_e1944_d_b8);
        let eq74_e1945_d_b9: f64 = (eq74_e1941_d_b9 + eq74_e1944_d_b9);
        let eq74_e1945_d_b10: f64 = (eq74_e1941_d_b10 + eq74_e1944_d_b10);
        let eq74_e1945_d_b11: f64 = (eq74_e1941_d_b11 + eq74_e1944_d_b11);
        let eq74_e1945_d_b12: f64 = (eq74_e1941_d_b12 + eq74_e1944_d_b12);
        let eq74_e1945_d_b13: f64 = (eq74_e1941_d_b13 + eq74_e1944_d_b13);
        let eq74_e1945_d_b14: f64 = (eq74_e1941_d_b14 + eq74_e1944_d_b14);
        let eq74_e1945_d_b15: f64 = (eq74_e1941_d_b15 + eq74_e1944_d_b15);
        let eq74_e1945_d_b16: f64 = (eq74_e1941_d_b16 + eq74_e1944_d_b16);
        let eq74_e1945_d_b17: f64 = (eq74_e1941_d_b17 + eq74_e1944_d_b17);
        let eq74_e1945_q: f64 = eq74_e1941_q;
        (eq74_e1945, eq74_e1945_d_n0, eq74_e1945_d_n1, eq74_e1945_d_n2, eq74_e1945_d_n3, eq74_e1945_d_n4, eq74_e1945_d_n5, eq74_e1945_d_n6, eq74_e1945_d_n7, eq74_e1945_d_n8, eq74_e1945_d_n9, eq74_e1945_d_n10, eq74_e1945_d_n11, eq74_e1945_d_n12, eq74_e1945_d_n13, eq74_e1945_d_b0, eq74_e1945_d_b1, eq74_e1945_d_b2, eq74_e1945_d_b3, eq74_e1945_d_b4, eq74_e1945_d_b5, eq74_e1945_d_b6, eq74_e1945_d_b7, eq74_e1945_d_b8, eq74_e1945_d_b9, eq74_e1945_d_b10, eq74_e1945_d_b11, eq74_e1945_d_b12, eq74_e1945_d_b13, eq74_e1945_d_b14, eq74_e1945_d_b15, eq74_e1945_d_b16, eq74_e1945_d_b17, eq74_e1945_q, eq74_e1939_d_n0, eq74_e1939_d_n1, eq74_e1939_d_n2, eq74_e1939_d_n3, eq74_e1939_d_n4, eq74_e1939_d_n5, eq74_e1939_d_n6, eq74_e1939_d_n7, eq74_e1939_d_n8, eq74_e1939_d_n9, eq74_e1939_d_n10, eq74_e1939_d_n11, eq74_e1939_d_n12, eq74_e1939_d_n13, eq74_e1939_d_b0, eq74_e1939_d_b1, eq74_e1939_d_b2, eq74_e1939_d_b3, eq74_e1939_d_b4, eq74_e1939_d_b5, eq74_e1939_d_b6, eq74_e1939_d_b7, eq74_e1939_d_b8, eq74_e1939_d_b9, eq74_e1939_d_b10, eq74_e1939_d_b11, eq74_e1939_d_b12, eq74_e1939_d_b13, eq74_e1939_d_b14, eq74_e1939_d_b15, eq74_e1939_d_b16, eq74_e1939_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_reactive_node_derivatives: [f64; 14] = [eq74_e1947_q_d_n0, eq74_e1947_q_d_n1, eq74_e1947_q_d_n2, eq74_e1947_q_d_n3, eq74_e1947_q_d_n4, eq74_e1947_q_d_n5, eq74_e1947_q_d_n6, eq74_e1947_q_d_n7, eq74_e1947_q_d_n8, eq74_e1947_q_d_n9, eq74_e1947_q_d_n10, eq74_e1947_q_d_n11, eq74_e1947_q_d_n12, eq74_e1947_q_d_n13];
        let eq74_reactive_branch_derivatives: [f64; 18] = [eq74_e1947_q_d_b0, eq74_e1947_q_d_b1, eq74_e1947_q_d_b2, eq74_e1947_q_d_b3, eq74_e1947_q_d_b4, eq74_e1947_q_d_b5, eq74_e1947_q_d_b6, eq74_e1947_q_d_b7, eq74_e1947_q_d_b8, eq74_e1947_q_d_b9, eq74_e1947_q_d_b10, eq74_e1947_q_d_b11, eq74_e1947_q_d_b12, eq74_e1947_q_d_b13, eq74_e1947_q_d_b14, eq74_e1947_q_d_b15, eq74_e1947_q_d_b16, eq74_e1947_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq74_reactive_node_derivatives,
            branches,
            &eq74_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq75_e1970, eq75_e1970_d_n0, eq75_e1970_d_n1, eq75_e1970_d_n2, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12, eq75_e1970_d_n13, eq75_e1970_d_b0, eq75_e1970_d_b1, eq75_e1970_d_b2, eq75_e1970_d_b3, eq75_e1970_d_b4, eq75_e1970_d_b5, eq75_e1970_d_b6, eq75_e1970_d_b7, eq75_e1970_d_b8, eq75_e1970_d_b9, eq75_e1970_d_b10, eq75_e1970_d_b11, eq75_e1970_d_b12, eq75_e1970_d_b13, eq75_e1970_d_b14, eq75_e1970_d_b15, eq75_e1970_d_b16, eq75_e1970_d_b17, eq75_e1970_q, eq75_e1970_q_d_n0, eq75_e1970_q_d_n1, eq75_e1970_q_d_n2, eq75_e1970_q_d_n3, eq75_e1970_q_d_n4, eq75_e1970_q_d_n5, eq75_e1970_q_d_n6, eq75_e1970_q_d_n7, eq75_e1970_q_d_n8, eq75_e1970_q_d_n9, eq75_e1970_q_d_n10, eq75_e1970_q_d_n11, eq75_e1970_q_d_n12, eq75_e1970_q_d_n13, eq75_e1970_q_d_b0, eq75_e1970_q_d_b1, eq75_e1970_q_d_b2, eq75_e1970_q_d_b3, eq75_e1970_q_d_b4, eq75_e1970_q_d_b5, eq75_e1970_q_d_b6, eq75_e1970_q_d_b7, eq75_e1970_q_d_b8, eq75_e1970_q_d_b9, eq75_e1970_q_d_b10, eq75_e1970_q_d_b11, eq75_e1970_q_d_b12, eq75_e1970_q_d_b13, eq75_e1970_q_d_b14, eq75_e1970_q_d_b15, eq75_e1970_q_d_b16, eq75_e1970_q_d_b17,) = {
    if ((s.b[1559] && (!s.b[1560])) && s.b[1564]) {
        let eq75_e1956: f64 = (s.v[885] / p.p30);
        let eq75_e1956_d_n0: f64 = (s.dn[885][0] / p.p30);
        let eq75_e1956_d_n1: f64 = (s.dn[885][1] / p.p30);
        let eq75_e1956_d_n2: f64 = (s.dn[885][2] / p.p30);
        let eq75_e1956_d_n3: f64 = (s.dn[885][3] / p.p30);
        let eq75_e1956_d_n4: f64 = (s.dn[885][4] / p.p30);
        let eq75_e1956_d_n5: f64 = (s.dn[885][5] / p.p30);
        let eq75_e1956_d_n6: f64 = (s.dn[885][6] / p.p30);
        let eq75_e1956_d_n7: f64 = (s.dn[885][7] / p.p30);
        let eq75_e1956_d_n8: f64 = (s.dn[885][8] / p.p30);
        let eq75_e1956_d_n9: f64 = (s.dn[885][9] / p.p30);
        let eq75_e1956_d_n10: f64 = (s.dn[885][10] / p.p30);
        let eq75_e1956_d_n11: f64 = (s.dn[885][11] / p.p30);
        let eq75_e1956_d_n12: f64 = (s.dn[885][12] / p.p30);
        let eq75_e1956_d_n13: f64 = (s.dn[885][13] / p.p30);
        let eq75_e1956_d_b0: f64 = (s.db[885][0] / p.p30);
        let eq75_e1956_d_b1: f64 = (s.db[885][1] / p.p30);
        let eq75_e1956_d_b2: f64 = (s.db[885][2] / p.p30);
        let eq75_e1956_d_b3: f64 = (s.db[885][3] / p.p30);
        let eq75_e1956_d_b4: f64 = (s.db[885][4] / p.p30);
        let eq75_e1956_d_b5: f64 = (s.db[885][5] / p.p30);
        let eq75_e1956_d_b6: f64 = (s.db[885][6] / p.p30);
        let eq75_e1956_d_b7: f64 = (s.db[885][7] / p.p30);
        let eq75_e1956_d_b8: f64 = (s.db[885][8] / p.p30);
        let eq75_e1956_d_b9: f64 = (s.db[885][9] / p.p30);
        let eq75_e1956_d_b10: f64 = (s.db[885][10] / p.p30);
        let eq75_e1956_d_b11: f64 = (s.db[885][11] / p.p30);
        let eq75_e1956_d_b12: f64 = (s.db[885][12] / p.p30);
        let eq75_e1956_d_b13: f64 = (s.db[885][13] / p.p30);
        let eq75_e1956_d_b14: f64 = (s.db[885][14] / p.p30);
        let eq75_e1956_d_b15: f64 = (s.db[885][15] / p.p30);
        let eq75_e1956_d_b16: f64 = (s.db[885][16] / p.p30);
        let eq75_e1956_d_b17: f64 = (s.db[885][17] / p.p30);
        let eq75_e1957: f64 = (-eq75_e1956);
        let eq75_e1957_d_n0: f64 = (-eq75_e1956_d_n0);
        let eq75_e1957_d_n1: f64 = (-eq75_e1956_d_n1);
        let eq75_e1957_d_n2: f64 = (-eq75_e1956_d_n2);
        let eq75_e1957_d_n3: f64 = (-eq75_e1956_d_n3);
        let eq75_e1957_d_n4: f64 = (-eq75_e1956_d_n4);
        let eq75_e1957_d_n5: f64 = (-eq75_e1956_d_n5);
        let eq75_e1957_d_n6: f64 = (-eq75_e1956_d_n6);
        let eq75_e1957_d_n7: f64 = (-eq75_e1956_d_n7);
        let eq75_e1957_d_n8: f64 = (-eq75_e1956_d_n8);
        let eq75_e1957_d_n9: f64 = (-eq75_e1956_d_n9);
        let eq75_e1957_d_n10: f64 = (-eq75_e1956_d_n10);
        let eq75_e1957_d_n11: f64 = (-eq75_e1956_d_n11);
        let eq75_e1957_d_n12: f64 = (-eq75_e1956_d_n12);
        let eq75_e1957_d_n13: f64 = (-eq75_e1956_d_n13);
        let eq75_e1957_d_b0: f64 = (-eq75_e1956_d_b0);
        let eq75_e1957_d_b1: f64 = (-eq75_e1956_d_b1);
        let eq75_e1957_d_b2: f64 = (-eq75_e1956_d_b2);
        let eq75_e1957_d_b3: f64 = (-eq75_e1956_d_b3);
        let eq75_e1957_d_b4: f64 = (-eq75_e1956_d_b4);
        let eq75_e1957_d_b5: f64 = (-eq75_e1956_d_b5);
        let eq75_e1957_d_b6: f64 = (-eq75_e1956_d_b6);
        let eq75_e1957_d_b7: f64 = (-eq75_e1956_d_b7);
        let eq75_e1957_d_b8: f64 = (-eq75_e1956_d_b8);
        let eq75_e1957_d_b9: f64 = (-eq75_e1956_d_b9);
        let eq75_e1957_d_b10: f64 = (-eq75_e1956_d_b10);
        let eq75_e1957_d_b11: f64 = (-eq75_e1956_d_b11);
        let eq75_e1957_d_b12: f64 = (-eq75_e1956_d_b12);
        let eq75_e1957_d_b13: f64 = (-eq75_e1956_d_b13);
        let eq75_e1957_d_b14: f64 = (-eq75_e1956_d_b14);
        let eq75_e1957_d_b15: f64 = (-eq75_e1956_d_b15);
        let eq75_e1957_d_b16: f64 = (-eq75_e1956_d_b16);
        let eq75_e1957_d_b17: f64 = (-eq75_e1956_d_b17);
        let eq75_e1959: f64 = (eq75_e1957 * s.v[822]);
        let eq75_e1959_d_n0: f64 = ((eq75_e1957_d_n0 * s.v[822]) + (eq75_e1957 * s.dn[822][0]));
        let eq75_e1959_d_n1: f64 = ((eq75_e1957_d_n1 * s.v[822]) + (eq75_e1957 * s.dn[822][1]));
        let eq75_e1959_d_n2: f64 = ((eq75_e1957_d_n2 * s.v[822]) + (eq75_e1957 * s.dn[822][2]));
        let eq75_e1959_d_n3: f64 = ((eq75_e1957_d_n3 * s.v[822]) + (eq75_e1957 * s.dn[822][3]));
        let eq75_e1959_d_n4: f64 = ((eq75_e1957_d_n4 * s.v[822]) + (eq75_e1957 * s.dn[822][4]));
        let eq75_e1959_d_n5: f64 = ((eq75_e1957_d_n5 * s.v[822]) + (eq75_e1957 * s.dn[822][5]));
        let eq75_e1959_d_n6: f64 = ((eq75_e1957_d_n6 * s.v[822]) + (eq75_e1957 * s.dn[822][6]));
        let eq75_e1959_d_n7: f64 = ((eq75_e1957_d_n7 * s.v[822]) + (eq75_e1957 * s.dn[822][7]));
        let eq75_e1959_d_n8: f64 = ((eq75_e1957_d_n8 * s.v[822]) + (eq75_e1957 * s.dn[822][8]));
        let eq75_e1959_d_n9: f64 = ((eq75_e1957_d_n9 * s.v[822]) + (eq75_e1957 * s.dn[822][9]));
        let eq75_e1959_d_n10: f64 = ((eq75_e1957_d_n10 * s.v[822]) + (eq75_e1957 * s.dn[822][10]));
        let eq75_e1959_d_n11: f64 = ((eq75_e1957_d_n11 * s.v[822]) + (eq75_e1957 * s.dn[822][11]));
        let eq75_e1959_d_n12: f64 = ((eq75_e1957_d_n12 * s.v[822]) + (eq75_e1957 * s.dn[822][12]));
        let eq75_e1959_d_n13: f64 = ((eq75_e1957_d_n13 * s.v[822]) + (eq75_e1957 * s.dn[822][13]));
        let eq75_e1959_d_b0: f64 = ((eq75_e1957_d_b0 * s.v[822]) + (eq75_e1957 * s.db[822][0]));
        let eq75_e1959_d_b1: f64 = ((eq75_e1957_d_b1 * s.v[822]) + (eq75_e1957 * s.db[822][1]));
        let eq75_e1959_d_b2: f64 = ((eq75_e1957_d_b2 * s.v[822]) + (eq75_e1957 * s.db[822][2]));
        let eq75_e1959_d_b3: f64 = ((eq75_e1957_d_b3 * s.v[822]) + (eq75_e1957 * s.db[822][3]));
        let eq75_e1959_d_b4: f64 = ((eq75_e1957_d_b4 * s.v[822]) + (eq75_e1957 * s.db[822][4]));
        let eq75_e1959_d_b5: f64 = ((eq75_e1957_d_b5 * s.v[822]) + (eq75_e1957 * s.db[822][5]));
        let eq75_e1959_d_b6: f64 = ((eq75_e1957_d_b6 * s.v[822]) + (eq75_e1957 * s.db[822][6]));
        let eq75_e1959_d_b7: f64 = ((eq75_e1957_d_b7 * s.v[822]) + (eq75_e1957 * s.db[822][7]));
        let eq75_e1959_d_b8: f64 = ((eq75_e1957_d_b8 * s.v[822]) + (eq75_e1957 * s.db[822][8]));
        let eq75_e1959_d_b9: f64 = ((eq75_e1957_d_b9 * s.v[822]) + (eq75_e1957 * s.db[822][9]));
        let eq75_e1959_d_b10: f64 = ((eq75_e1957_d_b10 * s.v[822]) + (eq75_e1957 * s.db[822][10]));
        let eq75_e1959_d_b11: f64 = ((eq75_e1957_d_b11 * s.v[822]) + (eq75_e1957 * s.db[822][11]));
        let eq75_e1959_d_b12: f64 = ((eq75_e1957_d_b12 * s.v[822]) + (eq75_e1957 * s.db[822][12]));
        let eq75_e1959_d_b13: f64 = ((eq75_e1957_d_b13 * s.v[822]) + (eq75_e1957 * s.db[822][13]));
        let eq75_e1959_d_b14: f64 = ((eq75_e1957_d_b14 * s.v[822]) + (eq75_e1957 * s.db[822][14]));
        let eq75_e1959_d_b15: f64 = ((eq75_e1957_d_b15 * s.v[822]) + (eq75_e1957 * s.db[822][15]));
        let eq75_e1959_d_b16: f64 = ((eq75_e1957_d_b16 * s.v[822]) + (eq75_e1957 * s.db[822][16]));
        let eq75_e1959_d_b17: f64 = ((eq75_e1957_d_b17 * s.v[822]) + (eq75_e1957 * s.db[822][17]));
        let eq75_e1962: f64 = (s.v[410] * s.v[158]);
        let eq75_e1962_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq75_e1962_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq75_e1962_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq75_e1962_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq75_e1962_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq75_e1962_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq75_e1962_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq75_e1962_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq75_e1962_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq75_e1962_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq75_e1962_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq75_e1962_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq75_e1962_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq75_e1962_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq75_e1962_d_b0: f64 = (s.db[410][0] * s.v[158]);
        let eq75_e1962_d_b1: f64 = (s.db[410][1] * s.v[158]);
        let eq75_e1962_d_b2: f64 = (s.db[410][2] * s.v[158]);
        let eq75_e1962_d_b3: f64 = (s.db[410][3] * s.v[158]);
        let eq75_e1962_d_b4: f64 = (s.db[410][4] * s.v[158]);
        let eq75_e1962_d_b5: f64 = (s.db[410][5] * s.v[158]);
        let eq75_e1962_d_b6: f64 = (s.db[410][6] * s.v[158]);
        let eq75_e1962_d_b7: f64 = (s.db[410][7] * s.v[158]);
        let eq75_e1962_d_b8: f64 = (s.db[410][8] * s.v[158]);
        let eq75_e1962_d_b9: f64 = (s.db[410][9] * s.v[158]);
        let eq75_e1962_d_b10: f64 = (s.db[410][10] * s.v[158]);
        let eq75_e1962_d_b11: f64 = (s.db[410][11] * s.v[158]);
        let eq75_e1962_d_b12: f64 = (s.db[410][12] * s.v[158]);
        let eq75_e1962_d_b13: f64 = (s.db[410][13] * s.v[158]);
        let eq75_e1962_d_b14: f64 = (s.db[410][14] * s.v[158]);
        let eq75_e1962_d_b15: f64 = (s.db[410][15] * s.v[158]);
        let eq75_e1962_d_b16: f64 = (s.db[410][16] * s.v[158]);
        let eq75_e1962_d_b17: f64 = (s.db[410][17] * s.v[158]);
        let eq75_e1963_q: f64 = eq75_e1962;
        let eq75_e1964: f64 = (eq75_e1959 + eq75_e1962);
        let eq75_e1964_d_n0: f64 = (eq75_e1959_d_n0 + eq75_e1962_d_n0);
        let eq75_e1964_d_n1: f64 = (eq75_e1959_d_n1 + eq75_e1962_d_n1);
        let eq75_e1964_d_n2: f64 = (eq75_e1959_d_n2 + eq75_e1962_d_n2);
        let eq75_e1964_d_n3: f64 = (eq75_e1959_d_n3 + eq75_e1962_d_n3);
        let eq75_e1964_d_n4: f64 = (eq75_e1959_d_n4 + eq75_e1962_d_n4);
        let eq75_e1964_d_n5: f64 = (eq75_e1959_d_n5 + eq75_e1962_d_n5);
        let eq75_e1964_d_n6: f64 = (eq75_e1959_d_n6 + eq75_e1962_d_n6);
        let eq75_e1964_d_n7: f64 = (eq75_e1959_d_n7 + eq75_e1962_d_n7);
        let eq75_e1964_d_n8: f64 = (eq75_e1959_d_n8 + eq75_e1962_d_n8);
        let eq75_e1964_d_n9: f64 = (eq75_e1959_d_n9 + eq75_e1962_d_n9);
        let eq75_e1964_d_n10: f64 = (eq75_e1959_d_n10 + eq75_e1962_d_n10);
        let eq75_e1964_d_n11: f64 = (eq75_e1959_d_n11 + eq75_e1962_d_n11);
        let eq75_e1964_d_n12: f64 = (eq75_e1959_d_n12 + eq75_e1962_d_n12);
        let eq75_e1964_d_n13: f64 = (eq75_e1959_d_n13 + eq75_e1962_d_n13);
        let eq75_e1964_d_b0: f64 = (eq75_e1959_d_b0 + eq75_e1962_d_b0);
        let eq75_e1964_d_b1: f64 = (eq75_e1959_d_b1 + eq75_e1962_d_b1);
        let eq75_e1964_d_b2: f64 = (eq75_e1959_d_b2 + eq75_e1962_d_b2);
        let eq75_e1964_d_b3: f64 = (eq75_e1959_d_b3 + eq75_e1962_d_b3);
        let eq75_e1964_d_b4: f64 = (eq75_e1959_d_b4 + eq75_e1962_d_b4);
        let eq75_e1964_d_b5: f64 = (eq75_e1959_d_b5 + eq75_e1962_d_b5);
        let eq75_e1964_d_b6: f64 = (eq75_e1959_d_b6 + eq75_e1962_d_b6);
        let eq75_e1964_d_b7: f64 = (eq75_e1959_d_b7 + eq75_e1962_d_b7);
        let eq75_e1964_d_b8: f64 = (eq75_e1959_d_b8 + eq75_e1962_d_b8);
        let eq75_e1964_d_b9: f64 = (eq75_e1959_d_b9 + eq75_e1962_d_b9);
        let eq75_e1964_d_b10: f64 = (eq75_e1959_d_b10 + eq75_e1962_d_b10);
        let eq75_e1964_d_b11: f64 = (eq75_e1959_d_b11 + eq75_e1962_d_b11);
        let eq75_e1964_d_b12: f64 = (eq75_e1959_d_b12 + eq75_e1962_d_b12);
        let eq75_e1964_d_b13: f64 = (eq75_e1959_d_b13 + eq75_e1962_d_b13);
        let eq75_e1964_d_b14: f64 = (eq75_e1959_d_b14 + eq75_e1962_d_b14);
        let eq75_e1964_d_b15: f64 = (eq75_e1959_d_b15 + eq75_e1962_d_b15);
        let eq75_e1964_d_b16: f64 = (eq75_e1959_d_b16 + eq75_e1962_d_b16);
        let eq75_e1964_d_b17: f64 = (eq75_e1959_d_b17 + eq75_e1962_d_b17);
        let eq75_e1964_q: f64 = eq75_e1963_q;
        let eq75_e1967: f64 = (s.v[410] / s.v[157]);
        let eq75_e1967_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq75_e1967_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq75_e1967_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq75_e1967_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq75_e1967_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq75_e1967_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq75_e1967_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq75_e1967_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq75_e1967_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq75_e1967_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq75_e1967_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq75_e1967_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq75_e1967_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq75_e1967_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq75_e1967_d_b0: f64 = (s.db[410][0] / s.v[157]);
        let eq75_e1967_d_b1: f64 = (s.db[410][1] / s.v[157]);
        let eq75_e1967_d_b2: f64 = (s.db[410][2] / s.v[157]);
        let eq75_e1967_d_b3: f64 = (s.db[410][3] / s.v[157]);
        let eq75_e1967_d_b4: f64 = (s.db[410][4] / s.v[157]);
        let eq75_e1967_d_b5: f64 = (s.db[410][5] / s.v[157]);
        let eq75_e1967_d_b6: f64 = (s.db[410][6] / s.v[157]);
        let eq75_e1967_d_b7: f64 = (s.db[410][7] / s.v[157]);
        let eq75_e1967_d_b8: f64 = (s.db[410][8] / s.v[157]);
        let eq75_e1967_d_b9: f64 = (s.db[410][9] / s.v[157]);
        let eq75_e1967_d_b10: f64 = (s.db[410][10] / s.v[157]);
        let eq75_e1967_d_b11: f64 = (s.db[410][11] / s.v[157]);
        let eq75_e1967_d_b12: f64 = (s.db[410][12] / s.v[157]);
        let eq75_e1967_d_b13: f64 = (s.db[410][13] / s.v[157]);
        let eq75_e1967_d_b14: f64 = (s.db[410][14] / s.v[157]);
        let eq75_e1967_d_b15: f64 = (s.db[410][15] / s.v[157]);
        let eq75_e1967_d_b16: f64 = (s.db[410][16] / s.v[157]);
        let eq75_e1967_d_b17: f64 = (s.db[410][17] / s.v[157]);
        let eq75_e1968: f64 = (eq75_e1964 + eq75_e1967);
        let eq75_e1968_d_n0: f64 = (eq75_e1964_d_n0 + eq75_e1967_d_n0);
        let eq75_e1968_d_n1: f64 = (eq75_e1964_d_n1 + eq75_e1967_d_n1);
        let eq75_e1968_d_n2: f64 = (eq75_e1964_d_n2 + eq75_e1967_d_n2);
        let eq75_e1968_d_n3: f64 = (eq75_e1964_d_n3 + eq75_e1967_d_n3);
        let eq75_e1968_d_n4: f64 = (eq75_e1964_d_n4 + eq75_e1967_d_n4);
        let eq75_e1968_d_n5: f64 = (eq75_e1964_d_n5 + eq75_e1967_d_n5);
        let eq75_e1968_d_n6: f64 = (eq75_e1964_d_n6 + eq75_e1967_d_n6);
        let eq75_e1968_d_n7: f64 = (eq75_e1964_d_n7 + eq75_e1967_d_n7);
        let eq75_e1968_d_n8: f64 = (eq75_e1964_d_n8 + eq75_e1967_d_n8);
        let eq75_e1968_d_n9: f64 = (eq75_e1964_d_n9 + eq75_e1967_d_n9);
        let eq75_e1968_d_n10: f64 = (eq75_e1964_d_n10 + eq75_e1967_d_n10);
        let eq75_e1968_d_n11: f64 = (eq75_e1964_d_n11 + eq75_e1967_d_n11);
        let eq75_e1968_d_n12: f64 = (eq75_e1964_d_n12 + eq75_e1967_d_n12);
        let eq75_e1968_d_n13: f64 = (eq75_e1964_d_n13 + eq75_e1967_d_n13);
        let eq75_e1968_d_b0: f64 = (eq75_e1964_d_b0 + eq75_e1967_d_b0);
        let eq75_e1968_d_b1: f64 = (eq75_e1964_d_b1 + eq75_e1967_d_b1);
        let eq75_e1968_d_b2: f64 = (eq75_e1964_d_b2 + eq75_e1967_d_b2);
        let eq75_e1968_d_b3: f64 = (eq75_e1964_d_b3 + eq75_e1967_d_b3);
        let eq75_e1968_d_b4: f64 = (eq75_e1964_d_b4 + eq75_e1967_d_b4);
        let eq75_e1968_d_b5: f64 = (eq75_e1964_d_b5 + eq75_e1967_d_b5);
        let eq75_e1968_d_b6: f64 = (eq75_e1964_d_b6 + eq75_e1967_d_b6);
        let eq75_e1968_d_b7: f64 = (eq75_e1964_d_b7 + eq75_e1967_d_b7);
        let eq75_e1968_d_b8: f64 = (eq75_e1964_d_b8 + eq75_e1967_d_b8);
        let eq75_e1968_d_b9: f64 = (eq75_e1964_d_b9 + eq75_e1967_d_b9);
        let eq75_e1968_d_b10: f64 = (eq75_e1964_d_b10 + eq75_e1967_d_b10);
        let eq75_e1968_d_b11: f64 = (eq75_e1964_d_b11 + eq75_e1967_d_b11);
        let eq75_e1968_d_b12: f64 = (eq75_e1964_d_b12 + eq75_e1967_d_b12);
        let eq75_e1968_d_b13: f64 = (eq75_e1964_d_b13 + eq75_e1967_d_b13);
        let eq75_e1968_d_b14: f64 = (eq75_e1964_d_b14 + eq75_e1967_d_b14);
        let eq75_e1968_d_b15: f64 = (eq75_e1964_d_b15 + eq75_e1967_d_b15);
        let eq75_e1968_d_b16: f64 = (eq75_e1964_d_b16 + eq75_e1967_d_b16);
        let eq75_e1968_d_b17: f64 = (eq75_e1964_d_b17 + eq75_e1967_d_b17);
        let eq75_e1968_q: f64 = eq75_e1964_q;
        (eq75_e1968, eq75_e1968_d_n0, eq75_e1968_d_n1, eq75_e1968_d_n2, eq75_e1968_d_n3, eq75_e1968_d_n4, eq75_e1968_d_n5, eq75_e1968_d_n6, eq75_e1968_d_n7, eq75_e1968_d_n8, eq75_e1968_d_n9, eq75_e1968_d_n10, eq75_e1968_d_n11, eq75_e1968_d_n12, eq75_e1968_d_n13, eq75_e1968_d_b0, eq75_e1968_d_b1, eq75_e1968_d_b2, eq75_e1968_d_b3, eq75_e1968_d_b4, eq75_e1968_d_b5, eq75_e1968_d_b6, eq75_e1968_d_b7, eq75_e1968_d_b8, eq75_e1968_d_b9, eq75_e1968_d_b10, eq75_e1968_d_b11, eq75_e1968_d_b12, eq75_e1968_d_b13, eq75_e1968_d_b14, eq75_e1968_d_b15, eq75_e1968_d_b16, eq75_e1968_d_b17, eq75_e1968_q, eq75_e1962_d_n0, eq75_e1962_d_n1, eq75_e1962_d_n2, eq75_e1962_d_n3, eq75_e1962_d_n4, eq75_e1962_d_n5, eq75_e1962_d_n6, eq75_e1962_d_n7, eq75_e1962_d_n8, eq75_e1962_d_n9, eq75_e1962_d_n10, eq75_e1962_d_n11, eq75_e1962_d_n12, eq75_e1962_d_n13, eq75_e1962_d_b0, eq75_e1962_d_b1, eq75_e1962_d_b2, eq75_e1962_d_b3, eq75_e1962_d_b4, eq75_e1962_d_b5, eq75_e1962_d_b6, eq75_e1962_d_b7, eq75_e1962_d_b8, eq75_e1962_d_b9, eq75_e1962_d_b10, eq75_e1962_d_b11, eq75_e1962_d_b12, eq75_e1962_d_b13, eq75_e1962_d_b14, eq75_e1962_d_b15, eq75_e1962_d_b16, eq75_e1962_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_reactive_node_derivatives: [f64; 14] = [eq75_e1970_q_d_n0, eq75_e1970_q_d_n1, eq75_e1970_q_d_n2, eq75_e1970_q_d_n3, eq75_e1970_q_d_n4, eq75_e1970_q_d_n5, eq75_e1970_q_d_n6, eq75_e1970_q_d_n7, eq75_e1970_q_d_n8, eq75_e1970_q_d_n9, eq75_e1970_q_d_n10, eq75_e1970_q_d_n11, eq75_e1970_q_d_n12, eq75_e1970_q_d_n13];
        let eq75_reactive_branch_derivatives: [f64; 18] = [eq75_e1970_q_d_b0, eq75_e1970_q_d_b1, eq75_e1970_q_d_b2, eq75_e1970_q_d_b3, eq75_e1970_q_d_b4, eq75_e1970_q_d_b5, eq75_e1970_q_d_b6, eq75_e1970_q_d_b7, eq75_e1970_q_d_b8, eq75_e1970_q_d_b9, eq75_e1970_q_d_b10, eq75_e1970_q_d_b11, eq75_e1970_q_d_b12, eq75_e1970_q_d_b13, eq75_e1970_q_d_b14, eq75_e1970_q_d_b15, eq75_e1970_q_d_b16, eq75_e1970_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq75_reactive_node_derivatives,
            branches,
            &eq75_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_5(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq76_e1992, eq76_e1992_d_n0, eq76_e1992_d_n1, eq76_e1992_d_n2, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12, eq76_e1992_d_n13, eq76_e1992_d_b0, eq76_e1992_d_b1, eq76_e1992_d_b2, eq76_e1992_d_b3, eq76_e1992_d_b4, eq76_e1992_d_b5, eq76_e1992_d_b6, eq76_e1992_d_b7, eq76_e1992_d_b8, eq76_e1992_d_b9, eq76_e1992_d_b10, eq76_e1992_d_b11, eq76_e1992_d_b12, eq76_e1992_d_b13, eq76_e1992_d_b14, eq76_e1992_d_b15, eq76_e1992_d_b16, eq76_e1992_d_b17, eq76_e1992_q, eq76_e1992_q_d_n0, eq76_e1992_q_d_n1, eq76_e1992_q_d_n2, eq76_e1992_q_d_n3, eq76_e1992_q_d_n4, eq76_e1992_q_d_n5, eq76_e1992_q_d_n6, eq76_e1992_q_d_n7, eq76_e1992_q_d_n8, eq76_e1992_q_d_n9, eq76_e1992_q_d_n10, eq76_e1992_q_d_n11, eq76_e1992_q_d_n12, eq76_e1992_q_d_n13, eq76_e1992_q_d_b0, eq76_e1992_q_d_b1, eq76_e1992_q_d_b2, eq76_e1992_q_d_b3, eq76_e1992_q_d_b4, eq76_e1992_q_d_b5, eq76_e1992_q_d_b6, eq76_e1992_q_d_b7, eq76_e1992_q_d_b8, eq76_e1992_q_d_b9, eq76_e1992_q_d_b10, eq76_e1992_q_d_b11, eq76_e1992_q_d_b12, eq76_e1992_q_d_b13, eq76_e1992_q_d_b14, eq76_e1992_q_d_b15, eq76_e1992_q_d_b16, eq76_e1992_q_d_b17,) = {
    if ((s.b[1559] && (!s.b[1560])) && (!s.b[1564])) {
        let eq76_e1979: f64 = (-s.v[885]);
        let eq76_e1979_d_n0: f64 = (-s.dn[885][0]);
        let eq76_e1979_d_n1: f64 = (-s.dn[885][1]);
        let eq76_e1979_d_n2: f64 = (-s.dn[885][2]);
        let eq76_e1979_d_n3: f64 = (-s.dn[885][3]);
        let eq76_e1979_d_n4: f64 = (-s.dn[885][4]);
        let eq76_e1979_d_n5: f64 = (-s.dn[885][5]);
        let eq76_e1979_d_n6: f64 = (-s.dn[885][6]);
        let eq76_e1979_d_n7: f64 = (-s.dn[885][7]);
        let eq76_e1979_d_n8: f64 = (-s.dn[885][8]);
        let eq76_e1979_d_n9: f64 = (-s.dn[885][9]);
        let eq76_e1979_d_n10: f64 = (-s.dn[885][10]);
        let eq76_e1979_d_n11: f64 = (-s.dn[885][11]);
        let eq76_e1979_d_n12: f64 = (-s.dn[885][12]);
        let eq76_e1979_d_n13: f64 = (-s.dn[885][13]);
        let eq76_e1979_d_b0: f64 = (-s.db[885][0]);
        let eq76_e1979_d_b1: f64 = (-s.db[885][1]);
        let eq76_e1979_d_b2: f64 = (-s.db[885][2]);
        let eq76_e1979_d_b3: f64 = (-s.db[885][3]);
        let eq76_e1979_d_b4: f64 = (-s.db[885][4]);
        let eq76_e1979_d_b5: f64 = (-s.db[885][5]);
        let eq76_e1979_d_b6: f64 = (-s.db[885][6]);
        let eq76_e1979_d_b7: f64 = (-s.db[885][7]);
        let eq76_e1979_d_b8: f64 = (-s.db[885][8]);
        let eq76_e1979_d_b9: f64 = (-s.db[885][9]);
        let eq76_e1979_d_b10: f64 = (-s.db[885][10]);
        let eq76_e1979_d_b11: f64 = (-s.db[885][11]);
        let eq76_e1979_d_b12: f64 = (-s.db[885][12]);
        let eq76_e1979_d_b13: f64 = (-s.db[885][13]);
        let eq76_e1979_d_b14: f64 = (-s.db[885][14]);
        let eq76_e1979_d_b15: f64 = (-s.db[885][15]);
        let eq76_e1979_d_b16: f64 = (-s.db[885][16]);
        let eq76_e1979_d_b17: f64 = (-s.db[885][17]);
        let eq76_e1981: f64 = (eq76_e1979 * s.v[822]);
        let eq76_e1981_d_n0: f64 = ((eq76_e1979_d_n0 * s.v[822]) + (eq76_e1979 * s.dn[822][0]));
        let eq76_e1981_d_n1: f64 = ((eq76_e1979_d_n1 * s.v[822]) + (eq76_e1979 * s.dn[822][1]));
        let eq76_e1981_d_n2: f64 = ((eq76_e1979_d_n2 * s.v[822]) + (eq76_e1979 * s.dn[822][2]));
        let eq76_e1981_d_n3: f64 = ((eq76_e1979_d_n3 * s.v[822]) + (eq76_e1979 * s.dn[822][3]));
        let eq76_e1981_d_n4: f64 = ((eq76_e1979_d_n4 * s.v[822]) + (eq76_e1979 * s.dn[822][4]));
        let eq76_e1981_d_n5: f64 = ((eq76_e1979_d_n5 * s.v[822]) + (eq76_e1979 * s.dn[822][5]));
        let eq76_e1981_d_n6: f64 = ((eq76_e1979_d_n6 * s.v[822]) + (eq76_e1979 * s.dn[822][6]));
        let eq76_e1981_d_n7: f64 = ((eq76_e1979_d_n7 * s.v[822]) + (eq76_e1979 * s.dn[822][7]));
        let eq76_e1981_d_n8: f64 = ((eq76_e1979_d_n8 * s.v[822]) + (eq76_e1979 * s.dn[822][8]));
        let eq76_e1981_d_n9: f64 = ((eq76_e1979_d_n9 * s.v[822]) + (eq76_e1979 * s.dn[822][9]));
        let eq76_e1981_d_n10: f64 = ((eq76_e1979_d_n10 * s.v[822]) + (eq76_e1979 * s.dn[822][10]));
        let eq76_e1981_d_n11: f64 = ((eq76_e1979_d_n11 * s.v[822]) + (eq76_e1979 * s.dn[822][11]));
        let eq76_e1981_d_n12: f64 = ((eq76_e1979_d_n12 * s.v[822]) + (eq76_e1979 * s.dn[822][12]));
        let eq76_e1981_d_n13: f64 = ((eq76_e1979_d_n13 * s.v[822]) + (eq76_e1979 * s.dn[822][13]));
        let eq76_e1981_d_b0: f64 = ((eq76_e1979_d_b0 * s.v[822]) + (eq76_e1979 * s.db[822][0]));
        let eq76_e1981_d_b1: f64 = ((eq76_e1979_d_b1 * s.v[822]) + (eq76_e1979 * s.db[822][1]));
        let eq76_e1981_d_b2: f64 = ((eq76_e1979_d_b2 * s.v[822]) + (eq76_e1979 * s.db[822][2]));
        let eq76_e1981_d_b3: f64 = ((eq76_e1979_d_b3 * s.v[822]) + (eq76_e1979 * s.db[822][3]));
        let eq76_e1981_d_b4: f64 = ((eq76_e1979_d_b4 * s.v[822]) + (eq76_e1979 * s.db[822][4]));
        let eq76_e1981_d_b5: f64 = ((eq76_e1979_d_b5 * s.v[822]) + (eq76_e1979 * s.db[822][5]));
        let eq76_e1981_d_b6: f64 = ((eq76_e1979_d_b6 * s.v[822]) + (eq76_e1979 * s.db[822][6]));
        let eq76_e1981_d_b7: f64 = ((eq76_e1979_d_b7 * s.v[822]) + (eq76_e1979 * s.db[822][7]));
        let eq76_e1981_d_b8: f64 = ((eq76_e1979_d_b8 * s.v[822]) + (eq76_e1979 * s.db[822][8]));
        let eq76_e1981_d_b9: f64 = ((eq76_e1979_d_b9 * s.v[822]) + (eq76_e1979 * s.db[822][9]));
        let eq76_e1981_d_b10: f64 = ((eq76_e1979_d_b10 * s.v[822]) + (eq76_e1979 * s.db[822][10]));
        let eq76_e1981_d_b11: f64 = ((eq76_e1979_d_b11 * s.v[822]) + (eq76_e1979 * s.db[822][11]));
        let eq76_e1981_d_b12: f64 = ((eq76_e1979_d_b12 * s.v[822]) + (eq76_e1979 * s.db[822][12]));
        let eq76_e1981_d_b13: f64 = ((eq76_e1979_d_b13 * s.v[822]) + (eq76_e1979 * s.db[822][13]));
        let eq76_e1981_d_b14: f64 = ((eq76_e1979_d_b14 * s.v[822]) + (eq76_e1979 * s.db[822][14]));
        let eq76_e1981_d_b15: f64 = ((eq76_e1979_d_b15 * s.v[822]) + (eq76_e1979 * s.db[822][15]));
        let eq76_e1981_d_b16: f64 = ((eq76_e1979_d_b16 * s.v[822]) + (eq76_e1979 * s.db[822][16]));
        let eq76_e1981_d_b17: f64 = ((eq76_e1979_d_b17 * s.v[822]) + (eq76_e1979 * s.db[822][17]));
        let eq76_e1984: f64 = (s.v[410] * s.v[158]);
        let eq76_e1984_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq76_e1984_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq76_e1984_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq76_e1984_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq76_e1984_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq76_e1984_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq76_e1984_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq76_e1984_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq76_e1984_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq76_e1984_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq76_e1984_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq76_e1984_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq76_e1984_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq76_e1984_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq76_e1984_d_b0: f64 = (s.db[410][0] * s.v[158]);
        let eq76_e1984_d_b1: f64 = (s.db[410][1] * s.v[158]);
        let eq76_e1984_d_b2: f64 = (s.db[410][2] * s.v[158]);
        let eq76_e1984_d_b3: f64 = (s.db[410][3] * s.v[158]);
        let eq76_e1984_d_b4: f64 = (s.db[410][4] * s.v[158]);
        let eq76_e1984_d_b5: f64 = (s.db[410][5] * s.v[158]);
        let eq76_e1984_d_b6: f64 = (s.db[410][6] * s.v[158]);
        let eq76_e1984_d_b7: f64 = (s.db[410][7] * s.v[158]);
        let eq76_e1984_d_b8: f64 = (s.db[410][8] * s.v[158]);
        let eq76_e1984_d_b9: f64 = (s.db[410][9] * s.v[158]);
        let eq76_e1984_d_b10: f64 = (s.db[410][10] * s.v[158]);
        let eq76_e1984_d_b11: f64 = (s.db[410][11] * s.v[158]);
        let eq76_e1984_d_b12: f64 = (s.db[410][12] * s.v[158]);
        let eq76_e1984_d_b13: f64 = (s.db[410][13] * s.v[158]);
        let eq76_e1984_d_b14: f64 = (s.db[410][14] * s.v[158]);
        let eq76_e1984_d_b15: f64 = (s.db[410][15] * s.v[158]);
        let eq76_e1984_d_b16: f64 = (s.db[410][16] * s.v[158]);
        let eq76_e1984_d_b17: f64 = (s.db[410][17] * s.v[158]);
        let eq76_e1985_q: f64 = eq76_e1984;
        let eq76_e1986: f64 = (eq76_e1981 + eq76_e1984);
        let eq76_e1986_d_n0: f64 = (eq76_e1981_d_n0 + eq76_e1984_d_n0);
        let eq76_e1986_d_n1: f64 = (eq76_e1981_d_n1 + eq76_e1984_d_n1);
        let eq76_e1986_d_n2: f64 = (eq76_e1981_d_n2 + eq76_e1984_d_n2);
        let eq76_e1986_d_n3: f64 = (eq76_e1981_d_n3 + eq76_e1984_d_n3);
        let eq76_e1986_d_n4: f64 = (eq76_e1981_d_n4 + eq76_e1984_d_n4);
        let eq76_e1986_d_n5: f64 = (eq76_e1981_d_n5 + eq76_e1984_d_n5);
        let eq76_e1986_d_n6: f64 = (eq76_e1981_d_n6 + eq76_e1984_d_n6);
        let eq76_e1986_d_n7: f64 = (eq76_e1981_d_n7 + eq76_e1984_d_n7);
        let eq76_e1986_d_n8: f64 = (eq76_e1981_d_n8 + eq76_e1984_d_n8);
        let eq76_e1986_d_n9: f64 = (eq76_e1981_d_n9 + eq76_e1984_d_n9);
        let eq76_e1986_d_n10: f64 = (eq76_e1981_d_n10 + eq76_e1984_d_n10);
        let eq76_e1986_d_n11: f64 = (eq76_e1981_d_n11 + eq76_e1984_d_n11);
        let eq76_e1986_d_n12: f64 = (eq76_e1981_d_n12 + eq76_e1984_d_n12);
        let eq76_e1986_d_n13: f64 = (eq76_e1981_d_n13 + eq76_e1984_d_n13);
        let eq76_e1986_d_b0: f64 = (eq76_e1981_d_b0 + eq76_e1984_d_b0);
        let eq76_e1986_d_b1: f64 = (eq76_e1981_d_b1 + eq76_e1984_d_b1);
        let eq76_e1986_d_b2: f64 = (eq76_e1981_d_b2 + eq76_e1984_d_b2);
        let eq76_e1986_d_b3: f64 = (eq76_e1981_d_b3 + eq76_e1984_d_b3);
        let eq76_e1986_d_b4: f64 = (eq76_e1981_d_b4 + eq76_e1984_d_b4);
        let eq76_e1986_d_b5: f64 = (eq76_e1981_d_b5 + eq76_e1984_d_b5);
        let eq76_e1986_d_b6: f64 = (eq76_e1981_d_b6 + eq76_e1984_d_b6);
        let eq76_e1986_d_b7: f64 = (eq76_e1981_d_b7 + eq76_e1984_d_b7);
        let eq76_e1986_d_b8: f64 = (eq76_e1981_d_b8 + eq76_e1984_d_b8);
        let eq76_e1986_d_b9: f64 = (eq76_e1981_d_b9 + eq76_e1984_d_b9);
        let eq76_e1986_d_b10: f64 = (eq76_e1981_d_b10 + eq76_e1984_d_b10);
        let eq76_e1986_d_b11: f64 = (eq76_e1981_d_b11 + eq76_e1984_d_b11);
        let eq76_e1986_d_b12: f64 = (eq76_e1981_d_b12 + eq76_e1984_d_b12);
        let eq76_e1986_d_b13: f64 = (eq76_e1981_d_b13 + eq76_e1984_d_b13);
        let eq76_e1986_d_b14: f64 = (eq76_e1981_d_b14 + eq76_e1984_d_b14);
        let eq76_e1986_d_b15: f64 = (eq76_e1981_d_b15 + eq76_e1984_d_b15);
        let eq76_e1986_d_b16: f64 = (eq76_e1981_d_b16 + eq76_e1984_d_b16);
        let eq76_e1986_d_b17: f64 = (eq76_e1981_d_b17 + eq76_e1984_d_b17);
        let eq76_e1986_q: f64 = eq76_e1985_q;
        let eq76_e1989: f64 = (s.v[410] / s.v[157]);
        let eq76_e1989_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq76_e1989_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq76_e1989_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq76_e1989_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq76_e1989_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq76_e1989_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq76_e1989_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq76_e1989_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq76_e1989_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq76_e1989_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq76_e1989_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq76_e1989_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq76_e1989_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq76_e1989_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq76_e1989_d_b0: f64 = (s.db[410][0] / s.v[157]);
        let eq76_e1989_d_b1: f64 = (s.db[410][1] / s.v[157]);
        let eq76_e1989_d_b2: f64 = (s.db[410][2] / s.v[157]);
        let eq76_e1989_d_b3: f64 = (s.db[410][3] / s.v[157]);
        let eq76_e1989_d_b4: f64 = (s.db[410][4] / s.v[157]);
        let eq76_e1989_d_b5: f64 = (s.db[410][5] / s.v[157]);
        let eq76_e1989_d_b6: f64 = (s.db[410][6] / s.v[157]);
        let eq76_e1989_d_b7: f64 = (s.db[410][7] / s.v[157]);
        let eq76_e1989_d_b8: f64 = (s.db[410][8] / s.v[157]);
        let eq76_e1989_d_b9: f64 = (s.db[410][9] / s.v[157]);
        let eq76_e1989_d_b10: f64 = (s.db[410][10] / s.v[157]);
        let eq76_e1989_d_b11: f64 = (s.db[410][11] / s.v[157]);
        let eq76_e1989_d_b12: f64 = (s.db[410][12] / s.v[157]);
        let eq76_e1989_d_b13: f64 = (s.db[410][13] / s.v[157]);
        let eq76_e1989_d_b14: f64 = (s.db[410][14] / s.v[157]);
        let eq76_e1989_d_b15: f64 = (s.db[410][15] / s.v[157]);
        let eq76_e1989_d_b16: f64 = (s.db[410][16] / s.v[157]);
        let eq76_e1989_d_b17: f64 = (s.db[410][17] / s.v[157]);
        let eq76_e1990: f64 = (eq76_e1986 + eq76_e1989);
        let eq76_e1990_d_n0: f64 = (eq76_e1986_d_n0 + eq76_e1989_d_n0);
        let eq76_e1990_d_n1: f64 = (eq76_e1986_d_n1 + eq76_e1989_d_n1);
        let eq76_e1990_d_n2: f64 = (eq76_e1986_d_n2 + eq76_e1989_d_n2);
        let eq76_e1990_d_n3: f64 = (eq76_e1986_d_n3 + eq76_e1989_d_n3);
        let eq76_e1990_d_n4: f64 = (eq76_e1986_d_n4 + eq76_e1989_d_n4);
        let eq76_e1990_d_n5: f64 = (eq76_e1986_d_n5 + eq76_e1989_d_n5);
        let eq76_e1990_d_n6: f64 = (eq76_e1986_d_n6 + eq76_e1989_d_n6);
        let eq76_e1990_d_n7: f64 = (eq76_e1986_d_n7 + eq76_e1989_d_n7);
        let eq76_e1990_d_n8: f64 = (eq76_e1986_d_n8 + eq76_e1989_d_n8);
        let eq76_e1990_d_n9: f64 = (eq76_e1986_d_n9 + eq76_e1989_d_n9);
        let eq76_e1990_d_n10: f64 = (eq76_e1986_d_n10 + eq76_e1989_d_n10);
        let eq76_e1990_d_n11: f64 = (eq76_e1986_d_n11 + eq76_e1989_d_n11);
        let eq76_e1990_d_n12: f64 = (eq76_e1986_d_n12 + eq76_e1989_d_n12);
        let eq76_e1990_d_n13: f64 = (eq76_e1986_d_n13 + eq76_e1989_d_n13);
        let eq76_e1990_d_b0: f64 = (eq76_e1986_d_b0 + eq76_e1989_d_b0);
        let eq76_e1990_d_b1: f64 = (eq76_e1986_d_b1 + eq76_e1989_d_b1);
        let eq76_e1990_d_b2: f64 = (eq76_e1986_d_b2 + eq76_e1989_d_b2);
        let eq76_e1990_d_b3: f64 = (eq76_e1986_d_b3 + eq76_e1989_d_b3);
        let eq76_e1990_d_b4: f64 = (eq76_e1986_d_b4 + eq76_e1989_d_b4);
        let eq76_e1990_d_b5: f64 = (eq76_e1986_d_b5 + eq76_e1989_d_b5);
        let eq76_e1990_d_b6: f64 = (eq76_e1986_d_b6 + eq76_e1989_d_b6);
        let eq76_e1990_d_b7: f64 = (eq76_e1986_d_b7 + eq76_e1989_d_b7);
        let eq76_e1990_d_b8: f64 = (eq76_e1986_d_b8 + eq76_e1989_d_b8);
        let eq76_e1990_d_b9: f64 = (eq76_e1986_d_b9 + eq76_e1989_d_b9);
        let eq76_e1990_d_b10: f64 = (eq76_e1986_d_b10 + eq76_e1989_d_b10);
        let eq76_e1990_d_b11: f64 = (eq76_e1986_d_b11 + eq76_e1989_d_b11);
        let eq76_e1990_d_b12: f64 = (eq76_e1986_d_b12 + eq76_e1989_d_b12);
        let eq76_e1990_d_b13: f64 = (eq76_e1986_d_b13 + eq76_e1989_d_b13);
        let eq76_e1990_d_b14: f64 = (eq76_e1986_d_b14 + eq76_e1989_d_b14);
        let eq76_e1990_d_b15: f64 = (eq76_e1986_d_b15 + eq76_e1989_d_b15);
        let eq76_e1990_d_b16: f64 = (eq76_e1986_d_b16 + eq76_e1989_d_b16);
        let eq76_e1990_d_b17: f64 = (eq76_e1986_d_b17 + eq76_e1989_d_b17);
        let eq76_e1990_q: f64 = eq76_e1986_q;
        (eq76_e1990, eq76_e1990_d_n0, eq76_e1990_d_n1, eq76_e1990_d_n2, eq76_e1990_d_n3, eq76_e1990_d_n4, eq76_e1990_d_n5, eq76_e1990_d_n6, eq76_e1990_d_n7, eq76_e1990_d_n8, eq76_e1990_d_n9, eq76_e1990_d_n10, eq76_e1990_d_n11, eq76_e1990_d_n12, eq76_e1990_d_n13, eq76_e1990_d_b0, eq76_e1990_d_b1, eq76_e1990_d_b2, eq76_e1990_d_b3, eq76_e1990_d_b4, eq76_e1990_d_b5, eq76_e1990_d_b6, eq76_e1990_d_b7, eq76_e1990_d_b8, eq76_e1990_d_b9, eq76_e1990_d_b10, eq76_e1990_d_b11, eq76_e1990_d_b12, eq76_e1990_d_b13, eq76_e1990_d_b14, eq76_e1990_d_b15, eq76_e1990_d_b16, eq76_e1990_d_b17, eq76_e1990_q, eq76_e1984_d_n0, eq76_e1984_d_n1, eq76_e1984_d_n2, eq76_e1984_d_n3, eq76_e1984_d_n4, eq76_e1984_d_n5, eq76_e1984_d_n6, eq76_e1984_d_n7, eq76_e1984_d_n8, eq76_e1984_d_n9, eq76_e1984_d_n10, eq76_e1984_d_n11, eq76_e1984_d_n12, eq76_e1984_d_n13, eq76_e1984_d_b0, eq76_e1984_d_b1, eq76_e1984_d_b2, eq76_e1984_d_b3, eq76_e1984_d_b4, eq76_e1984_d_b5, eq76_e1984_d_b6, eq76_e1984_d_b7, eq76_e1984_d_b8, eq76_e1984_d_b9, eq76_e1984_d_b10, eq76_e1984_d_b11, eq76_e1984_d_b12, eq76_e1984_d_b13, eq76_e1984_d_b14, eq76_e1984_d_b15, eq76_e1984_d_b16, eq76_e1984_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 14] = [eq76_e1992_q_d_n0, eq76_e1992_q_d_n1, eq76_e1992_q_d_n2, eq76_e1992_q_d_n3, eq76_e1992_q_d_n4, eq76_e1992_q_d_n5, eq76_e1992_q_d_n6, eq76_e1992_q_d_n7, eq76_e1992_q_d_n8, eq76_e1992_q_d_n9, eq76_e1992_q_d_n10, eq76_e1992_q_d_n11, eq76_e1992_q_d_n12, eq76_e1992_q_d_n13];
        let eq76_reactive_branch_derivatives: [f64; 18] = [eq76_e1992_q_d_b0, eq76_e1992_q_d_b1, eq76_e1992_q_d_b2, eq76_e1992_q_d_b3, eq76_e1992_q_d_b4, eq76_e1992_q_d_b5, eq76_e1992_q_d_b6, eq76_e1992_q_d_b7, eq76_e1992_q_d_b8, eq76_e1992_q_d_b9, eq76_e1992_q_d_b10, eq76_e1992_q_d_b11, eq76_e1992_q_d_b12, eq76_e1992_q_d_b13, eq76_e1992_q_d_b14, eq76_e1992_q_d_b15, eq76_e1992_q_d_b16, eq76_e1992_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
