#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq8_e218: f64 = (p.p3 * s.v[239]);
        let eq8_e218_d_n0: f64 = (p.p3 * s.dn[239][0]);
        let eq8_e218_d_n1: f64 = (p.p3 * s.dn[239][1]);
        let eq8_e218_d_n2: f64 = (p.p3 * s.dn[239][2]);
        let eq8_e218_d_n3: f64 = (p.p3 * s.dn[239][3]);
        let eq8_e218_d_n4: f64 = (p.p3 * s.dn[239][4]);
        let eq8_e218_d_n5: f64 = (p.p3 * s.dn[239][5]);
        let eq8_e218_d_n6: f64 = (p.p3 * s.dn[239][6]);
        let eq8_e218_d_n7: f64 = (p.p3 * s.dn[239][7]);
        let eq8_e218_d_n8: f64 = (p.p3 * s.dn[239][8]);
        let eq8_e218_d_n9: f64 = (p.p3 * s.dn[239][9]);
        let eq8_e218_d_n10: f64 = (p.p3 * s.dn[239][10]);
        let eq8_e220: f64 = (eq8_e218 / s.v[28]);
        let eq8_e220_d_n0: f64 = (((eq8_e218_d_n0 * s.v[28]) - (eq8_e218 * s.dn[28][0])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n1: f64 = (((eq8_e218_d_n1 * s.v[28]) - (eq8_e218 * s.dn[28][1])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n2: f64 = (((eq8_e218_d_n2 * s.v[28]) - (eq8_e218 * s.dn[28][2])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n3: f64 = (((eq8_e218_d_n3 * s.v[28]) - (eq8_e218 * s.dn[28][3])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n4: f64 = (((eq8_e218_d_n4 * s.v[28]) - (eq8_e218 * s.dn[28][4])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n5: f64 = (((eq8_e218_d_n5 * s.v[28]) - (eq8_e218 * s.dn[28][5])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n6: f64 = (((eq8_e218_d_n6 * s.v[28]) - (eq8_e218 * s.dn[28][6])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n7: f64 = (((eq8_e218_d_n7 * s.v[28]) - (eq8_e218 * s.dn[28][7])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n8: f64 = (((eq8_e218_d_n8 * s.v[28]) - (eq8_e218 * s.dn[28][8])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n9: f64 = (((eq8_e218_d_n9 * s.v[28]) - (eq8_e218 * s.dn[28][9])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n10: f64 = (((eq8_e218_d_n10 * s.v[28]) - (eq8_e218 * s.dn[28][10])) / (s.v[28] * s.v[28]));
        let eq8_e222: f64 = (eq8_e220 * p.p1);
        let eq8_e222_d_n0: f64 = (eq8_e220_d_n0 * p.p1);
        let eq8_e222_d_n1: f64 = (eq8_e220_d_n1 * p.p1);
        let eq8_e222_d_n2: f64 = (eq8_e220_d_n2 * p.p1);
        let eq8_e222_d_n3: f64 = (eq8_e220_d_n3 * p.p1);
        let eq8_e222_d_n4: f64 = (eq8_e220_d_n4 * p.p1);
        let eq8_e222_d_n5: f64 = (eq8_e220_d_n5 * p.p1);
        let eq8_e222_d_n6: f64 = (eq8_e220_d_n6 * p.p1);
        let eq8_e222_d_n7: f64 = (eq8_e220_d_n7 * p.p1);
        let eq8_e222_d_n8: f64 = (eq8_e220_d_n8 * p.p1);
        let eq8_e222_d_n9: f64 = (eq8_e220_d_n9 * p.p1);
        let eq8_e222_d_n10: f64 = (eq8_e220_d_n10 * p.p1);
        let eq8_value: f64 = eq8_e222;
        let eq8_node_derivatives: [f64; 11] = [eq8_e222_d_n0, eq8_e222_d_n1, eq8_e222_d_n2, eq8_e222_d_n3, eq8_e222_d_n4, eq8_e222_d_n5, eq8_e222_d_n6, eq8_e222_d_n7, eq8_e222_d_n8, eq8_e222_d_n9, eq8_e222_d_n10];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
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
        let eq9_e225: f64 = (p.p3 * s.v[240]);
        let eq9_e225_d_n0: f64 = (p.p3 * s.dn[240][0]);
        let eq9_e225_d_n1: f64 = (p.p3 * s.dn[240][1]);
        let eq9_e225_d_n2: f64 = (p.p3 * s.dn[240][2]);
        let eq9_e225_d_n3: f64 = (p.p3 * s.dn[240][3]);
        let eq9_e225_d_n4: f64 = (p.p3 * s.dn[240][4]);
        let eq9_e225_d_n5: f64 = (p.p3 * s.dn[240][5]);
        let eq9_e225_d_n6: f64 = (p.p3 * s.dn[240][6]);
        let eq9_e225_d_n7: f64 = (p.p3 * s.dn[240][7]);
        let eq9_e225_d_n8: f64 = (p.p3 * s.dn[240][8]);
        let eq9_e225_d_n9: f64 = (p.p3 * s.dn[240][9]);
        let eq9_e225_d_n10: f64 = (p.p3 * s.dn[240][10]);
        let eq9_e227: f64 = (eq9_e225 / s.v[30]);
        let eq9_e227_d_n0: f64 = (((eq9_e225_d_n0 * s.v[30]) - (eq9_e225 * s.dn[30][0])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n1: f64 = (((eq9_e225_d_n1 * s.v[30]) - (eq9_e225 * s.dn[30][1])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n2: f64 = (((eq9_e225_d_n2 * s.v[30]) - (eq9_e225 * s.dn[30][2])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n3: f64 = (((eq9_e225_d_n3 * s.v[30]) - (eq9_e225 * s.dn[30][3])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n4: f64 = (((eq9_e225_d_n4 * s.v[30]) - (eq9_e225 * s.dn[30][4])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n5: f64 = (((eq9_e225_d_n5 * s.v[30]) - (eq9_e225 * s.dn[30][5])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n6: f64 = (((eq9_e225_d_n6 * s.v[30]) - (eq9_e225 * s.dn[30][6])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n7: f64 = (((eq9_e225_d_n7 * s.v[30]) - (eq9_e225 * s.dn[30][7])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n8: f64 = (((eq9_e225_d_n8 * s.v[30]) - (eq9_e225 * s.dn[30][8])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n9: f64 = (((eq9_e225_d_n9 * s.v[30]) - (eq9_e225 * s.dn[30][9])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n10: f64 = (((eq9_e225_d_n10 * s.v[30]) - (eq9_e225 * s.dn[30][10])) / (s.v[30] * s.v[30]));
        let eq9_e229: f64 = (eq9_e227 * p.p1);
        let eq9_e229_d_n0: f64 = (eq9_e227_d_n0 * p.p1);
        let eq9_e229_d_n1: f64 = (eq9_e227_d_n1 * p.p1);
        let eq9_e229_d_n2: f64 = (eq9_e227_d_n2 * p.p1);
        let eq9_e229_d_n3: f64 = (eq9_e227_d_n3 * p.p1);
        let eq9_e229_d_n4: f64 = (eq9_e227_d_n4 * p.p1);
        let eq9_e229_d_n5: f64 = (eq9_e227_d_n5 * p.p1);
        let eq9_e229_d_n6: f64 = (eq9_e227_d_n6 * p.p1);
        let eq9_e229_d_n7: f64 = (eq9_e227_d_n7 * p.p1);
        let eq9_e229_d_n8: f64 = (eq9_e227_d_n8 * p.p1);
        let eq9_e229_d_n9: f64 = (eq9_e227_d_n9 * p.p1);
        let eq9_e229_d_n10: f64 = (eq9_e227_d_n10 * p.p1);
        let eq9_value: f64 = eq9_e229;
        let eq9_node_derivatives: [f64; 11] = [eq9_e229_d_n0, eq9_e229_d_n1, eq9_e229_d_n2, eq9_e229_d_n3, eq9_e229_d_n4, eq9_e229_d_n5, eq9_e229_d_n6, eq9_e229_d_n7, eq9_e229_d_n8, eq9_e229_d_n9, eq9_e229_d_n10];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[4]),
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
        let eq10_e233: f64 = (s.v[204] + s.v[209]);
        let eq10_e233_d_n0: f64 = (s.dn[204][0] + s.dn[209][0]);
        let eq10_e233_d_n1: f64 = (s.dn[204][1] + s.dn[209][1]);
        let eq10_e233_d_n2: f64 = (s.dn[204][2] + s.dn[209][2]);
        let eq10_e233_d_n3: f64 = (s.dn[204][3] + s.dn[209][3]);
        let eq10_e233_d_n4: f64 = (s.dn[204][4] + s.dn[209][4]);
        let eq10_e233_d_n5: f64 = (s.dn[204][5] + s.dn[209][5]);
        let eq10_e233_d_n6: f64 = (s.dn[204][6] + s.dn[209][6]);
        let eq10_e233_d_n7: f64 = (s.dn[204][7] + s.dn[209][7]);
        let eq10_e233_d_n8: f64 = (s.dn[204][8] + s.dn[209][8]);
        let eq10_e233_d_n9: f64 = (s.dn[204][9] + s.dn[209][9]);
        let eq10_e233_d_n10: f64 = (s.dn[204][10] + s.dn[209][10]);
        let eq10_e235: f64 = (eq10_e233 + s.v[221]);
        let eq10_e235_d_n0: f64 = (eq10_e233_d_n0 + s.dn[221][0]);
        let eq10_e235_d_n1: f64 = (eq10_e233_d_n1 + s.dn[221][1]);
        let eq10_e235_d_n2: f64 = (eq10_e233_d_n2 + s.dn[221][2]);
        let eq10_e235_d_n3: f64 = (eq10_e233_d_n3 + s.dn[221][3]);
        let eq10_e235_d_n4: f64 = (eq10_e233_d_n4 + s.dn[221][4]);
        let eq10_e235_d_n5: f64 = (eq10_e233_d_n5 + s.dn[221][5]);
        let eq10_e235_d_n6: f64 = (eq10_e233_d_n6 + s.dn[221][6]);
        let eq10_e235_d_n7: f64 = (eq10_e233_d_n7 + s.dn[221][7]);
        let eq10_e235_d_n8: f64 = (eq10_e233_d_n8 + s.dn[221][8]);
        let eq10_e235_d_n9: f64 = (eq10_e233_d_n9 + s.dn[221][9]);
        let eq10_e235_d_n10: f64 = (eq10_e233_d_n10 + s.dn[221][10]);
        let eq10_e236: f64 = (p.p3 * eq10_e235);
        let eq10_e236_d_n0: f64 = (p.p3 * eq10_e235_d_n0);
        let eq10_e236_d_n1: f64 = (p.p3 * eq10_e235_d_n1);
        let eq10_e236_d_n2: f64 = (p.p3 * eq10_e235_d_n2);
        let eq10_e236_d_n3: f64 = (p.p3 * eq10_e235_d_n3);
        let eq10_e236_d_n4: f64 = (p.p3 * eq10_e235_d_n4);
        let eq10_e236_d_n5: f64 = (p.p3 * eq10_e235_d_n5);
        let eq10_e236_d_n6: f64 = (p.p3 * eq10_e235_d_n6);
        let eq10_e236_d_n7: f64 = (p.p3 * eq10_e235_d_n7);
        let eq10_e236_d_n8: f64 = (p.p3 * eq10_e235_d_n8);
        let eq10_e236_d_n9: f64 = (p.p3 * eq10_e235_d_n9);
        let eq10_e236_d_n10: f64 = (p.p3 * eq10_e235_d_n10);
        let eq10_e237: f64 = self.eval_ddt(0, eq10_e236);
        let eq10_e237_d_n0: f64 = self.ddt_jacobian(eq10_e236_d_n0);
        let eq10_e237_d_n1: f64 = self.ddt_jacobian(eq10_e236_d_n1);
        let eq10_e237_d_n2: f64 = self.ddt_jacobian(eq10_e236_d_n2);
        let eq10_e237_d_n3: f64 = self.ddt_jacobian(eq10_e236_d_n3);
        let eq10_e237_d_n4: f64 = self.ddt_jacobian(eq10_e236_d_n4);
        let eq10_e237_d_n5: f64 = self.ddt_jacobian(eq10_e236_d_n5);
        let eq10_e237_d_n6: f64 = self.ddt_jacobian(eq10_e236_d_n6);
        let eq10_e237_d_n7: f64 = self.ddt_jacobian(eq10_e236_d_n7);
        let eq10_e237_d_n8: f64 = self.ddt_jacobian(eq10_e236_d_n8);
        let eq10_e237_d_n9: f64 = self.ddt_jacobian(eq10_e236_d_n9);
        let eq10_e237_d_n10: f64 = self.ddt_jacobian(eq10_e236_d_n10);
        let eq10_e239: f64 = (eq10_e237 * p.p1);
        let eq10_e239_d_n0: f64 = (eq10_e237_d_n0 * p.p1);
        let eq10_e239_d_n1: f64 = (eq10_e237_d_n1 * p.p1);
        let eq10_e239_d_n2: f64 = (eq10_e237_d_n2 * p.p1);
        let eq10_e239_d_n3: f64 = (eq10_e237_d_n3 * p.p1);
        let eq10_e239_d_n4: f64 = (eq10_e237_d_n4 * p.p1);
        let eq10_e239_d_n5: f64 = (eq10_e237_d_n5 * p.p1);
        let eq10_e239_d_n6: f64 = (eq10_e237_d_n6 * p.p1);
        let eq10_e239_d_n7: f64 = (eq10_e237_d_n7 * p.p1);
        let eq10_e239_d_n8: f64 = (eq10_e237_d_n8 * p.p1);
        let eq10_e239_d_n9: f64 = (eq10_e237_d_n9 * p.p1);
        let eq10_e239_d_n10: f64 = (eq10_e237_d_n10 * p.p1);
        let eq10_value: f64 = eq10_e239;
        let eq10_node_derivatives: [f64; 11] = [eq10_e239_d_n0, eq10_e239_d_n1, eq10_e239_d_n2, eq10_e239_d_n3, eq10_e239_d_n4, eq10_e239_d_n5, eq10_e239_d_n6, eq10_e239_d_n7, eq10_e239_d_n8, eq10_e239_d_n9, eq10_e239_d_n10];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[3]),
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
        let eq11_e242: f64 = (p.p3 * s.v[206]);
        let eq11_e242_d_n0: f64 = (p.p3 * s.dn[206][0]);
        let eq11_e242_d_n1: f64 = (p.p3 * s.dn[206][1]);
        let eq11_e242_d_n2: f64 = (p.p3 * s.dn[206][2]);
        let eq11_e242_d_n3: f64 = (p.p3 * s.dn[206][3]);
        let eq11_e242_d_n4: f64 = (p.p3 * s.dn[206][4]);
        let eq11_e242_d_n5: f64 = (p.p3 * s.dn[206][5]);
        let eq11_e242_d_n6: f64 = (p.p3 * s.dn[206][6]);
        let eq11_e242_d_n7: f64 = (p.p3 * s.dn[206][7]);
        let eq11_e242_d_n8: f64 = (p.p3 * s.dn[206][8]);
        let eq11_e242_d_n9: f64 = (p.p3 * s.dn[206][9]);
        let eq11_e242_d_n10: f64 = (p.p3 * s.dn[206][10]);
        let eq11_e243: f64 = self.eval_ddt(1, eq11_e242);
        let eq11_e243_d_n0: f64 = self.ddt_jacobian(eq11_e242_d_n0);
        let eq11_e243_d_n1: f64 = self.ddt_jacobian(eq11_e242_d_n1);
        let eq11_e243_d_n2: f64 = self.ddt_jacobian(eq11_e242_d_n2);
        let eq11_e243_d_n3: f64 = self.ddt_jacobian(eq11_e242_d_n3);
        let eq11_e243_d_n4: f64 = self.ddt_jacobian(eq11_e242_d_n4);
        let eq11_e243_d_n5: f64 = self.ddt_jacobian(eq11_e242_d_n5);
        let eq11_e243_d_n6: f64 = self.ddt_jacobian(eq11_e242_d_n6);
        let eq11_e243_d_n7: f64 = self.ddt_jacobian(eq11_e242_d_n7);
        let eq11_e243_d_n8: f64 = self.ddt_jacobian(eq11_e242_d_n8);
        let eq11_e243_d_n9: f64 = self.ddt_jacobian(eq11_e242_d_n9);
        let eq11_e243_d_n10: f64 = self.ddt_jacobian(eq11_e242_d_n10);
        let eq11_e245: f64 = (eq11_e243 * p.p1);
        let eq11_e245_d_n0: f64 = (eq11_e243_d_n0 * p.p1);
        let eq11_e245_d_n1: f64 = (eq11_e243_d_n1 * p.p1);
        let eq11_e245_d_n2: f64 = (eq11_e243_d_n2 * p.p1);
        let eq11_e245_d_n3: f64 = (eq11_e243_d_n3 * p.p1);
        let eq11_e245_d_n4: f64 = (eq11_e243_d_n4 * p.p1);
        let eq11_e245_d_n5: f64 = (eq11_e243_d_n5 * p.p1);
        let eq11_e245_d_n6: f64 = (eq11_e243_d_n6 * p.p1);
        let eq11_e245_d_n7: f64 = (eq11_e243_d_n7 * p.p1);
        let eq11_e245_d_n8: f64 = (eq11_e243_d_n8 * p.p1);
        let eq11_e245_d_n9: f64 = (eq11_e243_d_n9 * p.p1);
        let eq11_e245_d_n10: f64 = (eq11_e243_d_n10 * p.p1);
        let eq11_value: f64 = eq11_e245;
        let eq11_node_derivatives: [f64; 11] = [eq11_e245_d_n0, eq11_e245_d_n1, eq11_e245_d_n2, eq11_e245_d_n3, eq11_e245_d_n4, eq11_e245_d_n5, eq11_e245_d_n6, eq11_e245_d_n7, eq11_e245_d_n8, eq11_e245_d_n9, eq11_e245_d_n10];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[3]),
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
        let eq12_e249: f64 = (s.v[207] + s.v[210]);
        let eq12_e249_d_n0: f64 = (s.dn[207][0] + s.dn[210][0]);
        let eq12_e249_d_n1: f64 = (s.dn[207][1] + s.dn[210][1]);
        let eq12_e249_d_n2: f64 = (s.dn[207][2] + s.dn[210][2]);
        let eq12_e249_d_n3: f64 = (s.dn[207][3] + s.dn[210][3]);
        let eq12_e249_d_n4: f64 = (s.dn[207][4] + s.dn[210][4]);
        let eq12_e249_d_n5: f64 = (s.dn[207][5] + s.dn[210][5]);
        let eq12_e249_d_n6: f64 = (s.dn[207][6] + s.dn[210][6]);
        let eq12_e249_d_n7: f64 = (s.dn[207][7] + s.dn[210][7]);
        let eq12_e249_d_n8: f64 = (s.dn[207][8] + s.dn[210][8]);
        let eq12_e249_d_n9: f64 = (s.dn[207][9] + s.dn[210][9]);
        let eq12_e249_d_n10: f64 = (s.dn[207][10] + s.dn[210][10]);
        let eq12_e251: f64 = (eq12_e249 + s.v[224]);
        let eq12_e251_d_n0: f64 = (eq12_e249_d_n0 + s.dn[224][0]);
        let eq12_e251_d_n1: f64 = (eq12_e249_d_n1 + s.dn[224][1]);
        let eq12_e251_d_n2: f64 = (eq12_e249_d_n2 + s.dn[224][2]);
        let eq12_e251_d_n3: f64 = (eq12_e249_d_n3 + s.dn[224][3]);
        let eq12_e251_d_n4: f64 = (eq12_e249_d_n4 + s.dn[224][4]);
        let eq12_e251_d_n5: f64 = (eq12_e249_d_n5 + s.dn[224][5]);
        let eq12_e251_d_n6: f64 = (eq12_e249_d_n6 + s.dn[224][6]);
        let eq12_e251_d_n7: f64 = (eq12_e249_d_n7 + s.dn[224][7]);
        let eq12_e251_d_n8: f64 = (eq12_e249_d_n8 + s.dn[224][8]);
        let eq12_e251_d_n9: f64 = (eq12_e249_d_n9 + s.dn[224][9]);
        let eq12_e251_d_n10: f64 = (eq12_e249_d_n10 + s.dn[224][10]);
        let eq12_e252: f64 = (p.p3 * eq12_e251);
        let eq12_e252_d_n0: f64 = (p.p3 * eq12_e251_d_n0);
        let eq12_e252_d_n1: f64 = (p.p3 * eq12_e251_d_n1);
        let eq12_e252_d_n2: f64 = (p.p3 * eq12_e251_d_n2);
        let eq12_e252_d_n3: f64 = (p.p3 * eq12_e251_d_n3);
        let eq12_e252_d_n4: f64 = (p.p3 * eq12_e251_d_n4);
        let eq12_e252_d_n5: f64 = (p.p3 * eq12_e251_d_n5);
        let eq12_e252_d_n6: f64 = (p.p3 * eq12_e251_d_n6);
        let eq12_e252_d_n7: f64 = (p.p3 * eq12_e251_d_n7);
        let eq12_e252_d_n8: f64 = (p.p3 * eq12_e251_d_n8);
        let eq12_e252_d_n9: f64 = (p.p3 * eq12_e251_d_n9);
        let eq12_e252_d_n10: f64 = (p.p3 * eq12_e251_d_n10);
        let eq12_e253: f64 = self.eval_ddt(2, eq12_e252);
        let eq12_e253_d_n0: f64 = self.ddt_jacobian(eq12_e252_d_n0);
        let eq12_e253_d_n1: f64 = self.ddt_jacobian(eq12_e252_d_n1);
        let eq12_e253_d_n2: f64 = self.ddt_jacobian(eq12_e252_d_n2);
        let eq12_e253_d_n3: f64 = self.ddt_jacobian(eq12_e252_d_n3);
        let eq12_e253_d_n4: f64 = self.ddt_jacobian(eq12_e252_d_n4);
        let eq12_e253_d_n5: f64 = self.ddt_jacobian(eq12_e252_d_n5);
        let eq12_e253_d_n6: f64 = self.ddt_jacobian(eq12_e252_d_n6);
        let eq12_e253_d_n7: f64 = self.ddt_jacobian(eq12_e252_d_n7);
        let eq12_e253_d_n8: f64 = self.ddt_jacobian(eq12_e252_d_n8);
        let eq12_e253_d_n9: f64 = self.ddt_jacobian(eq12_e252_d_n9);
        let eq12_e253_d_n10: f64 = self.ddt_jacobian(eq12_e252_d_n10);
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
        let eq12_value: f64 = eq12_e255;
        let eq12_node_derivatives: [f64; 11] = [eq12_e255_d_n0, eq12_e255_d_n1, eq12_e255_d_n2, eq12_e255_d_n3, eq12_e255_d_n4, eq12_e255_d_n5, eq12_e255_d_n6, eq12_e255_d_n7, eq12_e255_d_n8, eq12_e255_d_n9, eq12_e255_d_n10];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
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
        let eq13_e258: f64 = (p.p3 * s.v[211]);
        let eq13_e258_d_n0: f64 = (p.p3 * s.dn[211][0]);
        let eq13_e258_d_n1: f64 = (p.p3 * s.dn[211][1]);
        let eq13_e258_d_n2: f64 = (p.p3 * s.dn[211][2]);
        let eq13_e258_d_n3: f64 = (p.p3 * s.dn[211][3]);
        let eq13_e258_d_n4: f64 = (p.p3 * s.dn[211][4]);
        let eq13_e258_d_n5: f64 = (p.p3 * s.dn[211][5]);
        let eq13_e258_d_n6: f64 = (p.p3 * s.dn[211][6]);
        let eq13_e258_d_n7: f64 = (p.p3 * s.dn[211][7]);
        let eq13_e258_d_n8: f64 = (p.p3 * s.dn[211][8]);
        let eq13_e258_d_n9: f64 = (p.p3 * s.dn[211][9]);
        let eq13_e258_d_n10: f64 = (p.p3 * s.dn[211][10]);
        let eq13_e259: f64 = self.eval_ddt(3, eq13_e258);
        let eq13_e259_d_n0: f64 = self.ddt_jacobian(eq13_e258_d_n0);
        let eq13_e259_d_n1: f64 = self.ddt_jacobian(eq13_e258_d_n1);
        let eq13_e259_d_n2: f64 = self.ddt_jacobian(eq13_e258_d_n2);
        let eq13_e259_d_n3: f64 = self.ddt_jacobian(eq13_e258_d_n3);
        let eq13_e259_d_n4: f64 = self.ddt_jacobian(eq13_e258_d_n4);
        let eq13_e259_d_n5: f64 = self.ddt_jacobian(eq13_e258_d_n5);
        let eq13_e259_d_n6: f64 = self.ddt_jacobian(eq13_e258_d_n6);
        let eq13_e259_d_n7: f64 = self.ddt_jacobian(eq13_e258_d_n7);
        let eq13_e259_d_n8: f64 = self.ddt_jacobian(eq13_e258_d_n8);
        let eq13_e259_d_n9: f64 = self.ddt_jacobian(eq13_e258_d_n9);
        let eq13_e259_d_n10: f64 = self.ddt_jacobian(eq13_e258_d_n10);
        let eq13_e261: f64 = (eq13_e259 * p.p1);
        let eq13_e261_d_n0: f64 = (eq13_e259_d_n0 * p.p1);
        let eq13_e261_d_n1: f64 = (eq13_e259_d_n1 * p.p1);
        let eq13_e261_d_n2: f64 = (eq13_e259_d_n2 * p.p1);
        let eq13_e261_d_n3: f64 = (eq13_e259_d_n3 * p.p1);
        let eq13_e261_d_n4: f64 = (eq13_e259_d_n4 * p.p1);
        let eq13_e261_d_n5: f64 = (eq13_e259_d_n5 * p.p1);
        let eq13_e261_d_n6: f64 = (eq13_e259_d_n6 * p.p1);
        let eq13_e261_d_n7: f64 = (eq13_e259_d_n7 * p.p1);
        let eq13_e261_d_n8: f64 = (eq13_e259_d_n8 * p.p1);
        let eq13_e261_d_n9: f64 = (eq13_e259_d_n9 * p.p1);
        let eq13_e261_d_n10: f64 = (eq13_e259_d_n10 * p.p1);
        let eq13_value: f64 = eq13_e261;
        let eq13_node_derivatives: [f64; 11] = [eq13_e261_d_n0, eq13_e261_d_n1, eq13_e261_d_n2, eq13_e261_d_n3, eq13_e261_d_n4, eq13_e261_d_n5, eq13_e261_d_n6, eq13_e261_d_n7, eq13_e261_d_n8, eq13_e261_d_n9, eq13_e261_d_n10];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
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
        let eq14_e264: f64 = (p.p3 * p.p68);
        let eq14_e266: f64 = (eq14_e264 * s.v[243]);
        let eq14_e266_d_n0: f64 = (eq14_e264 * s.dn[243][0]);
        let eq14_e266_d_n1: f64 = (eq14_e264 * s.dn[243][1]);
        let eq14_e266_d_n2: f64 = (eq14_e264 * s.dn[243][2]);
        let eq14_e266_d_n3: f64 = (eq14_e264 * s.dn[243][3]);
        let eq14_e266_d_n4: f64 = (eq14_e264 * s.dn[243][4]);
        let eq14_e266_d_n5: f64 = (eq14_e264 * s.dn[243][5]);
        let eq14_e266_d_n6: f64 = (eq14_e264 * s.dn[243][6]);
        let eq14_e266_d_n7: f64 = (eq14_e264 * s.dn[243][7]);
        let eq14_e266_d_n8: f64 = (eq14_e264 * s.dn[243][8]);
        let eq14_e266_d_n9: f64 = (eq14_e264 * s.dn[243][9]);
        let eq14_e266_d_n10: f64 = (eq14_e264 * s.dn[243][10]);
        let eq14_e267: f64 = self.eval_ddt(4, eq14_e266);
        let eq14_e267_d_n0: f64 = self.ddt_jacobian(eq14_e266_d_n0);
        let eq14_e267_d_n1: f64 = self.ddt_jacobian(eq14_e266_d_n1);
        let eq14_e267_d_n2: f64 = self.ddt_jacobian(eq14_e266_d_n2);
        let eq14_e267_d_n3: f64 = self.ddt_jacobian(eq14_e266_d_n3);
        let eq14_e267_d_n4: f64 = self.ddt_jacobian(eq14_e266_d_n4);
        let eq14_e267_d_n5: f64 = self.ddt_jacobian(eq14_e266_d_n5);
        let eq14_e267_d_n6: f64 = self.ddt_jacobian(eq14_e266_d_n6);
        let eq14_e267_d_n7: f64 = self.ddt_jacobian(eq14_e266_d_n7);
        let eq14_e267_d_n8: f64 = self.ddt_jacobian(eq14_e266_d_n8);
        let eq14_e267_d_n9: f64 = self.ddt_jacobian(eq14_e266_d_n9);
        let eq14_e267_d_n10: f64 = self.ddt_jacobian(eq14_e266_d_n10);
        let eq14_e269: f64 = (eq14_e267 * p.p1);
        let eq14_e269_d_n0: f64 = (eq14_e267_d_n0 * p.p1);
        let eq14_e269_d_n1: f64 = (eq14_e267_d_n1 * p.p1);
        let eq14_e269_d_n2: f64 = (eq14_e267_d_n2 * p.p1);
        let eq14_e269_d_n3: f64 = (eq14_e267_d_n3 * p.p1);
        let eq14_e269_d_n4: f64 = (eq14_e267_d_n4 * p.p1);
        let eq14_e269_d_n5: f64 = (eq14_e267_d_n5 * p.p1);
        let eq14_e269_d_n6: f64 = (eq14_e267_d_n6 * p.p1);
        let eq14_e269_d_n7: f64 = (eq14_e267_d_n7 * p.p1);
        let eq14_e269_d_n8: f64 = (eq14_e267_d_n8 * p.p1);
        let eq14_e269_d_n9: f64 = (eq14_e267_d_n9 * p.p1);
        let eq14_e269_d_n10: f64 = (eq14_e267_d_n10 * p.p1);
        let eq14_value: f64 = eq14_e269;
        let eq14_node_derivatives: [f64; 11] = [eq14_e269_d_n0, eq14_e269_d_n1, eq14_e269_d_n2, eq14_e269_d_n3, eq14_e269_d_n4, eq14_e269_d_n5, eq14_e269_d_n6, eq14_e269_d_n7, eq14_e269_d_n8, eq14_e269_d_n9, eq14_e269_d_n10];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[2]),
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
        let eq15_e272: f64 = (p.p3 * p.p77);
        let eq15_e274: f64 = (eq15_e272 * s.v[244]);
        let eq15_e274_d_n0: f64 = (eq15_e272 * s.dn[244][0]);
        let eq15_e274_d_n1: f64 = (eq15_e272 * s.dn[244][1]);
        let eq15_e274_d_n2: f64 = (eq15_e272 * s.dn[244][2]);
        let eq15_e274_d_n3: f64 = (eq15_e272 * s.dn[244][3]);
        let eq15_e274_d_n4: f64 = (eq15_e272 * s.dn[244][4]);
        let eq15_e274_d_n5: f64 = (eq15_e272 * s.dn[244][5]);
        let eq15_e274_d_n6: f64 = (eq15_e272 * s.dn[244][6]);
        let eq15_e274_d_n7: f64 = (eq15_e272 * s.dn[244][7]);
        let eq15_e274_d_n8: f64 = (eq15_e272 * s.dn[244][8]);
        let eq15_e274_d_n9: f64 = (eq15_e272 * s.dn[244][9]);
        let eq15_e274_d_n10: f64 = (eq15_e272 * s.dn[244][10]);
        let eq15_e275: f64 = self.eval_ddt(5, eq15_e274);
        let eq15_e275_d_n0: f64 = self.ddt_jacobian(eq15_e274_d_n0);
        let eq15_e275_d_n1: f64 = self.ddt_jacobian(eq15_e274_d_n1);
        let eq15_e275_d_n2: f64 = self.ddt_jacobian(eq15_e274_d_n2);
        let eq15_e275_d_n3: f64 = self.ddt_jacobian(eq15_e274_d_n3);
        let eq15_e275_d_n4: f64 = self.ddt_jacobian(eq15_e274_d_n4);
        let eq15_e275_d_n5: f64 = self.ddt_jacobian(eq15_e274_d_n5);
        let eq15_e275_d_n6: f64 = self.ddt_jacobian(eq15_e274_d_n6);
        let eq15_e275_d_n7: f64 = self.ddt_jacobian(eq15_e274_d_n7);
        let eq15_e275_d_n8: f64 = self.ddt_jacobian(eq15_e274_d_n8);
        let eq15_e275_d_n9: f64 = self.ddt_jacobian(eq15_e274_d_n9);
        let eq15_e275_d_n10: f64 = self.ddt_jacobian(eq15_e274_d_n10);
        let eq15_e277: f64 = (eq15_e275 * p.p1);
        let eq15_e277_d_n0: f64 = (eq15_e275_d_n0 * p.p1);
        let eq15_e277_d_n1: f64 = (eq15_e275_d_n1 * p.p1);
        let eq15_e277_d_n2: f64 = (eq15_e275_d_n2 * p.p1);
        let eq15_e277_d_n3: f64 = (eq15_e275_d_n3 * p.p1);
        let eq15_e277_d_n4: f64 = (eq15_e275_d_n4 * p.p1);
        let eq15_e277_d_n5: f64 = (eq15_e275_d_n5 * p.p1);
        let eq15_e277_d_n6: f64 = (eq15_e275_d_n6 * p.p1);
        let eq15_e277_d_n7: f64 = (eq15_e275_d_n7 * p.p1);
        let eq15_e277_d_n8: f64 = (eq15_e275_d_n8 * p.p1);
        let eq15_e277_d_n9: f64 = (eq15_e275_d_n9 * p.p1);
        let eq15_e277_d_n10: f64 = (eq15_e275_d_n10 * p.p1);
        let eq15_value: f64 = eq15_e277;
        let eq15_node_derivatives: [f64; 11] = [eq15_e277_d_n0, eq15_e277_d_n1, eq15_e277_d_n2, eq15_e277_d_n3, eq15_e277_d_n4, eq15_e277_d_n5, eq15_e277_d_n6, eq15_e277_d_n7, eq15_e277_d_n8, eq15_e277_d_n9, eq15_e277_d_n10];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[0]),
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
        let eq16_e280: f64 = (p.p3 * s.v[169]);
        let eq16_e280_d_n0: f64 = (p.p3 * s.dn[169][0]);
        let eq16_e280_d_n1: f64 = (p.p3 * s.dn[169][1]);
        let eq16_e280_d_n2: f64 = (p.p3 * s.dn[169][2]);
        let eq16_e280_d_n3: f64 = (p.p3 * s.dn[169][3]);
        let eq16_e280_d_n4: f64 = (p.p3 * s.dn[169][4]);
        let eq16_e280_d_n5: f64 = (p.p3 * s.dn[169][5]);
        let eq16_e280_d_n6: f64 = (p.p3 * s.dn[169][6]);
        let eq16_e280_d_n7: f64 = (p.p3 * s.dn[169][7]);
        let eq16_e280_d_n8: f64 = (p.p3 * s.dn[169][8]);
        let eq16_e280_d_n9: f64 = (p.p3 * s.dn[169][9]);
        let eq16_e280_d_n10: f64 = (p.p3 * s.dn[169][10]);
        let eq16_e282: f64 = (eq16_e280 * p.p1);
        let eq16_e282_d_n0: f64 = (eq16_e280_d_n0 * p.p1);
        let eq16_e282_d_n1: f64 = (eq16_e280_d_n1 * p.p1);
        let eq16_e282_d_n2: f64 = (eq16_e280_d_n2 * p.p1);
        let eq16_e282_d_n3: f64 = (eq16_e280_d_n3 * p.p1);
        let eq16_e282_d_n4: f64 = (eq16_e280_d_n4 * p.p1);
        let eq16_e282_d_n5: f64 = (eq16_e280_d_n5 * p.p1);
        let eq16_e282_d_n6: f64 = (eq16_e280_d_n6 * p.p1);
        let eq16_e282_d_n7: f64 = (eq16_e280_d_n7 * p.p1);
        let eq16_e282_d_n8: f64 = (eq16_e280_d_n8 * p.p1);
        let eq16_e282_d_n9: f64 = (eq16_e280_d_n9 * p.p1);
        let eq16_e282_d_n10: f64 = (eq16_e280_d_n10 * p.p1);
        let eq16_value: f64 = eq16_e282;
        let eq16_node_derivatives: [f64; 11] = [eq16_e282_d_n0, eq16_e282_d_n1, eq16_e282_d_n2, eq16_e282_d_n3, eq16_e282_d_n4, eq16_e282_d_n5, eq16_e282_d_n6, eq16_e282_d_n7, eq16_e282_d_n8, eq16_e282_d_n9, eq16_e282_d_n10];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
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
        let eq17_e285: f64 = (p.p3 * s.v[242]);
        let eq17_e285_d_n0: f64 = (p.p3 * s.dn[242][0]);
        let eq17_e285_d_n1: f64 = (p.p3 * s.dn[242][1]);
        let eq17_e285_d_n2: f64 = (p.p3 * s.dn[242][2]);
        let eq17_e285_d_n3: f64 = (p.p3 * s.dn[242][3]);
        let eq17_e285_d_n4: f64 = (p.p3 * s.dn[242][4]);
        let eq17_e285_d_n5: f64 = (p.p3 * s.dn[242][5]);
        let eq17_e285_d_n6: f64 = (p.p3 * s.dn[242][6]);
        let eq17_e285_d_n7: f64 = (p.p3 * s.dn[242][7]);
        let eq17_e285_d_n8: f64 = (p.p3 * s.dn[242][8]);
        let eq17_e285_d_n9: f64 = (p.p3 * s.dn[242][9]);
        let eq17_e285_d_n10: f64 = (p.p3 * s.dn[242][10]);
        let eq17_e287: f64 = (eq17_e285 * s.v[101]);
        let eq17_e287_d_n0: f64 = ((eq17_e285_d_n0 * s.v[101]) + (eq17_e285 * s.dn[101][0]));
        let eq17_e287_d_n1: f64 = ((eq17_e285_d_n1 * s.v[101]) + (eq17_e285 * s.dn[101][1]));
        let eq17_e287_d_n2: f64 = ((eq17_e285_d_n2 * s.v[101]) + (eq17_e285 * s.dn[101][2]));
        let eq17_e287_d_n3: f64 = ((eq17_e285_d_n3 * s.v[101]) + (eq17_e285 * s.dn[101][3]));
        let eq17_e287_d_n4: f64 = ((eq17_e285_d_n4 * s.v[101]) + (eq17_e285 * s.dn[101][4]));
        let eq17_e287_d_n5: f64 = ((eq17_e285_d_n5 * s.v[101]) + (eq17_e285 * s.dn[101][5]));
        let eq17_e287_d_n6: f64 = ((eq17_e285_d_n6 * s.v[101]) + (eq17_e285 * s.dn[101][6]));
        let eq17_e287_d_n7: f64 = ((eq17_e285_d_n7 * s.v[101]) + (eq17_e285 * s.dn[101][7]));
        let eq17_e287_d_n8: f64 = ((eq17_e285_d_n8 * s.v[101]) + (eq17_e285 * s.dn[101][8]));
        let eq17_e287_d_n9: f64 = ((eq17_e285_d_n9 * s.v[101]) + (eq17_e285 * s.dn[101][9]));
        let eq17_e287_d_n10: f64 = ((eq17_e285_d_n10 * s.v[101]) + (eq17_e285 * s.dn[101][10]));
        let eq17_e289: f64 = (eq17_e287 * p.p1);
        let eq17_e289_d_n0: f64 = (eq17_e287_d_n0 * p.p1);
        let eq17_e289_d_n1: f64 = (eq17_e287_d_n1 * p.p1);
        let eq17_e289_d_n2: f64 = (eq17_e287_d_n2 * p.p1);
        let eq17_e289_d_n3: f64 = (eq17_e287_d_n3 * p.p1);
        let eq17_e289_d_n4: f64 = (eq17_e287_d_n4 * p.p1);
        let eq17_e289_d_n5: f64 = (eq17_e287_d_n5 * p.p1);
        let eq17_e289_d_n6: f64 = (eq17_e287_d_n6 * p.p1);
        let eq17_e289_d_n7: f64 = (eq17_e287_d_n7 * p.p1);
        let eq17_e289_d_n8: f64 = (eq17_e287_d_n8 * p.p1);
        let eq17_e289_d_n9: f64 = (eq17_e287_d_n9 * p.p1);
        let eq17_e289_d_n10: f64 = (eq17_e287_d_n10 * p.p1);
        let eq17_value: f64 = eq17_e289;
        let eq17_node_derivatives: [f64; 11] = [eq17_e289_d_n0, eq17_e289_d_n1, eq17_e289_d_n2, eq17_e289_d_n3, eq17_e289_d_n4, eq17_e289_d_n5, eq17_e289_d_n6, eq17_e289_d_n7, eq17_e289_d_n8, eq17_e289_d_n9, eq17_e289_d_n10];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[8]),
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
        let eq18_e293: f64 = (s.v[219] + s.v[228]);
        let eq18_e293_d_n0: f64 = (s.dn[219][0] + s.dn[228][0]);
        let eq18_e293_d_n1: f64 = (s.dn[219][1] + s.dn[228][1]);
        let eq18_e293_d_n2: f64 = (s.dn[219][2] + s.dn[228][2]);
        let eq18_e293_d_n3: f64 = (s.dn[219][3] + s.dn[228][3]);
        let eq18_e293_d_n4: f64 = (s.dn[219][4] + s.dn[228][4]);
        let eq18_e293_d_n5: f64 = (s.dn[219][5] + s.dn[228][5]);
        let eq18_e293_d_n6: f64 = (s.dn[219][6] + s.dn[228][6]);
        let eq18_e293_d_n7: f64 = (s.dn[219][7] + s.dn[228][7]);
        let eq18_e293_d_n8: f64 = (s.dn[219][8] + s.dn[228][8]);
        let eq18_e293_d_n9: f64 = (s.dn[219][9] + s.dn[228][9]);
        let eq18_e293_d_n10: f64 = (s.dn[219][10] + s.dn[228][10]);
        let eq18_e294: f64 = (p.p3 * eq18_e293);
        let eq18_e294_d_n0: f64 = (p.p3 * eq18_e293_d_n0);
        let eq18_e294_d_n1: f64 = (p.p3 * eq18_e293_d_n1);
        let eq18_e294_d_n2: f64 = (p.p3 * eq18_e293_d_n2);
        let eq18_e294_d_n3: f64 = (p.p3 * eq18_e293_d_n3);
        let eq18_e294_d_n4: f64 = (p.p3 * eq18_e293_d_n4);
        let eq18_e294_d_n5: f64 = (p.p3 * eq18_e293_d_n5);
        let eq18_e294_d_n6: f64 = (p.p3 * eq18_e293_d_n6);
        let eq18_e294_d_n7: f64 = (p.p3 * eq18_e293_d_n7);
        let eq18_e294_d_n8: f64 = (p.p3 * eq18_e293_d_n8);
        let eq18_e294_d_n9: f64 = (p.p3 * eq18_e293_d_n9);
        let eq18_e294_d_n10: f64 = (p.p3 * eq18_e293_d_n10);
        let eq18_e295: f64 = self.eval_ddt(6, eq18_e294);
        let eq18_e295_d_n0: f64 = self.ddt_jacobian(eq18_e294_d_n0);
        let eq18_e295_d_n1: f64 = self.ddt_jacobian(eq18_e294_d_n1);
        let eq18_e295_d_n2: f64 = self.ddt_jacobian(eq18_e294_d_n2);
        let eq18_e295_d_n3: f64 = self.ddt_jacobian(eq18_e294_d_n3);
        let eq18_e295_d_n4: f64 = self.ddt_jacobian(eq18_e294_d_n4);
        let eq18_e295_d_n5: f64 = self.ddt_jacobian(eq18_e294_d_n5);
        let eq18_e295_d_n6: f64 = self.ddt_jacobian(eq18_e294_d_n6);
        let eq18_e295_d_n7: f64 = self.ddt_jacobian(eq18_e294_d_n7);
        let eq18_e295_d_n8: f64 = self.ddt_jacobian(eq18_e294_d_n8);
        let eq18_e295_d_n9: f64 = self.ddt_jacobian(eq18_e294_d_n9);
        let eq18_e295_d_n10: f64 = self.ddt_jacobian(eq18_e294_d_n10);
        let eq18_e297: f64 = (eq18_e295 * p.p1);
        let eq18_e297_d_n0: f64 = (eq18_e295_d_n0 * p.p1);
        let eq18_e297_d_n1: f64 = (eq18_e295_d_n1 * p.p1);
        let eq18_e297_d_n2: f64 = (eq18_e295_d_n2 * p.p1);
        let eq18_e297_d_n3: f64 = (eq18_e295_d_n3 * p.p1);
        let eq18_e297_d_n4: f64 = (eq18_e295_d_n4 * p.p1);
        let eq18_e297_d_n5: f64 = (eq18_e295_d_n5 * p.p1);
        let eq18_e297_d_n6: f64 = (eq18_e295_d_n6 * p.p1);
        let eq18_e297_d_n7: f64 = (eq18_e295_d_n7 * p.p1);
        let eq18_e297_d_n8: f64 = (eq18_e295_d_n8 * p.p1);
        let eq18_e297_d_n9: f64 = (eq18_e295_d_n9 * p.p1);
        let eq18_e297_d_n10: f64 = (eq18_e295_d_n10 * p.p1);
        let eq18_value: f64 = eq18_e297;
        let eq18_node_derivatives: [f64; 11] = [eq18_e297_d_n0, eq18_e297_d_n1, eq18_e297_d_n2, eq18_e297_d_n3, eq18_e297_d_n4, eq18_e297_d_n5, eq18_e297_d_n6, eq18_e297_d_n7, eq18_e297_d_n8, eq18_e297_d_n9, eq18_e297_d_n10];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[8]),
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
        let eq19_e302: f64 = (s.v[314] * s.v[235]);
        let eq19_e302_d_n0: f64 = (s.v[314] * s.dn[235][0]);
        let eq19_e302_d_n1: f64 = (s.v[314] * s.dn[235][1]);
        let eq19_e302_d_n2: f64 = (s.v[314] * s.dn[235][2]);
        let eq19_e302_d_n3: f64 = (s.v[314] * s.dn[235][3]);
        let eq19_e302_d_n4: f64 = (s.v[314] * s.dn[235][4]);
        let eq19_e302_d_n5: f64 = (s.v[314] * s.dn[235][5]);
        let eq19_e302_d_n6: f64 = (s.v[314] * s.dn[235][6]);
        let eq19_e302_d_n7: f64 = (s.v[314] * s.dn[235][7]);
        let eq19_e302_d_n8: f64 = (s.v[314] * s.dn[235][8]);
        let eq19_e302_d_n9: f64 = (s.v[314] * s.dn[235][9]);
        let eq19_e302_d_n10: f64 = (s.v[314] * s.dn[235][10]);
        let eq19_e303: f64 = (s.v[154] + eq19_e302);
        let eq19_e303_d_n0: f64 = (s.dn[154][0] + eq19_e302_d_n0);
        let eq19_e303_d_n1: f64 = (s.dn[154][1] + eq19_e302_d_n1);
        let eq19_e303_d_n2: f64 = (s.dn[154][2] + eq19_e302_d_n2);
        let eq19_e303_d_n3: f64 = (s.dn[154][3] + eq19_e302_d_n3);
        let eq19_e303_d_n4: f64 = (s.dn[154][4] + eq19_e302_d_n4);
        let eq19_e303_d_n5: f64 = (s.dn[154][5] + eq19_e302_d_n5);
        let eq19_e303_d_n6: f64 = (s.dn[154][6] + eq19_e302_d_n6);
        let eq19_e303_d_n7: f64 = (s.dn[154][7] + eq19_e302_d_n7);
        let eq19_e303_d_n8: f64 = (s.dn[154][8] + eq19_e302_d_n8);
        let eq19_e303_d_n9: f64 = (s.dn[154][9] + eq19_e302_d_n9);
        let eq19_e303_d_n10: f64 = (s.dn[154][10] + eq19_e302_d_n10);
        let eq19_e305: f64 = (eq19_e303 + s.v[157]);
        let eq19_e305_d_n0: f64 = (eq19_e303_d_n0 + s.dn[157][0]);
        let eq19_e305_d_n1: f64 = (eq19_e303_d_n1 + s.dn[157][1]);
        let eq19_e305_d_n2: f64 = (eq19_e303_d_n2 + s.dn[157][2]);
        let eq19_e305_d_n3: f64 = (eq19_e303_d_n3 + s.dn[157][3]);
        let eq19_e305_d_n4: f64 = (eq19_e303_d_n4 + s.dn[157][4]);
        let eq19_e305_d_n5: f64 = (eq19_e303_d_n5 + s.dn[157][5]);
        let eq19_e305_d_n6: f64 = (eq19_e303_d_n6 + s.dn[157][6]);
        let eq19_e305_d_n7: f64 = (eq19_e303_d_n7 + s.dn[157][7]);
        let eq19_e305_d_n8: f64 = (eq19_e303_d_n8 + s.dn[157][8]);
        let eq19_e305_d_n9: f64 = (eq19_e303_d_n9 + s.dn[157][9]);
        let eq19_e305_d_n10: f64 = (eq19_e303_d_n10 + s.dn[157][10]);
        let eq19_e306: f64 = (p.p3 * eq19_e305);
        let eq19_e306_d_n0: f64 = (p.p3 * eq19_e305_d_n0);
        let eq19_e306_d_n1: f64 = (p.p3 * eq19_e305_d_n1);
        let eq19_e306_d_n2: f64 = (p.p3 * eq19_e305_d_n2);
        let eq19_e306_d_n3: f64 = (p.p3 * eq19_e305_d_n3);
        let eq19_e306_d_n4: f64 = (p.p3 * eq19_e305_d_n4);
        let eq19_e306_d_n5: f64 = (p.p3 * eq19_e305_d_n5);
        let eq19_e306_d_n6: f64 = (p.p3 * eq19_e305_d_n6);
        let eq19_e306_d_n7: f64 = (p.p3 * eq19_e305_d_n7);
        let eq19_e306_d_n8: f64 = (p.p3 * eq19_e305_d_n8);
        let eq19_e306_d_n9: f64 = (p.p3 * eq19_e305_d_n9);
        let eq19_e306_d_n10: f64 = (p.p3 * eq19_e305_d_n10);
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
        let eq19_value: f64 = eq19_e308;
        let eq19_node_derivatives: [f64; 11] = [eq19_e308_d_n0, eq19_e308_d_n1, eq19_e308_d_n2, eq19_e308_d_n3, eq19_e308_d_n4, eq19_e308_d_n5, eq19_e308_d_n6, eq19_e308_d_n7, eq19_e308_d_n8, eq19_e308_d_n9, eq19_e308_d_n10];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
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
        let eq20_e312: f64 = (s.v[216] + s.v[229]);
        let eq20_e312_d_n0: f64 = (s.dn[216][0] + s.dn[229][0]);
        let eq20_e312_d_n1: f64 = (s.dn[216][1] + s.dn[229][1]);
        let eq20_e312_d_n2: f64 = (s.dn[216][2] + s.dn[229][2]);
        let eq20_e312_d_n3: f64 = (s.dn[216][3] + s.dn[229][3]);
        let eq20_e312_d_n4: f64 = (s.dn[216][4] + s.dn[229][4]);
        let eq20_e312_d_n5: f64 = (s.dn[216][5] + s.dn[229][5]);
        let eq20_e312_d_n6: f64 = (s.dn[216][6] + s.dn[229][6]);
        let eq20_e312_d_n7: f64 = (s.dn[216][7] + s.dn[229][7]);
        let eq20_e312_d_n8: f64 = (s.dn[216][8] + s.dn[229][8]);
        let eq20_e312_d_n9: f64 = (s.dn[216][9] + s.dn[229][9]);
        let eq20_e312_d_n10: f64 = (s.dn[216][10] + s.dn[229][10]);
        let eq20_e313: f64 = (p.p3 * eq20_e312);
        let eq20_e313_d_n0: f64 = (p.p3 * eq20_e312_d_n0);
        let eq20_e313_d_n1: f64 = (p.p3 * eq20_e312_d_n1);
        let eq20_e313_d_n2: f64 = (p.p3 * eq20_e312_d_n2);
        let eq20_e313_d_n3: f64 = (p.p3 * eq20_e312_d_n3);
        let eq20_e313_d_n4: f64 = (p.p3 * eq20_e312_d_n4);
        let eq20_e313_d_n5: f64 = (p.p3 * eq20_e312_d_n5);
        let eq20_e313_d_n6: f64 = (p.p3 * eq20_e312_d_n6);
        let eq20_e313_d_n7: f64 = (p.p3 * eq20_e312_d_n7);
        let eq20_e313_d_n8: f64 = (p.p3 * eq20_e312_d_n8);
        let eq20_e313_d_n9: f64 = (p.p3 * eq20_e312_d_n9);
        let eq20_e313_d_n10: f64 = (p.p3 * eq20_e312_d_n10);
        let eq20_e314: f64 = self.eval_ddt(7, eq20_e313);
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
        let eq20_value: f64 = eq20_e316;
        let eq20_node_derivatives: [f64; 11] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
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
        let (eq21_e326, eq21_e326_d_n0, eq21_e326_d_n1, eq21_e326_d_n2, eq21_e326_d_n3, eq21_e326_d_n4, eq21_e326_d_n5, eq21_e326_d_n6, eq21_e326_d_n7, eq21_e326_d_n8, eq21_e326_d_n9, eq21_e326_d_n10,) = {
    if (s.v[553] != 0.0) {
        let eq21_e320: f64 = (p.p3 * s.v[237]);
        let eq21_e320_d_n0: f64 = (p.p3 * s.dn[237][0]);
        let eq21_e320_d_n1: f64 = (p.p3 * s.dn[237][1]);
        let eq21_e320_d_n2: f64 = (p.p3 * s.dn[237][2]);
        let eq21_e320_d_n3: f64 = (p.p3 * s.dn[237][3]);
        let eq21_e320_d_n4: f64 = (p.p3 * s.dn[237][4]);
        let eq21_e320_d_n5: f64 = (p.p3 * s.dn[237][5]);
        let eq21_e320_d_n6: f64 = (p.p3 * s.dn[237][6]);
        let eq21_e320_d_n7: f64 = (p.p3 * s.dn[237][7]);
        let eq21_e320_d_n8: f64 = (p.p3 * s.dn[237][8]);
        let eq21_e320_d_n9: f64 = (p.p3 * s.dn[237][9]);
        let eq21_e320_d_n10: f64 = (p.p3 * s.dn[237][10]);
        let eq21_e322: f64 = (eq21_e320 * s.v[102]);
        let eq21_e322_d_n0: f64 = ((eq21_e320_d_n0 * s.v[102]) + (eq21_e320 * s.dn[102][0]));
        let eq21_e322_d_n1: f64 = ((eq21_e320_d_n1 * s.v[102]) + (eq21_e320 * s.dn[102][1]));
        let eq21_e322_d_n2: f64 = ((eq21_e320_d_n2 * s.v[102]) + (eq21_e320 * s.dn[102][2]));
        let eq21_e322_d_n3: f64 = ((eq21_e320_d_n3 * s.v[102]) + (eq21_e320 * s.dn[102][3]));
        let eq21_e322_d_n4: f64 = ((eq21_e320_d_n4 * s.v[102]) + (eq21_e320 * s.dn[102][4]));
        let eq21_e322_d_n5: f64 = ((eq21_e320_d_n5 * s.v[102]) + (eq21_e320 * s.dn[102][5]));
        let eq21_e322_d_n6: f64 = ((eq21_e320_d_n6 * s.v[102]) + (eq21_e320 * s.dn[102][6]));
        let eq21_e322_d_n7: f64 = ((eq21_e320_d_n7 * s.v[102]) + (eq21_e320 * s.dn[102][7]));
        let eq21_e322_d_n8: f64 = ((eq21_e320_d_n8 * s.v[102]) + (eq21_e320 * s.dn[102][8]));
        let eq21_e322_d_n9: f64 = ((eq21_e320_d_n9 * s.v[102]) + (eq21_e320 * s.dn[102][9]));
        let eq21_e322_d_n10: f64 = ((eq21_e320_d_n10 * s.v[102]) + (eq21_e320 * s.dn[102][10]));
        let eq21_e324: f64 = (eq21_e322 * p.p1);
        let eq21_e324_d_n0: f64 = (eq21_e322_d_n0 * p.p1);
        let eq21_e324_d_n1: f64 = (eq21_e322_d_n1 * p.p1);
        let eq21_e324_d_n2: f64 = (eq21_e322_d_n2 * p.p1);
        let eq21_e324_d_n3: f64 = (eq21_e322_d_n3 * p.p1);
        let eq21_e324_d_n4: f64 = (eq21_e322_d_n4 * p.p1);
        let eq21_e324_d_n5: f64 = (eq21_e322_d_n5 * p.p1);
        let eq21_e324_d_n6: f64 = (eq21_e322_d_n6 * p.p1);
        let eq21_e324_d_n7: f64 = (eq21_e322_d_n7 * p.p1);
        let eq21_e324_d_n8: f64 = (eq21_e322_d_n8 * p.p1);
        let eq21_e324_d_n9: f64 = (eq21_e322_d_n9 * p.p1);
        let eq21_e324_d_n10: f64 = (eq21_e322_d_n10 * p.p1);
        (eq21_e324, eq21_e324_d_n0, eq21_e324_d_n1, eq21_e324_d_n2, eq21_e324_d_n3, eq21_e324_d_n4, eq21_e324_d_n5, eq21_e324_d_n6, eq21_e324_d_n7, eq21_e324_d_n8, eq21_e324_d_n9, eq21_e324_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e326;
        let eq21_node_derivatives: [f64; 11] = [eq21_e326_d_n0, eq21_e326_d_n1, eq21_e326_d_n2, eq21_e326_d_n3, eq21_e326_d_n4, eq21_e326_d_n5, eq21_e326_d_n6, eq21_e326_d_n7, eq21_e326_d_n8, eq21_e326_d_n9, eq21_e326_d_n10];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
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
        let (eq22_e331,) = {
    if (!(s.v[553] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e331;
        stamper.stamp_potential(
            branches[0],
            eq22_value,
            &[
            ],
        );
    }

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
        let (eq23_e341, eq23_e341_d_n0, eq23_e341_d_n1, eq23_e341_d_n2, eq23_e341_d_n3, eq23_e341_d_n4, eq23_e341_d_n5, eq23_e341_d_n6, eq23_e341_d_n7, eq23_e341_d_n8, eq23_e341_d_n9, eq23_e341_d_n10,) = {
    if (s.v[554] != 0.0) {
        let eq23_e335: f64 = (p.p3 * s.v[238]);
        let eq23_e335_d_n0: f64 = (p.p3 * s.dn[238][0]);
        let eq23_e335_d_n1: f64 = (p.p3 * s.dn[238][1]);
        let eq23_e335_d_n2: f64 = (p.p3 * s.dn[238][2]);
        let eq23_e335_d_n3: f64 = (p.p3 * s.dn[238][3]);
        let eq23_e335_d_n4: f64 = (p.p3 * s.dn[238][4]);
        let eq23_e335_d_n5: f64 = (p.p3 * s.dn[238][5]);
        let eq23_e335_d_n6: f64 = (p.p3 * s.dn[238][6]);
        let eq23_e335_d_n7: f64 = (p.p3 * s.dn[238][7]);
        let eq23_e335_d_n8: f64 = (p.p3 * s.dn[238][8]);
        let eq23_e335_d_n9: f64 = (p.p3 * s.dn[238][9]);
        let eq23_e335_d_n10: f64 = (p.p3 * s.dn[238][10]);
        let eq23_e337: f64 = (eq23_e335 * s.v[103]);
        let eq23_e337_d_n0: f64 = ((eq23_e335_d_n0 * s.v[103]) + (eq23_e335 * s.dn[103][0]));
        let eq23_e337_d_n1: f64 = ((eq23_e335_d_n1 * s.v[103]) + (eq23_e335 * s.dn[103][1]));
        let eq23_e337_d_n2: f64 = ((eq23_e335_d_n2 * s.v[103]) + (eq23_e335 * s.dn[103][2]));
        let eq23_e337_d_n3: f64 = ((eq23_e335_d_n3 * s.v[103]) + (eq23_e335 * s.dn[103][3]));
        let eq23_e337_d_n4: f64 = ((eq23_e335_d_n4 * s.v[103]) + (eq23_e335 * s.dn[103][4]));
        let eq23_e337_d_n5: f64 = ((eq23_e335_d_n5 * s.v[103]) + (eq23_e335 * s.dn[103][5]));
        let eq23_e337_d_n6: f64 = ((eq23_e335_d_n6 * s.v[103]) + (eq23_e335 * s.dn[103][6]));
        let eq23_e337_d_n7: f64 = ((eq23_e335_d_n7 * s.v[103]) + (eq23_e335 * s.dn[103][7]));
        let eq23_e337_d_n8: f64 = ((eq23_e335_d_n8 * s.v[103]) + (eq23_e335 * s.dn[103][8]));
        let eq23_e337_d_n9: f64 = ((eq23_e335_d_n9 * s.v[103]) + (eq23_e335 * s.dn[103][9]));
        let eq23_e337_d_n10: f64 = ((eq23_e335_d_n10 * s.v[103]) + (eq23_e335 * s.dn[103][10]));
        let eq23_e339: f64 = (eq23_e337 * p.p1);
        let eq23_e339_d_n0: f64 = (eq23_e337_d_n0 * p.p1);
        let eq23_e339_d_n1: f64 = (eq23_e337_d_n1 * p.p1);
        let eq23_e339_d_n2: f64 = (eq23_e337_d_n2 * p.p1);
        let eq23_e339_d_n3: f64 = (eq23_e337_d_n3 * p.p1);
        let eq23_e339_d_n4: f64 = (eq23_e337_d_n4 * p.p1);
        let eq23_e339_d_n5: f64 = (eq23_e337_d_n5 * p.p1);
        let eq23_e339_d_n6: f64 = (eq23_e337_d_n6 * p.p1);
        let eq23_e339_d_n7: f64 = (eq23_e337_d_n7 * p.p1);
        let eq23_e339_d_n8: f64 = (eq23_e337_d_n8 * p.p1);
        let eq23_e339_d_n9: f64 = (eq23_e337_d_n9 * p.p1);
        let eq23_e339_d_n10: f64 = (eq23_e337_d_n10 * p.p1);
        (eq23_e339, eq23_e339_d_n0, eq23_e339_d_n1, eq23_e339_d_n2, eq23_e339_d_n3, eq23_e339_d_n4, eq23_e339_d_n5, eq23_e339_d_n6, eq23_e339_d_n7, eq23_e339_d_n8, eq23_e339_d_n9, eq23_e339_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e341;
        let eq23_node_derivatives: [f64; 11] = [eq23_e341_d_n0, eq23_e341_d_n1, eq23_e341_d_n2, eq23_e341_d_n3, eq23_e341_d_n4, eq23_e341_d_n5, eq23_e341_d_n6, eq23_e341_d_n7, eq23_e341_d_n8, eq23_e341_d_n9, eq23_e341_d_n10];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            self.multiplicity * (eq23_value),
            &nodes,
            &eq23_node_derivatives,
            &branches,
            &eq23_branch_derivatives,
            self.multiplicity,
        );
    }
}
