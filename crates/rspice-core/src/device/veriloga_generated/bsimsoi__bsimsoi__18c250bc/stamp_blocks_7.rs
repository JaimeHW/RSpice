#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq74_e1947, eq74_e1947_d_n0, eq74_e1947_d_n1, eq74_e1947_d_n2, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12, eq74_e1947_d_n13,) = {
    if (((((s.v[1559] != 0.0) && (s.v[1560] != 0.0)) && (!(s.v[1561] != 0.0))) && (!(s.v[1562] != 0.0))) && (!(s.v[1563] != 0.0))) {
        let eq74_e1934: f64 = (-s.v[885]);
        let eq74_e1934_d_n0: f64 = (-s.dn[885][0]);
        let eq74_e1934_d_n1: f64 = (-s.dn[885][1]);
        let eq74_e1934_d_n2: f64 = (-s.dn[885][2]);
        let eq74_e1934_d_n3: f64 = (-s.dn[885][3]);
        let eq74_e1934_d_n4: f64 = (-s.dn[885][4]);
        let eq74_e1934_d_n5: f64 = (-s.dn[885][5]);
        let eq74_e1934_d_n6: f64 = (-s.dn[885][6]);
        let eq74_e1934_d_n7: f64 = (-s.dn[885][7]);
        let eq74_e1934_d_n8: f64 = (-s.dn[885][8]);
        let eq74_e1934_d_n9: f64 = (-s.dn[885][9]);
        let eq74_e1934_d_n10: f64 = (-s.dn[885][10]);
        let eq74_e1934_d_n11: f64 = (-s.dn[885][11]);
        let eq74_e1934_d_n12: f64 = (-s.dn[885][12]);
        let eq74_e1934_d_n13: f64 = (-s.dn[885][13]);
        let eq74_e1936: f64 = (eq74_e1934 * s.v[822]);
        let eq74_e1936_d_n0: f64 = ((eq74_e1934_d_n0 * s.v[822]) + (eq74_e1934 * s.dn[822][0]));
        let eq74_e1936_d_n1: f64 = ((eq74_e1934_d_n1 * s.v[822]) + (eq74_e1934 * s.dn[822][1]));
        let eq74_e1936_d_n2: f64 = ((eq74_e1934_d_n2 * s.v[822]) + (eq74_e1934 * s.dn[822][2]));
        let eq74_e1936_d_n3: f64 = ((eq74_e1934_d_n3 * s.v[822]) + (eq74_e1934 * s.dn[822][3]));
        let eq74_e1936_d_n4: f64 = ((eq74_e1934_d_n4 * s.v[822]) + (eq74_e1934 * s.dn[822][4]));
        let eq74_e1936_d_n5: f64 = ((eq74_e1934_d_n5 * s.v[822]) + (eq74_e1934 * s.dn[822][5]));
        let eq74_e1936_d_n6: f64 = ((eq74_e1934_d_n6 * s.v[822]) + (eq74_e1934 * s.dn[822][6]));
        let eq74_e1936_d_n7: f64 = ((eq74_e1934_d_n7 * s.v[822]) + (eq74_e1934 * s.dn[822][7]));
        let eq74_e1936_d_n8: f64 = ((eq74_e1934_d_n8 * s.v[822]) + (eq74_e1934 * s.dn[822][8]));
        let eq74_e1936_d_n9: f64 = ((eq74_e1934_d_n9 * s.v[822]) + (eq74_e1934 * s.dn[822][9]));
        let eq74_e1936_d_n10: f64 = ((eq74_e1934_d_n10 * s.v[822]) + (eq74_e1934 * s.dn[822][10]));
        let eq74_e1936_d_n11: f64 = ((eq74_e1934_d_n11 * s.v[822]) + (eq74_e1934 * s.dn[822][11]));
        let eq74_e1936_d_n12: f64 = ((eq74_e1934_d_n12 * s.v[822]) + (eq74_e1934 * s.dn[822][12]));
        let eq74_e1936_d_n13: f64 = ((eq74_e1934_d_n13 * s.v[822]) + (eq74_e1934 * s.dn[822][13]));
        let eq74_e1939: f64 = (s.v[410] * s.v[158]);
        let eq74_e1939_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq74_e1939_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq74_e1939_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq74_e1939_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq74_e1939_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq74_e1939_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq74_e1939_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq74_e1939_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq74_e1939_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq74_e1939_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq74_e1939_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq74_e1939_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq74_e1939_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq74_e1939_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq74_e1940: f64 = self.eval_ddt(19, eq74_e1939);
        let eq74_e1940_d_n0: f64 = self.ddt_jacobian(eq74_e1939_d_n0);
        let eq74_e1940_d_n1: f64 = self.ddt_jacobian(eq74_e1939_d_n1);
        let eq74_e1940_d_n2: f64 = self.ddt_jacobian(eq74_e1939_d_n2);
        let eq74_e1940_d_n3: f64 = self.ddt_jacobian(eq74_e1939_d_n3);
        let eq74_e1940_d_n4: f64 = self.ddt_jacobian(eq74_e1939_d_n4);
        let eq74_e1940_d_n5: f64 = self.ddt_jacobian(eq74_e1939_d_n5);
        let eq74_e1940_d_n6: f64 = self.ddt_jacobian(eq74_e1939_d_n6);
        let eq74_e1940_d_n7: f64 = self.ddt_jacobian(eq74_e1939_d_n7);
        let eq74_e1940_d_n8: f64 = self.ddt_jacobian(eq74_e1939_d_n8);
        let eq74_e1940_d_n9: f64 = self.ddt_jacobian(eq74_e1939_d_n9);
        let eq74_e1940_d_n10: f64 = self.ddt_jacobian(eq74_e1939_d_n10);
        let eq74_e1940_d_n11: f64 = self.ddt_jacobian(eq74_e1939_d_n11);
        let eq74_e1940_d_n12: f64 = self.ddt_jacobian(eq74_e1939_d_n12);
        let eq74_e1940_d_n13: f64 = self.ddt_jacobian(eq74_e1939_d_n13);
        let eq74_e1941: f64 = (eq74_e1936 + eq74_e1940);
        let eq74_e1941_d_n0: f64 = (eq74_e1936_d_n0 + eq74_e1940_d_n0);
        let eq74_e1941_d_n1: f64 = (eq74_e1936_d_n1 + eq74_e1940_d_n1);
        let eq74_e1941_d_n2: f64 = (eq74_e1936_d_n2 + eq74_e1940_d_n2);
        let eq74_e1941_d_n3: f64 = (eq74_e1936_d_n3 + eq74_e1940_d_n3);
        let eq74_e1941_d_n4: f64 = (eq74_e1936_d_n4 + eq74_e1940_d_n4);
        let eq74_e1941_d_n5: f64 = (eq74_e1936_d_n5 + eq74_e1940_d_n5);
        let eq74_e1941_d_n6: f64 = (eq74_e1936_d_n6 + eq74_e1940_d_n6);
        let eq74_e1941_d_n7: f64 = (eq74_e1936_d_n7 + eq74_e1940_d_n7);
        let eq74_e1941_d_n8: f64 = (eq74_e1936_d_n8 + eq74_e1940_d_n8);
        let eq74_e1941_d_n9: f64 = (eq74_e1936_d_n9 + eq74_e1940_d_n9);
        let eq74_e1941_d_n10: f64 = (eq74_e1936_d_n10 + eq74_e1940_d_n10);
        let eq74_e1941_d_n11: f64 = (eq74_e1936_d_n11 + eq74_e1940_d_n11);
        let eq74_e1941_d_n12: f64 = (eq74_e1936_d_n12 + eq74_e1940_d_n12);
        let eq74_e1941_d_n13: f64 = (eq74_e1936_d_n13 + eq74_e1940_d_n13);
        let eq74_e1944: f64 = (s.v[410] / s.v[157]);
        let eq74_e1944_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq74_e1944_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq74_e1944_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq74_e1944_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq74_e1944_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq74_e1944_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq74_e1944_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq74_e1944_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq74_e1944_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq74_e1944_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq74_e1944_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq74_e1944_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq74_e1944_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq74_e1944_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq74_e1945: f64 = (eq74_e1941 + eq74_e1944);
        let eq74_e1945_d_n0: f64 = (eq74_e1941_d_n0 + eq74_e1944_d_n0);
        let eq74_e1945_d_n1: f64 = (eq74_e1941_d_n1 + eq74_e1944_d_n1);
        let eq74_e1945_d_n2: f64 = (eq74_e1941_d_n2 + eq74_e1944_d_n2);
        let eq74_e1945_d_n3: f64 = (eq74_e1941_d_n3 + eq74_e1944_d_n3);
        let eq74_e1945_d_n4: f64 = (eq74_e1941_d_n4 + eq74_e1944_d_n4);
        let eq74_e1945_d_n5: f64 = (eq74_e1941_d_n5 + eq74_e1944_d_n5);
        let eq74_e1945_d_n6: f64 = (eq74_e1941_d_n6 + eq74_e1944_d_n6);
        let eq74_e1945_d_n7: f64 = (eq74_e1941_d_n7 + eq74_e1944_d_n7);
        let eq74_e1945_d_n8: f64 = (eq74_e1941_d_n8 + eq74_e1944_d_n8);
        let eq74_e1945_d_n9: f64 = (eq74_e1941_d_n9 + eq74_e1944_d_n9);
        let eq74_e1945_d_n10: f64 = (eq74_e1941_d_n10 + eq74_e1944_d_n10);
        let eq74_e1945_d_n11: f64 = (eq74_e1941_d_n11 + eq74_e1944_d_n11);
        let eq74_e1945_d_n12: f64 = (eq74_e1941_d_n12 + eq74_e1944_d_n12);
        let eq74_e1945_d_n13: f64 = (eq74_e1941_d_n13 + eq74_e1944_d_n13);
        (eq74_e1945, eq74_e1945_d_n0, eq74_e1945_d_n1, eq74_e1945_d_n2, eq74_e1945_d_n3, eq74_e1945_d_n4, eq74_e1945_d_n5, eq74_e1945_d_n6, eq74_e1945_d_n7, eq74_e1945_d_n8, eq74_e1945_d_n9, eq74_e1945_d_n10, eq74_e1945_d_n11, eq74_e1945_d_n12, eq74_e1945_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1947;
        let eq74_node_derivatives: [f64; 14] = [eq74_e1947_d_n0, eq74_e1947_d_n1, eq74_e1947_d_n2, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12, eq74_e1947_d_n13];
        let eq74_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            None,
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
        let (eq75_e1970, eq75_e1970_d_n0, eq75_e1970_d_n1, eq75_e1970_d_n2, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12, eq75_e1970_d_n13,) = {
    if (((s.v[1559] != 0.0) && (!(s.v[1560] != 0.0))) && (s.v[1564] != 0.0)) {
        let eq75_e1956: f64 = (s.v[885] / p.p30);
        let eq75_e1956_d_n0: f64 = (s.dn[885][0] / p.p30);
        let eq75_e1956_d_n1: f64 = (s.dn[885][1] / p.p30);
        let eq75_e1956_d_n2: f64 = (s.dn[885][2] / p.p30);
        let eq75_e1956_d_n3: f64 = (s.dn[885][3] / p.p30);
        let eq75_e1956_d_n4: f64 = (s.dn[885][4] / p.p30);
        let eq75_e1956_d_n5: f64 = (s.dn[885][5] / p.p30);
        let eq75_e1956_d_n6: f64 = (s.dn[885][6] / p.p30);
        let eq75_e1956_d_n7: f64 = (s.dn[885][7] / p.p30);
        let eq75_e1956_d_n8: f64 = (s.dn[885][8] / p.p30);
        let eq75_e1956_d_n9: f64 = (s.dn[885][9] / p.p30);
        let eq75_e1956_d_n10: f64 = (s.dn[885][10] / p.p30);
        let eq75_e1956_d_n11: f64 = (s.dn[885][11] / p.p30);
        let eq75_e1956_d_n12: f64 = (s.dn[885][12] / p.p30);
        let eq75_e1956_d_n13: f64 = (s.dn[885][13] / p.p30);
        let eq75_e1957: f64 = (-eq75_e1956);
        let eq75_e1957_d_n0: f64 = (-eq75_e1956_d_n0);
        let eq75_e1957_d_n1: f64 = (-eq75_e1956_d_n1);
        let eq75_e1957_d_n2: f64 = (-eq75_e1956_d_n2);
        let eq75_e1957_d_n3: f64 = (-eq75_e1956_d_n3);
        let eq75_e1957_d_n4: f64 = (-eq75_e1956_d_n4);
        let eq75_e1957_d_n5: f64 = (-eq75_e1956_d_n5);
        let eq75_e1957_d_n6: f64 = (-eq75_e1956_d_n6);
        let eq75_e1957_d_n7: f64 = (-eq75_e1956_d_n7);
        let eq75_e1957_d_n8: f64 = (-eq75_e1956_d_n8);
        let eq75_e1957_d_n9: f64 = (-eq75_e1956_d_n9);
        let eq75_e1957_d_n10: f64 = (-eq75_e1956_d_n10);
        let eq75_e1957_d_n11: f64 = (-eq75_e1956_d_n11);
        let eq75_e1957_d_n12: f64 = (-eq75_e1956_d_n12);
        let eq75_e1957_d_n13: f64 = (-eq75_e1956_d_n13);
        let eq75_e1959: f64 = (eq75_e1957 * s.v[822]);
        let eq75_e1959_d_n0: f64 = ((eq75_e1957_d_n0 * s.v[822]) + (eq75_e1957 * s.dn[822][0]));
        let eq75_e1959_d_n1: f64 = ((eq75_e1957_d_n1 * s.v[822]) + (eq75_e1957 * s.dn[822][1]));
        let eq75_e1959_d_n2: f64 = ((eq75_e1957_d_n2 * s.v[822]) + (eq75_e1957 * s.dn[822][2]));
        let eq75_e1959_d_n3: f64 = ((eq75_e1957_d_n3 * s.v[822]) + (eq75_e1957 * s.dn[822][3]));
        let eq75_e1959_d_n4: f64 = ((eq75_e1957_d_n4 * s.v[822]) + (eq75_e1957 * s.dn[822][4]));
        let eq75_e1959_d_n5: f64 = ((eq75_e1957_d_n5 * s.v[822]) + (eq75_e1957 * s.dn[822][5]));
        let eq75_e1959_d_n6: f64 = ((eq75_e1957_d_n6 * s.v[822]) + (eq75_e1957 * s.dn[822][6]));
        let eq75_e1959_d_n7: f64 = ((eq75_e1957_d_n7 * s.v[822]) + (eq75_e1957 * s.dn[822][7]));
        let eq75_e1959_d_n8: f64 = ((eq75_e1957_d_n8 * s.v[822]) + (eq75_e1957 * s.dn[822][8]));
        let eq75_e1959_d_n9: f64 = ((eq75_e1957_d_n9 * s.v[822]) + (eq75_e1957 * s.dn[822][9]));
        let eq75_e1959_d_n10: f64 = ((eq75_e1957_d_n10 * s.v[822]) + (eq75_e1957 * s.dn[822][10]));
        let eq75_e1959_d_n11: f64 = ((eq75_e1957_d_n11 * s.v[822]) + (eq75_e1957 * s.dn[822][11]));
        let eq75_e1959_d_n12: f64 = ((eq75_e1957_d_n12 * s.v[822]) + (eq75_e1957 * s.dn[822][12]));
        let eq75_e1959_d_n13: f64 = ((eq75_e1957_d_n13 * s.v[822]) + (eq75_e1957 * s.dn[822][13]));
        let eq75_e1962: f64 = (s.v[410] * s.v[158]);
        let eq75_e1962_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq75_e1962_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq75_e1962_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq75_e1962_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq75_e1962_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq75_e1962_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq75_e1962_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq75_e1962_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq75_e1962_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq75_e1962_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq75_e1962_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq75_e1962_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq75_e1962_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq75_e1962_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq75_e1963: f64 = self.eval_ddt(20, eq75_e1962);
        let eq75_e1963_d_n0: f64 = self.ddt_jacobian(eq75_e1962_d_n0);
        let eq75_e1963_d_n1: f64 = self.ddt_jacobian(eq75_e1962_d_n1);
        let eq75_e1963_d_n2: f64 = self.ddt_jacobian(eq75_e1962_d_n2);
        let eq75_e1963_d_n3: f64 = self.ddt_jacobian(eq75_e1962_d_n3);
        let eq75_e1963_d_n4: f64 = self.ddt_jacobian(eq75_e1962_d_n4);
        let eq75_e1963_d_n5: f64 = self.ddt_jacobian(eq75_e1962_d_n5);
        let eq75_e1963_d_n6: f64 = self.ddt_jacobian(eq75_e1962_d_n6);
        let eq75_e1963_d_n7: f64 = self.ddt_jacobian(eq75_e1962_d_n7);
        let eq75_e1963_d_n8: f64 = self.ddt_jacobian(eq75_e1962_d_n8);
        let eq75_e1963_d_n9: f64 = self.ddt_jacobian(eq75_e1962_d_n9);
        let eq75_e1963_d_n10: f64 = self.ddt_jacobian(eq75_e1962_d_n10);
        let eq75_e1963_d_n11: f64 = self.ddt_jacobian(eq75_e1962_d_n11);
        let eq75_e1963_d_n12: f64 = self.ddt_jacobian(eq75_e1962_d_n12);
        let eq75_e1963_d_n13: f64 = self.ddt_jacobian(eq75_e1962_d_n13);
        let eq75_e1964: f64 = (eq75_e1959 + eq75_e1963);
        let eq75_e1964_d_n0: f64 = (eq75_e1959_d_n0 + eq75_e1963_d_n0);
        let eq75_e1964_d_n1: f64 = (eq75_e1959_d_n1 + eq75_e1963_d_n1);
        let eq75_e1964_d_n2: f64 = (eq75_e1959_d_n2 + eq75_e1963_d_n2);
        let eq75_e1964_d_n3: f64 = (eq75_e1959_d_n3 + eq75_e1963_d_n3);
        let eq75_e1964_d_n4: f64 = (eq75_e1959_d_n4 + eq75_e1963_d_n4);
        let eq75_e1964_d_n5: f64 = (eq75_e1959_d_n5 + eq75_e1963_d_n5);
        let eq75_e1964_d_n6: f64 = (eq75_e1959_d_n6 + eq75_e1963_d_n6);
        let eq75_e1964_d_n7: f64 = (eq75_e1959_d_n7 + eq75_e1963_d_n7);
        let eq75_e1964_d_n8: f64 = (eq75_e1959_d_n8 + eq75_e1963_d_n8);
        let eq75_e1964_d_n9: f64 = (eq75_e1959_d_n9 + eq75_e1963_d_n9);
        let eq75_e1964_d_n10: f64 = (eq75_e1959_d_n10 + eq75_e1963_d_n10);
        let eq75_e1964_d_n11: f64 = (eq75_e1959_d_n11 + eq75_e1963_d_n11);
        let eq75_e1964_d_n12: f64 = (eq75_e1959_d_n12 + eq75_e1963_d_n12);
        let eq75_e1964_d_n13: f64 = (eq75_e1959_d_n13 + eq75_e1963_d_n13);
        let eq75_e1967: f64 = (s.v[410] / s.v[157]);
        let eq75_e1967_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq75_e1967_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq75_e1967_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq75_e1967_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq75_e1967_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq75_e1967_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq75_e1967_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq75_e1967_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq75_e1967_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq75_e1967_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq75_e1967_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq75_e1967_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq75_e1967_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq75_e1967_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq75_e1968: f64 = (eq75_e1964 + eq75_e1967);
        let eq75_e1968_d_n0: f64 = (eq75_e1964_d_n0 + eq75_e1967_d_n0);
        let eq75_e1968_d_n1: f64 = (eq75_e1964_d_n1 + eq75_e1967_d_n1);
        let eq75_e1968_d_n2: f64 = (eq75_e1964_d_n2 + eq75_e1967_d_n2);
        let eq75_e1968_d_n3: f64 = (eq75_e1964_d_n3 + eq75_e1967_d_n3);
        let eq75_e1968_d_n4: f64 = (eq75_e1964_d_n4 + eq75_e1967_d_n4);
        let eq75_e1968_d_n5: f64 = (eq75_e1964_d_n5 + eq75_e1967_d_n5);
        let eq75_e1968_d_n6: f64 = (eq75_e1964_d_n6 + eq75_e1967_d_n6);
        let eq75_e1968_d_n7: f64 = (eq75_e1964_d_n7 + eq75_e1967_d_n7);
        let eq75_e1968_d_n8: f64 = (eq75_e1964_d_n8 + eq75_e1967_d_n8);
        let eq75_e1968_d_n9: f64 = (eq75_e1964_d_n9 + eq75_e1967_d_n9);
        let eq75_e1968_d_n10: f64 = (eq75_e1964_d_n10 + eq75_e1967_d_n10);
        let eq75_e1968_d_n11: f64 = (eq75_e1964_d_n11 + eq75_e1967_d_n11);
        let eq75_e1968_d_n12: f64 = (eq75_e1964_d_n12 + eq75_e1967_d_n12);
        let eq75_e1968_d_n13: f64 = (eq75_e1964_d_n13 + eq75_e1967_d_n13);
        (eq75_e1968, eq75_e1968_d_n0, eq75_e1968_d_n1, eq75_e1968_d_n2, eq75_e1968_d_n3, eq75_e1968_d_n4, eq75_e1968_d_n5, eq75_e1968_d_n6, eq75_e1968_d_n7, eq75_e1968_d_n8, eq75_e1968_d_n9, eq75_e1968_d_n10, eq75_e1968_d_n11, eq75_e1968_d_n12, eq75_e1968_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1970;
        let eq75_node_derivatives: [f64; 14] = [eq75_e1970_d_n0, eq75_e1970_d_n1, eq75_e1970_d_n2, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12, eq75_e1970_d_n13];
        let eq75_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            None,
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
        let (eq76_e1992, eq76_e1992_d_n0, eq76_e1992_d_n1, eq76_e1992_d_n2, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12, eq76_e1992_d_n13,) = {
    if (((s.v[1559] != 0.0) && (!(s.v[1560] != 0.0))) && (!(s.v[1564] != 0.0))) {
        let eq76_e1979: f64 = (-s.v[885]);
        let eq76_e1979_d_n0: f64 = (-s.dn[885][0]);
        let eq76_e1979_d_n1: f64 = (-s.dn[885][1]);
        let eq76_e1979_d_n2: f64 = (-s.dn[885][2]);
        let eq76_e1979_d_n3: f64 = (-s.dn[885][3]);
        let eq76_e1979_d_n4: f64 = (-s.dn[885][4]);
        let eq76_e1979_d_n5: f64 = (-s.dn[885][5]);
        let eq76_e1979_d_n6: f64 = (-s.dn[885][6]);
        let eq76_e1979_d_n7: f64 = (-s.dn[885][7]);
        let eq76_e1979_d_n8: f64 = (-s.dn[885][8]);
        let eq76_e1979_d_n9: f64 = (-s.dn[885][9]);
        let eq76_e1979_d_n10: f64 = (-s.dn[885][10]);
        let eq76_e1979_d_n11: f64 = (-s.dn[885][11]);
        let eq76_e1979_d_n12: f64 = (-s.dn[885][12]);
        let eq76_e1979_d_n13: f64 = (-s.dn[885][13]);
        let eq76_e1981: f64 = (eq76_e1979 * s.v[822]);
        let eq76_e1981_d_n0: f64 = ((eq76_e1979_d_n0 * s.v[822]) + (eq76_e1979 * s.dn[822][0]));
        let eq76_e1981_d_n1: f64 = ((eq76_e1979_d_n1 * s.v[822]) + (eq76_e1979 * s.dn[822][1]));
        let eq76_e1981_d_n2: f64 = ((eq76_e1979_d_n2 * s.v[822]) + (eq76_e1979 * s.dn[822][2]));
        let eq76_e1981_d_n3: f64 = ((eq76_e1979_d_n3 * s.v[822]) + (eq76_e1979 * s.dn[822][3]));
        let eq76_e1981_d_n4: f64 = ((eq76_e1979_d_n4 * s.v[822]) + (eq76_e1979 * s.dn[822][4]));
        let eq76_e1981_d_n5: f64 = ((eq76_e1979_d_n5 * s.v[822]) + (eq76_e1979 * s.dn[822][5]));
        let eq76_e1981_d_n6: f64 = ((eq76_e1979_d_n6 * s.v[822]) + (eq76_e1979 * s.dn[822][6]));
        let eq76_e1981_d_n7: f64 = ((eq76_e1979_d_n7 * s.v[822]) + (eq76_e1979 * s.dn[822][7]));
        let eq76_e1981_d_n8: f64 = ((eq76_e1979_d_n8 * s.v[822]) + (eq76_e1979 * s.dn[822][8]));
        let eq76_e1981_d_n9: f64 = ((eq76_e1979_d_n9 * s.v[822]) + (eq76_e1979 * s.dn[822][9]));
        let eq76_e1981_d_n10: f64 = ((eq76_e1979_d_n10 * s.v[822]) + (eq76_e1979 * s.dn[822][10]));
        let eq76_e1981_d_n11: f64 = ((eq76_e1979_d_n11 * s.v[822]) + (eq76_e1979 * s.dn[822][11]));
        let eq76_e1981_d_n12: f64 = ((eq76_e1979_d_n12 * s.v[822]) + (eq76_e1979 * s.dn[822][12]));
        let eq76_e1981_d_n13: f64 = ((eq76_e1979_d_n13 * s.v[822]) + (eq76_e1979 * s.dn[822][13]));
        let eq76_e1984: f64 = (s.v[410] * s.v[158]);
        let eq76_e1984_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq76_e1984_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq76_e1984_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq76_e1984_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq76_e1984_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq76_e1984_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq76_e1984_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq76_e1984_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq76_e1984_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq76_e1984_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq76_e1984_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq76_e1984_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq76_e1984_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq76_e1984_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq76_e1985: f64 = self.eval_ddt(21, eq76_e1984);
        let eq76_e1985_d_n0: f64 = self.ddt_jacobian(eq76_e1984_d_n0);
        let eq76_e1985_d_n1: f64 = self.ddt_jacobian(eq76_e1984_d_n1);
        let eq76_e1985_d_n2: f64 = self.ddt_jacobian(eq76_e1984_d_n2);
        let eq76_e1985_d_n3: f64 = self.ddt_jacobian(eq76_e1984_d_n3);
        let eq76_e1985_d_n4: f64 = self.ddt_jacobian(eq76_e1984_d_n4);
        let eq76_e1985_d_n5: f64 = self.ddt_jacobian(eq76_e1984_d_n5);
        let eq76_e1985_d_n6: f64 = self.ddt_jacobian(eq76_e1984_d_n6);
        let eq76_e1985_d_n7: f64 = self.ddt_jacobian(eq76_e1984_d_n7);
        let eq76_e1985_d_n8: f64 = self.ddt_jacobian(eq76_e1984_d_n8);
        let eq76_e1985_d_n9: f64 = self.ddt_jacobian(eq76_e1984_d_n9);
        let eq76_e1985_d_n10: f64 = self.ddt_jacobian(eq76_e1984_d_n10);
        let eq76_e1985_d_n11: f64 = self.ddt_jacobian(eq76_e1984_d_n11);
        let eq76_e1985_d_n12: f64 = self.ddt_jacobian(eq76_e1984_d_n12);
        let eq76_e1985_d_n13: f64 = self.ddt_jacobian(eq76_e1984_d_n13);
        let eq76_e1986: f64 = (eq76_e1981 + eq76_e1985);
        let eq76_e1986_d_n0: f64 = (eq76_e1981_d_n0 + eq76_e1985_d_n0);
        let eq76_e1986_d_n1: f64 = (eq76_e1981_d_n1 + eq76_e1985_d_n1);
        let eq76_e1986_d_n2: f64 = (eq76_e1981_d_n2 + eq76_e1985_d_n2);
        let eq76_e1986_d_n3: f64 = (eq76_e1981_d_n3 + eq76_e1985_d_n3);
        let eq76_e1986_d_n4: f64 = (eq76_e1981_d_n4 + eq76_e1985_d_n4);
        let eq76_e1986_d_n5: f64 = (eq76_e1981_d_n5 + eq76_e1985_d_n5);
        let eq76_e1986_d_n6: f64 = (eq76_e1981_d_n6 + eq76_e1985_d_n6);
        let eq76_e1986_d_n7: f64 = (eq76_e1981_d_n7 + eq76_e1985_d_n7);
        let eq76_e1986_d_n8: f64 = (eq76_e1981_d_n8 + eq76_e1985_d_n8);
        let eq76_e1986_d_n9: f64 = (eq76_e1981_d_n9 + eq76_e1985_d_n9);
        let eq76_e1986_d_n10: f64 = (eq76_e1981_d_n10 + eq76_e1985_d_n10);
        let eq76_e1986_d_n11: f64 = (eq76_e1981_d_n11 + eq76_e1985_d_n11);
        let eq76_e1986_d_n12: f64 = (eq76_e1981_d_n12 + eq76_e1985_d_n12);
        let eq76_e1986_d_n13: f64 = (eq76_e1981_d_n13 + eq76_e1985_d_n13);
        let eq76_e1989: f64 = (s.v[410] / s.v[157]);
        let eq76_e1989_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq76_e1989_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq76_e1989_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq76_e1989_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq76_e1989_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq76_e1989_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq76_e1989_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq76_e1989_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq76_e1989_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq76_e1989_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq76_e1989_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq76_e1989_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq76_e1989_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq76_e1989_d_n13: f64 = (s.dn[410][13] / s.v[157]);
        let eq76_e1990: f64 = (eq76_e1986 + eq76_e1989);
        let eq76_e1990_d_n0: f64 = (eq76_e1986_d_n0 + eq76_e1989_d_n0);
        let eq76_e1990_d_n1: f64 = (eq76_e1986_d_n1 + eq76_e1989_d_n1);
        let eq76_e1990_d_n2: f64 = (eq76_e1986_d_n2 + eq76_e1989_d_n2);
        let eq76_e1990_d_n3: f64 = (eq76_e1986_d_n3 + eq76_e1989_d_n3);
        let eq76_e1990_d_n4: f64 = (eq76_e1986_d_n4 + eq76_e1989_d_n4);
        let eq76_e1990_d_n5: f64 = (eq76_e1986_d_n5 + eq76_e1989_d_n5);
        let eq76_e1990_d_n6: f64 = (eq76_e1986_d_n6 + eq76_e1989_d_n6);
        let eq76_e1990_d_n7: f64 = (eq76_e1986_d_n7 + eq76_e1989_d_n7);
        let eq76_e1990_d_n8: f64 = (eq76_e1986_d_n8 + eq76_e1989_d_n8);
        let eq76_e1990_d_n9: f64 = (eq76_e1986_d_n9 + eq76_e1989_d_n9);
        let eq76_e1990_d_n10: f64 = (eq76_e1986_d_n10 + eq76_e1989_d_n10);
        let eq76_e1990_d_n11: f64 = (eq76_e1986_d_n11 + eq76_e1989_d_n11);
        let eq76_e1990_d_n12: f64 = (eq76_e1986_d_n12 + eq76_e1989_d_n12);
        let eq76_e1990_d_n13: f64 = (eq76_e1986_d_n13 + eq76_e1989_d_n13);
        (eq76_e1990, eq76_e1990_d_n0, eq76_e1990_d_n1, eq76_e1990_d_n2, eq76_e1990_d_n3, eq76_e1990_d_n4, eq76_e1990_d_n5, eq76_e1990_d_n6, eq76_e1990_d_n7, eq76_e1990_d_n8, eq76_e1990_d_n9, eq76_e1990_d_n10, eq76_e1990_d_n11, eq76_e1990_d_n12, eq76_e1990_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1992;
        let eq76_node_derivatives: [f64; 14] = [eq76_e1992_d_n0, eq76_e1992_d_n1, eq76_e1992_d_n2, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12, eq76_e1992_d_n13];
        let eq76_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            None,
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
        let (eq77_e2001,) = {
    if (((!(s.v[1559] != 0.0)) && (s.v[1565] != 0.0)) && (s.v[1566] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq77_value: f64 = eq77_e2001;
        stamper.stamp_potential(
            branches[15],
            eq77_value,
            &[
            ],
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
        let (eq78_e2013,) = {
    if ((((!(s.v[1559] != 0.0)) && (s.v[1565] != 0.0)) && (!(s.v[1566] != 0.0))) && (s.v[1567] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq78_value: f64 = eq78_e2013;
        stamper.stamp_potential(
            branches[16],
            eq78_value,
            &[
            ],
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
        let (eq79_e2026,) = {
    if ((((!(s.v[1559] != 0.0)) && (s.v[1565] != 0.0)) && (!(s.v[1566] != 0.0))) && (!(s.v[1567] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq79_value: f64 = eq79_e2026;
        stamper.stamp_potential(
            branches[17],
            eq79_value,
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
        let (eq80_e2034,) = {
    if ((!(s.v[1559] != 0.0)) && (!(s.v[1565] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e2034;
        stamper.stamp_potential(
            branches[18],
            eq80_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq14_e1376, eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13, eq14_e1376_q, eq14_e1376_q_d_n0, eq14_e1376_q_d_n1, eq14_e1376_q_d_n2, eq14_e1376_q_d_n3, eq14_e1376_q_d_n4, eq14_e1376_q_d_n5, eq14_e1376_q_d_n6, eq14_e1376_q_d_n7, eq14_e1376_q_d_n8, eq14_e1376_q_d_n9, eq14_e1376_q_d_n10, eq14_e1376_q_d_n11, eq14_e1376_q_d_n12, eq14_e1376_q_d_n13,) = {
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
        let eq14_e1374_q: f64 = eq14_e1373;
        (eq14_e1373, eq14_e1373_d_n0, eq14_e1373_d_n1, eq14_e1373_d_n2, eq14_e1373_d_n3, eq14_e1373_d_n4, eq14_e1373_d_n5, eq14_e1373_d_n6, eq14_e1373_d_n7, eq14_e1373_d_n8, eq14_e1373_d_n9, eq14_e1373_d_n10, eq14_e1373_d_n11, eq14_e1373_d_n12, eq14_e1373_d_n13, eq14_e1374_q, eq14_e1373_d_n0, eq14_e1373_d_n1, eq14_e1373_d_n2, eq14_e1373_d_n3, eq14_e1373_d_n4, eq14_e1373_d_n5, eq14_e1373_d_n6, eq14_e1373_d_n7, eq14_e1373_d_n8, eq14_e1373_d_n9, eq14_e1373_d_n10, eq14_e1373_d_n11, eq14_e1373_d_n12, eq14_e1373_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_reactive_node_derivatives: [f64; 14] = [eq14_e1376_q_d_n0, eq14_e1376_q_d_n1, eq14_e1376_q_d_n2, eq14_e1376_q_d_n3, eq14_e1376_q_d_n4, eq14_e1376_q_d_n5, eq14_e1376_q_d_n6, eq14_e1376_q_d_n7, eq14_e1376_q_d_n8, eq14_e1376_q_d_n9, eq14_e1376_q_d_n10, eq14_e1376_q_d_n11, eq14_e1376_q_d_n12, eq14_e1376_q_d_n13];
        let eq14_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            &nodes,
            &eq14_reactive_node_derivatives,
            &branches,
            &eq14_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq15_e1396, eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13, eq15_e1396_q, eq15_e1396_q_d_n0, eq15_e1396_q_d_n1, eq15_e1396_q_d_n2, eq15_e1396_q_d_n3, eq15_e1396_q_d_n4, eq15_e1396_q_d_n5, eq15_e1396_q_d_n6, eq15_e1396_q_d_n7, eq15_e1396_q_d_n8, eq15_e1396_q_d_n9, eq15_e1396_q_d_n10, eq15_e1396_q_d_n11, eq15_e1396_q_d_n12, eq15_e1396_q_d_n13,) = {
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
        let eq15_e1394_q: f64 = eq15_e1393;
        (eq15_e1393, eq15_e1393_d_n0, eq15_e1393_d_n1, eq15_e1393_d_n2, eq15_e1393_d_n3, eq15_e1393_d_n4, eq15_e1393_d_n5, eq15_e1393_d_n6, eq15_e1393_d_n7, eq15_e1393_d_n8, eq15_e1393_d_n9, eq15_e1393_d_n10, eq15_e1393_d_n11, eq15_e1393_d_n12, eq15_e1393_d_n13, eq15_e1394_q, eq15_e1393_d_n0, eq15_e1393_d_n1, eq15_e1393_d_n2, eq15_e1393_d_n3, eq15_e1393_d_n4, eq15_e1393_d_n5, eq15_e1393_d_n6, eq15_e1393_d_n7, eq15_e1393_d_n8, eq15_e1393_d_n9, eq15_e1393_d_n10, eq15_e1393_d_n11, eq15_e1393_d_n12, eq15_e1393_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_reactive_node_derivatives: [f64; 14] = [eq15_e1396_q_d_n0, eq15_e1396_q_d_n1, eq15_e1396_q_d_n2, eq15_e1396_q_d_n3, eq15_e1396_q_d_n4, eq15_e1396_q_d_n5, eq15_e1396_q_d_n6, eq15_e1396_q_d_n7, eq15_e1396_q_d_n8, eq15_e1396_q_d_n9, eq15_e1396_q_d_n10, eq15_e1396_q_d_n11, eq15_e1396_q_d_n12, eq15_e1396_q_d_n13];
        let eq15_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq15_reactive_node_derivatives,
            &branches,
            &eq15_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_44_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq44_e1647: f64 = (p.p33 * s.v[92]);
        let eq44_e1647_d_n0: f64 = (p.p33 * s.dn[92][0]);
        let eq44_e1647_d_n1: f64 = (p.p33 * s.dn[92][1]);
        let eq44_e1647_d_n2: f64 = (p.p33 * s.dn[92][2]);
        let eq44_e1647_d_n3: f64 = (p.p33 * s.dn[92][3]);
        let eq44_e1647_d_n4: f64 = (p.p33 * s.dn[92][4]);
        let eq44_e1647_d_n5: f64 = (p.p33 * s.dn[92][5]);
        let eq44_e1647_d_n6: f64 = (p.p33 * s.dn[92][6]);
        let eq44_e1647_d_n7: f64 = (p.p33 * s.dn[92][7]);
        let eq44_e1647_d_n8: f64 = (p.p33 * s.dn[92][8]);
        let eq44_e1647_d_n9: f64 = (p.p33 * s.dn[92][9]);
        let eq44_e1647_d_n10: f64 = (p.p33 * s.dn[92][10]);
        let eq44_e1647_d_n11: f64 = (p.p33 * s.dn[92][11]);
        let eq44_e1647_d_n12: f64 = (p.p33 * s.dn[92][12]);
        let eq44_e1647_d_n13: f64 = (p.p33 * s.dn[92][13]);
        let eq44_e1648_q: f64 = eq44_e1647;
        let eq44_reactive_node_derivatives: [f64; 14] = [eq44_e1647_d_n0, eq44_e1647_d_n1, eq44_e1647_d_n2, eq44_e1647_d_n3, eq44_e1647_d_n4, eq44_e1647_d_n5, eq44_e1647_d_n6, eq44_e1647_d_n7, eq44_e1647_d_n8, eq44_e1647_d_n9, eq44_e1647_d_n10, eq44_e1647_d_n11, eq44_e1647_d_n12, eq44_e1647_d_n13];
        let eq44_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &nodes,
            &eq44_reactive_node_derivatives,
            &branches,
            &eq44_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_45_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq45_e1651: f64 = (p.p33 * s.v[93]);
        let eq45_e1651_d_n0: f64 = (p.p33 * s.dn[93][0]);
        let eq45_e1651_d_n1: f64 = (p.p33 * s.dn[93][1]);
        let eq45_e1651_d_n2: f64 = (p.p33 * s.dn[93][2]);
        let eq45_e1651_d_n3: f64 = (p.p33 * s.dn[93][3]);
        let eq45_e1651_d_n4: f64 = (p.p33 * s.dn[93][4]);
        let eq45_e1651_d_n5: f64 = (p.p33 * s.dn[93][5]);
        let eq45_e1651_d_n6: f64 = (p.p33 * s.dn[93][6]);
        let eq45_e1651_d_n7: f64 = (p.p33 * s.dn[93][7]);
        let eq45_e1651_d_n8: f64 = (p.p33 * s.dn[93][8]);
        let eq45_e1651_d_n9: f64 = (p.p33 * s.dn[93][9]);
        let eq45_e1651_d_n10: f64 = (p.p33 * s.dn[93][10]);
        let eq45_e1651_d_n11: f64 = (p.p33 * s.dn[93][11]);
        let eq45_e1651_d_n12: f64 = (p.p33 * s.dn[93][12]);
        let eq45_e1651_d_n13: f64 = (p.p33 * s.dn[93][13]);
        let eq45_e1652_q: f64 = eq45_e1651;
        let eq45_reactive_node_derivatives: [f64; 14] = [eq45_e1651_d_n0, eq45_e1651_d_n1, eq45_e1651_d_n2, eq45_e1651_d_n3, eq45_e1651_d_n4, eq45_e1651_d_n5, eq45_e1651_d_n6, eq45_e1651_d_n7, eq45_e1651_d_n8, eq45_e1651_d_n9, eq45_e1651_d_n10, eq45_e1651_d_n11, eq45_e1651_d_n12, eq45_e1651_d_n13];
        let eq45_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            &nodes,
            &eq45_reactive_node_derivatives,
            &branches,
            &eq45_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq46_e1656: f64 = (p.p33 * s.v[916]);
        let eq46_e1656_d_n0: f64 = (p.p33 * s.dn[916][0]);
        let eq46_e1656_d_n1: f64 = (p.p33 * s.dn[916][1]);
        let eq46_e1656_d_n2: f64 = (p.p33 * s.dn[916][2]);
        let eq46_e1656_d_n3: f64 = (p.p33 * s.dn[916][3]);
        let eq46_e1656_d_n4: f64 = (p.p33 * s.dn[916][4]);
        let eq46_e1656_d_n5: f64 = (p.p33 * s.dn[916][5]);
        let eq46_e1656_d_n6: f64 = (p.p33 * s.dn[916][6]);
        let eq46_e1656_d_n7: f64 = (p.p33 * s.dn[916][7]);
        let eq46_e1656_d_n8: f64 = (p.p33 * s.dn[916][8]);
        let eq46_e1656_d_n9: f64 = (p.p33 * s.dn[916][9]);
        let eq46_e1656_d_n10: f64 = (p.p33 * s.dn[916][10]);
        let eq46_e1656_d_n11: f64 = (p.p33 * s.dn[916][11]);
        let eq46_e1656_d_n12: f64 = (p.p33 * s.dn[916][12]);
        let eq46_e1656_d_n13: f64 = (p.p33 * s.dn[916][13]);
        let eq46_e1657_q: f64 = eq46_e1656;
        let eq46_e1658: f64 = (p.p37 * eq46_e1656);
        let eq46_e1658_d_n0: f64 = (p.p37 * eq46_e1656_d_n0);
        let eq46_e1658_d_n1: f64 = (p.p37 * eq46_e1656_d_n1);
        let eq46_e1658_d_n2: f64 = (p.p37 * eq46_e1656_d_n2);
        let eq46_e1658_d_n3: f64 = (p.p37 * eq46_e1656_d_n3);
        let eq46_e1658_d_n4: f64 = (p.p37 * eq46_e1656_d_n4);
        let eq46_e1658_d_n5: f64 = (p.p37 * eq46_e1656_d_n5);
        let eq46_e1658_d_n6: f64 = (p.p37 * eq46_e1656_d_n6);
        let eq46_e1658_d_n7: f64 = (p.p37 * eq46_e1656_d_n7);
        let eq46_e1658_d_n8: f64 = (p.p37 * eq46_e1656_d_n8);
        let eq46_e1658_d_n9: f64 = (p.p37 * eq46_e1656_d_n9);
        let eq46_e1658_d_n10: f64 = (p.p37 * eq46_e1656_d_n10);
        let eq46_e1658_d_n11: f64 = (p.p37 * eq46_e1656_d_n11);
        let eq46_e1658_d_n12: f64 = (p.p37 * eq46_e1656_d_n12);
        let eq46_e1658_d_n13: f64 = (p.p37 * eq46_e1656_d_n13);
        let eq46_e1658_q: f64 = (p.p37 * eq46_e1657_q);
        let eq46_e1658_q_d_n0: f64 = (p.p37 * eq46_e1656_d_n0);
        let eq46_e1658_q_d_n1: f64 = (p.p37 * eq46_e1656_d_n1);
        let eq46_e1658_q_d_n2: f64 = (p.p37 * eq46_e1656_d_n2);
        let eq46_e1658_q_d_n3: f64 = (p.p37 * eq46_e1656_d_n3);
        let eq46_e1658_q_d_n4: f64 = (p.p37 * eq46_e1656_d_n4);
        let eq46_e1658_q_d_n5: f64 = (p.p37 * eq46_e1656_d_n5);
        let eq46_e1658_q_d_n6: f64 = (p.p37 * eq46_e1656_d_n6);
        let eq46_e1658_q_d_n7: f64 = (p.p37 * eq46_e1656_d_n7);
        let eq46_e1658_q_d_n8: f64 = (p.p37 * eq46_e1656_d_n8);
        let eq46_e1658_q_d_n9: f64 = (p.p37 * eq46_e1656_d_n9);
        let eq46_e1658_q_d_n10: f64 = (p.p37 * eq46_e1656_d_n10);
        let eq46_e1658_q_d_n11: f64 = (p.p37 * eq46_e1656_d_n11);
        let eq46_e1658_q_d_n12: f64 = (p.p37 * eq46_e1656_d_n12);
        let eq46_e1658_q_d_n13: f64 = (p.p37 * eq46_e1656_d_n13);
        let eq46_reactive_node_derivatives: [f64; 14] = [eq46_e1658_q_d_n0, eq46_e1658_q_d_n1, eq46_e1658_q_d_n2, eq46_e1658_q_d_n3, eq46_e1658_q_d_n4, eq46_e1658_q_d_n5, eq46_e1658_q_d_n6, eq46_e1658_q_d_n7, eq46_e1658_q_d_n8, eq46_e1658_q_d_n9, eq46_e1658_q_d_n10, eq46_e1658_q_d_n11, eq46_e1658_q_d_n12, eq46_e1658_q_d_n13];
        let eq46_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            &nodes,
            &eq46_reactive_node_derivatives,
            &branches,
            &eq46_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_47_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq47_e1662: f64 = (p.p33 * s.v[920]);
        let eq47_e1662_d_n0: f64 = (p.p33 * s.dn[920][0]);
        let eq47_e1662_d_n1: f64 = (p.p33 * s.dn[920][1]);
        let eq47_e1662_d_n2: f64 = (p.p33 * s.dn[920][2]);
        let eq47_e1662_d_n3: f64 = (p.p33 * s.dn[920][3]);
        let eq47_e1662_d_n4: f64 = (p.p33 * s.dn[920][4]);
        let eq47_e1662_d_n5: f64 = (p.p33 * s.dn[920][5]);
        let eq47_e1662_d_n6: f64 = (p.p33 * s.dn[920][6]);
        let eq47_e1662_d_n7: f64 = (p.p33 * s.dn[920][7]);
        let eq47_e1662_d_n8: f64 = (p.p33 * s.dn[920][8]);
        let eq47_e1662_d_n9: f64 = (p.p33 * s.dn[920][9]);
        let eq47_e1662_d_n10: f64 = (p.p33 * s.dn[920][10]);
        let eq47_e1662_d_n11: f64 = (p.p33 * s.dn[920][11]);
        let eq47_e1662_d_n12: f64 = (p.p33 * s.dn[920][12]);
        let eq47_e1662_d_n13: f64 = (p.p33 * s.dn[920][13]);
        let eq47_e1663_q: f64 = eq47_e1662;
        let eq47_e1664: f64 = (p.p37 * eq47_e1662);
        let eq47_e1664_d_n0: f64 = (p.p37 * eq47_e1662_d_n0);
        let eq47_e1664_d_n1: f64 = (p.p37 * eq47_e1662_d_n1);
        let eq47_e1664_d_n2: f64 = (p.p37 * eq47_e1662_d_n2);
        let eq47_e1664_d_n3: f64 = (p.p37 * eq47_e1662_d_n3);
        let eq47_e1664_d_n4: f64 = (p.p37 * eq47_e1662_d_n4);
        let eq47_e1664_d_n5: f64 = (p.p37 * eq47_e1662_d_n5);
        let eq47_e1664_d_n6: f64 = (p.p37 * eq47_e1662_d_n6);
        let eq47_e1664_d_n7: f64 = (p.p37 * eq47_e1662_d_n7);
        let eq47_e1664_d_n8: f64 = (p.p37 * eq47_e1662_d_n8);
        let eq47_e1664_d_n9: f64 = (p.p37 * eq47_e1662_d_n9);
        let eq47_e1664_d_n10: f64 = (p.p37 * eq47_e1662_d_n10);
        let eq47_e1664_d_n11: f64 = (p.p37 * eq47_e1662_d_n11);
        let eq47_e1664_d_n12: f64 = (p.p37 * eq47_e1662_d_n12);
        let eq47_e1664_d_n13: f64 = (p.p37 * eq47_e1662_d_n13);
        let eq47_e1664_q: f64 = (p.p37 * eq47_e1663_q);
        let eq47_e1664_q_d_n0: f64 = (p.p37 * eq47_e1662_d_n0);
        let eq47_e1664_q_d_n1: f64 = (p.p37 * eq47_e1662_d_n1);
        let eq47_e1664_q_d_n2: f64 = (p.p37 * eq47_e1662_d_n2);
        let eq47_e1664_q_d_n3: f64 = (p.p37 * eq47_e1662_d_n3);
        let eq47_e1664_q_d_n4: f64 = (p.p37 * eq47_e1662_d_n4);
        let eq47_e1664_q_d_n5: f64 = (p.p37 * eq47_e1662_d_n5);
        let eq47_e1664_q_d_n6: f64 = (p.p37 * eq47_e1662_d_n6);
        let eq47_e1664_q_d_n7: f64 = (p.p37 * eq47_e1662_d_n7);
        let eq47_e1664_q_d_n8: f64 = (p.p37 * eq47_e1662_d_n8);
        let eq47_e1664_q_d_n9: f64 = (p.p37 * eq47_e1662_d_n9);
        let eq47_e1664_q_d_n10: f64 = (p.p37 * eq47_e1662_d_n10);
        let eq47_e1664_q_d_n11: f64 = (p.p37 * eq47_e1662_d_n11);
        let eq47_e1664_q_d_n12: f64 = (p.p37 * eq47_e1662_d_n12);
        let eq47_e1664_q_d_n13: f64 = (p.p37 * eq47_e1662_d_n13);
        let eq47_reactive_node_derivatives: [f64; 14] = [eq47_e1664_q_d_n0, eq47_e1664_q_d_n1, eq47_e1664_q_d_n2, eq47_e1664_q_d_n3, eq47_e1664_q_d_n4, eq47_e1664_q_d_n5, eq47_e1664_q_d_n6, eq47_e1664_q_d_n7, eq47_e1664_q_d_n8, eq47_e1664_q_d_n9, eq47_e1664_q_d_n10, eq47_e1664_q_d_n11, eq47_e1664_q_d_n12, eq47_e1664_q_d_n13];
        let eq47_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            &nodes,
            &eq47_reactive_node_derivatives,
            &branches,
            &eq47_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_48_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq48_e1668: f64 = (p.p33 * s.v[909]);
        let eq48_e1668_d_n0: f64 = (p.p33 * s.dn[909][0]);
        let eq48_e1668_d_n1: f64 = (p.p33 * s.dn[909][1]);
        let eq48_e1668_d_n2: f64 = (p.p33 * s.dn[909][2]);
        let eq48_e1668_d_n3: f64 = (p.p33 * s.dn[909][3]);
        let eq48_e1668_d_n4: f64 = (p.p33 * s.dn[909][4]);
        let eq48_e1668_d_n5: f64 = (p.p33 * s.dn[909][5]);
        let eq48_e1668_d_n6: f64 = (p.p33 * s.dn[909][6]);
        let eq48_e1668_d_n7: f64 = (p.p33 * s.dn[909][7]);
        let eq48_e1668_d_n8: f64 = (p.p33 * s.dn[909][8]);
        let eq48_e1668_d_n9: f64 = (p.p33 * s.dn[909][9]);
        let eq48_e1668_d_n10: f64 = (p.p33 * s.dn[909][10]);
        let eq48_e1668_d_n11: f64 = (p.p33 * s.dn[909][11]);
        let eq48_e1668_d_n12: f64 = (p.p33 * s.dn[909][12]);
        let eq48_e1668_d_n13: f64 = (p.p33 * s.dn[909][13]);
        let eq48_e1669_q: f64 = eq48_e1668;
        let eq48_e1670: f64 = (p.p37 * eq48_e1668);
        let eq48_e1670_d_n0: f64 = (p.p37 * eq48_e1668_d_n0);
        let eq48_e1670_d_n1: f64 = (p.p37 * eq48_e1668_d_n1);
        let eq48_e1670_d_n2: f64 = (p.p37 * eq48_e1668_d_n2);
        let eq48_e1670_d_n3: f64 = (p.p37 * eq48_e1668_d_n3);
        let eq48_e1670_d_n4: f64 = (p.p37 * eq48_e1668_d_n4);
        let eq48_e1670_d_n5: f64 = (p.p37 * eq48_e1668_d_n5);
        let eq48_e1670_d_n6: f64 = (p.p37 * eq48_e1668_d_n6);
        let eq48_e1670_d_n7: f64 = (p.p37 * eq48_e1668_d_n7);
        let eq48_e1670_d_n8: f64 = (p.p37 * eq48_e1668_d_n8);
        let eq48_e1670_d_n9: f64 = (p.p37 * eq48_e1668_d_n9);
        let eq48_e1670_d_n10: f64 = (p.p37 * eq48_e1668_d_n10);
        let eq48_e1670_d_n11: f64 = (p.p37 * eq48_e1668_d_n11);
        let eq48_e1670_d_n12: f64 = (p.p37 * eq48_e1668_d_n12);
        let eq48_e1670_d_n13: f64 = (p.p37 * eq48_e1668_d_n13);
        let eq48_e1670_q: f64 = (p.p37 * eq48_e1669_q);
        let eq48_e1670_q_d_n0: f64 = (p.p37 * eq48_e1668_d_n0);
        let eq48_e1670_q_d_n1: f64 = (p.p37 * eq48_e1668_d_n1);
        let eq48_e1670_q_d_n2: f64 = (p.p37 * eq48_e1668_d_n2);
        let eq48_e1670_q_d_n3: f64 = (p.p37 * eq48_e1668_d_n3);
        let eq48_e1670_q_d_n4: f64 = (p.p37 * eq48_e1668_d_n4);
        let eq48_e1670_q_d_n5: f64 = (p.p37 * eq48_e1668_d_n5);
        let eq48_e1670_q_d_n6: f64 = (p.p37 * eq48_e1668_d_n6);
        let eq48_e1670_q_d_n7: f64 = (p.p37 * eq48_e1668_d_n7);
        let eq48_e1670_q_d_n8: f64 = (p.p37 * eq48_e1668_d_n8);
        let eq48_e1670_q_d_n9: f64 = (p.p37 * eq48_e1668_d_n9);
        let eq48_e1670_q_d_n10: f64 = (p.p37 * eq48_e1668_d_n10);
        let eq48_e1670_q_d_n11: f64 = (p.p37 * eq48_e1668_d_n11);
        let eq48_e1670_q_d_n12: f64 = (p.p37 * eq48_e1668_d_n12);
        let eq48_e1670_q_d_n13: f64 = (p.p37 * eq48_e1668_d_n13);
        let eq48_reactive_node_derivatives: [f64; 14] = [eq48_e1670_q_d_n0, eq48_e1670_q_d_n1, eq48_e1670_q_d_n2, eq48_e1670_q_d_n3, eq48_e1670_q_d_n4, eq48_e1670_q_d_n5, eq48_e1670_q_d_n6, eq48_e1670_q_d_n7, eq48_e1670_q_d_n8, eq48_e1670_q_d_n9, eq48_e1670_q_d_n10, eq48_e1670_q_d_n11, eq48_e1670_q_d_n12, eq48_e1670_q_d_n13];
        let eq48_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            &nodes,
            &eq48_reactive_node_derivatives,
            &branches,
            &eq48_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_49_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq49_e1674: f64 = (p.p33 * s.v[910]);
        let eq49_e1674_d_n0: f64 = (p.p33 * s.dn[910][0]);
        let eq49_e1674_d_n1: f64 = (p.p33 * s.dn[910][1]);
        let eq49_e1674_d_n2: f64 = (p.p33 * s.dn[910][2]);
        let eq49_e1674_d_n3: f64 = (p.p33 * s.dn[910][3]);
        let eq49_e1674_d_n4: f64 = (p.p33 * s.dn[910][4]);
        let eq49_e1674_d_n5: f64 = (p.p33 * s.dn[910][5]);
        let eq49_e1674_d_n6: f64 = (p.p33 * s.dn[910][6]);
        let eq49_e1674_d_n7: f64 = (p.p33 * s.dn[910][7]);
        let eq49_e1674_d_n8: f64 = (p.p33 * s.dn[910][8]);
        let eq49_e1674_d_n9: f64 = (p.p33 * s.dn[910][9]);
        let eq49_e1674_d_n10: f64 = (p.p33 * s.dn[910][10]);
        let eq49_e1674_d_n11: f64 = (p.p33 * s.dn[910][11]);
        let eq49_e1674_d_n12: f64 = (p.p33 * s.dn[910][12]);
        let eq49_e1674_d_n13: f64 = (p.p33 * s.dn[910][13]);
        let eq49_e1675_q: f64 = eq49_e1674;
        let eq49_e1676: f64 = (p.p37 * eq49_e1674);
        let eq49_e1676_d_n0: f64 = (p.p37 * eq49_e1674_d_n0);
        let eq49_e1676_d_n1: f64 = (p.p37 * eq49_e1674_d_n1);
        let eq49_e1676_d_n2: f64 = (p.p37 * eq49_e1674_d_n2);
        let eq49_e1676_d_n3: f64 = (p.p37 * eq49_e1674_d_n3);
        let eq49_e1676_d_n4: f64 = (p.p37 * eq49_e1674_d_n4);
        let eq49_e1676_d_n5: f64 = (p.p37 * eq49_e1674_d_n5);
        let eq49_e1676_d_n6: f64 = (p.p37 * eq49_e1674_d_n6);
        let eq49_e1676_d_n7: f64 = (p.p37 * eq49_e1674_d_n7);
        let eq49_e1676_d_n8: f64 = (p.p37 * eq49_e1674_d_n8);
        let eq49_e1676_d_n9: f64 = (p.p37 * eq49_e1674_d_n9);
        let eq49_e1676_d_n10: f64 = (p.p37 * eq49_e1674_d_n10);
        let eq49_e1676_d_n11: f64 = (p.p37 * eq49_e1674_d_n11);
        let eq49_e1676_d_n12: f64 = (p.p37 * eq49_e1674_d_n12);
        let eq49_e1676_d_n13: f64 = (p.p37 * eq49_e1674_d_n13);
        let eq49_e1676_q: f64 = (p.p37 * eq49_e1675_q);
        let eq49_e1676_q_d_n0: f64 = (p.p37 * eq49_e1674_d_n0);
        let eq49_e1676_q_d_n1: f64 = (p.p37 * eq49_e1674_d_n1);
        let eq49_e1676_q_d_n2: f64 = (p.p37 * eq49_e1674_d_n2);
        let eq49_e1676_q_d_n3: f64 = (p.p37 * eq49_e1674_d_n3);
        let eq49_e1676_q_d_n4: f64 = (p.p37 * eq49_e1674_d_n4);
        let eq49_e1676_q_d_n5: f64 = (p.p37 * eq49_e1674_d_n5);
        let eq49_e1676_q_d_n6: f64 = (p.p37 * eq49_e1674_d_n6);
        let eq49_e1676_q_d_n7: f64 = (p.p37 * eq49_e1674_d_n7);
        let eq49_e1676_q_d_n8: f64 = (p.p37 * eq49_e1674_d_n8);
        let eq49_e1676_q_d_n9: f64 = (p.p37 * eq49_e1674_d_n9);
        let eq49_e1676_q_d_n10: f64 = (p.p37 * eq49_e1674_d_n10);
        let eq49_e1676_q_d_n11: f64 = (p.p37 * eq49_e1674_d_n11);
        let eq49_e1676_q_d_n12: f64 = (p.p37 * eq49_e1674_d_n12);
        let eq49_e1676_q_d_n13: f64 = (p.p37 * eq49_e1674_d_n13);
        let eq49_reactive_node_derivatives: [f64; 14] = [eq49_e1676_q_d_n0, eq49_e1676_q_d_n1, eq49_e1676_q_d_n2, eq49_e1676_q_d_n3, eq49_e1676_q_d_n4, eq49_e1676_q_d_n5, eq49_e1676_q_d_n6, eq49_e1676_q_d_n7, eq49_e1676_q_d_n8, eq49_e1676_q_d_n9, eq49_e1676_q_d_n10, eq49_e1676_q_d_n11, eq49_e1676_q_d_n12, eq49_e1676_q_d_n13];
        let eq49_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            &nodes,
            &eq49_reactive_node_derivatives,
            &branches,
            &eq49_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_50_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq50_e1685, eq50_e1685_d_n0, eq50_e1685_d_n1, eq50_e1685_d_n2, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, eq50_e1685_d_n13, eq50_e1685_q, eq50_e1685_q_d_n0, eq50_e1685_q_d_n1, eq50_e1685_q_d_n2, eq50_e1685_q_d_n3, eq50_e1685_q_d_n4, eq50_e1685_q_d_n5, eq50_e1685_q_d_n6, eq50_e1685_q_d_n7, eq50_e1685_q_d_n8, eq50_e1685_q_d_n9, eq50_e1685_q_d_n10, eq50_e1685_q_d_n11, eq50_e1685_q_d_n12, eq50_e1685_q_d_n13,) = {
    if (s.v[1553] != 0.0) {
        let eq50_e1681: f64 = (p.p33 * s.v[895]);
        let eq50_e1681_d_n0: f64 = (p.p33 * s.dn[895][0]);
        let eq50_e1681_d_n1: f64 = (p.p33 * s.dn[895][1]);
        let eq50_e1681_d_n2: f64 = (p.p33 * s.dn[895][2]);
        let eq50_e1681_d_n3: f64 = (p.p33 * s.dn[895][3]);
        let eq50_e1681_d_n4: f64 = (p.p33 * s.dn[895][4]);
        let eq50_e1681_d_n5: f64 = (p.p33 * s.dn[895][5]);
        let eq50_e1681_d_n6: f64 = (p.p33 * s.dn[895][6]);
        let eq50_e1681_d_n7: f64 = (p.p33 * s.dn[895][7]);
        let eq50_e1681_d_n8: f64 = (p.p33 * s.dn[895][8]);
        let eq50_e1681_d_n9: f64 = (p.p33 * s.dn[895][9]);
        let eq50_e1681_d_n10: f64 = (p.p33 * s.dn[895][10]);
        let eq50_e1681_d_n11: f64 = (p.p33 * s.dn[895][11]);
        let eq50_e1681_d_n12: f64 = (p.p33 * s.dn[895][12]);
        let eq50_e1681_d_n13: f64 = (p.p33 * s.dn[895][13]);
        let eq50_e1682_q: f64 = eq50_e1681;
        let eq50_e1683: f64 = (p.p37 * eq50_e1681);
        let eq50_e1683_d_n0: f64 = (p.p37 * eq50_e1681_d_n0);
        let eq50_e1683_d_n1: f64 = (p.p37 * eq50_e1681_d_n1);
        let eq50_e1683_d_n2: f64 = (p.p37 * eq50_e1681_d_n2);
        let eq50_e1683_d_n3: f64 = (p.p37 * eq50_e1681_d_n3);
        let eq50_e1683_d_n4: f64 = (p.p37 * eq50_e1681_d_n4);
        let eq50_e1683_d_n5: f64 = (p.p37 * eq50_e1681_d_n5);
        let eq50_e1683_d_n6: f64 = (p.p37 * eq50_e1681_d_n6);
        let eq50_e1683_d_n7: f64 = (p.p37 * eq50_e1681_d_n7);
        let eq50_e1683_d_n8: f64 = (p.p37 * eq50_e1681_d_n8);
        let eq50_e1683_d_n9: f64 = (p.p37 * eq50_e1681_d_n9);
        let eq50_e1683_d_n10: f64 = (p.p37 * eq50_e1681_d_n10);
        let eq50_e1683_d_n11: f64 = (p.p37 * eq50_e1681_d_n11);
        let eq50_e1683_d_n12: f64 = (p.p37 * eq50_e1681_d_n12);
        let eq50_e1683_d_n13: f64 = (p.p37 * eq50_e1681_d_n13);
        let eq50_e1683_q: f64 = (p.p37 * eq50_e1682_q);
        let eq50_e1683_q_d_n0: f64 = (p.p37 * eq50_e1681_d_n0);
        let eq50_e1683_q_d_n1: f64 = (p.p37 * eq50_e1681_d_n1);
        let eq50_e1683_q_d_n2: f64 = (p.p37 * eq50_e1681_d_n2);
        let eq50_e1683_q_d_n3: f64 = (p.p37 * eq50_e1681_d_n3);
        let eq50_e1683_q_d_n4: f64 = (p.p37 * eq50_e1681_d_n4);
        let eq50_e1683_q_d_n5: f64 = (p.p37 * eq50_e1681_d_n5);
        let eq50_e1683_q_d_n6: f64 = (p.p37 * eq50_e1681_d_n6);
        let eq50_e1683_q_d_n7: f64 = (p.p37 * eq50_e1681_d_n7);
        let eq50_e1683_q_d_n8: f64 = (p.p37 * eq50_e1681_d_n8);
        let eq50_e1683_q_d_n9: f64 = (p.p37 * eq50_e1681_d_n9);
        let eq50_e1683_q_d_n10: f64 = (p.p37 * eq50_e1681_d_n10);
        let eq50_e1683_q_d_n11: f64 = (p.p37 * eq50_e1681_d_n11);
        let eq50_e1683_q_d_n12: f64 = (p.p37 * eq50_e1681_d_n12);
        let eq50_e1683_q_d_n13: f64 = (p.p37 * eq50_e1681_d_n13);
        (eq50_e1683, eq50_e1683_d_n0, eq50_e1683_d_n1, eq50_e1683_d_n2, eq50_e1683_d_n3, eq50_e1683_d_n4, eq50_e1683_d_n5, eq50_e1683_d_n6, eq50_e1683_d_n7, eq50_e1683_d_n8, eq50_e1683_d_n9, eq50_e1683_d_n10, eq50_e1683_d_n11, eq50_e1683_d_n12, eq50_e1683_d_n13, eq50_e1683_q, eq50_e1683_q_d_n0, eq50_e1683_q_d_n1, eq50_e1683_q_d_n2, eq50_e1683_q_d_n3, eq50_e1683_q_d_n4, eq50_e1683_q_d_n5, eq50_e1683_q_d_n6, eq50_e1683_q_d_n7, eq50_e1683_q_d_n8, eq50_e1683_q_d_n9, eq50_e1683_q_d_n10, eq50_e1683_q_d_n11, eq50_e1683_q_d_n12, eq50_e1683_q_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 14] = [eq50_e1685_q_d_n0, eq50_e1685_q_d_n1, eq50_e1685_q_d_n2, eq50_e1685_q_d_n3, eq50_e1685_q_d_n4, eq50_e1685_q_d_n5, eq50_e1685_q_d_n6, eq50_e1685_q_d_n7, eq50_e1685_q_d_n8, eq50_e1685_q_d_n9, eq50_e1685_q_d_n10, eq50_e1685_q_d_n11, eq50_e1685_q_d_n12, eq50_e1685_q_d_n13];
        let eq50_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            &nodes,
            &eq50_reactive_node_derivatives,
            &branches,
            &eq50_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
