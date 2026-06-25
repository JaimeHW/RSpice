#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq3_e987, eq3_e987_d_n0, eq3_e987_d_n1, eq3_e987_d_n2, eq3_e987_d_n3, eq3_e987_d_n4, eq3_e987_d_n5, eq3_e987_d_n6, eq3_e987_d_n7, eq3_e987_d_n8, eq3_e987_d_n9, eq3_e987_d_n10, eq3_e987_d_n11, eq3_e987_d_n12, eq3_e987_d_n13, eq3_e987_d_n14, eq3_e987_d_n15, eq3_e987_d_n16, eq3_e987_d_n17, eq3_e987_d_n18, eq3_e987_d_n19, eq3_e987_d_n20, eq3_e987_d_b0, eq3_e987_d_b1, eq3_e987_d_b2, eq3_e987_d_b3, eq3_e987_d_b4, eq3_e987_d_b5, eq3_e987_d_b6, eq3_e987_d_b7, eq3_e987_d_b8, eq3_e987_d_b9, eq3_e987_d_b10, eq3_e987_d_b11, eq3_e987_d_b12, eq3_e987_d_b13, eq3_e987_d_b14, eq3_e987_d_b15, eq3_e987_d_b16, eq3_e987_d_b17, eq3_e987_d_b18, eq3_e987_d_b19, eq3_e987_d_b20, eq3_e987_d_b21, eq3_e987_d_b22, eq3_e987_d_b23, eq3_e987_d_b24,) = {
    if (s.v[2913] != 0.0) {
        let eq3_e981: f64 = (s.v[0] * s.v[19]);
        let eq3_e981_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq3_e981_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq3_e981_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq3_e981_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq3_e981_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq3_e981_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq3_e981_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq3_e981_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq3_e981_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq3_e981_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq3_e981_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq3_e981_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq3_e981_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq3_e981_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq3_e981_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq3_e981_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq3_e981_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq3_e981_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq3_e981_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq3_e981_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq3_e981_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq3_e981_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq3_e981_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq3_e981_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq3_e981_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq3_e981_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq3_e981_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq3_e981_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq3_e981_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq3_e981_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq3_e981_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq3_e981_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq3_e981_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq3_e981_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq3_e981_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq3_e981_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq3_e981_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq3_e981_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq3_e981_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq3_e981_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq3_e981_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq3_e981_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq3_e981_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq3_e981_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq3_e981_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq3_e981_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq3_e983: f64 = (eq3_e981 * p.p32);
        let eq3_e983_d_n0: f64 = (eq3_e981_d_n0 * p.p32);
        let eq3_e983_d_n1: f64 = (eq3_e981_d_n1 * p.p32);
        let eq3_e983_d_n2: f64 = (eq3_e981_d_n2 * p.p32);
        let eq3_e983_d_n3: f64 = (eq3_e981_d_n3 * p.p32);
        let eq3_e983_d_n4: f64 = (eq3_e981_d_n4 * p.p32);
        let eq3_e983_d_n5: f64 = (eq3_e981_d_n5 * p.p32);
        let eq3_e983_d_n6: f64 = (eq3_e981_d_n6 * p.p32);
        let eq3_e983_d_n7: f64 = (eq3_e981_d_n7 * p.p32);
        let eq3_e983_d_n8: f64 = (eq3_e981_d_n8 * p.p32);
        let eq3_e983_d_n9: f64 = (eq3_e981_d_n9 * p.p32);
        let eq3_e983_d_n10: f64 = (eq3_e981_d_n10 * p.p32);
        let eq3_e983_d_n11: f64 = (eq3_e981_d_n11 * p.p32);
        let eq3_e983_d_n12: f64 = (eq3_e981_d_n12 * p.p32);
        let eq3_e983_d_n13: f64 = (eq3_e981_d_n13 * p.p32);
        let eq3_e983_d_n14: f64 = (eq3_e981_d_n14 * p.p32);
        let eq3_e983_d_n15: f64 = (eq3_e981_d_n15 * p.p32);
        let eq3_e983_d_n16: f64 = (eq3_e981_d_n16 * p.p32);
        let eq3_e983_d_n17: f64 = (eq3_e981_d_n17 * p.p32);
        let eq3_e983_d_n18: f64 = (eq3_e981_d_n18 * p.p32);
        let eq3_e983_d_n19: f64 = (eq3_e981_d_n19 * p.p32);
        let eq3_e983_d_n20: f64 = (eq3_e981_d_n20 * p.p32);
        let eq3_e983_d_b0: f64 = (eq3_e981_d_b0 * p.p32);
        let eq3_e983_d_b1: f64 = (eq3_e981_d_b1 * p.p32);
        let eq3_e983_d_b2: f64 = (eq3_e981_d_b2 * p.p32);
        let eq3_e983_d_b3: f64 = (eq3_e981_d_b3 * p.p32);
        let eq3_e983_d_b4: f64 = (eq3_e981_d_b4 * p.p32);
        let eq3_e983_d_b5: f64 = (eq3_e981_d_b5 * p.p32);
        let eq3_e983_d_b6: f64 = (eq3_e981_d_b6 * p.p32);
        let eq3_e983_d_b7: f64 = (eq3_e981_d_b7 * p.p32);
        let eq3_e983_d_b8: f64 = (eq3_e981_d_b8 * p.p32);
        let eq3_e983_d_b9: f64 = (eq3_e981_d_b9 * p.p32);
        let eq3_e983_d_b10: f64 = (eq3_e981_d_b10 * p.p32);
        let eq3_e983_d_b11: f64 = (eq3_e981_d_b11 * p.p32);
        let eq3_e983_d_b12: f64 = (eq3_e981_d_b12 * p.p32);
        let eq3_e983_d_b13: f64 = (eq3_e981_d_b13 * p.p32);
        let eq3_e983_d_b14: f64 = (eq3_e981_d_b14 * p.p32);
        let eq3_e983_d_b15: f64 = (eq3_e981_d_b15 * p.p32);
        let eq3_e983_d_b16: f64 = (eq3_e981_d_b16 * p.p32);
        let eq3_e983_d_b17: f64 = (eq3_e981_d_b17 * p.p32);
        let eq3_e983_d_b18: f64 = (eq3_e981_d_b18 * p.p32);
        let eq3_e983_d_b19: f64 = (eq3_e981_d_b19 * p.p32);
        let eq3_e983_d_b20: f64 = (eq3_e981_d_b20 * p.p32);
        let eq3_e983_d_b21: f64 = (eq3_e981_d_b21 * p.p32);
        let eq3_e983_d_b22: f64 = (eq3_e981_d_b22 * p.p32);
        let eq3_e983_d_b23: f64 = (eq3_e981_d_b23 * p.p32);
        let eq3_e983_d_b24: f64 = (eq3_e981_d_b24 * p.p32);
        let eq3_e985: f64 = (eq3_e983 * s.v[842]);
        let eq3_e985_d_n0: f64 = ((eq3_e983_d_n0 * s.v[842]) + (eq3_e983 * s.dn[842][0]));
        let eq3_e985_d_n1: f64 = ((eq3_e983_d_n1 * s.v[842]) + (eq3_e983 * s.dn[842][1]));
        let eq3_e985_d_n2: f64 = ((eq3_e983_d_n2 * s.v[842]) + (eq3_e983 * s.dn[842][2]));
        let eq3_e985_d_n3: f64 = ((eq3_e983_d_n3 * s.v[842]) + (eq3_e983 * s.dn[842][3]));
        let eq3_e985_d_n4: f64 = ((eq3_e983_d_n4 * s.v[842]) + (eq3_e983 * s.dn[842][4]));
        let eq3_e985_d_n5: f64 = ((eq3_e983_d_n5 * s.v[842]) + (eq3_e983 * s.dn[842][5]));
        let eq3_e985_d_n6: f64 = ((eq3_e983_d_n6 * s.v[842]) + (eq3_e983 * s.dn[842][6]));
        let eq3_e985_d_n7: f64 = ((eq3_e983_d_n7 * s.v[842]) + (eq3_e983 * s.dn[842][7]));
        let eq3_e985_d_n8: f64 = ((eq3_e983_d_n8 * s.v[842]) + (eq3_e983 * s.dn[842][8]));
        let eq3_e985_d_n9: f64 = ((eq3_e983_d_n9 * s.v[842]) + (eq3_e983 * s.dn[842][9]));
        let eq3_e985_d_n10: f64 = ((eq3_e983_d_n10 * s.v[842]) + (eq3_e983 * s.dn[842][10]));
        let eq3_e985_d_n11: f64 = ((eq3_e983_d_n11 * s.v[842]) + (eq3_e983 * s.dn[842][11]));
        let eq3_e985_d_n12: f64 = ((eq3_e983_d_n12 * s.v[842]) + (eq3_e983 * s.dn[842][12]));
        let eq3_e985_d_n13: f64 = ((eq3_e983_d_n13 * s.v[842]) + (eq3_e983 * s.dn[842][13]));
        let eq3_e985_d_n14: f64 = ((eq3_e983_d_n14 * s.v[842]) + (eq3_e983 * s.dn[842][14]));
        let eq3_e985_d_n15: f64 = ((eq3_e983_d_n15 * s.v[842]) + (eq3_e983 * s.dn[842][15]));
        let eq3_e985_d_n16: f64 = ((eq3_e983_d_n16 * s.v[842]) + (eq3_e983 * s.dn[842][16]));
        let eq3_e985_d_n17: f64 = ((eq3_e983_d_n17 * s.v[842]) + (eq3_e983 * s.dn[842][17]));
        let eq3_e985_d_n18: f64 = ((eq3_e983_d_n18 * s.v[842]) + (eq3_e983 * s.dn[842][18]));
        let eq3_e985_d_n19: f64 = ((eq3_e983_d_n19 * s.v[842]) + (eq3_e983 * s.dn[842][19]));
        let eq3_e985_d_n20: f64 = ((eq3_e983_d_n20 * s.v[842]) + (eq3_e983 * s.dn[842][20]));
        let eq3_e985_d_b0: f64 = ((eq3_e983_d_b0 * s.v[842]) + (eq3_e983 * s.db[842][0]));
        let eq3_e985_d_b1: f64 = ((eq3_e983_d_b1 * s.v[842]) + (eq3_e983 * s.db[842][1]));
        let eq3_e985_d_b2: f64 = ((eq3_e983_d_b2 * s.v[842]) + (eq3_e983 * s.db[842][2]));
        let eq3_e985_d_b3: f64 = ((eq3_e983_d_b3 * s.v[842]) + (eq3_e983 * s.db[842][3]));
        let eq3_e985_d_b4: f64 = ((eq3_e983_d_b4 * s.v[842]) + (eq3_e983 * s.db[842][4]));
        let eq3_e985_d_b5: f64 = ((eq3_e983_d_b5 * s.v[842]) + (eq3_e983 * s.db[842][5]));
        let eq3_e985_d_b6: f64 = ((eq3_e983_d_b6 * s.v[842]) + (eq3_e983 * s.db[842][6]));
        let eq3_e985_d_b7: f64 = ((eq3_e983_d_b7 * s.v[842]) + (eq3_e983 * s.db[842][7]));
        let eq3_e985_d_b8: f64 = ((eq3_e983_d_b8 * s.v[842]) + (eq3_e983 * s.db[842][8]));
        let eq3_e985_d_b9: f64 = ((eq3_e983_d_b9 * s.v[842]) + (eq3_e983 * s.db[842][9]));
        let eq3_e985_d_b10: f64 = ((eq3_e983_d_b10 * s.v[842]) + (eq3_e983 * s.db[842][10]));
        let eq3_e985_d_b11: f64 = ((eq3_e983_d_b11 * s.v[842]) + (eq3_e983 * s.db[842][11]));
        let eq3_e985_d_b12: f64 = ((eq3_e983_d_b12 * s.v[842]) + (eq3_e983 * s.db[842][12]));
        let eq3_e985_d_b13: f64 = ((eq3_e983_d_b13 * s.v[842]) + (eq3_e983 * s.db[842][13]));
        let eq3_e985_d_b14: f64 = ((eq3_e983_d_b14 * s.v[842]) + (eq3_e983 * s.db[842][14]));
        let eq3_e985_d_b15: f64 = ((eq3_e983_d_b15 * s.v[842]) + (eq3_e983 * s.db[842][15]));
        let eq3_e985_d_b16: f64 = ((eq3_e983_d_b16 * s.v[842]) + (eq3_e983 * s.db[842][16]));
        let eq3_e985_d_b17: f64 = ((eq3_e983_d_b17 * s.v[842]) + (eq3_e983 * s.db[842][17]));
        let eq3_e985_d_b18: f64 = ((eq3_e983_d_b18 * s.v[842]) + (eq3_e983 * s.db[842][18]));
        let eq3_e985_d_b19: f64 = ((eq3_e983_d_b19 * s.v[842]) + (eq3_e983 * s.db[842][19]));
        let eq3_e985_d_b20: f64 = ((eq3_e983_d_b20 * s.v[842]) + (eq3_e983 * s.db[842][20]));
        let eq3_e985_d_b21: f64 = ((eq3_e983_d_b21 * s.v[842]) + (eq3_e983 * s.db[842][21]));
        let eq3_e985_d_b22: f64 = ((eq3_e983_d_b22 * s.v[842]) + (eq3_e983 * s.db[842][22]));
        let eq3_e985_d_b23: f64 = ((eq3_e983_d_b23 * s.v[842]) + (eq3_e983 * s.db[842][23]));
        let eq3_e985_d_b24: f64 = ((eq3_e983_d_b24 * s.v[842]) + (eq3_e983 * s.db[842][24]));
        (eq3_e985, eq3_e985_d_n0, eq3_e985_d_n1, eq3_e985_d_n2, eq3_e985_d_n3, eq3_e985_d_n4, eq3_e985_d_n5, eq3_e985_d_n6, eq3_e985_d_n7, eq3_e985_d_n8, eq3_e985_d_n9, eq3_e985_d_n10, eq3_e985_d_n11, eq3_e985_d_n12, eq3_e985_d_n13, eq3_e985_d_n14, eq3_e985_d_n15, eq3_e985_d_n16, eq3_e985_d_n17, eq3_e985_d_n18, eq3_e985_d_n19, eq3_e985_d_n20, eq3_e985_d_b0, eq3_e985_d_b1, eq3_e985_d_b2, eq3_e985_d_b3, eq3_e985_d_b4, eq3_e985_d_b5, eq3_e985_d_b6, eq3_e985_d_b7, eq3_e985_d_b8, eq3_e985_d_b9, eq3_e985_d_b10, eq3_e985_d_b11, eq3_e985_d_b12, eq3_e985_d_b13, eq3_e985_d_b14, eq3_e985_d_b15, eq3_e985_d_b16, eq3_e985_d_b17, eq3_e985_d_b18, eq3_e985_d_b19, eq3_e985_d_b20, eq3_e985_d_b21, eq3_e985_d_b22, eq3_e985_d_b23, eq3_e985_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e987;
        let eq3_node_derivatives: [f64; 21] = [eq3_e987_d_n0, eq3_e987_d_n1, eq3_e987_d_n2, eq3_e987_d_n3, eq3_e987_d_n4, eq3_e987_d_n5, eq3_e987_d_n6, eq3_e987_d_n7, eq3_e987_d_n8, eq3_e987_d_n9, eq3_e987_d_n10, eq3_e987_d_n11, eq3_e987_d_n12, eq3_e987_d_n13, eq3_e987_d_n14, eq3_e987_d_n15, eq3_e987_d_n16, eq3_e987_d_n17, eq3_e987_d_n18, eq3_e987_d_n19, eq3_e987_d_n20];
        let eq3_branch_derivatives: [f64; 25] = [eq3_e987_d_b0, eq3_e987_d_b1, eq3_e987_d_b2, eq3_e987_d_b3, eq3_e987_d_b4, eq3_e987_d_b5, eq3_e987_d_b6, eq3_e987_d_b7, eq3_e987_d_b8, eq3_e987_d_b9, eq3_e987_d_b10, eq3_e987_d_b11, eq3_e987_d_b12, eq3_e987_d_b13, eq3_e987_d_b14, eq3_e987_d_b15, eq3_e987_d_b16, eq3_e987_d_b17, eq3_e987_d_b18, eq3_e987_d_b19, eq3_e987_d_b20, eq3_e987_d_b21, eq3_e987_d_b22, eq3_e987_d_b23, eq3_e987_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq3_value),
            &nodes,
            &eq3_node_derivatives,
            &branches,
            &eq3_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq4_e998, eq4_e998_d_n0, eq4_e998_d_n1, eq4_e998_d_n2, eq4_e998_d_n3, eq4_e998_d_n4, eq4_e998_d_n5, eq4_e998_d_n6, eq4_e998_d_n7, eq4_e998_d_n8, eq4_e998_d_n9, eq4_e998_d_n10, eq4_e998_d_n11, eq4_e998_d_n12, eq4_e998_d_n13, eq4_e998_d_n14, eq4_e998_d_n15, eq4_e998_d_n16, eq4_e998_d_n17, eq4_e998_d_n18, eq4_e998_d_n19, eq4_e998_d_n20, eq4_e998_d_b0, eq4_e998_d_b1, eq4_e998_d_b2, eq4_e998_d_b3, eq4_e998_d_b4, eq4_e998_d_b5, eq4_e998_d_b6, eq4_e998_d_b7, eq4_e998_d_b8, eq4_e998_d_b9, eq4_e998_d_b10, eq4_e998_d_b11, eq4_e998_d_b12, eq4_e998_d_b13, eq4_e998_d_b14, eq4_e998_d_b15, eq4_e998_d_b16, eq4_e998_d_b17, eq4_e998_d_b18, eq4_e998_d_b19, eq4_e998_d_b20, eq4_e998_d_b21, eq4_e998_d_b22, eq4_e998_d_b23, eq4_e998_d_b24,) = {
    if (!(s.v[2913] != 0.0)) {
        let eq4_e992: f64 = (s.v[0] * s.v[19]);
        let eq4_e992_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq4_e992_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq4_e992_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq4_e992_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq4_e992_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq4_e992_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq4_e992_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq4_e992_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq4_e992_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq4_e992_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq4_e992_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq4_e992_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq4_e992_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq4_e992_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq4_e992_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq4_e992_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq4_e992_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq4_e992_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq4_e992_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq4_e992_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq4_e992_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq4_e992_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq4_e992_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq4_e992_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq4_e992_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq4_e992_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq4_e992_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq4_e992_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq4_e992_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq4_e992_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq4_e992_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq4_e992_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq4_e992_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq4_e992_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq4_e992_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq4_e992_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq4_e992_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq4_e992_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq4_e992_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq4_e992_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq4_e992_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq4_e992_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq4_e992_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq4_e992_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq4_e992_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq4_e992_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq4_e994: f64 = (eq4_e992 * p.p32);
        let eq4_e994_d_n0: f64 = (eq4_e992_d_n0 * p.p32);
        let eq4_e994_d_n1: f64 = (eq4_e992_d_n1 * p.p32);
        let eq4_e994_d_n2: f64 = (eq4_e992_d_n2 * p.p32);
        let eq4_e994_d_n3: f64 = (eq4_e992_d_n3 * p.p32);
        let eq4_e994_d_n4: f64 = (eq4_e992_d_n4 * p.p32);
        let eq4_e994_d_n5: f64 = (eq4_e992_d_n5 * p.p32);
        let eq4_e994_d_n6: f64 = (eq4_e992_d_n6 * p.p32);
        let eq4_e994_d_n7: f64 = (eq4_e992_d_n7 * p.p32);
        let eq4_e994_d_n8: f64 = (eq4_e992_d_n8 * p.p32);
        let eq4_e994_d_n9: f64 = (eq4_e992_d_n9 * p.p32);
        let eq4_e994_d_n10: f64 = (eq4_e992_d_n10 * p.p32);
        let eq4_e994_d_n11: f64 = (eq4_e992_d_n11 * p.p32);
        let eq4_e994_d_n12: f64 = (eq4_e992_d_n12 * p.p32);
        let eq4_e994_d_n13: f64 = (eq4_e992_d_n13 * p.p32);
        let eq4_e994_d_n14: f64 = (eq4_e992_d_n14 * p.p32);
        let eq4_e994_d_n15: f64 = (eq4_e992_d_n15 * p.p32);
        let eq4_e994_d_n16: f64 = (eq4_e992_d_n16 * p.p32);
        let eq4_e994_d_n17: f64 = (eq4_e992_d_n17 * p.p32);
        let eq4_e994_d_n18: f64 = (eq4_e992_d_n18 * p.p32);
        let eq4_e994_d_n19: f64 = (eq4_e992_d_n19 * p.p32);
        let eq4_e994_d_n20: f64 = (eq4_e992_d_n20 * p.p32);
        let eq4_e994_d_b0: f64 = (eq4_e992_d_b0 * p.p32);
        let eq4_e994_d_b1: f64 = (eq4_e992_d_b1 * p.p32);
        let eq4_e994_d_b2: f64 = (eq4_e992_d_b2 * p.p32);
        let eq4_e994_d_b3: f64 = (eq4_e992_d_b3 * p.p32);
        let eq4_e994_d_b4: f64 = (eq4_e992_d_b4 * p.p32);
        let eq4_e994_d_b5: f64 = (eq4_e992_d_b5 * p.p32);
        let eq4_e994_d_b6: f64 = (eq4_e992_d_b6 * p.p32);
        let eq4_e994_d_b7: f64 = (eq4_e992_d_b7 * p.p32);
        let eq4_e994_d_b8: f64 = (eq4_e992_d_b8 * p.p32);
        let eq4_e994_d_b9: f64 = (eq4_e992_d_b9 * p.p32);
        let eq4_e994_d_b10: f64 = (eq4_e992_d_b10 * p.p32);
        let eq4_e994_d_b11: f64 = (eq4_e992_d_b11 * p.p32);
        let eq4_e994_d_b12: f64 = (eq4_e992_d_b12 * p.p32);
        let eq4_e994_d_b13: f64 = (eq4_e992_d_b13 * p.p32);
        let eq4_e994_d_b14: f64 = (eq4_e992_d_b14 * p.p32);
        let eq4_e994_d_b15: f64 = (eq4_e992_d_b15 * p.p32);
        let eq4_e994_d_b16: f64 = (eq4_e992_d_b16 * p.p32);
        let eq4_e994_d_b17: f64 = (eq4_e992_d_b17 * p.p32);
        let eq4_e994_d_b18: f64 = (eq4_e992_d_b18 * p.p32);
        let eq4_e994_d_b19: f64 = (eq4_e992_d_b19 * p.p32);
        let eq4_e994_d_b20: f64 = (eq4_e992_d_b20 * p.p32);
        let eq4_e994_d_b21: f64 = (eq4_e992_d_b21 * p.p32);
        let eq4_e994_d_b22: f64 = (eq4_e992_d_b22 * p.p32);
        let eq4_e994_d_b23: f64 = (eq4_e992_d_b23 * p.p32);
        let eq4_e994_d_b24: f64 = (eq4_e992_d_b24 * p.p32);
        let eq4_e996: f64 = (eq4_e994 * s.v[847]);
        let eq4_e996_d_n0: f64 = ((eq4_e994_d_n0 * s.v[847]) + (eq4_e994 * s.dn[847][0]));
        let eq4_e996_d_n1: f64 = ((eq4_e994_d_n1 * s.v[847]) + (eq4_e994 * s.dn[847][1]));
        let eq4_e996_d_n2: f64 = ((eq4_e994_d_n2 * s.v[847]) + (eq4_e994 * s.dn[847][2]));
        let eq4_e996_d_n3: f64 = ((eq4_e994_d_n3 * s.v[847]) + (eq4_e994 * s.dn[847][3]));
        let eq4_e996_d_n4: f64 = ((eq4_e994_d_n4 * s.v[847]) + (eq4_e994 * s.dn[847][4]));
        let eq4_e996_d_n5: f64 = ((eq4_e994_d_n5 * s.v[847]) + (eq4_e994 * s.dn[847][5]));
        let eq4_e996_d_n6: f64 = ((eq4_e994_d_n6 * s.v[847]) + (eq4_e994 * s.dn[847][6]));
        let eq4_e996_d_n7: f64 = ((eq4_e994_d_n7 * s.v[847]) + (eq4_e994 * s.dn[847][7]));
        let eq4_e996_d_n8: f64 = ((eq4_e994_d_n8 * s.v[847]) + (eq4_e994 * s.dn[847][8]));
        let eq4_e996_d_n9: f64 = ((eq4_e994_d_n9 * s.v[847]) + (eq4_e994 * s.dn[847][9]));
        let eq4_e996_d_n10: f64 = ((eq4_e994_d_n10 * s.v[847]) + (eq4_e994 * s.dn[847][10]));
        let eq4_e996_d_n11: f64 = ((eq4_e994_d_n11 * s.v[847]) + (eq4_e994 * s.dn[847][11]));
        let eq4_e996_d_n12: f64 = ((eq4_e994_d_n12 * s.v[847]) + (eq4_e994 * s.dn[847][12]));
        let eq4_e996_d_n13: f64 = ((eq4_e994_d_n13 * s.v[847]) + (eq4_e994 * s.dn[847][13]));
        let eq4_e996_d_n14: f64 = ((eq4_e994_d_n14 * s.v[847]) + (eq4_e994 * s.dn[847][14]));
        let eq4_e996_d_n15: f64 = ((eq4_e994_d_n15 * s.v[847]) + (eq4_e994 * s.dn[847][15]));
        let eq4_e996_d_n16: f64 = ((eq4_e994_d_n16 * s.v[847]) + (eq4_e994 * s.dn[847][16]));
        let eq4_e996_d_n17: f64 = ((eq4_e994_d_n17 * s.v[847]) + (eq4_e994 * s.dn[847][17]));
        let eq4_e996_d_n18: f64 = ((eq4_e994_d_n18 * s.v[847]) + (eq4_e994 * s.dn[847][18]));
        let eq4_e996_d_n19: f64 = ((eq4_e994_d_n19 * s.v[847]) + (eq4_e994 * s.dn[847][19]));
        let eq4_e996_d_n20: f64 = ((eq4_e994_d_n20 * s.v[847]) + (eq4_e994 * s.dn[847][20]));
        let eq4_e996_d_b0: f64 = ((eq4_e994_d_b0 * s.v[847]) + (eq4_e994 * s.db[847][0]));
        let eq4_e996_d_b1: f64 = ((eq4_e994_d_b1 * s.v[847]) + (eq4_e994 * s.db[847][1]));
        let eq4_e996_d_b2: f64 = ((eq4_e994_d_b2 * s.v[847]) + (eq4_e994 * s.db[847][2]));
        let eq4_e996_d_b3: f64 = ((eq4_e994_d_b3 * s.v[847]) + (eq4_e994 * s.db[847][3]));
        let eq4_e996_d_b4: f64 = ((eq4_e994_d_b4 * s.v[847]) + (eq4_e994 * s.db[847][4]));
        let eq4_e996_d_b5: f64 = ((eq4_e994_d_b5 * s.v[847]) + (eq4_e994 * s.db[847][5]));
        let eq4_e996_d_b6: f64 = ((eq4_e994_d_b6 * s.v[847]) + (eq4_e994 * s.db[847][6]));
        let eq4_e996_d_b7: f64 = ((eq4_e994_d_b7 * s.v[847]) + (eq4_e994 * s.db[847][7]));
        let eq4_e996_d_b8: f64 = ((eq4_e994_d_b8 * s.v[847]) + (eq4_e994 * s.db[847][8]));
        let eq4_e996_d_b9: f64 = ((eq4_e994_d_b9 * s.v[847]) + (eq4_e994 * s.db[847][9]));
        let eq4_e996_d_b10: f64 = ((eq4_e994_d_b10 * s.v[847]) + (eq4_e994 * s.db[847][10]));
        let eq4_e996_d_b11: f64 = ((eq4_e994_d_b11 * s.v[847]) + (eq4_e994 * s.db[847][11]));
        let eq4_e996_d_b12: f64 = ((eq4_e994_d_b12 * s.v[847]) + (eq4_e994 * s.db[847][12]));
        let eq4_e996_d_b13: f64 = ((eq4_e994_d_b13 * s.v[847]) + (eq4_e994 * s.db[847][13]));
        let eq4_e996_d_b14: f64 = ((eq4_e994_d_b14 * s.v[847]) + (eq4_e994 * s.db[847][14]));
        let eq4_e996_d_b15: f64 = ((eq4_e994_d_b15 * s.v[847]) + (eq4_e994 * s.db[847][15]));
        let eq4_e996_d_b16: f64 = ((eq4_e994_d_b16 * s.v[847]) + (eq4_e994 * s.db[847][16]));
        let eq4_e996_d_b17: f64 = ((eq4_e994_d_b17 * s.v[847]) + (eq4_e994 * s.db[847][17]));
        let eq4_e996_d_b18: f64 = ((eq4_e994_d_b18 * s.v[847]) + (eq4_e994 * s.db[847][18]));
        let eq4_e996_d_b19: f64 = ((eq4_e994_d_b19 * s.v[847]) + (eq4_e994 * s.db[847][19]));
        let eq4_e996_d_b20: f64 = ((eq4_e994_d_b20 * s.v[847]) + (eq4_e994 * s.db[847][20]));
        let eq4_e996_d_b21: f64 = ((eq4_e994_d_b21 * s.v[847]) + (eq4_e994 * s.db[847][21]));
        let eq4_e996_d_b22: f64 = ((eq4_e994_d_b22 * s.v[847]) + (eq4_e994 * s.db[847][22]));
        let eq4_e996_d_b23: f64 = ((eq4_e994_d_b23 * s.v[847]) + (eq4_e994 * s.db[847][23]));
        let eq4_e996_d_b24: f64 = ((eq4_e994_d_b24 * s.v[847]) + (eq4_e994 * s.db[847][24]));
        (eq4_e996, eq4_e996_d_n0, eq4_e996_d_n1, eq4_e996_d_n2, eq4_e996_d_n3, eq4_e996_d_n4, eq4_e996_d_n5, eq4_e996_d_n6, eq4_e996_d_n7, eq4_e996_d_n8, eq4_e996_d_n9, eq4_e996_d_n10, eq4_e996_d_n11, eq4_e996_d_n12, eq4_e996_d_n13, eq4_e996_d_n14, eq4_e996_d_n15, eq4_e996_d_n16, eq4_e996_d_n17, eq4_e996_d_n18, eq4_e996_d_n19, eq4_e996_d_n20, eq4_e996_d_b0, eq4_e996_d_b1, eq4_e996_d_b2, eq4_e996_d_b3, eq4_e996_d_b4, eq4_e996_d_b5, eq4_e996_d_b6, eq4_e996_d_b7, eq4_e996_d_b8, eq4_e996_d_b9, eq4_e996_d_b10, eq4_e996_d_b11, eq4_e996_d_b12, eq4_e996_d_b13, eq4_e996_d_b14, eq4_e996_d_b15, eq4_e996_d_b16, eq4_e996_d_b17, eq4_e996_d_b18, eq4_e996_d_b19, eq4_e996_d_b20, eq4_e996_d_b21, eq4_e996_d_b22, eq4_e996_d_b23, eq4_e996_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e998;
        let eq4_node_derivatives: [f64; 21] = [eq4_e998_d_n0, eq4_e998_d_n1, eq4_e998_d_n2, eq4_e998_d_n3, eq4_e998_d_n4, eq4_e998_d_n5, eq4_e998_d_n6, eq4_e998_d_n7, eq4_e998_d_n8, eq4_e998_d_n9, eq4_e998_d_n10, eq4_e998_d_n11, eq4_e998_d_n12, eq4_e998_d_n13, eq4_e998_d_n14, eq4_e998_d_n15, eq4_e998_d_n16, eq4_e998_d_n17, eq4_e998_d_n18, eq4_e998_d_n19, eq4_e998_d_n20];
        let eq4_branch_derivatives: [f64; 25] = [eq4_e998_d_b0, eq4_e998_d_b1, eq4_e998_d_b2, eq4_e998_d_b3, eq4_e998_d_b4, eq4_e998_d_b5, eq4_e998_d_b6, eq4_e998_d_b7, eq4_e998_d_b8, eq4_e998_d_b9, eq4_e998_d_b10, eq4_e998_d_b11, eq4_e998_d_b12, eq4_e998_d_b13, eq4_e998_d_b14, eq4_e998_d_b15, eq4_e998_d_b16, eq4_e998_d_b17, eq4_e998_d_b18, eq4_e998_d_b19, eq4_e998_d_b20, eq4_e998_d_b21, eq4_e998_d_b22, eq4_e998_d_b23, eq4_e998_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq4_value),
            &nodes,
            &eq4_node_derivatives,
            &branches,
            &eq4_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq5_e1011, eq5_e1011_d_n0, eq5_e1011_d_n1, eq5_e1011_d_n2, eq5_e1011_d_n3, eq5_e1011_d_n4, eq5_e1011_d_n5, eq5_e1011_d_n6, eq5_e1011_d_n7, eq5_e1011_d_n8, eq5_e1011_d_n9, eq5_e1011_d_n10, eq5_e1011_d_n11, eq5_e1011_d_n12, eq5_e1011_d_n13, eq5_e1011_d_n14, eq5_e1011_d_n15, eq5_e1011_d_n16, eq5_e1011_d_n17, eq5_e1011_d_n18, eq5_e1011_d_n19, eq5_e1011_d_n20, eq5_e1011_d_b0, eq5_e1011_d_b1, eq5_e1011_d_b2, eq5_e1011_d_b3, eq5_e1011_d_b4, eq5_e1011_d_b5, eq5_e1011_d_b6, eq5_e1011_d_b7, eq5_e1011_d_b8, eq5_e1011_d_b9, eq5_e1011_d_b10, eq5_e1011_d_b11, eq5_e1011_d_b12, eq5_e1011_d_b13, eq5_e1011_d_b14, eq5_e1011_d_b15, eq5_e1011_d_b16, eq5_e1011_d_b17, eq5_e1011_d_b18, eq5_e1011_d_b19, eq5_e1011_d_b20, eq5_e1011_d_b21, eq5_e1011_d_b22, eq5_e1011_d_b23, eq5_e1011_d_b24,) = {
    if (!(s.v[2913] != 0.0)) {
        let eq5_e1003: f64 = (s.v[0] * s.v[19]);
        let eq5_e1003_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq5_e1003_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq5_e1003_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq5_e1003_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq5_e1003_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq5_e1003_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq5_e1003_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq5_e1003_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq5_e1003_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq5_e1003_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq5_e1003_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq5_e1003_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq5_e1003_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq5_e1003_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq5_e1003_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq5_e1003_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq5_e1003_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq5_e1003_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq5_e1003_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq5_e1003_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq5_e1003_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq5_e1003_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq5_e1003_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq5_e1003_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq5_e1003_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq5_e1003_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq5_e1003_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq5_e1003_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq5_e1003_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq5_e1003_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq5_e1003_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq5_e1003_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq5_e1003_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq5_e1003_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq5_e1003_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq5_e1003_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq5_e1003_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq5_e1003_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq5_e1003_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq5_e1003_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq5_e1003_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq5_e1003_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq5_e1003_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq5_e1003_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq5_e1003_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq5_e1003_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq5_e1005: f64 = (eq5_e1003 * p.p32);
        let eq5_e1005_d_n0: f64 = (eq5_e1003_d_n0 * p.p32);
        let eq5_e1005_d_n1: f64 = (eq5_e1003_d_n1 * p.p32);
        let eq5_e1005_d_n2: f64 = (eq5_e1003_d_n2 * p.p32);
        let eq5_e1005_d_n3: f64 = (eq5_e1003_d_n3 * p.p32);
        let eq5_e1005_d_n4: f64 = (eq5_e1003_d_n4 * p.p32);
        let eq5_e1005_d_n5: f64 = (eq5_e1003_d_n5 * p.p32);
        let eq5_e1005_d_n6: f64 = (eq5_e1003_d_n6 * p.p32);
        let eq5_e1005_d_n7: f64 = (eq5_e1003_d_n7 * p.p32);
        let eq5_e1005_d_n8: f64 = (eq5_e1003_d_n8 * p.p32);
        let eq5_e1005_d_n9: f64 = (eq5_e1003_d_n9 * p.p32);
        let eq5_e1005_d_n10: f64 = (eq5_e1003_d_n10 * p.p32);
        let eq5_e1005_d_n11: f64 = (eq5_e1003_d_n11 * p.p32);
        let eq5_e1005_d_n12: f64 = (eq5_e1003_d_n12 * p.p32);
        let eq5_e1005_d_n13: f64 = (eq5_e1003_d_n13 * p.p32);
        let eq5_e1005_d_n14: f64 = (eq5_e1003_d_n14 * p.p32);
        let eq5_e1005_d_n15: f64 = (eq5_e1003_d_n15 * p.p32);
        let eq5_e1005_d_n16: f64 = (eq5_e1003_d_n16 * p.p32);
        let eq5_e1005_d_n17: f64 = (eq5_e1003_d_n17 * p.p32);
        let eq5_e1005_d_n18: f64 = (eq5_e1003_d_n18 * p.p32);
        let eq5_e1005_d_n19: f64 = (eq5_e1003_d_n19 * p.p32);
        let eq5_e1005_d_n20: f64 = (eq5_e1003_d_n20 * p.p32);
        let eq5_e1005_d_b0: f64 = (eq5_e1003_d_b0 * p.p32);
        let eq5_e1005_d_b1: f64 = (eq5_e1003_d_b1 * p.p32);
        let eq5_e1005_d_b2: f64 = (eq5_e1003_d_b2 * p.p32);
        let eq5_e1005_d_b3: f64 = (eq5_e1003_d_b3 * p.p32);
        let eq5_e1005_d_b4: f64 = (eq5_e1003_d_b4 * p.p32);
        let eq5_e1005_d_b5: f64 = (eq5_e1003_d_b5 * p.p32);
        let eq5_e1005_d_b6: f64 = (eq5_e1003_d_b6 * p.p32);
        let eq5_e1005_d_b7: f64 = (eq5_e1003_d_b7 * p.p32);
        let eq5_e1005_d_b8: f64 = (eq5_e1003_d_b8 * p.p32);
        let eq5_e1005_d_b9: f64 = (eq5_e1003_d_b9 * p.p32);
        let eq5_e1005_d_b10: f64 = (eq5_e1003_d_b10 * p.p32);
        let eq5_e1005_d_b11: f64 = (eq5_e1003_d_b11 * p.p32);
        let eq5_e1005_d_b12: f64 = (eq5_e1003_d_b12 * p.p32);
        let eq5_e1005_d_b13: f64 = (eq5_e1003_d_b13 * p.p32);
        let eq5_e1005_d_b14: f64 = (eq5_e1003_d_b14 * p.p32);
        let eq5_e1005_d_b15: f64 = (eq5_e1003_d_b15 * p.p32);
        let eq5_e1005_d_b16: f64 = (eq5_e1003_d_b16 * p.p32);
        let eq5_e1005_d_b17: f64 = (eq5_e1003_d_b17 * p.p32);
        let eq5_e1005_d_b18: f64 = (eq5_e1003_d_b18 * p.p32);
        let eq5_e1005_d_b19: f64 = (eq5_e1003_d_b19 * p.p32);
        let eq5_e1005_d_b20: f64 = (eq5_e1003_d_b20 * p.p32);
        let eq5_e1005_d_b21: f64 = (eq5_e1003_d_b21 * p.p32);
        let eq5_e1005_d_b22: f64 = (eq5_e1003_d_b22 * p.p32);
        let eq5_e1005_d_b23: f64 = (eq5_e1003_d_b23 * p.p32);
        let eq5_e1005_d_b24: f64 = (eq5_e1003_d_b24 * p.p32);
        let eq5_e1008: f64 = (s.v[838] + s.v[846]);
        let eq5_e1008_d_n0: f64 = (s.dn[838][0] + s.dn[846][0]);
        let eq5_e1008_d_n1: f64 = (s.dn[838][1] + s.dn[846][1]);
        let eq5_e1008_d_n2: f64 = (s.dn[838][2] + s.dn[846][2]);
        let eq5_e1008_d_n3: f64 = (s.dn[838][3] + s.dn[846][3]);
        let eq5_e1008_d_n4: f64 = (s.dn[838][4] + s.dn[846][4]);
        let eq5_e1008_d_n5: f64 = (s.dn[838][5] + s.dn[846][5]);
        let eq5_e1008_d_n6: f64 = (s.dn[838][6] + s.dn[846][6]);
        let eq5_e1008_d_n7: f64 = (s.dn[838][7] + s.dn[846][7]);
        let eq5_e1008_d_n8: f64 = (s.dn[838][8] + s.dn[846][8]);
        let eq5_e1008_d_n9: f64 = (s.dn[838][9] + s.dn[846][9]);
        let eq5_e1008_d_n10: f64 = (s.dn[838][10] + s.dn[846][10]);
        let eq5_e1008_d_n11: f64 = (s.dn[838][11] + s.dn[846][11]);
        let eq5_e1008_d_n12: f64 = (s.dn[838][12] + s.dn[846][12]);
        let eq5_e1008_d_n13: f64 = (s.dn[838][13] + s.dn[846][13]);
        let eq5_e1008_d_n14: f64 = (s.dn[838][14] + s.dn[846][14]);
        let eq5_e1008_d_n15: f64 = (s.dn[838][15] + s.dn[846][15]);
        let eq5_e1008_d_n16: f64 = (s.dn[838][16] + s.dn[846][16]);
        let eq5_e1008_d_n17: f64 = (s.dn[838][17] + s.dn[846][17]);
        let eq5_e1008_d_n18: f64 = (s.dn[838][18] + s.dn[846][18]);
        let eq5_e1008_d_n19: f64 = (s.dn[838][19] + s.dn[846][19]);
        let eq5_e1008_d_n20: f64 = (s.dn[838][20] + s.dn[846][20]);
        let eq5_e1008_d_b0: f64 = (s.db[838][0] + s.db[846][0]);
        let eq5_e1008_d_b1: f64 = (s.db[838][1] + s.db[846][1]);
        let eq5_e1008_d_b2: f64 = (s.db[838][2] + s.db[846][2]);
        let eq5_e1008_d_b3: f64 = (s.db[838][3] + s.db[846][3]);
        let eq5_e1008_d_b4: f64 = (s.db[838][4] + s.db[846][4]);
        let eq5_e1008_d_b5: f64 = (s.db[838][5] + s.db[846][5]);
        let eq5_e1008_d_b6: f64 = (s.db[838][6] + s.db[846][6]);
        let eq5_e1008_d_b7: f64 = (s.db[838][7] + s.db[846][7]);
        let eq5_e1008_d_b8: f64 = (s.db[838][8] + s.db[846][8]);
        let eq5_e1008_d_b9: f64 = (s.db[838][9] + s.db[846][9]);
        let eq5_e1008_d_b10: f64 = (s.db[838][10] + s.db[846][10]);
        let eq5_e1008_d_b11: f64 = (s.db[838][11] + s.db[846][11]);
        let eq5_e1008_d_b12: f64 = (s.db[838][12] + s.db[846][12]);
        let eq5_e1008_d_b13: f64 = (s.db[838][13] + s.db[846][13]);
        let eq5_e1008_d_b14: f64 = (s.db[838][14] + s.db[846][14]);
        let eq5_e1008_d_b15: f64 = (s.db[838][15] + s.db[846][15]);
        let eq5_e1008_d_b16: f64 = (s.db[838][16] + s.db[846][16]);
        let eq5_e1008_d_b17: f64 = (s.db[838][17] + s.db[846][17]);
        let eq5_e1008_d_b18: f64 = (s.db[838][18] + s.db[846][18]);
        let eq5_e1008_d_b19: f64 = (s.db[838][19] + s.db[846][19]);
        let eq5_e1008_d_b20: f64 = (s.db[838][20] + s.db[846][20]);
        let eq5_e1008_d_b21: f64 = (s.db[838][21] + s.db[846][21]);
        let eq5_e1008_d_b22: f64 = (s.db[838][22] + s.db[846][22]);
        let eq5_e1008_d_b23: f64 = (s.db[838][23] + s.db[846][23]);
        let eq5_e1008_d_b24: f64 = (s.db[838][24] + s.db[846][24]);
        let eq5_e1009: f64 = (eq5_e1005 * eq5_e1008);
        let eq5_e1009_d_n0: f64 = ((eq5_e1005_d_n0 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n0));
        let eq5_e1009_d_n1: f64 = ((eq5_e1005_d_n1 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n1));
        let eq5_e1009_d_n2: f64 = ((eq5_e1005_d_n2 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n2));
        let eq5_e1009_d_n3: f64 = ((eq5_e1005_d_n3 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n3));
        let eq5_e1009_d_n4: f64 = ((eq5_e1005_d_n4 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n4));
        let eq5_e1009_d_n5: f64 = ((eq5_e1005_d_n5 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n5));
        let eq5_e1009_d_n6: f64 = ((eq5_e1005_d_n6 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n6));
        let eq5_e1009_d_n7: f64 = ((eq5_e1005_d_n7 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n7));
        let eq5_e1009_d_n8: f64 = ((eq5_e1005_d_n8 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n8));
        let eq5_e1009_d_n9: f64 = ((eq5_e1005_d_n9 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n9));
        let eq5_e1009_d_n10: f64 = ((eq5_e1005_d_n10 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n10));
        let eq5_e1009_d_n11: f64 = ((eq5_e1005_d_n11 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n11));
        let eq5_e1009_d_n12: f64 = ((eq5_e1005_d_n12 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n12));
        let eq5_e1009_d_n13: f64 = ((eq5_e1005_d_n13 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n13));
        let eq5_e1009_d_n14: f64 = ((eq5_e1005_d_n14 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n14));
        let eq5_e1009_d_n15: f64 = ((eq5_e1005_d_n15 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n15));
        let eq5_e1009_d_n16: f64 = ((eq5_e1005_d_n16 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n16));
        let eq5_e1009_d_n17: f64 = ((eq5_e1005_d_n17 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n17));
        let eq5_e1009_d_n18: f64 = ((eq5_e1005_d_n18 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n18));
        let eq5_e1009_d_n19: f64 = ((eq5_e1005_d_n19 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n19));
        let eq5_e1009_d_n20: f64 = ((eq5_e1005_d_n20 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n20));
        let eq5_e1009_d_b0: f64 = ((eq5_e1005_d_b0 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b0));
        let eq5_e1009_d_b1: f64 = ((eq5_e1005_d_b1 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b1));
        let eq5_e1009_d_b2: f64 = ((eq5_e1005_d_b2 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b2));
        let eq5_e1009_d_b3: f64 = ((eq5_e1005_d_b3 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b3));
        let eq5_e1009_d_b4: f64 = ((eq5_e1005_d_b4 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b4));
        let eq5_e1009_d_b5: f64 = ((eq5_e1005_d_b5 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b5));
        let eq5_e1009_d_b6: f64 = ((eq5_e1005_d_b6 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b6));
        let eq5_e1009_d_b7: f64 = ((eq5_e1005_d_b7 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b7));
        let eq5_e1009_d_b8: f64 = ((eq5_e1005_d_b8 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b8));
        let eq5_e1009_d_b9: f64 = ((eq5_e1005_d_b9 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b9));
        let eq5_e1009_d_b10: f64 = ((eq5_e1005_d_b10 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b10));
        let eq5_e1009_d_b11: f64 = ((eq5_e1005_d_b11 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b11));
        let eq5_e1009_d_b12: f64 = ((eq5_e1005_d_b12 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b12));
        let eq5_e1009_d_b13: f64 = ((eq5_e1005_d_b13 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b13));
        let eq5_e1009_d_b14: f64 = ((eq5_e1005_d_b14 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b14));
        let eq5_e1009_d_b15: f64 = ((eq5_e1005_d_b15 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b15));
        let eq5_e1009_d_b16: f64 = ((eq5_e1005_d_b16 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b16));
        let eq5_e1009_d_b17: f64 = ((eq5_e1005_d_b17 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b17));
        let eq5_e1009_d_b18: f64 = ((eq5_e1005_d_b18 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b18));
        let eq5_e1009_d_b19: f64 = ((eq5_e1005_d_b19 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b19));
        let eq5_e1009_d_b20: f64 = ((eq5_e1005_d_b20 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b20));
        let eq5_e1009_d_b21: f64 = ((eq5_e1005_d_b21 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b21));
        let eq5_e1009_d_b22: f64 = ((eq5_e1005_d_b22 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b22));
        let eq5_e1009_d_b23: f64 = ((eq5_e1005_d_b23 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b23));
        let eq5_e1009_d_b24: f64 = ((eq5_e1005_d_b24 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b24));
        (eq5_e1009, eq5_e1009_d_n0, eq5_e1009_d_n1, eq5_e1009_d_n2, eq5_e1009_d_n3, eq5_e1009_d_n4, eq5_e1009_d_n5, eq5_e1009_d_n6, eq5_e1009_d_n7, eq5_e1009_d_n8, eq5_e1009_d_n9, eq5_e1009_d_n10, eq5_e1009_d_n11, eq5_e1009_d_n12, eq5_e1009_d_n13, eq5_e1009_d_n14, eq5_e1009_d_n15, eq5_e1009_d_n16, eq5_e1009_d_n17, eq5_e1009_d_n18, eq5_e1009_d_n19, eq5_e1009_d_n20, eq5_e1009_d_b0, eq5_e1009_d_b1, eq5_e1009_d_b2, eq5_e1009_d_b3, eq5_e1009_d_b4, eq5_e1009_d_b5, eq5_e1009_d_b6, eq5_e1009_d_b7, eq5_e1009_d_b8, eq5_e1009_d_b9, eq5_e1009_d_b10, eq5_e1009_d_b11, eq5_e1009_d_b12, eq5_e1009_d_b13, eq5_e1009_d_b14, eq5_e1009_d_b15, eq5_e1009_d_b16, eq5_e1009_d_b17, eq5_e1009_d_b18, eq5_e1009_d_b19, eq5_e1009_d_b20, eq5_e1009_d_b21, eq5_e1009_d_b22, eq5_e1009_d_b23, eq5_e1009_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1011;
        let eq5_node_derivatives: [f64; 21] = [eq5_e1011_d_n0, eq5_e1011_d_n1, eq5_e1011_d_n2, eq5_e1011_d_n3, eq5_e1011_d_n4, eq5_e1011_d_n5, eq5_e1011_d_n6, eq5_e1011_d_n7, eq5_e1011_d_n8, eq5_e1011_d_n9, eq5_e1011_d_n10, eq5_e1011_d_n11, eq5_e1011_d_n12, eq5_e1011_d_n13, eq5_e1011_d_n14, eq5_e1011_d_n15, eq5_e1011_d_n16, eq5_e1011_d_n17, eq5_e1011_d_n18, eq5_e1011_d_n19, eq5_e1011_d_n20];
        let eq5_branch_derivatives: [f64; 25] = [eq5_e1011_d_b0, eq5_e1011_d_b1, eq5_e1011_d_b2, eq5_e1011_d_b3, eq5_e1011_d_b4, eq5_e1011_d_b5, eq5_e1011_d_b6, eq5_e1011_d_b7, eq5_e1011_d_b8, eq5_e1011_d_b9, eq5_e1011_d_b10, eq5_e1011_d_b11, eq5_e1011_d_b12, eq5_e1011_d_b13, eq5_e1011_d_b14, eq5_e1011_d_b15, eq5_e1011_d_b16, eq5_e1011_d_b17, eq5_e1011_d_b18, eq5_e1011_d_b19, eq5_e1011_d_b20, eq5_e1011_d_b21, eq5_e1011_d_b22, eq5_e1011_d_b23, eq5_e1011_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq5_value),
            &nodes,
            &eq5_node_derivatives,
            &branches,
            &eq5_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq6_e1022, eq6_e1022_d_n0, eq6_e1022_d_n1, eq6_e1022_d_n2, eq6_e1022_d_n3, eq6_e1022_d_n4, eq6_e1022_d_n5, eq6_e1022_d_n6, eq6_e1022_d_n7, eq6_e1022_d_n8, eq6_e1022_d_n9, eq6_e1022_d_n10, eq6_e1022_d_n11, eq6_e1022_d_n12, eq6_e1022_d_n13, eq6_e1022_d_n14, eq6_e1022_d_n15, eq6_e1022_d_n16, eq6_e1022_d_n17, eq6_e1022_d_n18, eq6_e1022_d_n19, eq6_e1022_d_n20, eq6_e1022_d_b0, eq6_e1022_d_b1, eq6_e1022_d_b2, eq6_e1022_d_b3, eq6_e1022_d_b4, eq6_e1022_d_b5, eq6_e1022_d_b6, eq6_e1022_d_b7, eq6_e1022_d_b8, eq6_e1022_d_b9, eq6_e1022_d_b10, eq6_e1022_d_b11, eq6_e1022_d_b12, eq6_e1022_d_b13, eq6_e1022_d_b14, eq6_e1022_d_b15, eq6_e1022_d_b16, eq6_e1022_d_b17, eq6_e1022_d_b18, eq6_e1022_d_b19, eq6_e1022_d_b20, eq6_e1022_d_b21, eq6_e1022_d_b22, eq6_e1022_d_b23, eq6_e1022_d_b24,) = {
    if (!(s.v[2913] != 0.0)) {
        let eq6_e1016: f64 = (s.v[0] * s.v[19]);
        let eq6_e1016_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq6_e1016_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq6_e1016_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq6_e1016_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq6_e1016_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq6_e1016_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq6_e1016_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq6_e1016_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq6_e1016_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq6_e1016_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq6_e1016_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq6_e1016_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq6_e1016_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq6_e1016_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq6_e1016_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq6_e1016_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq6_e1016_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq6_e1016_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq6_e1016_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq6_e1016_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq6_e1016_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq6_e1016_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq6_e1016_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq6_e1016_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq6_e1016_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq6_e1016_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq6_e1016_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq6_e1016_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq6_e1016_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq6_e1016_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq6_e1016_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq6_e1016_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq6_e1016_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq6_e1016_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq6_e1016_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq6_e1016_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq6_e1016_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq6_e1016_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq6_e1016_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq6_e1016_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq6_e1016_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq6_e1016_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq6_e1016_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq6_e1016_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq6_e1016_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq6_e1016_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq6_e1018: f64 = (eq6_e1016 * p.p32);
        let eq6_e1018_d_n0: f64 = (eq6_e1016_d_n0 * p.p32);
        let eq6_e1018_d_n1: f64 = (eq6_e1016_d_n1 * p.p32);
        let eq6_e1018_d_n2: f64 = (eq6_e1016_d_n2 * p.p32);
        let eq6_e1018_d_n3: f64 = (eq6_e1016_d_n3 * p.p32);
        let eq6_e1018_d_n4: f64 = (eq6_e1016_d_n4 * p.p32);
        let eq6_e1018_d_n5: f64 = (eq6_e1016_d_n5 * p.p32);
        let eq6_e1018_d_n6: f64 = (eq6_e1016_d_n6 * p.p32);
        let eq6_e1018_d_n7: f64 = (eq6_e1016_d_n7 * p.p32);
        let eq6_e1018_d_n8: f64 = (eq6_e1016_d_n8 * p.p32);
        let eq6_e1018_d_n9: f64 = (eq6_e1016_d_n9 * p.p32);
        let eq6_e1018_d_n10: f64 = (eq6_e1016_d_n10 * p.p32);
        let eq6_e1018_d_n11: f64 = (eq6_e1016_d_n11 * p.p32);
        let eq6_e1018_d_n12: f64 = (eq6_e1016_d_n12 * p.p32);
        let eq6_e1018_d_n13: f64 = (eq6_e1016_d_n13 * p.p32);
        let eq6_e1018_d_n14: f64 = (eq6_e1016_d_n14 * p.p32);
        let eq6_e1018_d_n15: f64 = (eq6_e1016_d_n15 * p.p32);
        let eq6_e1018_d_n16: f64 = (eq6_e1016_d_n16 * p.p32);
        let eq6_e1018_d_n17: f64 = (eq6_e1016_d_n17 * p.p32);
        let eq6_e1018_d_n18: f64 = (eq6_e1016_d_n18 * p.p32);
        let eq6_e1018_d_n19: f64 = (eq6_e1016_d_n19 * p.p32);
        let eq6_e1018_d_n20: f64 = (eq6_e1016_d_n20 * p.p32);
        let eq6_e1018_d_b0: f64 = (eq6_e1016_d_b0 * p.p32);
        let eq6_e1018_d_b1: f64 = (eq6_e1016_d_b1 * p.p32);
        let eq6_e1018_d_b2: f64 = (eq6_e1016_d_b2 * p.p32);
        let eq6_e1018_d_b3: f64 = (eq6_e1016_d_b3 * p.p32);
        let eq6_e1018_d_b4: f64 = (eq6_e1016_d_b4 * p.p32);
        let eq6_e1018_d_b5: f64 = (eq6_e1016_d_b5 * p.p32);
        let eq6_e1018_d_b6: f64 = (eq6_e1016_d_b6 * p.p32);
        let eq6_e1018_d_b7: f64 = (eq6_e1016_d_b7 * p.p32);
        let eq6_e1018_d_b8: f64 = (eq6_e1016_d_b8 * p.p32);
        let eq6_e1018_d_b9: f64 = (eq6_e1016_d_b9 * p.p32);
        let eq6_e1018_d_b10: f64 = (eq6_e1016_d_b10 * p.p32);
        let eq6_e1018_d_b11: f64 = (eq6_e1016_d_b11 * p.p32);
        let eq6_e1018_d_b12: f64 = (eq6_e1016_d_b12 * p.p32);
        let eq6_e1018_d_b13: f64 = (eq6_e1016_d_b13 * p.p32);
        let eq6_e1018_d_b14: f64 = (eq6_e1016_d_b14 * p.p32);
        let eq6_e1018_d_b15: f64 = (eq6_e1016_d_b15 * p.p32);
        let eq6_e1018_d_b16: f64 = (eq6_e1016_d_b16 * p.p32);
        let eq6_e1018_d_b17: f64 = (eq6_e1016_d_b17 * p.p32);
        let eq6_e1018_d_b18: f64 = (eq6_e1016_d_b18 * p.p32);
        let eq6_e1018_d_b19: f64 = (eq6_e1016_d_b19 * p.p32);
        let eq6_e1018_d_b20: f64 = (eq6_e1016_d_b20 * p.p32);
        let eq6_e1018_d_b21: f64 = (eq6_e1016_d_b21 * p.p32);
        let eq6_e1018_d_b22: f64 = (eq6_e1016_d_b22 * p.p32);
        let eq6_e1018_d_b23: f64 = (eq6_e1016_d_b23 * p.p32);
        let eq6_e1018_d_b24: f64 = (eq6_e1016_d_b24 * p.p32);
        let eq6_e1020: f64 = (eq6_e1018 * s.v[841]);
        let eq6_e1020_d_n0: f64 = ((eq6_e1018_d_n0 * s.v[841]) + (eq6_e1018 * s.dn[841][0]));
        let eq6_e1020_d_n1: f64 = ((eq6_e1018_d_n1 * s.v[841]) + (eq6_e1018 * s.dn[841][1]));
        let eq6_e1020_d_n2: f64 = ((eq6_e1018_d_n2 * s.v[841]) + (eq6_e1018 * s.dn[841][2]));
        let eq6_e1020_d_n3: f64 = ((eq6_e1018_d_n3 * s.v[841]) + (eq6_e1018 * s.dn[841][3]));
        let eq6_e1020_d_n4: f64 = ((eq6_e1018_d_n4 * s.v[841]) + (eq6_e1018 * s.dn[841][4]));
        let eq6_e1020_d_n5: f64 = ((eq6_e1018_d_n5 * s.v[841]) + (eq6_e1018 * s.dn[841][5]));
        let eq6_e1020_d_n6: f64 = ((eq6_e1018_d_n6 * s.v[841]) + (eq6_e1018 * s.dn[841][6]));
        let eq6_e1020_d_n7: f64 = ((eq6_e1018_d_n7 * s.v[841]) + (eq6_e1018 * s.dn[841][7]));
        let eq6_e1020_d_n8: f64 = ((eq6_e1018_d_n8 * s.v[841]) + (eq6_e1018 * s.dn[841][8]));
        let eq6_e1020_d_n9: f64 = ((eq6_e1018_d_n9 * s.v[841]) + (eq6_e1018 * s.dn[841][9]));
        let eq6_e1020_d_n10: f64 = ((eq6_e1018_d_n10 * s.v[841]) + (eq6_e1018 * s.dn[841][10]));
        let eq6_e1020_d_n11: f64 = ((eq6_e1018_d_n11 * s.v[841]) + (eq6_e1018 * s.dn[841][11]));
        let eq6_e1020_d_n12: f64 = ((eq6_e1018_d_n12 * s.v[841]) + (eq6_e1018 * s.dn[841][12]));
        let eq6_e1020_d_n13: f64 = ((eq6_e1018_d_n13 * s.v[841]) + (eq6_e1018 * s.dn[841][13]));
        let eq6_e1020_d_n14: f64 = ((eq6_e1018_d_n14 * s.v[841]) + (eq6_e1018 * s.dn[841][14]));
        let eq6_e1020_d_n15: f64 = ((eq6_e1018_d_n15 * s.v[841]) + (eq6_e1018 * s.dn[841][15]));
        let eq6_e1020_d_n16: f64 = ((eq6_e1018_d_n16 * s.v[841]) + (eq6_e1018 * s.dn[841][16]));
        let eq6_e1020_d_n17: f64 = ((eq6_e1018_d_n17 * s.v[841]) + (eq6_e1018 * s.dn[841][17]));
        let eq6_e1020_d_n18: f64 = ((eq6_e1018_d_n18 * s.v[841]) + (eq6_e1018 * s.dn[841][18]));
        let eq6_e1020_d_n19: f64 = ((eq6_e1018_d_n19 * s.v[841]) + (eq6_e1018 * s.dn[841][19]));
        let eq6_e1020_d_n20: f64 = ((eq6_e1018_d_n20 * s.v[841]) + (eq6_e1018 * s.dn[841][20]));
        let eq6_e1020_d_b0: f64 = ((eq6_e1018_d_b0 * s.v[841]) + (eq6_e1018 * s.db[841][0]));
        let eq6_e1020_d_b1: f64 = ((eq6_e1018_d_b1 * s.v[841]) + (eq6_e1018 * s.db[841][1]));
        let eq6_e1020_d_b2: f64 = ((eq6_e1018_d_b2 * s.v[841]) + (eq6_e1018 * s.db[841][2]));
        let eq6_e1020_d_b3: f64 = ((eq6_e1018_d_b3 * s.v[841]) + (eq6_e1018 * s.db[841][3]));
        let eq6_e1020_d_b4: f64 = ((eq6_e1018_d_b4 * s.v[841]) + (eq6_e1018 * s.db[841][4]));
        let eq6_e1020_d_b5: f64 = ((eq6_e1018_d_b5 * s.v[841]) + (eq6_e1018 * s.db[841][5]));
        let eq6_e1020_d_b6: f64 = ((eq6_e1018_d_b6 * s.v[841]) + (eq6_e1018 * s.db[841][6]));
        let eq6_e1020_d_b7: f64 = ((eq6_e1018_d_b7 * s.v[841]) + (eq6_e1018 * s.db[841][7]));
        let eq6_e1020_d_b8: f64 = ((eq6_e1018_d_b8 * s.v[841]) + (eq6_e1018 * s.db[841][8]));
        let eq6_e1020_d_b9: f64 = ((eq6_e1018_d_b9 * s.v[841]) + (eq6_e1018 * s.db[841][9]));
        let eq6_e1020_d_b10: f64 = ((eq6_e1018_d_b10 * s.v[841]) + (eq6_e1018 * s.db[841][10]));
        let eq6_e1020_d_b11: f64 = ((eq6_e1018_d_b11 * s.v[841]) + (eq6_e1018 * s.db[841][11]));
        let eq6_e1020_d_b12: f64 = ((eq6_e1018_d_b12 * s.v[841]) + (eq6_e1018 * s.db[841][12]));
        let eq6_e1020_d_b13: f64 = ((eq6_e1018_d_b13 * s.v[841]) + (eq6_e1018 * s.db[841][13]));
        let eq6_e1020_d_b14: f64 = ((eq6_e1018_d_b14 * s.v[841]) + (eq6_e1018 * s.db[841][14]));
        let eq6_e1020_d_b15: f64 = ((eq6_e1018_d_b15 * s.v[841]) + (eq6_e1018 * s.db[841][15]));
        let eq6_e1020_d_b16: f64 = ((eq6_e1018_d_b16 * s.v[841]) + (eq6_e1018 * s.db[841][16]));
        let eq6_e1020_d_b17: f64 = ((eq6_e1018_d_b17 * s.v[841]) + (eq6_e1018 * s.db[841][17]));
        let eq6_e1020_d_b18: f64 = ((eq6_e1018_d_b18 * s.v[841]) + (eq6_e1018 * s.db[841][18]));
        let eq6_e1020_d_b19: f64 = ((eq6_e1018_d_b19 * s.v[841]) + (eq6_e1018 * s.db[841][19]));
        let eq6_e1020_d_b20: f64 = ((eq6_e1018_d_b20 * s.v[841]) + (eq6_e1018 * s.db[841][20]));
        let eq6_e1020_d_b21: f64 = ((eq6_e1018_d_b21 * s.v[841]) + (eq6_e1018 * s.db[841][21]));
        let eq6_e1020_d_b22: f64 = ((eq6_e1018_d_b22 * s.v[841]) + (eq6_e1018 * s.db[841][22]));
        let eq6_e1020_d_b23: f64 = ((eq6_e1018_d_b23 * s.v[841]) + (eq6_e1018 * s.db[841][23]));
        let eq6_e1020_d_b24: f64 = ((eq6_e1018_d_b24 * s.v[841]) + (eq6_e1018 * s.db[841][24]));
        (eq6_e1020, eq6_e1020_d_n0, eq6_e1020_d_n1, eq6_e1020_d_n2, eq6_e1020_d_n3, eq6_e1020_d_n4, eq6_e1020_d_n5, eq6_e1020_d_n6, eq6_e1020_d_n7, eq6_e1020_d_n8, eq6_e1020_d_n9, eq6_e1020_d_n10, eq6_e1020_d_n11, eq6_e1020_d_n12, eq6_e1020_d_n13, eq6_e1020_d_n14, eq6_e1020_d_n15, eq6_e1020_d_n16, eq6_e1020_d_n17, eq6_e1020_d_n18, eq6_e1020_d_n19, eq6_e1020_d_n20, eq6_e1020_d_b0, eq6_e1020_d_b1, eq6_e1020_d_b2, eq6_e1020_d_b3, eq6_e1020_d_b4, eq6_e1020_d_b5, eq6_e1020_d_b6, eq6_e1020_d_b7, eq6_e1020_d_b8, eq6_e1020_d_b9, eq6_e1020_d_b10, eq6_e1020_d_b11, eq6_e1020_d_b12, eq6_e1020_d_b13, eq6_e1020_d_b14, eq6_e1020_d_b15, eq6_e1020_d_b16, eq6_e1020_d_b17, eq6_e1020_d_b18, eq6_e1020_d_b19, eq6_e1020_d_b20, eq6_e1020_d_b21, eq6_e1020_d_b22, eq6_e1020_d_b23, eq6_e1020_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1022;
        let eq6_node_derivatives: [f64; 21] = [eq6_e1022_d_n0, eq6_e1022_d_n1, eq6_e1022_d_n2, eq6_e1022_d_n3, eq6_e1022_d_n4, eq6_e1022_d_n5, eq6_e1022_d_n6, eq6_e1022_d_n7, eq6_e1022_d_n8, eq6_e1022_d_n9, eq6_e1022_d_n10, eq6_e1022_d_n11, eq6_e1022_d_n12, eq6_e1022_d_n13, eq6_e1022_d_n14, eq6_e1022_d_n15, eq6_e1022_d_n16, eq6_e1022_d_n17, eq6_e1022_d_n18, eq6_e1022_d_n19, eq6_e1022_d_n20];
        let eq6_branch_derivatives: [f64; 25] = [eq6_e1022_d_b0, eq6_e1022_d_b1, eq6_e1022_d_b2, eq6_e1022_d_b3, eq6_e1022_d_b4, eq6_e1022_d_b5, eq6_e1022_d_b6, eq6_e1022_d_b7, eq6_e1022_d_b8, eq6_e1022_d_b9, eq6_e1022_d_b10, eq6_e1022_d_b11, eq6_e1022_d_b12, eq6_e1022_d_b13, eq6_e1022_d_b14, eq6_e1022_d_b15, eq6_e1022_d_b16, eq6_e1022_d_b17, eq6_e1022_d_b18, eq6_e1022_d_b19, eq6_e1022_d_b20, eq6_e1022_d_b21, eq6_e1022_d_b22, eq6_e1022_d_b23, eq6_e1022_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq7_e1033, eq7_e1033_d_n0, eq7_e1033_d_n1, eq7_e1033_d_n2, eq7_e1033_d_n3, eq7_e1033_d_n4, eq7_e1033_d_n5, eq7_e1033_d_n6, eq7_e1033_d_n7, eq7_e1033_d_n8, eq7_e1033_d_n9, eq7_e1033_d_n10, eq7_e1033_d_n11, eq7_e1033_d_n12, eq7_e1033_d_n13, eq7_e1033_d_n14, eq7_e1033_d_n15, eq7_e1033_d_n16, eq7_e1033_d_n17, eq7_e1033_d_n18, eq7_e1033_d_n19, eq7_e1033_d_n20, eq7_e1033_d_b0, eq7_e1033_d_b1, eq7_e1033_d_b2, eq7_e1033_d_b3, eq7_e1033_d_b4, eq7_e1033_d_b5, eq7_e1033_d_b6, eq7_e1033_d_b7, eq7_e1033_d_b8, eq7_e1033_d_b9, eq7_e1033_d_b10, eq7_e1033_d_b11, eq7_e1033_d_b12, eq7_e1033_d_b13, eq7_e1033_d_b14, eq7_e1033_d_b15, eq7_e1033_d_b16, eq7_e1033_d_b17, eq7_e1033_d_b18, eq7_e1033_d_b19, eq7_e1033_d_b20, eq7_e1033_d_b21, eq7_e1033_d_b22, eq7_e1033_d_b23, eq7_e1033_d_b24,) = {
    if (!(s.v[2913] != 0.0)) {
        let eq7_e1027: f64 = (s.v[0] * s.v[19]);
        let eq7_e1027_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq7_e1027_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq7_e1027_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq7_e1027_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq7_e1027_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq7_e1027_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq7_e1027_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq7_e1027_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq7_e1027_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq7_e1027_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq7_e1027_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq7_e1027_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq7_e1027_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq7_e1027_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq7_e1027_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq7_e1027_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq7_e1027_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq7_e1027_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq7_e1027_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq7_e1027_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq7_e1027_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq7_e1027_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq7_e1027_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq7_e1027_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq7_e1027_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq7_e1027_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq7_e1027_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq7_e1027_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq7_e1027_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq7_e1027_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq7_e1027_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq7_e1027_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq7_e1027_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq7_e1027_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq7_e1027_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq7_e1027_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq7_e1027_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq7_e1027_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq7_e1027_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq7_e1027_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq7_e1027_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq7_e1027_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq7_e1027_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq7_e1027_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq7_e1027_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq7_e1027_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq7_e1029: f64 = (eq7_e1027 * p.p32);
        let eq7_e1029_d_n0: f64 = (eq7_e1027_d_n0 * p.p32);
        let eq7_e1029_d_n1: f64 = (eq7_e1027_d_n1 * p.p32);
        let eq7_e1029_d_n2: f64 = (eq7_e1027_d_n2 * p.p32);
        let eq7_e1029_d_n3: f64 = (eq7_e1027_d_n3 * p.p32);
        let eq7_e1029_d_n4: f64 = (eq7_e1027_d_n4 * p.p32);
        let eq7_e1029_d_n5: f64 = (eq7_e1027_d_n5 * p.p32);
        let eq7_e1029_d_n6: f64 = (eq7_e1027_d_n6 * p.p32);
        let eq7_e1029_d_n7: f64 = (eq7_e1027_d_n7 * p.p32);
        let eq7_e1029_d_n8: f64 = (eq7_e1027_d_n8 * p.p32);
        let eq7_e1029_d_n9: f64 = (eq7_e1027_d_n9 * p.p32);
        let eq7_e1029_d_n10: f64 = (eq7_e1027_d_n10 * p.p32);
        let eq7_e1029_d_n11: f64 = (eq7_e1027_d_n11 * p.p32);
        let eq7_e1029_d_n12: f64 = (eq7_e1027_d_n12 * p.p32);
        let eq7_e1029_d_n13: f64 = (eq7_e1027_d_n13 * p.p32);
        let eq7_e1029_d_n14: f64 = (eq7_e1027_d_n14 * p.p32);
        let eq7_e1029_d_n15: f64 = (eq7_e1027_d_n15 * p.p32);
        let eq7_e1029_d_n16: f64 = (eq7_e1027_d_n16 * p.p32);
        let eq7_e1029_d_n17: f64 = (eq7_e1027_d_n17 * p.p32);
        let eq7_e1029_d_n18: f64 = (eq7_e1027_d_n18 * p.p32);
        let eq7_e1029_d_n19: f64 = (eq7_e1027_d_n19 * p.p32);
        let eq7_e1029_d_n20: f64 = (eq7_e1027_d_n20 * p.p32);
        let eq7_e1029_d_b0: f64 = (eq7_e1027_d_b0 * p.p32);
        let eq7_e1029_d_b1: f64 = (eq7_e1027_d_b1 * p.p32);
        let eq7_e1029_d_b2: f64 = (eq7_e1027_d_b2 * p.p32);
        let eq7_e1029_d_b3: f64 = (eq7_e1027_d_b3 * p.p32);
        let eq7_e1029_d_b4: f64 = (eq7_e1027_d_b4 * p.p32);
        let eq7_e1029_d_b5: f64 = (eq7_e1027_d_b5 * p.p32);
        let eq7_e1029_d_b6: f64 = (eq7_e1027_d_b6 * p.p32);
        let eq7_e1029_d_b7: f64 = (eq7_e1027_d_b7 * p.p32);
        let eq7_e1029_d_b8: f64 = (eq7_e1027_d_b8 * p.p32);
        let eq7_e1029_d_b9: f64 = (eq7_e1027_d_b9 * p.p32);
        let eq7_e1029_d_b10: f64 = (eq7_e1027_d_b10 * p.p32);
        let eq7_e1029_d_b11: f64 = (eq7_e1027_d_b11 * p.p32);
        let eq7_e1029_d_b12: f64 = (eq7_e1027_d_b12 * p.p32);
        let eq7_e1029_d_b13: f64 = (eq7_e1027_d_b13 * p.p32);
        let eq7_e1029_d_b14: f64 = (eq7_e1027_d_b14 * p.p32);
        let eq7_e1029_d_b15: f64 = (eq7_e1027_d_b15 * p.p32);
        let eq7_e1029_d_b16: f64 = (eq7_e1027_d_b16 * p.p32);
        let eq7_e1029_d_b17: f64 = (eq7_e1027_d_b17 * p.p32);
        let eq7_e1029_d_b18: f64 = (eq7_e1027_d_b18 * p.p32);
        let eq7_e1029_d_b19: f64 = (eq7_e1027_d_b19 * p.p32);
        let eq7_e1029_d_b20: f64 = (eq7_e1027_d_b20 * p.p32);
        let eq7_e1029_d_b21: f64 = (eq7_e1027_d_b21 * p.p32);
        let eq7_e1029_d_b22: f64 = (eq7_e1027_d_b22 * p.p32);
        let eq7_e1029_d_b23: f64 = (eq7_e1027_d_b23 * p.p32);
        let eq7_e1029_d_b24: f64 = (eq7_e1027_d_b24 * p.p32);
        let eq7_e1031: f64 = (eq7_e1029 * s.v[842]);
        let eq7_e1031_d_n0: f64 = ((eq7_e1029_d_n0 * s.v[842]) + (eq7_e1029 * s.dn[842][0]));
        let eq7_e1031_d_n1: f64 = ((eq7_e1029_d_n1 * s.v[842]) + (eq7_e1029 * s.dn[842][1]));
        let eq7_e1031_d_n2: f64 = ((eq7_e1029_d_n2 * s.v[842]) + (eq7_e1029 * s.dn[842][2]));
        let eq7_e1031_d_n3: f64 = ((eq7_e1029_d_n3 * s.v[842]) + (eq7_e1029 * s.dn[842][3]));
        let eq7_e1031_d_n4: f64 = ((eq7_e1029_d_n4 * s.v[842]) + (eq7_e1029 * s.dn[842][4]));
        let eq7_e1031_d_n5: f64 = ((eq7_e1029_d_n5 * s.v[842]) + (eq7_e1029 * s.dn[842][5]));
        let eq7_e1031_d_n6: f64 = ((eq7_e1029_d_n6 * s.v[842]) + (eq7_e1029 * s.dn[842][6]));
        let eq7_e1031_d_n7: f64 = ((eq7_e1029_d_n7 * s.v[842]) + (eq7_e1029 * s.dn[842][7]));
        let eq7_e1031_d_n8: f64 = ((eq7_e1029_d_n8 * s.v[842]) + (eq7_e1029 * s.dn[842][8]));
        let eq7_e1031_d_n9: f64 = ((eq7_e1029_d_n9 * s.v[842]) + (eq7_e1029 * s.dn[842][9]));
        let eq7_e1031_d_n10: f64 = ((eq7_e1029_d_n10 * s.v[842]) + (eq7_e1029 * s.dn[842][10]));
        let eq7_e1031_d_n11: f64 = ((eq7_e1029_d_n11 * s.v[842]) + (eq7_e1029 * s.dn[842][11]));
        let eq7_e1031_d_n12: f64 = ((eq7_e1029_d_n12 * s.v[842]) + (eq7_e1029 * s.dn[842][12]));
        let eq7_e1031_d_n13: f64 = ((eq7_e1029_d_n13 * s.v[842]) + (eq7_e1029 * s.dn[842][13]));
        let eq7_e1031_d_n14: f64 = ((eq7_e1029_d_n14 * s.v[842]) + (eq7_e1029 * s.dn[842][14]));
        let eq7_e1031_d_n15: f64 = ((eq7_e1029_d_n15 * s.v[842]) + (eq7_e1029 * s.dn[842][15]));
        let eq7_e1031_d_n16: f64 = ((eq7_e1029_d_n16 * s.v[842]) + (eq7_e1029 * s.dn[842][16]));
        let eq7_e1031_d_n17: f64 = ((eq7_e1029_d_n17 * s.v[842]) + (eq7_e1029 * s.dn[842][17]));
        let eq7_e1031_d_n18: f64 = ((eq7_e1029_d_n18 * s.v[842]) + (eq7_e1029 * s.dn[842][18]));
        let eq7_e1031_d_n19: f64 = ((eq7_e1029_d_n19 * s.v[842]) + (eq7_e1029 * s.dn[842][19]));
        let eq7_e1031_d_n20: f64 = ((eq7_e1029_d_n20 * s.v[842]) + (eq7_e1029 * s.dn[842][20]));
        let eq7_e1031_d_b0: f64 = ((eq7_e1029_d_b0 * s.v[842]) + (eq7_e1029 * s.db[842][0]));
        let eq7_e1031_d_b1: f64 = ((eq7_e1029_d_b1 * s.v[842]) + (eq7_e1029 * s.db[842][1]));
        let eq7_e1031_d_b2: f64 = ((eq7_e1029_d_b2 * s.v[842]) + (eq7_e1029 * s.db[842][2]));
        let eq7_e1031_d_b3: f64 = ((eq7_e1029_d_b3 * s.v[842]) + (eq7_e1029 * s.db[842][3]));
        let eq7_e1031_d_b4: f64 = ((eq7_e1029_d_b4 * s.v[842]) + (eq7_e1029 * s.db[842][4]));
        let eq7_e1031_d_b5: f64 = ((eq7_e1029_d_b5 * s.v[842]) + (eq7_e1029 * s.db[842][5]));
        let eq7_e1031_d_b6: f64 = ((eq7_e1029_d_b6 * s.v[842]) + (eq7_e1029 * s.db[842][6]));
        let eq7_e1031_d_b7: f64 = ((eq7_e1029_d_b7 * s.v[842]) + (eq7_e1029 * s.db[842][7]));
        let eq7_e1031_d_b8: f64 = ((eq7_e1029_d_b8 * s.v[842]) + (eq7_e1029 * s.db[842][8]));
        let eq7_e1031_d_b9: f64 = ((eq7_e1029_d_b9 * s.v[842]) + (eq7_e1029 * s.db[842][9]));
        let eq7_e1031_d_b10: f64 = ((eq7_e1029_d_b10 * s.v[842]) + (eq7_e1029 * s.db[842][10]));
        let eq7_e1031_d_b11: f64 = ((eq7_e1029_d_b11 * s.v[842]) + (eq7_e1029 * s.db[842][11]));
        let eq7_e1031_d_b12: f64 = ((eq7_e1029_d_b12 * s.v[842]) + (eq7_e1029 * s.db[842][12]));
        let eq7_e1031_d_b13: f64 = ((eq7_e1029_d_b13 * s.v[842]) + (eq7_e1029 * s.db[842][13]));
        let eq7_e1031_d_b14: f64 = ((eq7_e1029_d_b14 * s.v[842]) + (eq7_e1029 * s.db[842][14]));
        let eq7_e1031_d_b15: f64 = ((eq7_e1029_d_b15 * s.v[842]) + (eq7_e1029 * s.db[842][15]));
        let eq7_e1031_d_b16: f64 = ((eq7_e1029_d_b16 * s.v[842]) + (eq7_e1029 * s.db[842][16]));
        let eq7_e1031_d_b17: f64 = ((eq7_e1029_d_b17 * s.v[842]) + (eq7_e1029 * s.db[842][17]));
        let eq7_e1031_d_b18: f64 = ((eq7_e1029_d_b18 * s.v[842]) + (eq7_e1029 * s.db[842][18]));
        let eq7_e1031_d_b19: f64 = ((eq7_e1029_d_b19 * s.v[842]) + (eq7_e1029 * s.db[842][19]));
        let eq7_e1031_d_b20: f64 = ((eq7_e1029_d_b20 * s.v[842]) + (eq7_e1029 * s.db[842][20]));
        let eq7_e1031_d_b21: f64 = ((eq7_e1029_d_b21 * s.v[842]) + (eq7_e1029 * s.db[842][21]));
        let eq7_e1031_d_b22: f64 = ((eq7_e1029_d_b22 * s.v[842]) + (eq7_e1029 * s.db[842][22]));
        let eq7_e1031_d_b23: f64 = ((eq7_e1029_d_b23 * s.v[842]) + (eq7_e1029 * s.db[842][23]));
        let eq7_e1031_d_b24: f64 = ((eq7_e1029_d_b24 * s.v[842]) + (eq7_e1029 * s.db[842][24]));
        (eq7_e1031, eq7_e1031_d_n0, eq7_e1031_d_n1, eq7_e1031_d_n2, eq7_e1031_d_n3, eq7_e1031_d_n4, eq7_e1031_d_n5, eq7_e1031_d_n6, eq7_e1031_d_n7, eq7_e1031_d_n8, eq7_e1031_d_n9, eq7_e1031_d_n10, eq7_e1031_d_n11, eq7_e1031_d_n12, eq7_e1031_d_n13, eq7_e1031_d_n14, eq7_e1031_d_n15, eq7_e1031_d_n16, eq7_e1031_d_n17, eq7_e1031_d_n18, eq7_e1031_d_n19, eq7_e1031_d_n20, eq7_e1031_d_b0, eq7_e1031_d_b1, eq7_e1031_d_b2, eq7_e1031_d_b3, eq7_e1031_d_b4, eq7_e1031_d_b5, eq7_e1031_d_b6, eq7_e1031_d_b7, eq7_e1031_d_b8, eq7_e1031_d_b9, eq7_e1031_d_b10, eq7_e1031_d_b11, eq7_e1031_d_b12, eq7_e1031_d_b13, eq7_e1031_d_b14, eq7_e1031_d_b15, eq7_e1031_d_b16, eq7_e1031_d_b17, eq7_e1031_d_b18, eq7_e1031_d_b19, eq7_e1031_d_b20, eq7_e1031_d_b21, eq7_e1031_d_b22, eq7_e1031_d_b23, eq7_e1031_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1033;
        let eq7_node_derivatives: [f64; 21] = [eq7_e1033_d_n0, eq7_e1033_d_n1, eq7_e1033_d_n2, eq7_e1033_d_n3, eq7_e1033_d_n4, eq7_e1033_d_n5, eq7_e1033_d_n6, eq7_e1033_d_n7, eq7_e1033_d_n8, eq7_e1033_d_n9, eq7_e1033_d_n10, eq7_e1033_d_n11, eq7_e1033_d_n12, eq7_e1033_d_n13, eq7_e1033_d_n14, eq7_e1033_d_n15, eq7_e1033_d_n16, eq7_e1033_d_n17, eq7_e1033_d_n18, eq7_e1033_d_n19, eq7_e1033_d_n20];
        let eq7_branch_derivatives: [f64; 25] = [eq7_e1033_d_b0, eq7_e1033_d_b1, eq7_e1033_d_b2, eq7_e1033_d_b3, eq7_e1033_d_b4, eq7_e1033_d_b5, eq7_e1033_d_b6, eq7_e1033_d_b7, eq7_e1033_d_b8, eq7_e1033_d_b9, eq7_e1033_d_b10, eq7_e1033_d_b11, eq7_e1033_d_b12, eq7_e1033_d_b13, eq7_e1033_d_b14, eq7_e1033_d_b15, eq7_e1033_d_b16, eq7_e1033_d_b17, eq7_e1033_d_b18, eq7_e1033_d_b19, eq7_e1033_d_b20, eq7_e1033_d_b21, eq7_e1033_d_b22, eq7_e1033_d_b23, eq7_e1033_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq7_value),
            &nodes,
            &eq7_node_derivatives,
            &branches,
            &eq7_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_8_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq8_e1036: f64 = (s.v[0] * s.v[19]);
        let eq8_e1036_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq8_e1036_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq8_e1036_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq8_e1036_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq8_e1036_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq8_e1036_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq8_e1036_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq8_e1036_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq8_e1036_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq8_e1036_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq8_e1036_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq8_e1036_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq8_e1036_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq8_e1036_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq8_e1036_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq8_e1036_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq8_e1036_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq8_e1036_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq8_e1036_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq8_e1036_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq8_e1036_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq8_e1036_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq8_e1036_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq8_e1036_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq8_e1036_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq8_e1036_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq8_e1036_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq8_e1036_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq8_e1036_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq8_e1036_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq8_e1036_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq8_e1036_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq8_e1036_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq8_e1036_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq8_e1036_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq8_e1036_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq8_e1036_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq8_e1036_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq8_e1036_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq8_e1036_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq8_e1036_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq8_e1036_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq8_e1036_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq8_e1036_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq8_e1036_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq8_e1036_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq8_e1038: f64 = (eq8_e1036 * p.p32);
        let eq8_e1038_d_n0: f64 = (eq8_e1036_d_n0 * p.p32);
        let eq8_e1038_d_n1: f64 = (eq8_e1036_d_n1 * p.p32);
        let eq8_e1038_d_n2: f64 = (eq8_e1036_d_n2 * p.p32);
        let eq8_e1038_d_n3: f64 = (eq8_e1036_d_n3 * p.p32);
        let eq8_e1038_d_n4: f64 = (eq8_e1036_d_n4 * p.p32);
        let eq8_e1038_d_n5: f64 = (eq8_e1036_d_n5 * p.p32);
        let eq8_e1038_d_n6: f64 = (eq8_e1036_d_n6 * p.p32);
        let eq8_e1038_d_n7: f64 = (eq8_e1036_d_n7 * p.p32);
        let eq8_e1038_d_n8: f64 = (eq8_e1036_d_n8 * p.p32);
        let eq8_e1038_d_n9: f64 = (eq8_e1036_d_n9 * p.p32);
        let eq8_e1038_d_n10: f64 = (eq8_e1036_d_n10 * p.p32);
        let eq8_e1038_d_n11: f64 = (eq8_e1036_d_n11 * p.p32);
        let eq8_e1038_d_n12: f64 = (eq8_e1036_d_n12 * p.p32);
        let eq8_e1038_d_n13: f64 = (eq8_e1036_d_n13 * p.p32);
        let eq8_e1038_d_n14: f64 = (eq8_e1036_d_n14 * p.p32);
        let eq8_e1038_d_n15: f64 = (eq8_e1036_d_n15 * p.p32);
        let eq8_e1038_d_n16: f64 = (eq8_e1036_d_n16 * p.p32);
        let eq8_e1038_d_n17: f64 = (eq8_e1036_d_n17 * p.p32);
        let eq8_e1038_d_n18: f64 = (eq8_e1036_d_n18 * p.p32);
        let eq8_e1038_d_n19: f64 = (eq8_e1036_d_n19 * p.p32);
        let eq8_e1038_d_n20: f64 = (eq8_e1036_d_n20 * p.p32);
        let eq8_e1038_d_b0: f64 = (eq8_e1036_d_b0 * p.p32);
        let eq8_e1038_d_b1: f64 = (eq8_e1036_d_b1 * p.p32);
        let eq8_e1038_d_b2: f64 = (eq8_e1036_d_b2 * p.p32);
        let eq8_e1038_d_b3: f64 = (eq8_e1036_d_b3 * p.p32);
        let eq8_e1038_d_b4: f64 = (eq8_e1036_d_b4 * p.p32);
        let eq8_e1038_d_b5: f64 = (eq8_e1036_d_b5 * p.p32);
        let eq8_e1038_d_b6: f64 = (eq8_e1036_d_b6 * p.p32);
        let eq8_e1038_d_b7: f64 = (eq8_e1036_d_b7 * p.p32);
        let eq8_e1038_d_b8: f64 = (eq8_e1036_d_b8 * p.p32);
        let eq8_e1038_d_b9: f64 = (eq8_e1036_d_b9 * p.p32);
        let eq8_e1038_d_b10: f64 = (eq8_e1036_d_b10 * p.p32);
        let eq8_e1038_d_b11: f64 = (eq8_e1036_d_b11 * p.p32);
        let eq8_e1038_d_b12: f64 = (eq8_e1036_d_b12 * p.p32);
        let eq8_e1038_d_b13: f64 = (eq8_e1036_d_b13 * p.p32);
        let eq8_e1038_d_b14: f64 = (eq8_e1036_d_b14 * p.p32);
        let eq8_e1038_d_b15: f64 = (eq8_e1036_d_b15 * p.p32);
        let eq8_e1038_d_b16: f64 = (eq8_e1036_d_b16 * p.p32);
        let eq8_e1038_d_b17: f64 = (eq8_e1036_d_b17 * p.p32);
        let eq8_e1038_d_b18: f64 = (eq8_e1036_d_b18 * p.p32);
        let eq8_e1038_d_b19: f64 = (eq8_e1036_d_b19 * p.p32);
        let eq8_e1038_d_b20: f64 = (eq8_e1036_d_b20 * p.p32);
        let eq8_e1038_d_b21: f64 = (eq8_e1036_d_b21 * p.p32);
        let eq8_e1038_d_b22: f64 = (eq8_e1036_d_b22 * p.p32);
        let eq8_e1038_d_b23: f64 = (eq8_e1036_d_b23 * p.p32);
        let eq8_e1038_d_b24: f64 = (eq8_e1036_d_b24 * p.p32);
        let eq8_e1040: f64 = (eq8_e1038 * s.v[843]);
        let eq8_e1040_d_n0: f64 = ((eq8_e1038_d_n0 * s.v[843]) + (eq8_e1038 * s.dn[843][0]));
        let eq8_e1040_d_n1: f64 = ((eq8_e1038_d_n1 * s.v[843]) + (eq8_e1038 * s.dn[843][1]));
        let eq8_e1040_d_n2: f64 = ((eq8_e1038_d_n2 * s.v[843]) + (eq8_e1038 * s.dn[843][2]));
        let eq8_e1040_d_n3: f64 = ((eq8_e1038_d_n3 * s.v[843]) + (eq8_e1038 * s.dn[843][3]));
        let eq8_e1040_d_n4: f64 = ((eq8_e1038_d_n4 * s.v[843]) + (eq8_e1038 * s.dn[843][4]));
        let eq8_e1040_d_n5: f64 = ((eq8_e1038_d_n5 * s.v[843]) + (eq8_e1038 * s.dn[843][5]));
        let eq8_e1040_d_n6: f64 = ((eq8_e1038_d_n6 * s.v[843]) + (eq8_e1038 * s.dn[843][6]));
        let eq8_e1040_d_n7: f64 = ((eq8_e1038_d_n7 * s.v[843]) + (eq8_e1038 * s.dn[843][7]));
        let eq8_e1040_d_n8: f64 = ((eq8_e1038_d_n8 * s.v[843]) + (eq8_e1038 * s.dn[843][8]));
        let eq8_e1040_d_n9: f64 = ((eq8_e1038_d_n9 * s.v[843]) + (eq8_e1038 * s.dn[843][9]));
        let eq8_e1040_d_n10: f64 = ((eq8_e1038_d_n10 * s.v[843]) + (eq8_e1038 * s.dn[843][10]));
        let eq8_e1040_d_n11: f64 = ((eq8_e1038_d_n11 * s.v[843]) + (eq8_e1038 * s.dn[843][11]));
        let eq8_e1040_d_n12: f64 = ((eq8_e1038_d_n12 * s.v[843]) + (eq8_e1038 * s.dn[843][12]));
        let eq8_e1040_d_n13: f64 = ((eq8_e1038_d_n13 * s.v[843]) + (eq8_e1038 * s.dn[843][13]));
        let eq8_e1040_d_n14: f64 = ((eq8_e1038_d_n14 * s.v[843]) + (eq8_e1038 * s.dn[843][14]));
        let eq8_e1040_d_n15: f64 = ((eq8_e1038_d_n15 * s.v[843]) + (eq8_e1038 * s.dn[843][15]));
        let eq8_e1040_d_n16: f64 = ((eq8_e1038_d_n16 * s.v[843]) + (eq8_e1038 * s.dn[843][16]));
        let eq8_e1040_d_n17: f64 = ((eq8_e1038_d_n17 * s.v[843]) + (eq8_e1038 * s.dn[843][17]));
        let eq8_e1040_d_n18: f64 = ((eq8_e1038_d_n18 * s.v[843]) + (eq8_e1038 * s.dn[843][18]));
        let eq8_e1040_d_n19: f64 = ((eq8_e1038_d_n19 * s.v[843]) + (eq8_e1038 * s.dn[843][19]));
        let eq8_e1040_d_n20: f64 = ((eq8_e1038_d_n20 * s.v[843]) + (eq8_e1038 * s.dn[843][20]));
        let eq8_e1040_d_b0: f64 = ((eq8_e1038_d_b0 * s.v[843]) + (eq8_e1038 * s.db[843][0]));
        let eq8_e1040_d_b1: f64 = ((eq8_e1038_d_b1 * s.v[843]) + (eq8_e1038 * s.db[843][1]));
        let eq8_e1040_d_b2: f64 = ((eq8_e1038_d_b2 * s.v[843]) + (eq8_e1038 * s.db[843][2]));
        let eq8_e1040_d_b3: f64 = ((eq8_e1038_d_b3 * s.v[843]) + (eq8_e1038 * s.db[843][3]));
        let eq8_e1040_d_b4: f64 = ((eq8_e1038_d_b4 * s.v[843]) + (eq8_e1038 * s.db[843][4]));
        let eq8_e1040_d_b5: f64 = ((eq8_e1038_d_b5 * s.v[843]) + (eq8_e1038 * s.db[843][5]));
        let eq8_e1040_d_b6: f64 = ((eq8_e1038_d_b6 * s.v[843]) + (eq8_e1038 * s.db[843][6]));
        let eq8_e1040_d_b7: f64 = ((eq8_e1038_d_b7 * s.v[843]) + (eq8_e1038 * s.db[843][7]));
        let eq8_e1040_d_b8: f64 = ((eq8_e1038_d_b8 * s.v[843]) + (eq8_e1038 * s.db[843][8]));
        let eq8_e1040_d_b9: f64 = ((eq8_e1038_d_b9 * s.v[843]) + (eq8_e1038 * s.db[843][9]));
        let eq8_e1040_d_b10: f64 = ((eq8_e1038_d_b10 * s.v[843]) + (eq8_e1038 * s.db[843][10]));
        let eq8_e1040_d_b11: f64 = ((eq8_e1038_d_b11 * s.v[843]) + (eq8_e1038 * s.db[843][11]));
        let eq8_e1040_d_b12: f64 = ((eq8_e1038_d_b12 * s.v[843]) + (eq8_e1038 * s.db[843][12]));
        let eq8_e1040_d_b13: f64 = ((eq8_e1038_d_b13 * s.v[843]) + (eq8_e1038 * s.db[843][13]));
        let eq8_e1040_d_b14: f64 = ((eq8_e1038_d_b14 * s.v[843]) + (eq8_e1038 * s.db[843][14]));
        let eq8_e1040_d_b15: f64 = ((eq8_e1038_d_b15 * s.v[843]) + (eq8_e1038 * s.db[843][15]));
        let eq8_e1040_d_b16: f64 = ((eq8_e1038_d_b16 * s.v[843]) + (eq8_e1038 * s.db[843][16]));
        let eq8_e1040_d_b17: f64 = ((eq8_e1038_d_b17 * s.v[843]) + (eq8_e1038 * s.db[843][17]));
        let eq8_e1040_d_b18: f64 = ((eq8_e1038_d_b18 * s.v[843]) + (eq8_e1038 * s.db[843][18]));
        let eq8_e1040_d_b19: f64 = ((eq8_e1038_d_b19 * s.v[843]) + (eq8_e1038 * s.db[843][19]));
        let eq8_e1040_d_b20: f64 = ((eq8_e1038_d_b20 * s.v[843]) + (eq8_e1038 * s.db[843][20]));
        let eq8_e1040_d_b21: f64 = ((eq8_e1038_d_b21 * s.v[843]) + (eq8_e1038 * s.db[843][21]));
        let eq8_e1040_d_b22: f64 = ((eq8_e1038_d_b22 * s.v[843]) + (eq8_e1038 * s.db[843][22]));
        let eq8_e1040_d_b23: f64 = ((eq8_e1038_d_b23 * s.v[843]) + (eq8_e1038 * s.db[843][23]));
        let eq8_e1040_d_b24: f64 = ((eq8_e1038_d_b24 * s.v[843]) + (eq8_e1038 * s.db[843][24]));
        let eq8_value: f64 = eq8_e1040;
        let eq8_node_derivatives: [f64; 21] = [eq8_e1040_d_n0, eq8_e1040_d_n1, eq8_e1040_d_n2, eq8_e1040_d_n3, eq8_e1040_d_n4, eq8_e1040_d_n5, eq8_e1040_d_n6, eq8_e1040_d_n7, eq8_e1040_d_n8, eq8_e1040_d_n9, eq8_e1040_d_n10, eq8_e1040_d_n11, eq8_e1040_d_n12, eq8_e1040_d_n13, eq8_e1040_d_n14, eq8_e1040_d_n15, eq8_e1040_d_n16, eq8_e1040_d_n17, eq8_e1040_d_n18, eq8_e1040_d_n19, eq8_e1040_d_n20];
        let eq8_branch_derivatives: [f64; 25] = [eq8_e1040_d_b0, eq8_e1040_d_b1, eq8_e1040_d_b2, eq8_e1040_d_b3, eq8_e1040_d_b4, eq8_e1040_d_b5, eq8_e1040_d_b6, eq8_e1040_d_b7, eq8_e1040_d_b8, eq8_e1040_d_b9, eq8_e1040_d_b10, eq8_e1040_d_b11, eq8_e1040_d_b12, eq8_e1040_d_b13, eq8_e1040_d_b14, eq8_e1040_d_b15, eq8_e1040_d_b16, eq8_e1040_d_b17, eq8_e1040_d_b18, eq8_e1040_d_b19, eq8_e1040_d_b20, eq8_e1040_d_b21, eq8_e1040_d_b22, eq8_e1040_d_b23, eq8_e1040_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[8]),
            self.multiplicity * (eq8_value),
            &nodes,
            &eq8_node_derivatives,
            &branches,
            &eq8_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq9_e1043: f64 = (s.v[0] * s.v[19]);
        let eq9_e1043_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq9_e1043_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq9_e1043_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq9_e1043_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq9_e1043_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq9_e1043_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq9_e1043_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq9_e1043_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq9_e1043_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq9_e1043_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq9_e1043_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq9_e1043_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq9_e1043_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq9_e1043_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq9_e1043_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq9_e1043_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq9_e1043_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq9_e1043_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq9_e1043_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq9_e1043_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq9_e1043_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq9_e1043_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq9_e1043_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq9_e1043_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq9_e1043_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq9_e1043_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq9_e1043_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq9_e1043_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq9_e1043_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq9_e1043_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq9_e1043_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq9_e1043_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq9_e1043_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq9_e1043_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq9_e1043_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq9_e1043_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq9_e1043_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq9_e1043_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq9_e1043_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq9_e1043_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq9_e1043_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq9_e1043_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq9_e1043_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq9_e1043_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq9_e1043_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq9_e1043_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq9_e1045: f64 = (eq9_e1043 * p.p32);
        let eq9_e1045_d_n0: f64 = (eq9_e1043_d_n0 * p.p32);
        let eq9_e1045_d_n1: f64 = (eq9_e1043_d_n1 * p.p32);
        let eq9_e1045_d_n2: f64 = (eq9_e1043_d_n2 * p.p32);
        let eq9_e1045_d_n3: f64 = (eq9_e1043_d_n3 * p.p32);
        let eq9_e1045_d_n4: f64 = (eq9_e1043_d_n4 * p.p32);
        let eq9_e1045_d_n5: f64 = (eq9_e1043_d_n5 * p.p32);
        let eq9_e1045_d_n6: f64 = (eq9_e1043_d_n6 * p.p32);
        let eq9_e1045_d_n7: f64 = (eq9_e1043_d_n7 * p.p32);
        let eq9_e1045_d_n8: f64 = (eq9_e1043_d_n8 * p.p32);
        let eq9_e1045_d_n9: f64 = (eq9_e1043_d_n9 * p.p32);
        let eq9_e1045_d_n10: f64 = (eq9_e1043_d_n10 * p.p32);
        let eq9_e1045_d_n11: f64 = (eq9_e1043_d_n11 * p.p32);
        let eq9_e1045_d_n12: f64 = (eq9_e1043_d_n12 * p.p32);
        let eq9_e1045_d_n13: f64 = (eq9_e1043_d_n13 * p.p32);
        let eq9_e1045_d_n14: f64 = (eq9_e1043_d_n14 * p.p32);
        let eq9_e1045_d_n15: f64 = (eq9_e1043_d_n15 * p.p32);
        let eq9_e1045_d_n16: f64 = (eq9_e1043_d_n16 * p.p32);
        let eq9_e1045_d_n17: f64 = (eq9_e1043_d_n17 * p.p32);
        let eq9_e1045_d_n18: f64 = (eq9_e1043_d_n18 * p.p32);
        let eq9_e1045_d_n19: f64 = (eq9_e1043_d_n19 * p.p32);
        let eq9_e1045_d_n20: f64 = (eq9_e1043_d_n20 * p.p32);
        let eq9_e1045_d_b0: f64 = (eq9_e1043_d_b0 * p.p32);
        let eq9_e1045_d_b1: f64 = (eq9_e1043_d_b1 * p.p32);
        let eq9_e1045_d_b2: f64 = (eq9_e1043_d_b2 * p.p32);
        let eq9_e1045_d_b3: f64 = (eq9_e1043_d_b3 * p.p32);
        let eq9_e1045_d_b4: f64 = (eq9_e1043_d_b4 * p.p32);
        let eq9_e1045_d_b5: f64 = (eq9_e1043_d_b5 * p.p32);
        let eq9_e1045_d_b6: f64 = (eq9_e1043_d_b6 * p.p32);
        let eq9_e1045_d_b7: f64 = (eq9_e1043_d_b7 * p.p32);
        let eq9_e1045_d_b8: f64 = (eq9_e1043_d_b8 * p.p32);
        let eq9_e1045_d_b9: f64 = (eq9_e1043_d_b9 * p.p32);
        let eq9_e1045_d_b10: f64 = (eq9_e1043_d_b10 * p.p32);
        let eq9_e1045_d_b11: f64 = (eq9_e1043_d_b11 * p.p32);
        let eq9_e1045_d_b12: f64 = (eq9_e1043_d_b12 * p.p32);
        let eq9_e1045_d_b13: f64 = (eq9_e1043_d_b13 * p.p32);
        let eq9_e1045_d_b14: f64 = (eq9_e1043_d_b14 * p.p32);
        let eq9_e1045_d_b15: f64 = (eq9_e1043_d_b15 * p.p32);
        let eq9_e1045_d_b16: f64 = (eq9_e1043_d_b16 * p.p32);
        let eq9_e1045_d_b17: f64 = (eq9_e1043_d_b17 * p.p32);
        let eq9_e1045_d_b18: f64 = (eq9_e1043_d_b18 * p.p32);
        let eq9_e1045_d_b19: f64 = (eq9_e1043_d_b19 * p.p32);
        let eq9_e1045_d_b20: f64 = (eq9_e1043_d_b20 * p.p32);
        let eq9_e1045_d_b21: f64 = (eq9_e1043_d_b21 * p.p32);
        let eq9_e1045_d_b22: f64 = (eq9_e1043_d_b22 * p.p32);
        let eq9_e1045_d_b23: f64 = (eq9_e1043_d_b23 * p.p32);
        let eq9_e1045_d_b24: f64 = (eq9_e1043_d_b24 * p.p32);
        let eq9_e1047: f64 = (eq9_e1045 * s.v[839]);
        let eq9_e1047_d_n0: f64 = ((eq9_e1045_d_n0 * s.v[839]) + (eq9_e1045 * s.dn[839][0]));
        let eq9_e1047_d_n1: f64 = ((eq9_e1045_d_n1 * s.v[839]) + (eq9_e1045 * s.dn[839][1]));
        let eq9_e1047_d_n2: f64 = ((eq9_e1045_d_n2 * s.v[839]) + (eq9_e1045 * s.dn[839][2]));
        let eq9_e1047_d_n3: f64 = ((eq9_e1045_d_n3 * s.v[839]) + (eq9_e1045 * s.dn[839][3]));
        let eq9_e1047_d_n4: f64 = ((eq9_e1045_d_n4 * s.v[839]) + (eq9_e1045 * s.dn[839][4]));
        let eq9_e1047_d_n5: f64 = ((eq9_e1045_d_n5 * s.v[839]) + (eq9_e1045 * s.dn[839][5]));
        let eq9_e1047_d_n6: f64 = ((eq9_e1045_d_n6 * s.v[839]) + (eq9_e1045 * s.dn[839][6]));
        let eq9_e1047_d_n7: f64 = ((eq9_e1045_d_n7 * s.v[839]) + (eq9_e1045 * s.dn[839][7]));
        let eq9_e1047_d_n8: f64 = ((eq9_e1045_d_n8 * s.v[839]) + (eq9_e1045 * s.dn[839][8]));
        let eq9_e1047_d_n9: f64 = ((eq9_e1045_d_n9 * s.v[839]) + (eq9_e1045 * s.dn[839][9]));
        let eq9_e1047_d_n10: f64 = ((eq9_e1045_d_n10 * s.v[839]) + (eq9_e1045 * s.dn[839][10]));
        let eq9_e1047_d_n11: f64 = ((eq9_e1045_d_n11 * s.v[839]) + (eq9_e1045 * s.dn[839][11]));
        let eq9_e1047_d_n12: f64 = ((eq9_e1045_d_n12 * s.v[839]) + (eq9_e1045 * s.dn[839][12]));
        let eq9_e1047_d_n13: f64 = ((eq9_e1045_d_n13 * s.v[839]) + (eq9_e1045 * s.dn[839][13]));
        let eq9_e1047_d_n14: f64 = ((eq9_e1045_d_n14 * s.v[839]) + (eq9_e1045 * s.dn[839][14]));
        let eq9_e1047_d_n15: f64 = ((eq9_e1045_d_n15 * s.v[839]) + (eq9_e1045 * s.dn[839][15]));
        let eq9_e1047_d_n16: f64 = ((eq9_e1045_d_n16 * s.v[839]) + (eq9_e1045 * s.dn[839][16]));
        let eq9_e1047_d_n17: f64 = ((eq9_e1045_d_n17 * s.v[839]) + (eq9_e1045 * s.dn[839][17]));
        let eq9_e1047_d_n18: f64 = ((eq9_e1045_d_n18 * s.v[839]) + (eq9_e1045 * s.dn[839][18]));
        let eq9_e1047_d_n19: f64 = ((eq9_e1045_d_n19 * s.v[839]) + (eq9_e1045 * s.dn[839][19]));
        let eq9_e1047_d_n20: f64 = ((eq9_e1045_d_n20 * s.v[839]) + (eq9_e1045 * s.dn[839][20]));
        let eq9_e1047_d_b0: f64 = ((eq9_e1045_d_b0 * s.v[839]) + (eq9_e1045 * s.db[839][0]));
        let eq9_e1047_d_b1: f64 = ((eq9_e1045_d_b1 * s.v[839]) + (eq9_e1045 * s.db[839][1]));
        let eq9_e1047_d_b2: f64 = ((eq9_e1045_d_b2 * s.v[839]) + (eq9_e1045 * s.db[839][2]));
        let eq9_e1047_d_b3: f64 = ((eq9_e1045_d_b3 * s.v[839]) + (eq9_e1045 * s.db[839][3]));
        let eq9_e1047_d_b4: f64 = ((eq9_e1045_d_b4 * s.v[839]) + (eq9_e1045 * s.db[839][4]));
        let eq9_e1047_d_b5: f64 = ((eq9_e1045_d_b5 * s.v[839]) + (eq9_e1045 * s.db[839][5]));
        let eq9_e1047_d_b6: f64 = ((eq9_e1045_d_b6 * s.v[839]) + (eq9_e1045 * s.db[839][6]));
        let eq9_e1047_d_b7: f64 = ((eq9_e1045_d_b7 * s.v[839]) + (eq9_e1045 * s.db[839][7]));
        let eq9_e1047_d_b8: f64 = ((eq9_e1045_d_b8 * s.v[839]) + (eq9_e1045 * s.db[839][8]));
        let eq9_e1047_d_b9: f64 = ((eq9_e1045_d_b9 * s.v[839]) + (eq9_e1045 * s.db[839][9]));
        let eq9_e1047_d_b10: f64 = ((eq9_e1045_d_b10 * s.v[839]) + (eq9_e1045 * s.db[839][10]));
        let eq9_e1047_d_b11: f64 = ((eq9_e1045_d_b11 * s.v[839]) + (eq9_e1045 * s.db[839][11]));
        let eq9_e1047_d_b12: f64 = ((eq9_e1045_d_b12 * s.v[839]) + (eq9_e1045 * s.db[839][12]));
        let eq9_e1047_d_b13: f64 = ((eq9_e1045_d_b13 * s.v[839]) + (eq9_e1045 * s.db[839][13]));
        let eq9_e1047_d_b14: f64 = ((eq9_e1045_d_b14 * s.v[839]) + (eq9_e1045 * s.db[839][14]));
        let eq9_e1047_d_b15: f64 = ((eq9_e1045_d_b15 * s.v[839]) + (eq9_e1045 * s.db[839][15]));
        let eq9_e1047_d_b16: f64 = ((eq9_e1045_d_b16 * s.v[839]) + (eq9_e1045 * s.db[839][16]));
        let eq9_e1047_d_b17: f64 = ((eq9_e1045_d_b17 * s.v[839]) + (eq9_e1045 * s.db[839][17]));
        let eq9_e1047_d_b18: f64 = ((eq9_e1045_d_b18 * s.v[839]) + (eq9_e1045 * s.db[839][18]));
        let eq9_e1047_d_b19: f64 = ((eq9_e1045_d_b19 * s.v[839]) + (eq9_e1045 * s.db[839][19]));
        let eq9_e1047_d_b20: f64 = ((eq9_e1045_d_b20 * s.v[839]) + (eq9_e1045 * s.db[839][20]));
        let eq9_e1047_d_b21: f64 = ((eq9_e1045_d_b21 * s.v[839]) + (eq9_e1045 * s.db[839][21]));
        let eq9_e1047_d_b22: f64 = ((eq9_e1045_d_b22 * s.v[839]) + (eq9_e1045 * s.db[839][22]));
        let eq9_e1047_d_b23: f64 = ((eq9_e1045_d_b23 * s.v[839]) + (eq9_e1045 * s.db[839][23]));
        let eq9_e1047_d_b24: f64 = ((eq9_e1045_d_b24 * s.v[839]) + (eq9_e1045 * s.db[839][24]));
        let eq9_value: f64 = eq9_e1047;
        let eq9_node_derivatives: [f64; 21] = [eq9_e1047_d_n0, eq9_e1047_d_n1, eq9_e1047_d_n2, eq9_e1047_d_n3, eq9_e1047_d_n4, eq9_e1047_d_n5, eq9_e1047_d_n6, eq9_e1047_d_n7, eq9_e1047_d_n8, eq9_e1047_d_n9, eq9_e1047_d_n10, eq9_e1047_d_n11, eq9_e1047_d_n12, eq9_e1047_d_n13, eq9_e1047_d_n14, eq9_e1047_d_n15, eq9_e1047_d_n16, eq9_e1047_d_n17, eq9_e1047_d_n18, eq9_e1047_d_n19, eq9_e1047_d_n20];
        let eq9_branch_derivatives: [f64; 25] = [eq9_e1047_d_b0, eq9_e1047_d_b1, eq9_e1047_d_b2, eq9_e1047_d_b3, eq9_e1047_d_b4, eq9_e1047_d_b5, eq9_e1047_d_b6, eq9_e1047_d_b7, eq9_e1047_d_b8, eq9_e1047_d_b9, eq9_e1047_d_b10, eq9_e1047_d_b11, eq9_e1047_d_b12, eq9_e1047_d_b13, eq9_e1047_d_b14, eq9_e1047_d_b15, eq9_e1047_d_b16, eq9_e1047_d_b17, eq9_e1047_d_b18, eq9_e1047_d_b19, eq9_e1047_d_b20, eq9_e1047_d_b21, eq9_e1047_d_b22, eq9_e1047_d_b23, eq9_e1047_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq9_value),
            &nodes,
            &eq9_node_derivatives,
            &branches,
            &eq9_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq10_e1050: f64 = (s.v[0] * s.v[19]);
        let eq10_e1050_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq10_e1050_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq10_e1050_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq10_e1050_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq10_e1050_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq10_e1050_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq10_e1050_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq10_e1050_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq10_e1050_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq10_e1050_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq10_e1050_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq10_e1050_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq10_e1050_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq10_e1050_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq10_e1050_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq10_e1050_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq10_e1050_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq10_e1050_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq10_e1050_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq10_e1050_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq10_e1050_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq10_e1050_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq10_e1050_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq10_e1050_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq10_e1050_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq10_e1050_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq10_e1050_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq10_e1050_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq10_e1050_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq10_e1050_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq10_e1050_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq10_e1050_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq10_e1050_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq10_e1050_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq10_e1050_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq10_e1050_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq10_e1050_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq10_e1050_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq10_e1050_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq10_e1050_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq10_e1050_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq10_e1050_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq10_e1050_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq10_e1050_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq10_e1050_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq10_e1050_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq10_e1052: f64 = (eq10_e1050 * p.p32);
        let eq10_e1052_d_n0: f64 = (eq10_e1050_d_n0 * p.p32);
        let eq10_e1052_d_n1: f64 = (eq10_e1050_d_n1 * p.p32);
        let eq10_e1052_d_n2: f64 = (eq10_e1050_d_n2 * p.p32);
        let eq10_e1052_d_n3: f64 = (eq10_e1050_d_n3 * p.p32);
        let eq10_e1052_d_n4: f64 = (eq10_e1050_d_n4 * p.p32);
        let eq10_e1052_d_n5: f64 = (eq10_e1050_d_n5 * p.p32);
        let eq10_e1052_d_n6: f64 = (eq10_e1050_d_n6 * p.p32);
        let eq10_e1052_d_n7: f64 = (eq10_e1050_d_n7 * p.p32);
        let eq10_e1052_d_n8: f64 = (eq10_e1050_d_n8 * p.p32);
        let eq10_e1052_d_n9: f64 = (eq10_e1050_d_n9 * p.p32);
        let eq10_e1052_d_n10: f64 = (eq10_e1050_d_n10 * p.p32);
        let eq10_e1052_d_n11: f64 = (eq10_e1050_d_n11 * p.p32);
        let eq10_e1052_d_n12: f64 = (eq10_e1050_d_n12 * p.p32);
        let eq10_e1052_d_n13: f64 = (eq10_e1050_d_n13 * p.p32);
        let eq10_e1052_d_n14: f64 = (eq10_e1050_d_n14 * p.p32);
        let eq10_e1052_d_n15: f64 = (eq10_e1050_d_n15 * p.p32);
        let eq10_e1052_d_n16: f64 = (eq10_e1050_d_n16 * p.p32);
        let eq10_e1052_d_n17: f64 = (eq10_e1050_d_n17 * p.p32);
        let eq10_e1052_d_n18: f64 = (eq10_e1050_d_n18 * p.p32);
        let eq10_e1052_d_n19: f64 = (eq10_e1050_d_n19 * p.p32);
        let eq10_e1052_d_n20: f64 = (eq10_e1050_d_n20 * p.p32);
        let eq10_e1052_d_b0: f64 = (eq10_e1050_d_b0 * p.p32);
        let eq10_e1052_d_b1: f64 = (eq10_e1050_d_b1 * p.p32);
        let eq10_e1052_d_b2: f64 = (eq10_e1050_d_b2 * p.p32);
        let eq10_e1052_d_b3: f64 = (eq10_e1050_d_b3 * p.p32);
        let eq10_e1052_d_b4: f64 = (eq10_e1050_d_b4 * p.p32);
        let eq10_e1052_d_b5: f64 = (eq10_e1050_d_b5 * p.p32);
        let eq10_e1052_d_b6: f64 = (eq10_e1050_d_b6 * p.p32);
        let eq10_e1052_d_b7: f64 = (eq10_e1050_d_b7 * p.p32);
        let eq10_e1052_d_b8: f64 = (eq10_e1050_d_b8 * p.p32);
        let eq10_e1052_d_b9: f64 = (eq10_e1050_d_b9 * p.p32);
        let eq10_e1052_d_b10: f64 = (eq10_e1050_d_b10 * p.p32);
        let eq10_e1052_d_b11: f64 = (eq10_e1050_d_b11 * p.p32);
        let eq10_e1052_d_b12: f64 = (eq10_e1050_d_b12 * p.p32);
        let eq10_e1052_d_b13: f64 = (eq10_e1050_d_b13 * p.p32);
        let eq10_e1052_d_b14: f64 = (eq10_e1050_d_b14 * p.p32);
        let eq10_e1052_d_b15: f64 = (eq10_e1050_d_b15 * p.p32);
        let eq10_e1052_d_b16: f64 = (eq10_e1050_d_b16 * p.p32);
        let eq10_e1052_d_b17: f64 = (eq10_e1050_d_b17 * p.p32);
        let eq10_e1052_d_b18: f64 = (eq10_e1050_d_b18 * p.p32);
        let eq10_e1052_d_b19: f64 = (eq10_e1050_d_b19 * p.p32);
        let eq10_e1052_d_b20: f64 = (eq10_e1050_d_b20 * p.p32);
        let eq10_e1052_d_b21: f64 = (eq10_e1050_d_b21 * p.p32);
        let eq10_e1052_d_b22: f64 = (eq10_e1050_d_b22 * p.p32);
        let eq10_e1052_d_b23: f64 = (eq10_e1050_d_b23 * p.p32);
        let eq10_e1052_d_b24: f64 = (eq10_e1050_d_b24 * p.p32);
        let eq10_e1054: f64 = (eq10_e1052 * s.v[840]);
        let eq10_e1054_d_n0: f64 = ((eq10_e1052_d_n0 * s.v[840]) + (eq10_e1052 * s.dn[840][0]));
        let eq10_e1054_d_n1: f64 = ((eq10_e1052_d_n1 * s.v[840]) + (eq10_e1052 * s.dn[840][1]));
        let eq10_e1054_d_n2: f64 = ((eq10_e1052_d_n2 * s.v[840]) + (eq10_e1052 * s.dn[840][2]));
        let eq10_e1054_d_n3: f64 = ((eq10_e1052_d_n3 * s.v[840]) + (eq10_e1052 * s.dn[840][3]));
        let eq10_e1054_d_n4: f64 = ((eq10_e1052_d_n4 * s.v[840]) + (eq10_e1052 * s.dn[840][4]));
        let eq10_e1054_d_n5: f64 = ((eq10_e1052_d_n5 * s.v[840]) + (eq10_e1052 * s.dn[840][5]));
        let eq10_e1054_d_n6: f64 = ((eq10_e1052_d_n6 * s.v[840]) + (eq10_e1052 * s.dn[840][6]));
        let eq10_e1054_d_n7: f64 = ((eq10_e1052_d_n7 * s.v[840]) + (eq10_e1052 * s.dn[840][7]));
        let eq10_e1054_d_n8: f64 = ((eq10_e1052_d_n8 * s.v[840]) + (eq10_e1052 * s.dn[840][8]));
        let eq10_e1054_d_n9: f64 = ((eq10_e1052_d_n9 * s.v[840]) + (eq10_e1052 * s.dn[840][9]));
        let eq10_e1054_d_n10: f64 = ((eq10_e1052_d_n10 * s.v[840]) + (eq10_e1052 * s.dn[840][10]));
        let eq10_e1054_d_n11: f64 = ((eq10_e1052_d_n11 * s.v[840]) + (eq10_e1052 * s.dn[840][11]));
        let eq10_e1054_d_n12: f64 = ((eq10_e1052_d_n12 * s.v[840]) + (eq10_e1052 * s.dn[840][12]));
        let eq10_e1054_d_n13: f64 = ((eq10_e1052_d_n13 * s.v[840]) + (eq10_e1052 * s.dn[840][13]));
        let eq10_e1054_d_n14: f64 = ((eq10_e1052_d_n14 * s.v[840]) + (eq10_e1052 * s.dn[840][14]));
        let eq10_e1054_d_n15: f64 = ((eq10_e1052_d_n15 * s.v[840]) + (eq10_e1052 * s.dn[840][15]));
        let eq10_e1054_d_n16: f64 = ((eq10_e1052_d_n16 * s.v[840]) + (eq10_e1052 * s.dn[840][16]));
        let eq10_e1054_d_n17: f64 = ((eq10_e1052_d_n17 * s.v[840]) + (eq10_e1052 * s.dn[840][17]));
        let eq10_e1054_d_n18: f64 = ((eq10_e1052_d_n18 * s.v[840]) + (eq10_e1052 * s.dn[840][18]));
        let eq10_e1054_d_n19: f64 = ((eq10_e1052_d_n19 * s.v[840]) + (eq10_e1052 * s.dn[840][19]));
        let eq10_e1054_d_n20: f64 = ((eq10_e1052_d_n20 * s.v[840]) + (eq10_e1052 * s.dn[840][20]));
        let eq10_e1054_d_b0: f64 = ((eq10_e1052_d_b0 * s.v[840]) + (eq10_e1052 * s.db[840][0]));
        let eq10_e1054_d_b1: f64 = ((eq10_e1052_d_b1 * s.v[840]) + (eq10_e1052 * s.db[840][1]));
        let eq10_e1054_d_b2: f64 = ((eq10_e1052_d_b2 * s.v[840]) + (eq10_e1052 * s.db[840][2]));
        let eq10_e1054_d_b3: f64 = ((eq10_e1052_d_b3 * s.v[840]) + (eq10_e1052 * s.db[840][3]));
        let eq10_e1054_d_b4: f64 = ((eq10_e1052_d_b4 * s.v[840]) + (eq10_e1052 * s.db[840][4]));
        let eq10_e1054_d_b5: f64 = ((eq10_e1052_d_b5 * s.v[840]) + (eq10_e1052 * s.db[840][5]));
        let eq10_e1054_d_b6: f64 = ((eq10_e1052_d_b6 * s.v[840]) + (eq10_e1052 * s.db[840][6]));
        let eq10_e1054_d_b7: f64 = ((eq10_e1052_d_b7 * s.v[840]) + (eq10_e1052 * s.db[840][7]));
        let eq10_e1054_d_b8: f64 = ((eq10_e1052_d_b8 * s.v[840]) + (eq10_e1052 * s.db[840][8]));
        let eq10_e1054_d_b9: f64 = ((eq10_e1052_d_b9 * s.v[840]) + (eq10_e1052 * s.db[840][9]));
        let eq10_e1054_d_b10: f64 = ((eq10_e1052_d_b10 * s.v[840]) + (eq10_e1052 * s.db[840][10]));
        let eq10_e1054_d_b11: f64 = ((eq10_e1052_d_b11 * s.v[840]) + (eq10_e1052 * s.db[840][11]));
        let eq10_e1054_d_b12: f64 = ((eq10_e1052_d_b12 * s.v[840]) + (eq10_e1052 * s.db[840][12]));
        let eq10_e1054_d_b13: f64 = ((eq10_e1052_d_b13 * s.v[840]) + (eq10_e1052 * s.db[840][13]));
        let eq10_e1054_d_b14: f64 = ((eq10_e1052_d_b14 * s.v[840]) + (eq10_e1052 * s.db[840][14]));
        let eq10_e1054_d_b15: f64 = ((eq10_e1052_d_b15 * s.v[840]) + (eq10_e1052 * s.db[840][15]));
        let eq10_e1054_d_b16: f64 = ((eq10_e1052_d_b16 * s.v[840]) + (eq10_e1052 * s.db[840][16]));
        let eq10_e1054_d_b17: f64 = ((eq10_e1052_d_b17 * s.v[840]) + (eq10_e1052 * s.db[840][17]));
        let eq10_e1054_d_b18: f64 = ((eq10_e1052_d_b18 * s.v[840]) + (eq10_e1052 * s.db[840][18]));
        let eq10_e1054_d_b19: f64 = ((eq10_e1052_d_b19 * s.v[840]) + (eq10_e1052 * s.db[840][19]));
        let eq10_e1054_d_b20: f64 = ((eq10_e1052_d_b20 * s.v[840]) + (eq10_e1052 * s.db[840][20]));
        let eq10_e1054_d_b21: f64 = ((eq10_e1052_d_b21 * s.v[840]) + (eq10_e1052 * s.db[840][21]));
        let eq10_e1054_d_b22: f64 = ((eq10_e1052_d_b22 * s.v[840]) + (eq10_e1052 * s.db[840][22]));
        let eq10_e1054_d_b23: f64 = ((eq10_e1052_d_b23 * s.v[840]) + (eq10_e1052 * s.db[840][23]));
        let eq10_e1054_d_b24: f64 = ((eq10_e1052_d_b24 * s.v[840]) + (eq10_e1052 * s.db[840][24]));
        let eq10_value: f64 = eq10_e1054;
        let eq10_node_derivatives: [f64; 21] = [eq10_e1054_d_n0, eq10_e1054_d_n1, eq10_e1054_d_n2, eq10_e1054_d_n3, eq10_e1054_d_n4, eq10_e1054_d_n5, eq10_e1054_d_n6, eq10_e1054_d_n7, eq10_e1054_d_n8, eq10_e1054_d_n9, eq10_e1054_d_n10, eq10_e1054_d_n11, eq10_e1054_d_n12, eq10_e1054_d_n13, eq10_e1054_d_n14, eq10_e1054_d_n15, eq10_e1054_d_n16, eq10_e1054_d_n17, eq10_e1054_d_n18, eq10_e1054_d_n19, eq10_e1054_d_n20];
        let eq10_branch_derivatives: [f64; 25] = [eq10_e1054_d_b0, eq10_e1054_d_b1, eq10_e1054_d_b2, eq10_e1054_d_b3, eq10_e1054_d_b4, eq10_e1054_d_b5, eq10_e1054_d_b6, eq10_e1054_d_b7, eq10_e1054_d_b8, eq10_e1054_d_b9, eq10_e1054_d_b10, eq10_e1054_d_b11, eq10_e1054_d_b12, eq10_e1054_d_b13, eq10_e1054_d_b14, eq10_e1054_d_b15, eq10_e1054_d_b16, eq10_e1054_d_b17, eq10_e1054_d_b18, eq10_e1054_d_b19, eq10_e1054_d_b20, eq10_e1054_d_b21, eq10_e1054_d_b22, eq10_e1054_d_b23, eq10_e1054_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq10_value),
            &nodes,
            &eq10_node_derivatives,
            &branches,
            &eq10_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq11_e1057: f64 = (s.v[0] * s.v[19]);
        let eq11_e1057_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq11_e1057_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq11_e1057_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq11_e1057_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq11_e1057_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq11_e1057_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq11_e1057_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq11_e1057_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq11_e1057_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq11_e1057_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq11_e1057_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq11_e1057_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq11_e1057_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq11_e1057_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq11_e1057_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq11_e1057_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq11_e1057_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq11_e1057_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq11_e1057_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq11_e1057_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq11_e1057_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq11_e1057_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq11_e1057_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq11_e1057_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq11_e1057_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq11_e1057_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq11_e1057_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq11_e1057_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq11_e1057_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq11_e1057_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq11_e1057_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq11_e1057_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq11_e1057_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq11_e1057_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq11_e1057_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq11_e1057_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq11_e1057_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq11_e1057_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq11_e1057_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq11_e1057_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq11_e1057_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq11_e1057_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq11_e1057_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq11_e1057_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq11_e1057_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq11_e1057_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq11_e1059: f64 = (eq11_e1057 * p.p32);
        let eq11_e1059_d_n0: f64 = (eq11_e1057_d_n0 * p.p32);
        let eq11_e1059_d_n1: f64 = (eq11_e1057_d_n1 * p.p32);
        let eq11_e1059_d_n2: f64 = (eq11_e1057_d_n2 * p.p32);
        let eq11_e1059_d_n3: f64 = (eq11_e1057_d_n3 * p.p32);
        let eq11_e1059_d_n4: f64 = (eq11_e1057_d_n4 * p.p32);
        let eq11_e1059_d_n5: f64 = (eq11_e1057_d_n5 * p.p32);
        let eq11_e1059_d_n6: f64 = (eq11_e1057_d_n6 * p.p32);
        let eq11_e1059_d_n7: f64 = (eq11_e1057_d_n7 * p.p32);
        let eq11_e1059_d_n8: f64 = (eq11_e1057_d_n8 * p.p32);
        let eq11_e1059_d_n9: f64 = (eq11_e1057_d_n9 * p.p32);
        let eq11_e1059_d_n10: f64 = (eq11_e1057_d_n10 * p.p32);
        let eq11_e1059_d_n11: f64 = (eq11_e1057_d_n11 * p.p32);
        let eq11_e1059_d_n12: f64 = (eq11_e1057_d_n12 * p.p32);
        let eq11_e1059_d_n13: f64 = (eq11_e1057_d_n13 * p.p32);
        let eq11_e1059_d_n14: f64 = (eq11_e1057_d_n14 * p.p32);
        let eq11_e1059_d_n15: f64 = (eq11_e1057_d_n15 * p.p32);
        let eq11_e1059_d_n16: f64 = (eq11_e1057_d_n16 * p.p32);
        let eq11_e1059_d_n17: f64 = (eq11_e1057_d_n17 * p.p32);
        let eq11_e1059_d_n18: f64 = (eq11_e1057_d_n18 * p.p32);
        let eq11_e1059_d_n19: f64 = (eq11_e1057_d_n19 * p.p32);
        let eq11_e1059_d_n20: f64 = (eq11_e1057_d_n20 * p.p32);
        let eq11_e1059_d_b0: f64 = (eq11_e1057_d_b0 * p.p32);
        let eq11_e1059_d_b1: f64 = (eq11_e1057_d_b1 * p.p32);
        let eq11_e1059_d_b2: f64 = (eq11_e1057_d_b2 * p.p32);
        let eq11_e1059_d_b3: f64 = (eq11_e1057_d_b3 * p.p32);
        let eq11_e1059_d_b4: f64 = (eq11_e1057_d_b4 * p.p32);
        let eq11_e1059_d_b5: f64 = (eq11_e1057_d_b5 * p.p32);
        let eq11_e1059_d_b6: f64 = (eq11_e1057_d_b6 * p.p32);
        let eq11_e1059_d_b7: f64 = (eq11_e1057_d_b7 * p.p32);
        let eq11_e1059_d_b8: f64 = (eq11_e1057_d_b8 * p.p32);
        let eq11_e1059_d_b9: f64 = (eq11_e1057_d_b9 * p.p32);
        let eq11_e1059_d_b10: f64 = (eq11_e1057_d_b10 * p.p32);
        let eq11_e1059_d_b11: f64 = (eq11_e1057_d_b11 * p.p32);
        let eq11_e1059_d_b12: f64 = (eq11_e1057_d_b12 * p.p32);
        let eq11_e1059_d_b13: f64 = (eq11_e1057_d_b13 * p.p32);
        let eq11_e1059_d_b14: f64 = (eq11_e1057_d_b14 * p.p32);
        let eq11_e1059_d_b15: f64 = (eq11_e1057_d_b15 * p.p32);
        let eq11_e1059_d_b16: f64 = (eq11_e1057_d_b16 * p.p32);
        let eq11_e1059_d_b17: f64 = (eq11_e1057_d_b17 * p.p32);
        let eq11_e1059_d_b18: f64 = (eq11_e1057_d_b18 * p.p32);
        let eq11_e1059_d_b19: f64 = (eq11_e1057_d_b19 * p.p32);
        let eq11_e1059_d_b20: f64 = (eq11_e1057_d_b20 * p.p32);
        let eq11_e1059_d_b21: f64 = (eq11_e1057_d_b21 * p.p32);
        let eq11_e1059_d_b22: f64 = (eq11_e1057_d_b22 * p.p32);
        let eq11_e1059_d_b23: f64 = (eq11_e1057_d_b23 * p.p32);
        let eq11_e1059_d_b24: f64 = (eq11_e1057_d_b24 * p.p32);
        let eq11_e1061: f64 = (eq11_e1059 * s.v[844]);
        let eq11_e1061_d_n0: f64 = ((eq11_e1059_d_n0 * s.v[844]) + (eq11_e1059 * s.dn[844][0]));
        let eq11_e1061_d_n1: f64 = ((eq11_e1059_d_n1 * s.v[844]) + (eq11_e1059 * s.dn[844][1]));
        let eq11_e1061_d_n2: f64 = ((eq11_e1059_d_n2 * s.v[844]) + (eq11_e1059 * s.dn[844][2]));
        let eq11_e1061_d_n3: f64 = ((eq11_e1059_d_n3 * s.v[844]) + (eq11_e1059 * s.dn[844][3]));
        let eq11_e1061_d_n4: f64 = ((eq11_e1059_d_n4 * s.v[844]) + (eq11_e1059 * s.dn[844][4]));
        let eq11_e1061_d_n5: f64 = ((eq11_e1059_d_n5 * s.v[844]) + (eq11_e1059 * s.dn[844][5]));
        let eq11_e1061_d_n6: f64 = ((eq11_e1059_d_n6 * s.v[844]) + (eq11_e1059 * s.dn[844][6]));
        let eq11_e1061_d_n7: f64 = ((eq11_e1059_d_n7 * s.v[844]) + (eq11_e1059 * s.dn[844][7]));
        let eq11_e1061_d_n8: f64 = ((eq11_e1059_d_n8 * s.v[844]) + (eq11_e1059 * s.dn[844][8]));
        let eq11_e1061_d_n9: f64 = ((eq11_e1059_d_n9 * s.v[844]) + (eq11_e1059 * s.dn[844][9]));
        let eq11_e1061_d_n10: f64 = ((eq11_e1059_d_n10 * s.v[844]) + (eq11_e1059 * s.dn[844][10]));
        let eq11_e1061_d_n11: f64 = ((eq11_e1059_d_n11 * s.v[844]) + (eq11_e1059 * s.dn[844][11]));
        let eq11_e1061_d_n12: f64 = ((eq11_e1059_d_n12 * s.v[844]) + (eq11_e1059 * s.dn[844][12]));
        let eq11_e1061_d_n13: f64 = ((eq11_e1059_d_n13 * s.v[844]) + (eq11_e1059 * s.dn[844][13]));
        let eq11_e1061_d_n14: f64 = ((eq11_e1059_d_n14 * s.v[844]) + (eq11_e1059 * s.dn[844][14]));
        let eq11_e1061_d_n15: f64 = ((eq11_e1059_d_n15 * s.v[844]) + (eq11_e1059 * s.dn[844][15]));
        let eq11_e1061_d_n16: f64 = ((eq11_e1059_d_n16 * s.v[844]) + (eq11_e1059 * s.dn[844][16]));
        let eq11_e1061_d_n17: f64 = ((eq11_e1059_d_n17 * s.v[844]) + (eq11_e1059 * s.dn[844][17]));
        let eq11_e1061_d_n18: f64 = ((eq11_e1059_d_n18 * s.v[844]) + (eq11_e1059 * s.dn[844][18]));
        let eq11_e1061_d_n19: f64 = ((eq11_e1059_d_n19 * s.v[844]) + (eq11_e1059 * s.dn[844][19]));
        let eq11_e1061_d_n20: f64 = ((eq11_e1059_d_n20 * s.v[844]) + (eq11_e1059 * s.dn[844][20]));
        let eq11_e1061_d_b0: f64 = ((eq11_e1059_d_b0 * s.v[844]) + (eq11_e1059 * s.db[844][0]));
        let eq11_e1061_d_b1: f64 = ((eq11_e1059_d_b1 * s.v[844]) + (eq11_e1059 * s.db[844][1]));
        let eq11_e1061_d_b2: f64 = ((eq11_e1059_d_b2 * s.v[844]) + (eq11_e1059 * s.db[844][2]));
        let eq11_e1061_d_b3: f64 = ((eq11_e1059_d_b3 * s.v[844]) + (eq11_e1059 * s.db[844][3]));
        let eq11_e1061_d_b4: f64 = ((eq11_e1059_d_b4 * s.v[844]) + (eq11_e1059 * s.db[844][4]));
        let eq11_e1061_d_b5: f64 = ((eq11_e1059_d_b5 * s.v[844]) + (eq11_e1059 * s.db[844][5]));
        let eq11_e1061_d_b6: f64 = ((eq11_e1059_d_b6 * s.v[844]) + (eq11_e1059 * s.db[844][6]));
        let eq11_e1061_d_b7: f64 = ((eq11_e1059_d_b7 * s.v[844]) + (eq11_e1059 * s.db[844][7]));
        let eq11_e1061_d_b8: f64 = ((eq11_e1059_d_b8 * s.v[844]) + (eq11_e1059 * s.db[844][8]));
        let eq11_e1061_d_b9: f64 = ((eq11_e1059_d_b9 * s.v[844]) + (eq11_e1059 * s.db[844][9]));
        let eq11_e1061_d_b10: f64 = ((eq11_e1059_d_b10 * s.v[844]) + (eq11_e1059 * s.db[844][10]));
        let eq11_e1061_d_b11: f64 = ((eq11_e1059_d_b11 * s.v[844]) + (eq11_e1059 * s.db[844][11]));
        let eq11_e1061_d_b12: f64 = ((eq11_e1059_d_b12 * s.v[844]) + (eq11_e1059 * s.db[844][12]));
        let eq11_e1061_d_b13: f64 = ((eq11_e1059_d_b13 * s.v[844]) + (eq11_e1059 * s.db[844][13]));
        let eq11_e1061_d_b14: f64 = ((eq11_e1059_d_b14 * s.v[844]) + (eq11_e1059 * s.db[844][14]));
        let eq11_e1061_d_b15: f64 = ((eq11_e1059_d_b15 * s.v[844]) + (eq11_e1059 * s.db[844][15]));
        let eq11_e1061_d_b16: f64 = ((eq11_e1059_d_b16 * s.v[844]) + (eq11_e1059 * s.db[844][16]));
        let eq11_e1061_d_b17: f64 = ((eq11_e1059_d_b17 * s.v[844]) + (eq11_e1059 * s.db[844][17]));
        let eq11_e1061_d_b18: f64 = ((eq11_e1059_d_b18 * s.v[844]) + (eq11_e1059 * s.db[844][18]));
        let eq11_e1061_d_b19: f64 = ((eq11_e1059_d_b19 * s.v[844]) + (eq11_e1059 * s.db[844][19]));
        let eq11_e1061_d_b20: f64 = ((eq11_e1059_d_b20 * s.v[844]) + (eq11_e1059 * s.db[844][20]));
        let eq11_e1061_d_b21: f64 = ((eq11_e1059_d_b21 * s.v[844]) + (eq11_e1059 * s.db[844][21]));
        let eq11_e1061_d_b22: f64 = ((eq11_e1059_d_b22 * s.v[844]) + (eq11_e1059 * s.db[844][22]));
        let eq11_e1061_d_b23: f64 = ((eq11_e1059_d_b23 * s.v[844]) + (eq11_e1059 * s.db[844][23]));
        let eq11_e1061_d_b24: f64 = ((eq11_e1059_d_b24 * s.v[844]) + (eq11_e1059 * s.db[844][24]));
        let eq11_value: f64 = eq11_e1061;
        let eq11_node_derivatives: [f64; 21] = [eq11_e1061_d_n0, eq11_e1061_d_n1, eq11_e1061_d_n2, eq11_e1061_d_n3, eq11_e1061_d_n4, eq11_e1061_d_n5, eq11_e1061_d_n6, eq11_e1061_d_n7, eq11_e1061_d_n8, eq11_e1061_d_n9, eq11_e1061_d_n10, eq11_e1061_d_n11, eq11_e1061_d_n12, eq11_e1061_d_n13, eq11_e1061_d_n14, eq11_e1061_d_n15, eq11_e1061_d_n16, eq11_e1061_d_n17, eq11_e1061_d_n18, eq11_e1061_d_n19, eq11_e1061_d_n20];
        let eq11_branch_derivatives: [f64; 25] = [eq11_e1061_d_b0, eq11_e1061_d_b1, eq11_e1061_d_b2, eq11_e1061_d_b3, eq11_e1061_d_b4, eq11_e1061_d_b5, eq11_e1061_d_b6, eq11_e1061_d_b7, eq11_e1061_d_b8, eq11_e1061_d_b9, eq11_e1061_d_b10, eq11_e1061_d_b11, eq11_e1061_d_b12, eq11_e1061_d_b13, eq11_e1061_d_b14, eq11_e1061_d_b15, eq11_e1061_d_b16, eq11_e1061_d_b17, eq11_e1061_d_b18, eq11_e1061_d_b19, eq11_e1061_d_b20, eq11_e1061_d_b21, eq11_e1061_d_b22, eq11_e1061_d_b23, eq11_e1061_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq11_value),
            &nodes,
            &eq11_node_derivatives,
            &branches,
            &eq11_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq12_e1064: f64 = (s.v[0] * s.v[19]);
        let eq12_e1064_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq12_e1064_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq12_e1064_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq12_e1064_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq12_e1064_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq12_e1064_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq12_e1064_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq12_e1064_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq12_e1064_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq12_e1064_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq12_e1064_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq12_e1064_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq12_e1064_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq12_e1064_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq12_e1064_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq12_e1064_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq12_e1064_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq12_e1064_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq12_e1064_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq12_e1064_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq12_e1064_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq12_e1064_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq12_e1064_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq12_e1064_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq12_e1064_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq12_e1064_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq12_e1064_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq12_e1064_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq12_e1064_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq12_e1064_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq12_e1064_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq12_e1064_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq12_e1064_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq12_e1064_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq12_e1064_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq12_e1064_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq12_e1064_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq12_e1064_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq12_e1064_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq12_e1064_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq12_e1064_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq12_e1064_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq12_e1064_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq12_e1064_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq12_e1064_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq12_e1064_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq12_e1066: f64 = (eq12_e1064 * p.p32);
        let eq12_e1066_d_n0: f64 = (eq12_e1064_d_n0 * p.p32);
        let eq12_e1066_d_n1: f64 = (eq12_e1064_d_n1 * p.p32);
        let eq12_e1066_d_n2: f64 = (eq12_e1064_d_n2 * p.p32);
        let eq12_e1066_d_n3: f64 = (eq12_e1064_d_n3 * p.p32);
        let eq12_e1066_d_n4: f64 = (eq12_e1064_d_n4 * p.p32);
        let eq12_e1066_d_n5: f64 = (eq12_e1064_d_n5 * p.p32);
        let eq12_e1066_d_n6: f64 = (eq12_e1064_d_n6 * p.p32);
        let eq12_e1066_d_n7: f64 = (eq12_e1064_d_n7 * p.p32);
        let eq12_e1066_d_n8: f64 = (eq12_e1064_d_n8 * p.p32);
        let eq12_e1066_d_n9: f64 = (eq12_e1064_d_n9 * p.p32);
        let eq12_e1066_d_n10: f64 = (eq12_e1064_d_n10 * p.p32);
        let eq12_e1066_d_n11: f64 = (eq12_e1064_d_n11 * p.p32);
        let eq12_e1066_d_n12: f64 = (eq12_e1064_d_n12 * p.p32);
        let eq12_e1066_d_n13: f64 = (eq12_e1064_d_n13 * p.p32);
        let eq12_e1066_d_n14: f64 = (eq12_e1064_d_n14 * p.p32);
        let eq12_e1066_d_n15: f64 = (eq12_e1064_d_n15 * p.p32);
        let eq12_e1066_d_n16: f64 = (eq12_e1064_d_n16 * p.p32);
        let eq12_e1066_d_n17: f64 = (eq12_e1064_d_n17 * p.p32);
        let eq12_e1066_d_n18: f64 = (eq12_e1064_d_n18 * p.p32);
        let eq12_e1066_d_n19: f64 = (eq12_e1064_d_n19 * p.p32);
        let eq12_e1066_d_n20: f64 = (eq12_e1064_d_n20 * p.p32);
        let eq12_e1066_d_b0: f64 = (eq12_e1064_d_b0 * p.p32);
        let eq12_e1066_d_b1: f64 = (eq12_e1064_d_b1 * p.p32);
        let eq12_e1066_d_b2: f64 = (eq12_e1064_d_b2 * p.p32);
        let eq12_e1066_d_b3: f64 = (eq12_e1064_d_b3 * p.p32);
        let eq12_e1066_d_b4: f64 = (eq12_e1064_d_b4 * p.p32);
        let eq12_e1066_d_b5: f64 = (eq12_e1064_d_b5 * p.p32);
        let eq12_e1066_d_b6: f64 = (eq12_e1064_d_b6 * p.p32);
        let eq12_e1066_d_b7: f64 = (eq12_e1064_d_b7 * p.p32);
        let eq12_e1066_d_b8: f64 = (eq12_e1064_d_b8 * p.p32);
        let eq12_e1066_d_b9: f64 = (eq12_e1064_d_b9 * p.p32);
        let eq12_e1066_d_b10: f64 = (eq12_e1064_d_b10 * p.p32);
        let eq12_e1066_d_b11: f64 = (eq12_e1064_d_b11 * p.p32);
        let eq12_e1066_d_b12: f64 = (eq12_e1064_d_b12 * p.p32);
        let eq12_e1066_d_b13: f64 = (eq12_e1064_d_b13 * p.p32);
        let eq12_e1066_d_b14: f64 = (eq12_e1064_d_b14 * p.p32);
        let eq12_e1066_d_b15: f64 = (eq12_e1064_d_b15 * p.p32);
        let eq12_e1066_d_b16: f64 = (eq12_e1064_d_b16 * p.p32);
        let eq12_e1066_d_b17: f64 = (eq12_e1064_d_b17 * p.p32);
        let eq12_e1066_d_b18: f64 = (eq12_e1064_d_b18 * p.p32);
        let eq12_e1066_d_b19: f64 = (eq12_e1064_d_b19 * p.p32);
        let eq12_e1066_d_b20: f64 = (eq12_e1064_d_b20 * p.p32);
        let eq12_e1066_d_b21: f64 = (eq12_e1064_d_b21 * p.p32);
        let eq12_e1066_d_b22: f64 = (eq12_e1064_d_b22 * p.p32);
        let eq12_e1066_d_b23: f64 = (eq12_e1064_d_b23 * p.p32);
        let eq12_e1066_d_b24: f64 = (eq12_e1064_d_b24 * p.p32);
        let eq12_e1068: f64 = (eq12_e1066 * s.v[845]);
        let eq12_e1068_d_n0: f64 = ((eq12_e1066_d_n0 * s.v[845]) + (eq12_e1066 * s.dn[845][0]));
        let eq12_e1068_d_n1: f64 = ((eq12_e1066_d_n1 * s.v[845]) + (eq12_e1066 * s.dn[845][1]));
        let eq12_e1068_d_n2: f64 = ((eq12_e1066_d_n2 * s.v[845]) + (eq12_e1066 * s.dn[845][2]));
        let eq12_e1068_d_n3: f64 = ((eq12_e1066_d_n3 * s.v[845]) + (eq12_e1066 * s.dn[845][3]));
        let eq12_e1068_d_n4: f64 = ((eq12_e1066_d_n4 * s.v[845]) + (eq12_e1066 * s.dn[845][4]));
        let eq12_e1068_d_n5: f64 = ((eq12_e1066_d_n5 * s.v[845]) + (eq12_e1066 * s.dn[845][5]));
        let eq12_e1068_d_n6: f64 = ((eq12_e1066_d_n6 * s.v[845]) + (eq12_e1066 * s.dn[845][6]));
        let eq12_e1068_d_n7: f64 = ((eq12_e1066_d_n7 * s.v[845]) + (eq12_e1066 * s.dn[845][7]));
        let eq12_e1068_d_n8: f64 = ((eq12_e1066_d_n8 * s.v[845]) + (eq12_e1066 * s.dn[845][8]));
        let eq12_e1068_d_n9: f64 = ((eq12_e1066_d_n9 * s.v[845]) + (eq12_e1066 * s.dn[845][9]));
        let eq12_e1068_d_n10: f64 = ((eq12_e1066_d_n10 * s.v[845]) + (eq12_e1066 * s.dn[845][10]));
        let eq12_e1068_d_n11: f64 = ((eq12_e1066_d_n11 * s.v[845]) + (eq12_e1066 * s.dn[845][11]));
        let eq12_e1068_d_n12: f64 = ((eq12_e1066_d_n12 * s.v[845]) + (eq12_e1066 * s.dn[845][12]));
        let eq12_e1068_d_n13: f64 = ((eq12_e1066_d_n13 * s.v[845]) + (eq12_e1066 * s.dn[845][13]));
        let eq12_e1068_d_n14: f64 = ((eq12_e1066_d_n14 * s.v[845]) + (eq12_e1066 * s.dn[845][14]));
        let eq12_e1068_d_n15: f64 = ((eq12_e1066_d_n15 * s.v[845]) + (eq12_e1066 * s.dn[845][15]));
        let eq12_e1068_d_n16: f64 = ((eq12_e1066_d_n16 * s.v[845]) + (eq12_e1066 * s.dn[845][16]));
        let eq12_e1068_d_n17: f64 = ((eq12_e1066_d_n17 * s.v[845]) + (eq12_e1066 * s.dn[845][17]));
        let eq12_e1068_d_n18: f64 = ((eq12_e1066_d_n18 * s.v[845]) + (eq12_e1066 * s.dn[845][18]));
        let eq12_e1068_d_n19: f64 = ((eq12_e1066_d_n19 * s.v[845]) + (eq12_e1066 * s.dn[845][19]));
        let eq12_e1068_d_n20: f64 = ((eq12_e1066_d_n20 * s.v[845]) + (eq12_e1066 * s.dn[845][20]));
        let eq12_e1068_d_b0: f64 = ((eq12_e1066_d_b0 * s.v[845]) + (eq12_e1066 * s.db[845][0]));
        let eq12_e1068_d_b1: f64 = ((eq12_e1066_d_b1 * s.v[845]) + (eq12_e1066 * s.db[845][1]));
        let eq12_e1068_d_b2: f64 = ((eq12_e1066_d_b2 * s.v[845]) + (eq12_e1066 * s.db[845][2]));
        let eq12_e1068_d_b3: f64 = ((eq12_e1066_d_b3 * s.v[845]) + (eq12_e1066 * s.db[845][3]));
        let eq12_e1068_d_b4: f64 = ((eq12_e1066_d_b4 * s.v[845]) + (eq12_e1066 * s.db[845][4]));
        let eq12_e1068_d_b5: f64 = ((eq12_e1066_d_b5 * s.v[845]) + (eq12_e1066 * s.db[845][5]));
        let eq12_e1068_d_b6: f64 = ((eq12_e1066_d_b6 * s.v[845]) + (eq12_e1066 * s.db[845][6]));
        let eq12_e1068_d_b7: f64 = ((eq12_e1066_d_b7 * s.v[845]) + (eq12_e1066 * s.db[845][7]));
        let eq12_e1068_d_b8: f64 = ((eq12_e1066_d_b8 * s.v[845]) + (eq12_e1066 * s.db[845][8]));
        let eq12_e1068_d_b9: f64 = ((eq12_e1066_d_b9 * s.v[845]) + (eq12_e1066 * s.db[845][9]));
        let eq12_e1068_d_b10: f64 = ((eq12_e1066_d_b10 * s.v[845]) + (eq12_e1066 * s.db[845][10]));
        let eq12_e1068_d_b11: f64 = ((eq12_e1066_d_b11 * s.v[845]) + (eq12_e1066 * s.db[845][11]));
        let eq12_e1068_d_b12: f64 = ((eq12_e1066_d_b12 * s.v[845]) + (eq12_e1066 * s.db[845][12]));
        let eq12_e1068_d_b13: f64 = ((eq12_e1066_d_b13 * s.v[845]) + (eq12_e1066 * s.db[845][13]));
        let eq12_e1068_d_b14: f64 = ((eq12_e1066_d_b14 * s.v[845]) + (eq12_e1066 * s.db[845][14]));
        let eq12_e1068_d_b15: f64 = ((eq12_e1066_d_b15 * s.v[845]) + (eq12_e1066 * s.db[845][15]));
        let eq12_e1068_d_b16: f64 = ((eq12_e1066_d_b16 * s.v[845]) + (eq12_e1066 * s.db[845][16]));
        let eq12_e1068_d_b17: f64 = ((eq12_e1066_d_b17 * s.v[845]) + (eq12_e1066 * s.db[845][17]));
        let eq12_e1068_d_b18: f64 = ((eq12_e1066_d_b18 * s.v[845]) + (eq12_e1066 * s.db[845][18]));
        let eq12_e1068_d_b19: f64 = ((eq12_e1066_d_b19 * s.v[845]) + (eq12_e1066 * s.db[845][19]));
        let eq12_e1068_d_b20: f64 = ((eq12_e1066_d_b20 * s.v[845]) + (eq12_e1066 * s.db[845][20]));
        let eq12_e1068_d_b21: f64 = ((eq12_e1066_d_b21 * s.v[845]) + (eq12_e1066 * s.db[845][21]));
        let eq12_e1068_d_b22: f64 = ((eq12_e1066_d_b22 * s.v[845]) + (eq12_e1066 * s.db[845][22]));
        let eq12_e1068_d_b23: f64 = ((eq12_e1066_d_b23 * s.v[845]) + (eq12_e1066 * s.db[845][23]));
        let eq12_e1068_d_b24: f64 = ((eq12_e1066_d_b24 * s.v[845]) + (eq12_e1066 * s.db[845][24]));
        let eq12_value: f64 = eq12_e1068;
        let eq12_node_derivatives: [f64; 21] = [eq12_e1068_d_n0, eq12_e1068_d_n1, eq12_e1068_d_n2, eq12_e1068_d_n3, eq12_e1068_d_n4, eq12_e1068_d_n5, eq12_e1068_d_n6, eq12_e1068_d_n7, eq12_e1068_d_n8, eq12_e1068_d_n9, eq12_e1068_d_n10, eq12_e1068_d_n11, eq12_e1068_d_n12, eq12_e1068_d_n13, eq12_e1068_d_n14, eq12_e1068_d_n15, eq12_e1068_d_n16, eq12_e1068_d_n17, eq12_e1068_d_n18, eq12_e1068_d_n19, eq12_e1068_d_n20];
        let eq12_branch_derivatives: [f64; 25] = [eq12_e1068_d_b0, eq12_e1068_d_b1, eq12_e1068_d_b2, eq12_e1068_d_b3, eq12_e1068_d_b4, eq12_e1068_d_b5, eq12_e1068_d_b6, eq12_e1068_d_b7, eq12_e1068_d_b8, eq12_e1068_d_b9, eq12_e1068_d_b10, eq12_e1068_d_b11, eq12_e1068_d_b12, eq12_e1068_d_b13, eq12_e1068_d_b14, eq12_e1068_d_b15, eq12_e1068_d_b16, eq12_e1068_d_b17, eq12_e1068_d_b18, eq12_e1068_d_b19, eq12_e1068_d_b20, eq12_e1068_d_b21, eq12_e1068_d_b22, eq12_e1068_d_b23, eq12_e1068_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq12_value),
            &nodes,
            &eq12_node_derivatives,
            &branches,
            &eq12_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq13_e1071: f64 = (s.v[0] * s.v[19]);
        let eq13_e1071_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq13_e1071_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq13_e1071_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq13_e1071_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq13_e1071_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq13_e1071_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq13_e1071_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq13_e1071_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq13_e1071_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq13_e1071_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq13_e1071_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq13_e1071_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq13_e1071_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq13_e1071_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq13_e1071_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq13_e1071_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq13_e1071_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq13_e1071_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq13_e1071_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq13_e1071_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq13_e1071_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq13_e1071_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq13_e1071_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq13_e1071_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq13_e1071_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq13_e1071_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq13_e1071_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq13_e1071_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq13_e1071_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq13_e1071_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq13_e1071_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq13_e1071_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq13_e1071_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq13_e1071_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq13_e1071_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq13_e1071_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq13_e1071_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq13_e1071_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq13_e1071_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq13_e1071_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq13_e1071_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq13_e1071_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq13_e1071_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq13_e1071_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq13_e1071_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq13_e1071_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq13_e1073: f64 = (eq13_e1071 * p.p32);
        let eq13_e1073_d_n0: f64 = (eq13_e1071_d_n0 * p.p32);
        let eq13_e1073_d_n1: f64 = (eq13_e1071_d_n1 * p.p32);
        let eq13_e1073_d_n2: f64 = (eq13_e1071_d_n2 * p.p32);
        let eq13_e1073_d_n3: f64 = (eq13_e1071_d_n3 * p.p32);
        let eq13_e1073_d_n4: f64 = (eq13_e1071_d_n4 * p.p32);
        let eq13_e1073_d_n5: f64 = (eq13_e1071_d_n5 * p.p32);
        let eq13_e1073_d_n6: f64 = (eq13_e1071_d_n6 * p.p32);
        let eq13_e1073_d_n7: f64 = (eq13_e1071_d_n7 * p.p32);
        let eq13_e1073_d_n8: f64 = (eq13_e1071_d_n8 * p.p32);
        let eq13_e1073_d_n9: f64 = (eq13_e1071_d_n9 * p.p32);
        let eq13_e1073_d_n10: f64 = (eq13_e1071_d_n10 * p.p32);
        let eq13_e1073_d_n11: f64 = (eq13_e1071_d_n11 * p.p32);
        let eq13_e1073_d_n12: f64 = (eq13_e1071_d_n12 * p.p32);
        let eq13_e1073_d_n13: f64 = (eq13_e1071_d_n13 * p.p32);
        let eq13_e1073_d_n14: f64 = (eq13_e1071_d_n14 * p.p32);
        let eq13_e1073_d_n15: f64 = (eq13_e1071_d_n15 * p.p32);
        let eq13_e1073_d_n16: f64 = (eq13_e1071_d_n16 * p.p32);
        let eq13_e1073_d_n17: f64 = (eq13_e1071_d_n17 * p.p32);
        let eq13_e1073_d_n18: f64 = (eq13_e1071_d_n18 * p.p32);
        let eq13_e1073_d_n19: f64 = (eq13_e1071_d_n19 * p.p32);
        let eq13_e1073_d_n20: f64 = (eq13_e1071_d_n20 * p.p32);
        let eq13_e1073_d_b0: f64 = (eq13_e1071_d_b0 * p.p32);
        let eq13_e1073_d_b1: f64 = (eq13_e1071_d_b1 * p.p32);
        let eq13_e1073_d_b2: f64 = (eq13_e1071_d_b2 * p.p32);
        let eq13_e1073_d_b3: f64 = (eq13_e1071_d_b3 * p.p32);
        let eq13_e1073_d_b4: f64 = (eq13_e1071_d_b4 * p.p32);
        let eq13_e1073_d_b5: f64 = (eq13_e1071_d_b5 * p.p32);
        let eq13_e1073_d_b6: f64 = (eq13_e1071_d_b6 * p.p32);
        let eq13_e1073_d_b7: f64 = (eq13_e1071_d_b7 * p.p32);
        let eq13_e1073_d_b8: f64 = (eq13_e1071_d_b8 * p.p32);
        let eq13_e1073_d_b9: f64 = (eq13_e1071_d_b9 * p.p32);
        let eq13_e1073_d_b10: f64 = (eq13_e1071_d_b10 * p.p32);
        let eq13_e1073_d_b11: f64 = (eq13_e1071_d_b11 * p.p32);
        let eq13_e1073_d_b12: f64 = (eq13_e1071_d_b12 * p.p32);
        let eq13_e1073_d_b13: f64 = (eq13_e1071_d_b13 * p.p32);
        let eq13_e1073_d_b14: f64 = (eq13_e1071_d_b14 * p.p32);
        let eq13_e1073_d_b15: f64 = (eq13_e1071_d_b15 * p.p32);
        let eq13_e1073_d_b16: f64 = (eq13_e1071_d_b16 * p.p32);
        let eq13_e1073_d_b17: f64 = (eq13_e1071_d_b17 * p.p32);
        let eq13_e1073_d_b18: f64 = (eq13_e1071_d_b18 * p.p32);
        let eq13_e1073_d_b19: f64 = (eq13_e1071_d_b19 * p.p32);
        let eq13_e1073_d_b20: f64 = (eq13_e1071_d_b20 * p.p32);
        let eq13_e1073_d_b21: f64 = (eq13_e1071_d_b21 * p.p32);
        let eq13_e1073_d_b22: f64 = (eq13_e1071_d_b22 * p.p32);
        let eq13_e1073_d_b23: f64 = (eq13_e1071_d_b23 * p.p32);
        let eq13_e1073_d_b24: f64 = (eq13_e1071_d_b24 * p.p32);
        let eq13_e1075: f64 = (eq13_e1073 * s.v[848]);
        let eq13_e1075_d_n0: f64 = ((eq13_e1073_d_n0 * s.v[848]) + (eq13_e1073 * s.dn[848][0]));
        let eq13_e1075_d_n1: f64 = ((eq13_e1073_d_n1 * s.v[848]) + (eq13_e1073 * s.dn[848][1]));
        let eq13_e1075_d_n2: f64 = ((eq13_e1073_d_n2 * s.v[848]) + (eq13_e1073 * s.dn[848][2]));
        let eq13_e1075_d_n3: f64 = ((eq13_e1073_d_n3 * s.v[848]) + (eq13_e1073 * s.dn[848][3]));
        let eq13_e1075_d_n4: f64 = ((eq13_e1073_d_n4 * s.v[848]) + (eq13_e1073 * s.dn[848][4]));
        let eq13_e1075_d_n5: f64 = ((eq13_e1073_d_n5 * s.v[848]) + (eq13_e1073 * s.dn[848][5]));
        let eq13_e1075_d_n6: f64 = ((eq13_e1073_d_n6 * s.v[848]) + (eq13_e1073 * s.dn[848][6]));
        let eq13_e1075_d_n7: f64 = ((eq13_e1073_d_n7 * s.v[848]) + (eq13_e1073 * s.dn[848][7]));
        let eq13_e1075_d_n8: f64 = ((eq13_e1073_d_n8 * s.v[848]) + (eq13_e1073 * s.dn[848][8]));
        let eq13_e1075_d_n9: f64 = ((eq13_e1073_d_n9 * s.v[848]) + (eq13_e1073 * s.dn[848][9]));
        let eq13_e1075_d_n10: f64 = ((eq13_e1073_d_n10 * s.v[848]) + (eq13_e1073 * s.dn[848][10]));
        let eq13_e1075_d_n11: f64 = ((eq13_e1073_d_n11 * s.v[848]) + (eq13_e1073 * s.dn[848][11]));
        let eq13_e1075_d_n12: f64 = ((eq13_e1073_d_n12 * s.v[848]) + (eq13_e1073 * s.dn[848][12]));
        let eq13_e1075_d_n13: f64 = ((eq13_e1073_d_n13 * s.v[848]) + (eq13_e1073 * s.dn[848][13]));
        let eq13_e1075_d_n14: f64 = ((eq13_e1073_d_n14 * s.v[848]) + (eq13_e1073 * s.dn[848][14]));
        let eq13_e1075_d_n15: f64 = ((eq13_e1073_d_n15 * s.v[848]) + (eq13_e1073 * s.dn[848][15]));
        let eq13_e1075_d_n16: f64 = ((eq13_e1073_d_n16 * s.v[848]) + (eq13_e1073 * s.dn[848][16]));
        let eq13_e1075_d_n17: f64 = ((eq13_e1073_d_n17 * s.v[848]) + (eq13_e1073 * s.dn[848][17]));
        let eq13_e1075_d_n18: f64 = ((eq13_e1073_d_n18 * s.v[848]) + (eq13_e1073 * s.dn[848][18]));
        let eq13_e1075_d_n19: f64 = ((eq13_e1073_d_n19 * s.v[848]) + (eq13_e1073 * s.dn[848][19]));
        let eq13_e1075_d_n20: f64 = ((eq13_e1073_d_n20 * s.v[848]) + (eq13_e1073 * s.dn[848][20]));
        let eq13_e1075_d_b0: f64 = ((eq13_e1073_d_b0 * s.v[848]) + (eq13_e1073 * s.db[848][0]));
        let eq13_e1075_d_b1: f64 = ((eq13_e1073_d_b1 * s.v[848]) + (eq13_e1073 * s.db[848][1]));
        let eq13_e1075_d_b2: f64 = ((eq13_e1073_d_b2 * s.v[848]) + (eq13_e1073 * s.db[848][2]));
        let eq13_e1075_d_b3: f64 = ((eq13_e1073_d_b3 * s.v[848]) + (eq13_e1073 * s.db[848][3]));
        let eq13_e1075_d_b4: f64 = ((eq13_e1073_d_b4 * s.v[848]) + (eq13_e1073 * s.db[848][4]));
        let eq13_e1075_d_b5: f64 = ((eq13_e1073_d_b5 * s.v[848]) + (eq13_e1073 * s.db[848][5]));
        let eq13_e1075_d_b6: f64 = ((eq13_e1073_d_b6 * s.v[848]) + (eq13_e1073 * s.db[848][6]));
        let eq13_e1075_d_b7: f64 = ((eq13_e1073_d_b7 * s.v[848]) + (eq13_e1073 * s.db[848][7]));
        let eq13_e1075_d_b8: f64 = ((eq13_e1073_d_b8 * s.v[848]) + (eq13_e1073 * s.db[848][8]));
        let eq13_e1075_d_b9: f64 = ((eq13_e1073_d_b9 * s.v[848]) + (eq13_e1073 * s.db[848][9]));
        let eq13_e1075_d_b10: f64 = ((eq13_e1073_d_b10 * s.v[848]) + (eq13_e1073 * s.db[848][10]));
        let eq13_e1075_d_b11: f64 = ((eq13_e1073_d_b11 * s.v[848]) + (eq13_e1073 * s.db[848][11]));
        let eq13_e1075_d_b12: f64 = ((eq13_e1073_d_b12 * s.v[848]) + (eq13_e1073 * s.db[848][12]));
        let eq13_e1075_d_b13: f64 = ((eq13_e1073_d_b13 * s.v[848]) + (eq13_e1073 * s.db[848][13]));
        let eq13_e1075_d_b14: f64 = ((eq13_e1073_d_b14 * s.v[848]) + (eq13_e1073 * s.db[848][14]));
        let eq13_e1075_d_b15: f64 = ((eq13_e1073_d_b15 * s.v[848]) + (eq13_e1073 * s.db[848][15]));
        let eq13_e1075_d_b16: f64 = ((eq13_e1073_d_b16 * s.v[848]) + (eq13_e1073 * s.db[848][16]));
        let eq13_e1075_d_b17: f64 = ((eq13_e1073_d_b17 * s.v[848]) + (eq13_e1073 * s.db[848][17]));
        let eq13_e1075_d_b18: f64 = ((eq13_e1073_d_b18 * s.v[848]) + (eq13_e1073 * s.db[848][18]));
        let eq13_e1075_d_b19: f64 = ((eq13_e1073_d_b19 * s.v[848]) + (eq13_e1073 * s.db[848][19]));
        let eq13_e1075_d_b20: f64 = ((eq13_e1073_d_b20 * s.v[848]) + (eq13_e1073 * s.db[848][20]));
        let eq13_e1075_d_b21: f64 = ((eq13_e1073_d_b21 * s.v[848]) + (eq13_e1073 * s.db[848][21]));
        let eq13_e1075_d_b22: f64 = ((eq13_e1073_d_b22 * s.v[848]) + (eq13_e1073 * s.db[848][22]));
        let eq13_e1075_d_b23: f64 = ((eq13_e1073_d_b23 * s.v[848]) + (eq13_e1073 * s.db[848][23]));
        let eq13_e1075_d_b24: f64 = ((eq13_e1073_d_b24 * s.v[848]) + (eq13_e1073 * s.db[848][24]));
        let eq13_value: f64 = eq13_e1075;
        let eq13_node_derivatives: [f64; 21] = [eq13_e1075_d_n0, eq13_e1075_d_n1, eq13_e1075_d_n2, eq13_e1075_d_n3, eq13_e1075_d_n4, eq13_e1075_d_n5, eq13_e1075_d_n6, eq13_e1075_d_n7, eq13_e1075_d_n8, eq13_e1075_d_n9, eq13_e1075_d_n10, eq13_e1075_d_n11, eq13_e1075_d_n12, eq13_e1075_d_n13, eq13_e1075_d_n14, eq13_e1075_d_n15, eq13_e1075_d_n16, eq13_e1075_d_n17, eq13_e1075_d_n18, eq13_e1075_d_n19, eq13_e1075_d_n20];
        let eq13_branch_derivatives: [f64; 25] = [eq13_e1075_d_b0, eq13_e1075_d_b1, eq13_e1075_d_b2, eq13_e1075_d_b3, eq13_e1075_d_b4, eq13_e1075_d_b5, eq13_e1075_d_b6, eq13_e1075_d_b7, eq13_e1075_d_b8, eq13_e1075_d_b9, eq13_e1075_d_b10, eq13_e1075_d_b11, eq13_e1075_d_b12, eq13_e1075_d_b13, eq13_e1075_d_b14, eq13_e1075_d_b15, eq13_e1075_d_b16, eq13_e1075_d_b17, eq13_e1075_d_b18, eq13_e1075_d_b19, eq13_e1075_d_b20, eq13_e1075_d_b21, eq13_e1075_d_b22, eq13_e1075_d_b23, eq13_e1075_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            self.multiplicity * (eq13_value),
            &nodes,
            &eq13_node_derivatives,
            &branches,
            &eq13_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq14_e1078: f64 = (s.v[0] * s.v[19]);
        let eq14_e1078_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq14_e1078_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq14_e1078_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq14_e1078_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq14_e1078_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq14_e1078_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq14_e1078_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq14_e1078_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq14_e1078_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq14_e1078_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq14_e1078_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq14_e1078_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq14_e1078_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq14_e1078_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq14_e1078_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq14_e1078_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq14_e1078_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq14_e1078_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq14_e1078_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq14_e1078_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq14_e1078_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq14_e1078_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq14_e1078_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq14_e1078_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq14_e1078_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq14_e1078_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq14_e1078_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq14_e1078_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq14_e1078_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq14_e1078_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq14_e1078_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq14_e1078_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq14_e1078_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq14_e1078_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq14_e1078_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq14_e1078_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq14_e1078_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq14_e1078_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq14_e1078_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq14_e1078_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq14_e1078_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq14_e1078_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq14_e1078_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq14_e1078_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq14_e1078_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq14_e1078_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq14_e1080: f64 = (eq14_e1078 * p.p32);
        let eq14_e1080_d_n0: f64 = (eq14_e1078_d_n0 * p.p32);
        let eq14_e1080_d_n1: f64 = (eq14_e1078_d_n1 * p.p32);
        let eq14_e1080_d_n2: f64 = (eq14_e1078_d_n2 * p.p32);
        let eq14_e1080_d_n3: f64 = (eq14_e1078_d_n3 * p.p32);
        let eq14_e1080_d_n4: f64 = (eq14_e1078_d_n4 * p.p32);
        let eq14_e1080_d_n5: f64 = (eq14_e1078_d_n5 * p.p32);
        let eq14_e1080_d_n6: f64 = (eq14_e1078_d_n6 * p.p32);
        let eq14_e1080_d_n7: f64 = (eq14_e1078_d_n7 * p.p32);
        let eq14_e1080_d_n8: f64 = (eq14_e1078_d_n8 * p.p32);
        let eq14_e1080_d_n9: f64 = (eq14_e1078_d_n9 * p.p32);
        let eq14_e1080_d_n10: f64 = (eq14_e1078_d_n10 * p.p32);
        let eq14_e1080_d_n11: f64 = (eq14_e1078_d_n11 * p.p32);
        let eq14_e1080_d_n12: f64 = (eq14_e1078_d_n12 * p.p32);
        let eq14_e1080_d_n13: f64 = (eq14_e1078_d_n13 * p.p32);
        let eq14_e1080_d_n14: f64 = (eq14_e1078_d_n14 * p.p32);
        let eq14_e1080_d_n15: f64 = (eq14_e1078_d_n15 * p.p32);
        let eq14_e1080_d_n16: f64 = (eq14_e1078_d_n16 * p.p32);
        let eq14_e1080_d_n17: f64 = (eq14_e1078_d_n17 * p.p32);
        let eq14_e1080_d_n18: f64 = (eq14_e1078_d_n18 * p.p32);
        let eq14_e1080_d_n19: f64 = (eq14_e1078_d_n19 * p.p32);
        let eq14_e1080_d_n20: f64 = (eq14_e1078_d_n20 * p.p32);
        let eq14_e1080_d_b0: f64 = (eq14_e1078_d_b0 * p.p32);
        let eq14_e1080_d_b1: f64 = (eq14_e1078_d_b1 * p.p32);
        let eq14_e1080_d_b2: f64 = (eq14_e1078_d_b2 * p.p32);
        let eq14_e1080_d_b3: f64 = (eq14_e1078_d_b3 * p.p32);
        let eq14_e1080_d_b4: f64 = (eq14_e1078_d_b4 * p.p32);
        let eq14_e1080_d_b5: f64 = (eq14_e1078_d_b5 * p.p32);
        let eq14_e1080_d_b6: f64 = (eq14_e1078_d_b6 * p.p32);
        let eq14_e1080_d_b7: f64 = (eq14_e1078_d_b7 * p.p32);
        let eq14_e1080_d_b8: f64 = (eq14_e1078_d_b8 * p.p32);
        let eq14_e1080_d_b9: f64 = (eq14_e1078_d_b9 * p.p32);
        let eq14_e1080_d_b10: f64 = (eq14_e1078_d_b10 * p.p32);
        let eq14_e1080_d_b11: f64 = (eq14_e1078_d_b11 * p.p32);
        let eq14_e1080_d_b12: f64 = (eq14_e1078_d_b12 * p.p32);
        let eq14_e1080_d_b13: f64 = (eq14_e1078_d_b13 * p.p32);
        let eq14_e1080_d_b14: f64 = (eq14_e1078_d_b14 * p.p32);
        let eq14_e1080_d_b15: f64 = (eq14_e1078_d_b15 * p.p32);
        let eq14_e1080_d_b16: f64 = (eq14_e1078_d_b16 * p.p32);
        let eq14_e1080_d_b17: f64 = (eq14_e1078_d_b17 * p.p32);
        let eq14_e1080_d_b18: f64 = (eq14_e1078_d_b18 * p.p32);
        let eq14_e1080_d_b19: f64 = (eq14_e1078_d_b19 * p.p32);
        let eq14_e1080_d_b20: f64 = (eq14_e1078_d_b20 * p.p32);
        let eq14_e1080_d_b21: f64 = (eq14_e1078_d_b21 * p.p32);
        let eq14_e1080_d_b22: f64 = (eq14_e1078_d_b22 * p.p32);
        let eq14_e1080_d_b23: f64 = (eq14_e1078_d_b23 * p.p32);
        let eq14_e1080_d_b24: f64 = (eq14_e1078_d_b24 * p.p32);
        let eq14_e1082: f64 = (eq14_e1080 * s.v[849]);
        let eq14_e1082_d_n0: f64 = ((eq14_e1080_d_n0 * s.v[849]) + (eq14_e1080 * s.dn[849][0]));
        let eq14_e1082_d_n1: f64 = ((eq14_e1080_d_n1 * s.v[849]) + (eq14_e1080 * s.dn[849][1]));
        let eq14_e1082_d_n2: f64 = ((eq14_e1080_d_n2 * s.v[849]) + (eq14_e1080 * s.dn[849][2]));
        let eq14_e1082_d_n3: f64 = ((eq14_e1080_d_n3 * s.v[849]) + (eq14_e1080 * s.dn[849][3]));
        let eq14_e1082_d_n4: f64 = ((eq14_e1080_d_n4 * s.v[849]) + (eq14_e1080 * s.dn[849][4]));
        let eq14_e1082_d_n5: f64 = ((eq14_e1080_d_n5 * s.v[849]) + (eq14_e1080 * s.dn[849][5]));
        let eq14_e1082_d_n6: f64 = ((eq14_e1080_d_n6 * s.v[849]) + (eq14_e1080 * s.dn[849][6]));
        let eq14_e1082_d_n7: f64 = ((eq14_e1080_d_n7 * s.v[849]) + (eq14_e1080 * s.dn[849][7]));
        let eq14_e1082_d_n8: f64 = ((eq14_e1080_d_n8 * s.v[849]) + (eq14_e1080 * s.dn[849][8]));
        let eq14_e1082_d_n9: f64 = ((eq14_e1080_d_n9 * s.v[849]) + (eq14_e1080 * s.dn[849][9]));
        let eq14_e1082_d_n10: f64 = ((eq14_e1080_d_n10 * s.v[849]) + (eq14_e1080 * s.dn[849][10]));
        let eq14_e1082_d_n11: f64 = ((eq14_e1080_d_n11 * s.v[849]) + (eq14_e1080 * s.dn[849][11]));
        let eq14_e1082_d_n12: f64 = ((eq14_e1080_d_n12 * s.v[849]) + (eq14_e1080 * s.dn[849][12]));
        let eq14_e1082_d_n13: f64 = ((eq14_e1080_d_n13 * s.v[849]) + (eq14_e1080 * s.dn[849][13]));
        let eq14_e1082_d_n14: f64 = ((eq14_e1080_d_n14 * s.v[849]) + (eq14_e1080 * s.dn[849][14]));
        let eq14_e1082_d_n15: f64 = ((eq14_e1080_d_n15 * s.v[849]) + (eq14_e1080 * s.dn[849][15]));
        let eq14_e1082_d_n16: f64 = ((eq14_e1080_d_n16 * s.v[849]) + (eq14_e1080 * s.dn[849][16]));
        let eq14_e1082_d_n17: f64 = ((eq14_e1080_d_n17 * s.v[849]) + (eq14_e1080 * s.dn[849][17]));
        let eq14_e1082_d_n18: f64 = ((eq14_e1080_d_n18 * s.v[849]) + (eq14_e1080 * s.dn[849][18]));
        let eq14_e1082_d_n19: f64 = ((eq14_e1080_d_n19 * s.v[849]) + (eq14_e1080 * s.dn[849][19]));
        let eq14_e1082_d_n20: f64 = ((eq14_e1080_d_n20 * s.v[849]) + (eq14_e1080 * s.dn[849][20]));
        let eq14_e1082_d_b0: f64 = ((eq14_e1080_d_b0 * s.v[849]) + (eq14_e1080 * s.db[849][0]));
        let eq14_e1082_d_b1: f64 = ((eq14_e1080_d_b1 * s.v[849]) + (eq14_e1080 * s.db[849][1]));
        let eq14_e1082_d_b2: f64 = ((eq14_e1080_d_b2 * s.v[849]) + (eq14_e1080 * s.db[849][2]));
        let eq14_e1082_d_b3: f64 = ((eq14_e1080_d_b3 * s.v[849]) + (eq14_e1080 * s.db[849][3]));
        let eq14_e1082_d_b4: f64 = ((eq14_e1080_d_b4 * s.v[849]) + (eq14_e1080 * s.db[849][4]));
        let eq14_e1082_d_b5: f64 = ((eq14_e1080_d_b5 * s.v[849]) + (eq14_e1080 * s.db[849][5]));
        let eq14_e1082_d_b6: f64 = ((eq14_e1080_d_b6 * s.v[849]) + (eq14_e1080 * s.db[849][6]));
        let eq14_e1082_d_b7: f64 = ((eq14_e1080_d_b7 * s.v[849]) + (eq14_e1080 * s.db[849][7]));
        let eq14_e1082_d_b8: f64 = ((eq14_e1080_d_b8 * s.v[849]) + (eq14_e1080 * s.db[849][8]));
        let eq14_e1082_d_b9: f64 = ((eq14_e1080_d_b9 * s.v[849]) + (eq14_e1080 * s.db[849][9]));
        let eq14_e1082_d_b10: f64 = ((eq14_e1080_d_b10 * s.v[849]) + (eq14_e1080 * s.db[849][10]));
        let eq14_e1082_d_b11: f64 = ((eq14_e1080_d_b11 * s.v[849]) + (eq14_e1080 * s.db[849][11]));
        let eq14_e1082_d_b12: f64 = ((eq14_e1080_d_b12 * s.v[849]) + (eq14_e1080 * s.db[849][12]));
        let eq14_e1082_d_b13: f64 = ((eq14_e1080_d_b13 * s.v[849]) + (eq14_e1080 * s.db[849][13]));
        let eq14_e1082_d_b14: f64 = ((eq14_e1080_d_b14 * s.v[849]) + (eq14_e1080 * s.db[849][14]));
        let eq14_e1082_d_b15: f64 = ((eq14_e1080_d_b15 * s.v[849]) + (eq14_e1080 * s.db[849][15]));
        let eq14_e1082_d_b16: f64 = ((eq14_e1080_d_b16 * s.v[849]) + (eq14_e1080 * s.db[849][16]));
        let eq14_e1082_d_b17: f64 = ((eq14_e1080_d_b17 * s.v[849]) + (eq14_e1080 * s.db[849][17]));
        let eq14_e1082_d_b18: f64 = ((eq14_e1080_d_b18 * s.v[849]) + (eq14_e1080 * s.db[849][18]));
        let eq14_e1082_d_b19: f64 = ((eq14_e1080_d_b19 * s.v[849]) + (eq14_e1080 * s.db[849][19]));
        let eq14_e1082_d_b20: f64 = ((eq14_e1080_d_b20 * s.v[849]) + (eq14_e1080 * s.db[849][20]));
        let eq14_e1082_d_b21: f64 = ((eq14_e1080_d_b21 * s.v[849]) + (eq14_e1080 * s.db[849][21]));
        let eq14_e1082_d_b22: f64 = ((eq14_e1080_d_b22 * s.v[849]) + (eq14_e1080 * s.db[849][22]));
        let eq14_e1082_d_b23: f64 = ((eq14_e1080_d_b23 * s.v[849]) + (eq14_e1080 * s.db[849][23]));
        let eq14_e1082_d_b24: f64 = ((eq14_e1080_d_b24 * s.v[849]) + (eq14_e1080 * s.db[849][24]));
        let eq14_value: f64 = eq14_e1082;
        let eq14_node_derivatives: [f64; 21] = [eq14_e1082_d_n0, eq14_e1082_d_n1, eq14_e1082_d_n2, eq14_e1082_d_n3, eq14_e1082_d_n4, eq14_e1082_d_n5, eq14_e1082_d_n6, eq14_e1082_d_n7, eq14_e1082_d_n8, eq14_e1082_d_n9, eq14_e1082_d_n10, eq14_e1082_d_n11, eq14_e1082_d_n12, eq14_e1082_d_n13, eq14_e1082_d_n14, eq14_e1082_d_n15, eq14_e1082_d_n16, eq14_e1082_d_n17, eq14_e1082_d_n18, eq14_e1082_d_n19, eq14_e1082_d_n20];
        let eq14_branch_derivatives: [f64; 25] = [eq14_e1082_d_b0, eq14_e1082_d_b1, eq14_e1082_d_b2, eq14_e1082_d_b3, eq14_e1082_d_b4, eq14_e1082_d_b5, eq14_e1082_d_b6, eq14_e1082_d_b7, eq14_e1082_d_b8, eq14_e1082_d_b9, eq14_e1082_d_b10, eq14_e1082_d_b11, eq14_e1082_d_b12, eq14_e1082_d_b13, eq14_e1082_d_b14, eq14_e1082_d_b15, eq14_e1082_d_b16, eq14_e1082_d_b17, eq14_e1082_d_b18, eq14_e1082_d_b19, eq14_e1082_d_b20, eq14_e1082_d_b21, eq14_e1082_d_b22, eq14_e1082_d_b23, eq14_e1082_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq14_value),
            &nodes,
            &eq14_node_derivatives,
            &branches,
            &eq14_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq15_e1092, eq15_e1092_d_n0, eq15_e1092_d_n1, eq15_e1092_d_n2, eq15_e1092_d_n3, eq15_e1092_d_n4, eq15_e1092_d_n5, eq15_e1092_d_n6, eq15_e1092_d_n7, eq15_e1092_d_n8, eq15_e1092_d_n9, eq15_e1092_d_n10, eq15_e1092_d_n11, eq15_e1092_d_n12, eq15_e1092_d_n13, eq15_e1092_d_n14, eq15_e1092_d_n15, eq15_e1092_d_n16, eq15_e1092_d_n17, eq15_e1092_d_n18, eq15_e1092_d_n19, eq15_e1092_d_n20, eq15_e1092_d_b0, eq15_e1092_d_b1, eq15_e1092_d_b2, eq15_e1092_d_b3, eq15_e1092_d_b4, eq15_e1092_d_b5, eq15_e1092_d_b6, eq15_e1092_d_b7, eq15_e1092_d_b8, eq15_e1092_d_b9, eq15_e1092_d_b10, eq15_e1092_d_b11, eq15_e1092_d_b12, eq15_e1092_d_b13, eq15_e1092_d_b14, eq15_e1092_d_b15, eq15_e1092_d_b16, eq15_e1092_d_b17, eq15_e1092_d_b18, eq15_e1092_d_b19, eq15_e1092_d_b20, eq15_e1092_d_b21, eq15_e1092_d_b22, eq15_e1092_d_b23, eq15_e1092_d_b24,) = {
    if (s.v[2914] != 0.0) {
        let eq15_e1086: f64 = (s.v[19] * p.p32);
        let eq15_e1086_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq15_e1086_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq15_e1086_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq15_e1086_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq15_e1086_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq15_e1086_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq15_e1086_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq15_e1086_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq15_e1086_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq15_e1086_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq15_e1086_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq15_e1086_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq15_e1086_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq15_e1086_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq15_e1086_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq15_e1086_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq15_e1086_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq15_e1086_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq15_e1086_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq15_e1086_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq15_e1086_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq15_e1086_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq15_e1086_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq15_e1086_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq15_e1086_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq15_e1086_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq15_e1086_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq15_e1086_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq15_e1086_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq15_e1086_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq15_e1086_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq15_e1086_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq15_e1086_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq15_e1086_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq15_e1086_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq15_e1086_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq15_e1086_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq15_e1086_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq15_e1086_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq15_e1086_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq15_e1086_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq15_e1086_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq15_e1086_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq15_e1086_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq15_e1086_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq15_e1086_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq15_e1088: f64 = (eq15_e1086 * s.v[811]);
        let eq15_e1088_d_n0: f64 = ((eq15_e1086_d_n0 * s.v[811]) + (eq15_e1086 * s.dn[811][0]));
        let eq15_e1088_d_n1: f64 = ((eq15_e1086_d_n1 * s.v[811]) + (eq15_e1086 * s.dn[811][1]));
        let eq15_e1088_d_n2: f64 = ((eq15_e1086_d_n2 * s.v[811]) + (eq15_e1086 * s.dn[811][2]));
        let eq15_e1088_d_n3: f64 = ((eq15_e1086_d_n3 * s.v[811]) + (eq15_e1086 * s.dn[811][3]));
        let eq15_e1088_d_n4: f64 = ((eq15_e1086_d_n4 * s.v[811]) + (eq15_e1086 * s.dn[811][4]));
        let eq15_e1088_d_n5: f64 = ((eq15_e1086_d_n5 * s.v[811]) + (eq15_e1086 * s.dn[811][5]));
        let eq15_e1088_d_n6: f64 = ((eq15_e1086_d_n6 * s.v[811]) + (eq15_e1086 * s.dn[811][6]));
        let eq15_e1088_d_n7: f64 = ((eq15_e1086_d_n7 * s.v[811]) + (eq15_e1086 * s.dn[811][7]));
        let eq15_e1088_d_n8: f64 = ((eq15_e1086_d_n8 * s.v[811]) + (eq15_e1086 * s.dn[811][8]));
        let eq15_e1088_d_n9: f64 = ((eq15_e1086_d_n9 * s.v[811]) + (eq15_e1086 * s.dn[811][9]));
        let eq15_e1088_d_n10: f64 = ((eq15_e1086_d_n10 * s.v[811]) + (eq15_e1086 * s.dn[811][10]));
        let eq15_e1088_d_n11: f64 = ((eq15_e1086_d_n11 * s.v[811]) + (eq15_e1086 * s.dn[811][11]));
        let eq15_e1088_d_n12: f64 = ((eq15_e1086_d_n12 * s.v[811]) + (eq15_e1086 * s.dn[811][12]));
        let eq15_e1088_d_n13: f64 = ((eq15_e1086_d_n13 * s.v[811]) + (eq15_e1086 * s.dn[811][13]));
        let eq15_e1088_d_n14: f64 = ((eq15_e1086_d_n14 * s.v[811]) + (eq15_e1086 * s.dn[811][14]));
        let eq15_e1088_d_n15: f64 = ((eq15_e1086_d_n15 * s.v[811]) + (eq15_e1086 * s.dn[811][15]));
        let eq15_e1088_d_n16: f64 = ((eq15_e1086_d_n16 * s.v[811]) + (eq15_e1086 * s.dn[811][16]));
        let eq15_e1088_d_n17: f64 = ((eq15_e1086_d_n17 * s.v[811]) + (eq15_e1086 * s.dn[811][17]));
        let eq15_e1088_d_n18: f64 = ((eq15_e1086_d_n18 * s.v[811]) + (eq15_e1086 * s.dn[811][18]));
        let eq15_e1088_d_n19: f64 = ((eq15_e1086_d_n19 * s.v[811]) + (eq15_e1086 * s.dn[811][19]));
        let eq15_e1088_d_n20: f64 = ((eq15_e1086_d_n20 * s.v[811]) + (eq15_e1086 * s.dn[811][20]));
        let eq15_e1088_d_b0: f64 = ((eq15_e1086_d_b0 * s.v[811]) + (eq15_e1086 * s.db[811][0]));
        let eq15_e1088_d_b1: f64 = ((eq15_e1086_d_b1 * s.v[811]) + (eq15_e1086 * s.db[811][1]));
        let eq15_e1088_d_b2: f64 = ((eq15_e1086_d_b2 * s.v[811]) + (eq15_e1086 * s.db[811][2]));
        let eq15_e1088_d_b3: f64 = ((eq15_e1086_d_b3 * s.v[811]) + (eq15_e1086 * s.db[811][3]));
        let eq15_e1088_d_b4: f64 = ((eq15_e1086_d_b4 * s.v[811]) + (eq15_e1086 * s.db[811][4]));
        let eq15_e1088_d_b5: f64 = ((eq15_e1086_d_b5 * s.v[811]) + (eq15_e1086 * s.db[811][5]));
        let eq15_e1088_d_b6: f64 = ((eq15_e1086_d_b6 * s.v[811]) + (eq15_e1086 * s.db[811][6]));
        let eq15_e1088_d_b7: f64 = ((eq15_e1086_d_b7 * s.v[811]) + (eq15_e1086 * s.db[811][7]));
        let eq15_e1088_d_b8: f64 = ((eq15_e1086_d_b8 * s.v[811]) + (eq15_e1086 * s.db[811][8]));
        let eq15_e1088_d_b9: f64 = ((eq15_e1086_d_b9 * s.v[811]) + (eq15_e1086 * s.db[811][9]));
        let eq15_e1088_d_b10: f64 = ((eq15_e1086_d_b10 * s.v[811]) + (eq15_e1086 * s.db[811][10]));
        let eq15_e1088_d_b11: f64 = ((eq15_e1086_d_b11 * s.v[811]) + (eq15_e1086 * s.db[811][11]));
        let eq15_e1088_d_b12: f64 = ((eq15_e1086_d_b12 * s.v[811]) + (eq15_e1086 * s.db[811][12]));
        let eq15_e1088_d_b13: f64 = ((eq15_e1086_d_b13 * s.v[811]) + (eq15_e1086 * s.db[811][13]));
        let eq15_e1088_d_b14: f64 = ((eq15_e1086_d_b14 * s.v[811]) + (eq15_e1086 * s.db[811][14]));
        let eq15_e1088_d_b15: f64 = ((eq15_e1086_d_b15 * s.v[811]) + (eq15_e1086 * s.db[811][15]));
        let eq15_e1088_d_b16: f64 = ((eq15_e1086_d_b16 * s.v[811]) + (eq15_e1086 * s.db[811][16]));
        let eq15_e1088_d_b17: f64 = ((eq15_e1086_d_b17 * s.v[811]) + (eq15_e1086 * s.db[811][17]));
        let eq15_e1088_d_b18: f64 = ((eq15_e1086_d_b18 * s.v[811]) + (eq15_e1086 * s.db[811][18]));
        let eq15_e1088_d_b19: f64 = ((eq15_e1086_d_b19 * s.v[811]) + (eq15_e1086 * s.db[811][19]));
        let eq15_e1088_d_b20: f64 = ((eq15_e1086_d_b20 * s.v[811]) + (eq15_e1086 * s.db[811][20]));
        let eq15_e1088_d_b21: f64 = ((eq15_e1086_d_b21 * s.v[811]) + (eq15_e1086 * s.db[811][21]));
        let eq15_e1088_d_b22: f64 = ((eq15_e1086_d_b22 * s.v[811]) + (eq15_e1086 * s.db[811][22]));
        let eq15_e1088_d_b23: f64 = ((eq15_e1086_d_b23 * s.v[811]) + (eq15_e1086 * s.db[811][23]));
        let eq15_e1088_d_b24: f64 = ((eq15_e1086_d_b24 * s.v[811]) + (eq15_e1086 * s.db[811][24]));
        let eq15_e1090: f64 = (eq15_e1088 * (nv1 - nv5));
        let eq15_e1090_d_n0: f64 = (eq15_e1088_d_n0 * (nv1 - nv5));
        let eq15_e1090_d_n1: f64 = ((eq15_e1088_d_n1 * (nv1 - nv5)) + eq15_e1088);
        let eq15_e1090_d_n2: f64 = (eq15_e1088_d_n2 * (nv1 - nv5));
        let eq15_e1090_d_n3: f64 = (eq15_e1088_d_n3 * (nv1 - nv5));
        let eq15_e1090_d_n4: f64 = (eq15_e1088_d_n4 * (nv1 - nv5));
        let eq15_e1090_d_n5: f64 = ((eq15_e1088_d_n5 * (nv1 - nv5)) + (-eq15_e1088));
        let eq15_e1090_d_n6: f64 = (eq15_e1088_d_n6 * (nv1 - nv5));
        let eq15_e1090_d_n7: f64 = (eq15_e1088_d_n7 * (nv1 - nv5));
        let eq15_e1090_d_n8: f64 = (eq15_e1088_d_n8 * (nv1 - nv5));
        let eq15_e1090_d_n9: f64 = (eq15_e1088_d_n9 * (nv1 - nv5));
        let eq15_e1090_d_n10: f64 = (eq15_e1088_d_n10 * (nv1 - nv5));
        let eq15_e1090_d_n11: f64 = (eq15_e1088_d_n11 * (nv1 - nv5));
        let eq15_e1090_d_n12: f64 = (eq15_e1088_d_n12 * (nv1 - nv5));
        let eq15_e1090_d_n13: f64 = (eq15_e1088_d_n13 * (nv1 - nv5));
        let eq15_e1090_d_n14: f64 = (eq15_e1088_d_n14 * (nv1 - nv5));
        let eq15_e1090_d_n15: f64 = (eq15_e1088_d_n15 * (nv1 - nv5));
        let eq15_e1090_d_n16: f64 = (eq15_e1088_d_n16 * (nv1 - nv5));
        let eq15_e1090_d_n17: f64 = (eq15_e1088_d_n17 * (nv1 - nv5));
        let eq15_e1090_d_n18: f64 = (eq15_e1088_d_n18 * (nv1 - nv5));
        let eq15_e1090_d_n19: f64 = (eq15_e1088_d_n19 * (nv1 - nv5));
        let eq15_e1090_d_n20: f64 = (eq15_e1088_d_n20 * (nv1 - nv5));
        let eq15_e1090_d_b0: f64 = (eq15_e1088_d_b0 * (nv1 - nv5));
        let eq15_e1090_d_b1: f64 = (eq15_e1088_d_b1 * (nv1 - nv5));
        let eq15_e1090_d_b2: f64 = (eq15_e1088_d_b2 * (nv1 - nv5));
        let eq15_e1090_d_b3: f64 = (eq15_e1088_d_b3 * (nv1 - nv5));
        let eq15_e1090_d_b4: f64 = (eq15_e1088_d_b4 * (nv1 - nv5));
        let eq15_e1090_d_b5: f64 = (eq15_e1088_d_b5 * (nv1 - nv5));
        let eq15_e1090_d_b6: f64 = (eq15_e1088_d_b6 * (nv1 - nv5));
        let eq15_e1090_d_b7: f64 = (eq15_e1088_d_b7 * (nv1 - nv5));
        let eq15_e1090_d_b8: f64 = (eq15_e1088_d_b8 * (nv1 - nv5));
        let eq15_e1090_d_b9: f64 = (eq15_e1088_d_b9 * (nv1 - nv5));
        let eq15_e1090_d_b10: f64 = (eq15_e1088_d_b10 * (nv1 - nv5));
        let eq15_e1090_d_b11: f64 = (eq15_e1088_d_b11 * (nv1 - nv5));
        let eq15_e1090_d_b12: f64 = (eq15_e1088_d_b12 * (nv1 - nv5));
        let eq15_e1090_d_b13: f64 = (eq15_e1088_d_b13 * (nv1 - nv5));
        let eq15_e1090_d_b14: f64 = (eq15_e1088_d_b14 * (nv1 - nv5));
        let eq15_e1090_d_b15: f64 = (eq15_e1088_d_b15 * (nv1 - nv5));
        let eq15_e1090_d_b16: f64 = (eq15_e1088_d_b16 * (nv1 - nv5));
        let eq15_e1090_d_b17: f64 = (eq15_e1088_d_b17 * (nv1 - nv5));
        let eq15_e1090_d_b18: f64 = (eq15_e1088_d_b18 * (nv1 - nv5));
        let eq15_e1090_d_b19: f64 = (eq15_e1088_d_b19 * (nv1 - nv5));
        let eq15_e1090_d_b20: f64 = (eq15_e1088_d_b20 * (nv1 - nv5));
        let eq15_e1090_d_b21: f64 = (eq15_e1088_d_b21 * (nv1 - nv5));
        let eq15_e1090_d_b22: f64 = (eq15_e1088_d_b22 * (nv1 - nv5));
        let eq15_e1090_d_b23: f64 = (eq15_e1088_d_b23 * (nv1 - nv5));
        let eq15_e1090_d_b24: f64 = (eq15_e1088_d_b24 * (nv1 - nv5));
        (eq15_e1090, eq15_e1090_d_n0, eq15_e1090_d_n1, eq15_e1090_d_n2, eq15_e1090_d_n3, eq15_e1090_d_n4, eq15_e1090_d_n5, eq15_e1090_d_n6, eq15_e1090_d_n7, eq15_e1090_d_n8, eq15_e1090_d_n9, eq15_e1090_d_n10, eq15_e1090_d_n11, eq15_e1090_d_n12, eq15_e1090_d_n13, eq15_e1090_d_n14, eq15_e1090_d_n15, eq15_e1090_d_n16, eq15_e1090_d_n17, eq15_e1090_d_n18, eq15_e1090_d_n19, eq15_e1090_d_n20, eq15_e1090_d_b0, eq15_e1090_d_b1, eq15_e1090_d_b2, eq15_e1090_d_b3, eq15_e1090_d_b4, eq15_e1090_d_b5, eq15_e1090_d_b6, eq15_e1090_d_b7, eq15_e1090_d_b8, eq15_e1090_d_b9, eq15_e1090_d_b10, eq15_e1090_d_b11, eq15_e1090_d_b12, eq15_e1090_d_b13, eq15_e1090_d_b14, eq15_e1090_d_b15, eq15_e1090_d_b16, eq15_e1090_d_b17, eq15_e1090_d_b18, eq15_e1090_d_b19, eq15_e1090_d_b20, eq15_e1090_d_b21, eq15_e1090_d_b22, eq15_e1090_d_b23, eq15_e1090_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1092;
        let eq15_node_derivatives: [f64; 21] = [eq15_e1092_d_n0, eq15_e1092_d_n1, eq15_e1092_d_n2, eq15_e1092_d_n3, eq15_e1092_d_n4, eq15_e1092_d_n5, eq15_e1092_d_n6, eq15_e1092_d_n7, eq15_e1092_d_n8, eq15_e1092_d_n9, eq15_e1092_d_n10, eq15_e1092_d_n11, eq15_e1092_d_n12, eq15_e1092_d_n13, eq15_e1092_d_n14, eq15_e1092_d_n15, eq15_e1092_d_n16, eq15_e1092_d_n17, eq15_e1092_d_n18, eq15_e1092_d_n19, eq15_e1092_d_n20];
        let eq15_branch_derivatives: [f64; 25] = [eq15_e1092_d_b0, eq15_e1092_d_b1, eq15_e1092_d_b2, eq15_e1092_d_b3, eq15_e1092_d_b4, eq15_e1092_d_b5, eq15_e1092_d_b6, eq15_e1092_d_b7, eq15_e1092_d_b8, eq15_e1092_d_b9, eq15_e1092_d_b10, eq15_e1092_d_b11, eq15_e1092_d_b12, eq15_e1092_d_b13, eq15_e1092_d_b14, eq15_e1092_d_b15, eq15_e1092_d_b16, eq15_e1092_d_b17, eq15_e1092_d_b18, eq15_e1092_d_b19, eq15_e1092_d_b20, eq15_e1092_d_b21, eq15_e1092_d_b22, eq15_e1092_d_b23, eq15_e1092_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq15_value),
            &nodes,
            &eq15_node_derivatives,
            &branches,
            &eq15_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_16_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq16_e1102,) = {
    if (s.v[2914] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e1102;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq16_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq17_e1107,) = {
    if (!(s.v[2914] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1107;
        stamper.stamp_potential(
            branches[0],
            eq17_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_18_block_0(
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq18_e1117, eq18_e1117_d_n0, eq18_e1117_d_n1, eq18_e1117_d_n2, eq18_e1117_d_n3, eq18_e1117_d_n4, eq18_e1117_d_n5, eq18_e1117_d_n6, eq18_e1117_d_n7, eq18_e1117_d_n8, eq18_e1117_d_n9, eq18_e1117_d_n10, eq18_e1117_d_n11, eq18_e1117_d_n12, eq18_e1117_d_n13, eq18_e1117_d_n14, eq18_e1117_d_n15, eq18_e1117_d_n16, eq18_e1117_d_n17, eq18_e1117_d_n18, eq18_e1117_d_n19, eq18_e1117_d_n20, eq18_e1117_d_b0, eq18_e1117_d_b1, eq18_e1117_d_b2, eq18_e1117_d_b3, eq18_e1117_d_b4, eq18_e1117_d_b5, eq18_e1117_d_b6, eq18_e1117_d_b7, eq18_e1117_d_b8, eq18_e1117_d_b9, eq18_e1117_d_b10, eq18_e1117_d_b11, eq18_e1117_d_b12, eq18_e1117_d_b13, eq18_e1117_d_b14, eq18_e1117_d_b15, eq18_e1117_d_b16, eq18_e1117_d_b17, eq18_e1117_d_b18, eq18_e1117_d_b19, eq18_e1117_d_b20, eq18_e1117_d_b21, eq18_e1117_d_b22, eq18_e1117_d_b23, eq18_e1117_d_b24,) = {
    if (s.v[2915] != 0.0) {
        let eq18_e1111: f64 = (s.v[19] * p.p32);
        let eq18_e1111_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq18_e1111_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq18_e1111_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq18_e1111_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq18_e1111_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq18_e1111_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq18_e1111_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq18_e1111_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq18_e1111_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq18_e1111_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq18_e1111_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq18_e1111_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq18_e1111_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq18_e1111_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq18_e1111_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq18_e1111_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq18_e1111_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq18_e1111_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq18_e1111_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq18_e1111_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq18_e1111_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq18_e1111_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq18_e1111_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq18_e1111_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq18_e1111_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq18_e1111_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq18_e1111_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq18_e1111_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq18_e1111_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq18_e1111_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq18_e1111_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq18_e1111_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq18_e1111_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq18_e1111_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq18_e1111_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq18_e1111_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq18_e1111_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq18_e1111_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq18_e1111_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq18_e1111_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq18_e1111_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq18_e1111_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq18_e1111_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq18_e1111_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq18_e1111_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq18_e1111_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq18_e1113: f64 = (eq18_e1111 * s.v[812]);
        let eq18_e1113_d_n0: f64 = ((eq18_e1111_d_n0 * s.v[812]) + (eq18_e1111 * s.dn[812][0]));
        let eq18_e1113_d_n1: f64 = ((eq18_e1111_d_n1 * s.v[812]) + (eq18_e1111 * s.dn[812][1]));
        let eq18_e1113_d_n2: f64 = ((eq18_e1111_d_n2 * s.v[812]) + (eq18_e1111 * s.dn[812][2]));
        let eq18_e1113_d_n3: f64 = ((eq18_e1111_d_n3 * s.v[812]) + (eq18_e1111 * s.dn[812][3]));
        let eq18_e1113_d_n4: f64 = ((eq18_e1111_d_n4 * s.v[812]) + (eq18_e1111 * s.dn[812][4]));
        let eq18_e1113_d_n5: f64 = ((eq18_e1111_d_n5 * s.v[812]) + (eq18_e1111 * s.dn[812][5]));
        let eq18_e1113_d_n6: f64 = ((eq18_e1111_d_n6 * s.v[812]) + (eq18_e1111 * s.dn[812][6]));
        let eq18_e1113_d_n7: f64 = ((eq18_e1111_d_n7 * s.v[812]) + (eq18_e1111 * s.dn[812][7]));
        let eq18_e1113_d_n8: f64 = ((eq18_e1111_d_n8 * s.v[812]) + (eq18_e1111 * s.dn[812][8]));
        let eq18_e1113_d_n9: f64 = ((eq18_e1111_d_n9 * s.v[812]) + (eq18_e1111 * s.dn[812][9]));
        let eq18_e1113_d_n10: f64 = ((eq18_e1111_d_n10 * s.v[812]) + (eq18_e1111 * s.dn[812][10]));
        let eq18_e1113_d_n11: f64 = ((eq18_e1111_d_n11 * s.v[812]) + (eq18_e1111 * s.dn[812][11]));
        let eq18_e1113_d_n12: f64 = ((eq18_e1111_d_n12 * s.v[812]) + (eq18_e1111 * s.dn[812][12]));
        let eq18_e1113_d_n13: f64 = ((eq18_e1111_d_n13 * s.v[812]) + (eq18_e1111 * s.dn[812][13]));
        let eq18_e1113_d_n14: f64 = ((eq18_e1111_d_n14 * s.v[812]) + (eq18_e1111 * s.dn[812][14]));
        let eq18_e1113_d_n15: f64 = ((eq18_e1111_d_n15 * s.v[812]) + (eq18_e1111 * s.dn[812][15]));
        let eq18_e1113_d_n16: f64 = ((eq18_e1111_d_n16 * s.v[812]) + (eq18_e1111 * s.dn[812][16]));
        let eq18_e1113_d_n17: f64 = ((eq18_e1111_d_n17 * s.v[812]) + (eq18_e1111 * s.dn[812][17]));
        let eq18_e1113_d_n18: f64 = ((eq18_e1111_d_n18 * s.v[812]) + (eq18_e1111 * s.dn[812][18]));
        let eq18_e1113_d_n19: f64 = ((eq18_e1111_d_n19 * s.v[812]) + (eq18_e1111 * s.dn[812][19]));
        let eq18_e1113_d_n20: f64 = ((eq18_e1111_d_n20 * s.v[812]) + (eq18_e1111 * s.dn[812][20]));
        let eq18_e1113_d_b0: f64 = ((eq18_e1111_d_b0 * s.v[812]) + (eq18_e1111 * s.db[812][0]));
        let eq18_e1113_d_b1: f64 = ((eq18_e1111_d_b1 * s.v[812]) + (eq18_e1111 * s.db[812][1]));
        let eq18_e1113_d_b2: f64 = ((eq18_e1111_d_b2 * s.v[812]) + (eq18_e1111 * s.db[812][2]));
        let eq18_e1113_d_b3: f64 = ((eq18_e1111_d_b3 * s.v[812]) + (eq18_e1111 * s.db[812][3]));
        let eq18_e1113_d_b4: f64 = ((eq18_e1111_d_b4 * s.v[812]) + (eq18_e1111 * s.db[812][4]));
        let eq18_e1113_d_b5: f64 = ((eq18_e1111_d_b5 * s.v[812]) + (eq18_e1111 * s.db[812][5]));
        let eq18_e1113_d_b6: f64 = ((eq18_e1111_d_b6 * s.v[812]) + (eq18_e1111 * s.db[812][6]));
        let eq18_e1113_d_b7: f64 = ((eq18_e1111_d_b7 * s.v[812]) + (eq18_e1111 * s.db[812][7]));
        let eq18_e1113_d_b8: f64 = ((eq18_e1111_d_b8 * s.v[812]) + (eq18_e1111 * s.db[812][8]));
        let eq18_e1113_d_b9: f64 = ((eq18_e1111_d_b9 * s.v[812]) + (eq18_e1111 * s.db[812][9]));
        let eq18_e1113_d_b10: f64 = ((eq18_e1111_d_b10 * s.v[812]) + (eq18_e1111 * s.db[812][10]));
        let eq18_e1113_d_b11: f64 = ((eq18_e1111_d_b11 * s.v[812]) + (eq18_e1111 * s.db[812][11]));
        let eq18_e1113_d_b12: f64 = ((eq18_e1111_d_b12 * s.v[812]) + (eq18_e1111 * s.db[812][12]));
        let eq18_e1113_d_b13: f64 = ((eq18_e1111_d_b13 * s.v[812]) + (eq18_e1111 * s.db[812][13]));
        let eq18_e1113_d_b14: f64 = ((eq18_e1111_d_b14 * s.v[812]) + (eq18_e1111 * s.db[812][14]));
        let eq18_e1113_d_b15: f64 = ((eq18_e1111_d_b15 * s.v[812]) + (eq18_e1111 * s.db[812][15]));
        let eq18_e1113_d_b16: f64 = ((eq18_e1111_d_b16 * s.v[812]) + (eq18_e1111 * s.db[812][16]));
        let eq18_e1113_d_b17: f64 = ((eq18_e1111_d_b17 * s.v[812]) + (eq18_e1111 * s.db[812][17]));
        let eq18_e1113_d_b18: f64 = ((eq18_e1111_d_b18 * s.v[812]) + (eq18_e1111 * s.db[812][18]));
        let eq18_e1113_d_b19: f64 = ((eq18_e1111_d_b19 * s.v[812]) + (eq18_e1111 * s.db[812][19]));
        let eq18_e1113_d_b20: f64 = ((eq18_e1111_d_b20 * s.v[812]) + (eq18_e1111 * s.db[812][20]));
        let eq18_e1113_d_b21: f64 = ((eq18_e1111_d_b21 * s.v[812]) + (eq18_e1111 * s.db[812][21]));
        let eq18_e1113_d_b22: f64 = ((eq18_e1111_d_b22 * s.v[812]) + (eq18_e1111 * s.db[812][22]));
        let eq18_e1113_d_b23: f64 = ((eq18_e1111_d_b23 * s.v[812]) + (eq18_e1111 * s.db[812][23]));
        let eq18_e1113_d_b24: f64 = ((eq18_e1111_d_b24 * s.v[812]) + (eq18_e1111 * s.db[812][24]));
        let eq18_e1115: f64 = (eq18_e1113 * (nv2 - nv6));
        let eq18_e1115_d_n0: f64 = (eq18_e1113_d_n0 * (nv2 - nv6));
        let eq18_e1115_d_n1: f64 = (eq18_e1113_d_n1 * (nv2 - nv6));
        let eq18_e1115_d_n2: f64 = ((eq18_e1113_d_n2 * (nv2 - nv6)) + eq18_e1113);
        let eq18_e1115_d_n3: f64 = (eq18_e1113_d_n3 * (nv2 - nv6));
        let eq18_e1115_d_n4: f64 = (eq18_e1113_d_n4 * (nv2 - nv6));
        let eq18_e1115_d_n5: f64 = (eq18_e1113_d_n5 * (nv2 - nv6));
        let eq18_e1115_d_n6: f64 = ((eq18_e1113_d_n6 * (nv2 - nv6)) + (-eq18_e1113));
        let eq18_e1115_d_n7: f64 = (eq18_e1113_d_n7 * (nv2 - nv6));
        let eq18_e1115_d_n8: f64 = (eq18_e1113_d_n8 * (nv2 - nv6));
        let eq18_e1115_d_n9: f64 = (eq18_e1113_d_n9 * (nv2 - nv6));
        let eq18_e1115_d_n10: f64 = (eq18_e1113_d_n10 * (nv2 - nv6));
        let eq18_e1115_d_n11: f64 = (eq18_e1113_d_n11 * (nv2 - nv6));
        let eq18_e1115_d_n12: f64 = (eq18_e1113_d_n12 * (nv2 - nv6));
        let eq18_e1115_d_n13: f64 = (eq18_e1113_d_n13 * (nv2 - nv6));
        let eq18_e1115_d_n14: f64 = (eq18_e1113_d_n14 * (nv2 - nv6));
        let eq18_e1115_d_n15: f64 = (eq18_e1113_d_n15 * (nv2 - nv6));
        let eq18_e1115_d_n16: f64 = (eq18_e1113_d_n16 * (nv2 - nv6));
        let eq18_e1115_d_n17: f64 = (eq18_e1113_d_n17 * (nv2 - nv6));
        let eq18_e1115_d_n18: f64 = (eq18_e1113_d_n18 * (nv2 - nv6));
        let eq18_e1115_d_n19: f64 = (eq18_e1113_d_n19 * (nv2 - nv6));
        let eq18_e1115_d_n20: f64 = (eq18_e1113_d_n20 * (nv2 - nv6));
        let eq18_e1115_d_b0: f64 = (eq18_e1113_d_b0 * (nv2 - nv6));
        let eq18_e1115_d_b1: f64 = (eq18_e1113_d_b1 * (nv2 - nv6));
        let eq18_e1115_d_b2: f64 = (eq18_e1113_d_b2 * (nv2 - nv6));
        let eq18_e1115_d_b3: f64 = (eq18_e1113_d_b3 * (nv2 - nv6));
        let eq18_e1115_d_b4: f64 = (eq18_e1113_d_b4 * (nv2 - nv6));
        let eq18_e1115_d_b5: f64 = (eq18_e1113_d_b5 * (nv2 - nv6));
        let eq18_e1115_d_b6: f64 = (eq18_e1113_d_b6 * (nv2 - nv6));
        let eq18_e1115_d_b7: f64 = (eq18_e1113_d_b7 * (nv2 - nv6));
        let eq18_e1115_d_b8: f64 = (eq18_e1113_d_b8 * (nv2 - nv6));
        let eq18_e1115_d_b9: f64 = (eq18_e1113_d_b9 * (nv2 - nv6));
        let eq18_e1115_d_b10: f64 = (eq18_e1113_d_b10 * (nv2 - nv6));
        let eq18_e1115_d_b11: f64 = (eq18_e1113_d_b11 * (nv2 - nv6));
        let eq18_e1115_d_b12: f64 = (eq18_e1113_d_b12 * (nv2 - nv6));
        let eq18_e1115_d_b13: f64 = (eq18_e1113_d_b13 * (nv2 - nv6));
        let eq18_e1115_d_b14: f64 = (eq18_e1113_d_b14 * (nv2 - nv6));
        let eq18_e1115_d_b15: f64 = (eq18_e1113_d_b15 * (nv2 - nv6));
        let eq18_e1115_d_b16: f64 = (eq18_e1113_d_b16 * (nv2 - nv6));
        let eq18_e1115_d_b17: f64 = (eq18_e1113_d_b17 * (nv2 - nv6));
        let eq18_e1115_d_b18: f64 = (eq18_e1113_d_b18 * (nv2 - nv6));
        let eq18_e1115_d_b19: f64 = (eq18_e1113_d_b19 * (nv2 - nv6));
        let eq18_e1115_d_b20: f64 = (eq18_e1113_d_b20 * (nv2 - nv6));
        let eq18_e1115_d_b21: f64 = (eq18_e1113_d_b21 * (nv2 - nv6));
        let eq18_e1115_d_b22: f64 = (eq18_e1113_d_b22 * (nv2 - nv6));
        let eq18_e1115_d_b23: f64 = (eq18_e1113_d_b23 * (nv2 - nv6));
        let eq18_e1115_d_b24: f64 = (eq18_e1113_d_b24 * (nv2 - nv6));
        (eq18_e1115, eq18_e1115_d_n0, eq18_e1115_d_n1, eq18_e1115_d_n2, eq18_e1115_d_n3, eq18_e1115_d_n4, eq18_e1115_d_n5, eq18_e1115_d_n6, eq18_e1115_d_n7, eq18_e1115_d_n8, eq18_e1115_d_n9, eq18_e1115_d_n10, eq18_e1115_d_n11, eq18_e1115_d_n12, eq18_e1115_d_n13, eq18_e1115_d_n14, eq18_e1115_d_n15, eq18_e1115_d_n16, eq18_e1115_d_n17, eq18_e1115_d_n18, eq18_e1115_d_n19, eq18_e1115_d_n20, eq18_e1115_d_b0, eq18_e1115_d_b1, eq18_e1115_d_b2, eq18_e1115_d_b3, eq18_e1115_d_b4, eq18_e1115_d_b5, eq18_e1115_d_b6, eq18_e1115_d_b7, eq18_e1115_d_b8, eq18_e1115_d_b9, eq18_e1115_d_b10, eq18_e1115_d_b11, eq18_e1115_d_b12, eq18_e1115_d_b13, eq18_e1115_d_b14, eq18_e1115_d_b15, eq18_e1115_d_b16, eq18_e1115_d_b17, eq18_e1115_d_b18, eq18_e1115_d_b19, eq18_e1115_d_b20, eq18_e1115_d_b21, eq18_e1115_d_b22, eq18_e1115_d_b23, eq18_e1115_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1117;
        let eq18_node_derivatives: [f64; 21] = [eq18_e1117_d_n0, eq18_e1117_d_n1, eq18_e1117_d_n2, eq18_e1117_d_n3, eq18_e1117_d_n4, eq18_e1117_d_n5, eq18_e1117_d_n6, eq18_e1117_d_n7, eq18_e1117_d_n8, eq18_e1117_d_n9, eq18_e1117_d_n10, eq18_e1117_d_n11, eq18_e1117_d_n12, eq18_e1117_d_n13, eq18_e1117_d_n14, eq18_e1117_d_n15, eq18_e1117_d_n16, eq18_e1117_d_n17, eq18_e1117_d_n18, eq18_e1117_d_n19, eq18_e1117_d_n20];
        let eq18_branch_derivatives: [f64; 25] = [eq18_e1117_d_b0, eq18_e1117_d_b1, eq18_e1117_d_b2, eq18_e1117_d_b3, eq18_e1117_d_b4, eq18_e1117_d_b5, eq18_e1117_d_b6, eq18_e1117_d_b7, eq18_e1117_d_b8, eq18_e1117_d_b9, eq18_e1117_d_b10, eq18_e1117_d_b11, eq18_e1117_d_b12, eq18_e1117_d_b13, eq18_e1117_d_b14, eq18_e1117_d_b15, eq18_e1117_d_b16, eq18_e1117_d_b17, eq18_e1117_d_b18, eq18_e1117_d_b19, eq18_e1117_d_b20, eq18_e1117_d_b21, eq18_e1117_d_b22, eq18_e1117_d_b23, eq18_e1117_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[6]),
            self.multiplicity * (eq18_value),
            &nodes,
            &eq18_node_derivatives,
            &branches,
            &eq18_branch_derivatives,
            self.multiplicity,
        );
    }
}
