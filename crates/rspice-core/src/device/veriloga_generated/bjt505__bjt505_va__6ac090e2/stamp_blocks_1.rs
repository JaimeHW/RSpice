#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq7_e223: f64 = (p.p3 * s.v[182]);
        let eq7_e223_d_n0: f64 = (p.p3 * s.dn[182][0]);
        let eq7_e223_d_n1: f64 = (p.p3 * s.dn[182][1]);
        let eq7_e223_d_n2: f64 = (p.p3 * s.dn[182][2]);
        let eq7_e223_d_n3: f64 = (p.p3 * s.dn[182][3]);
        let eq7_e223_d_n4: f64 = (p.p3 * s.dn[182][4]);
        let eq7_e223_d_n5: f64 = (p.p3 * s.dn[182][5]);
        let eq7_e223_d_n6: f64 = (p.p3 * s.dn[182][6]);
        let eq7_e223_d_n7: f64 = (p.p3 * s.dn[182][7]);
        let eq7_e223_d_n8: f64 = (p.p3 * s.dn[182][8]);
        let eq7_e223_d_n9: f64 = (p.p3 * s.dn[182][9]);
        let eq7_e223_d_n10: f64 = (p.p3 * s.dn[182][10]);
        let eq7_e223_d_n11: f64 = (p.p3 * s.dn[182][11]);
        let eq7_e225: f64 = (eq7_e223 * p.p1);
        let eq7_e225_d_n0: f64 = (eq7_e223_d_n0 * p.p1);
        let eq7_e225_d_n1: f64 = (eq7_e223_d_n1 * p.p1);
        let eq7_e225_d_n2: f64 = (eq7_e223_d_n2 * p.p1);
        let eq7_e225_d_n3: f64 = (eq7_e223_d_n3 * p.p1);
        let eq7_e225_d_n4: f64 = (eq7_e223_d_n4 * p.p1);
        let eq7_e225_d_n5: f64 = (eq7_e223_d_n5 * p.p1);
        let eq7_e225_d_n6: f64 = (eq7_e223_d_n6 * p.p1);
        let eq7_e225_d_n7: f64 = (eq7_e223_d_n7 * p.p1);
        let eq7_e225_d_n8: f64 = (eq7_e223_d_n8 * p.p1);
        let eq7_e225_d_n9: f64 = (eq7_e223_d_n9 * p.p1);
        let eq7_e225_d_n10: f64 = (eq7_e223_d_n10 * p.p1);
        let eq7_e225_d_n11: f64 = (eq7_e223_d_n11 * p.p1);
        let eq7_value: f64 = eq7_e225;
        let eq7_node_derivatives: [f64; 12] = [eq7_e225_d_n0, eq7_e225_d_n1, eq7_e225_d_n2, eq7_e225_d_n3, eq7_e225_d_n4, eq7_e225_d_n5, eq7_e225_d_n6, eq7_e225_d_n7, eq7_e225_d_n8, eq7_e225_d_n9, eq7_e225_d_n10, eq7_e225_d_n11];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            self.multiplicity * (eq7_value),
            &nodes,
            &eq7_node_derivatives,
            &branches,
            &eq7_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_8_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq8_e228: f64 = (p.p3 * s.v[180]);
        let eq8_e228_d_n0: f64 = (p.p3 * s.dn[180][0]);
        let eq8_e228_d_n1: f64 = (p.p3 * s.dn[180][1]);
        let eq8_e228_d_n2: f64 = (p.p3 * s.dn[180][2]);
        let eq8_e228_d_n3: f64 = (p.p3 * s.dn[180][3]);
        let eq8_e228_d_n4: f64 = (p.p3 * s.dn[180][4]);
        let eq8_e228_d_n5: f64 = (p.p3 * s.dn[180][5]);
        let eq8_e228_d_n6: f64 = (p.p3 * s.dn[180][6]);
        let eq8_e228_d_n7: f64 = (p.p3 * s.dn[180][7]);
        let eq8_e228_d_n8: f64 = (p.p3 * s.dn[180][8]);
        let eq8_e228_d_n9: f64 = (p.p3 * s.dn[180][9]);
        let eq8_e228_d_n10: f64 = (p.p3 * s.dn[180][10]);
        let eq8_e228_d_n11: f64 = (p.p3 * s.dn[180][11]);
        let eq8_e230: f64 = (eq8_e228 * p.p1);
        let eq8_e230_d_n0: f64 = (eq8_e228_d_n0 * p.p1);
        let eq8_e230_d_n1: f64 = (eq8_e228_d_n1 * p.p1);
        let eq8_e230_d_n2: f64 = (eq8_e228_d_n2 * p.p1);
        let eq8_e230_d_n3: f64 = (eq8_e228_d_n3 * p.p1);
        let eq8_e230_d_n4: f64 = (eq8_e228_d_n4 * p.p1);
        let eq8_e230_d_n5: f64 = (eq8_e228_d_n5 * p.p1);
        let eq8_e230_d_n6: f64 = (eq8_e228_d_n6 * p.p1);
        let eq8_e230_d_n7: f64 = (eq8_e228_d_n7 * p.p1);
        let eq8_e230_d_n8: f64 = (eq8_e228_d_n8 * p.p1);
        let eq8_e230_d_n9: f64 = (eq8_e228_d_n9 * p.p1);
        let eq8_e230_d_n10: f64 = (eq8_e228_d_n10 * p.p1);
        let eq8_e230_d_n11: f64 = (eq8_e228_d_n11 * p.p1);
        let eq8_value: f64 = eq8_e230;
        let eq8_node_derivatives: [f64; 12] = [eq8_e230_d_n0, eq8_e230_d_n1, eq8_e230_d_n2, eq8_e230_d_n3, eq8_e230_d_n4, eq8_e230_d_n5, eq8_e230_d_n6, eq8_e230_d_n7, eq8_e230_d_n8, eq8_e230_d_n9, eq8_e230_d_n10, eq8_e230_d_n11];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[3]),
            self.multiplicity * (eq8_value),
            &nodes,
            &eq8_node_derivatives,
            &branches,
            &eq8_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq9_e233: f64 = (p.p3 * s.v[181]);
        let eq9_e233_d_n0: f64 = (p.p3 * s.dn[181][0]);
        let eq9_e233_d_n1: f64 = (p.p3 * s.dn[181][1]);
        let eq9_e233_d_n2: f64 = (p.p3 * s.dn[181][2]);
        let eq9_e233_d_n3: f64 = (p.p3 * s.dn[181][3]);
        let eq9_e233_d_n4: f64 = (p.p3 * s.dn[181][4]);
        let eq9_e233_d_n5: f64 = (p.p3 * s.dn[181][5]);
        let eq9_e233_d_n6: f64 = (p.p3 * s.dn[181][6]);
        let eq9_e233_d_n7: f64 = (p.p3 * s.dn[181][7]);
        let eq9_e233_d_n8: f64 = (p.p3 * s.dn[181][8]);
        let eq9_e233_d_n9: f64 = (p.p3 * s.dn[181][9]);
        let eq9_e233_d_n10: f64 = (p.p3 * s.dn[181][10]);
        let eq9_e233_d_n11: f64 = (p.p3 * s.dn[181][11]);
        let eq9_e235: f64 = (eq9_e233 * p.p1);
        let eq9_e235_d_n0: f64 = (eq9_e233_d_n0 * p.p1);
        let eq9_e235_d_n1: f64 = (eq9_e233_d_n1 * p.p1);
        let eq9_e235_d_n2: f64 = (eq9_e233_d_n2 * p.p1);
        let eq9_e235_d_n3: f64 = (eq9_e233_d_n3 * p.p1);
        let eq9_e235_d_n4: f64 = (eq9_e233_d_n4 * p.p1);
        let eq9_e235_d_n5: f64 = (eq9_e233_d_n5 * p.p1);
        let eq9_e235_d_n6: f64 = (eq9_e233_d_n6 * p.p1);
        let eq9_e235_d_n7: f64 = (eq9_e233_d_n7 * p.p1);
        let eq9_e235_d_n8: f64 = (eq9_e233_d_n8 * p.p1);
        let eq9_e235_d_n9: f64 = (eq9_e233_d_n9 * p.p1);
        let eq9_e235_d_n10: f64 = (eq9_e233_d_n10 * p.p1);
        let eq9_e235_d_n11: f64 = (eq9_e233_d_n11 * p.p1);
        let eq9_value: f64 = eq9_e235;
        let eq9_node_derivatives: [f64; 12] = [eq9_e235_d_n0, eq9_e235_d_n1, eq9_e235_d_n2, eq9_e235_d_n3, eq9_e235_d_n4, eq9_e235_d_n5, eq9_e235_d_n6, eq9_e235_d_n7, eq9_e235_d_n8, eq9_e235_d_n9, eq9_e235_d_n10, eq9_e235_d_n11];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            self.multiplicity * (eq9_value),
            &nodes,
            &eq9_node_derivatives,
            &branches,
            &eq9_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq10_e238: f64 = (p.p3 * s.v[188]);
        let eq10_e238_d_n0: f64 = (p.p3 * s.dn[188][0]);
        let eq10_e238_d_n1: f64 = (p.p3 * s.dn[188][1]);
        let eq10_e238_d_n2: f64 = (p.p3 * s.dn[188][2]);
        let eq10_e238_d_n3: f64 = (p.p3 * s.dn[188][3]);
        let eq10_e238_d_n4: f64 = (p.p3 * s.dn[188][4]);
        let eq10_e238_d_n5: f64 = (p.p3 * s.dn[188][5]);
        let eq10_e238_d_n6: f64 = (p.p3 * s.dn[188][6]);
        let eq10_e238_d_n7: f64 = (p.p3 * s.dn[188][7]);
        let eq10_e238_d_n8: f64 = (p.p3 * s.dn[188][8]);
        let eq10_e238_d_n9: f64 = (p.p3 * s.dn[188][9]);
        let eq10_e238_d_n10: f64 = (p.p3 * s.dn[188][10]);
        let eq10_e238_d_n11: f64 = (p.p3 * s.dn[188][11]);
        let eq10_e240: f64 = (eq10_e238 * p.p1);
        let eq10_e240_d_n0: f64 = (eq10_e238_d_n0 * p.p1);
        let eq10_e240_d_n1: f64 = (eq10_e238_d_n1 * p.p1);
        let eq10_e240_d_n2: f64 = (eq10_e238_d_n2 * p.p1);
        let eq10_e240_d_n3: f64 = (eq10_e238_d_n3 * p.p1);
        let eq10_e240_d_n4: f64 = (eq10_e238_d_n4 * p.p1);
        let eq10_e240_d_n5: f64 = (eq10_e238_d_n5 * p.p1);
        let eq10_e240_d_n6: f64 = (eq10_e238_d_n6 * p.p1);
        let eq10_e240_d_n7: f64 = (eq10_e238_d_n7 * p.p1);
        let eq10_e240_d_n8: f64 = (eq10_e238_d_n8 * p.p1);
        let eq10_e240_d_n9: f64 = (eq10_e238_d_n9 * p.p1);
        let eq10_e240_d_n10: f64 = (eq10_e238_d_n10 * p.p1);
        let eq10_e240_d_n11: f64 = (eq10_e238_d_n11 * p.p1);
        let eq10_value: f64 = eq10_e240;
        let eq10_node_derivatives: [f64; 12] = [eq10_e240_d_n0, eq10_e240_d_n1, eq10_e240_d_n2, eq10_e240_d_n3, eq10_e240_d_n4, eq10_e240_d_n5, eq10_e240_d_n6, eq10_e240_d_n7, eq10_e240_d_n8, eq10_e240_d_n9, eq10_e240_d_n10, eq10_e240_d_n11];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq10_value),
            &nodes,
            &eq10_node_derivatives,
            &branches,
            &eq10_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq11_e243: f64 = (-1.0);
        let eq11_e245: f64 = (eq11_e243 * s.v[209]);
        let eq11_e245_d_n0: f64 = (eq11_e243 * s.dn[209][0]);
        let eq11_e245_d_n1: f64 = (eq11_e243 * s.dn[209][1]);
        let eq11_e245_d_n2: f64 = (eq11_e243 * s.dn[209][2]);
        let eq11_e245_d_n3: f64 = (eq11_e243 * s.dn[209][3]);
        let eq11_e245_d_n4: f64 = (eq11_e243 * s.dn[209][4]);
        let eq11_e245_d_n5: f64 = (eq11_e243 * s.dn[209][5]);
        let eq11_e245_d_n6: f64 = (eq11_e243 * s.dn[209][6]);
        let eq11_e245_d_n7: f64 = (eq11_e243 * s.dn[209][7]);
        let eq11_e245_d_n8: f64 = (eq11_e243 * s.dn[209][8]);
        let eq11_e245_d_n9: f64 = (eq11_e243 * s.dn[209][9]);
        let eq11_e245_d_n10: f64 = (eq11_e243 * s.dn[209][10]);
        let eq11_e245_d_n11: f64 = (eq11_e243 * s.dn[209][11]);
        let eq11_e246: f64 = (p.p3 * eq11_e245);
        let eq11_e246_d_n0: f64 = (p.p3 * eq11_e245_d_n0);
        let eq11_e246_d_n1: f64 = (p.p3 * eq11_e245_d_n1);
        let eq11_e246_d_n2: f64 = (p.p3 * eq11_e245_d_n2);
        let eq11_e246_d_n3: f64 = (p.p3 * eq11_e245_d_n3);
        let eq11_e246_d_n4: f64 = (p.p3 * eq11_e245_d_n4);
        let eq11_e246_d_n5: f64 = (p.p3 * eq11_e245_d_n5);
        let eq11_e246_d_n6: f64 = (p.p3 * eq11_e245_d_n6);
        let eq11_e246_d_n7: f64 = (p.p3 * eq11_e245_d_n7);
        let eq11_e246_d_n8: f64 = (p.p3 * eq11_e245_d_n8);
        let eq11_e246_d_n9: f64 = (p.p3 * eq11_e245_d_n9);
        let eq11_e246_d_n10: f64 = (p.p3 * eq11_e245_d_n10);
        let eq11_e246_d_n11: f64 = (p.p3 * eq11_e245_d_n11);
        let eq11_e248: f64 = (eq11_e246 * p.p1);
        let eq11_e248_d_n0: f64 = (eq11_e246_d_n0 * p.p1);
        let eq11_e248_d_n1: f64 = (eq11_e246_d_n1 * p.p1);
        let eq11_e248_d_n2: f64 = (eq11_e246_d_n2 * p.p1);
        let eq11_e248_d_n3: f64 = (eq11_e246_d_n3 * p.p1);
        let eq11_e248_d_n4: f64 = (eq11_e246_d_n4 * p.p1);
        let eq11_e248_d_n5: f64 = (eq11_e246_d_n5 * p.p1);
        let eq11_e248_d_n6: f64 = (eq11_e246_d_n6 * p.p1);
        let eq11_e248_d_n7: f64 = (eq11_e246_d_n7 * p.p1);
        let eq11_e248_d_n8: f64 = (eq11_e246_d_n8 * p.p1);
        let eq11_e248_d_n9: f64 = (eq11_e246_d_n9 * p.p1);
        let eq11_e248_d_n10: f64 = (eq11_e246_d_n10 * p.p1);
        let eq11_e248_d_n11: f64 = (eq11_e246_d_n11 * p.p1);
        let eq11_value: f64 = eq11_e248;
        let eq11_node_derivatives: [f64; 12] = [eq11_e248_d_n0, eq11_e248_d_n1, eq11_e248_d_n2, eq11_e248_d_n3, eq11_e248_d_n4, eq11_e248_d_n5, eq11_e248_d_n6, eq11_e248_d_n7, eq11_e248_d_n8, eq11_e248_d_n9, eq11_e248_d_n10, eq11_e248_d_n11];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq11_value),
            &nodes,
            &eq11_node_derivatives,
            &branches,
            &eq11_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq12_e251: f64 = (p.p3 * s.v[259]);
        let eq12_e251_d_n0: f64 = (p.p3 * s.dn[259][0]);
        let eq12_e251_d_n1: f64 = (p.p3 * s.dn[259][1]);
        let eq12_e251_d_n2: f64 = (p.p3 * s.dn[259][2]);
        let eq12_e251_d_n3: f64 = (p.p3 * s.dn[259][3]);
        let eq12_e251_d_n4: f64 = (p.p3 * s.dn[259][4]);
        let eq12_e251_d_n5: f64 = (p.p3 * s.dn[259][5]);
        let eq12_e251_d_n6: f64 = (p.p3 * s.dn[259][6]);
        let eq12_e251_d_n7: f64 = (p.p3 * s.dn[259][7]);
        let eq12_e251_d_n8: f64 = (p.p3 * s.dn[259][8]);
        let eq12_e251_d_n9: f64 = (p.p3 * s.dn[259][9]);
        let eq12_e251_d_n10: f64 = (p.p3 * s.dn[259][10]);
        let eq12_e251_d_n11: f64 = (p.p3 * s.dn[259][11]);
        let eq12_e253: f64 = (eq12_e251 / s.v[28]);
        let eq12_e253_d_n0: f64 = (((eq12_e251_d_n0 * s.v[28]) - (eq12_e251 * s.dn[28][0])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n1: f64 = (((eq12_e251_d_n1 * s.v[28]) - (eq12_e251 * s.dn[28][1])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n2: f64 = (((eq12_e251_d_n2 * s.v[28]) - (eq12_e251 * s.dn[28][2])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n3: f64 = (((eq12_e251_d_n3 * s.v[28]) - (eq12_e251 * s.dn[28][3])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n4: f64 = (((eq12_e251_d_n4 * s.v[28]) - (eq12_e251 * s.dn[28][4])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n5: f64 = (((eq12_e251_d_n5 * s.v[28]) - (eq12_e251 * s.dn[28][5])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n6: f64 = (((eq12_e251_d_n6 * s.v[28]) - (eq12_e251 * s.dn[28][6])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n7: f64 = (((eq12_e251_d_n7 * s.v[28]) - (eq12_e251 * s.dn[28][7])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n8: f64 = (((eq12_e251_d_n8 * s.v[28]) - (eq12_e251 * s.dn[28][8])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n9: f64 = (((eq12_e251_d_n9 * s.v[28]) - (eq12_e251 * s.dn[28][9])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n10: f64 = (((eq12_e251_d_n10 * s.v[28]) - (eq12_e251 * s.dn[28][10])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n11: f64 = (((eq12_e251_d_n11 * s.v[28]) - (eq12_e251 * s.dn[28][11])) / (s.v[28] * s.v[28]));
        let eq12_e255: f64 = (eq12_e253 * p.p1);
        let eq12_e255_d_n0: f64 = (eq12_e253_d_n0 * p.p1);
        let eq12_e255_d_n1: f64 = (eq12_e253_d_n1 * p.p1);
        let eq12_e255_d_n2: f64 = (eq12_e253_d_n2 * p.p1);
        let eq12_e255_d_n3: f64 = (eq12_e253_d_n3 * p.p1);
        let eq12_e255_d_n4: f64 = (eq12_e253_d_n4 * p.p1);
        let eq12_e255_d_n5: f64 = (eq12_e253_d_n5 * p.p1);
        let eq12_e255_d_n6: f64 = (eq12_e253_d_n6 * p.p1);
        let eq12_e255_d_n7: f64 = (eq12_e253_d_n7 * p.p1);
        let eq12_e255_d_n8: f64 = (eq12_e253_d_n8 * p.p1);
        let eq12_e255_d_n9: f64 = (eq12_e253_d_n9 * p.p1);
        let eq12_e255_d_n10: f64 = (eq12_e253_d_n10 * p.p1);
        let eq12_e255_d_n11: f64 = (eq12_e253_d_n11 * p.p1);
        let eq12_value: f64 = eq12_e255;
        let eq12_node_derivatives: [f64; 12] = [eq12_e255_d_n0, eq12_e255_d_n1, eq12_e255_d_n2, eq12_e255_d_n3, eq12_e255_d_n4, eq12_e255_d_n5, eq12_e255_d_n6, eq12_e255_d_n7, eq12_e255_d_n8, eq12_e255_d_n9, eq12_e255_d_n10, eq12_e255_d_n11];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[4]),
            self.multiplicity * (eq12_value),
            &nodes,
            &eq12_node_derivatives,
            &branches,
            &eq12_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq13_e258: f64 = (p.p3 * s.v[260]);
        let eq13_e258_d_n0: f64 = (p.p3 * s.dn[260][0]);
        let eq13_e258_d_n1: f64 = (p.p3 * s.dn[260][1]);
        let eq13_e258_d_n2: f64 = (p.p3 * s.dn[260][2]);
        let eq13_e258_d_n3: f64 = (p.p3 * s.dn[260][3]);
        let eq13_e258_d_n4: f64 = (p.p3 * s.dn[260][4]);
        let eq13_e258_d_n5: f64 = (p.p3 * s.dn[260][5]);
        let eq13_e258_d_n6: f64 = (p.p3 * s.dn[260][6]);
        let eq13_e258_d_n7: f64 = (p.p3 * s.dn[260][7]);
        let eq13_e258_d_n8: f64 = (p.p3 * s.dn[260][8]);
        let eq13_e258_d_n9: f64 = (p.p3 * s.dn[260][9]);
        let eq13_e258_d_n10: f64 = (p.p3 * s.dn[260][10]);
        let eq13_e258_d_n11: f64 = (p.p3 * s.dn[260][11]);
        let eq13_e260: f64 = (eq13_e258 / s.v[30]);
        let eq13_e260_d_n0: f64 = (((eq13_e258_d_n0 * s.v[30]) - (eq13_e258 * s.dn[30][0])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n1: f64 = (((eq13_e258_d_n1 * s.v[30]) - (eq13_e258 * s.dn[30][1])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n2: f64 = (((eq13_e258_d_n2 * s.v[30]) - (eq13_e258 * s.dn[30][2])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n3: f64 = (((eq13_e258_d_n3 * s.v[30]) - (eq13_e258 * s.dn[30][3])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n4: f64 = (((eq13_e258_d_n4 * s.v[30]) - (eq13_e258 * s.dn[30][4])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n5: f64 = (((eq13_e258_d_n5 * s.v[30]) - (eq13_e258 * s.dn[30][5])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n6: f64 = (((eq13_e258_d_n6 * s.v[30]) - (eq13_e258 * s.dn[30][6])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n7: f64 = (((eq13_e258_d_n7 * s.v[30]) - (eq13_e258 * s.dn[30][7])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n8: f64 = (((eq13_e258_d_n8 * s.v[30]) - (eq13_e258 * s.dn[30][8])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n9: f64 = (((eq13_e258_d_n9 * s.v[30]) - (eq13_e258 * s.dn[30][9])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n10: f64 = (((eq13_e258_d_n10 * s.v[30]) - (eq13_e258 * s.dn[30][10])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n11: f64 = (((eq13_e258_d_n11 * s.v[30]) - (eq13_e258 * s.dn[30][11])) / (s.v[30] * s.v[30]));
        let eq13_e262: f64 = (eq13_e260 * p.p1);
        let eq13_e262_d_n0: f64 = (eq13_e260_d_n0 * p.p1);
        let eq13_e262_d_n1: f64 = (eq13_e260_d_n1 * p.p1);
        let eq13_e262_d_n2: f64 = (eq13_e260_d_n2 * p.p1);
        let eq13_e262_d_n3: f64 = (eq13_e260_d_n3 * p.p1);
        let eq13_e262_d_n4: f64 = (eq13_e260_d_n4 * p.p1);
        let eq13_e262_d_n5: f64 = (eq13_e260_d_n5 * p.p1);
        let eq13_e262_d_n6: f64 = (eq13_e260_d_n6 * p.p1);
        let eq13_e262_d_n7: f64 = (eq13_e260_d_n7 * p.p1);
        let eq13_e262_d_n8: f64 = (eq13_e260_d_n8 * p.p1);
        let eq13_e262_d_n9: f64 = (eq13_e260_d_n9 * p.p1);
        let eq13_e262_d_n10: f64 = (eq13_e260_d_n10 * p.p1);
        let eq13_e262_d_n11: f64 = (eq13_e260_d_n11 * p.p1);
        let eq13_value: f64 = eq13_e262;
        let eq13_node_derivatives: [f64; 12] = [eq13_e262_d_n0, eq13_e262_d_n1, eq13_e262_d_n2, eq13_e262_d_n3, eq13_e262_d_n4, eq13_e262_d_n5, eq13_e262_d_n6, eq13_e262_d_n7, eq13_e262_d_n8, eq13_e262_d_n9, eq13_e262_d_n10, eq13_e262_d_n11];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq13_value),
            &nodes,
            &eq13_node_derivatives,
            &branches,
            &eq13_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq14_e266: f64 = (s.v[215] + s.v[220]);
        let eq14_e266_d_n0: f64 = (s.dn[215][0] + s.dn[220][0]);
        let eq14_e266_d_n1: f64 = (s.dn[215][1] + s.dn[220][1]);
        let eq14_e266_d_n2: f64 = (s.dn[215][2] + s.dn[220][2]);
        let eq14_e266_d_n3: f64 = (s.dn[215][3] + s.dn[220][3]);
        let eq14_e266_d_n4: f64 = (s.dn[215][4] + s.dn[220][4]);
        let eq14_e266_d_n5: f64 = (s.dn[215][5] + s.dn[220][5]);
        let eq14_e266_d_n6: f64 = (s.dn[215][6] + s.dn[220][6]);
        let eq14_e266_d_n7: f64 = (s.dn[215][7] + s.dn[220][7]);
        let eq14_e266_d_n8: f64 = (s.dn[215][8] + s.dn[220][8]);
        let eq14_e266_d_n9: f64 = (s.dn[215][9] + s.dn[220][9]);
        let eq14_e266_d_n10: f64 = (s.dn[215][10] + s.dn[220][10]);
        let eq14_e266_d_n11: f64 = (s.dn[215][11] + s.dn[220][11]);
        let eq14_e268: f64 = (eq14_e266 + s.v[235]);
        let eq14_e268_d_n0: f64 = (eq14_e266_d_n0 + s.dn[235][0]);
        let eq14_e268_d_n1: f64 = (eq14_e266_d_n1 + s.dn[235][1]);
        let eq14_e268_d_n2: f64 = (eq14_e266_d_n2 + s.dn[235][2]);
        let eq14_e268_d_n3: f64 = (eq14_e266_d_n3 + s.dn[235][3]);
        let eq14_e268_d_n4: f64 = (eq14_e266_d_n4 + s.dn[235][4]);
        let eq14_e268_d_n5: f64 = (eq14_e266_d_n5 + s.dn[235][5]);
        let eq14_e268_d_n6: f64 = (eq14_e266_d_n6 + s.dn[235][6]);
        let eq14_e268_d_n7: f64 = (eq14_e266_d_n7 + s.dn[235][7]);
        let eq14_e268_d_n8: f64 = (eq14_e266_d_n8 + s.dn[235][8]);
        let eq14_e268_d_n9: f64 = (eq14_e266_d_n9 + s.dn[235][9]);
        let eq14_e268_d_n10: f64 = (eq14_e266_d_n10 + s.dn[235][10]);
        let eq14_e268_d_n11: f64 = (eq14_e266_d_n11 + s.dn[235][11]);
        let eq14_e269: f64 = (p.p3 * eq14_e268);
        let eq14_e269_d_n0: f64 = (p.p3 * eq14_e268_d_n0);
        let eq14_e269_d_n1: f64 = (p.p3 * eq14_e268_d_n1);
        let eq14_e269_d_n2: f64 = (p.p3 * eq14_e268_d_n2);
        let eq14_e269_d_n3: f64 = (p.p3 * eq14_e268_d_n3);
        let eq14_e269_d_n4: f64 = (p.p3 * eq14_e268_d_n4);
        let eq14_e269_d_n5: f64 = (p.p3 * eq14_e268_d_n5);
        let eq14_e269_d_n6: f64 = (p.p3 * eq14_e268_d_n6);
        let eq14_e269_d_n7: f64 = (p.p3 * eq14_e268_d_n7);
        let eq14_e269_d_n8: f64 = (p.p3 * eq14_e268_d_n8);
        let eq14_e269_d_n9: f64 = (p.p3 * eq14_e268_d_n9);
        let eq14_e269_d_n10: f64 = (p.p3 * eq14_e268_d_n10);
        let eq14_e269_d_n11: f64 = (p.p3 * eq14_e268_d_n11);
        let eq14_e270: f64 = self.eval_ddt(0, eq14_e269);
        let eq14_e270_d_n0: f64 = self.ddt_jacobian(eq14_e269_d_n0);
        let eq14_e270_d_n1: f64 = self.ddt_jacobian(eq14_e269_d_n1);
        let eq14_e270_d_n2: f64 = self.ddt_jacobian(eq14_e269_d_n2);
        let eq14_e270_d_n3: f64 = self.ddt_jacobian(eq14_e269_d_n3);
        let eq14_e270_d_n4: f64 = self.ddt_jacobian(eq14_e269_d_n4);
        let eq14_e270_d_n5: f64 = self.ddt_jacobian(eq14_e269_d_n5);
        let eq14_e270_d_n6: f64 = self.ddt_jacobian(eq14_e269_d_n6);
        let eq14_e270_d_n7: f64 = self.ddt_jacobian(eq14_e269_d_n7);
        let eq14_e270_d_n8: f64 = self.ddt_jacobian(eq14_e269_d_n8);
        let eq14_e270_d_n9: f64 = self.ddt_jacobian(eq14_e269_d_n9);
        let eq14_e270_d_n10: f64 = self.ddt_jacobian(eq14_e269_d_n10);
        let eq14_e270_d_n11: f64 = self.ddt_jacobian(eq14_e269_d_n11);
        let eq14_e272: f64 = (eq14_e270 * p.p1);
        let eq14_e272_d_n0: f64 = (eq14_e270_d_n0 * p.p1);
        let eq14_e272_d_n1: f64 = (eq14_e270_d_n1 * p.p1);
        let eq14_e272_d_n2: f64 = (eq14_e270_d_n2 * p.p1);
        let eq14_e272_d_n3: f64 = (eq14_e270_d_n3 * p.p1);
        let eq14_e272_d_n4: f64 = (eq14_e270_d_n4 * p.p1);
        let eq14_e272_d_n5: f64 = (eq14_e270_d_n5 * p.p1);
        let eq14_e272_d_n6: f64 = (eq14_e270_d_n6 * p.p1);
        let eq14_e272_d_n7: f64 = (eq14_e270_d_n7 * p.p1);
        let eq14_e272_d_n8: f64 = (eq14_e270_d_n8 * p.p1);
        let eq14_e272_d_n9: f64 = (eq14_e270_d_n9 * p.p1);
        let eq14_e272_d_n10: f64 = (eq14_e270_d_n10 * p.p1);
        let eq14_e272_d_n11: f64 = (eq14_e270_d_n11 * p.p1);
        let eq14_value: f64 = eq14_e272;
        let eq14_node_derivatives: [f64; 12] = [eq14_e272_d_n0, eq14_e272_d_n1, eq14_e272_d_n2, eq14_e272_d_n3, eq14_e272_d_n4, eq14_e272_d_n5, eq14_e272_d_n6, eq14_e272_d_n7, eq14_e272_d_n8, eq14_e272_d_n9, eq14_e272_d_n10, eq14_e272_d_n11];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            self.multiplicity * (eq14_value),
            &nodes,
            &eq14_node_derivatives,
            &branches,
            &eq14_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq15_e275: f64 = (p.p3 * s.v[217]);
        let eq15_e275_d_n0: f64 = (p.p3 * s.dn[217][0]);
        let eq15_e275_d_n1: f64 = (p.p3 * s.dn[217][1]);
        let eq15_e275_d_n2: f64 = (p.p3 * s.dn[217][2]);
        let eq15_e275_d_n3: f64 = (p.p3 * s.dn[217][3]);
        let eq15_e275_d_n4: f64 = (p.p3 * s.dn[217][4]);
        let eq15_e275_d_n5: f64 = (p.p3 * s.dn[217][5]);
        let eq15_e275_d_n6: f64 = (p.p3 * s.dn[217][6]);
        let eq15_e275_d_n7: f64 = (p.p3 * s.dn[217][7]);
        let eq15_e275_d_n8: f64 = (p.p3 * s.dn[217][8]);
        let eq15_e275_d_n9: f64 = (p.p3 * s.dn[217][9]);
        let eq15_e275_d_n10: f64 = (p.p3 * s.dn[217][10]);
        let eq15_e275_d_n11: f64 = (p.p3 * s.dn[217][11]);
        let eq15_e276: f64 = self.eval_ddt(1, eq15_e275);
        let eq15_e276_d_n0: f64 = self.ddt_jacobian(eq15_e275_d_n0);
        let eq15_e276_d_n1: f64 = self.ddt_jacobian(eq15_e275_d_n1);
        let eq15_e276_d_n2: f64 = self.ddt_jacobian(eq15_e275_d_n2);
        let eq15_e276_d_n3: f64 = self.ddt_jacobian(eq15_e275_d_n3);
        let eq15_e276_d_n4: f64 = self.ddt_jacobian(eq15_e275_d_n4);
        let eq15_e276_d_n5: f64 = self.ddt_jacobian(eq15_e275_d_n5);
        let eq15_e276_d_n6: f64 = self.ddt_jacobian(eq15_e275_d_n6);
        let eq15_e276_d_n7: f64 = self.ddt_jacobian(eq15_e275_d_n7);
        let eq15_e276_d_n8: f64 = self.ddt_jacobian(eq15_e275_d_n8);
        let eq15_e276_d_n9: f64 = self.ddt_jacobian(eq15_e275_d_n9);
        let eq15_e276_d_n10: f64 = self.ddt_jacobian(eq15_e275_d_n10);
        let eq15_e276_d_n11: f64 = self.ddt_jacobian(eq15_e275_d_n11);
        let eq15_e278: f64 = (eq15_e276 * p.p1);
        let eq15_e278_d_n0: f64 = (eq15_e276_d_n0 * p.p1);
        let eq15_e278_d_n1: f64 = (eq15_e276_d_n1 * p.p1);
        let eq15_e278_d_n2: f64 = (eq15_e276_d_n2 * p.p1);
        let eq15_e278_d_n3: f64 = (eq15_e276_d_n3 * p.p1);
        let eq15_e278_d_n4: f64 = (eq15_e276_d_n4 * p.p1);
        let eq15_e278_d_n5: f64 = (eq15_e276_d_n5 * p.p1);
        let eq15_e278_d_n6: f64 = (eq15_e276_d_n6 * p.p1);
        let eq15_e278_d_n7: f64 = (eq15_e276_d_n7 * p.p1);
        let eq15_e278_d_n8: f64 = (eq15_e276_d_n8 * p.p1);
        let eq15_e278_d_n9: f64 = (eq15_e276_d_n9 * p.p1);
        let eq15_e278_d_n10: f64 = (eq15_e276_d_n10 * p.p1);
        let eq15_e278_d_n11: f64 = (eq15_e276_d_n11 * p.p1);
        let eq15_value: f64 = eq15_e278;
        let eq15_node_derivatives: [f64; 12] = [eq15_e278_d_n0, eq15_e278_d_n1, eq15_e278_d_n2, eq15_e278_d_n3, eq15_e278_d_n4, eq15_e278_d_n5, eq15_e278_d_n6, eq15_e278_d_n7, eq15_e278_d_n8, eq15_e278_d_n9, eq15_e278_d_n10, eq15_e278_d_n11];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            self.multiplicity * (eq15_value),
            &nodes,
            &eq15_node_derivatives,
            &branches,
            &eq15_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_16_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq16_e282: f64 = (s.v[218] + s.v[221]);
        let eq16_e282_d_n0: f64 = (s.dn[218][0] + s.dn[221][0]);
        let eq16_e282_d_n1: f64 = (s.dn[218][1] + s.dn[221][1]);
        let eq16_e282_d_n2: f64 = (s.dn[218][2] + s.dn[221][2]);
        let eq16_e282_d_n3: f64 = (s.dn[218][3] + s.dn[221][3]);
        let eq16_e282_d_n4: f64 = (s.dn[218][4] + s.dn[221][4]);
        let eq16_e282_d_n5: f64 = (s.dn[218][5] + s.dn[221][5]);
        let eq16_e282_d_n6: f64 = (s.dn[218][6] + s.dn[221][6]);
        let eq16_e282_d_n7: f64 = (s.dn[218][7] + s.dn[221][7]);
        let eq16_e282_d_n8: f64 = (s.dn[218][8] + s.dn[221][8]);
        let eq16_e282_d_n9: f64 = (s.dn[218][9] + s.dn[221][9]);
        let eq16_e282_d_n10: f64 = (s.dn[218][10] + s.dn[221][10]);
        let eq16_e282_d_n11: f64 = (s.dn[218][11] + s.dn[221][11]);
        let eq16_e284: f64 = (eq16_e282 + s.v[238]);
        let eq16_e284_d_n0: f64 = (eq16_e282_d_n0 + s.dn[238][0]);
        let eq16_e284_d_n1: f64 = (eq16_e282_d_n1 + s.dn[238][1]);
        let eq16_e284_d_n2: f64 = (eq16_e282_d_n2 + s.dn[238][2]);
        let eq16_e284_d_n3: f64 = (eq16_e282_d_n3 + s.dn[238][3]);
        let eq16_e284_d_n4: f64 = (eq16_e282_d_n4 + s.dn[238][4]);
        let eq16_e284_d_n5: f64 = (eq16_e282_d_n5 + s.dn[238][5]);
        let eq16_e284_d_n6: f64 = (eq16_e282_d_n6 + s.dn[238][6]);
        let eq16_e284_d_n7: f64 = (eq16_e282_d_n7 + s.dn[238][7]);
        let eq16_e284_d_n8: f64 = (eq16_e282_d_n8 + s.dn[238][8]);
        let eq16_e284_d_n9: f64 = (eq16_e282_d_n9 + s.dn[238][9]);
        let eq16_e284_d_n10: f64 = (eq16_e282_d_n10 + s.dn[238][10]);
        let eq16_e284_d_n11: f64 = (eq16_e282_d_n11 + s.dn[238][11]);
        let eq16_e285: f64 = (p.p3 * eq16_e284);
        let eq16_e285_d_n0: f64 = (p.p3 * eq16_e284_d_n0);
        let eq16_e285_d_n1: f64 = (p.p3 * eq16_e284_d_n1);
        let eq16_e285_d_n2: f64 = (p.p3 * eq16_e284_d_n2);
        let eq16_e285_d_n3: f64 = (p.p3 * eq16_e284_d_n3);
        let eq16_e285_d_n4: f64 = (p.p3 * eq16_e284_d_n4);
        let eq16_e285_d_n5: f64 = (p.p3 * eq16_e284_d_n5);
        let eq16_e285_d_n6: f64 = (p.p3 * eq16_e284_d_n6);
        let eq16_e285_d_n7: f64 = (p.p3 * eq16_e284_d_n7);
        let eq16_e285_d_n8: f64 = (p.p3 * eq16_e284_d_n8);
        let eq16_e285_d_n9: f64 = (p.p3 * eq16_e284_d_n9);
        let eq16_e285_d_n10: f64 = (p.p3 * eq16_e284_d_n10);
        let eq16_e285_d_n11: f64 = (p.p3 * eq16_e284_d_n11);
        let eq16_e286: f64 = self.eval_ddt(2, eq16_e285);
        let eq16_e286_d_n0: f64 = self.ddt_jacobian(eq16_e285_d_n0);
        let eq16_e286_d_n1: f64 = self.ddt_jacobian(eq16_e285_d_n1);
        let eq16_e286_d_n2: f64 = self.ddt_jacobian(eq16_e285_d_n2);
        let eq16_e286_d_n3: f64 = self.ddt_jacobian(eq16_e285_d_n3);
        let eq16_e286_d_n4: f64 = self.ddt_jacobian(eq16_e285_d_n4);
        let eq16_e286_d_n5: f64 = self.ddt_jacobian(eq16_e285_d_n5);
        let eq16_e286_d_n6: f64 = self.ddt_jacobian(eq16_e285_d_n6);
        let eq16_e286_d_n7: f64 = self.ddt_jacobian(eq16_e285_d_n7);
        let eq16_e286_d_n8: f64 = self.ddt_jacobian(eq16_e285_d_n8);
        let eq16_e286_d_n9: f64 = self.ddt_jacobian(eq16_e285_d_n9);
        let eq16_e286_d_n10: f64 = self.ddt_jacobian(eq16_e285_d_n10);
        let eq16_e286_d_n11: f64 = self.ddt_jacobian(eq16_e285_d_n11);
        let eq16_e288: f64 = (eq16_e286 * p.p1);
        let eq16_e288_d_n0: f64 = (eq16_e286_d_n0 * p.p1);
        let eq16_e288_d_n1: f64 = (eq16_e286_d_n1 * p.p1);
        let eq16_e288_d_n2: f64 = (eq16_e286_d_n2 * p.p1);
        let eq16_e288_d_n3: f64 = (eq16_e286_d_n3 * p.p1);
        let eq16_e288_d_n4: f64 = (eq16_e286_d_n4 * p.p1);
        let eq16_e288_d_n5: f64 = (eq16_e286_d_n5 * p.p1);
        let eq16_e288_d_n6: f64 = (eq16_e286_d_n6 * p.p1);
        let eq16_e288_d_n7: f64 = (eq16_e286_d_n7 * p.p1);
        let eq16_e288_d_n8: f64 = (eq16_e286_d_n8 * p.p1);
        let eq16_e288_d_n9: f64 = (eq16_e286_d_n9 * p.p1);
        let eq16_e288_d_n10: f64 = (eq16_e286_d_n10 * p.p1);
        let eq16_e288_d_n11: f64 = (eq16_e286_d_n11 * p.p1);
        let eq16_value: f64 = eq16_e288;
        let eq16_node_derivatives: [f64; 12] = [eq16_e288_d_n0, eq16_e288_d_n1, eq16_e288_d_n2, eq16_e288_d_n3, eq16_e288_d_n4, eq16_e288_d_n5, eq16_e288_d_n6, eq16_e288_d_n7, eq16_e288_d_n8, eq16_e288_d_n9, eq16_e288_d_n10, eq16_e288_d_n11];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq16_value),
            &nodes,
            &eq16_node_derivatives,
            &branches,
            &eq16_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq17_e291: f64 = (p.p3 * s.v[233]);
        let eq17_e291_d_n0: f64 = (p.p3 * s.dn[233][0]);
        let eq17_e291_d_n1: f64 = (p.p3 * s.dn[233][1]);
        let eq17_e291_d_n2: f64 = (p.p3 * s.dn[233][2]);
        let eq17_e291_d_n3: f64 = (p.p3 * s.dn[233][3]);
        let eq17_e291_d_n4: f64 = (p.p3 * s.dn[233][4]);
        let eq17_e291_d_n5: f64 = (p.p3 * s.dn[233][5]);
        let eq17_e291_d_n6: f64 = (p.p3 * s.dn[233][6]);
        let eq17_e291_d_n7: f64 = (p.p3 * s.dn[233][7]);
        let eq17_e291_d_n8: f64 = (p.p3 * s.dn[233][8]);
        let eq17_e291_d_n9: f64 = (p.p3 * s.dn[233][9]);
        let eq17_e291_d_n10: f64 = (p.p3 * s.dn[233][10]);
        let eq17_e291_d_n11: f64 = (p.p3 * s.dn[233][11]);
        let eq17_e292: f64 = self.eval_ddt(3, eq17_e291);
        let eq17_e292_d_n0: f64 = self.ddt_jacobian(eq17_e291_d_n0);
        let eq17_e292_d_n1: f64 = self.ddt_jacobian(eq17_e291_d_n1);
        let eq17_e292_d_n2: f64 = self.ddt_jacobian(eq17_e291_d_n2);
        let eq17_e292_d_n3: f64 = self.ddt_jacobian(eq17_e291_d_n3);
        let eq17_e292_d_n4: f64 = self.ddt_jacobian(eq17_e291_d_n4);
        let eq17_e292_d_n5: f64 = self.ddt_jacobian(eq17_e291_d_n5);
        let eq17_e292_d_n6: f64 = self.ddt_jacobian(eq17_e291_d_n6);
        let eq17_e292_d_n7: f64 = self.ddt_jacobian(eq17_e291_d_n7);
        let eq17_e292_d_n8: f64 = self.ddt_jacobian(eq17_e291_d_n8);
        let eq17_e292_d_n9: f64 = self.ddt_jacobian(eq17_e291_d_n9);
        let eq17_e292_d_n10: f64 = self.ddt_jacobian(eq17_e291_d_n10);
        let eq17_e292_d_n11: f64 = self.ddt_jacobian(eq17_e291_d_n11);
        let eq17_e294: f64 = (eq17_e292 * p.p1);
        let eq17_e294_d_n0: f64 = (eq17_e292_d_n0 * p.p1);
        let eq17_e294_d_n1: f64 = (eq17_e292_d_n1 * p.p1);
        let eq17_e294_d_n2: f64 = (eq17_e292_d_n2 * p.p1);
        let eq17_e294_d_n3: f64 = (eq17_e292_d_n3 * p.p1);
        let eq17_e294_d_n4: f64 = (eq17_e292_d_n4 * p.p1);
        let eq17_e294_d_n5: f64 = (eq17_e292_d_n5 * p.p1);
        let eq17_e294_d_n6: f64 = (eq17_e292_d_n6 * p.p1);
        let eq17_e294_d_n7: f64 = (eq17_e292_d_n7 * p.p1);
        let eq17_e294_d_n8: f64 = (eq17_e292_d_n8 * p.p1);
        let eq17_e294_d_n9: f64 = (eq17_e292_d_n9 * p.p1);
        let eq17_e294_d_n10: f64 = (eq17_e292_d_n10 * p.p1);
        let eq17_e294_d_n11: f64 = (eq17_e292_d_n11 * p.p1);
        let eq17_value: f64 = eq17_e294;
        let eq17_node_derivatives: [f64; 12] = [eq17_e294_d_n0, eq17_e294_d_n1, eq17_e294_d_n2, eq17_e294_d_n3, eq17_e294_d_n4, eq17_e294_d_n5, eq17_e294_d_n6, eq17_e294_d_n7, eq17_e294_d_n8, eq17_e294_d_n9, eq17_e294_d_n10, eq17_e294_d_n11];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            self.multiplicity * (eq17_value),
            &nodes,
            &eq17_node_derivatives,
            &branches,
            &eq17_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq18_e297: f64 = (p.p3 * s.v[222]);
        let eq18_e297_d_n0: f64 = (p.p3 * s.dn[222][0]);
        let eq18_e297_d_n1: f64 = (p.p3 * s.dn[222][1]);
        let eq18_e297_d_n2: f64 = (p.p3 * s.dn[222][2]);
        let eq18_e297_d_n3: f64 = (p.p3 * s.dn[222][3]);
        let eq18_e297_d_n4: f64 = (p.p3 * s.dn[222][4]);
        let eq18_e297_d_n5: f64 = (p.p3 * s.dn[222][5]);
        let eq18_e297_d_n6: f64 = (p.p3 * s.dn[222][6]);
        let eq18_e297_d_n7: f64 = (p.p3 * s.dn[222][7]);
        let eq18_e297_d_n8: f64 = (p.p3 * s.dn[222][8]);
        let eq18_e297_d_n9: f64 = (p.p3 * s.dn[222][9]);
        let eq18_e297_d_n10: f64 = (p.p3 * s.dn[222][10]);
        let eq18_e297_d_n11: f64 = (p.p3 * s.dn[222][11]);
        let eq18_e298: f64 = self.eval_ddt(4, eq18_e297);
        let eq18_e298_d_n0: f64 = self.ddt_jacobian(eq18_e297_d_n0);
        let eq18_e298_d_n1: f64 = self.ddt_jacobian(eq18_e297_d_n1);
        let eq18_e298_d_n2: f64 = self.ddt_jacobian(eq18_e297_d_n2);
        let eq18_e298_d_n3: f64 = self.ddt_jacobian(eq18_e297_d_n3);
        let eq18_e298_d_n4: f64 = self.ddt_jacobian(eq18_e297_d_n4);
        let eq18_e298_d_n5: f64 = self.ddt_jacobian(eq18_e297_d_n5);
        let eq18_e298_d_n6: f64 = self.ddt_jacobian(eq18_e297_d_n6);
        let eq18_e298_d_n7: f64 = self.ddt_jacobian(eq18_e297_d_n7);
        let eq18_e298_d_n8: f64 = self.ddt_jacobian(eq18_e297_d_n8);
        let eq18_e298_d_n9: f64 = self.ddt_jacobian(eq18_e297_d_n9);
        let eq18_e298_d_n10: f64 = self.ddt_jacobian(eq18_e297_d_n10);
        let eq18_e298_d_n11: f64 = self.ddt_jacobian(eq18_e297_d_n11);
        let eq18_e300: f64 = (eq18_e298 * p.p1);
        let eq18_e300_d_n0: f64 = (eq18_e298_d_n0 * p.p1);
        let eq18_e300_d_n1: f64 = (eq18_e298_d_n1 * p.p1);
        let eq18_e300_d_n2: f64 = (eq18_e298_d_n2 * p.p1);
        let eq18_e300_d_n3: f64 = (eq18_e298_d_n3 * p.p1);
        let eq18_e300_d_n4: f64 = (eq18_e298_d_n4 * p.p1);
        let eq18_e300_d_n5: f64 = (eq18_e298_d_n5 * p.p1);
        let eq18_e300_d_n6: f64 = (eq18_e298_d_n6 * p.p1);
        let eq18_e300_d_n7: f64 = (eq18_e298_d_n7 * p.p1);
        let eq18_e300_d_n8: f64 = (eq18_e298_d_n8 * p.p1);
        let eq18_e300_d_n9: f64 = (eq18_e298_d_n9 * p.p1);
        let eq18_e300_d_n10: f64 = (eq18_e298_d_n10 * p.p1);
        let eq18_e300_d_n11: f64 = (eq18_e298_d_n11 * p.p1);
        let eq18_value: f64 = eq18_e300;
        let eq18_node_derivatives: [f64; 12] = [eq18_e300_d_n0, eq18_e300_d_n1, eq18_e300_d_n2, eq18_e300_d_n3, eq18_e300_d_n4, eq18_e300_d_n5, eq18_e300_d_n6, eq18_e300_d_n7, eq18_e300_d_n8, eq18_e300_d_n9, eq18_e300_d_n10, eq18_e300_d_n11];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq18_value),
            &nodes,
            &eq18_node_derivatives,
            &branches,
            &eq18_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq19_e303: f64 = (p.p3 * p.p69);
        let eq19_e305: f64 = (eq19_e303 * s.v[263]);
        let eq19_e305_d_n0: f64 = (eq19_e303 * s.dn[263][0]);
        let eq19_e305_d_n1: f64 = (eq19_e303 * s.dn[263][1]);
        let eq19_e305_d_n2: f64 = (eq19_e303 * s.dn[263][2]);
        let eq19_e305_d_n3: f64 = (eq19_e303 * s.dn[263][3]);
        let eq19_e305_d_n4: f64 = (eq19_e303 * s.dn[263][4]);
        let eq19_e305_d_n5: f64 = (eq19_e303 * s.dn[263][5]);
        let eq19_e305_d_n6: f64 = (eq19_e303 * s.dn[263][6]);
        let eq19_e305_d_n7: f64 = (eq19_e303 * s.dn[263][7]);
        let eq19_e305_d_n8: f64 = (eq19_e303 * s.dn[263][8]);
        let eq19_e305_d_n9: f64 = (eq19_e303 * s.dn[263][9]);
        let eq19_e305_d_n10: f64 = (eq19_e303 * s.dn[263][10]);
        let eq19_e305_d_n11: f64 = (eq19_e303 * s.dn[263][11]);
        let eq19_e306: f64 = self.eval_ddt(5, eq19_e305);
        let eq19_e306_d_n0: f64 = self.ddt_jacobian(eq19_e305_d_n0);
        let eq19_e306_d_n1: f64 = self.ddt_jacobian(eq19_e305_d_n1);
        let eq19_e306_d_n2: f64 = self.ddt_jacobian(eq19_e305_d_n2);
        let eq19_e306_d_n3: f64 = self.ddt_jacobian(eq19_e305_d_n3);
        let eq19_e306_d_n4: f64 = self.ddt_jacobian(eq19_e305_d_n4);
        let eq19_e306_d_n5: f64 = self.ddt_jacobian(eq19_e305_d_n5);
        let eq19_e306_d_n6: f64 = self.ddt_jacobian(eq19_e305_d_n6);
        let eq19_e306_d_n7: f64 = self.ddt_jacobian(eq19_e305_d_n7);
        let eq19_e306_d_n8: f64 = self.ddt_jacobian(eq19_e305_d_n8);
        let eq19_e306_d_n9: f64 = self.ddt_jacobian(eq19_e305_d_n9);
        let eq19_e306_d_n10: f64 = self.ddt_jacobian(eq19_e305_d_n10);
        let eq19_e306_d_n11: f64 = self.ddt_jacobian(eq19_e305_d_n11);
        let eq19_e308: f64 = (eq19_e306 * p.p1);
        let eq19_e308_d_n0: f64 = (eq19_e306_d_n0 * p.p1);
        let eq19_e308_d_n1: f64 = (eq19_e306_d_n1 * p.p1);
        let eq19_e308_d_n2: f64 = (eq19_e306_d_n2 * p.p1);
        let eq19_e308_d_n3: f64 = (eq19_e306_d_n3 * p.p1);
        let eq19_e308_d_n4: f64 = (eq19_e306_d_n4 * p.p1);
        let eq19_e308_d_n5: f64 = (eq19_e306_d_n5 * p.p1);
        let eq19_e308_d_n6: f64 = (eq19_e306_d_n6 * p.p1);
        let eq19_e308_d_n7: f64 = (eq19_e306_d_n7 * p.p1);
        let eq19_e308_d_n8: f64 = (eq19_e306_d_n8 * p.p1);
        let eq19_e308_d_n9: f64 = (eq19_e306_d_n9 * p.p1);
        let eq19_e308_d_n10: f64 = (eq19_e306_d_n10 * p.p1);
        let eq19_e308_d_n11: f64 = (eq19_e306_d_n11 * p.p1);
        let eq19_value: f64 = eq19_e308;
        let eq19_node_derivatives: [f64; 12] = [eq19_e308_d_n0, eq19_e308_d_n1, eq19_e308_d_n2, eq19_e308_d_n3, eq19_e308_d_n4, eq19_e308_d_n5, eq19_e308_d_n6, eq19_e308_d_n7, eq19_e308_d_n8, eq19_e308_d_n9, eq19_e308_d_n10, eq19_e308_d_n11];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            self.multiplicity * (eq19_value),
            &nodes,
            &eq19_node_derivatives,
            &branches,
            &eq19_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq20_e311: f64 = (p.p3 * p.p78);
        let eq20_e313: f64 = (eq20_e311 * s.v[264]);
        let eq20_e313_d_n0: f64 = (eq20_e311 * s.dn[264][0]);
        let eq20_e313_d_n1: f64 = (eq20_e311 * s.dn[264][1]);
        let eq20_e313_d_n2: f64 = (eq20_e311 * s.dn[264][2]);
        let eq20_e313_d_n3: f64 = (eq20_e311 * s.dn[264][3]);
        let eq20_e313_d_n4: f64 = (eq20_e311 * s.dn[264][4]);
        let eq20_e313_d_n5: f64 = (eq20_e311 * s.dn[264][5]);
        let eq20_e313_d_n6: f64 = (eq20_e311 * s.dn[264][6]);
        let eq20_e313_d_n7: f64 = (eq20_e311 * s.dn[264][7]);
        let eq20_e313_d_n8: f64 = (eq20_e311 * s.dn[264][8]);
        let eq20_e313_d_n9: f64 = (eq20_e311 * s.dn[264][9]);
        let eq20_e313_d_n10: f64 = (eq20_e311 * s.dn[264][10]);
        let eq20_e313_d_n11: f64 = (eq20_e311 * s.dn[264][11]);
        let eq20_e314: f64 = self.eval_ddt(6, eq20_e313);
        let eq20_e314_d_n0: f64 = self.ddt_jacobian(eq20_e313_d_n0);
        let eq20_e314_d_n1: f64 = self.ddt_jacobian(eq20_e313_d_n1);
        let eq20_e314_d_n2: f64 = self.ddt_jacobian(eq20_e313_d_n2);
        let eq20_e314_d_n3: f64 = self.ddt_jacobian(eq20_e313_d_n3);
        let eq20_e314_d_n4: f64 = self.ddt_jacobian(eq20_e313_d_n4);
        let eq20_e314_d_n5: f64 = self.ddt_jacobian(eq20_e313_d_n5);
        let eq20_e314_d_n6: f64 = self.ddt_jacobian(eq20_e313_d_n6);
        let eq20_e314_d_n7: f64 = self.ddt_jacobian(eq20_e313_d_n7);
        let eq20_e314_d_n8: f64 = self.ddt_jacobian(eq20_e313_d_n8);
        let eq20_e314_d_n9: f64 = self.ddt_jacobian(eq20_e313_d_n9);
        let eq20_e314_d_n10: f64 = self.ddt_jacobian(eq20_e313_d_n10);
        let eq20_e314_d_n11: f64 = self.ddt_jacobian(eq20_e313_d_n11);
        let eq20_e316: f64 = (eq20_e314 * p.p1);
        let eq20_e316_d_n0: f64 = (eq20_e314_d_n0 * p.p1);
        let eq20_e316_d_n1: f64 = (eq20_e314_d_n1 * p.p1);
        let eq20_e316_d_n2: f64 = (eq20_e314_d_n2 * p.p1);
        let eq20_e316_d_n3: f64 = (eq20_e314_d_n3 * p.p1);
        let eq20_e316_d_n4: f64 = (eq20_e314_d_n4 * p.p1);
        let eq20_e316_d_n5: f64 = (eq20_e314_d_n5 * p.p1);
        let eq20_e316_d_n6: f64 = (eq20_e314_d_n6 * p.p1);
        let eq20_e316_d_n7: f64 = (eq20_e314_d_n7 * p.p1);
        let eq20_e316_d_n8: f64 = (eq20_e314_d_n8 * p.p1);
        let eq20_e316_d_n9: f64 = (eq20_e314_d_n9 * p.p1);
        let eq20_e316_d_n10: f64 = (eq20_e314_d_n10 * p.p1);
        let eq20_e316_d_n11: f64 = (eq20_e314_d_n11 * p.p1);
        let eq20_value: f64 = eq20_e316;
        let eq20_node_derivatives: [f64; 12] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10, eq20_e316_d_n11];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            self.multiplicity * (eq20_value),
            &nodes,
            &eq20_node_derivatives,
            &branches,
            &eq20_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_21_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq21_e319: f64 = (p.p3 * s.v[176]);
        let eq21_e319_d_n0: f64 = (p.p3 * s.dn[176][0]);
        let eq21_e319_d_n1: f64 = (p.p3 * s.dn[176][1]);
        let eq21_e319_d_n2: f64 = (p.p3 * s.dn[176][2]);
        let eq21_e319_d_n3: f64 = (p.p3 * s.dn[176][3]);
        let eq21_e319_d_n4: f64 = (p.p3 * s.dn[176][4]);
        let eq21_e319_d_n5: f64 = (p.p3 * s.dn[176][5]);
        let eq21_e319_d_n6: f64 = (p.p3 * s.dn[176][6]);
        let eq21_e319_d_n7: f64 = (p.p3 * s.dn[176][7]);
        let eq21_e319_d_n8: f64 = (p.p3 * s.dn[176][8]);
        let eq21_e319_d_n9: f64 = (p.p3 * s.dn[176][9]);
        let eq21_e319_d_n10: f64 = (p.p3 * s.dn[176][10]);
        let eq21_e319_d_n11: f64 = (p.p3 * s.dn[176][11]);
        let eq21_e321: f64 = (eq21_e319 * p.p1);
        let eq21_e321_d_n0: f64 = (eq21_e319_d_n0 * p.p1);
        let eq21_e321_d_n1: f64 = (eq21_e319_d_n1 * p.p1);
        let eq21_e321_d_n2: f64 = (eq21_e319_d_n2 * p.p1);
        let eq21_e321_d_n3: f64 = (eq21_e319_d_n3 * p.p1);
        let eq21_e321_d_n4: f64 = (eq21_e319_d_n4 * p.p1);
        let eq21_e321_d_n5: f64 = (eq21_e319_d_n5 * p.p1);
        let eq21_e321_d_n6: f64 = (eq21_e319_d_n6 * p.p1);
        let eq21_e321_d_n7: f64 = (eq21_e319_d_n7 * p.p1);
        let eq21_e321_d_n8: f64 = (eq21_e319_d_n8 * p.p1);
        let eq21_e321_d_n9: f64 = (eq21_e319_d_n9 * p.p1);
        let eq21_e321_d_n10: f64 = (eq21_e319_d_n10 * p.p1);
        let eq21_e321_d_n11: f64 = (eq21_e319_d_n11 * p.p1);
        let eq21_value: f64 = eq21_e321;
        let eq21_node_derivatives: [f64; 12] = [eq21_e321_d_n0, eq21_e321_d_n1, eq21_e321_d_n2, eq21_e321_d_n3, eq21_e321_d_n4, eq21_e321_d_n5, eq21_e321_d_n6, eq21_e321_d_n7, eq21_e321_d_n8, eq21_e321_d_n9, eq21_e321_d_n10, eq21_e321_d_n11];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            self.multiplicity * (eq21_value),
            &nodes,
            &eq21_node_derivatives,
            &branches,
            &eq21_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq22_e324: f64 = (p.p3 * s.v[262]);
        let eq22_e324_d_n0: f64 = (p.p3 * s.dn[262][0]);
        let eq22_e324_d_n1: f64 = (p.p3 * s.dn[262][1]);
        let eq22_e324_d_n2: f64 = (p.p3 * s.dn[262][2]);
        let eq22_e324_d_n3: f64 = (p.p3 * s.dn[262][3]);
        let eq22_e324_d_n4: f64 = (p.p3 * s.dn[262][4]);
        let eq22_e324_d_n5: f64 = (p.p3 * s.dn[262][5]);
        let eq22_e324_d_n6: f64 = (p.p3 * s.dn[262][6]);
        let eq22_e324_d_n7: f64 = (p.p3 * s.dn[262][7]);
        let eq22_e324_d_n8: f64 = (p.p3 * s.dn[262][8]);
        let eq22_e324_d_n9: f64 = (p.p3 * s.dn[262][9]);
        let eq22_e324_d_n10: f64 = (p.p3 * s.dn[262][10]);
        let eq22_e324_d_n11: f64 = (p.p3 * s.dn[262][11]);
        let eq22_e326: f64 = (eq22_e324 * s.v[108]);
        let eq22_e326_d_n0: f64 = ((eq22_e324_d_n0 * s.v[108]) + (eq22_e324 * s.dn[108][0]));
        let eq22_e326_d_n1: f64 = ((eq22_e324_d_n1 * s.v[108]) + (eq22_e324 * s.dn[108][1]));
        let eq22_e326_d_n2: f64 = ((eq22_e324_d_n2 * s.v[108]) + (eq22_e324 * s.dn[108][2]));
        let eq22_e326_d_n3: f64 = ((eq22_e324_d_n3 * s.v[108]) + (eq22_e324 * s.dn[108][3]));
        let eq22_e326_d_n4: f64 = ((eq22_e324_d_n4 * s.v[108]) + (eq22_e324 * s.dn[108][4]));
        let eq22_e326_d_n5: f64 = ((eq22_e324_d_n5 * s.v[108]) + (eq22_e324 * s.dn[108][5]));
        let eq22_e326_d_n6: f64 = ((eq22_e324_d_n6 * s.v[108]) + (eq22_e324 * s.dn[108][6]));
        let eq22_e326_d_n7: f64 = ((eq22_e324_d_n7 * s.v[108]) + (eq22_e324 * s.dn[108][7]));
        let eq22_e326_d_n8: f64 = ((eq22_e324_d_n8 * s.v[108]) + (eq22_e324 * s.dn[108][8]));
        let eq22_e326_d_n9: f64 = ((eq22_e324_d_n9 * s.v[108]) + (eq22_e324 * s.dn[108][9]));
        let eq22_e326_d_n10: f64 = ((eq22_e324_d_n10 * s.v[108]) + (eq22_e324 * s.dn[108][10]));
        let eq22_e326_d_n11: f64 = ((eq22_e324_d_n11 * s.v[108]) + (eq22_e324 * s.dn[108][11]));
        let eq22_e328: f64 = (eq22_e326 * p.p1);
        let eq22_e328_d_n0: f64 = (eq22_e326_d_n0 * p.p1);
        let eq22_e328_d_n1: f64 = (eq22_e326_d_n1 * p.p1);
        let eq22_e328_d_n2: f64 = (eq22_e326_d_n2 * p.p1);
        let eq22_e328_d_n3: f64 = (eq22_e326_d_n3 * p.p1);
        let eq22_e328_d_n4: f64 = (eq22_e326_d_n4 * p.p1);
        let eq22_e328_d_n5: f64 = (eq22_e326_d_n5 * p.p1);
        let eq22_e328_d_n6: f64 = (eq22_e326_d_n6 * p.p1);
        let eq22_e328_d_n7: f64 = (eq22_e326_d_n7 * p.p1);
        let eq22_e328_d_n8: f64 = (eq22_e326_d_n8 * p.p1);
        let eq22_e328_d_n9: f64 = (eq22_e326_d_n9 * p.p1);
        let eq22_e328_d_n10: f64 = (eq22_e326_d_n10 * p.p1);
        let eq22_e328_d_n11: f64 = (eq22_e326_d_n11 * p.p1);
        let eq22_value: f64 = eq22_e328;
        let eq22_node_derivatives: [f64; 12] = [eq22_e328_d_n0, eq22_e328_d_n1, eq22_e328_d_n2, eq22_e328_d_n3, eq22_e328_d_n4, eq22_e328_d_n5, eq22_e328_d_n6, eq22_e328_d_n7, eq22_e328_d_n8, eq22_e328_d_n9, eq22_e328_d_n10, eq22_e328_d_n11];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[9]),
            self.multiplicity * (eq22_value),
            &nodes,
            &eq22_node_derivatives,
            &branches,
            &eq22_branch_derivatives,
            self.multiplicity,
        );
    }
}
