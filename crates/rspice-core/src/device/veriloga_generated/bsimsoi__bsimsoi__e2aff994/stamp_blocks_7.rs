#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_26_block_0(
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
        let (eq26_e1814, eq26_e1814_d_n0, eq26_e1814_d_n1, eq26_e1814_d_n2, eq26_e1814_d_n3, eq26_e1814_d_n4, eq26_e1814_d_n5, eq26_e1814_d_n6, eq26_e1814_d_n7, eq26_e1814_d_n8, eq26_e1814_d_n9, eq26_e1814_d_n10, eq26_e1814_d_n11, eq26_e1814_d_n12, eq26_e1814_d_n13,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        let eq26_e1812: f64 = (s.v[628] * (nv13 - 0.0));
        let eq26_e1812_d_n0: f64 = (s.dn[628][0] * (nv13 - 0.0));
        let eq26_e1812_d_n1: f64 = (s.dn[628][1] * (nv13 - 0.0));
        let eq26_e1812_d_n2: f64 = (s.dn[628][2] * (nv13 - 0.0));
        let eq26_e1812_d_n3: f64 = (s.dn[628][3] * (nv13 - 0.0));
        let eq26_e1812_d_n4: f64 = (s.dn[628][4] * (nv13 - 0.0));
        let eq26_e1812_d_n5: f64 = (s.dn[628][5] * (nv13 - 0.0));
        let eq26_e1812_d_n6: f64 = (s.dn[628][6] * (nv13 - 0.0));
        let eq26_e1812_d_n7: f64 = (s.dn[628][7] * (nv13 - 0.0));
        let eq26_e1812_d_n8: f64 = (s.dn[628][8] * (nv13 - 0.0));
        let eq26_e1812_d_n9: f64 = (s.dn[628][9] * (nv13 - 0.0));
        let eq26_e1812_d_n10: f64 = (s.dn[628][10] * (nv13 - 0.0));
        let eq26_e1812_d_n11: f64 = (s.dn[628][11] * (nv13 - 0.0));
        let eq26_e1812_d_n12: f64 = (s.dn[628][12] * (nv13 - 0.0));
        let eq26_e1812_d_n13: f64 = ((s.dn[628][13] * (nv13 - 0.0)) + s.v[628]);
        (eq26_e1812, eq26_e1812_d_n0, eq26_e1812_d_n1, eq26_e1812_d_n2, eq26_e1812_d_n3, eq26_e1812_d_n4, eq26_e1812_d_n5, eq26_e1812_d_n6, eq26_e1812_d_n7, eq26_e1812_d_n8, eq26_e1812_d_n9, eq26_e1812_d_n10, eq26_e1812_d_n11, eq26_e1812_d_n12, eq26_e1812_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1814;
        let eq26_node_derivatives: [f64; 14] = [eq26_e1814_d_n0, eq26_e1814_d_n1, eq26_e1814_d_n2, eq26_e1814_d_n3, eq26_e1814_d_n4, eq26_e1814_d_n5, eq26_e1814_d_n6, eq26_e1814_d_n7, eq26_e1814_d_n8, eq26_e1814_d_n9, eq26_e1814_d_n10, eq26_e1814_d_n11, eq26_e1814_d_n12, eq26_e1814_d_n13];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq26_value),
            &nodes,
            &eq26_node_derivatives,
            &branches,
            &eq26_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_27_block_0(
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
        let (eq27_e1841, eq27_e1841_d_n0, eq27_e1841_d_n1, eq27_e1841_d_n2, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_d_n13,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        let eq27_e1825: f64 = (1.0 + s.v[211]);
        let eq27_e1827: f64 = (eq27_e1825 * s.v[622]);
        let eq27_e1827_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq27_e1825 * s.dn[622][0]));
        let eq27_e1827_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq27_e1825 * s.dn[622][1]));
        let eq27_e1827_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq27_e1825 * s.dn[622][2]));
        let eq27_e1827_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq27_e1825 * s.dn[622][3]));
        let eq27_e1827_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq27_e1825 * s.dn[622][4]));
        let eq27_e1827_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq27_e1825 * s.dn[622][5]));
        let eq27_e1827_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq27_e1825 * s.dn[622][6]));
        let eq27_e1827_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq27_e1825 * s.dn[622][7]));
        let eq27_e1827_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq27_e1825 * s.dn[622][8]));
        let eq27_e1827_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq27_e1825 * s.dn[622][9]));
        let eq27_e1827_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq27_e1825 * s.dn[622][10]));
        let eq27_e1827_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq27_e1825 * s.dn[622][11]));
        let eq27_e1827_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq27_e1825 * s.dn[622][12]));
        let eq27_e1827_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq27_e1825 * s.dn[622][13]));
        let eq27_e1829: f64 = (eq27_e1827 * s.v[199]);
        let eq27_e1829_d_n0: f64 = (eq27_e1827_d_n0 * s.v[199]);
        let eq27_e1829_d_n1: f64 = (eq27_e1827_d_n1 * s.v[199]);
        let eq27_e1829_d_n2: f64 = (eq27_e1827_d_n2 * s.v[199]);
        let eq27_e1829_d_n3: f64 = (eq27_e1827_d_n3 * s.v[199]);
        let eq27_e1829_d_n4: f64 = (eq27_e1827_d_n4 * s.v[199]);
        let eq27_e1829_d_n5: f64 = (eq27_e1827_d_n5 * s.v[199]);
        let eq27_e1829_d_n6: f64 = (eq27_e1827_d_n6 * s.v[199]);
        let eq27_e1829_d_n7: f64 = (eq27_e1827_d_n7 * s.v[199]);
        let eq27_e1829_d_n8: f64 = (eq27_e1827_d_n8 * s.v[199]);
        let eq27_e1829_d_n9: f64 = (eq27_e1827_d_n9 * s.v[199]);
        let eq27_e1829_d_n10: f64 = (eq27_e1827_d_n10 * s.v[199]);
        let eq27_e1829_d_n11: f64 = (eq27_e1827_d_n11 * s.v[199]);
        let eq27_e1829_d_n12: f64 = (eq27_e1827_d_n12 * s.v[199]);
        let eq27_e1829_d_n13: f64 = (eq27_e1827_d_n13 * s.v[199]);
        let eq27_e1831: f64 = (eq27_e1829 * s.v[183]);
        let eq27_e1831_d_n0: f64 = (eq27_e1829_d_n0 * s.v[183]);
        let eq27_e1831_d_n1: f64 = (eq27_e1829_d_n1 * s.v[183]);
        let eq27_e1831_d_n2: f64 = (eq27_e1829_d_n2 * s.v[183]);
        let eq27_e1831_d_n3: f64 = (eq27_e1829_d_n3 * s.v[183]);
        let eq27_e1831_d_n4: f64 = (eq27_e1829_d_n4 * s.v[183]);
        let eq27_e1831_d_n5: f64 = (eq27_e1829_d_n5 * s.v[183]);
        let eq27_e1831_d_n6: f64 = (eq27_e1829_d_n6 * s.v[183]);
        let eq27_e1831_d_n7: f64 = (eq27_e1829_d_n7 * s.v[183]);
        let eq27_e1831_d_n8: f64 = (eq27_e1829_d_n8 * s.v[183]);
        let eq27_e1831_d_n9: f64 = (eq27_e1829_d_n9 * s.v[183]);
        let eq27_e1831_d_n10: f64 = (eq27_e1829_d_n10 * s.v[183]);
        let eq27_e1831_d_n11: f64 = (eq27_e1829_d_n11 * s.v[183]);
        let eq27_e1831_d_n12: f64 = (eq27_e1829_d_n12 * s.v[183]);
        let eq27_e1831_d_n13: f64 = (eq27_e1829_d_n13 * s.v[183]);
        let eq27_e1833: f64 = (eq27_e1831 * p.p2);
        let eq27_e1833_d_n0: f64 = (eq27_e1831_d_n0 * p.p2);
        let eq27_e1833_d_n1: f64 = (eq27_e1831_d_n1 * p.p2);
        let eq27_e1833_d_n2: f64 = (eq27_e1831_d_n2 * p.p2);
        let eq27_e1833_d_n3: f64 = (eq27_e1831_d_n3 * p.p2);
        let eq27_e1833_d_n4: f64 = (eq27_e1831_d_n4 * p.p2);
        let eq27_e1833_d_n5: f64 = (eq27_e1831_d_n5 * p.p2);
        let eq27_e1833_d_n6: f64 = (eq27_e1831_d_n6 * p.p2);
        let eq27_e1833_d_n7: f64 = (eq27_e1831_d_n7 * p.p2);
        let eq27_e1833_d_n8: f64 = (eq27_e1831_d_n8 * p.p2);
        let eq27_e1833_d_n9: f64 = (eq27_e1831_d_n9 * p.p2);
        let eq27_e1833_d_n10: f64 = (eq27_e1831_d_n10 * p.p2);
        let eq27_e1833_d_n11: f64 = (eq27_e1831_d_n11 * p.p2);
        let eq27_e1833_d_n12: f64 = (eq27_e1831_d_n12 * p.p2);
        let eq27_e1833_d_n13: f64 = (eq27_e1831_d_n13 * p.p2);
        let eq27_e1835: f64 = (eq27_e1833 * s.v[184]);
        let eq27_e1835_d_n0: f64 = (eq27_e1833_d_n0 * s.v[184]);
        let eq27_e1835_d_n1: f64 = (eq27_e1833_d_n1 * s.v[184]);
        let eq27_e1835_d_n2: f64 = (eq27_e1833_d_n2 * s.v[184]);
        let eq27_e1835_d_n3: f64 = (eq27_e1833_d_n3 * s.v[184]);
        let eq27_e1835_d_n4: f64 = (eq27_e1833_d_n4 * s.v[184]);
        let eq27_e1835_d_n5: f64 = (eq27_e1833_d_n5 * s.v[184]);
        let eq27_e1835_d_n6: f64 = (eq27_e1833_d_n6 * s.v[184]);
        let eq27_e1835_d_n7: f64 = (eq27_e1833_d_n7 * s.v[184]);
        let eq27_e1835_d_n8: f64 = (eq27_e1833_d_n8 * s.v[184]);
        let eq27_e1835_d_n9: f64 = (eq27_e1833_d_n9 * s.v[184]);
        let eq27_e1835_d_n10: f64 = (eq27_e1833_d_n10 * s.v[184]);
        let eq27_e1835_d_n11: f64 = (eq27_e1833_d_n11 * s.v[184]);
        let eq27_e1835_d_n12: f64 = (eq27_e1833_d_n12 * s.v[184]);
        let eq27_e1835_d_n13: f64 = (eq27_e1833_d_n13 * s.v[184]);
        let eq27_e1837: f64 = (eq27_e1835 * (nv12 - 0.0));
        let eq27_e1837_d_n0: f64 = (eq27_e1835_d_n0 * (nv12 - 0.0));
        let eq27_e1837_d_n1: f64 = (eq27_e1835_d_n1 * (nv12 - 0.0));
        let eq27_e1837_d_n2: f64 = (eq27_e1835_d_n2 * (nv12 - 0.0));
        let eq27_e1837_d_n3: f64 = (eq27_e1835_d_n3 * (nv12 - 0.0));
        let eq27_e1837_d_n4: f64 = (eq27_e1835_d_n4 * (nv12 - 0.0));
        let eq27_e1837_d_n5: f64 = (eq27_e1835_d_n5 * (nv12 - 0.0));
        let eq27_e1837_d_n6: f64 = (eq27_e1835_d_n6 * (nv12 - 0.0));
        let eq27_e1837_d_n7: f64 = (eq27_e1835_d_n7 * (nv12 - 0.0));
        let eq27_e1837_d_n8: f64 = (eq27_e1835_d_n8 * (nv12 - 0.0));
        let eq27_e1837_d_n9: f64 = (eq27_e1835_d_n9 * (nv12 - 0.0));
        let eq27_e1837_d_n10: f64 = (eq27_e1835_d_n10 * (nv12 - 0.0));
        let eq27_e1837_d_n11: f64 = (eq27_e1835_d_n11 * (nv12 - 0.0));
        let eq27_e1837_d_n12: f64 = ((eq27_e1835_d_n12 * (nv12 - 0.0)) + eq27_e1835);
        let eq27_e1837_d_n13: f64 = (eq27_e1835_d_n13 * (nv12 - 0.0));
        let eq27_e1838: f64 = (0.5 * eq27_e1837);
        let eq27_e1838_d_n0: f64 = (0.5 * eq27_e1837_d_n0);
        let eq27_e1838_d_n1: f64 = (0.5 * eq27_e1837_d_n1);
        let eq27_e1838_d_n2: f64 = (0.5 * eq27_e1837_d_n2);
        let eq27_e1838_d_n3: f64 = (0.5 * eq27_e1837_d_n3);
        let eq27_e1838_d_n4: f64 = (0.5 * eq27_e1837_d_n4);
        let eq27_e1838_d_n5: f64 = (0.5 * eq27_e1837_d_n5);
        let eq27_e1838_d_n6: f64 = (0.5 * eq27_e1837_d_n6);
        let eq27_e1838_d_n7: f64 = (0.5 * eq27_e1837_d_n7);
        let eq27_e1838_d_n8: f64 = (0.5 * eq27_e1837_d_n8);
        let eq27_e1838_d_n9: f64 = (0.5 * eq27_e1837_d_n9);
        let eq27_e1838_d_n10: f64 = (0.5 * eq27_e1837_d_n10);
        let eq27_e1838_d_n11: f64 = (0.5 * eq27_e1837_d_n11);
        let eq27_e1838_d_n12: f64 = (0.5 * eq27_e1837_d_n12);
        let eq27_e1838_d_n13: f64 = (0.5 * eq27_e1837_d_n13);
        let eq27_e1839: f64 = self.eval_ddt(4, eq27_e1838);
        let eq27_e1839_d_n0: f64 = self.ddt_jacobian(eq27_e1838_d_n0);
        let eq27_e1839_d_n1: f64 = self.ddt_jacobian(eq27_e1838_d_n1);
        let eq27_e1839_d_n2: f64 = self.ddt_jacobian(eq27_e1838_d_n2);
        let eq27_e1839_d_n3: f64 = self.ddt_jacobian(eq27_e1838_d_n3);
        let eq27_e1839_d_n4: f64 = self.ddt_jacobian(eq27_e1838_d_n4);
        let eq27_e1839_d_n5: f64 = self.ddt_jacobian(eq27_e1838_d_n5);
        let eq27_e1839_d_n6: f64 = self.ddt_jacobian(eq27_e1838_d_n6);
        let eq27_e1839_d_n7: f64 = self.ddt_jacobian(eq27_e1838_d_n7);
        let eq27_e1839_d_n8: f64 = self.ddt_jacobian(eq27_e1838_d_n8);
        let eq27_e1839_d_n9: f64 = self.ddt_jacobian(eq27_e1838_d_n9);
        let eq27_e1839_d_n10: f64 = self.ddt_jacobian(eq27_e1838_d_n10);
        let eq27_e1839_d_n11: f64 = self.ddt_jacobian(eq27_e1838_d_n11);
        let eq27_e1839_d_n12: f64 = self.ddt_jacobian(eq27_e1838_d_n12);
        let eq27_e1839_d_n13: f64 = self.ddt_jacobian(eq27_e1838_d_n13);
        (eq27_e1839, eq27_e1839_d_n0, eq27_e1839_d_n1, eq27_e1839_d_n2, eq27_e1839_d_n3, eq27_e1839_d_n4, eq27_e1839_d_n5, eq27_e1839_d_n6, eq27_e1839_d_n7, eq27_e1839_d_n8, eq27_e1839_d_n9, eq27_e1839_d_n10, eq27_e1839_d_n11, eq27_e1839_d_n12, eq27_e1839_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1841;
        let eq27_node_derivatives: [f64; 14] = [eq27_e1841_d_n0, eq27_e1841_d_n1, eq27_e1841_d_n2, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_d_n13];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq27_value),
            &nodes,
            &eq27_node_derivatives,
            &branches,
            &eq27_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq28_e1868, eq28_e1868_d_n0, eq28_e1868_d_n1, eq28_e1868_d_n2, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_d_n13,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        let eq28_e1852: f64 = (1.0 - s.v[211]);
        let eq28_e1852_d_n0: f64 = (-s.dn[211][0]);
        let eq28_e1852_d_n1: f64 = (-s.dn[211][1]);
        let eq28_e1852_d_n2: f64 = (-s.dn[211][2]);
        let eq28_e1852_d_n3: f64 = (-s.dn[211][3]);
        let eq28_e1852_d_n4: f64 = (-s.dn[211][4]);
        let eq28_e1852_d_n5: f64 = (-s.dn[211][5]);
        let eq28_e1852_d_n6: f64 = (-s.dn[211][6]);
        let eq28_e1852_d_n7: f64 = (-s.dn[211][7]);
        let eq28_e1852_d_n8: f64 = (-s.dn[211][8]);
        let eq28_e1852_d_n9: f64 = (-s.dn[211][9]);
        let eq28_e1852_d_n10: f64 = (-s.dn[211][10]);
        let eq28_e1852_d_n11: f64 = (-s.dn[211][11]);
        let eq28_e1852_d_n12: f64 = (-s.dn[211][12]);
        let eq28_e1852_d_n13: f64 = (-s.dn[211][13]);
        let eq28_e1854: f64 = (eq28_e1852 * s.v[622]);
        let eq28_e1854_d_n0: f64 = ((eq28_e1852_d_n0 * s.v[622]) + (eq28_e1852 * s.dn[622][0]));
        let eq28_e1854_d_n1: f64 = ((eq28_e1852_d_n1 * s.v[622]) + (eq28_e1852 * s.dn[622][1]));
        let eq28_e1854_d_n2: f64 = ((eq28_e1852_d_n2 * s.v[622]) + (eq28_e1852 * s.dn[622][2]));
        let eq28_e1854_d_n3: f64 = ((eq28_e1852_d_n3 * s.v[622]) + (eq28_e1852 * s.dn[622][3]));
        let eq28_e1854_d_n4: f64 = ((eq28_e1852_d_n4 * s.v[622]) + (eq28_e1852 * s.dn[622][4]));
        let eq28_e1854_d_n5: f64 = ((eq28_e1852_d_n5 * s.v[622]) + (eq28_e1852 * s.dn[622][5]));
        let eq28_e1854_d_n6: f64 = ((eq28_e1852_d_n6 * s.v[622]) + (eq28_e1852 * s.dn[622][6]));
        let eq28_e1854_d_n7: f64 = ((eq28_e1852_d_n7 * s.v[622]) + (eq28_e1852 * s.dn[622][7]));
        let eq28_e1854_d_n8: f64 = ((eq28_e1852_d_n8 * s.v[622]) + (eq28_e1852 * s.dn[622][8]));
        let eq28_e1854_d_n9: f64 = ((eq28_e1852_d_n9 * s.v[622]) + (eq28_e1852 * s.dn[622][9]));
        let eq28_e1854_d_n10: f64 = ((eq28_e1852_d_n10 * s.v[622]) + (eq28_e1852 * s.dn[622][10]));
        let eq28_e1854_d_n11: f64 = ((eq28_e1852_d_n11 * s.v[622]) + (eq28_e1852 * s.dn[622][11]));
        let eq28_e1854_d_n12: f64 = ((eq28_e1852_d_n12 * s.v[622]) + (eq28_e1852 * s.dn[622][12]));
        let eq28_e1854_d_n13: f64 = ((eq28_e1852_d_n13 * s.v[622]) + (eq28_e1852 * s.dn[622][13]));
        let eq28_e1856: f64 = (eq28_e1854 * s.v[199]);
        let eq28_e1856_d_n0: f64 = (eq28_e1854_d_n0 * s.v[199]);
        let eq28_e1856_d_n1: f64 = (eq28_e1854_d_n1 * s.v[199]);
        let eq28_e1856_d_n2: f64 = (eq28_e1854_d_n2 * s.v[199]);
        let eq28_e1856_d_n3: f64 = (eq28_e1854_d_n3 * s.v[199]);
        let eq28_e1856_d_n4: f64 = (eq28_e1854_d_n4 * s.v[199]);
        let eq28_e1856_d_n5: f64 = (eq28_e1854_d_n5 * s.v[199]);
        let eq28_e1856_d_n6: f64 = (eq28_e1854_d_n6 * s.v[199]);
        let eq28_e1856_d_n7: f64 = (eq28_e1854_d_n7 * s.v[199]);
        let eq28_e1856_d_n8: f64 = (eq28_e1854_d_n8 * s.v[199]);
        let eq28_e1856_d_n9: f64 = (eq28_e1854_d_n9 * s.v[199]);
        let eq28_e1856_d_n10: f64 = (eq28_e1854_d_n10 * s.v[199]);
        let eq28_e1856_d_n11: f64 = (eq28_e1854_d_n11 * s.v[199]);
        let eq28_e1856_d_n12: f64 = (eq28_e1854_d_n12 * s.v[199]);
        let eq28_e1856_d_n13: f64 = (eq28_e1854_d_n13 * s.v[199]);
        let eq28_e1858: f64 = (eq28_e1856 * s.v[183]);
        let eq28_e1858_d_n0: f64 = (eq28_e1856_d_n0 * s.v[183]);
        let eq28_e1858_d_n1: f64 = (eq28_e1856_d_n1 * s.v[183]);
        let eq28_e1858_d_n2: f64 = (eq28_e1856_d_n2 * s.v[183]);
        let eq28_e1858_d_n3: f64 = (eq28_e1856_d_n3 * s.v[183]);
        let eq28_e1858_d_n4: f64 = (eq28_e1856_d_n4 * s.v[183]);
        let eq28_e1858_d_n5: f64 = (eq28_e1856_d_n5 * s.v[183]);
        let eq28_e1858_d_n6: f64 = (eq28_e1856_d_n6 * s.v[183]);
        let eq28_e1858_d_n7: f64 = (eq28_e1856_d_n7 * s.v[183]);
        let eq28_e1858_d_n8: f64 = (eq28_e1856_d_n8 * s.v[183]);
        let eq28_e1858_d_n9: f64 = (eq28_e1856_d_n9 * s.v[183]);
        let eq28_e1858_d_n10: f64 = (eq28_e1856_d_n10 * s.v[183]);
        let eq28_e1858_d_n11: f64 = (eq28_e1856_d_n11 * s.v[183]);
        let eq28_e1858_d_n12: f64 = (eq28_e1856_d_n12 * s.v[183]);
        let eq28_e1858_d_n13: f64 = (eq28_e1856_d_n13 * s.v[183]);
        let eq28_e1860: f64 = (eq28_e1858 * p.p2);
        let eq28_e1860_d_n0: f64 = (eq28_e1858_d_n0 * p.p2);
        let eq28_e1860_d_n1: f64 = (eq28_e1858_d_n1 * p.p2);
        let eq28_e1860_d_n2: f64 = (eq28_e1858_d_n2 * p.p2);
        let eq28_e1860_d_n3: f64 = (eq28_e1858_d_n3 * p.p2);
        let eq28_e1860_d_n4: f64 = (eq28_e1858_d_n4 * p.p2);
        let eq28_e1860_d_n5: f64 = (eq28_e1858_d_n5 * p.p2);
        let eq28_e1860_d_n6: f64 = (eq28_e1858_d_n6 * p.p2);
        let eq28_e1860_d_n7: f64 = (eq28_e1858_d_n7 * p.p2);
        let eq28_e1860_d_n8: f64 = (eq28_e1858_d_n8 * p.p2);
        let eq28_e1860_d_n9: f64 = (eq28_e1858_d_n9 * p.p2);
        let eq28_e1860_d_n10: f64 = (eq28_e1858_d_n10 * p.p2);
        let eq28_e1860_d_n11: f64 = (eq28_e1858_d_n11 * p.p2);
        let eq28_e1860_d_n12: f64 = (eq28_e1858_d_n12 * p.p2);
        let eq28_e1860_d_n13: f64 = (eq28_e1858_d_n13 * p.p2);
        let eq28_e1862: f64 = (eq28_e1860 * s.v[184]);
        let eq28_e1862_d_n0: f64 = (eq28_e1860_d_n0 * s.v[184]);
        let eq28_e1862_d_n1: f64 = (eq28_e1860_d_n1 * s.v[184]);
        let eq28_e1862_d_n2: f64 = (eq28_e1860_d_n2 * s.v[184]);
        let eq28_e1862_d_n3: f64 = (eq28_e1860_d_n3 * s.v[184]);
        let eq28_e1862_d_n4: f64 = (eq28_e1860_d_n4 * s.v[184]);
        let eq28_e1862_d_n5: f64 = (eq28_e1860_d_n5 * s.v[184]);
        let eq28_e1862_d_n6: f64 = (eq28_e1860_d_n6 * s.v[184]);
        let eq28_e1862_d_n7: f64 = (eq28_e1860_d_n7 * s.v[184]);
        let eq28_e1862_d_n8: f64 = (eq28_e1860_d_n8 * s.v[184]);
        let eq28_e1862_d_n9: f64 = (eq28_e1860_d_n9 * s.v[184]);
        let eq28_e1862_d_n10: f64 = (eq28_e1860_d_n10 * s.v[184]);
        let eq28_e1862_d_n11: f64 = (eq28_e1860_d_n11 * s.v[184]);
        let eq28_e1862_d_n12: f64 = (eq28_e1860_d_n12 * s.v[184]);
        let eq28_e1862_d_n13: f64 = (eq28_e1860_d_n13 * s.v[184]);
        let eq28_e1864: f64 = (eq28_e1862 * (nv12 - 0.0));
        let eq28_e1864_d_n0: f64 = (eq28_e1862_d_n0 * (nv12 - 0.0));
        let eq28_e1864_d_n1: f64 = (eq28_e1862_d_n1 * (nv12 - 0.0));
        let eq28_e1864_d_n2: f64 = (eq28_e1862_d_n2 * (nv12 - 0.0));
        let eq28_e1864_d_n3: f64 = (eq28_e1862_d_n3 * (nv12 - 0.0));
        let eq28_e1864_d_n4: f64 = (eq28_e1862_d_n4 * (nv12 - 0.0));
        let eq28_e1864_d_n5: f64 = (eq28_e1862_d_n5 * (nv12 - 0.0));
        let eq28_e1864_d_n6: f64 = (eq28_e1862_d_n6 * (nv12 - 0.0));
        let eq28_e1864_d_n7: f64 = (eq28_e1862_d_n7 * (nv12 - 0.0));
        let eq28_e1864_d_n8: f64 = (eq28_e1862_d_n8 * (nv12 - 0.0));
        let eq28_e1864_d_n9: f64 = (eq28_e1862_d_n9 * (nv12 - 0.0));
        let eq28_e1864_d_n10: f64 = (eq28_e1862_d_n10 * (nv12 - 0.0));
        let eq28_e1864_d_n11: f64 = (eq28_e1862_d_n11 * (nv12 - 0.0));
        let eq28_e1864_d_n12: f64 = ((eq28_e1862_d_n12 * (nv12 - 0.0)) + eq28_e1862);
        let eq28_e1864_d_n13: f64 = (eq28_e1862_d_n13 * (nv12 - 0.0));
        let eq28_e1865: f64 = (0.5 * eq28_e1864);
        let eq28_e1865_d_n0: f64 = (0.5 * eq28_e1864_d_n0);
        let eq28_e1865_d_n1: f64 = (0.5 * eq28_e1864_d_n1);
        let eq28_e1865_d_n2: f64 = (0.5 * eq28_e1864_d_n2);
        let eq28_e1865_d_n3: f64 = (0.5 * eq28_e1864_d_n3);
        let eq28_e1865_d_n4: f64 = (0.5 * eq28_e1864_d_n4);
        let eq28_e1865_d_n5: f64 = (0.5 * eq28_e1864_d_n5);
        let eq28_e1865_d_n6: f64 = (0.5 * eq28_e1864_d_n6);
        let eq28_e1865_d_n7: f64 = (0.5 * eq28_e1864_d_n7);
        let eq28_e1865_d_n8: f64 = (0.5 * eq28_e1864_d_n8);
        let eq28_e1865_d_n9: f64 = (0.5 * eq28_e1864_d_n9);
        let eq28_e1865_d_n10: f64 = (0.5 * eq28_e1864_d_n10);
        let eq28_e1865_d_n11: f64 = (0.5 * eq28_e1864_d_n11);
        let eq28_e1865_d_n12: f64 = (0.5 * eq28_e1864_d_n12);
        let eq28_e1865_d_n13: f64 = (0.5 * eq28_e1864_d_n13);
        let eq28_e1866: f64 = self.eval_ddt(5, eq28_e1865);
        let eq28_e1866_d_n0: f64 = self.ddt_jacobian(eq28_e1865_d_n0);
        let eq28_e1866_d_n1: f64 = self.ddt_jacobian(eq28_e1865_d_n1);
        let eq28_e1866_d_n2: f64 = self.ddt_jacobian(eq28_e1865_d_n2);
        let eq28_e1866_d_n3: f64 = self.ddt_jacobian(eq28_e1865_d_n3);
        let eq28_e1866_d_n4: f64 = self.ddt_jacobian(eq28_e1865_d_n4);
        let eq28_e1866_d_n5: f64 = self.ddt_jacobian(eq28_e1865_d_n5);
        let eq28_e1866_d_n6: f64 = self.ddt_jacobian(eq28_e1865_d_n6);
        let eq28_e1866_d_n7: f64 = self.ddt_jacobian(eq28_e1865_d_n7);
        let eq28_e1866_d_n8: f64 = self.ddt_jacobian(eq28_e1865_d_n8);
        let eq28_e1866_d_n9: f64 = self.ddt_jacobian(eq28_e1865_d_n9);
        let eq28_e1866_d_n10: f64 = self.ddt_jacobian(eq28_e1865_d_n10);
        let eq28_e1866_d_n11: f64 = self.ddt_jacobian(eq28_e1865_d_n11);
        let eq28_e1866_d_n12: f64 = self.ddt_jacobian(eq28_e1865_d_n12);
        let eq28_e1866_d_n13: f64 = self.ddt_jacobian(eq28_e1865_d_n13);
        (eq28_e1866, eq28_e1866_d_n0, eq28_e1866_d_n1, eq28_e1866_d_n2, eq28_e1866_d_n3, eq28_e1866_d_n4, eq28_e1866_d_n5, eq28_e1866_d_n6, eq28_e1866_d_n7, eq28_e1866_d_n8, eq28_e1866_d_n9, eq28_e1866_d_n10, eq28_e1866_d_n11, eq28_e1866_d_n12, eq28_e1866_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e1868;
        let eq28_node_derivatives: [f64; 14] = [eq28_e1868_d_n0, eq28_e1868_d_n1, eq28_e1868_d_n2, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_d_n13];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq28_value),
            &nodes,
            &eq28_node_derivatives,
            &branches,
            &eq28_branch_derivatives,
            self.multiplicity,
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq29_e1873, eq29_e1873_d_n13,) = {
    if (!(s.v[1620] != 0.0)) {
        ((nv13 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e1873;
        stamper.stamp_current(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq29_value),
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * eq29_e1873_d_n13),
            ],
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq30_e1878, eq30_e1878_d_n12,) = {
    if (!(s.v[1620] != 0.0)) {
        ((nv12 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1878;
        stamper.stamp_current(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq30_value),
            &[
                GeneratedDerivative::node(nodes[12], self.multiplicity * eq30_e1878_d_n12),
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
        let (eq31_e1894,) = {
    if ((!(s.v[1620] != 0.0)) && (s.v[1968] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e1894;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq31_value),
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
        let (eq32_e1910,) = {
    if ((!(s.v[1620] != 0.0)) && (s.v[1968] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1910;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq32_value),
            &[
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
        let (eq33_e1924,) = {
    if ((!(s.v[1620] != 0.0)) && (s.v[1969] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq33_value: f64 = eq33_e1924;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[10]),
            self.multiplicity * (eq33_value),
            &[
            ],
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
        let (eq34_e1936,) = {
    if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e1936;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq34_value),
            &[
            ],
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
        let eq35_e1938: f64 = self.eval_ddt(6, s.v[1057]);
        let eq35_e1938_d_n0: f64 = self.ddt_jacobian(s.dn[1057][0]);
        let eq35_e1938_d_n1: f64 = self.ddt_jacobian(s.dn[1057][1]);
        let eq35_e1938_d_n2: f64 = self.ddt_jacobian(s.dn[1057][2]);
        let eq35_e1938_d_n3: f64 = self.ddt_jacobian(s.dn[1057][3]);
        let eq35_e1938_d_n4: f64 = self.ddt_jacobian(s.dn[1057][4]);
        let eq35_e1938_d_n5: f64 = self.ddt_jacobian(s.dn[1057][5]);
        let eq35_e1938_d_n6: f64 = self.ddt_jacobian(s.dn[1057][6]);
        let eq35_e1938_d_n7: f64 = self.ddt_jacobian(s.dn[1057][7]);
        let eq35_e1938_d_n8: f64 = self.ddt_jacobian(s.dn[1057][8]);
        let eq35_e1938_d_n9: f64 = self.ddt_jacobian(s.dn[1057][9]);
        let eq35_e1938_d_n10: f64 = self.ddt_jacobian(s.dn[1057][10]);
        let eq35_e1938_d_n11: f64 = self.ddt_jacobian(s.dn[1057][11]);
        let eq35_e1938_d_n12: f64 = self.ddt_jacobian(s.dn[1057][12]);
        let eq35_e1938_d_n13: f64 = self.ddt_jacobian(s.dn[1057][13]);
        let eq35_value: f64 = eq35_e1938;
        let eq35_node_derivatives: [f64; 14] = [eq35_e1938_d_n0, eq35_e1938_d_n1, eq35_e1938_d_n2, eq35_e1938_d_n3, eq35_e1938_d_n4, eq35_e1938_d_n5, eq35_e1938_d_n6, eq35_e1938_d_n7, eq35_e1938_d_n8, eq35_e1938_d_n9, eq35_e1938_d_n10, eq35_e1938_d_n11, eq35_e1938_d_n12, eq35_e1938_d_n13];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[10]),
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
        let eq36_e1940: f64 = self.eval_ddt(7, s.v[1058]);
        let eq36_e1940_d_n0: f64 = self.ddt_jacobian(s.dn[1058][0]);
        let eq36_e1940_d_n1: f64 = self.ddt_jacobian(s.dn[1058][1]);
        let eq36_e1940_d_n2: f64 = self.ddt_jacobian(s.dn[1058][2]);
        let eq36_e1940_d_n3: f64 = self.ddt_jacobian(s.dn[1058][3]);
        let eq36_e1940_d_n4: f64 = self.ddt_jacobian(s.dn[1058][4]);
        let eq36_e1940_d_n5: f64 = self.ddt_jacobian(s.dn[1058][5]);
        let eq36_e1940_d_n6: f64 = self.ddt_jacobian(s.dn[1058][6]);
        let eq36_e1940_d_n7: f64 = self.ddt_jacobian(s.dn[1058][7]);
        let eq36_e1940_d_n8: f64 = self.ddt_jacobian(s.dn[1058][8]);
        let eq36_e1940_d_n9: f64 = self.ddt_jacobian(s.dn[1058][9]);
        let eq36_e1940_d_n10: f64 = self.ddt_jacobian(s.dn[1058][10]);
        let eq36_e1940_d_n11: f64 = self.ddt_jacobian(s.dn[1058][11]);
        let eq36_e1940_d_n12: f64 = self.ddt_jacobian(s.dn[1058][12]);
        let eq36_e1940_d_n13: f64 = self.ddt_jacobian(s.dn[1058][13]);
        let eq36_value: f64 = eq36_e1940;
        let eq36_node_derivatives: [f64; 14] = [eq36_e1940_d_n0, eq36_e1940_d_n1, eq36_e1940_d_n2, eq36_e1940_d_n3, eq36_e1940_d_n4, eq36_e1940_d_n5, eq36_e1940_d_n6, eq36_e1940_d_n7, eq36_e1940_d_n8, eq36_e1940_d_n9, eq36_e1940_d_n10, eq36_e1940_d_n11, eq36_e1940_d_n12, eq36_e1940_d_n13];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[11]),
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
        let eq37_e1942: f64 = self.eval_ddt(8, s.v[1051]);
        let eq37_e1942_d_n0: f64 = self.ddt_jacobian(s.dn[1051][0]);
        let eq37_e1942_d_n1: f64 = self.ddt_jacobian(s.dn[1051][1]);
        let eq37_e1942_d_n2: f64 = self.ddt_jacobian(s.dn[1051][2]);
        let eq37_e1942_d_n3: f64 = self.ddt_jacobian(s.dn[1051][3]);
        let eq37_e1942_d_n4: f64 = self.ddt_jacobian(s.dn[1051][4]);
        let eq37_e1942_d_n5: f64 = self.ddt_jacobian(s.dn[1051][5]);
        let eq37_e1942_d_n6: f64 = self.ddt_jacobian(s.dn[1051][6]);
        let eq37_e1942_d_n7: f64 = self.ddt_jacobian(s.dn[1051][7]);
        let eq37_e1942_d_n8: f64 = self.ddt_jacobian(s.dn[1051][8]);
        let eq37_e1942_d_n9: f64 = self.ddt_jacobian(s.dn[1051][9]);
        let eq37_e1942_d_n10: f64 = self.ddt_jacobian(s.dn[1051][10]);
        let eq37_e1942_d_n11: f64 = self.ddt_jacobian(s.dn[1051][11]);
        let eq37_e1942_d_n12: f64 = self.ddt_jacobian(s.dn[1051][12]);
        let eq37_e1942_d_n13: f64 = self.ddt_jacobian(s.dn[1051][13]);
        let eq37_value: f64 = eq37_e1942;
        let eq37_node_derivatives: [f64; 14] = [eq37_e1942_d_n0, eq37_e1942_d_n1, eq37_e1942_d_n2, eq37_e1942_d_n3, eq37_e1942_d_n4, eq37_e1942_d_n5, eq37_e1942_d_n6, eq37_e1942_d_n7, eq37_e1942_d_n8, eq37_e1942_d_n9, eq37_e1942_d_n10, eq37_e1942_d_n11, eq37_e1942_d_n12, eq37_e1942_d_n13];
        let eq37_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[10]),
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
        let eq38_e1944: f64 = self.eval_ddt(9, s.v[1052]);
        let eq38_e1944_d_n0: f64 = self.ddt_jacobian(s.dn[1052][0]);
        let eq38_e1944_d_n1: f64 = self.ddt_jacobian(s.dn[1052][1]);
        let eq38_e1944_d_n2: f64 = self.ddt_jacobian(s.dn[1052][2]);
        let eq38_e1944_d_n3: f64 = self.ddt_jacobian(s.dn[1052][3]);
        let eq38_e1944_d_n4: f64 = self.ddt_jacobian(s.dn[1052][4]);
        let eq38_e1944_d_n5: f64 = self.ddt_jacobian(s.dn[1052][5]);
        let eq38_e1944_d_n6: f64 = self.ddt_jacobian(s.dn[1052][6]);
        let eq38_e1944_d_n7: f64 = self.ddt_jacobian(s.dn[1052][7]);
        let eq38_e1944_d_n8: f64 = self.ddt_jacobian(s.dn[1052][8]);
        let eq38_e1944_d_n9: f64 = self.ddt_jacobian(s.dn[1052][9]);
        let eq38_e1944_d_n10: f64 = self.ddt_jacobian(s.dn[1052][10]);
        let eq38_e1944_d_n11: f64 = self.ddt_jacobian(s.dn[1052][11]);
        let eq38_e1944_d_n12: f64 = self.ddt_jacobian(s.dn[1052][12]);
        let eq38_e1944_d_n13: f64 = self.ddt_jacobian(s.dn[1052][13]);
        let eq38_value: f64 = eq38_e1944;
        let eq38_node_derivatives: [f64; 14] = [eq38_e1944_d_n0, eq38_e1944_d_n1, eq38_e1944_d_n2, eq38_e1944_d_n3, eq38_e1944_d_n4, eq38_e1944_d_n5, eq38_e1944_d_n6, eq38_e1944_d_n7, eq38_e1944_d_n8, eq38_e1944_d_n9, eq38_e1944_d_n10, eq38_e1944_d_n11, eq38_e1944_d_n12, eq38_e1944_d_n13];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
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
        let eq39_e1946: f64 = self.eval_ddt(10, s.v[1054]);
        let eq39_e1946_d_n0: f64 = self.ddt_jacobian(s.dn[1054][0]);
        let eq39_e1946_d_n1: f64 = self.ddt_jacobian(s.dn[1054][1]);
        let eq39_e1946_d_n2: f64 = self.ddt_jacobian(s.dn[1054][2]);
        let eq39_e1946_d_n3: f64 = self.ddt_jacobian(s.dn[1054][3]);
        let eq39_e1946_d_n4: f64 = self.ddt_jacobian(s.dn[1054][4]);
        let eq39_e1946_d_n5: f64 = self.ddt_jacobian(s.dn[1054][5]);
        let eq39_e1946_d_n6: f64 = self.ddt_jacobian(s.dn[1054][6]);
        let eq39_e1946_d_n7: f64 = self.ddt_jacobian(s.dn[1054][7]);
        let eq39_e1946_d_n8: f64 = self.ddt_jacobian(s.dn[1054][8]);
        let eq39_e1946_d_n9: f64 = self.ddt_jacobian(s.dn[1054][9]);
        let eq39_e1946_d_n10: f64 = self.ddt_jacobian(s.dn[1054][10]);
        let eq39_e1946_d_n11: f64 = self.ddt_jacobian(s.dn[1054][11]);
        let eq39_e1946_d_n12: f64 = self.ddt_jacobian(s.dn[1054][12]);
        let eq39_e1946_d_n13: f64 = self.ddt_jacobian(s.dn[1054][13]);
        let eq39_value: f64 = eq39_e1946;
        let eq39_node_derivatives: [f64; 14] = [eq39_e1946_d_n0, eq39_e1946_d_n1, eq39_e1946_d_n2, eq39_e1946_d_n3, eq39_e1946_d_n4, eq39_e1946_d_n5, eq39_e1946_d_n6, eq39_e1946_d_n7, eq39_e1946_d_n8, eq39_e1946_d_n9, eq39_e1946_d_n10, eq39_e1946_d_n11, eq39_e1946_d_n12, eq39_e1946_d_n13];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[10]),
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
        let eq40_e1948: f64 = self.eval_ddt(11, s.v[1055]);
        let eq40_e1948_d_n0: f64 = self.ddt_jacobian(s.dn[1055][0]);
        let eq40_e1948_d_n1: f64 = self.ddt_jacobian(s.dn[1055][1]);
        let eq40_e1948_d_n2: f64 = self.ddt_jacobian(s.dn[1055][2]);
        let eq40_e1948_d_n3: f64 = self.ddt_jacobian(s.dn[1055][3]);
        let eq40_e1948_d_n4: f64 = self.ddt_jacobian(s.dn[1055][4]);
        let eq40_e1948_d_n5: f64 = self.ddt_jacobian(s.dn[1055][5]);
        let eq40_e1948_d_n6: f64 = self.ddt_jacobian(s.dn[1055][6]);
        let eq40_e1948_d_n7: f64 = self.ddt_jacobian(s.dn[1055][7]);
        let eq40_e1948_d_n8: f64 = self.ddt_jacobian(s.dn[1055][8]);
        let eq40_e1948_d_n9: f64 = self.ddt_jacobian(s.dn[1055][9]);
        let eq40_e1948_d_n10: f64 = self.ddt_jacobian(s.dn[1055][10]);
        let eq40_e1948_d_n11: f64 = self.ddt_jacobian(s.dn[1055][11]);
        let eq40_e1948_d_n12: f64 = self.ddt_jacobian(s.dn[1055][12]);
        let eq40_e1948_d_n13: f64 = self.ddt_jacobian(s.dn[1055][13]);
        let eq40_value: f64 = eq40_e1948;
        let eq40_node_derivatives: [f64; 14] = [eq40_e1948_d_n0, eq40_e1948_d_n1, eq40_e1948_d_n2, eq40_e1948_d_n3, eq40_e1948_d_n4, eq40_e1948_d_n5, eq40_e1948_d_n6, eq40_e1948_d_n7, eq40_e1948_d_n8, eq40_e1948_d_n9, eq40_e1948_d_n10, eq40_e1948_d_n11, eq40_e1948_d_n12, eq40_e1948_d_n13];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[11]),
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
        let eq41_e1950: f64 = (-s.v[379]);
        let eq41_e1950_d_n0: f64 = (-s.dn[379][0]);
        let eq41_e1950_d_n1: f64 = (-s.dn[379][1]);
        let eq41_e1950_d_n2: f64 = (-s.dn[379][2]);
        let eq41_e1950_d_n3: f64 = (-s.dn[379][3]);
        let eq41_e1950_d_n4: f64 = (-s.dn[379][4]);
        let eq41_e1950_d_n5: f64 = (-s.dn[379][5]);
        let eq41_e1950_d_n6: f64 = (-s.dn[379][6]);
        let eq41_e1950_d_n7: f64 = (-s.dn[379][7]);
        let eq41_e1950_d_n8: f64 = (-s.dn[379][8]);
        let eq41_e1950_d_n9: f64 = (-s.dn[379][9]);
        let eq41_e1950_d_n10: f64 = (-s.dn[379][10]);
        let eq41_e1950_d_n11: f64 = (-s.dn[379][11]);
        let eq41_e1950_d_n12: f64 = (-s.dn[379][12]);
        let eq41_e1950_d_n13: f64 = (-s.dn[379][13]);
        let eq41_e1952: f64 = (eq41_e1950 * s.v[423]);
        let eq41_e1952_d_n0: f64 = ((eq41_e1950_d_n0 * s.v[423]) + (eq41_e1950 * s.dn[423][0]));
        let eq41_e1952_d_n1: f64 = ((eq41_e1950_d_n1 * s.v[423]) + (eq41_e1950 * s.dn[423][1]));
        let eq41_e1952_d_n2: f64 = ((eq41_e1950_d_n2 * s.v[423]) + (eq41_e1950 * s.dn[423][2]));
        let eq41_e1952_d_n3: f64 = ((eq41_e1950_d_n3 * s.v[423]) + (eq41_e1950 * s.dn[423][3]));
        let eq41_e1952_d_n4: f64 = ((eq41_e1950_d_n4 * s.v[423]) + (eq41_e1950 * s.dn[423][4]));
        let eq41_e1952_d_n5: f64 = ((eq41_e1950_d_n5 * s.v[423]) + (eq41_e1950 * s.dn[423][5]));
        let eq41_e1952_d_n6: f64 = ((eq41_e1950_d_n6 * s.v[423]) + (eq41_e1950 * s.dn[423][6]));
        let eq41_e1952_d_n7: f64 = ((eq41_e1950_d_n7 * s.v[423]) + (eq41_e1950 * s.dn[423][7]));
        let eq41_e1952_d_n8: f64 = ((eq41_e1950_d_n8 * s.v[423]) + (eq41_e1950 * s.dn[423][8]));
        let eq41_e1952_d_n9: f64 = ((eq41_e1950_d_n9 * s.v[423]) + (eq41_e1950 * s.dn[423][9]));
        let eq41_e1952_d_n10: f64 = ((eq41_e1950_d_n10 * s.v[423]) + (eq41_e1950 * s.dn[423][10]));
        let eq41_e1952_d_n11: f64 = ((eq41_e1950_d_n11 * s.v[423]) + (eq41_e1950 * s.dn[423][11]));
        let eq41_e1952_d_n12: f64 = ((eq41_e1950_d_n12 * s.v[423]) + (eq41_e1950 * s.dn[423][12]));
        let eq41_e1952_d_n13: f64 = ((eq41_e1950_d_n13 * s.v[423]) + (eq41_e1950 * s.dn[423][13]));
        let eq41_e1953: f64 = self.eval_ddt(12, eq41_e1952);
        let eq41_e1953_d_n0: f64 = self.ddt_jacobian(eq41_e1952_d_n0);
        let eq41_e1953_d_n1: f64 = self.ddt_jacobian(eq41_e1952_d_n1);
        let eq41_e1953_d_n2: f64 = self.ddt_jacobian(eq41_e1952_d_n2);
        let eq41_e1953_d_n3: f64 = self.ddt_jacobian(eq41_e1952_d_n3);
        let eq41_e1953_d_n4: f64 = self.ddt_jacobian(eq41_e1952_d_n4);
        let eq41_e1953_d_n5: f64 = self.ddt_jacobian(eq41_e1952_d_n5);
        let eq41_e1953_d_n6: f64 = self.ddt_jacobian(eq41_e1952_d_n6);
        let eq41_e1953_d_n7: f64 = self.ddt_jacobian(eq41_e1952_d_n7);
        let eq41_e1953_d_n8: f64 = self.ddt_jacobian(eq41_e1952_d_n8);
        let eq41_e1953_d_n9: f64 = self.ddt_jacobian(eq41_e1952_d_n9);
        let eq41_e1953_d_n10: f64 = self.ddt_jacobian(eq41_e1952_d_n10);
        let eq41_e1953_d_n11: f64 = self.ddt_jacobian(eq41_e1952_d_n11);
        let eq41_e1953_d_n12: f64 = self.ddt_jacobian(eq41_e1952_d_n12);
        let eq41_e1953_d_n13: f64 = self.ddt_jacobian(eq41_e1952_d_n13);
        let eq41_value: f64 = eq41_e1953;
        let eq41_node_derivatives: [f64; 14] = [eq41_e1953_d_n0, eq41_e1953_d_n1, eq41_e1953_d_n2, eq41_e1953_d_n3, eq41_e1953_d_n4, eq41_e1953_d_n5, eq41_e1953_d_n6, eq41_e1953_d_n7, eq41_e1953_d_n8, eq41_e1953_d_n9, eq41_e1953_d_n10, eq41_e1953_d_n11, eq41_e1953_d_n12, eq41_e1953_d_n13];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq41_value),
            &nodes,
            &eq41_node_derivatives,
            &branches,
            &eq41_branch_derivatives,
            self.multiplicity,
        );
    }
}
