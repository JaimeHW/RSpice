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
        let eq7_e214: f64 = (-1.0);
        let eq7_e216: f64 = (eq7_e214 * s.v[201]);
        let eq7_e216_d_n0: f64 = (eq7_e214 * s.dn[201][0]);
        let eq7_e216_d_n1: f64 = (eq7_e214 * s.dn[201][1]);
        let eq7_e216_d_n2: f64 = (eq7_e214 * s.dn[201][2]);
        let eq7_e216_d_n3: f64 = (eq7_e214 * s.dn[201][3]);
        let eq7_e216_d_n4: f64 = (eq7_e214 * s.dn[201][4]);
        let eq7_e216_d_n5: f64 = (eq7_e214 * s.dn[201][5]);
        let eq7_e216_d_n6: f64 = (eq7_e214 * s.dn[201][6]);
        let eq7_e216_d_n7: f64 = (eq7_e214 * s.dn[201][7]);
        let eq7_e216_d_n8: f64 = (eq7_e214 * s.dn[201][8]);
        let eq7_e216_d_n9: f64 = (eq7_e214 * s.dn[201][9]);
        let eq7_e216_d_n10: f64 = (eq7_e214 * s.dn[201][10]);
        let eq7_e216_d_n11: f64 = (eq7_e214 * s.dn[201][11]);
        let eq7_e217: f64 = (p.p3 * eq7_e216);
        let eq7_e217_d_n0: f64 = (p.p3 * eq7_e216_d_n0);
        let eq7_e217_d_n1: f64 = (p.p3 * eq7_e216_d_n1);
        let eq7_e217_d_n2: f64 = (p.p3 * eq7_e216_d_n2);
        let eq7_e217_d_n3: f64 = (p.p3 * eq7_e216_d_n3);
        let eq7_e217_d_n4: f64 = (p.p3 * eq7_e216_d_n4);
        let eq7_e217_d_n5: f64 = (p.p3 * eq7_e216_d_n5);
        let eq7_e217_d_n6: f64 = (p.p3 * eq7_e216_d_n6);
        let eq7_e217_d_n7: f64 = (p.p3 * eq7_e216_d_n7);
        let eq7_e217_d_n8: f64 = (p.p3 * eq7_e216_d_n8);
        let eq7_e217_d_n9: f64 = (p.p3 * eq7_e216_d_n9);
        let eq7_e217_d_n10: f64 = (p.p3 * eq7_e216_d_n10);
        let eq7_e217_d_n11: f64 = (p.p3 * eq7_e216_d_n11);
        let eq7_e219: f64 = (eq7_e217 * p.p1);
        let eq7_e219_d_n0: f64 = (eq7_e217_d_n0 * p.p1);
        let eq7_e219_d_n1: f64 = (eq7_e217_d_n1 * p.p1);
        let eq7_e219_d_n2: f64 = (eq7_e217_d_n2 * p.p1);
        let eq7_e219_d_n3: f64 = (eq7_e217_d_n3 * p.p1);
        let eq7_e219_d_n4: f64 = (eq7_e217_d_n4 * p.p1);
        let eq7_e219_d_n5: f64 = (eq7_e217_d_n5 * p.p1);
        let eq7_e219_d_n6: f64 = (eq7_e217_d_n6 * p.p1);
        let eq7_e219_d_n7: f64 = (eq7_e217_d_n7 * p.p1);
        let eq7_e219_d_n8: f64 = (eq7_e217_d_n8 * p.p1);
        let eq7_e219_d_n9: f64 = (eq7_e217_d_n9 * p.p1);
        let eq7_e219_d_n10: f64 = (eq7_e217_d_n10 * p.p1);
        let eq7_e219_d_n11: f64 = (eq7_e217_d_n11 * p.p1);
        let eq7_value: f64 = eq7_e219;
        let eq7_node_derivatives: [f64; 12] = [eq7_e219_d_n0, eq7_e219_d_n1, eq7_e219_d_n2, eq7_e219_d_n3, eq7_e219_d_n4, eq7_e219_d_n5, eq7_e219_d_n6, eq7_e219_d_n7, eq7_e219_d_n8, eq7_e219_d_n9, eq7_e219_d_n10, eq7_e219_d_n11];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
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
        let eq8_e222: f64 = (p.p3 * s.v[245]);
        let eq8_e222_d_n0: f64 = (p.p3 * s.dn[245][0]);
        let eq8_e222_d_n1: f64 = (p.p3 * s.dn[245][1]);
        let eq8_e222_d_n2: f64 = (p.p3 * s.dn[245][2]);
        let eq8_e222_d_n3: f64 = (p.p3 * s.dn[245][3]);
        let eq8_e222_d_n4: f64 = (p.p3 * s.dn[245][4]);
        let eq8_e222_d_n5: f64 = (p.p3 * s.dn[245][5]);
        let eq8_e222_d_n6: f64 = (p.p3 * s.dn[245][6]);
        let eq8_e222_d_n7: f64 = (p.p3 * s.dn[245][7]);
        let eq8_e222_d_n8: f64 = (p.p3 * s.dn[245][8]);
        let eq8_e222_d_n9: f64 = (p.p3 * s.dn[245][9]);
        let eq8_e222_d_n10: f64 = (p.p3 * s.dn[245][10]);
        let eq8_e222_d_n11: f64 = (p.p3 * s.dn[245][11]);
        let eq8_e224: f64 = (eq8_e222 / s.v[28]);
        let eq8_e224_d_n0: f64 = (((eq8_e222_d_n0 * s.v[28]) - (eq8_e222 * s.dn[28][0])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n1: f64 = (((eq8_e222_d_n1 * s.v[28]) - (eq8_e222 * s.dn[28][1])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n2: f64 = (((eq8_e222_d_n2 * s.v[28]) - (eq8_e222 * s.dn[28][2])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n3: f64 = (((eq8_e222_d_n3 * s.v[28]) - (eq8_e222 * s.dn[28][3])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n4: f64 = (((eq8_e222_d_n4 * s.v[28]) - (eq8_e222 * s.dn[28][4])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n5: f64 = (((eq8_e222_d_n5 * s.v[28]) - (eq8_e222 * s.dn[28][5])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n6: f64 = (((eq8_e222_d_n6 * s.v[28]) - (eq8_e222 * s.dn[28][6])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n7: f64 = (((eq8_e222_d_n7 * s.v[28]) - (eq8_e222 * s.dn[28][7])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n8: f64 = (((eq8_e222_d_n8 * s.v[28]) - (eq8_e222 * s.dn[28][8])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n9: f64 = (((eq8_e222_d_n9 * s.v[28]) - (eq8_e222 * s.dn[28][9])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n10: f64 = (((eq8_e222_d_n10 * s.v[28]) - (eq8_e222 * s.dn[28][10])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n11: f64 = (((eq8_e222_d_n11 * s.v[28]) - (eq8_e222 * s.dn[28][11])) / (s.v[28] * s.v[28]));
        let eq8_e226: f64 = (eq8_e224 * p.p1);
        let eq8_e226_d_n0: f64 = (eq8_e224_d_n0 * p.p1);
        let eq8_e226_d_n1: f64 = (eq8_e224_d_n1 * p.p1);
        let eq8_e226_d_n2: f64 = (eq8_e224_d_n2 * p.p1);
        let eq8_e226_d_n3: f64 = (eq8_e224_d_n3 * p.p1);
        let eq8_e226_d_n4: f64 = (eq8_e224_d_n4 * p.p1);
        let eq8_e226_d_n5: f64 = (eq8_e224_d_n5 * p.p1);
        let eq8_e226_d_n6: f64 = (eq8_e224_d_n6 * p.p1);
        let eq8_e226_d_n7: f64 = (eq8_e224_d_n7 * p.p1);
        let eq8_e226_d_n8: f64 = (eq8_e224_d_n8 * p.p1);
        let eq8_e226_d_n9: f64 = (eq8_e224_d_n9 * p.p1);
        let eq8_e226_d_n10: f64 = (eq8_e224_d_n10 * p.p1);
        let eq8_e226_d_n11: f64 = (eq8_e224_d_n11 * p.p1);
        let eq8_value: f64 = eq8_e226;
        let eq8_node_derivatives: [f64; 12] = [eq8_e226_d_n0, eq8_e226_d_n1, eq8_e226_d_n2, eq8_e226_d_n3, eq8_e226_d_n4, eq8_e226_d_n5, eq8_e226_d_n6, eq8_e226_d_n7, eq8_e226_d_n8, eq8_e226_d_n9, eq8_e226_d_n10, eq8_e226_d_n11];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[4]),
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
        let eq9_e229: f64 = (p.p3 * s.v[246]);
        let eq9_e229_d_n0: f64 = (p.p3 * s.dn[246][0]);
        let eq9_e229_d_n1: f64 = (p.p3 * s.dn[246][1]);
        let eq9_e229_d_n2: f64 = (p.p3 * s.dn[246][2]);
        let eq9_e229_d_n3: f64 = (p.p3 * s.dn[246][3]);
        let eq9_e229_d_n4: f64 = (p.p3 * s.dn[246][4]);
        let eq9_e229_d_n5: f64 = (p.p3 * s.dn[246][5]);
        let eq9_e229_d_n6: f64 = (p.p3 * s.dn[246][6]);
        let eq9_e229_d_n7: f64 = (p.p3 * s.dn[246][7]);
        let eq9_e229_d_n8: f64 = (p.p3 * s.dn[246][8]);
        let eq9_e229_d_n9: f64 = (p.p3 * s.dn[246][9]);
        let eq9_e229_d_n10: f64 = (p.p3 * s.dn[246][10]);
        let eq9_e229_d_n11: f64 = (p.p3 * s.dn[246][11]);
        let eq9_e231: f64 = (eq9_e229 / s.v[30]);
        let eq9_e231_d_n0: f64 = (((eq9_e229_d_n0 * s.v[30]) - (eq9_e229 * s.dn[30][0])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n1: f64 = (((eq9_e229_d_n1 * s.v[30]) - (eq9_e229 * s.dn[30][1])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n2: f64 = (((eq9_e229_d_n2 * s.v[30]) - (eq9_e229 * s.dn[30][2])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n3: f64 = (((eq9_e229_d_n3 * s.v[30]) - (eq9_e229 * s.dn[30][3])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n4: f64 = (((eq9_e229_d_n4 * s.v[30]) - (eq9_e229 * s.dn[30][4])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n5: f64 = (((eq9_e229_d_n5 * s.v[30]) - (eq9_e229 * s.dn[30][5])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n6: f64 = (((eq9_e229_d_n6 * s.v[30]) - (eq9_e229 * s.dn[30][6])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n7: f64 = (((eq9_e229_d_n7 * s.v[30]) - (eq9_e229 * s.dn[30][7])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n8: f64 = (((eq9_e229_d_n8 * s.v[30]) - (eq9_e229 * s.dn[30][8])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n9: f64 = (((eq9_e229_d_n9 * s.v[30]) - (eq9_e229 * s.dn[30][9])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n10: f64 = (((eq9_e229_d_n10 * s.v[30]) - (eq9_e229 * s.dn[30][10])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n11: f64 = (((eq9_e229_d_n11 * s.v[30]) - (eq9_e229 * s.dn[30][11])) / (s.v[30] * s.v[30]));
        let eq9_e233: f64 = (eq9_e231 * p.p1);
        let eq9_e233_d_n0: f64 = (eq9_e231_d_n0 * p.p1);
        let eq9_e233_d_n1: f64 = (eq9_e231_d_n1 * p.p1);
        let eq9_e233_d_n2: f64 = (eq9_e231_d_n2 * p.p1);
        let eq9_e233_d_n3: f64 = (eq9_e231_d_n3 * p.p1);
        let eq9_e233_d_n4: f64 = (eq9_e231_d_n4 * p.p1);
        let eq9_e233_d_n5: f64 = (eq9_e231_d_n5 * p.p1);
        let eq9_e233_d_n6: f64 = (eq9_e231_d_n6 * p.p1);
        let eq9_e233_d_n7: f64 = (eq9_e231_d_n7 * p.p1);
        let eq9_e233_d_n8: f64 = (eq9_e231_d_n8 * p.p1);
        let eq9_e233_d_n9: f64 = (eq9_e231_d_n9 * p.p1);
        let eq9_e233_d_n10: f64 = (eq9_e231_d_n10 * p.p1);
        let eq9_e233_d_n11: f64 = (eq9_e231_d_n11 * p.p1);
        let eq9_value: f64 = eq9_e233;
        let eq9_node_derivatives: [f64; 12] = [eq9_e233_d_n0, eq9_e233_d_n1, eq9_e233_d_n2, eq9_e233_d_n3, eq9_e233_d_n4, eq9_e233_d_n5, eq9_e233_d_n6, eq9_e233_d_n7, eq9_e233_d_n8, eq9_e233_d_n9, eq9_e233_d_n10, eq9_e233_d_n11];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
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
        let eq10_value: f64 = s.v[102];
        let eq10_node_derivatives: [f64; 12] = [s.dn[102][0], s.dn[102][1], s.dn[102][2], s.dn[102][3], s.dn[102][4], s.dn[102][5], s.dn[102][6], s.dn[102][7], s.dn[102][8], s.dn[102][9], s.dn[102][10], s.dn[102][11]];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
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
        let eq11_value: f64 = s.v[209];
        let eq11_node_derivatives: [f64; 12] = [s.dn[209][0], s.dn[209][1], s.dn[209][2], s.dn[209][3], s.dn[209][4], s.dn[209][5], s.dn[209][6], s.dn[209][7], s.dn[209][8], s.dn[209][9], s.dn[209][10], s.dn[209][11]];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
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
        let eq12_e237: f64 = (-1.0);
        let eq12_e239: f64 = (eq12_e237 * s.v[208]);
        let eq12_e239_d_n0: f64 = (eq12_e237 * s.dn[208][0]);
        let eq12_e239_d_n1: f64 = (eq12_e237 * s.dn[208][1]);
        let eq12_e239_d_n2: f64 = (eq12_e237 * s.dn[208][2]);
        let eq12_e239_d_n3: f64 = (eq12_e237 * s.dn[208][3]);
        let eq12_e239_d_n4: f64 = (eq12_e237 * s.dn[208][4]);
        let eq12_e239_d_n5: f64 = (eq12_e237 * s.dn[208][5]);
        let eq12_e239_d_n6: f64 = (eq12_e237 * s.dn[208][6]);
        let eq12_e239_d_n7: f64 = (eq12_e237 * s.dn[208][7]);
        let eq12_e239_d_n8: f64 = (eq12_e237 * s.dn[208][8]);
        let eq12_e239_d_n9: f64 = (eq12_e237 * s.dn[208][9]);
        let eq12_e239_d_n10: f64 = (eq12_e237 * s.dn[208][10]);
        let eq12_e239_d_n11: f64 = (eq12_e237 * s.dn[208][11]);
        let eq12_e241: f64 = (eq12_e239 * p.p1);
        let eq12_e241_d_n0: f64 = (eq12_e239_d_n0 * p.p1);
        let eq12_e241_d_n1: f64 = (eq12_e239_d_n1 * p.p1);
        let eq12_e241_d_n2: f64 = (eq12_e239_d_n2 * p.p1);
        let eq12_e241_d_n3: f64 = (eq12_e239_d_n3 * p.p1);
        let eq12_e241_d_n4: f64 = (eq12_e239_d_n4 * p.p1);
        let eq12_e241_d_n5: f64 = (eq12_e239_d_n5 * p.p1);
        let eq12_e241_d_n6: f64 = (eq12_e239_d_n6 * p.p1);
        let eq12_e241_d_n7: f64 = (eq12_e239_d_n7 * p.p1);
        let eq12_e241_d_n8: f64 = (eq12_e239_d_n8 * p.p1);
        let eq12_e241_d_n9: f64 = (eq12_e239_d_n9 * p.p1);
        let eq12_e241_d_n10: f64 = (eq12_e239_d_n10 * p.p1);
        let eq12_e241_d_n11: f64 = (eq12_e239_d_n11 * p.p1);
        let eq12_value: f64 = eq12_e241;
        let eq12_node_derivatives: [f64; 12] = [eq12_e241_d_n0, eq12_e241_d_n1, eq12_e241_d_n2, eq12_e241_d_n3, eq12_e241_d_n4, eq12_e241_d_n5, eq12_e241_d_n6, eq12_e241_d_n7, eq12_e241_d_n8, eq12_e241_d_n9, eq12_e241_d_n10, eq12_e241_d_n11];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
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
        let eq13_e249: f64 = self.eval_ddt(1, eq13_e248);
        let eq13_e249_d_n0: f64 = self.ddt_jacobian(eq13_e248_d_n0);
        let eq13_e249_d_n1: f64 = self.ddt_jacobian(eq13_e248_d_n1);
        let eq13_e249_d_n2: f64 = self.ddt_jacobian(eq13_e248_d_n2);
        let eq13_e249_d_n3: f64 = self.ddt_jacobian(eq13_e248_d_n3);
        let eq13_e249_d_n4: f64 = self.ddt_jacobian(eq13_e248_d_n4);
        let eq13_e249_d_n5: f64 = self.ddt_jacobian(eq13_e248_d_n5);
        let eq13_e249_d_n6: f64 = self.ddt_jacobian(eq13_e248_d_n6);
        let eq13_e249_d_n7: f64 = self.ddt_jacobian(eq13_e248_d_n7);
        let eq13_e249_d_n8: f64 = self.ddt_jacobian(eq13_e248_d_n8);
        let eq13_e249_d_n9: f64 = self.ddt_jacobian(eq13_e248_d_n9);
        let eq13_e249_d_n10: f64 = self.ddt_jacobian(eq13_e248_d_n10);
        let eq13_e249_d_n11: f64 = self.ddt_jacobian(eq13_e248_d_n11);
        let eq13_e251: f64 = (eq13_e249 * p.p1);
        let eq13_e251_d_n0: f64 = (eq13_e249_d_n0 * p.p1);
        let eq13_e251_d_n1: f64 = (eq13_e249_d_n1 * p.p1);
        let eq13_e251_d_n2: f64 = (eq13_e249_d_n2 * p.p1);
        let eq13_e251_d_n3: f64 = (eq13_e249_d_n3 * p.p1);
        let eq13_e251_d_n4: f64 = (eq13_e249_d_n4 * p.p1);
        let eq13_e251_d_n5: f64 = (eq13_e249_d_n5 * p.p1);
        let eq13_e251_d_n6: f64 = (eq13_e249_d_n6 * p.p1);
        let eq13_e251_d_n7: f64 = (eq13_e249_d_n7 * p.p1);
        let eq13_e251_d_n8: f64 = (eq13_e249_d_n8 * p.p1);
        let eq13_e251_d_n9: f64 = (eq13_e249_d_n9 * p.p1);
        let eq13_e251_d_n10: f64 = (eq13_e249_d_n10 * p.p1);
        let eq13_e251_d_n11: f64 = (eq13_e249_d_n11 * p.p1);
        let eq13_value: f64 = eq13_e251;
        let eq13_node_derivatives: [f64; 12] = [eq13_e251_d_n0, eq13_e251_d_n1, eq13_e251_d_n2, eq13_e251_d_n3, eq13_e251_d_n4, eq13_e251_d_n5, eq13_e251_d_n6, eq13_e251_d_n7, eq13_e251_d_n8, eq13_e251_d_n9, eq13_e251_d_n10, eq13_e251_d_n11];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[4]),
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
        let eq14_e255: f64 = self.eval_ddt(2, eq14_e254);
        let eq14_e255_d_n0: f64 = self.ddt_jacobian(eq14_e254_d_n0);
        let eq14_e255_d_n1: f64 = self.ddt_jacobian(eq14_e254_d_n1);
        let eq14_e255_d_n2: f64 = self.ddt_jacobian(eq14_e254_d_n2);
        let eq14_e255_d_n3: f64 = self.ddt_jacobian(eq14_e254_d_n3);
        let eq14_e255_d_n4: f64 = self.ddt_jacobian(eq14_e254_d_n4);
        let eq14_e255_d_n5: f64 = self.ddt_jacobian(eq14_e254_d_n5);
        let eq14_e255_d_n6: f64 = self.ddt_jacobian(eq14_e254_d_n6);
        let eq14_e255_d_n7: f64 = self.ddt_jacobian(eq14_e254_d_n7);
        let eq14_e255_d_n8: f64 = self.ddt_jacobian(eq14_e254_d_n8);
        let eq14_e255_d_n9: f64 = self.ddt_jacobian(eq14_e254_d_n9);
        let eq14_e255_d_n10: f64 = self.ddt_jacobian(eq14_e254_d_n10);
        let eq14_e255_d_n11: f64 = self.ddt_jacobian(eq14_e254_d_n11);
        let eq14_e257: f64 = (eq14_e255 * p.p1);
        let eq14_e257_d_n0: f64 = (eq14_e255_d_n0 * p.p1);
        let eq14_e257_d_n1: f64 = (eq14_e255_d_n1 * p.p1);
        let eq14_e257_d_n2: f64 = (eq14_e255_d_n2 * p.p1);
        let eq14_e257_d_n3: f64 = (eq14_e255_d_n3 * p.p1);
        let eq14_e257_d_n4: f64 = (eq14_e255_d_n4 * p.p1);
        let eq14_e257_d_n5: f64 = (eq14_e255_d_n5 * p.p1);
        let eq14_e257_d_n6: f64 = (eq14_e255_d_n6 * p.p1);
        let eq14_e257_d_n7: f64 = (eq14_e255_d_n7 * p.p1);
        let eq14_e257_d_n8: f64 = (eq14_e255_d_n8 * p.p1);
        let eq14_e257_d_n9: f64 = (eq14_e255_d_n9 * p.p1);
        let eq14_e257_d_n10: f64 = (eq14_e255_d_n10 * p.p1);
        let eq14_e257_d_n11: f64 = (eq14_e255_d_n11 * p.p1);
        let eq14_value: f64 = eq14_e257;
        let eq14_node_derivatives: [f64; 12] = [eq14_e257_d_n0, eq14_e257_d_n1, eq14_e257_d_n2, eq14_e257_d_n3, eq14_e257_d_n4, eq14_e257_d_n5, eq14_e257_d_n6, eq14_e257_d_n7, eq14_e257_d_n8, eq14_e257_d_n9, eq14_e257_d_n10, eq14_e257_d_n11];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let eq15_e265: f64 = self.eval_ddt(3, eq15_e264);
        let eq15_e265_d_n0: f64 = self.ddt_jacobian(eq15_e264_d_n0);
        let eq15_e265_d_n1: f64 = self.ddt_jacobian(eq15_e264_d_n1);
        let eq15_e265_d_n2: f64 = self.ddt_jacobian(eq15_e264_d_n2);
        let eq15_e265_d_n3: f64 = self.ddt_jacobian(eq15_e264_d_n3);
        let eq15_e265_d_n4: f64 = self.ddt_jacobian(eq15_e264_d_n4);
        let eq15_e265_d_n5: f64 = self.ddt_jacobian(eq15_e264_d_n5);
        let eq15_e265_d_n6: f64 = self.ddt_jacobian(eq15_e264_d_n6);
        let eq15_e265_d_n7: f64 = self.ddt_jacobian(eq15_e264_d_n7);
        let eq15_e265_d_n8: f64 = self.ddt_jacobian(eq15_e264_d_n8);
        let eq15_e265_d_n9: f64 = self.ddt_jacobian(eq15_e264_d_n9);
        let eq15_e265_d_n10: f64 = self.ddt_jacobian(eq15_e264_d_n10);
        let eq15_e265_d_n11: f64 = self.ddt_jacobian(eq15_e264_d_n11);
        let eq15_e267: f64 = (eq15_e265 * p.p1);
        let eq15_e267_d_n0: f64 = (eq15_e265_d_n0 * p.p1);
        let eq15_e267_d_n1: f64 = (eq15_e265_d_n1 * p.p1);
        let eq15_e267_d_n2: f64 = (eq15_e265_d_n2 * p.p1);
        let eq15_e267_d_n3: f64 = (eq15_e265_d_n3 * p.p1);
        let eq15_e267_d_n4: f64 = (eq15_e265_d_n4 * p.p1);
        let eq15_e267_d_n5: f64 = (eq15_e265_d_n5 * p.p1);
        let eq15_e267_d_n6: f64 = (eq15_e265_d_n6 * p.p1);
        let eq15_e267_d_n7: f64 = (eq15_e265_d_n7 * p.p1);
        let eq15_e267_d_n8: f64 = (eq15_e265_d_n8 * p.p1);
        let eq15_e267_d_n9: f64 = (eq15_e265_d_n9 * p.p1);
        let eq15_e267_d_n10: f64 = (eq15_e265_d_n10 * p.p1);
        let eq15_e267_d_n11: f64 = (eq15_e265_d_n11 * p.p1);
        let eq15_value: f64 = eq15_e267;
        let eq15_node_derivatives: [f64; 12] = [eq15_e267_d_n0, eq15_e267_d_n1, eq15_e267_d_n2, eq15_e267_d_n3, eq15_e267_d_n4, eq15_e267_d_n5, eq15_e267_d_n6, eq15_e267_d_n7, eq15_e267_d_n8, eq15_e267_d_n9, eq15_e267_d_n10, eq15_e267_d_n11];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
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
        let eq16_e271: f64 = self.eval_ddt(4, eq16_e270);
        let eq16_e271_d_n0: f64 = self.ddt_jacobian(eq16_e270_d_n0);
        let eq16_e271_d_n1: f64 = self.ddt_jacobian(eq16_e270_d_n1);
        let eq16_e271_d_n2: f64 = self.ddt_jacobian(eq16_e270_d_n2);
        let eq16_e271_d_n3: f64 = self.ddt_jacobian(eq16_e270_d_n3);
        let eq16_e271_d_n4: f64 = self.ddt_jacobian(eq16_e270_d_n4);
        let eq16_e271_d_n5: f64 = self.ddt_jacobian(eq16_e270_d_n5);
        let eq16_e271_d_n6: f64 = self.ddt_jacobian(eq16_e270_d_n6);
        let eq16_e271_d_n7: f64 = self.ddt_jacobian(eq16_e270_d_n7);
        let eq16_e271_d_n8: f64 = self.ddt_jacobian(eq16_e270_d_n8);
        let eq16_e271_d_n9: f64 = self.ddt_jacobian(eq16_e270_d_n9);
        let eq16_e271_d_n10: f64 = self.ddt_jacobian(eq16_e270_d_n10);
        let eq16_e271_d_n11: f64 = self.ddt_jacobian(eq16_e270_d_n11);
        let eq16_e273: f64 = (eq16_e271 * p.p1);
        let eq16_e273_d_n0: f64 = (eq16_e271_d_n0 * p.p1);
        let eq16_e273_d_n1: f64 = (eq16_e271_d_n1 * p.p1);
        let eq16_e273_d_n2: f64 = (eq16_e271_d_n2 * p.p1);
        let eq16_e273_d_n3: f64 = (eq16_e271_d_n3 * p.p1);
        let eq16_e273_d_n4: f64 = (eq16_e271_d_n4 * p.p1);
        let eq16_e273_d_n5: f64 = (eq16_e271_d_n5 * p.p1);
        let eq16_e273_d_n6: f64 = (eq16_e271_d_n6 * p.p1);
        let eq16_e273_d_n7: f64 = (eq16_e271_d_n7 * p.p1);
        let eq16_e273_d_n8: f64 = (eq16_e271_d_n8 * p.p1);
        let eq16_e273_d_n9: f64 = (eq16_e271_d_n9 * p.p1);
        let eq16_e273_d_n10: f64 = (eq16_e271_d_n10 * p.p1);
        let eq16_e273_d_n11: f64 = (eq16_e271_d_n11 * p.p1);
        let eq16_value: f64 = eq16_e273;
        let eq16_node_derivatives: [f64; 12] = [eq16_e273_d_n0, eq16_e273_d_n1, eq16_e273_d_n2, eq16_e273_d_n3, eq16_e273_d_n4, eq16_e273_d_n5, eq16_e273_d_n6, eq16_e273_d_n7, eq16_e273_d_n8, eq16_e273_d_n9, eq16_e273_d_n10, eq16_e273_d_n11];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let eq17_e279: f64 = self.eval_ddt(5, eq17_e278);
        let eq17_e279_d_n0: f64 = self.ddt_jacobian(eq17_e278_d_n0);
        let eq17_e279_d_n1: f64 = self.ddt_jacobian(eq17_e278_d_n1);
        let eq17_e279_d_n2: f64 = self.ddt_jacobian(eq17_e278_d_n2);
        let eq17_e279_d_n3: f64 = self.ddt_jacobian(eq17_e278_d_n3);
        let eq17_e279_d_n4: f64 = self.ddt_jacobian(eq17_e278_d_n4);
        let eq17_e279_d_n5: f64 = self.ddt_jacobian(eq17_e278_d_n5);
        let eq17_e279_d_n6: f64 = self.ddt_jacobian(eq17_e278_d_n6);
        let eq17_e279_d_n7: f64 = self.ddt_jacobian(eq17_e278_d_n7);
        let eq17_e279_d_n8: f64 = self.ddt_jacobian(eq17_e278_d_n8);
        let eq17_e279_d_n9: f64 = self.ddt_jacobian(eq17_e278_d_n9);
        let eq17_e279_d_n10: f64 = self.ddt_jacobian(eq17_e278_d_n10);
        let eq17_e279_d_n11: f64 = self.ddt_jacobian(eq17_e278_d_n11);
        let eq17_e281: f64 = (eq17_e279 * p.p1);
        let eq17_e281_d_n0: f64 = (eq17_e279_d_n0 * p.p1);
        let eq17_e281_d_n1: f64 = (eq17_e279_d_n1 * p.p1);
        let eq17_e281_d_n2: f64 = (eq17_e279_d_n2 * p.p1);
        let eq17_e281_d_n3: f64 = (eq17_e279_d_n3 * p.p1);
        let eq17_e281_d_n4: f64 = (eq17_e279_d_n4 * p.p1);
        let eq17_e281_d_n5: f64 = (eq17_e279_d_n5 * p.p1);
        let eq17_e281_d_n6: f64 = (eq17_e279_d_n6 * p.p1);
        let eq17_e281_d_n7: f64 = (eq17_e279_d_n7 * p.p1);
        let eq17_e281_d_n8: f64 = (eq17_e279_d_n8 * p.p1);
        let eq17_e281_d_n9: f64 = (eq17_e279_d_n9 * p.p1);
        let eq17_e281_d_n10: f64 = (eq17_e279_d_n10 * p.p1);
        let eq17_e281_d_n11: f64 = (eq17_e279_d_n11 * p.p1);
        let eq17_value: f64 = eq17_e281;
        let eq17_node_derivatives: [f64; 12] = [eq17_e281_d_n0, eq17_e281_d_n1, eq17_e281_d_n2, eq17_e281_d_n3, eq17_e281_d_n4, eq17_e281_d_n5, eq17_e281_d_n6, eq17_e281_d_n7, eq17_e281_d_n8, eq17_e281_d_n9, eq17_e281_d_n10, eq17_e281_d_n11];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[2]),
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
        let eq18_e287: f64 = self.eval_ddt(6, eq18_e286);
        let eq18_e287_d_n0: f64 = self.ddt_jacobian(eq18_e286_d_n0);
        let eq18_e287_d_n1: f64 = self.ddt_jacobian(eq18_e286_d_n1);
        let eq18_e287_d_n2: f64 = self.ddt_jacobian(eq18_e286_d_n2);
        let eq18_e287_d_n3: f64 = self.ddt_jacobian(eq18_e286_d_n3);
        let eq18_e287_d_n4: f64 = self.ddt_jacobian(eq18_e286_d_n4);
        let eq18_e287_d_n5: f64 = self.ddt_jacobian(eq18_e286_d_n5);
        let eq18_e287_d_n6: f64 = self.ddt_jacobian(eq18_e286_d_n6);
        let eq18_e287_d_n7: f64 = self.ddt_jacobian(eq18_e286_d_n7);
        let eq18_e287_d_n8: f64 = self.ddt_jacobian(eq18_e286_d_n8);
        let eq18_e287_d_n9: f64 = self.ddt_jacobian(eq18_e286_d_n9);
        let eq18_e287_d_n10: f64 = self.ddt_jacobian(eq18_e286_d_n10);
        let eq18_e287_d_n11: f64 = self.ddt_jacobian(eq18_e286_d_n11);
        let eq18_e289: f64 = (eq18_e287 * p.p1);
        let eq18_e289_d_n0: f64 = (eq18_e287_d_n0 * p.p1);
        let eq18_e289_d_n1: f64 = (eq18_e287_d_n1 * p.p1);
        let eq18_e289_d_n2: f64 = (eq18_e287_d_n2 * p.p1);
        let eq18_e289_d_n3: f64 = (eq18_e287_d_n3 * p.p1);
        let eq18_e289_d_n4: f64 = (eq18_e287_d_n4 * p.p1);
        let eq18_e289_d_n5: f64 = (eq18_e287_d_n5 * p.p1);
        let eq18_e289_d_n6: f64 = (eq18_e287_d_n6 * p.p1);
        let eq18_e289_d_n7: f64 = (eq18_e287_d_n7 * p.p1);
        let eq18_e289_d_n8: f64 = (eq18_e287_d_n8 * p.p1);
        let eq18_e289_d_n9: f64 = (eq18_e287_d_n9 * p.p1);
        let eq18_e289_d_n10: f64 = (eq18_e287_d_n10 * p.p1);
        let eq18_e289_d_n11: f64 = (eq18_e287_d_n11 * p.p1);
        let eq18_value: f64 = eq18_e289;
        let eq18_node_derivatives: [f64; 12] = [eq18_e289_d_n0, eq18_e289_d_n1, eq18_e289_d_n2, eq18_e289_d_n3, eq18_e289_d_n4, eq18_e289_d_n5, eq18_e289_d_n6, eq18_e289_d_n7, eq18_e289_d_n8, eq18_e289_d_n9, eq18_e289_d_n10, eq18_e289_d_n11];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[0]),
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
        let eq19_e292: f64 = (p.p3 * s.v[172]);
        let eq19_e292_d_n0: f64 = (p.p3 * s.dn[172][0]);
        let eq19_e292_d_n1: f64 = (p.p3 * s.dn[172][1]);
        let eq19_e292_d_n2: f64 = (p.p3 * s.dn[172][2]);
        let eq19_e292_d_n3: f64 = (p.p3 * s.dn[172][3]);
        let eq19_e292_d_n4: f64 = (p.p3 * s.dn[172][4]);
        let eq19_e292_d_n5: f64 = (p.p3 * s.dn[172][5]);
        let eq19_e292_d_n6: f64 = (p.p3 * s.dn[172][6]);
        let eq19_e292_d_n7: f64 = (p.p3 * s.dn[172][7]);
        let eq19_e292_d_n8: f64 = (p.p3 * s.dn[172][8]);
        let eq19_e292_d_n9: f64 = (p.p3 * s.dn[172][9]);
        let eq19_e292_d_n10: f64 = (p.p3 * s.dn[172][10]);
        let eq19_e292_d_n11: f64 = (p.p3 * s.dn[172][11]);
        let eq19_e294: f64 = (eq19_e292 * p.p1);
        let eq19_e294_d_n0: f64 = (eq19_e292_d_n0 * p.p1);
        let eq19_e294_d_n1: f64 = (eq19_e292_d_n1 * p.p1);
        let eq19_e294_d_n2: f64 = (eq19_e292_d_n2 * p.p1);
        let eq19_e294_d_n3: f64 = (eq19_e292_d_n3 * p.p1);
        let eq19_e294_d_n4: f64 = (eq19_e292_d_n4 * p.p1);
        let eq19_e294_d_n5: f64 = (eq19_e292_d_n5 * p.p1);
        let eq19_e294_d_n6: f64 = (eq19_e292_d_n6 * p.p1);
        let eq19_e294_d_n7: f64 = (eq19_e292_d_n7 * p.p1);
        let eq19_e294_d_n8: f64 = (eq19_e292_d_n8 * p.p1);
        let eq19_e294_d_n9: f64 = (eq19_e292_d_n9 * p.p1);
        let eq19_e294_d_n10: f64 = (eq19_e292_d_n10 * p.p1);
        let eq19_e294_d_n11: f64 = (eq19_e292_d_n11 * p.p1);
        let eq19_value: f64 = eq19_e294;
        let eq19_node_derivatives: [f64; 12] = [eq19_e294_d_n0, eq19_e294_d_n1, eq19_e294_d_n2, eq19_e294_d_n3, eq19_e294_d_n4, eq19_e294_d_n5, eq19_e294_d_n6, eq19_e294_d_n7, eq19_e294_d_n8, eq19_e294_d_n9, eq19_e294_d_n10, eq19_e294_d_n11];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
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
        let eq20_e297: f64 = (p.p3 * s.v[248]);
        let eq20_e297_d_n0: f64 = (p.p3 * s.dn[248][0]);
        let eq20_e297_d_n1: f64 = (p.p3 * s.dn[248][1]);
        let eq20_e297_d_n2: f64 = (p.p3 * s.dn[248][2]);
        let eq20_e297_d_n3: f64 = (p.p3 * s.dn[248][3]);
        let eq20_e297_d_n4: f64 = (p.p3 * s.dn[248][4]);
        let eq20_e297_d_n5: f64 = (p.p3 * s.dn[248][5]);
        let eq20_e297_d_n6: f64 = (p.p3 * s.dn[248][6]);
        let eq20_e297_d_n7: f64 = (p.p3 * s.dn[248][7]);
        let eq20_e297_d_n8: f64 = (p.p3 * s.dn[248][8]);
        let eq20_e297_d_n9: f64 = (p.p3 * s.dn[248][9]);
        let eq20_e297_d_n10: f64 = (p.p3 * s.dn[248][10]);
        let eq20_e297_d_n11: f64 = (p.p3 * s.dn[248][11]);
        let eq20_e299: f64 = (eq20_e297 * s.v[104]);
        let eq20_e299_d_n0: f64 = ((eq20_e297_d_n0 * s.v[104]) + (eq20_e297 * s.dn[104][0]));
        let eq20_e299_d_n1: f64 = ((eq20_e297_d_n1 * s.v[104]) + (eq20_e297 * s.dn[104][1]));
        let eq20_e299_d_n2: f64 = ((eq20_e297_d_n2 * s.v[104]) + (eq20_e297 * s.dn[104][2]));
        let eq20_e299_d_n3: f64 = ((eq20_e297_d_n3 * s.v[104]) + (eq20_e297 * s.dn[104][3]));
        let eq20_e299_d_n4: f64 = ((eq20_e297_d_n4 * s.v[104]) + (eq20_e297 * s.dn[104][4]));
        let eq20_e299_d_n5: f64 = ((eq20_e297_d_n5 * s.v[104]) + (eq20_e297 * s.dn[104][5]));
        let eq20_e299_d_n6: f64 = ((eq20_e297_d_n6 * s.v[104]) + (eq20_e297 * s.dn[104][6]));
        let eq20_e299_d_n7: f64 = ((eq20_e297_d_n7 * s.v[104]) + (eq20_e297 * s.dn[104][7]));
        let eq20_e299_d_n8: f64 = ((eq20_e297_d_n8 * s.v[104]) + (eq20_e297 * s.dn[104][8]));
        let eq20_e299_d_n9: f64 = ((eq20_e297_d_n9 * s.v[104]) + (eq20_e297 * s.dn[104][9]));
        let eq20_e299_d_n10: f64 = ((eq20_e297_d_n10 * s.v[104]) + (eq20_e297 * s.dn[104][10]));
        let eq20_e299_d_n11: f64 = ((eq20_e297_d_n11 * s.v[104]) + (eq20_e297 * s.dn[104][11]));
        let eq20_e301: f64 = (eq20_e299 * p.p1);
        let eq20_e301_d_n0: f64 = (eq20_e299_d_n0 * p.p1);
        let eq20_e301_d_n1: f64 = (eq20_e299_d_n1 * p.p1);
        let eq20_e301_d_n2: f64 = (eq20_e299_d_n2 * p.p1);
        let eq20_e301_d_n3: f64 = (eq20_e299_d_n3 * p.p1);
        let eq20_e301_d_n4: f64 = (eq20_e299_d_n4 * p.p1);
        let eq20_e301_d_n5: f64 = (eq20_e299_d_n5 * p.p1);
        let eq20_e301_d_n6: f64 = (eq20_e299_d_n6 * p.p1);
        let eq20_e301_d_n7: f64 = (eq20_e299_d_n7 * p.p1);
        let eq20_e301_d_n8: f64 = (eq20_e299_d_n8 * p.p1);
        let eq20_e301_d_n9: f64 = (eq20_e299_d_n9 * p.p1);
        let eq20_e301_d_n10: f64 = (eq20_e299_d_n10 * p.p1);
        let eq20_e301_d_n11: f64 = (eq20_e299_d_n11 * p.p1);
        let eq20_value: f64 = eq20_e301;
        let eq20_node_derivatives: [f64; 12] = [eq20_e301_d_n0, eq20_e301_d_n1, eq20_e301_d_n2, eq20_e301_d_n3, eq20_e301_d_n4, eq20_e301_d_n5, eq20_e301_d_n6, eq20_e301_d_n7, eq20_e301_d_n8, eq20_e301_d_n9, eq20_e301_d_n10, eq20_e301_d_n11];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[9]),
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
        let eq21_e307: f64 = self.eval_ddt(7, eq21_e306);
        let eq21_e307_d_n0: f64 = self.ddt_jacobian(eq21_e306_d_n0);
        let eq21_e307_d_n1: f64 = self.ddt_jacobian(eq21_e306_d_n1);
        let eq21_e307_d_n2: f64 = self.ddt_jacobian(eq21_e306_d_n2);
        let eq21_e307_d_n3: f64 = self.ddt_jacobian(eq21_e306_d_n3);
        let eq21_e307_d_n4: f64 = self.ddt_jacobian(eq21_e306_d_n4);
        let eq21_e307_d_n5: f64 = self.ddt_jacobian(eq21_e306_d_n5);
        let eq21_e307_d_n6: f64 = self.ddt_jacobian(eq21_e306_d_n6);
        let eq21_e307_d_n7: f64 = self.ddt_jacobian(eq21_e306_d_n7);
        let eq21_e307_d_n8: f64 = self.ddt_jacobian(eq21_e306_d_n8);
        let eq21_e307_d_n9: f64 = self.ddt_jacobian(eq21_e306_d_n9);
        let eq21_e307_d_n10: f64 = self.ddt_jacobian(eq21_e306_d_n10);
        let eq21_e307_d_n11: f64 = self.ddt_jacobian(eq21_e306_d_n11);
        let eq21_e309: f64 = (eq21_e307 * p.p1);
        let eq21_e309_d_n0: f64 = (eq21_e307_d_n0 * p.p1);
        let eq21_e309_d_n1: f64 = (eq21_e307_d_n1 * p.p1);
        let eq21_e309_d_n2: f64 = (eq21_e307_d_n2 * p.p1);
        let eq21_e309_d_n3: f64 = (eq21_e307_d_n3 * p.p1);
        let eq21_e309_d_n4: f64 = (eq21_e307_d_n4 * p.p1);
        let eq21_e309_d_n5: f64 = (eq21_e307_d_n5 * p.p1);
        let eq21_e309_d_n6: f64 = (eq21_e307_d_n6 * p.p1);
        let eq21_e309_d_n7: f64 = (eq21_e307_d_n7 * p.p1);
        let eq21_e309_d_n8: f64 = (eq21_e307_d_n8 * p.p1);
        let eq21_e309_d_n9: f64 = (eq21_e307_d_n9 * p.p1);
        let eq21_e309_d_n10: f64 = (eq21_e307_d_n10 * p.p1);
        let eq21_e309_d_n11: f64 = (eq21_e307_d_n11 * p.p1);
        let eq21_value: f64 = eq21_e309;
        let eq21_node_derivatives: [f64; 12] = [eq21_e309_d_n0, eq21_e309_d_n1, eq21_e309_d_n2, eq21_e309_d_n3, eq21_e309_d_n4, eq21_e309_d_n5, eq21_e309_d_n6, eq21_e309_d_n7, eq21_e309_d_n8, eq21_e309_d_n9, eq21_e309_d_n10, eq21_e309_d_n11];
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
        let eq22_e314: f64 = (s.v[320] * s.v[241]);
        let eq22_e314_d_n0: f64 = (s.v[320] * s.dn[241][0]);
        let eq22_e314_d_n1: f64 = (s.v[320] * s.dn[241][1]);
        let eq22_e314_d_n2: f64 = (s.v[320] * s.dn[241][2]);
        let eq22_e314_d_n3: f64 = (s.v[320] * s.dn[241][3]);
        let eq22_e314_d_n4: f64 = (s.v[320] * s.dn[241][4]);
        let eq22_e314_d_n5: f64 = (s.v[320] * s.dn[241][5]);
        let eq22_e314_d_n6: f64 = (s.v[320] * s.dn[241][6]);
        let eq22_e314_d_n7: f64 = (s.v[320] * s.dn[241][7]);
        let eq22_e314_d_n8: f64 = (s.v[320] * s.dn[241][8]);
        let eq22_e314_d_n9: f64 = (s.v[320] * s.dn[241][9]);
        let eq22_e314_d_n10: f64 = (s.v[320] * s.dn[241][10]);
        let eq22_e314_d_n11: f64 = (s.v[320] * s.dn[241][11]);
        let eq22_e315: f64 = (s.v[157] + eq22_e314);
        let eq22_e315_d_n0: f64 = (s.dn[157][0] + eq22_e314_d_n0);
        let eq22_e315_d_n1: f64 = (s.dn[157][1] + eq22_e314_d_n1);
        let eq22_e315_d_n2: f64 = (s.dn[157][2] + eq22_e314_d_n2);
        let eq22_e315_d_n3: f64 = (s.dn[157][3] + eq22_e314_d_n3);
        let eq22_e315_d_n4: f64 = (s.dn[157][4] + eq22_e314_d_n4);
        let eq22_e315_d_n5: f64 = (s.dn[157][5] + eq22_e314_d_n5);
        let eq22_e315_d_n6: f64 = (s.dn[157][6] + eq22_e314_d_n6);
        let eq22_e315_d_n7: f64 = (s.dn[157][7] + eq22_e314_d_n7);
        let eq22_e315_d_n8: f64 = (s.dn[157][8] + eq22_e314_d_n8);
        let eq22_e315_d_n9: f64 = (s.dn[157][9] + eq22_e314_d_n9);
        let eq22_e315_d_n10: f64 = (s.dn[157][10] + eq22_e314_d_n10);
        let eq22_e315_d_n11: f64 = (s.dn[157][11] + eq22_e314_d_n11);
        let eq22_e317: f64 = (eq22_e315 + s.v[160]);
        let eq22_e317_d_n0: f64 = (eq22_e315_d_n0 + s.dn[160][0]);
        let eq22_e317_d_n1: f64 = (eq22_e315_d_n1 + s.dn[160][1]);
        let eq22_e317_d_n2: f64 = (eq22_e315_d_n2 + s.dn[160][2]);
        let eq22_e317_d_n3: f64 = (eq22_e315_d_n3 + s.dn[160][3]);
        let eq22_e317_d_n4: f64 = (eq22_e315_d_n4 + s.dn[160][4]);
        let eq22_e317_d_n5: f64 = (eq22_e315_d_n5 + s.dn[160][5]);
        let eq22_e317_d_n6: f64 = (eq22_e315_d_n6 + s.dn[160][6]);
        let eq22_e317_d_n7: f64 = (eq22_e315_d_n7 + s.dn[160][7]);
        let eq22_e317_d_n8: f64 = (eq22_e315_d_n8 + s.dn[160][8]);
        let eq22_e317_d_n9: f64 = (eq22_e315_d_n9 + s.dn[160][9]);
        let eq22_e317_d_n10: f64 = (eq22_e315_d_n10 + s.dn[160][10]);
        let eq22_e317_d_n11: f64 = (eq22_e315_d_n11 + s.dn[160][11]);
        let eq22_e318: f64 = (p.p3 * eq22_e317);
        let eq22_e318_d_n0: f64 = (p.p3 * eq22_e317_d_n0);
        let eq22_e318_d_n1: f64 = (p.p3 * eq22_e317_d_n1);
        let eq22_e318_d_n2: f64 = (p.p3 * eq22_e317_d_n2);
        let eq22_e318_d_n3: f64 = (p.p3 * eq22_e317_d_n3);
        let eq22_e318_d_n4: f64 = (p.p3 * eq22_e317_d_n4);
        let eq22_e318_d_n5: f64 = (p.p3 * eq22_e317_d_n5);
        let eq22_e318_d_n6: f64 = (p.p3 * eq22_e317_d_n6);
        let eq22_e318_d_n7: f64 = (p.p3 * eq22_e317_d_n7);
        let eq22_e318_d_n8: f64 = (p.p3 * eq22_e317_d_n8);
        let eq22_e318_d_n9: f64 = (p.p3 * eq22_e317_d_n9);
        let eq22_e318_d_n10: f64 = (p.p3 * eq22_e317_d_n10);
        let eq22_e318_d_n11: f64 = (p.p3 * eq22_e317_d_n11);
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
        let eq22_value: f64 = eq22_e320;
        let eq22_node_derivatives: [f64; 12] = [eq22_e320_d_n0, eq22_e320_d_n1, eq22_e320_d_n2, eq22_e320_d_n3, eq22_e320_d_n4, eq22_e320_d_n5, eq22_e320_d_n6, eq22_e320_d_n7, eq22_e320_d_n8, eq22_e320_d_n9, eq22_e320_d_n10, eq22_e320_d_n11];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            self.multiplicity * (eq22_value),
            &nodes,
            &eq22_node_derivatives,
            &branches,
            &eq22_branch_derivatives,
            self.multiplicity,
        );
    }
}
