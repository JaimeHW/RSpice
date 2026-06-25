#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq23_e489,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e489;
        stamper.stamp_potential(
            branches[14],
            eq23_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_24_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq24_e498,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e498;
        stamper.stamp_potential(
            branches[15],
            eq24_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_25_block_0(
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
        let (eq25_e511, eq25_e511_d_n5,) = {
    if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
        let eq25_e509: f64 = ((nv5 - 0.0) / p.p119);
        let eq25_e509_d_n5: f64 = (1.0 / p.p119);
        (eq25_e509, eq25_e509_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e511;
        stamper.stamp_current(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq25_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq25_e511_d_n5),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq26_e525, eq26_e525_d_n0, eq26_e525_d_n1, eq26_e525_d_n2, eq26_e525_d_n3, eq26_e525_d_n4, eq26_e525_d_n5, eq26_e525_d_n6, eq26_e525_d_n7, eq26_e525_d_n8, eq26_e525_d_n9, eq26_e525_d_n10, eq26_e525_d_n11, eq26_e525_d_n12, eq26_e525_d_n13, eq26_e525_d_n14, eq26_e525_d_n15, eq26_e525_d_n16, eq26_e525_d_n17, eq26_e525_d_n18, eq26_e525_d_n19, eq26_e525_d_n20, eq26_e525_d_n21, eq26_e525_d_n22,) = {
    if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
        let eq26_e521: f64 = (-1.0);
        let eq26_e523: f64 = (eq26_e521 * s.v[148]);
        let eq26_e523_d_n0: f64 = (eq26_e521 * s.dn[148][0]);
        let eq26_e523_d_n1: f64 = (eq26_e521 * s.dn[148][1]);
        let eq26_e523_d_n2: f64 = (eq26_e521 * s.dn[148][2]);
        let eq26_e523_d_n3: f64 = (eq26_e521 * s.dn[148][3]);
        let eq26_e523_d_n4: f64 = (eq26_e521 * s.dn[148][4]);
        let eq26_e523_d_n5: f64 = (eq26_e521 * s.dn[148][5]);
        let eq26_e523_d_n6: f64 = (eq26_e521 * s.dn[148][6]);
        let eq26_e523_d_n7: f64 = (eq26_e521 * s.dn[148][7]);
        let eq26_e523_d_n8: f64 = (eq26_e521 * s.dn[148][8]);
        let eq26_e523_d_n9: f64 = (eq26_e521 * s.dn[148][9]);
        let eq26_e523_d_n10: f64 = (eq26_e521 * s.dn[148][10]);
        let eq26_e523_d_n11: f64 = (eq26_e521 * s.dn[148][11]);
        let eq26_e523_d_n12: f64 = (eq26_e521 * s.dn[148][12]);
        let eq26_e523_d_n13: f64 = (eq26_e521 * s.dn[148][13]);
        let eq26_e523_d_n14: f64 = (eq26_e521 * s.dn[148][14]);
        let eq26_e523_d_n15: f64 = (eq26_e521 * s.dn[148][15]);
        let eq26_e523_d_n16: f64 = (eq26_e521 * s.dn[148][16]);
        let eq26_e523_d_n17: f64 = (eq26_e521 * s.dn[148][17]);
        let eq26_e523_d_n18: f64 = (eq26_e521 * s.dn[148][18]);
        let eq26_e523_d_n19: f64 = (eq26_e521 * s.dn[148][19]);
        let eq26_e523_d_n20: f64 = (eq26_e521 * s.dn[148][20]);
        let eq26_e523_d_n21: f64 = (eq26_e521 * s.dn[148][21]);
        let eq26_e523_d_n22: f64 = (eq26_e521 * s.dn[148][22]);
        (eq26_e523, eq26_e523_d_n0, eq26_e523_d_n1, eq26_e523_d_n2, eq26_e523_d_n3, eq26_e523_d_n4, eq26_e523_d_n5, eq26_e523_d_n6, eq26_e523_d_n7, eq26_e523_d_n8, eq26_e523_d_n9, eq26_e523_d_n10, eq26_e523_d_n11, eq26_e523_d_n12, eq26_e523_d_n13, eq26_e523_d_n14, eq26_e523_d_n15, eq26_e523_d_n16, eq26_e523_d_n17, eq26_e523_d_n18, eq26_e523_d_n19, eq26_e523_d_n20, eq26_e523_d_n21, eq26_e523_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e525;
        let eq26_node_derivatives: [f64; 23] = [eq26_e525_d_n0, eq26_e525_d_n1, eq26_e525_d_n2, eq26_e525_d_n3, eq26_e525_d_n4, eq26_e525_d_n5, eq26_e525_d_n6, eq26_e525_d_n7, eq26_e525_d_n8, eq26_e525_d_n9, eq26_e525_d_n10, eq26_e525_d_n11, eq26_e525_d_n12, eq26_e525_d_n13, eq26_e525_d_n14, eq26_e525_d_n15, eq26_e525_d_n16, eq26_e525_d_n17, eq26_e525_d_n18, eq26_e525_d_n19, eq26_e525_d_n20, eq26_e525_d_n21, eq26_e525_d_n22];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq26_value),
            &nodes,
            &eq26_node_derivatives,
            &branches,
            &eq26_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_27_block_0(
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
        let (eq27_e539, eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22,) = {
    if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
        let eq27_e536: f64 = self.eval_ddt(3, (nv5 - 0.0));
        let eq27_e536_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n5: f64 = self.ddt_jacobian(1.0);
        let eq27_e536_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq27_e536_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq27_e537: f64 = (s.v[149] * eq27_e536);
        let eq27_e537_d_n0: f64 = ((s.dn[149][0] * eq27_e536) + (s.v[149] * eq27_e536_d_n0));
        let eq27_e537_d_n1: f64 = ((s.dn[149][1] * eq27_e536) + (s.v[149] * eq27_e536_d_n1));
        let eq27_e537_d_n2: f64 = ((s.dn[149][2] * eq27_e536) + (s.v[149] * eq27_e536_d_n2));
        let eq27_e537_d_n3: f64 = ((s.dn[149][3] * eq27_e536) + (s.v[149] * eq27_e536_d_n3));
        let eq27_e537_d_n4: f64 = ((s.dn[149][4] * eq27_e536) + (s.v[149] * eq27_e536_d_n4));
        let eq27_e537_d_n5: f64 = ((s.dn[149][5] * eq27_e536) + (s.v[149] * eq27_e536_d_n5));
        let eq27_e537_d_n6: f64 = ((s.dn[149][6] * eq27_e536) + (s.v[149] * eq27_e536_d_n6));
        let eq27_e537_d_n7: f64 = ((s.dn[149][7] * eq27_e536) + (s.v[149] * eq27_e536_d_n7));
        let eq27_e537_d_n8: f64 = ((s.dn[149][8] * eq27_e536) + (s.v[149] * eq27_e536_d_n8));
        let eq27_e537_d_n9: f64 = ((s.dn[149][9] * eq27_e536) + (s.v[149] * eq27_e536_d_n9));
        let eq27_e537_d_n10: f64 = ((s.dn[149][10] * eq27_e536) + (s.v[149] * eq27_e536_d_n10));
        let eq27_e537_d_n11: f64 = ((s.dn[149][11] * eq27_e536) + (s.v[149] * eq27_e536_d_n11));
        let eq27_e537_d_n12: f64 = ((s.dn[149][12] * eq27_e536) + (s.v[149] * eq27_e536_d_n12));
        let eq27_e537_d_n13: f64 = ((s.dn[149][13] * eq27_e536) + (s.v[149] * eq27_e536_d_n13));
        let eq27_e537_d_n14: f64 = ((s.dn[149][14] * eq27_e536) + (s.v[149] * eq27_e536_d_n14));
        let eq27_e537_d_n15: f64 = ((s.dn[149][15] * eq27_e536) + (s.v[149] * eq27_e536_d_n15));
        let eq27_e537_d_n16: f64 = ((s.dn[149][16] * eq27_e536) + (s.v[149] * eq27_e536_d_n16));
        let eq27_e537_d_n17: f64 = ((s.dn[149][17] * eq27_e536) + (s.v[149] * eq27_e536_d_n17));
        let eq27_e537_d_n18: f64 = ((s.dn[149][18] * eq27_e536) + (s.v[149] * eq27_e536_d_n18));
        let eq27_e537_d_n19: f64 = ((s.dn[149][19] * eq27_e536) + (s.v[149] * eq27_e536_d_n19));
        let eq27_e537_d_n20: f64 = ((s.dn[149][20] * eq27_e536) + (s.v[149] * eq27_e536_d_n20));
        let eq27_e537_d_n21: f64 = ((s.dn[149][21] * eq27_e536) + (s.v[149] * eq27_e536_d_n21));
        let eq27_e537_d_n22: f64 = ((s.dn[149][22] * eq27_e536) + (s.v[149] * eq27_e536_d_n22));
        (eq27_e537, eq27_e537_d_n0, eq27_e537_d_n1, eq27_e537_d_n2, eq27_e537_d_n3, eq27_e537_d_n4, eq27_e537_d_n5, eq27_e537_d_n6, eq27_e537_d_n7, eq27_e537_d_n8, eq27_e537_d_n9, eq27_e537_d_n10, eq27_e537_d_n11, eq27_e537_d_n12, eq27_e537_d_n13, eq27_e537_d_n14, eq27_e537_d_n15, eq27_e537_d_n16, eq27_e537_d_n17, eq27_e537_d_n18, eq27_e537_d_n19, eq27_e537_d_n20, eq27_e537_d_n21, eq27_e537_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e539;
        let eq27_node_derivatives: [f64; 23] = [eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq27_value),
            &nodes,
            &eq27_node_derivatives,
            &branches,
            &eq27_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_28_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq28_e550,) = {
    if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e550;
        stamper.stamp_potential(
            branches[16],
            eq28_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_29_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq29_e561,) = {
    if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e561;
        stamper.stamp_potential(
            branches[17],
            eq29_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_30_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq30_e572,) = {
    if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e572;
        stamper.stamp_potential(
            branches[18],
            eq30_value,
            &[
            ],
        );
    }

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
        let (eq31_e583,) = {
    if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e583;
        stamper.stamp_potential(
            branches[19],
            eq31_value,
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
        let (eq32_e594,) = {
    if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e594;
        stamper.stamp_potential(
            branches[20],
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
        let (eq33_e607,) = {
    if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq33_value: f64 = eq33_e607;
        stamper.stamp_potential(
            branches[21],
            eq33_value,
            &[
            ],
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
        let (eq34_e620,) = {
    if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e620;
        stamper.stamp_potential(
            branches[22],
            eq34_value,
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
        let (eq35_e633, eq35_e633_d_n0, eq35_e633_d_n1, eq35_e633_d_n2, eq35_e633_d_n3, eq35_e633_d_n4, eq35_e633_d_n5, eq35_e633_d_n6, eq35_e633_d_n7, eq35_e633_d_n8, eq35_e633_d_n9, eq35_e633_d_n10, eq35_e633_d_n11, eq35_e633_d_n12, eq35_e633_d_n13, eq35_e633_d_n14, eq35_e633_d_n15, eq35_e633_d_n16, eq35_e633_d_n17, eq35_e633_d_n18, eq35_e633_d_n19, eq35_e633_d_n20, eq35_e633_d_n21, eq35_e633_d_n22,) = {
    if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
        (s.v[136], s.dn[136][0], s.dn[136][1], s.dn[136][2], s.dn[136][3], s.dn[136][4], s.dn[136][5], s.dn[136][6], s.dn[136][7], s.dn[136][8], s.dn[136][9], s.dn[136][10], s.dn[136][11], s.dn[136][12], s.dn[136][13], s.dn[136][14], s.dn[136][15], s.dn[136][16], s.dn[136][17], s.dn[136][18], s.dn[136][19], s.dn[136][20], s.dn[136][21], s.dn[136][22],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e633;
        let eq35_node_derivatives: [f64; 23] = [eq35_e633_d_n0, eq35_e633_d_n1, eq35_e633_d_n2, eq35_e633_d_n3, eq35_e633_d_n4, eq35_e633_d_n5, eq35_e633_d_n6, eq35_e633_d_n7, eq35_e633_d_n8, eq35_e633_d_n9, eq35_e633_d_n10, eq35_e633_d_n11, eq35_e633_d_n12, eq35_e633_d_n13, eq35_e633_d_n14, eq35_e633_d_n15, eq35_e633_d_n16, eq35_e633_d_n17, eq35_e633_d_n18, eq35_e633_d_n19, eq35_e633_d_n20, eq35_e633_d_n21, eq35_e633_d_n22];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_dense(
            branches[23],
            eq35_value,
            &nodes,
            &eq35_node_derivatives,
            &branches,
            &eq35_branch_derivatives,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq36_e648, eq36_e648_d_n0, eq36_e648_d_n1, eq36_e648_d_n2, eq36_e648_d_n3, eq36_e648_d_n4, eq36_e648_d_n5, eq36_e648_d_n6, eq36_e648_d_n7, eq36_e648_d_n8, eq36_e648_d_n9, eq36_e648_d_n10, eq36_e648_d_n11, eq36_e648_d_n12, eq36_e648_d_n13, eq36_e648_d_n14, eq36_e648_d_n15, eq36_e648_d_n16, eq36_e648_d_n17, eq36_e648_d_n18, eq36_e648_d_n19, eq36_e648_d_n20, eq36_e648_d_n21, eq36_e648_d_n22,) = {
    if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
        let eq36_e646: f64 = ((nv11 - nv12) / s.v[338]);
        let eq36_e646_d_n0: f64 = (-(((nv11 - nv12) * s.dn[338][0]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n1: f64 = (-(((nv11 - nv12) * s.dn[338][1]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n2: f64 = (-(((nv11 - nv12) * s.dn[338][2]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n3: f64 = (-(((nv11 - nv12) * s.dn[338][3]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n4: f64 = (-(((nv11 - nv12) * s.dn[338][4]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n5: f64 = (-(((nv11 - nv12) * s.dn[338][5]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n6: f64 = (-(((nv11 - nv12) * s.dn[338][6]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n7: f64 = (-(((nv11 - nv12) * s.dn[338][7]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n8: f64 = (-(((nv11 - nv12) * s.dn[338][8]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n9: f64 = (-(((nv11 - nv12) * s.dn[338][9]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n10: f64 = (-(((nv11 - nv12) * s.dn[338][10]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n11: f64 = ((s.v[338] - ((nv11 - nv12) * s.dn[338][11])) / (s.v[338] * s.v[338]));
        let eq36_e646_d_n12: f64 = (((-s.v[338]) - ((nv11 - nv12) * s.dn[338][12])) / (s.v[338] * s.v[338]));
        let eq36_e646_d_n13: f64 = (-(((nv11 - nv12) * s.dn[338][13]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n14: f64 = (-(((nv11 - nv12) * s.dn[338][14]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n15: f64 = (-(((nv11 - nv12) * s.dn[338][15]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n16: f64 = (-(((nv11 - nv12) * s.dn[338][16]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n17: f64 = (-(((nv11 - nv12) * s.dn[338][17]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n18: f64 = (-(((nv11 - nv12) * s.dn[338][18]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n19: f64 = (-(((nv11 - nv12) * s.dn[338][19]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n20: f64 = (-(((nv11 - nv12) * s.dn[338][20]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n21: f64 = (-(((nv11 - nv12) * s.dn[338][21]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n22: f64 = (-(((nv11 - nv12) * s.dn[338][22]) / (s.v[338] * s.v[338])));
        (eq36_e646, eq36_e646_d_n0, eq36_e646_d_n1, eq36_e646_d_n2, eq36_e646_d_n3, eq36_e646_d_n4, eq36_e646_d_n5, eq36_e646_d_n6, eq36_e646_d_n7, eq36_e646_d_n8, eq36_e646_d_n9, eq36_e646_d_n10, eq36_e646_d_n11, eq36_e646_d_n12, eq36_e646_d_n13, eq36_e646_d_n14, eq36_e646_d_n15, eq36_e646_d_n16, eq36_e646_d_n17, eq36_e646_d_n18, eq36_e646_d_n19, eq36_e646_d_n20, eq36_e646_d_n21, eq36_e646_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e648;
        let eq36_node_derivatives: [f64; 23] = [eq36_e648_d_n0, eq36_e648_d_n1, eq36_e648_d_n2, eq36_e648_d_n3, eq36_e648_d_n4, eq36_e648_d_n5, eq36_e648_d_n6, eq36_e648_d_n7, eq36_e648_d_n8, eq36_e648_d_n9, eq36_e648_d_n10, eq36_e648_d_n11, eq36_e648_d_n12, eq36_e648_d_n13, eq36_e648_d_n14, eq36_e648_d_n15, eq36_e648_d_n16, eq36_e648_d_n17, eq36_e648_d_n18, eq36_e648_d_n19, eq36_e648_d_n20, eq36_e648_d_n21, eq36_e648_d_n22];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            self.multiplicity * (eq36_value),
            &nodes,
            &eq36_node_derivatives,
            &branches,
            &eq36_branch_derivatives,
            self.multiplicity,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq37_e668, eq37_e668_d_n0, eq37_e668_d_n1, eq37_e668_d_n2, eq37_e668_d_n3, eq37_e668_d_n4, eq37_e668_d_n5, eq37_e668_d_n6, eq37_e668_d_n7, eq37_e668_d_n8, eq37_e668_d_n9, eq37_e668_d_n10, eq37_e668_d_n11, eq37_e668_d_n12, eq37_e668_d_n13, eq37_e668_d_n14, eq37_e668_d_n15, eq37_e668_d_n16, eq37_e668_d_n17, eq37_e668_d_n18, eq37_e668_d_n19, eq37_e668_d_n20, eq37_e668_d_n21, eq37_e668_d_n22,) = {
    if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
        let eq37_e661: f64 = self.eval_ddt(4, (nv12 - 0.0));
        let eq37_e661_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n12: f64 = self.ddt_jacobian(1.0);
        let eq37_e661_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq37_e661_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq37_e662: f64 = (p.p97 * eq37_e661);
        let eq37_e662_d_n0: f64 = (p.p97 * eq37_e661_d_n0);
        let eq37_e662_d_n1: f64 = (p.p97 * eq37_e661_d_n1);
        let eq37_e662_d_n2: f64 = (p.p97 * eq37_e661_d_n2);
        let eq37_e662_d_n3: f64 = (p.p97 * eq37_e661_d_n3);
        let eq37_e662_d_n4: f64 = (p.p97 * eq37_e661_d_n4);
        let eq37_e662_d_n5: f64 = (p.p97 * eq37_e661_d_n5);
        let eq37_e662_d_n6: f64 = (p.p97 * eq37_e661_d_n6);
        let eq37_e662_d_n7: f64 = (p.p97 * eq37_e661_d_n7);
        let eq37_e662_d_n8: f64 = (p.p97 * eq37_e661_d_n8);
        let eq37_e662_d_n9: f64 = (p.p97 * eq37_e661_d_n9);
        let eq37_e662_d_n10: f64 = (p.p97 * eq37_e661_d_n10);
        let eq37_e662_d_n11: f64 = (p.p97 * eq37_e661_d_n11);
        let eq37_e662_d_n12: f64 = (p.p97 * eq37_e661_d_n12);
        let eq37_e662_d_n13: f64 = (p.p97 * eq37_e661_d_n13);
        let eq37_e662_d_n14: f64 = (p.p97 * eq37_e661_d_n14);
        let eq37_e662_d_n15: f64 = (p.p97 * eq37_e661_d_n15);
        let eq37_e662_d_n16: f64 = (p.p97 * eq37_e661_d_n16);
        let eq37_e662_d_n17: f64 = (p.p97 * eq37_e661_d_n17);
        let eq37_e662_d_n18: f64 = (p.p97 * eq37_e661_d_n18);
        let eq37_e662_d_n19: f64 = (p.p97 * eq37_e661_d_n19);
        let eq37_e662_d_n20: f64 = (p.p97 * eq37_e661_d_n20);
        let eq37_e662_d_n21: f64 = (p.p97 * eq37_e661_d_n21);
        let eq37_e662_d_n22: f64 = (p.p97 * eq37_e661_d_n22);
        let eq37_e665: f64 = (1e-12 * (nv12 - 0.0));
        let eq37_e665_d_n12: f64 = 1e-12;
        let eq37_e666: f64 = (eq37_e662 + eq37_e665);
        let eq37_e666_d_n12: f64 = (eq37_e662_d_n12 + eq37_e665_d_n12);
        (eq37_e666, eq37_e662_d_n0, eq37_e662_d_n1, eq37_e662_d_n2, eq37_e662_d_n3, eq37_e662_d_n4, eq37_e662_d_n5, eq37_e662_d_n6, eq37_e662_d_n7, eq37_e662_d_n8, eq37_e662_d_n9, eq37_e662_d_n10, eq37_e662_d_n11, eq37_e666_d_n12, eq37_e662_d_n13, eq37_e662_d_n14, eq37_e662_d_n15, eq37_e662_d_n16, eq37_e662_d_n17, eq37_e662_d_n18, eq37_e662_d_n19, eq37_e662_d_n20, eq37_e662_d_n21, eq37_e662_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e668;
        let eq37_node_derivatives: [f64; 23] = [eq37_e668_d_n0, eq37_e668_d_n1, eq37_e668_d_n2, eq37_e668_d_n3, eq37_e668_d_n4, eq37_e668_d_n5, eq37_e668_d_n6, eq37_e668_d_n7, eq37_e668_d_n8, eq37_e668_d_n9, eq37_e668_d_n10, eq37_e668_d_n11, eq37_e668_d_n12, eq37_e668_d_n13, eq37_e668_d_n14, eq37_e668_d_n15, eq37_e668_d_n16, eq37_e668_d_n17, eq37_e668_d_n18, eq37_e668_d_n19, eq37_e668_d_n20, eq37_e668_d_n21, eq37_e668_d_n22];
        let eq37_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq37_value),
            &nodes,
            &eq37_node_derivatives,
            &branches,
            &eq37_branch_derivatives,
            self.multiplicity,
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
        let (eq38_e681, eq38_e681_d_n0, eq38_e681_d_n1, eq38_e681_d_n2, eq38_e681_d_n3, eq38_e681_d_n4, eq38_e681_d_n5, eq38_e681_d_n6, eq38_e681_d_n7, eq38_e681_d_n8, eq38_e681_d_n9, eq38_e681_d_n10, eq38_e681_d_n11, eq38_e681_d_n12, eq38_e681_d_n13, eq38_e681_d_n14, eq38_e681_d_n15, eq38_e681_d_n16, eq38_e681_d_n17, eq38_e681_d_n18, eq38_e681_d_n19, eq38_e681_d_n20, eq38_e681_d_n21, eq38_e681_d_n22,) = {
    if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
        (s.v[90], s.dn[90][0], s.dn[90][1], s.dn[90][2], s.dn[90][3], s.dn[90][4], s.dn[90][5], s.dn[90][6], s.dn[90][7], s.dn[90][8], s.dn[90][9], s.dn[90][10], s.dn[90][11], s.dn[90][12], s.dn[90][13], s.dn[90][14], s.dn[90][15], s.dn[90][16], s.dn[90][17], s.dn[90][18], s.dn[90][19], s.dn[90][20], s.dn[90][21], s.dn[90][22],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e681;
        let eq38_node_derivatives: [f64; 23] = [eq38_e681_d_n0, eq38_e681_d_n1, eq38_e681_d_n2, eq38_e681_d_n3, eq38_e681_d_n4, eq38_e681_d_n5, eq38_e681_d_n6, eq38_e681_d_n7, eq38_e681_d_n8, eq38_e681_d_n9, eq38_e681_d_n10, eq38_e681_d_n11, eq38_e681_d_n12, eq38_e681_d_n13, eq38_e681_d_n14, eq38_e681_d_n15, eq38_e681_d_n16, eq38_e681_d_n17, eq38_e681_d_n18, eq38_e681_d_n19, eq38_e681_d_n20, eq38_e681_d_n21, eq38_e681_d_n22];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_dense(
            branches[24],
            eq38_value,
            &nodes,
            &eq38_node_derivatives,
            &branches,
            &eq38_branch_derivatives,
        );
    }
}
