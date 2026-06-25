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

        stamper.stamp_potential_branch(
            Some(nodes[1]),
            Some(nodes[9]),
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
            Some(nodes[3]),
            Some(nodes[8]),
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
        let eq39_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq39_value),
            &[
            ],
        );
        self.stamp_transient_equation_40_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_41_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_42_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_43_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_44_block_0(ctx, stamper, &mut s);
        let eq45_e779: f64 = (s.v[334] * s.v[1805]);
        let eq45_e779_d_n0: f64 = ((s.dn[334][0] * s.v[1805]) + (s.v[334] * s.dn[1805][0]));
        let eq45_e779_d_n1: f64 = ((s.dn[334][1] * s.v[1805]) + (s.v[334] * s.dn[1805][1]));
        let eq45_e779_d_n2: f64 = ((s.dn[334][2] * s.v[1805]) + (s.v[334] * s.dn[1805][2]));
        let eq45_e779_d_n3: f64 = ((s.dn[334][3] * s.v[1805]) + (s.v[334] * s.dn[1805][3]));
        let eq45_e779_d_n4: f64 = ((s.dn[334][4] * s.v[1805]) + (s.v[334] * s.dn[1805][4]));
        let eq45_e779_d_n5: f64 = ((s.dn[334][5] * s.v[1805]) + (s.v[334] * s.dn[1805][5]));
        let eq45_e779_d_n6: f64 = ((s.dn[334][6] * s.v[1805]) + (s.v[334] * s.dn[1805][6]));
        let eq45_e779_d_n7: f64 = ((s.dn[334][7] * s.v[1805]) + (s.v[334] * s.dn[1805][7]));
        let eq45_e779_d_n8: f64 = ((s.dn[334][8] * s.v[1805]) + (s.v[334] * s.dn[1805][8]));
        let eq45_e779_d_n9: f64 = ((s.dn[334][9] * s.v[1805]) + (s.v[334] * s.dn[1805][9]));
        let eq45_e779_d_n10: f64 = ((s.dn[334][10] * s.v[1805]) + (s.v[334] * s.dn[1805][10]));
        let eq45_e779_d_n11: f64 = ((s.dn[334][11] * s.v[1805]) + (s.v[334] * s.dn[1805][11]));
        let eq45_e779_d_n12: f64 = ((s.dn[334][12] * s.v[1805]) + (s.v[334] * s.dn[1805][12]));
        let eq45_e779_d_n13: f64 = ((s.dn[334][13] * s.v[1805]) + (s.v[334] * s.dn[1805][13]));
        let eq45_e781: f64 = (eq45_e779 * eq39_value);
        let eq45_e781_d_n0: f64 = (eq45_e779_d_n0 * eq39_value);
        let eq45_e781_d_n1: f64 = (eq45_e779_d_n1 * eq39_value);
        let eq45_e781_d_n2: f64 = (eq45_e779_d_n2 * eq39_value);
        let eq45_e781_d_n3: f64 = (eq45_e779_d_n3 * eq39_value);
        let eq45_e781_d_n4: f64 = (eq45_e779_d_n4 * eq39_value);
        let eq45_e781_d_n5: f64 = (eq45_e779_d_n5 * eq39_value);
        let eq45_e781_d_n6: f64 = (eq45_e779_d_n6 * eq39_value);
        let eq45_e781_d_n7: f64 = (eq45_e779_d_n7 * eq39_value);
        let eq45_e781_d_n8: f64 = (eq45_e779_d_n8 * eq39_value);
        let eq45_e781_d_n9: f64 = (eq45_e779_d_n9 * eq39_value);
        let eq45_e781_d_n10: f64 = (eq45_e779_d_n10 * eq39_value);
        let eq45_e781_d_n11: f64 = (eq45_e779_d_n11 * eq39_value);
        let eq45_e781_d_n12: f64 = (eq45_e779_d_n12 * eq39_value);
        let eq45_e781_d_n13: f64 = (eq45_e779_d_n13 * eq39_value);
        let eq45_value: f64 = eq45_e781;
        let eq45_node_derivatives: [f64; 14] = [eq45_e781_d_n0, eq45_e781_d_n1, eq45_e781_d_n2, eq45_e781_d_n3, eq45_e781_d_n4, eq45_e781_d_n5, eq45_e781_d_n6, eq45_e781_d_n7, eq45_e781_d_n8, eq45_e781_d_n9, eq45_e781_d_n10, eq45_e781_d_n11, eq45_e781_d_n12, eq45_e781_d_n13];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq45_value),
            &nodes,
            &eq45_node_derivatives,
            &branches,
            &eq45_branch_derivatives,
            self.multiplicity,
        );
        self.stamp_transient_equation_46_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_47_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_48_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_49_block_0(ctx, stamper, &mut s);
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

        self.stamp_reactive_equation_23_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_25_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_26_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_28_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_29_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_31_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_32_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_33_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_34_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_35_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_36_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_37_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_38_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_41_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_42_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_43_block_0(ctx, stamper, &mut s);
    }
}
