#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_151_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq151_e1915, eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n10, eq151_e1915_d_n11, eq151_e1915_d_n12, eq151_e1915_d_n13, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22,) = {
    if (((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) && (s.v[584] != 0.0)) {
        let eq151_e1912: f64 = self.eval_ddt(50, s.v[252]);
        let eq151_e1912_d_n0: f64 = self.ddt_jacobian(s.dn[252][0]);
        let eq151_e1912_d_n1: f64 = self.ddt_jacobian(s.dn[252][1]);
        let eq151_e1912_d_n2: f64 = self.ddt_jacobian(s.dn[252][2]);
        let eq151_e1912_d_n3: f64 = self.ddt_jacobian(s.dn[252][3]);
        let eq151_e1912_d_n4: f64 = self.ddt_jacobian(s.dn[252][4]);
        let eq151_e1912_d_n5: f64 = self.ddt_jacobian(s.dn[252][5]);
        let eq151_e1912_d_n6: f64 = self.ddt_jacobian(s.dn[252][6]);
        let eq151_e1912_d_n7: f64 = self.ddt_jacobian(s.dn[252][7]);
        let eq151_e1912_d_n8: f64 = self.ddt_jacobian(s.dn[252][8]);
        let eq151_e1912_d_n9: f64 = self.ddt_jacobian(s.dn[252][9]);
        let eq151_e1912_d_n10: f64 = self.ddt_jacobian(s.dn[252][10]);
        let eq151_e1912_d_n11: f64 = self.ddt_jacobian(s.dn[252][11]);
        let eq151_e1912_d_n12: f64 = self.ddt_jacobian(s.dn[252][12]);
        let eq151_e1912_d_n13: f64 = self.ddt_jacobian(s.dn[252][13]);
        let eq151_e1912_d_n14: f64 = self.ddt_jacobian(s.dn[252][14]);
        let eq151_e1912_d_n15: f64 = self.ddt_jacobian(s.dn[252][15]);
        let eq151_e1912_d_n16: f64 = self.ddt_jacobian(s.dn[252][16]);
        let eq151_e1912_d_n17: f64 = self.ddt_jacobian(s.dn[252][17]);
        let eq151_e1912_d_n18: f64 = self.ddt_jacobian(s.dn[252][18]);
        let eq151_e1912_d_n19: f64 = self.ddt_jacobian(s.dn[252][19]);
        let eq151_e1912_d_n20: f64 = self.ddt_jacobian(s.dn[252][20]);
        let eq151_e1912_d_n21: f64 = self.ddt_jacobian(s.dn[252][21]);
        let eq151_e1912_d_n22: f64 = self.ddt_jacobian(s.dn[252][22]);
        let eq151_e1913: f64 = (p.p7 * eq151_e1912);
        let eq151_e1913_d_n0: f64 = (p.p7 * eq151_e1912_d_n0);
        let eq151_e1913_d_n1: f64 = (p.p7 * eq151_e1912_d_n1);
        let eq151_e1913_d_n2: f64 = (p.p7 * eq151_e1912_d_n2);
        let eq151_e1913_d_n3: f64 = (p.p7 * eq151_e1912_d_n3);
        let eq151_e1913_d_n4: f64 = (p.p7 * eq151_e1912_d_n4);
        let eq151_e1913_d_n5: f64 = (p.p7 * eq151_e1912_d_n5);
        let eq151_e1913_d_n6: f64 = (p.p7 * eq151_e1912_d_n6);
        let eq151_e1913_d_n7: f64 = (p.p7 * eq151_e1912_d_n7);
        let eq151_e1913_d_n8: f64 = (p.p7 * eq151_e1912_d_n8);
        let eq151_e1913_d_n9: f64 = (p.p7 * eq151_e1912_d_n9);
        let eq151_e1913_d_n10: f64 = (p.p7 * eq151_e1912_d_n10);
        let eq151_e1913_d_n11: f64 = (p.p7 * eq151_e1912_d_n11);
        let eq151_e1913_d_n12: f64 = (p.p7 * eq151_e1912_d_n12);
        let eq151_e1913_d_n13: f64 = (p.p7 * eq151_e1912_d_n13);
        let eq151_e1913_d_n14: f64 = (p.p7 * eq151_e1912_d_n14);
        let eq151_e1913_d_n15: f64 = (p.p7 * eq151_e1912_d_n15);
        let eq151_e1913_d_n16: f64 = (p.p7 * eq151_e1912_d_n16);
        let eq151_e1913_d_n17: f64 = (p.p7 * eq151_e1912_d_n17);
        let eq151_e1913_d_n18: f64 = (p.p7 * eq151_e1912_d_n18);
        let eq151_e1913_d_n19: f64 = (p.p7 * eq151_e1912_d_n19);
        let eq151_e1913_d_n20: f64 = (p.p7 * eq151_e1912_d_n20);
        let eq151_e1913_d_n21: f64 = (p.p7 * eq151_e1912_d_n21);
        let eq151_e1913_d_n22: f64 = (p.p7 * eq151_e1912_d_n22);
        (eq151_e1913, eq151_e1913_d_n0, eq151_e1913_d_n1, eq151_e1913_d_n2, eq151_e1913_d_n3, eq151_e1913_d_n4, eq151_e1913_d_n5, eq151_e1913_d_n6, eq151_e1913_d_n7, eq151_e1913_d_n8, eq151_e1913_d_n9, eq151_e1913_d_n10, eq151_e1913_d_n11, eq151_e1913_d_n12, eq151_e1913_d_n13, eq151_e1913_d_n14, eq151_e1913_d_n15, eq151_e1913_d_n16, eq151_e1913_d_n17, eq151_e1913_d_n18, eq151_e1913_d_n19, eq151_e1913_d_n20, eq151_e1913_d_n21, eq151_e1913_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_value: f64 = eq151_e1915;
        let eq151_node_derivatives: [f64; 23] = [eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n10, eq151_e1915_d_n11, eq151_e1915_d_n12, eq151_e1915_d_n13, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22];
        let eq151_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq151_value),
            &nodes,
            &eq151_node_derivatives,
            &branches,
            &eq151_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_152_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq152_e1929, eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n10, eq152_e1929_d_n11, eq152_e1929_d_n12, eq152_e1929_d_n13, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22,) = {
    if (((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) && (s.v[584] != 0.0)) {
        let eq152_e1924: f64 = (p.p7 * p.p247);
        let eq152_e1926: f64 = self.eval_ddt(51, s.v[252]);
        let eq152_e1926_d_n0: f64 = self.ddt_jacobian(s.dn[252][0]);
        let eq152_e1926_d_n1: f64 = self.ddt_jacobian(s.dn[252][1]);
        let eq152_e1926_d_n2: f64 = self.ddt_jacobian(s.dn[252][2]);
        let eq152_e1926_d_n3: f64 = self.ddt_jacobian(s.dn[252][3]);
        let eq152_e1926_d_n4: f64 = self.ddt_jacobian(s.dn[252][4]);
        let eq152_e1926_d_n5: f64 = self.ddt_jacobian(s.dn[252][5]);
        let eq152_e1926_d_n6: f64 = self.ddt_jacobian(s.dn[252][6]);
        let eq152_e1926_d_n7: f64 = self.ddt_jacobian(s.dn[252][7]);
        let eq152_e1926_d_n8: f64 = self.ddt_jacobian(s.dn[252][8]);
        let eq152_e1926_d_n9: f64 = self.ddt_jacobian(s.dn[252][9]);
        let eq152_e1926_d_n10: f64 = self.ddt_jacobian(s.dn[252][10]);
        let eq152_e1926_d_n11: f64 = self.ddt_jacobian(s.dn[252][11]);
        let eq152_e1926_d_n12: f64 = self.ddt_jacobian(s.dn[252][12]);
        let eq152_e1926_d_n13: f64 = self.ddt_jacobian(s.dn[252][13]);
        let eq152_e1926_d_n14: f64 = self.ddt_jacobian(s.dn[252][14]);
        let eq152_e1926_d_n15: f64 = self.ddt_jacobian(s.dn[252][15]);
        let eq152_e1926_d_n16: f64 = self.ddt_jacobian(s.dn[252][16]);
        let eq152_e1926_d_n17: f64 = self.ddt_jacobian(s.dn[252][17]);
        let eq152_e1926_d_n18: f64 = self.ddt_jacobian(s.dn[252][18]);
        let eq152_e1926_d_n19: f64 = self.ddt_jacobian(s.dn[252][19]);
        let eq152_e1926_d_n20: f64 = self.ddt_jacobian(s.dn[252][20]);
        let eq152_e1926_d_n21: f64 = self.ddt_jacobian(s.dn[252][21]);
        let eq152_e1926_d_n22: f64 = self.ddt_jacobian(s.dn[252][22]);
        let eq152_e1927: f64 = (eq152_e1924 * eq152_e1926);
        let eq152_e1927_d_n0: f64 = (eq152_e1924 * eq152_e1926_d_n0);
        let eq152_e1927_d_n1: f64 = (eq152_e1924 * eq152_e1926_d_n1);
        let eq152_e1927_d_n2: f64 = (eq152_e1924 * eq152_e1926_d_n2);
        let eq152_e1927_d_n3: f64 = (eq152_e1924 * eq152_e1926_d_n3);
        let eq152_e1927_d_n4: f64 = (eq152_e1924 * eq152_e1926_d_n4);
        let eq152_e1927_d_n5: f64 = (eq152_e1924 * eq152_e1926_d_n5);
        let eq152_e1927_d_n6: f64 = (eq152_e1924 * eq152_e1926_d_n6);
        let eq152_e1927_d_n7: f64 = (eq152_e1924 * eq152_e1926_d_n7);
        let eq152_e1927_d_n8: f64 = (eq152_e1924 * eq152_e1926_d_n8);
        let eq152_e1927_d_n9: f64 = (eq152_e1924 * eq152_e1926_d_n9);
        let eq152_e1927_d_n10: f64 = (eq152_e1924 * eq152_e1926_d_n10);
        let eq152_e1927_d_n11: f64 = (eq152_e1924 * eq152_e1926_d_n11);
        let eq152_e1927_d_n12: f64 = (eq152_e1924 * eq152_e1926_d_n12);
        let eq152_e1927_d_n13: f64 = (eq152_e1924 * eq152_e1926_d_n13);
        let eq152_e1927_d_n14: f64 = (eq152_e1924 * eq152_e1926_d_n14);
        let eq152_e1927_d_n15: f64 = (eq152_e1924 * eq152_e1926_d_n15);
        let eq152_e1927_d_n16: f64 = (eq152_e1924 * eq152_e1926_d_n16);
        let eq152_e1927_d_n17: f64 = (eq152_e1924 * eq152_e1926_d_n17);
        let eq152_e1927_d_n18: f64 = (eq152_e1924 * eq152_e1926_d_n18);
        let eq152_e1927_d_n19: f64 = (eq152_e1924 * eq152_e1926_d_n19);
        let eq152_e1927_d_n20: f64 = (eq152_e1924 * eq152_e1926_d_n20);
        let eq152_e1927_d_n21: f64 = (eq152_e1924 * eq152_e1926_d_n21);
        let eq152_e1927_d_n22: f64 = (eq152_e1924 * eq152_e1926_d_n22);
        (eq152_e1927, eq152_e1927_d_n0, eq152_e1927_d_n1, eq152_e1927_d_n2, eq152_e1927_d_n3, eq152_e1927_d_n4, eq152_e1927_d_n5, eq152_e1927_d_n6, eq152_e1927_d_n7, eq152_e1927_d_n8, eq152_e1927_d_n9, eq152_e1927_d_n10, eq152_e1927_d_n11, eq152_e1927_d_n12, eq152_e1927_d_n13, eq152_e1927_d_n14, eq152_e1927_d_n15, eq152_e1927_d_n16, eq152_e1927_d_n17, eq152_e1927_d_n18, eq152_e1927_d_n19, eq152_e1927_d_n20, eq152_e1927_d_n21, eq152_e1927_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_value: f64 = eq152_e1929;
        let eq152_node_derivatives: [f64; 23] = [eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n10, eq152_e1929_d_n11, eq152_e1929_d_n12, eq152_e1929_d_n13, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22];
        let eq152_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            self.multiplicity * (eq152_value),
            &nodes,
            &eq152_node_derivatives,
            &branches,
            &eq152_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_153_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq153_e1942, eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n10, eq153_e1942_d_n11, eq153_e1942_d_n12, eq153_e1942_d_n13, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22,) = {
    if (((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) && (!(s.v[584] != 0.0))) {
        let eq153_e1939: f64 = self.eval_ddt(52, s.v[252]);
        let eq153_e1939_d_n0: f64 = self.ddt_jacobian(s.dn[252][0]);
        let eq153_e1939_d_n1: f64 = self.ddt_jacobian(s.dn[252][1]);
        let eq153_e1939_d_n2: f64 = self.ddt_jacobian(s.dn[252][2]);
        let eq153_e1939_d_n3: f64 = self.ddt_jacobian(s.dn[252][3]);
        let eq153_e1939_d_n4: f64 = self.ddt_jacobian(s.dn[252][4]);
        let eq153_e1939_d_n5: f64 = self.ddt_jacobian(s.dn[252][5]);
        let eq153_e1939_d_n6: f64 = self.ddt_jacobian(s.dn[252][6]);
        let eq153_e1939_d_n7: f64 = self.ddt_jacobian(s.dn[252][7]);
        let eq153_e1939_d_n8: f64 = self.ddt_jacobian(s.dn[252][8]);
        let eq153_e1939_d_n9: f64 = self.ddt_jacobian(s.dn[252][9]);
        let eq153_e1939_d_n10: f64 = self.ddt_jacobian(s.dn[252][10]);
        let eq153_e1939_d_n11: f64 = self.ddt_jacobian(s.dn[252][11]);
        let eq153_e1939_d_n12: f64 = self.ddt_jacobian(s.dn[252][12]);
        let eq153_e1939_d_n13: f64 = self.ddt_jacobian(s.dn[252][13]);
        let eq153_e1939_d_n14: f64 = self.ddt_jacobian(s.dn[252][14]);
        let eq153_e1939_d_n15: f64 = self.ddt_jacobian(s.dn[252][15]);
        let eq153_e1939_d_n16: f64 = self.ddt_jacobian(s.dn[252][16]);
        let eq153_e1939_d_n17: f64 = self.ddt_jacobian(s.dn[252][17]);
        let eq153_e1939_d_n18: f64 = self.ddt_jacobian(s.dn[252][18]);
        let eq153_e1939_d_n19: f64 = self.ddt_jacobian(s.dn[252][19]);
        let eq153_e1939_d_n20: f64 = self.ddt_jacobian(s.dn[252][20]);
        let eq153_e1939_d_n21: f64 = self.ddt_jacobian(s.dn[252][21]);
        let eq153_e1939_d_n22: f64 = self.ddt_jacobian(s.dn[252][22]);
        let eq153_e1940: f64 = (p.p7 * eq153_e1939);
        let eq153_e1940_d_n0: f64 = (p.p7 * eq153_e1939_d_n0);
        let eq153_e1940_d_n1: f64 = (p.p7 * eq153_e1939_d_n1);
        let eq153_e1940_d_n2: f64 = (p.p7 * eq153_e1939_d_n2);
        let eq153_e1940_d_n3: f64 = (p.p7 * eq153_e1939_d_n3);
        let eq153_e1940_d_n4: f64 = (p.p7 * eq153_e1939_d_n4);
        let eq153_e1940_d_n5: f64 = (p.p7 * eq153_e1939_d_n5);
        let eq153_e1940_d_n6: f64 = (p.p7 * eq153_e1939_d_n6);
        let eq153_e1940_d_n7: f64 = (p.p7 * eq153_e1939_d_n7);
        let eq153_e1940_d_n8: f64 = (p.p7 * eq153_e1939_d_n8);
        let eq153_e1940_d_n9: f64 = (p.p7 * eq153_e1939_d_n9);
        let eq153_e1940_d_n10: f64 = (p.p7 * eq153_e1939_d_n10);
        let eq153_e1940_d_n11: f64 = (p.p7 * eq153_e1939_d_n11);
        let eq153_e1940_d_n12: f64 = (p.p7 * eq153_e1939_d_n12);
        let eq153_e1940_d_n13: f64 = (p.p7 * eq153_e1939_d_n13);
        let eq153_e1940_d_n14: f64 = (p.p7 * eq153_e1939_d_n14);
        let eq153_e1940_d_n15: f64 = (p.p7 * eq153_e1939_d_n15);
        let eq153_e1940_d_n16: f64 = (p.p7 * eq153_e1939_d_n16);
        let eq153_e1940_d_n17: f64 = (p.p7 * eq153_e1939_d_n17);
        let eq153_e1940_d_n18: f64 = (p.p7 * eq153_e1939_d_n18);
        let eq153_e1940_d_n19: f64 = (p.p7 * eq153_e1939_d_n19);
        let eq153_e1940_d_n20: f64 = (p.p7 * eq153_e1939_d_n20);
        let eq153_e1940_d_n21: f64 = (p.p7 * eq153_e1939_d_n21);
        let eq153_e1940_d_n22: f64 = (p.p7 * eq153_e1939_d_n22);
        (eq153_e1940, eq153_e1940_d_n0, eq153_e1940_d_n1, eq153_e1940_d_n2, eq153_e1940_d_n3, eq153_e1940_d_n4, eq153_e1940_d_n5, eq153_e1940_d_n6, eq153_e1940_d_n7, eq153_e1940_d_n8, eq153_e1940_d_n9, eq153_e1940_d_n10, eq153_e1940_d_n11, eq153_e1940_d_n12, eq153_e1940_d_n13, eq153_e1940_d_n14, eq153_e1940_d_n15, eq153_e1940_d_n16, eq153_e1940_d_n17, eq153_e1940_d_n18, eq153_e1940_d_n19, eq153_e1940_d_n20, eq153_e1940_d_n21, eq153_e1940_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_value: f64 = eq153_e1942;
        let eq153_node_derivatives: [f64; 23] = [eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n10, eq153_e1942_d_n11, eq153_e1942_d_n12, eq153_e1942_d_n13, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22];
        let eq153_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            self.multiplicity * (eq153_value),
            &nodes,
            &eq153_node_derivatives,
            &branches,
            &eq153_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_154_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq154_e1957, eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n10, eq154_e1957_d_n11, eq154_e1957_d_n12, eq154_e1957_d_n13, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22,) = {
    if (((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) && (!(s.v[584] != 0.0))) {
        let eq154_e1952: f64 = (p.p7 * p.p247);
        let eq154_e1954: f64 = self.eval_ddt(53, s.v[252]);
        let eq154_e1954_d_n0: f64 = self.ddt_jacobian(s.dn[252][0]);
        let eq154_e1954_d_n1: f64 = self.ddt_jacobian(s.dn[252][1]);
        let eq154_e1954_d_n2: f64 = self.ddt_jacobian(s.dn[252][2]);
        let eq154_e1954_d_n3: f64 = self.ddt_jacobian(s.dn[252][3]);
        let eq154_e1954_d_n4: f64 = self.ddt_jacobian(s.dn[252][4]);
        let eq154_e1954_d_n5: f64 = self.ddt_jacobian(s.dn[252][5]);
        let eq154_e1954_d_n6: f64 = self.ddt_jacobian(s.dn[252][6]);
        let eq154_e1954_d_n7: f64 = self.ddt_jacobian(s.dn[252][7]);
        let eq154_e1954_d_n8: f64 = self.ddt_jacobian(s.dn[252][8]);
        let eq154_e1954_d_n9: f64 = self.ddt_jacobian(s.dn[252][9]);
        let eq154_e1954_d_n10: f64 = self.ddt_jacobian(s.dn[252][10]);
        let eq154_e1954_d_n11: f64 = self.ddt_jacobian(s.dn[252][11]);
        let eq154_e1954_d_n12: f64 = self.ddt_jacobian(s.dn[252][12]);
        let eq154_e1954_d_n13: f64 = self.ddt_jacobian(s.dn[252][13]);
        let eq154_e1954_d_n14: f64 = self.ddt_jacobian(s.dn[252][14]);
        let eq154_e1954_d_n15: f64 = self.ddt_jacobian(s.dn[252][15]);
        let eq154_e1954_d_n16: f64 = self.ddt_jacobian(s.dn[252][16]);
        let eq154_e1954_d_n17: f64 = self.ddt_jacobian(s.dn[252][17]);
        let eq154_e1954_d_n18: f64 = self.ddt_jacobian(s.dn[252][18]);
        let eq154_e1954_d_n19: f64 = self.ddt_jacobian(s.dn[252][19]);
        let eq154_e1954_d_n20: f64 = self.ddt_jacobian(s.dn[252][20]);
        let eq154_e1954_d_n21: f64 = self.ddt_jacobian(s.dn[252][21]);
        let eq154_e1954_d_n22: f64 = self.ddt_jacobian(s.dn[252][22]);
        let eq154_e1955: f64 = (eq154_e1952 * eq154_e1954);
        let eq154_e1955_d_n0: f64 = (eq154_e1952 * eq154_e1954_d_n0);
        let eq154_e1955_d_n1: f64 = (eq154_e1952 * eq154_e1954_d_n1);
        let eq154_e1955_d_n2: f64 = (eq154_e1952 * eq154_e1954_d_n2);
        let eq154_e1955_d_n3: f64 = (eq154_e1952 * eq154_e1954_d_n3);
        let eq154_e1955_d_n4: f64 = (eq154_e1952 * eq154_e1954_d_n4);
        let eq154_e1955_d_n5: f64 = (eq154_e1952 * eq154_e1954_d_n5);
        let eq154_e1955_d_n6: f64 = (eq154_e1952 * eq154_e1954_d_n6);
        let eq154_e1955_d_n7: f64 = (eq154_e1952 * eq154_e1954_d_n7);
        let eq154_e1955_d_n8: f64 = (eq154_e1952 * eq154_e1954_d_n8);
        let eq154_e1955_d_n9: f64 = (eq154_e1952 * eq154_e1954_d_n9);
        let eq154_e1955_d_n10: f64 = (eq154_e1952 * eq154_e1954_d_n10);
        let eq154_e1955_d_n11: f64 = (eq154_e1952 * eq154_e1954_d_n11);
        let eq154_e1955_d_n12: f64 = (eq154_e1952 * eq154_e1954_d_n12);
        let eq154_e1955_d_n13: f64 = (eq154_e1952 * eq154_e1954_d_n13);
        let eq154_e1955_d_n14: f64 = (eq154_e1952 * eq154_e1954_d_n14);
        let eq154_e1955_d_n15: f64 = (eq154_e1952 * eq154_e1954_d_n15);
        let eq154_e1955_d_n16: f64 = (eq154_e1952 * eq154_e1954_d_n16);
        let eq154_e1955_d_n17: f64 = (eq154_e1952 * eq154_e1954_d_n17);
        let eq154_e1955_d_n18: f64 = (eq154_e1952 * eq154_e1954_d_n18);
        let eq154_e1955_d_n19: f64 = (eq154_e1952 * eq154_e1954_d_n19);
        let eq154_e1955_d_n20: f64 = (eq154_e1952 * eq154_e1954_d_n20);
        let eq154_e1955_d_n21: f64 = (eq154_e1952 * eq154_e1954_d_n21);
        let eq154_e1955_d_n22: f64 = (eq154_e1952 * eq154_e1954_d_n22);
        (eq154_e1955, eq154_e1955_d_n0, eq154_e1955_d_n1, eq154_e1955_d_n2, eq154_e1955_d_n3, eq154_e1955_d_n4, eq154_e1955_d_n5, eq154_e1955_d_n6, eq154_e1955_d_n7, eq154_e1955_d_n8, eq154_e1955_d_n9, eq154_e1955_d_n10, eq154_e1955_d_n11, eq154_e1955_d_n12, eq154_e1955_d_n13, eq154_e1955_d_n14, eq154_e1955_d_n15, eq154_e1955_d_n16, eq154_e1955_d_n17, eq154_e1955_d_n18, eq154_e1955_d_n19, eq154_e1955_d_n20, eq154_e1955_d_n21, eq154_e1955_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_value: f64 = eq154_e1957;
        let eq154_node_derivatives: [f64; 23] = [eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n10, eq154_e1957_d_n11, eq154_e1957_d_n12, eq154_e1957_d_n13, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22];
        let eq154_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq154_value),
            &nodes,
            &eq154_node_derivatives,
            &branches,
            &eq154_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_155_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq155_e1969, eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n10, eq155_e1969_d_n11, eq155_e1969_d_n12, eq155_e1969_d_n13, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22,) = {
    if ((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) {
        let eq155_e1965: f64 = (p.p252 * s.v[252]);
        let eq155_e1965_d_n0: f64 = (p.p252 * s.dn[252][0]);
        let eq155_e1965_d_n1: f64 = (p.p252 * s.dn[252][1]);
        let eq155_e1965_d_n2: f64 = (p.p252 * s.dn[252][2]);
        let eq155_e1965_d_n3: f64 = (p.p252 * s.dn[252][3]);
        let eq155_e1965_d_n4: f64 = (p.p252 * s.dn[252][4]);
        let eq155_e1965_d_n5: f64 = (p.p252 * s.dn[252][5]);
        let eq155_e1965_d_n6: f64 = (p.p252 * s.dn[252][6]);
        let eq155_e1965_d_n7: f64 = (p.p252 * s.dn[252][7]);
        let eq155_e1965_d_n8: f64 = (p.p252 * s.dn[252][8]);
        let eq155_e1965_d_n9: f64 = (p.p252 * s.dn[252][9]);
        let eq155_e1965_d_n10: f64 = (p.p252 * s.dn[252][10]);
        let eq155_e1965_d_n11: f64 = (p.p252 * s.dn[252][11]);
        let eq155_e1965_d_n12: f64 = (p.p252 * s.dn[252][12]);
        let eq155_e1965_d_n13: f64 = (p.p252 * s.dn[252][13]);
        let eq155_e1965_d_n14: f64 = (p.p252 * s.dn[252][14]);
        let eq155_e1965_d_n15: f64 = (p.p252 * s.dn[252][15]);
        let eq155_e1965_d_n16: f64 = (p.p252 * s.dn[252][16]);
        let eq155_e1965_d_n17: f64 = (p.p252 * s.dn[252][17]);
        let eq155_e1965_d_n18: f64 = (p.p252 * s.dn[252][18]);
        let eq155_e1965_d_n19: f64 = (p.p252 * s.dn[252][19]);
        let eq155_e1965_d_n20: f64 = (p.p252 * s.dn[252][20]);
        let eq155_e1965_d_n21: f64 = (p.p252 * s.dn[252][21]);
        let eq155_e1965_d_n22: f64 = (p.p252 * s.dn[252][22]);
        let eq155_e1966: f64 = self.eval_ddt(54, eq155_e1965);
        let eq155_e1966_d_n0: f64 = self.ddt_jacobian(eq155_e1965_d_n0);
        let eq155_e1966_d_n1: f64 = self.ddt_jacobian(eq155_e1965_d_n1);
        let eq155_e1966_d_n2: f64 = self.ddt_jacobian(eq155_e1965_d_n2);
        let eq155_e1966_d_n3: f64 = self.ddt_jacobian(eq155_e1965_d_n3);
        let eq155_e1966_d_n4: f64 = self.ddt_jacobian(eq155_e1965_d_n4);
        let eq155_e1966_d_n5: f64 = self.ddt_jacobian(eq155_e1965_d_n5);
        let eq155_e1966_d_n6: f64 = self.ddt_jacobian(eq155_e1965_d_n6);
        let eq155_e1966_d_n7: f64 = self.ddt_jacobian(eq155_e1965_d_n7);
        let eq155_e1966_d_n8: f64 = self.ddt_jacobian(eq155_e1965_d_n8);
        let eq155_e1966_d_n9: f64 = self.ddt_jacobian(eq155_e1965_d_n9);
        let eq155_e1966_d_n10: f64 = self.ddt_jacobian(eq155_e1965_d_n10);
        let eq155_e1966_d_n11: f64 = self.ddt_jacobian(eq155_e1965_d_n11);
        let eq155_e1966_d_n12: f64 = self.ddt_jacobian(eq155_e1965_d_n12);
        let eq155_e1966_d_n13: f64 = self.ddt_jacobian(eq155_e1965_d_n13);
        let eq155_e1966_d_n14: f64 = self.ddt_jacobian(eq155_e1965_d_n14);
        let eq155_e1966_d_n15: f64 = self.ddt_jacobian(eq155_e1965_d_n15);
        let eq155_e1966_d_n16: f64 = self.ddt_jacobian(eq155_e1965_d_n16);
        let eq155_e1966_d_n17: f64 = self.ddt_jacobian(eq155_e1965_d_n17);
        let eq155_e1966_d_n18: f64 = self.ddt_jacobian(eq155_e1965_d_n18);
        let eq155_e1966_d_n19: f64 = self.ddt_jacobian(eq155_e1965_d_n19);
        let eq155_e1966_d_n20: f64 = self.ddt_jacobian(eq155_e1965_d_n20);
        let eq155_e1966_d_n21: f64 = self.ddt_jacobian(eq155_e1965_d_n21);
        let eq155_e1966_d_n22: f64 = self.ddt_jacobian(eq155_e1965_d_n22);
        let eq155_e1967: f64 = (p.p7 * eq155_e1966);
        let eq155_e1967_d_n0: f64 = (p.p7 * eq155_e1966_d_n0);
        let eq155_e1967_d_n1: f64 = (p.p7 * eq155_e1966_d_n1);
        let eq155_e1967_d_n2: f64 = (p.p7 * eq155_e1966_d_n2);
        let eq155_e1967_d_n3: f64 = (p.p7 * eq155_e1966_d_n3);
        let eq155_e1967_d_n4: f64 = (p.p7 * eq155_e1966_d_n4);
        let eq155_e1967_d_n5: f64 = (p.p7 * eq155_e1966_d_n5);
        let eq155_e1967_d_n6: f64 = (p.p7 * eq155_e1966_d_n6);
        let eq155_e1967_d_n7: f64 = (p.p7 * eq155_e1966_d_n7);
        let eq155_e1967_d_n8: f64 = (p.p7 * eq155_e1966_d_n8);
        let eq155_e1967_d_n9: f64 = (p.p7 * eq155_e1966_d_n9);
        let eq155_e1967_d_n10: f64 = (p.p7 * eq155_e1966_d_n10);
        let eq155_e1967_d_n11: f64 = (p.p7 * eq155_e1966_d_n11);
        let eq155_e1967_d_n12: f64 = (p.p7 * eq155_e1966_d_n12);
        let eq155_e1967_d_n13: f64 = (p.p7 * eq155_e1966_d_n13);
        let eq155_e1967_d_n14: f64 = (p.p7 * eq155_e1966_d_n14);
        let eq155_e1967_d_n15: f64 = (p.p7 * eq155_e1966_d_n15);
        let eq155_e1967_d_n16: f64 = (p.p7 * eq155_e1966_d_n16);
        let eq155_e1967_d_n17: f64 = (p.p7 * eq155_e1966_d_n17);
        let eq155_e1967_d_n18: f64 = (p.p7 * eq155_e1966_d_n18);
        let eq155_e1967_d_n19: f64 = (p.p7 * eq155_e1966_d_n19);
        let eq155_e1967_d_n20: f64 = (p.p7 * eq155_e1966_d_n20);
        let eq155_e1967_d_n21: f64 = (p.p7 * eq155_e1966_d_n21);
        let eq155_e1967_d_n22: f64 = (p.p7 * eq155_e1966_d_n22);
        (eq155_e1967, eq155_e1967_d_n0, eq155_e1967_d_n1, eq155_e1967_d_n2, eq155_e1967_d_n3, eq155_e1967_d_n4, eq155_e1967_d_n5, eq155_e1967_d_n6, eq155_e1967_d_n7, eq155_e1967_d_n8, eq155_e1967_d_n9, eq155_e1967_d_n10, eq155_e1967_d_n11, eq155_e1967_d_n12, eq155_e1967_d_n13, eq155_e1967_d_n14, eq155_e1967_d_n15, eq155_e1967_d_n16, eq155_e1967_d_n17, eq155_e1967_d_n18, eq155_e1967_d_n19, eq155_e1967_d_n20, eq155_e1967_d_n21, eq155_e1967_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_value: f64 = eq155_e1969;
        let eq155_node_derivatives: [f64; 23] = [eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n10, eq155_e1969_d_n11, eq155_e1969_d_n12, eq155_e1969_d_n13, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22];
        let eq155_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            self.multiplicity * (eq155_value),
            &nodes,
            &eq155_node_derivatives,
            &branches,
            &eq155_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_156_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq156_e1978, eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n10, eq156_e1978_d_n11, eq156_e1978_d_n12, eq156_e1978_d_n13, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22,) = {
    if ((s.v[585] != 0.0) && (s.v[586] != 0.0)) {
        let eq156_e1975: f64 = self.eval_ddt(55, s.v[265]);
        let eq156_e1975_d_n0: f64 = self.ddt_jacobian(s.dn[265][0]);
        let eq156_e1975_d_n1: f64 = self.ddt_jacobian(s.dn[265][1]);
        let eq156_e1975_d_n2: f64 = self.ddt_jacobian(s.dn[265][2]);
        let eq156_e1975_d_n3: f64 = self.ddt_jacobian(s.dn[265][3]);
        let eq156_e1975_d_n4: f64 = self.ddt_jacobian(s.dn[265][4]);
        let eq156_e1975_d_n5: f64 = self.ddt_jacobian(s.dn[265][5]);
        let eq156_e1975_d_n6: f64 = self.ddt_jacobian(s.dn[265][6]);
        let eq156_e1975_d_n7: f64 = self.ddt_jacobian(s.dn[265][7]);
        let eq156_e1975_d_n8: f64 = self.ddt_jacobian(s.dn[265][8]);
        let eq156_e1975_d_n9: f64 = self.ddt_jacobian(s.dn[265][9]);
        let eq156_e1975_d_n10: f64 = self.ddt_jacobian(s.dn[265][10]);
        let eq156_e1975_d_n11: f64 = self.ddt_jacobian(s.dn[265][11]);
        let eq156_e1975_d_n12: f64 = self.ddt_jacobian(s.dn[265][12]);
        let eq156_e1975_d_n13: f64 = self.ddt_jacobian(s.dn[265][13]);
        let eq156_e1975_d_n14: f64 = self.ddt_jacobian(s.dn[265][14]);
        let eq156_e1975_d_n15: f64 = self.ddt_jacobian(s.dn[265][15]);
        let eq156_e1975_d_n16: f64 = self.ddt_jacobian(s.dn[265][16]);
        let eq156_e1975_d_n17: f64 = self.ddt_jacobian(s.dn[265][17]);
        let eq156_e1975_d_n18: f64 = self.ddt_jacobian(s.dn[265][18]);
        let eq156_e1975_d_n19: f64 = self.ddt_jacobian(s.dn[265][19]);
        let eq156_e1975_d_n20: f64 = self.ddt_jacobian(s.dn[265][20]);
        let eq156_e1975_d_n21: f64 = self.ddt_jacobian(s.dn[265][21]);
        let eq156_e1975_d_n22: f64 = self.ddt_jacobian(s.dn[265][22]);
        let eq156_e1976: f64 = (p.p7 * eq156_e1975);
        let eq156_e1976_d_n0: f64 = (p.p7 * eq156_e1975_d_n0);
        let eq156_e1976_d_n1: f64 = (p.p7 * eq156_e1975_d_n1);
        let eq156_e1976_d_n2: f64 = (p.p7 * eq156_e1975_d_n2);
        let eq156_e1976_d_n3: f64 = (p.p7 * eq156_e1975_d_n3);
        let eq156_e1976_d_n4: f64 = (p.p7 * eq156_e1975_d_n4);
        let eq156_e1976_d_n5: f64 = (p.p7 * eq156_e1975_d_n5);
        let eq156_e1976_d_n6: f64 = (p.p7 * eq156_e1975_d_n6);
        let eq156_e1976_d_n7: f64 = (p.p7 * eq156_e1975_d_n7);
        let eq156_e1976_d_n8: f64 = (p.p7 * eq156_e1975_d_n8);
        let eq156_e1976_d_n9: f64 = (p.p7 * eq156_e1975_d_n9);
        let eq156_e1976_d_n10: f64 = (p.p7 * eq156_e1975_d_n10);
        let eq156_e1976_d_n11: f64 = (p.p7 * eq156_e1975_d_n11);
        let eq156_e1976_d_n12: f64 = (p.p7 * eq156_e1975_d_n12);
        let eq156_e1976_d_n13: f64 = (p.p7 * eq156_e1975_d_n13);
        let eq156_e1976_d_n14: f64 = (p.p7 * eq156_e1975_d_n14);
        let eq156_e1976_d_n15: f64 = (p.p7 * eq156_e1975_d_n15);
        let eq156_e1976_d_n16: f64 = (p.p7 * eq156_e1975_d_n16);
        let eq156_e1976_d_n17: f64 = (p.p7 * eq156_e1975_d_n17);
        let eq156_e1976_d_n18: f64 = (p.p7 * eq156_e1975_d_n18);
        let eq156_e1976_d_n19: f64 = (p.p7 * eq156_e1975_d_n19);
        let eq156_e1976_d_n20: f64 = (p.p7 * eq156_e1975_d_n20);
        let eq156_e1976_d_n21: f64 = (p.p7 * eq156_e1975_d_n21);
        let eq156_e1976_d_n22: f64 = (p.p7 * eq156_e1975_d_n22);
        (eq156_e1976, eq156_e1976_d_n0, eq156_e1976_d_n1, eq156_e1976_d_n2, eq156_e1976_d_n3, eq156_e1976_d_n4, eq156_e1976_d_n5, eq156_e1976_d_n6, eq156_e1976_d_n7, eq156_e1976_d_n8, eq156_e1976_d_n9, eq156_e1976_d_n10, eq156_e1976_d_n11, eq156_e1976_d_n12, eq156_e1976_d_n13, eq156_e1976_d_n14, eq156_e1976_d_n15, eq156_e1976_d_n16, eq156_e1976_d_n17, eq156_e1976_d_n18, eq156_e1976_d_n19, eq156_e1976_d_n20, eq156_e1976_d_n21, eq156_e1976_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_value: f64 = eq156_e1978;
        let eq156_node_derivatives: [f64; 23] = [eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n10, eq156_e1978_d_n11, eq156_e1978_d_n12, eq156_e1978_d_n13, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22];
        let eq156_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[20]),
            self.multiplicity * (eq156_value),
            &nodes,
            &eq156_node_derivatives,
            &branches,
            &eq156_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_157_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq157_e1989, eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n10, eq157_e1989_d_n11, eq157_e1989_d_n12, eq157_e1989_d_n13, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22,) = {
    if (((s.v[585] != 0.0) && (s.v[586] != 0.0)) && (s.v[587] != 0.0)) {
        let eq157_e1986: f64 = self.eval_ddt(56, s.v[264]);
        let eq157_e1986_d_n0: f64 = self.ddt_jacobian(s.dn[264][0]);
        let eq157_e1986_d_n1: f64 = self.ddt_jacobian(s.dn[264][1]);
        let eq157_e1986_d_n2: f64 = self.ddt_jacobian(s.dn[264][2]);
        let eq157_e1986_d_n3: f64 = self.ddt_jacobian(s.dn[264][3]);
        let eq157_e1986_d_n4: f64 = self.ddt_jacobian(s.dn[264][4]);
        let eq157_e1986_d_n5: f64 = self.ddt_jacobian(s.dn[264][5]);
        let eq157_e1986_d_n6: f64 = self.ddt_jacobian(s.dn[264][6]);
        let eq157_e1986_d_n7: f64 = self.ddt_jacobian(s.dn[264][7]);
        let eq157_e1986_d_n8: f64 = self.ddt_jacobian(s.dn[264][8]);
        let eq157_e1986_d_n9: f64 = self.ddt_jacobian(s.dn[264][9]);
        let eq157_e1986_d_n10: f64 = self.ddt_jacobian(s.dn[264][10]);
        let eq157_e1986_d_n11: f64 = self.ddt_jacobian(s.dn[264][11]);
        let eq157_e1986_d_n12: f64 = self.ddt_jacobian(s.dn[264][12]);
        let eq157_e1986_d_n13: f64 = self.ddt_jacobian(s.dn[264][13]);
        let eq157_e1986_d_n14: f64 = self.ddt_jacobian(s.dn[264][14]);
        let eq157_e1986_d_n15: f64 = self.ddt_jacobian(s.dn[264][15]);
        let eq157_e1986_d_n16: f64 = self.ddt_jacobian(s.dn[264][16]);
        let eq157_e1986_d_n17: f64 = self.ddt_jacobian(s.dn[264][17]);
        let eq157_e1986_d_n18: f64 = self.ddt_jacobian(s.dn[264][18]);
        let eq157_e1986_d_n19: f64 = self.ddt_jacobian(s.dn[264][19]);
        let eq157_e1986_d_n20: f64 = self.ddt_jacobian(s.dn[264][20]);
        let eq157_e1986_d_n21: f64 = self.ddt_jacobian(s.dn[264][21]);
        let eq157_e1986_d_n22: f64 = self.ddt_jacobian(s.dn[264][22]);
        let eq157_e1987: f64 = (p.p7 * eq157_e1986);
        let eq157_e1987_d_n0: f64 = (p.p7 * eq157_e1986_d_n0);
        let eq157_e1987_d_n1: f64 = (p.p7 * eq157_e1986_d_n1);
        let eq157_e1987_d_n2: f64 = (p.p7 * eq157_e1986_d_n2);
        let eq157_e1987_d_n3: f64 = (p.p7 * eq157_e1986_d_n3);
        let eq157_e1987_d_n4: f64 = (p.p7 * eq157_e1986_d_n4);
        let eq157_e1987_d_n5: f64 = (p.p7 * eq157_e1986_d_n5);
        let eq157_e1987_d_n6: f64 = (p.p7 * eq157_e1986_d_n6);
        let eq157_e1987_d_n7: f64 = (p.p7 * eq157_e1986_d_n7);
        let eq157_e1987_d_n8: f64 = (p.p7 * eq157_e1986_d_n8);
        let eq157_e1987_d_n9: f64 = (p.p7 * eq157_e1986_d_n9);
        let eq157_e1987_d_n10: f64 = (p.p7 * eq157_e1986_d_n10);
        let eq157_e1987_d_n11: f64 = (p.p7 * eq157_e1986_d_n11);
        let eq157_e1987_d_n12: f64 = (p.p7 * eq157_e1986_d_n12);
        let eq157_e1987_d_n13: f64 = (p.p7 * eq157_e1986_d_n13);
        let eq157_e1987_d_n14: f64 = (p.p7 * eq157_e1986_d_n14);
        let eq157_e1987_d_n15: f64 = (p.p7 * eq157_e1986_d_n15);
        let eq157_e1987_d_n16: f64 = (p.p7 * eq157_e1986_d_n16);
        let eq157_e1987_d_n17: f64 = (p.p7 * eq157_e1986_d_n17);
        let eq157_e1987_d_n18: f64 = (p.p7 * eq157_e1986_d_n18);
        let eq157_e1987_d_n19: f64 = (p.p7 * eq157_e1986_d_n19);
        let eq157_e1987_d_n20: f64 = (p.p7 * eq157_e1986_d_n20);
        let eq157_e1987_d_n21: f64 = (p.p7 * eq157_e1986_d_n21);
        let eq157_e1987_d_n22: f64 = (p.p7 * eq157_e1986_d_n22);
        (eq157_e1987, eq157_e1987_d_n0, eq157_e1987_d_n1, eq157_e1987_d_n2, eq157_e1987_d_n3, eq157_e1987_d_n4, eq157_e1987_d_n5, eq157_e1987_d_n6, eq157_e1987_d_n7, eq157_e1987_d_n8, eq157_e1987_d_n9, eq157_e1987_d_n10, eq157_e1987_d_n11, eq157_e1987_d_n12, eq157_e1987_d_n13, eq157_e1987_d_n14, eq157_e1987_d_n15, eq157_e1987_d_n16, eq157_e1987_d_n17, eq157_e1987_d_n18, eq157_e1987_d_n19, eq157_e1987_d_n20, eq157_e1987_d_n21, eq157_e1987_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_value: f64 = eq157_e1989;
        let eq157_node_derivatives: [f64; 23] = [eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n10, eq157_e1989_d_n11, eq157_e1989_d_n12, eq157_e1989_d_n13, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22];
        let eq157_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            self.multiplicity * (eq157_value),
            &nodes,
            &eq157_node_derivatives,
            &branches,
            &eq157_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_158_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq158_e2002, eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n10, eq158_e2002_d_n11, eq158_e2002_d_n12, eq158_e2002_d_n13, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22,) = {
    if (((s.v[585] != 0.0) && (s.v[586] != 0.0)) && (s.v[587] != 0.0)) {
        let eq158_e1997: f64 = self.eval_ddt(57, s.v[264]);
        let eq158_e1997_d_n0: f64 = self.ddt_jacobian(s.dn[264][0]);
        let eq158_e1997_d_n1: f64 = self.ddt_jacobian(s.dn[264][1]);
        let eq158_e1997_d_n2: f64 = self.ddt_jacobian(s.dn[264][2]);
        let eq158_e1997_d_n3: f64 = self.ddt_jacobian(s.dn[264][3]);
        let eq158_e1997_d_n4: f64 = self.ddt_jacobian(s.dn[264][4]);
        let eq158_e1997_d_n5: f64 = self.ddt_jacobian(s.dn[264][5]);
        let eq158_e1997_d_n6: f64 = self.ddt_jacobian(s.dn[264][6]);
        let eq158_e1997_d_n7: f64 = self.ddt_jacobian(s.dn[264][7]);
        let eq158_e1997_d_n8: f64 = self.ddt_jacobian(s.dn[264][8]);
        let eq158_e1997_d_n9: f64 = self.ddt_jacobian(s.dn[264][9]);
        let eq158_e1997_d_n10: f64 = self.ddt_jacobian(s.dn[264][10]);
        let eq158_e1997_d_n11: f64 = self.ddt_jacobian(s.dn[264][11]);
        let eq158_e1997_d_n12: f64 = self.ddt_jacobian(s.dn[264][12]);
        let eq158_e1997_d_n13: f64 = self.ddt_jacobian(s.dn[264][13]);
        let eq158_e1997_d_n14: f64 = self.ddt_jacobian(s.dn[264][14]);
        let eq158_e1997_d_n15: f64 = self.ddt_jacobian(s.dn[264][15]);
        let eq158_e1997_d_n16: f64 = self.ddt_jacobian(s.dn[264][16]);
        let eq158_e1997_d_n17: f64 = self.ddt_jacobian(s.dn[264][17]);
        let eq158_e1997_d_n18: f64 = self.ddt_jacobian(s.dn[264][18]);
        let eq158_e1997_d_n19: f64 = self.ddt_jacobian(s.dn[264][19]);
        let eq158_e1997_d_n20: f64 = self.ddt_jacobian(s.dn[264][20]);
        let eq158_e1997_d_n21: f64 = self.ddt_jacobian(s.dn[264][21]);
        let eq158_e1997_d_n22: f64 = self.ddt_jacobian(s.dn[264][22]);
        let eq158_e1998: f64 = (p.p7 * eq158_e1997);
        let eq158_e1998_d_n0: f64 = (p.p7 * eq158_e1997_d_n0);
        let eq158_e1998_d_n1: f64 = (p.p7 * eq158_e1997_d_n1);
        let eq158_e1998_d_n2: f64 = (p.p7 * eq158_e1997_d_n2);
        let eq158_e1998_d_n3: f64 = (p.p7 * eq158_e1997_d_n3);
        let eq158_e1998_d_n4: f64 = (p.p7 * eq158_e1997_d_n4);
        let eq158_e1998_d_n5: f64 = (p.p7 * eq158_e1997_d_n5);
        let eq158_e1998_d_n6: f64 = (p.p7 * eq158_e1997_d_n6);
        let eq158_e1998_d_n7: f64 = (p.p7 * eq158_e1997_d_n7);
        let eq158_e1998_d_n8: f64 = (p.p7 * eq158_e1997_d_n8);
        let eq158_e1998_d_n9: f64 = (p.p7 * eq158_e1997_d_n9);
        let eq158_e1998_d_n10: f64 = (p.p7 * eq158_e1997_d_n10);
        let eq158_e1998_d_n11: f64 = (p.p7 * eq158_e1997_d_n11);
        let eq158_e1998_d_n12: f64 = (p.p7 * eq158_e1997_d_n12);
        let eq158_e1998_d_n13: f64 = (p.p7 * eq158_e1997_d_n13);
        let eq158_e1998_d_n14: f64 = (p.p7 * eq158_e1997_d_n14);
        let eq158_e1998_d_n15: f64 = (p.p7 * eq158_e1997_d_n15);
        let eq158_e1998_d_n16: f64 = (p.p7 * eq158_e1997_d_n16);
        let eq158_e1998_d_n17: f64 = (p.p7 * eq158_e1997_d_n17);
        let eq158_e1998_d_n18: f64 = (p.p7 * eq158_e1997_d_n18);
        let eq158_e1998_d_n19: f64 = (p.p7 * eq158_e1997_d_n19);
        let eq158_e1998_d_n20: f64 = (p.p7 * eq158_e1997_d_n20);
        let eq158_e1998_d_n21: f64 = (p.p7 * eq158_e1997_d_n21);
        let eq158_e1998_d_n22: f64 = (p.p7 * eq158_e1997_d_n22);
        let eq158_e2000: f64 = (eq158_e1998 * p.p247);
        let eq158_e2000_d_n0: f64 = (eq158_e1998_d_n0 * p.p247);
        let eq158_e2000_d_n1: f64 = (eq158_e1998_d_n1 * p.p247);
        let eq158_e2000_d_n2: f64 = (eq158_e1998_d_n2 * p.p247);
        let eq158_e2000_d_n3: f64 = (eq158_e1998_d_n3 * p.p247);
        let eq158_e2000_d_n4: f64 = (eq158_e1998_d_n4 * p.p247);
        let eq158_e2000_d_n5: f64 = (eq158_e1998_d_n5 * p.p247);
        let eq158_e2000_d_n6: f64 = (eq158_e1998_d_n6 * p.p247);
        let eq158_e2000_d_n7: f64 = (eq158_e1998_d_n7 * p.p247);
        let eq158_e2000_d_n8: f64 = (eq158_e1998_d_n8 * p.p247);
        let eq158_e2000_d_n9: f64 = (eq158_e1998_d_n9 * p.p247);
        let eq158_e2000_d_n10: f64 = (eq158_e1998_d_n10 * p.p247);
        let eq158_e2000_d_n11: f64 = (eq158_e1998_d_n11 * p.p247);
        let eq158_e2000_d_n12: f64 = (eq158_e1998_d_n12 * p.p247);
        let eq158_e2000_d_n13: f64 = (eq158_e1998_d_n13 * p.p247);
        let eq158_e2000_d_n14: f64 = (eq158_e1998_d_n14 * p.p247);
        let eq158_e2000_d_n15: f64 = (eq158_e1998_d_n15 * p.p247);
        let eq158_e2000_d_n16: f64 = (eq158_e1998_d_n16 * p.p247);
        let eq158_e2000_d_n17: f64 = (eq158_e1998_d_n17 * p.p247);
        let eq158_e2000_d_n18: f64 = (eq158_e1998_d_n18 * p.p247);
        let eq158_e2000_d_n19: f64 = (eq158_e1998_d_n19 * p.p247);
        let eq158_e2000_d_n20: f64 = (eq158_e1998_d_n20 * p.p247);
        let eq158_e2000_d_n21: f64 = (eq158_e1998_d_n21 * p.p247);
        let eq158_e2000_d_n22: f64 = (eq158_e1998_d_n22 * p.p247);
        (eq158_e2000, eq158_e2000_d_n0, eq158_e2000_d_n1, eq158_e2000_d_n2, eq158_e2000_d_n3, eq158_e2000_d_n4, eq158_e2000_d_n5, eq158_e2000_d_n6, eq158_e2000_d_n7, eq158_e2000_d_n8, eq158_e2000_d_n9, eq158_e2000_d_n10, eq158_e2000_d_n11, eq158_e2000_d_n12, eq158_e2000_d_n13, eq158_e2000_d_n14, eq158_e2000_d_n15, eq158_e2000_d_n16, eq158_e2000_d_n17, eq158_e2000_d_n18, eq158_e2000_d_n19, eq158_e2000_d_n20, eq158_e2000_d_n21, eq158_e2000_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq158_value: f64 = eq158_e2002;
        let eq158_node_derivatives: [f64; 23] = [eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n10, eq158_e2002_d_n11, eq158_e2002_d_n12, eq158_e2002_d_n13, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22];
        let eq158_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            self.multiplicity * (eq158_value),
            &nodes,
            &eq158_node_derivatives,
            &branches,
            &eq158_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_159_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq159_e2014, eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n10, eq159_e2014_d_n11, eq159_e2014_d_n12, eq159_e2014_d_n13, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22,) = {
    if (((s.v[585] != 0.0) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) {
        let eq159_e2011: f64 = self.eval_ddt(58, s.v[264]);
        let eq159_e2011_d_n0: f64 = self.ddt_jacobian(s.dn[264][0]);
        let eq159_e2011_d_n1: f64 = self.ddt_jacobian(s.dn[264][1]);
        let eq159_e2011_d_n2: f64 = self.ddt_jacobian(s.dn[264][2]);
        let eq159_e2011_d_n3: f64 = self.ddt_jacobian(s.dn[264][3]);
        let eq159_e2011_d_n4: f64 = self.ddt_jacobian(s.dn[264][4]);
        let eq159_e2011_d_n5: f64 = self.ddt_jacobian(s.dn[264][5]);
        let eq159_e2011_d_n6: f64 = self.ddt_jacobian(s.dn[264][6]);
        let eq159_e2011_d_n7: f64 = self.ddt_jacobian(s.dn[264][7]);
        let eq159_e2011_d_n8: f64 = self.ddt_jacobian(s.dn[264][8]);
        let eq159_e2011_d_n9: f64 = self.ddt_jacobian(s.dn[264][9]);
        let eq159_e2011_d_n10: f64 = self.ddt_jacobian(s.dn[264][10]);
        let eq159_e2011_d_n11: f64 = self.ddt_jacobian(s.dn[264][11]);
        let eq159_e2011_d_n12: f64 = self.ddt_jacobian(s.dn[264][12]);
        let eq159_e2011_d_n13: f64 = self.ddt_jacobian(s.dn[264][13]);
        let eq159_e2011_d_n14: f64 = self.ddt_jacobian(s.dn[264][14]);
        let eq159_e2011_d_n15: f64 = self.ddt_jacobian(s.dn[264][15]);
        let eq159_e2011_d_n16: f64 = self.ddt_jacobian(s.dn[264][16]);
        let eq159_e2011_d_n17: f64 = self.ddt_jacobian(s.dn[264][17]);
        let eq159_e2011_d_n18: f64 = self.ddt_jacobian(s.dn[264][18]);
        let eq159_e2011_d_n19: f64 = self.ddt_jacobian(s.dn[264][19]);
        let eq159_e2011_d_n20: f64 = self.ddt_jacobian(s.dn[264][20]);
        let eq159_e2011_d_n21: f64 = self.ddt_jacobian(s.dn[264][21]);
        let eq159_e2011_d_n22: f64 = self.ddt_jacobian(s.dn[264][22]);
        let eq159_e2012: f64 = (p.p7 * eq159_e2011);
        let eq159_e2012_d_n0: f64 = (p.p7 * eq159_e2011_d_n0);
        let eq159_e2012_d_n1: f64 = (p.p7 * eq159_e2011_d_n1);
        let eq159_e2012_d_n2: f64 = (p.p7 * eq159_e2011_d_n2);
        let eq159_e2012_d_n3: f64 = (p.p7 * eq159_e2011_d_n3);
        let eq159_e2012_d_n4: f64 = (p.p7 * eq159_e2011_d_n4);
        let eq159_e2012_d_n5: f64 = (p.p7 * eq159_e2011_d_n5);
        let eq159_e2012_d_n6: f64 = (p.p7 * eq159_e2011_d_n6);
        let eq159_e2012_d_n7: f64 = (p.p7 * eq159_e2011_d_n7);
        let eq159_e2012_d_n8: f64 = (p.p7 * eq159_e2011_d_n8);
        let eq159_e2012_d_n9: f64 = (p.p7 * eq159_e2011_d_n9);
        let eq159_e2012_d_n10: f64 = (p.p7 * eq159_e2011_d_n10);
        let eq159_e2012_d_n11: f64 = (p.p7 * eq159_e2011_d_n11);
        let eq159_e2012_d_n12: f64 = (p.p7 * eq159_e2011_d_n12);
        let eq159_e2012_d_n13: f64 = (p.p7 * eq159_e2011_d_n13);
        let eq159_e2012_d_n14: f64 = (p.p7 * eq159_e2011_d_n14);
        let eq159_e2012_d_n15: f64 = (p.p7 * eq159_e2011_d_n15);
        let eq159_e2012_d_n16: f64 = (p.p7 * eq159_e2011_d_n16);
        let eq159_e2012_d_n17: f64 = (p.p7 * eq159_e2011_d_n17);
        let eq159_e2012_d_n18: f64 = (p.p7 * eq159_e2011_d_n18);
        let eq159_e2012_d_n19: f64 = (p.p7 * eq159_e2011_d_n19);
        let eq159_e2012_d_n20: f64 = (p.p7 * eq159_e2011_d_n20);
        let eq159_e2012_d_n21: f64 = (p.p7 * eq159_e2011_d_n21);
        let eq159_e2012_d_n22: f64 = (p.p7 * eq159_e2011_d_n22);
        (eq159_e2012, eq159_e2012_d_n0, eq159_e2012_d_n1, eq159_e2012_d_n2, eq159_e2012_d_n3, eq159_e2012_d_n4, eq159_e2012_d_n5, eq159_e2012_d_n6, eq159_e2012_d_n7, eq159_e2012_d_n8, eq159_e2012_d_n9, eq159_e2012_d_n10, eq159_e2012_d_n11, eq159_e2012_d_n12, eq159_e2012_d_n13, eq159_e2012_d_n14, eq159_e2012_d_n15, eq159_e2012_d_n16, eq159_e2012_d_n17, eq159_e2012_d_n18, eq159_e2012_d_n19, eq159_e2012_d_n20, eq159_e2012_d_n21, eq159_e2012_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq159_value: f64 = eq159_e2014;
        let eq159_node_derivatives: [f64; 23] = [eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n10, eq159_e2014_d_n11, eq159_e2014_d_n12, eq159_e2014_d_n13, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22];
        let eq159_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            self.multiplicity * (eq159_value),
            &nodes,
            &eq159_node_derivatives,
            &branches,
            &eq159_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_160_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq160_e2028, eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n10, eq160_e2028_d_n11, eq160_e2028_d_n12, eq160_e2028_d_n13, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22,) = {
    if (((s.v[585] != 0.0) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) {
        let eq160_e2023: f64 = self.eval_ddt(59, s.v[264]);
        let eq160_e2023_d_n0: f64 = self.ddt_jacobian(s.dn[264][0]);
        let eq160_e2023_d_n1: f64 = self.ddt_jacobian(s.dn[264][1]);
        let eq160_e2023_d_n2: f64 = self.ddt_jacobian(s.dn[264][2]);
        let eq160_e2023_d_n3: f64 = self.ddt_jacobian(s.dn[264][3]);
        let eq160_e2023_d_n4: f64 = self.ddt_jacobian(s.dn[264][4]);
        let eq160_e2023_d_n5: f64 = self.ddt_jacobian(s.dn[264][5]);
        let eq160_e2023_d_n6: f64 = self.ddt_jacobian(s.dn[264][6]);
        let eq160_e2023_d_n7: f64 = self.ddt_jacobian(s.dn[264][7]);
        let eq160_e2023_d_n8: f64 = self.ddt_jacobian(s.dn[264][8]);
        let eq160_e2023_d_n9: f64 = self.ddt_jacobian(s.dn[264][9]);
        let eq160_e2023_d_n10: f64 = self.ddt_jacobian(s.dn[264][10]);
        let eq160_e2023_d_n11: f64 = self.ddt_jacobian(s.dn[264][11]);
        let eq160_e2023_d_n12: f64 = self.ddt_jacobian(s.dn[264][12]);
        let eq160_e2023_d_n13: f64 = self.ddt_jacobian(s.dn[264][13]);
        let eq160_e2023_d_n14: f64 = self.ddt_jacobian(s.dn[264][14]);
        let eq160_e2023_d_n15: f64 = self.ddt_jacobian(s.dn[264][15]);
        let eq160_e2023_d_n16: f64 = self.ddt_jacobian(s.dn[264][16]);
        let eq160_e2023_d_n17: f64 = self.ddt_jacobian(s.dn[264][17]);
        let eq160_e2023_d_n18: f64 = self.ddt_jacobian(s.dn[264][18]);
        let eq160_e2023_d_n19: f64 = self.ddt_jacobian(s.dn[264][19]);
        let eq160_e2023_d_n20: f64 = self.ddt_jacobian(s.dn[264][20]);
        let eq160_e2023_d_n21: f64 = self.ddt_jacobian(s.dn[264][21]);
        let eq160_e2023_d_n22: f64 = self.ddt_jacobian(s.dn[264][22]);
        let eq160_e2024: f64 = (p.p7 * eq160_e2023);
        let eq160_e2024_d_n0: f64 = (p.p7 * eq160_e2023_d_n0);
        let eq160_e2024_d_n1: f64 = (p.p7 * eq160_e2023_d_n1);
        let eq160_e2024_d_n2: f64 = (p.p7 * eq160_e2023_d_n2);
        let eq160_e2024_d_n3: f64 = (p.p7 * eq160_e2023_d_n3);
        let eq160_e2024_d_n4: f64 = (p.p7 * eq160_e2023_d_n4);
        let eq160_e2024_d_n5: f64 = (p.p7 * eq160_e2023_d_n5);
        let eq160_e2024_d_n6: f64 = (p.p7 * eq160_e2023_d_n6);
        let eq160_e2024_d_n7: f64 = (p.p7 * eq160_e2023_d_n7);
        let eq160_e2024_d_n8: f64 = (p.p7 * eq160_e2023_d_n8);
        let eq160_e2024_d_n9: f64 = (p.p7 * eq160_e2023_d_n9);
        let eq160_e2024_d_n10: f64 = (p.p7 * eq160_e2023_d_n10);
        let eq160_e2024_d_n11: f64 = (p.p7 * eq160_e2023_d_n11);
        let eq160_e2024_d_n12: f64 = (p.p7 * eq160_e2023_d_n12);
        let eq160_e2024_d_n13: f64 = (p.p7 * eq160_e2023_d_n13);
        let eq160_e2024_d_n14: f64 = (p.p7 * eq160_e2023_d_n14);
        let eq160_e2024_d_n15: f64 = (p.p7 * eq160_e2023_d_n15);
        let eq160_e2024_d_n16: f64 = (p.p7 * eq160_e2023_d_n16);
        let eq160_e2024_d_n17: f64 = (p.p7 * eq160_e2023_d_n17);
        let eq160_e2024_d_n18: f64 = (p.p7 * eq160_e2023_d_n18);
        let eq160_e2024_d_n19: f64 = (p.p7 * eq160_e2023_d_n19);
        let eq160_e2024_d_n20: f64 = (p.p7 * eq160_e2023_d_n20);
        let eq160_e2024_d_n21: f64 = (p.p7 * eq160_e2023_d_n21);
        let eq160_e2024_d_n22: f64 = (p.p7 * eq160_e2023_d_n22);
        let eq160_e2026: f64 = (eq160_e2024 * p.p247);
        let eq160_e2026_d_n0: f64 = (eq160_e2024_d_n0 * p.p247);
        let eq160_e2026_d_n1: f64 = (eq160_e2024_d_n1 * p.p247);
        let eq160_e2026_d_n2: f64 = (eq160_e2024_d_n2 * p.p247);
        let eq160_e2026_d_n3: f64 = (eq160_e2024_d_n3 * p.p247);
        let eq160_e2026_d_n4: f64 = (eq160_e2024_d_n4 * p.p247);
        let eq160_e2026_d_n5: f64 = (eq160_e2024_d_n5 * p.p247);
        let eq160_e2026_d_n6: f64 = (eq160_e2024_d_n6 * p.p247);
        let eq160_e2026_d_n7: f64 = (eq160_e2024_d_n7 * p.p247);
        let eq160_e2026_d_n8: f64 = (eq160_e2024_d_n8 * p.p247);
        let eq160_e2026_d_n9: f64 = (eq160_e2024_d_n9 * p.p247);
        let eq160_e2026_d_n10: f64 = (eq160_e2024_d_n10 * p.p247);
        let eq160_e2026_d_n11: f64 = (eq160_e2024_d_n11 * p.p247);
        let eq160_e2026_d_n12: f64 = (eq160_e2024_d_n12 * p.p247);
        let eq160_e2026_d_n13: f64 = (eq160_e2024_d_n13 * p.p247);
        let eq160_e2026_d_n14: f64 = (eq160_e2024_d_n14 * p.p247);
        let eq160_e2026_d_n15: f64 = (eq160_e2024_d_n15 * p.p247);
        let eq160_e2026_d_n16: f64 = (eq160_e2024_d_n16 * p.p247);
        let eq160_e2026_d_n17: f64 = (eq160_e2024_d_n17 * p.p247);
        let eq160_e2026_d_n18: f64 = (eq160_e2024_d_n18 * p.p247);
        let eq160_e2026_d_n19: f64 = (eq160_e2024_d_n19 * p.p247);
        let eq160_e2026_d_n20: f64 = (eq160_e2024_d_n20 * p.p247);
        let eq160_e2026_d_n21: f64 = (eq160_e2024_d_n21 * p.p247);
        let eq160_e2026_d_n22: f64 = (eq160_e2024_d_n22 * p.p247);
        (eq160_e2026, eq160_e2026_d_n0, eq160_e2026_d_n1, eq160_e2026_d_n2, eq160_e2026_d_n3, eq160_e2026_d_n4, eq160_e2026_d_n5, eq160_e2026_d_n6, eq160_e2026_d_n7, eq160_e2026_d_n8, eq160_e2026_d_n9, eq160_e2026_d_n10, eq160_e2026_d_n11, eq160_e2026_d_n12, eq160_e2026_d_n13, eq160_e2026_d_n14, eq160_e2026_d_n15, eq160_e2026_d_n16, eq160_e2026_d_n17, eq160_e2026_d_n18, eq160_e2026_d_n19, eq160_e2026_d_n20, eq160_e2026_d_n21, eq160_e2026_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_value: f64 = eq160_e2028;
        let eq160_node_derivatives: [f64; 23] = [eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n10, eq160_e2028_d_n11, eq160_e2028_d_n12, eq160_e2028_d_n13, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22];
        let eq160_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            self.multiplicity * (eq160_value),
            &nodes,
            &eq160_node_derivatives,
            &branches,
            &eq160_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_161_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq161_e2039, eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n10, eq161_e2039_d_n11, eq161_e2039_d_n12, eq161_e2039_d_n13, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22,) = {
    if ((s.v[585] != 0.0) && (s.v[586] != 0.0)) {
        let eq161_e2035: f64 = (p.p252 * s.v[264]);
        let eq161_e2035_d_n0: f64 = (p.p252 * s.dn[264][0]);
        let eq161_e2035_d_n1: f64 = (p.p252 * s.dn[264][1]);
        let eq161_e2035_d_n2: f64 = (p.p252 * s.dn[264][2]);
        let eq161_e2035_d_n3: f64 = (p.p252 * s.dn[264][3]);
        let eq161_e2035_d_n4: f64 = (p.p252 * s.dn[264][4]);
        let eq161_e2035_d_n5: f64 = (p.p252 * s.dn[264][5]);
        let eq161_e2035_d_n6: f64 = (p.p252 * s.dn[264][6]);
        let eq161_e2035_d_n7: f64 = (p.p252 * s.dn[264][7]);
        let eq161_e2035_d_n8: f64 = (p.p252 * s.dn[264][8]);
        let eq161_e2035_d_n9: f64 = (p.p252 * s.dn[264][9]);
        let eq161_e2035_d_n10: f64 = (p.p252 * s.dn[264][10]);
        let eq161_e2035_d_n11: f64 = (p.p252 * s.dn[264][11]);
        let eq161_e2035_d_n12: f64 = (p.p252 * s.dn[264][12]);
        let eq161_e2035_d_n13: f64 = (p.p252 * s.dn[264][13]);
        let eq161_e2035_d_n14: f64 = (p.p252 * s.dn[264][14]);
        let eq161_e2035_d_n15: f64 = (p.p252 * s.dn[264][15]);
        let eq161_e2035_d_n16: f64 = (p.p252 * s.dn[264][16]);
        let eq161_e2035_d_n17: f64 = (p.p252 * s.dn[264][17]);
        let eq161_e2035_d_n18: f64 = (p.p252 * s.dn[264][18]);
        let eq161_e2035_d_n19: f64 = (p.p252 * s.dn[264][19]);
        let eq161_e2035_d_n20: f64 = (p.p252 * s.dn[264][20]);
        let eq161_e2035_d_n21: f64 = (p.p252 * s.dn[264][21]);
        let eq161_e2035_d_n22: f64 = (p.p252 * s.dn[264][22]);
        let eq161_e2036: f64 = self.eval_ddt(60, eq161_e2035);
        let eq161_e2036_d_n0: f64 = self.ddt_jacobian(eq161_e2035_d_n0);
        let eq161_e2036_d_n1: f64 = self.ddt_jacobian(eq161_e2035_d_n1);
        let eq161_e2036_d_n2: f64 = self.ddt_jacobian(eq161_e2035_d_n2);
        let eq161_e2036_d_n3: f64 = self.ddt_jacobian(eq161_e2035_d_n3);
        let eq161_e2036_d_n4: f64 = self.ddt_jacobian(eq161_e2035_d_n4);
        let eq161_e2036_d_n5: f64 = self.ddt_jacobian(eq161_e2035_d_n5);
        let eq161_e2036_d_n6: f64 = self.ddt_jacobian(eq161_e2035_d_n6);
        let eq161_e2036_d_n7: f64 = self.ddt_jacobian(eq161_e2035_d_n7);
        let eq161_e2036_d_n8: f64 = self.ddt_jacobian(eq161_e2035_d_n8);
        let eq161_e2036_d_n9: f64 = self.ddt_jacobian(eq161_e2035_d_n9);
        let eq161_e2036_d_n10: f64 = self.ddt_jacobian(eq161_e2035_d_n10);
        let eq161_e2036_d_n11: f64 = self.ddt_jacobian(eq161_e2035_d_n11);
        let eq161_e2036_d_n12: f64 = self.ddt_jacobian(eq161_e2035_d_n12);
        let eq161_e2036_d_n13: f64 = self.ddt_jacobian(eq161_e2035_d_n13);
        let eq161_e2036_d_n14: f64 = self.ddt_jacobian(eq161_e2035_d_n14);
        let eq161_e2036_d_n15: f64 = self.ddt_jacobian(eq161_e2035_d_n15);
        let eq161_e2036_d_n16: f64 = self.ddt_jacobian(eq161_e2035_d_n16);
        let eq161_e2036_d_n17: f64 = self.ddt_jacobian(eq161_e2035_d_n17);
        let eq161_e2036_d_n18: f64 = self.ddt_jacobian(eq161_e2035_d_n18);
        let eq161_e2036_d_n19: f64 = self.ddt_jacobian(eq161_e2035_d_n19);
        let eq161_e2036_d_n20: f64 = self.ddt_jacobian(eq161_e2035_d_n20);
        let eq161_e2036_d_n21: f64 = self.ddt_jacobian(eq161_e2035_d_n21);
        let eq161_e2036_d_n22: f64 = self.ddt_jacobian(eq161_e2035_d_n22);
        let eq161_e2037: f64 = (p.p7 * eq161_e2036);
        let eq161_e2037_d_n0: f64 = (p.p7 * eq161_e2036_d_n0);
        let eq161_e2037_d_n1: f64 = (p.p7 * eq161_e2036_d_n1);
        let eq161_e2037_d_n2: f64 = (p.p7 * eq161_e2036_d_n2);
        let eq161_e2037_d_n3: f64 = (p.p7 * eq161_e2036_d_n3);
        let eq161_e2037_d_n4: f64 = (p.p7 * eq161_e2036_d_n4);
        let eq161_e2037_d_n5: f64 = (p.p7 * eq161_e2036_d_n5);
        let eq161_e2037_d_n6: f64 = (p.p7 * eq161_e2036_d_n6);
        let eq161_e2037_d_n7: f64 = (p.p7 * eq161_e2036_d_n7);
        let eq161_e2037_d_n8: f64 = (p.p7 * eq161_e2036_d_n8);
        let eq161_e2037_d_n9: f64 = (p.p7 * eq161_e2036_d_n9);
        let eq161_e2037_d_n10: f64 = (p.p7 * eq161_e2036_d_n10);
        let eq161_e2037_d_n11: f64 = (p.p7 * eq161_e2036_d_n11);
        let eq161_e2037_d_n12: f64 = (p.p7 * eq161_e2036_d_n12);
        let eq161_e2037_d_n13: f64 = (p.p7 * eq161_e2036_d_n13);
        let eq161_e2037_d_n14: f64 = (p.p7 * eq161_e2036_d_n14);
        let eq161_e2037_d_n15: f64 = (p.p7 * eq161_e2036_d_n15);
        let eq161_e2037_d_n16: f64 = (p.p7 * eq161_e2036_d_n16);
        let eq161_e2037_d_n17: f64 = (p.p7 * eq161_e2036_d_n17);
        let eq161_e2037_d_n18: f64 = (p.p7 * eq161_e2036_d_n18);
        let eq161_e2037_d_n19: f64 = (p.p7 * eq161_e2036_d_n19);
        let eq161_e2037_d_n20: f64 = (p.p7 * eq161_e2036_d_n20);
        let eq161_e2037_d_n21: f64 = (p.p7 * eq161_e2036_d_n21);
        let eq161_e2037_d_n22: f64 = (p.p7 * eq161_e2036_d_n22);
        (eq161_e2037, eq161_e2037_d_n0, eq161_e2037_d_n1, eq161_e2037_d_n2, eq161_e2037_d_n3, eq161_e2037_d_n4, eq161_e2037_d_n5, eq161_e2037_d_n6, eq161_e2037_d_n7, eq161_e2037_d_n8, eq161_e2037_d_n9, eq161_e2037_d_n10, eq161_e2037_d_n11, eq161_e2037_d_n12, eq161_e2037_d_n13, eq161_e2037_d_n14, eq161_e2037_d_n15, eq161_e2037_d_n16, eq161_e2037_d_n17, eq161_e2037_d_n18, eq161_e2037_d_n19, eq161_e2037_d_n20, eq161_e2037_d_n21, eq161_e2037_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_value: f64 = eq161_e2039;
        let eq161_node_derivatives: [f64; 23] = [eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n10, eq161_e2039_d_n11, eq161_e2039_d_n12, eq161_e2039_d_n13, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22];
        let eq161_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[20]),
            self.multiplicity * (eq161_value),
            &nodes,
            &eq161_node_derivatives,
            &branches,
            &eq161_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_162_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq162_e2049, eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n10, eq162_e2049_d_n11, eq162_e2049_d_n12, eq162_e2049_d_n13, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22,) = {
    if ((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) {
        let eq162_e2046: f64 = self.eval_ddt(61, s.v[265]);
        let eq162_e2046_d_n0: f64 = self.ddt_jacobian(s.dn[265][0]);
        let eq162_e2046_d_n1: f64 = self.ddt_jacobian(s.dn[265][1]);
        let eq162_e2046_d_n2: f64 = self.ddt_jacobian(s.dn[265][2]);
        let eq162_e2046_d_n3: f64 = self.ddt_jacobian(s.dn[265][3]);
        let eq162_e2046_d_n4: f64 = self.ddt_jacobian(s.dn[265][4]);
        let eq162_e2046_d_n5: f64 = self.ddt_jacobian(s.dn[265][5]);
        let eq162_e2046_d_n6: f64 = self.ddt_jacobian(s.dn[265][6]);
        let eq162_e2046_d_n7: f64 = self.ddt_jacobian(s.dn[265][7]);
        let eq162_e2046_d_n8: f64 = self.ddt_jacobian(s.dn[265][8]);
        let eq162_e2046_d_n9: f64 = self.ddt_jacobian(s.dn[265][9]);
        let eq162_e2046_d_n10: f64 = self.ddt_jacobian(s.dn[265][10]);
        let eq162_e2046_d_n11: f64 = self.ddt_jacobian(s.dn[265][11]);
        let eq162_e2046_d_n12: f64 = self.ddt_jacobian(s.dn[265][12]);
        let eq162_e2046_d_n13: f64 = self.ddt_jacobian(s.dn[265][13]);
        let eq162_e2046_d_n14: f64 = self.ddt_jacobian(s.dn[265][14]);
        let eq162_e2046_d_n15: f64 = self.ddt_jacobian(s.dn[265][15]);
        let eq162_e2046_d_n16: f64 = self.ddt_jacobian(s.dn[265][16]);
        let eq162_e2046_d_n17: f64 = self.ddt_jacobian(s.dn[265][17]);
        let eq162_e2046_d_n18: f64 = self.ddt_jacobian(s.dn[265][18]);
        let eq162_e2046_d_n19: f64 = self.ddt_jacobian(s.dn[265][19]);
        let eq162_e2046_d_n20: f64 = self.ddt_jacobian(s.dn[265][20]);
        let eq162_e2046_d_n21: f64 = self.ddt_jacobian(s.dn[265][21]);
        let eq162_e2046_d_n22: f64 = self.ddt_jacobian(s.dn[265][22]);
        let eq162_e2047: f64 = (p.p7 * eq162_e2046);
        let eq162_e2047_d_n0: f64 = (p.p7 * eq162_e2046_d_n0);
        let eq162_e2047_d_n1: f64 = (p.p7 * eq162_e2046_d_n1);
        let eq162_e2047_d_n2: f64 = (p.p7 * eq162_e2046_d_n2);
        let eq162_e2047_d_n3: f64 = (p.p7 * eq162_e2046_d_n3);
        let eq162_e2047_d_n4: f64 = (p.p7 * eq162_e2046_d_n4);
        let eq162_e2047_d_n5: f64 = (p.p7 * eq162_e2046_d_n5);
        let eq162_e2047_d_n6: f64 = (p.p7 * eq162_e2046_d_n6);
        let eq162_e2047_d_n7: f64 = (p.p7 * eq162_e2046_d_n7);
        let eq162_e2047_d_n8: f64 = (p.p7 * eq162_e2046_d_n8);
        let eq162_e2047_d_n9: f64 = (p.p7 * eq162_e2046_d_n9);
        let eq162_e2047_d_n10: f64 = (p.p7 * eq162_e2046_d_n10);
        let eq162_e2047_d_n11: f64 = (p.p7 * eq162_e2046_d_n11);
        let eq162_e2047_d_n12: f64 = (p.p7 * eq162_e2046_d_n12);
        let eq162_e2047_d_n13: f64 = (p.p7 * eq162_e2046_d_n13);
        let eq162_e2047_d_n14: f64 = (p.p7 * eq162_e2046_d_n14);
        let eq162_e2047_d_n15: f64 = (p.p7 * eq162_e2046_d_n15);
        let eq162_e2047_d_n16: f64 = (p.p7 * eq162_e2046_d_n16);
        let eq162_e2047_d_n17: f64 = (p.p7 * eq162_e2046_d_n17);
        let eq162_e2047_d_n18: f64 = (p.p7 * eq162_e2046_d_n18);
        let eq162_e2047_d_n19: f64 = (p.p7 * eq162_e2046_d_n19);
        let eq162_e2047_d_n20: f64 = (p.p7 * eq162_e2046_d_n20);
        let eq162_e2047_d_n21: f64 = (p.p7 * eq162_e2046_d_n21);
        let eq162_e2047_d_n22: f64 = (p.p7 * eq162_e2046_d_n22);
        (eq162_e2047, eq162_e2047_d_n0, eq162_e2047_d_n1, eq162_e2047_d_n2, eq162_e2047_d_n3, eq162_e2047_d_n4, eq162_e2047_d_n5, eq162_e2047_d_n6, eq162_e2047_d_n7, eq162_e2047_d_n8, eq162_e2047_d_n9, eq162_e2047_d_n10, eq162_e2047_d_n11, eq162_e2047_d_n12, eq162_e2047_d_n13, eq162_e2047_d_n14, eq162_e2047_d_n15, eq162_e2047_d_n16, eq162_e2047_d_n17, eq162_e2047_d_n18, eq162_e2047_d_n19, eq162_e2047_d_n20, eq162_e2047_d_n21, eq162_e2047_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_value: f64 = eq162_e2049;
        let eq162_node_derivatives: [f64; 23] = [eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n10, eq162_e2049_d_n11, eq162_e2049_d_n12, eq162_e2049_d_n13, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22];
        let eq162_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            self.multiplicity * (eq162_value),
            &nodes,
            &eq162_node_derivatives,
            &branches,
            &eq162_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_163_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq163_e2061, eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n10, eq163_e2061_d_n11, eq163_e2061_d_n12, eq163_e2061_d_n13, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22,) = {
    if (((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) && (s.v[589] != 0.0)) {
        let eq163_e2058: f64 = self.eval_ddt(62, s.v[264]);
        let eq163_e2058_d_n0: f64 = self.ddt_jacobian(s.dn[264][0]);
        let eq163_e2058_d_n1: f64 = self.ddt_jacobian(s.dn[264][1]);
        let eq163_e2058_d_n2: f64 = self.ddt_jacobian(s.dn[264][2]);
        let eq163_e2058_d_n3: f64 = self.ddt_jacobian(s.dn[264][3]);
        let eq163_e2058_d_n4: f64 = self.ddt_jacobian(s.dn[264][4]);
        let eq163_e2058_d_n5: f64 = self.ddt_jacobian(s.dn[264][5]);
        let eq163_e2058_d_n6: f64 = self.ddt_jacobian(s.dn[264][6]);
        let eq163_e2058_d_n7: f64 = self.ddt_jacobian(s.dn[264][7]);
        let eq163_e2058_d_n8: f64 = self.ddt_jacobian(s.dn[264][8]);
        let eq163_e2058_d_n9: f64 = self.ddt_jacobian(s.dn[264][9]);
        let eq163_e2058_d_n10: f64 = self.ddt_jacobian(s.dn[264][10]);
        let eq163_e2058_d_n11: f64 = self.ddt_jacobian(s.dn[264][11]);
        let eq163_e2058_d_n12: f64 = self.ddt_jacobian(s.dn[264][12]);
        let eq163_e2058_d_n13: f64 = self.ddt_jacobian(s.dn[264][13]);
        let eq163_e2058_d_n14: f64 = self.ddt_jacobian(s.dn[264][14]);
        let eq163_e2058_d_n15: f64 = self.ddt_jacobian(s.dn[264][15]);
        let eq163_e2058_d_n16: f64 = self.ddt_jacobian(s.dn[264][16]);
        let eq163_e2058_d_n17: f64 = self.ddt_jacobian(s.dn[264][17]);
        let eq163_e2058_d_n18: f64 = self.ddt_jacobian(s.dn[264][18]);
        let eq163_e2058_d_n19: f64 = self.ddt_jacobian(s.dn[264][19]);
        let eq163_e2058_d_n20: f64 = self.ddt_jacobian(s.dn[264][20]);
        let eq163_e2058_d_n21: f64 = self.ddt_jacobian(s.dn[264][21]);
        let eq163_e2058_d_n22: f64 = self.ddt_jacobian(s.dn[264][22]);
        let eq163_e2059: f64 = (p.p7 * eq163_e2058);
        let eq163_e2059_d_n0: f64 = (p.p7 * eq163_e2058_d_n0);
        let eq163_e2059_d_n1: f64 = (p.p7 * eq163_e2058_d_n1);
        let eq163_e2059_d_n2: f64 = (p.p7 * eq163_e2058_d_n2);
        let eq163_e2059_d_n3: f64 = (p.p7 * eq163_e2058_d_n3);
        let eq163_e2059_d_n4: f64 = (p.p7 * eq163_e2058_d_n4);
        let eq163_e2059_d_n5: f64 = (p.p7 * eq163_e2058_d_n5);
        let eq163_e2059_d_n6: f64 = (p.p7 * eq163_e2058_d_n6);
        let eq163_e2059_d_n7: f64 = (p.p7 * eq163_e2058_d_n7);
        let eq163_e2059_d_n8: f64 = (p.p7 * eq163_e2058_d_n8);
        let eq163_e2059_d_n9: f64 = (p.p7 * eq163_e2058_d_n9);
        let eq163_e2059_d_n10: f64 = (p.p7 * eq163_e2058_d_n10);
        let eq163_e2059_d_n11: f64 = (p.p7 * eq163_e2058_d_n11);
        let eq163_e2059_d_n12: f64 = (p.p7 * eq163_e2058_d_n12);
        let eq163_e2059_d_n13: f64 = (p.p7 * eq163_e2058_d_n13);
        let eq163_e2059_d_n14: f64 = (p.p7 * eq163_e2058_d_n14);
        let eq163_e2059_d_n15: f64 = (p.p7 * eq163_e2058_d_n15);
        let eq163_e2059_d_n16: f64 = (p.p7 * eq163_e2058_d_n16);
        let eq163_e2059_d_n17: f64 = (p.p7 * eq163_e2058_d_n17);
        let eq163_e2059_d_n18: f64 = (p.p7 * eq163_e2058_d_n18);
        let eq163_e2059_d_n19: f64 = (p.p7 * eq163_e2058_d_n19);
        let eq163_e2059_d_n20: f64 = (p.p7 * eq163_e2058_d_n20);
        let eq163_e2059_d_n21: f64 = (p.p7 * eq163_e2058_d_n21);
        let eq163_e2059_d_n22: f64 = (p.p7 * eq163_e2058_d_n22);
        (eq163_e2059, eq163_e2059_d_n0, eq163_e2059_d_n1, eq163_e2059_d_n2, eq163_e2059_d_n3, eq163_e2059_d_n4, eq163_e2059_d_n5, eq163_e2059_d_n6, eq163_e2059_d_n7, eq163_e2059_d_n8, eq163_e2059_d_n9, eq163_e2059_d_n10, eq163_e2059_d_n11, eq163_e2059_d_n12, eq163_e2059_d_n13, eq163_e2059_d_n14, eq163_e2059_d_n15, eq163_e2059_d_n16, eq163_e2059_d_n17, eq163_e2059_d_n18, eq163_e2059_d_n19, eq163_e2059_d_n20, eq163_e2059_d_n21, eq163_e2059_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_value: f64 = eq163_e2061;
        let eq163_node_derivatives: [f64; 23] = [eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n10, eq163_e2061_d_n11, eq163_e2061_d_n12, eq163_e2061_d_n13, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22];
        let eq163_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq163_value),
            &nodes,
            &eq163_node_derivatives,
            &branches,
            &eq163_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_164_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq164_e2075, eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n10, eq164_e2075_d_n11, eq164_e2075_d_n12, eq164_e2075_d_n13, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22,) = {
    if (((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) && (s.v[589] != 0.0)) {
        let eq164_e2070: f64 = self.eval_ddt(63, s.v[264]);
        let eq164_e2070_d_n0: f64 = self.ddt_jacobian(s.dn[264][0]);
        let eq164_e2070_d_n1: f64 = self.ddt_jacobian(s.dn[264][1]);
        let eq164_e2070_d_n2: f64 = self.ddt_jacobian(s.dn[264][2]);
        let eq164_e2070_d_n3: f64 = self.ddt_jacobian(s.dn[264][3]);
        let eq164_e2070_d_n4: f64 = self.ddt_jacobian(s.dn[264][4]);
        let eq164_e2070_d_n5: f64 = self.ddt_jacobian(s.dn[264][5]);
        let eq164_e2070_d_n6: f64 = self.ddt_jacobian(s.dn[264][6]);
        let eq164_e2070_d_n7: f64 = self.ddt_jacobian(s.dn[264][7]);
        let eq164_e2070_d_n8: f64 = self.ddt_jacobian(s.dn[264][8]);
        let eq164_e2070_d_n9: f64 = self.ddt_jacobian(s.dn[264][9]);
        let eq164_e2070_d_n10: f64 = self.ddt_jacobian(s.dn[264][10]);
        let eq164_e2070_d_n11: f64 = self.ddt_jacobian(s.dn[264][11]);
        let eq164_e2070_d_n12: f64 = self.ddt_jacobian(s.dn[264][12]);
        let eq164_e2070_d_n13: f64 = self.ddt_jacobian(s.dn[264][13]);
        let eq164_e2070_d_n14: f64 = self.ddt_jacobian(s.dn[264][14]);
        let eq164_e2070_d_n15: f64 = self.ddt_jacobian(s.dn[264][15]);
        let eq164_e2070_d_n16: f64 = self.ddt_jacobian(s.dn[264][16]);
        let eq164_e2070_d_n17: f64 = self.ddt_jacobian(s.dn[264][17]);
        let eq164_e2070_d_n18: f64 = self.ddt_jacobian(s.dn[264][18]);
        let eq164_e2070_d_n19: f64 = self.ddt_jacobian(s.dn[264][19]);
        let eq164_e2070_d_n20: f64 = self.ddt_jacobian(s.dn[264][20]);
        let eq164_e2070_d_n21: f64 = self.ddt_jacobian(s.dn[264][21]);
        let eq164_e2070_d_n22: f64 = self.ddt_jacobian(s.dn[264][22]);
        let eq164_e2071: f64 = (p.p7 * eq164_e2070);
        let eq164_e2071_d_n0: f64 = (p.p7 * eq164_e2070_d_n0);
        let eq164_e2071_d_n1: f64 = (p.p7 * eq164_e2070_d_n1);
        let eq164_e2071_d_n2: f64 = (p.p7 * eq164_e2070_d_n2);
        let eq164_e2071_d_n3: f64 = (p.p7 * eq164_e2070_d_n3);
        let eq164_e2071_d_n4: f64 = (p.p7 * eq164_e2070_d_n4);
        let eq164_e2071_d_n5: f64 = (p.p7 * eq164_e2070_d_n5);
        let eq164_e2071_d_n6: f64 = (p.p7 * eq164_e2070_d_n6);
        let eq164_e2071_d_n7: f64 = (p.p7 * eq164_e2070_d_n7);
        let eq164_e2071_d_n8: f64 = (p.p7 * eq164_e2070_d_n8);
        let eq164_e2071_d_n9: f64 = (p.p7 * eq164_e2070_d_n9);
        let eq164_e2071_d_n10: f64 = (p.p7 * eq164_e2070_d_n10);
        let eq164_e2071_d_n11: f64 = (p.p7 * eq164_e2070_d_n11);
        let eq164_e2071_d_n12: f64 = (p.p7 * eq164_e2070_d_n12);
        let eq164_e2071_d_n13: f64 = (p.p7 * eq164_e2070_d_n13);
        let eq164_e2071_d_n14: f64 = (p.p7 * eq164_e2070_d_n14);
        let eq164_e2071_d_n15: f64 = (p.p7 * eq164_e2070_d_n15);
        let eq164_e2071_d_n16: f64 = (p.p7 * eq164_e2070_d_n16);
        let eq164_e2071_d_n17: f64 = (p.p7 * eq164_e2070_d_n17);
        let eq164_e2071_d_n18: f64 = (p.p7 * eq164_e2070_d_n18);
        let eq164_e2071_d_n19: f64 = (p.p7 * eq164_e2070_d_n19);
        let eq164_e2071_d_n20: f64 = (p.p7 * eq164_e2070_d_n20);
        let eq164_e2071_d_n21: f64 = (p.p7 * eq164_e2070_d_n21);
        let eq164_e2071_d_n22: f64 = (p.p7 * eq164_e2070_d_n22);
        let eq164_e2073: f64 = (eq164_e2071 * p.p247);
        let eq164_e2073_d_n0: f64 = (eq164_e2071_d_n0 * p.p247);
        let eq164_e2073_d_n1: f64 = (eq164_e2071_d_n1 * p.p247);
        let eq164_e2073_d_n2: f64 = (eq164_e2071_d_n2 * p.p247);
        let eq164_e2073_d_n3: f64 = (eq164_e2071_d_n3 * p.p247);
        let eq164_e2073_d_n4: f64 = (eq164_e2071_d_n4 * p.p247);
        let eq164_e2073_d_n5: f64 = (eq164_e2071_d_n5 * p.p247);
        let eq164_e2073_d_n6: f64 = (eq164_e2071_d_n6 * p.p247);
        let eq164_e2073_d_n7: f64 = (eq164_e2071_d_n7 * p.p247);
        let eq164_e2073_d_n8: f64 = (eq164_e2071_d_n8 * p.p247);
        let eq164_e2073_d_n9: f64 = (eq164_e2071_d_n9 * p.p247);
        let eq164_e2073_d_n10: f64 = (eq164_e2071_d_n10 * p.p247);
        let eq164_e2073_d_n11: f64 = (eq164_e2071_d_n11 * p.p247);
        let eq164_e2073_d_n12: f64 = (eq164_e2071_d_n12 * p.p247);
        let eq164_e2073_d_n13: f64 = (eq164_e2071_d_n13 * p.p247);
        let eq164_e2073_d_n14: f64 = (eq164_e2071_d_n14 * p.p247);
        let eq164_e2073_d_n15: f64 = (eq164_e2071_d_n15 * p.p247);
        let eq164_e2073_d_n16: f64 = (eq164_e2071_d_n16 * p.p247);
        let eq164_e2073_d_n17: f64 = (eq164_e2071_d_n17 * p.p247);
        let eq164_e2073_d_n18: f64 = (eq164_e2071_d_n18 * p.p247);
        let eq164_e2073_d_n19: f64 = (eq164_e2071_d_n19 * p.p247);
        let eq164_e2073_d_n20: f64 = (eq164_e2071_d_n20 * p.p247);
        let eq164_e2073_d_n21: f64 = (eq164_e2071_d_n21 * p.p247);
        let eq164_e2073_d_n22: f64 = (eq164_e2071_d_n22 * p.p247);
        (eq164_e2073, eq164_e2073_d_n0, eq164_e2073_d_n1, eq164_e2073_d_n2, eq164_e2073_d_n3, eq164_e2073_d_n4, eq164_e2073_d_n5, eq164_e2073_d_n6, eq164_e2073_d_n7, eq164_e2073_d_n8, eq164_e2073_d_n9, eq164_e2073_d_n10, eq164_e2073_d_n11, eq164_e2073_d_n12, eq164_e2073_d_n13, eq164_e2073_d_n14, eq164_e2073_d_n15, eq164_e2073_d_n16, eq164_e2073_d_n17, eq164_e2073_d_n18, eq164_e2073_d_n19, eq164_e2073_d_n20, eq164_e2073_d_n21, eq164_e2073_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_value: f64 = eq164_e2075;
        let eq164_node_derivatives: [f64; 23] = [eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n10, eq164_e2075_d_n11, eq164_e2075_d_n12, eq164_e2075_d_n13, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22];
        let eq164_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq164_value),
            &nodes,
            &eq164_node_derivatives,
            &branches,
            &eq164_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_165_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq165_e2088, eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n10, eq165_e2088_d_n11, eq165_e2088_d_n12, eq165_e2088_d_n13, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22,) = {
    if (((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) && (!(s.v[589] != 0.0))) {
        let eq165_e2085: f64 = self.eval_ddt(64, s.v[264]);
        let eq165_e2085_d_n0: f64 = self.ddt_jacobian(s.dn[264][0]);
        let eq165_e2085_d_n1: f64 = self.ddt_jacobian(s.dn[264][1]);
        let eq165_e2085_d_n2: f64 = self.ddt_jacobian(s.dn[264][2]);
        let eq165_e2085_d_n3: f64 = self.ddt_jacobian(s.dn[264][3]);
        let eq165_e2085_d_n4: f64 = self.ddt_jacobian(s.dn[264][4]);
        let eq165_e2085_d_n5: f64 = self.ddt_jacobian(s.dn[264][5]);
        let eq165_e2085_d_n6: f64 = self.ddt_jacobian(s.dn[264][6]);
        let eq165_e2085_d_n7: f64 = self.ddt_jacobian(s.dn[264][7]);
        let eq165_e2085_d_n8: f64 = self.ddt_jacobian(s.dn[264][8]);
        let eq165_e2085_d_n9: f64 = self.ddt_jacobian(s.dn[264][9]);
        let eq165_e2085_d_n10: f64 = self.ddt_jacobian(s.dn[264][10]);
        let eq165_e2085_d_n11: f64 = self.ddt_jacobian(s.dn[264][11]);
        let eq165_e2085_d_n12: f64 = self.ddt_jacobian(s.dn[264][12]);
        let eq165_e2085_d_n13: f64 = self.ddt_jacobian(s.dn[264][13]);
        let eq165_e2085_d_n14: f64 = self.ddt_jacobian(s.dn[264][14]);
        let eq165_e2085_d_n15: f64 = self.ddt_jacobian(s.dn[264][15]);
        let eq165_e2085_d_n16: f64 = self.ddt_jacobian(s.dn[264][16]);
        let eq165_e2085_d_n17: f64 = self.ddt_jacobian(s.dn[264][17]);
        let eq165_e2085_d_n18: f64 = self.ddt_jacobian(s.dn[264][18]);
        let eq165_e2085_d_n19: f64 = self.ddt_jacobian(s.dn[264][19]);
        let eq165_e2085_d_n20: f64 = self.ddt_jacobian(s.dn[264][20]);
        let eq165_e2085_d_n21: f64 = self.ddt_jacobian(s.dn[264][21]);
        let eq165_e2085_d_n22: f64 = self.ddt_jacobian(s.dn[264][22]);
        let eq165_e2086: f64 = (p.p7 * eq165_e2085);
        let eq165_e2086_d_n0: f64 = (p.p7 * eq165_e2085_d_n0);
        let eq165_e2086_d_n1: f64 = (p.p7 * eq165_e2085_d_n1);
        let eq165_e2086_d_n2: f64 = (p.p7 * eq165_e2085_d_n2);
        let eq165_e2086_d_n3: f64 = (p.p7 * eq165_e2085_d_n3);
        let eq165_e2086_d_n4: f64 = (p.p7 * eq165_e2085_d_n4);
        let eq165_e2086_d_n5: f64 = (p.p7 * eq165_e2085_d_n5);
        let eq165_e2086_d_n6: f64 = (p.p7 * eq165_e2085_d_n6);
        let eq165_e2086_d_n7: f64 = (p.p7 * eq165_e2085_d_n7);
        let eq165_e2086_d_n8: f64 = (p.p7 * eq165_e2085_d_n8);
        let eq165_e2086_d_n9: f64 = (p.p7 * eq165_e2085_d_n9);
        let eq165_e2086_d_n10: f64 = (p.p7 * eq165_e2085_d_n10);
        let eq165_e2086_d_n11: f64 = (p.p7 * eq165_e2085_d_n11);
        let eq165_e2086_d_n12: f64 = (p.p7 * eq165_e2085_d_n12);
        let eq165_e2086_d_n13: f64 = (p.p7 * eq165_e2085_d_n13);
        let eq165_e2086_d_n14: f64 = (p.p7 * eq165_e2085_d_n14);
        let eq165_e2086_d_n15: f64 = (p.p7 * eq165_e2085_d_n15);
        let eq165_e2086_d_n16: f64 = (p.p7 * eq165_e2085_d_n16);
        let eq165_e2086_d_n17: f64 = (p.p7 * eq165_e2085_d_n17);
        let eq165_e2086_d_n18: f64 = (p.p7 * eq165_e2085_d_n18);
        let eq165_e2086_d_n19: f64 = (p.p7 * eq165_e2085_d_n19);
        let eq165_e2086_d_n20: f64 = (p.p7 * eq165_e2085_d_n20);
        let eq165_e2086_d_n21: f64 = (p.p7 * eq165_e2085_d_n21);
        let eq165_e2086_d_n22: f64 = (p.p7 * eq165_e2085_d_n22);
        (eq165_e2086, eq165_e2086_d_n0, eq165_e2086_d_n1, eq165_e2086_d_n2, eq165_e2086_d_n3, eq165_e2086_d_n4, eq165_e2086_d_n5, eq165_e2086_d_n6, eq165_e2086_d_n7, eq165_e2086_d_n8, eq165_e2086_d_n9, eq165_e2086_d_n10, eq165_e2086_d_n11, eq165_e2086_d_n12, eq165_e2086_d_n13, eq165_e2086_d_n14, eq165_e2086_d_n15, eq165_e2086_d_n16, eq165_e2086_d_n17, eq165_e2086_d_n18, eq165_e2086_d_n19, eq165_e2086_d_n20, eq165_e2086_d_n21, eq165_e2086_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq165_value: f64 = eq165_e2088;
        let eq165_node_derivatives: [f64; 23] = [eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n10, eq165_e2088_d_n11, eq165_e2088_d_n12, eq165_e2088_d_n13, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22];
        let eq165_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq165_value),
            &nodes,
            &eq165_node_derivatives,
            &branches,
            &eq165_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_166_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq166_e2103, eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n10, eq166_e2103_d_n11, eq166_e2103_d_n12, eq166_e2103_d_n13, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22,) = {
    if (((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) && (!(s.v[589] != 0.0))) {
        let eq166_e2098: f64 = self.eval_ddt(65, s.v[264]);
        let eq166_e2098_d_n0: f64 = self.ddt_jacobian(s.dn[264][0]);
        let eq166_e2098_d_n1: f64 = self.ddt_jacobian(s.dn[264][1]);
        let eq166_e2098_d_n2: f64 = self.ddt_jacobian(s.dn[264][2]);
        let eq166_e2098_d_n3: f64 = self.ddt_jacobian(s.dn[264][3]);
        let eq166_e2098_d_n4: f64 = self.ddt_jacobian(s.dn[264][4]);
        let eq166_e2098_d_n5: f64 = self.ddt_jacobian(s.dn[264][5]);
        let eq166_e2098_d_n6: f64 = self.ddt_jacobian(s.dn[264][6]);
        let eq166_e2098_d_n7: f64 = self.ddt_jacobian(s.dn[264][7]);
        let eq166_e2098_d_n8: f64 = self.ddt_jacobian(s.dn[264][8]);
        let eq166_e2098_d_n9: f64 = self.ddt_jacobian(s.dn[264][9]);
        let eq166_e2098_d_n10: f64 = self.ddt_jacobian(s.dn[264][10]);
        let eq166_e2098_d_n11: f64 = self.ddt_jacobian(s.dn[264][11]);
        let eq166_e2098_d_n12: f64 = self.ddt_jacobian(s.dn[264][12]);
        let eq166_e2098_d_n13: f64 = self.ddt_jacobian(s.dn[264][13]);
        let eq166_e2098_d_n14: f64 = self.ddt_jacobian(s.dn[264][14]);
        let eq166_e2098_d_n15: f64 = self.ddt_jacobian(s.dn[264][15]);
        let eq166_e2098_d_n16: f64 = self.ddt_jacobian(s.dn[264][16]);
        let eq166_e2098_d_n17: f64 = self.ddt_jacobian(s.dn[264][17]);
        let eq166_e2098_d_n18: f64 = self.ddt_jacobian(s.dn[264][18]);
        let eq166_e2098_d_n19: f64 = self.ddt_jacobian(s.dn[264][19]);
        let eq166_e2098_d_n20: f64 = self.ddt_jacobian(s.dn[264][20]);
        let eq166_e2098_d_n21: f64 = self.ddt_jacobian(s.dn[264][21]);
        let eq166_e2098_d_n22: f64 = self.ddt_jacobian(s.dn[264][22]);
        let eq166_e2099: f64 = (p.p7 * eq166_e2098);
        let eq166_e2099_d_n0: f64 = (p.p7 * eq166_e2098_d_n0);
        let eq166_e2099_d_n1: f64 = (p.p7 * eq166_e2098_d_n1);
        let eq166_e2099_d_n2: f64 = (p.p7 * eq166_e2098_d_n2);
        let eq166_e2099_d_n3: f64 = (p.p7 * eq166_e2098_d_n3);
        let eq166_e2099_d_n4: f64 = (p.p7 * eq166_e2098_d_n4);
        let eq166_e2099_d_n5: f64 = (p.p7 * eq166_e2098_d_n5);
        let eq166_e2099_d_n6: f64 = (p.p7 * eq166_e2098_d_n6);
        let eq166_e2099_d_n7: f64 = (p.p7 * eq166_e2098_d_n7);
        let eq166_e2099_d_n8: f64 = (p.p7 * eq166_e2098_d_n8);
        let eq166_e2099_d_n9: f64 = (p.p7 * eq166_e2098_d_n9);
        let eq166_e2099_d_n10: f64 = (p.p7 * eq166_e2098_d_n10);
        let eq166_e2099_d_n11: f64 = (p.p7 * eq166_e2098_d_n11);
        let eq166_e2099_d_n12: f64 = (p.p7 * eq166_e2098_d_n12);
        let eq166_e2099_d_n13: f64 = (p.p7 * eq166_e2098_d_n13);
        let eq166_e2099_d_n14: f64 = (p.p7 * eq166_e2098_d_n14);
        let eq166_e2099_d_n15: f64 = (p.p7 * eq166_e2098_d_n15);
        let eq166_e2099_d_n16: f64 = (p.p7 * eq166_e2098_d_n16);
        let eq166_e2099_d_n17: f64 = (p.p7 * eq166_e2098_d_n17);
        let eq166_e2099_d_n18: f64 = (p.p7 * eq166_e2098_d_n18);
        let eq166_e2099_d_n19: f64 = (p.p7 * eq166_e2098_d_n19);
        let eq166_e2099_d_n20: f64 = (p.p7 * eq166_e2098_d_n20);
        let eq166_e2099_d_n21: f64 = (p.p7 * eq166_e2098_d_n21);
        let eq166_e2099_d_n22: f64 = (p.p7 * eq166_e2098_d_n22);
        let eq166_e2101: f64 = (eq166_e2099 * p.p247);
        let eq166_e2101_d_n0: f64 = (eq166_e2099_d_n0 * p.p247);
        let eq166_e2101_d_n1: f64 = (eq166_e2099_d_n1 * p.p247);
        let eq166_e2101_d_n2: f64 = (eq166_e2099_d_n2 * p.p247);
        let eq166_e2101_d_n3: f64 = (eq166_e2099_d_n3 * p.p247);
        let eq166_e2101_d_n4: f64 = (eq166_e2099_d_n4 * p.p247);
        let eq166_e2101_d_n5: f64 = (eq166_e2099_d_n5 * p.p247);
        let eq166_e2101_d_n6: f64 = (eq166_e2099_d_n6 * p.p247);
        let eq166_e2101_d_n7: f64 = (eq166_e2099_d_n7 * p.p247);
        let eq166_e2101_d_n8: f64 = (eq166_e2099_d_n8 * p.p247);
        let eq166_e2101_d_n9: f64 = (eq166_e2099_d_n9 * p.p247);
        let eq166_e2101_d_n10: f64 = (eq166_e2099_d_n10 * p.p247);
        let eq166_e2101_d_n11: f64 = (eq166_e2099_d_n11 * p.p247);
        let eq166_e2101_d_n12: f64 = (eq166_e2099_d_n12 * p.p247);
        let eq166_e2101_d_n13: f64 = (eq166_e2099_d_n13 * p.p247);
        let eq166_e2101_d_n14: f64 = (eq166_e2099_d_n14 * p.p247);
        let eq166_e2101_d_n15: f64 = (eq166_e2099_d_n15 * p.p247);
        let eq166_e2101_d_n16: f64 = (eq166_e2099_d_n16 * p.p247);
        let eq166_e2101_d_n17: f64 = (eq166_e2099_d_n17 * p.p247);
        let eq166_e2101_d_n18: f64 = (eq166_e2099_d_n18 * p.p247);
        let eq166_e2101_d_n19: f64 = (eq166_e2099_d_n19 * p.p247);
        let eq166_e2101_d_n20: f64 = (eq166_e2099_d_n20 * p.p247);
        let eq166_e2101_d_n21: f64 = (eq166_e2099_d_n21 * p.p247);
        let eq166_e2101_d_n22: f64 = (eq166_e2099_d_n22 * p.p247);
        (eq166_e2101, eq166_e2101_d_n0, eq166_e2101_d_n1, eq166_e2101_d_n2, eq166_e2101_d_n3, eq166_e2101_d_n4, eq166_e2101_d_n5, eq166_e2101_d_n6, eq166_e2101_d_n7, eq166_e2101_d_n8, eq166_e2101_d_n9, eq166_e2101_d_n10, eq166_e2101_d_n11, eq166_e2101_d_n12, eq166_e2101_d_n13, eq166_e2101_d_n14, eq166_e2101_d_n15, eq166_e2101_d_n16, eq166_e2101_d_n17, eq166_e2101_d_n18, eq166_e2101_d_n19, eq166_e2101_d_n20, eq166_e2101_d_n21, eq166_e2101_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_value: f64 = eq166_e2103;
        let eq166_node_derivatives: [f64; 23] = [eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n10, eq166_e2103_d_n11, eq166_e2103_d_n12, eq166_e2103_d_n13, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22];
        let eq166_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq166_value),
            &nodes,
            &eq166_node_derivatives,
            &branches,
            &eq166_branch_derivatives,
            self.multiplicity,
        );
    }
}
