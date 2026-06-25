#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

use crate::device::veriloga_generated::support::{AdValue as GenericAdValue, ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch};

type A = GenericAdValue<{ Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type Scratch = GenericScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type ReactiveScratch = GenericReactiveScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;

#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut s = Scratch::new();

        self.stamp_transient_block_0(ctx, stamper, &mut s);
        self.stamp_transient_block_1(ctx, stamper, &mut s);
        self.stamp_transient_block_2(ctx, stamper, &mut s);
        self.stamp_transient_block_3(ctx, stamper, &mut s);
        self.stamp_transient_block_4(ctx, stamper, &mut s);
        self.stamp_transient_block_5(ctx, stamper, &mut s);
        self.stamp_transient_block_6(ctx, stamper, &mut s);
        self.stamp_transient_block_7(ctx, stamper, &mut s);
        self.stamp_transient_block_8(ctx, stamper, &mut s);
        self.stamp_transient_block_9(ctx, stamper, &mut s);
        self.stamp_transient_block_10(ctx, stamper, &mut s);
        self.stamp_transient_block_11(ctx, stamper, &mut s);
        self.stamp_transient_block_12(ctx, stamper, &mut s);

        let eq0_e71: f64 = (p.p1 * s.v[0]);
        let eq0_e73: f64 = (eq0_e71 * p.p7);
        let eq0_e75: f64 = (eq0_e73 * s.v[544]);
        let eq0_e75_d_n0: f64 = (eq0_e73 * s.dn[544][0]);
        let eq0_e75_d_n1: f64 = (eq0_e73 * s.dn[544][1]);
        let eq0_value: f64 = eq0_e75;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[1]),
            self.multiplicity * (eq0_value),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * eq0_e75_d_n0),
                GeneratedDerivative::node(nodes[1], self.multiplicity * eq0_e75_d_n1),
            ],
        );
        let eq1_e78: f64 = (p.p1 * s.v[0]);
        let eq1_e80: f64 = (eq1_e78 * p.p8);
        let eq1_e82: f64 = (eq1_e80 * s.v[545]);
        let eq1_e82_d_n0: f64 = (eq1_e80 * s.dn[545][0]);
        let eq1_e82_d_n1: f64 = (eq1_e80 * s.dn[545][1]);
        let eq1_e83: f64 = self.eval_ddt(0, eq1_e82);
        let eq1_e83_d_n0: f64 = self.ddt_jacobian(eq1_e82_d_n0);
        let eq1_e83_d_n1: f64 = self.ddt_jacobian(eq1_e82_d_n1);
        let eq1_value: f64 = eq1_e83;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[1]),
            self.multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * eq1_e83_d_n0),
                GeneratedDerivative::node(nodes[1], self.multiplicity * eq1_e83_d_n1),
            ],
        );
        let eq2_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[1]),
            self.multiplicity * (eq2_value),
            &[
            ],
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut s = ReactiveScratch::new();

        self.stamp_reactive_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_block_1(ctx, stamper, &mut s);
        self.stamp_reactive_block_2(ctx, stamper, &mut s);

        let eq1_e78: f64 = (p.p1 * s.v[0]);
        let eq1_e80: f64 = (eq1_e78 * p.p8);
        let eq1_e82: f64 = (eq1_e80 * s.v[545]);
        let eq1_e82_d_n0: f64 = (eq1_e80 * s.dn[545][0]);
        let eq1_e82_d_n1: f64 = (eq1_e80 * s.dn[545][1]);
        let eq1_e83_q: f64 = eq1_e82;
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[1]),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * (eq1_e82_d_n0)),
                GeneratedDerivative::node(nodes[1], self.multiplicity * (eq1_e82_d_n1)),
            ],
        );
    }
}
