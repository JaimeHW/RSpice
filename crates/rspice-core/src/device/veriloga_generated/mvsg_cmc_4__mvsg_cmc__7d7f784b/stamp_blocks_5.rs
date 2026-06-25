#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let nv25 = ctx.node_voltage(nodes[25]);
        let nv27 = ctx.node_voltage(nodes[27]);
        let (eq20_e645, eq20_e645_d_n25, eq20_e645_d_n27,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq20_e643: f64 = ((nv25 - nv27) / p.p340);
        let eq20_e643_d_n25: f64 = (1.0 / p.p340);
        let eq20_e643_d_n27: f64 = (-1.0 / p.p340);
        (eq20_e643, eq20_e643_d_n25, eq20_e643_d_n27,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e645;
        stamper.stamp_current(
            Some(nodes[25]),
            Some(nodes[27]),
            self.multiplicity * (eq20_value),
            &[
                GeneratedDerivative::node(nodes[25], self.multiplicity * eq20_e645_d_n25),
                GeneratedDerivative::node(nodes[27], self.multiplicity * eq20_e645_d_n27),
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
        let nv25 = ctx.node_voltage(nodes[25]);
        let nv26 = ctx.node_voltage(nodes[26]);
        let (eq21_e654, eq21_e654_d_n25, eq21_e654_d_n26,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq21_e652: f64 = ((nv25 - nv26) / p.p339);
        let eq21_e652_d_n25: f64 = (1.0 / p.p339);
        let eq21_e652_d_n26: f64 = (-1.0 / p.p339);
        (eq21_e652, eq21_e652_d_n25, eq21_e652_d_n26,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e654;
        stamper.stamp_current(
            Some(nodes[25]),
            Some(nodes[26]),
            self.multiplicity * (eq21_value),
            &[
                GeneratedDerivative::node(nodes[25], self.multiplicity * eq21_e654_d_n25),
                GeneratedDerivative::node(nodes[26], self.multiplicity * eq21_e654_d_n26),
            ],
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
        let (eq22_e682, eq22_e682_d_n0, eq22_e682_d_n1, eq22_e682_d_n2, eq22_e682_d_n3, eq22_e682_d_n4, eq22_e682_d_n5, eq22_e682_d_n6, eq22_e682_d_n7, eq22_e682_d_n8, eq22_e682_d_n9, eq22_e682_d_n10, eq22_e682_d_n11, eq22_e682_d_n12, eq22_e682_d_n13, eq22_e682_d_n14, eq22_e682_d_n15, eq22_e682_d_n16, eq22_e682_d_n17, eq22_e682_d_n18, eq22_e682_d_n19, eq22_e682_d_n20, eq22_e682_d_n21, eq22_e682_d_n22, eq22_e682_d_n23, eq22_e682_d_n24, eq22_e682_d_n25, eq22_e682_d_n26, eq22_e682_d_n27, eq22_e682_d_n28, eq22_e682_d_n29,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq22_e661: f64 = self.eval_ddt(3, s.v[227]);
        let eq22_e661_d_n0: f64 = self.ddt_jacobian(s.dn[227][0]);
        let eq22_e661_d_n1: f64 = self.ddt_jacobian(s.dn[227][1]);
        let eq22_e661_d_n2: f64 = self.ddt_jacobian(s.dn[227][2]);
        let eq22_e661_d_n3: f64 = self.ddt_jacobian(s.dn[227][3]);
        let eq22_e661_d_n4: f64 = self.ddt_jacobian(s.dn[227][4]);
        let eq22_e661_d_n5: f64 = self.ddt_jacobian(s.dn[227][5]);
        let eq22_e661_d_n6: f64 = self.ddt_jacobian(s.dn[227][6]);
        let eq22_e661_d_n7: f64 = self.ddt_jacobian(s.dn[227][7]);
        let eq22_e661_d_n8: f64 = self.ddt_jacobian(s.dn[227][8]);
        let eq22_e661_d_n9: f64 = self.ddt_jacobian(s.dn[227][9]);
        let eq22_e661_d_n10: f64 = self.ddt_jacobian(s.dn[227][10]);
        let eq22_e661_d_n11: f64 = self.ddt_jacobian(s.dn[227][11]);
        let eq22_e661_d_n12: f64 = self.ddt_jacobian(s.dn[227][12]);
        let eq22_e661_d_n13: f64 = self.ddt_jacobian(s.dn[227][13]);
        let eq22_e661_d_n14: f64 = self.ddt_jacobian(s.dn[227][14]);
        let eq22_e661_d_n15: f64 = self.ddt_jacobian(s.dn[227][15]);
        let eq22_e661_d_n16: f64 = self.ddt_jacobian(s.dn[227][16]);
        let eq22_e661_d_n17: f64 = self.ddt_jacobian(s.dn[227][17]);
        let eq22_e661_d_n18: f64 = self.ddt_jacobian(s.dn[227][18]);
        let eq22_e661_d_n19: f64 = self.ddt_jacobian(s.dn[227][19]);
        let eq22_e661_d_n20: f64 = self.ddt_jacobian(s.dn[227][20]);
        let eq22_e661_d_n21: f64 = self.ddt_jacobian(s.dn[227][21]);
        let eq22_e661_d_n22: f64 = self.ddt_jacobian(s.dn[227][22]);
        let eq22_e661_d_n23: f64 = self.ddt_jacobian(s.dn[227][23]);
        let eq22_e661_d_n24: f64 = self.ddt_jacobian(s.dn[227][24]);
        let eq22_e661_d_n25: f64 = self.ddt_jacobian(s.dn[227][25]);
        let eq22_e661_d_n26: f64 = self.ddt_jacobian(s.dn[227][26]);
        let eq22_e661_d_n27: f64 = self.ddt_jacobian(s.dn[227][27]);
        let eq22_e661_d_n28: f64 = self.ddt_jacobian(s.dn[227][28]);
        let eq22_e661_d_n29: f64 = self.ddt_jacobian(s.dn[227][29]);
        let eq22_e662: f64 = (p.p341 * eq22_e661);
        let eq22_e662_d_n0: f64 = (p.p341 * eq22_e661_d_n0);
        let eq22_e662_d_n1: f64 = (p.p341 * eq22_e661_d_n1);
        let eq22_e662_d_n2: f64 = (p.p341 * eq22_e661_d_n2);
        let eq22_e662_d_n3: f64 = (p.p341 * eq22_e661_d_n3);
        let eq22_e662_d_n4: f64 = (p.p341 * eq22_e661_d_n4);
        let eq22_e662_d_n5: f64 = (p.p341 * eq22_e661_d_n5);
        let eq22_e662_d_n6: f64 = (p.p341 * eq22_e661_d_n6);
        let eq22_e662_d_n7: f64 = (p.p341 * eq22_e661_d_n7);
        let eq22_e662_d_n8: f64 = (p.p341 * eq22_e661_d_n8);
        let eq22_e662_d_n9: f64 = (p.p341 * eq22_e661_d_n9);
        let eq22_e662_d_n10: f64 = (p.p341 * eq22_e661_d_n10);
        let eq22_e662_d_n11: f64 = (p.p341 * eq22_e661_d_n11);
        let eq22_e662_d_n12: f64 = (p.p341 * eq22_e661_d_n12);
        let eq22_e662_d_n13: f64 = (p.p341 * eq22_e661_d_n13);
        let eq22_e662_d_n14: f64 = (p.p341 * eq22_e661_d_n14);
        let eq22_e662_d_n15: f64 = (p.p341 * eq22_e661_d_n15);
        let eq22_e662_d_n16: f64 = (p.p341 * eq22_e661_d_n16);
        let eq22_e662_d_n17: f64 = (p.p341 * eq22_e661_d_n17);
        let eq22_e662_d_n18: f64 = (p.p341 * eq22_e661_d_n18);
        let eq22_e662_d_n19: f64 = (p.p341 * eq22_e661_d_n19);
        let eq22_e662_d_n20: f64 = (p.p341 * eq22_e661_d_n20);
        let eq22_e662_d_n21: f64 = (p.p341 * eq22_e661_d_n21);
        let eq22_e662_d_n22: f64 = (p.p341 * eq22_e661_d_n22);
        let eq22_e662_d_n23: f64 = (p.p341 * eq22_e661_d_n23);
        let eq22_e662_d_n24: f64 = (p.p341 * eq22_e661_d_n24);
        let eq22_e662_d_n25: f64 = (p.p341 * eq22_e661_d_n25);
        let eq22_e662_d_n26: f64 = (p.p341 * eq22_e661_d_n26);
        let eq22_e662_d_n27: f64 = (p.p341 * eq22_e661_d_n27);
        let eq22_e662_d_n28: f64 = (p.p341 * eq22_e661_d_n28);
        let eq22_e662_d_n29: f64 = (p.p341 * eq22_e661_d_n29);
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
        (eq22_e680, eq22_e680_d_n0, eq22_e680_d_n1, eq22_e680_d_n2, eq22_e680_d_n3, eq22_e680_d_n4, eq22_e680_d_n5, eq22_e680_d_n6, eq22_e680_d_n7, eq22_e680_d_n8, eq22_e680_d_n9, eq22_e680_d_n10, eq22_e680_d_n11, eq22_e680_d_n12, eq22_e680_d_n13, eq22_e680_d_n14, eq22_e680_d_n15, eq22_e680_d_n16, eq22_e680_d_n17, eq22_e680_d_n18, eq22_e680_d_n19, eq22_e680_d_n20, eq22_e680_d_n21, eq22_e680_d_n22, eq22_e680_d_n23, eq22_e680_d_n24, eq22_e680_d_n25, eq22_e680_d_n26, eq22_e680_d_n27, eq22_e680_d_n28, eq22_e680_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e682;
        let eq22_node_derivatives: [f64; 30] = [eq22_e682_d_n0, eq22_e682_d_n1, eq22_e682_d_n2, eq22_e682_d_n3, eq22_e682_d_n4, eq22_e682_d_n5, eq22_e682_d_n6, eq22_e682_d_n7, eq22_e682_d_n8, eq22_e682_d_n9, eq22_e682_d_n10, eq22_e682_d_n11, eq22_e682_d_n12, eq22_e682_d_n13, eq22_e682_d_n14, eq22_e682_d_n15, eq22_e682_d_n16, eq22_e682_d_n17, eq22_e682_d_n18, eq22_e682_d_n19, eq22_e682_d_n20, eq22_e682_d_n21, eq22_e682_d_n22, eq22_e682_d_n23, eq22_e682_d_n24, eq22_e682_d_n25, eq22_e682_d_n26, eq22_e682_d_n27, eq22_e682_d_n28, eq22_e682_d_n29];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[26]),
            None,
            self.multiplicity * (eq22_value),
            &nodes,
            &eq22_node_derivatives,
            &branches,
            &eq22_branch_derivatives,
            self.multiplicity,
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
        let (eq23_e690,) = {
    if ((!(s.v[308] != 0.0)) && (!(s.v[309] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e690;
        stamper.stamp_potential(
            branches[10],
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
        let (eq24_e698,) = {
    if ((!(s.v[308] != 0.0)) && (!(s.v[309] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e698;
        stamper.stamp_potential(
            branches[11],
            eq24_value,
            &[
            ],
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
        let (eq25_e706,) = {
    if ((!(s.v[308] != 0.0)) && (!(s.v[309] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e706;
        stamper.stamp_potential(
            branches[12],
            eq25_value,
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
        let (eq26_e714,) = {
    if ((!(s.v[308] != 0.0)) && (!(s.v[309] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e714;
        stamper.stamp_potential(
            branches[13],
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
        let (eq27_e722,) = {
    if ((!(s.v[308] != 0.0)) && (!(s.v[309] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e722;
        stamper.stamp_potential(
            branches[14],
            eq27_value,
            &[
            ],
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
        let (eq28_e730,) = {
    if ((!(s.v[308] != 0.0)) && (!(s.v[309] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e730;
        stamper.stamp_potential(
            branches[15],
            eq28_value,
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
        let (eq29_e738,) = {
    if ((!(s.v[308] != 0.0)) && (!(s.v[309] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e738;
        stamper.stamp_potential(
            branches[16],
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
        let (eq30_e746,) = {
    if ((!(s.v[308] != 0.0)) && (!(s.v[309] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e746;
        stamper.stamp_potential(
            branches[17],
            eq30_value,
            &[
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
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq31_e754, eq31_e754_d_n0, eq31_e754_d_n1, eq31_e754_d_n2, eq31_e754_d_n3, eq31_e754_d_n4, eq31_e754_d_n5, eq31_e754_d_n6, eq31_e754_d_n7, eq31_e754_d_n8, eq31_e754_d_n9, eq31_e754_d_n10, eq31_e754_d_n11, eq31_e754_d_n12, eq31_e754_d_n13, eq31_e754_d_n14, eq31_e754_d_n15, eq31_e754_d_n16, eq31_e754_d_n17, eq31_e754_d_n18, eq31_e754_d_n19, eq31_e754_d_n20, eq31_e754_d_n21, eq31_e754_d_n22, eq31_e754_d_n23, eq31_e754_d_n24, eq31_e754_d_n25, eq31_e754_d_n26, eq31_e754_d_n27, eq31_e754_d_n28, eq31_e754_d_n29,) = {
    if (s.v[320] != 0.0) {
        let eq31_e751: f64 = (s.v[0] * (nv17 - nv16));
        let eq31_e751_d_n16: f64 = (-s.v[0]);
        let eq31_e751_d_n17: f64 = s.v[0];
        let eq31_e752: f64 = (s.v[208] + eq31_e751);
        let eq31_e752_d_n16: f64 = (s.dn[208][16] + eq31_e751_d_n16);
        let eq31_e752_d_n17: f64 = (s.dn[208][17] + eq31_e751_d_n17);
        (eq31_e752, s.dn[208][0], s.dn[208][1], s.dn[208][2], s.dn[208][3], s.dn[208][4], s.dn[208][5], s.dn[208][6], s.dn[208][7], s.dn[208][8], s.dn[208][9], s.dn[208][10], s.dn[208][11], s.dn[208][12], s.dn[208][13], s.dn[208][14], s.dn[208][15], eq31_e752_d_n16, eq31_e752_d_n17, s.dn[208][18], s.dn[208][19], s.dn[208][20], s.dn[208][21], s.dn[208][22], s.dn[208][23], s.dn[208][24], s.dn[208][25], s.dn[208][26], s.dn[208][27], s.dn[208][28], s.dn[208][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e754;
        let eq31_node_derivatives: [f64; 30] = [eq31_e754_d_n0, eq31_e754_d_n1, eq31_e754_d_n2, eq31_e754_d_n3, eq31_e754_d_n4, eq31_e754_d_n5, eq31_e754_d_n6, eq31_e754_d_n7, eq31_e754_d_n8, eq31_e754_d_n9, eq31_e754_d_n10, eq31_e754_d_n11, eq31_e754_d_n12, eq31_e754_d_n13, eq31_e754_d_n14, eq31_e754_d_n15, eq31_e754_d_n16, eq31_e754_d_n17, eq31_e754_d_n18, eq31_e754_d_n19, eq31_e754_d_n20, eq31_e754_d_n21, eq31_e754_d_n22, eq31_e754_d_n23, eq31_e754_d_n24, eq31_e754_d_n25, eq31_e754_d_n26, eq31_e754_d_n27, eq31_e754_d_n28, eq31_e754_d_n29];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            Some(nodes[16]),
            self.multiplicity * (eq31_value),
            &nodes,
            &eq31_node_derivatives,
            &branches,
            &eq31_branch_derivatives,
            self.multiplicity,
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
        let (eq32_e759,) = {
    if (!(s.v[320] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e759;
        stamper.stamp_potential(
            branches[18],
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq33_e769, eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29,) = {
    if (s.v[466] != 0.0) {
        let eq33_e762: f64 = self.eval_ddt(4, s.v[209]);
        let eq33_e762_d_n0: f64 = self.ddt_jacobian(s.dn[209][0]);
        let eq33_e762_d_n1: f64 = self.ddt_jacobian(s.dn[209][1]);
        let eq33_e762_d_n2: f64 = self.ddt_jacobian(s.dn[209][2]);
        let eq33_e762_d_n3: f64 = self.ddt_jacobian(s.dn[209][3]);
        let eq33_e762_d_n4: f64 = self.ddt_jacobian(s.dn[209][4]);
        let eq33_e762_d_n5: f64 = self.ddt_jacobian(s.dn[209][5]);
        let eq33_e762_d_n6: f64 = self.ddt_jacobian(s.dn[209][6]);
        let eq33_e762_d_n7: f64 = self.ddt_jacobian(s.dn[209][7]);
        let eq33_e762_d_n8: f64 = self.ddt_jacobian(s.dn[209][8]);
        let eq33_e762_d_n9: f64 = self.ddt_jacobian(s.dn[209][9]);
        let eq33_e762_d_n10: f64 = self.ddt_jacobian(s.dn[209][10]);
        let eq33_e762_d_n11: f64 = self.ddt_jacobian(s.dn[209][11]);
        let eq33_e762_d_n12: f64 = self.ddt_jacobian(s.dn[209][12]);
        let eq33_e762_d_n13: f64 = self.ddt_jacobian(s.dn[209][13]);
        let eq33_e762_d_n14: f64 = self.ddt_jacobian(s.dn[209][14]);
        let eq33_e762_d_n15: f64 = self.ddt_jacobian(s.dn[209][15]);
        let eq33_e762_d_n16: f64 = self.ddt_jacobian(s.dn[209][16]);
        let eq33_e762_d_n17: f64 = self.ddt_jacobian(s.dn[209][17]);
        let eq33_e762_d_n18: f64 = self.ddt_jacobian(s.dn[209][18]);
        let eq33_e762_d_n19: f64 = self.ddt_jacobian(s.dn[209][19]);
        let eq33_e762_d_n20: f64 = self.ddt_jacobian(s.dn[209][20]);
        let eq33_e762_d_n21: f64 = self.ddt_jacobian(s.dn[209][21]);
        let eq33_e762_d_n22: f64 = self.ddt_jacobian(s.dn[209][22]);
        let eq33_e762_d_n23: f64 = self.ddt_jacobian(s.dn[209][23]);
        let eq33_e762_d_n24: f64 = self.ddt_jacobian(s.dn[209][24]);
        let eq33_e762_d_n25: f64 = self.ddt_jacobian(s.dn[209][25]);
        let eq33_e762_d_n26: f64 = self.ddt_jacobian(s.dn[209][26]);
        let eq33_e762_d_n27: f64 = self.ddt_jacobian(s.dn[209][27]);
        let eq33_e762_d_n28: f64 = self.ddt_jacobian(s.dn[209][28]);
        let eq33_e762_d_n29: f64 = self.ddt_jacobian(s.dn[209][29]);
        let eq33_e765: f64 = (p.p355 * (nv7 - nv16));
        let eq33_e765_d_n7: f64 = p.p355;
        let eq33_e765_d_n16: f64 = (-p.p355);
        let eq33_e766: f64 = self.eval_ddt(5, eq33_e765);
        let eq33_e766_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n7: f64 = self.ddt_jacobian(eq33_e765_d_n7);
        let eq33_e766_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n16: f64 = self.ddt_jacobian(eq33_e765_d_n16);
        let eq33_e766_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq33_e766_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq33_e767: f64 = (eq33_e762 + eq33_e766);
        let eq33_e767_d_n0: f64 = (eq33_e762_d_n0 + eq33_e766_d_n0);
        let eq33_e767_d_n1: f64 = (eq33_e762_d_n1 + eq33_e766_d_n1);
        let eq33_e767_d_n2: f64 = (eq33_e762_d_n2 + eq33_e766_d_n2);
        let eq33_e767_d_n3: f64 = (eq33_e762_d_n3 + eq33_e766_d_n3);
        let eq33_e767_d_n4: f64 = (eq33_e762_d_n4 + eq33_e766_d_n4);
        let eq33_e767_d_n5: f64 = (eq33_e762_d_n5 + eq33_e766_d_n5);
        let eq33_e767_d_n6: f64 = (eq33_e762_d_n6 + eq33_e766_d_n6);
        let eq33_e767_d_n7: f64 = (eq33_e762_d_n7 + eq33_e766_d_n7);
        let eq33_e767_d_n8: f64 = (eq33_e762_d_n8 + eq33_e766_d_n8);
        let eq33_e767_d_n9: f64 = (eq33_e762_d_n9 + eq33_e766_d_n9);
        let eq33_e767_d_n10: f64 = (eq33_e762_d_n10 + eq33_e766_d_n10);
        let eq33_e767_d_n11: f64 = (eq33_e762_d_n11 + eq33_e766_d_n11);
        let eq33_e767_d_n12: f64 = (eq33_e762_d_n12 + eq33_e766_d_n12);
        let eq33_e767_d_n13: f64 = (eq33_e762_d_n13 + eq33_e766_d_n13);
        let eq33_e767_d_n14: f64 = (eq33_e762_d_n14 + eq33_e766_d_n14);
        let eq33_e767_d_n15: f64 = (eq33_e762_d_n15 + eq33_e766_d_n15);
        let eq33_e767_d_n16: f64 = (eq33_e762_d_n16 + eq33_e766_d_n16);
        let eq33_e767_d_n17: f64 = (eq33_e762_d_n17 + eq33_e766_d_n17);
        let eq33_e767_d_n18: f64 = (eq33_e762_d_n18 + eq33_e766_d_n18);
        let eq33_e767_d_n19: f64 = (eq33_e762_d_n19 + eq33_e766_d_n19);
        let eq33_e767_d_n20: f64 = (eq33_e762_d_n20 + eq33_e766_d_n20);
        let eq33_e767_d_n21: f64 = (eq33_e762_d_n21 + eq33_e766_d_n21);
        let eq33_e767_d_n22: f64 = (eq33_e762_d_n22 + eq33_e766_d_n22);
        let eq33_e767_d_n23: f64 = (eq33_e762_d_n23 + eq33_e766_d_n23);
        let eq33_e767_d_n24: f64 = (eq33_e762_d_n24 + eq33_e766_d_n24);
        let eq33_e767_d_n25: f64 = (eq33_e762_d_n25 + eq33_e766_d_n25);
        let eq33_e767_d_n26: f64 = (eq33_e762_d_n26 + eq33_e766_d_n26);
        let eq33_e767_d_n27: f64 = (eq33_e762_d_n27 + eq33_e766_d_n27);
        let eq33_e767_d_n28: f64 = (eq33_e762_d_n28 + eq33_e766_d_n28);
        let eq33_e767_d_n29: f64 = (eq33_e762_d_n29 + eq33_e766_d_n29);
        (eq33_e767, eq33_e767_d_n0, eq33_e767_d_n1, eq33_e767_d_n2, eq33_e767_d_n3, eq33_e767_d_n4, eq33_e767_d_n5, eq33_e767_d_n6, eq33_e767_d_n7, eq33_e767_d_n8, eq33_e767_d_n9, eq33_e767_d_n10, eq33_e767_d_n11, eq33_e767_d_n12, eq33_e767_d_n13, eq33_e767_d_n14, eq33_e767_d_n15, eq33_e767_d_n16, eq33_e767_d_n17, eq33_e767_d_n18, eq33_e767_d_n19, eq33_e767_d_n20, eq33_e767_d_n21, eq33_e767_d_n22, eq33_e767_d_n23, eq33_e767_d_n24, eq33_e767_d_n25, eq33_e767_d_n26, eq33_e767_d_n27, eq33_e767_d_n28, eq33_e767_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e769;
        let eq33_node_derivatives: [f64; 30] = [eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[16]),
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq34_e779, eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29,) = {
    if (s.v[466] != 0.0) {
        let eq34_e772: f64 = self.eval_ddt(6, s.v[210]);
        let eq34_e772_d_n0: f64 = self.ddt_jacobian(s.dn[210][0]);
        let eq34_e772_d_n1: f64 = self.ddt_jacobian(s.dn[210][1]);
        let eq34_e772_d_n2: f64 = self.ddt_jacobian(s.dn[210][2]);
        let eq34_e772_d_n3: f64 = self.ddt_jacobian(s.dn[210][3]);
        let eq34_e772_d_n4: f64 = self.ddt_jacobian(s.dn[210][4]);
        let eq34_e772_d_n5: f64 = self.ddt_jacobian(s.dn[210][5]);
        let eq34_e772_d_n6: f64 = self.ddt_jacobian(s.dn[210][6]);
        let eq34_e772_d_n7: f64 = self.ddt_jacobian(s.dn[210][7]);
        let eq34_e772_d_n8: f64 = self.ddt_jacobian(s.dn[210][8]);
        let eq34_e772_d_n9: f64 = self.ddt_jacobian(s.dn[210][9]);
        let eq34_e772_d_n10: f64 = self.ddt_jacobian(s.dn[210][10]);
        let eq34_e772_d_n11: f64 = self.ddt_jacobian(s.dn[210][11]);
        let eq34_e772_d_n12: f64 = self.ddt_jacobian(s.dn[210][12]);
        let eq34_e772_d_n13: f64 = self.ddt_jacobian(s.dn[210][13]);
        let eq34_e772_d_n14: f64 = self.ddt_jacobian(s.dn[210][14]);
        let eq34_e772_d_n15: f64 = self.ddt_jacobian(s.dn[210][15]);
        let eq34_e772_d_n16: f64 = self.ddt_jacobian(s.dn[210][16]);
        let eq34_e772_d_n17: f64 = self.ddt_jacobian(s.dn[210][17]);
        let eq34_e772_d_n18: f64 = self.ddt_jacobian(s.dn[210][18]);
        let eq34_e772_d_n19: f64 = self.ddt_jacobian(s.dn[210][19]);
        let eq34_e772_d_n20: f64 = self.ddt_jacobian(s.dn[210][20]);
        let eq34_e772_d_n21: f64 = self.ddt_jacobian(s.dn[210][21]);
        let eq34_e772_d_n22: f64 = self.ddt_jacobian(s.dn[210][22]);
        let eq34_e772_d_n23: f64 = self.ddt_jacobian(s.dn[210][23]);
        let eq34_e772_d_n24: f64 = self.ddt_jacobian(s.dn[210][24]);
        let eq34_e772_d_n25: f64 = self.ddt_jacobian(s.dn[210][25]);
        let eq34_e772_d_n26: f64 = self.ddt_jacobian(s.dn[210][26]);
        let eq34_e772_d_n27: f64 = self.ddt_jacobian(s.dn[210][27]);
        let eq34_e772_d_n28: f64 = self.ddt_jacobian(s.dn[210][28]);
        let eq34_e772_d_n29: f64 = self.ddt_jacobian(s.dn[210][29]);
        let eq34_e775: f64 = (p.p355 * (nv7 - nv17));
        let eq34_e775_d_n7: f64 = p.p355;
        let eq34_e775_d_n17: f64 = (-p.p355);
        let eq34_e776: f64 = self.eval_ddt(7, eq34_e775);
        let eq34_e776_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n7: f64 = self.ddt_jacobian(eq34_e775_d_n7);
        let eq34_e776_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n17: f64 = self.ddt_jacobian(eq34_e775_d_n17);
        let eq34_e776_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq34_e776_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq34_e777: f64 = (eq34_e772 + eq34_e776);
        let eq34_e777_d_n0: f64 = (eq34_e772_d_n0 + eq34_e776_d_n0);
        let eq34_e777_d_n1: f64 = (eq34_e772_d_n1 + eq34_e776_d_n1);
        let eq34_e777_d_n2: f64 = (eq34_e772_d_n2 + eq34_e776_d_n2);
        let eq34_e777_d_n3: f64 = (eq34_e772_d_n3 + eq34_e776_d_n3);
        let eq34_e777_d_n4: f64 = (eq34_e772_d_n4 + eq34_e776_d_n4);
        let eq34_e777_d_n5: f64 = (eq34_e772_d_n5 + eq34_e776_d_n5);
        let eq34_e777_d_n6: f64 = (eq34_e772_d_n6 + eq34_e776_d_n6);
        let eq34_e777_d_n7: f64 = (eq34_e772_d_n7 + eq34_e776_d_n7);
        let eq34_e777_d_n8: f64 = (eq34_e772_d_n8 + eq34_e776_d_n8);
        let eq34_e777_d_n9: f64 = (eq34_e772_d_n9 + eq34_e776_d_n9);
        let eq34_e777_d_n10: f64 = (eq34_e772_d_n10 + eq34_e776_d_n10);
        let eq34_e777_d_n11: f64 = (eq34_e772_d_n11 + eq34_e776_d_n11);
        let eq34_e777_d_n12: f64 = (eq34_e772_d_n12 + eq34_e776_d_n12);
        let eq34_e777_d_n13: f64 = (eq34_e772_d_n13 + eq34_e776_d_n13);
        let eq34_e777_d_n14: f64 = (eq34_e772_d_n14 + eq34_e776_d_n14);
        let eq34_e777_d_n15: f64 = (eq34_e772_d_n15 + eq34_e776_d_n15);
        let eq34_e777_d_n16: f64 = (eq34_e772_d_n16 + eq34_e776_d_n16);
        let eq34_e777_d_n17: f64 = (eq34_e772_d_n17 + eq34_e776_d_n17);
        let eq34_e777_d_n18: f64 = (eq34_e772_d_n18 + eq34_e776_d_n18);
        let eq34_e777_d_n19: f64 = (eq34_e772_d_n19 + eq34_e776_d_n19);
        let eq34_e777_d_n20: f64 = (eq34_e772_d_n20 + eq34_e776_d_n20);
        let eq34_e777_d_n21: f64 = (eq34_e772_d_n21 + eq34_e776_d_n21);
        let eq34_e777_d_n22: f64 = (eq34_e772_d_n22 + eq34_e776_d_n22);
        let eq34_e777_d_n23: f64 = (eq34_e772_d_n23 + eq34_e776_d_n23);
        let eq34_e777_d_n24: f64 = (eq34_e772_d_n24 + eq34_e776_d_n24);
        let eq34_e777_d_n25: f64 = (eq34_e772_d_n25 + eq34_e776_d_n25);
        let eq34_e777_d_n26: f64 = (eq34_e772_d_n26 + eq34_e776_d_n26);
        let eq34_e777_d_n27: f64 = (eq34_e772_d_n27 + eq34_e776_d_n27);
        let eq34_e777_d_n28: f64 = (eq34_e772_d_n28 + eq34_e776_d_n28);
        let eq34_e777_d_n29: f64 = (eq34_e772_d_n29 + eq34_e776_d_n29);
        (eq34_e777, eq34_e777_d_n0, eq34_e777_d_n1, eq34_e777_d_n2, eq34_e777_d_n3, eq34_e777_d_n4, eq34_e777_d_n5, eq34_e777_d_n6, eq34_e777_d_n7, eq34_e777_d_n8, eq34_e777_d_n9, eq34_e777_d_n10, eq34_e777_d_n11, eq34_e777_d_n12, eq34_e777_d_n13, eq34_e777_d_n14, eq34_e777_d_n15, eq34_e777_d_n16, eq34_e777_d_n17, eq34_e777_d_n18, eq34_e777_d_n19, eq34_e777_d_n20, eq34_e777_d_n21, eq34_e777_d_n22, eq34_e777_d_n23, eq34_e777_d_n24, eq34_e777_d_n25, eq34_e777_d_n26, eq34_e777_d_n27, eq34_e777_d_n28, eq34_e777_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e779;
        let eq34_node_derivatives: [f64; 30] = [eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[17]),
            self.multiplicity * (eq34_value),
            &nodes,
            &eq34_node_derivatives,
            &branches,
            &eq34_branch_derivatives,
            self.multiplicity,
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq35_e789, eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29,) = {
    if (s.v[466] != 0.0) {
        let eq35_e782: f64 = self.eval_ddt(8, s.v[211]);
        let eq35_e782_d_n0: f64 = self.ddt_jacobian(s.dn[211][0]);
        let eq35_e782_d_n1: f64 = self.ddt_jacobian(s.dn[211][1]);
        let eq35_e782_d_n2: f64 = self.ddt_jacobian(s.dn[211][2]);
        let eq35_e782_d_n3: f64 = self.ddt_jacobian(s.dn[211][3]);
        let eq35_e782_d_n4: f64 = self.ddt_jacobian(s.dn[211][4]);
        let eq35_e782_d_n5: f64 = self.ddt_jacobian(s.dn[211][5]);
        let eq35_e782_d_n6: f64 = self.ddt_jacobian(s.dn[211][6]);
        let eq35_e782_d_n7: f64 = self.ddt_jacobian(s.dn[211][7]);
        let eq35_e782_d_n8: f64 = self.ddt_jacobian(s.dn[211][8]);
        let eq35_e782_d_n9: f64 = self.ddt_jacobian(s.dn[211][9]);
        let eq35_e782_d_n10: f64 = self.ddt_jacobian(s.dn[211][10]);
        let eq35_e782_d_n11: f64 = self.ddt_jacobian(s.dn[211][11]);
        let eq35_e782_d_n12: f64 = self.ddt_jacobian(s.dn[211][12]);
        let eq35_e782_d_n13: f64 = self.ddt_jacobian(s.dn[211][13]);
        let eq35_e782_d_n14: f64 = self.ddt_jacobian(s.dn[211][14]);
        let eq35_e782_d_n15: f64 = self.ddt_jacobian(s.dn[211][15]);
        let eq35_e782_d_n16: f64 = self.ddt_jacobian(s.dn[211][16]);
        let eq35_e782_d_n17: f64 = self.ddt_jacobian(s.dn[211][17]);
        let eq35_e782_d_n18: f64 = self.ddt_jacobian(s.dn[211][18]);
        let eq35_e782_d_n19: f64 = self.ddt_jacobian(s.dn[211][19]);
        let eq35_e782_d_n20: f64 = self.ddt_jacobian(s.dn[211][20]);
        let eq35_e782_d_n21: f64 = self.ddt_jacobian(s.dn[211][21]);
        let eq35_e782_d_n22: f64 = self.ddt_jacobian(s.dn[211][22]);
        let eq35_e782_d_n23: f64 = self.ddt_jacobian(s.dn[211][23]);
        let eq35_e782_d_n24: f64 = self.ddt_jacobian(s.dn[211][24]);
        let eq35_e782_d_n25: f64 = self.ddt_jacobian(s.dn[211][25]);
        let eq35_e782_d_n26: f64 = self.ddt_jacobian(s.dn[211][26]);
        let eq35_e782_d_n27: f64 = self.ddt_jacobian(s.dn[211][27]);
        let eq35_e782_d_n28: f64 = self.ddt_jacobian(s.dn[211][28]);
        let eq35_e782_d_n29: f64 = self.ddt_jacobian(s.dn[211][29]);
        let eq35_e785: f64 = (p.p355 * (nv2 - nv16));
        let eq35_e785_d_n2: f64 = p.p355;
        let eq35_e785_d_n16: f64 = (-p.p355);
        let eq35_e786: f64 = self.eval_ddt(9, eq35_e785);
        let eq35_e786_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n2: f64 = self.ddt_jacobian(eq35_e785_d_n2);
        let eq35_e786_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n16: f64 = self.ddt_jacobian(eq35_e785_d_n16);
        let eq35_e786_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq35_e786_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq35_e787: f64 = (eq35_e782 + eq35_e786);
        let eq35_e787_d_n0: f64 = (eq35_e782_d_n0 + eq35_e786_d_n0);
        let eq35_e787_d_n1: f64 = (eq35_e782_d_n1 + eq35_e786_d_n1);
        let eq35_e787_d_n2: f64 = (eq35_e782_d_n2 + eq35_e786_d_n2);
        let eq35_e787_d_n3: f64 = (eq35_e782_d_n3 + eq35_e786_d_n3);
        let eq35_e787_d_n4: f64 = (eq35_e782_d_n4 + eq35_e786_d_n4);
        let eq35_e787_d_n5: f64 = (eq35_e782_d_n5 + eq35_e786_d_n5);
        let eq35_e787_d_n6: f64 = (eq35_e782_d_n6 + eq35_e786_d_n6);
        let eq35_e787_d_n7: f64 = (eq35_e782_d_n7 + eq35_e786_d_n7);
        let eq35_e787_d_n8: f64 = (eq35_e782_d_n8 + eq35_e786_d_n8);
        let eq35_e787_d_n9: f64 = (eq35_e782_d_n9 + eq35_e786_d_n9);
        let eq35_e787_d_n10: f64 = (eq35_e782_d_n10 + eq35_e786_d_n10);
        let eq35_e787_d_n11: f64 = (eq35_e782_d_n11 + eq35_e786_d_n11);
        let eq35_e787_d_n12: f64 = (eq35_e782_d_n12 + eq35_e786_d_n12);
        let eq35_e787_d_n13: f64 = (eq35_e782_d_n13 + eq35_e786_d_n13);
        let eq35_e787_d_n14: f64 = (eq35_e782_d_n14 + eq35_e786_d_n14);
        let eq35_e787_d_n15: f64 = (eq35_e782_d_n15 + eq35_e786_d_n15);
        let eq35_e787_d_n16: f64 = (eq35_e782_d_n16 + eq35_e786_d_n16);
        let eq35_e787_d_n17: f64 = (eq35_e782_d_n17 + eq35_e786_d_n17);
        let eq35_e787_d_n18: f64 = (eq35_e782_d_n18 + eq35_e786_d_n18);
        let eq35_e787_d_n19: f64 = (eq35_e782_d_n19 + eq35_e786_d_n19);
        let eq35_e787_d_n20: f64 = (eq35_e782_d_n20 + eq35_e786_d_n20);
        let eq35_e787_d_n21: f64 = (eq35_e782_d_n21 + eq35_e786_d_n21);
        let eq35_e787_d_n22: f64 = (eq35_e782_d_n22 + eq35_e786_d_n22);
        let eq35_e787_d_n23: f64 = (eq35_e782_d_n23 + eq35_e786_d_n23);
        let eq35_e787_d_n24: f64 = (eq35_e782_d_n24 + eq35_e786_d_n24);
        let eq35_e787_d_n25: f64 = (eq35_e782_d_n25 + eq35_e786_d_n25);
        let eq35_e787_d_n26: f64 = (eq35_e782_d_n26 + eq35_e786_d_n26);
        let eq35_e787_d_n27: f64 = (eq35_e782_d_n27 + eq35_e786_d_n27);
        let eq35_e787_d_n28: f64 = (eq35_e782_d_n28 + eq35_e786_d_n28);
        let eq35_e787_d_n29: f64 = (eq35_e782_d_n29 + eq35_e786_d_n29);
        (eq35_e787, eq35_e787_d_n0, eq35_e787_d_n1, eq35_e787_d_n2, eq35_e787_d_n3, eq35_e787_d_n4, eq35_e787_d_n5, eq35_e787_d_n6, eq35_e787_d_n7, eq35_e787_d_n8, eq35_e787_d_n9, eq35_e787_d_n10, eq35_e787_d_n11, eq35_e787_d_n12, eq35_e787_d_n13, eq35_e787_d_n14, eq35_e787_d_n15, eq35_e787_d_n16, eq35_e787_d_n17, eq35_e787_d_n18, eq35_e787_d_n19, eq35_e787_d_n20, eq35_e787_d_n21, eq35_e787_d_n22, eq35_e787_d_n23, eq35_e787_d_n24, eq35_e787_d_n25, eq35_e787_d_n26, eq35_e787_d_n27, eq35_e787_d_n28, eq35_e787_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e789;
        let eq35_node_derivatives: [f64; 30] = [eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            self.multiplicity * (eq35_value),
            &nodes,
            &eq35_node_derivatives,
            &branches,
            &eq35_branch_derivatives,
            self.multiplicity,
        );
    }
}
