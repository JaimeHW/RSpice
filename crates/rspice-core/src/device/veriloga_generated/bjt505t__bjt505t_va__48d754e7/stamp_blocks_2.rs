#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq23_e326: f64 = self.eval_ddt(7, eq23_e325);
        let eq23_e326_d_n0: f64 = self.ddt_jacobian(eq23_e325_d_n0);
        let eq23_e326_d_n1: f64 = self.ddt_jacobian(eq23_e325_d_n1);
        let eq23_e326_d_n2: f64 = self.ddt_jacobian(eq23_e325_d_n2);
        let eq23_e326_d_n3: f64 = self.ddt_jacobian(eq23_e325_d_n3);
        let eq23_e326_d_n4: f64 = self.ddt_jacobian(eq23_e325_d_n4);
        let eq23_e326_d_n5: f64 = self.ddt_jacobian(eq23_e325_d_n5);
        let eq23_e326_d_n6: f64 = self.ddt_jacobian(eq23_e325_d_n6);
        let eq23_e326_d_n7: f64 = self.ddt_jacobian(eq23_e325_d_n7);
        let eq23_e326_d_n8: f64 = self.ddt_jacobian(eq23_e325_d_n8);
        let eq23_e326_d_n9: f64 = self.ddt_jacobian(eq23_e325_d_n9);
        let eq23_e326_d_n10: f64 = self.ddt_jacobian(eq23_e325_d_n10);
        let eq23_e326_d_n11: f64 = self.ddt_jacobian(eq23_e325_d_n11);
        let eq23_e326_d_n12: f64 = self.ddt_jacobian(eq23_e325_d_n12);
        let eq23_e328: f64 = (eq23_e326 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e326_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e326_d_n1 * p.p1);
        let eq23_e328_d_n2: f64 = (eq23_e326_d_n2 * p.p1);
        let eq23_e328_d_n3: f64 = (eq23_e326_d_n3 * p.p1);
        let eq23_e328_d_n4: f64 = (eq23_e326_d_n4 * p.p1);
        let eq23_e328_d_n5: f64 = (eq23_e326_d_n5 * p.p1);
        let eq23_e328_d_n6: f64 = (eq23_e326_d_n6 * p.p1);
        let eq23_e328_d_n7: f64 = (eq23_e326_d_n7 * p.p1);
        let eq23_e328_d_n8: f64 = (eq23_e326_d_n8 * p.p1);
        let eq23_e328_d_n9: f64 = (eq23_e326_d_n9 * p.p1);
        let eq23_e328_d_n10: f64 = (eq23_e326_d_n10 * p.p1);
        let eq23_e328_d_n11: f64 = (eq23_e326_d_n11 * p.p1);
        let eq23_e328_d_n12: f64 = (eq23_e326_d_n12 * p.p1);
        let eq23_value: f64 = eq23_e328;
        let eq23_node_derivatives: [f64; 13] = [eq23_e328_d_n0, eq23_e328_d_n1, eq23_e328_d_n2, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, eq23_e328_d_n11, eq23_e328_d_n12];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            self.multiplicity * (eq23_value),
            &nodes,
            &eq23_node_derivatives,
            &branches,
            &eq23_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_24_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq24_e331: f64 = (p.p3 * s.v[179]);
        let eq24_e331_d_n0: f64 = (p.p3 * s.dn[179][0]);
        let eq24_e331_d_n1: f64 = (p.p3 * s.dn[179][1]);
        let eq24_e331_d_n2: f64 = (p.p3 * s.dn[179][2]);
        let eq24_e331_d_n3: f64 = (p.p3 * s.dn[179][3]);
        let eq24_e331_d_n4: f64 = (p.p3 * s.dn[179][4]);
        let eq24_e331_d_n5: f64 = (p.p3 * s.dn[179][5]);
        let eq24_e331_d_n6: f64 = (p.p3 * s.dn[179][6]);
        let eq24_e331_d_n7: f64 = (p.p3 * s.dn[179][7]);
        let eq24_e331_d_n8: f64 = (p.p3 * s.dn[179][8]);
        let eq24_e331_d_n9: f64 = (p.p3 * s.dn[179][9]);
        let eq24_e331_d_n10: f64 = (p.p3 * s.dn[179][10]);
        let eq24_e331_d_n11: f64 = (p.p3 * s.dn[179][11]);
        let eq24_e331_d_n12: f64 = (p.p3 * s.dn[179][12]);
        let eq24_e333: f64 = (eq24_e331 * p.p1);
        let eq24_e333_d_n0: f64 = (eq24_e331_d_n0 * p.p1);
        let eq24_e333_d_n1: f64 = (eq24_e331_d_n1 * p.p1);
        let eq24_e333_d_n2: f64 = (eq24_e331_d_n2 * p.p1);
        let eq24_e333_d_n3: f64 = (eq24_e331_d_n3 * p.p1);
        let eq24_e333_d_n4: f64 = (eq24_e331_d_n4 * p.p1);
        let eq24_e333_d_n5: f64 = (eq24_e331_d_n5 * p.p1);
        let eq24_e333_d_n6: f64 = (eq24_e331_d_n6 * p.p1);
        let eq24_e333_d_n7: f64 = (eq24_e331_d_n7 * p.p1);
        let eq24_e333_d_n8: f64 = (eq24_e331_d_n8 * p.p1);
        let eq24_e333_d_n9: f64 = (eq24_e331_d_n9 * p.p1);
        let eq24_e333_d_n10: f64 = (eq24_e331_d_n10 * p.p1);
        let eq24_e333_d_n11: f64 = (eq24_e331_d_n11 * p.p1);
        let eq24_e333_d_n12: f64 = (eq24_e331_d_n12 * p.p1);
        let eq24_value: f64 = eq24_e333;
        let eq24_node_derivatives: [f64; 13] = [eq24_e333_d_n0, eq24_e333_d_n1, eq24_e333_d_n2, eq24_e333_d_n3, eq24_e333_d_n4, eq24_e333_d_n5, eq24_e333_d_n6, eq24_e333_d_n7, eq24_e333_d_n8, eq24_e333_d_n9, eq24_e333_d_n10, eq24_e333_d_n11, eq24_e333_d_n12];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            self.multiplicity * (eq24_value),
            &nodes,
            &eq24_node_derivatives,
            &branches,
            &eq24_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_25_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq25_e336: f64 = (p.p3 * s.v[268]);
        let eq25_e336_d_n0: f64 = (p.p3 * s.dn[268][0]);
        let eq25_e336_d_n1: f64 = (p.p3 * s.dn[268][1]);
        let eq25_e336_d_n2: f64 = (p.p3 * s.dn[268][2]);
        let eq25_e336_d_n3: f64 = (p.p3 * s.dn[268][3]);
        let eq25_e336_d_n4: f64 = (p.p3 * s.dn[268][4]);
        let eq25_e336_d_n5: f64 = (p.p3 * s.dn[268][5]);
        let eq25_e336_d_n6: f64 = (p.p3 * s.dn[268][6]);
        let eq25_e336_d_n7: f64 = (p.p3 * s.dn[268][7]);
        let eq25_e336_d_n8: f64 = (p.p3 * s.dn[268][8]);
        let eq25_e336_d_n9: f64 = (p.p3 * s.dn[268][9]);
        let eq25_e336_d_n10: f64 = (p.p3 * s.dn[268][10]);
        let eq25_e336_d_n11: f64 = (p.p3 * s.dn[268][11]);
        let eq25_e336_d_n12: f64 = (p.p3 * s.dn[268][12]);
        let eq25_e338: f64 = (eq25_e336 * s.v[111]);
        let eq25_e338_d_n0: f64 = ((eq25_e336_d_n0 * s.v[111]) + (eq25_e336 * s.dn[111][0]));
        let eq25_e338_d_n1: f64 = ((eq25_e336_d_n1 * s.v[111]) + (eq25_e336 * s.dn[111][1]));
        let eq25_e338_d_n2: f64 = ((eq25_e336_d_n2 * s.v[111]) + (eq25_e336 * s.dn[111][2]));
        let eq25_e338_d_n3: f64 = ((eq25_e336_d_n3 * s.v[111]) + (eq25_e336 * s.dn[111][3]));
        let eq25_e338_d_n4: f64 = ((eq25_e336_d_n4 * s.v[111]) + (eq25_e336 * s.dn[111][4]));
        let eq25_e338_d_n5: f64 = ((eq25_e336_d_n5 * s.v[111]) + (eq25_e336 * s.dn[111][5]));
        let eq25_e338_d_n6: f64 = ((eq25_e336_d_n6 * s.v[111]) + (eq25_e336 * s.dn[111][6]));
        let eq25_e338_d_n7: f64 = ((eq25_e336_d_n7 * s.v[111]) + (eq25_e336 * s.dn[111][7]));
        let eq25_e338_d_n8: f64 = ((eq25_e336_d_n8 * s.v[111]) + (eq25_e336 * s.dn[111][8]));
        let eq25_e338_d_n9: f64 = ((eq25_e336_d_n9 * s.v[111]) + (eq25_e336 * s.dn[111][9]));
        let eq25_e338_d_n10: f64 = ((eq25_e336_d_n10 * s.v[111]) + (eq25_e336 * s.dn[111][10]));
        let eq25_e338_d_n11: f64 = ((eq25_e336_d_n11 * s.v[111]) + (eq25_e336 * s.dn[111][11]));
        let eq25_e338_d_n12: f64 = ((eq25_e336_d_n12 * s.v[111]) + (eq25_e336 * s.dn[111][12]));
        let eq25_e340: f64 = (eq25_e338 * p.p1);
        let eq25_e340_d_n0: f64 = (eq25_e338_d_n0 * p.p1);
        let eq25_e340_d_n1: f64 = (eq25_e338_d_n1 * p.p1);
        let eq25_e340_d_n2: f64 = (eq25_e338_d_n2 * p.p1);
        let eq25_e340_d_n3: f64 = (eq25_e338_d_n3 * p.p1);
        let eq25_e340_d_n4: f64 = (eq25_e338_d_n4 * p.p1);
        let eq25_e340_d_n5: f64 = (eq25_e338_d_n5 * p.p1);
        let eq25_e340_d_n6: f64 = (eq25_e338_d_n6 * p.p1);
        let eq25_e340_d_n7: f64 = (eq25_e338_d_n7 * p.p1);
        let eq25_e340_d_n8: f64 = (eq25_e338_d_n8 * p.p1);
        let eq25_e340_d_n9: f64 = (eq25_e338_d_n9 * p.p1);
        let eq25_e340_d_n10: f64 = (eq25_e338_d_n10 * p.p1);
        let eq25_e340_d_n11: f64 = (eq25_e338_d_n11 * p.p1);
        let eq25_e340_d_n12: f64 = (eq25_e338_d_n12 * p.p1);
        let eq25_value: f64 = eq25_e340;
        let eq25_node_derivatives: [f64; 13] = [eq25_e340_d_n0, eq25_e340_d_n1, eq25_e340_d_n2, eq25_e340_d_n3, eq25_e340_d_n4, eq25_e340_d_n5, eq25_e340_d_n6, eq25_e340_d_n7, eq25_e340_d_n8, eq25_e340_d_n9, eq25_e340_d_n10, eq25_e340_d_n11, eq25_e340_d_n12];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[10]),
            self.multiplicity * (eq25_value),
            &nodes,
            &eq25_node_derivatives,
            &branches,
            &eq25_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq26_e346: f64 = self.eval_ddt(8, eq26_e345);
        let eq26_e346_d_n0: f64 = self.ddt_jacobian(eq26_e345_d_n0);
        let eq26_e346_d_n1: f64 = self.ddt_jacobian(eq26_e345_d_n1);
        let eq26_e346_d_n2: f64 = self.ddt_jacobian(eq26_e345_d_n2);
        let eq26_e346_d_n3: f64 = self.ddt_jacobian(eq26_e345_d_n3);
        let eq26_e346_d_n4: f64 = self.ddt_jacobian(eq26_e345_d_n4);
        let eq26_e346_d_n5: f64 = self.ddt_jacobian(eq26_e345_d_n5);
        let eq26_e346_d_n6: f64 = self.ddt_jacobian(eq26_e345_d_n6);
        let eq26_e346_d_n7: f64 = self.ddt_jacobian(eq26_e345_d_n7);
        let eq26_e346_d_n8: f64 = self.ddt_jacobian(eq26_e345_d_n8);
        let eq26_e346_d_n9: f64 = self.ddt_jacobian(eq26_e345_d_n9);
        let eq26_e346_d_n10: f64 = self.ddt_jacobian(eq26_e345_d_n10);
        let eq26_e346_d_n11: f64 = self.ddt_jacobian(eq26_e345_d_n11);
        let eq26_e346_d_n12: f64 = self.ddt_jacobian(eq26_e345_d_n12);
        let eq26_e348: f64 = (eq26_e346 * p.p1);
        let eq26_e348_d_n0: f64 = (eq26_e346_d_n0 * p.p1);
        let eq26_e348_d_n1: f64 = (eq26_e346_d_n1 * p.p1);
        let eq26_e348_d_n2: f64 = (eq26_e346_d_n2 * p.p1);
        let eq26_e348_d_n3: f64 = (eq26_e346_d_n3 * p.p1);
        let eq26_e348_d_n4: f64 = (eq26_e346_d_n4 * p.p1);
        let eq26_e348_d_n5: f64 = (eq26_e346_d_n5 * p.p1);
        let eq26_e348_d_n6: f64 = (eq26_e346_d_n6 * p.p1);
        let eq26_e348_d_n7: f64 = (eq26_e346_d_n7 * p.p1);
        let eq26_e348_d_n8: f64 = (eq26_e346_d_n8 * p.p1);
        let eq26_e348_d_n9: f64 = (eq26_e346_d_n9 * p.p1);
        let eq26_e348_d_n10: f64 = (eq26_e346_d_n10 * p.p1);
        let eq26_e348_d_n11: f64 = (eq26_e346_d_n11 * p.p1);
        let eq26_e348_d_n12: f64 = (eq26_e346_d_n12 * p.p1);
        let eq26_value: f64 = eq26_e348;
        let eq26_node_derivatives: [f64; 13] = [eq26_e348_d_n0, eq26_e348_d_n1, eq26_e348_d_n2, eq26_e348_d_n3, eq26_e348_d_n4, eq26_e348_d_n5, eq26_e348_d_n6, eq26_e348_d_n7, eq26_e348_d_n8, eq26_e348_d_n9, eq26_e348_d_n10, eq26_e348_d_n11, eq26_e348_d_n12];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            self.multiplicity * (eq26_value),
            &nodes,
            &eq26_node_derivatives,
            &branches,
            &eq26_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_27_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq27_e353: f64 = (s.v[344] * s.v[255]);
        let eq27_e353_d_n0: f64 = (s.v[344] * s.dn[255][0]);
        let eq27_e353_d_n1: f64 = (s.v[344] * s.dn[255][1]);
        let eq27_e353_d_n2: f64 = (s.v[344] * s.dn[255][2]);
        let eq27_e353_d_n3: f64 = (s.v[344] * s.dn[255][3]);
        let eq27_e353_d_n4: f64 = (s.v[344] * s.dn[255][4]);
        let eq27_e353_d_n5: f64 = (s.v[344] * s.dn[255][5]);
        let eq27_e353_d_n6: f64 = (s.v[344] * s.dn[255][6]);
        let eq27_e353_d_n7: f64 = (s.v[344] * s.dn[255][7]);
        let eq27_e353_d_n8: f64 = (s.v[344] * s.dn[255][8]);
        let eq27_e353_d_n9: f64 = (s.v[344] * s.dn[255][9]);
        let eq27_e353_d_n10: f64 = (s.v[344] * s.dn[255][10]);
        let eq27_e353_d_n11: f64 = (s.v[344] * s.dn[255][11]);
        let eq27_e353_d_n12: f64 = (s.v[344] * s.dn[255][12]);
        let eq27_e354: f64 = (s.v[164] + eq27_e353);
        let eq27_e354_d_n0: f64 = (s.dn[164][0] + eq27_e353_d_n0);
        let eq27_e354_d_n1: f64 = (s.dn[164][1] + eq27_e353_d_n1);
        let eq27_e354_d_n2: f64 = (s.dn[164][2] + eq27_e353_d_n2);
        let eq27_e354_d_n3: f64 = (s.dn[164][3] + eq27_e353_d_n3);
        let eq27_e354_d_n4: f64 = (s.dn[164][4] + eq27_e353_d_n4);
        let eq27_e354_d_n5: f64 = (s.dn[164][5] + eq27_e353_d_n5);
        let eq27_e354_d_n6: f64 = (s.dn[164][6] + eq27_e353_d_n6);
        let eq27_e354_d_n7: f64 = (s.dn[164][7] + eq27_e353_d_n7);
        let eq27_e354_d_n8: f64 = (s.dn[164][8] + eq27_e353_d_n8);
        let eq27_e354_d_n9: f64 = (s.dn[164][9] + eq27_e353_d_n9);
        let eq27_e354_d_n10: f64 = (s.dn[164][10] + eq27_e353_d_n10);
        let eq27_e354_d_n11: f64 = (s.dn[164][11] + eq27_e353_d_n11);
        let eq27_e354_d_n12: f64 = (s.dn[164][12] + eq27_e353_d_n12);
        let eq27_e356: f64 = (eq27_e354 + s.v[167]);
        let eq27_e356_d_n0: f64 = (eq27_e354_d_n0 + s.dn[167][0]);
        let eq27_e356_d_n1: f64 = (eq27_e354_d_n1 + s.dn[167][1]);
        let eq27_e356_d_n2: f64 = (eq27_e354_d_n2 + s.dn[167][2]);
        let eq27_e356_d_n3: f64 = (eq27_e354_d_n3 + s.dn[167][3]);
        let eq27_e356_d_n4: f64 = (eq27_e354_d_n4 + s.dn[167][4]);
        let eq27_e356_d_n5: f64 = (eq27_e354_d_n5 + s.dn[167][5]);
        let eq27_e356_d_n6: f64 = (eq27_e354_d_n6 + s.dn[167][6]);
        let eq27_e356_d_n7: f64 = (eq27_e354_d_n7 + s.dn[167][7]);
        let eq27_e356_d_n8: f64 = (eq27_e354_d_n8 + s.dn[167][8]);
        let eq27_e356_d_n9: f64 = (eq27_e354_d_n9 + s.dn[167][9]);
        let eq27_e356_d_n10: f64 = (eq27_e354_d_n10 + s.dn[167][10]);
        let eq27_e356_d_n11: f64 = (eq27_e354_d_n11 + s.dn[167][11]);
        let eq27_e356_d_n12: f64 = (eq27_e354_d_n12 + s.dn[167][12]);
        let eq27_e357: f64 = (p.p3 * eq27_e356);
        let eq27_e357_d_n0: f64 = (p.p3 * eq27_e356_d_n0);
        let eq27_e357_d_n1: f64 = (p.p3 * eq27_e356_d_n1);
        let eq27_e357_d_n2: f64 = (p.p3 * eq27_e356_d_n2);
        let eq27_e357_d_n3: f64 = (p.p3 * eq27_e356_d_n3);
        let eq27_e357_d_n4: f64 = (p.p3 * eq27_e356_d_n4);
        let eq27_e357_d_n5: f64 = (p.p3 * eq27_e356_d_n5);
        let eq27_e357_d_n6: f64 = (p.p3 * eq27_e356_d_n6);
        let eq27_e357_d_n7: f64 = (p.p3 * eq27_e356_d_n7);
        let eq27_e357_d_n8: f64 = (p.p3 * eq27_e356_d_n8);
        let eq27_e357_d_n9: f64 = (p.p3 * eq27_e356_d_n9);
        let eq27_e357_d_n10: f64 = (p.p3 * eq27_e356_d_n10);
        let eq27_e357_d_n11: f64 = (p.p3 * eq27_e356_d_n11);
        let eq27_e357_d_n12: f64 = (p.p3 * eq27_e356_d_n12);
        let eq27_e359: f64 = (eq27_e357 * p.p1);
        let eq27_e359_d_n0: f64 = (eq27_e357_d_n0 * p.p1);
        let eq27_e359_d_n1: f64 = (eq27_e357_d_n1 * p.p1);
        let eq27_e359_d_n2: f64 = (eq27_e357_d_n2 * p.p1);
        let eq27_e359_d_n3: f64 = (eq27_e357_d_n3 * p.p1);
        let eq27_e359_d_n4: f64 = (eq27_e357_d_n4 * p.p1);
        let eq27_e359_d_n5: f64 = (eq27_e357_d_n5 * p.p1);
        let eq27_e359_d_n6: f64 = (eq27_e357_d_n6 * p.p1);
        let eq27_e359_d_n7: f64 = (eq27_e357_d_n7 * p.p1);
        let eq27_e359_d_n8: f64 = (eq27_e357_d_n8 * p.p1);
        let eq27_e359_d_n9: f64 = (eq27_e357_d_n9 * p.p1);
        let eq27_e359_d_n10: f64 = (eq27_e357_d_n10 * p.p1);
        let eq27_e359_d_n11: f64 = (eq27_e357_d_n11 * p.p1);
        let eq27_e359_d_n12: f64 = (eq27_e357_d_n12 * p.p1);
        let eq27_value: f64 = eq27_e359;
        let eq27_node_derivatives: [f64; 13] = [eq27_e359_d_n0, eq27_e359_d_n1, eq27_e359_d_n2, eq27_e359_d_n3, eq27_e359_d_n4, eq27_e359_d_n5, eq27_e359_d_n6, eq27_e359_d_n7, eq27_e359_d_n8, eq27_e359_d_n9, eq27_e359_d_n10, eq27_e359_d_n11, eq27_e359_d_n12];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            self.multiplicity * (eq27_value),
            &nodes,
            &eq27_node_derivatives,
            &branches,
            &eq27_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_28_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq28_e365: f64 = self.eval_ddt(9, eq28_e364);
        let eq28_e365_d_n0: f64 = self.ddt_jacobian(eq28_e364_d_n0);
        let eq28_e365_d_n1: f64 = self.ddt_jacobian(eq28_e364_d_n1);
        let eq28_e365_d_n2: f64 = self.ddt_jacobian(eq28_e364_d_n2);
        let eq28_e365_d_n3: f64 = self.ddt_jacobian(eq28_e364_d_n3);
        let eq28_e365_d_n4: f64 = self.ddt_jacobian(eq28_e364_d_n4);
        let eq28_e365_d_n5: f64 = self.ddt_jacobian(eq28_e364_d_n5);
        let eq28_e365_d_n6: f64 = self.ddt_jacobian(eq28_e364_d_n6);
        let eq28_e365_d_n7: f64 = self.ddt_jacobian(eq28_e364_d_n7);
        let eq28_e365_d_n8: f64 = self.ddt_jacobian(eq28_e364_d_n8);
        let eq28_e365_d_n9: f64 = self.ddt_jacobian(eq28_e364_d_n9);
        let eq28_e365_d_n10: f64 = self.ddt_jacobian(eq28_e364_d_n10);
        let eq28_e365_d_n11: f64 = self.ddt_jacobian(eq28_e364_d_n11);
        let eq28_e365_d_n12: f64 = self.ddt_jacobian(eq28_e364_d_n12);
        let eq28_e367: f64 = (eq28_e365 * p.p1);
        let eq28_e367_d_n0: f64 = (eq28_e365_d_n0 * p.p1);
        let eq28_e367_d_n1: f64 = (eq28_e365_d_n1 * p.p1);
        let eq28_e367_d_n2: f64 = (eq28_e365_d_n2 * p.p1);
        let eq28_e367_d_n3: f64 = (eq28_e365_d_n3 * p.p1);
        let eq28_e367_d_n4: f64 = (eq28_e365_d_n4 * p.p1);
        let eq28_e367_d_n5: f64 = (eq28_e365_d_n5 * p.p1);
        let eq28_e367_d_n6: f64 = (eq28_e365_d_n6 * p.p1);
        let eq28_e367_d_n7: f64 = (eq28_e365_d_n7 * p.p1);
        let eq28_e367_d_n8: f64 = (eq28_e365_d_n8 * p.p1);
        let eq28_e367_d_n9: f64 = (eq28_e365_d_n9 * p.p1);
        let eq28_e367_d_n10: f64 = (eq28_e365_d_n10 * p.p1);
        let eq28_e367_d_n11: f64 = (eq28_e365_d_n11 * p.p1);
        let eq28_e367_d_n12: f64 = (eq28_e365_d_n12 * p.p1);
        let eq28_value: f64 = eq28_e367;
        let eq28_node_derivatives: [f64; 13] = [eq28_e367_d_n0, eq28_e367_d_n1, eq28_e367_d_n2, eq28_e367_d_n3, eq28_e367_d_n4, eq28_e367_d_n5, eq28_e367_d_n6, eq28_e367_d_n7, eq28_e367_d_n8, eq28_e367_d_n9, eq28_e367_d_n10, eq28_e367_d_n11, eq28_e367_d_n12];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            self.multiplicity * (eq28_value),
            &nodes,
            &eq28_node_derivatives,
            &branches,
            &eq28_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_29_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq29_e377, eq29_e377_d_n0, eq29_e377_d_n1, eq29_e377_d_n2, eq29_e377_d_n3, eq29_e377_d_n4, eq29_e377_d_n5, eq29_e377_d_n6, eq29_e377_d_n7, eq29_e377_d_n8, eq29_e377_d_n9, eq29_e377_d_n10, eq29_e377_d_n11, eq29_e377_d_n12,) = {
    if (s.v[612] != 0.0) {
        let eq29_e371: f64 = (p.p3 * s.v[257]);
        let eq29_e371_d_n0: f64 = (p.p3 * s.dn[257][0]);
        let eq29_e371_d_n1: f64 = (p.p3 * s.dn[257][1]);
        let eq29_e371_d_n2: f64 = (p.p3 * s.dn[257][2]);
        let eq29_e371_d_n3: f64 = (p.p3 * s.dn[257][3]);
        let eq29_e371_d_n4: f64 = (p.p3 * s.dn[257][4]);
        let eq29_e371_d_n5: f64 = (p.p3 * s.dn[257][5]);
        let eq29_e371_d_n6: f64 = (p.p3 * s.dn[257][6]);
        let eq29_e371_d_n7: f64 = (p.p3 * s.dn[257][7]);
        let eq29_e371_d_n8: f64 = (p.p3 * s.dn[257][8]);
        let eq29_e371_d_n9: f64 = (p.p3 * s.dn[257][9]);
        let eq29_e371_d_n10: f64 = (p.p3 * s.dn[257][10]);
        let eq29_e371_d_n11: f64 = (p.p3 * s.dn[257][11]);
        let eq29_e371_d_n12: f64 = (p.p3 * s.dn[257][12]);
        let eq29_e373: f64 = (eq29_e371 * s.v[112]);
        let eq29_e373_d_n0: f64 = ((eq29_e371_d_n0 * s.v[112]) + (eq29_e371 * s.dn[112][0]));
        let eq29_e373_d_n1: f64 = ((eq29_e371_d_n1 * s.v[112]) + (eq29_e371 * s.dn[112][1]));
        let eq29_e373_d_n2: f64 = ((eq29_e371_d_n2 * s.v[112]) + (eq29_e371 * s.dn[112][2]));
        let eq29_e373_d_n3: f64 = ((eq29_e371_d_n3 * s.v[112]) + (eq29_e371 * s.dn[112][3]));
        let eq29_e373_d_n4: f64 = ((eq29_e371_d_n4 * s.v[112]) + (eq29_e371 * s.dn[112][4]));
        let eq29_e373_d_n5: f64 = ((eq29_e371_d_n5 * s.v[112]) + (eq29_e371 * s.dn[112][5]));
        let eq29_e373_d_n6: f64 = ((eq29_e371_d_n6 * s.v[112]) + (eq29_e371 * s.dn[112][6]));
        let eq29_e373_d_n7: f64 = ((eq29_e371_d_n7 * s.v[112]) + (eq29_e371 * s.dn[112][7]));
        let eq29_e373_d_n8: f64 = ((eq29_e371_d_n8 * s.v[112]) + (eq29_e371 * s.dn[112][8]));
        let eq29_e373_d_n9: f64 = ((eq29_e371_d_n9 * s.v[112]) + (eq29_e371 * s.dn[112][9]));
        let eq29_e373_d_n10: f64 = ((eq29_e371_d_n10 * s.v[112]) + (eq29_e371 * s.dn[112][10]));
        let eq29_e373_d_n11: f64 = ((eq29_e371_d_n11 * s.v[112]) + (eq29_e371 * s.dn[112][11]));
        let eq29_e373_d_n12: f64 = ((eq29_e371_d_n12 * s.v[112]) + (eq29_e371 * s.dn[112][12]));
        let eq29_e375: f64 = (eq29_e373 * p.p1);
        let eq29_e375_d_n0: f64 = (eq29_e373_d_n0 * p.p1);
        let eq29_e375_d_n1: f64 = (eq29_e373_d_n1 * p.p1);
        let eq29_e375_d_n2: f64 = (eq29_e373_d_n2 * p.p1);
        let eq29_e375_d_n3: f64 = (eq29_e373_d_n3 * p.p1);
        let eq29_e375_d_n4: f64 = (eq29_e373_d_n4 * p.p1);
        let eq29_e375_d_n5: f64 = (eq29_e373_d_n5 * p.p1);
        let eq29_e375_d_n6: f64 = (eq29_e373_d_n6 * p.p1);
        let eq29_e375_d_n7: f64 = (eq29_e373_d_n7 * p.p1);
        let eq29_e375_d_n8: f64 = (eq29_e373_d_n8 * p.p1);
        let eq29_e375_d_n9: f64 = (eq29_e373_d_n9 * p.p1);
        let eq29_e375_d_n10: f64 = (eq29_e373_d_n10 * p.p1);
        let eq29_e375_d_n11: f64 = (eq29_e373_d_n11 * p.p1);
        let eq29_e375_d_n12: f64 = (eq29_e373_d_n12 * p.p1);
        (eq29_e375, eq29_e375_d_n0, eq29_e375_d_n1, eq29_e375_d_n2, eq29_e375_d_n3, eq29_e375_d_n4, eq29_e375_d_n5, eq29_e375_d_n6, eq29_e375_d_n7, eq29_e375_d_n8, eq29_e375_d_n9, eq29_e375_d_n10, eq29_e375_d_n11, eq29_e375_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e377;
        let eq29_node_derivatives: [f64; 13] = [eq29_e377_d_n0, eq29_e377_d_n1, eq29_e377_d_n2, eq29_e377_d_n3, eq29_e377_d_n4, eq29_e377_d_n5, eq29_e377_d_n6, eq29_e377_d_n7, eq29_e377_d_n8, eq29_e377_d_n9, eq29_e377_d_n10, eq29_e377_d_n11, eq29_e377_d_n12];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            self.multiplicity * (eq29_value),
            &nodes,
            &eq29_node_derivatives,
            &branches,
            &eq29_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_30_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq30_e382,) = {
    if (!(s.v[612] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e382;
        stamper.stamp_potential(
            branches[0],
            eq30_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_31_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq31_e392, eq31_e392_d_n0, eq31_e392_d_n1, eq31_e392_d_n2, eq31_e392_d_n3, eq31_e392_d_n4, eq31_e392_d_n5, eq31_e392_d_n6, eq31_e392_d_n7, eq31_e392_d_n8, eq31_e392_d_n9, eq31_e392_d_n10, eq31_e392_d_n11, eq31_e392_d_n12,) = {
    if (s.v[613] != 0.0) {
        let eq31_e386: f64 = (p.p3 * s.v[258]);
        let eq31_e386_d_n0: f64 = (p.p3 * s.dn[258][0]);
        let eq31_e386_d_n1: f64 = (p.p3 * s.dn[258][1]);
        let eq31_e386_d_n2: f64 = (p.p3 * s.dn[258][2]);
        let eq31_e386_d_n3: f64 = (p.p3 * s.dn[258][3]);
        let eq31_e386_d_n4: f64 = (p.p3 * s.dn[258][4]);
        let eq31_e386_d_n5: f64 = (p.p3 * s.dn[258][5]);
        let eq31_e386_d_n6: f64 = (p.p3 * s.dn[258][6]);
        let eq31_e386_d_n7: f64 = (p.p3 * s.dn[258][7]);
        let eq31_e386_d_n8: f64 = (p.p3 * s.dn[258][8]);
        let eq31_e386_d_n9: f64 = (p.p3 * s.dn[258][9]);
        let eq31_e386_d_n10: f64 = (p.p3 * s.dn[258][10]);
        let eq31_e386_d_n11: f64 = (p.p3 * s.dn[258][11]);
        let eq31_e386_d_n12: f64 = (p.p3 * s.dn[258][12]);
        let eq31_e388: f64 = (eq31_e386 * s.v[113]);
        let eq31_e388_d_n0: f64 = ((eq31_e386_d_n0 * s.v[113]) + (eq31_e386 * s.dn[113][0]));
        let eq31_e388_d_n1: f64 = ((eq31_e386_d_n1 * s.v[113]) + (eq31_e386 * s.dn[113][1]));
        let eq31_e388_d_n2: f64 = ((eq31_e386_d_n2 * s.v[113]) + (eq31_e386 * s.dn[113][2]));
        let eq31_e388_d_n3: f64 = ((eq31_e386_d_n3 * s.v[113]) + (eq31_e386 * s.dn[113][3]));
        let eq31_e388_d_n4: f64 = ((eq31_e386_d_n4 * s.v[113]) + (eq31_e386 * s.dn[113][4]));
        let eq31_e388_d_n5: f64 = ((eq31_e386_d_n5 * s.v[113]) + (eq31_e386 * s.dn[113][5]));
        let eq31_e388_d_n6: f64 = ((eq31_e386_d_n6 * s.v[113]) + (eq31_e386 * s.dn[113][6]));
        let eq31_e388_d_n7: f64 = ((eq31_e386_d_n7 * s.v[113]) + (eq31_e386 * s.dn[113][7]));
        let eq31_e388_d_n8: f64 = ((eq31_e386_d_n8 * s.v[113]) + (eq31_e386 * s.dn[113][8]));
        let eq31_e388_d_n9: f64 = ((eq31_e386_d_n9 * s.v[113]) + (eq31_e386 * s.dn[113][9]));
        let eq31_e388_d_n10: f64 = ((eq31_e386_d_n10 * s.v[113]) + (eq31_e386 * s.dn[113][10]));
        let eq31_e388_d_n11: f64 = ((eq31_e386_d_n11 * s.v[113]) + (eq31_e386 * s.dn[113][11]));
        let eq31_e388_d_n12: f64 = ((eq31_e386_d_n12 * s.v[113]) + (eq31_e386 * s.dn[113][12]));
        let eq31_e390: f64 = (eq31_e388 * p.p1);
        let eq31_e390_d_n0: f64 = (eq31_e388_d_n0 * p.p1);
        let eq31_e390_d_n1: f64 = (eq31_e388_d_n1 * p.p1);
        let eq31_e390_d_n2: f64 = (eq31_e388_d_n2 * p.p1);
        let eq31_e390_d_n3: f64 = (eq31_e388_d_n3 * p.p1);
        let eq31_e390_d_n4: f64 = (eq31_e388_d_n4 * p.p1);
        let eq31_e390_d_n5: f64 = (eq31_e388_d_n5 * p.p1);
        let eq31_e390_d_n6: f64 = (eq31_e388_d_n6 * p.p1);
        let eq31_e390_d_n7: f64 = (eq31_e388_d_n7 * p.p1);
        let eq31_e390_d_n8: f64 = (eq31_e388_d_n8 * p.p1);
        let eq31_e390_d_n9: f64 = (eq31_e388_d_n9 * p.p1);
        let eq31_e390_d_n10: f64 = (eq31_e388_d_n10 * p.p1);
        let eq31_e390_d_n11: f64 = (eq31_e388_d_n11 * p.p1);
        let eq31_e390_d_n12: f64 = (eq31_e388_d_n12 * p.p1);
        (eq31_e390, eq31_e390_d_n0, eq31_e390_d_n1, eq31_e390_d_n2, eq31_e390_d_n3, eq31_e390_d_n4, eq31_e390_d_n5, eq31_e390_d_n6, eq31_e390_d_n7, eq31_e390_d_n8, eq31_e390_d_n9, eq31_e390_d_n10, eq31_e390_d_n11, eq31_e390_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e392;
        let eq31_node_derivatives: [f64; 13] = [eq31_e392_d_n0, eq31_e392_d_n1, eq31_e392_d_n2, eq31_e392_d_n3, eq31_e392_d_n4, eq31_e392_d_n5, eq31_e392_d_n6, eq31_e392_d_n7, eq31_e392_d_n8, eq31_e392_d_n9, eq31_e392_d_n10, eq31_e392_d_n11, eq31_e392_d_n12];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            self.multiplicity * (eq31_value),
            &nodes,
            &eq31_node_derivatives,
            &branches,
            &eq31_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_32_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq32_e397,) = {
    if (!(s.v[613] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e397;
        stamper.stamp_potential(
            branches[1],
            eq32_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq33_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq33_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq34_value: f64 = (nv12 - 0.0);
        stamper.stamp_current(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq34_value),
            &[
                GeneratedDerivative::node(nodes[12], self.multiplicity * 1.0),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq35_e406: f64 = self.eval_ddt(10, (nv12 - 0.0));
        let eq35_e406_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq35_e406_d_n12: f64 = self.ddt_jacobian(1.0);
        let eq35_e407: f64 = (s.v[336] * eq35_e406);
        let eq35_e407_d_n0: f64 = ((s.dn[336][0] * eq35_e406) + (s.v[336] * eq35_e406_d_n0));
        let eq35_e407_d_n1: f64 = ((s.dn[336][1] * eq35_e406) + (s.v[336] * eq35_e406_d_n1));
        let eq35_e407_d_n2: f64 = ((s.dn[336][2] * eq35_e406) + (s.v[336] * eq35_e406_d_n2));
        let eq35_e407_d_n3: f64 = ((s.dn[336][3] * eq35_e406) + (s.v[336] * eq35_e406_d_n3));
        let eq35_e407_d_n4: f64 = ((s.dn[336][4] * eq35_e406) + (s.v[336] * eq35_e406_d_n4));
        let eq35_e407_d_n5: f64 = ((s.dn[336][5] * eq35_e406) + (s.v[336] * eq35_e406_d_n5));
        let eq35_e407_d_n6: f64 = ((s.dn[336][6] * eq35_e406) + (s.v[336] * eq35_e406_d_n6));
        let eq35_e407_d_n7: f64 = ((s.dn[336][7] * eq35_e406) + (s.v[336] * eq35_e406_d_n7));
        let eq35_e407_d_n8: f64 = ((s.dn[336][8] * eq35_e406) + (s.v[336] * eq35_e406_d_n8));
        let eq35_e407_d_n9: f64 = ((s.dn[336][9] * eq35_e406) + (s.v[336] * eq35_e406_d_n9));
        let eq35_e407_d_n10: f64 = ((s.dn[336][10] * eq35_e406) + (s.v[336] * eq35_e406_d_n10));
        let eq35_e407_d_n11: f64 = ((s.dn[336][11] * eq35_e406) + (s.v[336] * eq35_e406_d_n11));
        let eq35_e407_d_n12: f64 = ((s.dn[336][12] * eq35_e406) + (s.v[336] * eq35_e406_d_n12));
        let eq35_value: f64 = eq35_e407;
        let eq35_node_derivatives: [f64; 13] = [eq35_e407_d_n0, eq35_e407_d_n1, eq35_e407_d_n2, eq35_e407_d_n3, eq35_e407_d_n4, eq35_e407_d_n5, eq35_e407_d_n6, eq35_e407_d_n7, eq35_e407_d_n8, eq35_e407_d_n9, eq35_e407_d_n10, eq35_e407_d_n11, eq35_e407_d_n12];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            self.multiplicity * (eq35_value),
            &nodes,
            &eq35_node_derivatives,
            &branches,
            &eq35_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq36_e410: f64 = (s.v[334] * (nv12 - 0.0));
        let eq36_e410_d_n0: f64 = (s.dn[334][0] * (nv12 - 0.0));
        let eq36_e410_d_n1: f64 = (s.dn[334][1] * (nv12 - 0.0));
        let eq36_e410_d_n2: f64 = (s.dn[334][2] * (nv12 - 0.0));
        let eq36_e410_d_n3: f64 = (s.dn[334][3] * (nv12 - 0.0));
        let eq36_e410_d_n4: f64 = (s.dn[334][4] * (nv12 - 0.0));
        let eq36_e410_d_n5: f64 = (s.dn[334][5] * (nv12 - 0.0));
        let eq36_e410_d_n6: f64 = (s.dn[334][6] * (nv12 - 0.0));
        let eq36_e410_d_n7: f64 = (s.dn[334][7] * (nv12 - 0.0));
        let eq36_e410_d_n8: f64 = (s.dn[334][8] * (nv12 - 0.0));
        let eq36_e410_d_n9: f64 = (s.dn[334][9] * (nv12 - 0.0));
        let eq36_e410_d_n10: f64 = (s.dn[334][10] * (nv12 - 0.0));
        let eq36_e410_d_n11: f64 = (s.dn[334][11] * (nv12 - 0.0));
        let eq36_e410_d_n12: f64 = ((s.dn[334][12] * (nv12 - 0.0)) + s.v[334]);
        let eq36_value: f64 = eq36_e410;
        let eq36_node_derivatives: [f64; 13] = [eq36_e410_d_n0, eq36_e410_d_n1, eq36_e410_d_n2, eq36_e410_d_n3, eq36_e410_d_n4, eq36_e410_d_n5, eq36_e410_d_n6, eq36_e410_d_n7, eq36_e410_d_n8, eq36_e410_d_n9, eq36_e410_d_n10, eq36_e410_d_n11, eq36_e410_d_n12];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq36_value),
            &nodes,
            &eq36_node_derivatives,
            &branches,
            &eq36_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq37_value: f64 = (nv12 - 0.0);
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[5]),
            self.multiplicity * (eq37_value),
            &[
                GeneratedDerivative::node(nodes[12], self.multiplicity * 1.0),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_38_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq38_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq38_value),
            &[
            ],
        );
    }
}
