#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq12_e249: f64 = (s.v[207] + s.v[210]);
        let eq12_e249_d_n0: f64 = (s.dn[207][0] + s.dn[210][0]);
        let eq12_e249_d_n1: f64 = (s.dn[207][1] + s.dn[210][1]);
        let eq12_e249_d_n2: f64 = (s.dn[207][2] + s.dn[210][2]);
        let eq12_e249_d_n3: f64 = (s.dn[207][3] + s.dn[210][3]);
        let eq12_e249_d_n4: f64 = (s.dn[207][4] + s.dn[210][4]);
        let eq12_e249_d_n5: f64 = (s.dn[207][5] + s.dn[210][5]);
        let eq12_e249_d_n6: f64 = (s.dn[207][6] + s.dn[210][6]);
        let eq12_e249_d_n7: f64 = (s.dn[207][7] + s.dn[210][7]);
        let eq12_e249_d_n8: f64 = (s.dn[207][8] + s.dn[210][8]);
        let eq12_e249_d_n9: f64 = (s.dn[207][9] + s.dn[210][9]);
        let eq12_e249_d_n10: f64 = (s.dn[207][10] + s.dn[210][10]);
        let eq12_e251: f64 = (eq12_e249 + s.v[224]);
        let eq12_e251_d_n0: f64 = (eq12_e249_d_n0 + s.dn[224][0]);
        let eq12_e251_d_n1: f64 = (eq12_e249_d_n1 + s.dn[224][1]);
        let eq12_e251_d_n2: f64 = (eq12_e249_d_n2 + s.dn[224][2]);
        let eq12_e251_d_n3: f64 = (eq12_e249_d_n3 + s.dn[224][3]);
        let eq12_e251_d_n4: f64 = (eq12_e249_d_n4 + s.dn[224][4]);
        let eq12_e251_d_n5: f64 = (eq12_e249_d_n5 + s.dn[224][5]);
        let eq12_e251_d_n6: f64 = (eq12_e249_d_n6 + s.dn[224][6]);
        let eq12_e251_d_n7: f64 = (eq12_e249_d_n7 + s.dn[224][7]);
        let eq12_e251_d_n8: f64 = (eq12_e249_d_n8 + s.dn[224][8]);
        let eq12_e251_d_n9: f64 = (eq12_e249_d_n9 + s.dn[224][9]);
        let eq12_e251_d_n10: f64 = (eq12_e249_d_n10 + s.dn[224][10]);
        let eq12_e252: f64 = (p.p3 * eq12_e251);
        let eq12_e252_d_n0: f64 = (p.p3 * eq12_e251_d_n0);
        let eq12_e252_d_n1: f64 = (p.p3 * eq12_e251_d_n1);
        let eq12_e252_d_n2: f64 = (p.p3 * eq12_e251_d_n2);
        let eq12_e252_d_n3: f64 = (p.p3 * eq12_e251_d_n3);
        let eq12_e252_d_n4: f64 = (p.p3 * eq12_e251_d_n4);
        let eq12_e252_d_n5: f64 = (p.p3 * eq12_e251_d_n5);
        let eq12_e252_d_n6: f64 = (p.p3 * eq12_e251_d_n6);
        let eq12_e252_d_n7: f64 = (p.p3 * eq12_e251_d_n7);
        let eq12_e252_d_n8: f64 = (p.p3 * eq12_e251_d_n8);
        let eq12_e252_d_n9: f64 = (p.p3 * eq12_e251_d_n9);
        let eq12_e252_d_n10: f64 = (p.p3 * eq12_e251_d_n10);
        let eq12_e253_q: f64 = eq12_e252;
        let eq12_e255: f64 = (eq12_e252 * p.p1);
        let eq12_e255_d_n0: f64 = (eq12_e252_d_n0 * p.p1);
        let eq12_e255_d_n1: f64 = (eq12_e252_d_n1 * p.p1);
        let eq12_e255_d_n2: f64 = (eq12_e252_d_n2 * p.p1);
        let eq12_e255_d_n3: f64 = (eq12_e252_d_n3 * p.p1);
        let eq12_e255_d_n4: f64 = (eq12_e252_d_n4 * p.p1);
        let eq12_e255_d_n5: f64 = (eq12_e252_d_n5 * p.p1);
        let eq12_e255_d_n6: f64 = (eq12_e252_d_n6 * p.p1);
        let eq12_e255_d_n7: f64 = (eq12_e252_d_n7 * p.p1);
        let eq12_e255_d_n8: f64 = (eq12_e252_d_n8 * p.p1);
        let eq12_e255_d_n9: f64 = (eq12_e252_d_n9 * p.p1);
        let eq12_e255_d_n10: f64 = (eq12_e252_d_n10 * p.p1);
        let eq12_e255_q: f64 = (eq12_e253_q * p.p1);
        let eq12_e255_q_d_n0: f64 = (eq12_e252_d_n0 * p.p1);
        let eq12_e255_q_d_n1: f64 = (eq12_e252_d_n1 * p.p1);
        let eq12_e255_q_d_n2: f64 = (eq12_e252_d_n2 * p.p1);
        let eq12_e255_q_d_n3: f64 = (eq12_e252_d_n3 * p.p1);
        let eq12_e255_q_d_n4: f64 = (eq12_e252_d_n4 * p.p1);
        let eq12_e255_q_d_n5: f64 = (eq12_e252_d_n5 * p.p1);
        let eq12_e255_q_d_n6: f64 = (eq12_e252_d_n6 * p.p1);
        let eq12_e255_q_d_n7: f64 = (eq12_e252_d_n7 * p.p1);
        let eq12_e255_q_d_n8: f64 = (eq12_e252_d_n8 * p.p1);
        let eq12_e255_q_d_n9: f64 = (eq12_e252_d_n9 * p.p1);
        let eq12_e255_q_d_n10: f64 = (eq12_e252_d_n10 * p.p1);
        let eq12_reactive_node_derivatives: [f64; 11] = [eq12_e255_q_d_n0, eq12_e255_q_d_n1, eq12_e255_q_d_n2, eq12_e255_q_d_n3, eq12_e255_q_d_n4, eq12_e255_q_d_n5, eq12_e255_q_d_n6, eq12_e255_q_d_n7, eq12_e255_q_d_n8, eq12_e255_q_d_n9, eq12_e255_q_d_n10];
        let eq12_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            &nodes,
            &eq12_reactive_node_derivatives,
            &branches,
            &eq12_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq13_e258: f64 = (p.p3 * s.v[211]);
        let eq13_e258_d_n0: f64 = (p.p3 * s.dn[211][0]);
        let eq13_e258_d_n1: f64 = (p.p3 * s.dn[211][1]);
        let eq13_e258_d_n2: f64 = (p.p3 * s.dn[211][2]);
        let eq13_e258_d_n3: f64 = (p.p3 * s.dn[211][3]);
        let eq13_e258_d_n4: f64 = (p.p3 * s.dn[211][4]);
        let eq13_e258_d_n5: f64 = (p.p3 * s.dn[211][5]);
        let eq13_e258_d_n6: f64 = (p.p3 * s.dn[211][6]);
        let eq13_e258_d_n7: f64 = (p.p3 * s.dn[211][7]);
        let eq13_e258_d_n8: f64 = (p.p3 * s.dn[211][8]);
        let eq13_e258_d_n9: f64 = (p.p3 * s.dn[211][9]);
        let eq13_e258_d_n10: f64 = (p.p3 * s.dn[211][10]);
        let eq13_e259_q: f64 = eq13_e258;
        let eq13_e261: f64 = (eq13_e258 * p.p1);
        let eq13_e261_d_n0: f64 = (eq13_e258_d_n0 * p.p1);
        let eq13_e261_d_n1: f64 = (eq13_e258_d_n1 * p.p1);
        let eq13_e261_d_n2: f64 = (eq13_e258_d_n2 * p.p1);
        let eq13_e261_d_n3: f64 = (eq13_e258_d_n3 * p.p1);
        let eq13_e261_d_n4: f64 = (eq13_e258_d_n4 * p.p1);
        let eq13_e261_d_n5: f64 = (eq13_e258_d_n5 * p.p1);
        let eq13_e261_d_n6: f64 = (eq13_e258_d_n6 * p.p1);
        let eq13_e261_d_n7: f64 = (eq13_e258_d_n7 * p.p1);
        let eq13_e261_d_n8: f64 = (eq13_e258_d_n8 * p.p1);
        let eq13_e261_d_n9: f64 = (eq13_e258_d_n9 * p.p1);
        let eq13_e261_d_n10: f64 = (eq13_e258_d_n10 * p.p1);
        let eq13_e261_q: f64 = (eq13_e259_q * p.p1);
        let eq13_e261_q_d_n0: f64 = (eq13_e258_d_n0 * p.p1);
        let eq13_e261_q_d_n1: f64 = (eq13_e258_d_n1 * p.p1);
        let eq13_e261_q_d_n2: f64 = (eq13_e258_d_n2 * p.p1);
        let eq13_e261_q_d_n3: f64 = (eq13_e258_d_n3 * p.p1);
        let eq13_e261_q_d_n4: f64 = (eq13_e258_d_n4 * p.p1);
        let eq13_e261_q_d_n5: f64 = (eq13_e258_d_n5 * p.p1);
        let eq13_e261_q_d_n6: f64 = (eq13_e258_d_n6 * p.p1);
        let eq13_e261_q_d_n7: f64 = (eq13_e258_d_n7 * p.p1);
        let eq13_e261_q_d_n8: f64 = (eq13_e258_d_n8 * p.p1);
        let eq13_e261_q_d_n9: f64 = (eq13_e258_d_n9 * p.p1);
        let eq13_e261_q_d_n10: f64 = (eq13_e258_d_n10 * p.p1);
        let eq13_reactive_node_derivatives: [f64; 11] = [eq13_e261_q_d_n0, eq13_e261_q_d_n1, eq13_e261_q_d_n2, eq13_e261_q_d_n3, eq13_e261_q_d_n4, eq13_e261_q_d_n5, eq13_e261_q_d_n6, eq13_e261_q_d_n7, eq13_e261_q_d_n8, eq13_e261_q_d_n9, eq13_e261_q_d_n10];
        let eq13_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            &nodes,
            &eq13_reactive_node_derivatives,
            &branches,
            &eq13_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq14_e264: f64 = (p.p3 * p.p68);
        let eq14_e266: f64 = (eq14_e264 * s.v[243]);
        let eq14_e266_d_n0: f64 = (eq14_e264 * s.dn[243][0]);
        let eq14_e266_d_n1: f64 = (eq14_e264 * s.dn[243][1]);
        let eq14_e266_d_n2: f64 = (eq14_e264 * s.dn[243][2]);
        let eq14_e266_d_n3: f64 = (eq14_e264 * s.dn[243][3]);
        let eq14_e266_d_n4: f64 = (eq14_e264 * s.dn[243][4]);
        let eq14_e266_d_n5: f64 = (eq14_e264 * s.dn[243][5]);
        let eq14_e266_d_n6: f64 = (eq14_e264 * s.dn[243][6]);
        let eq14_e266_d_n7: f64 = (eq14_e264 * s.dn[243][7]);
        let eq14_e266_d_n8: f64 = (eq14_e264 * s.dn[243][8]);
        let eq14_e266_d_n9: f64 = (eq14_e264 * s.dn[243][9]);
        let eq14_e266_d_n10: f64 = (eq14_e264 * s.dn[243][10]);
        let eq14_e267_q: f64 = eq14_e266;
        let eq14_e269: f64 = (eq14_e266 * p.p1);
        let eq14_e269_d_n0: f64 = (eq14_e266_d_n0 * p.p1);
        let eq14_e269_d_n1: f64 = (eq14_e266_d_n1 * p.p1);
        let eq14_e269_d_n2: f64 = (eq14_e266_d_n2 * p.p1);
        let eq14_e269_d_n3: f64 = (eq14_e266_d_n3 * p.p1);
        let eq14_e269_d_n4: f64 = (eq14_e266_d_n4 * p.p1);
        let eq14_e269_d_n5: f64 = (eq14_e266_d_n5 * p.p1);
        let eq14_e269_d_n6: f64 = (eq14_e266_d_n6 * p.p1);
        let eq14_e269_d_n7: f64 = (eq14_e266_d_n7 * p.p1);
        let eq14_e269_d_n8: f64 = (eq14_e266_d_n8 * p.p1);
        let eq14_e269_d_n9: f64 = (eq14_e266_d_n9 * p.p1);
        let eq14_e269_d_n10: f64 = (eq14_e266_d_n10 * p.p1);
        let eq14_e269_q: f64 = (eq14_e267_q * p.p1);
        let eq14_e269_q_d_n0: f64 = (eq14_e266_d_n0 * p.p1);
        let eq14_e269_q_d_n1: f64 = (eq14_e266_d_n1 * p.p1);
        let eq14_e269_q_d_n2: f64 = (eq14_e266_d_n2 * p.p1);
        let eq14_e269_q_d_n3: f64 = (eq14_e266_d_n3 * p.p1);
        let eq14_e269_q_d_n4: f64 = (eq14_e266_d_n4 * p.p1);
        let eq14_e269_q_d_n5: f64 = (eq14_e266_d_n5 * p.p1);
        let eq14_e269_q_d_n6: f64 = (eq14_e266_d_n6 * p.p1);
        let eq14_e269_q_d_n7: f64 = (eq14_e266_d_n7 * p.p1);
        let eq14_e269_q_d_n8: f64 = (eq14_e266_d_n8 * p.p1);
        let eq14_e269_q_d_n9: f64 = (eq14_e266_d_n9 * p.p1);
        let eq14_e269_q_d_n10: f64 = (eq14_e266_d_n10 * p.p1);
        let eq14_reactive_node_derivatives: [f64; 11] = [eq14_e269_q_d_n0, eq14_e269_q_d_n1, eq14_e269_q_d_n2, eq14_e269_q_d_n3, eq14_e269_q_d_n4, eq14_e269_q_d_n5, eq14_e269_q_d_n6, eq14_e269_q_d_n7, eq14_e269_q_d_n8, eq14_e269_q_d_n9, eq14_e269_q_d_n10];
        let eq14_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            &nodes,
            &eq14_reactive_node_derivatives,
            &branches,
            &eq14_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq15_e272: f64 = (p.p3 * p.p77);
        let eq15_e274: f64 = (eq15_e272 * s.v[244]);
        let eq15_e274_d_n0: f64 = (eq15_e272 * s.dn[244][0]);
        let eq15_e274_d_n1: f64 = (eq15_e272 * s.dn[244][1]);
        let eq15_e274_d_n2: f64 = (eq15_e272 * s.dn[244][2]);
        let eq15_e274_d_n3: f64 = (eq15_e272 * s.dn[244][3]);
        let eq15_e274_d_n4: f64 = (eq15_e272 * s.dn[244][4]);
        let eq15_e274_d_n5: f64 = (eq15_e272 * s.dn[244][5]);
        let eq15_e274_d_n6: f64 = (eq15_e272 * s.dn[244][6]);
        let eq15_e274_d_n7: f64 = (eq15_e272 * s.dn[244][7]);
        let eq15_e274_d_n8: f64 = (eq15_e272 * s.dn[244][8]);
        let eq15_e274_d_n9: f64 = (eq15_e272 * s.dn[244][9]);
        let eq15_e274_d_n10: f64 = (eq15_e272 * s.dn[244][10]);
        let eq15_e275_q: f64 = eq15_e274;
        let eq15_e277: f64 = (eq15_e274 * p.p1);
        let eq15_e277_d_n0: f64 = (eq15_e274_d_n0 * p.p1);
        let eq15_e277_d_n1: f64 = (eq15_e274_d_n1 * p.p1);
        let eq15_e277_d_n2: f64 = (eq15_e274_d_n2 * p.p1);
        let eq15_e277_d_n3: f64 = (eq15_e274_d_n3 * p.p1);
        let eq15_e277_d_n4: f64 = (eq15_e274_d_n4 * p.p1);
        let eq15_e277_d_n5: f64 = (eq15_e274_d_n5 * p.p1);
        let eq15_e277_d_n6: f64 = (eq15_e274_d_n6 * p.p1);
        let eq15_e277_d_n7: f64 = (eq15_e274_d_n7 * p.p1);
        let eq15_e277_d_n8: f64 = (eq15_e274_d_n8 * p.p1);
        let eq15_e277_d_n9: f64 = (eq15_e274_d_n9 * p.p1);
        let eq15_e277_d_n10: f64 = (eq15_e274_d_n10 * p.p1);
        let eq15_e277_q: f64 = (eq15_e275_q * p.p1);
        let eq15_e277_q_d_n0: f64 = (eq15_e274_d_n0 * p.p1);
        let eq15_e277_q_d_n1: f64 = (eq15_e274_d_n1 * p.p1);
        let eq15_e277_q_d_n2: f64 = (eq15_e274_d_n2 * p.p1);
        let eq15_e277_q_d_n3: f64 = (eq15_e274_d_n3 * p.p1);
        let eq15_e277_q_d_n4: f64 = (eq15_e274_d_n4 * p.p1);
        let eq15_e277_q_d_n5: f64 = (eq15_e274_d_n5 * p.p1);
        let eq15_e277_q_d_n6: f64 = (eq15_e274_d_n6 * p.p1);
        let eq15_e277_q_d_n7: f64 = (eq15_e274_d_n7 * p.p1);
        let eq15_e277_q_d_n8: f64 = (eq15_e274_d_n8 * p.p1);
        let eq15_e277_q_d_n9: f64 = (eq15_e274_d_n9 * p.p1);
        let eq15_e277_q_d_n10: f64 = (eq15_e274_d_n10 * p.p1);
        let eq15_reactive_node_derivatives: [f64; 11] = [eq15_e277_q_d_n0, eq15_e277_q_d_n1, eq15_e277_q_d_n2, eq15_e277_q_d_n3, eq15_e277_q_d_n4, eq15_e277_q_d_n5, eq15_e277_q_d_n6, eq15_e277_q_d_n7, eq15_e277_q_d_n8, eq15_e277_q_d_n9, eq15_e277_q_d_n10];
        let eq15_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            &nodes,
            &eq15_reactive_node_derivatives,
            &branches,
            &eq15_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq18_e293: f64 = (s.v[219] + s.v[228]);
        let eq18_e293_d_n0: f64 = (s.dn[219][0] + s.dn[228][0]);
        let eq18_e293_d_n1: f64 = (s.dn[219][1] + s.dn[228][1]);
        let eq18_e293_d_n2: f64 = (s.dn[219][2] + s.dn[228][2]);
        let eq18_e293_d_n3: f64 = (s.dn[219][3] + s.dn[228][3]);
        let eq18_e293_d_n4: f64 = (s.dn[219][4] + s.dn[228][4]);
        let eq18_e293_d_n5: f64 = (s.dn[219][5] + s.dn[228][5]);
        let eq18_e293_d_n6: f64 = (s.dn[219][6] + s.dn[228][6]);
        let eq18_e293_d_n7: f64 = (s.dn[219][7] + s.dn[228][7]);
        let eq18_e293_d_n8: f64 = (s.dn[219][8] + s.dn[228][8]);
        let eq18_e293_d_n9: f64 = (s.dn[219][9] + s.dn[228][9]);
        let eq18_e293_d_n10: f64 = (s.dn[219][10] + s.dn[228][10]);
        let eq18_e294: f64 = (p.p3 * eq18_e293);
        let eq18_e294_d_n0: f64 = (p.p3 * eq18_e293_d_n0);
        let eq18_e294_d_n1: f64 = (p.p3 * eq18_e293_d_n1);
        let eq18_e294_d_n2: f64 = (p.p3 * eq18_e293_d_n2);
        let eq18_e294_d_n3: f64 = (p.p3 * eq18_e293_d_n3);
        let eq18_e294_d_n4: f64 = (p.p3 * eq18_e293_d_n4);
        let eq18_e294_d_n5: f64 = (p.p3 * eq18_e293_d_n5);
        let eq18_e294_d_n6: f64 = (p.p3 * eq18_e293_d_n6);
        let eq18_e294_d_n7: f64 = (p.p3 * eq18_e293_d_n7);
        let eq18_e294_d_n8: f64 = (p.p3 * eq18_e293_d_n8);
        let eq18_e294_d_n9: f64 = (p.p3 * eq18_e293_d_n9);
        let eq18_e294_d_n10: f64 = (p.p3 * eq18_e293_d_n10);
        let eq18_e295_q: f64 = eq18_e294;
        let eq18_e297: f64 = (eq18_e294 * p.p1);
        let eq18_e297_d_n0: f64 = (eq18_e294_d_n0 * p.p1);
        let eq18_e297_d_n1: f64 = (eq18_e294_d_n1 * p.p1);
        let eq18_e297_d_n2: f64 = (eq18_e294_d_n2 * p.p1);
        let eq18_e297_d_n3: f64 = (eq18_e294_d_n3 * p.p1);
        let eq18_e297_d_n4: f64 = (eq18_e294_d_n4 * p.p1);
        let eq18_e297_d_n5: f64 = (eq18_e294_d_n5 * p.p1);
        let eq18_e297_d_n6: f64 = (eq18_e294_d_n6 * p.p1);
        let eq18_e297_d_n7: f64 = (eq18_e294_d_n7 * p.p1);
        let eq18_e297_d_n8: f64 = (eq18_e294_d_n8 * p.p1);
        let eq18_e297_d_n9: f64 = (eq18_e294_d_n9 * p.p1);
        let eq18_e297_d_n10: f64 = (eq18_e294_d_n10 * p.p1);
        let eq18_e297_q: f64 = (eq18_e295_q * p.p1);
        let eq18_e297_q_d_n0: f64 = (eq18_e294_d_n0 * p.p1);
        let eq18_e297_q_d_n1: f64 = (eq18_e294_d_n1 * p.p1);
        let eq18_e297_q_d_n2: f64 = (eq18_e294_d_n2 * p.p1);
        let eq18_e297_q_d_n3: f64 = (eq18_e294_d_n3 * p.p1);
        let eq18_e297_q_d_n4: f64 = (eq18_e294_d_n4 * p.p1);
        let eq18_e297_q_d_n5: f64 = (eq18_e294_d_n5 * p.p1);
        let eq18_e297_q_d_n6: f64 = (eq18_e294_d_n6 * p.p1);
        let eq18_e297_q_d_n7: f64 = (eq18_e294_d_n7 * p.p1);
        let eq18_e297_q_d_n8: f64 = (eq18_e294_d_n8 * p.p1);
        let eq18_e297_q_d_n9: f64 = (eq18_e294_d_n9 * p.p1);
        let eq18_e297_q_d_n10: f64 = (eq18_e294_d_n10 * p.p1);
        let eq18_reactive_node_derivatives: [f64; 11] = [eq18_e297_q_d_n0, eq18_e297_q_d_n1, eq18_e297_q_d_n2, eq18_e297_q_d_n3, eq18_e297_q_d_n4, eq18_e297_q_d_n5, eq18_e297_q_d_n6, eq18_e297_q_d_n7, eq18_e297_q_d_n8, eq18_e297_q_d_n9, eq18_e297_q_d_n10];
        let eq18_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[8]),
            &nodes,
            &eq18_reactive_node_derivatives,
            &branches,
            &eq18_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq20_e312: f64 = (s.v[216] + s.v[229]);
        let eq20_e312_d_n0: f64 = (s.dn[216][0] + s.dn[229][0]);
        let eq20_e312_d_n1: f64 = (s.dn[216][1] + s.dn[229][1]);
        let eq20_e312_d_n2: f64 = (s.dn[216][2] + s.dn[229][2]);
        let eq20_e312_d_n3: f64 = (s.dn[216][3] + s.dn[229][3]);
        let eq20_e312_d_n4: f64 = (s.dn[216][4] + s.dn[229][4]);
        let eq20_e312_d_n5: f64 = (s.dn[216][5] + s.dn[229][5]);
        let eq20_e312_d_n6: f64 = (s.dn[216][6] + s.dn[229][6]);
        let eq20_e312_d_n7: f64 = (s.dn[216][7] + s.dn[229][7]);
        let eq20_e312_d_n8: f64 = (s.dn[216][8] + s.dn[229][8]);
        let eq20_e312_d_n9: f64 = (s.dn[216][9] + s.dn[229][9]);
        let eq20_e312_d_n10: f64 = (s.dn[216][10] + s.dn[229][10]);
        let eq20_e313: f64 = (p.p3 * eq20_e312);
        let eq20_e313_d_n0: f64 = (p.p3 * eq20_e312_d_n0);
        let eq20_e313_d_n1: f64 = (p.p3 * eq20_e312_d_n1);
        let eq20_e313_d_n2: f64 = (p.p3 * eq20_e312_d_n2);
        let eq20_e313_d_n3: f64 = (p.p3 * eq20_e312_d_n3);
        let eq20_e313_d_n4: f64 = (p.p3 * eq20_e312_d_n4);
        let eq20_e313_d_n5: f64 = (p.p3 * eq20_e312_d_n5);
        let eq20_e313_d_n6: f64 = (p.p3 * eq20_e312_d_n6);
        let eq20_e313_d_n7: f64 = (p.p3 * eq20_e312_d_n7);
        let eq20_e313_d_n8: f64 = (p.p3 * eq20_e312_d_n8);
        let eq20_e313_d_n9: f64 = (p.p3 * eq20_e312_d_n9);
        let eq20_e313_d_n10: f64 = (p.p3 * eq20_e312_d_n10);
        let eq20_e314_q: f64 = eq20_e313;
        let eq20_e316: f64 = (eq20_e313 * p.p1);
        let eq20_e316_d_n0: f64 = (eq20_e313_d_n0 * p.p1);
        let eq20_e316_d_n1: f64 = (eq20_e313_d_n1 * p.p1);
        let eq20_e316_d_n2: f64 = (eq20_e313_d_n2 * p.p1);
        let eq20_e316_d_n3: f64 = (eq20_e313_d_n3 * p.p1);
        let eq20_e316_d_n4: f64 = (eq20_e313_d_n4 * p.p1);
        let eq20_e316_d_n5: f64 = (eq20_e313_d_n5 * p.p1);
        let eq20_e316_d_n6: f64 = (eq20_e313_d_n6 * p.p1);
        let eq20_e316_d_n7: f64 = (eq20_e313_d_n7 * p.p1);
        let eq20_e316_d_n8: f64 = (eq20_e313_d_n8 * p.p1);
        let eq20_e316_d_n9: f64 = (eq20_e313_d_n9 * p.p1);
        let eq20_e316_d_n10: f64 = (eq20_e313_d_n10 * p.p1);
        let eq20_e316_q: f64 = (eq20_e314_q * p.p1);
        let eq20_e316_q_d_n0: f64 = (eq20_e313_d_n0 * p.p1);
        let eq20_e316_q_d_n1: f64 = (eq20_e313_d_n1 * p.p1);
        let eq20_e316_q_d_n2: f64 = (eq20_e313_d_n2 * p.p1);
        let eq20_e316_q_d_n3: f64 = (eq20_e313_d_n3 * p.p1);
        let eq20_e316_q_d_n4: f64 = (eq20_e313_d_n4 * p.p1);
        let eq20_e316_q_d_n5: f64 = (eq20_e313_d_n5 * p.p1);
        let eq20_e316_q_d_n6: f64 = (eq20_e313_d_n6 * p.p1);
        let eq20_e316_q_d_n7: f64 = (eq20_e313_d_n7 * p.p1);
        let eq20_e316_q_d_n8: f64 = (eq20_e313_d_n8 * p.p1);
        let eq20_e316_q_d_n9: f64 = (eq20_e313_d_n9 * p.p1);
        let eq20_e316_q_d_n10: f64 = (eq20_e313_d_n10 * p.p1);
        let eq20_reactive_node_derivatives: [f64; 11] = [eq20_e316_q_d_n0, eq20_e316_q_d_n1, eq20_e316_q_d_n2, eq20_e316_q_d_n3, eq20_e316_q_d_n4, eq20_e316_q_d_n5, eq20_e316_q_d_n6, eq20_e316_q_d_n7, eq20_e316_q_d_n8, eq20_e316_q_d_n9, eq20_e316_q_d_n10];
        let eq20_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[9]),
            &nodes,
            &eq20_reactive_node_derivatives,
            &branches,
            &eq20_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_27_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq27_e355_q: f64 = (nv10 - 0.0);
        let eq27_e356: f64 = (s.v[306] * (nv10 - 0.0));
        let eq27_e356_d_n0: f64 = (s.dn[306][0] * (nv10 - 0.0));
        let eq27_e356_d_n1: f64 = (s.dn[306][1] * (nv10 - 0.0));
        let eq27_e356_d_n2: f64 = (s.dn[306][2] * (nv10 - 0.0));
        let eq27_e356_d_n3: f64 = (s.dn[306][3] * (nv10 - 0.0));
        let eq27_e356_d_n4: f64 = (s.dn[306][4] * (nv10 - 0.0));
        let eq27_e356_d_n5: f64 = (s.dn[306][5] * (nv10 - 0.0));
        let eq27_e356_d_n6: f64 = (s.dn[306][6] * (nv10 - 0.0));
        let eq27_e356_d_n7: f64 = (s.dn[306][7] * (nv10 - 0.0));
        let eq27_e356_d_n8: f64 = (s.dn[306][8] * (nv10 - 0.0));
        let eq27_e356_d_n9: f64 = (s.dn[306][9] * (nv10 - 0.0));
        let eq27_e356_d_n10: f64 = ((s.dn[306][10] * (nv10 - 0.0)) + s.v[306]);
        let eq27_e356_q: f64 = (s.v[306] * eq27_e355_q);
        let eq27_e356_q_d_n0: f64 = (s.dn[306][0] * eq27_e355_q);
        let eq27_e356_q_d_n1: f64 = (s.dn[306][1] * eq27_e355_q);
        let eq27_e356_q_d_n2: f64 = (s.dn[306][2] * eq27_e355_q);
        let eq27_e356_q_d_n3: f64 = (s.dn[306][3] * eq27_e355_q);
        let eq27_e356_q_d_n4: f64 = (s.dn[306][4] * eq27_e355_q);
        let eq27_e356_q_d_n5: f64 = (s.dn[306][5] * eq27_e355_q);
        let eq27_e356_q_d_n6: f64 = (s.dn[306][6] * eq27_e355_q);
        let eq27_e356_q_d_n7: f64 = (s.dn[306][7] * eq27_e355_q);
        let eq27_e356_q_d_n8: f64 = (s.dn[306][8] * eq27_e355_q);
        let eq27_e356_q_d_n9: f64 = (s.dn[306][9] * eq27_e355_q);
        let eq27_e356_q_d_n10: f64 = ((s.dn[306][10] * eq27_e355_q) + s.v[306]);
        let eq27_reactive_node_derivatives: [f64; 11] = [eq27_e356_q_d_n0, eq27_e356_q_d_n1, eq27_e356_q_d_n2, eq27_e356_q_d_n3, eq27_e356_q_d_n4, eq27_e356_q_d_n5, eq27_e356_q_d_n6, eq27_e356_q_d_n7, eq27_e356_q_d_n8, eq27_e356_q_d_n9, eq27_e356_q_d_n10];
        let eq27_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            &nodes,
            &eq27_reactive_node_derivatives,
            &branches,
            &eq27_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
