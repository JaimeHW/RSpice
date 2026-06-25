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
        let eq7_e227: f64 = (p.p3 * s.v[185]);
        let eq7_e227_d_n0: f64 = (p.p3 * s.dn[185][0]);
        let eq7_e227_d_n1: f64 = (p.p3 * s.dn[185][1]);
        let eq7_e227_d_n2: f64 = (p.p3 * s.dn[185][2]);
        let eq7_e227_d_n3: f64 = (p.p3 * s.dn[185][3]);
        let eq7_e227_d_n4: f64 = (p.p3 * s.dn[185][4]);
        let eq7_e227_d_n5: f64 = (p.p3 * s.dn[185][5]);
        let eq7_e227_d_n6: f64 = (p.p3 * s.dn[185][6]);
        let eq7_e227_d_n7: f64 = (p.p3 * s.dn[185][7]);
        let eq7_e227_d_n8: f64 = (p.p3 * s.dn[185][8]);
        let eq7_e227_d_n9: f64 = (p.p3 * s.dn[185][9]);
        let eq7_e227_d_n10: f64 = (p.p3 * s.dn[185][10]);
        let eq7_e227_d_n11: f64 = (p.p3 * s.dn[185][11]);
        let eq7_e227_d_n12: f64 = (p.p3 * s.dn[185][12]);
        let eq7_e229: f64 = (eq7_e227 * p.p1);
        let eq7_e229_d_n0: f64 = (eq7_e227_d_n0 * p.p1);
        let eq7_e229_d_n1: f64 = (eq7_e227_d_n1 * p.p1);
        let eq7_e229_d_n2: f64 = (eq7_e227_d_n2 * p.p1);
        let eq7_e229_d_n3: f64 = (eq7_e227_d_n3 * p.p1);
        let eq7_e229_d_n4: f64 = (eq7_e227_d_n4 * p.p1);
        let eq7_e229_d_n5: f64 = (eq7_e227_d_n5 * p.p1);
        let eq7_e229_d_n6: f64 = (eq7_e227_d_n6 * p.p1);
        let eq7_e229_d_n7: f64 = (eq7_e227_d_n7 * p.p1);
        let eq7_e229_d_n8: f64 = (eq7_e227_d_n8 * p.p1);
        let eq7_e229_d_n9: f64 = (eq7_e227_d_n9 * p.p1);
        let eq7_e229_d_n10: f64 = (eq7_e227_d_n10 * p.p1);
        let eq7_e229_d_n11: f64 = (eq7_e227_d_n11 * p.p1);
        let eq7_e229_d_n12: f64 = (eq7_e227_d_n12 * p.p1);
        let eq7_value: f64 = eq7_e229;
        let eq7_node_derivatives: [f64; 13] = [eq7_e229_d_n0, eq7_e229_d_n1, eq7_e229_d_n2, eq7_e229_d_n3, eq7_e229_d_n4, eq7_e229_d_n5, eq7_e229_d_n6, eq7_e229_d_n7, eq7_e229_d_n8, eq7_e229_d_n9, eq7_e229_d_n10, eq7_e229_d_n11, eq7_e229_d_n12];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
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
        let eq8_e232: f64 = (p.p3 * s.v[183]);
        let eq8_e232_d_n0: f64 = (p.p3 * s.dn[183][0]);
        let eq8_e232_d_n1: f64 = (p.p3 * s.dn[183][1]);
        let eq8_e232_d_n2: f64 = (p.p3 * s.dn[183][2]);
        let eq8_e232_d_n3: f64 = (p.p3 * s.dn[183][3]);
        let eq8_e232_d_n4: f64 = (p.p3 * s.dn[183][4]);
        let eq8_e232_d_n5: f64 = (p.p3 * s.dn[183][5]);
        let eq8_e232_d_n6: f64 = (p.p3 * s.dn[183][6]);
        let eq8_e232_d_n7: f64 = (p.p3 * s.dn[183][7]);
        let eq8_e232_d_n8: f64 = (p.p3 * s.dn[183][8]);
        let eq8_e232_d_n9: f64 = (p.p3 * s.dn[183][9]);
        let eq8_e232_d_n10: f64 = (p.p3 * s.dn[183][10]);
        let eq8_e232_d_n11: f64 = (p.p3 * s.dn[183][11]);
        let eq8_e232_d_n12: f64 = (p.p3 * s.dn[183][12]);
        let eq8_e234: f64 = (eq8_e232 * p.p1);
        let eq8_e234_d_n0: f64 = (eq8_e232_d_n0 * p.p1);
        let eq8_e234_d_n1: f64 = (eq8_e232_d_n1 * p.p1);
        let eq8_e234_d_n2: f64 = (eq8_e232_d_n2 * p.p1);
        let eq8_e234_d_n3: f64 = (eq8_e232_d_n3 * p.p1);
        let eq8_e234_d_n4: f64 = (eq8_e232_d_n4 * p.p1);
        let eq8_e234_d_n5: f64 = (eq8_e232_d_n5 * p.p1);
        let eq8_e234_d_n6: f64 = (eq8_e232_d_n6 * p.p1);
        let eq8_e234_d_n7: f64 = (eq8_e232_d_n7 * p.p1);
        let eq8_e234_d_n8: f64 = (eq8_e232_d_n8 * p.p1);
        let eq8_e234_d_n9: f64 = (eq8_e232_d_n9 * p.p1);
        let eq8_e234_d_n10: f64 = (eq8_e232_d_n10 * p.p1);
        let eq8_e234_d_n11: f64 = (eq8_e232_d_n11 * p.p1);
        let eq8_e234_d_n12: f64 = (eq8_e232_d_n12 * p.p1);
        let eq8_value: f64 = eq8_e234;
        let eq8_node_derivatives: [f64; 13] = [eq8_e234_d_n0, eq8_e234_d_n1, eq8_e234_d_n2, eq8_e234_d_n3, eq8_e234_d_n4, eq8_e234_d_n5, eq8_e234_d_n6, eq8_e234_d_n7, eq8_e234_d_n8, eq8_e234_d_n9, eq8_e234_d_n10, eq8_e234_d_n11, eq8_e234_d_n12];
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
        let eq9_e237: f64 = (p.p3 * s.v[184]);
        let eq9_e237_d_n0: f64 = (p.p3 * s.dn[184][0]);
        let eq9_e237_d_n1: f64 = (p.p3 * s.dn[184][1]);
        let eq9_e237_d_n2: f64 = (p.p3 * s.dn[184][2]);
        let eq9_e237_d_n3: f64 = (p.p3 * s.dn[184][3]);
        let eq9_e237_d_n4: f64 = (p.p3 * s.dn[184][4]);
        let eq9_e237_d_n5: f64 = (p.p3 * s.dn[184][5]);
        let eq9_e237_d_n6: f64 = (p.p3 * s.dn[184][6]);
        let eq9_e237_d_n7: f64 = (p.p3 * s.dn[184][7]);
        let eq9_e237_d_n8: f64 = (p.p3 * s.dn[184][8]);
        let eq9_e237_d_n9: f64 = (p.p3 * s.dn[184][9]);
        let eq9_e237_d_n10: f64 = (p.p3 * s.dn[184][10]);
        let eq9_e237_d_n11: f64 = (p.p3 * s.dn[184][11]);
        let eq9_e237_d_n12: f64 = (p.p3 * s.dn[184][12]);
        let eq9_e239: f64 = (eq9_e237 * p.p1);
        let eq9_e239_d_n0: f64 = (eq9_e237_d_n0 * p.p1);
        let eq9_e239_d_n1: f64 = (eq9_e237_d_n1 * p.p1);
        let eq9_e239_d_n2: f64 = (eq9_e237_d_n2 * p.p1);
        let eq9_e239_d_n3: f64 = (eq9_e237_d_n3 * p.p1);
        let eq9_e239_d_n4: f64 = (eq9_e237_d_n4 * p.p1);
        let eq9_e239_d_n5: f64 = (eq9_e237_d_n5 * p.p1);
        let eq9_e239_d_n6: f64 = (eq9_e237_d_n6 * p.p1);
        let eq9_e239_d_n7: f64 = (eq9_e237_d_n7 * p.p1);
        let eq9_e239_d_n8: f64 = (eq9_e237_d_n8 * p.p1);
        let eq9_e239_d_n9: f64 = (eq9_e237_d_n9 * p.p1);
        let eq9_e239_d_n10: f64 = (eq9_e237_d_n10 * p.p1);
        let eq9_e239_d_n11: f64 = (eq9_e237_d_n11 * p.p1);
        let eq9_e239_d_n12: f64 = (eq9_e237_d_n12 * p.p1);
        let eq9_value: f64 = eq9_e239;
        let eq9_node_derivatives: [f64; 13] = [eq9_e239_d_n0, eq9_e239_d_n1, eq9_e239_d_n2, eq9_e239_d_n3, eq9_e239_d_n4, eq9_e239_d_n5, eq9_e239_d_n6, eq9_e239_d_n7, eq9_e239_d_n8, eq9_e239_d_n9, eq9_e239_d_n10, eq9_e239_d_n11, eq9_e239_d_n12];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
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
        let eq10_e242: f64 = (p.p3 * s.v[191]);
        let eq10_e242_d_n0: f64 = (p.p3 * s.dn[191][0]);
        let eq10_e242_d_n1: f64 = (p.p3 * s.dn[191][1]);
        let eq10_e242_d_n2: f64 = (p.p3 * s.dn[191][2]);
        let eq10_e242_d_n3: f64 = (p.p3 * s.dn[191][3]);
        let eq10_e242_d_n4: f64 = (p.p3 * s.dn[191][4]);
        let eq10_e242_d_n5: f64 = (p.p3 * s.dn[191][5]);
        let eq10_e242_d_n6: f64 = (p.p3 * s.dn[191][6]);
        let eq10_e242_d_n7: f64 = (p.p3 * s.dn[191][7]);
        let eq10_e242_d_n8: f64 = (p.p3 * s.dn[191][8]);
        let eq10_e242_d_n9: f64 = (p.p3 * s.dn[191][9]);
        let eq10_e242_d_n10: f64 = (p.p3 * s.dn[191][10]);
        let eq10_e242_d_n11: f64 = (p.p3 * s.dn[191][11]);
        let eq10_e242_d_n12: f64 = (p.p3 * s.dn[191][12]);
        let eq10_e244: f64 = (eq10_e242 * p.p1);
        let eq10_e244_d_n0: f64 = (eq10_e242_d_n0 * p.p1);
        let eq10_e244_d_n1: f64 = (eq10_e242_d_n1 * p.p1);
        let eq10_e244_d_n2: f64 = (eq10_e242_d_n2 * p.p1);
        let eq10_e244_d_n3: f64 = (eq10_e242_d_n3 * p.p1);
        let eq10_e244_d_n4: f64 = (eq10_e242_d_n4 * p.p1);
        let eq10_e244_d_n5: f64 = (eq10_e242_d_n5 * p.p1);
        let eq10_e244_d_n6: f64 = (eq10_e242_d_n6 * p.p1);
        let eq10_e244_d_n7: f64 = (eq10_e242_d_n7 * p.p1);
        let eq10_e244_d_n8: f64 = (eq10_e242_d_n8 * p.p1);
        let eq10_e244_d_n9: f64 = (eq10_e242_d_n9 * p.p1);
        let eq10_e244_d_n10: f64 = (eq10_e242_d_n10 * p.p1);
        let eq10_e244_d_n11: f64 = (eq10_e242_d_n11 * p.p1);
        let eq10_e244_d_n12: f64 = (eq10_e242_d_n12 * p.p1);
        let eq10_value: f64 = eq10_e244;
        let eq10_node_derivatives: [f64; 13] = [eq10_e244_d_n0, eq10_e244_d_n1, eq10_e244_d_n2, eq10_e244_d_n3, eq10_e244_d_n4, eq10_e244_d_n5, eq10_e244_d_n6, eq10_e244_d_n7, eq10_e244_d_n8, eq10_e244_d_n9, eq10_e244_d_n10, eq10_e244_d_n11, eq10_e244_d_n12];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let eq11_e247: f64 = (-1.0);
        let eq11_e249: f64 = (eq11_e247 * s.v[212]);
        let eq11_e249_d_n0: f64 = (eq11_e247 * s.dn[212][0]);
        let eq11_e249_d_n1: f64 = (eq11_e247 * s.dn[212][1]);
        let eq11_e249_d_n2: f64 = (eq11_e247 * s.dn[212][2]);
        let eq11_e249_d_n3: f64 = (eq11_e247 * s.dn[212][3]);
        let eq11_e249_d_n4: f64 = (eq11_e247 * s.dn[212][4]);
        let eq11_e249_d_n5: f64 = (eq11_e247 * s.dn[212][5]);
        let eq11_e249_d_n6: f64 = (eq11_e247 * s.dn[212][6]);
        let eq11_e249_d_n7: f64 = (eq11_e247 * s.dn[212][7]);
        let eq11_e249_d_n8: f64 = (eq11_e247 * s.dn[212][8]);
        let eq11_e249_d_n9: f64 = (eq11_e247 * s.dn[212][9]);
        let eq11_e249_d_n10: f64 = (eq11_e247 * s.dn[212][10]);
        let eq11_e249_d_n11: f64 = (eq11_e247 * s.dn[212][11]);
        let eq11_e249_d_n12: f64 = (eq11_e247 * s.dn[212][12]);
        let eq11_e250: f64 = (p.p3 * eq11_e249);
        let eq11_e250_d_n0: f64 = (p.p3 * eq11_e249_d_n0);
        let eq11_e250_d_n1: f64 = (p.p3 * eq11_e249_d_n1);
        let eq11_e250_d_n2: f64 = (p.p3 * eq11_e249_d_n2);
        let eq11_e250_d_n3: f64 = (p.p3 * eq11_e249_d_n3);
        let eq11_e250_d_n4: f64 = (p.p3 * eq11_e249_d_n4);
        let eq11_e250_d_n5: f64 = (p.p3 * eq11_e249_d_n5);
        let eq11_e250_d_n6: f64 = (p.p3 * eq11_e249_d_n6);
        let eq11_e250_d_n7: f64 = (p.p3 * eq11_e249_d_n7);
        let eq11_e250_d_n8: f64 = (p.p3 * eq11_e249_d_n8);
        let eq11_e250_d_n9: f64 = (p.p3 * eq11_e249_d_n9);
        let eq11_e250_d_n10: f64 = (p.p3 * eq11_e249_d_n10);
        let eq11_e250_d_n11: f64 = (p.p3 * eq11_e249_d_n11);
        let eq11_e250_d_n12: f64 = (p.p3 * eq11_e249_d_n12);
        let eq11_e252: f64 = (eq11_e250 * p.p1);
        let eq11_e252_d_n0: f64 = (eq11_e250_d_n0 * p.p1);
        let eq11_e252_d_n1: f64 = (eq11_e250_d_n1 * p.p1);
        let eq11_e252_d_n2: f64 = (eq11_e250_d_n2 * p.p1);
        let eq11_e252_d_n3: f64 = (eq11_e250_d_n3 * p.p1);
        let eq11_e252_d_n4: f64 = (eq11_e250_d_n4 * p.p1);
        let eq11_e252_d_n5: f64 = (eq11_e250_d_n5 * p.p1);
        let eq11_e252_d_n6: f64 = (eq11_e250_d_n6 * p.p1);
        let eq11_e252_d_n7: f64 = (eq11_e250_d_n7 * p.p1);
        let eq11_e252_d_n8: f64 = (eq11_e250_d_n8 * p.p1);
        let eq11_e252_d_n9: f64 = (eq11_e250_d_n9 * p.p1);
        let eq11_e252_d_n10: f64 = (eq11_e250_d_n10 * p.p1);
        let eq11_e252_d_n11: f64 = (eq11_e250_d_n11 * p.p1);
        let eq11_e252_d_n12: f64 = (eq11_e250_d_n12 * p.p1);
        let eq11_value: f64 = eq11_e252;
        let eq11_node_derivatives: [f64; 13] = [eq11_e252_d_n0, eq11_e252_d_n1, eq11_e252_d_n2, eq11_e252_d_n3, eq11_e252_d_n4, eq11_e252_d_n5, eq11_e252_d_n6, eq11_e252_d_n7, eq11_e252_d_n8, eq11_e252_d_n9, eq11_e252_d_n10, eq11_e252_d_n11, eq11_e252_d_n12];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
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
        let eq12_e255: f64 = (p.p3 * s.v[265]);
        let eq12_e255_d_n0: f64 = (p.p3 * s.dn[265][0]);
        let eq12_e255_d_n1: f64 = (p.p3 * s.dn[265][1]);
        let eq12_e255_d_n2: f64 = (p.p3 * s.dn[265][2]);
        let eq12_e255_d_n3: f64 = (p.p3 * s.dn[265][3]);
        let eq12_e255_d_n4: f64 = (p.p3 * s.dn[265][4]);
        let eq12_e255_d_n5: f64 = (p.p3 * s.dn[265][5]);
        let eq12_e255_d_n6: f64 = (p.p3 * s.dn[265][6]);
        let eq12_e255_d_n7: f64 = (p.p3 * s.dn[265][7]);
        let eq12_e255_d_n8: f64 = (p.p3 * s.dn[265][8]);
        let eq12_e255_d_n9: f64 = (p.p3 * s.dn[265][9]);
        let eq12_e255_d_n10: f64 = (p.p3 * s.dn[265][10]);
        let eq12_e255_d_n11: f64 = (p.p3 * s.dn[265][11]);
        let eq12_e255_d_n12: f64 = (p.p3 * s.dn[265][12]);
        let eq12_e257: f64 = (eq12_e255 / s.v[28]);
        let eq12_e257_d_n0: f64 = (((eq12_e255_d_n0 * s.v[28]) - (eq12_e255 * s.dn[28][0])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n1: f64 = (((eq12_e255_d_n1 * s.v[28]) - (eq12_e255 * s.dn[28][1])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n2: f64 = (((eq12_e255_d_n2 * s.v[28]) - (eq12_e255 * s.dn[28][2])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n3: f64 = (((eq12_e255_d_n3 * s.v[28]) - (eq12_e255 * s.dn[28][3])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n4: f64 = (((eq12_e255_d_n4 * s.v[28]) - (eq12_e255 * s.dn[28][4])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n5: f64 = (((eq12_e255_d_n5 * s.v[28]) - (eq12_e255 * s.dn[28][5])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n6: f64 = (((eq12_e255_d_n6 * s.v[28]) - (eq12_e255 * s.dn[28][6])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n7: f64 = (((eq12_e255_d_n7 * s.v[28]) - (eq12_e255 * s.dn[28][7])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n8: f64 = (((eq12_e255_d_n8 * s.v[28]) - (eq12_e255 * s.dn[28][8])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n9: f64 = (((eq12_e255_d_n9 * s.v[28]) - (eq12_e255 * s.dn[28][9])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n10: f64 = (((eq12_e255_d_n10 * s.v[28]) - (eq12_e255 * s.dn[28][10])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n11: f64 = (((eq12_e255_d_n11 * s.v[28]) - (eq12_e255 * s.dn[28][11])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n12: f64 = (((eq12_e255_d_n12 * s.v[28]) - (eq12_e255 * s.dn[28][12])) / (s.v[28] * s.v[28]));
        let eq12_e259: f64 = (eq12_e257 * p.p1);
        let eq12_e259_d_n0: f64 = (eq12_e257_d_n0 * p.p1);
        let eq12_e259_d_n1: f64 = (eq12_e257_d_n1 * p.p1);
        let eq12_e259_d_n2: f64 = (eq12_e257_d_n2 * p.p1);
        let eq12_e259_d_n3: f64 = (eq12_e257_d_n3 * p.p1);
        let eq12_e259_d_n4: f64 = (eq12_e257_d_n4 * p.p1);
        let eq12_e259_d_n5: f64 = (eq12_e257_d_n5 * p.p1);
        let eq12_e259_d_n6: f64 = (eq12_e257_d_n6 * p.p1);
        let eq12_e259_d_n7: f64 = (eq12_e257_d_n7 * p.p1);
        let eq12_e259_d_n8: f64 = (eq12_e257_d_n8 * p.p1);
        let eq12_e259_d_n9: f64 = (eq12_e257_d_n9 * p.p1);
        let eq12_e259_d_n10: f64 = (eq12_e257_d_n10 * p.p1);
        let eq12_e259_d_n11: f64 = (eq12_e257_d_n11 * p.p1);
        let eq12_e259_d_n12: f64 = (eq12_e257_d_n12 * p.p1);
        let eq12_value: f64 = eq12_e259;
        let eq12_node_derivatives: [f64; 13] = [eq12_e259_d_n0, eq12_e259_d_n1, eq12_e259_d_n2, eq12_e259_d_n3, eq12_e259_d_n4, eq12_e259_d_n5, eq12_e259_d_n6, eq12_e259_d_n7, eq12_e259_d_n8, eq12_e259_d_n9, eq12_e259_d_n10, eq12_e259_d_n11, eq12_e259_d_n12];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[5]),
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
        let eq13_e262: f64 = (p.p3 * s.v[266]);
        let eq13_e262_d_n0: f64 = (p.p3 * s.dn[266][0]);
        let eq13_e262_d_n1: f64 = (p.p3 * s.dn[266][1]);
        let eq13_e262_d_n2: f64 = (p.p3 * s.dn[266][2]);
        let eq13_e262_d_n3: f64 = (p.p3 * s.dn[266][3]);
        let eq13_e262_d_n4: f64 = (p.p3 * s.dn[266][4]);
        let eq13_e262_d_n5: f64 = (p.p3 * s.dn[266][5]);
        let eq13_e262_d_n6: f64 = (p.p3 * s.dn[266][6]);
        let eq13_e262_d_n7: f64 = (p.p3 * s.dn[266][7]);
        let eq13_e262_d_n8: f64 = (p.p3 * s.dn[266][8]);
        let eq13_e262_d_n9: f64 = (p.p3 * s.dn[266][9]);
        let eq13_e262_d_n10: f64 = (p.p3 * s.dn[266][10]);
        let eq13_e262_d_n11: f64 = (p.p3 * s.dn[266][11]);
        let eq13_e262_d_n12: f64 = (p.p3 * s.dn[266][12]);
        let eq13_e264: f64 = (eq13_e262 / s.v[30]);
        let eq13_e264_d_n0: f64 = (((eq13_e262_d_n0 * s.v[30]) - (eq13_e262 * s.dn[30][0])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n1: f64 = (((eq13_e262_d_n1 * s.v[30]) - (eq13_e262 * s.dn[30][1])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n2: f64 = (((eq13_e262_d_n2 * s.v[30]) - (eq13_e262 * s.dn[30][2])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n3: f64 = (((eq13_e262_d_n3 * s.v[30]) - (eq13_e262 * s.dn[30][3])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n4: f64 = (((eq13_e262_d_n4 * s.v[30]) - (eq13_e262 * s.dn[30][4])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n5: f64 = (((eq13_e262_d_n5 * s.v[30]) - (eq13_e262 * s.dn[30][5])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n6: f64 = (((eq13_e262_d_n6 * s.v[30]) - (eq13_e262 * s.dn[30][6])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n7: f64 = (((eq13_e262_d_n7 * s.v[30]) - (eq13_e262 * s.dn[30][7])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n8: f64 = (((eq13_e262_d_n8 * s.v[30]) - (eq13_e262 * s.dn[30][8])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n9: f64 = (((eq13_e262_d_n9 * s.v[30]) - (eq13_e262 * s.dn[30][9])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n10: f64 = (((eq13_e262_d_n10 * s.v[30]) - (eq13_e262 * s.dn[30][10])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n11: f64 = (((eq13_e262_d_n11 * s.v[30]) - (eq13_e262 * s.dn[30][11])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n12: f64 = (((eq13_e262_d_n12 * s.v[30]) - (eq13_e262 * s.dn[30][12])) / (s.v[30] * s.v[30]));
        let eq13_e266: f64 = (eq13_e264 * p.p1);
        let eq13_e266_d_n0: f64 = (eq13_e264_d_n0 * p.p1);
        let eq13_e266_d_n1: f64 = (eq13_e264_d_n1 * p.p1);
        let eq13_e266_d_n2: f64 = (eq13_e264_d_n2 * p.p1);
        let eq13_e266_d_n3: f64 = (eq13_e264_d_n3 * p.p1);
        let eq13_e266_d_n4: f64 = (eq13_e264_d_n4 * p.p1);
        let eq13_e266_d_n5: f64 = (eq13_e264_d_n5 * p.p1);
        let eq13_e266_d_n6: f64 = (eq13_e264_d_n6 * p.p1);
        let eq13_e266_d_n7: f64 = (eq13_e264_d_n7 * p.p1);
        let eq13_e266_d_n8: f64 = (eq13_e264_d_n8 * p.p1);
        let eq13_e266_d_n9: f64 = (eq13_e264_d_n9 * p.p1);
        let eq13_e266_d_n10: f64 = (eq13_e264_d_n10 * p.p1);
        let eq13_e266_d_n11: f64 = (eq13_e264_d_n11 * p.p1);
        let eq13_e266_d_n12: f64 = (eq13_e264_d_n12 * p.p1);
        let eq13_value: f64 = eq13_e266;
        let eq13_node_derivatives: [f64; 13] = [eq13_e266_d_n0, eq13_e266_d_n1, eq13_e266_d_n2, eq13_e266_d_n3, eq13_e266_d_n4, eq13_e266_d_n5, eq13_e266_d_n6, eq13_e266_d_n7, eq13_e266_d_n8, eq13_e266_d_n9, eq13_e266_d_n10, eq13_e266_d_n11, eq13_e266_d_n12];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[6]),
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
        let eq14_value: f64 = s.v[102];
        let eq14_node_derivatives: [f64; 13] = [s.dn[102][0], s.dn[102][1], s.dn[102][2], s.dn[102][3], s.dn[102][4], s.dn[102][5], s.dn[102][6], s.dn[102][7], s.dn[102][8], s.dn[102][9], s.dn[102][10], s.dn[102][11], s.dn[102][12]];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let eq15_value: f64 = s.v[220];
        let eq15_node_derivatives: [f64; 13] = [s.dn[220][0], s.dn[220][1], s.dn[220][2], s.dn[220][3], s.dn[220][4], s.dn[220][5], s.dn[220][6], s.dn[220][7], s.dn[220][8], s.dn[220][9], s.dn[220][10], s.dn[220][11], s.dn[220][12]];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let eq16_e270: f64 = (-1.0);
        let eq16_e272: f64 = (eq16_e270 * s.v[219]);
        let eq16_e272_d_n0: f64 = (eq16_e270 * s.dn[219][0]);
        let eq16_e272_d_n1: f64 = (eq16_e270 * s.dn[219][1]);
        let eq16_e272_d_n2: f64 = (eq16_e270 * s.dn[219][2]);
        let eq16_e272_d_n3: f64 = (eq16_e270 * s.dn[219][3]);
        let eq16_e272_d_n4: f64 = (eq16_e270 * s.dn[219][4]);
        let eq16_e272_d_n5: f64 = (eq16_e270 * s.dn[219][5]);
        let eq16_e272_d_n6: f64 = (eq16_e270 * s.dn[219][6]);
        let eq16_e272_d_n7: f64 = (eq16_e270 * s.dn[219][7]);
        let eq16_e272_d_n8: f64 = (eq16_e270 * s.dn[219][8]);
        let eq16_e272_d_n9: f64 = (eq16_e270 * s.dn[219][9]);
        let eq16_e272_d_n10: f64 = (eq16_e270 * s.dn[219][10]);
        let eq16_e272_d_n11: f64 = (eq16_e270 * s.dn[219][11]);
        let eq16_e272_d_n12: f64 = (eq16_e270 * s.dn[219][12]);
        let eq16_e274: f64 = (eq16_e272 * p.p1);
        let eq16_e274_d_n0: f64 = (eq16_e272_d_n0 * p.p1);
        let eq16_e274_d_n1: f64 = (eq16_e272_d_n1 * p.p1);
        let eq16_e274_d_n2: f64 = (eq16_e272_d_n2 * p.p1);
        let eq16_e274_d_n3: f64 = (eq16_e272_d_n3 * p.p1);
        let eq16_e274_d_n4: f64 = (eq16_e272_d_n4 * p.p1);
        let eq16_e274_d_n5: f64 = (eq16_e272_d_n5 * p.p1);
        let eq16_e274_d_n6: f64 = (eq16_e272_d_n6 * p.p1);
        let eq16_e274_d_n7: f64 = (eq16_e272_d_n7 * p.p1);
        let eq16_e274_d_n8: f64 = (eq16_e272_d_n8 * p.p1);
        let eq16_e274_d_n9: f64 = (eq16_e272_d_n9 * p.p1);
        let eq16_e274_d_n10: f64 = (eq16_e272_d_n10 * p.p1);
        let eq16_e274_d_n11: f64 = (eq16_e272_d_n11 * p.p1);
        let eq16_e274_d_n12: f64 = (eq16_e272_d_n12 * p.p1);
        let eq16_value: f64 = eq16_e274;
        let eq16_node_derivatives: [f64; 13] = [eq16_e274_d_n0, eq16_e274_d_n1, eq16_e274_d_n2, eq16_e274_d_n3, eq16_e274_d_n4, eq16_e274_d_n5, eq16_e274_d_n6, eq16_e274_d_n7, eq16_e274_d_n8, eq16_e274_d_n9, eq16_e274_d_n10, eq16_e274_d_n11, eq16_e274_d_n12];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let eq17_e278: f64 = (s.v[221] + s.v[226]);
        let eq17_e278_d_n0: f64 = (s.dn[221][0] + s.dn[226][0]);
        let eq17_e278_d_n1: f64 = (s.dn[221][1] + s.dn[226][1]);
        let eq17_e278_d_n2: f64 = (s.dn[221][2] + s.dn[226][2]);
        let eq17_e278_d_n3: f64 = (s.dn[221][3] + s.dn[226][3]);
        let eq17_e278_d_n4: f64 = (s.dn[221][4] + s.dn[226][4]);
        let eq17_e278_d_n5: f64 = (s.dn[221][5] + s.dn[226][5]);
        let eq17_e278_d_n6: f64 = (s.dn[221][6] + s.dn[226][6]);
        let eq17_e278_d_n7: f64 = (s.dn[221][7] + s.dn[226][7]);
        let eq17_e278_d_n8: f64 = (s.dn[221][8] + s.dn[226][8]);
        let eq17_e278_d_n9: f64 = (s.dn[221][9] + s.dn[226][9]);
        let eq17_e278_d_n10: f64 = (s.dn[221][10] + s.dn[226][10]);
        let eq17_e278_d_n11: f64 = (s.dn[221][11] + s.dn[226][11]);
        let eq17_e278_d_n12: f64 = (s.dn[221][12] + s.dn[226][12]);
        let eq17_e280: f64 = (eq17_e278 + s.v[241]);
        let eq17_e280_d_n0: f64 = (eq17_e278_d_n0 + s.dn[241][0]);
        let eq17_e280_d_n1: f64 = (eq17_e278_d_n1 + s.dn[241][1]);
        let eq17_e280_d_n2: f64 = (eq17_e278_d_n2 + s.dn[241][2]);
        let eq17_e280_d_n3: f64 = (eq17_e278_d_n3 + s.dn[241][3]);
        let eq17_e280_d_n4: f64 = (eq17_e278_d_n4 + s.dn[241][4]);
        let eq17_e280_d_n5: f64 = (eq17_e278_d_n5 + s.dn[241][5]);
        let eq17_e280_d_n6: f64 = (eq17_e278_d_n6 + s.dn[241][6]);
        let eq17_e280_d_n7: f64 = (eq17_e278_d_n7 + s.dn[241][7]);
        let eq17_e280_d_n8: f64 = (eq17_e278_d_n8 + s.dn[241][8]);
        let eq17_e280_d_n9: f64 = (eq17_e278_d_n9 + s.dn[241][9]);
        let eq17_e280_d_n10: f64 = (eq17_e278_d_n10 + s.dn[241][10]);
        let eq17_e280_d_n11: f64 = (eq17_e278_d_n11 + s.dn[241][11]);
        let eq17_e280_d_n12: f64 = (eq17_e278_d_n12 + s.dn[241][12]);
        let eq17_e281: f64 = (p.p3 * eq17_e280);
        let eq17_e281_d_n0: f64 = (p.p3 * eq17_e280_d_n0);
        let eq17_e281_d_n1: f64 = (p.p3 * eq17_e280_d_n1);
        let eq17_e281_d_n2: f64 = (p.p3 * eq17_e280_d_n2);
        let eq17_e281_d_n3: f64 = (p.p3 * eq17_e280_d_n3);
        let eq17_e281_d_n4: f64 = (p.p3 * eq17_e280_d_n4);
        let eq17_e281_d_n5: f64 = (p.p3 * eq17_e280_d_n5);
        let eq17_e281_d_n6: f64 = (p.p3 * eq17_e280_d_n6);
        let eq17_e281_d_n7: f64 = (p.p3 * eq17_e280_d_n7);
        let eq17_e281_d_n8: f64 = (p.p3 * eq17_e280_d_n8);
        let eq17_e281_d_n9: f64 = (p.p3 * eq17_e280_d_n9);
        let eq17_e281_d_n10: f64 = (p.p3 * eq17_e280_d_n10);
        let eq17_e281_d_n11: f64 = (p.p3 * eq17_e280_d_n11);
        let eq17_e281_d_n12: f64 = (p.p3 * eq17_e280_d_n12);
        let eq17_e282: f64 = self.eval_ddt(1, eq17_e281);
        let eq17_e282_d_n0: f64 = self.ddt_jacobian(eq17_e281_d_n0);
        let eq17_e282_d_n1: f64 = self.ddt_jacobian(eq17_e281_d_n1);
        let eq17_e282_d_n2: f64 = self.ddt_jacobian(eq17_e281_d_n2);
        let eq17_e282_d_n3: f64 = self.ddt_jacobian(eq17_e281_d_n3);
        let eq17_e282_d_n4: f64 = self.ddt_jacobian(eq17_e281_d_n4);
        let eq17_e282_d_n5: f64 = self.ddt_jacobian(eq17_e281_d_n5);
        let eq17_e282_d_n6: f64 = self.ddt_jacobian(eq17_e281_d_n6);
        let eq17_e282_d_n7: f64 = self.ddt_jacobian(eq17_e281_d_n7);
        let eq17_e282_d_n8: f64 = self.ddt_jacobian(eq17_e281_d_n8);
        let eq17_e282_d_n9: f64 = self.ddt_jacobian(eq17_e281_d_n9);
        let eq17_e282_d_n10: f64 = self.ddt_jacobian(eq17_e281_d_n10);
        let eq17_e282_d_n11: f64 = self.ddt_jacobian(eq17_e281_d_n11);
        let eq17_e282_d_n12: f64 = self.ddt_jacobian(eq17_e281_d_n12);
        let eq17_e284: f64 = (eq17_e282 * p.p1);
        let eq17_e284_d_n0: f64 = (eq17_e282_d_n0 * p.p1);
        let eq17_e284_d_n1: f64 = (eq17_e282_d_n1 * p.p1);
        let eq17_e284_d_n2: f64 = (eq17_e282_d_n2 * p.p1);
        let eq17_e284_d_n3: f64 = (eq17_e282_d_n3 * p.p1);
        let eq17_e284_d_n4: f64 = (eq17_e282_d_n4 * p.p1);
        let eq17_e284_d_n5: f64 = (eq17_e282_d_n5 * p.p1);
        let eq17_e284_d_n6: f64 = (eq17_e282_d_n6 * p.p1);
        let eq17_e284_d_n7: f64 = (eq17_e282_d_n7 * p.p1);
        let eq17_e284_d_n8: f64 = (eq17_e282_d_n8 * p.p1);
        let eq17_e284_d_n9: f64 = (eq17_e282_d_n9 * p.p1);
        let eq17_e284_d_n10: f64 = (eq17_e282_d_n10 * p.p1);
        let eq17_e284_d_n11: f64 = (eq17_e282_d_n11 * p.p1);
        let eq17_e284_d_n12: f64 = (eq17_e282_d_n12 * p.p1);
        let eq17_value: f64 = eq17_e284;
        let eq17_node_derivatives: [f64; 13] = [eq17_e284_d_n0, eq17_e284_d_n1, eq17_e284_d_n2, eq17_e284_d_n3, eq17_e284_d_n4, eq17_e284_d_n5, eq17_e284_d_n6, eq17_e284_d_n7, eq17_e284_d_n8, eq17_e284_d_n9, eq17_e284_d_n10, eq17_e284_d_n11, eq17_e284_d_n12];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
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
        let eq18_e287: f64 = (p.p3 * s.v[223]);
        let eq18_e287_d_n0: f64 = (p.p3 * s.dn[223][0]);
        let eq18_e287_d_n1: f64 = (p.p3 * s.dn[223][1]);
        let eq18_e287_d_n2: f64 = (p.p3 * s.dn[223][2]);
        let eq18_e287_d_n3: f64 = (p.p3 * s.dn[223][3]);
        let eq18_e287_d_n4: f64 = (p.p3 * s.dn[223][4]);
        let eq18_e287_d_n5: f64 = (p.p3 * s.dn[223][5]);
        let eq18_e287_d_n6: f64 = (p.p3 * s.dn[223][6]);
        let eq18_e287_d_n7: f64 = (p.p3 * s.dn[223][7]);
        let eq18_e287_d_n8: f64 = (p.p3 * s.dn[223][8]);
        let eq18_e287_d_n9: f64 = (p.p3 * s.dn[223][9]);
        let eq18_e287_d_n10: f64 = (p.p3 * s.dn[223][10]);
        let eq18_e287_d_n11: f64 = (p.p3 * s.dn[223][11]);
        let eq18_e287_d_n12: f64 = (p.p3 * s.dn[223][12]);
        let eq18_e288: f64 = self.eval_ddt(2, eq18_e287);
        let eq18_e288_d_n0: f64 = self.ddt_jacobian(eq18_e287_d_n0);
        let eq18_e288_d_n1: f64 = self.ddt_jacobian(eq18_e287_d_n1);
        let eq18_e288_d_n2: f64 = self.ddt_jacobian(eq18_e287_d_n2);
        let eq18_e288_d_n3: f64 = self.ddt_jacobian(eq18_e287_d_n3);
        let eq18_e288_d_n4: f64 = self.ddt_jacobian(eq18_e287_d_n4);
        let eq18_e288_d_n5: f64 = self.ddt_jacobian(eq18_e287_d_n5);
        let eq18_e288_d_n6: f64 = self.ddt_jacobian(eq18_e287_d_n6);
        let eq18_e288_d_n7: f64 = self.ddt_jacobian(eq18_e287_d_n7);
        let eq18_e288_d_n8: f64 = self.ddt_jacobian(eq18_e287_d_n8);
        let eq18_e288_d_n9: f64 = self.ddt_jacobian(eq18_e287_d_n9);
        let eq18_e288_d_n10: f64 = self.ddt_jacobian(eq18_e287_d_n10);
        let eq18_e288_d_n11: f64 = self.ddt_jacobian(eq18_e287_d_n11);
        let eq18_e288_d_n12: f64 = self.ddt_jacobian(eq18_e287_d_n12);
        let eq18_e290: f64 = (eq18_e288 * p.p1);
        let eq18_e290_d_n0: f64 = (eq18_e288_d_n0 * p.p1);
        let eq18_e290_d_n1: f64 = (eq18_e288_d_n1 * p.p1);
        let eq18_e290_d_n2: f64 = (eq18_e288_d_n2 * p.p1);
        let eq18_e290_d_n3: f64 = (eq18_e288_d_n3 * p.p1);
        let eq18_e290_d_n4: f64 = (eq18_e288_d_n4 * p.p1);
        let eq18_e290_d_n5: f64 = (eq18_e288_d_n5 * p.p1);
        let eq18_e290_d_n6: f64 = (eq18_e288_d_n6 * p.p1);
        let eq18_e290_d_n7: f64 = (eq18_e288_d_n7 * p.p1);
        let eq18_e290_d_n8: f64 = (eq18_e288_d_n8 * p.p1);
        let eq18_e290_d_n9: f64 = (eq18_e288_d_n9 * p.p1);
        let eq18_e290_d_n10: f64 = (eq18_e288_d_n10 * p.p1);
        let eq18_e290_d_n11: f64 = (eq18_e288_d_n11 * p.p1);
        let eq18_e290_d_n12: f64 = (eq18_e288_d_n12 * p.p1);
        let eq18_value: f64 = eq18_e290;
        let eq18_node_derivatives: [f64; 13] = [eq18_e290_d_n0, eq18_e290_d_n1, eq18_e290_d_n2, eq18_e290_d_n3, eq18_e290_d_n4, eq18_e290_d_n5, eq18_e290_d_n6, eq18_e290_d_n7, eq18_e290_d_n8, eq18_e290_d_n9, eq18_e290_d_n10, eq18_e290_d_n11, eq18_e290_d_n12];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let eq19_e294: f64 = (s.v[224] + s.v[227]);
        let eq19_e294_d_n0: f64 = (s.dn[224][0] + s.dn[227][0]);
        let eq19_e294_d_n1: f64 = (s.dn[224][1] + s.dn[227][1]);
        let eq19_e294_d_n2: f64 = (s.dn[224][2] + s.dn[227][2]);
        let eq19_e294_d_n3: f64 = (s.dn[224][3] + s.dn[227][3]);
        let eq19_e294_d_n4: f64 = (s.dn[224][4] + s.dn[227][4]);
        let eq19_e294_d_n5: f64 = (s.dn[224][5] + s.dn[227][5]);
        let eq19_e294_d_n6: f64 = (s.dn[224][6] + s.dn[227][6]);
        let eq19_e294_d_n7: f64 = (s.dn[224][7] + s.dn[227][7]);
        let eq19_e294_d_n8: f64 = (s.dn[224][8] + s.dn[227][8]);
        let eq19_e294_d_n9: f64 = (s.dn[224][9] + s.dn[227][9]);
        let eq19_e294_d_n10: f64 = (s.dn[224][10] + s.dn[227][10]);
        let eq19_e294_d_n11: f64 = (s.dn[224][11] + s.dn[227][11]);
        let eq19_e294_d_n12: f64 = (s.dn[224][12] + s.dn[227][12]);
        let eq19_e296: f64 = (eq19_e294 + s.v[244]);
        let eq19_e296_d_n0: f64 = (eq19_e294_d_n0 + s.dn[244][0]);
        let eq19_e296_d_n1: f64 = (eq19_e294_d_n1 + s.dn[244][1]);
        let eq19_e296_d_n2: f64 = (eq19_e294_d_n2 + s.dn[244][2]);
        let eq19_e296_d_n3: f64 = (eq19_e294_d_n3 + s.dn[244][3]);
        let eq19_e296_d_n4: f64 = (eq19_e294_d_n4 + s.dn[244][4]);
        let eq19_e296_d_n5: f64 = (eq19_e294_d_n5 + s.dn[244][5]);
        let eq19_e296_d_n6: f64 = (eq19_e294_d_n6 + s.dn[244][6]);
        let eq19_e296_d_n7: f64 = (eq19_e294_d_n7 + s.dn[244][7]);
        let eq19_e296_d_n8: f64 = (eq19_e294_d_n8 + s.dn[244][8]);
        let eq19_e296_d_n9: f64 = (eq19_e294_d_n9 + s.dn[244][9]);
        let eq19_e296_d_n10: f64 = (eq19_e294_d_n10 + s.dn[244][10]);
        let eq19_e296_d_n11: f64 = (eq19_e294_d_n11 + s.dn[244][11]);
        let eq19_e296_d_n12: f64 = (eq19_e294_d_n12 + s.dn[244][12]);
        let eq19_e297: f64 = (p.p3 * eq19_e296);
        let eq19_e297_d_n0: f64 = (p.p3 * eq19_e296_d_n0);
        let eq19_e297_d_n1: f64 = (p.p3 * eq19_e296_d_n1);
        let eq19_e297_d_n2: f64 = (p.p3 * eq19_e296_d_n2);
        let eq19_e297_d_n3: f64 = (p.p3 * eq19_e296_d_n3);
        let eq19_e297_d_n4: f64 = (p.p3 * eq19_e296_d_n4);
        let eq19_e297_d_n5: f64 = (p.p3 * eq19_e296_d_n5);
        let eq19_e297_d_n6: f64 = (p.p3 * eq19_e296_d_n6);
        let eq19_e297_d_n7: f64 = (p.p3 * eq19_e296_d_n7);
        let eq19_e297_d_n8: f64 = (p.p3 * eq19_e296_d_n8);
        let eq19_e297_d_n9: f64 = (p.p3 * eq19_e296_d_n9);
        let eq19_e297_d_n10: f64 = (p.p3 * eq19_e296_d_n10);
        let eq19_e297_d_n11: f64 = (p.p3 * eq19_e296_d_n11);
        let eq19_e297_d_n12: f64 = (p.p3 * eq19_e296_d_n12);
        let eq19_e298: f64 = self.eval_ddt(3, eq19_e297);
        let eq19_e298_d_n0: f64 = self.ddt_jacobian(eq19_e297_d_n0);
        let eq19_e298_d_n1: f64 = self.ddt_jacobian(eq19_e297_d_n1);
        let eq19_e298_d_n2: f64 = self.ddt_jacobian(eq19_e297_d_n2);
        let eq19_e298_d_n3: f64 = self.ddt_jacobian(eq19_e297_d_n3);
        let eq19_e298_d_n4: f64 = self.ddt_jacobian(eq19_e297_d_n4);
        let eq19_e298_d_n5: f64 = self.ddt_jacobian(eq19_e297_d_n5);
        let eq19_e298_d_n6: f64 = self.ddt_jacobian(eq19_e297_d_n6);
        let eq19_e298_d_n7: f64 = self.ddt_jacobian(eq19_e297_d_n7);
        let eq19_e298_d_n8: f64 = self.ddt_jacobian(eq19_e297_d_n8);
        let eq19_e298_d_n9: f64 = self.ddt_jacobian(eq19_e297_d_n9);
        let eq19_e298_d_n10: f64 = self.ddt_jacobian(eq19_e297_d_n10);
        let eq19_e298_d_n11: f64 = self.ddt_jacobian(eq19_e297_d_n11);
        let eq19_e298_d_n12: f64 = self.ddt_jacobian(eq19_e297_d_n12);
        let eq19_e300: f64 = (eq19_e298 * p.p1);
        let eq19_e300_d_n0: f64 = (eq19_e298_d_n0 * p.p1);
        let eq19_e300_d_n1: f64 = (eq19_e298_d_n1 * p.p1);
        let eq19_e300_d_n2: f64 = (eq19_e298_d_n2 * p.p1);
        let eq19_e300_d_n3: f64 = (eq19_e298_d_n3 * p.p1);
        let eq19_e300_d_n4: f64 = (eq19_e298_d_n4 * p.p1);
        let eq19_e300_d_n5: f64 = (eq19_e298_d_n5 * p.p1);
        let eq19_e300_d_n6: f64 = (eq19_e298_d_n6 * p.p1);
        let eq19_e300_d_n7: f64 = (eq19_e298_d_n7 * p.p1);
        let eq19_e300_d_n8: f64 = (eq19_e298_d_n8 * p.p1);
        let eq19_e300_d_n9: f64 = (eq19_e298_d_n9 * p.p1);
        let eq19_e300_d_n10: f64 = (eq19_e298_d_n10 * p.p1);
        let eq19_e300_d_n11: f64 = (eq19_e298_d_n11 * p.p1);
        let eq19_e300_d_n12: f64 = (eq19_e298_d_n12 * p.p1);
        let eq19_value: f64 = eq19_e300;
        let eq19_node_derivatives: [f64; 13] = [eq19_e300_d_n0, eq19_e300_d_n1, eq19_e300_d_n2, eq19_e300_d_n3, eq19_e300_d_n4, eq19_e300_d_n5, eq19_e300_d_n6, eq19_e300_d_n7, eq19_e300_d_n8, eq19_e300_d_n9, eq19_e300_d_n10, eq19_e300_d_n11, eq19_e300_d_n12];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
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
        let eq20_e303: f64 = (p.p3 * s.v[239]);
        let eq20_e303_d_n0: f64 = (p.p3 * s.dn[239][0]);
        let eq20_e303_d_n1: f64 = (p.p3 * s.dn[239][1]);
        let eq20_e303_d_n2: f64 = (p.p3 * s.dn[239][2]);
        let eq20_e303_d_n3: f64 = (p.p3 * s.dn[239][3]);
        let eq20_e303_d_n4: f64 = (p.p3 * s.dn[239][4]);
        let eq20_e303_d_n5: f64 = (p.p3 * s.dn[239][5]);
        let eq20_e303_d_n6: f64 = (p.p3 * s.dn[239][6]);
        let eq20_e303_d_n7: f64 = (p.p3 * s.dn[239][7]);
        let eq20_e303_d_n8: f64 = (p.p3 * s.dn[239][8]);
        let eq20_e303_d_n9: f64 = (p.p3 * s.dn[239][9]);
        let eq20_e303_d_n10: f64 = (p.p3 * s.dn[239][10]);
        let eq20_e303_d_n11: f64 = (p.p3 * s.dn[239][11]);
        let eq20_e303_d_n12: f64 = (p.p3 * s.dn[239][12]);
        let eq20_e304: f64 = self.eval_ddt(4, eq20_e303);
        let eq20_e304_d_n0: f64 = self.ddt_jacobian(eq20_e303_d_n0);
        let eq20_e304_d_n1: f64 = self.ddt_jacobian(eq20_e303_d_n1);
        let eq20_e304_d_n2: f64 = self.ddt_jacobian(eq20_e303_d_n2);
        let eq20_e304_d_n3: f64 = self.ddt_jacobian(eq20_e303_d_n3);
        let eq20_e304_d_n4: f64 = self.ddt_jacobian(eq20_e303_d_n4);
        let eq20_e304_d_n5: f64 = self.ddt_jacobian(eq20_e303_d_n5);
        let eq20_e304_d_n6: f64 = self.ddt_jacobian(eq20_e303_d_n6);
        let eq20_e304_d_n7: f64 = self.ddt_jacobian(eq20_e303_d_n7);
        let eq20_e304_d_n8: f64 = self.ddt_jacobian(eq20_e303_d_n8);
        let eq20_e304_d_n9: f64 = self.ddt_jacobian(eq20_e303_d_n9);
        let eq20_e304_d_n10: f64 = self.ddt_jacobian(eq20_e303_d_n10);
        let eq20_e304_d_n11: f64 = self.ddt_jacobian(eq20_e303_d_n11);
        let eq20_e304_d_n12: f64 = self.ddt_jacobian(eq20_e303_d_n12);
        let eq20_e306: f64 = (eq20_e304 * p.p1);
        let eq20_e306_d_n0: f64 = (eq20_e304_d_n0 * p.p1);
        let eq20_e306_d_n1: f64 = (eq20_e304_d_n1 * p.p1);
        let eq20_e306_d_n2: f64 = (eq20_e304_d_n2 * p.p1);
        let eq20_e306_d_n3: f64 = (eq20_e304_d_n3 * p.p1);
        let eq20_e306_d_n4: f64 = (eq20_e304_d_n4 * p.p1);
        let eq20_e306_d_n5: f64 = (eq20_e304_d_n5 * p.p1);
        let eq20_e306_d_n6: f64 = (eq20_e304_d_n6 * p.p1);
        let eq20_e306_d_n7: f64 = (eq20_e304_d_n7 * p.p1);
        let eq20_e306_d_n8: f64 = (eq20_e304_d_n8 * p.p1);
        let eq20_e306_d_n9: f64 = (eq20_e304_d_n9 * p.p1);
        let eq20_e306_d_n10: f64 = (eq20_e304_d_n10 * p.p1);
        let eq20_e306_d_n11: f64 = (eq20_e304_d_n11 * p.p1);
        let eq20_e306_d_n12: f64 = (eq20_e304_d_n12 * p.p1);
        let eq20_value: f64 = eq20_e306;
        let eq20_node_derivatives: [f64; 13] = [eq20_e306_d_n0, eq20_e306_d_n1, eq20_e306_d_n2, eq20_e306_d_n3, eq20_e306_d_n4, eq20_e306_d_n5, eq20_e306_d_n6, eq20_e306_d_n7, eq20_e306_d_n8, eq20_e306_d_n9, eq20_e306_d_n10, eq20_e306_d_n11, eq20_e306_d_n12];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
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
        let eq21_e309: f64 = (p.p3 * s.v[228]);
        let eq21_e309_d_n0: f64 = (p.p3 * s.dn[228][0]);
        let eq21_e309_d_n1: f64 = (p.p3 * s.dn[228][1]);
        let eq21_e309_d_n2: f64 = (p.p3 * s.dn[228][2]);
        let eq21_e309_d_n3: f64 = (p.p3 * s.dn[228][3]);
        let eq21_e309_d_n4: f64 = (p.p3 * s.dn[228][4]);
        let eq21_e309_d_n5: f64 = (p.p3 * s.dn[228][5]);
        let eq21_e309_d_n6: f64 = (p.p3 * s.dn[228][6]);
        let eq21_e309_d_n7: f64 = (p.p3 * s.dn[228][7]);
        let eq21_e309_d_n8: f64 = (p.p3 * s.dn[228][8]);
        let eq21_e309_d_n9: f64 = (p.p3 * s.dn[228][9]);
        let eq21_e309_d_n10: f64 = (p.p3 * s.dn[228][10]);
        let eq21_e309_d_n11: f64 = (p.p3 * s.dn[228][11]);
        let eq21_e309_d_n12: f64 = (p.p3 * s.dn[228][12]);
        let eq21_e310: f64 = self.eval_ddt(5, eq21_e309);
        let eq21_e310_d_n0: f64 = self.ddt_jacobian(eq21_e309_d_n0);
        let eq21_e310_d_n1: f64 = self.ddt_jacobian(eq21_e309_d_n1);
        let eq21_e310_d_n2: f64 = self.ddt_jacobian(eq21_e309_d_n2);
        let eq21_e310_d_n3: f64 = self.ddt_jacobian(eq21_e309_d_n3);
        let eq21_e310_d_n4: f64 = self.ddt_jacobian(eq21_e309_d_n4);
        let eq21_e310_d_n5: f64 = self.ddt_jacobian(eq21_e309_d_n5);
        let eq21_e310_d_n6: f64 = self.ddt_jacobian(eq21_e309_d_n6);
        let eq21_e310_d_n7: f64 = self.ddt_jacobian(eq21_e309_d_n7);
        let eq21_e310_d_n8: f64 = self.ddt_jacobian(eq21_e309_d_n8);
        let eq21_e310_d_n9: f64 = self.ddt_jacobian(eq21_e309_d_n9);
        let eq21_e310_d_n10: f64 = self.ddt_jacobian(eq21_e309_d_n10);
        let eq21_e310_d_n11: f64 = self.ddt_jacobian(eq21_e309_d_n11);
        let eq21_e310_d_n12: f64 = self.ddt_jacobian(eq21_e309_d_n12);
        let eq21_e312: f64 = (eq21_e310 * p.p1);
        let eq21_e312_d_n0: f64 = (eq21_e310_d_n0 * p.p1);
        let eq21_e312_d_n1: f64 = (eq21_e310_d_n1 * p.p1);
        let eq21_e312_d_n2: f64 = (eq21_e310_d_n2 * p.p1);
        let eq21_e312_d_n3: f64 = (eq21_e310_d_n3 * p.p1);
        let eq21_e312_d_n4: f64 = (eq21_e310_d_n4 * p.p1);
        let eq21_e312_d_n5: f64 = (eq21_e310_d_n5 * p.p1);
        let eq21_e312_d_n6: f64 = (eq21_e310_d_n6 * p.p1);
        let eq21_e312_d_n7: f64 = (eq21_e310_d_n7 * p.p1);
        let eq21_e312_d_n8: f64 = (eq21_e310_d_n8 * p.p1);
        let eq21_e312_d_n9: f64 = (eq21_e310_d_n9 * p.p1);
        let eq21_e312_d_n10: f64 = (eq21_e310_d_n10 * p.p1);
        let eq21_e312_d_n11: f64 = (eq21_e310_d_n11 * p.p1);
        let eq21_e312_d_n12: f64 = (eq21_e310_d_n12 * p.p1);
        let eq21_value: f64 = eq21_e312;
        let eq21_node_derivatives: [f64; 13] = [eq21_e312_d_n0, eq21_e312_d_n1, eq21_e312_d_n2, eq21_e312_d_n3, eq21_e312_d_n4, eq21_e312_d_n5, eq21_e312_d_n6, eq21_e312_d_n7, eq21_e312_d_n8, eq21_e312_d_n9, eq21_e312_d_n10, eq21_e312_d_n11, eq21_e312_d_n12];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let eq22_e318: f64 = self.eval_ddt(6, eq22_e317);
        let eq22_e318_d_n0: f64 = self.ddt_jacobian(eq22_e317_d_n0);
        let eq22_e318_d_n1: f64 = self.ddt_jacobian(eq22_e317_d_n1);
        let eq22_e318_d_n2: f64 = self.ddt_jacobian(eq22_e317_d_n2);
        let eq22_e318_d_n3: f64 = self.ddt_jacobian(eq22_e317_d_n3);
        let eq22_e318_d_n4: f64 = self.ddt_jacobian(eq22_e317_d_n4);
        let eq22_e318_d_n5: f64 = self.ddt_jacobian(eq22_e317_d_n5);
        let eq22_e318_d_n6: f64 = self.ddt_jacobian(eq22_e317_d_n6);
        let eq22_e318_d_n7: f64 = self.ddt_jacobian(eq22_e317_d_n7);
        let eq22_e318_d_n8: f64 = self.ddt_jacobian(eq22_e317_d_n8);
        let eq22_e318_d_n9: f64 = self.ddt_jacobian(eq22_e317_d_n9);
        let eq22_e318_d_n10: f64 = self.ddt_jacobian(eq22_e317_d_n10);
        let eq22_e318_d_n11: f64 = self.ddt_jacobian(eq22_e317_d_n11);
        let eq22_e318_d_n12: f64 = self.ddt_jacobian(eq22_e317_d_n12);
        let eq22_e320: f64 = (eq22_e318 * p.p1);
        let eq22_e320_d_n0: f64 = (eq22_e318_d_n0 * p.p1);
        let eq22_e320_d_n1: f64 = (eq22_e318_d_n1 * p.p1);
        let eq22_e320_d_n2: f64 = (eq22_e318_d_n2 * p.p1);
        let eq22_e320_d_n3: f64 = (eq22_e318_d_n3 * p.p1);
        let eq22_e320_d_n4: f64 = (eq22_e318_d_n4 * p.p1);
        let eq22_e320_d_n5: f64 = (eq22_e318_d_n5 * p.p1);
        let eq22_e320_d_n6: f64 = (eq22_e318_d_n6 * p.p1);
        let eq22_e320_d_n7: f64 = (eq22_e318_d_n7 * p.p1);
        let eq22_e320_d_n8: f64 = (eq22_e318_d_n8 * p.p1);
        let eq22_e320_d_n9: f64 = (eq22_e318_d_n9 * p.p1);
        let eq22_e320_d_n10: f64 = (eq22_e318_d_n10 * p.p1);
        let eq22_e320_d_n11: f64 = (eq22_e318_d_n11 * p.p1);
        let eq22_e320_d_n12: f64 = (eq22_e318_d_n12 * p.p1);
        let eq22_value: f64 = eq22_e320;
        let eq22_node_derivatives: [f64; 13] = [eq22_e320_d_n0, eq22_e320_d_n1, eq22_e320_d_n2, eq22_e320_d_n3, eq22_e320_d_n4, eq22_e320_d_n5, eq22_e320_d_n6, eq22_e320_d_n7, eq22_e320_d_n8, eq22_e320_d_n9, eq22_e320_d_n10, eq22_e320_d_n11, eq22_e320_d_n12];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            self.multiplicity * (eq22_value),
            &nodes,
            &eq22_node_derivatives,
            &branches,
            &eq22_branch_derivatives,
            self.multiplicity,
        );
    }
}
