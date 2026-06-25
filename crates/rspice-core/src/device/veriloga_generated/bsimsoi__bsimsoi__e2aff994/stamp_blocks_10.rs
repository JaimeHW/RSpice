#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_74_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq74_e2172,) = {
    if (s.v[2028] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e2172;
        stamper.stamp_potential(
            branches[10],
            eq74_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_75_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq75_e2179,) = {
    if ((!(s.v[2028] != 0.0)) && (s.v[2029] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e2179;
        stamper.stamp_potential(
            branches[11],
            eq75_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_76_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq76_e2187, eq76_e2187_d_n0, eq76_e2187_d_n1, eq76_e2187_d_n2, eq76_e2187_d_n3, eq76_e2187_d_n4, eq76_e2187_d_n5, eq76_e2187_d_n6, eq76_e2187_d_n7, eq76_e2187_d_n8, eq76_e2187_d_n9, eq76_e2187_d_n10, eq76_e2187_d_n11, eq76_e2187_d_n12, eq76_e2187_d_n13,) = {
    if ((s.v[2037] != 0.0) && (s.v[2038] != 0.0)) {
        let eq76_e2185: f64 = ((nv4 - nv10) * s.v[1021]);
        let eq76_e2185_d_n0: f64 = ((nv4 - nv10) * s.dn[1021][0]);
        let eq76_e2185_d_n1: f64 = ((nv4 - nv10) * s.dn[1021][1]);
        let eq76_e2185_d_n2: f64 = ((nv4 - nv10) * s.dn[1021][2]);
        let eq76_e2185_d_n3: f64 = ((nv4 - nv10) * s.dn[1021][3]);
        let eq76_e2185_d_n4: f64 = (s.v[1021] + ((nv4 - nv10) * s.dn[1021][4]));
        let eq76_e2185_d_n5: f64 = ((nv4 - nv10) * s.dn[1021][5]);
        let eq76_e2185_d_n6: f64 = ((nv4 - nv10) * s.dn[1021][6]);
        let eq76_e2185_d_n7: f64 = ((nv4 - nv10) * s.dn[1021][7]);
        let eq76_e2185_d_n8: f64 = ((nv4 - nv10) * s.dn[1021][8]);
        let eq76_e2185_d_n9: f64 = ((nv4 - nv10) * s.dn[1021][9]);
        let eq76_e2185_d_n10: f64 = ((-s.v[1021]) + ((nv4 - nv10) * s.dn[1021][10]));
        let eq76_e2185_d_n11: f64 = ((nv4 - nv10) * s.dn[1021][11]);
        let eq76_e2185_d_n12: f64 = ((nv4 - nv10) * s.dn[1021][12]);
        let eq76_e2185_d_n13: f64 = ((nv4 - nv10) * s.dn[1021][13]);
        (eq76_e2185, eq76_e2185_d_n0, eq76_e2185_d_n1, eq76_e2185_d_n2, eq76_e2185_d_n3, eq76_e2185_d_n4, eq76_e2185_d_n5, eq76_e2185_d_n6, eq76_e2185_d_n7, eq76_e2185_d_n8, eq76_e2185_d_n9, eq76_e2185_d_n10, eq76_e2185_d_n11, eq76_e2185_d_n12, eq76_e2185_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e2187;
        let eq76_node_derivatives: [f64; 14] = [eq76_e2187_d_n0, eq76_e2187_d_n1, eq76_e2187_d_n2, eq76_e2187_d_n3, eq76_e2187_d_n4, eq76_e2187_d_n5, eq76_e2187_d_n6, eq76_e2187_d_n7, eq76_e2187_d_n8, eq76_e2187_d_n9, eq76_e2187_d_n10, eq76_e2187_d_n11, eq76_e2187_d_n12, eq76_e2187_d_n13];
        let eq76_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[10]),
            self.multiplicity * (eq76_value),
            &nodes,
            &eq76_node_derivatives,
            &branches,
            &eq76_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_77_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq77_e2195, eq77_e2195_d_n0, eq77_e2195_d_n1, eq77_e2195_d_n2, eq77_e2195_d_n3, eq77_e2195_d_n4, eq77_e2195_d_n5, eq77_e2195_d_n6, eq77_e2195_d_n7, eq77_e2195_d_n8, eq77_e2195_d_n9, eq77_e2195_d_n10, eq77_e2195_d_n11, eq77_e2195_d_n12, eq77_e2195_d_n13,) = {
    if ((s.v[2037] != 0.0) && (s.v[2038] != 0.0)) {
        let eq77_e2193: f64 = ((nv4 - nv11) * s.v[1022]);
        let eq77_e2193_d_n0: f64 = ((nv4 - nv11) * s.dn[1022][0]);
        let eq77_e2193_d_n1: f64 = ((nv4 - nv11) * s.dn[1022][1]);
        let eq77_e2193_d_n2: f64 = ((nv4 - nv11) * s.dn[1022][2]);
        let eq77_e2193_d_n3: f64 = ((nv4 - nv11) * s.dn[1022][3]);
        let eq77_e2193_d_n4: f64 = (s.v[1022] + ((nv4 - nv11) * s.dn[1022][4]));
        let eq77_e2193_d_n5: f64 = ((nv4 - nv11) * s.dn[1022][5]);
        let eq77_e2193_d_n6: f64 = ((nv4 - nv11) * s.dn[1022][6]);
        let eq77_e2193_d_n7: f64 = ((nv4 - nv11) * s.dn[1022][7]);
        let eq77_e2193_d_n8: f64 = ((nv4 - nv11) * s.dn[1022][8]);
        let eq77_e2193_d_n9: f64 = ((nv4 - nv11) * s.dn[1022][9]);
        let eq77_e2193_d_n10: f64 = ((nv4 - nv11) * s.dn[1022][10]);
        let eq77_e2193_d_n11: f64 = ((-s.v[1022]) + ((nv4 - nv11) * s.dn[1022][11]));
        let eq77_e2193_d_n12: f64 = ((nv4 - nv11) * s.dn[1022][12]);
        let eq77_e2193_d_n13: f64 = ((nv4 - nv11) * s.dn[1022][13]);
        (eq77_e2193, eq77_e2193_d_n0, eq77_e2193_d_n1, eq77_e2193_d_n2, eq77_e2193_d_n3, eq77_e2193_d_n4, eq77_e2193_d_n5, eq77_e2193_d_n6, eq77_e2193_d_n7, eq77_e2193_d_n8, eq77_e2193_d_n9, eq77_e2193_d_n10, eq77_e2193_d_n11, eq77_e2193_d_n12, eq77_e2193_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e2195;
        let eq77_node_derivatives: [f64; 14] = [eq77_e2195_d_n0, eq77_e2195_d_n1, eq77_e2195_d_n2, eq77_e2195_d_n3, eq77_e2195_d_n4, eq77_e2195_d_n5, eq77_e2195_d_n6, eq77_e2195_d_n7, eq77_e2195_d_n8, eq77_e2195_d_n9, eq77_e2195_d_n10, eq77_e2195_d_n11, eq77_e2195_d_n12, eq77_e2195_d_n13];
        let eq77_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[11]),
            self.multiplicity * (eq77_value),
            &nodes,
            &eq77_node_derivatives,
            &branches,
            &eq77_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_78_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq78_e2198: f64 = (s.v[379] * s.v[496]);
        let eq78_e2198_d_n0: f64 = ((s.dn[379][0] * s.v[496]) + (s.v[379] * s.dn[496][0]));
        let eq78_e2198_d_n1: f64 = ((s.dn[379][1] * s.v[496]) + (s.v[379] * s.dn[496][1]));
        let eq78_e2198_d_n2: f64 = ((s.dn[379][2] * s.v[496]) + (s.v[379] * s.dn[496][2]));
        let eq78_e2198_d_n3: f64 = ((s.dn[379][3] * s.v[496]) + (s.v[379] * s.dn[496][3]));
        let eq78_e2198_d_n4: f64 = ((s.dn[379][4] * s.v[496]) + (s.v[379] * s.dn[496][4]));
        let eq78_e2198_d_n5: f64 = ((s.dn[379][5] * s.v[496]) + (s.v[379] * s.dn[496][5]));
        let eq78_e2198_d_n6: f64 = ((s.dn[379][6] * s.v[496]) + (s.v[379] * s.dn[496][6]));
        let eq78_e2198_d_n7: f64 = ((s.dn[379][7] * s.v[496]) + (s.v[379] * s.dn[496][7]));
        let eq78_e2198_d_n8: f64 = ((s.dn[379][8] * s.v[496]) + (s.v[379] * s.dn[496][8]));
        let eq78_e2198_d_n9: f64 = ((s.dn[379][9] * s.v[496]) + (s.v[379] * s.dn[496][9]));
        let eq78_e2198_d_n10: f64 = ((s.dn[379][10] * s.v[496]) + (s.v[379] * s.dn[496][10]));
        let eq78_e2198_d_n11: f64 = ((s.dn[379][11] * s.v[496]) + (s.v[379] * s.dn[496][11]));
        let eq78_e2198_d_n12: f64 = ((s.dn[379][12] * s.v[496]) + (s.v[379] * s.dn[496][12]));
        let eq78_e2198_d_n13: f64 = ((s.dn[379][13] * s.v[496]) + (s.v[379] * s.dn[496][13]));
        let eq78_e2201: f64 = ((nv10 - nv7) * s.v[1018]);
        let eq78_e2201_d_n7: f64 = (-s.v[1018]);
        let eq78_e2201_d_n10: f64 = s.v[1018];
        let eq78_e2202: f64 = (eq78_e2198 + eq78_e2201);
        let eq78_e2202_d_n7: f64 = (eq78_e2198_d_n7 + eq78_e2201_d_n7);
        let eq78_e2202_d_n10: f64 = (eq78_e2198_d_n10 + eq78_e2201_d_n10);
        let eq78_value: f64 = eq78_e2202;
        let eq78_node_derivatives: [f64; 14] = [eq78_e2198_d_n0, eq78_e2198_d_n1, eq78_e2198_d_n2, eq78_e2198_d_n3, eq78_e2198_d_n4, eq78_e2198_d_n5, eq78_e2198_d_n6, eq78_e2202_d_n7, eq78_e2198_d_n8, eq78_e2198_d_n9, eq78_e2202_d_n10, eq78_e2198_d_n11, eq78_e2198_d_n12, eq78_e2198_d_n13];
        let eq78_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            self.multiplicity * (eq78_value),
            &nodes,
            &eq78_node_derivatives,
            &branches,
            &eq78_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_79_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq79_e2205: f64 = (s.v[379] * s.v[497]);
        let eq79_e2205_d_n0: f64 = ((s.dn[379][0] * s.v[497]) + (s.v[379] * s.dn[497][0]));
        let eq79_e2205_d_n1: f64 = ((s.dn[379][1] * s.v[497]) + (s.v[379] * s.dn[497][1]));
        let eq79_e2205_d_n2: f64 = ((s.dn[379][2] * s.v[497]) + (s.v[379] * s.dn[497][2]));
        let eq79_e2205_d_n3: f64 = ((s.dn[379][3] * s.v[497]) + (s.v[379] * s.dn[497][3]));
        let eq79_e2205_d_n4: f64 = ((s.dn[379][4] * s.v[497]) + (s.v[379] * s.dn[497][4]));
        let eq79_e2205_d_n5: f64 = ((s.dn[379][5] * s.v[497]) + (s.v[379] * s.dn[497][5]));
        let eq79_e2205_d_n6: f64 = ((s.dn[379][6] * s.v[497]) + (s.v[379] * s.dn[497][6]));
        let eq79_e2205_d_n7: f64 = ((s.dn[379][7] * s.v[497]) + (s.v[379] * s.dn[497][7]));
        let eq79_e2205_d_n8: f64 = ((s.dn[379][8] * s.v[497]) + (s.v[379] * s.dn[497][8]));
        let eq79_e2205_d_n9: f64 = ((s.dn[379][9] * s.v[497]) + (s.v[379] * s.dn[497][9]));
        let eq79_e2205_d_n10: f64 = ((s.dn[379][10] * s.v[497]) + (s.v[379] * s.dn[497][10]));
        let eq79_e2205_d_n11: f64 = ((s.dn[379][11] * s.v[497]) + (s.v[379] * s.dn[497][11]));
        let eq79_e2205_d_n12: f64 = ((s.dn[379][12] * s.v[497]) + (s.v[379] * s.dn[497][12]));
        let eq79_e2205_d_n13: f64 = ((s.dn[379][13] * s.v[497]) + (s.v[379] * s.dn[497][13]));
        let eq79_e2208: f64 = ((nv10 - nv6) * s.v[1018]);
        let eq79_e2208_d_n6: f64 = (-s.v[1018]);
        let eq79_e2208_d_n10: f64 = s.v[1018];
        let eq79_e2209: f64 = (eq79_e2205 + eq79_e2208);
        let eq79_e2209_d_n6: f64 = (eq79_e2205_d_n6 + eq79_e2208_d_n6);
        let eq79_e2209_d_n10: f64 = (eq79_e2205_d_n10 + eq79_e2208_d_n10);
        let eq79_value: f64 = eq79_e2209;
        let eq79_node_derivatives: [f64; 14] = [eq79_e2205_d_n0, eq79_e2205_d_n1, eq79_e2205_d_n2, eq79_e2205_d_n3, eq79_e2205_d_n4, eq79_e2205_d_n5, eq79_e2209_d_n6, eq79_e2205_d_n7, eq79_e2205_d_n8, eq79_e2205_d_n9, eq79_e2209_d_n10, eq79_e2205_d_n11, eq79_e2205_d_n12, eq79_e2205_d_n13];
        let eq79_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            self.multiplicity * (eq79_value),
            &nodes,
            &eq79_node_derivatives,
            &branches,
            &eq79_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_80_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq80_e2212: f64 = self.eval_ddt(21, s.v[520]);
        let eq80_e2212_d_n0: f64 = self.ddt_jacobian(s.dn[520][0]);
        let eq80_e2212_d_n1: f64 = self.ddt_jacobian(s.dn[520][1]);
        let eq80_e2212_d_n2: f64 = self.ddt_jacobian(s.dn[520][2]);
        let eq80_e2212_d_n3: f64 = self.ddt_jacobian(s.dn[520][3]);
        let eq80_e2212_d_n4: f64 = self.ddt_jacobian(s.dn[520][4]);
        let eq80_e2212_d_n5: f64 = self.ddt_jacobian(s.dn[520][5]);
        let eq80_e2212_d_n6: f64 = self.ddt_jacobian(s.dn[520][6]);
        let eq80_e2212_d_n7: f64 = self.ddt_jacobian(s.dn[520][7]);
        let eq80_e2212_d_n8: f64 = self.ddt_jacobian(s.dn[520][8]);
        let eq80_e2212_d_n9: f64 = self.ddt_jacobian(s.dn[520][9]);
        let eq80_e2212_d_n10: f64 = self.ddt_jacobian(s.dn[520][10]);
        let eq80_e2212_d_n11: f64 = self.ddt_jacobian(s.dn[520][11]);
        let eq80_e2212_d_n12: f64 = self.ddt_jacobian(s.dn[520][12]);
        let eq80_e2212_d_n13: f64 = self.ddt_jacobian(s.dn[520][13]);
        let eq80_e2213: f64 = (s.v[379] * eq80_e2212);
        let eq80_e2213_d_n0: f64 = ((s.dn[379][0] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n0));
        let eq80_e2213_d_n1: f64 = ((s.dn[379][1] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n1));
        let eq80_e2213_d_n2: f64 = ((s.dn[379][2] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n2));
        let eq80_e2213_d_n3: f64 = ((s.dn[379][3] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n3));
        let eq80_e2213_d_n4: f64 = ((s.dn[379][4] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n4));
        let eq80_e2213_d_n5: f64 = ((s.dn[379][5] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n5));
        let eq80_e2213_d_n6: f64 = ((s.dn[379][6] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n6));
        let eq80_e2213_d_n7: f64 = ((s.dn[379][7] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n7));
        let eq80_e2213_d_n8: f64 = ((s.dn[379][8] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n8));
        let eq80_e2213_d_n9: f64 = ((s.dn[379][9] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n9));
        let eq80_e2213_d_n10: f64 = ((s.dn[379][10] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n10));
        let eq80_e2213_d_n11: f64 = ((s.dn[379][11] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n11));
        let eq80_e2213_d_n12: f64 = ((s.dn[379][12] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n12));
        let eq80_e2213_d_n13: f64 = ((s.dn[379][13] * eq80_e2212) + (s.v[379] * eq80_e2212_d_n13));
        let eq80_value: f64 = eq80_e2213;
        let eq80_node_derivatives: [f64; 14] = [eq80_e2213_d_n0, eq80_e2213_d_n1, eq80_e2213_d_n2, eq80_e2213_d_n3, eq80_e2213_d_n4, eq80_e2213_d_n5, eq80_e2213_d_n6, eq80_e2213_d_n7, eq80_e2213_d_n8, eq80_e2213_d_n9, eq80_e2213_d_n10, eq80_e2213_d_n11, eq80_e2213_d_n12, eq80_e2213_d_n13];
        let eq80_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            self.multiplicity * (eq80_value),
            &nodes,
            &eq80_node_derivatives,
            &branches,
            &eq80_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_81_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq81_e2216: f64 = self.eval_ddt(22, s.v[525]);
        let eq81_e2216_d_n0: f64 = self.ddt_jacobian(s.dn[525][0]);
        let eq81_e2216_d_n1: f64 = self.ddt_jacobian(s.dn[525][1]);
        let eq81_e2216_d_n2: f64 = self.ddt_jacobian(s.dn[525][2]);
        let eq81_e2216_d_n3: f64 = self.ddt_jacobian(s.dn[525][3]);
        let eq81_e2216_d_n4: f64 = self.ddt_jacobian(s.dn[525][4]);
        let eq81_e2216_d_n5: f64 = self.ddt_jacobian(s.dn[525][5]);
        let eq81_e2216_d_n6: f64 = self.ddt_jacobian(s.dn[525][6]);
        let eq81_e2216_d_n7: f64 = self.ddt_jacobian(s.dn[525][7]);
        let eq81_e2216_d_n8: f64 = self.ddt_jacobian(s.dn[525][8]);
        let eq81_e2216_d_n9: f64 = self.ddt_jacobian(s.dn[525][9]);
        let eq81_e2216_d_n10: f64 = self.ddt_jacobian(s.dn[525][10]);
        let eq81_e2216_d_n11: f64 = self.ddt_jacobian(s.dn[525][11]);
        let eq81_e2216_d_n12: f64 = self.ddt_jacobian(s.dn[525][12]);
        let eq81_e2216_d_n13: f64 = self.ddt_jacobian(s.dn[525][13]);
        let eq81_e2217: f64 = (s.v[379] * eq81_e2216);
        let eq81_e2217_d_n0: f64 = ((s.dn[379][0] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n0));
        let eq81_e2217_d_n1: f64 = ((s.dn[379][1] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n1));
        let eq81_e2217_d_n2: f64 = ((s.dn[379][2] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n2));
        let eq81_e2217_d_n3: f64 = ((s.dn[379][3] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n3));
        let eq81_e2217_d_n4: f64 = ((s.dn[379][4] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n4));
        let eq81_e2217_d_n5: f64 = ((s.dn[379][5] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n5));
        let eq81_e2217_d_n6: f64 = ((s.dn[379][6] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n6));
        let eq81_e2217_d_n7: f64 = ((s.dn[379][7] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n7));
        let eq81_e2217_d_n8: f64 = ((s.dn[379][8] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n8));
        let eq81_e2217_d_n9: f64 = ((s.dn[379][9] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n9));
        let eq81_e2217_d_n10: f64 = ((s.dn[379][10] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n10));
        let eq81_e2217_d_n11: f64 = ((s.dn[379][11] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n11));
        let eq81_e2217_d_n12: f64 = ((s.dn[379][12] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n12));
        let eq81_e2217_d_n13: f64 = ((s.dn[379][13] * eq81_e2216) + (s.v[379] * eq81_e2216_d_n13));
        let eq81_value: f64 = eq81_e2217;
        let eq81_node_derivatives: [f64; 14] = [eq81_e2217_d_n0, eq81_e2217_d_n1, eq81_e2217_d_n2, eq81_e2217_d_n3, eq81_e2217_d_n4, eq81_e2217_d_n5, eq81_e2217_d_n6, eq81_e2217_d_n7, eq81_e2217_d_n8, eq81_e2217_d_n9, eq81_e2217_d_n10, eq81_e2217_d_n11, eq81_e2217_d_n12, eq81_e2217_d_n13];
        let eq81_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            self.multiplicity * (eq81_value),
            &nodes,
            &eq81_node_derivatives,
            &branches,
            &eq81_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_82_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq82_e2223, eq82_e2223_d_n7, eq82_e2223_d_n11,) = {
    if (s.v[2039] != 0.0) {
        let eq82_e2221: f64 = ((nv11 - nv7) * s.v[1018]);
        let eq82_e2221_d_n7: f64 = (-s.v[1018]);
        let eq82_e2221_d_n11: f64 = s.v[1018];
        (eq82_e2221, eq82_e2221_d_n7, eq82_e2221_d_n11,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e2223;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq82_value),
            &[
                GeneratedDerivative::node(nodes[7], self.multiplicity * eq82_e2223_d_n7),
                GeneratedDerivative::node(nodes[11], self.multiplicity * eq82_e2223_d_n11),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_83_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq83_e2229, eq83_e2229_d_n6, eq83_e2229_d_n11,) = {
    if (s.v[2039] != 0.0) {
        let eq83_e2227: f64 = ((nv11 - nv6) * s.v[1018]);
        let eq83_e2227_d_n6: f64 = (-s.v[1018]);
        let eq83_e2227_d_n11: f64 = s.v[1018];
        (eq83_e2227, eq83_e2227_d_n6, eq83_e2227_d_n11,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e2229;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq83_value),
            &[
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq83_e2229_d_n6),
                GeneratedDerivative::node(nodes[11], self.multiplicity * eq83_e2229_d_n11),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq7_e1546, eq7_e1546_d_n0, eq7_e1546_d_n1, eq7_e1546_d_n2, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, eq7_e1546_d_n13, eq7_e1546_q, eq7_e1546_q_d_n0, eq7_e1546_q_d_n1, eq7_e1546_q_d_n2, eq7_e1546_q_d_n3, eq7_e1546_q_d_n4, eq7_e1546_q_d_n5, eq7_e1546_q_d_n6, eq7_e1546_q_d_n7, eq7_e1546_q_d_n8, eq7_e1546_q_d_n9, eq7_e1546_q_d_n10, eq7_e1546_q_d_n11, eq7_e1546_q_d_n12, eq7_e1546_q_d_n13,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        let eq7_e1535: f64 = (s.v[622] * s.v[199]);
        let eq7_e1535_d_n0: f64 = (s.dn[622][0] * s.v[199]);
        let eq7_e1535_d_n1: f64 = (s.dn[622][1] * s.v[199]);
        let eq7_e1535_d_n2: f64 = (s.dn[622][2] * s.v[199]);
        let eq7_e1535_d_n3: f64 = (s.dn[622][3] * s.v[199]);
        let eq7_e1535_d_n4: f64 = (s.dn[622][4] * s.v[199]);
        let eq7_e1535_d_n5: f64 = (s.dn[622][5] * s.v[199]);
        let eq7_e1535_d_n6: f64 = (s.dn[622][6] * s.v[199]);
        let eq7_e1535_d_n7: f64 = (s.dn[622][7] * s.v[199]);
        let eq7_e1535_d_n8: f64 = (s.dn[622][8] * s.v[199]);
        let eq7_e1535_d_n9: f64 = (s.dn[622][9] * s.v[199]);
        let eq7_e1535_d_n10: f64 = (s.dn[622][10] * s.v[199]);
        let eq7_e1535_d_n11: f64 = (s.dn[622][11] * s.v[199]);
        let eq7_e1535_d_n12: f64 = (s.dn[622][12] * s.v[199]);
        let eq7_e1535_d_n13: f64 = (s.dn[622][13] * s.v[199]);
        let eq7_e1537: f64 = (eq7_e1535 * s.v[183]);
        let eq7_e1537_d_n0: f64 = (eq7_e1535_d_n0 * s.v[183]);
        let eq7_e1537_d_n1: f64 = (eq7_e1535_d_n1 * s.v[183]);
        let eq7_e1537_d_n2: f64 = (eq7_e1535_d_n2 * s.v[183]);
        let eq7_e1537_d_n3: f64 = (eq7_e1535_d_n3 * s.v[183]);
        let eq7_e1537_d_n4: f64 = (eq7_e1535_d_n4 * s.v[183]);
        let eq7_e1537_d_n5: f64 = (eq7_e1535_d_n5 * s.v[183]);
        let eq7_e1537_d_n6: f64 = (eq7_e1535_d_n6 * s.v[183]);
        let eq7_e1537_d_n7: f64 = (eq7_e1535_d_n7 * s.v[183]);
        let eq7_e1537_d_n8: f64 = (eq7_e1535_d_n8 * s.v[183]);
        let eq7_e1537_d_n9: f64 = (eq7_e1535_d_n9 * s.v[183]);
        let eq7_e1537_d_n10: f64 = (eq7_e1535_d_n10 * s.v[183]);
        let eq7_e1537_d_n11: f64 = (eq7_e1535_d_n11 * s.v[183]);
        let eq7_e1537_d_n12: f64 = (eq7_e1535_d_n12 * s.v[183]);
        let eq7_e1537_d_n13: f64 = (eq7_e1535_d_n13 * s.v[183]);
        let eq7_e1539: f64 = (eq7_e1537 * p.p2);
        let eq7_e1539_d_n0: f64 = (eq7_e1537_d_n0 * p.p2);
        let eq7_e1539_d_n1: f64 = (eq7_e1537_d_n1 * p.p2);
        let eq7_e1539_d_n2: f64 = (eq7_e1537_d_n2 * p.p2);
        let eq7_e1539_d_n3: f64 = (eq7_e1537_d_n3 * p.p2);
        let eq7_e1539_d_n4: f64 = (eq7_e1537_d_n4 * p.p2);
        let eq7_e1539_d_n5: f64 = (eq7_e1537_d_n5 * p.p2);
        let eq7_e1539_d_n6: f64 = (eq7_e1537_d_n6 * p.p2);
        let eq7_e1539_d_n7: f64 = (eq7_e1537_d_n7 * p.p2);
        let eq7_e1539_d_n8: f64 = (eq7_e1537_d_n8 * p.p2);
        let eq7_e1539_d_n9: f64 = (eq7_e1537_d_n9 * p.p2);
        let eq7_e1539_d_n10: f64 = (eq7_e1537_d_n10 * p.p2);
        let eq7_e1539_d_n11: f64 = (eq7_e1537_d_n11 * p.p2);
        let eq7_e1539_d_n12: f64 = (eq7_e1537_d_n12 * p.p2);
        let eq7_e1539_d_n13: f64 = (eq7_e1537_d_n13 * p.p2);
        let eq7_e1541: f64 = (eq7_e1539 * s.v[184]);
        let eq7_e1541_d_n0: f64 = (eq7_e1539_d_n0 * s.v[184]);
        let eq7_e1541_d_n1: f64 = (eq7_e1539_d_n1 * s.v[184]);
        let eq7_e1541_d_n2: f64 = (eq7_e1539_d_n2 * s.v[184]);
        let eq7_e1541_d_n3: f64 = (eq7_e1539_d_n3 * s.v[184]);
        let eq7_e1541_d_n4: f64 = (eq7_e1539_d_n4 * s.v[184]);
        let eq7_e1541_d_n5: f64 = (eq7_e1539_d_n5 * s.v[184]);
        let eq7_e1541_d_n6: f64 = (eq7_e1539_d_n6 * s.v[184]);
        let eq7_e1541_d_n7: f64 = (eq7_e1539_d_n7 * s.v[184]);
        let eq7_e1541_d_n8: f64 = (eq7_e1539_d_n8 * s.v[184]);
        let eq7_e1541_d_n9: f64 = (eq7_e1539_d_n9 * s.v[184]);
        let eq7_e1541_d_n10: f64 = (eq7_e1539_d_n10 * s.v[184]);
        let eq7_e1541_d_n11: f64 = (eq7_e1539_d_n11 * s.v[184]);
        let eq7_e1541_d_n12: f64 = (eq7_e1539_d_n12 * s.v[184]);
        let eq7_e1541_d_n13: f64 = (eq7_e1539_d_n13 * s.v[184]);
        let eq7_e1543: f64 = (eq7_e1541 * (nv12 - 0.0));
        let eq7_e1543_d_n0: f64 = (eq7_e1541_d_n0 * (nv12 - 0.0));
        let eq7_e1543_d_n1: f64 = (eq7_e1541_d_n1 * (nv12 - 0.0));
        let eq7_e1543_d_n2: f64 = (eq7_e1541_d_n2 * (nv12 - 0.0));
        let eq7_e1543_d_n3: f64 = (eq7_e1541_d_n3 * (nv12 - 0.0));
        let eq7_e1543_d_n4: f64 = (eq7_e1541_d_n4 * (nv12 - 0.0));
        let eq7_e1543_d_n5: f64 = (eq7_e1541_d_n5 * (nv12 - 0.0));
        let eq7_e1543_d_n6: f64 = (eq7_e1541_d_n6 * (nv12 - 0.0));
        let eq7_e1543_d_n7: f64 = (eq7_e1541_d_n7 * (nv12 - 0.0));
        let eq7_e1543_d_n8: f64 = (eq7_e1541_d_n8 * (nv12 - 0.0));
        let eq7_e1543_d_n9: f64 = (eq7_e1541_d_n9 * (nv12 - 0.0));
        let eq7_e1543_d_n10: f64 = (eq7_e1541_d_n10 * (nv12 - 0.0));
        let eq7_e1543_d_n11: f64 = (eq7_e1541_d_n11 * (nv12 - 0.0));
        let eq7_e1543_d_n12: f64 = ((eq7_e1541_d_n12 * (nv12 - 0.0)) + eq7_e1541);
        let eq7_e1543_d_n13: f64 = (eq7_e1541_d_n13 * (nv12 - 0.0));
        let eq7_e1544_q: f64 = eq7_e1543;
        (eq7_e1543, eq7_e1543_d_n0, eq7_e1543_d_n1, eq7_e1543_d_n2, eq7_e1543_d_n3, eq7_e1543_d_n4, eq7_e1543_d_n5, eq7_e1543_d_n6, eq7_e1543_d_n7, eq7_e1543_d_n8, eq7_e1543_d_n9, eq7_e1543_d_n10, eq7_e1543_d_n11, eq7_e1543_d_n12, eq7_e1543_d_n13, eq7_e1544_q, eq7_e1543_d_n0, eq7_e1543_d_n1, eq7_e1543_d_n2, eq7_e1543_d_n3, eq7_e1543_d_n4, eq7_e1543_d_n5, eq7_e1543_d_n6, eq7_e1543_d_n7, eq7_e1543_d_n8, eq7_e1543_d_n9, eq7_e1543_d_n10, eq7_e1543_d_n11, eq7_e1543_d_n12, eq7_e1543_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 14] = [eq7_e1546_q_d_n0, eq7_e1546_q_d_n1, eq7_e1546_q_d_n2, eq7_e1546_q_d_n3, eq7_e1546_q_d_n4, eq7_e1546_q_d_n5, eq7_e1546_q_d_n6, eq7_e1546_q_d_n7, eq7_e1546_q_d_n8, eq7_e1546_q_d_n9, eq7_e1546_q_d_n10, eq7_e1546_q_d_n11, eq7_e1546_q_d_n12, eq7_e1546_q_d_n13];
        let eq7_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            &nodes,
            &eq7_reactive_node_derivatives,
            &branches,
            &eq7_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq10_e1600, eq10_e1600_d_n0, eq10_e1600_d_n1, eq10_e1600_d_n2, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, eq10_e1600_d_n13, eq10_e1600_q, eq10_e1600_q_d_n0, eq10_e1600_q_d_n1, eq10_e1600_q_d_n2, eq10_e1600_q_d_n3, eq10_e1600_q_d_n4, eq10_e1600_q_d_n5, eq10_e1600_q_d_n6, eq10_e1600_q_d_n7, eq10_e1600_q_d_n8, eq10_e1600_q_d_n9, eq10_e1600_q_d_n10, eq10_e1600_q_d_n11, eq10_e1600_q_d_n12, eq10_e1600_q_d_n13,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        let eq10_e1584: f64 = (1.0 + s.v[211]);
        let eq10_e1586: f64 = (eq10_e1584 * s.v[622]);
        let eq10_e1586_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq10_e1584 * s.dn[622][0]));
        let eq10_e1586_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq10_e1584 * s.dn[622][1]));
        let eq10_e1586_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq10_e1584 * s.dn[622][2]));
        let eq10_e1586_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq10_e1584 * s.dn[622][3]));
        let eq10_e1586_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq10_e1584 * s.dn[622][4]));
        let eq10_e1586_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq10_e1584 * s.dn[622][5]));
        let eq10_e1586_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq10_e1584 * s.dn[622][6]));
        let eq10_e1586_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq10_e1584 * s.dn[622][7]));
        let eq10_e1586_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq10_e1584 * s.dn[622][8]));
        let eq10_e1586_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq10_e1584 * s.dn[622][9]));
        let eq10_e1586_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq10_e1584 * s.dn[622][10]));
        let eq10_e1586_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq10_e1584 * s.dn[622][11]));
        let eq10_e1586_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq10_e1584 * s.dn[622][12]));
        let eq10_e1586_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq10_e1584 * s.dn[622][13]));
        let eq10_e1588: f64 = (eq10_e1586 * s.v[199]);
        let eq10_e1588_d_n0: f64 = (eq10_e1586_d_n0 * s.v[199]);
        let eq10_e1588_d_n1: f64 = (eq10_e1586_d_n1 * s.v[199]);
        let eq10_e1588_d_n2: f64 = (eq10_e1586_d_n2 * s.v[199]);
        let eq10_e1588_d_n3: f64 = (eq10_e1586_d_n3 * s.v[199]);
        let eq10_e1588_d_n4: f64 = (eq10_e1586_d_n4 * s.v[199]);
        let eq10_e1588_d_n5: f64 = (eq10_e1586_d_n5 * s.v[199]);
        let eq10_e1588_d_n6: f64 = (eq10_e1586_d_n6 * s.v[199]);
        let eq10_e1588_d_n7: f64 = (eq10_e1586_d_n7 * s.v[199]);
        let eq10_e1588_d_n8: f64 = (eq10_e1586_d_n8 * s.v[199]);
        let eq10_e1588_d_n9: f64 = (eq10_e1586_d_n9 * s.v[199]);
        let eq10_e1588_d_n10: f64 = (eq10_e1586_d_n10 * s.v[199]);
        let eq10_e1588_d_n11: f64 = (eq10_e1586_d_n11 * s.v[199]);
        let eq10_e1588_d_n12: f64 = (eq10_e1586_d_n12 * s.v[199]);
        let eq10_e1588_d_n13: f64 = (eq10_e1586_d_n13 * s.v[199]);
        let eq10_e1590: f64 = (eq10_e1588 * s.v[183]);
        let eq10_e1590_d_n0: f64 = (eq10_e1588_d_n0 * s.v[183]);
        let eq10_e1590_d_n1: f64 = (eq10_e1588_d_n1 * s.v[183]);
        let eq10_e1590_d_n2: f64 = (eq10_e1588_d_n2 * s.v[183]);
        let eq10_e1590_d_n3: f64 = (eq10_e1588_d_n3 * s.v[183]);
        let eq10_e1590_d_n4: f64 = (eq10_e1588_d_n4 * s.v[183]);
        let eq10_e1590_d_n5: f64 = (eq10_e1588_d_n5 * s.v[183]);
        let eq10_e1590_d_n6: f64 = (eq10_e1588_d_n6 * s.v[183]);
        let eq10_e1590_d_n7: f64 = (eq10_e1588_d_n7 * s.v[183]);
        let eq10_e1590_d_n8: f64 = (eq10_e1588_d_n8 * s.v[183]);
        let eq10_e1590_d_n9: f64 = (eq10_e1588_d_n9 * s.v[183]);
        let eq10_e1590_d_n10: f64 = (eq10_e1588_d_n10 * s.v[183]);
        let eq10_e1590_d_n11: f64 = (eq10_e1588_d_n11 * s.v[183]);
        let eq10_e1590_d_n12: f64 = (eq10_e1588_d_n12 * s.v[183]);
        let eq10_e1590_d_n13: f64 = (eq10_e1588_d_n13 * s.v[183]);
        let eq10_e1592: f64 = (eq10_e1590 * p.p2);
        let eq10_e1592_d_n0: f64 = (eq10_e1590_d_n0 * p.p2);
        let eq10_e1592_d_n1: f64 = (eq10_e1590_d_n1 * p.p2);
        let eq10_e1592_d_n2: f64 = (eq10_e1590_d_n2 * p.p2);
        let eq10_e1592_d_n3: f64 = (eq10_e1590_d_n3 * p.p2);
        let eq10_e1592_d_n4: f64 = (eq10_e1590_d_n4 * p.p2);
        let eq10_e1592_d_n5: f64 = (eq10_e1590_d_n5 * p.p2);
        let eq10_e1592_d_n6: f64 = (eq10_e1590_d_n6 * p.p2);
        let eq10_e1592_d_n7: f64 = (eq10_e1590_d_n7 * p.p2);
        let eq10_e1592_d_n8: f64 = (eq10_e1590_d_n8 * p.p2);
        let eq10_e1592_d_n9: f64 = (eq10_e1590_d_n9 * p.p2);
        let eq10_e1592_d_n10: f64 = (eq10_e1590_d_n10 * p.p2);
        let eq10_e1592_d_n11: f64 = (eq10_e1590_d_n11 * p.p2);
        let eq10_e1592_d_n12: f64 = (eq10_e1590_d_n12 * p.p2);
        let eq10_e1592_d_n13: f64 = (eq10_e1590_d_n13 * p.p2);
        let eq10_e1594: f64 = (eq10_e1592 * s.v[184]);
        let eq10_e1594_d_n0: f64 = (eq10_e1592_d_n0 * s.v[184]);
        let eq10_e1594_d_n1: f64 = (eq10_e1592_d_n1 * s.v[184]);
        let eq10_e1594_d_n2: f64 = (eq10_e1592_d_n2 * s.v[184]);
        let eq10_e1594_d_n3: f64 = (eq10_e1592_d_n3 * s.v[184]);
        let eq10_e1594_d_n4: f64 = (eq10_e1592_d_n4 * s.v[184]);
        let eq10_e1594_d_n5: f64 = (eq10_e1592_d_n5 * s.v[184]);
        let eq10_e1594_d_n6: f64 = (eq10_e1592_d_n6 * s.v[184]);
        let eq10_e1594_d_n7: f64 = (eq10_e1592_d_n7 * s.v[184]);
        let eq10_e1594_d_n8: f64 = (eq10_e1592_d_n8 * s.v[184]);
        let eq10_e1594_d_n9: f64 = (eq10_e1592_d_n9 * s.v[184]);
        let eq10_e1594_d_n10: f64 = (eq10_e1592_d_n10 * s.v[184]);
        let eq10_e1594_d_n11: f64 = (eq10_e1592_d_n11 * s.v[184]);
        let eq10_e1594_d_n12: f64 = (eq10_e1592_d_n12 * s.v[184]);
        let eq10_e1594_d_n13: f64 = (eq10_e1592_d_n13 * s.v[184]);
        let eq10_e1596: f64 = (eq10_e1594 * (nv12 - 0.0));
        let eq10_e1596_d_n0: f64 = (eq10_e1594_d_n0 * (nv12 - 0.0));
        let eq10_e1596_d_n1: f64 = (eq10_e1594_d_n1 * (nv12 - 0.0));
        let eq10_e1596_d_n2: f64 = (eq10_e1594_d_n2 * (nv12 - 0.0));
        let eq10_e1596_d_n3: f64 = (eq10_e1594_d_n3 * (nv12 - 0.0));
        let eq10_e1596_d_n4: f64 = (eq10_e1594_d_n4 * (nv12 - 0.0));
        let eq10_e1596_d_n5: f64 = (eq10_e1594_d_n5 * (nv12 - 0.0));
        let eq10_e1596_d_n6: f64 = (eq10_e1594_d_n6 * (nv12 - 0.0));
        let eq10_e1596_d_n7: f64 = (eq10_e1594_d_n7 * (nv12 - 0.0));
        let eq10_e1596_d_n8: f64 = (eq10_e1594_d_n8 * (nv12 - 0.0));
        let eq10_e1596_d_n9: f64 = (eq10_e1594_d_n9 * (nv12 - 0.0));
        let eq10_e1596_d_n10: f64 = (eq10_e1594_d_n10 * (nv12 - 0.0));
        let eq10_e1596_d_n11: f64 = (eq10_e1594_d_n11 * (nv12 - 0.0));
        let eq10_e1596_d_n12: f64 = ((eq10_e1594_d_n12 * (nv12 - 0.0)) + eq10_e1594);
        let eq10_e1596_d_n13: f64 = (eq10_e1594_d_n13 * (nv12 - 0.0));
        let eq10_e1597: f64 = (0.5 * eq10_e1596);
        let eq10_e1597_d_n0: f64 = (0.5 * eq10_e1596_d_n0);
        let eq10_e1597_d_n1: f64 = (0.5 * eq10_e1596_d_n1);
        let eq10_e1597_d_n2: f64 = (0.5 * eq10_e1596_d_n2);
        let eq10_e1597_d_n3: f64 = (0.5 * eq10_e1596_d_n3);
        let eq10_e1597_d_n4: f64 = (0.5 * eq10_e1596_d_n4);
        let eq10_e1597_d_n5: f64 = (0.5 * eq10_e1596_d_n5);
        let eq10_e1597_d_n6: f64 = (0.5 * eq10_e1596_d_n6);
        let eq10_e1597_d_n7: f64 = (0.5 * eq10_e1596_d_n7);
        let eq10_e1597_d_n8: f64 = (0.5 * eq10_e1596_d_n8);
        let eq10_e1597_d_n9: f64 = (0.5 * eq10_e1596_d_n9);
        let eq10_e1597_d_n10: f64 = (0.5 * eq10_e1596_d_n10);
        let eq10_e1597_d_n11: f64 = (0.5 * eq10_e1596_d_n11);
        let eq10_e1597_d_n12: f64 = (0.5 * eq10_e1596_d_n12);
        let eq10_e1597_d_n13: f64 = (0.5 * eq10_e1596_d_n13);
        let eq10_e1598_q: f64 = eq10_e1597;
        (eq10_e1597, eq10_e1597_d_n0, eq10_e1597_d_n1, eq10_e1597_d_n2, eq10_e1597_d_n3, eq10_e1597_d_n4, eq10_e1597_d_n5, eq10_e1597_d_n6, eq10_e1597_d_n7, eq10_e1597_d_n8, eq10_e1597_d_n9, eq10_e1597_d_n10, eq10_e1597_d_n11, eq10_e1597_d_n12, eq10_e1597_d_n13, eq10_e1598_q, eq10_e1597_d_n0, eq10_e1597_d_n1, eq10_e1597_d_n2, eq10_e1597_d_n3, eq10_e1597_d_n4, eq10_e1597_d_n5, eq10_e1597_d_n6, eq10_e1597_d_n7, eq10_e1597_d_n8, eq10_e1597_d_n9, eq10_e1597_d_n10, eq10_e1597_d_n11, eq10_e1597_d_n12, eq10_e1597_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 14] = [eq10_e1600_q_d_n0, eq10_e1600_q_d_n1, eq10_e1600_q_d_n2, eq10_e1600_q_d_n3, eq10_e1600_q_d_n4, eq10_e1600_q_d_n5, eq10_e1600_q_d_n6, eq10_e1600_q_d_n7, eq10_e1600_q_d_n8, eq10_e1600_q_d_n9, eq10_e1600_q_d_n10, eq10_e1600_q_d_n11, eq10_e1600_q_d_n12, eq10_e1600_q_d_n13];
        let eq10_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            &nodes,
            &eq10_reactive_node_derivatives,
            &branches,
            &eq10_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq11_e1626, eq11_e1626_d_n0, eq11_e1626_d_n1, eq11_e1626_d_n2, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, eq11_e1626_d_n13, eq11_e1626_q, eq11_e1626_q_d_n0, eq11_e1626_q_d_n1, eq11_e1626_q_d_n2, eq11_e1626_q_d_n3, eq11_e1626_q_d_n4, eq11_e1626_q_d_n5, eq11_e1626_q_d_n6, eq11_e1626_q_d_n7, eq11_e1626_q_d_n8, eq11_e1626_q_d_n9, eq11_e1626_q_d_n10, eq11_e1626_q_d_n11, eq11_e1626_q_d_n12, eq11_e1626_q_d_n13,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        let eq11_e1610: f64 = (1.0 - s.v[211]);
        let eq11_e1610_d_n0: f64 = (-s.dn[211][0]);
        let eq11_e1610_d_n1: f64 = (-s.dn[211][1]);
        let eq11_e1610_d_n2: f64 = (-s.dn[211][2]);
        let eq11_e1610_d_n3: f64 = (-s.dn[211][3]);
        let eq11_e1610_d_n4: f64 = (-s.dn[211][4]);
        let eq11_e1610_d_n5: f64 = (-s.dn[211][5]);
        let eq11_e1610_d_n6: f64 = (-s.dn[211][6]);
        let eq11_e1610_d_n7: f64 = (-s.dn[211][7]);
        let eq11_e1610_d_n8: f64 = (-s.dn[211][8]);
        let eq11_e1610_d_n9: f64 = (-s.dn[211][9]);
        let eq11_e1610_d_n10: f64 = (-s.dn[211][10]);
        let eq11_e1610_d_n11: f64 = (-s.dn[211][11]);
        let eq11_e1610_d_n12: f64 = (-s.dn[211][12]);
        let eq11_e1610_d_n13: f64 = (-s.dn[211][13]);
        let eq11_e1612: f64 = (eq11_e1610 * s.v[622]);
        let eq11_e1612_d_n0: f64 = ((eq11_e1610_d_n0 * s.v[622]) + (eq11_e1610 * s.dn[622][0]));
        let eq11_e1612_d_n1: f64 = ((eq11_e1610_d_n1 * s.v[622]) + (eq11_e1610 * s.dn[622][1]));
        let eq11_e1612_d_n2: f64 = ((eq11_e1610_d_n2 * s.v[622]) + (eq11_e1610 * s.dn[622][2]));
        let eq11_e1612_d_n3: f64 = ((eq11_e1610_d_n3 * s.v[622]) + (eq11_e1610 * s.dn[622][3]));
        let eq11_e1612_d_n4: f64 = ((eq11_e1610_d_n4 * s.v[622]) + (eq11_e1610 * s.dn[622][4]));
        let eq11_e1612_d_n5: f64 = ((eq11_e1610_d_n5 * s.v[622]) + (eq11_e1610 * s.dn[622][5]));
        let eq11_e1612_d_n6: f64 = ((eq11_e1610_d_n6 * s.v[622]) + (eq11_e1610 * s.dn[622][6]));
        let eq11_e1612_d_n7: f64 = ((eq11_e1610_d_n7 * s.v[622]) + (eq11_e1610 * s.dn[622][7]));
        let eq11_e1612_d_n8: f64 = ((eq11_e1610_d_n8 * s.v[622]) + (eq11_e1610 * s.dn[622][8]));
        let eq11_e1612_d_n9: f64 = ((eq11_e1610_d_n9 * s.v[622]) + (eq11_e1610 * s.dn[622][9]));
        let eq11_e1612_d_n10: f64 = ((eq11_e1610_d_n10 * s.v[622]) + (eq11_e1610 * s.dn[622][10]));
        let eq11_e1612_d_n11: f64 = ((eq11_e1610_d_n11 * s.v[622]) + (eq11_e1610 * s.dn[622][11]));
        let eq11_e1612_d_n12: f64 = ((eq11_e1610_d_n12 * s.v[622]) + (eq11_e1610 * s.dn[622][12]));
        let eq11_e1612_d_n13: f64 = ((eq11_e1610_d_n13 * s.v[622]) + (eq11_e1610 * s.dn[622][13]));
        let eq11_e1614: f64 = (eq11_e1612 * s.v[199]);
        let eq11_e1614_d_n0: f64 = (eq11_e1612_d_n0 * s.v[199]);
        let eq11_e1614_d_n1: f64 = (eq11_e1612_d_n1 * s.v[199]);
        let eq11_e1614_d_n2: f64 = (eq11_e1612_d_n2 * s.v[199]);
        let eq11_e1614_d_n3: f64 = (eq11_e1612_d_n3 * s.v[199]);
        let eq11_e1614_d_n4: f64 = (eq11_e1612_d_n4 * s.v[199]);
        let eq11_e1614_d_n5: f64 = (eq11_e1612_d_n5 * s.v[199]);
        let eq11_e1614_d_n6: f64 = (eq11_e1612_d_n6 * s.v[199]);
        let eq11_e1614_d_n7: f64 = (eq11_e1612_d_n7 * s.v[199]);
        let eq11_e1614_d_n8: f64 = (eq11_e1612_d_n8 * s.v[199]);
        let eq11_e1614_d_n9: f64 = (eq11_e1612_d_n9 * s.v[199]);
        let eq11_e1614_d_n10: f64 = (eq11_e1612_d_n10 * s.v[199]);
        let eq11_e1614_d_n11: f64 = (eq11_e1612_d_n11 * s.v[199]);
        let eq11_e1614_d_n12: f64 = (eq11_e1612_d_n12 * s.v[199]);
        let eq11_e1614_d_n13: f64 = (eq11_e1612_d_n13 * s.v[199]);
        let eq11_e1616: f64 = (eq11_e1614 * s.v[183]);
        let eq11_e1616_d_n0: f64 = (eq11_e1614_d_n0 * s.v[183]);
        let eq11_e1616_d_n1: f64 = (eq11_e1614_d_n1 * s.v[183]);
        let eq11_e1616_d_n2: f64 = (eq11_e1614_d_n2 * s.v[183]);
        let eq11_e1616_d_n3: f64 = (eq11_e1614_d_n3 * s.v[183]);
        let eq11_e1616_d_n4: f64 = (eq11_e1614_d_n4 * s.v[183]);
        let eq11_e1616_d_n5: f64 = (eq11_e1614_d_n5 * s.v[183]);
        let eq11_e1616_d_n6: f64 = (eq11_e1614_d_n6 * s.v[183]);
        let eq11_e1616_d_n7: f64 = (eq11_e1614_d_n7 * s.v[183]);
        let eq11_e1616_d_n8: f64 = (eq11_e1614_d_n8 * s.v[183]);
        let eq11_e1616_d_n9: f64 = (eq11_e1614_d_n9 * s.v[183]);
        let eq11_e1616_d_n10: f64 = (eq11_e1614_d_n10 * s.v[183]);
        let eq11_e1616_d_n11: f64 = (eq11_e1614_d_n11 * s.v[183]);
        let eq11_e1616_d_n12: f64 = (eq11_e1614_d_n12 * s.v[183]);
        let eq11_e1616_d_n13: f64 = (eq11_e1614_d_n13 * s.v[183]);
        let eq11_e1618: f64 = (eq11_e1616 * p.p2);
        let eq11_e1618_d_n0: f64 = (eq11_e1616_d_n0 * p.p2);
        let eq11_e1618_d_n1: f64 = (eq11_e1616_d_n1 * p.p2);
        let eq11_e1618_d_n2: f64 = (eq11_e1616_d_n2 * p.p2);
        let eq11_e1618_d_n3: f64 = (eq11_e1616_d_n3 * p.p2);
        let eq11_e1618_d_n4: f64 = (eq11_e1616_d_n4 * p.p2);
        let eq11_e1618_d_n5: f64 = (eq11_e1616_d_n5 * p.p2);
        let eq11_e1618_d_n6: f64 = (eq11_e1616_d_n6 * p.p2);
        let eq11_e1618_d_n7: f64 = (eq11_e1616_d_n7 * p.p2);
        let eq11_e1618_d_n8: f64 = (eq11_e1616_d_n8 * p.p2);
        let eq11_e1618_d_n9: f64 = (eq11_e1616_d_n9 * p.p2);
        let eq11_e1618_d_n10: f64 = (eq11_e1616_d_n10 * p.p2);
        let eq11_e1618_d_n11: f64 = (eq11_e1616_d_n11 * p.p2);
        let eq11_e1618_d_n12: f64 = (eq11_e1616_d_n12 * p.p2);
        let eq11_e1618_d_n13: f64 = (eq11_e1616_d_n13 * p.p2);
        let eq11_e1620: f64 = (eq11_e1618 * s.v[184]);
        let eq11_e1620_d_n0: f64 = (eq11_e1618_d_n0 * s.v[184]);
        let eq11_e1620_d_n1: f64 = (eq11_e1618_d_n1 * s.v[184]);
        let eq11_e1620_d_n2: f64 = (eq11_e1618_d_n2 * s.v[184]);
        let eq11_e1620_d_n3: f64 = (eq11_e1618_d_n3 * s.v[184]);
        let eq11_e1620_d_n4: f64 = (eq11_e1618_d_n4 * s.v[184]);
        let eq11_e1620_d_n5: f64 = (eq11_e1618_d_n5 * s.v[184]);
        let eq11_e1620_d_n6: f64 = (eq11_e1618_d_n6 * s.v[184]);
        let eq11_e1620_d_n7: f64 = (eq11_e1618_d_n7 * s.v[184]);
        let eq11_e1620_d_n8: f64 = (eq11_e1618_d_n8 * s.v[184]);
        let eq11_e1620_d_n9: f64 = (eq11_e1618_d_n9 * s.v[184]);
        let eq11_e1620_d_n10: f64 = (eq11_e1618_d_n10 * s.v[184]);
        let eq11_e1620_d_n11: f64 = (eq11_e1618_d_n11 * s.v[184]);
        let eq11_e1620_d_n12: f64 = (eq11_e1618_d_n12 * s.v[184]);
        let eq11_e1620_d_n13: f64 = (eq11_e1618_d_n13 * s.v[184]);
        let eq11_e1622: f64 = (eq11_e1620 * (nv12 - 0.0));
        let eq11_e1622_d_n0: f64 = (eq11_e1620_d_n0 * (nv12 - 0.0));
        let eq11_e1622_d_n1: f64 = (eq11_e1620_d_n1 * (nv12 - 0.0));
        let eq11_e1622_d_n2: f64 = (eq11_e1620_d_n2 * (nv12 - 0.0));
        let eq11_e1622_d_n3: f64 = (eq11_e1620_d_n3 * (nv12 - 0.0));
        let eq11_e1622_d_n4: f64 = (eq11_e1620_d_n4 * (nv12 - 0.0));
        let eq11_e1622_d_n5: f64 = (eq11_e1620_d_n5 * (nv12 - 0.0));
        let eq11_e1622_d_n6: f64 = (eq11_e1620_d_n6 * (nv12 - 0.0));
        let eq11_e1622_d_n7: f64 = (eq11_e1620_d_n7 * (nv12 - 0.0));
        let eq11_e1622_d_n8: f64 = (eq11_e1620_d_n8 * (nv12 - 0.0));
        let eq11_e1622_d_n9: f64 = (eq11_e1620_d_n9 * (nv12 - 0.0));
        let eq11_e1622_d_n10: f64 = (eq11_e1620_d_n10 * (nv12 - 0.0));
        let eq11_e1622_d_n11: f64 = (eq11_e1620_d_n11 * (nv12 - 0.0));
        let eq11_e1622_d_n12: f64 = ((eq11_e1620_d_n12 * (nv12 - 0.0)) + eq11_e1620);
        let eq11_e1622_d_n13: f64 = (eq11_e1620_d_n13 * (nv12 - 0.0));
        let eq11_e1623: f64 = (0.5 * eq11_e1622);
        let eq11_e1623_d_n0: f64 = (0.5 * eq11_e1622_d_n0);
        let eq11_e1623_d_n1: f64 = (0.5 * eq11_e1622_d_n1);
        let eq11_e1623_d_n2: f64 = (0.5 * eq11_e1622_d_n2);
        let eq11_e1623_d_n3: f64 = (0.5 * eq11_e1622_d_n3);
        let eq11_e1623_d_n4: f64 = (0.5 * eq11_e1622_d_n4);
        let eq11_e1623_d_n5: f64 = (0.5 * eq11_e1622_d_n5);
        let eq11_e1623_d_n6: f64 = (0.5 * eq11_e1622_d_n6);
        let eq11_e1623_d_n7: f64 = (0.5 * eq11_e1622_d_n7);
        let eq11_e1623_d_n8: f64 = (0.5 * eq11_e1622_d_n8);
        let eq11_e1623_d_n9: f64 = (0.5 * eq11_e1622_d_n9);
        let eq11_e1623_d_n10: f64 = (0.5 * eq11_e1622_d_n10);
        let eq11_e1623_d_n11: f64 = (0.5 * eq11_e1622_d_n11);
        let eq11_e1623_d_n12: f64 = (0.5 * eq11_e1622_d_n12);
        let eq11_e1623_d_n13: f64 = (0.5 * eq11_e1622_d_n13);
        let eq11_e1624_q: f64 = eq11_e1623;
        (eq11_e1623, eq11_e1623_d_n0, eq11_e1623_d_n1, eq11_e1623_d_n2, eq11_e1623_d_n3, eq11_e1623_d_n4, eq11_e1623_d_n5, eq11_e1623_d_n6, eq11_e1623_d_n7, eq11_e1623_d_n8, eq11_e1623_d_n9, eq11_e1623_d_n10, eq11_e1623_d_n11, eq11_e1623_d_n12, eq11_e1623_d_n13, eq11_e1624_q, eq11_e1623_d_n0, eq11_e1623_d_n1, eq11_e1623_d_n2, eq11_e1623_d_n3, eq11_e1623_d_n4, eq11_e1623_d_n5, eq11_e1623_d_n6, eq11_e1623_d_n7, eq11_e1623_d_n8, eq11_e1623_d_n9, eq11_e1623_d_n10, eq11_e1623_d_n11, eq11_e1623_d_n12, eq11_e1623_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 14] = [eq11_e1626_q_d_n0, eq11_e1626_q_d_n1, eq11_e1626_q_d_n2, eq11_e1626_q_d_n3, eq11_e1626_q_d_n4, eq11_e1626_q_d_n5, eq11_e1626_q_d_n6, eq11_e1626_q_d_n7, eq11_e1626_q_d_n8, eq11_e1626_q_d_n9, eq11_e1626_q_d_n10, eq11_e1626_q_d_n11, eq11_e1626_q_d_n12, eq11_e1626_q_d_n13];
        let eq11_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &nodes,
            &eq11_reactive_node_derivatives,
            &branches,
            &eq11_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_24_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq24_e1784, eq24_e1784_d_n0, eq24_e1784_d_n1, eq24_e1784_d_n2, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, eq24_e1784_d_n13, eq24_e1784_q, eq24_e1784_q_d_n0, eq24_e1784_q_d_n1, eq24_e1784_q_d_n2, eq24_e1784_q_d_n3, eq24_e1784_q_d_n4, eq24_e1784_q_d_n5, eq24_e1784_q_d_n6, eq24_e1784_q_d_n7, eq24_e1784_q_d_n8, eq24_e1784_q_d_n9, eq24_e1784_q_d_n10, eq24_e1784_q_d_n11, eq24_e1784_q_d_n12, eq24_e1784_q_d_n13,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        let eq24_e1773: f64 = (s.v[622] * s.v[199]);
        let eq24_e1773_d_n0: f64 = (s.dn[622][0] * s.v[199]);
        let eq24_e1773_d_n1: f64 = (s.dn[622][1] * s.v[199]);
        let eq24_e1773_d_n2: f64 = (s.dn[622][2] * s.v[199]);
        let eq24_e1773_d_n3: f64 = (s.dn[622][3] * s.v[199]);
        let eq24_e1773_d_n4: f64 = (s.dn[622][4] * s.v[199]);
        let eq24_e1773_d_n5: f64 = (s.dn[622][5] * s.v[199]);
        let eq24_e1773_d_n6: f64 = (s.dn[622][6] * s.v[199]);
        let eq24_e1773_d_n7: f64 = (s.dn[622][7] * s.v[199]);
        let eq24_e1773_d_n8: f64 = (s.dn[622][8] * s.v[199]);
        let eq24_e1773_d_n9: f64 = (s.dn[622][9] * s.v[199]);
        let eq24_e1773_d_n10: f64 = (s.dn[622][10] * s.v[199]);
        let eq24_e1773_d_n11: f64 = (s.dn[622][11] * s.v[199]);
        let eq24_e1773_d_n12: f64 = (s.dn[622][12] * s.v[199]);
        let eq24_e1773_d_n13: f64 = (s.dn[622][13] * s.v[199]);
        let eq24_e1775: f64 = (eq24_e1773 * s.v[183]);
        let eq24_e1775_d_n0: f64 = (eq24_e1773_d_n0 * s.v[183]);
        let eq24_e1775_d_n1: f64 = (eq24_e1773_d_n1 * s.v[183]);
        let eq24_e1775_d_n2: f64 = (eq24_e1773_d_n2 * s.v[183]);
        let eq24_e1775_d_n3: f64 = (eq24_e1773_d_n3 * s.v[183]);
        let eq24_e1775_d_n4: f64 = (eq24_e1773_d_n4 * s.v[183]);
        let eq24_e1775_d_n5: f64 = (eq24_e1773_d_n5 * s.v[183]);
        let eq24_e1775_d_n6: f64 = (eq24_e1773_d_n6 * s.v[183]);
        let eq24_e1775_d_n7: f64 = (eq24_e1773_d_n7 * s.v[183]);
        let eq24_e1775_d_n8: f64 = (eq24_e1773_d_n8 * s.v[183]);
        let eq24_e1775_d_n9: f64 = (eq24_e1773_d_n9 * s.v[183]);
        let eq24_e1775_d_n10: f64 = (eq24_e1773_d_n10 * s.v[183]);
        let eq24_e1775_d_n11: f64 = (eq24_e1773_d_n11 * s.v[183]);
        let eq24_e1775_d_n12: f64 = (eq24_e1773_d_n12 * s.v[183]);
        let eq24_e1775_d_n13: f64 = (eq24_e1773_d_n13 * s.v[183]);
        let eq24_e1777: f64 = (eq24_e1775 * p.p2);
        let eq24_e1777_d_n0: f64 = (eq24_e1775_d_n0 * p.p2);
        let eq24_e1777_d_n1: f64 = (eq24_e1775_d_n1 * p.p2);
        let eq24_e1777_d_n2: f64 = (eq24_e1775_d_n2 * p.p2);
        let eq24_e1777_d_n3: f64 = (eq24_e1775_d_n3 * p.p2);
        let eq24_e1777_d_n4: f64 = (eq24_e1775_d_n4 * p.p2);
        let eq24_e1777_d_n5: f64 = (eq24_e1775_d_n5 * p.p2);
        let eq24_e1777_d_n6: f64 = (eq24_e1775_d_n6 * p.p2);
        let eq24_e1777_d_n7: f64 = (eq24_e1775_d_n7 * p.p2);
        let eq24_e1777_d_n8: f64 = (eq24_e1775_d_n8 * p.p2);
        let eq24_e1777_d_n9: f64 = (eq24_e1775_d_n9 * p.p2);
        let eq24_e1777_d_n10: f64 = (eq24_e1775_d_n10 * p.p2);
        let eq24_e1777_d_n11: f64 = (eq24_e1775_d_n11 * p.p2);
        let eq24_e1777_d_n12: f64 = (eq24_e1775_d_n12 * p.p2);
        let eq24_e1777_d_n13: f64 = (eq24_e1775_d_n13 * p.p2);
        let eq24_e1779: f64 = (eq24_e1777 * s.v[184]);
        let eq24_e1779_d_n0: f64 = (eq24_e1777_d_n0 * s.v[184]);
        let eq24_e1779_d_n1: f64 = (eq24_e1777_d_n1 * s.v[184]);
        let eq24_e1779_d_n2: f64 = (eq24_e1777_d_n2 * s.v[184]);
        let eq24_e1779_d_n3: f64 = (eq24_e1777_d_n3 * s.v[184]);
        let eq24_e1779_d_n4: f64 = (eq24_e1777_d_n4 * s.v[184]);
        let eq24_e1779_d_n5: f64 = (eq24_e1777_d_n5 * s.v[184]);
        let eq24_e1779_d_n6: f64 = (eq24_e1777_d_n6 * s.v[184]);
        let eq24_e1779_d_n7: f64 = (eq24_e1777_d_n7 * s.v[184]);
        let eq24_e1779_d_n8: f64 = (eq24_e1777_d_n8 * s.v[184]);
        let eq24_e1779_d_n9: f64 = (eq24_e1777_d_n9 * s.v[184]);
        let eq24_e1779_d_n10: f64 = (eq24_e1777_d_n10 * s.v[184]);
        let eq24_e1779_d_n11: f64 = (eq24_e1777_d_n11 * s.v[184]);
        let eq24_e1779_d_n12: f64 = (eq24_e1777_d_n12 * s.v[184]);
        let eq24_e1779_d_n13: f64 = (eq24_e1777_d_n13 * s.v[184]);
        let eq24_e1781: f64 = (eq24_e1779 * (nv12 - 0.0));
        let eq24_e1781_d_n0: f64 = (eq24_e1779_d_n0 * (nv12 - 0.0));
        let eq24_e1781_d_n1: f64 = (eq24_e1779_d_n1 * (nv12 - 0.0));
        let eq24_e1781_d_n2: f64 = (eq24_e1779_d_n2 * (nv12 - 0.0));
        let eq24_e1781_d_n3: f64 = (eq24_e1779_d_n3 * (nv12 - 0.0));
        let eq24_e1781_d_n4: f64 = (eq24_e1779_d_n4 * (nv12 - 0.0));
        let eq24_e1781_d_n5: f64 = (eq24_e1779_d_n5 * (nv12 - 0.0));
        let eq24_e1781_d_n6: f64 = (eq24_e1779_d_n6 * (nv12 - 0.0));
        let eq24_e1781_d_n7: f64 = (eq24_e1779_d_n7 * (nv12 - 0.0));
        let eq24_e1781_d_n8: f64 = (eq24_e1779_d_n8 * (nv12 - 0.0));
        let eq24_e1781_d_n9: f64 = (eq24_e1779_d_n9 * (nv12 - 0.0));
        let eq24_e1781_d_n10: f64 = (eq24_e1779_d_n10 * (nv12 - 0.0));
        let eq24_e1781_d_n11: f64 = (eq24_e1779_d_n11 * (nv12 - 0.0));
        let eq24_e1781_d_n12: f64 = ((eq24_e1779_d_n12 * (nv12 - 0.0)) + eq24_e1779);
        let eq24_e1781_d_n13: f64 = (eq24_e1779_d_n13 * (nv12 - 0.0));
        let eq24_e1782_q: f64 = eq24_e1781;
        (eq24_e1781, eq24_e1781_d_n0, eq24_e1781_d_n1, eq24_e1781_d_n2, eq24_e1781_d_n3, eq24_e1781_d_n4, eq24_e1781_d_n5, eq24_e1781_d_n6, eq24_e1781_d_n7, eq24_e1781_d_n8, eq24_e1781_d_n9, eq24_e1781_d_n10, eq24_e1781_d_n11, eq24_e1781_d_n12, eq24_e1781_d_n13, eq24_e1782_q, eq24_e1781_d_n0, eq24_e1781_d_n1, eq24_e1781_d_n2, eq24_e1781_d_n3, eq24_e1781_d_n4, eq24_e1781_d_n5, eq24_e1781_d_n6, eq24_e1781_d_n7, eq24_e1781_d_n8, eq24_e1781_d_n9, eq24_e1781_d_n10, eq24_e1781_d_n11, eq24_e1781_d_n12, eq24_e1781_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_reactive_node_derivatives: [f64; 14] = [eq24_e1784_q_d_n0, eq24_e1784_q_d_n1, eq24_e1784_q_d_n2, eq24_e1784_q_d_n3, eq24_e1784_q_d_n4, eq24_e1784_q_d_n5, eq24_e1784_q_d_n6, eq24_e1784_q_d_n7, eq24_e1784_q_d_n8, eq24_e1784_q_d_n9, eq24_e1784_q_d_n10, eq24_e1784_q_d_n11, eq24_e1784_q_d_n12, eq24_e1784_q_d_n13];
        let eq24_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            &nodes,
            &eq24_reactive_node_derivatives,
            &branches,
            &eq24_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_27_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq27_e1841, eq27_e1841_d_n0, eq27_e1841_d_n1, eq27_e1841_d_n2, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_d_n13, eq27_e1841_q, eq27_e1841_q_d_n0, eq27_e1841_q_d_n1, eq27_e1841_q_d_n2, eq27_e1841_q_d_n3, eq27_e1841_q_d_n4, eq27_e1841_q_d_n5, eq27_e1841_q_d_n6, eq27_e1841_q_d_n7, eq27_e1841_q_d_n8, eq27_e1841_q_d_n9, eq27_e1841_q_d_n10, eq27_e1841_q_d_n11, eq27_e1841_q_d_n12, eq27_e1841_q_d_n13,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        let eq27_e1825: f64 = (1.0 + s.v[211]);
        let eq27_e1827: f64 = (eq27_e1825 * s.v[622]);
        let eq27_e1827_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq27_e1825 * s.dn[622][0]));
        let eq27_e1827_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq27_e1825 * s.dn[622][1]));
        let eq27_e1827_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq27_e1825 * s.dn[622][2]));
        let eq27_e1827_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq27_e1825 * s.dn[622][3]));
        let eq27_e1827_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq27_e1825 * s.dn[622][4]));
        let eq27_e1827_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq27_e1825 * s.dn[622][5]));
        let eq27_e1827_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq27_e1825 * s.dn[622][6]));
        let eq27_e1827_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq27_e1825 * s.dn[622][7]));
        let eq27_e1827_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq27_e1825 * s.dn[622][8]));
        let eq27_e1827_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq27_e1825 * s.dn[622][9]));
        let eq27_e1827_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq27_e1825 * s.dn[622][10]));
        let eq27_e1827_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq27_e1825 * s.dn[622][11]));
        let eq27_e1827_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq27_e1825 * s.dn[622][12]));
        let eq27_e1827_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq27_e1825 * s.dn[622][13]));
        let eq27_e1829: f64 = (eq27_e1827 * s.v[199]);
        let eq27_e1829_d_n0: f64 = (eq27_e1827_d_n0 * s.v[199]);
        let eq27_e1829_d_n1: f64 = (eq27_e1827_d_n1 * s.v[199]);
        let eq27_e1829_d_n2: f64 = (eq27_e1827_d_n2 * s.v[199]);
        let eq27_e1829_d_n3: f64 = (eq27_e1827_d_n3 * s.v[199]);
        let eq27_e1829_d_n4: f64 = (eq27_e1827_d_n4 * s.v[199]);
        let eq27_e1829_d_n5: f64 = (eq27_e1827_d_n5 * s.v[199]);
        let eq27_e1829_d_n6: f64 = (eq27_e1827_d_n6 * s.v[199]);
        let eq27_e1829_d_n7: f64 = (eq27_e1827_d_n7 * s.v[199]);
        let eq27_e1829_d_n8: f64 = (eq27_e1827_d_n8 * s.v[199]);
        let eq27_e1829_d_n9: f64 = (eq27_e1827_d_n9 * s.v[199]);
        let eq27_e1829_d_n10: f64 = (eq27_e1827_d_n10 * s.v[199]);
        let eq27_e1829_d_n11: f64 = (eq27_e1827_d_n11 * s.v[199]);
        let eq27_e1829_d_n12: f64 = (eq27_e1827_d_n12 * s.v[199]);
        let eq27_e1829_d_n13: f64 = (eq27_e1827_d_n13 * s.v[199]);
        let eq27_e1831: f64 = (eq27_e1829 * s.v[183]);
        let eq27_e1831_d_n0: f64 = (eq27_e1829_d_n0 * s.v[183]);
        let eq27_e1831_d_n1: f64 = (eq27_e1829_d_n1 * s.v[183]);
        let eq27_e1831_d_n2: f64 = (eq27_e1829_d_n2 * s.v[183]);
        let eq27_e1831_d_n3: f64 = (eq27_e1829_d_n3 * s.v[183]);
        let eq27_e1831_d_n4: f64 = (eq27_e1829_d_n4 * s.v[183]);
        let eq27_e1831_d_n5: f64 = (eq27_e1829_d_n5 * s.v[183]);
        let eq27_e1831_d_n6: f64 = (eq27_e1829_d_n6 * s.v[183]);
        let eq27_e1831_d_n7: f64 = (eq27_e1829_d_n7 * s.v[183]);
        let eq27_e1831_d_n8: f64 = (eq27_e1829_d_n8 * s.v[183]);
        let eq27_e1831_d_n9: f64 = (eq27_e1829_d_n9 * s.v[183]);
        let eq27_e1831_d_n10: f64 = (eq27_e1829_d_n10 * s.v[183]);
        let eq27_e1831_d_n11: f64 = (eq27_e1829_d_n11 * s.v[183]);
        let eq27_e1831_d_n12: f64 = (eq27_e1829_d_n12 * s.v[183]);
        let eq27_e1831_d_n13: f64 = (eq27_e1829_d_n13 * s.v[183]);
        let eq27_e1833: f64 = (eq27_e1831 * p.p2);
        let eq27_e1833_d_n0: f64 = (eq27_e1831_d_n0 * p.p2);
        let eq27_e1833_d_n1: f64 = (eq27_e1831_d_n1 * p.p2);
        let eq27_e1833_d_n2: f64 = (eq27_e1831_d_n2 * p.p2);
        let eq27_e1833_d_n3: f64 = (eq27_e1831_d_n3 * p.p2);
        let eq27_e1833_d_n4: f64 = (eq27_e1831_d_n4 * p.p2);
        let eq27_e1833_d_n5: f64 = (eq27_e1831_d_n5 * p.p2);
        let eq27_e1833_d_n6: f64 = (eq27_e1831_d_n6 * p.p2);
        let eq27_e1833_d_n7: f64 = (eq27_e1831_d_n7 * p.p2);
        let eq27_e1833_d_n8: f64 = (eq27_e1831_d_n8 * p.p2);
        let eq27_e1833_d_n9: f64 = (eq27_e1831_d_n9 * p.p2);
        let eq27_e1833_d_n10: f64 = (eq27_e1831_d_n10 * p.p2);
        let eq27_e1833_d_n11: f64 = (eq27_e1831_d_n11 * p.p2);
        let eq27_e1833_d_n12: f64 = (eq27_e1831_d_n12 * p.p2);
        let eq27_e1833_d_n13: f64 = (eq27_e1831_d_n13 * p.p2);
        let eq27_e1835: f64 = (eq27_e1833 * s.v[184]);
        let eq27_e1835_d_n0: f64 = (eq27_e1833_d_n0 * s.v[184]);
        let eq27_e1835_d_n1: f64 = (eq27_e1833_d_n1 * s.v[184]);
        let eq27_e1835_d_n2: f64 = (eq27_e1833_d_n2 * s.v[184]);
        let eq27_e1835_d_n3: f64 = (eq27_e1833_d_n3 * s.v[184]);
        let eq27_e1835_d_n4: f64 = (eq27_e1833_d_n4 * s.v[184]);
        let eq27_e1835_d_n5: f64 = (eq27_e1833_d_n5 * s.v[184]);
        let eq27_e1835_d_n6: f64 = (eq27_e1833_d_n6 * s.v[184]);
        let eq27_e1835_d_n7: f64 = (eq27_e1833_d_n7 * s.v[184]);
        let eq27_e1835_d_n8: f64 = (eq27_e1833_d_n8 * s.v[184]);
        let eq27_e1835_d_n9: f64 = (eq27_e1833_d_n9 * s.v[184]);
        let eq27_e1835_d_n10: f64 = (eq27_e1833_d_n10 * s.v[184]);
        let eq27_e1835_d_n11: f64 = (eq27_e1833_d_n11 * s.v[184]);
        let eq27_e1835_d_n12: f64 = (eq27_e1833_d_n12 * s.v[184]);
        let eq27_e1835_d_n13: f64 = (eq27_e1833_d_n13 * s.v[184]);
        let eq27_e1837: f64 = (eq27_e1835 * (nv12 - 0.0));
        let eq27_e1837_d_n0: f64 = (eq27_e1835_d_n0 * (nv12 - 0.0));
        let eq27_e1837_d_n1: f64 = (eq27_e1835_d_n1 * (nv12 - 0.0));
        let eq27_e1837_d_n2: f64 = (eq27_e1835_d_n2 * (nv12 - 0.0));
        let eq27_e1837_d_n3: f64 = (eq27_e1835_d_n3 * (nv12 - 0.0));
        let eq27_e1837_d_n4: f64 = (eq27_e1835_d_n4 * (nv12 - 0.0));
        let eq27_e1837_d_n5: f64 = (eq27_e1835_d_n5 * (nv12 - 0.0));
        let eq27_e1837_d_n6: f64 = (eq27_e1835_d_n6 * (nv12 - 0.0));
        let eq27_e1837_d_n7: f64 = (eq27_e1835_d_n7 * (nv12 - 0.0));
        let eq27_e1837_d_n8: f64 = (eq27_e1835_d_n8 * (nv12 - 0.0));
        let eq27_e1837_d_n9: f64 = (eq27_e1835_d_n9 * (nv12 - 0.0));
        let eq27_e1837_d_n10: f64 = (eq27_e1835_d_n10 * (nv12 - 0.0));
        let eq27_e1837_d_n11: f64 = (eq27_e1835_d_n11 * (nv12 - 0.0));
        let eq27_e1837_d_n12: f64 = ((eq27_e1835_d_n12 * (nv12 - 0.0)) + eq27_e1835);
        let eq27_e1837_d_n13: f64 = (eq27_e1835_d_n13 * (nv12 - 0.0));
        let eq27_e1838: f64 = (0.5 * eq27_e1837);
        let eq27_e1838_d_n0: f64 = (0.5 * eq27_e1837_d_n0);
        let eq27_e1838_d_n1: f64 = (0.5 * eq27_e1837_d_n1);
        let eq27_e1838_d_n2: f64 = (0.5 * eq27_e1837_d_n2);
        let eq27_e1838_d_n3: f64 = (0.5 * eq27_e1837_d_n3);
        let eq27_e1838_d_n4: f64 = (0.5 * eq27_e1837_d_n4);
        let eq27_e1838_d_n5: f64 = (0.5 * eq27_e1837_d_n5);
        let eq27_e1838_d_n6: f64 = (0.5 * eq27_e1837_d_n6);
        let eq27_e1838_d_n7: f64 = (0.5 * eq27_e1837_d_n7);
        let eq27_e1838_d_n8: f64 = (0.5 * eq27_e1837_d_n8);
        let eq27_e1838_d_n9: f64 = (0.5 * eq27_e1837_d_n9);
        let eq27_e1838_d_n10: f64 = (0.5 * eq27_e1837_d_n10);
        let eq27_e1838_d_n11: f64 = (0.5 * eq27_e1837_d_n11);
        let eq27_e1838_d_n12: f64 = (0.5 * eq27_e1837_d_n12);
        let eq27_e1838_d_n13: f64 = (0.5 * eq27_e1837_d_n13);
        let eq27_e1839_q: f64 = eq27_e1838;
        (eq27_e1838, eq27_e1838_d_n0, eq27_e1838_d_n1, eq27_e1838_d_n2, eq27_e1838_d_n3, eq27_e1838_d_n4, eq27_e1838_d_n5, eq27_e1838_d_n6, eq27_e1838_d_n7, eq27_e1838_d_n8, eq27_e1838_d_n9, eq27_e1838_d_n10, eq27_e1838_d_n11, eq27_e1838_d_n12, eq27_e1838_d_n13, eq27_e1839_q, eq27_e1838_d_n0, eq27_e1838_d_n1, eq27_e1838_d_n2, eq27_e1838_d_n3, eq27_e1838_d_n4, eq27_e1838_d_n5, eq27_e1838_d_n6, eq27_e1838_d_n7, eq27_e1838_d_n8, eq27_e1838_d_n9, eq27_e1838_d_n10, eq27_e1838_d_n11, eq27_e1838_d_n12, eq27_e1838_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_reactive_node_derivatives: [f64; 14] = [eq27_e1841_q_d_n0, eq27_e1841_q_d_n1, eq27_e1841_q_d_n2, eq27_e1841_q_d_n3, eq27_e1841_q_d_n4, eq27_e1841_q_d_n5, eq27_e1841_q_d_n6, eq27_e1841_q_d_n7, eq27_e1841_q_d_n8, eq27_e1841_q_d_n9, eq27_e1841_q_d_n10, eq27_e1841_q_d_n11, eq27_e1841_q_d_n12, eq27_e1841_q_d_n13];
        let eq27_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            &nodes,
            &eq27_reactive_node_derivatives,
            &branches,
            &eq27_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_28_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq28_e1868, eq28_e1868_d_n0, eq28_e1868_d_n1, eq28_e1868_d_n2, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_d_n13, eq28_e1868_q, eq28_e1868_q_d_n0, eq28_e1868_q_d_n1, eq28_e1868_q_d_n2, eq28_e1868_q_d_n3, eq28_e1868_q_d_n4, eq28_e1868_q_d_n5, eq28_e1868_q_d_n6, eq28_e1868_q_d_n7, eq28_e1868_q_d_n8, eq28_e1868_q_d_n9, eq28_e1868_q_d_n10, eq28_e1868_q_d_n11, eq28_e1868_q_d_n12, eq28_e1868_q_d_n13,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        let eq28_e1852: f64 = (1.0 - s.v[211]);
        let eq28_e1852_d_n0: f64 = (-s.dn[211][0]);
        let eq28_e1852_d_n1: f64 = (-s.dn[211][1]);
        let eq28_e1852_d_n2: f64 = (-s.dn[211][2]);
        let eq28_e1852_d_n3: f64 = (-s.dn[211][3]);
        let eq28_e1852_d_n4: f64 = (-s.dn[211][4]);
        let eq28_e1852_d_n5: f64 = (-s.dn[211][5]);
        let eq28_e1852_d_n6: f64 = (-s.dn[211][6]);
        let eq28_e1852_d_n7: f64 = (-s.dn[211][7]);
        let eq28_e1852_d_n8: f64 = (-s.dn[211][8]);
        let eq28_e1852_d_n9: f64 = (-s.dn[211][9]);
        let eq28_e1852_d_n10: f64 = (-s.dn[211][10]);
        let eq28_e1852_d_n11: f64 = (-s.dn[211][11]);
        let eq28_e1852_d_n12: f64 = (-s.dn[211][12]);
        let eq28_e1852_d_n13: f64 = (-s.dn[211][13]);
        let eq28_e1854: f64 = (eq28_e1852 * s.v[622]);
        let eq28_e1854_d_n0: f64 = ((eq28_e1852_d_n0 * s.v[622]) + (eq28_e1852 * s.dn[622][0]));
        let eq28_e1854_d_n1: f64 = ((eq28_e1852_d_n1 * s.v[622]) + (eq28_e1852 * s.dn[622][1]));
        let eq28_e1854_d_n2: f64 = ((eq28_e1852_d_n2 * s.v[622]) + (eq28_e1852 * s.dn[622][2]));
        let eq28_e1854_d_n3: f64 = ((eq28_e1852_d_n3 * s.v[622]) + (eq28_e1852 * s.dn[622][3]));
        let eq28_e1854_d_n4: f64 = ((eq28_e1852_d_n4 * s.v[622]) + (eq28_e1852 * s.dn[622][4]));
        let eq28_e1854_d_n5: f64 = ((eq28_e1852_d_n5 * s.v[622]) + (eq28_e1852 * s.dn[622][5]));
        let eq28_e1854_d_n6: f64 = ((eq28_e1852_d_n6 * s.v[622]) + (eq28_e1852 * s.dn[622][6]));
        let eq28_e1854_d_n7: f64 = ((eq28_e1852_d_n7 * s.v[622]) + (eq28_e1852 * s.dn[622][7]));
        let eq28_e1854_d_n8: f64 = ((eq28_e1852_d_n8 * s.v[622]) + (eq28_e1852 * s.dn[622][8]));
        let eq28_e1854_d_n9: f64 = ((eq28_e1852_d_n9 * s.v[622]) + (eq28_e1852 * s.dn[622][9]));
        let eq28_e1854_d_n10: f64 = ((eq28_e1852_d_n10 * s.v[622]) + (eq28_e1852 * s.dn[622][10]));
        let eq28_e1854_d_n11: f64 = ((eq28_e1852_d_n11 * s.v[622]) + (eq28_e1852 * s.dn[622][11]));
        let eq28_e1854_d_n12: f64 = ((eq28_e1852_d_n12 * s.v[622]) + (eq28_e1852 * s.dn[622][12]));
        let eq28_e1854_d_n13: f64 = ((eq28_e1852_d_n13 * s.v[622]) + (eq28_e1852 * s.dn[622][13]));
        let eq28_e1856: f64 = (eq28_e1854 * s.v[199]);
        let eq28_e1856_d_n0: f64 = (eq28_e1854_d_n0 * s.v[199]);
        let eq28_e1856_d_n1: f64 = (eq28_e1854_d_n1 * s.v[199]);
        let eq28_e1856_d_n2: f64 = (eq28_e1854_d_n2 * s.v[199]);
        let eq28_e1856_d_n3: f64 = (eq28_e1854_d_n3 * s.v[199]);
        let eq28_e1856_d_n4: f64 = (eq28_e1854_d_n4 * s.v[199]);
        let eq28_e1856_d_n5: f64 = (eq28_e1854_d_n5 * s.v[199]);
        let eq28_e1856_d_n6: f64 = (eq28_e1854_d_n6 * s.v[199]);
        let eq28_e1856_d_n7: f64 = (eq28_e1854_d_n7 * s.v[199]);
        let eq28_e1856_d_n8: f64 = (eq28_e1854_d_n8 * s.v[199]);
        let eq28_e1856_d_n9: f64 = (eq28_e1854_d_n9 * s.v[199]);
        let eq28_e1856_d_n10: f64 = (eq28_e1854_d_n10 * s.v[199]);
        let eq28_e1856_d_n11: f64 = (eq28_e1854_d_n11 * s.v[199]);
        let eq28_e1856_d_n12: f64 = (eq28_e1854_d_n12 * s.v[199]);
        let eq28_e1856_d_n13: f64 = (eq28_e1854_d_n13 * s.v[199]);
        let eq28_e1858: f64 = (eq28_e1856 * s.v[183]);
        let eq28_e1858_d_n0: f64 = (eq28_e1856_d_n0 * s.v[183]);
        let eq28_e1858_d_n1: f64 = (eq28_e1856_d_n1 * s.v[183]);
        let eq28_e1858_d_n2: f64 = (eq28_e1856_d_n2 * s.v[183]);
        let eq28_e1858_d_n3: f64 = (eq28_e1856_d_n3 * s.v[183]);
        let eq28_e1858_d_n4: f64 = (eq28_e1856_d_n4 * s.v[183]);
        let eq28_e1858_d_n5: f64 = (eq28_e1856_d_n5 * s.v[183]);
        let eq28_e1858_d_n6: f64 = (eq28_e1856_d_n6 * s.v[183]);
        let eq28_e1858_d_n7: f64 = (eq28_e1856_d_n7 * s.v[183]);
        let eq28_e1858_d_n8: f64 = (eq28_e1856_d_n8 * s.v[183]);
        let eq28_e1858_d_n9: f64 = (eq28_e1856_d_n9 * s.v[183]);
        let eq28_e1858_d_n10: f64 = (eq28_e1856_d_n10 * s.v[183]);
        let eq28_e1858_d_n11: f64 = (eq28_e1856_d_n11 * s.v[183]);
        let eq28_e1858_d_n12: f64 = (eq28_e1856_d_n12 * s.v[183]);
        let eq28_e1858_d_n13: f64 = (eq28_e1856_d_n13 * s.v[183]);
        let eq28_e1860: f64 = (eq28_e1858 * p.p2);
        let eq28_e1860_d_n0: f64 = (eq28_e1858_d_n0 * p.p2);
        let eq28_e1860_d_n1: f64 = (eq28_e1858_d_n1 * p.p2);
        let eq28_e1860_d_n2: f64 = (eq28_e1858_d_n2 * p.p2);
        let eq28_e1860_d_n3: f64 = (eq28_e1858_d_n3 * p.p2);
        let eq28_e1860_d_n4: f64 = (eq28_e1858_d_n4 * p.p2);
        let eq28_e1860_d_n5: f64 = (eq28_e1858_d_n5 * p.p2);
        let eq28_e1860_d_n6: f64 = (eq28_e1858_d_n6 * p.p2);
        let eq28_e1860_d_n7: f64 = (eq28_e1858_d_n7 * p.p2);
        let eq28_e1860_d_n8: f64 = (eq28_e1858_d_n8 * p.p2);
        let eq28_e1860_d_n9: f64 = (eq28_e1858_d_n9 * p.p2);
        let eq28_e1860_d_n10: f64 = (eq28_e1858_d_n10 * p.p2);
        let eq28_e1860_d_n11: f64 = (eq28_e1858_d_n11 * p.p2);
        let eq28_e1860_d_n12: f64 = (eq28_e1858_d_n12 * p.p2);
        let eq28_e1860_d_n13: f64 = (eq28_e1858_d_n13 * p.p2);
        let eq28_e1862: f64 = (eq28_e1860 * s.v[184]);
        let eq28_e1862_d_n0: f64 = (eq28_e1860_d_n0 * s.v[184]);
        let eq28_e1862_d_n1: f64 = (eq28_e1860_d_n1 * s.v[184]);
        let eq28_e1862_d_n2: f64 = (eq28_e1860_d_n2 * s.v[184]);
        let eq28_e1862_d_n3: f64 = (eq28_e1860_d_n3 * s.v[184]);
        let eq28_e1862_d_n4: f64 = (eq28_e1860_d_n4 * s.v[184]);
        let eq28_e1862_d_n5: f64 = (eq28_e1860_d_n5 * s.v[184]);
        let eq28_e1862_d_n6: f64 = (eq28_e1860_d_n6 * s.v[184]);
        let eq28_e1862_d_n7: f64 = (eq28_e1860_d_n7 * s.v[184]);
        let eq28_e1862_d_n8: f64 = (eq28_e1860_d_n8 * s.v[184]);
        let eq28_e1862_d_n9: f64 = (eq28_e1860_d_n9 * s.v[184]);
        let eq28_e1862_d_n10: f64 = (eq28_e1860_d_n10 * s.v[184]);
        let eq28_e1862_d_n11: f64 = (eq28_e1860_d_n11 * s.v[184]);
        let eq28_e1862_d_n12: f64 = (eq28_e1860_d_n12 * s.v[184]);
        let eq28_e1862_d_n13: f64 = (eq28_e1860_d_n13 * s.v[184]);
        let eq28_e1864: f64 = (eq28_e1862 * (nv12 - 0.0));
        let eq28_e1864_d_n0: f64 = (eq28_e1862_d_n0 * (nv12 - 0.0));
        let eq28_e1864_d_n1: f64 = (eq28_e1862_d_n1 * (nv12 - 0.0));
        let eq28_e1864_d_n2: f64 = (eq28_e1862_d_n2 * (nv12 - 0.0));
        let eq28_e1864_d_n3: f64 = (eq28_e1862_d_n3 * (nv12 - 0.0));
        let eq28_e1864_d_n4: f64 = (eq28_e1862_d_n4 * (nv12 - 0.0));
        let eq28_e1864_d_n5: f64 = (eq28_e1862_d_n5 * (nv12 - 0.0));
        let eq28_e1864_d_n6: f64 = (eq28_e1862_d_n6 * (nv12 - 0.0));
        let eq28_e1864_d_n7: f64 = (eq28_e1862_d_n7 * (nv12 - 0.0));
        let eq28_e1864_d_n8: f64 = (eq28_e1862_d_n8 * (nv12 - 0.0));
        let eq28_e1864_d_n9: f64 = (eq28_e1862_d_n9 * (nv12 - 0.0));
        let eq28_e1864_d_n10: f64 = (eq28_e1862_d_n10 * (nv12 - 0.0));
        let eq28_e1864_d_n11: f64 = (eq28_e1862_d_n11 * (nv12 - 0.0));
        let eq28_e1864_d_n12: f64 = ((eq28_e1862_d_n12 * (nv12 - 0.0)) + eq28_e1862);
        let eq28_e1864_d_n13: f64 = (eq28_e1862_d_n13 * (nv12 - 0.0));
        let eq28_e1865: f64 = (0.5 * eq28_e1864);
        let eq28_e1865_d_n0: f64 = (0.5 * eq28_e1864_d_n0);
        let eq28_e1865_d_n1: f64 = (0.5 * eq28_e1864_d_n1);
        let eq28_e1865_d_n2: f64 = (0.5 * eq28_e1864_d_n2);
        let eq28_e1865_d_n3: f64 = (0.5 * eq28_e1864_d_n3);
        let eq28_e1865_d_n4: f64 = (0.5 * eq28_e1864_d_n4);
        let eq28_e1865_d_n5: f64 = (0.5 * eq28_e1864_d_n5);
        let eq28_e1865_d_n6: f64 = (0.5 * eq28_e1864_d_n6);
        let eq28_e1865_d_n7: f64 = (0.5 * eq28_e1864_d_n7);
        let eq28_e1865_d_n8: f64 = (0.5 * eq28_e1864_d_n8);
        let eq28_e1865_d_n9: f64 = (0.5 * eq28_e1864_d_n9);
        let eq28_e1865_d_n10: f64 = (0.5 * eq28_e1864_d_n10);
        let eq28_e1865_d_n11: f64 = (0.5 * eq28_e1864_d_n11);
        let eq28_e1865_d_n12: f64 = (0.5 * eq28_e1864_d_n12);
        let eq28_e1865_d_n13: f64 = (0.5 * eq28_e1864_d_n13);
        let eq28_e1866_q: f64 = eq28_e1865;
        (eq28_e1865, eq28_e1865_d_n0, eq28_e1865_d_n1, eq28_e1865_d_n2, eq28_e1865_d_n3, eq28_e1865_d_n4, eq28_e1865_d_n5, eq28_e1865_d_n6, eq28_e1865_d_n7, eq28_e1865_d_n8, eq28_e1865_d_n9, eq28_e1865_d_n10, eq28_e1865_d_n11, eq28_e1865_d_n12, eq28_e1865_d_n13, eq28_e1866_q, eq28_e1865_d_n0, eq28_e1865_d_n1, eq28_e1865_d_n2, eq28_e1865_d_n3, eq28_e1865_d_n4, eq28_e1865_d_n5, eq28_e1865_d_n6, eq28_e1865_d_n7, eq28_e1865_d_n8, eq28_e1865_d_n9, eq28_e1865_d_n10, eq28_e1865_d_n11, eq28_e1865_d_n12, eq28_e1865_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 14] = [eq28_e1868_q_d_n0, eq28_e1868_q_d_n1, eq28_e1868_q_d_n2, eq28_e1868_q_d_n3, eq28_e1868_q_d_n4, eq28_e1868_q_d_n5, eq28_e1868_q_d_n6, eq28_e1868_q_d_n7, eq28_e1868_q_d_n8, eq28_e1868_q_d_n9, eq28_e1868_q_d_n10, eq28_e1868_q_d_n11, eq28_e1868_q_d_n12, eq28_e1868_q_d_n13];
        let eq28_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &nodes,
            &eq28_reactive_node_derivatives,
            &branches,
            &eq28_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
