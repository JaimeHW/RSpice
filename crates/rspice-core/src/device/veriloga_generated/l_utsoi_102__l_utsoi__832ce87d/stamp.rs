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
        let eq30_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq30_value),
            &[
            ],
        );
        self.stamp_transient_equation_31_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_32_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_33_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_34_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_35_block_0(ctx, stamper, &mut s);
        let eq36_e707: f64 = (s.v[330] * s.v[1795]);
        let eq36_e707_d_n0: f64 = ((s.dn[330][0] * s.v[1795]) + (s.v[330] * s.dn[1795][0]));
        let eq36_e707_d_n1: f64 = ((s.dn[330][1] * s.v[1795]) + (s.v[330] * s.dn[1795][1]));
        let eq36_e707_d_n2: f64 = ((s.dn[330][2] * s.v[1795]) + (s.v[330] * s.dn[1795][2]));
        let eq36_e707_d_n3: f64 = ((s.dn[330][3] * s.v[1795]) + (s.v[330] * s.dn[1795][3]));
        let eq36_e707_d_n4: f64 = ((s.dn[330][4] * s.v[1795]) + (s.v[330] * s.dn[1795][4]));
        let eq36_e707_d_n5: f64 = ((s.dn[330][5] * s.v[1795]) + (s.v[330] * s.dn[1795][5]));
        let eq36_e707_d_n6: f64 = ((s.dn[330][6] * s.v[1795]) + (s.v[330] * s.dn[1795][6]));
        let eq36_e707_d_n7: f64 = ((s.dn[330][7] * s.v[1795]) + (s.v[330] * s.dn[1795][7]));
        let eq36_e707_d_n8: f64 = ((s.dn[330][8] * s.v[1795]) + (s.v[330] * s.dn[1795][8]));
        let eq36_e707_d_n9: f64 = ((s.dn[330][9] * s.v[1795]) + (s.v[330] * s.dn[1795][9]));
        let eq36_e709: f64 = (eq36_e707 * eq30_value);
        let eq36_e709_d_n0: f64 = (eq36_e707_d_n0 * eq30_value);
        let eq36_e709_d_n1: f64 = (eq36_e707_d_n1 * eq30_value);
        let eq36_e709_d_n2: f64 = (eq36_e707_d_n2 * eq30_value);
        let eq36_e709_d_n3: f64 = (eq36_e707_d_n3 * eq30_value);
        let eq36_e709_d_n4: f64 = (eq36_e707_d_n4 * eq30_value);
        let eq36_e709_d_n5: f64 = (eq36_e707_d_n5 * eq30_value);
        let eq36_e709_d_n6: f64 = (eq36_e707_d_n6 * eq30_value);
        let eq36_e709_d_n7: f64 = (eq36_e707_d_n7 * eq30_value);
        let eq36_e709_d_n8: f64 = (eq36_e707_d_n8 * eq30_value);
        let eq36_e709_d_n9: f64 = (eq36_e707_d_n9 * eq30_value);
        let eq36_value: f64 = eq36_e709;
        let eq36_node_derivatives: [f64; 10] = [eq36_e709_d_n0, eq36_e709_d_n1, eq36_e709_d_n2, eq36_e709_d_n3, eq36_e709_d_n4, eq36_e709_d_n5, eq36_e709_d_n6, eq36_e709_d_n7, eq36_e709_d_n8, eq36_e709_d_n9];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq36_value),
            &nodes,
            &eq36_node_derivatives,
            &branches,
            &eq36_branch_derivatives,
            self.multiplicity,
        );
        self.stamp_transient_equation_37_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_38_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_39_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_40_block_0(ctx, stamper, &mut s);
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
        self.stamp_reactive_equation_24_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_25_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_26_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_27_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_28_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_29_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_32_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_33_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_34_block_0(ctx, stamper, &mut s);
    }
}
