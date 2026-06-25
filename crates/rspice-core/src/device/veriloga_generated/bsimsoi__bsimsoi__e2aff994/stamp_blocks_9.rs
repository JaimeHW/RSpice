#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_58_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq58_e2037,) = {
    if (!(s.v[2012] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e2037;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[9]),
            self.multiplicity * (eq58_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_59_block_0(
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq59_e2043, eq59_e2043_d_n0, eq59_e2043_d_n1, eq59_e2043_d_n2, eq59_e2043_d_n3, eq59_e2043_d_n4, eq59_e2043_d_n5, eq59_e2043_d_n6, eq59_e2043_d_n7, eq59_e2043_d_n8, eq59_e2043_d_n9, eq59_e2043_d_n10, eq59_e2043_d_n11, eq59_e2043_d_n12, eq59_e2043_d_n13,) = {
    if (s.v[2016] != 0.0) {
        let eq59_e2041: f64 = ((nv0 - nv6) * s.v[618]);
        let eq59_e2041_d_n0: f64 = (s.v[618] + ((nv0 - nv6) * s.dn[618][0]));
        let eq59_e2041_d_n1: f64 = ((nv0 - nv6) * s.dn[618][1]);
        let eq59_e2041_d_n2: f64 = ((nv0 - nv6) * s.dn[618][2]);
        let eq59_e2041_d_n3: f64 = ((nv0 - nv6) * s.dn[618][3]);
        let eq59_e2041_d_n4: f64 = ((nv0 - nv6) * s.dn[618][4]);
        let eq59_e2041_d_n5: f64 = ((nv0 - nv6) * s.dn[618][5]);
        let eq59_e2041_d_n6: f64 = ((-s.v[618]) + ((nv0 - nv6) * s.dn[618][6]));
        let eq59_e2041_d_n7: f64 = ((nv0 - nv6) * s.dn[618][7]);
        let eq59_e2041_d_n8: f64 = ((nv0 - nv6) * s.dn[618][8]);
        let eq59_e2041_d_n9: f64 = ((nv0 - nv6) * s.dn[618][9]);
        let eq59_e2041_d_n10: f64 = ((nv0 - nv6) * s.dn[618][10]);
        let eq59_e2041_d_n11: f64 = ((nv0 - nv6) * s.dn[618][11]);
        let eq59_e2041_d_n12: f64 = ((nv0 - nv6) * s.dn[618][12]);
        let eq59_e2041_d_n13: f64 = ((nv0 - nv6) * s.dn[618][13]);
        (eq59_e2041, eq59_e2041_d_n0, eq59_e2041_d_n1, eq59_e2041_d_n2, eq59_e2041_d_n3, eq59_e2041_d_n4, eq59_e2041_d_n5, eq59_e2041_d_n6, eq59_e2041_d_n7, eq59_e2041_d_n8, eq59_e2041_d_n9, eq59_e2041_d_n10, eq59_e2041_d_n11, eq59_e2041_d_n12, eq59_e2041_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e2043;
        let eq59_node_derivatives: [f64; 14] = [eq59_e2043_d_n0, eq59_e2043_d_n1, eq59_e2043_d_n2, eq59_e2043_d_n3, eq59_e2043_d_n4, eq59_e2043_d_n5, eq59_e2043_d_n6, eq59_e2043_d_n7, eq59_e2043_d_n8, eq59_e2043_d_n9, eq59_e2043_d_n10, eq59_e2043_d_n11, eq59_e2043_d_n12, eq59_e2043_d_n13];
        let eq59_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[6]),
            self.multiplicity * (eq59_value),
            &nodes,
            &eq59_node_derivatives,
            &branches,
            &eq59_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_60_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq60_e2048,) = {
    if (!(s.v[2016] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2048;
        stamper.stamp_potential(
            branches[3],
            eq60_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_61_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq61_e2056,) = {
    if (s.v[2017] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e2056;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[6]),
            self.multiplicity * (eq61_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_62_block_0(
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq62_e2062, eq62_e2062_d_n0, eq62_e2062_d_n1, eq62_e2062_d_n2, eq62_e2062_d_n3, eq62_e2062_d_n4, eq62_e2062_d_n5, eq62_e2062_d_n6, eq62_e2062_d_n7, eq62_e2062_d_n8, eq62_e2062_d_n9, eq62_e2062_d_n10, eq62_e2062_d_n11, eq62_e2062_d_n12, eq62_e2062_d_n13,) = {
    if (s.v[2018] != 0.0) {
        let eq62_e2060: f64 = ((nv2 - nv7) * s.v[617]);
        let eq62_e2060_d_n0: f64 = ((nv2 - nv7) * s.dn[617][0]);
        let eq62_e2060_d_n1: f64 = ((nv2 - nv7) * s.dn[617][1]);
        let eq62_e2060_d_n2: f64 = (s.v[617] + ((nv2 - nv7) * s.dn[617][2]));
        let eq62_e2060_d_n3: f64 = ((nv2 - nv7) * s.dn[617][3]);
        let eq62_e2060_d_n4: f64 = ((nv2 - nv7) * s.dn[617][4]);
        let eq62_e2060_d_n5: f64 = ((nv2 - nv7) * s.dn[617][5]);
        let eq62_e2060_d_n6: f64 = ((nv2 - nv7) * s.dn[617][6]);
        let eq62_e2060_d_n7: f64 = ((-s.v[617]) + ((nv2 - nv7) * s.dn[617][7]));
        let eq62_e2060_d_n8: f64 = ((nv2 - nv7) * s.dn[617][8]);
        let eq62_e2060_d_n9: f64 = ((nv2 - nv7) * s.dn[617][9]);
        let eq62_e2060_d_n10: f64 = ((nv2 - nv7) * s.dn[617][10]);
        let eq62_e2060_d_n11: f64 = ((nv2 - nv7) * s.dn[617][11]);
        let eq62_e2060_d_n12: f64 = ((nv2 - nv7) * s.dn[617][12]);
        let eq62_e2060_d_n13: f64 = ((nv2 - nv7) * s.dn[617][13]);
        (eq62_e2060, eq62_e2060_d_n0, eq62_e2060_d_n1, eq62_e2060_d_n2, eq62_e2060_d_n3, eq62_e2060_d_n4, eq62_e2060_d_n5, eq62_e2060_d_n6, eq62_e2060_d_n7, eq62_e2060_d_n8, eq62_e2060_d_n9, eq62_e2060_d_n10, eq62_e2060_d_n11, eq62_e2060_d_n12, eq62_e2060_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2062;
        let eq62_node_derivatives: [f64; 14] = [eq62_e2062_d_n0, eq62_e2062_d_n1, eq62_e2062_d_n2, eq62_e2062_d_n3, eq62_e2062_d_n4, eq62_e2062_d_n5, eq62_e2062_d_n6, eq62_e2062_d_n7, eq62_e2062_d_n8, eq62_e2062_d_n9, eq62_e2062_d_n10, eq62_e2062_d_n11, eq62_e2062_d_n12, eq62_e2062_d_n13];
        let eq62_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            self.multiplicity * (eq62_value),
            &nodes,
            &eq62_node_derivatives,
            &branches,
            &eq62_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_63_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq63_e2067,) = {
    if (!(s.v[2018] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e2067;
        stamper.stamp_potential(
            branches[4],
            eq63_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_64_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq64_e2075,) = {
    if (s.v[2019] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e2075;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[7]),
            self.multiplicity * (eq64_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_65_block_0(
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
        let (eq65_e2081, eq65_e2081_d_n0, eq65_e2081_d_n1, eq65_e2081_d_n2, eq65_e2081_d_n3, eq65_e2081_d_n4, eq65_e2081_d_n5, eq65_e2081_d_n6, eq65_e2081_d_n7, eq65_e2081_d_n8, eq65_e2081_d_n9, eq65_e2081_d_n10, eq65_e2081_d_n11, eq65_e2081_d_n12, eq65_e2081_d_n13,) = {
    if (s.v[2020] != 0.0) {
        let eq65_e2079: f64 = ((nv9 - nv8) * s.v[467]);
        let eq65_e2079_d_n0: f64 = ((nv9 - nv8) * s.dn[467][0]);
        let eq65_e2079_d_n1: f64 = ((nv9 - nv8) * s.dn[467][1]);
        let eq65_e2079_d_n2: f64 = ((nv9 - nv8) * s.dn[467][2]);
        let eq65_e2079_d_n3: f64 = ((nv9 - nv8) * s.dn[467][3]);
        let eq65_e2079_d_n4: f64 = ((nv9 - nv8) * s.dn[467][4]);
        let eq65_e2079_d_n5: f64 = ((nv9 - nv8) * s.dn[467][5]);
        let eq65_e2079_d_n6: f64 = ((nv9 - nv8) * s.dn[467][6]);
        let eq65_e2079_d_n7: f64 = ((nv9 - nv8) * s.dn[467][7]);
        let eq65_e2079_d_n8: f64 = ((-s.v[467]) + ((nv9 - nv8) * s.dn[467][8]));
        let eq65_e2079_d_n9: f64 = (s.v[467] + ((nv9 - nv8) * s.dn[467][9]));
        let eq65_e2079_d_n10: f64 = ((nv9 - nv8) * s.dn[467][10]);
        let eq65_e2079_d_n11: f64 = ((nv9 - nv8) * s.dn[467][11]);
        let eq65_e2079_d_n12: f64 = ((nv9 - nv8) * s.dn[467][12]);
        let eq65_e2079_d_n13: f64 = ((nv9 - nv8) * s.dn[467][13]);
        (eq65_e2079, eq65_e2079_d_n0, eq65_e2079_d_n1, eq65_e2079_d_n2, eq65_e2079_d_n3, eq65_e2079_d_n4, eq65_e2079_d_n5, eq65_e2079_d_n6, eq65_e2079_d_n7, eq65_e2079_d_n8, eq65_e2079_d_n9, eq65_e2079_d_n10, eq65_e2079_d_n11, eq65_e2079_d_n12, eq65_e2079_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e2081;
        let eq65_node_derivatives: [f64; 14] = [eq65_e2081_d_n0, eq65_e2081_d_n1, eq65_e2081_d_n2, eq65_e2081_d_n3, eq65_e2081_d_n4, eq65_e2081_d_n5, eq65_e2081_d_n6, eq65_e2081_d_n7, eq65_e2081_d_n8, eq65_e2081_d_n9, eq65_e2081_d_n10, eq65_e2081_d_n11, eq65_e2081_d_n12, eq65_e2081_d_n13];
        let eq65_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq65_value),
            &nodes,
            &eq65_node_derivatives,
            &branches,
            &eq65_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_66_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq66_e2086,) = {
    if (!(s.v[2020] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e2086;
        stamper.stamp_potential(
            branches[5],
            eq66_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_67_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n1, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11, eq67_e2103_d_n12, eq67_e2103_d_n13,) = {
    if (((s.v[2021] != 0.0) && (s.v[2024] != 0.0)) && (s.v[2025] != 0.0)) {
        let eq67_e2094: f64 = (s.v[634] * s.v[1015]);
        let eq67_e2094_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq67_e2094_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq67_e2094_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq67_e2094_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq67_e2094_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq67_e2094_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq67_e2094_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq67_e2094_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq67_e2094_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq67_e2094_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq67_e2094_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq67_e2094_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq67_e2094_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq67_e2094_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq67_e2097: f64 = (s.v[634] * s.v[1016]);
        let eq67_e2097_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq67_e2097_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq67_e2097_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq67_e2097_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq67_e2097_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq67_e2097_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq67_e2097_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq67_e2097_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq67_e2097_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq67_e2097_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq67_e2097_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq67_e2097_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq67_e2097_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq67_e2097_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq67_e2098: f64 = self.eval_ddt(18, eq67_e2097);
        let eq67_e2098_d_n0: f64 = self.ddt_jacobian(eq67_e2097_d_n0);
        let eq67_e2098_d_n1: f64 = self.ddt_jacobian(eq67_e2097_d_n1);
        let eq67_e2098_d_n2: f64 = self.ddt_jacobian(eq67_e2097_d_n2);
        let eq67_e2098_d_n3: f64 = self.ddt_jacobian(eq67_e2097_d_n3);
        let eq67_e2098_d_n4: f64 = self.ddt_jacobian(eq67_e2097_d_n4);
        let eq67_e2098_d_n5: f64 = self.ddt_jacobian(eq67_e2097_d_n5);
        let eq67_e2098_d_n6: f64 = self.ddt_jacobian(eq67_e2097_d_n6);
        let eq67_e2098_d_n7: f64 = self.ddt_jacobian(eq67_e2097_d_n7);
        let eq67_e2098_d_n8: f64 = self.ddt_jacobian(eq67_e2097_d_n8);
        let eq67_e2098_d_n9: f64 = self.ddt_jacobian(eq67_e2097_d_n9);
        let eq67_e2098_d_n10: f64 = self.ddt_jacobian(eq67_e2097_d_n10);
        let eq67_e2098_d_n11: f64 = self.ddt_jacobian(eq67_e2097_d_n11);
        let eq67_e2098_d_n12: f64 = self.ddt_jacobian(eq67_e2097_d_n12);
        let eq67_e2098_d_n13: f64 = self.ddt_jacobian(eq67_e2097_d_n13);
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2098);
        let eq67_e2099_d_n0: f64 = (eq67_e2094_d_n0 + eq67_e2098_d_n0);
        let eq67_e2099_d_n1: f64 = (eq67_e2094_d_n1 + eq67_e2098_d_n1);
        let eq67_e2099_d_n2: f64 = (eq67_e2094_d_n2 + eq67_e2098_d_n2);
        let eq67_e2099_d_n3: f64 = (eq67_e2094_d_n3 + eq67_e2098_d_n3);
        let eq67_e2099_d_n4: f64 = (eq67_e2094_d_n4 + eq67_e2098_d_n4);
        let eq67_e2099_d_n5: f64 = (eq67_e2094_d_n5 + eq67_e2098_d_n5);
        let eq67_e2099_d_n6: f64 = (eq67_e2094_d_n6 + eq67_e2098_d_n6);
        let eq67_e2099_d_n7: f64 = (eq67_e2094_d_n7 + eq67_e2098_d_n7);
        let eq67_e2099_d_n8: f64 = (eq67_e2094_d_n8 + eq67_e2098_d_n8);
        let eq67_e2099_d_n9: f64 = (eq67_e2094_d_n9 + eq67_e2098_d_n9);
        let eq67_e2099_d_n10: f64 = (eq67_e2094_d_n10 + eq67_e2098_d_n10);
        let eq67_e2099_d_n11: f64 = (eq67_e2094_d_n11 + eq67_e2098_d_n11);
        let eq67_e2099_d_n12: f64 = (eq67_e2094_d_n12 + eq67_e2098_d_n12);
        let eq67_e2099_d_n13: f64 = (eq67_e2094_d_n13 + eq67_e2098_d_n13);
        let eq67_e2101: f64 = (eq67_e2099 - s.v[1017]);
        let eq67_e2101_d_n0: f64 = (eq67_e2099_d_n0 - s.dn[1017][0]);
        let eq67_e2101_d_n1: f64 = (eq67_e2099_d_n1 - s.dn[1017][1]);
        let eq67_e2101_d_n2: f64 = (eq67_e2099_d_n2 - s.dn[1017][2]);
        let eq67_e2101_d_n3: f64 = (eq67_e2099_d_n3 - s.dn[1017][3]);
        let eq67_e2101_d_n4: f64 = (eq67_e2099_d_n4 - s.dn[1017][4]);
        let eq67_e2101_d_n5: f64 = (eq67_e2099_d_n5 - s.dn[1017][5]);
        let eq67_e2101_d_n6: f64 = (eq67_e2099_d_n6 - s.dn[1017][6]);
        let eq67_e2101_d_n7: f64 = (eq67_e2099_d_n7 - s.dn[1017][7]);
        let eq67_e2101_d_n8: f64 = (eq67_e2099_d_n8 - s.dn[1017][8]);
        let eq67_e2101_d_n9: f64 = (eq67_e2099_d_n9 - s.dn[1017][9]);
        let eq67_e2101_d_n10: f64 = (eq67_e2099_d_n10 - s.dn[1017][10]);
        let eq67_e2101_d_n11: f64 = (eq67_e2099_d_n11 - s.dn[1017][11]);
        let eq67_e2101_d_n12: f64 = (eq67_e2099_d_n12 - s.dn[1017][12]);
        let eq67_e2101_d_n13: f64 = (eq67_e2099_d_n13 - s.dn[1017][13]);
        (eq67_e2101, eq67_e2101_d_n0, eq67_e2101_d_n1, eq67_e2101_d_n2, eq67_e2101_d_n3, eq67_e2101_d_n4, eq67_e2101_d_n5, eq67_e2101_d_n6, eq67_e2101_d_n7, eq67_e2101_d_n8, eq67_e2101_d_n9, eq67_e2101_d_n10, eq67_e2101_d_n11, eq67_e2101_d_n12, eq67_e2101_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2103;
        let eq67_node_derivatives: [f64; 14] = [eq67_e2103_d_n0, eq67_e2103_d_n1, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11, eq67_e2103_d_n12, eq67_e2103_d_n13];
        let eq67_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq67_value),
            &nodes,
            &eq67_node_derivatives,
            &branches,
            &eq67_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_68_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n1, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11, eq68_e2121_d_n12, eq68_e2121_d_n13,) = {
    if (((s.v[2021] != 0.0) && (s.v[2024] != 0.0)) && (!(s.v[2025] != 0.0))) {
        let eq68_e2112: f64 = (s.v[634] * s.v[1015]);
        let eq68_e2112_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq68_e2112_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq68_e2112_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq68_e2112_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq68_e2112_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq68_e2112_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq68_e2112_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq68_e2112_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq68_e2112_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq68_e2112_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq68_e2112_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq68_e2112_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq68_e2112_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq68_e2112_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq68_e2115: f64 = (s.v[634] * s.v[1016]);
        let eq68_e2115_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq68_e2115_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq68_e2115_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq68_e2115_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq68_e2115_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq68_e2115_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq68_e2115_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq68_e2115_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq68_e2115_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq68_e2115_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq68_e2115_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq68_e2115_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq68_e2115_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq68_e2115_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq68_e2116: f64 = self.eval_ddt(19, eq68_e2115);
        let eq68_e2116_d_n0: f64 = self.ddt_jacobian(eq68_e2115_d_n0);
        let eq68_e2116_d_n1: f64 = self.ddt_jacobian(eq68_e2115_d_n1);
        let eq68_e2116_d_n2: f64 = self.ddt_jacobian(eq68_e2115_d_n2);
        let eq68_e2116_d_n3: f64 = self.ddt_jacobian(eq68_e2115_d_n3);
        let eq68_e2116_d_n4: f64 = self.ddt_jacobian(eq68_e2115_d_n4);
        let eq68_e2116_d_n5: f64 = self.ddt_jacobian(eq68_e2115_d_n5);
        let eq68_e2116_d_n6: f64 = self.ddt_jacobian(eq68_e2115_d_n6);
        let eq68_e2116_d_n7: f64 = self.ddt_jacobian(eq68_e2115_d_n7);
        let eq68_e2116_d_n8: f64 = self.ddt_jacobian(eq68_e2115_d_n8);
        let eq68_e2116_d_n9: f64 = self.ddt_jacobian(eq68_e2115_d_n9);
        let eq68_e2116_d_n10: f64 = self.ddt_jacobian(eq68_e2115_d_n10);
        let eq68_e2116_d_n11: f64 = self.ddt_jacobian(eq68_e2115_d_n11);
        let eq68_e2116_d_n12: f64 = self.ddt_jacobian(eq68_e2115_d_n12);
        let eq68_e2116_d_n13: f64 = self.ddt_jacobian(eq68_e2115_d_n13);
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2116);
        let eq68_e2117_d_n0: f64 = (eq68_e2112_d_n0 + eq68_e2116_d_n0);
        let eq68_e2117_d_n1: f64 = (eq68_e2112_d_n1 + eq68_e2116_d_n1);
        let eq68_e2117_d_n2: f64 = (eq68_e2112_d_n2 + eq68_e2116_d_n2);
        let eq68_e2117_d_n3: f64 = (eq68_e2112_d_n3 + eq68_e2116_d_n3);
        let eq68_e2117_d_n4: f64 = (eq68_e2112_d_n4 + eq68_e2116_d_n4);
        let eq68_e2117_d_n5: f64 = (eq68_e2112_d_n5 + eq68_e2116_d_n5);
        let eq68_e2117_d_n6: f64 = (eq68_e2112_d_n6 + eq68_e2116_d_n6);
        let eq68_e2117_d_n7: f64 = (eq68_e2112_d_n7 + eq68_e2116_d_n7);
        let eq68_e2117_d_n8: f64 = (eq68_e2112_d_n8 + eq68_e2116_d_n8);
        let eq68_e2117_d_n9: f64 = (eq68_e2112_d_n9 + eq68_e2116_d_n9);
        let eq68_e2117_d_n10: f64 = (eq68_e2112_d_n10 + eq68_e2116_d_n10);
        let eq68_e2117_d_n11: f64 = (eq68_e2112_d_n11 + eq68_e2116_d_n11);
        let eq68_e2117_d_n12: f64 = (eq68_e2112_d_n12 + eq68_e2116_d_n12);
        let eq68_e2117_d_n13: f64 = (eq68_e2112_d_n13 + eq68_e2116_d_n13);
        let eq68_e2119: f64 = (eq68_e2117 - s.v[1017]);
        let eq68_e2119_d_n0: f64 = (eq68_e2117_d_n0 - s.dn[1017][0]);
        let eq68_e2119_d_n1: f64 = (eq68_e2117_d_n1 - s.dn[1017][1]);
        let eq68_e2119_d_n2: f64 = (eq68_e2117_d_n2 - s.dn[1017][2]);
        let eq68_e2119_d_n3: f64 = (eq68_e2117_d_n3 - s.dn[1017][3]);
        let eq68_e2119_d_n4: f64 = (eq68_e2117_d_n4 - s.dn[1017][4]);
        let eq68_e2119_d_n5: f64 = (eq68_e2117_d_n5 - s.dn[1017][5]);
        let eq68_e2119_d_n6: f64 = (eq68_e2117_d_n6 - s.dn[1017][6]);
        let eq68_e2119_d_n7: f64 = (eq68_e2117_d_n7 - s.dn[1017][7]);
        let eq68_e2119_d_n8: f64 = (eq68_e2117_d_n8 - s.dn[1017][8]);
        let eq68_e2119_d_n9: f64 = (eq68_e2117_d_n9 - s.dn[1017][9]);
        let eq68_e2119_d_n10: f64 = (eq68_e2117_d_n10 - s.dn[1017][10]);
        let eq68_e2119_d_n11: f64 = (eq68_e2117_d_n11 - s.dn[1017][11]);
        let eq68_e2119_d_n12: f64 = (eq68_e2117_d_n12 - s.dn[1017][12]);
        let eq68_e2119_d_n13: f64 = (eq68_e2117_d_n13 - s.dn[1017][13]);
        (eq68_e2119, eq68_e2119_d_n0, eq68_e2119_d_n1, eq68_e2119_d_n2, eq68_e2119_d_n3, eq68_e2119_d_n4, eq68_e2119_d_n5, eq68_e2119_d_n6, eq68_e2119_d_n7, eq68_e2119_d_n8, eq68_e2119_d_n9, eq68_e2119_d_n10, eq68_e2119_d_n11, eq68_e2119_d_n12, eq68_e2119_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e2121;
        let eq68_node_derivatives: [f64; 14] = [eq68_e2121_d_n0, eq68_e2121_d_n1, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11, eq68_e2121_d_n12, eq68_e2121_d_n13];
        let eq68_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq68_value),
            &nodes,
            &eq68_node_derivatives,
            &branches,
            &eq68_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_69_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n1, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11, eq69_e2137_d_n12, eq69_e2137_d_n13,) = {
    if ((s.v[2021] != 0.0) && (!(s.v[2024] != 0.0))) {
        let eq69_e2128: f64 = (s.v[634] * s.v[1015]);
        let eq69_e2128_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq69_e2128_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq69_e2128_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq69_e2128_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq69_e2128_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq69_e2128_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq69_e2128_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq69_e2128_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq69_e2128_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq69_e2128_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq69_e2128_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq69_e2128_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq69_e2128_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq69_e2128_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq69_e2131: f64 = (s.v[634] * s.v[1016]);
        let eq69_e2131_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq69_e2131_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq69_e2131_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq69_e2131_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq69_e2131_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq69_e2131_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq69_e2131_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq69_e2131_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq69_e2131_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq69_e2131_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq69_e2131_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq69_e2131_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq69_e2131_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq69_e2131_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq69_e2132: f64 = self.eval_ddt(20, eq69_e2131);
        let eq69_e2132_d_n0: f64 = self.ddt_jacobian(eq69_e2131_d_n0);
        let eq69_e2132_d_n1: f64 = self.ddt_jacobian(eq69_e2131_d_n1);
        let eq69_e2132_d_n2: f64 = self.ddt_jacobian(eq69_e2131_d_n2);
        let eq69_e2132_d_n3: f64 = self.ddt_jacobian(eq69_e2131_d_n3);
        let eq69_e2132_d_n4: f64 = self.ddt_jacobian(eq69_e2131_d_n4);
        let eq69_e2132_d_n5: f64 = self.ddt_jacobian(eq69_e2131_d_n5);
        let eq69_e2132_d_n6: f64 = self.ddt_jacobian(eq69_e2131_d_n6);
        let eq69_e2132_d_n7: f64 = self.ddt_jacobian(eq69_e2131_d_n7);
        let eq69_e2132_d_n8: f64 = self.ddt_jacobian(eq69_e2131_d_n8);
        let eq69_e2132_d_n9: f64 = self.ddt_jacobian(eq69_e2131_d_n9);
        let eq69_e2132_d_n10: f64 = self.ddt_jacobian(eq69_e2131_d_n10);
        let eq69_e2132_d_n11: f64 = self.ddt_jacobian(eq69_e2131_d_n11);
        let eq69_e2132_d_n12: f64 = self.ddt_jacobian(eq69_e2131_d_n12);
        let eq69_e2132_d_n13: f64 = self.ddt_jacobian(eq69_e2131_d_n13);
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2132);
        let eq69_e2133_d_n0: f64 = (eq69_e2128_d_n0 + eq69_e2132_d_n0);
        let eq69_e2133_d_n1: f64 = (eq69_e2128_d_n1 + eq69_e2132_d_n1);
        let eq69_e2133_d_n2: f64 = (eq69_e2128_d_n2 + eq69_e2132_d_n2);
        let eq69_e2133_d_n3: f64 = (eq69_e2128_d_n3 + eq69_e2132_d_n3);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + eq69_e2132_d_n4);
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + eq69_e2132_d_n5);
        let eq69_e2133_d_n6: f64 = (eq69_e2128_d_n6 + eq69_e2132_d_n6);
        let eq69_e2133_d_n7: f64 = (eq69_e2128_d_n7 + eq69_e2132_d_n7);
        let eq69_e2133_d_n8: f64 = (eq69_e2128_d_n8 + eq69_e2132_d_n8);
        let eq69_e2133_d_n9: f64 = (eq69_e2128_d_n9 + eq69_e2132_d_n9);
        let eq69_e2133_d_n10: f64 = (eq69_e2128_d_n10 + eq69_e2132_d_n10);
        let eq69_e2133_d_n11: f64 = (eq69_e2128_d_n11 + eq69_e2132_d_n11);
        let eq69_e2133_d_n12: f64 = (eq69_e2128_d_n12 + eq69_e2132_d_n12);
        let eq69_e2133_d_n13: f64 = (eq69_e2128_d_n13 + eq69_e2132_d_n13);
        let eq69_e2135: f64 = (eq69_e2133 - s.v[1017]);
        let eq69_e2135_d_n0: f64 = (eq69_e2133_d_n0 - s.dn[1017][0]);
        let eq69_e2135_d_n1: f64 = (eq69_e2133_d_n1 - s.dn[1017][1]);
        let eq69_e2135_d_n2: f64 = (eq69_e2133_d_n2 - s.dn[1017][2]);
        let eq69_e2135_d_n3: f64 = (eq69_e2133_d_n3 - s.dn[1017][3]);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - s.dn[1017][4]);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - s.dn[1017][5]);
        let eq69_e2135_d_n6: f64 = (eq69_e2133_d_n6 - s.dn[1017][6]);
        let eq69_e2135_d_n7: f64 = (eq69_e2133_d_n7 - s.dn[1017][7]);
        let eq69_e2135_d_n8: f64 = (eq69_e2133_d_n8 - s.dn[1017][8]);
        let eq69_e2135_d_n9: f64 = (eq69_e2133_d_n9 - s.dn[1017][9]);
        let eq69_e2135_d_n10: f64 = (eq69_e2133_d_n10 - s.dn[1017][10]);
        let eq69_e2135_d_n11: f64 = (eq69_e2133_d_n11 - s.dn[1017][11]);
        let eq69_e2135_d_n12: f64 = (eq69_e2133_d_n12 - s.dn[1017][12]);
        let eq69_e2135_d_n13: f64 = (eq69_e2133_d_n13 - s.dn[1017][13]);
        (eq69_e2135, eq69_e2135_d_n0, eq69_e2135_d_n1, eq69_e2135_d_n2, eq69_e2135_d_n3, eq69_e2135_d_n4, eq69_e2135_d_n5, eq69_e2135_d_n6, eq69_e2135_d_n7, eq69_e2135_d_n8, eq69_e2135_d_n9, eq69_e2135_d_n10, eq69_e2135_d_n11, eq69_e2135_d_n12, eq69_e2135_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2137;
        let eq69_node_derivatives: [f64; 14] = [eq69_e2137_d_n0, eq69_e2137_d_n1, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11, eq69_e2137_d_n12, eq69_e2137_d_n13];
        let eq69_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq69_value),
            &nodes,
            &eq69_node_derivatives,
            &branches,
            &eq69_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_70_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq70_e2146,) = {
    if (((!(s.v[2021] != 0.0)) && (s.v[2026] != 0.0)) && (s.v[2027] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e2146;
        stamper.stamp_potential(
            branches[6],
            eq70_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_71_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq71_e2156,) = {
    if (((!(s.v[2021] != 0.0)) && (s.v[2026] != 0.0)) && (!(s.v[2027] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e2156;
        stamper.stamp_potential(
            branches[7],
            eq71_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_72_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq72_e2164,) = {
    if ((!(s.v[2021] != 0.0)) && (!(s.v[2026] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq72_value: f64 = eq72_e2164;
        stamper.stamp_potential(
            branches[8],
            eq72_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_73_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq73_e2168,) = {
    if (s.v[2028] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e2168;
        stamper.stamp_potential(
            branches[9],
            eq73_value,
            &[
            ],
        );
    }
}
