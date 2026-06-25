#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq28_e334,) = {
    if (!(s.v[126] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e334;
        stamper.stamp_potential(
            branches[6],
            eq28_value,
            &[
            ],
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq29_e349, eq29_e349_d_n0, eq29_e349_d_n1, eq29_e349_d_n2, eq29_e349_d_n3, eq29_e349_d_n4, eq29_e349_d_n5, eq29_e349_d_n6, eq29_e349_d_n7, eq29_e349_d_n8, eq29_e349_d_n9, eq29_e349_d_b0, eq29_e349_d_b1, eq29_e349_d_b2, eq29_e349_d_b3, eq29_e349_d_b4, eq29_e349_d_b5, eq29_e349_d_b6, eq29_e349_d_b7,) = {
    if (s.v[127] != 0.0) {
        let eq29_e339: f64 = (s.v[52] / s.v[3]);
        let eq29_e339_d_n0: f64 = (s.dn[52][0] / s.v[3]);
        let eq29_e339_d_n1: f64 = (s.dn[52][1] / s.v[3]);
        let eq29_e339_d_n2: f64 = (s.dn[52][2] / s.v[3]);
        let eq29_e339_d_n3: f64 = (s.dn[52][3] / s.v[3]);
        let eq29_e339_d_n4: f64 = (s.dn[52][4] / s.v[3]);
        let eq29_e339_d_n5: f64 = (s.dn[52][5] / s.v[3]);
        let eq29_e339_d_n6: f64 = (s.dn[52][6] / s.v[3]);
        let eq29_e339_d_n7: f64 = (s.dn[52][7] / s.v[3]);
        let eq29_e339_d_n8: f64 = (s.dn[52][8] / s.v[3]);
        let eq29_e339_d_n9: f64 = (s.dn[52][9] / s.v[3]);
        let eq29_e339_d_b0: f64 = (s.db[52][0] / s.v[3]);
        let eq29_e339_d_b1: f64 = (s.db[52][1] / s.v[3]);
        let eq29_e339_d_b2: f64 = (s.db[52][2] / s.v[3]);
        let eq29_e339_d_b3: f64 = (s.db[52][3] / s.v[3]);
        let eq29_e339_d_b4: f64 = (s.db[52][4] / s.v[3]);
        let eq29_e339_d_b5: f64 = (s.db[52][5] / s.v[3]);
        let eq29_e339_d_b6: f64 = (s.db[52][6] / s.v[3]);
        let eq29_e339_d_b7: f64 = (s.db[52][7] / s.v[3]);
        let (eq29_e346, eq29_e346_d_n0, eq29_e346_d_n1, eq29_e346_d_n2, eq29_e346_d_n3, eq29_e346_d_n4, eq29_e346_d_n5, eq29_e346_d_n6, eq29_e346_d_n7, eq29_e346_d_n8, eq29_e346_d_n9, eq29_e346_d_b0, eq29_e346_d_b1, eq29_e346_d_b2, eq29_e346_d_b3, eq29_e346_d_b4, eq29_e346_d_b5, eq29_e346_d_b6, eq29_e346_d_b7,) = {
            if (eq29_e339 > p.p46) {
                let eq29_e344: f64 = (s.v[52] / s.v[3]);
                let eq29_e344_d_n0: f64 = (s.dn[52][0] / s.v[3]);
                let eq29_e344_d_n1: f64 = (s.dn[52][1] / s.v[3]);
                let eq29_e344_d_n2: f64 = (s.dn[52][2] / s.v[3]);
                let eq29_e344_d_n3: f64 = (s.dn[52][3] / s.v[3]);
                let eq29_e344_d_n4: f64 = (s.dn[52][4] / s.v[3]);
                let eq29_e344_d_n5: f64 = (s.dn[52][5] / s.v[3]);
                let eq29_e344_d_n6: f64 = (s.dn[52][6] / s.v[3]);
                let eq29_e344_d_n7: f64 = (s.dn[52][7] / s.v[3]);
                let eq29_e344_d_n8: f64 = (s.dn[52][8] / s.v[3]);
                let eq29_e344_d_n9: f64 = (s.dn[52][9] / s.v[3]);
                let eq29_e344_d_b0: f64 = (s.db[52][0] / s.v[3]);
                let eq29_e344_d_b1: f64 = (s.db[52][1] / s.v[3]);
                let eq29_e344_d_b2: f64 = (s.db[52][2] / s.v[3]);
                let eq29_e344_d_b3: f64 = (s.db[52][3] / s.v[3]);
                let eq29_e344_d_b4: f64 = (s.db[52][4] / s.v[3]);
                let eq29_e344_d_b5: f64 = (s.db[52][5] / s.v[3]);
                let eq29_e344_d_b6: f64 = (s.db[52][6] / s.v[3]);
                let eq29_e344_d_b7: f64 = (s.db[52][7] / s.v[3]);
                (eq29_e344, eq29_e344_d_n0, eq29_e344_d_n1, eq29_e344_d_n2, eq29_e344_d_n3, eq29_e344_d_n4, eq29_e344_d_n5, eq29_e344_d_n6, eq29_e344_d_n7, eq29_e344_d_n8, eq29_e344_d_n9, eq29_e344_d_b0, eq29_e344_d_b1, eq29_e344_d_b2, eq29_e344_d_b3, eq29_e344_d_b4, eq29_e344_d_b5, eq29_e344_d_b6, eq29_e344_d_b7,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq29_e347: f64 = ((nv0 - nv4) / eq29_e346);
        let eq29_e347_d_n0: f64 = ((eq29_e346 - ((nv0 - nv4) * eq29_e346_d_n0)) / (eq29_e346 * eq29_e346));
        let eq29_e347_d_n1: f64 = (-(((nv0 - nv4) * eq29_e346_d_n1) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n2: f64 = (-(((nv0 - nv4) * eq29_e346_d_n2) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n3: f64 = (-(((nv0 - nv4) * eq29_e346_d_n3) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n4: f64 = (((-eq29_e346) - ((nv0 - nv4) * eq29_e346_d_n4)) / (eq29_e346 * eq29_e346));
        let eq29_e347_d_n5: f64 = (-(((nv0 - nv4) * eq29_e346_d_n5) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n6: f64 = (-(((nv0 - nv4) * eq29_e346_d_n6) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n7: f64 = (-(((nv0 - nv4) * eq29_e346_d_n7) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n8: f64 = (-(((nv0 - nv4) * eq29_e346_d_n8) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n9: f64 = (-(((nv0 - nv4) * eq29_e346_d_n9) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b0: f64 = (-(((nv0 - nv4) * eq29_e346_d_b0) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b1: f64 = (-(((nv0 - nv4) * eq29_e346_d_b1) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b2: f64 = (-(((nv0 - nv4) * eq29_e346_d_b2) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b3: f64 = (-(((nv0 - nv4) * eq29_e346_d_b3) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b4: f64 = (-(((nv0 - nv4) * eq29_e346_d_b4) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b5: f64 = (-(((nv0 - nv4) * eq29_e346_d_b5) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b6: f64 = (-(((nv0 - nv4) * eq29_e346_d_b6) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b7: f64 = (-(((nv0 - nv4) * eq29_e346_d_b7) / (eq29_e346 * eq29_e346)));
        (eq29_e347, eq29_e347_d_n0, eq29_e347_d_n1, eq29_e347_d_n2, eq29_e347_d_n3, eq29_e347_d_n4, eq29_e347_d_n5, eq29_e347_d_n6, eq29_e347_d_n7, eq29_e347_d_n8, eq29_e347_d_n9, eq29_e347_d_b0, eq29_e347_d_b1, eq29_e347_d_b2, eq29_e347_d_b3, eq29_e347_d_b4, eq29_e347_d_b5, eq29_e347_d_b6, eq29_e347_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e349;
        let eq29_node_derivatives: [f64; 10] = [eq29_e349_d_n0, eq29_e349_d_n1, eq29_e349_d_n2, eq29_e349_d_n3, eq29_e349_d_n4, eq29_e349_d_n5, eq29_e349_d_n6, eq29_e349_d_n7, eq29_e349_d_n8, eq29_e349_d_n9];
        let eq29_branch_derivatives: [f64; 8] = [eq29_e349_d_b0, eq29_e349_d_b1, eq29_e349_d_b2, eq29_e349_d_b3, eq29_e349_d_b4, eq29_e349_d_b5, eq29_e349_d_b6, eq29_e349_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[4]),
            self.multiplicity * (eq29_value),
            &nodes,
            &eq29_node_derivatives,
            &branches,
            &eq29_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_30_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq30_e355,) = {
    if (s.v[127] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e355;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[4]),
            self.multiplicity * (eq30_value),
            &[
            ],
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
        let (eq31_e360,) = {
    if (!(s.v[127] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e360;
        stamper.stamp_potential(
            branches[7],
            eq31_value,
            &[
            ],
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
        let eq32_e363: f64 = (s.v[9] * s.v[37]);
        let eq32_e363_d_n0: f64 = (s.v[9] * s.dn[37][0]);
        let eq32_e363_d_n1: f64 = (s.v[9] * s.dn[37][1]);
        let eq32_e363_d_n2: f64 = (s.v[9] * s.dn[37][2]);
        let eq32_e363_d_n3: f64 = (s.v[9] * s.dn[37][3]);
        let eq32_e363_d_n4: f64 = (s.v[9] * s.dn[37][4]);
        let eq32_e363_d_n5: f64 = (s.v[9] * s.dn[37][5]);
        let eq32_e363_d_n6: f64 = (s.v[9] * s.dn[37][6]);
        let eq32_e363_d_n7: f64 = (s.v[9] * s.dn[37][7]);
        let eq32_e363_d_n8: f64 = (s.v[9] * s.dn[37][8]);
        let eq32_e363_d_n9: f64 = (s.v[9] * s.dn[37][9]);
        let eq32_e363_d_b0: f64 = (s.v[9] * s.db[37][0]);
        let eq32_e363_d_b1: f64 = (s.v[9] * s.db[37][1]);
        let eq32_e363_d_b2: f64 = (s.v[9] * s.db[37][2]);
        let eq32_e363_d_b3: f64 = (s.v[9] * s.db[37][3]);
        let eq32_e363_d_b4: f64 = (s.v[9] * s.db[37][4]);
        let eq32_e363_d_b5: f64 = (s.v[9] * s.db[37][5]);
        let eq32_e363_d_b6: f64 = (s.v[9] * s.db[37][6]);
        let eq32_e363_d_b7: f64 = (s.v[9] * s.db[37][7]);
        let eq32_e365: f64 = (eq32_e363 * s.v[3]);
        let eq32_e365_d_n0: f64 = (eq32_e363_d_n0 * s.v[3]);
        let eq32_e365_d_n1: f64 = (eq32_e363_d_n1 * s.v[3]);
        let eq32_e365_d_n2: f64 = (eq32_e363_d_n2 * s.v[3]);
        let eq32_e365_d_n3: f64 = (eq32_e363_d_n3 * s.v[3]);
        let eq32_e365_d_n4: f64 = (eq32_e363_d_n4 * s.v[3]);
        let eq32_e365_d_n5: f64 = (eq32_e363_d_n5 * s.v[3]);
        let eq32_e365_d_n6: f64 = (eq32_e363_d_n6 * s.v[3]);
        let eq32_e365_d_n7: f64 = (eq32_e363_d_n7 * s.v[3]);
        let eq32_e365_d_n8: f64 = (eq32_e363_d_n8 * s.v[3]);
        let eq32_e365_d_n9: f64 = (eq32_e363_d_n9 * s.v[3]);
        let eq32_e365_d_b0: f64 = (eq32_e363_d_b0 * s.v[3]);
        let eq32_e365_d_b1: f64 = (eq32_e363_d_b1 * s.v[3]);
        let eq32_e365_d_b2: f64 = (eq32_e363_d_b2 * s.v[3]);
        let eq32_e365_d_b3: f64 = (eq32_e363_d_b3 * s.v[3]);
        let eq32_e365_d_b4: f64 = (eq32_e363_d_b4 * s.v[3]);
        let eq32_e365_d_b5: f64 = (eq32_e363_d_b5 * s.v[3]);
        let eq32_e365_d_b6: f64 = (eq32_e363_d_b6 * s.v[3]);
        let eq32_e365_d_b7: f64 = (eq32_e363_d_b7 * s.v[3]);
        let eq32_value: f64 = eq32_e365;
        let eq32_node_derivatives: [f64; 10] = [eq32_e365_d_n0, eq32_e365_d_n1, eq32_e365_d_n2, eq32_e365_d_n3, eq32_e365_d_n4, eq32_e365_d_n5, eq32_e365_d_n6, eq32_e365_d_n7, eq32_e365_d_n8, eq32_e365_d_n9];
        let eq32_branch_derivatives: [f64; 8] = [eq32_e365_d_b0, eq32_e365_d_b1, eq32_e365_d_b2, eq32_e365_d_b3, eq32_e365_d_b4, eq32_e365_d_b5, eq32_e365_d_b6, eq32_e365_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let eq33_e368: f64 = (s.v[9] * s.v[40]);
        let eq33_e368_d_n0: f64 = (s.v[9] * s.dn[40][0]);
        let eq33_e368_d_n1: f64 = (s.v[9] * s.dn[40][1]);
        let eq33_e368_d_n2: f64 = (s.v[9] * s.dn[40][2]);
        let eq33_e368_d_n3: f64 = (s.v[9] * s.dn[40][3]);
        let eq33_e368_d_n4: f64 = (s.v[9] * s.dn[40][4]);
        let eq33_e368_d_n5: f64 = (s.v[9] * s.dn[40][5]);
        let eq33_e368_d_n6: f64 = (s.v[9] * s.dn[40][6]);
        let eq33_e368_d_n7: f64 = (s.v[9] * s.dn[40][7]);
        let eq33_e368_d_n8: f64 = (s.v[9] * s.dn[40][8]);
        let eq33_e368_d_n9: f64 = (s.v[9] * s.dn[40][9]);
        let eq33_e368_d_b0: f64 = (s.v[9] * s.db[40][0]);
        let eq33_e368_d_b1: f64 = (s.v[9] * s.db[40][1]);
        let eq33_e368_d_b2: f64 = (s.v[9] * s.db[40][2]);
        let eq33_e368_d_b3: f64 = (s.v[9] * s.db[40][3]);
        let eq33_e368_d_b4: f64 = (s.v[9] * s.db[40][4]);
        let eq33_e368_d_b5: f64 = (s.v[9] * s.db[40][5]);
        let eq33_e368_d_b6: f64 = (s.v[9] * s.db[40][6]);
        let eq33_e368_d_b7: f64 = (s.v[9] * s.db[40][7]);
        let eq33_e370: f64 = (eq33_e368 * s.v[3]);
        let eq33_e370_d_n0: f64 = (eq33_e368_d_n0 * s.v[3]);
        let eq33_e370_d_n1: f64 = (eq33_e368_d_n1 * s.v[3]);
        let eq33_e370_d_n2: f64 = (eq33_e368_d_n2 * s.v[3]);
        let eq33_e370_d_n3: f64 = (eq33_e368_d_n3 * s.v[3]);
        let eq33_e370_d_n4: f64 = (eq33_e368_d_n4 * s.v[3]);
        let eq33_e370_d_n5: f64 = (eq33_e368_d_n5 * s.v[3]);
        let eq33_e370_d_n6: f64 = (eq33_e368_d_n6 * s.v[3]);
        let eq33_e370_d_n7: f64 = (eq33_e368_d_n7 * s.v[3]);
        let eq33_e370_d_n8: f64 = (eq33_e368_d_n8 * s.v[3]);
        let eq33_e370_d_n9: f64 = (eq33_e368_d_n9 * s.v[3]);
        let eq33_e370_d_b0: f64 = (eq33_e368_d_b0 * s.v[3]);
        let eq33_e370_d_b1: f64 = (eq33_e368_d_b1 * s.v[3]);
        let eq33_e370_d_b2: f64 = (eq33_e368_d_b2 * s.v[3]);
        let eq33_e370_d_b3: f64 = (eq33_e368_d_b3 * s.v[3]);
        let eq33_e370_d_b4: f64 = (eq33_e368_d_b4 * s.v[3]);
        let eq33_e370_d_b5: f64 = (eq33_e368_d_b5 * s.v[3]);
        let eq33_e370_d_b6: f64 = (eq33_e368_d_b6 * s.v[3]);
        let eq33_e370_d_b7: f64 = (eq33_e368_d_b7 * s.v[3]);
        let eq33_value: f64 = eq33_e370;
        let eq33_node_derivatives: [f64; 10] = [eq33_e370_d_n0, eq33_e370_d_n1, eq33_e370_d_n2, eq33_e370_d_n3, eq33_e370_d_n4, eq33_e370_d_n5, eq33_e370_d_n6, eq33_e370_d_n7, eq33_e370_d_n8, eq33_e370_d_n9];
        let eq33_branch_derivatives: [f64; 8] = [eq33_e370_d_b0, eq33_e370_d_b1, eq33_e370_d_b2, eq33_e370_d_b3, eq33_e370_d_b4, eq33_e370_d_b5, eq33_e370_d_b6, eq33_e370_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[4]),
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
        let eq34_e373: f64 = (-s.v[45]);
        let eq34_e373_d_n0: f64 = (-s.dn[45][0]);
        let eq34_e373_d_n1: f64 = (-s.dn[45][1]);
        let eq34_e373_d_n2: f64 = (-s.dn[45][2]);
        let eq34_e373_d_n3: f64 = (-s.dn[45][3]);
        let eq34_e373_d_n4: f64 = (-s.dn[45][4]);
        let eq34_e373_d_n5: f64 = (-s.dn[45][5]);
        let eq34_e373_d_n6: f64 = (-s.dn[45][6]);
        let eq34_e373_d_n7: f64 = (-s.dn[45][7]);
        let eq34_e373_d_n8: f64 = (-s.dn[45][8]);
        let eq34_e373_d_n9: f64 = (-s.dn[45][9]);
        let eq34_e373_d_b0: f64 = (-s.db[45][0]);
        let eq34_e373_d_b1: f64 = (-s.db[45][1]);
        let eq34_e373_d_b2: f64 = (-s.db[45][2]);
        let eq34_e373_d_b3: f64 = (-s.db[45][3]);
        let eq34_e373_d_b4: f64 = (-s.db[45][4]);
        let eq34_e373_d_b5: f64 = (-s.db[45][5]);
        let eq34_e373_d_b6: f64 = (-s.db[45][6]);
        let eq34_e373_d_b7: f64 = (-s.db[45][7]);
        let eq34_e375: f64 = (eq34_e373 * s.v[3]);
        let eq34_e375_d_n0: f64 = (eq34_e373_d_n0 * s.v[3]);
        let eq34_e375_d_n1: f64 = (eq34_e373_d_n1 * s.v[3]);
        let eq34_e375_d_n2: f64 = (eq34_e373_d_n2 * s.v[3]);
        let eq34_e375_d_n3: f64 = (eq34_e373_d_n3 * s.v[3]);
        let eq34_e375_d_n4: f64 = (eq34_e373_d_n4 * s.v[3]);
        let eq34_e375_d_n5: f64 = (eq34_e373_d_n5 * s.v[3]);
        let eq34_e375_d_n6: f64 = (eq34_e373_d_n6 * s.v[3]);
        let eq34_e375_d_n7: f64 = (eq34_e373_d_n7 * s.v[3]);
        let eq34_e375_d_n8: f64 = (eq34_e373_d_n8 * s.v[3]);
        let eq34_e375_d_n9: f64 = (eq34_e373_d_n9 * s.v[3]);
        let eq34_e375_d_b0: f64 = (eq34_e373_d_b0 * s.v[3]);
        let eq34_e375_d_b1: f64 = (eq34_e373_d_b1 * s.v[3]);
        let eq34_e375_d_b2: f64 = (eq34_e373_d_b2 * s.v[3]);
        let eq34_e375_d_b3: f64 = (eq34_e373_d_b3 * s.v[3]);
        let eq34_e375_d_b4: f64 = (eq34_e373_d_b4 * s.v[3]);
        let eq34_e375_d_b5: f64 = (eq34_e373_d_b5 * s.v[3]);
        let eq34_e375_d_b6: f64 = (eq34_e373_d_b6 * s.v[3]);
        let eq34_e375_d_b7: f64 = (eq34_e373_d_b7 * s.v[3]);
        let eq34_e376: f64 = (s.v[9] * eq34_e375);
        let eq34_e376_d_n0: f64 = (s.v[9] * eq34_e375_d_n0);
        let eq34_e376_d_n1: f64 = (s.v[9] * eq34_e375_d_n1);
        let eq34_e376_d_n2: f64 = (s.v[9] * eq34_e375_d_n2);
        let eq34_e376_d_n3: f64 = (s.v[9] * eq34_e375_d_n3);
        let eq34_e376_d_n4: f64 = (s.v[9] * eq34_e375_d_n4);
        let eq34_e376_d_n5: f64 = (s.v[9] * eq34_e375_d_n5);
        let eq34_e376_d_n6: f64 = (s.v[9] * eq34_e375_d_n6);
        let eq34_e376_d_n7: f64 = (s.v[9] * eq34_e375_d_n7);
        let eq34_e376_d_n8: f64 = (s.v[9] * eq34_e375_d_n8);
        let eq34_e376_d_n9: f64 = (s.v[9] * eq34_e375_d_n9);
        let eq34_e376_d_b0: f64 = (s.v[9] * eq34_e375_d_b0);
        let eq34_e376_d_b1: f64 = (s.v[9] * eq34_e375_d_b1);
        let eq34_e376_d_b2: f64 = (s.v[9] * eq34_e375_d_b2);
        let eq34_e376_d_b3: f64 = (s.v[9] * eq34_e375_d_b3);
        let eq34_e376_d_b4: f64 = (s.v[9] * eq34_e375_d_b4);
        let eq34_e376_d_b5: f64 = (s.v[9] * eq34_e375_d_b5);
        let eq34_e376_d_b6: f64 = (s.v[9] * eq34_e375_d_b6);
        let eq34_e376_d_b7: f64 = (s.v[9] * eq34_e375_d_b7);
        let eq34_value: f64 = eq34_e376;
        let eq34_node_derivatives: [f64; 10] = [eq34_e376_d_n0, eq34_e376_d_n1, eq34_e376_d_n2, eq34_e376_d_n3, eq34_e376_d_n4, eq34_e376_d_n5, eq34_e376_d_n6, eq34_e376_d_n7, eq34_e376_d_n8, eq34_e376_d_n9];
        let eq34_branch_derivatives: [f64; 8] = [eq34_e376_d_b0, eq34_e376_d_b1, eq34_e376_d_b2, eq34_e376_d_b3, eq34_e376_d_b4, eq34_e376_d_b5, eq34_e376_d_b6, eq34_e376_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[6]),
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
        let eq35_e379: f64 = (s.v[9] * s.v[46]);
        let eq35_e379_d_n0: f64 = (s.v[9] * s.dn[46][0]);
        let eq35_e379_d_n1: f64 = (s.v[9] * s.dn[46][1]);
        let eq35_e379_d_n2: f64 = (s.v[9] * s.dn[46][2]);
        let eq35_e379_d_n3: f64 = (s.v[9] * s.dn[46][3]);
        let eq35_e379_d_n4: f64 = (s.v[9] * s.dn[46][4]);
        let eq35_e379_d_n5: f64 = (s.v[9] * s.dn[46][5]);
        let eq35_e379_d_n6: f64 = (s.v[9] * s.dn[46][6]);
        let eq35_e379_d_n7: f64 = (s.v[9] * s.dn[46][7]);
        let eq35_e379_d_n8: f64 = (s.v[9] * s.dn[46][8]);
        let eq35_e379_d_n9: f64 = (s.v[9] * s.dn[46][9]);
        let eq35_e379_d_b0: f64 = (s.v[9] * s.db[46][0]);
        let eq35_e379_d_b1: f64 = (s.v[9] * s.db[46][1]);
        let eq35_e379_d_b2: f64 = (s.v[9] * s.db[46][2]);
        let eq35_e379_d_b3: f64 = (s.v[9] * s.db[46][3]);
        let eq35_e379_d_b4: f64 = (s.v[9] * s.db[46][4]);
        let eq35_e379_d_b5: f64 = (s.v[9] * s.db[46][5]);
        let eq35_e379_d_b6: f64 = (s.v[9] * s.db[46][6]);
        let eq35_e379_d_b7: f64 = (s.v[9] * s.db[46][7]);
        let eq35_e381: f64 = (eq35_e379 * s.v[3]);
        let eq35_e381_d_n0: f64 = (eq35_e379_d_n0 * s.v[3]);
        let eq35_e381_d_n1: f64 = (eq35_e379_d_n1 * s.v[3]);
        let eq35_e381_d_n2: f64 = (eq35_e379_d_n2 * s.v[3]);
        let eq35_e381_d_n3: f64 = (eq35_e379_d_n3 * s.v[3]);
        let eq35_e381_d_n4: f64 = (eq35_e379_d_n4 * s.v[3]);
        let eq35_e381_d_n5: f64 = (eq35_e379_d_n5 * s.v[3]);
        let eq35_e381_d_n6: f64 = (eq35_e379_d_n6 * s.v[3]);
        let eq35_e381_d_n7: f64 = (eq35_e379_d_n7 * s.v[3]);
        let eq35_e381_d_n8: f64 = (eq35_e379_d_n8 * s.v[3]);
        let eq35_e381_d_n9: f64 = (eq35_e379_d_n9 * s.v[3]);
        let eq35_e381_d_b0: f64 = (eq35_e379_d_b0 * s.v[3]);
        let eq35_e381_d_b1: f64 = (eq35_e379_d_b1 * s.v[3]);
        let eq35_e381_d_b2: f64 = (eq35_e379_d_b2 * s.v[3]);
        let eq35_e381_d_b3: f64 = (eq35_e379_d_b3 * s.v[3]);
        let eq35_e381_d_b4: f64 = (eq35_e379_d_b4 * s.v[3]);
        let eq35_e381_d_b5: f64 = (eq35_e379_d_b5 * s.v[3]);
        let eq35_e381_d_b6: f64 = (eq35_e379_d_b6 * s.v[3]);
        let eq35_e381_d_b7: f64 = (eq35_e379_d_b7 * s.v[3]);
        let eq35_value: f64 = eq35_e381;
        let eq35_node_derivatives: [f64; 10] = [eq35_e381_d_n0, eq35_e381_d_n1, eq35_e381_d_n2, eq35_e381_d_n3, eq35_e381_d_n4, eq35_e381_d_n5, eq35_e381_d_n6, eq35_e381_d_n7, eq35_e381_d_n8, eq35_e381_d_n9];
        let eq35_branch_derivatives: [f64; 8] = [eq35_e381_d_b0, eq35_e381_d_b1, eq35_e381_d_b2, eq35_e381_d_b3, eq35_e381_d_b4, eq35_e381_d_b5, eq35_e381_d_b6, eq35_e381_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[6]),
            self.multiplicity * (eq35_value),
            &nodes,
            &eq35_node_derivatives,
            &branches,
            &eq35_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq36_e384: f64 = (s.v[9] * s.v[58]);
        let eq36_e384_d_n0: f64 = (s.v[9] * s.dn[58][0]);
        let eq36_e384_d_n1: f64 = (s.v[9] * s.dn[58][1]);
        let eq36_e384_d_n2: f64 = (s.v[9] * s.dn[58][2]);
        let eq36_e384_d_n3: f64 = (s.v[9] * s.dn[58][3]);
        let eq36_e384_d_n4: f64 = (s.v[9] * s.dn[58][4]);
        let eq36_e384_d_n5: f64 = (s.v[9] * s.dn[58][5]);
        let eq36_e384_d_n6: f64 = (s.v[9] * s.dn[58][6]);
        let eq36_e384_d_n7: f64 = (s.v[9] * s.dn[58][7]);
        let eq36_e384_d_n8: f64 = (s.v[9] * s.dn[58][8]);
        let eq36_e384_d_n9: f64 = (s.v[9] * s.dn[58][9]);
        let eq36_e384_d_b0: f64 = (s.v[9] * s.db[58][0]);
        let eq36_e384_d_b1: f64 = (s.v[9] * s.db[58][1]);
        let eq36_e384_d_b2: f64 = (s.v[9] * s.db[58][2]);
        let eq36_e384_d_b3: f64 = (s.v[9] * s.db[58][3]);
        let eq36_e384_d_b4: f64 = (s.v[9] * s.db[58][4]);
        let eq36_e384_d_b5: f64 = (s.v[9] * s.db[58][5]);
        let eq36_e384_d_b6: f64 = (s.v[9] * s.db[58][6]);
        let eq36_e384_d_b7: f64 = (s.v[9] * s.db[58][7]);
        let eq36_e386: f64 = (eq36_e384 * s.v[3]);
        let eq36_e386_d_n0: f64 = (eq36_e384_d_n0 * s.v[3]);
        let eq36_e386_d_n1: f64 = (eq36_e384_d_n1 * s.v[3]);
        let eq36_e386_d_n2: f64 = (eq36_e384_d_n2 * s.v[3]);
        let eq36_e386_d_n3: f64 = (eq36_e384_d_n3 * s.v[3]);
        let eq36_e386_d_n4: f64 = (eq36_e384_d_n4 * s.v[3]);
        let eq36_e386_d_n5: f64 = (eq36_e384_d_n5 * s.v[3]);
        let eq36_e386_d_n6: f64 = (eq36_e384_d_n6 * s.v[3]);
        let eq36_e386_d_n7: f64 = (eq36_e384_d_n7 * s.v[3]);
        let eq36_e386_d_n8: f64 = (eq36_e384_d_n8 * s.v[3]);
        let eq36_e386_d_n9: f64 = (eq36_e384_d_n9 * s.v[3]);
        let eq36_e386_d_b0: f64 = (eq36_e384_d_b0 * s.v[3]);
        let eq36_e386_d_b1: f64 = (eq36_e384_d_b1 * s.v[3]);
        let eq36_e386_d_b2: f64 = (eq36_e384_d_b2 * s.v[3]);
        let eq36_e386_d_b3: f64 = (eq36_e384_d_b3 * s.v[3]);
        let eq36_e386_d_b4: f64 = (eq36_e384_d_b4 * s.v[3]);
        let eq36_e386_d_b5: f64 = (eq36_e384_d_b5 * s.v[3]);
        let eq36_e386_d_b6: f64 = (eq36_e384_d_b6 * s.v[3]);
        let eq36_e386_d_b7: f64 = (eq36_e384_d_b7 * s.v[3]);
        let eq36_e387: f64 = self.eval_ddt(5, eq36_e386);
        let eq36_e387_d_n0: f64 = self.ddt_jacobian(eq36_e386_d_n0);
        let eq36_e387_d_n1: f64 = self.ddt_jacobian(eq36_e386_d_n1);
        let eq36_e387_d_n2: f64 = self.ddt_jacobian(eq36_e386_d_n2);
        let eq36_e387_d_n3: f64 = self.ddt_jacobian(eq36_e386_d_n3);
        let eq36_e387_d_n4: f64 = self.ddt_jacobian(eq36_e386_d_n4);
        let eq36_e387_d_n5: f64 = self.ddt_jacobian(eq36_e386_d_n5);
        let eq36_e387_d_n6: f64 = self.ddt_jacobian(eq36_e386_d_n6);
        let eq36_e387_d_n7: f64 = self.ddt_jacobian(eq36_e386_d_n7);
        let eq36_e387_d_n8: f64 = self.ddt_jacobian(eq36_e386_d_n8);
        let eq36_e387_d_n9: f64 = self.ddt_jacobian(eq36_e386_d_n9);
        let eq36_e387_d_b0: f64 = self.ddt_jacobian(eq36_e386_d_b0);
        let eq36_e387_d_b1: f64 = self.ddt_jacobian(eq36_e386_d_b1);
        let eq36_e387_d_b2: f64 = self.ddt_jacobian(eq36_e386_d_b2);
        let eq36_e387_d_b3: f64 = self.ddt_jacobian(eq36_e386_d_b3);
        let eq36_e387_d_b4: f64 = self.ddt_jacobian(eq36_e386_d_b4);
        let eq36_e387_d_b5: f64 = self.ddt_jacobian(eq36_e386_d_b5);
        let eq36_e387_d_b6: f64 = self.ddt_jacobian(eq36_e386_d_b6);
        let eq36_e387_d_b7: f64 = self.ddt_jacobian(eq36_e386_d_b7);
        let eq36_value: f64 = eq36_e387;
        let eq36_node_derivatives: [f64; 10] = [eq36_e387_d_n0, eq36_e387_d_n1, eq36_e387_d_n2, eq36_e387_d_n3, eq36_e387_d_n4, eq36_e387_d_n5, eq36_e387_d_n6, eq36_e387_d_n7, eq36_e387_d_n8, eq36_e387_d_n9];
        let eq36_branch_derivatives: [f64; 8] = [eq36_e387_d_b0, eq36_e387_d_b1, eq36_e387_d_b2, eq36_e387_d_b3, eq36_e387_d_b4, eq36_e387_d_b5, eq36_e387_d_b6, eq36_e387_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq36_value),
            &nodes,
            &eq36_node_derivatives,
            &branches,
            &eq36_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq37_e390: f64 = (s.v[9] * s.v[55]);
        let eq37_e390_d_n0: f64 = (s.v[9] * s.dn[55][0]);
        let eq37_e390_d_n1: f64 = (s.v[9] * s.dn[55][1]);
        let eq37_e390_d_n2: f64 = (s.v[9] * s.dn[55][2]);
        let eq37_e390_d_n3: f64 = (s.v[9] * s.dn[55][3]);
        let eq37_e390_d_n4: f64 = (s.v[9] * s.dn[55][4]);
        let eq37_e390_d_n5: f64 = (s.v[9] * s.dn[55][5]);
        let eq37_e390_d_n6: f64 = (s.v[9] * s.dn[55][6]);
        let eq37_e390_d_n7: f64 = (s.v[9] * s.dn[55][7]);
        let eq37_e390_d_n8: f64 = (s.v[9] * s.dn[55][8]);
        let eq37_e390_d_n9: f64 = (s.v[9] * s.dn[55][9]);
        let eq37_e390_d_b0: f64 = (s.v[9] * s.db[55][0]);
        let eq37_e390_d_b1: f64 = (s.v[9] * s.db[55][1]);
        let eq37_e390_d_b2: f64 = (s.v[9] * s.db[55][2]);
        let eq37_e390_d_b3: f64 = (s.v[9] * s.db[55][3]);
        let eq37_e390_d_b4: f64 = (s.v[9] * s.db[55][4]);
        let eq37_e390_d_b5: f64 = (s.v[9] * s.db[55][5]);
        let eq37_e390_d_b6: f64 = (s.v[9] * s.db[55][6]);
        let eq37_e390_d_b7: f64 = (s.v[9] * s.db[55][7]);
        let eq37_e392: f64 = (eq37_e390 * s.v[3]);
        let eq37_e392_d_n0: f64 = (eq37_e390_d_n0 * s.v[3]);
        let eq37_e392_d_n1: f64 = (eq37_e390_d_n1 * s.v[3]);
        let eq37_e392_d_n2: f64 = (eq37_e390_d_n2 * s.v[3]);
        let eq37_e392_d_n3: f64 = (eq37_e390_d_n3 * s.v[3]);
        let eq37_e392_d_n4: f64 = (eq37_e390_d_n4 * s.v[3]);
        let eq37_e392_d_n5: f64 = (eq37_e390_d_n5 * s.v[3]);
        let eq37_e392_d_n6: f64 = (eq37_e390_d_n6 * s.v[3]);
        let eq37_e392_d_n7: f64 = (eq37_e390_d_n7 * s.v[3]);
        let eq37_e392_d_n8: f64 = (eq37_e390_d_n8 * s.v[3]);
        let eq37_e392_d_n9: f64 = (eq37_e390_d_n9 * s.v[3]);
        let eq37_e392_d_b0: f64 = (eq37_e390_d_b0 * s.v[3]);
        let eq37_e392_d_b1: f64 = (eq37_e390_d_b1 * s.v[3]);
        let eq37_e392_d_b2: f64 = (eq37_e390_d_b2 * s.v[3]);
        let eq37_e392_d_b3: f64 = (eq37_e390_d_b3 * s.v[3]);
        let eq37_e392_d_b4: f64 = (eq37_e390_d_b4 * s.v[3]);
        let eq37_e392_d_b5: f64 = (eq37_e390_d_b5 * s.v[3]);
        let eq37_e392_d_b6: f64 = (eq37_e390_d_b6 * s.v[3]);
        let eq37_e392_d_b7: f64 = (eq37_e390_d_b7 * s.v[3]);
        let eq37_e393: f64 = self.eval_ddt(6, eq37_e392);
        let eq37_e393_d_n0: f64 = self.ddt_jacobian(eq37_e392_d_n0);
        let eq37_e393_d_n1: f64 = self.ddt_jacobian(eq37_e392_d_n1);
        let eq37_e393_d_n2: f64 = self.ddt_jacobian(eq37_e392_d_n2);
        let eq37_e393_d_n3: f64 = self.ddt_jacobian(eq37_e392_d_n3);
        let eq37_e393_d_n4: f64 = self.ddt_jacobian(eq37_e392_d_n4);
        let eq37_e393_d_n5: f64 = self.ddt_jacobian(eq37_e392_d_n5);
        let eq37_e393_d_n6: f64 = self.ddt_jacobian(eq37_e392_d_n6);
        let eq37_e393_d_n7: f64 = self.ddt_jacobian(eq37_e392_d_n7);
        let eq37_e393_d_n8: f64 = self.ddt_jacobian(eq37_e392_d_n8);
        let eq37_e393_d_n9: f64 = self.ddt_jacobian(eq37_e392_d_n9);
        let eq37_e393_d_b0: f64 = self.ddt_jacobian(eq37_e392_d_b0);
        let eq37_e393_d_b1: f64 = self.ddt_jacobian(eq37_e392_d_b1);
        let eq37_e393_d_b2: f64 = self.ddt_jacobian(eq37_e392_d_b2);
        let eq37_e393_d_b3: f64 = self.ddt_jacobian(eq37_e392_d_b3);
        let eq37_e393_d_b4: f64 = self.ddt_jacobian(eq37_e392_d_b4);
        let eq37_e393_d_b5: f64 = self.ddt_jacobian(eq37_e392_d_b5);
        let eq37_e393_d_b6: f64 = self.ddt_jacobian(eq37_e392_d_b6);
        let eq37_e393_d_b7: f64 = self.ddt_jacobian(eq37_e392_d_b7);
        let eq37_value: f64 = eq37_e393;
        let eq37_node_derivatives: [f64; 10] = [eq37_e393_d_n0, eq37_e393_d_n1, eq37_e393_d_n2, eq37_e393_d_n3, eq37_e393_d_n4, eq37_e393_d_n5, eq37_e393_d_n6, eq37_e393_d_n7, eq37_e393_d_n8, eq37_e393_d_n9];
        let eq37_branch_derivatives: [f64; 8] = [eq37_e393_d_b0, eq37_e393_d_b1, eq37_e393_d_b2, eq37_e393_d_b3, eq37_e393_d_b4, eq37_e393_d_b5, eq37_e393_d_b6, eq37_e393_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq37_value),
            &nodes,
            &eq37_node_derivatives,
            &branches,
            &eq37_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_38_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq38_e396: f64 = (s.v[9] * s.v[60]);
        let eq38_e396_d_n0: f64 = (s.v[9] * s.dn[60][0]);
        let eq38_e396_d_n1: f64 = (s.v[9] * s.dn[60][1]);
        let eq38_e396_d_n2: f64 = (s.v[9] * s.dn[60][2]);
        let eq38_e396_d_n3: f64 = (s.v[9] * s.dn[60][3]);
        let eq38_e396_d_n4: f64 = (s.v[9] * s.dn[60][4]);
        let eq38_e396_d_n5: f64 = (s.v[9] * s.dn[60][5]);
        let eq38_e396_d_n6: f64 = (s.v[9] * s.dn[60][6]);
        let eq38_e396_d_n7: f64 = (s.v[9] * s.dn[60][7]);
        let eq38_e396_d_n8: f64 = (s.v[9] * s.dn[60][8]);
        let eq38_e396_d_n9: f64 = (s.v[9] * s.dn[60][9]);
        let eq38_e396_d_b0: f64 = (s.v[9] * s.db[60][0]);
        let eq38_e396_d_b1: f64 = (s.v[9] * s.db[60][1]);
        let eq38_e396_d_b2: f64 = (s.v[9] * s.db[60][2]);
        let eq38_e396_d_b3: f64 = (s.v[9] * s.db[60][3]);
        let eq38_e396_d_b4: f64 = (s.v[9] * s.db[60][4]);
        let eq38_e396_d_b5: f64 = (s.v[9] * s.db[60][5]);
        let eq38_e396_d_b6: f64 = (s.v[9] * s.db[60][6]);
        let eq38_e396_d_b7: f64 = (s.v[9] * s.db[60][7]);
        let eq38_e398: f64 = (eq38_e396 * s.v[3]);
        let eq38_e398_d_n0: f64 = (eq38_e396_d_n0 * s.v[3]);
        let eq38_e398_d_n1: f64 = (eq38_e396_d_n1 * s.v[3]);
        let eq38_e398_d_n2: f64 = (eq38_e396_d_n2 * s.v[3]);
        let eq38_e398_d_n3: f64 = (eq38_e396_d_n3 * s.v[3]);
        let eq38_e398_d_n4: f64 = (eq38_e396_d_n4 * s.v[3]);
        let eq38_e398_d_n5: f64 = (eq38_e396_d_n5 * s.v[3]);
        let eq38_e398_d_n6: f64 = (eq38_e396_d_n6 * s.v[3]);
        let eq38_e398_d_n7: f64 = (eq38_e396_d_n7 * s.v[3]);
        let eq38_e398_d_n8: f64 = (eq38_e396_d_n8 * s.v[3]);
        let eq38_e398_d_n9: f64 = (eq38_e396_d_n9 * s.v[3]);
        let eq38_e398_d_b0: f64 = (eq38_e396_d_b0 * s.v[3]);
        let eq38_e398_d_b1: f64 = (eq38_e396_d_b1 * s.v[3]);
        let eq38_e398_d_b2: f64 = (eq38_e396_d_b2 * s.v[3]);
        let eq38_e398_d_b3: f64 = (eq38_e396_d_b3 * s.v[3]);
        let eq38_e398_d_b4: f64 = (eq38_e396_d_b4 * s.v[3]);
        let eq38_e398_d_b5: f64 = (eq38_e396_d_b5 * s.v[3]);
        let eq38_e398_d_b6: f64 = (eq38_e396_d_b6 * s.v[3]);
        let eq38_e398_d_b7: f64 = (eq38_e396_d_b7 * s.v[3]);
        let eq38_e399: f64 = self.eval_ddt(7, eq38_e398);
        let eq38_e399_d_n0: f64 = self.ddt_jacobian(eq38_e398_d_n0);
        let eq38_e399_d_n1: f64 = self.ddt_jacobian(eq38_e398_d_n1);
        let eq38_e399_d_n2: f64 = self.ddt_jacobian(eq38_e398_d_n2);
        let eq38_e399_d_n3: f64 = self.ddt_jacobian(eq38_e398_d_n3);
        let eq38_e399_d_n4: f64 = self.ddt_jacobian(eq38_e398_d_n4);
        let eq38_e399_d_n5: f64 = self.ddt_jacobian(eq38_e398_d_n5);
        let eq38_e399_d_n6: f64 = self.ddt_jacobian(eq38_e398_d_n6);
        let eq38_e399_d_n7: f64 = self.ddt_jacobian(eq38_e398_d_n7);
        let eq38_e399_d_n8: f64 = self.ddt_jacobian(eq38_e398_d_n8);
        let eq38_e399_d_n9: f64 = self.ddt_jacobian(eq38_e398_d_n9);
        let eq38_e399_d_b0: f64 = self.ddt_jacobian(eq38_e398_d_b0);
        let eq38_e399_d_b1: f64 = self.ddt_jacobian(eq38_e398_d_b1);
        let eq38_e399_d_b2: f64 = self.ddt_jacobian(eq38_e398_d_b2);
        let eq38_e399_d_b3: f64 = self.ddt_jacobian(eq38_e398_d_b3);
        let eq38_e399_d_b4: f64 = self.ddt_jacobian(eq38_e398_d_b4);
        let eq38_e399_d_b5: f64 = self.ddt_jacobian(eq38_e398_d_b5);
        let eq38_e399_d_b6: f64 = self.ddt_jacobian(eq38_e398_d_b6);
        let eq38_e399_d_b7: f64 = self.ddt_jacobian(eq38_e398_d_b7);
        let eq38_value: f64 = eq38_e399;
        let eq38_node_derivatives: [f64; 10] = [eq38_e399_d_n0, eq38_e399_d_n1, eq38_e399_d_n2, eq38_e399_d_n3, eq38_e399_d_n4, eq38_e399_d_n5, eq38_e399_d_n6, eq38_e399_d_n7, eq38_e399_d_n8, eq38_e399_d_n9];
        let eq38_branch_derivatives: [f64; 8] = [eq38_e399_d_b0, eq38_e399_d_b1, eq38_e399_d_b2, eq38_e399_d_b3, eq38_e399_d_b4, eq38_e399_d_b5, eq38_e399_d_b6, eq38_e399_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            self.multiplicity * (eq38_value),
            &nodes,
            &eq38_node_derivatives,
            &branches,
            &eq38_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_39_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq39_e402: f64 = (s.v[9] * s.v[62]);
        let eq39_e402_d_n0: f64 = (s.v[9] * s.dn[62][0]);
        let eq39_e402_d_n1: f64 = (s.v[9] * s.dn[62][1]);
        let eq39_e402_d_n2: f64 = (s.v[9] * s.dn[62][2]);
        let eq39_e402_d_n3: f64 = (s.v[9] * s.dn[62][3]);
        let eq39_e402_d_n4: f64 = (s.v[9] * s.dn[62][4]);
        let eq39_e402_d_n5: f64 = (s.v[9] * s.dn[62][5]);
        let eq39_e402_d_n6: f64 = (s.v[9] * s.dn[62][6]);
        let eq39_e402_d_n7: f64 = (s.v[9] * s.dn[62][7]);
        let eq39_e402_d_n8: f64 = (s.v[9] * s.dn[62][8]);
        let eq39_e402_d_n9: f64 = (s.v[9] * s.dn[62][9]);
        let eq39_e402_d_b0: f64 = (s.v[9] * s.db[62][0]);
        let eq39_e402_d_b1: f64 = (s.v[9] * s.db[62][1]);
        let eq39_e402_d_b2: f64 = (s.v[9] * s.db[62][2]);
        let eq39_e402_d_b3: f64 = (s.v[9] * s.db[62][3]);
        let eq39_e402_d_b4: f64 = (s.v[9] * s.db[62][4]);
        let eq39_e402_d_b5: f64 = (s.v[9] * s.db[62][5]);
        let eq39_e402_d_b6: f64 = (s.v[9] * s.db[62][6]);
        let eq39_e402_d_b7: f64 = (s.v[9] * s.db[62][7]);
        let eq39_e404: f64 = (eq39_e402 * s.v[3]);
        let eq39_e404_d_n0: f64 = (eq39_e402_d_n0 * s.v[3]);
        let eq39_e404_d_n1: f64 = (eq39_e402_d_n1 * s.v[3]);
        let eq39_e404_d_n2: f64 = (eq39_e402_d_n2 * s.v[3]);
        let eq39_e404_d_n3: f64 = (eq39_e402_d_n3 * s.v[3]);
        let eq39_e404_d_n4: f64 = (eq39_e402_d_n4 * s.v[3]);
        let eq39_e404_d_n5: f64 = (eq39_e402_d_n5 * s.v[3]);
        let eq39_e404_d_n6: f64 = (eq39_e402_d_n6 * s.v[3]);
        let eq39_e404_d_n7: f64 = (eq39_e402_d_n7 * s.v[3]);
        let eq39_e404_d_n8: f64 = (eq39_e402_d_n8 * s.v[3]);
        let eq39_e404_d_n9: f64 = (eq39_e402_d_n9 * s.v[3]);
        let eq39_e404_d_b0: f64 = (eq39_e402_d_b0 * s.v[3]);
        let eq39_e404_d_b1: f64 = (eq39_e402_d_b1 * s.v[3]);
        let eq39_e404_d_b2: f64 = (eq39_e402_d_b2 * s.v[3]);
        let eq39_e404_d_b3: f64 = (eq39_e402_d_b3 * s.v[3]);
        let eq39_e404_d_b4: f64 = (eq39_e402_d_b4 * s.v[3]);
        let eq39_e404_d_b5: f64 = (eq39_e402_d_b5 * s.v[3]);
        let eq39_e404_d_b6: f64 = (eq39_e402_d_b6 * s.v[3]);
        let eq39_e404_d_b7: f64 = (eq39_e402_d_b7 * s.v[3]);
        let eq39_e405: f64 = self.eval_ddt(8, eq39_e404);
        let eq39_e405_d_n0: f64 = self.ddt_jacobian(eq39_e404_d_n0);
        let eq39_e405_d_n1: f64 = self.ddt_jacobian(eq39_e404_d_n1);
        let eq39_e405_d_n2: f64 = self.ddt_jacobian(eq39_e404_d_n2);
        let eq39_e405_d_n3: f64 = self.ddt_jacobian(eq39_e404_d_n3);
        let eq39_e405_d_n4: f64 = self.ddt_jacobian(eq39_e404_d_n4);
        let eq39_e405_d_n5: f64 = self.ddt_jacobian(eq39_e404_d_n5);
        let eq39_e405_d_n6: f64 = self.ddt_jacobian(eq39_e404_d_n6);
        let eq39_e405_d_n7: f64 = self.ddt_jacobian(eq39_e404_d_n7);
        let eq39_e405_d_n8: f64 = self.ddt_jacobian(eq39_e404_d_n8);
        let eq39_e405_d_n9: f64 = self.ddt_jacobian(eq39_e404_d_n9);
        let eq39_e405_d_b0: f64 = self.ddt_jacobian(eq39_e404_d_b0);
        let eq39_e405_d_b1: f64 = self.ddt_jacobian(eq39_e404_d_b1);
        let eq39_e405_d_b2: f64 = self.ddt_jacobian(eq39_e404_d_b2);
        let eq39_e405_d_b3: f64 = self.ddt_jacobian(eq39_e404_d_b3);
        let eq39_e405_d_b4: f64 = self.ddt_jacobian(eq39_e404_d_b4);
        let eq39_e405_d_b5: f64 = self.ddt_jacobian(eq39_e404_d_b5);
        let eq39_e405_d_b6: f64 = self.ddt_jacobian(eq39_e404_d_b6);
        let eq39_e405_d_b7: f64 = self.ddt_jacobian(eq39_e404_d_b7);
        let eq39_value: f64 = eq39_e405;
        let eq39_node_derivatives: [f64; 10] = [eq39_e405_d_n0, eq39_e405_d_n1, eq39_e405_d_n2, eq39_e405_d_n3, eq39_e405_d_n4, eq39_e405_d_n5, eq39_e405_d_n6, eq39_e405_d_n7, eq39_e405_d_n8, eq39_e405_d_n9];
        let eq39_branch_derivatives: [f64; 8] = [eq39_e405_d_b0, eq39_e405_d_b1, eq39_e405_d_b2, eq39_e405_d_b3, eq39_e405_d_b4, eq39_e405_d_b5, eq39_e405_d_b6, eq39_e405_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            self.multiplicity * (eq39_value),
            &nodes,
            &eq39_node_derivatives,
            &branches,
            &eq39_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_40_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq40_e408: f64 = (s.v[9] * s.v[56]);
        let eq40_e408_d_n0: f64 = (s.v[9] * s.dn[56][0]);
        let eq40_e408_d_n1: f64 = (s.v[9] * s.dn[56][1]);
        let eq40_e408_d_n2: f64 = (s.v[9] * s.dn[56][2]);
        let eq40_e408_d_n3: f64 = (s.v[9] * s.dn[56][3]);
        let eq40_e408_d_n4: f64 = (s.v[9] * s.dn[56][4]);
        let eq40_e408_d_n5: f64 = (s.v[9] * s.dn[56][5]);
        let eq40_e408_d_n6: f64 = (s.v[9] * s.dn[56][6]);
        let eq40_e408_d_n7: f64 = (s.v[9] * s.dn[56][7]);
        let eq40_e408_d_n8: f64 = (s.v[9] * s.dn[56][8]);
        let eq40_e408_d_n9: f64 = (s.v[9] * s.dn[56][9]);
        let eq40_e408_d_b0: f64 = (s.v[9] * s.db[56][0]);
        let eq40_e408_d_b1: f64 = (s.v[9] * s.db[56][1]);
        let eq40_e408_d_b2: f64 = (s.v[9] * s.db[56][2]);
        let eq40_e408_d_b3: f64 = (s.v[9] * s.db[56][3]);
        let eq40_e408_d_b4: f64 = (s.v[9] * s.db[56][4]);
        let eq40_e408_d_b5: f64 = (s.v[9] * s.db[56][5]);
        let eq40_e408_d_b6: f64 = (s.v[9] * s.db[56][6]);
        let eq40_e408_d_b7: f64 = (s.v[9] * s.db[56][7]);
        let eq40_e410: f64 = (eq40_e408 * s.v[3]);
        let eq40_e410_d_n0: f64 = (eq40_e408_d_n0 * s.v[3]);
        let eq40_e410_d_n1: f64 = (eq40_e408_d_n1 * s.v[3]);
        let eq40_e410_d_n2: f64 = (eq40_e408_d_n2 * s.v[3]);
        let eq40_e410_d_n3: f64 = (eq40_e408_d_n3 * s.v[3]);
        let eq40_e410_d_n4: f64 = (eq40_e408_d_n4 * s.v[3]);
        let eq40_e410_d_n5: f64 = (eq40_e408_d_n5 * s.v[3]);
        let eq40_e410_d_n6: f64 = (eq40_e408_d_n6 * s.v[3]);
        let eq40_e410_d_n7: f64 = (eq40_e408_d_n7 * s.v[3]);
        let eq40_e410_d_n8: f64 = (eq40_e408_d_n8 * s.v[3]);
        let eq40_e410_d_n9: f64 = (eq40_e408_d_n9 * s.v[3]);
        let eq40_e410_d_b0: f64 = (eq40_e408_d_b0 * s.v[3]);
        let eq40_e410_d_b1: f64 = (eq40_e408_d_b1 * s.v[3]);
        let eq40_e410_d_b2: f64 = (eq40_e408_d_b2 * s.v[3]);
        let eq40_e410_d_b3: f64 = (eq40_e408_d_b3 * s.v[3]);
        let eq40_e410_d_b4: f64 = (eq40_e408_d_b4 * s.v[3]);
        let eq40_e410_d_b5: f64 = (eq40_e408_d_b5 * s.v[3]);
        let eq40_e410_d_b6: f64 = (eq40_e408_d_b6 * s.v[3]);
        let eq40_e410_d_b7: f64 = (eq40_e408_d_b7 * s.v[3]);
        let eq40_e411: f64 = self.eval_ddt(9, eq40_e410);
        let eq40_e411_d_n0: f64 = self.ddt_jacobian(eq40_e410_d_n0);
        let eq40_e411_d_n1: f64 = self.ddt_jacobian(eq40_e410_d_n1);
        let eq40_e411_d_n2: f64 = self.ddt_jacobian(eq40_e410_d_n2);
        let eq40_e411_d_n3: f64 = self.ddt_jacobian(eq40_e410_d_n3);
        let eq40_e411_d_n4: f64 = self.ddt_jacobian(eq40_e410_d_n4);
        let eq40_e411_d_n5: f64 = self.ddt_jacobian(eq40_e410_d_n5);
        let eq40_e411_d_n6: f64 = self.ddt_jacobian(eq40_e410_d_n6);
        let eq40_e411_d_n7: f64 = self.ddt_jacobian(eq40_e410_d_n7);
        let eq40_e411_d_n8: f64 = self.ddt_jacobian(eq40_e410_d_n8);
        let eq40_e411_d_n9: f64 = self.ddt_jacobian(eq40_e410_d_n9);
        let eq40_e411_d_b0: f64 = self.ddt_jacobian(eq40_e410_d_b0);
        let eq40_e411_d_b1: f64 = self.ddt_jacobian(eq40_e410_d_b1);
        let eq40_e411_d_b2: f64 = self.ddt_jacobian(eq40_e410_d_b2);
        let eq40_e411_d_b3: f64 = self.ddt_jacobian(eq40_e410_d_b3);
        let eq40_e411_d_b4: f64 = self.ddt_jacobian(eq40_e410_d_b4);
        let eq40_e411_d_b5: f64 = self.ddt_jacobian(eq40_e410_d_b5);
        let eq40_e411_d_b6: f64 = self.ddt_jacobian(eq40_e410_d_b6);
        let eq40_e411_d_b7: f64 = self.ddt_jacobian(eq40_e410_d_b7);
        let eq40_value: f64 = eq40_e411;
        let eq40_node_derivatives: [f64; 10] = [eq40_e411_d_n0, eq40_e411_d_n1, eq40_e411_d_n2, eq40_e411_d_n3, eq40_e411_d_n4, eq40_e411_d_n5, eq40_e411_d_n6, eq40_e411_d_n7, eq40_e411_d_n8, eq40_e411_d_n9];
        let eq40_branch_derivatives: [f64; 8] = [eq40_e411_d_b0, eq40_e411_d_b1, eq40_e411_d_b2, eq40_e411_d_b3, eq40_e411_d_b4, eq40_e411_d_b5, eq40_e411_d_b6, eq40_e411_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            self.multiplicity * (eq40_value),
            &nodes,
            &eq40_node_derivatives,
            &branches,
            &eq40_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_41_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq41_e414: f64 = (s.v[9] * s.v[57]);
        let eq41_e414_d_n0: f64 = (s.v[9] * s.dn[57][0]);
        let eq41_e414_d_n1: f64 = (s.v[9] * s.dn[57][1]);
        let eq41_e414_d_n2: f64 = (s.v[9] * s.dn[57][2]);
        let eq41_e414_d_n3: f64 = (s.v[9] * s.dn[57][3]);
        let eq41_e414_d_n4: f64 = (s.v[9] * s.dn[57][4]);
        let eq41_e414_d_n5: f64 = (s.v[9] * s.dn[57][5]);
        let eq41_e414_d_n6: f64 = (s.v[9] * s.dn[57][6]);
        let eq41_e414_d_n7: f64 = (s.v[9] * s.dn[57][7]);
        let eq41_e414_d_n8: f64 = (s.v[9] * s.dn[57][8]);
        let eq41_e414_d_n9: f64 = (s.v[9] * s.dn[57][9]);
        let eq41_e414_d_b0: f64 = (s.v[9] * s.db[57][0]);
        let eq41_e414_d_b1: f64 = (s.v[9] * s.db[57][1]);
        let eq41_e414_d_b2: f64 = (s.v[9] * s.db[57][2]);
        let eq41_e414_d_b3: f64 = (s.v[9] * s.db[57][3]);
        let eq41_e414_d_b4: f64 = (s.v[9] * s.db[57][4]);
        let eq41_e414_d_b5: f64 = (s.v[9] * s.db[57][5]);
        let eq41_e414_d_b6: f64 = (s.v[9] * s.db[57][6]);
        let eq41_e414_d_b7: f64 = (s.v[9] * s.db[57][7]);
        let eq41_e416: f64 = (eq41_e414 * s.v[3]);
        let eq41_e416_d_n0: f64 = (eq41_e414_d_n0 * s.v[3]);
        let eq41_e416_d_n1: f64 = (eq41_e414_d_n1 * s.v[3]);
        let eq41_e416_d_n2: f64 = (eq41_e414_d_n2 * s.v[3]);
        let eq41_e416_d_n3: f64 = (eq41_e414_d_n3 * s.v[3]);
        let eq41_e416_d_n4: f64 = (eq41_e414_d_n4 * s.v[3]);
        let eq41_e416_d_n5: f64 = (eq41_e414_d_n5 * s.v[3]);
        let eq41_e416_d_n6: f64 = (eq41_e414_d_n6 * s.v[3]);
        let eq41_e416_d_n7: f64 = (eq41_e414_d_n7 * s.v[3]);
        let eq41_e416_d_n8: f64 = (eq41_e414_d_n8 * s.v[3]);
        let eq41_e416_d_n9: f64 = (eq41_e414_d_n9 * s.v[3]);
        let eq41_e416_d_b0: f64 = (eq41_e414_d_b0 * s.v[3]);
        let eq41_e416_d_b1: f64 = (eq41_e414_d_b1 * s.v[3]);
        let eq41_e416_d_b2: f64 = (eq41_e414_d_b2 * s.v[3]);
        let eq41_e416_d_b3: f64 = (eq41_e414_d_b3 * s.v[3]);
        let eq41_e416_d_b4: f64 = (eq41_e414_d_b4 * s.v[3]);
        let eq41_e416_d_b5: f64 = (eq41_e414_d_b5 * s.v[3]);
        let eq41_e416_d_b6: f64 = (eq41_e414_d_b6 * s.v[3]);
        let eq41_e416_d_b7: f64 = (eq41_e414_d_b7 * s.v[3]);
        let eq41_e417: f64 = self.eval_ddt(10, eq41_e416);
        let eq41_e417_d_n0: f64 = self.ddt_jacobian(eq41_e416_d_n0);
        let eq41_e417_d_n1: f64 = self.ddt_jacobian(eq41_e416_d_n1);
        let eq41_e417_d_n2: f64 = self.ddt_jacobian(eq41_e416_d_n2);
        let eq41_e417_d_n3: f64 = self.ddt_jacobian(eq41_e416_d_n3);
        let eq41_e417_d_n4: f64 = self.ddt_jacobian(eq41_e416_d_n4);
        let eq41_e417_d_n5: f64 = self.ddt_jacobian(eq41_e416_d_n5);
        let eq41_e417_d_n6: f64 = self.ddt_jacobian(eq41_e416_d_n6);
        let eq41_e417_d_n7: f64 = self.ddt_jacobian(eq41_e416_d_n7);
        let eq41_e417_d_n8: f64 = self.ddt_jacobian(eq41_e416_d_n8);
        let eq41_e417_d_n9: f64 = self.ddt_jacobian(eq41_e416_d_n9);
        let eq41_e417_d_b0: f64 = self.ddt_jacobian(eq41_e416_d_b0);
        let eq41_e417_d_b1: f64 = self.ddt_jacobian(eq41_e416_d_b1);
        let eq41_e417_d_b2: f64 = self.ddt_jacobian(eq41_e416_d_b2);
        let eq41_e417_d_b3: f64 = self.ddt_jacobian(eq41_e416_d_b3);
        let eq41_e417_d_b4: f64 = self.ddt_jacobian(eq41_e416_d_b4);
        let eq41_e417_d_b5: f64 = self.ddt_jacobian(eq41_e416_d_b5);
        let eq41_e417_d_b6: f64 = self.ddt_jacobian(eq41_e416_d_b6);
        let eq41_e417_d_b7: f64 = self.ddt_jacobian(eq41_e416_d_b7);
        let eq41_value: f64 = eq41_e417;
        let eq41_node_derivatives: [f64; 10] = [eq41_e417_d_n0, eq41_e417_d_n1, eq41_e417_d_n2, eq41_e417_d_n3, eq41_e417_d_n4, eq41_e417_d_n5, eq41_e417_d_n6, eq41_e417_d_n7, eq41_e417_d_n8, eq41_e417_d_n9];
        let eq41_branch_derivatives: [f64; 8] = [eq41_e417_d_b0, eq41_e417_d_b1, eq41_e417_d_b2, eq41_e417_d_b3, eq41_e417_d_b4, eq41_e417_d_b5, eq41_e417_d_b6, eq41_e417_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[4]),
            self.multiplicity * (eq41_value),
            &nodes,
            &eq41_node_derivatives,
            &branches,
            &eq41_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let eq42_e419: f64 = (-s.v[63]);
        let eq42_e419_d_n0: f64 = (-s.dn[63][0]);
        let eq42_e419_d_n1: f64 = (-s.dn[63][1]);
        let eq42_e419_d_n2: f64 = (-s.dn[63][2]);
        let eq42_e419_d_n3: f64 = (-s.dn[63][3]);
        let eq42_e419_d_n4: f64 = (-s.dn[63][4]);
        let eq42_e419_d_n5: f64 = (-s.dn[63][5]);
        let eq42_e419_d_n6: f64 = (-s.dn[63][6]);
        let eq42_e419_d_n7: f64 = (-s.dn[63][7]);
        let eq42_e419_d_n8: f64 = (-s.dn[63][8]);
        let eq42_e419_d_n9: f64 = (-s.dn[63][9]);
        let eq42_e419_d_b0: f64 = (-s.db[63][0]);
        let eq42_e419_d_b1: f64 = (-s.db[63][1]);
        let eq42_e419_d_b2: f64 = (-s.db[63][2]);
        let eq42_e419_d_b3: f64 = (-s.db[63][3]);
        let eq42_e419_d_b4: f64 = (-s.db[63][4]);
        let eq42_e419_d_b5: f64 = (-s.db[63][5]);
        let eq42_e419_d_b6: f64 = (-s.db[63][6]);
        let eq42_e419_d_b7: f64 = (-s.db[63][7]);
        let eq42_e421: f64 = (eq42_e419 * s.v[3]);
        let eq42_e421_d_n0: f64 = (eq42_e419_d_n0 * s.v[3]);
        let eq42_e421_d_n1: f64 = (eq42_e419_d_n1 * s.v[3]);
        let eq42_e421_d_n2: f64 = (eq42_e419_d_n2 * s.v[3]);
        let eq42_e421_d_n3: f64 = (eq42_e419_d_n3 * s.v[3]);
        let eq42_e421_d_n4: f64 = (eq42_e419_d_n4 * s.v[3]);
        let eq42_e421_d_n5: f64 = (eq42_e419_d_n5 * s.v[3]);
        let eq42_e421_d_n6: f64 = (eq42_e419_d_n6 * s.v[3]);
        let eq42_e421_d_n7: f64 = (eq42_e419_d_n7 * s.v[3]);
        let eq42_e421_d_n8: f64 = (eq42_e419_d_n8 * s.v[3]);
        let eq42_e421_d_n9: f64 = (eq42_e419_d_n9 * s.v[3]);
        let eq42_e421_d_b0: f64 = (eq42_e419_d_b0 * s.v[3]);
        let eq42_e421_d_b1: f64 = (eq42_e419_d_b1 * s.v[3]);
        let eq42_e421_d_b2: f64 = (eq42_e419_d_b2 * s.v[3]);
        let eq42_e421_d_b3: f64 = (eq42_e419_d_b3 * s.v[3]);
        let eq42_e421_d_b4: f64 = (eq42_e419_d_b4 * s.v[3]);
        let eq42_e421_d_b5: f64 = (eq42_e419_d_b5 * s.v[3]);
        let eq42_e421_d_b6: f64 = (eq42_e419_d_b6 * s.v[3]);
        let eq42_e421_d_b7: f64 = (eq42_e419_d_b7 * s.v[3]);
        let eq42_e422: f64 = self.eval_ddt(11, eq42_e421);
        let eq42_e422_d_n0: f64 = self.ddt_jacobian(eq42_e421_d_n0);
        let eq42_e422_d_n1: f64 = self.ddt_jacobian(eq42_e421_d_n1);
        let eq42_e422_d_n2: f64 = self.ddt_jacobian(eq42_e421_d_n2);
        let eq42_e422_d_n3: f64 = self.ddt_jacobian(eq42_e421_d_n3);
        let eq42_e422_d_n4: f64 = self.ddt_jacobian(eq42_e421_d_n4);
        let eq42_e422_d_n5: f64 = self.ddt_jacobian(eq42_e421_d_n5);
        let eq42_e422_d_n6: f64 = self.ddt_jacobian(eq42_e421_d_n6);
        let eq42_e422_d_n7: f64 = self.ddt_jacobian(eq42_e421_d_n7);
        let eq42_e422_d_n8: f64 = self.ddt_jacobian(eq42_e421_d_n8);
        let eq42_e422_d_n9: f64 = self.ddt_jacobian(eq42_e421_d_n9);
        let eq42_e422_d_b0: f64 = self.ddt_jacobian(eq42_e421_d_b0);
        let eq42_e422_d_b1: f64 = self.ddt_jacobian(eq42_e421_d_b1);
        let eq42_e422_d_b2: f64 = self.ddt_jacobian(eq42_e421_d_b2);
        let eq42_e422_d_b3: f64 = self.ddt_jacobian(eq42_e421_d_b3);
        let eq42_e422_d_b4: f64 = self.ddt_jacobian(eq42_e421_d_b4);
        let eq42_e422_d_b5: f64 = self.ddt_jacobian(eq42_e421_d_b5);
        let eq42_e422_d_b6: f64 = self.ddt_jacobian(eq42_e421_d_b6);
        let eq42_e422_d_b7: f64 = self.ddt_jacobian(eq42_e421_d_b7);
        let eq42_value: f64 = eq42_e422;
        let eq42_node_derivatives: [f64; 10] = [eq42_e422_d_n0, eq42_e422_d_n1, eq42_e422_d_n2, eq42_e422_d_n3, eq42_e422_d_n4, eq42_e422_d_n5, eq42_e422_d_n6, eq42_e422_d_n7, eq42_e422_d_n8, eq42_e422_d_n9];
        let eq42_branch_derivatives: [f64; 8] = [eq42_e422_d_b0, eq42_e422_d_b1, eq42_e422_d_b2, eq42_e422_d_b3, eq42_e422_d_b4, eq42_e422_d_b5, eq42_e422_d_b6, eq42_e422_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let eq43_e425: f64 = (s.v[63] * s.v[3]);
        let eq43_e425_d_n0: f64 = (s.dn[63][0] * s.v[3]);
        let eq43_e425_d_n1: f64 = (s.dn[63][1] * s.v[3]);
        let eq43_e425_d_n2: f64 = (s.dn[63][2] * s.v[3]);
        let eq43_e425_d_n3: f64 = (s.dn[63][3] * s.v[3]);
        let eq43_e425_d_n4: f64 = (s.dn[63][4] * s.v[3]);
        let eq43_e425_d_n5: f64 = (s.dn[63][5] * s.v[3]);
        let eq43_e425_d_n6: f64 = (s.dn[63][6] * s.v[3]);
        let eq43_e425_d_n7: f64 = (s.dn[63][7] * s.v[3]);
        let eq43_e425_d_n8: f64 = (s.dn[63][8] * s.v[3]);
        let eq43_e425_d_n9: f64 = (s.dn[63][9] * s.v[3]);
        let eq43_e425_d_b0: f64 = (s.db[63][0] * s.v[3]);
        let eq43_e425_d_b1: f64 = (s.db[63][1] * s.v[3]);
        let eq43_e425_d_b2: f64 = (s.db[63][2] * s.v[3]);
        let eq43_e425_d_b3: f64 = (s.db[63][3] * s.v[3]);
        let eq43_e425_d_b4: f64 = (s.db[63][4] * s.v[3]);
        let eq43_e425_d_b5: f64 = (s.db[63][5] * s.v[3]);
        let eq43_e425_d_b6: f64 = (s.db[63][6] * s.v[3]);
        let eq43_e425_d_b7: f64 = (s.db[63][7] * s.v[3]);
        let eq43_e426: f64 = self.eval_ddt(12, eq43_e425);
        let eq43_e426_d_n0: f64 = self.ddt_jacobian(eq43_e425_d_n0);
        let eq43_e426_d_n1: f64 = self.ddt_jacobian(eq43_e425_d_n1);
        let eq43_e426_d_n2: f64 = self.ddt_jacobian(eq43_e425_d_n2);
        let eq43_e426_d_n3: f64 = self.ddt_jacobian(eq43_e425_d_n3);
        let eq43_e426_d_n4: f64 = self.ddt_jacobian(eq43_e425_d_n4);
        let eq43_e426_d_n5: f64 = self.ddt_jacobian(eq43_e425_d_n5);
        let eq43_e426_d_n6: f64 = self.ddt_jacobian(eq43_e425_d_n6);
        let eq43_e426_d_n7: f64 = self.ddt_jacobian(eq43_e425_d_n7);
        let eq43_e426_d_n8: f64 = self.ddt_jacobian(eq43_e425_d_n8);
        let eq43_e426_d_n9: f64 = self.ddt_jacobian(eq43_e425_d_n9);
        let eq43_e426_d_b0: f64 = self.ddt_jacobian(eq43_e425_d_b0);
        let eq43_e426_d_b1: f64 = self.ddt_jacobian(eq43_e425_d_b1);
        let eq43_e426_d_b2: f64 = self.ddt_jacobian(eq43_e425_d_b2);
        let eq43_e426_d_b3: f64 = self.ddt_jacobian(eq43_e425_d_b3);
        let eq43_e426_d_b4: f64 = self.ddt_jacobian(eq43_e425_d_b4);
        let eq43_e426_d_b5: f64 = self.ddt_jacobian(eq43_e425_d_b5);
        let eq43_e426_d_b6: f64 = self.ddt_jacobian(eq43_e425_d_b6);
        let eq43_e426_d_b7: f64 = self.ddt_jacobian(eq43_e425_d_b7);
        let eq43_value: f64 = eq43_e426;
        let eq43_node_derivatives: [f64; 10] = [eq43_e426_d_n0, eq43_e426_d_n1, eq43_e426_d_n2, eq43_e426_d_n3, eq43_e426_d_n4, eq43_e426_d_n5, eq43_e426_d_n6, eq43_e426_d_n7, eq43_e426_d_n8, eq43_e426_d_n9];
        let eq43_branch_derivatives: [f64; 8] = [eq43_e426_d_b0, eq43_e426_d_b1, eq43_e426_d_b2, eq43_e426_d_b3, eq43_e426_d_b4, eq43_e426_d_b5, eq43_e426_d_b6, eq43_e426_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            self.multiplicity * (eq43_value),
            &nodes,
            &eq43_node_derivatives,
            &branches,
            &eq43_branch_derivatives,
            self.multiplicity,
        );
    }
}
