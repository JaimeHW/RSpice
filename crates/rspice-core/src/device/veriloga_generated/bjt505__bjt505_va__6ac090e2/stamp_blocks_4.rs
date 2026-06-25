#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq55_e526,) = {
    if ((s.v[611] != 0.0) && (s.v[612] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e526;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[10]),
            self.multiplicity * (eq55_value),
            &[
            ],
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
        let (eq56_e536,) = {
    if ((s.v[611] != 0.0) && (s.v[612] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e536;
        stamper.stamp_current(
            Some(nodes[10]),
            Some(nodes[7]),
            self.multiplicity * (eq56_value),
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
        let (eq57_e547,) = {
    if ((s.v[611] != 0.0) && (!(s.v[612] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq57_value: f64 = eq57_e547;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[9]),
            self.multiplicity * (eq57_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_58_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq58_e558,) = {
    if ((s.v[611] != 0.0) && (!(s.v[612] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e558;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq58_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_59_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq59_e569,) = {
    if ((!(s.v[611] != 0.0)) && (s.v[613] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e569;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[10]),
            self.multiplicity * (eq59_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_60_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq60_e580,) = {
    if ((!(s.v[611] != 0.0)) && (s.v[613] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e580;
        stamper.stamp_current(
            Some(nodes[10]),
            Some(nodes[7]),
            self.multiplicity * (eq60_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_61_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq61_e592,) = {
    if ((!(s.v[611] != 0.0)) && (!(s.v[613] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e592;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq61_value),
            &[
            ],
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
        let eq14_e266: f64 = (s.v[215] + s.v[220]);
        let eq14_e266_d_n0: f64 = (s.dn[215][0] + s.dn[220][0]);
        let eq14_e266_d_n1: f64 = (s.dn[215][1] + s.dn[220][1]);
        let eq14_e266_d_n2: f64 = (s.dn[215][2] + s.dn[220][2]);
        let eq14_e266_d_n3: f64 = (s.dn[215][3] + s.dn[220][3]);
        let eq14_e266_d_n4: f64 = (s.dn[215][4] + s.dn[220][4]);
        let eq14_e266_d_n5: f64 = (s.dn[215][5] + s.dn[220][5]);
        let eq14_e266_d_n6: f64 = (s.dn[215][6] + s.dn[220][6]);
        let eq14_e266_d_n7: f64 = (s.dn[215][7] + s.dn[220][7]);
        let eq14_e266_d_n8: f64 = (s.dn[215][8] + s.dn[220][8]);
        let eq14_e266_d_n9: f64 = (s.dn[215][9] + s.dn[220][9]);
        let eq14_e266_d_n10: f64 = (s.dn[215][10] + s.dn[220][10]);
        let eq14_e266_d_n11: f64 = (s.dn[215][11] + s.dn[220][11]);
        let eq14_e268: f64 = (eq14_e266 + s.v[235]);
        let eq14_e268_d_n0: f64 = (eq14_e266_d_n0 + s.dn[235][0]);
        let eq14_e268_d_n1: f64 = (eq14_e266_d_n1 + s.dn[235][1]);
        let eq14_e268_d_n2: f64 = (eq14_e266_d_n2 + s.dn[235][2]);
        let eq14_e268_d_n3: f64 = (eq14_e266_d_n3 + s.dn[235][3]);
        let eq14_e268_d_n4: f64 = (eq14_e266_d_n4 + s.dn[235][4]);
        let eq14_e268_d_n5: f64 = (eq14_e266_d_n5 + s.dn[235][5]);
        let eq14_e268_d_n6: f64 = (eq14_e266_d_n6 + s.dn[235][6]);
        let eq14_e268_d_n7: f64 = (eq14_e266_d_n7 + s.dn[235][7]);
        let eq14_e268_d_n8: f64 = (eq14_e266_d_n8 + s.dn[235][8]);
        let eq14_e268_d_n9: f64 = (eq14_e266_d_n9 + s.dn[235][9]);
        let eq14_e268_d_n10: f64 = (eq14_e266_d_n10 + s.dn[235][10]);
        let eq14_e268_d_n11: f64 = (eq14_e266_d_n11 + s.dn[235][11]);
        let eq14_e269: f64 = (p.p3 * eq14_e268);
        let eq14_e269_d_n0: f64 = (p.p3 * eq14_e268_d_n0);
        let eq14_e269_d_n1: f64 = (p.p3 * eq14_e268_d_n1);
        let eq14_e269_d_n2: f64 = (p.p3 * eq14_e268_d_n2);
        let eq14_e269_d_n3: f64 = (p.p3 * eq14_e268_d_n3);
        let eq14_e269_d_n4: f64 = (p.p3 * eq14_e268_d_n4);
        let eq14_e269_d_n5: f64 = (p.p3 * eq14_e268_d_n5);
        let eq14_e269_d_n6: f64 = (p.p3 * eq14_e268_d_n6);
        let eq14_e269_d_n7: f64 = (p.p3 * eq14_e268_d_n7);
        let eq14_e269_d_n8: f64 = (p.p3 * eq14_e268_d_n8);
        let eq14_e269_d_n9: f64 = (p.p3 * eq14_e268_d_n9);
        let eq14_e269_d_n10: f64 = (p.p3 * eq14_e268_d_n10);
        let eq14_e269_d_n11: f64 = (p.p3 * eq14_e268_d_n11);
        let eq14_e270_q: f64 = eq14_e269;
        let eq14_e272: f64 = (eq14_e269 * p.p1);
        let eq14_e272_d_n0: f64 = (eq14_e269_d_n0 * p.p1);
        let eq14_e272_d_n1: f64 = (eq14_e269_d_n1 * p.p1);
        let eq14_e272_d_n2: f64 = (eq14_e269_d_n2 * p.p1);
        let eq14_e272_d_n3: f64 = (eq14_e269_d_n3 * p.p1);
        let eq14_e272_d_n4: f64 = (eq14_e269_d_n4 * p.p1);
        let eq14_e272_d_n5: f64 = (eq14_e269_d_n5 * p.p1);
        let eq14_e272_d_n6: f64 = (eq14_e269_d_n6 * p.p1);
        let eq14_e272_d_n7: f64 = (eq14_e269_d_n7 * p.p1);
        let eq14_e272_d_n8: f64 = (eq14_e269_d_n8 * p.p1);
        let eq14_e272_d_n9: f64 = (eq14_e269_d_n9 * p.p1);
        let eq14_e272_d_n10: f64 = (eq14_e269_d_n10 * p.p1);
        let eq14_e272_d_n11: f64 = (eq14_e269_d_n11 * p.p1);
        let eq14_e272_q: f64 = (eq14_e270_q * p.p1);
        let eq14_e272_q_d_n0: f64 = (eq14_e269_d_n0 * p.p1);
        let eq14_e272_q_d_n1: f64 = (eq14_e269_d_n1 * p.p1);
        let eq14_e272_q_d_n2: f64 = (eq14_e269_d_n2 * p.p1);
        let eq14_e272_q_d_n3: f64 = (eq14_e269_d_n3 * p.p1);
        let eq14_e272_q_d_n4: f64 = (eq14_e269_d_n4 * p.p1);
        let eq14_e272_q_d_n5: f64 = (eq14_e269_d_n5 * p.p1);
        let eq14_e272_q_d_n6: f64 = (eq14_e269_d_n6 * p.p1);
        let eq14_e272_q_d_n7: f64 = (eq14_e269_d_n7 * p.p1);
        let eq14_e272_q_d_n8: f64 = (eq14_e269_d_n8 * p.p1);
        let eq14_e272_q_d_n9: f64 = (eq14_e269_d_n9 * p.p1);
        let eq14_e272_q_d_n10: f64 = (eq14_e269_d_n10 * p.p1);
        let eq14_e272_q_d_n11: f64 = (eq14_e269_d_n11 * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e272_q_d_n0, eq14_e272_q_d_n1, eq14_e272_q_d_n2, eq14_e272_q_d_n3, eq14_e272_q_d_n4, eq14_e272_q_d_n5, eq14_e272_q_d_n6, eq14_e272_q_d_n7, eq14_e272_q_d_n8, eq14_e272_q_d_n9, eq14_e272_q_d_n10, eq14_e272_q_d_n11];
        let eq14_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
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
        let eq15_e275: f64 = (p.p3 * s.v[217]);
        let eq15_e275_d_n0: f64 = (p.p3 * s.dn[217][0]);
        let eq15_e275_d_n1: f64 = (p.p3 * s.dn[217][1]);
        let eq15_e275_d_n2: f64 = (p.p3 * s.dn[217][2]);
        let eq15_e275_d_n3: f64 = (p.p3 * s.dn[217][3]);
        let eq15_e275_d_n4: f64 = (p.p3 * s.dn[217][4]);
        let eq15_e275_d_n5: f64 = (p.p3 * s.dn[217][5]);
        let eq15_e275_d_n6: f64 = (p.p3 * s.dn[217][6]);
        let eq15_e275_d_n7: f64 = (p.p3 * s.dn[217][7]);
        let eq15_e275_d_n8: f64 = (p.p3 * s.dn[217][8]);
        let eq15_e275_d_n9: f64 = (p.p3 * s.dn[217][9]);
        let eq15_e275_d_n10: f64 = (p.p3 * s.dn[217][10]);
        let eq15_e275_d_n11: f64 = (p.p3 * s.dn[217][11]);
        let eq15_e276_q: f64 = eq15_e275;
        let eq15_e278: f64 = (eq15_e275 * p.p1);
        let eq15_e278_d_n0: f64 = (eq15_e275_d_n0 * p.p1);
        let eq15_e278_d_n1: f64 = (eq15_e275_d_n1 * p.p1);
        let eq15_e278_d_n2: f64 = (eq15_e275_d_n2 * p.p1);
        let eq15_e278_d_n3: f64 = (eq15_e275_d_n3 * p.p1);
        let eq15_e278_d_n4: f64 = (eq15_e275_d_n4 * p.p1);
        let eq15_e278_d_n5: f64 = (eq15_e275_d_n5 * p.p1);
        let eq15_e278_d_n6: f64 = (eq15_e275_d_n6 * p.p1);
        let eq15_e278_d_n7: f64 = (eq15_e275_d_n7 * p.p1);
        let eq15_e278_d_n8: f64 = (eq15_e275_d_n8 * p.p1);
        let eq15_e278_d_n9: f64 = (eq15_e275_d_n9 * p.p1);
        let eq15_e278_d_n10: f64 = (eq15_e275_d_n10 * p.p1);
        let eq15_e278_d_n11: f64 = (eq15_e275_d_n11 * p.p1);
        let eq15_e278_q: f64 = (eq15_e276_q * p.p1);
        let eq15_e278_q_d_n0: f64 = (eq15_e275_d_n0 * p.p1);
        let eq15_e278_q_d_n1: f64 = (eq15_e275_d_n1 * p.p1);
        let eq15_e278_q_d_n2: f64 = (eq15_e275_d_n2 * p.p1);
        let eq15_e278_q_d_n3: f64 = (eq15_e275_d_n3 * p.p1);
        let eq15_e278_q_d_n4: f64 = (eq15_e275_d_n4 * p.p1);
        let eq15_e278_q_d_n5: f64 = (eq15_e275_d_n5 * p.p1);
        let eq15_e278_q_d_n6: f64 = (eq15_e275_d_n6 * p.p1);
        let eq15_e278_q_d_n7: f64 = (eq15_e275_d_n7 * p.p1);
        let eq15_e278_q_d_n8: f64 = (eq15_e275_d_n8 * p.p1);
        let eq15_e278_q_d_n9: f64 = (eq15_e275_d_n9 * p.p1);
        let eq15_e278_q_d_n10: f64 = (eq15_e275_d_n10 * p.p1);
        let eq15_e278_q_d_n11: f64 = (eq15_e275_d_n11 * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e278_q_d_n0, eq15_e278_q_d_n1, eq15_e278_q_d_n2, eq15_e278_q_d_n3, eq15_e278_q_d_n4, eq15_e278_q_d_n5, eq15_e278_q_d_n6, eq15_e278_q_d_n7, eq15_e278_q_d_n8, eq15_e278_q_d_n9, eq15_e278_q_d_n10, eq15_e278_q_d_n11];
        let eq15_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &nodes,
            &eq15_reactive_node_derivatives,
            &branches,
            &eq15_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_16_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq16_e282: f64 = (s.v[218] + s.v[221]);
        let eq16_e282_d_n0: f64 = (s.dn[218][0] + s.dn[221][0]);
        let eq16_e282_d_n1: f64 = (s.dn[218][1] + s.dn[221][1]);
        let eq16_e282_d_n2: f64 = (s.dn[218][2] + s.dn[221][2]);
        let eq16_e282_d_n3: f64 = (s.dn[218][3] + s.dn[221][3]);
        let eq16_e282_d_n4: f64 = (s.dn[218][4] + s.dn[221][4]);
        let eq16_e282_d_n5: f64 = (s.dn[218][5] + s.dn[221][5]);
        let eq16_e282_d_n6: f64 = (s.dn[218][6] + s.dn[221][6]);
        let eq16_e282_d_n7: f64 = (s.dn[218][7] + s.dn[221][7]);
        let eq16_e282_d_n8: f64 = (s.dn[218][8] + s.dn[221][8]);
        let eq16_e282_d_n9: f64 = (s.dn[218][9] + s.dn[221][9]);
        let eq16_e282_d_n10: f64 = (s.dn[218][10] + s.dn[221][10]);
        let eq16_e282_d_n11: f64 = (s.dn[218][11] + s.dn[221][11]);
        let eq16_e284: f64 = (eq16_e282 + s.v[238]);
        let eq16_e284_d_n0: f64 = (eq16_e282_d_n0 + s.dn[238][0]);
        let eq16_e284_d_n1: f64 = (eq16_e282_d_n1 + s.dn[238][1]);
        let eq16_e284_d_n2: f64 = (eq16_e282_d_n2 + s.dn[238][2]);
        let eq16_e284_d_n3: f64 = (eq16_e282_d_n3 + s.dn[238][3]);
        let eq16_e284_d_n4: f64 = (eq16_e282_d_n4 + s.dn[238][4]);
        let eq16_e284_d_n5: f64 = (eq16_e282_d_n5 + s.dn[238][5]);
        let eq16_e284_d_n6: f64 = (eq16_e282_d_n6 + s.dn[238][6]);
        let eq16_e284_d_n7: f64 = (eq16_e282_d_n7 + s.dn[238][7]);
        let eq16_e284_d_n8: f64 = (eq16_e282_d_n8 + s.dn[238][8]);
        let eq16_e284_d_n9: f64 = (eq16_e282_d_n9 + s.dn[238][9]);
        let eq16_e284_d_n10: f64 = (eq16_e282_d_n10 + s.dn[238][10]);
        let eq16_e284_d_n11: f64 = (eq16_e282_d_n11 + s.dn[238][11]);
        let eq16_e285: f64 = (p.p3 * eq16_e284);
        let eq16_e285_d_n0: f64 = (p.p3 * eq16_e284_d_n0);
        let eq16_e285_d_n1: f64 = (p.p3 * eq16_e284_d_n1);
        let eq16_e285_d_n2: f64 = (p.p3 * eq16_e284_d_n2);
        let eq16_e285_d_n3: f64 = (p.p3 * eq16_e284_d_n3);
        let eq16_e285_d_n4: f64 = (p.p3 * eq16_e284_d_n4);
        let eq16_e285_d_n5: f64 = (p.p3 * eq16_e284_d_n5);
        let eq16_e285_d_n6: f64 = (p.p3 * eq16_e284_d_n6);
        let eq16_e285_d_n7: f64 = (p.p3 * eq16_e284_d_n7);
        let eq16_e285_d_n8: f64 = (p.p3 * eq16_e284_d_n8);
        let eq16_e285_d_n9: f64 = (p.p3 * eq16_e284_d_n9);
        let eq16_e285_d_n10: f64 = (p.p3 * eq16_e284_d_n10);
        let eq16_e285_d_n11: f64 = (p.p3 * eq16_e284_d_n11);
        let eq16_e286_q: f64 = eq16_e285;
        let eq16_e288: f64 = (eq16_e285 * p.p1);
        let eq16_e288_d_n0: f64 = (eq16_e285_d_n0 * p.p1);
        let eq16_e288_d_n1: f64 = (eq16_e285_d_n1 * p.p1);
        let eq16_e288_d_n2: f64 = (eq16_e285_d_n2 * p.p1);
        let eq16_e288_d_n3: f64 = (eq16_e285_d_n3 * p.p1);
        let eq16_e288_d_n4: f64 = (eq16_e285_d_n4 * p.p1);
        let eq16_e288_d_n5: f64 = (eq16_e285_d_n5 * p.p1);
        let eq16_e288_d_n6: f64 = (eq16_e285_d_n6 * p.p1);
        let eq16_e288_d_n7: f64 = (eq16_e285_d_n7 * p.p1);
        let eq16_e288_d_n8: f64 = (eq16_e285_d_n8 * p.p1);
        let eq16_e288_d_n9: f64 = (eq16_e285_d_n9 * p.p1);
        let eq16_e288_d_n10: f64 = (eq16_e285_d_n10 * p.p1);
        let eq16_e288_d_n11: f64 = (eq16_e285_d_n11 * p.p1);
        let eq16_e288_q: f64 = (eq16_e286_q * p.p1);
        let eq16_e288_q_d_n0: f64 = (eq16_e285_d_n0 * p.p1);
        let eq16_e288_q_d_n1: f64 = (eq16_e285_d_n1 * p.p1);
        let eq16_e288_q_d_n2: f64 = (eq16_e285_d_n2 * p.p1);
        let eq16_e288_q_d_n3: f64 = (eq16_e285_d_n3 * p.p1);
        let eq16_e288_q_d_n4: f64 = (eq16_e285_d_n4 * p.p1);
        let eq16_e288_q_d_n5: f64 = (eq16_e285_d_n5 * p.p1);
        let eq16_e288_q_d_n6: f64 = (eq16_e285_d_n6 * p.p1);
        let eq16_e288_q_d_n7: f64 = (eq16_e285_d_n7 * p.p1);
        let eq16_e288_q_d_n8: f64 = (eq16_e285_d_n8 * p.p1);
        let eq16_e288_q_d_n9: f64 = (eq16_e285_d_n9 * p.p1);
        let eq16_e288_q_d_n10: f64 = (eq16_e285_d_n10 * p.p1);
        let eq16_e288_q_d_n11: f64 = (eq16_e285_d_n11 * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e288_q_d_n0, eq16_e288_q_d_n1, eq16_e288_q_d_n2, eq16_e288_q_d_n3, eq16_e288_q_d_n4, eq16_e288_q_d_n5, eq16_e288_q_d_n6, eq16_e288_q_d_n7, eq16_e288_q_d_n8, eq16_e288_q_d_n9, eq16_e288_q_d_n10, eq16_e288_q_d_n11];
        let eq16_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &nodes,
            &eq16_reactive_node_derivatives,
            &branches,
            &eq16_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq17_e291: f64 = (p.p3 * s.v[233]);
        let eq17_e291_d_n0: f64 = (p.p3 * s.dn[233][0]);
        let eq17_e291_d_n1: f64 = (p.p3 * s.dn[233][1]);
        let eq17_e291_d_n2: f64 = (p.p3 * s.dn[233][2]);
        let eq17_e291_d_n3: f64 = (p.p3 * s.dn[233][3]);
        let eq17_e291_d_n4: f64 = (p.p3 * s.dn[233][4]);
        let eq17_e291_d_n5: f64 = (p.p3 * s.dn[233][5]);
        let eq17_e291_d_n6: f64 = (p.p3 * s.dn[233][6]);
        let eq17_e291_d_n7: f64 = (p.p3 * s.dn[233][7]);
        let eq17_e291_d_n8: f64 = (p.p3 * s.dn[233][8]);
        let eq17_e291_d_n9: f64 = (p.p3 * s.dn[233][9]);
        let eq17_e291_d_n10: f64 = (p.p3 * s.dn[233][10]);
        let eq17_e291_d_n11: f64 = (p.p3 * s.dn[233][11]);
        let eq17_e292_q: f64 = eq17_e291;
        let eq17_e294: f64 = (eq17_e291 * p.p1);
        let eq17_e294_d_n0: f64 = (eq17_e291_d_n0 * p.p1);
        let eq17_e294_d_n1: f64 = (eq17_e291_d_n1 * p.p1);
        let eq17_e294_d_n2: f64 = (eq17_e291_d_n2 * p.p1);
        let eq17_e294_d_n3: f64 = (eq17_e291_d_n3 * p.p1);
        let eq17_e294_d_n4: f64 = (eq17_e291_d_n4 * p.p1);
        let eq17_e294_d_n5: f64 = (eq17_e291_d_n5 * p.p1);
        let eq17_e294_d_n6: f64 = (eq17_e291_d_n6 * p.p1);
        let eq17_e294_d_n7: f64 = (eq17_e291_d_n7 * p.p1);
        let eq17_e294_d_n8: f64 = (eq17_e291_d_n8 * p.p1);
        let eq17_e294_d_n9: f64 = (eq17_e291_d_n9 * p.p1);
        let eq17_e294_d_n10: f64 = (eq17_e291_d_n10 * p.p1);
        let eq17_e294_d_n11: f64 = (eq17_e291_d_n11 * p.p1);
        let eq17_e294_q: f64 = (eq17_e292_q * p.p1);
        let eq17_e294_q_d_n0: f64 = (eq17_e291_d_n0 * p.p1);
        let eq17_e294_q_d_n1: f64 = (eq17_e291_d_n1 * p.p1);
        let eq17_e294_q_d_n2: f64 = (eq17_e291_d_n2 * p.p1);
        let eq17_e294_q_d_n3: f64 = (eq17_e291_d_n3 * p.p1);
        let eq17_e294_q_d_n4: f64 = (eq17_e291_d_n4 * p.p1);
        let eq17_e294_q_d_n5: f64 = (eq17_e291_d_n5 * p.p1);
        let eq17_e294_q_d_n6: f64 = (eq17_e291_d_n6 * p.p1);
        let eq17_e294_q_d_n7: f64 = (eq17_e291_d_n7 * p.p1);
        let eq17_e294_q_d_n8: f64 = (eq17_e291_d_n8 * p.p1);
        let eq17_e294_q_d_n9: f64 = (eq17_e291_d_n9 * p.p1);
        let eq17_e294_q_d_n10: f64 = (eq17_e291_d_n10 * p.p1);
        let eq17_e294_q_d_n11: f64 = (eq17_e291_d_n11 * p.p1);
        let eq17_reactive_node_derivatives: [f64; 12] = [eq17_e294_q_d_n0, eq17_e294_q_d_n1, eq17_e294_q_d_n2, eq17_e294_q_d_n3, eq17_e294_q_d_n4, eq17_e294_q_d_n5, eq17_e294_q_d_n6, eq17_e294_q_d_n7, eq17_e294_q_d_n8, eq17_e294_q_d_n9, eq17_e294_q_d_n10, eq17_e294_q_d_n11];
        let eq17_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            &nodes,
            &eq17_reactive_node_derivatives,
            &branches,
            &eq17_reactive_branch_derivatives,
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
        let eq18_e297: f64 = (p.p3 * s.v[222]);
        let eq18_e297_d_n0: f64 = (p.p3 * s.dn[222][0]);
        let eq18_e297_d_n1: f64 = (p.p3 * s.dn[222][1]);
        let eq18_e297_d_n2: f64 = (p.p3 * s.dn[222][2]);
        let eq18_e297_d_n3: f64 = (p.p3 * s.dn[222][3]);
        let eq18_e297_d_n4: f64 = (p.p3 * s.dn[222][4]);
        let eq18_e297_d_n5: f64 = (p.p3 * s.dn[222][5]);
        let eq18_e297_d_n6: f64 = (p.p3 * s.dn[222][6]);
        let eq18_e297_d_n7: f64 = (p.p3 * s.dn[222][7]);
        let eq18_e297_d_n8: f64 = (p.p3 * s.dn[222][8]);
        let eq18_e297_d_n9: f64 = (p.p3 * s.dn[222][9]);
        let eq18_e297_d_n10: f64 = (p.p3 * s.dn[222][10]);
        let eq18_e297_d_n11: f64 = (p.p3 * s.dn[222][11]);
        let eq18_e298_q: f64 = eq18_e297;
        let eq18_e300: f64 = (eq18_e297 * p.p1);
        let eq18_e300_d_n0: f64 = (eq18_e297_d_n0 * p.p1);
        let eq18_e300_d_n1: f64 = (eq18_e297_d_n1 * p.p1);
        let eq18_e300_d_n2: f64 = (eq18_e297_d_n2 * p.p1);
        let eq18_e300_d_n3: f64 = (eq18_e297_d_n3 * p.p1);
        let eq18_e300_d_n4: f64 = (eq18_e297_d_n4 * p.p1);
        let eq18_e300_d_n5: f64 = (eq18_e297_d_n5 * p.p1);
        let eq18_e300_d_n6: f64 = (eq18_e297_d_n6 * p.p1);
        let eq18_e300_d_n7: f64 = (eq18_e297_d_n7 * p.p1);
        let eq18_e300_d_n8: f64 = (eq18_e297_d_n8 * p.p1);
        let eq18_e300_d_n9: f64 = (eq18_e297_d_n9 * p.p1);
        let eq18_e300_d_n10: f64 = (eq18_e297_d_n10 * p.p1);
        let eq18_e300_d_n11: f64 = (eq18_e297_d_n11 * p.p1);
        let eq18_e300_q: f64 = (eq18_e298_q * p.p1);
        let eq18_e300_q_d_n0: f64 = (eq18_e297_d_n0 * p.p1);
        let eq18_e300_q_d_n1: f64 = (eq18_e297_d_n1 * p.p1);
        let eq18_e300_q_d_n2: f64 = (eq18_e297_d_n2 * p.p1);
        let eq18_e300_q_d_n3: f64 = (eq18_e297_d_n3 * p.p1);
        let eq18_e300_q_d_n4: f64 = (eq18_e297_d_n4 * p.p1);
        let eq18_e300_q_d_n5: f64 = (eq18_e297_d_n5 * p.p1);
        let eq18_e300_q_d_n6: f64 = (eq18_e297_d_n6 * p.p1);
        let eq18_e300_q_d_n7: f64 = (eq18_e297_d_n7 * p.p1);
        let eq18_e300_q_d_n8: f64 = (eq18_e297_d_n8 * p.p1);
        let eq18_e300_q_d_n9: f64 = (eq18_e297_d_n9 * p.p1);
        let eq18_e300_q_d_n10: f64 = (eq18_e297_d_n10 * p.p1);
        let eq18_e300_q_d_n11: f64 = (eq18_e297_d_n11 * p.p1);
        let eq18_reactive_node_derivatives: [f64; 12] = [eq18_e300_q_d_n0, eq18_e300_q_d_n1, eq18_e300_q_d_n2, eq18_e300_q_d_n3, eq18_e300_q_d_n4, eq18_e300_q_d_n5, eq18_e300_q_d_n6, eq18_e300_q_d_n7, eq18_e300_q_d_n8, eq18_e300_q_d_n9, eq18_e300_q_d_n10, eq18_e300_q_d_n11];
        let eq18_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &nodes,
            &eq18_reactive_node_derivatives,
            &branches,
            &eq18_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq19_e303: f64 = (p.p3 * p.p69);
        let eq19_e305: f64 = (eq19_e303 * s.v[263]);
        let eq19_e305_d_n0: f64 = (eq19_e303 * s.dn[263][0]);
        let eq19_e305_d_n1: f64 = (eq19_e303 * s.dn[263][1]);
        let eq19_e305_d_n2: f64 = (eq19_e303 * s.dn[263][2]);
        let eq19_e305_d_n3: f64 = (eq19_e303 * s.dn[263][3]);
        let eq19_e305_d_n4: f64 = (eq19_e303 * s.dn[263][4]);
        let eq19_e305_d_n5: f64 = (eq19_e303 * s.dn[263][5]);
        let eq19_e305_d_n6: f64 = (eq19_e303 * s.dn[263][6]);
        let eq19_e305_d_n7: f64 = (eq19_e303 * s.dn[263][7]);
        let eq19_e305_d_n8: f64 = (eq19_e303 * s.dn[263][8]);
        let eq19_e305_d_n9: f64 = (eq19_e303 * s.dn[263][9]);
        let eq19_e305_d_n10: f64 = (eq19_e303 * s.dn[263][10]);
        let eq19_e305_d_n11: f64 = (eq19_e303 * s.dn[263][11]);
        let eq19_e306_q: f64 = eq19_e305;
        let eq19_e308: f64 = (eq19_e305 * p.p1);
        let eq19_e308_d_n0: f64 = (eq19_e305_d_n0 * p.p1);
        let eq19_e308_d_n1: f64 = (eq19_e305_d_n1 * p.p1);
        let eq19_e308_d_n2: f64 = (eq19_e305_d_n2 * p.p1);
        let eq19_e308_d_n3: f64 = (eq19_e305_d_n3 * p.p1);
        let eq19_e308_d_n4: f64 = (eq19_e305_d_n4 * p.p1);
        let eq19_e308_d_n5: f64 = (eq19_e305_d_n5 * p.p1);
        let eq19_e308_d_n6: f64 = (eq19_e305_d_n6 * p.p1);
        let eq19_e308_d_n7: f64 = (eq19_e305_d_n7 * p.p1);
        let eq19_e308_d_n8: f64 = (eq19_e305_d_n8 * p.p1);
        let eq19_e308_d_n9: f64 = (eq19_e305_d_n9 * p.p1);
        let eq19_e308_d_n10: f64 = (eq19_e305_d_n10 * p.p1);
        let eq19_e308_d_n11: f64 = (eq19_e305_d_n11 * p.p1);
        let eq19_e308_q: f64 = (eq19_e306_q * p.p1);
        let eq19_e308_q_d_n0: f64 = (eq19_e305_d_n0 * p.p1);
        let eq19_e308_q_d_n1: f64 = (eq19_e305_d_n1 * p.p1);
        let eq19_e308_q_d_n2: f64 = (eq19_e305_d_n2 * p.p1);
        let eq19_e308_q_d_n3: f64 = (eq19_e305_d_n3 * p.p1);
        let eq19_e308_q_d_n4: f64 = (eq19_e305_d_n4 * p.p1);
        let eq19_e308_q_d_n5: f64 = (eq19_e305_d_n5 * p.p1);
        let eq19_e308_q_d_n6: f64 = (eq19_e305_d_n6 * p.p1);
        let eq19_e308_q_d_n7: f64 = (eq19_e305_d_n7 * p.p1);
        let eq19_e308_q_d_n8: f64 = (eq19_e305_d_n8 * p.p1);
        let eq19_e308_q_d_n9: f64 = (eq19_e305_d_n9 * p.p1);
        let eq19_e308_q_d_n10: f64 = (eq19_e305_d_n10 * p.p1);
        let eq19_e308_q_d_n11: f64 = (eq19_e305_d_n11 * p.p1);
        let eq19_reactive_node_derivatives: [f64; 12] = [eq19_e308_q_d_n0, eq19_e308_q_d_n1, eq19_e308_q_d_n2, eq19_e308_q_d_n3, eq19_e308_q_d_n4, eq19_e308_q_d_n5, eq19_e308_q_d_n6, eq19_e308_q_d_n7, eq19_e308_q_d_n8, eq19_e308_q_d_n9, eq19_e308_q_d_n10, eq19_e308_q_d_n11];
        let eq19_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            &nodes,
            &eq19_reactive_node_derivatives,
            &branches,
            &eq19_reactive_branch_derivatives,
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
        let eq20_e311: f64 = (p.p3 * p.p78);
        let eq20_e313: f64 = (eq20_e311 * s.v[264]);
        let eq20_e313_d_n0: f64 = (eq20_e311 * s.dn[264][0]);
        let eq20_e313_d_n1: f64 = (eq20_e311 * s.dn[264][1]);
        let eq20_e313_d_n2: f64 = (eq20_e311 * s.dn[264][2]);
        let eq20_e313_d_n3: f64 = (eq20_e311 * s.dn[264][3]);
        let eq20_e313_d_n4: f64 = (eq20_e311 * s.dn[264][4]);
        let eq20_e313_d_n5: f64 = (eq20_e311 * s.dn[264][5]);
        let eq20_e313_d_n6: f64 = (eq20_e311 * s.dn[264][6]);
        let eq20_e313_d_n7: f64 = (eq20_e311 * s.dn[264][7]);
        let eq20_e313_d_n8: f64 = (eq20_e311 * s.dn[264][8]);
        let eq20_e313_d_n9: f64 = (eq20_e311 * s.dn[264][9]);
        let eq20_e313_d_n10: f64 = (eq20_e311 * s.dn[264][10]);
        let eq20_e313_d_n11: f64 = (eq20_e311 * s.dn[264][11]);
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
        let eq20_e316_d_n11: f64 = (eq20_e313_d_n11 * p.p1);
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
        let eq20_e316_q_d_n11: f64 = (eq20_e313_d_n11 * p.p1);
        let eq20_reactive_node_derivatives: [f64; 12] = [eq20_e316_q_d_n0, eq20_e316_q_d_n1, eq20_e316_q_d_n2, eq20_e316_q_d_n3, eq20_e316_q_d_n4, eq20_e316_q_d_n5, eq20_e316_q_d_n6, eq20_e316_q_d_n7, eq20_e316_q_d_n8, eq20_e316_q_d_n9, eq20_e316_q_d_n10, eq20_e316_q_d_n11];
        let eq20_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            &nodes,
            &eq20_reactive_node_derivatives,
            &branches,
            &eq20_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq23_e332: f64 = (s.v[230] + s.v[242]);
        let eq23_e332_d_n0: f64 = (s.dn[230][0] + s.dn[242][0]);
        let eq23_e332_d_n1: f64 = (s.dn[230][1] + s.dn[242][1]);
        let eq23_e332_d_n2: f64 = (s.dn[230][2] + s.dn[242][2]);
        let eq23_e332_d_n3: f64 = (s.dn[230][3] + s.dn[242][3]);
        let eq23_e332_d_n4: f64 = (s.dn[230][4] + s.dn[242][4]);
        let eq23_e332_d_n5: f64 = (s.dn[230][5] + s.dn[242][5]);
        let eq23_e332_d_n6: f64 = (s.dn[230][6] + s.dn[242][6]);
        let eq23_e332_d_n7: f64 = (s.dn[230][7] + s.dn[242][7]);
        let eq23_e332_d_n8: f64 = (s.dn[230][8] + s.dn[242][8]);
        let eq23_e332_d_n9: f64 = (s.dn[230][9] + s.dn[242][9]);
        let eq23_e332_d_n10: f64 = (s.dn[230][10] + s.dn[242][10]);
        let eq23_e332_d_n11: f64 = (s.dn[230][11] + s.dn[242][11]);
        let eq23_e333: f64 = (p.p3 * eq23_e332);
        let eq23_e333_d_n0: f64 = (p.p3 * eq23_e332_d_n0);
        let eq23_e333_d_n1: f64 = (p.p3 * eq23_e332_d_n1);
        let eq23_e333_d_n2: f64 = (p.p3 * eq23_e332_d_n2);
        let eq23_e333_d_n3: f64 = (p.p3 * eq23_e332_d_n3);
        let eq23_e333_d_n4: f64 = (p.p3 * eq23_e332_d_n4);
        let eq23_e333_d_n5: f64 = (p.p3 * eq23_e332_d_n5);
        let eq23_e333_d_n6: f64 = (p.p3 * eq23_e332_d_n6);
        let eq23_e333_d_n7: f64 = (p.p3 * eq23_e332_d_n7);
        let eq23_e333_d_n8: f64 = (p.p3 * eq23_e332_d_n8);
        let eq23_e333_d_n9: f64 = (p.p3 * eq23_e332_d_n9);
        let eq23_e333_d_n10: f64 = (p.p3 * eq23_e332_d_n10);
        let eq23_e333_d_n11: f64 = (p.p3 * eq23_e332_d_n11);
        let eq23_e334_q: f64 = eq23_e333;
        let eq23_e336: f64 = (eq23_e333 * p.p1);
        let eq23_e336_d_n0: f64 = (eq23_e333_d_n0 * p.p1);
        let eq23_e336_d_n1: f64 = (eq23_e333_d_n1 * p.p1);
        let eq23_e336_d_n2: f64 = (eq23_e333_d_n2 * p.p1);
        let eq23_e336_d_n3: f64 = (eq23_e333_d_n3 * p.p1);
        let eq23_e336_d_n4: f64 = (eq23_e333_d_n4 * p.p1);
        let eq23_e336_d_n5: f64 = (eq23_e333_d_n5 * p.p1);
        let eq23_e336_d_n6: f64 = (eq23_e333_d_n6 * p.p1);
        let eq23_e336_d_n7: f64 = (eq23_e333_d_n7 * p.p1);
        let eq23_e336_d_n8: f64 = (eq23_e333_d_n8 * p.p1);
        let eq23_e336_d_n9: f64 = (eq23_e333_d_n9 * p.p1);
        let eq23_e336_d_n10: f64 = (eq23_e333_d_n10 * p.p1);
        let eq23_e336_d_n11: f64 = (eq23_e333_d_n11 * p.p1);
        let eq23_e336_q: f64 = (eq23_e334_q * p.p1);
        let eq23_e336_q_d_n0: f64 = (eq23_e333_d_n0 * p.p1);
        let eq23_e336_q_d_n1: f64 = (eq23_e333_d_n1 * p.p1);
        let eq23_e336_q_d_n2: f64 = (eq23_e333_d_n2 * p.p1);
        let eq23_e336_q_d_n3: f64 = (eq23_e333_d_n3 * p.p1);
        let eq23_e336_q_d_n4: f64 = (eq23_e333_d_n4 * p.p1);
        let eq23_e336_q_d_n5: f64 = (eq23_e333_d_n5 * p.p1);
        let eq23_e336_q_d_n6: f64 = (eq23_e333_d_n6 * p.p1);
        let eq23_e336_q_d_n7: f64 = (eq23_e333_d_n7 * p.p1);
        let eq23_e336_q_d_n8: f64 = (eq23_e333_d_n8 * p.p1);
        let eq23_e336_q_d_n9: f64 = (eq23_e333_d_n9 * p.p1);
        let eq23_e336_q_d_n10: f64 = (eq23_e333_d_n10 * p.p1);
        let eq23_e336_q_d_n11: f64 = (eq23_e333_d_n11 * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e336_q_d_n0, eq23_e336_q_d_n1, eq23_e336_q_d_n2, eq23_e336_q_d_n3, eq23_e336_q_d_n4, eq23_e336_q_d_n5, eq23_e336_q_d_n6, eq23_e336_q_d_n7, eq23_e336_q_d_n8, eq23_e336_q_d_n9, eq23_e336_q_d_n10, eq23_e336_q_d_n11];
        let eq23_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            &nodes,
            &eq23_reactive_node_derivatives,
            &branches,
            &eq23_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_25_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq25_e351: f64 = (s.v[227] + s.v[243]);
        let eq25_e351_d_n0: f64 = (s.dn[227][0] + s.dn[243][0]);
        let eq25_e351_d_n1: f64 = (s.dn[227][1] + s.dn[243][1]);
        let eq25_e351_d_n2: f64 = (s.dn[227][2] + s.dn[243][2]);
        let eq25_e351_d_n3: f64 = (s.dn[227][3] + s.dn[243][3]);
        let eq25_e351_d_n4: f64 = (s.dn[227][4] + s.dn[243][4]);
        let eq25_e351_d_n5: f64 = (s.dn[227][5] + s.dn[243][5]);
        let eq25_e351_d_n6: f64 = (s.dn[227][6] + s.dn[243][6]);
        let eq25_e351_d_n7: f64 = (s.dn[227][7] + s.dn[243][7]);
        let eq25_e351_d_n8: f64 = (s.dn[227][8] + s.dn[243][8]);
        let eq25_e351_d_n9: f64 = (s.dn[227][9] + s.dn[243][9]);
        let eq25_e351_d_n10: f64 = (s.dn[227][10] + s.dn[243][10]);
        let eq25_e351_d_n11: f64 = (s.dn[227][11] + s.dn[243][11]);
        let eq25_e352: f64 = (p.p3 * eq25_e351);
        let eq25_e352_d_n0: f64 = (p.p3 * eq25_e351_d_n0);
        let eq25_e352_d_n1: f64 = (p.p3 * eq25_e351_d_n1);
        let eq25_e352_d_n2: f64 = (p.p3 * eq25_e351_d_n2);
        let eq25_e352_d_n3: f64 = (p.p3 * eq25_e351_d_n3);
        let eq25_e352_d_n4: f64 = (p.p3 * eq25_e351_d_n4);
        let eq25_e352_d_n5: f64 = (p.p3 * eq25_e351_d_n5);
        let eq25_e352_d_n6: f64 = (p.p3 * eq25_e351_d_n6);
        let eq25_e352_d_n7: f64 = (p.p3 * eq25_e351_d_n7);
        let eq25_e352_d_n8: f64 = (p.p3 * eq25_e351_d_n8);
        let eq25_e352_d_n9: f64 = (p.p3 * eq25_e351_d_n9);
        let eq25_e352_d_n10: f64 = (p.p3 * eq25_e351_d_n10);
        let eq25_e352_d_n11: f64 = (p.p3 * eq25_e351_d_n11);
        let eq25_e353_q: f64 = eq25_e352;
        let eq25_e355: f64 = (eq25_e352 * p.p1);
        let eq25_e355_d_n0: f64 = (eq25_e352_d_n0 * p.p1);
        let eq25_e355_d_n1: f64 = (eq25_e352_d_n1 * p.p1);
        let eq25_e355_d_n2: f64 = (eq25_e352_d_n2 * p.p1);
        let eq25_e355_d_n3: f64 = (eq25_e352_d_n3 * p.p1);
        let eq25_e355_d_n4: f64 = (eq25_e352_d_n4 * p.p1);
        let eq25_e355_d_n5: f64 = (eq25_e352_d_n5 * p.p1);
        let eq25_e355_d_n6: f64 = (eq25_e352_d_n6 * p.p1);
        let eq25_e355_d_n7: f64 = (eq25_e352_d_n7 * p.p1);
        let eq25_e355_d_n8: f64 = (eq25_e352_d_n8 * p.p1);
        let eq25_e355_d_n9: f64 = (eq25_e352_d_n9 * p.p1);
        let eq25_e355_d_n10: f64 = (eq25_e352_d_n10 * p.p1);
        let eq25_e355_d_n11: f64 = (eq25_e352_d_n11 * p.p1);
        let eq25_e355_q: f64 = (eq25_e353_q * p.p1);
        let eq25_e355_q_d_n0: f64 = (eq25_e352_d_n0 * p.p1);
        let eq25_e355_q_d_n1: f64 = (eq25_e352_d_n1 * p.p1);
        let eq25_e355_q_d_n2: f64 = (eq25_e352_d_n2 * p.p1);
        let eq25_e355_q_d_n3: f64 = (eq25_e352_d_n3 * p.p1);
        let eq25_e355_q_d_n4: f64 = (eq25_e352_d_n4 * p.p1);
        let eq25_e355_q_d_n5: f64 = (eq25_e352_d_n5 * p.p1);
        let eq25_e355_q_d_n6: f64 = (eq25_e352_d_n6 * p.p1);
        let eq25_e355_q_d_n7: f64 = (eq25_e352_d_n7 * p.p1);
        let eq25_e355_q_d_n8: f64 = (eq25_e352_d_n8 * p.p1);
        let eq25_e355_q_d_n9: f64 = (eq25_e352_d_n9 * p.p1);
        let eq25_e355_q_d_n10: f64 = (eq25_e352_d_n10 * p.p1);
        let eq25_e355_q_d_n11: f64 = (eq25_e352_d_n11 * p.p1);
        let eq25_reactive_node_derivatives: [f64; 12] = [eq25_e355_q_d_n0, eq25_e355_q_d_n1, eq25_e355_q_d_n2, eq25_e355_q_d_n3, eq25_e355_q_d_n4, eq25_e355_q_d_n5, eq25_e355_q_d_n6, eq25_e355_q_d_n7, eq25_e355_q_d_n8, eq25_e355_q_d_n9, eq25_e355_q_d_n10, eq25_e355_q_d_n11];
        let eq25_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            &nodes,
            &eq25_reactive_node_derivatives,
            &branches,
            &eq25_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
