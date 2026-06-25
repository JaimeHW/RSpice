#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq12_e191,) = {
    if (p.p13 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e191;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[4]),
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
        let (eq13_e198,) = {
    if (p.p13 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e198;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[4]),
            self.multiplicity * (eq13_value),
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
        let (eq14_e210,) = {
    if (p.p13 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq14_value: f64 = eq14_e210;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[4]),
            self.multiplicity * (eq14_value),
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
        let (eq15_e222,) = {
    if (p.p13 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e222;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[5]),
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
        let (eq16_e242,) = {
    if ((p.p13 != 0.0) && (s.v[326] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e242;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[4]),
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
        let (eq17_e262,) = {
    if ((p.p13 != 0.0) && (s.v[327] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e262;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq17_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq9_e181_q: f64 = s.v[96];
        let eq9_reactive_node_derivatives: [f64; 6] = [s.dn[96][0], s.dn[96][1], s.dn[96][2], s.dn[96][3], s.dn[96][4], s.dn[96][5]];
        let eq9_reactive_branch_derivatives: [f64; 2] = [s.db[96][0], s.db[96][1]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            &nodes,
            &eq9_reactive_node_derivatives,
            &branches,
            &eq9_reactive_branch_derivatives,
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
        let eq10_e183_q: f64 = s.v[97];
        let eq10_reactive_node_derivatives: [f64; 6] = [s.dn[97][0], s.dn[97][1], s.dn[97][2], s.dn[97][3], s.dn[97][4], s.dn[97][5]];
        let eq10_reactive_branch_derivatives: [f64; 2] = [s.db[97][0], s.db[97][1]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
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
        let eq11_e185_q: f64 = s.v[98];
        let eq11_reactive_node_derivatives: [f64; 6] = [s.dn[98][0], s.dn[98][1], s.dn[98][2], s.dn[98][3], s.dn[98][4], s.dn[98][5]];
        let eq11_reactive_branch_derivatives: [f64; 2] = [s.db[98][0], s.db[98][1]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            None,
            &nodes,
            &eq11_reactive_node_derivatives,
            &branches,
            &eq11_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
