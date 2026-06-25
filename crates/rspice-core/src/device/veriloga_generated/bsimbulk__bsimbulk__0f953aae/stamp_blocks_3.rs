#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq6_e1262,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e1262;
        stamper.stamp_current(
            Some(nodes[15]),
            None,
            self.multiplicity * (eq6_value),
            &[
            ],
        );
    }

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
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq7_e1272, eq7_e1272_d_n0, eq7_e1272_d_n1, eq7_e1272_d_n2, eq7_e1272_d_n3, eq7_e1272_d_n4, eq7_e1272_d_n5, eq7_e1272_d_n6, eq7_e1272_d_n7, eq7_e1272_d_n8, eq7_e1272_d_n9, eq7_e1272_d_n10, eq7_e1272_d_n11, eq7_e1272_d_n12, eq7_e1272_d_n13, eq7_e1272_d_n14, eq7_e1272_d_n15, eq7_e1272_d_n16,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        let eq7_e1268: f64 = (-s.v[385]);
        let eq7_e1268_d_n0: f64 = (-s.dn[385][0]);
        let eq7_e1268_d_n1: f64 = (-s.dn[385][1]);
        let eq7_e1268_d_n2: f64 = (-s.dn[385][2]);
        let eq7_e1268_d_n3: f64 = (-s.dn[385][3]);
        let eq7_e1268_d_n4: f64 = (-s.dn[385][4]);
        let eq7_e1268_d_n5: f64 = (-s.dn[385][5]);
        let eq7_e1268_d_n6: f64 = (-s.dn[385][6]);
        let eq7_e1268_d_n7: f64 = (-s.dn[385][7]);
        let eq7_e1268_d_n8: f64 = (-s.dn[385][8]);
        let eq7_e1268_d_n9: f64 = (-s.dn[385][9]);
        let eq7_e1268_d_n10: f64 = (-s.dn[385][10]);
        let eq7_e1268_d_n11: f64 = (-s.dn[385][11]);
        let eq7_e1268_d_n12: f64 = (-s.dn[385][12]);
        let eq7_e1268_d_n13: f64 = (-s.dn[385][13]);
        let eq7_e1268_d_n14: f64 = (-s.dn[385][14]);
        let eq7_e1268_d_n15: f64 = (-s.dn[385][15]);
        let eq7_e1268_d_n16: f64 = (-s.dn[385][16]);
        let eq7_e1270: f64 = (eq7_e1268 * (nv16 - 0.0));
        let eq7_e1270_d_n0: f64 = (eq7_e1268_d_n0 * (nv16 - 0.0));
        let eq7_e1270_d_n1: f64 = (eq7_e1268_d_n1 * (nv16 - 0.0));
        let eq7_e1270_d_n2: f64 = (eq7_e1268_d_n2 * (nv16 - 0.0));
        let eq7_e1270_d_n3: f64 = (eq7_e1268_d_n3 * (nv16 - 0.0));
        let eq7_e1270_d_n4: f64 = (eq7_e1268_d_n4 * (nv16 - 0.0));
        let eq7_e1270_d_n5: f64 = (eq7_e1268_d_n5 * (nv16 - 0.0));
        let eq7_e1270_d_n6: f64 = (eq7_e1268_d_n6 * (nv16 - 0.0));
        let eq7_e1270_d_n7: f64 = (eq7_e1268_d_n7 * (nv16 - 0.0));
        let eq7_e1270_d_n8: f64 = (eq7_e1268_d_n8 * (nv16 - 0.0));
        let eq7_e1270_d_n9: f64 = (eq7_e1268_d_n9 * (nv16 - 0.0));
        let eq7_e1270_d_n10: f64 = (eq7_e1268_d_n10 * (nv16 - 0.0));
        let eq7_e1270_d_n11: f64 = (eq7_e1268_d_n11 * (nv16 - 0.0));
        let eq7_e1270_d_n12: f64 = (eq7_e1268_d_n12 * (nv16 - 0.0));
        let eq7_e1270_d_n13: f64 = (eq7_e1268_d_n13 * (nv16 - 0.0));
        let eq7_e1270_d_n14: f64 = (eq7_e1268_d_n14 * (nv16 - 0.0));
        let eq7_e1270_d_n15: f64 = (eq7_e1268_d_n15 * (nv16 - 0.0));
        let eq7_e1270_d_n16: f64 = ((eq7_e1268_d_n16 * (nv16 - 0.0)) + eq7_e1268);
        (eq7_e1270, eq7_e1270_d_n0, eq7_e1270_d_n1, eq7_e1270_d_n2, eq7_e1270_d_n3, eq7_e1270_d_n4, eq7_e1270_d_n5, eq7_e1270_d_n6, eq7_e1270_d_n7, eq7_e1270_d_n8, eq7_e1270_d_n9, eq7_e1270_d_n10, eq7_e1270_d_n11, eq7_e1270_d_n12, eq7_e1270_d_n13, eq7_e1270_d_n14, eq7_e1270_d_n15, eq7_e1270_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1272;
        let eq7_node_derivatives: [f64; 17] = [eq7_e1272_d_n0, eq7_e1272_d_n1, eq7_e1272_d_n2, eq7_e1272_d_n3, eq7_e1272_d_n4, eq7_e1272_d_n5, eq7_e1272_d_n6, eq7_e1272_d_n7, eq7_e1272_d_n8, eq7_e1272_d_n9, eq7_e1272_d_n10, eq7_e1272_d_n11, eq7_e1272_d_n12, eq7_e1272_d_n13, eq7_e1272_d_n14, eq7_e1272_d_n15, eq7_e1272_d_n16];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            None,
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n1, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_d_n16,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        let eq8_e1279: f64 = (s.v[378] * s.v[46]);
        let eq8_e1279_d_n0: f64 = (s.dn[378][0] * s.v[46]);
        let eq8_e1279_d_n1: f64 = (s.dn[378][1] * s.v[46]);
        let eq8_e1279_d_n2: f64 = (s.dn[378][2] * s.v[46]);
        let eq8_e1279_d_n3: f64 = (s.dn[378][3] * s.v[46]);
        let eq8_e1279_d_n4: f64 = (s.dn[378][4] * s.v[46]);
        let eq8_e1279_d_n5: f64 = (s.dn[378][5] * s.v[46]);
        let eq8_e1279_d_n6: f64 = (s.dn[378][6] * s.v[46]);
        let eq8_e1279_d_n7: f64 = (s.dn[378][7] * s.v[46]);
        let eq8_e1279_d_n8: f64 = (s.dn[378][8] * s.v[46]);
        let eq8_e1279_d_n9: f64 = (s.dn[378][9] * s.v[46]);
        let eq8_e1279_d_n10: f64 = (s.dn[378][10] * s.v[46]);
        let eq8_e1279_d_n11: f64 = (s.dn[378][11] * s.v[46]);
        let eq8_e1279_d_n12: f64 = (s.dn[378][12] * s.v[46]);
        let eq8_e1279_d_n13: f64 = (s.dn[378][13] * s.v[46]);
        let eq8_e1279_d_n14: f64 = (s.dn[378][14] * s.v[46]);
        let eq8_e1279_d_n15: f64 = (s.dn[378][15] * s.v[46]);
        let eq8_e1279_d_n16: f64 = (s.dn[378][16] * s.v[46]);
        let eq8_e1281: f64 = (eq8_e1279 * s.v[29]);
        let eq8_e1281_d_n0: f64 = (eq8_e1279_d_n0 * s.v[29]);
        let eq8_e1281_d_n1: f64 = (eq8_e1279_d_n1 * s.v[29]);
        let eq8_e1281_d_n2: f64 = (eq8_e1279_d_n2 * s.v[29]);
        let eq8_e1281_d_n3: f64 = (eq8_e1279_d_n3 * s.v[29]);
        let eq8_e1281_d_n4: f64 = (eq8_e1279_d_n4 * s.v[29]);
        let eq8_e1281_d_n5: f64 = (eq8_e1279_d_n5 * s.v[29]);
        let eq8_e1281_d_n6: f64 = (eq8_e1279_d_n6 * s.v[29]);
        let eq8_e1281_d_n7: f64 = (eq8_e1279_d_n7 * s.v[29]);
        let eq8_e1281_d_n8: f64 = (eq8_e1279_d_n8 * s.v[29]);
        let eq8_e1281_d_n9: f64 = (eq8_e1279_d_n9 * s.v[29]);
        let eq8_e1281_d_n10: f64 = (eq8_e1279_d_n10 * s.v[29]);
        let eq8_e1281_d_n11: f64 = (eq8_e1279_d_n11 * s.v[29]);
        let eq8_e1281_d_n12: f64 = (eq8_e1279_d_n12 * s.v[29]);
        let eq8_e1281_d_n13: f64 = (eq8_e1279_d_n13 * s.v[29]);
        let eq8_e1281_d_n14: f64 = (eq8_e1279_d_n14 * s.v[29]);
        let eq8_e1281_d_n15: f64 = (eq8_e1279_d_n15 * s.v[29]);
        let eq8_e1281_d_n16: f64 = (eq8_e1279_d_n16 * s.v[29]);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n1: f64 = (eq8_e1281_d_n1 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1283_d_n15: f64 = (eq8_e1281_d_n15 * p.p2);
        let eq8_e1283_d_n16: f64 = (eq8_e1281_d_n16 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * s.v[30]);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * s.v[30]);
        let eq8_e1285_d_n1: f64 = (eq8_e1283_d_n1 * s.v[30]);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * s.v[30]);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * s.v[30]);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * s.v[30]);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * s.v[30]);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * s.v[30]);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * s.v[30]);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * s.v[30]);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * s.v[30]);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * s.v[30]);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * s.v[30]);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * s.v[30]);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * s.v[30]);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * s.v[30]);
        let eq8_e1285_d_n15: f64 = (eq8_e1283_d_n15 * s.v[30]);
        let eq8_e1285_d_n16: f64 = (eq8_e1283_d_n16 * s.v[30]);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n1: f64 = (eq8_e1285_d_n1 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1287_d_n15: f64 = ((eq8_e1285_d_n15 * (nv15 - 0.0)) + eq8_e1285);
        let eq8_e1287_d_n16: f64 = (eq8_e1285_d_n16 * (nv15 - 0.0));
        let eq8_e1288: f64 = self.eval_ddt(0, eq8_e1287);
        let eq8_e1288_d_n0: f64 = self.ddt_jacobian(eq8_e1287_d_n0);
        let eq8_e1288_d_n1: f64 = self.ddt_jacobian(eq8_e1287_d_n1);
        let eq8_e1288_d_n2: f64 = self.ddt_jacobian(eq8_e1287_d_n2);
        let eq8_e1288_d_n3: f64 = self.ddt_jacobian(eq8_e1287_d_n3);
        let eq8_e1288_d_n4: f64 = self.ddt_jacobian(eq8_e1287_d_n4);
        let eq8_e1288_d_n5: f64 = self.ddt_jacobian(eq8_e1287_d_n5);
        let eq8_e1288_d_n6: f64 = self.ddt_jacobian(eq8_e1287_d_n6);
        let eq8_e1288_d_n7: f64 = self.ddt_jacobian(eq8_e1287_d_n7);
        let eq8_e1288_d_n8: f64 = self.ddt_jacobian(eq8_e1287_d_n8);
        let eq8_e1288_d_n9: f64 = self.ddt_jacobian(eq8_e1287_d_n9);
        let eq8_e1288_d_n10: f64 = self.ddt_jacobian(eq8_e1287_d_n10);
        let eq8_e1288_d_n11: f64 = self.ddt_jacobian(eq8_e1287_d_n11);
        let eq8_e1288_d_n12: f64 = self.ddt_jacobian(eq8_e1287_d_n12);
        let eq8_e1288_d_n13: f64 = self.ddt_jacobian(eq8_e1287_d_n13);
        let eq8_e1288_d_n14: f64 = self.ddt_jacobian(eq8_e1287_d_n14);
        let eq8_e1288_d_n15: f64 = self.ddt_jacobian(eq8_e1287_d_n15);
        let eq8_e1288_d_n16: f64 = self.ddt_jacobian(eq8_e1287_d_n16);
        (eq8_e1288, eq8_e1288_d_n0, eq8_e1288_d_n1, eq8_e1288_d_n2, eq8_e1288_d_n3, eq8_e1288_d_n4, eq8_e1288_d_n5, eq8_e1288_d_n6, eq8_e1288_d_n7, eq8_e1288_d_n8, eq8_e1288_d_n9, eq8_e1288_d_n10, eq8_e1288_d_n11, eq8_e1288_d_n12, eq8_e1288_d_n13, eq8_e1288_d_n14, eq8_e1288_d_n15, eq8_e1288_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e1290;
        let eq8_node_derivatives: [f64; 17] = [eq8_e1290_d_n0, eq8_e1290_d_n1, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_d_n16];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            None,
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
        let (eq9_e1307,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e1307;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq9_value),
            &[
            ],
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
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq10_e1318, eq10_e1318_d_n0, eq10_e1318_d_n1, eq10_e1318_d_n2, eq10_e1318_d_n3, eq10_e1318_d_n4, eq10_e1318_d_n5, eq10_e1318_d_n6, eq10_e1318_d_n7, eq10_e1318_d_n8, eq10_e1318_d_n9, eq10_e1318_d_n10, eq10_e1318_d_n11, eq10_e1318_d_n12, eq10_e1318_d_n13, eq10_e1318_d_n14, eq10_e1318_d_n15, eq10_e1318_d_n16,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        let eq10_e1314: f64 = (s.v[384] * p.p28);
        let eq10_e1314_d_n0: f64 = (s.dn[384][0] * p.p28);
        let eq10_e1314_d_n1: f64 = (s.dn[384][1] * p.p28);
        let eq10_e1314_d_n2: f64 = (s.dn[384][2] * p.p28);
        let eq10_e1314_d_n3: f64 = (s.dn[384][3] * p.p28);
        let eq10_e1314_d_n4: f64 = (s.dn[384][4] * p.p28);
        let eq10_e1314_d_n5: f64 = (s.dn[384][5] * p.p28);
        let eq10_e1314_d_n6: f64 = (s.dn[384][6] * p.p28);
        let eq10_e1314_d_n7: f64 = (s.dn[384][7] * p.p28);
        let eq10_e1314_d_n8: f64 = (s.dn[384][8] * p.p28);
        let eq10_e1314_d_n9: f64 = (s.dn[384][9] * p.p28);
        let eq10_e1314_d_n10: f64 = (s.dn[384][10] * p.p28);
        let eq10_e1314_d_n11: f64 = (s.dn[384][11] * p.p28);
        let eq10_e1314_d_n12: f64 = (s.dn[384][12] * p.p28);
        let eq10_e1314_d_n13: f64 = (s.dn[384][13] * p.p28);
        let eq10_e1314_d_n14: f64 = (s.dn[384][14] * p.p28);
        let eq10_e1314_d_n15: f64 = (s.dn[384][15] * p.p28);
        let eq10_e1314_d_n16: f64 = (s.dn[384][16] * p.p28);
        let eq10_e1316: f64 = (eq10_e1314 * (nv16 - 0.0));
        let eq10_e1316_d_n0: f64 = (eq10_e1314_d_n0 * (nv16 - 0.0));
        let eq10_e1316_d_n1: f64 = (eq10_e1314_d_n1 * (nv16 - 0.0));
        let eq10_e1316_d_n2: f64 = (eq10_e1314_d_n2 * (nv16 - 0.0));
        let eq10_e1316_d_n3: f64 = (eq10_e1314_d_n3 * (nv16 - 0.0));
        let eq10_e1316_d_n4: f64 = (eq10_e1314_d_n4 * (nv16 - 0.0));
        let eq10_e1316_d_n5: f64 = (eq10_e1314_d_n5 * (nv16 - 0.0));
        let eq10_e1316_d_n6: f64 = (eq10_e1314_d_n6 * (nv16 - 0.0));
        let eq10_e1316_d_n7: f64 = (eq10_e1314_d_n7 * (nv16 - 0.0));
        let eq10_e1316_d_n8: f64 = (eq10_e1314_d_n8 * (nv16 - 0.0));
        let eq10_e1316_d_n9: f64 = (eq10_e1314_d_n9 * (nv16 - 0.0));
        let eq10_e1316_d_n10: f64 = (eq10_e1314_d_n10 * (nv16 - 0.0));
        let eq10_e1316_d_n11: f64 = (eq10_e1314_d_n11 * (nv16 - 0.0));
        let eq10_e1316_d_n12: f64 = (eq10_e1314_d_n12 * (nv16 - 0.0));
        let eq10_e1316_d_n13: f64 = (eq10_e1314_d_n13 * (nv16 - 0.0));
        let eq10_e1316_d_n14: f64 = (eq10_e1314_d_n14 * (nv16 - 0.0));
        let eq10_e1316_d_n15: f64 = (eq10_e1314_d_n15 * (nv16 - 0.0));
        let eq10_e1316_d_n16: f64 = ((eq10_e1314_d_n16 * (nv16 - 0.0)) + eq10_e1314);
        (eq10_e1316, eq10_e1316_d_n0, eq10_e1316_d_n1, eq10_e1316_d_n2, eq10_e1316_d_n3, eq10_e1316_d_n4, eq10_e1316_d_n5, eq10_e1316_d_n6, eq10_e1316_d_n7, eq10_e1316_d_n8, eq10_e1316_d_n9, eq10_e1316_d_n10, eq10_e1316_d_n11, eq10_e1316_d_n12, eq10_e1316_d_n13, eq10_e1316_d_n14, eq10_e1316_d_n15, eq10_e1316_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1318;
        let eq10_node_derivatives: [f64; 17] = [eq10_e1318_d_n0, eq10_e1318_d_n1, eq10_e1318_d_n2, eq10_e1318_d_n3, eq10_e1318_d_n4, eq10_e1318_d_n5, eq10_e1318_d_n6, eq10_e1318_d_n7, eq10_e1318_d_n8, eq10_e1318_d_n9, eq10_e1318_d_n10, eq10_e1318_d_n11, eq10_e1318_d_n12, eq10_e1318_d_n13, eq10_e1318_d_n14, eq10_e1318_d_n15, eq10_e1318_d_n16];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n1, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_d_n16,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        let eq11_e1327: f64 = (1.0 + s.v[57]);
        let eq11_e1329: f64 = (eq11_e1327 * s.v[378]);
        let eq11_e1329_d_n0: f64 = ((s.dn[57][0] * s.v[378]) + (eq11_e1327 * s.dn[378][0]));
        let eq11_e1329_d_n1: f64 = ((s.dn[57][1] * s.v[378]) + (eq11_e1327 * s.dn[378][1]));
        let eq11_e1329_d_n2: f64 = ((s.dn[57][2] * s.v[378]) + (eq11_e1327 * s.dn[378][2]));
        let eq11_e1329_d_n3: f64 = ((s.dn[57][3] * s.v[378]) + (eq11_e1327 * s.dn[378][3]));
        let eq11_e1329_d_n4: f64 = ((s.dn[57][4] * s.v[378]) + (eq11_e1327 * s.dn[378][4]));
        let eq11_e1329_d_n5: f64 = ((s.dn[57][5] * s.v[378]) + (eq11_e1327 * s.dn[378][5]));
        let eq11_e1329_d_n6: f64 = ((s.dn[57][6] * s.v[378]) + (eq11_e1327 * s.dn[378][6]));
        let eq11_e1329_d_n7: f64 = ((s.dn[57][7] * s.v[378]) + (eq11_e1327 * s.dn[378][7]));
        let eq11_e1329_d_n8: f64 = ((s.dn[57][8] * s.v[378]) + (eq11_e1327 * s.dn[378][8]));
        let eq11_e1329_d_n9: f64 = ((s.dn[57][9] * s.v[378]) + (eq11_e1327 * s.dn[378][9]));
        let eq11_e1329_d_n10: f64 = ((s.dn[57][10] * s.v[378]) + (eq11_e1327 * s.dn[378][10]));
        let eq11_e1329_d_n11: f64 = ((s.dn[57][11] * s.v[378]) + (eq11_e1327 * s.dn[378][11]));
        let eq11_e1329_d_n12: f64 = ((s.dn[57][12] * s.v[378]) + (eq11_e1327 * s.dn[378][12]));
        let eq11_e1329_d_n13: f64 = ((s.dn[57][13] * s.v[378]) + (eq11_e1327 * s.dn[378][13]));
        let eq11_e1329_d_n14: f64 = ((s.dn[57][14] * s.v[378]) + (eq11_e1327 * s.dn[378][14]));
        let eq11_e1329_d_n15: f64 = ((s.dn[57][15] * s.v[378]) + (eq11_e1327 * s.dn[378][15]));
        let eq11_e1329_d_n16: f64 = ((s.dn[57][16] * s.v[378]) + (eq11_e1327 * s.dn[378][16]));
        let eq11_e1331: f64 = (eq11_e1329 * s.v[46]);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * s.v[46]);
        let eq11_e1331_d_n1: f64 = (eq11_e1329_d_n1 * s.v[46]);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * s.v[46]);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * s.v[46]);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * s.v[46]);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * s.v[46]);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * s.v[46]);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * s.v[46]);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * s.v[46]);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * s.v[46]);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * s.v[46]);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * s.v[46]);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * s.v[46]);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * s.v[46]);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * s.v[46]);
        let eq11_e1331_d_n15: f64 = (eq11_e1329_d_n15 * s.v[46]);
        let eq11_e1331_d_n16: f64 = (eq11_e1329_d_n16 * s.v[46]);
        let eq11_e1333: f64 = (eq11_e1331 * s.v[29]);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * s.v[29]);
        let eq11_e1333_d_n1: f64 = (eq11_e1331_d_n1 * s.v[29]);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * s.v[29]);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * s.v[29]);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * s.v[29]);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * s.v[29]);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * s.v[29]);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * s.v[29]);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * s.v[29]);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * s.v[29]);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * s.v[29]);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * s.v[29]);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * s.v[29]);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * s.v[29]);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * s.v[29]);
        let eq11_e1333_d_n15: f64 = (eq11_e1331_d_n15 * s.v[29]);
        let eq11_e1333_d_n16: f64 = (eq11_e1331_d_n16 * s.v[29]);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n1: f64 = (eq11_e1333_d_n1 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1335_d_n15: f64 = (eq11_e1333_d_n15 * p.p2);
        let eq11_e1335_d_n16: f64 = (eq11_e1333_d_n16 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * s.v[30]);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * s.v[30]);
        let eq11_e1337_d_n1: f64 = (eq11_e1335_d_n1 * s.v[30]);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * s.v[30]);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * s.v[30]);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * s.v[30]);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * s.v[30]);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * s.v[30]);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * s.v[30]);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * s.v[30]);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * s.v[30]);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * s.v[30]);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * s.v[30]);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * s.v[30]);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * s.v[30]);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * s.v[30]);
        let eq11_e1337_d_n15: f64 = (eq11_e1335_d_n15 * s.v[30]);
        let eq11_e1337_d_n16: f64 = (eq11_e1335_d_n16 * s.v[30]);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n1: f64 = (eq11_e1337_d_n1 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1339_d_n15: f64 = ((eq11_e1337_d_n15 * (nv15 - 0.0)) + eq11_e1337);
        let eq11_e1339_d_n16: f64 = (eq11_e1337_d_n16 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n1: f64 = (0.5 * eq11_e1339_d_n1);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1339_d_n15);
        let eq11_e1340_d_n16: f64 = (0.5 * eq11_e1339_d_n16);
        let eq11_e1341: f64 = self.eval_ddt(1, eq11_e1340);
        let eq11_e1341_d_n0: f64 = self.ddt_jacobian(eq11_e1340_d_n0);
        let eq11_e1341_d_n1: f64 = self.ddt_jacobian(eq11_e1340_d_n1);
        let eq11_e1341_d_n2: f64 = self.ddt_jacobian(eq11_e1340_d_n2);
        let eq11_e1341_d_n3: f64 = self.ddt_jacobian(eq11_e1340_d_n3);
        let eq11_e1341_d_n4: f64 = self.ddt_jacobian(eq11_e1340_d_n4);
        let eq11_e1341_d_n5: f64 = self.ddt_jacobian(eq11_e1340_d_n5);
        let eq11_e1341_d_n6: f64 = self.ddt_jacobian(eq11_e1340_d_n6);
        let eq11_e1341_d_n7: f64 = self.ddt_jacobian(eq11_e1340_d_n7);
        let eq11_e1341_d_n8: f64 = self.ddt_jacobian(eq11_e1340_d_n8);
        let eq11_e1341_d_n9: f64 = self.ddt_jacobian(eq11_e1340_d_n9);
        let eq11_e1341_d_n10: f64 = self.ddt_jacobian(eq11_e1340_d_n10);
        let eq11_e1341_d_n11: f64 = self.ddt_jacobian(eq11_e1340_d_n11);
        let eq11_e1341_d_n12: f64 = self.ddt_jacobian(eq11_e1340_d_n12);
        let eq11_e1341_d_n13: f64 = self.ddt_jacobian(eq11_e1340_d_n13);
        let eq11_e1341_d_n14: f64 = self.ddt_jacobian(eq11_e1340_d_n14);
        let eq11_e1341_d_n15: f64 = self.ddt_jacobian(eq11_e1340_d_n15);
        let eq11_e1341_d_n16: f64 = self.ddt_jacobian(eq11_e1340_d_n16);
        let eq11_e1342: f64 = (p.p29 * eq11_e1341);
        let eq11_e1342_d_n0: f64 = (p.p29 * eq11_e1341_d_n0);
        let eq11_e1342_d_n1: f64 = (p.p29 * eq11_e1341_d_n1);
        let eq11_e1342_d_n2: f64 = (p.p29 * eq11_e1341_d_n2);
        let eq11_e1342_d_n3: f64 = (p.p29 * eq11_e1341_d_n3);
        let eq11_e1342_d_n4: f64 = (p.p29 * eq11_e1341_d_n4);
        let eq11_e1342_d_n5: f64 = (p.p29 * eq11_e1341_d_n5);
        let eq11_e1342_d_n6: f64 = (p.p29 * eq11_e1341_d_n6);
        let eq11_e1342_d_n7: f64 = (p.p29 * eq11_e1341_d_n7);
        let eq11_e1342_d_n8: f64 = (p.p29 * eq11_e1341_d_n8);
        let eq11_e1342_d_n9: f64 = (p.p29 * eq11_e1341_d_n9);
        let eq11_e1342_d_n10: f64 = (p.p29 * eq11_e1341_d_n10);
        let eq11_e1342_d_n11: f64 = (p.p29 * eq11_e1341_d_n11);
        let eq11_e1342_d_n12: f64 = (p.p29 * eq11_e1341_d_n12);
        let eq11_e1342_d_n13: f64 = (p.p29 * eq11_e1341_d_n13);
        let eq11_e1342_d_n14: f64 = (p.p29 * eq11_e1341_d_n14);
        let eq11_e1342_d_n15: f64 = (p.p29 * eq11_e1341_d_n15);
        let eq11_e1342_d_n16: f64 = (p.p29 * eq11_e1341_d_n16);
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n1, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15, eq11_e1342_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1344;
        let eq11_node_derivatives: [f64; 17] = [eq11_e1344_d_n0, eq11_e1344_d_n1, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_d_n16];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n1, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_d_n16,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        let eq12_e1353: f64 = (1.0 - s.v[57]);
        let eq12_e1353_d_n0: f64 = (-s.dn[57][0]);
        let eq12_e1353_d_n1: f64 = (-s.dn[57][1]);
        let eq12_e1353_d_n2: f64 = (-s.dn[57][2]);
        let eq12_e1353_d_n3: f64 = (-s.dn[57][3]);
        let eq12_e1353_d_n4: f64 = (-s.dn[57][4]);
        let eq12_e1353_d_n5: f64 = (-s.dn[57][5]);
        let eq12_e1353_d_n6: f64 = (-s.dn[57][6]);
        let eq12_e1353_d_n7: f64 = (-s.dn[57][7]);
        let eq12_e1353_d_n8: f64 = (-s.dn[57][8]);
        let eq12_e1353_d_n9: f64 = (-s.dn[57][9]);
        let eq12_e1353_d_n10: f64 = (-s.dn[57][10]);
        let eq12_e1353_d_n11: f64 = (-s.dn[57][11]);
        let eq12_e1353_d_n12: f64 = (-s.dn[57][12]);
        let eq12_e1353_d_n13: f64 = (-s.dn[57][13]);
        let eq12_e1353_d_n14: f64 = (-s.dn[57][14]);
        let eq12_e1353_d_n15: f64 = (-s.dn[57][15]);
        let eq12_e1353_d_n16: f64 = (-s.dn[57][16]);
        let eq12_e1355: f64 = (eq12_e1353 * s.v[378]);
        let eq12_e1355_d_n0: f64 = ((eq12_e1353_d_n0 * s.v[378]) + (eq12_e1353 * s.dn[378][0]));
        let eq12_e1355_d_n1: f64 = ((eq12_e1353_d_n1 * s.v[378]) + (eq12_e1353 * s.dn[378][1]));
        let eq12_e1355_d_n2: f64 = ((eq12_e1353_d_n2 * s.v[378]) + (eq12_e1353 * s.dn[378][2]));
        let eq12_e1355_d_n3: f64 = ((eq12_e1353_d_n3 * s.v[378]) + (eq12_e1353 * s.dn[378][3]));
        let eq12_e1355_d_n4: f64 = ((eq12_e1353_d_n4 * s.v[378]) + (eq12_e1353 * s.dn[378][4]));
        let eq12_e1355_d_n5: f64 = ((eq12_e1353_d_n5 * s.v[378]) + (eq12_e1353 * s.dn[378][5]));
        let eq12_e1355_d_n6: f64 = ((eq12_e1353_d_n6 * s.v[378]) + (eq12_e1353 * s.dn[378][6]));
        let eq12_e1355_d_n7: f64 = ((eq12_e1353_d_n7 * s.v[378]) + (eq12_e1353 * s.dn[378][7]));
        let eq12_e1355_d_n8: f64 = ((eq12_e1353_d_n8 * s.v[378]) + (eq12_e1353 * s.dn[378][8]));
        let eq12_e1355_d_n9: f64 = ((eq12_e1353_d_n9 * s.v[378]) + (eq12_e1353 * s.dn[378][9]));
        let eq12_e1355_d_n10: f64 = ((eq12_e1353_d_n10 * s.v[378]) + (eq12_e1353 * s.dn[378][10]));
        let eq12_e1355_d_n11: f64 = ((eq12_e1353_d_n11 * s.v[378]) + (eq12_e1353 * s.dn[378][11]));
        let eq12_e1355_d_n12: f64 = ((eq12_e1353_d_n12 * s.v[378]) + (eq12_e1353 * s.dn[378][12]));
        let eq12_e1355_d_n13: f64 = ((eq12_e1353_d_n13 * s.v[378]) + (eq12_e1353 * s.dn[378][13]));
        let eq12_e1355_d_n14: f64 = ((eq12_e1353_d_n14 * s.v[378]) + (eq12_e1353 * s.dn[378][14]));
        let eq12_e1355_d_n15: f64 = ((eq12_e1353_d_n15 * s.v[378]) + (eq12_e1353 * s.dn[378][15]));
        let eq12_e1355_d_n16: f64 = ((eq12_e1353_d_n16 * s.v[378]) + (eq12_e1353 * s.dn[378][16]));
        let eq12_e1357: f64 = (eq12_e1355 * s.v[46]);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * s.v[46]);
        let eq12_e1357_d_n1: f64 = (eq12_e1355_d_n1 * s.v[46]);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * s.v[46]);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * s.v[46]);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * s.v[46]);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * s.v[46]);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * s.v[46]);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * s.v[46]);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * s.v[46]);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * s.v[46]);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * s.v[46]);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * s.v[46]);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * s.v[46]);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * s.v[46]);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * s.v[46]);
        let eq12_e1357_d_n15: f64 = (eq12_e1355_d_n15 * s.v[46]);
        let eq12_e1357_d_n16: f64 = (eq12_e1355_d_n16 * s.v[46]);
        let eq12_e1359: f64 = (eq12_e1357 * s.v[29]);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * s.v[29]);
        let eq12_e1359_d_n1: f64 = (eq12_e1357_d_n1 * s.v[29]);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * s.v[29]);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * s.v[29]);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * s.v[29]);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * s.v[29]);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * s.v[29]);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * s.v[29]);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * s.v[29]);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * s.v[29]);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * s.v[29]);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * s.v[29]);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * s.v[29]);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * s.v[29]);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * s.v[29]);
        let eq12_e1359_d_n15: f64 = (eq12_e1357_d_n15 * s.v[29]);
        let eq12_e1359_d_n16: f64 = (eq12_e1357_d_n16 * s.v[29]);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n1: f64 = (eq12_e1359_d_n1 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1361_d_n15: f64 = (eq12_e1359_d_n15 * p.p2);
        let eq12_e1361_d_n16: f64 = (eq12_e1359_d_n16 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * s.v[30]);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * s.v[30]);
        let eq12_e1363_d_n1: f64 = (eq12_e1361_d_n1 * s.v[30]);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * s.v[30]);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * s.v[30]);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * s.v[30]);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * s.v[30]);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * s.v[30]);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * s.v[30]);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * s.v[30]);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * s.v[30]);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * s.v[30]);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * s.v[30]);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * s.v[30]);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * s.v[30]);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * s.v[30]);
        let eq12_e1363_d_n15: f64 = (eq12_e1361_d_n15 * s.v[30]);
        let eq12_e1363_d_n16: f64 = (eq12_e1361_d_n16 * s.v[30]);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n1: f64 = (eq12_e1363_d_n1 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1365_d_n15: f64 = ((eq12_e1363_d_n15 * (nv15 - 0.0)) + eq12_e1363);
        let eq12_e1365_d_n16: f64 = (eq12_e1363_d_n16 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n1: f64 = (0.5 * eq12_e1365_d_n1);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1365_d_n15);
        let eq12_e1366_d_n16: f64 = (0.5 * eq12_e1365_d_n16);
        let eq12_e1367: f64 = self.eval_ddt(2, eq12_e1366);
        let eq12_e1367_d_n0: f64 = self.ddt_jacobian(eq12_e1366_d_n0);
        let eq12_e1367_d_n1: f64 = self.ddt_jacobian(eq12_e1366_d_n1);
        let eq12_e1367_d_n2: f64 = self.ddt_jacobian(eq12_e1366_d_n2);
        let eq12_e1367_d_n3: f64 = self.ddt_jacobian(eq12_e1366_d_n3);
        let eq12_e1367_d_n4: f64 = self.ddt_jacobian(eq12_e1366_d_n4);
        let eq12_e1367_d_n5: f64 = self.ddt_jacobian(eq12_e1366_d_n5);
        let eq12_e1367_d_n6: f64 = self.ddt_jacobian(eq12_e1366_d_n6);
        let eq12_e1367_d_n7: f64 = self.ddt_jacobian(eq12_e1366_d_n7);
        let eq12_e1367_d_n8: f64 = self.ddt_jacobian(eq12_e1366_d_n8);
        let eq12_e1367_d_n9: f64 = self.ddt_jacobian(eq12_e1366_d_n9);
        let eq12_e1367_d_n10: f64 = self.ddt_jacobian(eq12_e1366_d_n10);
        let eq12_e1367_d_n11: f64 = self.ddt_jacobian(eq12_e1366_d_n11);
        let eq12_e1367_d_n12: f64 = self.ddt_jacobian(eq12_e1366_d_n12);
        let eq12_e1367_d_n13: f64 = self.ddt_jacobian(eq12_e1366_d_n13);
        let eq12_e1367_d_n14: f64 = self.ddt_jacobian(eq12_e1366_d_n14);
        let eq12_e1367_d_n15: f64 = self.ddt_jacobian(eq12_e1366_d_n15);
        let eq12_e1367_d_n16: f64 = self.ddt_jacobian(eq12_e1366_d_n16);
        let eq12_e1368: f64 = (p.p29 * eq12_e1367);
        let eq12_e1368_d_n0: f64 = (p.p29 * eq12_e1367_d_n0);
        let eq12_e1368_d_n1: f64 = (p.p29 * eq12_e1367_d_n1);
        let eq12_e1368_d_n2: f64 = (p.p29 * eq12_e1367_d_n2);
        let eq12_e1368_d_n3: f64 = (p.p29 * eq12_e1367_d_n3);
        let eq12_e1368_d_n4: f64 = (p.p29 * eq12_e1367_d_n4);
        let eq12_e1368_d_n5: f64 = (p.p29 * eq12_e1367_d_n5);
        let eq12_e1368_d_n6: f64 = (p.p29 * eq12_e1367_d_n6);
        let eq12_e1368_d_n7: f64 = (p.p29 * eq12_e1367_d_n7);
        let eq12_e1368_d_n8: f64 = (p.p29 * eq12_e1367_d_n8);
        let eq12_e1368_d_n9: f64 = (p.p29 * eq12_e1367_d_n9);
        let eq12_e1368_d_n10: f64 = (p.p29 * eq12_e1367_d_n10);
        let eq12_e1368_d_n11: f64 = (p.p29 * eq12_e1367_d_n11);
        let eq12_e1368_d_n12: f64 = (p.p29 * eq12_e1367_d_n12);
        let eq12_e1368_d_n13: f64 = (p.p29 * eq12_e1367_d_n13);
        let eq12_e1368_d_n14: f64 = (p.p29 * eq12_e1367_d_n14);
        let eq12_e1368_d_n15: f64 = (p.p29 * eq12_e1367_d_n15);
        let eq12_e1368_d_n16: f64 = (p.p29 * eq12_e1367_d_n16);
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n1, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15, eq12_e1368_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1370;
        let eq12_node_derivatives: [f64; 17] = [eq12_e1370_d_n0, eq12_e1370_d_n1, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_d_n16];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
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
        let nv16 = ctx.node_voltage(nodes[16]);
        let eq13_value: f64 = (nv16 - 0.0);
        stamper.stamp_current(
            Some(nodes[16]),
            None,
            self.multiplicity * (eq13_value),
            &[
                GeneratedDerivative::node(nodes[16], self.multiplicity * 1.0),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq14_value: f64 = (nv15 - 0.0);
        stamper.stamp_current(
            Some(nodes[15]),
            None,
            self.multiplicity * (eq14_value),
            &[
                GeneratedDerivative::node(nodes[15], self.multiplicity * 1.0),
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
        let (eq15_e1387,) = {
    if (s.v[1560] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e1387;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq15_value),
            &[
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
        let (eq16_e1402,) = {
    if (s.v[1560] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e1402;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[5]),
            self.multiplicity * (eq16_value),
            &[
            ],
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
        let (eq17_e1415,) = {
    if (s.v[1561] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1415;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[11]),
            self.multiplicity * (eq17_value),
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
        let (eq18_e1426,) = {
    if (s.v[1588] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e1426;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq18_value),
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
        let eq19_e1428: f64 = self.eval_ddt(3, s.v[787]);
        let eq19_e1428_d_n0: f64 = self.ddt_jacobian(s.dn[787][0]);
        let eq19_e1428_d_n1: f64 = self.ddt_jacobian(s.dn[787][1]);
        let eq19_e1428_d_n2: f64 = self.ddt_jacobian(s.dn[787][2]);
        let eq19_e1428_d_n3: f64 = self.ddt_jacobian(s.dn[787][3]);
        let eq19_e1428_d_n4: f64 = self.ddt_jacobian(s.dn[787][4]);
        let eq19_e1428_d_n5: f64 = self.ddt_jacobian(s.dn[787][5]);
        let eq19_e1428_d_n6: f64 = self.ddt_jacobian(s.dn[787][6]);
        let eq19_e1428_d_n7: f64 = self.ddt_jacobian(s.dn[787][7]);
        let eq19_e1428_d_n8: f64 = self.ddt_jacobian(s.dn[787][8]);
        let eq19_e1428_d_n9: f64 = self.ddt_jacobian(s.dn[787][9]);
        let eq19_e1428_d_n10: f64 = self.ddt_jacobian(s.dn[787][10]);
        let eq19_e1428_d_n11: f64 = self.ddt_jacobian(s.dn[787][11]);
        let eq19_e1428_d_n12: f64 = self.ddt_jacobian(s.dn[787][12]);
        let eq19_e1428_d_n13: f64 = self.ddt_jacobian(s.dn[787][13]);
        let eq19_e1428_d_n14: f64 = self.ddt_jacobian(s.dn[787][14]);
        let eq19_e1428_d_n15: f64 = self.ddt_jacobian(s.dn[787][15]);
        let eq19_e1428_d_n16: f64 = self.ddt_jacobian(s.dn[787][16]);
        let eq19_value: f64 = eq19_e1428;
        let eq19_node_derivatives: [f64; 17] = [eq19_e1428_d_n0, eq19_e1428_d_n1, eq19_e1428_d_n2, eq19_e1428_d_n3, eq19_e1428_d_n4, eq19_e1428_d_n5, eq19_e1428_d_n6, eq19_e1428_d_n7, eq19_e1428_d_n8, eq19_e1428_d_n9, eq19_e1428_d_n10, eq19_e1428_d_n11, eq19_e1428_d_n12, eq19_e1428_d_n13, eq19_e1428_d_n14, eq19_e1428_d_n15, eq19_e1428_d_n16];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[11]),
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
        let eq20_e1430: f64 = self.eval_ddt(4, s.v[785]);
        let eq20_e1430_d_n0: f64 = self.ddt_jacobian(s.dn[785][0]);
        let eq20_e1430_d_n1: f64 = self.ddt_jacobian(s.dn[785][1]);
        let eq20_e1430_d_n2: f64 = self.ddt_jacobian(s.dn[785][2]);
        let eq20_e1430_d_n3: f64 = self.ddt_jacobian(s.dn[785][3]);
        let eq20_e1430_d_n4: f64 = self.ddt_jacobian(s.dn[785][4]);
        let eq20_e1430_d_n5: f64 = self.ddt_jacobian(s.dn[785][5]);
        let eq20_e1430_d_n6: f64 = self.ddt_jacobian(s.dn[785][6]);
        let eq20_e1430_d_n7: f64 = self.ddt_jacobian(s.dn[785][7]);
        let eq20_e1430_d_n8: f64 = self.ddt_jacobian(s.dn[785][8]);
        let eq20_e1430_d_n9: f64 = self.ddt_jacobian(s.dn[785][9]);
        let eq20_e1430_d_n10: f64 = self.ddt_jacobian(s.dn[785][10]);
        let eq20_e1430_d_n11: f64 = self.ddt_jacobian(s.dn[785][11]);
        let eq20_e1430_d_n12: f64 = self.ddt_jacobian(s.dn[785][12]);
        let eq20_e1430_d_n13: f64 = self.ddt_jacobian(s.dn[785][13]);
        let eq20_e1430_d_n14: f64 = self.ddt_jacobian(s.dn[785][14]);
        let eq20_e1430_d_n15: f64 = self.ddt_jacobian(s.dn[785][15]);
        let eq20_e1430_d_n16: f64 = self.ddt_jacobian(s.dn[785][16]);
        let eq20_value: f64 = eq20_e1430;
        let eq20_node_derivatives: [f64; 17] = [eq20_e1430_d_n0, eq20_e1430_d_n1, eq20_e1430_d_n2, eq20_e1430_d_n3, eq20_e1430_d_n4, eq20_e1430_d_n5, eq20_e1430_d_n6, eq20_e1430_d_n7, eq20_e1430_d_n8, eq20_e1430_d_n9, eq20_e1430_d_n10, eq20_e1430_d_n11, eq20_e1430_d_n12, eq20_e1430_d_n13, eq20_e1430_d_n14, eq20_e1430_d_n15, eq20_e1430_d_n16];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
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
        let eq21_e1432: f64 = self.eval_ddt(5, s.v[786]);
        let eq21_e1432_d_n0: f64 = self.ddt_jacobian(s.dn[786][0]);
        let eq21_e1432_d_n1: f64 = self.ddt_jacobian(s.dn[786][1]);
        let eq21_e1432_d_n2: f64 = self.ddt_jacobian(s.dn[786][2]);
        let eq21_e1432_d_n3: f64 = self.ddt_jacobian(s.dn[786][3]);
        let eq21_e1432_d_n4: f64 = self.ddt_jacobian(s.dn[786][4]);
        let eq21_e1432_d_n5: f64 = self.ddt_jacobian(s.dn[786][5]);
        let eq21_e1432_d_n6: f64 = self.ddt_jacobian(s.dn[786][6]);
        let eq21_e1432_d_n7: f64 = self.ddt_jacobian(s.dn[786][7]);
        let eq21_e1432_d_n8: f64 = self.ddt_jacobian(s.dn[786][8]);
        let eq21_e1432_d_n9: f64 = self.ddt_jacobian(s.dn[786][9]);
        let eq21_e1432_d_n10: f64 = self.ddt_jacobian(s.dn[786][10]);
        let eq21_e1432_d_n11: f64 = self.ddt_jacobian(s.dn[786][11]);
        let eq21_e1432_d_n12: f64 = self.ddt_jacobian(s.dn[786][12]);
        let eq21_e1432_d_n13: f64 = self.ddt_jacobian(s.dn[786][13]);
        let eq21_e1432_d_n14: f64 = self.ddt_jacobian(s.dn[786][14]);
        let eq21_e1432_d_n15: f64 = self.ddt_jacobian(s.dn[786][15]);
        let eq21_e1432_d_n16: f64 = self.ddt_jacobian(s.dn[786][16]);
        let eq21_value: f64 = eq21_e1432;
        let eq21_node_derivatives: [f64; 17] = [eq21_e1432_d_n0, eq21_e1432_d_n1, eq21_e1432_d_n2, eq21_e1432_d_n3, eq21_e1432_d_n4, eq21_e1432_d_n5, eq21_e1432_d_n6, eq21_e1432_d_n7, eq21_e1432_d_n8, eq21_e1432_d_n9, eq21_e1432_d_n10, eq21_e1432_d_n11, eq21_e1432_d_n12, eq21_e1432_d_n13, eq21_e1432_d_n14, eq21_e1432_d_n15, eq21_e1432_d_n16];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            self.multiplicity * (eq21_value),
            &nodes,
            &eq21_node_derivatives,
            &branches,
            &eq21_branch_derivatives,
            self.multiplicity,
        );
    }
}
