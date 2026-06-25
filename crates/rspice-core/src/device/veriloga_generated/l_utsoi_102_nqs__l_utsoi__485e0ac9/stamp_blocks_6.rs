#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq35_e710: f64 = self.eval_ddt(14, s.v[374]);
        let eq35_e710_d_n0: f64 = self.ddt_jacobian(s.dn[374][0]);
        let eq35_e710_d_n1: f64 = self.ddt_jacobian(s.dn[374][1]);
        let eq35_e710_d_n2: f64 = self.ddt_jacobian(s.dn[374][2]);
        let eq35_e710_d_n3: f64 = self.ddt_jacobian(s.dn[374][3]);
        let eq35_e710_d_n4: f64 = self.ddt_jacobian(s.dn[374][4]);
        let eq35_e710_d_n5: f64 = self.ddt_jacobian(s.dn[374][5]);
        let eq35_e710_d_n6: f64 = self.ddt_jacobian(s.dn[374][6]);
        let eq35_e710_d_n7: f64 = self.ddt_jacobian(s.dn[374][7]);
        let eq35_e710_d_n8: f64 = self.ddt_jacobian(s.dn[374][8]);
        let eq35_e710_d_n9: f64 = self.ddt_jacobian(s.dn[374][9]);
        let eq35_e710_d_n10: f64 = self.ddt_jacobian(s.dn[374][10]);
        let eq35_e710_d_n11: f64 = self.ddt_jacobian(s.dn[374][11]);
        let eq35_e710_d_n12: f64 = self.ddt_jacobian(s.dn[374][12]);
        let eq35_e710_d_n13: f64 = self.ddt_jacobian(s.dn[374][13]);
        let eq35_e711: f64 = (p.p14 * eq35_e710);
        let eq35_e711_d_n0: f64 = (p.p14 * eq35_e710_d_n0);
        let eq35_e711_d_n1: f64 = (p.p14 * eq35_e710_d_n1);
        let eq35_e711_d_n2: f64 = (p.p14 * eq35_e710_d_n2);
        let eq35_e711_d_n3: f64 = (p.p14 * eq35_e710_d_n3);
        let eq35_e711_d_n4: f64 = (p.p14 * eq35_e710_d_n4);
        let eq35_e711_d_n5: f64 = (p.p14 * eq35_e710_d_n5);
        let eq35_e711_d_n6: f64 = (p.p14 * eq35_e710_d_n6);
        let eq35_e711_d_n7: f64 = (p.p14 * eq35_e710_d_n7);
        let eq35_e711_d_n8: f64 = (p.p14 * eq35_e710_d_n8);
        let eq35_e711_d_n9: f64 = (p.p14 * eq35_e710_d_n9);
        let eq35_e711_d_n10: f64 = (p.p14 * eq35_e710_d_n10);
        let eq35_e711_d_n11: f64 = (p.p14 * eq35_e710_d_n11);
        let eq35_e711_d_n12: f64 = (p.p14 * eq35_e710_d_n12);
        let eq35_e711_d_n13: f64 = (p.p14 * eq35_e710_d_n13);
        let eq35_value: f64 = eq35_e711;
        let eq35_node_derivatives: [f64; 14] = [eq35_e711_d_n0, eq35_e711_d_n1, eq35_e711_d_n2, eq35_e711_d_n3, eq35_e711_d_n4, eq35_e711_d_n5, eq35_e711_d_n6, eq35_e711_d_n7, eq35_e711_d_n8, eq35_e711_d_n9, eq35_e711_d_n10, eq35_e711_d_n11, eq35_e711_d_n12, eq35_e711_d_n13];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq36_e714: f64 = (-s.v[1773]);
        let eq36_e714_d_n0: f64 = (-s.dn[1773][0]);
        let eq36_e714_d_n1: f64 = (-s.dn[1773][1]);
        let eq36_e714_d_n2: f64 = (-s.dn[1773][2]);
        let eq36_e714_d_n3: f64 = (-s.dn[1773][3]);
        let eq36_e714_d_n4: f64 = (-s.dn[1773][4]);
        let eq36_e714_d_n5: f64 = (-s.dn[1773][5]);
        let eq36_e714_d_n6: f64 = (-s.dn[1773][6]);
        let eq36_e714_d_n7: f64 = (-s.dn[1773][7]);
        let eq36_e714_d_n8: f64 = (-s.dn[1773][8]);
        let eq36_e714_d_n9: f64 = (-s.dn[1773][9]);
        let eq36_e714_d_n10: f64 = (-s.dn[1773][10]);
        let eq36_e714_d_n11: f64 = (-s.dn[1773][11]);
        let eq36_e714_d_n12: f64 = (-s.dn[1773][12]);
        let eq36_e714_d_n13: f64 = (-s.dn[1773][13]);
        let eq36_e716: f64 = (eq36_e714 * p.p32);
        let eq36_e716_d_n0: f64 = (eq36_e714_d_n0 * p.p32);
        let eq36_e716_d_n1: f64 = (eq36_e714_d_n1 * p.p32);
        let eq36_e716_d_n2: f64 = (eq36_e714_d_n2 * p.p32);
        let eq36_e716_d_n3: f64 = (eq36_e714_d_n3 * p.p32);
        let eq36_e716_d_n4: f64 = (eq36_e714_d_n4 * p.p32);
        let eq36_e716_d_n5: f64 = (eq36_e714_d_n5 * p.p32);
        let eq36_e716_d_n6: f64 = (eq36_e714_d_n6 * p.p32);
        let eq36_e716_d_n7: f64 = (eq36_e714_d_n7 * p.p32);
        let eq36_e716_d_n8: f64 = (eq36_e714_d_n8 * p.p32);
        let eq36_e716_d_n9: f64 = (eq36_e714_d_n9 * p.p32);
        let eq36_e716_d_n10: f64 = (eq36_e714_d_n10 * p.p32);
        let eq36_e716_d_n11: f64 = (eq36_e714_d_n11 * p.p32);
        let eq36_e716_d_n12: f64 = (eq36_e714_d_n12 * p.p32);
        let eq36_e716_d_n13: f64 = (eq36_e714_d_n13 * p.p32);
        let eq36_e718: f64 = (eq36_e716 * s.v[13]);
        let eq36_e718_d_n0: f64 = ((eq36_e716_d_n0 * s.v[13]) + (eq36_e716 * s.dn[13][0]));
        let eq36_e718_d_n1: f64 = ((eq36_e716_d_n1 * s.v[13]) + (eq36_e716 * s.dn[13][1]));
        let eq36_e718_d_n2: f64 = ((eq36_e716_d_n2 * s.v[13]) + (eq36_e716 * s.dn[13][2]));
        let eq36_e718_d_n3: f64 = ((eq36_e716_d_n3 * s.v[13]) + (eq36_e716 * s.dn[13][3]));
        let eq36_e718_d_n4: f64 = ((eq36_e716_d_n4 * s.v[13]) + (eq36_e716 * s.dn[13][4]));
        let eq36_e718_d_n5: f64 = ((eq36_e716_d_n5 * s.v[13]) + (eq36_e716 * s.dn[13][5]));
        let eq36_e718_d_n6: f64 = ((eq36_e716_d_n6 * s.v[13]) + (eq36_e716 * s.dn[13][6]));
        let eq36_e718_d_n7: f64 = ((eq36_e716_d_n7 * s.v[13]) + (eq36_e716 * s.dn[13][7]));
        let eq36_e718_d_n8: f64 = ((eq36_e716_d_n8 * s.v[13]) + (eq36_e716 * s.dn[13][8]));
        let eq36_e718_d_n9: f64 = ((eq36_e716_d_n9 * s.v[13]) + (eq36_e716 * s.dn[13][9]));
        let eq36_e718_d_n10: f64 = ((eq36_e716_d_n10 * s.v[13]) + (eq36_e716 * s.dn[13][10]));
        let eq36_e718_d_n11: f64 = ((eq36_e716_d_n11 * s.v[13]) + (eq36_e716 * s.dn[13][11]));
        let eq36_e718_d_n12: f64 = ((eq36_e716_d_n12 * s.v[13]) + (eq36_e716 * s.dn[13][12]));
        let eq36_e718_d_n13: f64 = ((eq36_e716_d_n13 * s.v[13]) + (eq36_e716 * s.dn[13][13]));
        let eq36_e722: f64 = (s.v[182]).sqrt();
        let eq36_e722_d_n0: f64 = (s.dn[182][0] / (2.0 * eq36_e722));
        let eq36_e722_d_n1: f64 = (s.dn[182][1] / (2.0 * eq36_e722));
        let eq36_e722_d_n2: f64 = (s.dn[182][2] / (2.0 * eq36_e722));
        let eq36_e722_d_n3: f64 = (s.dn[182][3] / (2.0 * eq36_e722));
        let eq36_e722_d_n4: f64 = (s.dn[182][4] / (2.0 * eq36_e722));
        let eq36_e722_d_n5: f64 = (s.dn[182][5] / (2.0 * eq36_e722));
        let eq36_e722_d_n6: f64 = (s.dn[182][6] / (2.0 * eq36_e722));
        let eq36_e722_d_n7: f64 = (s.dn[182][7] / (2.0 * eq36_e722));
        let eq36_e722_d_n8: f64 = (s.dn[182][8] / (2.0 * eq36_e722));
        let eq36_e722_d_n9: f64 = (s.dn[182][9] / (2.0 * eq36_e722));
        let eq36_e722_d_n10: f64 = (s.dn[182][10] / (2.0 * eq36_e722));
        let eq36_e722_d_n11: f64 = (s.dn[182][11] / (2.0 * eq36_e722));
        let eq36_e722_d_n12: f64 = (s.dn[182][12] / (2.0 * eq36_e722));
        let eq36_e722_d_n13: f64 = (s.dn[182][13] / (2.0 * eq36_e722));
        let eq36_e723: f64 = ((nv11 - nv13) / eq36_e722);
        let eq36_e723_d_n0: f64 = (-(((nv11 - nv13) * eq36_e722_d_n0) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n1: f64 = (-(((nv11 - nv13) * eq36_e722_d_n1) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n2: f64 = (-(((nv11 - nv13) * eq36_e722_d_n2) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n3: f64 = (-(((nv11 - nv13) * eq36_e722_d_n3) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n4: f64 = (-(((nv11 - nv13) * eq36_e722_d_n4) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n5: f64 = (-(((nv11 - nv13) * eq36_e722_d_n5) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n6: f64 = (-(((nv11 - nv13) * eq36_e722_d_n6) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n7: f64 = (-(((nv11 - nv13) * eq36_e722_d_n7) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n8: f64 = (-(((nv11 - nv13) * eq36_e722_d_n8) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n9: f64 = (-(((nv11 - nv13) * eq36_e722_d_n9) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n10: f64 = (-(((nv11 - nv13) * eq36_e722_d_n10) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n11: f64 = ((eq36_e722 - ((nv11 - nv13) * eq36_e722_d_n11)) / (eq36_e722 * eq36_e722));
        let eq36_e723_d_n12: f64 = (-(((nv11 - nv13) * eq36_e722_d_n12) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n13: f64 = (((-eq36_e722) - ((nv11 - nv13) * eq36_e722_d_n13)) / (eq36_e722 * eq36_e722));
        let eq36_e724: f64 = ((nv10 - nv13) + eq36_e723);
        let eq36_e724_d_n10: f64 = (1.0 + eq36_e723_d_n10);
        let eq36_e724_d_n13: f64 = (-1.0 + eq36_e723_d_n13);
        let eq36_e725: f64 = (eq36_e718 * eq36_e724);
        let eq36_e725_d_n0: f64 = ((eq36_e718_d_n0 * eq36_e724) + (eq36_e718 * eq36_e723_d_n0));
        let eq36_e725_d_n1: f64 = ((eq36_e718_d_n1 * eq36_e724) + (eq36_e718 * eq36_e723_d_n1));
        let eq36_e725_d_n2: f64 = ((eq36_e718_d_n2 * eq36_e724) + (eq36_e718 * eq36_e723_d_n2));
        let eq36_e725_d_n3: f64 = ((eq36_e718_d_n3 * eq36_e724) + (eq36_e718 * eq36_e723_d_n3));
        let eq36_e725_d_n4: f64 = ((eq36_e718_d_n4 * eq36_e724) + (eq36_e718 * eq36_e723_d_n4));
        let eq36_e725_d_n5: f64 = ((eq36_e718_d_n5 * eq36_e724) + (eq36_e718 * eq36_e723_d_n5));
        let eq36_e725_d_n6: f64 = ((eq36_e718_d_n6 * eq36_e724) + (eq36_e718 * eq36_e723_d_n6));
        let eq36_e725_d_n7: f64 = ((eq36_e718_d_n7 * eq36_e724) + (eq36_e718 * eq36_e723_d_n7));
        let eq36_e725_d_n8: f64 = ((eq36_e718_d_n8 * eq36_e724) + (eq36_e718 * eq36_e723_d_n8));
        let eq36_e725_d_n9: f64 = ((eq36_e718_d_n9 * eq36_e724) + (eq36_e718 * eq36_e723_d_n9));
        let eq36_e725_d_n10: f64 = ((eq36_e718_d_n10 * eq36_e724) + (eq36_e718 * eq36_e724_d_n10));
        let eq36_e725_d_n11: f64 = ((eq36_e718_d_n11 * eq36_e724) + (eq36_e718 * eq36_e723_d_n11));
        let eq36_e725_d_n12: f64 = ((eq36_e718_d_n12 * eq36_e724) + (eq36_e718 * eq36_e723_d_n12));
        let eq36_e725_d_n13: f64 = ((eq36_e718_d_n13 * eq36_e724) + (eq36_e718 * eq36_e724_d_n13));
        let eq36_e727: f64 = self.eval_ddt(15, s.v[362]);
        let eq36_e727_d_n0: f64 = self.ddt_jacobian(s.dn[362][0]);
        let eq36_e727_d_n1: f64 = self.ddt_jacobian(s.dn[362][1]);
        let eq36_e727_d_n2: f64 = self.ddt_jacobian(s.dn[362][2]);
        let eq36_e727_d_n3: f64 = self.ddt_jacobian(s.dn[362][3]);
        let eq36_e727_d_n4: f64 = self.ddt_jacobian(s.dn[362][4]);
        let eq36_e727_d_n5: f64 = self.ddt_jacobian(s.dn[362][5]);
        let eq36_e727_d_n6: f64 = self.ddt_jacobian(s.dn[362][6]);
        let eq36_e727_d_n7: f64 = self.ddt_jacobian(s.dn[362][7]);
        let eq36_e727_d_n8: f64 = self.ddt_jacobian(s.dn[362][8]);
        let eq36_e727_d_n9: f64 = self.ddt_jacobian(s.dn[362][9]);
        let eq36_e727_d_n10: f64 = self.ddt_jacobian(s.dn[362][10]);
        let eq36_e727_d_n11: f64 = self.ddt_jacobian(s.dn[362][11]);
        let eq36_e727_d_n12: f64 = self.ddt_jacobian(s.dn[362][12]);
        let eq36_e727_d_n13: f64 = self.ddt_jacobian(s.dn[362][13]);
        let eq36_e728: f64 = (eq36_e725 - eq36_e727);
        let eq36_e728_d_n0: f64 = (eq36_e725_d_n0 - eq36_e727_d_n0);
        let eq36_e728_d_n1: f64 = (eq36_e725_d_n1 - eq36_e727_d_n1);
        let eq36_e728_d_n2: f64 = (eq36_e725_d_n2 - eq36_e727_d_n2);
        let eq36_e728_d_n3: f64 = (eq36_e725_d_n3 - eq36_e727_d_n3);
        let eq36_e728_d_n4: f64 = (eq36_e725_d_n4 - eq36_e727_d_n4);
        let eq36_e728_d_n5: f64 = (eq36_e725_d_n5 - eq36_e727_d_n5);
        let eq36_e728_d_n6: f64 = (eq36_e725_d_n6 - eq36_e727_d_n6);
        let eq36_e728_d_n7: f64 = (eq36_e725_d_n7 - eq36_e727_d_n7);
        let eq36_e728_d_n8: f64 = (eq36_e725_d_n8 - eq36_e727_d_n8);
        let eq36_e728_d_n9: f64 = (eq36_e725_d_n9 - eq36_e727_d_n9);
        let eq36_e728_d_n10: f64 = (eq36_e725_d_n10 - eq36_e727_d_n10);
        let eq36_e728_d_n11: f64 = (eq36_e725_d_n11 - eq36_e727_d_n11);
        let eq36_e728_d_n12: f64 = (eq36_e725_d_n12 - eq36_e727_d_n12);
        let eq36_e728_d_n13: f64 = (eq36_e725_d_n13 - eq36_e727_d_n13);
        let eq36_e730: f64 = self.eval_ddt(16, s.v[370]);
        let eq36_e730_d_n0: f64 = self.ddt_jacobian(s.dn[370][0]);
        let eq36_e730_d_n1: f64 = self.ddt_jacobian(s.dn[370][1]);
        let eq36_e730_d_n2: f64 = self.ddt_jacobian(s.dn[370][2]);
        let eq36_e730_d_n3: f64 = self.ddt_jacobian(s.dn[370][3]);
        let eq36_e730_d_n4: f64 = self.ddt_jacobian(s.dn[370][4]);
        let eq36_e730_d_n5: f64 = self.ddt_jacobian(s.dn[370][5]);
        let eq36_e730_d_n6: f64 = self.ddt_jacobian(s.dn[370][6]);
        let eq36_e730_d_n7: f64 = self.ddt_jacobian(s.dn[370][7]);
        let eq36_e730_d_n8: f64 = self.ddt_jacobian(s.dn[370][8]);
        let eq36_e730_d_n9: f64 = self.ddt_jacobian(s.dn[370][9]);
        let eq36_e730_d_n10: f64 = self.ddt_jacobian(s.dn[370][10]);
        let eq36_e730_d_n11: f64 = self.ddt_jacobian(s.dn[370][11]);
        let eq36_e730_d_n12: f64 = self.ddt_jacobian(s.dn[370][12]);
        let eq36_e730_d_n13: f64 = self.ddt_jacobian(s.dn[370][13]);
        let eq36_e731: f64 = (eq36_e728 + eq36_e730);
        let eq36_e731_d_n0: f64 = (eq36_e728_d_n0 + eq36_e730_d_n0);
        let eq36_e731_d_n1: f64 = (eq36_e728_d_n1 + eq36_e730_d_n1);
        let eq36_e731_d_n2: f64 = (eq36_e728_d_n2 + eq36_e730_d_n2);
        let eq36_e731_d_n3: f64 = (eq36_e728_d_n3 + eq36_e730_d_n3);
        let eq36_e731_d_n4: f64 = (eq36_e728_d_n4 + eq36_e730_d_n4);
        let eq36_e731_d_n5: f64 = (eq36_e728_d_n5 + eq36_e730_d_n5);
        let eq36_e731_d_n6: f64 = (eq36_e728_d_n6 + eq36_e730_d_n6);
        let eq36_e731_d_n7: f64 = (eq36_e728_d_n7 + eq36_e730_d_n7);
        let eq36_e731_d_n8: f64 = (eq36_e728_d_n8 + eq36_e730_d_n8);
        let eq36_e731_d_n9: f64 = (eq36_e728_d_n9 + eq36_e730_d_n9);
        let eq36_e731_d_n10: f64 = (eq36_e728_d_n10 + eq36_e730_d_n10);
        let eq36_e731_d_n11: f64 = (eq36_e728_d_n11 + eq36_e730_d_n11);
        let eq36_e731_d_n12: f64 = (eq36_e728_d_n12 + eq36_e730_d_n12);
        let eq36_e731_d_n13: f64 = (eq36_e728_d_n13 + eq36_e730_d_n13);
        let eq36_e733: f64 = self.eval_ddt(17, s.v[372]);
        let eq36_e733_d_n0: f64 = self.ddt_jacobian(s.dn[372][0]);
        let eq36_e733_d_n1: f64 = self.ddt_jacobian(s.dn[372][1]);
        let eq36_e733_d_n2: f64 = self.ddt_jacobian(s.dn[372][2]);
        let eq36_e733_d_n3: f64 = self.ddt_jacobian(s.dn[372][3]);
        let eq36_e733_d_n4: f64 = self.ddt_jacobian(s.dn[372][4]);
        let eq36_e733_d_n5: f64 = self.ddt_jacobian(s.dn[372][5]);
        let eq36_e733_d_n6: f64 = self.ddt_jacobian(s.dn[372][6]);
        let eq36_e733_d_n7: f64 = self.ddt_jacobian(s.dn[372][7]);
        let eq36_e733_d_n8: f64 = self.ddt_jacobian(s.dn[372][8]);
        let eq36_e733_d_n9: f64 = self.ddt_jacobian(s.dn[372][9]);
        let eq36_e733_d_n10: f64 = self.ddt_jacobian(s.dn[372][10]);
        let eq36_e733_d_n11: f64 = self.ddt_jacobian(s.dn[372][11]);
        let eq36_e733_d_n12: f64 = self.ddt_jacobian(s.dn[372][12]);
        let eq36_e733_d_n13: f64 = self.ddt_jacobian(s.dn[372][13]);
        let eq36_e734: f64 = (eq36_e731 + eq36_e733);
        let eq36_e734_d_n0: f64 = (eq36_e731_d_n0 + eq36_e733_d_n0);
        let eq36_e734_d_n1: f64 = (eq36_e731_d_n1 + eq36_e733_d_n1);
        let eq36_e734_d_n2: f64 = (eq36_e731_d_n2 + eq36_e733_d_n2);
        let eq36_e734_d_n3: f64 = (eq36_e731_d_n3 + eq36_e733_d_n3);
        let eq36_e734_d_n4: f64 = (eq36_e731_d_n4 + eq36_e733_d_n4);
        let eq36_e734_d_n5: f64 = (eq36_e731_d_n5 + eq36_e733_d_n5);
        let eq36_e734_d_n6: f64 = (eq36_e731_d_n6 + eq36_e733_d_n6);
        let eq36_e734_d_n7: f64 = (eq36_e731_d_n7 + eq36_e733_d_n7);
        let eq36_e734_d_n8: f64 = (eq36_e731_d_n8 + eq36_e733_d_n8);
        let eq36_e734_d_n9: f64 = (eq36_e731_d_n9 + eq36_e733_d_n9);
        let eq36_e734_d_n10: f64 = (eq36_e731_d_n10 + eq36_e733_d_n10);
        let eq36_e734_d_n11: f64 = (eq36_e731_d_n11 + eq36_e733_d_n11);
        let eq36_e734_d_n12: f64 = (eq36_e731_d_n12 + eq36_e733_d_n12);
        let eq36_e734_d_n13: f64 = (eq36_e731_d_n13 + eq36_e733_d_n13);
        let eq36_e736: f64 = self.eval_ddt(18, s.v[379]);
        let eq36_e736_d_n0: f64 = self.ddt_jacobian(s.dn[379][0]);
        let eq36_e736_d_n1: f64 = self.ddt_jacobian(s.dn[379][1]);
        let eq36_e736_d_n2: f64 = self.ddt_jacobian(s.dn[379][2]);
        let eq36_e736_d_n3: f64 = self.ddt_jacobian(s.dn[379][3]);
        let eq36_e736_d_n4: f64 = self.ddt_jacobian(s.dn[379][4]);
        let eq36_e736_d_n5: f64 = self.ddt_jacobian(s.dn[379][5]);
        let eq36_e736_d_n6: f64 = self.ddt_jacobian(s.dn[379][6]);
        let eq36_e736_d_n7: f64 = self.ddt_jacobian(s.dn[379][7]);
        let eq36_e736_d_n8: f64 = self.ddt_jacobian(s.dn[379][8]);
        let eq36_e736_d_n9: f64 = self.ddt_jacobian(s.dn[379][9]);
        let eq36_e736_d_n10: f64 = self.ddt_jacobian(s.dn[379][10]);
        let eq36_e736_d_n11: f64 = self.ddt_jacobian(s.dn[379][11]);
        let eq36_e736_d_n12: f64 = self.ddt_jacobian(s.dn[379][12]);
        let eq36_e736_d_n13: f64 = self.ddt_jacobian(s.dn[379][13]);
        let eq36_e737: f64 = (eq36_e734 + eq36_e736);
        let eq36_e737_d_n0: f64 = (eq36_e734_d_n0 + eq36_e736_d_n0);
        let eq36_e737_d_n1: f64 = (eq36_e734_d_n1 + eq36_e736_d_n1);
        let eq36_e737_d_n2: f64 = (eq36_e734_d_n2 + eq36_e736_d_n2);
        let eq36_e737_d_n3: f64 = (eq36_e734_d_n3 + eq36_e736_d_n3);
        let eq36_e737_d_n4: f64 = (eq36_e734_d_n4 + eq36_e736_d_n4);
        let eq36_e737_d_n5: f64 = (eq36_e734_d_n5 + eq36_e736_d_n5);
        let eq36_e737_d_n6: f64 = (eq36_e734_d_n6 + eq36_e736_d_n6);
        let eq36_e737_d_n7: f64 = (eq36_e734_d_n7 + eq36_e736_d_n7);
        let eq36_e737_d_n8: f64 = (eq36_e734_d_n8 + eq36_e736_d_n8);
        let eq36_e737_d_n9: f64 = (eq36_e734_d_n9 + eq36_e736_d_n9);
        let eq36_e737_d_n10: f64 = (eq36_e734_d_n10 + eq36_e736_d_n10);
        let eq36_e737_d_n11: f64 = (eq36_e734_d_n11 + eq36_e736_d_n11);
        let eq36_e737_d_n12: f64 = (eq36_e734_d_n12 + eq36_e736_d_n12);
        let eq36_e737_d_n13: f64 = (eq36_e734_d_n13 + eq36_e736_d_n13);
        let eq36_e738: f64 = (p.p14 * eq36_e737);
        let eq36_e738_d_n0: f64 = (p.p14 * eq36_e737_d_n0);
        let eq36_e738_d_n1: f64 = (p.p14 * eq36_e737_d_n1);
        let eq36_e738_d_n2: f64 = (p.p14 * eq36_e737_d_n2);
        let eq36_e738_d_n3: f64 = (p.p14 * eq36_e737_d_n3);
        let eq36_e738_d_n4: f64 = (p.p14 * eq36_e737_d_n4);
        let eq36_e738_d_n5: f64 = (p.p14 * eq36_e737_d_n5);
        let eq36_e738_d_n6: f64 = (p.p14 * eq36_e737_d_n6);
        let eq36_e738_d_n7: f64 = (p.p14 * eq36_e737_d_n7);
        let eq36_e738_d_n8: f64 = (p.p14 * eq36_e737_d_n8);
        let eq36_e738_d_n9: f64 = (p.p14 * eq36_e737_d_n9);
        let eq36_e738_d_n10: f64 = (p.p14 * eq36_e737_d_n10);
        let eq36_e738_d_n11: f64 = (p.p14 * eq36_e737_d_n11);
        let eq36_e738_d_n12: f64 = (p.p14 * eq36_e737_d_n12);
        let eq36_e738_d_n13: f64 = (p.p14 * eq36_e737_d_n13);
        let eq36_value: f64 = eq36_e738;
        let eq36_node_derivatives: [f64; 14] = [eq36_e738_d_n0, eq36_e738_d_n1, eq36_e738_d_n2, eq36_e738_d_n3, eq36_e738_d_n4, eq36_e738_d_n5, eq36_e738_d_n6, eq36_e738_d_n7, eq36_e738_d_n8, eq36_e738_d_n9, eq36_e738_d_n10, eq36_e738_d_n11, eq36_e738_d_n12, eq36_e738_d_n13];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq37_e741: f64 = (-s.v[1773]);
        let eq37_e741_d_n0: f64 = (-s.dn[1773][0]);
        let eq37_e741_d_n1: f64 = (-s.dn[1773][1]);
        let eq37_e741_d_n2: f64 = (-s.dn[1773][2]);
        let eq37_e741_d_n3: f64 = (-s.dn[1773][3]);
        let eq37_e741_d_n4: f64 = (-s.dn[1773][4]);
        let eq37_e741_d_n5: f64 = (-s.dn[1773][5]);
        let eq37_e741_d_n6: f64 = (-s.dn[1773][6]);
        let eq37_e741_d_n7: f64 = (-s.dn[1773][7]);
        let eq37_e741_d_n8: f64 = (-s.dn[1773][8]);
        let eq37_e741_d_n9: f64 = (-s.dn[1773][9]);
        let eq37_e741_d_n10: f64 = (-s.dn[1773][10]);
        let eq37_e741_d_n11: f64 = (-s.dn[1773][11]);
        let eq37_e741_d_n12: f64 = (-s.dn[1773][12]);
        let eq37_e741_d_n13: f64 = (-s.dn[1773][13]);
        let eq37_e743: f64 = (eq37_e741 * p.p31);
        let eq37_e743_d_n0: f64 = (eq37_e741_d_n0 * p.p31);
        let eq37_e743_d_n1: f64 = (eq37_e741_d_n1 * p.p31);
        let eq37_e743_d_n2: f64 = (eq37_e741_d_n2 * p.p31);
        let eq37_e743_d_n3: f64 = (eq37_e741_d_n3 * p.p31);
        let eq37_e743_d_n4: f64 = (eq37_e741_d_n4 * p.p31);
        let eq37_e743_d_n5: f64 = (eq37_e741_d_n5 * p.p31);
        let eq37_e743_d_n6: f64 = (eq37_e741_d_n6 * p.p31);
        let eq37_e743_d_n7: f64 = (eq37_e741_d_n7 * p.p31);
        let eq37_e743_d_n8: f64 = (eq37_e741_d_n8 * p.p31);
        let eq37_e743_d_n9: f64 = (eq37_e741_d_n9 * p.p31);
        let eq37_e743_d_n10: f64 = (eq37_e741_d_n10 * p.p31);
        let eq37_e743_d_n11: f64 = (eq37_e741_d_n11 * p.p31);
        let eq37_e743_d_n12: f64 = (eq37_e741_d_n12 * p.p31);
        let eq37_e743_d_n13: f64 = (eq37_e741_d_n13 * p.p31);
        let eq37_e745: f64 = (eq37_e743 * s.v[13]);
        let eq37_e745_d_n0: f64 = ((eq37_e743_d_n0 * s.v[13]) + (eq37_e743 * s.dn[13][0]));
        let eq37_e745_d_n1: f64 = ((eq37_e743_d_n1 * s.v[13]) + (eq37_e743 * s.dn[13][1]));
        let eq37_e745_d_n2: f64 = ((eq37_e743_d_n2 * s.v[13]) + (eq37_e743 * s.dn[13][2]));
        let eq37_e745_d_n3: f64 = ((eq37_e743_d_n3 * s.v[13]) + (eq37_e743 * s.dn[13][3]));
        let eq37_e745_d_n4: f64 = ((eq37_e743_d_n4 * s.v[13]) + (eq37_e743 * s.dn[13][4]));
        let eq37_e745_d_n5: f64 = ((eq37_e743_d_n5 * s.v[13]) + (eq37_e743 * s.dn[13][5]));
        let eq37_e745_d_n6: f64 = ((eq37_e743_d_n6 * s.v[13]) + (eq37_e743 * s.dn[13][6]));
        let eq37_e745_d_n7: f64 = ((eq37_e743_d_n7 * s.v[13]) + (eq37_e743 * s.dn[13][7]));
        let eq37_e745_d_n8: f64 = ((eq37_e743_d_n8 * s.v[13]) + (eq37_e743 * s.dn[13][8]));
        let eq37_e745_d_n9: f64 = ((eq37_e743_d_n9 * s.v[13]) + (eq37_e743 * s.dn[13][9]));
        let eq37_e745_d_n10: f64 = ((eq37_e743_d_n10 * s.v[13]) + (eq37_e743 * s.dn[13][10]));
        let eq37_e745_d_n11: f64 = ((eq37_e743_d_n11 * s.v[13]) + (eq37_e743 * s.dn[13][11]));
        let eq37_e745_d_n12: f64 = ((eq37_e743_d_n12 * s.v[13]) + (eq37_e743 * s.dn[13][12]));
        let eq37_e745_d_n13: f64 = ((eq37_e743_d_n13 * s.v[13]) + (eq37_e743 * s.dn[13][13]));
        let eq37_e747: f64 = (eq37_e745 * (nv12 - nv13));
        let eq37_e747_d_n0: f64 = (eq37_e745_d_n0 * (nv12 - nv13));
        let eq37_e747_d_n1: f64 = (eq37_e745_d_n1 * (nv12 - nv13));
        let eq37_e747_d_n2: f64 = (eq37_e745_d_n2 * (nv12 - nv13));
        let eq37_e747_d_n3: f64 = (eq37_e745_d_n3 * (nv12 - nv13));
        let eq37_e747_d_n4: f64 = (eq37_e745_d_n4 * (nv12 - nv13));
        let eq37_e747_d_n5: f64 = (eq37_e745_d_n5 * (nv12 - nv13));
        let eq37_e747_d_n6: f64 = (eq37_e745_d_n6 * (nv12 - nv13));
        let eq37_e747_d_n7: f64 = (eq37_e745_d_n7 * (nv12 - nv13));
        let eq37_e747_d_n8: f64 = (eq37_e745_d_n8 * (nv12 - nv13));
        let eq37_e747_d_n9: f64 = (eq37_e745_d_n9 * (nv12 - nv13));
        let eq37_e747_d_n10: f64 = (eq37_e745_d_n10 * (nv12 - nv13));
        let eq37_e747_d_n11: f64 = (eq37_e745_d_n11 * (nv12 - nv13));
        let eq37_e747_d_n12: f64 = ((eq37_e745_d_n12 * (nv12 - nv13)) + eq37_e745);
        let eq37_e747_d_n13: f64 = ((eq37_e745_d_n13 * (nv12 - nv13)) + (-eq37_e745));
        let eq37_e749: f64 = self.eval_ddt(19, s.v[375]);
        let eq37_e749_d_n0: f64 = self.ddt_jacobian(s.dn[375][0]);
        let eq37_e749_d_n1: f64 = self.ddt_jacobian(s.dn[375][1]);
        let eq37_e749_d_n2: f64 = self.ddt_jacobian(s.dn[375][2]);
        let eq37_e749_d_n3: f64 = self.ddt_jacobian(s.dn[375][3]);
        let eq37_e749_d_n4: f64 = self.ddt_jacobian(s.dn[375][4]);
        let eq37_e749_d_n5: f64 = self.ddt_jacobian(s.dn[375][5]);
        let eq37_e749_d_n6: f64 = self.ddt_jacobian(s.dn[375][6]);
        let eq37_e749_d_n7: f64 = self.ddt_jacobian(s.dn[375][7]);
        let eq37_e749_d_n8: f64 = self.ddt_jacobian(s.dn[375][8]);
        let eq37_e749_d_n9: f64 = self.ddt_jacobian(s.dn[375][9]);
        let eq37_e749_d_n10: f64 = self.ddt_jacobian(s.dn[375][10]);
        let eq37_e749_d_n11: f64 = self.ddt_jacobian(s.dn[375][11]);
        let eq37_e749_d_n12: f64 = self.ddt_jacobian(s.dn[375][12]);
        let eq37_e749_d_n13: f64 = self.ddt_jacobian(s.dn[375][13]);
        let eq37_e750: f64 = (eq37_e747 + eq37_e749);
        let eq37_e750_d_n0: f64 = (eq37_e747_d_n0 + eq37_e749_d_n0);
        let eq37_e750_d_n1: f64 = (eq37_e747_d_n1 + eq37_e749_d_n1);
        let eq37_e750_d_n2: f64 = (eq37_e747_d_n2 + eq37_e749_d_n2);
        let eq37_e750_d_n3: f64 = (eq37_e747_d_n3 + eq37_e749_d_n3);
        let eq37_e750_d_n4: f64 = (eq37_e747_d_n4 + eq37_e749_d_n4);
        let eq37_e750_d_n5: f64 = (eq37_e747_d_n5 + eq37_e749_d_n5);
        let eq37_e750_d_n6: f64 = (eq37_e747_d_n6 + eq37_e749_d_n6);
        let eq37_e750_d_n7: f64 = (eq37_e747_d_n7 + eq37_e749_d_n7);
        let eq37_e750_d_n8: f64 = (eq37_e747_d_n8 + eq37_e749_d_n8);
        let eq37_e750_d_n9: f64 = (eq37_e747_d_n9 + eq37_e749_d_n9);
        let eq37_e750_d_n10: f64 = (eq37_e747_d_n10 + eq37_e749_d_n10);
        let eq37_e750_d_n11: f64 = (eq37_e747_d_n11 + eq37_e749_d_n11);
        let eq37_e750_d_n12: f64 = (eq37_e747_d_n12 + eq37_e749_d_n12);
        let eq37_e750_d_n13: f64 = (eq37_e747_d_n13 + eq37_e749_d_n13);
        let eq37_e751: f64 = (p.p14 * eq37_e750);
        let eq37_e751_d_n0: f64 = (p.p14 * eq37_e750_d_n0);
        let eq37_e751_d_n1: f64 = (p.p14 * eq37_e750_d_n1);
        let eq37_e751_d_n2: f64 = (p.p14 * eq37_e750_d_n2);
        let eq37_e751_d_n3: f64 = (p.p14 * eq37_e750_d_n3);
        let eq37_e751_d_n4: f64 = (p.p14 * eq37_e750_d_n4);
        let eq37_e751_d_n5: f64 = (p.p14 * eq37_e750_d_n5);
        let eq37_e751_d_n6: f64 = (p.p14 * eq37_e750_d_n6);
        let eq37_e751_d_n7: f64 = (p.p14 * eq37_e750_d_n7);
        let eq37_e751_d_n8: f64 = (p.p14 * eq37_e750_d_n8);
        let eq37_e751_d_n9: f64 = (p.p14 * eq37_e750_d_n9);
        let eq37_e751_d_n10: f64 = (p.p14 * eq37_e750_d_n10);
        let eq37_e751_d_n11: f64 = (p.p14 * eq37_e750_d_n11);
        let eq37_e751_d_n12: f64 = (p.p14 * eq37_e750_d_n12);
        let eq37_e751_d_n13: f64 = (p.p14 * eq37_e750_d_n13);
        let eq37_value: f64 = eq37_e751;
        let eq37_node_derivatives: [f64; 14] = [eq37_e751_d_n0, eq37_e751_d_n1, eq37_e751_d_n2, eq37_e751_d_n3, eq37_e751_d_n4, eq37_e751_d_n5, eq37_e751_d_n6, eq37_e751_d_n7, eq37_e751_d_n8, eq37_e751_d_n9, eq37_e751_d_n10, eq37_e751_d_n11, eq37_e751_d_n12, eq37_e751_d_n13];
        let eq37_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
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
        let eq38_e753: f64 = self.eval_ddt(20, s.v[378]);
        let eq38_e753_d_n0: f64 = self.ddt_jacobian(s.dn[378][0]);
        let eq38_e753_d_n1: f64 = self.ddt_jacobian(s.dn[378][1]);
        let eq38_e753_d_n2: f64 = self.ddt_jacobian(s.dn[378][2]);
        let eq38_e753_d_n3: f64 = self.ddt_jacobian(s.dn[378][3]);
        let eq38_e753_d_n4: f64 = self.ddt_jacobian(s.dn[378][4]);
        let eq38_e753_d_n5: f64 = self.ddt_jacobian(s.dn[378][5]);
        let eq38_e753_d_n6: f64 = self.ddt_jacobian(s.dn[378][6]);
        let eq38_e753_d_n7: f64 = self.ddt_jacobian(s.dn[378][7]);
        let eq38_e753_d_n8: f64 = self.ddt_jacobian(s.dn[378][8]);
        let eq38_e753_d_n9: f64 = self.ddt_jacobian(s.dn[378][9]);
        let eq38_e753_d_n10: f64 = self.ddt_jacobian(s.dn[378][10]);
        let eq38_e753_d_n11: f64 = self.ddt_jacobian(s.dn[378][11]);
        let eq38_e753_d_n12: f64 = self.ddt_jacobian(s.dn[378][12]);
        let eq38_e753_d_n13: f64 = self.ddt_jacobian(s.dn[378][13]);
        let eq38_value: f64 = eq38_e753;
        let eq38_node_derivatives: [f64; 14] = [eq38_e753_d_n0, eq38_e753_d_n1, eq38_e753_d_n2, eq38_e753_d_n3, eq38_e753_d_n4, eq38_e753_d_n5, eq38_e753_d_n6, eq38_e753_d_n7, eq38_e753_d_n8, eq38_e753_d_n9, eq38_e753_d_n10, eq38_e753_d_n11, eq38_e753_d_n12, eq38_e753_d_n13];
        let eq38_branch_derivatives: [f64; 0] = [];
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq40_e759: f64 = (s.v[1803] * (nv5 - 0.0));
        let eq40_e759_d_n0: f64 = (s.dn[1803][0] * (nv5 - 0.0));
        let eq40_e759_d_n1: f64 = (s.dn[1803][1] * (nv5 - 0.0));
        let eq40_e759_d_n2: f64 = (s.dn[1803][2] * (nv5 - 0.0));
        let eq40_e759_d_n3: f64 = (s.dn[1803][3] * (nv5 - 0.0));
        let eq40_e759_d_n4: f64 = (s.dn[1803][4] * (nv5 - 0.0));
        let eq40_e759_d_n5: f64 = ((s.dn[1803][5] * (nv5 - 0.0)) + s.v[1803]);
        let eq40_e759_d_n6: f64 = (s.dn[1803][6] * (nv5 - 0.0));
        let eq40_e759_d_n7: f64 = (s.dn[1803][7] * (nv5 - 0.0));
        let eq40_e759_d_n8: f64 = (s.dn[1803][8] * (nv5 - 0.0));
        let eq40_e759_d_n9: f64 = (s.dn[1803][9] * (nv5 - 0.0));
        let eq40_e759_d_n10: f64 = (s.dn[1803][10] * (nv5 - 0.0));
        let eq40_e759_d_n11: f64 = (s.dn[1803][11] * (nv5 - 0.0));
        let eq40_e759_d_n12: f64 = (s.dn[1803][12] * (nv5 - 0.0));
        let eq40_e759_d_n13: f64 = (s.dn[1803][13] * (nv5 - 0.0));
        let eq40_value: f64 = eq40_e759;
        let eq40_node_derivatives: [f64; 14] = [eq40_e759_d_n0, eq40_e759_d_n1, eq40_e759_d_n2, eq40_e759_d_n3, eq40_e759_d_n4, eq40_e759_d_n5, eq40_e759_d_n6, eq40_e759_d_n7, eq40_e759_d_n8, eq40_e759_d_n9, eq40_e759_d_n10, eq40_e759_d_n11, eq40_e759_d_n12, eq40_e759_d_n13];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let eq41_e762: f64 = (s.v[1800] * (nv5 - 0.0));
        let eq41_e762_d_n0: f64 = (s.dn[1800][0] * (nv5 - 0.0));
        let eq41_e762_d_n1: f64 = (s.dn[1800][1] * (nv5 - 0.0));
        let eq41_e762_d_n2: f64 = (s.dn[1800][2] * (nv5 - 0.0));
        let eq41_e762_d_n3: f64 = (s.dn[1800][3] * (nv5 - 0.0));
        let eq41_e762_d_n4: f64 = (s.dn[1800][4] * (nv5 - 0.0));
        let eq41_e762_d_n5: f64 = ((s.dn[1800][5] * (nv5 - 0.0)) + s.v[1800]);
        let eq41_e762_d_n6: f64 = (s.dn[1800][6] * (nv5 - 0.0));
        let eq41_e762_d_n7: f64 = (s.dn[1800][7] * (nv5 - 0.0));
        let eq41_e762_d_n8: f64 = (s.dn[1800][8] * (nv5 - 0.0));
        let eq41_e762_d_n9: f64 = (s.dn[1800][9] * (nv5 - 0.0));
        let eq41_e762_d_n10: f64 = (s.dn[1800][10] * (nv5 - 0.0));
        let eq41_e762_d_n11: f64 = (s.dn[1800][11] * (nv5 - 0.0));
        let eq41_e762_d_n12: f64 = (s.dn[1800][12] * (nv5 - 0.0));
        let eq41_e762_d_n13: f64 = (s.dn[1800][13] * (nv5 - 0.0));
        let eq41_e763: f64 = self.eval_ddt(21, eq41_e762);
        let eq41_e763_d_n0: f64 = self.ddt_jacobian(eq41_e762_d_n0);
        let eq41_e763_d_n1: f64 = self.ddt_jacobian(eq41_e762_d_n1);
        let eq41_e763_d_n2: f64 = self.ddt_jacobian(eq41_e762_d_n2);
        let eq41_e763_d_n3: f64 = self.ddt_jacobian(eq41_e762_d_n3);
        let eq41_e763_d_n4: f64 = self.ddt_jacobian(eq41_e762_d_n4);
        let eq41_e763_d_n5: f64 = self.ddt_jacobian(eq41_e762_d_n5);
        let eq41_e763_d_n6: f64 = self.ddt_jacobian(eq41_e762_d_n6);
        let eq41_e763_d_n7: f64 = self.ddt_jacobian(eq41_e762_d_n7);
        let eq41_e763_d_n8: f64 = self.ddt_jacobian(eq41_e762_d_n8);
        let eq41_e763_d_n9: f64 = self.ddt_jacobian(eq41_e762_d_n9);
        let eq41_e763_d_n10: f64 = self.ddt_jacobian(eq41_e762_d_n10);
        let eq41_e763_d_n11: f64 = self.ddt_jacobian(eq41_e762_d_n11);
        let eq41_e763_d_n12: f64 = self.ddt_jacobian(eq41_e762_d_n12);
        let eq41_e763_d_n13: f64 = self.ddt_jacobian(eq41_e762_d_n13);
        let eq41_value: f64 = eq41_e763;
        let eq41_node_derivatives: [f64; 14] = [eq41_e763_d_n0, eq41_e763_d_n1, eq41_e763_d_n2, eq41_e763_d_n3, eq41_e763_d_n4, eq41_e763_d_n5, eq41_e763_d_n6, eq41_e763_d_n7, eq41_e763_d_n8, eq41_e763_d_n9, eq41_e763_d_n10, eq41_e763_d_n11, eq41_e763_d_n12, eq41_e763_d_n13];
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
        let eq42_e765: f64 = (-s.v[1801]);
        let eq42_e765_d_n0: f64 = (-s.dn[1801][0]);
        let eq42_e765_d_n1: f64 = (-s.dn[1801][1]);
        let eq42_e765_d_n2: f64 = (-s.dn[1801][2]);
        let eq42_e765_d_n3: f64 = (-s.dn[1801][3]);
        let eq42_e765_d_n4: f64 = (-s.dn[1801][4]);
        let eq42_e765_d_n5: f64 = (-s.dn[1801][5]);
        let eq42_e765_d_n6: f64 = (-s.dn[1801][6]);
        let eq42_e765_d_n7: f64 = (-s.dn[1801][7]);
        let eq42_e765_d_n8: f64 = (-s.dn[1801][8]);
        let eq42_e765_d_n9: f64 = (-s.dn[1801][9]);
        let eq42_e765_d_n10: f64 = (-s.dn[1801][10]);
        let eq42_e765_d_n11: f64 = (-s.dn[1801][11]);
        let eq42_e765_d_n12: f64 = (-s.dn[1801][12]);
        let eq42_e765_d_n13: f64 = (-s.dn[1801][13]);
        let eq42_e767: f64 = (eq42_e765 * (nv5 - 0.0));
        let eq42_e767_d_n0: f64 = (eq42_e765_d_n0 * (nv5 - 0.0));
        let eq42_e767_d_n1: f64 = (eq42_e765_d_n1 * (nv5 - 0.0));
        let eq42_e767_d_n2: f64 = (eq42_e765_d_n2 * (nv5 - 0.0));
        let eq42_e767_d_n3: f64 = (eq42_e765_d_n3 * (nv5 - 0.0));
        let eq42_e767_d_n4: f64 = (eq42_e765_d_n4 * (nv5 - 0.0));
        let eq42_e767_d_n5: f64 = ((eq42_e765_d_n5 * (nv5 - 0.0)) + eq42_e765);
        let eq42_e767_d_n6: f64 = (eq42_e765_d_n6 * (nv5 - 0.0));
        let eq42_e767_d_n7: f64 = (eq42_e765_d_n7 * (nv5 - 0.0));
        let eq42_e767_d_n8: f64 = (eq42_e765_d_n8 * (nv5 - 0.0));
        let eq42_e767_d_n9: f64 = (eq42_e765_d_n9 * (nv5 - 0.0));
        let eq42_e767_d_n10: f64 = (eq42_e765_d_n10 * (nv5 - 0.0));
        let eq42_e767_d_n11: f64 = (eq42_e765_d_n11 * (nv5 - 0.0));
        let eq42_e767_d_n12: f64 = (eq42_e765_d_n12 * (nv5 - 0.0));
        let eq42_e767_d_n13: f64 = (eq42_e765_d_n13 * (nv5 - 0.0));
        let eq42_e768: f64 = self.eval_ddt(22, eq42_e767);
        let eq42_e768_d_n0: f64 = self.ddt_jacobian(eq42_e767_d_n0);
        let eq42_e768_d_n1: f64 = self.ddt_jacobian(eq42_e767_d_n1);
        let eq42_e768_d_n2: f64 = self.ddt_jacobian(eq42_e767_d_n2);
        let eq42_e768_d_n3: f64 = self.ddt_jacobian(eq42_e767_d_n3);
        let eq42_e768_d_n4: f64 = self.ddt_jacobian(eq42_e767_d_n4);
        let eq42_e768_d_n5: f64 = self.ddt_jacobian(eq42_e767_d_n5);
        let eq42_e768_d_n6: f64 = self.ddt_jacobian(eq42_e767_d_n6);
        let eq42_e768_d_n7: f64 = self.ddt_jacobian(eq42_e767_d_n7);
        let eq42_e768_d_n8: f64 = self.ddt_jacobian(eq42_e767_d_n8);
        let eq42_e768_d_n9: f64 = self.ddt_jacobian(eq42_e767_d_n9);
        let eq42_e768_d_n10: f64 = self.ddt_jacobian(eq42_e767_d_n10);
        let eq42_e768_d_n11: f64 = self.ddt_jacobian(eq42_e767_d_n11);
        let eq42_e768_d_n12: f64 = self.ddt_jacobian(eq42_e767_d_n12);
        let eq42_e768_d_n13: f64 = self.ddt_jacobian(eq42_e767_d_n13);
        let eq42_value: f64 = eq42_e768;
        let eq42_node_derivatives: [f64; 14] = [eq42_e768_d_n0, eq42_e768_d_n1, eq42_e768_d_n2, eq42_e768_d_n3, eq42_e768_d_n4, eq42_e768_d_n5, eq42_e768_d_n6, eq42_e768_d_n7, eq42_e768_d_n8, eq42_e768_d_n9, eq42_e768_d_n10, eq42_e768_d_n11, eq42_e768_d_n12, eq42_e768_d_n13];
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq43_e770: f64 = (-s.v[1802]);
        let eq43_e770_d_n0: f64 = (-s.dn[1802][0]);
        let eq43_e770_d_n1: f64 = (-s.dn[1802][1]);
        let eq43_e770_d_n2: f64 = (-s.dn[1802][2]);
        let eq43_e770_d_n3: f64 = (-s.dn[1802][3]);
        let eq43_e770_d_n4: f64 = (-s.dn[1802][4]);
        let eq43_e770_d_n5: f64 = (-s.dn[1802][5]);
        let eq43_e770_d_n6: f64 = (-s.dn[1802][6]);
        let eq43_e770_d_n7: f64 = (-s.dn[1802][7]);
        let eq43_e770_d_n8: f64 = (-s.dn[1802][8]);
        let eq43_e770_d_n9: f64 = (-s.dn[1802][9]);
        let eq43_e770_d_n10: f64 = (-s.dn[1802][10]);
        let eq43_e770_d_n11: f64 = (-s.dn[1802][11]);
        let eq43_e770_d_n12: f64 = (-s.dn[1802][12]);
        let eq43_e770_d_n13: f64 = (-s.dn[1802][13]);
        let eq43_e772: f64 = (eq43_e770 * (nv5 - 0.0));
        let eq43_e772_d_n0: f64 = (eq43_e770_d_n0 * (nv5 - 0.0));
        let eq43_e772_d_n1: f64 = (eq43_e770_d_n1 * (nv5 - 0.0));
        let eq43_e772_d_n2: f64 = (eq43_e770_d_n2 * (nv5 - 0.0));
        let eq43_e772_d_n3: f64 = (eq43_e770_d_n3 * (nv5 - 0.0));
        let eq43_e772_d_n4: f64 = (eq43_e770_d_n4 * (nv5 - 0.0));
        let eq43_e772_d_n5: f64 = ((eq43_e770_d_n5 * (nv5 - 0.0)) + eq43_e770);
        let eq43_e772_d_n6: f64 = (eq43_e770_d_n6 * (nv5 - 0.0));
        let eq43_e772_d_n7: f64 = (eq43_e770_d_n7 * (nv5 - 0.0));
        let eq43_e772_d_n8: f64 = (eq43_e770_d_n8 * (nv5 - 0.0));
        let eq43_e772_d_n9: f64 = (eq43_e770_d_n9 * (nv5 - 0.0));
        let eq43_e772_d_n10: f64 = (eq43_e770_d_n10 * (nv5 - 0.0));
        let eq43_e772_d_n11: f64 = (eq43_e770_d_n11 * (nv5 - 0.0));
        let eq43_e772_d_n12: f64 = (eq43_e770_d_n12 * (nv5 - 0.0));
        let eq43_e772_d_n13: f64 = (eq43_e770_d_n13 * (nv5 - 0.0));
        let eq43_e773: f64 = self.eval_ddt(23, eq43_e772);
        let eq43_e773_d_n0: f64 = self.ddt_jacobian(eq43_e772_d_n0);
        let eq43_e773_d_n1: f64 = self.ddt_jacobian(eq43_e772_d_n1);
        let eq43_e773_d_n2: f64 = self.ddt_jacobian(eq43_e772_d_n2);
        let eq43_e773_d_n3: f64 = self.ddt_jacobian(eq43_e772_d_n3);
        let eq43_e773_d_n4: f64 = self.ddt_jacobian(eq43_e772_d_n4);
        let eq43_e773_d_n5: f64 = self.ddt_jacobian(eq43_e772_d_n5);
        let eq43_e773_d_n6: f64 = self.ddt_jacobian(eq43_e772_d_n6);
        let eq43_e773_d_n7: f64 = self.ddt_jacobian(eq43_e772_d_n7);
        let eq43_e773_d_n8: f64 = self.ddt_jacobian(eq43_e772_d_n8);
        let eq43_e773_d_n9: f64 = self.ddt_jacobian(eq43_e772_d_n9);
        let eq43_e773_d_n10: f64 = self.ddt_jacobian(eq43_e772_d_n10);
        let eq43_e773_d_n11: f64 = self.ddt_jacobian(eq43_e772_d_n11);
        let eq43_e773_d_n12: f64 = self.ddt_jacobian(eq43_e772_d_n12);
        let eq43_e773_d_n13: f64 = self.ddt_jacobian(eq43_e772_d_n13);
        let eq43_value: f64 = eq43_e773;
        let eq43_node_derivatives: [f64; 14] = [eq43_e773_d_n0, eq43_e773_d_n1, eq43_e773_d_n2, eq43_e773_d_n3, eq43_e773_d_n4, eq43_e773_d_n5, eq43_e773_d_n6, eq43_e773_d_n7, eq43_e773_d_n8, eq43_e773_d_n9, eq43_e773_d_n10, eq43_e773_d_n11, eq43_e773_d_n12, eq43_e773_d_n13];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
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
        let eq44_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq44_value),
            &[
            ],
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
        let eq46_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq46_value),
            &[
            ],
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
        let eq47_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[6]),
            self.multiplicity * (eq47_value),
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
        let eq48_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq48_value),
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
        let eq49_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq49_value),
            &[
            ],
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
        let eq23_e642: f64 = (s.v[1774] + s.v[1775]);
        let eq23_e642_d_n0: f64 = (s.dn[1774][0] + s.dn[1775][0]);
        let eq23_e642_d_n1: f64 = (s.dn[1774][1] + s.dn[1775][1]);
        let eq23_e642_d_n2: f64 = (s.dn[1774][2] + s.dn[1775][2]);
        let eq23_e642_d_n3: f64 = (s.dn[1774][3] + s.dn[1775][3]);
        let eq23_e642_d_n4: f64 = (s.dn[1774][4] + s.dn[1775][4]);
        let eq23_e642_d_n5: f64 = (s.dn[1774][5] + s.dn[1775][5]);
        let eq23_e642_d_n6: f64 = (s.dn[1774][6] + s.dn[1775][6]);
        let eq23_e642_d_n7: f64 = (s.dn[1774][7] + s.dn[1775][7]);
        let eq23_e642_d_n8: f64 = (s.dn[1774][8] + s.dn[1775][8]);
        let eq23_e642_d_n9: f64 = (s.dn[1774][9] + s.dn[1775][9]);
        let eq23_e642_d_n10: f64 = (s.dn[1774][10] + s.dn[1775][10]);
        let eq23_e642_d_n11: f64 = (s.dn[1774][11] + s.dn[1775][11]);
        let eq23_e642_d_n12: f64 = (s.dn[1774][12] + s.dn[1775][12]);
        let eq23_e642_d_n13: f64 = (s.dn[1774][13] + s.dn[1775][13]);
        let eq23_e643: f64 = (s.v[181] * eq23_e642);
        let eq23_e643_d_n0: f64 = ((s.dn[181][0] * eq23_e642) + (s.v[181] * eq23_e642_d_n0));
        let eq23_e643_d_n1: f64 = ((s.dn[181][1] * eq23_e642) + (s.v[181] * eq23_e642_d_n1));
        let eq23_e643_d_n2: f64 = ((s.dn[181][2] * eq23_e642) + (s.v[181] * eq23_e642_d_n2));
        let eq23_e643_d_n3: f64 = ((s.dn[181][3] * eq23_e642) + (s.v[181] * eq23_e642_d_n3));
        let eq23_e643_d_n4: f64 = ((s.dn[181][4] * eq23_e642) + (s.v[181] * eq23_e642_d_n4));
        let eq23_e643_d_n5: f64 = ((s.dn[181][5] * eq23_e642) + (s.v[181] * eq23_e642_d_n5));
        let eq23_e643_d_n6: f64 = ((s.dn[181][6] * eq23_e642) + (s.v[181] * eq23_e642_d_n6));
        let eq23_e643_d_n7: f64 = ((s.dn[181][7] * eq23_e642) + (s.v[181] * eq23_e642_d_n7));
        let eq23_e643_d_n8: f64 = ((s.dn[181][8] * eq23_e642) + (s.v[181] * eq23_e642_d_n8));
        let eq23_e643_d_n9: f64 = ((s.dn[181][9] * eq23_e642) + (s.v[181] * eq23_e642_d_n9));
        let eq23_e643_d_n10: f64 = ((s.dn[181][10] * eq23_e642) + (s.v[181] * eq23_e642_d_n10));
        let eq23_e643_d_n11: f64 = ((s.dn[181][11] * eq23_e642) + (s.v[181] * eq23_e642_d_n11));
        let eq23_e643_d_n12: f64 = ((s.dn[181][12] * eq23_e642) + (s.v[181] * eq23_e642_d_n12));
        let eq23_e643_d_n13: f64 = ((s.dn[181][13] * eq23_e642) + (s.v[181] * eq23_e642_d_n13));
        let eq23_e644_q: f64 = eq23_e643;
        let eq23_reactive_node_derivatives: [f64; 14] = [eq23_e643_d_n0, eq23_e643_d_n1, eq23_e643_d_n2, eq23_e643_d_n3, eq23_e643_d_n4, eq23_e643_d_n5, eq23_e643_d_n6, eq23_e643_d_n7, eq23_e643_d_n8, eq23_e643_d_n9, eq23_e643_d_n10, eq23_e643_d_n11, eq23_e643_d_n12, eq23_e643_d_n13];
        let eq23_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[13]),
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq25_e650: f64 = (1e-9 * (nv10 - nv13));
        let eq25_e650_d_n10: f64 = 1e-9;
        let eq25_e650_d_n13: f64 = (-1e-9);
        let eq25_e651_q: f64 = eq25_e650;
        stamper.stamp_current_reactive(
            Some(nodes[10]),
            Some(nodes[13]),
            &[
                GeneratedDerivative::node(nodes[10], self.multiplicity * (eq25_e650_d_n10)),
                GeneratedDerivative::node(nodes[13], self.multiplicity * (eq25_e650_d_n13)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq26_e653_q: f64 = s.v[1776];
        let eq26_reactive_node_derivatives: [f64; 14] = [s.dn[1776][0], s.dn[1776][1], s.dn[1776][2], s.dn[1776][3], s.dn[1776][4], s.dn[1776][5], s.dn[1776][6], s.dn[1776][7], s.dn[1776][8], s.dn[1776][9], s.dn[1776][10], s.dn[1776][11], s.dn[1776][12], s.dn[1776][13]];
        let eq26_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[13]),
            &nodes,
            &eq26_reactive_node_derivatives,
            &branches,
            &eq26_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
