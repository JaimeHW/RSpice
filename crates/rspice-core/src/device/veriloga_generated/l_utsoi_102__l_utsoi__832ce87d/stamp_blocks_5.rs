#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq19_e603,) = {
    if (!(s.v[1766] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e603;
        stamper.stamp_potential(
            branches[2],
            eq19_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq20_e613, eq20_e613_d_n0, eq20_e613_d_n1, eq20_e613_d_n2, eq20_e613_d_n3, eq20_e613_d_n4, eq20_e613_d_n5, eq20_e613_d_n6, eq20_e613_d_n7, eq20_e613_d_n8, eq20_e613_d_n9,) = {
    if (s.v[1767] != 0.0) {
        let eq20_e607: f64 = (p.p31 * s.v[13]);
        let eq20_e607_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq20_e607_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq20_e607_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq20_e607_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq20_e607_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq20_e607_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq20_e607_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq20_e607_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq20_e607_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq20_e607_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq20_e609: f64 = (eq20_e607 * s.v[323]);
        let eq20_e609_d_n0: f64 = ((eq20_e607_d_n0 * s.v[323]) + (eq20_e607 * s.dn[323][0]));
        let eq20_e609_d_n1: f64 = ((eq20_e607_d_n1 * s.v[323]) + (eq20_e607 * s.dn[323][1]));
        let eq20_e609_d_n2: f64 = ((eq20_e607_d_n2 * s.v[323]) + (eq20_e607 * s.dn[323][2]));
        let eq20_e609_d_n3: f64 = ((eq20_e607_d_n3 * s.v[323]) + (eq20_e607 * s.dn[323][3]));
        let eq20_e609_d_n4: f64 = ((eq20_e607_d_n4 * s.v[323]) + (eq20_e607 * s.dn[323][4]));
        let eq20_e609_d_n5: f64 = ((eq20_e607_d_n5 * s.v[323]) + (eq20_e607 * s.dn[323][5]));
        let eq20_e609_d_n6: f64 = ((eq20_e607_d_n6 * s.v[323]) + (eq20_e607 * s.dn[323][6]));
        let eq20_e609_d_n7: f64 = ((eq20_e607_d_n7 * s.v[323]) + (eq20_e607 * s.dn[323][7]));
        let eq20_e609_d_n8: f64 = ((eq20_e607_d_n8 * s.v[323]) + (eq20_e607 * s.dn[323][8]));
        let eq20_e609_d_n9: f64 = ((eq20_e607_d_n9 * s.v[323]) + (eq20_e607 * s.dn[323][9]));
        let eq20_e611: f64 = (eq20_e609 * (nv3 - nv8));
        let eq20_e611_d_n0: f64 = (eq20_e609_d_n0 * (nv3 - nv8));
        let eq20_e611_d_n1: f64 = (eq20_e609_d_n1 * (nv3 - nv8));
        let eq20_e611_d_n2: f64 = (eq20_e609_d_n2 * (nv3 - nv8));
        let eq20_e611_d_n3: f64 = ((eq20_e609_d_n3 * (nv3 - nv8)) + eq20_e609);
        let eq20_e611_d_n4: f64 = (eq20_e609_d_n4 * (nv3 - nv8));
        let eq20_e611_d_n5: f64 = (eq20_e609_d_n5 * (nv3 - nv8));
        let eq20_e611_d_n6: f64 = (eq20_e609_d_n6 * (nv3 - nv8));
        let eq20_e611_d_n7: f64 = (eq20_e609_d_n7 * (nv3 - nv8));
        let eq20_e611_d_n8: f64 = ((eq20_e609_d_n8 * (nv3 - nv8)) + (-eq20_e609));
        let eq20_e611_d_n9: f64 = (eq20_e609_d_n9 * (nv3 - nv8));
        (eq20_e611, eq20_e611_d_n0, eq20_e611_d_n1, eq20_e611_d_n2, eq20_e611_d_n3, eq20_e611_d_n4, eq20_e611_d_n5, eq20_e611_d_n6, eq20_e611_d_n7, eq20_e611_d_n8, eq20_e611_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e613;
        let eq20_node_derivatives: [f64; 10] = [eq20_e613_d_n0, eq20_e613_d_n1, eq20_e613_d_n2, eq20_e613_d_n3, eq20_e613_d_n4, eq20_e613_d_n5, eq20_e613_d_n6, eq20_e613_d_n7, eq20_e613_d_n8, eq20_e613_d_n9];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            self.multiplicity * (eq20_value),
            &nodes,
            &eq20_node_derivatives,
            &branches,
            &eq20_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_21_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq21_e623,) = {
    if (s.v[1767] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e623;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[8]),
            self.multiplicity * (eq21_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq22_e628,) = {
    if (!(s.v[1767] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e628;
        stamper.stamp_potential(
            branches[3],
            eq22_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq23_e631: f64 = self.eval_ddt(0, s.v[358]);
        let eq23_e631_d_n0: f64 = self.ddt_jacobian(s.dn[358][0]);
        let eq23_e631_d_n1: f64 = self.ddt_jacobian(s.dn[358][1]);
        let eq23_e631_d_n2: f64 = self.ddt_jacobian(s.dn[358][2]);
        let eq23_e631_d_n3: f64 = self.ddt_jacobian(s.dn[358][3]);
        let eq23_e631_d_n4: f64 = self.ddt_jacobian(s.dn[358][4]);
        let eq23_e631_d_n5: f64 = self.ddt_jacobian(s.dn[358][5]);
        let eq23_e631_d_n6: f64 = self.ddt_jacobian(s.dn[358][6]);
        let eq23_e631_d_n7: f64 = self.ddt_jacobian(s.dn[358][7]);
        let eq23_e631_d_n8: f64 = self.ddt_jacobian(s.dn[358][8]);
        let eq23_e631_d_n9: f64 = self.ddt_jacobian(s.dn[358][9]);
        let eq23_e633: f64 = self.eval_ddt(1, s.v[373]);
        let eq23_e633_d_n0: f64 = self.ddt_jacobian(s.dn[373][0]);
        let eq23_e633_d_n1: f64 = self.ddt_jacobian(s.dn[373][1]);
        let eq23_e633_d_n2: f64 = self.ddt_jacobian(s.dn[373][2]);
        let eq23_e633_d_n3: f64 = self.ddt_jacobian(s.dn[373][3]);
        let eq23_e633_d_n4: f64 = self.ddt_jacobian(s.dn[373][4]);
        let eq23_e633_d_n5: f64 = self.ddt_jacobian(s.dn[373][5]);
        let eq23_e633_d_n6: f64 = self.ddt_jacobian(s.dn[373][6]);
        let eq23_e633_d_n7: f64 = self.ddt_jacobian(s.dn[373][7]);
        let eq23_e633_d_n8: f64 = self.ddt_jacobian(s.dn[373][8]);
        let eq23_e633_d_n9: f64 = self.ddt_jacobian(s.dn[373][9]);
        let eq23_e634: f64 = (eq23_e631 + eq23_e633);
        let eq23_e634_d_n0: f64 = (eq23_e631_d_n0 + eq23_e633_d_n0);
        let eq23_e634_d_n1: f64 = (eq23_e631_d_n1 + eq23_e633_d_n1);
        let eq23_e634_d_n2: f64 = (eq23_e631_d_n2 + eq23_e633_d_n2);
        let eq23_e634_d_n3: f64 = (eq23_e631_d_n3 + eq23_e633_d_n3);
        let eq23_e634_d_n4: f64 = (eq23_e631_d_n4 + eq23_e633_d_n4);
        let eq23_e634_d_n5: f64 = (eq23_e631_d_n5 + eq23_e633_d_n5);
        let eq23_e634_d_n6: f64 = (eq23_e631_d_n6 + eq23_e633_d_n6);
        let eq23_e634_d_n7: f64 = (eq23_e631_d_n7 + eq23_e633_d_n7);
        let eq23_e634_d_n8: f64 = (eq23_e631_d_n8 + eq23_e633_d_n8);
        let eq23_e634_d_n9: f64 = (eq23_e631_d_n9 + eq23_e633_d_n9);
        let eq23_e636: f64 = self.eval_ddt(2, s.v[377]);
        let eq23_e636_d_n0: f64 = self.ddt_jacobian(s.dn[377][0]);
        let eq23_e636_d_n1: f64 = self.ddt_jacobian(s.dn[377][1]);
        let eq23_e636_d_n2: f64 = self.ddt_jacobian(s.dn[377][2]);
        let eq23_e636_d_n3: f64 = self.ddt_jacobian(s.dn[377][3]);
        let eq23_e636_d_n4: f64 = self.ddt_jacobian(s.dn[377][4]);
        let eq23_e636_d_n5: f64 = self.ddt_jacobian(s.dn[377][5]);
        let eq23_e636_d_n6: f64 = self.ddt_jacobian(s.dn[377][6]);
        let eq23_e636_d_n7: f64 = self.ddt_jacobian(s.dn[377][7]);
        let eq23_e636_d_n8: f64 = self.ddt_jacobian(s.dn[377][8]);
        let eq23_e636_d_n9: f64 = self.ddt_jacobian(s.dn[377][9]);
        let eq23_e637: f64 = (eq23_e634 + eq23_e636);
        let eq23_e637_d_n0: f64 = (eq23_e634_d_n0 + eq23_e636_d_n0);
        let eq23_e637_d_n1: f64 = (eq23_e634_d_n1 + eq23_e636_d_n1);
        let eq23_e637_d_n2: f64 = (eq23_e634_d_n2 + eq23_e636_d_n2);
        let eq23_e637_d_n3: f64 = (eq23_e634_d_n3 + eq23_e636_d_n3);
        let eq23_e637_d_n4: f64 = (eq23_e634_d_n4 + eq23_e636_d_n4);
        let eq23_e637_d_n5: f64 = (eq23_e634_d_n5 + eq23_e636_d_n5);
        let eq23_e637_d_n6: f64 = (eq23_e634_d_n6 + eq23_e636_d_n6);
        let eq23_e637_d_n7: f64 = (eq23_e634_d_n7 + eq23_e636_d_n7);
        let eq23_e637_d_n8: f64 = (eq23_e634_d_n8 + eq23_e636_d_n8);
        let eq23_e637_d_n9: f64 = (eq23_e634_d_n9 + eq23_e636_d_n9);
        let eq23_e638: f64 = (p.p14 * eq23_e637);
        let eq23_e638_d_n0: f64 = (p.p14 * eq23_e637_d_n0);
        let eq23_e638_d_n1: f64 = (p.p14 * eq23_e637_d_n1);
        let eq23_e638_d_n2: f64 = (p.p14 * eq23_e637_d_n2);
        let eq23_e638_d_n3: f64 = (p.p14 * eq23_e637_d_n3);
        let eq23_e638_d_n4: f64 = (p.p14 * eq23_e637_d_n4);
        let eq23_e638_d_n5: f64 = (p.p14 * eq23_e637_d_n5);
        let eq23_e638_d_n6: f64 = (p.p14 * eq23_e637_d_n6);
        let eq23_e638_d_n7: f64 = (p.p14 * eq23_e637_d_n7);
        let eq23_e638_d_n8: f64 = (p.p14 * eq23_e637_d_n8);
        let eq23_e638_d_n9: f64 = (p.p14 * eq23_e637_d_n9);
        let eq23_value: f64 = eq23_e638;
        let eq23_node_derivatives: [f64; 10] = [eq23_e638_d_n0, eq23_e638_d_n1, eq23_e638_d_n2, eq23_e638_d_n3, eq23_e638_d_n4, eq23_e638_d_n5, eq23_e638_d_n6, eq23_e638_d_n7, eq23_e638_d_n8, eq23_e638_d_n9];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq23_value),
            &nodes,
            &eq23_node_derivatives,
            &branches,
            &eq23_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_24_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq24_e641: f64 = self.eval_ddt(3, s.v[367]);
        let eq24_e641_d_n0: f64 = self.ddt_jacobian(s.dn[367][0]);
        let eq24_e641_d_n1: f64 = self.ddt_jacobian(s.dn[367][1]);
        let eq24_e641_d_n2: f64 = self.ddt_jacobian(s.dn[367][2]);
        let eq24_e641_d_n3: f64 = self.ddt_jacobian(s.dn[367][3]);
        let eq24_e641_d_n4: f64 = self.ddt_jacobian(s.dn[367][4]);
        let eq24_e641_d_n5: f64 = self.ddt_jacobian(s.dn[367][5]);
        let eq24_e641_d_n6: f64 = self.ddt_jacobian(s.dn[367][6]);
        let eq24_e641_d_n7: f64 = self.ddt_jacobian(s.dn[367][7]);
        let eq24_e641_d_n8: f64 = self.ddt_jacobian(s.dn[367][8]);
        let eq24_e641_d_n9: f64 = self.ddt_jacobian(s.dn[367][9]);
        let eq24_e643: f64 = self.eval_ddt(4, s.v[369]);
        let eq24_e643_d_n0: f64 = self.ddt_jacobian(s.dn[369][0]);
        let eq24_e643_d_n1: f64 = self.ddt_jacobian(s.dn[369][1]);
        let eq24_e643_d_n2: f64 = self.ddt_jacobian(s.dn[369][2]);
        let eq24_e643_d_n3: f64 = self.ddt_jacobian(s.dn[369][3]);
        let eq24_e643_d_n4: f64 = self.ddt_jacobian(s.dn[369][4]);
        let eq24_e643_d_n5: f64 = self.ddt_jacobian(s.dn[369][5]);
        let eq24_e643_d_n6: f64 = self.ddt_jacobian(s.dn[369][6]);
        let eq24_e643_d_n7: f64 = self.ddt_jacobian(s.dn[369][7]);
        let eq24_e643_d_n8: f64 = self.ddt_jacobian(s.dn[369][8]);
        let eq24_e643_d_n9: f64 = self.ddt_jacobian(s.dn[369][9]);
        let eq24_e644: f64 = (eq24_e641 + eq24_e643);
        let eq24_e644_d_n0: f64 = (eq24_e641_d_n0 + eq24_e643_d_n0);
        let eq24_e644_d_n1: f64 = (eq24_e641_d_n1 + eq24_e643_d_n1);
        let eq24_e644_d_n2: f64 = (eq24_e641_d_n2 + eq24_e643_d_n2);
        let eq24_e644_d_n3: f64 = (eq24_e641_d_n3 + eq24_e643_d_n3);
        let eq24_e644_d_n4: f64 = (eq24_e641_d_n4 + eq24_e643_d_n4);
        let eq24_e644_d_n5: f64 = (eq24_e641_d_n5 + eq24_e643_d_n5);
        let eq24_e644_d_n6: f64 = (eq24_e641_d_n6 + eq24_e643_d_n6);
        let eq24_e644_d_n7: f64 = (eq24_e641_d_n7 + eq24_e643_d_n7);
        let eq24_e644_d_n8: f64 = (eq24_e641_d_n8 + eq24_e643_d_n8);
        let eq24_e644_d_n9: f64 = (eq24_e641_d_n9 + eq24_e643_d_n9);
        let eq24_e646: f64 = self.eval_ddt(5, s.v[376]);
        let eq24_e646_d_n0: f64 = self.ddt_jacobian(s.dn[376][0]);
        let eq24_e646_d_n1: f64 = self.ddt_jacobian(s.dn[376][1]);
        let eq24_e646_d_n2: f64 = self.ddt_jacobian(s.dn[376][2]);
        let eq24_e646_d_n3: f64 = self.ddt_jacobian(s.dn[376][3]);
        let eq24_e646_d_n4: f64 = self.ddt_jacobian(s.dn[376][4]);
        let eq24_e646_d_n5: f64 = self.ddt_jacobian(s.dn[376][5]);
        let eq24_e646_d_n6: f64 = self.ddt_jacobian(s.dn[376][6]);
        let eq24_e646_d_n7: f64 = self.ddt_jacobian(s.dn[376][7]);
        let eq24_e646_d_n8: f64 = self.ddt_jacobian(s.dn[376][8]);
        let eq24_e646_d_n9: f64 = self.ddt_jacobian(s.dn[376][9]);
        let eq24_e647: f64 = (eq24_e644 + eq24_e646);
        let eq24_e647_d_n0: f64 = (eq24_e644_d_n0 + eq24_e646_d_n0);
        let eq24_e647_d_n1: f64 = (eq24_e644_d_n1 + eq24_e646_d_n1);
        let eq24_e647_d_n2: f64 = (eq24_e644_d_n2 + eq24_e646_d_n2);
        let eq24_e647_d_n3: f64 = (eq24_e644_d_n3 + eq24_e646_d_n3);
        let eq24_e647_d_n4: f64 = (eq24_e644_d_n4 + eq24_e646_d_n4);
        let eq24_e647_d_n5: f64 = (eq24_e644_d_n5 + eq24_e646_d_n5);
        let eq24_e647_d_n6: f64 = (eq24_e644_d_n6 + eq24_e646_d_n6);
        let eq24_e647_d_n7: f64 = (eq24_e644_d_n7 + eq24_e646_d_n7);
        let eq24_e647_d_n8: f64 = (eq24_e644_d_n8 + eq24_e646_d_n8);
        let eq24_e647_d_n9: f64 = (eq24_e644_d_n9 + eq24_e646_d_n9);
        let eq24_e648: f64 = (p.p14 * eq24_e647);
        let eq24_e648_d_n0: f64 = (p.p14 * eq24_e647_d_n0);
        let eq24_e648_d_n1: f64 = (p.p14 * eq24_e647_d_n1);
        let eq24_e648_d_n2: f64 = (p.p14 * eq24_e647_d_n2);
        let eq24_e648_d_n3: f64 = (p.p14 * eq24_e647_d_n3);
        let eq24_e648_d_n4: f64 = (p.p14 * eq24_e647_d_n4);
        let eq24_e648_d_n5: f64 = (p.p14 * eq24_e647_d_n5);
        let eq24_e648_d_n6: f64 = (p.p14 * eq24_e647_d_n6);
        let eq24_e648_d_n7: f64 = (p.p14 * eq24_e647_d_n7);
        let eq24_e648_d_n8: f64 = (p.p14 * eq24_e647_d_n8);
        let eq24_e648_d_n9: f64 = (p.p14 * eq24_e647_d_n9);
        let eq24_value: f64 = eq24_e648;
        let eq24_node_derivatives: [f64; 10] = [eq24_e648_d_n0, eq24_e648_d_n1, eq24_e648_d_n2, eq24_e648_d_n3, eq24_e648_d_n4, eq24_e648_d_n5, eq24_e648_d_n6, eq24_e648_d_n7, eq24_e648_d_n8, eq24_e648_d_n9];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq24_value),
            &nodes,
            &eq24_node_derivatives,
            &branches,
            &eq24_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_25_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq25_e651: f64 = self.eval_ddt(6, s.v[372]);
        let eq25_e651_d_n0: f64 = self.ddt_jacobian(s.dn[372][0]);
        let eq25_e651_d_n1: f64 = self.ddt_jacobian(s.dn[372][1]);
        let eq25_e651_d_n2: f64 = self.ddt_jacobian(s.dn[372][2]);
        let eq25_e651_d_n3: f64 = self.ddt_jacobian(s.dn[372][3]);
        let eq25_e651_d_n4: f64 = self.ddt_jacobian(s.dn[372][4]);
        let eq25_e651_d_n5: f64 = self.ddt_jacobian(s.dn[372][5]);
        let eq25_e651_d_n6: f64 = self.ddt_jacobian(s.dn[372][6]);
        let eq25_e651_d_n7: f64 = self.ddt_jacobian(s.dn[372][7]);
        let eq25_e651_d_n8: f64 = self.ddt_jacobian(s.dn[372][8]);
        let eq25_e651_d_n9: f64 = self.ddt_jacobian(s.dn[372][9]);
        let eq25_e653: f64 = self.eval_ddt(7, s.v[378]);
        let eq25_e653_d_n0: f64 = self.ddt_jacobian(s.dn[378][0]);
        let eq25_e653_d_n1: f64 = self.ddt_jacobian(s.dn[378][1]);
        let eq25_e653_d_n2: f64 = self.ddt_jacobian(s.dn[378][2]);
        let eq25_e653_d_n3: f64 = self.ddt_jacobian(s.dn[378][3]);
        let eq25_e653_d_n4: f64 = self.ddt_jacobian(s.dn[378][4]);
        let eq25_e653_d_n5: f64 = self.ddt_jacobian(s.dn[378][5]);
        let eq25_e653_d_n6: f64 = self.ddt_jacobian(s.dn[378][6]);
        let eq25_e653_d_n7: f64 = self.ddt_jacobian(s.dn[378][7]);
        let eq25_e653_d_n8: f64 = self.ddt_jacobian(s.dn[378][8]);
        let eq25_e653_d_n9: f64 = self.ddt_jacobian(s.dn[378][9]);
        let eq25_e654: f64 = (eq25_e651 + eq25_e653);
        let eq25_e654_d_n0: f64 = (eq25_e651_d_n0 + eq25_e653_d_n0);
        let eq25_e654_d_n1: f64 = (eq25_e651_d_n1 + eq25_e653_d_n1);
        let eq25_e654_d_n2: f64 = (eq25_e651_d_n2 + eq25_e653_d_n2);
        let eq25_e654_d_n3: f64 = (eq25_e651_d_n3 + eq25_e653_d_n3);
        let eq25_e654_d_n4: f64 = (eq25_e651_d_n4 + eq25_e653_d_n4);
        let eq25_e654_d_n5: f64 = (eq25_e651_d_n5 + eq25_e653_d_n5);
        let eq25_e654_d_n6: f64 = (eq25_e651_d_n6 + eq25_e653_d_n6);
        let eq25_e654_d_n7: f64 = (eq25_e651_d_n7 + eq25_e653_d_n7);
        let eq25_e654_d_n8: f64 = (eq25_e651_d_n8 + eq25_e653_d_n8);
        let eq25_e654_d_n9: f64 = (eq25_e651_d_n9 + eq25_e653_d_n9);
        let eq25_e655: f64 = (p.p14 * eq25_e654);
        let eq25_e655_d_n0: f64 = (p.p14 * eq25_e654_d_n0);
        let eq25_e655_d_n1: f64 = (p.p14 * eq25_e654_d_n1);
        let eq25_e655_d_n2: f64 = (p.p14 * eq25_e654_d_n2);
        let eq25_e655_d_n3: f64 = (p.p14 * eq25_e654_d_n3);
        let eq25_e655_d_n4: f64 = (p.p14 * eq25_e654_d_n4);
        let eq25_e655_d_n5: f64 = (p.p14 * eq25_e654_d_n5);
        let eq25_e655_d_n6: f64 = (p.p14 * eq25_e654_d_n6);
        let eq25_e655_d_n7: f64 = (p.p14 * eq25_e654_d_n7);
        let eq25_e655_d_n8: f64 = (p.p14 * eq25_e654_d_n8);
        let eq25_e655_d_n9: f64 = (p.p14 * eq25_e654_d_n9);
        let eq25_value: f64 = eq25_e655;
        let eq25_node_derivatives: [f64; 10] = [eq25_e655_d_n0, eq25_e655_d_n1, eq25_e655_d_n2, eq25_e655_d_n3, eq25_e655_d_n4, eq25_e655_d_n5, eq25_e655_d_n6, eq25_e655_d_n7, eq25_e655_d_n8, eq25_e655_d_n9];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq25_value),
            &nodes,
            &eq25_node_derivatives,
            &branches,
            &eq25_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq26_e658: f64 = self.eval_ddt(8, s.v[370]);
        let eq26_e658_d_n0: f64 = self.ddt_jacobian(s.dn[370][0]);
        let eq26_e658_d_n1: f64 = self.ddt_jacobian(s.dn[370][1]);
        let eq26_e658_d_n2: f64 = self.ddt_jacobian(s.dn[370][2]);
        let eq26_e658_d_n3: f64 = self.ddt_jacobian(s.dn[370][3]);
        let eq26_e658_d_n4: f64 = self.ddt_jacobian(s.dn[370][4]);
        let eq26_e658_d_n5: f64 = self.ddt_jacobian(s.dn[370][5]);
        let eq26_e658_d_n6: f64 = self.ddt_jacobian(s.dn[370][6]);
        let eq26_e658_d_n7: f64 = self.ddt_jacobian(s.dn[370][7]);
        let eq26_e658_d_n8: f64 = self.ddt_jacobian(s.dn[370][8]);
        let eq26_e658_d_n9: f64 = self.ddt_jacobian(s.dn[370][9]);
        let eq26_e659: f64 = (p.p14 * eq26_e658);
        let eq26_e659_d_n0: f64 = (p.p14 * eq26_e658_d_n0);
        let eq26_e659_d_n1: f64 = (p.p14 * eq26_e658_d_n1);
        let eq26_e659_d_n2: f64 = (p.p14 * eq26_e658_d_n2);
        let eq26_e659_d_n3: f64 = (p.p14 * eq26_e658_d_n3);
        let eq26_e659_d_n4: f64 = (p.p14 * eq26_e658_d_n4);
        let eq26_e659_d_n5: f64 = (p.p14 * eq26_e658_d_n5);
        let eq26_e659_d_n6: f64 = (p.p14 * eq26_e658_d_n6);
        let eq26_e659_d_n7: f64 = (p.p14 * eq26_e658_d_n7);
        let eq26_e659_d_n8: f64 = (p.p14 * eq26_e658_d_n8);
        let eq26_e659_d_n9: f64 = (p.p14 * eq26_e658_d_n9);
        let eq26_value: f64 = eq26_e659;
        let eq26_node_derivatives: [f64; 10] = [eq26_e659_d_n0, eq26_e659_d_n1, eq26_e659_d_n2, eq26_e659_d_n3, eq26_e659_d_n4, eq26_e659_d_n5, eq26_e659_d_n6, eq26_e659_d_n7, eq26_e659_d_n8, eq26_e659_d_n9];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq26_value),
            &nodes,
            &eq26_node_derivatives,
            &branches,
            &eq26_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_27_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq27_e662: f64 = self.eval_ddt(9, s.v[357]);
        let eq27_e662_d_n0: f64 = self.ddt_jacobian(s.dn[357][0]);
        let eq27_e662_d_n1: f64 = self.ddt_jacobian(s.dn[357][1]);
        let eq27_e662_d_n2: f64 = self.ddt_jacobian(s.dn[357][2]);
        let eq27_e662_d_n3: f64 = self.ddt_jacobian(s.dn[357][3]);
        let eq27_e662_d_n4: f64 = self.ddt_jacobian(s.dn[357][4]);
        let eq27_e662_d_n5: f64 = self.ddt_jacobian(s.dn[357][5]);
        let eq27_e662_d_n6: f64 = self.ddt_jacobian(s.dn[357][6]);
        let eq27_e662_d_n7: f64 = self.ddt_jacobian(s.dn[357][7]);
        let eq27_e662_d_n8: f64 = self.ddt_jacobian(s.dn[357][8]);
        let eq27_e662_d_n9: f64 = self.ddt_jacobian(s.dn[357][9]);
        let eq27_e664: f64 = self.eval_ddt(10, s.v[366]);
        let eq27_e664_d_n0: f64 = self.ddt_jacobian(s.dn[366][0]);
        let eq27_e664_d_n1: f64 = self.ddt_jacobian(s.dn[366][1]);
        let eq27_e664_d_n2: f64 = self.ddt_jacobian(s.dn[366][2]);
        let eq27_e664_d_n3: f64 = self.ddt_jacobian(s.dn[366][3]);
        let eq27_e664_d_n4: f64 = self.ddt_jacobian(s.dn[366][4]);
        let eq27_e664_d_n5: f64 = self.ddt_jacobian(s.dn[366][5]);
        let eq27_e664_d_n6: f64 = self.ddt_jacobian(s.dn[366][6]);
        let eq27_e664_d_n7: f64 = self.ddt_jacobian(s.dn[366][7]);
        let eq27_e664_d_n8: f64 = self.ddt_jacobian(s.dn[366][8]);
        let eq27_e664_d_n9: f64 = self.ddt_jacobian(s.dn[366][9]);
        let eq27_e665: f64 = (eq27_e662 + eq27_e664);
        let eq27_e665_d_n0: f64 = (eq27_e662_d_n0 + eq27_e664_d_n0);
        let eq27_e665_d_n1: f64 = (eq27_e662_d_n1 + eq27_e664_d_n1);
        let eq27_e665_d_n2: f64 = (eq27_e662_d_n2 + eq27_e664_d_n2);
        let eq27_e665_d_n3: f64 = (eq27_e662_d_n3 + eq27_e664_d_n3);
        let eq27_e665_d_n4: f64 = (eq27_e662_d_n4 + eq27_e664_d_n4);
        let eq27_e665_d_n5: f64 = (eq27_e662_d_n5 + eq27_e664_d_n5);
        let eq27_e665_d_n6: f64 = (eq27_e662_d_n6 + eq27_e664_d_n6);
        let eq27_e665_d_n7: f64 = (eq27_e662_d_n7 + eq27_e664_d_n7);
        let eq27_e665_d_n8: f64 = (eq27_e662_d_n8 + eq27_e664_d_n8);
        let eq27_e665_d_n9: f64 = (eq27_e662_d_n9 + eq27_e664_d_n9);
        let eq27_e667: f64 = self.eval_ddt(11, s.v[368]);
        let eq27_e667_d_n0: f64 = self.ddt_jacobian(s.dn[368][0]);
        let eq27_e667_d_n1: f64 = self.ddt_jacobian(s.dn[368][1]);
        let eq27_e667_d_n2: f64 = self.ddt_jacobian(s.dn[368][2]);
        let eq27_e667_d_n3: f64 = self.ddt_jacobian(s.dn[368][3]);
        let eq27_e667_d_n4: f64 = self.ddt_jacobian(s.dn[368][4]);
        let eq27_e667_d_n5: f64 = self.ddt_jacobian(s.dn[368][5]);
        let eq27_e667_d_n6: f64 = self.ddt_jacobian(s.dn[368][6]);
        let eq27_e667_d_n7: f64 = self.ddt_jacobian(s.dn[368][7]);
        let eq27_e667_d_n8: f64 = self.ddt_jacobian(s.dn[368][8]);
        let eq27_e667_d_n9: f64 = self.ddt_jacobian(s.dn[368][9]);
        let eq27_e668: f64 = (eq27_e665 + eq27_e667);
        let eq27_e668_d_n0: f64 = (eq27_e665_d_n0 + eq27_e667_d_n0);
        let eq27_e668_d_n1: f64 = (eq27_e665_d_n1 + eq27_e667_d_n1);
        let eq27_e668_d_n2: f64 = (eq27_e665_d_n2 + eq27_e667_d_n2);
        let eq27_e668_d_n3: f64 = (eq27_e665_d_n3 + eq27_e667_d_n3);
        let eq27_e668_d_n4: f64 = (eq27_e665_d_n4 + eq27_e667_d_n4);
        let eq27_e668_d_n5: f64 = (eq27_e665_d_n5 + eq27_e667_d_n5);
        let eq27_e668_d_n6: f64 = (eq27_e665_d_n6 + eq27_e667_d_n6);
        let eq27_e668_d_n7: f64 = (eq27_e665_d_n7 + eq27_e667_d_n7);
        let eq27_e668_d_n8: f64 = (eq27_e665_d_n8 + eq27_e667_d_n8);
        let eq27_e668_d_n9: f64 = (eq27_e665_d_n9 + eq27_e667_d_n9);
        let eq27_e670: f64 = self.eval_ddt(12, s.v[375]);
        let eq27_e670_d_n0: f64 = self.ddt_jacobian(s.dn[375][0]);
        let eq27_e670_d_n1: f64 = self.ddt_jacobian(s.dn[375][1]);
        let eq27_e670_d_n2: f64 = self.ddt_jacobian(s.dn[375][2]);
        let eq27_e670_d_n3: f64 = self.ddt_jacobian(s.dn[375][3]);
        let eq27_e670_d_n4: f64 = self.ddt_jacobian(s.dn[375][4]);
        let eq27_e670_d_n5: f64 = self.ddt_jacobian(s.dn[375][5]);
        let eq27_e670_d_n6: f64 = self.ddt_jacobian(s.dn[375][6]);
        let eq27_e670_d_n7: f64 = self.ddt_jacobian(s.dn[375][7]);
        let eq27_e670_d_n8: f64 = self.ddt_jacobian(s.dn[375][8]);
        let eq27_e670_d_n9: f64 = self.ddt_jacobian(s.dn[375][9]);
        let eq27_e671: f64 = (eq27_e668 + eq27_e670);
        let eq27_e671_d_n0: f64 = (eq27_e668_d_n0 + eq27_e670_d_n0);
        let eq27_e671_d_n1: f64 = (eq27_e668_d_n1 + eq27_e670_d_n1);
        let eq27_e671_d_n2: f64 = (eq27_e668_d_n2 + eq27_e670_d_n2);
        let eq27_e671_d_n3: f64 = (eq27_e668_d_n3 + eq27_e670_d_n3);
        let eq27_e671_d_n4: f64 = (eq27_e668_d_n4 + eq27_e670_d_n4);
        let eq27_e671_d_n5: f64 = (eq27_e668_d_n5 + eq27_e670_d_n5);
        let eq27_e671_d_n6: f64 = (eq27_e668_d_n6 + eq27_e670_d_n6);
        let eq27_e671_d_n7: f64 = (eq27_e668_d_n7 + eq27_e670_d_n7);
        let eq27_e671_d_n8: f64 = (eq27_e668_d_n8 + eq27_e670_d_n8);
        let eq27_e671_d_n9: f64 = (eq27_e668_d_n9 + eq27_e670_d_n9);
        let eq27_e672: f64 = (p.p14 * eq27_e671);
        let eq27_e672_d_n0: f64 = (p.p14 * eq27_e671_d_n0);
        let eq27_e672_d_n1: f64 = (p.p14 * eq27_e671_d_n1);
        let eq27_e672_d_n2: f64 = (p.p14 * eq27_e671_d_n2);
        let eq27_e672_d_n3: f64 = (p.p14 * eq27_e671_d_n3);
        let eq27_e672_d_n4: f64 = (p.p14 * eq27_e671_d_n4);
        let eq27_e672_d_n5: f64 = (p.p14 * eq27_e671_d_n5);
        let eq27_e672_d_n6: f64 = (p.p14 * eq27_e671_d_n6);
        let eq27_e672_d_n7: f64 = (p.p14 * eq27_e671_d_n7);
        let eq27_e672_d_n8: f64 = (p.p14 * eq27_e671_d_n8);
        let eq27_e672_d_n9: f64 = (p.p14 * eq27_e671_d_n9);
        let eq27_value: f64 = eq27_e672;
        let eq27_node_derivatives: [f64; 10] = [eq27_e672_d_n0, eq27_e672_d_n1, eq27_e672_d_n2, eq27_e672_d_n3, eq27_e672_d_n4, eq27_e672_d_n5, eq27_e672_d_n6, eq27_e672_d_n7, eq27_e672_d_n8, eq27_e672_d_n9];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            self.multiplicity * (eq27_value),
            &nodes,
            &eq27_node_derivatives,
            &branches,
            &eq27_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_28_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq28_e675: f64 = self.eval_ddt(13, s.v[359]);
        let eq28_e675_d_n0: f64 = self.ddt_jacobian(s.dn[359][0]);
        let eq28_e675_d_n1: f64 = self.ddt_jacobian(s.dn[359][1]);
        let eq28_e675_d_n2: f64 = self.ddt_jacobian(s.dn[359][2]);
        let eq28_e675_d_n3: f64 = self.ddt_jacobian(s.dn[359][3]);
        let eq28_e675_d_n4: f64 = self.ddt_jacobian(s.dn[359][4]);
        let eq28_e675_d_n5: f64 = self.ddt_jacobian(s.dn[359][5]);
        let eq28_e675_d_n6: f64 = self.ddt_jacobian(s.dn[359][6]);
        let eq28_e675_d_n7: f64 = self.ddt_jacobian(s.dn[359][7]);
        let eq28_e675_d_n8: f64 = self.ddt_jacobian(s.dn[359][8]);
        let eq28_e675_d_n9: f64 = self.ddt_jacobian(s.dn[359][9]);
        let eq28_e677: f64 = self.eval_ddt(14, s.v[371]);
        let eq28_e677_d_n0: f64 = self.ddt_jacobian(s.dn[371][0]);
        let eq28_e677_d_n1: f64 = self.ddt_jacobian(s.dn[371][1]);
        let eq28_e677_d_n2: f64 = self.ddt_jacobian(s.dn[371][2]);
        let eq28_e677_d_n3: f64 = self.ddt_jacobian(s.dn[371][3]);
        let eq28_e677_d_n4: f64 = self.ddt_jacobian(s.dn[371][4]);
        let eq28_e677_d_n5: f64 = self.ddt_jacobian(s.dn[371][5]);
        let eq28_e677_d_n6: f64 = self.ddt_jacobian(s.dn[371][6]);
        let eq28_e677_d_n7: f64 = self.ddt_jacobian(s.dn[371][7]);
        let eq28_e677_d_n8: f64 = self.ddt_jacobian(s.dn[371][8]);
        let eq28_e677_d_n9: f64 = self.ddt_jacobian(s.dn[371][9]);
        let eq28_e678: f64 = (eq28_e675 + eq28_e677);
        let eq28_e678_d_n0: f64 = (eq28_e675_d_n0 + eq28_e677_d_n0);
        let eq28_e678_d_n1: f64 = (eq28_e675_d_n1 + eq28_e677_d_n1);
        let eq28_e678_d_n2: f64 = (eq28_e675_d_n2 + eq28_e677_d_n2);
        let eq28_e678_d_n3: f64 = (eq28_e675_d_n3 + eq28_e677_d_n3);
        let eq28_e678_d_n4: f64 = (eq28_e675_d_n4 + eq28_e677_d_n4);
        let eq28_e678_d_n5: f64 = (eq28_e675_d_n5 + eq28_e677_d_n5);
        let eq28_e678_d_n6: f64 = (eq28_e675_d_n6 + eq28_e677_d_n6);
        let eq28_e678_d_n7: f64 = (eq28_e675_d_n7 + eq28_e677_d_n7);
        let eq28_e678_d_n8: f64 = (eq28_e675_d_n8 + eq28_e677_d_n8);
        let eq28_e678_d_n9: f64 = (eq28_e675_d_n9 + eq28_e677_d_n9);
        let eq28_e679: f64 = (p.p14 * eq28_e678);
        let eq28_e679_d_n0: f64 = (p.p14 * eq28_e678_d_n0);
        let eq28_e679_d_n1: f64 = (p.p14 * eq28_e678_d_n1);
        let eq28_e679_d_n2: f64 = (p.p14 * eq28_e678_d_n2);
        let eq28_e679_d_n3: f64 = (p.p14 * eq28_e678_d_n3);
        let eq28_e679_d_n4: f64 = (p.p14 * eq28_e678_d_n4);
        let eq28_e679_d_n5: f64 = (p.p14 * eq28_e678_d_n5);
        let eq28_e679_d_n6: f64 = (p.p14 * eq28_e678_d_n6);
        let eq28_e679_d_n7: f64 = (p.p14 * eq28_e678_d_n7);
        let eq28_e679_d_n8: f64 = (p.p14 * eq28_e678_d_n8);
        let eq28_e679_d_n9: f64 = (p.p14 * eq28_e678_d_n9);
        let eq28_value: f64 = eq28_e679;
        let eq28_node_derivatives: [f64; 10] = [eq28_e679_d_n0, eq28_e679_d_n1, eq28_e679_d_n2, eq28_e679_d_n3, eq28_e679_d_n4, eq28_e679_d_n5, eq28_e679_d_n6, eq28_e679_d_n7, eq28_e679_d_n8, eq28_e679_d_n9];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq28_value),
            &nodes,
            &eq28_node_derivatives,
            &branches,
            &eq28_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_29_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq29_e681: f64 = self.eval_ddt(15, s.v[374]);
        let eq29_e681_d_n0: f64 = self.ddt_jacobian(s.dn[374][0]);
        let eq29_e681_d_n1: f64 = self.ddt_jacobian(s.dn[374][1]);
        let eq29_e681_d_n2: f64 = self.ddt_jacobian(s.dn[374][2]);
        let eq29_e681_d_n3: f64 = self.ddt_jacobian(s.dn[374][3]);
        let eq29_e681_d_n4: f64 = self.ddt_jacobian(s.dn[374][4]);
        let eq29_e681_d_n5: f64 = self.ddt_jacobian(s.dn[374][5]);
        let eq29_e681_d_n6: f64 = self.ddt_jacobian(s.dn[374][6]);
        let eq29_e681_d_n7: f64 = self.ddt_jacobian(s.dn[374][7]);
        let eq29_e681_d_n8: f64 = self.ddt_jacobian(s.dn[374][8]);
        let eq29_e681_d_n9: f64 = self.ddt_jacobian(s.dn[374][9]);
        let eq29_value: f64 = eq29_e681;
        let eq29_node_derivatives: [f64; 10] = [eq29_e681_d_n0, eq29_e681_d_n1, eq29_e681_d_n2, eq29_e681_d_n3, eq29_e681_d_n4, eq29_e681_d_n5, eq29_e681_d_n6, eq29_e681_d_n7, eq29_e681_d_n8, eq29_e681_d_n9];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq29_value),
            &nodes,
            &eq29_node_derivatives,
            &branches,
            &eq29_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_31_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq31_e687: f64 = (s.v[1793] * (nv5 - 0.0));
        let eq31_e687_d_n0: f64 = (s.dn[1793][0] * (nv5 - 0.0));
        let eq31_e687_d_n1: f64 = (s.dn[1793][1] * (nv5 - 0.0));
        let eq31_e687_d_n2: f64 = (s.dn[1793][2] * (nv5 - 0.0));
        let eq31_e687_d_n3: f64 = (s.dn[1793][3] * (nv5 - 0.0));
        let eq31_e687_d_n4: f64 = (s.dn[1793][4] * (nv5 - 0.0));
        let eq31_e687_d_n5: f64 = ((s.dn[1793][5] * (nv5 - 0.0)) + s.v[1793]);
        let eq31_e687_d_n6: f64 = (s.dn[1793][6] * (nv5 - 0.0));
        let eq31_e687_d_n7: f64 = (s.dn[1793][7] * (nv5 - 0.0));
        let eq31_e687_d_n8: f64 = (s.dn[1793][8] * (nv5 - 0.0));
        let eq31_e687_d_n9: f64 = (s.dn[1793][9] * (nv5 - 0.0));
        let eq31_value: f64 = eq31_e687;
        let eq31_node_derivatives: [f64; 10] = [eq31_e687_d_n0, eq31_e687_d_n1, eq31_e687_d_n2, eq31_e687_d_n3, eq31_e687_d_n4, eq31_e687_d_n5, eq31_e687_d_n6, eq31_e687_d_n7, eq31_e687_d_n8, eq31_e687_d_n9];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq31_value),
            &nodes,
            &eq31_node_derivatives,
            &branches,
            &eq31_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_32_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq32_e690: f64 = (s.v[1790] * (nv5 - 0.0));
        let eq32_e690_d_n0: f64 = (s.dn[1790][0] * (nv5 - 0.0));
        let eq32_e690_d_n1: f64 = (s.dn[1790][1] * (nv5 - 0.0));
        let eq32_e690_d_n2: f64 = (s.dn[1790][2] * (nv5 - 0.0));
        let eq32_e690_d_n3: f64 = (s.dn[1790][3] * (nv5 - 0.0));
        let eq32_e690_d_n4: f64 = (s.dn[1790][4] * (nv5 - 0.0));
        let eq32_e690_d_n5: f64 = ((s.dn[1790][5] * (nv5 - 0.0)) + s.v[1790]);
        let eq32_e690_d_n6: f64 = (s.dn[1790][6] * (nv5 - 0.0));
        let eq32_e690_d_n7: f64 = (s.dn[1790][7] * (nv5 - 0.0));
        let eq32_e690_d_n8: f64 = (s.dn[1790][8] * (nv5 - 0.0));
        let eq32_e690_d_n9: f64 = (s.dn[1790][9] * (nv5 - 0.0));
        let eq32_e691: f64 = self.eval_ddt(16, eq32_e690);
        let eq32_e691_d_n0: f64 = self.ddt_jacobian(eq32_e690_d_n0);
        let eq32_e691_d_n1: f64 = self.ddt_jacobian(eq32_e690_d_n1);
        let eq32_e691_d_n2: f64 = self.ddt_jacobian(eq32_e690_d_n2);
        let eq32_e691_d_n3: f64 = self.ddt_jacobian(eq32_e690_d_n3);
        let eq32_e691_d_n4: f64 = self.ddt_jacobian(eq32_e690_d_n4);
        let eq32_e691_d_n5: f64 = self.ddt_jacobian(eq32_e690_d_n5);
        let eq32_e691_d_n6: f64 = self.ddt_jacobian(eq32_e690_d_n6);
        let eq32_e691_d_n7: f64 = self.ddt_jacobian(eq32_e690_d_n7);
        let eq32_e691_d_n8: f64 = self.ddt_jacobian(eq32_e690_d_n8);
        let eq32_e691_d_n9: f64 = self.ddt_jacobian(eq32_e690_d_n9);
        let eq32_value: f64 = eq32_e691;
        let eq32_node_derivatives: [f64; 10] = [eq32_e691_d_n0, eq32_e691_d_n1, eq32_e691_d_n2, eq32_e691_d_n3, eq32_e691_d_n4, eq32_e691_d_n5, eq32_e691_d_n6, eq32_e691_d_n7, eq32_e691_d_n8, eq32_e691_d_n9];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq32_value),
            &nodes,
            &eq32_node_derivatives,
            &branches,
            &eq32_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq33_e693: f64 = (-s.v[1791]);
        let eq33_e693_d_n0: f64 = (-s.dn[1791][0]);
        let eq33_e693_d_n1: f64 = (-s.dn[1791][1]);
        let eq33_e693_d_n2: f64 = (-s.dn[1791][2]);
        let eq33_e693_d_n3: f64 = (-s.dn[1791][3]);
        let eq33_e693_d_n4: f64 = (-s.dn[1791][4]);
        let eq33_e693_d_n5: f64 = (-s.dn[1791][5]);
        let eq33_e693_d_n6: f64 = (-s.dn[1791][6]);
        let eq33_e693_d_n7: f64 = (-s.dn[1791][7]);
        let eq33_e693_d_n8: f64 = (-s.dn[1791][8]);
        let eq33_e693_d_n9: f64 = (-s.dn[1791][9]);
        let eq33_e695: f64 = (eq33_e693 * (nv5 - 0.0));
        let eq33_e695_d_n0: f64 = (eq33_e693_d_n0 * (nv5 - 0.0));
        let eq33_e695_d_n1: f64 = (eq33_e693_d_n1 * (nv5 - 0.0));
        let eq33_e695_d_n2: f64 = (eq33_e693_d_n2 * (nv5 - 0.0));
        let eq33_e695_d_n3: f64 = (eq33_e693_d_n3 * (nv5 - 0.0));
        let eq33_e695_d_n4: f64 = (eq33_e693_d_n4 * (nv5 - 0.0));
        let eq33_e695_d_n5: f64 = ((eq33_e693_d_n5 * (nv5 - 0.0)) + eq33_e693);
        let eq33_e695_d_n6: f64 = (eq33_e693_d_n6 * (nv5 - 0.0));
        let eq33_e695_d_n7: f64 = (eq33_e693_d_n7 * (nv5 - 0.0));
        let eq33_e695_d_n8: f64 = (eq33_e693_d_n8 * (nv5 - 0.0));
        let eq33_e695_d_n9: f64 = (eq33_e693_d_n9 * (nv5 - 0.0));
        let eq33_e696: f64 = self.eval_ddt(17, eq33_e695);
        let eq33_e696_d_n0: f64 = self.ddt_jacobian(eq33_e695_d_n0);
        let eq33_e696_d_n1: f64 = self.ddt_jacobian(eq33_e695_d_n1);
        let eq33_e696_d_n2: f64 = self.ddt_jacobian(eq33_e695_d_n2);
        let eq33_e696_d_n3: f64 = self.ddt_jacobian(eq33_e695_d_n3);
        let eq33_e696_d_n4: f64 = self.ddt_jacobian(eq33_e695_d_n4);
        let eq33_e696_d_n5: f64 = self.ddt_jacobian(eq33_e695_d_n5);
        let eq33_e696_d_n6: f64 = self.ddt_jacobian(eq33_e695_d_n6);
        let eq33_e696_d_n7: f64 = self.ddt_jacobian(eq33_e695_d_n7);
        let eq33_e696_d_n8: f64 = self.ddt_jacobian(eq33_e695_d_n8);
        let eq33_e696_d_n9: f64 = self.ddt_jacobian(eq33_e695_d_n9);
        let eq33_value: f64 = eq33_e696;
        let eq33_node_derivatives: [f64; 10] = [eq33_e696_d_n0, eq33_e696_d_n1, eq33_e696_d_n2, eq33_e696_d_n3, eq33_e696_d_n4, eq33_e696_d_n5, eq33_e696_d_n6, eq33_e696_d_n7, eq33_e696_d_n8, eq33_e696_d_n9];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            self.multiplicity * (eq33_value),
            &nodes,
            &eq33_node_derivatives,
            &branches,
            &eq33_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq34_e698: f64 = (-s.v[1792]);
        let eq34_e698_d_n0: f64 = (-s.dn[1792][0]);
        let eq34_e698_d_n1: f64 = (-s.dn[1792][1]);
        let eq34_e698_d_n2: f64 = (-s.dn[1792][2]);
        let eq34_e698_d_n3: f64 = (-s.dn[1792][3]);
        let eq34_e698_d_n4: f64 = (-s.dn[1792][4]);
        let eq34_e698_d_n5: f64 = (-s.dn[1792][5]);
        let eq34_e698_d_n6: f64 = (-s.dn[1792][6]);
        let eq34_e698_d_n7: f64 = (-s.dn[1792][7]);
        let eq34_e698_d_n8: f64 = (-s.dn[1792][8]);
        let eq34_e698_d_n9: f64 = (-s.dn[1792][9]);
        let eq34_e700: f64 = (eq34_e698 * (nv5 - 0.0));
        let eq34_e700_d_n0: f64 = (eq34_e698_d_n0 * (nv5 - 0.0));
        let eq34_e700_d_n1: f64 = (eq34_e698_d_n1 * (nv5 - 0.0));
        let eq34_e700_d_n2: f64 = (eq34_e698_d_n2 * (nv5 - 0.0));
        let eq34_e700_d_n3: f64 = (eq34_e698_d_n3 * (nv5 - 0.0));
        let eq34_e700_d_n4: f64 = (eq34_e698_d_n4 * (nv5 - 0.0));
        let eq34_e700_d_n5: f64 = ((eq34_e698_d_n5 * (nv5 - 0.0)) + eq34_e698);
        let eq34_e700_d_n6: f64 = (eq34_e698_d_n6 * (nv5 - 0.0));
        let eq34_e700_d_n7: f64 = (eq34_e698_d_n7 * (nv5 - 0.0));
        let eq34_e700_d_n8: f64 = (eq34_e698_d_n8 * (nv5 - 0.0));
        let eq34_e700_d_n9: f64 = (eq34_e698_d_n9 * (nv5 - 0.0));
        let eq34_e701: f64 = self.eval_ddt(18, eq34_e700);
        let eq34_e701_d_n0: f64 = self.ddt_jacobian(eq34_e700_d_n0);
        let eq34_e701_d_n1: f64 = self.ddt_jacobian(eq34_e700_d_n1);
        let eq34_e701_d_n2: f64 = self.ddt_jacobian(eq34_e700_d_n2);
        let eq34_e701_d_n3: f64 = self.ddt_jacobian(eq34_e700_d_n3);
        let eq34_e701_d_n4: f64 = self.ddt_jacobian(eq34_e700_d_n4);
        let eq34_e701_d_n5: f64 = self.ddt_jacobian(eq34_e700_d_n5);
        let eq34_e701_d_n6: f64 = self.ddt_jacobian(eq34_e700_d_n6);
        let eq34_e701_d_n7: f64 = self.ddt_jacobian(eq34_e700_d_n7);
        let eq34_e701_d_n8: f64 = self.ddt_jacobian(eq34_e700_d_n8);
        let eq34_e701_d_n9: f64 = self.ddt_jacobian(eq34_e700_d_n9);
        let eq34_value: f64 = eq34_e701;
        let eq34_node_derivatives: [f64; 10] = [eq34_e701_d_n0, eq34_e701_d_n1, eq34_e701_d_n2, eq34_e701_d_n3, eq34_e701_d_n4, eq34_e701_d_n5, eq34_e701_d_n6, eq34_e701_d_n7, eq34_e701_d_n8, eq34_e701_d_n9];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq34_value),
            &nodes,
            &eq34_node_derivatives,
            &branches,
            &eq34_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq35_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq35_value),
            &[
            ],
        );
    }
}
