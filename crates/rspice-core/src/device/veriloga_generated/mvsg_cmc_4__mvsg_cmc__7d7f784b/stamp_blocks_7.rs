#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq52_e932, eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29,) = {
    if (!(s.v[613] != 0.0)) {
        let eq52_e925: f64 = self.eval_ddt(30, s.v[204]);
        let eq52_e925_d_n0: f64 = self.ddt_jacobian(s.dn[204][0]);
        let eq52_e925_d_n1: f64 = self.ddt_jacobian(s.dn[204][1]);
        let eq52_e925_d_n2: f64 = self.ddt_jacobian(s.dn[204][2]);
        let eq52_e925_d_n3: f64 = self.ddt_jacobian(s.dn[204][3]);
        let eq52_e925_d_n4: f64 = self.ddt_jacobian(s.dn[204][4]);
        let eq52_e925_d_n5: f64 = self.ddt_jacobian(s.dn[204][5]);
        let eq52_e925_d_n6: f64 = self.ddt_jacobian(s.dn[204][6]);
        let eq52_e925_d_n7: f64 = self.ddt_jacobian(s.dn[204][7]);
        let eq52_e925_d_n8: f64 = self.ddt_jacobian(s.dn[204][8]);
        let eq52_e925_d_n9: f64 = self.ddt_jacobian(s.dn[204][9]);
        let eq52_e925_d_n10: f64 = self.ddt_jacobian(s.dn[204][10]);
        let eq52_e925_d_n11: f64 = self.ddt_jacobian(s.dn[204][11]);
        let eq52_e925_d_n12: f64 = self.ddt_jacobian(s.dn[204][12]);
        let eq52_e925_d_n13: f64 = self.ddt_jacobian(s.dn[204][13]);
        let eq52_e925_d_n14: f64 = self.ddt_jacobian(s.dn[204][14]);
        let eq52_e925_d_n15: f64 = self.ddt_jacobian(s.dn[204][15]);
        let eq52_e925_d_n16: f64 = self.ddt_jacobian(s.dn[204][16]);
        let eq52_e925_d_n17: f64 = self.ddt_jacobian(s.dn[204][17]);
        let eq52_e925_d_n18: f64 = self.ddt_jacobian(s.dn[204][18]);
        let eq52_e925_d_n19: f64 = self.ddt_jacobian(s.dn[204][19]);
        let eq52_e925_d_n20: f64 = self.ddt_jacobian(s.dn[204][20]);
        let eq52_e925_d_n21: f64 = self.ddt_jacobian(s.dn[204][21]);
        let eq52_e925_d_n22: f64 = self.ddt_jacobian(s.dn[204][22]);
        let eq52_e925_d_n23: f64 = self.ddt_jacobian(s.dn[204][23]);
        let eq52_e925_d_n24: f64 = self.ddt_jacobian(s.dn[204][24]);
        let eq52_e925_d_n25: f64 = self.ddt_jacobian(s.dn[204][25]);
        let eq52_e925_d_n26: f64 = self.ddt_jacobian(s.dn[204][26]);
        let eq52_e925_d_n27: f64 = self.ddt_jacobian(s.dn[204][27]);
        let eq52_e925_d_n28: f64 = self.ddt_jacobian(s.dn[204][28]);
        let eq52_e925_d_n29: f64 = self.ddt_jacobian(s.dn[204][29]);
        let eq52_e928: f64 = (p.p355 * (nv2 - nv16));
        let eq52_e928_d_n2: f64 = p.p355;
        let eq52_e928_d_n16: f64 = (-p.p355);
        let eq52_e929: f64 = self.eval_ddt(31, eq52_e928);
        let eq52_e929_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n2: f64 = self.ddt_jacobian(eq52_e928_d_n2);
        let eq52_e929_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n16: f64 = self.ddt_jacobian(eq52_e928_d_n16);
        let eq52_e929_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq52_e929_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq52_e930: f64 = (eq52_e925 + eq52_e929);
        let eq52_e930_d_n0: f64 = (eq52_e925_d_n0 + eq52_e929_d_n0);
        let eq52_e930_d_n1: f64 = (eq52_e925_d_n1 + eq52_e929_d_n1);
        let eq52_e930_d_n2: f64 = (eq52_e925_d_n2 + eq52_e929_d_n2);
        let eq52_e930_d_n3: f64 = (eq52_e925_d_n3 + eq52_e929_d_n3);
        let eq52_e930_d_n4: f64 = (eq52_e925_d_n4 + eq52_e929_d_n4);
        let eq52_e930_d_n5: f64 = (eq52_e925_d_n5 + eq52_e929_d_n5);
        let eq52_e930_d_n6: f64 = (eq52_e925_d_n6 + eq52_e929_d_n6);
        let eq52_e930_d_n7: f64 = (eq52_e925_d_n7 + eq52_e929_d_n7);
        let eq52_e930_d_n8: f64 = (eq52_e925_d_n8 + eq52_e929_d_n8);
        let eq52_e930_d_n9: f64 = (eq52_e925_d_n9 + eq52_e929_d_n9);
        let eq52_e930_d_n10: f64 = (eq52_e925_d_n10 + eq52_e929_d_n10);
        let eq52_e930_d_n11: f64 = (eq52_e925_d_n11 + eq52_e929_d_n11);
        let eq52_e930_d_n12: f64 = (eq52_e925_d_n12 + eq52_e929_d_n12);
        let eq52_e930_d_n13: f64 = (eq52_e925_d_n13 + eq52_e929_d_n13);
        let eq52_e930_d_n14: f64 = (eq52_e925_d_n14 + eq52_e929_d_n14);
        let eq52_e930_d_n15: f64 = (eq52_e925_d_n15 + eq52_e929_d_n15);
        let eq52_e930_d_n16: f64 = (eq52_e925_d_n16 + eq52_e929_d_n16);
        let eq52_e930_d_n17: f64 = (eq52_e925_d_n17 + eq52_e929_d_n17);
        let eq52_e930_d_n18: f64 = (eq52_e925_d_n18 + eq52_e929_d_n18);
        let eq52_e930_d_n19: f64 = (eq52_e925_d_n19 + eq52_e929_d_n19);
        let eq52_e930_d_n20: f64 = (eq52_e925_d_n20 + eq52_e929_d_n20);
        let eq52_e930_d_n21: f64 = (eq52_e925_d_n21 + eq52_e929_d_n21);
        let eq52_e930_d_n22: f64 = (eq52_e925_d_n22 + eq52_e929_d_n22);
        let eq52_e930_d_n23: f64 = (eq52_e925_d_n23 + eq52_e929_d_n23);
        let eq52_e930_d_n24: f64 = (eq52_e925_d_n24 + eq52_e929_d_n24);
        let eq52_e930_d_n25: f64 = (eq52_e925_d_n25 + eq52_e929_d_n25);
        let eq52_e930_d_n26: f64 = (eq52_e925_d_n26 + eq52_e929_d_n26);
        let eq52_e930_d_n27: f64 = (eq52_e925_d_n27 + eq52_e929_d_n27);
        let eq52_e930_d_n28: f64 = (eq52_e925_d_n28 + eq52_e929_d_n28);
        let eq52_e930_d_n29: f64 = (eq52_e925_d_n29 + eq52_e929_d_n29);
        (eq52_e930, eq52_e930_d_n0, eq52_e930_d_n1, eq52_e930_d_n2, eq52_e930_d_n3, eq52_e930_d_n4, eq52_e930_d_n5, eq52_e930_d_n6, eq52_e930_d_n7, eq52_e930_d_n8, eq52_e930_d_n9, eq52_e930_d_n10, eq52_e930_d_n11, eq52_e930_d_n12, eq52_e930_d_n13, eq52_e930_d_n14, eq52_e930_d_n15, eq52_e930_d_n16, eq52_e930_d_n17, eq52_e930_d_n18, eq52_e930_d_n19, eq52_e930_d_n20, eq52_e930_d_n21, eq52_e930_d_n22, eq52_e930_d_n23, eq52_e930_d_n24, eq52_e930_d_n25, eq52_e930_d_n26, eq52_e930_d_n27, eq52_e930_d_n28, eq52_e930_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e932;
        let eq52_node_derivatives: [f64; 30] = [eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29];
        let eq52_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[16]),
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq53_e943, eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29,) = {
    if (!(s.v[613] != 0.0)) {
        let eq53_e936: f64 = self.eval_ddt(32, s.v[205]);
        let eq53_e936_d_n0: f64 = self.ddt_jacobian(s.dn[205][0]);
        let eq53_e936_d_n1: f64 = self.ddt_jacobian(s.dn[205][1]);
        let eq53_e936_d_n2: f64 = self.ddt_jacobian(s.dn[205][2]);
        let eq53_e936_d_n3: f64 = self.ddt_jacobian(s.dn[205][3]);
        let eq53_e936_d_n4: f64 = self.ddt_jacobian(s.dn[205][4]);
        let eq53_e936_d_n5: f64 = self.ddt_jacobian(s.dn[205][5]);
        let eq53_e936_d_n6: f64 = self.ddt_jacobian(s.dn[205][6]);
        let eq53_e936_d_n7: f64 = self.ddt_jacobian(s.dn[205][7]);
        let eq53_e936_d_n8: f64 = self.ddt_jacobian(s.dn[205][8]);
        let eq53_e936_d_n9: f64 = self.ddt_jacobian(s.dn[205][9]);
        let eq53_e936_d_n10: f64 = self.ddt_jacobian(s.dn[205][10]);
        let eq53_e936_d_n11: f64 = self.ddt_jacobian(s.dn[205][11]);
        let eq53_e936_d_n12: f64 = self.ddt_jacobian(s.dn[205][12]);
        let eq53_e936_d_n13: f64 = self.ddt_jacobian(s.dn[205][13]);
        let eq53_e936_d_n14: f64 = self.ddt_jacobian(s.dn[205][14]);
        let eq53_e936_d_n15: f64 = self.ddt_jacobian(s.dn[205][15]);
        let eq53_e936_d_n16: f64 = self.ddt_jacobian(s.dn[205][16]);
        let eq53_e936_d_n17: f64 = self.ddt_jacobian(s.dn[205][17]);
        let eq53_e936_d_n18: f64 = self.ddt_jacobian(s.dn[205][18]);
        let eq53_e936_d_n19: f64 = self.ddt_jacobian(s.dn[205][19]);
        let eq53_e936_d_n20: f64 = self.ddt_jacobian(s.dn[205][20]);
        let eq53_e936_d_n21: f64 = self.ddt_jacobian(s.dn[205][21]);
        let eq53_e936_d_n22: f64 = self.ddt_jacobian(s.dn[205][22]);
        let eq53_e936_d_n23: f64 = self.ddt_jacobian(s.dn[205][23]);
        let eq53_e936_d_n24: f64 = self.ddt_jacobian(s.dn[205][24]);
        let eq53_e936_d_n25: f64 = self.ddt_jacobian(s.dn[205][25]);
        let eq53_e936_d_n26: f64 = self.ddt_jacobian(s.dn[205][26]);
        let eq53_e936_d_n27: f64 = self.ddt_jacobian(s.dn[205][27]);
        let eq53_e936_d_n28: f64 = self.ddt_jacobian(s.dn[205][28]);
        let eq53_e936_d_n29: f64 = self.ddt_jacobian(s.dn[205][29]);
        let eq53_e939: f64 = (p.p355 * (nv7 - nv15));
        let eq53_e939_d_n7: f64 = p.p355;
        let eq53_e939_d_n15: f64 = (-p.p355);
        let eq53_e940: f64 = self.eval_ddt(33, eq53_e939);
        let eq53_e940_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n7: f64 = self.ddt_jacobian(eq53_e939_d_n7);
        let eq53_e940_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n15: f64 = self.ddt_jacobian(eq53_e939_d_n15);
        let eq53_e940_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq53_e940_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq53_e941: f64 = (eq53_e936 + eq53_e940);
        let eq53_e941_d_n0: f64 = (eq53_e936_d_n0 + eq53_e940_d_n0);
        let eq53_e941_d_n1: f64 = (eq53_e936_d_n1 + eq53_e940_d_n1);
        let eq53_e941_d_n2: f64 = (eq53_e936_d_n2 + eq53_e940_d_n2);
        let eq53_e941_d_n3: f64 = (eq53_e936_d_n3 + eq53_e940_d_n3);
        let eq53_e941_d_n4: f64 = (eq53_e936_d_n4 + eq53_e940_d_n4);
        let eq53_e941_d_n5: f64 = (eq53_e936_d_n5 + eq53_e940_d_n5);
        let eq53_e941_d_n6: f64 = (eq53_e936_d_n6 + eq53_e940_d_n6);
        let eq53_e941_d_n7: f64 = (eq53_e936_d_n7 + eq53_e940_d_n7);
        let eq53_e941_d_n8: f64 = (eq53_e936_d_n8 + eq53_e940_d_n8);
        let eq53_e941_d_n9: f64 = (eq53_e936_d_n9 + eq53_e940_d_n9);
        let eq53_e941_d_n10: f64 = (eq53_e936_d_n10 + eq53_e940_d_n10);
        let eq53_e941_d_n11: f64 = (eq53_e936_d_n11 + eq53_e940_d_n11);
        let eq53_e941_d_n12: f64 = (eq53_e936_d_n12 + eq53_e940_d_n12);
        let eq53_e941_d_n13: f64 = (eq53_e936_d_n13 + eq53_e940_d_n13);
        let eq53_e941_d_n14: f64 = (eq53_e936_d_n14 + eq53_e940_d_n14);
        let eq53_e941_d_n15: f64 = (eq53_e936_d_n15 + eq53_e940_d_n15);
        let eq53_e941_d_n16: f64 = (eq53_e936_d_n16 + eq53_e940_d_n16);
        let eq53_e941_d_n17: f64 = (eq53_e936_d_n17 + eq53_e940_d_n17);
        let eq53_e941_d_n18: f64 = (eq53_e936_d_n18 + eq53_e940_d_n18);
        let eq53_e941_d_n19: f64 = (eq53_e936_d_n19 + eq53_e940_d_n19);
        let eq53_e941_d_n20: f64 = (eq53_e936_d_n20 + eq53_e940_d_n20);
        let eq53_e941_d_n21: f64 = (eq53_e936_d_n21 + eq53_e940_d_n21);
        let eq53_e941_d_n22: f64 = (eq53_e936_d_n22 + eq53_e940_d_n22);
        let eq53_e941_d_n23: f64 = (eq53_e936_d_n23 + eq53_e940_d_n23);
        let eq53_e941_d_n24: f64 = (eq53_e936_d_n24 + eq53_e940_d_n24);
        let eq53_e941_d_n25: f64 = (eq53_e936_d_n25 + eq53_e940_d_n25);
        let eq53_e941_d_n26: f64 = (eq53_e936_d_n26 + eq53_e940_d_n26);
        let eq53_e941_d_n27: f64 = (eq53_e936_d_n27 + eq53_e940_d_n27);
        let eq53_e941_d_n28: f64 = (eq53_e936_d_n28 + eq53_e940_d_n28);
        let eq53_e941_d_n29: f64 = (eq53_e936_d_n29 + eq53_e940_d_n29);
        (eq53_e941, eq53_e941_d_n0, eq53_e941_d_n1, eq53_e941_d_n2, eq53_e941_d_n3, eq53_e941_d_n4, eq53_e941_d_n5, eq53_e941_d_n6, eq53_e941_d_n7, eq53_e941_d_n8, eq53_e941_d_n9, eq53_e941_d_n10, eq53_e941_d_n11, eq53_e941_d_n12, eq53_e941_d_n13, eq53_e941_d_n14, eq53_e941_d_n15, eq53_e941_d_n16, eq53_e941_d_n17, eq53_e941_d_n18, eq53_e941_d_n19, eq53_e941_d_n20, eq53_e941_d_n21, eq53_e941_d_n22, eq53_e941_d_n23, eq53_e941_d_n24, eq53_e941_d_n25, eq53_e941_d_n26, eq53_e941_d_n27, eq53_e941_d_n28, eq53_e941_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e943;
        let eq53_node_derivatives: [f64; 30] = [eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[15]),
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
        let (eq54_e948,) = {
    if (!(s.v[613] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e948;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[16]),
            self.multiplicity * (eq54_value),
            &[
            ],
        );
    }

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
        let (eq55_e953,) = {
    if (!(s.v[613] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e953;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq55_value),
            &[
            ],
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq56_e955: f64 = self.eval_ddt(34, s.v[206]);
        let eq56_e955_d_n0: f64 = self.ddt_jacobian(s.dn[206][0]);
        let eq56_e955_d_n1: f64 = self.ddt_jacobian(s.dn[206][1]);
        let eq56_e955_d_n2: f64 = self.ddt_jacobian(s.dn[206][2]);
        let eq56_e955_d_n3: f64 = self.ddt_jacobian(s.dn[206][3]);
        let eq56_e955_d_n4: f64 = self.ddt_jacobian(s.dn[206][4]);
        let eq56_e955_d_n5: f64 = self.ddt_jacobian(s.dn[206][5]);
        let eq56_e955_d_n6: f64 = self.ddt_jacobian(s.dn[206][6]);
        let eq56_e955_d_n7: f64 = self.ddt_jacobian(s.dn[206][7]);
        let eq56_e955_d_n8: f64 = self.ddt_jacobian(s.dn[206][8]);
        let eq56_e955_d_n9: f64 = self.ddt_jacobian(s.dn[206][9]);
        let eq56_e955_d_n10: f64 = self.ddt_jacobian(s.dn[206][10]);
        let eq56_e955_d_n11: f64 = self.ddt_jacobian(s.dn[206][11]);
        let eq56_e955_d_n12: f64 = self.ddt_jacobian(s.dn[206][12]);
        let eq56_e955_d_n13: f64 = self.ddt_jacobian(s.dn[206][13]);
        let eq56_e955_d_n14: f64 = self.ddt_jacobian(s.dn[206][14]);
        let eq56_e955_d_n15: f64 = self.ddt_jacobian(s.dn[206][15]);
        let eq56_e955_d_n16: f64 = self.ddt_jacobian(s.dn[206][16]);
        let eq56_e955_d_n17: f64 = self.ddt_jacobian(s.dn[206][17]);
        let eq56_e955_d_n18: f64 = self.ddt_jacobian(s.dn[206][18]);
        let eq56_e955_d_n19: f64 = self.ddt_jacobian(s.dn[206][19]);
        let eq56_e955_d_n20: f64 = self.ddt_jacobian(s.dn[206][20]);
        let eq56_e955_d_n21: f64 = self.ddt_jacobian(s.dn[206][21]);
        let eq56_e955_d_n22: f64 = self.ddt_jacobian(s.dn[206][22]);
        let eq56_e955_d_n23: f64 = self.ddt_jacobian(s.dn[206][23]);
        let eq56_e955_d_n24: f64 = self.ddt_jacobian(s.dn[206][24]);
        let eq56_e955_d_n25: f64 = self.ddt_jacobian(s.dn[206][25]);
        let eq56_e955_d_n26: f64 = self.ddt_jacobian(s.dn[206][26]);
        let eq56_e955_d_n27: f64 = self.ddt_jacobian(s.dn[206][27]);
        let eq56_e955_d_n28: f64 = self.ddt_jacobian(s.dn[206][28]);
        let eq56_e955_d_n29: f64 = self.ddt_jacobian(s.dn[206][29]);
        let eq56_e958: f64 = (p.p355 * (nv3 - nv15));
        let eq56_e958_d_n3: f64 = p.p355;
        let eq56_e958_d_n15: f64 = (-p.p355);
        let eq56_e959: f64 = self.eval_ddt(35, eq56_e958);
        let eq56_e959_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n3: f64 = self.ddt_jacobian(eq56_e958_d_n3);
        let eq56_e959_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n15: f64 = self.ddt_jacobian(eq56_e958_d_n15);
        let eq56_e959_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq56_e959_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq56_e960: f64 = (eq56_e955 + eq56_e959);
        let eq56_e960_d_n0: f64 = (eq56_e955_d_n0 + eq56_e959_d_n0);
        let eq56_e960_d_n1: f64 = (eq56_e955_d_n1 + eq56_e959_d_n1);
        let eq56_e960_d_n2: f64 = (eq56_e955_d_n2 + eq56_e959_d_n2);
        let eq56_e960_d_n3: f64 = (eq56_e955_d_n3 + eq56_e959_d_n3);
        let eq56_e960_d_n4: f64 = (eq56_e955_d_n4 + eq56_e959_d_n4);
        let eq56_e960_d_n5: f64 = (eq56_e955_d_n5 + eq56_e959_d_n5);
        let eq56_e960_d_n6: f64 = (eq56_e955_d_n6 + eq56_e959_d_n6);
        let eq56_e960_d_n7: f64 = (eq56_e955_d_n7 + eq56_e959_d_n7);
        let eq56_e960_d_n8: f64 = (eq56_e955_d_n8 + eq56_e959_d_n8);
        let eq56_e960_d_n9: f64 = (eq56_e955_d_n9 + eq56_e959_d_n9);
        let eq56_e960_d_n10: f64 = (eq56_e955_d_n10 + eq56_e959_d_n10);
        let eq56_e960_d_n11: f64 = (eq56_e955_d_n11 + eq56_e959_d_n11);
        let eq56_e960_d_n12: f64 = (eq56_e955_d_n12 + eq56_e959_d_n12);
        let eq56_e960_d_n13: f64 = (eq56_e955_d_n13 + eq56_e959_d_n13);
        let eq56_e960_d_n14: f64 = (eq56_e955_d_n14 + eq56_e959_d_n14);
        let eq56_e960_d_n15: f64 = (eq56_e955_d_n15 + eq56_e959_d_n15);
        let eq56_e960_d_n16: f64 = (eq56_e955_d_n16 + eq56_e959_d_n16);
        let eq56_e960_d_n17: f64 = (eq56_e955_d_n17 + eq56_e959_d_n17);
        let eq56_e960_d_n18: f64 = (eq56_e955_d_n18 + eq56_e959_d_n18);
        let eq56_e960_d_n19: f64 = (eq56_e955_d_n19 + eq56_e959_d_n19);
        let eq56_e960_d_n20: f64 = (eq56_e955_d_n20 + eq56_e959_d_n20);
        let eq56_e960_d_n21: f64 = (eq56_e955_d_n21 + eq56_e959_d_n21);
        let eq56_e960_d_n22: f64 = (eq56_e955_d_n22 + eq56_e959_d_n22);
        let eq56_e960_d_n23: f64 = (eq56_e955_d_n23 + eq56_e959_d_n23);
        let eq56_e960_d_n24: f64 = (eq56_e955_d_n24 + eq56_e959_d_n24);
        let eq56_e960_d_n25: f64 = (eq56_e955_d_n25 + eq56_e959_d_n25);
        let eq56_e960_d_n26: f64 = (eq56_e955_d_n26 + eq56_e959_d_n26);
        let eq56_e960_d_n27: f64 = (eq56_e955_d_n27 + eq56_e959_d_n27);
        let eq56_e960_d_n28: f64 = (eq56_e955_d_n28 + eq56_e959_d_n28);
        let eq56_e960_d_n29: f64 = (eq56_e955_d_n29 + eq56_e959_d_n29);
        let eq56_value: f64 = eq56_e960;
        let eq56_node_derivatives: [f64; 30] = [eq56_e960_d_n0, eq56_e960_d_n1, eq56_e960_d_n2, eq56_e960_d_n3, eq56_e960_d_n4, eq56_e960_d_n5, eq56_e960_d_n6, eq56_e960_d_n7, eq56_e960_d_n8, eq56_e960_d_n9, eq56_e960_d_n10, eq56_e960_d_n11, eq56_e960_d_n12, eq56_e960_d_n13, eq56_e960_d_n14, eq56_e960_d_n15, eq56_e960_d_n16, eq56_e960_d_n17, eq56_e960_d_n18, eq56_e960_d_n19, eq56_e960_d_n20, eq56_e960_d_n21, eq56_e960_d_n22, eq56_e960_d_n23, eq56_e960_d_n24, eq56_e960_d_n25, eq56_e960_d_n26, eq56_e960_d_n27, eq56_e960_d_n28, eq56_e960_d_n29];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[15]),
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq57_e968, eq57_e968_d_n0, eq57_e968_d_n1, eq57_e968_d_n2, eq57_e968_d_n3, eq57_e968_d_n4, eq57_e968_d_n5, eq57_e968_d_n6, eq57_e968_d_n7, eq57_e968_d_n8, eq57_e968_d_n9, eq57_e968_d_n10, eq57_e968_d_n11, eq57_e968_d_n12, eq57_e968_d_n13, eq57_e968_d_n14, eq57_e968_d_n15, eq57_e968_d_n16, eq57_e968_d_n17, eq57_e968_d_n18, eq57_e968_d_n19, eq57_e968_d_n20, eq57_e968_d_n21, eq57_e968_d_n22, eq57_e968_d_n23, eq57_e968_d_n24, eq57_e968_d_n25, eq57_e968_d_n26, eq57_e968_d_n27, eq57_e968_d_n28, eq57_e968_d_n29,) = {
    if (s.v[614] != 0.0) {
        let eq57_e965: f64 = (s.v[0] * (nv15 - nv14));
        let eq57_e965_d_n14: f64 = (-s.v[0]);
        let eq57_e965_d_n15: f64 = s.v[0];
        let eq57_e966: f64 = (s.v[196] + eq57_e965);
        let eq57_e966_d_n14: f64 = (s.dn[196][14] + eq57_e965_d_n14);
        let eq57_e966_d_n15: f64 = (s.dn[196][15] + eq57_e965_d_n15);
        (eq57_e966, s.dn[196][0], s.dn[196][1], s.dn[196][2], s.dn[196][3], s.dn[196][4], s.dn[196][5], s.dn[196][6], s.dn[196][7], s.dn[196][8], s.dn[196][9], s.dn[196][10], s.dn[196][11], s.dn[196][12], s.dn[196][13], eq57_e966_d_n14, eq57_e966_d_n15, s.dn[196][16], s.dn[196][17], s.dn[196][18], s.dn[196][19], s.dn[196][20], s.dn[196][21], s.dn[196][22], s.dn[196][23], s.dn[196][24], s.dn[196][25], s.dn[196][26], s.dn[196][27], s.dn[196][28], s.dn[196][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e968;
        let eq57_node_derivatives: [f64; 30] = [eq57_e968_d_n0, eq57_e968_d_n1, eq57_e968_d_n2, eq57_e968_d_n3, eq57_e968_d_n4, eq57_e968_d_n5, eq57_e968_d_n6, eq57_e968_d_n7, eq57_e968_d_n8, eq57_e968_d_n9, eq57_e968_d_n10, eq57_e968_d_n11, eq57_e968_d_n12, eq57_e968_d_n13, eq57_e968_d_n14, eq57_e968_d_n15, eq57_e968_d_n16, eq57_e968_d_n17, eq57_e968_d_n18, eq57_e968_d_n19, eq57_e968_d_n20, eq57_e968_d_n21, eq57_e968_d_n22, eq57_e968_d_n23, eq57_e968_d_n24, eq57_e968_d_n25, eq57_e968_d_n26, eq57_e968_d_n27, eq57_e968_d_n28, eq57_e968_d_n29];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            Some(nodes[14]),
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
        let (eq58_e973,) = {
    if (!(s.v[614] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e973;
        stamper.stamp_potential(
            branches[20],
            eq58_value,
            &[
            ],
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq59_e983, eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29,) = {
    if (s.v[760] != 0.0) {
        let eq59_e976: f64 = self.eval_ddt(36, s.v[197]);
        let eq59_e976_d_n0: f64 = self.ddt_jacobian(s.dn[197][0]);
        let eq59_e976_d_n1: f64 = self.ddt_jacobian(s.dn[197][1]);
        let eq59_e976_d_n2: f64 = self.ddt_jacobian(s.dn[197][2]);
        let eq59_e976_d_n3: f64 = self.ddt_jacobian(s.dn[197][3]);
        let eq59_e976_d_n4: f64 = self.ddt_jacobian(s.dn[197][4]);
        let eq59_e976_d_n5: f64 = self.ddt_jacobian(s.dn[197][5]);
        let eq59_e976_d_n6: f64 = self.ddt_jacobian(s.dn[197][6]);
        let eq59_e976_d_n7: f64 = self.ddt_jacobian(s.dn[197][7]);
        let eq59_e976_d_n8: f64 = self.ddt_jacobian(s.dn[197][8]);
        let eq59_e976_d_n9: f64 = self.ddt_jacobian(s.dn[197][9]);
        let eq59_e976_d_n10: f64 = self.ddt_jacobian(s.dn[197][10]);
        let eq59_e976_d_n11: f64 = self.ddt_jacobian(s.dn[197][11]);
        let eq59_e976_d_n12: f64 = self.ddt_jacobian(s.dn[197][12]);
        let eq59_e976_d_n13: f64 = self.ddt_jacobian(s.dn[197][13]);
        let eq59_e976_d_n14: f64 = self.ddt_jacobian(s.dn[197][14]);
        let eq59_e976_d_n15: f64 = self.ddt_jacobian(s.dn[197][15]);
        let eq59_e976_d_n16: f64 = self.ddt_jacobian(s.dn[197][16]);
        let eq59_e976_d_n17: f64 = self.ddt_jacobian(s.dn[197][17]);
        let eq59_e976_d_n18: f64 = self.ddt_jacobian(s.dn[197][18]);
        let eq59_e976_d_n19: f64 = self.ddt_jacobian(s.dn[197][19]);
        let eq59_e976_d_n20: f64 = self.ddt_jacobian(s.dn[197][20]);
        let eq59_e976_d_n21: f64 = self.ddt_jacobian(s.dn[197][21]);
        let eq59_e976_d_n22: f64 = self.ddt_jacobian(s.dn[197][22]);
        let eq59_e976_d_n23: f64 = self.ddt_jacobian(s.dn[197][23]);
        let eq59_e976_d_n24: f64 = self.ddt_jacobian(s.dn[197][24]);
        let eq59_e976_d_n25: f64 = self.ddt_jacobian(s.dn[197][25]);
        let eq59_e976_d_n26: f64 = self.ddt_jacobian(s.dn[197][26]);
        let eq59_e976_d_n27: f64 = self.ddt_jacobian(s.dn[197][27]);
        let eq59_e976_d_n28: f64 = self.ddt_jacobian(s.dn[197][28]);
        let eq59_e976_d_n29: f64 = self.ddt_jacobian(s.dn[197][29]);
        let eq59_e979: f64 = (p.p355 * (nv7 - nv14));
        let eq59_e979_d_n7: f64 = p.p355;
        let eq59_e979_d_n14: f64 = (-p.p355);
        let eq59_e980: f64 = self.eval_ddt(37, eq59_e979);
        let eq59_e980_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n7: f64 = self.ddt_jacobian(eq59_e979_d_n7);
        let eq59_e980_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n14: f64 = self.ddt_jacobian(eq59_e979_d_n14);
        let eq59_e980_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq59_e980_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq59_e981: f64 = (eq59_e976 + eq59_e980);
        let eq59_e981_d_n0: f64 = (eq59_e976_d_n0 + eq59_e980_d_n0);
        let eq59_e981_d_n1: f64 = (eq59_e976_d_n1 + eq59_e980_d_n1);
        let eq59_e981_d_n2: f64 = (eq59_e976_d_n2 + eq59_e980_d_n2);
        let eq59_e981_d_n3: f64 = (eq59_e976_d_n3 + eq59_e980_d_n3);
        let eq59_e981_d_n4: f64 = (eq59_e976_d_n4 + eq59_e980_d_n4);
        let eq59_e981_d_n5: f64 = (eq59_e976_d_n5 + eq59_e980_d_n5);
        let eq59_e981_d_n6: f64 = (eq59_e976_d_n6 + eq59_e980_d_n6);
        let eq59_e981_d_n7: f64 = (eq59_e976_d_n7 + eq59_e980_d_n7);
        let eq59_e981_d_n8: f64 = (eq59_e976_d_n8 + eq59_e980_d_n8);
        let eq59_e981_d_n9: f64 = (eq59_e976_d_n9 + eq59_e980_d_n9);
        let eq59_e981_d_n10: f64 = (eq59_e976_d_n10 + eq59_e980_d_n10);
        let eq59_e981_d_n11: f64 = (eq59_e976_d_n11 + eq59_e980_d_n11);
        let eq59_e981_d_n12: f64 = (eq59_e976_d_n12 + eq59_e980_d_n12);
        let eq59_e981_d_n13: f64 = (eq59_e976_d_n13 + eq59_e980_d_n13);
        let eq59_e981_d_n14: f64 = (eq59_e976_d_n14 + eq59_e980_d_n14);
        let eq59_e981_d_n15: f64 = (eq59_e976_d_n15 + eq59_e980_d_n15);
        let eq59_e981_d_n16: f64 = (eq59_e976_d_n16 + eq59_e980_d_n16);
        let eq59_e981_d_n17: f64 = (eq59_e976_d_n17 + eq59_e980_d_n17);
        let eq59_e981_d_n18: f64 = (eq59_e976_d_n18 + eq59_e980_d_n18);
        let eq59_e981_d_n19: f64 = (eq59_e976_d_n19 + eq59_e980_d_n19);
        let eq59_e981_d_n20: f64 = (eq59_e976_d_n20 + eq59_e980_d_n20);
        let eq59_e981_d_n21: f64 = (eq59_e976_d_n21 + eq59_e980_d_n21);
        let eq59_e981_d_n22: f64 = (eq59_e976_d_n22 + eq59_e980_d_n22);
        let eq59_e981_d_n23: f64 = (eq59_e976_d_n23 + eq59_e980_d_n23);
        let eq59_e981_d_n24: f64 = (eq59_e976_d_n24 + eq59_e980_d_n24);
        let eq59_e981_d_n25: f64 = (eq59_e976_d_n25 + eq59_e980_d_n25);
        let eq59_e981_d_n26: f64 = (eq59_e976_d_n26 + eq59_e980_d_n26);
        let eq59_e981_d_n27: f64 = (eq59_e976_d_n27 + eq59_e980_d_n27);
        let eq59_e981_d_n28: f64 = (eq59_e976_d_n28 + eq59_e980_d_n28);
        let eq59_e981_d_n29: f64 = (eq59_e976_d_n29 + eq59_e980_d_n29);
        (eq59_e981, eq59_e981_d_n0, eq59_e981_d_n1, eq59_e981_d_n2, eq59_e981_d_n3, eq59_e981_d_n4, eq59_e981_d_n5, eq59_e981_d_n6, eq59_e981_d_n7, eq59_e981_d_n8, eq59_e981_d_n9, eq59_e981_d_n10, eq59_e981_d_n11, eq59_e981_d_n12, eq59_e981_d_n13, eq59_e981_d_n14, eq59_e981_d_n15, eq59_e981_d_n16, eq59_e981_d_n17, eq59_e981_d_n18, eq59_e981_d_n19, eq59_e981_d_n20, eq59_e981_d_n21, eq59_e981_d_n22, eq59_e981_d_n23, eq59_e981_d_n24, eq59_e981_d_n25, eq59_e981_d_n26, eq59_e981_d_n27, eq59_e981_d_n28, eq59_e981_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e983;
        let eq59_node_derivatives: [f64; 30] = [eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29];
        let eq59_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            self.multiplicity * (eq59_value),
            &nodes,
            &eq59_node_derivatives,
            &branches,
            &eq59_branch_derivatives,
            self.multiplicity,
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq60_e993, eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29,) = {
    if (s.v[760] != 0.0) {
        let eq60_e986: f64 = self.eval_ddt(38, s.v[198]);
        let eq60_e986_d_n0: f64 = self.ddt_jacobian(s.dn[198][0]);
        let eq60_e986_d_n1: f64 = self.ddt_jacobian(s.dn[198][1]);
        let eq60_e986_d_n2: f64 = self.ddt_jacobian(s.dn[198][2]);
        let eq60_e986_d_n3: f64 = self.ddt_jacobian(s.dn[198][3]);
        let eq60_e986_d_n4: f64 = self.ddt_jacobian(s.dn[198][4]);
        let eq60_e986_d_n5: f64 = self.ddt_jacobian(s.dn[198][5]);
        let eq60_e986_d_n6: f64 = self.ddt_jacobian(s.dn[198][6]);
        let eq60_e986_d_n7: f64 = self.ddt_jacobian(s.dn[198][7]);
        let eq60_e986_d_n8: f64 = self.ddt_jacobian(s.dn[198][8]);
        let eq60_e986_d_n9: f64 = self.ddt_jacobian(s.dn[198][9]);
        let eq60_e986_d_n10: f64 = self.ddt_jacobian(s.dn[198][10]);
        let eq60_e986_d_n11: f64 = self.ddt_jacobian(s.dn[198][11]);
        let eq60_e986_d_n12: f64 = self.ddt_jacobian(s.dn[198][12]);
        let eq60_e986_d_n13: f64 = self.ddt_jacobian(s.dn[198][13]);
        let eq60_e986_d_n14: f64 = self.ddt_jacobian(s.dn[198][14]);
        let eq60_e986_d_n15: f64 = self.ddt_jacobian(s.dn[198][15]);
        let eq60_e986_d_n16: f64 = self.ddt_jacobian(s.dn[198][16]);
        let eq60_e986_d_n17: f64 = self.ddt_jacobian(s.dn[198][17]);
        let eq60_e986_d_n18: f64 = self.ddt_jacobian(s.dn[198][18]);
        let eq60_e986_d_n19: f64 = self.ddt_jacobian(s.dn[198][19]);
        let eq60_e986_d_n20: f64 = self.ddt_jacobian(s.dn[198][20]);
        let eq60_e986_d_n21: f64 = self.ddt_jacobian(s.dn[198][21]);
        let eq60_e986_d_n22: f64 = self.ddt_jacobian(s.dn[198][22]);
        let eq60_e986_d_n23: f64 = self.ddt_jacobian(s.dn[198][23]);
        let eq60_e986_d_n24: f64 = self.ddt_jacobian(s.dn[198][24]);
        let eq60_e986_d_n25: f64 = self.ddt_jacobian(s.dn[198][25]);
        let eq60_e986_d_n26: f64 = self.ddt_jacobian(s.dn[198][26]);
        let eq60_e986_d_n27: f64 = self.ddt_jacobian(s.dn[198][27]);
        let eq60_e986_d_n28: f64 = self.ddt_jacobian(s.dn[198][28]);
        let eq60_e986_d_n29: f64 = self.ddt_jacobian(s.dn[198][29]);
        let eq60_e989: f64 = (p.p355 * (nv7 - nv15));
        let eq60_e989_d_n7: f64 = p.p355;
        let eq60_e989_d_n15: f64 = (-p.p355);
        let eq60_e990: f64 = self.eval_ddt(39, eq60_e989);
        let eq60_e990_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n7: f64 = self.ddt_jacobian(eq60_e989_d_n7);
        let eq60_e990_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n15: f64 = self.ddt_jacobian(eq60_e989_d_n15);
        let eq60_e990_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq60_e990_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq60_e991: f64 = (eq60_e986 + eq60_e990);
        let eq60_e991_d_n0: f64 = (eq60_e986_d_n0 + eq60_e990_d_n0);
        let eq60_e991_d_n1: f64 = (eq60_e986_d_n1 + eq60_e990_d_n1);
        let eq60_e991_d_n2: f64 = (eq60_e986_d_n2 + eq60_e990_d_n2);
        let eq60_e991_d_n3: f64 = (eq60_e986_d_n3 + eq60_e990_d_n3);
        let eq60_e991_d_n4: f64 = (eq60_e986_d_n4 + eq60_e990_d_n4);
        let eq60_e991_d_n5: f64 = (eq60_e986_d_n5 + eq60_e990_d_n5);
        let eq60_e991_d_n6: f64 = (eq60_e986_d_n6 + eq60_e990_d_n6);
        let eq60_e991_d_n7: f64 = (eq60_e986_d_n7 + eq60_e990_d_n7);
        let eq60_e991_d_n8: f64 = (eq60_e986_d_n8 + eq60_e990_d_n8);
        let eq60_e991_d_n9: f64 = (eq60_e986_d_n9 + eq60_e990_d_n9);
        let eq60_e991_d_n10: f64 = (eq60_e986_d_n10 + eq60_e990_d_n10);
        let eq60_e991_d_n11: f64 = (eq60_e986_d_n11 + eq60_e990_d_n11);
        let eq60_e991_d_n12: f64 = (eq60_e986_d_n12 + eq60_e990_d_n12);
        let eq60_e991_d_n13: f64 = (eq60_e986_d_n13 + eq60_e990_d_n13);
        let eq60_e991_d_n14: f64 = (eq60_e986_d_n14 + eq60_e990_d_n14);
        let eq60_e991_d_n15: f64 = (eq60_e986_d_n15 + eq60_e990_d_n15);
        let eq60_e991_d_n16: f64 = (eq60_e986_d_n16 + eq60_e990_d_n16);
        let eq60_e991_d_n17: f64 = (eq60_e986_d_n17 + eq60_e990_d_n17);
        let eq60_e991_d_n18: f64 = (eq60_e986_d_n18 + eq60_e990_d_n18);
        let eq60_e991_d_n19: f64 = (eq60_e986_d_n19 + eq60_e990_d_n19);
        let eq60_e991_d_n20: f64 = (eq60_e986_d_n20 + eq60_e990_d_n20);
        let eq60_e991_d_n21: f64 = (eq60_e986_d_n21 + eq60_e990_d_n21);
        let eq60_e991_d_n22: f64 = (eq60_e986_d_n22 + eq60_e990_d_n22);
        let eq60_e991_d_n23: f64 = (eq60_e986_d_n23 + eq60_e990_d_n23);
        let eq60_e991_d_n24: f64 = (eq60_e986_d_n24 + eq60_e990_d_n24);
        let eq60_e991_d_n25: f64 = (eq60_e986_d_n25 + eq60_e990_d_n25);
        let eq60_e991_d_n26: f64 = (eq60_e986_d_n26 + eq60_e990_d_n26);
        let eq60_e991_d_n27: f64 = (eq60_e986_d_n27 + eq60_e990_d_n27);
        let eq60_e991_d_n28: f64 = (eq60_e986_d_n28 + eq60_e990_d_n28);
        let eq60_e991_d_n29: f64 = (eq60_e986_d_n29 + eq60_e990_d_n29);
        (eq60_e991, eq60_e991_d_n0, eq60_e991_d_n1, eq60_e991_d_n2, eq60_e991_d_n3, eq60_e991_d_n4, eq60_e991_d_n5, eq60_e991_d_n6, eq60_e991_d_n7, eq60_e991_d_n8, eq60_e991_d_n9, eq60_e991_d_n10, eq60_e991_d_n11, eq60_e991_d_n12, eq60_e991_d_n13, eq60_e991_d_n14, eq60_e991_d_n15, eq60_e991_d_n16, eq60_e991_d_n17, eq60_e991_d_n18, eq60_e991_d_n19, eq60_e991_d_n20, eq60_e991_d_n21, eq60_e991_d_n22, eq60_e991_d_n23, eq60_e991_d_n24, eq60_e991_d_n25, eq60_e991_d_n26, eq60_e991_d_n27, eq60_e991_d_n28, eq60_e991_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e993;
        let eq60_node_derivatives: [f64; 30] = [eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29];
        let eq60_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            self.multiplicity * (eq60_value),
            &nodes,
            &eq60_node_derivatives,
            &branches,
            &eq60_branch_derivatives,
            self.multiplicity,
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq61_e1003, eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29,) = {
    if (s.v[760] != 0.0) {
        let eq61_e996: f64 = self.eval_ddt(40, s.v[199]);
        let eq61_e996_d_n0: f64 = self.ddt_jacobian(s.dn[199][0]);
        let eq61_e996_d_n1: f64 = self.ddt_jacobian(s.dn[199][1]);
        let eq61_e996_d_n2: f64 = self.ddt_jacobian(s.dn[199][2]);
        let eq61_e996_d_n3: f64 = self.ddt_jacobian(s.dn[199][3]);
        let eq61_e996_d_n4: f64 = self.ddt_jacobian(s.dn[199][4]);
        let eq61_e996_d_n5: f64 = self.ddt_jacobian(s.dn[199][5]);
        let eq61_e996_d_n6: f64 = self.ddt_jacobian(s.dn[199][6]);
        let eq61_e996_d_n7: f64 = self.ddt_jacobian(s.dn[199][7]);
        let eq61_e996_d_n8: f64 = self.ddt_jacobian(s.dn[199][8]);
        let eq61_e996_d_n9: f64 = self.ddt_jacobian(s.dn[199][9]);
        let eq61_e996_d_n10: f64 = self.ddt_jacobian(s.dn[199][10]);
        let eq61_e996_d_n11: f64 = self.ddt_jacobian(s.dn[199][11]);
        let eq61_e996_d_n12: f64 = self.ddt_jacobian(s.dn[199][12]);
        let eq61_e996_d_n13: f64 = self.ddt_jacobian(s.dn[199][13]);
        let eq61_e996_d_n14: f64 = self.ddt_jacobian(s.dn[199][14]);
        let eq61_e996_d_n15: f64 = self.ddt_jacobian(s.dn[199][15]);
        let eq61_e996_d_n16: f64 = self.ddt_jacobian(s.dn[199][16]);
        let eq61_e996_d_n17: f64 = self.ddt_jacobian(s.dn[199][17]);
        let eq61_e996_d_n18: f64 = self.ddt_jacobian(s.dn[199][18]);
        let eq61_e996_d_n19: f64 = self.ddt_jacobian(s.dn[199][19]);
        let eq61_e996_d_n20: f64 = self.ddt_jacobian(s.dn[199][20]);
        let eq61_e996_d_n21: f64 = self.ddt_jacobian(s.dn[199][21]);
        let eq61_e996_d_n22: f64 = self.ddt_jacobian(s.dn[199][22]);
        let eq61_e996_d_n23: f64 = self.ddt_jacobian(s.dn[199][23]);
        let eq61_e996_d_n24: f64 = self.ddt_jacobian(s.dn[199][24]);
        let eq61_e996_d_n25: f64 = self.ddt_jacobian(s.dn[199][25]);
        let eq61_e996_d_n26: f64 = self.ddt_jacobian(s.dn[199][26]);
        let eq61_e996_d_n27: f64 = self.ddt_jacobian(s.dn[199][27]);
        let eq61_e996_d_n28: f64 = self.ddt_jacobian(s.dn[199][28]);
        let eq61_e996_d_n29: f64 = self.ddt_jacobian(s.dn[199][29]);
        let eq61_e999: f64 = (p.p355 * (nv2 - nv14));
        let eq61_e999_d_n2: f64 = p.p355;
        let eq61_e999_d_n14: f64 = (-p.p355);
        let eq61_e1000: f64 = self.eval_ddt(41, eq61_e999);
        let eq61_e1000_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n2: f64 = self.ddt_jacobian(eq61_e999_d_n2);
        let eq61_e1000_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n14: f64 = self.ddt_jacobian(eq61_e999_d_n14);
        let eq61_e1000_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq61_e1000_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq61_e1001: f64 = (eq61_e996 + eq61_e1000);
        let eq61_e1001_d_n0: f64 = (eq61_e996_d_n0 + eq61_e1000_d_n0);
        let eq61_e1001_d_n1: f64 = (eq61_e996_d_n1 + eq61_e1000_d_n1);
        let eq61_e1001_d_n2: f64 = (eq61_e996_d_n2 + eq61_e1000_d_n2);
        let eq61_e1001_d_n3: f64 = (eq61_e996_d_n3 + eq61_e1000_d_n3);
        let eq61_e1001_d_n4: f64 = (eq61_e996_d_n4 + eq61_e1000_d_n4);
        let eq61_e1001_d_n5: f64 = (eq61_e996_d_n5 + eq61_e1000_d_n5);
        let eq61_e1001_d_n6: f64 = (eq61_e996_d_n6 + eq61_e1000_d_n6);
        let eq61_e1001_d_n7: f64 = (eq61_e996_d_n7 + eq61_e1000_d_n7);
        let eq61_e1001_d_n8: f64 = (eq61_e996_d_n8 + eq61_e1000_d_n8);
        let eq61_e1001_d_n9: f64 = (eq61_e996_d_n9 + eq61_e1000_d_n9);
        let eq61_e1001_d_n10: f64 = (eq61_e996_d_n10 + eq61_e1000_d_n10);
        let eq61_e1001_d_n11: f64 = (eq61_e996_d_n11 + eq61_e1000_d_n11);
        let eq61_e1001_d_n12: f64 = (eq61_e996_d_n12 + eq61_e1000_d_n12);
        let eq61_e1001_d_n13: f64 = (eq61_e996_d_n13 + eq61_e1000_d_n13);
        let eq61_e1001_d_n14: f64 = (eq61_e996_d_n14 + eq61_e1000_d_n14);
        let eq61_e1001_d_n15: f64 = (eq61_e996_d_n15 + eq61_e1000_d_n15);
        let eq61_e1001_d_n16: f64 = (eq61_e996_d_n16 + eq61_e1000_d_n16);
        let eq61_e1001_d_n17: f64 = (eq61_e996_d_n17 + eq61_e1000_d_n17);
        let eq61_e1001_d_n18: f64 = (eq61_e996_d_n18 + eq61_e1000_d_n18);
        let eq61_e1001_d_n19: f64 = (eq61_e996_d_n19 + eq61_e1000_d_n19);
        let eq61_e1001_d_n20: f64 = (eq61_e996_d_n20 + eq61_e1000_d_n20);
        let eq61_e1001_d_n21: f64 = (eq61_e996_d_n21 + eq61_e1000_d_n21);
        let eq61_e1001_d_n22: f64 = (eq61_e996_d_n22 + eq61_e1000_d_n22);
        let eq61_e1001_d_n23: f64 = (eq61_e996_d_n23 + eq61_e1000_d_n23);
        let eq61_e1001_d_n24: f64 = (eq61_e996_d_n24 + eq61_e1000_d_n24);
        let eq61_e1001_d_n25: f64 = (eq61_e996_d_n25 + eq61_e1000_d_n25);
        let eq61_e1001_d_n26: f64 = (eq61_e996_d_n26 + eq61_e1000_d_n26);
        let eq61_e1001_d_n27: f64 = (eq61_e996_d_n27 + eq61_e1000_d_n27);
        let eq61_e1001_d_n28: f64 = (eq61_e996_d_n28 + eq61_e1000_d_n28);
        let eq61_e1001_d_n29: f64 = (eq61_e996_d_n29 + eq61_e1000_d_n29);
        (eq61_e1001, eq61_e1001_d_n0, eq61_e1001_d_n1, eq61_e1001_d_n2, eq61_e1001_d_n3, eq61_e1001_d_n4, eq61_e1001_d_n5, eq61_e1001_d_n6, eq61_e1001_d_n7, eq61_e1001_d_n8, eq61_e1001_d_n9, eq61_e1001_d_n10, eq61_e1001_d_n11, eq61_e1001_d_n12, eq61_e1001_d_n13, eq61_e1001_d_n14, eq61_e1001_d_n15, eq61_e1001_d_n16, eq61_e1001_d_n17, eq61_e1001_d_n18, eq61_e1001_d_n19, eq61_e1001_d_n20, eq61_e1001_d_n21, eq61_e1001_d_n22, eq61_e1001_d_n23, eq61_e1001_d_n24, eq61_e1001_d_n25, eq61_e1001_d_n26, eq61_e1001_d_n27, eq61_e1001_d_n28, eq61_e1001_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1003;
        let eq61_node_derivatives: [f64; 30] = [eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29];
        let eq61_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            self.multiplicity * (eq61_value),
            &nodes,
            &eq61_node_derivatives,
            &branches,
            &eq61_branch_derivatives,
            self.multiplicity,
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
        let (eq62_e1007,) = {
    if (s.v[760] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e1007;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[15]),
            self.multiplicity * (eq62_value),
            &[
            ],
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq63_e1017, eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29,) = {
    if (s.v[760] != 0.0) {
        let eq63_e1010: f64 = self.eval_ddt(42, s.v[201]);
        let eq63_e1010_d_n0: f64 = self.ddt_jacobian(s.dn[201][0]);
        let eq63_e1010_d_n1: f64 = self.ddt_jacobian(s.dn[201][1]);
        let eq63_e1010_d_n2: f64 = self.ddt_jacobian(s.dn[201][2]);
        let eq63_e1010_d_n3: f64 = self.ddt_jacobian(s.dn[201][3]);
        let eq63_e1010_d_n4: f64 = self.ddt_jacobian(s.dn[201][4]);
        let eq63_e1010_d_n5: f64 = self.ddt_jacobian(s.dn[201][5]);
        let eq63_e1010_d_n6: f64 = self.ddt_jacobian(s.dn[201][6]);
        let eq63_e1010_d_n7: f64 = self.ddt_jacobian(s.dn[201][7]);
        let eq63_e1010_d_n8: f64 = self.ddt_jacobian(s.dn[201][8]);
        let eq63_e1010_d_n9: f64 = self.ddt_jacobian(s.dn[201][9]);
        let eq63_e1010_d_n10: f64 = self.ddt_jacobian(s.dn[201][10]);
        let eq63_e1010_d_n11: f64 = self.ddt_jacobian(s.dn[201][11]);
        let eq63_e1010_d_n12: f64 = self.ddt_jacobian(s.dn[201][12]);
        let eq63_e1010_d_n13: f64 = self.ddt_jacobian(s.dn[201][13]);
        let eq63_e1010_d_n14: f64 = self.ddt_jacobian(s.dn[201][14]);
        let eq63_e1010_d_n15: f64 = self.ddt_jacobian(s.dn[201][15]);
        let eq63_e1010_d_n16: f64 = self.ddt_jacobian(s.dn[201][16]);
        let eq63_e1010_d_n17: f64 = self.ddt_jacobian(s.dn[201][17]);
        let eq63_e1010_d_n18: f64 = self.ddt_jacobian(s.dn[201][18]);
        let eq63_e1010_d_n19: f64 = self.ddt_jacobian(s.dn[201][19]);
        let eq63_e1010_d_n20: f64 = self.ddt_jacobian(s.dn[201][20]);
        let eq63_e1010_d_n21: f64 = self.ddt_jacobian(s.dn[201][21]);
        let eq63_e1010_d_n22: f64 = self.ddt_jacobian(s.dn[201][22]);
        let eq63_e1010_d_n23: f64 = self.ddt_jacobian(s.dn[201][23]);
        let eq63_e1010_d_n24: f64 = self.ddt_jacobian(s.dn[201][24]);
        let eq63_e1010_d_n25: f64 = self.ddt_jacobian(s.dn[201][25]);
        let eq63_e1010_d_n26: f64 = self.ddt_jacobian(s.dn[201][26]);
        let eq63_e1010_d_n27: f64 = self.ddt_jacobian(s.dn[201][27]);
        let eq63_e1010_d_n28: f64 = self.ddt_jacobian(s.dn[201][28]);
        let eq63_e1010_d_n29: f64 = self.ddt_jacobian(s.dn[201][29]);
        let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));
        let eq63_e1013_d_n7: f64 = p.p355;
        let eq63_e1013_d_n9: f64 = (-p.p355);
        let eq63_e1014: f64 = self.eval_ddt(43, eq63_e1013);
        let eq63_e1014_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n7: f64 = self.ddt_jacobian(eq63_e1013_d_n7);
        let eq63_e1014_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n9: f64 = self.ddt_jacobian(eq63_e1013_d_n9);
        let eq63_e1014_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq63_e1014_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq63_e1015: f64 = (eq63_e1010 + eq63_e1014);
        let eq63_e1015_d_n0: f64 = (eq63_e1010_d_n0 + eq63_e1014_d_n0);
        let eq63_e1015_d_n1: f64 = (eq63_e1010_d_n1 + eq63_e1014_d_n1);
        let eq63_e1015_d_n2: f64 = (eq63_e1010_d_n2 + eq63_e1014_d_n2);
        let eq63_e1015_d_n3: f64 = (eq63_e1010_d_n3 + eq63_e1014_d_n3);
        let eq63_e1015_d_n4: f64 = (eq63_e1010_d_n4 + eq63_e1014_d_n4);
        let eq63_e1015_d_n5: f64 = (eq63_e1010_d_n5 + eq63_e1014_d_n5);
        let eq63_e1015_d_n6: f64 = (eq63_e1010_d_n6 + eq63_e1014_d_n6);
        let eq63_e1015_d_n7: f64 = (eq63_e1010_d_n7 + eq63_e1014_d_n7);
        let eq63_e1015_d_n8: f64 = (eq63_e1010_d_n8 + eq63_e1014_d_n8);
        let eq63_e1015_d_n9: f64 = (eq63_e1010_d_n9 + eq63_e1014_d_n9);
        let eq63_e1015_d_n10: f64 = (eq63_e1010_d_n10 + eq63_e1014_d_n10);
        let eq63_e1015_d_n11: f64 = (eq63_e1010_d_n11 + eq63_e1014_d_n11);
        let eq63_e1015_d_n12: f64 = (eq63_e1010_d_n12 + eq63_e1014_d_n12);
        let eq63_e1015_d_n13: f64 = (eq63_e1010_d_n13 + eq63_e1014_d_n13);
        let eq63_e1015_d_n14: f64 = (eq63_e1010_d_n14 + eq63_e1014_d_n14);
        let eq63_e1015_d_n15: f64 = (eq63_e1010_d_n15 + eq63_e1014_d_n15);
        let eq63_e1015_d_n16: f64 = (eq63_e1010_d_n16 + eq63_e1014_d_n16);
        let eq63_e1015_d_n17: f64 = (eq63_e1010_d_n17 + eq63_e1014_d_n17);
        let eq63_e1015_d_n18: f64 = (eq63_e1010_d_n18 + eq63_e1014_d_n18);
        let eq63_e1015_d_n19: f64 = (eq63_e1010_d_n19 + eq63_e1014_d_n19);
        let eq63_e1015_d_n20: f64 = (eq63_e1010_d_n20 + eq63_e1014_d_n20);
        let eq63_e1015_d_n21: f64 = (eq63_e1010_d_n21 + eq63_e1014_d_n21);
        let eq63_e1015_d_n22: f64 = (eq63_e1010_d_n22 + eq63_e1014_d_n22);
        let eq63_e1015_d_n23: f64 = (eq63_e1010_d_n23 + eq63_e1014_d_n23);
        let eq63_e1015_d_n24: f64 = (eq63_e1010_d_n24 + eq63_e1014_d_n24);
        let eq63_e1015_d_n25: f64 = (eq63_e1010_d_n25 + eq63_e1014_d_n25);
        let eq63_e1015_d_n26: f64 = (eq63_e1010_d_n26 + eq63_e1014_d_n26);
        let eq63_e1015_d_n27: f64 = (eq63_e1010_d_n27 + eq63_e1014_d_n27);
        let eq63_e1015_d_n28: f64 = (eq63_e1010_d_n28 + eq63_e1014_d_n28);
        let eq63_e1015_d_n29: f64 = (eq63_e1010_d_n29 + eq63_e1014_d_n29);
        (eq63_e1015, eq63_e1015_d_n0, eq63_e1015_d_n1, eq63_e1015_d_n2, eq63_e1015_d_n3, eq63_e1015_d_n4, eq63_e1015_d_n5, eq63_e1015_d_n6, eq63_e1015_d_n7, eq63_e1015_d_n8, eq63_e1015_d_n9, eq63_e1015_d_n10, eq63_e1015_d_n11, eq63_e1015_d_n12, eq63_e1015_d_n13, eq63_e1015_d_n14, eq63_e1015_d_n15, eq63_e1015_d_n16, eq63_e1015_d_n17, eq63_e1015_d_n18, eq63_e1015_d_n19, eq63_e1015_d_n20, eq63_e1015_d_n21, eq63_e1015_d_n22, eq63_e1015_d_n23, eq63_e1015_d_n24, eq63_e1015_d_n25, eq63_e1015_d_n26, eq63_e1015_d_n27, eq63_e1015_d_n28, eq63_e1015_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1017;
        let eq63_node_derivatives: [f64; 30] = [eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29];
        let eq63_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq64_e1028, eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29,) = {
    if (!(s.v[760] != 0.0)) {
        let eq64_e1021: f64 = self.eval_ddt(44, s.v[197]);
        let eq64_e1021_d_n0: f64 = self.ddt_jacobian(s.dn[197][0]);
        let eq64_e1021_d_n1: f64 = self.ddt_jacobian(s.dn[197][1]);
        let eq64_e1021_d_n2: f64 = self.ddt_jacobian(s.dn[197][2]);
        let eq64_e1021_d_n3: f64 = self.ddt_jacobian(s.dn[197][3]);
        let eq64_e1021_d_n4: f64 = self.ddt_jacobian(s.dn[197][4]);
        let eq64_e1021_d_n5: f64 = self.ddt_jacobian(s.dn[197][5]);
        let eq64_e1021_d_n6: f64 = self.ddt_jacobian(s.dn[197][6]);
        let eq64_e1021_d_n7: f64 = self.ddt_jacobian(s.dn[197][7]);
        let eq64_e1021_d_n8: f64 = self.ddt_jacobian(s.dn[197][8]);
        let eq64_e1021_d_n9: f64 = self.ddt_jacobian(s.dn[197][9]);
        let eq64_e1021_d_n10: f64 = self.ddt_jacobian(s.dn[197][10]);
        let eq64_e1021_d_n11: f64 = self.ddt_jacobian(s.dn[197][11]);
        let eq64_e1021_d_n12: f64 = self.ddt_jacobian(s.dn[197][12]);
        let eq64_e1021_d_n13: f64 = self.ddt_jacobian(s.dn[197][13]);
        let eq64_e1021_d_n14: f64 = self.ddt_jacobian(s.dn[197][14]);
        let eq64_e1021_d_n15: f64 = self.ddt_jacobian(s.dn[197][15]);
        let eq64_e1021_d_n16: f64 = self.ddt_jacobian(s.dn[197][16]);
        let eq64_e1021_d_n17: f64 = self.ddt_jacobian(s.dn[197][17]);
        let eq64_e1021_d_n18: f64 = self.ddt_jacobian(s.dn[197][18]);
        let eq64_e1021_d_n19: f64 = self.ddt_jacobian(s.dn[197][19]);
        let eq64_e1021_d_n20: f64 = self.ddt_jacobian(s.dn[197][20]);
        let eq64_e1021_d_n21: f64 = self.ddt_jacobian(s.dn[197][21]);
        let eq64_e1021_d_n22: f64 = self.ddt_jacobian(s.dn[197][22]);
        let eq64_e1021_d_n23: f64 = self.ddt_jacobian(s.dn[197][23]);
        let eq64_e1021_d_n24: f64 = self.ddt_jacobian(s.dn[197][24]);
        let eq64_e1021_d_n25: f64 = self.ddt_jacobian(s.dn[197][25]);
        let eq64_e1021_d_n26: f64 = self.ddt_jacobian(s.dn[197][26]);
        let eq64_e1021_d_n27: f64 = self.ddt_jacobian(s.dn[197][27]);
        let eq64_e1021_d_n28: f64 = self.ddt_jacobian(s.dn[197][28]);
        let eq64_e1021_d_n29: f64 = self.ddt_jacobian(s.dn[197][29]);
        let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));
        let eq64_e1024_d_n2: f64 = p.p355;
        let eq64_e1024_d_n14: f64 = (-p.p355);
        let eq64_e1025: f64 = self.eval_ddt(45, eq64_e1024);
        let eq64_e1025_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n2: f64 = self.ddt_jacobian(eq64_e1024_d_n2);
        let eq64_e1025_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n14: f64 = self.ddt_jacobian(eq64_e1024_d_n14);
        let eq64_e1025_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq64_e1025_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq64_e1026: f64 = (eq64_e1021 + eq64_e1025);
        let eq64_e1026_d_n0: f64 = (eq64_e1021_d_n0 + eq64_e1025_d_n0);
        let eq64_e1026_d_n1: f64 = (eq64_e1021_d_n1 + eq64_e1025_d_n1);
        let eq64_e1026_d_n2: f64 = (eq64_e1021_d_n2 + eq64_e1025_d_n2);
        let eq64_e1026_d_n3: f64 = (eq64_e1021_d_n3 + eq64_e1025_d_n3);
        let eq64_e1026_d_n4: f64 = (eq64_e1021_d_n4 + eq64_e1025_d_n4);
        let eq64_e1026_d_n5: f64 = (eq64_e1021_d_n5 + eq64_e1025_d_n5);
        let eq64_e1026_d_n6: f64 = (eq64_e1021_d_n6 + eq64_e1025_d_n6);
        let eq64_e1026_d_n7: f64 = (eq64_e1021_d_n7 + eq64_e1025_d_n7);
        let eq64_e1026_d_n8: f64 = (eq64_e1021_d_n8 + eq64_e1025_d_n8);
        let eq64_e1026_d_n9: f64 = (eq64_e1021_d_n9 + eq64_e1025_d_n9);
        let eq64_e1026_d_n10: f64 = (eq64_e1021_d_n10 + eq64_e1025_d_n10);
        let eq64_e1026_d_n11: f64 = (eq64_e1021_d_n11 + eq64_e1025_d_n11);
        let eq64_e1026_d_n12: f64 = (eq64_e1021_d_n12 + eq64_e1025_d_n12);
        let eq64_e1026_d_n13: f64 = (eq64_e1021_d_n13 + eq64_e1025_d_n13);
        let eq64_e1026_d_n14: f64 = (eq64_e1021_d_n14 + eq64_e1025_d_n14);
        let eq64_e1026_d_n15: f64 = (eq64_e1021_d_n15 + eq64_e1025_d_n15);
        let eq64_e1026_d_n16: f64 = (eq64_e1021_d_n16 + eq64_e1025_d_n16);
        let eq64_e1026_d_n17: f64 = (eq64_e1021_d_n17 + eq64_e1025_d_n17);
        let eq64_e1026_d_n18: f64 = (eq64_e1021_d_n18 + eq64_e1025_d_n18);
        let eq64_e1026_d_n19: f64 = (eq64_e1021_d_n19 + eq64_e1025_d_n19);
        let eq64_e1026_d_n20: f64 = (eq64_e1021_d_n20 + eq64_e1025_d_n20);
        let eq64_e1026_d_n21: f64 = (eq64_e1021_d_n21 + eq64_e1025_d_n21);
        let eq64_e1026_d_n22: f64 = (eq64_e1021_d_n22 + eq64_e1025_d_n22);
        let eq64_e1026_d_n23: f64 = (eq64_e1021_d_n23 + eq64_e1025_d_n23);
        let eq64_e1026_d_n24: f64 = (eq64_e1021_d_n24 + eq64_e1025_d_n24);
        let eq64_e1026_d_n25: f64 = (eq64_e1021_d_n25 + eq64_e1025_d_n25);
        let eq64_e1026_d_n26: f64 = (eq64_e1021_d_n26 + eq64_e1025_d_n26);
        let eq64_e1026_d_n27: f64 = (eq64_e1021_d_n27 + eq64_e1025_d_n27);
        let eq64_e1026_d_n28: f64 = (eq64_e1021_d_n28 + eq64_e1025_d_n28);
        let eq64_e1026_d_n29: f64 = (eq64_e1021_d_n29 + eq64_e1025_d_n29);
        (eq64_e1026, eq64_e1026_d_n0, eq64_e1026_d_n1, eq64_e1026_d_n2, eq64_e1026_d_n3, eq64_e1026_d_n4, eq64_e1026_d_n5, eq64_e1026_d_n6, eq64_e1026_d_n7, eq64_e1026_d_n8, eq64_e1026_d_n9, eq64_e1026_d_n10, eq64_e1026_d_n11, eq64_e1026_d_n12, eq64_e1026_d_n13, eq64_e1026_d_n14, eq64_e1026_d_n15, eq64_e1026_d_n16, eq64_e1026_d_n17, eq64_e1026_d_n18, eq64_e1026_d_n19, eq64_e1026_d_n20, eq64_e1026_d_n21, eq64_e1026_d_n22, eq64_e1026_d_n23, eq64_e1026_d_n24, eq64_e1026_d_n25, eq64_e1026_d_n26, eq64_e1026_d_n27, eq64_e1026_d_n28, eq64_e1026_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e1028;
        let eq64_node_derivatives: [f64; 30] = [eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29];
        let eq64_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            self.multiplicity * (eq64_value),
            &nodes,
            &eq64_node_derivatives,
            &branches,
            &eq64_branch_derivatives,
            self.multiplicity,
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq65_e1039, eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29,) = {
    if (!(s.v[760] != 0.0)) {
        let eq65_e1032: f64 = self.eval_ddt(46, s.v[198]);
        let eq65_e1032_d_n0: f64 = self.ddt_jacobian(s.dn[198][0]);
        let eq65_e1032_d_n1: f64 = self.ddt_jacobian(s.dn[198][1]);
        let eq65_e1032_d_n2: f64 = self.ddt_jacobian(s.dn[198][2]);
        let eq65_e1032_d_n3: f64 = self.ddt_jacobian(s.dn[198][3]);
        let eq65_e1032_d_n4: f64 = self.ddt_jacobian(s.dn[198][4]);
        let eq65_e1032_d_n5: f64 = self.ddt_jacobian(s.dn[198][5]);
        let eq65_e1032_d_n6: f64 = self.ddt_jacobian(s.dn[198][6]);
        let eq65_e1032_d_n7: f64 = self.ddt_jacobian(s.dn[198][7]);
        let eq65_e1032_d_n8: f64 = self.ddt_jacobian(s.dn[198][8]);
        let eq65_e1032_d_n9: f64 = self.ddt_jacobian(s.dn[198][9]);
        let eq65_e1032_d_n10: f64 = self.ddt_jacobian(s.dn[198][10]);
        let eq65_e1032_d_n11: f64 = self.ddt_jacobian(s.dn[198][11]);
        let eq65_e1032_d_n12: f64 = self.ddt_jacobian(s.dn[198][12]);
        let eq65_e1032_d_n13: f64 = self.ddt_jacobian(s.dn[198][13]);
        let eq65_e1032_d_n14: f64 = self.ddt_jacobian(s.dn[198][14]);
        let eq65_e1032_d_n15: f64 = self.ddt_jacobian(s.dn[198][15]);
        let eq65_e1032_d_n16: f64 = self.ddt_jacobian(s.dn[198][16]);
        let eq65_e1032_d_n17: f64 = self.ddt_jacobian(s.dn[198][17]);
        let eq65_e1032_d_n18: f64 = self.ddt_jacobian(s.dn[198][18]);
        let eq65_e1032_d_n19: f64 = self.ddt_jacobian(s.dn[198][19]);
        let eq65_e1032_d_n20: f64 = self.ddt_jacobian(s.dn[198][20]);
        let eq65_e1032_d_n21: f64 = self.ddt_jacobian(s.dn[198][21]);
        let eq65_e1032_d_n22: f64 = self.ddt_jacobian(s.dn[198][22]);
        let eq65_e1032_d_n23: f64 = self.ddt_jacobian(s.dn[198][23]);
        let eq65_e1032_d_n24: f64 = self.ddt_jacobian(s.dn[198][24]);
        let eq65_e1032_d_n25: f64 = self.ddt_jacobian(s.dn[198][25]);
        let eq65_e1032_d_n26: f64 = self.ddt_jacobian(s.dn[198][26]);
        let eq65_e1032_d_n27: f64 = self.ddt_jacobian(s.dn[198][27]);
        let eq65_e1032_d_n28: f64 = self.ddt_jacobian(s.dn[198][28]);
        let eq65_e1032_d_n29: f64 = self.ddt_jacobian(s.dn[198][29]);
        let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));
        let eq65_e1035_d_n2: f64 = p.p355;
        let eq65_e1035_d_n15: f64 = (-p.p355);
        let eq65_e1036: f64 = self.eval_ddt(47, eq65_e1035);
        let eq65_e1036_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n2: f64 = self.ddt_jacobian(eq65_e1035_d_n2);
        let eq65_e1036_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n15: f64 = self.ddt_jacobian(eq65_e1035_d_n15);
        let eq65_e1036_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq65_e1036_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq65_e1037: f64 = (eq65_e1032 + eq65_e1036);
        let eq65_e1037_d_n0: f64 = (eq65_e1032_d_n0 + eq65_e1036_d_n0);
        let eq65_e1037_d_n1: f64 = (eq65_e1032_d_n1 + eq65_e1036_d_n1);
        let eq65_e1037_d_n2: f64 = (eq65_e1032_d_n2 + eq65_e1036_d_n2);
        let eq65_e1037_d_n3: f64 = (eq65_e1032_d_n3 + eq65_e1036_d_n3);
        let eq65_e1037_d_n4: f64 = (eq65_e1032_d_n4 + eq65_e1036_d_n4);
        let eq65_e1037_d_n5: f64 = (eq65_e1032_d_n5 + eq65_e1036_d_n5);
        let eq65_e1037_d_n6: f64 = (eq65_e1032_d_n6 + eq65_e1036_d_n6);
        let eq65_e1037_d_n7: f64 = (eq65_e1032_d_n7 + eq65_e1036_d_n7);
        let eq65_e1037_d_n8: f64 = (eq65_e1032_d_n8 + eq65_e1036_d_n8);
        let eq65_e1037_d_n9: f64 = (eq65_e1032_d_n9 + eq65_e1036_d_n9);
        let eq65_e1037_d_n10: f64 = (eq65_e1032_d_n10 + eq65_e1036_d_n10);
        let eq65_e1037_d_n11: f64 = (eq65_e1032_d_n11 + eq65_e1036_d_n11);
        let eq65_e1037_d_n12: f64 = (eq65_e1032_d_n12 + eq65_e1036_d_n12);
        let eq65_e1037_d_n13: f64 = (eq65_e1032_d_n13 + eq65_e1036_d_n13);
        let eq65_e1037_d_n14: f64 = (eq65_e1032_d_n14 + eq65_e1036_d_n14);
        let eq65_e1037_d_n15: f64 = (eq65_e1032_d_n15 + eq65_e1036_d_n15);
        let eq65_e1037_d_n16: f64 = (eq65_e1032_d_n16 + eq65_e1036_d_n16);
        let eq65_e1037_d_n17: f64 = (eq65_e1032_d_n17 + eq65_e1036_d_n17);
        let eq65_e1037_d_n18: f64 = (eq65_e1032_d_n18 + eq65_e1036_d_n18);
        let eq65_e1037_d_n19: f64 = (eq65_e1032_d_n19 + eq65_e1036_d_n19);
        let eq65_e1037_d_n20: f64 = (eq65_e1032_d_n20 + eq65_e1036_d_n20);
        let eq65_e1037_d_n21: f64 = (eq65_e1032_d_n21 + eq65_e1036_d_n21);
        let eq65_e1037_d_n22: f64 = (eq65_e1032_d_n22 + eq65_e1036_d_n22);
        let eq65_e1037_d_n23: f64 = (eq65_e1032_d_n23 + eq65_e1036_d_n23);
        let eq65_e1037_d_n24: f64 = (eq65_e1032_d_n24 + eq65_e1036_d_n24);
        let eq65_e1037_d_n25: f64 = (eq65_e1032_d_n25 + eq65_e1036_d_n25);
        let eq65_e1037_d_n26: f64 = (eq65_e1032_d_n26 + eq65_e1036_d_n26);
        let eq65_e1037_d_n27: f64 = (eq65_e1032_d_n27 + eq65_e1036_d_n27);
        let eq65_e1037_d_n28: f64 = (eq65_e1032_d_n28 + eq65_e1036_d_n28);
        let eq65_e1037_d_n29: f64 = (eq65_e1032_d_n29 + eq65_e1036_d_n29);
        (eq65_e1037, eq65_e1037_d_n0, eq65_e1037_d_n1, eq65_e1037_d_n2, eq65_e1037_d_n3, eq65_e1037_d_n4, eq65_e1037_d_n5, eq65_e1037_d_n6, eq65_e1037_d_n7, eq65_e1037_d_n8, eq65_e1037_d_n9, eq65_e1037_d_n10, eq65_e1037_d_n11, eq65_e1037_d_n12, eq65_e1037_d_n13, eq65_e1037_d_n14, eq65_e1037_d_n15, eq65_e1037_d_n16, eq65_e1037_d_n17, eq65_e1037_d_n18, eq65_e1037_d_n19, eq65_e1037_d_n20, eq65_e1037_d_n21, eq65_e1037_d_n22, eq65_e1037_d_n23, eq65_e1037_d_n24, eq65_e1037_d_n25, eq65_e1037_d_n26, eq65_e1037_d_n27, eq65_e1037_d_n28, eq65_e1037_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1039;
        let eq65_node_derivatives: [f64; 30] = [eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29];
        let eq65_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            self.multiplicity * (eq65_value),
            &nodes,
            &eq65_node_derivatives,
            &branches,
            &eq65_branch_derivatives,
            self.multiplicity,
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq66_e1050, eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29,) = {
    if (!(s.v[760] != 0.0)) {
        let eq66_e1043: f64 = self.eval_ddt(48, s.v[199]);
        let eq66_e1043_d_n0: f64 = self.ddt_jacobian(s.dn[199][0]);
        let eq66_e1043_d_n1: f64 = self.ddt_jacobian(s.dn[199][1]);
        let eq66_e1043_d_n2: f64 = self.ddt_jacobian(s.dn[199][2]);
        let eq66_e1043_d_n3: f64 = self.ddt_jacobian(s.dn[199][3]);
        let eq66_e1043_d_n4: f64 = self.ddt_jacobian(s.dn[199][4]);
        let eq66_e1043_d_n5: f64 = self.ddt_jacobian(s.dn[199][5]);
        let eq66_e1043_d_n6: f64 = self.ddt_jacobian(s.dn[199][6]);
        let eq66_e1043_d_n7: f64 = self.ddt_jacobian(s.dn[199][7]);
        let eq66_e1043_d_n8: f64 = self.ddt_jacobian(s.dn[199][8]);
        let eq66_e1043_d_n9: f64 = self.ddt_jacobian(s.dn[199][9]);
        let eq66_e1043_d_n10: f64 = self.ddt_jacobian(s.dn[199][10]);
        let eq66_e1043_d_n11: f64 = self.ddt_jacobian(s.dn[199][11]);
        let eq66_e1043_d_n12: f64 = self.ddt_jacobian(s.dn[199][12]);
        let eq66_e1043_d_n13: f64 = self.ddt_jacobian(s.dn[199][13]);
        let eq66_e1043_d_n14: f64 = self.ddt_jacobian(s.dn[199][14]);
        let eq66_e1043_d_n15: f64 = self.ddt_jacobian(s.dn[199][15]);
        let eq66_e1043_d_n16: f64 = self.ddt_jacobian(s.dn[199][16]);
        let eq66_e1043_d_n17: f64 = self.ddt_jacobian(s.dn[199][17]);
        let eq66_e1043_d_n18: f64 = self.ddt_jacobian(s.dn[199][18]);
        let eq66_e1043_d_n19: f64 = self.ddt_jacobian(s.dn[199][19]);
        let eq66_e1043_d_n20: f64 = self.ddt_jacobian(s.dn[199][20]);
        let eq66_e1043_d_n21: f64 = self.ddt_jacobian(s.dn[199][21]);
        let eq66_e1043_d_n22: f64 = self.ddt_jacobian(s.dn[199][22]);
        let eq66_e1043_d_n23: f64 = self.ddt_jacobian(s.dn[199][23]);
        let eq66_e1043_d_n24: f64 = self.ddt_jacobian(s.dn[199][24]);
        let eq66_e1043_d_n25: f64 = self.ddt_jacobian(s.dn[199][25]);
        let eq66_e1043_d_n26: f64 = self.ddt_jacobian(s.dn[199][26]);
        let eq66_e1043_d_n27: f64 = self.ddt_jacobian(s.dn[199][27]);
        let eq66_e1043_d_n28: f64 = self.ddt_jacobian(s.dn[199][28]);
        let eq66_e1043_d_n29: f64 = self.ddt_jacobian(s.dn[199][29]);
        let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));
        let eq66_e1046_d_n7: f64 = p.p355;
        let eq66_e1046_d_n14: f64 = (-p.p355);
        let eq66_e1047: f64 = self.eval_ddt(49, eq66_e1046);
        let eq66_e1047_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n7: f64 = self.ddt_jacobian(eq66_e1046_d_n7);
        let eq66_e1047_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n14: f64 = self.ddt_jacobian(eq66_e1046_d_n14);
        let eq66_e1047_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq66_e1047_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq66_e1048: f64 = (eq66_e1043 + eq66_e1047);
        let eq66_e1048_d_n0: f64 = (eq66_e1043_d_n0 + eq66_e1047_d_n0);
        let eq66_e1048_d_n1: f64 = (eq66_e1043_d_n1 + eq66_e1047_d_n1);
        let eq66_e1048_d_n2: f64 = (eq66_e1043_d_n2 + eq66_e1047_d_n2);
        let eq66_e1048_d_n3: f64 = (eq66_e1043_d_n3 + eq66_e1047_d_n3);
        let eq66_e1048_d_n4: f64 = (eq66_e1043_d_n4 + eq66_e1047_d_n4);
        let eq66_e1048_d_n5: f64 = (eq66_e1043_d_n5 + eq66_e1047_d_n5);
        let eq66_e1048_d_n6: f64 = (eq66_e1043_d_n6 + eq66_e1047_d_n6);
        let eq66_e1048_d_n7: f64 = (eq66_e1043_d_n7 + eq66_e1047_d_n7);
        let eq66_e1048_d_n8: f64 = (eq66_e1043_d_n8 + eq66_e1047_d_n8);
        let eq66_e1048_d_n9: f64 = (eq66_e1043_d_n9 + eq66_e1047_d_n9);
        let eq66_e1048_d_n10: f64 = (eq66_e1043_d_n10 + eq66_e1047_d_n10);
        let eq66_e1048_d_n11: f64 = (eq66_e1043_d_n11 + eq66_e1047_d_n11);
        let eq66_e1048_d_n12: f64 = (eq66_e1043_d_n12 + eq66_e1047_d_n12);
        let eq66_e1048_d_n13: f64 = (eq66_e1043_d_n13 + eq66_e1047_d_n13);
        let eq66_e1048_d_n14: f64 = (eq66_e1043_d_n14 + eq66_e1047_d_n14);
        let eq66_e1048_d_n15: f64 = (eq66_e1043_d_n15 + eq66_e1047_d_n15);
        let eq66_e1048_d_n16: f64 = (eq66_e1043_d_n16 + eq66_e1047_d_n16);
        let eq66_e1048_d_n17: f64 = (eq66_e1043_d_n17 + eq66_e1047_d_n17);
        let eq66_e1048_d_n18: f64 = (eq66_e1043_d_n18 + eq66_e1047_d_n18);
        let eq66_e1048_d_n19: f64 = (eq66_e1043_d_n19 + eq66_e1047_d_n19);
        let eq66_e1048_d_n20: f64 = (eq66_e1043_d_n20 + eq66_e1047_d_n20);
        let eq66_e1048_d_n21: f64 = (eq66_e1043_d_n21 + eq66_e1047_d_n21);
        let eq66_e1048_d_n22: f64 = (eq66_e1043_d_n22 + eq66_e1047_d_n22);
        let eq66_e1048_d_n23: f64 = (eq66_e1043_d_n23 + eq66_e1047_d_n23);
        let eq66_e1048_d_n24: f64 = (eq66_e1043_d_n24 + eq66_e1047_d_n24);
        let eq66_e1048_d_n25: f64 = (eq66_e1043_d_n25 + eq66_e1047_d_n25);
        let eq66_e1048_d_n26: f64 = (eq66_e1043_d_n26 + eq66_e1047_d_n26);
        let eq66_e1048_d_n27: f64 = (eq66_e1043_d_n27 + eq66_e1047_d_n27);
        let eq66_e1048_d_n28: f64 = (eq66_e1043_d_n28 + eq66_e1047_d_n28);
        let eq66_e1048_d_n29: f64 = (eq66_e1043_d_n29 + eq66_e1047_d_n29);
        (eq66_e1048, eq66_e1048_d_n0, eq66_e1048_d_n1, eq66_e1048_d_n2, eq66_e1048_d_n3, eq66_e1048_d_n4, eq66_e1048_d_n5, eq66_e1048_d_n6, eq66_e1048_d_n7, eq66_e1048_d_n8, eq66_e1048_d_n9, eq66_e1048_d_n10, eq66_e1048_d_n11, eq66_e1048_d_n12, eq66_e1048_d_n13, eq66_e1048_d_n14, eq66_e1048_d_n15, eq66_e1048_d_n16, eq66_e1048_d_n17, eq66_e1048_d_n18, eq66_e1048_d_n19, eq66_e1048_d_n20, eq66_e1048_d_n21, eq66_e1048_d_n22, eq66_e1048_d_n23, eq66_e1048_d_n24, eq66_e1048_d_n25, eq66_e1048_d_n26, eq66_e1048_d_n27, eq66_e1048_d_n28, eq66_e1048_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1050;
        let eq66_node_derivatives: [f64; 30] = [eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29];
        let eq66_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            self.multiplicity * (eq66_value),
            &nodes,
            &eq66_node_derivatives,
            &branches,
            &eq66_branch_derivatives,
            self.multiplicity,
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
        let (eq67_e1055,) = {
    if (!(s.v[760] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1055;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[15]),
            self.multiplicity * (eq67_value),
            &[
            ],
        );
    }
}
