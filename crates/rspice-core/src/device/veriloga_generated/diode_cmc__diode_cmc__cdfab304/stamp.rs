#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

use crate::device::veriloga_generated::support::{AdValue as GenericAdValue, ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch};

type A = GenericAdValue<{ Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type Scratch = GenericScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type ReactiveScratch = GenericReactiveScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;

#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;
#[path = "stamp_blocks_1.rs"]
mod stamp_blocks_1;
#[path = "stamp_blocks_2.rs"]
mod stamp_blocks_2;
#[path = "stamp_blocks_3.rs"]
mod stamp_blocks_3;
#[path = "stamp_blocks_4.rs"]
mod stamp_blocks_4;

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
        self.stamp_transient_block_13(ctx, stamper, &mut s);
        self.stamp_transient_block_14(ctx, stamper, &mut s);
        self.stamp_transient_block_15(ctx, stamper, &mut s);
        self.stamp_transient_block_16(ctx, stamper, &mut s);
        self.stamp_transient_block_17(ctx, stamper, &mut s);
        self.stamp_transient_block_18(ctx, stamper, &mut s);
        self.stamp_transient_block_19(ctx, stamper, &mut s);
        self.stamp_transient_block_20(ctx, stamper, &mut s);
        self.stamp_transient_block_21(ctx, stamper, &mut s);
        self.stamp_transient_block_22(ctx, stamper, &mut s);
        self.stamp_transient_block_23(ctx, stamper, &mut s);
        self.stamp_transient_block_24(ctx, stamper, &mut s);
        self.stamp_transient_block_25(ctx, stamper, &mut s);
        self.stamp_transient_block_26(ctx, stamper, &mut s);

        stamper.stamp_potential_branch(
            Some(nodes[2]),
            Some(nodes[1]),
            branches[0],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[3]),
            None,
            branches[1],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[4]),
            None,
            branches[2],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[5]),
            None,
            branches[3],
            self.multiplicity,
        );

        self.stamp_transient_equation_0_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_1_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_2_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_3_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_4_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_5_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_6_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_7_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_8_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_9_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_10_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_11_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_12_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_13_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_14_block_0(ctx, stamper, &mut s);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut s = ReactiveScratch::new();

        self.stamp_reactive_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_block_1(ctx, stamper, &mut s);
        self.stamp_reactive_block_2(ctx, stamper, &mut s);
        self.stamp_reactive_block_3(ctx, stamper, &mut s);
        self.stamp_reactive_block_4(ctx, stamper, &mut s);
        self.stamp_reactive_block_5(ctx, stamper, &mut s);
        self.stamp_reactive_block_6(ctx, stamper, &mut s);
        self.stamp_reactive_block_7(ctx, stamper, &mut s);
        self.stamp_reactive_block_8(ctx, stamper, &mut s);
        self.stamp_reactive_block_9(ctx, stamper, &mut s);
        self.stamp_reactive_block_10(ctx, stamper, &mut s);
        self.stamp_reactive_block_11(ctx, stamper, &mut s);
        self.stamp_reactive_block_12(ctx, stamper, &mut s);
        self.stamp_reactive_block_13(ctx, stamper, &mut s);
        self.stamp_reactive_block_14(ctx, stamper, &mut s);
        self.stamp_reactive_block_15(ctx, stamper, &mut s);
        self.stamp_reactive_block_16(ctx, stamper, &mut s);
        self.stamp_reactive_block_17(ctx, stamper, &mut s);
        self.stamp_reactive_block_18(ctx, stamper, &mut s);
        self.stamp_reactive_block_19(ctx, stamper, &mut s);
        self.stamp_reactive_block_20(ctx, stamper, &mut s);
        self.stamp_reactive_block_21(ctx, stamper, &mut s);
        self.stamp_reactive_block_22(ctx, stamper, &mut s);
        self.stamp_reactive_block_23(ctx, stamper, &mut s);
        self.stamp_reactive_block_24(ctx, stamper, &mut s);
        self.stamp_reactive_block_25(ctx, stamper, &mut s);
        self.stamp_reactive_block_26(ctx, stamper, &mut s);

        self.stamp_reactive_equation_7_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_8_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_11_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_13_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_14_block_0(ctx, stamper, &mut s);
    }
}
