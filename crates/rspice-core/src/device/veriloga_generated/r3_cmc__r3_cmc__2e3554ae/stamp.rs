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

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let bi0 = ctx.branch_current(branches[0]);
        let bi1 = ctx.branch_current(branches[1]);
        let mut s = Scratch::new();

        self.stamp_transient_block_0(ctx, stamper, &mut s);
        self.stamp_transient_block_1(ctx, stamper, &mut s);
        self.stamp_transient_block_2(ctx, stamper, &mut s);
        self.stamp_transient_block_3(ctx, stamper, &mut s);

        stamper.stamp_potential_branch(
            Some(nodes[0]),
            Some(nodes[4]),
            branches[0],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[2]),
            Some(nodes[5]),
            branches[1],
            self.multiplicity,
        );

        self.stamp_transient_equation_0_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_1_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_2_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_3_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_4_block_0(ctx, stamper, &mut s);
        let (eq5_e153, eq5_e153_d_n0, eq5_e153_d_n1, eq5_e153_d_n2, eq5_e153_d_n3, eq5_e153_d_n4, eq5_e153_d_n5, eq5_e153_d_b0, eq5_e153_d_b1,) = {
    if (s.v[321] != 0.0) {
        let eq5_e149: f64 = (bi0 * s.v[54]);
        let eq5_e149_d_n0: f64 = (bi0 * s.dn[54][0]);
        let eq5_e149_d_n1: f64 = (bi0 * s.dn[54][1]);
        let eq5_e149_d_n2: f64 = (bi0 * s.dn[54][2]);
        let eq5_e149_d_n3: f64 = (bi0 * s.dn[54][3]);
        let eq5_e149_d_n4: f64 = (bi0 * s.dn[54][4]);
        let eq5_e149_d_n5: f64 = (bi0 * s.dn[54][5]);
        let eq5_e149_d_b0: f64 = (s.v[54] + (bi0 * s.db[54][0]));
        let eq5_e149_d_b1: f64 = (bi0 * s.db[54][1]);
        let eq5_e151: f64 = (eq5_e149 * s.v[58]);
        let eq5_e151_d_n0: f64 = ((eq5_e149_d_n0 * s.v[58]) + (eq5_e149 * s.dn[58][0]));
        let eq5_e151_d_n1: f64 = ((eq5_e149_d_n1 * s.v[58]) + (eq5_e149 * s.dn[58][1]));
        let eq5_e151_d_n2: f64 = ((eq5_e149_d_n2 * s.v[58]) + (eq5_e149 * s.dn[58][2]));
        let eq5_e151_d_n3: f64 = ((eq5_e149_d_n3 * s.v[58]) + (eq5_e149 * s.dn[58][3]));
        let eq5_e151_d_n4: f64 = ((eq5_e149_d_n4 * s.v[58]) + (eq5_e149 * s.dn[58][4]));
        let eq5_e151_d_n5: f64 = ((eq5_e149_d_n5 * s.v[58]) + (eq5_e149 * s.dn[58][5]));
        let eq5_e151_d_b0: f64 = ((eq5_e149_d_b0 * s.v[58]) + (eq5_e149 * s.db[58][0]));
        let eq5_e151_d_b1: f64 = ((eq5_e149_d_b1 * s.v[58]) + (eq5_e149 * s.db[58][1]));
        (eq5_e151, eq5_e151_d_n0, eq5_e151_d_n1, eq5_e151_d_n2, eq5_e151_d_n3, eq5_e151_d_n4, eq5_e151_d_n5, eq5_e151_d_b0, eq5_e151_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e153;
        let eq5_node_derivatives: [f64; 6] = [eq5_e153_d_n0, eq5_e153_d_n1, eq5_e153_d_n2, eq5_e153_d_n3, eq5_e153_d_n4, eq5_e153_d_n5];
        let eq5_branch_derivatives: [f64; 2] = [eq5_e153_d_b0, eq5_e153_d_b1];
        stamper.stamp_potential_dense(
            branches[0],
            eq5_value,
            &nodes,
            &eq5_node_derivatives,
            &branches,
            &eq5_branch_derivatives,
        );
        self.stamp_transient_equation_6_block_0(ctx, stamper, &mut s);
        let (eq7_e170, eq7_e170_d_n0, eq7_e170_d_n1, eq7_e170_d_n2, eq7_e170_d_n3, eq7_e170_d_n4, eq7_e170_d_n5, eq7_e170_d_b0, eq7_e170_d_b1,) = {
    if (s.v[322] != 0.0) {
        let eq7_e166: f64 = (bi1 * s.v[55]);
        let eq7_e166_d_n0: f64 = (bi1 * s.dn[55][0]);
        let eq7_e166_d_n1: f64 = (bi1 * s.dn[55][1]);
        let eq7_e166_d_n2: f64 = (bi1 * s.dn[55][2]);
        let eq7_e166_d_n3: f64 = (bi1 * s.dn[55][3]);
        let eq7_e166_d_n4: f64 = (bi1 * s.dn[55][4]);
        let eq7_e166_d_n5: f64 = (bi1 * s.dn[55][5]);
        let eq7_e166_d_b0: f64 = (bi1 * s.db[55][0]);
        let eq7_e166_d_b1: f64 = (s.v[55] + (bi1 * s.db[55][1]));
        let eq7_e168: f64 = (eq7_e166 * s.v[58]);
        let eq7_e168_d_n0: f64 = ((eq7_e166_d_n0 * s.v[58]) + (eq7_e166 * s.dn[58][0]));
        let eq7_e168_d_n1: f64 = ((eq7_e166_d_n1 * s.v[58]) + (eq7_e166 * s.dn[58][1]));
        let eq7_e168_d_n2: f64 = ((eq7_e166_d_n2 * s.v[58]) + (eq7_e166 * s.dn[58][2]));
        let eq7_e168_d_n3: f64 = ((eq7_e166_d_n3 * s.v[58]) + (eq7_e166 * s.dn[58][3]));
        let eq7_e168_d_n4: f64 = ((eq7_e166_d_n4 * s.v[58]) + (eq7_e166 * s.dn[58][4]));
        let eq7_e168_d_n5: f64 = ((eq7_e166_d_n5 * s.v[58]) + (eq7_e166 * s.dn[58][5]));
        let eq7_e168_d_b0: f64 = ((eq7_e166_d_b0 * s.v[58]) + (eq7_e166 * s.db[58][0]));
        let eq7_e168_d_b1: f64 = ((eq7_e166_d_b1 * s.v[58]) + (eq7_e166 * s.db[58][1]));
        (eq7_e168, eq7_e168_d_n0, eq7_e168_d_n1, eq7_e168_d_n2, eq7_e168_d_n3, eq7_e168_d_n4, eq7_e168_d_n5, eq7_e168_d_b0, eq7_e168_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e170;
        let eq7_node_derivatives: [f64; 6] = [eq7_e170_d_n0, eq7_e170_d_n1, eq7_e170_d_n2, eq7_e170_d_n3, eq7_e170_d_n4, eq7_e170_d_n5];
        let eq7_branch_derivatives: [f64; 2] = [eq7_e170_d_b0, eq7_e170_d_b1];
        stamper.stamp_potential_dense(
            branches[1],
            eq7_value,
            &nodes,
            &eq7_node_derivatives,
            &branches,
            &eq7_branch_derivatives,
        );
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
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut s = ReactiveScratch::new();

        self.stamp_reactive_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_block_1(ctx, stamper, &mut s);

        self.stamp_reactive_equation_9_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_10_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_11_block_0(ctx, stamper, &mut s);
    }
}
