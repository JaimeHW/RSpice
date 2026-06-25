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
        let (eq7_e331, eq7_e331_d_n0, eq7_e331_d_n1, eq7_e331_d_n2, eq7_e331_d_n3, eq7_e331_d_n4, eq7_e331_d_n5, eq7_e331_d_n6, eq7_e331_d_n7, eq7_e331_d_n8, eq7_e331_d_n9, eq7_e331_d_n10, eq7_e331_d_n11, eq7_e331_d_n12, eq7_e331_d_n13, eq7_e331_d_n14, eq7_e331_d_n15, eq7_e331_d_n16, eq7_e331_d_n17, eq7_e331_d_n18, eq7_e331_d_n19, eq7_e331_d_n20, eq7_e331_d_n21, eq7_e331_d_n22,) = {
    if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
        let eq7_e329: f64 = (s.v[38] * s.v[38]);
        let eq7_e329_d_n0: f64 = ((s.dn[38][0] * s.v[38]) + (s.v[38] * s.dn[38][0]));
        let eq7_e329_d_n1: f64 = ((s.dn[38][1] * s.v[38]) + (s.v[38] * s.dn[38][1]));
        let eq7_e329_d_n2: f64 = ((s.dn[38][2] * s.v[38]) + (s.v[38] * s.dn[38][2]));
        let eq7_e329_d_n3: f64 = ((s.dn[38][3] * s.v[38]) + (s.v[38] * s.dn[38][3]));
        let eq7_e329_d_n4: f64 = ((s.dn[38][4] * s.v[38]) + (s.v[38] * s.dn[38][4]));
        let eq7_e329_d_n5: f64 = ((s.dn[38][5] * s.v[38]) + (s.v[38] * s.dn[38][5]));
        let eq7_e329_d_n6: f64 = ((s.dn[38][6] * s.v[38]) + (s.v[38] * s.dn[38][6]));
        let eq7_e329_d_n7: f64 = ((s.dn[38][7] * s.v[38]) + (s.v[38] * s.dn[38][7]));
        let eq7_e329_d_n8: f64 = ((s.dn[38][8] * s.v[38]) + (s.v[38] * s.dn[38][8]));
        let eq7_e329_d_n9: f64 = ((s.dn[38][9] * s.v[38]) + (s.v[38] * s.dn[38][9]));
        let eq7_e329_d_n10: f64 = ((s.dn[38][10] * s.v[38]) + (s.v[38] * s.dn[38][10]));
        let eq7_e329_d_n11: f64 = ((s.dn[38][11] * s.v[38]) + (s.v[38] * s.dn[38][11]));
        let eq7_e329_d_n12: f64 = ((s.dn[38][12] * s.v[38]) + (s.v[38] * s.dn[38][12]));
        let eq7_e329_d_n13: f64 = ((s.dn[38][13] * s.v[38]) + (s.v[38] * s.dn[38][13]));
        let eq7_e329_d_n14: f64 = ((s.dn[38][14] * s.v[38]) + (s.v[38] * s.dn[38][14]));
        let eq7_e329_d_n15: f64 = ((s.dn[38][15] * s.v[38]) + (s.v[38] * s.dn[38][15]));
        let eq7_e329_d_n16: f64 = ((s.dn[38][16] * s.v[38]) + (s.v[38] * s.dn[38][16]));
        let eq7_e329_d_n17: f64 = ((s.dn[38][17] * s.v[38]) + (s.v[38] * s.dn[38][17]));
        let eq7_e329_d_n18: f64 = ((s.dn[38][18] * s.v[38]) + (s.v[38] * s.dn[38][18]));
        let eq7_e329_d_n19: f64 = ((s.dn[38][19] * s.v[38]) + (s.v[38] * s.dn[38][19]));
        let eq7_e329_d_n20: f64 = ((s.dn[38][20] * s.v[38]) + (s.v[38] * s.dn[38][20]));
        let eq7_e329_d_n21: f64 = ((s.dn[38][21] * s.v[38]) + (s.v[38] * s.dn[38][21]));
        let eq7_e329_d_n22: f64 = ((s.dn[38][22] * s.v[38]) + (s.v[38] * s.dn[38][22]));
        (eq7_e329, eq7_e329_d_n0, eq7_e329_d_n1, eq7_e329_d_n2, eq7_e329_d_n3, eq7_e329_d_n4, eq7_e329_d_n5, eq7_e329_d_n6, eq7_e329_d_n7, eq7_e329_d_n8, eq7_e329_d_n9, eq7_e329_d_n10, eq7_e329_d_n11, eq7_e329_d_n12, eq7_e329_d_n13, eq7_e329_d_n14, eq7_e329_d_n15, eq7_e329_d_n16, eq7_e329_d_n17, eq7_e329_d_n18, eq7_e329_d_n19, eq7_e329_d_n20, eq7_e329_d_n21, eq7_e329_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e331;
        let eq7_node_derivatives: [f64; 23] = [eq7_e331_d_n0, eq7_e331_d_n1, eq7_e331_d_n2, eq7_e331_d_n3, eq7_e331_d_n4, eq7_e331_d_n5, eq7_e331_d_n6, eq7_e331_d_n7, eq7_e331_d_n8, eq7_e331_d_n9, eq7_e331_d_n10, eq7_e331_d_n11, eq7_e331_d_n12, eq7_e331_d_n13, eq7_e331_d_n14, eq7_e331_d_n15, eq7_e331_d_n16, eq7_e331_d_n17, eq7_e331_d_n18, eq7_e331_d_n19, eq7_e331_d_n20, eq7_e331_d_n21, eq7_e331_d_n22];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_dense(
            branches[7],
            eq7_value,
            &nodes,
            &eq7_node_derivatives,
            &branches,
            &eq7_branch_derivatives,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq8_e345, eq8_e345_d_n5, eq8_e345_d_n6,) = {
    if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
        let eq8_e339: f64 = ((nv6 - nv5) / 10.0);
        let eq8_e339_d_n5: f64 = (-1.0 / 10.0);
        let eq8_e339_d_n6: f64 = (1.0 / 10.0);
        let eq8_e340: f64 = if eq8_e339 > 80.0 { 5.540622384e34 * (1.0 + eq8_e339 - 80.0) } else if eq8_e339 < -80.0 { 1.804851387e-35 } else { (eq8_e339).exp() };
        let eq8_e340_d_n5: f64 = (if eq8_e339 > 80.0 { 5.540622384e34 } else if eq8_e339 < -80.0 { 0.0 } else { (eq8_e339).exp() } * eq8_e339_d_n5);
        let eq8_e340_d_n6: f64 = (if eq8_e339 > 80.0 { 5.540622384e34 } else if eq8_e339 < -80.0 { 0.0 } else { (eq8_e339).exp() } * eq8_e339_d_n6);
        let eq8_e342: f64 = (eq8_e340 - 1.0);
        let eq8_e343: f64 = (p.p99 * eq8_e342);
        let eq8_e343_d_n5: f64 = (p.p99 * eq8_e340_d_n5);
        let eq8_e343_d_n6: f64 = (p.p99 * eq8_e340_d_n6);
        (eq8_e343, eq8_e343_d_n5, eq8_e343_d_n6,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e345;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[5]),
            self.multiplicity * (eq8_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq8_e345_d_n5),
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq8_e345_d_n6),
            ],
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq9_e355, eq9_e355_d_n0, eq9_e355_d_n1, eq9_e355_d_n2, eq9_e355_d_n3, eq9_e355_d_n4, eq9_e355_d_n5, eq9_e355_d_n6, eq9_e355_d_n7, eq9_e355_d_n8, eq9_e355_d_n9, eq9_e355_d_n10, eq9_e355_d_n11, eq9_e355_d_n12, eq9_e355_d_n13, eq9_e355_d_n14, eq9_e355_d_n15, eq9_e355_d_n16, eq9_e355_d_n17, eq9_e355_d_n18, eq9_e355_d_n19, eq9_e355_d_n20, eq9_e355_d_n21, eq9_e355_d_n22,) = {
    if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
        let eq9_e352: f64 = self.eval_ddt(0, (nv5 - 0.0));
        let eq9_e352_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n5: f64 = self.ddt_jacobian(1.0);
        let eq9_e352_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq9_e352_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq9_e353: f64 = (p.p97 * eq9_e352);
        let eq9_e353_d_n0: f64 = (p.p97 * eq9_e352_d_n0);
        let eq9_e353_d_n1: f64 = (p.p97 * eq9_e352_d_n1);
        let eq9_e353_d_n2: f64 = (p.p97 * eq9_e352_d_n2);
        let eq9_e353_d_n3: f64 = (p.p97 * eq9_e352_d_n3);
        let eq9_e353_d_n4: f64 = (p.p97 * eq9_e352_d_n4);
        let eq9_e353_d_n5: f64 = (p.p97 * eq9_e352_d_n5);
        let eq9_e353_d_n6: f64 = (p.p97 * eq9_e352_d_n6);
        let eq9_e353_d_n7: f64 = (p.p97 * eq9_e352_d_n7);
        let eq9_e353_d_n8: f64 = (p.p97 * eq9_e352_d_n8);
        let eq9_e353_d_n9: f64 = (p.p97 * eq9_e352_d_n9);
        let eq9_e353_d_n10: f64 = (p.p97 * eq9_e352_d_n10);
        let eq9_e353_d_n11: f64 = (p.p97 * eq9_e352_d_n11);
        let eq9_e353_d_n12: f64 = (p.p97 * eq9_e352_d_n12);
        let eq9_e353_d_n13: f64 = (p.p97 * eq9_e352_d_n13);
        let eq9_e353_d_n14: f64 = (p.p97 * eq9_e352_d_n14);
        let eq9_e353_d_n15: f64 = (p.p97 * eq9_e352_d_n15);
        let eq9_e353_d_n16: f64 = (p.p97 * eq9_e352_d_n16);
        let eq9_e353_d_n17: f64 = (p.p97 * eq9_e352_d_n17);
        let eq9_e353_d_n18: f64 = (p.p97 * eq9_e352_d_n18);
        let eq9_e353_d_n19: f64 = (p.p97 * eq9_e352_d_n19);
        let eq9_e353_d_n20: f64 = (p.p97 * eq9_e352_d_n20);
        let eq9_e353_d_n21: f64 = (p.p97 * eq9_e352_d_n21);
        let eq9_e353_d_n22: f64 = (p.p97 * eq9_e352_d_n22);
        (eq9_e353, eq9_e353_d_n0, eq9_e353_d_n1, eq9_e353_d_n2, eq9_e353_d_n3, eq9_e353_d_n4, eq9_e353_d_n5, eq9_e353_d_n6, eq9_e353_d_n7, eq9_e353_d_n8, eq9_e353_d_n9, eq9_e353_d_n10, eq9_e353_d_n11, eq9_e353_d_n12, eq9_e353_d_n13, eq9_e353_d_n14, eq9_e353_d_n15, eq9_e353_d_n16, eq9_e353_d_n17, eq9_e353_d_n18, eq9_e353_d_n19, eq9_e353_d_n20, eq9_e353_d_n21, eq9_e353_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e355;
        let eq9_node_derivatives: [f64; 23] = [eq9_e355_d_n0, eq9_e355_d_n1, eq9_e355_d_n2, eq9_e355_d_n3, eq9_e355_d_n4, eq9_e355_d_n5, eq9_e355_d_n6, eq9_e355_d_n7, eq9_e355_d_n8, eq9_e355_d_n9, eq9_e355_d_n10, eq9_e355_d_n11, eq9_e355_d_n12, eq9_e355_d_n13, eq9_e355_d_n14, eq9_e355_d_n15, eq9_e355_d_n16, eq9_e355_d_n17, eq9_e355_d_n18, eq9_e355_d_n19, eq9_e355_d_n20, eq9_e355_d_n21, eq9_e355_d_n22];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq10_e364, eq10_e364_d_n5,) = {
    if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
        let eq10_e362: f64 = ((nv5 - 0.0) / p.p98);
        let eq10_e362_d_n5: f64 = (1.0 / p.p98);
        (eq10_e362, eq10_e362_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e364;
        stamper.stamp_current(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq10_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq10_e364_d_n5),
            ],
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
        let (eq11_e371,) = {
    if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq11_value: f64 = eq11_e371;
        stamper.stamp_potential(
            branches[8],
            eq11_value,
            &[
            ],
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
        let (eq12_e378,) = {
    if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e378;
        stamper.stamp_potential(
            branches[9],
            eq12_value,
            &[
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
        let (eq13_e385,) = {
    if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e385;
        stamper.stamp_potential(
            branches[10],
            eq13_value,
            &[
            ],
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
        let (eq14_e392,) = {
    if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq14_value: f64 = eq14_e392;
        stamper.stamp_potential(
            branches[11],
            eq14_value,
            &[
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq15_e403, eq15_e403_d_n5,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        let eq15_e401: f64 = ((nv5 - 0.0) / p.p108);
        let eq15_e401_d_n5: f64 = (1.0 / p.p108);
        (eq15_e401, eq15_e401_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e403;
        stamper.stamp_current(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq15_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq15_e403_d_n5),
            ],
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
        let (eq16_e415, eq16_e415_d_n0, eq16_e415_d_n1, eq16_e415_d_n2, eq16_e415_d_n3, eq16_e415_d_n4, eq16_e415_d_n5, eq16_e415_d_n6, eq16_e415_d_n7, eq16_e415_d_n8, eq16_e415_d_n9, eq16_e415_d_n10, eq16_e415_d_n11, eq16_e415_d_n12, eq16_e415_d_n13, eq16_e415_d_n14, eq16_e415_d_n15, eq16_e415_d_n16, eq16_e415_d_n17, eq16_e415_d_n18, eq16_e415_d_n19, eq16_e415_d_n20, eq16_e415_d_n21, eq16_e415_d_n22,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        let eq16_e411: f64 = (-1.0);
        let eq16_e413: f64 = (eq16_e411 * s.v[208]);
        let eq16_e413_d_n0: f64 = (eq16_e411 * s.dn[208][0]);
        let eq16_e413_d_n1: f64 = (eq16_e411 * s.dn[208][1]);
        let eq16_e413_d_n2: f64 = (eq16_e411 * s.dn[208][2]);
        let eq16_e413_d_n3: f64 = (eq16_e411 * s.dn[208][3]);
        let eq16_e413_d_n4: f64 = (eq16_e411 * s.dn[208][4]);
        let eq16_e413_d_n5: f64 = (eq16_e411 * s.dn[208][5]);
        let eq16_e413_d_n6: f64 = (eq16_e411 * s.dn[208][6]);
        let eq16_e413_d_n7: f64 = (eq16_e411 * s.dn[208][7]);
        let eq16_e413_d_n8: f64 = (eq16_e411 * s.dn[208][8]);
        let eq16_e413_d_n9: f64 = (eq16_e411 * s.dn[208][9]);
        let eq16_e413_d_n10: f64 = (eq16_e411 * s.dn[208][10]);
        let eq16_e413_d_n11: f64 = (eq16_e411 * s.dn[208][11]);
        let eq16_e413_d_n12: f64 = (eq16_e411 * s.dn[208][12]);
        let eq16_e413_d_n13: f64 = (eq16_e411 * s.dn[208][13]);
        let eq16_e413_d_n14: f64 = (eq16_e411 * s.dn[208][14]);
        let eq16_e413_d_n15: f64 = (eq16_e411 * s.dn[208][15]);
        let eq16_e413_d_n16: f64 = (eq16_e411 * s.dn[208][16]);
        let eq16_e413_d_n17: f64 = (eq16_e411 * s.dn[208][17]);
        let eq16_e413_d_n18: f64 = (eq16_e411 * s.dn[208][18]);
        let eq16_e413_d_n19: f64 = (eq16_e411 * s.dn[208][19]);
        let eq16_e413_d_n20: f64 = (eq16_e411 * s.dn[208][20]);
        let eq16_e413_d_n21: f64 = (eq16_e411 * s.dn[208][21]);
        let eq16_e413_d_n22: f64 = (eq16_e411 * s.dn[208][22]);
        (eq16_e413, eq16_e413_d_n0, eq16_e413_d_n1, eq16_e413_d_n2, eq16_e413_d_n3, eq16_e413_d_n4, eq16_e413_d_n5, eq16_e413_d_n6, eq16_e413_d_n7, eq16_e413_d_n8, eq16_e413_d_n9, eq16_e413_d_n10, eq16_e413_d_n11, eq16_e413_d_n12, eq16_e413_d_n13, eq16_e413_d_n14, eq16_e413_d_n15, eq16_e413_d_n16, eq16_e413_d_n17, eq16_e413_d_n18, eq16_e413_d_n19, eq16_e413_d_n20, eq16_e413_d_n21, eq16_e413_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e415;
        let eq16_node_derivatives: [f64; 23] = [eq16_e415_d_n0, eq16_e415_d_n1, eq16_e415_d_n2, eq16_e415_d_n3, eq16_e415_d_n4, eq16_e415_d_n5, eq16_e415_d_n6, eq16_e415_d_n7, eq16_e415_d_n8, eq16_e415_d_n9, eq16_e415_d_n10, eq16_e415_d_n11, eq16_e415_d_n12, eq16_e415_d_n13, eq16_e415_d_n14, eq16_e415_d_n15, eq16_e415_d_n16, eq16_e415_d_n17, eq16_e415_d_n18, eq16_e415_d_n19, eq16_e415_d_n20, eq16_e415_d_n21, eq16_e415_d_n22];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq17_e427, eq17_e427_d_n0, eq17_e427_d_n1, eq17_e427_d_n2, eq17_e427_d_n3, eq17_e427_d_n4, eq17_e427_d_n5, eq17_e427_d_n6, eq17_e427_d_n7, eq17_e427_d_n8, eq17_e427_d_n9, eq17_e427_d_n10, eq17_e427_d_n11, eq17_e427_d_n12, eq17_e427_d_n13, eq17_e427_d_n14, eq17_e427_d_n15, eq17_e427_d_n16, eq17_e427_d_n17, eq17_e427_d_n18, eq17_e427_d_n19, eq17_e427_d_n20, eq17_e427_d_n21, eq17_e427_d_n22,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        let eq17_e424: f64 = self.eval_ddt(1, (nv5 - 0.0));
        let eq17_e424_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n5: f64 = self.ddt_jacobian(1.0);
        let eq17_e424_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq17_e424_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq17_e425: f64 = (p.p110 * eq17_e424);
        let eq17_e425_d_n0: f64 = (p.p110 * eq17_e424_d_n0);
        let eq17_e425_d_n1: f64 = (p.p110 * eq17_e424_d_n1);
        let eq17_e425_d_n2: f64 = (p.p110 * eq17_e424_d_n2);
        let eq17_e425_d_n3: f64 = (p.p110 * eq17_e424_d_n3);
        let eq17_e425_d_n4: f64 = (p.p110 * eq17_e424_d_n4);
        let eq17_e425_d_n5: f64 = (p.p110 * eq17_e424_d_n5);
        let eq17_e425_d_n6: f64 = (p.p110 * eq17_e424_d_n6);
        let eq17_e425_d_n7: f64 = (p.p110 * eq17_e424_d_n7);
        let eq17_e425_d_n8: f64 = (p.p110 * eq17_e424_d_n8);
        let eq17_e425_d_n9: f64 = (p.p110 * eq17_e424_d_n9);
        let eq17_e425_d_n10: f64 = (p.p110 * eq17_e424_d_n10);
        let eq17_e425_d_n11: f64 = (p.p110 * eq17_e424_d_n11);
        let eq17_e425_d_n12: f64 = (p.p110 * eq17_e424_d_n12);
        let eq17_e425_d_n13: f64 = (p.p110 * eq17_e424_d_n13);
        let eq17_e425_d_n14: f64 = (p.p110 * eq17_e424_d_n14);
        let eq17_e425_d_n15: f64 = (p.p110 * eq17_e424_d_n15);
        let eq17_e425_d_n16: f64 = (p.p110 * eq17_e424_d_n16);
        let eq17_e425_d_n17: f64 = (p.p110 * eq17_e424_d_n17);
        let eq17_e425_d_n18: f64 = (p.p110 * eq17_e424_d_n18);
        let eq17_e425_d_n19: f64 = (p.p110 * eq17_e424_d_n19);
        let eq17_e425_d_n20: f64 = (p.p110 * eq17_e424_d_n20);
        let eq17_e425_d_n21: f64 = (p.p110 * eq17_e424_d_n21);
        let eq17_e425_d_n22: f64 = (p.p110 * eq17_e424_d_n22);
        (eq17_e425, eq17_e425_d_n0, eq17_e425_d_n1, eq17_e425_d_n2, eq17_e425_d_n3, eq17_e425_d_n4, eq17_e425_d_n5, eq17_e425_d_n6, eq17_e425_d_n7, eq17_e425_d_n8, eq17_e425_d_n9, eq17_e425_d_n10, eq17_e425_d_n11, eq17_e425_d_n12, eq17_e425_d_n13, eq17_e425_d_n14, eq17_e425_d_n15, eq17_e425_d_n16, eq17_e425_d_n17, eq17_e425_d_n18, eq17_e425_d_n19, eq17_e425_d_n20, eq17_e425_d_n21, eq17_e425_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e427;
        let eq17_node_derivatives: [f64; 23] = [eq17_e427_d_n0, eq17_e427_d_n1, eq17_e427_d_n2, eq17_e427_d_n3, eq17_e427_d_n4, eq17_e427_d_n5, eq17_e427_d_n6, eq17_e427_d_n7, eq17_e427_d_n8, eq17_e427_d_n9, eq17_e427_d_n10, eq17_e427_d_n11, eq17_e427_d_n12, eq17_e427_d_n13, eq17_e427_d_n14, eq17_e427_d_n15, eq17_e427_d_n16, eq17_e427_d_n17, eq17_e427_d_n18, eq17_e427_d_n19, eq17_e427_d_n20, eq17_e427_d_n21, eq17_e427_d_n22];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq18_e438, eq18_e438_d_n6,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        let eq18_e436: f64 = ((nv6 - 0.0) / p.p109);
        let eq18_e436_d_n6: f64 = (1.0 / p.p109);
        (eq18_e436, eq18_e436_d_n6,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e438;
        stamper.stamp_current(
            Some(nodes[6]),
            None,
            self.multiplicity * (eq18_value),
            &[
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq18_e438_d_n6),
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq19_e450, eq19_e450_d_n0, eq19_e450_d_n2,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        let eq19_e446: f64 = (-1.0);
        let eq19_e448: f64 = (eq19_e446 * (nv0 - nv2));
        let eq19_e448_d_n2: f64 = (-eq19_e446);
        (eq19_e448, eq19_e446, eq19_e448_d_n2,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e450;
        stamper.stamp_current(
            Some(nodes[6]),
            None,
            self.multiplicity * (eq19_value),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * eq19_e450_d_n0),
                GeneratedDerivative::node(nodes[2], self.multiplicity * eq19_e450_d_n2),
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq20_e462, eq20_e462_d_n0, eq20_e462_d_n1, eq20_e462_d_n2, eq20_e462_d_n3, eq20_e462_d_n4, eq20_e462_d_n5, eq20_e462_d_n6, eq20_e462_d_n7, eq20_e462_d_n8, eq20_e462_d_n9, eq20_e462_d_n10, eq20_e462_d_n11, eq20_e462_d_n12, eq20_e462_d_n13, eq20_e462_d_n14, eq20_e462_d_n15, eq20_e462_d_n16, eq20_e462_d_n17, eq20_e462_d_n18, eq20_e462_d_n19, eq20_e462_d_n20, eq20_e462_d_n21, eq20_e462_d_n22,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        let eq20_e459: f64 = self.eval_ddt(2, (nv6 - 0.0));
        let eq20_e459_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n6: f64 = self.ddt_jacobian(1.0);
        let eq20_e459_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq20_e459_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq20_e460: f64 = (p.p111 * eq20_e459);
        let eq20_e460_d_n0: f64 = (p.p111 * eq20_e459_d_n0);
        let eq20_e460_d_n1: f64 = (p.p111 * eq20_e459_d_n1);
        let eq20_e460_d_n2: f64 = (p.p111 * eq20_e459_d_n2);
        let eq20_e460_d_n3: f64 = (p.p111 * eq20_e459_d_n3);
        let eq20_e460_d_n4: f64 = (p.p111 * eq20_e459_d_n4);
        let eq20_e460_d_n5: f64 = (p.p111 * eq20_e459_d_n5);
        let eq20_e460_d_n6: f64 = (p.p111 * eq20_e459_d_n6);
        let eq20_e460_d_n7: f64 = (p.p111 * eq20_e459_d_n7);
        let eq20_e460_d_n8: f64 = (p.p111 * eq20_e459_d_n8);
        let eq20_e460_d_n9: f64 = (p.p111 * eq20_e459_d_n9);
        let eq20_e460_d_n10: f64 = (p.p111 * eq20_e459_d_n10);
        let eq20_e460_d_n11: f64 = (p.p111 * eq20_e459_d_n11);
        let eq20_e460_d_n12: f64 = (p.p111 * eq20_e459_d_n12);
        let eq20_e460_d_n13: f64 = (p.p111 * eq20_e459_d_n13);
        let eq20_e460_d_n14: f64 = (p.p111 * eq20_e459_d_n14);
        let eq20_e460_d_n15: f64 = (p.p111 * eq20_e459_d_n15);
        let eq20_e460_d_n16: f64 = (p.p111 * eq20_e459_d_n16);
        let eq20_e460_d_n17: f64 = (p.p111 * eq20_e459_d_n17);
        let eq20_e460_d_n18: f64 = (p.p111 * eq20_e459_d_n18);
        let eq20_e460_d_n19: f64 = (p.p111 * eq20_e459_d_n19);
        let eq20_e460_d_n20: f64 = (p.p111 * eq20_e459_d_n20);
        let eq20_e460_d_n21: f64 = (p.p111 * eq20_e459_d_n21);
        let eq20_e460_d_n22: f64 = (p.p111 * eq20_e459_d_n22);
        (eq20_e460, eq20_e460_d_n0, eq20_e460_d_n1, eq20_e460_d_n2, eq20_e460_d_n3, eq20_e460_d_n4, eq20_e460_d_n5, eq20_e460_d_n6, eq20_e460_d_n7, eq20_e460_d_n8, eq20_e460_d_n9, eq20_e460_d_n10, eq20_e460_d_n11, eq20_e460_d_n12, eq20_e460_d_n13, eq20_e460_d_n14, eq20_e460_d_n15, eq20_e460_d_n16, eq20_e460_d_n17, eq20_e460_d_n18, eq20_e460_d_n19, eq20_e460_d_n20, eq20_e460_d_n21, eq20_e460_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e462;
        let eq20_node_derivatives: [f64; 23] = [eq20_e462_d_n0, eq20_e462_d_n1, eq20_e462_d_n2, eq20_e462_d_n3, eq20_e462_d_n4, eq20_e462_d_n5, eq20_e462_d_n6, eq20_e462_d_n7, eq20_e462_d_n8, eq20_e462_d_n9, eq20_e462_d_n10, eq20_e462_d_n11, eq20_e462_d_n12, eq20_e462_d_n13, eq20_e462_d_n14, eq20_e462_d_n15, eq20_e462_d_n16, eq20_e462_d_n17, eq20_e462_d_n18, eq20_e462_d_n19, eq20_e462_d_n20, eq20_e462_d_n21, eq20_e462_d_n22];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            None,
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
        let (eq21_e471,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e471;
        stamper.stamp_potential(
            branches[12],
            eq21_value,
            &[
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
        let (eq22_e480,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e480;
        stamper.stamp_potential(
            branches[13],
            eq22_value,
            &[
            ],
        );
    }
}
