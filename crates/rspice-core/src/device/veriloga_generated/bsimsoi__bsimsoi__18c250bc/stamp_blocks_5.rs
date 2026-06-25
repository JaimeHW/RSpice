#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_42_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq42_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq42_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_43_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq43_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[5]),
            self.multiplicity * (eq43_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_44_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq44_e1647: f64 = (p.p33 * s.v[92]);
        let eq44_e1647_d_n0: f64 = (p.p33 * s.dn[92][0]);
        let eq44_e1647_d_n1: f64 = (p.p33 * s.dn[92][1]);
        let eq44_e1647_d_n2: f64 = (p.p33 * s.dn[92][2]);
        let eq44_e1647_d_n3: f64 = (p.p33 * s.dn[92][3]);
        let eq44_e1647_d_n4: f64 = (p.p33 * s.dn[92][4]);
        let eq44_e1647_d_n5: f64 = (p.p33 * s.dn[92][5]);
        let eq44_e1647_d_n6: f64 = (p.p33 * s.dn[92][6]);
        let eq44_e1647_d_n7: f64 = (p.p33 * s.dn[92][7]);
        let eq44_e1647_d_n8: f64 = (p.p33 * s.dn[92][8]);
        let eq44_e1647_d_n9: f64 = (p.p33 * s.dn[92][9]);
        let eq44_e1647_d_n10: f64 = (p.p33 * s.dn[92][10]);
        let eq44_e1647_d_n11: f64 = (p.p33 * s.dn[92][11]);
        let eq44_e1647_d_n12: f64 = (p.p33 * s.dn[92][12]);
        let eq44_e1647_d_n13: f64 = (p.p33 * s.dn[92][13]);
        let eq44_e1648: f64 = self.eval_ddt(2, eq44_e1647);
        let eq44_e1648_d_n0: f64 = self.ddt_jacobian(eq44_e1647_d_n0);
        let eq44_e1648_d_n1: f64 = self.ddt_jacobian(eq44_e1647_d_n1);
        let eq44_e1648_d_n2: f64 = self.ddt_jacobian(eq44_e1647_d_n2);
        let eq44_e1648_d_n3: f64 = self.ddt_jacobian(eq44_e1647_d_n3);
        let eq44_e1648_d_n4: f64 = self.ddt_jacobian(eq44_e1647_d_n4);
        let eq44_e1648_d_n5: f64 = self.ddt_jacobian(eq44_e1647_d_n5);
        let eq44_e1648_d_n6: f64 = self.ddt_jacobian(eq44_e1647_d_n6);
        let eq44_e1648_d_n7: f64 = self.ddt_jacobian(eq44_e1647_d_n7);
        let eq44_e1648_d_n8: f64 = self.ddt_jacobian(eq44_e1647_d_n8);
        let eq44_e1648_d_n9: f64 = self.ddt_jacobian(eq44_e1647_d_n9);
        let eq44_e1648_d_n10: f64 = self.ddt_jacobian(eq44_e1647_d_n10);
        let eq44_e1648_d_n11: f64 = self.ddt_jacobian(eq44_e1647_d_n11);
        let eq44_e1648_d_n12: f64 = self.ddt_jacobian(eq44_e1647_d_n12);
        let eq44_e1648_d_n13: f64 = self.ddt_jacobian(eq44_e1647_d_n13);
        let eq44_value: f64 = eq44_e1648;
        let eq44_node_derivatives: [f64; 14] = [eq44_e1648_d_n0, eq44_e1648_d_n1, eq44_e1648_d_n2, eq44_e1648_d_n3, eq44_e1648_d_n4, eq44_e1648_d_n5, eq44_e1648_d_n6, eq44_e1648_d_n7, eq44_e1648_d_n8, eq44_e1648_d_n9, eq44_e1648_d_n10, eq44_e1648_d_n11, eq44_e1648_d_n12, eq44_e1648_d_n13];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            self.multiplicity * (eq44_value),
            &nodes,
            &eq44_node_derivatives,
            &branches,
            &eq44_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_45_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq45_e1651: f64 = (p.p33 * s.v[93]);
        let eq45_e1651_d_n0: f64 = (p.p33 * s.dn[93][0]);
        let eq45_e1651_d_n1: f64 = (p.p33 * s.dn[93][1]);
        let eq45_e1651_d_n2: f64 = (p.p33 * s.dn[93][2]);
        let eq45_e1651_d_n3: f64 = (p.p33 * s.dn[93][3]);
        let eq45_e1651_d_n4: f64 = (p.p33 * s.dn[93][4]);
        let eq45_e1651_d_n5: f64 = (p.p33 * s.dn[93][5]);
        let eq45_e1651_d_n6: f64 = (p.p33 * s.dn[93][6]);
        let eq45_e1651_d_n7: f64 = (p.p33 * s.dn[93][7]);
        let eq45_e1651_d_n8: f64 = (p.p33 * s.dn[93][8]);
        let eq45_e1651_d_n9: f64 = (p.p33 * s.dn[93][9]);
        let eq45_e1651_d_n10: f64 = (p.p33 * s.dn[93][10]);
        let eq45_e1651_d_n11: f64 = (p.p33 * s.dn[93][11]);
        let eq45_e1651_d_n12: f64 = (p.p33 * s.dn[93][12]);
        let eq45_e1651_d_n13: f64 = (p.p33 * s.dn[93][13]);
        let eq45_e1652: f64 = self.eval_ddt(3, eq45_e1651);
        let eq45_e1652_d_n0: f64 = self.ddt_jacobian(eq45_e1651_d_n0);
        let eq45_e1652_d_n1: f64 = self.ddt_jacobian(eq45_e1651_d_n1);
        let eq45_e1652_d_n2: f64 = self.ddt_jacobian(eq45_e1651_d_n2);
        let eq45_e1652_d_n3: f64 = self.ddt_jacobian(eq45_e1651_d_n3);
        let eq45_e1652_d_n4: f64 = self.ddt_jacobian(eq45_e1651_d_n4);
        let eq45_e1652_d_n5: f64 = self.ddt_jacobian(eq45_e1651_d_n5);
        let eq45_e1652_d_n6: f64 = self.ddt_jacobian(eq45_e1651_d_n6);
        let eq45_e1652_d_n7: f64 = self.ddt_jacobian(eq45_e1651_d_n7);
        let eq45_e1652_d_n8: f64 = self.ddt_jacobian(eq45_e1651_d_n8);
        let eq45_e1652_d_n9: f64 = self.ddt_jacobian(eq45_e1651_d_n9);
        let eq45_e1652_d_n10: f64 = self.ddt_jacobian(eq45_e1651_d_n10);
        let eq45_e1652_d_n11: f64 = self.ddt_jacobian(eq45_e1651_d_n11);
        let eq45_e1652_d_n12: f64 = self.ddt_jacobian(eq45_e1651_d_n12);
        let eq45_e1652_d_n13: f64 = self.ddt_jacobian(eq45_e1651_d_n13);
        let eq45_value: f64 = eq45_e1652;
        let eq45_node_derivatives: [f64; 14] = [eq45_e1652_d_n0, eq45_e1652_d_n1, eq45_e1652_d_n2, eq45_e1652_d_n3, eq45_e1652_d_n4, eq45_e1652_d_n5, eq45_e1652_d_n6, eq45_e1652_d_n7, eq45_e1652_d_n8, eq45_e1652_d_n9, eq45_e1652_d_n10, eq45_e1652_d_n11, eq45_e1652_d_n12, eq45_e1652_d_n13];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq45_value),
            &nodes,
            &eq45_node_derivatives,
            &branches,
            &eq45_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq46_e1656: f64 = (p.p33 * s.v[916]);
        let eq46_e1656_d_n0: f64 = (p.p33 * s.dn[916][0]);
        let eq46_e1656_d_n1: f64 = (p.p33 * s.dn[916][1]);
        let eq46_e1656_d_n2: f64 = (p.p33 * s.dn[916][2]);
        let eq46_e1656_d_n3: f64 = (p.p33 * s.dn[916][3]);
        let eq46_e1656_d_n4: f64 = (p.p33 * s.dn[916][4]);
        let eq46_e1656_d_n5: f64 = (p.p33 * s.dn[916][5]);
        let eq46_e1656_d_n6: f64 = (p.p33 * s.dn[916][6]);
        let eq46_e1656_d_n7: f64 = (p.p33 * s.dn[916][7]);
        let eq46_e1656_d_n8: f64 = (p.p33 * s.dn[916][8]);
        let eq46_e1656_d_n9: f64 = (p.p33 * s.dn[916][9]);
        let eq46_e1656_d_n10: f64 = (p.p33 * s.dn[916][10]);
        let eq46_e1656_d_n11: f64 = (p.p33 * s.dn[916][11]);
        let eq46_e1656_d_n12: f64 = (p.p33 * s.dn[916][12]);
        let eq46_e1656_d_n13: f64 = (p.p33 * s.dn[916][13]);
        let eq46_e1657: f64 = self.eval_ddt(4, eq46_e1656);
        let eq46_e1657_d_n0: f64 = self.ddt_jacobian(eq46_e1656_d_n0);
        let eq46_e1657_d_n1: f64 = self.ddt_jacobian(eq46_e1656_d_n1);
        let eq46_e1657_d_n2: f64 = self.ddt_jacobian(eq46_e1656_d_n2);
        let eq46_e1657_d_n3: f64 = self.ddt_jacobian(eq46_e1656_d_n3);
        let eq46_e1657_d_n4: f64 = self.ddt_jacobian(eq46_e1656_d_n4);
        let eq46_e1657_d_n5: f64 = self.ddt_jacobian(eq46_e1656_d_n5);
        let eq46_e1657_d_n6: f64 = self.ddt_jacobian(eq46_e1656_d_n6);
        let eq46_e1657_d_n7: f64 = self.ddt_jacobian(eq46_e1656_d_n7);
        let eq46_e1657_d_n8: f64 = self.ddt_jacobian(eq46_e1656_d_n8);
        let eq46_e1657_d_n9: f64 = self.ddt_jacobian(eq46_e1656_d_n9);
        let eq46_e1657_d_n10: f64 = self.ddt_jacobian(eq46_e1656_d_n10);
        let eq46_e1657_d_n11: f64 = self.ddt_jacobian(eq46_e1656_d_n11);
        let eq46_e1657_d_n12: f64 = self.ddt_jacobian(eq46_e1656_d_n12);
        let eq46_e1657_d_n13: f64 = self.ddt_jacobian(eq46_e1656_d_n13);
        let eq46_e1658: f64 = (p.p37 * eq46_e1657);
        let eq46_e1658_d_n0: f64 = (p.p37 * eq46_e1657_d_n0);
        let eq46_e1658_d_n1: f64 = (p.p37 * eq46_e1657_d_n1);
        let eq46_e1658_d_n2: f64 = (p.p37 * eq46_e1657_d_n2);
        let eq46_e1658_d_n3: f64 = (p.p37 * eq46_e1657_d_n3);
        let eq46_e1658_d_n4: f64 = (p.p37 * eq46_e1657_d_n4);
        let eq46_e1658_d_n5: f64 = (p.p37 * eq46_e1657_d_n5);
        let eq46_e1658_d_n6: f64 = (p.p37 * eq46_e1657_d_n6);
        let eq46_e1658_d_n7: f64 = (p.p37 * eq46_e1657_d_n7);
        let eq46_e1658_d_n8: f64 = (p.p37 * eq46_e1657_d_n8);
        let eq46_e1658_d_n9: f64 = (p.p37 * eq46_e1657_d_n9);
        let eq46_e1658_d_n10: f64 = (p.p37 * eq46_e1657_d_n10);
        let eq46_e1658_d_n11: f64 = (p.p37 * eq46_e1657_d_n11);
        let eq46_e1658_d_n12: f64 = (p.p37 * eq46_e1657_d_n12);
        let eq46_e1658_d_n13: f64 = (p.p37 * eq46_e1657_d_n13);
        let eq46_value: f64 = eq46_e1658;
        let eq46_node_derivatives: [f64; 14] = [eq46_e1658_d_n0, eq46_e1658_d_n1, eq46_e1658_d_n2, eq46_e1658_d_n3, eq46_e1658_d_n4, eq46_e1658_d_n5, eq46_e1658_d_n6, eq46_e1658_d_n7, eq46_e1658_d_n8, eq46_e1658_d_n9, eq46_e1658_d_n10, eq46_e1658_d_n11, eq46_e1658_d_n12, eq46_e1658_d_n13];
        let eq46_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            self.multiplicity * (eq46_value),
            &nodes,
            &eq46_node_derivatives,
            &branches,
            &eq46_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_47_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq47_e1662: f64 = (p.p33 * s.v[920]);
        let eq47_e1662_d_n0: f64 = (p.p33 * s.dn[920][0]);
        let eq47_e1662_d_n1: f64 = (p.p33 * s.dn[920][1]);
        let eq47_e1662_d_n2: f64 = (p.p33 * s.dn[920][2]);
        let eq47_e1662_d_n3: f64 = (p.p33 * s.dn[920][3]);
        let eq47_e1662_d_n4: f64 = (p.p33 * s.dn[920][4]);
        let eq47_e1662_d_n5: f64 = (p.p33 * s.dn[920][5]);
        let eq47_e1662_d_n6: f64 = (p.p33 * s.dn[920][6]);
        let eq47_e1662_d_n7: f64 = (p.p33 * s.dn[920][7]);
        let eq47_e1662_d_n8: f64 = (p.p33 * s.dn[920][8]);
        let eq47_e1662_d_n9: f64 = (p.p33 * s.dn[920][9]);
        let eq47_e1662_d_n10: f64 = (p.p33 * s.dn[920][10]);
        let eq47_e1662_d_n11: f64 = (p.p33 * s.dn[920][11]);
        let eq47_e1662_d_n12: f64 = (p.p33 * s.dn[920][12]);
        let eq47_e1662_d_n13: f64 = (p.p33 * s.dn[920][13]);
        let eq47_e1663: f64 = self.eval_ddt(5, eq47_e1662);
        let eq47_e1663_d_n0: f64 = self.ddt_jacobian(eq47_e1662_d_n0);
        let eq47_e1663_d_n1: f64 = self.ddt_jacobian(eq47_e1662_d_n1);
        let eq47_e1663_d_n2: f64 = self.ddt_jacobian(eq47_e1662_d_n2);
        let eq47_e1663_d_n3: f64 = self.ddt_jacobian(eq47_e1662_d_n3);
        let eq47_e1663_d_n4: f64 = self.ddt_jacobian(eq47_e1662_d_n4);
        let eq47_e1663_d_n5: f64 = self.ddt_jacobian(eq47_e1662_d_n5);
        let eq47_e1663_d_n6: f64 = self.ddt_jacobian(eq47_e1662_d_n6);
        let eq47_e1663_d_n7: f64 = self.ddt_jacobian(eq47_e1662_d_n7);
        let eq47_e1663_d_n8: f64 = self.ddt_jacobian(eq47_e1662_d_n8);
        let eq47_e1663_d_n9: f64 = self.ddt_jacobian(eq47_e1662_d_n9);
        let eq47_e1663_d_n10: f64 = self.ddt_jacobian(eq47_e1662_d_n10);
        let eq47_e1663_d_n11: f64 = self.ddt_jacobian(eq47_e1662_d_n11);
        let eq47_e1663_d_n12: f64 = self.ddt_jacobian(eq47_e1662_d_n12);
        let eq47_e1663_d_n13: f64 = self.ddt_jacobian(eq47_e1662_d_n13);
        let eq47_e1664: f64 = (p.p37 * eq47_e1663);
        let eq47_e1664_d_n0: f64 = (p.p37 * eq47_e1663_d_n0);
        let eq47_e1664_d_n1: f64 = (p.p37 * eq47_e1663_d_n1);
        let eq47_e1664_d_n2: f64 = (p.p37 * eq47_e1663_d_n2);
        let eq47_e1664_d_n3: f64 = (p.p37 * eq47_e1663_d_n3);
        let eq47_e1664_d_n4: f64 = (p.p37 * eq47_e1663_d_n4);
        let eq47_e1664_d_n5: f64 = (p.p37 * eq47_e1663_d_n5);
        let eq47_e1664_d_n6: f64 = (p.p37 * eq47_e1663_d_n6);
        let eq47_e1664_d_n7: f64 = (p.p37 * eq47_e1663_d_n7);
        let eq47_e1664_d_n8: f64 = (p.p37 * eq47_e1663_d_n8);
        let eq47_e1664_d_n9: f64 = (p.p37 * eq47_e1663_d_n9);
        let eq47_e1664_d_n10: f64 = (p.p37 * eq47_e1663_d_n10);
        let eq47_e1664_d_n11: f64 = (p.p37 * eq47_e1663_d_n11);
        let eq47_e1664_d_n12: f64 = (p.p37 * eq47_e1663_d_n12);
        let eq47_e1664_d_n13: f64 = (p.p37 * eq47_e1663_d_n13);
        let eq47_value: f64 = eq47_e1664;
        let eq47_node_derivatives: [f64; 14] = [eq47_e1664_d_n0, eq47_e1664_d_n1, eq47_e1664_d_n2, eq47_e1664_d_n3, eq47_e1664_d_n4, eq47_e1664_d_n5, eq47_e1664_d_n6, eq47_e1664_d_n7, eq47_e1664_d_n8, eq47_e1664_d_n9, eq47_e1664_d_n10, eq47_e1664_d_n11, eq47_e1664_d_n12, eq47_e1664_d_n13];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            self.multiplicity * (eq47_value),
            &nodes,
            &eq47_node_derivatives,
            &branches,
            &eq47_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_48_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq48_e1668: f64 = (p.p33 * s.v[909]);
        let eq48_e1668_d_n0: f64 = (p.p33 * s.dn[909][0]);
        let eq48_e1668_d_n1: f64 = (p.p33 * s.dn[909][1]);
        let eq48_e1668_d_n2: f64 = (p.p33 * s.dn[909][2]);
        let eq48_e1668_d_n3: f64 = (p.p33 * s.dn[909][3]);
        let eq48_e1668_d_n4: f64 = (p.p33 * s.dn[909][4]);
        let eq48_e1668_d_n5: f64 = (p.p33 * s.dn[909][5]);
        let eq48_e1668_d_n6: f64 = (p.p33 * s.dn[909][6]);
        let eq48_e1668_d_n7: f64 = (p.p33 * s.dn[909][7]);
        let eq48_e1668_d_n8: f64 = (p.p33 * s.dn[909][8]);
        let eq48_e1668_d_n9: f64 = (p.p33 * s.dn[909][9]);
        let eq48_e1668_d_n10: f64 = (p.p33 * s.dn[909][10]);
        let eq48_e1668_d_n11: f64 = (p.p33 * s.dn[909][11]);
        let eq48_e1668_d_n12: f64 = (p.p33 * s.dn[909][12]);
        let eq48_e1668_d_n13: f64 = (p.p33 * s.dn[909][13]);
        let eq48_e1669: f64 = self.eval_ddt(6, eq48_e1668);
        let eq48_e1669_d_n0: f64 = self.ddt_jacobian(eq48_e1668_d_n0);
        let eq48_e1669_d_n1: f64 = self.ddt_jacobian(eq48_e1668_d_n1);
        let eq48_e1669_d_n2: f64 = self.ddt_jacobian(eq48_e1668_d_n2);
        let eq48_e1669_d_n3: f64 = self.ddt_jacobian(eq48_e1668_d_n3);
        let eq48_e1669_d_n4: f64 = self.ddt_jacobian(eq48_e1668_d_n4);
        let eq48_e1669_d_n5: f64 = self.ddt_jacobian(eq48_e1668_d_n5);
        let eq48_e1669_d_n6: f64 = self.ddt_jacobian(eq48_e1668_d_n6);
        let eq48_e1669_d_n7: f64 = self.ddt_jacobian(eq48_e1668_d_n7);
        let eq48_e1669_d_n8: f64 = self.ddt_jacobian(eq48_e1668_d_n8);
        let eq48_e1669_d_n9: f64 = self.ddt_jacobian(eq48_e1668_d_n9);
        let eq48_e1669_d_n10: f64 = self.ddt_jacobian(eq48_e1668_d_n10);
        let eq48_e1669_d_n11: f64 = self.ddt_jacobian(eq48_e1668_d_n11);
        let eq48_e1669_d_n12: f64 = self.ddt_jacobian(eq48_e1668_d_n12);
        let eq48_e1669_d_n13: f64 = self.ddt_jacobian(eq48_e1668_d_n13);
        let eq48_e1670: f64 = (p.p37 * eq48_e1669);
        let eq48_e1670_d_n0: f64 = (p.p37 * eq48_e1669_d_n0);
        let eq48_e1670_d_n1: f64 = (p.p37 * eq48_e1669_d_n1);
        let eq48_e1670_d_n2: f64 = (p.p37 * eq48_e1669_d_n2);
        let eq48_e1670_d_n3: f64 = (p.p37 * eq48_e1669_d_n3);
        let eq48_e1670_d_n4: f64 = (p.p37 * eq48_e1669_d_n4);
        let eq48_e1670_d_n5: f64 = (p.p37 * eq48_e1669_d_n5);
        let eq48_e1670_d_n6: f64 = (p.p37 * eq48_e1669_d_n6);
        let eq48_e1670_d_n7: f64 = (p.p37 * eq48_e1669_d_n7);
        let eq48_e1670_d_n8: f64 = (p.p37 * eq48_e1669_d_n8);
        let eq48_e1670_d_n9: f64 = (p.p37 * eq48_e1669_d_n9);
        let eq48_e1670_d_n10: f64 = (p.p37 * eq48_e1669_d_n10);
        let eq48_e1670_d_n11: f64 = (p.p37 * eq48_e1669_d_n11);
        let eq48_e1670_d_n12: f64 = (p.p37 * eq48_e1669_d_n12);
        let eq48_e1670_d_n13: f64 = (p.p37 * eq48_e1669_d_n13);
        let eq48_value: f64 = eq48_e1670;
        let eq48_node_derivatives: [f64; 14] = [eq48_e1670_d_n0, eq48_e1670_d_n1, eq48_e1670_d_n2, eq48_e1670_d_n3, eq48_e1670_d_n4, eq48_e1670_d_n5, eq48_e1670_d_n6, eq48_e1670_d_n7, eq48_e1670_d_n8, eq48_e1670_d_n9, eq48_e1670_d_n10, eq48_e1670_d_n11, eq48_e1670_d_n12, eq48_e1670_d_n13];
        let eq48_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            self.multiplicity * (eq48_value),
            &nodes,
            &eq48_node_derivatives,
            &branches,
            &eq48_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_49_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq49_e1674: f64 = (p.p33 * s.v[910]);
        let eq49_e1674_d_n0: f64 = (p.p33 * s.dn[910][0]);
        let eq49_e1674_d_n1: f64 = (p.p33 * s.dn[910][1]);
        let eq49_e1674_d_n2: f64 = (p.p33 * s.dn[910][2]);
        let eq49_e1674_d_n3: f64 = (p.p33 * s.dn[910][3]);
        let eq49_e1674_d_n4: f64 = (p.p33 * s.dn[910][4]);
        let eq49_e1674_d_n5: f64 = (p.p33 * s.dn[910][5]);
        let eq49_e1674_d_n6: f64 = (p.p33 * s.dn[910][6]);
        let eq49_e1674_d_n7: f64 = (p.p33 * s.dn[910][7]);
        let eq49_e1674_d_n8: f64 = (p.p33 * s.dn[910][8]);
        let eq49_e1674_d_n9: f64 = (p.p33 * s.dn[910][9]);
        let eq49_e1674_d_n10: f64 = (p.p33 * s.dn[910][10]);
        let eq49_e1674_d_n11: f64 = (p.p33 * s.dn[910][11]);
        let eq49_e1674_d_n12: f64 = (p.p33 * s.dn[910][12]);
        let eq49_e1674_d_n13: f64 = (p.p33 * s.dn[910][13]);
        let eq49_e1675: f64 = self.eval_ddt(7, eq49_e1674);
        let eq49_e1675_d_n0: f64 = self.ddt_jacobian(eq49_e1674_d_n0);
        let eq49_e1675_d_n1: f64 = self.ddt_jacobian(eq49_e1674_d_n1);
        let eq49_e1675_d_n2: f64 = self.ddt_jacobian(eq49_e1674_d_n2);
        let eq49_e1675_d_n3: f64 = self.ddt_jacobian(eq49_e1674_d_n3);
        let eq49_e1675_d_n4: f64 = self.ddt_jacobian(eq49_e1674_d_n4);
        let eq49_e1675_d_n5: f64 = self.ddt_jacobian(eq49_e1674_d_n5);
        let eq49_e1675_d_n6: f64 = self.ddt_jacobian(eq49_e1674_d_n6);
        let eq49_e1675_d_n7: f64 = self.ddt_jacobian(eq49_e1674_d_n7);
        let eq49_e1675_d_n8: f64 = self.ddt_jacobian(eq49_e1674_d_n8);
        let eq49_e1675_d_n9: f64 = self.ddt_jacobian(eq49_e1674_d_n9);
        let eq49_e1675_d_n10: f64 = self.ddt_jacobian(eq49_e1674_d_n10);
        let eq49_e1675_d_n11: f64 = self.ddt_jacobian(eq49_e1674_d_n11);
        let eq49_e1675_d_n12: f64 = self.ddt_jacobian(eq49_e1674_d_n12);
        let eq49_e1675_d_n13: f64 = self.ddt_jacobian(eq49_e1674_d_n13);
        let eq49_e1676: f64 = (p.p37 * eq49_e1675);
        let eq49_e1676_d_n0: f64 = (p.p37 * eq49_e1675_d_n0);
        let eq49_e1676_d_n1: f64 = (p.p37 * eq49_e1675_d_n1);
        let eq49_e1676_d_n2: f64 = (p.p37 * eq49_e1675_d_n2);
        let eq49_e1676_d_n3: f64 = (p.p37 * eq49_e1675_d_n3);
        let eq49_e1676_d_n4: f64 = (p.p37 * eq49_e1675_d_n4);
        let eq49_e1676_d_n5: f64 = (p.p37 * eq49_e1675_d_n5);
        let eq49_e1676_d_n6: f64 = (p.p37 * eq49_e1675_d_n6);
        let eq49_e1676_d_n7: f64 = (p.p37 * eq49_e1675_d_n7);
        let eq49_e1676_d_n8: f64 = (p.p37 * eq49_e1675_d_n8);
        let eq49_e1676_d_n9: f64 = (p.p37 * eq49_e1675_d_n9);
        let eq49_e1676_d_n10: f64 = (p.p37 * eq49_e1675_d_n10);
        let eq49_e1676_d_n11: f64 = (p.p37 * eq49_e1675_d_n11);
        let eq49_e1676_d_n12: f64 = (p.p37 * eq49_e1675_d_n12);
        let eq49_e1676_d_n13: f64 = (p.p37 * eq49_e1675_d_n13);
        let eq49_value: f64 = eq49_e1676;
        let eq49_node_derivatives: [f64; 14] = [eq49_e1676_d_n0, eq49_e1676_d_n1, eq49_e1676_d_n2, eq49_e1676_d_n3, eq49_e1676_d_n4, eq49_e1676_d_n5, eq49_e1676_d_n6, eq49_e1676_d_n7, eq49_e1676_d_n8, eq49_e1676_d_n9, eq49_e1676_d_n10, eq49_e1676_d_n11, eq49_e1676_d_n12, eq49_e1676_d_n13];
        let eq49_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            self.multiplicity * (eq49_value),
            &nodes,
            &eq49_node_derivatives,
            &branches,
            &eq49_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_50_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq50_e1685, eq50_e1685_d_n0, eq50_e1685_d_n1, eq50_e1685_d_n2, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, eq50_e1685_d_n13,) = {
    if (s.v[1553] != 0.0) {
        let eq50_e1681: f64 = (p.p33 * s.v[895]);
        let eq50_e1681_d_n0: f64 = (p.p33 * s.dn[895][0]);
        let eq50_e1681_d_n1: f64 = (p.p33 * s.dn[895][1]);
        let eq50_e1681_d_n2: f64 = (p.p33 * s.dn[895][2]);
        let eq50_e1681_d_n3: f64 = (p.p33 * s.dn[895][3]);
        let eq50_e1681_d_n4: f64 = (p.p33 * s.dn[895][4]);
        let eq50_e1681_d_n5: f64 = (p.p33 * s.dn[895][5]);
        let eq50_e1681_d_n6: f64 = (p.p33 * s.dn[895][6]);
        let eq50_e1681_d_n7: f64 = (p.p33 * s.dn[895][7]);
        let eq50_e1681_d_n8: f64 = (p.p33 * s.dn[895][8]);
        let eq50_e1681_d_n9: f64 = (p.p33 * s.dn[895][9]);
        let eq50_e1681_d_n10: f64 = (p.p33 * s.dn[895][10]);
        let eq50_e1681_d_n11: f64 = (p.p33 * s.dn[895][11]);
        let eq50_e1681_d_n12: f64 = (p.p33 * s.dn[895][12]);
        let eq50_e1681_d_n13: f64 = (p.p33 * s.dn[895][13]);
        let eq50_e1682: f64 = self.eval_ddt(8, eq50_e1681);
        let eq50_e1682_d_n0: f64 = self.ddt_jacobian(eq50_e1681_d_n0);
        let eq50_e1682_d_n1: f64 = self.ddt_jacobian(eq50_e1681_d_n1);
        let eq50_e1682_d_n2: f64 = self.ddt_jacobian(eq50_e1681_d_n2);
        let eq50_e1682_d_n3: f64 = self.ddt_jacobian(eq50_e1681_d_n3);
        let eq50_e1682_d_n4: f64 = self.ddt_jacobian(eq50_e1681_d_n4);
        let eq50_e1682_d_n5: f64 = self.ddt_jacobian(eq50_e1681_d_n5);
        let eq50_e1682_d_n6: f64 = self.ddt_jacobian(eq50_e1681_d_n6);
        let eq50_e1682_d_n7: f64 = self.ddt_jacobian(eq50_e1681_d_n7);
        let eq50_e1682_d_n8: f64 = self.ddt_jacobian(eq50_e1681_d_n8);
        let eq50_e1682_d_n9: f64 = self.ddt_jacobian(eq50_e1681_d_n9);
        let eq50_e1682_d_n10: f64 = self.ddt_jacobian(eq50_e1681_d_n10);
        let eq50_e1682_d_n11: f64 = self.ddt_jacobian(eq50_e1681_d_n11);
        let eq50_e1682_d_n12: f64 = self.ddt_jacobian(eq50_e1681_d_n12);
        let eq50_e1682_d_n13: f64 = self.ddt_jacobian(eq50_e1681_d_n13);
        let eq50_e1683: f64 = (p.p37 * eq50_e1682);
        let eq50_e1683_d_n0: f64 = (p.p37 * eq50_e1682_d_n0);
        let eq50_e1683_d_n1: f64 = (p.p37 * eq50_e1682_d_n1);
        let eq50_e1683_d_n2: f64 = (p.p37 * eq50_e1682_d_n2);
        let eq50_e1683_d_n3: f64 = (p.p37 * eq50_e1682_d_n3);
        let eq50_e1683_d_n4: f64 = (p.p37 * eq50_e1682_d_n4);
        let eq50_e1683_d_n5: f64 = (p.p37 * eq50_e1682_d_n5);
        let eq50_e1683_d_n6: f64 = (p.p37 * eq50_e1682_d_n6);
        let eq50_e1683_d_n7: f64 = (p.p37 * eq50_e1682_d_n7);
        let eq50_e1683_d_n8: f64 = (p.p37 * eq50_e1682_d_n8);
        let eq50_e1683_d_n9: f64 = (p.p37 * eq50_e1682_d_n9);
        let eq50_e1683_d_n10: f64 = (p.p37 * eq50_e1682_d_n10);
        let eq50_e1683_d_n11: f64 = (p.p37 * eq50_e1682_d_n11);
        let eq50_e1683_d_n12: f64 = (p.p37 * eq50_e1682_d_n12);
        let eq50_e1683_d_n13: f64 = (p.p37 * eq50_e1682_d_n13);
        (eq50_e1683, eq50_e1683_d_n0, eq50_e1683_d_n1, eq50_e1683_d_n2, eq50_e1683_d_n3, eq50_e1683_d_n4, eq50_e1683_d_n5, eq50_e1683_d_n6, eq50_e1683_d_n7, eq50_e1683_d_n8, eq50_e1683_d_n9, eq50_e1683_d_n10, eq50_e1683_d_n11, eq50_e1683_d_n12, eq50_e1683_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1685;
        let eq50_node_derivatives: [f64; 14] = [eq50_e1685_d_n0, eq50_e1685_d_n1, eq50_e1685_d_n2, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, eq50_e1685_d_n13];
        let eq50_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            self.multiplicity * (eq50_value),
            &nodes,
            &eq50_node_derivatives,
            &branches,
            &eq50_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_51_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq51_e1694, eq51_e1694_d_n0, eq51_e1694_d_n1, eq51_e1694_d_n2, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, eq51_e1694_d_n13,) = {
    if (s.v[1553] != 0.0) {
        let eq51_e1690: f64 = (p.p33 * s.v[896]);
        let eq51_e1690_d_n0: f64 = (p.p33 * s.dn[896][0]);
        let eq51_e1690_d_n1: f64 = (p.p33 * s.dn[896][1]);
        let eq51_e1690_d_n2: f64 = (p.p33 * s.dn[896][2]);
        let eq51_e1690_d_n3: f64 = (p.p33 * s.dn[896][3]);
        let eq51_e1690_d_n4: f64 = (p.p33 * s.dn[896][4]);
        let eq51_e1690_d_n5: f64 = (p.p33 * s.dn[896][5]);
        let eq51_e1690_d_n6: f64 = (p.p33 * s.dn[896][6]);
        let eq51_e1690_d_n7: f64 = (p.p33 * s.dn[896][7]);
        let eq51_e1690_d_n8: f64 = (p.p33 * s.dn[896][8]);
        let eq51_e1690_d_n9: f64 = (p.p33 * s.dn[896][9]);
        let eq51_e1690_d_n10: f64 = (p.p33 * s.dn[896][10]);
        let eq51_e1690_d_n11: f64 = (p.p33 * s.dn[896][11]);
        let eq51_e1690_d_n12: f64 = (p.p33 * s.dn[896][12]);
        let eq51_e1690_d_n13: f64 = (p.p33 * s.dn[896][13]);
        let eq51_e1691: f64 = self.eval_ddt(9, eq51_e1690);
        let eq51_e1691_d_n0: f64 = self.ddt_jacobian(eq51_e1690_d_n0);
        let eq51_e1691_d_n1: f64 = self.ddt_jacobian(eq51_e1690_d_n1);
        let eq51_e1691_d_n2: f64 = self.ddt_jacobian(eq51_e1690_d_n2);
        let eq51_e1691_d_n3: f64 = self.ddt_jacobian(eq51_e1690_d_n3);
        let eq51_e1691_d_n4: f64 = self.ddt_jacobian(eq51_e1690_d_n4);
        let eq51_e1691_d_n5: f64 = self.ddt_jacobian(eq51_e1690_d_n5);
        let eq51_e1691_d_n6: f64 = self.ddt_jacobian(eq51_e1690_d_n6);
        let eq51_e1691_d_n7: f64 = self.ddt_jacobian(eq51_e1690_d_n7);
        let eq51_e1691_d_n8: f64 = self.ddt_jacobian(eq51_e1690_d_n8);
        let eq51_e1691_d_n9: f64 = self.ddt_jacobian(eq51_e1690_d_n9);
        let eq51_e1691_d_n10: f64 = self.ddt_jacobian(eq51_e1690_d_n10);
        let eq51_e1691_d_n11: f64 = self.ddt_jacobian(eq51_e1690_d_n11);
        let eq51_e1691_d_n12: f64 = self.ddt_jacobian(eq51_e1690_d_n12);
        let eq51_e1691_d_n13: f64 = self.ddt_jacobian(eq51_e1690_d_n13);
        let eq51_e1692: f64 = (p.p37 * eq51_e1691);
        let eq51_e1692_d_n0: f64 = (p.p37 * eq51_e1691_d_n0);
        let eq51_e1692_d_n1: f64 = (p.p37 * eq51_e1691_d_n1);
        let eq51_e1692_d_n2: f64 = (p.p37 * eq51_e1691_d_n2);
        let eq51_e1692_d_n3: f64 = (p.p37 * eq51_e1691_d_n3);
        let eq51_e1692_d_n4: f64 = (p.p37 * eq51_e1691_d_n4);
        let eq51_e1692_d_n5: f64 = (p.p37 * eq51_e1691_d_n5);
        let eq51_e1692_d_n6: f64 = (p.p37 * eq51_e1691_d_n6);
        let eq51_e1692_d_n7: f64 = (p.p37 * eq51_e1691_d_n7);
        let eq51_e1692_d_n8: f64 = (p.p37 * eq51_e1691_d_n8);
        let eq51_e1692_d_n9: f64 = (p.p37 * eq51_e1691_d_n9);
        let eq51_e1692_d_n10: f64 = (p.p37 * eq51_e1691_d_n10);
        let eq51_e1692_d_n11: f64 = (p.p37 * eq51_e1691_d_n11);
        let eq51_e1692_d_n12: f64 = (p.p37 * eq51_e1691_d_n12);
        let eq51_e1692_d_n13: f64 = (p.p37 * eq51_e1691_d_n13);
        (eq51_e1692, eq51_e1692_d_n0, eq51_e1692_d_n1, eq51_e1692_d_n2, eq51_e1692_d_n3, eq51_e1692_d_n4, eq51_e1692_d_n5, eq51_e1692_d_n6, eq51_e1692_d_n7, eq51_e1692_d_n8, eq51_e1692_d_n9, eq51_e1692_d_n10, eq51_e1692_d_n11, eq51_e1692_d_n12, eq51_e1692_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1694;
        let eq51_node_derivatives: [f64; 14] = [eq51_e1694_d_n0, eq51_e1694_d_n1, eq51_e1694_d_n2, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, eq51_e1694_d_n13];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[8]),
            self.multiplicity * (eq51_value),
            &nodes,
            &eq51_node_derivatives,
            &branches,
            &eq51_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_52_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq52_e1703, eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13,) = {
    if (s.v[1553] != 0.0) {
        let eq52_e1698: f64 = (p.p33 * (nv10 - nv3));
        let eq52_e1698_d_n3: f64 = (-p.p33);
        let eq52_e1698_d_n10: f64 = p.p33;
        let eq52_e1700: f64 = (eq52_e1698 * s.v[336]);
        let eq52_e1700_d_n0: f64 = (eq52_e1698 * s.dn[336][0]);
        let eq52_e1700_d_n1: f64 = (eq52_e1698 * s.dn[336][1]);
        let eq52_e1700_d_n2: f64 = (eq52_e1698 * s.dn[336][2]);
        let eq52_e1700_d_n3: f64 = ((eq52_e1698_d_n3 * s.v[336]) + (eq52_e1698 * s.dn[336][3]));
        let eq52_e1700_d_n4: f64 = (eq52_e1698 * s.dn[336][4]);
        let eq52_e1700_d_n5: f64 = (eq52_e1698 * s.dn[336][5]);
        let eq52_e1700_d_n6: f64 = (eq52_e1698 * s.dn[336][6]);
        let eq52_e1700_d_n7: f64 = (eq52_e1698 * s.dn[336][7]);
        let eq52_e1700_d_n8: f64 = (eq52_e1698 * s.dn[336][8]);
        let eq52_e1700_d_n9: f64 = (eq52_e1698 * s.dn[336][9]);
        let eq52_e1700_d_n10: f64 = ((eq52_e1698_d_n10 * s.v[336]) + (eq52_e1698 * s.dn[336][10]));
        let eq52_e1700_d_n11: f64 = (eq52_e1698 * s.dn[336][11]);
        let eq52_e1700_d_n12: f64 = (eq52_e1698 * s.dn[336][12]);
        let eq52_e1700_d_n13: f64 = (eq52_e1698 * s.dn[336][13]);
        let eq52_e1701: f64 = self.eval_ddt(10, eq52_e1700);
        let eq52_e1701_d_n0: f64 = self.ddt_jacobian(eq52_e1700_d_n0);
        let eq52_e1701_d_n1: f64 = self.ddt_jacobian(eq52_e1700_d_n1);
        let eq52_e1701_d_n2: f64 = self.ddt_jacobian(eq52_e1700_d_n2);
        let eq52_e1701_d_n3: f64 = self.ddt_jacobian(eq52_e1700_d_n3);
        let eq52_e1701_d_n4: f64 = self.ddt_jacobian(eq52_e1700_d_n4);
        let eq52_e1701_d_n5: f64 = self.ddt_jacobian(eq52_e1700_d_n5);
        let eq52_e1701_d_n6: f64 = self.ddt_jacobian(eq52_e1700_d_n6);
        let eq52_e1701_d_n7: f64 = self.ddt_jacobian(eq52_e1700_d_n7);
        let eq52_e1701_d_n8: f64 = self.ddt_jacobian(eq52_e1700_d_n8);
        let eq52_e1701_d_n9: f64 = self.ddt_jacobian(eq52_e1700_d_n9);
        let eq52_e1701_d_n10: f64 = self.ddt_jacobian(eq52_e1700_d_n10);
        let eq52_e1701_d_n11: f64 = self.ddt_jacobian(eq52_e1700_d_n11);
        let eq52_e1701_d_n12: f64 = self.ddt_jacobian(eq52_e1700_d_n12);
        let eq52_e1701_d_n13: f64 = self.ddt_jacobian(eq52_e1700_d_n13);
        (eq52_e1701, eq52_e1701_d_n0, eq52_e1701_d_n1, eq52_e1701_d_n2, eq52_e1701_d_n3, eq52_e1701_d_n4, eq52_e1701_d_n5, eq52_e1701_d_n6, eq52_e1701_d_n7, eq52_e1701_d_n8, eq52_e1701_d_n9, eq52_e1701_d_n10, eq52_e1701_d_n11, eq52_e1701_d_n12, eq52_e1701_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e1703;
        let eq52_node_derivatives: [f64; 14] = [eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13];
        let eq52_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[3]),
            self.multiplicity * (eq52_value),
            &nodes,
            &eq52_node_derivatives,
            &branches,
            &eq52_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_53_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq53_e1713, eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13,) = {
    if (!(s.v[1553] != 0.0)) {
        let eq53_e1709: f64 = (p.p33 * s.v[895]);
        let eq53_e1709_d_n0: f64 = (p.p33 * s.dn[895][0]);
        let eq53_e1709_d_n1: f64 = (p.p33 * s.dn[895][1]);
        let eq53_e1709_d_n2: f64 = (p.p33 * s.dn[895][2]);
        let eq53_e1709_d_n3: f64 = (p.p33 * s.dn[895][3]);
        let eq53_e1709_d_n4: f64 = (p.p33 * s.dn[895][4]);
        let eq53_e1709_d_n5: f64 = (p.p33 * s.dn[895][5]);
        let eq53_e1709_d_n6: f64 = (p.p33 * s.dn[895][6]);
        let eq53_e1709_d_n7: f64 = (p.p33 * s.dn[895][7]);
        let eq53_e1709_d_n8: f64 = (p.p33 * s.dn[895][8]);
        let eq53_e1709_d_n9: f64 = (p.p33 * s.dn[895][9]);
        let eq53_e1709_d_n10: f64 = (p.p33 * s.dn[895][10]);
        let eq53_e1709_d_n11: f64 = (p.p33 * s.dn[895][11]);
        let eq53_e1709_d_n12: f64 = (p.p33 * s.dn[895][12]);
        let eq53_e1709_d_n13: f64 = (p.p33 * s.dn[895][13]);
        let eq53_e1710: f64 = self.eval_ddt(11, eq53_e1709);
        let eq53_e1710_d_n0: f64 = self.ddt_jacobian(eq53_e1709_d_n0);
        let eq53_e1710_d_n1: f64 = self.ddt_jacobian(eq53_e1709_d_n1);
        let eq53_e1710_d_n2: f64 = self.ddt_jacobian(eq53_e1709_d_n2);
        let eq53_e1710_d_n3: f64 = self.ddt_jacobian(eq53_e1709_d_n3);
        let eq53_e1710_d_n4: f64 = self.ddt_jacobian(eq53_e1709_d_n4);
        let eq53_e1710_d_n5: f64 = self.ddt_jacobian(eq53_e1709_d_n5);
        let eq53_e1710_d_n6: f64 = self.ddt_jacobian(eq53_e1709_d_n6);
        let eq53_e1710_d_n7: f64 = self.ddt_jacobian(eq53_e1709_d_n7);
        let eq53_e1710_d_n8: f64 = self.ddt_jacobian(eq53_e1709_d_n8);
        let eq53_e1710_d_n9: f64 = self.ddt_jacobian(eq53_e1709_d_n9);
        let eq53_e1710_d_n10: f64 = self.ddt_jacobian(eq53_e1709_d_n10);
        let eq53_e1710_d_n11: f64 = self.ddt_jacobian(eq53_e1709_d_n11);
        let eq53_e1710_d_n12: f64 = self.ddt_jacobian(eq53_e1709_d_n12);
        let eq53_e1710_d_n13: f64 = self.ddt_jacobian(eq53_e1709_d_n13);
        let eq53_e1711: f64 = (p.p37 * eq53_e1710);
        let eq53_e1711_d_n0: f64 = (p.p37 * eq53_e1710_d_n0);
        let eq53_e1711_d_n1: f64 = (p.p37 * eq53_e1710_d_n1);
        let eq53_e1711_d_n2: f64 = (p.p37 * eq53_e1710_d_n2);
        let eq53_e1711_d_n3: f64 = (p.p37 * eq53_e1710_d_n3);
        let eq53_e1711_d_n4: f64 = (p.p37 * eq53_e1710_d_n4);
        let eq53_e1711_d_n5: f64 = (p.p37 * eq53_e1710_d_n5);
        let eq53_e1711_d_n6: f64 = (p.p37 * eq53_e1710_d_n6);
        let eq53_e1711_d_n7: f64 = (p.p37 * eq53_e1710_d_n7);
        let eq53_e1711_d_n8: f64 = (p.p37 * eq53_e1710_d_n8);
        let eq53_e1711_d_n9: f64 = (p.p37 * eq53_e1710_d_n9);
        let eq53_e1711_d_n10: f64 = (p.p37 * eq53_e1710_d_n10);
        let eq53_e1711_d_n11: f64 = (p.p37 * eq53_e1710_d_n11);
        let eq53_e1711_d_n12: f64 = (p.p37 * eq53_e1710_d_n12);
        let eq53_e1711_d_n13: f64 = (p.p37 * eq53_e1710_d_n13);
        (eq53_e1711, eq53_e1711_d_n0, eq53_e1711_d_n1, eq53_e1711_d_n2, eq53_e1711_d_n3, eq53_e1711_d_n4, eq53_e1711_d_n5, eq53_e1711_d_n6, eq53_e1711_d_n7, eq53_e1711_d_n8, eq53_e1711_d_n9, eq53_e1711_d_n10, eq53_e1711_d_n11, eq53_e1711_d_n12, eq53_e1711_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e1713;
        let eq53_node_derivatives: [f64; 14] = [eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq53_value),
            &nodes,
            &eq53_node_derivatives,
            &branches,
            &eq53_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_54_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq54_e1723, eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13,) = {
    if (!(s.v[1553] != 0.0)) {
        let eq54_e1719: f64 = (p.p33 * s.v[896]);
        let eq54_e1719_d_n0: f64 = (p.p33 * s.dn[896][0]);
        let eq54_e1719_d_n1: f64 = (p.p33 * s.dn[896][1]);
        let eq54_e1719_d_n2: f64 = (p.p33 * s.dn[896][2]);
        let eq54_e1719_d_n3: f64 = (p.p33 * s.dn[896][3]);
        let eq54_e1719_d_n4: f64 = (p.p33 * s.dn[896][4]);
        let eq54_e1719_d_n5: f64 = (p.p33 * s.dn[896][5]);
        let eq54_e1719_d_n6: f64 = (p.p33 * s.dn[896][6]);
        let eq54_e1719_d_n7: f64 = (p.p33 * s.dn[896][7]);
        let eq54_e1719_d_n8: f64 = (p.p33 * s.dn[896][8]);
        let eq54_e1719_d_n9: f64 = (p.p33 * s.dn[896][9]);
        let eq54_e1719_d_n10: f64 = (p.p33 * s.dn[896][10]);
        let eq54_e1719_d_n11: f64 = (p.p33 * s.dn[896][11]);
        let eq54_e1719_d_n12: f64 = (p.p33 * s.dn[896][12]);
        let eq54_e1719_d_n13: f64 = (p.p33 * s.dn[896][13]);
        let eq54_e1720: f64 = self.eval_ddt(12, eq54_e1719);
        let eq54_e1720_d_n0: f64 = self.ddt_jacobian(eq54_e1719_d_n0);
        let eq54_e1720_d_n1: f64 = self.ddt_jacobian(eq54_e1719_d_n1);
        let eq54_e1720_d_n2: f64 = self.ddt_jacobian(eq54_e1719_d_n2);
        let eq54_e1720_d_n3: f64 = self.ddt_jacobian(eq54_e1719_d_n3);
        let eq54_e1720_d_n4: f64 = self.ddt_jacobian(eq54_e1719_d_n4);
        let eq54_e1720_d_n5: f64 = self.ddt_jacobian(eq54_e1719_d_n5);
        let eq54_e1720_d_n6: f64 = self.ddt_jacobian(eq54_e1719_d_n6);
        let eq54_e1720_d_n7: f64 = self.ddt_jacobian(eq54_e1719_d_n7);
        let eq54_e1720_d_n8: f64 = self.ddt_jacobian(eq54_e1719_d_n8);
        let eq54_e1720_d_n9: f64 = self.ddt_jacobian(eq54_e1719_d_n9);
        let eq54_e1720_d_n10: f64 = self.ddt_jacobian(eq54_e1719_d_n10);
        let eq54_e1720_d_n11: f64 = self.ddt_jacobian(eq54_e1719_d_n11);
        let eq54_e1720_d_n12: f64 = self.ddt_jacobian(eq54_e1719_d_n12);
        let eq54_e1720_d_n13: f64 = self.ddt_jacobian(eq54_e1719_d_n13);
        let eq54_e1721: f64 = (p.p37 * eq54_e1720);
        let eq54_e1721_d_n0: f64 = (p.p37 * eq54_e1720_d_n0);
        let eq54_e1721_d_n1: f64 = (p.p37 * eq54_e1720_d_n1);
        let eq54_e1721_d_n2: f64 = (p.p37 * eq54_e1720_d_n2);
        let eq54_e1721_d_n3: f64 = (p.p37 * eq54_e1720_d_n3);
        let eq54_e1721_d_n4: f64 = (p.p37 * eq54_e1720_d_n4);
        let eq54_e1721_d_n5: f64 = (p.p37 * eq54_e1720_d_n5);
        let eq54_e1721_d_n6: f64 = (p.p37 * eq54_e1720_d_n6);
        let eq54_e1721_d_n7: f64 = (p.p37 * eq54_e1720_d_n7);
        let eq54_e1721_d_n8: f64 = (p.p37 * eq54_e1720_d_n8);
        let eq54_e1721_d_n9: f64 = (p.p37 * eq54_e1720_d_n9);
        let eq54_e1721_d_n10: f64 = (p.p37 * eq54_e1720_d_n10);
        let eq54_e1721_d_n11: f64 = (p.p37 * eq54_e1720_d_n11);
        let eq54_e1721_d_n12: f64 = (p.p37 * eq54_e1720_d_n12);
        let eq54_e1721_d_n13: f64 = (p.p37 * eq54_e1720_d_n13);
        (eq54_e1721, eq54_e1721_d_n0, eq54_e1721_d_n1, eq54_e1721_d_n2, eq54_e1721_d_n3, eq54_e1721_d_n4, eq54_e1721_d_n5, eq54_e1721_d_n6, eq54_e1721_d_n7, eq54_e1721_d_n8, eq54_e1721_d_n9, eq54_e1721_d_n10, eq54_e1721_d_n11, eq54_e1721_d_n12, eq54_e1721_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e1723;
        let eq54_node_derivatives: [f64; 14] = [eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13];
        let eq54_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq54_value),
            &nodes,
            &eq54_node_derivatives,
            &branches,
            &eq54_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_55_block_0(
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq55_e1733, eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13,) = {
    if (!(s.v[1553] != 0.0)) {
        let eq55_e1728: f64 = (p.p33 * (nv9 - nv3));
        let eq55_e1728_d_n3: f64 = (-p.p33);
        let eq55_e1728_d_n9: f64 = p.p33;
        let eq55_e1730: f64 = (eq55_e1728 * s.v[336]);
        let eq55_e1730_d_n0: f64 = (eq55_e1728 * s.dn[336][0]);
        let eq55_e1730_d_n1: f64 = (eq55_e1728 * s.dn[336][1]);
        let eq55_e1730_d_n2: f64 = (eq55_e1728 * s.dn[336][2]);
        let eq55_e1730_d_n3: f64 = ((eq55_e1728_d_n3 * s.v[336]) + (eq55_e1728 * s.dn[336][3]));
        let eq55_e1730_d_n4: f64 = (eq55_e1728 * s.dn[336][4]);
        let eq55_e1730_d_n5: f64 = (eq55_e1728 * s.dn[336][5]);
        let eq55_e1730_d_n6: f64 = (eq55_e1728 * s.dn[336][6]);
        let eq55_e1730_d_n7: f64 = (eq55_e1728 * s.dn[336][7]);
        let eq55_e1730_d_n8: f64 = (eq55_e1728 * s.dn[336][8]);
        let eq55_e1730_d_n9: f64 = ((eq55_e1728_d_n9 * s.v[336]) + (eq55_e1728 * s.dn[336][9]));
        let eq55_e1730_d_n10: f64 = (eq55_e1728 * s.dn[336][10]);
        let eq55_e1730_d_n11: f64 = (eq55_e1728 * s.dn[336][11]);
        let eq55_e1730_d_n12: f64 = (eq55_e1728 * s.dn[336][12]);
        let eq55_e1730_d_n13: f64 = (eq55_e1728 * s.dn[336][13]);
        let eq55_e1731: f64 = self.eval_ddt(13, eq55_e1730);
        let eq55_e1731_d_n0: f64 = self.ddt_jacobian(eq55_e1730_d_n0);
        let eq55_e1731_d_n1: f64 = self.ddt_jacobian(eq55_e1730_d_n1);
        let eq55_e1731_d_n2: f64 = self.ddt_jacobian(eq55_e1730_d_n2);
        let eq55_e1731_d_n3: f64 = self.ddt_jacobian(eq55_e1730_d_n3);
        let eq55_e1731_d_n4: f64 = self.ddt_jacobian(eq55_e1730_d_n4);
        let eq55_e1731_d_n5: f64 = self.ddt_jacobian(eq55_e1730_d_n5);
        let eq55_e1731_d_n6: f64 = self.ddt_jacobian(eq55_e1730_d_n6);
        let eq55_e1731_d_n7: f64 = self.ddt_jacobian(eq55_e1730_d_n7);
        let eq55_e1731_d_n8: f64 = self.ddt_jacobian(eq55_e1730_d_n8);
        let eq55_e1731_d_n9: f64 = self.ddt_jacobian(eq55_e1730_d_n9);
        let eq55_e1731_d_n10: f64 = self.ddt_jacobian(eq55_e1730_d_n10);
        let eq55_e1731_d_n11: f64 = self.ddt_jacobian(eq55_e1730_d_n11);
        let eq55_e1731_d_n12: f64 = self.ddt_jacobian(eq55_e1730_d_n12);
        let eq55_e1731_d_n13: f64 = self.ddt_jacobian(eq55_e1730_d_n13);
        (eq55_e1731, eq55_e1731_d_n0, eq55_e1731_d_n1, eq55_e1731_d_n2, eq55_e1731_d_n3, eq55_e1731_d_n4, eq55_e1731_d_n5, eq55_e1731_d_n6, eq55_e1731_d_n7, eq55_e1731_d_n8, eq55_e1731_d_n9, eq55_e1731_d_n10, eq55_e1731_d_n11, eq55_e1731_d_n12, eq55_e1731_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1733;
        let eq55_node_derivatives: [f64; 14] = [eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[3]),
            self.multiplicity * (eq55_value),
            &nodes,
            &eq55_node_derivatives,
            &branches,
            &eq55_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_56_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq56_e1736: f64 = (p.p33 * s.v[87]);
        let eq56_e1736_d_n0: f64 = (p.p33 * s.dn[87][0]);
        let eq56_e1736_d_n1: f64 = (p.p33 * s.dn[87][1]);
        let eq56_e1736_d_n2: f64 = (p.p33 * s.dn[87][2]);
        let eq56_e1736_d_n3: f64 = (p.p33 * s.dn[87][3]);
        let eq56_e1736_d_n4: f64 = (p.p33 * s.dn[87][4]);
        let eq56_e1736_d_n5: f64 = (p.p33 * s.dn[87][5]);
        let eq56_e1736_d_n6: f64 = (p.p33 * s.dn[87][6]);
        let eq56_e1736_d_n7: f64 = (p.p33 * s.dn[87][7]);
        let eq56_e1736_d_n8: f64 = (p.p33 * s.dn[87][8]);
        let eq56_e1736_d_n9: f64 = (p.p33 * s.dn[87][9]);
        let eq56_e1736_d_n10: f64 = (p.p33 * s.dn[87][10]);
        let eq56_e1736_d_n11: f64 = (p.p33 * s.dn[87][11]);
        let eq56_e1736_d_n12: f64 = (p.p33 * s.dn[87][12]);
        let eq56_e1736_d_n13: f64 = (p.p33 * s.dn[87][13]);
        let eq56_e1737: f64 = self.eval_ddt(14, eq56_e1736);
        let eq56_e1737_d_n0: f64 = self.ddt_jacobian(eq56_e1736_d_n0);
        let eq56_e1737_d_n1: f64 = self.ddt_jacobian(eq56_e1736_d_n1);
        let eq56_e1737_d_n2: f64 = self.ddt_jacobian(eq56_e1736_d_n2);
        let eq56_e1737_d_n3: f64 = self.ddt_jacobian(eq56_e1736_d_n3);
        let eq56_e1737_d_n4: f64 = self.ddt_jacobian(eq56_e1736_d_n4);
        let eq56_e1737_d_n5: f64 = self.ddt_jacobian(eq56_e1736_d_n5);
        let eq56_e1737_d_n6: f64 = self.ddt_jacobian(eq56_e1736_d_n6);
        let eq56_e1737_d_n7: f64 = self.ddt_jacobian(eq56_e1736_d_n7);
        let eq56_e1737_d_n8: f64 = self.ddt_jacobian(eq56_e1736_d_n8);
        let eq56_e1737_d_n9: f64 = self.ddt_jacobian(eq56_e1736_d_n9);
        let eq56_e1737_d_n10: f64 = self.ddt_jacobian(eq56_e1736_d_n10);
        let eq56_e1737_d_n11: f64 = self.ddt_jacobian(eq56_e1736_d_n11);
        let eq56_e1737_d_n12: f64 = self.ddt_jacobian(eq56_e1736_d_n12);
        let eq56_e1737_d_n13: f64 = self.ddt_jacobian(eq56_e1736_d_n13);
        let eq56_value: f64 = eq56_e1737;
        let eq56_node_derivatives: [f64; 14] = [eq56_e1737_d_n0, eq56_e1737_d_n1, eq56_e1737_d_n2, eq56_e1737_d_n3, eq56_e1737_d_n4, eq56_e1737_d_n5, eq56_e1737_d_n6, eq56_e1737_d_n7, eq56_e1737_d_n8, eq56_e1737_d_n9, eq56_e1737_d_n10, eq56_e1737_d_n11, eq56_e1737_d_n12, eq56_e1737_d_n13];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            self.multiplicity * (eq56_value),
            &nodes,
            &eq56_node_derivatives,
            &branches,
            &eq56_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_57_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq57_e1740: f64 = (p.p33 * s.v[86]);
        let eq57_e1740_d_n0: f64 = (p.p33 * s.dn[86][0]);
        let eq57_e1740_d_n1: f64 = (p.p33 * s.dn[86][1]);
        let eq57_e1740_d_n2: f64 = (p.p33 * s.dn[86][2]);
        let eq57_e1740_d_n3: f64 = (p.p33 * s.dn[86][3]);
        let eq57_e1740_d_n4: f64 = (p.p33 * s.dn[86][4]);
        let eq57_e1740_d_n5: f64 = (p.p33 * s.dn[86][5]);
        let eq57_e1740_d_n6: f64 = (p.p33 * s.dn[86][6]);
        let eq57_e1740_d_n7: f64 = (p.p33 * s.dn[86][7]);
        let eq57_e1740_d_n8: f64 = (p.p33 * s.dn[86][8]);
        let eq57_e1740_d_n9: f64 = (p.p33 * s.dn[86][9]);
        let eq57_e1740_d_n10: f64 = (p.p33 * s.dn[86][10]);
        let eq57_e1740_d_n11: f64 = (p.p33 * s.dn[86][11]);
        let eq57_e1740_d_n12: f64 = (p.p33 * s.dn[86][12]);
        let eq57_e1740_d_n13: f64 = (p.p33 * s.dn[86][13]);
        let eq57_e1741: f64 = self.eval_ddt(15, eq57_e1740);
        let eq57_e1741_d_n0: f64 = self.ddt_jacobian(eq57_e1740_d_n0);
        let eq57_e1741_d_n1: f64 = self.ddt_jacobian(eq57_e1740_d_n1);
        let eq57_e1741_d_n2: f64 = self.ddt_jacobian(eq57_e1740_d_n2);
        let eq57_e1741_d_n3: f64 = self.ddt_jacobian(eq57_e1740_d_n3);
        let eq57_e1741_d_n4: f64 = self.ddt_jacobian(eq57_e1740_d_n4);
        let eq57_e1741_d_n5: f64 = self.ddt_jacobian(eq57_e1740_d_n5);
        let eq57_e1741_d_n6: f64 = self.ddt_jacobian(eq57_e1740_d_n6);
        let eq57_e1741_d_n7: f64 = self.ddt_jacobian(eq57_e1740_d_n7);
        let eq57_e1741_d_n8: f64 = self.ddt_jacobian(eq57_e1740_d_n8);
        let eq57_e1741_d_n9: f64 = self.ddt_jacobian(eq57_e1740_d_n9);
        let eq57_e1741_d_n10: f64 = self.ddt_jacobian(eq57_e1740_d_n10);
        let eq57_e1741_d_n11: f64 = self.ddt_jacobian(eq57_e1740_d_n11);
        let eq57_e1741_d_n12: f64 = self.ddt_jacobian(eq57_e1740_d_n12);
        let eq57_e1741_d_n13: f64 = self.ddt_jacobian(eq57_e1740_d_n13);
        let eq57_value: f64 = eq57_e1741;
        let eq57_node_derivatives: [f64; 14] = [eq57_e1741_d_n0, eq57_e1741_d_n1, eq57_e1741_d_n2, eq57_e1741_d_n3, eq57_e1741_d_n4, eq57_e1741_d_n5, eq57_e1741_d_n6, eq57_e1741_d_n7, eq57_e1741_d_n8, eq57_e1741_d_n9, eq57_e1741_d_n10, eq57_e1741_d_n11, eq57_e1741_d_n12, eq57_e1741_d_n13];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[3]),
            self.multiplicity * (eq57_value),
            &nodes,
            &eq57_node_derivatives,
            &branches,
            &eq57_branch_derivatives,
            self.multiplicity,
        );
    }
}
