#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq12_e399: f64 = (s.v[90] + s.v[548]);
        let eq12_e399_d_n0: f64 = (s.dn[90][0] + s.dn[548][0]);
        let eq12_e399_d_n1: f64 = (s.dn[90][1] + s.dn[548][1]);
        let eq12_e399_d_n2: f64 = (s.dn[90][2] + s.dn[548][2]);
        let eq12_e399_d_n3: f64 = (s.dn[90][3] + s.dn[548][3]);
        let eq12_e399_d_n4: f64 = (s.dn[90][4] + s.dn[548][4]);
        let eq12_e399_d_n5: f64 = (s.dn[90][5] + s.dn[548][5]);
        let eq12_e399_d_n6: f64 = (s.dn[90][6] + s.dn[548][6]);
        let eq12_e399_d_n7: f64 = (s.dn[90][7] + s.dn[548][7]);
        let eq12_e399_d_n8: f64 = (s.dn[90][8] + s.dn[548][8]);
        let eq12_e399_d_n9: f64 = (s.dn[90][9] + s.dn[548][9]);
        let eq12_e399_d_n10: f64 = (s.dn[90][10] + s.dn[548][10]);
        let eq12_e399_d_n11: f64 = (s.dn[90][11] + s.dn[548][11]);
        let eq12_e399_d_n12: f64 = (s.dn[90][12] + s.dn[548][12]);
        let eq12_e399_d_b0: f64 = (s.db[90][0] + s.db[548][0]);
        let eq12_e399_d_b1: f64 = (s.db[90][1] + s.db[548][1]);
        let eq12_e399_d_b2: f64 = (s.db[90][2] + s.db[548][2]);
        let eq12_e399_d_b3: f64 = (s.db[90][3] + s.db[548][3]);
        let eq12_e400_q: f64 = eq12_e399;
        let eq12_e401: f64 = (p.p33 * eq12_e399);
        let eq12_e401_d_n0: f64 = (p.p33 * eq12_e399_d_n0);
        let eq12_e401_d_n1: f64 = (p.p33 * eq12_e399_d_n1);
        let eq12_e401_d_n2: f64 = (p.p33 * eq12_e399_d_n2);
        let eq12_e401_d_n3: f64 = (p.p33 * eq12_e399_d_n3);
        let eq12_e401_d_n4: f64 = (p.p33 * eq12_e399_d_n4);
        let eq12_e401_d_n5: f64 = (p.p33 * eq12_e399_d_n5);
        let eq12_e401_d_n6: f64 = (p.p33 * eq12_e399_d_n6);
        let eq12_e401_d_n7: f64 = (p.p33 * eq12_e399_d_n7);
        let eq12_e401_d_n8: f64 = (p.p33 * eq12_e399_d_n8);
        let eq12_e401_d_n9: f64 = (p.p33 * eq12_e399_d_n9);
        let eq12_e401_d_n10: f64 = (p.p33 * eq12_e399_d_n10);
        let eq12_e401_d_n11: f64 = (p.p33 * eq12_e399_d_n11);
        let eq12_e401_d_n12: f64 = (p.p33 * eq12_e399_d_n12);
        let eq12_e401_d_b0: f64 = (p.p33 * eq12_e399_d_b0);
        let eq12_e401_d_b1: f64 = (p.p33 * eq12_e399_d_b1);
        let eq12_e401_d_b2: f64 = (p.p33 * eq12_e399_d_b2);
        let eq12_e401_d_b3: f64 = (p.p33 * eq12_e399_d_b3);
        let eq12_e401_q: f64 = (p.p33 * eq12_e400_q);
        let eq12_e401_q_d_n0: f64 = (p.p33 * eq12_e399_d_n0);
        let eq12_e401_q_d_n1: f64 = (p.p33 * eq12_e399_d_n1);
        let eq12_e401_q_d_n2: f64 = (p.p33 * eq12_e399_d_n2);
        let eq12_e401_q_d_n3: f64 = (p.p33 * eq12_e399_d_n3);
        let eq12_e401_q_d_n4: f64 = (p.p33 * eq12_e399_d_n4);
        let eq12_e401_q_d_n5: f64 = (p.p33 * eq12_e399_d_n5);
        let eq12_e401_q_d_n6: f64 = (p.p33 * eq12_e399_d_n6);
        let eq12_e401_q_d_n7: f64 = (p.p33 * eq12_e399_d_n7);
        let eq12_e401_q_d_n8: f64 = (p.p33 * eq12_e399_d_n8);
        let eq12_e401_q_d_n9: f64 = (p.p33 * eq12_e399_d_n9);
        let eq12_e401_q_d_n10: f64 = (p.p33 * eq12_e399_d_n10);
        let eq12_e401_q_d_n11: f64 = (p.p33 * eq12_e399_d_n11);
        let eq12_e401_q_d_n12: f64 = (p.p33 * eq12_e399_d_n12);
        let eq12_e401_q_d_b0: f64 = (p.p33 * eq12_e399_d_b0);
        let eq12_e401_q_d_b1: f64 = (p.p33 * eq12_e399_d_b1);
        let eq12_e401_q_d_b2: f64 = (p.p33 * eq12_e399_d_b2);
        let eq12_e401_q_d_b3: f64 = (p.p33 * eq12_e399_d_b3);
        let eq12_reactive_node_derivatives: [f64; 13] = [eq12_e401_q_d_n0, eq12_e401_q_d_n1, eq12_e401_q_d_n2, eq12_e401_q_d_n3, eq12_e401_q_d_n4, eq12_e401_q_d_n5, eq12_e401_q_d_n6, eq12_e401_q_d_n7, eq12_e401_q_d_n8, eq12_e401_q_d_n9, eq12_e401_q_d_n10, eq12_e401_q_d_n11, eq12_e401_q_d_n12];
        let eq12_reactive_branch_derivatives: [f64; 4] = [eq12_e401_q_d_b0, eq12_e401_q_d_b1, eq12_e401_q_d_b2, eq12_e401_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[12]),
            &nodes,
            &eq12_reactive_node_derivatives,
            &branches,
            &eq12_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq18_e430: f64 = ((nv7 - 0.0) * s.v[611]);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * s.dn[611][0]);
        let eq18_e430_d_n1: f64 = ((nv7 - 0.0) * s.dn[611][1]);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * s.dn[611][2]);
        let eq18_e430_d_n3: f64 = ((nv7 - 0.0) * s.dn[611][3]);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * s.dn[611][4]);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * s.dn[611][5]);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * s.dn[611][6]);
        let eq18_e430_d_n7: f64 = (s.v[611] + ((nv7 - 0.0) * s.dn[611][7]));
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * s.dn[611][8]);
        let eq18_e430_d_n9: f64 = ((nv7 - 0.0) * s.dn[611][9]);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * s.dn[611][10]);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * s.dn[611][11]);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * s.dn[611][12]);
        let eq18_e430_d_b0: f64 = ((nv7 - 0.0) * s.db[611][0]);
        let eq18_e430_d_b1: f64 = ((nv7 - 0.0) * s.db[611][1]);
        let eq18_e430_d_b2: f64 = ((nv7 - 0.0) * s.db[611][2]);
        let eq18_e430_d_b3: f64 = ((nv7 - 0.0) * s.db[611][3]);
        let eq18_e431_q: f64 = eq18_e430;
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e430_d_n0, eq18_e430_d_n1, eq18_e430_d_n2, eq18_e430_d_n3, eq18_e430_d_n4, eq18_e430_d_n5, eq18_e430_d_n6, eq18_e430_d_n7, eq18_e430_d_n8, eq18_e430_d_n9, eq18_e430_d_n10, eq18_e430_d_n11, eq18_e430_d_n12];
        let eq18_reactive_branch_derivatives: [f64; 4] = [eq18_e430_d_b0, eq18_e430_d_b1, eq18_e430_d_b2, eq18_e430_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            &nodes,
            &eq18_reactive_node_derivatives,
            &branches,
            &eq18_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq19_e434: f64 = ((nv7 - 0.0) * s.v[612]);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * s.dn[612][0]);
        let eq19_e434_d_n1: f64 = ((nv7 - 0.0) * s.dn[612][1]);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * s.dn[612][2]);
        let eq19_e434_d_n3: f64 = ((nv7 - 0.0) * s.dn[612][3]);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * s.dn[612][4]);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * s.dn[612][5]);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * s.dn[612][6]);
        let eq19_e434_d_n7: f64 = (s.v[612] + ((nv7 - 0.0) * s.dn[612][7]));
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * s.dn[612][8]);
        let eq19_e434_d_n9: f64 = ((nv7 - 0.0) * s.dn[612][9]);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * s.dn[612][10]);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * s.dn[612][11]);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * s.dn[612][12]);
        let eq19_e434_d_b0: f64 = ((nv7 - 0.0) * s.db[612][0]);
        let eq19_e434_d_b1: f64 = ((nv7 - 0.0) * s.db[612][1]);
        let eq19_e434_d_b2: f64 = ((nv7 - 0.0) * s.db[612][2]);
        let eq19_e434_d_b3: f64 = ((nv7 - 0.0) * s.db[612][3]);
        let eq19_e435_q: f64 = eq19_e434;
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e434_d_n0, eq19_e434_d_n1, eq19_e434_d_n2, eq19_e434_d_n3, eq19_e434_d_n4, eq19_e434_d_n5, eq19_e434_d_n6, eq19_e434_d_n7, eq19_e434_d_n8, eq19_e434_d_n9, eq19_e434_d_n10, eq19_e434_d_n11, eq19_e434_d_n12];
        let eq19_reactive_branch_derivatives: [f64; 4] = [eq19_e434_d_b0, eq19_e434_d_b1, eq19_e434_d_b2, eq19_e434_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            &nodes,
            &eq19_reactive_node_derivatives,
            &branches,
            &eq19_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_28_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n1, eq28_e498_d_n2, eq28_e498_d_n3, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n7, eq28_e498_d_n8, eq28_e498_d_n9, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12, eq28_e498_d_b0, eq28_e498_d_b1, eq28_e498_d_b2, eq28_e498_d_b3, eq28_e498_q, eq28_e498_q_d_n0, eq28_e498_q_d_n1, eq28_e498_q_d_n2, eq28_e498_q_d_n3, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, eq28_e498_q_d_n7, eq28_e498_q_d_n8, eq28_e498_q_d_n9, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12, eq28_e498_q_d_b0, eq28_e498_q_d_b1, eq28_e498_q_d_b2, eq28_e498_q_d_b3,) = {
    if (s.v[1094] != 0.0) {
        let eq28_e487: f64 = (-s.v[547]);
        let eq28_e487_d_n0: f64 = (-s.dn[547][0]);
        let eq28_e487_d_n1: f64 = (-s.dn[547][1]);
        let eq28_e487_d_n2: f64 = (-s.dn[547][2]);
        let eq28_e487_d_n3: f64 = (-s.dn[547][3]);
        let eq28_e487_d_n4: f64 = (-s.dn[547][4]);
        let eq28_e487_d_n5: f64 = (-s.dn[547][5]);
        let eq28_e487_d_n6: f64 = (-s.dn[547][6]);
        let eq28_e487_d_n7: f64 = (-s.dn[547][7]);
        let eq28_e487_d_n8: f64 = (-s.dn[547][8]);
        let eq28_e487_d_n9: f64 = (-s.dn[547][9]);
        let eq28_e487_d_n10: f64 = (-s.dn[547][10]);
        let eq28_e487_d_n11: f64 = (-s.dn[547][11]);
        let eq28_e487_d_n12: f64 = (-s.dn[547][12]);
        let eq28_e487_d_b0: f64 = (-s.db[547][0]);
        let eq28_e487_d_b1: f64 = (-s.db[547][1]);
        let eq28_e487_d_b2: f64 = (-s.db[547][2]);
        let eq28_e487_d_b3: f64 = (-s.db[547][3]);
        let eq28_e490: f64 = (s.v[516] * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (s.dn[516][0] * (nv4 - 0.0));
        let eq28_e490_d_n1: f64 = (s.dn[516][1] * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (s.dn[516][2] * (nv4 - 0.0));
        let eq28_e490_d_n3: f64 = (s.dn[516][3] * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((s.dn[516][4] * (nv4 - 0.0)) + s.v[516]);
        let eq28_e490_d_n5: f64 = (s.dn[516][5] * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (s.dn[516][6] * (nv4 - 0.0));
        let eq28_e490_d_n7: f64 = (s.dn[516][7] * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (s.dn[516][8] * (nv4 - 0.0));
        let eq28_e490_d_n9: f64 = (s.dn[516][9] * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (s.dn[516][10] * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (s.dn[516][11] * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (s.dn[516][12] * (nv4 - 0.0));
        let eq28_e490_d_b0: f64 = (s.db[516][0] * (nv4 - 0.0));
        let eq28_e490_d_b1: f64 = (s.db[516][1] * (nv4 - 0.0));
        let eq28_e490_d_b2: f64 = (s.db[516][2] * (nv4 - 0.0));
        let eq28_e490_d_b3: f64 = (s.db[516][3] * (nv4 - 0.0));
        let eq28_e491_q: f64 = eq28_e490;
        let eq28_e492: f64 = (eq28_e487 + eq28_e490);
        let eq28_e492_d_n0: f64 = (eq28_e487_d_n0 + eq28_e490_d_n0);
        let eq28_e492_d_n1: f64 = (eq28_e487_d_n1 + eq28_e490_d_n1);
        let eq28_e492_d_n2: f64 = (eq28_e487_d_n2 + eq28_e490_d_n2);
        let eq28_e492_d_n3: f64 = (eq28_e487_d_n3 + eq28_e490_d_n3);
        let eq28_e492_d_n4: f64 = (eq28_e487_d_n4 + eq28_e490_d_n4);
        let eq28_e492_d_n5: f64 = (eq28_e487_d_n5 + eq28_e490_d_n5);
        let eq28_e492_d_n6: f64 = (eq28_e487_d_n6 + eq28_e490_d_n6);
        let eq28_e492_d_n7: f64 = (eq28_e487_d_n7 + eq28_e490_d_n7);
        let eq28_e492_d_n8: f64 = (eq28_e487_d_n8 + eq28_e490_d_n8);
        let eq28_e492_d_n9: f64 = (eq28_e487_d_n9 + eq28_e490_d_n9);
        let eq28_e492_d_n10: f64 = (eq28_e487_d_n10 + eq28_e490_d_n10);
        let eq28_e492_d_n11: f64 = (eq28_e487_d_n11 + eq28_e490_d_n11);
        let eq28_e492_d_n12: f64 = (eq28_e487_d_n12 + eq28_e490_d_n12);
        let eq28_e492_d_b0: f64 = (eq28_e487_d_b0 + eq28_e490_d_b0);
        let eq28_e492_d_b1: f64 = (eq28_e487_d_b1 + eq28_e490_d_b1);
        let eq28_e492_d_b2: f64 = (eq28_e487_d_b2 + eq28_e490_d_b2);
        let eq28_e492_d_b3: f64 = (eq28_e487_d_b3 + eq28_e490_d_b3);
        let eq28_e492_q: f64 = eq28_e491_q;
        let eq28_e495: f64 = ((nv4 - 0.0) * s.v[557]);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * s.dn[557][0]);
        let eq28_e495_d_n1: f64 = ((nv4 - 0.0) * s.dn[557][1]);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * s.dn[557][2]);
        let eq28_e495_d_n3: f64 = ((nv4 - 0.0) * s.dn[557][3]);
        let eq28_e495_d_n4: f64 = (s.v[557] + ((nv4 - 0.0) * s.dn[557][4]));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * s.dn[557][5]);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * s.dn[557][6]);
        let eq28_e495_d_n7: f64 = ((nv4 - 0.0) * s.dn[557][7]);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * s.dn[557][8]);
        let eq28_e495_d_n9: f64 = ((nv4 - 0.0) * s.dn[557][9]);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * s.dn[557][10]);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * s.dn[557][11]);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * s.dn[557][12]);
        let eq28_e495_d_b0: f64 = ((nv4 - 0.0) * s.db[557][0]);
        let eq28_e495_d_b1: f64 = ((nv4 - 0.0) * s.db[557][1]);
        let eq28_e495_d_b2: f64 = ((nv4 - 0.0) * s.db[557][2]);
        let eq28_e495_d_b3: f64 = ((nv4 - 0.0) * s.db[557][3]);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n1: f64 = (eq28_e492_d_n1 + eq28_e495_d_n1);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n3: f64 = (eq28_e492_d_n3 + eq28_e495_d_n3);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n7: f64 = (eq28_e492_d_n7 + eq28_e495_d_n7);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n9: f64 = (eq28_e492_d_n9 + eq28_e495_d_n9);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        let eq28_e496_d_b0: f64 = (eq28_e492_d_b0 + eq28_e495_d_b0);
        let eq28_e496_d_b1: f64 = (eq28_e492_d_b1 + eq28_e495_d_b1);
        let eq28_e496_d_b2: f64 = (eq28_e492_d_b2 + eq28_e495_d_b2);
        let eq28_e496_d_b3: f64 = (eq28_e492_d_b3 + eq28_e495_d_b3);
        let eq28_e496_q: f64 = eq28_e492_q;
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n1, eq28_e496_d_n2, eq28_e496_d_n3, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n7, eq28_e496_d_n8, eq28_e496_d_n9, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12, eq28_e496_d_b0, eq28_e496_d_b1, eq28_e496_d_b2, eq28_e496_d_b3, eq28_e496_q, eq28_e490_d_n0, eq28_e490_d_n1, eq28_e490_d_n2, eq28_e490_d_n3, eq28_e490_d_n4, eq28_e490_d_n5, eq28_e490_d_n6, eq28_e490_d_n7, eq28_e490_d_n8, eq28_e490_d_n9, eq28_e490_d_n10, eq28_e490_d_n11, eq28_e490_d_n12, eq28_e490_d_b0, eq28_e490_d_b1, eq28_e490_d_b2, eq28_e490_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e498_q_d_n0, eq28_e498_q_d_n1, eq28_e498_q_d_n2, eq28_e498_q_d_n3, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, eq28_e498_q_d_n7, eq28_e498_q_d_n8, eq28_e498_q_d_n9, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12];
        let eq28_reactive_branch_derivatives: [f64; 4] = [eq28_e498_q_d_b0, eq28_e498_q_d_b1, eq28_e498_q_d_b2, eq28_e498_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            &nodes,
            &eq28_reactive_node_derivatives,
            &branches,
            &eq28_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_30_block_0(
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
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n1, eq30_e512_d_n2, eq30_e512_d_n3, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n7, eq30_e512_d_n8, eq30_e512_d_n9, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12, eq30_e512_d_b0, eq30_e512_d_b1, eq30_e512_d_b2, eq30_e512_d_b3, eq30_e512_q, eq30_e512_q_d_n10,) = {
    if (s.v[1095] != 0.0) {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e508_d_n10: f64 = 1e-9;
        let eq30_e509_q: f64 = eq30_e508;
        let eq30_e510: f64 = (s.v[558] + eq30_e508);
        let eq30_e510_d_n10: f64 = (s.dn[558][10] + eq30_e508_d_n10);
        let eq30_e510_q: f64 = eq30_e509_q;
        (eq30_e510, s.dn[558][0], s.dn[558][1], s.dn[558][2], s.dn[558][3], s.dn[558][4], s.dn[558][5], s.dn[558][6], s.dn[558][7], s.dn[558][8], s.dn[558][9], eq30_e510_d_n10, s.dn[558][11], s.dn[558][12], s.db[558][0], s.db[558][1], s.db[558][2], s.db[558][3], eq30_e510_q, eq30_e508_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[10]),
            None,
            &[
                GeneratedDerivative::node(nodes[10], self.multiplicity * (eq30_e512_q_d_n10)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_32_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n1, eq32_e526_d_n2, eq32_e526_d_n3, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n7, eq32_e526_d_n8, eq32_e526_d_n9, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12, eq32_e526_d_b0, eq32_e526_d_b1, eq32_e526_d_b2, eq32_e526_d_b3, eq32_e526_q, eq32_e526_q_d_n8,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e522_d_n8: f64 = 1e-9;
        let eq32_e523_q: f64 = eq32_e522;
        let eq32_e524: f64 = (s.v[549] + eq32_e522);
        let eq32_e524_d_n8: f64 = (s.dn[549][8] + eq32_e522_d_n8);
        let eq32_e524_q: f64 = eq32_e523_q;
        (eq32_e524, s.dn[549][0], s.dn[549][1], s.dn[549][2], s.dn[549][3], s.dn[549][4], s.dn[549][5], s.dn[549][6], s.dn[549][7], eq32_e524_d_n8, s.dn[549][9], s.dn[549][10], s.dn[549][11], s.dn[549][12], s.db[549][0], s.db[549][1], s.db[549][2], s.db[549][3], eq32_e524_q, eq32_e522_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[8]),
            None,
            &[
                GeneratedDerivative::node(nodes[8], self.multiplicity * (eq32_e526_q_d_n8)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n1, eq33_e535_d_n2, eq33_e535_d_n3, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n7, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12, eq33_e535_d_b0, eq33_e535_d_b1, eq33_e535_d_b2, eq33_e535_d_b3, eq33_e535_q, eq33_e535_q_d_n9,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e531_d_n9: f64 = 1e-9;
        let eq33_e532_q: f64 = eq33_e531;
        let eq33_e533: f64 = (s.v[550] + eq33_e531);
        let eq33_e533_d_n9: f64 = (s.dn[550][9] + eq33_e531_d_n9);
        let eq33_e533_q: f64 = eq33_e532_q;
        (eq33_e533, s.dn[550][0], s.dn[550][1], s.dn[550][2], s.dn[550][3], s.dn[550][4], s.dn[550][5], s.dn[550][6], s.dn[550][7], s.dn[550][8], eq33_e533_d_n9, s.dn[550][10], s.dn[550][11], s.dn[550][12], s.db[550][0], s.db[550][1], s.db[550][2], s.db[550][3], eq33_e533_q, eq33_e531_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[9]),
            None,
            &[
                GeneratedDerivative::node(nodes[9], self.multiplicity * (eq33_e535_q_d_n9)),
            ],
        );
    }
}
