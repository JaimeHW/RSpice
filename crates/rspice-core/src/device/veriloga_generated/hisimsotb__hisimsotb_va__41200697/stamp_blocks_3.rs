#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq6_e367, eq6_e367_d_n0, eq6_e367_d_n1, eq6_e367_d_n2, eq6_e367_d_n3, eq6_e367_d_n4, eq6_e367_d_n5, eq6_e367_d_n6, eq6_e367_d_n7, eq6_e367_d_n8, eq6_e367_d_n9, eq6_e367_d_n10, eq6_e367_d_n11, eq6_e367_d_n12, eq6_e367_d_b0, eq6_e367_d_b1, eq6_e367_d_b2, eq6_e367_d_b3,) = {
    if (p.p312 != 0.0) {
        let eq6_e365: f64 = ((nv12 - nv2) / s.v[27]);
        let eq6_e365_d_n0: f64 = (-(((nv12 - nv2) * s.dn[27][0]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n1: f64 = (-(((nv12 - nv2) * s.dn[27][1]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n2: f64 = (((-s.v[27]) - ((nv12 - nv2) * s.dn[27][2])) / (s.v[27] * s.v[27]));
        let eq6_e365_d_n3: f64 = (-(((nv12 - nv2) * s.dn[27][3]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n4: f64 = (-(((nv12 - nv2) * s.dn[27][4]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n5: f64 = (-(((nv12 - nv2) * s.dn[27][5]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n6: f64 = (-(((nv12 - nv2) * s.dn[27][6]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n7: f64 = (-(((nv12 - nv2) * s.dn[27][7]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n8: f64 = (-(((nv12 - nv2) * s.dn[27][8]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n9: f64 = (-(((nv12 - nv2) * s.dn[27][9]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n10: f64 = (-(((nv12 - nv2) * s.dn[27][10]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n11: f64 = (-(((nv12 - nv2) * s.dn[27][11]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_n12: f64 = ((s.v[27] - ((nv12 - nv2) * s.dn[27][12])) / (s.v[27] * s.v[27]));
        let eq6_e365_d_b0: f64 = (-(((nv12 - nv2) * s.db[27][0]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_b1: f64 = (-(((nv12 - nv2) * s.db[27][1]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_b2: f64 = (-(((nv12 - nv2) * s.db[27][2]) / (s.v[27] * s.v[27])));
        let eq6_e365_d_b3: f64 = (-(((nv12 - nv2) * s.db[27][3]) / (s.v[27] * s.v[27])));
        (eq6_e365, eq6_e365_d_n0, eq6_e365_d_n1, eq6_e365_d_n2, eq6_e365_d_n3, eq6_e365_d_n4, eq6_e365_d_n5, eq6_e365_d_n6, eq6_e365_d_n7, eq6_e365_d_n8, eq6_e365_d_n9, eq6_e365_d_n10, eq6_e365_d_n11, eq6_e365_d_n12, eq6_e365_d_b0, eq6_e365_d_b1, eq6_e365_d_b2, eq6_e365_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e367;
        let eq6_node_derivatives: [f64; 13] = [eq6_e367_d_n0, eq6_e367_d_n1, eq6_e367_d_n2, eq6_e367_d_n3, eq6_e367_d_n4, eq6_e367_d_n5, eq6_e367_d_n6, eq6_e367_d_n7, eq6_e367_d_n8, eq6_e367_d_n9, eq6_e367_d_n10, eq6_e367_d_n11, eq6_e367_d_n12];
        let eq6_branch_derivatives: [f64; 4] = [eq6_e367_d_b0, eq6_e367_d_b1, eq6_e367_d_b2, eq6_e367_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[2]),
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq7_e372,) = {
    if (!(p.p312 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e372;
        stamper.stamp_potential(
            branches[0],
            eq7_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_8_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq8_e378, eq8_e378_d_n0, eq8_e378_d_n1, eq8_e378_d_n2, eq8_e378_d_n3, eq8_e378_d_n4, eq8_e378_d_n5, eq8_e378_d_n6, eq8_e378_d_n7, eq8_e378_d_n8, eq8_e378_d_n9, eq8_e378_d_n10, eq8_e378_d_n11, eq8_e378_d_n12, eq8_e378_d_b0, eq8_e378_d_b1, eq8_e378_d_b2, eq8_e378_d_b3,) = {
    if (p.p313 != 0.0) {
        let eq8_e376: f64 = ((nv0 - nv11) / s.v[26]);
        let eq8_e376_d_n0: f64 = ((s.v[26] - ((nv0 - nv11) * s.dn[26][0])) / (s.v[26] * s.v[26]));
        let eq8_e376_d_n1: f64 = (-(((nv0 - nv11) * s.dn[26][1]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n2: f64 = (-(((nv0 - nv11) * s.dn[26][2]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n3: f64 = (-(((nv0 - nv11) * s.dn[26][3]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n4: f64 = (-(((nv0 - nv11) * s.dn[26][4]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n5: f64 = (-(((nv0 - nv11) * s.dn[26][5]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n6: f64 = (-(((nv0 - nv11) * s.dn[26][6]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n7: f64 = (-(((nv0 - nv11) * s.dn[26][7]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n8: f64 = (-(((nv0 - nv11) * s.dn[26][8]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n9: f64 = (-(((nv0 - nv11) * s.dn[26][9]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n10: f64 = (-(((nv0 - nv11) * s.dn[26][10]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_n11: f64 = (((-s.v[26]) - ((nv0 - nv11) * s.dn[26][11])) / (s.v[26] * s.v[26]));
        let eq8_e376_d_n12: f64 = (-(((nv0 - nv11) * s.dn[26][12]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b0: f64 = (-(((nv0 - nv11) * s.db[26][0]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b1: f64 = (-(((nv0 - nv11) * s.db[26][1]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b2: f64 = (-(((nv0 - nv11) * s.db[26][2]) / (s.v[26] * s.v[26])));
        let eq8_e376_d_b3: f64 = (-(((nv0 - nv11) * s.db[26][3]) / (s.v[26] * s.v[26])));
        (eq8_e376, eq8_e376_d_n0, eq8_e376_d_n1, eq8_e376_d_n2, eq8_e376_d_n3, eq8_e376_d_n4, eq8_e376_d_n5, eq8_e376_d_n6, eq8_e376_d_n7, eq8_e376_d_n8, eq8_e376_d_n9, eq8_e376_d_n10, eq8_e376_d_n11, eq8_e376_d_n12, eq8_e376_d_b0, eq8_e376_d_b1, eq8_e376_d_b2, eq8_e376_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e378;
        let eq8_node_derivatives: [f64; 13] = [eq8_e378_d_n0, eq8_e378_d_n1, eq8_e378_d_n2, eq8_e378_d_n3, eq8_e378_d_n4, eq8_e378_d_n5, eq8_e378_d_n6, eq8_e378_d_n7, eq8_e378_d_n8, eq8_e378_d_n9, eq8_e378_d_n10, eq8_e378_d_n11, eq8_e378_d_n12];
        let eq8_branch_derivatives: [f64; 4] = [eq8_e378_d_b0, eq8_e378_d_b1, eq8_e378_d_b2, eq8_e378_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[11]),
            self.multiplicity * (eq8_value),
            &nodes,
            &eq8_node_derivatives,
            &branches,
            &eq8_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq9_e383,) = {
    if (!(p.p313 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e383;
        stamper.stamp_potential(
            branches[1],
            eq9_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq10_e387: f64 = (s.v[561] + s.v[554]);
        let eq10_e387_d_n0: f64 = (s.dn[561][0] + s.dn[554][0]);
        let eq10_e387_d_n1: f64 = (s.dn[561][1] + s.dn[554][1]);
        let eq10_e387_d_n2: f64 = (s.dn[561][2] + s.dn[554][2]);
        let eq10_e387_d_n3: f64 = (s.dn[561][3] + s.dn[554][3]);
        let eq10_e387_d_n4: f64 = (s.dn[561][4] + s.dn[554][4]);
        let eq10_e387_d_n5: f64 = (s.dn[561][5] + s.dn[554][5]);
        let eq10_e387_d_n6: f64 = (s.dn[561][6] + s.dn[554][6]);
        let eq10_e387_d_n7: f64 = (s.dn[561][7] + s.dn[554][7]);
        let eq10_e387_d_n8: f64 = (s.dn[561][8] + s.dn[554][8]);
        let eq10_e387_d_n9: f64 = (s.dn[561][9] + s.dn[554][9]);
        let eq10_e387_d_n10: f64 = (s.dn[561][10] + s.dn[554][10]);
        let eq10_e387_d_n11: f64 = (s.dn[561][11] + s.dn[554][11]);
        let eq10_e387_d_n12: f64 = (s.dn[561][12] + s.dn[554][12]);
        let eq10_e387_d_b0: f64 = (s.db[561][0] + s.db[554][0]);
        let eq10_e387_d_b1: f64 = (s.db[561][1] + s.db[554][1]);
        let eq10_e387_d_b2: f64 = (s.db[561][2] + s.db[554][2]);
        let eq10_e387_d_b3: f64 = (s.db[561][3] + s.db[554][3]);
        let eq10_e388: f64 = self.eval_ddt(0, eq10_e387);
        let eq10_e388_d_n0: f64 = self.ddt_jacobian(eq10_e387_d_n0);
        let eq10_e388_d_n1: f64 = self.ddt_jacobian(eq10_e387_d_n1);
        let eq10_e388_d_n2: f64 = self.ddt_jacobian(eq10_e387_d_n2);
        let eq10_e388_d_n3: f64 = self.ddt_jacobian(eq10_e387_d_n3);
        let eq10_e388_d_n4: f64 = self.ddt_jacobian(eq10_e387_d_n4);
        let eq10_e388_d_n5: f64 = self.ddt_jacobian(eq10_e387_d_n5);
        let eq10_e388_d_n6: f64 = self.ddt_jacobian(eq10_e387_d_n6);
        let eq10_e388_d_n7: f64 = self.ddt_jacobian(eq10_e387_d_n7);
        let eq10_e388_d_n8: f64 = self.ddt_jacobian(eq10_e387_d_n8);
        let eq10_e388_d_n9: f64 = self.ddt_jacobian(eq10_e387_d_n9);
        let eq10_e388_d_n10: f64 = self.ddt_jacobian(eq10_e387_d_n10);
        let eq10_e388_d_n11: f64 = self.ddt_jacobian(eq10_e387_d_n11);
        let eq10_e388_d_n12: f64 = self.ddt_jacobian(eq10_e387_d_n12);
        let eq10_e388_d_b0: f64 = self.ddt_jacobian(eq10_e387_d_b0);
        let eq10_e388_d_b1: f64 = self.ddt_jacobian(eq10_e387_d_b1);
        let eq10_e388_d_b2: f64 = self.ddt_jacobian(eq10_e387_d_b2);
        let eq10_e388_d_b3: f64 = self.ddt_jacobian(eq10_e387_d_b3);
        let eq10_e389: f64 = (p.p33 * eq10_e388);
        let eq10_e389_d_n0: f64 = (p.p33 * eq10_e388_d_n0);
        let eq10_e389_d_n1: f64 = (p.p33 * eq10_e388_d_n1);
        let eq10_e389_d_n2: f64 = (p.p33 * eq10_e388_d_n2);
        let eq10_e389_d_n3: f64 = (p.p33 * eq10_e388_d_n3);
        let eq10_e389_d_n4: f64 = (p.p33 * eq10_e388_d_n4);
        let eq10_e389_d_n5: f64 = (p.p33 * eq10_e388_d_n5);
        let eq10_e389_d_n6: f64 = (p.p33 * eq10_e388_d_n6);
        let eq10_e389_d_n7: f64 = (p.p33 * eq10_e388_d_n7);
        let eq10_e389_d_n8: f64 = (p.p33 * eq10_e388_d_n8);
        let eq10_e389_d_n9: f64 = (p.p33 * eq10_e388_d_n9);
        let eq10_e389_d_n10: f64 = (p.p33 * eq10_e388_d_n10);
        let eq10_e389_d_n11: f64 = (p.p33 * eq10_e388_d_n11);
        let eq10_e389_d_n12: f64 = (p.p33 * eq10_e388_d_n12);
        let eq10_e389_d_b0: f64 = (p.p33 * eq10_e388_d_b0);
        let eq10_e389_d_b1: f64 = (p.p33 * eq10_e388_d_b1);
        let eq10_e389_d_b2: f64 = (p.p33 * eq10_e388_d_b2);
        let eq10_e389_d_b3: f64 = (p.p33 * eq10_e388_d_b3);
        let eq10_value: f64 = eq10_e389;
        let eq10_node_derivatives: [f64; 13] = [eq10_e389_d_n0, eq10_e389_d_n1, eq10_e389_d_n2, eq10_e389_d_n3, eq10_e389_d_n4, eq10_e389_d_n5, eq10_e389_d_n6, eq10_e389_d_n7, eq10_e389_d_n8, eq10_e389_d_n9, eq10_e389_d_n10, eq10_e389_d_n11, eq10_e389_d_n12];
        let eq10_branch_derivatives: [f64; 4] = [eq10_e389_d_b0, eq10_e389_d_b1, eq10_e389_d_b2, eq10_e389_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            self.multiplicity * (eq10_value),
            &nodes,
            &eq10_node_derivatives,
            &branches,
            &eq10_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq11_e393: f64 = (s.v[93] + s.v[552]);
        let eq11_e393_d_n0: f64 = (s.dn[93][0] + s.dn[552][0]);
        let eq11_e393_d_n1: f64 = (s.dn[93][1] + s.dn[552][1]);
        let eq11_e393_d_n2: f64 = (s.dn[93][2] + s.dn[552][2]);
        let eq11_e393_d_n3: f64 = (s.dn[93][3] + s.dn[552][3]);
        let eq11_e393_d_n4: f64 = (s.dn[93][4] + s.dn[552][4]);
        let eq11_e393_d_n5: f64 = (s.dn[93][5] + s.dn[552][5]);
        let eq11_e393_d_n6: f64 = (s.dn[93][6] + s.dn[552][6]);
        let eq11_e393_d_n7: f64 = (s.dn[93][7] + s.dn[552][7]);
        let eq11_e393_d_n8: f64 = (s.dn[93][8] + s.dn[552][8]);
        let eq11_e393_d_n9: f64 = (s.dn[93][9] + s.dn[552][9]);
        let eq11_e393_d_n10: f64 = (s.dn[93][10] + s.dn[552][10]);
        let eq11_e393_d_n11: f64 = (s.dn[93][11] + s.dn[552][11]);
        let eq11_e393_d_n12: f64 = (s.dn[93][12] + s.dn[552][12]);
        let eq11_e393_d_b0: f64 = (s.db[93][0] + s.db[552][0]);
        let eq11_e393_d_b1: f64 = (s.db[93][1] + s.db[552][1]);
        let eq11_e393_d_b2: f64 = (s.db[93][2] + s.db[552][2]);
        let eq11_e393_d_b3: f64 = (s.db[93][3] + s.db[552][3]);
        let eq11_e394: f64 = self.eval_ddt(1, eq11_e393);
        let eq11_e394_d_n0: f64 = self.ddt_jacobian(eq11_e393_d_n0);
        let eq11_e394_d_n1: f64 = self.ddt_jacobian(eq11_e393_d_n1);
        let eq11_e394_d_n2: f64 = self.ddt_jacobian(eq11_e393_d_n2);
        let eq11_e394_d_n3: f64 = self.ddt_jacobian(eq11_e393_d_n3);
        let eq11_e394_d_n4: f64 = self.ddt_jacobian(eq11_e393_d_n4);
        let eq11_e394_d_n5: f64 = self.ddt_jacobian(eq11_e393_d_n5);
        let eq11_e394_d_n6: f64 = self.ddt_jacobian(eq11_e393_d_n6);
        let eq11_e394_d_n7: f64 = self.ddt_jacobian(eq11_e393_d_n7);
        let eq11_e394_d_n8: f64 = self.ddt_jacobian(eq11_e393_d_n8);
        let eq11_e394_d_n9: f64 = self.ddt_jacobian(eq11_e393_d_n9);
        let eq11_e394_d_n10: f64 = self.ddt_jacobian(eq11_e393_d_n10);
        let eq11_e394_d_n11: f64 = self.ddt_jacobian(eq11_e393_d_n11);
        let eq11_e394_d_n12: f64 = self.ddt_jacobian(eq11_e393_d_n12);
        let eq11_e394_d_b0: f64 = self.ddt_jacobian(eq11_e393_d_b0);
        let eq11_e394_d_b1: f64 = self.ddt_jacobian(eq11_e393_d_b1);
        let eq11_e394_d_b2: f64 = self.ddt_jacobian(eq11_e393_d_b2);
        let eq11_e394_d_b3: f64 = self.ddt_jacobian(eq11_e393_d_b3);
        let eq11_e395: f64 = (p.p33 * eq11_e394);
        let eq11_e395_d_n0: f64 = (p.p33 * eq11_e394_d_n0);
        let eq11_e395_d_n1: f64 = (p.p33 * eq11_e394_d_n1);
        let eq11_e395_d_n2: f64 = (p.p33 * eq11_e394_d_n2);
        let eq11_e395_d_n3: f64 = (p.p33 * eq11_e394_d_n3);
        let eq11_e395_d_n4: f64 = (p.p33 * eq11_e394_d_n4);
        let eq11_e395_d_n5: f64 = (p.p33 * eq11_e394_d_n5);
        let eq11_e395_d_n6: f64 = (p.p33 * eq11_e394_d_n6);
        let eq11_e395_d_n7: f64 = (p.p33 * eq11_e394_d_n7);
        let eq11_e395_d_n8: f64 = (p.p33 * eq11_e394_d_n8);
        let eq11_e395_d_n9: f64 = (p.p33 * eq11_e394_d_n9);
        let eq11_e395_d_n10: f64 = (p.p33 * eq11_e394_d_n10);
        let eq11_e395_d_n11: f64 = (p.p33 * eq11_e394_d_n11);
        let eq11_e395_d_n12: f64 = (p.p33 * eq11_e394_d_n12);
        let eq11_e395_d_b0: f64 = (p.p33 * eq11_e394_d_b0);
        let eq11_e395_d_b1: f64 = (p.p33 * eq11_e394_d_b1);
        let eq11_e395_d_b2: f64 = (p.p33 * eq11_e394_d_b2);
        let eq11_e395_d_b3: f64 = (p.p33 * eq11_e394_d_b3);
        let eq11_value: f64 = eq11_e395;
        let eq11_node_derivatives: [f64; 13] = [eq11_e395_d_n0, eq11_e395_d_n1, eq11_e395_d_n2, eq11_e395_d_n3, eq11_e395_d_n4, eq11_e395_d_n5, eq11_e395_d_n6, eq11_e395_d_n7, eq11_e395_d_n8, eq11_e395_d_n9, eq11_e395_d_n10, eq11_e395_d_n11, eq11_e395_d_n12];
        let eq11_branch_derivatives: [f64; 4] = [eq11_e395_d_b0, eq11_e395_d_b1, eq11_e395_d_b2, eq11_e395_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            self.multiplicity * (eq11_value),
            &nodes,
            &eq11_node_derivatives,
            &branches,
            &eq11_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq12_e399: f64 = (s.v[90] + s.v[548]);
        let eq12_e399_d_n0: f64 = (s.dn[90][0] + s.dn[548][0]);
        let eq12_e399_d_n1: f64 = (s.dn[90][1] + s.dn[548][1]);
        let eq12_e399_d_n2: f64 = (s.dn[90][2] + s.dn[548][2]);
        let eq12_e399_d_n3: f64 = (s.dn[90][3] + s.dn[548][3]);
        let eq12_e399_d_n4: f64 = (s.dn[90][4] + s.dn[548][4]);
        let eq12_e399_d_n5: f64 = (s.dn[90][5] + s.dn[548][5]);
        let eq12_e399_d_n6: f64 = (s.dn[90][6] + s.dn[548][6]);
        let eq12_e399_d_n7: f64 = (s.dn[90][7] + s.dn[548][7]);
        let eq12_e399_d_n8: f64 = (s.dn[90][8] + s.dn[548][8]);
        let eq12_e399_d_n9: f64 = (s.dn[90][9] + s.dn[548][9]);
        let eq12_e399_d_n10: f64 = (s.dn[90][10] + s.dn[548][10]);
        let eq12_e399_d_n11: f64 = (s.dn[90][11] + s.dn[548][11]);
        let eq12_e399_d_n12: f64 = (s.dn[90][12] + s.dn[548][12]);
        let eq12_e399_d_b0: f64 = (s.db[90][0] + s.db[548][0]);
        let eq12_e399_d_b1: f64 = (s.db[90][1] + s.db[548][1]);
        let eq12_e399_d_b2: f64 = (s.db[90][2] + s.db[548][2]);
        let eq12_e399_d_b3: f64 = (s.db[90][3] + s.db[548][3]);
        let eq12_e400: f64 = self.eval_ddt(2, eq12_e399);
        let eq12_e400_d_n0: f64 = self.ddt_jacobian(eq12_e399_d_n0);
        let eq12_e400_d_n1: f64 = self.ddt_jacobian(eq12_e399_d_n1);
        let eq12_e400_d_n2: f64 = self.ddt_jacobian(eq12_e399_d_n2);
        let eq12_e400_d_n3: f64 = self.ddt_jacobian(eq12_e399_d_n3);
        let eq12_e400_d_n4: f64 = self.ddt_jacobian(eq12_e399_d_n4);
        let eq12_e400_d_n5: f64 = self.ddt_jacobian(eq12_e399_d_n5);
        let eq12_e400_d_n6: f64 = self.ddt_jacobian(eq12_e399_d_n6);
        let eq12_e400_d_n7: f64 = self.ddt_jacobian(eq12_e399_d_n7);
        let eq12_e400_d_n8: f64 = self.ddt_jacobian(eq12_e399_d_n8);
        let eq12_e400_d_n9: f64 = self.ddt_jacobian(eq12_e399_d_n9);
        let eq12_e400_d_n10: f64 = self.ddt_jacobian(eq12_e399_d_n10);
        let eq12_e400_d_n11: f64 = self.ddt_jacobian(eq12_e399_d_n11);
        let eq12_e400_d_n12: f64 = self.ddt_jacobian(eq12_e399_d_n12);
        let eq12_e400_d_b0: f64 = self.ddt_jacobian(eq12_e399_d_b0);
        let eq12_e400_d_b1: f64 = self.ddt_jacobian(eq12_e399_d_b1);
        let eq12_e400_d_b2: f64 = self.ddt_jacobian(eq12_e399_d_b2);
        let eq12_e400_d_b3: f64 = self.ddt_jacobian(eq12_e399_d_b3);
        let eq12_e401: f64 = (p.p33 * eq12_e400);
        let eq12_e401_d_n0: f64 = (p.p33 * eq12_e400_d_n0);
        let eq12_e401_d_n1: f64 = (p.p33 * eq12_e400_d_n1);
        let eq12_e401_d_n2: f64 = (p.p33 * eq12_e400_d_n2);
        let eq12_e401_d_n3: f64 = (p.p33 * eq12_e400_d_n3);
        let eq12_e401_d_n4: f64 = (p.p33 * eq12_e400_d_n4);
        let eq12_e401_d_n5: f64 = (p.p33 * eq12_e400_d_n5);
        let eq12_e401_d_n6: f64 = (p.p33 * eq12_e400_d_n6);
        let eq12_e401_d_n7: f64 = (p.p33 * eq12_e400_d_n7);
        let eq12_e401_d_n8: f64 = (p.p33 * eq12_e400_d_n8);
        let eq12_e401_d_n9: f64 = (p.p33 * eq12_e400_d_n9);
        let eq12_e401_d_n10: f64 = (p.p33 * eq12_e400_d_n10);
        let eq12_e401_d_n11: f64 = (p.p33 * eq12_e400_d_n11);
        let eq12_e401_d_n12: f64 = (p.p33 * eq12_e400_d_n12);
        let eq12_e401_d_b0: f64 = (p.p33 * eq12_e400_d_b0);
        let eq12_e401_d_b1: f64 = (p.p33 * eq12_e400_d_b1);
        let eq12_e401_d_b2: f64 = (p.p33 * eq12_e400_d_b2);
        let eq12_e401_d_b3: f64 = (p.p33 * eq12_e400_d_b3);
        let eq12_value: f64 = eq12_e401;
        let eq12_node_derivatives: [f64; 13] = [eq12_e401_d_n0, eq12_e401_d_n1, eq12_e401_d_n2, eq12_e401_d_n3, eq12_e401_d_n4, eq12_e401_d_n5, eq12_e401_d_n6, eq12_e401_d_n7, eq12_e401_d_n8, eq12_e401_d_n9, eq12_e401_d_n10, eq12_e401_d_n11, eq12_e401_d_n12];
        let eq12_branch_derivatives: [f64; 4] = [eq12_e401_d_b0, eq12_e401_d_b1, eq12_e401_d_b2, eq12_e401_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[12]),
            self.multiplicity * (eq12_value),
            &nodes,
            &eq12_node_derivatives,
            &branches,
            &eq12_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq13_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[12]),
            self.multiplicity * (eq13_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq14_e412: f64 = (nv7 - 0.0);
        let eq14_value: f64 = eq14_e412;
        stamper.stamp_current(
            Some(nodes[7]),
            None,
            self.multiplicity * (eq14_value),
            &[
                GeneratedDerivative::node(nodes[7], self.multiplicity * 1.0),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq15_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            None,
            self.multiplicity * (eq15_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_16_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq16_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[12]),
            self.multiplicity * (eq16_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq17_e427: f64 = (s.v[609] * (nv7 - 0.0));
        let eq17_e427_d_n0: f64 = (s.dn[609][0] * (nv7 - 0.0));
        let eq17_e427_d_n1: f64 = (s.dn[609][1] * (nv7 - 0.0));
        let eq17_e427_d_n2: f64 = (s.dn[609][2] * (nv7 - 0.0));
        let eq17_e427_d_n3: f64 = (s.dn[609][3] * (nv7 - 0.0));
        let eq17_e427_d_n4: f64 = (s.dn[609][4] * (nv7 - 0.0));
        let eq17_e427_d_n5: f64 = (s.dn[609][5] * (nv7 - 0.0));
        let eq17_e427_d_n6: f64 = (s.dn[609][6] * (nv7 - 0.0));
        let eq17_e427_d_n7: f64 = ((s.dn[609][7] * (nv7 - 0.0)) + s.v[609]);
        let eq17_e427_d_n8: f64 = (s.dn[609][8] * (nv7 - 0.0));
        let eq17_e427_d_n9: f64 = (s.dn[609][9] * (nv7 - 0.0));
        let eq17_e427_d_n10: f64 = (s.dn[609][10] * (nv7 - 0.0));
        let eq17_e427_d_n11: f64 = (s.dn[609][11] * (nv7 - 0.0));
        let eq17_e427_d_n12: f64 = (s.dn[609][12] * (nv7 - 0.0));
        let eq17_e427_d_b0: f64 = (s.db[609][0] * (nv7 - 0.0));
        let eq17_e427_d_b1: f64 = (s.db[609][1] * (nv7 - 0.0));
        let eq17_e427_d_b2: f64 = (s.db[609][2] * (nv7 - 0.0));
        let eq17_e427_d_b3: f64 = (s.db[609][3] * (nv7 - 0.0));
        let eq17_value: f64 = eq17_e427;
        let eq17_node_derivatives: [f64; 13] = [eq17_e427_d_n0, eq17_e427_d_n1, eq17_e427_d_n2, eq17_e427_d_n3, eq17_e427_d_n4, eq17_e427_d_n5, eq17_e427_d_n6, eq17_e427_d_n7, eq17_e427_d_n8, eq17_e427_d_n9, eq17_e427_d_n10, eq17_e427_d_n11, eq17_e427_d_n12];
        let eq17_branch_derivatives: [f64; 4] = [eq17_e427_d_b0, eq17_e427_d_b1, eq17_e427_d_b2, eq17_e427_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            self.multiplicity * (eq17_value),
            &nodes,
            &eq17_node_derivatives,
            &branches,
            &eq17_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq18_e430: f64 = ((nv7 - 0.0) * s.v[611]);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * s.dn[611][0]);
        let eq18_e430_d_n1: f64 = ((nv7 - 0.0) * s.dn[611][1]);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * s.dn[611][2]);
        let eq18_e430_d_n3: f64 = ((nv7 - 0.0) * s.dn[611][3]);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * s.dn[611][4]);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * s.dn[611][5]);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * s.dn[611][6]);
        let eq18_e430_d_n7: f64 = (s.v[611] + ((nv7 - 0.0) * s.dn[611][7]));
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * s.dn[611][8]);
        let eq18_e430_d_n9: f64 = ((nv7 - 0.0) * s.dn[611][9]);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * s.dn[611][10]);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * s.dn[611][11]);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * s.dn[611][12]);
        let eq18_e430_d_b0: f64 = ((nv7 - 0.0) * s.db[611][0]);
        let eq18_e430_d_b1: f64 = ((nv7 - 0.0) * s.db[611][1]);
        let eq18_e430_d_b2: f64 = ((nv7 - 0.0) * s.db[611][2]);
        let eq18_e430_d_b3: f64 = ((nv7 - 0.0) * s.db[611][3]);
        let eq18_e431: f64 = self.eval_ddt(3, eq18_e430);
        let eq18_e431_d_n0: f64 = self.ddt_jacobian(eq18_e430_d_n0);
        let eq18_e431_d_n1: f64 = self.ddt_jacobian(eq18_e430_d_n1);
        let eq18_e431_d_n2: f64 = self.ddt_jacobian(eq18_e430_d_n2);
        let eq18_e431_d_n3: f64 = self.ddt_jacobian(eq18_e430_d_n3);
        let eq18_e431_d_n4: f64 = self.ddt_jacobian(eq18_e430_d_n4);
        let eq18_e431_d_n5: f64 = self.ddt_jacobian(eq18_e430_d_n5);
        let eq18_e431_d_n6: f64 = self.ddt_jacobian(eq18_e430_d_n6);
        let eq18_e431_d_n7: f64 = self.ddt_jacobian(eq18_e430_d_n7);
        let eq18_e431_d_n8: f64 = self.ddt_jacobian(eq18_e430_d_n8);
        let eq18_e431_d_n9: f64 = self.ddt_jacobian(eq18_e430_d_n9);
        let eq18_e431_d_n10: f64 = self.ddt_jacobian(eq18_e430_d_n10);
        let eq18_e431_d_n11: f64 = self.ddt_jacobian(eq18_e430_d_n11);
        let eq18_e431_d_n12: f64 = self.ddt_jacobian(eq18_e430_d_n12);
        let eq18_e431_d_b0: f64 = self.ddt_jacobian(eq18_e430_d_b0);
        let eq18_e431_d_b1: f64 = self.ddt_jacobian(eq18_e430_d_b1);
        let eq18_e431_d_b2: f64 = self.ddt_jacobian(eq18_e430_d_b2);
        let eq18_e431_d_b3: f64 = self.ddt_jacobian(eq18_e430_d_b3);
        let eq18_value: f64 = eq18_e431;
        let eq18_node_derivatives: [f64; 13] = [eq18_e431_d_n0, eq18_e431_d_n1, eq18_e431_d_n2, eq18_e431_d_n3, eq18_e431_d_n4, eq18_e431_d_n5, eq18_e431_d_n6, eq18_e431_d_n7, eq18_e431_d_n8, eq18_e431_d_n9, eq18_e431_d_n10, eq18_e431_d_n11, eq18_e431_d_n12];
        let eq18_branch_derivatives: [f64; 4] = [eq18_e431_d_b0, eq18_e431_d_b1, eq18_e431_d_b2, eq18_e431_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            self.multiplicity * (eq18_value),
            &nodes,
            &eq18_node_derivatives,
            &branches,
            &eq18_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq19_e434: f64 = ((nv7 - 0.0) * s.v[612]);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * s.dn[612][0]);
        let eq19_e434_d_n1: f64 = ((nv7 - 0.0) * s.dn[612][1]);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * s.dn[612][2]);
        let eq19_e434_d_n3: f64 = ((nv7 - 0.0) * s.dn[612][3]);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * s.dn[612][4]);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * s.dn[612][5]);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * s.dn[612][6]);
        let eq19_e434_d_n7: f64 = (s.v[612] + ((nv7 - 0.0) * s.dn[612][7]));
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * s.dn[612][8]);
        let eq19_e434_d_n9: f64 = ((nv7 - 0.0) * s.dn[612][9]);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * s.dn[612][10]);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * s.dn[612][11]);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * s.dn[612][12]);
        let eq19_e434_d_b0: f64 = ((nv7 - 0.0) * s.db[612][0]);
        let eq19_e434_d_b1: f64 = ((nv7 - 0.0) * s.db[612][1]);
        let eq19_e434_d_b2: f64 = ((nv7 - 0.0) * s.db[612][2]);
        let eq19_e434_d_b3: f64 = ((nv7 - 0.0) * s.db[612][3]);
        let eq19_e435: f64 = self.eval_ddt(4, eq19_e434);
        let eq19_e435_d_n0: f64 = self.ddt_jacobian(eq19_e434_d_n0);
        let eq19_e435_d_n1: f64 = self.ddt_jacobian(eq19_e434_d_n1);
        let eq19_e435_d_n2: f64 = self.ddt_jacobian(eq19_e434_d_n2);
        let eq19_e435_d_n3: f64 = self.ddt_jacobian(eq19_e434_d_n3);
        let eq19_e435_d_n4: f64 = self.ddt_jacobian(eq19_e434_d_n4);
        let eq19_e435_d_n5: f64 = self.ddt_jacobian(eq19_e434_d_n5);
        let eq19_e435_d_n6: f64 = self.ddt_jacobian(eq19_e434_d_n6);
        let eq19_e435_d_n7: f64 = self.ddt_jacobian(eq19_e434_d_n7);
        let eq19_e435_d_n8: f64 = self.ddt_jacobian(eq19_e434_d_n8);
        let eq19_e435_d_n9: f64 = self.ddt_jacobian(eq19_e434_d_n9);
        let eq19_e435_d_n10: f64 = self.ddt_jacobian(eq19_e434_d_n10);
        let eq19_e435_d_n11: f64 = self.ddt_jacobian(eq19_e434_d_n11);
        let eq19_e435_d_n12: f64 = self.ddt_jacobian(eq19_e434_d_n12);
        let eq19_e435_d_b0: f64 = self.ddt_jacobian(eq19_e434_d_b0);
        let eq19_e435_d_b1: f64 = self.ddt_jacobian(eq19_e434_d_b1);
        let eq19_e435_d_b2: f64 = self.ddt_jacobian(eq19_e434_d_b2);
        let eq19_e435_d_b3: f64 = self.ddt_jacobian(eq19_e434_d_b3);
        let eq19_value: f64 = eq19_e435;
        let eq19_node_derivatives: [f64; 13] = [eq19_e435_d_n0, eq19_e435_d_n1, eq19_e435_d_n2, eq19_e435_d_n3, eq19_e435_d_n4, eq19_e435_d_n5, eq19_e435_d_n6, eq19_e435_d_n7, eq19_e435_d_n8, eq19_e435_d_n9, eq19_e435_d_n10, eq19_e435_d_n11, eq19_e435_d_n12];
        let eq19_branch_derivatives: [f64; 4] = [eq19_e435_d_b0, eq19_e435_d_b1, eq19_e435_d_b2, eq19_e435_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            self.multiplicity * (eq19_value),
            &nodes,
            &eq19_node_derivatives,
            &branches,
            &eq19_branch_derivatives,
            self.multiplicity,
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
        let (eq20_e443,) = {
    if (p.p312 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e443;
        stamper.stamp_current(
            Some(nodes[12]),
            Some(nodes[2]),
            self.multiplicity * (eq20_value),
            &[
            ],
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
        let (eq21_e451,) = {
    if (p.p313 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e451;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[11]),
            self.multiplicity * (eq21_value),
            &[
            ],
        );
    }
}
