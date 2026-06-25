#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq40_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[9]),
            self.multiplicity * (eq40_value),
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
        let eq41_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[9]),
            self.multiplicity * (eq41_value),
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
        let eq42_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[8]),
            self.multiplicity * (eq42_value),
            &[
            ],
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
        let eq43_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[1]),
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
        let (eq44_e443,) = {
    if (s.v[565] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq44_value: f64 = eq44_e443;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[5]),
            self.multiplicity * (eq44_value),
            &[
            ],
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
        let (eq45_e452,) = {
    if (!(s.v[565] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq45_value: f64 = eq45_e452;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[5]),
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
        let (eq46_e462,) = {
    if ((s.v[566] != 0.0) && (s.v[567] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq46_value: f64 = eq46_e462;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[8]),
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
        let (eq47_e472,) = {
    if ((s.v[566] != 0.0) && (s.v[567] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e472;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[9]),
            self.multiplicity * (eq47_value),
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
        let (eq48_e482,) = {
    if ((s.v[566] != 0.0) && (s.v[567] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e482;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[6]),
            self.multiplicity * (eq48_value),
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
        let (eq49_e493,) = {
    if ((s.v[566] != 0.0) && (!(s.v[567] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e493;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[8]),
            self.multiplicity * (eq49_value),
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
        let (eq50_e504,) = {
    if ((s.v[566] != 0.0) && (!(s.v[567] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e504;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq50_value),
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
        let (eq51_e515,) = {
    if ((!(s.v[566] != 0.0)) && (s.v[568] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq51_value: f64 = eq51_e515;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[9]),
            self.multiplicity * (eq51_value),
            &[
            ],
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
        let (eq52_e526,) = {
    if ((!(s.v[566] != 0.0)) && (s.v[568] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e526;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[6]),
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
        let (eq53_e538,) = {
    if ((!(s.v[566] != 0.0)) && (!(s.v[568] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e538;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[6]),
            self.multiplicity * (eq53_value),
            &[
            ],
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
        let eq10_e233: f64 = (s.v[204] + s.v[209]);
        let eq10_e233_d_n0: f64 = (s.dn[204][0] + s.dn[209][0]);
        let eq10_e233_d_n1: f64 = (s.dn[204][1] + s.dn[209][1]);
        let eq10_e233_d_n2: f64 = (s.dn[204][2] + s.dn[209][2]);
        let eq10_e233_d_n3: f64 = (s.dn[204][3] + s.dn[209][3]);
        let eq10_e233_d_n4: f64 = (s.dn[204][4] + s.dn[209][4]);
        let eq10_e233_d_n5: f64 = (s.dn[204][5] + s.dn[209][5]);
        let eq10_e233_d_n6: f64 = (s.dn[204][6] + s.dn[209][6]);
        let eq10_e233_d_n7: f64 = (s.dn[204][7] + s.dn[209][7]);
        let eq10_e233_d_n8: f64 = (s.dn[204][8] + s.dn[209][8]);
        let eq10_e233_d_n9: f64 = (s.dn[204][9] + s.dn[209][9]);
        let eq10_e233_d_n10: f64 = (s.dn[204][10] + s.dn[209][10]);
        let eq10_e235: f64 = (eq10_e233 + s.v[221]);
        let eq10_e235_d_n0: f64 = (eq10_e233_d_n0 + s.dn[221][0]);
        let eq10_e235_d_n1: f64 = (eq10_e233_d_n1 + s.dn[221][1]);
        let eq10_e235_d_n2: f64 = (eq10_e233_d_n2 + s.dn[221][2]);
        let eq10_e235_d_n3: f64 = (eq10_e233_d_n3 + s.dn[221][3]);
        let eq10_e235_d_n4: f64 = (eq10_e233_d_n4 + s.dn[221][4]);
        let eq10_e235_d_n5: f64 = (eq10_e233_d_n5 + s.dn[221][5]);
        let eq10_e235_d_n6: f64 = (eq10_e233_d_n6 + s.dn[221][6]);
        let eq10_e235_d_n7: f64 = (eq10_e233_d_n7 + s.dn[221][7]);
        let eq10_e235_d_n8: f64 = (eq10_e233_d_n8 + s.dn[221][8]);
        let eq10_e235_d_n9: f64 = (eq10_e233_d_n9 + s.dn[221][9]);
        let eq10_e235_d_n10: f64 = (eq10_e233_d_n10 + s.dn[221][10]);
        let eq10_e236: f64 = (p.p3 * eq10_e235);
        let eq10_e236_d_n0: f64 = (p.p3 * eq10_e235_d_n0);
        let eq10_e236_d_n1: f64 = (p.p3 * eq10_e235_d_n1);
        let eq10_e236_d_n2: f64 = (p.p3 * eq10_e235_d_n2);
        let eq10_e236_d_n3: f64 = (p.p3 * eq10_e235_d_n3);
        let eq10_e236_d_n4: f64 = (p.p3 * eq10_e235_d_n4);
        let eq10_e236_d_n5: f64 = (p.p3 * eq10_e235_d_n5);
        let eq10_e236_d_n6: f64 = (p.p3 * eq10_e235_d_n6);
        let eq10_e236_d_n7: f64 = (p.p3 * eq10_e235_d_n7);
        let eq10_e236_d_n8: f64 = (p.p3 * eq10_e235_d_n8);
        let eq10_e236_d_n9: f64 = (p.p3 * eq10_e235_d_n9);
        let eq10_e236_d_n10: f64 = (p.p3 * eq10_e235_d_n10);
        let eq10_e237_q: f64 = eq10_e236;
        let eq10_e239: f64 = (eq10_e236 * p.p1);
        let eq10_e239_d_n0: f64 = (eq10_e236_d_n0 * p.p1);
        let eq10_e239_d_n1: f64 = (eq10_e236_d_n1 * p.p1);
        let eq10_e239_d_n2: f64 = (eq10_e236_d_n2 * p.p1);
        let eq10_e239_d_n3: f64 = (eq10_e236_d_n3 * p.p1);
        let eq10_e239_d_n4: f64 = (eq10_e236_d_n4 * p.p1);
        let eq10_e239_d_n5: f64 = (eq10_e236_d_n5 * p.p1);
        let eq10_e239_d_n6: f64 = (eq10_e236_d_n6 * p.p1);
        let eq10_e239_d_n7: f64 = (eq10_e236_d_n7 * p.p1);
        let eq10_e239_d_n8: f64 = (eq10_e236_d_n8 * p.p1);
        let eq10_e239_d_n9: f64 = (eq10_e236_d_n9 * p.p1);
        let eq10_e239_d_n10: f64 = (eq10_e236_d_n10 * p.p1);
        let eq10_e239_q: f64 = (eq10_e237_q * p.p1);
        let eq10_e239_q_d_n0: f64 = (eq10_e236_d_n0 * p.p1);
        let eq10_e239_q_d_n1: f64 = (eq10_e236_d_n1 * p.p1);
        let eq10_e239_q_d_n2: f64 = (eq10_e236_d_n2 * p.p1);
        let eq10_e239_q_d_n3: f64 = (eq10_e236_d_n3 * p.p1);
        let eq10_e239_q_d_n4: f64 = (eq10_e236_d_n4 * p.p1);
        let eq10_e239_q_d_n5: f64 = (eq10_e236_d_n5 * p.p1);
        let eq10_e239_q_d_n6: f64 = (eq10_e236_d_n6 * p.p1);
        let eq10_e239_q_d_n7: f64 = (eq10_e236_d_n7 * p.p1);
        let eq10_e239_q_d_n8: f64 = (eq10_e236_d_n8 * p.p1);
        let eq10_e239_q_d_n9: f64 = (eq10_e236_d_n9 * p.p1);
        let eq10_e239_q_d_n10: f64 = (eq10_e236_d_n10 * p.p1);
        let eq10_reactive_node_derivatives: [f64; 11] = [eq10_e239_q_d_n0, eq10_e239_q_d_n1, eq10_e239_q_d_n2, eq10_e239_q_d_n3, eq10_e239_q_d_n4, eq10_e239_q_d_n5, eq10_e239_q_d_n6, eq10_e239_q_d_n7, eq10_e239_q_d_n8, eq10_e239_q_d_n9, eq10_e239_q_d_n10];
        let eq10_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
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
        let eq11_e242: f64 = (p.p3 * s.v[206]);
        let eq11_e242_d_n0: f64 = (p.p3 * s.dn[206][0]);
        let eq11_e242_d_n1: f64 = (p.p3 * s.dn[206][1]);
        let eq11_e242_d_n2: f64 = (p.p3 * s.dn[206][2]);
        let eq11_e242_d_n3: f64 = (p.p3 * s.dn[206][3]);
        let eq11_e242_d_n4: f64 = (p.p3 * s.dn[206][4]);
        let eq11_e242_d_n5: f64 = (p.p3 * s.dn[206][5]);
        let eq11_e242_d_n6: f64 = (p.p3 * s.dn[206][6]);
        let eq11_e242_d_n7: f64 = (p.p3 * s.dn[206][7]);
        let eq11_e242_d_n8: f64 = (p.p3 * s.dn[206][8]);
        let eq11_e242_d_n9: f64 = (p.p3 * s.dn[206][9]);
        let eq11_e242_d_n10: f64 = (p.p3 * s.dn[206][10]);
        let eq11_e243_q: f64 = eq11_e242;
        let eq11_e245: f64 = (eq11_e242 * p.p1);
        let eq11_e245_d_n0: f64 = (eq11_e242_d_n0 * p.p1);
        let eq11_e245_d_n1: f64 = (eq11_e242_d_n1 * p.p1);
        let eq11_e245_d_n2: f64 = (eq11_e242_d_n2 * p.p1);
        let eq11_e245_d_n3: f64 = (eq11_e242_d_n3 * p.p1);
        let eq11_e245_d_n4: f64 = (eq11_e242_d_n4 * p.p1);
        let eq11_e245_d_n5: f64 = (eq11_e242_d_n5 * p.p1);
        let eq11_e245_d_n6: f64 = (eq11_e242_d_n6 * p.p1);
        let eq11_e245_d_n7: f64 = (eq11_e242_d_n7 * p.p1);
        let eq11_e245_d_n8: f64 = (eq11_e242_d_n8 * p.p1);
        let eq11_e245_d_n9: f64 = (eq11_e242_d_n9 * p.p1);
        let eq11_e245_d_n10: f64 = (eq11_e242_d_n10 * p.p1);
        let eq11_e245_q: f64 = (eq11_e243_q * p.p1);
        let eq11_e245_q_d_n0: f64 = (eq11_e242_d_n0 * p.p1);
        let eq11_e245_q_d_n1: f64 = (eq11_e242_d_n1 * p.p1);
        let eq11_e245_q_d_n2: f64 = (eq11_e242_d_n2 * p.p1);
        let eq11_e245_q_d_n3: f64 = (eq11_e242_d_n3 * p.p1);
        let eq11_e245_q_d_n4: f64 = (eq11_e242_d_n4 * p.p1);
        let eq11_e245_q_d_n5: f64 = (eq11_e242_d_n5 * p.p1);
        let eq11_e245_q_d_n6: f64 = (eq11_e242_d_n6 * p.p1);
        let eq11_e245_q_d_n7: f64 = (eq11_e242_d_n7 * p.p1);
        let eq11_e245_q_d_n8: f64 = (eq11_e242_d_n8 * p.p1);
        let eq11_e245_q_d_n9: f64 = (eq11_e242_d_n9 * p.p1);
        let eq11_e245_q_d_n10: f64 = (eq11_e242_d_n10 * p.p1);
        let eq11_reactive_node_derivatives: [f64; 11] = [eq11_e245_q_d_n0, eq11_e245_q_d_n1, eq11_e245_q_d_n2, eq11_e245_q_d_n3, eq11_e245_q_d_n4, eq11_e245_q_d_n5, eq11_e245_q_d_n6, eq11_e245_q_d_n7, eq11_e245_q_d_n8, eq11_e245_q_d_n9, eq11_e245_q_d_n10];
        let eq11_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[3]),
            &nodes,
            &eq11_reactive_node_derivatives,
            &branches,
            &eq11_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
