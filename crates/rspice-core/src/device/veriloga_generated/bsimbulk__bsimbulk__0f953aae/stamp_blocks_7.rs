#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_70_block_0(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq70_e1832, eq70_e1832_d_n0, eq70_e1832_d_n1, eq70_e1832_d_n2, eq70_e1832_d_n3, eq70_e1832_d_n4, eq70_e1832_d_n5, eq70_e1832_d_n6, eq70_e1832_d_n7, eq70_e1832_d_n8, eq70_e1832_d_n9, eq70_e1832_d_n10, eq70_e1832_d_n11, eq70_e1832_d_n12, eq70_e1832_d_n13, eq70_e1832_d_n14, eq70_e1832_d_n15, eq70_e1832_d_n16,) = {
    if (s.v[1627] != 0.0) {
        let eq70_e1822: f64 = (s.v[187] * p.p28);
        let eq70_e1822_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq70_e1822_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq70_e1822_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq70_e1822_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq70_e1822_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq70_e1822_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq70_e1822_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq70_e1822_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq70_e1822_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq70_e1822_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq70_e1822_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq70_e1822_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq70_e1822_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq70_e1822_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq70_e1822_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq70_e1822_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq70_e1822_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq70_e1824: f64 = (eq70_e1822 * s.v[303]);
        let eq70_e1824_d_n0: f64 = ((eq70_e1822_d_n0 * s.v[303]) + (eq70_e1822 * s.dn[303][0]));
        let eq70_e1824_d_n1: f64 = ((eq70_e1822_d_n1 * s.v[303]) + (eq70_e1822 * s.dn[303][1]));
        let eq70_e1824_d_n2: f64 = ((eq70_e1822_d_n2 * s.v[303]) + (eq70_e1822 * s.dn[303][2]));
        let eq70_e1824_d_n3: f64 = ((eq70_e1822_d_n3 * s.v[303]) + (eq70_e1822 * s.dn[303][3]));
        let eq70_e1824_d_n4: f64 = ((eq70_e1822_d_n4 * s.v[303]) + (eq70_e1822 * s.dn[303][4]));
        let eq70_e1824_d_n5: f64 = ((eq70_e1822_d_n5 * s.v[303]) + (eq70_e1822 * s.dn[303][5]));
        let eq70_e1824_d_n6: f64 = ((eq70_e1822_d_n6 * s.v[303]) + (eq70_e1822 * s.dn[303][6]));
        let eq70_e1824_d_n7: f64 = ((eq70_e1822_d_n7 * s.v[303]) + (eq70_e1822 * s.dn[303][7]));
        let eq70_e1824_d_n8: f64 = ((eq70_e1822_d_n8 * s.v[303]) + (eq70_e1822 * s.dn[303][8]));
        let eq70_e1824_d_n9: f64 = ((eq70_e1822_d_n9 * s.v[303]) + (eq70_e1822 * s.dn[303][9]));
        let eq70_e1824_d_n10: f64 = ((eq70_e1822_d_n10 * s.v[303]) + (eq70_e1822 * s.dn[303][10]));
        let eq70_e1824_d_n11: f64 = ((eq70_e1822_d_n11 * s.v[303]) + (eq70_e1822 * s.dn[303][11]));
        let eq70_e1824_d_n12: f64 = ((eq70_e1822_d_n12 * s.v[303]) + (eq70_e1822 * s.dn[303][12]));
        let eq70_e1824_d_n13: f64 = ((eq70_e1822_d_n13 * s.v[303]) + (eq70_e1822 * s.dn[303][13]));
        let eq70_e1824_d_n14: f64 = ((eq70_e1822_d_n14 * s.v[303]) + (eq70_e1822 * s.dn[303][14]));
        let eq70_e1824_d_n15: f64 = ((eq70_e1822_d_n15 * s.v[303]) + (eq70_e1822 * s.dn[303][15]));
        let eq70_e1824_d_n16: f64 = ((eq70_e1822_d_n16 * s.v[303]) + (eq70_e1822 * s.dn[303][16]));
        let eq70_e1827: f64 = ((nv12 - nv7) * p.p28);
        let eq70_e1827_d_n7: f64 = (-p.p28);
        let eq70_e1827_d_n12: f64 = p.p28;
        let eq70_e1829: f64 = (eq70_e1827 * s.v[781]);
        let eq70_e1829_d_n7: f64 = (eq70_e1827_d_n7 * s.v[781]);
        let eq70_e1829_d_n12: f64 = (eq70_e1827_d_n12 * s.v[781]);
        let eq70_e1830: f64 = (eq70_e1824 + eq70_e1829);
        let eq70_e1830_d_n7: f64 = (eq70_e1824_d_n7 + eq70_e1829_d_n7);
        let eq70_e1830_d_n12: f64 = (eq70_e1824_d_n12 + eq70_e1829_d_n12);
        (eq70_e1830, eq70_e1824_d_n0, eq70_e1824_d_n1, eq70_e1824_d_n2, eq70_e1824_d_n3, eq70_e1824_d_n4, eq70_e1824_d_n5, eq70_e1824_d_n6, eq70_e1830_d_n7, eq70_e1824_d_n8, eq70_e1824_d_n9, eq70_e1824_d_n10, eq70_e1824_d_n11, eq70_e1830_d_n12, eq70_e1824_d_n13, eq70_e1824_d_n14, eq70_e1824_d_n15, eq70_e1824_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1832;
        let eq70_node_derivatives: [f64; 17] = [eq70_e1832_d_n0, eq70_e1832_d_n1, eq70_e1832_d_n2, eq70_e1832_d_n3, eq70_e1832_d_n4, eq70_e1832_d_n5, eq70_e1832_d_n6, eq70_e1832_d_n7, eq70_e1832_d_n8, eq70_e1832_d_n9, eq70_e1832_d_n10, eq70_e1832_d_n11, eq70_e1832_d_n12, eq70_e1832_d_n13, eq70_e1832_d_n14, eq70_e1832_d_n15, eq70_e1832_d_n16];
        let eq70_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            self.multiplicity * (eq70_value),
            &nodes,
            &eq70_node_derivatives,
            &branches,
            &eq70_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_71_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n1, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, eq71_e1841_d_n15, eq71_e1841_d_n16,) = {
    if (s.v[1627] != 0.0) {
        let eq71_e1837: f64 = (p.p29 * s.v[330]);
        let eq71_e1837_d_n0: f64 = (p.p29 * s.dn[330][0]);
        let eq71_e1837_d_n1: f64 = (p.p29 * s.dn[330][1]);
        let eq71_e1837_d_n2: f64 = (p.p29 * s.dn[330][2]);
        let eq71_e1837_d_n3: f64 = (p.p29 * s.dn[330][3]);
        let eq71_e1837_d_n4: f64 = (p.p29 * s.dn[330][4]);
        let eq71_e1837_d_n5: f64 = (p.p29 * s.dn[330][5]);
        let eq71_e1837_d_n6: f64 = (p.p29 * s.dn[330][6]);
        let eq71_e1837_d_n7: f64 = (p.p29 * s.dn[330][7]);
        let eq71_e1837_d_n8: f64 = (p.p29 * s.dn[330][8]);
        let eq71_e1837_d_n9: f64 = (p.p29 * s.dn[330][9]);
        let eq71_e1837_d_n10: f64 = (p.p29 * s.dn[330][10]);
        let eq71_e1837_d_n11: f64 = (p.p29 * s.dn[330][11]);
        let eq71_e1837_d_n12: f64 = (p.p29 * s.dn[330][12]);
        let eq71_e1837_d_n13: f64 = (p.p29 * s.dn[330][13]);
        let eq71_e1837_d_n14: f64 = (p.p29 * s.dn[330][14]);
        let eq71_e1837_d_n15: f64 = (p.p29 * s.dn[330][15]);
        let eq71_e1837_d_n16: f64 = (p.p29 * s.dn[330][16]);
        let eq71_e1838: f64 = self.eval_ddt(10, eq71_e1837);
        let eq71_e1838_d_n0: f64 = self.ddt_jacobian(eq71_e1837_d_n0);
        let eq71_e1838_d_n1: f64 = self.ddt_jacobian(eq71_e1837_d_n1);
        let eq71_e1838_d_n2: f64 = self.ddt_jacobian(eq71_e1837_d_n2);
        let eq71_e1838_d_n3: f64 = self.ddt_jacobian(eq71_e1837_d_n3);
        let eq71_e1838_d_n4: f64 = self.ddt_jacobian(eq71_e1837_d_n4);
        let eq71_e1838_d_n5: f64 = self.ddt_jacobian(eq71_e1837_d_n5);
        let eq71_e1838_d_n6: f64 = self.ddt_jacobian(eq71_e1837_d_n6);
        let eq71_e1838_d_n7: f64 = self.ddt_jacobian(eq71_e1837_d_n7);
        let eq71_e1838_d_n8: f64 = self.ddt_jacobian(eq71_e1837_d_n8);
        let eq71_e1838_d_n9: f64 = self.ddt_jacobian(eq71_e1837_d_n9);
        let eq71_e1838_d_n10: f64 = self.ddt_jacobian(eq71_e1837_d_n10);
        let eq71_e1838_d_n11: f64 = self.ddt_jacobian(eq71_e1837_d_n11);
        let eq71_e1838_d_n12: f64 = self.ddt_jacobian(eq71_e1837_d_n12);
        let eq71_e1838_d_n13: f64 = self.ddt_jacobian(eq71_e1837_d_n13);
        let eq71_e1838_d_n14: f64 = self.ddt_jacobian(eq71_e1837_d_n14);
        let eq71_e1838_d_n15: f64 = self.ddt_jacobian(eq71_e1837_d_n15);
        let eq71_e1838_d_n16: f64 = self.ddt_jacobian(eq71_e1837_d_n16);
        let eq71_e1839: f64 = (s.v[187] * eq71_e1838);
        let eq71_e1839_d_n0: f64 = ((s.dn[187][0] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n0));
        let eq71_e1839_d_n1: f64 = ((s.dn[187][1] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n1));
        let eq71_e1839_d_n2: f64 = ((s.dn[187][2] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n2));
        let eq71_e1839_d_n3: f64 = ((s.dn[187][3] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n3));
        let eq71_e1839_d_n4: f64 = ((s.dn[187][4] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n4));
        let eq71_e1839_d_n5: f64 = ((s.dn[187][5] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n5));
        let eq71_e1839_d_n6: f64 = ((s.dn[187][6] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n6));
        let eq71_e1839_d_n7: f64 = ((s.dn[187][7] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n7));
        let eq71_e1839_d_n8: f64 = ((s.dn[187][8] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n8));
        let eq71_e1839_d_n9: f64 = ((s.dn[187][9] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n9));
        let eq71_e1839_d_n10: f64 = ((s.dn[187][10] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n10));
        let eq71_e1839_d_n11: f64 = ((s.dn[187][11] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n11));
        let eq71_e1839_d_n12: f64 = ((s.dn[187][12] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n12));
        let eq71_e1839_d_n13: f64 = ((s.dn[187][13] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n13));
        let eq71_e1839_d_n14: f64 = ((s.dn[187][14] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n14));
        let eq71_e1839_d_n15: f64 = ((s.dn[187][15] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n15));
        let eq71_e1839_d_n16: f64 = ((s.dn[187][16] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n16));
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n1, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14, eq71_e1839_d_n15, eq71_e1839_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e1841;
        let eq71_node_derivatives: [f64; 17] = [eq71_e1841_d_n0, eq71_e1841_d_n1, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, eq71_e1841_d_n15, eq71_e1841_d_n16];
        let eq71_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            self.multiplicity * (eq71_value),
            &nodes,
            &eq71_node_derivatives,
            &branches,
            &eq71_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_72_block_0(
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq72_e1857, eq72_e1857_d_n0, eq72_e1857_d_n1, eq72_e1857_d_n2, eq72_e1857_d_n3, eq72_e1857_d_n4, eq72_e1857_d_n5, eq72_e1857_d_n6, eq72_e1857_d_n7, eq72_e1857_d_n8, eq72_e1857_d_n9, eq72_e1857_d_n10, eq72_e1857_d_n11, eq72_e1857_d_n12, eq72_e1857_d_n13, eq72_e1857_d_n14, eq72_e1857_d_n15, eq72_e1857_d_n16,) = {
    if ((s.v[1627] != 0.0) && (s.v[1628] != 0.0)) {
        let eq72_e1847: f64 = (s.v[187] * p.p28);
        let eq72_e1847_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq72_e1847_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq72_e1847_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq72_e1847_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq72_e1847_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq72_e1847_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq72_e1847_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq72_e1847_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq72_e1847_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq72_e1847_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq72_e1847_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq72_e1847_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq72_e1847_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq72_e1847_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq72_e1847_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq72_e1847_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq72_e1847_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq72_e1849: f64 = (eq72_e1847 * s.v[304]);
        let eq72_e1849_d_n0: f64 = ((eq72_e1847_d_n0 * s.v[304]) + (eq72_e1847 * s.dn[304][0]));
        let eq72_e1849_d_n1: f64 = ((eq72_e1847_d_n1 * s.v[304]) + (eq72_e1847 * s.dn[304][1]));
        let eq72_e1849_d_n2: f64 = ((eq72_e1847_d_n2 * s.v[304]) + (eq72_e1847 * s.dn[304][2]));
        let eq72_e1849_d_n3: f64 = ((eq72_e1847_d_n3 * s.v[304]) + (eq72_e1847 * s.dn[304][3]));
        let eq72_e1849_d_n4: f64 = ((eq72_e1847_d_n4 * s.v[304]) + (eq72_e1847 * s.dn[304][4]));
        let eq72_e1849_d_n5: f64 = ((eq72_e1847_d_n5 * s.v[304]) + (eq72_e1847 * s.dn[304][5]));
        let eq72_e1849_d_n6: f64 = ((eq72_e1847_d_n6 * s.v[304]) + (eq72_e1847 * s.dn[304][6]));
        let eq72_e1849_d_n7: f64 = ((eq72_e1847_d_n7 * s.v[304]) + (eq72_e1847 * s.dn[304][7]));
        let eq72_e1849_d_n8: f64 = ((eq72_e1847_d_n8 * s.v[304]) + (eq72_e1847 * s.dn[304][8]));
        let eq72_e1849_d_n9: f64 = ((eq72_e1847_d_n9 * s.v[304]) + (eq72_e1847 * s.dn[304][9]));
        let eq72_e1849_d_n10: f64 = ((eq72_e1847_d_n10 * s.v[304]) + (eq72_e1847 * s.dn[304][10]));
        let eq72_e1849_d_n11: f64 = ((eq72_e1847_d_n11 * s.v[304]) + (eq72_e1847 * s.dn[304][11]));
        let eq72_e1849_d_n12: f64 = ((eq72_e1847_d_n12 * s.v[304]) + (eq72_e1847 * s.dn[304][12]));
        let eq72_e1849_d_n13: f64 = ((eq72_e1847_d_n13 * s.v[304]) + (eq72_e1847 * s.dn[304][13]));
        let eq72_e1849_d_n14: f64 = ((eq72_e1847_d_n14 * s.v[304]) + (eq72_e1847 * s.dn[304][14]));
        let eq72_e1849_d_n15: f64 = ((eq72_e1847_d_n15 * s.v[304]) + (eq72_e1847 * s.dn[304][15]));
        let eq72_e1849_d_n16: f64 = ((eq72_e1847_d_n16 * s.v[304]) + (eq72_e1847 * s.dn[304][16]));
        let eq72_e1852: f64 = ((nv13 - nv5) * p.p28);
        let eq72_e1852_d_n5: f64 = (-p.p28);
        let eq72_e1852_d_n13: f64 = p.p28;
        let eq72_e1854: f64 = (eq72_e1852 * s.v[781]);
        let eq72_e1854_d_n5: f64 = (eq72_e1852_d_n5 * s.v[781]);
        let eq72_e1854_d_n13: f64 = (eq72_e1852_d_n13 * s.v[781]);
        let eq72_e1855: f64 = (eq72_e1849 + eq72_e1854);
        let eq72_e1855_d_n5: f64 = (eq72_e1849_d_n5 + eq72_e1854_d_n5);
        let eq72_e1855_d_n13: f64 = (eq72_e1849_d_n13 + eq72_e1854_d_n13);
        (eq72_e1855, eq72_e1849_d_n0, eq72_e1849_d_n1, eq72_e1849_d_n2, eq72_e1849_d_n3, eq72_e1849_d_n4, eq72_e1855_d_n5, eq72_e1849_d_n6, eq72_e1849_d_n7, eq72_e1849_d_n8, eq72_e1849_d_n9, eq72_e1849_d_n10, eq72_e1849_d_n11, eq72_e1849_d_n12, eq72_e1855_d_n13, eq72_e1849_d_n14, eq72_e1849_d_n15, eq72_e1849_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1857;
        let eq72_node_derivatives: [f64; 17] = [eq72_e1857_d_n0, eq72_e1857_d_n1, eq72_e1857_d_n2, eq72_e1857_d_n3, eq72_e1857_d_n4, eq72_e1857_d_n5, eq72_e1857_d_n6, eq72_e1857_d_n7, eq72_e1857_d_n8, eq72_e1857_d_n9, eq72_e1857_d_n10, eq72_e1857_d_n11, eq72_e1857_d_n12, eq72_e1857_d_n13, eq72_e1857_d_n14, eq72_e1857_d_n15, eq72_e1857_d_n16];
        let eq72_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            self.multiplicity * (eq72_value),
            &nodes,
            &eq72_node_derivatives,
            &branches,
            &eq72_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_73_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n1, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, eq73_e1868_d_n15, eq73_e1868_d_n16,) = {
    if ((s.v[1627] != 0.0) && (s.v[1628] != 0.0)) {
        let eq73_e1864: f64 = (p.p29 * s.v[334]);
        let eq73_e1864_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq73_e1864_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq73_e1864_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq73_e1864_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq73_e1864_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq73_e1864_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq73_e1864_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq73_e1864_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq73_e1864_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq73_e1864_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq73_e1864_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq73_e1864_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq73_e1864_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq73_e1864_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq73_e1864_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq73_e1864_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq73_e1864_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq73_e1865: f64 = self.eval_ddt(11, eq73_e1864);
        let eq73_e1865_d_n0: f64 = self.ddt_jacobian(eq73_e1864_d_n0);
        let eq73_e1865_d_n1: f64 = self.ddt_jacobian(eq73_e1864_d_n1);
        let eq73_e1865_d_n2: f64 = self.ddt_jacobian(eq73_e1864_d_n2);
        let eq73_e1865_d_n3: f64 = self.ddt_jacobian(eq73_e1864_d_n3);
        let eq73_e1865_d_n4: f64 = self.ddt_jacobian(eq73_e1864_d_n4);
        let eq73_e1865_d_n5: f64 = self.ddt_jacobian(eq73_e1864_d_n5);
        let eq73_e1865_d_n6: f64 = self.ddt_jacobian(eq73_e1864_d_n6);
        let eq73_e1865_d_n7: f64 = self.ddt_jacobian(eq73_e1864_d_n7);
        let eq73_e1865_d_n8: f64 = self.ddt_jacobian(eq73_e1864_d_n8);
        let eq73_e1865_d_n9: f64 = self.ddt_jacobian(eq73_e1864_d_n9);
        let eq73_e1865_d_n10: f64 = self.ddt_jacobian(eq73_e1864_d_n10);
        let eq73_e1865_d_n11: f64 = self.ddt_jacobian(eq73_e1864_d_n11);
        let eq73_e1865_d_n12: f64 = self.ddt_jacobian(eq73_e1864_d_n12);
        let eq73_e1865_d_n13: f64 = self.ddt_jacobian(eq73_e1864_d_n13);
        let eq73_e1865_d_n14: f64 = self.ddt_jacobian(eq73_e1864_d_n14);
        let eq73_e1865_d_n15: f64 = self.ddt_jacobian(eq73_e1864_d_n15);
        let eq73_e1865_d_n16: f64 = self.ddt_jacobian(eq73_e1864_d_n16);
        let eq73_e1866: f64 = (s.v[187] * eq73_e1865);
        let eq73_e1866_d_n0: f64 = ((s.dn[187][0] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n0));
        let eq73_e1866_d_n1: f64 = ((s.dn[187][1] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n1));
        let eq73_e1866_d_n2: f64 = ((s.dn[187][2] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n2));
        let eq73_e1866_d_n3: f64 = ((s.dn[187][3] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n3));
        let eq73_e1866_d_n4: f64 = ((s.dn[187][4] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n4));
        let eq73_e1866_d_n5: f64 = ((s.dn[187][5] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n5));
        let eq73_e1866_d_n6: f64 = ((s.dn[187][6] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n6));
        let eq73_e1866_d_n7: f64 = ((s.dn[187][7] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n7));
        let eq73_e1866_d_n8: f64 = ((s.dn[187][8] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n8));
        let eq73_e1866_d_n9: f64 = ((s.dn[187][9] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n9));
        let eq73_e1866_d_n10: f64 = ((s.dn[187][10] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n10));
        let eq73_e1866_d_n11: f64 = ((s.dn[187][11] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n11));
        let eq73_e1866_d_n12: f64 = ((s.dn[187][12] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n12));
        let eq73_e1866_d_n13: f64 = ((s.dn[187][13] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n13));
        let eq73_e1866_d_n14: f64 = ((s.dn[187][14] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n14));
        let eq73_e1866_d_n15: f64 = ((s.dn[187][15] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n15));
        let eq73_e1866_d_n16: f64 = ((s.dn[187][16] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n16));
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n1, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14, eq73_e1866_d_n15, eq73_e1866_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1868;
        let eq73_node_derivatives: [f64; 17] = [eq73_e1868_d_n0, eq73_e1868_d_n1, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, eq73_e1868_d_n15, eq73_e1868_d_n16];
        let eq73_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            self.multiplicity * (eq73_value),
            &nodes,
            &eq73_node_derivatives,
            &branches,
            &eq73_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_74_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq74_e1883, eq74_e1883_d_n0, eq74_e1883_d_n1, eq74_e1883_d_n2, eq74_e1883_d_n3, eq74_e1883_d_n4, eq74_e1883_d_n5, eq74_e1883_d_n6, eq74_e1883_d_n7, eq74_e1883_d_n8, eq74_e1883_d_n9, eq74_e1883_d_n10, eq74_e1883_d_n11, eq74_e1883_d_n12, eq74_e1883_d_n13, eq74_e1883_d_n14, eq74_e1883_d_n15, eq74_e1883_d_n16,) = {
    if (!(s.v[1627] != 0.0)) {
        let eq74_e1873: f64 = (s.v[187] * p.p28);
        let eq74_e1873_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq74_e1873_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq74_e1873_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq74_e1873_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq74_e1873_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq74_e1873_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq74_e1873_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq74_e1873_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq74_e1873_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq74_e1873_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq74_e1873_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq74_e1873_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq74_e1873_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq74_e1873_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq74_e1873_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq74_e1873_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq74_e1873_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq74_e1875: f64 = (eq74_e1873 * s.v[303]);
        let eq74_e1875_d_n0: f64 = ((eq74_e1873_d_n0 * s.v[303]) + (eq74_e1873 * s.dn[303][0]));
        let eq74_e1875_d_n1: f64 = ((eq74_e1873_d_n1 * s.v[303]) + (eq74_e1873 * s.dn[303][1]));
        let eq74_e1875_d_n2: f64 = ((eq74_e1873_d_n2 * s.v[303]) + (eq74_e1873 * s.dn[303][2]));
        let eq74_e1875_d_n3: f64 = ((eq74_e1873_d_n3 * s.v[303]) + (eq74_e1873 * s.dn[303][3]));
        let eq74_e1875_d_n4: f64 = ((eq74_e1873_d_n4 * s.v[303]) + (eq74_e1873 * s.dn[303][4]));
        let eq74_e1875_d_n5: f64 = ((eq74_e1873_d_n5 * s.v[303]) + (eq74_e1873 * s.dn[303][5]));
        let eq74_e1875_d_n6: f64 = ((eq74_e1873_d_n6 * s.v[303]) + (eq74_e1873 * s.dn[303][6]));
        let eq74_e1875_d_n7: f64 = ((eq74_e1873_d_n7 * s.v[303]) + (eq74_e1873 * s.dn[303][7]));
        let eq74_e1875_d_n8: f64 = ((eq74_e1873_d_n8 * s.v[303]) + (eq74_e1873 * s.dn[303][8]));
        let eq74_e1875_d_n9: f64 = ((eq74_e1873_d_n9 * s.v[303]) + (eq74_e1873 * s.dn[303][9]));
        let eq74_e1875_d_n10: f64 = ((eq74_e1873_d_n10 * s.v[303]) + (eq74_e1873 * s.dn[303][10]));
        let eq74_e1875_d_n11: f64 = ((eq74_e1873_d_n11 * s.v[303]) + (eq74_e1873 * s.dn[303][11]));
        let eq74_e1875_d_n12: f64 = ((eq74_e1873_d_n12 * s.v[303]) + (eq74_e1873 * s.dn[303][12]));
        let eq74_e1875_d_n13: f64 = ((eq74_e1873_d_n13 * s.v[303]) + (eq74_e1873 * s.dn[303][13]));
        let eq74_e1875_d_n14: f64 = ((eq74_e1873_d_n14 * s.v[303]) + (eq74_e1873 * s.dn[303][14]));
        let eq74_e1875_d_n15: f64 = ((eq74_e1873_d_n15 * s.v[303]) + (eq74_e1873 * s.dn[303][15]));
        let eq74_e1875_d_n16: f64 = ((eq74_e1873_d_n16 * s.v[303]) + (eq74_e1873 * s.dn[303][16]));
        let eq74_e1878: f64 = ((nv11 - nv7) * p.p28);
        let eq74_e1878_d_n7: f64 = (-p.p28);
        let eq74_e1878_d_n11: f64 = p.p28;
        let eq74_e1880: f64 = (eq74_e1878 * s.v[781]);
        let eq74_e1880_d_n7: f64 = (eq74_e1878_d_n7 * s.v[781]);
        let eq74_e1880_d_n11: f64 = (eq74_e1878_d_n11 * s.v[781]);
        let eq74_e1881: f64 = (eq74_e1875 + eq74_e1880);
        let eq74_e1881_d_n7: f64 = (eq74_e1875_d_n7 + eq74_e1880_d_n7);
        let eq74_e1881_d_n11: f64 = (eq74_e1875_d_n11 + eq74_e1880_d_n11);
        (eq74_e1881, eq74_e1875_d_n0, eq74_e1875_d_n1, eq74_e1875_d_n2, eq74_e1875_d_n3, eq74_e1875_d_n4, eq74_e1875_d_n5, eq74_e1875_d_n6, eq74_e1881_d_n7, eq74_e1875_d_n8, eq74_e1875_d_n9, eq74_e1875_d_n10, eq74_e1881_d_n11, eq74_e1875_d_n12, eq74_e1875_d_n13, eq74_e1875_d_n14, eq74_e1875_d_n15, eq74_e1875_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1883;
        let eq74_node_derivatives: [f64; 17] = [eq74_e1883_d_n0, eq74_e1883_d_n1, eq74_e1883_d_n2, eq74_e1883_d_n3, eq74_e1883_d_n4, eq74_e1883_d_n5, eq74_e1883_d_n6, eq74_e1883_d_n7, eq74_e1883_d_n8, eq74_e1883_d_n9, eq74_e1883_d_n10, eq74_e1883_d_n11, eq74_e1883_d_n12, eq74_e1883_d_n13, eq74_e1883_d_n14, eq74_e1883_d_n15, eq74_e1883_d_n16];
        let eq74_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq74_value),
            &nodes,
            &eq74_node_derivatives,
            &branches,
            &eq74_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_75_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq75_e1898, eq75_e1898_d_n0, eq75_e1898_d_n1, eq75_e1898_d_n2, eq75_e1898_d_n3, eq75_e1898_d_n4, eq75_e1898_d_n5, eq75_e1898_d_n6, eq75_e1898_d_n7, eq75_e1898_d_n8, eq75_e1898_d_n9, eq75_e1898_d_n10, eq75_e1898_d_n11, eq75_e1898_d_n12, eq75_e1898_d_n13, eq75_e1898_d_n14, eq75_e1898_d_n15, eq75_e1898_d_n16,) = {
    if (!(s.v[1627] != 0.0)) {
        let eq75_e1888: f64 = (s.v[187] * p.p28);
        let eq75_e1888_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq75_e1888_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq75_e1888_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq75_e1888_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq75_e1888_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq75_e1888_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq75_e1888_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq75_e1888_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq75_e1888_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq75_e1888_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq75_e1888_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq75_e1888_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq75_e1888_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq75_e1888_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq75_e1888_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq75_e1888_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq75_e1888_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq75_e1890: f64 = (eq75_e1888 * s.v[304]);
        let eq75_e1890_d_n0: f64 = ((eq75_e1888_d_n0 * s.v[304]) + (eq75_e1888 * s.dn[304][0]));
        let eq75_e1890_d_n1: f64 = ((eq75_e1888_d_n1 * s.v[304]) + (eq75_e1888 * s.dn[304][1]));
        let eq75_e1890_d_n2: f64 = ((eq75_e1888_d_n2 * s.v[304]) + (eq75_e1888 * s.dn[304][2]));
        let eq75_e1890_d_n3: f64 = ((eq75_e1888_d_n3 * s.v[304]) + (eq75_e1888 * s.dn[304][3]));
        let eq75_e1890_d_n4: f64 = ((eq75_e1888_d_n4 * s.v[304]) + (eq75_e1888 * s.dn[304][4]));
        let eq75_e1890_d_n5: f64 = ((eq75_e1888_d_n5 * s.v[304]) + (eq75_e1888 * s.dn[304][5]));
        let eq75_e1890_d_n6: f64 = ((eq75_e1888_d_n6 * s.v[304]) + (eq75_e1888 * s.dn[304][6]));
        let eq75_e1890_d_n7: f64 = ((eq75_e1888_d_n7 * s.v[304]) + (eq75_e1888 * s.dn[304][7]));
        let eq75_e1890_d_n8: f64 = ((eq75_e1888_d_n8 * s.v[304]) + (eq75_e1888 * s.dn[304][8]));
        let eq75_e1890_d_n9: f64 = ((eq75_e1888_d_n9 * s.v[304]) + (eq75_e1888 * s.dn[304][9]));
        let eq75_e1890_d_n10: f64 = ((eq75_e1888_d_n10 * s.v[304]) + (eq75_e1888 * s.dn[304][10]));
        let eq75_e1890_d_n11: f64 = ((eq75_e1888_d_n11 * s.v[304]) + (eq75_e1888 * s.dn[304][11]));
        let eq75_e1890_d_n12: f64 = ((eq75_e1888_d_n12 * s.v[304]) + (eq75_e1888 * s.dn[304][12]));
        let eq75_e1890_d_n13: f64 = ((eq75_e1888_d_n13 * s.v[304]) + (eq75_e1888 * s.dn[304][13]));
        let eq75_e1890_d_n14: f64 = ((eq75_e1888_d_n14 * s.v[304]) + (eq75_e1888 * s.dn[304][14]));
        let eq75_e1890_d_n15: f64 = ((eq75_e1888_d_n15 * s.v[304]) + (eq75_e1888 * s.dn[304][15]));
        let eq75_e1890_d_n16: f64 = ((eq75_e1888_d_n16 * s.v[304]) + (eq75_e1888 * s.dn[304][16]));
        let eq75_e1893: f64 = ((nv11 - nv5) * p.p28);
        let eq75_e1893_d_n5: f64 = (-p.p28);
        let eq75_e1893_d_n11: f64 = p.p28;
        let eq75_e1895: f64 = (eq75_e1893 * s.v[781]);
        let eq75_e1895_d_n5: f64 = (eq75_e1893_d_n5 * s.v[781]);
        let eq75_e1895_d_n11: f64 = (eq75_e1893_d_n11 * s.v[781]);
        let eq75_e1896: f64 = (eq75_e1890 + eq75_e1895);
        let eq75_e1896_d_n5: f64 = (eq75_e1890_d_n5 + eq75_e1895_d_n5);
        let eq75_e1896_d_n11: f64 = (eq75_e1890_d_n11 + eq75_e1895_d_n11);
        (eq75_e1896, eq75_e1890_d_n0, eq75_e1890_d_n1, eq75_e1890_d_n2, eq75_e1890_d_n3, eq75_e1890_d_n4, eq75_e1896_d_n5, eq75_e1890_d_n6, eq75_e1890_d_n7, eq75_e1890_d_n8, eq75_e1890_d_n9, eq75_e1890_d_n10, eq75_e1896_d_n11, eq75_e1890_d_n12, eq75_e1890_d_n13, eq75_e1890_d_n14, eq75_e1890_d_n15, eq75_e1890_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1898;
        let eq75_node_derivatives: [f64; 17] = [eq75_e1898_d_n0, eq75_e1898_d_n1, eq75_e1898_d_n2, eq75_e1898_d_n3, eq75_e1898_d_n4, eq75_e1898_d_n5, eq75_e1898_d_n6, eq75_e1898_d_n7, eq75_e1898_d_n8, eq75_e1898_d_n9, eq75_e1898_d_n10, eq75_e1898_d_n11, eq75_e1898_d_n12, eq75_e1898_d_n13, eq75_e1898_d_n14, eq75_e1898_d_n15, eq75_e1898_d_n16];
        let eq75_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            self.multiplicity * (eq75_value),
            &nodes,
            &eq75_node_derivatives,
            &branches,
            &eq75_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_76_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n1, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_d_n15, eq76_e1908_d_n16,) = {
    if (!(s.v[1627] != 0.0)) {
        let eq76_e1904: f64 = (p.p29 * s.v[330]);
        let eq76_e1904_d_n0: f64 = (p.p29 * s.dn[330][0]);
        let eq76_e1904_d_n1: f64 = (p.p29 * s.dn[330][1]);
        let eq76_e1904_d_n2: f64 = (p.p29 * s.dn[330][2]);
        let eq76_e1904_d_n3: f64 = (p.p29 * s.dn[330][3]);
        let eq76_e1904_d_n4: f64 = (p.p29 * s.dn[330][4]);
        let eq76_e1904_d_n5: f64 = (p.p29 * s.dn[330][5]);
        let eq76_e1904_d_n6: f64 = (p.p29 * s.dn[330][6]);
        let eq76_e1904_d_n7: f64 = (p.p29 * s.dn[330][7]);
        let eq76_e1904_d_n8: f64 = (p.p29 * s.dn[330][8]);
        let eq76_e1904_d_n9: f64 = (p.p29 * s.dn[330][9]);
        let eq76_e1904_d_n10: f64 = (p.p29 * s.dn[330][10]);
        let eq76_e1904_d_n11: f64 = (p.p29 * s.dn[330][11]);
        let eq76_e1904_d_n12: f64 = (p.p29 * s.dn[330][12]);
        let eq76_e1904_d_n13: f64 = (p.p29 * s.dn[330][13]);
        let eq76_e1904_d_n14: f64 = (p.p29 * s.dn[330][14]);
        let eq76_e1904_d_n15: f64 = (p.p29 * s.dn[330][15]);
        let eq76_e1904_d_n16: f64 = (p.p29 * s.dn[330][16]);
        let eq76_e1905: f64 = self.eval_ddt(12, eq76_e1904);
        let eq76_e1905_d_n0: f64 = self.ddt_jacobian(eq76_e1904_d_n0);
        let eq76_e1905_d_n1: f64 = self.ddt_jacobian(eq76_e1904_d_n1);
        let eq76_e1905_d_n2: f64 = self.ddt_jacobian(eq76_e1904_d_n2);
        let eq76_e1905_d_n3: f64 = self.ddt_jacobian(eq76_e1904_d_n3);
        let eq76_e1905_d_n4: f64 = self.ddt_jacobian(eq76_e1904_d_n4);
        let eq76_e1905_d_n5: f64 = self.ddt_jacobian(eq76_e1904_d_n5);
        let eq76_e1905_d_n6: f64 = self.ddt_jacobian(eq76_e1904_d_n6);
        let eq76_e1905_d_n7: f64 = self.ddt_jacobian(eq76_e1904_d_n7);
        let eq76_e1905_d_n8: f64 = self.ddt_jacobian(eq76_e1904_d_n8);
        let eq76_e1905_d_n9: f64 = self.ddt_jacobian(eq76_e1904_d_n9);
        let eq76_e1905_d_n10: f64 = self.ddt_jacobian(eq76_e1904_d_n10);
        let eq76_e1905_d_n11: f64 = self.ddt_jacobian(eq76_e1904_d_n11);
        let eq76_e1905_d_n12: f64 = self.ddt_jacobian(eq76_e1904_d_n12);
        let eq76_e1905_d_n13: f64 = self.ddt_jacobian(eq76_e1904_d_n13);
        let eq76_e1905_d_n14: f64 = self.ddt_jacobian(eq76_e1904_d_n14);
        let eq76_e1905_d_n15: f64 = self.ddt_jacobian(eq76_e1904_d_n15);
        let eq76_e1905_d_n16: f64 = self.ddt_jacobian(eq76_e1904_d_n16);
        let eq76_e1906: f64 = (s.v[187] * eq76_e1905);
        let eq76_e1906_d_n0: f64 = ((s.dn[187][0] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n0));
        let eq76_e1906_d_n1: f64 = ((s.dn[187][1] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n1));
        let eq76_e1906_d_n2: f64 = ((s.dn[187][2] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n2));
        let eq76_e1906_d_n3: f64 = ((s.dn[187][3] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n3));
        let eq76_e1906_d_n4: f64 = ((s.dn[187][4] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n4));
        let eq76_e1906_d_n5: f64 = ((s.dn[187][5] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n5));
        let eq76_e1906_d_n6: f64 = ((s.dn[187][6] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n6));
        let eq76_e1906_d_n7: f64 = ((s.dn[187][7] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n7));
        let eq76_e1906_d_n8: f64 = ((s.dn[187][8] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n8));
        let eq76_e1906_d_n9: f64 = ((s.dn[187][9] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n9));
        let eq76_e1906_d_n10: f64 = ((s.dn[187][10] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n10));
        let eq76_e1906_d_n11: f64 = ((s.dn[187][11] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n11));
        let eq76_e1906_d_n12: f64 = ((s.dn[187][12] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n12));
        let eq76_e1906_d_n13: f64 = ((s.dn[187][13] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n13));
        let eq76_e1906_d_n14: f64 = ((s.dn[187][14] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n14));
        let eq76_e1906_d_n15: f64 = ((s.dn[187][15] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n15));
        let eq76_e1906_d_n16: f64 = ((s.dn[187][16] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n16));
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n1, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14, eq76_e1906_d_n15, eq76_e1906_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1908;
        let eq76_node_derivatives: [f64; 17] = [eq76_e1908_d_n0, eq76_e1908_d_n1, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_d_n15, eq76_e1908_d_n16];
        let eq76_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq76_value),
            &nodes,
            &eq76_node_derivatives,
            &branches,
            &eq76_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_77_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n1, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_d_n15, eq77_e1918_d_n16,) = {
    if (!(s.v[1627] != 0.0)) {
        let eq77_e1914: f64 = (p.p29 * s.v[334]);
        let eq77_e1914_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq77_e1914_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq77_e1914_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq77_e1914_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq77_e1914_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq77_e1914_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq77_e1914_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq77_e1914_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq77_e1914_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq77_e1914_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq77_e1914_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq77_e1914_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq77_e1914_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq77_e1914_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq77_e1914_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq77_e1914_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq77_e1914_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq77_e1915: f64 = self.eval_ddt(13, eq77_e1914);
        let eq77_e1915_d_n0: f64 = self.ddt_jacobian(eq77_e1914_d_n0);
        let eq77_e1915_d_n1: f64 = self.ddt_jacobian(eq77_e1914_d_n1);
        let eq77_e1915_d_n2: f64 = self.ddt_jacobian(eq77_e1914_d_n2);
        let eq77_e1915_d_n3: f64 = self.ddt_jacobian(eq77_e1914_d_n3);
        let eq77_e1915_d_n4: f64 = self.ddt_jacobian(eq77_e1914_d_n4);
        let eq77_e1915_d_n5: f64 = self.ddt_jacobian(eq77_e1914_d_n5);
        let eq77_e1915_d_n6: f64 = self.ddt_jacobian(eq77_e1914_d_n6);
        let eq77_e1915_d_n7: f64 = self.ddt_jacobian(eq77_e1914_d_n7);
        let eq77_e1915_d_n8: f64 = self.ddt_jacobian(eq77_e1914_d_n8);
        let eq77_e1915_d_n9: f64 = self.ddt_jacobian(eq77_e1914_d_n9);
        let eq77_e1915_d_n10: f64 = self.ddt_jacobian(eq77_e1914_d_n10);
        let eq77_e1915_d_n11: f64 = self.ddt_jacobian(eq77_e1914_d_n11);
        let eq77_e1915_d_n12: f64 = self.ddt_jacobian(eq77_e1914_d_n12);
        let eq77_e1915_d_n13: f64 = self.ddt_jacobian(eq77_e1914_d_n13);
        let eq77_e1915_d_n14: f64 = self.ddt_jacobian(eq77_e1914_d_n14);
        let eq77_e1915_d_n15: f64 = self.ddt_jacobian(eq77_e1914_d_n15);
        let eq77_e1915_d_n16: f64 = self.ddt_jacobian(eq77_e1914_d_n16);
        let eq77_e1916: f64 = (s.v[187] * eq77_e1915);
        let eq77_e1916_d_n0: f64 = ((s.dn[187][0] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n0));
        let eq77_e1916_d_n1: f64 = ((s.dn[187][1] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n1));
        let eq77_e1916_d_n2: f64 = ((s.dn[187][2] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n2));
        let eq77_e1916_d_n3: f64 = ((s.dn[187][3] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n3));
        let eq77_e1916_d_n4: f64 = ((s.dn[187][4] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n4));
        let eq77_e1916_d_n5: f64 = ((s.dn[187][5] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n5));
        let eq77_e1916_d_n6: f64 = ((s.dn[187][6] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n6));
        let eq77_e1916_d_n7: f64 = ((s.dn[187][7] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n7));
        let eq77_e1916_d_n8: f64 = ((s.dn[187][8] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n8));
        let eq77_e1916_d_n9: f64 = ((s.dn[187][9] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n9));
        let eq77_e1916_d_n10: f64 = ((s.dn[187][10] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n10));
        let eq77_e1916_d_n11: f64 = ((s.dn[187][11] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n11));
        let eq77_e1916_d_n12: f64 = ((s.dn[187][12] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n12));
        let eq77_e1916_d_n13: f64 = ((s.dn[187][13] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n13));
        let eq77_e1916_d_n14: f64 = ((s.dn[187][14] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n14));
        let eq77_e1916_d_n15: f64 = ((s.dn[187][15] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n15));
        let eq77_e1916_d_n16: f64 = ((s.dn[187][16] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n16));
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n1, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14, eq77_e1916_d_n15, eq77_e1916_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1918;
        let eq77_node_derivatives: [f64; 17] = [eq77_e1918_d_n0, eq77_e1918_d_n1, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_d_n15, eq77_e1918_d_n16];
        let eq77_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            self.multiplicity * (eq77_value),
            &nodes,
            &eq77_node_derivatives,
            &branches,
            &eq77_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_78_block_0(
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq78_e1926, eq78_e1926_d_n0, eq78_e1926_d_n1, eq78_e1926_d_n2, eq78_e1926_d_n3, eq78_e1926_d_n4, eq78_e1926_d_n5, eq78_e1926_d_n6, eq78_e1926_d_n7, eq78_e1926_d_n8, eq78_e1926_d_n9, eq78_e1926_d_n10, eq78_e1926_d_n11, eq78_e1926_d_n12, eq78_e1926_d_n13, eq78_e1926_d_n14, eq78_e1926_d_n15, eq78_e1926_d_n16,) = {
    if (s.v[1629] != 0.0) {
        let eq78_e1922: f64 = ((nv14 - nv0) * p.p28);
        let eq78_e1922_d_n0: f64 = (-p.p28);
        let eq78_e1922_d_n14: f64 = p.p28;
        let eq78_e1924: f64 = (eq78_e1922 * s.v[276]);
        let eq78_e1924_d_n0: f64 = ((eq78_e1922_d_n0 * s.v[276]) + (eq78_e1922 * s.dn[276][0]));
        let eq78_e1924_d_n1: f64 = (eq78_e1922 * s.dn[276][1]);
        let eq78_e1924_d_n2: f64 = (eq78_e1922 * s.dn[276][2]);
        let eq78_e1924_d_n3: f64 = (eq78_e1922 * s.dn[276][3]);
        let eq78_e1924_d_n4: f64 = (eq78_e1922 * s.dn[276][4]);
        let eq78_e1924_d_n5: f64 = (eq78_e1922 * s.dn[276][5]);
        let eq78_e1924_d_n6: f64 = (eq78_e1922 * s.dn[276][6]);
        let eq78_e1924_d_n7: f64 = (eq78_e1922 * s.dn[276][7]);
        let eq78_e1924_d_n8: f64 = (eq78_e1922 * s.dn[276][8]);
        let eq78_e1924_d_n9: f64 = (eq78_e1922 * s.dn[276][9]);
        let eq78_e1924_d_n10: f64 = (eq78_e1922 * s.dn[276][10]);
        let eq78_e1924_d_n11: f64 = (eq78_e1922 * s.dn[276][11]);
        let eq78_e1924_d_n12: f64 = (eq78_e1922 * s.dn[276][12]);
        let eq78_e1924_d_n13: f64 = (eq78_e1922 * s.dn[276][13]);
        let eq78_e1924_d_n14: f64 = ((eq78_e1922_d_n14 * s.v[276]) + (eq78_e1922 * s.dn[276][14]));
        let eq78_e1924_d_n15: f64 = (eq78_e1922 * s.dn[276][15]);
        let eq78_e1924_d_n16: f64 = (eq78_e1922 * s.dn[276][16]);
        (eq78_e1924, eq78_e1924_d_n0, eq78_e1924_d_n1, eq78_e1924_d_n2, eq78_e1924_d_n3, eq78_e1924_d_n4, eq78_e1924_d_n5, eq78_e1924_d_n6, eq78_e1924_d_n7, eq78_e1924_d_n8, eq78_e1924_d_n9, eq78_e1924_d_n10, eq78_e1924_d_n11, eq78_e1924_d_n12, eq78_e1924_d_n13, eq78_e1924_d_n14, eq78_e1924_d_n15, eq78_e1924_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_value: f64 = eq78_e1926;
        let eq78_node_derivatives: [f64; 17] = [eq78_e1926_d_n0, eq78_e1926_d_n1, eq78_e1926_d_n2, eq78_e1926_d_n3, eq78_e1926_d_n4, eq78_e1926_d_n5, eq78_e1926_d_n6, eq78_e1926_d_n7, eq78_e1926_d_n8, eq78_e1926_d_n9, eq78_e1926_d_n10, eq78_e1926_d_n11, eq78_e1926_d_n12, eq78_e1926_d_n13, eq78_e1926_d_n14, eq78_e1926_d_n15, eq78_e1926_d_n16];
        let eq78_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[0]),
            self.multiplicity * (eq78_value),
            &nodes,
            &eq78_node_derivatives,
            &branches,
            &eq78_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_79_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq79_e1936,) = {
    if (s.v[1629] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq79_value: f64 = eq79_e1936;
        stamper.stamp_current(
            Some(nodes[14]),
            Some(nodes[0]),
            self.multiplicity * (eq79_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_80_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq80_e1941,) = {
    if (!(s.v[1629] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e1941;
        stamper.stamp_potential(
            branches[13],
            eq80_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_81_block_0(
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq81_e1959, eq81_e1959_d_n0, eq81_e1959_d_n1, eq81_e1959_d_n2, eq81_e1959_d_n3, eq81_e1959_d_n4, eq81_e1959_d_n5, eq81_e1959_d_n6, eq81_e1959_d_n7, eq81_e1959_d_n8, eq81_e1959_d_n9, eq81_e1959_d_n10, eq81_e1959_d_n11, eq81_e1959_d_n12, eq81_e1959_d_n13, eq81_e1959_d_n14, eq81_e1959_d_n15, eq81_e1959_d_n16,) = {
    if (s.v[1630] != 0.0) {
        let eq81_e1945: f64 = (s.v[187] * p.p28);
        let eq81_e1945_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq81_e1945_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq81_e1945_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq81_e1945_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq81_e1945_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq81_e1945_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq81_e1945_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq81_e1945_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq81_e1945_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq81_e1945_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq81_e1945_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq81_e1945_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq81_e1945_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq81_e1945_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq81_e1945_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq81_e1945_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq81_e1945_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq81_e1947: f64 = (eq81_e1945 * s.v[304]);
        let eq81_e1947_d_n0: f64 = ((eq81_e1945_d_n0 * s.v[304]) + (eq81_e1945 * s.dn[304][0]));
        let eq81_e1947_d_n1: f64 = ((eq81_e1945_d_n1 * s.v[304]) + (eq81_e1945 * s.dn[304][1]));
        let eq81_e1947_d_n2: f64 = ((eq81_e1945_d_n2 * s.v[304]) + (eq81_e1945 * s.dn[304][2]));
        let eq81_e1947_d_n3: f64 = ((eq81_e1945_d_n3 * s.v[304]) + (eq81_e1945 * s.dn[304][3]));
        let eq81_e1947_d_n4: f64 = ((eq81_e1945_d_n4 * s.v[304]) + (eq81_e1945 * s.dn[304][4]));
        let eq81_e1947_d_n5: f64 = ((eq81_e1945_d_n5 * s.v[304]) + (eq81_e1945 * s.dn[304][5]));
        let eq81_e1947_d_n6: f64 = ((eq81_e1945_d_n6 * s.v[304]) + (eq81_e1945 * s.dn[304][6]));
        let eq81_e1947_d_n7: f64 = ((eq81_e1945_d_n7 * s.v[304]) + (eq81_e1945 * s.dn[304][7]));
        let eq81_e1947_d_n8: f64 = ((eq81_e1945_d_n8 * s.v[304]) + (eq81_e1945 * s.dn[304][8]));
        let eq81_e1947_d_n9: f64 = ((eq81_e1945_d_n9 * s.v[304]) + (eq81_e1945 * s.dn[304][9]));
        let eq81_e1947_d_n10: f64 = ((eq81_e1945_d_n10 * s.v[304]) + (eq81_e1945 * s.dn[304][10]));
        let eq81_e1947_d_n11: f64 = ((eq81_e1945_d_n11 * s.v[304]) + (eq81_e1945 * s.dn[304][11]));
        let eq81_e1947_d_n12: f64 = ((eq81_e1945_d_n12 * s.v[304]) + (eq81_e1945 * s.dn[304][12]));
        let eq81_e1947_d_n13: f64 = ((eq81_e1945_d_n13 * s.v[304]) + (eq81_e1945 * s.dn[304][13]));
        let eq81_e1947_d_n14: f64 = ((eq81_e1945_d_n14 * s.v[304]) + (eq81_e1945 * s.dn[304][14]));
        let eq81_e1947_d_n15: f64 = ((eq81_e1945_d_n15 * s.v[304]) + (eq81_e1945 * s.dn[304][15]));
        let eq81_e1947_d_n16: f64 = ((eq81_e1945_d_n16 * s.v[304]) + (eq81_e1945 * s.dn[304][16]));
        let eq81_e1950: f64 = (1.0 - p.p1128);
        let eq81_e1952: f64 = (eq81_e1950 * p.p28);
        let eq81_e1954: f64 = (eq81_e1952 * (nv13 - nv5));
        let eq81_e1954_d_n5: f64 = (-eq81_e1952);
        let eq81_e1956: f64 = (eq81_e1954 * s.v[781]);
        let eq81_e1956_d_n5: f64 = (eq81_e1954_d_n5 * s.v[781]);
        let eq81_e1956_d_n13: f64 = (eq81_e1952 * s.v[781]);
        let eq81_e1957: f64 = (eq81_e1947 + eq81_e1956);
        let eq81_e1957_d_n5: f64 = (eq81_e1947_d_n5 + eq81_e1956_d_n5);
        let eq81_e1957_d_n13: f64 = (eq81_e1947_d_n13 + eq81_e1956_d_n13);
        (eq81_e1957, eq81_e1947_d_n0, eq81_e1947_d_n1, eq81_e1947_d_n2, eq81_e1947_d_n3, eq81_e1947_d_n4, eq81_e1957_d_n5, eq81_e1947_d_n6, eq81_e1947_d_n7, eq81_e1947_d_n8, eq81_e1947_d_n9, eq81_e1947_d_n10, eq81_e1947_d_n11, eq81_e1947_d_n12, eq81_e1957_d_n13, eq81_e1947_d_n14, eq81_e1947_d_n15, eq81_e1947_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq81_value: f64 = eq81_e1959;
        let eq81_node_derivatives: [f64; 17] = [eq81_e1959_d_n0, eq81_e1959_d_n1, eq81_e1959_d_n2, eq81_e1959_d_n3, eq81_e1959_d_n4, eq81_e1959_d_n5, eq81_e1959_d_n6, eq81_e1959_d_n7, eq81_e1959_d_n8, eq81_e1959_d_n9, eq81_e1959_d_n10, eq81_e1959_d_n11, eq81_e1959_d_n12, eq81_e1959_d_n13, eq81_e1959_d_n14, eq81_e1959_d_n15, eq81_e1959_d_n16];
        let eq81_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            self.multiplicity * (eq81_value),
            &nodes,
            &eq81_node_derivatives,
            &branches,
            &eq81_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_82_block_0(
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
        let (eq82_e1975, eq82_e1975_d_n0, eq82_e1975_d_n1, eq82_e1975_d_n2, eq82_e1975_d_n3, eq82_e1975_d_n4, eq82_e1975_d_n5, eq82_e1975_d_n6, eq82_e1975_d_n7, eq82_e1975_d_n8, eq82_e1975_d_n9, eq82_e1975_d_n10, eq82_e1975_d_n11, eq82_e1975_d_n12, eq82_e1975_d_n13, eq82_e1975_d_n14, eq82_e1975_d_n15, eq82_e1975_d_n16,) = {
    if (s.v[1630] != 0.0) {
        let eq82_e1963: f64 = (s.v[187] * p.p28);
        let eq82_e1963_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq82_e1963_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq82_e1963_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq82_e1963_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq82_e1963_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq82_e1963_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq82_e1963_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq82_e1963_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq82_e1963_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq82_e1963_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq82_e1963_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq82_e1963_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq82_e1963_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq82_e1963_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq82_e1963_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq82_e1963_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq82_e1963_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq82_e1965: f64 = (eq82_e1963 * s.v[305]);
        let eq82_e1965_d_n0: f64 = ((eq82_e1963_d_n0 * s.v[305]) + (eq82_e1963 * s.dn[305][0]));
        let eq82_e1965_d_n1: f64 = ((eq82_e1963_d_n1 * s.v[305]) + (eq82_e1963 * s.dn[305][1]));
        let eq82_e1965_d_n2: f64 = ((eq82_e1963_d_n2 * s.v[305]) + (eq82_e1963 * s.dn[305][2]));
        let eq82_e1965_d_n3: f64 = ((eq82_e1963_d_n3 * s.v[305]) + (eq82_e1963 * s.dn[305][3]));
        let eq82_e1965_d_n4: f64 = ((eq82_e1963_d_n4 * s.v[305]) + (eq82_e1963 * s.dn[305][4]));
        let eq82_e1965_d_n5: f64 = ((eq82_e1963_d_n5 * s.v[305]) + (eq82_e1963 * s.dn[305][5]));
        let eq82_e1965_d_n6: f64 = ((eq82_e1963_d_n6 * s.v[305]) + (eq82_e1963 * s.dn[305][6]));
        let eq82_e1965_d_n7: f64 = ((eq82_e1963_d_n7 * s.v[305]) + (eq82_e1963 * s.dn[305][7]));
        let eq82_e1965_d_n8: f64 = ((eq82_e1963_d_n8 * s.v[305]) + (eq82_e1963 * s.dn[305][8]));
        let eq82_e1965_d_n9: f64 = ((eq82_e1963_d_n9 * s.v[305]) + (eq82_e1963 * s.dn[305][9]));
        let eq82_e1965_d_n10: f64 = ((eq82_e1963_d_n10 * s.v[305]) + (eq82_e1963 * s.dn[305][10]));
        let eq82_e1965_d_n11: f64 = ((eq82_e1963_d_n11 * s.v[305]) + (eq82_e1963 * s.dn[305][11]));
        let eq82_e1965_d_n12: f64 = ((eq82_e1963_d_n12 * s.v[305]) + (eq82_e1963 * s.dn[305][12]));
        let eq82_e1965_d_n13: f64 = ((eq82_e1963_d_n13 * s.v[305]) + (eq82_e1963 * s.dn[305][13]));
        let eq82_e1965_d_n14: f64 = ((eq82_e1963_d_n14 * s.v[305]) + (eq82_e1963 * s.dn[305][14]));
        let eq82_e1965_d_n15: f64 = ((eq82_e1963_d_n15 * s.v[305]) + (eq82_e1963 * s.dn[305][15]));
        let eq82_e1965_d_n16: f64 = ((eq82_e1963_d_n16 * s.v[305]) + (eq82_e1963 * s.dn[305][16]));
        let eq82_e1968: f64 = (p.p1128 * p.p28);
        let eq82_e1970: f64 = (eq82_e1968 * (nv13 - nv14));
        let eq82_e1970_d_n14: f64 = (-eq82_e1968);
        let eq82_e1972: f64 = (eq82_e1970 * s.v[781]);
        let eq82_e1972_d_n13: f64 = (eq82_e1968 * s.v[781]);
        let eq82_e1972_d_n14: f64 = (eq82_e1970_d_n14 * s.v[781]);
        let eq82_e1973: f64 = (eq82_e1965 + eq82_e1972);
        let eq82_e1973_d_n13: f64 = (eq82_e1965_d_n13 + eq82_e1972_d_n13);
        let eq82_e1973_d_n14: f64 = (eq82_e1965_d_n14 + eq82_e1972_d_n14);
        (eq82_e1973, eq82_e1965_d_n0, eq82_e1965_d_n1, eq82_e1965_d_n2, eq82_e1965_d_n3, eq82_e1965_d_n4, eq82_e1965_d_n5, eq82_e1965_d_n6, eq82_e1965_d_n7, eq82_e1965_d_n8, eq82_e1965_d_n9, eq82_e1965_d_n10, eq82_e1965_d_n11, eq82_e1965_d_n12, eq82_e1973_d_n13, eq82_e1973_d_n14, eq82_e1965_d_n15, eq82_e1965_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e1975;
        let eq82_node_derivatives: [f64; 17] = [eq82_e1975_d_n0, eq82_e1975_d_n1, eq82_e1975_d_n2, eq82_e1975_d_n3, eq82_e1975_d_n4, eq82_e1975_d_n5, eq82_e1975_d_n6, eq82_e1975_d_n7, eq82_e1975_d_n8, eq82_e1975_d_n9, eq82_e1975_d_n10, eq82_e1975_d_n11, eq82_e1975_d_n12, eq82_e1975_d_n13, eq82_e1975_d_n14, eq82_e1975_d_n15, eq82_e1975_d_n16];
        let eq82_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[14]),
            self.multiplicity * (eq82_value),
            &nodes,
            &eq82_node_derivatives,
            &branches,
            &eq82_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_83_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n1, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_d_n15, eq83_e1984_d_n16,) = {
    if (s.v[1630] != 0.0) {
        let eq83_e1980: f64 = (p.p29 * s.v[334]);
        let eq83_e1980_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq83_e1980_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq83_e1980_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq83_e1980_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq83_e1980_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq83_e1980_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq83_e1980_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq83_e1980_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq83_e1980_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq83_e1980_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq83_e1980_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq83_e1980_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq83_e1980_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq83_e1980_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq83_e1980_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq83_e1980_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq83_e1980_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq83_e1981: f64 = self.eval_ddt(14, eq83_e1980);
        let eq83_e1981_d_n0: f64 = self.ddt_jacobian(eq83_e1980_d_n0);
        let eq83_e1981_d_n1: f64 = self.ddt_jacobian(eq83_e1980_d_n1);
        let eq83_e1981_d_n2: f64 = self.ddt_jacobian(eq83_e1980_d_n2);
        let eq83_e1981_d_n3: f64 = self.ddt_jacobian(eq83_e1980_d_n3);
        let eq83_e1981_d_n4: f64 = self.ddt_jacobian(eq83_e1980_d_n4);
        let eq83_e1981_d_n5: f64 = self.ddt_jacobian(eq83_e1980_d_n5);
        let eq83_e1981_d_n6: f64 = self.ddt_jacobian(eq83_e1980_d_n6);
        let eq83_e1981_d_n7: f64 = self.ddt_jacobian(eq83_e1980_d_n7);
        let eq83_e1981_d_n8: f64 = self.ddt_jacobian(eq83_e1980_d_n8);
        let eq83_e1981_d_n9: f64 = self.ddt_jacobian(eq83_e1980_d_n9);
        let eq83_e1981_d_n10: f64 = self.ddt_jacobian(eq83_e1980_d_n10);
        let eq83_e1981_d_n11: f64 = self.ddt_jacobian(eq83_e1980_d_n11);
        let eq83_e1981_d_n12: f64 = self.ddt_jacobian(eq83_e1980_d_n12);
        let eq83_e1981_d_n13: f64 = self.ddt_jacobian(eq83_e1980_d_n13);
        let eq83_e1981_d_n14: f64 = self.ddt_jacobian(eq83_e1980_d_n14);
        let eq83_e1981_d_n15: f64 = self.ddt_jacobian(eq83_e1980_d_n15);
        let eq83_e1981_d_n16: f64 = self.ddt_jacobian(eq83_e1980_d_n16);
        let eq83_e1982: f64 = (s.v[187] * eq83_e1981);
        let eq83_e1982_d_n0: f64 = ((s.dn[187][0] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n0));
        let eq83_e1982_d_n1: f64 = ((s.dn[187][1] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n1));
        let eq83_e1982_d_n2: f64 = ((s.dn[187][2] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n2));
        let eq83_e1982_d_n3: f64 = ((s.dn[187][3] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n3));
        let eq83_e1982_d_n4: f64 = ((s.dn[187][4] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n4));
        let eq83_e1982_d_n5: f64 = ((s.dn[187][5] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n5));
        let eq83_e1982_d_n6: f64 = ((s.dn[187][6] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n6));
        let eq83_e1982_d_n7: f64 = ((s.dn[187][7] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n7));
        let eq83_e1982_d_n8: f64 = ((s.dn[187][8] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n8));
        let eq83_e1982_d_n9: f64 = ((s.dn[187][9] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n9));
        let eq83_e1982_d_n10: f64 = ((s.dn[187][10] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n10));
        let eq83_e1982_d_n11: f64 = ((s.dn[187][11] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n11));
        let eq83_e1982_d_n12: f64 = ((s.dn[187][12] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n12));
        let eq83_e1982_d_n13: f64 = ((s.dn[187][13] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n13));
        let eq83_e1982_d_n14: f64 = ((s.dn[187][14] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n14));
        let eq83_e1982_d_n15: f64 = ((s.dn[187][15] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n15));
        let eq83_e1982_d_n16: f64 = ((s.dn[187][16] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n16));
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n1, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14, eq83_e1982_d_n15, eq83_e1982_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1984;
        let eq83_node_derivatives: [f64; 17] = [eq83_e1984_d_n0, eq83_e1984_d_n1, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_d_n15, eq83_e1984_d_n16];
        let eq83_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            self.multiplicity * (eq83_value),
            &nodes,
            &eq83_node_derivatives,
            &branches,
            &eq83_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_84_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n1, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_d_n15, eq84_e1993_d_n16,) = {
    if (s.v[1630] != 0.0) {
        let eq84_e1989: f64 = (p.p29 * s.v[338]);
        let eq84_e1989_d_n0: f64 = (p.p29 * s.dn[338][0]);
        let eq84_e1989_d_n1: f64 = (p.p29 * s.dn[338][1]);
        let eq84_e1989_d_n2: f64 = (p.p29 * s.dn[338][2]);
        let eq84_e1989_d_n3: f64 = (p.p29 * s.dn[338][3]);
        let eq84_e1989_d_n4: f64 = (p.p29 * s.dn[338][4]);
        let eq84_e1989_d_n5: f64 = (p.p29 * s.dn[338][5]);
        let eq84_e1989_d_n6: f64 = (p.p29 * s.dn[338][6]);
        let eq84_e1989_d_n7: f64 = (p.p29 * s.dn[338][7]);
        let eq84_e1989_d_n8: f64 = (p.p29 * s.dn[338][8]);
        let eq84_e1989_d_n9: f64 = (p.p29 * s.dn[338][9]);
        let eq84_e1989_d_n10: f64 = (p.p29 * s.dn[338][10]);
        let eq84_e1989_d_n11: f64 = (p.p29 * s.dn[338][11]);
        let eq84_e1989_d_n12: f64 = (p.p29 * s.dn[338][12]);
        let eq84_e1989_d_n13: f64 = (p.p29 * s.dn[338][13]);
        let eq84_e1989_d_n14: f64 = (p.p29 * s.dn[338][14]);
        let eq84_e1989_d_n15: f64 = (p.p29 * s.dn[338][15]);
        let eq84_e1989_d_n16: f64 = (p.p29 * s.dn[338][16]);
        let eq84_e1990: f64 = self.eval_ddt(15, eq84_e1989);
        let eq84_e1990_d_n0: f64 = self.ddt_jacobian(eq84_e1989_d_n0);
        let eq84_e1990_d_n1: f64 = self.ddt_jacobian(eq84_e1989_d_n1);
        let eq84_e1990_d_n2: f64 = self.ddt_jacobian(eq84_e1989_d_n2);
        let eq84_e1990_d_n3: f64 = self.ddt_jacobian(eq84_e1989_d_n3);
        let eq84_e1990_d_n4: f64 = self.ddt_jacobian(eq84_e1989_d_n4);
        let eq84_e1990_d_n5: f64 = self.ddt_jacobian(eq84_e1989_d_n5);
        let eq84_e1990_d_n6: f64 = self.ddt_jacobian(eq84_e1989_d_n6);
        let eq84_e1990_d_n7: f64 = self.ddt_jacobian(eq84_e1989_d_n7);
        let eq84_e1990_d_n8: f64 = self.ddt_jacobian(eq84_e1989_d_n8);
        let eq84_e1990_d_n9: f64 = self.ddt_jacobian(eq84_e1989_d_n9);
        let eq84_e1990_d_n10: f64 = self.ddt_jacobian(eq84_e1989_d_n10);
        let eq84_e1990_d_n11: f64 = self.ddt_jacobian(eq84_e1989_d_n11);
        let eq84_e1990_d_n12: f64 = self.ddt_jacobian(eq84_e1989_d_n12);
        let eq84_e1990_d_n13: f64 = self.ddt_jacobian(eq84_e1989_d_n13);
        let eq84_e1990_d_n14: f64 = self.ddt_jacobian(eq84_e1989_d_n14);
        let eq84_e1990_d_n15: f64 = self.ddt_jacobian(eq84_e1989_d_n15);
        let eq84_e1990_d_n16: f64 = self.ddt_jacobian(eq84_e1989_d_n16);
        let eq84_e1991: f64 = (s.v[187] * eq84_e1990);
        let eq84_e1991_d_n0: f64 = ((s.dn[187][0] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n0));
        let eq84_e1991_d_n1: f64 = ((s.dn[187][1] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n1));
        let eq84_e1991_d_n2: f64 = ((s.dn[187][2] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n2));
        let eq84_e1991_d_n3: f64 = ((s.dn[187][3] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n3));
        let eq84_e1991_d_n4: f64 = ((s.dn[187][4] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n4));
        let eq84_e1991_d_n5: f64 = ((s.dn[187][5] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n5));
        let eq84_e1991_d_n6: f64 = ((s.dn[187][6] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n6));
        let eq84_e1991_d_n7: f64 = ((s.dn[187][7] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n7));
        let eq84_e1991_d_n8: f64 = ((s.dn[187][8] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n8));
        let eq84_e1991_d_n9: f64 = ((s.dn[187][9] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n9));
        let eq84_e1991_d_n10: f64 = ((s.dn[187][10] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n10));
        let eq84_e1991_d_n11: f64 = ((s.dn[187][11] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n11));
        let eq84_e1991_d_n12: f64 = ((s.dn[187][12] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n12));
        let eq84_e1991_d_n13: f64 = ((s.dn[187][13] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n13));
        let eq84_e1991_d_n14: f64 = ((s.dn[187][14] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n14));
        let eq84_e1991_d_n15: f64 = ((s.dn[187][15] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n15));
        let eq84_e1991_d_n16: f64 = ((s.dn[187][16] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n16));
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n1, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14, eq84_e1991_d_n15, eq84_e1991_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_value: f64 = eq84_e1993;
        let eq84_node_derivatives: [f64; 17] = [eq84_e1993_d_n0, eq84_e1993_d_n1, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_d_n15, eq84_e1993_d_n16];
        let eq84_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[14]),
            self.multiplicity * (eq84_value),
            &nodes,
            &eq84_node_derivatives,
            &branches,
            &eq84_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_8_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n1, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_d_n16, eq8_e1290_q, eq8_e1290_q_d_n0, eq8_e1290_q_d_n1, eq8_e1290_q_d_n2, eq8_e1290_q_d_n3, eq8_e1290_q_d_n4, eq8_e1290_q_d_n5, eq8_e1290_q_d_n6, eq8_e1290_q_d_n7, eq8_e1290_q_d_n8, eq8_e1290_q_d_n9, eq8_e1290_q_d_n10, eq8_e1290_q_d_n11, eq8_e1290_q_d_n12, eq8_e1290_q_d_n13, eq8_e1290_q_d_n14, eq8_e1290_q_d_n15, eq8_e1290_q_d_n16,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        let eq8_e1279: f64 = (s.v[378] * s.v[46]);
        let eq8_e1279_d_n0: f64 = (s.dn[378][0] * s.v[46]);
        let eq8_e1279_d_n1: f64 = (s.dn[378][1] * s.v[46]);
        let eq8_e1279_d_n2: f64 = (s.dn[378][2] * s.v[46]);
        let eq8_e1279_d_n3: f64 = (s.dn[378][3] * s.v[46]);
        let eq8_e1279_d_n4: f64 = (s.dn[378][4] * s.v[46]);
        let eq8_e1279_d_n5: f64 = (s.dn[378][5] * s.v[46]);
        let eq8_e1279_d_n6: f64 = (s.dn[378][6] * s.v[46]);
        let eq8_e1279_d_n7: f64 = (s.dn[378][7] * s.v[46]);
        let eq8_e1279_d_n8: f64 = (s.dn[378][8] * s.v[46]);
        let eq8_e1279_d_n9: f64 = (s.dn[378][9] * s.v[46]);
        let eq8_e1279_d_n10: f64 = (s.dn[378][10] * s.v[46]);
        let eq8_e1279_d_n11: f64 = (s.dn[378][11] * s.v[46]);
        let eq8_e1279_d_n12: f64 = (s.dn[378][12] * s.v[46]);
        let eq8_e1279_d_n13: f64 = (s.dn[378][13] * s.v[46]);
        let eq8_e1279_d_n14: f64 = (s.dn[378][14] * s.v[46]);
        let eq8_e1279_d_n15: f64 = (s.dn[378][15] * s.v[46]);
        let eq8_e1279_d_n16: f64 = (s.dn[378][16] * s.v[46]);
        let eq8_e1281: f64 = (eq8_e1279 * s.v[29]);
        let eq8_e1281_d_n0: f64 = (eq8_e1279_d_n0 * s.v[29]);
        let eq8_e1281_d_n1: f64 = (eq8_e1279_d_n1 * s.v[29]);
        let eq8_e1281_d_n2: f64 = (eq8_e1279_d_n2 * s.v[29]);
        let eq8_e1281_d_n3: f64 = (eq8_e1279_d_n3 * s.v[29]);
        let eq8_e1281_d_n4: f64 = (eq8_e1279_d_n4 * s.v[29]);
        let eq8_e1281_d_n5: f64 = (eq8_e1279_d_n5 * s.v[29]);
        let eq8_e1281_d_n6: f64 = (eq8_e1279_d_n6 * s.v[29]);
        let eq8_e1281_d_n7: f64 = (eq8_e1279_d_n7 * s.v[29]);
        let eq8_e1281_d_n8: f64 = (eq8_e1279_d_n8 * s.v[29]);
        let eq8_e1281_d_n9: f64 = (eq8_e1279_d_n9 * s.v[29]);
        let eq8_e1281_d_n10: f64 = (eq8_e1279_d_n10 * s.v[29]);
        let eq8_e1281_d_n11: f64 = (eq8_e1279_d_n11 * s.v[29]);
        let eq8_e1281_d_n12: f64 = (eq8_e1279_d_n12 * s.v[29]);
        let eq8_e1281_d_n13: f64 = (eq8_e1279_d_n13 * s.v[29]);
        let eq8_e1281_d_n14: f64 = (eq8_e1279_d_n14 * s.v[29]);
        let eq8_e1281_d_n15: f64 = (eq8_e1279_d_n15 * s.v[29]);
        let eq8_e1281_d_n16: f64 = (eq8_e1279_d_n16 * s.v[29]);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n1: f64 = (eq8_e1281_d_n1 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1283_d_n15: f64 = (eq8_e1281_d_n15 * p.p2);
        let eq8_e1283_d_n16: f64 = (eq8_e1281_d_n16 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * s.v[30]);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * s.v[30]);
        let eq8_e1285_d_n1: f64 = (eq8_e1283_d_n1 * s.v[30]);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * s.v[30]);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * s.v[30]);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * s.v[30]);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * s.v[30]);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * s.v[30]);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * s.v[30]);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * s.v[30]);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * s.v[30]);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * s.v[30]);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * s.v[30]);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * s.v[30]);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * s.v[30]);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * s.v[30]);
        let eq8_e1285_d_n15: f64 = (eq8_e1283_d_n15 * s.v[30]);
        let eq8_e1285_d_n16: f64 = (eq8_e1283_d_n16 * s.v[30]);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n1: f64 = (eq8_e1285_d_n1 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1287_d_n15: f64 = ((eq8_e1285_d_n15 * (nv15 - 0.0)) + eq8_e1285);
        let eq8_e1287_d_n16: f64 = (eq8_e1285_d_n16 * (nv15 - 0.0));
        let eq8_e1288_q: f64 = eq8_e1287;
        (eq8_e1287, eq8_e1287_d_n0, eq8_e1287_d_n1, eq8_e1287_d_n2, eq8_e1287_d_n3, eq8_e1287_d_n4, eq8_e1287_d_n5, eq8_e1287_d_n6, eq8_e1287_d_n7, eq8_e1287_d_n8, eq8_e1287_d_n9, eq8_e1287_d_n10, eq8_e1287_d_n11, eq8_e1287_d_n12, eq8_e1287_d_n13, eq8_e1287_d_n14, eq8_e1287_d_n15, eq8_e1287_d_n16, eq8_e1288_q, eq8_e1287_d_n0, eq8_e1287_d_n1, eq8_e1287_d_n2, eq8_e1287_d_n3, eq8_e1287_d_n4, eq8_e1287_d_n5, eq8_e1287_d_n6, eq8_e1287_d_n7, eq8_e1287_d_n8, eq8_e1287_d_n9, eq8_e1287_d_n10, eq8_e1287_d_n11, eq8_e1287_d_n12, eq8_e1287_d_n13, eq8_e1287_d_n14, eq8_e1287_d_n15, eq8_e1287_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 17] = [eq8_e1290_q_d_n0, eq8_e1290_q_d_n1, eq8_e1290_q_d_n2, eq8_e1290_q_d_n3, eq8_e1290_q_d_n4, eq8_e1290_q_d_n5, eq8_e1290_q_d_n6, eq8_e1290_q_d_n7, eq8_e1290_q_d_n8, eq8_e1290_q_d_n9, eq8_e1290_q_d_n10, eq8_e1290_q_d_n11, eq8_e1290_q_d_n12, eq8_e1290_q_d_n13, eq8_e1290_q_d_n14, eq8_e1290_q_d_n15, eq8_e1290_q_d_n16];
        let eq8_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            &nodes,
            &eq8_reactive_node_derivatives,
            &branches,
            &eq8_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
