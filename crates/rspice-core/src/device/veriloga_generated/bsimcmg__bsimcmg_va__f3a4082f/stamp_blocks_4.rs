#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq7_e1998,) = {
    if (!(s.v[1697] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e1998;
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
        let (eq8_e2004, eq8_e2004_d_n0, eq8_e2004_d_n1, eq8_e2004_d_n2, eq8_e2004_d_n3, eq8_e2004_d_n4, eq8_e2004_d_n5, eq8_e2004_d_n6, eq8_e2004_d_n7, eq8_e2004_d_n8, eq8_e2004_d_n9, eq8_e2004_d_n10, eq8_e2004_d_n11, eq8_e2004_d_n12, eq8_e2004_d_n13, eq8_e2004_d_n14, eq8_e2004_d_n15, eq8_e2004_d_n16,) = {
    if (s.v[1698] != 0.0) {
        let eq8_e2002: f64 = (s.v[114] * s.v[556]);
        let eq8_e2002_d_n0: f64 = ((s.dn[114][0] * s.v[556]) + (s.v[114] * s.dn[556][0]));
        let eq8_e2002_d_n1: f64 = ((s.dn[114][1] * s.v[556]) + (s.v[114] * s.dn[556][1]));
        let eq8_e2002_d_n2: f64 = ((s.dn[114][2] * s.v[556]) + (s.v[114] * s.dn[556][2]));
        let eq8_e2002_d_n3: f64 = ((s.dn[114][3] * s.v[556]) + (s.v[114] * s.dn[556][3]));
        let eq8_e2002_d_n4: f64 = ((s.dn[114][4] * s.v[556]) + (s.v[114] * s.dn[556][4]));
        let eq8_e2002_d_n5: f64 = ((s.dn[114][5] * s.v[556]) + (s.v[114] * s.dn[556][5]));
        let eq8_e2002_d_n6: f64 = ((s.dn[114][6] * s.v[556]) + (s.v[114] * s.dn[556][6]));
        let eq8_e2002_d_n7: f64 = ((s.dn[114][7] * s.v[556]) + (s.v[114] * s.dn[556][7]));
        let eq8_e2002_d_n8: f64 = ((s.dn[114][8] * s.v[556]) + (s.v[114] * s.dn[556][8]));
        let eq8_e2002_d_n9: f64 = ((s.dn[114][9] * s.v[556]) + (s.v[114] * s.dn[556][9]));
        let eq8_e2002_d_n10: f64 = ((s.dn[114][10] * s.v[556]) + (s.v[114] * s.dn[556][10]));
        let eq8_e2002_d_n11: f64 = ((s.dn[114][11] * s.v[556]) + (s.v[114] * s.dn[556][11]));
        let eq8_e2002_d_n12: f64 = ((s.dn[114][12] * s.v[556]) + (s.v[114] * s.dn[556][12]));
        let eq8_e2002_d_n13: f64 = ((s.dn[114][13] * s.v[556]) + (s.v[114] * s.dn[556][13]));
        let eq8_e2002_d_n14: f64 = ((s.dn[114][14] * s.v[556]) + (s.v[114] * s.dn[556][14]));
        let eq8_e2002_d_n15: f64 = ((s.dn[114][15] * s.v[556]) + (s.v[114] * s.dn[556][15]));
        let eq8_e2002_d_n16: f64 = ((s.dn[114][16] * s.v[556]) + (s.v[114] * s.dn[556][16]));
        (eq8_e2002, eq8_e2002_d_n0, eq8_e2002_d_n1, eq8_e2002_d_n2, eq8_e2002_d_n3, eq8_e2002_d_n4, eq8_e2002_d_n5, eq8_e2002_d_n6, eq8_e2002_d_n7, eq8_e2002_d_n8, eq8_e2002_d_n9, eq8_e2002_d_n10, eq8_e2002_d_n11, eq8_e2002_d_n12, eq8_e2002_d_n13, eq8_e2002_d_n14, eq8_e2002_d_n15, eq8_e2002_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e2004;
        let eq8_node_derivatives: [f64; 17] = [eq8_e2004_d_n0, eq8_e2004_d_n1, eq8_e2004_d_n2, eq8_e2004_d_n3, eq8_e2004_d_n4, eq8_e2004_d_n5, eq8_e2004_d_n6, eq8_e2004_d_n7, eq8_e2004_d_n8, eq8_e2004_d_n9, eq8_e2004_d_n10, eq8_e2004_d_n11, eq8_e2004_d_n12, eq8_e2004_d_n13, eq8_e2004_d_n14, eq8_e2004_d_n15, eq8_e2004_d_n16];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let (eq9_e2012, eq9_e2012_d_n0, eq9_e2012_d_n1, eq9_e2012_d_n2, eq9_e2012_d_n3, eq9_e2012_d_n4, eq9_e2012_d_n5, eq9_e2012_d_n6, eq9_e2012_d_n7, eq9_e2012_d_n8, eq9_e2012_d_n9, eq9_e2012_d_n10, eq9_e2012_d_n11, eq9_e2012_d_n12, eq9_e2012_d_n13, eq9_e2012_d_n14, eq9_e2012_d_n15, eq9_e2012_d_n16,) = {
    if (s.v[1698] != 0.0) {
        let eq9_e2009: f64 = (s.v[470] + s.v[480]);
        let eq9_e2009_d_n0: f64 = (s.dn[470][0] + s.dn[480][0]);
        let eq9_e2009_d_n1: f64 = (s.dn[470][1] + s.dn[480][1]);
        let eq9_e2009_d_n2: f64 = (s.dn[470][2] + s.dn[480][2]);
        let eq9_e2009_d_n3: f64 = (s.dn[470][3] + s.dn[480][3]);
        let eq9_e2009_d_n4: f64 = (s.dn[470][4] + s.dn[480][4]);
        let eq9_e2009_d_n5: f64 = (s.dn[470][5] + s.dn[480][5]);
        let eq9_e2009_d_n6: f64 = (s.dn[470][6] + s.dn[480][6]);
        let eq9_e2009_d_n7: f64 = (s.dn[470][7] + s.dn[480][7]);
        let eq9_e2009_d_n8: f64 = (s.dn[470][8] + s.dn[480][8]);
        let eq9_e2009_d_n9: f64 = (s.dn[470][9] + s.dn[480][9]);
        let eq9_e2009_d_n10: f64 = (s.dn[470][10] + s.dn[480][10]);
        let eq9_e2009_d_n11: f64 = (s.dn[470][11] + s.dn[480][11]);
        let eq9_e2009_d_n12: f64 = (s.dn[470][12] + s.dn[480][12]);
        let eq9_e2009_d_n13: f64 = (s.dn[470][13] + s.dn[480][13]);
        let eq9_e2009_d_n14: f64 = (s.dn[470][14] + s.dn[480][14]);
        let eq9_e2009_d_n15: f64 = (s.dn[470][15] + s.dn[480][15]);
        let eq9_e2009_d_n16: f64 = (s.dn[470][16] + s.dn[480][16]);
        let eq9_e2010: f64 = (s.v[114] * eq9_e2009);
        let eq9_e2010_d_n0: f64 = ((s.dn[114][0] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n0));
        let eq9_e2010_d_n1: f64 = ((s.dn[114][1] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n1));
        let eq9_e2010_d_n2: f64 = ((s.dn[114][2] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n2));
        let eq9_e2010_d_n3: f64 = ((s.dn[114][3] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n3));
        let eq9_e2010_d_n4: f64 = ((s.dn[114][4] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n4));
        let eq9_e2010_d_n5: f64 = ((s.dn[114][5] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n5));
        let eq9_e2010_d_n6: f64 = ((s.dn[114][6] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n6));
        let eq9_e2010_d_n7: f64 = ((s.dn[114][7] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n7));
        let eq9_e2010_d_n8: f64 = ((s.dn[114][8] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n8));
        let eq9_e2010_d_n9: f64 = ((s.dn[114][9] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n9));
        let eq9_e2010_d_n10: f64 = ((s.dn[114][10] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n10));
        let eq9_e2010_d_n11: f64 = ((s.dn[114][11] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n11));
        let eq9_e2010_d_n12: f64 = ((s.dn[114][12] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n12));
        let eq9_e2010_d_n13: f64 = ((s.dn[114][13] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n13));
        let eq9_e2010_d_n14: f64 = ((s.dn[114][14] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n14));
        let eq9_e2010_d_n15: f64 = ((s.dn[114][15] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n15));
        let eq9_e2010_d_n16: f64 = ((s.dn[114][16] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n16));
        (eq9_e2010, eq9_e2010_d_n0, eq9_e2010_d_n1, eq9_e2010_d_n2, eq9_e2010_d_n3, eq9_e2010_d_n4, eq9_e2010_d_n5, eq9_e2010_d_n6, eq9_e2010_d_n7, eq9_e2010_d_n8, eq9_e2010_d_n9, eq9_e2010_d_n10, eq9_e2010_d_n11, eq9_e2010_d_n12, eq9_e2010_d_n13, eq9_e2010_d_n14, eq9_e2010_d_n15, eq9_e2010_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e2012;
        let eq9_node_derivatives: [f64; 17] = [eq9_e2012_d_n0, eq9_e2012_d_n1, eq9_e2012_d_n2, eq9_e2012_d_n3, eq9_e2012_d_n4, eq9_e2012_d_n5, eq9_e2012_d_n6, eq9_e2012_d_n7, eq9_e2012_d_n8, eq9_e2012_d_n9, eq9_e2012_d_n10, eq9_e2012_d_n11, eq9_e2012_d_n12, eq9_e2012_d_n13, eq9_e2012_d_n14, eq9_e2012_d_n15, eq9_e2012_d_n16];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq9_value),
            &nodes,
            &eq9_node_derivatives,
            &branches,
            &eq9_branch_derivatives,
            self.multiplicity,
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
        let (eq10_e2020, eq10_e2020_d_n0, eq10_e2020_d_n1, eq10_e2020_d_n2, eq10_e2020_d_n3, eq10_e2020_d_n4, eq10_e2020_d_n5, eq10_e2020_d_n6, eq10_e2020_d_n7, eq10_e2020_d_n8, eq10_e2020_d_n9, eq10_e2020_d_n10, eq10_e2020_d_n11, eq10_e2020_d_n12, eq10_e2020_d_n13, eq10_e2020_d_n14, eq10_e2020_d_n15, eq10_e2020_d_n16,) = {
    if (s.v[1698] != 0.0) {
        let eq10_e2017: f64 = (s.v[471] + s.v[481]);
        let eq10_e2017_d_n0: f64 = (s.dn[471][0] + s.dn[481][0]);
        let eq10_e2017_d_n1: f64 = (s.dn[471][1] + s.dn[481][1]);
        let eq10_e2017_d_n2: f64 = (s.dn[471][2] + s.dn[481][2]);
        let eq10_e2017_d_n3: f64 = (s.dn[471][3] + s.dn[481][3]);
        let eq10_e2017_d_n4: f64 = (s.dn[471][4] + s.dn[481][4]);
        let eq10_e2017_d_n5: f64 = (s.dn[471][5] + s.dn[481][5]);
        let eq10_e2017_d_n6: f64 = (s.dn[471][6] + s.dn[481][6]);
        let eq10_e2017_d_n7: f64 = (s.dn[471][7] + s.dn[481][7]);
        let eq10_e2017_d_n8: f64 = (s.dn[471][8] + s.dn[481][8]);
        let eq10_e2017_d_n9: f64 = (s.dn[471][9] + s.dn[481][9]);
        let eq10_e2017_d_n10: f64 = (s.dn[471][10] + s.dn[481][10]);
        let eq10_e2017_d_n11: f64 = (s.dn[471][11] + s.dn[481][11]);
        let eq10_e2017_d_n12: f64 = (s.dn[471][12] + s.dn[481][12]);
        let eq10_e2017_d_n13: f64 = (s.dn[471][13] + s.dn[481][13]);
        let eq10_e2017_d_n14: f64 = (s.dn[471][14] + s.dn[481][14]);
        let eq10_e2017_d_n15: f64 = (s.dn[471][15] + s.dn[481][15]);
        let eq10_e2017_d_n16: f64 = (s.dn[471][16] + s.dn[481][16]);
        let eq10_e2018: f64 = (s.v[114] * eq10_e2017);
        let eq10_e2018_d_n0: f64 = ((s.dn[114][0] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n0));
        let eq10_e2018_d_n1: f64 = ((s.dn[114][1] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n1));
        let eq10_e2018_d_n2: f64 = ((s.dn[114][2] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n2));
        let eq10_e2018_d_n3: f64 = ((s.dn[114][3] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n3));
        let eq10_e2018_d_n4: f64 = ((s.dn[114][4] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n4));
        let eq10_e2018_d_n5: f64 = ((s.dn[114][5] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n5));
        let eq10_e2018_d_n6: f64 = ((s.dn[114][6] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n6));
        let eq10_e2018_d_n7: f64 = ((s.dn[114][7] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n7));
        let eq10_e2018_d_n8: f64 = ((s.dn[114][8] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n8));
        let eq10_e2018_d_n9: f64 = ((s.dn[114][9] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n9));
        let eq10_e2018_d_n10: f64 = ((s.dn[114][10] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n10));
        let eq10_e2018_d_n11: f64 = ((s.dn[114][11] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n11));
        let eq10_e2018_d_n12: f64 = ((s.dn[114][12] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n12));
        let eq10_e2018_d_n13: f64 = ((s.dn[114][13] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n13));
        let eq10_e2018_d_n14: f64 = ((s.dn[114][14] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n14));
        let eq10_e2018_d_n15: f64 = ((s.dn[114][15] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n15));
        let eq10_e2018_d_n16: f64 = ((s.dn[114][16] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n16));
        (eq10_e2018, eq10_e2018_d_n0, eq10_e2018_d_n1, eq10_e2018_d_n2, eq10_e2018_d_n3, eq10_e2018_d_n4, eq10_e2018_d_n5, eq10_e2018_d_n6, eq10_e2018_d_n7, eq10_e2018_d_n8, eq10_e2018_d_n9, eq10_e2018_d_n10, eq10_e2018_d_n11, eq10_e2018_d_n12, eq10_e2018_d_n13, eq10_e2018_d_n14, eq10_e2018_d_n15, eq10_e2018_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e2020;
        let eq10_node_derivatives: [f64; 17] = [eq10_e2020_d_n0, eq10_e2020_d_n1, eq10_e2020_d_n2, eq10_e2020_d_n3, eq10_e2020_d_n4, eq10_e2020_d_n5, eq10_e2020_d_n6, eq10_e2020_d_n7, eq10_e2020_d_n8, eq10_e2020_d_n9, eq10_e2020_d_n10, eq10_e2020_d_n11, eq10_e2020_d_n12, eq10_e2020_d_n13, eq10_e2020_d_n14, eq10_e2020_d_n15, eq10_e2020_d_n16];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[5]),
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
        let (eq11_e2032, eq11_e2032_d_n0, eq11_e2032_d_n1, eq11_e2032_d_n2, eq11_e2032_d_n3, eq11_e2032_d_n4, eq11_e2032_d_n5, eq11_e2032_d_n6, eq11_e2032_d_n7, eq11_e2032_d_n8, eq11_e2032_d_n9, eq11_e2032_d_n10, eq11_e2032_d_n11, eq11_e2032_d_n12, eq11_e2032_d_n13, eq11_e2032_d_n14, eq11_e2032_d_n15, eq11_e2032_d_n16,) = {
    if (((s.v[1698] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) {
        let eq11_e2029: f64 = (s.v[476] + s.v[488]);
        let eq11_e2029_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq11_e2029_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq11_e2029_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq11_e2029_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq11_e2029_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq11_e2029_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq11_e2029_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq11_e2029_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq11_e2029_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq11_e2029_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq11_e2029_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq11_e2029_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq11_e2029_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq11_e2029_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq11_e2029_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq11_e2029_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq11_e2029_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq11_e2030: f64 = (s.v[114] * eq11_e2029);
        let eq11_e2030_d_n0: f64 = ((s.dn[114][0] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n0));
        let eq11_e2030_d_n1: f64 = ((s.dn[114][1] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n1));
        let eq11_e2030_d_n2: f64 = ((s.dn[114][2] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n2));
        let eq11_e2030_d_n3: f64 = ((s.dn[114][3] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n3));
        let eq11_e2030_d_n4: f64 = ((s.dn[114][4] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n4));
        let eq11_e2030_d_n5: f64 = ((s.dn[114][5] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n5));
        let eq11_e2030_d_n6: f64 = ((s.dn[114][6] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n6));
        let eq11_e2030_d_n7: f64 = ((s.dn[114][7] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n7));
        let eq11_e2030_d_n8: f64 = ((s.dn[114][8] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n8));
        let eq11_e2030_d_n9: f64 = ((s.dn[114][9] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n9));
        let eq11_e2030_d_n10: f64 = ((s.dn[114][10] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n10));
        let eq11_e2030_d_n11: f64 = ((s.dn[114][11] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n11));
        let eq11_e2030_d_n12: f64 = ((s.dn[114][12] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n12));
        let eq11_e2030_d_n13: f64 = ((s.dn[114][13] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n13));
        let eq11_e2030_d_n14: f64 = ((s.dn[114][14] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n14));
        let eq11_e2030_d_n15: f64 = ((s.dn[114][15] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n15));
        let eq11_e2030_d_n16: f64 = ((s.dn[114][16] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n16));
        (eq11_e2030, eq11_e2030_d_n0, eq11_e2030_d_n1, eq11_e2030_d_n2, eq11_e2030_d_n3, eq11_e2030_d_n4, eq11_e2030_d_n5, eq11_e2030_d_n6, eq11_e2030_d_n7, eq11_e2030_d_n8, eq11_e2030_d_n9, eq11_e2030_d_n10, eq11_e2030_d_n11, eq11_e2030_d_n12, eq11_e2030_d_n13, eq11_e2030_d_n14, eq11_e2030_d_n15, eq11_e2030_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e2032;
        let eq11_node_derivatives: [f64; 17] = [eq11_e2032_d_n0, eq11_e2032_d_n1, eq11_e2032_d_n2, eq11_e2032_d_n3, eq11_e2032_d_n4, eq11_e2032_d_n5, eq11_e2032_d_n6, eq11_e2032_d_n7, eq11_e2032_d_n8, eq11_e2032_d_n9, eq11_e2032_d_n10, eq11_e2032_d_n11, eq11_e2032_d_n12, eq11_e2032_d_n13, eq11_e2032_d_n14, eq11_e2032_d_n15, eq11_e2032_d_n16];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let (eq12_e2042, eq12_e2042_d_n0, eq12_e2042_d_n1, eq12_e2042_d_n2, eq12_e2042_d_n3, eq12_e2042_d_n4, eq12_e2042_d_n5, eq12_e2042_d_n6, eq12_e2042_d_n7, eq12_e2042_d_n8, eq12_e2042_d_n9, eq12_e2042_d_n10, eq12_e2042_d_n11, eq12_e2042_d_n12, eq12_e2042_d_n13, eq12_e2042_d_n14, eq12_e2042_d_n15, eq12_e2042_d_n16,) = {
    if (((s.v[1698] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) {
        let eq12_e2040: f64 = (s.v[114] * s.v[475]);
        let eq12_e2040_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq12_e2040_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq12_e2040_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq12_e2040_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq12_e2040_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq12_e2040_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq12_e2040_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq12_e2040_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq12_e2040_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq12_e2040_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq12_e2040_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq12_e2040_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq12_e2040_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq12_e2040_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq12_e2040_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq12_e2040_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq12_e2040_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        (eq12_e2040, eq12_e2040_d_n0, eq12_e2040_d_n1, eq12_e2040_d_n2, eq12_e2040_d_n3, eq12_e2040_d_n4, eq12_e2040_d_n5, eq12_e2040_d_n6, eq12_e2040_d_n7, eq12_e2040_d_n8, eq12_e2040_d_n9, eq12_e2040_d_n10, eq12_e2040_d_n11, eq12_e2040_d_n12, eq12_e2040_d_n13, eq12_e2040_d_n14, eq12_e2040_d_n15, eq12_e2040_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e2042;
        let eq12_node_derivatives: [f64; 17] = [eq12_e2042_d_n0, eq12_e2042_d_n1, eq12_e2042_d_n2, eq12_e2042_d_n3, eq12_e2042_d_n4, eq12_e2042_d_n5, eq12_e2042_d_n6, eq12_e2042_d_n7, eq12_e2042_d_n8, eq12_e2042_d_n9, eq12_e2042_d_n10, eq12_e2042_d_n11, eq12_e2042_d_n12, eq12_e2042_d_n13, eq12_e2042_d_n14, eq12_e2042_d_n15, eq12_e2042_d_n16];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let (eq13_e2052, eq13_e2052_d_n0, eq13_e2052_d_n1, eq13_e2052_d_n2, eq13_e2052_d_n3, eq13_e2052_d_n4, eq13_e2052_d_n5, eq13_e2052_d_n6, eq13_e2052_d_n7, eq13_e2052_d_n8, eq13_e2052_d_n9, eq13_e2052_d_n10, eq13_e2052_d_n11, eq13_e2052_d_n12, eq13_e2052_d_n13, eq13_e2052_d_n14, eq13_e2052_d_n15, eq13_e2052_d_n16,) = {
    if (((s.v[1698] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) {
        let eq13_e2050: f64 = (s.v[114] * s.v[478]);
        let eq13_e2050_d_n0: f64 = ((s.dn[114][0] * s.v[478]) + (s.v[114] * s.dn[478][0]));
        let eq13_e2050_d_n1: f64 = ((s.dn[114][1] * s.v[478]) + (s.v[114] * s.dn[478][1]));
        let eq13_e2050_d_n2: f64 = ((s.dn[114][2] * s.v[478]) + (s.v[114] * s.dn[478][2]));
        let eq13_e2050_d_n3: f64 = ((s.dn[114][3] * s.v[478]) + (s.v[114] * s.dn[478][3]));
        let eq13_e2050_d_n4: f64 = ((s.dn[114][4] * s.v[478]) + (s.v[114] * s.dn[478][4]));
        let eq13_e2050_d_n5: f64 = ((s.dn[114][5] * s.v[478]) + (s.v[114] * s.dn[478][5]));
        let eq13_e2050_d_n6: f64 = ((s.dn[114][6] * s.v[478]) + (s.v[114] * s.dn[478][6]));
        let eq13_e2050_d_n7: f64 = ((s.dn[114][7] * s.v[478]) + (s.v[114] * s.dn[478][7]));
        let eq13_e2050_d_n8: f64 = ((s.dn[114][8] * s.v[478]) + (s.v[114] * s.dn[478][8]));
        let eq13_e2050_d_n9: f64 = ((s.dn[114][9] * s.v[478]) + (s.v[114] * s.dn[478][9]));
        let eq13_e2050_d_n10: f64 = ((s.dn[114][10] * s.v[478]) + (s.v[114] * s.dn[478][10]));
        let eq13_e2050_d_n11: f64 = ((s.dn[114][11] * s.v[478]) + (s.v[114] * s.dn[478][11]));
        let eq13_e2050_d_n12: f64 = ((s.dn[114][12] * s.v[478]) + (s.v[114] * s.dn[478][12]));
        let eq13_e2050_d_n13: f64 = ((s.dn[114][13] * s.v[478]) + (s.v[114] * s.dn[478][13]));
        let eq13_e2050_d_n14: f64 = ((s.dn[114][14] * s.v[478]) + (s.v[114] * s.dn[478][14]));
        let eq13_e2050_d_n15: f64 = ((s.dn[114][15] * s.v[478]) + (s.v[114] * s.dn[478][15]));
        let eq13_e2050_d_n16: f64 = ((s.dn[114][16] * s.v[478]) + (s.v[114] * s.dn[478][16]));
        (eq13_e2050, eq13_e2050_d_n0, eq13_e2050_d_n1, eq13_e2050_d_n2, eq13_e2050_d_n3, eq13_e2050_d_n4, eq13_e2050_d_n5, eq13_e2050_d_n6, eq13_e2050_d_n7, eq13_e2050_d_n8, eq13_e2050_d_n9, eq13_e2050_d_n10, eq13_e2050_d_n11, eq13_e2050_d_n12, eq13_e2050_d_n13, eq13_e2050_d_n14, eq13_e2050_d_n15, eq13_e2050_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e2052;
        let eq13_node_derivatives: [f64; 17] = [eq13_e2052_d_n0, eq13_e2052_d_n1, eq13_e2052_d_n2, eq13_e2052_d_n3, eq13_e2052_d_n4, eq13_e2052_d_n5, eq13_e2052_d_n6, eq13_e2052_d_n7, eq13_e2052_d_n8, eq13_e2052_d_n9, eq13_e2052_d_n10, eq13_e2052_d_n11, eq13_e2052_d_n12, eq13_e2052_d_n13, eq13_e2052_d_n14, eq13_e2052_d_n15, eq13_e2052_d_n16];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[3]),
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
        let (eq14_e2062, eq14_e2062_d_n0, eq14_e2062_d_n1, eq14_e2062_d_n2, eq14_e2062_d_n3, eq14_e2062_d_n4, eq14_e2062_d_n5, eq14_e2062_d_n6, eq14_e2062_d_n7, eq14_e2062_d_n8, eq14_e2062_d_n9, eq14_e2062_d_n10, eq14_e2062_d_n11, eq14_e2062_d_n12, eq14_e2062_d_n13, eq14_e2062_d_n14, eq14_e2062_d_n15, eq14_e2062_d_n16,) = {
    if (((s.v[1698] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) {
        let eq14_e2060: f64 = (s.v[114] * s.v[477]);
        let eq14_e2060_d_n0: f64 = ((s.dn[114][0] * s.v[477]) + (s.v[114] * s.dn[477][0]));
        let eq14_e2060_d_n1: f64 = ((s.dn[114][1] * s.v[477]) + (s.v[114] * s.dn[477][1]));
        let eq14_e2060_d_n2: f64 = ((s.dn[114][2] * s.v[477]) + (s.v[114] * s.dn[477][2]));
        let eq14_e2060_d_n3: f64 = ((s.dn[114][3] * s.v[477]) + (s.v[114] * s.dn[477][3]));
        let eq14_e2060_d_n4: f64 = ((s.dn[114][4] * s.v[477]) + (s.v[114] * s.dn[477][4]));
        let eq14_e2060_d_n5: f64 = ((s.dn[114][5] * s.v[477]) + (s.v[114] * s.dn[477][5]));
        let eq14_e2060_d_n6: f64 = ((s.dn[114][6] * s.v[477]) + (s.v[114] * s.dn[477][6]));
        let eq14_e2060_d_n7: f64 = ((s.dn[114][7] * s.v[477]) + (s.v[114] * s.dn[477][7]));
        let eq14_e2060_d_n8: f64 = ((s.dn[114][8] * s.v[477]) + (s.v[114] * s.dn[477][8]));
        let eq14_e2060_d_n9: f64 = ((s.dn[114][9] * s.v[477]) + (s.v[114] * s.dn[477][9]));
        let eq14_e2060_d_n10: f64 = ((s.dn[114][10] * s.v[477]) + (s.v[114] * s.dn[477][10]));
        let eq14_e2060_d_n11: f64 = ((s.dn[114][11] * s.v[477]) + (s.v[114] * s.dn[477][11]));
        let eq14_e2060_d_n12: f64 = ((s.dn[114][12] * s.v[477]) + (s.v[114] * s.dn[477][12]));
        let eq14_e2060_d_n13: f64 = ((s.dn[114][13] * s.v[477]) + (s.v[114] * s.dn[477][13]));
        let eq14_e2060_d_n14: f64 = ((s.dn[114][14] * s.v[477]) + (s.v[114] * s.dn[477][14]));
        let eq14_e2060_d_n15: f64 = ((s.dn[114][15] * s.v[477]) + (s.v[114] * s.dn[477][15]));
        let eq14_e2060_d_n16: f64 = ((s.dn[114][16] * s.v[477]) + (s.v[114] * s.dn[477][16]));
        (eq14_e2060, eq14_e2060_d_n0, eq14_e2060_d_n1, eq14_e2060_d_n2, eq14_e2060_d_n3, eq14_e2060_d_n4, eq14_e2060_d_n5, eq14_e2060_d_n6, eq14_e2060_d_n7, eq14_e2060_d_n8, eq14_e2060_d_n9, eq14_e2060_d_n10, eq14_e2060_d_n11, eq14_e2060_d_n12, eq14_e2060_d_n13, eq14_e2060_d_n14, eq14_e2060_d_n15, eq14_e2060_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e2062;
        let eq14_node_derivatives: [f64; 17] = [eq14_e2062_d_n0, eq14_e2062_d_n1, eq14_e2062_d_n2, eq14_e2062_d_n3, eq14_e2062_d_n4, eq14_e2062_d_n5, eq14_e2062_d_n6, eq14_e2062_d_n7, eq14_e2062_d_n8, eq14_e2062_d_n9, eq14_e2062_d_n10, eq14_e2062_d_n11, eq14_e2062_d_n12, eq14_e2062_d_n13, eq14_e2062_d_n14, eq14_e2062_d_n15, eq14_e2062_d_n16];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
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
        let (eq15_e2075, eq15_e2075_d_n0, eq15_e2075_d_n1, eq15_e2075_d_n2, eq15_e2075_d_n3, eq15_e2075_d_n4, eq15_e2075_d_n5, eq15_e2075_d_n6, eq15_e2075_d_n7, eq15_e2075_d_n8, eq15_e2075_d_n9, eq15_e2075_d_n10, eq15_e2075_d_n11, eq15_e2075_d_n12, eq15_e2075_d_n13, eq15_e2075_d_n14, eq15_e2075_d_n15, eq15_e2075_d_n16,) = {
    if (((s.v[1698] != 0.0) && (s.v[1699] != 0.0)) && (!(s.v[1700] != 0.0))) {
        let eq15_e2072: f64 = (s.v[476] + s.v[488]);
        let eq15_e2072_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq15_e2072_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq15_e2072_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq15_e2072_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq15_e2072_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq15_e2072_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq15_e2072_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq15_e2072_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq15_e2072_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq15_e2072_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq15_e2072_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq15_e2072_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq15_e2072_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq15_e2072_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq15_e2072_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq15_e2072_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq15_e2072_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq15_e2073: f64 = (s.v[114] * eq15_e2072);
        let eq15_e2073_d_n0: f64 = ((s.dn[114][0] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n0));
        let eq15_e2073_d_n1: f64 = ((s.dn[114][1] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n1));
        let eq15_e2073_d_n2: f64 = ((s.dn[114][2] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n2));
        let eq15_e2073_d_n3: f64 = ((s.dn[114][3] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n3));
        let eq15_e2073_d_n4: f64 = ((s.dn[114][4] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n4));
        let eq15_e2073_d_n5: f64 = ((s.dn[114][5] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n5));
        let eq15_e2073_d_n6: f64 = ((s.dn[114][6] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n6));
        let eq15_e2073_d_n7: f64 = ((s.dn[114][7] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n7));
        let eq15_e2073_d_n8: f64 = ((s.dn[114][8] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n8));
        let eq15_e2073_d_n9: f64 = ((s.dn[114][9] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n9));
        let eq15_e2073_d_n10: f64 = ((s.dn[114][10] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n10));
        let eq15_e2073_d_n11: f64 = ((s.dn[114][11] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n11));
        let eq15_e2073_d_n12: f64 = ((s.dn[114][12] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n12));
        let eq15_e2073_d_n13: f64 = ((s.dn[114][13] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n13));
        let eq15_e2073_d_n14: f64 = ((s.dn[114][14] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n14));
        let eq15_e2073_d_n15: f64 = ((s.dn[114][15] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n15));
        let eq15_e2073_d_n16: f64 = ((s.dn[114][16] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n16));
        (eq15_e2073, eq15_e2073_d_n0, eq15_e2073_d_n1, eq15_e2073_d_n2, eq15_e2073_d_n3, eq15_e2073_d_n4, eq15_e2073_d_n5, eq15_e2073_d_n6, eq15_e2073_d_n7, eq15_e2073_d_n8, eq15_e2073_d_n9, eq15_e2073_d_n10, eq15_e2073_d_n11, eq15_e2073_d_n12, eq15_e2073_d_n13, eq15_e2073_d_n14, eq15_e2073_d_n15, eq15_e2073_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e2075;
        let eq15_node_derivatives: [f64; 17] = [eq15_e2075_d_n0, eq15_e2075_d_n1, eq15_e2075_d_n2, eq15_e2075_d_n3, eq15_e2075_d_n4, eq15_e2075_d_n5, eq15_e2075_d_n6, eq15_e2075_d_n7, eq15_e2075_d_n8, eq15_e2075_d_n9, eq15_e2075_d_n10, eq15_e2075_d_n11, eq15_e2075_d_n12, eq15_e2075_d_n13, eq15_e2075_d_n14, eq15_e2075_d_n15, eq15_e2075_d_n16];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[3]),
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
        let (eq16_e2086, eq16_e2086_d_n0, eq16_e2086_d_n1, eq16_e2086_d_n2, eq16_e2086_d_n3, eq16_e2086_d_n4, eq16_e2086_d_n5, eq16_e2086_d_n6, eq16_e2086_d_n7, eq16_e2086_d_n8, eq16_e2086_d_n9, eq16_e2086_d_n10, eq16_e2086_d_n11, eq16_e2086_d_n12, eq16_e2086_d_n13, eq16_e2086_d_n14, eq16_e2086_d_n15, eq16_e2086_d_n16,) = {
    if (((s.v[1698] != 0.0) && (s.v[1699] != 0.0)) && (!(s.v[1700] != 0.0))) {
        let eq16_e2084: f64 = (s.v[114] * s.v[475]);
        let eq16_e2084_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq16_e2084_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq16_e2084_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq16_e2084_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq16_e2084_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq16_e2084_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq16_e2084_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq16_e2084_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq16_e2084_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq16_e2084_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq16_e2084_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq16_e2084_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq16_e2084_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq16_e2084_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq16_e2084_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq16_e2084_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq16_e2084_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        (eq16_e2084, eq16_e2084_d_n0, eq16_e2084_d_n1, eq16_e2084_d_n2, eq16_e2084_d_n3, eq16_e2084_d_n4, eq16_e2084_d_n5, eq16_e2084_d_n6, eq16_e2084_d_n7, eq16_e2084_d_n8, eq16_e2084_d_n9, eq16_e2084_d_n10, eq16_e2084_d_n11, eq16_e2084_d_n12, eq16_e2084_d_n13, eq16_e2084_d_n14, eq16_e2084_d_n15, eq16_e2084_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e2086;
        let eq16_node_derivatives: [f64; 17] = [eq16_e2086_d_n0, eq16_e2086_d_n1, eq16_e2086_d_n2, eq16_e2086_d_n3, eq16_e2086_d_n4, eq16_e2086_d_n5, eq16_e2086_d_n6, eq16_e2086_d_n7, eq16_e2086_d_n8, eq16_e2086_d_n9, eq16_e2086_d_n10, eq16_e2086_d_n11, eq16_e2086_d_n12, eq16_e2086_d_n13, eq16_e2086_d_n14, eq16_e2086_d_n15, eq16_e2086_d_n16];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            self.multiplicity * (eq16_value),
            &nodes,
            &eq16_node_derivatives,
            &branches,
            &eq16_branch_derivatives,
            self.multiplicity,
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
        let (eq17_e2096, eq17_e2096_d_n0, eq17_e2096_d_n1, eq17_e2096_d_n2, eq17_e2096_d_n3, eq17_e2096_d_n4, eq17_e2096_d_n5, eq17_e2096_d_n6, eq17_e2096_d_n7, eq17_e2096_d_n8, eq17_e2096_d_n9, eq17_e2096_d_n10, eq17_e2096_d_n11, eq17_e2096_d_n12, eq17_e2096_d_n13, eq17_e2096_d_n14, eq17_e2096_d_n15, eq17_e2096_d_n16,) = {
    if ((s.v[1698] != 0.0) && (s.v[1699] != 0.0)) {
        let eq17_e2093: f64 = (s.v[461] + s.v[469]);
        let eq17_e2093_d_n0: f64 = (s.dn[461][0] + s.dn[469][0]);
        let eq17_e2093_d_n1: f64 = (s.dn[461][1] + s.dn[469][1]);
        let eq17_e2093_d_n2: f64 = (s.dn[461][2] + s.dn[469][2]);
        let eq17_e2093_d_n3: f64 = (s.dn[461][3] + s.dn[469][3]);
        let eq17_e2093_d_n4: f64 = (s.dn[461][4] + s.dn[469][4]);
        let eq17_e2093_d_n5: f64 = (s.dn[461][5] + s.dn[469][5]);
        let eq17_e2093_d_n6: f64 = (s.dn[461][6] + s.dn[469][6]);
        let eq17_e2093_d_n7: f64 = (s.dn[461][7] + s.dn[469][7]);
        let eq17_e2093_d_n8: f64 = (s.dn[461][8] + s.dn[469][8]);
        let eq17_e2093_d_n9: f64 = (s.dn[461][9] + s.dn[469][9]);
        let eq17_e2093_d_n10: f64 = (s.dn[461][10] + s.dn[469][10]);
        let eq17_e2093_d_n11: f64 = (s.dn[461][11] + s.dn[469][11]);
        let eq17_e2093_d_n12: f64 = (s.dn[461][12] + s.dn[469][12]);
        let eq17_e2093_d_n13: f64 = (s.dn[461][13] + s.dn[469][13]);
        let eq17_e2093_d_n14: f64 = (s.dn[461][14] + s.dn[469][14]);
        let eq17_e2093_d_n15: f64 = (s.dn[461][15] + s.dn[469][15]);
        let eq17_e2093_d_n16: f64 = (s.dn[461][16] + s.dn[469][16]);
        let eq17_e2094: f64 = (s.v[114] * eq17_e2093);
        let eq17_e2094_d_n0: f64 = ((s.dn[114][0] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n0));
        let eq17_e2094_d_n1: f64 = ((s.dn[114][1] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n1));
        let eq17_e2094_d_n2: f64 = ((s.dn[114][2] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n2));
        let eq17_e2094_d_n3: f64 = ((s.dn[114][3] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n3));
        let eq17_e2094_d_n4: f64 = ((s.dn[114][4] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n4));
        let eq17_e2094_d_n5: f64 = ((s.dn[114][5] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n5));
        let eq17_e2094_d_n6: f64 = ((s.dn[114][6] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n6));
        let eq17_e2094_d_n7: f64 = ((s.dn[114][7] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n7));
        let eq17_e2094_d_n8: f64 = ((s.dn[114][8] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n8));
        let eq17_e2094_d_n9: f64 = ((s.dn[114][9] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n9));
        let eq17_e2094_d_n10: f64 = ((s.dn[114][10] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n10));
        let eq17_e2094_d_n11: f64 = ((s.dn[114][11] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n11));
        let eq17_e2094_d_n12: f64 = ((s.dn[114][12] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n12));
        let eq17_e2094_d_n13: f64 = ((s.dn[114][13] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n13));
        let eq17_e2094_d_n14: f64 = ((s.dn[114][14] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n14));
        let eq17_e2094_d_n15: f64 = ((s.dn[114][15] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n15));
        let eq17_e2094_d_n16: f64 = ((s.dn[114][16] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n16));
        (eq17_e2094, eq17_e2094_d_n0, eq17_e2094_d_n1, eq17_e2094_d_n2, eq17_e2094_d_n3, eq17_e2094_d_n4, eq17_e2094_d_n5, eq17_e2094_d_n6, eq17_e2094_d_n7, eq17_e2094_d_n8, eq17_e2094_d_n9, eq17_e2094_d_n10, eq17_e2094_d_n11, eq17_e2094_d_n12, eq17_e2094_d_n13, eq17_e2094_d_n14, eq17_e2094_d_n15, eq17_e2094_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e2096;
        let eq17_node_derivatives: [f64; 17] = [eq17_e2096_d_n0, eq17_e2096_d_n1, eq17_e2096_d_n2, eq17_e2096_d_n3, eq17_e2096_d_n4, eq17_e2096_d_n5, eq17_e2096_d_n6, eq17_e2096_d_n7, eq17_e2096_d_n8, eq17_e2096_d_n9, eq17_e2096_d_n10, eq17_e2096_d_n11, eq17_e2096_d_n12, eq17_e2096_d_n13, eq17_e2096_d_n14, eq17_e2096_d_n15, eq17_e2096_d_n16];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[3]),
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
        let (eq18_e2107, eq18_e2107_d_n0, eq18_e2107_d_n1, eq18_e2107_d_n2, eq18_e2107_d_n3, eq18_e2107_d_n4, eq18_e2107_d_n5, eq18_e2107_d_n6, eq18_e2107_d_n7, eq18_e2107_d_n8, eq18_e2107_d_n9, eq18_e2107_d_n10, eq18_e2107_d_n11, eq18_e2107_d_n12, eq18_e2107_d_n13, eq18_e2107_d_n14, eq18_e2107_d_n15, eq18_e2107_d_n16,) = {
    if ((s.v[1698] != 0.0) && (!(s.v[1699] != 0.0))) {
        let eq18_e2104: f64 = (s.v[476] + s.v[488]);
        let eq18_e2104_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq18_e2104_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq18_e2104_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq18_e2104_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq18_e2104_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq18_e2104_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq18_e2104_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq18_e2104_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq18_e2104_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq18_e2104_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq18_e2104_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq18_e2104_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq18_e2104_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq18_e2104_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq18_e2104_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq18_e2104_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq18_e2104_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq18_e2105: f64 = (s.v[114] * eq18_e2104);
        let eq18_e2105_d_n0: f64 = ((s.dn[114][0] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n0));
        let eq18_e2105_d_n1: f64 = ((s.dn[114][1] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n1));
        let eq18_e2105_d_n2: f64 = ((s.dn[114][2] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n2));
        let eq18_e2105_d_n3: f64 = ((s.dn[114][3] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n3));
        let eq18_e2105_d_n4: f64 = ((s.dn[114][4] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n4));
        let eq18_e2105_d_n5: f64 = ((s.dn[114][5] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n5));
        let eq18_e2105_d_n6: f64 = ((s.dn[114][6] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n6));
        let eq18_e2105_d_n7: f64 = ((s.dn[114][7] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n7));
        let eq18_e2105_d_n8: f64 = ((s.dn[114][8] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n8));
        let eq18_e2105_d_n9: f64 = ((s.dn[114][9] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n9));
        let eq18_e2105_d_n10: f64 = ((s.dn[114][10] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n10));
        let eq18_e2105_d_n11: f64 = ((s.dn[114][11] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n11));
        let eq18_e2105_d_n12: f64 = ((s.dn[114][12] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n12));
        let eq18_e2105_d_n13: f64 = ((s.dn[114][13] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n13));
        let eq18_e2105_d_n14: f64 = ((s.dn[114][14] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n14));
        let eq18_e2105_d_n15: f64 = ((s.dn[114][15] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n15));
        let eq18_e2105_d_n16: f64 = ((s.dn[114][16] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n16));
        (eq18_e2105, eq18_e2105_d_n0, eq18_e2105_d_n1, eq18_e2105_d_n2, eq18_e2105_d_n3, eq18_e2105_d_n4, eq18_e2105_d_n5, eq18_e2105_d_n6, eq18_e2105_d_n7, eq18_e2105_d_n8, eq18_e2105_d_n9, eq18_e2105_d_n10, eq18_e2105_d_n11, eq18_e2105_d_n12, eq18_e2105_d_n13, eq18_e2105_d_n14, eq18_e2105_d_n15, eq18_e2105_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e2107;
        let eq18_node_derivatives: [f64; 17] = [eq18_e2107_d_n0, eq18_e2107_d_n1, eq18_e2107_d_n2, eq18_e2107_d_n3, eq18_e2107_d_n4, eq18_e2107_d_n5, eq18_e2107_d_n6, eq18_e2107_d_n7, eq18_e2107_d_n8, eq18_e2107_d_n9, eq18_e2107_d_n10, eq18_e2107_d_n11, eq18_e2107_d_n12, eq18_e2107_d_n13, eq18_e2107_d_n14, eq18_e2107_d_n15, eq18_e2107_d_n16];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let (eq19_e2116, eq19_e2116_d_n0, eq19_e2116_d_n1, eq19_e2116_d_n2, eq19_e2116_d_n3, eq19_e2116_d_n4, eq19_e2116_d_n5, eq19_e2116_d_n6, eq19_e2116_d_n7, eq19_e2116_d_n8, eq19_e2116_d_n9, eq19_e2116_d_n10, eq19_e2116_d_n11, eq19_e2116_d_n12, eq19_e2116_d_n13, eq19_e2116_d_n14, eq19_e2116_d_n15, eq19_e2116_d_n16,) = {
    if ((s.v[1698] != 0.0) && (!(s.v[1699] != 0.0))) {
        let eq19_e2114: f64 = (s.v[114] * s.v[475]);
        let eq19_e2114_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq19_e2114_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq19_e2114_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq19_e2114_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq19_e2114_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq19_e2114_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq19_e2114_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq19_e2114_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq19_e2114_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq19_e2114_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq19_e2114_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq19_e2114_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq19_e2114_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq19_e2114_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq19_e2114_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq19_e2114_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq19_e2114_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        (eq19_e2114, eq19_e2114_d_n0, eq19_e2114_d_n1, eq19_e2114_d_n2, eq19_e2114_d_n3, eq19_e2114_d_n4, eq19_e2114_d_n5, eq19_e2114_d_n6, eq19_e2114_d_n7, eq19_e2114_d_n8, eq19_e2114_d_n9, eq19_e2114_d_n10, eq19_e2114_d_n11, eq19_e2114_d_n12, eq19_e2114_d_n13, eq19_e2114_d_n14, eq19_e2114_d_n15, eq19_e2114_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e2116;
        let eq19_node_derivatives: [f64; 17] = [eq19_e2116_d_n0, eq19_e2116_d_n1, eq19_e2116_d_n2, eq19_e2116_d_n3, eq19_e2116_d_n4, eq19_e2116_d_n5, eq19_e2116_d_n6, eq19_e2116_d_n7, eq19_e2116_d_n8, eq19_e2116_d_n9, eq19_e2116_d_n10, eq19_e2116_d_n11, eq19_e2116_d_n12, eq19_e2116_d_n13, eq19_e2116_d_n14, eq19_e2116_d_n15, eq19_e2116_d_n16];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let (eq20_e2123, eq20_e2123_d_n0, eq20_e2123_d_n1, eq20_e2123_d_n2, eq20_e2123_d_n3, eq20_e2123_d_n4, eq20_e2123_d_n5, eq20_e2123_d_n6, eq20_e2123_d_n7, eq20_e2123_d_n8, eq20_e2123_d_n9, eq20_e2123_d_n10, eq20_e2123_d_n11, eq20_e2123_d_n12, eq20_e2123_d_n13, eq20_e2123_d_n14, eq20_e2123_d_n15, eq20_e2123_d_n16,) = {
    if (!(s.v[1698] != 0.0)) {
        let eq20_e2121: f64 = (s.v[114] * s.v[556]);
        let eq20_e2121_d_n0: f64 = ((s.dn[114][0] * s.v[556]) + (s.v[114] * s.dn[556][0]));
        let eq20_e2121_d_n1: f64 = ((s.dn[114][1] * s.v[556]) + (s.v[114] * s.dn[556][1]));
        let eq20_e2121_d_n2: f64 = ((s.dn[114][2] * s.v[556]) + (s.v[114] * s.dn[556][2]));
        let eq20_e2121_d_n3: f64 = ((s.dn[114][3] * s.v[556]) + (s.v[114] * s.dn[556][3]));
        let eq20_e2121_d_n4: f64 = ((s.dn[114][4] * s.v[556]) + (s.v[114] * s.dn[556][4]));
        let eq20_e2121_d_n5: f64 = ((s.dn[114][5] * s.v[556]) + (s.v[114] * s.dn[556][5]));
        let eq20_e2121_d_n6: f64 = ((s.dn[114][6] * s.v[556]) + (s.v[114] * s.dn[556][6]));
        let eq20_e2121_d_n7: f64 = ((s.dn[114][7] * s.v[556]) + (s.v[114] * s.dn[556][7]));
        let eq20_e2121_d_n8: f64 = ((s.dn[114][8] * s.v[556]) + (s.v[114] * s.dn[556][8]));
        let eq20_e2121_d_n9: f64 = ((s.dn[114][9] * s.v[556]) + (s.v[114] * s.dn[556][9]));
        let eq20_e2121_d_n10: f64 = ((s.dn[114][10] * s.v[556]) + (s.v[114] * s.dn[556][10]));
        let eq20_e2121_d_n11: f64 = ((s.dn[114][11] * s.v[556]) + (s.v[114] * s.dn[556][11]));
        let eq20_e2121_d_n12: f64 = ((s.dn[114][12] * s.v[556]) + (s.v[114] * s.dn[556][12]));
        let eq20_e2121_d_n13: f64 = ((s.dn[114][13] * s.v[556]) + (s.v[114] * s.dn[556][13]));
        let eq20_e2121_d_n14: f64 = ((s.dn[114][14] * s.v[556]) + (s.v[114] * s.dn[556][14]));
        let eq20_e2121_d_n15: f64 = ((s.dn[114][15] * s.v[556]) + (s.v[114] * s.dn[556][15]));
        let eq20_e2121_d_n16: f64 = ((s.dn[114][16] * s.v[556]) + (s.v[114] * s.dn[556][16]));
        (eq20_e2121, eq20_e2121_d_n0, eq20_e2121_d_n1, eq20_e2121_d_n2, eq20_e2121_d_n3, eq20_e2121_d_n4, eq20_e2121_d_n5, eq20_e2121_d_n6, eq20_e2121_d_n7, eq20_e2121_d_n8, eq20_e2121_d_n9, eq20_e2121_d_n10, eq20_e2121_d_n11, eq20_e2121_d_n12, eq20_e2121_d_n13, eq20_e2121_d_n14, eq20_e2121_d_n15, eq20_e2121_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e2123;
        let eq20_node_derivatives: [f64; 17] = [eq20_e2123_d_n0, eq20_e2123_d_n1, eq20_e2123_d_n2, eq20_e2123_d_n3, eq20_e2123_d_n4, eq20_e2123_d_n5, eq20_e2123_d_n6, eq20_e2123_d_n7, eq20_e2123_d_n8, eq20_e2123_d_n9, eq20_e2123_d_n10, eq20_e2123_d_n11, eq20_e2123_d_n12, eq20_e2123_d_n13, eq20_e2123_d_n14, eq20_e2123_d_n15, eq20_e2123_d_n16];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let (eq21_e2132, eq21_e2132_d_n0, eq21_e2132_d_n1, eq21_e2132_d_n2, eq21_e2132_d_n3, eq21_e2132_d_n4, eq21_e2132_d_n5, eq21_e2132_d_n6, eq21_e2132_d_n7, eq21_e2132_d_n8, eq21_e2132_d_n9, eq21_e2132_d_n10, eq21_e2132_d_n11, eq21_e2132_d_n12, eq21_e2132_d_n13, eq21_e2132_d_n14, eq21_e2132_d_n15, eq21_e2132_d_n16,) = {
    if (!(s.v[1698] != 0.0)) {
        let eq21_e2129: f64 = (s.v[470] + s.v[480]);
        let eq21_e2129_d_n0: f64 = (s.dn[470][0] + s.dn[480][0]);
        let eq21_e2129_d_n1: f64 = (s.dn[470][1] + s.dn[480][1]);
        let eq21_e2129_d_n2: f64 = (s.dn[470][2] + s.dn[480][2]);
        let eq21_e2129_d_n3: f64 = (s.dn[470][3] + s.dn[480][3]);
        let eq21_e2129_d_n4: f64 = (s.dn[470][4] + s.dn[480][4]);
        let eq21_e2129_d_n5: f64 = (s.dn[470][5] + s.dn[480][5]);
        let eq21_e2129_d_n6: f64 = (s.dn[470][6] + s.dn[480][6]);
        let eq21_e2129_d_n7: f64 = (s.dn[470][7] + s.dn[480][7]);
        let eq21_e2129_d_n8: f64 = (s.dn[470][8] + s.dn[480][8]);
        let eq21_e2129_d_n9: f64 = (s.dn[470][9] + s.dn[480][9]);
        let eq21_e2129_d_n10: f64 = (s.dn[470][10] + s.dn[480][10]);
        let eq21_e2129_d_n11: f64 = (s.dn[470][11] + s.dn[480][11]);
        let eq21_e2129_d_n12: f64 = (s.dn[470][12] + s.dn[480][12]);
        let eq21_e2129_d_n13: f64 = (s.dn[470][13] + s.dn[480][13]);
        let eq21_e2129_d_n14: f64 = (s.dn[470][14] + s.dn[480][14]);
        let eq21_e2129_d_n15: f64 = (s.dn[470][15] + s.dn[480][15]);
        let eq21_e2129_d_n16: f64 = (s.dn[470][16] + s.dn[480][16]);
        let eq21_e2130: f64 = (s.v[114] * eq21_e2129);
        let eq21_e2130_d_n0: f64 = ((s.dn[114][0] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n0));
        let eq21_e2130_d_n1: f64 = ((s.dn[114][1] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n1));
        let eq21_e2130_d_n2: f64 = ((s.dn[114][2] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n2));
        let eq21_e2130_d_n3: f64 = ((s.dn[114][3] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n3));
        let eq21_e2130_d_n4: f64 = ((s.dn[114][4] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n4));
        let eq21_e2130_d_n5: f64 = ((s.dn[114][5] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n5));
        let eq21_e2130_d_n6: f64 = ((s.dn[114][6] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n6));
        let eq21_e2130_d_n7: f64 = ((s.dn[114][7] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n7));
        let eq21_e2130_d_n8: f64 = ((s.dn[114][8] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n8));
        let eq21_e2130_d_n9: f64 = ((s.dn[114][9] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n9));
        let eq21_e2130_d_n10: f64 = ((s.dn[114][10] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n10));
        let eq21_e2130_d_n11: f64 = ((s.dn[114][11] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n11));
        let eq21_e2130_d_n12: f64 = ((s.dn[114][12] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n12));
        let eq21_e2130_d_n13: f64 = ((s.dn[114][13] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n13));
        let eq21_e2130_d_n14: f64 = ((s.dn[114][14] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n14));
        let eq21_e2130_d_n15: f64 = ((s.dn[114][15] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n15));
        let eq21_e2130_d_n16: f64 = ((s.dn[114][16] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n16));
        (eq21_e2130, eq21_e2130_d_n0, eq21_e2130_d_n1, eq21_e2130_d_n2, eq21_e2130_d_n3, eq21_e2130_d_n4, eq21_e2130_d_n5, eq21_e2130_d_n6, eq21_e2130_d_n7, eq21_e2130_d_n8, eq21_e2130_d_n9, eq21_e2130_d_n10, eq21_e2130_d_n11, eq21_e2130_d_n12, eq21_e2130_d_n13, eq21_e2130_d_n14, eq21_e2130_d_n15, eq21_e2130_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e2132;
        let eq21_node_derivatives: [f64; 17] = [eq21_e2132_d_n0, eq21_e2132_d_n1, eq21_e2132_d_n2, eq21_e2132_d_n3, eq21_e2132_d_n4, eq21_e2132_d_n5, eq21_e2132_d_n6, eq21_e2132_d_n7, eq21_e2132_d_n8, eq21_e2132_d_n9, eq21_e2132_d_n10, eq21_e2132_d_n11, eq21_e2132_d_n12, eq21_e2132_d_n13, eq21_e2132_d_n14, eq21_e2132_d_n15, eq21_e2132_d_n16];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[5]),
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
        let (eq22_e2141, eq22_e2141_d_n0, eq22_e2141_d_n1, eq22_e2141_d_n2, eq22_e2141_d_n3, eq22_e2141_d_n4, eq22_e2141_d_n5, eq22_e2141_d_n6, eq22_e2141_d_n7, eq22_e2141_d_n8, eq22_e2141_d_n9, eq22_e2141_d_n10, eq22_e2141_d_n11, eq22_e2141_d_n12, eq22_e2141_d_n13, eq22_e2141_d_n14, eq22_e2141_d_n15, eq22_e2141_d_n16,) = {
    if (!(s.v[1698] != 0.0)) {
        let eq22_e2138: f64 = (s.v[471] + s.v[481]);
        let eq22_e2138_d_n0: f64 = (s.dn[471][0] + s.dn[481][0]);
        let eq22_e2138_d_n1: f64 = (s.dn[471][1] + s.dn[481][1]);
        let eq22_e2138_d_n2: f64 = (s.dn[471][2] + s.dn[481][2]);
        let eq22_e2138_d_n3: f64 = (s.dn[471][3] + s.dn[481][3]);
        let eq22_e2138_d_n4: f64 = (s.dn[471][4] + s.dn[481][4]);
        let eq22_e2138_d_n5: f64 = (s.dn[471][5] + s.dn[481][5]);
        let eq22_e2138_d_n6: f64 = (s.dn[471][6] + s.dn[481][6]);
        let eq22_e2138_d_n7: f64 = (s.dn[471][7] + s.dn[481][7]);
        let eq22_e2138_d_n8: f64 = (s.dn[471][8] + s.dn[481][8]);
        let eq22_e2138_d_n9: f64 = (s.dn[471][9] + s.dn[481][9]);
        let eq22_e2138_d_n10: f64 = (s.dn[471][10] + s.dn[481][10]);
        let eq22_e2138_d_n11: f64 = (s.dn[471][11] + s.dn[481][11]);
        let eq22_e2138_d_n12: f64 = (s.dn[471][12] + s.dn[481][12]);
        let eq22_e2138_d_n13: f64 = (s.dn[471][13] + s.dn[481][13]);
        let eq22_e2138_d_n14: f64 = (s.dn[471][14] + s.dn[481][14]);
        let eq22_e2138_d_n15: f64 = (s.dn[471][15] + s.dn[481][15]);
        let eq22_e2138_d_n16: f64 = (s.dn[471][16] + s.dn[481][16]);
        let eq22_e2139: f64 = (s.v[114] * eq22_e2138);
        let eq22_e2139_d_n0: f64 = ((s.dn[114][0] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n0));
        let eq22_e2139_d_n1: f64 = ((s.dn[114][1] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n1));
        let eq22_e2139_d_n2: f64 = ((s.dn[114][2] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n2));
        let eq22_e2139_d_n3: f64 = ((s.dn[114][3] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n3));
        let eq22_e2139_d_n4: f64 = ((s.dn[114][4] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n4));
        let eq22_e2139_d_n5: f64 = ((s.dn[114][5] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n5));
        let eq22_e2139_d_n6: f64 = ((s.dn[114][6] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n6));
        let eq22_e2139_d_n7: f64 = ((s.dn[114][7] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n7));
        let eq22_e2139_d_n8: f64 = ((s.dn[114][8] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n8));
        let eq22_e2139_d_n9: f64 = ((s.dn[114][9] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n9));
        let eq22_e2139_d_n10: f64 = ((s.dn[114][10] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n10));
        let eq22_e2139_d_n11: f64 = ((s.dn[114][11] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n11));
        let eq22_e2139_d_n12: f64 = ((s.dn[114][12] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n12));
        let eq22_e2139_d_n13: f64 = ((s.dn[114][13] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n13));
        let eq22_e2139_d_n14: f64 = ((s.dn[114][14] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n14));
        let eq22_e2139_d_n15: f64 = ((s.dn[114][15] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n15));
        let eq22_e2139_d_n16: f64 = ((s.dn[114][16] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n16));
        (eq22_e2139, eq22_e2139_d_n0, eq22_e2139_d_n1, eq22_e2139_d_n2, eq22_e2139_d_n3, eq22_e2139_d_n4, eq22_e2139_d_n5, eq22_e2139_d_n6, eq22_e2139_d_n7, eq22_e2139_d_n8, eq22_e2139_d_n9, eq22_e2139_d_n10, eq22_e2139_d_n11, eq22_e2139_d_n12, eq22_e2139_d_n13, eq22_e2139_d_n14, eq22_e2139_d_n15, eq22_e2139_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e2141;
        let eq22_node_derivatives: [f64; 17] = [eq22_e2141_d_n0, eq22_e2141_d_n1, eq22_e2141_d_n2, eq22_e2141_d_n3, eq22_e2141_d_n4, eq22_e2141_d_n5, eq22_e2141_d_n6, eq22_e2141_d_n7, eq22_e2141_d_n8, eq22_e2141_d_n9, eq22_e2141_d_n10, eq22_e2141_d_n11, eq22_e2141_d_n12, eq22_e2141_d_n13, eq22_e2141_d_n14, eq22_e2141_d_n15, eq22_e2141_d_n16];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq22_value),
            &nodes,
            &eq22_node_derivatives,
            &branches,
            &eq22_branch_derivatives,
            self.multiplicity,
        );
    }
}
