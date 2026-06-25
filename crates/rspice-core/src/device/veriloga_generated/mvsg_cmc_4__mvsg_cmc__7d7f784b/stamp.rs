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
#[path = "stamp_blocks_18.rs"]
mod stamp_blocks_18;
#[path = "stamp_blocks_19.rs"]
mod stamp_blocks_19;
#[path = "stamp_blocks_20.rs"]
mod stamp_blocks_20;
#[path = "stamp_blocks_21.rs"]
mod stamp_blocks_21;

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

        stamper.stamp_potential_branch(
            Some(nodes[22]),
            None,
            branches[0],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[23]),
            None,
            branches[1],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[24]),
            None,
            branches[2],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[25]),
            None,
            branches[3],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[26]),
            None,
            branches[4],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[27]),
            None,
            branches[5],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[21]),
            None,
            branches[6],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[20]),
            None,
            branches[7],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[22]),
            None,
            branches[8],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[25]),
            None,
            branches[9],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[21]),
            None,
            branches[10],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[20]),
            None,
            branches[11],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[22]),
            None,
            branches[12],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[23]),
            None,
            branches[13],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[24]),
            None,
            branches[14],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[25]),
            None,
            branches[15],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[26]),
            None,
            branches[16],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[27]),
            None,
            branches[17],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[17]),
            Some(nodes[16]),
            branches[18],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[16]),
            Some(nodes[15]),
            branches[19],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[15]),
            Some(nodes[14]),
            branches[20],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[14]),
            Some(nodes[5]),
            branches[21],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[9]),
            Some(nodes[10]),
            branches[22],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[10]),
            Some(nodes[11]),
            branches[23],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[11]),
            Some(nodes[12]),
            branches[24],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[12]),
            Some(nodes[13]),
            branches[25],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[13]),
            Some(nodes[19]),
            branches[26],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[18]),
            Some(nodes[17]),
            branches[27],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[28]),
            None,
            branches[28],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[29]),
            None,
            branches[29],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[7]),
            Some(nodes[8]),
            branches[30],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[0]),
            Some(nodes[18]),
            branches[31],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[19]),
            Some(nodes[2]),
            branches[32],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[1]),
            Some(nodes[6]),
            branches[33],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[6]),
            Some(nodes[7]),
            branches[34],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[4]),
            None,
            branches[35],
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
        self.stamp_transient_equation_76_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_77_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_78_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_79_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_80_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_81_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_82_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_83_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_84_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_85_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_86_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_87_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_88_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_89_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_90_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_91_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_92_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_93_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_94_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_95_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_96_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_97_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_98_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_99_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_100_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_101_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_102_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_103_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_104_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_105_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_106_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_107_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_108_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_109_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_110_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_111_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_112_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_113_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_114_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_115_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_116_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_117_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_118_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_119_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_120_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_121_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_122_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_123_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_124_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_125_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_126_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_127_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_128_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_129_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_130_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_131_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_132_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_133_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_134_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_135_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_136_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_137_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_138_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_139_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_140_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_141_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_142_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_143_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_144_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_145_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_146_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_147_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_148_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_149_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_150_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_151_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_152_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_153_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_154_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_155_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_156_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_157_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_158_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_159_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_160_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_161_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_162_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_163_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_164_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_165_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_166_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_167_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_168_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_169_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_170_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_171_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_172_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_173_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_174_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_175_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_176_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_177_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_178_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_179_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_180_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_181_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_182_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_183_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_184_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_185_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_186_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_187_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_188_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_189_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_190_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_191_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_192_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_193_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_194_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_195_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_196_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_197_block_0(ctx, stamper, &mut s);
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

        self.stamp_reactive_equation_8_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_9_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_17_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_22_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_33_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_34_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_35_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_37_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_38_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_39_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_40_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_43_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_46_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_47_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_48_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_50_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_51_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_52_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_53_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_56_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_59_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_60_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_61_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_63_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_64_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_65_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_66_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_69_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_72_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_73_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_74_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_76_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_77_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_78_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_79_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_82_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_85_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_86_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_87_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_89_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_90_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_91_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_92_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_95_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_98_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_99_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_100_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_102_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_103_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_104_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_105_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_108_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_111_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_112_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_113_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_115_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_116_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_117_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_118_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_121_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_124_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_125_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_126_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_128_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_129_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_130_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_131_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_134_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_142_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_143_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_145_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_146_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_157_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_172_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_173_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_174_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_175_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_176_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_177_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_194_block_0(ctx, stamper, &mut s);
    }
}
