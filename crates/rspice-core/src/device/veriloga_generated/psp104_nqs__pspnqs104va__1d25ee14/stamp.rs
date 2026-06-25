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

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let bi7 = ctx.branch_current(branches[7]);
        let bi9 = ctx.branch_current(branches[9]);
        let bi11 = ctx.branch_current(branches[11]);
        let bi13 = ctx.branch_current(branches[13]);
        let bi15 = ctx.branch_current(branches[15]);
        let bi17 = ctx.branch_current(branches[17]);
        let bi19 = ctx.branch_current(branches[19]);
        let bi21 = ctx.branch_current(branches[21]);
        let bi23 = ctx.branch_current(branches[23]);
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

        stamper.stamp_potential_branch(
            Some(nodes[1]),
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
            Some(nodes[0]),
            Some(nodes[7]),
            branches[2],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[8]),
            Some(nodes[9]),
            branches[3],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[10]),
            Some(nodes[9]),
            branches[4],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[11]),
            Some(nodes[9]),
            branches[5],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[3]),
            Some(nodes[9]),
            branches[6],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[12]),
            None,
            branches[7],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[12]),
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
            Some(nodes[13]),
            None,
            branches[10],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[14]),
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
        stamper.stamp_potential_branch(
            Some(nodes[15]),
            None,
            branches[13],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[15]),
            None,
            branches[14],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[16]),
            None,
            branches[15],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[16]),
            None,
            branches[16],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[17]),
            None,
            branches[17],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[17]),
            None,
            branches[18],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[18]),
            None,
            branches[19],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[18]),
            None,
            branches[20],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[19]),
            None,
            branches[21],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[19]),
            None,
            branches[22],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[20]),
            None,
            branches[23],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[20]),
            None,
            branches[24],
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
        let eq38_e1270: f64 = (s.v[4] * bi7);
        let eq38_e1270_d_b7: f64 = s.v[4];
        let eq38_e1272: f64 = (eq38_e1270 * s.v[2]);
        let eq38_e1272_d_b7: f64 = (eq38_e1270_d_b7 * s.v[2]);
        let eq38_value: f64 = eq38_e1272;
        stamper.stamp_potential(
            branches[7],
            eq38_value,
            &[
                GeneratedDerivative::branch(branches[7], eq38_e1272_d_b7),
            ],
        );
        self.stamp_transient_equation_39_block_0(ctx, stamper, &mut s);
        let eq40_e1283: f64 = (s.v[4] * bi9);
        let eq40_e1283_d_b9: f64 = s.v[4];
        let eq40_e1285: f64 = (eq40_e1283 * s.v[2]);
        let eq40_e1285_d_b9: f64 = (eq40_e1283_d_b9 * s.v[2]);
        let eq40_value: f64 = eq40_e1285;
        stamper.stamp_potential(
            branches[9],
            eq40_value,
            &[
                GeneratedDerivative::branch(branches[9], eq40_e1285_d_b9),
            ],
        );
        self.stamp_transient_equation_41_block_0(ctx, stamper, &mut s);
        let eq42_e1296: f64 = (s.v[4] * bi11);
        let eq42_e1296_d_b11: f64 = s.v[4];
        let eq42_e1298: f64 = (eq42_e1296 * s.v[2]);
        let eq42_e1298_d_b11: f64 = (eq42_e1296_d_b11 * s.v[2]);
        let eq42_value: f64 = eq42_e1298;
        stamper.stamp_potential(
            branches[11],
            eq42_value,
            &[
                GeneratedDerivative::branch(branches[11], eq42_e1298_d_b11),
            ],
        );
        self.stamp_transient_equation_43_block_0(ctx, stamper, &mut s);
        let eq44_e1309: f64 = (s.v[4] * bi13);
        let eq44_e1309_d_b13: f64 = s.v[4];
        let eq44_e1311: f64 = (eq44_e1309 * s.v[2]);
        let eq44_e1311_d_b13: f64 = (eq44_e1309_d_b13 * s.v[2]);
        let eq44_value: f64 = eq44_e1311;
        stamper.stamp_potential(
            branches[13],
            eq44_value,
            &[
                GeneratedDerivative::branch(branches[13], eq44_e1311_d_b13),
            ],
        );
        self.stamp_transient_equation_45_block_0(ctx, stamper, &mut s);
        let eq46_e1322: f64 = (s.v[4] * bi15);
        let eq46_e1322_d_b15: f64 = s.v[4];
        let eq46_e1324: f64 = (eq46_e1322 * s.v[2]);
        let eq46_e1324_d_b15: f64 = (eq46_e1322_d_b15 * s.v[2]);
        let eq46_value: f64 = eq46_e1324;
        stamper.stamp_potential(
            branches[15],
            eq46_value,
            &[
                GeneratedDerivative::branch(branches[15], eq46_e1324_d_b15),
            ],
        );
        self.stamp_transient_equation_47_block_0(ctx, stamper, &mut s);
        let eq48_e1335: f64 = (s.v[4] * bi17);
        let eq48_e1335_d_b17: f64 = s.v[4];
        let eq48_e1337: f64 = (eq48_e1335 * s.v[2]);
        let eq48_e1337_d_b17: f64 = (eq48_e1335_d_b17 * s.v[2]);
        let eq48_value: f64 = eq48_e1337;
        stamper.stamp_potential(
            branches[17],
            eq48_value,
            &[
                GeneratedDerivative::branch(branches[17], eq48_e1337_d_b17),
            ],
        );
        self.stamp_transient_equation_49_block_0(ctx, stamper, &mut s);
        let eq50_e1348: f64 = (s.v[4] * bi19);
        let eq50_e1348_d_b19: f64 = s.v[4];
        let eq50_e1350: f64 = (eq50_e1348 * s.v[2]);
        let eq50_e1350_d_b19: f64 = (eq50_e1348_d_b19 * s.v[2]);
        let eq50_value: f64 = eq50_e1350;
        stamper.stamp_potential(
            branches[19],
            eq50_value,
            &[
                GeneratedDerivative::branch(branches[19], eq50_e1350_d_b19),
            ],
        );
        self.stamp_transient_equation_51_block_0(ctx, stamper, &mut s);
        let eq52_e1361: f64 = (s.v[4] * bi21);
        let eq52_e1361_d_b21: f64 = s.v[4];
        let eq52_e1363: f64 = (eq52_e1361 * s.v[2]);
        let eq52_e1363_d_b21: f64 = (eq52_e1361_d_b21 * s.v[2]);
        let eq52_value: f64 = eq52_e1363;
        stamper.stamp_potential(
            branches[21],
            eq52_value,
            &[
                GeneratedDerivative::branch(branches[21], eq52_e1363_d_b21),
            ],
        );
        self.stamp_transient_equation_53_block_0(ctx, stamper, &mut s);
        let eq54_e1374: f64 = (s.v[4] * bi23);
        let eq54_e1374_d_b23: f64 = s.v[4];
        let eq54_e1376: f64 = (eq54_e1374 * s.v[2]);
        let eq54_e1376_d_b23: f64 = (eq54_e1374_d_b23 * s.v[2]);
        let eq54_value: f64 = eq54_e1376;
        stamper.stamp_potential(
            branches[23],
            eq54_value,
            &[
                GeneratedDerivative::branch(branches[23], eq54_e1376_d_b23),
            ],
        );
        self.stamp_transient_equation_55_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_56_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_57_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_58_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_59_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_60_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_61_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_62_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_63_block_0(ctx, stamper, &mut s);
        let eq64_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq64_value),
            &[
            ],
        );
        self.stamp_transient_equation_65_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_66_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_67_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_68_block_0(ctx, stamper, &mut s);
        let eq69_e1488: f64 = (s.v[19] * p.p32);
        let eq69_e1488_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq69_e1488_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq69_e1488_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq69_e1488_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq69_e1488_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq69_e1488_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq69_e1488_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq69_e1488_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq69_e1488_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq69_e1488_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq69_e1488_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq69_e1488_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq69_e1488_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq69_e1488_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq69_e1488_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq69_e1488_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq69_e1488_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq69_e1488_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq69_e1488_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq69_e1488_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq69_e1488_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq69_e1488_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq69_e1488_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq69_e1488_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq69_e1488_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq69_e1488_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq69_e1488_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq69_e1488_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq69_e1488_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq69_e1488_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq69_e1488_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq69_e1488_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq69_e1488_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq69_e1488_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq69_e1488_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq69_e1488_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq69_e1488_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq69_e1488_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq69_e1488_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq69_e1488_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq69_e1488_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq69_e1488_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq69_e1488_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq69_e1488_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq69_e1488_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq69_e1488_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq69_e1489: f64 = (eq69_e1488).sqrt();
        let eq69_e1489_d_n0: f64 = (eq69_e1488_d_n0 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n1: f64 = (eq69_e1488_d_n1 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n2: f64 = (eq69_e1488_d_n2 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n3: f64 = (eq69_e1488_d_n3 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n4: f64 = (eq69_e1488_d_n4 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n5: f64 = (eq69_e1488_d_n5 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n6: f64 = (eq69_e1488_d_n6 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n7: f64 = (eq69_e1488_d_n7 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n8: f64 = (eq69_e1488_d_n8 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n9: f64 = (eq69_e1488_d_n9 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n10: f64 = (eq69_e1488_d_n10 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n11: f64 = (eq69_e1488_d_n11 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n12: f64 = (eq69_e1488_d_n12 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n13: f64 = (eq69_e1488_d_n13 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n14: f64 = (eq69_e1488_d_n14 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n15: f64 = (eq69_e1488_d_n15 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n16: f64 = (eq69_e1488_d_n16 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n17: f64 = (eq69_e1488_d_n17 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n18: f64 = (eq69_e1488_d_n18 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n19: f64 = (eq69_e1488_d_n19 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n20: f64 = (eq69_e1488_d_n20 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b0: f64 = (eq69_e1488_d_b0 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b1: f64 = (eq69_e1488_d_b1 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b2: f64 = (eq69_e1488_d_b2 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b3: f64 = (eq69_e1488_d_b3 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b4: f64 = (eq69_e1488_d_b4 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b5: f64 = (eq69_e1488_d_b5 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b6: f64 = (eq69_e1488_d_b6 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b7: f64 = (eq69_e1488_d_b7 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b8: f64 = (eq69_e1488_d_b8 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b9: f64 = (eq69_e1488_d_b9 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b10: f64 = (eq69_e1488_d_b10 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b11: f64 = (eq69_e1488_d_b11 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b12: f64 = (eq69_e1488_d_b12 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b13: f64 = (eq69_e1488_d_b13 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b14: f64 = (eq69_e1488_d_b14 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b15: f64 = (eq69_e1488_d_b15 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b16: f64 = (eq69_e1488_d_b16 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b17: f64 = (eq69_e1488_d_b17 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b18: f64 = (eq69_e1488_d_b18 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b19: f64 = (eq69_e1488_d_b19 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b20: f64 = (eq69_e1488_d_b20 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b21: f64 = (eq69_e1488_d_b21 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b22: f64 = (eq69_e1488_d_b22 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b23: f64 = (eq69_e1488_d_b23 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b24: f64 = (eq69_e1488_d_b24 / (2.0 * eq69_e1489));
        let eq69_e1490: f64 = (s.v[831] * eq69_e1489);
        let eq69_e1490_d_n0: f64 = ((s.dn[831][0] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n0));
        let eq69_e1490_d_n1: f64 = ((s.dn[831][1] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n1));
        let eq69_e1490_d_n2: f64 = ((s.dn[831][2] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n2));
        let eq69_e1490_d_n3: f64 = ((s.dn[831][3] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n3));
        let eq69_e1490_d_n4: f64 = ((s.dn[831][4] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n4));
        let eq69_e1490_d_n5: f64 = ((s.dn[831][5] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n5));
        let eq69_e1490_d_n6: f64 = ((s.dn[831][6] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n6));
        let eq69_e1490_d_n7: f64 = ((s.dn[831][7] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n7));
        let eq69_e1490_d_n8: f64 = ((s.dn[831][8] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n8));
        let eq69_e1490_d_n9: f64 = ((s.dn[831][9] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n9));
        let eq69_e1490_d_n10: f64 = ((s.dn[831][10] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n10));
        let eq69_e1490_d_n11: f64 = ((s.dn[831][11] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n11));
        let eq69_e1490_d_n12: f64 = ((s.dn[831][12] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n12));
        let eq69_e1490_d_n13: f64 = ((s.dn[831][13] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n13));
        let eq69_e1490_d_n14: f64 = ((s.dn[831][14] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n14));
        let eq69_e1490_d_n15: f64 = ((s.dn[831][15] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n15));
        let eq69_e1490_d_n16: f64 = ((s.dn[831][16] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n16));
        let eq69_e1490_d_n17: f64 = ((s.dn[831][17] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n17));
        let eq69_e1490_d_n18: f64 = ((s.dn[831][18] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n18));
        let eq69_e1490_d_n19: f64 = ((s.dn[831][19] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n19));
        let eq69_e1490_d_n20: f64 = ((s.dn[831][20] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n20));
        let eq69_e1490_d_b0: f64 = ((s.db[831][0] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b0));
        let eq69_e1490_d_b1: f64 = ((s.db[831][1] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b1));
        let eq69_e1490_d_b2: f64 = ((s.db[831][2] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b2));
        let eq69_e1490_d_b3: f64 = ((s.db[831][3] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b3));
        let eq69_e1490_d_b4: f64 = ((s.db[831][4] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b4));
        let eq69_e1490_d_b5: f64 = ((s.db[831][5] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b5));
        let eq69_e1490_d_b6: f64 = ((s.db[831][6] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b6));
        let eq69_e1490_d_b7: f64 = ((s.db[831][7] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b7));
        let eq69_e1490_d_b8: f64 = ((s.db[831][8] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b8));
        let eq69_e1490_d_b9: f64 = ((s.db[831][9] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b9));
        let eq69_e1490_d_b10: f64 = ((s.db[831][10] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b10));
        let eq69_e1490_d_b11: f64 = ((s.db[831][11] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b11));
        let eq69_e1490_d_b12: f64 = ((s.db[831][12] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b12));
        let eq69_e1490_d_b13: f64 = ((s.db[831][13] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b13));
        let eq69_e1490_d_b14: f64 = ((s.db[831][14] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b14));
        let eq69_e1490_d_b15: f64 = ((s.db[831][15] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b15));
        let eq69_e1490_d_b16: f64 = ((s.db[831][16] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b16));
        let eq69_e1490_d_b17: f64 = ((s.db[831][17] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b17));
        let eq69_e1490_d_b18: f64 = ((s.db[831][18] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b18));
        let eq69_e1490_d_b19: f64 = ((s.db[831][19] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b19));
        let eq69_e1490_d_b20: f64 = ((s.db[831][20] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b20));
        let eq69_e1490_d_b21: f64 = ((s.db[831][21] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b21));
        let eq69_e1490_d_b22: f64 = ((s.db[831][22] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b22));
        let eq69_e1490_d_b23: f64 = ((s.db[831][23] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b23));
        let eq69_e1490_d_b24: f64 = ((s.db[831][24] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b24));
        let eq69_e1492: f64 = (eq69_e1490 * s.v[861]);
        let eq69_e1492_d_n0: f64 = ((eq69_e1490_d_n0 * s.v[861]) + (eq69_e1490 * s.dn[861][0]));
        let eq69_e1492_d_n1: f64 = ((eq69_e1490_d_n1 * s.v[861]) + (eq69_e1490 * s.dn[861][1]));
        let eq69_e1492_d_n2: f64 = ((eq69_e1490_d_n2 * s.v[861]) + (eq69_e1490 * s.dn[861][2]));
        let eq69_e1492_d_n3: f64 = ((eq69_e1490_d_n3 * s.v[861]) + (eq69_e1490 * s.dn[861][3]));
        let eq69_e1492_d_n4: f64 = ((eq69_e1490_d_n4 * s.v[861]) + (eq69_e1490 * s.dn[861][4]));
        let eq69_e1492_d_n5: f64 = ((eq69_e1490_d_n5 * s.v[861]) + (eq69_e1490 * s.dn[861][5]));
        let eq69_e1492_d_n6: f64 = ((eq69_e1490_d_n6 * s.v[861]) + (eq69_e1490 * s.dn[861][6]));
        let eq69_e1492_d_n7: f64 = ((eq69_e1490_d_n7 * s.v[861]) + (eq69_e1490 * s.dn[861][7]));
        let eq69_e1492_d_n8: f64 = ((eq69_e1490_d_n8 * s.v[861]) + (eq69_e1490 * s.dn[861][8]));
        let eq69_e1492_d_n9: f64 = ((eq69_e1490_d_n9 * s.v[861]) + (eq69_e1490 * s.dn[861][9]));
        let eq69_e1492_d_n10: f64 = ((eq69_e1490_d_n10 * s.v[861]) + (eq69_e1490 * s.dn[861][10]));
        let eq69_e1492_d_n11: f64 = ((eq69_e1490_d_n11 * s.v[861]) + (eq69_e1490 * s.dn[861][11]));
        let eq69_e1492_d_n12: f64 = ((eq69_e1490_d_n12 * s.v[861]) + (eq69_e1490 * s.dn[861][12]));
        let eq69_e1492_d_n13: f64 = ((eq69_e1490_d_n13 * s.v[861]) + (eq69_e1490 * s.dn[861][13]));
        let eq69_e1492_d_n14: f64 = ((eq69_e1490_d_n14 * s.v[861]) + (eq69_e1490 * s.dn[861][14]));
        let eq69_e1492_d_n15: f64 = ((eq69_e1490_d_n15 * s.v[861]) + (eq69_e1490 * s.dn[861][15]));
        let eq69_e1492_d_n16: f64 = ((eq69_e1490_d_n16 * s.v[861]) + (eq69_e1490 * s.dn[861][16]));
        let eq69_e1492_d_n17: f64 = ((eq69_e1490_d_n17 * s.v[861]) + (eq69_e1490 * s.dn[861][17]));
        let eq69_e1492_d_n18: f64 = ((eq69_e1490_d_n18 * s.v[861]) + (eq69_e1490 * s.dn[861][18]));
        let eq69_e1492_d_n19: f64 = ((eq69_e1490_d_n19 * s.v[861]) + (eq69_e1490 * s.dn[861][19]));
        let eq69_e1492_d_n20: f64 = ((eq69_e1490_d_n20 * s.v[861]) + (eq69_e1490 * s.dn[861][20]));
        let eq69_e1492_d_b0: f64 = ((eq69_e1490_d_b0 * s.v[861]) + (eq69_e1490 * s.db[861][0]));
        let eq69_e1492_d_b1: f64 = ((eq69_e1490_d_b1 * s.v[861]) + (eq69_e1490 * s.db[861][1]));
        let eq69_e1492_d_b2: f64 = ((eq69_e1490_d_b2 * s.v[861]) + (eq69_e1490 * s.db[861][2]));
        let eq69_e1492_d_b3: f64 = ((eq69_e1490_d_b3 * s.v[861]) + (eq69_e1490 * s.db[861][3]));
        let eq69_e1492_d_b4: f64 = ((eq69_e1490_d_b4 * s.v[861]) + (eq69_e1490 * s.db[861][4]));
        let eq69_e1492_d_b5: f64 = ((eq69_e1490_d_b5 * s.v[861]) + (eq69_e1490 * s.db[861][5]));
        let eq69_e1492_d_b6: f64 = ((eq69_e1490_d_b6 * s.v[861]) + (eq69_e1490 * s.db[861][6]));
        let eq69_e1492_d_b7: f64 = ((eq69_e1490_d_b7 * s.v[861]) + (eq69_e1490 * s.db[861][7]));
        let eq69_e1492_d_b8: f64 = ((eq69_e1490_d_b8 * s.v[861]) + (eq69_e1490 * s.db[861][8]));
        let eq69_e1492_d_b9: f64 = ((eq69_e1490_d_b9 * s.v[861]) + (eq69_e1490 * s.db[861][9]));
        let eq69_e1492_d_b10: f64 = ((eq69_e1490_d_b10 * s.v[861]) + (eq69_e1490 * s.db[861][10]));
        let eq69_e1492_d_b11: f64 = ((eq69_e1490_d_b11 * s.v[861]) + (eq69_e1490 * s.db[861][11]));
        let eq69_e1492_d_b12: f64 = ((eq69_e1490_d_b12 * s.v[861]) + (eq69_e1490 * s.db[861][12]));
        let eq69_e1492_d_b13: f64 = ((eq69_e1490_d_b13 * s.v[861]) + (eq69_e1490 * s.db[861][13]));
        let eq69_e1492_d_b14: f64 = ((eq69_e1490_d_b14 * s.v[861]) + (eq69_e1490 * s.db[861][14]));
        let eq69_e1492_d_b15: f64 = ((eq69_e1490_d_b15 * s.v[861]) + (eq69_e1490 * s.db[861][15]));
        let eq69_e1492_d_b16: f64 = ((eq69_e1490_d_b16 * s.v[861]) + (eq69_e1490 * s.db[861][16]));
        let eq69_e1492_d_b17: f64 = ((eq69_e1490_d_b17 * s.v[861]) + (eq69_e1490 * s.db[861][17]));
        let eq69_e1492_d_b18: f64 = ((eq69_e1490_d_b18 * s.v[861]) + (eq69_e1490 * s.db[861][18]));
        let eq69_e1492_d_b19: f64 = ((eq69_e1490_d_b19 * s.v[861]) + (eq69_e1490 * s.db[861][19]));
        let eq69_e1492_d_b20: f64 = ((eq69_e1490_d_b20 * s.v[861]) + (eq69_e1490 * s.db[861][20]));
        let eq69_e1492_d_b21: f64 = ((eq69_e1490_d_b21 * s.v[861]) + (eq69_e1490 * s.db[861][21]));
        let eq69_e1492_d_b22: f64 = ((eq69_e1490_d_b22 * s.v[861]) + (eq69_e1490 * s.db[861][22]));
        let eq69_e1492_d_b23: f64 = ((eq69_e1490_d_b23 * s.v[861]) + (eq69_e1490 * s.db[861][23]));
        let eq69_e1492_d_b24: f64 = ((eq69_e1490_d_b24 * s.v[861]) + (eq69_e1490 * s.db[861][24]));
        let eq69_e1494: f64 = (eq69_e1492 * eq64_value);
        let eq69_e1494_d_n0: f64 = (eq69_e1492_d_n0 * eq64_value);
        let eq69_e1494_d_n1: f64 = (eq69_e1492_d_n1 * eq64_value);
        let eq69_e1494_d_n2: f64 = (eq69_e1492_d_n2 * eq64_value);
        let eq69_e1494_d_n3: f64 = (eq69_e1492_d_n3 * eq64_value);
        let eq69_e1494_d_n4: f64 = (eq69_e1492_d_n4 * eq64_value);
        let eq69_e1494_d_n5: f64 = (eq69_e1492_d_n5 * eq64_value);
        let eq69_e1494_d_n6: f64 = (eq69_e1492_d_n6 * eq64_value);
        let eq69_e1494_d_n7: f64 = (eq69_e1492_d_n7 * eq64_value);
        let eq69_e1494_d_n8: f64 = (eq69_e1492_d_n8 * eq64_value);
        let eq69_e1494_d_n9: f64 = (eq69_e1492_d_n9 * eq64_value);
        let eq69_e1494_d_n10: f64 = (eq69_e1492_d_n10 * eq64_value);
        let eq69_e1494_d_n11: f64 = (eq69_e1492_d_n11 * eq64_value);
        let eq69_e1494_d_n12: f64 = (eq69_e1492_d_n12 * eq64_value);
        let eq69_e1494_d_n13: f64 = (eq69_e1492_d_n13 * eq64_value);
        let eq69_e1494_d_n14: f64 = (eq69_e1492_d_n14 * eq64_value);
        let eq69_e1494_d_n15: f64 = (eq69_e1492_d_n15 * eq64_value);
        let eq69_e1494_d_n16: f64 = (eq69_e1492_d_n16 * eq64_value);
        let eq69_e1494_d_n17: f64 = (eq69_e1492_d_n17 * eq64_value);
        let eq69_e1494_d_n18: f64 = (eq69_e1492_d_n18 * eq64_value);
        let eq69_e1494_d_n19: f64 = (eq69_e1492_d_n19 * eq64_value);
        let eq69_e1494_d_n20: f64 = (eq69_e1492_d_n20 * eq64_value);
        let eq69_e1494_d_b0: f64 = (eq69_e1492_d_b0 * eq64_value);
        let eq69_e1494_d_b1: f64 = (eq69_e1492_d_b1 * eq64_value);
        let eq69_e1494_d_b2: f64 = (eq69_e1492_d_b2 * eq64_value);
        let eq69_e1494_d_b3: f64 = (eq69_e1492_d_b3 * eq64_value);
        let eq69_e1494_d_b4: f64 = (eq69_e1492_d_b4 * eq64_value);
        let eq69_e1494_d_b5: f64 = (eq69_e1492_d_b5 * eq64_value);
        let eq69_e1494_d_b6: f64 = (eq69_e1492_d_b6 * eq64_value);
        let eq69_e1494_d_b7: f64 = (eq69_e1492_d_b7 * eq64_value);
        let eq69_e1494_d_b8: f64 = (eq69_e1492_d_b8 * eq64_value);
        let eq69_e1494_d_b9: f64 = (eq69_e1492_d_b9 * eq64_value);
        let eq69_e1494_d_b10: f64 = (eq69_e1492_d_b10 * eq64_value);
        let eq69_e1494_d_b11: f64 = (eq69_e1492_d_b11 * eq64_value);
        let eq69_e1494_d_b12: f64 = (eq69_e1492_d_b12 * eq64_value);
        let eq69_e1494_d_b13: f64 = (eq69_e1492_d_b13 * eq64_value);
        let eq69_e1494_d_b14: f64 = (eq69_e1492_d_b14 * eq64_value);
        let eq69_e1494_d_b15: f64 = (eq69_e1492_d_b15 * eq64_value);
        let eq69_e1494_d_b16: f64 = (eq69_e1492_d_b16 * eq64_value);
        let eq69_e1494_d_b17: f64 = (eq69_e1492_d_b17 * eq64_value);
        let eq69_e1494_d_b18: f64 = (eq69_e1492_d_b18 * eq64_value);
        let eq69_e1494_d_b19: f64 = (eq69_e1492_d_b19 * eq64_value);
        let eq69_e1494_d_b20: f64 = (eq69_e1492_d_b20 * eq64_value);
        let eq69_e1494_d_b21: f64 = (eq69_e1492_d_b21 * eq64_value);
        let eq69_e1494_d_b22: f64 = (eq69_e1492_d_b22 * eq64_value);
        let eq69_e1494_d_b23: f64 = (eq69_e1492_d_b23 * eq64_value);
        let eq69_e1494_d_b24: f64 = (eq69_e1492_d_b24 * eq64_value);
        let eq69_value: f64 = eq69_e1494;
        let eq69_node_derivatives: [f64; 21] = [eq69_e1494_d_n0, eq69_e1494_d_n1, eq69_e1494_d_n2, eq69_e1494_d_n3, eq69_e1494_d_n4, eq69_e1494_d_n5, eq69_e1494_d_n6, eq69_e1494_d_n7, eq69_e1494_d_n8, eq69_e1494_d_n9, eq69_e1494_d_n10, eq69_e1494_d_n11, eq69_e1494_d_n12, eq69_e1494_d_n13, eq69_e1494_d_n14, eq69_e1494_d_n15, eq69_e1494_d_n16, eq69_e1494_d_n17, eq69_e1494_d_n18, eq69_e1494_d_n19, eq69_e1494_d_n20];
        let eq69_branch_derivatives: [f64; 25] = [eq69_e1494_d_b0, eq69_e1494_d_b1, eq69_e1494_d_b2, eq69_e1494_d_b3, eq69_e1494_d_b4, eq69_e1494_d_b5, eq69_e1494_d_b6, eq69_e1494_d_b7, eq69_e1494_d_b8, eq69_e1494_d_b9, eq69_e1494_d_b10, eq69_e1494_d_b11, eq69_e1494_d_b12, eq69_e1494_d_b13, eq69_e1494_d_b14, eq69_e1494_d_b15, eq69_e1494_d_b16, eq69_e1494_d_b17, eq69_e1494_d_b18, eq69_e1494_d_b19, eq69_e1494_d_b20, eq69_e1494_d_b21, eq69_e1494_d_b22, eq69_e1494_d_b23, eq69_e1494_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq69_value),
            &nodes,
            &eq69_node_derivatives,
            &branches,
            &eq69_branch_derivatives,
            self.multiplicity,
        );
        self.stamp_transient_equation_70_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_71_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_72_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_73_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_74_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_75_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_76_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_77_block_0(ctx, stamper, &mut s);
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

        self.stamp_reactive_equation_56_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_57_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_58_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_59_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_60_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_61_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_62_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_63_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_66_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_67_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_68_block_0(ctx, stamper, &mut s);
    }
}
