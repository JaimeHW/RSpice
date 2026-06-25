#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq35_e1938_q: f64 = s.v[1057];
        let eq35_reactive_node_derivatives: [f64; 14] = [s.dn[1057][0], s.dn[1057][1], s.dn[1057][2], s.dn[1057][3], s.dn[1057][4], s.dn[1057][5], s.dn[1057][6], s.dn[1057][7], s.dn[1057][8], s.dn[1057][9], s.dn[1057][10], s.dn[1057][11], s.dn[1057][12], s.dn[1057][13]];
        let eq35_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[10]),
            &nodes,
            &eq35_reactive_node_derivatives,
            &branches,
            &eq35_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq36_e1940_q: f64 = s.v[1058];
        let eq36_reactive_node_derivatives: [f64; 14] = [s.dn[1058][0], s.dn[1058][1], s.dn[1058][2], s.dn[1058][3], s.dn[1058][4], s.dn[1058][5], s.dn[1058][6], s.dn[1058][7], s.dn[1058][8], s.dn[1058][9], s.dn[1058][10], s.dn[1058][11], s.dn[1058][12], s.dn[1058][13]];
        let eq36_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[11]),
            &nodes,
            &eq36_reactive_node_derivatives,
            &branches,
            &eq36_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq37_e1942_q: f64 = s.v[1051];
        let eq37_reactive_node_derivatives: [f64; 14] = [s.dn[1051][0], s.dn[1051][1], s.dn[1051][2], s.dn[1051][3], s.dn[1051][4], s.dn[1051][5], s.dn[1051][6], s.dn[1051][7], s.dn[1051][8], s.dn[1051][9], s.dn[1051][10], s.dn[1051][11], s.dn[1051][12], s.dn[1051][13]];
        let eq37_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &nodes,
            &eq37_reactive_node_derivatives,
            &branches,
            &eq37_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_38_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq38_e1944_q: f64 = s.v[1052];
        let eq38_reactive_node_derivatives: [f64; 14] = [s.dn[1052][0], s.dn[1052][1], s.dn[1052][2], s.dn[1052][3], s.dn[1052][4], s.dn[1052][5], s.dn[1052][6], s.dn[1052][7], s.dn[1052][8], s.dn[1052][9], s.dn[1052][10], s.dn[1052][11], s.dn[1052][12], s.dn[1052][13]];
        let eq38_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            &nodes,
            &eq38_reactive_node_derivatives,
            &branches,
            &eq38_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_39_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq39_e1946_q: f64 = s.v[1054];
        let eq39_reactive_node_derivatives: [f64; 14] = [s.dn[1054][0], s.dn[1054][1], s.dn[1054][2], s.dn[1054][3], s.dn[1054][4], s.dn[1054][5], s.dn[1054][6], s.dn[1054][7], s.dn[1054][8], s.dn[1054][9], s.dn[1054][10], s.dn[1054][11], s.dn[1054][12], s.dn[1054][13]];
        let eq39_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[10]),
            &nodes,
            &eq39_reactive_node_derivatives,
            &branches,
            &eq39_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_40_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq40_e1948_q: f64 = s.v[1055];
        let eq40_reactive_node_derivatives: [f64; 14] = [s.dn[1055][0], s.dn[1055][1], s.dn[1055][2], s.dn[1055][3], s.dn[1055][4], s.dn[1055][5], s.dn[1055][6], s.dn[1055][7], s.dn[1055][8], s.dn[1055][9], s.dn[1055][10], s.dn[1055][11], s.dn[1055][12], s.dn[1055][13]];
        let eq40_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            &nodes,
            &eq40_reactive_node_derivatives,
            &branches,
            &eq40_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_41_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq41_e1950: f64 = (-s.v[379]);
        let eq41_e1950_d_n0: f64 = (-s.dn[379][0]);
        let eq41_e1950_d_n1: f64 = (-s.dn[379][1]);
        let eq41_e1950_d_n2: f64 = (-s.dn[379][2]);
        let eq41_e1950_d_n3: f64 = (-s.dn[379][3]);
        let eq41_e1950_d_n4: f64 = (-s.dn[379][4]);
        let eq41_e1950_d_n5: f64 = (-s.dn[379][5]);
        let eq41_e1950_d_n6: f64 = (-s.dn[379][6]);
        let eq41_e1950_d_n7: f64 = (-s.dn[379][7]);
        let eq41_e1950_d_n8: f64 = (-s.dn[379][8]);
        let eq41_e1950_d_n9: f64 = (-s.dn[379][9]);
        let eq41_e1950_d_n10: f64 = (-s.dn[379][10]);
        let eq41_e1950_d_n11: f64 = (-s.dn[379][11]);
        let eq41_e1950_d_n12: f64 = (-s.dn[379][12]);
        let eq41_e1950_d_n13: f64 = (-s.dn[379][13]);
        let eq41_e1952: f64 = (eq41_e1950 * s.v[423]);
        let eq41_e1952_d_n0: f64 = ((eq41_e1950_d_n0 * s.v[423]) + (eq41_e1950 * s.dn[423][0]));
        let eq41_e1952_d_n1: f64 = ((eq41_e1950_d_n1 * s.v[423]) + (eq41_e1950 * s.dn[423][1]));
        let eq41_e1952_d_n2: f64 = ((eq41_e1950_d_n2 * s.v[423]) + (eq41_e1950 * s.dn[423][2]));
        let eq41_e1952_d_n3: f64 = ((eq41_e1950_d_n3 * s.v[423]) + (eq41_e1950 * s.dn[423][3]));
        let eq41_e1952_d_n4: f64 = ((eq41_e1950_d_n4 * s.v[423]) + (eq41_e1950 * s.dn[423][4]));
        let eq41_e1952_d_n5: f64 = ((eq41_e1950_d_n5 * s.v[423]) + (eq41_e1950 * s.dn[423][5]));
        let eq41_e1952_d_n6: f64 = ((eq41_e1950_d_n6 * s.v[423]) + (eq41_e1950 * s.dn[423][6]));
        let eq41_e1952_d_n7: f64 = ((eq41_e1950_d_n7 * s.v[423]) + (eq41_e1950 * s.dn[423][7]));
        let eq41_e1952_d_n8: f64 = ((eq41_e1950_d_n8 * s.v[423]) + (eq41_e1950 * s.dn[423][8]));
        let eq41_e1952_d_n9: f64 = ((eq41_e1950_d_n9 * s.v[423]) + (eq41_e1950 * s.dn[423][9]));
        let eq41_e1952_d_n10: f64 = ((eq41_e1950_d_n10 * s.v[423]) + (eq41_e1950 * s.dn[423][10]));
        let eq41_e1952_d_n11: f64 = ((eq41_e1950_d_n11 * s.v[423]) + (eq41_e1950 * s.dn[423][11]));
        let eq41_e1952_d_n12: f64 = ((eq41_e1950_d_n12 * s.v[423]) + (eq41_e1950 * s.dn[423][12]));
        let eq41_e1952_d_n13: f64 = ((eq41_e1950_d_n13 * s.v[423]) + (eq41_e1950 * s.dn[423][13]));
        let eq41_e1953_q: f64 = eq41_e1952;
        let eq41_reactive_node_derivatives: [f64; 14] = [eq41_e1952_d_n0, eq41_e1952_d_n1, eq41_e1952_d_n2, eq41_e1952_d_n3, eq41_e1952_d_n4, eq41_e1952_d_n5, eq41_e1952_d_n6, eq41_e1952_d_n7, eq41_e1952_d_n8, eq41_e1952_d_n9, eq41_e1952_d_n10, eq41_e1952_d_n11, eq41_e1952_d_n12, eq41_e1952_d_n13];
        let eq41_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq41_reactive_node_derivatives,
            &branches,
            &eq41_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_42_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
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
        let eq42_e1958_q: f64 = eq42_e1957;
        let eq42_reactive_node_derivatives: [f64; 14] = [eq42_e1957_d_n0, eq42_e1957_d_n1, eq42_e1957_d_n2, eq42_e1957_d_n3, eq42_e1957_d_n4, eq42_e1957_d_n5, eq42_e1957_d_n6, eq42_e1957_d_n7, eq42_e1957_d_n8, eq42_e1957_d_n9, eq42_e1957_d_n10, eq42_e1957_d_n11, eq42_e1957_d_n12, eq42_e1957_d_n13];
        let eq42_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            &nodes,
            &eq42_reactive_node_derivatives,
            &branches,
            &eq42_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_43_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
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
        let eq43_e1963_q: f64 = eq43_e1962;
        let eq43_reactive_node_derivatives: [f64; 14] = [eq43_e1962_d_n0, eq43_e1962_d_n1, eq43_e1962_d_n2, eq43_e1962_d_n3, eq43_e1962_d_n4, eq43_e1962_d_n5, eq43_e1962_d_n6, eq43_e1962_d_n7, eq43_e1962_d_n8, eq43_e1962_d_n9, eq43_e1962_d_n10, eq43_e1962_d_n11, eq43_e1962_d_n12, eq43_e1962_d_n13];
        let eq43_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[10]),
            &nodes,
            &eq43_reactive_node_derivatives,
            &branches,
            &eq43_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_45_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq45_e1969_q: f64 = s.v[1039];
        let eq45_e1970: f64 = (s.v[379] * s.v[1039]);
        let eq45_e1970_d_n0: f64 = ((s.dn[379][0] * s.v[1039]) + (s.v[379] * s.dn[1039][0]));
        let eq45_e1970_d_n1: f64 = ((s.dn[379][1] * s.v[1039]) + (s.v[379] * s.dn[1039][1]));
        let eq45_e1970_d_n2: f64 = ((s.dn[379][2] * s.v[1039]) + (s.v[379] * s.dn[1039][2]));
        let eq45_e1970_d_n3: f64 = ((s.dn[379][3] * s.v[1039]) + (s.v[379] * s.dn[1039][3]));
        let eq45_e1970_d_n4: f64 = ((s.dn[379][4] * s.v[1039]) + (s.v[379] * s.dn[1039][4]));
        let eq45_e1970_d_n5: f64 = ((s.dn[379][5] * s.v[1039]) + (s.v[379] * s.dn[1039][5]));
        let eq45_e1970_d_n6: f64 = ((s.dn[379][6] * s.v[1039]) + (s.v[379] * s.dn[1039][6]));
        let eq45_e1970_d_n7: f64 = ((s.dn[379][7] * s.v[1039]) + (s.v[379] * s.dn[1039][7]));
        let eq45_e1970_d_n8: f64 = ((s.dn[379][8] * s.v[1039]) + (s.v[379] * s.dn[1039][8]));
        let eq45_e1970_d_n9: f64 = ((s.dn[379][9] * s.v[1039]) + (s.v[379] * s.dn[1039][9]));
        let eq45_e1970_d_n10: f64 = ((s.dn[379][10] * s.v[1039]) + (s.v[379] * s.dn[1039][10]));
        let eq45_e1970_d_n11: f64 = ((s.dn[379][11] * s.v[1039]) + (s.v[379] * s.dn[1039][11]));
        let eq45_e1970_d_n12: f64 = ((s.dn[379][12] * s.v[1039]) + (s.v[379] * s.dn[1039][12]));
        let eq45_e1970_d_n13: f64 = ((s.dn[379][13] * s.v[1039]) + (s.v[379] * s.dn[1039][13]));
        let eq45_e1970_q: f64 = (s.v[379] * eq45_e1969_q);
        let eq45_e1970_q_d_n0: f64 = ((s.dn[379][0] * eq45_e1969_q) + (s.v[379] * s.dn[1039][0]));
        let eq45_e1970_q_d_n1: f64 = ((s.dn[379][1] * eq45_e1969_q) + (s.v[379] * s.dn[1039][1]));
        let eq45_e1970_q_d_n2: f64 = ((s.dn[379][2] * eq45_e1969_q) + (s.v[379] * s.dn[1039][2]));
        let eq45_e1970_q_d_n3: f64 = ((s.dn[379][3] * eq45_e1969_q) + (s.v[379] * s.dn[1039][3]));
        let eq45_e1970_q_d_n4: f64 = ((s.dn[379][4] * eq45_e1969_q) + (s.v[379] * s.dn[1039][4]));
        let eq45_e1970_q_d_n5: f64 = ((s.dn[379][5] * eq45_e1969_q) + (s.v[379] * s.dn[1039][5]));
        let eq45_e1970_q_d_n6: f64 = ((s.dn[379][6] * eq45_e1969_q) + (s.v[379] * s.dn[1039][6]));
        let eq45_e1970_q_d_n7: f64 = ((s.dn[379][7] * eq45_e1969_q) + (s.v[379] * s.dn[1039][7]));
        let eq45_e1970_q_d_n8: f64 = ((s.dn[379][8] * eq45_e1969_q) + (s.v[379] * s.dn[1039][8]));
        let eq45_e1970_q_d_n9: f64 = ((s.dn[379][9] * eq45_e1969_q) + (s.v[379] * s.dn[1039][9]));
        let eq45_e1970_q_d_n10: f64 = ((s.dn[379][10] * eq45_e1969_q) + (s.v[379] * s.dn[1039][10]));
        let eq45_e1970_q_d_n11: f64 = ((s.dn[379][11] * eq45_e1969_q) + (s.v[379] * s.dn[1039][11]));
        let eq45_e1970_q_d_n12: f64 = ((s.dn[379][12] * eq45_e1969_q) + (s.v[379] * s.dn[1039][12]));
        let eq45_e1970_q_d_n13: f64 = ((s.dn[379][13] * eq45_e1969_q) + (s.v[379] * s.dn[1039][13]));
        let eq45_reactive_node_derivatives: [f64; 14] = [eq45_e1970_q_d_n0, eq45_e1970_q_d_n1, eq45_e1970_q_d_n2, eq45_e1970_q_d_n3, eq45_e1970_q_d_n4, eq45_e1970_q_d_n5, eq45_e1970_q_d_n6, eq45_e1970_q_d_n7, eq45_e1970_q_d_n8, eq45_e1970_q_d_n9, eq45_e1970_q_d_n10, eq45_e1970_q_d_n11, eq45_e1970_q_d_n12, eq45_e1970_q_d_n13];
        let eq45_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            &nodes,
            &eq45_reactive_node_derivatives,
            &branches,
            &eq45_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq46_e1972_q: f64 = s.v[1047];
        let eq46_reactive_node_derivatives: [f64; 14] = [s.dn[1047][0], s.dn[1047][1], s.dn[1047][2], s.dn[1047][3], s.dn[1047][4], s.dn[1047][5], s.dn[1047][6], s.dn[1047][7], s.dn[1047][8], s.dn[1047][9], s.dn[1047][10], s.dn[1047][11], s.dn[1047][12], s.dn[1047][13]];
        let eq46_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            &nodes,
            &eq46_reactive_node_derivatives,
            &branches,
            &eq46_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_47_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq47_e1974_q: f64 = s.v[1046];
        let eq47_reactive_node_derivatives: [f64; 14] = [s.dn[1046][0], s.dn[1046][1], s.dn[1046][2], s.dn[1046][3], s.dn[1046][4], s.dn[1046][5], s.dn[1046][6], s.dn[1046][7], s.dn[1046][8], s.dn[1046][9], s.dn[1046][10], s.dn[1046][11], s.dn[1046][12], s.dn[1046][13]];
        let eq47_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            &nodes,
            &eq47_reactive_node_derivatives,
            &branches,
            &eq47_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_67_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n1, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11, eq67_e2103_d_n12, eq67_e2103_d_n13, eq67_e2103_q, eq67_e2103_q_d_n0, eq67_e2103_q_d_n1, eq67_e2103_q_d_n2, eq67_e2103_q_d_n3, eq67_e2103_q_d_n4, eq67_e2103_q_d_n5, eq67_e2103_q_d_n6, eq67_e2103_q_d_n7, eq67_e2103_q_d_n8, eq67_e2103_q_d_n9, eq67_e2103_q_d_n10, eq67_e2103_q_d_n11, eq67_e2103_q_d_n12, eq67_e2103_q_d_n13,) = {
    if (((s.v[2021] != 0.0) && (s.v[2024] != 0.0)) && (s.v[2025] != 0.0)) {
        let eq67_e2094: f64 = (s.v[634] * s.v[1015]);
        let eq67_e2094_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq67_e2094_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq67_e2094_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq67_e2094_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq67_e2094_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq67_e2094_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq67_e2094_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq67_e2094_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq67_e2094_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq67_e2094_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq67_e2094_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq67_e2094_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq67_e2094_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq67_e2094_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq67_e2097: f64 = (s.v[634] * s.v[1016]);
        let eq67_e2097_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq67_e2097_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq67_e2097_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq67_e2097_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq67_e2097_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq67_e2097_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq67_e2097_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq67_e2097_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq67_e2097_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq67_e2097_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq67_e2097_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq67_e2097_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq67_e2097_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq67_e2097_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq67_e2098_q: f64 = eq67_e2097;
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2097);
        let eq67_e2099_d_n0: f64 = (eq67_e2094_d_n0 + eq67_e2097_d_n0);
        let eq67_e2099_d_n1: f64 = (eq67_e2094_d_n1 + eq67_e2097_d_n1);
        let eq67_e2099_d_n2: f64 = (eq67_e2094_d_n2 + eq67_e2097_d_n2);
        let eq67_e2099_d_n3: f64 = (eq67_e2094_d_n3 + eq67_e2097_d_n3);
        let eq67_e2099_d_n4: f64 = (eq67_e2094_d_n4 + eq67_e2097_d_n4);
        let eq67_e2099_d_n5: f64 = (eq67_e2094_d_n5 + eq67_e2097_d_n5);
        let eq67_e2099_d_n6: f64 = (eq67_e2094_d_n6 + eq67_e2097_d_n6);
        let eq67_e2099_d_n7: f64 = (eq67_e2094_d_n7 + eq67_e2097_d_n7);
        let eq67_e2099_d_n8: f64 = (eq67_e2094_d_n8 + eq67_e2097_d_n8);
        let eq67_e2099_d_n9: f64 = (eq67_e2094_d_n9 + eq67_e2097_d_n9);
        let eq67_e2099_d_n10: f64 = (eq67_e2094_d_n10 + eq67_e2097_d_n10);
        let eq67_e2099_d_n11: f64 = (eq67_e2094_d_n11 + eq67_e2097_d_n11);
        let eq67_e2099_d_n12: f64 = (eq67_e2094_d_n12 + eq67_e2097_d_n12);
        let eq67_e2099_d_n13: f64 = (eq67_e2094_d_n13 + eq67_e2097_d_n13);
        let eq67_e2099_q: f64 = eq67_e2098_q;
        let eq67_e2101: f64 = (eq67_e2099 - s.v[1017]);
        let eq67_e2101_d_n0: f64 = (eq67_e2099_d_n0 - s.dn[1017][0]);
        let eq67_e2101_d_n1: f64 = (eq67_e2099_d_n1 - s.dn[1017][1]);
        let eq67_e2101_d_n2: f64 = (eq67_e2099_d_n2 - s.dn[1017][2]);
        let eq67_e2101_d_n3: f64 = (eq67_e2099_d_n3 - s.dn[1017][3]);
        let eq67_e2101_d_n4: f64 = (eq67_e2099_d_n4 - s.dn[1017][4]);
        let eq67_e2101_d_n5: f64 = (eq67_e2099_d_n5 - s.dn[1017][5]);
        let eq67_e2101_d_n6: f64 = (eq67_e2099_d_n6 - s.dn[1017][6]);
        let eq67_e2101_d_n7: f64 = (eq67_e2099_d_n7 - s.dn[1017][7]);
        let eq67_e2101_d_n8: f64 = (eq67_e2099_d_n8 - s.dn[1017][8]);
        let eq67_e2101_d_n9: f64 = (eq67_e2099_d_n9 - s.dn[1017][9]);
        let eq67_e2101_d_n10: f64 = (eq67_e2099_d_n10 - s.dn[1017][10]);
        let eq67_e2101_d_n11: f64 = (eq67_e2099_d_n11 - s.dn[1017][11]);
        let eq67_e2101_d_n12: f64 = (eq67_e2099_d_n12 - s.dn[1017][12]);
        let eq67_e2101_d_n13: f64 = (eq67_e2099_d_n13 - s.dn[1017][13]);
        let eq67_e2101_q: f64 = eq67_e2099_q;
        (eq67_e2101, eq67_e2101_d_n0, eq67_e2101_d_n1, eq67_e2101_d_n2, eq67_e2101_d_n3, eq67_e2101_d_n4, eq67_e2101_d_n5, eq67_e2101_d_n6, eq67_e2101_d_n7, eq67_e2101_d_n8, eq67_e2101_d_n9, eq67_e2101_d_n10, eq67_e2101_d_n11, eq67_e2101_d_n12, eq67_e2101_d_n13, eq67_e2101_q, eq67_e2097_d_n0, eq67_e2097_d_n1, eq67_e2097_d_n2, eq67_e2097_d_n3, eq67_e2097_d_n4, eq67_e2097_d_n5, eq67_e2097_d_n6, eq67_e2097_d_n7, eq67_e2097_d_n8, eq67_e2097_d_n9, eq67_e2097_d_n10, eq67_e2097_d_n11, eq67_e2097_d_n12, eq67_e2097_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_reactive_node_derivatives: [f64; 14] = [eq67_e2103_q_d_n0, eq67_e2103_q_d_n1, eq67_e2103_q_d_n2, eq67_e2103_q_d_n3, eq67_e2103_q_d_n4, eq67_e2103_q_d_n5, eq67_e2103_q_d_n6, eq67_e2103_q_d_n7, eq67_e2103_q_d_n8, eq67_e2103_q_d_n9, eq67_e2103_q_d_n10, eq67_e2103_q_d_n11, eq67_e2103_q_d_n12, eq67_e2103_q_d_n13];
        let eq67_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            &nodes,
            &eq67_reactive_node_derivatives,
            &branches,
            &eq67_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_68_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n1, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11, eq68_e2121_d_n12, eq68_e2121_d_n13, eq68_e2121_q, eq68_e2121_q_d_n0, eq68_e2121_q_d_n1, eq68_e2121_q_d_n2, eq68_e2121_q_d_n3, eq68_e2121_q_d_n4, eq68_e2121_q_d_n5, eq68_e2121_q_d_n6, eq68_e2121_q_d_n7, eq68_e2121_q_d_n8, eq68_e2121_q_d_n9, eq68_e2121_q_d_n10, eq68_e2121_q_d_n11, eq68_e2121_q_d_n12, eq68_e2121_q_d_n13,) = {
    if (((s.v[2021] != 0.0) && (s.v[2024] != 0.0)) && (!(s.v[2025] != 0.0))) {
        let eq68_e2112: f64 = (s.v[634] * s.v[1015]);
        let eq68_e2112_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq68_e2112_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq68_e2112_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq68_e2112_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq68_e2112_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq68_e2112_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq68_e2112_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq68_e2112_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq68_e2112_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq68_e2112_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq68_e2112_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq68_e2112_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq68_e2112_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq68_e2112_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq68_e2115: f64 = (s.v[634] * s.v[1016]);
        let eq68_e2115_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq68_e2115_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq68_e2115_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq68_e2115_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq68_e2115_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq68_e2115_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq68_e2115_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq68_e2115_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq68_e2115_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq68_e2115_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq68_e2115_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq68_e2115_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq68_e2115_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq68_e2115_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq68_e2116_q: f64 = eq68_e2115;
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2115);
        let eq68_e2117_d_n0: f64 = (eq68_e2112_d_n0 + eq68_e2115_d_n0);
        let eq68_e2117_d_n1: f64 = (eq68_e2112_d_n1 + eq68_e2115_d_n1);
        let eq68_e2117_d_n2: f64 = (eq68_e2112_d_n2 + eq68_e2115_d_n2);
        let eq68_e2117_d_n3: f64 = (eq68_e2112_d_n3 + eq68_e2115_d_n3);
        let eq68_e2117_d_n4: f64 = (eq68_e2112_d_n4 + eq68_e2115_d_n4);
        let eq68_e2117_d_n5: f64 = (eq68_e2112_d_n5 + eq68_e2115_d_n5);
        let eq68_e2117_d_n6: f64 = (eq68_e2112_d_n6 + eq68_e2115_d_n6);
        let eq68_e2117_d_n7: f64 = (eq68_e2112_d_n7 + eq68_e2115_d_n7);
        let eq68_e2117_d_n8: f64 = (eq68_e2112_d_n8 + eq68_e2115_d_n8);
        let eq68_e2117_d_n9: f64 = (eq68_e2112_d_n9 + eq68_e2115_d_n9);
        let eq68_e2117_d_n10: f64 = (eq68_e2112_d_n10 + eq68_e2115_d_n10);
        let eq68_e2117_d_n11: f64 = (eq68_e2112_d_n11 + eq68_e2115_d_n11);
        let eq68_e2117_d_n12: f64 = (eq68_e2112_d_n12 + eq68_e2115_d_n12);
        let eq68_e2117_d_n13: f64 = (eq68_e2112_d_n13 + eq68_e2115_d_n13);
        let eq68_e2117_q: f64 = eq68_e2116_q;
        let eq68_e2119: f64 = (eq68_e2117 - s.v[1017]);
        let eq68_e2119_d_n0: f64 = (eq68_e2117_d_n0 - s.dn[1017][0]);
        let eq68_e2119_d_n1: f64 = (eq68_e2117_d_n1 - s.dn[1017][1]);
        let eq68_e2119_d_n2: f64 = (eq68_e2117_d_n2 - s.dn[1017][2]);
        let eq68_e2119_d_n3: f64 = (eq68_e2117_d_n3 - s.dn[1017][3]);
        let eq68_e2119_d_n4: f64 = (eq68_e2117_d_n4 - s.dn[1017][4]);
        let eq68_e2119_d_n5: f64 = (eq68_e2117_d_n5 - s.dn[1017][5]);
        let eq68_e2119_d_n6: f64 = (eq68_e2117_d_n6 - s.dn[1017][6]);
        let eq68_e2119_d_n7: f64 = (eq68_e2117_d_n7 - s.dn[1017][7]);
        let eq68_e2119_d_n8: f64 = (eq68_e2117_d_n8 - s.dn[1017][8]);
        let eq68_e2119_d_n9: f64 = (eq68_e2117_d_n9 - s.dn[1017][9]);
        let eq68_e2119_d_n10: f64 = (eq68_e2117_d_n10 - s.dn[1017][10]);
        let eq68_e2119_d_n11: f64 = (eq68_e2117_d_n11 - s.dn[1017][11]);
        let eq68_e2119_d_n12: f64 = (eq68_e2117_d_n12 - s.dn[1017][12]);
        let eq68_e2119_d_n13: f64 = (eq68_e2117_d_n13 - s.dn[1017][13]);
        let eq68_e2119_q: f64 = eq68_e2117_q;
        (eq68_e2119, eq68_e2119_d_n0, eq68_e2119_d_n1, eq68_e2119_d_n2, eq68_e2119_d_n3, eq68_e2119_d_n4, eq68_e2119_d_n5, eq68_e2119_d_n6, eq68_e2119_d_n7, eq68_e2119_d_n8, eq68_e2119_d_n9, eq68_e2119_d_n10, eq68_e2119_d_n11, eq68_e2119_d_n12, eq68_e2119_d_n13, eq68_e2119_q, eq68_e2115_d_n0, eq68_e2115_d_n1, eq68_e2115_d_n2, eq68_e2115_d_n3, eq68_e2115_d_n4, eq68_e2115_d_n5, eq68_e2115_d_n6, eq68_e2115_d_n7, eq68_e2115_d_n8, eq68_e2115_d_n9, eq68_e2115_d_n10, eq68_e2115_d_n11, eq68_e2115_d_n12, eq68_e2115_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_reactive_node_derivatives: [f64; 14] = [eq68_e2121_q_d_n0, eq68_e2121_q_d_n1, eq68_e2121_q_d_n2, eq68_e2121_q_d_n3, eq68_e2121_q_d_n4, eq68_e2121_q_d_n5, eq68_e2121_q_d_n6, eq68_e2121_q_d_n7, eq68_e2121_q_d_n8, eq68_e2121_q_d_n9, eq68_e2121_q_d_n10, eq68_e2121_q_d_n11, eq68_e2121_q_d_n12, eq68_e2121_q_d_n13];
        let eq68_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            &nodes,
            &eq68_reactive_node_derivatives,
            &branches,
            &eq68_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_69_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n1, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11, eq69_e2137_d_n12, eq69_e2137_d_n13, eq69_e2137_q, eq69_e2137_q_d_n0, eq69_e2137_q_d_n1, eq69_e2137_q_d_n2, eq69_e2137_q_d_n3, eq69_e2137_q_d_n4, eq69_e2137_q_d_n5, eq69_e2137_q_d_n6, eq69_e2137_q_d_n7, eq69_e2137_q_d_n8, eq69_e2137_q_d_n9, eq69_e2137_q_d_n10, eq69_e2137_q_d_n11, eq69_e2137_q_d_n12, eq69_e2137_q_d_n13,) = {
    if ((s.v[2021] != 0.0) && (!(s.v[2024] != 0.0))) {
        let eq69_e2128: f64 = (s.v[634] * s.v[1015]);
        let eq69_e2128_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq69_e2128_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq69_e2128_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq69_e2128_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq69_e2128_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq69_e2128_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq69_e2128_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq69_e2128_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq69_e2128_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq69_e2128_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq69_e2128_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq69_e2128_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq69_e2128_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq69_e2128_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq69_e2131: f64 = (s.v[634] * s.v[1016]);
        let eq69_e2131_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq69_e2131_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq69_e2131_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq69_e2131_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq69_e2131_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq69_e2131_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq69_e2131_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq69_e2131_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq69_e2131_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq69_e2131_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq69_e2131_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq69_e2131_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq69_e2131_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq69_e2131_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq69_e2132_q: f64 = eq69_e2131;
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2131);
        let eq69_e2133_d_n0: f64 = (eq69_e2128_d_n0 + eq69_e2131_d_n0);
        let eq69_e2133_d_n1: f64 = (eq69_e2128_d_n1 + eq69_e2131_d_n1);
        let eq69_e2133_d_n2: f64 = (eq69_e2128_d_n2 + eq69_e2131_d_n2);
        let eq69_e2133_d_n3: f64 = (eq69_e2128_d_n3 + eq69_e2131_d_n3);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + eq69_e2131_d_n4);
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + eq69_e2131_d_n5);
        let eq69_e2133_d_n6: f64 = (eq69_e2128_d_n6 + eq69_e2131_d_n6);
        let eq69_e2133_d_n7: f64 = (eq69_e2128_d_n7 + eq69_e2131_d_n7);
        let eq69_e2133_d_n8: f64 = (eq69_e2128_d_n8 + eq69_e2131_d_n8);
        let eq69_e2133_d_n9: f64 = (eq69_e2128_d_n9 + eq69_e2131_d_n9);
        let eq69_e2133_d_n10: f64 = (eq69_e2128_d_n10 + eq69_e2131_d_n10);
        let eq69_e2133_d_n11: f64 = (eq69_e2128_d_n11 + eq69_e2131_d_n11);
        let eq69_e2133_d_n12: f64 = (eq69_e2128_d_n12 + eq69_e2131_d_n12);
        let eq69_e2133_d_n13: f64 = (eq69_e2128_d_n13 + eq69_e2131_d_n13);
        let eq69_e2133_q: f64 = eq69_e2132_q;
        let eq69_e2135: f64 = (eq69_e2133 - s.v[1017]);
        let eq69_e2135_d_n0: f64 = (eq69_e2133_d_n0 - s.dn[1017][0]);
        let eq69_e2135_d_n1: f64 = (eq69_e2133_d_n1 - s.dn[1017][1]);
        let eq69_e2135_d_n2: f64 = (eq69_e2133_d_n2 - s.dn[1017][2]);
        let eq69_e2135_d_n3: f64 = (eq69_e2133_d_n3 - s.dn[1017][3]);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - s.dn[1017][4]);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - s.dn[1017][5]);
        let eq69_e2135_d_n6: f64 = (eq69_e2133_d_n6 - s.dn[1017][6]);
        let eq69_e2135_d_n7: f64 = (eq69_e2133_d_n7 - s.dn[1017][7]);
        let eq69_e2135_d_n8: f64 = (eq69_e2133_d_n8 - s.dn[1017][8]);
        let eq69_e2135_d_n9: f64 = (eq69_e2133_d_n9 - s.dn[1017][9]);
        let eq69_e2135_d_n10: f64 = (eq69_e2133_d_n10 - s.dn[1017][10]);
        let eq69_e2135_d_n11: f64 = (eq69_e2133_d_n11 - s.dn[1017][11]);
        let eq69_e2135_d_n12: f64 = (eq69_e2133_d_n12 - s.dn[1017][12]);
        let eq69_e2135_d_n13: f64 = (eq69_e2133_d_n13 - s.dn[1017][13]);
        let eq69_e2135_q: f64 = eq69_e2133_q;
        (eq69_e2135, eq69_e2135_d_n0, eq69_e2135_d_n1, eq69_e2135_d_n2, eq69_e2135_d_n3, eq69_e2135_d_n4, eq69_e2135_d_n5, eq69_e2135_d_n6, eq69_e2135_d_n7, eq69_e2135_d_n8, eq69_e2135_d_n9, eq69_e2135_d_n10, eq69_e2135_d_n11, eq69_e2135_d_n12, eq69_e2135_d_n13, eq69_e2135_q, eq69_e2131_d_n0, eq69_e2131_d_n1, eq69_e2131_d_n2, eq69_e2131_d_n3, eq69_e2131_d_n4, eq69_e2131_d_n5, eq69_e2131_d_n6, eq69_e2131_d_n7, eq69_e2131_d_n8, eq69_e2131_d_n9, eq69_e2131_d_n10, eq69_e2131_d_n11, eq69_e2131_d_n12, eq69_e2131_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_reactive_node_derivatives: [f64; 14] = [eq69_e2137_q_d_n0, eq69_e2137_q_d_n1, eq69_e2137_q_d_n2, eq69_e2137_q_d_n3, eq69_e2137_q_d_n4, eq69_e2137_q_d_n5, eq69_e2137_q_d_n6, eq69_e2137_q_d_n7, eq69_e2137_q_d_n8, eq69_e2137_q_d_n9, eq69_e2137_q_d_n10, eq69_e2137_q_d_n11, eq69_e2137_q_d_n12, eq69_e2137_q_d_n13];
        let eq69_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            &nodes,
            &eq69_reactive_node_derivatives,
            &branches,
            &eq69_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_80_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq80_e2212_q: f64 = s.v[520];
        let eq80_e2213: f64 = (s.v[379] * s.v[520]);
        let eq80_e2213_d_n0: f64 = ((s.dn[379][0] * s.v[520]) + (s.v[379] * s.dn[520][0]));
        let eq80_e2213_d_n1: f64 = ((s.dn[379][1] * s.v[520]) + (s.v[379] * s.dn[520][1]));
        let eq80_e2213_d_n2: f64 = ((s.dn[379][2] * s.v[520]) + (s.v[379] * s.dn[520][2]));
        let eq80_e2213_d_n3: f64 = ((s.dn[379][3] * s.v[520]) + (s.v[379] * s.dn[520][3]));
        let eq80_e2213_d_n4: f64 = ((s.dn[379][4] * s.v[520]) + (s.v[379] * s.dn[520][4]));
        let eq80_e2213_d_n5: f64 = ((s.dn[379][5] * s.v[520]) + (s.v[379] * s.dn[520][5]));
        let eq80_e2213_d_n6: f64 = ((s.dn[379][6] * s.v[520]) + (s.v[379] * s.dn[520][6]));
        let eq80_e2213_d_n7: f64 = ((s.dn[379][7] * s.v[520]) + (s.v[379] * s.dn[520][7]));
        let eq80_e2213_d_n8: f64 = ((s.dn[379][8] * s.v[520]) + (s.v[379] * s.dn[520][8]));
        let eq80_e2213_d_n9: f64 = ((s.dn[379][9] * s.v[520]) + (s.v[379] * s.dn[520][9]));
        let eq80_e2213_d_n10: f64 = ((s.dn[379][10] * s.v[520]) + (s.v[379] * s.dn[520][10]));
        let eq80_e2213_d_n11: f64 = ((s.dn[379][11] * s.v[520]) + (s.v[379] * s.dn[520][11]));
        let eq80_e2213_d_n12: f64 = ((s.dn[379][12] * s.v[520]) + (s.v[379] * s.dn[520][12]));
        let eq80_e2213_d_n13: f64 = ((s.dn[379][13] * s.v[520]) + (s.v[379] * s.dn[520][13]));
        let eq80_e2213_q: f64 = (s.v[379] * eq80_e2212_q);
        let eq80_e2213_q_d_n0: f64 = ((s.dn[379][0] * eq80_e2212_q) + (s.v[379] * s.dn[520][0]));
        let eq80_e2213_q_d_n1: f64 = ((s.dn[379][1] * eq80_e2212_q) + (s.v[379] * s.dn[520][1]));
        let eq80_e2213_q_d_n2: f64 = ((s.dn[379][2] * eq80_e2212_q) + (s.v[379] * s.dn[520][2]));
        let eq80_e2213_q_d_n3: f64 = ((s.dn[379][3] * eq80_e2212_q) + (s.v[379] * s.dn[520][3]));
        let eq80_e2213_q_d_n4: f64 = ((s.dn[379][4] * eq80_e2212_q) + (s.v[379] * s.dn[520][4]));
        let eq80_e2213_q_d_n5: f64 = ((s.dn[379][5] * eq80_e2212_q) + (s.v[379] * s.dn[520][5]));
        let eq80_e2213_q_d_n6: f64 = ((s.dn[379][6] * eq80_e2212_q) + (s.v[379] * s.dn[520][6]));
        let eq80_e2213_q_d_n7: f64 = ((s.dn[379][7] * eq80_e2212_q) + (s.v[379] * s.dn[520][7]));
        let eq80_e2213_q_d_n8: f64 = ((s.dn[379][8] * eq80_e2212_q) + (s.v[379] * s.dn[520][8]));
        let eq80_e2213_q_d_n9: f64 = ((s.dn[379][9] * eq80_e2212_q) + (s.v[379] * s.dn[520][9]));
        let eq80_e2213_q_d_n10: f64 = ((s.dn[379][10] * eq80_e2212_q) + (s.v[379] * s.dn[520][10]));
        let eq80_e2213_q_d_n11: f64 = ((s.dn[379][11] * eq80_e2212_q) + (s.v[379] * s.dn[520][11]));
        let eq80_e2213_q_d_n12: f64 = ((s.dn[379][12] * eq80_e2212_q) + (s.v[379] * s.dn[520][12]));
        let eq80_e2213_q_d_n13: f64 = ((s.dn[379][13] * eq80_e2212_q) + (s.v[379] * s.dn[520][13]));
        let eq80_reactive_node_derivatives: [f64; 14] = [eq80_e2213_q_d_n0, eq80_e2213_q_d_n1, eq80_e2213_q_d_n2, eq80_e2213_q_d_n3, eq80_e2213_q_d_n4, eq80_e2213_q_d_n5, eq80_e2213_q_d_n6, eq80_e2213_q_d_n7, eq80_e2213_q_d_n8, eq80_e2213_q_d_n9, eq80_e2213_q_d_n10, eq80_e2213_q_d_n11, eq80_e2213_q_d_n12, eq80_e2213_q_d_n13];
        let eq80_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            &nodes,
            &eq80_reactive_node_derivatives,
            &branches,
            &eq80_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
