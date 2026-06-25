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
        let eq23_e332: f64 = (s.v[230] + s.v[242]);
        let eq23_e332_d_n0: f64 = (s.dn[230][0] + s.dn[242][0]);
        let eq23_e332_d_n1: f64 = (s.dn[230][1] + s.dn[242][1]);
        let eq23_e332_d_n2: f64 = (s.dn[230][2] + s.dn[242][2]);
        let eq23_e332_d_n3: f64 = (s.dn[230][3] + s.dn[242][3]);
        let eq23_e332_d_n4: f64 = (s.dn[230][4] + s.dn[242][4]);
        let eq23_e332_d_n5: f64 = (s.dn[230][5] + s.dn[242][5]);
        let eq23_e332_d_n6: f64 = (s.dn[230][6] + s.dn[242][6]);
        let eq23_e332_d_n7: f64 = (s.dn[230][7] + s.dn[242][7]);
        let eq23_e332_d_n8: f64 = (s.dn[230][8] + s.dn[242][8]);
        let eq23_e332_d_n9: f64 = (s.dn[230][9] + s.dn[242][9]);
        let eq23_e332_d_n10: f64 = (s.dn[230][10] + s.dn[242][10]);
        let eq23_e332_d_n11: f64 = (s.dn[230][11] + s.dn[242][11]);
        let eq23_e333: f64 = (p.p3 * eq23_e332);
        let eq23_e333_d_n0: f64 = (p.p3 * eq23_e332_d_n0);
        let eq23_e333_d_n1: f64 = (p.p3 * eq23_e332_d_n1);
        let eq23_e333_d_n2: f64 = (p.p3 * eq23_e332_d_n2);
        let eq23_e333_d_n3: f64 = (p.p3 * eq23_e332_d_n3);
        let eq23_e333_d_n4: f64 = (p.p3 * eq23_e332_d_n4);
        let eq23_e333_d_n5: f64 = (p.p3 * eq23_e332_d_n5);
        let eq23_e333_d_n6: f64 = (p.p3 * eq23_e332_d_n6);
        let eq23_e333_d_n7: f64 = (p.p3 * eq23_e332_d_n7);
        let eq23_e333_d_n8: f64 = (p.p3 * eq23_e332_d_n8);
        let eq23_e333_d_n9: f64 = (p.p3 * eq23_e332_d_n9);
        let eq23_e333_d_n10: f64 = (p.p3 * eq23_e332_d_n10);
        let eq23_e333_d_n11: f64 = (p.p3 * eq23_e332_d_n11);
        let eq23_e334: f64 = self.eval_ddt(7, eq23_e333);
        let eq23_e334_d_n0: f64 = self.ddt_jacobian(eq23_e333_d_n0);
        let eq23_e334_d_n1: f64 = self.ddt_jacobian(eq23_e333_d_n1);
        let eq23_e334_d_n2: f64 = self.ddt_jacobian(eq23_e333_d_n2);
        let eq23_e334_d_n3: f64 = self.ddt_jacobian(eq23_e333_d_n3);
        let eq23_e334_d_n4: f64 = self.ddt_jacobian(eq23_e333_d_n4);
        let eq23_e334_d_n5: f64 = self.ddt_jacobian(eq23_e333_d_n5);
        let eq23_e334_d_n6: f64 = self.ddt_jacobian(eq23_e333_d_n6);
        let eq23_e334_d_n7: f64 = self.ddt_jacobian(eq23_e333_d_n7);
        let eq23_e334_d_n8: f64 = self.ddt_jacobian(eq23_e333_d_n8);
        let eq23_e334_d_n9: f64 = self.ddt_jacobian(eq23_e333_d_n9);
        let eq23_e334_d_n10: f64 = self.ddt_jacobian(eq23_e333_d_n10);
        let eq23_e334_d_n11: f64 = self.ddt_jacobian(eq23_e333_d_n11);
        let eq23_e336: f64 = (eq23_e334 * p.p1);
        let eq23_e336_d_n0: f64 = (eq23_e334_d_n0 * p.p1);
        let eq23_e336_d_n1: f64 = (eq23_e334_d_n1 * p.p1);
        let eq23_e336_d_n2: f64 = (eq23_e334_d_n2 * p.p1);
        let eq23_e336_d_n3: f64 = (eq23_e334_d_n3 * p.p1);
        let eq23_e336_d_n4: f64 = (eq23_e334_d_n4 * p.p1);
        let eq23_e336_d_n5: f64 = (eq23_e334_d_n5 * p.p1);
        let eq23_e336_d_n6: f64 = (eq23_e334_d_n6 * p.p1);
        let eq23_e336_d_n7: f64 = (eq23_e334_d_n7 * p.p1);
        let eq23_e336_d_n8: f64 = (eq23_e334_d_n8 * p.p1);
        let eq23_e336_d_n9: f64 = (eq23_e334_d_n9 * p.p1);
        let eq23_e336_d_n10: f64 = (eq23_e334_d_n10 * p.p1);
        let eq23_e336_d_n11: f64 = (eq23_e334_d_n11 * p.p1);
        let eq23_value: f64 = eq23_e336;
        let eq23_node_derivatives: [f64; 12] = [eq23_e336_d_n0, eq23_e336_d_n1, eq23_e336_d_n2, eq23_e336_d_n3, eq23_e336_d_n4, eq23_e336_d_n5, eq23_e336_d_n6, eq23_e336_d_n7, eq23_e336_d_n8, eq23_e336_d_n9, eq23_e336_d_n10, eq23_e336_d_n11];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[9]),
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
        let eq24_e341: f64 = (s.v[338] * s.v[249]);
        let eq24_e341_d_n0: f64 = (s.v[338] * s.dn[249][0]);
        let eq24_e341_d_n1: f64 = (s.v[338] * s.dn[249][1]);
        let eq24_e341_d_n2: f64 = (s.v[338] * s.dn[249][2]);
        let eq24_e341_d_n3: f64 = (s.v[338] * s.dn[249][3]);
        let eq24_e341_d_n4: f64 = (s.v[338] * s.dn[249][4]);
        let eq24_e341_d_n5: f64 = (s.v[338] * s.dn[249][5]);
        let eq24_e341_d_n6: f64 = (s.v[338] * s.dn[249][6]);
        let eq24_e341_d_n7: f64 = (s.v[338] * s.dn[249][7]);
        let eq24_e341_d_n8: f64 = (s.v[338] * s.dn[249][8]);
        let eq24_e341_d_n9: f64 = (s.v[338] * s.dn[249][9]);
        let eq24_e341_d_n10: f64 = (s.v[338] * s.dn[249][10]);
        let eq24_e341_d_n11: f64 = (s.v[338] * s.dn[249][11]);
        let eq24_e342: f64 = (s.v[161] + eq24_e341);
        let eq24_e342_d_n0: f64 = (s.dn[161][0] + eq24_e341_d_n0);
        let eq24_e342_d_n1: f64 = (s.dn[161][1] + eq24_e341_d_n1);
        let eq24_e342_d_n2: f64 = (s.dn[161][2] + eq24_e341_d_n2);
        let eq24_e342_d_n3: f64 = (s.dn[161][3] + eq24_e341_d_n3);
        let eq24_e342_d_n4: f64 = (s.dn[161][4] + eq24_e341_d_n4);
        let eq24_e342_d_n5: f64 = (s.dn[161][5] + eq24_e341_d_n5);
        let eq24_e342_d_n6: f64 = (s.dn[161][6] + eq24_e341_d_n6);
        let eq24_e342_d_n7: f64 = (s.dn[161][7] + eq24_e341_d_n7);
        let eq24_e342_d_n8: f64 = (s.dn[161][8] + eq24_e341_d_n8);
        let eq24_e342_d_n9: f64 = (s.dn[161][9] + eq24_e341_d_n9);
        let eq24_e342_d_n10: f64 = (s.dn[161][10] + eq24_e341_d_n10);
        let eq24_e342_d_n11: f64 = (s.dn[161][11] + eq24_e341_d_n11);
        let eq24_e344: f64 = (eq24_e342 + s.v[164]);
        let eq24_e344_d_n0: f64 = (eq24_e342_d_n0 + s.dn[164][0]);
        let eq24_e344_d_n1: f64 = (eq24_e342_d_n1 + s.dn[164][1]);
        let eq24_e344_d_n2: f64 = (eq24_e342_d_n2 + s.dn[164][2]);
        let eq24_e344_d_n3: f64 = (eq24_e342_d_n3 + s.dn[164][3]);
        let eq24_e344_d_n4: f64 = (eq24_e342_d_n4 + s.dn[164][4]);
        let eq24_e344_d_n5: f64 = (eq24_e342_d_n5 + s.dn[164][5]);
        let eq24_e344_d_n6: f64 = (eq24_e342_d_n6 + s.dn[164][6]);
        let eq24_e344_d_n7: f64 = (eq24_e342_d_n7 + s.dn[164][7]);
        let eq24_e344_d_n8: f64 = (eq24_e342_d_n8 + s.dn[164][8]);
        let eq24_e344_d_n9: f64 = (eq24_e342_d_n9 + s.dn[164][9]);
        let eq24_e344_d_n10: f64 = (eq24_e342_d_n10 + s.dn[164][10]);
        let eq24_e344_d_n11: f64 = (eq24_e342_d_n11 + s.dn[164][11]);
        let eq24_e345: f64 = (p.p3 * eq24_e344);
        let eq24_e345_d_n0: f64 = (p.p3 * eq24_e344_d_n0);
        let eq24_e345_d_n1: f64 = (p.p3 * eq24_e344_d_n1);
        let eq24_e345_d_n2: f64 = (p.p3 * eq24_e344_d_n2);
        let eq24_e345_d_n3: f64 = (p.p3 * eq24_e344_d_n3);
        let eq24_e345_d_n4: f64 = (p.p3 * eq24_e344_d_n4);
        let eq24_e345_d_n5: f64 = (p.p3 * eq24_e344_d_n5);
        let eq24_e345_d_n6: f64 = (p.p3 * eq24_e344_d_n6);
        let eq24_e345_d_n7: f64 = (p.p3 * eq24_e344_d_n7);
        let eq24_e345_d_n8: f64 = (p.p3 * eq24_e344_d_n8);
        let eq24_e345_d_n9: f64 = (p.p3 * eq24_e344_d_n9);
        let eq24_e345_d_n10: f64 = (p.p3 * eq24_e344_d_n10);
        let eq24_e345_d_n11: f64 = (p.p3 * eq24_e344_d_n11);
        let eq24_e347: f64 = (eq24_e345 * p.p1);
        let eq24_e347_d_n0: f64 = (eq24_e345_d_n0 * p.p1);
        let eq24_e347_d_n1: f64 = (eq24_e345_d_n1 * p.p1);
        let eq24_e347_d_n2: f64 = (eq24_e345_d_n2 * p.p1);
        let eq24_e347_d_n3: f64 = (eq24_e345_d_n3 * p.p1);
        let eq24_e347_d_n4: f64 = (eq24_e345_d_n4 * p.p1);
        let eq24_e347_d_n5: f64 = (eq24_e345_d_n5 * p.p1);
        let eq24_e347_d_n6: f64 = (eq24_e345_d_n6 * p.p1);
        let eq24_e347_d_n7: f64 = (eq24_e345_d_n7 * p.p1);
        let eq24_e347_d_n8: f64 = (eq24_e345_d_n8 * p.p1);
        let eq24_e347_d_n9: f64 = (eq24_e345_d_n9 * p.p1);
        let eq24_e347_d_n10: f64 = (eq24_e345_d_n10 * p.p1);
        let eq24_e347_d_n11: f64 = (eq24_e345_d_n11 * p.p1);
        let eq24_value: f64 = eq24_e347;
        let eq24_node_derivatives: [f64; 12] = [eq24_e347_d_n0, eq24_e347_d_n1, eq24_e347_d_n2, eq24_e347_d_n3, eq24_e347_d_n4, eq24_e347_d_n5, eq24_e347_d_n6, eq24_e347_d_n7, eq24_e347_d_n8, eq24_e347_d_n9, eq24_e347_d_n10, eq24_e347_d_n11];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let eq25_e351: f64 = (s.v[227] + s.v[243]);
        let eq25_e351_d_n0: f64 = (s.dn[227][0] + s.dn[243][0]);
        let eq25_e351_d_n1: f64 = (s.dn[227][1] + s.dn[243][1]);
        let eq25_e351_d_n2: f64 = (s.dn[227][2] + s.dn[243][2]);
        let eq25_e351_d_n3: f64 = (s.dn[227][3] + s.dn[243][3]);
        let eq25_e351_d_n4: f64 = (s.dn[227][4] + s.dn[243][4]);
        let eq25_e351_d_n5: f64 = (s.dn[227][5] + s.dn[243][5]);
        let eq25_e351_d_n6: f64 = (s.dn[227][6] + s.dn[243][6]);
        let eq25_e351_d_n7: f64 = (s.dn[227][7] + s.dn[243][7]);
        let eq25_e351_d_n8: f64 = (s.dn[227][8] + s.dn[243][8]);
        let eq25_e351_d_n9: f64 = (s.dn[227][9] + s.dn[243][9]);
        let eq25_e351_d_n10: f64 = (s.dn[227][10] + s.dn[243][10]);
        let eq25_e351_d_n11: f64 = (s.dn[227][11] + s.dn[243][11]);
        let eq25_e352: f64 = (p.p3 * eq25_e351);
        let eq25_e352_d_n0: f64 = (p.p3 * eq25_e351_d_n0);
        let eq25_e352_d_n1: f64 = (p.p3 * eq25_e351_d_n1);
        let eq25_e352_d_n2: f64 = (p.p3 * eq25_e351_d_n2);
        let eq25_e352_d_n3: f64 = (p.p3 * eq25_e351_d_n3);
        let eq25_e352_d_n4: f64 = (p.p3 * eq25_e351_d_n4);
        let eq25_e352_d_n5: f64 = (p.p3 * eq25_e351_d_n5);
        let eq25_e352_d_n6: f64 = (p.p3 * eq25_e351_d_n6);
        let eq25_e352_d_n7: f64 = (p.p3 * eq25_e351_d_n7);
        let eq25_e352_d_n8: f64 = (p.p3 * eq25_e351_d_n8);
        let eq25_e352_d_n9: f64 = (p.p3 * eq25_e351_d_n9);
        let eq25_e352_d_n10: f64 = (p.p3 * eq25_e351_d_n10);
        let eq25_e352_d_n11: f64 = (p.p3 * eq25_e351_d_n11);
        let eq25_e353: f64 = self.eval_ddt(8, eq25_e352);
        let eq25_e353_d_n0: f64 = self.ddt_jacobian(eq25_e352_d_n0);
        let eq25_e353_d_n1: f64 = self.ddt_jacobian(eq25_e352_d_n1);
        let eq25_e353_d_n2: f64 = self.ddt_jacobian(eq25_e352_d_n2);
        let eq25_e353_d_n3: f64 = self.ddt_jacobian(eq25_e352_d_n3);
        let eq25_e353_d_n4: f64 = self.ddt_jacobian(eq25_e352_d_n4);
        let eq25_e353_d_n5: f64 = self.ddt_jacobian(eq25_e352_d_n5);
        let eq25_e353_d_n6: f64 = self.ddt_jacobian(eq25_e352_d_n6);
        let eq25_e353_d_n7: f64 = self.ddt_jacobian(eq25_e352_d_n7);
        let eq25_e353_d_n8: f64 = self.ddt_jacobian(eq25_e352_d_n8);
        let eq25_e353_d_n9: f64 = self.ddt_jacobian(eq25_e352_d_n9);
        let eq25_e353_d_n10: f64 = self.ddt_jacobian(eq25_e352_d_n10);
        let eq25_e353_d_n11: f64 = self.ddt_jacobian(eq25_e352_d_n11);
        let eq25_e355: f64 = (eq25_e353 * p.p1);
        let eq25_e355_d_n0: f64 = (eq25_e353_d_n0 * p.p1);
        let eq25_e355_d_n1: f64 = (eq25_e353_d_n1 * p.p1);
        let eq25_e355_d_n2: f64 = (eq25_e353_d_n2 * p.p1);
        let eq25_e355_d_n3: f64 = (eq25_e353_d_n3 * p.p1);
        let eq25_e355_d_n4: f64 = (eq25_e353_d_n4 * p.p1);
        let eq25_e355_d_n5: f64 = (eq25_e353_d_n5 * p.p1);
        let eq25_e355_d_n6: f64 = (eq25_e353_d_n6 * p.p1);
        let eq25_e355_d_n7: f64 = (eq25_e353_d_n7 * p.p1);
        let eq25_e355_d_n8: f64 = (eq25_e353_d_n8 * p.p1);
        let eq25_e355_d_n9: f64 = (eq25_e353_d_n9 * p.p1);
        let eq25_e355_d_n10: f64 = (eq25_e353_d_n10 * p.p1);
        let eq25_e355_d_n11: f64 = (eq25_e353_d_n11 * p.p1);
        let eq25_value: f64 = eq25_e355;
        let eq25_node_derivatives: [f64; 12] = [eq25_e355_d_n0, eq25_e355_d_n1, eq25_e355_d_n2, eq25_e355_d_n3, eq25_e355_d_n4, eq25_e355_d_n5, eq25_e355_d_n6, eq25_e355_d_n7, eq25_e355_d_n8, eq25_e355_d_n9, eq25_e355_d_n10, eq25_e355_d_n11];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let (eq26_e365, eq26_e365_d_n0, eq26_e365_d_n1, eq26_e365_d_n2, eq26_e365_d_n3, eq26_e365_d_n4, eq26_e365_d_n5, eq26_e365_d_n6, eq26_e365_d_n7, eq26_e365_d_n8, eq26_e365_d_n9, eq26_e365_d_n10, eq26_e365_d_n11,) = {
    if (s.v[598] != 0.0) {
        let eq26_e359: f64 = (p.p3 * s.v[251]);
        let eq26_e359_d_n0: f64 = (p.p3 * s.dn[251][0]);
        let eq26_e359_d_n1: f64 = (p.p3 * s.dn[251][1]);
        let eq26_e359_d_n2: f64 = (p.p3 * s.dn[251][2]);
        let eq26_e359_d_n3: f64 = (p.p3 * s.dn[251][3]);
        let eq26_e359_d_n4: f64 = (p.p3 * s.dn[251][4]);
        let eq26_e359_d_n5: f64 = (p.p3 * s.dn[251][5]);
        let eq26_e359_d_n6: f64 = (p.p3 * s.dn[251][6]);
        let eq26_e359_d_n7: f64 = (p.p3 * s.dn[251][7]);
        let eq26_e359_d_n8: f64 = (p.p3 * s.dn[251][8]);
        let eq26_e359_d_n9: f64 = (p.p3 * s.dn[251][9]);
        let eq26_e359_d_n10: f64 = (p.p3 * s.dn[251][10]);
        let eq26_e359_d_n11: f64 = (p.p3 * s.dn[251][11]);
        let eq26_e361: f64 = (eq26_e359 * s.v[109]);
        let eq26_e361_d_n0: f64 = ((eq26_e359_d_n0 * s.v[109]) + (eq26_e359 * s.dn[109][0]));
        let eq26_e361_d_n1: f64 = ((eq26_e359_d_n1 * s.v[109]) + (eq26_e359 * s.dn[109][1]));
        let eq26_e361_d_n2: f64 = ((eq26_e359_d_n2 * s.v[109]) + (eq26_e359 * s.dn[109][2]));
        let eq26_e361_d_n3: f64 = ((eq26_e359_d_n3 * s.v[109]) + (eq26_e359 * s.dn[109][3]));
        let eq26_e361_d_n4: f64 = ((eq26_e359_d_n4 * s.v[109]) + (eq26_e359 * s.dn[109][4]));
        let eq26_e361_d_n5: f64 = ((eq26_e359_d_n5 * s.v[109]) + (eq26_e359 * s.dn[109][5]));
        let eq26_e361_d_n6: f64 = ((eq26_e359_d_n6 * s.v[109]) + (eq26_e359 * s.dn[109][6]));
        let eq26_e361_d_n7: f64 = ((eq26_e359_d_n7 * s.v[109]) + (eq26_e359 * s.dn[109][7]));
        let eq26_e361_d_n8: f64 = ((eq26_e359_d_n8 * s.v[109]) + (eq26_e359 * s.dn[109][8]));
        let eq26_e361_d_n9: f64 = ((eq26_e359_d_n9 * s.v[109]) + (eq26_e359 * s.dn[109][9]));
        let eq26_e361_d_n10: f64 = ((eq26_e359_d_n10 * s.v[109]) + (eq26_e359 * s.dn[109][10]));
        let eq26_e361_d_n11: f64 = ((eq26_e359_d_n11 * s.v[109]) + (eq26_e359 * s.dn[109][11]));
        let eq26_e363: f64 = (eq26_e361 * p.p1);
        let eq26_e363_d_n0: f64 = (eq26_e361_d_n0 * p.p1);
        let eq26_e363_d_n1: f64 = (eq26_e361_d_n1 * p.p1);
        let eq26_e363_d_n2: f64 = (eq26_e361_d_n2 * p.p1);
        let eq26_e363_d_n3: f64 = (eq26_e361_d_n3 * p.p1);
        let eq26_e363_d_n4: f64 = (eq26_e361_d_n4 * p.p1);
        let eq26_e363_d_n5: f64 = (eq26_e361_d_n5 * p.p1);
        let eq26_e363_d_n6: f64 = (eq26_e361_d_n6 * p.p1);
        let eq26_e363_d_n7: f64 = (eq26_e361_d_n7 * p.p1);
        let eq26_e363_d_n8: f64 = (eq26_e361_d_n8 * p.p1);
        let eq26_e363_d_n9: f64 = (eq26_e361_d_n9 * p.p1);
        let eq26_e363_d_n10: f64 = (eq26_e361_d_n10 * p.p1);
        let eq26_e363_d_n11: f64 = (eq26_e361_d_n11 * p.p1);
        (eq26_e363, eq26_e363_d_n0, eq26_e363_d_n1, eq26_e363_d_n2, eq26_e363_d_n3, eq26_e363_d_n4, eq26_e363_d_n5, eq26_e363_d_n6, eq26_e363_d_n7, eq26_e363_d_n8, eq26_e363_d_n9, eq26_e363_d_n10, eq26_e363_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e365;
        let eq26_node_derivatives: [f64; 12] = [eq26_e365_d_n0, eq26_e365_d_n1, eq26_e365_d_n2, eq26_e365_d_n3, eq26_e365_d_n4, eq26_e365_d_n5, eq26_e365_d_n6, eq26_e365_d_n7, eq26_e365_d_n8, eq26_e365_d_n9, eq26_e365_d_n10, eq26_e365_d_n11];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
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
        let (eq27_e370,) = {
    if (!(s.v[598] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e370;
        stamper.stamp_potential(
            branches[0],
            eq27_value,
            &[
            ],
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
        let (eq28_e380, eq28_e380_d_n0, eq28_e380_d_n1, eq28_e380_d_n2, eq28_e380_d_n3, eq28_e380_d_n4, eq28_e380_d_n5, eq28_e380_d_n6, eq28_e380_d_n7, eq28_e380_d_n8, eq28_e380_d_n9, eq28_e380_d_n10, eq28_e380_d_n11,) = {
    if (s.v[599] != 0.0) {
        let eq28_e374: f64 = (p.p3 * s.v[252]);
        let eq28_e374_d_n0: f64 = (p.p3 * s.dn[252][0]);
        let eq28_e374_d_n1: f64 = (p.p3 * s.dn[252][1]);
        let eq28_e374_d_n2: f64 = (p.p3 * s.dn[252][2]);
        let eq28_e374_d_n3: f64 = (p.p3 * s.dn[252][3]);
        let eq28_e374_d_n4: f64 = (p.p3 * s.dn[252][4]);
        let eq28_e374_d_n5: f64 = (p.p3 * s.dn[252][5]);
        let eq28_e374_d_n6: f64 = (p.p3 * s.dn[252][6]);
        let eq28_e374_d_n7: f64 = (p.p3 * s.dn[252][7]);
        let eq28_e374_d_n8: f64 = (p.p3 * s.dn[252][8]);
        let eq28_e374_d_n9: f64 = (p.p3 * s.dn[252][9]);
        let eq28_e374_d_n10: f64 = (p.p3 * s.dn[252][10]);
        let eq28_e374_d_n11: f64 = (p.p3 * s.dn[252][11]);
        let eq28_e376: f64 = (eq28_e374 * s.v[110]);
        let eq28_e376_d_n0: f64 = ((eq28_e374_d_n0 * s.v[110]) + (eq28_e374 * s.dn[110][0]));
        let eq28_e376_d_n1: f64 = ((eq28_e374_d_n1 * s.v[110]) + (eq28_e374 * s.dn[110][1]));
        let eq28_e376_d_n2: f64 = ((eq28_e374_d_n2 * s.v[110]) + (eq28_e374 * s.dn[110][2]));
        let eq28_e376_d_n3: f64 = ((eq28_e374_d_n3 * s.v[110]) + (eq28_e374 * s.dn[110][3]));
        let eq28_e376_d_n4: f64 = ((eq28_e374_d_n4 * s.v[110]) + (eq28_e374 * s.dn[110][4]));
        let eq28_e376_d_n5: f64 = ((eq28_e374_d_n5 * s.v[110]) + (eq28_e374 * s.dn[110][5]));
        let eq28_e376_d_n6: f64 = ((eq28_e374_d_n6 * s.v[110]) + (eq28_e374 * s.dn[110][6]));
        let eq28_e376_d_n7: f64 = ((eq28_e374_d_n7 * s.v[110]) + (eq28_e374 * s.dn[110][7]));
        let eq28_e376_d_n8: f64 = ((eq28_e374_d_n8 * s.v[110]) + (eq28_e374 * s.dn[110][8]));
        let eq28_e376_d_n9: f64 = ((eq28_e374_d_n9 * s.v[110]) + (eq28_e374 * s.dn[110][9]));
        let eq28_e376_d_n10: f64 = ((eq28_e374_d_n10 * s.v[110]) + (eq28_e374 * s.dn[110][10]));
        let eq28_e376_d_n11: f64 = ((eq28_e374_d_n11 * s.v[110]) + (eq28_e374 * s.dn[110][11]));
        let eq28_e378: f64 = (eq28_e376 * p.p1);
        let eq28_e378_d_n0: f64 = (eq28_e376_d_n0 * p.p1);
        let eq28_e378_d_n1: f64 = (eq28_e376_d_n1 * p.p1);
        let eq28_e378_d_n2: f64 = (eq28_e376_d_n2 * p.p1);
        let eq28_e378_d_n3: f64 = (eq28_e376_d_n3 * p.p1);
        let eq28_e378_d_n4: f64 = (eq28_e376_d_n4 * p.p1);
        let eq28_e378_d_n5: f64 = (eq28_e376_d_n5 * p.p1);
        let eq28_e378_d_n6: f64 = (eq28_e376_d_n6 * p.p1);
        let eq28_e378_d_n7: f64 = (eq28_e376_d_n7 * p.p1);
        let eq28_e378_d_n8: f64 = (eq28_e376_d_n8 * p.p1);
        let eq28_e378_d_n9: f64 = (eq28_e376_d_n9 * p.p1);
        let eq28_e378_d_n10: f64 = (eq28_e376_d_n10 * p.p1);
        let eq28_e378_d_n11: f64 = (eq28_e376_d_n11 * p.p1);
        (eq28_e378, eq28_e378_d_n0, eq28_e378_d_n1, eq28_e378_d_n2, eq28_e378_d_n3, eq28_e378_d_n4, eq28_e378_d_n5, eq28_e378_d_n6, eq28_e378_d_n7, eq28_e378_d_n8, eq28_e378_d_n9, eq28_e378_d_n10, eq28_e378_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e380;
        let eq28_node_derivatives: [f64; 12] = [eq28_e380_d_n0, eq28_e380_d_n1, eq28_e380_d_n2, eq28_e380_d_n3, eq28_e380_d_n4, eq28_e380_d_n5, eq28_e380_d_n6, eq28_e380_d_n7, eq28_e380_d_n8, eq28_e380_d_n9, eq28_e380_d_n10, eq28_e380_d_n11];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
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
        let (eq29_e385,) = {
    if (!(s.v[599] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e385;
        stamper.stamp_potential(
            branches[1],
            eq29_value,
            &[
            ],
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
        let eq30_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[11]),
            None,
            self.multiplicity * (eq30_value),
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq31_value: f64 = (nv11 - 0.0);
        stamper.stamp_current(
            Some(nodes[11]),
            None,
            self.multiplicity * (eq31_value),
            &[
                GeneratedDerivative::node(nodes[11], self.multiplicity * 1.0),
            ],
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq32_e394: f64 = self.eval_ddt(9, (nv11 - 0.0));
        let eq32_e394_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq32_e394_d_n11: f64 = self.ddt_jacobian(1.0);
        let eq32_e395: f64 = (s.v[330] * eq32_e394);
        let eq32_e395_d_n0: f64 = ((s.dn[330][0] * eq32_e394) + (s.v[330] * eq32_e394_d_n0));
        let eq32_e395_d_n1: f64 = ((s.dn[330][1] * eq32_e394) + (s.v[330] * eq32_e394_d_n1));
        let eq32_e395_d_n2: f64 = ((s.dn[330][2] * eq32_e394) + (s.v[330] * eq32_e394_d_n2));
        let eq32_e395_d_n3: f64 = ((s.dn[330][3] * eq32_e394) + (s.v[330] * eq32_e394_d_n3));
        let eq32_e395_d_n4: f64 = ((s.dn[330][4] * eq32_e394) + (s.v[330] * eq32_e394_d_n4));
        let eq32_e395_d_n5: f64 = ((s.dn[330][5] * eq32_e394) + (s.v[330] * eq32_e394_d_n5));
        let eq32_e395_d_n6: f64 = ((s.dn[330][6] * eq32_e394) + (s.v[330] * eq32_e394_d_n6));
        let eq32_e395_d_n7: f64 = ((s.dn[330][7] * eq32_e394) + (s.v[330] * eq32_e394_d_n7));
        let eq32_e395_d_n8: f64 = ((s.dn[330][8] * eq32_e394) + (s.v[330] * eq32_e394_d_n8));
        let eq32_e395_d_n9: f64 = ((s.dn[330][9] * eq32_e394) + (s.v[330] * eq32_e394_d_n9));
        let eq32_e395_d_n10: f64 = ((s.dn[330][10] * eq32_e394) + (s.v[330] * eq32_e394_d_n10));
        let eq32_e395_d_n11: f64 = ((s.dn[330][11] * eq32_e394) + (s.v[330] * eq32_e394_d_n11));
        let eq32_value: f64 = eq32_e395;
        let eq32_node_derivatives: [f64; 12] = [eq32_e395_d_n0, eq32_e395_d_n1, eq32_e395_d_n2, eq32_e395_d_n3, eq32_e395_d_n4, eq32_e395_d_n5, eq32_e395_d_n6, eq32_e395_d_n7, eq32_e395_d_n8, eq32_e395_d_n9, eq32_e395_d_n10, eq32_e395_d_n11];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            self.multiplicity * (eq32_value),
            &nodes,
            &eq32_node_derivatives,
            &branches,
            &eq32_branch_derivatives,
            self.multiplicity,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq33_e398: f64 = (s.v[328] * (nv11 - 0.0));
        let eq33_e398_d_n0: f64 = (s.dn[328][0] * (nv11 - 0.0));
        let eq33_e398_d_n1: f64 = (s.dn[328][1] * (nv11 - 0.0));
        let eq33_e398_d_n2: f64 = (s.dn[328][2] * (nv11 - 0.0));
        let eq33_e398_d_n3: f64 = (s.dn[328][3] * (nv11 - 0.0));
        let eq33_e398_d_n4: f64 = (s.dn[328][4] * (nv11 - 0.0));
        let eq33_e398_d_n5: f64 = (s.dn[328][5] * (nv11 - 0.0));
        let eq33_e398_d_n6: f64 = (s.dn[328][6] * (nv11 - 0.0));
        let eq33_e398_d_n7: f64 = (s.dn[328][7] * (nv11 - 0.0));
        let eq33_e398_d_n8: f64 = (s.dn[328][8] * (nv11 - 0.0));
        let eq33_e398_d_n9: f64 = (s.dn[328][9] * (nv11 - 0.0));
        let eq33_e398_d_n10: f64 = (s.dn[328][10] * (nv11 - 0.0));
        let eq33_e398_d_n11: f64 = ((s.dn[328][11] * (nv11 - 0.0)) + s.v[328]);
        let eq33_value: f64 = eq33_e398;
        let eq33_node_derivatives: [f64; 12] = [eq33_e398_d_n0, eq33_e398_d_n1, eq33_e398_d_n2, eq33_e398_d_n3, eq33_e398_d_n4, eq33_e398_d_n5, eq33_e398_d_n6, eq33_e398_d_n7, eq33_e398_d_n8, eq33_e398_d_n9, eq33_e398_d_n10, eq33_e398_d_n11];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq33_value),
            &nodes,
            &eq33_node_derivatives,
            &branches,
            &eq33_branch_derivatives,
            self.multiplicity,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq34_value: f64 = (nv11 - 0.0);
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[4]),
            self.multiplicity * (eq34_value),
            &[
                GeneratedDerivative::node(nodes[11], self.multiplicity * 1.0),
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
        let eq35_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq35_value),
            &[
            ],
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
        let eq36_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[4]),
            self.multiplicity * (eq36_value),
            &[
            ],
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
        let eq37_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[4]),
            self.multiplicity * (eq37_value),
            &[
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
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq38_value),
            &[
            ],
        );
    }
}
