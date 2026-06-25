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
        let eq42_e1955: f64 = (-s.v[379]);
        let eq42_e1955_d_n0: f64 = (-s.dn[379][0]);
        let eq42_e1955_d_n1: f64 = (-s.dn[379][1]);
        let eq42_e1955_d_n2: f64 = (-s.dn[379][2]);
        let eq42_e1955_d_n3: f64 = (-s.dn[379][3]);
        let eq42_e1955_d_n4: f64 = (-s.dn[379][4]);
        let eq42_e1955_d_n5: f64 = (-s.dn[379][5]);
        let eq42_e1955_d_n6: f64 = (-s.dn[379][6]);
        let eq42_e1955_d_n7: f64 = (-s.dn[379][7]);
        let eq42_e1955_d_n8: f64 = (-s.dn[379][8]);
        let eq42_e1955_d_n9: f64 = (-s.dn[379][9]);
        let eq42_e1955_d_n10: f64 = (-s.dn[379][10]);
        let eq42_e1955_d_n11: f64 = (-s.dn[379][11]);
        let eq42_e1955_d_n12: f64 = (-s.dn[379][12]);
        let eq42_e1955_d_n13: f64 = (-s.dn[379][13]);
        let eq42_e1957: f64 = (eq42_e1955 * s.v[424]);
        let eq42_e1957_d_n0: f64 = ((eq42_e1955_d_n0 * s.v[424]) + (eq42_e1955 * s.dn[424][0]));
        let eq42_e1957_d_n1: f64 = ((eq42_e1955_d_n1 * s.v[424]) + (eq42_e1955 * s.dn[424][1]));
        let eq42_e1957_d_n2: f64 = ((eq42_e1955_d_n2 * s.v[424]) + (eq42_e1955 * s.dn[424][2]));
        let eq42_e1957_d_n3: f64 = ((eq42_e1955_d_n3 * s.v[424]) + (eq42_e1955 * s.dn[424][3]));
        let eq42_e1957_d_n4: f64 = ((eq42_e1955_d_n4 * s.v[424]) + (eq42_e1955 * s.dn[424][4]));
        let eq42_e1957_d_n5: f64 = ((eq42_e1955_d_n5 * s.v[424]) + (eq42_e1955 * s.dn[424][5]));
        let eq42_e1957_d_n6: f64 = ((eq42_e1955_d_n6 * s.v[424]) + (eq42_e1955 * s.dn[424][6]));
        let eq42_e1957_d_n7: f64 = ((eq42_e1955_d_n7 * s.v[424]) + (eq42_e1955 * s.dn[424][7]));
        let eq42_e1957_d_n8: f64 = ((eq42_e1955_d_n8 * s.v[424]) + (eq42_e1955 * s.dn[424][8]));
        let eq42_e1957_d_n9: f64 = ((eq42_e1955_d_n9 * s.v[424]) + (eq42_e1955 * s.dn[424][9]));
        let eq42_e1957_d_n10: f64 = ((eq42_e1955_d_n10 * s.v[424]) + (eq42_e1955 * s.dn[424][10]));
        let eq42_e1957_d_n11: f64 = ((eq42_e1955_d_n11 * s.v[424]) + (eq42_e1955 * s.dn[424][11]));
        let eq42_e1957_d_n12: f64 = ((eq42_e1955_d_n12 * s.v[424]) + (eq42_e1955 * s.dn[424][12]));
        let eq42_e1957_d_n13: f64 = ((eq42_e1955_d_n13 * s.v[424]) + (eq42_e1955 * s.dn[424][13]));
        let eq42_e1958: f64 = self.eval_ddt(13, eq42_e1957);
        let eq42_e1958_d_n0: f64 = self.ddt_jacobian(eq42_e1957_d_n0);
        let eq42_e1958_d_n1: f64 = self.ddt_jacobian(eq42_e1957_d_n1);
        let eq42_e1958_d_n2: f64 = self.ddt_jacobian(eq42_e1957_d_n2);
        let eq42_e1958_d_n3: f64 = self.ddt_jacobian(eq42_e1957_d_n3);
        let eq42_e1958_d_n4: f64 = self.ddt_jacobian(eq42_e1957_d_n4);
        let eq42_e1958_d_n5: f64 = self.ddt_jacobian(eq42_e1957_d_n5);
        let eq42_e1958_d_n6: f64 = self.ddt_jacobian(eq42_e1957_d_n6);
        let eq42_e1958_d_n7: f64 = self.ddt_jacobian(eq42_e1957_d_n7);
        let eq42_e1958_d_n8: f64 = self.ddt_jacobian(eq42_e1957_d_n8);
        let eq42_e1958_d_n9: f64 = self.ddt_jacobian(eq42_e1957_d_n9);
        let eq42_e1958_d_n10: f64 = self.ddt_jacobian(eq42_e1957_d_n10);
        let eq42_e1958_d_n11: f64 = self.ddt_jacobian(eq42_e1957_d_n11);
        let eq42_e1958_d_n12: f64 = self.ddt_jacobian(eq42_e1957_d_n12);
        let eq42_e1958_d_n13: f64 = self.ddt_jacobian(eq42_e1957_d_n13);
        let eq42_value: f64 = eq42_e1958;
        let eq42_node_derivatives: [f64; 14] = [eq42_e1958_d_n0, eq42_e1958_d_n1, eq42_e1958_d_n2, eq42_e1958_d_n3, eq42_e1958_d_n4, eq42_e1958_d_n5, eq42_e1958_d_n6, eq42_e1958_d_n7, eq42_e1958_d_n8, eq42_e1958_d_n9, eq42_e1958_d_n10, eq42_e1958_d_n11, eq42_e1958_d_n12, eq42_e1958_d_n13];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            self.multiplicity * (eq42_value),
            &nodes,
            &eq42_node_derivatives,
            &branches,
            &eq42_branch_derivatives,
            self.multiplicity,
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
        let eq43_e1960: f64 = (-s.v[379]);
        let eq43_e1960_d_n0: f64 = (-s.dn[379][0]);
        let eq43_e1960_d_n1: f64 = (-s.dn[379][1]);
        let eq43_e1960_d_n2: f64 = (-s.dn[379][2]);
        let eq43_e1960_d_n3: f64 = (-s.dn[379][3]);
        let eq43_e1960_d_n4: f64 = (-s.dn[379][4]);
        let eq43_e1960_d_n5: f64 = (-s.dn[379][5]);
        let eq43_e1960_d_n6: f64 = (-s.dn[379][6]);
        let eq43_e1960_d_n7: f64 = (-s.dn[379][7]);
        let eq43_e1960_d_n8: f64 = (-s.dn[379][8]);
        let eq43_e1960_d_n9: f64 = (-s.dn[379][9]);
        let eq43_e1960_d_n10: f64 = (-s.dn[379][10]);
        let eq43_e1960_d_n11: f64 = (-s.dn[379][11]);
        let eq43_e1960_d_n12: f64 = (-s.dn[379][12]);
        let eq43_e1960_d_n13: f64 = (-s.dn[379][13]);
        let eq43_e1962: f64 = (eq43_e1960 * s.v[421]);
        let eq43_e1962_d_n0: f64 = ((eq43_e1960_d_n0 * s.v[421]) + (eq43_e1960 * s.dn[421][0]));
        let eq43_e1962_d_n1: f64 = ((eq43_e1960_d_n1 * s.v[421]) + (eq43_e1960 * s.dn[421][1]));
        let eq43_e1962_d_n2: f64 = ((eq43_e1960_d_n2 * s.v[421]) + (eq43_e1960 * s.dn[421][2]));
        let eq43_e1962_d_n3: f64 = ((eq43_e1960_d_n3 * s.v[421]) + (eq43_e1960 * s.dn[421][3]));
        let eq43_e1962_d_n4: f64 = ((eq43_e1960_d_n4 * s.v[421]) + (eq43_e1960 * s.dn[421][4]));
        let eq43_e1962_d_n5: f64 = ((eq43_e1960_d_n5 * s.v[421]) + (eq43_e1960 * s.dn[421][5]));
        let eq43_e1962_d_n6: f64 = ((eq43_e1960_d_n6 * s.v[421]) + (eq43_e1960 * s.dn[421][6]));
        let eq43_e1962_d_n7: f64 = ((eq43_e1960_d_n7 * s.v[421]) + (eq43_e1960 * s.dn[421][7]));
        let eq43_e1962_d_n8: f64 = ((eq43_e1960_d_n8 * s.v[421]) + (eq43_e1960 * s.dn[421][8]));
        let eq43_e1962_d_n9: f64 = ((eq43_e1960_d_n9 * s.v[421]) + (eq43_e1960 * s.dn[421][9]));
        let eq43_e1962_d_n10: f64 = ((eq43_e1960_d_n10 * s.v[421]) + (eq43_e1960 * s.dn[421][10]));
        let eq43_e1962_d_n11: f64 = ((eq43_e1960_d_n11 * s.v[421]) + (eq43_e1960 * s.dn[421][11]));
        let eq43_e1962_d_n12: f64 = ((eq43_e1960_d_n12 * s.v[421]) + (eq43_e1960 * s.dn[421][12]));
        let eq43_e1962_d_n13: f64 = ((eq43_e1960_d_n13 * s.v[421]) + (eq43_e1960 * s.dn[421][13]));
        let eq43_e1963: f64 = self.eval_ddt(14, eq43_e1962);
        let eq43_e1963_d_n0: f64 = self.ddt_jacobian(eq43_e1962_d_n0);
        let eq43_e1963_d_n1: f64 = self.ddt_jacobian(eq43_e1962_d_n1);
        let eq43_e1963_d_n2: f64 = self.ddt_jacobian(eq43_e1962_d_n2);
        let eq43_e1963_d_n3: f64 = self.ddt_jacobian(eq43_e1962_d_n3);
        let eq43_e1963_d_n4: f64 = self.ddt_jacobian(eq43_e1962_d_n4);
        let eq43_e1963_d_n5: f64 = self.ddt_jacobian(eq43_e1962_d_n5);
        let eq43_e1963_d_n6: f64 = self.ddt_jacobian(eq43_e1962_d_n6);
        let eq43_e1963_d_n7: f64 = self.ddt_jacobian(eq43_e1962_d_n7);
        let eq43_e1963_d_n8: f64 = self.ddt_jacobian(eq43_e1962_d_n8);
        let eq43_e1963_d_n9: f64 = self.ddt_jacobian(eq43_e1962_d_n9);
        let eq43_e1963_d_n10: f64 = self.ddt_jacobian(eq43_e1962_d_n10);
        let eq43_e1963_d_n11: f64 = self.ddt_jacobian(eq43_e1962_d_n11);
        let eq43_e1963_d_n12: f64 = self.ddt_jacobian(eq43_e1962_d_n12);
        let eq43_e1963_d_n13: f64 = self.ddt_jacobian(eq43_e1962_d_n13);
        let eq43_value: f64 = eq43_e1963;
        let eq43_node_derivatives: [f64; 14] = [eq43_e1963_d_n0, eq43_e1963_d_n1, eq43_e1963_d_n2, eq43_e1963_d_n3, eq43_e1963_d_n4, eq43_e1963_d_n5, eq43_e1963_d_n6, eq43_e1963_d_n7, eq43_e1963_d_n8, eq43_e1963_d_n9, eq43_e1963_d_n10, eq43_e1963_d_n11, eq43_e1963_d_n12, eq43_e1963_d_n13];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[10]),
            self.multiplicity * (eq43_value),
            &nodes,
            &eq43_node_derivatives,
            &branches,
            &eq43_branch_derivatives,
            self.multiplicity,
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
        let eq44_e1966: f64 = (s.v[379] * s.v[210]);
        let eq44_e1966_d_n0: f64 = ((s.dn[379][0] * s.v[210]) + (s.v[379] * s.dn[210][0]));
        let eq44_e1966_d_n1: f64 = ((s.dn[379][1] * s.v[210]) + (s.v[379] * s.dn[210][1]));
        let eq44_e1966_d_n2: f64 = ((s.dn[379][2] * s.v[210]) + (s.v[379] * s.dn[210][2]));
        let eq44_e1966_d_n3: f64 = ((s.dn[379][3] * s.v[210]) + (s.v[379] * s.dn[210][3]));
        let eq44_e1966_d_n4: f64 = ((s.dn[379][4] * s.v[210]) + (s.v[379] * s.dn[210][4]));
        let eq44_e1966_d_n5: f64 = ((s.dn[379][5] * s.v[210]) + (s.v[379] * s.dn[210][5]));
        let eq44_e1966_d_n6: f64 = ((s.dn[379][6] * s.v[210]) + (s.v[379] * s.dn[210][6]));
        let eq44_e1966_d_n7: f64 = ((s.dn[379][7] * s.v[210]) + (s.v[379] * s.dn[210][7]));
        let eq44_e1966_d_n8: f64 = ((s.dn[379][8] * s.v[210]) + (s.v[379] * s.dn[210][8]));
        let eq44_e1966_d_n9: f64 = ((s.dn[379][9] * s.v[210]) + (s.v[379] * s.dn[210][9]));
        let eq44_e1966_d_n10: f64 = ((s.dn[379][10] * s.v[210]) + (s.v[379] * s.dn[210][10]));
        let eq44_e1966_d_n11: f64 = ((s.dn[379][11] * s.v[210]) + (s.v[379] * s.dn[210][11]));
        let eq44_e1966_d_n12: f64 = ((s.dn[379][12] * s.v[210]) + (s.v[379] * s.dn[210][12]));
        let eq44_e1966_d_n13: f64 = ((s.dn[379][13] * s.v[210]) + (s.v[379] * s.dn[210][13]));
        let eq44_value: f64 = eq44_e1966;
        let eq44_node_derivatives: [f64; 14] = [eq44_e1966_d_n0, eq44_e1966_d_n1, eq44_e1966_d_n2, eq44_e1966_d_n3, eq44_e1966_d_n4, eq44_e1966_d_n5, eq44_e1966_d_n6, eq44_e1966_d_n7, eq44_e1966_d_n8, eq44_e1966_d_n9, eq44_e1966_d_n10, eq44_e1966_d_n11, eq44_e1966_d_n12, eq44_e1966_d_n13];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[11]),
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
        let eq45_e1969: f64 = self.eval_ddt(15, s.v[1039]);
        let eq45_e1969_d_n0: f64 = self.ddt_jacobian(s.dn[1039][0]);
        let eq45_e1969_d_n1: f64 = self.ddt_jacobian(s.dn[1039][1]);
        let eq45_e1969_d_n2: f64 = self.ddt_jacobian(s.dn[1039][2]);
        let eq45_e1969_d_n3: f64 = self.ddt_jacobian(s.dn[1039][3]);
        let eq45_e1969_d_n4: f64 = self.ddt_jacobian(s.dn[1039][4]);
        let eq45_e1969_d_n5: f64 = self.ddt_jacobian(s.dn[1039][5]);
        let eq45_e1969_d_n6: f64 = self.ddt_jacobian(s.dn[1039][6]);
        let eq45_e1969_d_n7: f64 = self.ddt_jacobian(s.dn[1039][7]);
        let eq45_e1969_d_n8: f64 = self.ddt_jacobian(s.dn[1039][8]);
        let eq45_e1969_d_n9: f64 = self.ddt_jacobian(s.dn[1039][9]);
        let eq45_e1969_d_n10: f64 = self.ddt_jacobian(s.dn[1039][10]);
        let eq45_e1969_d_n11: f64 = self.ddt_jacobian(s.dn[1039][11]);
        let eq45_e1969_d_n12: f64 = self.ddt_jacobian(s.dn[1039][12]);
        let eq45_e1969_d_n13: f64 = self.ddt_jacobian(s.dn[1039][13]);
        let eq45_e1970: f64 = (s.v[379] * eq45_e1969);
        let eq45_e1970_d_n0: f64 = ((s.dn[379][0] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n0));
        let eq45_e1970_d_n1: f64 = ((s.dn[379][1] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n1));
        let eq45_e1970_d_n2: f64 = ((s.dn[379][2] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n2));
        let eq45_e1970_d_n3: f64 = ((s.dn[379][3] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n3));
        let eq45_e1970_d_n4: f64 = ((s.dn[379][4] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n4));
        let eq45_e1970_d_n5: f64 = ((s.dn[379][5] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n5));
        let eq45_e1970_d_n6: f64 = ((s.dn[379][6] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n6));
        let eq45_e1970_d_n7: f64 = ((s.dn[379][7] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n7));
        let eq45_e1970_d_n8: f64 = ((s.dn[379][8] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n8));
        let eq45_e1970_d_n9: f64 = ((s.dn[379][9] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n9));
        let eq45_e1970_d_n10: f64 = ((s.dn[379][10] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n10));
        let eq45_e1970_d_n11: f64 = ((s.dn[379][11] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n11));
        let eq45_e1970_d_n12: f64 = ((s.dn[379][12] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n12));
        let eq45_e1970_d_n13: f64 = ((s.dn[379][13] * eq45_e1969) + (s.v[379] * eq45_e1969_d_n13));
        let eq45_value: f64 = eq45_e1970;
        let eq45_node_derivatives: [f64; 14] = [eq45_e1970_d_n0, eq45_e1970_d_n1, eq45_e1970_d_n2, eq45_e1970_d_n3, eq45_e1970_d_n4, eq45_e1970_d_n5, eq45_e1970_d_n6, eq45_e1970_d_n7, eq45_e1970_d_n8, eq45_e1970_d_n9, eq45_e1970_d_n10, eq45_e1970_d_n11, eq45_e1970_d_n12, eq45_e1970_d_n13];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[10]),
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
        let eq46_e1972: f64 = self.eval_ddt(16, s.v[1047]);
        let eq46_e1972_d_n0: f64 = self.ddt_jacobian(s.dn[1047][0]);
        let eq46_e1972_d_n1: f64 = self.ddt_jacobian(s.dn[1047][1]);
        let eq46_e1972_d_n2: f64 = self.ddt_jacobian(s.dn[1047][2]);
        let eq46_e1972_d_n3: f64 = self.ddt_jacobian(s.dn[1047][3]);
        let eq46_e1972_d_n4: f64 = self.ddt_jacobian(s.dn[1047][4]);
        let eq46_e1972_d_n5: f64 = self.ddt_jacobian(s.dn[1047][5]);
        let eq46_e1972_d_n6: f64 = self.ddt_jacobian(s.dn[1047][6]);
        let eq46_e1972_d_n7: f64 = self.ddt_jacobian(s.dn[1047][7]);
        let eq46_e1972_d_n8: f64 = self.ddt_jacobian(s.dn[1047][8]);
        let eq46_e1972_d_n9: f64 = self.ddt_jacobian(s.dn[1047][9]);
        let eq46_e1972_d_n10: f64 = self.ddt_jacobian(s.dn[1047][10]);
        let eq46_e1972_d_n11: f64 = self.ddt_jacobian(s.dn[1047][11]);
        let eq46_e1972_d_n12: f64 = self.ddt_jacobian(s.dn[1047][12]);
        let eq46_e1972_d_n13: f64 = self.ddt_jacobian(s.dn[1047][13]);
        let eq46_value: f64 = eq46_e1972;
        let eq46_node_derivatives: [f64; 14] = [eq46_e1972_d_n0, eq46_e1972_d_n1, eq46_e1972_d_n2, eq46_e1972_d_n3, eq46_e1972_d_n4, eq46_e1972_d_n5, eq46_e1972_d_n6, eq46_e1972_d_n7, eq46_e1972_d_n8, eq46_e1972_d_n9, eq46_e1972_d_n10, eq46_e1972_d_n11, eq46_e1972_d_n12, eq46_e1972_d_n13];
        let eq46_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
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
        let eq47_e1974: f64 = self.eval_ddt(17, s.v[1046]);
        let eq47_e1974_d_n0: f64 = self.ddt_jacobian(s.dn[1046][0]);
        let eq47_e1974_d_n1: f64 = self.ddt_jacobian(s.dn[1046][1]);
        let eq47_e1974_d_n2: f64 = self.ddt_jacobian(s.dn[1046][2]);
        let eq47_e1974_d_n3: f64 = self.ddt_jacobian(s.dn[1046][3]);
        let eq47_e1974_d_n4: f64 = self.ddt_jacobian(s.dn[1046][4]);
        let eq47_e1974_d_n5: f64 = self.ddt_jacobian(s.dn[1046][5]);
        let eq47_e1974_d_n6: f64 = self.ddt_jacobian(s.dn[1046][6]);
        let eq47_e1974_d_n7: f64 = self.ddt_jacobian(s.dn[1046][7]);
        let eq47_e1974_d_n8: f64 = self.ddt_jacobian(s.dn[1046][8]);
        let eq47_e1974_d_n9: f64 = self.ddt_jacobian(s.dn[1046][9]);
        let eq47_e1974_d_n10: f64 = self.ddt_jacobian(s.dn[1046][10]);
        let eq47_e1974_d_n11: f64 = self.ddt_jacobian(s.dn[1046][11]);
        let eq47_e1974_d_n12: f64 = self.ddt_jacobian(s.dn[1046][12]);
        let eq47_e1974_d_n13: f64 = self.ddt_jacobian(s.dn[1046][13]);
        let eq47_value: f64 = eq47_e1974;
        let eq47_node_derivatives: [f64; 14] = [eq47_e1974_d_n0, eq47_e1974_d_n1, eq47_e1974_d_n2, eq47_e1974_d_n3, eq47_e1974_d_n4, eq47_e1974_d_n5, eq47_e1974_d_n6, eq47_e1974_d_n7, eq47_e1974_d_n8, eq47_e1974_d_n9, eq47_e1974_d_n10, eq47_e1974_d_n11, eq47_e1974_d_n12, eq47_e1974_d_n13];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[3]),
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
        let eq48_e1977: f64 = (s.v[379] * s.v[211]);
        let eq48_e1977_d_n0: f64 = ((s.dn[379][0] * s.v[211]) + (s.v[379] * s.dn[211][0]));
        let eq48_e1977_d_n1: f64 = ((s.dn[379][1] * s.v[211]) + (s.v[379] * s.dn[211][1]));
        let eq48_e1977_d_n2: f64 = ((s.dn[379][2] * s.v[211]) + (s.v[379] * s.dn[211][2]));
        let eq48_e1977_d_n3: f64 = ((s.dn[379][3] * s.v[211]) + (s.v[379] * s.dn[211][3]));
        let eq48_e1977_d_n4: f64 = ((s.dn[379][4] * s.v[211]) + (s.v[379] * s.dn[211][4]));
        let eq48_e1977_d_n5: f64 = ((s.dn[379][5] * s.v[211]) + (s.v[379] * s.dn[211][5]));
        let eq48_e1977_d_n6: f64 = ((s.dn[379][6] * s.v[211]) + (s.v[379] * s.dn[211][6]));
        let eq48_e1977_d_n7: f64 = ((s.dn[379][7] * s.v[211]) + (s.v[379] * s.dn[211][7]));
        let eq48_e1977_d_n8: f64 = ((s.dn[379][8] * s.v[211]) + (s.v[379] * s.dn[211][8]));
        let eq48_e1977_d_n9: f64 = ((s.dn[379][9] * s.v[211]) + (s.v[379] * s.dn[211][9]));
        let eq48_e1977_d_n10: f64 = ((s.dn[379][10] * s.v[211]) + (s.v[379] * s.dn[211][10]));
        let eq48_e1977_d_n11: f64 = ((s.dn[379][11] * s.v[211]) + (s.v[379] * s.dn[211][11]));
        let eq48_e1977_d_n12: f64 = ((s.dn[379][12] * s.v[211]) + (s.v[379] * s.dn[211][12]));
        let eq48_e1977_d_n13: f64 = ((s.dn[379][13] * s.v[211]) + (s.v[379] * s.dn[211][13]));
        let eq48_e1979: f64 = (eq48_e1977 * s.v[380]);
        let eq48_e1979_d_n0: f64 = ((eq48_e1977_d_n0 * s.v[380]) + (eq48_e1977 * s.dn[380][0]));
        let eq48_e1979_d_n1: f64 = ((eq48_e1977_d_n1 * s.v[380]) + (eq48_e1977 * s.dn[380][1]));
        let eq48_e1979_d_n2: f64 = ((eq48_e1977_d_n2 * s.v[380]) + (eq48_e1977 * s.dn[380][2]));
        let eq48_e1979_d_n3: f64 = ((eq48_e1977_d_n3 * s.v[380]) + (eq48_e1977 * s.dn[380][3]));
        let eq48_e1979_d_n4: f64 = ((eq48_e1977_d_n4 * s.v[380]) + (eq48_e1977 * s.dn[380][4]));
        let eq48_e1979_d_n5: f64 = ((eq48_e1977_d_n5 * s.v[380]) + (eq48_e1977 * s.dn[380][5]));
        let eq48_e1979_d_n6: f64 = ((eq48_e1977_d_n6 * s.v[380]) + (eq48_e1977 * s.dn[380][6]));
        let eq48_e1979_d_n7: f64 = ((eq48_e1977_d_n7 * s.v[380]) + (eq48_e1977 * s.dn[380][7]));
        let eq48_e1979_d_n8: f64 = ((eq48_e1977_d_n8 * s.v[380]) + (eq48_e1977 * s.dn[380][8]));
        let eq48_e1979_d_n9: f64 = ((eq48_e1977_d_n9 * s.v[380]) + (eq48_e1977 * s.dn[380][9]));
        let eq48_e1979_d_n10: f64 = ((eq48_e1977_d_n10 * s.v[380]) + (eq48_e1977 * s.dn[380][10]));
        let eq48_e1979_d_n11: f64 = ((eq48_e1977_d_n11 * s.v[380]) + (eq48_e1977 * s.dn[380][11]));
        let eq48_e1979_d_n12: f64 = ((eq48_e1977_d_n12 * s.v[380]) + (eq48_e1977 * s.dn[380][12]));
        let eq48_e1979_d_n13: f64 = ((eq48_e1977_d_n13 * s.v[380]) + (eq48_e1977 * s.dn[380][13]));
        let eq48_value: f64 = eq48_e1979;
        let eq48_node_derivatives: [f64; 14] = [eq48_e1979_d_n0, eq48_e1979_d_n1, eq48_e1979_d_n2, eq48_e1979_d_n3, eq48_e1979_d_n4, eq48_e1979_d_n5, eq48_e1979_d_n6, eq48_e1979_d_n7, eq48_e1979_d_n8, eq48_e1979_d_n9, eq48_e1979_d_n10, eq48_e1979_d_n11, eq48_e1979_d_n12, eq48_e1979_d_n13];
        let eq48_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
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
        let (eq49_e1983, eq49_e1983_d_n0, eq49_e1983_d_n1, eq49_e1983_d_n2, eq49_e1983_d_n3, eq49_e1983_d_n4, eq49_e1983_d_n5, eq49_e1983_d_n6, eq49_e1983_d_n7, eq49_e1983_d_n8, eq49_e1983_d_n9, eq49_e1983_d_n10, eq49_e1983_d_n11, eq49_e1983_d_n12, eq49_e1983_d_n13,) = {
    if (s.v[2009] != 0.0) {
        (s.v[1102], s.dn[1102][0], s.dn[1102][1], s.dn[1102][2], s.dn[1102][3], s.dn[1102][4], s.dn[1102][5], s.dn[1102][6], s.dn[1102][7], s.dn[1102][8], s.dn[1102][9], s.dn[1102][10], s.dn[1102][11], s.dn[1102][12], s.dn[1102][13],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e1983;
        let eq49_node_derivatives: [f64; 14] = [eq49_e1983_d_n0, eq49_e1983_d_n1, eq49_e1983_d_n2, eq49_e1983_d_n3, eq49_e1983_d_n4, eq49_e1983_d_n5, eq49_e1983_d_n6, eq49_e1983_d_n7, eq49_e1983_d_n8, eq49_e1983_d_n9, eq49_e1983_d_n10, eq49_e1983_d_n11, eq49_e1983_d_n12, eq49_e1983_d_n13];
        let eq49_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[10]),
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
        let (eq50_e1989, eq50_e1989_d_n0, eq50_e1989_d_n1, eq50_e1989_d_n2, eq50_e1989_d_n3, eq50_e1989_d_n4, eq50_e1989_d_n5, eq50_e1989_d_n6, eq50_e1989_d_n7, eq50_e1989_d_n8, eq50_e1989_d_n9, eq50_e1989_d_n10, eq50_e1989_d_n11, eq50_e1989_d_n12, eq50_e1989_d_n13,) = {
    if (s.v[2010] != 0.0) {
        let eq50_e1987: f64 = (s.v[1098] + s.v[1100]);
        let eq50_e1987_d_n0: f64 = (s.dn[1098][0] + s.dn[1100][0]);
        let eq50_e1987_d_n1: f64 = (s.dn[1098][1] + s.dn[1100][1]);
        let eq50_e1987_d_n2: f64 = (s.dn[1098][2] + s.dn[1100][2]);
        let eq50_e1987_d_n3: f64 = (s.dn[1098][3] + s.dn[1100][3]);
        let eq50_e1987_d_n4: f64 = (s.dn[1098][4] + s.dn[1100][4]);
        let eq50_e1987_d_n5: f64 = (s.dn[1098][5] + s.dn[1100][5]);
        let eq50_e1987_d_n6: f64 = (s.dn[1098][6] + s.dn[1100][6]);
        let eq50_e1987_d_n7: f64 = (s.dn[1098][7] + s.dn[1100][7]);
        let eq50_e1987_d_n8: f64 = (s.dn[1098][8] + s.dn[1100][8]);
        let eq50_e1987_d_n9: f64 = (s.dn[1098][9] + s.dn[1100][9]);
        let eq50_e1987_d_n10: f64 = (s.dn[1098][10] + s.dn[1100][10]);
        let eq50_e1987_d_n11: f64 = (s.dn[1098][11] + s.dn[1100][11]);
        let eq50_e1987_d_n12: f64 = (s.dn[1098][12] + s.dn[1100][12]);
        let eq50_e1987_d_n13: f64 = (s.dn[1098][13] + s.dn[1100][13]);
        (eq50_e1987, eq50_e1987_d_n0, eq50_e1987_d_n1, eq50_e1987_d_n2, eq50_e1987_d_n3, eq50_e1987_d_n4, eq50_e1987_d_n5, eq50_e1987_d_n6, eq50_e1987_d_n7, eq50_e1987_d_n8, eq50_e1987_d_n9, eq50_e1987_d_n10, eq50_e1987_d_n11, eq50_e1987_d_n12, eq50_e1987_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1989;
        let eq50_node_derivatives: [f64; 14] = [eq50_e1989_d_n0, eq50_e1989_d_n1, eq50_e1989_d_n2, eq50_e1989_d_n3, eq50_e1989_d_n4, eq50_e1989_d_n5, eq50_e1989_d_n6, eq50_e1989_d_n7, eq50_e1989_d_n8, eq50_e1989_d_n9, eq50_e1989_d_n10, eq50_e1989_d_n11, eq50_e1989_d_n12, eq50_e1989_d_n13];
        let eq50_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
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
        let (eq51_e1995, eq51_e1995_d_n0, eq51_e1995_d_n1, eq51_e1995_d_n2, eq51_e1995_d_n3, eq51_e1995_d_n4, eq51_e1995_d_n5, eq51_e1995_d_n6, eq51_e1995_d_n7, eq51_e1995_d_n8, eq51_e1995_d_n9, eq51_e1995_d_n10, eq51_e1995_d_n11, eq51_e1995_d_n12, eq51_e1995_d_n13,) = {
    if (s.v[2010] != 0.0) {
        let eq51_e1993: f64 = (s.v[1099] + s.v[1101]);
        let eq51_e1993_d_n0: f64 = (s.dn[1099][0] + s.dn[1101][0]);
        let eq51_e1993_d_n1: f64 = (s.dn[1099][1] + s.dn[1101][1]);
        let eq51_e1993_d_n2: f64 = (s.dn[1099][2] + s.dn[1101][2]);
        let eq51_e1993_d_n3: f64 = (s.dn[1099][3] + s.dn[1101][3]);
        let eq51_e1993_d_n4: f64 = (s.dn[1099][4] + s.dn[1101][4]);
        let eq51_e1993_d_n5: f64 = (s.dn[1099][5] + s.dn[1101][5]);
        let eq51_e1993_d_n6: f64 = (s.dn[1099][6] + s.dn[1101][6]);
        let eq51_e1993_d_n7: f64 = (s.dn[1099][7] + s.dn[1101][7]);
        let eq51_e1993_d_n8: f64 = (s.dn[1099][8] + s.dn[1101][8]);
        let eq51_e1993_d_n9: f64 = (s.dn[1099][9] + s.dn[1101][9]);
        let eq51_e1993_d_n10: f64 = (s.dn[1099][10] + s.dn[1101][10]);
        let eq51_e1993_d_n11: f64 = (s.dn[1099][11] + s.dn[1101][11]);
        let eq51_e1993_d_n12: f64 = (s.dn[1099][12] + s.dn[1101][12]);
        let eq51_e1993_d_n13: f64 = (s.dn[1099][13] + s.dn[1101][13]);
        (eq51_e1993, eq51_e1993_d_n0, eq51_e1993_d_n1, eq51_e1993_d_n2, eq51_e1993_d_n3, eq51_e1993_d_n4, eq51_e1993_d_n5, eq51_e1993_d_n6, eq51_e1993_d_n7, eq51_e1993_d_n8, eq51_e1993_d_n9, eq51_e1993_d_n10, eq51_e1993_d_n11, eq51_e1993_d_n12, eq51_e1993_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1995;
        let eq51_node_derivatives: [f64; 14] = [eq51_e1995_d_n0, eq51_e1995_d_n1, eq51_e1995_d_n2, eq51_e1995_d_n3, eq51_e1995_d_n4, eq51_e1995_d_n5, eq51_e1995_d_n6, eq51_e1995_d_n7, eq51_e1995_d_n8, eq51_e1995_d_n9, eq51_e1995_d_n10, eq51_e1995_d_n11, eq51_e1995_d_n12, eq51_e1995_d_n13];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
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
        let (eq52_e2001, eq52_e2001_d_n0, eq52_e2001_d_n1, eq52_e2001_d_n2, eq52_e2001_d_n3, eq52_e2001_d_n4, eq52_e2001_d_n5, eq52_e2001_d_n6, eq52_e2001_d_n7, eq52_e2001_d_n8, eq52_e2001_d_n9, eq52_e2001_d_n10, eq52_e2001_d_n11, eq52_e2001_d_n12, eq52_e2001_d_n13,) = {
    if (s.v[2011] != 0.0) {
        let eq52_e1999: f64 = (s.v[1095] + s.v[1096]);
        let eq52_e1999_d_n0: f64 = (s.dn[1095][0] + s.dn[1096][0]);
        let eq52_e1999_d_n1: f64 = (s.dn[1095][1] + s.dn[1096][1]);
        let eq52_e1999_d_n2: f64 = (s.dn[1095][2] + s.dn[1096][2]);
        let eq52_e1999_d_n3: f64 = (s.dn[1095][3] + s.dn[1096][3]);
        let eq52_e1999_d_n4: f64 = (s.dn[1095][4] + s.dn[1096][4]);
        let eq52_e1999_d_n5: f64 = (s.dn[1095][5] + s.dn[1096][5]);
        let eq52_e1999_d_n6: f64 = (s.dn[1095][6] + s.dn[1096][6]);
        let eq52_e1999_d_n7: f64 = (s.dn[1095][7] + s.dn[1096][7]);
        let eq52_e1999_d_n8: f64 = (s.dn[1095][8] + s.dn[1096][8]);
        let eq52_e1999_d_n9: f64 = (s.dn[1095][9] + s.dn[1096][9]);
        let eq52_e1999_d_n10: f64 = (s.dn[1095][10] + s.dn[1096][10]);
        let eq52_e1999_d_n11: f64 = (s.dn[1095][11] + s.dn[1096][11]);
        let eq52_e1999_d_n12: f64 = (s.dn[1095][12] + s.dn[1096][12]);
        let eq52_e1999_d_n13: f64 = (s.dn[1095][13] + s.dn[1096][13]);
        (eq52_e1999, eq52_e1999_d_n0, eq52_e1999_d_n1, eq52_e1999_d_n2, eq52_e1999_d_n3, eq52_e1999_d_n4, eq52_e1999_d_n5, eq52_e1999_d_n6, eq52_e1999_d_n7, eq52_e1999_d_n8, eq52_e1999_d_n9, eq52_e1999_d_n10, eq52_e1999_d_n11, eq52_e1999_d_n12, eq52_e1999_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e2001;
        let eq52_node_derivatives: [f64; 14] = [eq52_e2001_d_n0, eq52_e2001_d_n1, eq52_e2001_d_n2, eq52_e2001_d_n3, eq52_e2001_d_n4, eq52_e2001_d_n5, eq52_e2001_d_n6, eq52_e2001_d_n7, eq52_e2001_d_n8, eq52_e2001_d_n9, eq52_e2001_d_n10, eq52_e2001_d_n11, eq52_e2001_d_n12, eq52_e2001_d_n13];
        let eq52_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[10]),
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
        let (eq53_e2005, eq53_e2005_d_n0, eq53_e2005_d_n1, eq53_e2005_d_n2, eq53_e2005_d_n3, eq53_e2005_d_n4, eq53_e2005_d_n5, eq53_e2005_d_n6, eq53_e2005_d_n7, eq53_e2005_d_n8, eq53_e2005_d_n9, eq53_e2005_d_n10, eq53_e2005_d_n11, eq53_e2005_d_n12, eq53_e2005_d_n13,) = {
    if (s.v[2011] != 0.0) {
        (s.v[1097], s.dn[1097][0], s.dn[1097][1], s.dn[1097][2], s.dn[1097][3], s.dn[1097][4], s.dn[1097][5], s.dn[1097][6], s.dn[1097][7], s.dn[1097][8], s.dn[1097][9], s.dn[1097][10], s.dn[1097][11], s.dn[1097][12], s.dn[1097][13],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e2005;
        let eq53_node_derivatives: [f64; 14] = [eq53_e2005_d_n0, eq53_e2005_d_n1, eq53_e2005_d_n2, eq53_e2005_d_n3, eq53_e2005_d_n4, eq53_e2005_d_n5, eq53_e2005_d_n6, eq53_e2005_d_n7, eq53_e2005_d_n8, eq53_e2005_d_n9, eq53_e2005_d_n10, eq53_e2005_d_n11, eq53_e2005_d_n12, eq53_e2005_d_n13];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[10]),
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
        let (eq54_e2010, eq54_e2010_d_n0, eq54_e2010_d_n1, eq54_e2010_d_n2, eq54_e2010_d_n3, eq54_e2010_d_n4, eq54_e2010_d_n5, eq54_e2010_d_n6, eq54_e2010_d_n7, eq54_e2010_d_n8, eq54_e2010_d_n9, eq54_e2010_d_n10, eq54_e2010_d_n11, eq54_e2010_d_n12, eq54_e2010_d_n13,) = {
    if (!(s.v[2011] != 0.0)) {
        (s.v[1096], s.dn[1096][0], s.dn[1096][1], s.dn[1096][2], s.dn[1096][3], s.dn[1096][4], s.dn[1096][5], s.dn[1096][6], s.dn[1096][7], s.dn[1096][8], s.dn[1096][9], s.dn[1096][10], s.dn[1096][11], s.dn[1096][12], s.dn[1096][13],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2010;
        let eq54_node_derivatives: [f64; 14] = [eq54_e2010_d_n0, eq54_e2010_d_n1, eq54_e2010_d_n2, eq54_e2010_d_n3, eq54_e2010_d_n4, eq54_e2010_d_n5, eq54_e2010_d_n6, eq54_e2010_d_n7, eq54_e2010_d_n8, eq54_e2010_d_n9, eq54_e2010_d_n10, eq54_e2010_d_n11, eq54_e2010_d_n12, eq54_e2010_d_n13];
        let eq54_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[10]),
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
        let (eq55_e2017, eq55_e2017_d_n0, eq55_e2017_d_n1, eq55_e2017_d_n2, eq55_e2017_d_n3, eq55_e2017_d_n4, eq55_e2017_d_n5, eq55_e2017_d_n6, eq55_e2017_d_n7, eq55_e2017_d_n8, eq55_e2017_d_n9, eq55_e2017_d_n10, eq55_e2017_d_n11, eq55_e2017_d_n12, eq55_e2017_d_n13,) = {
    if (!(s.v[2011] != 0.0)) {
        let eq55_e2015: f64 = (s.v[1095] + s.v[1097]);
        let eq55_e2015_d_n0: f64 = (s.dn[1095][0] + s.dn[1097][0]);
        let eq55_e2015_d_n1: f64 = (s.dn[1095][1] + s.dn[1097][1]);
        let eq55_e2015_d_n2: f64 = (s.dn[1095][2] + s.dn[1097][2]);
        let eq55_e2015_d_n3: f64 = (s.dn[1095][3] + s.dn[1097][3]);
        let eq55_e2015_d_n4: f64 = (s.dn[1095][4] + s.dn[1097][4]);
        let eq55_e2015_d_n5: f64 = (s.dn[1095][5] + s.dn[1097][5]);
        let eq55_e2015_d_n6: f64 = (s.dn[1095][6] + s.dn[1097][6]);
        let eq55_e2015_d_n7: f64 = (s.dn[1095][7] + s.dn[1097][7]);
        let eq55_e2015_d_n8: f64 = (s.dn[1095][8] + s.dn[1097][8]);
        let eq55_e2015_d_n9: f64 = (s.dn[1095][9] + s.dn[1097][9]);
        let eq55_e2015_d_n10: f64 = (s.dn[1095][10] + s.dn[1097][10]);
        let eq55_e2015_d_n11: f64 = (s.dn[1095][11] + s.dn[1097][11]);
        let eq55_e2015_d_n12: f64 = (s.dn[1095][12] + s.dn[1097][12]);
        let eq55_e2015_d_n13: f64 = (s.dn[1095][13] + s.dn[1097][13]);
        (eq55_e2015, eq55_e2015_d_n0, eq55_e2015_d_n1, eq55_e2015_d_n2, eq55_e2015_d_n3, eq55_e2015_d_n4, eq55_e2015_d_n5, eq55_e2015_d_n6, eq55_e2015_d_n7, eq55_e2015_d_n8, eq55_e2015_d_n9, eq55_e2015_d_n10, eq55_e2015_d_n11, eq55_e2015_d_n12, eq55_e2015_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2017;
        let eq55_node_derivatives: [f64; 14] = [eq55_e2017_d_n0, eq55_e2017_d_n1, eq55_e2017_d_n2, eq55_e2017_d_n3, eq55_e2017_d_n4, eq55_e2017_d_n5, eq55_e2017_d_n6, eq55_e2017_d_n7, eq55_e2017_d_n8, eq55_e2017_d_n9, eq55_e2017_d_n10, eq55_e2017_d_n11, eq55_e2017_d_n12, eq55_e2017_d_n13];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[10]),
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
        let (eq56_e2021,) = {
    if (s.v[2012] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e2021;
        stamper.stamp_potential(
            branches[2],
            eq56_value,
            &[
            ],
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq57_e2028, eq57_e2028_d_n0, eq57_e2028_d_n1, eq57_e2028_d_n2, eq57_e2028_d_n3, eq57_e2028_d_n4, eq57_e2028_d_n5, eq57_e2028_d_n6, eq57_e2028_d_n7, eq57_e2028_d_n8, eq57_e2028_d_n9, eq57_e2028_d_n10, eq57_e2028_d_n11, eq57_e2028_d_n12, eq57_e2028_d_n13,) = {
    if (!(s.v[2012] != 0.0)) {
        let eq57_e2026: f64 = ((nv1 - nv9) * s.v[2013]);
        let eq57_e2026_d_n0: f64 = ((nv1 - nv9) * s.dn[2013][0]);
        let eq57_e2026_d_n1: f64 = (s.v[2013] + ((nv1 - nv9) * s.dn[2013][1]));
        let eq57_e2026_d_n2: f64 = ((nv1 - nv9) * s.dn[2013][2]);
        let eq57_e2026_d_n3: f64 = ((nv1 - nv9) * s.dn[2013][3]);
        let eq57_e2026_d_n4: f64 = ((nv1 - nv9) * s.dn[2013][4]);
        let eq57_e2026_d_n5: f64 = ((nv1 - nv9) * s.dn[2013][5]);
        let eq57_e2026_d_n6: f64 = ((nv1 - nv9) * s.dn[2013][6]);
        let eq57_e2026_d_n7: f64 = ((nv1 - nv9) * s.dn[2013][7]);
        let eq57_e2026_d_n8: f64 = ((nv1 - nv9) * s.dn[2013][8]);
        let eq57_e2026_d_n9: f64 = ((-s.v[2013]) + ((nv1 - nv9) * s.dn[2013][9]));
        let eq57_e2026_d_n10: f64 = ((nv1 - nv9) * s.dn[2013][10]);
        let eq57_e2026_d_n11: f64 = ((nv1 - nv9) * s.dn[2013][11]);
        let eq57_e2026_d_n12: f64 = ((nv1 - nv9) * s.dn[2013][12]);
        let eq57_e2026_d_n13: f64 = ((nv1 - nv9) * s.dn[2013][13]);
        (eq57_e2026, eq57_e2026_d_n0, eq57_e2026_d_n1, eq57_e2026_d_n2, eq57_e2026_d_n3, eq57_e2026_d_n4, eq57_e2026_d_n5, eq57_e2026_d_n6, eq57_e2026_d_n7, eq57_e2026_d_n8, eq57_e2026_d_n9, eq57_e2026_d_n10, eq57_e2026_d_n11, eq57_e2026_d_n12, eq57_e2026_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2028;
        let eq57_node_derivatives: [f64; 14] = [eq57_e2028_d_n0, eq57_e2028_d_n1, eq57_e2028_d_n2, eq57_e2028_d_n3, eq57_e2028_d_n4, eq57_e2028_d_n5, eq57_e2028_d_n6, eq57_e2028_d_n7, eq57_e2028_d_n8, eq57_e2028_d_n9, eq57_e2028_d_n10, eq57_e2028_d_n11, eq57_e2028_d_n12, eq57_e2028_d_n13];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            self.multiplicity * (eq57_value),
            &nodes,
            &eq57_node_derivatives,
            &branches,
            &eq57_branch_derivatives,
            self.multiplicity,
        );
    }
}
