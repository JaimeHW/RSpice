#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq16_e204,) = {
    if (((!(s.v[70] != 0.0)) && (!(s.v[71] != 0.0))) && (!(s.v[72] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e204;
        stamper.stamp_potential(
            branches[4],
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq17_e207: f64 = 0.0;
        let eq17_e209: f64 = (eq17_e207 * (nv3 - nv4));
        let eq17_e209_d_n4: f64 = (-eq17_e207);
        let eq17_value: f64 = eq17_e209;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[4]),
            self.multiplicity * (eq17_value),
            &[
                GeneratedDerivative::node(nodes[3], self.multiplicity * eq17_e207),
                GeneratedDerivative::node(nodes[4], self.multiplicity * eq17_e209_d_n4),
            ],
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let (eq18_e224, eq18_e224_d_n0, eq18_e224_d_n1, eq18_e224_d_n2, eq18_e224_d_n3, eq18_e224_d_n4, eq18_e224_d_n5, eq18_e224_d_n6, eq18_e224_d_b0, eq18_e224_d_b1, eq18_e224_d_b2, eq18_e224_d_b3, eq18_e224_d_b4, eq18_e224_d_b5, eq18_e224_d_b6,) = {
    if (s.v[73] != 0.0) {
        let eq18_e214: f64 = (s.v[29] / s.v[3]);
        let eq18_e214_d_n0: f64 = (s.dn[29][0] / s.v[3]);
        let eq18_e214_d_n1: f64 = (s.dn[29][1] / s.v[3]);
        let eq18_e214_d_n2: f64 = (s.dn[29][2] / s.v[3]);
        let eq18_e214_d_n3: f64 = (s.dn[29][3] / s.v[3]);
        let eq18_e214_d_n4: f64 = (s.dn[29][4] / s.v[3]);
        let eq18_e214_d_n5: f64 = (s.dn[29][5] / s.v[3]);
        let eq18_e214_d_n6: f64 = (s.dn[29][6] / s.v[3]);
        let eq18_e214_d_b0: f64 = (s.db[29][0] / s.v[3]);
        let eq18_e214_d_b1: f64 = (s.db[29][1] / s.v[3]);
        let eq18_e214_d_b2: f64 = (s.db[29][2] / s.v[3]);
        let eq18_e214_d_b3: f64 = (s.db[29][3] / s.v[3]);
        let eq18_e214_d_b4: f64 = (s.db[29][4] / s.v[3]);
        let eq18_e214_d_b5: f64 = (s.db[29][5] / s.v[3]);
        let eq18_e214_d_b6: f64 = (s.db[29][6] / s.v[3]);
        let (eq18_e221, eq18_e221_d_n0, eq18_e221_d_n1, eq18_e221_d_n2, eq18_e221_d_n3, eq18_e221_d_n4, eq18_e221_d_n5, eq18_e221_d_n6, eq18_e221_d_b0, eq18_e221_d_b1, eq18_e221_d_b2, eq18_e221_d_b3, eq18_e221_d_b4, eq18_e221_d_b5, eq18_e221_d_b6,) = {
            if (eq18_e214 > p.p46) {
                let eq18_e219: f64 = (s.v[29] / s.v[3]);
                let eq18_e219_d_n0: f64 = (s.dn[29][0] / s.v[3]);
                let eq18_e219_d_n1: f64 = (s.dn[29][1] / s.v[3]);
                let eq18_e219_d_n2: f64 = (s.dn[29][2] / s.v[3]);
                let eq18_e219_d_n3: f64 = (s.dn[29][3] / s.v[3]);
                let eq18_e219_d_n4: f64 = (s.dn[29][4] / s.v[3]);
                let eq18_e219_d_n5: f64 = (s.dn[29][5] / s.v[3]);
                let eq18_e219_d_n6: f64 = (s.dn[29][6] / s.v[3]);
                let eq18_e219_d_b0: f64 = (s.db[29][0] / s.v[3]);
                let eq18_e219_d_b1: f64 = (s.db[29][1] / s.v[3]);
                let eq18_e219_d_b2: f64 = (s.db[29][2] / s.v[3]);
                let eq18_e219_d_b3: f64 = (s.db[29][3] / s.v[3]);
                let eq18_e219_d_b4: f64 = (s.db[29][4] / s.v[3]);
                let eq18_e219_d_b5: f64 = (s.db[29][5] / s.v[3]);
                let eq18_e219_d_b6: f64 = (s.db[29][6] / s.v[3]);
                (eq18_e219, eq18_e219_d_n0, eq18_e219_d_n1, eq18_e219_d_n2, eq18_e219_d_n3, eq18_e219_d_n4, eq18_e219_d_n5, eq18_e219_d_n6, eq18_e219_d_b0, eq18_e219_d_b1, eq18_e219_d_b2, eq18_e219_d_b3, eq18_e219_d_b4, eq18_e219_d_b5, eq18_e219_d_b6,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq18_e222: f64 = ((nv0 - nv3) / eq18_e221);
        let eq18_e222_d_n0: f64 = ((eq18_e221 - ((nv0 - nv3) * eq18_e221_d_n0)) / (eq18_e221 * eq18_e221));
        let eq18_e222_d_n1: f64 = (-(((nv0 - nv3) * eq18_e221_d_n1) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_n2: f64 = (-(((nv0 - nv3) * eq18_e221_d_n2) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_n3: f64 = (((-eq18_e221) - ((nv0 - nv3) * eq18_e221_d_n3)) / (eq18_e221 * eq18_e221));
        let eq18_e222_d_n4: f64 = (-(((nv0 - nv3) * eq18_e221_d_n4) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_n5: f64 = (-(((nv0 - nv3) * eq18_e221_d_n5) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_n6: f64 = (-(((nv0 - nv3) * eq18_e221_d_n6) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b0: f64 = (-(((nv0 - nv3) * eq18_e221_d_b0) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b1: f64 = (-(((nv0 - nv3) * eq18_e221_d_b1) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b2: f64 = (-(((nv0 - nv3) * eq18_e221_d_b2) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b3: f64 = (-(((nv0 - nv3) * eq18_e221_d_b3) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b4: f64 = (-(((nv0 - nv3) * eq18_e221_d_b4) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b5: f64 = (-(((nv0 - nv3) * eq18_e221_d_b5) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b6: f64 = (-(((nv0 - nv3) * eq18_e221_d_b6) / (eq18_e221 * eq18_e221)));
        (eq18_e222, eq18_e222_d_n0, eq18_e222_d_n1, eq18_e222_d_n2, eq18_e222_d_n3, eq18_e222_d_n4, eq18_e222_d_n5, eq18_e222_d_n6, eq18_e222_d_b0, eq18_e222_d_b1, eq18_e222_d_b2, eq18_e222_d_b3, eq18_e222_d_b4, eq18_e222_d_b5, eq18_e222_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e224;
        let eq18_node_derivatives: [f64; 7] = [eq18_e224_d_n0, eq18_e224_d_n1, eq18_e224_d_n2, eq18_e224_d_n3, eq18_e224_d_n4, eq18_e224_d_n5, eq18_e224_d_n6];
        let eq18_branch_derivatives: [f64; 7] = [eq18_e224_d_b0, eq18_e224_d_b1, eq18_e224_d_b2, eq18_e224_d_b3, eq18_e224_d_b4, eq18_e224_d_b5, eq18_e224_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            self.multiplicity * (eq18_value),
            &nodes,
            &eq18_node_derivatives,
            &branches,
            &eq18_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq19_e230,) = {
    if (s.v[73] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e230;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[3]),
            self.multiplicity * (eq19_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq20_e235,) = {
    if (!(s.v[73] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e235;
        stamper.stamp_potential(
            branches[5],
            eq20_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_21_block_0(
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq21_e250, eq21_e250_d_n0, eq21_e250_d_n1, eq21_e250_d_n2, eq21_e250_d_n3, eq21_e250_d_n4, eq21_e250_d_n5, eq21_e250_d_n6, eq21_e250_d_b0, eq21_e250_d_b1, eq21_e250_d_b2, eq21_e250_d_b3, eq21_e250_d_b4, eq21_e250_d_b5, eq21_e250_d_b6,) = {
    if (s.v[74] != 0.0) {
        let eq21_e240: f64 = (s.v[30] / s.v[3]);
        let eq21_e240_d_n0: f64 = (s.dn[30][0] / s.v[3]);
        let eq21_e240_d_n1: f64 = (s.dn[30][1] / s.v[3]);
        let eq21_e240_d_n2: f64 = (s.dn[30][2] / s.v[3]);
        let eq21_e240_d_n3: f64 = (s.dn[30][3] / s.v[3]);
        let eq21_e240_d_n4: f64 = (s.dn[30][4] / s.v[3]);
        let eq21_e240_d_n5: f64 = (s.dn[30][5] / s.v[3]);
        let eq21_e240_d_n6: f64 = (s.dn[30][6] / s.v[3]);
        let eq21_e240_d_b0: f64 = (s.db[30][0] / s.v[3]);
        let eq21_e240_d_b1: f64 = (s.db[30][1] / s.v[3]);
        let eq21_e240_d_b2: f64 = (s.db[30][2] / s.v[3]);
        let eq21_e240_d_b3: f64 = (s.db[30][3] / s.v[3]);
        let eq21_e240_d_b4: f64 = (s.db[30][4] / s.v[3]);
        let eq21_e240_d_b5: f64 = (s.db[30][5] / s.v[3]);
        let eq21_e240_d_b6: f64 = (s.db[30][6] / s.v[3]);
        let (eq21_e247, eq21_e247_d_n0, eq21_e247_d_n1, eq21_e247_d_n2, eq21_e247_d_n3, eq21_e247_d_n4, eq21_e247_d_n5, eq21_e247_d_n6, eq21_e247_d_b0, eq21_e247_d_b1, eq21_e247_d_b2, eq21_e247_d_b3, eq21_e247_d_b4, eq21_e247_d_b5, eq21_e247_d_b6,) = {
            if (eq21_e240 > p.p46) {
                let eq21_e245: f64 = (s.v[30] / s.v[3]);
                let eq21_e245_d_n0: f64 = (s.dn[30][0] / s.v[3]);
                let eq21_e245_d_n1: f64 = (s.dn[30][1] / s.v[3]);
                let eq21_e245_d_n2: f64 = (s.dn[30][2] / s.v[3]);
                let eq21_e245_d_n3: f64 = (s.dn[30][3] / s.v[3]);
                let eq21_e245_d_n4: f64 = (s.dn[30][4] / s.v[3]);
                let eq21_e245_d_n5: f64 = (s.dn[30][5] / s.v[3]);
                let eq21_e245_d_n6: f64 = (s.dn[30][6] / s.v[3]);
                let eq21_e245_d_b0: f64 = (s.db[30][0] / s.v[3]);
                let eq21_e245_d_b1: f64 = (s.db[30][1] / s.v[3]);
                let eq21_e245_d_b2: f64 = (s.db[30][2] / s.v[3]);
                let eq21_e245_d_b3: f64 = (s.db[30][3] / s.v[3]);
                let eq21_e245_d_b4: f64 = (s.db[30][4] / s.v[3]);
                let eq21_e245_d_b5: f64 = (s.db[30][5] / s.v[3]);
                let eq21_e245_d_b6: f64 = (s.db[30][6] / s.v[3]);
                (eq21_e245, eq21_e245_d_n0, eq21_e245_d_n1, eq21_e245_d_n2, eq21_e245_d_n3, eq21_e245_d_n4, eq21_e245_d_n5, eq21_e245_d_n6, eq21_e245_d_b0, eq21_e245_d_b1, eq21_e245_d_b2, eq21_e245_d_b3, eq21_e245_d_b4, eq21_e245_d_b5, eq21_e245_d_b6,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq21_e248: f64 = ((nv1 - nv4) / eq21_e247);
        let eq21_e248_d_n0: f64 = (-(((nv1 - nv4) * eq21_e247_d_n0) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_n1: f64 = ((eq21_e247 - ((nv1 - nv4) * eq21_e247_d_n1)) / (eq21_e247 * eq21_e247));
        let eq21_e248_d_n2: f64 = (-(((nv1 - nv4) * eq21_e247_d_n2) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_n3: f64 = (-(((nv1 - nv4) * eq21_e247_d_n3) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_n4: f64 = (((-eq21_e247) - ((nv1 - nv4) * eq21_e247_d_n4)) / (eq21_e247 * eq21_e247));
        let eq21_e248_d_n5: f64 = (-(((nv1 - nv4) * eq21_e247_d_n5) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_n6: f64 = (-(((nv1 - nv4) * eq21_e247_d_n6) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b0: f64 = (-(((nv1 - nv4) * eq21_e247_d_b0) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b1: f64 = (-(((nv1 - nv4) * eq21_e247_d_b1) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b2: f64 = (-(((nv1 - nv4) * eq21_e247_d_b2) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b3: f64 = (-(((nv1 - nv4) * eq21_e247_d_b3) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b4: f64 = (-(((nv1 - nv4) * eq21_e247_d_b4) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b5: f64 = (-(((nv1 - nv4) * eq21_e247_d_b5) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b6: f64 = (-(((nv1 - nv4) * eq21_e247_d_b6) / (eq21_e247 * eq21_e247)));
        (eq21_e248, eq21_e248_d_n0, eq21_e248_d_n1, eq21_e248_d_n2, eq21_e248_d_n3, eq21_e248_d_n4, eq21_e248_d_n5, eq21_e248_d_n6, eq21_e248_d_b0, eq21_e248_d_b1, eq21_e248_d_b2, eq21_e248_d_b3, eq21_e248_d_b4, eq21_e248_d_b5, eq21_e248_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e250;
        let eq21_node_derivatives: [f64; 7] = [eq21_e250_d_n0, eq21_e250_d_n1, eq21_e250_d_n2, eq21_e250_d_n3, eq21_e250_d_n4, eq21_e250_d_n5, eq21_e250_d_n6];
        let eq21_branch_derivatives: [f64; 7] = [eq21_e250_d_b0, eq21_e250_d_b1, eq21_e250_d_b2, eq21_e250_d_b3, eq21_e250_d_b4, eq21_e250_d_b5, eq21_e250_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            self.multiplicity * (eq21_value),
            &nodes,
            &eq21_node_derivatives,
            &branches,
            &eq21_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq22_e256,) = {
    if (s.v[74] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e256;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[4]),
            self.multiplicity * (eq22_value),
            &[
            ],
        );
    }

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
        let (eq23_e261,) = {
    if (!(s.v[74] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e261;
        stamper.stamp_potential(
            branches[6],
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
        let eq24_e264: f64 = (s.v[9] * s.v[24]);
        let eq24_e264_d_n0: f64 = (s.v[9] * s.dn[24][0]);
        let eq24_e264_d_n1: f64 = (s.v[9] * s.dn[24][1]);
        let eq24_e264_d_n2: f64 = (s.v[9] * s.dn[24][2]);
        let eq24_e264_d_n3: f64 = (s.v[9] * s.dn[24][3]);
        let eq24_e264_d_n4: f64 = (s.v[9] * s.dn[24][4]);
        let eq24_e264_d_n5: f64 = (s.v[9] * s.dn[24][5]);
        let eq24_e264_d_n6: f64 = (s.v[9] * s.dn[24][6]);
        let eq24_e264_d_b0: f64 = (s.v[9] * s.db[24][0]);
        let eq24_e264_d_b1: f64 = (s.v[9] * s.db[24][1]);
        let eq24_e264_d_b2: f64 = (s.v[9] * s.db[24][2]);
        let eq24_e264_d_b3: f64 = (s.v[9] * s.db[24][3]);
        let eq24_e264_d_b4: f64 = (s.v[9] * s.db[24][4]);
        let eq24_e264_d_b5: f64 = (s.v[9] * s.db[24][5]);
        let eq24_e264_d_b6: f64 = (s.v[9] * s.db[24][6]);
        let eq24_e266: f64 = (eq24_e264 * s.v[3]);
        let eq24_e266_d_n0: f64 = (eq24_e264_d_n0 * s.v[3]);
        let eq24_e266_d_n1: f64 = (eq24_e264_d_n1 * s.v[3]);
        let eq24_e266_d_n2: f64 = (eq24_e264_d_n2 * s.v[3]);
        let eq24_e266_d_n3: f64 = (eq24_e264_d_n3 * s.v[3]);
        let eq24_e266_d_n4: f64 = (eq24_e264_d_n4 * s.v[3]);
        let eq24_e266_d_n5: f64 = (eq24_e264_d_n5 * s.v[3]);
        let eq24_e266_d_n6: f64 = (eq24_e264_d_n6 * s.v[3]);
        let eq24_e266_d_b0: f64 = (eq24_e264_d_b0 * s.v[3]);
        let eq24_e266_d_b1: f64 = (eq24_e264_d_b1 * s.v[3]);
        let eq24_e266_d_b2: f64 = (eq24_e264_d_b2 * s.v[3]);
        let eq24_e266_d_b3: f64 = (eq24_e264_d_b3 * s.v[3]);
        let eq24_e266_d_b4: f64 = (eq24_e264_d_b4 * s.v[3]);
        let eq24_e266_d_b5: f64 = (eq24_e264_d_b5 * s.v[3]);
        let eq24_e266_d_b6: f64 = (eq24_e264_d_b6 * s.v[3]);
        let eq24_value: f64 = eq24_e266;
        let eq24_node_derivatives: [f64; 7] = [eq24_e266_d_n0, eq24_e266_d_n1, eq24_e266_d_n2, eq24_e266_d_n3, eq24_e266_d_n4, eq24_e266_d_n5, eq24_e266_d_n6];
        let eq24_branch_derivatives: [f64; 7] = [eq24_e266_d_b0, eq24_e266_d_b1, eq24_e266_d_b2, eq24_e266_d_b3, eq24_e266_d_b4, eq24_e266_d_b5, eq24_e266_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[4]),
            self.multiplicity * (eq24_value),
            &nodes,
            &eq24_node_derivatives,
            &branches,
            &eq24_branch_derivatives,
            self.multiplicity,
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
        let eq25_e269: f64 = (s.v[9] * s.v[33]);
        let eq25_e269_d_n0: f64 = (s.v[9] * s.dn[33][0]);
        let eq25_e269_d_n1: f64 = (s.v[9] * s.dn[33][1]);
        let eq25_e269_d_n2: f64 = (s.v[9] * s.dn[33][2]);
        let eq25_e269_d_n3: f64 = (s.v[9] * s.dn[33][3]);
        let eq25_e269_d_n4: f64 = (s.v[9] * s.dn[33][4]);
        let eq25_e269_d_n5: f64 = (s.v[9] * s.dn[33][5]);
        let eq25_e269_d_n6: f64 = (s.v[9] * s.dn[33][6]);
        let eq25_e269_d_b0: f64 = (s.v[9] * s.db[33][0]);
        let eq25_e269_d_b1: f64 = (s.v[9] * s.db[33][1]);
        let eq25_e269_d_b2: f64 = (s.v[9] * s.db[33][2]);
        let eq25_e269_d_b3: f64 = (s.v[9] * s.db[33][3]);
        let eq25_e269_d_b4: f64 = (s.v[9] * s.db[33][4]);
        let eq25_e269_d_b5: f64 = (s.v[9] * s.db[33][5]);
        let eq25_e269_d_b6: f64 = (s.v[9] * s.db[33][6]);
        let eq25_e271: f64 = (eq25_e269 * s.v[3]);
        let eq25_e271_d_n0: f64 = (eq25_e269_d_n0 * s.v[3]);
        let eq25_e271_d_n1: f64 = (eq25_e269_d_n1 * s.v[3]);
        let eq25_e271_d_n2: f64 = (eq25_e269_d_n2 * s.v[3]);
        let eq25_e271_d_n3: f64 = (eq25_e269_d_n3 * s.v[3]);
        let eq25_e271_d_n4: f64 = (eq25_e269_d_n4 * s.v[3]);
        let eq25_e271_d_n5: f64 = (eq25_e269_d_n5 * s.v[3]);
        let eq25_e271_d_n6: f64 = (eq25_e269_d_n6 * s.v[3]);
        let eq25_e271_d_b0: f64 = (eq25_e269_d_b0 * s.v[3]);
        let eq25_e271_d_b1: f64 = (eq25_e269_d_b1 * s.v[3]);
        let eq25_e271_d_b2: f64 = (eq25_e269_d_b2 * s.v[3]);
        let eq25_e271_d_b3: f64 = (eq25_e269_d_b3 * s.v[3]);
        let eq25_e271_d_b4: f64 = (eq25_e269_d_b4 * s.v[3]);
        let eq25_e271_d_b5: f64 = (eq25_e269_d_b5 * s.v[3]);
        let eq25_e271_d_b6: f64 = (eq25_e269_d_b6 * s.v[3]);
        let eq25_e272: f64 = self.eval_ddt(4, eq25_e271);
        let eq25_e272_d_n0: f64 = self.ddt_jacobian(eq25_e271_d_n0);
        let eq25_e272_d_n1: f64 = self.ddt_jacobian(eq25_e271_d_n1);
        let eq25_e272_d_n2: f64 = self.ddt_jacobian(eq25_e271_d_n2);
        let eq25_e272_d_n3: f64 = self.ddt_jacobian(eq25_e271_d_n3);
        let eq25_e272_d_n4: f64 = self.ddt_jacobian(eq25_e271_d_n4);
        let eq25_e272_d_n5: f64 = self.ddt_jacobian(eq25_e271_d_n5);
        let eq25_e272_d_n6: f64 = self.ddt_jacobian(eq25_e271_d_n6);
        let eq25_e272_d_b0: f64 = self.ddt_jacobian(eq25_e271_d_b0);
        let eq25_e272_d_b1: f64 = self.ddt_jacobian(eq25_e271_d_b1);
        let eq25_e272_d_b2: f64 = self.ddt_jacobian(eq25_e271_d_b2);
        let eq25_e272_d_b3: f64 = self.ddt_jacobian(eq25_e271_d_b3);
        let eq25_e272_d_b4: f64 = self.ddt_jacobian(eq25_e271_d_b4);
        let eq25_e272_d_b5: f64 = self.ddt_jacobian(eq25_e271_d_b5);
        let eq25_e272_d_b6: f64 = self.ddt_jacobian(eq25_e271_d_b6);
        let eq25_value: f64 = eq25_e272;
        let eq25_node_derivatives: [f64; 7] = [eq25_e272_d_n0, eq25_e272_d_n1, eq25_e272_d_n2, eq25_e272_d_n3, eq25_e272_d_n4, eq25_e272_d_n5, eq25_e272_d_n6];
        let eq25_branch_derivatives: [f64; 7] = [eq25_e272_d_b0, eq25_e272_d_b1, eq25_e272_d_b2, eq25_e272_d_b3, eq25_e272_d_b4, eq25_e272_d_b5, eq25_e272_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[4]),
            self.multiplicity * (eq25_value),
            &nodes,
            &eq25_node_derivatives,
            &branches,
            &eq25_branch_derivatives,
            self.multiplicity,
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
        let eq26_e275: f64 = (s.v[9] * s.v[32]);
        let eq26_e275_d_n0: f64 = (s.v[9] * s.dn[32][0]);
        let eq26_e275_d_n1: f64 = (s.v[9] * s.dn[32][1]);
        let eq26_e275_d_n2: f64 = (s.v[9] * s.dn[32][2]);
        let eq26_e275_d_n3: f64 = (s.v[9] * s.dn[32][3]);
        let eq26_e275_d_n4: f64 = (s.v[9] * s.dn[32][4]);
        let eq26_e275_d_n5: f64 = (s.v[9] * s.dn[32][5]);
        let eq26_e275_d_n6: f64 = (s.v[9] * s.dn[32][6]);
        let eq26_e275_d_b0: f64 = (s.v[9] * s.db[32][0]);
        let eq26_e275_d_b1: f64 = (s.v[9] * s.db[32][1]);
        let eq26_e275_d_b2: f64 = (s.v[9] * s.db[32][2]);
        let eq26_e275_d_b3: f64 = (s.v[9] * s.db[32][3]);
        let eq26_e275_d_b4: f64 = (s.v[9] * s.db[32][4]);
        let eq26_e275_d_b5: f64 = (s.v[9] * s.db[32][5]);
        let eq26_e275_d_b6: f64 = (s.v[9] * s.db[32][6]);
        let eq26_e277: f64 = (eq26_e275 * s.v[3]);
        let eq26_e277_d_n0: f64 = (eq26_e275_d_n0 * s.v[3]);
        let eq26_e277_d_n1: f64 = (eq26_e275_d_n1 * s.v[3]);
        let eq26_e277_d_n2: f64 = (eq26_e275_d_n2 * s.v[3]);
        let eq26_e277_d_n3: f64 = (eq26_e275_d_n3 * s.v[3]);
        let eq26_e277_d_n4: f64 = (eq26_e275_d_n4 * s.v[3]);
        let eq26_e277_d_n5: f64 = (eq26_e275_d_n5 * s.v[3]);
        let eq26_e277_d_n6: f64 = (eq26_e275_d_n6 * s.v[3]);
        let eq26_e277_d_b0: f64 = (eq26_e275_d_b0 * s.v[3]);
        let eq26_e277_d_b1: f64 = (eq26_e275_d_b1 * s.v[3]);
        let eq26_e277_d_b2: f64 = (eq26_e275_d_b2 * s.v[3]);
        let eq26_e277_d_b3: f64 = (eq26_e275_d_b3 * s.v[3]);
        let eq26_e277_d_b4: f64 = (eq26_e275_d_b4 * s.v[3]);
        let eq26_e277_d_b5: f64 = (eq26_e275_d_b5 * s.v[3]);
        let eq26_e277_d_b6: f64 = (eq26_e275_d_b6 * s.v[3]);
        let eq26_e278: f64 = self.eval_ddt(5, eq26_e277);
        let eq26_e278_d_n0: f64 = self.ddt_jacobian(eq26_e277_d_n0);
        let eq26_e278_d_n1: f64 = self.ddt_jacobian(eq26_e277_d_n1);
        let eq26_e278_d_n2: f64 = self.ddt_jacobian(eq26_e277_d_n2);
        let eq26_e278_d_n3: f64 = self.ddt_jacobian(eq26_e277_d_n3);
        let eq26_e278_d_n4: f64 = self.ddt_jacobian(eq26_e277_d_n4);
        let eq26_e278_d_n5: f64 = self.ddt_jacobian(eq26_e277_d_n5);
        let eq26_e278_d_n6: f64 = self.ddt_jacobian(eq26_e277_d_n6);
        let eq26_e278_d_b0: f64 = self.ddt_jacobian(eq26_e277_d_b0);
        let eq26_e278_d_b1: f64 = self.ddt_jacobian(eq26_e277_d_b1);
        let eq26_e278_d_b2: f64 = self.ddt_jacobian(eq26_e277_d_b2);
        let eq26_e278_d_b3: f64 = self.ddt_jacobian(eq26_e277_d_b3);
        let eq26_e278_d_b4: f64 = self.ddt_jacobian(eq26_e277_d_b4);
        let eq26_e278_d_b5: f64 = self.ddt_jacobian(eq26_e277_d_b5);
        let eq26_e278_d_b6: f64 = self.ddt_jacobian(eq26_e277_d_b6);
        let eq26_value: f64 = eq26_e278;
        let eq26_node_derivatives: [f64; 7] = [eq26_e278_d_n0, eq26_e278_d_n1, eq26_e278_d_n2, eq26_e278_d_n3, eq26_e278_d_n4, eq26_e278_d_n5, eq26_e278_d_n6];
        let eq26_branch_derivatives: [f64; 7] = [eq26_e278_d_b0, eq26_e278_d_b1, eq26_e278_d_b2, eq26_e278_d_b3, eq26_e278_d_b4, eq26_e278_d_b5, eq26_e278_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[4]),
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
        let eq27_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[4]),
            self.multiplicity * (eq27_value),
            &[
            ],
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
        let eq28_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[4]),
            self.multiplicity * (eq28_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq2_e73, eq2_e73_d_n0, eq2_e73_d_n1, eq2_e73_d_n2, eq2_e73_d_n3, eq2_e73_d_n4, eq2_e73_d_n5, eq2_e73_d_n6, eq2_e73_d_b0, eq2_e73_d_b1, eq2_e73_d_b2, eq2_e73_d_b3, eq2_e73_d_b4, eq2_e73_d_b5, eq2_e73_d_b6, eq2_e73_q, eq2_e73_q_d_n0, eq2_e73_q_d_n1, eq2_e73_q_d_n2, eq2_e73_q_d_n3, eq2_e73_q_d_n4, eq2_e73_q_d_n5, eq2_e73_q_d_n6, eq2_e73_q_d_b0, eq2_e73_q_d_b1, eq2_e73_q_d_b2, eq2_e73_q_d_b3, eq2_e73_q_d_b4, eq2_e73_q_d_b5, eq2_e73_q_d_b6,) = {
    if (s.v[68] != 0.0) {
        let eq2_e70_q: f64 = (nv6 - 0.0);
        let eq2_e71: f64 = (s.v[31] * (nv6 - 0.0));
        let eq2_e71_d_n0: f64 = (s.dn[31][0] * (nv6 - 0.0));
        let eq2_e71_d_n1: f64 = (s.dn[31][1] * (nv6 - 0.0));
        let eq2_e71_d_n2: f64 = (s.dn[31][2] * (nv6 - 0.0));
        let eq2_e71_d_n3: f64 = (s.dn[31][3] * (nv6 - 0.0));
        let eq2_e71_d_n4: f64 = (s.dn[31][4] * (nv6 - 0.0));
        let eq2_e71_d_n5: f64 = (s.dn[31][5] * (nv6 - 0.0));
        let eq2_e71_d_n6: f64 = ((s.dn[31][6] * (nv6 - 0.0)) + s.v[31]);
        let eq2_e71_d_b0: f64 = (s.db[31][0] * (nv6 - 0.0));
        let eq2_e71_d_b1: f64 = (s.db[31][1] * (nv6 - 0.0));
        let eq2_e71_d_b2: f64 = (s.db[31][2] * (nv6 - 0.0));
        let eq2_e71_d_b3: f64 = (s.db[31][3] * (nv6 - 0.0));
        let eq2_e71_d_b4: f64 = (s.db[31][4] * (nv6 - 0.0));
        let eq2_e71_d_b5: f64 = (s.db[31][5] * (nv6 - 0.0));
        let eq2_e71_d_b6: f64 = (s.db[31][6] * (nv6 - 0.0));
        let eq2_e71_q: f64 = (s.v[31] * eq2_e70_q);
        let eq2_e71_q_d_n0: f64 = (s.dn[31][0] * eq2_e70_q);
        let eq2_e71_q_d_n1: f64 = (s.dn[31][1] * eq2_e70_q);
        let eq2_e71_q_d_n2: f64 = (s.dn[31][2] * eq2_e70_q);
        let eq2_e71_q_d_n3: f64 = (s.dn[31][3] * eq2_e70_q);
        let eq2_e71_q_d_n4: f64 = (s.dn[31][4] * eq2_e70_q);
        let eq2_e71_q_d_n5: f64 = (s.dn[31][5] * eq2_e70_q);
        let eq2_e71_q_d_n6: f64 = ((s.dn[31][6] * eq2_e70_q) + s.v[31]);
        let eq2_e71_q_d_b0: f64 = (s.db[31][0] * eq2_e70_q);
        let eq2_e71_q_d_b1: f64 = (s.db[31][1] * eq2_e70_q);
        let eq2_e71_q_d_b2: f64 = (s.db[31][2] * eq2_e70_q);
        let eq2_e71_q_d_b3: f64 = (s.db[31][3] * eq2_e70_q);
        let eq2_e71_q_d_b4: f64 = (s.db[31][4] * eq2_e70_q);
        let eq2_e71_q_d_b5: f64 = (s.db[31][5] * eq2_e70_q);
        let eq2_e71_q_d_b6: f64 = (s.db[31][6] * eq2_e70_q);
        (eq2_e71, eq2_e71_d_n0, eq2_e71_d_n1, eq2_e71_d_n2, eq2_e71_d_n3, eq2_e71_d_n4, eq2_e71_d_n5, eq2_e71_d_n6, eq2_e71_d_b0, eq2_e71_d_b1, eq2_e71_d_b2, eq2_e71_d_b3, eq2_e71_d_b4, eq2_e71_d_b5, eq2_e71_d_b6, eq2_e71_q, eq2_e71_q_d_n0, eq2_e71_q_d_n1, eq2_e71_q_d_n2, eq2_e71_q_d_n3, eq2_e71_q_d_n4, eq2_e71_q_d_n5, eq2_e71_q_d_n6, eq2_e71_q_d_b0, eq2_e71_q_d_b1, eq2_e71_q_d_b2, eq2_e71_q_d_b3, eq2_e71_q_d_b4, eq2_e71_q_d_b5, eq2_e71_q_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_reactive_node_derivatives: [f64; 7] = [eq2_e73_q_d_n0, eq2_e73_q_d_n1, eq2_e73_q_d_n2, eq2_e73_q_d_n3, eq2_e73_q_d_n4, eq2_e73_q_d_n5, eq2_e73_q_d_n6];
        let eq2_reactive_branch_derivatives: [f64; 7] = [eq2_e73_q_d_b0, eq2_e73_q_d_b1, eq2_e73_q_d_b2, eq2_e73_q_d_b3, eq2_e73_q_d_b4, eq2_e73_q_d_b5, eq2_e73_q_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            &nodes,
            &eq2_reactive_node_derivatives,
            &branches,
            &eq2_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq6_e101, eq6_e101_d_n2, eq6_e101_q, eq6_e101_q_d_n2,) = {
    if (s.v[70] != 0.0) {
        let eq6_e98: f64 = ((nv2 - 0.0) * p.p34);
        let eq6_e98_d_n2: f64 = p.p34;
        let eq6_e99_q: f64 = eq6_e98;
        (eq6_e98, eq6_e98_d_n2, eq6_e99_q, eq6_e98_d_n2,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            None,
            &[
                GeneratedDerivative::node(nodes[2], self.multiplicity * (eq6_e101_q_d_n2)),
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq10_e137, eq10_e137_d_n2, eq10_e137_q, eq10_e137_q_d_n2,) = {
    if ((!(s.v[70] != 0.0)) && (s.v[71] != 0.0)) {
        let eq10_e134: f64 = (p.p34 * (nv2 - 0.0));
        let eq10_e134_d_n2: f64 = p.p34;
        let eq10_e135_q: f64 = eq10_e134;
        (eq10_e134, eq10_e134_d_n2, eq10_e135_q, eq10_e134_d_n2,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            None,
            &[
                GeneratedDerivative::node(nodes[2], self.multiplicity * (eq10_e137_q_d_n2)),
            ],
        );
    }
}
