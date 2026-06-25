#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_196_block_0(
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
        let (eq196_e2178, eq196_e2178_d_n4,) = {
    if (s.v[2700] != 0.0) {
        let eq196_e2176: f64 = ((nv4 - 0.0) / p.p320);
        let eq196_e2176_d_n4: f64 = (1.0 / p.p320);
        (eq196_e2176, eq196_e2176_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq196_value: f64 = eq196_e2178;
        stamper.stamp_current(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq196_value),
            &[
                GeneratedDerivative::node(nodes[4], self.multiplicity * eq196_e2178_d_n4),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_197_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq197_e2183,) = {
    if (!(s.v[2700] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq197_value: f64 = eq197_e2183;
        stamper.stamp_potential(
            branches[35],
            eq197_value,
            &[
            ],
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
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let (eq8_e421, eq8_e421_d_n20, eq8_e421_d_n21, eq8_e421_q, eq8_e421_q_d_n20, eq8_e421_q_d_n21,) = {
    if (s.v[308] != 0.0) {
        let eq8_e418: f64 = (p.p330 * (nv21 - nv20));
        let eq8_e418_d_n20: f64 = (-p.p330);
        let eq8_e418_d_n21: f64 = p.p330;
        let eq8_e419_q: f64 = eq8_e418;
        (eq8_e418, eq8_e418_d_n20, eq8_e418_d_n21, eq8_e419_q, eq8_e418_d_n20, eq8_e418_d_n21,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[21]),
            Some(nodes[20]),
            &[
                GeneratedDerivative::node(nodes[20], self.multiplicity * (eq8_e421_q_d_n20)),
                GeneratedDerivative::node(nodes[21], self.multiplicity * (eq8_e421_q_d_n21)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv20 = ctx.node_voltage(nodes[20]);
        let (eq9_e428, eq9_e428_d_n20, eq9_e428_q, eq9_e428_q_d_n20,) = {
    if (s.v[308] != 0.0) {
        let eq9_e425: f64 = (p.p332 * (nv20 - 0.0));
        let eq9_e425_d_n20: f64 = p.p332;
        let eq9_e426_q: f64 = eq9_e425;
        (eq9_e425, eq9_e425_d_n20, eq9_e426_q, eq9_e425_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[20]),
            None,
            &[
                GeneratedDerivative::node(nodes[20], self.multiplicity * (eq9_e428_q_d_n20)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq17_e564, eq17_e564_d_n0, eq17_e564_d_n1, eq17_e564_d_n2, eq17_e564_d_n3, eq17_e564_d_n4, eq17_e564_d_n5, eq17_e564_d_n6, eq17_e564_d_n7, eq17_e564_d_n8, eq17_e564_d_n9, eq17_e564_d_n10, eq17_e564_d_n11, eq17_e564_d_n12, eq17_e564_d_n13, eq17_e564_d_n14, eq17_e564_d_n15, eq17_e564_d_n16, eq17_e564_d_n17, eq17_e564_d_n18, eq17_e564_d_n19, eq17_e564_d_n20, eq17_e564_d_n21, eq17_e564_d_n22, eq17_e564_d_n23, eq17_e564_d_n24, eq17_e564_d_n25, eq17_e564_d_n26, eq17_e564_d_n27, eq17_e564_d_n28, eq17_e564_d_n29, eq17_e564_q, eq17_e564_q_d_n0, eq17_e564_q_d_n1, eq17_e564_q_d_n2, eq17_e564_q_d_n3, eq17_e564_q_d_n4, eq17_e564_q_d_n5, eq17_e564_q_d_n6, eq17_e564_q_d_n7, eq17_e564_q_d_n8, eq17_e564_q_d_n9, eq17_e564_q_d_n10, eq17_e564_q_d_n11, eq17_e564_q_d_n12, eq17_e564_q_d_n13, eq17_e564_q_d_n14, eq17_e564_q_d_n15, eq17_e564_q_d_n16, eq17_e564_q_d_n17, eq17_e564_q_d_n18, eq17_e564_q_d_n19, eq17_e564_q_d_n20, eq17_e564_q_d_n21, eq17_e564_q_d_n22, eq17_e564_q_d_n23, eq17_e564_q_d_n24, eq17_e564_q_d_n25, eq17_e564_q_d_n26, eq17_e564_q_d_n27, eq17_e564_q_d_n28, eq17_e564_q_d_n29,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq17_e543_q: f64 = s.v[225];
        let eq17_e544: f64 = (p.p341 * s.v[225]);
        let eq17_e544_d_n0: f64 = (p.p341 * s.dn[225][0]);
        let eq17_e544_d_n1: f64 = (p.p341 * s.dn[225][1]);
        let eq17_e544_d_n2: f64 = (p.p341 * s.dn[225][2]);
        let eq17_e544_d_n3: f64 = (p.p341 * s.dn[225][3]);
        let eq17_e544_d_n4: f64 = (p.p341 * s.dn[225][4]);
        let eq17_e544_d_n5: f64 = (p.p341 * s.dn[225][5]);
        let eq17_e544_d_n6: f64 = (p.p341 * s.dn[225][6]);
        let eq17_e544_d_n7: f64 = (p.p341 * s.dn[225][7]);
        let eq17_e544_d_n8: f64 = (p.p341 * s.dn[225][8]);
        let eq17_e544_d_n9: f64 = (p.p341 * s.dn[225][9]);
        let eq17_e544_d_n10: f64 = (p.p341 * s.dn[225][10]);
        let eq17_e544_d_n11: f64 = (p.p341 * s.dn[225][11]);
        let eq17_e544_d_n12: f64 = (p.p341 * s.dn[225][12]);
        let eq17_e544_d_n13: f64 = (p.p341 * s.dn[225][13]);
        let eq17_e544_d_n14: f64 = (p.p341 * s.dn[225][14]);
        let eq17_e544_d_n15: f64 = (p.p341 * s.dn[225][15]);
        let eq17_e544_d_n16: f64 = (p.p341 * s.dn[225][16]);
        let eq17_e544_d_n17: f64 = (p.p341 * s.dn[225][17]);
        let eq17_e544_d_n18: f64 = (p.p341 * s.dn[225][18]);
        let eq17_e544_d_n19: f64 = (p.p341 * s.dn[225][19]);
        let eq17_e544_d_n20: f64 = (p.p341 * s.dn[225][20]);
        let eq17_e544_d_n21: f64 = (p.p341 * s.dn[225][21]);
        let eq17_e544_d_n22: f64 = (p.p341 * s.dn[225][22]);
        let eq17_e544_d_n23: f64 = (p.p341 * s.dn[225][23]);
        let eq17_e544_d_n24: f64 = (p.p341 * s.dn[225][24]);
        let eq17_e544_d_n25: f64 = (p.p341 * s.dn[225][25]);
        let eq17_e544_d_n26: f64 = (p.p341 * s.dn[225][26]);
        let eq17_e544_d_n27: f64 = (p.p341 * s.dn[225][27]);
        let eq17_e544_d_n28: f64 = (p.p341 * s.dn[225][28]);
        let eq17_e544_d_n29: f64 = (p.p341 * s.dn[225][29]);
        let eq17_e544_q: f64 = (p.p341 * eq17_e543_q);
        let eq17_e544_q_d_n0: f64 = (p.p341 * s.dn[225][0]);
        let eq17_e544_q_d_n1: f64 = (p.p341 * s.dn[225][1]);
        let eq17_e544_q_d_n2: f64 = (p.p341 * s.dn[225][2]);
        let eq17_e544_q_d_n3: f64 = (p.p341 * s.dn[225][3]);
        let eq17_e544_q_d_n4: f64 = (p.p341 * s.dn[225][4]);
        let eq17_e544_q_d_n5: f64 = (p.p341 * s.dn[225][5]);
        let eq17_e544_q_d_n6: f64 = (p.p341 * s.dn[225][6]);
        let eq17_e544_q_d_n7: f64 = (p.p341 * s.dn[225][7]);
        let eq17_e544_q_d_n8: f64 = (p.p341 * s.dn[225][8]);
        let eq17_e544_q_d_n9: f64 = (p.p341 * s.dn[225][9]);
        let eq17_e544_q_d_n10: f64 = (p.p341 * s.dn[225][10]);
        let eq17_e544_q_d_n11: f64 = (p.p341 * s.dn[225][11]);
        let eq17_e544_q_d_n12: f64 = (p.p341 * s.dn[225][12]);
        let eq17_e544_q_d_n13: f64 = (p.p341 * s.dn[225][13]);
        let eq17_e544_q_d_n14: f64 = (p.p341 * s.dn[225][14]);
        let eq17_e544_q_d_n15: f64 = (p.p341 * s.dn[225][15]);
        let eq17_e544_q_d_n16: f64 = (p.p341 * s.dn[225][16]);
        let eq17_e544_q_d_n17: f64 = (p.p341 * s.dn[225][17]);
        let eq17_e544_q_d_n18: f64 = (p.p341 * s.dn[225][18]);
        let eq17_e544_q_d_n19: f64 = (p.p341 * s.dn[225][19]);
        let eq17_e544_q_d_n20: f64 = (p.p341 * s.dn[225][20]);
        let eq17_e544_q_d_n21: f64 = (p.p341 * s.dn[225][21]);
        let eq17_e544_q_d_n22: f64 = (p.p341 * s.dn[225][22]);
        let eq17_e544_q_d_n23: f64 = (p.p341 * s.dn[225][23]);
        let eq17_e544_q_d_n24: f64 = (p.p341 * s.dn[225][24]);
        let eq17_e544_q_d_n25: f64 = (p.p341 * s.dn[225][25]);
        let eq17_e544_q_d_n26: f64 = (p.p341 * s.dn[225][26]);
        let eq17_e544_q_d_n27: f64 = (p.p341 * s.dn[225][27]);
        let eq17_e544_q_d_n28: f64 = (p.p341 * s.dn[225][28]);
        let eq17_e544_q_d_n29: f64 = (p.p341 * s.dn[225][29]);
        let eq17_e549: f64 = (s.v[111] - s.v[109]);
        let eq17_e550: f64 = (p.p342 * eq17_e549);
        let eq17_e550_d_n0: f64 = (p.p342 * s.dn[111][0]);
        let eq17_e550_d_n1: f64 = (p.p342 * s.dn[111][1]);
        let eq17_e550_d_n2: f64 = (p.p342 * s.dn[111][2]);
        let eq17_e550_d_n3: f64 = (p.p342 * s.dn[111][3]);
        let eq17_e550_d_n4: f64 = (p.p342 * s.dn[111][4]);
        let eq17_e550_d_n5: f64 = (p.p342 * s.dn[111][5]);
        let eq17_e550_d_n6: f64 = (p.p342 * s.dn[111][6]);
        let eq17_e550_d_n7: f64 = (p.p342 * s.dn[111][7]);
        let eq17_e550_d_n8: f64 = (p.p342 * s.dn[111][8]);
        let eq17_e550_d_n9: f64 = (p.p342 * s.dn[111][9]);
        let eq17_e550_d_n10: f64 = (p.p342 * s.dn[111][10]);
        let eq17_e550_d_n11: f64 = (p.p342 * s.dn[111][11]);
        let eq17_e550_d_n12: f64 = (p.p342 * s.dn[111][12]);
        let eq17_e550_d_n13: f64 = (p.p342 * s.dn[111][13]);
        let eq17_e550_d_n14: f64 = (p.p342 * s.dn[111][14]);
        let eq17_e550_d_n15: f64 = (p.p342 * s.dn[111][15]);
        let eq17_e550_d_n16: f64 = (p.p342 * s.dn[111][16]);
        let eq17_e550_d_n17: f64 = (p.p342 * s.dn[111][17]);
        let eq17_e550_d_n18: f64 = (p.p342 * s.dn[111][18]);
        let eq17_e550_d_n19: f64 = (p.p342 * s.dn[111][19]);
        let eq17_e550_d_n20: f64 = (p.p342 * s.dn[111][20]);
        let eq17_e550_d_n21: f64 = (p.p342 * s.dn[111][21]);
        let eq17_e550_d_n22: f64 = (p.p342 * s.dn[111][22]);
        let eq17_e550_d_n23: f64 = (p.p342 * s.dn[111][23]);
        let eq17_e550_d_n24: f64 = (p.p342 * s.dn[111][24]);
        let eq17_e550_d_n25: f64 = (p.p342 * s.dn[111][25]);
        let eq17_e550_d_n26: f64 = (p.p342 * s.dn[111][26]);
        let eq17_e550_d_n27: f64 = (p.p342 * s.dn[111][27]);
        let eq17_e550_d_n28: f64 = (p.p342 * s.dn[111][28]);
        let eq17_e550_d_n29: f64 = (p.p342 * s.dn[111][29]);
        let eq17_e551: f64 = (1.0 + eq17_e550);
        let eq17_e555: f64 = (s.v[111] - s.v[109]);
        let eq17_e556: f64 = (p.p344 * eq17_e555);
        let eq17_e556_d_n0: f64 = (p.p344 * s.dn[111][0]);
        let eq17_e556_d_n1: f64 = (p.p344 * s.dn[111][1]);
        let eq17_e556_d_n2: f64 = (p.p344 * s.dn[111][2]);
        let eq17_e556_d_n3: f64 = (p.p344 * s.dn[111][3]);
        let eq17_e556_d_n4: f64 = (p.p344 * s.dn[111][4]);
        let eq17_e556_d_n5: f64 = (p.p344 * s.dn[111][5]);
        let eq17_e556_d_n6: f64 = (p.p344 * s.dn[111][6]);
        let eq17_e556_d_n7: f64 = (p.p344 * s.dn[111][7]);
        let eq17_e556_d_n8: f64 = (p.p344 * s.dn[111][8]);
        let eq17_e556_d_n9: f64 = (p.p344 * s.dn[111][9]);
        let eq17_e556_d_n10: f64 = (p.p344 * s.dn[111][10]);
        let eq17_e556_d_n11: f64 = (p.p344 * s.dn[111][11]);
        let eq17_e556_d_n12: f64 = (p.p344 * s.dn[111][12]);
        let eq17_e556_d_n13: f64 = (p.p344 * s.dn[111][13]);
        let eq17_e556_d_n14: f64 = (p.p344 * s.dn[111][14]);
        let eq17_e556_d_n15: f64 = (p.p344 * s.dn[111][15]);
        let eq17_e556_d_n16: f64 = (p.p344 * s.dn[111][16]);
        let eq17_e556_d_n17: f64 = (p.p344 * s.dn[111][17]);
        let eq17_e556_d_n18: f64 = (p.p344 * s.dn[111][18]);
        let eq17_e556_d_n19: f64 = (p.p344 * s.dn[111][19]);
        let eq17_e556_d_n20: f64 = (p.p344 * s.dn[111][20]);
        let eq17_e556_d_n21: f64 = (p.p344 * s.dn[111][21]);
        let eq17_e556_d_n22: f64 = (p.p344 * s.dn[111][22]);
        let eq17_e556_d_n23: f64 = (p.p344 * s.dn[111][23]);
        let eq17_e556_d_n24: f64 = (p.p344 * s.dn[111][24]);
        let eq17_e556_d_n25: f64 = (p.p344 * s.dn[111][25]);
        let eq17_e556_d_n26: f64 = (p.p344 * s.dn[111][26]);
        let eq17_e556_d_n27: f64 = (p.p344 * s.dn[111][27]);
        let eq17_e556_d_n28: f64 = (p.p344 * s.dn[111][28]);
        let eq17_e556_d_n29: f64 = (p.p344 * s.dn[111][29]);
        let eq17_e559: f64 = (s.v[111] - s.v[109]);
        let eq17_e560: f64 = (eq17_e556 * eq17_e559);
        let eq17_e560_d_n0: f64 = ((eq17_e556_d_n0 * eq17_e559) + (eq17_e556 * s.dn[111][0]));
        let eq17_e560_d_n1: f64 = ((eq17_e556_d_n1 * eq17_e559) + (eq17_e556 * s.dn[111][1]));
        let eq17_e560_d_n2: f64 = ((eq17_e556_d_n2 * eq17_e559) + (eq17_e556 * s.dn[111][2]));
        let eq17_e560_d_n3: f64 = ((eq17_e556_d_n3 * eq17_e559) + (eq17_e556 * s.dn[111][3]));
        let eq17_e560_d_n4: f64 = ((eq17_e556_d_n4 * eq17_e559) + (eq17_e556 * s.dn[111][4]));
        let eq17_e560_d_n5: f64 = ((eq17_e556_d_n5 * eq17_e559) + (eq17_e556 * s.dn[111][5]));
        let eq17_e560_d_n6: f64 = ((eq17_e556_d_n6 * eq17_e559) + (eq17_e556 * s.dn[111][6]));
        let eq17_e560_d_n7: f64 = ((eq17_e556_d_n7 * eq17_e559) + (eq17_e556 * s.dn[111][7]));
        let eq17_e560_d_n8: f64 = ((eq17_e556_d_n8 * eq17_e559) + (eq17_e556 * s.dn[111][8]));
        let eq17_e560_d_n9: f64 = ((eq17_e556_d_n9 * eq17_e559) + (eq17_e556 * s.dn[111][9]));
        let eq17_e560_d_n10: f64 = ((eq17_e556_d_n10 * eq17_e559) + (eq17_e556 * s.dn[111][10]));
        let eq17_e560_d_n11: f64 = ((eq17_e556_d_n11 * eq17_e559) + (eq17_e556 * s.dn[111][11]));
        let eq17_e560_d_n12: f64 = ((eq17_e556_d_n12 * eq17_e559) + (eq17_e556 * s.dn[111][12]));
        let eq17_e560_d_n13: f64 = ((eq17_e556_d_n13 * eq17_e559) + (eq17_e556 * s.dn[111][13]));
        let eq17_e560_d_n14: f64 = ((eq17_e556_d_n14 * eq17_e559) + (eq17_e556 * s.dn[111][14]));
        let eq17_e560_d_n15: f64 = ((eq17_e556_d_n15 * eq17_e559) + (eq17_e556 * s.dn[111][15]));
        let eq17_e560_d_n16: f64 = ((eq17_e556_d_n16 * eq17_e559) + (eq17_e556 * s.dn[111][16]));
        let eq17_e560_d_n17: f64 = ((eq17_e556_d_n17 * eq17_e559) + (eq17_e556 * s.dn[111][17]));
        let eq17_e560_d_n18: f64 = ((eq17_e556_d_n18 * eq17_e559) + (eq17_e556 * s.dn[111][18]));
        let eq17_e560_d_n19: f64 = ((eq17_e556_d_n19 * eq17_e559) + (eq17_e556 * s.dn[111][19]));
        let eq17_e560_d_n20: f64 = ((eq17_e556_d_n20 * eq17_e559) + (eq17_e556 * s.dn[111][20]));
        let eq17_e560_d_n21: f64 = ((eq17_e556_d_n21 * eq17_e559) + (eq17_e556 * s.dn[111][21]));
        let eq17_e560_d_n22: f64 = ((eq17_e556_d_n22 * eq17_e559) + (eq17_e556 * s.dn[111][22]));
        let eq17_e560_d_n23: f64 = ((eq17_e556_d_n23 * eq17_e559) + (eq17_e556 * s.dn[111][23]));
        let eq17_e560_d_n24: f64 = ((eq17_e556_d_n24 * eq17_e559) + (eq17_e556 * s.dn[111][24]));
        let eq17_e560_d_n25: f64 = ((eq17_e556_d_n25 * eq17_e559) + (eq17_e556 * s.dn[111][25]));
        let eq17_e560_d_n26: f64 = ((eq17_e556_d_n26 * eq17_e559) + (eq17_e556 * s.dn[111][26]));
        let eq17_e560_d_n27: f64 = ((eq17_e556_d_n27 * eq17_e559) + (eq17_e556 * s.dn[111][27]));
        let eq17_e560_d_n28: f64 = ((eq17_e556_d_n28 * eq17_e559) + (eq17_e556 * s.dn[111][28]));
        let eq17_e560_d_n29: f64 = ((eq17_e556_d_n29 * eq17_e559) + (eq17_e556 * s.dn[111][29]));
        let eq17_e561: f64 = (eq17_e551 + eq17_e560);
        let eq17_e561_d_n0: f64 = (eq17_e550_d_n0 + eq17_e560_d_n0);
        let eq17_e561_d_n1: f64 = (eq17_e550_d_n1 + eq17_e560_d_n1);
        let eq17_e561_d_n2: f64 = (eq17_e550_d_n2 + eq17_e560_d_n2);
        let eq17_e561_d_n3: f64 = (eq17_e550_d_n3 + eq17_e560_d_n3);
        let eq17_e561_d_n4: f64 = (eq17_e550_d_n4 + eq17_e560_d_n4);
        let eq17_e561_d_n5: f64 = (eq17_e550_d_n5 + eq17_e560_d_n5);
        let eq17_e561_d_n6: f64 = (eq17_e550_d_n6 + eq17_e560_d_n6);
        let eq17_e561_d_n7: f64 = (eq17_e550_d_n7 + eq17_e560_d_n7);
        let eq17_e561_d_n8: f64 = (eq17_e550_d_n8 + eq17_e560_d_n8);
        let eq17_e561_d_n9: f64 = (eq17_e550_d_n9 + eq17_e560_d_n9);
        let eq17_e561_d_n10: f64 = (eq17_e550_d_n10 + eq17_e560_d_n10);
        let eq17_e561_d_n11: f64 = (eq17_e550_d_n11 + eq17_e560_d_n11);
        let eq17_e561_d_n12: f64 = (eq17_e550_d_n12 + eq17_e560_d_n12);
        let eq17_e561_d_n13: f64 = (eq17_e550_d_n13 + eq17_e560_d_n13);
        let eq17_e561_d_n14: f64 = (eq17_e550_d_n14 + eq17_e560_d_n14);
        let eq17_e561_d_n15: f64 = (eq17_e550_d_n15 + eq17_e560_d_n15);
        let eq17_e561_d_n16: f64 = (eq17_e550_d_n16 + eq17_e560_d_n16);
        let eq17_e561_d_n17: f64 = (eq17_e550_d_n17 + eq17_e560_d_n17);
        let eq17_e561_d_n18: f64 = (eq17_e550_d_n18 + eq17_e560_d_n18);
        let eq17_e561_d_n19: f64 = (eq17_e550_d_n19 + eq17_e560_d_n19);
        let eq17_e561_d_n20: f64 = (eq17_e550_d_n20 + eq17_e560_d_n20);
        let eq17_e561_d_n21: f64 = (eq17_e550_d_n21 + eq17_e560_d_n21);
        let eq17_e561_d_n22: f64 = (eq17_e550_d_n22 + eq17_e560_d_n22);
        let eq17_e561_d_n23: f64 = (eq17_e550_d_n23 + eq17_e560_d_n23);
        let eq17_e561_d_n24: f64 = (eq17_e550_d_n24 + eq17_e560_d_n24);
        let eq17_e561_d_n25: f64 = (eq17_e550_d_n25 + eq17_e560_d_n25);
        let eq17_e561_d_n26: f64 = (eq17_e550_d_n26 + eq17_e560_d_n26);
        let eq17_e561_d_n27: f64 = (eq17_e550_d_n27 + eq17_e560_d_n27);
        let eq17_e561_d_n28: f64 = (eq17_e550_d_n28 + eq17_e560_d_n28);
        let eq17_e561_d_n29: f64 = (eq17_e550_d_n29 + eq17_e560_d_n29);
        let eq17_e562: f64 = (eq17_e544 * eq17_e561);
        let eq17_e562_d_n0: f64 = ((eq17_e544_d_n0 * eq17_e561) + (eq17_e544 * eq17_e561_d_n0));
        let eq17_e562_d_n1: f64 = ((eq17_e544_d_n1 * eq17_e561) + (eq17_e544 * eq17_e561_d_n1));
        let eq17_e562_d_n2: f64 = ((eq17_e544_d_n2 * eq17_e561) + (eq17_e544 * eq17_e561_d_n2));
        let eq17_e562_d_n3: f64 = ((eq17_e544_d_n3 * eq17_e561) + (eq17_e544 * eq17_e561_d_n3));
        let eq17_e562_d_n4: f64 = ((eq17_e544_d_n4 * eq17_e561) + (eq17_e544 * eq17_e561_d_n4));
        let eq17_e562_d_n5: f64 = ((eq17_e544_d_n5 * eq17_e561) + (eq17_e544 * eq17_e561_d_n5));
        let eq17_e562_d_n6: f64 = ((eq17_e544_d_n6 * eq17_e561) + (eq17_e544 * eq17_e561_d_n6));
        let eq17_e562_d_n7: f64 = ((eq17_e544_d_n7 * eq17_e561) + (eq17_e544 * eq17_e561_d_n7));
        let eq17_e562_d_n8: f64 = ((eq17_e544_d_n8 * eq17_e561) + (eq17_e544 * eq17_e561_d_n8));
        let eq17_e562_d_n9: f64 = ((eq17_e544_d_n9 * eq17_e561) + (eq17_e544 * eq17_e561_d_n9));
        let eq17_e562_d_n10: f64 = ((eq17_e544_d_n10 * eq17_e561) + (eq17_e544 * eq17_e561_d_n10));
        let eq17_e562_d_n11: f64 = ((eq17_e544_d_n11 * eq17_e561) + (eq17_e544 * eq17_e561_d_n11));
        let eq17_e562_d_n12: f64 = ((eq17_e544_d_n12 * eq17_e561) + (eq17_e544 * eq17_e561_d_n12));
        let eq17_e562_d_n13: f64 = ((eq17_e544_d_n13 * eq17_e561) + (eq17_e544 * eq17_e561_d_n13));
        let eq17_e562_d_n14: f64 = ((eq17_e544_d_n14 * eq17_e561) + (eq17_e544 * eq17_e561_d_n14));
        let eq17_e562_d_n15: f64 = ((eq17_e544_d_n15 * eq17_e561) + (eq17_e544 * eq17_e561_d_n15));
        let eq17_e562_d_n16: f64 = ((eq17_e544_d_n16 * eq17_e561) + (eq17_e544 * eq17_e561_d_n16));
        let eq17_e562_d_n17: f64 = ((eq17_e544_d_n17 * eq17_e561) + (eq17_e544 * eq17_e561_d_n17));
        let eq17_e562_d_n18: f64 = ((eq17_e544_d_n18 * eq17_e561) + (eq17_e544 * eq17_e561_d_n18));
        let eq17_e562_d_n19: f64 = ((eq17_e544_d_n19 * eq17_e561) + (eq17_e544 * eq17_e561_d_n19));
        let eq17_e562_d_n20: f64 = ((eq17_e544_d_n20 * eq17_e561) + (eq17_e544 * eq17_e561_d_n20));
        let eq17_e562_d_n21: f64 = ((eq17_e544_d_n21 * eq17_e561) + (eq17_e544 * eq17_e561_d_n21));
        let eq17_e562_d_n22: f64 = ((eq17_e544_d_n22 * eq17_e561) + (eq17_e544 * eq17_e561_d_n22));
        let eq17_e562_d_n23: f64 = ((eq17_e544_d_n23 * eq17_e561) + (eq17_e544 * eq17_e561_d_n23));
        let eq17_e562_d_n24: f64 = ((eq17_e544_d_n24 * eq17_e561) + (eq17_e544 * eq17_e561_d_n24));
        let eq17_e562_d_n25: f64 = ((eq17_e544_d_n25 * eq17_e561) + (eq17_e544 * eq17_e561_d_n25));
        let eq17_e562_d_n26: f64 = ((eq17_e544_d_n26 * eq17_e561) + (eq17_e544 * eq17_e561_d_n26));
        let eq17_e562_d_n27: f64 = ((eq17_e544_d_n27 * eq17_e561) + (eq17_e544 * eq17_e561_d_n27));
        let eq17_e562_d_n28: f64 = ((eq17_e544_d_n28 * eq17_e561) + (eq17_e544 * eq17_e561_d_n28));
        let eq17_e562_d_n29: f64 = ((eq17_e544_d_n29 * eq17_e561) + (eq17_e544 * eq17_e561_d_n29));
        let eq17_e562_q: f64 = (eq17_e544_q * eq17_e561);
        let eq17_e562_q_d_n0: f64 = ((eq17_e544_q_d_n0 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n0));
        let eq17_e562_q_d_n1: f64 = ((eq17_e544_q_d_n1 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n1));
        let eq17_e562_q_d_n2: f64 = ((eq17_e544_q_d_n2 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n2));
        let eq17_e562_q_d_n3: f64 = ((eq17_e544_q_d_n3 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n3));
        let eq17_e562_q_d_n4: f64 = ((eq17_e544_q_d_n4 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n4));
        let eq17_e562_q_d_n5: f64 = ((eq17_e544_q_d_n5 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n5));
        let eq17_e562_q_d_n6: f64 = ((eq17_e544_q_d_n6 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n6));
        let eq17_e562_q_d_n7: f64 = ((eq17_e544_q_d_n7 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n7));
        let eq17_e562_q_d_n8: f64 = ((eq17_e544_q_d_n8 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n8));
        let eq17_e562_q_d_n9: f64 = ((eq17_e544_q_d_n9 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n9));
        let eq17_e562_q_d_n10: f64 = ((eq17_e544_q_d_n10 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n10));
        let eq17_e562_q_d_n11: f64 = ((eq17_e544_q_d_n11 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n11));
        let eq17_e562_q_d_n12: f64 = ((eq17_e544_q_d_n12 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n12));
        let eq17_e562_q_d_n13: f64 = ((eq17_e544_q_d_n13 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n13));
        let eq17_e562_q_d_n14: f64 = ((eq17_e544_q_d_n14 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n14));
        let eq17_e562_q_d_n15: f64 = ((eq17_e544_q_d_n15 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n15));
        let eq17_e562_q_d_n16: f64 = ((eq17_e544_q_d_n16 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n16));
        let eq17_e562_q_d_n17: f64 = ((eq17_e544_q_d_n17 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n17));
        let eq17_e562_q_d_n18: f64 = ((eq17_e544_q_d_n18 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n18));
        let eq17_e562_q_d_n19: f64 = ((eq17_e544_q_d_n19 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n19));
        let eq17_e562_q_d_n20: f64 = ((eq17_e544_q_d_n20 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n20));
        let eq17_e562_q_d_n21: f64 = ((eq17_e544_q_d_n21 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n21));
        let eq17_e562_q_d_n22: f64 = ((eq17_e544_q_d_n22 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n22));
        let eq17_e562_q_d_n23: f64 = ((eq17_e544_q_d_n23 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n23));
        let eq17_e562_q_d_n24: f64 = ((eq17_e544_q_d_n24 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n24));
        let eq17_e562_q_d_n25: f64 = ((eq17_e544_q_d_n25 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n25));
        let eq17_e562_q_d_n26: f64 = ((eq17_e544_q_d_n26 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n26));
        let eq17_e562_q_d_n27: f64 = ((eq17_e544_q_d_n27 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n27));
        let eq17_e562_q_d_n28: f64 = ((eq17_e544_q_d_n28 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n28));
        let eq17_e562_q_d_n29: f64 = ((eq17_e544_q_d_n29 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n29));
        (eq17_e562, eq17_e562_d_n0, eq17_e562_d_n1, eq17_e562_d_n2, eq17_e562_d_n3, eq17_e562_d_n4, eq17_e562_d_n5, eq17_e562_d_n6, eq17_e562_d_n7, eq17_e562_d_n8, eq17_e562_d_n9, eq17_e562_d_n10, eq17_e562_d_n11, eq17_e562_d_n12, eq17_e562_d_n13, eq17_e562_d_n14, eq17_e562_d_n15, eq17_e562_d_n16, eq17_e562_d_n17, eq17_e562_d_n18, eq17_e562_d_n19, eq17_e562_d_n20, eq17_e562_d_n21, eq17_e562_d_n22, eq17_e562_d_n23, eq17_e562_d_n24, eq17_e562_d_n25, eq17_e562_d_n26, eq17_e562_d_n27, eq17_e562_d_n28, eq17_e562_d_n29, eq17_e562_q, eq17_e562_q_d_n0, eq17_e562_q_d_n1, eq17_e562_q_d_n2, eq17_e562_q_d_n3, eq17_e562_q_d_n4, eq17_e562_q_d_n5, eq17_e562_q_d_n6, eq17_e562_q_d_n7, eq17_e562_q_d_n8, eq17_e562_q_d_n9, eq17_e562_q_d_n10, eq17_e562_q_d_n11, eq17_e562_q_d_n12, eq17_e562_q_d_n13, eq17_e562_q_d_n14, eq17_e562_q_d_n15, eq17_e562_q_d_n16, eq17_e562_q_d_n17, eq17_e562_q_d_n18, eq17_e562_q_d_n19, eq17_e562_q_d_n20, eq17_e562_q_d_n21, eq17_e562_q_d_n22, eq17_e562_q_d_n23, eq17_e562_q_d_n24, eq17_e562_q_d_n25, eq17_e562_q_d_n26, eq17_e562_q_d_n27, eq17_e562_q_d_n28, eq17_e562_q_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_reactive_node_derivatives: [f64; 30] = [eq17_e564_q_d_n0, eq17_e564_q_d_n1, eq17_e564_q_d_n2, eq17_e564_q_d_n3, eq17_e564_q_d_n4, eq17_e564_q_d_n5, eq17_e564_q_d_n6, eq17_e564_q_d_n7, eq17_e564_q_d_n8, eq17_e564_q_d_n9, eq17_e564_q_d_n10, eq17_e564_q_d_n11, eq17_e564_q_d_n12, eq17_e564_q_d_n13, eq17_e564_q_d_n14, eq17_e564_q_d_n15, eq17_e564_q_d_n16, eq17_e564_q_d_n17, eq17_e564_q_d_n18, eq17_e564_q_d_n19, eq17_e564_q_d_n20, eq17_e564_q_d_n21, eq17_e564_q_d_n22, eq17_e564_q_d_n23, eq17_e564_q_d_n24, eq17_e564_q_d_n25, eq17_e564_q_d_n26, eq17_e564_q_d_n27, eq17_e564_q_d_n28, eq17_e564_q_d_n29];
        let eq17_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[23]),
            None,
            &nodes,
            &eq17_reactive_node_derivatives,
            &branches,
            &eq17_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq22_e682, eq22_e682_d_n0, eq22_e682_d_n1, eq22_e682_d_n2, eq22_e682_d_n3, eq22_e682_d_n4, eq22_e682_d_n5, eq22_e682_d_n6, eq22_e682_d_n7, eq22_e682_d_n8, eq22_e682_d_n9, eq22_e682_d_n10, eq22_e682_d_n11, eq22_e682_d_n12, eq22_e682_d_n13, eq22_e682_d_n14, eq22_e682_d_n15, eq22_e682_d_n16, eq22_e682_d_n17, eq22_e682_d_n18, eq22_e682_d_n19, eq22_e682_d_n20, eq22_e682_d_n21, eq22_e682_d_n22, eq22_e682_d_n23, eq22_e682_d_n24, eq22_e682_d_n25, eq22_e682_d_n26, eq22_e682_d_n27, eq22_e682_d_n28, eq22_e682_d_n29, eq22_e682_q, eq22_e682_q_d_n0, eq22_e682_q_d_n1, eq22_e682_q_d_n2, eq22_e682_q_d_n3, eq22_e682_q_d_n4, eq22_e682_q_d_n5, eq22_e682_q_d_n6, eq22_e682_q_d_n7, eq22_e682_q_d_n8, eq22_e682_q_d_n9, eq22_e682_q_d_n10, eq22_e682_q_d_n11, eq22_e682_q_d_n12, eq22_e682_q_d_n13, eq22_e682_q_d_n14, eq22_e682_q_d_n15, eq22_e682_q_d_n16, eq22_e682_q_d_n17, eq22_e682_q_d_n18, eq22_e682_q_d_n19, eq22_e682_q_d_n20, eq22_e682_q_d_n21, eq22_e682_q_d_n22, eq22_e682_q_d_n23, eq22_e682_q_d_n24, eq22_e682_q_d_n25, eq22_e682_q_d_n26, eq22_e682_q_d_n27, eq22_e682_q_d_n28, eq22_e682_q_d_n29,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq22_e661_q: f64 = s.v[227];
        let eq22_e662: f64 = (p.p341 * s.v[227]);
        let eq22_e662_d_n0: f64 = (p.p341 * s.dn[227][0]);
        let eq22_e662_d_n1: f64 = (p.p341 * s.dn[227][1]);
        let eq22_e662_d_n2: f64 = (p.p341 * s.dn[227][2]);
        let eq22_e662_d_n3: f64 = (p.p341 * s.dn[227][3]);
        let eq22_e662_d_n4: f64 = (p.p341 * s.dn[227][4]);
        let eq22_e662_d_n5: f64 = (p.p341 * s.dn[227][5]);
        let eq22_e662_d_n6: f64 = (p.p341 * s.dn[227][6]);
        let eq22_e662_d_n7: f64 = (p.p341 * s.dn[227][7]);
        let eq22_e662_d_n8: f64 = (p.p341 * s.dn[227][8]);
        let eq22_e662_d_n9: f64 = (p.p341 * s.dn[227][9]);
        let eq22_e662_d_n10: f64 = (p.p341 * s.dn[227][10]);
        let eq22_e662_d_n11: f64 = (p.p341 * s.dn[227][11]);
        let eq22_e662_d_n12: f64 = (p.p341 * s.dn[227][12]);
        let eq22_e662_d_n13: f64 = (p.p341 * s.dn[227][13]);
        let eq22_e662_d_n14: f64 = (p.p341 * s.dn[227][14]);
        let eq22_e662_d_n15: f64 = (p.p341 * s.dn[227][15]);
        let eq22_e662_d_n16: f64 = (p.p341 * s.dn[227][16]);
        let eq22_e662_d_n17: f64 = (p.p341 * s.dn[227][17]);
        let eq22_e662_d_n18: f64 = (p.p341 * s.dn[227][18]);
        let eq22_e662_d_n19: f64 = (p.p341 * s.dn[227][19]);
        let eq22_e662_d_n20: f64 = (p.p341 * s.dn[227][20]);
        let eq22_e662_d_n21: f64 = (p.p341 * s.dn[227][21]);
        let eq22_e662_d_n22: f64 = (p.p341 * s.dn[227][22]);
        let eq22_e662_d_n23: f64 = (p.p341 * s.dn[227][23]);
        let eq22_e662_d_n24: f64 = (p.p341 * s.dn[227][24]);
        let eq22_e662_d_n25: f64 = (p.p341 * s.dn[227][25]);
        let eq22_e662_d_n26: f64 = (p.p341 * s.dn[227][26]);
        let eq22_e662_d_n27: f64 = (p.p341 * s.dn[227][27]);
        let eq22_e662_d_n28: f64 = (p.p341 * s.dn[227][28]);
        let eq22_e662_d_n29: f64 = (p.p341 * s.dn[227][29]);
        let eq22_e662_q: f64 = (p.p341 * eq22_e661_q);
        let eq22_e662_q_d_n0: f64 = (p.p341 * s.dn[227][0]);
        let eq22_e662_q_d_n1: f64 = (p.p341 * s.dn[227][1]);
        let eq22_e662_q_d_n2: f64 = (p.p341 * s.dn[227][2]);
        let eq22_e662_q_d_n3: f64 = (p.p341 * s.dn[227][3]);
        let eq22_e662_q_d_n4: f64 = (p.p341 * s.dn[227][4]);
        let eq22_e662_q_d_n5: f64 = (p.p341 * s.dn[227][5]);
        let eq22_e662_q_d_n6: f64 = (p.p341 * s.dn[227][6]);
        let eq22_e662_q_d_n7: f64 = (p.p341 * s.dn[227][7]);
        let eq22_e662_q_d_n8: f64 = (p.p341 * s.dn[227][8]);
        let eq22_e662_q_d_n9: f64 = (p.p341 * s.dn[227][9]);
        let eq22_e662_q_d_n10: f64 = (p.p341 * s.dn[227][10]);
        let eq22_e662_q_d_n11: f64 = (p.p341 * s.dn[227][11]);
        let eq22_e662_q_d_n12: f64 = (p.p341 * s.dn[227][12]);
        let eq22_e662_q_d_n13: f64 = (p.p341 * s.dn[227][13]);
        let eq22_e662_q_d_n14: f64 = (p.p341 * s.dn[227][14]);
        let eq22_e662_q_d_n15: f64 = (p.p341 * s.dn[227][15]);
        let eq22_e662_q_d_n16: f64 = (p.p341 * s.dn[227][16]);
        let eq22_e662_q_d_n17: f64 = (p.p341 * s.dn[227][17]);
        let eq22_e662_q_d_n18: f64 = (p.p341 * s.dn[227][18]);
        let eq22_e662_q_d_n19: f64 = (p.p341 * s.dn[227][19]);
        let eq22_e662_q_d_n20: f64 = (p.p341 * s.dn[227][20]);
        let eq22_e662_q_d_n21: f64 = (p.p341 * s.dn[227][21]);
        let eq22_e662_q_d_n22: f64 = (p.p341 * s.dn[227][22]);
        let eq22_e662_q_d_n23: f64 = (p.p341 * s.dn[227][23]);
        let eq22_e662_q_d_n24: f64 = (p.p341 * s.dn[227][24]);
        let eq22_e662_q_d_n25: f64 = (p.p341 * s.dn[227][25]);
        let eq22_e662_q_d_n26: f64 = (p.p341 * s.dn[227][26]);
        let eq22_e662_q_d_n27: f64 = (p.p341 * s.dn[227][27]);
        let eq22_e662_q_d_n28: f64 = (p.p341 * s.dn[227][28]);
        let eq22_e662_q_d_n29: f64 = (p.p341 * s.dn[227][29]);
        let eq22_e667: f64 = (s.v[111] - s.v[109]);
        let eq22_e668: f64 = (p.p343 * eq22_e667);
        let eq22_e668_d_n0: f64 = (p.p343 * s.dn[111][0]);
        let eq22_e668_d_n1: f64 = (p.p343 * s.dn[111][1]);
        let eq22_e668_d_n2: f64 = (p.p343 * s.dn[111][2]);
        let eq22_e668_d_n3: f64 = (p.p343 * s.dn[111][3]);
        let eq22_e668_d_n4: f64 = (p.p343 * s.dn[111][4]);
        let eq22_e668_d_n5: f64 = (p.p343 * s.dn[111][5]);
        let eq22_e668_d_n6: f64 = (p.p343 * s.dn[111][6]);
        let eq22_e668_d_n7: f64 = (p.p343 * s.dn[111][7]);
        let eq22_e668_d_n8: f64 = (p.p343 * s.dn[111][8]);
        let eq22_e668_d_n9: f64 = (p.p343 * s.dn[111][9]);
        let eq22_e668_d_n10: f64 = (p.p343 * s.dn[111][10]);
        let eq22_e668_d_n11: f64 = (p.p343 * s.dn[111][11]);
        let eq22_e668_d_n12: f64 = (p.p343 * s.dn[111][12]);
        let eq22_e668_d_n13: f64 = (p.p343 * s.dn[111][13]);
        let eq22_e668_d_n14: f64 = (p.p343 * s.dn[111][14]);
        let eq22_e668_d_n15: f64 = (p.p343 * s.dn[111][15]);
        let eq22_e668_d_n16: f64 = (p.p343 * s.dn[111][16]);
        let eq22_e668_d_n17: f64 = (p.p343 * s.dn[111][17]);
        let eq22_e668_d_n18: f64 = (p.p343 * s.dn[111][18]);
        let eq22_e668_d_n19: f64 = (p.p343 * s.dn[111][19]);
        let eq22_e668_d_n20: f64 = (p.p343 * s.dn[111][20]);
        let eq22_e668_d_n21: f64 = (p.p343 * s.dn[111][21]);
        let eq22_e668_d_n22: f64 = (p.p343 * s.dn[111][22]);
        let eq22_e668_d_n23: f64 = (p.p343 * s.dn[111][23]);
        let eq22_e668_d_n24: f64 = (p.p343 * s.dn[111][24]);
        let eq22_e668_d_n25: f64 = (p.p343 * s.dn[111][25]);
        let eq22_e668_d_n26: f64 = (p.p343 * s.dn[111][26]);
        let eq22_e668_d_n27: f64 = (p.p343 * s.dn[111][27]);
        let eq22_e668_d_n28: f64 = (p.p343 * s.dn[111][28]);
        let eq22_e668_d_n29: f64 = (p.p343 * s.dn[111][29]);
        let eq22_e669: f64 = (1.0 + eq22_e668);
        let eq22_e673: f64 = (s.v[111] - s.v[109]);
        let eq22_e674: f64 = (p.p345 * eq22_e673);
        let eq22_e674_d_n0: f64 = (p.p345 * s.dn[111][0]);
        let eq22_e674_d_n1: f64 = (p.p345 * s.dn[111][1]);
        let eq22_e674_d_n2: f64 = (p.p345 * s.dn[111][2]);
        let eq22_e674_d_n3: f64 = (p.p345 * s.dn[111][3]);
        let eq22_e674_d_n4: f64 = (p.p345 * s.dn[111][4]);
        let eq22_e674_d_n5: f64 = (p.p345 * s.dn[111][5]);
        let eq22_e674_d_n6: f64 = (p.p345 * s.dn[111][6]);
        let eq22_e674_d_n7: f64 = (p.p345 * s.dn[111][7]);
        let eq22_e674_d_n8: f64 = (p.p345 * s.dn[111][8]);
        let eq22_e674_d_n9: f64 = (p.p345 * s.dn[111][9]);
        let eq22_e674_d_n10: f64 = (p.p345 * s.dn[111][10]);
        let eq22_e674_d_n11: f64 = (p.p345 * s.dn[111][11]);
        let eq22_e674_d_n12: f64 = (p.p345 * s.dn[111][12]);
        let eq22_e674_d_n13: f64 = (p.p345 * s.dn[111][13]);
        let eq22_e674_d_n14: f64 = (p.p345 * s.dn[111][14]);
        let eq22_e674_d_n15: f64 = (p.p345 * s.dn[111][15]);
        let eq22_e674_d_n16: f64 = (p.p345 * s.dn[111][16]);
        let eq22_e674_d_n17: f64 = (p.p345 * s.dn[111][17]);
        let eq22_e674_d_n18: f64 = (p.p345 * s.dn[111][18]);
        let eq22_e674_d_n19: f64 = (p.p345 * s.dn[111][19]);
        let eq22_e674_d_n20: f64 = (p.p345 * s.dn[111][20]);
        let eq22_e674_d_n21: f64 = (p.p345 * s.dn[111][21]);
        let eq22_e674_d_n22: f64 = (p.p345 * s.dn[111][22]);
        let eq22_e674_d_n23: f64 = (p.p345 * s.dn[111][23]);
        let eq22_e674_d_n24: f64 = (p.p345 * s.dn[111][24]);
        let eq22_e674_d_n25: f64 = (p.p345 * s.dn[111][25]);
        let eq22_e674_d_n26: f64 = (p.p345 * s.dn[111][26]);
        let eq22_e674_d_n27: f64 = (p.p345 * s.dn[111][27]);
        let eq22_e674_d_n28: f64 = (p.p345 * s.dn[111][28]);
        let eq22_e674_d_n29: f64 = (p.p345 * s.dn[111][29]);
        let eq22_e677: f64 = (s.v[111] - s.v[109]);
        let eq22_e678: f64 = (eq22_e674 * eq22_e677);
        let eq22_e678_d_n0: f64 = ((eq22_e674_d_n0 * eq22_e677) + (eq22_e674 * s.dn[111][0]));
        let eq22_e678_d_n1: f64 = ((eq22_e674_d_n1 * eq22_e677) + (eq22_e674 * s.dn[111][1]));
        let eq22_e678_d_n2: f64 = ((eq22_e674_d_n2 * eq22_e677) + (eq22_e674 * s.dn[111][2]));
        let eq22_e678_d_n3: f64 = ((eq22_e674_d_n3 * eq22_e677) + (eq22_e674 * s.dn[111][3]));
        let eq22_e678_d_n4: f64 = ((eq22_e674_d_n4 * eq22_e677) + (eq22_e674 * s.dn[111][4]));
        let eq22_e678_d_n5: f64 = ((eq22_e674_d_n5 * eq22_e677) + (eq22_e674 * s.dn[111][5]));
        let eq22_e678_d_n6: f64 = ((eq22_e674_d_n6 * eq22_e677) + (eq22_e674 * s.dn[111][6]));
        let eq22_e678_d_n7: f64 = ((eq22_e674_d_n7 * eq22_e677) + (eq22_e674 * s.dn[111][7]));
        let eq22_e678_d_n8: f64 = ((eq22_e674_d_n8 * eq22_e677) + (eq22_e674 * s.dn[111][8]));
        let eq22_e678_d_n9: f64 = ((eq22_e674_d_n9 * eq22_e677) + (eq22_e674 * s.dn[111][9]));
        let eq22_e678_d_n10: f64 = ((eq22_e674_d_n10 * eq22_e677) + (eq22_e674 * s.dn[111][10]));
        let eq22_e678_d_n11: f64 = ((eq22_e674_d_n11 * eq22_e677) + (eq22_e674 * s.dn[111][11]));
        let eq22_e678_d_n12: f64 = ((eq22_e674_d_n12 * eq22_e677) + (eq22_e674 * s.dn[111][12]));
        let eq22_e678_d_n13: f64 = ((eq22_e674_d_n13 * eq22_e677) + (eq22_e674 * s.dn[111][13]));
        let eq22_e678_d_n14: f64 = ((eq22_e674_d_n14 * eq22_e677) + (eq22_e674 * s.dn[111][14]));
        let eq22_e678_d_n15: f64 = ((eq22_e674_d_n15 * eq22_e677) + (eq22_e674 * s.dn[111][15]));
        let eq22_e678_d_n16: f64 = ((eq22_e674_d_n16 * eq22_e677) + (eq22_e674 * s.dn[111][16]));
        let eq22_e678_d_n17: f64 = ((eq22_e674_d_n17 * eq22_e677) + (eq22_e674 * s.dn[111][17]));
        let eq22_e678_d_n18: f64 = ((eq22_e674_d_n18 * eq22_e677) + (eq22_e674 * s.dn[111][18]));
        let eq22_e678_d_n19: f64 = ((eq22_e674_d_n19 * eq22_e677) + (eq22_e674 * s.dn[111][19]));
        let eq22_e678_d_n20: f64 = ((eq22_e674_d_n20 * eq22_e677) + (eq22_e674 * s.dn[111][20]));
        let eq22_e678_d_n21: f64 = ((eq22_e674_d_n21 * eq22_e677) + (eq22_e674 * s.dn[111][21]));
        let eq22_e678_d_n22: f64 = ((eq22_e674_d_n22 * eq22_e677) + (eq22_e674 * s.dn[111][22]));
        let eq22_e678_d_n23: f64 = ((eq22_e674_d_n23 * eq22_e677) + (eq22_e674 * s.dn[111][23]));
        let eq22_e678_d_n24: f64 = ((eq22_e674_d_n24 * eq22_e677) + (eq22_e674 * s.dn[111][24]));
        let eq22_e678_d_n25: f64 = ((eq22_e674_d_n25 * eq22_e677) + (eq22_e674 * s.dn[111][25]));
        let eq22_e678_d_n26: f64 = ((eq22_e674_d_n26 * eq22_e677) + (eq22_e674 * s.dn[111][26]));
        let eq22_e678_d_n27: f64 = ((eq22_e674_d_n27 * eq22_e677) + (eq22_e674 * s.dn[111][27]));
        let eq22_e678_d_n28: f64 = ((eq22_e674_d_n28 * eq22_e677) + (eq22_e674 * s.dn[111][28]));
        let eq22_e678_d_n29: f64 = ((eq22_e674_d_n29 * eq22_e677) + (eq22_e674 * s.dn[111][29]));
        let eq22_e679: f64 = (eq22_e669 + eq22_e678);
        let eq22_e679_d_n0: f64 = (eq22_e668_d_n0 + eq22_e678_d_n0);
        let eq22_e679_d_n1: f64 = (eq22_e668_d_n1 + eq22_e678_d_n1);
        let eq22_e679_d_n2: f64 = (eq22_e668_d_n2 + eq22_e678_d_n2);
        let eq22_e679_d_n3: f64 = (eq22_e668_d_n3 + eq22_e678_d_n3);
        let eq22_e679_d_n4: f64 = (eq22_e668_d_n4 + eq22_e678_d_n4);
        let eq22_e679_d_n5: f64 = (eq22_e668_d_n5 + eq22_e678_d_n5);
        let eq22_e679_d_n6: f64 = (eq22_e668_d_n6 + eq22_e678_d_n6);
        let eq22_e679_d_n7: f64 = (eq22_e668_d_n7 + eq22_e678_d_n7);
        let eq22_e679_d_n8: f64 = (eq22_e668_d_n8 + eq22_e678_d_n8);
        let eq22_e679_d_n9: f64 = (eq22_e668_d_n9 + eq22_e678_d_n9);
        let eq22_e679_d_n10: f64 = (eq22_e668_d_n10 + eq22_e678_d_n10);
        let eq22_e679_d_n11: f64 = (eq22_e668_d_n11 + eq22_e678_d_n11);
        let eq22_e679_d_n12: f64 = (eq22_e668_d_n12 + eq22_e678_d_n12);
        let eq22_e679_d_n13: f64 = (eq22_e668_d_n13 + eq22_e678_d_n13);
        let eq22_e679_d_n14: f64 = (eq22_e668_d_n14 + eq22_e678_d_n14);
        let eq22_e679_d_n15: f64 = (eq22_e668_d_n15 + eq22_e678_d_n15);
        let eq22_e679_d_n16: f64 = (eq22_e668_d_n16 + eq22_e678_d_n16);
        let eq22_e679_d_n17: f64 = (eq22_e668_d_n17 + eq22_e678_d_n17);
        let eq22_e679_d_n18: f64 = (eq22_e668_d_n18 + eq22_e678_d_n18);
        let eq22_e679_d_n19: f64 = (eq22_e668_d_n19 + eq22_e678_d_n19);
        let eq22_e679_d_n20: f64 = (eq22_e668_d_n20 + eq22_e678_d_n20);
        let eq22_e679_d_n21: f64 = (eq22_e668_d_n21 + eq22_e678_d_n21);
        let eq22_e679_d_n22: f64 = (eq22_e668_d_n22 + eq22_e678_d_n22);
        let eq22_e679_d_n23: f64 = (eq22_e668_d_n23 + eq22_e678_d_n23);
        let eq22_e679_d_n24: f64 = (eq22_e668_d_n24 + eq22_e678_d_n24);
        let eq22_e679_d_n25: f64 = (eq22_e668_d_n25 + eq22_e678_d_n25);
        let eq22_e679_d_n26: f64 = (eq22_e668_d_n26 + eq22_e678_d_n26);
        let eq22_e679_d_n27: f64 = (eq22_e668_d_n27 + eq22_e678_d_n27);
        let eq22_e679_d_n28: f64 = (eq22_e668_d_n28 + eq22_e678_d_n28);
        let eq22_e679_d_n29: f64 = (eq22_e668_d_n29 + eq22_e678_d_n29);
        let eq22_e680: f64 = (eq22_e662 * eq22_e679);
        let eq22_e680_d_n0: f64 = ((eq22_e662_d_n0 * eq22_e679) + (eq22_e662 * eq22_e679_d_n0));
        let eq22_e680_d_n1: f64 = ((eq22_e662_d_n1 * eq22_e679) + (eq22_e662 * eq22_e679_d_n1));
        let eq22_e680_d_n2: f64 = ((eq22_e662_d_n2 * eq22_e679) + (eq22_e662 * eq22_e679_d_n2));
        let eq22_e680_d_n3: f64 = ((eq22_e662_d_n3 * eq22_e679) + (eq22_e662 * eq22_e679_d_n3));
        let eq22_e680_d_n4: f64 = ((eq22_e662_d_n4 * eq22_e679) + (eq22_e662 * eq22_e679_d_n4));
        let eq22_e680_d_n5: f64 = ((eq22_e662_d_n5 * eq22_e679) + (eq22_e662 * eq22_e679_d_n5));
        let eq22_e680_d_n6: f64 = ((eq22_e662_d_n6 * eq22_e679) + (eq22_e662 * eq22_e679_d_n6));
        let eq22_e680_d_n7: f64 = ((eq22_e662_d_n7 * eq22_e679) + (eq22_e662 * eq22_e679_d_n7));
        let eq22_e680_d_n8: f64 = ((eq22_e662_d_n8 * eq22_e679) + (eq22_e662 * eq22_e679_d_n8));
        let eq22_e680_d_n9: f64 = ((eq22_e662_d_n9 * eq22_e679) + (eq22_e662 * eq22_e679_d_n9));
        let eq22_e680_d_n10: f64 = ((eq22_e662_d_n10 * eq22_e679) + (eq22_e662 * eq22_e679_d_n10));
        let eq22_e680_d_n11: f64 = ((eq22_e662_d_n11 * eq22_e679) + (eq22_e662 * eq22_e679_d_n11));
        let eq22_e680_d_n12: f64 = ((eq22_e662_d_n12 * eq22_e679) + (eq22_e662 * eq22_e679_d_n12));
        let eq22_e680_d_n13: f64 = ((eq22_e662_d_n13 * eq22_e679) + (eq22_e662 * eq22_e679_d_n13));
        let eq22_e680_d_n14: f64 = ((eq22_e662_d_n14 * eq22_e679) + (eq22_e662 * eq22_e679_d_n14));
        let eq22_e680_d_n15: f64 = ((eq22_e662_d_n15 * eq22_e679) + (eq22_e662 * eq22_e679_d_n15));
        let eq22_e680_d_n16: f64 = ((eq22_e662_d_n16 * eq22_e679) + (eq22_e662 * eq22_e679_d_n16));
        let eq22_e680_d_n17: f64 = ((eq22_e662_d_n17 * eq22_e679) + (eq22_e662 * eq22_e679_d_n17));
        let eq22_e680_d_n18: f64 = ((eq22_e662_d_n18 * eq22_e679) + (eq22_e662 * eq22_e679_d_n18));
        let eq22_e680_d_n19: f64 = ((eq22_e662_d_n19 * eq22_e679) + (eq22_e662 * eq22_e679_d_n19));
        let eq22_e680_d_n20: f64 = ((eq22_e662_d_n20 * eq22_e679) + (eq22_e662 * eq22_e679_d_n20));
        let eq22_e680_d_n21: f64 = ((eq22_e662_d_n21 * eq22_e679) + (eq22_e662 * eq22_e679_d_n21));
        let eq22_e680_d_n22: f64 = ((eq22_e662_d_n22 * eq22_e679) + (eq22_e662 * eq22_e679_d_n22));
        let eq22_e680_d_n23: f64 = ((eq22_e662_d_n23 * eq22_e679) + (eq22_e662 * eq22_e679_d_n23));
        let eq22_e680_d_n24: f64 = ((eq22_e662_d_n24 * eq22_e679) + (eq22_e662 * eq22_e679_d_n24));
        let eq22_e680_d_n25: f64 = ((eq22_e662_d_n25 * eq22_e679) + (eq22_e662 * eq22_e679_d_n25));
        let eq22_e680_d_n26: f64 = ((eq22_e662_d_n26 * eq22_e679) + (eq22_e662 * eq22_e679_d_n26));
        let eq22_e680_d_n27: f64 = ((eq22_e662_d_n27 * eq22_e679) + (eq22_e662 * eq22_e679_d_n27));
        let eq22_e680_d_n28: f64 = ((eq22_e662_d_n28 * eq22_e679) + (eq22_e662 * eq22_e679_d_n28));
        let eq22_e680_d_n29: f64 = ((eq22_e662_d_n29 * eq22_e679) + (eq22_e662 * eq22_e679_d_n29));
        let eq22_e680_q: f64 = (eq22_e662_q * eq22_e679);
        let eq22_e680_q_d_n0: f64 = ((eq22_e662_q_d_n0 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n0));
        let eq22_e680_q_d_n1: f64 = ((eq22_e662_q_d_n1 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n1));
        let eq22_e680_q_d_n2: f64 = ((eq22_e662_q_d_n2 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n2));
        let eq22_e680_q_d_n3: f64 = ((eq22_e662_q_d_n3 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n3));
        let eq22_e680_q_d_n4: f64 = ((eq22_e662_q_d_n4 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n4));
        let eq22_e680_q_d_n5: f64 = ((eq22_e662_q_d_n5 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n5));
        let eq22_e680_q_d_n6: f64 = ((eq22_e662_q_d_n6 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n6));
        let eq22_e680_q_d_n7: f64 = ((eq22_e662_q_d_n7 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n7));
        let eq22_e680_q_d_n8: f64 = ((eq22_e662_q_d_n8 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n8));
        let eq22_e680_q_d_n9: f64 = ((eq22_e662_q_d_n9 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n9));
        let eq22_e680_q_d_n10: f64 = ((eq22_e662_q_d_n10 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n10));
        let eq22_e680_q_d_n11: f64 = ((eq22_e662_q_d_n11 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n11));
        let eq22_e680_q_d_n12: f64 = ((eq22_e662_q_d_n12 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n12));
        let eq22_e680_q_d_n13: f64 = ((eq22_e662_q_d_n13 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n13));
        let eq22_e680_q_d_n14: f64 = ((eq22_e662_q_d_n14 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n14));
        let eq22_e680_q_d_n15: f64 = ((eq22_e662_q_d_n15 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n15));
        let eq22_e680_q_d_n16: f64 = ((eq22_e662_q_d_n16 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n16));
        let eq22_e680_q_d_n17: f64 = ((eq22_e662_q_d_n17 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n17));
        let eq22_e680_q_d_n18: f64 = ((eq22_e662_q_d_n18 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n18));
        let eq22_e680_q_d_n19: f64 = ((eq22_e662_q_d_n19 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n19));
        let eq22_e680_q_d_n20: f64 = ((eq22_e662_q_d_n20 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n20));
        let eq22_e680_q_d_n21: f64 = ((eq22_e662_q_d_n21 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n21));
        let eq22_e680_q_d_n22: f64 = ((eq22_e662_q_d_n22 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n22));
        let eq22_e680_q_d_n23: f64 = ((eq22_e662_q_d_n23 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n23));
        let eq22_e680_q_d_n24: f64 = ((eq22_e662_q_d_n24 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n24));
        let eq22_e680_q_d_n25: f64 = ((eq22_e662_q_d_n25 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n25));
        let eq22_e680_q_d_n26: f64 = ((eq22_e662_q_d_n26 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n26));
        let eq22_e680_q_d_n27: f64 = ((eq22_e662_q_d_n27 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n27));
        let eq22_e680_q_d_n28: f64 = ((eq22_e662_q_d_n28 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n28));
        let eq22_e680_q_d_n29: f64 = ((eq22_e662_q_d_n29 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n29));
        (eq22_e680, eq22_e680_d_n0, eq22_e680_d_n1, eq22_e680_d_n2, eq22_e680_d_n3, eq22_e680_d_n4, eq22_e680_d_n5, eq22_e680_d_n6, eq22_e680_d_n7, eq22_e680_d_n8, eq22_e680_d_n9, eq22_e680_d_n10, eq22_e680_d_n11, eq22_e680_d_n12, eq22_e680_d_n13, eq22_e680_d_n14, eq22_e680_d_n15, eq22_e680_d_n16, eq22_e680_d_n17, eq22_e680_d_n18, eq22_e680_d_n19, eq22_e680_d_n20, eq22_e680_d_n21, eq22_e680_d_n22, eq22_e680_d_n23, eq22_e680_d_n24, eq22_e680_d_n25, eq22_e680_d_n26, eq22_e680_d_n27, eq22_e680_d_n28, eq22_e680_d_n29, eq22_e680_q, eq22_e680_q_d_n0, eq22_e680_q_d_n1, eq22_e680_q_d_n2, eq22_e680_q_d_n3, eq22_e680_q_d_n4, eq22_e680_q_d_n5, eq22_e680_q_d_n6, eq22_e680_q_d_n7, eq22_e680_q_d_n8, eq22_e680_q_d_n9, eq22_e680_q_d_n10, eq22_e680_q_d_n11, eq22_e680_q_d_n12, eq22_e680_q_d_n13, eq22_e680_q_d_n14, eq22_e680_q_d_n15, eq22_e680_q_d_n16, eq22_e680_q_d_n17, eq22_e680_q_d_n18, eq22_e680_q_d_n19, eq22_e680_q_d_n20, eq22_e680_q_d_n21, eq22_e680_q_d_n22, eq22_e680_q_d_n23, eq22_e680_q_d_n24, eq22_e680_q_d_n25, eq22_e680_q_d_n26, eq22_e680_q_d_n27, eq22_e680_q_d_n28, eq22_e680_q_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_reactive_node_derivatives: [f64; 30] = [eq22_e682_q_d_n0, eq22_e682_q_d_n1, eq22_e682_q_d_n2, eq22_e682_q_d_n3, eq22_e682_q_d_n4, eq22_e682_q_d_n5, eq22_e682_q_d_n6, eq22_e682_q_d_n7, eq22_e682_q_d_n8, eq22_e682_q_d_n9, eq22_e682_q_d_n10, eq22_e682_q_d_n11, eq22_e682_q_d_n12, eq22_e682_q_d_n13, eq22_e682_q_d_n14, eq22_e682_q_d_n15, eq22_e682_q_d_n16, eq22_e682_q_d_n17, eq22_e682_q_d_n18, eq22_e682_q_d_n19, eq22_e682_q_d_n20, eq22_e682_q_d_n21, eq22_e682_q_d_n22, eq22_e682_q_d_n23, eq22_e682_q_d_n24, eq22_e682_q_d_n25, eq22_e682_q_d_n26, eq22_e682_q_d_n27, eq22_e682_q_d_n28, eq22_e682_q_d_n29];
        let eq22_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[26]),
            None,
            &nodes,
            &eq22_reactive_node_derivatives,
            &branches,
            &eq22_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq33_e769, eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29, eq33_e769_q, eq33_e769_q_d_n0, eq33_e769_q_d_n1, eq33_e769_q_d_n2, eq33_e769_q_d_n3, eq33_e769_q_d_n4, eq33_e769_q_d_n5, eq33_e769_q_d_n6, eq33_e769_q_d_n7, eq33_e769_q_d_n8, eq33_e769_q_d_n9, eq33_e769_q_d_n10, eq33_e769_q_d_n11, eq33_e769_q_d_n12, eq33_e769_q_d_n13, eq33_e769_q_d_n14, eq33_e769_q_d_n15, eq33_e769_q_d_n16, eq33_e769_q_d_n17, eq33_e769_q_d_n18, eq33_e769_q_d_n19, eq33_e769_q_d_n20, eq33_e769_q_d_n21, eq33_e769_q_d_n22, eq33_e769_q_d_n23, eq33_e769_q_d_n24, eq33_e769_q_d_n25, eq33_e769_q_d_n26, eq33_e769_q_d_n27, eq33_e769_q_d_n28, eq33_e769_q_d_n29,) = {
    if (s.v[466] != 0.0) {
        let eq33_e762_q: f64 = s.v[209];
        let eq33_e765: f64 = (p.p355 * (nv7 - nv16));
        let eq33_e765_d_n7: f64 = p.p355;
        let eq33_e765_d_n16: f64 = (-p.p355);
        let eq33_e766_q: f64 = eq33_e765;
        let eq33_e767: f64 = (s.v[209] + eq33_e765);
        let eq33_e767_d_n7: f64 = (s.dn[209][7] + eq33_e765_d_n7);
        let eq33_e767_d_n16: f64 = (s.dn[209][16] + eq33_e765_d_n16);
        let eq33_e767_q: f64 = (eq33_e762_q + eq33_e766_q);
        let eq33_e767_q_d_n7: f64 = (s.dn[209][7] + eq33_e765_d_n7);
        let eq33_e767_q_d_n16: f64 = (s.dn[209][16] + eq33_e765_d_n16);
        (eq33_e767, s.dn[209][0], s.dn[209][1], s.dn[209][2], s.dn[209][3], s.dn[209][4], s.dn[209][5], s.dn[209][6], eq33_e767_d_n7, s.dn[209][8], s.dn[209][9], s.dn[209][10], s.dn[209][11], s.dn[209][12], s.dn[209][13], s.dn[209][14], s.dn[209][15], eq33_e767_d_n16, s.dn[209][17], s.dn[209][18], s.dn[209][19], s.dn[209][20], s.dn[209][21], s.dn[209][22], s.dn[209][23], s.dn[209][24], s.dn[209][25], s.dn[209][26], s.dn[209][27], s.dn[209][28], s.dn[209][29], eq33_e767_q, s.dn[209][0], s.dn[209][1], s.dn[209][2], s.dn[209][3], s.dn[209][4], s.dn[209][5], s.dn[209][6], eq33_e767_q_d_n7, s.dn[209][8], s.dn[209][9], s.dn[209][10], s.dn[209][11], s.dn[209][12], s.dn[209][13], s.dn[209][14], s.dn[209][15], eq33_e767_q_d_n16, s.dn[209][17], s.dn[209][18], s.dn[209][19], s.dn[209][20], s.dn[209][21], s.dn[209][22], s.dn[209][23], s.dn[209][24], s.dn[209][25], s.dn[209][26], s.dn[209][27], s.dn[209][28], s.dn[209][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_reactive_node_derivatives: [f64; 30] = [eq33_e769_q_d_n0, eq33_e769_q_d_n1, eq33_e769_q_d_n2, eq33_e769_q_d_n3, eq33_e769_q_d_n4, eq33_e769_q_d_n5, eq33_e769_q_d_n6, eq33_e769_q_d_n7, eq33_e769_q_d_n8, eq33_e769_q_d_n9, eq33_e769_q_d_n10, eq33_e769_q_d_n11, eq33_e769_q_d_n12, eq33_e769_q_d_n13, eq33_e769_q_d_n14, eq33_e769_q_d_n15, eq33_e769_q_d_n16, eq33_e769_q_d_n17, eq33_e769_q_d_n18, eq33_e769_q_d_n19, eq33_e769_q_d_n20, eq33_e769_q_d_n21, eq33_e769_q_d_n22, eq33_e769_q_d_n23, eq33_e769_q_d_n24, eq33_e769_q_d_n25, eq33_e769_q_d_n26, eq33_e769_q_d_n27, eq33_e769_q_d_n28, eq33_e769_q_d_n29];
        let eq33_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            &nodes,
            &eq33_reactive_node_derivatives,
            &branches,
            &eq33_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq34_e779, eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29, eq34_e779_q, eq34_e779_q_d_n0, eq34_e779_q_d_n1, eq34_e779_q_d_n2, eq34_e779_q_d_n3, eq34_e779_q_d_n4, eq34_e779_q_d_n5, eq34_e779_q_d_n6, eq34_e779_q_d_n7, eq34_e779_q_d_n8, eq34_e779_q_d_n9, eq34_e779_q_d_n10, eq34_e779_q_d_n11, eq34_e779_q_d_n12, eq34_e779_q_d_n13, eq34_e779_q_d_n14, eq34_e779_q_d_n15, eq34_e779_q_d_n16, eq34_e779_q_d_n17, eq34_e779_q_d_n18, eq34_e779_q_d_n19, eq34_e779_q_d_n20, eq34_e779_q_d_n21, eq34_e779_q_d_n22, eq34_e779_q_d_n23, eq34_e779_q_d_n24, eq34_e779_q_d_n25, eq34_e779_q_d_n26, eq34_e779_q_d_n27, eq34_e779_q_d_n28, eq34_e779_q_d_n29,) = {
    if (s.v[466] != 0.0) {
        let eq34_e772_q: f64 = s.v[210];
        let eq34_e775: f64 = (p.p355 * (nv7 - nv17));
        let eq34_e775_d_n7: f64 = p.p355;
        let eq34_e775_d_n17: f64 = (-p.p355);
        let eq34_e776_q: f64 = eq34_e775;
        let eq34_e777: f64 = (s.v[210] + eq34_e775);
        let eq34_e777_d_n7: f64 = (s.dn[210][7] + eq34_e775_d_n7);
        let eq34_e777_d_n17: f64 = (s.dn[210][17] + eq34_e775_d_n17);
        let eq34_e777_q: f64 = (eq34_e772_q + eq34_e776_q);
        let eq34_e777_q_d_n7: f64 = (s.dn[210][7] + eq34_e775_d_n7);
        let eq34_e777_q_d_n17: f64 = (s.dn[210][17] + eq34_e775_d_n17);
        (eq34_e777, s.dn[210][0], s.dn[210][1], s.dn[210][2], s.dn[210][3], s.dn[210][4], s.dn[210][5], s.dn[210][6], eq34_e777_d_n7, s.dn[210][8], s.dn[210][9], s.dn[210][10], s.dn[210][11], s.dn[210][12], s.dn[210][13], s.dn[210][14], s.dn[210][15], s.dn[210][16], eq34_e777_d_n17, s.dn[210][18], s.dn[210][19], s.dn[210][20], s.dn[210][21], s.dn[210][22], s.dn[210][23], s.dn[210][24], s.dn[210][25], s.dn[210][26], s.dn[210][27], s.dn[210][28], s.dn[210][29], eq34_e777_q, s.dn[210][0], s.dn[210][1], s.dn[210][2], s.dn[210][3], s.dn[210][4], s.dn[210][5], s.dn[210][6], eq34_e777_q_d_n7, s.dn[210][8], s.dn[210][9], s.dn[210][10], s.dn[210][11], s.dn[210][12], s.dn[210][13], s.dn[210][14], s.dn[210][15], s.dn[210][16], eq34_e777_q_d_n17, s.dn[210][18], s.dn[210][19], s.dn[210][20], s.dn[210][21], s.dn[210][22], s.dn[210][23], s.dn[210][24], s.dn[210][25], s.dn[210][26], s.dn[210][27], s.dn[210][28], s.dn[210][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 30] = [eq34_e779_q_d_n0, eq34_e779_q_d_n1, eq34_e779_q_d_n2, eq34_e779_q_d_n3, eq34_e779_q_d_n4, eq34_e779_q_d_n5, eq34_e779_q_d_n6, eq34_e779_q_d_n7, eq34_e779_q_d_n8, eq34_e779_q_d_n9, eq34_e779_q_d_n10, eq34_e779_q_d_n11, eq34_e779_q_d_n12, eq34_e779_q_d_n13, eq34_e779_q_d_n14, eq34_e779_q_d_n15, eq34_e779_q_d_n16, eq34_e779_q_d_n17, eq34_e779_q_d_n18, eq34_e779_q_d_n19, eq34_e779_q_d_n20, eq34_e779_q_d_n21, eq34_e779_q_d_n22, eq34_e779_q_d_n23, eq34_e779_q_d_n24, eq34_e779_q_d_n25, eq34_e779_q_d_n26, eq34_e779_q_d_n27, eq34_e779_q_d_n28, eq34_e779_q_d_n29];
        let eq34_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[17]),
            &nodes,
            &eq34_reactive_node_derivatives,
            &branches,
            &eq34_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq35_e789, eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29, eq35_e789_q, eq35_e789_q_d_n0, eq35_e789_q_d_n1, eq35_e789_q_d_n2, eq35_e789_q_d_n3, eq35_e789_q_d_n4, eq35_e789_q_d_n5, eq35_e789_q_d_n6, eq35_e789_q_d_n7, eq35_e789_q_d_n8, eq35_e789_q_d_n9, eq35_e789_q_d_n10, eq35_e789_q_d_n11, eq35_e789_q_d_n12, eq35_e789_q_d_n13, eq35_e789_q_d_n14, eq35_e789_q_d_n15, eq35_e789_q_d_n16, eq35_e789_q_d_n17, eq35_e789_q_d_n18, eq35_e789_q_d_n19, eq35_e789_q_d_n20, eq35_e789_q_d_n21, eq35_e789_q_d_n22, eq35_e789_q_d_n23, eq35_e789_q_d_n24, eq35_e789_q_d_n25, eq35_e789_q_d_n26, eq35_e789_q_d_n27, eq35_e789_q_d_n28, eq35_e789_q_d_n29,) = {
    if (s.v[466] != 0.0) {
        let eq35_e782_q: f64 = s.v[211];
        let eq35_e785: f64 = (p.p355 * (nv2 - nv16));
        let eq35_e785_d_n2: f64 = p.p355;
        let eq35_e785_d_n16: f64 = (-p.p355);
        let eq35_e786_q: f64 = eq35_e785;
        let eq35_e787: f64 = (s.v[211] + eq35_e785);
        let eq35_e787_d_n2: f64 = (s.dn[211][2] + eq35_e785_d_n2);
        let eq35_e787_d_n16: f64 = (s.dn[211][16] + eq35_e785_d_n16);
        let eq35_e787_q: f64 = (eq35_e782_q + eq35_e786_q);
        let eq35_e787_q_d_n2: f64 = (s.dn[211][2] + eq35_e785_d_n2);
        let eq35_e787_q_d_n16: f64 = (s.dn[211][16] + eq35_e785_d_n16);
        (eq35_e787, s.dn[211][0], s.dn[211][1], eq35_e787_d_n2, s.dn[211][3], s.dn[211][4], s.dn[211][5], s.dn[211][6], s.dn[211][7], s.dn[211][8], s.dn[211][9], s.dn[211][10], s.dn[211][11], s.dn[211][12], s.dn[211][13], s.dn[211][14], s.dn[211][15], eq35_e787_d_n16, s.dn[211][17], s.dn[211][18], s.dn[211][19], s.dn[211][20], s.dn[211][21], s.dn[211][22], s.dn[211][23], s.dn[211][24], s.dn[211][25], s.dn[211][26], s.dn[211][27], s.dn[211][28], s.dn[211][29], eq35_e787_q, s.dn[211][0], s.dn[211][1], eq35_e787_q_d_n2, s.dn[211][3], s.dn[211][4], s.dn[211][5], s.dn[211][6], s.dn[211][7], s.dn[211][8], s.dn[211][9], s.dn[211][10], s.dn[211][11], s.dn[211][12], s.dn[211][13], s.dn[211][14], s.dn[211][15], eq35_e787_q_d_n16, s.dn[211][17], s.dn[211][18], s.dn[211][19], s.dn[211][20], s.dn[211][21], s.dn[211][22], s.dn[211][23], s.dn[211][24], s.dn[211][25], s.dn[211][26], s.dn[211][27], s.dn[211][28], s.dn[211][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 30] = [eq35_e789_q_d_n0, eq35_e789_q_d_n1, eq35_e789_q_d_n2, eq35_e789_q_d_n3, eq35_e789_q_d_n4, eq35_e789_q_d_n5, eq35_e789_q_d_n6, eq35_e789_q_d_n7, eq35_e789_q_d_n8, eq35_e789_q_d_n9, eq35_e789_q_d_n10, eq35_e789_q_d_n11, eq35_e789_q_d_n12, eq35_e789_q_d_n13, eq35_e789_q_d_n14, eq35_e789_q_d_n15, eq35_e789_q_d_n16, eq35_e789_q_d_n17, eq35_e789_q_d_n18, eq35_e789_q_d_n19, eq35_e789_q_d_n20, eq35_e789_q_d_n21, eq35_e789_q_d_n22, eq35_e789_q_d_n23, eq35_e789_q_d_n24, eq35_e789_q_d_n25, eq35_e789_q_d_n26, eq35_e789_q_d_n27, eq35_e789_q_d_n28, eq35_e789_q_d_n29];
        let eq35_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            &nodes,
            &eq35_reactive_node_derivatives,
            &branches,
            &eq35_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq37_e803, eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29, eq37_e803_q, eq37_e803_q_d_n0, eq37_e803_q_d_n1, eq37_e803_q_d_n2, eq37_e803_q_d_n3, eq37_e803_q_d_n4, eq37_e803_q_d_n5, eq37_e803_q_d_n6, eq37_e803_q_d_n7, eq37_e803_q_d_n8, eq37_e803_q_d_n9, eq37_e803_q_d_n10, eq37_e803_q_d_n11, eq37_e803_q_d_n12, eq37_e803_q_d_n13, eq37_e803_q_d_n14, eq37_e803_q_d_n15, eq37_e803_q_d_n16, eq37_e803_q_d_n17, eq37_e803_q_d_n18, eq37_e803_q_d_n19, eq37_e803_q_d_n20, eq37_e803_q_d_n21, eq37_e803_q_d_n22, eq37_e803_q_d_n23, eq37_e803_q_d_n24, eq37_e803_q_d_n25, eq37_e803_q_d_n26, eq37_e803_q_d_n27, eq37_e803_q_d_n28, eq37_e803_q_d_n29,) = {
    if (s.v[466] != 0.0) {
        let eq37_e796_q: f64 = s.v[213];
        let eq37_e799: f64 = (p.p355 * (nv7 - nv9));
        let eq37_e799_d_n7: f64 = p.p355;
        let eq37_e799_d_n9: f64 = (-p.p355);
        let eq37_e800_q: f64 = eq37_e799;
        let eq37_e801: f64 = (s.v[213] + eq37_e799);
        let eq37_e801_d_n7: f64 = (s.dn[213][7] + eq37_e799_d_n7);
        let eq37_e801_d_n9: f64 = (s.dn[213][9] + eq37_e799_d_n9);
        let eq37_e801_q: f64 = (eq37_e796_q + eq37_e800_q);
        let eq37_e801_q_d_n7: f64 = (s.dn[213][7] + eq37_e799_d_n7);
        let eq37_e801_q_d_n9: f64 = (s.dn[213][9] + eq37_e799_d_n9);
        (eq37_e801, s.dn[213][0], s.dn[213][1], s.dn[213][2], s.dn[213][3], s.dn[213][4], s.dn[213][5], s.dn[213][6], eq37_e801_d_n7, s.dn[213][8], eq37_e801_d_n9, s.dn[213][10], s.dn[213][11], s.dn[213][12], s.dn[213][13], s.dn[213][14], s.dn[213][15], s.dn[213][16], s.dn[213][17], s.dn[213][18], s.dn[213][19], s.dn[213][20], s.dn[213][21], s.dn[213][22], s.dn[213][23], s.dn[213][24], s.dn[213][25], s.dn[213][26], s.dn[213][27], s.dn[213][28], s.dn[213][29], eq37_e801_q, s.dn[213][0], s.dn[213][1], s.dn[213][2], s.dn[213][3], s.dn[213][4], s.dn[213][5], s.dn[213][6], eq37_e801_q_d_n7, s.dn[213][8], eq37_e801_q_d_n9, s.dn[213][10], s.dn[213][11], s.dn[213][12], s.dn[213][13], s.dn[213][14], s.dn[213][15], s.dn[213][16], s.dn[213][17], s.dn[213][18], s.dn[213][19], s.dn[213][20], s.dn[213][21], s.dn[213][22], s.dn[213][23], s.dn[213][24], s.dn[213][25], s.dn[213][26], s.dn[213][27], s.dn[213][28], s.dn[213][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_reactive_node_derivatives: [f64; 30] = [eq37_e803_q_d_n0, eq37_e803_q_d_n1, eq37_e803_q_d_n2, eq37_e803_q_d_n3, eq37_e803_q_d_n4, eq37_e803_q_d_n5, eq37_e803_q_d_n6, eq37_e803_q_d_n7, eq37_e803_q_d_n8, eq37_e803_q_d_n9, eq37_e803_q_d_n10, eq37_e803_q_d_n11, eq37_e803_q_d_n12, eq37_e803_q_d_n13, eq37_e803_q_d_n14, eq37_e803_q_d_n15, eq37_e803_q_d_n16, eq37_e803_q_d_n17, eq37_e803_q_d_n18, eq37_e803_q_d_n19, eq37_e803_q_d_n20, eq37_e803_q_d_n21, eq37_e803_q_d_n22, eq37_e803_q_d_n23, eq37_e803_q_d_n24, eq37_e803_q_d_n25, eq37_e803_q_d_n26, eq37_e803_q_d_n27, eq37_e803_q_d_n28, eq37_e803_q_d_n29];
        let eq37_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &nodes,
            &eq37_reactive_node_derivatives,
            &branches,
            &eq37_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_38_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq38_e814, eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29, eq38_e814_q, eq38_e814_q_d_n0, eq38_e814_q_d_n1, eq38_e814_q_d_n2, eq38_e814_q_d_n3, eq38_e814_q_d_n4, eq38_e814_q_d_n5, eq38_e814_q_d_n6, eq38_e814_q_d_n7, eq38_e814_q_d_n8, eq38_e814_q_d_n9, eq38_e814_q_d_n10, eq38_e814_q_d_n11, eq38_e814_q_d_n12, eq38_e814_q_d_n13, eq38_e814_q_d_n14, eq38_e814_q_d_n15, eq38_e814_q_d_n16, eq38_e814_q_d_n17, eq38_e814_q_d_n18, eq38_e814_q_d_n19, eq38_e814_q_d_n20, eq38_e814_q_d_n21, eq38_e814_q_d_n22, eq38_e814_q_d_n23, eq38_e814_q_d_n24, eq38_e814_q_d_n25, eq38_e814_q_d_n26, eq38_e814_q_d_n27, eq38_e814_q_d_n28, eq38_e814_q_d_n29,) = {
    if (!(s.v[466] != 0.0)) {
        let eq38_e807_q: f64 = s.v[209];
        let eq38_e810: f64 = (p.p355 * (nv2 - nv16));
        let eq38_e810_d_n2: f64 = p.p355;
        let eq38_e810_d_n16: f64 = (-p.p355);
        let eq38_e811_q: f64 = eq38_e810;
        let eq38_e812: f64 = (s.v[209] + eq38_e810);
        let eq38_e812_d_n2: f64 = (s.dn[209][2] + eq38_e810_d_n2);
        let eq38_e812_d_n16: f64 = (s.dn[209][16] + eq38_e810_d_n16);
        let eq38_e812_q: f64 = (eq38_e807_q + eq38_e811_q);
        let eq38_e812_q_d_n2: f64 = (s.dn[209][2] + eq38_e810_d_n2);
        let eq38_e812_q_d_n16: f64 = (s.dn[209][16] + eq38_e810_d_n16);
        (eq38_e812, s.dn[209][0], s.dn[209][1], eq38_e812_d_n2, s.dn[209][3], s.dn[209][4], s.dn[209][5], s.dn[209][6], s.dn[209][7], s.dn[209][8], s.dn[209][9], s.dn[209][10], s.dn[209][11], s.dn[209][12], s.dn[209][13], s.dn[209][14], s.dn[209][15], eq38_e812_d_n16, s.dn[209][17], s.dn[209][18], s.dn[209][19], s.dn[209][20], s.dn[209][21], s.dn[209][22], s.dn[209][23], s.dn[209][24], s.dn[209][25], s.dn[209][26], s.dn[209][27], s.dn[209][28], s.dn[209][29], eq38_e812_q, s.dn[209][0], s.dn[209][1], eq38_e812_q_d_n2, s.dn[209][3], s.dn[209][4], s.dn[209][5], s.dn[209][6], s.dn[209][7], s.dn[209][8], s.dn[209][9], s.dn[209][10], s.dn[209][11], s.dn[209][12], s.dn[209][13], s.dn[209][14], s.dn[209][15], eq38_e812_q_d_n16, s.dn[209][17], s.dn[209][18], s.dn[209][19], s.dn[209][20], s.dn[209][21], s.dn[209][22], s.dn[209][23], s.dn[209][24], s.dn[209][25], s.dn[209][26], s.dn[209][27], s.dn[209][28], s.dn[209][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_reactive_node_derivatives: [f64; 30] = [eq38_e814_q_d_n0, eq38_e814_q_d_n1, eq38_e814_q_d_n2, eq38_e814_q_d_n3, eq38_e814_q_d_n4, eq38_e814_q_d_n5, eq38_e814_q_d_n6, eq38_e814_q_d_n7, eq38_e814_q_d_n8, eq38_e814_q_d_n9, eq38_e814_q_d_n10, eq38_e814_q_d_n11, eq38_e814_q_d_n12, eq38_e814_q_d_n13, eq38_e814_q_d_n14, eq38_e814_q_d_n15, eq38_e814_q_d_n16, eq38_e814_q_d_n17, eq38_e814_q_d_n18, eq38_e814_q_d_n19, eq38_e814_q_d_n20, eq38_e814_q_d_n21, eq38_e814_q_d_n22, eq38_e814_q_d_n23, eq38_e814_q_d_n24, eq38_e814_q_d_n25, eq38_e814_q_d_n26, eq38_e814_q_d_n27, eq38_e814_q_d_n28, eq38_e814_q_d_n29];
        let eq38_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            &nodes,
            &eq38_reactive_node_derivatives,
            &branches,
            &eq38_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_39_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq39_e825, eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29, eq39_e825_q, eq39_e825_q_d_n0, eq39_e825_q_d_n1, eq39_e825_q_d_n2, eq39_e825_q_d_n3, eq39_e825_q_d_n4, eq39_e825_q_d_n5, eq39_e825_q_d_n6, eq39_e825_q_d_n7, eq39_e825_q_d_n8, eq39_e825_q_d_n9, eq39_e825_q_d_n10, eq39_e825_q_d_n11, eq39_e825_q_d_n12, eq39_e825_q_d_n13, eq39_e825_q_d_n14, eq39_e825_q_d_n15, eq39_e825_q_d_n16, eq39_e825_q_d_n17, eq39_e825_q_d_n18, eq39_e825_q_d_n19, eq39_e825_q_d_n20, eq39_e825_q_d_n21, eq39_e825_q_d_n22, eq39_e825_q_d_n23, eq39_e825_q_d_n24, eq39_e825_q_d_n25, eq39_e825_q_d_n26, eq39_e825_q_d_n27, eq39_e825_q_d_n28, eq39_e825_q_d_n29,) = {
    if (!(s.v[466] != 0.0)) {
        let eq39_e818_q: f64 = s.v[210];
        let eq39_e821: f64 = (p.p355 * (nv2 - nv17));
        let eq39_e821_d_n2: f64 = p.p355;
        let eq39_e821_d_n17: f64 = (-p.p355);
        let eq39_e822_q: f64 = eq39_e821;
        let eq39_e823: f64 = (s.v[210] + eq39_e821);
        let eq39_e823_d_n2: f64 = (s.dn[210][2] + eq39_e821_d_n2);
        let eq39_e823_d_n17: f64 = (s.dn[210][17] + eq39_e821_d_n17);
        let eq39_e823_q: f64 = (eq39_e818_q + eq39_e822_q);
        let eq39_e823_q_d_n2: f64 = (s.dn[210][2] + eq39_e821_d_n2);
        let eq39_e823_q_d_n17: f64 = (s.dn[210][17] + eq39_e821_d_n17);
        (eq39_e823, s.dn[210][0], s.dn[210][1], eq39_e823_d_n2, s.dn[210][3], s.dn[210][4], s.dn[210][5], s.dn[210][6], s.dn[210][7], s.dn[210][8], s.dn[210][9], s.dn[210][10], s.dn[210][11], s.dn[210][12], s.dn[210][13], s.dn[210][14], s.dn[210][15], s.dn[210][16], eq39_e823_d_n17, s.dn[210][18], s.dn[210][19], s.dn[210][20], s.dn[210][21], s.dn[210][22], s.dn[210][23], s.dn[210][24], s.dn[210][25], s.dn[210][26], s.dn[210][27], s.dn[210][28], s.dn[210][29], eq39_e823_q, s.dn[210][0], s.dn[210][1], eq39_e823_q_d_n2, s.dn[210][3], s.dn[210][4], s.dn[210][5], s.dn[210][6], s.dn[210][7], s.dn[210][8], s.dn[210][9], s.dn[210][10], s.dn[210][11], s.dn[210][12], s.dn[210][13], s.dn[210][14], s.dn[210][15], s.dn[210][16], eq39_e823_q_d_n17, s.dn[210][18], s.dn[210][19], s.dn[210][20], s.dn[210][21], s.dn[210][22], s.dn[210][23], s.dn[210][24], s.dn[210][25], s.dn[210][26], s.dn[210][27], s.dn[210][28], s.dn[210][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 30] = [eq39_e825_q_d_n0, eq39_e825_q_d_n1, eq39_e825_q_d_n2, eq39_e825_q_d_n3, eq39_e825_q_d_n4, eq39_e825_q_d_n5, eq39_e825_q_d_n6, eq39_e825_q_d_n7, eq39_e825_q_d_n8, eq39_e825_q_d_n9, eq39_e825_q_d_n10, eq39_e825_q_d_n11, eq39_e825_q_d_n12, eq39_e825_q_d_n13, eq39_e825_q_d_n14, eq39_e825_q_d_n15, eq39_e825_q_d_n16, eq39_e825_q_d_n17, eq39_e825_q_d_n18, eq39_e825_q_d_n19, eq39_e825_q_d_n20, eq39_e825_q_d_n21, eq39_e825_q_d_n22, eq39_e825_q_d_n23, eq39_e825_q_d_n24, eq39_e825_q_d_n25, eq39_e825_q_d_n26, eq39_e825_q_d_n27, eq39_e825_q_d_n28, eq39_e825_q_d_n29];
        let eq39_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            &nodes,
            &eq39_reactive_node_derivatives,
            &branches,
            &eq39_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_40_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq40_e836, eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29, eq40_e836_q, eq40_e836_q_d_n0, eq40_e836_q_d_n1, eq40_e836_q_d_n2, eq40_e836_q_d_n3, eq40_e836_q_d_n4, eq40_e836_q_d_n5, eq40_e836_q_d_n6, eq40_e836_q_d_n7, eq40_e836_q_d_n8, eq40_e836_q_d_n9, eq40_e836_q_d_n10, eq40_e836_q_d_n11, eq40_e836_q_d_n12, eq40_e836_q_d_n13, eq40_e836_q_d_n14, eq40_e836_q_d_n15, eq40_e836_q_d_n16, eq40_e836_q_d_n17, eq40_e836_q_d_n18, eq40_e836_q_d_n19, eq40_e836_q_d_n20, eq40_e836_q_d_n21, eq40_e836_q_d_n22, eq40_e836_q_d_n23, eq40_e836_q_d_n24, eq40_e836_q_d_n25, eq40_e836_q_d_n26, eq40_e836_q_d_n27, eq40_e836_q_d_n28, eq40_e836_q_d_n29,) = {
    if (!(s.v[466] != 0.0)) {
        let eq40_e829_q: f64 = s.v[211];
        let eq40_e832: f64 = (p.p355 * (nv7 - nv16));
        let eq40_e832_d_n7: f64 = p.p355;
        let eq40_e832_d_n16: f64 = (-p.p355);
        let eq40_e833_q: f64 = eq40_e832;
        let eq40_e834: f64 = (s.v[211] + eq40_e832);
        let eq40_e834_d_n7: f64 = (s.dn[211][7] + eq40_e832_d_n7);
        let eq40_e834_d_n16: f64 = (s.dn[211][16] + eq40_e832_d_n16);
        let eq40_e834_q: f64 = (eq40_e829_q + eq40_e833_q);
        let eq40_e834_q_d_n7: f64 = (s.dn[211][7] + eq40_e832_d_n7);
        let eq40_e834_q_d_n16: f64 = (s.dn[211][16] + eq40_e832_d_n16);
        (eq40_e834, s.dn[211][0], s.dn[211][1], s.dn[211][2], s.dn[211][3], s.dn[211][4], s.dn[211][5], s.dn[211][6], eq40_e834_d_n7, s.dn[211][8], s.dn[211][9], s.dn[211][10], s.dn[211][11], s.dn[211][12], s.dn[211][13], s.dn[211][14], s.dn[211][15], eq40_e834_d_n16, s.dn[211][17], s.dn[211][18], s.dn[211][19], s.dn[211][20], s.dn[211][21], s.dn[211][22], s.dn[211][23], s.dn[211][24], s.dn[211][25], s.dn[211][26], s.dn[211][27], s.dn[211][28], s.dn[211][29], eq40_e834_q, s.dn[211][0], s.dn[211][1], s.dn[211][2], s.dn[211][3], s.dn[211][4], s.dn[211][5], s.dn[211][6], eq40_e834_q_d_n7, s.dn[211][8], s.dn[211][9], s.dn[211][10], s.dn[211][11], s.dn[211][12], s.dn[211][13], s.dn[211][14], s.dn[211][15], eq40_e834_q_d_n16, s.dn[211][17], s.dn[211][18], s.dn[211][19], s.dn[211][20], s.dn[211][21], s.dn[211][22], s.dn[211][23], s.dn[211][24], s.dn[211][25], s.dn[211][26], s.dn[211][27], s.dn[211][28], s.dn[211][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 30] = [eq40_e836_q_d_n0, eq40_e836_q_d_n1, eq40_e836_q_d_n2, eq40_e836_q_d_n3, eq40_e836_q_d_n4, eq40_e836_q_d_n5, eq40_e836_q_d_n6, eq40_e836_q_d_n7, eq40_e836_q_d_n8, eq40_e836_q_d_n9, eq40_e836_q_d_n10, eq40_e836_q_d_n11, eq40_e836_q_d_n12, eq40_e836_q_d_n13, eq40_e836_q_d_n14, eq40_e836_q_d_n15, eq40_e836_q_d_n16, eq40_e836_q_d_n17, eq40_e836_q_d_n18, eq40_e836_q_d_n19, eq40_e836_q_d_n20, eq40_e836_q_d_n21, eq40_e836_q_d_n22, eq40_e836_q_d_n23, eq40_e836_q_d_n24, eq40_e836_q_d_n25, eq40_e836_q_d_n26, eq40_e836_q_d_n27, eq40_e836_q_d_n28, eq40_e836_q_d_n29];
        let eq40_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            &nodes,
            &eq40_reactive_node_derivatives,
            &branches,
            &eq40_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_43_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let eq43_e848_q: f64 = s.v[212];
        let eq43_e851: f64 = (p.p355 * (nv3 - nv16));
        let eq43_e851_d_n3: f64 = p.p355;
        let eq43_e851_d_n16: f64 = (-p.p355);
        let eq43_e852_q: f64 = eq43_e851;
        let eq43_e853: f64 = (s.v[212] + eq43_e851);
        let eq43_e853_d_n3: f64 = (s.dn[212][3] + eq43_e851_d_n3);
        let eq43_e853_d_n16: f64 = (s.dn[212][16] + eq43_e851_d_n16);
        let eq43_e853_q: f64 = (eq43_e848_q + eq43_e852_q);
        let eq43_e853_q_d_n3: f64 = (s.dn[212][3] + eq43_e851_d_n3);
        let eq43_e853_q_d_n16: f64 = (s.dn[212][16] + eq43_e851_d_n16);
        let eq43_reactive_node_derivatives: [f64; 30] = [s.dn[212][0], s.dn[212][1], s.dn[212][2], eq43_e853_q_d_n3, s.dn[212][4], s.dn[212][5], s.dn[212][6], s.dn[212][7], s.dn[212][8], s.dn[212][9], s.dn[212][10], s.dn[212][11], s.dn[212][12], s.dn[212][13], s.dn[212][14], s.dn[212][15], eq43_e853_q_d_n16, s.dn[212][17], s.dn[212][18], s.dn[212][19], s.dn[212][20], s.dn[212][21], s.dn[212][22], s.dn[212][23], s.dn[212][24], s.dn[212][25], s.dn[212][26], s.dn[212][27], s.dn[212][28], s.dn[212][29]];
        let eq43_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[16]),
            &nodes,
            &eq43_reactive_node_derivatives,
            &branches,
            &eq43_reactive_branch_derivatives,
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq46_e876, eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29, eq46_e876_q, eq46_e876_q_d_n0, eq46_e876_q_d_n1, eq46_e876_q_d_n2, eq46_e876_q_d_n3, eq46_e876_q_d_n4, eq46_e876_q_d_n5, eq46_e876_q_d_n6, eq46_e876_q_d_n7, eq46_e876_q_d_n8, eq46_e876_q_d_n9, eq46_e876_q_d_n10, eq46_e876_q_d_n11, eq46_e876_q_d_n12, eq46_e876_q_d_n13, eq46_e876_q_d_n14, eq46_e876_q_d_n15, eq46_e876_q_d_n16, eq46_e876_q_d_n17, eq46_e876_q_d_n18, eq46_e876_q_d_n19, eq46_e876_q_d_n20, eq46_e876_q_d_n21, eq46_e876_q_d_n22, eq46_e876_q_d_n23, eq46_e876_q_d_n24, eq46_e876_q_d_n25, eq46_e876_q_d_n26, eq46_e876_q_d_n27, eq46_e876_q_d_n28, eq46_e876_q_d_n29,) = {
    if (s.v[613] != 0.0) {
        let eq46_e869_q: f64 = s.v[203];
        let eq46_e872: f64 = (p.p355 * (nv7 - nv15));
        let eq46_e872_d_n7: f64 = p.p355;
        let eq46_e872_d_n15: f64 = (-p.p355);
        let eq46_e873_q: f64 = eq46_e872;
        let eq46_e874: f64 = (s.v[203] + eq46_e872);
        let eq46_e874_d_n7: f64 = (s.dn[203][7] + eq46_e872_d_n7);
        let eq46_e874_d_n15: f64 = (s.dn[203][15] + eq46_e872_d_n15);
        let eq46_e874_q: f64 = (eq46_e869_q + eq46_e873_q);
        let eq46_e874_q_d_n7: f64 = (s.dn[203][7] + eq46_e872_d_n7);
        let eq46_e874_q_d_n15: f64 = (s.dn[203][15] + eq46_e872_d_n15);
        (eq46_e874, s.dn[203][0], s.dn[203][1], s.dn[203][2], s.dn[203][3], s.dn[203][4], s.dn[203][5], s.dn[203][6], eq46_e874_d_n7, s.dn[203][8], s.dn[203][9], s.dn[203][10], s.dn[203][11], s.dn[203][12], s.dn[203][13], s.dn[203][14], eq46_e874_d_n15, s.dn[203][16], s.dn[203][17], s.dn[203][18], s.dn[203][19], s.dn[203][20], s.dn[203][21], s.dn[203][22], s.dn[203][23], s.dn[203][24], s.dn[203][25], s.dn[203][26], s.dn[203][27], s.dn[203][28], s.dn[203][29], eq46_e874_q, s.dn[203][0], s.dn[203][1], s.dn[203][2], s.dn[203][3], s.dn[203][4], s.dn[203][5], s.dn[203][6], eq46_e874_q_d_n7, s.dn[203][8], s.dn[203][9], s.dn[203][10], s.dn[203][11], s.dn[203][12], s.dn[203][13], s.dn[203][14], eq46_e874_q_d_n15, s.dn[203][16], s.dn[203][17], s.dn[203][18], s.dn[203][19], s.dn[203][20], s.dn[203][21], s.dn[203][22], s.dn[203][23], s.dn[203][24], s.dn[203][25], s.dn[203][26], s.dn[203][27], s.dn[203][28], s.dn[203][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 30] = [eq46_e876_q_d_n0, eq46_e876_q_d_n1, eq46_e876_q_d_n2, eq46_e876_q_d_n3, eq46_e876_q_d_n4, eq46_e876_q_d_n5, eq46_e876_q_d_n6, eq46_e876_q_d_n7, eq46_e876_q_d_n8, eq46_e876_q_d_n9, eq46_e876_q_d_n10, eq46_e876_q_d_n11, eq46_e876_q_d_n12, eq46_e876_q_d_n13, eq46_e876_q_d_n14, eq46_e876_q_d_n15, eq46_e876_q_d_n16, eq46_e876_q_d_n17, eq46_e876_q_d_n18, eq46_e876_q_d_n19, eq46_e876_q_d_n20, eq46_e876_q_d_n21, eq46_e876_q_d_n22, eq46_e876_q_d_n23, eq46_e876_q_d_n24, eq46_e876_q_d_n25, eq46_e876_q_d_n26, eq46_e876_q_d_n27, eq46_e876_q_d_n28, eq46_e876_q_d_n29];
        let eq46_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq47_e886, eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29, eq47_e886_q, eq47_e886_q_d_n0, eq47_e886_q_d_n1, eq47_e886_q_d_n2, eq47_e886_q_d_n3, eq47_e886_q_d_n4, eq47_e886_q_d_n5, eq47_e886_q_d_n6, eq47_e886_q_d_n7, eq47_e886_q_d_n8, eq47_e886_q_d_n9, eq47_e886_q_d_n10, eq47_e886_q_d_n11, eq47_e886_q_d_n12, eq47_e886_q_d_n13, eq47_e886_q_d_n14, eq47_e886_q_d_n15, eq47_e886_q_d_n16, eq47_e886_q_d_n17, eq47_e886_q_d_n18, eq47_e886_q_d_n19, eq47_e886_q_d_n20, eq47_e886_q_d_n21, eq47_e886_q_d_n22, eq47_e886_q_d_n23, eq47_e886_q_d_n24, eq47_e886_q_d_n25, eq47_e886_q_d_n26, eq47_e886_q_d_n27, eq47_e886_q_d_n28, eq47_e886_q_d_n29,) = {
    if (s.v[613] != 0.0) {
        let eq47_e879_q: f64 = s.v[204];
        let eq47_e882: f64 = (p.p355 * (nv7 - nv16));
        let eq47_e882_d_n7: f64 = p.p355;
        let eq47_e882_d_n16: f64 = (-p.p355);
        let eq47_e883_q: f64 = eq47_e882;
        let eq47_e884: f64 = (s.v[204] + eq47_e882);
        let eq47_e884_d_n7: f64 = (s.dn[204][7] + eq47_e882_d_n7);
        let eq47_e884_d_n16: f64 = (s.dn[204][16] + eq47_e882_d_n16);
        let eq47_e884_q: f64 = (eq47_e879_q + eq47_e883_q);
        let eq47_e884_q_d_n7: f64 = (s.dn[204][7] + eq47_e882_d_n7);
        let eq47_e884_q_d_n16: f64 = (s.dn[204][16] + eq47_e882_d_n16);
        (eq47_e884, s.dn[204][0], s.dn[204][1], s.dn[204][2], s.dn[204][3], s.dn[204][4], s.dn[204][5], s.dn[204][6], eq47_e884_d_n7, s.dn[204][8], s.dn[204][9], s.dn[204][10], s.dn[204][11], s.dn[204][12], s.dn[204][13], s.dn[204][14], s.dn[204][15], eq47_e884_d_n16, s.dn[204][17], s.dn[204][18], s.dn[204][19], s.dn[204][20], s.dn[204][21], s.dn[204][22], s.dn[204][23], s.dn[204][24], s.dn[204][25], s.dn[204][26], s.dn[204][27], s.dn[204][28], s.dn[204][29], eq47_e884_q, s.dn[204][0], s.dn[204][1], s.dn[204][2], s.dn[204][3], s.dn[204][4], s.dn[204][5], s.dn[204][6], eq47_e884_q_d_n7, s.dn[204][8], s.dn[204][9], s.dn[204][10], s.dn[204][11], s.dn[204][12], s.dn[204][13], s.dn[204][14], s.dn[204][15], eq47_e884_q_d_n16, s.dn[204][17], s.dn[204][18], s.dn[204][19], s.dn[204][20], s.dn[204][21], s.dn[204][22], s.dn[204][23], s.dn[204][24], s.dn[204][25], s.dn[204][26], s.dn[204][27], s.dn[204][28], s.dn[204][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_reactive_node_derivatives: [f64; 30] = [eq47_e886_q_d_n0, eq47_e886_q_d_n1, eq47_e886_q_d_n2, eq47_e886_q_d_n3, eq47_e886_q_d_n4, eq47_e886_q_d_n5, eq47_e886_q_d_n6, eq47_e886_q_d_n7, eq47_e886_q_d_n8, eq47_e886_q_d_n9, eq47_e886_q_d_n10, eq47_e886_q_d_n11, eq47_e886_q_d_n12, eq47_e886_q_d_n13, eq47_e886_q_d_n14, eq47_e886_q_d_n15, eq47_e886_q_d_n16, eq47_e886_q_d_n17, eq47_e886_q_d_n18, eq47_e886_q_d_n19, eq47_e886_q_d_n20, eq47_e886_q_d_n21, eq47_e886_q_d_n22, eq47_e886_q_d_n23, eq47_e886_q_d_n24, eq47_e886_q_d_n25, eq47_e886_q_d_n26, eq47_e886_q_d_n27, eq47_e886_q_d_n28, eq47_e886_q_d_n29];
        let eq47_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            &nodes,
            &eq47_reactive_node_derivatives,
            &branches,
            &eq47_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
