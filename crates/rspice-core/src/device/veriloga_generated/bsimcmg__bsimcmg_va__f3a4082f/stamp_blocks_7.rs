#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_55_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq55_e2419, eq55_e2419_d_n0, eq55_e2419_d_n1, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n12, eq55_e2419_d_n13, eq55_e2419_d_n14, eq55_e2419_d_n15, eq55_e2419_d_n16,) = {
    if (s.v[1710] != 0.0) {
        let eq55_e2416: f64 = self.eval_ddt(21, s.v[495]);
        let eq55_e2416_d_n0: f64 = self.ddt_jacobian(s.dn[495][0]);
        let eq55_e2416_d_n1: f64 = self.ddt_jacobian(s.dn[495][1]);
        let eq55_e2416_d_n2: f64 = self.ddt_jacobian(s.dn[495][2]);
        let eq55_e2416_d_n3: f64 = self.ddt_jacobian(s.dn[495][3]);
        let eq55_e2416_d_n4: f64 = self.ddt_jacobian(s.dn[495][4]);
        let eq55_e2416_d_n5: f64 = self.ddt_jacobian(s.dn[495][5]);
        let eq55_e2416_d_n6: f64 = self.ddt_jacobian(s.dn[495][6]);
        let eq55_e2416_d_n7: f64 = self.ddt_jacobian(s.dn[495][7]);
        let eq55_e2416_d_n8: f64 = self.ddt_jacobian(s.dn[495][8]);
        let eq55_e2416_d_n9: f64 = self.ddt_jacobian(s.dn[495][9]);
        let eq55_e2416_d_n10: f64 = self.ddt_jacobian(s.dn[495][10]);
        let eq55_e2416_d_n11: f64 = self.ddt_jacobian(s.dn[495][11]);
        let eq55_e2416_d_n12: f64 = self.ddt_jacobian(s.dn[495][12]);
        let eq55_e2416_d_n13: f64 = self.ddt_jacobian(s.dn[495][13]);
        let eq55_e2416_d_n14: f64 = self.ddt_jacobian(s.dn[495][14]);
        let eq55_e2416_d_n15: f64 = self.ddt_jacobian(s.dn[495][15]);
        let eq55_e2416_d_n16: f64 = self.ddt_jacobian(s.dn[495][16]);
        let eq55_e2417: f64 = (s.v[114] * eq55_e2416);
        let eq55_e2417_d_n0: f64 = ((s.dn[114][0] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n0));
        let eq55_e2417_d_n1: f64 = ((s.dn[114][1] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n1));
        let eq55_e2417_d_n2: f64 = ((s.dn[114][2] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n2));
        let eq55_e2417_d_n3: f64 = ((s.dn[114][3] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n3));
        let eq55_e2417_d_n4: f64 = ((s.dn[114][4] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n4));
        let eq55_e2417_d_n5: f64 = ((s.dn[114][5] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n5));
        let eq55_e2417_d_n6: f64 = ((s.dn[114][6] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n6));
        let eq55_e2417_d_n7: f64 = ((s.dn[114][7] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n7));
        let eq55_e2417_d_n8: f64 = ((s.dn[114][8] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n8));
        let eq55_e2417_d_n9: f64 = ((s.dn[114][9] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n9));
        let eq55_e2417_d_n10: f64 = ((s.dn[114][10] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n10));
        let eq55_e2417_d_n11: f64 = ((s.dn[114][11] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n11));
        let eq55_e2417_d_n12: f64 = ((s.dn[114][12] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n12));
        let eq55_e2417_d_n13: f64 = ((s.dn[114][13] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n13));
        let eq55_e2417_d_n14: f64 = ((s.dn[114][14] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n14));
        let eq55_e2417_d_n15: f64 = ((s.dn[114][15] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n15));
        let eq55_e2417_d_n16: f64 = ((s.dn[114][16] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n16));
        (eq55_e2417, eq55_e2417_d_n0, eq55_e2417_d_n1, eq55_e2417_d_n2, eq55_e2417_d_n3, eq55_e2417_d_n4, eq55_e2417_d_n5, eq55_e2417_d_n6, eq55_e2417_d_n7, eq55_e2417_d_n8, eq55_e2417_d_n9, eq55_e2417_d_n10, eq55_e2417_d_n11, eq55_e2417_d_n12, eq55_e2417_d_n13, eq55_e2417_d_n14, eq55_e2417_d_n15, eq55_e2417_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2419;
        let eq55_node_derivatives: [f64; 17] = [eq55_e2419_d_n0, eq55_e2419_d_n1, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n12, eq55_e2419_d_n13, eq55_e2419_d_n14, eq55_e2419_d_n15, eq55_e2419_d_n16];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq55_value),
            &nodes,
            &eq55_node_derivatives,
            &branches,
            &eq55_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_56_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq56_e2426, eq56_e2426_d_n0, eq56_e2426_d_n1, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n12, eq56_e2426_d_n13, eq56_e2426_d_n14, eq56_e2426_d_n15, eq56_e2426_d_n16,) = {
    if (s.v[1710] != 0.0) {
        let eq56_e2423: f64 = self.eval_ddt(22, s.v[496]);
        let eq56_e2423_d_n0: f64 = self.ddt_jacobian(s.dn[496][0]);
        let eq56_e2423_d_n1: f64 = self.ddt_jacobian(s.dn[496][1]);
        let eq56_e2423_d_n2: f64 = self.ddt_jacobian(s.dn[496][2]);
        let eq56_e2423_d_n3: f64 = self.ddt_jacobian(s.dn[496][3]);
        let eq56_e2423_d_n4: f64 = self.ddt_jacobian(s.dn[496][4]);
        let eq56_e2423_d_n5: f64 = self.ddt_jacobian(s.dn[496][5]);
        let eq56_e2423_d_n6: f64 = self.ddt_jacobian(s.dn[496][6]);
        let eq56_e2423_d_n7: f64 = self.ddt_jacobian(s.dn[496][7]);
        let eq56_e2423_d_n8: f64 = self.ddt_jacobian(s.dn[496][8]);
        let eq56_e2423_d_n9: f64 = self.ddt_jacobian(s.dn[496][9]);
        let eq56_e2423_d_n10: f64 = self.ddt_jacobian(s.dn[496][10]);
        let eq56_e2423_d_n11: f64 = self.ddt_jacobian(s.dn[496][11]);
        let eq56_e2423_d_n12: f64 = self.ddt_jacobian(s.dn[496][12]);
        let eq56_e2423_d_n13: f64 = self.ddt_jacobian(s.dn[496][13]);
        let eq56_e2423_d_n14: f64 = self.ddt_jacobian(s.dn[496][14]);
        let eq56_e2423_d_n15: f64 = self.ddt_jacobian(s.dn[496][15]);
        let eq56_e2423_d_n16: f64 = self.ddt_jacobian(s.dn[496][16]);
        let eq56_e2424: f64 = (s.v[114] * eq56_e2423);
        let eq56_e2424_d_n0: f64 = ((s.dn[114][0] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n0));
        let eq56_e2424_d_n1: f64 = ((s.dn[114][1] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n1));
        let eq56_e2424_d_n2: f64 = ((s.dn[114][2] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n2));
        let eq56_e2424_d_n3: f64 = ((s.dn[114][3] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n3));
        let eq56_e2424_d_n4: f64 = ((s.dn[114][4] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n4));
        let eq56_e2424_d_n5: f64 = ((s.dn[114][5] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n5));
        let eq56_e2424_d_n6: f64 = ((s.dn[114][6] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n6));
        let eq56_e2424_d_n7: f64 = ((s.dn[114][7] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n7));
        let eq56_e2424_d_n8: f64 = ((s.dn[114][8] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n8));
        let eq56_e2424_d_n9: f64 = ((s.dn[114][9] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n9));
        let eq56_e2424_d_n10: f64 = ((s.dn[114][10] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n10));
        let eq56_e2424_d_n11: f64 = ((s.dn[114][11] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n11));
        let eq56_e2424_d_n12: f64 = ((s.dn[114][12] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n12));
        let eq56_e2424_d_n13: f64 = ((s.dn[114][13] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n13));
        let eq56_e2424_d_n14: f64 = ((s.dn[114][14] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n14));
        let eq56_e2424_d_n15: f64 = ((s.dn[114][15] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n15));
        let eq56_e2424_d_n16: f64 = ((s.dn[114][16] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n16));
        (eq56_e2424, eq56_e2424_d_n0, eq56_e2424_d_n1, eq56_e2424_d_n2, eq56_e2424_d_n3, eq56_e2424_d_n4, eq56_e2424_d_n5, eq56_e2424_d_n6, eq56_e2424_d_n7, eq56_e2424_d_n8, eq56_e2424_d_n9, eq56_e2424_d_n10, eq56_e2424_d_n11, eq56_e2424_d_n12, eq56_e2424_d_n13, eq56_e2424_d_n14, eq56_e2424_d_n15, eq56_e2424_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e2426;
        let eq56_node_derivatives: [f64; 17] = [eq56_e2426_d_n0, eq56_e2426_d_n1, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n12, eq56_e2426_d_n13, eq56_e2426_d_n14, eq56_e2426_d_n15, eq56_e2426_d_n16];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            self.multiplicity * (eq56_value),
            &nodes,
            &eq56_node_derivatives,
            &branches,
            &eq56_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_57_block_0(
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq57_e2432, eq57_e2432_d_n0, eq57_e2432_d_n1, eq57_e2432_d_n2, eq57_e2432_d_n3, eq57_e2432_d_n4, eq57_e2432_d_n5, eq57_e2432_d_n6, eq57_e2432_d_n7, eq57_e2432_d_n8, eq57_e2432_d_n9, eq57_e2432_d_n10, eq57_e2432_d_n11, eq57_e2432_d_n12, eq57_e2432_d_n13, eq57_e2432_d_n14, eq57_e2432_d_n15, eq57_e2432_d_n16,) = {
    if (s.v[1718] != 0.0) {
        let eq57_e2430: f64 = ((nv0 - nv9) * s.v[596]);
        let eq57_e2430_d_n0: f64 = (s.v[596] + ((nv0 - nv9) * s.dn[596][0]));
        let eq57_e2430_d_n1: f64 = ((nv0 - nv9) * s.dn[596][1]);
        let eq57_e2430_d_n2: f64 = ((nv0 - nv9) * s.dn[596][2]);
        let eq57_e2430_d_n3: f64 = ((nv0 - nv9) * s.dn[596][3]);
        let eq57_e2430_d_n4: f64 = ((nv0 - nv9) * s.dn[596][4]);
        let eq57_e2430_d_n5: f64 = ((nv0 - nv9) * s.dn[596][5]);
        let eq57_e2430_d_n6: f64 = ((nv0 - nv9) * s.dn[596][6]);
        let eq57_e2430_d_n7: f64 = ((nv0 - nv9) * s.dn[596][7]);
        let eq57_e2430_d_n8: f64 = ((nv0 - nv9) * s.dn[596][8]);
        let eq57_e2430_d_n9: f64 = ((-s.v[596]) + ((nv0 - nv9) * s.dn[596][9]));
        let eq57_e2430_d_n10: f64 = ((nv0 - nv9) * s.dn[596][10]);
        let eq57_e2430_d_n11: f64 = ((nv0 - nv9) * s.dn[596][11]);
        let eq57_e2430_d_n12: f64 = ((nv0 - nv9) * s.dn[596][12]);
        let eq57_e2430_d_n13: f64 = ((nv0 - nv9) * s.dn[596][13]);
        let eq57_e2430_d_n14: f64 = ((nv0 - nv9) * s.dn[596][14]);
        let eq57_e2430_d_n15: f64 = ((nv0 - nv9) * s.dn[596][15]);
        let eq57_e2430_d_n16: f64 = ((nv0 - nv9) * s.dn[596][16]);
        (eq57_e2430, eq57_e2430_d_n0, eq57_e2430_d_n1, eq57_e2430_d_n2, eq57_e2430_d_n3, eq57_e2430_d_n4, eq57_e2430_d_n5, eq57_e2430_d_n6, eq57_e2430_d_n7, eq57_e2430_d_n8, eq57_e2430_d_n9, eq57_e2430_d_n10, eq57_e2430_d_n11, eq57_e2430_d_n12, eq57_e2430_d_n13, eq57_e2430_d_n14, eq57_e2430_d_n15, eq57_e2430_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2432;
        let eq57_node_derivatives: [f64; 17] = [eq57_e2432_d_n0, eq57_e2432_d_n1, eq57_e2432_d_n2, eq57_e2432_d_n3, eq57_e2432_d_n4, eq57_e2432_d_n5, eq57_e2432_d_n6, eq57_e2432_d_n7, eq57_e2432_d_n8, eq57_e2432_d_n9, eq57_e2432_d_n10, eq57_e2432_d_n11, eq57_e2432_d_n12, eq57_e2432_d_n13, eq57_e2432_d_n14, eq57_e2432_d_n15, eq57_e2432_d_n16];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[9]),
            self.multiplicity * (eq57_value),
            &nodes,
            &eq57_node_derivatives,
            &branches,
            &eq57_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_58_block_0(
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq58_e2440, eq58_e2440_d_n0, eq58_e2440_d_n1, eq58_e2440_d_n2, eq58_e2440_d_n3, eq58_e2440_d_n4, eq58_e2440_d_n5, eq58_e2440_d_n6, eq58_e2440_d_n7, eq58_e2440_d_n8, eq58_e2440_d_n9, eq58_e2440_d_n10, eq58_e2440_d_n11, eq58_e2440_d_n12, eq58_e2440_d_n13, eq58_e2440_d_n14, eq58_e2440_d_n15, eq58_e2440_d_n16,) = {
    if ((s.v[1718] != 0.0) && (s.v[1719] != 0.0)) {
        let eq58_e2438: f64 = ((nv9 - nv7) * s.v[1042]);
        let eq58_e2438_d_n0: f64 = ((nv9 - nv7) * s.dn[1042][0]);
        let eq58_e2438_d_n1: f64 = ((nv9 - nv7) * s.dn[1042][1]);
        let eq58_e2438_d_n2: f64 = ((nv9 - nv7) * s.dn[1042][2]);
        let eq58_e2438_d_n3: f64 = ((nv9 - nv7) * s.dn[1042][3]);
        let eq58_e2438_d_n4: f64 = ((nv9 - nv7) * s.dn[1042][4]);
        let eq58_e2438_d_n5: f64 = ((nv9 - nv7) * s.dn[1042][5]);
        let eq58_e2438_d_n6: f64 = ((nv9 - nv7) * s.dn[1042][6]);
        let eq58_e2438_d_n7: f64 = ((-s.v[1042]) + ((nv9 - nv7) * s.dn[1042][7]));
        let eq58_e2438_d_n8: f64 = ((nv9 - nv7) * s.dn[1042][8]);
        let eq58_e2438_d_n9: f64 = (s.v[1042] + ((nv9 - nv7) * s.dn[1042][9]));
        let eq58_e2438_d_n10: f64 = ((nv9 - nv7) * s.dn[1042][10]);
        let eq58_e2438_d_n11: f64 = ((nv9 - nv7) * s.dn[1042][11]);
        let eq58_e2438_d_n12: f64 = ((nv9 - nv7) * s.dn[1042][12]);
        let eq58_e2438_d_n13: f64 = ((nv9 - nv7) * s.dn[1042][13]);
        let eq58_e2438_d_n14: f64 = ((nv9 - nv7) * s.dn[1042][14]);
        let eq58_e2438_d_n15: f64 = ((nv9 - nv7) * s.dn[1042][15]);
        let eq58_e2438_d_n16: f64 = ((nv9 - nv7) * s.dn[1042][16]);
        (eq58_e2438, eq58_e2438_d_n0, eq58_e2438_d_n1, eq58_e2438_d_n2, eq58_e2438_d_n3, eq58_e2438_d_n4, eq58_e2438_d_n5, eq58_e2438_d_n6, eq58_e2438_d_n7, eq58_e2438_d_n8, eq58_e2438_d_n9, eq58_e2438_d_n10, eq58_e2438_d_n11, eq58_e2438_d_n12, eq58_e2438_d_n13, eq58_e2438_d_n14, eq58_e2438_d_n15, eq58_e2438_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e2440;
        let eq58_node_derivatives: [f64; 17] = [eq58_e2440_d_n0, eq58_e2440_d_n1, eq58_e2440_d_n2, eq58_e2440_d_n3, eq58_e2440_d_n4, eq58_e2440_d_n5, eq58_e2440_d_n6, eq58_e2440_d_n7, eq58_e2440_d_n8, eq58_e2440_d_n9, eq58_e2440_d_n10, eq58_e2440_d_n11, eq58_e2440_d_n12, eq58_e2440_d_n13, eq58_e2440_d_n14, eq58_e2440_d_n15, eq58_e2440_d_n16];
        let eq58_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq58_value),
            &nodes,
            &eq58_node_derivatives,
            &branches,
            &eq58_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_59_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq59_e2447,) = {
    if ((s.v[1718] != 0.0) && (!(s.v[1719] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e2447;
        stamper.stamp_potential(
            branches[1],
            eq59_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_60_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq60_e2452,) = {
    if (!(s.v[1718] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2452;
        stamper.stamp_potential(
            branches[2],
            eq60_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_61_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq61_e2457,) = {
    if (!(s.v[1718] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e2457;
        stamper.stamp_potential(
            branches[3],
            eq61_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_62_block_0(
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
        let (eq62_e2463, eq62_e2463_d_n0, eq62_e2463_d_n1, eq62_e2463_d_n2, eq62_e2463_d_n3, eq62_e2463_d_n4, eq62_e2463_d_n5, eq62_e2463_d_n6, eq62_e2463_d_n7, eq62_e2463_d_n8, eq62_e2463_d_n9, eq62_e2463_d_n10, eq62_e2463_d_n11, eq62_e2463_d_n12, eq62_e2463_d_n13, eq62_e2463_d_n14, eq62_e2463_d_n15, eq62_e2463_d_n16,) = {
    if (s.v[1720] != 0.0) {
        let eq62_e2461: f64 = ((nv2 - nv8) * s.v[595]);
        let eq62_e2461_d_n0: f64 = ((nv2 - nv8) * s.dn[595][0]);
        let eq62_e2461_d_n1: f64 = ((nv2 - nv8) * s.dn[595][1]);
        let eq62_e2461_d_n2: f64 = (s.v[595] + ((nv2 - nv8) * s.dn[595][2]));
        let eq62_e2461_d_n3: f64 = ((nv2 - nv8) * s.dn[595][3]);
        let eq62_e2461_d_n4: f64 = ((nv2 - nv8) * s.dn[595][4]);
        let eq62_e2461_d_n5: f64 = ((nv2 - nv8) * s.dn[595][5]);
        let eq62_e2461_d_n6: f64 = ((nv2 - nv8) * s.dn[595][6]);
        let eq62_e2461_d_n7: f64 = ((nv2 - nv8) * s.dn[595][7]);
        let eq62_e2461_d_n8: f64 = ((-s.v[595]) + ((nv2 - nv8) * s.dn[595][8]));
        let eq62_e2461_d_n9: f64 = ((nv2 - nv8) * s.dn[595][9]);
        let eq62_e2461_d_n10: f64 = ((nv2 - nv8) * s.dn[595][10]);
        let eq62_e2461_d_n11: f64 = ((nv2 - nv8) * s.dn[595][11]);
        let eq62_e2461_d_n12: f64 = ((nv2 - nv8) * s.dn[595][12]);
        let eq62_e2461_d_n13: f64 = ((nv2 - nv8) * s.dn[595][13]);
        let eq62_e2461_d_n14: f64 = ((nv2 - nv8) * s.dn[595][14]);
        let eq62_e2461_d_n15: f64 = ((nv2 - nv8) * s.dn[595][15]);
        let eq62_e2461_d_n16: f64 = ((nv2 - nv8) * s.dn[595][16]);
        (eq62_e2461, eq62_e2461_d_n0, eq62_e2461_d_n1, eq62_e2461_d_n2, eq62_e2461_d_n3, eq62_e2461_d_n4, eq62_e2461_d_n5, eq62_e2461_d_n6, eq62_e2461_d_n7, eq62_e2461_d_n8, eq62_e2461_d_n9, eq62_e2461_d_n10, eq62_e2461_d_n11, eq62_e2461_d_n12, eq62_e2461_d_n13, eq62_e2461_d_n14, eq62_e2461_d_n15, eq62_e2461_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2463;
        let eq62_node_derivatives: [f64; 17] = [eq62_e2463_d_n0, eq62_e2463_d_n1, eq62_e2463_d_n2, eq62_e2463_d_n3, eq62_e2463_d_n4, eq62_e2463_d_n5, eq62_e2463_d_n6, eq62_e2463_d_n7, eq62_e2463_d_n8, eq62_e2463_d_n9, eq62_e2463_d_n10, eq62_e2463_d_n11, eq62_e2463_d_n12, eq62_e2463_d_n13, eq62_e2463_d_n14, eq62_e2463_d_n15, eq62_e2463_d_n16];
        let eq62_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq62_value),
            &nodes,
            &eq62_node_derivatives,
            &branches,
            &eq62_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_63_block_0(
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq63_e2471, eq63_e2471_d_n0, eq63_e2471_d_n1, eq63_e2471_d_n2, eq63_e2471_d_n3, eq63_e2471_d_n4, eq63_e2471_d_n5, eq63_e2471_d_n6, eq63_e2471_d_n7, eq63_e2471_d_n8, eq63_e2471_d_n9, eq63_e2471_d_n10, eq63_e2471_d_n11, eq63_e2471_d_n12, eq63_e2471_d_n13, eq63_e2471_d_n14, eq63_e2471_d_n15, eq63_e2471_d_n16,) = {
    if ((s.v[1720] != 0.0) && (s.v[1721] != 0.0)) {
        let eq63_e2469: f64 = ((nv8 - nv6) * s.v[1043]);
        let eq63_e2469_d_n0: f64 = ((nv8 - nv6) * s.dn[1043][0]);
        let eq63_e2469_d_n1: f64 = ((nv8 - nv6) * s.dn[1043][1]);
        let eq63_e2469_d_n2: f64 = ((nv8 - nv6) * s.dn[1043][2]);
        let eq63_e2469_d_n3: f64 = ((nv8 - nv6) * s.dn[1043][3]);
        let eq63_e2469_d_n4: f64 = ((nv8 - nv6) * s.dn[1043][4]);
        let eq63_e2469_d_n5: f64 = ((nv8 - nv6) * s.dn[1043][5]);
        let eq63_e2469_d_n6: f64 = ((-s.v[1043]) + ((nv8 - nv6) * s.dn[1043][6]));
        let eq63_e2469_d_n7: f64 = ((nv8 - nv6) * s.dn[1043][7]);
        let eq63_e2469_d_n8: f64 = (s.v[1043] + ((nv8 - nv6) * s.dn[1043][8]));
        let eq63_e2469_d_n9: f64 = ((nv8 - nv6) * s.dn[1043][9]);
        let eq63_e2469_d_n10: f64 = ((nv8 - nv6) * s.dn[1043][10]);
        let eq63_e2469_d_n11: f64 = ((nv8 - nv6) * s.dn[1043][11]);
        let eq63_e2469_d_n12: f64 = ((nv8 - nv6) * s.dn[1043][12]);
        let eq63_e2469_d_n13: f64 = ((nv8 - nv6) * s.dn[1043][13]);
        let eq63_e2469_d_n14: f64 = ((nv8 - nv6) * s.dn[1043][14]);
        let eq63_e2469_d_n15: f64 = ((nv8 - nv6) * s.dn[1043][15]);
        let eq63_e2469_d_n16: f64 = ((nv8 - nv6) * s.dn[1043][16]);
        (eq63_e2469, eq63_e2469_d_n0, eq63_e2469_d_n1, eq63_e2469_d_n2, eq63_e2469_d_n3, eq63_e2469_d_n4, eq63_e2469_d_n5, eq63_e2469_d_n6, eq63_e2469_d_n7, eq63_e2469_d_n8, eq63_e2469_d_n9, eq63_e2469_d_n10, eq63_e2469_d_n11, eq63_e2469_d_n12, eq63_e2469_d_n13, eq63_e2469_d_n14, eq63_e2469_d_n15, eq63_e2469_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e2471;
        let eq63_node_derivatives: [f64; 17] = [eq63_e2471_d_n0, eq63_e2471_d_n1, eq63_e2471_d_n2, eq63_e2471_d_n3, eq63_e2471_d_n4, eq63_e2471_d_n5, eq63_e2471_d_n6, eq63_e2471_d_n7, eq63_e2471_d_n8, eq63_e2471_d_n9, eq63_e2471_d_n10, eq63_e2471_d_n11, eq63_e2471_d_n12, eq63_e2471_d_n13, eq63_e2471_d_n14, eq63_e2471_d_n15, eq63_e2471_d_n16];
        let eq63_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq63_value),
            &nodes,
            &eq63_node_derivatives,
            &branches,
            &eq63_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_64_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq64_e2478,) = {
    if ((s.v[1720] != 0.0) && (!(s.v[1721] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e2478;
        stamper.stamp_potential(
            branches[4],
            eq64_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_65_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq65_e2483,) = {
    if (!(s.v[1720] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e2483;
        stamper.stamp_potential(
            branches[5],
            eq65_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_66_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq66_e2488,) = {
    if (!(s.v[1720] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e2488;
        stamper.stamp_potential(
            branches[6],
            eq66_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_67_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq67_e2494, eq67_e2494_d_n0, eq67_e2494_d_n1, eq67_e2494_d_n2, eq67_e2494_d_n3, eq67_e2494_d_n4, eq67_e2494_d_n5, eq67_e2494_d_n6, eq67_e2494_d_n7, eq67_e2494_d_n8, eq67_e2494_d_n9, eq67_e2494_d_n10, eq67_e2494_d_n11, eq67_e2494_d_n12, eq67_e2494_d_n13, eq67_e2494_d_n14, eq67_e2494_d_n15, eq67_e2494_d_n16,) = {
    if (s.v[1722] != 0.0) {
        let eq67_e2492: f64 = ((nv12 - nv11) * s.v[569]);
        let eq67_e2492_d_n0: f64 = ((nv12 - nv11) * s.dn[569][0]);
        let eq67_e2492_d_n1: f64 = ((nv12 - nv11) * s.dn[569][1]);
        let eq67_e2492_d_n2: f64 = ((nv12 - nv11) * s.dn[569][2]);
        let eq67_e2492_d_n3: f64 = ((nv12 - nv11) * s.dn[569][3]);
        let eq67_e2492_d_n4: f64 = ((nv12 - nv11) * s.dn[569][4]);
        let eq67_e2492_d_n5: f64 = ((nv12 - nv11) * s.dn[569][5]);
        let eq67_e2492_d_n6: f64 = ((nv12 - nv11) * s.dn[569][6]);
        let eq67_e2492_d_n7: f64 = ((nv12 - nv11) * s.dn[569][7]);
        let eq67_e2492_d_n8: f64 = ((nv12 - nv11) * s.dn[569][8]);
        let eq67_e2492_d_n9: f64 = ((nv12 - nv11) * s.dn[569][9]);
        let eq67_e2492_d_n10: f64 = ((nv12 - nv11) * s.dn[569][10]);
        let eq67_e2492_d_n11: f64 = ((-s.v[569]) + ((nv12 - nv11) * s.dn[569][11]));
        let eq67_e2492_d_n12: f64 = (s.v[569] + ((nv12 - nv11) * s.dn[569][12]));
        let eq67_e2492_d_n13: f64 = ((nv12 - nv11) * s.dn[569][13]);
        let eq67_e2492_d_n14: f64 = ((nv12 - nv11) * s.dn[569][14]);
        let eq67_e2492_d_n15: f64 = ((nv12 - nv11) * s.dn[569][15]);
        let eq67_e2492_d_n16: f64 = ((nv12 - nv11) * s.dn[569][16]);
        (eq67_e2492, eq67_e2492_d_n0, eq67_e2492_d_n1, eq67_e2492_d_n2, eq67_e2492_d_n3, eq67_e2492_d_n4, eq67_e2492_d_n5, eq67_e2492_d_n6, eq67_e2492_d_n7, eq67_e2492_d_n8, eq67_e2492_d_n9, eq67_e2492_d_n10, eq67_e2492_d_n11, eq67_e2492_d_n12, eq67_e2492_d_n13, eq67_e2492_d_n14, eq67_e2492_d_n15, eq67_e2492_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2494;
        let eq67_node_derivatives: [f64; 17] = [eq67_e2494_d_n0, eq67_e2494_d_n1, eq67_e2494_d_n2, eq67_e2494_d_n3, eq67_e2494_d_n4, eq67_e2494_d_n5, eq67_e2494_d_n6, eq67_e2494_d_n7, eq67_e2494_d_n8, eq67_e2494_d_n9, eq67_e2494_d_n10, eq67_e2494_d_n11, eq67_e2494_d_n12, eq67_e2494_d_n13, eq67_e2494_d_n14, eq67_e2494_d_n15, eq67_e2494_d_n16];
        let eq67_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[11]),
            self.multiplicity * (eq67_value),
            &nodes,
            &eq67_node_derivatives,
            &branches,
            &eq67_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_68_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq68_e2499,) = {
    if (!(s.v[1722] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e2499;
        stamper.stamp_potential(
            branches[7],
            eq68_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_69_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq69_e2506, eq69_e2506_d_n0, eq69_e2506_d_n1, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n12, eq69_e2506_d_n13, eq69_e2506_d_n14, eq69_e2506_d_n15, eq69_e2506_d_n16,) = {
    if (s.v[1723] != 0.0) {
        let eq69_e2503: f64 = (s.v[138] - s.v[140]);
        let eq69_e2503_d_n0: f64 = (s.dn[138][0] - s.dn[140][0]);
        let eq69_e2503_d_n1: f64 = (s.dn[138][1] - s.dn[140][1]);
        let eq69_e2503_d_n2: f64 = (s.dn[138][2] - s.dn[140][2]);
        let eq69_e2503_d_n3: f64 = (s.dn[138][3] - s.dn[140][3]);
        let eq69_e2503_d_n4: f64 = (s.dn[138][4] - s.dn[140][4]);
        let eq69_e2503_d_n5: f64 = (s.dn[138][5] - s.dn[140][5]);
        let eq69_e2503_d_n6: f64 = (s.dn[138][6] - s.dn[140][6]);
        let eq69_e2503_d_n7: f64 = (s.dn[138][7] - s.dn[140][7]);
        let eq69_e2503_d_n8: f64 = (s.dn[138][8] - s.dn[140][8]);
        let eq69_e2503_d_n9: f64 = (s.dn[138][9] - s.dn[140][9]);
        let eq69_e2503_d_n10: f64 = (s.dn[138][10] - s.dn[140][10]);
        let eq69_e2503_d_n11: f64 = (s.dn[138][11] - s.dn[140][11]);
        let eq69_e2503_d_n12: f64 = (s.dn[138][12] - s.dn[140][12]);
        let eq69_e2503_d_n13: f64 = (s.dn[138][13] - s.dn[140][13]);
        let eq69_e2503_d_n14: f64 = (s.dn[138][14] - s.dn[140][14]);
        let eq69_e2503_d_n15: f64 = (s.dn[138][15] - s.dn[140][15]);
        let eq69_e2503_d_n16: f64 = (s.dn[138][16] - s.dn[140][16]);
        let eq69_e2504: f64 = self.eval_ddt(23, eq69_e2503);
        let eq69_e2504_d_n0: f64 = self.ddt_jacobian(eq69_e2503_d_n0);
        let eq69_e2504_d_n1: f64 = self.ddt_jacobian(eq69_e2503_d_n1);
        let eq69_e2504_d_n2: f64 = self.ddt_jacobian(eq69_e2503_d_n2);
        let eq69_e2504_d_n3: f64 = self.ddt_jacobian(eq69_e2503_d_n3);
        let eq69_e2504_d_n4: f64 = self.ddt_jacobian(eq69_e2503_d_n4);
        let eq69_e2504_d_n5: f64 = self.ddt_jacobian(eq69_e2503_d_n5);
        let eq69_e2504_d_n6: f64 = self.ddt_jacobian(eq69_e2503_d_n6);
        let eq69_e2504_d_n7: f64 = self.ddt_jacobian(eq69_e2503_d_n7);
        let eq69_e2504_d_n8: f64 = self.ddt_jacobian(eq69_e2503_d_n8);
        let eq69_e2504_d_n9: f64 = self.ddt_jacobian(eq69_e2503_d_n9);
        let eq69_e2504_d_n10: f64 = self.ddt_jacobian(eq69_e2503_d_n10);
        let eq69_e2504_d_n11: f64 = self.ddt_jacobian(eq69_e2503_d_n11);
        let eq69_e2504_d_n12: f64 = self.ddt_jacobian(eq69_e2503_d_n12);
        let eq69_e2504_d_n13: f64 = self.ddt_jacobian(eq69_e2503_d_n13);
        let eq69_e2504_d_n14: f64 = self.ddt_jacobian(eq69_e2503_d_n14);
        let eq69_e2504_d_n15: f64 = self.ddt_jacobian(eq69_e2503_d_n15);
        let eq69_e2504_d_n16: f64 = self.ddt_jacobian(eq69_e2503_d_n16);
        (eq69_e2504, eq69_e2504_d_n0, eq69_e2504_d_n1, eq69_e2504_d_n2, eq69_e2504_d_n3, eq69_e2504_d_n4, eq69_e2504_d_n5, eq69_e2504_d_n6, eq69_e2504_d_n7, eq69_e2504_d_n8, eq69_e2504_d_n9, eq69_e2504_d_n10, eq69_e2504_d_n11, eq69_e2504_d_n12, eq69_e2504_d_n13, eq69_e2504_d_n14, eq69_e2504_d_n15, eq69_e2504_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2506;
        let eq69_node_derivatives: [f64; 17] = [eq69_e2506_d_n0, eq69_e2506_d_n1, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n12, eq69_e2506_d_n13, eq69_e2506_d_n14, eq69_e2506_d_n15, eq69_e2506_d_n16];
        let eq69_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            None,
            self.multiplicity * (eq69_value),
            &nodes,
            &eq69_node_derivatives,
            &branches,
            &eq69_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq70_e2512, eq70_e2512_d_n0, eq70_e2512_d_n1, eq70_e2512_d_n2, eq70_e2512_d_n3, eq70_e2512_d_n4, eq70_e2512_d_n5, eq70_e2512_d_n6, eq70_e2512_d_n7, eq70_e2512_d_n8, eq70_e2512_d_n9, eq70_e2512_d_n10, eq70_e2512_d_n11, eq70_e2512_d_n12, eq70_e2512_d_n13, eq70_e2512_d_n14, eq70_e2512_d_n15, eq70_e2512_d_n16,) = {
    if (s.v[1723] != 0.0) {
        let eq70_e2510: f64 = ((nv15 - 0.0) * s.v[570]);
        let eq70_e2510_d_n0: f64 = ((nv15 - 0.0) * s.dn[570][0]);
        let eq70_e2510_d_n1: f64 = ((nv15 - 0.0) * s.dn[570][1]);
        let eq70_e2510_d_n2: f64 = ((nv15 - 0.0) * s.dn[570][2]);
        let eq70_e2510_d_n3: f64 = ((nv15 - 0.0) * s.dn[570][3]);
        let eq70_e2510_d_n4: f64 = ((nv15 - 0.0) * s.dn[570][4]);
        let eq70_e2510_d_n5: f64 = ((nv15 - 0.0) * s.dn[570][5]);
        let eq70_e2510_d_n6: f64 = ((nv15 - 0.0) * s.dn[570][6]);
        let eq70_e2510_d_n7: f64 = ((nv15 - 0.0) * s.dn[570][7]);
        let eq70_e2510_d_n8: f64 = ((nv15 - 0.0) * s.dn[570][8]);
        let eq70_e2510_d_n9: f64 = ((nv15 - 0.0) * s.dn[570][9]);
        let eq70_e2510_d_n10: f64 = ((nv15 - 0.0) * s.dn[570][10]);
        let eq70_e2510_d_n11: f64 = ((nv15 - 0.0) * s.dn[570][11]);
        let eq70_e2510_d_n12: f64 = ((nv15 - 0.0) * s.dn[570][12]);
        let eq70_e2510_d_n13: f64 = ((nv15 - 0.0) * s.dn[570][13]);
        let eq70_e2510_d_n14: f64 = ((nv15 - 0.0) * s.dn[570][14]);
        let eq70_e2510_d_n15: f64 = (s.v[570] + ((nv15 - 0.0) * s.dn[570][15]));
        let eq70_e2510_d_n16: f64 = ((nv15 - 0.0) * s.dn[570][16]);
        (eq70_e2510, eq70_e2510_d_n0, eq70_e2510_d_n1, eq70_e2510_d_n2, eq70_e2510_d_n3, eq70_e2510_d_n4, eq70_e2510_d_n5, eq70_e2510_d_n6, eq70_e2510_d_n7, eq70_e2510_d_n8, eq70_e2510_d_n9, eq70_e2510_d_n10, eq70_e2510_d_n11, eq70_e2510_d_n12, eq70_e2510_d_n13, eq70_e2510_d_n14, eq70_e2510_d_n15, eq70_e2510_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e2512;
        let eq70_node_derivatives: [f64; 17] = [eq70_e2512_d_n0, eq70_e2512_d_n1, eq70_e2512_d_n2, eq70_e2512_d_n3, eq70_e2512_d_n4, eq70_e2512_d_n5, eq70_e2512_d_n6, eq70_e2512_d_n7, eq70_e2512_d_n8, eq70_e2512_d_n9, eq70_e2512_d_n10, eq70_e2512_d_n11, eq70_e2512_d_n12, eq70_e2512_d_n13, eq70_e2512_d_n14, eq70_e2512_d_n15, eq70_e2512_d_n16];
        let eq70_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            None,
            self.multiplicity * (eq70_value),
            &nodes,
            &eq70_node_derivatives,
            &branches,
            &eq70_branch_derivatives,
            self.multiplicity,
        );
    }
}
