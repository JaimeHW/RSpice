#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq20_e409: f64 = ((nv14 - 0.0) * s.v[618]);
        let eq20_e409_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);
        let eq20_e409_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);
        let eq20_e409_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);
        let eq20_e409_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);
        let eq20_e409_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);
        let eq20_e409_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);
        let eq20_e409_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);
        let eq20_e409_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);
        let eq20_e409_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);
        let eq20_e409_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);
        let eq20_e409_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);
        let eq20_e409_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);
        let eq20_e409_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);
        let eq20_e409_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);
        let eq20_e409_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));
        let eq20_e409_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);
        let eq20_e409_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);
        let eq20_e409_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);
        let eq20_e409_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);
        let eq20_e409_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);
        let eq20_e409_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);
        let eq20_e409_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);
        let eq20_e409_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);
        let eq20_e409_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);
        let eq20_e409_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);
        let eq20_e409_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);
        let eq20_e409_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);
        let eq20_e409_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);
        let eq20_e409_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);
        let eq20_e409_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);
        let eq20_e409_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);
        let eq20_e409_d_b12: f64 = ((nv14 - 0.0) * s.db[618][12]);
        let eq20_e410_q: f64 = eq20_e409;
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e409_d_n0, eq20_e409_d_n1, eq20_e409_d_n2, eq20_e409_d_n3, eq20_e409_d_n4, eq20_e409_d_n5, eq20_e409_d_n6, eq20_e409_d_n7, eq20_e409_d_n8, eq20_e409_d_n9, eq20_e409_d_n10, eq20_e409_d_n11, eq20_e409_d_n12, eq20_e409_d_n13, eq20_e409_d_n14, eq20_e409_d_n15, eq20_e409_d_n16, eq20_e409_d_n17, eq20_e409_d_n18];
        let eq20_reactive_branch_derivatives: [f64; 13] = [eq20_e409_d_b0, eq20_e409_d_b1, eq20_e409_d_b2, eq20_e409_d_b3, eq20_e409_d_b4, eq20_e409_d_b5, eq20_e409_d_b6, eq20_e409_d_b7, eq20_e409_d_b8, eq20_e409_d_b9, eq20_e409_d_b10, eq20_e409_d_b11, eq20_e409_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            &nodes,
            &eq20_reactive_node_derivatives,
            &branches,
            &eq20_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_31_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq31_e491, eq31_e491_d_n0, eq31_e491_d_n1, eq31_e491_d_n2, eq31_e491_d_n3, eq31_e491_d_n4, eq31_e491_d_n5, eq31_e491_d_n6, eq31_e491_d_n7, eq31_e491_d_n8, eq31_e491_d_n9, eq31_e491_d_n10, eq31_e491_d_n11, eq31_e491_d_n12, eq31_e491_d_n13, eq31_e491_d_n14, eq31_e491_d_n15, eq31_e491_d_n16, eq31_e491_d_n17, eq31_e491_d_n18, eq31_e491_d_b0, eq31_e491_d_b1, eq31_e491_d_b2, eq31_e491_d_b3, eq31_e491_d_b4, eq31_e491_d_b5, eq31_e491_d_b6, eq31_e491_d_b7, eq31_e491_d_b8, eq31_e491_d_b9, eq31_e491_d_b10, eq31_e491_d_b11, eq31_e491_d_b12, eq31_e491_q, eq31_e491_q_d_n0, eq31_e491_q_d_n1, eq31_e491_q_d_n2, eq31_e491_q_d_n3, eq31_e491_q_d_n4, eq31_e491_q_d_n5, eq31_e491_q_d_n6, eq31_e491_q_d_n7, eq31_e491_q_d_n8, eq31_e491_q_d_n9, eq31_e491_q_d_n10, eq31_e491_q_d_n11, eq31_e491_q_d_n12, eq31_e491_q_d_n13, eq31_e491_q_d_n14, eq31_e491_q_d_n15, eq31_e491_q_d_n16, eq31_e491_q_d_n17, eq31_e491_q_d_n18, eq31_e491_q_d_b0, eq31_e491_q_d_b1, eq31_e491_q_d_b2, eq31_e491_q_d_b3, eq31_e491_q_d_b4, eq31_e491_q_d_b5, eq31_e491_q_d_b6, eq31_e491_q_d_b7, eq31_e491_q_d_b8, eq31_e491_q_d_b9, eq31_e491_q_d_b10, eq31_e491_q_d_b11, eq31_e491_q_d_b12,) = {
    if (s.v[1850] != 0.0) {
        let eq31_e488: f64 = (s.v[563] * (nv10 - 0.0));
        let eq31_e488_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));
        let eq31_e488_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));
        let eq31_e488_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));
        let eq31_e488_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));
        let eq31_e488_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));
        let eq31_e488_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));
        let eq31_e488_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));
        let eq31_e488_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));
        let eq31_e488_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));
        let eq31_e488_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));
        let eq31_e488_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);
        let eq31_e488_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));
        let eq31_e488_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));
        let eq31_e488_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));
        let eq31_e488_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));
        let eq31_e488_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));
        let eq31_e488_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));
        let eq31_e488_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));
        let eq31_e488_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));
        let eq31_e488_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));
        let eq31_e488_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));
        let eq31_e488_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));
        let eq31_e488_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));
        let eq31_e488_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));
        let eq31_e488_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));
        let eq31_e488_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));
        let eq31_e488_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));
        let eq31_e488_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));
        let eq31_e488_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));
        let eq31_e488_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));
        let eq31_e488_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));
        let eq31_e488_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));
        let eq31_e489_q: f64 = eq31_e488;
        (eq31_e488, eq31_e488_d_n0, eq31_e488_d_n1, eq31_e488_d_n2, eq31_e488_d_n3, eq31_e488_d_n4, eq31_e488_d_n5, eq31_e488_d_n6, eq31_e488_d_n7, eq31_e488_d_n8, eq31_e488_d_n9, eq31_e488_d_n10, eq31_e488_d_n11, eq31_e488_d_n12, eq31_e488_d_n13, eq31_e488_d_n14, eq31_e488_d_n15, eq31_e488_d_n16, eq31_e488_d_n17, eq31_e488_d_n18, eq31_e488_d_b0, eq31_e488_d_b1, eq31_e488_d_b2, eq31_e488_d_b3, eq31_e488_d_b4, eq31_e488_d_b5, eq31_e488_d_b6, eq31_e488_d_b7, eq31_e488_d_b8, eq31_e488_d_b9, eq31_e488_d_b10, eq31_e488_d_b11, eq31_e488_d_b12, eq31_e489_q, eq31_e488_d_n0, eq31_e488_d_n1, eq31_e488_d_n2, eq31_e488_d_n3, eq31_e488_d_n4, eq31_e488_d_n5, eq31_e488_d_n6, eq31_e488_d_n7, eq31_e488_d_n8, eq31_e488_d_n9, eq31_e488_d_n10, eq31_e488_d_n11, eq31_e488_d_n12, eq31_e488_d_n13, eq31_e488_d_n14, eq31_e488_d_n15, eq31_e488_d_n16, eq31_e488_d_n17, eq31_e488_d_n18, eq31_e488_d_b0, eq31_e488_d_b1, eq31_e488_d_b2, eq31_e488_d_b3, eq31_e488_d_b4, eq31_e488_d_b5, eq31_e488_d_b6, eq31_e488_d_b7, eq31_e488_d_b8, eq31_e488_d_b9, eq31_e488_d_b10, eq31_e488_d_b11, eq31_e488_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_reactive_node_derivatives: [f64; 19] = [eq31_e491_q_d_n0, eq31_e491_q_d_n1, eq31_e491_q_d_n2, eq31_e491_q_d_n3, eq31_e491_q_d_n4, eq31_e491_q_d_n5, eq31_e491_q_d_n6, eq31_e491_q_d_n7, eq31_e491_q_d_n8, eq31_e491_q_d_n9, eq31_e491_q_d_n10, eq31_e491_q_d_n11, eq31_e491_q_d_n12, eq31_e491_q_d_n13, eq31_e491_q_d_n14, eq31_e491_q_d_n15, eq31_e491_q_d_n16, eq31_e491_q_d_n17, eq31_e491_q_d_n18];
        let eq31_reactive_branch_derivatives: [f64; 13] = [eq31_e491_q_d_b0, eq31_e491_q_d_b1, eq31_e491_q_d_b2, eq31_e491_q_d_b3, eq31_e491_q_d_b4, eq31_e491_q_d_b5, eq31_e491_q_d_b6, eq31_e491_q_d_b7, eq31_e491_q_d_b8, eq31_e491_q_d_b9, eq31_e491_q_d_b10, eq31_e491_q_d_b11, eq31_e491_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            None,
            &nodes,
            &eq31_reactive_node_derivatives,
            &branches,
            &eq31_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n1, eq35_e523_d_n2, eq35_e523_d_n3, eq35_e523_d_n4, eq35_e523_d_n5, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n8, eq35_e523_d_n9, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n13, eq35_e523_d_n14, eq35_e523_d_n15, eq35_e523_d_n16, eq35_e523_d_n17, eq35_e523_d_n18, eq35_e523_d_b0, eq35_e523_d_b1, eq35_e523_d_b2, eq35_e523_d_b3, eq35_e523_d_b4, eq35_e523_d_b5, eq35_e523_d_b6, eq35_e523_d_b7, eq35_e523_d_b8, eq35_e523_d_b9, eq35_e523_d_b10, eq35_e523_d_b11, eq35_e523_d_b12, eq35_e523_q, eq35_e523_q_d_n0, eq35_e523_q_d_n1, eq35_e523_q_d_n2, eq35_e523_q_d_n3, eq35_e523_q_d_n4, eq35_e523_q_d_n5, eq35_e523_q_d_n6, eq35_e523_q_d_n7, eq35_e523_q_d_n8, eq35_e523_q_d_n9, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, eq35_e523_q_d_n13, eq35_e523_q_d_n14, eq35_e523_q_d_n15, eq35_e523_q_d_n16, eq35_e523_q_d_n17, eq35_e523_q_d_n18, eq35_e523_q_d_b0, eq35_e523_q_d_b1, eq35_e523_q_d_b2, eq35_e523_q_d_b3, eq35_e523_q_d_b4, eq35_e523_q_d_b5, eq35_e523_q_d_b6, eq35_e523_q_d_b7, eq35_e523_q_d_b8, eq35_e523_q_d_b9, eq35_e523_q_d_b10, eq35_e523_q_d_b11, eq35_e523_q_d_b12,) = {
    if (s.v[1851] != 0.0) {
        let eq35_e519_q: f64 = s.v[283];
        let eq35_e520: f64 = (s.v[281] + s.v[283]);
        let eq35_e520_d_n0: f64 = (s.dn[281][0] + s.dn[283][0]);
        let eq35_e520_d_n1: f64 = (s.dn[281][1] + s.dn[283][1]);
        let eq35_e520_d_n2: f64 = (s.dn[281][2] + s.dn[283][2]);
        let eq35_e520_d_n3: f64 = (s.dn[281][3] + s.dn[283][3]);
        let eq35_e520_d_n4: f64 = (s.dn[281][4] + s.dn[283][4]);
        let eq35_e520_d_n5: f64 = (s.dn[281][5] + s.dn[283][5]);
        let eq35_e520_d_n6: f64 = (s.dn[281][6] + s.dn[283][6]);
        let eq35_e520_d_n7: f64 = (s.dn[281][7] + s.dn[283][7]);
        let eq35_e520_d_n8: f64 = (s.dn[281][8] + s.dn[283][8]);
        let eq35_e520_d_n9: f64 = (s.dn[281][9] + s.dn[283][9]);
        let eq35_e520_d_n10: f64 = (s.dn[281][10] + s.dn[283][10]);
        let eq35_e520_d_n11: f64 = (s.dn[281][11] + s.dn[283][11]);
        let eq35_e520_d_n12: f64 = (s.dn[281][12] + s.dn[283][12]);
        let eq35_e520_d_n13: f64 = (s.dn[281][13] + s.dn[283][13]);
        let eq35_e520_d_n14: f64 = (s.dn[281][14] + s.dn[283][14]);
        let eq35_e520_d_n15: f64 = (s.dn[281][15] + s.dn[283][15]);
        let eq35_e520_d_n16: f64 = (s.dn[281][16] + s.dn[283][16]);
        let eq35_e520_d_n17: f64 = (s.dn[281][17] + s.dn[283][17]);
        let eq35_e520_d_n18: f64 = (s.dn[281][18] + s.dn[283][18]);
        let eq35_e520_d_b0: f64 = (s.db[281][0] + s.db[283][0]);
        let eq35_e520_d_b1: f64 = (s.db[281][1] + s.db[283][1]);
        let eq35_e520_d_b2: f64 = (s.db[281][2] + s.db[283][2]);
        let eq35_e520_d_b3: f64 = (s.db[281][3] + s.db[283][3]);
        let eq35_e520_d_b4: f64 = (s.db[281][4] + s.db[283][4]);
        let eq35_e520_d_b5: f64 = (s.db[281][5] + s.db[283][5]);
        let eq35_e520_d_b6: f64 = (s.db[281][6] + s.db[283][6]);
        let eq35_e520_d_b7: f64 = (s.db[281][7] + s.db[283][7]);
        let eq35_e520_d_b8: f64 = (s.db[281][8] + s.db[283][8]);
        let eq35_e520_d_b9: f64 = (s.db[281][9] + s.db[283][9]);
        let eq35_e520_d_b10: f64 = (s.db[281][10] + s.db[283][10]);
        let eq35_e520_d_b11: f64 = (s.db[281][11] + s.db[283][11]);
        let eq35_e520_d_b12: f64 = (s.db[281][12] + s.db[283][12]);
        let eq35_e520_q: f64 = eq35_e519_q;
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n1: f64 = (p.p50 * eq35_e520_d_n1);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n3: f64 = (p.p50 * eq35_e520_d_n3);
        let eq35_e521_d_n4: f64 = (p.p50 * eq35_e520_d_n4);
        let eq35_e521_d_n5: f64 = (p.p50 * eq35_e520_d_n5);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n8: f64 = (p.p50 * eq35_e520_d_n8);
        let eq35_e521_d_n9: f64 = (p.p50 * eq35_e520_d_n9);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n13: f64 = (p.p50 * eq35_e520_d_n13);
        let eq35_e521_d_n14: f64 = (p.p50 * eq35_e520_d_n14);
        let eq35_e521_d_n15: f64 = (p.p50 * eq35_e520_d_n15);
        let eq35_e521_d_n16: f64 = (p.p50 * eq35_e520_d_n16);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        let eq35_e521_d_n18: f64 = (p.p50 * eq35_e520_d_n18);
        let eq35_e521_d_b0: f64 = (p.p50 * eq35_e520_d_b0);
        let eq35_e521_d_b1: f64 = (p.p50 * eq35_e520_d_b1);
        let eq35_e521_d_b2: f64 = (p.p50 * eq35_e520_d_b2);
        let eq35_e521_d_b3: f64 = (p.p50 * eq35_e520_d_b3);
        let eq35_e521_d_b4: f64 = (p.p50 * eq35_e520_d_b4);
        let eq35_e521_d_b5: f64 = (p.p50 * eq35_e520_d_b5);
        let eq35_e521_d_b6: f64 = (p.p50 * eq35_e520_d_b6);
        let eq35_e521_d_b7: f64 = (p.p50 * eq35_e520_d_b7);
        let eq35_e521_d_b8: f64 = (p.p50 * eq35_e520_d_b8);
        let eq35_e521_d_b9: f64 = (p.p50 * eq35_e520_d_b9);
        let eq35_e521_d_b10: f64 = (p.p50 * eq35_e520_d_b10);
        let eq35_e521_d_b11: f64 = (p.p50 * eq35_e520_d_b11);
        let eq35_e521_d_b12: f64 = (p.p50 * eq35_e520_d_b12);
        let eq35_e521_q: f64 = (p.p50 * eq35_e520_q);
        let eq35_e521_q_d_n0: f64 = (p.p50 * s.dn[283][0]);
        let eq35_e521_q_d_n1: f64 = (p.p50 * s.dn[283][1]);
        let eq35_e521_q_d_n2: f64 = (p.p50 * s.dn[283][2]);
        let eq35_e521_q_d_n3: f64 = (p.p50 * s.dn[283][3]);
        let eq35_e521_q_d_n4: f64 = (p.p50 * s.dn[283][4]);
        let eq35_e521_q_d_n5: f64 = (p.p50 * s.dn[283][5]);
        let eq35_e521_q_d_n6: f64 = (p.p50 * s.dn[283][6]);
        let eq35_e521_q_d_n7: f64 = (p.p50 * s.dn[283][7]);
        let eq35_e521_q_d_n8: f64 = (p.p50 * s.dn[283][8]);
        let eq35_e521_q_d_n9: f64 = (p.p50 * s.dn[283][9]);
        let eq35_e521_q_d_n10: f64 = (p.p50 * s.dn[283][10]);
        let eq35_e521_q_d_n11: f64 = (p.p50 * s.dn[283][11]);
        let eq35_e521_q_d_n12: f64 = (p.p50 * s.dn[283][12]);
        let eq35_e521_q_d_n13: f64 = (p.p50 * s.dn[283][13]);
        let eq35_e521_q_d_n14: f64 = (p.p50 * s.dn[283][14]);
        let eq35_e521_q_d_n15: f64 = (p.p50 * s.dn[283][15]);
        let eq35_e521_q_d_n16: f64 = (p.p50 * s.dn[283][16]);
        let eq35_e521_q_d_n17: f64 = (p.p50 * s.dn[283][17]);
        let eq35_e521_q_d_n18: f64 = (p.p50 * s.dn[283][18]);
        let eq35_e521_q_d_b0: f64 = (p.p50 * s.db[283][0]);
        let eq35_e521_q_d_b1: f64 = (p.p50 * s.db[283][1]);
        let eq35_e521_q_d_b2: f64 = (p.p50 * s.db[283][2]);
        let eq35_e521_q_d_b3: f64 = (p.p50 * s.db[283][3]);
        let eq35_e521_q_d_b4: f64 = (p.p50 * s.db[283][4]);
        let eq35_e521_q_d_b5: f64 = (p.p50 * s.db[283][5]);
        let eq35_e521_q_d_b6: f64 = (p.p50 * s.db[283][6]);
        let eq35_e521_q_d_b7: f64 = (p.p50 * s.db[283][7]);
        let eq35_e521_q_d_b8: f64 = (p.p50 * s.db[283][8]);
        let eq35_e521_q_d_b9: f64 = (p.p50 * s.db[283][9]);
        let eq35_e521_q_d_b10: f64 = (p.p50 * s.db[283][10]);
        let eq35_e521_q_d_b11: f64 = (p.p50 * s.db[283][11]);
        let eq35_e521_q_d_b12: f64 = (p.p50 * s.db[283][12]);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n1, eq35_e521_d_n2, eq35_e521_d_n3, eq35_e521_d_n4, eq35_e521_d_n5, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n8, eq35_e521_d_n9, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n13, eq35_e521_d_n14, eq35_e521_d_n15, eq35_e521_d_n16, eq35_e521_d_n17, eq35_e521_d_n18, eq35_e521_d_b0, eq35_e521_d_b1, eq35_e521_d_b2, eq35_e521_d_b3, eq35_e521_d_b4, eq35_e521_d_b5, eq35_e521_d_b6, eq35_e521_d_b7, eq35_e521_d_b8, eq35_e521_d_b9, eq35_e521_d_b10, eq35_e521_d_b11, eq35_e521_d_b12, eq35_e521_q, eq35_e521_q_d_n0, eq35_e521_q_d_n1, eq35_e521_q_d_n2, eq35_e521_q_d_n3, eq35_e521_q_d_n4, eq35_e521_q_d_n5, eq35_e521_q_d_n6, eq35_e521_q_d_n7, eq35_e521_q_d_n8, eq35_e521_q_d_n9, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, eq35_e521_q_d_n13, eq35_e521_q_d_n14, eq35_e521_q_d_n15, eq35_e521_q_d_n16, eq35_e521_q_d_n17, eq35_e521_q_d_n18, eq35_e521_q_d_b0, eq35_e521_q_d_b1, eq35_e521_q_d_b2, eq35_e521_q_d_b3, eq35_e521_q_d_b4, eq35_e521_q_d_b5, eq35_e521_q_d_b6, eq35_e521_q_d_b7, eq35_e521_q_d_b8, eq35_e521_q_d_b9, eq35_e521_q_d_b10, eq35_e521_q_d_b11, eq35_e521_q_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e523_q_d_n0, eq35_e523_q_d_n1, eq35_e523_q_d_n2, eq35_e523_q_d_n3, eq35_e523_q_d_n4, eq35_e523_q_d_n5, eq35_e523_q_d_n6, eq35_e523_q_d_n7, eq35_e523_q_d_n8, eq35_e523_q_d_n9, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, eq35_e523_q_d_n13, eq35_e523_q_d_n14, eq35_e523_q_d_n15, eq35_e523_q_d_n16, eq35_e523_q_d_n17, eq35_e523_q_d_n18];
        let eq35_reactive_branch_derivatives: [f64; 13] = [eq35_e523_q_d_b0, eq35_e523_q_d_b1, eq35_e523_q_d_b2, eq35_e523_q_d_b3, eq35_e523_q_d_b4, eq35_e523_q_d_b5, eq35_e523_q_d_b6, eq35_e523_q_d_b7, eq35_e523_q_d_b8, eq35_e523_q_d_b9, eq35_e523_q_d_b10, eq35_e523_q_d_b11, eq35_e523_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq35_reactive_node_derivatives,
            &branches,
            &eq35_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n1, eq36_e532_d_n2, eq36_e532_d_n3, eq36_e532_d_n4, eq36_e532_d_n5, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n8, eq36_e532_d_n9, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n13, eq36_e532_d_n14, eq36_e532_d_n15, eq36_e532_d_n16, eq36_e532_d_n17, eq36_e532_d_n18, eq36_e532_d_b0, eq36_e532_d_b1, eq36_e532_d_b2, eq36_e532_d_b3, eq36_e532_d_b4, eq36_e532_d_b5, eq36_e532_d_b6, eq36_e532_d_b7, eq36_e532_d_b8, eq36_e532_d_b9, eq36_e532_d_b10, eq36_e532_d_b11, eq36_e532_d_b12, eq36_e532_q, eq36_e532_q_d_n0, eq36_e532_q_d_n1, eq36_e532_q_d_n2, eq36_e532_q_d_n3, eq36_e532_q_d_n4, eq36_e532_q_d_n5, eq36_e532_q_d_n6, eq36_e532_q_d_n7, eq36_e532_q_d_n8, eq36_e532_q_d_n9, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, eq36_e532_q_d_n13, eq36_e532_q_d_n14, eq36_e532_q_d_n15, eq36_e532_q_d_n16, eq36_e532_q_d_n17, eq36_e532_q_d_n18, eq36_e532_q_d_b0, eq36_e532_q_d_b1, eq36_e532_q_d_b2, eq36_e532_q_d_b3, eq36_e532_q_d_b4, eq36_e532_q_d_b5, eq36_e532_q_d_b6, eq36_e532_q_d_b7, eq36_e532_q_d_b8, eq36_e532_q_d_b9, eq36_e532_q_d_b10, eq36_e532_q_d_b11, eq36_e532_q_d_b12,) = {
    if (s.v[1851] != 0.0) {
        let eq36_e528_q: f64 = s.v[284];
        let eq36_e529: f64 = (s.v[282] + s.v[284]);
        let eq36_e529_d_n0: f64 = (s.dn[282][0] + s.dn[284][0]);
        let eq36_e529_d_n1: f64 = (s.dn[282][1] + s.dn[284][1]);
        let eq36_e529_d_n2: f64 = (s.dn[282][2] + s.dn[284][2]);
        let eq36_e529_d_n3: f64 = (s.dn[282][3] + s.dn[284][3]);
        let eq36_e529_d_n4: f64 = (s.dn[282][4] + s.dn[284][4]);
        let eq36_e529_d_n5: f64 = (s.dn[282][5] + s.dn[284][5]);
        let eq36_e529_d_n6: f64 = (s.dn[282][6] + s.dn[284][6]);
        let eq36_e529_d_n7: f64 = (s.dn[282][7] + s.dn[284][7]);
        let eq36_e529_d_n8: f64 = (s.dn[282][8] + s.dn[284][8]);
        let eq36_e529_d_n9: f64 = (s.dn[282][9] + s.dn[284][9]);
        let eq36_e529_d_n10: f64 = (s.dn[282][10] + s.dn[284][10]);
        let eq36_e529_d_n11: f64 = (s.dn[282][11] + s.dn[284][11]);
        let eq36_e529_d_n12: f64 = (s.dn[282][12] + s.dn[284][12]);
        let eq36_e529_d_n13: f64 = (s.dn[282][13] + s.dn[284][13]);
        let eq36_e529_d_n14: f64 = (s.dn[282][14] + s.dn[284][14]);
        let eq36_e529_d_n15: f64 = (s.dn[282][15] + s.dn[284][15]);
        let eq36_e529_d_n16: f64 = (s.dn[282][16] + s.dn[284][16]);
        let eq36_e529_d_n17: f64 = (s.dn[282][17] + s.dn[284][17]);
        let eq36_e529_d_n18: f64 = (s.dn[282][18] + s.dn[284][18]);
        let eq36_e529_d_b0: f64 = (s.db[282][0] + s.db[284][0]);
        let eq36_e529_d_b1: f64 = (s.db[282][1] + s.db[284][1]);
        let eq36_e529_d_b2: f64 = (s.db[282][2] + s.db[284][2]);
        let eq36_e529_d_b3: f64 = (s.db[282][3] + s.db[284][3]);
        let eq36_e529_d_b4: f64 = (s.db[282][4] + s.db[284][4]);
        let eq36_e529_d_b5: f64 = (s.db[282][5] + s.db[284][5]);
        let eq36_e529_d_b6: f64 = (s.db[282][6] + s.db[284][6]);
        let eq36_e529_d_b7: f64 = (s.db[282][7] + s.db[284][7]);
        let eq36_e529_d_b8: f64 = (s.db[282][8] + s.db[284][8]);
        let eq36_e529_d_b9: f64 = (s.db[282][9] + s.db[284][9]);
        let eq36_e529_d_b10: f64 = (s.db[282][10] + s.db[284][10]);
        let eq36_e529_d_b11: f64 = (s.db[282][11] + s.db[284][11]);
        let eq36_e529_d_b12: f64 = (s.db[282][12] + s.db[284][12]);
        let eq36_e529_q: f64 = eq36_e528_q;
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n1: f64 = (p.p50 * eq36_e529_d_n1);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n3: f64 = (p.p50 * eq36_e529_d_n3);
        let eq36_e530_d_n4: f64 = (p.p50 * eq36_e529_d_n4);
        let eq36_e530_d_n5: f64 = (p.p50 * eq36_e529_d_n5);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n8: f64 = (p.p50 * eq36_e529_d_n8);
        let eq36_e530_d_n9: f64 = (p.p50 * eq36_e529_d_n9);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n13: f64 = (p.p50 * eq36_e529_d_n13);
        let eq36_e530_d_n14: f64 = (p.p50 * eq36_e529_d_n14);
        let eq36_e530_d_n15: f64 = (p.p50 * eq36_e529_d_n15);
        let eq36_e530_d_n16: f64 = (p.p50 * eq36_e529_d_n16);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        let eq36_e530_d_n18: f64 = (p.p50 * eq36_e529_d_n18);
        let eq36_e530_d_b0: f64 = (p.p50 * eq36_e529_d_b0);
        let eq36_e530_d_b1: f64 = (p.p50 * eq36_e529_d_b1);
        let eq36_e530_d_b2: f64 = (p.p50 * eq36_e529_d_b2);
        let eq36_e530_d_b3: f64 = (p.p50 * eq36_e529_d_b3);
        let eq36_e530_d_b4: f64 = (p.p50 * eq36_e529_d_b4);
        let eq36_e530_d_b5: f64 = (p.p50 * eq36_e529_d_b5);
        let eq36_e530_d_b6: f64 = (p.p50 * eq36_e529_d_b6);
        let eq36_e530_d_b7: f64 = (p.p50 * eq36_e529_d_b7);
        let eq36_e530_d_b8: f64 = (p.p50 * eq36_e529_d_b8);
        let eq36_e530_d_b9: f64 = (p.p50 * eq36_e529_d_b9);
        let eq36_e530_d_b10: f64 = (p.p50 * eq36_e529_d_b10);
        let eq36_e530_d_b11: f64 = (p.p50 * eq36_e529_d_b11);
        let eq36_e530_d_b12: f64 = (p.p50 * eq36_e529_d_b12);
        let eq36_e530_q: f64 = (p.p50 * eq36_e529_q);
        let eq36_e530_q_d_n0: f64 = (p.p50 * s.dn[284][0]);
        let eq36_e530_q_d_n1: f64 = (p.p50 * s.dn[284][1]);
        let eq36_e530_q_d_n2: f64 = (p.p50 * s.dn[284][2]);
        let eq36_e530_q_d_n3: f64 = (p.p50 * s.dn[284][3]);
        let eq36_e530_q_d_n4: f64 = (p.p50 * s.dn[284][4]);
        let eq36_e530_q_d_n5: f64 = (p.p50 * s.dn[284][5]);
        let eq36_e530_q_d_n6: f64 = (p.p50 * s.dn[284][6]);
        let eq36_e530_q_d_n7: f64 = (p.p50 * s.dn[284][7]);
        let eq36_e530_q_d_n8: f64 = (p.p50 * s.dn[284][8]);
        let eq36_e530_q_d_n9: f64 = (p.p50 * s.dn[284][9]);
        let eq36_e530_q_d_n10: f64 = (p.p50 * s.dn[284][10]);
        let eq36_e530_q_d_n11: f64 = (p.p50 * s.dn[284][11]);
        let eq36_e530_q_d_n12: f64 = (p.p50 * s.dn[284][12]);
        let eq36_e530_q_d_n13: f64 = (p.p50 * s.dn[284][13]);
        let eq36_e530_q_d_n14: f64 = (p.p50 * s.dn[284][14]);
        let eq36_e530_q_d_n15: f64 = (p.p50 * s.dn[284][15]);
        let eq36_e530_q_d_n16: f64 = (p.p50 * s.dn[284][16]);
        let eq36_e530_q_d_n17: f64 = (p.p50 * s.dn[284][17]);
        let eq36_e530_q_d_n18: f64 = (p.p50 * s.dn[284][18]);
        let eq36_e530_q_d_b0: f64 = (p.p50 * s.db[284][0]);
        let eq36_e530_q_d_b1: f64 = (p.p50 * s.db[284][1]);
        let eq36_e530_q_d_b2: f64 = (p.p50 * s.db[284][2]);
        let eq36_e530_q_d_b3: f64 = (p.p50 * s.db[284][3]);
        let eq36_e530_q_d_b4: f64 = (p.p50 * s.db[284][4]);
        let eq36_e530_q_d_b5: f64 = (p.p50 * s.db[284][5]);
        let eq36_e530_q_d_b6: f64 = (p.p50 * s.db[284][6]);
        let eq36_e530_q_d_b7: f64 = (p.p50 * s.db[284][7]);
        let eq36_e530_q_d_b8: f64 = (p.p50 * s.db[284][8]);
        let eq36_e530_q_d_b9: f64 = (p.p50 * s.db[284][9]);
        let eq36_e530_q_d_b10: f64 = (p.p50 * s.db[284][10]);
        let eq36_e530_q_d_b11: f64 = (p.p50 * s.db[284][11]);
        let eq36_e530_q_d_b12: f64 = (p.p50 * s.db[284][12]);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n1, eq36_e530_d_n2, eq36_e530_d_n3, eq36_e530_d_n4, eq36_e530_d_n5, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n8, eq36_e530_d_n9, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n13, eq36_e530_d_n14, eq36_e530_d_n15, eq36_e530_d_n16, eq36_e530_d_n17, eq36_e530_d_n18, eq36_e530_d_b0, eq36_e530_d_b1, eq36_e530_d_b2, eq36_e530_d_b3, eq36_e530_d_b4, eq36_e530_d_b5, eq36_e530_d_b6, eq36_e530_d_b7, eq36_e530_d_b8, eq36_e530_d_b9, eq36_e530_d_b10, eq36_e530_d_b11, eq36_e530_d_b12, eq36_e530_q, eq36_e530_q_d_n0, eq36_e530_q_d_n1, eq36_e530_q_d_n2, eq36_e530_q_d_n3, eq36_e530_q_d_n4, eq36_e530_q_d_n5, eq36_e530_q_d_n6, eq36_e530_q_d_n7, eq36_e530_q_d_n8, eq36_e530_q_d_n9, eq36_e530_q_d_n10, eq36_e530_q_d_n11, eq36_e530_q_d_n12, eq36_e530_q_d_n13, eq36_e530_q_d_n14, eq36_e530_q_d_n15, eq36_e530_q_d_n16, eq36_e530_q_d_n17, eq36_e530_q_d_n18, eq36_e530_q_d_b0, eq36_e530_q_d_b1, eq36_e530_q_d_b2, eq36_e530_q_d_b3, eq36_e530_q_d_b4, eq36_e530_q_d_b5, eq36_e530_q_d_b6, eq36_e530_q_d_b7, eq36_e530_q_d_b8, eq36_e530_q_d_b9, eq36_e530_q_d_b10, eq36_e530_q_d_b11, eq36_e530_q_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 19] = [eq36_e532_q_d_n0, eq36_e532_q_d_n1, eq36_e532_q_d_n2, eq36_e532_q_d_n3, eq36_e532_q_d_n4, eq36_e532_q_d_n5, eq36_e532_q_d_n6, eq36_e532_q_d_n7, eq36_e532_q_d_n8, eq36_e532_q_d_n9, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, eq36_e532_q_d_n13, eq36_e532_q_d_n14, eq36_e532_q_d_n15, eq36_e532_q_d_n16, eq36_e532_q_d_n17, eq36_e532_q_d_n18];
        let eq36_reactive_branch_derivatives: [f64; 13] = [eq36_e532_q_d_b0, eq36_e532_q_d_b1, eq36_e532_q_d_b2, eq36_e532_q_d_b3, eq36_e532_q_d_b4, eq36_e532_q_d_b5, eq36_e532_q_d_b6, eq36_e532_q_d_b7, eq36_e532_q_d_b8, eq36_e532_q_d_b9, eq36_e532_q_d_b10, eq36_e532_q_d_b11, eq36_e532_q_d_b12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &nodes,
            &eq36_reactive_node_derivatives,
            &branches,
            &eq36_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_47_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq47_e616, eq47_e616_d_n18, eq47_e616_q, eq47_e616_q_d_n18,) = {
    if ((s.v[1851] != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614_q: f64 = eq47_e613;
        (eq47_e613, eq47_e611, eq47_e614_q, eq47_e611,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[18]),
            None,
            &[
                GeneratedDerivative::node(nodes[18], self.multiplicity * (eq47_e616_q_d_n18)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_48_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq48_e627, eq48_e627_d_n13, eq48_e627_q, eq48_e627_q_d_n13,) = {
    if ((s.v[1851] != 0.0) && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625_q: f64 = eq48_e624;
        (eq48_e624, eq48_e622, eq48_e625_q, eq48_e622,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[13]),
            None,
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * (eq48_e627_q_d_n13)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_53_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq53_e666, eq53_e666_d_n17, eq53_e666_q, eq53_e666_q_d_n17,) = {
    if ((s.v[1851] != 0.0) && (s.v[1852] != 0.0)) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664_q: f64 = eq53_e663;
        (eq53_e663, eq53_e661, eq53_e664_q, eq53_e661,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[17]),
            None,
            &[
                GeneratedDerivative::node(nodes[17], self.multiplicity * (eq53_e666_q_d_n17)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_60_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq60_e724, eq60_e724_d_n17, eq60_e724_q, eq60_e724_q_d_n17,) = {
    if ((!(s.v[1851] != 0.0)) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722_q: f64 = eq60_e721;
        (eq60_e721, eq60_e719, eq60_e722_q, eq60_e719,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[17]),
            None,
            &[
                GeneratedDerivative::node(nodes[17], self.multiplicity * (eq60_e724_q_d_n17)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_68_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq68_e792, eq68_e792_d_n15, eq68_e792_q, eq68_e792_q_d_n15,) = {
    if ((!(s.v[1851] != 0.0)) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790_q: f64 = eq68_e789;
        (eq68_e789, eq68_e787, eq68_e790_q, eq68_e787,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[15]),
            None,
            &[
                GeneratedDerivative::node(nodes[15], self.multiplicity * (eq68_e792_q_d_n15)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_69_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq69_e804, eq69_e804_d_n16, eq69_e804_q, eq69_e804_q_d_n16,) = {
    if ((!(s.v[1851] != 0.0)) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802_q: f64 = eq69_e801;
        (eq69_e801, eq69_e799, eq69_e802_q, eq69_e799,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[16]),
            None,
            &[
                GeneratedDerivative::node(nodes[16], self.multiplicity * (eq69_e804_q_d_n16)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_70_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq70_e816, eq70_e816_d_n13, eq70_e816_q, eq70_e816_q_d_n13,) = {
    if ((!(s.v[1851] != 0.0)) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814_q: f64 = eq70_e813;
        (eq70_e813, eq70_e811, eq70_e814_q, eq70_e811,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[13]),
            None,
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * (eq70_e816_q_d_n13)),
            ],
        );
    }
}
