#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq10_e1297,) = {
    if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e1297;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq10_value),
            &[
            ],
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq11_e1314, eq11_e1314_d_n0, eq11_e1314_d_n1, eq11_e1314_d_n2, eq11_e1314_d_n3, eq11_e1314_d_n4, eq11_e1314_d_n5, eq11_e1314_d_n6, eq11_e1314_d_n7, eq11_e1314_d_n8, eq11_e1314_d_n9, eq11_e1314_d_n10, eq11_e1314_d_n11, eq11_e1314_d_n12, eq11_e1314_d_n13,) = {
    if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
        let eq11_e1308: f64 = (p.p32 * (nv13 - 0.0));
        let eq11_e1308_d_n13: f64 = p.p32;
        let eq11_e1310: f64 = (eq11_e1308 * s.v[1497]);
        let eq11_e1310_d_n0: f64 = (eq11_e1308 * s.dn[1497][0]);
        let eq11_e1310_d_n1: f64 = (eq11_e1308 * s.dn[1497][1]);
        let eq11_e1310_d_n2: f64 = (eq11_e1308 * s.dn[1497][2]);
        let eq11_e1310_d_n3: f64 = (eq11_e1308 * s.dn[1497][3]);
        let eq11_e1310_d_n4: f64 = (eq11_e1308 * s.dn[1497][4]);
        let eq11_e1310_d_n5: f64 = (eq11_e1308 * s.dn[1497][5]);
        let eq11_e1310_d_n6: f64 = (eq11_e1308 * s.dn[1497][6]);
        let eq11_e1310_d_n7: f64 = (eq11_e1308 * s.dn[1497][7]);
        let eq11_e1310_d_n8: f64 = (eq11_e1308 * s.dn[1497][8]);
        let eq11_e1310_d_n9: f64 = (eq11_e1308 * s.dn[1497][9]);
        let eq11_e1310_d_n10: f64 = (eq11_e1308 * s.dn[1497][10]);
        let eq11_e1310_d_n11: f64 = (eq11_e1308 * s.dn[1497][11]);
        let eq11_e1310_d_n12: f64 = (eq11_e1308 * s.dn[1497][12]);
        let eq11_e1310_d_n13: f64 = ((eq11_e1308_d_n13 * s.v[1497]) + (eq11_e1308 * s.dn[1497][13]));
        let eq11_e1312: f64 = (eq11_e1310 * p.p226);
        let eq11_e1312_d_n0: f64 = (eq11_e1310_d_n0 * p.p226);
        let eq11_e1312_d_n1: f64 = (eq11_e1310_d_n1 * p.p226);
        let eq11_e1312_d_n2: f64 = (eq11_e1310_d_n2 * p.p226);
        let eq11_e1312_d_n3: f64 = (eq11_e1310_d_n3 * p.p226);
        let eq11_e1312_d_n4: f64 = (eq11_e1310_d_n4 * p.p226);
        let eq11_e1312_d_n5: f64 = (eq11_e1310_d_n5 * p.p226);
        let eq11_e1312_d_n6: f64 = (eq11_e1310_d_n6 * p.p226);
        let eq11_e1312_d_n7: f64 = (eq11_e1310_d_n7 * p.p226);
        let eq11_e1312_d_n8: f64 = (eq11_e1310_d_n8 * p.p226);
        let eq11_e1312_d_n9: f64 = (eq11_e1310_d_n9 * p.p226);
        let eq11_e1312_d_n10: f64 = (eq11_e1310_d_n10 * p.p226);
        let eq11_e1312_d_n11: f64 = (eq11_e1310_d_n11 * p.p226);
        let eq11_e1312_d_n12: f64 = (eq11_e1310_d_n12 * p.p226);
        let eq11_e1312_d_n13: f64 = (eq11_e1310_d_n13 * p.p226);
        (eq11_e1312, eq11_e1312_d_n0, eq11_e1312_d_n1, eq11_e1312_d_n2, eq11_e1312_d_n3, eq11_e1312_d_n4, eq11_e1312_d_n5, eq11_e1312_d_n6, eq11_e1312_d_n7, eq11_e1312_d_n8, eq11_e1312_d_n9, eq11_e1312_d_n10, eq11_e1312_d_n11, eq11_e1312_d_n12, eq11_e1312_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1314;
        let eq11_node_derivatives: [f64; 14] = [eq11_e1314_d_n0, eq11_e1314_d_n1, eq11_e1314_d_n2, eq11_e1314_d_n3, eq11_e1314_d_n4, eq11_e1314_d_n5, eq11_e1314_d_n6, eq11_e1314_d_n7, eq11_e1314_d_n8, eq11_e1314_d_n9, eq11_e1314_d_n10, eq11_e1314_d_n11, eq11_e1314_d_n12, eq11_e1314_d_n13];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            None,
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
        let (eq12_e1337,) = {
    if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e1337;
        stamper.stamp_current(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq12_value),
            &[
            ],
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq13_e1356, eq13_e1356_d_n0, eq13_e1356_d_n1, eq13_e1356_d_n2, eq13_e1356_d_n3, eq13_e1356_d_n4, eq13_e1356_d_n5, eq13_e1356_d_n6, eq13_e1356_d_n7, eq13_e1356_d_n8, eq13_e1356_d_n9, eq13_e1356_d_n10, eq13_e1356_d_n11, eq13_e1356_d_n12, eq13_e1356_d_n13,) = {
    if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
        let eq13_e1348: f64 = (p.p32 * s.v[1498]);
        let eq13_e1348_d_n0: f64 = (p.p32 * s.dn[1498][0]);
        let eq13_e1348_d_n1: f64 = (p.p32 * s.dn[1498][1]);
        let eq13_e1348_d_n2: f64 = (p.p32 * s.dn[1498][2]);
        let eq13_e1348_d_n3: f64 = (p.p32 * s.dn[1498][3]);
        let eq13_e1348_d_n4: f64 = (p.p32 * s.dn[1498][4]);
        let eq13_e1348_d_n5: f64 = (p.p32 * s.dn[1498][5]);
        let eq13_e1348_d_n6: f64 = (p.p32 * s.dn[1498][6]);
        let eq13_e1348_d_n7: f64 = (p.p32 * s.dn[1498][7]);
        let eq13_e1348_d_n8: f64 = (p.p32 * s.dn[1498][8]);
        let eq13_e1348_d_n9: f64 = (p.p32 * s.dn[1498][9]);
        let eq13_e1348_d_n10: f64 = (p.p32 * s.dn[1498][10]);
        let eq13_e1348_d_n11: f64 = (p.p32 * s.dn[1498][11]);
        let eq13_e1348_d_n12: f64 = (p.p32 * s.dn[1498][12]);
        let eq13_e1348_d_n13: f64 = (p.p32 * s.dn[1498][13]);
        let eq13_e1350: f64 = (eq13_e1348 * (nv13 - 0.0));
        let eq13_e1350_d_n0: f64 = (eq13_e1348_d_n0 * (nv13 - 0.0));
        let eq13_e1350_d_n1: f64 = (eq13_e1348_d_n1 * (nv13 - 0.0));
        let eq13_e1350_d_n2: f64 = (eq13_e1348_d_n2 * (nv13 - 0.0));
        let eq13_e1350_d_n3: f64 = (eq13_e1348_d_n3 * (nv13 - 0.0));
        let eq13_e1350_d_n4: f64 = (eq13_e1348_d_n4 * (nv13 - 0.0));
        let eq13_e1350_d_n5: f64 = (eq13_e1348_d_n5 * (nv13 - 0.0));
        let eq13_e1350_d_n6: f64 = (eq13_e1348_d_n6 * (nv13 - 0.0));
        let eq13_e1350_d_n7: f64 = (eq13_e1348_d_n7 * (nv13 - 0.0));
        let eq13_e1350_d_n8: f64 = (eq13_e1348_d_n8 * (nv13 - 0.0));
        let eq13_e1350_d_n9: f64 = (eq13_e1348_d_n9 * (nv13 - 0.0));
        let eq13_e1350_d_n10: f64 = (eq13_e1348_d_n10 * (nv13 - 0.0));
        let eq13_e1350_d_n11: f64 = (eq13_e1348_d_n11 * (nv13 - 0.0));
        let eq13_e1350_d_n12: f64 = (eq13_e1348_d_n12 * (nv13 - 0.0));
        let eq13_e1350_d_n13: f64 = ((eq13_e1348_d_n13 * (nv13 - 0.0)) + eq13_e1348);
        let eq13_e1352: f64 = (eq13_e1350 * s.v[1497]);
        let eq13_e1352_d_n0: f64 = ((eq13_e1350_d_n0 * s.v[1497]) + (eq13_e1350 * s.dn[1497][0]));
        let eq13_e1352_d_n1: f64 = ((eq13_e1350_d_n1 * s.v[1497]) + (eq13_e1350 * s.dn[1497][1]));
        let eq13_e1352_d_n2: f64 = ((eq13_e1350_d_n2 * s.v[1497]) + (eq13_e1350 * s.dn[1497][2]));
        let eq13_e1352_d_n3: f64 = ((eq13_e1350_d_n3 * s.v[1497]) + (eq13_e1350 * s.dn[1497][3]));
        let eq13_e1352_d_n4: f64 = ((eq13_e1350_d_n4 * s.v[1497]) + (eq13_e1350 * s.dn[1497][4]));
        let eq13_e1352_d_n5: f64 = ((eq13_e1350_d_n5 * s.v[1497]) + (eq13_e1350 * s.dn[1497][5]));
        let eq13_e1352_d_n6: f64 = ((eq13_e1350_d_n6 * s.v[1497]) + (eq13_e1350 * s.dn[1497][6]));
        let eq13_e1352_d_n7: f64 = ((eq13_e1350_d_n7 * s.v[1497]) + (eq13_e1350 * s.dn[1497][7]));
        let eq13_e1352_d_n8: f64 = ((eq13_e1350_d_n8 * s.v[1497]) + (eq13_e1350 * s.dn[1497][8]));
        let eq13_e1352_d_n9: f64 = ((eq13_e1350_d_n9 * s.v[1497]) + (eq13_e1350 * s.dn[1497][9]));
        let eq13_e1352_d_n10: f64 = ((eq13_e1350_d_n10 * s.v[1497]) + (eq13_e1350 * s.dn[1497][10]));
        let eq13_e1352_d_n11: f64 = ((eq13_e1350_d_n11 * s.v[1497]) + (eq13_e1350 * s.dn[1497][11]));
        let eq13_e1352_d_n12: f64 = ((eq13_e1350_d_n12 * s.v[1497]) + (eq13_e1350 * s.dn[1497][12]));
        let eq13_e1352_d_n13: f64 = ((eq13_e1350_d_n13 * s.v[1497]) + (eq13_e1350 * s.dn[1497][13]));
        let eq13_e1354: f64 = (eq13_e1352 * p.p226);
        let eq13_e1354_d_n0: f64 = (eq13_e1352_d_n0 * p.p226);
        let eq13_e1354_d_n1: f64 = (eq13_e1352_d_n1 * p.p226);
        let eq13_e1354_d_n2: f64 = (eq13_e1352_d_n2 * p.p226);
        let eq13_e1354_d_n3: f64 = (eq13_e1352_d_n3 * p.p226);
        let eq13_e1354_d_n4: f64 = (eq13_e1352_d_n4 * p.p226);
        let eq13_e1354_d_n5: f64 = (eq13_e1352_d_n5 * p.p226);
        let eq13_e1354_d_n6: f64 = (eq13_e1352_d_n6 * p.p226);
        let eq13_e1354_d_n7: f64 = (eq13_e1352_d_n7 * p.p226);
        let eq13_e1354_d_n8: f64 = (eq13_e1352_d_n8 * p.p226);
        let eq13_e1354_d_n9: f64 = (eq13_e1352_d_n9 * p.p226);
        let eq13_e1354_d_n10: f64 = (eq13_e1352_d_n10 * p.p226);
        let eq13_e1354_d_n11: f64 = (eq13_e1352_d_n11 * p.p226);
        let eq13_e1354_d_n12: f64 = (eq13_e1352_d_n12 * p.p226);
        let eq13_e1354_d_n13: f64 = (eq13_e1352_d_n13 * p.p226);
        (eq13_e1354, eq13_e1354_d_n0, eq13_e1354_d_n1, eq13_e1354_d_n2, eq13_e1354_d_n3, eq13_e1354_d_n4, eq13_e1354_d_n5, eq13_e1354_d_n6, eq13_e1354_d_n7, eq13_e1354_d_n8, eq13_e1354_d_n9, eq13_e1354_d_n10, eq13_e1354_d_n11, eq13_e1354_d_n12, eq13_e1354_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1356;
        let eq13_node_derivatives: [f64; 14] = [eq13_e1356_d_n0, eq13_e1356_d_n1, eq13_e1356_d_n2, eq13_e1356_d_n3, eq13_e1356_d_n4, eq13_e1356_d_n5, eq13_e1356_d_n6, eq13_e1356_d_n7, eq13_e1356_d_n8, eq13_e1356_d_n9, eq13_e1356_d_n10, eq13_e1356_d_n11, eq13_e1356_d_n12, eq13_e1356_d_n13];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq13_value),
            &nodes,
            &eq13_node_derivatives,
            &branches,
            &eq13_branch_derivatives,
            self.multiplicity,
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq14_e1376, eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13,) = {
    if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
        let eq14_e1367: f64 = (p.p33 * 0.5);
        let eq14_e1369: f64 = (eq14_e1367 * s.v[1501]);
        let eq14_e1369_d_n0: f64 = (eq14_e1367 * s.dn[1501][0]);
        let eq14_e1369_d_n1: f64 = (eq14_e1367 * s.dn[1501][1]);
        let eq14_e1369_d_n2: f64 = (eq14_e1367 * s.dn[1501][2]);
        let eq14_e1369_d_n3: f64 = (eq14_e1367 * s.dn[1501][3]);
        let eq14_e1369_d_n4: f64 = (eq14_e1367 * s.dn[1501][4]);
        let eq14_e1369_d_n5: f64 = (eq14_e1367 * s.dn[1501][5]);
        let eq14_e1369_d_n6: f64 = (eq14_e1367 * s.dn[1501][6]);
        let eq14_e1369_d_n7: f64 = (eq14_e1367 * s.dn[1501][7]);
        let eq14_e1369_d_n8: f64 = (eq14_e1367 * s.dn[1501][8]);
        let eq14_e1369_d_n9: f64 = (eq14_e1367 * s.dn[1501][9]);
        let eq14_e1369_d_n10: f64 = (eq14_e1367 * s.dn[1501][10]);
        let eq14_e1369_d_n11: f64 = (eq14_e1367 * s.dn[1501][11]);
        let eq14_e1369_d_n12: f64 = (eq14_e1367 * s.dn[1501][12]);
        let eq14_e1369_d_n13: f64 = (eq14_e1367 * s.dn[1501][13]);
        let eq14_e1371: f64 = (eq14_e1369 * p.p226);
        let eq14_e1371_d_n0: f64 = (eq14_e1369_d_n0 * p.p226);
        let eq14_e1371_d_n1: f64 = (eq14_e1369_d_n1 * p.p226);
        let eq14_e1371_d_n2: f64 = (eq14_e1369_d_n2 * p.p226);
        let eq14_e1371_d_n3: f64 = (eq14_e1369_d_n3 * p.p226);
        let eq14_e1371_d_n4: f64 = (eq14_e1369_d_n4 * p.p226);
        let eq14_e1371_d_n5: f64 = (eq14_e1369_d_n5 * p.p226);
        let eq14_e1371_d_n6: f64 = (eq14_e1369_d_n6 * p.p226);
        let eq14_e1371_d_n7: f64 = (eq14_e1369_d_n7 * p.p226);
        let eq14_e1371_d_n8: f64 = (eq14_e1369_d_n8 * p.p226);
        let eq14_e1371_d_n9: f64 = (eq14_e1369_d_n9 * p.p226);
        let eq14_e1371_d_n10: f64 = (eq14_e1369_d_n10 * p.p226);
        let eq14_e1371_d_n11: f64 = (eq14_e1369_d_n11 * p.p226);
        let eq14_e1371_d_n12: f64 = (eq14_e1369_d_n12 * p.p226);
        let eq14_e1371_d_n13: f64 = (eq14_e1369_d_n13 * p.p226);
        let eq14_e1373: f64 = (eq14_e1371 * (nv13 - 0.0));
        let eq14_e1373_d_n0: f64 = (eq14_e1371_d_n0 * (nv13 - 0.0));
        let eq14_e1373_d_n1: f64 = (eq14_e1371_d_n1 * (nv13 - 0.0));
        let eq14_e1373_d_n2: f64 = (eq14_e1371_d_n2 * (nv13 - 0.0));
        let eq14_e1373_d_n3: f64 = (eq14_e1371_d_n3 * (nv13 - 0.0));
        let eq14_e1373_d_n4: f64 = (eq14_e1371_d_n4 * (nv13 - 0.0));
        let eq14_e1373_d_n5: f64 = (eq14_e1371_d_n5 * (nv13 - 0.0));
        let eq14_e1373_d_n6: f64 = (eq14_e1371_d_n6 * (nv13 - 0.0));
        let eq14_e1373_d_n7: f64 = (eq14_e1371_d_n7 * (nv13 - 0.0));
        let eq14_e1373_d_n8: f64 = (eq14_e1371_d_n8 * (nv13 - 0.0));
        let eq14_e1373_d_n9: f64 = (eq14_e1371_d_n9 * (nv13 - 0.0));
        let eq14_e1373_d_n10: f64 = (eq14_e1371_d_n10 * (nv13 - 0.0));
        let eq14_e1373_d_n11: f64 = (eq14_e1371_d_n11 * (nv13 - 0.0));
        let eq14_e1373_d_n12: f64 = (eq14_e1371_d_n12 * (nv13 - 0.0));
        let eq14_e1373_d_n13: f64 = ((eq14_e1371_d_n13 * (nv13 - 0.0)) + eq14_e1371);
        let eq14_e1374: f64 = self.eval_ddt(0, eq14_e1373);
        let eq14_e1374_d_n0: f64 = self.ddt_jacobian(eq14_e1373_d_n0);
        let eq14_e1374_d_n1: f64 = self.ddt_jacobian(eq14_e1373_d_n1);
        let eq14_e1374_d_n2: f64 = self.ddt_jacobian(eq14_e1373_d_n2);
        let eq14_e1374_d_n3: f64 = self.ddt_jacobian(eq14_e1373_d_n3);
        let eq14_e1374_d_n4: f64 = self.ddt_jacobian(eq14_e1373_d_n4);
        let eq14_e1374_d_n5: f64 = self.ddt_jacobian(eq14_e1373_d_n5);
        let eq14_e1374_d_n6: f64 = self.ddt_jacobian(eq14_e1373_d_n6);
        let eq14_e1374_d_n7: f64 = self.ddt_jacobian(eq14_e1373_d_n7);
        let eq14_e1374_d_n8: f64 = self.ddt_jacobian(eq14_e1373_d_n8);
        let eq14_e1374_d_n9: f64 = self.ddt_jacobian(eq14_e1373_d_n9);
        let eq14_e1374_d_n10: f64 = self.ddt_jacobian(eq14_e1373_d_n10);
        let eq14_e1374_d_n11: f64 = self.ddt_jacobian(eq14_e1373_d_n11);
        let eq14_e1374_d_n12: f64 = self.ddt_jacobian(eq14_e1373_d_n12);
        let eq14_e1374_d_n13: f64 = self.ddt_jacobian(eq14_e1373_d_n13);
        (eq14_e1374, eq14_e1374_d_n0, eq14_e1374_d_n1, eq14_e1374_d_n2, eq14_e1374_d_n3, eq14_e1374_d_n4, eq14_e1374_d_n5, eq14_e1374_d_n6, eq14_e1374_d_n7, eq14_e1374_d_n8, eq14_e1374_d_n9, eq14_e1374_d_n10, eq14_e1374_d_n11, eq14_e1374_d_n12, eq14_e1374_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e1376;
        let eq14_node_derivatives: [f64; 14] = [eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq14_value),
            &nodes,
            &eq14_node_derivatives,
            &branches,
            &eq14_branch_derivatives,
            self.multiplicity,
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq15_e1396, eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13,) = {
    if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
        let eq15_e1387: f64 = (p.p33 * 0.5);
        let eq15_e1389: f64 = (eq15_e1387 * s.v[1501]);
        let eq15_e1389_d_n0: f64 = (eq15_e1387 * s.dn[1501][0]);
        let eq15_e1389_d_n1: f64 = (eq15_e1387 * s.dn[1501][1]);
        let eq15_e1389_d_n2: f64 = (eq15_e1387 * s.dn[1501][2]);
        let eq15_e1389_d_n3: f64 = (eq15_e1387 * s.dn[1501][3]);
        let eq15_e1389_d_n4: f64 = (eq15_e1387 * s.dn[1501][4]);
        let eq15_e1389_d_n5: f64 = (eq15_e1387 * s.dn[1501][5]);
        let eq15_e1389_d_n6: f64 = (eq15_e1387 * s.dn[1501][6]);
        let eq15_e1389_d_n7: f64 = (eq15_e1387 * s.dn[1501][7]);
        let eq15_e1389_d_n8: f64 = (eq15_e1387 * s.dn[1501][8]);
        let eq15_e1389_d_n9: f64 = (eq15_e1387 * s.dn[1501][9]);
        let eq15_e1389_d_n10: f64 = (eq15_e1387 * s.dn[1501][10]);
        let eq15_e1389_d_n11: f64 = (eq15_e1387 * s.dn[1501][11]);
        let eq15_e1389_d_n12: f64 = (eq15_e1387 * s.dn[1501][12]);
        let eq15_e1389_d_n13: f64 = (eq15_e1387 * s.dn[1501][13]);
        let eq15_e1391: f64 = (eq15_e1389 * p.p226);
        let eq15_e1391_d_n0: f64 = (eq15_e1389_d_n0 * p.p226);
        let eq15_e1391_d_n1: f64 = (eq15_e1389_d_n1 * p.p226);
        let eq15_e1391_d_n2: f64 = (eq15_e1389_d_n2 * p.p226);
        let eq15_e1391_d_n3: f64 = (eq15_e1389_d_n3 * p.p226);
        let eq15_e1391_d_n4: f64 = (eq15_e1389_d_n4 * p.p226);
        let eq15_e1391_d_n5: f64 = (eq15_e1389_d_n5 * p.p226);
        let eq15_e1391_d_n6: f64 = (eq15_e1389_d_n6 * p.p226);
        let eq15_e1391_d_n7: f64 = (eq15_e1389_d_n7 * p.p226);
        let eq15_e1391_d_n8: f64 = (eq15_e1389_d_n8 * p.p226);
        let eq15_e1391_d_n9: f64 = (eq15_e1389_d_n9 * p.p226);
        let eq15_e1391_d_n10: f64 = (eq15_e1389_d_n10 * p.p226);
        let eq15_e1391_d_n11: f64 = (eq15_e1389_d_n11 * p.p226);
        let eq15_e1391_d_n12: f64 = (eq15_e1389_d_n12 * p.p226);
        let eq15_e1391_d_n13: f64 = (eq15_e1389_d_n13 * p.p226);
        let eq15_e1393: f64 = (eq15_e1391 * (nv13 - 0.0));
        let eq15_e1393_d_n0: f64 = (eq15_e1391_d_n0 * (nv13 - 0.0));
        let eq15_e1393_d_n1: f64 = (eq15_e1391_d_n1 * (nv13 - 0.0));
        let eq15_e1393_d_n2: f64 = (eq15_e1391_d_n2 * (nv13 - 0.0));
        let eq15_e1393_d_n3: f64 = (eq15_e1391_d_n3 * (nv13 - 0.0));
        let eq15_e1393_d_n4: f64 = (eq15_e1391_d_n4 * (nv13 - 0.0));
        let eq15_e1393_d_n5: f64 = (eq15_e1391_d_n5 * (nv13 - 0.0));
        let eq15_e1393_d_n6: f64 = (eq15_e1391_d_n6 * (nv13 - 0.0));
        let eq15_e1393_d_n7: f64 = (eq15_e1391_d_n7 * (nv13 - 0.0));
        let eq15_e1393_d_n8: f64 = (eq15_e1391_d_n8 * (nv13 - 0.0));
        let eq15_e1393_d_n9: f64 = (eq15_e1391_d_n9 * (nv13 - 0.0));
        let eq15_e1393_d_n10: f64 = (eq15_e1391_d_n10 * (nv13 - 0.0));
        let eq15_e1393_d_n11: f64 = (eq15_e1391_d_n11 * (nv13 - 0.0));
        let eq15_e1393_d_n12: f64 = (eq15_e1391_d_n12 * (nv13 - 0.0));
        let eq15_e1393_d_n13: f64 = ((eq15_e1391_d_n13 * (nv13 - 0.0)) + eq15_e1391);
        let eq15_e1394: f64 = self.eval_ddt(1, eq15_e1393);
        let eq15_e1394_d_n0: f64 = self.ddt_jacobian(eq15_e1393_d_n0);
        let eq15_e1394_d_n1: f64 = self.ddt_jacobian(eq15_e1393_d_n1);
        let eq15_e1394_d_n2: f64 = self.ddt_jacobian(eq15_e1393_d_n2);
        let eq15_e1394_d_n3: f64 = self.ddt_jacobian(eq15_e1393_d_n3);
        let eq15_e1394_d_n4: f64 = self.ddt_jacobian(eq15_e1393_d_n4);
        let eq15_e1394_d_n5: f64 = self.ddt_jacobian(eq15_e1393_d_n5);
        let eq15_e1394_d_n6: f64 = self.ddt_jacobian(eq15_e1393_d_n6);
        let eq15_e1394_d_n7: f64 = self.ddt_jacobian(eq15_e1393_d_n7);
        let eq15_e1394_d_n8: f64 = self.ddt_jacobian(eq15_e1393_d_n8);
        let eq15_e1394_d_n9: f64 = self.ddt_jacobian(eq15_e1393_d_n9);
        let eq15_e1394_d_n10: f64 = self.ddt_jacobian(eq15_e1393_d_n10);
        let eq15_e1394_d_n11: f64 = self.ddt_jacobian(eq15_e1393_d_n11);
        let eq15_e1394_d_n12: f64 = self.ddt_jacobian(eq15_e1393_d_n12);
        let eq15_e1394_d_n13: f64 = self.ddt_jacobian(eq15_e1393_d_n13);
        (eq15_e1394, eq15_e1394_d_n0, eq15_e1394_d_n1, eq15_e1394_d_n2, eq15_e1394_d_n3, eq15_e1394_d_n4, eq15_e1394_d_n5, eq15_e1394_d_n6, eq15_e1394_d_n7, eq15_e1394_d_n8, eq15_e1394_d_n9, eq15_e1394_d_n10, eq15_e1394_d_n11, eq15_e1394_d_n12, eq15_e1394_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1396;
        let eq15_node_derivatives: [f64; 14] = [eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq15_value),
            &nodes,
            &eq15_node_derivatives,
            &branches,
            &eq15_branch_derivatives,
            self.multiplicity,
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq16_e1400, eq16_e1400_d_n13,) = {
    if (s.v[1514] != 0.0) {
        ((nv13 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e1400;
        stamper.stamp_current(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq16_value),
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * eq16_e1400_d_n13),
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
        let eq17_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq17_value),
            &[
            ],
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq18_e1414, eq18_e1414_d_n0, eq18_e1414_d_n1, eq18_e1414_d_n2, eq18_e1414_d_n3, eq18_e1414_d_n4, eq18_e1414_d_n5, eq18_e1414_d_n6, eq18_e1414_d_n7, eq18_e1414_d_n8, eq18_e1414_d_n9, eq18_e1414_d_n10, eq18_e1414_d_n11, eq18_e1414_d_n12, eq18_e1414_d_n13,) = {
    if (s.v[1546] != 0.0) {
        let eq18_e1410: f64 = (p.p32 * (nv0 - nv7));
        let eq18_e1410_d_n0: f64 = p.p32;
        let eq18_e1410_d_n7: f64 = (-p.p32);
        let eq18_e1412: f64 = (eq18_e1410 / s.v[1099]);
        let eq18_e1412_d_n0: f64 = (((eq18_e1410_d_n0 * s.v[1099]) - (eq18_e1410 * s.dn[1099][0])) / (s.v[1099] * s.v[1099]));
        let eq18_e1412_d_n1: f64 = (-((eq18_e1410 * s.dn[1099][1]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n2: f64 = (-((eq18_e1410 * s.dn[1099][2]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n3: f64 = (-((eq18_e1410 * s.dn[1099][3]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n4: f64 = (-((eq18_e1410 * s.dn[1099][4]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n5: f64 = (-((eq18_e1410 * s.dn[1099][5]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n6: f64 = (-((eq18_e1410 * s.dn[1099][6]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n7: f64 = (((eq18_e1410_d_n7 * s.v[1099]) - (eq18_e1410 * s.dn[1099][7])) / (s.v[1099] * s.v[1099]));
        let eq18_e1412_d_n8: f64 = (-((eq18_e1410 * s.dn[1099][8]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n9: f64 = (-((eq18_e1410 * s.dn[1099][9]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n10: f64 = (-((eq18_e1410 * s.dn[1099][10]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n11: f64 = (-((eq18_e1410 * s.dn[1099][11]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n12: f64 = (-((eq18_e1410 * s.dn[1099][12]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n13: f64 = (-((eq18_e1410 * s.dn[1099][13]) / (s.v[1099] * s.v[1099])));
        (eq18_e1412, eq18_e1412_d_n0, eq18_e1412_d_n1, eq18_e1412_d_n2, eq18_e1412_d_n3, eq18_e1412_d_n4, eq18_e1412_d_n5, eq18_e1412_d_n6, eq18_e1412_d_n7, eq18_e1412_d_n8, eq18_e1412_d_n9, eq18_e1412_d_n10, eq18_e1412_d_n11, eq18_e1412_d_n12, eq18_e1412_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1414;
        let eq18_node_derivatives: [f64; 14] = [eq18_e1414_d_n0, eq18_e1414_d_n1, eq18_e1414_d_n2, eq18_e1414_d_n3, eq18_e1414_d_n4, eq18_e1414_d_n5, eq18_e1414_d_n6, eq18_e1414_d_n7, eq18_e1414_d_n8, eq18_e1414_d_n9, eq18_e1414_d_n10, eq18_e1414_d_n11, eq18_e1414_d_n12, eq18_e1414_d_n13];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
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
        let (eq19_e1425,) = {
    if (s.v[1546] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e1425;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq19_value),
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
        let (eq20_e1430,) = {
    if (!(s.v[1546] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1430;
        stamper.stamp_potential(
            branches[7],
            eq20_value,
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq21_e1438, eq21_e1438_d_n0, eq21_e1438_d_n1, eq21_e1438_d_n2, eq21_e1438_d_n3, eq21_e1438_d_n4, eq21_e1438_d_n5, eq21_e1438_d_n6, eq21_e1438_d_n7, eq21_e1438_d_n8, eq21_e1438_d_n9, eq21_e1438_d_n10, eq21_e1438_d_n11, eq21_e1438_d_n12, eq21_e1438_d_n13,) = {
    if (s.v[1547] != 0.0) {
        let eq21_e1434: f64 = (p.p32 * (nv2 - nv8));
        let eq21_e1434_d_n2: f64 = p.p32;
        let eq21_e1434_d_n8: f64 = (-p.p32);
        let eq21_e1436: f64 = (eq21_e1434 / s.v[1100]);
        let eq21_e1436_d_n0: f64 = (-((eq21_e1434 * s.dn[1100][0]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n1: f64 = (-((eq21_e1434 * s.dn[1100][1]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n2: f64 = (((eq21_e1434_d_n2 * s.v[1100]) - (eq21_e1434 * s.dn[1100][2])) / (s.v[1100] * s.v[1100]));
        let eq21_e1436_d_n3: f64 = (-((eq21_e1434 * s.dn[1100][3]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n4: f64 = (-((eq21_e1434 * s.dn[1100][4]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n5: f64 = (-((eq21_e1434 * s.dn[1100][5]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n6: f64 = (-((eq21_e1434 * s.dn[1100][6]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n7: f64 = (-((eq21_e1434 * s.dn[1100][7]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n8: f64 = (((eq21_e1434_d_n8 * s.v[1100]) - (eq21_e1434 * s.dn[1100][8])) / (s.v[1100] * s.v[1100]));
        let eq21_e1436_d_n9: f64 = (-((eq21_e1434 * s.dn[1100][9]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n10: f64 = (-((eq21_e1434 * s.dn[1100][10]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n11: f64 = (-((eq21_e1434 * s.dn[1100][11]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n12: f64 = (-((eq21_e1434 * s.dn[1100][12]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n13: f64 = (-((eq21_e1434 * s.dn[1100][13]) / (s.v[1100] * s.v[1100])));
        (eq21_e1436, eq21_e1436_d_n0, eq21_e1436_d_n1, eq21_e1436_d_n2, eq21_e1436_d_n3, eq21_e1436_d_n4, eq21_e1436_d_n5, eq21_e1436_d_n6, eq21_e1436_d_n7, eq21_e1436_d_n8, eq21_e1436_d_n9, eq21_e1436_d_n10, eq21_e1436_d_n11, eq21_e1436_d_n12, eq21_e1436_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1438;
        let eq21_node_derivatives: [f64; 14] = [eq21_e1438_d_n0, eq21_e1438_d_n1, eq21_e1438_d_n2, eq21_e1438_d_n3, eq21_e1438_d_n4, eq21_e1438_d_n5, eq21_e1438_d_n6, eq21_e1438_d_n7, eq21_e1438_d_n8, eq21_e1438_d_n9, eq21_e1438_d_n10, eq21_e1438_d_n11, eq21_e1438_d_n12, eq21_e1438_d_n13];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq21_value),
            &nodes,
            &eq21_node_derivatives,
            &branches,
            &eq21_branch_derivatives,
            self.multiplicity,
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
        let (eq22_e1449,) = {
    if (s.v[1547] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e1449;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq22_value),
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
        let (eq23_e1454,) = {
    if (!(s.v[1547] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1454;
        stamper.stamp_potential(
            branches[8],
            eq23_value,
            &[
            ],
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq24_e1472, eq24_e1472_d_n0, eq24_e1472_d_n1, eq24_e1472_d_n2, eq24_e1472_d_n3, eq24_e1472_d_n4, eq24_e1472_d_n5, eq24_e1472_d_n6, eq24_e1472_d_n7, eq24_e1472_d_n8, eq24_e1472_d_n9, eq24_e1472_d_n10, eq24_e1472_d_n11, eq24_e1472_d_n12, eq24_e1472_d_n13,) = {
    if (s.v[1548] != 0.0) {
        let eq24_e1458: f64 = (p.p37 * p.p32);
        let eq24_e1461: f64 = (s.v[885] + s.v[933]);
        let eq24_e1461_d_n0: f64 = (s.dn[885][0] + s.dn[933][0]);
        let eq24_e1461_d_n1: f64 = (s.dn[885][1] + s.dn[933][1]);
        let eq24_e1461_d_n2: f64 = (s.dn[885][2] + s.dn[933][2]);
        let eq24_e1461_d_n3: f64 = (s.dn[885][3] + s.dn[933][3]);
        let eq24_e1461_d_n4: f64 = (s.dn[885][4] + s.dn[933][4]);
        let eq24_e1461_d_n5: f64 = (s.dn[885][5] + s.dn[933][5]);
        let eq24_e1461_d_n6: f64 = (s.dn[885][6] + s.dn[933][6]);
        let eq24_e1461_d_n7: f64 = (s.dn[885][7] + s.dn[933][7]);
        let eq24_e1461_d_n8: f64 = (s.dn[885][8] + s.dn[933][8]);
        let eq24_e1461_d_n9: f64 = (s.dn[885][9] + s.dn[933][9]);
        let eq24_e1461_d_n10: f64 = (s.dn[885][10] + s.dn[933][10]);
        let eq24_e1461_d_n11: f64 = (s.dn[885][11] + s.dn[933][11]);
        let eq24_e1461_d_n12: f64 = (s.dn[885][12] + s.dn[933][12]);
        let eq24_e1461_d_n13: f64 = (s.dn[885][13] + s.dn[933][13]);
        let eq24_e1462: f64 = (eq24_e1458 * eq24_e1461);
        let eq24_e1462_d_n0: f64 = (eq24_e1458 * eq24_e1461_d_n0);
        let eq24_e1462_d_n1: f64 = (eq24_e1458 * eq24_e1461_d_n1);
        let eq24_e1462_d_n2: f64 = (eq24_e1458 * eq24_e1461_d_n2);
        let eq24_e1462_d_n3: f64 = (eq24_e1458 * eq24_e1461_d_n3);
        let eq24_e1462_d_n4: f64 = (eq24_e1458 * eq24_e1461_d_n4);
        let eq24_e1462_d_n5: f64 = (eq24_e1458 * eq24_e1461_d_n5);
        let eq24_e1462_d_n6: f64 = (eq24_e1458 * eq24_e1461_d_n6);
        let eq24_e1462_d_n7: f64 = (eq24_e1458 * eq24_e1461_d_n7);
        let eq24_e1462_d_n8: f64 = (eq24_e1458 * eq24_e1461_d_n8);
        let eq24_e1462_d_n9: f64 = (eq24_e1458 * eq24_e1461_d_n9);
        let eq24_e1462_d_n10: f64 = (eq24_e1458 * eq24_e1461_d_n10);
        let eq24_e1462_d_n11: f64 = (eq24_e1458 * eq24_e1461_d_n11);
        let eq24_e1462_d_n12: f64 = (eq24_e1458 * eq24_e1461_d_n12);
        let eq24_e1462_d_n13: f64 = (eq24_e1458 * eq24_e1461_d_n13);
        let eq24_e1466: f64 = 0.0;
        let eq24_e1468: f64 = (eq24_e1466 * (nv7 - nv8));
        let eq24_e1468_d_n8: f64 = (-eq24_e1466);
        let eq24_e1469: f64 = (p.p32 * eq24_e1468);
        let eq24_e1469_d_n7: f64 = (p.p32 * eq24_e1466);
        let eq24_e1469_d_n8: f64 = (p.p32 * eq24_e1468_d_n8);
        let eq24_e1470: f64 = (eq24_e1462 + eq24_e1469);
        let eq24_e1470_d_n7: f64 = (eq24_e1462_d_n7 + eq24_e1469_d_n7);
        let eq24_e1470_d_n8: f64 = (eq24_e1462_d_n8 + eq24_e1469_d_n8);
        (eq24_e1470, eq24_e1462_d_n0, eq24_e1462_d_n1, eq24_e1462_d_n2, eq24_e1462_d_n3, eq24_e1462_d_n4, eq24_e1462_d_n5, eq24_e1462_d_n6, eq24_e1470_d_n7, eq24_e1470_d_n8, eq24_e1462_d_n9, eq24_e1462_d_n10, eq24_e1462_d_n11, eq24_e1462_d_n12, eq24_e1462_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1472;
        let eq24_node_derivatives: [f64; 14] = [eq24_e1472_d_n0, eq24_e1472_d_n1, eq24_e1472_d_n2, eq24_e1472_d_n3, eq24_e1472_d_n4, eq24_e1472_d_n5, eq24_e1472_d_n6, eq24_e1472_d_n7, eq24_e1472_d_n8, eq24_e1472_d_n9, eq24_e1472_d_n10, eq24_e1472_d_n11, eq24_e1472_d_n12, eq24_e1472_d_n13];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
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
        let (eq25_e1480, eq25_e1480_d_n0, eq25_e1480_d_n1, eq25_e1480_d_n2, eq25_e1480_d_n3, eq25_e1480_d_n4, eq25_e1480_d_n5, eq25_e1480_d_n6, eq25_e1480_d_n7, eq25_e1480_d_n8, eq25_e1480_d_n9, eq25_e1480_d_n10, eq25_e1480_d_n11, eq25_e1480_d_n12, eq25_e1480_d_n13,) = {
    if (s.v[1548] != 0.0) {
        let eq25_e1476: f64 = (p.p37 * p.p32);
        let eq25_e1478: f64 = (eq25_e1476 * s.v[908]);
        let eq25_e1478_d_n0: f64 = (eq25_e1476 * s.dn[908][0]);
        let eq25_e1478_d_n1: f64 = (eq25_e1476 * s.dn[908][1]);
        let eq25_e1478_d_n2: f64 = (eq25_e1476 * s.dn[908][2]);
        let eq25_e1478_d_n3: f64 = (eq25_e1476 * s.dn[908][3]);
        let eq25_e1478_d_n4: f64 = (eq25_e1476 * s.dn[908][4]);
        let eq25_e1478_d_n5: f64 = (eq25_e1476 * s.dn[908][5]);
        let eq25_e1478_d_n6: f64 = (eq25_e1476 * s.dn[908][6]);
        let eq25_e1478_d_n7: f64 = (eq25_e1476 * s.dn[908][7]);
        let eq25_e1478_d_n8: f64 = (eq25_e1476 * s.dn[908][8]);
        let eq25_e1478_d_n9: f64 = (eq25_e1476 * s.dn[908][9]);
        let eq25_e1478_d_n10: f64 = (eq25_e1476 * s.dn[908][10]);
        let eq25_e1478_d_n11: f64 = (eq25_e1476 * s.dn[908][11]);
        let eq25_e1478_d_n12: f64 = (eq25_e1476 * s.dn[908][12]);
        let eq25_e1478_d_n13: f64 = (eq25_e1476 * s.dn[908][13]);
        (eq25_e1478, eq25_e1478_d_n0, eq25_e1478_d_n1, eq25_e1478_d_n2, eq25_e1478_d_n3, eq25_e1478_d_n4, eq25_e1478_d_n5, eq25_e1478_d_n6, eq25_e1478_d_n7, eq25_e1478_d_n8, eq25_e1478_d_n9, eq25_e1478_d_n10, eq25_e1478_d_n11, eq25_e1478_d_n12, eq25_e1478_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e1480;
        let eq25_node_derivatives: [f64; 14] = [eq25_e1480_d_n0, eq25_e1480_d_n1, eq25_e1480_d_n2, eq25_e1480_d_n3, eq25_e1480_d_n4, eq25_e1480_d_n5, eq25_e1480_d_n6, eq25_e1480_d_n7, eq25_e1480_d_n8, eq25_e1480_d_n9, eq25_e1480_d_n10, eq25_e1480_d_n11, eq25_e1480_d_n12, eq25_e1480_d_n13];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            self.multiplicity * (eq25_value),
            &nodes,
            &eq25_node_derivatives,
            &branches,
            &eq25_branch_derivatives,
            self.multiplicity,
        );
    }
}
