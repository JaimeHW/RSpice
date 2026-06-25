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
#[path = "stamp_blocks_10.rs"]
mod stamp_blocks_10;
#[path = "stamp_blocks_11.rs"]
mod stamp_blocks_11;
#[path = "stamp_blocks_12.rs"]
mod stamp_blocks_12;
#[path = "stamp_blocks_13.rs"]
mod stamp_blocks_13;
#[path = "stamp_blocks_14.rs"]
mod stamp_blocks_14;
#[path = "stamp_blocks_15.rs"]
mod stamp_blocks_15;
#[path = "stamp_blocks_16.rs"]
mod stamp_blocks_16;
#[path = "stamp_blocks_17.rs"]
mod stamp_blocks_17;

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
        self.stamp_transient_block_32(ctx, stamper, &mut s);
        self.stamp_transient_block_33(ctx, stamper, &mut s);
        self.stamp_transient_block_34(ctx, stamper, &mut s);
        self.stamp_transient_block_35(ctx, stamper, &mut s);
        self.stamp_transient_block_36(ctx, stamper, &mut s);
        self.stamp_transient_block_37(ctx, stamper, &mut s);
        self.stamp_transient_block_38(ctx, stamper, &mut s);
        self.stamp_transient_block_39(ctx, stamper, &mut s);
        self.stamp_transient_block_40(ctx, stamper, &mut s);
        self.stamp_transient_block_41(ctx, stamper, &mut s);
        self.stamp_transient_block_42(ctx, stamper, &mut s);
        self.stamp_transient_block_43(ctx, stamper, &mut s);
        self.stamp_transient_block_44(ctx, stamper, &mut s);
        self.stamp_transient_block_45(ctx, stamper, &mut s);
        self.stamp_transient_block_46(ctx, stamper, &mut s);
        self.stamp_transient_block_47(ctx, stamper, &mut s);
        self.stamp_transient_block_48(ctx, stamper, &mut s);
        self.stamp_transient_block_49(ctx, stamper, &mut s);
        self.stamp_transient_block_50(ctx, stamper, &mut s);
        self.stamp_transient_block_51(ctx, stamper, &mut s);
        self.stamp_transient_block_52(ctx, stamper, &mut s);
        self.stamp_transient_block_53(ctx, stamper, &mut s);
        self.stamp_transient_block_54(ctx, stamper, &mut s);
        self.stamp_transient_block_55(ctx, stamper, &mut s);
        self.stamp_transient_block_56(ctx, stamper, &mut s);
        self.stamp_transient_block_57(ctx, stamper, &mut s);
        self.stamp_transient_block_58(ctx, stamper, &mut s);
        self.stamp_transient_block_59(ctx, stamper, &mut s);
        self.stamp_transient_block_60(ctx, stamper, &mut s);
        self.stamp_transient_block_61(ctx, stamper, &mut s);
        self.stamp_transient_block_62(ctx, stamper, &mut s);
        self.stamp_transient_block_63(ctx, stamper, &mut s);
        self.stamp_transient_block_64(ctx, stamper, &mut s);
        self.stamp_transient_block_65(ctx, stamper, &mut s);
        self.stamp_transient_block_66(ctx, stamper, &mut s);
        self.stamp_transient_block_67(ctx, stamper, &mut s);
        self.stamp_transient_block_68(ctx, stamper, &mut s);
        self.stamp_transient_block_69(ctx, stamper, &mut s);
        self.stamp_transient_block_70(ctx, stamper, &mut s);
        self.stamp_transient_block_71(ctx, stamper, &mut s);
        self.stamp_transient_block_72(ctx, stamper, &mut s);
        self.stamp_transient_block_73(ctx, stamper, &mut s);
        self.stamp_transient_block_74(ctx, stamper, &mut s);
        self.stamp_transient_block_75(ctx, stamper, &mut s);
        self.stamp_transient_block_76(ctx, stamper, &mut s);
        self.stamp_transient_block_77(ctx, stamper, &mut s);
        self.stamp_transient_block_78(ctx, stamper, &mut s);
        self.stamp_transient_block_79(ctx, stamper, &mut s);
        self.stamp_transient_block_80(ctx, stamper, &mut s);
        self.stamp_transient_block_81(ctx, stamper, &mut s);
        self.stamp_transient_block_82(ctx, stamper, &mut s);
        self.stamp_transient_block_83(ctx, stamper, &mut s);
        self.stamp_transient_block_84(ctx, stamper, &mut s);
        self.stamp_transient_block_85(ctx, stamper, &mut s);
        self.stamp_transient_block_86(ctx, stamper, &mut s);
        self.stamp_transient_block_87(ctx, stamper, &mut s);
        self.stamp_transient_block_88(ctx, stamper, &mut s);
        self.stamp_transient_block_89(ctx, stamper, &mut s);
        self.stamp_transient_block_90(ctx, stamper, &mut s);
        self.stamp_transient_block_91(ctx, stamper, &mut s);
        self.stamp_transient_block_92(ctx, stamper, &mut s);
        self.stamp_transient_block_93(ctx, stamper, &mut s);
        self.stamp_transient_block_94(ctx, stamper, &mut s);
        self.stamp_transient_block_95(ctx, stamper, &mut s);

        stamper.stamp_potential_branch(
            Some(nodes[4]),
            Some(nodes[5]),
            branches[0],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[16]),
            None,
            branches[1],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[17]),
            None,
            branches[2],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[18]),
            None,
            branches[3],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[0]),
            Some(nodes[6]),
            branches[4],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[8]),
            Some(nodes[2]),
            branches[5],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[1]),
            Some(nodes[7]),
            branches[6],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[3]),
            Some(nodes[9]),
            branches[7],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[11]),
            Some(nodes[9]),
            branches[8],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[10]),
            Some(nodes[9]),
            branches[9],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[12]),
            None,
            branches[10],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[13]),
            None,
            branches[11],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[14]),
            None,
            branches[12],
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
        self.stamp_reactive_block_31(ctx, stamper, &mut s);
        self.stamp_reactive_block_32(ctx, stamper, &mut s);
        self.stamp_reactive_block_33(ctx, stamper, &mut s);
        self.stamp_reactive_block_34(ctx, stamper, &mut s);
        self.stamp_reactive_block_35(ctx, stamper, &mut s);
        self.stamp_reactive_block_36(ctx, stamper, &mut s);
        self.stamp_reactive_block_37(ctx, stamper, &mut s);
        self.stamp_reactive_block_38(ctx, stamper, &mut s);
        self.stamp_reactive_block_39(ctx, stamper, &mut s);
        self.stamp_reactive_block_40(ctx, stamper, &mut s);
        self.stamp_reactive_block_41(ctx, stamper, &mut s);
        self.stamp_reactive_block_42(ctx, stamper, &mut s);
        self.stamp_reactive_block_43(ctx, stamper, &mut s);
        self.stamp_reactive_block_44(ctx, stamper, &mut s);
        self.stamp_reactive_block_45(ctx, stamper, &mut s);
        self.stamp_reactive_block_46(ctx, stamper, &mut s);
        self.stamp_reactive_block_47(ctx, stamper, &mut s);
        self.stamp_reactive_block_48(ctx, stamper, &mut s);
        self.stamp_reactive_block_49(ctx, stamper, &mut s);
        self.stamp_reactive_block_50(ctx, stamper, &mut s);
        self.stamp_reactive_block_51(ctx, stamper, &mut s);
        self.stamp_reactive_block_52(ctx, stamper, &mut s);
        self.stamp_reactive_block_53(ctx, stamper, &mut s);
        self.stamp_reactive_block_54(ctx, stamper, &mut s);
        self.stamp_reactive_block_55(ctx, stamper, &mut s);
        self.stamp_reactive_block_56(ctx, stamper, &mut s);
        self.stamp_reactive_block_57(ctx, stamper, &mut s);
        self.stamp_reactive_block_58(ctx, stamper, &mut s);
        self.stamp_reactive_block_59(ctx, stamper, &mut s);
        self.stamp_reactive_block_60(ctx, stamper, &mut s);
        self.stamp_reactive_block_61(ctx, stamper, &mut s);
        self.stamp_reactive_block_62(ctx, stamper, &mut s);
        self.stamp_reactive_block_63(ctx, stamper, &mut s);
        self.stamp_reactive_block_64(ctx, stamper, &mut s);
        self.stamp_reactive_block_65(ctx, stamper, &mut s);
        self.stamp_reactive_block_66(ctx, stamper, &mut s);
        self.stamp_reactive_block_67(ctx, stamper, &mut s);
        self.stamp_reactive_block_68(ctx, stamper, &mut s);
        self.stamp_reactive_block_69(ctx, stamper, &mut s);
        self.stamp_reactive_block_70(ctx, stamper, &mut s);
        self.stamp_reactive_block_71(ctx, stamper, &mut s);
        self.stamp_reactive_block_72(ctx, stamper, &mut s);
        self.stamp_reactive_block_73(ctx, stamper, &mut s);
        self.stamp_reactive_block_74(ctx, stamper, &mut s);
        self.stamp_reactive_block_75(ctx, stamper, &mut s);
        self.stamp_reactive_block_76(ctx, stamper, &mut s);
        self.stamp_reactive_block_77(ctx, stamper, &mut s);
        self.stamp_reactive_block_78(ctx, stamper, &mut s);
        self.stamp_reactive_block_79(ctx, stamper, &mut s);
        self.stamp_reactive_block_80(ctx, stamper, &mut s);
        self.stamp_reactive_block_81(ctx, stamper, &mut s);
        self.stamp_reactive_block_82(ctx, stamper, &mut s);
        self.stamp_reactive_block_83(ctx, stamper, &mut s);
        self.stamp_reactive_block_84(ctx, stamper, &mut s);
        self.stamp_reactive_block_85(ctx, stamper, &mut s);
        self.stamp_reactive_block_86(ctx, stamper, &mut s);
        self.stamp_reactive_block_87(ctx, stamper, &mut s);
        self.stamp_reactive_block_88(ctx, stamper, &mut s);
        self.stamp_reactive_block_89(ctx, stamper, &mut s);
        self.stamp_reactive_block_90(ctx, stamper, &mut s);
        self.stamp_reactive_block_91(ctx, stamper, &mut s);
        self.stamp_reactive_block_92(ctx, stamper, &mut s);
        self.stamp_reactive_block_93(ctx, stamper, &mut s);

        self.stamp_reactive_equation_1_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_2_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_5_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_15_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_16_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_19_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_20_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_28_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_29_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_30_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_31_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_32_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_33_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_34_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_35_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_41_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_42_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_59_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_62_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_63_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_67_block_0(ctx, stamper, &mut s);
    }
}
