#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq27_e1841, eq27_e1841_d_n0, eq27_e1841_d_n1, eq27_e1841_d_n2, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_d_n13, eq27_e1841_d_b0, eq27_e1841_d_b1, eq27_e1841_d_b2, eq27_e1841_d_b3, eq27_e1841_d_b4, eq27_e1841_d_b5, eq27_e1841_d_b6, eq27_e1841_d_b7, eq27_e1841_d_b8, eq27_e1841_d_b9, eq27_e1841_d_b10, eq27_e1841_d_b11, eq27_e1841_q,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
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
        let eq27_e1827_d_b0: f64 = ((s.db[211][0] * s.v[622]) + (eq27_e1825 * s.db[622][0]));
        let eq27_e1827_d_b1: f64 = ((s.db[211][1] * s.v[622]) + (eq27_e1825 * s.db[622][1]));
        let eq27_e1827_d_b2: f64 = ((s.db[211][2] * s.v[622]) + (eq27_e1825 * s.db[622][2]));
        let eq27_e1827_d_b3: f64 = ((s.db[211][3] * s.v[622]) + (eq27_e1825 * s.db[622][3]));
        let eq27_e1827_d_b4: f64 = ((s.db[211][4] * s.v[622]) + (eq27_e1825 * s.db[622][4]));
        let eq27_e1827_d_b5: f64 = ((s.db[211][5] * s.v[622]) + (eq27_e1825 * s.db[622][5]));
        let eq27_e1827_d_b6: f64 = ((s.db[211][6] * s.v[622]) + (eq27_e1825 * s.db[622][6]));
        let eq27_e1827_d_b7: f64 = ((s.db[211][7] * s.v[622]) + (eq27_e1825 * s.db[622][7]));
        let eq27_e1827_d_b8: f64 = ((s.db[211][8] * s.v[622]) + (eq27_e1825 * s.db[622][8]));
        let eq27_e1827_d_b9: f64 = ((s.db[211][9] * s.v[622]) + (eq27_e1825 * s.db[622][9]));
        let eq27_e1827_d_b10: f64 = ((s.db[211][10] * s.v[622]) + (eq27_e1825 * s.db[622][10]));
        let eq27_e1827_d_b11: f64 = ((s.db[211][11] * s.v[622]) + (eq27_e1825 * s.db[622][11]));
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
        let eq27_e1829_d_b0: f64 = (eq27_e1827_d_b0 * s.v[199]);
        let eq27_e1829_d_b1: f64 = (eq27_e1827_d_b1 * s.v[199]);
        let eq27_e1829_d_b2: f64 = (eq27_e1827_d_b2 * s.v[199]);
        let eq27_e1829_d_b3: f64 = (eq27_e1827_d_b3 * s.v[199]);
        let eq27_e1829_d_b4: f64 = (eq27_e1827_d_b4 * s.v[199]);
        let eq27_e1829_d_b5: f64 = (eq27_e1827_d_b5 * s.v[199]);
        let eq27_e1829_d_b6: f64 = (eq27_e1827_d_b6 * s.v[199]);
        let eq27_e1829_d_b7: f64 = (eq27_e1827_d_b7 * s.v[199]);
        let eq27_e1829_d_b8: f64 = (eq27_e1827_d_b8 * s.v[199]);
        let eq27_e1829_d_b9: f64 = (eq27_e1827_d_b9 * s.v[199]);
        let eq27_e1829_d_b10: f64 = (eq27_e1827_d_b10 * s.v[199]);
        let eq27_e1829_d_b11: f64 = (eq27_e1827_d_b11 * s.v[199]);
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
        let eq27_e1831_d_b0: f64 = (eq27_e1829_d_b0 * s.v[183]);
        let eq27_e1831_d_b1: f64 = (eq27_e1829_d_b1 * s.v[183]);
        let eq27_e1831_d_b2: f64 = (eq27_e1829_d_b2 * s.v[183]);
        let eq27_e1831_d_b3: f64 = (eq27_e1829_d_b3 * s.v[183]);
        let eq27_e1831_d_b4: f64 = (eq27_e1829_d_b4 * s.v[183]);
        let eq27_e1831_d_b5: f64 = (eq27_e1829_d_b5 * s.v[183]);
        let eq27_e1831_d_b6: f64 = (eq27_e1829_d_b6 * s.v[183]);
        let eq27_e1831_d_b7: f64 = (eq27_e1829_d_b7 * s.v[183]);
        let eq27_e1831_d_b8: f64 = (eq27_e1829_d_b8 * s.v[183]);
        let eq27_e1831_d_b9: f64 = (eq27_e1829_d_b9 * s.v[183]);
        let eq27_e1831_d_b10: f64 = (eq27_e1829_d_b10 * s.v[183]);
        let eq27_e1831_d_b11: f64 = (eq27_e1829_d_b11 * s.v[183]);
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
        let eq27_e1833_d_b0: f64 = (eq27_e1831_d_b0 * p.p2);
        let eq27_e1833_d_b1: f64 = (eq27_e1831_d_b1 * p.p2);
        let eq27_e1833_d_b2: f64 = (eq27_e1831_d_b2 * p.p2);
        let eq27_e1833_d_b3: f64 = (eq27_e1831_d_b3 * p.p2);
        let eq27_e1833_d_b4: f64 = (eq27_e1831_d_b4 * p.p2);
        let eq27_e1833_d_b5: f64 = (eq27_e1831_d_b5 * p.p2);
        let eq27_e1833_d_b6: f64 = (eq27_e1831_d_b6 * p.p2);
        let eq27_e1833_d_b7: f64 = (eq27_e1831_d_b7 * p.p2);
        let eq27_e1833_d_b8: f64 = (eq27_e1831_d_b8 * p.p2);
        let eq27_e1833_d_b9: f64 = (eq27_e1831_d_b9 * p.p2);
        let eq27_e1833_d_b10: f64 = (eq27_e1831_d_b10 * p.p2);
        let eq27_e1833_d_b11: f64 = (eq27_e1831_d_b11 * p.p2);
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
        let eq27_e1835_d_b0: f64 = (eq27_e1833_d_b0 * s.v[184]);
        let eq27_e1835_d_b1: f64 = (eq27_e1833_d_b1 * s.v[184]);
        let eq27_e1835_d_b2: f64 = (eq27_e1833_d_b2 * s.v[184]);
        let eq27_e1835_d_b3: f64 = (eq27_e1833_d_b3 * s.v[184]);
        let eq27_e1835_d_b4: f64 = (eq27_e1833_d_b4 * s.v[184]);
        let eq27_e1835_d_b5: f64 = (eq27_e1833_d_b5 * s.v[184]);
        let eq27_e1835_d_b6: f64 = (eq27_e1833_d_b6 * s.v[184]);
        let eq27_e1835_d_b7: f64 = (eq27_e1833_d_b7 * s.v[184]);
        let eq27_e1835_d_b8: f64 = (eq27_e1833_d_b8 * s.v[184]);
        let eq27_e1835_d_b9: f64 = (eq27_e1833_d_b9 * s.v[184]);
        let eq27_e1835_d_b10: f64 = (eq27_e1833_d_b10 * s.v[184]);
        let eq27_e1835_d_b11: f64 = (eq27_e1833_d_b11 * s.v[184]);
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
        let eq27_e1837_d_b0: f64 = (eq27_e1835_d_b0 * (nv12 - 0.0));
        let eq27_e1837_d_b1: f64 = (eq27_e1835_d_b1 * (nv12 - 0.0));
        let eq27_e1837_d_b2: f64 = (eq27_e1835_d_b2 * (nv12 - 0.0));
        let eq27_e1837_d_b3: f64 = (eq27_e1835_d_b3 * (nv12 - 0.0));
        let eq27_e1837_d_b4: f64 = (eq27_e1835_d_b4 * (nv12 - 0.0));
        let eq27_e1837_d_b5: f64 = (eq27_e1835_d_b5 * (nv12 - 0.0));
        let eq27_e1837_d_b6: f64 = (eq27_e1835_d_b6 * (nv12 - 0.0));
        let eq27_e1837_d_b7: f64 = (eq27_e1835_d_b7 * (nv12 - 0.0));
        let eq27_e1837_d_b8: f64 = (eq27_e1835_d_b8 * (nv12 - 0.0));
        let eq27_e1837_d_b9: f64 = (eq27_e1835_d_b9 * (nv12 - 0.0));
        let eq27_e1837_d_b10: f64 = (eq27_e1835_d_b10 * (nv12 - 0.0));
        let eq27_e1837_d_b11: f64 = (eq27_e1835_d_b11 * (nv12 - 0.0));
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
        let eq27_e1838_d_b0: f64 = (0.5 * eq27_e1837_d_b0);
        let eq27_e1838_d_b1: f64 = (0.5 * eq27_e1837_d_b1);
        let eq27_e1838_d_b2: f64 = (0.5 * eq27_e1837_d_b2);
        let eq27_e1838_d_b3: f64 = (0.5 * eq27_e1837_d_b3);
        let eq27_e1838_d_b4: f64 = (0.5 * eq27_e1837_d_b4);
        let eq27_e1838_d_b5: f64 = (0.5 * eq27_e1837_d_b5);
        let eq27_e1838_d_b6: f64 = (0.5 * eq27_e1837_d_b6);
        let eq27_e1838_d_b7: f64 = (0.5 * eq27_e1837_d_b7);
        let eq27_e1838_d_b8: f64 = (0.5 * eq27_e1837_d_b8);
        let eq27_e1838_d_b9: f64 = (0.5 * eq27_e1837_d_b9);
        let eq27_e1838_d_b10: f64 = (0.5 * eq27_e1837_d_b10);
        let eq27_e1838_d_b11: f64 = (0.5 * eq27_e1837_d_b11);
        let eq27_e1839_q: f64 = eq27_e1838;
        (eq27_e1838, eq27_e1838_d_n0, eq27_e1838_d_n1, eq27_e1838_d_n2, eq27_e1838_d_n3, eq27_e1838_d_n4, eq27_e1838_d_n5, eq27_e1838_d_n6, eq27_e1838_d_n7, eq27_e1838_d_n8, eq27_e1838_d_n9, eq27_e1838_d_n10, eq27_e1838_d_n11, eq27_e1838_d_n12, eq27_e1838_d_n13, eq27_e1838_d_b0, eq27_e1838_d_b1, eq27_e1838_d_b2, eq27_e1838_d_b3, eq27_e1838_d_b4, eq27_e1838_d_b5, eq27_e1838_d_b6, eq27_e1838_d_b7, eq27_e1838_d_b8, eq27_e1838_d_b9, eq27_e1838_d_b10, eq27_e1838_d_b11, eq27_e1839_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_reactive_node_derivatives: [f64; 14] = [eq27_e1841_d_n0, eq27_e1841_d_n1, eq27_e1841_d_n2, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_d_n13];
        let eq27_reactive_branch_derivatives: [f64; 12] = [eq27_e1841_d_b0, eq27_e1841_d_b1, eq27_e1841_d_b2, eq27_e1841_d_b3, eq27_e1841_d_b4, eq27_e1841_d_b5, eq27_e1841_d_b6, eq27_e1841_d_b7, eq27_e1841_d_b8, eq27_e1841_d_b9, eq27_e1841_d_b10, eq27_e1841_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1868, eq28_e1868_d_n0, eq28_e1868_d_n1, eq28_e1868_d_n2, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_d_n13, eq28_e1868_d_b0, eq28_e1868_d_b1, eq28_e1868_d_b2, eq28_e1868_d_b3, eq28_e1868_d_b4, eq28_e1868_d_b5, eq28_e1868_d_b6, eq28_e1868_d_b7, eq28_e1868_d_b8, eq28_e1868_d_b9, eq28_e1868_d_b10, eq28_e1868_d_b11, eq28_e1868_q,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq28_e1852: f64 = (1.0 - s.v[211]);
        let eq28_e1854: f64 = (eq28_e1852 * s.v[622]);
        let eq28_e1854_d_n0: f64 = (((-s.dn[211][0]) * s.v[622]) + (eq28_e1852 * s.dn[622][0]));
        let eq28_e1854_d_n1: f64 = (((-s.dn[211][1]) * s.v[622]) + (eq28_e1852 * s.dn[622][1]));
        let eq28_e1854_d_n2: f64 = (((-s.dn[211][2]) * s.v[622]) + (eq28_e1852 * s.dn[622][2]));
        let eq28_e1854_d_n3: f64 = (((-s.dn[211][3]) * s.v[622]) + (eq28_e1852 * s.dn[622][3]));
        let eq28_e1854_d_n4: f64 = (((-s.dn[211][4]) * s.v[622]) + (eq28_e1852 * s.dn[622][4]));
        let eq28_e1854_d_n5: f64 = (((-s.dn[211][5]) * s.v[622]) + (eq28_e1852 * s.dn[622][5]));
        let eq28_e1854_d_n6: f64 = (((-s.dn[211][6]) * s.v[622]) + (eq28_e1852 * s.dn[622][6]));
        let eq28_e1854_d_n7: f64 = (((-s.dn[211][7]) * s.v[622]) + (eq28_e1852 * s.dn[622][7]));
        let eq28_e1854_d_n8: f64 = (((-s.dn[211][8]) * s.v[622]) + (eq28_e1852 * s.dn[622][8]));
        let eq28_e1854_d_n9: f64 = (((-s.dn[211][9]) * s.v[622]) + (eq28_e1852 * s.dn[622][9]));
        let eq28_e1854_d_n10: f64 = (((-s.dn[211][10]) * s.v[622]) + (eq28_e1852 * s.dn[622][10]));
        let eq28_e1854_d_n11: f64 = (((-s.dn[211][11]) * s.v[622]) + (eq28_e1852 * s.dn[622][11]));
        let eq28_e1854_d_n12: f64 = (((-s.dn[211][12]) * s.v[622]) + (eq28_e1852 * s.dn[622][12]));
        let eq28_e1854_d_n13: f64 = (((-s.dn[211][13]) * s.v[622]) + (eq28_e1852 * s.dn[622][13]));
        let eq28_e1854_d_b0: f64 = (((-s.db[211][0]) * s.v[622]) + (eq28_e1852 * s.db[622][0]));
        let eq28_e1854_d_b1: f64 = (((-s.db[211][1]) * s.v[622]) + (eq28_e1852 * s.db[622][1]));
        let eq28_e1854_d_b2: f64 = (((-s.db[211][2]) * s.v[622]) + (eq28_e1852 * s.db[622][2]));
        let eq28_e1854_d_b3: f64 = (((-s.db[211][3]) * s.v[622]) + (eq28_e1852 * s.db[622][3]));
        let eq28_e1854_d_b4: f64 = (((-s.db[211][4]) * s.v[622]) + (eq28_e1852 * s.db[622][4]));
        let eq28_e1854_d_b5: f64 = (((-s.db[211][5]) * s.v[622]) + (eq28_e1852 * s.db[622][5]));
        let eq28_e1854_d_b6: f64 = (((-s.db[211][6]) * s.v[622]) + (eq28_e1852 * s.db[622][6]));
        let eq28_e1854_d_b7: f64 = (((-s.db[211][7]) * s.v[622]) + (eq28_e1852 * s.db[622][7]));
        let eq28_e1854_d_b8: f64 = (((-s.db[211][8]) * s.v[622]) + (eq28_e1852 * s.db[622][8]));
        let eq28_e1854_d_b9: f64 = (((-s.db[211][9]) * s.v[622]) + (eq28_e1852 * s.db[622][9]));
        let eq28_e1854_d_b10: f64 = (((-s.db[211][10]) * s.v[622]) + (eq28_e1852 * s.db[622][10]));
        let eq28_e1854_d_b11: f64 = (((-s.db[211][11]) * s.v[622]) + (eq28_e1852 * s.db[622][11]));
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
        let eq28_e1856_d_b0: f64 = (eq28_e1854_d_b0 * s.v[199]);
        let eq28_e1856_d_b1: f64 = (eq28_e1854_d_b1 * s.v[199]);
        let eq28_e1856_d_b2: f64 = (eq28_e1854_d_b2 * s.v[199]);
        let eq28_e1856_d_b3: f64 = (eq28_e1854_d_b3 * s.v[199]);
        let eq28_e1856_d_b4: f64 = (eq28_e1854_d_b4 * s.v[199]);
        let eq28_e1856_d_b5: f64 = (eq28_e1854_d_b5 * s.v[199]);
        let eq28_e1856_d_b6: f64 = (eq28_e1854_d_b6 * s.v[199]);
        let eq28_e1856_d_b7: f64 = (eq28_e1854_d_b7 * s.v[199]);
        let eq28_e1856_d_b8: f64 = (eq28_e1854_d_b8 * s.v[199]);
        let eq28_e1856_d_b9: f64 = (eq28_e1854_d_b9 * s.v[199]);
        let eq28_e1856_d_b10: f64 = (eq28_e1854_d_b10 * s.v[199]);
        let eq28_e1856_d_b11: f64 = (eq28_e1854_d_b11 * s.v[199]);
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
        let eq28_e1858_d_b0: f64 = (eq28_e1856_d_b0 * s.v[183]);
        let eq28_e1858_d_b1: f64 = (eq28_e1856_d_b1 * s.v[183]);
        let eq28_e1858_d_b2: f64 = (eq28_e1856_d_b2 * s.v[183]);
        let eq28_e1858_d_b3: f64 = (eq28_e1856_d_b3 * s.v[183]);
        let eq28_e1858_d_b4: f64 = (eq28_e1856_d_b4 * s.v[183]);
        let eq28_e1858_d_b5: f64 = (eq28_e1856_d_b5 * s.v[183]);
        let eq28_e1858_d_b6: f64 = (eq28_e1856_d_b6 * s.v[183]);
        let eq28_e1858_d_b7: f64 = (eq28_e1856_d_b7 * s.v[183]);
        let eq28_e1858_d_b8: f64 = (eq28_e1856_d_b8 * s.v[183]);
        let eq28_e1858_d_b9: f64 = (eq28_e1856_d_b9 * s.v[183]);
        let eq28_e1858_d_b10: f64 = (eq28_e1856_d_b10 * s.v[183]);
        let eq28_e1858_d_b11: f64 = (eq28_e1856_d_b11 * s.v[183]);
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
        let eq28_e1860_d_b0: f64 = (eq28_e1858_d_b0 * p.p2);
        let eq28_e1860_d_b1: f64 = (eq28_e1858_d_b1 * p.p2);
        let eq28_e1860_d_b2: f64 = (eq28_e1858_d_b2 * p.p2);
        let eq28_e1860_d_b3: f64 = (eq28_e1858_d_b3 * p.p2);
        let eq28_e1860_d_b4: f64 = (eq28_e1858_d_b4 * p.p2);
        let eq28_e1860_d_b5: f64 = (eq28_e1858_d_b5 * p.p2);
        let eq28_e1860_d_b6: f64 = (eq28_e1858_d_b6 * p.p2);
        let eq28_e1860_d_b7: f64 = (eq28_e1858_d_b7 * p.p2);
        let eq28_e1860_d_b8: f64 = (eq28_e1858_d_b8 * p.p2);
        let eq28_e1860_d_b9: f64 = (eq28_e1858_d_b9 * p.p2);
        let eq28_e1860_d_b10: f64 = (eq28_e1858_d_b10 * p.p2);
        let eq28_e1860_d_b11: f64 = (eq28_e1858_d_b11 * p.p2);
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
        let eq28_e1862_d_b0: f64 = (eq28_e1860_d_b0 * s.v[184]);
        let eq28_e1862_d_b1: f64 = (eq28_e1860_d_b1 * s.v[184]);
        let eq28_e1862_d_b2: f64 = (eq28_e1860_d_b2 * s.v[184]);
        let eq28_e1862_d_b3: f64 = (eq28_e1860_d_b3 * s.v[184]);
        let eq28_e1862_d_b4: f64 = (eq28_e1860_d_b4 * s.v[184]);
        let eq28_e1862_d_b5: f64 = (eq28_e1860_d_b5 * s.v[184]);
        let eq28_e1862_d_b6: f64 = (eq28_e1860_d_b6 * s.v[184]);
        let eq28_e1862_d_b7: f64 = (eq28_e1860_d_b7 * s.v[184]);
        let eq28_e1862_d_b8: f64 = (eq28_e1860_d_b8 * s.v[184]);
        let eq28_e1862_d_b9: f64 = (eq28_e1860_d_b9 * s.v[184]);
        let eq28_e1862_d_b10: f64 = (eq28_e1860_d_b10 * s.v[184]);
        let eq28_e1862_d_b11: f64 = (eq28_e1860_d_b11 * s.v[184]);
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
        let eq28_e1864_d_b0: f64 = (eq28_e1862_d_b0 * (nv12 - 0.0));
        let eq28_e1864_d_b1: f64 = (eq28_e1862_d_b1 * (nv12 - 0.0));
        let eq28_e1864_d_b2: f64 = (eq28_e1862_d_b2 * (nv12 - 0.0));
        let eq28_e1864_d_b3: f64 = (eq28_e1862_d_b3 * (nv12 - 0.0));
        let eq28_e1864_d_b4: f64 = (eq28_e1862_d_b4 * (nv12 - 0.0));
        let eq28_e1864_d_b5: f64 = (eq28_e1862_d_b5 * (nv12 - 0.0));
        let eq28_e1864_d_b6: f64 = (eq28_e1862_d_b6 * (nv12 - 0.0));
        let eq28_e1864_d_b7: f64 = (eq28_e1862_d_b7 * (nv12 - 0.0));
        let eq28_e1864_d_b8: f64 = (eq28_e1862_d_b8 * (nv12 - 0.0));
        let eq28_e1864_d_b9: f64 = (eq28_e1862_d_b9 * (nv12 - 0.0));
        let eq28_e1864_d_b10: f64 = (eq28_e1862_d_b10 * (nv12 - 0.0));
        let eq28_e1864_d_b11: f64 = (eq28_e1862_d_b11 * (nv12 - 0.0));
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
        let eq28_e1865_d_b0: f64 = (0.5 * eq28_e1864_d_b0);
        let eq28_e1865_d_b1: f64 = (0.5 * eq28_e1864_d_b1);
        let eq28_e1865_d_b2: f64 = (0.5 * eq28_e1864_d_b2);
        let eq28_e1865_d_b3: f64 = (0.5 * eq28_e1864_d_b3);
        let eq28_e1865_d_b4: f64 = (0.5 * eq28_e1864_d_b4);
        let eq28_e1865_d_b5: f64 = (0.5 * eq28_e1864_d_b5);
        let eq28_e1865_d_b6: f64 = (0.5 * eq28_e1864_d_b6);
        let eq28_e1865_d_b7: f64 = (0.5 * eq28_e1864_d_b7);
        let eq28_e1865_d_b8: f64 = (0.5 * eq28_e1864_d_b8);
        let eq28_e1865_d_b9: f64 = (0.5 * eq28_e1864_d_b9);
        let eq28_e1865_d_b10: f64 = (0.5 * eq28_e1864_d_b10);
        let eq28_e1865_d_b11: f64 = (0.5 * eq28_e1864_d_b11);
        let eq28_e1866_q: f64 = eq28_e1865;
        (eq28_e1865, eq28_e1865_d_n0, eq28_e1865_d_n1, eq28_e1865_d_n2, eq28_e1865_d_n3, eq28_e1865_d_n4, eq28_e1865_d_n5, eq28_e1865_d_n6, eq28_e1865_d_n7, eq28_e1865_d_n8, eq28_e1865_d_n9, eq28_e1865_d_n10, eq28_e1865_d_n11, eq28_e1865_d_n12, eq28_e1865_d_n13, eq28_e1865_d_b0, eq28_e1865_d_b1, eq28_e1865_d_b2, eq28_e1865_d_b3, eq28_e1865_d_b4, eq28_e1865_d_b5, eq28_e1865_d_b6, eq28_e1865_d_b7, eq28_e1865_d_b8, eq28_e1865_d_b9, eq28_e1865_d_b10, eq28_e1865_d_b11, eq28_e1866_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 14] = [eq28_e1868_d_n0, eq28_e1868_d_n1, eq28_e1868_d_n2, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_d_n13];
        let eq28_reactive_branch_derivatives: [f64; 12] = [eq28_e1868_d_b0, eq28_e1868_d_b1, eq28_e1868_d_b2, eq28_e1868_d_b3, eq28_e1868_d_b4, eq28_e1868_d_b5, eq28_e1868_d_b6, eq28_e1868_d_b7, eq28_e1868_d_b8, eq28_e1868_d_b9, eq28_e1868_d_b10, eq28_e1868_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e1938_q: f64 = s.v[1057];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[10]),
            nodes,
            &s.dn[1057],
            branches,
            &s.db[1057],
            multiplicity,
        );
        let eq36_e1940_q: f64 = s.v[1058];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[11]),
            nodes,
            &s.dn[1058],
            branches,
            &s.db[1058],
            multiplicity,
        );
        let eq37_e1942_q: f64 = s.v[1051];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &s.dn[1051],
            branches,
            &s.db[1051],
            multiplicity,
        );
        let eq38_e1944_q: f64 = s.v[1052];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &s.dn[1052],
            branches,
            &s.db[1052],
            multiplicity,
        );
        let eq39_e1946_q: f64 = s.v[1054];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[10]),
            nodes,
            &s.dn[1054],
            branches,
            &s.db[1054],
            multiplicity,
        );
        let eq40_e1948_q: f64 = s.v[1055];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            nodes,
            &s.dn[1055],
            branches,
            &s.db[1055],
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let __rspice_deriv_cse_13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let __rspice_deriv_cse_14: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));
        let __rspice_deriv_cse_15: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));
        let __rspice_deriv_cse_16: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));
        let __rspice_deriv_cse_17: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));
        let __rspice_deriv_cse_18: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));
        let __rspice_deriv_cse_19: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));
        let __rspice_deriv_cse_20: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));
        let __rspice_deriv_cse_21: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));
        let __rspice_deriv_cse_22: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));
        let __rspice_deriv_cse_23: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));
        let __rspice_deriv_cse_24: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));
        let __rspice_deriv_cse_25: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));
        let __rspice_deriv_cse_26: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let __rspice_deriv_cse_27: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let __rspice_deriv_cse_28: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let __rspice_deriv_cse_29: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let __rspice_deriv_cse_30: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let __rspice_deriv_cse_31: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let __rspice_deriv_cse_32: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let __rspice_deriv_cse_33: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let __rspice_deriv_cse_34: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let __rspice_deriv_cse_35: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let __rspice_deriv_cse_36: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let __rspice_deriv_cse_37: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let __rspice_deriv_cse_38: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let __rspice_deriv_cse_39: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let __rspice_deriv_cse_40: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));
        let __rspice_deriv_cse_41: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));
        let __rspice_deriv_cse_42: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));
        let __rspice_deriv_cse_43: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));
        let __rspice_deriv_cse_44: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));
        let __rspice_deriv_cse_45: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let __rspice_deriv_cse_46: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));
        let __rspice_deriv_cse_47: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));
        let __rspice_deriv_cse_48: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));
        let __rspice_deriv_cse_49: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));
        let __rspice_deriv_cse_50: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));
        let __rspice_deriv_cse_51: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));
        let __rspice_deriv_cse_52: f64 = (__rspice_deriv_cse_0 + __rspice_deriv_cse_26);
        let __rspice_deriv_cse_53: f64 = (__rspice_deriv_cse_1 + __rspice_deriv_cse_27);
        let __rspice_deriv_cse_54: f64 = (__rspice_deriv_cse_2 + __rspice_deriv_cse_28);
        let __rspice_deriv_cse_55: f64 = (__rspice_deriv_cse_3 + __rspice_deriv_cse_29);
        let __rspice_deriv_cse_56: f64 = (__rspice_deriv_cse_4 + __rspice_deriv_cse_30);
        let __rspice_deriv_cse_57: f64 = (__rspice_deriv_cse_5 + __rspice_deriv_cse_31);
        let __rspice_deriv_cse_58: f64 = (__rspice_deriv_cse_6 + __rspice_deriv_cse_32);
        let __rspice_deriv_cse_59: f64 = (__rspice_deriv_cse_7 + __rspice_deriv_cse_33);
        let __rspice_deriv_cse_60: f64 = (__rspice_deriv_cse_8 + __rspice_deriv_cse_34);
        let __rspice_deriv_cse_61: f64 = (__rspice_deriv_cse_9 + __rspice_deriv_cse_35);
        let __rspice_deriv_cse_62: f64 = (__rspice_deriv_cse_10 + __rspice_deriv_cse_36);
        let __rspice_deriv_cse_63: f64 = (__rspice_deriv_cse_11 + __rspice_deriv_cse_37);
        let __rspice_deriv_cse_64: f64 = (__rspice_deriv_cse_12 + __rspice_deriv_cse_38);
        let __rspice_deriv_cse_65: f64 = (__rspice_deriv_cse_13 + __rspice_deriv_cse_39);
        let __rspice_deriv_cse_66: f64 = (__rspice_deriv_cse_14 + __rspice_deriv_cse_40);
        let __rspice_deriv_cse_67: f64 = (__rspice_deriv_cse_15 + __rspice_deriv_cse_41);
        let __rspice_deriv_cse_68: f64 = (__rspice_deriv_cse_16 + __rspice_deriv_cse_42);
        let __rspice_deriv_cse_69: f64 = (__rspice_deriv_cse_17 + __rspice_deriv_cse_43);
        let __rspice_deriv_cse_70: f64 = (__rspice_deriv_cse_18 + __rspice_deriv_cse_44);
        let __rspice_deriv_cse_71: f64 = (__rspice_deriv_cse_19 + __rspice_deriv_cse_45);
        let __rspice_deriv_cse_72: f64 = (__rspice_deriv_cse_20 + __rspice_deriv_cse_46);
        let __rspice_deriv_cse_73: f64 = (__rspice_deriv_cse_21 + __rspice_deriv_cse_47);
        let __rspice_deriv_cse_74: f64 = (__rspice_deriv_cse_22 + __rspice_deriv_cse_48);
        let __rspice_deriv_cse_75: f64 = (__rspice_deriv_cse_23 + __rspice_deriv_cse_49);
        let __rspice_deriv_cse_76: f64 = (__rspice_deriv_cse_24 + __rspice_deriv_cse_50);
        let __rspice_deriv_cse_77: f64 = (__rspice_deriv_cse_25 + __rspice_deriv_cse_51);
        let __rspice_deriv_cse_78: f64 = (__rspice_deriv_cse_52 - s.dn[1017][0]);
        let __rspice_deriv_cse_79: f64 = (__rspice_deriv_cse_53 - s.dn[1017][1]);
        let __rspice_deriv_cse_80: f64 = (__rspice_deriv_cse_54 - s.dn[1017][2]);
        let __rspice_deriv_cse_81: f64 = (__rspice_deriv_cse_55 - s.dn[1017][3]);
        let __rspice_deriv_cse_82: f64 = (__rspice_deriv_cse_56 - s.dn[1017][4]);
        let __rspice_deriv_cse_83: f64 = (__rspice_deriv_cse_57 - s.dn[1017][5]);
        let __rspice_deriv_cse_84: f64 = (__rspice_deriv_cse_58 - s.dn[1017][6]);
        let __rspice_deriv_cse_85: f64 = (__rspice_deriv_cse_59 - s.dn[1017][7]);
        let __rspice_deriv_cse_86: f64 = (__rspice_deriv_cse_60 - s.dn[1017][8]);
        let __rspice_deriv_cse_87: f64 = (__rspice_deriv_cse_61 - s.dn[1017][9]);
        let __rspice_deriv_cse_88: f64 = (__rspice_deriv_cse_62 - s.dn[1017][10]);
        let __rspice_deriv_cse_89: f64 = (__rspice_deriv_cse_63 - s.dn[1017][11]);
        let __rspice_deriv_cse_90: f64 = (__rspice_deriv_cse_64 - s.dn[1017][12]);
        let __rspice_deriv_cse_91: f64 = (__rspice_deriv_cse_65 - s.dn[1017][13]);
        let __rspice_deriv_cse_92: f64 = (__rspice_deriv_cse_66 - s.db[1017][0]);
        let __rspice_deriv_cse_93: f64 = (__rspice_deriv_cse_67 - s.db[1017][1]);
        let __rspice_deriv_cse_94: f64 = (__rspice_deriv_cse_68 - s.db[1017][2]);
        let __rspice_deriv_cse_95: f64 = (__rspice_deriv_cse_69 - s.db[1017][3]);
        let __rspice_deriv_cse_96: f64 = (__rspice_deriv_cse_70 - s.db[1017][4]);
        let __rspice_deriv_cse_97: f64 = (__rspice_deriv_cse_71 - s.db[1017][5]);
        let __rspice_deriv_cse_98: f64 = (__rspice_deriv_cse_72 - s.db[1017][6]);
        let __rspice_deriv_cse_99: f64 = (__rspice_deriv_cse_73 - s.db[1017][7]);
        let __rspice_deriv_cse_100: f64 = (__rspice_deriv_cse_74 - s.db[1017][8]);
        let __rspice_deriv_cse_101: f64 = (__rspice_deriv_cse_75 - s.db[1017][9]);
        let __rspice_deriv_cse_102: f64 = (__rspice_deriv_cse_76 - s.db[1017][10]);
        let __rspice_deriv_cse_103: f64 = (__rspice_deriv_cse_77 - s.db[1017][11]);
        let eq41_e1950: f64 = (-s.v[379]);
        let eq41_e1952: f64 = (eq41_e1950 * s.v[423]);
        let eq41_e1952_d_n0: f64 = (((-s.dn[379][0]) * s.v[423]) + (eq41_e1950 * s.dn[423][0]));
        let eq41_e1952_d_n1: f64 = (((-s.dn[379][1]) * s.v[423]) + (eq41_e1950 * s.dn[423][1]));
        let eq41_e1952_d_n2: f64 = (((-s.dn[379][2]) * s.v[423]) + (eq41_e1950 * s.dn[423][2]));
        let eq41_e1952_d_n3: f64 = (((-s.dn[379][3]) * s.v[423]) + (eq41_e1950 * s.dn[423][3]));
        let eq41_e1952_d_n4: f64 = (((-s.dn[379][4]) * s.v[423]) + (eq41_e1950 * s.dn[423][4]));
        let eq41_e1952_d_n5: f64 = (((-s.dn[379][5]) * s.v[423]) + (eq41_e1950 * s.dn[423][5]));
        let eq41_e1952_d_n6: f64 = (((-s.dn[379][6]) * s.v[423]) + (eq41_e1950 * s.dn[423][6]));
        let eq41_e1952_d_n7: f64 = (((-s.dn[379][7]) * s.v[423]) + (eq41_e1950 * s.dn[423][7]));
        let eq41_e1952_d_n8: f64 = (((-s.dn[379][8]) * s.v[423]) + (eq41_e1950 * s.dn[423][8]));
        let eq41_e1952_d_n9: f64 = (((-s.dn[379][9]) * s.v[423]) + (eq41_e1950 * s.dn[423][9]));
        let eq41_e1952_d_n10: f64 = (((-s.dn[379][10]) * s.v[423]) + (eq41_e1950 * s.dn[423][10]));
        let eq41_e1952_d_n11: f64 = (((-s.dn[379][11]) * s.v[423]) + (eq41_e1950 * s.dn[423][11]));
        let eq41_e1952_d_n12: f64 = (((-s.dn[379][12]) * s.v[423]) + (eq41_e1950 * s.dn[423][12]));
        let eq41_e1952_d_n13: f64 = (((-s.dn[379][13]) * s.v[423]) + (eq41_e1950 * s.dn[423][13]));
        let eq41_e1952_d_b0: f64 = (((-s.db[379][0]) * s.v[423]) + (eq41_e1950 * s.db[423][0]));
        let eq41_e1952_d_b1: f64 = (((-s.db[379][1]) * s.v[423]) + (eq41_e1950 * s.db[423][1]));
        let eq41_e1952_d_b2: f64 = (((-s.db[379][2]) * s.v[423]) + (eq41_e1950 * s.db[423][2]));
        let eq41_e1952_d_b3: f64 = (((-s.db[379][3]) * s.v[423]) + (eq41_e1950 * s.db[423][3]));
        let eq41_e1952_d_b4: f64 = (((-s.db[379][4]) * s.v[423]) + (eq41_e1950 * s.db[423][4]));
        let eq41_e1952_d_b5: f64 = (((-s.db[379][5]) * s.v[423]) + (eq41_e1950 * s.db[423][5]));
        let eq41_e1952_d_b6: f64 = (((-s.db[379][6]) * s.v[423]) + (eq41_e1950 * s.db[423][6]));
        let eq41_e1952_d_b7: f64 = (((-s.db[379][7]) * s.v[423]) + (eq41_e1950 * s.db[423][7]));
        let eq41_e1952_d_b8: f64 = (((-s.db[379][8]) * s.v[423]) + (eq41_e1950 * s.db[423][8]));
        let eq41_e1952_d_b9: f64 = (((-s.db[379][9]) * s.v[423]) + (eq41_e1950 * s.db[423][9]));
        let eq41_e1952_d_b10: f64 = (((-s.db[379][10]) * s.v[423]) + (eq41_e1950 * s.db[423][10]));
        let eq41_e1952_d_b11: f64 = (((-s.db[379][11]) * s.v[423]) + (eq41_e1950 * s.db[423][11]));
        let eq41_e1953_q: f64 = eq41_e1952;
        let eq41_reactive_node_derivatives: [f64; 14] = [eq41_e1952_d_n0, eq41_e1952_d_n1, eq41_e1952_d_n2, eq41_e1952_d_n3, eq41_e1952_d_n4, eq41_e1952_d_n5, eq41_e1952_d_n6, eq41_e1952_d_n7, eq41_e1952_d_n8, eq41_e1952_d_n9, eq41_e1952_d_n10, eq41_e1952_d_n11, eq41_e1952_d_n12, eq41_e1952_d_n13];
        let eq41_reactive_branch_derivatives: [f64; 12] = [eq41_e1952_d_b0, eq41_e1952_d_b1, eq41_e1952_d_b2, eq41_e1952_d_b3, eq41_e1952_d_b4, eq41_e1952_d_b5, eq41_e1952_d_b6, eq41_e1952_d_b7, eq41_e1952_d_b8, eq41_e1952_d_b9, eq41_e1952_d_b10, eq41_e1952_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1955: f64 = (-s.v[379]);
        let eq42_e1957: f64 = (eq42_e1955 * s.v[424]);
        let eq42_e1957_d_n0: f64 = (((-s.dn[379][0]) * s.v[424]) + (eq42_e1955 * s.dn[424][0]));
        let eq42_e1957_d_n1: f64 = (((-s.dn[379][1]) * s.v[424]) + (eq42_e1955 * s.dn[424][1]));
        let eq42_e1957_d_n2: f64 = (((-s.dn[379][2]) * s.v[424]) + (eq42_e1955 * s.dn[424][2]));
        let eq42_e1957_d_n3: f64 = (((-s.dn[379][3]) * s.v[424]) + (eq42_e1955 * s.dn[424][3]));
        let eq42_e1957_d_n4: f64 = (((-s.dn[379][4]) * s.v[424]) + (eq42_e1955 * s.dn[424][4]));
        let eq42_e1957_d_n5: f64 = (((-s.dn[379][5]) * s.v[424]) + (eq42_e1955 * s.dn[424][5]));
        let eq42_e1957_d_n6: f64 = (((-s.dn[379][6]) * s.v[424]) + (eq42_e1955 * s.dn[424][6]));
        let eq42_e1957_d_n7: f64 = (((-s.dn[379][7]) * s.v[424]) + (eq42_e1955 * s.dn[424][7]));
        let eq42_e1957_d_n8: f64 = (((-s.dn[379][8]) * s.v[424]) + (eq42_e1955 * s.dn[424][8]));
        let eq42_e1957_d_n9: f64 = (((-s.dn[379][9]) * s.v[424]) + (eq42_e1955 * s.dn[424][9]));
        let eq42_e1957_d_n10: f64 = (((-s.dn[379][10]) * s.v[424]) + (eq42_e1955 * s.dn[424][10]));
        let eq42_e1957_d_n11: f64 = (((-s.dn[379][11]) * s.v[424]) + (eq42_e1955 * s.dn[424][11]));
        let eq42_e1957_d_n12: f64 = (((-s.dn[379][12]) * s.v[424]) + (eq42_e1955 * s.dn[424][12]));
        let eq42_e1957_d_n13: f64 = (((-s.dn[379][13]) * s.v[424]) + (eq42_e1955 * s.dn[424][13]));
        let eq42_e1957_d_b0: f64 = (((-s.db[379][0]) * s.v[424]) + (eq42_e1955 * s.db[424][0]));
        let eq42_e1957_d_b1: f64 = (((-s.db[379][1]) * s.v[424]) + (eq42_e1955 * s.db[424][1]));
        let eq42_e1957_d_b2: f64 = (((-s.db[379][2]) * s.v[424]) + (eq42_e1955 * s.db[424][2]));
        let eq42_e1957_d_b3: f64 = (((-s.db[379][3]) * s.v[424]) + (eq42_e1955 * s.db[424][3]));
        let eq42_e1957_d_b4: f64 = (((-s.db[379][4]) * s.v[424]) + (eq42_e1955 * s.db[424][4]));
        let eq42_e1957_d_b5: f64 = (((-s.db[379][5]) * s.v[424]) + (eq42_e1955 * s.db[424][5]));
        let eq42_e1957_d_b6: f64 = (((-s.db[379][6]) * s.v[424]) + (eq42_e1955 * s.db[424][6]));
        let eq42_e1957_d_b7: f64 = (((-s.db[379][7]) * s.v[424]) + (eq42_e1955 * s.db[424][7]));
        let eq42_e1957_d_b8: f64 = (((-s.db[379][8]) * s.v[424]) + (eq42_e1955 * s.db[424][8]));
        let eq42_e1957_d_b9: f64 = (((-s.db[379][9]) * s.v[424]) + (eq42_e1955 * s.db[424][9]));
        let eq42_e1957_d_b10: f64 = (((-s.db[379][10]) * s.v[424]) + (eq42_e1955 * s.db[424][10]));
        let eq42_e1957_d_b11: f64 = (((-s.db[379][11]) * s.v[424]) + (eq42_e1955 * s.db[424][11]));
        let eq42_e1958_q: f64 = eq42_e1957;
        let eq42_reactive_node_derivatives: [f64; 14] = [eq42_e1957_d_n0, eq42_e1957_d_n1, eq42_e1957_d_n2, eq42_e1957_d_n3, eq42_e1957_d_n4, eq42_e1957_d_n5, eq42_e1957_d_n6, eq42_e1957_d_n7, eq42_e1957_d_n8, eq42_e1957_d_n9, eq42_e1957_d_n10, eq42_e1957_d_n11, eq42_e1957_d_n12, eq42_e1957_d_n13];
        let eq42_reactive_branch_derivatives: [f64; 12] = [eq42_e1957_d_b0, eq42_e1957_d_b1, eq42_e1957_d_b2, eq42_e1957_d_b3, eq42_e1957_d_b4, eq42_e1957_d_b5, eq42_e1957_d_b6, eq42_e1957_d_b7, eq42_e1957_d_b8, eq42_e1957_d_b9, eq42_e1957_d_b10, eq42_e1957_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq45_e1969_q: f64 = s.v[1039];
        let eq45_e1970: f64 = (s.v[379] * s.v[1039]);
        let eq45_e1970_d_n0: f64 = ((s.dn[379][0] * s.v[1039]) + (s.v[379] * s.dn[1039][0]));
        let eq45_e1970_d_n1: f64 = ((s.dn[379][1] * s.v[1039]) + (s.v[379] * s.dn[1039][1]));
        let eq45_e1970_d_n2: f64 = ((s.dn[379][2] * s.v[1039]) + (s.v[379] * s.dn[1039][2]));
        let eq45_e1970_d_n3: f64 = ((s.dn[379][3] * s.v[1039]) + (s.v[379] * s.dn[1039][3]));
        let eq45_e1970_d_n4: f64 = ((s.dn[379][4] * s.v[1039]) + (s.v[379] * s.dn[1039][4]));
        let eq45_e1970_d_n5: f64 = ((s.dn[379][5] * s.v[1039]) + (s.v[379] * s.dn[1039][5]));
        let eq45_e1970_d_n6: f64 = ((s.dn[379][6] * s.v[1039]) + (s.v[379] * s.dn[1039][6]));
        let eq45_e1970_d_n7: f64 = ((s.dn[379][7] * s.v[1039]) + (s.v[379] * s.dn[1039][7]));
        let eq45_e1970_d_n8: f64 = ((s.dn[379][8] * s.v[1039]) + (s.v[379] * s.dn[1039][8]));
        let eq45_e1970_d_n9: f64 = ((s.dn[379][9] * s.v[1039]) + (s.v[379] * s.dn[1039][9]));
        let eq45_e1970_d_n10: f64 = ((s.dn[379][10] * s.v[1039]) + (s.v[379] * s.dn[1039][10]));
        let eq45_e1970_d_n11: f64 = ((s.dn[379][11] * s.v[1039]) + (s.v[379] * s.dn[1039][11]));
        let eq45_e1970_d_n12: f64 = ((s.dn[379][12] * s.v[1039]) + (s.v[379] * s.dn[1039][12]));
        let eq45_e1970_d_n13: f64 = ((s.dn[379][13] * s.v[1039]) + (s.v[379] * s.dn[1039][13]));
        let eq45_e1970_d_b0: f64 = ((s.db[379][0] * s.v[1039]) + (s.v[379] * s.db[1039][0]));
        let eq45_e1970_d_b1: f64 = ((s.db[379][1] * s.v[1039]) + (s.v[379] * s.db[1039][1]));
        let eq45_e1970_d_b2: f64 = ((s.db[379][2] * s.v[1039]) + (s.v[379] * s.db[1039][2]));
        let eq45_e1970_d_b3: f64 = ((s.db[379][3] * s.v[1039]) + (s.v[379] * s.db[1039][3]));
        let eq45_e1970_d_b4: f64 = ((s.db[379][4] * s.v[1039]) + (s.v[379] * s.db[1039][4]));
        let eq45_e1970_d_b5: f64 = ((s.db[379][5] * s.v[1039]) + (s.v[379] * s.db[1039][5]));
        let eq45_e1970_d_b6: f64 = ((s.db[379][6] * s.v[1039]) + (s.v[379] * s.db[1039][6]));
        let eq45_e1970_d_b7: f64 = ((s.db[379][7] * s.v[1039]) + (s.v[379] * s.db[1039][7]));
        let eq45_e1970_d_b8: f64 = ((s.db[379][8] * s.v[1039]) + (s.v[379] * s.db[1039][8]));
        let eq45_e1970_d_b9: f64 = ((s.db[379][9] * s.v[1039]) + (s.v[379] * s.db[1039][9]));
        let eq45_e1970_d_b10: f64 = ((s.db[379][10] * s.v[1039]) + (s.v[379] * s.db[1039][10]));
        let eq45_e1970_d_b11: f64 = ((s.db[379][11] * s.v[1039]) + (s.v[379] * s.db[1039][11]));
        let eq45_e1970_q: f64 = (s.v[379] * eq45_e1969_q);
        let eq45_e1970_q_d_n0: f64 = ((s.dn[379][0] * eq45_e1969_q) + (s.v[379] * s.dn[1039][0]));
        let eq45_e1970_q_d_n1: f64 = ((s.dn[379][1] * eq45_e1969_q) + (s.v[379] * s.dn[1039][1]));
        let eq45_e1970_q_d_n2: f64 = ((s.dn[379][2] * eq45_e1969_q) + (s.v[379] * s.dn[1039][2]));
        let eq45_e1970_q_d_n3: f64 = ((s.dn[379][3] * eq45_e1969_q) + (s.v[379] * s.dn[1039][3]));
        let eq45_e1970_q_d_n4: f64 = ((s.dn[379][4] * eq45_e1969_q) + (s.v[379] * s.dn[1039][4]));
        let eq45_e1970_q_d_n5: f64 = ((s.dn[379][5] * eq45_e1969_q) + (s.v[379] * s.dn[1039][5]));
        let eq45_e1970_q_d_n6: f64 = ((s.dn[379][6] * eq45_e1969_q) + (s.v[379] * s.dn[1039][6]));
        let eq45_e1970_q_d_n7: f64 = ((s.dn[379][7] * eq45_e1969_q) + (s.v[379] * s.dn[1039][7]));
        let eq45_e1970_q_d_n8: f64 = ((s.dn[379][8] * eq45_e1969_q) + (s.v[379] * s.dn[1039][8]));
        let eq45_e1970_q_d_n9: f64 = ((s.dn[379][9] * eq45_e1969_q) + (s.v[379] * s.dn[1039][9]));
        let eq45_e1970_q_d_n10: f64 = ((s.dn[379][10] * eq45_e1969_q) + (s.v[379] * s.dn[1039][10]));
        let eq45_e1970_q_d_n11: f64 = ((s.dn[379][11] * eq45_e1969_q) + (s.v[379] * s.dn[1039][11]));
        let eq45_e1970_q_d_n12: f64 = ((s.dn[379][12] * eq45_e1969_q) + (s.v[379] * s.dn[1039][12]));
        let eq45_e1970_q_d_n13: f64 = ((s.dn[379][13] * eq45_e1969_q) + (s.v[379] * s.dn[1039][13]));
        let eq45_e1970_q_d_b0: f64 = ((s.db[379][0] * eq45_e1969_q) + (s.v[379] * s.db[1039][0]));
        let eq45_e1970_q_d_b1: f64 = ((s.db[379][1] * eq45_e1969_q) + (s.v[379] * s.db[1039][1]));
        let eq45_e1970_q_d_b2: f64 = ((s.db[379][2] * eq45_e1969_q) + (s.v[379] * s.db[1039][2]));
        let eq45_e1970_q_d_b3: f64 = ((s.db[379][3] * eq45_e1969_q) + (s.v[379] * s.db[1039][3]));
        let eq45_e1970_q_d_b4: f64 = ((s.db[379][4] * eq45_e1969_q) + (s.v[379] * s.db[1039][4]));
        let eq45_e1970_q_d_b5: f64 = ((s.db[379][5] * eq45_e1969_q) + (s.v[379] * s.db[1039][5]));
        let eq45_e1970_q_d_b6: f64 = ((s.db[379][6] * eq45_e1969_q) + (s.v[379] * s.db[1039][6]));
        let eq45_e1970_q_d_b7: f64 = ((s.db[379][7] * eq45_e1969_q) + (s.v[379] * s.db[1039][7]));
        let eq45_e1970_q_d_b8: f64 = ((s.db[379][8] * eq45_e1969_q) + (s.v[379] * s.db[1039][8]));
        let eq45_e1970_q_d_b9: f64 = ((s.db[379][9] * eq45_e1969_q) + (s.v[379] * s.db[1039][9]));
        let eq45_e1970_q_d_b10: f64 = ((s.db[379][10] * eq45_e1969_q) + (s.v[379] * s.db[1039][10]));
        let eq45_e1970_q_d_b11: f64 = ((s.db[379][11] * eq45_e1969_q) + (s.v[379] * s.db[1039][11]));
        let eq45_reactive_node_derivatives: [f64; 14] = [eq45_e1970_q_d_n0, eq45_e1970_q_d_n1, eq45_e1970_q_d_n2, eq45_e1970_q_d_n3, eq45_e1970_q_d_n4, eq45_e1970_q_d_n5, eq45_e1970_q_d_n6, eq45_e1970_q_d_n7, eq45_e1970_q_d_n8, eq45_e1970_q_d_n9, eq45_e1970_q_d_n10, eq45_e1970_q_d_n11, eq45_e1970_q_d_n12, eq45_e1970_q_d_n13];
        let eq45_reactive_branch_derivatives: [f64; 12] = [eq45_e1970_q_d_b0, eq45_e1970_q_d_b1, eq45_e1970_q_d_b2, eq45_e1970_q_d_b3, eq45_e1970_q_d_b4, eq45_e1970_q_d_b5, eq45_e1970_q_d_b6, eq45_e1970_q_d_b7, eq45_e1970_q_d_b8, eq45_e1970_q_d_b9, eq45_e1970_q_d_b10, eq45_e1970_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1972_q: f64 = s.v[1047];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            nodes,
            &s.dn[1047],
            branches,
            &s.db[1047],
            multiplicity,
        );
        let eq47_e1974_q: f64 = s.v[1046];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &s.dn[1046],
            branches,
            &s.db[1046],
            multiplicity,
        );
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n1, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11, eq67_e2103_d_n12, eq67_e2103_d_n13, eq67_e2103_d_b0, eq67_e2103_d_b1, eq67_e2103_d_b2, eq67_e2103_d_b3, eq67_e2103_d_b4, eq67_e2103_d_b5, eq67_e2103_d_b6, eq67_e2103_d_b7, eq67_e2103_d_b8, eq67_e2103_d_b9, eq67_e2103_d_b10, eq67_e2103_d_b11, eq67_e2103_q, eq67_e2103_q_d_n0, eq67_e2103_q_d_n1, eq67_e2103_q_d_n2, eq67_e2103_q_d_n3, eq67_e2103_q_d_n4, eq67_e2103_q_d_n5, eq67_e2103_q_d_n6, eq67_e2103_q_d_n7, eq67_e2103_q_d_n8, eq67_e2103_q_d_n9, eq67_e2103_q_d_n10, eq67_e2103_q_d_n11, eq67_e2103_q_d_n12, eq67_e2103_q_d_n13, eq67_e2103_q_d_b0, eq67_e2103_q_d_b1, eq67_e2103_q_d_b2, eq67_e2103_q_d_b3, eq67_e2103_q_d_b4, eq67_e2103_q_d_b5, eq67_e2103_q_d_b6, eq67_e2103_q_d_b7, eq67_e2103_q_d_b8, eq67_e2103_q_d_b9, eq67_e2103_q_d_b10, eq67_e2103_q_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && s.b[2025]) {
        let eq67_e2094: f64 = (s.v[634] * s.v[1015]);
        let eq67_e2097: f64 = (s.v[634] * s.v[1016]);
        let eq67_e2098_q: f64 = eq67_e2097;
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2097);
        let eq67_e2099_q: f64 = eq67_e2098_q;
        let eq67_e2101: f64 = (eq67_e2099 - s.v[1017]);
        let eq67_e2101_q: f64 = eq67_e2099_q;
        (eq67_e2101, __rspice_deriv_cse_78, __rspice_deriv_cse_79, __rspice_deriv_cse_80, __rspice_deriv_cse_81, __rspice_deriv_cse_82, __rspice_deriv_cse_83, __rspice_deriv_cse_84, __rspice_deriv_cse_85, __rspice_deriv_cse_86, __rspice_deriv_cse_87, __rspice_deriv_cse_88, __rspice_deriv_cse_89, __rspice_deriv_cse_90, __rspice_deriv_cse_91, __rspice_deriv_cse_92, __rspice_deriv_cse_93, __rspice_deriv_cse_94, __rspice_deriv_cse_95, __rspice_deriv_cse_96, __rspice_deriv_cse_97, __rspice_deriv_cse_98, __rspice_deriv_cse_99, __rspice_deriv_cse_100, __rspice_deriv_cse_101, __rspice_deriv_cse_102, __rspice_deriv_cse_103, eq67_e2101_q, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_reactive_node_derivatives: [f64; 14] = [eq67_e2103_q_d_n0, eq67_e2103_q_d_n1, eq67_e2103_q_d_n2, eq67_e2103_q_d_n3, eq67_e2103_q_d_n4, eq67_e2103_q_d_n5, eq67_e2103_q_d_n6, eq67_e2103_q_d_n7, eq67_e2103_q_d_n8, eq67_e2103_q_d_n9, eq67_e2103_q_d_n10, eq67_e2103_q_d_n11, eq67_e2103_q_d_n12, eq67_e2103_q_d_n13];
        let eq67_reactive_branch_derivatives: [f64; 12] = [eq67_e2103_q_d_b0, eq67_e2103_q_d_b1, eq67_e2103_q_d_b2, eq67_e2103_q_d_b3, eq67_e2103_q_d_b4, eq67_e2103_q_d_b5, eq67_e2103_q_d_b6, eq67_e2103_q_d_b7, eq67_e2103_q_d_b8, eq67_e2103_q_d_b9, eq67_e2103_q_d_b10, eq67_e2103_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq67_reactive_node_derivatives,
            branches,
            &eq67_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n1, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11, eq68_e2121_d_n12, eq68_e2121_d_n13, eq68_e2121_d_b0, eq68_e2121_d_b1, eq68_e2121_d_b2, eq68_e2121_d_b3, eq68_e2121_d_b4, eq68_e2121_d_b5, eq68_e2121_d_b6, eq68_e2121_d_b7, eq68_e2121_d_b8, eq68_e2121_d_b9, eq68_e2121_d_b10, eq68_e2121_d_b11, eq68_e2121_q, eq68_e2121_q_d_n0, eq68_e2121_q_d_n1, eq68_e2121_q_d_n2, eq68_e2121_q_d_n3, eq68_e2121_q_d_n4, eq68_e2121_q_d_n5, eq68_e2121_q_d_n6, eq68_e2121_q_d_n7, eq68_e2121_q_d_n8, eq68_e2121_q_d_n9, eq68_e2121_q_d_n10, eq68_e2121_q_d_n11, eq68_e2121_q_d_n12, eq68_e2121_q_d_n13, eq68_e2121_q_d_b0, eq68_e2121_q_d_b1, eq68_e2121_q_d_b2, eq68_e2121_q_d_b3, eq68_e2121_q_d_b4, eq68_e2121_q_d_b5, eq68_e2121_q_d_b6, eq68_e2121_q_d_b7, eq68_e2121_q_d_b8, eq68_e2121_q_d_b9, eq68_e2121_q_d_b10, eq68_e2121_q_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && (!s.b[2025])) {
        let eq68_e2112: f64 = (s.v[634] * s.v[1015]);
        let eq68_e2115: f64 = (s.v[634] * s.v[1016]);
        let eq68_e2116_q: f64 = eq68_e2115;
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2115);
        let eq68_e2117_q: f64 = eq68_e2116_q;
        let eq68_e2119: f64 = (eq68_e2117 - s.v[1017]);
        let eq68_e2119_q: f64 = eq68_e2117_q;
        (eq68_e2119, __rspice_deriv_cse_78, __rspice_deriv_cse_79, __rspice_deriv_cse_80, __rspice_deriv_cse_81, __rspice_deriv_cse_82, __rspice_deriv_cse_83, __rspice_deriv_cse_84, __rspice_deriv_cse_85, __rspice_deriv_cse_86, __rspice_deriv_cse_87, __rspice_deriv_cse_88, __rspice_deriv_cse_89, __rspice_deriv_cse_90, __rspice_deriv_cse_91, __rspice_deriv_cse_92, __rspice_deriv_cse_93, __rspice_deriv_cse_94, __rspice_deriv_cse_95, __rspice_deriv_cse_96, __rspice_deriv_cse_97, __rspice_deriv_cse_98, __rspice_deriv_cse_99, __rspice_deriv_cse_100, __rspice_deriv_cse_101, __rspice_deriv_cse_102, __rspice_deriv_cse_103, eq68_e2119_q, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_reactive_node_derivatives: [f64; 14] = [eq68_e2121_q_d_n0, eq68_e2121_q_d_n1, eq68_e2121_q_d_n2, eq68_e2121_q_d_n3, eq68_e2121_q_d_n4, eq68_e2121_q_d_n5, eq68_e2121_q_d_n6, eq68_e2121_q_d_n7, eq68_e2121_q_d_n8, eq68_e2121_q_d_n9, eq68_e2121_q_d_n10, eq68_e2121_q_d_n11, eq68_e2121_q_d_n12, eq68_e2121_q_d_n13];
        let eq68_reactive_branch_derivatives: [f64; 12] = [eq68_e2121_q_d_b0, eq68_e2121_q_d_b1, eq68_e2121_q_d_b2, eq68_e2121_q_d_b3, eq68_e2121_q_d_b4, eq68_e2121_q_d_b5, eq68_e2121_q_d_b6, eq68_e2121_q_d_b7, eq68_e2121_q_d_b8, eq68_e2121_q_d_b9, eq68_e2121_q_d_b10, eq68_e2121_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq68_reactive_node_derivatives,
            branches,
            &eq68_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_4(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n1, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11, eq69_e2137_d_n12, eq69_e2137_d_n13, eq69_e2137_d_b0, eq69_e2137_d_b1, eq69_e2137_d_b2, eq69_e2137_d_b3, eq69_e2137_d_b4, eq69_e2137_d_b5, eq69_e2137_d_b6, eq69_e2137_d_b7, eq69_e2137_d_b8, eq69_e2137_d_b9, eq69_e2137_d_b10, eq69_e2137_d_b11, eq69_e2137_q, eq69_e2137_q_d_n0, eq69_e2137_q_d_n1, eq69_e2137_q_d_n2, eq69_e2137_q_d_n3, eq69_e2137_q_d_n4, eq69_e2137_q_d_n5, eq69_e2137_q_d_n6, eq69_e2137_q_d_n7, eq69_e2137_q_d_n8, eq69_e2137_q_d_n9, eq69_e2137_q_d_n10, eq69_e2137_q_d_n11, eq69_e2137_q_d_n12, eq69_e2137_q_d_n13, eq69_e2137_q_d_b0, eq69_e2137_q_d_b1, eq69_e2137_q_d_b2, eq69_e2137_q_d_b3, eq69_e2137_q_d_b4, eq69_e2137_q_d_b5, eq69_e2137_q_d_b6, eq69_e2137_q_d_b7, eq69_e2137_q_d_b8, eq69_e2137_q_d_b9, eq69_e2137_q_d_b10, eq69_e2137_q_d_b11,) = {
    if (s.b[2021] && (!s.b[2024])) {
        let eq69_e2128: f64 = (s.v[634] * s.v[1015]);
        let eq69_e2128_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq69_e2128_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq69_e2128_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq69_e2128_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq69_e2128_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq69_e2128_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq69_e2128_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq69_e2128_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq69_e2128_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq69_e2128_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq69_e2128_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq69_e2128_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq69_e2128_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq69_e2128_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq69_e2128_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));
        let eq69_e2128_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));
        let eq69_e2128_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));
        let eq69_e2128_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));
        let eq69_e2128_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));
        let eq69_e2128_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));
        let eq69_e2128_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));
        let eq69_e2128_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));
        let eq69_e2128_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));
        let eq69_e2128_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));
        let eq69_e2128_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));
        let eq69_e2128_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));
        let eq69_e2131: f64 = (s.v[634] * s.v[1016]);
        let eq69_e2131_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq69_e2131_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq69_e2131_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq69_e2131_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq69_e2131_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq69_e2131_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq69_e2131_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq69_e2131_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq69_e2131_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq69_e2131_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq69_e2131_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq69_e2131_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq69_e2131_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq69_e2131_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq69_e2131_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));
        let eq69_e2131_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));
        let eq69_e2131_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));
        let eq69_e2131_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));
        let eq69_e2131_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));
        let eq69_e2131_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq69_e2131_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));
        let eq69_e2131_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));
        let eq69_e2131_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));
        let eq69_e2131_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));
        let eq69_e2131_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));
        let eq69_e2131_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));
        let eq69_e2132_q: f64 = eq69_e2131;
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2131);
        let eq69_e2133_d_n0: f64 = (eq69_e2128_d_n0 + eq69_e2131_d_n0);
        let eq69_e2133_d_n1: f64 = (eq69_e2128_d_n1 + eq69_e2131_d_n1);
        let eq69_e2133_d_n2: f64 = (eq69_e2128_d_n2 + eq69_e2131_d_n2);
        let eq69_e2133_d_n3: f64 = (eq69_e2128_d_n3 + eq69_e2131_d_n3);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + eq69_e2131_d_n4);
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + eq69_e2131_d_n5);
        let eq69_e2133_d_n6: f64 = (eq69_e2128_d_n6 + eq69_e2131_d_n6);
        let eq69_e2133_d_n7: f64 = (eq69_e2128_d_n7 + eq69_e2131_d_n7);
        let eq69_e2133_d_n8: f64 = (eq69_e2128_d_n8 + eq69_e2131_d_n8);
        let eq69_e2133_d_n9: f64 = (eq69_e2128_d_n9 + eq69_e2131_d_n9);
        let eq69_e2133_d_n10: f64 = (eq69_e2128_d_n10 + eq69_e2131_d_n10);
        let eq69_e2133_d_n11: f64 = (eq69_e2128_d_n11 + eq69_e2131_d_n11);
        let eq69_e2133_d_n12: f64 = (eq69_e2128_d_n12 + eq69_e2131_d_n12);
        let eq69_e2133_d_n13: f64 = (eq69_e2128_d_n13 + eq69_e2131_d_n13);
        let eq69_e2133_d_b0: f64 = (eq69_e2128_d_b0 + eq69_e2131_d_b0);
        let eq69_e2133_d_b1: f64 = (eq69_e2128_d_b1 + eq69_e2131_d_b1);
        let eq69_e2133_d_b2: f64 = (eq69_e2128_d_b2 + eq69_e2131_d_b2);
        let eq69_e2133_d_b3: f64 = (eq69_e2128_d_b3 + eq69_e2131_d_b3);
        let eq69_e2133_d_b4: f64 = (eq69_e2128_d_b4 + eq69_e2131_d_b4);
        let eq69_e2133_d_b5: f64 = (eq69_e2128_d_b5 + eq69_e2131_d_b5);
        let eq69_e2133_d_b6: f64 = (eq69_e2128_d_b6 + eq69_e2131_d_b6);
        let eq69_e2133_d_b7: f64 = (eq69_e2128_d_b7 + eq69_e2131_d_b7);
        let eq69_e2133_d_b8: f64 = (eq69_e2128_d_b8 + eq69_e2131_d_b8);
        let eq69_e2133_d_b9: f64 = (eq69_e2128_d_b9 + eq69_e2131_d_b9);
        let eq69_e2133_d_b10: f64 = (eq69_e2128_d_b10 + eq69_e2131_d_b10);
        let eq69_e2133_d_b11: f64 = (eq69_e2128_d_b11 + eq69_e2131_d_b11);
        let eq69_e2133_q: f64 = eq69_e2132_q;
        let eq69_e2135: f64 = (eq69_e2133 - s.v[1017]);
        let eq69_e2135_d_n0: f64 = (eq69_e2133_d_n0 - s.dn[1017][0]);
        let eq69_e2135_d_n1: f64 = (eq69_e2133_d_n1 - s.dn[1017][1]);
        let eq69_e2135_d_n2: f64 = (eq69_e2133_d_n2 - s.dn[1017][2]);
        let eq69_e2135_d_n3: f64 = (eq69_e2133_d_n3 - s.dn[1017][3]);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - s.dn[1017][4]);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - s.dn[1017][5]);
        let eq69_e2135_d_n6: f64 = (eq69_e2133_d_n6 - s.dn[1017][6]);
        let eq69_e2135_d_n7: f64 = (eq69_e2133_d_n7 - s.dn[1017][7]);
        let eq69_e2135_d_n8: f64 = (eq69_e2133_d_n8 - s.dn[1017][8]);
        let eq69_e2135_d_n9: f64 = (eq69_e2133_d_n9 - s.dn[1017][9]);
        let eq69_e2135_d_n10: f64 = (eq69_e2133_d_n10 - s.dn[1017][10]);
        let eq69_e2135_d_n11: f64 = (eq69_e2133_d_n11 - s.dn[1017][11]);
        let eq69_e2135_d_n12: f64 = (eq69_e2133_d_n12 - s.dn[1017][12]);
        let eq69_e2135_d_n13: f64 = (eq69_e2133_d_n13 - s.dn[1017][13]);
        let eq69_e2135_d_b0: f64 = (eq69_e2133_d_b0 - s.db[1017][0]);
        let eq69_e2135_d_b1: f64 = (eq69_e2133_d_b1 - s.db[1017][1]);
        let eq69_e2135_d_b2: f64 = (eq69_e2133_d_b2 - s.db[1017][2]);
        let eq69_e2135_d_b3: f64 = (eq69_e2133_d_b3 - s.db[1017][3]);
        let eq69_e2135_d_b4: f64 = (eq69_e2133_d_b4 - s.db[1017][4]);
        let eq69_e2135_d_b5: f64 = (eq69_e2133_d_b5 - s.db[1017][5]);
        let eq69_e2135_d_b6: f64 = (eq69_e2133_d_b6 - s.db[1017][6]);
        let eq69_e2135_d_b7: f64 = (eq69_e2133_d_b7 - s.db[1017][7]);
        let eq69_e2135_d_b8: f64 = (eq69_e2133_d_b8 - s.db[1017][8]);
        let eq69_e2135_d_b9: f64 = (eq69_e2133_d_b9 - s.db[1017][9]);
        let eq69_e2135_d_b10: f64 = (eq69_e2133_d_b10 - s.db[1017][10]);
        let eq69_e2135_d_b11: f64 = (eq69_e2133_d_b11 - s.db[1017][11]);
        let eq69_e2135_q: f64 = eq69_e2133_q;
        (eq69_e2135, eq69_e2135_d_n0, eq69_e2135_d_n1, eq69_e2135_d_n2, eq69_e2135_d_n3, eq69_e2135_d_n4, eq69_e2135_d_n5, eq69_e2135_d_n6, eq69_e2135_d_n7, eq69_e2135_d_n8, eq69_e2135_d_n9, eq69_e2135_d_n10, eq69_e2135_d_n11, eq69_e2135_d_n12, eq69_e2135_d_n13, eq69_e2135_d_b0, eq69_e2135_d_b1, eq69_e2135_d_b2, eq69_e2135_d_b3, eq69_e2135_d_b4, eq69_e2135_d_b5, eq69_e2135_d_b6, eq69_e2135_d_b7, eq69_e2135_d_b8, eq69_e2135_d_b9, eq69_e2135_d_b10, eq69_e2135_d_b11, eq69_e2135_q, eq69_e2131_d_n0, eq69_e2131_d_n1, eq69_e2131_d_n2, eq69_e2131_d_n3, eq69_e2131_d_n4, eq69_e2131_d_n5, eq69_e2131_d_n6, eq69_e2131_d_n7, eq69_e2131_d_n8, eq69_e2131_d_n9, eq69_e2131_d_n10, eq69_e2131_d_n11, eq69_e2131_d_n12, eq69_e2131_d_n13, eq69_e2131_d_b0, eq69_e2131_d_b1, eq69_e2131_d_b2, eq69_e2131_d_b3, eq69_e2131_d_b4, eq69_e2131_d_b5, eq69_e2131_d_b6, eq69_e2131_d_b7, eq69_e2131_d_b8, eq69_e2131_d_b9, eq69_e2131_d_b10, eq69_e2131_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_reactive_node_derivatives: [f64; 14] = [eq69_e2137_q_d_n0, eq69_e2137_q_d_n1, eq69_e2137_q_d_n2, eq69_e2137_q_d_n3, eq69_e2137_q_d_n4, eq69_e2137_q_d_n5, eq69_e2137_q_d_n6, eq69_e2137_q_d_n7, eq69_e2137_q_d_n8, eq69_e2137_q_d_n9, eq69_e2137_q_d_n10, eq69_e2137_q_d_n11, eq69_e2137_q_d_n12, eq69_e2137_q_d_n13];
        let eq69_reactive_branch_derivatives: [f64; 12] = [eq69_e2137_q_d_b0, eq69_e2137_q_d_b1, eq69_e2137_q_d_b2, eq69_e2137_q_d_b3, eq69_e2137_q_d_b4, eq69_e2137_q_d_b5, eq69_e2137_q_d_b6, eq69_e2137_q_d_b7, eq69_e2137_q_d_b8, eq69_e2137_q_d_b9, eq69_e2137_q_d_b10, eq69_e2137_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq69_reactive_node_derivatives,
            branches,
            &eq69_reactive_branch_derivatives,
            multiplicity,
        );
        let eq80_e2212_q: f64 = s.v[520];
        let eq80_e2213: f64 = (s.v[379] * s.v[520]);
        let eq80_e2213_d_n0: f64 = ((s.dn[379][0] * s.v[520]) + (s.v[379] * s.dn[520][0]));
        let eq80_e2213_d_n1: f64 = ((s.dn[379][1] * s.v[520]) + (s.v[379] * s.dn[520][1]));
        let eq80_e2213_d_n2: f64 = ((s.dn[379][2] * s.v[520]) + (s.v[379] * s.dn[520][2]));
        let eq80_e2213_d_n3: f64 = ((s.dn[379][3] * s.v[520]) + (s.v[379] * s.dn[520][3]));
        let eq80_e2213_d_n4: f64 = ((s.dn[379][4] * s.v[520]) + (s.v[379] * s.dn[520][4]));
        let eq80_e2213_d_n5: f64 = ((s.dn[379][5] * s.v[520]) + (s.v[379] * s.dn[520][5]));
        let eq80_e2213_d_n6: f64 = ((s.dn[379][6] * s.v[520]) + (s.v[379] * s.dn[520][6]));
        let eq80_e2213_d_n7: f64 = ((s.dn[379][7] * s.v[520]) + (s.v[379] * s.dn[520][7]));
        let eq80_e2213_d_n8: f64 = ((s.dn[379][8] * s.v[520]) + (s.v[379] * s.dn[520][8]));
        let eq80_e2213_d_n9: f64 = ((s.dn[379][9] * s.v[520]) + (s.v[379] * s.dn[520][9]));
        let eq80_e2213_d_n10: f64 = ((s.dn[379][10] * s.v[520]) + (s.v[379] * s.dn[520][10]));
        let eq80_e2213_d_n11: f64 = ((s.dn[379][11] * s.v[520]) + (s.v[379] * s.dn[520][11]));
        let eq80_e2213_d_n12: f64 = ((s.dn[379][12] * s.v[520]) + (s.v[379] * s.dn[520][12]));
        let eq80_e2213_d_n13: f64 = ((s.dn[379][13] * s.v[520]) + (s.v[379] * s.dn[520][13]));
        let eq80_e2213_d_b0: f64 = ((s.db[379][0] * s.v[520]) + (s.v[379] * s.db[520][0]));
        let eq80_e2213_d_b1: f64 = ((s.db[379][1] * s.v[520]) + (s.v[379] * s.db[520][1]));
        let eq80_e2213_d_b2: f64 = ((s.db[379][2] * s.v[520]) + (s.v[379] * s.db[520][2]));
        let eq80_e2213_d_b3: f64 = ((s.db[379][3] * s.v[520]) + (s.v[379] * s.db[520][3]));
        let eq80_e2213_d_b4: f64 = ((s.db[379][4] * s.v[520]) + (s.v[379] * s.db[520][4]));
        let eq80_e2213_d_b5: f64 = ((s.db[379][5] * s.v[520]) + (s.v[379] * s.db[520][5]));
        let eq80_e2213_d_b6: f64 = ((s.db[379][6] * s.v[520]) + (s.v[379] * s.db[520][6]));
        let eq80_e2213_d_b7: f64 = ((s.db[379][7] * s.v[520]) + (s.v[379] * s.db[520][7]));
        let eq80_e2213_d_b8: f64 = ((s.db[379][8] * s.v[520]) + (s.v[379] * s.db[520][8]));
        let eq80_e2213_d_b9: f64 = ((s.db[379][9] * s.v[520]) + (s.v[379] * s.db[520][9]));
        let eq80_e2213_d_b10: f64 = ((s.db[379][10] * s.v[520]) + (s.v[379] * s.db[520][10]));
        let eq80_e2213_d_b11: f64 = ((s.db[379][11] * s.v[520]) + (s.v[379] * s.db[520][11]));
        let eq80_e2213_q: f64 = (s.v[379] * eq80_e2212_q);
        let eq80_e2213_q_d_n0: f64 = ((s.dn[379][0] * eq80_e2212_q) + (s.v[379] * s.dn[520][0]));
        let eq80_e2213_q_d_n1: f64 = ((s.dn[379][1] * eq80_e2212_q) + (s.v[379] * s.dn[520][1]));
        let eq80_e2213_q_d_n2: f64 = ((s.dn[379][2] * eq80_e2212_q) + (s.v[379] * s.dn[520][2]));
        let eq80_e2213_q_d_n3: f64 = ((s.dn[379][3] * eq80_e2212_q) + (s.v[379] * s.dn[520][3]));
        let eq80_e2213_q_d_n4: f64 = ((s.dn[379][4] * eq80_e2212_q) + (s.v[379] * s.dn[520][4]));
        let eq80_e2213_q_d_n5: f64 = ((s.dn[379][5] * eq80_e2212_q) + (s.v[379] * s.dn[520][5]));
        let eq80_e2213_q_d_n6: f64 = ((s.dn[379][6] * eq80_e2212_q) + (s.v[379] * s.dn[520][6]));
        let eq80_e2213_q_d_n7: f64 = ((s.dn[379][7] * eq80_e2212_q) + (s.v[379] * s.dn[520][7]));
        let eq80_e2213_q_d_n8: f64 = ((s.dn[379][8] * eq80_e2212_q) + (s.v[379] * s.dn[520][8]));
        let eq80_e2213_q_d_n9: f64 = ((s.dn[379][9] * eq80_e2212_q) + (s.v[379] * s.dn[520][9]));
        let eq80_e2213_q_d_n10: f64 = ((s.dn[379][10] * eq80_e2212_q) + (s.v[379] * s.dn[520][10]));
        let eq80_e2213_q_d_n11: f64 = ((s.dn[379][11] * eq80_e2212_q) + (s.v[379] * s.dn[520][11]));
        let eq80_e2213_q_d_n12: f64 = ((s.dn[379][12] * eq80_e2212_q) + (s.v[379] * s.dn[520][12]));
        let eq80_e2213_q_d_n13: f64 = ((s.dn[379][13] * eq80_e2212_q) + (s.v[379] * s.dn[520][13]));
        let eq80_e2213_q_d_b0: f64 = ((s.db[379][0] * eq80_e2212_q) + (s.v[379] * s.db[520][0]));
        let eq80_e2213_q_d_b1: f64 = ((s.db[379][1] * eq80_e2212_q) + (s.v[379] * s.db[520][1]));
        let eq80_e2213_q_d_b2: f64 = ((s.db[379][2] * eq80_e2212_q) + (s.v[379] * s.db[520][2]));
        let eq80_e2213_q_d_b3: f64 = ((s.db[379][3] * eq80_e2212_q) + (s.v[379] * s.db[520][3]));
        let eq80_e2213_q_d_b4: f64 = ((s.db[379][4] * eq80_e2212_q) + (s.v[379] * s.db[520][4]));
        let eq80_e2213_q_d_b5: f64 = ((s.db[379][5] * eq80_e2212_q) + (s.v[379] * s.db[520][5]));
        let eq80_e2213_q_d_b6: f64 = ((s.db[379][6] * eq80_e2212_q) + (s.v[379] * s.db[520][6]));
        let eq80_e2213_q_d_b7: f64 = ((s.db[379][7] * eq80_e2212_q) + (s.v[379] * s.db[520][7]));
        let eq80_e2213_q_d_b8: f64 = ((s.db[379][8] * eq80_e2212_q) + (s.v[379] * s.db[520][8]));
        let eq80_e2213_q_d_b9: f64 = ((s.db[379][9] * eq80_e2212_q) + (s.v[379] * s.db[520][9]));
        let eq80_e2213_q_d_b10: f64 = ((s.db[379][10] * eq80_e2212_q) + (s.v[379] * s.db[520][10]));
        let eq80_e2213_q_d_b11: f64 = ((s.db[379][11] * eq80_e2212_q) + (s.v[379] * s.db[520][11]));
        let eq80_reactive_node_derivatives: [f64; 14] = [eq80_e2213_q_d_n0, eq80_e2213_q_d_n1, eq80_e2213_q_d_n2, eq80_e2213_q_d_n3, eq80_e2213_q_d_n4, eq80_e2213_q_d_n5, eq80_e2213_q_d_n6, eq80_e2213_q_d_n7, eq80_e2213_q_d_n8, eq80_e2213_q_d_n9, eq80_e2213_q_d_n10, eq80_e2213_q_d_n11, eq80_e2213_q_d_n12, eq80_e2213_q_d_n13];
        let eq80_reactive_branch_derivatives: [f64; 12] = [eq80_e2213_q_d_b0, eq80_e2213_q_d_b1, eq80_e2213_q_d_b2, eq80_e2213_q_d_b3, eq80_e2213_q_d_b4, eq80_e2213_q_d_b5, eq80_e2213_q_d_b6, eq80_e2213_q_d_b7, eq80_e2213_q_d_b8, eq80_e2213_q_d_b9, eq80_e2213_q_d_b10, eq80_e2213_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq80_reactive_node_derivatives,
            branches,
            &eq80_reactive_branch_derivatives,
            multiplicity,
        );
        let eq81_e2216_q: f64 = s.v[525];
        let eq81_e2217: f64 = (s.v[379] * s.v[525]);
        let eq81_e2217_d_n0: f64 = ((s.dn[379][0] * s.v[525]) + (s.v[379] * s.dn[525][0]));
        let eq81_e2217_d_n1: f64 = ((s.dn[379][1] * s.v[525]) + (s.v[379] * s.dn[525][1]));
        let eq81_e2217_d_n2: f64 = ((s.dn[379][2] * s.v[525]) + (s.v[379] * s.dn[525][2]));
        let eq81_e2217_d_n3: f64 = ((s.dn[379][3] * s.v[525]) + (s.v[379] * s.dn[525][3]));
        let eq81_e2217_d_n4: f64 = ((s.dn[379][4] * s.v[525]) + (s.v[379] * s.dn[525][4]));
        let eq81_e2217_d_n5: f64 = ((s.dn[379][5] * s.v[525]) + (s.v[379] * s.dn[525][5]));
        let eq81_e2217_d_n6: f64 = ((s.dn[379][6] * s.v[525]) + (s.v[379] * s.dn[525][6]));
        let eq81_e2217_d_n7: f64 = ((s.dn[379][7] * s.v[525]) + (s.v[379] * s.dn[525][7]));
        let eq81_e2217_d_n8: f64 = ((s.dn[379][8] * s.v[525]) + (s.v[379] * s.dn[525][8]));
        let eq81_e2217_d_n9: f64 = ((s.dn[379][9] * s.v[525]) + (s.v[379] * s.dn[525][9]));
        let eq81_e2217_d_n10: f64 = ((s.dn[379][10] * s.v[525]) + (s.v[379] * s.dn[525][10]));
        let eq81_e2217_d_n11: f64 = ((s.dn[379][11] * s.v[525]) + (s.v[379] * s.dn[525][11]));
        let eq81_e2217_d_n12: f64 = ((s.dn[379][12] * s.v[525]) + (s.v[379] * s.dn[525][12]));
        let eq81_e2217_d_n13: f64 = ((s.dn[379][13] * s.v[525]) + (s.v[379] * s.dn[525][13]));
        let eq81_e2217_d_b0: f64 = ((s.db[379][0] * s.v[525]) + (s.v[379] * s.db[525][0]));
        let eq81_e2217_d_b1: f64 = ((s.db[379][1] * s.v[525]) + (s.v[379] * s.db[525][1]));
        let eq81_e2217_d_b2: f64 = ((s.db[379][2] * s.v[525]) + (s.v[379] * s.db[525][2]));
        let eq81_e2217_d_b3: f64 = ((s.db[379][3] * s.v[525]) + (s.v[379] * s.db[525][3]));
        let eq81_e2217_d_b4: f64 = ((s.db[379][4] * s.v[525]) + (s.v[379] * s.db[525][4]));
        let eq81_e2217_d_b5: f64 = ((s.db[379][5] * s.v[525]) + (s.v[379] * s.db[525][5]));
        let eq81_e2217_d_b6: f64 = ((s.db[379][6] * s.v[525]) + (s.v[379] * s.db[525][6]));
        let eq81_e2217_d_b7: f64 = ((s.db[379][7] * s.v[525]) + (s.v[379] * s.db[525][7]));
        let eq81_e2217_d_b8: f64 = ((s.db[379][8] * s.v[525]) + (s.v[379] * s.db[525][8]));
        let eq81_e2217_d_b9: f64 = ((s.db[379][9] * s.v[525]) + (s.v[379] * s.db[525][9]));
        let eq81_e2217_d_b10: f64 = ((s.db[379][10] * s.v[525]) + (s.v[379] * s.db[525][10]));
        let eq81_e2217_d_b11: f64 = ((s.db[379][11] * s.v[525]) + (s.v[379] * s.db[525][11]));
        let eq81_e2217_q: f64 = (s.v[379] * eq81_e2216_q);
        let eq81_e2217_q_d_n0: f64 = ((s.dn[379][0] * eq81_e2216_q) + (s.v[379] * s.dn[525][0]));
        let eq81_e2217_q_d_n1: f64 = ((s.dn[379][1] * eq81_e2216_q) + (s.v[379] * s.dn[525][1]));
        let eq81_e2217_q_d_n2: f64 = ((s.dn[379][2] * eq81_e2216_q) + (s.v[379] * s.dn[525][2]));
        let eq81_e2217_q_d_n3: f64 = ((s.dn[379][3] * eq81_e2216_q) + (s.v[379] * s.dn[525][3]));
        let eq81_e2217_q_d_n4: f64 = ((s.dn[379][4] * eq81_e2216_q) + (s.v[379] * s.dn[525][4]));
        let eq81_e2217_q_d_n5: f64 = ((s.dn[379][5] * eq81_e2216_q) + (s.v[379] * s.dn[525][5]));
        let eq81_e2217_q_d_n6: f64 = ((s.dn[379][6] * eq81_e2216_q) + (s.v[379] * s.dn[525][6]));
        let eq81_e2217_q_d_n7: f64 = ((s.dn[379][7] * eq81_e2216_q) + (s.v[379] * s.dn[525][7]));
        let eq81_e2217_q_d_n8: f64 = ((s.dn[379][8] * eq81_e2216_q) + (s.v[379] * s.dn[525][8]));
        let eq81_e2217_q_d_n9: f64 = ((s.dn[379][9] * eq81_e2216_q) + (s.v[379] * s.dn[525][9]));
        let eq81_e2217_q_d_n10: f64 = ((s.dn[379][10] * eq81_e2216_q) + (s.v[379] * s.dn[525][10]));
        let eq81_e2217_q_d_n11: f64 = ((s.dn[379][11] * eq81_e2216_q) + (s.v[379] * s.dn[525][11]));
        let eq81_e2217_q_d_n12: f64 = ((s.dn[379][12] * eq81_e2216_q) + (s.v[379] * s.dn[525][12]));
        let eq81_e2217_q_d_n13: f64 = ((s.dn[379][13] * eq81_e2216_q) + (s.v[379] * s.dn[525][13]));
        let eq81_e2217_q_d_b0: f64 = ((s.db[379][0] * eq81_e2216_q) + (s.v[379] * s.db[525][0]));
        let eq81_e2217_q_d_b1: f64 = ((s.db[379][1] * eq81_e2216_q) + (s.v[379] * s.db[525][1]));
        let eq81_e2217_q_d_b2: f64 = ((s.db[379][2] * eq81_e2216_q) + (s.v[379] * s.db[525][2]));
        let eq81_e2217_q_d_b3: f64 = ((s.db[379][3] * eq81_e2216_q) + (s.v[379] * s.db[525][3]));
        let eq81_e2217_q_d_b4: f64 = ((s.db[379][4] * eq81_e2216_q) + (s.v[379] * s.db[525][4]));
        let eq81_e2217_q_d_b5: f64 = ((s.db[379][5] * eq81_e2216_q) + (s.v[379] * s.db[525][5]));
        let eq81_e2217_q_d_b6: f64 = ((s.db[379][6] * eq81_e2216_q) + (s.v[379] * s.db[525][6]));
        let eq81_e2217_q_d_b7: f64 = ((s.db[379][7] * eq81_e2216_q) + (s.v[379] * s.db[525][7]));
        let eq81_e2217_q_d_b8: f64 = ((s.db[379][8] * eq81_e2216_q) + (s.v[379] * s.db[525][8]));
        let eq81_e2217_q_d_b9: f64 = ((s.db[379][9] * eq81_e2216_q) + (s.v[379] * s.db[525][9]));
        let eq81_e2217_q_d_b10: f64 = ((s.db[379][10] * eq81_e2216_q) + (s.v[379] * s.db[525][10]));
        let eq81_e2217_q_d_b11: f64 = ((s.db[379][11] * eq81_e2216_q) + (s.v[379] * s.db[525][11]));
        let eq81_reactive_node_derivatives: [f64; 14] = [eq81_e2217_q_d_n0, eq81_e2217_q_d_n1, eq81_e2217_q_d_n2, eq81_e2217_q_d_n3, eq81_e2217_q_d_n4, eq81_e2217_q_d_n5, eq81_e2217_q_d_n6, eq81_e2217_q_d_n7, eq81_e2217_q_d_n8, eq81_e2217_q_d_n9, eq81_e2217_q_d_n10, eq81_e2217_q_d_n11, eq81_e2217_q_d_n12, eq81_e2217_q_d_n13];
        let eq81_reactive_branch_derivatives: [f64; 12] = [eq81_e2217_q_d_b0, eq81_e2217_q_d_b1, eq81_e2217_q_d_b2, eq81_e2217_q_d_b3, eq81_e2217_q_d_b4, eq81_e2217_q_d_b5, eq81_e2217_q_d_b6, eq81_e2217_q_d_b7, eq81_e2217_q_d_b8, eq81_e2217_q_d_b9, eq81_e2217_q_d_b10, eq81_e2217_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq81_reactive_node_derivatives,
            branches,
            &eq81_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
