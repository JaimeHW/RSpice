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
        let (eq31_e1244,) = {
    if (s.v[2721] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e1244;
        stamper.stamp_current(
            Some(nodes[12]),
            Some(nodes[10]),
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
        let (eq32_e1249,) = {
    if (!(s.v[2721] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1249;
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq33_e1259, eq33_e1259_d_n0, eq33_e1259_d_n1, eq33_e1259_d_n2, eq33_e1259_d_n3, eq33_e1259_d_n4, eq33_e1259_d_n5, eq33_e1259_d_n6, eq33_e1259_d_n7, eq33_e1259_d_n8, eq33_e1259_d_n9, eq33_e1259_d_n10, eq33_e1259_d_n11, eq33_e1259_d_n12,) = {
    if (s.v[2722] != 0.0) {
        let eq33_e1253: f64 = (s.v[15] * p.p32);
        let eq33_e1253_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq33_e1253_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq33_e1253_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq33_e1253_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq33_e1253_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq33_e1253_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq33_e1253_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq33_e1253_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq33_e1253_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq33_e1253_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq33_e1253_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq33_e1253_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq33_e1253_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq33_e1255: f64 = (eq33_e1253 * s.v[806]);
        let eq33_e1255_d_n0: f64 = ((eq33_e1253_d_n0 * s.v[806]) + (eq33_e1253 * s.dn[806][0]));
        let eq33_e1255_d_n1: f64 = ((eq33_e1253_d_n1 * s.v[806]) + (eq33_e1253 * s.dn[806][1]));
        let eq33_e1255_d_n2: f64 = ((eq33_e1253_d_n2 * s.v[806]) + (eq33_e1253 * s.dn[806][2]));
        let eq33_e1255_d_n3: f64 = ((eq33_e1253_d_n3 * s.v[806]) + (eq33_e1253 * s.dn[806][3]));
        let eq33_e1255_d_n4: f64 = ((eq33_e1253_d_n4 * s.v[806]) + (eq33_e1253 * s.dn[806][4]));
        let eq33_e1255_d_n5: f64 = ((eq33_e1253_d_n5 * s.v[806]) + (eq33_e1253 * s.dn[806][5]));
        let eq33_e1255_d_n6: f64 = ((eq33_e1253_d_n6 * s.v[806]) + (eq33_e1253 * s.dn[806][6]));
        let eq33_e1255_d_n7: f64 = ((eq33_e1253_d_n7 * s.v[806]) + (eq33_e1253 * s.dn[806][7]));
        let eq33_e1255_d_n8: f64 = ((eq33_e1253_d_n8 * s.v[806]) + (eq33_e1253 * s.dn[806][8]));
        let eq33_e1255_d_n9: f64 = ((eq33_e1253_d_n9 * s.v[806]) + (eq33_e1253 * s.dn[806][9]));
        let eq33_e1255_d_n10: f64 = ((eq33_e1253_d_n10 * s.v[806]) + (eq33_e1253 * s.dn[806][10]));
        let eq33_e1255_d_n11: f64 = ((eq33_e1253_d_n11 * s.v[806]) + (eq33_e1253 * s.dn[806][11]));
        let eq33_e1255_d_n12: f64 = ((eq33_e1253_d_n12 * s.v[806]) + (eq33_e1253 * s.dn[806][12]));
        let eq33_e1257: f64 = (eq33_e1255 * (nv3 - nv10));
        let eq33_e1257_d_n0: f64 = (eq33_e1255_d_n0 * (nv3 - nv10));
        let eq33_e1257_d_n1: f64 = (eq33_e1255_d_n1 * (nv3 - nv10));
        let eq33_e1257_d_n2: f64 = (eq33_e1255_d_n2 * (nv3 - nv10));
        let eq33_e1257_d_n3: f64 = ((eq33_e1255_d_n3 * (nv3 - nv10)) + eq33_e1255);
        let eq33_e1257_d_n4: f64 = (eq33_e1255_d_n4 * (nv3 - nv10));
        let eq33_e1257_d_n5: f64 = (eq33_e1255_d_n5 * (nv3 - nv10));
        let eq33_e1257_d_n6: f64 = (eq33_e1255_d_n6 * (nv3 - nv10));
        let eq33_e1257_d_n7: f64 = (eq33_e1255_d_n7 * (nv3 - nv10));
        let eq33_e1257_d_n8: f64 = (eq33_e1255_d_n8 * (nv3 - nv10));
        let eq33_e1257_d_n9: f64 = (eq33_e1255_d_n9 * (nv3 - nv10));
        let eq33_e1257_d_n10: f64 = ((eq33_e1255_d_n10 * (nv3 - nv10)) + (-eq33_e1255));
        let eq33_e1257_d_n11: f64 = (eq33_e1255_d_n11 * (nv3 - nv10));
        let eq33_e1257_d_n12: f64 = (eq33_e1255_d_n12 * (nv3 - nv10));
        (eq33_e1257, eq33_e1257_d_n0, eq33_e1257_d_n1, eq33_e1257_d_n2, eq33_e1257_d_n3, eq33_e1257_d_n4, eq33_e1257_d_n5, eq33_e1257_d_n6, eq33_e1257_d_n7, eq33_e1257_d_n8, eq33_e1257_d_n9, eq33_e1257_d_n10, eq33_e1257_d_n11, eq33_e1257_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1259;
        let eq33_node_derivatives: [f64; 13] = [eq33_e1259_d_n0, eq33_e1259_d_n1, eq33_e1259_d_n2, eq33_e1259_d_n3, eq33_e1259_d_n4, eq33_e1259_d_n5, eq33_e1259_d_n6, eq33_e1259_d_n7, eq33_e1259_d_n8, eq33_e1259_d_n9, eq33_e1259_d_n10, eq33_e1259_d_n11, eq33_e1259_d_n12];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[10]),
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
        let (eq34_e1269,) = {
    if (s.v[2722] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e1269;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[10]),
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
        let (eq35_e1274,) = {
    if (!(s.v[2722] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1274;
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq36_e1277: f64 = (p.p32 * s.v[867]);
        let eq36_e1279: f64 = (eq36_e1277 * (nv8 - nv9));
        let eq36_e1279_d_n9: f64 = (-eq36_e1277);
        let eq36_value: f64 = eq36_e1279;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[9]),
            self.multiplicity * (eq36_value),
            &[
                GeneratedDerivative::node(nodes[8], self.multiplicity * eq36_e1277),
                GeneratedDerivative::node(nodes[9], self.multiplicity * eq36_e1279_d_n9),
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq37_e1282: f64 = (p.p32 * s.v[867]);
        let eq37_e1284: f64 = (eq37_e1282 * (nv7 - nv9));
        let eq37_e1284_d_n9: f64 = (-eq37_e1282);
        let eq37_value: f64 = eq37_e1284;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq37_value),
            &[
                GeneratedDerivative::node(nodes[7], self.multiplicity * eq37_e1282),
                GeneratedDerivative::node(nodes[9], self.multiplicity * eq37_e1284_d_n9),
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
        let eq38_e1286: f64 = (-s.v[15]);
        let eq38_e1286_d_n0: f64 = (-s.dn[15][0]);
        let eq38_e1286_d_n1: f64 = (-s.dn[15][1]);
        let eq38_e1286_d_n2: f64 = (-s.dn[15][2]);
        let eq38_e1286_d_n3: f64 = (-s.dn[15][3]);
        let eq38_e1286_d_n4: f64 = (-s.dn[15][4]);
        let eq38_e1286_d_n5: f64 = (-s.dn[15][5]);
        let eq38_e1286_d_n6: f64 = (-s.dn[15][6]);
        let eq38_e1286_d_n7: f64 = (-s.dn[15][7]);
        let eq38_e1286_d_n8: f64 = (-s.dn[15][8]);
        let eq38_e1286_d_n9: f64 = (-s.dn[15][9]);
        let eq38_e1286_d_n10: f64 = (-s.dn[15][10]);
        let eq38_e1286_d_n11: f64 = (-s.dn[15][11]);
        let eq38_e1286_d_n12: f64 = (-s.dn[15][12]);
        let eq38_e1288: f64 = (eq38_e1286 * s.v[1915]);
        let eq38_e1288_d_n0: f64 = ((eq38_e1286_d_n0 * s.v[1915]) + (eq38_e1286 * s.dn[1915][0]));
        let eq38_e1288_d_n1: f64 = ((eq38_e1286_d_n1 * s.v[1915]) + (eq38_e1286 * s.dn[1915][1]));
        let eq38_e1288_d_n2: f64 = ((eq38_e1286_d_n2 * s.v[1915]) + (eq38_e1286 * s.dn[1915][2]));
        let eq38_e1288_d_n3: f64 = ((eq38_e1286_d_n3 * s.v[1915]) + (eq38_e1286 * s.dn[1915][3]));
        let eq38_e1288_d_n4: f64 = ((eq38_e1286_d_n4 * s.v[1915]) + (eq38_e1286 * s.dn[1915][4]));
        let eq38_e1288_d_n5: f64 = ((eq38_e1286_d_n5 * s.v[1915]) + (eq38_e1286 * s.dn[1915][5]));
        let eq38_e1288_d_n6: f64 = ((eq38_e1286_d_n6 * s.v[1915]) + (eq38_e1286 * s.dn[1915][6]));
        let eq38_e1288_d_n7: f64 = ((eq38_e1286_d_n7 * s.v[1915]) + (eq38_e1286 * s.dn[1915][7]));
        let eq38_e1288_d_n8: f64 = ((eq38_e1286_d_n8 * s.v[1915]) + (eq38_e1286 * s.dn[1915][8]));
        let eq38_e1288_d_n9: f64 = ((eq38_e1286_d_n9 * s.v[1915]) + (eq38_e1286 * s.dn[1915][9]));
        let eq38_e1288_d_n10: f64 = ((eq38_e1286_d_n10 * s.v[1915]) + (eq38_e1286 * s.dn[1915][10]));
        let eq38_e1288_d_n11: f64 = ((eq38_e1286_d_n11 * s.v[1915]) + (eq38_e1286 * s.dn[1915][11]));
        let eq38_e1288_d_n12: f64 = ((eq38_e1286_d_n12 * s.v[1915]) + (eq38_e1286 * s.dn[1915][12]));
        let eq38_value: f64 = eq38_e1288;
        let eq38_node_derivatives: [f64; 13] = [eq38_e1288_d_n0, eq38_e1288_d_n1, eq38_e1288_d_n2, eq38_e1288_d_n3, eq38_e1288_d_n4, eq38_e1288_d_n5, eq38_e1288_d_n6, eq38_e1288_d_n7, eq38_e1288_d_n8, eq38_e1288_d_n9, eq38_e1288_d_n10, eq38_e1288_d_n11, eq38_e1288_d_n12];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq39_e1291: f64 = (s.v[15] * s.v[306]);
        let eq39_e1291_d_n0: f64 = ((s.dn[15][0] * s.v[306]) + (s.v[15] * s.dn[306][0]));
        let eq39_e1291_d_n1: f64 = ((s.dn[15][1] * s.v[306]) + (s.v[15] * s.dn[306][1]));
        let eq39_e1291_d_n2: f64 = ((s.dn[15][2] * s.v[306]) + (s.v[15] * s.dn[306][2]));
        let eq39_e1291_d_n3: f64 = ((s.dn[15][3] * s.v[306]) + (s.v[15] * s.dn[306][3]));
        let eq39_e1291_d_n4: f64 = ((s.dn[15][4] * s.v[306]) + (s.v[15] * s.dn[306][4]));
        let eq39_e1291_d_n5: f64 = ((s.dn[15][5] * s.v[306]) + (s.v[15] * s.dn[306][5]));
        let eq39_e1291_d_n6: f64 = ((s.dn[15][6] * s.v[306]) + (s.v[15] * s.dn[306][6]));
        let eq39_e1291_d_n7: f64 = ((s.dn[15][7] * s.v[306]) + (s.v[15] * s.dn[306][7]));
        let eq39_e1291_d_n8: f64 = ((s.dn[15][8] * s.v[306]) + (s.v[15] * s.dn[306][8]));
        let eq39_e1291_d_n9: f64 = ((s.dn[15][9] * s.v[306]) + (s.v[15] * s.dn[306][9]));
        let eq39_e1291_d_n10: f64 = ((s.dn[15][10] * s.v[306]) + (s.v[15] * s.dn[306][10]));
        let eq39_e1291_d_n11: f64 = ((s.dn[15][11] * s.v[306]) + (s.v[15] * s.dn[306][11]));
        let eq39_e1291_d_n12: f64 = ((s.dn[15][12] * s.v[306]) + (s.v[15] * s.dn[306][12]));
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1293_d_n0: f64 = (eq39_e1291_d_n0 * (nv4 - 0.0));
        let eq39_e1293_d_n1: f64 = (eq39_e1291_d_n1 * (nv4 - 0.0));
        let eq39_e1293_d_n2: f64 = (eq39_e1291_d_n2 * (nv4 - 0.0));
        let eq39_e1293_d_n3: f64 = (eq39_e1291_d_n3 * (nv4 - 0.0));
        let eq39_e1293_d_n4: f64 = ((eq39_e1291_d_n4 * (nv4 - 0.0)) + eq39_e1291);
        let eq39_e1293_d_n5: f64 = (eq39_e1291_d_n5 * (nv4 - 0.0));
        let eq39_e1293_d_n6: f64 = (eq39_e1291_d_n6 * (nv4 - 0.0));
        let eq39_e1293_d_n7: f64 = (eq39_e1291_d_n7 * (nv4 - 0.0));
        let eq39_e1293_d_n8: f64 = (eq39_e1291_d_n8 * (nv4 - 0.0));
        let eq39_e1293_d_n9: f64 = (eq39_e1291_d_n9 * (nv4 - 0.0));
        let eq39_e1293_d_n10: f64 = (eq39_e1291_d_n10 * (nv4 - 0.0));
        let eq39_e1293_d_n11: f64 = (eq39_e1291_d_n11 * (nv4 - 0.0));
        let eq39_e1293_d_n12: f64 = (eq39_e1291_d_n12 * (nv4 - 0.0));
        let eq39_e1294: f64 = self.eval_ddt(0, eq39_e1293);
        let eq39_e1294_d_n0: f64 = self.ddt_jacobian(eq39_e1293_d_n0);
        let eq39_e1294_d_n1: f64 = self.ddt_jacobian(eq39_e1293_d_n1);
        let eq39_e1294_d_n2: f64 = self.ddt_jacobian(eq39_e1293_d_n2);
        let eq39_e1294_d_n3: f64 = self.ddt_jacobian(eq39_e1293_d_n3);
        let eq39_e1294_d_n4: f64 = self.ddt_jacobian(eq39_e1293_d_n4);
        let eq39_e1294_d_n5: f64 = self.ddt_jacobian(eq39_e1293_d_n5);
        let eq39_e1294_d_n6: f64 = self.ddt_jacobian(eq39_e1293_d_n6);
        let eq39_e1294_d_n7: f64 = self.ddt_jacobian(eq39_e1293_d_n7);
        let eq39_e1294_d_n8: f64 = self.ddt_jacobian(eq39_e1293_d_n8);
        let eq39_e1294_d_n9: f64 = self.ddt_jacobian(eq39_e1293_d_n9);
        let eq39_e1294_d_n10: f64 = self.ddt_jacobian(eq39_e1293_d_n10);
        let eq39_e1294_d_n11: f64 = self.ddt_jacobian(eq39_e1293_d_n11);
        let eq39_e1294_d_n12: f64 = self.ddt_jacobian(eq39_e1293_d_n12);
        let eq39_value: f64 = eq39_e1294;
        let eq39_node_derivatives: [f64; 13] = [eq39_e1294_d_n0, eq39_e1294_d_n1, eq39_e1294_d_n2, eq39_e1294_d_n3, eq39_e1294_d_n4, eq39_e1294_d_n5, eq39_e1294_d_n6, eq39_e1294_d_n7, eq39_e1294_d_n8, eq39_e1294_d_n9, eq39_e1294_d_n10, eq39_e1294_d_n11, eq39_e1294_d_n12];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq40_e1297: f64 = (s.v[15] * (nv4 - 0.0));
        let eq40_e1297_d_n0: f64 = (s.dn[15][0] * (nv4 - 0.0));
        let eq40_e1297_d_n1: f64 = (s.dn[15][1] * (nv4 - 0.0));
        let eq40_e1297_d_n2: f64 = (s.dn[15][2] * (nv4 - 0.0));
        let eq40_e1297_d_n3: f64 = (s.dn[15][3] * (nv4 - 0.0));
        let eq40_e1297_d_n4: f64 = ((s.dn[15][4] * (nv4 - 0.0)) + s.v[15]);
        let eq40_e1297_d_n5: f64 = (s.dn[15][5] * (nv4 - 0.0));
        let eq40_e1297_d_n6: f64 = (s.dn[15][6] * (nv4 - 0.0));
        let eq40_e1297_d_n7: f64 = (s.dn[15][7] * (nv4 - 0.0));
        let eq40_e1297_d_n8: f64 = (s.dn[15][8] * (nv4 - 0.0));
        let eq40_e1297_d_n9: f64 = (s.dn[15][9] * (nv4 - 0.0));
        let eq40_e1297_d_n10: f64 = (s.dn[15][10] * (nv4 - 0.0));
        let eq40_e1297_d_n11: f64 = (s.dn[15][11] * (nv4 - 0.0));
        let eq40_e1297_d_n12: f64 = (s.dn[15][12] * (nv4 - 0.0));
        let eq40_e1299: f64 = (eq40_e1297 / s.v[716]);
        let eq40_e1299_d_n0: f64 = (((eq40_e1297_d_n0 * s.v[716]) - (eq40_e1297 * s.dn[716][0])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n1: f64 = (((eq40_e1297_d_n1 * s.v[716]) - (eq40_e1297 * s.dn[716][1])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n2: f64 = (((eq40_e1297_d_n2 * s.v[716]) - (eq40_e1297 * s.dn[716][2])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n3: f64 = (((eq40_e1297_d_n3 * s.v[716]) - (eq40_e1297 * s.dn[716][3])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n4: f64 = (((eq40_e1297_d_n4 * s.v[716]) - (eq40_e1297 * s.dn[716][4])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n5: f64 = (((eq40_e1297_d_n5 * s.v[716]) - (eq40_e1297 * s.dn[716][5])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n6: f64 = (((eq40_e1297_d_n6 * s.v[716]) - (eq40_e1297 * s.dn[716][6])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n7: f64 = (((eq40_e1297_d_n7 * s.v[716]) - (eq40_e1297 * s.dn[716][7])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n8: f64 = (((eq40_e1297_d_n8 * s.v[716]) - (eq40_e1297 * s.dn[716][8])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n9: f64 = (((eq40_e1297_d_n9 * s.v[716]) - (eq40_e1297 * s.dn[716][9])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n10: f64 = (((eq40_e1297_d_n10 * s.v[716]) - (eq40_e1297 * s.dn[716][10])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n11: f64 = (((eq40_e1297_d_n11 * s.v[716]) - (eq40_e1297 * s.dn[716][11])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n12: f64 = (((eq40_e1297_d_n12 * s.v[716]) - (eq40_e1297 * s.dn[716][12])) / (s.v[716] * s.v[716]));
        let eq40_value: f64 = eq40_e1299;
        let eq40_node_derivatives: [f64; 13] = [eq40_e1299_d_n0, eq40_e1299_d_n1, eq40_e1299_d_n2, eq40_e1299_d_n3, eq40_e1299_d_n4, eq40_e1299_d_n5, eq40_e1299_d_n6, eq40_e1299_d_n7, eq40_e1299_d_n8, eq40_e1299_d_n9, eq40_e1299_d_n10, eq40_e1299_d_n11, eq40_e1299_d_n12];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let eq41_e1302: f64 = (s.v[0] * s.v[15]);
        let eq41_e1302_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq41_e1302_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq41_e1302_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq41_e1302_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq41_e1302_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq41_e1302_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq41_e1302_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq41_e1302_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq41_e1302_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq41_e1302_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq41_e1302_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq41_e1302_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq41_e1302_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1304_d_n0: f64 = (eq41_e1302_d_n0 * p.p33);
        let eq41_e1304_d_n1: f64 = (eq41_e1302_d_n1 * p.p33);
        let eq41_e1304_d_n2: f64 = (eq41_e1302_d_n2 * p.p33);
        let eq41_e1304_d_n3: f64 = (eq41_e1302_d_n3 * p.p33);
        let eq41_e1304_d_n4: f64 = (eq41_e1302_d_n4 * p.p33);
        let eq41_e1304_d_n5: f64 = (eq41_e1302_d_n5 * p.p33);
        let eq41_e1304_d_n6: f64 = (eq41_e1302_d_n6 * p.p33);
        let eq41_e1304_d_n7: f64 = (eq41_e1302_d_n7 * p.p33);
        let eq41_e1304_d_n8: f64 = (eq41_e1302_d_n8 * p.p33);
        let eq41_e1304_d_n9: f64 = (eq41_e1302_d_n9 * p.p33);
        let eq41_e1304_d_n10: f64 = (eq41_e1302_d_n10 * p.p33);
        let eq41_e1304_d_n11: f64 = (eq41_e1302_d_n11 * p.p33);
        let eq41_e1304_d_n12: f64 = (eq41_e1302_d_n12 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * s.v[840]);
        let eq41_e1306_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[840]) + (eq41_e1304 * s.dn[840][0]));
        let eq41_e1306_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[840]) + (eq41_e1304 * s.dn[840][1]));
        let eq41_e1306_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[840]) + (eq41_e1304 * s.dn[840][2]));
        let eq41_e1306_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[840]) + (eq41_e1304 * s.dn[840][3]));
        let eq41_e1306_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[840]) + (eq41_e1304 * s.dn[840][4]));
        let eq41_e1306_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[840]) + (eq41_e1304 * s.dn[840][5]));
        let eq41_e1306_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[840]) + (eq41_e1304 * s.dn[840][6]));
        let eq41_e1306_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[840]) + (eq41_e1304 * s.dn[840][7]));
        let eq41_e1306_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[840]) + (eq41_e1304 * s.dn[840][8]));
        let eq41_e1306_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[840]) + (eq41_e1304 * s.dn[840][9]));
        let eq41_e1306_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[840]) + (eq41_e1304 * s.dn[840][10]));
        let eq41_e1306_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[840]) + (eq41_e1304 * s.dn[840][11]));
        let eq41_e1306_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[840]) + (eq41_e1304 * s.dn[840][12]));
        let eq41_e1307: f64 = self.eval_ddt(1, eq41_e1306);
        let eq41_e1307_d_n0: f64 = self.ddt_jacobian(eq41_e1306_d_n0);
        let eq41_e1307_d_n1: f64 = self.ddt_jacobian(eq41_e1306_d_n1);
        let eq41_e1307_d_n2: f64 = self.ddt_jacobian(eq41_e1306_d_n2);
        let eq41_e1307_d_n3: f64 = self.ddt_jacobian(eq41_e1306_d_n3);
        let eq41_e1307_d_n4: f64 = self.ddt_jacobian(eq41_e1306_d_n4);
        let eq41_e1307_d_n5: f64 = self.ddt_jacobian(eq41_e1306_d_n5);
        let eq41_e1307_d_n6: f64 = self.ddt_jacobian(eq41_e1306_d_n6);
        let eq41_e1307_d_n7: f64 = self.ddt_jacobian(eq41_e1306_d_n7);
        let eq41_e1307_d_n8: f64 = self.ddt_jacobian(eq41_e1306_d_n8);
        let eq41_e1307_d_n9: f64 = self.ddt_jacobian(eq41_e1306_d_n9);
        let eq41_e1307_d_n10: f64 = self.ddt_jacobian(eq41_e1306_d_n10);
        let eq41_e1307_d_n11: f64 = self.ddt_jacobian(eq41_e1306_d_n11);
        let eq41_e1307_d_n12: f64 = self.ddt_jacobian(eq41_e1306_d_n12);
        let eq41_value: f64 = eq41_e1307;
        let eq41_node_derivatives: [f64; 13] = [eq41_e1307_d_n0, eq41_e1307_d_n1, eq41_e1307_d_n2, eq41_e1307_d_n3, eq41_e1307_d_n4, eq41_e1307_d_n5, eq41_e1307_d_n6, eq41_e1307_d_n7, eq41_e1307_d_n8, eq41_e1307_d_n9, eq41_e1307_d_n10, eq41_e1307_d_n11, eq41_e1307_d_n12];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let eq42_e1310: f64 = (s.v[0] * s.v[15]);
        let eq42_e1310_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq42_e1310_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq42_e1310_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq42_e1310_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq42_e1310_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq42_e1310_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq42_e1310_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq42_e1310_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq42_e1310_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq42_e1310_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq42_e1310_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq42_e1310_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq42_e1310_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1312_d_n0: f64 = (eq42_e1310_d_n0 * p.p33);
        let eq42_e1312_d_n1: f64 = (eq42_e1310_d_n1 * p.p33);
        let eq42_e1312_d_n2: f64 = (eq42_e1310_d_n2 * p.p33);
        let eq42_e1312_d_n3: f64 = (eq42_e1310_d_n3 * p.p33);
        let eq42_e1312_d_n4: f64 = (eq42_e1310_d_n4 * p.p33);
        let eq42_e1312_d_n5: f64 = (eq42_e1310_d_n5 * p.p33);
        let eq42_e1312_d_n6: f64 = (eq42_e1310_d_n6 * p.p33);
        let eq42_e1312_d_n7: f64 = (eq42_e1310_d_n7 * p.p33);
        let eq42_e1312_d_n8: f64 = (eq42_e1310_d_n8 * p.p33);
        let eq42_e1312_d_n9: f64 = (eq42_e1310_d_n9 * p.p33);
        let eq42_e1312_d_n10: f64 = (eq42_e1310_d_n10 * p.p33);
        let eq42_e1312_d_n11: f64 = (eq42_e1310_d_n11 * p.p33);
        let eq42_e1312_d_n12: f64 = (eq42_e1310_d_n12 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * s.v[841]);
        let eq42_e1314_d_n0: f64 = ((eq42_e1312_d_n0 * s.v[841]) + (eq42_e1312 * s.dn[841][0]));
        let eq42_e1314_d_n1: f64 = ((eq42_e1312_d_n1 * s.v[841]) + (eq42_e1312 * s.dn[841][1]));
        let eq42_e1314_d_n2: f64 = ((eq42_e1312_d_n2 * s.v[841]) + (eq42_e1312 * s.dn[841][2]));
        let eq42_e1314_d_n3: f64 = ((eq42_e1312_d_n3 * s.v[841]) + (eq42_e1312 * s.dn[841][3]));
        let eq42_e1314_d_n4: f64 = ((eq42_e1312_d_n4 * s.v[841]) + (eq42_e1312 * s.dn[841][4]));
        let eq42_e1314_d_n5: f64 = ((eq42_e1312_d_n5 * s.v[841]) + (eq42_e1312 * s.dn[841][5]));
        let eq42_e1314_d_n6: f64 = ((eq42_e1312_d_n6 * s.v[841]) + (eq42_e1312 * s.dn[841][6]));
        let eq42_e1314_d_n7: f64 = ((eq42_e1312_d_n7 * s.v[841]) + (eq42_e1312 * s.dn[841][7]));
        let eq42_e1314_d_n8: f64 = ((eq42_e1312_d_n8 * s.v[841]) + (eq42_e1312 * s.dn[841][8]));
        let eq42_e1314_d_n9: f64 = ((eq42_e1312_d_n9 * s.v[841]) + (eq42_e1312 * s.dn[841][9]));
        let eq42_e1314_d_n10: f64 = ((eq42_e1312_d_n10 * s.v[841]) + (eq42_e1312 * s.dn[841][10]));
        let eq42_e1314_d_n11: f64 = ((eq42_e1312_d_n11 * s.v[841]) + (eq42_e1312 * s.dn[841][11]));
        let eq42_e1314_d_n12: f64 = ((eq42_e1312_d_n12 * s.v[841]) + (eq42_e1312 * s.dn[841][12]));
        let eq42_e1315: f64 = self.eval_ddt(2, eq42_e1314);
        let eq42_e1315_d_n0: f64 = self.ddt_jacobian(eq42_e1314_d_n0);
        let eq42_e1315_d_n1: f64 = self.ddt_jacobian(eq42_e1314_d_n1);
        let eq42_e1315_d_n2: f64 = self.ddt_jacobian(eq42_e1314_d_n2);
        let eq42_e1315_d_n3: f64 = self.ddt_jacobian(eq42_e1314_d_n3);
        let eq42_e1315_d_n4: f64 = self.ddt_jacobian(eq42_e1314_d_n4);
        let eq42_e1315_d_n5: f64 = self.ddt_jacobian(eq42_e1314_d_n5);
        let eq42_e1315_d_n6: f64 = self.ddt_jacobian(eq42_e1314_d_n6);
        let eq42_e1315_d_n7: f64 = self.ddt_jacobian(eq42_e1314_d_n7);
        let eq42_e1315_d_n8: f64 = self.ddt_jacobian(eq42_e1314_d_n8);
        let eq42_e1315_d_n9: f64 = self.ddt_jacobian(eq42_e1314_d_n9);
        let eq42_e1315_d_n10: f64 = self.ddt_jacobian(eq42_e1314_d_n10);
        let eq42_e1315_d_n11: f64 = self.ddt_jacobian(eq42_e1314_d_n11);
        let eq42_e1315_d_n12: f64 = self.ddt_jacobian(eq42_e1314_d_n12);
        let eq42_value: f64 = eq42_e1315;
        let eq42_node_derivatives: [f64; 13] = [eq42_e1315_d_n0, eq42_e1315_d_n1, eq42_e1315_d_n2, eq42_e1315_d_n3, eq42_e1315_d_n4, eq42_e1315_d_n5, eq42_e1315_d_n6, eq42_e1315_d_n7, eq42_e1315_d_n8, eq42_e1315_d_n9, eq42_e1315_d_n10, eq42_e1315_d_n11, eq42_e1315_d_n12];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
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
        let eq43_e1318: f64 = (s.v[0] * s.v[15]);
        let eq43_e1318_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq43_e1318_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq43_e1318_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq43_e1318_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq43_e1318_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq43_e1318_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq43_e1318_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq43_e1318_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq43_e1318_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq43_e1318_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq43_e1318_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq43_e1318_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq43_e1318_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1320_d_n0: f64 = (eq43_e1318_d_n0 * p.p33);
        let eq43_e1320_d_n1: f64 = (eq43_e1318_d_n1 * p.p33);
        let eq43_e1320_d_n2: f64 = (eq43_e1318_d_n2 * p.p33);
        let eq43_e1320_d_n3: f64 = (eq43_e1318_d_n3 * p.p33);
        let eq43_e1320_d_n4: f64 = (eq43_e1318_d_n4 * p.p33);
        let eq43_e1320_d_n5: f64 = (eq43_e1318_d_n5 * p.p33);
        let eq43_e1320_d_n6: f64 = (eq43_e1318_d_n6 * p.p33);
        let eq43_e1320_d_n7: f64 = (eq43_e1318_d_n7 * p.p33);
        let eq43_e1320_d_n8: f64 = (eq43_e1318_d_n8 * p.p33);
        let eq43_e1320_d_n9: f64 = (eq43_e1318_d_n9 * p.p33);
        let eq43_e1320_d_n10: f64 = (eq43_e1318_d_n10 * p.p33);
        let eq43_e1320_d_n11: f64 = (eq43_e1318_d_n11 * p.p33);
        let eq43_e1320_d_n12: f64 = (eq43_e1318_d_n12 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * s.v[842]);
        let eq43_e1322_d_n0: f64 = ((eq43_e1320_d_n0 * s.v[842]) + (eq43_e1320 * s.dn[842][0]));
        let eq43_e1322_d_n1: f64 = ((eq43_e1320_d_n1 * s.v[842]) + (eq43_e1320 * s.dn[842][1]));
        let eq43_e1322_d_n2: f64 = ((eq43_e1320_d_n2 * s.v[842]) + (eq43_e1320 * s.dn[842][2]));
        let eq43_e1322_d_n3: f64 = ((eq43_e1320_d_n3 * s.v[842]) + (eq43_e1320 * s.dn[842][3]));
        let eq43_e1322_d_n4: f64 = ((eq43_e1320_d_n4 * s.v[842]) + (eq43_e1320 * s.dn[842][4]));
        let eq43_e1322_d_n5: f64 = ((eq43_e1320_d_n5 * s.v[842]) + (eq43_e1320 * s.dn[842][5]));
        let eq43_e1322_d_n6: f64 = ((eq43_e1320_d_n6 * s.v[842]) + (eq43_e1320 * s.dn[842][6]));
        let eq43_e1322_d_n7: f64 = ((eq43_e1320_d_n7 * s.v[842]) + (eq43_e1320 * s.dn[842][7]));
        let eq43_e1322_d_n8: f64 = ((eq43_e1320_d_n8 * s.v[842]) + (eq43_e1320 * s.dn[842][8]));
        let eq43_e1322_d_n9: f64 = ((eq43_e1320_d_n9 * s.v[842]) + (eq43_e1320 * s.dn[842][9]));
        let eq43_e1322_d_n10: f64 = ((eq43_e1320_d_n10 * s.v[842]) + (eq43_e1320 * s.dn[842][10]));
        let eq43_e1322_d_n11: f64 = ((eq43_e1320_d_n11 * s.v[842]) + (eq43_e1320 * s.dn[842][11]));
        let eq43_e1322_d_n12: f64 = ((eq43_e1320_d_n12 * s.v[842]) + (eq43_e1320 * s.dn[842][12]));
        let eq43_e1323: f64 = self.eval_ddt(3, eq43_e1322);
        let eq43_e1323_d_n0: f64 = self.ddt_jacobian(eq43_e1322_d_n0);
        let eq43_e1323_d_n1: f64 = self.ddt_jacobian(eq43_e1322_d_n1);
        let eq43_e1323_d_n2: f64 = self.ddt_jacobian(eq43_e1322_d_n2);
        let eq43_e1323_d_n3: f64 = self.ddt_jacobian(eq43_e1322_d_n3);
        let eq43_e1323_d_n4: f64 = self.ddt_jacobian(eq43_e1322_d_n4);
        let eq43_e1323_d_n5: f64 = self.ddt_jacobian(eq43_e1322_d_n5);
        let eq43_e1323_d_n6: f64 = self.ddt_jacobian(eq43_e1322_d_n6);
        let eq43_e1323_d_n7: f64 = self.ddt_jacobian(eq43_e1322_d_n7);
        let eq43_e1323_d_n8: f64 = self.ddt_jacobian(eq43_e1322_d_n8);
        let eq43_e1323_d_n9: f64 = self.ddt_jacobian(eq43_e1322_d_n9);
        let eq43_e1323_d_n10: f64 = self.ddt_jacobian(eq43_e1322_d_n10);
        let eq43_e1323_d_n11: f64 = self.ddt_jacobian(eq43_e1322_d_n11);
        let eq43_e1323_d_n12: f64 = self.ddt_jacobian(eq43_e1322_d_n12);
        let eq43_value: f64 = eq43_e1323;
        let eq43_node_derivatives: [f64; 13] = [eq43_e1323_d_n0, eq43_e1323_d_n1, eq43_e1323_d_n2, eq43_e1323_d_n3, eq43_e1323_d_n4, eq43_e1323_d_n5, eq43_e1323_d_n6, eq43_e1323_d_n7, eq43_e1323_d_n8, eq43_e1323_d_n9, eq43_e1323_d_n10, eq43_e1323_d_n11, eq43_e1323_d_n12];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
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
        let eq44_e1326: f64 = (s.v[0] * s.v[15]);
        let eq44_e1326_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq44_e1326_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq44_e1326_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq44_e1326_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq44_e1326_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq44_e1326_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq44_e1326_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq44_e1326_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq44_e1326_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq44_e1326_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq44_e1326_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq44_e1326_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq44_e1326_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq44_e1328: f64 = (eq44_e1326 * p.p33);
        let eq44_e1328_d_n0: f64 = (eq44_e1326_d_n0 * p.p33);
        let eq44_e1328_d_n1: f64 = (eq44_e1326_d_n1 * p.p33);
        let eq44_e1328_d_n2: f64 = (eq44_e1326_d_n2 * p.p33);
        let eq44_e1328_d_n3: f64 = (eq44_e1326_d_n3 * p.p33);
        let eq44_e1328_d_n4: f64 = (eq44_e1326_d_n4 * p.p33);
        let eq44_e1328_d_n5: f64 = (eq44_e1326_d_n5 * p.p33);
        let eq44_e1328_d_n6: f64 = (eq44_e1326_d_n6 * p.p33);
        let eq44_e1328_d_n7: f64 = (eq44_e1326_d_n7 * p.p33);
        let eq44_e1328_d_n8: f64 = (eq44_e1326_d_n8 * p.p33);
        let eq44_e1328_d_n9: f64 = (eq44_e1326_d_n9 * p.p33);
        let eq44_e1328_d_n10: f64 = (eq44_e1326_d_n10 * p.p33);
        let eq44_e1328_d_n11: f64 = (eq44_e1326_d_n11 * p.p33);
        let eq44_e1328_d_n12: f64 = (eq44_e1326_d_n12 * p.p33);
        let eq44_e1330: f64 = (eq44_e1328 * s.v[843]);
        let eq44_e1330_d_n0: f64 = ((eq44_e1328_d_n0 * s.v[843]) + (eq44_e1328 * s.dn[843][0]));
        let eq44_e1330_d_n1: f64 = ((eq44_e1328_d_n1 * s.v[843]) + (eq44_e1328 * s.dn[843][1]));
        let eq44_e1330_d_n2: f64 = ((eq44_e1328_d_n2 * s.v[843]) + (eq44_e1328 * s.dn[843][2]));
        let eq44_e1330_d_n3: f64 = ((eq44_e1328_d_n3 * s.v[843]) + (eq44_e1328 * s.dn[843][3]));
        let eq44_e1330_d_n4: f64 = ((eq44_e1328_d_n4 * s.v[843]) + (eq44_e1328 * s.dn[843][4]));
        let eq44_e1330_d_n5: f64 = ((eq44_e1328_d_n5 * s.v[843]) + (eq44_e1328 * s.dn[843][5]));
        let eq44_e1330_d_n6: f64 = ((eq44_e1328_d_n6 * s.v[843]) + (eq44_e1328 * s.dn[843][6]));
        let eq44_e1330_d_n7: f64 = ((eq44_e1328_d_n7 * s.v[843]) + (eq44_e1328 * s.dn[843][7]));
        let eq44_e1330_d_n8: f64 = ((eq44_e1328_d_n8 * s.v[843]) + (eq44_e1328 * s.dn[843][8]));
        let eq44_e1330_d_n9: f64 = ((eq44_e1328_d_n9 * s.v[843]) + (eq44_e1328 * s.dn[843][9]));
        let eq44_e1330_d_n10: f64 = ((eq44_e1328_d_n10 * s.v[843]) + (eq44_e1328 * s.dn[843][10]));
        let eq44_e1330_d_n11: f64 = ((eq44_e1328_d_n11 * s.v[843]) + (eq44_e1328 * s.dn[843][11]));
        let eq44_e1330_d_n12: f64 = ((eq44_e1328_d_n12 * s.v[843]) + (eq44_e1328 * s.dn[843][12]));
        let eq44_e1331: f64 = self.eval_ddt(4, eq44_e1330);
        let eq44_e1331_d_n0: f64 = self.ddt_jacobian(eq44_e1330_d_n0);
        let eq44_e1331_d_n1: f64 = self.ddt_jacobian(eq44_e1330_d_n1);
        let eq44_e1331_d_n2: f64 = self.ddt_jacobian(eq44_e1330_d_n2);
        let eq44_e1331_d_n3: f64 = self.ddt_jacobian(eq44_e1330_d_n3);
        let eq44_e1331_d_n4: f64 = self.ddt_jacobian(eq44_e1330_d_n4);
        let eq44_e1331_d_n5: f64 = self.ddt_jacobian(eq44_e1330_d_n5);
        let eq44_e1331_d_n6: f64 = self.ddt_jacobian(eq44_e1330_d_n6);
        let eq44_e1331_d_n7: f64 = self.ddt_jacobian(eq44_e1330_d_n7);
        let eq44_e1331_d_n8: f64 = self.ddt_jacobian(eq44_e1330_d_n8);
        let eq44_e1331_d_n9: f64 = self.ddt_jacobian(eq44_e1330_d_n9);
        let eq44_e1331_d_n10: f64 = self.ddt_jacobian(eq44_e1330_d_n10);
        let eq44_e1331_d_n11: f64 = self.ddt_jacobian(eq44_e1330_d_n11);
        let eq44_e1331_d_n12: f64 = self.ddt_jacobian(eq44_e1330_d_n12);
        let eq44_value: f64 = eq44_e1331;
        let eq44_node_derivatives: [f64; 13] = [eq44_e1331_d_n0, eq44_e1331_d_n1, eq44_e1331_d_n2, eq44_e1331_d_n3, eq44_e1331_d_n4, eq44_e1331_d_n5, eq44_e1331_d_n6, eq44_e1331_d_n7, eq44_e1331_d_n8, eq44_e1331_d_n9, eq44_e1331_d_n10, eq44_e1331_d_n11, eq44_e1331_d_n12];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let eq45_e1334: f64 = (s.v[0] * s.v[15]);
        let eq45_e1334_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq45_e1334_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq45_e1334_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq45_e1334_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq45_e1334_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq45_e1334_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq45_e1334_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq45_e1334_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq45_e1334_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq45_e1334_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq45_e1334_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq45_e1334_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq45_e1334_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq45_e1336: f64 = (eq45_e1334 * p.p33);
        let eq45_e1336_d_n0: f64 = (eq45_e1334_d_n0 * p.p33);
        let eq45_e1336_d_n1: f64 = (eq45_e1334_d_n1 * p.p33);
        let eq45_e1336_d_n2: f64 = (eq45_e1334_d_n2 * p.p33);
        let eq45_e1336_d_n3: f64 = (eq45_e1334_d_n3 * p.p33);
        let eq45_e1336_d_n4: f64 = (eq45_e1334_d_n4 * p.p33);
        let eq45_e1336_d_n5: f64 = (eq45_e1334_d_n5 * p.p33);
        let eq45_e1336_d_n6: f64 = (eq45_e1334_d_n6 * p.p33);
        let eq45_e1336_d_n7: f64 = (eq45_e1334_d_n7 * p.p33);
        let eq45_e1336_d_n8: f64 = (eq45_e1334_d_n8 * p.p33);
        let eq45_e1336_d_n9: f64 = (eq45_e1334_d_n9 * p.p33);
        let eq45_e1336_d_n10: f64 = (eq45_e1334_d_n10 * p.p33);
        let eq45_e1336_d_n11: f64 = (eq45_e1334_d_n11 * p.p33);
        let eq45_e1336_d_n12: f64 = (eq45_e1334_d_n12 * p.p33);
        let eq45_e1338: f64 = (eq45_e1336 * s.v[844]);
        let eq45_e1338_d_n0: f64 = ((eq45_e1336_d_n0 * s.v[844]) + (eq45_e1336 * s.dn[844][0]));
        let eq45_e1338_d_n1: f64 = ((eq45_e1336_d_n1 * s.v[844]) + (eq45_e1336 * s.dn[844][1]));
        let eq45_e1338_d_n2: f64 = ((eq45_e1336_d_n2 * s.v[844]) + (eq45_e1336 * s.dn[844][2]));
        let eq45_e1338_d_n3: f64 = ((eq45_e1336_d_n3 * s.v[844]) + (eq45_e1336 * s.dn[844][3]));
        let eq45_e1338_d_n4: f64 = ((eq45_e1336_d_n4 * s.v[844]) + (eq45_e1336 * s.dn[844][4]));
        let eq45_e1338_d_n5: f64 = ((eq45_e1336_d_n5 * s.v[844]) + (eq45_e1336 * s.dn[844][5]));
        let eq45_e1338_d_n6: f64 = ((eq45_e1336_d_n6 * s.v[844]) + (eq45_e1336 * s.dn[844][6]));
        let eq45_e1338_d_n7: f64 = ((eq45_e1336_d_n7 * s.v[844]) + (eq45_e1336 * s.dn[844][7]));
        let eq45_e1338_d_n8: f64 = ((eq45_e1336_d_n8 * s.v[844]) + (eq45_e1336 * s.dn[844][8]));
        let eq45_e1338_d_n9: f64 = ((eq45_e1336_d_n9 * s.v[844]) + (eq45_e1336 * s.dn[844][9]));
        let eq45_e1338_d_n10: f64 = ((eq45_e1336_d_n10 * s.v[844]) + (eq45_e1336 * s.dn[844][10]));
        let eq45_e1338_d_n11: f64 = ((eq45_e1336_d_n11 * s.v[844]) + (eq45_e1336 * s.dn[844][11]));
        let eq45_e1338_d_n12: f64 = ((eq45_e1336_d_n12 * s.v[844]) + (eq45_e1336 * s.dn[844][12]));
        let eq45_e1339: f64 = self.eval_ddt(5, eq45_e1338);
        let eq45_e1339_d_n0: f64 = self.ddt_jacobian(eq45_e1338_d_n0);
        let eq45_e1339_d_n1: f64 = self.ddt_jacobian(eq45_e1338_d_n1);
        let eq45_e1339_d_n2: f64 = self.ddt_jacobian(eq45_e1338_d_n2);
        let eq45_e1339_d_n3: f64 = self.ddt_jacobian(eq45_e1338_d_n3);
        let eq45_e1339_d_n4: f64 = self.ddt_jacobian(eq45_e1338_d_n4);
        let eq45_e1339_d_n5: f64 = self.ddt_jacobian(eq45_e1338_d_n5);
        let eq45_e1339_d_n6: f64 = self.ddt_jacobian(eq45_e1338_d_n6);
        let eq45_e1339_d_n7: f64 = self.ddt_jacobian(eq45_e1338_d_n7);
        let eq45_e1339_d_n8: f64 = self.ddt_jacobian(eq45_e1338_d_n8);
        let eq45_e1339_d_n9: f64 = self.ddt_jacobian(eq45_e1338_d_n9);
        let eq45_e1339_d_n10: f64 = self.ddt_jacobian(eq45_e1338_d_n10);
        let eq45_e1339_d_n11: f64 = self.ddt_jacobian(eq45_e1338_d_n11);
        let eq45_e1339_d_n12: f64 = self.ddt_jacobian(eq45_e1338_d_n12);
        let eq45_value: f64 = eq45_e1339;
        let eq45_node_derivatives: [f64; 13] = [eq45_e1339_d_n0, eq45_e1339_d_n1, eq45_e1339_d_n2, eq45_e1339_d_n3, eq45_e1339_d_n4, eq45_e1339_d_n5, eq45_e1339_d_n6, eq45_e1339_d_n7, eq45_e1339_d_n8, eq45_e1339_d_n9, eq45_e1339_d_n10, eq45_e1339_d_n11, eq45_e1339_d_n12];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq45_value),
            &nodes,
            &eq45_node_derivatives,
            &branches,
            &eq45_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq46_e1342: f64 = (s.v[0] * s.v[15]);
        let eq46_e1342_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq46_e1342_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq46_e1342_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq46_e1342_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq46_e1342_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq46_e1342_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq46_e1342_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq46_e1342_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq46_e1342_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq46_e1342_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq46_e1342_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq46_e1342_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq46_e1342_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1344_d_n0: f64 = (eq46_e1342_d_n0 * p.p33);
        let eq46_e1344_d_n1: f64 = (eq46_e1342_d_n1 * p.p33);
        let eq46_e1344_d_n2: f64 = (eq46_e1342_d_n2 * p.p33);
        let eq46_e1344_d_n3: f64 = (eq46_e1342_d_n3 * p.p33);
        let eq46_e1344_d_n4: f64 = (eq46_e1342_d_n4 * p.p33);
        let eq46_e1344_d_n5: f64 = (eq46_e1342_d_n5 * p.p33);
        let eq46_e1344_d_n6: f64 = (eq46_e1342_d_n6 * p.p33);
        let eq46_e1344_d_n7: f64 = (eq46_e1342_d_n7 * p.p33);
        let eq46_e1344_d_n8: f64 = (eq46_e1342_d_n8 * p.p33);
        let eq46_e1344_d_n9: f64 = (eq46_e1342_d_n9 * p.p33);
        let eq46_e1344_d_n10: f64 = (eq46_e1342_d_n10 * p.p33);
        let eq46_e1344_d_n11: f64 = (eq46_e1342_d_n11 * p.p33);
        let eq46_e1344_d_n12: f64 = (eq46_e1342_d_n12 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * s.v[845]);
        let eq46_e1346_d_n0: f64 = ((eq46_e1344_d_n0 * s.v[845]) + (eq46_e1344 * s.dn[845][0]));
        let eq46_e1346_d_n1: f64 = ((eq46_e1344_d_n1 * s.v[845]) + (eq46_e1344 * s.dn[845][1]));
        let eq46_e1346_d_n2: f64 = ((eq46_e1344_d_n2 * s.v[845]) + (eq46_e1344 * s.dn[845][2]));
        let eq46_e1346_d_n3: f64 = ((eq46_e1344_d_n3 * s.v[845]) + (eq46_e1344 * s.dn[845][3]));
        let eq46_e1346_d_n4: f64 = ((eq46_e1344_d_n4 * s.v[845]) + (eq46_e1344 * s.dn[845][4]));
        let eq46_e1346_d_n5: f64 = ((eq46_e1344_d_n5 * s.v[845]) + (eq46_e1344 * s.dn[845][5]));
        let eq46_e1346_d_n6: f64 = ((eq46_e1344_d_n6 * s.v[845]) + (eq46_e1344 * s.dn[845][6]));
        let eq46_e1346_d_n7: f64 = ((eq46_e1344_d_n7 * s.v[845]) + (eq46_e1344 * s.dn[845][7]));
        let eq46_e1346_d_n8: f64 = ((eq46_e1344_d_n8 * s.v[845]) + (eq46_e1344 * s.dn[845][8]));
        let eq46_e1346_d_n9: f64 = ((eq46_e1344_d_n9 * s.v[845]) + (eq46_e1344 * s.dn[845][9]));
        let eq46_e1346_d_n10: f64 = ((eq46_e1344_d_n10 * s.v[845]) + (eq46_e1344 * s.dn[845][10]));
        let eq46_e1346_d_n11: f64 = ((eq46_e1344_d_n11 * s.v[845]) + (eq46_e1344 * s.dn[845][11]));
        let eq46_e1346_d_n12: f64 = ((eq46_e1344_d_n12 * s.v[845]) + (eq46_e1344 * s.dn[845][12]));
        let eq46_e1347: f64 = self.eval_ddt(6, eq46_e1346);
        let eq46_e1347_d_n0: f64 = self.ddt_jacobian(eq46_e1346_d_n0);
        let eq46_e1347_d_n1: f64 = self.ddt_jacobian(eq46_e1346_d_n1);
        let eq46_e1347_d_n2: f64 = self.ddt_jacobian(eq46_e1346_d_n2);
        let eq46_e1347_d_n3: f64 = self.ddt_jacobian(eq46_e1346_d_n3);
        let eq46_e1347_d_n4: f64 = self.ddt_jacobian(eq46_e1346_d_n4);
        let eq46_e1347_d_n5: f64 = self.ddt_jacobian(eq46_e1346_d_n5);
        let eq46_e1347_d_n6: f64 = self.ddt_jacobian(eq46_e1346_d_n6);
        let eq46_e1347_d_n7: f64 = self.ddt_jacobian(eq46_e1346_d_n7);
        let eq46_e1347_d_n8: f64 = self.ddt_jacobian(eq46_e1346_d_n8);
        let eq46_e1347_d_n9: f64 = self.ddt_jacobian(eq46_e1346_d_n9);
        let eq46_e1347_d_n10: f64 = self.ddt_jacobian(eq46_e1346_d_n10);
        let eq46_e1347_d_n11: f64 = self.ddt_jacobian(eq46_e1346_d_n11);
        let eq46_e1347_d_n12: f64 = self.ddt_jacobian(eq46_e1346_d_n12);
        let eq46_value: f64 = eq46_e1347;
        let eq46_node_derivatives: [f64; 13] = [eq46_e1347_d_n0, eq46_e1347_d_n1, eq46_e1347_d_n2, eq46_e1347_d_n3, eq46_e1347_d_n4, eq46_e1347_d_n5, eq46_e1347_d_n6, eq46_e1347_d_n7, eq46_e1347_d_n8, eq46_e1347_d_n9, eq46_e1347_d_n10, eq46_e1347_d_n11, eq46_e1347_d_n12];
        let eq46_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[9]),
            self.multiplicity * (eq46_value),
            &nodes,
            &eq46_node_derivatives,
            &branches,
            &eq46_branch_derivatives,
            self.multiplicity,
        );
    }
}
