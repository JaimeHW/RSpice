#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_54_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq54_e1695,) = {
    if (!(s.v[1620] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e1695;
        stamper.stamp_potential(
            branches[8],
            eq54_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_55_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n1, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14, eq55_e1708_d_n15, eq55_e1708_d_n16,) = {
    if (s.v[1621] != 0.0) {
        let eq55_e1699: f64 = (s.v[390] * s.v[747]);
        let eq55_e1699_d_n0: f64 = ((s.dn[390][0] * s.v[747]) + (s.v[390] * s.dn[747][0]));
        let eq55_e1699_d_n1: f64 = ((s.dn[390][1] * s.v[747]) + (s.v[390] * s.dn[747][1]));
        let eq55_e1699_d_n2: f64 = ((s.dn[390][2] * s.v[747]) + (s.v[390] * s.dn[747][2]));
        let eq55_e1699_d_n3: f64 = ((s.dn[390][3] * s.v[747]) + (s.v[390] * s.dn[747][3]));
        let eq55_e1699_d_n4: f64 = ((s.dn[390][4] * s.v[747]) + (s.v[390] * s.dn[747][4]));
        let eq55_e1699_d_n5: f64 = ((s.dn[390][5] * s.v[747]) + (s.v[390] * s.dn[747][5]));
        let eq55_e1699_d_n6: f64 = ((s.dn[390][6] * s.v[747]) + (s.v[390] * s.dn[747][6]));
        let eq55_e1699_d_n7: f64 = ((s.dn[390][7] * s.v[747]) + (s.v[390] * s.dn[747][7]));
        let eq55_e1699_d_n8: f64 = ((s.dn[390][8] * s.v[747]) + (s.v[390] * s.dn[747][8]));
        let eq55_e1699_d_n9: f64 = ((s.dn[390][9] * s.v[747]) + (s.v[390] * s.dn[747][9]));
        let eq55_e1699_d_n10: f64 = ((s.dn[390][10] * s.v[747]) + (s.v[390] * s.dn[747][10]));
        let eq55_e1699_d_n11: f64 = ((s.dn[390][11] * s.v[747]) + (s.v[390] * s.dn[747][11]));
        let eq55_e1699_d_n12: f64 = ((s.dn[390][12] * s.v[747]) + (s.v[390] * s.dn[747][12]));
        let eq55_e1699_d_n13: f64 = ((s.dn[390][13] * s.v[747]) + (s.v[390] * s.dn[747][13]));
        let eq55_e1699_d_n14: f64 = ((s.dn[390][14] * s.v[747]) + (s.v[390] * s.dn[747][14]));
        let eq55_e1699_d_n15: f64 = ((s.dn[390][15] * s.v[747]) + (s.v[390] * s.dn[747][15]));
        let eq55_e1699_d_n16: f64 = ((s.dn[390][16] * s.v[747]) + (s.v[390] * s.dn[747][16]));
        let eq55_e1702: f64 = (s.v[390] * s.v[748]);
        let eq55_e1702_d_n0: f64 = ((s.dn[390][0] * s.v[748]) + (s.v[390] * s.dn[748][0]));
        let eq55_e1702_d_n1: f64 = ((s.dn[390][1] * s.v[748]) + (s.v[390] * s.dn[748][1]));
        let eq55_e1702_d_n2: f64 = ((s.dn[390][2] * s.v[748]) + (s.v[390] * s.dn[748][2]));
        let eq55_e1702_d_n3: f64 = ((s.dn[390][3] * s.v[748]) + (s.v[390] * s.dn[748][3]));
        let eq55_e1702_d_n4: f64 = ((s.dn[390][4] * s.v[748]) + (s.v[390] * s.dn[748][4]));
        let eq55_e1702_d_n5: f64 = ((s.dn[390][5] * s.v[748]) + (s.v[390] * s.dn[748][5]));
        let eq55_e1702_d_n6: f64 = ((s.dn[390][6] * s.v[748]) + (s.v[390] * s.dn[748][6]));
        let eq55_e1702_d_n7: f64 = ((s.dn[390][7] * s.v[748]) + (s.v[390] * s.dn[748][7]));
        let eq55_e1702_d_n8: f64 = ((s.dn[390][8] * s.v[748]) + (s.v[390] * s.dn[748][8]));
        let eq55_e1702_d_n9: f64 = ((s.dn[390][9] * s.v[748]) + (s.v[390] * s.dn[748][9]));
        let eq55_e1702_d_n10: f64 = ((s.dn[390][10] * s.v[748]) + (s.v[390] * s.dn[748][10]));
        let eq55_e1702_d_n11: f64 = ((s.dn[390][11] * s.v[748]) + (s.v[390] * s.dn[748][11]));
        let eq55_e1702_d_n12: f64 = ((s.dn[390][12] * s.v[748]) + (s.v[390] * s.dn[748][12]));
        let eq55_e1702_d_n13: f64 = ((s.dn[390][13] * s.v[748]) + (s.v[390] * s.dn[748][13]));
        let eq55_e1702_d_n14: f64 = ((s.dn[390][14] * s.v[748]) + (s.v[390] * s.dn[748][14]));
        let eq55_e1702_d_n15: f64 = ((s.dn[390][15] * s.v[748]) + (s.v[390] * s.dn[748][15]));
        let eq55_e1702_d_n16: f64 = ((s.dn[390][16] * s.v[748]) + (s.v[390] * s.dn[748][16]));
        let eq55_e1703: f64 = self.eval_ddt(9, eq55_e1702);
        let eq55_e1703_d_n0: f64 = self.ddt_jacobian(eq55_e1702_d_n0);
        let eq55_e1703_d_n1: f64 = self.ddt_jacobian(eq55_e1702_d_n1);
        let eq55_e1703_d_n2: f64 = self.ddt_jacobian(eq55_e1702_d_n2);
        let eq55_e1703_d_n3: f64 = self.ddt_jacobian(eq55_e1702_d_n3);
        let eq55_e1703_d_n4: f64 = self.ddt_jacobian(eq55_e1702_d_n4);
        let eq55_e1703_d_n5: f64 = self.ddt_jacobian(eq55_e1702_d_n5);
        let eq55_e1703_d_n6: f64 = self.ddt_jacobian(eq55_e1702_d_n6);
        let eq55_e1703_d_n7: f64 = self.ddt_jacobian(eq55_e1702_d_n7);
        let eq55_e1703_d_n8: f64 = self.ddt_jacobian(eq55_e1702_d_n8);
        let eq55_e1703_d_n9: f64 = self.ddt_jacobian(eq55_e1702_d_n9);
        let eq55_e1703_d_n10: f64 = self.ddt_jacobian(eq55_e1702_d_n10);
        let eq55_e1703_d_n11: f64 = self.ddt_jacobian(eq55_e1702_d_n11);
        let eq55_e1703_d_n12: f64 = self.ddt_jacobian(eq55_e1702_d_n12);
        let eq55_e1703_d_n13: f64 = self.ddt_jacobian(eq55_e1702_d_n13);
        let eq55_e1703_d_n14: f64 = self.ddt_jacobian(eq55_e1702_d_n14);
        let eq55_e1703_d_n15: f64 = self.ddt_jacobian(eq55_e1702_d_n15);
        let eq55_e1703_d_n16: f64 = self.ddt_jacobian(eq55_e1702_d_n16);
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1703);
        let eq55_e1704_d_n0: f64 = (eq55_e1699_d_n0 + eq55_e1703_d_n0);
        let eq55_e1704_d_n1: f64 = (eq55_e1699_d_n1 + eq55_e1703_d_n1);
        let eq55_e1704_d_n2: f64 = (eq55_e1699_d_n2 + eq55_e1703_d_n2);
        let eq55_e1704_d_n3: f64 = (eq55_e1699_d_n3 + eq55_e1703_d_n3);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + eq55_e1703_d_n4);
        let eq55_e1704_d_n5: f64 = (eq55_e1699_d_n5 + eq55_e1703_d_n5);
        let eq55_e1704_d_n6: f64 = (eq55_e1699_d_n6 + eq55_e1703_d_n6);
        let eq55_e1704_d_n7: f64 = (eq55_e1699_d_n7 + eq55_e1703_d_n7);
        let eq55_e1704_d_n8: f64 = (eq55_e1699_d_n8 + eq55_e1703_d_n8);
        let eq55_e1704_d_n9: f64 = (eq55_e1699_d_n9 + eq55_e1703_d_n9);
        let eq55_e1704_d_n10: f64 = (eq55_e1699_d_n10 + eq55_e1703_d_n10);
        let eq55_e1704_d_n11: f64 = (eq55_e1699_d_n11 + eq55_e1703_d_n11);
        let eq55_e1704_d_n12: f64 = (eq55_e1699_d_n12 + eq55_e1703_d_n12);
        let eq55_e1704_d_n13: f64 = (eq55_e1699_d_n13 + eq55_e1703_d_n13);
        let eq55_e1704_d_n14: f64 = (eq55_e1699_d_n14 + eq55_e1703_d_n14);
        let eq55_e1704_d_n15: f64 = (eq55_e1699_d_n15 + eq55_e1703_d_n15);
        let eq55_e1704_d_n16: f64 = (eq55_e1699_d_n16 + eq55_e1703_d_n16);
        let eq55_e1706: f64 = (eq55_e1704 - s.v[749]);
        let eq55_e1706_d_n0: f64 = (eq55_e1704_d_n0 - s.dn[749][0]);
        let eq55_e1706_d_n1: f64 = (eq55_e1704_d_n1 - s.dn[749][1]);
        let eq55_e1706_d_n2: f64 = (eq55_e1704_d_n2 - s.dn[749][2]);
        let eq55_e1706_d_n3: f64 = (eq55_e1704_d_n3 - s.dn[749][3]);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - s.dn[749][4]);
        let eq55_e1706_d_n5: f64 = (eq55_e1704_d_n5 - s.dn[749][5]);
        let eq55_e1706_d_n6: f64 = (eq55_e1704_d_n6 - s.dn[749][6]);
        let eq55_e1706_d_n7: f64 = (eq55_e1704_d_n7 - s.dn[749][7]);
        let eq55_e1706_d_n8: f64 = (eq55_e1704_d_n8 - s.dn[749][8]);
        let eq55_e1706_d_n9: f64 = (eq55_e1704_d_n9 - s.dn[749][9]);
        let eq55_e1706_d_n10: f64 = (eq55_e1704_d_n10 - s.dn[749][10]);
        let eq55_e1706_d_n11: f64 = (eq55_e1704_d_n11 - s.dn[749][11]);
        let eq55_e1706_d_n12: f64 = (eq55_e1704_d_n12 - s.dn[749][12]);
        let eq55_e1706_d_n13: f64 = (eq55_e1704_d_n13 - s.dn[749][13]);
        let eq55_e1706_d_n14: f64 = (eq55_e1704_d_n14 - s.dn[749][14]);
        let eq55_e1706_d_n15: f64 = (eq55_e1704_d_n15 - s.dn[749][15]);
        let eq55_e1706_d_n16: f64 = (eq55_e1704_d_n16 - s.dn[749][16]);
        (eq55_e1706, eq55_e1706_d_n0, eq55_e1706_d_n1, eq55_e1706_d_n2, eq55_e1706_d_n3, eq55_e1706_d_n4, eq55_e1706_d_n5, eq55_e1706_d_n6, eq55_e1706_d_n7, eq55_e1706_d_n8, eq55_e1706_d_n9, eq55_e1706_d_n10, eq55_e1706_d_n11, eq55_e1706_d_n12, eq55_e1706_d_n13, eq55_e1706_d_n14, eq55_e1706_d_n15, eq55_e1706_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1708;
        let eq55_node_derivatives: [f64; 17] = [eq55_e1708_d_n0, eq55_e1708_d_n1, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14, eq55_e1708_d_n15, eq55_e1708_d_n16];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq55_value),
            &nodes,
            &eq55_node_derivatives,
            &branches,
            &eq55_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_56_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq56_e1713,) = {
    if (!(s.v[1621] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e1713;
        stamper.stamp_potential(
            branches[9],
            eq56_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_57_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq57_e1721, eq57_e1721_d_n0, eq57_e1721_d_n1, eq57_e1721_d_n2, eq57_e1721_d_n3, eq57_e1721_d_n4, eq57_e1721_d_n5, eq57_e1721_d_n6, eq57_e1721_d_n7, eq57_e1721_d_n8, eq57_e1721_d_n9, eq57_e1721_d_n10, eq57_e1721_d_n11, eq57_e1721_d_n12, eq57_e1721_d_n13, eq57_e1721_d_n14, eq57_e1721_d_n15, eq57_e1721_d_n16,) = {
    if (s.v[1626] != 0.0) {
        let eq57_e1717: f64 = (p.p28 * (nv11 - nv12));
        let eq57_e1717_d_n11: f64 = p.p28;
        let eq57_e1717_d_n12: f64 = (-p.p28);
        let eq57_e1719: f64 = (eq57_e1717 * s.v[274]);
        let eq57_e1719_d_n0: f64 = (eq57_e1717 * s.dn[274][0]);
        let eq57_e1719_d_n1: f64 = (eq57_e1717 * s.dn[274][1]);
        let eq57_e1719_d_n2: f64 = (eq57_e1717 * s.dn[274][2]);
        let eq57_e1719_d_n3: f64 = (eq57_e1717 * s.dn[274][3]);
        let eq57_e1719_d_n4: f64 = (eq57_e1717 * s.dn[274][4]);
        let eq57_e1719_d_n5: f64 = (eq57_e1717 * s.dn[274][5]);
        let eq57_e1719_d_n6: f64 = (eq57_e1717 * s.dn[274][6]);
        let eq57_e1719_d_n7: f64 = (eq57_e1717 * s.dn[274][7]);
        let eq57_e1719_d_n8: f64 = (eq57_e1717 * s.dn[274][8]);
        let eq57_e1719_d_n9: f64 = (eq57_e1717 * s.dn[274][9]);
        let eq57_e1719_d_n10: f64 = (eq57_e1717 * s.dn[274][10]);
        let eq57_e1719_d_n11: f64 = ((eq57_e1717_d_n11 * s.v[274]) + (eq57_e1717 * s.dn[274][11]));
        let eq57_e1719_d_n12: f64 = ((eq57_e1717_d_n12 * s.v[274]) + (eq57_e1717 * s.dn[274][12]));
        let eq57_e1719_d_n13: f64 = (eq57_e1717 * s.dn[274][13]);
        let eq57_e1719_d_n14: f64 = (eq57_e1717 * s.dn[274][14]);
        let eq57_e1719_d_n15: f64 = (eq57_e1717 * s.dn[274][15]);
        let eq57_e1719_d_n16: f64 = (eq57_e1717 * s.dn[274][16]);
        (eq57_e1719, eq57_e1719_d_n0, eq57_e1719_d_n1, eq57_e1719_d_n2, eq57_e1719_d_n3, eq57_e1719_d_n4, eq57_e1719_d_n5, eq57_e1719_d_n6, eq57_e1719_d_n7, eq57_e1719_d_n8, eq57_e1719_d_n9, eq57_e1719_d_n10, eq57_e1719_d_n11, eq57_e1719_d_n12, eq57_e1719_d_n13, eq57_e1719_d_n14, eq57_e1719_d_n15, eq57_e1719_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1721;
        let eq57_node_derivatives: [f64; 17] = [eq57_e1721_d_n0, eq57_e1721_d_n1, eq57_e1721_d_n2, eq57_e1721_d_n3, eq57_e1721_d_n4, eq57_e1721_d_n5, eq57_e1721_d_n6, eq57_e1721_d_n7, eq57_e1721_d_n8, eq57_e1721_d_n9, eq57_e1721_d_n10, eq57_e1721_d_n11, eq57_e1721_d_n12, eq57_e1721_d_n13, eq57_e1721_d_n14, eq57_e1721_d_n15, eq57_e1721_d_n16];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            self.multiplicity * (eq57_value),
            &nodes,
            &eq57_node_derivatives,
            &branches,
            &eq57_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq58_e1729, eq58_e1729_d_n0, eq58_e1729_d_n1, eq58_e1729_d_n2, eq58_e1729_d_n3, eq58_e1729_d_n4, eq58_e1729_d_n5, eq58_e1729_d_n6, eq58_e1729_d_n7, eq58_e1729_d_n8, eq58_e1729_d_n9, eq58_e1729_d_n10, eq58_e1729_d_n11, eq58_e1729_d_n12, eq58_e1729_d_n13, eq58_e1729_d_n14, eq58_e1729_d_n15, eq58_e1729_d_n16,) = {
    if (s.v[1626] != 0.0) {
        let eq58_e1725: f64 = (p.p28 * (nv3 - nv12));
        let eq58_e1725_d_n3: f64 = p.p28;
        let eq58_e1725_d_n12: f64 = (-p.p28);
        let eq58_e1727: f64 = (eq58_e1725 * s.v[271]);
        let eq58_e1727_d_n0: f64 = (eq58_e1725 * s.dn[271][0]);
        let eq58_e1727_d_n1: f64 = (eq58_e1725 * s.dn[271][1]);
        let eq58_e1727_d_n2: f64 = (eq58_e1725 * s.dn[271][2]);
        let eq58_e1727_d_n3: f64 = ((eq58_e1725_d_n3 * s.v[271]) + (eq58_e1725 * s.dn[271][3]));
        let eq58_e1727_d_n4: f64 = (eq58_e1725 * s.dn[271][4]);
        let eq58_e1727_d_n5: f64 = (eq58_e1725 * s.dn[271][5]);
        let eq58_e1727_d_n6: f64 = (eq58_e1725 * s.dn[271][6]);
        let eq58_e1727_d_n7: f64 = (eq58_e1725 * s.dn[271][7]);
        let eq58_e1727_d_n8: f64 = (eq58_e1725 * s.dn[271][8]);
        let eq58_e1727_d_n9: f64 = (eq58_e1725 * s.dn[271][9]);
        let eq58_e1727_d_n10: f64 = (eq58_e1725 * s.dn[271][10]);
        let eq58_e1727_d_n11: f64 = (eq58_e1725 * s.dn[271][11]);
        let eq58_e1727_d_n12: f64 = ((eq58_e1725_d_n12 * s.v[271]) + (eq58_e1725 * s.dn[271][12]));
        let eq58_e1727_d_n13: f64 = (eq58_e1725 * s.dn[271][13]);
        let eq58_e1727_d_n14: f64 = (eq58_e1725 * s.dn[271][14]);
        let eq58_e1727_d_n15: f64 = (eq58_e1725 * s.dn[271][15]);
        let eq58_e1727_d_n16: f64 = (eq58_e1725 * s.dn[271][16]);
        (eq58_e1727, eq58_e1727_d_n0, eq58_e1727_d_n1, eq58_e1727_d_n2, eq58_e1727_d_n3, eq58_e1727_d_n4, eq58_e1727_d_n5, eq58_e1727_d_n6, eq58_e1727_d_n7, eq58_e1727_d_n8, eq58_e1727_d_n9, eq58_e1727_d_n10, eq58_e1727_d_n11, eq58_e1727_d_n12, eq58_e1727_d_n13, eq58_e1727_d_n14, eq58_e1727_d_n15, eq58_e1727_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e1729;
        let eq58_node_derivatives: [f64; 17] = [eq58_e1729_d_n0, eq58_e1729_d_n1, eq58_e1729_d_n2, eq58_e1729_d_n3, eq58_e1729_d_n4, eq58_e1729_d_n5, eq58_e1729_d_n6, eq58_e1729_d_n7, eq58_e1729_d_n8, eq58_e1729_d_n9, eq58_e1729_d_n10, eq58_e1729_d_n11, eq58_e1729_d_n12, eq58_e1729_d_n13, eq58_e1729_d_n14, eq58_e1729_d_n15, eq58_e1729_d_n16];
        let eq58_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[12]),
            self.multiplicity * (eq58_value),
            &nodes,
            &eq58_node_derivatives,
            &branches,
            &eq58_branch_derivatives,
            self.multiplicity,
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq59_e1737, eq59_e1737_d_n0, eq59_e1737_d_n1, eq59_e1737_d_n2, eq59_e1737_d_n3, eq59_e1737_d_n4, eq59_e1737_d_n5, eq59_e1737_d_n6, eq59_e1737_d_n7, eq59_e1737_d_n8, eq59_e1737_d_n9, eq59_e1737_d_n10, eq59_e1737_d_n11, eq59_e1737_d_n12, eq59_e1737_d_n13, eq59_e1737_d_n14, eq59_e1737_d_n15, eq59_e1737_d_n16,) = {
    if (s.v[1626] != 0.0) {
        let eq59_e1733: f64 = (p.p28 * (nv3 - nv11));
        let eq59_e1733_d_n3: f64 = p.p28;
        let eq59_e1733_d_n11: f64 = (-p.p28);
        let eq59_e1735: f64 = (eq59_e1733 * s.v[273]);
        let eq59_e1735_d_n0: f64 = (eq59_e1733 * s.dn[273][0]);
        let eq59_e1735_d_n1: f64 = (eq59_e1733 * s.dn[273][1]);
        let eq59_e1735_d_n2: f64 = (eq59_e1733 * s.dn[273][2]);
        let eq59_e1735_d_n3: f64 = ((eq59_e1733_d_n3 * s.v[273]) + (eq59_e1733 * s.dn[273][3]));
        let eq59_e1735_d_n4: f64 = (eq59_e1733 * s.dn[273][4]);
        let eq59_e1735_d_n5: f64 = (eq59_e1733 * s.dn[273][5]);
        let eq59_e1735_d_n6: f64 = (eq59_e1733 * s.dn[273][6]);
        let eq59_e1735_d_n7: f64 = (eq59_e1733 * s.dn[273][7]);
        let eq59_e1735_d_n8: f64 = (eq59_e1733 * s.dn[273][8]);
        let eq59_e1735_d_n9: f64 = (eq59_e1733 * s.dn[273][9]);
        let eq59_e1735_d_n10: f64 = (eq59_e1733 * s.dn[273][10]);
        let eq59_e1735_d_n11: f64 = ((eq59_e1733_d_n11 * s.v[273]) + (eq59_e1733 * s.dn[273][11]));
        let eq59_e1735_d_n12: f64 = (eq59_e1733 * s.dn[273][12]);
        let eq59_e1735_d_n13: f64 = (eq59_e1733 * s.dn[273][13]);
        let eq59_e1735_d_n14: f64 = (eq59_e1733 * s.dn[273][14]);
        let eq59_e1735_d_n15: f64 = (eq59_e1733 * s.dn[273][15]);
        let eq59_e1735_d_n16: f64 = (eq59_e1733 * s.dn[273][16]);
        (eq59_e1735, eq59_e1735_d_n0, eq59_e1735_d_n1, eq59_e1735_d_n2, eq59_e1735_d_n3, eq59_e1735_d_n4, eq59_e1735_d_n5, eq59_e1735_d_n6, eq59_e1735_d_n7, eq59_e1735_d_n8, eq59_e1735_d_n9, eq59_e1735_d_n10, eq59_e1735_d_n11, eq59_e1735_d_n12, eq59_e1735_d_n13, eq59_e1735_d_n14, eq59_e1735_d_n15, eq59_e1735_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1737;
        let eq59_node_derivatives: [f64; 17] = [eq59_e1737_d_n0, eq59_e1737_d_n1, eq59_e1737_d_n2, eq59_e1737_d_n3, eq59_e1737_d_n4, eq59_e1737_d_n5, eq59_e1737_d_n6, eq59_e1737_d_n7, eq59_e1737_d_n8, eq59_e1737_d_n9, eq59_e1737_d_n10, eq59_e1737_d_n11, eq59_e1737_d_n12, eq59_e1737_d_n13, eq59_e1737_d_n14, eq59_e1737_d_n15, eq59_e1737_d_n16];
        let eq59_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[11]),
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq60_e1745, eq60_e1745_d_n0, eq60_e1745_d_n1, eq60_e1745_d_n2, eq60_e1745_d_n3, eq60_e1745_d_n4, eq60_e1745_d_n5, eq60_e1745_d_n6, eq60_e1745_d_n7, eq60_e1745_d_n8, eq60_e1745_d_n9, eq60_e1745_d_n10, eq60_e1745_d_n11, eq60_e1745_d_n12, eq60_e1745_d_n13, eq60_e1745_d_n14, eq60_e1745_d_n15, eq60_e1745_d_n16,) = {
    if (s.v[1626] != 0.0) {
        let eq60_e1741: f64 = (p.p28 * (nv3 - nv13));
        let eq60_e1741_d_n3: f64 = p.p28;
        let eq60_e1741_d_n13: f64 = (-p.p28);
        let eq60_e1743: f64 = (eq60_e1741 * s.v[272]);
        let eq60_e1743_d_n0: f64 = (eq60_e1741 * s.dn[272][0]);
        let eq60_e1743_d_n1: f64 = (eq60_e1741 * s.dn[272][1]);
        let eq60_e1743_d_n2: f64 = (eq60_e1741 * s.dn[272][2]);
        let eq60_e1743_d_n3: f64 = ((eq60_e1741_d_n3 * s.v[272]) + (eq60_e1741 * s.dn[272][3]));
        let eq60_e1743_d_n4: f64 = (eq60_e1741 * s.dn[272][4]);
        let eq60_e1743_d_n5: f64 = (eq60_e1741 * s.dn[272][5]);
        let eq60_e1743_d_n6: f64 = (eq60_e1741 * s.dn[272][6]);
        let eq60_e1743_d_n7: f64 = (eq60_e1741 * s.dn[272][7]);
        let eq60_e1743_d_n8: f64 = (eq60_e1741 * s.dn[272][8]);
        let eq60_e1743_d_n9: f64 = (eq60_e1741 * s.dn[272][9]);
        let eq60_e1743_d_n10: f64 = (eq60_e1741 * s.dn[272][10]);
        let eq60_e1743_d_n11: f64 = (eq60_e1741 * s.dn[272][11]);
        let eq60_e1743_d_n12: f64 = (eq60_e1741 * s.dn[272][12]);
        let eq60_e1743_d_n13: f64 = ((eq60_e1741_d_n13 * s.v[272]) + (eq60_e1741 * s.dn[272][13]));
        let eq60_e1743_d_n14: f64 = (eq60_e1741 * s.dn[272][14]);
        let eq60_e1743_d_n15: f64 = (eq60_e1741 * s.dn[272][15]);
        let eq60_e1743_d_n16: f64 = (eq60_e1741 * s.dn[272][16]);
        (eq60_e1743, eq60_e1743_d_n0, eq60_e1743_d_n1, eq60_e1743_d_n2, eq60_e1743_d_n3, eq60_e1743_d_n4, eq60_e1743_d_n5, eq60_e1743_d_n6, eq60_e1743_d_n7, eq60_e1743_d_n8, eq60_e1743_d_n9, eq60_e1743_d_n10, eq60_e1743_d_n11, eq60_e1743_d_n12, eq60_e1743_d_n13, eq60_e1743_d_n14, eq60_e1743_d_n15, eq60_e1743_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1745;
        let eq60_node_derivatives: [f64; 17] = [eq60_e1745_d_n0, eq60_e1745_d_n1, eq60_e1745_d_n2, eq60_e1745_d_n3, eq60_e1745_d_n4, eq60_e1745_d_n5, eq60_e1745_d_n6, eq60_e1745_d_n7, eq60_e1745_d_n8, eq60_e1745_d_n9, eq60_e1745_d_n10, eq60_e1745_d_n11, eq60_e1745_d_n12, eq60_e1745_d_n13, eq60_e1745_d_n14, eq60_e1745_d_n15, eq60_e1745_d_n16];
        let eq60_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[13]),
            self.multiplicity * (eq60_value),
            &nodes,
            &eq60_node_derivatives,
            &branches,
            &eq60_branch_derivatives,
            self.multiplicity,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq61_e1753, eq61_e1753_d_n0, eq61_e1753_d_n1, eq61_e1753_d_n2, eq61_e1753_d_n3, eq61_e1753_d_n4, eq61_e1753_d_n5, eq61_e1753_d_n6, eq61_e1753_d_n7, eq61_e1753_d_n8, eq61_e1753_d_n9, eq61_e1753_d_n10, eq61_e1753_d_n11, eq61_e1753_d_n12, eq61_e1753_d_n13, eq61_e1753_d_n14, eq61_e1753_d_n15, eq61_e1753_d_n16,) = {
    if (s.v[1626] != 0.0) {
        let eq61_e1749: f64 = (p.p28 * (nv11 - nv13));
        let eq61_e1749_d_n11: f64 = p.p28;
        let eq61_e1749_d_n13: f64 = (-p.p28);
        let eq61_e1751: f64 = (eq61_e1749 * s.v[275]);
        let eq61_e1751_d_n0: f64 = (eq61_e1749 * s.dn[275][0]);
        let eq61_e1751_d_n1: f64 = (eq61_e1749 * s.dn[275][1]);
        let eq61_e1751_d_n2: f64 = (eq61_e1749 * s.dn[275][2]);
        let eq61_e1751_d_n3: f64 = (eq61_e1749 * s.dn[275][3]);
        let eq61_e1751_d_n4: f64 = (eq61_e1749 * s.dn[275][4]);
        let eq61_e1751_d_n5: f64 = (eq61_e1749 * s.dn[275][5]);
        let eq61_e1751_d_n6: f64 = (eq61_e1749 * s.dn[275][6]);
        let eq61_e1751_d_n7: f64 = (eq61_e1749 * s.dn[275][7]);
        let eq61_e1751_d_n8: f64 = (eq61_e1749 * s.dn[275][8]);
        let eq61_e1751_d_n9: f64 = (eq61_e1749 * s.dn[275][9]);
        let eq61_e1751_d_n10: f64 = (eq61_e1749 * s.dn[275][10]);
        let eq61_e1751_d_n11: f64 = ((eq61_e1749_d_n11 * s.v[275]) + (eq61_e1749 * s.dn[275][11]));
        let eq61_e1751_d_n12: f64 = (eq61_e1749 * s.dn[275][12]);
        let eq61_e1751_d_n13: f64 = ((eq61_e1749_d_n13 * s.v[275]) + (eq61_e1749 * s.dn[275][13]));
        let eq61_e1751_d_n14: f64 = (eq61_e1749 * s.dn[275][14]);
        let eq61_e1751_d_n15: f64 = (eq61_e1749 * s.dn[275][15]);
        let eq61_e1751_d_n16: f64 = (eq61_e1749 * s.dn[275][16]);
        (eq61_e1751, eq61_e1751_d_n0, eq61_e1751_d_n1, eq61_e1751_d_n2, eq61_e1751_d_n3, eq61_e1751_d_n4, eq61_e1751_d_n5, eq61_e1751_d_n6, eq61_e1751_d_n7, eq61_e1751_d_n8, eq61_e1751_d_n9, eq61_e1751_d_n10, eq61_e1751_d_n11, eq61_e1751_d_n12, eq61_e1751_d_n13, eq61_e1751_d_n14, eq61_e1751_d_n15, eq61_e1751_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1753;
        let eq61_node_derivatives: [f64; 17] = [eq61_e1753_d_n0, eq61_e1753_d_n1, eq61_e1753_d_n2, eq61_e1753_d_n3, eq61_e1753_d_n4, eq61_e1753_d_n5, eq61_e1753_d_n6, eq61_e1753_d_n7, eq61_e1753_d_n8, eq61_e1753_d_n9, eq61_e1753_d_n10, eq61_e1753_d_n11, eq61_e1753_d_n12, eq61_e1753_d_n13, eq61_e1753_d_n14, eq61_e1753_d_n15, eq61_e1753_d_n16];
        let eq61_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[13]),
            self.multiplicity * (eq61_value),
            &nodes,
            &eq61_node_derivatives,
            &branches,
            &eq61_branch_derivatives,
            self.multiplicity,
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
        let (eq62_e1763,) = {
    if (s.v[1626] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e1763;
        stamper.stamp_current(
            Some(nodes[12]),
            Some(nodes[11]),
            self.multiplicity * (eq62_value),
            &[
            ],
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
        let (eq63_e1773,) = {
    if (s.v[1626] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e1773;
        stamper.stamp_current(
            Some(nodes[12]),
            Some(nodes[3]),
            self.multiplicity * (eq63_value),
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
        let (eq64_e1783,) = {
    if (s.v[1626] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e1783;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[11]),
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
        let (eq65_e1793,) = {
    if (s.v[1626] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e1793;
        stamper.stamp_current(
            Some(nodes[13]),
            Some(nodes[11]),
            self.multiplicity * (eq65_value),
            &[
            ],
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
        let (eq66_e1803,) = {
    if (s.v[1626] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e1803;
        stamper.stamp_current(
            Some(nodes[13]),
            Some(nodes[3]),
            self.multiplicity * (eq66_value),
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
        let (eq67_e1808,) = {
    if (!(s.v[1626] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1808;
        stamper.stamp_potential(
            branches[10],
            eq67_value,
            &[
            ],
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
        let (eq68_e1813,) = {
    if (!(s.v[1626] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1813;
        stamper.stamp_potential(
            branches[11],
            eq68_value,
            &[
            ],
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
        let (eq69_e1818,) = {
    if (!(s.v[1626] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq69_value: f64 = eq69_e1818;
        stamper.stamp_potential(
            branches[12],
            eq69_value,
            &[
            ],
        );
    }
}
