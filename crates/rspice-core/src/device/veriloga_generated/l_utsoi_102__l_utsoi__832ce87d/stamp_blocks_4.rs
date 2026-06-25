#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq3_e515: f64 = (p.p14 * s.v[362]);
        let eq3_e515_d_n0: f64 = (p.p14 * s.dn[362][0]);
        let eq3_e515_d_n1: f64 = (p.p14 * s.dn[362][1]);
        let eq3_e515_d_n2: f64 = (p.p14 * s.dn[362][2]);
        let eq3_e515_d_n3: f64 = (p.p14 * s.dn[362][3]);
        let eq3_e515_d_n4: f64 = (p.p14 * s.dn[362][4]);
        let eq3_e515_d_n5: f64 = (p.p14 * s.dn[362][5]);
        let eq3_e515_d_n6: f64 = (p.p14 * s.dn[362][6]);
        let eq3_e515_d_n7: f64 = (p.p14 * s.dn[362][7]);
        let eq3_e515_d_n8: f64 = (p.p14 * s.dn[362][8]);
        let eq3_e515_d_n9: f64 = (p.p14 * s.dn[362][9]);
        let eq3_value: f64 = eq3_e515;
        let eq3_node_derivatives: [f64; 10] = [eq3_e515_d_n0, eq3_e515_d_n1, eq3_e515_d_n2, eq3_e515_d_n3, eq3_e515_d_n4, eq3_e515_d_n5, eq3_e515_d_n6, eq3_e515_d_n7, eq3_e515_d_n8, eq3_e515_d_n9];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            self.multiplicity * (eq3_value),
            &nodes,
            &eq3_node_derivatives,
            &branches,
            &eq3_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq4_e518: f64 = (p.p14 * s.v[363]);
        let eq4_e518_d_n0: f64 = (p.p14 * s.dn[363][0]);
        let eq4_e518_d_n1: f64 = (p.p14 * s.dn[363][1]);
        let eq4_e518_d_n2: f64 = (p.p14 * s.dn[363][2]);
        let eq4_e518_d_n3: f64 = (p.p14 * s.dn[363][3]);
        let eq4_e518_d_n4: f64 = (p.p14 * s.dn[363][4]);
        let eq4_e518_d_n5: f64 = (p.p14 * s.dn[363][5]);
        let eq4_e518_d_n6: f64 = (p.p14 * s.dn[363][6]);
        let eq4_e518_d_n7: f64 = (p.p14 * s.dn[363][7]);
        let eq4_e518_d_n8: f64 = (p.p14 * s.dn[363][8]);
        let eq4_e518_d_n9: f64 = (p.p14 * s.dn[363][9]);
        let eq4_value: f64 = eq4_e518;
        let eq4_node_derivatives: [f64; 10] = [eq4_e518_d_n0, eq4_e518_d_n1, eq4_e518_d_n2, eq4_e518_d_n3, eq4_e518_d_n4, eq4_e518_d_n5, eq4_e518_d_n6, eq4_e518_d_n7, eq4_e518_d_n8, eq4_e518_d_n9];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq4_value),
            &nodes,
            &eq4_node_derivatives,
            &branches,
            &eq4_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq5_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq5_value),
            &[
            ],
        );
    }

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
        let eq6_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[8]),
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
        let eq7_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq7_value),
            &[
            ],
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq8_e524: f64 = (p.p31 * s.v[471]);
        let eq8_e526: f64 = (eq8_e524 * (nv7 - nv6));
        let eq8_e526_d_n6: f64 = (-eq8_e524);
        let eq8_value: f64 = eq8_e526;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq8_value),
            &[
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq8_e526_d_n6),
                GeneratedDerivative::node(nodes[7], self.multiplicity * eq8_e524),
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
        let eq9_value: f64 = s.v[1761];
        let eq9_node_derivatives: [f64; 10] = [s.dn[1761][0], s.dn[1761][1], s.dn[1761][2], s.dn[1761][3], s.dn[1761][4], s.dn[1761][5], s.dn[1761][6], s.dn[1761][7], s.dn[1761][8], s.dn[1761][9]];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
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
        let eq10_value: f64 = s.v[1762];
        let eq10_node_derivatives: [f64; 10] = [s.dn[1762][0], s.dn[1762][1], s.dn[1762][2], s.dn[1762][3], s.dn[1762][4], s.dn[1762][5], s.dn[1762][6], s.dn[1762][7], s.dn[1762][8], s.dn[1762][9]];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq11_e538, eq11_e538_d_n0, eq11_e538_d_n1, eq11_e538_d_n2, eq11_e538_d_n3, eq11_e538_d_n4, eq11_e538_d_n5, eq11_e538_d_n6, eq11_e538_d_n7, eq11_e538_d_n8, eq11_e538_d_n9,) = {
    if (s.v[1764] != 0.0) {
        let eq11_e532: f64 = (p.p31 * s.v[13]);
        let eq11_e532_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq11_e532_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq11_e532_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq11_e532_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq11_e532_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq11_e532_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq11_e532_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq11_e532_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq11_e532_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq11_e532_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq11_e534: f64 = (eq11_e532 * s.v[312]);
        let eq11_e534_d_n0: f64 = ((eq11_e532_d_n0 * s.v[312]) + (eq11_e532 * s.dn[312][0]));
        let eq11_e534_d_n1: f64 = ((eq11_e532_d_n1 * s.v[312]) + (eq11_e532 * s.dn[312][1]));
        let eq11_e534_d_n2: f64 = ((eq11_e532_d_n2 * s.v[312]) + (eq11_e532 * s.dn[312][2]));
        let eq11_e534_d_n3: f64 = ((eq11_e532_d_n3 * s.v[312]) + (eq11_e532 * s.dn[312][3]));
        let eq11_e534_d_n4: f64 = ((eq11_e532_d_n4 * s.v[312]) + (eq11_e532 * s.dn[312][4]));
        let eq11_e534_d_n5: f64 = ((eq11_e532_d_n5 * s.v[312]) + (eq11_e532 * s.dn[312][5]));
        let eq11_e534_d_n6: f64 = ((eq11_e532_d_n6 * s.v[312]) + (eq11_e532 * s.dn[312][6]));
        let eq11_e534_d_n7: f64 = ((eq11_e532_d_n7 * s.v[312]) + (eq11_e532 * s.dn[312][7]));
        let eq11_e534_d_n8: f64 = ((eq11_e532_d_n8 * s.v[312]) + (eq11_e532 * s.dn[312][8]));
        let eq11_e534_d_n9: f64 = ((eq11_e532_d_n9 * s.v[312]) + (eq11_e532 * s.dn[312][9]));
        let eq11_e536: f64 = (eq11_e534 * (nv1 - nv9));
        let eq11_e536_d_n0: f64 = (eq11_e534_d_n0 * (nv1 - nv9));
        let eq11_e536_d_n1: f64 = ((eq11_e534_d_n1 * (nv1 - nv9)) + eq11_e534);
        let eq11_e536_d_n2: f64 = (eq11_e534_d_n2 * (nv1 - nv9));
        let eq11_e536_d_n3: f64 = (eq11_e534_d_n3 * (nv1 - nv9));
        let eq11_e536_d_n4: f64 = (eq11_e534_d_n4 * (nv1 - nv9));
        let eq11_e536_d_n5: f64 = (eq11_e534_d_n5 * (nv1 - nv9));
        let eq11_e536_d_n6: f64 = (eq11_e534_d_n6 * (nv1 - nv9));
        let eq11_e536_d_n7: f64 = (eq11_e534_d_n7 * (nv1 - nv9));
        let eq11_e536_d_n8: f64 = (eq11_e534_d_n8 * (nv1 - nv9));
        let eq11_e536_d_n9: f64 = ((eq11_e534_d_n9 * (nv1 - nv9)) + (-eq11_e534));
        (eq11_e536, eq11_e536_d_n0, eq11_e536_d_n1, eq11_e536_d_n2, eq11_e536_d_n3, eq11_e536_d_n4, eq11_e536_d_n5, eq11_e536_d_n6, eq11_e536_d_n7, eq11_e536_d_n8, eq11_e536_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e538;
        let eq11_node_derivatives: [f64; 10] = [eq11_e538_d_n0, eq11_e538_d_n1, eq11_e538_d_n2, eq11_e538_d_n3, eq11_e538_d_n4, eq11_e538_d_n5, eq11_e538_d_n6, eq11_e538_d_n7, eq11_e538_d_n8, eq11_e538_d_n9];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
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
        let (eq12_e548,) = {
    if (s.v[1764] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e548;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[9]),
            self.multiplicity * (eq12_value),
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
        let (eq13_e553,) = {
    if (!(s.v[1764] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e553;
        stamper.stamp_potential(
            branches[0],
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq14_e563, eq14_e563_d_n0, eq14_e563_d_n1, eq14_e563_d_n2, eq14_e563_d_n3, eq14_e563_d_n4, eq14_e563_d_n5, eq14_e563_d_n6, eq14_e563_d_n7, eq14_e563_d_n8, eq14_e563_d_n9,) = {
    if (s.v[1765] != 0.0) {
        let eq14_e557: f64 = (p.p31 * s.v[13]);
        let eq14_e557_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq14_e557_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq14_e557_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq14_e557_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq14_e557_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq14_e557_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq14_e557_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq14_e557_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq14_e557_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq14_e557_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq14_e559: f64 = (eq14_e557 * s.v[316]);
        let eq14_e559_d_n0: f64 = ((eq14_e557_d_n0 * s.v[316]) + (eq14_e557 * s.dn[316][0]));
        let eq14_e559_d_n1: f64 = ((eq14_e557_d_n1 * s.v[316]) + (eq14_e557 * s.dn[316][1]));
        let eq14_e559_d_n2: f64 = ((eq14_e557_d_n2 * s.v[316]) + (eq14_e557 * s.dn[316][2]));
        let eq14_e559_d_n3: f64 = ((eq14_e557_d_n3 * s.v[316]) + (eq14_e557 * s.dn[316][3]));
        let eq14_e559_d_n4: f64 = ((eq14_e557_d_n4 * s.v[316]) + (eq14_e557 * s.dn[316][4]));
        let eq14_e559_d_n5: f64 = ((eq14_e557_d_n5 * s.v[316]) + (eq14_e557 * s.dn[316][5]));
        let eq14_e559_d_n6: f64 = ((eq14_e557_d_n6 * s.v[316]) + (eq14_e557 * s.dn[316][6]));
        let eq14_e559_d_n7: f64 = ((eq14_e557_d_n7 * s.v[316]) + (eq14_e557 * s.dn[316][7]));
        let eq14_e559_d_n8: f64 = ((eq14_e557_d_n8 * s.v[316]) + (eq14_e557 * s.dn[316][8]));
        let eq14_e559_d_n9: f64 = ((eq14_e557_d_n9 * s.v[316]) + (eq14_e557 * s.dn[316][9]));
        let eq14_e561: f64 = (eq14_e559 * (nv2 - nv6));
        let eq14_e561_d_n0: f64 = (eq14_e559_d_n0 * (nv2 - nv6));
        let eq14_e561_d_n1: f64 = (eq14_e559_d_n1 * (nv2 - nv6));
        let eq14_e561_d_n2: f64 = ((eq14_e559_d_n2 * (nv2 - nv6)) + eq14_e559);
        let eq14_e561_d_n3: f64 = (eq14_e559_d_n3 * (nv2 - nv6));
        let eq14_e561_d_n4: f64 = (eq14_e559_d_n4 * (nv2 - nv6));
        let eq14_e561_d_n5: f64 = (eq14_e559_d_n5 * (nv2 - nv6));
        let eq14_e561_d_n6: f64 = ((eq14_e559_d_n6 * (nv2 - nv6)) + (-eq14_e559));
        let eq14_e561_d_n7: f64 = (eq14_e559_d_n7 * (nv2 - nv6));
        let eq14_e561_d_n8: f64 = (eq14_e559_d_n8 * (nv2 - nv6));
        let eq14_e561_d_n9: f64 = (eq14_e559_d_n9 * (nv2 - nv6));
        (eq14_e561, eq14_e561_d_n0, eq14_e561_d_n1, eq14_e561_d_n2, eq14_e561_d_n3, eq14_e561_d_n4, eq14_e561_d_n5, eq14_e561_d_n6, eq14_e561_d_n7, eq14_e561_d_n8, eq14_e561_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e563;
        let eq14_node_derivatives: [f64; 10] = [eq14_e563_d_n0, eq14_e563_d_n1, eq14_e563_d_n2, eq14_e563_d_n3, eq14_e563_d_n4, eq14_e563_d_n5, eq14_e563_d_n6, eq14_e563_d_n7, eq14_e563_d_n8, eq14_e563_d_n9];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[6]),
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
        let (eq15_e573,) = {
    if (s.v[1765] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e573;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[6]),
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
        let (eq16_e578,) = {
    if (!(s.v[1765] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e578;
        stamper.stamp_potential(
            branches[1],
            eq16_value,
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq17_e588, eq17_e588_d_n0, eq17_e588_d_n1, eq17_e588_d_n2, eq17_e588_d_n3, eq17_e588_d_n4, eq17_e588_d_n5, eq17_e588_d_n6, eq17_e588_d_n7, eq17_e588_d_n8, eq17_e588_d_n9,) = {
    if (s.v[1766] != 0.0) {
        let eq17_e582: f64 = (p.p31 * s.v[13]);
        let eq17_e582_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq17_e582_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq17_e582_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq17_e582_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq17_e582_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq17_e582_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq17_e582_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq17_e582_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq17_e582_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq17_e582_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq17_e584: f64 = (eq17_e582 * s.v[320]);
        let eq17_e584_d_n0: f64 = ((eq17_e582_d_n0 * s.v[320]) + (eq17_e582 * s.dn[320][0]));
        let eq17_e584_d_n1: f64 = ((eq17_e582_d_n1 * s.v[320]) + (eq17_e582 * s.dn[320][1]));
        let eq17_e584_d_n2: f64 = ((eq17_e582_d_n2 * s.v[320]) + (eq17_e582 * s.dn[320][2]));
        let eq17_e584_d_n3: f64 = ((eq17_e582_d_n3 * s.v[320]) + (eq17_e582 * s.dn[320][3]));
        let eq17_e584_d_n4: f64 = ((eq17_e582_d_n4 * s.v[320]) + (eq17_e582 * s.dn[320][4]));
        let eq17_e584_d_n5: f64 = ((eq17_e582_d_n5 * s.v[320]) + (eq17_e582 * s.dn[320][5]));
        let eq17_e584_d_n6: f64 = ((eq17_e582_d_n6 * s.v[320]) + (eq17_e582 * s.dn[320][6]));
        let eq17_e584_d_n7: f64 = ((eq17_e582_d_n7 * s.v[320]) + (eq17_e582 * s.dn[320][7]));
        let eq17_e584_d_n8: f64 = ((eq17_e582_d_n8 * s.v[320]) + (eq17_e582 * s.dn[320][8]));
        let eq17_e584_d_n9: f64 = ((eq17_e582_d_n9 * s.v[320]) + (eq17_e582 * s.dn[320][9]));
        let eq17_e586: f64 = (eq17_e584 * (nv0 - nv7));
        let eq17_e586_d_n0: f64 = ((eq17_e584_d_n0 * (nv0 - nv7)) + eq17_e584);
        let eq17_e586_d_n1: f64 = (eq17_e584_d_n1 * (nv0 - nv7));
        let eq17_e586_d_n2: f64 = (eq17_e584_d_n2 * (nv0 - nv7));
        let eq17_e586_d_n3: f64 = (eq17_e584_d_n3 * (nv0 - nv7));
        let eq17_e586_d_n4: f64 = (eq17_e584_d_n4 * (nv0 - nv7));
        let eq17_e586_d_n5: f64 = (eq17_e584_d_n5 * (nv0 - nv7));
        let eq17_e586_d_n6: f64 = (eq17_e584_d_n6 * (nv0 - nv7));
        let eq17_e586_d_n7: f64 = ((eq17_e584_d_n7 * (nv0 - nv7)) + (-eq17_e584));
        let eq17_e586_d_n8: f64 = (eq17_e584_d_n8 * (nv0 - nv7));
        let eq17_e586_d_n9: f64 = (eq17_e584_d_n9 * (nv0 - nv7));
        (eq17_e586, eq17_e586_d_n0, eq17_e586_d_n1, eq17_e586_d_n2, eq17_e586_d_n3, eq17_e586_d_n4, eq17_e586_d_n5, eq17_e586_d_n6, eq17_e586_d_n7, eq17_e586_d_n8, eq17_e586_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e588;
        let eq17_node_derivatives: [f64; 10] = [eq17_e588_d_n0, eq17_e588_d_n1, eq17_e588_d_n2, eq17_e588_d_n3, eq17_e588_d_n4, eq17_e588_d_n5, eq17_e588_d_n6, eq17_e588_d_n7, eq17_e588_d_n8, eq17_e588_d_n9];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
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
        let (eq18_e598,) = {
    if (s.v[1766] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e598;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq18_value),
            &[
            ],
        );
    }
}
