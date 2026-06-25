#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq19_e1127,) = {
    if (s.v[2915] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e1127;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[6]),
            self.multiplicity * (eq19_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq20_e1132,) = {
    if (!(s.v[2915] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1132;
        stamper.stamp_potential(
            branches[1],
            eq20_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_21_block_0(
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq21_e1142, eq21_e1142_d_n0, eq21_e1142_d_n1, eq21_e1142_d_n2, eq21_e1142_d_n3, eq21_e1142_d_n4, eq21_e1142_d_n5, eq21_e1142_d_n6, eq21_e1142_d_n7, eq21_e1142_d_n8, eq21_e1142_d_n9, eq21_e1142_d_n10, eq21_e1142_d_n11, eq21_e1142_d_n12, eq21_e1142_d_n13, eq21_e1142_d_n14, eq21_e1142_d_n15, eq21_e1142_d_n16, eq21_e1142_d_n17, eq21_e1142_d_n18, eq21_e1142_d_n19, eq21_e1142_d_n20, eq21_e1142_d_b0, eq21_e1142_d_b1, eq21_e1142_d_b2, eq21_e1142_d_b3, eq21_e1142_d_b4, eq21_e1142_d_b5, eq21_e1142_d_b6, eq21_e1142_d_b7, eq21_e1142_d_b8, eq21_e1142_d_b9, eq21_e1142_d_b10, eq21_e1142_d_b11, eq21_e1142_d_b12, eq21_e1142_d_b13, eq21_e1142_d_b14, eq21_e1142_d_b15, eq21_e1142_d_b16, eq21_e1142_d_b17, eq21_e1142_d_b18, eq21_e1142_d_b19, eq21_e1142_d_b20, eq21_e1142_d_b21, eq21_e1142_d_b22, eq21_e1142_d_b23, eq21_e1142_d_b24,) = {
    if (s.v[2916] != 0.0) {
        let eq21_e1136: f64 = (s.v[19] * p.p32);
        let eq21_e1136_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq21_e1136_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq21_e1136_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq21_e1136_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq21_e1136_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq21_e1136_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq21_e1136_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq21_e1136_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq21_e1136_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq21_e1136_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq21_e1136_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq21_e1136_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq21_e1136_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq21_e1136_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq21_e1136_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq21_e1136_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq21_e1136_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq21_e1136_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq21_e1136_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq21_e1136_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq21_e1136_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq21_e1136_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq21_e1136_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq21_e1136_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq21_e1136_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq21_e1136_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq21_e1136_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq21_e1136_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq21_e1136_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq21_e1136_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq21_e1136_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq21_e1136_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq21_e1136_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq21_e1136_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq21_e1136_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq21_e1136_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq21_e1136_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq21_e1136_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq21_e1136_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq21_e1136_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq21_e1136_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq21_e1136_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq21_e1136_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq21_e1136_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq21_e1136_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq21_e1136_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq21_e1138: f64 = (eq21_e1136 * s.v[813]);
        let eq21_e1138_d_n0: f64 = ((eq21_e1136_d_n0 * s.v[813]) + (eq21_e1136 * s.dn[813][0]));
        let eq21_e1138_d_n1: f64 = ((eq21_e1136_d_n1 * s.v[813]) + (eq21_e1136 * s.dn[813][1]));
        let eq21_e1138_d_n2: f64 = ((eq21_e1136_d_n2 * s.v[813]) + (eq21_e1136 * s.dn[813][2]));
        let eq21_e1138_d_n3: f64 = ((eq21_e1136_d_n3 * s.v[813]) + (eq21_e1136 * s.dn[813][3]));
        let eq21_e1138_d_n4: f64 = ((eq21_e1136_d_n4 * s.v[813]) + (eq21_e1136 * s.dn[813][4]));
        let eq21_e1138_d_n5: f64 = ((eq21_e1136_d_n5 * s.v[813]) + (eq21_e1136 * s.dn[813][5]));
        let eq21_e1138_d_n6: f64 = ((eq21_e1136_d_n6 * s.v[813]) + (eq21_e1136 * s.dn[813][6]));
        let eq21_e1138_d_n7: f64 = ((eq21_e1136_d_n7 * s.v[813]) + (eq21_e1136 * s.dn[813][7]));
        let eq21_e1138_d_n8: f64 = ((eq21_e1136_d_n8 * s.v[813]) + (eq21_e1136 * s.dn[813][8]));
        let eq21_e1138_d_n9: f64 = ((eq21_e1136_d_n9 * s.v[813]) + (eq21_e1136 * s.dn[813][9]));
        let eq21_e1138_d_n10: f64 = ((eq21_e1136_d_n10 * s.v[813]) + (eq21_e1136 * s.dn[813][10]));
        let eq21_e1138_d_n11: f64 = ((eq21_e1136_d_n11 * s.v[813]) + (eq21_e1136 * s.dn[813][11]));
        let eq21_e1138_d_n12: f64 = ((eq21_e1136_d_n12 * s.v[813]) + (eq21_e1136 * s.dn[813][12]));
        let eq21_e1138_d_n13: f64 = ((eq21_e1136_d_n13 * s.v[813]) + (eq21_e1136 * s.dn[813][13]));
        let eq21_e1138_d_n14: f64 = ((eq21_e1136_d_n14 * s.v[813]) + (eq21_e1136 * s.dn[813][14]));
        let eq21_e1138_d_n15: f64 = ((eq21_e1136_d_n15 * s.v[813]) + (eq21_e1136 * s.dn[813][15]));
        let eq21_e1138_d_n16: f64 = ((eq21_e1136_d_n16 * s.v[813]) + (eq21_e1136 * s.dn[813][16]));
        let eq21_e1138_d_n17: f64 = ((eq21_e1136_d_n17 * s.v[813]) + (eq21_e1136 * s.dn[813][17]));
        let eq21_e1138_d_n18: f64 = ((eq21_e1136_d_n18 * s.v[813]) + (eq21_e1136 * s.dn[813][18]));
        let eq21_e1138_d_n19: f64 = ((eq21_e1136_d_n19 * s.v[813]) + (eq21_e1136 * s.dn[813][19]));
        let eq21_e1138_d_n20: f64 = ((eq21_e1136_d_n20 * s.v[813]) + (eq21_e1136 * s.dn[813][20]));
        let eq21_e1138_d_b0: f64 = ((eq21_e1136_d_b0 * s.v[813]) + (eq21_e1136 * s.db[813][0]));
        let eq21_e1138_d_b1: f64 = ((eq21_e1136_d_b1 * s.v[813]) + (eq21_e1136 * s.db[813][1]));
        let eq21_e1138_d_b2: f64 = ((eq21_e1136_d_b2 * s.v[813]) + (eq21_e1136 * s.db[813][2]));
        let eq21_e1138_d_b3: f64 = ((eq21_e1136_d_b3 * s.v[813]) + (eq21_e1136 * s.db[813][3]));
        let eq21_e1138_d_b4: f64 = ((eq21_e1136_d_b4 * s.v[813]) + (eq21_e1136 * s.db[813][4]));
        let eq21_e1138_d_b5: f64 = ((eq21_e1136_d_b5 * s.v[813]) + (eq21_e1136 * s.db[813][5]));
        let eq21_e1138_d_b6: f64 = ((eq21_e1136_d_b6 * s.v[813]) + (eq21_e1136 * s.db[813][6]));
        let eq21_e1138_d_b7: f64 = ((eq21_e1136_d_b7 * s.v[813]) + (eq21_e1136 * s.db[813][7]));
        let eq21_e1138_d_b8: f64 = ((eq21_e1136_d_b8 * s.v[813]) + (eq21_e1136 * s.db[813][8]));
        let eq21_e1138_d_b9: f64 = ((eq21_e1136_d_b9 * s.v[813]) + (eq21_e1136 * s.db[813][9]));
        let eq21_e1138_d_b10: f64 = ((eq21_e1136_d_b10 * s.v[813]) + (eq21_e1136 * s.db[813][10]));
        let eq21_e1138_d_b11: f64 = ((eq21_e1136_d_b11 * s.v[813]) + (eq21_e1136 * s.db[813][11]));
        let eq21_e1138_d_b12: f64 = ((eq21_e1136_d_b12 * s.v[813]) + (eq21_e1136 * s.db[813][12]));
        let eq21_e1138_d_b13: f64 = ((eq21_e1136_d_b13 * s.v[813]) + (eq21_e1136 * s.db[813][13]));
        let eq21_e1138_d_b14: f64 = ((eq21_e1136_d_b14 * s.v[813]) + (eq21_e1136 * s.db[813][14]));
        let eq21_e1138_d_b15: f64 = ((eq21_e1136_d_b15 * s.v[813]) + (eq21_e1136 * s.db[813][15]));
        let eq21_e1138_d_b16: f64 = ((eq21_e1136_d_b16 * s.v[813]) + (eq21_e1136 * s.db[813][16]));
        let eq21_e1138_d_b17: f64 = ((eq21_e1136_d_b17 * s.v[813]) + (eq21_e1136 * s.db[813][17]));
        let eq21_e1138_d_b18: f64 = ((eq21_e1136_d_b18 * s.v[813]) + (eq21_e1136 * s.db[813][18]));
        let eq21_e1138_d_b19: f64 = ((eq21_e1136_d_b19 * s.v[813]) + (eq21_e1136 * s.db[813][19]));
        let eq21_e1138_d_b20: f64 = ((eq21_e1136_d_b20 * s.v[813]) + (eq21_e1136 * s.db[813][20]));
        let eq21_e1138_d_b21: f64 = ((eq21_e1136_d_b21 * s.v[813]) + (eq21_e1136 * s.db[813][21]));
        let eq21_e1138_d_b22: f64 = ((eq21_e1136_d_b22 * s.v[813]) + (eq21_e1136 * s.db[813][22]));
        let eq21_e1138_d_b23: f64 = ((eq21_e1136_d_b23 * s.v[813]) + (eq21_e1136 * s.db[813][23]));
        let eq21_e1138_d_b24: f64 = ((eq21_e1136_d_b24 * s.v[813]) + (eq21_e1136 * s.db[813][24]));
        let eq21_e1140: f64 = (eq21_e1138 * (nv0 - nv7));
        let eq21_e1140_d_n0: f64 = ((eq21_e1138_d_n0 * (nv0 - nv7)) + eq21_e1138);
        let eq21_e1140_d_n1: f64 = (eq21_e1138_d_n1 * (nv0 - nv7));
        let eq21_e1140_d_n2: f64 = (eq21_e1138_d_n2 * (nv0 - nv7));
        let eq21_e1140_d_n3: f64 = (eq21_e1138_d_n3 * (nv0 - nv7));
        let eq21_e1140_d_n4: f64 = (eq21_e1138_d_n4 * (nv0 - nv7));
        let eq21_e1140_d_n5: f64 = (eq21_e1138_d_n5 * (nv0 - nv7));
        let eq21_e1140_d_n6: f64 = (eq21_e1138_d_n6 * (nv0 - nv7));
        let eq21_e1140_d_n7: f64 = ((eq21_e1138_d_n7 * (nv0 - nv7)) + (-eq21_e1138));
        let eq21_e1140_d_n8: f64 = (eq21_e1138_d_n8 * (nv0 - nv7));
        let eq21_e1140_d_n9: f64 = (eq21_e1138_d_n9 * (nv0 - nv7));
        let eq21_e1140_d_n10: f64 = (eq21_e1138_d_n10 * (nv0 - nv7));
        let eq21_e1140_d_n11: f64 = (eq21_e1138_d_n11 * (nv0 - nv7));
        let eq21_e1140_d_n12: f64 = (eq21_e1138_d_n12 * (nv0 - nv7));
        let eq21_e1140_d_n13: f64 = (eq21_e1138_d_n13 * (nv0 - nv7));
        let eq21_e1140_d_n14: f64 = (eq21_e1138_d_n14 * (nv0 - nv7));
        let eq21_e1140_d_n15: f64 = (eq21_e1138_d_n15 * (nv0 - nv7));
        let eq21_e1140_d_n16: f64 = (eq21_e1138_d_n16 * (nv0 - nv7));
        let eq21_e1140_d_n17: f64 = (eq21_e1138_d_n17 * (nv0 - nv7));
        let eq21_e1140_d_n18: f64 = (eq21_e1138_d_n18 * (nv0 - nv7));
        let eq21_e1140_d_n19: f64 = (eq21_e1138_d_n19 * (nv0 - nv7));
        let eq21_e1140_d_n20: f64 = (eq21_e1138_d_n20 * (nv0 - nv7));
        let eq21_e1140_d_b0: f64 = (eq21_e1138_d_b0 * (nv0 - nv7));
        let eq21_e1140_d_b1: f64 = (eq21_e1138_d_b1 * (nv0 - nv7));
        let eq21_e1140_d_b2: f64 = (eq21_e1138_d_b2 * (nv0 - nv7));
        let eq21_e1140_d_b3: f64 = (eq21_e1138_d_b3 * (nv0 - nv7));
        let eq21_e1140_d_b4: f64 = (eq21_e1138_d_b4 * (nv0 - nv7));
        let eq21_e1140_d_b5: f64 = (eq21_e1138_d_b5 * (nv0 - nv7));
        let eq21_e1140_d_b6: f64 = (eq21_e1138_d_b6 * (nv0 - nv7));
        let eq21_e1140_d_b7: f64 = (eq21_e1138_d_b7 * (nv0 - nv7));
        let eq21_e1140_d_b8: f64 = (eq21_e1138_d_b8 * (nv0 - nv7));
        let eq21_e1140_d_b9: f64 = (eq21_e1138_d_b9 * (nv0 - nv7));
        let eq21_e1140_d_b10: f64 = (eq21_e1138_d_b10 * (nv0 - nv7));
        let eq21_e1140_d_b11: f64 = (eq21_e1138_d_b11 * (nv0 - nv7));
        let eq21_e1140_d_b12: f64 = (eq21_e1138_d_b12 * (nv0 - nv7));
        let eq21_e1140_d_b13: f64 = (eq21_e1138_d_b13 * (nv0 - nv7));
        let eq21_e1140_d_b14: f64 = (eq21_e1138_d_b14 * (nv0 - nv7));
        let eq21_e1140_d_b15: f64 = (eq21_e1138_d_b15 * (nv0 - nv7));
        let eq21_e1140_d_b16: f64 = (eq21_e1138_d_b16 * (nv0 - nv7));
        let eq21_e1140_d_b17: f64 = (eq21_e1138_d_b17 * (nv0 - nv7));
        let eq21_e1140_d_b18: f64 = (eq21_e1138_d_b18 * (nv0 - nv7));
        let eq21_e1140_d_b19: f64 = (eq21_e1138_d_b19 * (nv0 - nv7));
        let eq21_e1140_d_b20: f64 = (eq21_e1138_d_b20 * (nv0 - nv7));
        let eq21_e1140_d_b21: f64 = (eq21_e1138_d_b21 * (nv0 - nv7));
        let eq21_e1140_d_b22: f64 = (eq21_e1138_d_b22 * (nv0 - nv7));
        let eq21_e1140_d_b23: f64 = (eq21_e1138_d_b23 * (nv0 - nv7));
        let eq21_e1140_d_b24: f64 = (eq21_e1138_d_b24 * (nv0 - nv7));
        (eq21_e1140, eq21_e1140_d_n0, eq21_e1140_d_n1, eq21_e1140_d_n2, eq21_e1140_d_n3, eq21_e1140_d_n4, eq21_e1140_d_n5, eq21_e1140_d_n6, eq21_e1140_d_n7, eq21_e1140_d_n8, eq21_e1140_d_n9, eq21_e1140_d_n10, eq21_e1140_d_n11, eq21_e1140_d_n12, eq21_e1140_d_n13, eq21_e1140_d_n14, eq21_e1140_d_n15, eq21_e1140_d_n16, eq21_e1140_d_n17, eq21_e1140_d_n18, eq21_e1140_d_n19, eq21_e1140_d_n20, eq21_e1140_d_b0, eq21_e1140_d_b1, eq21_e1140_d_b2, eq21_e1140_d_b3, eq21_e1140_d_b4, eq21_e1140_d_b5, eq21_e1140_d_b6, eq21_e1140_d_b7, eq21_e1140_d_b8, eq21_e1140_d_b9, eq21_e1140_d_b10, eq21_e1140_d_b11, eq21_e1140_d_b12, eq21_e1140_d_b13, eq21_e1140_d_b14, eq21_e1140_d_b15, eq21_e1140_d_b16, eq21_e1140_d_b17, eq21_e1140_d_b18, eq21_e1140_d_b19, eq21_e1140_d_b20, eq21_e1140_d_b21, eq21_e1140_d_b22, eq21_e1140_d_b23, eq21_e1140_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1142;
        let eq21_node_derivatives: [f64; 21] = [eq21_e1142_d_n0, eq21_e1142_d_n1, eq21_e1142_d_n2, eq21_e1142_d_n3, eq21_e1142_d_n4, eq21_e1142_d_n5, eq21_e1142_d_n6, eq21_e1142_d_n7, eq21_e1142_d_n8, eq21_e1142_d_n9, eq21_e1142_d_n10, eq21_e1142_d_n11, eq21_e1142_d_n12, eq21_e1142_d_n13, eq21_e1142_d_n14, eq21_e1142_d_n15, eq21_e1142_d_n16, eq21_e1142_d_n17, eq21_e1142_d_n18, eq21_e1142_d_n19, eq21_e1142_d_n20];
        let eq21_branch_derivatives: [f64; 25] = [eq21_e1142_d_b0, eq21_e1142_d_b1, eq21_e1142_d_b2, eq21_e1142_d_b3, eq21_e1142_d_b4, eq21_e1142_d_b5, eq21_e1142_d_b6, eq21_e1142_d_b7, eq21_e1142_d_b8, eq21_e1142_d_b9, eq21_e1142_d_b10, eq21_e1142_d_b11, eq21_e1142_d_b12, eq21_e1142_d_b13, eq21_e1142_d_b14, eq21_e1142_d_b15, eq21_e1142_d_b16, eq21_e1142_d_b17, eq21_e1142_d_b18, eq21_e1142_d_b19, eq21_e1142_d_b20, eq21_e1142_d_b21, eq21_e1142_d_b22, eq21_e1142_d_b23, eq21_e1142_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq21_value),
            &nodes,
            &eq21_node_derivatives,
            &branches,
            &eq21_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq22_e1152,) = {
    if (s.v[2916] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e1152;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq22_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq23_e1157,) = {
    if (!(s.v[2916] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1157;
        stamper.stamp_potential(
            branches[2],
            eq23_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_24_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq24_e1167, eq24_e1167_d_n0, eq24_e1167_d_n1, eq24_e1167_d_n2, eq24_e1167_d_n3, eq24_e1167_d_n4, eq24_e1167_d_n5, eq24_e1167_d_n6, eq24_e1167_d_n7, eq24_e1167_d_n8, eq24_e1167_d_n9, eq24_e1167_d_n10, eq24_e1167_d_n11, eq24_e1167_d_n12, eq24_e1167_d_n13, eq24_e1167_d_n14, eq24_e1167_d_n15, eq24_e1167_d_n16, eq24_e1167_d_n17, eq24_e1167_d_n18, eq24_e1167_d_n19, eq24_e1167_d_n20, eq24_e1167_d_b0, eq24_e1167_d_b1, eq24_e1167_d_b2, eq24_e1167_d_b3, eq24_e1167_d_b4, eq24_e1167_d_b5, eq24_e1167_d_b6, eq24_e1167_d_b7, eq24_e1167_d_b8, eq24_e1167_d_b9, eq24_e1167_d_b10, eq24_e1167_d_b11, eq24_e1167_d_b12, eq24_e1167_d_b13, eq24_e1167_d_b14, eq24_e1167_d_b15, eq24_e1167_d_b16, eq24_e1167_d_b17, eq24_e1167_d_b18, eq24_e1167_d_b19, eq24_e1167_d_b20, eq24_e1167_d_b21, eq24_e1167_d_b22, eq24_e1167_d_b23, eq24_e1167_d_b24,) = {
    if (s.v[2917] != 0.0) {
        let eq24_e1161: f64 = (s.v[19] * p.p32);
        let eq24_e1161_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq24_e1161_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq24_e1161_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq24_e1161_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq24_e1161_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq24_e1161_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq24_e1161_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq24_e1161_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq24_e1161_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq24_e1161_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq24_e1161_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq24_e1161_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq24_e1161_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq24_e1161_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq24_e1161_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq24_e1161_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq24_e1161_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq24_e1161_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq24_e1161_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq24_e1161_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq24_e1161_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq24_e1161_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq24_e1161_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq24_e1161_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq24_e1161_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq24_e1161_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq24_e1161_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq24_e1161_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq24_e1161_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq24_e1161_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq24_e1161_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq24_e1161_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq24_e1161_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq24_e1161_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq24_e1161_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq24_e1161_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq24_e1161_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq24_e1161_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq24_e1161_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq24_e1161_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq24_e1161_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq24_e1161_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq24_e1161_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq24_e1161_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq24_e1161_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq24_e1161_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq24_e1163: f64 = (eq24_e1161 * s.v[814]);
        let eq24_e1163_d_n0: f64 = ((eq24_e1161_d_n0 * s.v[814]) + (eq24_e1161 * s.dn[814][0]));
        let eq24_e1163_d_n1: f64 = ((eq24_e1161_d_n1 * s.v[814]) + (eq24_e1161 * s.dn[814][1]));
        let eq24_e1163_d_n2: f64 = ((eq24_e1161_d_n2 * s.v[814]) + (eq24_e1161 * s.dn[814][2]));
        let eq24_e1163_d_n3: f64 = ((eq24_e1161_d_n3 * s.v[814]) + (eq24_e1161 * s.dn[814][3]));
        let eq24_e1163_d_n4: f64 = ((eq24_e1161_d_n4 * s.v[814]) + (eq24_e1161 * s.dn[814][4]));
        let eq24_e1163_d_n5: f64 = ((eq24_e1161_d_n5 * s.v[814]) + (eq24_e1161 * s.dn[814][5]));
        let eq24_e1163_d_n6: f64 = ((eq24_e1161_d_n6 * s.v[814]) + (eq24_e1161 * s.dn[814][6]));
        let eq24_e1163_d_n7: f64 = ((eq24_e1161_d_n7 * s.v[814]) + (eq24_e1161 * s.dn[814][7]));
        let eq24_e1163_d_n8: f64 = ((eq24_e1161_d_n8 * s.v[814]) + (eq24_e1161 * s.dn[814][8]));
        let eq24_e1163_d_n9: f64 = ((eq24_e1161_d_n9 * s.v[814]) + (eq24_e1161 * s.dn[814][9]));
        let eq24_e1163_d_n10: f64 = ((eq24_e1161_d_n10 * s.v[814]) + (eq24_e1161 * s.dn[814][10]));
        let eq24_e1163_d_n11: f64 = ((eq24_e1161_d_n11 * s.v[814]) + (eq24_e1161 * s.dn[814][11]));
        let eq24_e1163_d_n12: f64 = ((eq24_e1161_d_n12 * s.v[814]) + (eq24_e1161 * s.dn[814][12]));
        let eq24_e1163_d_n13: f64 = ((eq24_e1161_d_n13 * s.v[814]) + (eq24_e1161 * s.dn[814][13]));
        let eq24_e1163_d_n14: f64 = ((eq24_e1161_d_n14 * s.v[814]) + (eq24_e1161 * s.dn[814][14]));
        let eq24_e1163_d_n15: f64 = ((eq24_e1161_d_n15 * s.v[814]) + (eq24_e1161 * s.dn[814][15]));
        let eq24_e1163_d_n16: f64 = ((eq24_e1161_d_n16 * s.v[814]) + (eq24_e1161 * s.dn[814][16]));
        let eq24_e1163_d_n17: f64 = ((eq24_e1161_d_n17 * s.v[814]) + (eq24_e1161 * s.dn[814][17]));
        let eq24_e1163_d_n18: f64 = ((eq24_e1161_d_n18 * s.v[814]) + (eq24_e1161 * s.dn[814][18]));
        let eq24_e1163_d_n19: f64 = ((eq24_e1161_d_n19 * s.v[814]) + (eq24_e1161 * s.dn[814][19]));
        let eq24_e1163_d_n20: f64 = ((eq24_e1161_d_n20 * s.v[814]) + (eq24_e1161 * s.dn[814][20]));
        let eq24_e1163_d_b0: f64 = ((eq24_e1161_d_b0 * s.v[814]) + (eq24_e1161 * s.db[814][0]));
        let eq24_e1163_d_b1: f64 = ((eq24_e1161_d_b1 * s.v[814]) + (eq24_e1161 * s.db[814][1]));
        let eq24_e1163_d_b2: f64 = ((eq24_e1161_d_b2 * s.v[814]) + (eq24_e1161 * s.db[814][2]));
        let eq24_e1163_d_b3: f64 = ((eq24_e1161_d_b3 * s.v[814]) + (eq24_e1161 * s.db[814][3]));
        let eq24_e1163_d_b4: f64 = ((eq24_e1161_d_b4 * s.v[814]) + (eq24_e1161 * s.db[814][4]));
        let eq24_e1163_d_b5: f64 = ((eq24_e1161_d_b5 * s.v[814]) + (eq24_e1161 * s.db[814][5]));
        let eq24_e1163_d_b6: f64 = ((eq24_e1161_d_b6 * s.v[814]) + (eq24_e1161 * s.db[814][6]));
        let eq24_e1163_d_b7: f64 = ((eq24_e1161_d_b7 * s.v[814]) + (eq24_e1161 * s.db[814][7]));
        let eq24_e1163_d_b8: f64 = ((eq24_e1161_d_b8 * s.v[814]) + (eq24_e1161 * s.db[814][8]));
        let eq24_e1163_d_b9: f64 = ((eq24_e1161_d_b9 * s.v[814]) + (eq24_e1161 * s.db[814][9]));
        let eq24_e1163_d_b10: f64 = ((eq24_e1161_d_b10 * s.v[814]) + (eq24_e1161 * s.db[814][10]));
        let eq24_e1163_d_b11: f64 = ((eq24_e1161_d_b11 * s.v[814]) + (eq24_e1161 * s.db[814][11]));
        let eq24_e1163_d_b12: f64 = ((eq24_e1161_d_b12 * s.v[814]) + (eq24_e1161 * s.db[814][12]));
        let eq24_e1163_d_b13: f64 = ((eq24_e1161_d_b13 * s.v[814]) + (eq24_e1161 * s.db[814][13]));
        let eq24_e1163_d_b14: f64 = ((eq24_e1161_d_b14 * s.v[814]) + (eq24_e1161 * s.db[814][14]));
        let eq24_e1163_d_b15: f64 = ((eq24_e1161_d_b15 * s.v[814]) + (eq24_e1161 * s.db[814][15]));
        let eq24_e1163_d_b16: f64 = ((eq24_e1161_d_b16 * s.v[814]) + (eq24_e1161 * s.db[814][16]));
        let eq24_e1163_d_b17: f64 = ((eq24_e1161_d_b17 * s.v[814]) + (eq24_e1161 * s.db[814][17]));
        let eq24_e1163_d_b18: f64 = ((eq24_e1161_d_b18 * s.v[814]) + (eq24_e1161 * s.db[814][18]));
        let eq24_e1163_d_b19: f64 = ((eq24_e1161_d_b19 * s.v[814]) + (eq24_e1161 * s.db[814][19]));
        let eq24_e1163_d_b20: f64 = ((eq24_e1161_d_b20 * s.v[814]) + (eq24_e1161 * s.db[814][20]));
        let eq24_e1163_d_b21: f64 = ((eq24_e1161_d_b21 * s.v[814]) + (eq24_e1161 * s.db[814][21]));
        let eq24_e1163_d_b22: f64 = ((eq24_e1161_d_b22 * s.v[814]) + (eq24_e1161 * s.db[814][22]));
        let eq24_e1163_d_b23: f64 = ((eq24_e1161_d_b23 * s.v[814]) + (eq24_e1161 * s.db[814][23]));
        let eq24_e1163_d_b24: f64 = ((eq24_e1161_d_b24 * s.v[814]) + (eq24_e1161 * s.db[814][24]));
        let eq24_e1165: f64 = (eq24_e1163 * (nv8 - nv9));
        let eq24_e1165_d_n0: f64 = (eq24_e1163_d_n0 * (nv8 - nv9));
        let eq24_e1165_d_n1: f64 = (eq24_e1163_d_n1 * (nv8 - nv9));
        let eq24_e1165_d_n2: f64 = (eq24_e1163_d_n2 * (nv8 - nv9));
        let eq24_e1165_d_n3: f64 = (eq24_e1163_d_n3 * (nv8 - nv9));
        let eq24_e1165_d_n4: f64 = (eq24_e1163_d_n4 * (nv8 - nv9));
        let eq24_e1165_d_n5: f64 = (eq24_e1163_d_n5 * (nv8 - nv9));
        let eq24_e1165_d_n6: f64 = (eq24_e1163_d_n6 * (nv8 - nv9));
        let eq24_e1165_d_n7: f64 = (eq24_e1163_d_n7 * (nv8 - nv9));
        let eq24_e1165_d_n8: f64 = ((eq24_e1163_d_n8 * (nv8 - nv9)) + eq24_e1163);
        let eq24_e1165_d_n9: f64 = ((eq24_e1163_d_n9 * (nv8 - nv9)) + (-eq24_e1163));
        let eq24_e1165_d_n10: f64 = (eq24_e1163_d_n10 * (nv8 - nv9));
        let eq24_e1165_d_n11: f64 = (eq24_e1163_d_n11 * (nv8 - nv9));
        let eq24_e1165_d_n12: f64 = (eq24_e1163_d_n12 * (nv8 - nv9));
        let eq24_e1165_d_n13: f64 = (eq24_e1163_d_n13 * (nv8 - nv9));
        let eq24_e1165_d_n14: f64 = (eq24_e1163_d_n14 * (nv8 - nv9));
        let eq24_e1165_d_n15: f64 = (eq24_e1163_d_n15 * (nv8 - nv9));
        let eq24_e1165_d_n16: f64 = (eq24_e1163_d_n16 * (nv8 - nv9));
        let eq24_e1165_d_n17: f64 = (eq24_e1163_d_n17 * (nv8 - nv9));
        let eq24_e1165_d_n18: f64 = (eq24_e1163_d_n18 * (nv8 - nv9));
        let eq24_e1165_d_n19: f64 = (eq24_e1163_d_n19 * (nv8 - nv9));
        let eq24_e1165_d_n20: f64 = (eq24_e1163_d_n20 * (nv8 - nv9));
        let eq24_e1165_d_b0: f64 = (eq24_e1163_d_b0 * (nv8 - nv9));
        let eq24_e1165_d_b1: f64 = (eq24_e1163_d_b1 * (nv8 - nv9));
        let eq24_e1165_d_b2: f64 = (eq24_e1163_d_b2 * (nv8 - nv9));
        let eq24_e1165_d_b3: f64 = (eq24_e1163_d_b3 * (nv8 - nv9));
        let eq24_e1165_d_b4: f64 = (eq24_e1163_d_b4 * (nv8 - nv9));
        let eq24_e1165_d_b5: f64 = (eq24_e1163_d_b5 * (nv8 - nv9));
        let eq24_e1165_d_b6: f64 = (eq24_e1163_d_b6 * (nv8 - nv9));
        let eq24_e1165_d_b7: f64 = (eq24_e1163_d_b7 * (nv8 - nv9));
        let eq24_e1165_d_b8: f64 = (eq24_e1163_d_b8 * (nv8 - nv9));
        let eq24_e1165_d_b9: f64 = (eq24_e1163_d_b9 * (nv8 - nv9));
        let eq24_e1165_d_b10: f64 = (eq24_e1163_d_b10 * (nv8 - nv9));
        let eq24_e1165_d_b11: f64 = (eq24_e1163_d_b11 * (nv8 - nv9));
        let eq24_e1165_d_b12: f64 = (eq24_e1163_d_b12 * (nv8 - nv9));
        let eq24_e1165_d_b13: f64 = (eq24_e1163_d_b13 * (nv8 - nv9));
        let eq24_e1165_d_b14: f64 = (eq24_e1163_d_b14 * (nv8 - nv9));
        let eq24_e1165_d_b15: f64 = (eq24_e1163_d_b15 * (nv8 - nv9));
        let eq24_e1165_d_b16: f64 = (eq24_e1163_d_b16 * (nv8 - nv9));
        let eq24_e1165_d_b17: f64 = (eq24_e1163_d_b17 * (nv8 - nv9));
        let eq24_e1165_d_b18: f64 = (eq24_e1163_d_b18 * (nv8 - nv9));
        let eq24_e1165_d_b19: f64 = (eq24_e1163_d_b19 * (nv8 - nv9));
        let eq24_e1165_d_b20: f64 = (eq24_e1163_d_b20 * (nv8 - nv9));
        let eq24_e1165_d_b21: f64 = (eq24_e1163_d_b21 * (nv8 - nv9));
        let eq24_e1165_d_b22: f64 = (eq24_e1163_d_b22 * (nv8 - nv9));
        let eq24_e1165_d_b23: f64 = (eq24_e1163_d_b23 * (nv8 - nv9));
        let eq24_e1165_d_b24: f64 = (eq24_e1163_d_b24 * (nv8 - nv9));
        (eq24_e1165, eq24_e1165_d_n0, eq24_e1165_d_n1, eq24_e1165_d_n2, eq24_e1165_d_n3, eq24_e1165_d_n4, eq24_e1165_d_n5, eq24_e1165_d_n6, eq24_e1165_d_n7, eq24_e1165_d_n8, eq24_e1165_d_n9, eq24_e1165_d_n10, eq24_e1165_d_n11, eq24_e1165_d_n12, eq24_e1165_d_n13, eq24_e1165_d_n14, eq24_e1165_d_n15, eq24_e1165_d_n16, eq24_e1165_d_n17, eq24_e1165_d_n18, eq24_e1165_d_n19, eq24_e1165_d_n20, eq24_e1165_d_b0, eq24_e1165_d_b1, eq24_e1165_d_b2, eq24_e1165_d_b3, eq24_e1165_d_b4, eq24_e1165_d_b5, eq24_e1165_d_b6, eq24_e1165_d_b7, eq24_e1165_d_b8, eq24_e1165_d_b9, eq24_e1165_d_b10, eq24_e1165_d_b11, eq24_e1165_d_b12, eq24_e1165_d_b13, eq24_e1165_d_b14, eq24_e1165_d_b15, eq24_e1165_d_b16, eq24_e1165_d_b17, eq24_e1165_d_b18, eq24_e1165_d_b19, eq24_e1165_d_b20, eq24_e1165_d_b21, eq24_e1165_d_b22, eq24_e1165_d_b23, eq24_e1165_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1167;
        let eq24_node_derivatives: [f64; 21] = [eq24_e1167_d_n0, eq24_e1167_d_n1, eq24_e1167_d_n2, eq24_e1167_d_n3, eq24_e1167_d_n4, eq24_e1167_d_n5, eq24_e1167_d_n6, eq24_e1167_d_n7, eq24_e1167_d_n8, eq24_e1167_d_n9, eq24_e1167_d_n10, eq24_e1167_d_n11, eq24_e1167_d_n12, eq24_e1167_d_n13, eq24_e1167_d_n14, eq24_e1167_d_n15, eq24_e1167_d_n16, eq24_e1167_d_n17, eq24_e1167_d_n18, eq24_e1167_d_n19, eq24_e1167_d_n20];
        let eq24_branch_derivatives: [f64; 25] = [eq24_e1167_d_b0, eq24_e1167_d_b1, eq24_e1167_d_b2, eq24_e1167_d_b3, eq24_e1167_d_b4, eq24_e1167_d_b5, eq24_e1167_d_b6, eq24_e1167_d_b7, eq24_e1167_d_b8, eq24_e1167_d_b9, eq24_e1167_d_b10, eq24_e1167_d_b11, eq24_e1167_d_b12, eq24_e1167_d_b13, eq24_e1167_d_b14, eq24_e1167_d_b15, eq24_e1167_d_b16, eq24_e1167_d_b17, eq24_e1167_d_b18, eq24_e1167_d_b19, eq24_e1167_d_b20, eq24_e1167_d_b21, eq24_e1167_d_b22, eq24_e1167_d_b23, eq24_e1167_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            self.multiplicity * (eq24_value),
            &nodes,
            &eq24_node_derivatives,
            &branches,
            &eq24_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_25_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq25_e1177,) = {
    if (s.v[2917] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e1177;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[9]),
            self.multiplicity * (eq25_value),
            &[
            ],
        );
    }

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
        let (eq26_e1182,) = {
    if (!(s.v[2917] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1182;
        stamper.stamp_potential(
            branches[3],
            eq26_value,
            &[
            ],
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq27_e1192, eq27_e1192_d_n0, eq27_e1192_d_n1, eq27_e1192_d_n2, eq27_e1192_d_n3, eq27_e1192_d_n4, eq27_e1192_d_n5, eq27_e1192_d_n6, eq27_e1192_d_n7, eq27_e1192_d_n8, eq27_e1192_d_n9, eq27_e1192_d_n10, eq27_e1192_d_n11, eq27_e1192_d_n12, eq27_e1192_d_n13, eq27_e1192_d_n14, eq27_e1192_d_n15, eq27_e1192_d_n16, eq27_e1192_d_n17, eq27_e1192_d_n18, eq27_e1192_d_n19, eq27_e1192_d_n20, eq27_e1192_d_b0, eq27_e1192_d_b1, eq27_e1192_d_b2, eq27_e1192_d_b3, eq27_e1192_d_b4, eq27_e1192_d_b5, eq27_e1192_d_b6, eq27_e1192_d_b7, eq27_e1192_d_b8, eq27_e1192_d_b9, eq27_e1192_d_b10, eq27_e1192_d_b11, eq27_e1192_d_b12, eq27_e1192_d_b13, eq27_e1192_d_b14, eq27_e1192_d_b15, eq27_e1192_d_b16, eq27_e1192_d_b17, eq27_e1192_d_b18, eq27_e1192_d_b19, eq27_e1192_d_b20, eq27_e1192_d_b21, eq27_e1192_d_b22, eq27_e1192_d_b23, eq27_e1192_d_b24,) = {
    if (s.v[2918] != 0.0) {
        let eq27_e1186: f64 = (s.v[19] * p.p32);
        let eq27_e1186_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq27_e1186_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq27_e1186_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq27_e1186_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq27_e1186_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq27_e1186_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq27_e1186_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq27_e1186_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq27_e1186_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq27_e1186_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq27_e1186_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq27_e1186_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq27_e1186_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq27_e1186_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq27_e1186_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq27_e1186_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq27_e1186_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq27_e1186_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq27_e1186_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq27_e1186_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq27_e1186_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq27_e1186_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq27_e1186_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq27_e1186_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq27_e1186_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq27_e1186_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq27_e1186_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq27_e1186_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq27_e1186_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq27_e1186_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq27_e1186_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq27_e1186_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq27_e1186_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq27_e1186_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq27_e1186_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq27_e1186_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq27_e1186_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq27_e1186_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq27_e1186_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq27_e1186_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq27_e1186_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq27_e1186_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq27_e1186_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq27_e1186_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq27_e1186_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq27_e1186_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq27_e1188: f64 = (eq27_e1186 * s.v[815]);
        let eq27_e1188_d_n0: f64 = ((eq27_e1186_d_n0 * s.v[815]) + (eq27_e1186 * s.dn[815][0]));
        let eq27_e1188_d_n1: f64 = ((eq27_e1186_d_n1 * s.v[815]) + (eq27_e1186 * s.dn[815][1]));
        let eq27_e1188_d_n2: f64 = ((eq27_e1186_d_n2 * s.v[815]) + (eq27_e1186 * s.dn[815][2]));
        let eq27_e1188_d_n3: f64 = ((eq27_e1186_d_n3 * s.v[815]) + (eq27_e1186 * s.dn[815][3]));
        let eq27_e1188_d_n4: f64 = ((eq27_e1186_d_n4 * s.v[815]) + (eq27_e1186 * s.dn[815][4]));
        let eq27_e1188_d_n5: f64 = ((eq27_e1186_d_n5 * s.v[815]) + (eq27_e1186 * s.dn[815][5]));
        let eq27_e1188_d_n6: f64 = ((eq27_e1186_d_n6 * s.v[815]) + (eq27_e1186 * s.dn[815][6]));
        let eq27_e1188_d_n7: f64 = ((eq27_e1186_d_n7 * s.v[815]) + (eq27_e1186 * s.dn[815][7]));
        let eq27_e1188_d_n8: f64 = ((eq27_e1186_d_n8 * s.v[815]) + (eq27_e1186 * s.dn[815][8]));
        let eq27_e1188_d_n9: f64 = ((eq27_e1186_d_n9 * s.v[815]) + (eq27_e1186 * s.dn[815][9]));
        let eq27_e1188_d_n10: f64 = ((eq27_e1186_d_n10 * s.v[815]) + (eq27_e1186 * s.dn[815][10]));
        let eq27_e1188_d_n11: f64 = ((eq27_e1186_d_n11 * s.v[815]) + (eq27_e1186 * s.dn[815][11]));
        let eq27_e1188_d_n12: f64 = ((eq27_e1186_d_n12 * s.v[815]) + (eq27_e1186 * s.dn[815][12]));
        let eq27_e1188_d_n13: f64 = ((eq27_e1186_d_n13 * s.v[815]) + (eq27_e1186 * s.dn[815][13]));
        let eq27_e1188_d_n14: f64 = ((eq27_e1186_d_n14 * s.v[815]) + (eq27_e1186 * s.dn[815][14]));
        let eq27_e1188_d_n15: f64 = ((eq27_e1186_d_n15 * s.v[815]) + (eq27_e1186 * s.dn[815][15]));
        let eq27_e1188_d_n16: f64 = ((eq27_e1186_d_n16 * s.v[815]) + (eq27_e1186 * s.dn[815][16]));
        let eq27_e1188_d_n17: f64 = ((eq27_e1186_d_n17 * s.v[815]) + (eq27_e1186 * s.dn[815][17]));
        let eq27_e1188_d_n18: f64 = ((eq27_e1186_d_n18 * s.v[815]) + (eq27_e1186 * s.dn[815][18]));
        let eq27_e1188_d_n19: f64 = ((eq27_e1186_d_n19 * s.v[815]) + (eq27_e1186 * s.dn[815][19]));
        let eq27_e1188_d_n20: f64 = ((eq27_e1186_d_n20 * s.v[815]) + (eq27_e1186 * s.dn[815][20]));
        let eq27_e1188_d_b0: f64 = ((eq27_e1186_d_b0 * s.v[815]) + (eq27_e1186 * s.db[815][0]));
        let eq27_e1188_d_b1: f64 = ((eq27_e1186_d_b1 * s.v[815]) + (eq27_e1186 * s.db[815][1]));
        let eq27_e1188_d_b2: f64 = ((eq27_e1186_d_b2 * s.v[815]) + (eq27_e1186 * s.db[815][2]));
        let eq27_e1188_d_b3: f64 = ((eq27_e1186_d_b3 * s.v[815]) + (eq27_e1186 * s.db[815][3]));
        let eq27_e1188_d_b4: f64 = ((eq27_e1186_d_b4 * s.v[815]) + (eq27_e1186 * s.db[815][4]));
        let eq27_e1188_d_b5: f64 = ((eq27_e1186_d_b5 * s.v[815]) + (eq27_e1186 * s.db[815][5]));
        let eq27_e1188_d_b6: f64 = ((eq27_e1186_d_b6 * s.v[815]) + (eq27_e1186 * s.db[815][6]));
        let eq27_e1188_d_b7: f64 = ((eq27_e1186_d_b7 * s.v[815]) + (eq27_e1186 * s.db[815][7]));
        let eq27_e1188_d_b8: f64 = ((eq27_e1186_d_b8 * s.v[815]) + (eq27_e1186 * s.db[815][8]));
        let eq27_e1188_d_b9: f64 = ((eq27_e1186_d_b9 * s.v[815]) + (eq27_e1186 * s.db[815][9]));
        let eq27_e1188_d_b10: f64 = ((eq27_e1186_d_b10 * s.v[815]) + (eq27_e1186 * s.db[815][10]));
        let eq27_e1188_d_b11: f64 = ((eq27_e1186_d_b11 * s.v[815]) + (eq27_e1186 * s.db[815][11]));
        let eq27_e1188_d_b12: f64 = ((eq27_e1186_d_b12 * s.v[815]) + (eq27_e1186 * s.db[815][12]));
        let eq27_e1188_d_b13: f64 = ((eq27_e1186_d_b13 * s.v[815]) + (eq27_e1186 * s.db[815][13]));
        let eq27_e1188_d_b14: f64 = ((eq27_e1186_d_b14 * s.v[815]) + (eq27_e1186 * s.db[815][14]));
        let eq27_e1188_d_b15: f64 = ((eq27_e1186_d_b15 * s.v[815]) + (eq27_e1186 * s.db[815][15]));
        let eq27_e1188_d_b16: f64 = ((eq27_e1186_d_b16 * s.v[815]) + (eq27_e1186 * s.db[815][16]));
        let eq27_e1188_d_b17: f64 = ((eq27_e1186_d_b17 * s.v[815]) + (eq27_e1186 * s.db[815][17]));
        let eq27_e1188_d_b18: f64 = ((eq27_e1186_d_b18 * s.v[815]) + (eq27_e1186 * s.db[815][18]));
        let eq27_e1188_d_b19: f64 = ((eq27_e1186_d_b19 * s.v[815]) + (eq27_e1186 * s.db[815][19]));
        let eq27_e1188_d_b20: f64 = ((eq27_e1186_d_b20 * s.v[815]) + (eq27_e1186 * s.db[815][20]));
        let eq27_e1188_d_b21: f64 = ((eq27_e1186_d_b21 * s.v[815]) + (eq27_e1186 * s.db[815][21]));
        let eq27_e1188_d_b22: f64 = ((eq27_e1186_d_b22 * s.v[815]) + (eq27_e1186 * s.db[815][22]));
        let eq27_e1188_d_b23: f64 = ((eq27_e1186_d_b23 * s.v[815]) + (eq27_e1186 * s.db[815][23]));
        let eq27_e1188_d_b24: f64 = ((eq27_e1186_d_b24 * s.v[815]) + (eq27_e1186 * s.db[815][24]));
        let eq27_e1190: f64 = (eq27_e1188 * (nv10 - nv9));
        let eq27_e1190_d_n0: f64 = (eq27_e1188_d_n0 * (nv10 - nv9));
        let eq27_e1190_d_n1: f64 = (eq27_e1188_d_n1 * (nv10 - nv9));
        let eq27_e1190_d_n2: f64 = (eq27_e1188_d_n2 * (nv10 - nv9));
        let eq27_e1190_d_n3: f64 = (eq27_e1188_d_n3 * (nv10 - nv9));
        let eq27_e1190_d_n4: f64 = (eq27_e1188_d_n4 * (nv10 - nv9));
        let eq27_e1190_d_n5: f64 = (eq27_e1188_d_n5 * (nv10 - nv9));
        let eq27_e1190_d_n6: f64 = (eq27_e1188_d_n6 * (nv10 - nv9));
        let eq27_e1190_d_n7: f64 = (eq27_e1188_d_n7 * (nv10 - nv9));
        let eq27_e1190_d_n8: f64 = (eq27_e1188_d_n8 * (nv10 - nv9));
        let eq27_e1190_d_n9: f64 = ((eq27_e1188_d_n9 * (nv10 - nv9)) + (-eq27_e1188));
        let eq27_e1190_d_n10: f64 = ((eq27_e1188_d_n10 * (nv10 - nv9)) + eq27_e1188);
        let eq27_e1190_d_n11: f64 = (eq27_e1188_d_n11 * (nv10 - nv9));
        let eq27_e1190_d_n12: f64 = (eq27_e1188_d_n12 * (nv10 - nv9));
        let eq27_e1190_d_n13: f64 = (eq27_e1188_d_n13 * (nv10 - nv9));
        let eq27_e1190_d_n14: f64 = (eq27_e1188_d_n14 * (nv10 - nv9));
        let eq27_e1190_d_n15: f64 = (eq27_e1188_d_n15 * (nv10 - nv9));
        let eq27_e1190_d_n16: f64 = (eq27_e1188_d_n16 * (nv10 - nv9));
        let eq27_e1190_d_n17: f64 = (eq27_e1188_d_n17 * (nv10 - nv9));
        let eq27_e1190_d_n18: f64 = (eq27_e1188_d_n18 * (nv10 - nv9));
        let eq27_e1190_d_n19: f64 = (eq27_e1188_d_n19 * (nv10 - nv9));
        let eq27_e1190_d_n20: f64 = (eq27_e1188_d_n20 * (nv10 - nv9));
        let eq27_e1190_d_b0: f64 = (eq27_e1188_d_b0 * (nv10 - nv9));
        let eq27_e1190_d_b1: f64 = (eq27_e1188_d_b1 * (nv10 - nv9));
        let eq27_e1190_d_b2: f64 = (eq27_e1188_d_b2 * (nv10 - nv9));
        let eq27_e1190_d_b3: f64 = (eq27_e1188_d_b3 * (nv10 - nv9));
        let eq27_e1190_d_b4: f64 = (eq27_e1188_d_b4 * (nv10 - nv9));
        let eq27_e1190_d_b5: f64 = (eq27_e1188_d_b5 * (nv10 - nv9));
        let eq27_e1190_d_b6: f64 = (eq27_e1188_d_b6 * (nv10 - nv9));
        let eq27_e1190_d_b7: f64 = (eq27_e1188_d_b7 * (nv10 - nv9));
        let eq27_e1190_d_b8: f64 = (eq27_e1188_d_b8 * (nv10 - nv9));
        let eq27_e1190_d_b9: f64 = (eq27_e1188_d_b9 * (nv10 - nv9));
        let eq27_e1190_d_b10: f64 = (eq27_e1188_d_b10 * (nv10 - nv9));
        let eq27_e1190_d_b11: f64 = (eq27_e1188_d_b11 * (nv10 - nv9));
        let eq27_e1190_d_b12: f64 = (eq27_e1188_d_b12 * (nv10 - nv9));
        let eq27_e1190_d_b13: f64 = (eq27_e1188_d_b13 * (nv10 - nv9));
        let eq27_e1190_d_b14: f64 = (eq27_e1188_d_b14 * (nv10 - nv9));
        let eq27_e1190_d_b15: f64 = (eq27_e1188_d_b15 * (nv10 - nv9));
        let eq27_e1190_d_b16: f64 = (eq27_e1188_d_b16 * (nv10 - nv9));
        let eq27_e1190_d_b17: f64 = (eq27_e1188_d_b17 * (nv10 - nv9));
        let eq27_e1190_d_b18: f64 = (eq27_e1188_d_b18 * (nv10 - nv9));
        let eq27_e1190_d_b19: f64 = (eq27_e1188_d_b19 * (nv10 - nv9));
        let eq27_e1190_d_b20: f64 = (eq27_e1188_d_b20 * (nv10 - nv9));
        let eq27_e1190_d_b21: f64 = (eq27_e1188_d_b21 * (nv10 - nv9));
        let eq27_e1190_d_b22: f64 = (eq27_e1188_d_b22 * (nv10 - nv9));
        let eq27_e1190_d_b23: f64 = (eq27_e1188_d_b23 * (nv10 - nv9));
        let eq27_e1190_d_b24: f64 = (eq27_e1188_d_b24 * (nv10 - nv9));
        (eq27_e1190, eq27_e1190_d_n0, eq27_e1190_d_n1, eq27_e1190_d_n2, eq27_e1190_d_n3, eq27_e1190_d_n4, eq27_e1190_d_n5, eq27_e1190_d_n6, eq27_e1190_d_n7, eq27_e1190_d_n8, eq27_e1190_d_n9, eq27_e1190_d_n10, eq27_e1190_d_n11, eq27_e1190_d_n12, eq27_e1190_d_n13, eq27_e1190_d_n14, eq27_e1190_d_n15, eq27_e1190_d_n16, eq27_e1190_d_n17, eq27_e1190_d_n18, eq27_e1190_d_n19, eq27_e1190_d_n20, eq27_e1190_d_b0, eq27_e1190_d_b1, eq27_e1190_d_b2, eq27_e1190_d_b3, eq27_e1190_d_b4, eq27_e1190_d_b5, eq27_e1190_d_b6, eq27_e1190_d_b7, eq27_e1190_d_b8, eq27_e1190_d_b9, eq27_e1190_d_b10, eq27_e1190_d_b11, eq27_e1190_d_b12, eq27_e1190_d_b13, eq27_e1190_d_b14, eq27_e1190_d_b15, eq27_e1190_d_b16, eq27_e1190_d_b17, eq27_e1190_d_b18, eq27_e1190_d_b19, eq27_e1190_d_b20, eq27_e1190_d_b21, eq27_e1190_d_b22, eq27_e1190_d_b23, eq27_e1190_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1192;
        let eq27_node_derivatives: [f64; 21] = [eq27_e1192_d_n0, eq27_e1192_d_n1, eq27_e1192_d_n2, eq27_e1192_d_n3, eq27_e1192_d_n4, eq27_e1192_d_n5, eq27_e1192_d_n6, eq27_e1192_d_n7, eq27_e1192_d_n8, eq27_e1192_d_n9, eq27_e1192_d_n10, eq27_e1192_d_n11, eq27_e1192_d_n12, eq27_e1192_d_n13, eq27_e1192_d_n14, eq27_e1192_d_n15, eq27_e1192_d_n16, eq27_e1192_d_n17, eq27_e1192_d_n18, eq27_e1192_d_n19, eq27_e1192_d_n20];
        let eq27_branch_derivatives: [f64; 25] = [eq27_e1192_d_b0, eq27_e1192_d_b1, eq27_e1192_d_b2, eq27_e1192_d_b3, eq27_e1192_d_b4, eq27_e1192_d_b5, eq27_e1192_d_b6, eq27_e1192_d_b7, eq27_e1192_d_b8, eq27_e1192_d_b9, eq27_e1192_d_b10, eq27_e1192_d_b11, eq27_e1192_d_b12, eq27_e1192_d_b13, eq27_e1192_d_b14, eq27_e1192_d_b15, eq27_e1192_d_b16, eq27_e1192_d_b17, eq27_e1192_d_b18, eq27_e1192_d_b19, eq27_e1192_d_b20, eq27_e1192_d_b21, eq27_e1192_d_b22, eq27_e1192_d_b23, eq27_e1192_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[9]),
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
        let (eq28_e1202,) = {
    if (s.v[2918] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e1202;
        stamper.stamp_current(
            Some(nodes[10]),
            Some(nodes[9]),
            self.multiplicity * (eq28_value),
            &[
            ],
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
        let (eq29_e1207,) = {
    if (!(s.v[2918] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1207;
        stamper.stamp_potential(
            branches[4],
            eq29_value,
            &[
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq30_e1217, eq30_e1217_d_n0, eq30_e1217_d_n1, eq30_e1217_d_n2, eq30_e1217_d_n3, eq30_e1217_d_n4, eq30_e1217_d_n5, eq30_e1217_d_n6, eq30_e1217_d_n7, eq30_e1217_d_n8, eq30_e1217_d_n9, eq30_e1217_d_n10, eq30_e1217_d_n11, eq30_e1217_d_n12, eq30_e1217_d_n13, eq30_e1217_d_n14, eq30_e1217_d_n15, eq30_e1217_d_n16, eq30_e1217_d_n17, eq30_e1217_d_n18, eq30_e1217_d_n19, eq30_e1217_d_n20, eq30_e1217_d_b0, eq30_e1217_d_b1, eq30_e1217_d_b2, eq30_e1217_d_b3, eq30_e1217_d_b4, eq30_e1217_d_b5, eq30_e1217_d_b6, eq30_e1217_d_b7, eq30_e1217_d_b8, eq30_e1217_d_b9, eq30_e1217_d_b10, eq30_e1217_d_b11, eq30_e1217_d_b12, eq30_e1217_d_b13, eq30_e1217_d_b14, eq30_e1217_d_b15, eq30_e1217_d_b16, eq30_e1217_d_b17, eq30_e1217_d_b18, eq30_e1217_d_b19, eq30_e1217_d_b20, eq30_e1217_d_b21, eq30_e1217_d_b22, eq30_e1217_d_b23, eq30_e1217_d_b24,) = {
    if (s.v[2919] != 0.0) {
        let eq30_e1211: f64 = (s.v[19] * p.p32);
        let eq30_e1211_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq30_e1211_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq30_e1211_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq30_e1211_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq30_e1211_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq30_e1211_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq30_e1211_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq30_e1211_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq30_e1211_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq30_e1211_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq30_e1211_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq30_e1211_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq30_e1211_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq30_e1211_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq30_e1211_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq30_e1211_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq30_e1211_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq30_e1211_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq30_e1211_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq30_e1211_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq30_e1211_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq30_e1211_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq30_e1211_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq30_e1211_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq30_e1211_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq30_e1211_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq30_e1211_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq30_e1211_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq30_e1211_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq30_e1211_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq30_e1211_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq30_e1211_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq30_e1211_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq30_e1211_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq30_e1211_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq30_e1211_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq30_e1211_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq30_e1211_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq30_e1211_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq30_e1211_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq30_e1211_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq30_e1211_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq30_e1211_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq30_e1211_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq30_e1211_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq30_e1211_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq30_e1213: f64 = (eq30_e1211 * s.v[816]);
        let eq30_e1213_d_n0: f64 = ((eq30_e1211_d_n0 * s.v[816]) + (eq30_e1211 * s.dn[816][0]));
        let eq30_e1213_d_n1: f64 = ((eq30_e1211_d_n1 * s.v[816]) + (eq30_e1211 * s.dn[816][1]));
        let eq30_e1213_d_n2: f64 = ((eq30_e1211_d_n2 * s.v[816]) + (eq30_e1211 * s.dn[816][2]));
        let eq30_e1213_d_n3: f64 = ((eq30_e1211_d_n3 * s.v[816]) + (eq30_e1211 * s.dn[816][3]));
        let eq30_e1213_d_n4: f64 = ((eq30_e1211_d_n4 * s.v[816]) + (eq30_e1211 * s.dn[816][4]));
        let eq30_e1213_d_n5: f64 = ((eq30_e1211_d_n5 * s.v[816]) + (eq30_e1211 * s.dn[816][5]));
        let eq30_e1213_d_n6: f64 = ((eq30_e1211_d_n6 * s.v[816]) + (eq30_e1211 * s.dn[816][6]));
        let eq30_e1213_d_n7: f64 = ((eq30_e1211_d_n7 * s.v[816]) + (eq30_e1211 * s.dn[816][7]));
        let eq30_e1213_d_n8: f64 = ((eq30_e1211_d_n8 * s.v[816]) + (eq30_e1211 * s.dn[816][8]));
        let eq30_e1213_d_n9: f64 = ((eq30_e1211_d_n9 * s.v[816]) + (eq30_e1211 * s.dn[816][9]));
        let eq30_e1213_d_n10: f64 = ((eq30_e1211_d_n10 * s.v[816]) + (eq30_e1211 * s.dn[816][10]));
        let eq30_e1213_d_n11: f64 = ((eq30_e1211_d_n11 * s.v[816]) + (eq30_e1211 * s.dn[816][11]));
        let eq30_e1213_d_n12: f64 = ((eq30_e1211_d_n12 * s.v[816]) + (eq30_e1211 * s.dn[816][12]));
        let eq30_e1213_d_n13: f64 = ((eq30_e1211_d_n13 * s.v[816]) + (eq30_e1211 * s.dn[816][13]));
        let eq30_e1213_d_n14: f64 = ((eq30_e1211_d_n14 * s.v[816]) + (eq30_e1211 * s.dn[816][14]));
        let eq30_e1213_d_n15: f64 = ((eq30_e1211_d_n15 * s.v[816]) + (eq30_e1211 * s.dn[816][15]));
        let eq30_e1213_d_n16: f64 = ((eq30_e1211_d_n16 * s.v[816]) + (eq30_e1211 * s.dn[816][16]));
        let eq30_e1213_d_n17: f64 = ((eq30_e1211_d_n17 * s.v[816]) + (eq30_e1211 * s.dn[816][17]));
        let eq30_e1213_d_n18: f64 = ((eq30_e1211_d_n18 * s.v[816]) + (eq30_e1211 * s.dn[816][18]));
        let eq30_e1213_d_n19: f64 = ((eq30_e1211_d_n19 * s.v[816]) + (eq30_e1211 * s.dn[816][19]));
        let eq30_e1213_d_n20: f64 = ((eq30_e1211_d_n20 * s.v[816]) + (eq30_e1211 * s.dn[816][20]));
        let eq30_e1213_d_b0: f64 = ((eq30_e1211_d_b0 * s.v[816]) + (eq30_e1211 * s.db[816][0]));
        let eq30_e1213_d_b1: f64 = ((eq30_e1211_d_b1 * s.v[816]) + (eq30_e1211 * s.db[816][1]));
        let eq30_e1213_d_b2: f64 = ((eq30_e1211_d_b2 * s.v[816]) + (eq30_e1211 * s.db[816][2]));
        let eq30_e1213_d_b3: f64 = ((eq30_e1211_d_b3 * s.v[816]) + (eq30_e1211 * s.db[816][3]));
        let eq30_e1213_d_b4: f64 = ((eq30_e1211_d_b4 * s.v[816]) + (eq30_e1211 * s.db[816][4]));
        let eq30_e1213_d_b5: f64 = ((eq30_e1211_d_b5 * s.v[816]) + (eq30_e1211 * s.db[816][5]));
        let eq30_e1213_d_b6: f64 = ((eq30_e1211_d_b6 * s.v[816]) + (eq30_e1211 * s.db[816][6]));
        let eq30_e1213_d_b7: f64 = ((eq30_e1211_d_b7 * s.v[816]) + (eq30_e1211 * s.db[816][7]));
        let eq30_e1213_d_b8: f64 = ((eq30_e1211_d_b8 * s.v[816]) + (eq30_e1211 * s.db[816][8]));
        let eq30_e1213_d_b9: f64 = ((eq30_e1211_d_b9 * s.v[816]) + (eq30_e1211 * s.db[816][9]));
        let eq30_e1213_d_b10: f64 = ((eq30_e1211_d_b10 * s.v[816]) + (eq30_e1211 * s.db[816][10]));
        let eq30_e1213_d_b11: f64 = ((eq30_e1211_d_b11 * s.v[816]) + (eq30_e1211 * s.db[816][11]));
        let eq30_e1213_d_b12: f64 = ((eq30_e1211_d_b12 * s.v[816]) + (eq30_e1211 * s.db[816][12]));
        let eq30_e1213_d_b13: f64 = ((eq30_e1211_d_b13 * s.v[816]) + (eq30_e1211 * s.db[816][13]));
        let eq30_e1213_d_b14: f64 = ((eq30_e1211_d_b14 * s.v[816]) + (eq30_e1211 * s.db[816][14]));
        let eq30_e1213_d_b15: f64 = ((eq30_e1211_d_b15 * s.v[816]) + (eq30_e1211 * s.db[816][15]));
        let eq30_e1213_d_b16: f64 = ((eq30_e1211_d_b16 * s.v[816]) + (eq30_e1211 * s.db[816][16]));
        let eq30_e1213_d_b17: f64 = ((eq30_e1211_d_b17 * s.v[816]) + (eq30_e1211 * s.db[816][17]));
        let eq30_e1213_d_b18: f64 = ((eq30_e1211_d_b18 * s.v[816]) + (eq30_e1211 * s.db[816][18]));
        let eq30_e1213_d_b19: f64 = ((eq30_e1211_d_b19 * s.v[816]) + (eq30_e1211 * s.db[816][19]));
        let eq30_e1213_d_b20: f64 = ((eq30_e1211_d_b20 * s.v[816]) + (eq30_e1211 * s.db[816][20]));
        let eq30_e1213_d_b21: f64 = ((eq30_e1211_d_b21 * s.v[816]) + (eq30_e1211 * s.db[816][21]));
        let eq30_e1213_d_b22: f64 = ((eq30_e1211_d_b22 * s.v[816]) + (eq30_e1211 * s.db[816][22]));
        let eq30_e1213_d_b23: f64 = ((eq30_e1211_d_b23 * s.v[816]) + (eq30_e1211 * s.db[816][23]));
        let eq30_e1213_d_b24: f64 = ((eq30_e1211_d_b24 * s.v[816]) + (eq30_e1211 * s.db[816][24]));
        let eq30_e1215: f64 = (eq30_e1213 * (nv11 - nv9));
        let eq30_e1215_d_n0: f64 = (eq30_e1213_d_n0 * (nv11 - nv9));
        let eq30_e1215_d_n1: f64 = (eq30_e1213_d_n1 * (nv11 - nv9));
        let eq30_e1215_d_n2: f64 = (eq30_e1213_d_n2 * (nv11 - nv9));
        let eq30_e1215_d_n3: f64 = (eq30_e1213_d_n3 * (nv11 - nv9));
        let eq30_e1215_d_n4: f64 = (eq30_e1213_d_n4 * (nv11 - nv9));
        let eq30_e1215_d_n5: f64 = (eq30_e1213_d_n5 * (nv11 - nv9));
        let eq30_e1215_d_n6: f64 = (eq30_e1213_d_n6 * (nv11 - nv9));
        let eq30_e1215_d_n7: f64 = (eq30_e1213_d_n7 * (nv11 - nv9));
        let eq30_e1215_d_n8: f64 = (eq30_e1213_d_n8 * (nv11 - nv9));
        let eq30_e1215_d_n9: f64 = ((eq30_e1213_d_n9 * (nv11 - nv9)) + (-eq30_e1213));
        let eq30_e1215_d_n10: f64 = (eq30_e1213_d_n10 * (nv11 - nv9));
        let eq30_e1215_d_n11: f64 = ((eq30_e1213_d_n11 * (nv11 - nv9)) + eq30_e1213);
        let eq30_e1215_d_n12: f64 = (eq30_e1213_d_n12 * (nv11 - nv9));
        let eq30_e1215_d_n13: f64 = (eq30_e1213_d_n13 * (nv11 - nv9));
        let eq30_e1215_d_n14: f64 = (eq30_e1213_d_n14 * (nv11 - nv9));
        let eq30_e1215_d_n15: f64 = (eq30_e1213_d_n15 * (nv11 - nv9));
        let eq30_e1215_d_n16: f64 = (eq30_e1213_d_n16 * (nv11 - nv9));
        let eq30_e1215_d_n17: f64 = (eq30_e1213_d_n17 * (nv11 - nv9));
        let eq30_e1215_d_n18: f64 = (eq30_e1213_d_n18 * (nv11 - nv9));
        let eq30_e1215_d_n19: f64 = (eq30_e1213_d_n19 * (nv11 - nv9));
        let eq30_e1215_d_n20: f64 = (eq30_e1213_d_n20 * (nv11 - nv9));
        let eq30_e1215_d_b0: f64 = (eq30_e1213_d_b0 * (nv11 - nv9));
        let eq30_e1215_d_b1: f64 = (eq30_e1213_d_b1 * (nv11 - nv9));
        let eq30_e1215_d_b2: f64 = (eq30_e1213_d_b2 * (nv11 - nv9));
        let eq30_e1215_d_b3: f64 = (eq30_e1213_d_b3 * (nv11 - nv9));
        let eq30_e1215_d_b4: f64 = (eq30_e1213_d_b4 * (nv11 - nv9));
        let eq30_e1215_d_b5: f64 = (eq30_e1213_d_b5 * (nv11 - nv9));
        let eq30_e1215_d_b6: f64 = (eq30_e1213_d_b6 * (nv11 - nv9));
        let eq30_e1215_d_b7: f64 = (eq30_e1213_d_b7 * (nv11 - nv9));
        let eq30_e1215_d_b8: f64 = (eq30_e1213_d_b8 * (nv11 - nv9));
        let eq30_e1215_d_b9: f64 = (eq30_e1213_d_b9 * (nv11 - nv9));
        let eq30_e1215_d_b10: f64 = (eq30_e1213_d_b10 * (nv11 - nv9));
        let eq30_e1215_d_b11: f64 = (eq30_e1213_d_b11 * (nv11 - nv9));
        let eq30_e1215_d_b12: f64 = (eq30_e1213_d_b12 * (nv11 - nv9));
        let eq30_e1215_d_b13: f64 = (eq30_e1213_d_b13 * (nv11 - nv9));
        let eq30_e1215_d_b14: f64 = (eq30_e1213_d_b14 * (nv11 - nv9));
        let eq30_e1215_d_b15: f64 = (eq30_e1213_d_b15 * (nv11 - nv9));
        let eq30_e1215_d_b16: f64 = (eq30_e1213_d_b16 * (nv11 - nv9));
        let eq30_e1215_d_b17: f64 = (eq30_e1213_d_b17 * (nv11 - nv9));
        let eq30_e1215_d_b18: f64 = (eq30_e1213_d_b18 * (nv11 - nv9));
        let eq30_e1215_d_b19: f64 = (eq30_e1213_d_b19 * (nv11 - nv9));
        let eq30_e1215_d_b20: f64 = (eq30_e1213_d_b20 * (nv11 - nv9));
        let eq30_e1215_d_b21: f64 = (eq30_e1213_d_b21 * (nv11 - nv9));
        let eq30_e1215_d_b22: f64 = (eq30_e1213_d_b22 * (nv11 - nv9));
        let eq30_e1215_d_b23: f64 = (eq30_e1213_d_b23 * (nv11 - nv9));
        let eq30_e1215_d_b24: f64 = (eq30_e1213_d_b24 * (nv11 - nv9));
        (eq30_e1215, eq30_e1215_d_n0, eq30_e1215_d_n1, eq30_e1215_d_n2, eq30_e1215_d_n3, eq30_e1215_d_n4, eq30_e1215_d_n5, eq30_e1215_d_n6, eq30_e1215_d_n7, eq30_e1215_d_n8, eq30_e1215_d_n9, eq30_e1215_d_n10, eq30_e1215_d_n11, eq30_e1215_d_n12, eq30_e1215_d_n13, eq30_e1215_d_n14, eq30_e1215_d_n15, eq30_e1215_d_n16, eq30_e1215_d_n17, eq30_e1215_d_n18, eq30_e1215_d_n19, eq30_e1215_d_n20, eq30_e1215_d_b0, eq30_e1215_d_b1, eq30_e1215_d_b2, eq30_e1215_d_b3, eq30_e1215_d_b4, eq30_e1215_d_b5, eq30_e1215_d_b6, eq30_e1215_d_b7, eq30_e1215_d_b8, eq30_e1215_d_b9, eq30_e1215_d_b10, eq30_e1215_d_b11, eq30_e1215_d_b12, eq30_e1215_d_b13, eq30_e1215_d_b14, eq30_e1215_d_b15, eq30_e1215_d_b16, eq30_e1215_d_b17, eq30_e1215_d_b18, eq30_e1215_d_b19, eq30_e1215_d_b20, eq30_e1215_d_b21, eq30_e1215_d_b22, eq30_e1215_d_b23, eq30_e1215_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1217;
        let eq30_node_derivatives: [f64; 21] = [eq30_e1217_d_n0, eq30_e1217_d_n1, eq30_e1217_d_n2, eq30_e1217_d_n3, eq30_e1217_d_n4, eq30_e1217_d_n5, eq30_e1217_d_n6, eq30_e1217_d_n7, eq30_e1217_d_n8, eq30_e1217_d_n9, eq30_e1217_d_n10, eq30_e1217_d_n11, eq30_e1217_d_n12, eq30_e1217_d_n13, eq30_e1217_d_n14, eq30_e1217_d_n15, eq30_e1217_d_n16, eq30_e1217_d_n17, eq30_e1217_d_n18, eq30_e1217_d_n19, eq30_e1217_d_n20];
        let eq30_branch_derivatives: [f64; 25] = [eq30_e1217_d_b0, eq30_e1217_d_b1, eq30_e1217_d_b2, eq30_e1217_d_b3, eq30_e1217_d_b4, eq30_e1217_d_b5, eq30_e1217_d_b6, eq30_e1217_d_b7, eq30_e1217_d_b8, eq30_e1217_d_b9, eq30_e1217_d_b10, eq30_e1217_d_b11, eq30_e1217_d_b12, eq30_e1217_d_b13, eq30_e1217_d_b14, eq30_e1217_d_b15, eq30_e1217_d_b16, eq30_e1217_d_b17, eq30_e1217_d_b18, eq30_e1217_d_b19, eq30_e1217_d_b20, eq30_e1217_d_b21, eq30_e1217_d_b22, eq30_e1217_d_b23, eq30_e1217_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[9]),
            self.multiplicity * (eq30_value),
            &nodes,
            &eq30_node_derivatives,
            &branches,
            &eq30_branch_derivatives,
            self.multiplicity,
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
        let (eq31_e1227,) = {
    if (s.v[2919] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e1227;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[9]),
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
        let (eq32_e1232,) = {
    if (!(s.v[2919] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1232;
        stamper.stamp_potential(
            branches[5],
            eq32_value,
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq33_e1242, eq33_e1242_d_n0, eq33_e1242_d_n1, eq33_e1242_d_n2, eq33_e1242_d_n3, eq33_e1242_d_n4, eq33_e1242_d_n5, eq33_e1242_d_n6, eq33_e1242_d_n7, eq33_e1242_d_n8, eq33_e1242_d_n9, eq33_e1242_d_n10, eq33_e1242_d_n11, eq33_e1242_d_n12, eq33_e1242_d_n13, eq33_e1242_d_n14, eq33_e1242_d_n15, eq33_e1242_d_n16, eq33_e1242_d_n17, eq33_e1242_d_n18, eq33_e1242_d_n19, eq33_e1242_d_n20, eq33_e1242_d_b0, eq33_e1242_d_b1, eq33_e1242_d_b2, eq33_e1242_d_b3, eq33_e1242_d_b4, eq33_e1242_d_b5, eq33_e1242_d_b6, eq33_e1242_d_b7, eq33_e1242_d_b8, eq33_e1242_d_b9, eq33_e1242_d_b10, eq33_e1242_d_b11, eq33_e1242_d_b12, eq33_e1242_d_b13, eq33_e1242_d_b14, eq33_e1242_d_b15, eq33_e1242_d_b16, eq33_e1242_d_b17, eq33_e1242_d_b18, eq33_e1242_d_b19, eq33_e1242_d_b20, eq33_e1242_d_b21, eq33_e1242_d_b22, eq33_e1242_d_b23, eq33_e1242_d_b24,) = {
    if (s.v[2920] != 0.0) {
        let eq33_e1236: f64 = (s.v[19] * p.p32);
        let eq33_e1236_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq33_e1236_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq33_e1236_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq33_e1236_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq33_e1236_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq33_e1236_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq33_e1236_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq33_e1236_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq33_e1236_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq33_e1236_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq33_e1236_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq33_e1236_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq33_e1236_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq33_e1236_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq33_e1236_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq33_e1236_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq33_e1236_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq33_e1236_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq33_e1236_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq33_e1236_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq33_e1236_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq33_e1236_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq33_e1236_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq33_e1236_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq33_e1236_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq33_e1236_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq33_e1236_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq33_e1236_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq33_e1236_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq33_e1236_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq33_e1236_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq33_e1236_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq33_e1236_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq33_e1236_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq33_e1236_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq33_e1236_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq33_e1236_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq33_e1236_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq33_e1236_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq33_e1236_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq33_e1236_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq33_e1236_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq33_e1236_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq33_e1236_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq33_e1236_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq33_e1236_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq33_e1238: f64 = (eq33_e1236 * s.v[817]);
        let eq33_e1238_d_n0: f64 = ((eq33_e1236_d_n0 * s.v[817]) + (eq33_e1236 * s.dn[817][0]));
        let eq33_e1238_d_n1: f64 = ((eq33_e1236_d_n1 * s.v[817]) + (eq33_e1236 * s.dn[817][1]));
        let eq33_e1238_d_n2: f64 = ((eq33_e1236_d_n2 * s.v[817]) + (eq33_e1236 * s.dn[817][2]));
        let eq33_e1238_d_n3: f64 = ((eq33_e1236_d_n3 * s.v[817]) + (eq33_e1236 * s.dn[817][3]));
        let eq33_e1238_d_n4: f64 = ((eq33_e1236_d_n4 * s.v[817]) + (eq33_e1236 * s.dn[817][4]));
        let eq33_e1238_d_n5: f64 = ((eq33_e1236_d_n5 * s.v[817]) + (eq33_e1236 * s.dn[817][5]));
        let eq33_e1238_d_n6: f64 = ((eq33_e1236_d_n6 * s.v[817]) + (eq33_e1236 * s.dn[817][6]));
        let eq33_e1238_d_n7: f64 = ((eq33_e1236_d_n7 * s.v[817]) + (eq33_e1236 * s.dn[817][7]));
        let eq33_e1238_d_n8: f64 = ((eq33_e1236_d_n8 * s.v[817]) + (eq33_e1236 * s.dn[817][8]));
        let eq33_e1238_d_n9: f64 = ((eq33_e1236_d_n9 * s.v[817]) + (eq33_e1236 * s.dn[817][9]));
        let eq33_e1238_d_n10: f64 = ((eq33_e1236_d_n10 * s.v[817]) + (eq33_e1236 * s.dn[817][10]));
        let eq33_e1238_d_n11: f64 = ((eq33_e1236_d_n11 * s.v[817]) + (eq33_e1236 * s.dn[817][11]));
        let eq33_e1238_d_n12: f64 = ((eq33_e1236_d_n12 * s.v[817]) + (eq33_e1236 * s.dn[817][12]));
        let eq33_e1238_d_n13: f64 = ((eq33_e1236_d_n13 * s.v[817]) + (eq33_e1236 * s.dn[817][13]));
        let eq33_e1238_d_n14: f64 = ((eq33_e1236_d_n14 * s.v[817]) + (eq33_e1236 * s.dn[817][14]));
        let eq33_e1238_d_n15: f64 = ((eq33_e1236_d_n15 * s.v[817]) + (eq33_e1236 * s.dn[817][15]));
        let eq33_e1238_d_n16: f64 = ((eq33_e1236_d_n16 * s.v[817]) + (eq33_e1236 * s.dn[817][16]));
        let eq33_e1238_d_n17: f64 = ((eq33_e1236_d_n17 * s.v[817]) + (eq33_e1236 * s.dn[817][17]));
        let eq33_e1238_d_n18: f64 = ((eq33_e1236_d_n18 * s.v[817]) + (eq33_e1236 * s.dn[817][18]));
        let eq33_e1238_d_n19: f64 = ((eq33_e1236_d_n19 * s.v[817]) + (eq33_e1236 * s.dn[817][19]));
        let eq33_e1238_d_n20: f64 = ((eq33_e1236_d_n20 * s.v[817]) + (eq33_e1236 * s.dn[817][20]));
        let eq33_e1238_d_b0: f64 = ((eq33_e1236_d_b0 * s.v[817]) + (eq33_e1236 * s.db[817][0]));
        let eq33_e1238_d_b1: f64 = ((eq33_e1236_d_b1 * s.v[817]) + (eq33_e1236 * s.db[817][1]));
        let eq33_e1238_d_b2: f64 = ((eq33_e1236_d_b2 * s.v[817]) + (eq33_e1236 * s.db[817][2]));
        let eq33_e1238_d_b3: f64 = ((eq33_e1236_d_b3 * s.v[817]) + (eq33_e1236 * s.db[817][3]));
        let eq33_e1238_d_b4: f64 = ((eq33_e1236_d_b4 * s.v[817]) + (eq33_e1236 * s.db[817][4]));
        let eq33_e1238_d_b5: f64 = ((eq33_e1236_d_b5 * s.v[817]) + (eq33_e1236 * s.db[817][5]));
        let eq33_e1238_d_b6: f64 = ((eq33_e1236_d_b6 * s.v[817]) + (eq33_e1236 * s.db[817][6]));
        let eq33_e1238_d_b7: f64 = ((eq33_e1236_d_b7 * s.v[817]) + (eq33_e1236 * s.db[817][7]));
        let eq33_e1238_d_b8: f64 = ((eq33_e1236_d_b8 * s.v[817]) + (eq33_e1236 * s.db[817][8]));
        let eq33_e1238_d_b9: f64 = ((eq33_e1236_d_b9 * s.v[817]) + (eq33_e1236 * s.db[817][9]));
        let eq33_e1238_d_b10: f64 = ((eq33_e1236_d_b10 * s.v[817]) + (eq33_e1236 * s.db[817][10]));
        let eq33_e1238_d_b11: f64 = ((eq33_e1236_d_b11 * s.v[817]) + (eq33_e1236 * s.db[817][11]));
        let eq33_e1238_d_b12: f64 = ((eq33_e1236_d_b12 * s.v[817]) + (eq33_e1236 * s.db[817][12]));
        let eq33_e1238_d_b13: f64 = ((eq33_e1236_d_b13 * s.v[817]) + (eq33_e1236 * s.db[817][13]));
        let eq33_e1238_d_b14: f64 = ((eq33_e1236_d_b14 * s.v[817]) + (eq33_e1236 * s.db[817][14]));
        let eq33_e1238_d_b15: f64 = ((eq33_e1236_d_b15 * s.v[817]) + (eq33_e1236 * s.db[817][15]));
        let eq33_e1238_d_b16: f64 = ((eq33_e1236_d_b16 * s.v[817]) + (eq33_e1236 * s.db[817][16]));
        let eq33_e1238_d_b17: f64 = ((eq33_e1236_d_b17 * s.v[817]) + (eq33_e1236 * s.db[817][17]));
        let eq33_e1238_d_b18: f64 = ((eq33_e1236_d_b18 * s.v[817]) + (eq33_e1236 * s.db[817][18]));
        let eq33_e1238_d_b19: f64 = ((eq33_e1236_d_b19 * s.v[817]) + (eq33_e1236 * s.db[817][19]));
        let eq33_e1238_d_b20: f64 = ((eq33_e1236_d_b20 * s.v[817]) + (eq33_e1236 * s.db[817][20]));
        let eq33_e1238_d_b21: f64 = ((eq33_e1236_d_b21 * s.v[817]) + (eq33_e1236 * s.db[817][21]));
        let eq33_e1238_d_b22: f64 = ((eq33_e1236_d_b22 * s.v[817]) + (eq33_e1236 * s.db[817][22]));
        let eq33_e1238_d_b23: f64 = ((eq33_e1236_d_b23 * s.v[817]) + (eq33_e1236 * s.db[817][23]));
        let eq33_e1238_d_b24: f64 = ((eq33_e1236_d_b24 * s.v[817]) + (eq33_e1236 * s.db[817][24]));
        let eq33_e1240: f64 = (eq33_e1238 * (nv3 - nv9));
        let eq33_e1240_d_n0: f64 = (eq33_e1238_d_n0 * (nv3 - nv9));
        let eq33_e1240_d_n1: f64 = (eq33_e1238_d_n1 * (nv3 - nv9));
        let eq33_e1240_d_n2: f64 = (eq33_e1238_d_n2 * (nv3 - nv9));
        let eq33_e1240_d_n3: f64 = ((eq33_e1238_d_n3 * (nv3 - nv9)) + eq33_e1238);
        let eq33_e1240_d_n4: f64 = (eq33_e1238_d_n4 * (nv3 - nv9));
        let eq33_e1240_d_n5: f64 = (eq33_e1238_d_n5 * (nv3 - nv9));
        let eq33_e1240_d_n6: f64 = (eq33_e1238_d_n6 * (nv3 - nv9));
        let eq33_e1240_d_n7: f64 = (eq33_e1238_d_n7 * (nv3 - nv9));
        let eq33_e1240_d_n8: f64 = (eq33_e1238_d_n8 * (nv3 - nv9));
        let eq33_e1240_d_n9: f64 = ((eq33_e1238_d_n9 * (nv3 - nv9)) + (-eq33_e1238));
        let eq33_e1240_d_n10: f64 = (eq33_e1238_d_n10 * (nv3 - nv9));
        let eq33_e1240_d_n11: f64 = (eq33_e1238_d_n11 * (nv3 - nv9));
        let eq33_e1240_d_n12: f64 = (eq33_e1238_d_n12 * (nv3 - nv9));
        let eq33_e1240_d_n13: f64 = (eq33_e1238_d_n13 * (nv3 - nv9));
        let eq33_e1240_d_n14: f64 = (eq33_e1238_d_n14 * (nv3 - nv9));
        let eq33_e1240_d_n15: f64 = (eq33_e1238_d_n15 * (nv3 - nv9));
        let eq33_e1240_d_n16: f64 = (eq33_e1238_d_n16 * (nv3 - nv9));
        let eq33_e1240_d_n17: f64 = (eq33_e1238_d_n17 * (nv3 - nv9));
        let eq33_e1240_d_n18: f64 = (eq33_e1238_d_n18 * (nv3 - nv9));
        let eq33_e1240_d_n19: f64 = (eq33_e1238_d_n19 * (nv3 - nv9));
        let eq33_e1240_d_n20: f64 = (eq33_e1238_d_n20 * (nv3 - nv9));
        let eq33_e1240_d_b0: f64 = (eq33_e1238_d_b0 * (nv3 - nv9));
        let eq33_e1240_d_b1: f64 = (eq33_e1238_d_b1 * (nv3 - nv9));
        let eq33_e1240_d_b2: f64 = (eq33_e1238_d_b2 * (nv3 - nv9));
        let eq33_e1240_d_b3: f64 = (eq33_e1238_d_b3 * (nv3 - nv9));
        let eq33_e1240_d_b4: f64 = (eq33_e1238_d_b4 * (nv3 - nv9));
        let eq33_e1240_d_b5: f64 = (eq33_e1238_d_b5 * (nv3 - nv9));
        let eq33_e1240_d_b6: f64 = (eq33_e1238_d_b6 * (nv3 - nv9));
        let eq33_e1240_d_b7: f64 = (eq33_e1238_d_b7 * (nv3 - nv9));
        let eq33_e1240_d_b8: f64 = (eq33_e1238_d_b8 * (nv3 - nv9));
        let eq33_e1240_d_b9: f64 = (eq33_e1238_d_b9 * (nv3 - nv9));
        let eq33_e1240_d_b10: f64 = (eq33_e1238_d_b10 * (nv3 - nv9));
        let eq33_e1240_d_b11: f64 = (eq33_e1238_d_b11 * (nv3 - nv9));
        let eq33_e1240_d_b12: f64 = (eq33_e1238_d_b12 * (nv3 - nv9));
        let eq33_e1240_d_b13: f64 = (eq33_e1238_d_b13 * (nv3 - nv9));
        let eq33_e1240_d_b14: f64 = (eq33_e1238_d_b14 * (nv3 - nv9));
        let eq33_e1240_d_b15: f64 = (eq33_e1238_d_b15 * (nv3 - nv9));
        let eq33_e1240_d_b16: f64 = (eq33_e1238_d_b16 * (nv3 - nv9));
        let eq33_e1240_d_b17: f64 = (eq33_e1238_d_b17 * (nv3 - nv9));
        let eq33_e1240_d_b18: f64 = (eq33_e1238_d_b18 * (nv3 - nv9));
        let eq33_e1240_d_b19: f64 = (eq33_e1238_d_b19 * (nv3 - nv9));
        let eq33_e1240_d_b20: f64 = (eq33_e1238_d_b20 * (nv3 - nv9));
        let eq33_e1240_d_b21: f64 = (eq33_e1238_d_b21 * (nv3 - nv9));
        let eq33_e1240_d_b22: f64 = (eq33_e1238_d_b22 * (nv3 - nv9));
        let eq33_e1240_d_b23: f64 = (eq33_e1238_d_b23 * (nv3 - nv9));
        let eq33_e1240_d_b24: f64 = (eq33_e1238_d_b24 * (nv3 - nv9));
        (eq33_e1240, eq33_e1240_d_n0, eq33_e1240_d_n1, eq33_e1240_d_n2, eq33_e1240_d_n3, eq33_e1240_d_n4, eq33_e1240_d_n5, eq33_e1240_d_n6, eq33_e1240_d_n7, eq33_e1240_d_n8, eq33_e1240_d_n9, eq33_e1240_d_n10, eq33_e1240_d_n11, eq33_e1240_d_n12, eq33_e1240_d_n13, eq33_e1240_d_n14, eq33_e1240_d_n15, eq33_e1240_d_n16, eq33_e1240_d_n17, eq33_e1240_d_n18, eq33_e1240_d_n19, eq33_e1240_d_n20, eq33_e1240_d_b0, eq33_e1240_d_b1, eq33_e1240_d_b2, eq33_e1240_d_b3, eq33_e1240_d_b4, eq33_e1240_d_b5, eq33_e1240_d_b6, eq33_e1240_d_b7, eq33_e1240_d_b8, eq33_e1240_d_b9, eq33_e1240_d_b10, eq33_e1240_d_b11, eq33_e1240_d_b12, eq33_e1240_d_b13, eq33_e1240_d_b14, eq33_e1240_d_b15, eq33_e1240_d_b16, eq33_e1240_d_b17, eq33_e1240_d_b18, eq33_e1240_d_b19, eq33_e1240_d_b20, eq33_e1240_d_b21, eq33_e1240_d_b22, eq33_e1240_d_b23, eq33_e1240_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1242;
        let eq33_node_derivatives: [f64; 21] = [eq33_e1242_d_n0, eq33_e1242_d_n1, eq33_e1242_d_n2, eq33_e1242_d_n3, eq33_e1242_d_n4, eq33_e1242_d_n5, eq33_e1242_d_n6, eq33_e1242_d_n7, eq33_e1242_d_n8, eq33_e1242_d_n9, eq33_e1242_d_n10, eq33_e1242_d_n11, eq33_e1242_d_n12, eq33_e1242_d_n13, eq33_e1242_d_n14, eq33_e1242_d_n15, eq33_e1242_d_n16, eq33_e1242_d_n17, eq33_e1242_d_n18, eq33_e1242_d_n19, eq33_e1242_d_n20];
        let eq33_branch_derivatives: [f64; 25] = [eq33_e1242_d_b0, eq33_e1242_d_b1, eq33_e1242_d_b2, eq33_e1242_d_b3, eq33_e1242_d_b4, eq33_e1242_d_b5, eq33_e1242_d_b6, eq33_e1242_d_b7, eq33_e1242_d_b8, eq33_e1242_d_b9, eq33_e1242_d_b10, eq33_e1242_d_b11, eq33_e1242_d_b12, eq33_e1242_d_b13, eq33_e1242_d_b14, eq33_e1242_d_b15, eq33_e1242_d_b16, eq33_e1242_d_b17, eq33_e1242_d_b18, eq33_e1242_d_b19, eq33_e1242_d_b20, eq33_e1242_d_b21, eq33_e1242_d_b22, eq33_e1242_d_b23, eq33_e1242_d_b24];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[9]),
            self.multiplicity * (eq33_value),
            &nodes,
            &eq33_node_derivatives,
            &branches,
            &eq33_branch_derivatives,
            self.multiplicity,
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
        let (eq34_e1252,) = {
    if (s.v[2920] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e1252;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[9]),
            self.multiplicity * (eq34_value),
            &[
            ],
        );
    }
}
