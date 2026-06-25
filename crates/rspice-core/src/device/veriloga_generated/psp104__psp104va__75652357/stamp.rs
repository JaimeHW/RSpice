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
        let eq46_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq46_value),
            &[
            ],
        );
        self.stamp_transient_equation_47_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_48_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_49_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_50_block_0(ctx, stamper, &mut s);
        let eq51_e1364: f64 = (s.v[15] * p.p32);
        let eq51_e1364_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq51_e1364_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq51_e1364_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq51_e1364_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq51_e1364_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq51_e1364_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq51_e1364_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq51_e1364_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq51_e1364_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq51_e1364_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq51_e1364_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq51_e1364_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq51_e1365: f64 = (eq51_e1364).sqrt();
        let eq51_e1365_d_n0: f64 = (eq51_e1364_d_n0 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n1: f64 = (eq51_e1364_d_n1 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n2: f64 = (eq51_e1364_d_n2 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n3: f64 = (eq51_e1364_d_n3 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n4: f64 = (eq51_e1364_d_n4 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n5: f64 = (eq51_e1364_d_n5 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n6: f64 = (eq51_e1364_d_n6 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n7: f64 = (eq51_e1364_d_n7 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n8: f64 = (eq51_e1364_d_n8 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n9: f64 = (eq51_e1364_d_n9 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n10: f64 = (eq51_e1364_d_n10 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n11: f64 = (eq51_e1364_d_n11 / (2.0 * eq51_e1365));
        let eq51_e1366: f64 = (s.v[825] * eq51_e1365);
        let eq51_e1366_d_n0: f64 = ((s.dn[825][0] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n0));
        let eq51_e1366_d_n1: f64 = ((s.dn[825][1] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n1));
        let eq51_e1366_d_n2: f64 = ((s.dn[825][2] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n2));
        let eq51_e1366_d_n3: f64 = ((s.dn[825][3] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n3));
        let eq51_e1366_d_n4: f64 = ((s.dn[825][4] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n4));
        let eq51_e1366_d_n5: f64 = ((s.dn[825][5] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n5));
        let eq51_e1366_d_n6: f64 = ((s.dn[825][6] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n6));
        let eq51_e1366_d_n7: f64 = ((s.dn[825][7] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n7));
        let eq51_e1366_d_n8: f64 = ((s.dn[825][8] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n8));
        let eq51_e1366_d_n9: f64 = ((s.dn[825][9] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n9));
        let eq51_e1366_d_n10: f64 = ((s.dn[825][10] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n10));
        let eq51_e1366_d_n11: f64 = ((s.dn[825][11] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n11));
        let eq51_e1368: f64 = (eq51_e1366 * s.v[855]);
        let eq51_e1368_d_n0: f64 = ((eq51_e1366_d_n0 * s.v[855]) + (eq51_e1366 * s.dn[855][0]));
        let eq51_e1368_d_n1: f64 = ((eq51_e1366_d_n1 * s.v[855]) + (eq51_e1366 * s.dn[855][1]));
        let eq51_e1368_d_n2: f64 = ((eq51_e1366_d_n2 * s.v[855]) + (eq51_e1366 * s.dn[855][2]));
        let eq51_e1368_d_n3: f64 = ((eq51_e1366_d_n3 * s.v[855]) + (eq51_e1366 * s.dn[855][3]));
        let eq51_e1368_d_n4: f64 = ((eq51_e1366_d_n4 * s.v[855]) + (eq51_e1366 * s.dn[855][4]));
        let eq51_e1368_d_n5: f64 = ((eq51_e1366_d_n5 * s.v[855]) + (eq51_e1366 * s.dn[855][5]));
        let eq51_e1368_d_n6: f64 = ((eq51_e1366_d_n6 * s.v[855]) + (eq51_e1366 * s.dn[855][6]));
        let eq51_e1368_d_n7: f64 = ((eq51_e1366_d_n7 * s.v[855]) + (eq51_e1366 * s.dn[855][7]));
        let eq51_e1368_d_n8: f64 = ((eq51_e1366_d_n8 * s.v[855]) + (eq51_e1366 * s.dn[855][8]));
        let eq51_e1368_d_n9: f64 = ((eq51_e1366_d_n9 * s.v[855]) + (eq51_e1366 * s.dn[855][9]));
        let eq51_e1368_d_n10: f64 = ((eq51_e1366_d_n10 * s.v[855]) + (eq51_e1366 * s.dn[855][10]));
        let eq51_e1368_d_n11: f64 = ((eq51_e1366_d_n11 * s.v[855]) + (eq51_e1366 * s.dn[855][11]));
        let eq51_e1370: f64 = (eq51_e1368 * eq46_value);
        let eq51_e1370_d_n0: f64 = (eq51_e1368_d_n0 * eq46_value);
        let eq51_e1370_d_n1: f64 = (eq51_e1368_d_n1 * eq46_value);
        let eq51_e1370_d_n2: f64 = (eq51_e1368_d_n2 * eq46_value);
        let eq51_e1370_d_n3: f64 = (eq51_e1368_d_n3 * eq46_value);
        let eq51_e1370_d_n4: f64 = (eq51_e1368_d_n4 * eq46_value);
        let eq51_e1370_d_n5: f64 = (eq51_e1368_d_n5 * eq46_value);
        let eq51_e1370_d_n6: f64 = (eq51_e1368_d_n6 * eq46_value);
        let eq51_e1370_d_n7: f64 = (eq51_e1368_d_n7 * eq46_value);
        let eq51_e1370_d_n8: f64 = (eq51_e1368_d_n8 * eq46_value);
        let eq51_e1370_d_n9: f64 = (eq51_e1368_d_n9 * eq46_value);
        let eq51_e1370_d_n10: f64 = (eq51_e1368_d_n10 * eq46_value);
        let eq51_e1370_d_n11: f64 = (eq51_e1368_d_n11 * eq46_value);
        let eq51_value: f64 = eq51_e1370;
        let eq51_node_derivatives: [f64; 12] = [eq51_e1370_d_n0, eq51_e1370_d_n1, eq51_e1370_d_n2, eq51_e1370_d_n3, eq51_e1370_d_n4, eq51_e1370_d_n5, eq51_e1370_d_n6, eq51_e1370_d_n7, eq51_e1370_d_n8, eq51_e1370_d_n9, eq51_e1370_d_n10, eq51_e1370_d_n11];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq51_value),
            &nodes,
            &eq51_node_derivatives,
            &branches,
            &eq51_branch_derivatives,
            self.multiplicity,
        );
        self.stamp_transient_equation_52_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_53_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_54_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_55_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_56_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_57_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_58_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_59_block_0(ctx, stamper, &mut s);
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

        self.stamp_reactive_equation_38_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_39_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_40_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_41_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_42_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_43_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_44_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_45_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_48_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_49_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_50_block_0(ctx, stamper, &mut s);
    }
}
