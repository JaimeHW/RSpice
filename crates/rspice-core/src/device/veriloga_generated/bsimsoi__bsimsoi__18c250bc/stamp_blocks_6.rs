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
        let (eq58_e1745,) = {
    if (s.v[1554] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e1745;
        stamper.stamp_potential(
            branches[10],
            eq58_value,
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq59_e1754, eq59_e1754_d_n0, eq59_e1754_d_n1, eq59_e1754_d_n2, eq59_e1754_d_n3, eq59_e1754_d_n4, eq59_e1754_d_n5, eq59_e1754_d_n6, eq59_e1754_d_n7, eq59_e1754_d_n8, eq59_e1754_d_n9, eq59_e1754_d_n10, eq59_e1754_d_n11, eq59_e1754_d_n12, eq59_e1754_d_n13,) = {
    if (!(s.v[1554] != 0.0)) {
        let eq59_e1750: f64 = (p.p32 * (nv1 - nv10));
        let eq59_e1750_d_n1: f64 = p.p32;
        let eq59_e1750_d_n10: f64 = (-p.p32);
        let eq59_e1752: f64 = (eq59_e1750 * s.v[64]);
        let eq59_e1752_d_n0: f64 = (eq59_e1750 * s.dn[64][0]);
        let eq59_e1752_d_n1: f64 = ((eq59_e1750_d_n1 * s.v[64]) + (eq59_e1750 * s.dn[64][1]));
        let eq59_e1752_d_n2: f64 = (eq59_e1750 * s.dn[64][2]);
        let eq59_e1752_d_n3: f64 = (eq59_e1750 * s.dn[64][3]);
        let eq59_e1752_d_n4: f64 = (eq59_e1750 * s.dn[64][4]);
        let eq59_e1752_d_n5: f64 = (eq59_e1750 * s.dn[64][5]);
        let eq59_e1752_d_n6: f64 = (eq59_e1750 * s.dn[64][6]);
        let eq59_e1752_d_n7: f64 = (eq59_e1750 * s.dn[64][7]);
        let eq59_e1752_d_n8: f64 = (eq59_e1750 * s.dn[64][8]);
        let eq59_e1752_d_n9: f64 = (eq59_e1750 * s.dn[64][9]);
        let eq59_e1752_d_n10: f64 = ((eq59_e1750_d_n10 * s.v[64]) + (eq59_e1750 * s.dn[64][10]));
        let eq59_e1752_d_n11: f64 = (eq59_e1750 * s.dn[64][11]);
        let eq59_e1752_d_n12: f64 = (eq59_e1750 * s.dn[64][12]);
        let eq59_e1752_d_n13: f64 = (eq59_e1750 * s.dn[64][13]);
        (eq59_e1752, eq59_e1752_d_n0, eq59_e1752_d_n1, eq59_e1752_d_n2, eq59_e1752_d_n3, eq59_e1752_d_n4, eq59_e1752_d_n5, eq59_e1752_d_n6, eq59_e1752_d_n7, eq59_e1752_d_n8, eq59_e1752_d_n9, eq59_e1752_d_n10, eq59_e1752_d_n11, eq59_e1752_d_n12, eq59_e1752_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1754;
        let eq59_node_derivatives: [f64; 14] = [eq59_e1754_d_n0, eq59_e1754_d_n1, eq59_e1754_d_n2, eq59_e1754_d_n3, eq59_e1754_d_n4, eq59_e1754_d_n5, eq59_e1754_d_n6, eq59_e1754_d_n7, eq59_e1754_d_n8, eq59_e1754_d_n9, eq59_e1754_d_n10, eq59_e1754_d_n11, eq59_e1754_d_n12, eq59_e1754_d_n13];
        let eq59_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[10]),
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
        let (eq60_e1766,) = {
    if (!(s.v[1554] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e1766;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[10]),
            self.multiplicity * (eq60_value),
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
        let (eq61_e1770,) = {
    if (s.v[1555] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e1770;
        stamper.stamp_potential(
            branches[11],
            eq61_value,
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq62_e1779, eq62_e1779_d_n0, eq62_e1779_d_n1, eq62_e1779_d_n2, eq62_e1779_d_n3, eq62_e1779_d_n4, eq62_e1779_d_n5, eq62_e1779_d_n6, eq62_e1779_d_n7, eq62_e1779_d_n8, eq62_e1779_d_n9, eq62_e1779_d_n10, eq62_e1779_d_n11, eq62_e1779_d_n12, eq62_e1779_d_n13,) = {
    if (!(s.v[1555] != 0.0)) {
        let eq62_e1775: f64 = (p.p32 * (nv10 - nv9));
        let eq62_e1775_d_n9: f64 = (-p.p32);
        let eq62_e1775_d_n10: f64 = p.p32;
        let eq62_e1777: f64 = (eq62_e1775 * s.v[81]);
        let eq62_e1777_d_n0: f64 = (eq62_e1775 * s.dn[81][0]);
        let eq62_e1777_d_n1: f64 = (eq62_e1775 * s.dn[81][1]);
        let eq62_e1777_d_n2: f64 = (eq62_e1775 * s.dn[81][2]);
        let eq62_e1777_d_n3: f64 = (eq62_e1775 * s.dn[81][3]);
        let eq62_e1777_d_n4: f64 = (eq62_e1775 * s.dn[81][4]);
        let eq62_e1777_d_n5: f64 = (eq62_e1775 * s.dn[81][5]);
        let eq62_e1777_d_n6: f64 = (eq62_e1775 * s.dn[81][6]);
        let eq62_e1777_d_n7: f64 = (eq62_e1775 * s.dn[81][7]);
        let eq62_e1777_d_n8: f64 = (eq62_e1775 * s.dn[81][8]);
        let eq62_e1777_d_n9: f64 = ((eq62_e1775_d_n9 * s.v[81]) + (eq62_e1775 * s.dn[81][9]));
        let eq62_e1777_d_n10: f64 = ((eq62_e1775_d_n10 * s.v[81]) + (eq62_e1775 * s.dn[81][10]));
        let eq62_e1777_d_n11: f64 = (eq62_e1775 * s.dn[81][11]);
        let eq62_e1777_d_n12: f64 = (eq62_e1775 * s.dn[81][12]);
        let eq62_e1777_d_n13: f64 = (eq62_e1775 * s.dn[81][13]);
        (eq62_e1777, eq62_e1777_d_n0, eq62_e1777_d_n1, eq62_e1777_d_n2, eq62_e1777_d_n3, eq62_e1777_d_n4, eq62_e1777_d_n5, eq62_e1777_d_n6, eq62_e1777_d_n7, eq62_e1777_d_n8, eq62_e1777_d_n9, eq62_e1777_d_n10, eq62_e1777_d_n11, eq62_e1777_d_n12, eq62_e1777_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1779;
        let eq62_node_derivatives: [f64; 14] = [eq62_e1779_d_n0, eq62_e1779_d_n1, eq62_e1779_d_n2, eq62_e1779_d_n3, eq62_e1779_d_n4, eq62_e1779_d_n5, eq62_e1779_d_n6, eq62_e1779_d_n7, eq62_e1779_d_n8, eq62_e1779_d_n9, eq62_e1779_d_n10, eq62_e1779_d_n11, eq62_e1779_d_n12, eq62_e1779_d_n13];
        let eq62_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[9]),
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
        let (eq63_e1797,) = {
    if ((!(s.v[1555] != 0.0)) && (s.v[1556] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e1797;
        stamper.stamp_current(
            Some(nodes[10]),
            Some(nodes[9]),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq64_e1805, eq64_e1805_d_n0, eq64_e1805_d_n1, eq64_e1805_d_n2, eq64_e1805_d_n3, eq64_e1805_d_n4, eq64_e1805_d_n5, eq64_e1805_d_n6, eq64_e1805_d_n7, eq64_e1805_d_n8, eq64_e1805_d_n9, eq64_e1805_d_n10, eq64_e1805_d_n11, eq64_e1805_d_n12, eq64_e1805_d_n13,) = {
    if (p.p40 != 0.0) {
        let eq64_e1801: f64 = (p.p32 * (nv5 - nv12));
        let eq64_e1801_d_n5: f64 = p.p32;
        let eq64_e1801_d_n12: f64 = (-p.p32);
        let eq64_e1803: f64 = (eq64_e1801 * s.v[65]);
        let eq64_e1803_d_n0: f64 = (eq64_e1801 * s.dn[65][0]);
        let eq64_e1803_d_n1: f64 = (eq64_e1801 * s.dn[65][1]);
        let eq64_e1803_d_n2: f64 = (eq64_e1801 * s.dn[65][2]);
        let eq64_e1803_d_n3: f64 = (eq64_e1801 * s.dn[65][3]);
        let eq64_e1803_d_n4: f64 = (eq64_e1801 * s.dn[65][4]);
        let eq64_e1803_d_n5: f64 = ((eq64_e1801_d_n5 * s.v[65]) + (eq64_e1801 * s.dn[65][5]));
        let eq64_e1803_d_n6: f64 = (eq64_e1801 * s.dn[65][6]);
        let eq64_e1803_d_n7: f64 = (eq64_e1801 * s.dn[65][7]);
        let eq64_e1803_d_n8: f64 = (eq64_e1801 * s.dn[65][8]);
        let eq64_e1803_d_n9: f64 = (eq64_e1801 * s.dn[65][9]);
        let eq64_e1803_d_n10: f64 = (eq64_e1801 * s.dn[65][10]);
        let eq64_e1803_d_n11: f64 = (eq64_e1801 * s.dn[65][11]);
        let eq64_e1803_d_n12: f64 = ((eq64_e1801_d_n12 * s.v[65]) + (eq64_e1801 * s.dn[65][12]));
        let eq64_e1803_d_n13: f64 = (eq64_e1801 * s.dn[65][13]);
        (eq64_e1803, eq64_e1803_d_n0, eq64_e1803_d_n1, eq64_e1803_d_n2, eq64_e1803_d_n3, eq64_e1803_d_n4, eq64_e1803_d_n5, eq64_e1803_d_n6, eq64_e1803_d_n7, eq64_e1803_d_n8, eq64_e1803_d_n9, eq64_e1803_d_n10, eq64_e1803_d_n11, eq64_e1803_d_n12, eq64_e1803_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e1805;
        let eq64_node_derivatives: [f64; 14] = [eq64_e1805_d_n0, eq64_e1805_d_n1, eq64_e1805_d_n2, eq64_e1805_d_n3, eq64_e1805_d_n4, eq64_e1805_d_n5, eq64_e1805_d_n6, eq64_e1805_d_n7, eq64_e1805_d_n8, eq64_e1805_d_n9, eq64_e1805_d_n10, eq64_e1805_d_n11, eq64_e1805_d_n12, eq64_e1805_d_n13];
        let eq64_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            self.multiplicity * (eq64_value),
            &nodes,
            &eq64_node_derivatives,
            &branches,
            &eq64_branch_derivatives,
            self.multiplicity,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq65_e1813, eq65_e1813_d_n0, eq65_e1813_d_n1, eq65_e1813_d_n2, eq65_e1813_d_n3, eq65_e1813_d_n4, eq65_e1813_d_n5, eq65_e1813_d_n6, eq65_e1813_d_n7, eq65_e1813_d_n8, eq65_e1813_d_n9, eq65_e1813_d_n10, eq65_e1813_d_n11, eq65_e1813_d_n12, eq65_e1813_d_n13,) = {
    if (p.p40 != 0.0) {
        let eq65_e1809: f64 = (p.p32 * (nv5 - nv11));
        let eq65_e1809_d_n5: f64 = p.p32;
        let eq65_e1809_d_n11: f64 = (-p.p32);
        let eq65_e1811: f64 = (eq65_e1809 * s.v[66]);
        let eq65_e1811_d_n0: f64 = (eq65_e1809 * s.dn[66][0]);
        let eq65_e1811_d_n1: f64 = (eq65_e1809 * s.dn[66][1]);
        let eq65_e1811_d_n2: f64 = (eq65_e1809 * s.dn[66][2]);
        let eq65_e1811_d_n3: f64 = (eq65_e1809 * s.dn[66][3]);
        let eq65_e1811_d_n4: f64 = (eq65_e1809 * s.dn[66][4]);
        let eq65_e1811_d_n5: f64 = ((eq65_e1809_d_n5 * s.v[66]) + (eq65_e1809 * s.dn[66][5]));
        let eq65_e1811_d_n6: f64 = (eq65_e1809 * s.dn[66][6]);
        let eq65_e1811_d_n7: f64 = (eq65_e1809 * s.dn[66][7]);
        let eq65_e1811_d_n8: f64 = (eq65_e1809 * s.dn[66][8]);
        let eq65_e1811_d_n9: f64 = (eq65_e1809 * s.dn[66][9]);
        let eq65_e1811_d_n10: f64 = (eq65_e1809 * s.dn[66][10]);
        let eq65_e1811_d_n11: f64 = ((eq65_e1809_d_n11 * s.v[66]) + (eq65_e1809 * s.dn[66][11]));
        let eq65_e1811_d_n12: f64 = (eq65_e1809 * s.dn[66][12]);
        let eq65_e1811_d_n13: f64 = (eq65_e1809 * s.dn[66][13]);
        (eq65_e1811, eq65_e1811_d_n0, eq65_e1811_d_n1, eq65_e1811_d_n2, eq65_e1811_d_n3, eq65_e1811_d_n4, eq65_e1811_d_n5, eq65_e1811_d_n6, eq65_e1811_d_n7, eq65_e1811_d_n8, eq65_e1811_d_n9, eq65_e1811_d_n10, eq65_e1811_d_n11, eq65_e1811_d_n12, eq65_e1811_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1813;
        let eq65_node_derivatives: [f64; 14] = [eq65_e1813_d_n0, eq65_e1813_d_n1, eq65_e1813_d_n2, eq65_e1813_d_n3, eq65_e1813_d_n4, eq65_e1813_d_n5, eq65_e1813_d_n6, eq65_e1813_d_n7, eq65_e1813_d_n8, eq65_e1813_d_n9, eq65_e1813_d_n10, eq65_e1813_d_n11, eq65_e1813_d_n12, eq65_e1813_d_n13];
        let eq65_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[11]),
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
        let (eq66_e1824,) = {
    if (p.p40 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e1824;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[12]),
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
        let (eq67_e1835,) = {
    if (p.p40 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1835;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[11]),
            self.multiplicity * (eq67_value),
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
        let (eq68_e1840,) = {
    if (!(p.p40 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1840;
        stamper.stamp_potential(
            branches[12],
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
        let (eq69_e1845,) = {
    if (!(p.p40 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq69_value: f64 = eq69_e1845;
        stamper.stamp_potential(
            branches[13],
            eq69_value,
            &[
            ],
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
        let (eq70_e1849,) = {
    if (s.v[1558] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e1849;
        stamper.stamp_potential(
            branches[14],
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
        let (eq71_e1869, eq71_e1869_d_n0, eq71_e1869_d_n1, eq71_e1869_d_n2, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12, eq71_e1869_d_n13,) = {
    if (((s.v[1559] != 0.0) && (s.v[1560] != 0.0)) && (s.v[1561] != 0.0)) {
        let eq71_e1856: f64 = (-s.v[885]);
        let eq71_e1856_d_n0: f64 = (-s.dn[885][0]);
        let eq71_e1856_d_n1: f64 = (-s.dn[885][1]);
        let eq71_e1856_d_n2: f64 = (-s.dn[885][2]);
        let eq71_e1856_d_n3: f64 = (-s.dn[885][3]);
        let eq71_e1856_d_n4: f64 = (-s.dn[885][4]);
        let eq71_e1856_d_n5: f64 = (-s.dn[885][5]);
        let eq71_e1856_d_n6: f64 = (-s.dn[885][6]);
        let eq71_e1856_d_n7: f64 = (-s.dn[885][7]);
        let eq71_e1856_d_n8: f64 = (-s.dn[885][8]);
        let eq71_e1856_d_n9: f64 = (-s.dn[885][9]);
        let eq71_e1856_d_n10: f64 = (-s.dn[885][10]);
        let eq71_e1856_d_n11: f64 = (-s.dn[885][11]);
        let eq71_e1856_d_n12: f64 = (-s.dn[885][12]);
        let eq71_e1856_d_n13: f64 = (-s.dn[885][13]);
        let eq71_e1858: f64 = (eq71_e1856 * s.v[822]);
        let eq71_e1858_d_n0: f64 = ((eq71_e1856_d_n0 * s.v[822]) + (eq71_e1856 * s.dn[822][0]));
        let eq71_e1858_d_n1: f64 = ((eq71_e1856_d_n1 * s.v[822]) + (eq71_e1856 * s.dn[822][1]));
        let eq71_e1858_d_n2: f64 = ((eq71_e1856_d_n2 * s.v[822]) + (eq71_e1856 * s.dn[822][2]));
        let eq71_e1858_d_n3: f64 = ((eq71_e1856_d_n3 * s.v[822]) + (eq71_e1856 * s.dn[822][3]));
        let eq71_e1858_d_n4: f64 = ((eq71_e1856_d_n4 * s.v[822]) + (eq71_e1856 * s.dn[822][4]));
        let eq71_e1858_d_n5: f64 = ((eq71_e1856_d_n5 * s.v[822]) + (eq71_e1856 * s.dn[822][5]));
        let eq71_e1858_d_n6: f64 = ((eq71_e1856_d_n6 * s.v[822]) + (eq71_e1856 * s.dn[822][6]));
        let eq71_e1858_d_n7: f64 = ((eq71_e1856_d_n7 * s.v[822]) + (eq71_e1856 * s.dn[822][7]));
        let eq71_e1858_d_n8: f64 = ((eq71_e1856_d_n8 * s.v[822]) + (eq71_e1856 * s.dn[822][8]));
        let eq71_e1858_d_n9: f64 = ((eq71_e1856_d_n9 * s.v[822]) + (eq71_e1856 * s.dn[822][9]));
        let eq71_e1858_d_n10: f64 = ((eq71_e1856_d_n10 * s.v[822]) + (eq71_e1856 * s.dn[822][10]));
        let eq71_e1858_d_n11: f64 = ((eq71_e1856_d_n11 * s.v[822]) + (eq71_e1856 * s.dn[822][11]));
        let eq71_e1858_d_n12: f64 = ((eq71_e1856_d_n12 * s.v[822]) + (eq71_e1856 * s.dn[822][12]));
        let eq71_e1858_d_n13: f64 = ((eq71_e1856_d_n13 * s.v[822]) + (eq71_e1856 * s.dn[822][13]));
        let eq71_e1861: f64 = (s.v[410] * s.v[158]);
        let eq71_e1861_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq71_e1861_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq71_e1861_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq71_e1861_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq71_e1861_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq71_e1861_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq71_e1861_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq71_e1861_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq71_e1861_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq71_e1861_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq71_e1861_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq71_e1861_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq71_e1861_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq71_e1861_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq71_e1862: f64 = self.eval_ddt(16, eq71_e1861);
        let eq71_e1862_d_n0: f64 = self.ddt_jacobian(eq71_e1861_d_n0);
        let eq71_e1862_d_n1: f64 = self.ddt_jacobian(eq71_e1861_d_n1);
        let eq71_e1862_d_n2: f64 = self.ddt_jacobian(eq71_e1861_d_n2);
        let eq71_e1862_d_n3: f64 = self.ddt_jacobian(eq71_e1861_d_n3);
        let eq71_e1862_d_n4: f64 = self.ddt_jacobian(eq71_e1861_d_n4);
        let eq71_e1862_d_n5: f64 = self.ddt_jacobian(eq71_e1861_d_n5);
        let eq71_e1862_d_n6: f64 = self.ddt_jacobian(eq71_e1861_d_n6);
        let eq71_e1862_d_n7: f64 = self.ddt_jacobian(eq71_e1861_d_n7);
        let eq71_e1862_d_n8: f64 = self.ddt_jacobian(eq71_e1861_d_n8);
        let eq71_e1862_d_n9: f64 = self.ddt_jacobian(eq71_e1861_d_n9);
        let eq71_e1862_d_n10: f64 = self.ddt_jacobian(eq71_e1861_d_n10);
        let eq71_e1862_d_n11: f64 = self.ddt_jacobian(eq71_e1861_d_n11);
        let eq71_e1862_d_n12: f64 = self.ddt_jacobian(eq71_e1861_d_n12);
        let eq71_e1862_d_n13: f64 = self.ddt_jacobian(eq71_e1861_d_n13);
        let eq71_e1863: f64 = (eq71_e1858 + eq71_e1862);
        let eq71_e1863_d_n0: f64 = (eq71_e1858_d_n0 + eq71_e1862_d_n0);
        let eq71_e1863_d_n1: f64 = (eq71_e1858_d_n1 + eq71_e1862_d_n1);
        let eq71_e1863_d_n2: f64 = (eq71_e1858_d_n2 + eq71_e1862_d_n2);
        let eq71_e1863_d_n3: f64 = (eq71_e1858_d_n3 + eq71_e1862_d_n3);
        let eq71_e1863_d_n4: f64 = (eq71_e1858_d_n4 + eq71_e1862_d_n4);
        let eq71_e1863_d_n5: f64 = (eq71_e1858_d_n5 + eq71_e1862_d_n5);
        let eq71_e1863_d_n6: f64 = (eq71_e1858_d_n6 + eq71_e1862_d_n6);
        let eq71_e1863_d_n7: f64 = (eq71_e1858_d_n7 + eq71_e1862_d_n7);
        let eq71_e1863_d_n8: f64 = (eq71_e1858_d_n8 + eq71_e1862_d_n8);
        let eq71_e1863_d_n9: f64 = (eq71_e1858_d_n9 + eq71_e1862_d_n9);
        let eq71_e1863_d_n10: f64 = (eq71_e1858_d_n10 + eq71_e1862_d_n10);
        let eq71_e1863_d_n11: f64 = (eq71_e1858_d_n11 + eq71_e1862_d_n11);
        let eq71_e1863_d_n12: f64 = (eq71_e1858_d_n12 + eq71_e1862_d_n12);
        let eq71_e1863_d_n13: f64 = (eq71_e1858_d_n13 + eq71_e1862_d_n13);
        let eq71_e1866: f64 = (s.v[410] / s.v[157]);
        let eq71_e1866_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq71_e1866_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq71_e1866_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq71_e1866_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq71_e1866_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq71_e1866_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq71_e1866_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq71_e1866_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq71_e1866_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq71_e1866_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq71_e1866_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq71_e1866_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq71_e1866_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq71_e1866_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq71_e1867: f64 = (eq71_e1863 + eq71_e1866);
        let eq71_e1867_d_n0: f64 = (eq71_e1863_d_n0 + eq71_e1866_d_n0);
        let eq71_e1867_d_n1: f64 = (eq71_e1863_d_n1 + eq71_e1866_d_n1);
        let eq71_e1867_d_n2: f64 = (eq71_e1863_d_n2 + eq71_e1866_d_n2);
        let eq71_e1867_d_n3: f64 = (eq71_e1863_d_n3 + eq71_e1866_d_n3);
        let eq71_e1867_d_n4: f64 = (eq71_e1863_d_n4 + eq71_e1866_d_n4);
        let eq71_e1867_d_n5: f64 = (eq71_e1863_d_n5 + eq71_e1866_d_n5);
        let eq71_e1867_d_n6: f64 = (eq71_e1863_d_n6 + eq71_e1866_d_n6);
        let eq71_e1867_d_n7: f64 = (eq71_e1863_d_n7 + eq71_e1866_d_n7);
        let eq71_e1867_d_n8: f64 = (eq71_e1863_d_n8 + eq71_e1866_d_n8);
        let eq71_e1867_d_n9: f64 = (eq71_e1863_d_n9 + eq71_e1866_d_n9);
        let eq71_e1867_d_n10: f64 = (eq71_e1863_d_n10 + eq71_e1866_d_n10);
        let eq71_e1867_d_n11: f64 = (eq71_e1863_d_n11 + eq71_e1866_d_n11);
        let eq71_e1867_d_n12: f64 = (eq71_e1863_d_n12 + eq71_e1866_d_n12);
        let eq71_e1867_d_n13: f64 = (eq71_e1863_d_n13 + eq71_e1866_d_n13);
        (eq71_e1867, eq71_e1867_d_n0, eq71_e1867_d_n1, eq71_e1867_d_n2, eq71_e1867_d_n3, eq71_e1867_d_n4, eq71_e1867_d_n5, eq71_e1867_d_n6, eq71_e1867_d_n7, eq71_e1867_d_n8, eq71_e1867_d_n9, eq71_e1867_d_n10, eq71_e1867_d_n11, eq71_e1867_d_n12, eq71_e1867_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e1869;
        let eq71_node_derivatives: [f64; 14] = [eq71_e1869_d_n0, eq71_e1869_d_n1, eq71_e1869_d_n2, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12, eq71_e1869_d_n13];
        let eq71_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq71_value),
            &nodes,
            &eq71_node_derivatives,
            &branches,
            &eq71_branch_derivatives,
            self.multiplicity,
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
        let (eq72_e1892, eq72_e1892_d_n0, eq72_e1892_d_n1, eq72_e1892_d_n2, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12, eq72_e1892_d_n13,) = {
    if ((((s.v[1559] != 0.0) && (s.v[1560] != 0.0)) && (!(s.v[1561] != 0.0))) && (s.v[1562] != 0.0)) {
        let eq72_e1879: f64 = (-s.v[885]);
        let eq72_e1879_d_n0: f64 = (-s.dn[885][0]);
        let eq72_e1879_d_n1: f64 = (-s.dn[885][1]);
        let eq72_e1879_d_n2: f64 = (-s.dn[885][2]);
        let eq72_e1879_d_n3: f64 = (-s.dn[885][3]);
        let eq72_e1879_d_n4: f64 = (-s.dn[885][4]);
        let eq72_e1879_d_n5: f64 = (-s.dn[885][5]);
        let eq72_e1879_d_n6: f64 = (-s.dn[885][6]);
        let eq72_e1879_d_n7: f64 = (-s.dn[885][7]);
        let eq72_e1879_d_n8: f64 = (-s.dn[885][8]);
        let eq72_e1879_d_n9: f64 = (-s.dn[885][9]);
        let eq72_e1879_d_n10: f64 = (-s.dn[885][10]);
        let eq72_e1879_d_n11: f64 = (-s.dn[885][11]);
        let eq72_e1879_d_n12: f64 = (-s.dn[885][12]);
        let eq72_e1879_d_n13: f64 = (-s.dn[885][13]);
        let eq72_e1881: f64 = (eq72_e1879 * s.v[822]);
        let eq72_e1881_d_n0: f64 = ((eq72_e1879_d_n0 * s.v[822]) + (eq72_e1879 * s.dn[822][0]));
        let eq72_e1881_d_n1: f64 = ((eq72_e1879_d_n1 * s.v[822]) + (eq72_e1879 * s.dn[822][1]));
        let eq72_e1881_d_n2: f64 = ((eq72_e1879_d_n2 * s.v[822]) + (eq72_e1879 * s.dn[822][2]));
        let eq72_e1881_d_n3: f64 = ((eq72_e1879_d_n3 * s.v[822]) + (eq72_e1879 * s.dn[822][3]));
        let eq72_e1881_d_n4: f64 = ((eq72_e1879_d_n4 * s.v[822]) + (eq72_e1879 * s.dn[822][4]));
        let eq72_e1881_d_n5: f64 = ((eq72_e1879_d_n5 * s.v[822]) + (eq72_e1879 * s.dn[822][5]));
        let eq72_e1881_d_n6: f64 = ((eq72_e1879_d_n6 * s.v[822]) + (eq72_e1879 * s.dn[822][6]));
        let eq72_e1881_d_n7: f64 = ((eq72_e1879_d_n7 * s.v[822]) + (eq72_e1879 * s.dn[822][7]));
        let eq72_e1881_d_n8: f64 = ((eq72_e1879_d_n8 * s.v[822]) + (eq72_e1879 * s.dn[822][8]));
        let eq72_e1881_d_n9: f64 = ((eq72_e1879_d_n9 * s.v[822]) + (eq72_e1879 * s.dn[822][9]));
        let eq72_e1881_d_n10: f64 = ((eq72_e1879_d_n10 * s.v[822]) + (eq72_e1879 * s.dn[822][10]));
        let eq72_e1881_d_n11: f64 = ((eq72_e1879_d_n11 * s.v[822]) + (eq72_e1879 * s.dn[822][11]));
        let eq72_e1881_d_n12: f64 = ((eq72_e1879_d_n12 * s.v[822]) + (eq72_e1879 * s.dn[822][12]));
        let eq72_e1881_d_n13: f64 = ((eq72_e1879_d_n13 * s.v[822]) + (eq72_e1879 * s.dn[822][13]));
        let eq72_e1884: f64 = (s.v[410] * s.v[158]);
        let eq72_e1884_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq72_e1884_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq72_e1884_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq72_e1884_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq72_e1884_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq72_e1884_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq72_e1884_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq72_e1884_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq72_e1884_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq72_e1884_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq72_e1884_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq72_e1884_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq72_e1884_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq72_e1884_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq72_e1885: f64 = self.eval_ddt(17, eq72_e1884);
        let eq72_e1885_d_n0: f64 = self.ddt_jacobian(eq72_e1884_d_n0);
        let eq72_e1885_d_n1: f64 = self.ddt_jacobian(eq72_e1884_d_n1);
        let eq72_e1885_d_n2: f64 = self.ddt_jacobian(eq72_e1884_d_n2);
        let eq72_e1885_d_n3: f64 = self.ddt_jacobian(eq72_e1884_d_n3);
        let eq72_e1885_d_n4: f64 = self.ddt_jacobian(eq72_e1884_d_n4);
        let eq72_e1885_d_n5: f64 = self.ddt_jacobian(eq72_e1884_d_n5);
        let eq72_e1885_d_n6: f64 = self.ddt_jacobian(eq72_e1884_d_n6);
        let eq72_e1885_d_n7: f64 = self.ddt_jacobian(eq72_e1884_d_n7);
        let eq72_e1885_d_n8: f64 = self.ddt_jacobian(eq72_e1884_d_n8);
        let eq72_e1885_d_n9: f64 = self.ddt_jacobian(eq72_e1884_d_n9);
        let eq72_e1885_d_n10: f64 = self.ddt_jacobian(eq72_e1884_d_n10);
        let eq72_e1885_d_n11: f64 = self.ddt_jacobian(eq72_e1884_d_n11);
        let eq72_e1885_d_n12: f64 = self.ddt_jacobian(eq72_e1884_d_n12);
        let eq72_e1885_d_n13: f64 = self.ddt_jacobian(eq72_e1884_d_n13);
        let eq72_e1886: f64 = (eq72_e1881 + eq72_e1885);
        let eq72_e1886_d_n0: f64 = (eq72_e1881_d_n0 + eq72_e1885_d_n0);
        let eq72_e1886_d_n1: f64 = (eq72_e1881_d_n1 + eq72_e1885_d_n1);
        let eq72_e1886_d_n2: f64 = (eq72_e1881_d_n2 + eq72_e1885_d_n2);
        let eq72_e1886_d_n3: f64 = (eq72_e1881_d_n3 + eq72_e1885_d_n3);
        let eq72_e1886_d_n4: f64 = (eq72_e1881_d_n4 + eq72_e1885_d_n4);
        let eq72_e1886_d_n5: f64 = (eq72_e1881_d_n5 + eq72_e1885_d_n5);
        let eq72_e1886_d_n6: f64 = (eq72_e1881_d_n6 + eq72_e1885_d_n6);
        let eq72_e1886_d_n7: f64 = (eq72_e1881_d_n7 + eq72_e1885_d_n7);
        let eq72_e1886_d_n8: f64 = (eq72_e1881_d_n8 + eq72_e1885_d_n8);
        let eq72_e1886_d_n9: f64 = (eq72_e1881_d_n9 + eq72_e1885_d_n9);
        let eq72_e1886_d_n10: f64 = (eq72_e1881_d_n10 + eq72_e1885_d_n10);
        let eq72_e1886_d_n11: f64 = (eq72_e1881_d_n11 + eq72_e1885_d_n11);
        let eq72_e1886_d_n12: f64 = (eq72_e1881_d_n12 + eq72_e1885_d_n12);
        let eq72_e1886_d_n13: f64 = (eq72_e1881_d_n13 + eq72_e1885_d_n13);
        let eq72_e1889: f64 = (s.v[410] / s.v[157]);
        let eq72_e1889_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq72_e1889_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq72_e1889_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq72_e1889_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq72_e1889_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq72_e1889_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq72_e1889_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq72_e1889_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq72_e1889_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq72_e1889_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq72_e1889_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq72_e1889_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq72_e1889_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq72_e1889_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq72_e1890: f64 = (eq72_e1886 + eq72_e1889);
        let eq72_e1890_d_n0: f64 = (eq72_e1886_d_n0 + eq72_e1889_d_n0);
        let eq72_e1890_d_n1: f64 = (eq72_e1886_d_n1 + eq72_e1889_d_n1);
        let eq72_e1890_d_n2: f64 = (eq72_e1886_d_n2 + eq72_e1889_d_n2);
        let eq72_e1890_d_n3: f64 = (eq72_e1886_d_n3 + eq72_e1889_d_n3);
        let eq72_e1890_d_n4: f64 = (eq72_e1886_d_n4 + eq72_e1889_d_n4);
        let eq72_e1890_d_n5: f64 = (eq72_e1886_d_n5 + eq72_e1889_d_n5);
        let eq72_e1890_d_n6: f64 = (eq72_e1886_d_n6 + eq72_e1889_d_n6);
        let eq72_e1890_d_n7: f64 = (eq72_e1886_d_n7 + eq72_e1889_d_n7);
        let eq72_e1890_d_n8: f64 = (eq72_e1886_d_n8 + eq72_e1889_d_n8);
        let eq72_e1890_d_n9: f64 = (eq72_e1886_d_n9 + eq72_e1889_d_n9);
        let eq72_e1890_d_n10: f64 = (eq72_e1886_d_n10 + eq72_e1889_d_n10);
        let eq72_e1890_d_n11: f64 = (eq72_e1886_d_n11 + eq72_e1889_d_n11);
        let eq72_e1890_d_n12: f64 = (eq72_e1886_d_n12 + eq72_e1889_d_n12);
        let eq72_e1890_d_n13: f64 = (eq72_e1886_d_n13 + eq72_e1889_d_n13);
        (eq72_e1890, eq72_e1890_d_n0, eq72_e1890_d_n1, eq72_e1890_d_n2, eq72_e1890_d_n3, eq72_e1890_d_n4, eq72_e1890_d_n5, eq72_e1890_d_n6, eq72_e1890_d_n7, eq72_e1890_d_n8, eq72_e1890_d_n9, eq72_e1890_d_n10, eq72_e1890_d_n11, eq72_e1890_d_n12, eq72_e1890_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1892;
        let eq72_node_derivatives: [f64; 14] = [eq72_e1892_d_n0, eq72_e1892_d_n1, eq72_e1892_d_n2, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12, eq72_e1892_d_n13];
        let eq72_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq72_value),
            &nodes,
            &eq72_node_derivatives,
            &branches,
            &eq72_branch_derivatives,
            self.multiplicity,
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
        let (eq73_e1920, eq73_e1920_d_n0, eq73_e1920_d_n1, eq73_e1920_d_n2, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12, eq73_e1920_d_n13,) = {
    if (((((s.v[1559] != 0.0) && (s.v[1560] != 0.0)) && (!(s.v[1561] != 0.0))) && (!(s.v[1562] != 0.0))) && (s.v[1563] != 0.0)) {
        let eq73_e1906: f64 = (s.v[885] / p.p30);
        let eq73_e1906_d_n0: f64 = (s.dn[885][0] / p.p30);
        let eq73_e1906_d_n1: f64 = (s.dn[885][1] / p.p30);
        let eq73_e1906_d_n2: f64 = (s.dn[885][2] / p.p30);
        let eq73_e1906_d_n3: f64 = (s.dn[885][3] / p.p30);
        let eq73_e1906_d_n4: f64 = (s.dn[885][4] / p.p30);
        let eq73_e1906_d_n5: f64 = (s.dn[885][5] / p.p30);
        let eq73_e1906_d_n6: f64 = (s.dn[885][6] / p.p30);
        let eq73_e1906_d_n7: f64 = (s.dn[885][7] / p.p30);
        let eq73_e1906_d_n8: f64 = (s.dn[885][8] / p.p30);
        let eq73_e1906_d_n9: f64 = (s.dn[885][9] / p.p30);
        let eq73_e1906_d_n10: f64 = (s.dn[885][10] / p.p30);
        let eq73_e1906_d_n11: f64 = (s.dn[885][11] / p.p30);
        let eq73_e1906_d_n12: f64 = (s.dn[885][12] / p.p30);
        let eq73_e1906_d_n13: f64 = (s.dn[885][13] / p.p30);
        let eq73_e1907: f64 = (-eq73_e1906);
        let eq73_e1907_d_n0: f64 = (-eq73_e1906_d_n0);
        let eq73_e1907_d_n1: f64 = (-eq73_e1906_d_n1);
        let eq73_e1907_d_n2: f64 = (-eq73_e1906_d_n2);
        let eq73_e1907_d_n3: f64 = (-eq73_e1906_d_n3);
        let eq73_e1907_d_n4: f64 = (-eq73_e1906_d_n4);
        let eq73_e1907_d_n5: f64 = (-eq73_e1906_d_n5);
        let eq73_e1907_d_n6: f64 = (-eq73_e1906_d_n6);
        let eq73_e1907_d_n7: f64 = (-eq73_e1906_d_n7);
        let eq73_e1907_d_n8: f64 = (-eq73_e1906_d_n8);
        let eq73_e1907_d_n9: f64 = (-eq73_e1906_d_n9);
        let eq73_e1907_d_n10: f64 = (-eq73_e1906_d_n10);
        let eq73_e1907_d_n11: f64 = (-eq73_e1906_d_n11);
        let eq73_e1907_d_n12: f64 = (-eq73_e1906_d_n12);
        let eq73_e1907_d_n13: f64 = (-eq73_e1906_d_n13);
        let eq73_e1909: f64 = (eq73_e1907 * s.v[822]);
        let eq73_e1909_d_n0: f64 = ((eq73_e1907_d_n0 * s.v[822]) + (eq73_e1907 * s.dn[822][0]));
        let eq73_e1909_d_n1: f64 = ((eq73_e1907_d_n1 * s.v[822]) + (eq73_e1907 * s.dn[822][1]));
        let eq73_e1909_d_n2: f64 = ((eq73_e1907_d_n2 * s.v[822]) + (eq73_e1907 * s.dn[822][2]));
        let eq73_e1909_d_n3: f64 = ((eq73_e1907_d_n3 * s.v[822]) + (eq73_e1907 * s.dn[822][3]));
        let eq73_e1909_d_n4: f64 = ((eq73_e1907_d_n4 * s.v[822]) + (eq73_e1907 * s.dn[822][4]));
        let eq73_e1909_d_n5: f64 = ((eq73_e1907_d_n5 * s.v[822]) + (eq73_e1907 * s.dn[822][5]));
        let eq73_e1909_d_n6: f64 = ((eq73_e1907_d_n6 * s.v[822]) + (eq73_e1907 * s.dn[822][6]));
        let eq73_e1909_d_n7: f64 = ((eq73_e1907_d_n7 * s.v[822]) + (eq73_e1907 * s.dn[822][7]));
        let eq73_e1909_d_n8: f64 = ((eq73_e1907_d_n8 * s.v[822]) + (eq73_e1907 * s.dn[822][8]));
        let eq73_e1909_d_n9: f64 = ((eq73_e1907_d_n9 * s.v[822]) + (eq73_e1907 * s.dn[822][9]));
        let eq73_e1909_d_n10: f64 = ((eq73_e1907_d_n10 * s.v[822]) + (eq73_e1907 * s.dn[822][10]));
        let eq73_e1909_d_n11: f64 = ((eq73_e1907_d_n11 * s.v[822]) + (eq73_e1907 * s.dn[822][11]));
        let eq73_e1909_d_n12: f64 = ((eq73_e1907_d_n12 * s.v[822]) + (eq73_e1907 * s.dn[822][12]));
        let eq73_e1909_d_n13: f64 = ((eq73_e1907_d_n13 * s.v[822]) + (eq73_e1907 * s.dn[822][13]));
        let eq73_e1912: f64 = (s.v[410] * s.v[158]);
        let eq73_e1912_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq73_e1912_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq73_e1912_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq73_e1912_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq73_e1912_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq73_e1912_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq73_e1912_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq73_e1912_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq73_e1912_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq73_e1912_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq73_e1912_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq73_e1912_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq73_e1912_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq73_e1912_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq73_e1913: f64 = self.eval_ddt(18, eq73_e1912);
        let eq73_e1913_d_n0: f64 = self.ddt_jacobian(eq73_e1912_d_n0);
        let eq73_e1913_d_n1: f64 = self.ddt_jacobian(eq73_e1912_d_n1);
        let eq73_e1913_d_n2: f64 = self.ddt_jacobian(eq73_e1912_d_n2);
        let eq73_e1913_d_n3: f64 = self.ddt_jacobian(eq73_e1912_d_n3);
        let eq73_e1913_d_n4: f64 = self.ddt_jacobian(eq73_e1912_d_n4);
        let eq73_e1913_d_n5: f64 = self.ddt_jacobian(eq73_e1912_d_n5);
        let eq73_e1913_d_n6: f64 = self.ddt_jacobian(eq73_e1912_d_n6);
        let eq73_e1913_d_n7: f64 = self.ddt_jacobian(eq73_e1912_d_n7);
        let eq73_e1913_d_n8: f64 = self.ddt_jacobian(eq73_e1912_d_n8);
        let eq73_e1913_d_n9: f64 = self.ddt_jacobian(eq73_e1912_d_n9);
        let eq73_e1913_d_n10: f64 = self.ddt_jacobian(eq73_e1912_d_n10);
        let eq73_e1913_d_n11: f64 = self.ddt_jacobian(eq73_e1912_d_n11);
        let eq73_e1913_d_n12: f64 = self.ddt_jacobian(eq73_e1912_d_n12);
        let eq73_e1913_d_n13: f64 = self.ddt_jacobian(eq73_e1912_d_n13);
        let eq73_e1914: f64 = (eq73_e1909 + eq73_e1913);
        let eq73_e1914_d_n0: f64 = (eq73_e1909_d_n0 + eq73_e1913_d_n0);
        let eq73_e1914_d_n1: f64 = (eq73_e1909_d_n1 + eq73_e1913_d_n1);
        let eq73_e1914_d_n2: f64 = (eq73_e1909_d_n2 + eq73_e1913_d_n2);
        let eq73_e1914_d_n3: f64 = (eq73_e1909_d_n3 + eq73_e1913_d_n3);
        let eq73_e1914_d_n4: f64 = (eq73_e1909_d_n4 + eq73_e1913_d_n4);
        let eq73_e1914_d_n5: f64 = (eq73_e1909_d_n5 + eq73_e1913_d_n5);
        let eq73_e1914_d_n6: f64 = (eq73_e1909_d_n6 + eq73_e1913_d_n6);
        let eq73_e1914_d_n7: f64 = (eq73_e1909_d_n7 + eq73_e1913_d_n7);
        let eq73_e1914_d_n8: f64 = (eq73_e1909_d_n8 + eq73_e1913_d_n8);
        let eq73_e1914_d_n9: f64 = (eq73_e1909_d_n9 + eq73_e1913_d_n9);
        let eq73_e1914_d_n10: f64 = (eq73_e1909_d_n10 + eq73_e1913_d_n10);
        let eq73_e1914_d_n11: f64 = (eq73_e1909_d_n11 + eq73_e1913_d_n11);
        let eq73_e1914_d_n12: f64 = (eq73_e1909_d_n12 + eq73_e1913_d_n12);
        let eq73_e1914_d_n13: f64 = (eq73_e1909_d_n13 + eq73_e1913_d_n13);
        let eq73_e1917: f64 = (s.v[410] / s.v[157]);
        let eq73_e1917_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq73_e1917_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq73_e1917_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq73_e1917_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq73_e1917_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq73_e1917_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq73_e1917_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq73_e1917_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq73_e1917_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq73_e1917_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq73_e1917_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq73_e1917_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq73_e1917_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq73_e1917_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq73_e1918: f64 = (eq73_e1914 + eq73_e1917);
        let eq73_e1918_d_n0: f64 = (eq73_e1914_d_n0 + eq73_e1917_d_n0);
        let eq73_e1918_d_n1: f64 = (eq73_e1914_d_n1 + eq73_e1917_d_n1);
        let eq73_e1918_d_n2: f64 = (eq73_e1914_d_n2 + eq73_e1917_d_n2);
        let eq73_e1918_d_n3: f64 = (eq73_e1914_d_n3 + eq73_e1917_d_n3);
        let eq73_e1918_d_n4: f64 = (eq73_e1914_d_n4 + eq73_e1917_d_n4);
        let eq73_e1918_d_n5: f64 = (eq73_e1914_d_n5 + eq73_e1917_d_n5);
        let eq73_e1918_d_n6: f64 = (eq73_e1914_d_n6 + eq73_e1917_d_n6);
        let eq73_e1918_d_n7: f64 = (eq73_e1914_d_n7 + eq73_e1917_d_n7);
        let eq73_e1918_d_n8: f64 = (eq73_e1914_d_n8 + eq73_e1917_d_n8);
        let eq73_e1918_d_n9: f64 = (eq73_e1914_d_n9 + eq73_e1917_d_n9);
        let eq73_e1918_d_n10: f64 = (eq73_e1914_d_n10 + eq73_e1917_d_n10);
        let eq73_e1918_d_n11: f64 = (eq73_e1914_d_n11 + eq73_e1917_d_n11);
        let eq73_e1918_d_n12: f64 = (eq73_e1914_d_n12 + eq73_e1917_d_n12);
        let eq73_e1918_d_n13: f64 = (eq73_e1914_d_n13 + eq73_e1917_d_n13);
        (eq73_e1918, eq73_e1918_d_n0, eq73_e1918_d_n1, eq73_e1918_d_n2, eq73_e1918_d_n3, eq73_e1918_d_n4, eq73_e1918_d_n5, eq73_e1918_d_n6, eq73_e1918_d_n7, eq73_e1918_d_n8, eq73_e1918_d_n9, eq73_e1918_d_n10, eq73_e1918_d_n11, eq73_e1918_d_n12, eq73_e1918_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1920;
        let eq73_node_derivatives: [f64; 14] = [eq73_e1920_d_n0, eq73_e1920_d_n1, eq73_e1920_d_n2, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12, eq73_e1920_d_n13];
        let eq73_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            None,
            self.multiplicity * (eq73_value),
            &nodes,
            &eq73_node_derivatives,
            &branches,
            &eq73_branch_derivatives,
            self.multiplicity,
        );
    }
}
