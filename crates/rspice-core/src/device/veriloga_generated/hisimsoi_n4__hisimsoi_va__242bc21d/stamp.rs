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
#[path = "stamp_blocks_5.rs"]
mod stamp_blocks_5;
#[path = "stamp_blocks_6.rs"]
mod stamp_blocks_6;
#[path = "stamp_blocks_7.rs"]
mod stamp_blocks_7;
#[path = "stamp_blocks_8.rs"]
mod stamp_blocks_8;
#[path = "stamp_blocks_9.rs"]
mod stamp_blocks_9;

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
        self.stamp_transient_block_27(ctx, stamper, &mut s);
        self.stamp_transient_block_28(ctx, stamper, &mut s);
        self.stamp_transient_block_29(ctx, stamper, &mut s);
        self.stamp_transient_block_30(ctx, stamper, &mut s);
        self.stamp_transient_block_31(ctx, stamper, &mut s);

        stamper.stamp_potential_branch(
            Some(nodes[5]),
            None,
            branches[0],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[4]),
            None,
            branches[1],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[7]),
            Some(nodes[2]),
            branches[2],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[0]),
            Some(nodes[6]),
            branches[3],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[1]),
            Some(nodes[11]),
            branches[4],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[4]),
            Some(nodes[12]),
            branches[5],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[9]),
            Some(nodes[12]),
            branches[6],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[8]),
            Some(nodes[12]),
            branches[7],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[18]),
            None,
            branches[8],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[13]),
            None,
            branches[9],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[17]),
            None,
            branches[10],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[3]),
            Some(nodes[12]),
            branches[11],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[17]),
            None,
            branches[12],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[15]),
            None,
            branches[13],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[16]),
            None,
            branches[14],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[13]),
            None,
            branches[15],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[18]),
            None,
            branches[16],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[15]),
            None,
            branches[17],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[16]),
            None,
            branches[18],
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
        self.stamp_transient_equation_43_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_44_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_45_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_46_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_47_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_48_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_49_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_50_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_51_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_52_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_53_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_54_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_55_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_56_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_57_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_58_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_59_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_60_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_61_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_62_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_63_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_64_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_65_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_66_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_67_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_68_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_69_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_70_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_71_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_72_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_73_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_74_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_75_block_0(ctx, stamper, &mut s);
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
        self.stamp_reactive_block_27(ctx, stamper, &mut s);
        self.stamp_reactive_block_28(ctx, stamper, &mut s);
        self.stamp_reactive_block_29(ctx, stamper, &mut s);
        self.stamp_reactive_block_30(ctx, stamper, &mut s);

        self.stamp_reactive_equation_10_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_11_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_12_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_18_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_19_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_30_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_34_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_35_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_46_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_47_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_52_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_59_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_67_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_68_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_69_block_0(ctx, stamper, &mut s);
    }
}
