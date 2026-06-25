#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq12_e185, eq12_e185_d_n3, eq12_e185_d_n7,) = {
    if ((!(s.v[122] != 0.0)) && (s.v[123] != 0.0)) {
        let eq12_e183: f64 = ((nv3 - nv7) / p.p33);
        let eq12_e183_d_n3: f64 = (1.0 / p.p33);
        let eq12_e183_d_n7: f64 = (-1.0 / p.p33);
        (eq12_e183, eq12_e183_d_n3, eq12_e183_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e185;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[7]),
            self.multiplicity * (eq12_value),
            &[
                GeneratedDerivative::node(nodes[3], self.multiplicity * eq12_e185_d_n3),
                GeneratedDerivative::node(nodes[7], self.multiplicity * eq12_e185_d_n7),
            ],
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let (eq13_e195, eq13_e195_d_n0, eq13_e195_d_n1, eq13_e195_d_n2, eq13_e195_d_n3, eq13_e195_d_n4, eq13_e195_d_n5, eq13_e195_d_n6, eq13_e195_d_n7, eq13_e195_d_n8, eq13_e195_d_n9, eq13_e195_d_b0, eq13_e195_d_b1, eq13_e195_d_b2, eq13_e195_d_b3, eq13_e195_d_b4, eq13_e195_d_b5, eq13_e195_d_b6, eq13_e195_d_b7,) = {
    if ((!(s.v[122] != 0.0)) && (s.v[123] != 0.0)) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e192_d_n3: f64 = p.p34;
        let eq13_e193: f64 = self.eval_ddt(3, eq13_e192);
        let eq13_e193_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_n3: f64 = self.ddt_jacobian(eq13_e192_d_n3);
        let eq13_e193_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq13_e193_d_b7: f64 = self.ddt_jacobian(0.0);
        (eq13_e193, eq13_e193_d_n0, eq13_e193_d_n1, eq13_e193_d_n2, eq13_e193_d_n3, eq13_e193_d_n4, eq13_e193_d_n5, eq13_e193_d_n6, eq13_e193_d_n7, eq13_e193_d_n8, eq13_e193_d_n9, eq13_e193_d_b0, eq13_e193_d_b1, eq13_e193_d_b2, eq13_e193_d_b3, eq13_e193_d_b4, eq13_e193_d_b5, eq13_e193_d_b6, eq13_e193_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e195;
        let eq13_node_derivatives: [f64; 10] = [eq13_e195_d_n0, eq13_e195_d_n1, eq13_e195_d_n2, eq13_e195_d_n3, eq13_e195_d_n4, eq13_e195_d_n5, eq13_e195_d_n6, eq13_e195_d_n7, eq13_e195_d_n8, eq13_e195_d_n9];
        let eq13_branch_derivatives: [f64; 8] = [eq13_e195_d_b0, eq13_e195_d_b1, eq13_e195_d_b2, eq13_e195_d_b3, eq13_e195_d_b4, eq13_e195_d_b5, eq13_e195_d_b6, eq13_e195_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq14_e204, eq14_e204_d_n7,) = {
    if ((!(s.v[122] != 0.0)) && (s.v[123] != 0.0)) {
        let eq14_e202: f64 = ((nv7 - 0.0) / p.p35);
        let eq14_e202_d_n7: f64 = (1.0 / p.p35);
        (eq14_e202, eq14_e202_d_n7,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e204;
        stamper.stamp_current(
            Some(nodes[7]),
            None,
            self.multiplicity * (eq14_value),
            &[
                GeneratedDerivative::node(nodes[7], self.multiplicity * eq14_e204_d_n7),
            ],
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq15_e214, eq15_e214_d_n0, eq15_e214_d_n1, eq15_e214_d_n2, eq15_e214_d_n3, eq15_e214_d_n4, eq15_e214_d_n5, eq15_e214_d_n6, eq15_e214_d_n7, eq15_e214_d_n8, eq15_e214_d_n9, eq15_e214_d_b0, eq15_e214_d_b1, eq15_e214_d_b2, eq15_e214_d_b3, eq15_e214_d_b4, eq15_e214_d_b5, eq15_e214_d_b6, eq15_e214_d_b7,) = {
    if ((!(s.v[122] != 0.0)) && (s.v[123] != 0.0)) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e211_d_n7: f64 = p.p36;
        let eq15_e212: f64 = self.eval_ddt(4, eq15_e211);
        let eq15_e212_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_n7: f64 = self.ddt_jacobian(eq15_e211_d_n7);
        let eq15_e212_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq15_e212_d_b7: f64 = self.ddt_jacobian(0.0);
        (eq15_e212, eq15_e212_d_n0, eq15_e212_d_n1, eq15_e212_d_n2, eq15_e212_d_n3, eq15_e212_d_n4, eq15_e212_d_n5, eq15_e212_d_n6, eq15_e212_d_n7, eq15_e212_d_n8, eq15_e212_d_n9, eq15_e212_d_b0, eq15_e212_d_b1, eq15_e212_d_b2, eq15_e212_d_b3, eq15_e212_d_b4, eq15_e212_d_b5, eq15_e212_d_b6, eq15_e212_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e214;
        let eq15_node_derivatives: [f64; 10] = [eq15_e214_d_n0, eq15_e214_d_n1, eq15_e214_d_n2, eq15_e214_d_n3, eq15_e214_d_n4, eq15_e214_d_n5, eq15_e214_d_n6, eq15_e214_d_n7, eq15_e214_d_n8, eq15_e214_d_n9];
        let eq15_branch_derivatives: [f64; 8] = [eq15_e214_d_b0, eq15_e214_d_b1, eq15_e214_d_b2, eq15_e214_d_b3, eq15_e214_d_b4, eq15_e214_d_b5, eq15_e214_d_b6, eq15_e214_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[7]),
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq16_e235, eq16_e235_d_n0, eq16_e235_d_n1, eq16_e235_d_n2, eq16_e235_d_n3, eq16_e235_d_n4, eq16_e235_d_n5, eq16_e235_d_n6, eq16_e235_d_n7, eq16_e235_d_n8, eq16_e235_d_n9, eq16_e235_d_b0, eq16_e235_d_b1, eq16_e235_d_b2, eq16_e235_d_b3, eq16_e235_d_b4, eq16_e235_d_b5, eq16_e235_d_b6, eq16_e235_d_b7,) = {
    if (((!(s.v[122] != 0.0)) && (!(s.v[123] != 0.0))) && (s.v[124] != 0.0)) {
        let eq16_e223: f64 = (-1.0);
        let eq16_e226: f64 = (s.v[37] * (nv1 - nv2));
        let eq16_e226_d_n0: f64 = (s.dn[37][0] * (nv1 - nv2));
        let eq16_e226_d_n1: f64 = ((s.dn[37][1] * (nv1 - nv2)) + s.v[37]);
        let eq16_e226_d_n2: f64 = ((s.dn[37][2] * (nv1 - nv2)) + (-s.v[37]));
        let eq16_e226_d_n3: f64 = (s.dn[37][3] * (nv1 - nv2));
        let eq16_e226_d_n4: f64 = (s.dn[37][4] * (nv1 - nv2));
        let eq16_e226_d_n5: f64 = (s.dn[37][5] * (nv1 - nv2));
        let eq16_e226_d_n6: f64 = (s.dn[37][6] * (nv1 - nv2));
        let eq16_e226_d_n7: f64 = (s.dn[37][7] * (nv1 - nv2));
        let eq16_e226_d_n8: f64 = (s.dn[37][8] * (nv1 - nv2));
        let eq16_e226_d_n9: f64 = (s.dn[37][9] * (nv1 - nv2));
        let eq16_e226_d_b0: f64 = (s.db[37][0] * (nv1 - nv2));
        let eq16_e226_d_b1: f64 = (s.db[37][1] * (nv1 - nv2));
        let eq16_e226_d_b2: f64 = (s.db[37][2] * (nv1 - nv2));
        let eq16_e226_d_b3: f64 = (s.db[37][3] * (nv1 - nv2));
        let eq16_e226_d_b4: f64 = (s.db[37][4] * (nv1 - nv2));
        let eq16_e226_d_b5: f64 = (s.db[37][5] * (nv1 - nv2));
        let eq16_e226_d_b6: f64 = (s.db[37][6] * (nv1 - nv2));
        let eq16_e226_d_b7: f64 = (s.db[37][7] * (nv1 - nv2));
        let eq16_e227: f64 = (eq16_e226).abs();
        let eq16_e227_d_n0: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n0 } else { (-eq16_e226_d_n0) };
        let eq16_e227_d_n1: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n1 } else { (-eq16_e226_d_n1) };
        let eq16_e227_d_n2: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n2 } else { (-eq16_e226_d_n2) };
        let eq16_e227_d_n3: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n3 } else { (-eq16_e226_d_n3) };
        let eq16_e227_d_n4: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n4 } else { (-eq16_e226_d_n4) };
        let eq16_e227_d_n5: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n5 } else { (-eq16_e226_d_n5) };
        let eq16_e227_d_n6: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n6 } else { (-eq16_e226_d_n6) };
        let eq16_e227_d_n7: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n7 } else { (-eq16_e226_d_n7) };
        let eq16_e227_d_n8: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n8 } else { (-eq16_e226_d_n8) };
        let eq16_e227_d_n9: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n9 } else { (-eq16_e226_d_n9) };
        let eq16_e227_d_b0: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b0 } else { (-eq16_e226_d_b0) };
        let eq16_e227_d_b1: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b1 } else { (-eq16_e226_d_b1) };
        let eq16_e227_d_b2: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b2 } else { (-eq16_e226_d_b2) };
        let eq16_e227_d_b3: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b3 } else { (-eq16_e226_d_b3) };
        let eq16_e227_d_b4: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b4 } else { (-eq16_e226_d_b4) };
        let eq16_e227_d_b5: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b5 } else { (-eq16_e226_d_b5) };
        let eq16_e227_d_b6: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b6 } else { (-eq16_e226_d_b6) };
        let eq16_e227_d_b7: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b7 } else { (-eq16_e226_d_b7) };
        let eq16_e228: f64 = (eq16_e223 * eq16_e227);
        let eq16_e228_d_n0: f64 = (eq16_e223 * eq16_e227_d_n0);
        let eq16_e228_d_n1: f64 = (eq16_e223 * eq16_e227_d_n1);
        let eq16_e228_d_n2: f64 = (eq16_e223 * eq16_e227_d_n2);
        let eq16_e228_d_n3: f64 = (eq16_e223 * eq16_e227_d_n3);
        let eq16_e228_d_n4: f64 = (eq16_e223 * eq16_e227_d_n4);
        let eq16_e228_d_n5: f64 = (eq16_e223 * eq16_e227_d_n5);
        let eq16_e228_d_n6: f64 = (eq16_e223 * eq16_e227_d_n6);
        let eq16_e228_d_n7: f64 = (eq16_e223 * eq16_e227_d_n7);
        let eq16_e228_d_n8: f64 = (eq16_e223 * eq16_e227_d_n8);
        let eq16_e228_d_n9: f64 = (eq16_e223 * eq16_e227_d_n9);
        let eq16_e228_d_b0: f64 = (eq16_e223 * eq16_e227_d_b0);
        let eq16_e228_d_b1: f64 = (eq16_e223 * eq16_e227_d_b1);
        let eq16_e228_d_b2: f64 = (eq16_e223 * eq16_e227_d_b2);
        let eq16_e228_d_b3: f64 = (eq16_e223 * eq16_e227_d_b3);
        let eq16_e228_d_b4: f64 = (eq16_e223 * eq16_e227_d_b4);
        let eq16_e228_d_b5: f64 = (eq16_e223 * eq16_e227_d_b5);
        let eq16_e228_d_b6: f64 = (eq16_e223 * eq16_e227_d_b6);
        let eq16_e228_d_b7: f64 = (eq16_e223 * eq16_e227_d_b7);
        let eq16_e231: f64 = (s.v[40] * (nv1 - nv0));
        let eq16_e231_d_n0: f64 = ((s.dn[40][0] * (nv1 - nv0)) + (-s.v[40]));
        let eq16_e231_d_n1: f64 = ((s.dn[40][1] * (nv1 - nv0)) + s.v[40]);
        let eq16_e231_d_n2: f64 = (s.dn[40][2] * (nv1 - nv0));
        let eq16_e231_d_n3: f64 = (s.dn[40][3] * (nv1 - nv0));
        let eq16_e231_d_n4: f64 = (s.dn[40][4] * (nv1 - nv0));
        let eq16_e231_d_n5: f64 = (s.dn[40][5] * (nv1 - nv0));
        let eq16_e231_d_n6: f64 = (s.dn[40][6] * (nv1 - nv0));
        let eq16_e231_d_n7: f64 = (s.dn[40][7] * (nv1 - nv0));
        let eq16_e231_d_n8: f64 = (s.dn[40][8] * (nv1 - nv0));
        let eq16_e231_d_n9: f64 = (s.dn[40][9] * (nv1 - nv0));
        let eq16_e231_d_b0: f64 = (s.db[40][0] * (nv1 - nv0));
        let eq16_e231_d_b1: f64 = (s.db[40][1] * (nv1 - nv0));
        let eq16_e231_d_b2: f64 = (s.db[40][2] * (nv1 - nv0));
        let eq16_e231_d_b3: f64 = (s.db[40][3] * (nv1 - nv0));
        let eq16_e231_d_b4: f64 = (s.db[40][4] * (nv1 - nv0));
        let eq16_e231_d_b5: f64 = (s.db[40][5] * (nv1 - nv0));
        let eq16_e231_d_b6: f64 = (s.db[40][6] * (nv1 - nv0));
        let eq16_e231_d_b7: f64 = (s.db[40][7] * (nv1 - nv0));
        let eq16_e232: f64 = (eq16_e231).abs();
        let eq16_e232_d_n0: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n0 } else { (-eq16_e231_d_n0) };
        let eq16_e232_d_n1: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n1 } else { (-eq16_e231_d_n1) };
        let eq16_e232_d_n2: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n2 } else { (-eq16_e231_d_n2) };
        let eq16_e232_d_n3: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n3 } else { (-eq16_e231_d_n3) };
        let eq16_e232_d_n4: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n4 } else { (-eq16_e231_d_n4) };
        let eq16_e232_d_n5: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n5 } else { (-eq16_e231_d_n5) };
        let eq16_e232_d_n6: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n6 } else { (-eq16_e231_d_n6) };
        let eq16_e232_d_n7: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n7 } else { (-eq16_e231_d_n7) };
        let eq16_e232_d_n8: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n8 } else { (-eq16_e231_d_n8) };
        let eq16_e232_d_n9: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n9 } else { (-eq16_e231_d_n9) };
        let eq16_e232_d_b0: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b0 } else { (-eq16_e231_d_b0) };
        let eq16_e232_d_b1: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b1 } else { (-eq16_e231_d_b1) };
        let eq16_e232_d_b2: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b2 } else { (-eq16_e231_d_b2) };
        let eq16_e232_d_b3: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b3 } else { (-eq16_e231_d_b3) };
        let eq16_e232_d_b4: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b4 } else { (-eq16_e231_d_b4) };
        let eq16_e232_d_b5: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b5 } else { (-eq16_e231_d_b5) };
        let eq16_e232_d_b6: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b6 } else { (-eq16_e231_d_b6) };
        let eq16_e232_d_b7: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b7 } else { (-eq16_e231_d_b7) };
        let eq16_e233: f64 = (eq16_e228 - eq16_e232);
        let eq16_e233_d_n0: f64 = (eq16_e228_d_n0 - eq16_e232_d_n0);
        let eq16_e233_d_n1: f64 = (eq16_e228_d_n1 - eq16_e232_d_n1);
        let eq16_e233_d_n2: f64 = (eq16_e228_d_n2 - eq16_e232_d_n2);
        let eq16_e233_d_n3: f64 = (eq16_e228_d_n3 - eq16_e232_d_n3);
        let eq16_e233_d_n4: f64 = (eq16_e228_d_n4 - eq16_e232_d_n4);
        let eq16_e233_d_n5: f64 = (eq16_e228_d_n5 - eq16_e232_d_n5);
        let eq16_e233_d_n6: f64 = (eq16_e228_d_n6 - eq16_e232_d_n6);
        let eq16_e233_d_n7: f64 = (eq16_e228_d_n7 - eq16_e232_d_n7);
        let eq16_e233_d_n8: f64 = (eq16_e228_d_n8 - eq16_e232_d_n8);
        let eq16_e233_d_n9: f64 = (eq16_e228_d_n9 - eq16_e232_d_n9);
        let eq16_e233_d_b0: f64 = (eq16_e228_d_b0 - eq16_e232_d_b0);
        let eq16_e233_d_b1: f64 = (eq16_e228_d_b1 - eq16_e232_d_b1);
        let eq16_e233_d_b2: f64 = (eq16_e228_d_b2 - eq16_e232_d_b2);
        let eq16_e233_d_b3: f64 = (eq16_e228_d_b3 - eq16_e232_d_b3);
        let eq16_e233_d_b4: f64 = (eq16_e228_d_b4 - eq16_e232_d_b4);
        let eq16_e233_d_b5: f64 = (eq16_e228_d_b5 - eq16_e232_d_b5);
        let eq16_e233_d_b6: f64 = (eq16_e228_d_b6 - eq16_e232_d_b6);
        let eq16_e233_d_b7: f64 = (eq16_e228_d_b7 - eq16_e232_d_b7);
        (eq16_e233, eq16_e233_d_n0, eq16_e233_d_n1, eq16_e233_d_n2, eq16_e233_d_n3, eq16_e233_d_n4, eq16_e233_d_n5, eq16_e233_d_n6, eq16_e233_d_n7, eq16_e233_d_n8, eq16_e233_d_n9, eq16_e233_d_b0, eq16_e233_d_b1, eq16_e233_d_b2, eq16_e233_d_b3, eq16_e233_d_b4, eq16_e233_d_b5, eq16_e233_d_b6, eq16_e233_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e235;
        let eq16_node_derivatives: [f64; 10] = [eq16_e235_d_n0, eq16_e235_d_n1, eq16_e235_d_n2, eq16_e235_d_n3, eq16_e235_d_n4, eq16_e235_d_n5, eq16_e235_d_n6, eq16_e235_d_n7, eq16_e235_d_n8, eq16_e235_d_n9];
        let eq16_branch_derivatives: [f64; 8] = [eq16_e235_d_b0, eq16_e235_d_b1, eq16_e235_d_b2, eq16_e235_d_b3, eq16_e235_d_b4, eq16_e235_d_b5, eq16_e235_d_b6, eq16_e235_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[3]),
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
        let (eq17_e245,) = {
    if (((!(s.v[122] != 0.0)) && (!(s.v[123] != 0.0))) && (s.v[124] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e245;
        stamper.stamp_potential(
            branches[2],
            eq17_value,
            &[
            ],
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
        let (eq18_e256,) = {
    if (((!(s.v[122] != 0.0)) && (!(s.v[123] != 0.0))) && (!(s.v[124] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e256;
        stamper.stamp_potential(
            branches[3],
            eq18_value,
            &[
            ],
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
        let (eq19_e267,) = {
    if (((!(s.v[122] != 0.0)) && (!(s.v[123] != 0.0))) && (!(s.v[124] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e267;
        stamper.stamp_potential(
            branches[4],
            eq19_value,
            &[
            ],
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let eq20_e270: f64 = 0.0;
        let eq20_e272: f64 = (eq20_e270 * (nv5 - nv6));
        let eq20_e272_d_n6: f64 = (-eq20_e270);
        let eq20_value: f64 = eq20_e272;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq20_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq20_e270),
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq20_e272_d_n6),
            ],
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq21_e275: f64 = 0.0;
        let eq21_e277: f64 = (eq21_e275 * (nv5 - nv4));
        let eq21_e277_d_n4: f64 = (-eq21_e275);
        let eq21_value: f64 = eq21_e277;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[4]),
            self.multiplicity * (eq21_value),
            &[
                GeneratedDerivative::node(nodes[4], self.multiplicity * eq21_e277_d_n4),
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq21_e275),
            ],
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let eq22_e280: f64 = 0.0;
        let eq22_e282: f64 = (eq22_e280 * (nv4 - nv6));
        let eq22_e282_d_n6: f64 = (-eq22_e280);
        let eq22_value: f64 = eq22_e282;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[6]),
            self.multiplicity * (eq22_value),
            &[
                GeneratedDerivative::node(nodes[4], self.multiplicity * eq22_e280),
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq22_e282_d_n6),
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq23_e297, eq23_e297_d_n0, eq23_e297_d_n1, eq23_e297_d_n2, eq23_e297_d_n3, eq23_e297_d_n4, eq23_e297_d_n5, eq23_e297_d_n6, eq23_e297_d_n7, eq23_e297_d_n8, eq23_e297_d_n9, eq23_e297_d_b0, eq23_e297_d_b1, eq23_e297_d_b2, eq23_e297_d_b3, eq23_e297_d_b4, eq23_e297_d_b5, eq23_e297_d_b6, eq23_e297_d_b7,) = {
    if (s.v[125] != 0.0) {
        let eq23_e287: f64 = (s.v[51] / s.v[3]);
        let eq23_e287_d_n0: f64 = (s.dn[51][0] / s.v[3]);
        let eq23_e287_d_n1: f64 = (s.dn[51][1] / s.v[3]);
        let eq23_e287_d_n2: f64 = (s.dn[51][2] / s.v[3]);
        let eq23_e287_d_n3: f64 = (s.dn[51][3] / s.v[3]);
        let eq23_e287_d_n4: f64 = (s.dn[51][4] / s.v[3]);
        let eq23_e287_d_n5: f64 = (s.dn[51][5] / s.v[3]);
        let eq23_e287_d_n6: f64 = (s.dn[51][6] / s.v[3]);
        let eq23_e287_d_n7: f64 = (s.dn[51][7] / s.v[3]);
        let eq23_e287_d_n8: f64 = (s.dn[51][8] / s.v[3]);
        let eq23_e287_d_n9: f64 = (s.dn[51][9] / s.v[3]);
        let eq23_e287_d_b0: f64 = (s.db[51][0] / s.v[3]);
        let eq23_e287_d_b1: f64 = (s.db[51][1] / s.v[3]);
        let eq23_e287_d_b2: f64 = (s.db[51][2] / s.v[3]);
        let eq23_e287_d_b3: f64 = (s.db[51][3] / s.v[3]);
        let eq23_e287_d_b4: f64 = (s.db[51][4] / s.v[3]);
        let eq23_e287_d_b5: f64 = (s.db[51][5] / s.v[3]);
        let eq23_e287_d_b6: f64 = (s.db[51][6] / s.v[3]);
        let eq23_e287_d_b7: f64 = (s.db[51][7] / s.v[3]);
        let (eq23_e294, eq23_e294_d_n0, eq23_e294_d_n1, eq23_e294_d_n2, eq23_e294_d_n3, eq23_e294_d_n4, eq23_e294_d_n5, eq23_e294_d_n6, eq23_e294_d_n7, eq23_e294_d_n8, eq23_e294_d_n9, eq23_e294_d_b0, eq23_e294_d_b1, eq23_e294_d_b2, eq23_e294_d_b3, eq23_e294_d_b4, eq23_e294_d_b5, eq23_e294_d_b6, eq23_e294_d_b7,) = {
            if (eq23_e287 > p.p46) {
                let eq23_e292: f64 = (s.v[51] / s.v[3]);
                let eq23_e292_d_n0: f64 = (s.dn[51][0] / s.v[3]);
                let eq23_e292_d_n1: f64 = (s.dn[51][1] / s.v[3]);
                let eq23_e292_d_n2: f64 = (s.dn[51][2] / s.v[3]);
                let eq23_e292_d_n3: f64 = (s.dn[51][3] / s.v[3]);
                let eq23_e292_d_n4: f64 = (s.dn[51][4] / s.v[3]);
                let eq23_e292_d_n5: f64 = (s.dn[51][5] / s.v[3]);
                let eq23_e292_d_n6: f64 = (s.dn[51][6] / s.v[3]);
                let eq23_e292_d_n7: f64 = (s.dn[51][7] / s.v[3]);
                let eq23_e292_d_n8: f64 = (s.dn[51][8] / s.v[3]);
                let eq23_e292_d_n9: f64 = (s.dn[51][9] / s.v[3]);
                let eq23_e292_d_b0: f64 = (s.db[51][0] / s.v[3]);
                let eq23_e292_d_b1: f64 = (s.db[51][1] / s.v[3]);
                let eq23_e292_d_b2: f64 = (s.db[51][2] / s.v[3]);
                let eq23_e292_d_b3: f64 = (s.db[51][3] / s.v[3]);
                let eq23_e292_d_b4: f64 = (s.db[51][4] / s.v[3]);
                let eq23_e292_d_b5: f64 = (s.db[51][5] / s.v[3]);
                let eq23_e292_d_b6: f64 = (s.db[51][6] / s.v[3]);
                let eq23_e292_d_b7: f64 = (s.db[51][7] / s.v[3]);
                (eq23_e292, eq23_e292_d_n0, eq23_e292_d_n1, eq23_e292_d_n2, eq23_e292_d_n3, eq23_e292_d_n4, eq23_e292_d_n5, eq23_e292_d_n6, eq23_e292_d_n7, eq23_e292_d_n8, eq23_e292_d_n9, eq23_e292_d_b0, eq23_e292_d_b1, eq23_e292_d_b2, eq23_e292_d_b3, eq23_e292_d_b4, eq23_e292_d_b5, eq23_e292_d_b6, eq23_e292_d_b7,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq23_e295: f64 = ((nv1 - nv5) / eq23_e294);
        let eq23_e295_d_n0: f64 = (-(((nv1 - nv5) * eq23_e294_d_n0) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n1: f64 = ((eq23_e294 - ((nv1 - nv5) * eq23_e294_d_n1)) / (eq23_e294 * eq23_e294));
        let eq23_e295_d_n2: f64 = (-(((nv1 - nv5) * eq23_e294_d_n2) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n3: f64 = (-(((nv1 - nv5) * eq23_e294_d_n3) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n4: f64 = (-(((nv1 - nv5) * eq23_e294_d_n4) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n5: f64 = (((-eq23_e294) - ((nv1 - nv5) * eq23_e294_d_n5)) / (eq23_e294 * eq23_e294));
        let eq23_e295_d_n6: f64 = (-(((nv1 - nv5) * eq23_e294_d_n6) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n7: f64 = (-(((nv1 - nv5) * eq23_e294_d_n7) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n8: f64 = (-(((nv1 - nv5) * eq23_e294_d_n8) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n9: f64 = (-(((nv1 - nv5) * eq23_e294_d_n9) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b0: f64 = (-(((nv1 - nv5) * eq23_e294_d_b0) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b1: f64 = (-(((nv1 - nv5) * eq23_e294_d_b1) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b2: f64 = (-(((nv1 - nv5) * eq23_e294_d_b2) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b3: f64 = (-(((nv1 - nv5) * eq23_e294_d_b3) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b4: f64 = (-(((nv1 - nv5) * eq23_e294_d_b4) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b5: f64 = (-(((nv1 - nv5) * eq23_e294_d_b5) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b6: f64 = (-(((nv1 - nv5) * eq23_e294_d_b6) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b7: f64 = (-(((nv1 - nv5) * eq23_e294_d_b7) / (eq23_e294 * eq23_e294)));
        (eq23_e295, eq23_e295_d_n0, eq23_e295_d_n1, eq23_e295_d_n2, eq23_e295_d_n3, eq23_e295_d_n4, eq23_e295_d_n5, eq23_e295_d_n6, eq23_e295_d_n7, eq23_e295_d_n8, eq23_e295_d_n9, eq23_e295_d_b0, eq23_e295_d_b1, eq23_e295_d_b2, eq23_e295_d_b3, eq23_e295_d_b4, eq23_e295_d_b5, eq23_e295_d_b6, eq23_e295_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e297;
        let eq23_node_derivatives: [f64; 10] = [eq23_e297_d_n0, eq23_e297_d_n1, eq23_e297_d_n2, eq23_e297_d_n3, eq23_e297_d_n4, eq23_e297_d_n5, eq23_e297_d_n6, eq23_e297_d_n7, eq23_e297_d_n8, eq23_e297_d_n9];
        let eq23_branch_derivatives: [f64; 8] = [eq23_e297_d_b0, eq23_e297_d_b1, eq23_e297_d_b2, eq23_e297_d_b3, eq23_e297_d_b4, eq23_e297_d_b5, eq23_e297_d_b6, eq23_e297_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
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
        let (eq24_e303,) = {
    if (s.v[125] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e303;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq24_value),
            &[
            ],
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
        let (eq25_e308,) = {
    if (!(s.v[125] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e308;
        stamper.stamp_potential(
            branches[5],
            eq25_value,
            &[
            ],
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq26_e323, eq26_e323_d_n0, eq26_e323_d_n1, eq26_e323_d_n2, eq26_e323_d_n3, eq26_e323_d_n4, eq26_e323_d_n5, eq26_e323_d_n6, eq26_e323_d_n7, eq26_e323_d_n8, eq26_e323_d_n9, eq26_e323_d_b0, eq26_e323_d_b1, eq26_e323_d_b2, eq26_e323_d_b3, eq26_e323_d_b4, eq26_e323_d_b5, eq26_e323_d_b6, eq26_e323_d_b7,) = {
    if (s.v[126] != 0.0) {
        let eq26_e313: f64 = (s.v[53] / s.v[3]);
        let eq26_e313_d_n0: f64 = (s.dn[53][0] / s.v[3]);
        let eq26_e313_d_n1: f64 = (s.dn[53][1] / s.v[3]);
        let eq26_e313_d_n2: f64 = (s.dn[53][2] / s.v[3]);
        let eq26_e313_d_n3: f64 = (s.dn[53][3] / s.v[3]);
        let eq26_e313_d_n4: f64 = (s.dn[53][4] / s.v[3]);
        let eq26_e313_d_n5: f64 = (s.dn[53][5] / s.v[3]);
        let eq26_e313_d_n6: f64 = (s.dn[53][6] / s.v[3]);
        let eq26_e313_d_n7: f64 = (s.dn[53][7] / s.v[3]);
        let eq26_e313_d_n8: f64 = (s.dn[53][8] / s.v[3]);
        let eq26_e313_d_n9: f64 = (s.dn[53][9] / s.v[3]);
        let eq26_e313_d_b0: f64 = (s.db[53][0] / s.v[3]);
        let eq26_e313_d_b1: f64 = (s.db[53][1] / s.v[3]);
        let eq26_e313_d_b2: f64 = (s.db[53][2] / s.v[3]);
        let eq26_e313_d_b3: f64 = (s.db[53][3] / s.v[3]);
        let eq26_e313_d_b4: f64 = (s.db[53][4] / s.v[3]);
        let eq26_e313_d_b5: f64 = (s.db[53][5] / s.v[3]);
        let eq26_e313_d_b6: f64 = (s.db[53][6] / s.v[3]);
        let eq26_e313_d_b7: f64 = (s.db[53][7] / s.v[3]);
        let (eq26_e320, eq26_e320_d_n0, eq26_e320_d_n1, eq26_e320_d_n2, eq26_e320_d_n3, eq26_e320_d_n4, eq26_e320_d_n5, eq26_e320_d_n6, eq26_e320_d_n7, eq26_e320_d_n8, eq26_e320_d_n9, eq26_e320_d_b0, eq26_e320_d_b1, eq26_e320_d_b2, eq26_e320_d_b3, eq26_e320_d_b4, eq26_e320_d_b5, eq26_e320_d_b6, eq26_e320_d_b7,) = {
            if (eq26_e313 > p.p46) {
                let eq26_e318: f64 = (s.v[53] / s.v[3]);
                let eq26_e318_d_n0: f64 = (s.dn[53][0] / s.v[3]);
                let eq26_e318_d_n1: f64 = (s.dn[53][1] / s.v[3]);
                let eq26_e318_d_n2: f64 = (s.dn[53][2] / s.v[3]);
                let eq26_e318_d_n3: f64 = (s.dn[53][3] / s.v[3]);
                let eq26_e318_d_n4: f64 = (s.dn[53][4] / s.v[3]);
                let eq26_e318_d_n5: f64 = (s.dn[53][5] / s.v[3]);
                let eq26_e318_d_n6: f64 = (s.dn[53][6] / s.v[3]);
                let eq26_e318_d_n7: f64 = (s.dn[53][7] / s.v[3]);
                let eq26_e318_d_n8: f64 = (s.dn[53][8] / s.v[3]);
                let eq26_e318_d_n9: f64 = (s.dn[53][9] / s.v[3]);
                let eq26_e318_d_b0: f64 = (s.db[53][0] / s.v[3]);
                let eq26_e318_d_b1: f64 = (s.db[53][1] / s.v[3]);
                let eq26_e318_d_b2: f64 = (s.db[53][2] / s.v[3]);
                let eq26_e318_d_b3: f64 = (s.db[53][3] / s.v[3]);
                let eq26_e318_d_b4: f64 = (s.db[53][4] / s.v[3]);
                let eq26_e318_d_b5: f64 = (s.db[53][5] / s.v[3]);
                let eq26_e318_d_b6: f64 = (s.db[53][6] / s.v[3]);
                let eq26_e318_d_b7: f64 = (s.db[53][7] / s.v[3]);
                (eq26_e318, eq26_e318_d_n0, eq26_e318_d_n1, eq26_e318_d_n2, eq26_e318_d_n3, eq26_e318_d_n4, eq26_e318_d_n5, eq26_e318_d_n6, eq26_e318_d_n7, eq26_e318_d_n8, eq26_e318_d_n9, eq26_e318_d_b0, eq26_e318_d_b1, eq26_e318_d_b2, eq26_e318_d_b3, eq26_e318_d_b4, eq26_e318_d_b5, eq26_e318_d_b6, eq26_e318_d_b7,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq26_e321: f64 = ((nv2 - nv6) / eq26_e320);
        let eq26_e321_d_n0: f64 = (-(((nv2 - nv6) * eq26_e320_d_n0) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n1: f64 = (-(((nv2 - nv6) * eq26_e320_d_n1) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n2: f64 = ((eq26_e320 - ((nv2 - nv6) * eq26_e320_d_n2)) / (eq26_e320 * eq26_e320));
        let eq26_e321_d_n3: f64 = (-(((nv2 - nv6) * eq26_e320_d_n3) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n4: f64 = (-(((nv2 - nv6) * eq26_e320_d_n4) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n5: f64 = (-(((nv2 - nv6) * eq26_e320_d_n5) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n6: f64 = (((-eq26_e320) - ((nv2 - nv6) * eq26_e320_d_n6)) / (eq26_e320 * eq26_e320));
        let eq26_e321_d_n7: f64 = (-(((nv2 - nv6) * eq26_e320_d_n7) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n8: f64 = (-(((nv2 - nv6) * eq26_e320_d_n8) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n9: f64 = (-(((nv2 - nv6) * eq26_e320_d_n9) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b0: f64 = (-(((nv2 - nv6) * eq26_e320_d_b0) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b1: f64 = (-(((nv2 - nv6) * eq26_e320_d_b1) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b2: f64 = (-(((nv2 - nv6) * eq26_e320_d_b2) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b3: f64 = (-(((nv2 - nv6) * eq26_e320_d_b3) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b4: f64 = (-(((nv2 - nv6) * eq26_e320_d_b4) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b5: f64 = (-(((nv2 - nv6) * eq26_e320_d_b5) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b6: f64 = (-(((nv2 - nv6) * eq26_e320_d_b6) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b7: f64 = (-(((nv2 - nv6) * eq26_e320_d_b7) / (eq26_e320 * eq26_e320)));
        (eq26_e321, eq26_e321_d_n0, eq26_e321_d_n1, eq26_e321_d_n2, eq26_e321_d_n3, eq26_e321_d_n4, eq26_e321_d_n5, eq26_e321_d_n6, eq26_e321_d_n7, eq26_e321_d_n8, eq26_e321_d_n9, eq26_e321_d_b0, eq26_e321_d_b1, eq26_e321_d_b2, eq26_e321_d_b3, eq26_e321_d_b4, eq26_e321_d_b5, eq26_e321_d_b6, eq26_e321_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e323;
        let eq26_node_derivatives: [f64; 10] = [eq26_e323_d_n0, eq26_e323_d_n1, eq26_e323_d_n2, eq26_e323_d_n3, eq26_e323_d_n4, eq26_e323_d_n5, eq26_e323_d_n6, eq26_e323_d_n7, eq26_e323_d_n8, eq26_e323_d_n9];
        let eq26_branch_derivatives: [f64; 8] = [eq26_e323_d_b0, eq26_e323_d_b1, eq26_e323_d_b2, eq26_e323_d_b3, eq26_e323_d_b4, eq26_e323_d_b5, eq26_e323_d_b6, eq26_e323_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[6]),
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
        let (eq27_e329,) = {
    if (s.v[126] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e329;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[6]),
            self.multiplicity * (eq27_value),
            &[
            ],
        );
    }
}
