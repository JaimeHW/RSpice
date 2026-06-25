#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq31_e1220,) = {
    if (s.v[2707] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e1220;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[9]),
            self.multiplicity * (eq31_value),
            &[
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
        let (eq32_e1225,) = {
    if (!(s.v[2707] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1225;
        stamper.stamp_potential(
            branches[5],
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq33_e1235, eq33_e1235_d_n0, eq33_e1235_d_n1, eq33_e1235_d_n2, eq33_e1235_d_n3, eq33_e1235_d_n4, eq33_e1235_d_n5, eq33_e1235_d_n6, eq33_e1235_d_n7, eq33_e1235_d_n8, eq33_e1235_d_n9, eq33_e1235_d_n10, eq33_e1235_d_n11,) = {
    if (s.v[2708] != 0.0) {
        let eq33_e1229: f64 = (s.v[15] * p.p32);
        let eq33_e1229_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq33_e1229_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq33_e1229_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq33_e1229_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq33_e1229_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq33_e1229_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq33_e1229_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq33_e1229_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq33_e1229_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq33_e1229_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq33_e1229_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq33_e1229_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq33_e1231: f64 = (eq33_e1229 * s.v[811]);
        let eq33_e1231_d_n0: f64 = ((eq33_e1229_d_n0 * s.v[811]) + (eq33_e1229 * s.dn[811][0]));
        let eq33_e1231_d_n1: f64 = ((eq33_e1229_d_n1 * s.v[811]) + (eq33_e1229 * s.dn[811][1]));
        let eq33_e1231_d_n2: f64 = ((eq33_e1229_d_n2 * s.v[811]) + (eq33_e1229 * s.dn[811][2]));
        let eq33_e1231_d_n3: f64 = ((eq33_e1229_d_n3 * s.v[811]) + (eq33_e1229 * s.dn[811][3]));
        let eq33_e1231_d_n4: f64 = ((eq33_e1229_d_n4 * s.v[811]) + (eq33_e1229 * s.dn[811][4]));
        let eq33_e1231_d_n5: f64 = ((eq33_e1229_d_n5 * s.v[811]) + (eq33_e1229 * s.dn[811][5]));
        let eq33_e1231_d_n6: f64 = ((eq33_e1229_d_n6 * s.v[811]) + (eq33_e1229 * s.dn[811][6]));
        let eq33_e1231_d_n7: f64 = ((eq33_e1229_d_n7 * s.v[811]) + (eq33_e1229 * s.dn[811][7]));
        let eq33_e1231_d_n8: f64 = ((eq33_e1229_d_n8 * s.v[811]) + (eq33_e1229 * s.dn[811][8]));
        let eq33_e1231_d_n9: f64 = ((eq33_e1229_d_n9 * s.v[811]) + (eq33_e1229 * s.dn[811][9]));
        let eq33_e1231_d_n10: f64 = ((eq33_e1229_d_n10 * s.v[811]) + (eq33_e1229 * s.dn[811][10]));
        let eq33_e1231_d_n11: f64 = ((eq33_e1229_d_n11 * s.v[811]) + (eq33_e1229 * s.dn[811][11]));
        let eq33_e1233: f64 = (eq33_e1231 * (nv3 - nv9));
        let eq33_e1233_d_n0: f64 = (eq33_e1231_d_n0 * (nv3 - nv9));
        let eq33_e1233_d_n1: f64 = (eq33_e1231_d_n1 * (nv3 - nv9));
        let eq33_e1233_d_n2: f64 = (eq33_e1231_d_n2 * (nv3 - nv9));
        let eq33_e1233_d_n3: f64 = ((eq33_e1231_d_n3 * (nv3 - nv9)) + eq33_e1231);
        let eq33_e1233_d_n4: f64 = (eq33_e1231_d_n4 * (nv3 - nv9));
        let eq33_e1233_d_n5: f64 = (eq33_e1231_d_n5 * (nv3 - nv9));
        let eq33_e1233_d_n6: f64 = (eq33_e1231_d_n6 * (nv3 - nv9));
        let eq33_e1233_d_n7: f64 = (eq33_e1231_d_n7 * (nv3 - nv9));
        let eq33_e1233_d_n8: f64 = (eq33_e1231_d_n8 * (nv3 - nv9));
        let eq33_e1233_d_n9: f64 = ((eq33_e1231_d_n9 * (nv3 - nv9)) + (-eq33_e1231));
        let eq33_e1233_d_n10: f64 = (eq33_e1231_d_n10 * (nv3 - nv9));
        let eq33_e1233_d_n11: f64 = (eq33_e1231_d_n11 * (nv3 - nv9));
        (eq33_e1233, eq33_e1233_d_n0, eq33_e1233_d_n1, eq33_e1233_d_n2, eq33_e1233_d_n3, eq33_e1233_d_n4, eq33_e1233_d_n5, eq33_e1233_d_n6, eq33_e1233_d_n7, eq33_e1233_d_n8, eq33_e1233_d_n9, eq33_e1233_d_n10, eq33_e1233_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1235;
        let eq33_node_derivatives: [f64; 12] = [eq33_e1235_d_n0, eq33_e1235_d_n1, eq33_e1235_d_n2, eq33_e1235_d_n3, eq33_e1235_d_n4, eq33_e1235_d_n5, eq33_e1235_d_n6, eq33_e1235_d_n7, eq33_e1235_d_n8, eq33_e1235_d_n9, eq33_e1235_d_n10, eq33_e1235_d_n11];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[9]),
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
        let (eq34_e1245,) = {
    if (s.v[2708] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e1245;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[9]),
            self.multiplicity * (eq34_value),
            &[
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
        let (eq35_e1250,) = {
    if (!(s.v[2708] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1250;
        stamper.stamp_potential(
            branches[6],
            eq35_value,
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq36_e1253: f64 = (p.p32 * s.v[872]);
        let eq36_e1255: f64 = (eq36_e1253 * (nv7 - nv8));
        let eq36_e1255_d_n8: f64 = (-eq36_e1253);
        let eq36_value: f64 = eq36_e1255;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq36_value),
            &[
                GeneratedDerivative::node(nodes[7], self.multiplicity * eq36_e1253),
                GeneratedDerivative::node(nodes[8], self.multiplicity * eq36_e1255_d_n8),
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq37_e1258: f64 = (p.p32 * s.v[872]);
        let eq37_e1260: f64 = (eq37_e1258 * (nv6 - nv8));
        let eq37_e1260_d_n8: f64 = (-eq37_e1258);
        let eq37_value: f64 = eq37_e1260;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq37_value),
            &[
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq37_e1258),
                GeneratedDerivative::node(nodes[8], self.multiplicity * eq37_e1260_d_n8),
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
        let eq38_e1263: f64 = (s.v[0] * s.v[15]);
        let eq38_e1263_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq38_e1263_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq38_e1263_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq38_e1263_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq38_e1263_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq38_e1263_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq38_e1263_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq38_e1263_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq38_e1263_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq38_e1263_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq38_e1263_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq38_e1263_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq38_e1265: f64 = (eq38_e1263 * p.p33);
        let eq38_e1265_d_n0: f64 = (eq38_e1263_d_n0 * p.p33);
        let eq38_e1265_d_n1: f64 = (eq38_e1263_d_n1 * p.p33);
        let eq38_e1265_d_n2: f64 = (eq38_e1263_d_n2 * p.p33);
        let eq38_e1265_d_n3: f64 = (eq38_e1263_d_n3 * p.p33);
        let eq38_e1265_d_n4: f64 = (eq38_e1263_d_n4 * p.p33);
        let eq38_e1265_d_n5: f64 = (eq38_e1263_d_n5 * p.p33);
        let eq38_e1265_d_n6: f64 = (eq38_e1263_d_n6 * p.p33);
        let eq38_e1265_d_n7: f64 = (eq38_e1263_d_n7 * p.p33);
        let eq38_e1265_d_n8: f64 = (eq38_e1263_d_n8 * p.p33);
        let eq38_e1265_d_n9: f64 = (eq38_e1263_d_n9 * p.p33);
        let eq38_e1265_d_n10: f64 = (eq38_e1263_d_n10 * p.p33);
        let eq38_e1265_d_n11: f64 = (eq38_e1263_d_n11 * p.p33);
        let eq38_e1267: f64 = (eq38_e1265 * s.v[845]);
        let eq38_e1267_d_n0: f64 = ((eq38_e1265_d_n0 * s.v[845]) + (eq38_e1265 * s.dn[845][0]));
        let eq38_e1267_d_n1: f64 = ((eq38_e1265_d_n1 * s.v[845]) + (eq38_e1265 * s.dn[845][1]));
        let eq38_e1267_d_n2: f64 = ((eq38_e1265_d_n2 * s.v[845]) + (eq38_e1265 * s.dn[845][2]));
        let eq38_e1267_d_n3: f64 = ((eq38_e1265_d_n3 * s.v[845]) + (eq38_e1265 * s.dn[845][3]));
        let eq38_e1267_d_n4: f64 = ((eq38_e1265_d_n4 * s.v[845]) + (eq38_e1265 * s.dn[845][4]));
        let eq38_e1267_d_n5: f64 = ((eq38_e1265_d_n5 * s.v[845]) + (eq38_e1265 * s.dn[845][5]));
        let eq38_e1267_d_n6: f64 = ((eq38_e1265_d_n6 * s.v[845]) + (eq38_e1265 * s.dn[845][6]));
        let eq38_e1267_d_n7: f64 = ((eq38_e1265_d_n7 * s.v[845]) + (eq38_e1265 * s.dn[845][7]));
        let eq38_e1267_d_n8: f64 = ((eq38_e1265_d_n8 * s.v[845]) + (eq38_e1265 * s.dn[845][8]));
        let eq38_e1267_d_n9: f64 = ((eq38_e1265_d_n9 * s.v[845]) + (eq38_e1265 * s.dn[845][9]));
        let eq38_e1267_d_n10: f64 = ((eq38_e1265_d_n10 * s.v[845]) + (eq38_e1265 * s.dn[845][10]));
        let eq38_e1267_d_n11: f64 = ((eq38_e1265_d_n11 * s.v[845]) + (eq38_e1265 * s.dn[845][11]));
        let eq38_e1268: f64 = self.eval_ddt(0, eq38_e1267);
        let eq38_e1268_d_n0: f64 = self.ddt_jacobian(eq38_e1267_d_n0);
        let eq38_e1268_d_n1: f64 = self.ddt_jacobian(eq38_e1267_d_n1);
        let eq38_e1268_d_n2: f64 = self.ddt_jacobian(eq38_e1267_d_n2);
        let eq38_e1268_d_n3: f64 = self.ddt_jacobian(eq38_e1267_d_n3);
        let eq38_e1268_d_n4: f64 = self.ddt_jacobian(eq38_e1267_d_n4);
        let eq38_e1268_d_n5: f64 = self.ddt_jacobian(eq38_e1267_d_n5);
        let eq38_e1268_d_n6: f64 = self.ddt_jacobian(eq38_e1267_d_n6);
        let eq38_e1268_d_n7: f64 = self.ddt_jacobian(eq38_e1267_d_n7);
        let eq38_e1268_d_n8: f64 = self.ddt_jacobian(eq38_e1267_d_n8);
        let eq38_e1268_d_n9: f64 = self.ddt_jacobian(eq38_e1267_d_n9);
        let eq38_e1268_d_n10: f64 = self.ddt_jacobian(eq38_e1267_d_n10);
        let eq38_e1268_d_n11: f64 = self.ddt_jacobian(eq38_e1267_d_n11);
        let eq38_value: f64 = eq38_e1268;
        let eq38_node_derivatives: [f64; 12] = [eq38_e1268_d_n0, eq38_e1268_d_n1, eq38_e1268_d_n2, eq38_e1268_d_n3, eq38_e1268_d_n4, eq38_e1268_d_n5, eq38_e1268_d_n6, eq38_e1268_d_n7, eq38_e1268_d_n8, eq38_e1268_d_n9, eq38_e1268_d_n10, eq38_e1268_d_n11];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq38_value),
            &nodes,
            &eq38_node_derivatives,
            &branches,
            &eq38_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_39_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq39_e1271: f64 = (s.v[0] * s.v[15]);
        let eq39_e1271_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq39_e1271_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq39_e1271_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq39_e1271_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq39_e1271_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq39_e1271_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq39_e1271_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq39_e1271_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq39_e1271_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq39_e1271_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq39_e1271_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq39_e1271_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq39_e1273: f64 = (eq39_e1271 * p.p33);
        let eq39_e1273_d_n0: f64 = (eq39_e1271_d_n0 * p.p33);
        let eq39_e1273_d_n1: f64 = (eq39_e1271_d_n1 * p.p33);
        let eq39_e1273_d_n2: f64 = (eq39_e1271_d_n2 * p.p33);
        let eq39_e1273_d_n3: f64 = (eq39_e1271_d_n3 * p.p33);
        let eq39_e1273_d_n4: f64 = (eq39_e1271_d_n4 * p.p33);
        let eq39_e1273_d_n5: f64 = (eq39_e1271_d_n5 * p.p33);
        let eq39_e1273_d_n6: f64 = (eq39_e1271_d_n6 * p.p33);
        let eq39_e1273_d_n7: f64 = (eq39_e1271_d_n7 * p.p33);
        let eq39_e1273_d_n8: f64 = (eq39_e1271_d_n8 * p.p33);
        let eq39_e1273_d_n9: f64 = (eq39_e1271_d_n9 * p.p33);
        let eq39_e1273_d_n10: f64 = (eq39_e1271_d_n10 * p.p33);
        let eq39_e1273_d_n11: f64 = (eq39_e1271_d_n11 * p.p33);
        let eq39_e1275: f64 = (eq39_e1273 * s.v[846]);
        let eq39_e1275_d_n0: f64 = ((eq39_e1273_d_n0 * s.v[846]) + (eq39_e1273 * s.dn[846][0]));
        let eq39_e1275_d_n1: f64 = ((eq39_e1273_d_n1 * s.v[846]) + (eq39_e1273 * s.dn[846][1]));
        let eq39_e1275_d_n2: f64 = ((eq39_e1273_d_n2 * s.v[846]) + (eq39_e1273 * s.dn[846][2]));
        let eq39_e1275_d_n3: f64 = ((eq39_e1273_d_n3 * s.v[846]) + (eq39_e1273 * s.dn[846][3]));
        let eq39_e1275_d_n4: f64 = ((eq39_e1273_d_n4 * s.v[846]) + (eq39_e1273 * s.dn[846][4]));
        let eq39_e1275_d_n5: f64 = ((eq39_e1273_d_n5 * s.v[846]) + (eq39_e1273 * s.dn[846][5]));
        let eq39_e1275_d_n6: f64 = ((eq39_e1273_d_n6 * s.v[846]) + (eq39_e1273 * s.dn[846][6]));
        let eq39_e1275_d_n7: f64 = ((eq39_e1273_d_n7 * s.v[846]) + (eq39_e1273 * s.dn[846][7]));
        let eq39_e1275_d_n8: f64 = ((eq39_e1273_d_n8 * s.v[846]) + (eq39_e1273 * s.dn[846][8]));
        let eq39_e1275_d_n9: f64 = ((eq39_e1273_d_n9 * s.v[846]) + (eq39_e1273 * s.dn[846][9]));
        let eq39_e1275_d_n10: f64 = ((eq39_e1273_d_n10 * s.v[846]) + (eq39_e1273 * s.dn[846][10]));
        let eq39_e1275_d_n11: f64 = ((eq39_e1273_d_n11 * s.v[846]) + (eq39_e1273 * s.dn[846][11]));
        let eq39_e1276: f64 = self.eval_ddt(1, eq39_e1275);
        let eq39_e1276_d_n0: f64 = self.ddt_jacobian(eq39_e1275_d_n0);
        let eq39_e1276_d_n1: f64 = self.ddt_jacobian(eq39_e1275_d_n1);
        let eq39_e1276_d_n2: f64 = self.ddt_jacobian(eq39_e1275_d_n2);
        let eq39_e1276_d_n3: f64 = self.ddt_jacobian(eq39_e1275_d_n3);
        let eq39_e1276_d_n4: f64 = self.ddt_jacobian(eq39_e1275_d_n4);
        let eq39_e1276_d_n5: f64 = self.ddt_jacobian(eq39_e1275_d_n5);
        let eq39_e1276_d_n6: f64 = self.ddt_jacobian(eq39_e1275_d_n6);
        let eq39_e1276_d_n7: f64 = self.ddt_jacobian(eq39_e1275_d_n7);
        let eq39_e1276_d_n8: f64 = self.ddt_jacobian(eq39_e1275_d_n8);
        let eq39_e1276_d_n9: f64 = self.ddt_jacobian(eq39_e1275_d_n9);
        let eq39_e1276_d_n10: f64 = self.ddt_jacobian(eq39_e1275_d_n10);
        let eq39_e1276_d_n11: f64 = self.ddt_jacobian(eq39_e1275_d_n11);
        let eq39_value: f64 = eq39_e1276;
        let eq39_node_derivatives: [f64; 12] = [eq39_e1276_d_n0, eq39_e1276_d_n1, eq39_e1276_d_n2, eq39_e1276_d_n3, eq39_e1276_d_n4, eq39_e1276_d_n5, eq39_e1276_d_n6, eq39_e1276_d_n7, eq39_e1276_d_n8, eq39_e1276_d_n9, eq39_e1276_d_n10, eq39_e1276_d_n11];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq39_value),
            &nodes,
            &eq39_node_derivatives,
            &branches,
            &eq39_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_40_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq40_e1279: f64 = (s.v[0] * s.v[15]);
        let eq40_e1279_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq40_e1279_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq40_e1279_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq40_e1279_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq40_e1279_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq40_e1279_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq40_e1279_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq40_e1279_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq40_e1279_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq40_e1279_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq40_e1279_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq40_e1279_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq40_e1281: f64 = (eq40_e1279 * p.p33);
        let eq40_e1281_d_n0: f64 = (eq40_e1279_d_n0 * p.p33);
        let eq40_e1281_d_n1: f64 = (eq40_e1279_d_n1 * p.p33);
        let eq40_e1281_d_n2: f64 = (eq40_e1279_d_n2 * p.p33);
        let eq40_e1281_d_n3: f64 = (eq40_e1279_d_n3 * p.p33);
        let eq40_e1281_d_n4: f64 = (eq40_e1279_d_n4 * p.p33);
        let eq40_e1281_d_n5: f64 = (eq40_e1279_d_n5 * p.p33);
        let eq40_e1281_d_n6: f64 = (eq40_e1279_d_n6 * p.p33);
        let eq40_e1281_d_n7: f64 = (eq40_e1279_d_n7 * p.p33);
        let eq40_e1281_d_n8: f64 = (eq40_e1279_d_n8 * p.p33);
        let eq40_e1281_d_n9: f64 = (eq40_e1279_d_n9 * p.p33);
        let eq40_e1281_d_n10: f64 = (eq40_e1279_d_n10 * p.p33);
        let eq40_e1281_d_n11: f64 = (eq40_e1279_d_n11 * p.p33);
        let eq40_e1283: f64 = (eq40_e1281 * s.v[847]);
        let eq40_e1283_d_n0: f64 = ((eq40_e1281_d_n0 * s.v[847]) + (eq40_e1281 * s.dn[847][0]));
        let eq40_e1283_d_n1: f64 = ((eq40_e1281_d_n1 * s.v[847]) + (eq40_e1281 * s.dn[847][1]));
        let eq40_e1283_d_n2: f64 = ((eq40_e1281_d_n2 * s.v[847]) + (eq40_e1281 * s.dn[847][2]));
        let eq40_e1283_d_n3: f64 = ((eq40_e1281_d_n3 * s.v[847]) + (eq40_e1281 * s.dn[847][3]));
        let eq40_e1283_d_n4: f64 = ((eq40_e1281_d_n4 * s.v[847]) + (eq40_e1281 * s.dn[847][4]));
        let eq40_e1283_d_n5: f64 = ((eq40_e1281_d_n5 * s.v[847]) + (eq40_e1281 * s.dn[847][5]));
        let eq40_e1283_d_n6: f64 = ((eq40_e1281_d_n6 * s.v[847]) + (eq40_e1281 * s.dn[847][6]));
        let eq40_e1283_d_n7: f64 = ((eq40_e1281_d_n7 * s.v[847]) + (eq40_e1281 * s.dn[847][7]));
        let eq40_e1283_d_n8: f64 = ((eq40_e1281_d_n8 * s.v[847]) + (eq40_e1281 * s.dn[847][8]));
        let eq40_e1283_d_n9: f64 = ((eq40_e1281_d_n9 * s.v[847]) + (eq40_e1281 * s.dn[847][9]));
        let eq40_e1283_d_n10: f64 = ((eq40_e1281_d_n10 * s.v[847]) + (eq40_e1281 * s.dn[847][10]));
        let eq40_e1283_d_n11: f64 = ((eq40_e1281_d_n11 * s.v[847]) + (eq40_e1281 * s.dn[847][11]));
        let eq40_e1284: f64 = self.eval_ddt(2, eq40_e1283);
        let eq40_e1284_d_n0: f64 = self.ddt_jacobian(eq40_e1283_d_n0);
        let eq40_e1284_d_n1: f64 = self.ddt_jacobian(eq40_e1283_d_n1);
        let eq40_e1284_d_n2: f64 = self.ddt_jacobian(eq40_e1283_d_n2);
        let eq40_e1284_d_n3: f64 = self.ddt_jacobian(eq40_e1283_d_n3);
        let eq40_e1284_d_n4: f64 = self.ddt_jacobian(eq40_e1283_d_n4);
        let eq40_e1284_d_n5: f64 = self.ddt_jacobian(eq40_e1283_d_n5);
        let eq40_e1284_d_n6: f64 = self.ddt_jacobian(eq40_e1283_d_n6);
        let eq40_e1284_d_n7: f64 = self.ddt_jacobian(eq40_e1283_d_n7);
        let eq40_e1284_d_n8: f64 = self.ddt_jacobian(eq40_e1283_d_n8);
        let eq40_e1284_d_n9: f64 = self.ddt_jacobian(eq40_e1283_d_n9);
        let eq40_e1284_d_n10: f64 = self.ddt_jacobian(eq40_e1283_d_n10);
        let eq40_e1284_d_n11: f64 = self.ddt_jacobian(eq40_e1283_d_n11);
        let eq40_value: f64 = eq40_e1284;
        let eq40_node_derivatives: [f64; 12] = [eq40_e1284_d_n0, eq40_e1284_d_n1, eq40_e1284_d_n2, eq40_e1284_d_n3, eq40_e1284_d_n4, eq40_e1284_d_n5, eq40_e1284_d_n6, eq40_e1284_d_n7, eq40_e1284_d_n8, eq40_e1284_d_n9, eq40_e1284_d_n10, eq40_e1284_d_n11];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq40_value),
            &nodes,
            &eq40_node_derivatives,
            &branches,
            &eq40_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_41_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq41_e1287: f64 = (s.v[0] * s.v[15]);
        let eq41_e1287_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq41_e1287_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq41_e1287_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq41_e1287_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq41_e1287_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq41_e1287_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq41_e1287_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq41_e1287_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq41_e1287_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq41_e1287_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq41_e1287_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq41_e1287_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq41_e1289: f64 = (eq41_e1287 * p.p33);
        let eq41_e1289_d_n0: f64 = (eq41_e1287_d_n0 * p.p33);
        let eq41_e1289_d_n1: f64 = (eq41_e1287_d_n1 * p.p33);
        let eq41_e1289_d_n2: f64 = (eq41_e1287_d_n2 * p.p33);
        let eq41_e1289_d_n3: f64 = (eq41_e1287_d_n3 * p.p33);
        let eq41_e1289_d_n4: f64 = (eq41_e1287_d_n4 * p.p33);
        let eq41_e1289_d_n5: f64 = (eq41_e1287_d_n5 * p.p33);
        let eq41_e1289_d_n6: f64 = (eq41_e1287_d_n6 * p.p33);
        let eq41_e1289_d_n7: f64 = (eq41_e1287_d_n7 * p.p33);
        let eq41_e1289_d_n8: f64 = (eq41_e1287_d_n8 * p.p33);
        let eq41_e1289_d_n9: f64 = (eq41_e1287_d_n9 * p.p33);
        let eq41_e1289_d_n10: f64 = (eq41_e1287_d_n10 * p.p33);
        let eq41_e1289_d_n11: f64 = (eq41_e1287_d_n11 * p.p33);
        let eq41_e1291: f64 = (eq41_e1289 * s.v[848]);
        let eq41_e1291_d_n0: f64 = ((eq41_e1289_d_n0 * s.v[848]) + (eq41_e1289 * s.dn[848][0]));
        let eq41_e1291_d_n1: f64 = ((eq41_e1289_d_n1 * s.v[848]) + (eq41_e1289 * s.dn[848][1]));
        let eq41_e1291_d_n2: f64 = ((eq41_e1289_d_n2 * s.v[848]) + (eq41_e1289 * s.dn[848][2]));
        let eq41_e1291_d_n3: f64 = ((eq41_e1289_d_n3 * s.v[848]) + (eq41_e1289 * s.dn[848][3]));
        let eq41_e1291_d_n4: f64 = ((eq41_e1289_d_n4 * s.v[848]) + (eq41_e1289 * s.dn[848][4]));
        let eq41_e1291_d_n5: f64 = ((eq41_e1289_d_n5 * s.v[848]) + (eq41_e1289 * s.dn[848][5]));
        let eq41_e1291_d_n6: f64 = ((eq41_e1289_d_n6 * s.v[848]) + (eq41_e1289 * s.dn[848][6]));
        let eq41_e1291_d_n7: f64 = ((eq41_e1289_d_n7 * s.v[848]) + (eq41_e1289 * s.dn[848][7]));
        let eq41_e1291_d_n8: f64 = ((eq41_e1289_d_n8 * s.v[848]) + (eq41_e1289 * s.dn[848][8]));
        let eq41_e1291_d_n9: f64 = ((eq41_e1289_d_n9 * s.v[848]) + (eq41_e1289 * s.dn[848][9]));
        let eq41_e1291_d_n10: f64 = ((eq41_e1289_d_n10 * s.v[848]) + (eq41_e1289 * s.dn[848][10]));
        let eq41_e1291_d_n11: f64 = ((eq41_e1289_d_n11 * s.v[848]) + (eq41_e1289 * s.dn[848][11]));
        let eq41_e1292: f64 = self.eval_ddt(3, eq41_e1291);
        let eq41_e1292_d_n0: f64 = self.ddt_jacobian(eq41_e1291_d_n0);
        let eq41_e1292_d_n1: f64 = self.ddt_jacobian(eq41_e1291_d_n1);
        let eq41_e1292_d_n2: f64 = self.ddt_jacobian(eq41_e1291_d_n2);
        let eq41_e1292_d_n3: f64 = self.ddt_jacobian(eq41_e1291_d_n3);
        let eq41_e1292_d_n4: f64 = self.ddt_jacobian(eq41_e1291_d_n4);
        let eq41_e1292_d_n5: f64 = self.ddt_jacobian(eq41_e1291_d_n5);
        let eq41_e1292_d_n6: f64 = self.ddt_jacobian(eq41_e1291_d_n6);
        let eq41_e1292_d_n7: f64 = self.ddt_jacobian(eq41_e1291_d_n7);
        let eq41_e1292_d_n8: f64 = self.ddt_jacobian(eq41_e1291_d_n8);
        let eq41_e1292_d_n9: f64 = self.ddt_jacobian(eq41_e1291_d_n9);
        let eq41_e1292_d_n10: f64 = self.ddt_jacobian(eq41_e1291_d_n10);
        let eq41_e1292_d_n11: f64 = self.ddt_jacobian(eq41_e1291_d_n11);
        let eq41_value: f64 = eq41_e1292;
        let eq41_node_derivatives: [f64; 12] = [eq41_e1292_d_n0, eq41_e1292_d_n1, eq41_e1292_d_n2, eq41_e1292_d_n3, eq41_e1292_d_n4, eq41_e1292_d_n5, eq41_e1292_d_n6, eq41_e1292_d_n7, eq41_e1292_d_n8, eq41_e1292_d_n9, eq41_e1292_d_n10, eq41_e1292_d_n11];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq41_value),
            &nodes,
            &eq41_node_derivatives,
            &branches,
            &eq41_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_42_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq42_e1295: f64 = (s.v[0] * s.v[15]);
        let eq42_e1295_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq42_e1295_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq42_e1295_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq42_e1295_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq42_e1295_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq42_e1295_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq42_e1295_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq42_e1295_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq42_e1295_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq42_e1295_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq42_e1295_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq42_e1295_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq42_e1297: f64 = (eq42_e1295 * p.p33);
        let eq42_e1297_d_n0: f64 = (eq42_e1295_d_n0 * p.p33);
        let eq42_e1297_d_n1: f64 = (eq42_e1295_d_n1 * p.p33);
        let eq42_e1297_d_n2: f64 = (eq42_e1295_d_n2 * p.p33);
        let eq42_e1297_d_n3: f64 = (eq42_e1295_d_n3 * p.p33);
        let eq42_e1297_d_n4: f64 = (eq42_e1295_d_n4 * p.p33);
        let eq42_e1297_d_n5: f64 = (eq42_e1295_d_n5 * p.p33);
        let eq42_e1297_d_n6: f64 = (eq42_e1295_d_n6 * p.p33);
        let eq42_e1297_d_n7: f64 = (eq42_e1295_d_n7 * p.p33);
        let eq42_e1297_d_n8: f64 = (eq42_e1295_d_n8 * p.p33);
        let eq42_e1297_d_n9: f64 = (eq42_e1295_d_n9 * p.p33);
        let eq42_e1297_d_n10: f64 = (eq42_e1295_d_n10 * p.p33);
        let eq42_e1297_d_n11: f64 = (eq42_e1295_d_n11 * p.p33);
        let eq42_e1299: f64 = (eq42_e1297 * s.v[849]);
        let eq42_e1299_d_n0: f64 = ((eq42_e1297_d_n0 * s.v[849]) + (eq42_e1297 * s.dn[849][0]));
        let eq42_e1299_d_n1: f64 = ((eq42_e1297_d_n1 * s.v[849]) + (eq42_e1297 * s.dn[849][1]));
        let eq42_e1299_d_n2: f64 = ((eq42_e1297_d_n2 * s.v[849]) + (eq42_e1297 * s.dn[849][2]));
        let eq42_e1299_d_n3: f64 = ((eq42_e1297_d_n3 * s.v[849]) + (eq42_e1297 * s.dn[849][3]));
        let eq42_e1299_d_n4: f64 = ((eq42_e1297_d_n4 * s.v[849]) + (eq42_e1297 * s.dn[849][4]));
        let eq42_e1299_d_n5: f64 = ((eq42_e1297_d_n5 * s.v[849]) + (eq42_e1297 * s.dn[849][5]));
        let eq42_e1299_d_n6: f64 = ((eq42_e1297_d_n6 * s.v[849]) + (eq42_e1297 * s.dn[849][6]));
        let eq42_e1299_d_n7: f64 = ((eq42_e1297_d_n7 * s.v[849]) + (eq42_e1297 * s.dn[849][7]));
        let eq42_e1299_d_n8: f64 = ((eq42_e1297_d_n8 * s.v[849]) + (eq42_e1297 * s.dn[849][8]));
        let eq42_e1299_d_n9: f64 = ((eq42_e1297_d_n9 * s.v[849]) + (eq42_e1297 * s.dn[849][9]));
        let eq42_e1299_d_n10: f64 = ((eq42_e1297_d_n10 * s.v[849]) + (eq42_e1297 * s.dn[849][10]));
        let eq42_e1299_d_n11: f64 = ((eq42_e1297_d_n11 * s.v[849]) + (eq42_e1297 * s.dn[849][11]));
        let eq42_e1300: f64 = self.eval_ddt(4, eq42_e1299);
        let eq42_e1300_d_n0: f64 = self.ddt_jacobian(eq42_e1299_d_n0);
        let eq42_e1300_d_n1: f64 = self.ddt_jacobian(eq42_e1299_d_n1);
        let eq42_e1300_d_n2: f64 = self.ddt_jacobian(eq42_e1299_d_n2);
        let eq42_e1300_d_n3: f64 = self.ddt_jacobian(eq42_e1299_d_n3);
        let eq42_e1300_d_n4: f64 = self.ddt_jacobian(eq42_e1299_d_n4);
        let eq42_e1300_d_n5: f64 = self.ddt_jacobian(eq42_e1299_d_n5);
        let eq42_e1300_d_n6: f64 = self.ddt_jacobian(eq42_e1299_d_n6);
        let eq42_e1300_d_n7: f64 = self.ddt_jacobian(eq42_e1299_d_n7);
        let eq42_e1300_d_n8: f64 = self.ddt_jacobian(eq42_e1299_d_n8);
        let eq42_e1300_d_n9: f64 = self.ddt_jacobian(eq42_e1299_d_n9);
        let eq42_e1300_d_n10: f64 = self.ddt_jacobian(eq42_e1299_d_n10);
        let eq42_e1300_d_n11: f64 = self.ddt_jacobian(eq42_e1299_d_n11);
        let eq42_value: f64 = eq42_e1300;
        let eq42_node_derivatives: [f64; 12] = [eq42_e1300_d_n0, eq42_e1300_d_n1, eq42_e1300_d_n2, eq42_e1300_d_n3, eq42_e1300_d_n4, eq42_e1300_d_n5, eq42_e1300_d_n6, eq42_e1300_d_n7, eq42_e1300_d_n8, eq42_e1300_d_n9, eq42_e1300_d_n10, eq42_e1300_d_n11];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq42_value),
            &nodes,
            &eq42_node_derivatives,
            &branches,
            &eq42_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_43_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq43_e1303: f64 = (s.v[0] * s.v[15]);
        let eq43_e1303_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq43_e1303_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq43_e1303_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq43_e1303_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq43_e1303_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq43_e1303_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq43_e1303_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq43_e1303_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq43_e1303_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq43_e1303_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq43_e1303_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq43_e1303_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq43_e1305: f64 = (eq43_e1303 * p.p33);
        let eq43_e1305_d_n0: f64 = (eq43_e1303_d_n0 * p.p33);
        let eq43_e1305_d_n1: f64 = (eq43_e1303_d_n1 * p.p33);
        let eq43_e1305_d_n2: f64 = (eq43_e1303_d_n2 * p.p33);
        let eq43_e1305_d_n3: f64 = (eq43_e1303_d_n3 * p.p33);
        let eq43_e1305_d_n4: f64 = (eq43_e1303_d_n4 * p.p33);
        let eq43_e1305_d_n5: f64 = (eq43_e1303_d_n5 * p.p33);
        let eq43_e1305_d_n6: f64 = (eq43_e1303_d_n6 * p.p33);
        let eq43_e1305_d_n7: f64 = (eq43_e1303_d_n7 * p.p33);
        let eq43_e1305_d_n8: f64 = (eq43_e1303_d_n8 * p.p33);
        let eq43_e1305_d_n9: f64 = (eq43_e1303_d_n9 * p.p33);
        let eq43_e1305_d_n10: f64 = (eq43_e1303_d_n10 * p.p33);
        let eq43_e1305_d_n11: f64 = (eq43_e1303_d_n11 * p.p33);
        let eq43_e1307: f64 = (eq43_e1305 * s.v[850]);
        let eq43_e1307_d_n0: f64 = ((eq43_e1305_d_n0 * s.v[850]) + (eq43_e1305 * s.dn[850][0]));
        let eq43_e1307_d_n1: f64 = ((eq43_e1305_d_n1 * s.v[850]) + (eq43_e1305 * s.dn[850][1]));
        let eq43_e1307_d_n2: f64 = ((eq43_e1305_d_n2 * s.v[850]) + (eq43_e1305 * s.dn[850][2]));
        let eq43_e1307_d_n3: f64 = ((eq43_e1305_d_n3 * s.v[850]) + (eq43_e1305 * s.dn[850][3]));
        let eq43_e1307_d_n4: f64 = ((eq43_e1305_d_n4 * s.v[850]) + (eq43_e1305 * s.dn[850][4]));
        let eq43_e1307_d_n5: f64 = ((eq43_e1305_d_n5 * s.v[850]) + (eq43_e1305 * s.dn[850][5]));
        let eq43_e1307_d_n6: f64 = ((eq43_e1305_d_n6 * s.v[850]) + (eq43_e1305 * s.dn[850][6]));
        let eq43_e1307_d_n7: f64 = ((eq43_e1305_d_n7 * s.v[850]) + (eq43_e1305 * s.dn[850][7]));
        let eq43_e1307_d_n8: f64 = ((eq43_e1305_d_n8 * s.v[850]) + (eq43_e1305 * s.dn[850][8]));
        let eq43_e1307_d_n9: f64 = ((eq43_e1305_d_n9 * s.v[850]) + (eq43_e1305 * s.dn[850][9]));
        let eq43_e1307_d_n10: f64 = ((eq43_e1305_d_n10 * s.v[850]) + (eq43_e1305 * s.dn[850][10]));
        let eq43_e1307_d_n11: f64 = ((eq43_e1305_d_n11 * s.v[850]) + (eq43_e1305 * s.dn[850][11]));
        let eq43_e1308: f64 = self.eval_ddt(5, eq43_e1307);
        let eq43_e1308_d_n0: f64 = self.ddt_jacobian(eq43_e1307_d_n0);
        let eq43_e1308_d_n1: f64 = self.ddt_jacobian(eq43_e1307_d_n1);
        let eq43_e1308_d_n2: f64 = self.ddt_jacobian(eq43_e1307_d_n2);
        let eq43_e1308_d_n3: f64 = self.ddt_jacobian(eq43_e1307_d_n3);
        let eq43_e1308_d_n4: f64 = self.ddt_jacobian(eq43_e1307_d_n4);
        let eq43_e1308_d_n5: f64 = self.ddt_jacobian(eq43_e1307_d_n5);
        let eq43_e1308_d_n6: f64 = self.ddt_jacobian(eq43_e1307_d_n6);
        let eq43_e1308_d_n7: f64 = self.ddt_jacobian(eq43_e1307_d_n7);
        let eq43_e1308_d_n8: f64 = self.ddt_jacobian(eq43_e1307_d_n8);
        let eq43_e1308_d_n9: f64 = self.ddt_jacobian(eq43_e1307_d_n9);
        let eq43_e1308_d_n10: f64 = self.ddt_jacobian(eq43_e1307_d_n10);
        let eq43_e1308_d_n11: f64 = self.ddt_jacobian(eq43_e1307_d_n11);
        let eq43_value: f64 = eq43_e1308;
        let eq43_node_derivatives: [f64; 12] = [eq43_e1308_d_n0, eq43_e1308_d_n1, eq43_e1308_d_n2, eq43_e1308_d_n3, eq43_e1308_d_n4, eq43_e1308_d_n5, eq43_e1308_d_n6, eq43_e1308_d_n7, eq43_e1308_d_n8, eq43_e1308_d_n9, eq43_e1308_d_n10, eq43_e1308_d_n11];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[8]),
            self.multiplicity * (eq43_value),
            &nodes,
            &eq43_node_derivatives,
            &branches,
            &eq43_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_44_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq44_e1311: f64 = (s.v[0] * s.v[15]);
        let eq44_e1311_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq44_e1311_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq44_e1311_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq44_e1311_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq44_e1311_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq44_e1311_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq44_e1311_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq44_e1311_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq44_e1311_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq44_e1311_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq44_e1311_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq44_e1311_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq44_e1313: f64 = (eq44_e1311 * p.p33);
        let eq44_e1313_d_n0: f64 = (eq44_e1311_d_n0 * p.p33);
        let eq44_e1313_d_n1: f64 = (eq44_e1311_d_n1 * p.p33);
        let eq44_e1313_d_n2: f64 = (eq44_e1311_d_n2 * p.p33);
        let eq44_e1313_d_n3: f64 = (eq44_e1311_d_n3 * p.p33);
        let eq44_e1313_d_n4: f64 = (eq44_e1311_d_n4 * p.p33);
        let eq44_e1313_d_n5: f64 = (eq44_e1311_d_n5 * p.p33);
        let eq44_e1313_d_n6: f64 = (eq44_e1311_d_n6 * p.p33);
        let eq44_e1313_d_n7: f64 = (eq44_e1311_d_n7 * p.p33);
        let eq44_e1313_d_n8: f64 = (eq44_e1311_d_n8 * p.p33);
        let eq44_e1313_d_n9: f64 = (eq44_e1311_d_n9 * p.p33);
        let eq44_e1313_d_n10: f64 = (eq44_e1311_d_n10 * p.p33);
        let eq44_e1313_d_n11: f64 = (eq44_e1311_d_n11 * p.p33);
        let eq44_e1315: f64 = (eq44_e1313 * s.v[851]);
        let eq44_e1315_d_n0: f64 = ((eq44_e1313_d_n0 * s.v[851]) + (eq44_e1313 * s.dn[851][0]));
        let eq44_e1315_d_n1: f64 = ((eq44_e1313_d_n1 * s.v[851]) + (eq44_e1313 * s.dn[851][1]));
        let eq44_e1315_d_n2: f64 = ((eq44_e1313_d_n2 * s.v[851]) + (eq44_e1313 * s.dn[851][2]));
        let eq44_e1315_d_n3: f64 = ((eq44_e1313_d_n3 * s.v[851]) + (eq44_e1313 * s.dn[851][3]));
        let eq44_e1315_d_n4: f64 = ((eq44_e1313_d_n4 * s.v[851]) + (eq44_e1313 * s.dn[851][4]));
        let eq44_e1315_d_n5: f64 = ((eq44_e1313_d_n5 * s.v[851]) + (eq44_e1313 * s.dn[851][5]));
        let eq44_e1315_d_n6: f64 = ((eq44_e1313_d_n6 * s.v[851]) + (eq44_e1313 * s.dn[851][6]));
        let eq44_e1315_d_n7: f64 = ((eq44_e1313_d_n7 * s.v[851]) + (eq44_e1313 * s.dn[851][7]));
        let eq44_e1315_d_n8: f64 = ((eq44_e1313_d_n8 * s.v[851]) + (eq44_e1313 * s.dn[851][8]));
        let eq44_e1315_d_n9: f64 = ((eq44_e1313_d_n9 * s.v[851]) + (eq44_e1313 * s.dn[851][9]));
        let eq44_e1315_d_n10: f64 = ((eq44_e1313_d_n10 * s.v[851]) + (eq44_e1313 * s.dn[851][10]));
        let eq44_e1315_d_n11: f64 = ((eq44_e1313_d_n11 * s.v[851]) + (eq44_e1313 * s.dn[851][11]));
        let eq44_e1316: f64 = self.eval_ddt(6, eq44_e1315);
        let eq44_e1316_d_n0: f64 = self.ddt_jacobian(eq44_e1315_d_n0);
        let eq44_e1316_d_n1: f64 = self.ddt_jacobian(eq44_e1315_d_n1);
        let eq44_e1316_d_n2: f64 = self.ddt_jacobian(eq44_e1315_d_n2);
        let eq44_e1316_d_n3: f64 = self.ddt_jacobian(eq44_e1315_d_n3);
        let eq44_e1316_d_n4: f64 = self.ddt_jacobian(eq44_e1315_d_n4);
        let eq44_e1316_d_n5: f64 = self.ddt_jacobian(eq44_e1315_d_n5);
        let eq44_e1316_d_n6: f64 = self.ddt_jacobian(eq44_e1315_d_n6);
        let eq44_e1316_d_n7: f64 = self.ddt_jacobian(eq44_e1315_d_n7);
        let eq44_e1316_d_n8: f64 = self.ddt_jacobian(eq44_e1315_d_n8);
        let eq44_e1316_d_n9: f64 = self.ddt_jacobian(eq44_e1315_d_n9);
        let eq44_e1316_d_n10: f64 = self.ddt_jacobian(eq44_e1315_d_n10);
        let eq44_e1316_d_n11: f64 = self.ddt_jacobian(eq44_e1315_d_n11);
        let eq44_value: f64 = eq44_e1316;
        let eq44_node_derivatives: [f64; 12] = [eq44_e1316_d_n0, eq44_e1316_d_n1, eq44_e1316_d_n2, eq44_e1316_d_n3, eq44_e1316_d_n4, eq44_e1316_d_n5, eq44_e1316_d_n6, eq44_e1316_d_n7, eq44_e1316_d_n8, eq44_e1316_d_n9, eq44_e1316_d_n10, eq44_e1316_d_n11];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            self.multiplicity * (eq44_value),
            &nodes,
            &eq44_node_derivatives,
            &branches,
            &eq44_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_45_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq45_e1319: f64 = (s.v[0] * s.v[15]);
        let eq45_e1319_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq45_e1319_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq45_e1319_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq45_e1319_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq45_e1319_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq45_e1319_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq45_e1319_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq45_e1319_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq45_e1319_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq45_e1319_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq45_e1319_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq45_e1319_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq45_e1321: f64 = (eq45_e1319 * p.p33);
        let eq45_e1321_d_n0: f64 = (eq45_e1319_d_n0 * p.p33);
        let eq45_e1321_d_n1: f64 = (eq45_e1319_d_n1 * p.p33);
        let eq45_e1321_d_n2: f64 = (eq45_e1319_d_n2 * p.p33);
        let eq45_e1321_d_n3: f64 = (eq45_e1319_d_n3 * p.p33);
        let eq45_e1321_d_n4: f64 = (eq45_e1319_d_n4 * p.p33);
        let eq45_e1321_d_n5: f64 = (eq45_e1319_d_n5 * p.p33);
        let eq45_e1321_d_n6: f64 = (eq45_e1319_d_n6 * p.p33);
        let eq45_e1321_d_n7: f64 = (eq45_e1319_d_n7 * p.p33);
        let eq45_e1321_d_n8: f64 = (eq45_e1319_d_n8 * p.p33);
        let eq45_e1321_d_n9: f64 = (eq45_e1319_d_n9 * p.p33);
        let eq45_e1321_d_n10: f64 = (eq45_e1319_d_n10 * p.p33);
        let eq45_e1321_d_n11: f64 = (eq45_e1319_d_n11 * p.p33);
        let eq45_e1323: f64 = (eq45_e1321 * s.v[852]);
        let eq45_e1323_d_n0: f64 = ((eq45_e1321_d_n0 * s.v[852]) + (eq45_e1321 * s.dn[852][0]));
        let eq45_e1323_d_n1: f64 = ((eq45_e1321_d_n1 * s.v[852]) + (eq45_e1321 * s.dn[852][1]));
        let eq45_e1323_d_n2: f64 = ((eq45_e1321_d_n2 * s.v[852]) + (eq45_e1321 * s.dn[852][2]));
        let eq45_e1323_d_n3: f64 = ((eq45_e1321_d_n3 * s.v[852]) + (eq45_e1321 * s.dn[852][3]));
        let eq45_e1323_d_n4: f64 = ((eq45_e1321_d_n4 * s.v[852]) + (eq45_e1321 * s.dn[852][4]));
        let eq45_e1323_d_n5: f64 = ((eq45_e1321_d_n5 * s.v[852]) + (eq45_e1321 * s.dn[852][5]));
        let eq45_e1323_d_n6: f64 = ((eq45_e1321_d_n6 * s.v[852]) + (eq45_e1321 * s.dn[852][6]));
        let eq45_e1323_d_n7: f64 = ((eq45_e1321_d_n7 * s.v[852]) + (eq45_e1321 * s.dn[852][7]));
        let eq45_e1323_d_n8: f64 = ((eq45_e1321_d_n8 * s.v[852]) + (eq45_e1321 * s.dn[852][8]));
        let eq45_e1323_d_n9: f64 = ((eq45_e1321_d_n9 * s.v[852]) + (eq45_e1321 * s.dn[852][9]));
        let eq45_e1323_d_n10: f64 = ((eq45_e1321_d_n10 * s.v[852]) + (eq45_e1321 * s.dn[852][10]));
        let eq45_e1323_d_n11: f64 = ((eq45_e1321_d_n11 * s.v[852]) + (eq45_e1321 * s.dn[852][11]));
        let eq45_e1324: f64 = self.eval_ddt(7, eq45_e1323);
        let eq45_e1324_d_n0: f64 = self.ddt_jacobian(eq45_e1323_d_n0);
        let eq45_e1324_d_n1: f64 = self.ddt_jacobian(eq45_e1323_d_n1);
        let eq45_e1324_d_n2: f64 = self.ddt_jacobian(eq45_e1323_d_n2);
        let eq45_e1324_d_n3: f64 = self.ddt_jacobian(eq45_e1323_d_n3);
        let eq45_e1324_d_n4: f64 = self.ddt_jacobian(eq45_e1323_d_n4);
        let eq45_e1324_d_n5: f64 = self.ddt_jacobian(eq45_e1323_d_n5);
        let eq45_e1324_d_n6: f64 = self.ddt_jacobian(eq45_e1323_d_n6);
        let eq45_e1324_d_n7: f64 = self.ddt_jacobian(eq45_e1323_d_n7);
        let eq45_e1324_d_n8: f64 = self.ddt_jacobian(eq45_e1323_d_n8);
        let eq45_e1324_d_n9: f64 = self.ddt_jacobian(eq45_e1323_d_n9);
        let eq45_e1324_d_n10: f64 = self.ddt_jacobian(eq45_e1323_d_n10);
        let eq45_e1324_d_n11: f64 = self.ddt_jacobian(eq45_e1323_d_n11);
        let eq45_value: f64 = eq45_e1324;
        let eq45_node_derivatives: [f64; 12] = [eq45_e1324_d_n0, eq45_e1324_d_n1, eq45_e1324_d_n2, eq45_e1324_d_n3, eq45_e1324_d_n4, eq45_e1324_d_n5, eq45_e1324_d_n6, eq45_e1324_d_n7, eq45_e1324_d_n8, eq45_e1324_d_n9, eq45_e1324_d_n10, eq45_e1324_d_n11];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq45_value),
            &nodes,
            &eq45_node_derivatives,
            &branches,
            &eq45_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_47_block_0(
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
        let eq47_e1332: f64 = ((nv4 - 0.0) / s.v[853]);
        let eq47_e1332_d_n0: f64 = (-(((nv4 - 0.0) * s.dn[853][0]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n1: f64 = (-(((nv4 - 0.0) * s.dn[853][1]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n2: f64 = (-(((nv4 - 0.0) * s.dn[853][2]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n3: f64 = (-(((nv4 - 0.0) * s.dn[853][3]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n4: f64 = ((s.v[853] - ((nv4 - 0.0) * s.dn[853][4])) / (s.v[853] * s.v[853]));
        let eq47_e1332_d_n5: f64 = (-(((nv4 - 0.0) * s.dn[853][5]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n6: f64 = (-(((nv4 - 0.0) * s.dn[853][6]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n7: f64 = (-(((nv4 - 0.0) * s.dn[853][7]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n8: f64 = (-(((nv4 - 0.0) * s.dn[853][8]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n9: f64 = (-(((nv4 - 0.0) * s.dn[853][9]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n10: f64 = (-(((nv4 - 0.0) * s.dn[853][10]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n11: f64 = (-(((nv4 - 0.0) * s.dn[853][11]) / (s.v[853] * s.v[853])));
        let eq47_value: f64 = eq47_e1332;
        let eq47_node_derivatives: [f64; 12] = [eq47_e1332_d_n0, eq47_e1332_d_n1, eq47_e1332_d_n2, eq47_e1332_d_n3, eq47_e1332_d_n4, eq47_e1332_d_n5, eq47_e1332_d_n6, eq47_e1332_d_n7, eq47_e1332_d_n8, eq47_e1332_d_n9, eq47_e1332_d_n10, eq47_e1332_d_n11];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq47_value),
            &nodes,
            &eq47_node_derivatives,
            &branches,
            &eq47_branch_derivatives,
            self.multiplicity,
        );
    }
}
