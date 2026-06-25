#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq22_e315: f64 = (p.p3 * p.p69);
        let eq22_e317: f64 = (eq22_e315 * s.v[269]);
        let eq22_e317_d_n0: f64 = (eq22_e315 * s.dn[269][0]);
        let eq22_e317_d_n1: f64 = (eq22_e315 * s.dn[269][1]);
        let eq22_e317_d_n2: f64 = (eq22_e315 * s.dn[269][2]);
        let eq22_e317_d_n3: f64 = (eq22_e315 * s.dn[269][3]);
        let eq22_e317_d_n4: f64 = (eq22_e315 * s.dn[269][4]);
        let eq22_e317_d_n5: f64 = (eq22_e315 * s.dn[269][5]);
        let eq22_e317_d_n6: f64 = (eq22_e315 * s.dn[269][6]);
        let eq22_e317_d_n7: f64 = (eq22_e315 * s.dn[269][7]);
        let eq22_e317_d_n8: f64 = (eq22_e315 * s.dn[269][8]);
        let eq22_e317_d_n9: f64 = (eq22_e315 * s.dn[269][9]);
        let eq22_e317_d_n10: f64 = (eq22_e315 * s.dn[269][10]);
        let eq22_e317_d_n11: f64 = (eq22_e315 * s.dn[269][11]);
        let eq22_e317_d_n12: f64 = (eq22_e315 * s.dn[269][12]);
        let eq22_e318_q: f64 = eq22_e317;
        let eq22_e320: f64 = (eq22_e317 * p.p1);
        let eq22_e320_d_n0: f64 = (eq22_e317_d_n0 * p.p1);
        let eq22_e320_d_n1: f64 = (eq22_e317_d_n1 * p.p1);
        let eq22_e320_d_n2: f64 = (eq22_e317_d_n2 * p.p1);
        let eq22_e320_d_n3: f64 = (eq22_e317_d_n3 * p.p1);
        let eq22_e320_d_n4: f64 = (eq22_e317_d_n4 * p.p1);
        let eq22_e320_d_n5: f64 = (eq22_e317_d_n5 * p.p1);
        let eq22_e320_d_n6: f64 = (eq22_e317_d_n6 * p.p1);
        let eq22_e320_d_n7: f64 = (eq22_e317_d_n7 * p.p1);
        let eq22_e320_d_n8: f64 = (eq22_e317_d_n8 * p.p1);
        let eq22_e320_d_n9: f64 = (eq22_e317_d_n9 * p.p1);
        let eq22_e320_d_n10: f64 = (eq22_e317_d_n10 * p.p1);
        let eq22_e320_d_n11: f64 = (eq22_e317_d_n11 * p.p1);
        let eq22_e320_d_n12: f64 = (eq22_e317_d_n12 * p.p1);
        let eq22_e320_q: f64 = (eq22_e318_q * p.p1);
        let eq22_e320_q_d_n0: f64 = (eq22_e317_d_n0 * p.p1);
        let eq22_e320_q_d_n1: f64 = (eq22_e317_d_n1 * p.p1);
        let eq22_e320_q_d_n2: f64 = (eq22_e317_d_n2 * p.p1);
        let eq22_e320_q_d_n3: f64 = (eq22_e317_d_n3 * p.p1);
        let eq22_e320_q_d_n4: f64 = (eq22_e317_d_n4 * p.p1);
        let eq22_e320_q_d_n5: f64 = (eq22_e317_d_n5 * p.p1);
        let eq22_e320_q_d_n6: f64 = (eq22_e317_d_n6 * p.p1);
        let eq22_e320_q_d_n7: f64 = (eq22_e317_d_n7 * p.p1);
        let eq22_e320_q_d_n8: f64 = (eq22_e317_d_n8 * p.p1);
        let eq22_e320_q_d_n9: f64 = (eq22_e317_d_n9 * p.p1);
        let eq22_e320_q_d_n10: f64 = (eq22_e317_d_n10 * p.p1);
        let eq22_e320_q_d_n11: f64 = (eq22_e317_d_n11 * p.p1);
        let eq22_e320_q_d_n12: f64 = (eq22_e317_d_n12 * p.p1);
        let eq22_reactive_node_derivatives: [f64; 13] = [eq22_e320_q_d_n0, eq22_e320_q_d_n1, eq22_e320_q_d_n2, eq22_e320_q_d_n3, eq22_e320_q_d_n4, eq22_e320_q_d_n5, eq22_e320_q_d_n6, eq22_e320_q_d_n7, eq22_e320_q_d_n8, eq22_e320_q_d_n9, eq22_e320_q_d_n10, eq22_e320_q_d_n11, eq22_e320_q_d_n12];
        let eq22_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            &nodes,
            &eq22_reactive_node_derivatives,
            &branches,
            &eq22_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq23_e323: f64 = (p.p3 * p.p78);
        let eq23_e325: f64 = (eq23_e323 * s.v[270]);
        let eq23_e325_d_n0: f64 = (eq23_e323 * s.dn[270][0]);
        let eq23_e325_d_n1: f64 = (eq23_e323 * s.dn[270][1]);
        let eq23_e325_d_n2: f64 = (eq23_e323 * s.dn[270][2]);
        let eq23_e325_d_n3: f64 = (eq23_e323 * s.dn[270][3]);
        let eq23_e325_d_n4: f64 = (eq23_e323 * s.dn[270][4]);
        let eq23_e325_d_n5: f64 = (eq23_e323 * s.dn[270][5]);
        let eq23_e325_d_n6: f64 = (eq23_e323 * s.dn[270][6]);
        let eq23_e325_d_n7: f64 = (eq23_e323 * s.dn[270][7]);
        let eq23_e325_d_n8: f64 = (eq23_e323 * s.dn[270][8]);
        let eq23_e325_d_n9: f64 = (eq23_e323 * s.dn[270][9]);
        let eq23_e325_d_n10: f64 = (eq23_e323 * s.dn[270][10]);
        let eq23_e325_d_n11: f64 = (eq23_e323 * s.dn[270][11]);
        let eq23_e325_d_n12: f64 = (eq23_e323 * s.dn[270][12]);
        let eq23_e326_q: f64 = eq23_e325;
        let eq23_e328: f64 = (eq23_e325 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e325_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e325_d_n1 * p.p1);
        let eq23_e328_d_n2: f64 = (eq23_e325_d_n2 * p.p1);
        let eq23_e328_d_n3: f64 = (eq23_e325_d_n3 * p.p1);
        let eq23_e328_d_n4: f64 = (eq23_e325_d_n4 * p.p1);
        let eq23_e328_d_n5: f64 = (eq23_e325_d_n5 * p.p1);
        let eq23_e328_d_n6: f64 = (eq23_e325_d_n6 * p.p1);
        let eq23_e328_d_n7: f64 = (eq23_e325_d_n7 * p.p1);
        let eq23_e328_d_n8: f64 = (eq23_e325_d_n8 * p.p1);
        let eq23_e328_d_n9: f64 = (eq23_e325_d_n9 * p.p1);
        let eq23_e328_d_n10: f64 = (eq23_e325_d_n10 * p.p1);
        let eq23_e328_d_n11: f64 = (eq23_e325_d_n11 * p.p1);
        let eq23_e328_d_n12: f64 = (eq23_e325_d_n12 * p.p1);
        let eq23_e328_q: f64 = (eq23_e326_q * p.p1);
        let eq23_e328_q_d_n0: f64 = (eq23_e325_d_n0 * p.p1);
        let eq23_e328_q_d_n1: f64 = (eq23_e325_d_n1 * p.p1);
        let eq23_e328_q_d_n2: f64 = (eq23_e325_d_n2 * p.p1);
        let eq23_e328_q_d_n3: f64 = (eq23_e325_d_n3 * p.p1);
        let eq23_e328_q_d_n4: f64 = (eq23_e325_d_n4 * p.p1);
        let eq23_e328_q_d_n5: f64 = (eq23_e325_d_n5 * p.p1);
        let eq23_e328_q_d_n6: f64 = (eq23_e325_d_n6 * p.p1);
        let eq23_e328_q_d_n7: f64 = (eq23_e325_d_n7 * p.p1);
        let eq23_e328_q_d_n8: f64 = (eq23_e325_d_n8 * p.p1);
        let eq23_e328_q_d_n9: f64 = (eq23_e325_d_n9 * p.p1);
        let eq23_e328_q_d_n10: f64 = (eq23_e325_d_n10 * p.p1);
        let eq23_e328_q_d_n11: f64 = (eq23_e325_d_n11 * p.p1);
        let eq23_e328_q_d_n12: f64 = (eq23_e325_d_n12 * p.p1);
        let eq23_reactive_node_derivatives: [f64; 13] = [eq23_e328_q_d_n0, eq23_e328_q_d_n1, eq23_e328_q_d_n2, eq23_e328_q_d_n3, eq23_e328_q_d_n4, eq23_e328_q_d_n5, eq23_e328_q_d_n6, eq23_e328_q_d_n7, eq23_e328_q_d_n8, eq23_e328_q_d_n9, eq23_e328_q_d_n10, eq23_e328_q_d_n11, eq23_e328_q_d_n12];
        let eq23_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            &nodes,
            &eq23_reactive_node_derivatives,
            &branches,
            &eq23_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq26_e344: f64 = (s.v[236] + s.v[248]);
        let eq26_e344_d_n0: f64 = (s.dn[236][0] + s.dn[248][0]);
        let eq26_e344_d_n1: f64 = (s.dn[236][1] + s.dn[248][1]);
        let eq26_e344_d_n2: f64 = (s.dn[236][2] + s.dn[248][2]);
        let eq26_e344_d_n3: f64 = (s.dn[236][3] + s.dn[248][3]);
        let eq26_e344_d_n4: f64 = (s.dn[236][4] + s.dn[248][4]);
        let eq26_e344_d_n5: f64 = (s.dn[236][5] + s.dn[248][5]);
        let eq26_e344_d_n6: f64 = (s.dn[236][6] + s.dn[248][6]);
        let eq26_e344_d_n7: f64 = (s.dn[236][7] + s.dn[248][7]);
        let eq26_e344_d_n8: f64 = (s.dn[236][8] + s.dn[248][8]);
        let eq26_e344_d_n9: f64 = (s.dn[236][9] + s.dn[248][9]);
        let eq26_e344_d_n10: f64 = (s.dn[236][10] + s.dn[248][10]);
        let eq26_e344_d_n11: f64 = (s.dn[236][11] + s.dn[248][11]);
        let eq26_e344_d_n12: f64 = (s.dn[236][12] + s.dn[248][12]);
        let eq26_e345: f64 = (p.p3 * eq26_e344);
        let eq26_e345_d_n0: f64 = (p.p3 * eq26_e344_d_n0);
        let eq26_e345_d_n1: f64 = (p.p3 * eq26_e344_d_n1);
        let eq26_e345_d_n2: f64 = (p.p3 * eq26_e344_d_n2);
        let eq26_e345_d_n3: f64 = (p.p3 * eq26_e344_d_n3);
        let eq26_e345_d_n4: f64 = (p.p3 * eq26_e344_d_n4);
        let eq26_e345_d_n5: f64 = (p.p3 * eq26_e344_d_n5);
        let eq26_e345_d_n6: f64 = (p.p3 * eq26_e344_d_n6);
        let eq26_e345_d_n7: f64 = (p.p3 * eq26_e344_d_n7);
        let eq26_e345_d_n8: f64 = (p.p3 * eq26_e344_d_n8);
        let eq26_e345_d_n9: f64 = (p.p3 * eq26_e344_d_n9);
        let eq26_e345_d_n10: f64 = (p.p3 * eq26_e344_d_n10);
        let eq26_e345_d_n11: f64 = (p.p3 * eq26_e344_d_n11);
        let eq26_e345_d_n12: f64 = (p.p3 * eq26_e344_d_n12);
        let eq26_e346_q: f64 = eq26_e345;
        let eq26_e348: f64 = (eq26_e345 * p.p1);
        let eq26_e348_d_n0: f64 = (eq26_e345_d_n0 * p.p1);
        let eq26_e348_d_n1: f64 = (eq26_e345_d_n1 * p.p1);
        let eq26_e348_d_n2: f64 = (eq26_e345_d_n2 * p.p1);
        let eq26_e348_d_n3: f64 = (eq26_e345_d_n3 * p.p1);
        let eq26_e348_d_n4: f64 = (eq26_e345_d_n4 * p.p1);
        let eq26_e348_d_n5: f64 = (eq26_e345_d_n5 * p.p1);
        let eq26_e348_d_n6: f64 = (eq26_e345_d_n6 * p.p1);
        let eq26_e348_d_n7: f64 = (eq26_e345_d_n7 * p.p1);
        let eq26_e348_d_n8: f64 = (eq26_e345_d_n8 * p.p1);
        let eq26_e348_d_n9: f64 = (eq26_e345_d_n9 * p.p1);
        let eq26_e348_d_n10: f64 = (eq26_e345_d_n10 * p.p1);
        let eq26_e348_d_n11: f64 = (eq26_e345_d_n11 * p.p1);
        let eq26_e348_d_n12: f64 = (eq26_e345_d_n12 * p.p1);
        let eq26_e348_q: f64 = (eq26_e346_q * p.p1);
        let eq26_e348_q_d_n0: f64 = (eq26_e345_d_n0 * p.p1);
        let eq26_e348_q_d_n1: f64 = (eq26_e345_d_n1 * p.p1);
        let eq26_e348_q_d_n2: f64 = (eq26_e345_d_n2 * p.p1);
        let eq26_e348_q_d_n3: f64 = (eq26_e345_d_n3 * p.p1);
        let eq26_e348_q_d_n4: f64 = (eq26_e345_d_n4 * p.p1);
        let eq26_e348_q_d_n5: f64 = (eq26_e345_d_n5 * p.p1);
        let eq26_e348_q_d_n6: f64 = (eq26_e345_d_n6 * p.p1);
        let eq26_e348_q_d_n7: f64 = (eq26_e345_d_n7 * p.p1);
        let eq26_e348_q_d_n8: f64 = (eq26_e345_d_n8 * p.p1);
        let eq26_e348_q_d_n9: f64 = (eq26_e345_d_n9 * p.p1);
        let eq26_e348_q_d_n10: f64 = (eq26_e345_d_n10 * p.p1);
        let eq26_e348_q_d_n11: f64 = (eq26_e345_d_n11 * p.p1);
        let eq26_e348_q_d_n12: f64 = (eq26_e345_d_n12 * p.p1);
        let eq26_reactive_node_derivatives: [f64; 13] = [eq26_e348_q_d_n0, eq26_e348_q_d_n1, eq26_e348_q_d_n2, eq26_e348_q_d_n3, eq26_e348_q_d_n4, eq26_e348_q_d_n5, eq26_e348_q_d_n6, eq26_e348_q_d_n7, eq26_e348_q_d_n8, eq26_e348_q_d_n9, eq26_e348_q_d_n10, eq26_e348_q_d_n11, eq26_e348_q_d_n12];
        let eq26_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            &nodes,
            &eq26_reactive_node_derivatives,
            &branches,
            &eq26_reactive_branch_derivatives,
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
        let eq28_e363: f64 = (s.v[233] + s.v[249]);
        let eq28_e363_d_n0: f64 = (s.dn[233][0] + s.dn[249][0]);
        let eq28_e363_d_n1: f64 = (s.dn[233][1] + s.dn[249][1]);
        let eq28_e363_d_n2: f64 = (s.dn[233][2] + s.dn[249][2]);
        let eq28_e363_d_n3: f64 = (s.dn[233][3] + s.dn[249][3]);
        let eq28_e363_d_n4: f64 = (s.dn[233][4] + s.dn[249][4]);
        let eq28_e363_d_n5: f64 = (s.dn[233][5] + s.dn[249][5]);
        let eq28_e363_d_n6: f64 = (s.dn[233][6] + s.dn[249][6]);
        let eq28_e363_d_n7: f64 = (s.dn[233][7] + s.dn[249][7]);
        let eq28_e363_d_n8: f64 = (s.dn[233][8] + s.dn[249][8]);
        let eq28_e363_d_n9: f64 = (s.dn[233][9] + s.dn[249][9]);
        let eq28_e363_d_n10: f64 = (s.dn[233][10] + s.dn[249][10]);
        let eq28_e363_d_n11: f64 = (s.dn[233][11] + s.dn[249][11]);
        let eq28_e363_d_n12: f64 = (s.dn[233][12] + s.dn[249][12]);
        let eq28_e364: f64 = (p.p3 * eq28_e363);
        let eq28_e364_d_n0: f64 = (p.p3 * eq28_e363_d_n0);
        let eq28_e364_d_n1: f64 = (p.p3 * eq28_e363_d_n1);
        let eq28_e364_d_n2: f64 = (p.p3 * eq28_e363_d_n2);
        let eq28_e364_d_n3: f64 = (p.p3 * eq28_e363_d_n3);
        let eq28_e364_d_n4: f64 = (p.p3 * eq28_e363_d_n4);
        let eq28_e364_d_n5: f64 = (p.p3 * eq28_e363_d_n5);
        let eq28_e364_d_n6: f64 = (p.p3 * eq28_e363_d_n6);
        let eq28_e364_d_n7: f64 = (p.p3 * eq28_e363_d_n7);
        let eq28_e364_d_n8: f64 = (p.p3 * eq28_e363_d_n8);
        let eq28_e364_d_n9: f64 = (p.p3 * eq28_e363_d_n9);
        let eq28_e364_d_n10: f64 = (p.p3 * eq28_e363_d_n10);
        let eq28_e364_d_n11: f64 = (p.p3 * eq28_e363_d_n11);
        let eq28_e364_d_n12: f64 = (p.p3 * eq28_e363_d_n12);
        let eq28_e365_q: f64 = eq28_e364;
        let eq28_e367: f64 = (eq28_e364 * p.p1);
        let eq28_e367_d_n0: f64 = (eq28_e364_d_n0 * p.p1);
        let eq28_e367_d_n1: f64 = (eq28_e364_d_n1 * p.p1);
        let eq28_e367_d_n2: f64 = (eq28_e364_d_n2 * p.p1);
        let eq28_e367_d_n3: f64 = (eq28_e364_d_n3 * p.p1);
        let eq28_e367_d_n4: f64 = (eq28_e364_d_n4 * p.p1);
        let eq28_e367_d_n5: f64 = (eq28_e364_d_n5 * p.p1);
        let eq28_e367_d_n6: f64 = (eq28_e364_d_n6 * p.p1);
        let eq28_e367_d_n7: f64 = (eq28_e364_d_n7 * p.p1);
        let eq28_e367_d_n8: f64 = (eq28_e364_d_n8 * p.p1);
        let eq28_e367_d_n9: f64 = (eq28_e364_d_n9 * p.p1);
        let eq28_e367_d_n10: f64 = (eq28_e364_d_n10 * p.p1);
        let eq28_e367_d_n11: f64 = (eq28_e364_d_n11 * p.p1);
        let eq28_e367_d_n12: f64 = (eq28_e364_d_n12 * p.p1);
        let eq28_e367_q: f64 = (eq28_e365_q * p.p1);
        let eq28_e367_q_d_n0: f64 = (eq28_e364_d_n0 * p.p1);
        let eq28_e367_q_d_n1: f64 = (eq28_e364_d_n1 * p.p1);
        let eq28_e367_q_d_n2: f64 = (eq28_e364_d_n2 * p.p1);
        let eq28_e367_q_d_n3: f64 = (eq28_e364_d_n3 * p.p1);
        let eq28_e367_q_d_n4: f64 = (eq28_e364_d_n4 * p.p1);
        let eq28_e367_q_d_n5: f64 = (eq28_e364_d_n5 * p.p1);
        let eq28_e367_q_d_n6: f64 = (eq28_e364_d_n6 * p.p1);
        let eq28_e367_q_d_n7: f64 = (eq28_e364_d_n7 * p.p1);
        let eq28_e367_q_d_n8: f64 = (eq28_e364_d_n8 * p.p1);
        let eq28_e367_q_d_n9: f64 = (eq28_e364_d_n9 * p.p1);
        let eq28_e367_q_d_n10: f64 = (eq28_e364_d_n10 * p.p1);
        let eq28_e367_q_d_n11: f64 = (eq28_e364_d_n11 * p.p1);
        let eq28_e367_q_d_n12: f64 = (eq28_e364_d_n12 * p.p1);
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e367_q_d_n0, eq28_e367_q_d_n1, eq28_e367_q_d_n2, eq28_e367_q_d_n3, eq28_e367_q_d_n4, eq28_e367_q_d_n5, eq28_e367_q_d_n6, eq28_e367_q_d_n7, eq28_e367_q_d_n8, eq28_e367_q_d_n9, eq28_e367_q_d_n10, eq28_e367_q_d_n11, eq28_e367_q_d_n12];
        let eq28_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            &nodes,
            &eq28_reactive_node_derivatives,
            &branches,
            &eq28_reactive_branch_derivatives,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq35_e406_q: f64 = (nv12 - 0.0);
        let eq35_e407: f64 = (s.v[336] * (nv12 - 0.0));
        let eq35_e407_d_n0: f64 = (s.dn[336][0] * (nv12 - 0.0));
        let eq35_e407_d_n1: f64 = (s.dn[336][1] * (nv12 - 0.0));
        let eq35_e407_d_n2: f64 = (s.dn[336][2] * (nv12 - 0.0));
        let eq35_e407_d_n3: f64 = (s.dn[336][3] * (nv12 - 0.0));
        let eq35_e407_d_n4: f64 = (s.dn[336][4] * (nv12 - 0.0));
        let eq35_e407_d_n5: f64 = (s.dn[336][5] * (nv12 - 0.0));
        let eq35_e407_d_n6: f64 = (s.dn[336][6] * (nv12 - 0.0));
        let eq35_e407_d_n7: f64 = (s.dn[336][7] * (nv12 - 0.0));
        let eq35_e407_d_n8: f64 = (s.dn[336][8] * (nv12 - 0.0));
        let eq35_e407_d_n9: f64 = (s.dn[336][9] * (nv12 - 0.0));
        let eq35_e407_d_n10: f64 = (s.dn[336][10] * (nv12 - 0.0));
        let eq35_e407_d_n11: f64 = (s.dn[336][11] * (nv12 - 0.0));
        let eq35_e407_d_n12: f64 = ((s.dn[336][12] * (nv12 - 0.0)) + s.v[336]);
        let eq35_e407_q: f64 = (s.v[336] * eq35_e406_q);
        let eq35_e407_q_d_n0: f64 = (s.dn[336][0] * eq35_e406_q);
        let eq35_e407_q_d_n1: f64 = (s.dn[336][1] * eq35_e406_q);
        let eq35_e407_q_d_n2: f64 = (s.dn[336][2] * eq35_e406_q);
        let eq35_e407_q_d_n3: f64 = (s.dn[336][3] * eq35_e406_q);
        let eq35_e407_q_d_n4: f64 = (s.dn[336][4] * eq35_e406_q);
        let eq35_e407_q_d_n5: f64 = (s.dn[336][5] * eq35_e406_q);
        let eq35_e407_q_d_n6: f64 = (s.dn[336][6] * eq35_e406_q);
        let eq35_e407_q_d_n7: f64 = (s.dn[336][7] * eq35_e406_q);
        let eq35_e407_q_d_n8: f64 = (s.dn[336][8] * eq35_e406_q);
        let eq35_e407_q_d_n9: f64 = (s.dn[336][9] * eq35_e406_q);
        let eq35_e407_q_d_n10: f64 = (s.dn[336][10] * eq35_e406_q);
        let eq35_e407_q_d_n11: f64 = (s.dn[336][11] * eq35_e406_q);
        let eq35_e407_q_d_n12: f64 = ((s.dn[336][12] * eq35_e406_q) + s.v[336]);
        let eq35_reactive_node_derivatives: [f64; 13] = [eq35_e407_q_d_n0, eq35_e407_q_d_n1, eq35_e407_q_d_n2, eq35_e407_q_d_n3, eq35_e407_q_d_n4, eq35_e407_q_d_n5, eq35_e407_q_d_n6, eq35_e407_q_d_n7, eq35_e407_q_d_n8, eq35_e407_q_d_n9, eq35_e407_q_d_n10, eq35_e407_q_d_n11, eq35_e407_q_d_n12];
        let eq35_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &nodes,
            &eq35_reactive_node_derivatives,
            &branches,
            &eq35_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
