#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_55_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq55_e538,) = {
    if ((!(s.v[580] != 0.0)) && (s.v[582] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e538;
        stamper.stamp_current(
            Some(nodes[10]),
            Some(nodes[7]),
            self.multiplicity * (eq55_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_56_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq56_e550,) = {
    if ((!(s.v[580] != 0.0)) && (!(s.v[582] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e550;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq56_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq11_e235_q: f64 = s.rv[209];
        stamper.stamp_current_reactive(
            Some(nodes[3]),
            None,
            &[
                GeneratedDerivative::node(nodes[3], self.multiplicity * (s.rdn[209][3])),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq13_e245: f64 = (s.v[210] + s.v[215]);
        let eq13_e245_d_n0: f64 = (s.dn[210][0] + s.dn[215][0]);
        let eq13_e245_d_n1: f64 = (s.dn[210][1] + s.dn[215][1]);
        let eq13_e245_d_n2: f64 = (s.dn[210][2] + s.dn[215][2]);
        let eq13_e245_d_n3: f64 = (s.dn[210][3] + s.dn[215][3]);
        let eq13_e245_d_n4: f64 = (s.dn[210][4] + s.dn[215][4]);
        let eq13_e245_d_n5: f64 = (s.dn[210][5] + s.dn[215][5]);
        let eq13_e245_d_n6: f64 = (s.dn[210][6] + s.dn[215][6]);
        let eq13_e245_d_n7: f64 = (s.dn[210][7] + s.dn[215][7]);
        let eq13_e245_d_n8: f64 = (s.dn[210][8] + s.dn[215][8]);
        let eq13_e245_d_n9: f64 = (s.dn[210][9] + s.dn[215][9]);
        let eq13_e245_d_n10: f64 = (s.dn[210][10] + s.dn[215][10]);
        let eq13_e245_d_n11: f64 = (s.dn[210][11] + s.dn[215][11]);
        let eq13_e247: f64 = (eq13_e245 + s.v[227]);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + s.dn[227][0]);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + s.dn[227][1]);
        let eq13_e247_d_n2: f64 = (eq13_e245_d_n2 + s.dn[227][2]);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + s.dn[227][3]);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + s.dn[227][4]);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + s.dn[227][5]);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + s.dn[227][6]);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + s.dn[227][7]);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + s.dn[227][8]);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + s.dn[227][9]);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + s.dn[227][10]);
        let eq13_e247_d_n11: f64 = (eq13_e245_d_n11 + s.dn[227][11]);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n2: f64 = (p.p3 * eq13_e247_d_n2);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e248_d_n11: f64 = (p.p3 * eq13_e247_d_n11);
        let eq13_e249_q: f64 = eq13_e248;
        let eq13_e251: f64 = (eq13_e248 * p.p1);
        let eq13_e251_d_n0: f64 = (eq13_e248_d_n0 * p.p1);
        let eq13_e251_d_n1: f64 = (eq13_e248_d_n1 * p.p1);
        let eq13_e251_d_n2: f64 = (eq13_e248_d_n2 * p.p1);
        let eq13_e251_d_n3: f64 = (eq13_e248_d_n3 * p.p1);
        let eq13_e251_d_n4: f64 = (eq13_e248_d_n4 * p.p1);
        let eq13_e251_d_n5: f64 = (eq13_e248_d_n5 * p.p1);
        let eq13_e251_d_n6: f64 = (eq13_e248_d_n6 * p.p1);
        let eq13_e251_d_n7: f64 = (eq13_e248_d_n7 * p.p1);
        let eq13_e251_d_n8: f64 = (eq13_e248_d_n8 * p.p1);
        let eq13_e251_d_n9: f64 = (eq13_e248_d_n9 * p.p1);
        let eq13_e251_d_n10: f64 = (eq13_e248_d_n10 * p.p1);
        let eq13_e251_d_n11: f64 = (eq13_e248_d_n11 * p.p1);
        let eq13_e251_q: f64 = (eq13_e249_q * p.p1);
        let eq13_e251_q_d_n0: f64 = (eq13_e248_d_n0 * p.p1);
        let eq13_e251_q_d_n1: f64 = (eq13_e248_d_n1 * p.p1);
        let eq13_e251_q_d_n2: f64 = (eq13_e248_d_n2 * p.p1);
        let eq13_e251_q_d_n3: f64 = (eq13_e248_d_n3 * p.p1);
        let eq13_e251_q_d_n4: f64 = (eq13_e248_d_n4 * p.p1);
        let eq13_e251_q_d_n5: f64 = (eq13_e248_d_n5 * p.p1);
        let eq13_e251_q_d_n6: f64 = (eq13_e248_d_n6 * p.p1);
        let eq13_e251_q_d_n7: f64 = (eq13_e248_d_n7 * p.p1);
        let eq13_e251_q_d_n8: f64 = (eq13_e248_d_n8 * p.p1);
        let eq13_e251_q_d_n9: f64 = (eq13_e248_d_n9 * p.p1);
        let eq13_e251_q_d_n10: f64 = (eq13_e248_d_n10 * p.p1);
        let eq13_e251_q_d_n11: f64 = (eq13_e248_d_n11 * p.p1);
        let eq13_reactive_node_derivatives: [f64; 12] = [eq13_e251_q_d_n0, eq13_e251_q_d_n1, eq13_e251_q_d_n2, eq13_e251_q_d_n3, eq13_e251_q_d_n4, eq13_e251_q_d_n5, eq13_e251_q_d_n6, eq13_e251_q_d_n7, eq13_e251_q_d_n8, eq13_e251_q_d_n9, eq13_e251_q_d_n10, eq13_e251_q_d_n11];
        let eq13_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &nodes,
            &eq13_reactive_node_derivatives,
            &branches,
            &eq13_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq14_e254: f64 = (p.p3 * s.v[212]);
        let eq14_e254_d_n0: f64 = (p.p3 * s.dn[212][0]);
        let eq14_e254_d_n1: f64 = (p.p3 * s.dn[212][1]);
        let eq14_e254_d_n2: f64 = (p.p3 * s.dn[212][2]);
        let eq14_e254_d_n3: f64 = (p.p3 * s.dn[212][3]);
        let eq14_e254_d_n4: f64 = (p.p3 * s.dn[212][4]);
        let eq14_e254_d_n5: f64 = (p.p3 * s.dn[212][5]);
        let eq14_e254_d_n6: f64 = (p.p3 * s.dn[212][6]);
        let eq14_e254_d_n7: f64 = (p.p3 * s.dn[212][7]);
        let eq14_e254_d_n8: f64 = (p.p3 * s.dn[212][8]);
        let eq14_e254_d_n9: f64 = (p.p3 * s.dn[212][9]);
        let eq14_e254_d_n10: f64 = (p.p3 * s.dn[212][10]);
        let eq14_e254_d_n11: f64 = (p.p3 * s.dn[212][11]);
        let eq14_e255_q: f64 = eq14_e254;
        let eq14_e257: f64 = (eq14_e254 * p.p1);
        let eq14_e257_d_n0: f64 = (eq14_e254_d_n0 * p.p1);
        let eq14_e257_d_n1: f64 = (eq14_e254_d_n1 * p.p1);
        let eq14_e257_d_n2: f64 = (eq14_e254_d_n2 * p.p1);
        let eq14_e257_d_n3: f64 = (eq14_e254_d_n3 * p.p1);
        let eq14_e257_d_n4: f64 = (eq14_e254_d_n4 * p.p1);
        let eq14_e257_d_n5: f64 = (eq14_e254_d_n5 * p.p1);
        let eq14_e257_d_n6: f64 = (eq14_e254_d_n6 * p.p1);
        let eq14_e257_d_n7: f64 = (eq14_e254_d_n7 * p.p1);
        let eq14_e257_d_n8: f64 = (eq14_e254_d_n8 * p.p1);
        let eq14_e257_d_n9: f64 = (eq14_e254_d_n9 * p.p1);
        let eq14_e257_d_n10: f64 = (eq14_e254_d_n10 * p.p1);
        let eq14_e257_d_n11: f64 = (eq14_e254_d_n11 * p.p1);
        let eq14_e257_q: f64 = (eq14_e255_q * p.p1);
        let eq14_e257_q_d_n0: f64 = (eq14_e254_d_n0 * p.p1);
        let eq14_e257_q_d_n1: f64 = (eq14_e254_d_n1 * p.p1);
        let eq14_e257_q_d_n2: f64 = (eq14_e254_d_n2 * p.p1);
        let eq14_e257_q_d_n3: f64 = (eq14_e254_d_n3 * p.p1);
        let eq14_e257_q_d_n4: f64 = (eq14_e254_d_n4 * p.p1);
        let eq14_e257_q_d_n5: f64 = (eq14_e254_d_n5 * p.p1);
        let eq14_e257_q_d_n6: f64 = (eq14_e254_d_n6 * p.p1);
        let eq14_e257_q_d_n7: f64 = (eq14_e254_d_n7 * p.p1);
        let eq14_e257_q_d_n8: f64 = (eq14_e254_d_n8 * p.p1);
        let eq14_e257_q_d_n9: f64 = (eq14_e254_d_n9 * p.p1);
        let eq14_e257_q_d_n10: f64 = (eq14_e254_d_n10 * p.p1);
        let eq14_e257_q_d_n11: f64 = (eq14_e254_d_n11 * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e257_q_d_n0, eq14_e257_q_d_n1, eq14_e257_q_d_n2, eq14_e257_q_d_n3, eq14_e257_q_d_n4, eq14_e257_q_d_n5, eq14_e257_q_d_n6, eq14_e257_q_d_n7, eq14_e257_q_d_n8, eq14_e257_q_d_n9, eq14_e257_q_d_n10, eq14_e257_q_d_n11];
        let eq14_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &nodes,
            &eq14_reactive_node_derivatives,
            &branches,
            &eq14_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq15_e261: f64 = (s.v[213] + s.v[216]);
        let eq15_e261_d_n0: f64 = (s.dn[213][0] + s.dn[216][0]);
        let eq15_e261_d_n1: f64 = (s.dn[213][1] + s.dn[216][1]);
        let eq15_e261_d_n2: f64 = (s.dn[213][2] + s.dn[216][2]);
        let eq15_e261_d_n3: f64 = (s.dn[213][3] + s.dn[216][3]);
        let eq15_e261_d_n4: f64 = (s.dn[213][4] + s.dn[216][4]);
        let eq15_e261_d_n5: f64 = (s.dn[213][5] + s.dn[216][5]);
        let eq15_e261_d_n6: f64 = (s.dn[213][6] + s.dn[216][6]);
        let eq15_e261_d_n7: f64 = (s.dn[213][7] + s.dn[216][7]);
        let eq15_e261_d_n8: f64 = (s.dn[213][8] + s.dn[216][8]);
        let eq15_e261_d_n9: f64 = (s.dn[213][9] + s.dn[216][9]);
        let eq15_e261_d_n10: f64 = (s.dn[213][10] + s.dn[216][10]);
        let eq15_e261_d_n11: f64 = (s.dn[213][11] + s.dn[216][11]);
        let eq15_e263: f64 = (eq15_e261 + s.v[230]);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + s.dn[230][0]);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + s.dn[230][1]);
        let eq15_e263_d_n2: f64 = (eq15_e261_d_n2 + s.dn[230][2]);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + s.dn[230][3]);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + s.dn[230][4]);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + s.dn[230][5]);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + s.dn[230][6]);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + s.dn[230][7]);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + s.dn[230][8]);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + s.dn[230][9]);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + s.dn[230][10]);
        let eq15_e263_d_n11: f64 = (eq15_e261_d_n11 + s.dn[230][11]);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n2: f64 = (p.p3 * eq15_e263_d_n2);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e264_d_n11: f64 = (p.p3 * eq15_e263_d_n11);
        let eq15_e265_q: f64 = eq15_e264;
        let eq15_e267: f64 = (eq15_e264 * p.p1);
        let eq15_e267_d_n0: f64 = (eq15_e264_d_n0 * p.p1);
        let eq15_e267_d_n1: f64 = (eq15_e264_d_n1 * p.p1);
        let eq15_e267_d_n2: f64 = (eq15_e264_d_n2 * p.p1);
        let eq15_e267_d_n3: f64 = (eq15_e264_d_n3 * p.p1);
        let eq15_e267_d_n4: f64 = (eq15_e264_d_n4 * p.p1);
        let eq15_e267_d_n5: f64 = (eq15_e264_d_n5 * p.p1);
        let eq15_e267_d_n6: f64 = (eq15_e264_d_n6 * p.p1);
        let eq15_e267_d_n7: f64 = (eq15_e264_d_n7 * p.p1);
        let eq15_e267_d_n8: f64 = (eq15_e264_d_n8 * p.p1);
        let eq15_e267_d_n9: f64 = (eq15_e264_d_n9 * p.p1);
        let eq15_e267_d_n10: f64 = (eq15_e264_d_n10 * p.p1);
        let eq15_e267_d_n11: f64 = (eq15_e264_d_n11 * p.p1);
        let eq15_e267_q: f64 = (eq15_e265_q * p.p1);
        let eq15_e267_q_d_n0: f64 = (eq15_e264_d_n0 * p.p1);
        let eq15_e267_q_d_n1: f64 = (eq15_e264_d_n1 * p.p1);
        let eq15_e267_q_d_n2: f64 = (eq15_e264_d_n2 * p.p1);
        let eq15_e267_q_d_n3: f64 = (eq15_e264_d_n3 * p.p1);
        let eq15_e267_q_d_n4: f64 = (eq15_e264_d_n4 * p.p1);
        let eq15_e267_q_d_n5: f64 = (eq15_e264_d_n5 * p.p1);
        let eq15_e267_q_d_n6: f64 = (eq15_e264_d_n6 * p.p1);
        let eq15_e267_q_d_n7: f64 = (eq15_e264_d_n7 * p.p1);
        let eq15_e267_q_d_n8: f64 = (eq15_e264_d_n8 * p.p1);
        let eq15_e267_q_d_n9: f64 = (eq15_e264_d_n9 * p.p1);
        let eq15_e267_q_d_n10: f64 = (eq15_e264_d_n10 * p.p1);
        let eq15_e267_q_d_n11: f64 = (eq15_e264_d_n11 * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e267_q_d_n0, eq15_e267_q_d_n1, eq15_e267_q_d_n2, eq15_e267_q_d_n3, eq15_e267_q_d_n4, eq15_e267_q_d_n5, eq15_e267_q_d_n6, eq15_e267_q_d_n7, eq15_e267_q_d_n8, eq15_e267_q_d_n9, eq15_e267_q_d_n10, eq15_e267_q_d_n11];
        let eq15_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &nodes,
            &eq15_reactive_node_derivatives,
            &branches,
            &eq15_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_16_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq16_e270: f64 = (p.p3 * s.v[217]);
        let eq16_e270_d_n0: f64 = (p.p3 * s.dn[217][0]);
        let eq16_e270_d_n1: f64 = (p.p3 * s.dn[217][1]);
        let eq16_e270_d_n2: f64 = (p.p3 * s.dn[217][2]);
        let eq16_e270_d_n3: f64 = (p.p3 * s.dn[217][3]);
        let eq16_e270_d_n4: f64 = (p.p3 * s.dn[217][4]);
        let eq16_e270_d_n5: f64 = (p.p3 * s.dn[217][5]);
        let eq16_e270_d_n6: f64 = (p.p3 * s.dn[217][6]);
        let eq16_e270_d_n7: f64 = (p.p3 * s.dn[217][7]);
        let eq16_e270_d_n8: f64 = (p.p3 * s.dn[217][8]);
        let eq16_e270_d_n9: f64 = (p.p3 * s.dn[217][9]);
        let eq16_e270_d_n10: f64 = (p.p3 * s.dn[217][10]);
        let eq16_e270_d_n11: f64 = (p.p3 * s.dn[217][11]);
        let eq16_e271_q: f64 = eq16_e270;
        let eq16_e273: f64 = (eq16_e270 * p.p1);
        let eq16_e273_d_n0: f64 = (eq16_e270_d_n0 * p.p1);
        let eq16_e273_d_n1: f64 = (eq16_e270_d_n1 * p.p1);
        let eq16_e273_d_n2: f64 = (eq16_e270_d_n2 * p.p1);
        let eq16_e273_d_n3: f64 = (eq16_e270_d_n3 * p.p1);
        let eq16_e273_d_n4: f64 = (eq16_e270_d_n4 * p.p1);
        let eq16_e273_d_n5: f64 = (eq16_e270_d_n5 * p.p1);
        let eq16_e273_d_n6: f64 = (eq16_e270_d_n6 * p.p1);
        let eq16_e273_d_n7: f64 = (eq16_e270_d_n7 * p.p1);
        let eq16_e273_d_n8: f64 = (eq16_e270_d_n8 * p.p1);
        let eq16_e273_d_n9: f64 = (eq16_e270_d_n9 * p.p1);
        let eq16_e273_d_n10: f64 = (eq16_e270_d_n10 * p.p1);
        let eq16_e273_d_n11: f64 = (eq16_e270_d_n11 * p.p1);
        let eq16_e273_q: f64 = (eq16_e271_q * p.p1);
        let eq16_e273_q_d_n0: f64 = (eq16_e270_d_n0 * p.p1);
        let eq16_e273_q_d_n1: f64 = (eq16_e270_d_n1 * p.p1);
        let eq16_e273_q_d_n2: f64 = (eq16_e270_d_n2 * p.p1);
        let eq16_e273_q_d_n3: f64 = (eq16_e270_d_n3 * p.p1);
        let eq16_e273_q_d_n4: f64 = (eq16_e270_d_n4 * p.p1);
        let eq16_e273_q_d_n5: f64 = (eq16_e270_d_n5 * p.p1);
        let eq16_e273_q_d_n6: f64 = (eq16_e270_d_n6 * p.p1);
        let eq16_e273_q_d_n7: f64 = (eq16_e270_d_n7 * p.p1);
        let eq16_e273_q_d_n8: f64 = (eq16_e270_d_n8 * p.p1);
        let eq16_e273_q_d_n9: f64 = (eq16_e270_d_n9 * p.p1);
        let eq16_e273_q_d_n10: f64 = (eq16_e270_d_n10 * p.p1);
        let eq16_e273_q_d_n11: f64 = (eq16_e270_d_n11 * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e273_q_d_n0, eq16_e273_q_d_n1, eq16_e273_q_d_n2, eq16_e273_q_d_n3, eq16_e273_q_d_n4, eq16_e273_q_d_n5, eq16_e273_q_d_n6, eq16_e273_q_d_n7, eq16_e273_q_d_n8, eq16_e273_q_d_n9, eq16_e273_q_d_n10, eq16_e273_q_d_n11];
        let eq16_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &nodes,
            &eq16_reactive_node_derivatives,
            &branches,
            &eq16_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * s.v[249]);
        let eq17_e278_d_n0: f64 = (eq17_e276 * s.dn[249][0]);
        let eq17_e278_d_n1: f64 = (eq17_e276 * s.dn[249][1]);
        let eq17_e278_d_n2: f64 = (eq17_e276 * s.dn[249][2]);
        let eq17_e278_d_n3: f64 = (eq17_e276 * s.dn[249][3]);
        let eq17_e278_d_n4: f64 = (eq17_e276 * s.dn[249][4]);
        let eq17_e278_d_n5: f64 = (eq17_e276 * s.dn[249][5]);
        let eq17_e278_d_n6: f64 = (eq17_e276 * s.dn[249][6]);
        let eq17_e278_d_n7: f64 = (eq17_e276 * s.dn[249][7]);
        let eq17_e278_d_n8: f64 = (eq17_e276 * s.dn[249][8]);
        let eq17_e278_d_n9: f64 = (eq17_e276 * s.dn[249][9]);
        let eq17_e278_d_n10: f64 = (eq17_e276 * s.dn[249][10]);
        let eq17_e278_d_n11: f64 = (eq17_e276 * s.dn[249][11]);
        let eq17_e279_q: f64 = eq17_e278;
        let eq17_e281: f64 = (eq17_e278 * p.p1);
        let eq17_e281_d_n0: f64 = (eq17_e278_d_n0 * p.p1);
        let eq17_e281_d_n1: f64 = (eq17_e278_d_n1 * p.p1);
        let eq17_e281_d_n2: f64 = (eq17_e278_d_n2 * p.p1);
        let eq17_e281_d_n3: f64 = (eq17_e278_d_n3 * p.p1);
        let eq17_e281_d_n4: f64 = (eq17_e278_d_n4 * p.p1);
        let eq17_e281_d_n5: f64 = (eq17_e278_d_n5 * p.p1);
        let eq17_e281_d_n6: f64 = (eq17_e278_d_n6 * p.p1);
        let eq17_e281_d_n7: f64 = (eq17_e278_d_n7 * p.p1);
        let eq17_e281_d_n8: f64 = (eq17_e278_d_n8 * p.p1);
        let eq17_e281_d_n9: f64 = (eq17_e278_d_n9 * p.p1);
        let eq17_e281_d_n10: f64 = (eq17_e278_d_n10 * p.p1);
        let eq17_e281_d_n11: f64 = (eq17_e278_d_n11 * p.p1);
        let eq17_e281_q: f64 = (eq17_e279_q * p.p1);
        let eq17_e281_q_d_n0: f64 = (eq17_e278_d_n0 * p.p1);
        let eq17_e281_q_d_n1: f64 = (eq17_e278_d_n1 * p.p1);
        let eq17_e281_q_d_n2: f64 = (eq17_e278_d_n2 * p.p1);
        let eq17_e281_q_d_n3: f64 = (eq17_e278_d_n3 * p.p1);
        let eq17_e281_q_d_n4: f64 = (eq17_e278_d_n4 * p.p1);
        let eq17_e281_q_d_n5: f64 = (eq17_e278_d_n5 * p.p1);
        let eq17_e281_q_d_n6: f64 = (eq17_e278_d_n6 * p.p1);
        let eq17_e281_q_d_n7: f64 = (eq17_e278_d_n7 * p.p1);
        let eq17_e281_q_d_n8: f64 = (eq17_e278_d_n8 * p.p1);
        let eq17_e281_q_d_n9: f64 = (eq17_e278_d_n9 * p.p1);
        let eq17_e281_q_d_n10: f64 = (eq17_e278_d_n10 * p.p1);
        let eq17_e281_q_d_n11: f64 = (eq17_e278_d_n11 * p.p1);
        let eq17_reactive_node_derivatives: [f64; 12] = [eq17_e281_q_d_n0, eq17_e281_q_d_n1, eq17_e281_q_d_n2, eq17_e281_q_d_n3, eq17_e281_q_d_n4, eq17_e281_q_d_n5, eq17_e281_q_d_n6, eq17_e281_q_d_n7, eq17_e281_q_d_n8, eq17_e281_q_d_n9, eq17_e281_q_d_n10, eq17_e281_q_d_n11];
        let eq17_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            &nodes,
            &eq17_reactive_node_derivatives,
            &branches,
            &eq17_reactive_branch_derivatives,
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
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * s.v[250]);
        let eq18_e286_d_n0: f64 = (eq18_e284 * s.dn[250][0]);
        let eq18_e286_d_n1: f64 = (eq18_e284 * s.dn[250][1]);
        let eq18_e286_d_n2: f64 = (eq18_e284 * s.dn[250][2]);
        let eq18_e286_d_n3: f64 = (eq18_e284 * s.dn[250][3]);
        let eq18_e286_d_n4: f64 = (eq18_e284 * s.dn[250][4]);
        let eq18_e286_d_n5: f64 = (eq18_e284 * s.dn[250][5]);
        let eq18_e286_d_n6: f64 = (eq18_e284 * s.dn[250][6]);
        let eq18_e286_d_n7: f64 = (eq18_e284 * s.dn[250][7]);
        let eq18_e286_d_n8: f64 = (eq18_e284 * s.dn[250][8]);
        let eq18_e286_d_n9: f64 = (eq18_e284 * s.dn[250][9]);
        let eq18_e286_d_n10: f64 = (eq18_e284 * s.dn[250][10]);
        let eq18_e286_d_n11: f64 = (eq18_e284 * s.dn[250][11]);
        let eq18_e287_q: f64 = eq18_e286;
        let eq18_e289: f64 = (eq18_e286 * p.p1);
        let eq18_e289_d_n0: f64 = (eq18_e286_d_n0 * p.p1);
        let eq18_e289_d_n1: f64 = (eq18_e286_d_n1 * p.p1);
        let eq18_e289_d_n2: f64 = (eq18_e286_d_n2 * p.p1);
        let eq18_e289_d_n3: f64 = (eq18_e286_d_n3 * p.p1);
        let eq18_e289_d_n4: f64 = (eq18_e286_d_n4 * p.p1);
        let eq18_e289_d_n5: f64 = (eq18_e286_d_n5 * p.p1);
        let eq18_e289_d_n6: f64 = (eq18_e286_d_n6 * p.p1);
        let eq18_e289_d_n7: f64 = (eq18_e286_d_n7 * p.p1);
        let eq18_e289_d_n8: f64 = (eq18_e286_d_n8 * p.p1);
        let eq18_e289_d_n9: f64 = (eq18_e286_d_n9 * p.p1);
        let eq18_e289_d_n10: f64 = (eq18_e286_d_n10 * p.p1);
        let eq18_e289_d_n11: f64 = (eq18_e286_d_n11 * p.p1);
        let eq18_e289_q: f64 = (eq18_e287_q * p.p1);
        let eq18_e289_q_d_n0: f64 = (eq18_e286_d_n0 * p.p1);
        let eq18_e289_q_d_n1: f64 = (eq18_e286_d_n1 * p.p1);
        let eq18_e289_q_d_n2: f64 = (eq18_e286_d_n2 * p.p1);
        let eq18_e289_q_d_n3: f64 = (eq18_e286_d_n3 * p.p1);
        let eq18_e289_q_d_n4: f64 = (eq18_e286_d_n4 * p.p1);
        let eq18_e289_q_d_n5: f64 = (eq18_e286_d_n5 * p.p1);
        let eq18_e289_q_d_n6: f64 = (eq18_e286_d_n6 * p.p1);
        let eq18_e289_q_d_n7: f64 = (eq18_e286_d_n7 * p.p1);
        let eq18_e289_q_d_n8: f64 = (eq18_e286_d_n8 * p.p1);
        let eq18_e289_q_d_n9: f64 = (eq18_e286_d_n9 * p.p1);
        let eq18_e289_q_d_n10: f64 = (eq18_e286_d_n10 * p.p1);
        let eq18_e289_q_d_n11: f64 = (eq18_e286_d_n11 * p.p1);
        let eq18_reactive_node_derivatives: [f64; 12] = [eq18_e289_q_d_n0, eq18_e289_q_d_n1, eq18_e289_q_d_n2, eq18_e289_q_d_n3, eq18_e289_q_d_n4, eq18_e289_q_d_n5, eq18_e289_q_d_n6, eq18_e289_q_d_n7, eq18_e289_q_d_n8, eq18_e289_q_d_n9, eq18_e289_q_d_n10, eq18_e289_q_d_n11];
        let eq18_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            &nodes,
            &eq18_reactive_node_derivatives,
            &branches,
            &eq18_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_21_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq21_e305: f64 = (s.v[225] + s.v[234]);
        let eq21_e305_d_n0: f64 = (s.dn[225][0] + s.dn[234][0]);
        let eq21_e305_d_n1: f64 = (s.dn[225][1] + s.dn[234][1]);
        let eq21_e305_d_n2: f64 = (s.dn[225][2] + s.dn[234][2]);
        let eq21_e305_d_n3: f64 = (s.dn[225][3] + s.dn[234][3]);
        let eq21_e305_d_n4: f64 = (s.dn[225][4] + s.dn[234][4]);
        let eq21_e305_d_n5: f64 = (s.dn[225][5] + s.dn[234][5]);
        let eq21_e305_d_n6: f64 = (s.dn[225][6] + s.dn[234][6]);
        let eq21_e305_d_n7: f64 = (s.dn[225][7] + s.dn[234][7]);
        let eq21_e305_d_n8: f64 = (s.dn[225][8] + s.dn[234][8]);
        let eq21_e305_d_n9: f64 = (s.dn[225][9] + s.dn[234][9]);
        let eq21_e305_d_n10: f64 = (s.dn[225][10] + s.dn[234][10]);
        let eq21_e305_d_n11: f64 = (s.dn[225][11] + s.dn[234][11]);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n2: f64 = (p.p3 * eq21_e305_d_n2);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e306_d_n11: f64 = (p.p3 * eq21_e305_d_n11);
        let eq21_e307_q: f64 = eq21_e306;
        let eq21_e309: f64 = (eq21_e306 * p.p1);
        let eq21_e309_d_n0: f64 = (eq21_e306_d_n0 * p.p1);
        let eq21_e309_d_n1: f64 = (eq21_e306_d_n1 * p.p1);
        let eq21_e309_d_n2: f64 = (eq21_e306_d_n2 * p.p1);
        let eq21_e309_d_n3: f64 = (eq21_e306_d_n3 * p.p1);
        let eq21_e309_d_n4: f64 = (eq21_e306_d_n4 * p.p1);
        let eq21_e309_d_n5: f64 = (eq21_e306_d_n5 * p.p1);
        let eq21_e309_d_n6: f64 = (eq21_e306_d_n6 * p.p1);
        let eq21_e309_d_n7: f64 = (eq21_e306_d_n7 * p.p1);
        let eq21_e309_d_n8: f64 = (eq21_e306_d_n8 * p.p1);
        let eq21_e309_d_n9: f64 = (eq21_e306_d_n9 * p.p1);
        let eq21_e309_d_n10: f64 = (eq21_e306_d_n10 * p.p1);
        let eq21_e309_d_n11: f64 = (eq21_e306_d_n11 * p.p1);
        let eq21_e309_q: f64 = (eq21_e307_q * p.p1);
        let eq21_e309_q_d_n0: f64 = (eq21_e306_d_n0 * p.p1);
        let eq21_e309_q_d_n1: f64 = (eq21_e306_d_n1 * p.p1);
        let eq21_e309_q_d_n2: f64 = (eq21_e306_d_n2 * p.p1);
        let eq21_e309_q_d_n3: f64 = (eq21_e306_d_n3 * p.p1);
        let eq21_e309_q_d_n4: f64 = (eq21_e306_d_n4 * p.p1);
        let eq21_e309_q_d_n5: f64 = (eq21_e306_d_n5 * p.p1);
        let eq21_e309_q_d_n6: f64 = (eq21_e306_d_n6 * p.p1);
        let eq21_e309_q_d_n7: f64 = (eq21_e306_d_n7 * p.p1);
        let eq21_e309_q_d_n8: f64 = (eq21_e306_d_n8 * p.p1);
        let eq21_e309_q_d_n9: f64 = (eq21_e306_d_n9 * p.p1);
        let eq21_e309_q_d_n10: f64 = (eq21_e306_d_n10 * p.p1);
        let eq21_e309_q_d_n11: f64 = (eq21_e306_d_n11 * p.p1);
        let eq21_reactive_node_derivatives: [f64; 12] = [eq21_e309_q_d_n0, eq21_e309_q_d_n1, eq21_e309_q_d_n2, eq21_e309_q_d_n3, eq21_e309_q_d_n4, eq21_e309_q_d_n5, eq21_e309_q_d_n6, eq21_e309_q_d_n7, eq21_e309_q_d_n8, eq21_e309_q_d_n9, eq21_e309_q_d_n10, eq21_e309_q_d_n11];
        let eq21_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            &nodes,
            &eq21_reactive_node_derivatives,
            &branches,
            &eq21_reactive_branch_derivatives,
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
        let eq23_e324: f64 = (s.v[222] + s.v[235]);
        let eq23_e324_d_n0: f64 = (s.dn[222][0] + s.dn[235][0]);
        let eq23_e324_d_n1: f64 = (s.dn[222][1] + s.dn[235][1]);
        let eq23_e324_d_n2: f64 = (s.dn[222][2] + s.dn[235][2]);
        let eq23_e324_d_n3: f64 = (s.dn[222][3] + s.dn[235][3]);
        let eq23_e324_d_n4: f64 = (s.dn[222][4] + s.dn[235][4]);
        let eq23_e324_d_n5: f64 = (s.dn[222][5] + s.dn[235][5]);
        let eq23_e324_d_n6: f64 = (s.dn[222][6] + s.dn[235][6]);
        let eq23_e324_d_n7: f64 = (s.dn[222][7] + s.dn[235][7]);
        let eq23_e324_d_n8: f64 = (s.dn[222][8] + s.dn[235][8]);
        let eq23_e324_d_n9: f64 = (s.dn[222][9] + s.dn[235][9]);
        let eq23_e324_d_n10: f64 = (s.dn[222][10] + s.dn[235][10]);
        let eq23_e324_d_n11: f64 = (s.dn[222][11] + s.dn[235][11]);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n2: f64 = (p.p3 * eq23_e324_d_n2);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e325_d_n11: f64 = (p.p3 * eq23_e324_d_n11);
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
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e328_q_d_n0, eq23_e328_q_d_n1, eq23_e328_q_d_n2, eq23_e328_q_d_n3, eq23_e328_q_d_n4, eq23_e328_q_d_n5, eq23_e328_q_d_n6, eq23_e328_q_d_n7, eq23_e328_q_d_n8, eq23_e328_q_d_n9, eq23_e328_q_d_n10, eq23_e328_q_d_n11];
        let eq23_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            &nodes,
            &eq23_reactive_node_derivatives,
            &branches,
            &eq23_reactive_branch_derivatives,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq30_e367_q: f64 = (nv11 - 0.0);
        let eq30_e368: f64 = (s.v[312] * (nv11 - 0.0));
        let eq30_e368_d_n0: f64 = (s.dn[312][0] * (nv11 - 0.0));
        let eq30_e368_d_n1: f64 = (s.dn[312][1] * (nv11 - 0.0));
        let eq30_e368_d_n2: f64 = (s.dn[312][2] * (nv11 - 0.0));
        let eq30_e368_d_n3: f64 = (s.dn[312][3] * (nv11 - 0.0));
        let eq30_e368_d_n4: f64 = (s.dn[312][4] * (nv11 - 0.0));
        let eq30_e368_d_n5: f64 = (s.dn[312][5] * (nv11 - 0.0));
        let eq30_e368_d_n6: f64 = (s.dn[312][6] * (nv11 - 0.0));
        let eq30_e368_d_n7: f64 = (s.dn[312][7] * (nv11 - 0.0));
        let eq30_e368_d_n8: f64 = (s.dn[312][8] * (nv11 - 0.0));
        let eq30_e368_d_n9: f64 = (s.dn[312][9] * (nv11 - 0.0));
        let eq30_e368_d_n10: f64 = (s.dn[312][10] * (nv11 - 0.0));
        let eq30_e368_d_n11: f64 = ((s.dn[312][11] * (nv11 - 0.0)) + s.v[312]);
        let eq30_e368_q: f64 = (s.v[312] * eq30_e367_q);
        let eq30_e368_q_d_n0: f64 = (s.dn[312][0] * eq30_e367_q);
        let eq30_e368_q_d_n1: f64 = (s.dn[312][1] * eq30_e367_q);
        let eq30_e368_q_d_n2: f64 = (s.dn[312][2] * eq30_e367_q);
        let eq30_e368_q_d_n3: f64 = (s.dn[312][3] * eq30_e367_q);
        let eq30_e368_q_d_n4: f64 = (s.dn[312][4] * eq30_e367_q);
        let eq30_e368_q_d_n5: f64 = (s.dn[312][5] * eq30_e367_q);
        let eq30_e368_q_d_n6: f64 = (s.dn[312][6] * eq30_e367_q);
        let eq30_e368_q_d_n7: f64 = (s.dn[312][7] * eq30_e367_q);
        let eq30_e368_q_d_n8: f64 = (s.dn[312][8] * eq30_e367_q);
        let eq30_e368_q_d_n9: f64 = (s.dn[312][9] * eq30_e367_q);
        let eq30_e368_q_d_n10: f64 = (s.dn[312][10] * eq30_e367_q);
        let eq30_e368_q_d_n11: f64 = ((s.dn[312][11] * eq30_e367_q) + s.v[312]);
        let eq30_reactive_node_derivatives: [f64; 12] = [eq30_e368_q_d_n0, eq30_e368_q_d_n1, eq30_e368_q_d_n2, eq30_e368_q_d_n3, eq30_e368_q_d_n4, eq30_e368_q_d_n5, eq30_e368_q_d_n6, eq30_e368_q_d_n7, eq30_e368_q_d_n8, eq30_e368_q_d_n9, eq30_e368_q_d_n10, eq30_e368_q_d_n11];
        let eq30_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &nodes,
            &eq30_reactive_node_derivatives,
            &branches,
            &eq30_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
