#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq60_e1419: f64 = (s.v[0] * s.v[19]);
        let eq60_e1419_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq60_e1419_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq60_e1419_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq60_e1419_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq60_e1419_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq60_e1419_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq60_e1419_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq60_e1419_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq60_e1419_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq60_e1419_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq60_e1419_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq60_e1419_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq60_e1419_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq60_e1419_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq60_e1419_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq60_e1419_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq60_e1419_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq60_e1419_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq60_e1419_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq60_e1419_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq60_e1419_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq60_e1419_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq60_e1419_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq60_e1419_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq60_e1419_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq60_e1419_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq60_e1419_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq60_e1419_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq60_e1419_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq60_e1419_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq60_e1419_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq60_e1419_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq60_e1419_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq60_e1419_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq60_e1419_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq60_e1419_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq60_e1419_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq60_e1419_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq60_e1419_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq60_e1419_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq60_e1419_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq60_e1419_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq60_e1419_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq60_e1419_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq60_e1419_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq60_e1419_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq60_e1421: f64 = (eq60_e1419 * p.p33);
        let eq60_e1421_d_n0: f64 = (eq60_e1419_d_n0 * p.p33);
        let eq60_e1421_d_n1: f64 = (eq60_e1419_d_n1 * p.p33);
        let eq60_e1421_d_n2: f64 = (eq60_e1419_d_n2 * p.p33);
        let eq60_e1421_d_n3: f64 = (eq60_e1419_d_n3 * p.p33);
        let eq60_e1421_d_n4: f64 = (eq60_e1419_d_n4 * p.p33);
        let eq60_e1421_d_n5: f64 = (eq60_e1419_d_n5 * p.p33);
        let eq60_e1421_d_n6: f64 = (eq60_e1419_d_n6 * p.p33);
        let eq60_e1421_d_n7: f64 = (eq60_e1419_d_n7 * p.p33);
        let eq60_e1421_d_n8: f64 = (eq60_e1419_d_n8 * p.p33);
        let eq60_e1421_d_n9: f64 = (eq60_e1419_d_n9 * p.p33);
        let eq60_e1421_d_n10: f64 = (eq60_e1419_d_n10 * p.p33);
        let eq60_e1421_d_n11: f64 = (eq60_e1419_d_n11 * p.p33);
        let eq60_e1421_d_n12: f64 = (eq60_e1419_d_n12 * p.p33);
        let eq60_e1421_d_n13: f64 = (eq60_e1419_d_n13 * p.p33);
        let eq60_e1421_d_n14: f64 = (eq60_e1419_d_n14 * p.p33);
        let eq60_e1421_d_n15: f64 = (eq60_e1419_d_n15 * p.p33);
        let eq60_e1421_d_n16: f64 = (eq60_e1419_d_n16 * p.p33);
        let eq60_e1421_d_n17: f64 = (eq60_e1419_d_n17 * p.p33);
        let eq60_e1421_d_n18: f64 = (eq60_e1419_d_n18 * p.p33);
        let eq60_e1421_d_n19: f64 = (eq60_e1419_d_n19 * p.p33);
        let eq60_e1421_d_n20: f64 = (eq60_e1419_d_n20 * p.p33);
        let eq60_e1421_d_b0: f64 = (eq60_e1419_d_b0 * p.p33);
        let eq60_e1421_d_b1: f64 = (eq60_e1419_d_b1 * p.p33);
        let eq60_e1421_d_b2: f64 = (eq60_e1419_d_b2 * p.p33);
        let eq60_e1421_d_b3: f64 = (eq60_e1419_d_b3 * p.p33);
        let eq60_e1421_d_b4: f64 = (eq60_e1419_d_b4 * p.p33);
        let eq60_e1421_d_b5: f64 = (eq60_e1419_d_b5 * p.p33);
        let eq60_e1421_d_b6: f64 = (eq60_e1419_d_b6 * p.p33);
        let eq60_e1421_d_b7: f64 = (eq60_e1419_d_b7 * p.p33);
        let eq60_e1421_d_b8: f64 = (eq60_e1419_d_b8 * p.p33);
        let eq60_e1421_d_b9: f64 = (eq60_e1419_d_b9 * p.p33);
        let eq60_e1421_d_b10: f64 = (eq60_e1419_d_b10 * p.p33);
        let eq60_e1421_d_b11: f64 = (eq60_e1419_d_b11 * p.p33);
        let eq60_e1421_d_b12: f64 = (eq60_e1419_d_b12 * p.p33);
        let eq60_e1421_d_b13: f64 = (eq60_e1419_d_b13 * p.p33);
        let eq60_e1421_d_b14: f64 = (eq60_e1419_d_b14 * p.p33);
        let eq60_e1421_d_b15: f64 = (eq60_e1419_d_b15 * p.p33);
        let eq60_e1421_d_b16: f64 = (eq60_e1419_d_b16 * p.p33);
        let eq60_e1421_d_b17: f64 = (eq60_e1419_d_b17 * p.p33);
        let eq60_e1421_d_b18: f64 = (eq60_e1419_d_b18 * p.p33);
        let eq60_e1421_d_b19: f64 = (eq60_e1419_d_b19 * p.p33);
        let eq60_e1421_d_b20: f64 = (eq60_e1419_d_b20 * p.p33);
        let eq60_e1421_d_b21: f64 = (eq60_e1419_d_b21 * p.p33);
        let eq60_e1421_d_b22: f64 = (eq60_e1419_d_b22 * p.p33);
        let eq60_e1421_d_b23: f64 = (eq60_e1419_d_b23 * p.p33);
        let eq60_e1421_d_b24: f64 = (eq60_e1419_d_b24 * p.p33);
        let eq60_e1423: f64 = (eq60_e1421 * s.v[855]);
        let eq60_e1423_d_n0: f64 = ((eq60_e1421_d_n0 * s.v[855]) + (eq60_e1421 * s.dn[855][0]));
        let eq60_e1423_d_n1: f64 = ((eq60_e1421_d_n1 * s.v[855]) + (eq60_e1421 * s.dn[855][1]));
        let eq60_e1423_d_n2: f64 = ((eq60_e1421_d_n2 * s.v[855]) + (eq60_e1421 * s.dn[855][2]));
        let eq60_e1423_d_n3: f64 = ((eq60_e1421_d_n3 * s.v[855]) + (eq60_e1421 * s.dn[855][3]));
        let eq60_e1423_d_n4: f64 = ((eq60_e1421_d_n4 * s.v[855]) + (eq60_e1421 * s.dn[855][4]));
        let eq60_e1423_d_n5: f64 = ((eq60_e1421_d_n5 * s.v[855]) + (eq60_e1421 * s.dn[855][5]));
        let eq60_e1423_d_n6: f64 = ((eq60_e1421_d_n6 * s.v[855]) + (eq60_e1421 * s.dn[855][6]));
        let eq60_e1423_d_n7: f64 = ((eq60_e1421_d_n7 * s.v[855]) + (eq60_e1421 * s.dn[855][7]));
        let eq60_e1423_d_n8: f64 = ((eq60_e1421_d_n8 * s.v[855]) + (eq60_e1421 * s.dn[855][8]));
        let eq60_e1423_d_n9: f64 = ((eq60_e1421_d_n9 * s.v[855]) + (eq60_e1421 * s.dn[855][9]));
        let eq60_e1423_d_n10: f64 = ((eq60_e1421_d_n10 * s.v[855]) + (eq60_e1421 * s.dn[855][10]));
        let eq60_e1423_d_n11: f64 = ((eq60_e1421_d_n11 * s.v[855]) + (eq60_e1421 * s.dn[855][11]));
        let eq60_e1423_d_n12: f64 = ((eq60_e1421_d_n12 * s.v[855]) + (eq60_e1421 * s.dn[855][12]));
        let eq60_e1423_d_n13: f64 = ((eq60_e1421_d_n13 * s.v[855]) + (eq60_e1421 * s.dn[855][13]));
        let eq60_e1423_d_n14: f64 = ((eq60_e1421_d_n14 * s.v[855]) + (eq60_e1421 * s.dn[855][14]));
        let eq60_e1423_d_n15: f64 = ((eq60_e1421_d_n15 * s.v[855]) + (eq60_e1421 * s.dn[855][15]));
        let eq60_e1423_d_n16: f64 = ((eq60_e1421_d_n16 * s.v[855]) + (eq60_e1421 * s.dn[855][16]));
        let eq60_e1423_d_n17: f64 = ((eq60_e1421_d_n17 * s.v[855]) + (eq60_e1421 * s.dn[855][17]));
        let eq60_e1423_d_n18: f64 = ((eq60_e1421_d_n18 * s.v[855]) + (eq60_e1421 * s.dn[855][18]));
        let eq60_e1423_d_n19: f64 = ((eq60_e1421_d_n19 * s.v[855]) + (eq60_e1421 * s.dn[855][19]));
        let eq60_e1423_d_n20: f64 = ((eq60_e1421_d_n20 * s.v[855]) + (eq60_e1421 * s.dn[855][20]));
        let eq60_e1423_d_b0: f64 = ((eq60_e1421_d_b0 * s.v[855]) + (eq60_e1421 * s.db[855][0]));
        let eq60_e1423_d_b1: f64 = ((eq60_e1421_d_b1 * s.v[855]) + (eq60_e1421 * s.db[855][1]));
        let eq60_e1423_d_b2: f64 = ((eq60_e1421_d_b2 * s.v[855]) + (eq60_e1421 * s.db[855][2]));
        let eq60_e1423_d_b3: f64 = ((eq60_e1421_d_b3 * s.v[855]) + (eq60_e1421 * s.db[855][3]));
        let eq60_e1423_d_b4: f64 = ((eq60_e1421_d_b4 * s.v[855]) + (eq60_e1421 * s.db[855][4]));
        let eq60_e1423_d_b5: f64 = ((eq60_e1421_d_b5 * s.v[855]) + (eq60_e1421 * s.db[855][5]));
        let eq60_e1423_d_b6: f64 = ((eq60_e1421_d_b6 * s.v[855]) + (eq60_e1421 * s.db[855][6]));
        let eq60_e1423_d_b7: f64 = ((eq60_e1421_d_b7 * s.v[855]) + (eq60_e1421 * s.db[855][7]));
        let eq60_e1423_d_b8: f64 = ((eq60_e1421_d_b8 * s.v[855]) + (eq60_e1421 * s.db[855][8]));
        let eq60_e1423_d_b9: f64 = ((eq60_e1421_d_b9 * s.v[855]) + (eq60_e1421 * s.db[855][9]));
        let eq60_e1423_d_b10: f64 = ((eq60_e1421_d_b10 * s.v[855]) + (eq60_e1421 * s.db[855][10]));
        let eq60_e1423_d_b11: f64 = ((eq60_e1421_d_b11 * s.v[855]) + (eq60_e1421 * s.db[855][11]));
        let eq60_e1423_d_b12: f64 = ((eq60_e1421_d_b12 * s.v[855]) + (eq60_e1421 * s.db[855][12]));
        let eq60_e1423_d_b13: f64 = ((eq60_e1421_d_b13 * s.v[855]) + (eq60_e1421 * s.db[855][13]));
        let eq60_e1423_d_b14: f64 = ((eq60_e1421_d_b14 * s.v[855]) + (eq60_e1421 * s.db[855][14]));
        let eq60_e1423_d_b15: f64 = ((eq60_e1421_d_b15 * s.v[855]) + (eq60_e1421 * s.db[855][15]));
        let eq60_e1423_d_b16: f64 = ((eq60_e1421_d_b16 * s.v[855]) + (eq60_e1421 * s.db[855][16]));
        let eq60_e1423_d_b17: f64 = ((eq60_e1421_d_b17 * s.v[855]) + (eq60_e1421 * s.db[855][17]));
        let eq60_e1423_d_b18: f64 = ((eq60_e1421_d_b18 * s.v[855]) + (eq60_e1421 * s.db[855][18]));
        let eq60_e1423_d_b19: f64 = ((eq60_e1421_d_b19 * s.v[855]) + (eq60_e1421 * s.db[855][19]));
        let eq60_e1423_d_b20: f64 = ((eq60_e1421_d_b20 * s.v[855]) + (eq60_e1421 * s.db[855][20]));
        let eq60_e1423_d_b21: f64 = ((eq60_e1421_d_b21 * s.v[855]) + (eq60_e1421 * s.db[855][21]));
        let eq60_e1423_d_b22: f64 = ((eq60_e1421_d_b22 * s.v[855]) + (eq60_e1421 * s.db[855][22]));
        let eq60_e1423_d_b23: f64 = ((eq60_e1421_d_b23 * s.v[855]) + (eq60_e1421 * s.db[855][23]));
        let eq60_e1423_d_b24: f64 = ((eq60_e1421_d_b24 * s.v[855]) + (eq60_e1421 * s.db[855][24]));
        let eq60_e1424: f64 = self.eval_ddt(4, eq60_e1423);
        let eq60_e1424_d_n0: f64 = self.ddt_jacobian(eq60_e1423_d_n0);
        let eq60_e1424_d_n1: f64 = self.ddt_jacobian(eq60_e1423_d_n1);
        let eq60_e1424_d_n2: f64 = self.ddt_jacobian(eq60_e1423_d_n2);
        let eq60_e1424_d_n3: f64 = self.ddt_jacobian(eq60_e1423_d_n3);
        let eq60_e1424_d_n4: f64 = self.ddt_jacobian(eq60_e1423_d_n4);
        let eq60_e1424_d_n5: f64 = self.ddt_jacobian(eq60_e1423_d_n5);
        let eq60_e1424_d_n6: f64 = self.ddt_jacobian(eq60_e1423_d_n6);
        let eq60_e1424_d_n7: f64 = self.ddt_jacobian(eq60_e1423_d_n7);
        let eq60_e1424_d_n8: f64 = self.ddt_jacobian(eq60_e1423_d_n8);
        let eq60_e1424_d_n9: f64 = self.ddt_jacobian(eq60_e1423_d_n9);
        let eq60_e1424_d_n10: f64 = self.ddt_jacobian(eq60_e1423_d_n10);
        let eq60_e1424_d_n11: f64 = self.ddt_jacobian(eq60_e1423_d_n11);
        let eq60_e1424_d_n12: f64 = self.ddt_jacobian(eq60_e1423_d_n12);
        let eq60_e1424_d_n13: f64 = self.ddt_jacobian(eq60_e1423_d_n13);
        let eq60_e1424_d_n14: f64 = self.ddt_jacobian(eq60_e1423_d_n14);
        let eq60_e1424_d_n15: f64 = self.ddt_jacobian(eq60_e1423_d_n15);
        let eq60_e1424_d_n16: f64 = self.ddt_jacobian(eq60_e1423_d_n16);
        let eq60_e1424_d_n17: f64 = self.ddt_jacobian(eq60_e1423_d_n17);
        let eq60_e1424_d_n18: f64 = self.ddt_jacobian(eq60_e1423_d_n18);
        let eq60_e1424_d_n19: f64 = self.ddt_jacobian(eq60_e1423_d_n19);
        let eq60_e1424_d_n20: f64 = self.ddt_jacobian(eq60_e1423_d_n20);
        let eq60_e1424_d_b0: f64 = self.ddt_jacobian(eq60_e1423_d_b0);
        let eq60_e1424_d_b1: f64 = self.ddt_jacobian(eq60_e1423_d_b1);
        let eq60_e1424_d_b2: f64 = self.ddt_jacobian(eq60_e1423_d_b2);
        let eq60_e1424_d_b3: f64 = self.ddt_jacobian(eq60_e1423_d_b3);
        let eq60_e1424_d_b4: f64 = self.ddt_jacobian(eq60_e1423_d_b4);
        let eq60_e1424_d_b5: f64 = self.ddt_jacobian(eq60_e1423_d_b5);
        let eq60_e1424_d_b6: f64 = self.ddt_jacobian(eq60_e1423_d_b6);
        let eq60_e1424_d_b7: f64 = self.ddt_jacobian(eq60_e1423_d_b7);
        let eq60_e1424_d_b8: f64 = self.ddt_jacobian(eq60_e1423_d_b8);
        let eq60_e1424_d_b9: f64 = self.ddt_jacobian(eq60_e1423_d_b9);
        let eq60_e1424_d_b10: f64 = self.ddt_jacobian(eq60_e1423_d_b10);
        let eq60_e1424_d_b11: f64 = self.ddt_jacobian(eq60_e1423_d_b11);
        let eq60_e1424_d_b12: f64 = self.ddt_jacobian(eq60_e1423_d_b12);
        let eq60_e1424_d_b13: f64 = self.ddt_jacobian(eq60_e1423_d_b13);
        let eq60_e1424_d_b14: f64 = self.ddt_jacobian(eq60_e1423_d_b14);
        let eq60_e1424_d_b15: f64 = self.ddt_jacobian(eq60_e1423_d_b15);
        let eq60_e1424_d_b16: f64 = self.ddt_jacobian(eq60_e1423_d_b16);
        let eq60_e1424_d_b17: f64 = self.ddt_jacobian(eq60_e1423_d_b17);
        let eq60_e1424_d_b18: f64 = self.ddt_jacobian(eq60_e1423_d_b18);
        let eq60_e1424_d_b19: f64 = self.ddt_jacobian(eq60_e1423_d_b19);
        let eq60_e1424_d_b20: f64 = self.ddt_jacobian(eq60_e1423_d_b20);
        let eq60_e1424_d_b21: f64 = self.ddt_jacobian(eq60_e1423_d_b21);
        let eq60_e1424_d_b22: f64 = self.ddt_jacobian(eq60_e1423_d_b22);
        let eq60_e1424_d_b23: f64 = self.ddt_jacobian(eq60_e1423_d_b23);
        let eq60_e1424_d_b24: f64 = self.ddt_jacobian(eq60_e1423_d_b24);
        let eq60_value: f64 = eq60_e1424;
        let eq60_node_derivatives: [f64; 21] = [eq60_e1424_d_n0, eq60_e1424_d_n1, eq60_e1424_d_n2, eq60_e1424_d_n3, eq60_e1424_d_n4, eq60_e1424_d_n5, eq60_e1424_d_n6, eq60_e1424_d_n7, eq60_e1424_d_n8, eq60_e1424_d_n9, eq60_e1424_d_n10, eq60_e1424_d_n11, eq60_e1424_d_n12, eq60_e1424_d_n13, eq60_e1424_d_n14, eq60_e1424_d_n15, eq60_e1424_d_n16, eq60_e1424_d_n17, eq60_e1424_d_n18, eq60_e1424_d_n19, eq60_e1424_d_n20];
        let eq60_branch_derivatives: [f64; 25] = [eq60_e1424_d_b0, eq60_e1424_d_b1, eq60_e1424_d_b2, eq60_e1424_d_b3, eq60_e1424_d_b4, eq60_e1424_d_b5, eq60_e1424_d_b6, eq60_e1424_d_b7, eq60_e1424_d_b8, eq60_e1424_d_b9, eq60_e1424_d_b10, eq60_e1424_d_b11, eq60_e1424_d_b12, eq60_e1424_d_b13, eq60_e1424_d_b14, eq60_e1424_d_b15, eq60_e1424_d_b16, eq60_e1424_d_b17, eq60_e1424_d_b18, eq60_e1424_d_b19, eq60_e1424_d_b20, eq60_e1424_d_b21, eq60_e1424_d_b22, eq60_e1424_d_b23, eq60_e1424_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
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
        let eq61_e1427: f64 = (s.v[0] * s.v[19]);
        let eq61_e1427_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq61_e1427_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq61_e1427_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq61_e1427_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq61_e1427_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq61_e1427_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq61_e1427_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq61_e1427_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq61_e1427_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq61_e1427_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq61_e1427_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq61_e1427_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq61_e1427_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq61_e1427_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq61_e1427_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq61_e1427_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq61_e1427_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq61_e1427_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq61_e1427_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq61_e1427_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq61_e1427_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq61_e1427_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq61_e1427_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq61_e1427_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq61_e1427_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq61_e1427_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq61_e1427_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq61_e1427_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq61_e1427_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq61_e1427_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq61_e1427_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq61_e1427_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq61_e1427_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq61_e1427_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq61_e1427_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq61_e1427_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq61_e1427_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq61_e1427_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq61_e1427_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq61_e1427_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq61_e1427_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq61_e1427_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq61_e1427_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq61_e1427_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq61_e1427_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq61_e1427_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq61_e1429: f64 = (eq61_e1427 * p.p33);
        let eq61_e1429_d_n0: f64 = (eq61_e1427_d_n0 * p.p33);
        let eq61_e1429_d_n1: f64 = (eq61_e1427_d_n1 * p.p33);
        let eq61_e1429_d_n2: f64 = (eq61_e1427_d_n2 * p.p33);
        let eq61_e1429_d_n3: f64 = (eq61_e1427_d_n3 * p.p33);
        let eq61_e1429_d_n4: f64 = (eq61_e1427_d_n4 * p.p33);
        let eq61_e1429_d_n5: f64 = (eq61_e1427_d_n5 * p.p33);
        let eq61_e1429_d_n6: f64 = (eq61_e1427_d_n6 * p.p33);
        let eq61_e1429_d_n7: f64 = (eq61_e1427_d_n7 * p.p33);
        let eq61_e1429_d_n8: f64 = (eq61_e1427_d_n8 * p.p33);
        let eq61_e1429_d_n9: f64 = (eq61_e1427_d_n9 * p.p33);
        let eq61_e1429_d_n10: f64 = (eq61_e1427_d_n10 * p.p33);
        let eq61_e1429_d_n11: f64 = (eq61_e1427_d_n11 * p.p33);
        let eq61_e1429_d_n12: f64 = (eq61_e1427_d_n12 * p.p33);
        let eq61_e1429_d_n13: f64 = (eq61_e1427_d_n13 * p.p33);
        let eq61_e1429_d_n14: f64 = (eq61_e1427_d_n14 * p.p33);
        let eq61_e1429_d_n15: f64 = (eq61_e1427_d_n15 * p.p33);
        let eq61_e1429_d_n16: f64 = (eq61_e1427_d_n16 * p.p33);
        let eq61_e1429_d_n17: f64 = (eq61_e1427_d_n17 * p.p33);
        let eq61_e1429_d_n18: f64 = (eq61_e1427_d_n18 * p.p33);
        let eq61_e1429_d_n19: f64 = (eq61_e1427_d_n19 * p.p33);
        let eq61_e1429_d_n20: f64 = (eq61_e1427_d_n20 * p.p33);
        let eq61_e1429_d_b0: f64 = (eq61_e1427_d_b0 * p.p33);
        let eq61_e1429_d_b1: f64 = (eq61_e1427_d_b1 * p.p33);
        let eq61_e1429_d_b2: f64 = (eq61_e1427_d_b2 * p.p33);
        let eq61_e1429_d_b3: f64 = (eq61_e1427_d_b3 * p.p33);
        let eq61_e1429_d_b4: f64 = (eq61_e1427_d_b4 * p.p33);
        let eq61_e1429_d_b5: f64 = (eq61_e1427_d_b5 * p.p33);
        let eq61_e1429_d_b6: f64 = (eq61_e1427_d_b6 * p.p33);
        let eq61_e1429_d_b7: f64 = (eq61_e1427_d_b7 * p.p33);
        let eq61_e1429_d_b8: f64 = (eq61_e1427_d_b8 * p.p33);
        let eq61_e1429_d_b9: f64 = (eq61_e1427_d_b9 * p.p33);
        let eq61_e1429_d_b10: f64 = (eq61_e1427_d_b10 * p.p33);
        let eq61_e1429_d_b11: f64 = (eq61_e1427_d_b11 * p.p33);
        let eq61_e1429_d_b12: f64 = (eq61_e1427_d_b12 * p.p33);
        let eq61_e1429_d_b13: f64 = (eq61_e1427_d_b13 * p.p33);
        let eq61_e1429_d_b14: f64 = (eq61_e1427_d_b14 * p.p33);
        let eq61_e1429_d_b15: f64 = (eq61_e1427_d_b15 * p.p33);
        let eq61_e1429_d_b16: f64 = (eq61_e1427_d_b16 * p.p33);
        let eq61_e1429_d_b17: f64 = (eq61_e1427_d_b17 * p.p33);
        let eq61_e1429_d_b18: f64 = (eq61_e1427_d_b18 * p.p33);
        let eq61_e1429_d_b19: f64 = (eq61_e1427_d_b19 * p.p33);
        let eq61_e1429_d_b20: f64 = (eq61_e1427_d_b20 * p.p33);
        let eq61_e1429_d_b21: f64 = (eq61_e1427_d_b21 * p.p33);
        let eq61_e1429_d_b22: f64 = (eq61_e1427_d_b22 * p.p33);
        let eq61_e1429_d_b23: f64 = (eq61_e1427_d_b23 * p.p33);
        let eq61_e1429_d_b24: f64 = (eq61_e1427_d_b24 * p.p33);
        let eq61_e1431: f64 = (eq61_e1429 * s.v[856]);
        let eq61_e1431_d_n0: f64 = ((eq61_e1429_d_n0 * s.v[856]) + (eq61_e1429 * s.dn[856][0]));
        let eq61_e1431_d_n1: f64 = ((eq61_e1429_d_n1 * s.v[856]) + (eq61_e1429 * s.dn[856][1]));
        let eq61_e1431_d_n2: f64 = ((eq61_e1429_d_n2 * s.v[856]) + (eq61_e1429 * s.dn[856][2]));
        let eq61_e1431_d_n3: f64 = ((eq61_e1429_d_n3 * s.v[856]) + (eq61_e1429 * s.dn[856][3]));
        let eq61_e1431_d_n4: f64 = ((eq61_e1429_d_n4 * s.v[856]) + (eq61_e1429 * s.dn[856][4]));
        let eq61_e1431_d_n5: f64 = ((eq61_e1429_d_n5 * s.v[856]) + (eq61_e1429 * s.dn[856][5]));
        let eq61_e1431_d_n6: f64 = ((eq61_e1429_d_n6 * s.v[856]) + (eq61_e1429 * s.dn[856][6]));
        let eq61_e1431_d_n7: f64 = ((eq61_e1429_d_n7 * s.v[856]) + (eq61_e1429 * s.dn[856][7]));
        let eq61_e1431_d_n8: f64 = ((eq61_e1429_d_n8 * s.v[856]) + (eq61_e1429 * s.dn[856][8]));
        let eq61_e1431_d_n9: f64 = ((eq61_e1429_d_n9 * s.v[856]) + (eq61_e1429 * s.dn[856][9]));
        let eq61_e1431_d_n10: f64 = ((eq61_e1429_d_n10 * s.v[856]) + (eq61_e1429 * s.dn[856][10]));
        let eq61_e1431_d_n11: f64 = ((eq61_e1429_d_n11 * s.v[856]) + (eq61_e1429 * s.dn[856][11]));
        let eq61_e1431_d_n12: f64 = ((eq61_e1429_d_n12 * s.v[856]) + (eq61_e1429 * s.dn[856][12]));
        let eq61_e1431_d_n13: f64 = ((eq61_e1429_d_n13 * s.v[856]) + (eq61_e1429 * s.dn[856][13]));
        let eq61_e1431_d_n14: f64 = ((eq61_e1429_d_n14 * s.v[856]) + (eq61_e1429 * s.dn[856][14]));
        let eq61_e1431_d_n15: f64 = ((eq61_e1429_d_n15 * s.v[856]) + (eq61_e1429 * s.dn[856][15]));
        let eq61_e1431_d_n16: f64 = ((eq61_e1429_d_n16 * s.v[856]) + (eq61_e1429 * s.dn[856][16]));
        let eq61_e1431_d_n17: f64 = ((eq61_e1429_d_n17 * s.v[856]) + (eq61_e1429 * s.dn[856][17]));
        let eq61_e1431_d_n18: f64 = ((eq61_e1429_d_n18 * s.v[856]) + (eq61_e1429 * s.dn[856][18]));
        let eq61_e1431_d_n19: f64 = ((eq61_e1429_d_n19 * s.v[856]) + (eq61_e1429 * s.dn[856][19]));
        let eq61_e1431_d_n20: f64 = ((eq61_e1429_d_n20 * s.v[856]) + (eq61_e1429 * s.dn[856][20]));
        let eq61_e1431_d_b0: f64 = ((eq61_e1429_d_b0 * s.v[856]) + (eq61_e1429 * s.db[856][0]));
        let eq61_e1431_d_b1: f64 = ((eq61_e1429_d_b1 * s.v[856]) + (eq61_e1429 * s.db[856][1]));
        let eq61_e1431_d_b2: f64 = ((eq61_e1429_d_b2 * s.v[856]) + (eq61_e1429 * s.db[856][2]));
        let eq61_e1431_d_b3: f64 = ((eq61_e1429_d_b3 * s.v[856]) + (eq61_e1429 * s.db[856][3]));
        let eq61_e1431_d_b4: f64 = ((eq61_e1429_d_b4 * s.v[856]) + (eq61_e1429 * s.db[856][4]));
        let eq61_e1431_d_b5: f64 = ((eq61_e1429_d_b5 * s.v[856]) + (eq61_e1429 * s.db[856][5]));
        let eq61_e1431_d_b6: f64 = ((eq61_e1429_d_b6 * s.v[856]) + (eq61_e1429 * s.db[856][6]));
        let eq61_e1431_d_b7: f64 = ((eq61_e1429_d_b7 * s.v[856]) + (eq61_e1429 * s.db[856][7]));
        let eq61_e1431_d_b8: f64 = ((eq61_e1429_d_b8 * s.v[856]) + (eq61_e1429 * s.db[856][8]));
        let eq61_e1431_d_b9: f64 = ((eq61_e1429_d_b9 * s.v[856]) + (eq61_e1429 * s.db[856][9]));
        let eq61_e1431_d_b10: f64 = ((eq61_e1429_d_b10 * s.v[856]) + (eq61_e1429 * s.db[856][10]));
        let eq61_e1431_d_b11: f64 = ((eq61_e1429_d_b11 * s.v[856]) + (eq61_e1429 * s.db[856][11]));
        let eq61_e1431_d_b12: f64 = ((eq61_e1429_d_b12 * s.v[856]) + (eq61_e1429 * s.db[856][12]));
        let eq61_e1431_d_b13: f64 = ((eq61_e1429_d_b13 * s.v[856]) + (eq61_e1429 * s.db[856][13]));
        let eq61_e1431_d_b14: f64 = ((eq61_e1429_d_b14 * s.v[856]) + (eq61_e1429 * s.db[856][14]));
        let eq61_e1431_d_b15: f64 = ((eq61_e1429_d_b15 * s.v[856]) + (eq61_e1429 * s.db[856][15]));
        let eq61_e1431_d_b16: f64 = ((eq61_e1429_d_b16 * s.v[856]) + (eq61_e1429 * s.db[856][16]));
        let eq61_e1431_d_b17: f64 = ((eq61_e1429_d_b17 * s.v[856]) + (eq61_e1429 * s.db[856][17]));
        let eq61_e1431_d_b18: f64 = ((eq61_e1429_d_b18 * s.v[856]) + (eq61_e1429 * s.db[856][18]));
        let eq61_e1431_d_b19: f64 = ((eq61_e1429_d_b19 * s.v[856]) + (eq61_e1429 * s.db[856][19]));
        let eq61_e1431_d_b20: f64 = ((eq61_e1429_d_b20 * s.v[856]) + (eq61_e1429 * s.db[856][20]));
        let eq61_e1431_d_b21: f64 = ((eq61_e1429_d_b21 * s.v[856]) + (eq61_e1429 * s.db[856][21]));
        let eq61_e1431_d_b22: f64 = ((eq61_e1429_d_b22 * s.v[856]) + (eq61_e1429 * s.db[856][22]));
        let eq61_e1431_d_b23: f64 = ((eq61_e1429_d_b23 * s.v[856]) + (eq61_e1429 * s.db[856][23]));
        let eq61_e1431_d_b24: f64 = ((eq61_e1429_d_b24 * s.v[856]) + (eq61_e1429 * s.db[856][24]));
        let eq61_e1432: f64 = self.eval_ddt(5, eq61_e1431);
        let eq61_e1432_d_n0: f64 = self.ddt_jacobian(eq61_e1431_d_n0);
        let eq61_e1432_d_n1: f64 = self.ddt_jacobian(eq61_e1431_d_n1);
        let eq61_e1432_d_n2: f64 = self.ddt_jacobian(eq61_e1431_d_n2);
        let eq61_e1432_d_n3: f64 = self.ddt_jacobian(eq61_e1431_d_n3);
        let eq61_e1432_d_n4: f64 = self.ddt_jacobian(eq61_e1431_d_n4);
        let eq61_e1432_d_n5: f64 = self.ddt_jacobian(eq61_e1431_d_n5);
        let eq61_e1432_d_n6: f64 = self.ddt_jacobian(eq61_e1431_d_n6);
        let eq61_e1432_d_n7: f64 = self.ddt_jacobian(eq61_e1431_d_n7);
        let eq61_e1432_d_n8: f64 = self.ddt_jacobian(eq61_e1431_d_n8);
        let eq61_e1432_d_n9: f64 = self.ddt_jacobian(eq61_e1431_d_n9);
        let eq61_e1432_d_n10: f64 = self.ddt_jacobian(eq61_e1431_d_n10);
        let eq61_e1432_d_n11: f64 = self.ddt_jacobian(eq61_e1431_d_n11);
        let eq61_e1432_d_n12: f64 = self.ddt_jacobian(eq61_e1431_d_n12);
        let eq61_e1432_d_n13: f64 = self.ddt_jacobian(eq61_e1431_d_n13);
        let eq61_e1432_d_n14: f64 = self.ddt_jacobian(eq61_e1431_d_n14);
        let eq61_e1432_d_n15: f64 = self.ddt_jacobian(eq61_e1431_d_n15);
        let eq61_e1432_d_n16: f64 = self.ddt_jacobian(eq61_e1431_d_n16);
        let eq61_e1432_d_n17: f64 = self.ddt_jacobian(eq61_e1431_d_n17);
        let eq61_e1432_d_n18: f64 = self.ddt_jacobian(eq61_e1431_d_n18);
        let eq61_e1432_d_n19: f64 = self.ddt_jacobian(eq61_e1431_d_n19);
        let eq61_e1432_d_n20: f64 = self.ddt_jacobian(eq61_e1431_d_n20);
        let eq61_e1432_d_b0: f64 = self.ddt_jacobian(eq61_e1431_d_b0);
        let eq61_e1432_d_b1: f64 = self.ddt_jacobian(eq61_e1431_d_b1);
        let eq61_e1432_d_b2: f64 = self.ddt_jacobian(eq61_e1431_d_b2);
        let eq61_e1432_d_b3: f64 = self.ddt_jacobian(eq61_e1431_d_b3);
        let eq61_e1432_d_b4: f64 = self.ddt_jacobian(eq61_e1431_d_b4);
        let eq61_e1432_d_b5: f64 = self.ddt_jacobian(eq61_e1431_d_b5);
        let eq61_e1432_d_b6: f64 = self.ddt_jacobian(eq61_e1431_d_b6);
        let eq61_e1432_d_b7: f64 = self.ddt_jacobian(eq61_e1431_d_b7);
        let eq61_e1432_d_b8: f64 = self.ddt_jacobian(eq61_e1431_d_b8);
        let eq61_e1432_d_b9: f64 = self.ddt_jacobian(eq61_e1431_d_b9);
        let eq61_e1432_d_b10: f64 = self.ddt_jacobian(eq61_e1431_d_b10);
        let eq61_e1432_d_b11: f64 = self.ddt_jacobian(eq61_e1431_d_b11);
        let eq61_e1432_d_b12: f64 = self.ddt_jacobian(eq61_e1431_d_b12);
        let eq61_e1432_d_b13: f64 = self.ddt_jacobian(eq61_e1431_d_b13);
        let eq61_e1432_d_b14: f64 = self.ddt_jacobian(eq61_e1431_d_b14);
        let eq61_e1432_d_b15: f64 = self.ddt_jacobian(eq61_e1431_d_b15);
        let eq61_e1432_d_b16: f64 = self.ddt_jacobian(eq61_e1431_d_b16);
        let eq61_e1432_d_b17: f64 = self.ddt_jacobian(eq61_e1431_d_b17);
        let eq61_e1432_d_b18: f64 = self.ddt_jacobian(eq61_e1431_d_b18);
        let eq61_e1432_d_b19: f64 = self.ddt_jacobian(eq61_e1431_d_b19);
        let eq61_e1432_d_b20: f64 = self.ddt_jacobian(eq61_e1431_d_b20);
        let eq61_e1432_d_b21: f64 = self.ddt_jacobian(eq61_e1431_d_b21);
        let eq61_e1432_d_b22: f64 = self.ddt_jacobian(eq61_e1431_d_b22);
        let eq61_e1432_d_b23: f64 = self.ddt_jacobian(eq61_e1431_d_b23);
        let eq61_e1432_d_b24: f64 = self.ddt_jacobian(eq61_e1431_d_b24);
        let eq61_value: f64 = eq61_e1432;
        let eq61_node_derivatives: [f64; 21] = [eq61_e1432_d_n0, eq61_e1432_d_n1, eq61_e1432_d_n2, eq61_e1432_d_n3, eq61_e1432_d_n4, eq61_e1432_d_n5, eq61_e1432_d_n6, eq61_e1432_d_n7, eq61_e1432_d_n8, eq61_e1432_d_n9, eq61_e1432_d_n10, eq61_e1432_d_n11, eq61_e1432_d_n12, eq61_e1432_d_n13, eq61_e1432_d_n14, eq61_e1432_d_n15, eq61_e1432_d_n16, eq61_e1432_d_n17, eq61_e1432_d_n18, eq61_e1432_d_n19, eq61_e1432_d_n20];
        let eq61_branch_derivatives: [f64; 25] = [eq61_e1432_d_b0, eq61_e1432_d_b1, eq61_e1432_d_b2, eq61_e1432_d_b3, eq61_e1432_d_b4, eq61_e1432_d_b5, eq61_e1432_d_b6, eq61_e1432_d_b7, eq61_e1432_d_b8, eq61_e1432_d_b9, eq61_e1432_d_b10, eq61_e1432_d_b11, eq61_e1432_d_b12, eq61_e1432_d_b13, eq61_e1432_d_b14, eq61_e1432_d_b15, eq61_e1432_d_b16, eq61_e1432_d_b17, eq61_e1432_d_b18, eq61_e1432_d_b19, eq61_e1432_d_b20, eq61_e1432_d_b21, eq61_e1432_d_b22, eq61_e1432_d_b23, eq61_e1432_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[8]),
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
        let eq62_e1435: f64 = (s.v[0] * s.v[19]);
        let eq62_e1435_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq62_e1435_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq62_e1435_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq62_e1435_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq62_e1435_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq62_e1435_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq62_e1435_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq62_e1435_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq62_e1435_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq62_e1435_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq62_e1435_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq62_e1435_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq62_e1435_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq62_e1435_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq62_e1435_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq62_e1435_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq62_e1435_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq62_e1435_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq62_e1435_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq62_e1435_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq62_e1435_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq62_e1435_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq62_e1435_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq62_e1435_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq62_e1435_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq62_e1435_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq62_e1435_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq62_e1435_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq62_e1435_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq62_e1435_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq62_e1435_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq62_e1435_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq62_e1435_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq62_e1435_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq62_e1435_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq62_e1435_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq62_e1435_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq62_e1435_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq62_e1435_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq62_e1435_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq62_e1435_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq62_e1435_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq62_e1435_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq62_e1435_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq62_e1435_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq62_e1435_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq62_e1437: f64 = (eq62_e1435 * p.p33);
        let eq62_e1437_d_n0: f64 = (eq62_e1435_d_n0 * p.p33);
        let eq62_e1437_d_n1: f64 = (eq62_e1435_d_n1 * p.p33);
        let eq62_e1437_d_n2: f64 = (eq62_e1435_d_n2 * p.p33);
        let eq62_e1437_d_n3: f64 = (eq62_e1435_d_n3 * p.p33);
        let eq62_e1437_d_n4: f64 = (eq62_e1435_d_n4 * p.p33);
        let eq62_e1437_d_n5: f64 = (eq62_e1435_d_n5 * p.p33);
        let eq62_e1437_d_n6: f64 = (eq62_e1435_d_n6 * p.p33);
        let eq62_e1437_d_n7: f64 = (eq62_e1435_d_n7 * p.p33);
        let eq62_e1437_d_n8: f64 = (eq62_e1435_d_n8 * p.p33);
        let eq62_e1437_d_n9: f64 = (eq62_e1435_d_n9 * p.p33);
        let eq62_e1437_d_n10: f64 = (eq62_e1435_d_n10 * p.p33);
        let eq62_e1437_d_n11: f64 = (eq62_e1435_d_n11 * p.p33);
        let eq62_e1437_d_n12: f64 = (eq62_e1435_d_n12 * p.p33);
        let eq62_e1437_d_n13: f64 = (eq62_e1435_d_n13 * p.p33);
        let eq62_e1437_d_n14: f64 = (eq62_e1435_d_n14 * p.p33);
        let eq62_e1437_d_n15: f64 = (eq62_e1435_d_n15 * p.p33);
        let eq62_e1437_d_n16: f64 = (eq62_e1435_d_n16 * p.p33);
        let eq62_e1437_d_n17: f64 = (eq62_e1435_d_n17 * p.p33);
        let eq62_e1437_d_n18: f64 = (eq62_e1435_d_n18 * p.p33);
        let eq62_e1437_d_n19: f64 = (eq62_e1435_d_n19 * p.p33);
        let eq62_e1437_d_n20: f64 = (eq62_e1435_d_n20 * p.p33);
        let eq62_e1437_d_b0: f64 = (eq62_e1435_d_b0 * p.p33);
        let eq62_e1437_d_b1: f64 = (eq62_e1435_d_b1 * p.p33);
        let eq62_e1437_d_b2: f64 = (eq62_e1435_d_b2 * p.p33);
        let eq62_e1437_d_b3: f64 = (eq62_e1435_d_b3 * p.p33);
        let eq62_e1437_d_b4: f64 = (eq62_e1435_d_b4 * p.p33);
        let eq62_e1437_d_b5: f64 = (eq62_e1435_d_b5 * p.p33);
        let eq62_e1437_d_b6: f64 = (eq62_e1435_d_b6 * p.p33);
        let eq62_e1437_d_b7: f64 = (eq62_e1435_d_b7 * p.p33);
        let eq62_e1437_d_b8: f64 = (eq62_e1435_d_b8 * p.p33);
        let eq62_e1437_d_b9: f64 = (eq62_e1435_d_b9 * p.p33);
        let eq62_e1437_d_b10: f64 = (eq62_e1435_d_b10 * p.p33);
        let eq62_e1437_d_b11: f64 = (eq62_e1435_d_b11 * p.p33);
        let eq62_e1437_d_b12: f64 = (eq62_e1435_d_b12 * p.p33);
        let eq62_e1437_d_b13: f64 = (eq62_e1435_d_b13 * p.p33);
        let eq62_e1437_d_b14: f64 = (eq62_e1435_d_b14 * p.p33);
        let eq62_e1437_d_b15: f64 = (eq62_e1435_d_b15 * p.p33);
        let eq62_e1437_d_b16: f64 = (eq62_e1435_d_b16 * p.p33);
        let eq62_e1437_d_b17: f64 = (eq62_e1435_d_b17 * p.p33);
        let eq62_e1437_d_b18: f64 = (eq62_e1435_d_b18 * p.p33);
        let eq62_e1437_d_b19: f64 = (eq62_e1435_d_b19 * p.p33);
        let eq62_e1437_d_b20: f64 = (eq62_e1435_d_b20 * p.p33);
        let eq62_e1437_d_b21: f64 = (eq62_e1435_d_b21 * p.p33);
        let eq62_e1437_d_b22: f64 = (eq62_e1435_d_b22 * p.p33);
        let eq62_e1437_d_b23: f64 = (eq62_e1435_d_b23 * p.p33);
        let eq62_e1437_d_b24: f64 = (eq62_e1435_d_b24 * p.p33);
        let eq62_e1439: f64 = (eq62_e1437 * s.v[857]);
        let eq62_e1439_d_n0: f64 = ((eq62_e1437_d_n0 * s.v[857]) + (eq62_e1437 * s.dn[857][0]));
        let eq62_e1439_d_n1: f64 = ((eq62_e1437_d_n1 * s.v[857]) + (eq62_e1437 * s.dn[857][1]));
        let eq62_e1439_d_n2: f64 = ((eq62_e1437_d_n2 * s.v[857]) + (eq62_e1437 * s.dn[857][2]));
        let eq62_e1439_d_n3: f64 = ((eq62_e1437_d_n3 * s.v[857]) + (eq62_e1437 * s.dn[857][3]));
        let eq62_e1439_d_n4: f64 = ((eq62_e1437_d_n4 * s.v[857]) + (eq62_e1437 * s.dn[857][4]));
        let eq62_e1439_d_n5: f64 = ((eq62_e1437_d_n5 * s.v[857]) + (eq62_e1437 * s.dn[857][5]));
        let eq62_e1439_d_n6: f64 = ((eq62_e1437_d_n6 * s.v[857]) + (eq62_e1437 * s.dn[857][6]));
        let eq62_e1439_d_n7: f64 = ((eq62_e1437_d_n7 * s.v[857]) + (eq62_e1437 * s.dn[857][7]));
        let eq62_e1439_d_n8: f64 = ((eq62_e1437_d_n8 * s.v[857]) + (eq62_e1437 * s.dn[857][8]));
        let eq62_e1439_d_n9: f64 = ((eq62_e1437_d_n9 * s.v[857]) + (eq62_e1437 * s.dn[857][9]));
        let eq62_e1439_d_n10: f64 = ((eq62_e1437_d_n10 * s.v[857]) + (eq62_e1437 * s.dn[857][10]));
        let eq62_e1439_d_n11: f64 = ((eq62_e1437_d_n11 * s.v[857]) + (eq62_e1437 * s.dn[857][11]));
        let eq62_e1439_d_n12: f64 = ((eq62_e1437_d_n12 * s.v[857]) + (eq62_e1437 * s.dn[857][12]));
        let eq62_e1439_d_n13: f64 = ((eq62_e1437_d_n13 * s.v[857]) + (eq62_e1437 * s.dn[857][13]));
        let eq62_e1439_d_n14: f64 = ((eq62_e1437_d_n14 * s.v[857]) + (eq62_e1437 * s.dn[857][14]));
        let eq62_e1439_d_n15: f64 = ((eq62_e1437_d_n15 * s.v[857]) + (eq62_e1437 * s.dn[857][15]));
        let eq62_e1439_d_n16: f64 = ((eq62_e1437_d_n16 * s.v[857]) + (eq62_e1437 * s.dn[857][16]));
        let eq62_e1439_d_n17: f64 = ((eq62_e1437_d_n17 * s.v[857]) + (eq62_e1437 * s.dn[857][17]));
        let eq62_e1439_d_n18: f64 = ((eq62_e1437_d_n18 * s.v[857]) + (eq62_e1437 * s.dn[857][18]));
        let eq62_e1439_d_n19: f64 = ((eq62_e1437_d_n19 * s.v[857]) + (eq62_e1437 * s.dn[857][19]));
        let eq62_e1439_d_n20: f64 = ((eq62_e1437_d_n20 * s.v[857]) + (eq62_e1437 * s.dn[857][20]));
        let eq62_e1439_d_b0: f64 = ((eq62_e1437_d_b0 * s.v[857]) + (eq62_e1437 * s.db[857][0]));
        let eq62_e1439_d_b1: f64 = ((eq62_e1437_d_b1 * s.v[857]) + (eq62_e1437 * s.db[857][1]));
        let eq62_e1439_d_b2: f64 = ((eq62_e1437_d_b2 * s.v[857]) + (eq62_e1437 * s.db[857][2]));
        let eq62_e1439_d_b3: f64 = ((eq62_e1437_d_b3 * s.v[857]) + (eq62_e1437 * s.db[857][3]));
        let eq62_e1439_d_b4: f64 = ((eq62_e1437_d_b4 * s.v[857]) + (eq62_e1437 * s.db[857][4]));
        let eq62_e1439_d_b5: f64 = ((eq62_e1437_d_b5 * s.v[857]) + (eq62_e1437 * s.db[857][5]));
        let eq62_e1439_d_b6: f64 = ((eq62_e1437_d_b6 * s.v[857]) + (eq62_e1437 * s.db[857][6]));
        let eq62_e1439_d_b7: f64 = ((eq62_e1437_d_b7 * s.v[857]) + (eq62_e1437 * s.db[857][7]));
        let eq62_e1439_d_b8: f64 = ((eq62_e1437_d_b8 * s.v[857]) + (eq62_e1437 * s.db[857][8]));
        let eq62_e1439_d_b9: f64 = ((eq62_e1437_d_b9 * s.v[857]) + (eq62_e1437 * s.db[857][9]));
        let eq62_e1439_d_b10: f64 = ((eq62_e1437_d_b10 * s.v[857]) + (eq62_e1437 * s.db[857][10]));
        let eq62_e1439_d_b11: f64 = ((eq62_e1437_d_b11 * s.v[857]) + (eq62_e1437 * s.db[857][11]));
        let eq62_e1439_d_b12: f64 = ((eq62_e1437_d_b12 * s.v[857]) + (eq62_e1437 * s.db[857][12]));
        let eq62_e1439_d_b13: f64 = ((eq62_e1437_d_b13 * s.v[857]) + (eq62_e1437 * s.db[857][13]));
        let eq62_e1439_d_b14: f64 = ((eq62_e1437_d_b14 * s.v[857]) + (eq62_e1437 * s.db[857][14]));
        let eq62_e1439_d_b15: f64 = ((eq62_e1437_d_b15 * s.v[857]) + (eq62_e1437 * s.db[857][15]));
        let eq62_e1439_d_b16: f64 = ((eq62_e1437_d_b16 * s.v[857]) + (eq62_e1437 * s.db[857][16]));
        let eq62_e1439_d_b17: f64 = ((eq62_e1437_d_b17 * s.v[857]) + (eq62_e1437 * s.db[857][17]));
        let eq62_e1439_d_b18: f64 = ((eq62_e1437_d_b18 * s.v[857]) + (eq62_e1437 * s.db[857][18]));
        let eq62_e1439_d_b19: f64 = ((eq62_e1437_d_b19 * s.v[857]) + (eq62_e1437 * s.db[857][19]));
        let eq62_e1439_d_b20: f64 = ((eq62_e1437_d_b20 * s.v[857]) + (eq62_e1437 * s.db[857][20]));
        let eq62_e1439_d_b21: f64 = ((eq62_e1437_d_b21 * s.v[857]) + (eq62_e1437 * s.db[857][21]));
        let eq62_e1439_d_b22: f64 = ((eq62_e1437_d_b22 * s.v[857]) + (eq62_e1437 * s.db[857][22]));
        let eq62_e1439_d_b23: f64 = ((eq62_e1437_d_b23 * s.v[857]) + (eq62_e1437 * s.db[857][23]));
        let eq62_e1439_d_b24: f64 = ((eq62_e1437_d_b24 * s.v[857]) + (eq62_e1437 * s.db[857][24]));
        let eq62_e1440: f64 = self.eval_ddt(6, eq62_e1439);
        let eq62_e1440_d_n0: f64 = self.ddt_jacobian(eq62_e1439_d_n0);
        let eq62_e1440_d_n1: f64 = self.ddt_jacobian(eq62_e1439_d_n1);
        let eq62_e1440_d_n2: f64 = self.ddt_jacobian(eq62_e1439_d_n2);
        let eq62_e1440_d_n3: f64 = self.ddt_jacobian(eq62_e1439_d_n3);
        let eq62_e1440_d_n4: f64 = self.ddt_jacobian(eq62_e1439_d_n4);
        let eq62_e1440_d_n5: f64 = self.ddt_jacobian(eq62_e1439_d_n5);
        let eq62_e1440_d_n6: f64 = self.ddt_jacobian(eq62_e1439_d_n6);
        let eq62_e1440_d_n7: f64 = self.ddt_jacobian(eq62_e1439_d_n7);
        let eq62_e1440_d_n8: f64 = self.ddt_jacobian(eq62_e1439_d_n8);
        let eq62_e1440_d_n9: f64 = self.ddt_jacobian(eq62_e1439_d_n9);
        let eq62_e1440_d_n10: f64 = self.ddt_jacobian(eq62_e1439_d_n10);
        let eq62_e1440_d_n11: f64 = self.ddt_jacobian(eq62_e1439_d_n11);
        let eq62_e1440_d_n12: f64 = self.ddt_jacobian(eq62_e1439_d_n12);
        let eq62_e1440_d_n13: f64 = self.ddt_jacobian(eq62_e1439_d_n13);
        let eq62_e1440_d_n14: f64 = self.ddt_jacobian(eq62_e1439_d_n14);
        let eq62_e1440_d_n15: f64 = self.ddt_jacobian(eq62_e1439_d_n15);
        let eq62_e1440_d_n16: f64 = self.ddt_jacobian(eq62_e1439_d_n16);
        let eq62_e1440_d_n17: f64 = self.ddt_jacobian(eq62_e1439_d_n17);
        let eq62_e1440_d_n18: f64 = self.ddt_jacobian(eq62_e1439_d_n18);
        let eq62_e1440_d_n19: f64 = self.ddt_jacobian(eq62_e1439_d_n19);
        let eq62_e1440_d_n20: f64 = self.ddt_jacobian(eq62_e1439_d_n20);
        let eq62_e1440_d_b0: f64 = self.ddt_jacobian(eq62_e1439_d_b0);
        let eq62_e1440_d_b1: f64 = self.ddt_jacobian(eq62_e1439_d_b1);
        let eq62_e1440_d_b2: f64 = self.ddt_jacobian(eq62_e1439_d_b2);
        let eq62_e1440_d_b3: f64 = self.ddt_jacobian(eq62_e1439_d_b3);
        let eq62_e1440_d_b4: f64 = self.ddt_jacobian(eq62_e1439_d_b4);
        let eq62_e1440_d_b5: f64 = self.ddt_jacobian(eq62_e1439_d_b5);
        let eq62_e1440_d_b6: f64 = self.ddt_jacobian(eq62_e1439_d_b6);
        let eq62_e1440_d_b7: f64 = self.ddt_jacobian(eq62_e1439_d_b7);
        let eq62_e1440_d_b8: f64 = self.ddt_jacobian(eq62_e1439_d_b8);
        let eq62_e1440_d_b9: f64 = self.ddt_jacobian(eq62_e1439_d_b9);
        let eq62_e1440_d_b10: f64 = self.ddt_jacobian(eq62_e1439_d_b10);
        let eq62_e1440_d_b11: f64 = self.ddt_jacobian(eq62_e1439_d_b11);
        let eq62_e1440_d_b12: f64 = self.ddt_jacobian(eq62_e1439_d_b12);
        let eq62_e1440_d_b13: f64 = self.ddt_jacobian(eq62_e1439_d_b13);
        let eq62_e1440_d_b14: f64 = self.ddt_jacobian(eq62_e1439_d_b14);
        let eq62_e1440_d_b15: f64 = self.ddt_jacobian(eq62_e1439_d_b15);
        let eq62_e1440_d_b16: f64 = self.ddt_jacobian(eq62_e1439_d_b16);
        let eq62_e1440_d_b17: f64 = self.ddt_jacobian(eq62_e1439_d_b17);
        let eq62_e1440_d_b18: f64 = self.ddt_jacobian(eq62_e1439_d_b18);
        let eq62_e1440_d_b19: f64 = self.ddt_jacobian(eq62_e1439_d_b19);
        let eq62_e1440_d_b20: f64 = self.ddt_jacobian(eq62_e1439_d_b20);
        let eq62_e1440_d_b21: f64 = self.ddt_jacobian(eq62_e1439_d_b21);
        let eq62_e1440_d_b22: f64 = self.ddt_jacobian(eq62_e1439_d_b22);
        let eq62_e1440_d_b23: f64 = self.ddt_jacobian(eq62_e1439_d_b23);
        let eq62_e1440_d_b24: f64 = self.ddt_jacobian(eq62_e1439_d_b24);
        let eq62_value: f64 = eq62_e1440;
        let eq62_node_derivatives: [f64; 21] = [eq62_e1440_d_n0, eq62_e1440_d_n1, eq62_e1440_d_n2, eq62_e1440_d_n3, eq62_e1440_d_n4, eq62_e1440_d_n5, eq62_e1440_d_n6, eq62_e1440_d_n7, eq62_e1440_d_n8, eq62_e1440_d_n9, eq62_e1440_d_n10, eq62_e1440_d_n11, eq62_e1440_d_n12, eq62_e1440_d_n13, eq62_e1440_d_n14, eq62_e1440_d_n15, eq62_e1440_d_n16, eq62_e1440_d_n17, eq62_e1440_d_n18, eq62_e1440_d_n19, eq62_e1440_d_n20];
        let eq62_branch_derivatives: [f64; 25] = [eq62_e1440_d_b0, eq62_e1440_d_b1, eq62_e1440_d_b2, eq62_e1440_d_b3, eq62_e1440_d_b4, eq62_e1440_d_b5, eq62_e1440_d_b6, eq62_e1440_d_b7, eq62_e1440_d_b8, eq62_e1440_d_b9, eq62_e1440_d_b10, eq62_e1440_d_b11, eq62_e1440_d_b12, eq62_e1440_d_b13, eq62_e1440_d_b14, eq62_e1440_d_b15, eq62_e1440_d_b16, eq62_e1440_d_b17, eq62_e1440_d_b18, eq62_e1440_d_b19, eq62_e1440_d_b20, eq62_e1440_d_b21, eq62_e1440_d_b22, eq62_e1440_d_b23, eq62_e1440_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[6]),
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
        let eq63_e1443: f64 = (s.v[0] * s.v[19]);
        let eq63_e1443_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq63_e1443_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq63_e1443_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq63_e1443_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq63_e1443_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq63_e1443_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq63_e1443_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq63_e1443_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq63_e1443_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq63_e1443_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq63_e1443_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq63_e1443_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq63_e1443_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq63_e1443_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq63_e1443_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq63_e1443_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq63_e1443_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq63_e1443_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq63_e1443_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq63_e1443_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq63_e1443_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq63_e1443_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq63_e1443_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq63_e1443_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq63_e1443_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq63_e1443_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq63_e1443_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq63_e1443_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq63_e1443_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq63_e1443_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq63_e1443_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq63_e1443_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq63_e1443_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq63_e1443_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq63_e1443_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq63_e1443_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq63_e1443_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq63_e1443_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq63_e1443_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq63_e1443_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq63_e1443_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq63_e1443_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq63_e1443_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq63_e1443_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq63_e1443_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq63_e1443_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq63_e1445: f64 = (eq63_e1443 * p.p33);
        let eq63_e1445_d_n0: f64 = (eq63_e1443_d_n0 * p.p33);
        let eq63_e1445_d_n1: f64 = (eq63_e1443_d_n1 * p.p33);
        let eq63_e1445_d_n2: f64 = (eq63_e1443_d_n2 * p.p33);
        let eq63_e1445_d_n3: f64 = (eq63_e1443_d_n3 * p.p33);
        let eq63_e1445_d_n4: f64 = (eq63_e1443_d_n4 * p.p33);
        let eq63_e1445_d_n5: f64 = (eq63_e1443_d_n5 * p.p33);
        let eq63_e1445_d_n6: f64 = (eq63_e1443_d_n6 * p.p33);
        let eq63_e1445_d_n7: f64 = (eq63_e1443_d_n7 * p.p33);
        let eq63_e1445_d_n8: f64 = (eq63_e1443_d_n8 * p.p33);
        let eq63_e1445_d_n9: f64 = (eq63_e1443_d_n9 * p.p33);
        let eq63_e1445_d_n10: f64 = (eq63_e1443_d_n10 * p.p33);
        let eq63_e1445_d_n11: f64 = (eq63_e1443_d_n11 * p.p33);
        let eq63_e1445_d_n12: f64 = (eq63_e1443_d_n12 * p.p33);
        let eq63_e1445_d_n13: f64 = (eq63_e1443_d_n13 * p.p33);
        let eq63_e1445_d_n14: f64 = (eq63_e1443_d_n14 * p.p33);
        let eq63_e1445_d_n15: f64 = (eq63_e1443_d_n15 * p.p33);
        let eq63_e1445_d_n16: f64 = (eq63_e1443_d_n16 * p.p33);
        let eq63_e1445_d_n17: f64 = (eq63_e1443_d_n17 * p.p33);
        let eq63_e1445_d_n18: f64 = (eq63_e1443_d_n18 * p.p33);
        let eq63_e1445_d_n19: f64 = (eq63_e1443_d_n19 * p.p33);
        let eq63_e1445_d_n20: f64 = (eq63_e1443_d_n20 * p.p33);
        let eq63_e1445_d_b0: f64 = (eq63_e1443_d_b0 * p.p33);
        let eq63_e1445_d_b1: f64 = (eq63_e1443_d_b1 * p.p33);
        let eq63_e1445_d_b2: f64 = (eq63_e1443_d_b2 * p.p33);
        let eq63_e1445_d_b3: f64 = (eq63_e1443_d_b3 * p.p33);
        let eq63_e1445_d_b4: f64 = (eq63_e1443_d_b4 * p.p33);
        let eq63_e1445_d_b5: f64 = (eq63_e1443_d_b5 * p.p33);
        let eq63_e1445_d_b6: f64 = (eq63_e1443_d_b6 * p.p33);
        let eq63_e1445_d_b7: f64 = (eq63_e1443_d_b7 * p.p33);
        let eq63_e1445_d_b8: f64 = (eq63_e1443_d_b8 * p.p33);
        let eq63_e1445_d_b9: f64 = (eq63_e1443_d_b9 * p.p33);
        let eq63_e1445_d_b10: f64 = (eq63_e1443_d_b10 * p.p33);
        let eq63_e1445_d_b11: f64 = (eq63_e1443_d_b11 * p.p33);
        let eq63_e1445_d_b12: f64 = (eq63_e1443_d_b12 * p.p33);
        let eq63_e1445_d_b13: f64 = (eq63_e1443_d_b13 * p.p33);
        let eq63_e1445_d_b14: f64 = (eq63_e1443_d_b14 * p.p33);
        let eq63_e1445_d_b15: f64 = (eq63_e1443_d_b15 * p.p33);
        let eq63_e1445_d_b16: f64 = (eq63_e1443_d_b16 * p.p33);
        let eq63_e1445_d_b17: f64 = (eq63_e1443_d_b17 * p.p33);
        let eq63_e1445_d_b18: f64 = (eq63_e1443_d_b18 * p.p33);
        let eq63_e1445_d_b19: f64 = (eq63_e1443_d_b19 * p.p33);
        let eq63_e1445_d_b20: f64 = (eq63_e1443_d_b20 * p.p33);
        let eq63_e1445_d_b21: f64 = (eq63_e1443_d_b21 * p.p33);
        let eq63_e1445_d_b22: f64 = (eq63_e1443_d_b22 * p.p33);
        let eq63_e1445_d_b23: f64 = (eq63_e1443_d_b23 * p.p33);
        let eq63_e1445_d_b24: f64 = (eq63_e1443_d_b24 * p.p33);
        let eq63_e1447: f64 = (eq63_e1445 * s.v[858]);
        let eq63_e1447_d_n0: f64 = ((eq63_e1445_d_n0 * s.v[858]) + (eq63_e1445 * s.dn[858][0]));
        let eq63_e1447_d_n1: f64 = ((eq63_e1445_d_n1 * s.v[858]) + (eq63_e1445 * s.dn[858][1]));
        let eq63_e1447_d_n2: f64 = ((eq63_e1445_d_n2 * s.v[858]) + (eq63_e1445 * s.dn[858][2]));
        let eq63_e1447_d_n3: f64 = ((eq63_e1445_d_n3 * s.v[858]) + (eq63_e1445 * s.dn[858][3]));
        let eq63_e1447_d_n4: f64 = ((eq63_e1445_d_n4 * s.v[858]) + (eq63_e1445 * s.dn[858][4]));
        let eq63_e1447_d_n5: f64 = ((eq63_e1445_d_n5 * s.v[858]) + (eq63_e1445 * s.dn[858][5]));
        let eq63_e1447_d_n6: f64 = ((eq63_e1445_d_n6 * s.v[858]) + (eq63_e1445 * s.dn[858][6]));
        let eq63_e1447_d_n7: f64 = ((eq63_e1445_d_n7 * s.v[858]) + (eq63_e1445 * s.dn[858][7]));
        let eq63_e1447_d_n8: f64 = ((eq63_e1445_d_n8 * s.v[858]) + (eq63_e1445 * s.dn[858][8]));
        let eq63_e1447_d_n9: f64 = ((eq63_e1445_d_n9 * s.v[858]) + (eq63_e1445 * s.dn[858][9]));
        let eq63_e1447_d_n10: f64 = ((eq63_e1445_d_n10 * s.v[858]) + (eq63_e1445 * s.dn[858][10]));
        let eq63_e1447_d_n11: f64 = ((eq63_e1445_d_n11 * s.v[858]) + (eq63_e1445 * s.dn[858][11]));
        let eq63_e1447_d_n12: f64 = ((eq63_e1445_d_n12 * s.v[858]) + (eq63_e1445 * s.dn[858][12]));
        let eq63_e1447_d_n13: f64 = ((eq63_e1445_d_n13 * s.v[858]) + (eq63_e1445 * s.dn[858][13]));
        let eq63_e1447_d_n14: f64 = ((eq63_e1445_d_n14 * s.v[858]) + (eq63_e1445 * s.dn[858][14]));
        let eq63_e1447_d_n15: f64 = ((eq63_e1445_d_n15 * s.v[858]) + (eq63_e1445 * s.dn[858][15]));
        let eq63_e1447_d_n16: f64 = ((eq63_e1445_d_n16 * s.v[858]) + (eq63_e1445 * s.dn[858][16]));
        let eq63_e1447_d_n17: f64 = ((eq63_e1445_d_n17 * s.v[858]) + (eq63_e1445 * s.dn[858][17]));
        let eq63_e1447_d_n18: f64 = ((eq63_e1445_d_n18 * s.v[858]) + (eq63_e1445 * s.dn[858][18]));
        let eq63_e1447_d_n19: f64 = ((eq63_e1445_d_n19 * s.v[858]) + (eq63_e1445 * s.dn[858][19]));
        let eq63_e1447_d_n20: f64 = ((eq63_e1445_d_n20 * s.v[858]) + (eq63_e1445 * s.dn[858][20]));
        let eq63_e1447_d_b0: f64 = ((eq63_e1445_d_b0 * s.v[858]) + (eq63_e1445 * s.db[858][0]));
        let eq63_e1447_d_b1: f64 = ((eq63_e1445_d_b1 * s.v[858]) + (eq63_e1445 * s.db[858][1]));
        let eq63_e1447_d_b2: f64 = ((eq63_e1445_d_b2 * s.v[858]) + (eq63_e1445 * s.db[858][2]));
        let eq63_e1447_d_b3: f64 = ((eq63_e1445_d_b3 * s.v[858]) + (eq63_e1445 * s.db[858][3]));
        let eq63_e1447_d_b4: f64 = ((eq63_e1445_d_b4 * s.v[858]) + (eq63_e1445 * s.db[858][4]));
        let eq63_e1447_d_b5: f64 = ((eq63_e1445_d_b5 * s.v[858]) + (eq63_e1445 * s.db[858][5]));
        let eq63_e1447_d_b6: f64 = ((eq63_e1445_d_b6 * s.v[858]) + (eq63_e1445 * s.db[858][6]));
        let eq63_e1447_d_b7: f64 = ((eq63_e1445_d_b7 * s.v[858]) + (eq63_e1445 * s.db[858][7]));
        let eq63_e1447_d_b8: f64 = ((eq63_e1445_d_b8 * s.v[858]) + (eq63_e1445 * s.db[858][8]));
        let eq63_e1447_d_b9: f64 = ((eq63_e1445_d_b9 * s.v[858]) + (eq63_e1445 * s.db[858][9]));
        let eq63_e1447_d_b10: f64 = ((eq63_e1445_d_b10 * s.v[858]) + (eq63_e1445 * s.db[858][10]));
        let eq63_e1447_d_b11: f64 = ((eq63_e1445_d_b11 * s.v[858]) + (eq63_e1445 * s.db[858][11]));
        let eq63_e1447_d_b12: f64 = ((eq63_e1445_d_b12 * s.v[858]) + (eq63_e1445 * s.db[858][12]));
        let eq63_e1447_d_b13: f64 = ((eq63_e1445_d_b13 * s.v[858]) + (eq63_e1445 * s.db[858][13]));
        let eq63_e1447_d_b14: f64 = ((eq63_e1445_d_b14 * s.v[858]) + (eq63_e1445 * s.db[858][14]));
        let eq63_e1447_d_b15: f64 = ((eq63_e1445_d_b15 * s.v[858]) + (eq63_e1445 * s.db[858][15]));
        let eq63_e1447_d_b16: f64 = ((eq63_e1445_d_b16 * s.v[858]) + (eq63_e1445 * s.db[858][16]));
        let eq63_e1447_d_b17: f64 = ((eq63_e1445_d_b17 * s.v[858]) + (eq63_e1445 * s.db[858][17]));
        let eq63_e1447_d_b18: f64 = ((eq63_e1445_d_b18 * s.v[858]) + (eq63_e1445 * s.db[858][18]));
        let eq63_e1447_d_b19: f64 = ((eq63_e1445_d_b19 * s.v[858]) + (eq63_e1445 * s.db[858][19]));
        let eq63_e1447_d_b20: f64 = ((eq63_e1445_d_b20 * s.v[858]) + (eq63_e1445 * s.db[858][20]));
        let eq63_e1447_d_b21: f64 = ((eq63_e1445_d_b21 * s.v[858]) + (eq63_e1445 * s.db[858][21]));
        let eq63_e1447_d_b22: f64 = ((eq63_e1445_d_b22 * s.v[858]) + (eq63_e1445 * s.db[858][22]));
        let eq63_e1447_d_b23: f64 = ((eq63_e1445_d_b23 * s.v[858]) + (eq63_e1445 * s.db[858][23]));
        let eq63_e1447_d_b24: f64 = ((eq63_e1445_d_b24 * s.v[858]) + (eq63_e1445 * s.db[858][24]));
        let eq63_e1448: f64 = self.eval_ddt(7, eq63_e1447);
        let eq63_e1448_d_n0: f64 = self.ddt_jacobian(eq63_e1447_d_n0);
        let eq63_e1448_d_n1: f64 = self.ddt_jacobian(eq63_e1447_d_n1);
        let eq63_e1448_d_n2: f64 = self.ddt_jacobian(eq63_e1447_d_n2);
        let eq63_e1448_d_n3: f64 = self.ddt_jacobian(eq63_e1447_d_n3);
        let eq63_e1448_d_n4: f64 = self.ddt_jacobian(eq63_e1447_d_n4);
        let eq63_e1448_d_n5: f64 = self.ddt_jacobian(eq63_e1447_d_n5);
        let eq63_e1448_d_n6: f64 = self.ddt_jacobian(eq63_e1447_d_n6);
        let eq63_e1448_d_n7: f64 = self.ddt_jacobian(eq63_e1447_d_n7);
        let eq63_e1448_d_n8: f64 = self.ddt_jacobian(eq63_e1447_d_n8);
        let eq63_e1448_d_n9: f64 = self.ddt_jacobian(eq63_e1447_d_n9);
        let eq63_e1448_d_n10: f64 = self.ddt_jacobian(eq63_e1447_d_n10);
        let eq63_e1448_d_n11: f64 = self.ddt_jacobian(eq63_e1447_d_n11);
        let eq63_e1448_d_n12: f64 = self.ddt_jacobian(eq63_e1447_d_n12);
        let eq63_e1448_d_n13: f64 = self.ddt_jacobian(eq63_e1447_d_n13);
        let eq63_e1448_d_n14: f64 = self.ddt_jacobian(eq63_e1447_d_n14);
        let eq63_e1448_d_n15: f64 = self.ddt_jacobian(eq63_e1447_d_n15);
        let eq63_e1448_d_n16: f64 = self.ddt_jacobian(eq63_e1447_d_n16);
        let eq63_e1448_d_n17: f64 = self.ddt_jacobian(eq63_e1447_d_n17);
        let eq63_e1448_d_n18: f64 = self.ddt_jacobian(eq63_e1447_d_n18);
        let eq63_e1448_d_n19: f64 = self.ddt_jacobian(eq63_e1447_d_n19);
        let eq63_e1448_d_n20: f64 = self.ddt_jacobian(eq63_e1447_d_n20);
        let eq63_e1448_d_b0: f64 = self.ddt_jacobian(eq63_e1447_d_b0);
        let eq63_e1448_d_b1: f64 = self.ddt_jacobian(eq63_e1447_d_b1);
        let eq63_e1448_d_b2: f64 = self.ddt_jacobian(eq63_e1447_d_b2);
        let eq63_e1448_d_b3: f64 = self.ddt_jacobian(eq63_e1447_d_b3);
        let eq63_e1448_d_b4: f64 = self.ddt_jacobian(eq63_e1447_d_b4);
        let eq63_e1448_d_b5: f64 = self.ddt_jacobian(eq63_e1447_d_b5);
        let eq63_e1448_d_b6: f64 = self.ddt_jacobian(eq63_e1447_d_b6);
        let eq63_e1448_d_b7: f64 = self.ddt_jacobian(eq63_e1447_d_b7);
        let eq63_e1448_d_b8: f64 = self.ddt_jacobian(eq63_e1447_d_b8);
        let eq63_e1448_d_b9: f64 = self.ddt_jacobian(eq63_e1447_d_b9);
        let eq63_e1448_d_b10: f64 = self.ddt_jacobian(eq63_e1447_d_b10);
        let eq63_e1448_d_b11: f64 = self.ddt_jacobian(eq63_e1447_d_b11);
        let eq63_e1448_d_b12: f64 = self.ddt_jacobian(eq63_e1447_d_b12);
        let eq63_e1448_d_b13: f64 = self.ddt_jacobian(eq63_e1447_d_b13);
        let eq63_e1448_d_b14: f64 = self.ddt_jacobian(eq63_e1447_d_b14);
        let eq63_e1448_d_b15: f64 = self.ddt_jacobian(eq63_e1447_d_b15);
        let eq63_e1448_d_b16: f64 = self.ddt_jacobian(eq63_e1447_d_b16);
        let eq63_e1448_d_b17: f64 = self.ddt_jacobian(eq63_e1447_d_b17);
        let eq63_e1448_d_b18: f64 = self.ddt_jacobian(eq63_e1447_d_b18);
        let eq63_e1448_d_b19: f64 = self.ddt_jacobian(eq63_e1447_d_b19);
        let eq63_e1448_d_b20: f64 = self.ddt_jacobian(eq63_e1447_d_b20);
        let eq63_e1448_d_b21: f64 = self.ddt_jacobian(eq63_e1447_d_b21);
        let eq63_e1448_d_b22: f64 = self.ddt_jacobian(eq63_e1447_d_b22);
        let eq63_e1448_d_b23: f64 = self.ddt_jacobian(eq63_e1447_d_b23);
        let eq63_e1448_d_b24: f64 = self.ddt_jacobian(eq63_e1447_d_b24);
        let eq63_value: f64 = eq63_e1448;
        let eq63_node_derivatives: [f64; 21] = [eq63_e1448_d_n0, eq63_e1448_d_n1, eq63_e1448_d_n2, eq63_e1448_d_n3, eq63_e1448_d_n4, eq63_e1448_d_n5, eq63_e1448_d_n6, eq63_e1448_d_n7, eq63_e1448_d_n8, eq63_e1448_d_n9, eq63_e1448_d_n10, eq63_e1448_d_n11, eq63_e1448_d_n12, eq63_e1448_d_n13, eq63_e1448_d_n14, eq63_e1448_d_n15, eq63_e1448_d_n16, eq63_e1448_d_n17, eq63_e1448_d_n18, eq63_e1448_d_n19, eq63_e1448_d_n20];
        let eq63_branch_derivatives: [f64; 25] = [eq63_e1448_d_b0, eq63_e1448_d_b1, eq63_e1448_d_b2, eq63_e1448_d_b3, eq63_e1448_d_b4, eq63_e1448_d_b5, eq63_e1448_d_b6, eq63_e1448_d_b7, eq63_e1448_d_b8, eq63_e1448_d_b9, eq63_e1448_d_b10, eq63_e1448_d_b11, eq63_e1448_d_b12, eq63_e1448_d_b13, eq63_e1448_d_b14, eq63_e1448_d_b15, eq63_e1448_d_b16, eq63_e1448_d_b17, eq63_e1448_d_b18, eq63_e1448_d_b19, eq63_e1448_d_b20, eq63_e1448_d_b21, eq63_e1448_d_b22, eq63_e1448_d_b23, eq63_e1448_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq63_value),
            &nodes,
            &eq63_node_derivatives,
            &branches,
            &eq63_branch_derivatives,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq65_e1456: f64 = ((nv4 - 0.0) / s.v[859]);
        let eq65_e1456_d_n0: f64 = (-(((nv4 - 0.0) * s.dn[859][0]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n1: f64 = (-(((nv4 - 0.0) * s.dn[859][1]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n2: f64 = (-(((nv4 - 0.0) * s.dn[859][2]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n3: f64 = (-(((nv4 - 0.0) * s.dn[859][3]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n4: f64 = ((s.v[859] - ((nv4 - 0.0) * s.dn[859][4])) / (s.v[859] * s.v[859]));
        let eq65_e1456_d_n5: f64 = (-(((nv4 - 0.0) * s.dn[859][5]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n6: f64 = (-(((nv4 - 0.0) * s.dn[859][6]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n7: f64 = (-(((nv4 - 0.0) * s.dn[859][7]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n8: f64 = (-(((nv4 - 0.0) * s.dn[859][8]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n9: f64 = (-(((nv4 - 0.0) * s.dn[859][9]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n10: f64 = (-(((nv4 - 0.0) * s.dn[859][10]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n11: f64 = (-(((nv4 - 0.0) * s.dn[859][11]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n12: f64 = (-(((nv4 - 0.0) * s.dn[859][12]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n13: f64 = (-(((nv4 - 0.0) * s.dn[859][13]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n14: f64 = (-(((nv4 - 0.0) * s.dn[859][14]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n15: f64 = (-(((nv4 - 0.0) * s.dn[859][15]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n16: f64 = (-(((nv4 - 0.0) * s.dn[859][16]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n17: f64 = (-(((nv4 - 0.0) * s.dn[859][17]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n18: f64 = (-(((nv4 - 0.0) * s.dn[859][18]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n19: f64 = (-(((nv4 - 0.0) * s.dn[859][19]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n20: f64 = (-(((nv4 - 0.0) * s.dn[859][20]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b0: f64 = (-(((nv4 - 0.0) * s.db[859][0]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b1: f64 = (-(((nv4 - 0.0) * s.db[859][1]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b2: f64 = (-(((nv4 - 0.0) * s.db[859][2]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b3: f64 = (-(((nv4 - 0.0) * s.db[859][3]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b4: f64 = (-(((nv4 - 0.0) * s.db[859][4]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b5: f64 = (-(((nv4 - 0.0) * s.db[859][5]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b6: f64 = (-(((nv4 - 0.0) * s.db[859][6]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b7: f64 = (-(((nv4 - 0.0) * s.db[859][7]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b8: f64 = (-(((nv4 - 0.0) * s.db[859][8]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b9: f64 = (-(((nv4 - 0.0) * s.db[859][9]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b10: f64 = (-(((nv4 - 0.0) * s.db[859][10]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b11: f64 = (-(((nv4 - 0.0) * s.db[859][11]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b12: f64 = (-(((nv4 - 0.0) * s.db[859][12]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b13: f64 = (-(((nv4 - 0.0) * s.db[859][13]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b14: f64 = (-(((nv4 - 0.0) * s.db[859][14]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b15: f64 = (-(((nv4 - 0.0) * s.db[859][15]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b16: f64 = (-(((nv4 - 0.0) * s.db[859][16]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b17: f64 = (-(((nv4 - 0.0) * s.db[859][17]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b18: f64 = (-(((nv4 - 0.0) * s.db[859][18]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b19: f64 = (-(((nv4 - 0.0) * s.db[859][19]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b20: f64 = (-(((nv4 - 0.0) * s.db[859][20]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b21: f64 = (-(((nv4 - 0.0) * s.db[859][21]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b22: f64 = (-(((nv4 - 0.0) * s.db[859][22]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b23: f64 = (-(((nv4 - 0.0) * s.db[859][23]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b24: f64 = (-(((nv4 - 0.0) * s.db[859][24]) / (s.v[859] * s.v[859])));
        let eq65_value: f64 = eq65_e1456;
        let eq65_node_derivatives: [f64; 21] = [eq65_e1456_d_n0, eq65_e1456_d_n1, eq65_e1456_d_n2, eq65_e1456_d_n3, eq65_e1456_d_n4, eq65_e1456_d_n5, eq65_e1456_d_n6, eq65_e1456_d_n7, eq65_e1456_d_n8, eq65_e1456_d_n9, eq65_e1456_d_n10, eq65_e1456_d_n11, eq65_e1456_d_n12, eq65_e1456_d_n13, eq65_e1456_d_n14, eq65_e1456_d_n15, eq65_e1456_d_n16, eq65_e1456_d_n17, eq65_e1456_d_n18, eq65_e1456_d_n19, eq65_e1456_d_n20];
        let eq65_branch_derivatives: [f64; 25] = [eq65_e1456_d_b0, eq65_e1456_d_b1, eq65_e1456_d_b2, eq65_e1456_d_b3, eq65_e1456_d_b4, eq65_e1456_d_b5, eq65_e1456_d_b6, eq65_e1456_d_b7, eq65_e1456_d_b8, eq65_e1456_d_b9, eq65_e1456_d_b10, eq65_e1456_d_b11, eq65_e1456_d_b12, eq65_e1456_d_b13, eq65_e1456_d_b14, eq65_e1456_d_b15, eq65_e1456_d_b16, eq65_e1456_d_b17, eq65_e1456_d_b18, eq65_e1456_d_b19, eq65_e1456_d_b20, eq65_e1456_d_b21, eq65_e1456_d_b22, eq65_e1456_d_b23, eq65_e1456_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq66_e1459: f64 = (s.v[860] * (nv4 - 0.0));
        let eq66_e1459_d_n0: f64 = (s.dn[860][0] * (nv4 - 0.0));
        let eq66_e1459_d_n1: f64 = (s.dn[860][1] * (nv4 - 0.0));
        let eq66_e1459_d_n2: f64 = (s.dn[860][2] * (nv4 - 0.0));
        let eq66_e1459_d_n3: f64 = (s.dn[860][3] * (nv4 - 0.0));
        let eq66_e1459_d_n4: f64 = ((s.dn[860][4] * (nv4 - 0.0)) + s.v[860]);
        let eq66_e1459_d_n5: f64 = (s.dn[860][5] * (nv4 - 0.0));
        let eq66_e1459_d_n6: f64 = (s.dn[860][6] * (nv4 - 0.0));
        let eq66_e1459_d_n7: f64 = (s.dn[860][7] * (nv4 - 0.0));
        let eq66_e1459_d_n8: f64 = (s.dn[860][8] * (nv4 - 0.0));
        let eq66_e1459_d_n9: f64 = (s.dn[860][9] * (nv4 - 0.0));
        let eq66_e1459_d_n10: f64 = (s.dn[860][10] * (nv4 - 0.0));
        let eq66_e1459_d_n11: f64 = (s.dn[860][11] * (nv4 - 0.0));
        let eq66_e1459_d_n12: f64 = (s.dn[860][12] * (nv4 - 0.0));
        let eq66_e1459_d_n13: f64 = (s.dn[860][13] * (nv4 - 0.0));
        let eq66_e1459_d_n14: f64 = (s.dn[860][14] * (nv4 - 0.0));
        let eq66_e1459_d_n15: f64 = (s.dn[860][15] * (nv4 - 0.0));
        let eq66_e1459_d_n16: f64 = (s.dn[860][16] * (nv4 - 0.0));
        let eq66_e1459_d_n17: f64 = (s.dn[860][17] * (nv4 - 0.0));
        let eq66_e1459_d_n18: f64 = (s.dn[860][18] * (nv4 - 0.0));
        let eq66_e1459_d_n19: f64 = (s.dn[860][19] * (nv4 - 0.0));
        let eq66_e1459_d_n20: f64 = (s.dn[860][20] * (nv4 - 0.0));
        let eq66_e1459_d_b0: f64 = (s.db[860][0] * (nv4 - 0.0));
        let eq66_e1459_d_b1: f64 = (s.db[860][1] * (nv4 - 0.0));
        let eq66_e1459_d_b2: f64 = (s.db[860][2] * (nv4 - 0.0));
        let eq66_e1459_d_b3: f64 = (s.db[860][3] * (nv4 - 0.0));
        let eq66_e1459_d_b4: f64 = (s.db[860][4] * (nv4 - 0.0));
        let eq66_e1459_d_b5: f64 = (s.db[860][5] * (nv4 - 0.0));
        let eq66_e1459_d_b6: f64 = (s.db[860][6] * (nv4 - 0.0));
        let eq66_e1459_d_b7: f64 = (s.db[860][7] * (nv4 - 0.0));
        let eq66_e1459_d_b8: f64 = (s.db[860][8] * (nv4 - 0.0));
        let eq66_e1459_d_b9: f64 = (s.db[860][9] * (nv4 - 0.0));
        let eq66_e1459_d_b10: f64 = (s.db[860][10] * (nv4 - 0.0));
        let eq66_e1459_d_b11: f64 = (s.db[860][11] * (nv4 - 0.0));
        let eq66_e1459_d_b12: f64 = (s.db[860][12] * (nv4 - 0.0));
        let eq66_e1459_d_b13: f64 = (s.db[860][13] * (nv4 - 0.0));
        let eq66_e1459_d_b14: f64 = (s.db[860][14] * (nv4 - 0.0));
        let eq66_e1459_d_b15: f64 = (s.db[860][15] * (nv4 - 0.0));
        let eq66_e1459_d_b16: f64 = (s.db[860][16] * (nv4 - 0.0));
        let eq66_e1459_d_b17: f64 = (s.db[860][17] * (nv4 - 0.0));
        let eq66_e1459_d_b18: f64 = (s.db[860][18] * (nv4 - 0.0));
        let eq66_e1459_d_b19: f64 = (s.db[860][19] * (nv4 - 0.0));
        let eq66_e1459_d_b20: f64 = (s.db[860][20] * (nv4 - 0.0));
        let eq66_e1459_d_b21: f64 = (s.db[860][21] * (nv4 - 0.0));
        let eq66_e1459_d_b22: f64 = (s.db[860][22] * (nv4 - 0.0));
        let eq66_e1459_d_b23: f64 = (s.db[860][23] * (nv4 - 0.0));
        let eq66_e1459_d_b24: f64 = (s.db[860][24] * (nv4 - 0.0));
        let eq66_e1460: f64 = self.eval_ddt(8, eq66_e1459);
        let eq66_e1460_d_n0: f64 = self.ddt_jacobian(eq66_e1459_d_n0);
        let eq66_e1460_d_n1: f64 = self.ddt_jacobian(eq66_e1459_d_n1);
        let eq66_e1460_d_n2: f64 = self.ddt_jacobian(eq66_e1459_d_n2);
        let eq66_e1460_d_n3: f64 = self.ddt_jacobian(eq66_e1459_d_n3);
        let eq66_e1460_d_n4: f64 = self.ddt_jacobian(eq66_e1459_d_n4);
        let eq66_e1460_d_n5: f64 = self.ddt_jacobian(eq66_e1459_d_n5);
        let eq66_e1460_d_n6: f64 = self.ddt_jacobian(eq66_e1459_d_n6);
        let eq66_e1460_d_n7: f64 = self.ddt_jacobian(eq66_e1459_d_n7);
        let eq66_e1460_d_n8: f64 = self.ddt_jacobian(eq66_e1459_d_n8);
        let eq66_e1460_d_n9: f64 = self.ddt_jacobian(eq66_e1459_d_n9);
        let eq66_e1460_d_n10: f64 = self.ddt_jacobian(eq66_e1459_d_n10);
        let eq66_e1460_d_n11: f64 = self.ddt_jacobian(eq66_e1459_d_n11);
        let eq66_e1460_d_n12: f64 = self.ddt_jacobian(eq66_e1459_d_n12);
        let eq66_e1460_d_n13: f64 = self.ddt_jacobian(eq66_e1459_d_n13);
        let eq66_e1460_d_n14: f64 = self.ddt_jacobian(eq66_e1459_d_n14);
        let eq66_e1460_d_n15: f64 = self.ddt_jacobian(eq66_e1459_d_n15);
        let eq66_e1460_d_n16: f64 = self.ddt_jacobian(eq66_e1459_d_n16);
        let eq66_e1460_d_n17: f64 = self.ddt_jacobian(eq66_e1459_d_n17);
        let eq66_e1460_d_n18: f64 = self.ddt_jacobian(eq66_e1459_d_n18);
        let eq66_e1460_d_n19: f64 = self.ddt_jacobian(eq66_e1459_d_n19);
        let eq66_e1460_d_n20: f64 = self.ddt_jacobian(eq66_e1459_d_n20);
        let eq66_e1460_d_b0: f64 = self.ddt_jacobian(eq66_e1459_d_b0);
        let eq66_e1460_d_b1: f64 = self.ddt_jacobian(eq66_e1459_d_b1);
        let eq66_e1460_d_b2: f64 = self.ddt_jacobian(eq66_e1459_d_b2);
        let eq66_e1460_d_b3: f64 = self.ddt_jacobian(eq66_e1459_d_b3);
        let eq66_e1460_d_b4: f64 = self.ddt_jacobian(eq66_e1459_d_b4);
        let eq66_e1460_d_b5: f64 = self.ddt_jacobian(eq66_e1459_d_b5);
        let eq66_e1460_d_b6: f64 = self.ddt_jacobian(eq66_e1459_d_b6);
        let eq66_e1460_d_b7: f64 = self.ddt_jacobian(eq66_e1459_d_b7);
        let eq66_e1460_d_b8: f64 = self.ddt_jacobian(eq66_e1459_d_b8);
        let eq66_e1460_d_b9: f64 = self.ddt_jacobian(eq66_e1459_d_b9);
        let eq66_e1460_d_b10: f64 = self.ddt_jacobian(eq66_e1459_d_b10);
        let eq66_e1460_d_b11: f64 = self.ddt_jacobian(eq66_e1459_d_b11);
        let eq66_e1460_d_b12: f64 = self.ddt_jacobian(eq66_e1459_d_b12);
        let eq66_e1460_d_b13: f64 = self.ddt_jacobian(eq66_e1459_d_b13);
        let eq66_e1460_d_b14: f64 = self.ddt_jacobian(eq66_e1459_d_b14);
        let eq66_e1460_d_b15: f64 = self.ddt_jacobian(eq66_e1459_d_b15);
        let eq66_e1460_d_b16: f64 = self.ddt_jacobian(eq66_e1459_d_b16);
        let eq66_e1460_d_b17: f64 = self.ddt_jacobian(eq66_e1459_d_b17);
        let eq66_e1460_d_b18: f64 = self.ddt_jacobian(eq66_e1459_d_b18);
        let eq66_e1460_d_b19: f64 = self.ddt_jacobian(eq66_e1459_d_b19);
        let eq66_e1460_d_b20: f64 = self.ddt_jacobian(eq66_e1459_d_b20);
        let eq66_e1460_d_b21: f64 = self.ddt_jacobian(eq66_e1459_d_b21);
        let eq66_e1460_d_b22: f64 = self.ddt_jacobian(eq66_e1459_d_b22);
        let eq66_e1460_d_b23: f64 = self.ddt_jacobian(eq66_e1459_d_b23);
        let eq66_e1460_d_b24: f64 = self.ddt_jacobian(eq66_e1459_d_b24);
        let eq66_value: f64 = eq66_e1460;
        let eq66_node_derivatives: [f64; 21] = [eq66_e1460_d_n0, eq66_e1460_d_n1, eq66_e1460_d_n2, eq66_e1460_d_n3, eq66_e1460_d_n4, eq66_e1460_d_n5, eq66_e1460_d_n6, eq66_e1460_d_n7, eq66_e1460_d_n8, eq66_e1460_d_n9, eq66_e1460_d_n10, eq66_e1460_d_n11, eq66_e1460_d_n12, eq66_e1460_d_n13, eq66_e1460_d_n14, eq66_e1460_d_n15, eq66_e1460_d_n16, eq66_e1460_d_n17, eq66_e1460_d_n18, eq66_e1460_d_n19, eq66_e1460_d_n20];
        let eq66_branch_derivatives: [f64; 25] = [eq66_e1460_d_b0, eq66_e1460_d_b1, eq66_e1460_d_b2, eq66_e1460_d_b3, eq66_e1460_d_b4, eq66_e1460_d_b5, eq66_e1460_d_b6, eq66_e1460_d_b7, eq66_e1460_d_b8, eq66_e1460_d_b9, eq66_e1460_d_b10, eq66_e1460_d_b11, eq66_e1460_d_b12, eq66_e1460_d_b13, eq66_e1460_d_b14, eq66_e1460_d_b15, eq66_e1460_d_b16, eq66_e1460_d_b17, eq66_e1460_d_b18, eq66_e1460_d_b19, eq66_e1460_d_b20, eq66_e1460_d_b21, eq66_e1460_d_b22, eq66_e1460_d_b23, eq66_e1460_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq67_e1463: f64 = (s.v[19] * p.p32);
        let eq67_e1463_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq67_e1463_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq67_e1463_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq67_e1463_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq67_e1463_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq67_e1463_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq67_e1463_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq67_e1463_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq67_e1463_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq67_e1463_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq67_e1463_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq67_e1463_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq67_e1463_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq67_e1463_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq67_e1463_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq67_e1463_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq67_e1463_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq67_e1463_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq67_e1463_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq67_e1463_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq67_e1463_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq67_e1463_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq67_e1463_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq67_e1463_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq67_e1463_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq67_e1463_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq67_e1463_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq67_e1463_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq67_e1463_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq67_e1463_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq67_e1463_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq67_e1463_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq67_e1463_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq67_e1463_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq67_e1463_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq67_e1463_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq67_e1463_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq67_e1463_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq67_e1463_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq67_e1463_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq67_e1463_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq67_e1463_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq67_e1463_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq67_e1463_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq67_e1463_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq67_e1463_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq67_e1464: f64 = (eq67_e1463).sqrt();
        let eq67_e1464_d_n0: f64 = (eq67_e1463_d_n0 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n1: f64 = (eq67_e1463_d_n1 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n2: f64 = (eq67_e1463_d_n2 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n3: f64 = (eq67_e1463_d_n3 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n4: f64 = (eq67_e1463_d_n4 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n5: f64 = (eq67_e1463_d_n5 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n6: f64 = (eq67_e1463_d_n6 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n7: f64 = (eq67_e1463_d_n7 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n8: f64 = (eq67_e1463_d_n8 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n9: f64 = (eq67_e1463_d_n9 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n10: f64 = (eq67_e1463_d_n10 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n11: f64 = (eq67_e1463_d_n11 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n12: f64 = (eq67_e1463_d_n12 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n13: f64 = (eq67_e1463_d_n13 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n14: f64 = (eq67_e1463_d_n14 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n15: f64 = (eq67_e1463_d_n15 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n16: f64 = (eq67_e1463_d_n16 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n17: f64 = (eq67_e1463_d_n17 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n18: f64 = (eq67_e1463_d_n18 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n19: f64 = (eq67_e1463_d_n19 / (2.0 * eq67_e1464));
        let eq67_e1464_d_n20: f64 = (eq67_e1463_d_n20 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b0: f64 = (eq67_e1463_d_b0 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b1: f64 = (eq67_e1463_d_b1 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b2: f64 = (eq67_e1463_d_b2 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b3: f64 = (eq67_e1463_d_b3 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b4: f64 = (eq67_e1463_d_b4 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b5: f64 = (eq67_e1463_d_b5 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b6: f64 = (eq67_e1463_d_b6 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b7: f64 = (eq67_e1463_d_b7 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b8: f64 = (eq67_e1463_d_b8 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b9: f64 = (eq67_e1463_d_b9 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b10: f64 = (eq67_e1463_d_b10 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b11: f64 = (eq67_e1463_d_b11 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b12: f64 = (eq67_e1463_d_b12 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b13: f64 = (eq67_e1463_d_b13 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b14: f64 = (eq67_e1463_d_b14 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b15: f64 = (eq67_e1463_d_b15 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b16: f64 = (eq67_e1463_d_b16 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b17: f64 = (eq67_e1463_d_b17 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b18: f64 = (eq67_e1463_d_b18 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b19: f64 = (eq67_e1463_d_b19 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b20: f64 = (eq67_e1463_d_b20 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b21: f64 = (eq67_e1463_d_b21 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b22: f64 = (eq67_e1463_d_b22 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b23: f64 = (eq67_e1463_d_b23 / (2.0 * eq67_e1464));
        let eq67_e1464_d_b24: f64 = (eq67_e1463_d_b24 / (2.0 * eq67_e1464));
        let eq67_e1466: f64 = (eq67_e1464 * 0.5);
        let eq67_e1466_d_n0: f64 = (eq67_e1464_d_n0 * 0.5);
        let eq67_e1466_d_n1: f64 = (eq67_e1464_d_n1 * 0.5);
        let eq67_e1466_d_n2: f64 = (eq67_e1464_d_n2 * 0.5);
        let eq67_e1466_d_n3: f64 = (eq67_e1464_d_n3 * 0.5);
        let eq67_e1466_d_n4: f64 = (eq67_e1464_d_n4 * 0.5);
        let eq67_e1466_d_n5: f64 = (eq67_e1464_d_n5 * 0.5);
        let eq67_e1466_d_n6: f64 = (eq67_e1464_d_n6 * 0.5);
        let eq67_e1466_d_n7: f64 = (eq67_e1464_d_n7 * 0.5);
        let eq67_e1466_d_n8: f64 = (eq67_e1464_d_n8 * 0.5);
        let eq67_e1466_d_n9: f64 = (eq67_e1464_d_n9 * 0.5);
        let eq67_e1466_d_n10: f64 = (eq67_e1464_d_n10 * 0.5);
        let eq67_e1466_d_n11: f64 = (eq67_e1464_d_n11 * 0.5);
        let eq67_e1466_d_n12: f64 = (eq67_e1464_d_n12 * 0.5);
        let eq67_e1466_d_n13: f64 = (eq67_e1464_d_n13 * 0.5);
        let eq67_e1466_d_n14: f64 = (eq67_e1464_d_n14 * 0.5);
        let eq67_e1466_d_n15: f64 = (eq67_e1464_d_n15 * 0.5);
        let eq67_e1466_d_n16: f64 = (eq67_e1464_d_n16 * 0.5);
        let eq67_e1466_d_n17: f64 = (eq67_e1464_d_n17 * 0.5);
        let eq67_e1466_d_n18: f64 = (eq67_e1464_d_n18 * 0.5);
        let eq67_e1466_d_n19: f64 = (eq67_e1464_d_n19 * 0.5);
        let eq67_e1466_d_n20: f64 = (eq67_e1464_d_n20 * 0.5);
        let eq67_e1466_d_b0: f64 = (eq67_e1464_d_b0 * 0.5);
        let eq67_e1466_d_b1: f64 = (eq67_e1464_d_b1 * 0.5);
        let eq67_e1466_d_b2: f64 = (eq67_e1464_d_b2 * 0.5);
        let eq67_e1466_d_b3: f64 = (eq67_e1464_d_b3 * 0.5);
        let eq67_e1466_d_b4: f64 = (eq67_e1464_d_b4 * 0.5);
        let eq67_e1466_d_b5: f64 = (eq67_e1464_d_b5 * 0.5);
        let eq67_e1466_d_b6: f64 = (eq67_e1464_d_b6 * 0.5);
        let eq67_e1466_d_b7: f64 = (eq67_e1464_d_b7 * 0.5);
        let eq67_e1466_d_b8: f64 = (eq67_e1464_d_b8 * 0.5);
        let eq67_e1466_d_b9: f64 = (eq67_e1464_d_b9 * 0.5);
        let eq67_e1466_d_b10: f64 = (eq67_e1464_d_b10 * 0.5);
        let eq67_e1466_d_b11: f64 = (eq67_e1464_d_b11 * 0.5);
        let eq67_e1466_d_b12: f64 = (eq67_e1464_d_b12 * 0.5);
        let eq67_e1466_d_b13: f64 = (eq67_e1464_d_b13 * 0.5);
        let eq67_e1466_d_b14: f64 = (eq67_e1464_d_b14 * 0.5);
        let eq67_e1466_d_b15: f64 = (eq67_e1464_d_b15 * 0.5);
        let eq67_e1466_d_b16: f64 = (eq67_e1464_d_b16 * 0.5);
        let eq67_e1466_d_b17: f64 = (eq67_e1464_d_b17 * 0.5);
        let eq67_e1466_d_b18: f64 = (eq67_e1464_d_b18 * 0.5);
        let eq67_e1466_d_b19: f64 = (eq67_e1464_d_b19 * 0.5);
        let eq67_e1466_d_b20: f64 = (eq67_e1464_d_b20 * 0.5);
        let eq67_e1466_d_b21: f64 = (eq67_e1464_d_b21 * 0.5);
        let eq67_e1466_d_b22: f64 = (eq67_e1464_d_b22 * 0.5);
        let eq67_e1466_d_b23: f64 = (eq67_e1464_d_b23 * 0.5);
        let eq67_e1466_d_b24: f64 = (eq67_e1464_d_b24 * 0.5);
        let eq67_e1468: f64 = (eq67_e1466 * s.v[860]);
        let eq67_e1468_d_n0: f64 = ((eq67_e1466_d_n0 * s.v[860]) + (eq67_e1466 * s.dn[860][0]));
        let eq67_e1468_d_n1: f64 = ((eq67_e1466_d_n1 * s.v[860]) + (eq67_e1466 * s.dn[860][1]));
        let eq67_e1468_d_n2: f64 = ((eq67_e1466_d_n2 * s.v[860]) + (eq67_e1466 * s.dn[860][2]));
        let eq67_e1468_d_n3: f64 = ((eq67_e1466_d_n3 * s.v[860]) + (eq67_e1466 * s.dn[860][3]));
        let eq67_e1468_d_n4: f64 = ((eq67_e1466_d_n4 * s.v[860]) + (eq67_e1466 * s.dn[860][4]));
        let eq67_e1468_d_n5: f64 = ((eq67_e1466_d_n5 * s.v[860]) + (eq67_e1466 * s.dn[860][5]));
        let eq67_e1468_d_n6: f64 = ((eq67_e1466_d_n6 * s.v[860]) + (eq67_e1466 * s.dn[860][6]));
        let eq67_e1468_d_n7: f64 = ((eq67_e1466_d_n7 * s.v[860]) + (eq67_e1466 * s.dn[860][7]));
        let eq67_e1468_d_n8: f64 = ((eq67_e1466_d_n8 * s.v[860]) + (eq67_e1466 * s.dn[860][8]));
        let eq67_e1468_d_n9: f64 = ((eq67_e1466_d_n9 * s.v[860]) + (eq67_e1466 * s.dn[860][9]));
        let eq67_e1468_d_n10: f64 = ((eq67_e1466_d_n10 * s.v[860]) + (eq67_e1466 * s.dn[860][10]));
        let eq67_e1468_d_n11: f64 = ((eq67_e1466_d_n11 * s.v[860]) + (eq67_e1466 * s.dn[860][11]));
        let eq67_e1468_d_n12: f64 = ((eq67_e1466_d_n12 * s.v[860]) + (eq67_e1466 * s.dn[860][12]));
        let eq67_e1468_d_n13: f64 = ((eq67_e1466_d_n13 * s.v[860]) + (eq67_e1466 * s.dn[860][13]));
        let eq67_e1468_d_n14: f64 = ((eq67_e1466_d_n14 * s.v[860]) + (eq67_e1466 * s.dn[860][14]));
        let eq67_e1468_d_n15: f64 = ((eq67_e1466_d_n15 * s.v[860]) + (eq67_e1466 * s.dn[860][15]));
        let eq67_e1468_d_n16: f64 = ((eq67_e1466_d_n16 * s.v[860]) + (eq67_e1466 * s.dn[860][16]));
        let eq67_e1468_d_n17: f64 = ((eq67_e1466_d_n17 * s.v[860]) + (eq67_e1466 * s.dn[860][17]));
        let eq67_e1468_d_n18: f64 = ((eq67_e1466_d_n18 * s.v[860]) + (eq67_e1466 * s.dn[860][18]));
        let eq67_e1468_d_n19: f64 = ((eq67_e1466_d_n19 * s.v[860]) + (eq67_e1466 * s.dn[860][19]));
        let eq67_e1468_d_n20: f64 = ((eq67_e1466_d_n20 * s.v[860]) + (eq67_e1466 * s.dn[860][20]));
        let eq67_e1468_d_b0: f64 = ((eq67_e1466_d_b0 * s.v[860]) + (eq67_e1466 * s.db[860][0]));
        let eq67_e1468_d_b1: f64 = ((eq67_e1466_d_b1 * s.v[860]) + (eq67_e1466 * s.db[860][1]));
        let eq67_e1468_d_b2: f64 = ((eq67_e1466_d_b2 * s.v[860]) + (eq67_e1466 * s.db[860][2]));
        let eq67_e1468_d_b3: f64 = ((eq67_e1466_d_b3 * s.v[860]) + (eq67_e1466 * s.db[860][3]));
        let eq67_e1468_d_b4: f64 = ((eq67_e1466_d_b4 * s.v[860]) + (eq67_e1466 * s.db[860][4]));
        let eq67_e1468_d_b5: f64 = ((eq67_e1466_d_b5 * s.v[860]) + (eq67_e1466 * s.db[860][5]));
        let eq67_e1468_d_b6: f64 = ((eq67_e1466_d_b6 * s.v[860]) + (eq67_e1466 * s.db[860][6]));
        let eq67_e1468_d_b7: f64 = ((eq67_e1466_d_b7 * s.v[860]) + (eq67_e1466 * s.db[860][7]));
        let eq67_e1468_d_b8: f64 = ((eq67_e1466_d_b8 * s.v[860]) + (eq67_e1466 * s.db[860][8]));
        let eq67_e1468_d_b9: f64 = ((eq67_e1466_d_b9 * s.v[860]) + (eq67_e1466 * s.db[860][9]));
        let eq67_e1468_d_b10: f64 = ((eq67_e1466_d_b10 * s.v[860]) + (eq67_e1466 * s.db[860][10]));
        let eq67_e1468_d_b11: f64 = ((eq67_e1466_d_b11 * s.v[860]) + (eq67_e1466 * s.db[860][11]));
        let eq67_e1468_d_b12: f64 = ((eq67_e1466_d_b12 * s.v[860]) + (eq67_e1466 * s.db[860][12]));
        let eq67_e1468_d_b13: f64 = ((eq67_e1466_d_b13 * s.v[860]) + (eq67_e1466 * s.db[860][13]));
        let eq67_e1468_d_b14: f64 = ((eq67_e1466_d_b14 * s.v[860]) + (eq67_e1466 * s.db[860][14]));
        let eq67_e1468_d_b15: f64 = ((eq67_e1466_d_b15 * s.v[860]) + (eq67_e1466 * s.db[860][15]));
        let eq67_e1468_d_b16: f64 = ((eq67_e1466_d_b16 * s.v[860]) + (eq67_e1466 * s.db[860][16]));
        let eq67_e1468_d_b17: f64 = ((eq67_e1466_d_b17 * s.v[860]) + (eq67_e1466 * s.db[860][17]));
        let eq67_e1468_d_b18: f64 = ((eq67_e1466_d_b18 * s.v[860]) + (eq67_e1466 * s.db[860][18]));
        let eq67_e1468_d_b19: f64 = ((eq67_e1466_d_b19 * s.v[860]) + (eq67_e1466 * s.db[860][19]));
        let eq67_e1468_d_b20: f64 = ((eq67_e1466_d_b20 * s.v[860]) + (eq67_e1466 * s.db[860][20]));
        let eq67_e1468_d_b21: f64 = ((eq67_e1466_d_b21 * s.v[860]) + (eq67_e1466 * s.db[860][21]));
        let eq67_e1468_d_b22: f64 = ((eq67_e1466_d_b22 * s.v[860]) + (eq67_e1466 * s.db[860][22]));
        let eq67_e1468_d_b23: f64 = ((eq67_e1466_d_b23 * s.v[860]) + (eq67_e1466 * s.db[860][23]));
        let eq67_e1468_d_b24: f64 = ((eq67_e1466_d_b24 * s.v[860]) + (eq67_e1466 * s.db[860][24]));
        let eq67_e1470: f64 = (eq67_e1468 * (nv4 - 0.0));
        let eq67_e1470_d_n0: f64 = (eq67_e1468_d_n0 * (nv4 - 0.0));
        let eq67_e1470_d_n1: f64 = (eq67_e1468_d_n1 * (nv4 - 0.0));
        let eq67_e1470_d_n2: f64 = (eq67_e1468_d_n2 * (nv4 - 0.0));
        let eq67_e1470_d_n3: f64 = (eq67_e1468_d_n3 * (nv4 - 0.0));
        let eq67_e1470_d_n4: f64 = ((eq67_e1468_d_n4 * (nv4 - 0.0)) + eq67_e1468);
        let eq67_e1470_d_n5: f64 = (eq67_e1468_d_n5 * (nv4 - 0.0));
        let eq67_e1470_d_n6: f64 = (eq67_e1468_d_n6 * (nv4 - 0.0));
        let eq67_e1470_d_n7: f64 = (eq67_e1468_d_n7 * (nv4 - 0.0));
        let eq67_e1470_d_n8: f64 = (eq67_e1468_d_n8 * (nv4 - 0.0));
        let eq67_e1470_d_n9: f64 = (eq67_e1468_d_n9 * (nv4 - 0.0));
        let eq67_e1470_d_n10: f64 = (eq67_e1468_d_n10 * (nv4 - 0.0));
        let eq67_e1470_d_n11: f64 = (eq67_e1468_d_n11 * (nv4 - 0.0));
        let eq67_e1470_d_n12: f64 = (eq67_e1468_d_n12 * (nv4 - 0.0));
        let eq67_e1470_d_n13: f64 = (eq67_e1468_d_n13 * (nv4 - 0.0));
        let eq67_e1470_d_n14: f64 = (eq67_e1468_d_n14 * (nv4 - 0.0));
        let eq67_e1470_d_n15: f64 = (eq67_e1468_d_n15 * (nv4 - 0.0));
        let eq67_e1470_d_n16: f64 = (eq67_e1468_d_n16 * (nv4 - 0.0));
        let eq67_e1470_d_n17: f64 = (eq67_e1468_d_n17 * (nv4 - 0.0));
        let eq67_e1470_d_n18: f64 = (eq67_e1468_d_n18 * (nv4 - 0.0));
        let eq67_e1470_d_n19: f64 = (eq67_e1468_d_n19 * (nv4 - 0.0));
        let eq67_e1470_d_n20: f64 = (eq67_e1468_d_n20 * (nv4 - 0.0));
        let eq67_e1470_d_b0: f64 = (eq67_e1468_d_b0 * (nv4 - 0.0));
        let eq67_e1470_d_b1: f64 = (eq67_e1468_d_b1 * (nv4 - 0.0));
        let eq67_e1470_d_b2: f64 = (eq67_e1468_d_b2 * (nv4 - 0.0));
        let eq67_e1470_d_b3: f64 = (eq67_e1468_d_b3 * (nv4 - 0.0));
        let eq67_e1470_d_b4: f64 = (eq67_e1468_d_b4 * (nv4 - 0.0));
        let eq67_e1470_d_b5: f64 = (eq67_e1468_d_b5 * (nv4 - 0.0));
        let eq67_e1470_d_b6: f64 = (eq67_e1468_d_b6 * (nv4 - 0.0));
        let eq67_e1470_d_b7: f64 = (eq67_e1468_d_b7 * (nv4 - 0.0));
        let eq67_e1470_d_b8: f64 = (eq67_e1468_d_b8 * (nv4 - 0.0));
        let eq67_e1470_d_b9: f64 = (eq67_e1468_d_b9 * (nv4 - 0.0));
        let eq67_e1470_d_b10: f64 = (eq67_e1468_d_b10 * (nv4 - 0.0));
        let eq67_e1470_d_b11: f64 = (eq67_e1468_d_b11 * (nv4 - 0.0));
        let eq67_e1470_d_b12: f64 = (eq67_e1468_d_b12 * (nv4 - 0.0));
        let eq67_e1470_d_b13: f64 = (eq67_e1468_d_b13 * (nv4 - 0.0));
        let eq67_e1470_d_b14: f64 = (eq67_e1468_d_b14 * (nv4 - 0.0));
        let eq67_e1470_d_b15: f64 = (eq67_e1468_d_b15 * (nv4 - 0.0));
        let eq67_e1470_d_b16: f64 = (eq67_e1468_d_b16 * (nv4 - 0.0));
        let eq67_e1470_d_b17: f64 = (eq67_e1468_d_b17 * (nv4 - 0.0));
        let eq67_e1470_d_b18: f64 = (eq67_e1468_d_b18 * (nv4 - 0.0));
        let eq67_e1470_d_b19: f64 = (eq67_e1468_d_b19 * (nv4 - 0.0));
        let eq67_e1470_d_b20: f64 = (eq67_e1468_d_b20 * (nv4 - 0.0));
        let eq67_e1470_d_b21: f64 = (eq67_e1468_d_b21 * (nv4 - 0.0));
        let eq67_e1470_d_b22: f64 = (eq67_e1468_d_b22 * (nv4 - 0.0));
        let eq67_e1470_d_b23: f64 = (eq67_e1468_d_b23 * (nv4 - 0.0));
        let eq67_e1470_d_b24: f64 = (eq67_e1468_d_b24 * (nv4 - 0.0));
        let eq67_e1471: f64 = self.eval_ddt(9, eq67_e1470);
        let eq67_e1471_d_n0: f64 = self.ddt_jacobian(eq67_e1470_d_n0);
        let eq67_e1471_d_n1: f64 = self.ddt_jacobian(eq67_e1470_d_n1);
        let eq67_e1471_d_n2: f64 = self.ddt_jacobian(eq67_e1470_d_n2);
        let eq67_e1471_d_n3: f64 = self.ddt_jacobian(eq67_e1470_d_n3);
        let eq67_e1471_d_n4: f64 = self.ddt_jacobian(eq67_e1470_d_n4);
        let eq67_e1471_d_n5: f64 = self.ddt_jacobian(eq67_e1470_d_n5);
        let eq67_e1471_d_n6: f64 = self.ddt_jacobian(eq67_e1470_d_n6);
        let eq67_e1471_d_n7: f64 = self.ddt_jacobian(eq67_e1470_d_n7);
        let eq67_e1471_d_n8: f64 = self.ddt_jacobian(eq67_e1470_d_n8);
        let eq67_e1471_d_n9: f64 = self.ddt_jacobian(eq67_e1470_d_n9);
        let eq67_e1471_d_n10: f64 = self.ddt_jacobian(eq67_e1470_d_n10);
        let eq67_e1471_d_n11: f64 = self.ddt_jacobian(eq67_e1470_d_n11);
        let eq67_e1471_d_n12: f64 = self.ddt_jacobian(eq67_e1470_d_n12);
        let eq67_e1471_d_n13: f64 = self.ddt_jacobian(eq67_e1470_d_n13);
        let eq67_e1471_d_n14: f64 = self.ddt_jacobian(eq67_e1470_d_n14);
        let eq67_e1471_d_n15: f64 = self.ddt_jacobian(eq67_e1470_d_n15);
        let eq67_e1471_d_n16: f64 = self.ddt_jacobian(eq67_e1470_d_n16);
        let eq67_e1471_d_n17: f64 = self.ddt_jacobian(eq67_e1470_d_n17);
        let eq67_e1471_d_n18: f64 = self.ddt_jacobian(eq67_e1470_d_n18);
        let eq67_e1471_d_n19: f64 = self.ddt_jacobian(eq67_e1470_d_n19);
        let eq67_e1471_d_n20: f64 = self.ddt_jacobian(eq67_e1470_d_n20);
        let eq67_e1471_d_b0: f64 = self.ddt_jacobian(eq67_e1470_d_b0);
        let eq67_e1471_d_b1: f64 = self.ddt_jacobian(eq67_e1470_d_b1);
        let eq67_e1471_d_b2: f64 = self.ddt_jacobian(eq67_e1470_d_b2);
        let eq67_e1471_d_b3: f64 = self.ddt_jacobian(eq67_e1470_d_b3);
        let eq67_e1471_d_b4: f64 = self.ddt_jacobian(eq67_e1470_d_b4);
        let eq67_e1471_d_b5: f64 = self.ddt_jacobian(eq67_e1470_d_b5);
        let eq67_e1471_d_b6: f64 = self.ddt_jacobian(eq67_e1470_d_b6);
        let eq67_e1471_d_b7: f64 = self.ddt_jacobian(eq67_e1470_d_b7);
        let eq67_e1471_d_b8: f64 = self.ddt_jacobian(eq67_e1470_d_b8);
        let eq67_e1471_d_b9: f64 = self.ddt_jacobian(eq67_e1470_d_b9);
        let eq67_e1471_d_b10: f64 = self.ddt_jacobian(eq67_e1470_d_b10);
        let eq67_e1471_d_b11: f64 = self.ddt_jacobian(eq67_e1470_d_b11);
        let eq67_e1471_d_b12: f64 = self.ddt_jacobian(eq67_e1470_d_b12);
        let eq67_e1471_d_b13: f64 = self.ddt_jacobian(eq67_e1470_d_b13);
        let eq67_e1471_d_b14: f64 = self.ddt_jacobian(eq67_e1470_d_b14);
        let eq67_e1471_d_b15: f64 = self.ddt_jacobian(eq67_e1470_d_b15);
        let eq67_e1471_d_b16: f64 = self.ddt_jacobian(eq67_e1470_d_b16);
        let eq67_e1471_d_b17: f64 = self.ddt_jacobian(eq67_e1470_d_b17);
        let eq67_e1471_d_b18: f64 = self.ddt_jacobian(eq67_e1470_d_b18);
        let eq67_e1471_d_b19: f64 = self.ddt_jacobian(eq67_e1470_d_b19);
        let eq67_e1471_d_b20: f64 = self.ddt_jacobian(eq67_e1470_d_b20);
        let eq67_e1471_d_b21: f64 = self.ddt_jacobian(eq67_e1470_d_b21);
        let eq67_e1471_d_b22: f64 = self.ddt_jacobian(eq67_e1470_d_b22);
        let eq67_e1471_d_b23: f64 = self.ddt_jacobian(eq67_e1470_d_b23);
        let eq67_e1471_d_b24: f64 = self.ddt_jacobian(eq67_e1470_d_b24);
        let eq67_e1472: f64 = (-eq67_e1471);
        let eq67_e1472_d_n0: f64 = (-eq67_e1471_d_n0);
        let eq67_e1472_d_n1: f64 = (-eq67_e1471_d_n1);
        let eq67_e1472_d_n2: f64 = (-eq67_e1471_d_n2);
        let eq67_e1472_d_n3: f64 = (-eq67_e1471_d_n3);
        let eq67_e1472_d_n4: f64 = (-eq67_e1471_d_n4);
        let eq67_e1472_d_n5: f64 = (-eq67_e1471_d_n5);
        let eq67_e1472_d_n6: f64 = (-eq67_e1471_d_n6);
        let eq67_e1472_d_n7: f64 = (-eq67_e1471_d_n7);
        let eq67_e1472_d_n8: f64 = (-eq67_e1471_d_n8);
        let eq67_e1472_d_n9: f64 = (-eq67_e1471_d_n9);
        let eq67_e1472_d_n10: f64 = (-eq67_e1471_d_n10);
        let eq67_e1472_d_n11: f64 = (-eq67_e1471_d_n11);
        let eq67_e1472_d_n12: f64 = (-eq67_e1471_d_n12);
        let eq67_e1472_d_n13: f64 = (-eq67_e1471_d_n13);
        let eq67_e1472_d_n14: f64 = (-eq67_e1471_d_n14);
        let eq67_e1472_d_n15: f64 = (-eq67_e1471_d_n15);
        let eq67_e1472_d_n16: f64 = (-eq67_e1471_d_n16);
        let eq67_e1472_d_n17: f64 = (-eq67_e1471_d_n17);
        let eq67_e1472_d_n18: f64 = (-eq67_e1471_d_n18);
        let eq67_e1472_d_n19: f64 = (-eq67_e1471_d_n19);
        let eq67_e1472_d_n20: f64 = (-eq67_e1471_d_n20);
        let eq67_e1472_d_b0: f64 = (-eq67_e1471_d_b0);
        let eq67_e1472_d_b1: f64 = (-eq67_e1471_d_b1);
        let eq67_e1472_d_b2: f64 = (-eq67_e1471_d_b2);
        let eq67_e1472_d_b3: f64 = (-eq67_e1471_d_b3);
        let eq67_e1472_d_b4: f64 = (-eq67_e1471_d_b4);
        let eq67_e1472_d_b5: f64 = (-eq67_e1471_d_b5);
        let eq67_e1472_d_b6: f64 = (-eq67_e1471_d_b6);
        let eq67_e1472_d_b7: f64 = (-eq67_e1471_d_b7);
        let eq67_e1472_d_b8: f64 = (-eq67_e1471_d_b8);
        let eq67_e1472_d_b9: f64 = (-eq67_e1471_d_b9);
        let eq67_e1472_d_b10: f64 = (-eq67_e1471_d_b10);
        let eq67_e1472_d_b11: f64 = (-eq67_e1471_d_b11);
        let eq67_e1472_d_b12: f64 = (-eq67_e1471_d_b12);
        let eq67_e1472_d_b13: f64 = (-eq67_e1471_d_b13);
        let eq67_e1472_d_b14: f64 = (-eq67_e1471_d_b14);
        let eq67_e1472_d_b15: f64 = (-eq67_e1471_d_b15);
        let eq67_e1472_d_b16: f64 = (-eq67_e1471_d_b16);
        let eq67_e1472_d_b17: f64 = (-eq67_e1471_d_b17);
        let eq67_e1472_d_b18: f64 = (-eq67_e1471_d_b18);
        let eq67_e1472_d_b19: f64 = (-eq67_e1471_d_b19);
        let eq67_e1472_d_b20: f64 = (-eq67_e1471_d_b20);
        let eq67_e1472_d_b21: f64 = (-eq67_e1471_d_b21);
        let eq67_e1472_d_b22: f64 = (-eq67_e1471_d_b22);
        let eq67_e1472_d_b23: f64 = (-eq67_e1471_d_b23);
        let eq67_e1472_d_b24: f64 = (-eq67_e1471_d_b24);
        let eq67_value: f64 = eq67_e1472;
        let eq67_node_derivatives: [f64; 21] = [eq67_e1472_d_n0, eq67_e1472_d_n1, eq67_e1472_d_n2, eq67_e1472_d_n3, eq67_e1472_d_n4, eq67_e1472_d_n5, eq67_e1472_d_n6, eq67_e1472_d_n7, eq67_e1472_d_n8, eq67_e1472_d_n9, eq67_e1472_d_n10, eq67_e1472_d_n11, eq67_e1472_d_n12, eq67_e1472_d_n13, eq67_e1472_d_n14, eq67_e1472_d_n15, eq67_e1472_d_n16, eq67_e1472_d_n17, eq67_e1472_d_n18, eq67_e1472_d_n19, eq67_e1472_d_n20];
        let eq67_branch_derivatives: [f64; 25] = [eq67_e1472_d_b0, eq67_e1472_d_b1, eq67_e1472_d_b2, eq67_e1472_d_b3, eq67_e1472_d_b4, eq67_e1472_d_b5, eq67_e1472_d_b6, eq67_e1472_d_b7, eq67_e1472_d_b8, eq67_e1472_d_b9, eq67_e1472_d_b10, eq67_e1472_d_b11, eq67_e1472_d_b12, eq67_e1472_d_b13, eq67_e1472_d_b14, eq67_e1472_d_b15, eq67_e1472_d_b16, eq67_e1472_d_b17, eq67_e1472_d_b18, eq67_e1472_d_b19, eq67_e1472_d_b20, eq67_e1472_d_b21, eq67_e1472_d_b22, eq67_e1472_d_b23, eq67_e1472_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq68_e1475: f64 = (s.v[19] * p.p32);
        let eq68_e1475_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq68_e1475_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq68_e1475_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq68_e1475_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq68_e1475_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq68_e1475_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq68_e1475_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq68_e1475_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq68_e1475_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq68_e1475_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq68_e1475_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq68_e1475_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq68_e1475_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq68_e1475_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq68_e1475_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq68_e1475_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq68_e1475_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq68_e1475_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq68_e1475_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq68_e1475_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq68_e1475_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq68_e1475_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq68_e1475_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq68_e1475_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq68_e1475_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq68_e1475_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq68_e1475_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq68_e1475_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq68_e1475_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq68_e1475_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq68_e1475_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq68_e1475_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq68_e1475_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq68_e1475_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq68_e1475_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq68_e1475_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq68_e1475_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq68_e1475_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq68_e1475_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq68_e1475_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq68_e1475_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq68_e1475_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq68_e1475_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq68_e1475_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq68_e1475_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq68_e1475_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq68_e1476: f64 = (eq68_e1475).sqrt();
        let eq68_e1476_d_n0: f64 = (eq68_e1475_d_n0 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n1: f64 = (eq68_e1475_d_n1 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n2: f64 = (eq68_e1475_d_n2 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n3: f64 = (eq68_e1475_d_n3 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n4: f64 = (eq68_e1475_d_n4 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n5: f64 = (eq68_e1475_d_n5 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n6: f64 = (eq68_e1475_d_n6 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n7: f64 = (eq68_e1475_d_n7 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n8: f64 = (eq68_e1475_d_n8 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n9: f64 = (eq68_e1475_d_n9 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n10: f64 = (eq68_e1475_d_n10 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n11: f64 = (eq68_e1475_d_n11 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n12: f64 = (eq68_e1475_d_n12 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n13: f64 = (eq68_e1475_d_n13 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n14: f64 = (eq68_e1475_d_n14 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n15: f64 = (eq68_e1475_d_n15 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n16: f64 = (eq68_e1475_d_n16 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n17: f64 = (eq68_e1475_d_n17 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n18: f64 = (eq68_e1475_d_n18 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n19: f64 = (eq68_e1475_d_n19 / (2.0 * eq68_e1476));
        let eq68_e1476_d_n20: f64 = (eq68_e1475_d_n20 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b0: f64 = (eq68_e1475_d_b0 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b1: f64 = (eq68_e1475_d_b1 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b2: f64 = (eq68_e1475_d_b2 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b3: f64 = (eq68_e1475_d_b3 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b4: f64 = (eq68_e1475_d_b4 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b5: f64 = (eq68_e1475_d_b5 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b6: f64 = (eq68_e1475_d_b6 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b7: f64 = (eq68_e1475_d_b7 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b8: f64 = (eq68_e1475_d_b8 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b9: f64 = (eq68_e1475_d_b9 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b10: f64 = (eq68_e1475_d_b10 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b11: f64 = (eq68_e1475_d_b11 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b12: f64 = (eq68_e1475_d_b12 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b13: f64 = (eq68_e1475_d_b13 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b14: f64 = (eq68_e1475_d_b14 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b15: f64 = (eq68_e1475_d_b15 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b16: f64 = (eq68_e1475_d_b16 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b17: f64 = (eq68_e1475_d_b17 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b18: f64 = (eq68_e1475_d_b18 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b19: f64 = (eq68_e1475_d_b19 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b20: f64 = (eq68_e1475_d_b20 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b21: f64 = (eq68_e1475_d_b21 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b22: f64 = (eq68_e1475_d_b22 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b23: f64 = (eq68_e1475_d_b23 / (2.0 * eq68_e1476));
        let eq68_e1476_d_b24: f64 = (eq68_e1475_d_b24 / (2.0 * eq68_e1476));
        let eq68_e1478: f64 = (eq68_e1476 * 0.5);
        let eq68_e1478_d_n0: f64 = (eq68_e1476_d_n0 * 0.5);
        let eq68_e1478_d_n1: f64 = (eq68_e1476_d_n1 * 0.5);
        let eq68_e1478_d_n2: f64 = (eq68_e1476_d_n2 * 0.5);
        let eq68_e1478_d_n3: f64 = (eq68_e1476_d_n3 * 0.5);
        let eq68_e1478_d_n4: f64 = (eq68_e1476_d_n4 * 0.5);
        let eq68_e1478_d_n5: f64 = (eq68_e1476_d_n5 * 0.5);
        let eq68_e1478_d_n6: f64 = (eq68_e1476_d_n6 * 0.5);
        let eq68_e1478_d_n7: f64 = (eq68_e1476_d_n7 * 0.5);
        let eq68_e1478_d_n8: f64 = (eq68_e1476_d_n8 * 0.5);
        let eq68_e1478_d_n9: f64 = (eq68_e1476_d_n9 * 0.5);
        let eq68_e1478_d_n10: f64 = (eq68_e1476_d_n10 * 0.5);
        let eq68_e1478_d_n11: f64 = (eq68_e1476_d_n11 * 0.5);
        let eq68_e1478_d_n12: f64 = (eq68_e1476_d_n12 * 0.5);
        let eq68_e1478_d_n13: f64 = (eq68_e1476_d_n13 * 0.5);
        let eq68_e1478_d_n14: f64 = (eq68_e1476_d_n14 * 0.5);
        let eq68_e1478_d_n15: f64 = (eq68_e1476_d_n15 * 0.5);
        let eq68_e1478_d_n16: f64 = (eq68_e1476_d_n16 * 0.5);
        let eq68_e1478_d_n17: f64 = (eq68_e1476_d_n17 * 0.5);
        let eq68_e1478_d_n18: f64 = (eq68_e1476_d_n18 * 0.5);
        let eq68_e1478_d_n19: f64 = (eq68_e1476_d_n19 * 0.5);
        let eq68_e1478_d_n20: f64 = (eq68_e1476_d_n20 * 0.5);
        let eq68_e1478_d_b0: f64 = (eq68_e1476_d_b0 * 0.5);
        let eq68_e1478_d_b1: f64 = (eq68_e1476_d_b1 * 0.5);
        let eq68_e1478_d_b2: f64 = (eq68_e1476_d_b2 * 0.5);
        let eq68_e1478_d_b3: f64 = (eq68_e1476_d_b3 * 0.5);
        let eq68_e1478_d_b4: f64 = (eq68_e1476_d_b4 * 0.5);
        let eq68_e1478_d_b5: f64 = (eq68_e1476_d_b5 * 0.5);
        let eq68_e1478_d_b6: f64 = (eq68_e1476_d_b6 * 0.5);
        let eq68_e1478_d_b7: f64 = (eq68_e1476_d_b7 * 0.5);
        let eq68_e1478_d_b8: f64 = (eq68_e1476_d_b8 * 0.5);
        let eq68_e1478_d_b9: f64 = (eq68_e1476_d_b9 * 0.5);
        let eq68_e1478_d_b10: f64 = (eq68_e1476_d_b10 * 0.5);
        let eq68_e1478_d_b11: f64 = (eq68_e1476_d_b11 * 0.5);
        let eq68_e1478_d_b12: f64 = (eq68_e1476_d_b12 * 0.5);
        let eq68_e1478_d_b13: f64 = (eq68_e1476_d_b13 * 0.5);
        let eq68_e1478_d_b14: f64 = (eq68_e1476_d_b14 * 0.5);
        let eq68_e1478_d_b15: f64 = (eq68_e1476_d_b15 * 0.5);
        let eq68_e1478_d_b16: f64 = (eq68_e1476_d_b16 * 0.5);
        let eq68_e1478_d_b17: f64 = (eq68_e1476_d_b17 * 0.5);
        let eq68_e1478_d_b18: f64 = (eq68_e1476_d_b18 * 0.5);
        let eq68_e1478_d_b19: f64 = (eq68_e1476_d_b19 * 0.5);
        let eq68_e1478_d_b20: f64 = (eq68_e1476_d_b20 * 0.5);
        let eq68_e1478_d_b21: f64 = (eq68_e1476_d_b21 * 0.5);
        let eq68_e1478_d_b22: f64 = (eq68_e1476_d_b22 * 0.5);
        let eq68_e1478_d_b23: f64 = (eq68_e1476_d_b23 * 0.5);
        let eq68_e1478_d_b24: f64 = (eq68_e1476_d_b24 * 0.5);
        let eq68_e1480: f64 = (eq68_e1478 * s.v[860]);
        let eq68_e1480_d_n0: f64 = ((eq68_e1478_d_n0 * s.v[860]) + (eq68_e1478 * s.dn[860][0]));
        let eq68_e1480_d_n1: f64 = ((eq68_e1478_d_n1 * s.v[860]) + (eq68_e1478 * s.dn[860][1]));
        let eq68_e1480_d_n2: f64 = ((eq68_e1478_d_n2 * s.v[860]) + (eq68_e1478 * s.dn[860][2]));
        let eq68_e1480_d_n3: f64 = ((eq68_e1478_d_n3 * s.v[860]) + (eq68_e1478 * s.dn[860][3]));
        let eq68_e1480_d_n4: f64 = ((eq68_e1478_d_n4 * s.v[860]) + (eq68_e1478 * s.dn[860][4]));
        let eq68_e1480_d_n5: f64 = ((eq68_e1478_d_n5 * s.v[860]) + (eq68_e1478 * s.dn[860][5]));
        let eq68_e1480_d_n6: f64 = ((eq68_e1478_d_n6 * s.v[860]) + (eq68_e1478 * s.dn[860][6]));
        let eq68_e1480_d_n7: f64 = ((eq68_e1478_d_n7 * s.v[860]) + (eq68_e1478 * s.dn[860][7]));
        let eq68_e1480_d_n8: f64 = ((eq68_e1478_d_n8 * s.v[860]) + (eq68_e1478 * s.dn[860][8]));
        let eq68_e1480_d_n9: f64 = ((eq68_e1478_d_n9 * s.v[860]) + (eq68_e1478 * s.dn[860][9]));
        let eq68_e1480_d_n10: f64 = ((eq68_e1478_d_n10 * s.v[860]) + (eq68_e1478 * s.dn[860][10]));
        let eq68_e1480_d_n11: f64 = ((eq68_e1478_d_n11 * s.v[860]) + (eq68_e1478 * s.dn[860][11]));
        let eq68_e1480_d_n12: f64 = ((eq68_e1478_d_n12 * s.v[860]) + (eq68_e1478 * s.dn[860][12]));
        let eq68_e1480_d_n13: f64 = ((eq68_e1478_d_n13 * s.v[860]) + (eq68_e1478 * s.dn[860][13]));
        let eq68_e1480_d_n14: f64 = ((eq68_e1478_d_n14 * s.v[860]) + (eq68_e1478 * s.dn[860][14]));
        let eq68_e1480_d_n15: f64 = ((eq68_e1478_d_n15 * s.v[860]) + (eq68_e1478 * s.dn[860][15]));
        let eq68_e1480_d_n16: f64 = ((eq68_e1478_d_n16 * s.v[860]) + (eq68_e1478 * s.dn[860][16]));
        let eq68_e1480_d_n17: f64 = ((eq68_e1478_d_n17 * s.v[860]) + (eq68_e1478 * s.dn[860][17]));
        let eq68_e1480_d_n18: f64 = ((eq68_e1478_d_n18 * s.v[860]) + (eq68_e1478 * s.dn[860][18]));
        let eq68_e1480_d_n19: f64 = ((eq68_e1478_d_n19 * s.v[860]) + (eq68_e1478 * s.dn[860][19]));
        let eq68_e1480_d_n20: f64 = ((eq68_e1478_d_n20 * s.v[860]) + (eq68_e1478 * s.dn[860][20]));
        let eq68_e1480_d_b0: f64 = ((eq68_e1478_d_b0 * s.v[860]) + (eq68_e1478 * s.db[860][0]));
        let eq68_e1480_d_b1: f64 = ((eq68_e1478_d_b1 * s.v[860]) + (eq68_e1478 * s.db[860][1]));
        let eq68_e1480_d_b2: f64 = ((eq68_e1478_d_b2 * s.v[860]) + (eq68_e1478 * s.db[860][2]));
        let eq68_e1480_d_b3: f64 = ((eq68_e1478_d_b3 * s.v[860]) + (eq68_e1478 * s.db[860][3]));
        let eq68_e1480_d_b4: f64 = ((eq68_e1478_d_b4 * s.v[860]) + (eq68_e1478 * s.db[860][4]));
        let eq68_e1480_d_b5: f64 = ((eq68_e1478_d_b5 * s.v[860]) + (eq68_e1478 * s.db[860][5]));
        let eq68_e1480_d_b6: f64 = ((eq68_e1478_d_b6 * s.v[860]) + (eq68_e1478 * s.db[860][6]));
        let eq68_e1480_d_b7: f64 = ((eq68_e1478_d_b7 * s.v[860]) + (eq68_e1478 * s.db[860][7]));
        let eq68_e1480_d_b8: f64 = ((eq68_e1478_d_b8 * s.v[860]) + (eq68_e1478 * s.db[860][8]));
        let eq68_e1480_d_b9: f64 = ((eq68_e1478_d_b9 * s.v[860]) + (eq68_e1478 * s.db[860][9]));
        let eq68_e1480_d_b10: f64 = ((eq68_e1478_d_b10 * s.v[860]) + (eq68_e1478 * s.db[860][10]));
        let eq68_e1480_d_b11: f64 = ((eq68_e1478_d_b11 * s.v[860]) + (eq68_e1478 * s.db[860][11]));
        let eq68_e1480_d_b12: f64 = ((eq68_e1478_d_b12 * s.v[860]) + (eq68_e1478 * s.db[860][12]));
        let eq68_e1480_d_b13: f64 = ((eq68_e1478_d_b13 * s.v[860]) + (eq68_e1478 * s.db[860][13]));
        let eq68_e1480_d_b14: f64 = ((eq68_e1478_d_b14 * s.v[860]) + (eq68_e1478 * s.db[860][14]));
        let eq68_e1480_d_b15: f64 = ((eq68_e1478_d_b15 * s.v[860]) + (eq68_e1478 * s.db[860][15]));
        let eq68_e1480_d_b16: f64 = ((eq68_e1478_d_b16 * s.v[860]) + (eq68_e1478 * s.db[860][16]));
        let eq68_e1480_d_b17: f64 = ((eq68_e1478_d_b17 * s.v[860]) + (eq68_e1478 * s.db[860][17]));
        let eq68_e1480_d_b18: f64 = ((eq68_e1478_d_b18 * s.v[860]) + (eq68_e1478 * s.db[860][18]));
        let eq68_e1480_d_b19: f64 = ((eq68_e1478_d_b19 * s.v[860]) + (eq68_e1478 * s.db[860][19]));
        let eq68_e1480_d_b20: f64 = ((eq68_e1478_d_b20 * s.v[860]) + (eq68_e1478 * s.db[860][20]));
        let eq68_e1480_d_b21: f64 = ((eq68_e1478_d_b21 * s.v[860]) + (eq68_e1478 * s.db[860][21]));
        let eq68_e1480_d_b22: f64 = ((eq68_e1478_d_b22 * s.v[860]) + (eq68_e1478 * s.db[860][22]));
        let eq68_e1480_d_b23: f64 = ((eq68_e1478_d_b23 * s.v[860]) + (eq68_e1478 * s.db[860][23]));
        let eq68_e1480_d_b24: f64 = ((eq68_e1478_d_b24 * s.v[860]) + (eq68_e1478 * s.db[860][24]));
        let eq68_e1482: f64 = (eq68_e1480 * (nv4 - 0.0));
        let eq68_e1482_d_n0: f64 = (eq68_e1480_d_n0 * (nv4 - 0.0));
        let eq68_e1482_d_n1: f64 = (eq68_e1480_d_n1 * (nv4 - 0.0));
        let eq68_e1482_d_n2: f64 = (eq68_e1480_d_n2 * (nv4 - 0.0));
        let eq68_e1482_d_n3: f64 = (eq68_e1480_d_n3 * (nv4 - 0.0));
        let eq68_e1482_d_n4: f64 = ((eq68_e1480_d_n4 * (nv4 - 0.0)) + eq68_e1480);
        let eq68_e1482_d_n5: f64 = (eq68_e1480_d_n5 * (nv4 - 0.0));
        let eq68_e1482_d_n6: f64 = (eq68_e1480_d_n6 * (nv4 - 0.0));
        let eq68_e1482_d_n7: f64 = (eq68_e1480_d_n7 * (nv4 - 0.0));
        let eq68_e1482_d_n8: f64 = (eq68_e1480_d_n8 * (nv4 - 0.0));
        let eq68_e1482_d_n9: f64 = (eq68_e1480_d_n9 * (nv4 - 0.0));
        let eq68_e1482_d_n10: f64 = (eq68_e1480_d_n10 * (nv4 - 0.0));
        let eq68_e1482_d_n11: f64 = (eq68_e1480_d_n11 * (nv4 - 0.0));
        let eq68_e1482_d_n12: f64 = (eq68_e1480_d_n12 * (nv4 - 0.0));
        let eq68_e1482_d_n13: f64 = (eq68_e1480_d_n13 * (nv4 - 0.0));
        let eq68_e1482_d_n14: f64 = (eq68_e1480_d_n14 * (nv4 - 0.0));
        let eq68_e1482_d_n15: f64 = (eq68_e1480_d_n15 * (nv4 - 0.0));
        let eq68_e1482_d_n16: f64 = (eq68_e1480_d_n16 * (nv4 - 0.0));
        let eq68_e1482_d_n17: f64 = (eq68_e1480_d_n17 * (nv4 - 0.0));
        let eq68_e1482_d_n18: f64 = (eq68_e1480_d_n18 * (nv4 - 0.0));
        let eq68_e1482_d_n19: f64 = (eq68_e1480_d_n19 * (nv4 - 0.0));
        let eq68_e1482_d_n20: f64 = (eq68_e1480_d_n20 * (nv4 - 0.0));
        let eq68_e1482_d_b0: f64 = (eq68_e1480_d_b0 * (nv4 - 0.0));
        let eq68_e1482_d_b1: f64 = (eq68_e1480_d_b1 * (nv4 - 0.0));
        let eq68_e1482_d_b2: f64 = (eq68_e1480_d_b2 * (nv4 - 0.0));
        let eq68_e1482_d_b3: f64 = (eq68_e1480_d_b3 * (nv4 - 0.0));
        let eq68_e1482_d_b4: f64 = (eq68_e1480_d_b4 * (nv4 - 0.0));
        let eq68_e1482_d_b5: f64 = (eq68_e1480_d_b5 * (nv4 - 0.0));
        let eq68_e1482_d_b6: f64 = (eq68_e1480_d_b6 * (nv4 - 0.0));
        let eq68_e1482_d_b7: f64 = (eq68_e1480_d_b7 * (nv4 - 0.0));
        let eq68_e1482_d_b8: f64 = (eq68_e1480_d_b8 * (nv4 - 0.0));
        let eq68_e1482_d_b9: f64 = (eq68_e1480_d_b9 * (nv4 - 0.0));
        let eq68_e1482_d_b10: f64 = (eq68_e1480_d_b10 * (nv4 - 0.0));
        let eq68_e1482_d_b11: f64 = (eq68_e1480_d_b11 * (nv4 - 0.0));
        let eq68_e1482_d_b12: f64 = (eq68_e1480_d_b12 * (nv4 - 0.0));
        let eq68_e1482_d_b13: f64 = (eq68_e1480_d_b13 * (nv4 - 0.0));
        let eq68_e1482_d_b14: f64 = (eq68_e1480_d_b14 * (nv4 - 0.0));
        let eq68_e1482_d_b15: f64 = (eq68_e1480_d_b15 * (nv4 - 0.0));
        let eq68_e1482_d_b16: f64 = (eq68_e1480_d_b16 * (nv4 - 0.0));
        let eq68_e1482_d_b17: f64 = (eq68_e1480_d_b17 * (nv4 - 0.0));
        let eq68_e1482_d_b18: f64 = (eq68_e1480_d_b18 * (nv4 - 0.0));
        let eq68_e1482_d_b19: f64 = (eq68_e1480_d_b19 * (nv4 - 0.0));
        let eq68_e1482_d_b20: f64 = (eq68_e1480_d_b20 * (nv4 - 0.0));
        let eq68_e1482_d_b21: f64 = (eq68_e1480_d_b21 * (nv4 - 0.0));
        let eq68_e1482_d_b22: f64 = (eq68_e1480_d_b22 * (nv4 - 0.0));
        let eq68_e1482_d_b23: f64 = (eq68_e1480_d_b23 * (nv4 - 0.0));
        let eq68_e1482_d_b24: f64 = (eq68_e1480_d_b24 * (nv4 - 0.0));
        let eq68_e1483: f64 = self.eval_ddt(10, eq68_e1482);
        let eq68_e1483_d_n0: f64 = self.ddt_jacobian(eq68_e1482_d_n0);
        let eq68_e1483_d_n1: f64 = self.ddt_jacobian(eq68_e1482_d_n1);
        let eq68_e1483_d_n2: f64 = self.ddt_jacobian(eq68_e1482_d_n2);
        let eq68_e1483_d_n3: f64 = self.ddt_jacobian(eq68_e1482_d_n3);
        let eq68_e1483_d_n4: f64 = self.ddt_jacobian(eq68_e1482_d_n4);
        let eq68_e1483_d_n5: f64 = self.ddt_jacobian(eq68_e1482_d_n5);
        let eq68_e1483_d_n6: f64 = self.ddt_jacobian(eq68_e1482_d_n6);
        let eq68_e1483_d_n7: f64 = self.ddt_jacobian(eq68_e1482_d_n7);
        let eq68_e1483_d_n8: f64 = self.ddt_jacobian(eq68_e1482_d_n8);
        let eq68_e1483_d_n9: f64 = self.ddt_jacobian(eq68_e1482_d_n9);
        let eq68_e1483_d_n10: f64 = self.ddt_jacobian(eq68_e1482_d_n10);
        let eq68_e1483_d_n11: f64 = self.ddt_jacobian(eq68_e1482_d_n11);
        let eq68_e1483_d_n12: f64 = self.ddt_jacobian(eq68_e1482_d_n12);
        let eq68_e1483_d_n13: f64 = self.ddt_jacobian(eq68_e1482_d_n13);
        let eq68_e1483_d_n14: f64 = self.ddt_jacobian(eq68_e1482_d_n14);
        let eq68_e1483_d_n15: f64 = self.ddt_jacobian(eq68_e1482_d_n15);
        let eq68_e1483_d_n16: f64 = self.ddt_jacobian(eq68_e1482_d_n16);
        let eq68_e1483_d_n17: f64 = self.ddt_jacobian(eq68_e1482_d_n17);
        let eq68_e1483_d_n18: f64 = self.ddt_jacobian(eq68_e1482_d_n18);
        let eq68_e1483_d_n19: f64 = self.ddt_jacobian(eq68_e1482_d_n19);
        let eq68_e1483_d_n20: f64 = self.ddt_jacobian(eq68_e1482_d_n20);
        let eq68_e1483_d_b0: f64 = self.ddt_jacobian(eq68_e1482_d_b0);
        let eq68_e1483_d_b1: f64 = self.ddt_jacobian(eq68_e1482_d_b1);
        let eq68_e1483_d_b2: f64 = self.ddt_jacobian(eq68_e1482_d_b2);
        let eq68_e1483_d_b3: f64 = self.ddt_jacobian(eq68_e1482_d_b3);
        let eq68_e1483_d_b4: f64 = self.ddt_jacobian(eq68_e1482_d_b4);
        let eq68_e1483_d_b5: f64 = self.ddt_jacobian(eq68_e1482_d_b5);
        let eq68_e1483_d_b6: f64 = self.ddt_jacobian(eq68_e1482_d_b6);
        let eq68_e1483_d_b7: f64 = self.ddt_jacobian(eq68_e1482_d_b7);
        let eq68_e1483_d_b8: f64 = self.ddt_jacobian(eq68_e1482_d_b8);
        let eq68_e1483_d_b9: f64 = self.ddt_jacobian(eq68_e1482_d_b9);
        let eq68_e1483_d_b10: f64 = self.ddt_jacobian(eq68_e1482_d_b10);
        let eq68_e1483_d_b11: f64 = self.ddt_jacobian(eq68_e1482_d_b11);
        let eq68_e1483_d_b12: f64 = self.ddt_jacobian(eq68_e1482_d_b12);
        let eq68_e1483_d_b13: f64 = self.ddt_jacobian(eq68_e1482_d_b13);
        let eq68_e1483_d_b14: f64 = self.ddt_jacobian(eq68_e1482_d_b14);
        let eq68_e1483_d_b15: f64 = self.ddt_jacobian(eq68_e1482_d_b15);
        let eq68_e1483_d_b16: f64 = self.ddt_jacobian(eq68_e1482_d_b16);
        let eq68_e1483_d_b17: f64 = self.ddt_jacobian(eq68_e1482_d_b17);
        let eq68_e1483_d_b18: f64 = self.ddt_jacobian(eq68_e1482_d_b18);
        let eq68_e1483_d_b19: f64 = self.ddt_jacobian(eq68_e1482_d_b19);
        let eq68_e1483_d_b20: f64 = self.ddt_jacobian(eq68_e1482_d_b20);
        let eq68_e1483_d_b21: f64 = self.ddt_jacobian(eq68_e1482_d_b21);
        let eq68_e1483_d_b22: f64 = self.ddt_jacobian(eq68_e1482_d_b22);
        let eq68_e1483_d_b23: f64 = self.ddt_jacobian(eq68_e1482_d_b23);
        let eq68_e1483_d_b24: f64 = self.ddt_jacobian(eq68_e1482_d_b24);
        let eq68_e1484: f64 = (-eq68_e1483);
        let eq68_e1484_d_n0: f64 = (-eq68_e1483_d_n0);
        let eq68_e1484_d_n1: f64 = (-eq68_e1483_d_n1);
        let eq68_e1484_d_n2: f64 = (-eq68_e1483_d_n2);
        let eq68_e1484_d_n3: f64 = (-eq68_e1483_d_n3);
        let eq68_e1484_d_n4: f64 = (-eq68_e1483_d_n4);
        let eq68_e1484_d_n5: f64 = (-eq68_e1483_d_n5);
        let eq68_e1484_d_n6: f64 = (-eq68_e1483_d_n6);
        let eq68_e1484_d_n7: f64 = (-eq68_e1483_d_n7);
        let eq68_e1484_d_n8: f64 = (-eq68_e1483_d_n8);
        let eq68_e1484_d_n9: f64 = (-eq68_e1483_d_n9);
        let eq68_e1484_d_n10: f64 = (-eq68_e1483_d_n10);
        let eq68_e1484_d_n11: f64 = (-eq68_e1483_d_n11);
        let eq68_e1484_d_n12: f64 = (-eq68_e1483_d_n12);
        let eq68_e1484_d_n13: f64 = (-eq68_e1483_d_n13);
        let eq68_e1484_d_n14: f64 = (-eq68_e1483_d_n14);
        let eq68_e1484_d_n15: f64 = (-eq68_e1483_d_n15);
        let eq68_e1484_d_n16: f64 = (-eq68_e1483_d_n16);
        let eq68_e1484_d_n17: f64 = (-eq68_e1483_d_n17);
        let eq68_e1484_d_n18: f64 = (-eq68_e1483_d_n18);
        let eq68_e1484_d_n19: f64 = (-eq68_e1483_d_n19);
        let eq68_e1484_d_n20: f64 = (-eq68_e1483_d_n20);
        let eq68_e1484_d_b0: f64 = (-eq68_e1483_d_b0);
        let eq68_e1484_d_b1: f64 = (-eq68_e1483_d_b1);
        let eq68_e1484_d_b2: f64 = (-eq68_e1483_d_b2);
        let eq68_e1484_d_b3: f64 = (-eq68_e1483_d_b3);
        let eq68_e1484_d_b4: f64 = (-eq68_e1483_d_b4);
        let eq68_e1484_d_b5: f64 = (-eq68_e1483_d_b5);
        let eq68_e1484_d_b6: f64 = (-eq68_e1483_d_b6);
        let eq68_e1484_d_b7: f64 = (-eq68_e1483_d_b7);
        let eq68_e1484_d_b8: f64 = (-eq68_e1483_d_b8);
        let eq68_e1484_d_b9: f64 = (-eq68_e1483_d_b9);
        let eq68_e1484_d_b10: f64 = (-eq68_e1483_d_b10);
        let eq68_e1484_d_b11: f64 = (-eq68_e1483_d_b11);
        let eq68_e1484_d_b12: f64 = (-eq68_e1483_d_b12);
        let eq68_e1484_d_b13: f64 = (-eq68_e1483_d_b13);
        let eq68_e1484_d_b14: f64 = (-eq68_e1483_d_b14);
        let eq68_e1484_d_b15: f64 = (-eq68_e1483_d_b15);
        let eq68_e1484_d_b16: f64 = (-eq68_e1483_d_b16);
        let eq68_e1484_d_b17: f64 = (-eq68_e1483_d_b17);
        let eq68_e1484_d_b18: f64 = (-eq68_e1483_d_b18);
        let eq68_e1484_d_b19: f64 = (-eq68_e1483_d_b19);
        let eq68_e1484_d_b20: f64 = (-eq68_e1483_d_b20);
        let eq68_e1484_d_b21: f64 = (-eq68_e1483_d_b21);
        let eq68_e1484_d_b22: f64 = (-eq68_e1483_d_b22);
        let eq68_e1484_d_b23: f64 = (-eq68_e1483_d_b23);
        let eq68_e1484_d_b24: f64 = (-eq68_e1483_d_b24);
        let eq68_value: f64 = eq68_e1484;
        let eq68_node_derivatives: [f64; 21] = [eq68_e1484_d_n0, eq68_e1484_d_n1, eq68_e1484_d_n2, eq68_e1484_d_n3, eq68_e1484_d_n4, eq68_e1484_d_n5, eq68_e1484_d_n6, eq68_e1484_d_n7, eq68_e1484_d_n8, eq68_e1484_d_n9, eq68_e1484_d_n10, eq68_e1484_d_n11, eq68_e1484_d_n12, eq68_e1484_d_n13, eq68_e1484_d_n14, eq68_e1484_d_n15, eq68_e1484_d_n16, eq68_e1484_d_n17, eq68_e1484_d_n18, eq68_e1484_d_n19, eq68_e1484_d_n20];
        let eq68_branch_derivatives: [f64; 25] = [eq68_e1484_d_b0, eq68_e1484_d_b1, eq68_e1484_d_b2, eq68_e1484_d_b3, eq68_e1484_d_b4, eq68_e1484_d_b5, eq68_e1484_d_b6, eq68_e1484_d_b7, eq68_e1484_d_b8, eq68_e1484_d_b9, eq68_e1484_d_b10, eq68_e1484_d_b11, eq68_e1484_d_b12, eq68_e1484_d_b13, eq68_e1484_d_b14, eq68_e1484_d_b15, eq68_e1484_d_b16, eq68_e1484_d_b17, eq68_e1484_d_b18, eq68_e1484_d_b19, eq68_e1484_d_b20, eq68_e1484_d_b21, eq68_e1484_d_b22, eq68_e1484_d_b23, eq68_e1484_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq68_value),
            &nodes,
            &eq68_node_derivatives,
            &branches,
            &eq68_branch_derivatives,
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
        let eq70_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq70_value),
            &[
            ],
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
        let eq71_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq71_value),
            &[
            ],
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
        let eq72_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq72_value),
            &[
            ],
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
        let eq73_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq73_value),
            &[
            ],
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
        let eq74_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[10]),
            Some(nodes[6]),
            self.multiplicity * (eq74_value),
            &[
            ],
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
        let eq75_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq75_value),
            &[
            ],
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
        let eq76_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq76_value),
            &[
            ],
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
        let eq77_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq77_value),
            &[
            ],
        );
    }
}
