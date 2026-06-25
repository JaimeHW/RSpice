#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq38_e1565,) = {
    if ((s.v[1612] != 0.0) && (s.v[1613] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq38_value: f64 = eq38_e1565;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[5]),
            self.multiplicity * (eq38_value),
            &[
            ],
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
        let (eq39_e1572,) = {
    if ((s.v[1612] != 0.0) && (!(s.v[1613] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq39_value: f64 = eq39_e1572;
        stamper.stamp_potential(
            branches[1],
            eq39_value,
            &[
            ],
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
        let (eq40_e1577,) = {
    if (!(s.v[1612] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e1577;
        stamper.stamp_potential(
            branches[2],
            eq40_value,
            &[
            ],
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
        let (eq41_e1582,) = {
    if (!(s.v[1612] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e1582;
        stamper.stamp_potential(
            branches[3],
            eq41_value,
            &[
            ],
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq42_e1590, eq42_e1590_d_n0, eq42_e1590_d_n1, eq42_e1590_d_n2, eq42_e1590_d_n3, eq42_e1590_d_n4, eq42_e1590_d_n5, eq42_e1590_d_n6, eq42_e1590_d_n7, eq42_e1590_d_n8, eq42_e1590_d_n9, eq42_e1590_d_n10, eq42_e1590_d_n11, eq42_e1590_d_n12, eq42_e1590_d_n13, eq42_e1590_d_n14, eq42_e1590_d_n15, eq42_e1590_d_n16,) = {
    if (s.v[1614] != 0.0) {
        let eq42_e1586: f64 = (p.p28 * (nv2 - nv8));
        let eq42_e1586_d_n2: f64 = p.p28;
        let eq42_e1586_d_n8: f64 = (-p.p28);
        let eq42_e1588: f64 = (eq42_e1586 * s.v[371]);
        let eq42_e1588_d_n0: f64 = (eq42_e1586 * s.dn[371][0]);
        let eq42_e1588_d_n1: f64 = (eq42_e1586 * s.dn[371][1]);
        let eq42_e1588_d_n2: f64 = ((eq42_e1586_d_n2 * s.v[371]) + (eq42_e1586 * s.dn[371][2]));
        let eq42_e1588_d_n3: f64 = (eq42_e1586 * s.dn[371][3]);
        let eq42_e1588_d_n4: f64 = (eq42_e1586 * s.dn[371][4]);
        let eq42_e1588_d_n5: f64 = (eq42_e1586 * s.dn[371][5]);
        let eq42_e1588_d_n6: f64 = (eq42_e1586 * s.dn[371][6]);
        let eq42_e1588_d_n7: f64 = (eq42_e1586 * s.dn[371][7]);
        let eq42_e1588_d_n8: f64 = ((eq42_e1586_d_n8 * s.v[371]) + (eq42_e1586 * s.dn[371][8]));
        let eq42_e1588_d_n9: f64 = (eq42_e1586 * s.dn[371][9]);
        let eq42_e1588_d_n10: f64 = (eq42_e1586 * s.dn[371][10]);
        let eq42_e1588_d_n11: f64 = (eq42_e1586 * s.dn[371][11]);
        let eq42_e1588_d_n12: f64 = (eq42_e1586 * s.dn[371][12]);
        let eq42_e1588_d_n13: f64 = (eq42_e1586 * s.dn[371][13]);
        let eq42_e1588_d_n14: f64 = (eq42_e1586 * s.dn[371][14]);
        let eq42_e1588_d_n15: f64 = (eq42_e1586 * s.dn[371][15]);
        let eq42_e1588_d_n16: f64 = (eq42_e1586 * s.dn[371][16]);
        (eq42_e1588, eq42_e1588_d_n0, eq42_e1588_d_n1, eq42_e1588_d_n2, eq42_e1588_d_n3, eq42_e1588_d_n4, eq42_e1588_d_n5, eq42_e1588_d_n6, eq42_e1588_d_n7, eq42_e1588_d_n8, eq42_e1588_d_n9, eq42_e1588_d_n10, eq42_e1588_d_n11, eq42_e1588_d_n12, eq42_e1588_d_n13, eq42_e1588_d_n14, eq42_e1588_d_n15, eq42_e1588_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e1590;
        let eq42_node_derivatives: [f64; 17] = [eq42_e1590_d_n0, eq42_e1590_d_n1, eq42_e1590_d_n2, eq42_e1590_d_n3, eq42_e1590_d_n4, eq42_e1590_d_n5, eq42_e1590_d_n6, eq42_e1590_d_n7, eq42_e1590_d_n8, eq42_e1590_d_n9, eq42_e1590_d_n10, eq42_e1590_d_n11, eq42_e1590_d_n12, eq42_e1590_d_n13, eq42_e1590_d_n14, eq42_e1590_d_n15, eq42_e1590_d_n16];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
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
        let (eq43_e1600,) = {
    if (s.v[1614] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq43_value: f64 = eq43_e1600;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq43_value),
            &[
            ],
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq44_e1610, eq44_e1610_d_n0, eq44_e1610_d_n1, eq44_e1610_d_n2, eq44_e1610_d_n3, eq44_e1610_d_n4, eq44_e1610_d_n5, eq44_e1610_d_n6, eq44_e1610_d_n7, eq44_e1610_d_n8, eq44_e1610_d_n9, eq44_e1610_d_n10, eq44_e1610_d_n11, eq44_e1610_d_n12, eq44_e1610_d_n13, eq44_e1610_d_n14, eq44_e1610_d_n15, eq44_e1610_d_n16,) = {
    if ((s.v[1614] != 0.0) && (s.v[1615] != 0.0)) {
        let eq44_e1606: f64 = (p.p28 * (nv8 - nv7));
        let eq44_e1606_d_n7: f64 = (-p.p28);
        let eq44_e1606_d_n8: f64 = p.p28;
        let eq44_e1608: f64 = (eq44_e1606 * s.v[373]);
        let eq44_e1608_d_n0: f64 = (eq44_e1606 * s.dn[373][0]);
        let eq44_e1608_d_n1: f64 = (eq44_e1606 * s.dn[373][1]);
        let eq44_e1608_d_n2: f64 = (eq44_e1606 * s.dn[373][2]);
        let eq44_e1608_d_n3: f64 = (eq44_e1606 * s.dn[373][3]);
        let eq44_e1608_d_n4: f64 = (eq44_e1606 * s.dn[373][4]);
        let eq44_e1608_d_n5: f64 = (eq44_e1606 * s.dn[373][5]);
        let eq44_e1608_d_n6: f64 = (eq44_e1606 * s.dn[373][6]);
        let eq44_e1608_d_n7: f64 = ((eq44_e1606_d_n7 * s.v[373]) + (eq44_e1606 * s.dn[373][7]));
        let eq44_e1608_d_n8: f64 = ((eq44_e1606_d_n8 * s.v[373]) + (eq44_e1606 * s.dn[373][8]));
        let eq44_e1608_d_n9: f64 = (eq44_e1606 * s.dn[373][9]);
        let eq44_e1608_d_n10: f64 = (eq44_e1606 * s.dn[373][10]);
        let eq44_e1608_d_n11: f64 = (eq44_e1606 * s.dn[373][11]);
        let eq44_e1608_d_n12: f64 = (eq44_e1606 * s.dn[373][12]);
        let eq44_e1608_d_n13: f64 = (eq44_e1606 * s.dn[373][13]);
        let eq44_e1608_d_n14: f64 = (eq44_e1606 * s.dn[373][14]);
        let eq44_e1608_d_n15: f64 = (eq44_e1606 * s.dn[373][15]);
        let eq44_e1608_d_n16: f64 = (eq44_e1606 * s.dn[373][16]);
        (eq44_e1608, eq44_e1608_d_n0, eq44_e1608_d_n1, eq44_e1608_d_n2, eq44_e1608_d_n3, eq44_e1608_d_n4, eq44_e1608_d_n5, eq44_e1608_d_n6, eq44_e1608_d_n7, eq44_e1608_d_n8, eq44_e1608_d_n9, eq44_e1608_d_n10, eq44_e1608_d_n11, eq44_e1608_d_n12, eq44_e1608_d_n13, eq44_e1608_d_n14, eq44_e1608_d_n15, eq44_e1608_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e1610;
        let eq44_node_derivatives: [f64; 17] = [eq44_e1610_d_n0, eq44_e1610_d_n1, eq44_e1610_d_n2, eq44_e1610_d_n3, eq44_e1610_d_n4, eq44_e1610_d_n5, eq44_e1610_d_n6, eq44_e1610_d_n7, eq44_e1610_d_n8, eq44_e1610_d_n9, eq44_e1610_d_n10, eq44_e1610_d_n11, eq44_e1610_d_n12, eq44_e1610_d_n13, eq44_e1610_d_n14, eq44_e1610_d_n15, eq44_e1610_d_n16];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
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
        let (eq45_e1622,) = {
    if ((s.v[1614] != 0.0) && (s.v[1615] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq45_value: f64 = eq45_e1622;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq45_value),
            &[
            ],
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
        let (eq46_e1641,) = {
    if ((s.v[1614] != 0.0) && (s.v[1615] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq46_value: f64 = eq46_e1641;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq46_value),
            &[
            ],
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
        let (eq47_e1648,) = {
    if ((s.v[1614] != 0.0) && (!(s.v[1615] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e1648;
        stamper.stamp_potential(
            branches[4],
            eq47_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_48_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq48_e1653,) = {
    if (!(s.v[1614] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e1653;
        stamper.stamp_potential(
            branches[5],
            eq48_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_49_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq49_e1658,) = {
    if (!(s.v[1614] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e1658;
        stamper.stamp_potential(
            branches[6],
            eq49_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_50_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq50_e1662,) = {
    if (s.v[1616] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e1662;
        stamper.stamp_potential(
            branches[7],
            eq50_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_51_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq51_e1671, eq51_e1671_d_n0, eq51_e1671_d_n1, eq51_e1671_d_n2, eq51_e1671_d_n3, eq51_e1671_d_n4, eq51_e1671_d_n5, eq51_e1671_d_n6, eq51_e1671_d_n7, eq51_e1671_d_n8, eq51_e1671_d_n9, eq51_e1671_d_n10, eq51_e1671_d_n11, eq51_e1671_d_n12, eq51_e1671_d_n13, eq51_e1671_d_n14, eq51_e1671_d_n15, eq51_e1671_d_n16,) = {
    if (!(s.v[1616] != 0.0)) {
        let eq51_e1667: f64 = (p.p28 * (nv1 - nv10));
        let eq51_e1667_d_n1: f64 = p.p28;
        let eq51_e1667_d_n10: f64 = (-p.p28);
        let eq51_e1669: f64 = (eq51_e1667 * s.v[1617]);
        let eq51_e1669_d_n0: f64 = (eq51_e1667 * s.dn[1617][0]);
        let eq51_e1669_d_n1: f64 = ((eq51_e1667_d_n1 * s.v[1617]) + (eq51_e1667 * s.dn[1617][1]));
        let eq51_e1669_d_n2: f64 = (eq51_e1667 * s.dn[1617][2]);
        let eq51_e1669_d_n3: f64 = (eq51_e1667 * s.dn[1617][3]);
        let eq51_e1669_d_n4: f64 = (eq51_e1667 * s.dn[1617][4]);
        let eq51_e1669_d_n5: f64 = (eq51_e1667 * s.dn[1617][5]);
        let eq51_e1669_d_n6: f64 = (eq51_e1667 * s.dn[1617][6]);
        let eq51_e1669_d_n7: f64 = (eq51_e1667 * s.dn[1617][7]);
        let eq51_e1669_d_n8: f64 = (eq51_e1667 * s.dn[1617][8]);
        let eq51_e1669_d_n9: f64 = (eq51_e1667 * s.dn[1617][9]);
        let eq51_e1669_d_n10: f64 = ((eq51_e1667_d_n10 * s.v[1617]) + (eq51_e1667 * s.dn[1617][10]));
        let eq51_e1669_d_n11: f64 = (eq51_e1667 * s.dn[1617][11]);
        let eq51_e1669_d_n12: f64 = (eq51_e1667 * s.dn[1617][12]);
        let eq51_e1669_d_n13: f64 = (eq51_e1667 * s.dn[1617][13]);
        let eq51_e1669_d_n14: f64 = (eq51_e1667 * s.dn[1617][14]);
        let eq51_e1669_d_n15: f64 = (eq51_e1667 * s.dn[1617][15]);
        let eq51_e1669_d_n16: f64 = (eq51_e1667 * s.dn[1617][16]);
        (eq51_e1669, eq51_e1669_d_n0, eq51_e1669_d_n1, eq51_e1669_d_n2, eq51_e1669_d_n3, eq51_e1669_d_n4, eq51_e1669_d_n5, eq51_e1669_d_n6, eq51_e1669_d_n7, eq51_e1669_d_n8, eq51_e1669_d_n9, eq51_e1669_d_n10, eq51_e1669_d_n11, eq51_e1669_d_n12, eq51_e1669_d_n13, eq51_e1669_d_n14, eq51_e1669_d_n15, eq51_e1669_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1671;
        let eq51_node_derivatives: [f64; 17] = [eq51_e1671_d_n0, eq51_e1671_d_n1, eq51_e1671_d_n2, eq51_e1671_d_n3, eq51_e1671_d_n4, eq51_e1671_d_n5, eq51_e1671_d_n6, eq51_e1671_d_n7, eq51_e1671_d_n8, eq51_e1671_d_n9, eq51_e1671_d_n10, eq51_e1671_d_n11, eq51_e1671_d_n12, eq51_e1671_d_n13, eq51_e1671_d_n14, eq51_e1671_d_n15, eq51_e1671_d_n16];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            self.multiplicity * (eq51_value),
            &nodes,
            &eq51_node_derivatives,
            &branches,
            &eq51_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_52_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq52_e1682,) = {
    if (!(s.v[1616] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e1682;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[10]),
            self.multiplicity * (eq52_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_53_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq53_e1690, eq53_e1690_d_n0, eq53_e1690_d_n1, eq53_e1690_d_n2, eq53_e1690_d_n3, eq53_e1690_d_n4, eq53_e1690_d_n5, eq53_e1690_d_n6, eq53_e1690_d_n7, eq53_e1690_d_n8, eq53_e1690_d_n9, eq53_e1690_d_n10, eq53_e1690_d_n11, eq53_e1690_d_n12, eq53_e1690_d_n13, eq53_e1690_d_n14, eq53_e1690_d_n15, eq53_e1690_d_n16,) = {
    if (s.v[1620] != 0.0) {
        let eq53_e1686: f64 = ((nv10 - nv9) * p.p28);
        let eq53_e1686_d_n9: f64 = (-p.p28);
        let eq53_e1686_d_n10: f64 = p.p28;
        let eq53_e1688: f64 = (eq53_e1686 * s.v[254]);
        let eq53_e1688_d_n0: f64 = (eq53_e1686 * s.dn[254][0]);
        let eq53_e1688_d_n1: f64 = (eq53_e1686 * s.dn[254][1]);
        let eq53_e1688_d_n2: f64 = (eq53_e1686 * s.dn[254][2]);
        let eq53_e1688_d_n3: f64 = (eq53_e1686 * s.dn[254][3]);
        let eq53_e1688_d_n4: f64 = (eq53_e1686 * s.dn[254][4]);
        let eq53_e1688_d_n5: f64 = (eq53_e1686 * s.dn[254][5]);
        let eq53_e1688_d_n6: f64 = (eq53_e1686 * s.dn[254][6]);
        let eq53_e1688_d_n7: f64 = (eq53_e1686 * s.dn[254][7]);
        let eq53_e1688_d_n8: f64 = (eq53_e1686 * s.dn[254][8]);
        let eq53_e1688_d_n9: f64 = ((eq53_e1686_d_n9 * s.v[254]) + (eq53_e1686 * s.dn[254][9]));
        let eq53_e1688_d_n10: f64 = ((eq53_e1686_d_n10 * s.v[254]) + (eq53_e1686 * s.dn[254][10]));
        let eq53_e1688_d_n11: f64 = (eq53_e1686 * s.dn[254][11]);
        let eq53_e1688_d_n12: f64 = (eq53_e1686 * s.dn[254][12]);
        let eq53_e1688_d_n13: f64 = (eq53_e1686 * s.dn[254][13]);
        let eq53_e1688_d_n14: f64 = (eq53_e1686 * s.dn[254][14]);
        let eq53_e1688_d_n15: f64 = (eq53_e1686 * s.dn[254][15]);
        let eq53_e1688_d_n16: f64 = (eq53_e1686 * s.dn[254][16]);
        (eq53_e1688, eq53_e1688_d_n0, eq53_e1688_d_n1, eq53_e1688_d_n2, eq53_e1688_d_n3, eq53_e1688_d_n4, eq53_e1688_d_n5, eq53_e1688_d_n6, eq53_e1688_d_n7, eq53_e1688_d_n8, eq53_e1688_d_n9, eq53_e1688_d_n10, eq53_e1688_d_n11, eq53_e1688_d_n12, eq53_e1688_d_n13, eq53_e1688_d_n14, eq53_e1688_d_n15, eq53_e1688_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e1690;
        let eq53_node_derivatives: [f64; 17] = [eq53_e1690_d_n0, eq53_e1690_d_n1, eq53_e1690_d_n2, eq53_e1690_d_n3, eq53_e1690_d_n4, eq53_e1690_d_n5, eq53_e1690_d_n6, eq53_e1690_d_n7, eq53_e1690_d_n8, eq53_e1690_d_n9, eq53_e1690_d_n10, eq53_e1690_d_n11, eq53_e1690_d_n12, eq53_e1690_d_n13, eq53_e1690_d_n14, eq53_e1690_d_n15, eq53_e1690_d_n16];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[9]),
            self.multiplicity * (eq53_value),
            &nodes,
            &eq53_node_derivatives,
            &branches,
            &eq53_branch_derivatives,
            self.multiplicity,
        );
    }
}
