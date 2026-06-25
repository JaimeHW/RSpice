#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq39_e696, eq39_e696_d_n0, eq39_e696_d_n1, eq39_e696_d_n2, eq39_e696_d_n3, eq39_e696_d_n4, eq39_e696_d_n5, eq39_e696_d_n6, eq39_e696_d_n7, eq39_e696_d_n8, eq39_e696_d_n9, eq39_e696_d_n10, eq39_e696_d_n11, eq39_e696_d_n12, eq39_e696_d_n13, eq39_e696_d_n14, eq39_e696_d_n15, eq39_e696_d_n16, eq39_e696_d_n17, eq39_e696_d_n18, eq39_e696_d_n19, eq39_e696_d_n20, eq39_e696_d_n21, eq39_e696_d_n22,) = {
    if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
        let eq39_e694: f64 = ((nv13 - nv14) / s.v[343]);
        let eq39_e694_d_n0: f64 = (-(((nv13 - nv14) * s.dn[343][0]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n1: f64 = (-(((nv13 - nv14) * s.dn[343][1]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n2: f64 = (-(((nv13 - nv14) * s.dn[343][2]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n3: f64 = (-(((nv13 - nv14) * s.dn[343][3]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n4: f64 = (-(((nv13 - nv14) * s.dn[343][4]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n5: f64 = (-(((nv13 - nv14) * s.dn[343][5]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n6: f64 = (-(((nv13 - nv14) * s.dn[343][6]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n7: f64 = (-(((nv13 - nv14) * s.dn[343][7]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n8: f64 = (-(((nv13 - nv14) * s.dn[343][8]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n9: f64 = (-(((nv13 - nv14) * s.dn[343][9]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n10: f64 = (-(((nv13 - nv14) * s.dn[343][10]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n11: f64 = (-(((nv13 - nv14) * s.dn[343][11]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n12: f64 = (-(((nv13 - nv14) * s.dn[343][12]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n13: f64 = ((s.v[343] - ((nv13 - nv14) * s.dn[343][13])) / (s.v[343] * s.v[343]));
        let eq39_e694_d_n14: f64 = (((-s.v[343]) - ((nv13 - nv14) * s.dn[343][14])) / (s.v[343] * s.v[343]));
        let eq39_e694_d_n15: f64 = (-(((nv13 - nv14) * s.dn[343][15]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n16: f64 = (-(((nv13 - nv14) * s.dn[343][16]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n17: f64 = (-(((nv13 - nv14) * s.dn[343][17]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n18: f64 = (-(((nv13 - nv14) * s.dn[343][18]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n19: f64 = (-(((nv13 - nv14) * s.dn[343][19]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n20: f64 = (-(((nv13 - nv14) * s.dn[343][20]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n21: f64 = (-(((nv13 - nv14) * s.dn[343][21]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n22: f64 = (-(((nv13 - nv14) * s.dn[343][22]) / (s.v[343] * s.v[343])));
        (eq39_e694, eq39_e694_d_n0, eq39_e694_d_n1, eq39_e694_d_n2, eq39_e694_d_n3, eq39_e694_d_n4, eq39_e694_d_n5, eq39_e694_d_n6, eq39_e694_d_n7, eq39_e694_d_n8, eq39_e694_d_n9, eq39_e694_d_n10, eq39_e694_d_n11, eq39_e694_d_n12, eq39_e694_d_n13, eq39_e694_d_n14, eq39_e694_d_n15, eq39_e694_d_n16, eq39_e694_d_n17, eq39_e694_d_n18, eq39_e694_d_n19, eq39_e694_d_n20, eq39_e694_d_n21, eq39_e694_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e696;
        let eq39_node_derivatives: [f64; 23] = [eq39_e696_d_n0, eq39_e696_d_n1, eq39_e696_d_n2, eq39_e696_d_n3, eq39_e696_d_n4, eq39_e696_d_n5, eq39_e696_d_n6, eq39_e696_d_n7, eq39_e696_d_n8, eq39_e696_d_n9, eq39_e696_d_n10, eq39_e696_d_n11, eq39_e696_d_n12, eq39_e696_d_n13, eq39_e696_d_n14, eq39_e696_d_n15, eq39_e696_d_n16, eq39_e696_d_n17, eq39_e696_d_n18, eq39_e696_d_n19, eq39_e696_d_n20, eq39_e696_d_n21, eq39_e696_d_n22];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[14]),
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq40_e716, eq40_e716_d_n0, eq40_e716_d_n1, eq40_e716_d_n2, eq40_e716_d_n3, eq40_e716_d_n4, eq40_e716_d_n5, eq40_e716_d_n6, eq40_e716_d_n7, eq40_e716_d_n8, eq40_e716_d_n9, eq40_e716_d_n10, eq40_e716_d_n11, eq40_e716_d_n12, eq40_e716_d_n13, eq40_e716_d_n14, eq40_e716_d_n15, eq40_e716_d_n16, eq40_e716_d_n17, eq40_e716_d_n18, eq40_e716_d_n19, eq40_e716_d_n20, eq40_e716_d_n21, eq40_e716_d_n22,) = {
    if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
        let eq40_e709: f64 = self.eval_ddt(5, (nv14 - 0.0));
        let eq40_e709_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n14: f64 = self.ddt_jacobian(1.0);
        let eq40_e709_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq40_e709_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq40_e710: f64 = (p.p83 * eq40_e709);
        let eq40_e710_d_n0: f64 = (p.p83 * eq40_e709_d_n0);
        let eq40_e710_d_n1: f64 = (p.p83 * eq40_e709_d_n1);
        let eq40_e710_d_n2: f64 = (p.p83 * eq40_e709_d_n2);
        let eq40_e710_d_n3: f64 = (p.p83 * eq40_e709_d_n3);
        let eq40_e710_d_n4: f64 = (p.p83 * eq40_e709_d_n4);
        let eq40_e710_d_n5: f64 = (p.p83 * eq40_e709_d_n5);
        let eq40_e710_d_n6: f64 = (p.p83 * eq40_e709_d_n6);
        let eq40_e710_d_n7: f64 = (p.p83 * eq40_e709_d_n7);
        let eq40_e710_d_n8: f64 = (p.p83 * eq40_e709_d_n8);
        let eq40_e710_d_n9: f64 = (p.p83 * eq40_e709_d_n9);
        let eq40_e710_d_n10: f64 = (p.p83 * eq40_e709_d_n10);
        let eq40_e710_d_n11: f64 = (p.p83 * eq40_e709_d_n11);
        let eq40_e710_d_n12: f64 = (p.p83 * eq40_e709_d_n12);
        let eq40_e710_d_n13: f64 = (p.p83 * eq40_e709_d_n13);
        let eq40_e710_d_n14: f64 = (p.p83 * eq40_e709_d_n14);
        let eq40_e710_d_n15: f64 = (p.p83 * eq40_e709_d_n15);
        let eq40_e710_d_n16: f64 = (p.p83 * eq40_e709_d_n16);
        let eq40_e710_d_n17: f64 = (p.p83 * eq40_e709_d_n17);
        let eq40_e710_d_n18: f64 = (p.p83 * eq40_e709_d_n18);
        let eq40_e710_d_n19: f64 = (p.p83 * eq40_e709_d_n19);
        let eq40_e710_d_n20: f64 = (p.p83 * eq40_e709_d_n20);
        let eq40_e710_d_n21: f64 = (p.p83 * eq40_e709_d_n21);
        let eq40_e710_d_n22: f64 = (p.p83 * eq40_e709_d_n22);
        let eq40_e713: f64 = (1e-12 * (nv14 - 0.0));
        let eq40_e713_d_n14: f64 = 1e-12;
        let eq40_e714: f64 = (eq40_e710 + eq40_e713);
        let eq40_e714_d_n14: f64 = (eq40_e710_d_n14 + eq40_e713_d_n14);
        (eq40_e714, eq40_e710_d_n0, eq40_e710_d_n1, eq40_e710_d_n2, eq40_e710_d_n3, eq40_e710_d_n4, eq40_e710_d_n5, eq40_e710_d_n6, eq40_e710_d_n7, eq40_e710_d_n8, eq40_e710_d_n9, eq40_e710_d_n10, eq40_e710_d_n11, eq40_e710_d_n12, eq40_e710_d_n13, eq40_e714_d_n14, eq40_e710_d_n15, eq40_e710_d_n16, eq40_e710_d_n17, eq40_e710_d_n18, eq40_e710_d_n19, eq40_e710_d_n20, eq40_e710_d_n21, eq40_e710_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e716;
        let eq40_node_derivatives: [f64; 23] = [eq40_e716_d_n0, eq40_e716_d_n1, eq40_e716_d_n2, eq40_e716_d_n3, eq40_e716_d_n4, eq40_e716_d_n5, eq40_e716_d_n6, eq40_e716_d_n7, eq40_e716_d_n8, eq40_e716_d_n9, eq40_e716_d_n10, eq40_e716_d_n11, eq40_e716_d_n12, eq40_e716_d_n13, eq40_e716_d_n14, eq40_e716_d_n15, eq40_e716_d_n16, eq40_e716_d_n17, eq40_e716_d_n18, eq40_e716_d_n19, eq40_e716_d_n20, eq40_e716_d_n21, eq40_e716_d_n22];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            None,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq41_e747, eq41_e747_d_n0, eq41_e747_d_n1, eq41_e747_d_n2, eq41_e747_d_n3, eq41_e747_d_n4, eq41_e747_d_n5, eq41_e747_d_n6, eq41_e747_d_n7, eq41_e747_d_n8, eq41_e747_d_n9, eq41_e747_d_n10, eq41_e747_d_n11, eq41_e747_d_n12, eq41_e747_d_n13, eq41_e747_d_n14, eq41_e747_d_n15, eq41_e747_d_n16, eq41_e747_d_n17, eq41_e747_d_n18, eq41_e747_d_n19, eq41_e747_d_n20, eq41_e747_d_n21, eq41_e747_d_n22,) = {
    if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
        let eq41_e730: f64 = (-p.p135);
        let eq41_e732: f64 = (eq41_e730 * s.v[363]);
        let eq41_e732_d_n0: f64 = (eq41_e730 * s.dn[363][0]);
        let eq41_e732_d_n1: f64 = (eq41_e730 * s.dn[363][1]);
        let eq41_e732_d_n2: f64 = (eq41_e730 * s.dn[363][2]);
        let eq41_e732_d_n3: f64 = (eq41_e730 * s.dn[363][3]);
        let eq41_e732_d_n4: f64 = (eq41_e730 * s.dn[363][4]);
        let eq41_e732_d_n5: f64 = (eq41_e730 * s.dn[363][5]);
        let eq41_e732_d_n6: f64 = (eq41_e730 * s.dn[363][6]);
        let eq41_e732_d_n7: f64 = (eq41_e730 * s.dn[363][7]);
        let eq41_e732_d_n8: f64 = (eq41_e730 * s.dn[363][8]);
        let eq41_e732_d_n9: f64 = (eq41_e730 * s.dn[363][9]);
        let eq41_e732_d_n10: f64 = (eq41_e730 * s.dn[363][10]);
        let eq41_e732_d_n11: f64 = (eq41_e730 * s.dn[363][11]);
        let eq41_e732_d_n12: f64 = (eq41_e730 * s.dn[363][12]);
        let eq41_e732_d_n13: f64 = (eq41_e730 * s.dn[363][13]);
        let eq41_e732_d_n14: f64 = (eq41_e730 * s.dn[363][14]);
        let eq41_e732_d_n15: f64 = (eq41_e730 * s.dn[363][15]);
        let eq41_e732_d_n16: f64 = (eq41_e730 * s.dn[363][16]);
        let eq41_e732_d_n17: f64 = (eq41_e730 * s.dn[363][17]);
        let eq41_e732_d_n18: f64 = (eq41_e730 * s.dn[363][18]);
        let eq41_e732_d_n19: f64 = (eq41_e730 * s.dn[363][19]);
        let eq41_e732_d_n20: f64 = (eq41_e730 * s.dn[363][20]);
        let eq41_e732_d_n21: f64 = (eq41_e730 * s.dn[363][21]);
        let eq41_e732_d_n22: f64 = (eq41_e730 * s.dn[363][22]);
        let eq41_e735: f64 = (p.p136 - (nv5 - 0.0));
        let eq41_e735_d_n5: f64 = (-1.0);
        let eq41_e736: f64 = (eq41_e732 * eq41_e735);
        let eq41_e736_d_n0: f64 = (eq41_e732_d_n0 * eq41_e735);
        let eq41_e736_d_n1: f64 = (eq41_e732_d_n1 * eq41_e735);
        let eq41_e736_d_n2: f64 = (eq41_e732_d_n2 * eq41_e735);
        let eq41_e736_d_n3: f64 = (eq41_e732_d_n3 * eq41_e735);
        let eq41_e736_d_n4: f64 = (eq41_e732_d_n4 * eq41_e735);
        let eq41_e736_d_n5: f64 = ((eq41_e732_d_n5 * eq41_e735) + (eq41_e732 * eq41_e735_d_n5));
        let eq41_e736_d_n6: f64 = (eq41_e732_d_n6 * eq41_e735);
        let eq41_e736_d_n7: f64 = (eq41_e732_d_n7 * eq41_e735);
        let eq41_e736_d_n8: f64 = (eq41_e732_d_n8 * eq41_e735);
        let eq41_e736_d_n9: f64 = (eq41_e732_d_n9 * eq41_e735);
        let eq41_e736_d_n10: f64 = (eq41_e732_d_n10 * eq41_e735);
        let eq41_e736_d_n11: f64 = (eq41_e732_d_n11 * eq41_e735);
        let eq41_e736_d_n12: f64 = (eq41_e732_d_n12 * eq41_e735);
        let eq41_e736_d_n13: f64 = (eq41_e732_d_n13 * eq41_e735);
        let eq41_e736_d_n14: f64 = (eq41_e732_d_n14 * eq41_e735);
        let eq41_e736_d_n15: f64 = (eq41_e732_d_n15 * eq41_e735);
        let eq41_e736_d_n16: f64 = (eq41_e732_d_n16 * eq41_e735);
        let eq41_e736_d_n17: f64 = (eq41_e732_d_n17 * eq41_e735);
        let eq41_e736_d_n18: f64 = (eq41_e732_d_n18 * eq41_e735);
        let eq41_e736_d_n19: f64 = (eq41_e732_d_n19 * eq41_e735);
        let eq41_e736_d_n20: f64 = (eq41_e732_d_n20 * eq41_e735);
        let eq41_e736_d_n21: f64 = (eq41_e732_d_n21 * eq41_e735);
        let eq41_e736_d_n22: f64 = (eq41_e732_d_n22 * eq41_e735);
        let eq41_e739: f64 = (2.0 * s.v[362]);
        let eq41_e739_d_n0: f64 = (2.0 * s.dn[362][0]);
        let eq41_e739_d_n1: f64 = (2.0 * s.dn[362][1]);
        let eq41_e739_d_n2: f64 = (2.0 * s.dn[362][2]);
        let eq41_e739_d_n3: f64 = (2.0 * s.dn[362][3]);
        let eq41_e739_d_n4: f64 = (2.0 * s.dn[362][4]);
        let eq41_e739_d_n5: f64 = (2.0 * s.dn[362][5]);
        let eq41_e739_d_n6: f64 = (2.0 * s.dn[362][6]);
        let eq41_e739_d_n7: f64 = (2.0 * s.dn[362][7]);
        let eq41_e739_d_n8: f64 = (2.0 * s.dn[362][8]);
        let eq41_e739_d_n9: f64 = (2.0 * s.dn[362][9]);
        let eq41_e739_d_n10: f64 = (2.0 * s.dn[362][10]);
        let eq41_e739_d_n11: f64 = (2.0 * s.dn[362][11]);
        let eq41_e739_d_n12: f64 = (2.0 * s.dn[362][12]);
        let eq41_e739_d_n13: f64 = (2.0 * s.dn[362][13]);
        let eq41_e739_d_n14: f64 = (2.0 * s.dn[362][14]);
        let eq41_e739_d_n15: f64 = (2.0 * s.dn[362][15]);
        let eq41_e739_d_n16: f64 = (2.0 * s.dn[362][16]);
        let eq41_e739_d_n17: f64 = (2.0 * s.dn[362][17]);
        let eq41_e739_d_n18: f64 = (2.0 * s.dn[362][18]);
        let eq41_e739_d_n19: f64 = (2.0 * s.dn[362][19]);
        let eq41_e739_d_n20: f64 = (2.0 * s.dn[362][20]);
        let eq41_e739_d_n21: f64 = (2.0 * s.dn[362][21]);
        let eq41_e739_d_n22: f64 = (2.0 * s.dn[362][22]);
        let eq41_e740: f64 = (eq41_e739).exp();
        let eq41_e740_d_n0: f64 = (eq41_e740 * eq41_e739_d_n0);
        let eq41_e740_d_n1: f64 = (eq41_e740 * eq41_e739_d_n1);
        let eq41_e740_d_n2: f64 = (eq41_e740 * eq41_e739_d_n2);
        let eq41_e740_d_n3: f64 = (eq41_e740 * eq41_e739_d_n3);
        let eq41_e740_d_n4: f64 = (eq41_e740 * eq41_e739_d_n4);
        let eq41_e740_d_n5: f64 = (eq41_e740 * eq41_e739_d_n5);
        let eq41_e740_d_n6: f64 = (eq41_e740 * eq41_e739_d_n6);
        let eq41_e740_d_n7: f64 = (eq41_e740 * eq41_e739_d_n7);
        let eq41_e740_d_n8: f64 = (eq41_e740 * eq41_e739_d_n8);
        let eq41_e740_d_n9: f64 = (eq41_e740 * eq41_e739_d_n9);
        let eq41_e740_d_n10: f64 = (eq41_e740 * eq41_e739_d_n10);
        let eq41_e740_d_n11: f64 = (eq41_e740 * eq41_e739_d_n11);
        let eq41_e740_d_n12: f64 = (eq41_e740 * eq41_e739_d_n12);
        let eq41_e740_d_n13: f64 = (eq41_e740 * eq41_e739_d_n13);
        let eq41_e740_d_n14: f64 = (eq41_e740 * eq41_e739_d_n14);
        let eq41_e740_d_n15: f64 = (eq41_e740 * eq41_e739_d_n15);
        let eq41_e740_d_n16: f64 = (eq41_e740 * eq41_e739_d_n16);
        let eq41_e740_d_n17: f64 = (eq41_e740 * eq41_e739_d_n17);
        let eq41_e740_d_n18: f64 = (eq41_e740 * eq41_e739_d_n18);
        let eq41_e740_d_n19: f64 = (eq41_e740 * eq41_e739_d_n19);
        let eq41_e740_d_n20: f64 = (eq41_e740 * eq41_e739_d_n20);
        let eq41_e740_d_n21: f64 = (eq41_e740 * eq41_e739_d_n21);
        let eq41_e740_d_n22: f64 = (eq41_e740 * eq41_e739_d_n22);
        let eq41_e742: f64 = (eq41_e740 - 1.0);
        let eq41_e743: f64 = (eq41_e736 * eq41_e742);
        let eq41_e743_d_n0: f64 = ((eq41_e736_d_n0 * eq41_e742) + (eq41_e736 * eq41_e740_d_n0));
        let eq41_e743_d_n1: f64 = ((eq41_e736_d_n1 * eq41_e742) + (eq41_e736 * eq41_e740_d_n1));
        let eq41_e743_d_n2: f64 = ((eq41_e736_d_n2 * eq41_e742) + (eq41_e736 * eq41_e740_d_n2));
        let eq41_e743_d_n3: f64 = ((eq41_e736_d_n3 * eq41_e742) + (eq41_e736 * eq41_e740_d_n3));
        let eq41_e743_d_n4: f64 = ((eq41_e736_d_n4 * eq41_e742) + (eq41_e736 * eq41_e740_d_n4));
        let eq41_e743_d_n5: f64 = ((eq41_e736_d_n5 * eq41_e742) + (eq41_e736 * eq41_e740_d_n5));
        let eq41_e743_d_n6: f64 = ((eq41_e736_d_n6 * eq41_e742) + (eq41_e736 * eq41_e740_d_n6));
        let eq41_e743_d_n7: f64 = ((eq41_e736_d_n7 * eq41_e742) + (eq41_e736 * eq41_e740_d_n7));
        let eq41_e743_d_n8: f64 = ((eq41_e736_d_n8 * eq41_e742) + (eq41_e736 * eq41_e740_d_n8));
        let eq41_e743_d_n9: f64 = ((eq41_e736_d_n9 * eq41_e742) + (eq41_e736 * eq41_e740_d_n9));
        let eq41_e743_d_n10: f64 = ((eq41_e736_d_n10 * eq41_e742) + (eq41_e736 * eq41_e740_d_n10));
        let eq41_e743_d_n11: f64 = ((eq41_e736_d_n11 * eq41_e742) + (eq41_e736 * eq41_e740_d_n11));
        let eq41_e743_d_n12: f64 = ((eq41_e736_d_n12 * eq41_e742) + (eq41_e736 * eq41_e740_d_n12));
        let eq41_e743_d_n13: f64 = ((eq41_e736_d_n13 * eq41_e742) + (eq41_e736 * eq41_e740_d_n13));
        let eq41_e743_d_n14: f64 = ((eq41_e736_d_n14 * eq41_e742) + (eq41_e736 * eq41_e740_d_n14));
        let eq41_e743_d_n15: f64 = ((eq41_e736_d_n15 * eq41_e742) + (eq41_e736 * eq41_e740_d_n15));
        let eq41_e743_d_n16: f64 = ((eq41_e736_d_n16 * eq41_e742) + (eq41_e736 * eq41_e740_d_n16));
        let eq41_e743_d_n17: f64 = ((eq41_e736_d_n17 * eq41_e742) + (eq41_e736 * eq41_e740_d_n17));
        let eq41_e743_d_n18: f64 = ((eq41_e736_d_n18 * eq41_e742) + (eq41_e736 * eq41_e740_d_n18));
        let eq41_e743_d_n19: f64 = ((eq41_e736_d_n19 * eq41_e742) + (eq41_e736 * eq41_e740_d_n19));
        let eq41_e743_d_n20: f64 = ((eq41_e736_d_n20 * eq41_e742) + (eq41_e736 * eq41_e740_d_n20));
        let eq41_e743_d_n21: f64 = ((eq41_e736_d_n21 * eq41_e742) + (eq41_e736 * eq41_e740_d_n21));
        let eq41_e743_d_n22: f64 = ((eq41_e736_d_n22 * eq41_e742) + (eq41_e736 * eq41_e740_d_n22));
        let eq41_e745: f64 = (eq41_e743 * 0.5);
        let eq41_e745_d_n0: f64 = (eq41_e743_d_n0 * 0.5);
        let eq41_e745_d_n1: f64 = (eq41_e743_d_n1 * 0.5);
        let eq41_e745_d_n2: f64 = (eq41_e743_d_n2 * 0.5);
        let eq41_e745_d_n3: f64 = (eq41_e743_d_n3 * 0.5);
        let eq41_e745_d_n4: f64 = (eq41_e743_d_n4 * 0.5);
        let eq41_e745_d_n5: f64 = (eq41_e743_d_n5 * 0.5);
        let eq41_e745_d_n6: f64 = (eq41_e743_d_n6 * 0.5);
        let eq41_e745_d_n7: f64 = (eq41_e743_d_n7 * 0.5);
        let eq41_e745_d_n8: f64 = (eq41_e743_d_n8 * 0.5);
        let eq41_e745_d_n9: f64 = (eq41_e743_d_n9 * 0.5);
        let eq41_e745_d_n10: f64 = (eq41_e743_d_n10 * 0.5);
        let eq41_e745_d_n11: f64 = (eq41_e743_d_n11 * 0.5);
        let eq41_e745_d_n12: f64 = (eq41_e743_d_n12 * 0.5);
        let eq41_e745_d_n13: f64 = (eq41_e743_d_n13 * 0.5);
        let eq41_e745_d_n14: f64 = (eq41_e743_d_n14 * 0.5);
        let eq41_e745_d_n15: f64 = (eq41_e743_d_n15 * 0.5);
        let eq41_e745_d_n16: f64 = (eq41_e743_d_n16 * 0.5);
        let eq41_e745_d_n17: f64 = (eq41_e743_d_n17 * 0.5);
        let eq41_e745_d_n18: f64 = (eq41_e743_d_n18 * 0.5);
        let eq41_e745_d_n19: f64 = (eq41_e743_d_n19 * 0.5);
        let eq41_e745_d_n20: f64 = (eq41_e743_d_n20 * 0.5);
        let eq41_e745_d_n21: f64 = (eq41_e743_d_n21 * 0.5);
        let eq41_e745_d_n22: f64 = (eq41_e743_d_n22 * 0.5);
        (eq41_e745, eq41_e745_d_n0, eq41_e745_d_n1, eq41_e745_d_n2, eq41_e745_d_n3, eq41_e745_d_n4, eq41_e745_d_n5, eq41_e745_d_n6, eq41_e745_d_n7, eq41_e745_d_n8, eq41_e745_d_n9, eq41_e745_d_n10, eq41_e745_d_n11, eq41_e745_d_n12, eq41_e745_d_n13, eq41_e745_d_n14, eq41_e745_d_n15, eq41_e745_d_n16, eq41_e745_d_n17, eq41_e745_d_n18, eq41_e745_d_n19, eq41_e745_d_n20, eq41_e745_d_n21, eq41_e745_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e747;
        let eq41_node_derivatives: [f64; 23] = [eq41_e747_d_n0, eq41_e747_d_n1, eq41_e747_d_n2, eq41_e747_d_n3, eq41_e747_d_n4, eq41_e747_d_n5, eq41_e747_d_n6, eq41_e747_d_n7, eq41_e747_d_n8, eq41_e747_d_n9, eq41_e747_d_n10, eq41_e747_d_n11, eq41_e747_d_n12, eq41_e747_d_n13, eq41_e747_d_n14, eq41_e747_d_n15, eq41_e747_d_n16, eq41_e747_d_n17, eq41_e747_d_n18, eq41_e747_d_n19, eq41_e747_d_n20, eq41_e747_d_n21, eq41_e747_d_n22];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq42_e766, eq42_e766_d_n0, eq42_e766_d_n1, eq42_e766_d_n2, eq42_e766_d_n3, eq42_e766_d_n4, eq42_e766_d_n5, eq42_e766_d_n6, eq42_e766_d_n7, eq42_e766_d_n8, eq42_e766_d_n9, eq42_e766_d_n10, eq42_e766_d_n11, eq42_e766_d_n12, eq42_e766_d_n13, eq42_e766_d_n14, eq42_e766_d_n15, eq42_e766_d_n16, eq42_e766_d_n17, eq42_e766_d_n18, eq42_e766_d_n19, eq42_e766_d_n20, eq42_e766_d_n21, eq42_e766_d_n22,) = {
    if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
        let eq42_e762: f64 = (p.p135 * s.v[363]);
        let eq42_e762_d_n0: f64 = (p.p135 * s.dn[363][0]);
        let eq42_e762_d_n1: f64 = (p.p135 * s.dn[363][1]);
        let eq42_e762_d_n2: f64 = (p.p135 * s.dn[363][2]);
        let eq42_e762_d_n3: f64 = (p.p135 * s.dn[363][3]);
        let eq42_e762_d_n4: f64 = (p.p135 * s.dn[363][4]);
        let eq42_e762_d_n5: f64 = (p.p135 * s.dn[363][5]);
        let eq42_e762_d_n6: f64 = (p.p135 * s.dn[363][6]);
        let eq42_e762_d_n7: f64 = (p.p135 * s.dn[363][7]);
        let eq42_e762_d_n8: f64 = (p.p135 * s.dn[363][8]);
        let eq42_e762_d_n9: f64 = (p.p135 * s.dn[363][9]);
        let eq42_e762_d_n10: f64 = (p.p135 * s.dn[363][10]);
        let eq42_e762_d_n11: f64 = (p.p135 * s.dn[363][11]);
        let eq42_e762_d_n12: f64 = (p.p135 * s.dn[363][12]);
        let eq42_e762_d_n13: f64 = (p.p135 * s.dn[363][13]);
        let eq42_e762_d_n14: f64 = (p.p135 * s.dn[363][14]);
        let eq42_e762_d_n15: f64 = (p.p135 * s.dn[363][15]);
        let eq42_e762_d_n16: f64 = (p.p135 * s.dn[363][16]);
        let eq42_e762_d_n17: f64 = (p.p135 * s.dn[363][17]);
        let eq42_e762_d_n18: f64 = (p.p135 * s.dn[363][18]);
        let eq42_e762_d_n19: f64 = (p.p135 * s.dn[363][19]);
        let eq42_e762_d_n20: f64 = (p.p135 * s.dn[363][20]);
        let eq42_e762_d_n21: f64 = (p.p135 * s.dn[363][21]);
        let eq42_e762_d_n22: f64 = (p.p135 * s.dn[363][22]);
        let eq42_e764: f64 = (eq42_e762 * (nv5 - 0.0));
        let eq42_e764_d_n0: f64 = (eq42_e762_d_n0 * (nv5 - 0.0));
        let eq42_e764_d_n1: f64 = (eq42_e762_d_n1 * (nv5 - 0.0));
        let eq42_e764_d_n2: f64 = (eq42_e762_d_n2 * (nv5 - 0.0));
        let eq42_e764_d_n3: f64 = (eq42_e762_d_n3 * (nv5 - 0.0));
        let eq42_e764_d_n4: f64 = (eq42_e762_d_n4 * (nv5 - 0.0));
        let eq42_e764_d_n5: f64 = ((eq42_e762_d_n5 * (nv5 - 0.0)) + eq42_e762);
        let eq42_e764_d_n6: f64 = (eq42_e762_d_n6 * (nv5 - 0.0));
        let eq42_e764_d_n7: f64 = (eq42_e762_d_n7 * (nv5 - 0.0));
        let eq42_e764_d_n8: f64 = (eq42_e762_d_n8 * (nv5 - 0.0));
        let eq42_e764_d_n9: f64 = (eq42_e762_d_n9 * (nv5 - 0.0));
        let eq42_e764_d_n10: f64 = (eq42_e762_d_n10 * (nv5 - 0.0));
        let eq42_e764_d_n11: f64 = (eq42_e762_d_n11 * (nv5 - 0.0));
        let eq42_e764_d_n12: f64 = (eq42_e762_d_n12 * (nv5 - 0.0));
        let eq42_e764_d_n13: f64 = (eq42_e762_d_n13 * (nv5 - 0.0));
        let eq42_e764_d_n14: f64 = (eq42_e762_d_n14 * (nv5 - 0.0));
        let eq42_e764_d_n15: f64 = (eq42_e762_d_n15 * (nv5 - 0.0));
        let eq42_e764_d_n16: f64 = (eq42_e762_d_n16 * (nv5 - 0.0));
        let eq42_e764_d_n17: f64 = (eq42_e762_d_n17 * (nv5 - 0.0));
        let eq42_e764_d_n18: f64 = (eq42_e762_d_n18 * (nv5 - 0.0));
        let eq42_e764_d_n19: f64 = (eq42_e762_d_n19 * (nv5 - 0.0));
        let eq42_e764_d_n20: f64 = (eq42_e762_d_n20 * (nv5 - 0.0));
        let eq42_e764_d_n21: f64 = (eq42_e762_d_n21 * (nv5 - 0.0));
        let eq42_e764_d_n22: f64 = (eq42_e762_d_n22 * (nv5 - 0.0));
        (eq42_e764, eq42_e764_d_n0, eq42_e764_d_n1, eq42_e764_d_n2, eq42_e764_d_n3, eq42_e764_d_n4, eq42_e764_d_n5, eq42_e764_d_n6, eq42_e764_d_n7, eq42_e764_d_n8, eq42_e764_d_n9, eq42_e764_d_n10, eq42_e764_d_n11, eq42_e764_d_n12, eq42_e764_d_n13, eq42_e764_d_n14, eq42_e764_d_n15, eq42_e764_d_n16, eq42_e764_d_n17, eq42_e764_d_n18, eq42_e764_d_n19, eq42_e764_d_n20, eq42_e764_d_n21, eq42_e764_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e766;
        let eq42_node_derivatives: [f64; 23] = [eq42_e766_d_n0, eq42_e766_d_n1, eq42_e766_d_n2, eq42_e766_d_n3, eq42_e766_d_n4, eq42_e766_d_n5, eq42_e766_d_n6, eq42_e766_d_n7, eq42_e766_d_n8, eq42_e766_d_n9, eq42_e766_d_n10, eq42_e766_d_n11, eq42_e766_d_n12, eq42_e766_d_n13, eq42_e766_d_n14, eq42_e766_d_n15, eq42_e766_d_n16, eq42_e766_d_n17, eq42_e766_d_n18, eq42_e766_d_n19, eq42_e766_d_n20, eq42_e766_d_n21, eq42_e766_d_n22];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq43_e784, eq43_e784_d_n0, eq43_e784_d_n1, eq43_e784_d_n2, eq43_e784_d_n3, eq43_e784_d_n4, eq43_e784_d_n5, eq43_e784_d_n6, eq43_e784_d_n7, eq43_e784_d_n8, eq43_e784_d_n9, eq43_e784_d_n10, eq43_e784_d_n11, eq43_e784_d_n12, eq43_e784_d_n13, eq43_e784_d_n14, eq43_e784_d_n15, eq43_e784_d_n16, eq43_e784_d_n17, eq43_e784_d_n18, eq43_e784_d_n19, eq43_e784_d_n20, eq43_e784_d_n21, eq43_e784_d_n22,) = {
    if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
        let eq43_e781: f64 = self.eval_ddt(6, (nv5 - 0.0));
        let eq43_e781_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n5: f64 = self.ddt_jacobian(1.0);
        let eq43_e781_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq43_e781_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq43_e782: f64 = (p.p135 * eq43_e781);
        let eq43_e782_d_n0: f64 = (p.p135 * eq43_e781_d_n0);
        let eq43_e782_d_n1: f64 = (p.p135 * eq43_e781_d_n1);
        let eq43_e782_d_n2: f64 = (p.p135 * eq43_e781_d_n2);
        let eq43_e782_d_n3: f64 = (p.p135 * eq43_e781_d_n3);
        let eq43_e782_d_n4: f64 = (p.p135 * eq43_e781_d_n4);
        let eq43_e782_d_n5: f64 = (p.p135 * eq43_e781_d_n5);
        let eq43_e782_d_n6: f64 = (p.p135 * eq43_e781_d_n6);
        let eq43_e782_d_n7: f64 = (p.p135 * eq43_e781_d_n7);
        let eq43_e782_d_n8: f64 = (p.p135 * eq43_e781_d_n8);
        let eq43_e782_d_n9: f64 = (p.p135 * eq43_e781_d_n9);
        let eq43_e782_d_n10: f64 = (p.p135 * eq43_e781_d_n10);
        let eq43_e782_d_n11: f64 = (p.p135 * eq43_e781_d_n11);
        let eq43_e782_d_n12: f64 = (p.p135 * eq43_e781_d_n12);
        let eq43_e782_d_n13: f64 = (p.p135 * eq43_e781_d_n13);
        let eq43_e782_d_n14: f64 = (p.p135 * eq43_e781_d_n14);
        let eq43_e782_d_n15: f64 = (p.p135 * eq43_e781_d_n15);
        let eq43_e782_d_n16: f64 = (p.p135 * eq43_e781_d_n16);
        let eq43_e782_d_n17: f64 = (p.p135 * eq43_e781_d_n17);
        let eq43_e782_d_n18: f64 = (p.p135 * eq43_e781_d_n18);
        let eq43_e782_d_n19: f64 = (p.p135 * eq43_e781_d_n19);
        let eq43_e782_d_n20: f64 = (p.p135 * eq43_e781_d_n20);
        let eq43_e782_d_n21: f64 = (p.p135 * eq43_e781_d_n21);
        let eq43_e782_d_n22: f64 = (p.p135 * eq43_e781_d_n22);
        (eq43_e782, eq43_e782_d_n0, eq43_e782_d_n1, eq43_e782_d_n2, eq43_e782_d_n3, eq43_e782_d_n4, eq43_e782_d_n5, eq43_e782_d_n6, eq43_e782_d_n7, eq43_e782_d_n8, eq43_e782_d_n9, eq43_e782_d_n10, eq43_e782_d_n11, eq43_e782_d_n12, eq43_e782_d_n13, eq43_e782_d_n14, eq43_e782_d_n15, eq43_e782_d_n16, eq43_e782_d_n17, eq43_e782_d_n18, eq43_e782_d_n19, eq43_e782_d_n20, eq43_e782_d_n21, eq43_e782_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e784;
        let eq43_node_derivatives: [f64; 23] = [eq43_e784_d_n0, eq43_e784_d_n1, eq43_e784_d_n2, eq43_e784_d_n3, eq43_e784_d_n4, eq43_e784_d_n5, eq43_e784_d_n6, eq43_e784_d_n7, eq43_e784_d_n8, eq43_e784_d_n9, eq43_e784_d_n10, eq43_e784_d_n11, eq43_e784_d_n12, eq43_e784_d_n13, eq43_e784_d_n14, eq43_e784_d_n15, eq43_e784_d_n16, eq43_e784_d_n17, eq43_e784_d_n18, eq43_e784_d_n19, eq43_e784_d_n20, eq43_e784_d_n21, eq43_e784_d_n22];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq44_e815, eq44_e815_d_n0, eq44_e815_d_n1, eq44_e815_d_n2, eq44_e815_d_n3, eq44_e815_d_n4, eq44_e815_d_n5, eq44_e815_d_n6, eq44_e815_d_n7, eq44_e815_d_n8, eq44_e815_d_n9, eq44_e815_d_n10, eq44_e815_d_n11, eq44_e815_d_n12, eq44_e815_d_n13, eq44_e815_d_n14, eq44_e815_d_n15, eq44_e815_d_n16, eq44_e815_d_n17, eq44_e815_d_n18, eq44_e815_d_n19, eq44_e815_d_n20, eq44_e815_d_n21, eq44_e815_d_n22,) = {
    if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
        let eq44_e798: f64 = (-p.p144);
        let eq44_e800: f64 = (eq44_e798 * s.v[367]);
        let eq44_e800_d_n0: f64 = (eq44_e798 * s.dn[367][0]);
        let eq44_e800_d_n1: f64 = (eq44_e798 * s.dn[367][1]);
        let eq44_e800_d_n2: f64 = (eq44_e798 * s.dn[367][2]);
        let eq44_e800_d_n3: f64 = (eq44_e798 * s.dn[367][3]);
        let eq44_e800_d_n4: f64 = (eq44_e798 * s.dn[367][4]);
        let eq44_e800_d_n5: f64 = (eq44_e798 * s.dn[367][5]);
        let eq44_e800_d_n6: f64 = (eq44_e798 * s.dn[367][6]);
        let eq44_e800_d_n7: f64 = (eq44_e798 * s.dn[367][7]);
        let eq44_e800_d_n8: f64 = (eq44_e798 * s.dn[367][8]);
        let eq44_e800_d_n9: f64 = (eq44_e798 * s.dn[367][9]);
        let eq44_e800_d_n10: f64 = (eq44_e798 * s.dn[367][10]);
        let eq44_e800_d_n11: f64 = (eq44_e798 * s.dn[367][11]);
        let eq44_e800_d_n12: f64 = (eq44_e798 * s.dn[367][12]);
        let eq44_e800_d_n13: f64 = (eq44_e798 * s.dn[367][13]);
        let eq44_e800_d_n14: f64 = (eq44_e798 * s.dn[367][14]);
        let eq44_e800_d_n15: f64 = (eq44_e798 * s.dn[367][15]);
        let eq44_e800_d_n16: f64 = (eq44_e798 * s.dn[367][16]);
        let eq44_e800_d_n17: f64 = (eq44_e798 * s.dn[367][17]);
        let eq44_e800_d_n18: f64 = (eq44_e798 * s.dn[367][18]);
        let eq44_e800_d_n19: f64 = (eq44_e798 * s.dn[367][19]);
        let eq44_e800_d_n20: f64 = (eq44_e798 * s.dn[367][20]);
        let eq44_e800_d_n21: f64 = (eq44_e798 * s.dn[367][21]);
        let eq44_e800_d_n22: f64 = (eq44_e798 * s.dn[367][22]);
        let eq44_e803: f64 = (p.p145 - (nv6 - 0.0));
        let eq44_e803_d_n6: f64 = (-1.0);
        let eq44_e804: f64 = (eq44_e800 * eq44_e803);
        let eq44_e804_d_n0: f64 = (eq44_e800_d_n0 * eq44_e803);
        let eq44_e804_d_n1: f64 = (eq44_e800_d_n1 * eq44_e803);
        let eq44_e804_d_n2: f64 = (eq44_e800_d_n2 * eq44_e803);
        let eq44_e804_d_n3: f64 = (eq44_e800_d_n3 * eq44_e803);
        let eq44_e804_d_n4: f64 = (eq44_e800_d_n4 * eq44_e803);
        let eq44_e804_d_n5: f64 = (eq44_e800_d_n5 * eq44_e803);
        let eq44_e804_d_n6: f64 = ((eq44_e800_d_n6 * eq44_e803) + (eq44_e800 * eq44_e803_d_n6));
        let eq44_e804_d_n7: f64 = (eq44_e800_d_n7 * eq44_e803);
        let eq44_e804_d_n8: f64 = (eq44_e800_d_n8 * eq44_e803);
        let eq44_e804_d_n9: f64 = (eq44_e800_d_n9 * eq44_e803);
        let eq44_e804_d_n10: f64 = (eq44_e800_d_n10 * eq44_e803);
        let eq44_e804_d_n11: f64 = (eq44_e800_d_n11 * eq44_e803);
        let eq44_e804_d_n12: f64 = (eq44_e800_d_n12 * eq44_e803);
        let eq44_e804_d_n13: f64 = (eq44_e800_d_n13 * eq44_e803);
        let eq44_e804_d_n14: f64 = (eq44_e800_d_n14 * eq44_e803);
        let eq44_e804_d_n15: f64 = (eq44_e800_d_n15 * eq44_e803);
        let eq44_e804_d_n16: f64 = (eq44_e800_d_n16 * eq44_e803);
        let eq44_e804_d_n17: f64 = (eq44_e800_d_n17 * eq44_e803);
        let eq44_e804_d_n18: f64 = (eq44_e800_d_n18 * eq44_e803);
        let eq44_e804_d_n19: f64 = (eq44_e800_d_n19 * eq44_e803);
        let eq44_e804_d_n20: f64 = (eq44_e800_d_n20 * eq44_e803);
        let eq44_e804_d_n21: f64 = (eq44_e800_d_n21 * eq44_e803);
        let eq44_e804_d_n22: f64 = (eq44_e800_d_n22 * eq44_e803);
        let eq44_e807: f64 = (2.0 * s.v[368]);
        let eq44_e807_d_n0: f64 = (2.0 * s.dn[368][0]);
        let eq44_e807_d_n1: f64 = (2.0 * s.dn[368][1]);
        let eq44_e807_d_n2: f64 = (2.0 * s.dn[368][2]);
        let eq44_e807_d_n3: f64 = (2.0 * s.dn[368][3]);
        let eq44_e807_d_n4: f64 = (2.0 * s.dn[368][4]);
        let eq44_e807_d_n5: f64 = (2.0 * s.dn[368][5]);
        let eq44_e807_d_n6: f64 = (2.0 * s.dn[368][6]);
        let eq44_e807_d_n7: f64 = (2.0 * s.dn[368][7]);
        let eq44_e807_d_n8: f64 = (2.0 * s.dn[368][8]);
        let eq44_e807_d_n9: f64 = (2.0 * s.dn[368][9]);
        let eq44_e807_d_n10: f64 = (2.0 * s.dn[368][10]);
        let eq44_e807_d_n11: f64 = (2.0 * s.dn[368][11]);
        let eq44_e807_d_n12: f64 = (2.0 * s.dn[368][12]);
        let eq44_e807_d_n13: f64 = (2.0 * s.dn[368][13]);
        let eq44_e807_d_n14: f64 = (2.0 * s.dn[368][14]);
        let eq44_e807_d_n15: f64 = (2.0 * s.dn[368][15]);
        let eq44_e807_d_n16: f64 = (2.0 * s.dn[368][16]);
        let eq44_e807_d_n17: f64 = (2.0 * s.dn[368][17]);
        let eq44_e807_d_n18: f64 = (2.0 * s.dn[368][18]);
        let eq44_e807_d_n19: f64 = (2.0 * s.dn[368][19]);
        let eq44_e807_d_n20: f64 = (2.0 * s.dn[368][20]);
        let eq44_e807_d_n21: f64 = (2.0 * s.dn[368][21]);
        let eq44_e807_d_n22: f64 = (2.0 * s.dn[368][22]);
        let eq44_e808: f64 = (eq44_e807).exp();
        let eq44_e808_d_n0: f64 = (eq44_e808 * eq44_e807_d_n0);
        let eq44_e808_d_n1: f64 = (eq44_e808 * eq44_e807_d_n1);
        let eq44_e808_d_n2: f64 = (eq44_e808 * eq44_e807_d_n2);
        let eq44_e808_d_n3: f64 = (eq44_e808 * eq44_e807_d_n3);
        let eq44_e808_d_n4: f64 = (eq44_e808 * eq44_e807_d_n4);
        let eq44_e808_d_n5: f64 = (eq44_e808 * eq44_e807_d_n5);
        let eq44_e808_d_n6: f64 = (eq44_e808 * eq44_e807_d_n6);
        let eq44_e808_d_n7: f64 = (eq44_e808 * eq44_e807_d_n7);
        let eq44_e808_d_n8: f64 = (eq44_e808 * eq44_e807_d_n8);
        let eq44_e808_d_n9: f64 = (eq44_e808 * eq44_e807_d_n9);
        let eq44_e808_d_n10: f64 = (eq44_e808 * eq44_e807_d_n10);
        let eq44_e808_d_n11: f64 = (eq44_e808 * eq44_e807_d_n11);
        let eq44_e808_d_n12: f64 = (eq44_e808 * eq44_e807_d_n12);
        let eq44_e808_d_n13: f64 = (eq44_e808 * eq44_e807_d_n13);
        let eq44_e808_d_n14: f64 = (eq44_e808 * eq44_e807_d_n14);
        let eq44_e808_d_n15: f64 = (eq44_e808 * eq44_e807_d_n15);
        let eq44_e808_d_n16: f64 = (eq44_e808 * eq44_e807_d_n16);
        let eq44_e808_d_n17: f64 = (eq44_e808 * eq44_e807_d_n17);
        let eq44_e808_d_n18: f64 = (eq44_e808 * eq44_e807_d_n18);
        let eq44_e808_d_n19: f64 = (eq44_e808 * eq44_e807_d_n19);
        let eq44_e808_d_n20: f64 = (eq44_e808 * eq44_e807_d_n20);
        let eq44_e808_d_n21: f64 = (eq44_e808 * eq44_e807_d_n21);
        let eq44_e808_d_n22: f64 = (eq44_e808 * eq44_e807_d_n22);
        let eq44_e810: f64 = (eq44_e808 - 1.0);
        let eq44_e811: f64 = (eq44_e804 * eq44_e810);
        let eq44_e811_d_n0: f64 = ((eq44_e804_d_n0 * eq44_e810) + (eq44_e804 * eq44_e808_d_n0));
        let eq44_e811_d_n1: f64 = ((eq44_e804_d_n1 * eq44_e810) + (eq44_e804 * eq44_e808_d_n1));
        let eq44_e811_d_n2: f64 = ((eq44_e804_d_n2 * eq44_e810) + (eq44_e804 * eq44_e808_d_n2));
        let eq44_e811_d_n3: f64 = ((eq44_e804_d_n3 * eq44_e810) + (eq44_e804 * eq44_e808_d_n3));
        let eq44_e811_d_n4: f64 = ((eq44_e804_d_n4 * eq44_e810) + (eq44_e804 * eq44_e808_d_n4));
        let eq44_e811_d_n5: f64 = ((eq44_e804_d_n5 * eq44_e810) + (eq44_e804 * eq44_e808_d_n5));
        let eq44_e811_d_n6: f64 = ((eq44_e804_d_n6 * eq44_e810) + (eq44_e804 * eq44_e808_d_n6));
        let eq44_e811_d_n7: f64 = ((eq44_e804_d_n7 * eq44_e810) + (eq44_e804 * eq44_e808_d_n7));
        let eq44_e811_d_n8: f64 = ((eq44_e804_d_n8 * eq44_e810) + (eq44_e804 * eq44_e808_d_n8));
        let eq44_e811_d_n9: f64 = ((eq44_e804_d_n9 * eq44_e810) + (eq44_e804 * eq44_e808_d_n9));
        let eq44_e811_d_n10: f64 = ((eq44_e804_d_n10 * eq44_e810) + (eq44_e804 * eq44_e808_d_n10));
        let eq44_e811_d_n11: f64 = ((eq44_e804_d_n11 * eq44_e810) + (eq44_e804 * eq44_e808_d_n11));
        let eq44_e811_d_n12: f64 = ((eq44_e804_d_n12 * eq44_e810) + (eq44_e804 * eq44_e808_d_n12));
        let eq44_e811_d_n13: f64 = ((eq44_e804_d_n13 * eq44_e810) + (eq44_e804 * eq44_e808_d_n13));
        let eq44_e811_d_n14: f64 = ((eq44_e804_d_n14 * eq44_e810) + (eq44_e804 * eq44_e808_d_n14));
        let eq44_e811_d_n15: f64 = ((eq44_e804_d_n15 * eq44_e810) + (eq44_e804 * eq44_e808_d_n15));
        let eq44_e811_d_n16: f64 = ((eq44_e804_d_n16 * eq44_e810) + (eq44_e804 * eq44_e808_d_n16));
        let eq44_e811_d_n17: f64 = ((eq44_e804_d_n17 * eq44_e810) + (eq44_e804 * eq44_e808_d_n17));
        let eq44_e811_d_n18: f64 = ((eq44_e804_d_n18 * eq44_e810) + (eq44_e804 * eq44_e808_d_n18));
        let eq44_e811_d_n19: f64 = ((eq44_e804_d_n19 * eq44_e810) + (eq44_e804 * eq44_e808_d_n19));
        let eq44_e811_d_n20: f64 = ((eq44_e804_d_n20 * eq44_e810) + (eq44_e804 * eq44_e808_d_n20));
        let eq44_e811_d_n21: f64 = ((eq44_e804_d_n21 * eq44_e810) + (eq44_e804 * eq44_e808_d_n21));
        let eq44_e811_d_n22: f64 = ((eq44_e804_d_n22 * eq44_e810) + (eq44_e804 * eq44_e808_d_n22));
        let eq44_e813: f64 = (eq44_e811 * 0.5);
        let eq44_e813_d_n0: f64 = (eq44_e811_d_n0 * 0.5);
        let eq44_e813_d_n1: f64 = (eq44_e811_d_n1 * 0.5);
        let eq44_e813_d_n2: f64 = (eq44_e811_d_n2 * 0.5);
        let eq44_e813_d_n3: f64 = (eq44_e811_d_n3 * 0.5);
        let eq44_e813_d_n4: f64 = (eq44_e811_d_n4 * 0.5);
        let eq44_e813_d_n5: f64 = (eq44_e811_d_n5 * 0.5);
        let eq44_e813_d_n6: f64 = (eq44_e811_d_n6 * 0.5);
        let eq44_e813_d_n7: f64 = (eq44_e811_d_n7 * 0.5);
        let eq44_e813_d_n8: f64 = (eq44_e811_d_n8 * 0.5);
        let eq44_e813_d_n9: f64 = (eq44_e811_d_n9 * 0.5);
        let eq44_e813_d_n10: f64 = (eq44_e811_d_n10 * 0.5);
        let eq44_e813_d_n11: f64 = (eq44_e811_d_n11 * 0.5);
        let eq44_e813_d_n12: f64 = (eq44_e811_d_n12 * 0.5);
        let eq44_e813_d_n13: f64 = (eq44_e811_d_n13 * 0.5);
        let eq44_e813_d_n14: f64 = (eq44_e811_d_n14 * 0.5);
        let eq44_e813_d_n15: f64 = (eq44_e811_d_n15 * 0.5);
        let eq44_e813_d_n16: f64 = (eq44_e811_d_n16 * 0.5);
        let eq44_e813_d_n17: f64 = (eq44_e811_d_n17 * 0.5);
        let eq44_e813_d_n18: f64 = (eq44_e811_d_n18 * 0.5);
        let eq44_e813_d_n19: f64 = (eq44_e811_d_n19 * 0.5);
        let eq44_e813_d_n20: f64 = (eq44_e811_d_n20 * 0.5);
        let eq44_e813_d_n21: f64 = (eq44_e811_d_n21 * 0.5);
        let eq44_e813_d_n22: f64 = (eq44_e811_d_n22 * 0.5);
        (eq44_e813, eq44_e813_d_n0, eq44_e813_d_n1, eq44_e813_d_n2, eq44_e813_d_n3, eq44_e813_d_n4, eq44_e813_d_n5, eq44_e813_d_n6, eq44_e813_d_n7, eq44_e813_d_n8, eq44_e813_d_n9, eq44_e813_d_n10, eq44_e813_d_n11, eq44_e813_d_n12, eq44_e813_d_n13, eq44_e813_d_n14, eq44_e813_d_n15, eq44_e813_d_n16, eq44_e813_d_n17, eq44_e813_d_n18, eq44_e813_d_n19, eq44_e813_d_n20, eq44_e813_d_n21, eq44_e813_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e815;
        let eq44_node_derivatives: [f64; 23] = [eq44_e815_d_n0, eq44_e815_d_n1, eq44_e815_d_n2, eq44_e815_d_n3, eq44_e815_d_n4, eq44_e815_d_n5, eq44_e815_d_n6, eq44_e815_d_n7, eq44_e815_d_n8, eq44_e815_d_n9, eq44_e815_d_n10, eq44_e815_d_n11, eq44_e815_d_n12, eq44_e815_d_n13, eq44_e815_d_n14, eq44_e815_d_n15, eq44_e815_d_n16, eq44_e815_d_n17, eq44_e815_d_n18, eq44_e815_d_n19, eq44_e815_d_n20, eq44_e815_d_n21, eq44_e815_d_n22];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq45_e834, eq45_e834_d_n0, eq45_e834_d_n1, eq45_e834_d_n2, eq45_e834_d_n3, eq45_e834_d_n4, eq45_e834_d_n5, eq45_e834_d_n6, eq45_e834_d_n7, eq45_e834_d_n8, eq45_e834_d_n9, eq45_e834_d_n10, eq45_e834_d_n11, eq45_e834_d_n12, eq45_e834_d_n13, eq45_e834_d_n14, eq45_e834_d_n15, eq45_e834_d_n16, eq45_e834_d_n17, eq45_e834_d_n18, eq45_e834_d_n19, eq45_e834_d_n20, eq45_e834_d_n21, eq45_e834_d_n22,) = {
    if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
        let eq45_e830: f64 = (p.p144 * s.v[367]);
        let eq45_e830_d_n0: f64 = (p.p144 * s.dn[367][0]);
        let eq45_e830_d_n1: f64 = (p.p144 * s.dn[367][1]);
        let eq45_e830_d_n2: f64 = (p.p144 * s.dn[367][2]);
        let eq45_e830_d_n3: f64 = (p.p144 * s.dn[367][3]);
        let eq45_e830_d_n4: f64 = (p.p144 * s.dn[367][4]);
        let eq45_e830_d_n5: f64 = (p.p144 * s.dn[367][5]);
        let eq45_e830_d_n6: f64 = (p.p144 * s.dn[367][6]);
        let eq45_e830_d_n7: f64 = (p.p144 * s.dn[367][7]);
        let eq45_e830_d_n8: f64 = (p.p144 * s.dn[367][8]);
        let eq45_e830_d_n9: f64 = (p.p144 * s.dn[367][9]);
        let eq45_e830_d_n10: f64 = (p.p144 * s.dn[367][10]);
        let eq45_e830_d_n11: f64 = (p.p144 * s.dn[367][11]);
        let eq45_e830_d_n12: f64 = (p.p144 * s.dn[367][12]);
        let eq45_e830_d_n13: f64 = (p.p144 * s.dn[367][13]);
        let eq45_e830_d_n14: f64 = (p.p144 * s.dn[367][14]);
        let eq45_e830_d_n15: f64 = (p.p144 * s.dn[367][15]);
        let eq45_e830_d_n16: f64 = (p.p144 * s.dn[367][16]);
        let eq45_e830_d_n17: f64 = (p.p144 * s.dn[367][17]);
        let eq45_e830_d_n18: f64 = (p.p144 * s.dn[367][18]);
        let eq45_e830_d_n19: f64 = (p.p144 * s.dn[367][19]);
        let eq45_e830_d_n20: f64 = (p.p144 * s.dn[367][20]);
        let eq45_e830_d_n21: f64 = (p.p144 * s.dn[367][21]);
        let eq45_e830_d_n22: f64 = (p.p144 * s.dn[367][22]);
        let eq45_e832: f64 = (eq45_e830 * (nv6 - 0.0));
        let eq45_e832_d_n0: f64 = (eq45_e830_d_n0 * (nv6 - 0.0));
        let eq45_e832_d_n1: f64 = (eq45_e830_d_n1 * (nv6 - 0.0));
        let eq45_e832_d_n2: f64 = (eq45_e830_d_n2 * (nv6 - 0.0));
        let eq45_e832_d_n3: f64 = (eq45_e830_d_n3 * (nv6 - 0.0));
        let eq45_e832_d_n4: f64 = (eq45_e830_d_n4 * (nv6 - 0.0));
        let eq45_e832_d_n5: f64 = (eq45_e830_d_n5 * (nv6 - 0.0));
        let eq45_e832_d_n6: f64 = ((eq45_e830_d_n6 * (nv6 - 0.0)) + eq45_e830);
        let eq45_e832_d_n7: f64 = (eq45_e830_d_n7 * (nv6 - 0.0));
        let eq45_e832_d_n8: f64 = (eq45_e830_d_n8 * (nv6 - 0.0));
        let eq45_e832_d_n9: f64 = (eq45_e830_d_n9 * (nv6 - 0.0));
        let eq45_e832_d_n10: f64 = (eq45_e830_d_n10 * (nv6 - 0.0));
        let eq45_e832_d_n11: f64 = (eq45_e830_d_n11 * (nv6 - 0.0));
        let eq45_e832_d_n12: f64 = (eq45_e830_d_n12 * (nv6 - 0.0));
        let eq45_e832_d_n13: f64 = (eq45_e830_d_n13 * (nv6 - 0.0));
        let eq45_e832_d_n14: f64 = (eq45_e830_d_n14 * (nv6 - 0.0));
        let eq45_e832_d_n15: f64 = (eq45_e830_d_n15 * (nv6 - 0.0));
        let eq45_e832_d_n16: f64 = (eq45_e830_d_n16 * (nv6 - 0.0));
        let eq45_e832_d_n17: f64 = (eq45_e830_d_n17 * (nv6 - 0.0));
        let eq45_e832_d_n18: f64 = (eq45_e830_d_n18 * (nv6 - 0.0));
        let eq45_e832_d_n19: f64 = (eq45_e830_d_n19 * (nv6 - 0.0));
        let eq45_e832_d_n20: f64 = (eq45_e830_d_n20 * (nv6 - 0.0));
        let eq45_e832_d_n21: f64 = (eq45_e830_d_n21 * (nv6 - 0.0));
        let eq45_e832_d_n22: f64 = (eq45_e830_d_n22 * (nv6 - 0.0));
        (eq45_e832, eq45_e832_d_n0, eq45_e832_d_n1, eq45_e832_d_n2, eq45_e832_d_n3, eq45_e832_d_n4, eq45_e832_d_n5, eq45_e832_d_n6, eq45_e832_d_n7, eq45_e832_d_n8, eq45_e832_d_n9, eq45_e832_d_n10, eq45_e832_d_n11, eq45_e832_d_n12, eq45_e832_d_n13, eq45_e832_d_n14, eq45_e832_d_n15, eq45_e832_d_n16, eq45_e832_d_n17, eq45_e832_d_n18, eq45_e832_d_n19, eq45_e832_d_n20, eq45_e832_d_n21, eq45_e832_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e834;
        let eq45_node_derivatives: [f64; 23] = [eq45_e834_d_n0, eq45_e834_d_n1, eq45_e834_d_n2, eq45_e834_d_n3, eq45_e834_d_n4, eq45_e834_d_n5, eq45_e834_d_n6, eq45_e834_d_n7, eq45_e834_d_n8, eq45_e834_d_n9, eq45_e834_d_n10, eq45_e834_d_n11, eq45_e834_d_n12, eq45_e834_d_n13, eq45_e834_d_n14, eq45_e834_d_n15, eq45_e834_d_n16, eq45_e834_d_n17, eq45_e834_d_n18, eq45_e834_d_n19, eq45_e834_d_n20, eq45_e834_d_n21, eq45_e834_d_n22];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq46_e852, eq46_e852_d_n0, eq46_e852_d_n1, eq46_e852_d_n2, eq46_e852_d_n3, eq46_e852_d_n4, eq46_e852_d_n5, eq46_e852_d_n6, eq46_e852_d_n7, eq46_e852_d_n8, eq46_e852_d_n9, eq46_e852_d_n10, eq46_e852_d_n11, eq46_e852_d_n12, eq46_e852_d_n13, eq46_e852_d_n14, eq46_e852_d_n15, eq46_e852_d_n16, eq46_e852_d_n17, eq46_e852_d_n18, eq46_e852_d_n19, eq46_e852_d_n20, eq46_e852_d_n21, eq46_e852_d_n22,) = {
    if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
        let eq46_e849: f64 = self.eval_ddt(7, (nv6 - 0.0));
        let eq46_e849_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n6: f64 = self.ddt_jacobian(1.0);
        let eq46_e849_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq46_e849_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq46_e850: f64 = (p.p144 * eq46_e849);
        let eq46_e850_d_n0: f64 = (p.p144 * eq46_e849_d_n0);
        let eq46_e850_d_n1: f64 = (p.p144 * eq46_e849_d_n1);
        let eq46_e850_d_n2: f64 = (p.p144 * eq46_e849_d_n2);
        let eq46_e850_d_n3: f64 = (p.p144 * eq46_e849_d_n3);
        let eq46_e850_d_n4: f64 = (p.p144 * eq46_e849_d_n4);
        let eq46_e850_d_n5: f64 = (p.p144 * eq46_e849_d_n5);
        let eq46_e850_d_n6: f64 = (p.p144 * eq46_e849_d_n6);
        let eq46_e850_d_n7: f64 = (p.p144 * eq46_e849_d_n7);
        let eq46_e850_d_n8: f64 = (p.p144 * eq46_e849_d_n8);
        let eq46_e850_d_n9: f64 = (p.p144 * eq46_e849_d_n9);
        let eq46_e850_d_n10: f64 = (p.p144 * eq46_e849_d_n10);
        let eq46_e850_d_n11: f64 = (p.p144 * eq46_e849_d_n11);
        let eq46_e850_d_n12: f64 = (p.p144 * eq46_e849_d_n12);
        let eq46_e850_d_n13: f64 = (p.p144 * eq46_e849_d_n13);
        let eq46_e850_d_n14: f64 = (p.p144 * eq46_e849_d_n14);
        let eq46_e850_d_n15: f64 = (p.p144 * eq46_e849_d_n15);
        let eq46_e850_d_n16: f64 = (p.p144 * eq46_e849_d_n16);
        let eq46_e850_d_n17: f64 = (p.p144 * eq46_e849_d_n17);
        let eq46_e850_d_n18: f64 = (p.p144 * eq46_e849_d_n18);
        let eq46_e850_d_n19: f64 = (p.p144 * eq46_e849_d_n19);
        let eq46_e850_d_n20: f64 = (p.p144 * eq46_e849_d_n20);
        let eq46_e850_d_n21: f64 = (p.p144 * eq46_e849_d_n21);
        let eq46_e850_d_n22: f64 = (p.p144 * eq46_e849_d_n22);
        (eq46_e850, eq46_e850_d_n0, eq46_e850_d_n1, eq46_e850_d_n2, eq46_e850_d_n3, eq46_e850_d_n4, eq46_e850_d_n5, eq46_e850_d_n6, eq46_e850_d_n7, eq46_e850_d_n8, eq46_e850_d_n9, eq46_e850_d_n10, eq46_e850_d_n11, eq46_e850_d_n12, eq46_e850_d_n13, eq46_e850_d_n14, eq46_e850_d_n15, eq46_e850_d_n16, eq46_e850_d_n17, eq46_e850_d_n18, eq46_e850_d_n19, eq46_e850_d_n20, eq46_e850_d_n21, eq46_e850_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e852;
        let eq46_node_derivatives: [f64; 23] = [eq46_e852_d_n0, eq46_e852_d_n1, eq46_e852_d_n2, eq46_e852_d_n3, eq46_e852_d_n4, eq46_e852_d_n5, eq46_e852_d_n6, eq46_e852_d_n7, eq46_e852_d_n8, eq46_e852_d_n9, eq46_e852_d_n10, eq46_e852_d_n11, eq46_e852_d_n12, eq46_e852_d_n13, eq46_e852_d_n14, eq46_e852_d_n15, eq46_e852_d_n16, eq46_e852_d_n17, eq46_e852_d_n18, eq46_e852_d_n19, eq46_e852_d_n20, eq46_e852_d_n21, eq46_e852_d_n22];
        let eq46_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            None,
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
        let (eq47_e867,) = {
    if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e867;
        stamper.stamp_potential(
            branches[25],
            eq47_value,
            &[
            ],
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
        let (eq48_e882,) = {
    if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e882;
        stamper.stamp_potential(
            branches[26],
            eq48_value,
            &[
            ],
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
        let (eq49_e897,) = {
    if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e897;
        stamper.stamp_potential(
            branches[27],
            eq49_value,
            &[
            ],
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
        let (eq50_e912,) = {
    if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e912;
        stamper.stamp_potential(
            branches[28],
            eq50_value,
            &[
            ],
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq51_e915: f64 = (p.p6 * s.v[41]);
        let eq51_e915_d_n0: f64 = (p.p6 * s.dn[41][0]);
        let eq51_e915_d_n1: f64 = (p.p6 * s.dn[41][1]);
        let eq51_e915_d_n2: f64 = (p.p6 * s.dn[41][2]);
        let eq51_e915_d_n3: f64 = (p.p6 * s.dn[41][3]);
        let eq51_e915_d_n4: f64 = (p.p6 * s.dn[41][4]);
        let eq51_e915_d_n5: f64 = (p.p6 * s.dn[41][5]);
        let eq51_e915_d_n6: f64 = (p.p6 * s.dn[41][6]);
        let eq51_e915_d_n7: f64 = (p.p6 * s.dn[41][7]);
        let eq51_e915_d_n8: f64 = (p.p6 * s.dn[41][8]);
        let eq51_e915_d_n9: f64 = (p.p6 * s.dn[41][9]);
        let eq51_e915_d_n10: f64 = (p.p6 * s.dn[41][10]);
        let eq51_e915_d_n11: f64 = (p.p6 * s.dn[41][11]);
        let eq51_e915_d_n12: f64 = (p.p6 * s.dn[41][12]);
        let eq51_e915_d_n13: f64 = (p.p6 * s.dn[41][13]);
        let eq51_e915_d_n14: f64 = (p.p6 * s.dn[41][14]);
        let eq51_e915_d_n15: f64 = (p.p6 * s.dn[41][15]);
        let eq51_e915_d_n16: f64 = (p.p6 * s.dn[41][16]);
        let eq51_e915_d_n17: f64 = (p.p6 * s.dn[41][17]);
        let eq51_e915_d_n18: f64 = (p.p6 * s.dn[41][18]);
        let eq51_e915_d_n19: f64 = (p.p6 * s.dn[41][19]);
        let eq51_e915_d_n20: f64 = (p.p6 * s.dn[41][20]);
        let eq51_e915_d_n21: f64 = (p.p6 * s.dn[41][21]);
        let eq51_e915_d_n22: f64 = (p.p6 * s.dn[41][22]);
        let eq51_e917: f64 = (eq51_e915 * s.v[94]);
        let eq51_e917_d_n0: f64 = ((eq51_e915_d_n0 * s.v[94]) + (eq51_e915 * s.dn[94][0]));
        let eq51_e917_d_n1: f64 = ((eq51_e915_d_n1 * s.v[94]) + (eq51_e915 * s.dn[94][1]));
        let eq51_e917_d_n2: f64 = ((eq51_e915_d_n2 * s.v[94]) + (eq51_e915 * s.dn[94][2]));
        let eq51_e917_d_n3: f64 = ((eq51_e915_d_n3 * s.v[94]) + (eq51_e915 * s.dn[94][3]));
        let eq51_e917_d_n4: f64 = ((eq51_e915_d_n4 * s.v[94]) + (eq51_e915 * s.dn[94][4]));
        let eq51_e917_d_n5: f64 = ((eq51_e915_d_n5 * s.v[94]) + (eq51_e915 * s.dn[94][5]));
        let eq51_e917_d_n6: f64 = ((eq51_e915_d_n6 * s.v[94]) + (eq51_e915 * s.dn[94][6]));
        let eq51_e917_d_n7: f64 = ((eq51_e915_d_n7 * s.v[94]) + (eq51_e915 * s.dn[94][7]));
        let eq51_e917_d_n8: f64 = ((eq51_e915_d_n8 * s.v[94]) + (eq51_e915 * s.dn[94][8]));
        let eq51_e917_d_n9: f64 = ((eq51_e915_d_n9 * s.v[94]) + (eq51_e915 * s.dn[94][9]));
        let eq51_e917_d_n10: f64 = ((eq51_e915_d_n10 * s.v[94]) + (eq51_e915 * s.dn[94][10]));
        let eq51_e917_d_n11: f64 = ((eq51_e915_d_n11 * s.v[94]) + (eq51_e915 * s.dn[94][11]));
        let eq51_e917_d_n12: f64 = ((eq51_e915_d_n12 * s.v[94]) + (eq51_e915 * s.dn[94][12]));
        let eq51_e917_d_n13: f64 = ((eq51_e915_d_n13 * s.v[94]) + (eq51_e915 * s.dn[94][13]));
        let eq51_e917_d_n14: f64 = ((eq51_e915_d_n14 * s.v[94]) + (eq51_e915 * s.dn[94][14]));
        let eq51_e917_d_n15: f64 = ((eq51_e915_d_n15 * s.v[94]) + (eq51_e915 * s.dn[94][15]));
        let eq51_e917_d_n16: f64 = ((eq51_e915_d_n16 * s.v[94]) + (eq51_e915 * s.dn[94][16]));
        let eq51_e917_d_n17: f64 = ((eq51_e915_d_n17 * s.v[94]) + (eq51_e915 * s.dn[94][17]));
        let eq51_e917_d_n18: f64 = ((eq51_e915_d_n18 * s.v[94]) + (eq51_e915 * s.dn[94][18]));
        let eq51_e917_d_n19: f64 = ((eq51_e915_d_n19 * s.v[94]) + (eq51_e915 * s.dn[94][19]));
        let eq51_e917_d_n20: f64 = ((eq51_e915_d_n20 * s.v[94]) + (eq51_e915 * s.dn[94][20]));
        let eq51_e917_d_n21: f64 = ((eq51_e915_d_n21 * s.v[94]) + (eq51_e915 * s.dn[94][21]));
        let eq51_e917_d_n22: f64 = ((eq51_e915_d_n22 * s.v[94]) + (eq51_e915 * s.dn[94][22]));
        let eq51_e920: f64 = (p.p6 * s.v[379]);
        let eq51_e920_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq51_e920_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq51_e920_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq51_e920_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq51_e920_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq51_e920_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq51_e920_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq51_e920_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq51_e920_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq51_e920_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq51_e920_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq51_e920_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq51_e920_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq51_e920_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq51_e920_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq51_e920_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq51_e920_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq51_e920_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq51_e920_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq51_e920_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq51_e920_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq51_e920_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq51_e920_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq51_e922: f64 = (eq51_e920 * (nv7 - nv8));
        let eq51_e922_d_n0: f64 = (eq51_e920_d_n0 * (nv7 - nv8));
        let eq51_e922_d_n1: f64 = (eq51_e920_d_n1 * (nv7 - nv8));
        let eq51_e922_d_n2: f64 = (eq51_e920_d_n2 * (nv7 - nv8));
        let eq51_e922_d_n3: f64 = (eq51_e920_d_n3 * (nv7 - nv8));
        let eq51_e922_d_n4: f64 = (eq51_e920_d_n4 * (nv7 - nv8));
        let eq51_e922_d_n5: f64 = (eq51_e920_d_n5 * (nv7 - nv8));
        let eq51_e922_d_n6: f64 = (eq51_e920_d_n6 * (nv7 - nv8));
        let eq51_e922_d_n7: f64 = ((eq51_e920_d_n7 * (nv7 - nv8)) + eq51_e920);
        let eq51_e922_d_n8: f64 = ((eq51_e920_d_n8 * (nv7 - nv8)) + (-eq51_e920));
        let eq51_e922_d_n9: f64 = (eq51_e920_d_n9 * (nv7 - nv8));
        let eq51_e922_d_n10: f64 = (eq51_e920_d_n10 * (nv7 - nv8));
        let eq51_e922_d_n11: f64 = (eq51_e920_d_n11 * (nv7 - nv8));
        let eq51_e922_d_n12: f64 = (eq51_e920_d_n12 * (nv7 - nv8));
        let eq51_e922_d_n13: f64 = (eq51_e920_d_n13 * (nv7 - nv8));
        let eq51_e922_d_n14: f64 = (eq51_e920_d_n14 * (nv7 - nv8));
        let eq51_e922_d_n15: f64 = (eq51_e920_d_n15 * (nv7 - nv8));
        let eq51_e922_d_n16: f64 = (eq51_e920_d_n16 * (nv7 - nv8));
        let eq51_e922_d_n17: f64 = (eq51_e920_d_n17 * (nv7 - nv8));
        let eq51_e922_d_n18: f64 = (eq51_e920_d_n18 * (nv7 - nv8));
        let eq51_e922_d_n19: f64 = (eq51_e920_d_n19 * (nv7 - nv8));
        let eq51_e922_d_n20: f64 = (eq51_e920_d_n20 * (nv7 - nv8));
        let eq51_e922_d_n21: f64 = (eq51_e920_d_n21 * (nv7 - nv8));
        let eq51_e922_d_n22: f64 = (eq51_e920_d_n22 * (nv7 - nv8));
        let eq51_e923: f64 = (eq51_e917 + eq51_e922);
        let eq51_e923_d_n0: f64 = (eq51_e917_d_n0 + eq51_e922_d_n0);
        let eq51_e923_d_n1: f64 = (eq51_e917_d_n1 + eq51_e922_d_n1);
        let eq51_e923_d_n2: f64 = (eq51_e917_d_n2 + eq51_e922_d_n2);
        let eq51_e923_d_n3: f64 = (eq51_e917_d_n3 + eq51_e922_d_n3);
        let eq51_e923_d_n4: f64 = (eq51_e917_d_n4 + eq51_e922_d_n4);
        let eq51_e923_d_n5: f64 = (eq51_e917_d_n5 + eq51_e922_d_n5);
        let eq51_e923_d_n6: f64 = (eq51_e917_d_n6 + eq51_e922_d_n6);
        let eq51_e923_d_n7: f64 = (eq51_e917_d_n7 + eq51_e922_d_n7);
        let eq51_e923_d_n8: f64 = (eq51_e917_d_n8 + eq51_e922_d_n8);
        let eq51_e923_d_n9: f64 = (eq51_e917_d_n9 + eq51_e922_d_n9);
        let eq51_e923_d_n10: f64 = (eq51_e917_d_n10 + eq51_e922_d_n10);
        let eq51_e923_d_n11: f64 = (eq51_e917_d_n11 + eq51_e922_d_n11);
        let eq51_e923_d_n12: f64 = (eq51_e917_d_n12 + eq51_e922_d_n12);
        let eq51_e923_d_n13: f64 = (eq51_e917_d_n13 + eq51_e922_d_n13);
        let eq51_e923_d_n14: f64 = (eq51_e917_d_n14 + eq51_e922_d_n14);
        let eq51_e923_d_n15: f64 = (eq51_e917_d_n15 + eq51_e922_d_n15);
        let eq51_e923_d_n16: f64 = (eq51_e917_d_n16 + eq51_e922_d_n16);
        let eq51_e923_d_n17: f64 = (eq51_e917_d_n17 + eq51_e922_d_n17);
        let eq51_e923_d_n18: f64 = (eq51_e917_d_n18 + eq51_e922_d_n18);
        let eq51_e923_d_n19: f64 = (eq51_e917_d_n19 + eq51_e922_d_n19);
        let eq51_e923_d_n20: f64 = (eq51_e917_d_n20 + eq51_e922_d_n20);
        let eq51_e923_d_n21: f64 = (eq51_e917_d_n21 + eq51_e922_d_n21);
        let eq51_e923_d_n22: f64 = (eq51_e917_d_n22 + eq51_e922_d_n22);
        let eq51_value: f64 = eq51_e923;
        let eq51_node_derivatives: [f64; 23] = [eq51_e923_d_n0, eq51_e923_d_n1, eq51_e923_d_n2, eq51_e923_d_n3, eq51_e923_d_n4, eq51_e923_d_n5, eq51_e923_d_n6, eq51_e923_d_n7, eq51_e923_d_n8, eq51_e923_d_n9, eq51_e923_d_n10, eq51_e923_d_n11, eq51_e923_d_n12, eq51_e923_d_n13, eq51_e923_d_n14, eq51_e923_d_n15, eq51_e923_d_n16, eq51_e923_d_n17, eq51_e923_d_n18, eq51_e923_d_n19, eq51_e923_d_n20, eq51_e923_d_n21, eq51_e923_d_n22];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
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
        let eq52_e926: f64 = (p.p6 * s.v[41]);
        let eq52_e926_d_n0: f64 = (p.p6 * s.dn[41][0]);
        let eq52_e926_d_n1: f64 = (p.p6 * s.dn[41][1]);
        let eq52_e926_d_n2: f64 = (p.p6 * s.dn[41][2]);
        let eq52_e926_d_n3: f64 = (p.p6 * s.dn[41][3]);
        let eq52_e926_d_n4: f64 = (p.p6 * s.dn[41][4]);
        let eq52_e926_d_n5: f64 = (p.p6 * s.dn[41][5]);
        let eq52_e926_d_n6: f64 = (p.p6 * s.dn[41][6]);
        let eq52_e926_d_n7: f64 = (p.p6 * s.dn[41][7]);
        let eq52_e926_d_n8: f64 = (p.p6 * s.dn[41][8]);
        let eq52_e926_d_n9: f64 = (p.p6 * s.dn[41][9]);
        let eq52_e926_d_n10: f64 = (p.p6 * s.dn[41][10]);
        let eq52_e926_d_n11: f64 = (p.p6 * s.dn[41][11]);
        let eq52_e926_d_n12: f64 = (p.p6 * s.dn[41][12]);
        let eq52_e926_d_n13: f64 = (p.p6 * s.dn[41][13]);
        let eq52_e926_d_n14: f64 = (p.p6 * s.dn[41][14]);
        let eq52_e926_d_n15: f64 = (p.p6 * s.dn[41][15]);
        let eq52_e926_d_n16: f64 = (p.p6 * s.dn[41][16]);
        let eq52_e926_d_n17: f64 = (p.p6 * s.dn[41][17]);
        let eq52_e926_d_n18: f64 = (p.p6 * s.dn[41][18]);
        let eq52_e926_d_n19: f64 = (p.p6 * s.dn[41][19]);
        let eq52_e926_d_n20: f64 = (p.p6 * s.dn[41][20]);
        let eq52_e926_d_n21: f64 = (p.p6 * s.dn[41][21]);
        let eq52_e926_d_n22: f64 = (p.p6 * s.dn[41][22]);
        let eq52_e929: f64 = (p.p4 * p.p5);
        let eq52_e931: f64 = (eq52_e929 * s.v[332]);
        let eq52_e931_d_n0: f64 = (eq52_e929 * s.dn[332][0]);
        let eq52_e931_d_n1: f64 = (eq52_e929 * s.dn[332][1]);
        let eq52_e931_d_n2: f64 = (eq52_e929 * s.dn[332][2]);
        let eq52_e931_d_n3: f64 = (eq52_e929 * s.dn[332][3]);
        let eq52_e931_d_n4: f64 = (eq52_e929 * s.dn[332][4]);
        let eq52_e931_d_n5: f64 = (eq52_e929 * s.dn[332][5]);
        let eq52_e931_d_n6: f64 = (eq52_e929 * s.dn[332][6]);
        let eq52_e931_d_n7: f64 = (eq52_e929 * s.dn[332][7]);
        let eq52_e931_d_n8: f64 = (eq52_e929 * s.dn[332][8]);
        let eq52_e931_d_n9: f64 = (eq52_e929 * s.dn[332][9]);
        let eq52_e931_d_n10: f64 = (eq52_e929 * s.dn[332][10]);
        let eq52_e931_d_n11: f64 = (eq52_e929 * s.dn[332][11]);
        let eq52_e931_d_n12: f64 = (eq52_e929 * s.dn[332][12]);
        let eq52_e931_d_n13: f64 = (eq52_e929 * s.dn[332][13]);
        let eq52_e931_d_n14: f64 = (eq52_e929 * s.dn[332][14]);
        let eq52_e931_d_n15: f64 = (eq52_e929 * s.dn[332][15]);
        let eq52_e931_d_n16: f64 = (eq52_e929 * s.dn[332][16]);
        let eq52_e931_d_n17: f64 = (eq52_e929 * s.dn[332][17]);
        let eq52_e931_d_n18: f64 = (eq52_e929 * s.dn[332][18]);
        let eq52_e931_d_n19: f64 = (eq52_e929 * s.dn[332][19]);
        let eq52_e931_d_n20: f64 = (eq52_e929 * s.dn[332][20]);
        let eq52_e931_d_n21: f64 = (eq52_e929 * s.dn[332][21]);
        let eq52_e931_d_n22: f64 = (eq52_e929 * s.dn[332][22]);
        let eq52_e932: f64 = (eq52_e926 * eq52_e931);
        let eq52_e932_d_n0: f64 = ((eq52_e926_d_n0 * eq52_e931) + (eq52_e926 * eq52_e931_d_n0));
        let eq52_e932_d_n1: f64 = ((eq52_e926_d_n1 * eq52_e931) + (eq52_e926 * eq52_e931_d_n1));
        let eq52_e932_d_n2: f64 = ((eq52_e926_d_n2 * eq52_e931) + (eq52_e926 * eq52_e931_d_n2));
        let eq52_e932_d_n3: f64 = ((eq52_e926_d_n3 * eq52_e931) + (eq52_e926 * eq52_e931_d_n3));
        let eq52_e932_d_n4: f64 = ((eq52_e926_d_n4 * eq52_e931) + (eq52_e926 * eq52_e931_d_n4));
        let eq52_e932_d_n5: f64 = ((eq52_e926_d_n5 * eq52_e931) + (eq52_e926 * eq52_e931_d_n5));
        let eq52_e932_d_n6: f64 = ((eq52_e926_d_n6 * eq52_e931) + (eq52_e926 * eq52_e931_d_n6));
        let eq52_e932_d_n7: f64 = ((eq52_e926_d_n7 * eq52_e931) + (eq52_e926 * eq52_e931_d_n7));
        let eq52_e932_d_n8: f64 = ((eq52_e926_d_n8 * eq52_e931) + (eq52_e926 * eq52_e931_d_n8));
        let eq52_e932_d_n9: f64 = ((eq52_e926_d_n9 * eq52_e931) + (eq52_e926 * eq52_e931_d_n9));
        let eq52_e932_d_n10: f64 = ((eq52_e926_d_n10 * eq52_e931) + (eq52_e926 * eq52_e931_d_n10));
        let eq52_e932_d_n11: f64 = ((eq52_e926_d_n11 * eq52_e931) + (eq52_e926 * eq52_e931_d_n11));
        let eq52_e932_d_n12: f64 = ((eq52_e926_d_n12 * eq52_e931) + (eq52_e926 * eq52_e931_d_n12));
        let eq52_e932_d_n13: f64 = ((eq52_e926_d_n13 * eq52_e931) + (eq52_e926 * eq52_e931_d_n13));
        let eq52_e932_d_n14: f64 = ((eq52_e926_d_n14 * eq52_e931) + (eq52_e926 * eq52_e931_d_n14));
        let eq52_e932_d_n15: f64 = ((eq52_e926_d_n15 * eq52_e931) + (eq52_e926 * eq52_e931_d_n15));
        let eq52_e932_d_n16: f64 = ((eq52_e926_d_n16 * eq52_e931) + (eq52_e926 * eq52_e931_d_n16));
        let eq52_e932_d_n17: f64 = ((eq52_e926_d_n17 * eq52_e931) + (eq52_e926 * eq52_e931_d_n17));
        let eq52_e932_d_n18: f64 = ((eq52_e926_d_n18 * eq52_e931) + (eq52_e926 * eq52_e931_d_n18));
        let eq52_e932_d_n19: f64 = ((eq52_e926_d_n19 * eq52_e931) + (eq52_e926 * eq52_e931_d_n19));
        let eq52_e932_d_n20: f64 = ((eq52_e926_d_n20 * eq52_e931) + (eq52_e926 * eq52_e931_d_n20));
        let eq52_e932_d_n21: f64 = ((eq52_e926_d_n21 * eq52_e931) + (eq52_e926 * eq52_e931_d_n21));
        let eq52_e932_d_n22: f64 = ((eq52_e926_d_n22 * eq52_e931) + (eq52_e926 * eq52_e931_d_n22));
        let eq52_value: f64 = eq52_e932;
        let eq52_node_derivatives: [f64; 23] = [eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22];
        let eq52_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
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
        let (eq53_e938, eq53_e938_d_n0, eq53_e938_d_n1, eq53_e938_d_n2, eq53_e938_d_n3, eq53_e938_d_n4, eq53_e938_d_n5, eq53_e938_d_n6, eq53_e938_d_n7, eq53_e938_d_n8, eq53_e938_d_n9, eq53_e938_d_n10, eq53_e938_d_n11, eq53_e938_d_n12, eq53_e938_d_n13, eq53_e938_d_n14, eq53_e938_d_n15, eq53_e938_d_n16, eq53_e938_d_n17, eq53_e938_d_n18, eq53_e938_d_n19, eq53_e938_d_n20, eq53_e938_d_n21, eq53_e938_d_n22,) = {
    if (s.v[423] != 0.0) {
        let eq53_e936: f64 = (p.p6 * s.v[206]);
        let eq53_e936_d_n0: f64 = (p.p6 * s.dn[206][0]);
        let eq53_e936_d_n1: f64 = (p.p6 * s.dn[206][1]);
        let eq53_e936_d_n2: f64 = (p.p6 * s.dn[206][2]);
        let eq53_e936_d_n3: f64 = (p.p6 * s.dn[206][3]);
        let eq53_e936_d_n4: f64 = (p.p6 * s.dn[206][4]);
        let eq53_e936_d_n5: f64 = (p.p6 * s.dn[206][5]);
        let eq53_e936_d_n6: f64 = (p.p6 * s.dn[206][6]);
        let eq53_e936_d_n7: f64 = (p.p6 * s.dn[206][7]);
        let eq53_e936_d_n8: f64 = (p.p6 * s.dn[206][8]);
        let eq53_e936_d_n9: f64 = (p.p6 * s.dn[206][9]);
        let eq53_e936_d_n10: f64 = (p.p6 * s.dn[206][10]);
        let eq53_e936_d_n11: f64 = (p.p6 * s.dn[206][11]);
        let eq53_e936_d_n12: f64 = (p.p6 * s.dn[206][12]);
        let eq53_e936_d_n13: f64 = (p.p6 * s.dn[206][13]);
        let eq53_e936_d_n14: f64 = (p.p6 * s.dn[206][14]);
        let eq53_e936_d_n15: f64 = (p.p6 * s.dn[206][15]);
        let eq53_e936_d_n16: f64 = (p.p6 * s.dn[206][16]);
        let eq53_e936_d_n17: f64 = (p.p6 * s.dn[206][17]);
        let eq53_e936_d_n18: f64 = (p.p6 * s.dn[206][18]);
        let eq53_e936_d_n19: f64 = (p.p6 * s.dn[206][19]);
        let eq53_e936_d_n20: f64 = (p.p6 * s.dn[206][20]);
        let eq53_e936_d_n21: f64 = (p.p6 * s.dn[206][21]);
        let eq53_e936_d_n22: f64 = (p.p6 * s.dn[206][22]);
        (eq53_e936, eq53_e936_d_n0, eq53_e936_d_n1, eq53_e936_d_n2, eq53_e936_d_n3, eq53_e936_d_n4, eq53_e936_d_n5, eq53_e936_d_n6, eq53_e936_d_n7, eq53_e936_d_n8, eq53_e936_d_n9, eq53_e936_d_n10, eq53_e936_d_n11, eq53_e936_d_n12, eq53_e936_d_n13, eq53_e936_d_n14, eq53_e936_d_n15, eq53_e936_d_n16, eq53_e936_d_n17, eq53_e936_d_n18, eq53_e936_d_n19, eq53_e936_d_n20, eq53_e936_d_n21, eq53_e936_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e938;
        let eq53_node_derivatives: [f64; 23] = [eq53_e938_d_n0, eq53_e938_d_n1, eq53_e938_d_n2, eq53_e938_d_n3, eq53_e938_d_n4, eq53_e938_d_n5, eq53_e938_d_n6, eq53_e938_d_n7, eq53_e938_d_n8, eq53_e938_d_n9, eq53_e938_d_n10, eq53_e938_d_n11, eq53_e938_d_n12, eq53_e938_d_n13, eq53_e938_d_n14, eq53_e938_d_n15, eq53_e938_d_n16, eq53_e938_d_n17, eq53_e938_d_n18, eq53_e938_d_n19, eq53_e938_d_n20, eq53_e938_d_n21, eq53_e938_d_n22];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
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
        let (eq54_e944, eq54_e944_d_n0, eq54_e944_d_n1, eq54_e944_d_n2, eq54_e944_d_n3, eq54_e944_d_n4, eq54_e944_d_n5, eq54_e944_d_n6, eq54_e944_d_n7, eq54_e944_d_n8, eq54_e944_d_n9, eq54_e944_d_n10, eq54_e944_d_n11, eq54_e944_d_n12, eq54_e944_d_n13, eq54_e944_d_n14, eq54_e944_d_n15, eq54_e944_d_n16, eq54_e944_d_n17, eq54_e944_d_n18, eq54_e944_d_n19, eq54_e944_d_n20, eq54_e944_d_n21, eq54_e944_d_n22,) = {
    if (s.v[423] != 0.0) {
        let eq54_e942: f64 = (p.p6 * s.v[207]);
        let eq54_e942_d_n0: f64 = (p.p6 * s.dn[207][0]);
        let eq54_e942_d_n1: f64 = (p.p6 * s.dn[207][1]);
        let eq54_e942_d_n2: f64 = (p.p6 * s.dn[207][2]);
        let eq54_e942_d_n3: f64 = (p.p6 * s.dn[207][3]);
        let eq54_e942_d_n4: f64 = (p.p6 * s.dn[207][4]);
        let eq54_e942_d_n5: f64 = (p.p6 * s.dn[207][5]);
        let eq54_e942_d_n6: f64 = (p.p6 * s.dn[207][6]);
        let eq54_e942_d_n7: f64 = (p.p6 * s.dn[207][7]);
        let eq54_e942_d_n8: f64 = (p.p6 * s.dn[207][8]);
        let eq54_e942_d_n9: f64 = (p.p6 * s.dn[207][9]);
        let eq54_e942_d_n10: f64 = (p.p6 * s.dn[207][10]);
        let eq54_e942_d_n11: f64 = (p.p6 * s.dn[207][11]);
        let eq54_e942_d_n12: f64 = (p.p6 * s.dn[207][12]);
        let eq54_e942_d_n13: f64 = (p.p6 * s.dn[207][13]);
        let eq54_e942_d_n14: f64 = (p.p6 * s.dn[207][14]);
        let eq54_e942_d_n15: f64 = (p.p6 * s.dn[207][15]);
        let eq54_e942_d_n16: f64 = (p.p6 * s.dn[207][16]);
        let eq54_e942_d_n17: f64 = (p.p6 * s.dn[207][17]);
        let eq54_e942_d_n18: f64 = (p.p6 * s.dn[207][18]);
        let eq54_e942_d_n19: f64 = (p.p6 * s.dn[207][19]);
        let eq54_e942_d_n20: f64 = (p.p6 * s.dn[207][20]);
        let eq54_e942_d_n21: f64 = (p.p6 * s.dn[207][21]);
        let eq54_e942_d_n22: f64 = (p.p6 * s.dn[207][22]);
        (eq54_e942, eq54_e942_d_n0, eq54_e942_d_n1, eq54_e942_d_n2, eq54_e942_d_n3, eq54_e942_d_n4, eq54_e942_d_n5, eq54_e942_d_n6, eq54_e942_d_n7, eq54_e942_d_n8, eq54_e942_d_n9, eq54_e942_d_n10, eq54_e942_d_n11, eq54_e942_d_n12, eq54_e942_d_n13, eq54_e942_d_n14, eq54_e942_d_n15, eq54_e942_d_n16, eq54_e942_d_n17, eq54_e942_d_n18, eq54_e942_d_n19, eq54_e942_d_n20, eq54_e942_d_n21, eq54_e942_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e944;
        let eq54_node_derivatives: [f64; 23] = [eq54_e944_d_n0, eq54_e944_d_n1, eq54_e944_d_n2, eq54_e944_d_n3, eq54_e944_d_n4, eq54_e944_d_n5, eq54_e944_d_n6, eq54_e944_d_n7, eq54_e944_d_n8, eq54_e944_d_n9, eq54_e944_d_n10, eq54_e944_d_n11, eq54_e944_d_n12, eq54_e944_d_n13, eq54_e944_d_n14, eq54_e944_d_n15, eq54_e944_d_n16, eq54_e944_d_n17, eq54_e944_d_n18, eq54_e944_d_n19, eq54_e944_d_n20, eq54_e944_d_n21, eq54_e944_d_n22];
        let eq54_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq54_value),
            &nodes,
            &eq54_node_derivatives,
            &branches,
            &eq54_branch_derivatives,
            self.multiplicity,
        );
    }
}
