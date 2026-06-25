#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq55_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[3]),
            self.multiplicity * (eq55_value),
            &[
            ],
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
        let eq56_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[3]),
            self.multiplicity * (eq56_value),
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
        let (eq57_e528,) = {
    if ((s.v[625] != 0.0) && (s.v[626] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq57_value: f64 = eq57_e528;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[10]),
            self.multiplicity * (eq57_value),
            &[
            ],
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
        let (eq58_e538,) = {
    if ((s.v[625] != 0.0) && (s.v[626] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e538;
        stamper.stamp_current(
            Some(nodes[10]),
            Some(nodes[11]),
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
        let (eq59_e548,) = {
    if ((s.v[625] != 0.0) && (s.v[626] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e548;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[8]),
            self.multiplicity * (eq59_value),
            &[
            ],
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
        let (eq60_e559,) = {
    if ((s.v[625] != 0.0) && (!(s.v[626] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e559;
        stamper.stamp_current(
            Some(nodes[0]),
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
        let (eq61_e570,) = {
    if ((s.v[625] != 0.0) && (!(s.v[626] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e570;
        stamper.stamp_current(
            Some(nodes[10]),
            Some(nodes[8]),
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
        let (eq62_e581,) = {
    if ((!(s.v[625] != 0.0)) && (s.v[627] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e581;
        stamper.stamp_current(
            Some(nodes[0]),
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
        let (eq63_e592,) = {
    if ((!(s.v[625] != 0.0)) && (s.v[627] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e592;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[8]),
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
        let (eq64_e604,) = {
    if ((!(s.v[625] != 0.0)) && (!(s.v[627] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e604;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[8]),
            self.multiplicity * (eq64_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq15_e268_q: f64 = s.rv[220];
        stamper.stamp_current_reactive(
            Some(nodes[4]),
            None,
            &[
                GeneratedDerivative::node(nodes[4], self.multiplicity * (s.rdn[220][4])),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq17_e278: f64 = (s.v[221] + s.v[226]);
        let eq17_e278_d_n0: f64 = (s.dn[221][0] + s.dn[226][0]);
        let eq17_e278_d_n1: f64 = (s.dn[221][1] + s.dn[226][1]);
        let eq17_e278_d_n2: f64 = (s.dn[221][2] + s.dn[226][2]);
        let eq17_e278_d_n3: f64 = (s.dn[221][3] + s.dn[226][3]);
        let eq17_e278_d_n4: f64 = (s.dn[221][4] + s.dn[226][4]);
        let eq17_e278_d_n5: f64 = (s.dn[221][5] + s.dn[226][5]);
        let eq17_e278_d_n6: f64 = (s.dn[221][6] + s.dn[226][6]);
        let eq17_e278_d_n7: f64 = (s.dn[221][7] + s.dn[226][7]);
        let eq17_e278_d_n8: f64 = (s.dn[221][8] + s.dn[226][8]);
        let eq17_e278_d_n9: f64 = (s.dn[221][9] + s.dn[226][9]);
        let eq17_e278_d_n10: f64 = (s.dn[221][10] + s.dn[226][10]);
        let eq17_e278_d_n11: f64 = (s.dn[221][11] + s.dn[226][11]);
        let eq17_e278_d_n12: f64 = (s.dn[221][12] + s.dn[226][12]);
        let eq17_e280: f64 = (eq17_e278 + s.v[241]);
        let eq17_e280_d_n0: f64 = (eq17_e278_d_n0 + s.dn[241][0]);
        let eq17_e280_d_n1: f64 = (eq17_e278_d_n1 + s.dn[241][1]);
        let eq17_e280_d_n2: f64 = (eq17_e278_d_n2 + s.dn[241][2]);
        let eq17_e280_d_n3: f64 = (eq17_e278_d_n3 + s.dn[241][3]);
        let eq17_e280_d_n4: f64 = (eq17_e278_d_n4 + s.dn[241][4]);
        let eq17_e280_d_n5: f64 = (eq17_e278_d_n5 + s.dn[241][5]);
        let eq17_e280_d_n6: f64 = (eq17_e278_d_n6 + s.dn[241][6]);
        let eq17_e280_d_n7: f64 = (eq17_e278_d_n7 + s.dn[241][7]);
        let eq17_e280_d_n8: f64 = (eq17_e278_d_n8 + s.dn[241][8]);
        let eq17_e280_d_n9: f64 = (eq17_e278_d_n9 + s.dn[241][9]);
        let eq17_e280_d_n10: f64 = (eq17_e278_d_n10 + s.dn[241][10]);
        let eq17_e280_d_n11: f64 = (eq17_e278_d_n11 + s.dn[241][11]);
        let eq17_e280_d_n12: f64 = (eq17_e278_d_n12 + s.dn[241][12]);
        let eq17_e281: f64 = (p.p3 * eq17_e280);
        let eq17_e281_d_n0: f64 = (p.p3 * eq17_e280_d_n0);
        let eq17_e281_d_n1: f64 = (p.p3 * eq17_e280_d_n1);
        let eq17_e281_d_n2: f64 = (p.p3 * eq17_e280_d_n2);
        let eq17_e281_d_n3: f64 = (p.p3 * eq17_e280_d_n3);
        let eq17_e281_d_n4: f64 = (p.p3 * eq17_e280_d_n4);
        let eq17_e281_d_n5: f64 = (p.p3 * eq17_e280_d_n5);
        let eq17_e281_d_n6: f64 = (p.p3 * eq17_e280_d_n6);
        let eq17_e281_d_n7: f64 = (p.p3 * eq17_e280_d_n7);
        let eq17_e281_d_n8: f64 = (p.p3 * eq17_e280_d_n8);
        let eq17_e281_d_n9: f64 = (p.p3 * eq17_e280_d_n9);
        let eq17_e281_d_n10: f64 = (p.p3 * eq17_e280_d_n10);
        let eq17_e281_d_n11: f64 = (p.p3 * eq17_e280_d_n11);
        let eq17_e281_d_n12: f64 = (p.p3 * eq17_e280_d_n12);
        let eq17_e282_q: f64 = eq17_e281;
        let eq17_e284: f64 = (eq17_e281 * p.p1);
        let eq17_e284_d_n0: f64 = (eq17_e281_d_n0 * p.p1);
        let eq17_e284_d_n1: f64 = (eq17_e281_d_n1 * p.p1);
        let eq17_e284_d_n2: f64 = (eq17_e281_d_n2 * p.p1);
        let eq17_e284_d_n3: f64 = (eq17_e281_d_n3 * p.p1);
        let eq17_e284_d_n4: f64 = (eq17_e281_d_n4 * p.p1);
        let eq17_e284_d_n5: f64 = (eq17_e281_d_n5 * p.p1);
        let eq17_e284_d_n6: f64 = (eq17_e281_d_n6 * p.p1);
        let eq17_e284_d_n7: f64 = (eq17_e281_d_n7 * p.p1);
        let eq17_e284_d_n8: f64 = (eq17_e281_d_n8 * p.p1);
        let eq17_e284_d_n9: f64 = (eq17_e281_d_n9 * p.p1);
        let eq17_e284_d_n10: f64 = (eq17_e281_d_n10 * p.p1);
        let eq17_e284_d_n11: f64 = (eq17_e281_d_n11 * p.p1);
        let eq17_e284_d_n12: f64 = (eq17_e281_d_n12 * p.p1);
        let eq17_e284_q: f64 = (eq17_e282_q * p.p1);
        let eq17_e284_q_d_n0: f64 = (eq17_e281_d_n0 * p.p1);
        let eq17_e284_q_d_n1: f64 = (eq17_e281_d_n1 * p.p1);
        let eq17_e284_q_d_n2: f64 = (eq17_e281_d_n2 * p.p1);
        let eq17_e284_q_d_n3: f64 = (eq17_e281_d_n3 * p.p1);
        let eq17_e284_q_d_n4: f64 = (eq17_e281_d_n4 * p.p1);
        let eq17_e284_q_d_n5: f64 = (eq17_e281_d_n5 * p.p1);
        let eq17_e284_q_d_n6: f64 = (eq17_e281_d_n6 * p.p1);
        let eq17_e284_q_d_n7: f64 = (eq17_e281_d_n7 * p.p1);
        let eq17_e284_q_d_n8: f64 = (eq17_e281_d_n8 * p.p1);
        let eq17_e284_q_d_n9: f64 = (eq17_e281_d_n9 * p.p1);
        let eq17_e284_q_d_n10: f64 = (eq17_e281_d_n10 * p.p1);
        let eq17_e284_q_d_n11: f64 = (eq17_e281_d_n11 * p.p1);
        let eq17_e284_q_d_n12: f64 = (eq17_e281_d_n12 * p.p1);
        let eq17_reactive_node_derivatives: [f64; 13] = [eq17_e284_q_d_n0, eq17_e284_q_d_n1, eq17_e284_q_d_n2, eq17_e284_q_d_n3, eq17_e284_q_d_n4, eq17_e284_q_d_n5, eq17_e284_q_d_n6, eq17_e284_q_d_n7, eq17_e284_q_d_n8, eq17_e284_q_d_n9, eq17_e284_q_d_n10, eq17_e284_q_d_n11, eq17_e284_q_d_n12];
        let eq17_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &nodes,
            &eq17_reactive_node_derivatives,
            &branches,
            &eq17_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq18_e287: f64 = (p.p3 * s.v[223]);
        let eq18_e287_d_n0: f64 = (p.p3 * s.dn[223][0]);
        let eq18_e287_d_n1: f64 = (p.p3 * s.dn[223][1]);
        let eq18_e287_d_n2: f64 = (p.p3 * s.dn[223][2]);
        let eq18_e287_d_n3: f64 = (p.p3 * s.dn[223][3]);
        let eq18_e287_d_n4: f64 = (p.p3 * s.dn[223][4]);
        let eq18_e287_d_n5: f64 = (p.p3 * s.dn[223][5]);
        let eq18_e287_d_n6: f64 = (p.p3 * s.dn[223][6]);
        let eq18_e287_d_n7: f64 = (p.p3 * s.dn[223][7]);
        let eq18_e287_d_n8: f64 = (p.p3 * s.dn[223][8]);
        let eq18_e287_d_n9: f64 = (p.p3 * s.dn[223][9]);
        let eq18_e287_d_n10: f64 = (p.p3 * s.dn[223][10]);
        let eq18_e287_d_n11: f64 = (p.p3 * s.dn[223][11]);
        let eq18_e287_d_n12: f64 = (p.p3 * s.dn[223][12]);
        let eq18_e288_q: f64 = eq18_e287;
        let eq18_e290: f64 = (eq18_e287 * p.p1);
        let eq18_e290_d_n0: f64 = (eq18_e287_d_n0 * p.p1);
        let eq18_e290_d_n1: f64 = (eq18_e287_d_n1 * p.p1);
        let eq18_e290_d_n2: f64 = (eq18_e287_d_n2 * p.p1);
        let eq18_e290_d_n3: f64 = (eq18_e287_d_n3 * p.p1);
        let eq18_e290_d_n4: f64 = (eq18_e287_d_n4 * p.p1);
        let eq18_e290_d_n5: f64 = (eq18_e287_d_n5 * p.p1);
        let eq18_e290_d_n6: f64 = (eq18_e287_d_n6 * p.p1);
        let eq18_e290_d_n7: f64 = (eq18_e287_d_n7 * p.p1);
        let eq18_e290_d_n8: f64 = (eq18_e287_d_n8 * p.p1);
        let eq18_e290_d_n9: f64 = (eq18_e287_d_n9 * p.p1);
        let eq18_e290_d_n10: f64 = (eq18_e287_d_n10 * p.p1);
        let eq18_e290_d_n11: f64 = (eq18_e287_d_n11 * p.p1);
        let eq18_e290_d_n12: f64 = (eq18_e287_d_n12 * p.p1);
        let eq18_e290_q: f64 = (eq18_e288_q * p.p1);
        let eq18_e290_q_d_n0: f64 = (eq18_e287_d_n0 * p.p1);
        let eq18_e290_q_d_n1: f64 = (eq18_e287_d_n1 * p.p1);
        let eq18_e290_q_d_n2: f64 = (eq18_e287_d_n2 * p.p1);
        let eq18_e290_q_d_n3: f64 = (eq18_e287_d_n3 * p.p1);
        let eq18_e290_q_d_n4: f64 = (eq18_e287_d_n4 * p.p1);
        let eq18_e290_q_d_n5: f64 = (eq18_e287_d_n5 * p.p1);
        let eq18_e290_q_d_n6: f64 = (eq18_e287_d_n6 * p.p1);
        let eq18_e290_q_d_n7: f64 = (eq18_e287_d_n7 * p.p1);
        let eq18_e290_q_d_n8: f64 = (eq18_e287_d_n8 * p.p1);
        let eq18_e290_q_d_n9: f64 = (eq18_e287_d_n9 * p.p1);
        let eq18_e290_q_d_n10: f64 = (eq18_e287_d_n10 * p.p1);
        let eq18_e290_q_d_n11: f64 = (eq18_e287_d_n11 * p.p1);
        let eq18_e290_q_d_n12: f64 = (eq18_e287_d_n12 * p.p1);
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e290_q_d_n0, eq18_e290_q_d_n1, eq18_e290_q_d_n2, eq18_e290_q_d_n3, eq18_e290_q_d_n4, eq18_e290_q_d_n5, eq18_e290_q_d_n6, eq18_e290_q_d_n7, eq18_e290_q_d_n8, eq18_e290_q_d_n9, eq18_e290_q_d_n10, eq18_e290_q_d_n11, eq18_e290_q_d_n12];
        let eq18_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            &nodes,
            &eq18_reactive_node_derivatives,
            &branches,
            &eq18_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq19_e294: f64 = (s.v[224] + s.v[227]);
        let eq19_e294_d_n0: f64 = (s.dn[224][0] + s.dn[227][0]);
        let eq19_e294_d_n1: f64 = (s.dn[224][1] + s.dn[227][1]);
        let eq19_e294_d_n2: f64 = (s.dn[224][2] + s.dn[227][2]);
        let eq19_e294_d_n3: f64 = (s.dn[224][3] + s.dn[227][3]);
        let eq19_e294_d_n4: f64 = (s.dn[224][4] + s.dn[227][4]);
        let eq19_e294_d_n5: f64 = (s.dn[224][5] + s.dn[227][5]);
        let eq19_e294_d_n6: f64 = (s.dn[224][6] + s.dn[227][6]);
        let eq19_e294_d_n7: f64 = (s.dn[224][7] + s.dn[227][7]);
        let eq19_e294_d_n8: f64 = (s.dn[224][8] + s.dn[227][8]);
        let eq19_e294_d_n9: f64 = (s.dn[224][9] + s.dn[227][9]);
        let eq19_e294_d_n10: f64 = (s.dn[224][10] + s.dn[227][10]);
        let eq19_e294_d_n11: f64 = (s.dn[224][11] + s.dn[227][11]);
        let eq19_e294_d_n12: f64 = (s.dn[224][12] + s.dn[227][12]);
        let eq19_e296: f64 = (eq19_e294 + s.v[244]);
        let eq19_e296_d_n0: f64 = (eq19_e294_d_n0 + s.dn[244][0]);
        let eq19_e296_d_n1: f64 = (eq19_e294_d_n1 + s.dn[244][1]);
        let eq19_e296_d_n2: f64 = (eq19_e294_d_n2 + s.dn[244][2]);
        let eq19_e296_d_n3: f64 = (eq19_e294_d_n3 + s.dn[244][3]);
        let eq19_e296_d_n4: f64 = (eq19_e294_d_n4 + s.dn[244][4]);
        let eq19_e296_d_n5: f64 = (eq19_e294_d_n5 + s.dn[244][5]);
        let eq19_e296_d_n6: f64 = (eq19_e294_d_n6 + s.dn[244][6]);
        let eq19_e296_d_n7: f64 = (eq19_e294_d_n7 + s.dn[244][7]);
        let eq19_e296_d_n8: f64 = (eq19_e294_d_n8 + s.dn[244][8]);
        let eq19_e296_d_n9: f64 = (eq19_e294_d_n9 + s.dn[244][9]);
        let eq19_e296_d_n10: f64 = (eq19_e294_d_n10 + s.dn[244][10]);
        let eq19_e296_d_n11: f64 = (eq19_e294_d_n11 + s.dn[244][11]);
        let eq19_e296_d_n12: f64 = (eq19_e294_d_n12 + s.dn[244][12]);
        let eq19_e297: f64 = (p.p3 * eq19_e296);
        let eq19_e297_d_n0: f64 = (p.p3 * eq19_e296_d_n0);
        let eq19_e297_d_n1: f64 = (p.p3 * eq19_e296_d_n1);
        let eq19_e297_d_n2: f64 = (p.p3 * eq19_e296_d_n2);
        let eq19_e297_d_n3: f64 = (p.p3 * eq19_e296_d_n3);
        let eq19_e297_d_n4: f64 = (p.p3 * eq19_e296_d_n4);
        let eq19_e297_d_n5: f64 = (p.p3 * eq19_e296_d_n5);
        let eq19_e297_d_n6: f64 = (p.p3 * eq19_e296_d_n6);
        let eq19_e297_d_n7: f64 = (p.p3 * eq19_e296_d_n7);
        let eq19_e297_d_n8: f64 = (p.p3 * eq19_e296_d_n8);
        let eq19_e297_d_n9: f64 = (p.p3 * eq19_e296_d_n9);
        let eq19_e297_d_n10: f64 = (p.p3 * eq19_e296_d_n10);
        let eq19_e297_d_n11: f64 = (p.p3 * eq19_e296_d_n11);
        let eq19_e297_d_n12: f64 = (p.p3 * eq19_e296_d_n12);
        let eq19_e298_q: f64 = eq19_e297;
        let eq19_e300: f64 = (eq19_e297 * p.p1);
        let eq19_e300_d_n0: f64 = (eq19_e297_d_n0 * p.p1);
        let eq19_e300_d_n1: f64 = (eq19_e297_d_n1 * p.p1);
        let eq19_e300_d_n2: f64 = (eq19_e297_d_n2 * p.p1);
        let eq19_e300_d_n3: f64 = (eq19_e297_d_n3 * p.p1);
        let eq19_e300_d_n4: f64 = (eq19_e297_d_n4 * p.p1);
        let eq19_e300_d_n5: f64 = (eq19_e297_d_n5 * p.p1);
        let eq19_e300_d_n6: f64 = (eq19_e297_d_n6 * p.p1);
        let eq19_e300_d_n7: f64 = (eq19_e297_d_n7 * p.p1);
        let eq19_e300_d_n8: f64 = (eq19_e297_d_n8 * p.p1);
        let eq19_e300_d_n9: f64 = (eq19_e297_d_n9 * p.p1);
        let eq19_e300_d_n10: f64 = (eq19_e297_d_n10 * p.p1);
        let eq19_e300_d_n11: f64 = (eq19_e297_d_n11 * p.p1);
        let eq19_e300_d_n12: f64 = (eq19_e297_d_n12 * p.p1);
        let eq19_e300_q: f64 = (eq19_e298_q * p.p1);
        let eq19_e300_q_d_n0: f64 = (eq19_e297_d_n0 * p.p1);
        let eq19_e300_q_d_n1: f64 = (eq19_e297_d_n1 * p.p1);
        let eq19_e300_q_d_n2: f64 = (eq19_e297_d_n2 * p.p1);
        let eq19_e300_q_d_n3: f64 = (eq19_e297_d_n3 * p.p1);
        let eq19_e300_q_d_n4: f64 = (eq19_e297_d_n4 * p.p1);
        let eq19_e300_q_d_n5: f64 = (eq19_e297_d_n5 * p.p1);
        let eq19_e300_q_d_n6: f64 = (eq19_e297_d_n6 * p.p1);
        let eq19_e300_q_d_n7: f64 = (eq19_e297_d_n7 * p.p1);
        let eq19_e300_q_d_n8: f64 = (eq19_e297_d_n8 * p.p1);
        let eq19_e300_q_d_n9: f64 = (eq19_e297_d_n9 * p.p1);
        let eq19_e300_q_d_n10: f64 = (eq19_e297_d_n10 * p.p1);
        let eq19_e300_q_d_n11: f64 = (eq19_e297_d_n11 * p.p1);
        let eq19_e300_q_d_n12: f64 = (eq19_e297_d_n12 * p.p1);
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e300_q_d_n0, eq19_e300_q_d_n1, eq19_e300_q_d_n2, eq19_e300_q_d_n3, eq19_e300_q_d_n4, eq19_e300_q_d_n5, eq19_e300_q_d_n6, eq19_e300_q_d_n7, eq19_e300_q_d_n8, eq19_e300_q_d_n9, eq19_e300_q_d_n10, eq19_e300_q_d_n11, eq19_e300_q_d_n12];
        let eq19_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &nodes,
            &eq19_reactive_node_derivatives,
            &branches,
            &eq19_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq20_e303: f64 = (p.p3 * s.v[239]);
        let eq20_e303_d_n0: f64 = (p.p3 * s.dn[239][0]);
        let eq20_e303_d_n1: f64 = (p.p3 * s.dn[239][1]);
        let eq20_e303_d_n2: f64 = (p.p3 * s.dn[239][2]);
        let eq20_e303_d_n3: f64 = (p.p3 * s.dn[239][3]);
        let eq20_e303_d_n4: f64 = (p.p3 * s.dn[239][4]);
        let eq20_e303_d_n5: f64 = (p.p3 * s.dn[239][5]);
        let eq20_e303_d_n6: f64 = (p.p3 * s.dn[239][6]);
        let eq20_e303_d_n7: f64 = (p.p3 * s.dn[239][7]);
        let eq20_e303_d_n8: f64 = (p.p3 * s.dn[239][8]);
        let eq20_e303_d_n9: f64 = (p.p3 * s.dn[239][9]);
        let eq20_e303_d_n10: f64 = (p.p3 * s.dn[239][10]);
        let eq20_e303_d_n11: f64 = (p.p3 * s.dn[239][11]);
        let eq20_e303_d_n12: f64 = (p.p3 * s.dn[239][12]);
        let eq20_e304_q: f64 = eq20_e303;
        let eq20_e306: f64 = (eq20_e303 * p.p1);
        let eq20_e306_d_n0: f64 = (eq20_e303_d_n0 * p.p1);
        let eq20_e306_d_n1: f64 = (eq20_e303_d_n1 * p.p1);
        let eq20_e306_d_n2: f64 = (eq20_e303_d_n2 * p.p1);
        let eq20_e306_d_n3: f64 = (eq20_e303_d_n3 * p.p1);
        let eq20_e306_d_n4: f64 = (eq20_e303_d_n4 * p.p1);
        let eq20_e306_d_n5: f64 = (eq20_e303_d_n5 * p.p1);
        let eq20_e306_d_n6: f64 = (eq20_e303_d_n6 * p.p1);
        let eq20_e306_d_n7: f64 = (eq20_e303_d_n7 * p.p1);
        let eq20_e306_d_n8: f64 = (eq20_e303_d_n8 * p.p1);
        let eq20_e306_d_n9: f64 = (eq20_e303_d_n9 * p.p1);
        let eq20_e306_d_n10: f64 = (eq20_e303_d_n10 * p.p1);
        let eq20_e306_d_n11: f64 = (eq20_e303_d_n11 * p.p1);
        let eq20_e306_d_n12: f64 = (eq20_e303_d_n12 * p.p1);
        let eq20_e306_q: f64 = (eq20_e304_q * p.p1);
        let eq20_e306_q_d_n0: f64 = (eq20_e303_d_n0 * p.p1);
        let eq20_e306_q_d_n1: f64 = (eq20_e303_d_n1 * p.p1);
        let eq20_e306_q_d_n2: f64 = (eq20_e303_d_n2 * p.p1);
        let eq20_e306_q_d_n3: f64 = (eq20_e303_d_n3 * p.p1);
        let eq20_e306_q_d_n4: f64 = (eq20_e303_d_n4 * p.p1);
        let eq20_e306_q_d_n5: f64 = (eq20_e303_d_n5 * p.p1);
        let eq20_e306_q_d_n6: f64 = (eq20_e303_d_n6 * p.p1);
        let eq20_e306_q_d_n7: f64 = (eq20_e303_d_n7 * p.p1);
        let eq20_e306_q_d_n8: f64 = (eq20_e303_d_n8 * p.p1);
        let eq20_e306_q_d_n9: f64 = (eq20_e303_d_n9 * p.p1);
        let eq20_e306_q_d_n10: f64 = (eq20_e303_d_n10 * p.p1);
        let eq20_e306_q_d_n11: f64 = (eq20_e303_d_n11 * p.p1);
        let eq20_e306_q_d_n12: f64 = (eq20_e303_d_n12 * p.p1);
        let eq20_reactive_node_derivatives: [f64; 13] = [eq20_e306_q_d_n0, eq20_e306_q_d_n1, eq20_e306_q_d_n2, eq20_e306_q_d_n3, eq20_e306_q_d_n4, eq20_e306_q_d_n5, eq20_e306_q_d_n6, eq20_e306_q_d_n7, eq20_e306_q_d_n8, eq20_e306_q_d_n9, eq20_e306_q_d_n10, eq20_e306_q_d_n11, eq20_e306_q_d_n12];
        let eq20_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            &nodes,
            &eq20_reactive_node_derivatives,
            &branches,
            &eq20_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_21_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq21_e309: f64 = (p.p3 * s.v[228]);
        let eq21_e309_d_n0: f64 = (p.p3 * s.dn[228][0]);
        let eq21_e309_d_n1: f64 = (p.p3 * s.dn[228][1]);
        let eq21_e309_d_n2: f64 = (p.p3 * s.dn[228][2]);
        let eq21_e309_d_n3: f64 = (p.p3 * s.dn[228][3]);
        let eq21_e309_d_n4: f64 = (p.p3 * s.dn[228][4]);
        let eq21_e309_d_n5: f64 = (p.p3 * s.dn[228][5]);
        let eq21_e309_d_n6: f64 = (p.p3 * s.dn[228][6]);
        let eq21_e309_d_n7: f64 = (p.p3 * s.dn[228][7]);
        let eq21_e309_d_n8: f64 = (p.p3 * s.dn[228][8]);
        let eq21_e309_d_n9: f64 = (p.p3 * s.dn[228][9]);
        let eq21_e309_d_n10: f64 = (p.p3 * s.dn[228][10]);
        let eq21_e309_d_n11: f64 = (p.p3 * s.dn[228][11]);
        let eq21_e309_d_n12: f64 = (p.p3 * s.dn[228][12]);
        let eq21_e310_q: f64 = eq21_e309;
        let eq21_e312: f64 = (eq21_e309 * p.p1);
        let eq21_e312_d_n0: f64 = (eq21_e309_d_n0 * p.p1);
        let eq21_e312_d_n1: f64 = (eq21_e309_d_n1 * p.p1);
        let eq21_e312_d_n2: f64 = (eq21_e309_d_n2 * p.p1);
        let eq21_e312_d_n3: f64 = (eq21_e309_d_n3 * p.p1);
        let eq21_e312_d_n4: f64 = (eq21_e309_d_n4 * p.p1);
        let eq21_e312_d_n5: f64 = (eq21_e309_d_n5 * p.p1);
        let eq21_e312_d_n6: f64 = (eq21_e309_d_n6 * p.p1);
        let eq21_e312_d_n7: f64 = (eq21_e309_d_n7 * p.p1);
        let eq21_e312_d_n8: f64 = (eq21_e309_d_n8 * p.p1);
        let eq21_e312_d_n9: f64 = (eq21_e309_d_n9 * p.p1);
        let eq21_e312_d_n10: f64 = (eq21_e309_d_n10 * p.p1);
        let eq21_e312_d_n11: f64 = (eq21_e309_d_n11 * p.p1);
        let eq21_e312_d_n12: f64 = (eq21_e309_d_n12 * p.p1);
        let eq21_e312_q: f64 = (eq21_e310_q * p.p1);
        let eq21_e312_q_d_n0: f64 = (eq21_e309_d_n0 * p.p1);
        let eq21_e312_q_d_n1: f64 = (eq21_e309_d_n1 * p.p1);
        let eq21_e312_q_d_n2: f64 = (eq21_e309_d_n2 * p.p1);
        let eq21_e312_q_d_n3: f64 = (eq21_e309_d_n3 * p.p1);
        let eq21_e312_q_d_n4: f64 = (eq21_e309_d_n4 * p.p1);
        let eq21_e312_q_d_n5: f64 = (eq21_e309_d_n5 * p.p1);
        let eq21_e312_q_d_n6: f64 = (eq21_e309_d_n6 * p.p1);
        let eq21_e312_q_d_n7: f64 = (eq21_e309_d_n7 * p.p1);
        let eq21_e312_q_d_n8: f64 = (eq21_e309_d_n8 * p.p1);
        let eq21_e312_q_d_n9: f64 = (eq21_e309_d_n9 * p.p1);
        let eq21_e312_q_d_n10: f64 = (eq21_e309_d_n10 * p.p1);
        let eq21_e312_q_d_n11: f64 = (eq21_e309_d_n11 * p.p1);
        let eq21_e312_q_d_n12: f64 = (eq21_e309_d_n12 * p.p1);
        let eq21_reactive_node_derivatives: [f64; 13] = [eq21_e312_q_d_n0, eq21_e312_q_d_n1, eq21_e312_q_d_n2, eq21_e312_q_d_n3, eq21_e312_q_d_n4, eq21_e312_q_d_n5, eq21_e312_q_d_n6, eq21_e312_q_d_n7, eq21_e312_q_d_n8, eq21_e312_q_d_n9, eq21_e312_q_d_n10, eq21_e312_q_d_n11, eq21_e312_q_d_n12];
        let eq21_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            &nodes,
            &eq21_reactive_node_derivatives,
            &branches,
            &eq21_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
