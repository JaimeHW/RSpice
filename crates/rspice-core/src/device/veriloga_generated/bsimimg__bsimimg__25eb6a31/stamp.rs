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

        stamper.stamp_potential_branch(
            Some(nodes[0]),
            Some(nodes[5]),
            branches[0],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[2]),
            Some(nodes[6]),
            branches[1],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[7]),
            Some(nodes[8]),
            branches[2],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[1]),
            Some(nodes[7]),
            branches[3],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[4]),
            None,
            branches[4],
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
        self.stamp_transient_equation_15_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_16_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_17_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_18_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_19_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_20_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_21_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_22_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_23_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_24_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_25_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_26_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_27_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_28_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_29_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_30_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_31_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_32_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_33_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_34_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_35_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_36_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_37_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_38_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_39_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_40_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_41_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_42_block_0(ctx, stamper, &mut s);
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

        self.stamp_reactive_equation_12_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_13_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_14_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_15_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_16_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_17_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_18_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_41_block_0(ctx, stamper, &mut s);
    }
}
