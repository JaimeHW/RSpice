#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq31_e331, eq31_e331_d_n0, eq31_e331_d_n1, eq31_e331_d_n2, eq31_e331_d_n3, eq31_e331_d_n4, eq31_e331_d_n5, eq31_e331_d_n6, eq31_e331_d_n7, eq31_e331_d_n8, eq31_e331_d_n9, eq31_e331_d_n10, eq31_e331_d_n11, eq31_e331_d_n12, eq31_e331_d_n13, eq31_e331_d_n14, eq31_e331_d_b0, eq31_e331_d_b1, eq31_e331_d_b2, eq31_e331_d_b3, eq31_e331_d_b4, eq31_e331_d_b5,) = {
    if (!(s.v[514] != 0.0)) {
        let eq31_e329: f64 = (p.p148 * s.v[195]);
        let eq31_e329_d_n0: f64 = (p.p148 * s.dn[195][0]);
        let eq31_e329_d_n1: f64 = (p.p148 * s.dn[195][1]);
        let eq31_e329_d_n2: f64 = (p.p148 * s.dn[195][2]);
        let eq31_e329_d_n3: f64 = (p.p148 * s.dn[195][3]);
        let eq31_e329_d_n4: f64 = (p.p148 * s.dn[195][4]);
        let eq31_e329_d_n5: f64 = (p.p148 * s.dn[195][5]);
        let eq31_e329_d_n6: f64 = (p.p148 * s.dn[195][6]);
        let eq31_e329_d_n7: f64 = (p.p148 * s.dn[195][7]);
        let eq31_e329_d_n8: f64 = (p.p148 * s.dn[195][8]);
        let eq31_e329_d_n9: f64 = (p.p148 * s.dn[195][9]);
        let eq31_e329_d_n10: f64 = (p.p148 * s.dn[195][10]);
        let eq31_e329_d_n11: f64 = (p.p148 * s.dn[195][11]);
        let eq31_e329_d_n12: f64 = (p.p148 * s.dn[195][12]);
        let eq31_e329_d_n13: f64 = (p.p148 * s.dn[195][13]);
        let eq31_e329_d_n14: f64 = (p.p148 * s.dn[195][14]);
        let eq31_e329_d_b0: f64 = (p.p148 * s.db[195][0]);
        let eq31_e329_d_b1: f64 = (p.p148 * s.db[195][1]);
        let eq31_e329_d_b2: f64 = (p.p148 * s.db[195][2]);
        let eq31_e329_d_b3: f64 = (p.p148 * s.db[195][3]);
        let eq31_e329_d_b4: f64 = (p.p148 * s.db[195][4]);
        let eq31_e329_d_b5: f64 = (p.p148 * s.db[195][5]);
        (eq31_e329, eq31_e329_d_n0, eq31_e329_d_n1, eq31_e329_d_n2, eq31_e329_d_n3, eq31_e329_d_n4, eq31_e329_d_n5, eq31_e329_d_n6, eq31_e329_d_n7, eq31_e329_d_n8, eq31_e329_d_n9, eq31_e329_d_n10, eq31_e329_d_n11, eq31_e329_d_n12, eq31_e329_d_n13, eq31_e329_d_n14, eq31_e329_d_b0, eq31_e329_d_b1, eq31_e329_d_b2, eq31_e329_d_b3, eq31_e329_d_b4, eq31_e329_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e331;
        let eq31_node_derivatives: [f64; 15] = [eq31_e331_d_n0, eq31_e331_d_n1, eq31_e331_d_n2, eq31_e331_d_n3, eq31_e331_d_n4, eq31_e331_d_n5, eq31_e331_d_n6, eq31_e331_d_n7, eq31_e331_d_n8, eq31_e331_d_n9, eq31_e331_d_n10, eq31_e331_d_n11, eq31_e331_d_n12, eq31_e331_d_n13, eq31_e331_d_n14];
        let eq31_branch_derivatives: [f64; 6] = [eq31_e331_d_b0, eq31_e331_d_b1, eq31_e331_d_b2, eq31_e331_d_b3, eq31_e331_d_b4, eq31_e331_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[5]),
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq32_e340, eq32_e340_d_n5, eq32_e340_d_n9,) = {
    if ((!(s.v[514] != 0.0)) && (s.v[516] != 0.0)) {
        let eq32_e338: f64 = (s.v[233] * (nv9 - nv5));
        let eq32_e338_d_n5: f64 = (-s.v[233]);
        let eq32_e338_d_n9: f64 = s.v[233];
        (eq32_e338, eq32_e338_d_n5, eq32_e338_d_n9,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e340;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[5]),
            self.multiplicity * (eq32_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq32_e340_d_n5),
                GeneratedDerivative::node(nodes[9], self.multiplicity * eq32_e340_d_n9),
            ],
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
        let eq33_e343: f64 = (p.p148 * s.v[196]);
        let eq33_e343_d_n0: f64 = (p.p148 * s.dn[196][0]);
        let eq33_e343_d_n1: f64 = (p.p148 * s.dn[196][1]);
        let eq33_e343_d_n2: f64 = (p.p148 * s.dn[196][2]);
        let eq33_e343_d_n3: f64 = (p.p148 * s.dn[196][3]);
        let eq33_e343_d_n4: f64 = (p.p148 * s.dn[196][4]);
        let eq33_e343_d_n5: f64 = (p.p148 * s.dn[196][5]);
        let eq33_e343_d_n6: f64 = (p.p148 * s.dn[196][6]);
        let eq33_e343_d_n7: f64 = (p.p148 * s.dn[196][7]);
        let eq33_e343_d_n8: f64 = (p.p148 * s.dn[196][8]);
        let eq33_e343_d_n9: f64 = (p.p148 * s.dn[196][9]);
        let eq33_e343_d_n10: f64 = (p.p148 * s.dn[196][10]);
        let eq33_e343_d_n11: f64 = (p.p148 * s.dn[196][11]);
        let eq33_e343_d_n12: f64 = (p.p148 * s.dn[196][12]);
        let eq33_e343_d_n13: f64 = (p.p148 * s.dn[196][13]);
        let eq33_e343_d_n14: f64 = (p.p148 * s.dn[196][14]);
        let eq33_e343_d_b0: f64 = (p.p148 * s.db[196][0]);
        let eq33_e343_d_b1: f64 = (p.p148 * s.db[196][1]);
        let eq33_e343_d_b2: f64 = (p.p148 * s.db[196][2]);
        let eq33_e343_d_b3: f64 = (p.p148 * s.db[196][3]);
        let eq33_e343_d_b4: f64 = (p.p148 * s.db[196][4]);
        let eq33_e343_d_b5: f64 = (p.p148 * s.db[196][5]);
        let eq33_e344: f64 = self.eval_ddt(11, eq33_e343);
        let eq33_e344_d_n0: f64 = self.ddt_jacobian(eq33_e343_d_n0);
        let eq33_e344_d_n1: f64 = self.ddt_jacobian(eq33_e343_d_n1);
        let eq33_e344_d_n2: f64 = self.ddt_jacobian(eq33_e343_d_n2);
        let eq33_e344_d_n3: f64 = self.ddt_jacobian(eq33_e343_d_n3);
        let eq33_e344_d_n4: f64 = self.ddt_jacobian(eq33_e343_d_n4);
        let eq33_e344_d_n5: f64 = self.ddt_jacobian(eq33_e343_d_n5);
        let eq33_e344_d_n6: f64 = self.ddt_jacobian(eq33_e343_d_n6);
        let eq33_e344_d_n7: f64 = self.ddt_jacobian(eq33_e343_d_n7);
        let eq33_e344_d_n8: f64 = self.ddt_jacobian(eq33_e343_d_n8);
        let eq33_e344_d_n9: f64 = self.ddt_jacobian(eq33_e343_d_n9);
        let eq33_e344_d_n10: f64 = self.ddt_jacobian(eq33_e343_d_n10);
        let eq33_e344_d_n11: f64 = self.ddt_jacobian(eq33_e343_d_n11);
        let eq33_e344_d_n12: f64 = self.ddt_jacobian(eq33_e343_d_n12);
        let eq33_e344_d_n13: f64 = self.ddt_jacobian(eq33_e343_d_n13);
        let eq33_e344_d_n14: f64 = self.ddt_jacobian(eq33_e343_d_n14);
        let eq33_e344_d_b0: f64 = self.ddt_jacobian(eq33_e343_d_b0);
        let eq33_e344_d_b1: f64 = self.ddt_jacobian(eq33_e343_d_b1);
        let eq33_e344_d_b2: f64 = self.ddt_jacobian(eq33_e343_d_b2);
        let eq33_e344_d_b3: f64 = self.ddt_jacobian(eq33_e343_d_b3);
        let eq33_e344_d_b4: f64 = self.ddt_jacobian(eq33_e343_d_b4);
        let eq33_e344_d_b5: f64 = self.ddt_jacobian(eq33_e343_d_b5);
        let eq33_value: f64 = eq33_e344;
        let eq33_node_derivatives: [f64; 15] = [eq33_e344_d_n0, eq33_e344_d_n1, eq33_e344_d_n2, eq33_e344_d_n3, eq33_e344_d_n4, eq33_e344_d_n5, eq33_e344_d_n6, eq33_e344_d_n7, eq33_e344_d_n8, eq33_e344_d_n9, eq33_e344_d_n10, eq33_e344_d_n11, eq33_e344_d_n12, eq33_e344_d_n13, eq33_e344_d_n14];
        let eq33_branch_derivatives: [f64; 6] = [eq33_e344_d_b0, eq33_e344_d_b1, eq33_e344_d_b2, eq33_e344_d_b3, eq33_e344_d_b4, eq33_e344_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[5]),
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
        let eq34_e347: f64 = (p.p148 * s.v[197]);
        let eq34_e347_d_n0: f64 = (p.p148 * s.dn[197][0]);
        let eq34_e347_d_n1: f64 = (p.p148 * s.dn[197][1]);
        let eq34_e347_d_n2: f64 = (p.p148 * s.dn[197][2]);
        let eq34_e347_d_n3: f64 = (p.p148 * s.dn[197][3]);
        let eq34_e347_d_n4: f64 = (p.p148 * s.dn[197][4]);
        let eq34_e347_d_n5: f64 = (p.p148 * s.dn[197][5]);
        let eq34_e347_d_n6: f64 = (p.p148 * s.dn[197][6]);
        let eq34_e347_d_n7: f64 = (p.p148 * s.dn[197][7]);
        let eq34_e347_d_n8: f64 = (p.p148 * s.dn[197][8]);
        let eq34_e347_d_n9: f64 = (p.p148 * s.dn[197][9]);
        let eq34_e347_d_n10: f64 = (p.p148 * s.dn[197][10]);
        let eq34_e347_d_n11: f64 = (p.p148 * s.dn[197][11]);
        let eq34_e347_d_n12: f64 = (p.p148 * s.dn[197][12]);
        let eq34_e347_d_n13: f64 = (p.p148 * s.dn[197][13]);
        let eq34_e347_d_n14: f64 = (p.p148 * s.dn[197][14]);
        let eq34_e347_d_b0: f64 = (p.p148 * s.db[197][0]);
        let eq34_e347_d_b1: f64 = (p.p148 * s.db[197][1]);
        let eq34_e347_d_b2: f64 = (p.p148 * s.db[197][2]);
        let eq34_e347_d_b3: f64 = (p.p148 * s.db[197][3]);
        let eq34_e347_d_b4: f64 = (p.p148 * s.db[197][4]);
        let eq34_e347_d_b5: f64 = (p.p148 * s.db[197][5]);
        let eq34_e348: f64 = self.eval_ddt(12, eq34_e347);
        let eq34_e348_d_n0: f64 = self.ddt_jacobian(eq34_e347_d_n0);
        let eq34_e348_d_n1: f64 = self.ddt_jacobian(eq34_e347_d_n1);
        let eq34_e348_d_n2: f64 = self.ddt_jacobian(eq34_e347_d_n2);
        let eq34_e348_d_n3: f64 = self.ddt_jacobian(eq34_e347_d_n3);
        let eq34_e348_d_n4: f64 = self.ddt_jacobian(eq34_e347_d_n4);
        let eq34_e348_d_n5: f64 = self.ddt_jacobian(eq34_e347_d_n5);
        let eq34_e348_d_n6: f64 = self.ddt_jacobian(eq34_e347_d_n6);
        let eq34_e348_d_n7: f64 = self.ddt_jacobian(eq34_e347_d_n7);
        let eq34_e348_d_n8: f64 = self.ddt_jacobian(eq34_e347_d_n8);
        let eq34_e348_d_n9: f64 = self.ddt_jacobian(eq34_e347_d_n9);
        let eq34_e348_d_n10: f64 = self.ddt_jacobian(eq34_e347_d_n10);
        let eq34_e348_d_n11: f64 = self.ddt_jacobian(eq34_e347_d_n11);
        let eq34_e348_d_n12: f64 = self.ddt_jacobian(eq34_e347_d_n12);
        let eq34_e348_d_n13: f64 = self.ddt_jacobian(eq34_e347_d_n13);
        let eq34_e348_d_n14: f64 = self.ddt_jacobian(eq34_e347_d_n14);
        let eq34_e348_d_b0: f64 = self.ddt_jacobian(eq34_e347_d_b0);
        let eq34_e348_d_b1: f64 = self.ddt_jacobian(eq34_e347_d_b1);
        let eq34_e348_d_b2: f64 = self.ddt_jacobian(eq34_e347_d_b2);
        let eq34_e348_d_b3: f64 = self.ddt_jacobian(eq34_e347_d_b3);
        let eq34_e348_d_b4: f64 = self.ddt_jacobian(eq34_e347_d_b4);
        let eq34_e348_d_b5: f64 = self.ddt_jacobian(eq34_e347_d_b5);
        let eq34_value: f64 = eq34_e348;
        let eq34_node_derivatives: [f64; 15] = [eq34_e348_d_n0, eq34_e348_d_n1, eq34_e348_d_n2, eq34_e348_d_n3, eq34_e348_d_n4, eq34_e348_d_n5, eq34_e348_d_n6, eq34_e348_d_n7, eq34_e348_d_n8, eq34_e348_d_n9, eq34_e348_d_n10, eq34_e348_d_n11, eq34_e348_d_n12, eq34_e348_d_n13, eq34_e348_d_n14];
        let eq34_branch_derivatives: [f64; 6] = [eq34_e348_d_b0, eq34_e348_d_b1, eq34_e348_d_b2, eq34_e348_d_b3, eq34_e348_d_b4, eq34_e348_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[0]),
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq35_e354, eq35_e354_d_n3, eq35_e354_d_n9,) = {
    if (s.v[517] != 0.0) {
        let eq35_e352: f64 = ((nv9 - nv3) / p.p102);
        let eq35_e352_d_n3: f64 = (-1.0 / p.p102);
        let eq35_e352_d_n9: f64 = (1.0 / p.p102);
        (eq35_e352, eq35_e352_d_n3, eq35_e352_d_n9,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e354;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[3]),
            self.multiplicity * (eq35_value),
            &[
                GeneratedDerivative::node(nodes[3], self.multiplicity * eq35_e354_d_n3),
                GeneratedDerivative::node(nodes[9], self.multiplicity * eq35_e354_d_n9),
            ],
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq36_e363, eq36_e363_d_n0, eq36_e363_d_n1, eq36_e363_d_n2, eq36_e363_d_n3, eq36_e363_d_n4, eq36_e363_d_n5, eq36_e363_d_n6, eq36_e363_d_n7, eq36_e363_d_n8, eq36_e363_d_n9, eq36_e363_d_n10, eq36_e363_d_n11, eq36_e363_d_n12, eq36_e363_d_n13, eq36_e363_d_n14, eq36_e363_d_b0, eq36_e363_d_b1, eq36_e363_d_b2, eq36_e363_d_b3, eq36_e363_d_b4, eq36_e363_d_b5,) = {
    if ((s.v[517] != 0.0) && (s.v[518] != 0.0)) {
        let eq36_e360: f64 = (p.p103 * (nv9 - nv3));
        let eq36_e360_d_n3: f64 = (-p.p103);
        let eq36_e360_d_n9: f64 = p.p103;
        let eq36_e361: f64 = self.eval_ddt(13, eq36_e360);
        let eq36_e361_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n3: f64 = self.ddt_jacobian(eq36_e360_d_n3);
        let eq36_e361_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n9: f64 = self.ddt_jacobian(eq36_e360_d_n9);
        let eq36_e361_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq36_e361_d_b5: f64 = self.ddt_jacobian(0.0);
        (eq36_e361, eq36_e361_d_n0, eq36_e361_d_n1, eq36_e361_d_n2, eq36_e361_d_n3, eq36_e361_d_n4, eq36_e361_d_n5, eq36_e361_d_n6, eq36_e361_d_n7, eq36_e361_d_n8, eq36_e361_d_n9, eq36_e361_d_n10, eq36_e361_d_n11, eq36_e361_d_n12, eq36_e361_d_n13, eq36_e361_d_n14, eq36_e361_d_b0, eq36_e361_d_b1, eq36_e361_d_b2, eq36_e361_d_b3, eq36_e361_d_b4, eq36_e361_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e363;
        let eq36_node_derivatives: [f64; 15] = [eq36_e363_d_n0, eq36_e363_d_n1, eq36_e363_d_n2, eq36_e363_d_n3, eq36_e363_d_n4, eq36_e363_d_n5, eq36_e363_d_n6, eq36_e363_d_n7, eq36_e363_d_n8, eq36_e363_d_n9, eq36_e363_d_n10, eq36_e363_d_n11, eq36_e363_d_n12, eq36_e363_d_n13, eq36_e363_d_n14];
        let eq36_branch_derivatives: [f64; 6] = [eq36_e363_d_b0, eq36_e363_d_b1, eq36_e363_d_b2, eq36_e363_d_b3, eq36_e363_d_b4, eq36_e363_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[3]),
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
        let (eq37_e368,) = {
    if (!(s.v[517] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e368;
        stamper.stamp_potential(
            branches[4],
            eq37_value,
            &[
            ],
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq38_e376, eq38_e376_d_n0, eq38_e376_d_n1, eq38_e376_d_n2, eq38_e376_d_n3, eq38_e376_d_n4, eq38_e376_d_n5, eq38_e376_d_n6, eq38_e376_d_n7, eq38_e376_d_n8, eq38_e376_d_n9, eq38_e376_d_n10, eq38_e376_d_n11, eq38_e376_d_n12, eq38_e376_d_n13, eq38_e376_d_n14, eq38_e376_d_b0, eq38_e376_d_b1, eq38_e376_d_b2, eq38_e376_d_b3, eq38_e376_d_b4, eq38_e376_d_b5,) = {
    if (s.v[519] != 0.0) {
        let eq38_e372: f64 = ((nv4 - 0.0) / s.v[201]);
        let eq38_e372_d_n0: f64 = (-(((nv4 - 0.0) * s.dn[201][0]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n1: f64 = (-(((nv4 - 0.0) * s.dn[201][1]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n2: f64 = (-(((nv4 - 0.0) * s.dn[201][2]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n3: f64 = (-(((nv4 - 0.0) * s.dn[201][3]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n4: f64 = ((s.v[201] - ((nv4 - 0.0) * s.dn[201][4])) / (s.v[201] * s.v[201]));
        let eq38_e372_d_n5: f64 = (-(((nv4 - 0.0) * s.dn[201][5]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n6: f64 = (-(((nv4 - 0.0) * s.dn[201][6]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n7: f64 = (-(((nv4 - 0.0) * s.dn[201][7]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n8: f64 = (-(((nv4 - 0.0) * s.dn[201][8]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n9: f64 = (-(((nv4 - 0.0) * s.dn[201][9]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n10: f64 = (-(((nv4 - 0.0) * s.dn[201][10]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n11: f64 = (-(((nv4 - 0.0) * s.dn[201][11]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n12: f64 = (-(((nv4 - 0.0) * s.dn[201][12]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n13: f64 = (-(((nv4 - 0.0) * s.dn[201][13]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n14: f64 = (-(((nv4 - 0.0) * s.dn[201][14]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b0: f64 = (-(((nv4 - 0.0) * s.db[201][0]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b1: f64 = (-(((nv4 - 0.0) * s.db[201][1]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b2: f64 = (-(((nv4 - 0.0) * s.db[201][2]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b3: f64 = (-(((nv4 - 0.0) * s.db[201][3]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b4: f64 = (-(((nv4 - 0.0) * s.db[201][4]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b5: f64 = (-(((nv4 - 0.0) * s.db[201][5]) / (s.v[201] * s.v[201])));
        let eq38_e374: f64 = (eq38_e372 - s.v[200]);
        let eq38_e374_d_n0: f64 = (eq38_e372_d_n0 - s.dn[200][0]);
        let eq38_e374_d_n1: f64 = (eq38_e372_d_n1 - s.dn[200][1]);
        let eq38_e374_d_n2: f64 = (eq38_e372_d_n2 - s.dn[200][2]);
        let eq38_e374_d_n3: f64 = (eq38_e372_d_n3 - s.dn[200][3]);
        let eq38_e374_d_n4: f64 = (eq38_e372_d_n4 - s.dn[200][4]);
        let eq38_e374_d_n5: f64 = (eq38_e372_d_n5 - s.dn[200][5]);
        let eq38_e374_d_n6: f64 = (eq38_e372_d_n6 - s.dn[200][6]);
        let eq38_e374_d_n7: f64 = (eq38_e372_d_n7 - s.dn[200][7]);
        let eq38_e374_d_n8: f64 = (eq38_e372_d_n8 - s.dn[200][8]);
        let eq38_e374_d_n9: f64 = (eq38_e372_d_n9 - s.dn[200][9]);
        let eq38_e374_d_n10: f64 = (eq38_e372_d_n10 - s.dn[200][10]);
        let eq38_e374_d_n11: f64 = (eq38_e372_d_n11 - s.dn[200][11]);
        let eq38_e374_d_n12: f64 = (eq38_e372_d_n12 - s.dn[200][12]);
        let eq38_e374_d_n13: f64 = (eq38_e372_d_n13 - s.dn[200][13]);
        let eq38_e374_d_n14: f64 = (eq38_e372_d_n14 - s.dn[200][14]);
        let eq38_e374_d_b0: f64 = (eq38_e372_d_b0 - s.db[200][0]);
        let eq38_e374_d_b1: f64 = (eq38_e372_d_b1 - s.db[200][1]);
        let eq38_e374_d_b2: f64 = (eq38_e372_d_b2 - s.db[200][2]);
        let eq38_e374_d_b3: f64 = (eq38_e372_d_b3 - s.db[200][3]);
        let eq38_e374_d_b4: f64 = (eq38_e372_d_b4 - s.db[200][4]);
        let eq38_e374_d_b5: f64 = (eq38_e372_d_b5 - s.db[200][5]);
        (eq38_e374, eq38_e374_d_n0, eq38_e374_d_n1, eq38_e374_d_n2, eq38_e374_d_n3, eq38_e374_d_n4, eq38_e374_d_n5, eq38_e374_d_n6, eq38_e374_d_n7, eq38_e374_d_n8, eq38_e374_d_n9, eq38_e374_d_n10, eq38_e374_d_n11, eq38_e374_d_n12, eq38_e374_d_n13, eq38_e374_d_n14, eq38_e374_d_b0, eq38_e374_d_b1, eq38_e374_d_b2, eq38_e374_d_b3, eq38_e374_d_b4, eq38_e374_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e376;
        let eq38_node_derivatives: [f64; 15] = [eq38_e376_d_n0, eq38_e376_d_n1, eq38_e376_d_n2, eq38_e376_d_n3, eq38_e376_d_n4, eq38_e376_d_n5, eq38_e376_d_n6, eq38_e376_d_n7, eq38_e376_d_n8, eq38_e376_d_n9, eq38_e376_d_n10, eq38_e376_d_n11, eq38_e376_d_n12, eq38_e376_d_n13, eq38_e376_d_n14];
        let eq38_branch_derivatives: [f64; 6] = [eq38_e376_d_b0, eq38_e376_d_b1, eq38_e376_d_b2, eq38_e376_d_b3, eq38_e376_d_b4, eq38_e376_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq39_e385, eq39_e385_d_n0, eq39_e385_d_n1, eq39_e385_d_n2, eq39_e385_d_n3, eq39_e385_d_n4, eq39_e385_d_n5, eq39_e385_d_n6, eq39_e385_d_n7, eq39_e385_d_n8, eq39_e385_d_n9, eq39_e385_d_n10, eq39_e385_d_n11, eq39_e385_d_n12, eq39_e385_d_n13, eq39_e385_d_n14, eq39_e385_d_b0, eq39_e385_d_b1, eq39_e385_d_b2, eq39_e385_d_b3, eq39_e385_d_b4, eq39_e385_d_b5,) = {
    if ((s.v[519] != 0.0) && (s.v[520] != 0.0)) {
        let eq39_e382: f64 = (p.p145 * (nv4 - 0.0));
        let eq39_e382_d_n4: f64 = p.p145;
        let eq39_e383: f64 = self.eval_ddt(14, eq39_e382);
        let eq39_e383_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n4: f64 = self.ddt_jacobian(eq39_e382_d_n4);
        let eq39_e383_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq39_e383_d_b5: f64 = self.ddt_jacobian(0.0);
        (eq39_e383, eq39_e383_d_n0, eq39_e383_d_n1, eq39_e383_d_n2, eq39_e383_d_n3, eq39_e383_d_n4, eq39_e383_d_n5, eq39_e383_d_n6, eq39_e383_d_n7, eq39_e383_d_n8, eq39_e383_d_n9, eq39_e383_d_n10, eq39_e383_d_n11, eq39_e383_d_n12, eq39_e383_d_n13, eq39_e383_d_n14, eq39_e383_d_b0, eq39_e383_d_b1, eq39_e383_d_b2, eq39_e383_d_b3, eq39_e383_d_b4, eq39_e383_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e385;
        let eq39_node_derivatives: [f64; 15] = [eq39_e385_d_n0, eq39_e385_d_n1, eq39_e385_d_n2, eq39_e385_d_n3, eq39_e385_d_n4, eq39_e385_d_n5, eq39_e385_d_n6, eq39_e385_d_n7, eq39_e385_d_n8, eq39_e385_d_n9, eq39_e385_d_n10, eq39_e385_d_n11, eq39_e385_d_n12, eq39_e385_d_n13, eq39_e385_d_n14];
        let eq39_branch_derivatives: [f64; 6] = [eq39_e385_d_b0, eq39_e385_d_b1, eq39_e385_d_b2, eq39_e385_d_b3, eq39_e385_d_b4, eq39_e385_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let (eq40_e390,) = {
    if (!(s.v[519] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e390;
        stamper.stamp_potential(
            branches[5],
            eq40_value,
            &[
            ],
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
        let eq41_value: f64 = s.v[237];
        let eq41_node_derivatives: [f64; 15] = [s.dn[237][0], s.dn[237][1], s.dn[237][2], s.dn[237][3], s.dn[237][4], s.dn[237][5], s.dn[237][6], s.dn[237][7], s.dn[237][8], s.dn[237][9], s.dn[237][10], s.dn[237][11], s.dn[237][12], s.dn[237][13], s.dn[237][14]];
        let eq41_branch_derivatives: [f64; 6] = [s.db[237][0], s.db[237][1], s.db[237][2], s.db[237][3], s.db[237][4], s.db[237][5]];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            None,
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
        let eq42_e393: f64 = self.eval_ddt(15, s.v[239]);
        let eq42_e393_d_n0: f64 = self.ddt_jacobian(s.dn[239][0]);
        let eq42_e393_d_n1: f64 = self.ddt_jacobian(s.dn[239][1]);
        let eq42_e393_d_n2: f64 = self.ddt_jacobian(s.dn[239][2]);
        let eq42_e393_d_n3: f64 = self.ddt_jacobian(s.dn[239][3]);
        let eq42_e393_d_n4: f64 = self.ddt_jacobian(s.dn[239][4]);
        let eq42_e393_d_n5: f64 = self.ddt_jacobian(s.dn[239][5]);
        let eq42_e393_d_n6: f64 = self.ddt_jacobian(s.dn[239][6]);
        let eq42_e393_d_n7: f64 = self.ddt_jacobian(s.dn[239][7]);
        let eq42_e393_d_n8: f64 = self.ddt_jacobian(s.dn[239][8]);
        let eq42_e393_d_n9: f64 = self.ddt_jacobian(s.dn[239][9]);
        let eq42_e393_d_n10: f64 = self.ddt_jacobian(s.dn[239][10]);
        let eq42_e393_d_n11: f64 = self.ddt_jacobian(s.dn[239][11]);
        let eq42_e393_d_n12: f64 = self.ddt_jacobian(s.dn[239][12]);
        let eq42_e393_d_n13: f64 = self.ddt_jacobian(s.dn[239][13]);
        let eq42_e393_d_n14: f64 = self.ddt_jacobian(s.dn[239][14]);
        let eq42_e393_d_b0: f64 = self.ddt_jacobian(s.db[239][0]);
        let eq42_e393_d_b1: f64 = self.ddt_jacobian(s.db[239][1]);
        let eq42_e393_d_b2: f64 = self.ddt_jacobian(s.db[239][2]);
        let eq42_e393_d_b3: f64 = self.ddt_jacobian(s.db[239][3]);
        let eq42_e393_d_b4: f64 = self.ddt_jacobian(s.db[239][4]);
        let eq42_e393_d_b5: f64 = self.ddt_jacobian(s.db[239][5]);
        let eq42_value: f64 = eq42_e393;
        let eq42_node_derivatives: [f64; 15] = [eq42_e393_d_n0, eq42_e393_d_n1, eq42_e393_d_n2, eq42_e393_d_n3, eq42_e393_d_n4, eq42_e393_d_n5, eq42_e393_d_n6, eq42_e393_d_n7, eq42_e393_d_n8, eq42_e393_d_n9, eq42_e393_d_n10, eq42_e393_d_n11, eq42_e393_d_n12, eq42_e393_d_n13, eq42_e393_d_n14];
        let eq42_branch_derivatives: [f64; 6] = [eq42_e393_d_b0, eq42_e393_d_b1, eq42_e393_d_b2, eq42_e393_d_b3, eq42_e393_d_b4, eq42_e393_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            None,
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
        let eq43_value: f64 = s.v[238];
        let eq43_node_derivatives: [f64; 15] = [s.dn[238][0], s.dn[238][1], s.dn[238][2], s.dn[238][3], s.dn[238][4], s.dn[238][5], s.dn[238][6], s.dn[238][7], s.dn[238][8], s.dn[238][9], s.dn[238][10], s.dn[238][11], s.dn[238][12], s.dn[238][13], s.dn[238][14]];
        let eq43_branch_derivatives: [f64; 6] = [s.db[238][0], s.db[238][1], s.db[238][2], s.db[238][3], s.db[238][4], s.db[238][5]];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            None,
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
        let eq44_e396: f64 = self.eval_ddt(16, s.v[240]);
        let eq44_e396_d_n0: f64 = self.ddt_jacobian(s.dn[240][0]);
        let eq44_e396_d_n1: f64 = self.ddt_jacobian(s.dn[240][1]);
        let eq44_e396_d_n2: f64 = self.ddt_jacobian(s.dn[240][2]);
        let eq44_e396_d_n3: f64 = self.ddt_jacobian(s.dn[240][3]);
        let eq44_e396_d_n4: f64 = self.ddt_jacobian(s.dn[240][4]);
        let eq44_e396_d_n5: f64 = self.ddt_jacobian(s.dn[240][5]);
        let eq44_e396_d_n6: f64 = self.ddt_jacobian(s.dn[240][6]);
        let eq44_e396_d_n7: f64 = self.ddt_jacobian(s.dn[240][7]);
        let eq44_e396_d_n8: f64 = self.ddt_jacobian(s.dn[240][8]);
        let eq44_e396_d_n9: f64 = self.ddt_jacobian(s.dn[240][9]);
        let eq44_e396_d_n10: f64 = self.ddt_jacobian(s.dn[240][10]);
        let eq44_e396_d_n11: f64 = self.ddt_jacobian(s.dn[240][11]);
        let eq44_e396_d_n12: f64 = self.ddt_jacobian(s.dn[240][12]);
        let eq44_e396_d_n13: f64 = self.ddt_jacobian(s.dn[240][13]);
        let eq44_e396_d_n14: f64 = self.ddt_jacobian(s.dn[240][14]);
        let eq44_e396_d_b0: f64 = self.ddt_jacobian(s.db[240][0]);
        let eq44_e396_d_b1: f64 = self.ddt_jacobian(s.db[240][1]);
        let eq44_e396_d_b2: f64 = self.ddt_jacobian(s.db[240][2]);
        let eq44_e396_d_b3: f64 = self.ddt_jacobian(s.db[240][3]);
        let eq44_e396_d_b4: f64 = self.ddt_jacobian(s.db[240][4]);
        let eq44_e396_d_b5: f64 = self.ddt_jacobian(s.db[240][5]);
        let eq44_value: f64 = eq44_e396;
        let eq44_node_derivatives: [f64; 15] = [eq44_e396_d_n0, eq44_e396_d_n1, eq44_e396_d_n2, eq44_e396_d_n3, eq44_e396_d_n4, eq44_e396_d_n5, eq44_e396_d_n6, eq44_e396_d_n7, eq44_e396_d_n8, eq44_e396_d_n9, eq44_e396_d_n10, eq44_e396_d_n11, eq44_e396_d_n12, eq44_e396_d_n13, eq44_e396_d_n14];
        let eq44_branch_derivatives: [f64; 6] = [eq44_e396_d_b0, eq44_e396_d_b1, eq44_e396_d_b2, eq44_e396_d_b3, eq44_e396_d_b4, eq44_e396_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            None,
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
        let eq45_value: f64 = s.v[235];
        let eq45_node_derivatives: [f64; 15] = [s.dn[235][0], s.dn[235][1], s.dn[235][2], s.dn[235][3], s.dn[235][4], s.dn[235][5], s.dn[235][6], s.dn[235][7], s.dn[235][8], s.dn[235][9], s.dn[235][10], s.dn[235][11], s.dn[235][12], s.dn[235][13], s.dn[235][14]];
        let eq45_branch_derivatives: [f64; 6] = [s.db[235][0], s.db[235][1], s.db[235][2], s.db[235][3], s.db[235][4], s.db[235][5]];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            None,
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
        let eq46_e399: f64 = self.eval_ddt(17, s.v[236]);
        let eq46_e399_d_n0: f64 = self.ddt_jacobian(s.dn[236][0]);
        let eq46_e399_d_n1: f64 = self.ddt_jacobian(s.dn[236][1]);
        let eq46_e399_d_n2: f64 = self.ddt_jacobian(s.dn[236][2]);
        let eq46_e399_d_n3: f64 = self.ddt_jacobian(s.dn[236][3]);
        let eq46_e399_d_n4: f64 = self.ddt_jacobian(s.dn[236][4]);
        let eq46_e399_d_n5: f64 = self.ddt_jacobian(s.dn[236][5]);
        let eq46_e399_d_n6: f64 = self.ddt_jacobian(s.dn[236][6]);
        let eq46_e399_d_n7: f64 = self.ddt_jacobian(s.dn[236][7]);
        let eq46_e399_d_n8: f64 = self.ddt_jacobian(s.dn[236][8]);
        let eq46_e399_d_n9: f64 = self.ddt_jacobian(s.dn[236][9]);
        let eq46_e399_d_n10: f64 = self.ddt_jacobian(s.dn[236][10]);
        let eq46_e399_d_n11: f64 = self.ddt_jacobian(s.dn[236][11]);
        let eq46_e399_d_n12: f64 = self.ddt_jacobian(s.dn[236][12]);
        let eq46_e399_d_n13: f64 = self.ddt_jacobian(s.dn[236][13]);
        let eq46_e399_d_n14: f64 = self.ddt_jacobian(s.dn[236][14]);
        let eq46_e399_d_b0: f64 = self.ddt_jacobian(s.db[236][0]);
        let eq46_e399_d_b1: f64 = self.ddt_jacobian(s.db[236][1]);
        let eq46_e399_d_b2: f64 = self.ddt_jacobian(s.db[236][2]);
        let eq46_e399_d_b3: f64 = self.ddt_jacobian(s.db[236][3]);
        let eq46_e399_d_b4: f64 = self.ddt_jacobian(s.db[236][4]);
        let eq46_e399_d_b5: f64 = self.ddt_jacobian(s.db[236][5]);
        let eq46_value: f64 = eq46_e399;
        let eq46_node_derivatives: [f64; 15] = [eq46_e399_d_n0, eq46_e399_d_n1, eq46_e399_d_n2, eq46_e399_d_n3, eq46_e399_d_n4, eq46_e399_d_n5, eq46_e399_d_n6, eq46_e399_d_n7, eq46_e399_d_n8, eq46_e399_d_n9, eq46_e399_d_n10, eq46_e399_d_n11, eq46_e399_d_n12, eq46_e399_d_n13, eq46_e399_d_n14];
        let eq46_branch_derivatives: [f64; 6] = [eq46_e399_d_b0, eq46_e399_d_b1, eq46_e399_d_b2, eq46_e399_d_b3, eq46_e399_d_b4, eq46_e399_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq46_value),
            &nodes,
            &eq46_node_derivatives,
            &branches,
            &eq46_branch_derivatives,
            self.multiplicity,
        );
    }
}
